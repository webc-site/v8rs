#!/usr/bin/env bun
// tool/src/objects/bytecode-array.h.js — 字节码数组族生成
// 组→文件映射 (本脚本管辖):
//   v8/src/objects/bytecode-array.{h,cc,-inl.h,tq} → v8-objects-rest/src/bytecode_array.rs
// 规则面: BytecodeArray/BytecodeWrapper 字段布局 (tq + h 双源核对) + 常量
//   (kMaxSize/kMaxLength/kHeaderSize/kBytesOffset) + 方法面; 解释器语义留骨架
import { readV8, writeSrc, banner } from "../../lib.js";

const h = readV8("src/objects/bytecode-array.h");
const tq = readV8("src/objects/bytecode-array.tq");
const inl = readV8("src/objects/bytecode-array-inl.h");
const cc = readV8("src/objects/bytecode-array.cc");

let checks = 0;
const need = (cond, msg) => { checks++; if (!cond) throw new Error(msg); };

// —— tq 字段表 (@cppObjectLayoutDefinition, TAGGED_SIZE_8_BYTES 为非压缩恒真 → padding 在列) ——
const tqClass = tq.match(/extern class BytecodeArray extends ExposedTrustedObject \{([\s\S]*?)\n\}/);
need(!!tqClass, "tq BytecodeArray 类缺失");
const tqFields = [...tqClass[1].matchAll(/^  (?:const |@if\(TAGGED_SIZE_8_BYTES\) )?([\w]+)(?:\[([\w]+)\])?: ([\w<>]+);/gm)]
  .map((m) => ({ name: m[1], indexed: m[2] ?? null, type: m[3] }));
const TQ_EXPECT = [
  ["length", "Smi"], ["wrapper", "BytecodeWrapper"], ["source_position_table", "ProtectedPointer<TrustedByteArray>"],
  ["handler_table", "ProtectedPointer<TrustedByteArray>"], ["constant_pool", "ProtectedPointer<TrustedFixedArray>"],
  ["frame_size", "int32"], ["parameter_size", "uint16"], ["max_arguments", "uint16"],
  ["incoming_new_target_or_generator_register", "int32"], ["optional_padding", "uint32"], ["bytes", "uint8"],
];
if (tqFields.length !== TQ_EXPECT.length || tqFields.some((f, i) => f.name !== TQ_EXPECT[i][0] || f.type !== TQ_EXPECT[i][1]))
  throw new Error(`tq 字段表漂移:\n  ${tqFields.map((f) => `${f.name}:${f.type}`).join(", ")}`);
need(tqFields[10].indexed === "length", "bytes 应为 bytes[length] 柔性尾");

// —— h 侧 C++ 成员面 (与 tq 双源核对) ——
need(h.includes("V8_TQ_CONST TaggedMember<Smi> length_;"), "length_ 成员缺失");
need(h.includes("ProtectedTaggedMember<TrustedByteArray> source_position_table_;"), "source_position_table_ 成员缺失");
need(h.includes("FLEXIBLE_ARRAY_MEMBER(uint8_t, bytes);"), "bytes 柔性成员缺失");
need(h.includes("uint32_t optional_padding_;"), "optional_padding_ (TAGGED_SIZE_8_BYTES) 缺失");

// —— 常量 ——
need(/static const int kMaxSize = 512 \* MB;/.test(h), "kMaxSize = 512*MB 缺失");
const mb = 512 * 1024 * 1024;
need(/kMaxLength =\s*\n\s*BytecodeArray::kMaxSize - BytecodeArray::kHeaderSize;/.test(h), "kMaxLength 公式缺失");
need(/inline constexpr int BytecodeArray::kHeaderSize =\s*\n\s*OFFSET_OF_DATA_START\(BytecodeArray\);/.test(h), "kHeaderSize 定义缺失");

// —— 布局计算 (非压缩: kTaggedSize=8, 沙箱关 → ExposedTrustedObject 头 = HeapObject 头 = 8) ——
// 字段: map(8) + length(8) + wrapper(8) + source_position_table(8) + handler_table(8) + constant_pool(8)
//      + frame_size(4) + parameter_size(2) + max_arguments(2) + incoming(4) + padding(4) = 60 → data start (bytes) 60?
// 布局细节: 前缀 6×8=48; frame_size@48(4); parameter_size@52(2); max_arguments@54(2); incoming@56(4); padding@60(4) → kHeaderSize=64
const K_HEADER_SIZE = 64;
const K_MAX_LENGTH = mb - K_HEADER_SIZE;

// —— 方法面 (h 声明, 内联+非内联; 宏声明访问器另走文本锚) ——
const methods = [...h.matchAll(/^\s{2}(?:inline )?(?:int|int32_t|uint8_t|uint16_t|void|bool|Address|Tagged<[\w<>]+>) (\w+)\(/gm)].map((m) => m[1]);
for (const need2 of ["length", "set_length", "wrapper", "parameter_count", "parameter_count_without_receiver",
  "max_arguments", "register_count", "get", "set", "GetFirstBytecodeAddress", "HasSourcePositionTable",
  "SourcePositionTable", "clear_padding", "BytecodeArraySize", "SizeIncludingMetadata", "CopyBytecodesTo", "MarkVerified"])
  need(methods.includes(need2), `BytecodeArray 方法缺 ${need2}`);
// 宏声明访问器 (DECL_*, 不进上面的签名正则)
for (const decl of ["DECL_PROTECTED_POINTER_ACCESSORS(handler_table, TrustedByteArray)",
  "DECL_PROTECTED_POINTER_ACCESSORS(constant_pool, TrustedFixedArray)",
  "DECL_INT32_ACCESSORS(frame_size)", "static constexpr int SizeFor(int length)"])
  need(h.includes(decl), `BytecodeArray 宏声明缺 ${decl}`);

// —— inl/cc 语义锚 ——
need(/Address BytecodeArray::GetFirstBytecodeAddress\(\) \{\s*\n\s*return reinterpret_cast<Address>\(bytes\(\)\);/.test(inl),
  "GetFirstBytecodeAddress 应基于 bytes() 柔性尾 (= kBytesOffset)");
need(/return static_cast<int>\(frame_size\(\)\) \/ kSystemPointerSize;/.test(inl), "register_count = frame_size / kSystemPointerSize 缺失");
need(/return frame_size\(\) \+ \(max_arguments\(\) << kSystemPointerSizeLog2\);/.test(inl), "max_frame_size 公式缺失");
need(cc.includes("void BytecodeArray::CopyBytecodesTo("), "CopyBytecodesTo 缺失");

// —— BytecodeWrapper ——
need(/extern class BytecodeWrapper extends Struct \{\s*\n\s*bytecode: TrustedPointer<BytecodeArray>;\s*\n\}/.test(tq), "tq BytecodeWrapper 漂移");
need(h.includes("DECL_TRUSTED_POINTER_ACCESSORS(bytecode, BytecodeArray)"), "wrapper 受信指针访问器缺失");

const body = `${banner("src/objects/bytecode-array.h.js", "src/objects/bytecode-array.{h,cc,-inl.h,tq}", "非压缩沙箱关布局: ExposedTrustedObject 头 8")}
//! 字节码数组族 (原版 bytecode-array.{h,cc,-inl.h,tq})
//!
//! BytecodeArray 活在 trusted space (沙箱外), 经 BytecodeWrapper (沙箱内
//! Struct) 与 tagged 世界互转. 布局: 头 ${K_HEADER_SIZE} 字节 + 柔性 bytes 尾.

// ===== 布局常量 (非压缩: kTaggedSize=8, 沙箱关) =====
/// 头大小 (map 8 + 5 个 tagged/protected 槽 40 + frame_size 4 + parameter 2 + max_arguments 2 + incoming 4 + padding 4)
pub const K_HEADER_SIZE: usize = ${K_HEADER_SIZE};

/// bytes 柔性尾偏移 (原版 kBytesOffset == kHeaderSize == OFFSET_OF_DATA_START)
pub const K_BYTES_OFFSET: usize = ${K_HEADER_SIZE};

/// 单个 BytecodeArray 内存上限 (原版 kMaxSize = 512 * MB)
pub const K_MAX_SIZE: usize = ${mb};

/// 最大字节码长度 (原版 kMaxLength = kMaxSize - kHeaderSize)
pub const K_MAX_LENGTH: i32 = ${K_MAX_LENGTH};

// ===== 字段表 (tq 与 h 双源核对一致, ${tqFields.length} 字段) =====
/// 字段清单 (声明顺序: 名, 类型, 偏移)
pub const BYTECODE_ARRAY_FIELDS: [(&str, &str, usize); ${tqFields.length}] = [
${tqFields.map((f, i) => {
    const offsets = [8, 16, 24, 32, 40, 48, 52, 54, 56, 60, 64];
    return `  ("${f.name}", "${f.type}", ${offsets[i]}),`;
  }).join("\n")}
];

/// 按长度求尺寸 (原版 SizeFor: OBJECT_POINTER_ALIGN(kHeaderSize + length))
pub const fn size_for(length: usize) -> usize {
  (K_HEADER_SIZE + length + 7) & !7
}

// ===== 方法面 =====
/// 字节码单字节读 (原版 get: 越界 DCHECK 后读 bytes()[index])
pub fn get(/* this, index: i32 */) -> u8 {
  todo!("就地修订: index 域检查 + bytes 尾读取")
}

/// 字节码单字节写
pub fn set(/* this, index, value: u8 */) {
  todo!("就地修订: 同上写路径")
}

/// 首字节码地址 (原版 GetFirstBytecodeAddress: address() + kBytesOffset)
pub fn get_first_bytecode_address(/* this */) -> usize {
  todo!("就地修订: self.addr + K_BYTES_OFFSET")
}

/// 寄存器数 (原版 register_count: frame_size / kSystemPointerSize)
pub fn register_count(/* this */) -> i32 {
  todo!("就地修订: frame_size() / 8")
}

/// 去接收者参数量 (原版 parameter_count_without_receiver: parameter_count - 1)
pub fn parameter_count_without_receiver(/* this */) -> u16 {
  todo!("就地修订: parameter_count() - 1")
}

/// 最大帧尺寸 (原版 max_frame_size: frame_size + max_arguments << kSystemPointerSizeLog2)
pub fn max_frame_size(/* this */) -> i32 {
  todo!("就地修订: frame_size() + (max_arguments() << 3)")
}

/// 源位置表取用 (原版 SourcePositionTable: acquire 读槽, Smi::zero() 视为收集失败 → 空表)
pub fn source_position_table(/* this */) /* -> Tagged<TrustedByteArray> */ {
  todo!("就地修订: 原版 -inl 170 行起: 空表/空槽分支")
}

/// 源位置收集失败标记 (原版 SetSourcePositionsFailedToCollect: 槽置 Smi::zero())
pub fn set_source_positions_failed_to_collect(/* this */) {
  todo!("就地修订: release 写 Smi::zero()")
}

/// 含元数据总大小 (原版 SizeIncludingMetadata: 字节码 + 常量池 + 源位置表 + 处理表)
pub fn size_including_metadata(/* this */) -> i32 {
  todo!("就地修订: BytecodeArraySize + 三表 Size 累加")
}

/// 复制字节码 (原版 CopyBytecodesTo: 等长断言 + CopyBytes)
pub fn copy_bytecodes_to(/* this, to */) {
  todo!("就地修订: DCHECK_EQ(length) + CopyBytes(首址, 首址, length)")
}

/// 验证通过发布 (原版 MarkVerified: 受信对象发布, 供解释器执行)
pub fn mark_verified(/* this, isolate */) {
  todo!("就地修订: Publish 语义 (trusted-object.h.js 席位已有骨架)")
}

/// 清填充 (快照确定性; TAGGED_SIZE_8_BYTES 下 padding 4 字节清零)
pub fn clear_padding(/* this */) {
  todo!("就地修订: kOptionalPaddingOffset 区间清零")
}

/// 对象总大小 (原版 BytecodeArraySize = SizeFor(length))
pub fn bytecode_array_size(/* this */) -> i32 {
  todo!("就地修订: size_for(length())")
}
`;

const p = writeSrc("v8-objects-rest", "bytecode_array.rs", body);
console.log(`bytecode-array.h.js: 已生成 ${p} (must ${checks}: 字段 ${tqFields.length}, K_HEADER_SIZE ${K_HEADER_SIZE}, K_MAX_LENGTH ${K_MAX_LENGTH}, 方法面 ${methods.length})`);
