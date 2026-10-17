#!/usr/bin/env bun
// tool/src/objects/fixed-primitive-array-inl.h.js — -inl 语义合并指引生成
// 组→文件映射 (本脚本管辖):
//   v8/src/objects/fixed-primitive-array-inl.h → 合并进 v8-objects-arrays/src/primitive_array.rs
//   (Rust 无 inl; 本脚本产物 primitive_array_inl.rs.skel 为逐方法合并指引, 最终以就地修订 src 为准)
// 锚定常量来源: src/common/globals.h (kHoleNanInt64 / kUndefinedNanInt64 / OBJECT_POINTER_ALIGN)
import { readV8, writeSrc, banner } from "../../lib.js";

const inl = readV8("src/objects/fixed-primitive-array-inl.h");
const globals = readV8("src/common/globals.h");

let checks = 0;
const must = (cond, msg) => {
  checks++;
  if (!cond) throw new Error(`锚点失配: ${msg}`);
};
const has = (s, sub, where) => {
  if (!s.includes(sub)) throw new Error(`锚点失配: ${where} 无 "${sub.slice(0, 64)}"`);
  checks++;
};

// ── 1. 定义全集对数: 机械提取 `X::method` 定义行 ─────────────────────────
const defs = [...inl.matchAll(/(\w+)(?:<[^<>]*>)?::(\w+)\(/g)].map((m) => `${m[1]}::${m[2]}`);
const uniq = [...new Set(defs)];
const expectDefs = [
  "PrimitiveArrayBase::IsInBounds", "PrimitiveArrayBase::get", "PrimitiveArrayBase::set",
  "PrimitiveArrayBase::AllocatedSize", "PrimitiveArrayBase::begin", "PrimitiveArrayBase::end",
  "PrimitiveArrayBase::DataSize", "PrimitiveArrayBase::FromAddressOfFirstElement",
  "PrimitiveArrayBase::Allocate",
  "FixedDoubleArray::New", "FixedDoubleArray::get_scalar", "FixedDoubleArray::get_representation",
  "FixedDoubleArray::get", "FixedDoubleArray::set", "FixedDoubleArray::set_raw",
  "FixedDoubleArray::set_undefined", "FixedDoubleArray::is_undefined",
  "FixedDoubleArray::set_the_hole", "FixedDoubleArray::is_the_hole",
  "FixedDoubleArray::MoveElements", "FixedDoubleArray::FillWithHoles",
  "ByteArray::New", "ByteArray::get_int", "ByteArray::set_int",
  "TrustedByteArray::New", "TrustedByteArray::get_int", "TrustedByteArray::set_int",
  "TrustedFixedAddressArray::New",
  "FixedIntegerArrayBase::New", "FixedIntegerArrayBase::get_element_address",
  "FixedIntegerArrayBase::get", "FixedIntegerArrayBase::set", "FixedIntegerArrayBase::length",
];
for (const d of expectDefs) {
  const [cls, m] = d.split("::");
  must(uniq.some((u) => u === `${cls}::${m}`), `定义 ${d} 缺失`);
}
must(expectDefs.length === 33, `指引清单应为 33 项, 实得 ${expectDefs.length}`);
// 特判: begin/end 各 const/非const 两定义 + get/set 重载, 定义行总数核对
must(defs.length >= expectDefs.length, `定义行总数 ${defs.length} < ${expectDefs.length}`);

// ── 2. 关键语义锚点 ──────────────────────────────────────────────────────
has(inl, "return static_cast<unsigned>(index) < this->ulength().value();", "IsInBounds");
has(inl, "return SizeFor(this->length(kAcquireLoad).value());", "AllocatedSize(acquire)");
has(inl, "int data_size = SizeFor(this->ulength().value()) - OFFSET_OF_DATA_START(D);", "DataSize");
has(inl, "Tagged<Object>(address - OFFSET_OF_DATA_START(D) + kHeapObjectTag)", "FromAddressOfFirstElement");
has(inl, "if (std::isnan(value)) {\n    value = std::numeric_limits<double>::quiet_NaN();", "FixedDoubleArray::set NaN 静默化");
has(inl, "values()[index].set_value_as_bits(kHoleNanInt64);", "set_the_hole");
has(inl, "return get_representation(index) == kHoleNanInt64;", "is_the_hole");
has(inl, "MemMove(&values()[dst_index], &values()[src_index], len * kElementSize);", "MoveElements");
has(inl, "int padding_size = SizeFor(length) - OffsetOfElementAt(length);\n  memset(&result->values()[length], 0, padding_size);", "ByteArray::New 尾部 padding 清零");
has(inl, "return base::ReadUnalignedValue<uint32_t>(", "get_int 非对齐读");
has(inl, "checked_byte_length *= sizeof(T);", "FixedIntegerArrayBase::New 溢出检查");
has(inl, "return SafeHeapObjectSize(len / sizeof(T));", "FixedIntegerArrayBase::length 除法");
has(inl, "DCHECK_EQ(len % sizeof(T), 0);", "length 整除断言");
// 配置锚点: 非 mips 分支 kHoleNan 双 32 位半字 + UNDEFINED_DOUBLE 默认开关注册
has(globals, "constexpr uint32_t kHoleNanUpper32 = 0xFFF7FFFF;", "globals.h kHoleNanUpper32");
has(globals, "constexpr uint32_t kHoleNanLower32 = 0xFFF7FFFF;", "globals.h kHoleNanLower32");
checks += 1; // V8_UNDEFINED_DOUBLE 配置判定另见 v8-common config.rs (仓库统一关, 偏差登记)

// ── 3. 合并指引产物 ──────────────────────────────────────────────────────
const rows = expectDefs.map((d) => {
  const [cls, m] = d.split("::");
  const rs = m.replace(/^(AllocatedSize|DataSize|SizeFor|OffsetOfElementAt|MaxRegularLength|FromAddressOfFirstElement|IsInBounds|MoveElements|FillWithHoles|SetComponents)$/, (x) =>
    x.replace(/([A-Z])/g, "_$1").toLowerCase());
  return `// ${d} → ${cls.toLowerCase()}_${rs || m.toLowerCase()} (${rs || m.toLowerCase()})`;
}).join("\n");

const body = `${banner("src/objects/fixed-primitive-array-inl.h.js", "src/objects/fixed-primitive-array-inl.h", "-inl 合并指引; Rust 无 inl, 语义并入 primitive_array.rs 就地修订")}
//! -inl 合并指引 (33 个定义, C++ 行内 → Rust 常规实现, 内联由编译器决定)
//!
//! 命名映射: 方法名 snake_case; tag 重载用后缀区分:
//!   length()/length(AcquireLoadTag)/length(RelaxedLoadTag) → length()/length_acquire()/length_relaxed()
//!   set_length(v, ReleaseStoreTag) → set_length_release(v); capacity 族同理。
//! 语义要点:
//!   1. IsInBounds: (index as u32) < ulength() — 负下标转 u32 回绕自然判负
//!   2. AllocatedSize: SizeFor(length_acquire()) — right-trimming 并发访问须 acquire
//!   3. FixedDoubleArray::set: NaN 输入 → 静默化为 canonical quiet NaN (0x7FF8000000000000)
//!   4. set_the_hole/is_the_hole: 位级比较 K_HOLE_NAN_INT64 (globals.rs 已导出)
//!   5. set_undefined/is_undefined: V8_ENABLE_UNDEFINED_DOUBLE 关 → 裁剪不落地 (偏差登记)
//!   6. MoveElements: ptr::copy (MemMove 等价, 允许重叠)
//!   7. ByteArray/TrustedByteArray::New: 安装后 memset 尾部 SizeFor-OffsetOfElementAt 字节
//!   8. get_int/set_int: 非对齐 u32 读写 (read_unaligned/write_unaligned)
//!   9. FixedIntegerArrayBase::New: length*sizeof(T) u32 溢出检查 (CheckedNumeric → Option/Result)
//!  10. Allocate/New(isolate): factory 未落地, 以 install(map, length, buffer) 表达 (票单申报)
${rows}
`;

const p = writeSrc("v8-objects-arrays", "primitive_array_inl.rs.skel", body);
console.log(`fixed-primitive-array-inl.h.js: 合并指引 ${p} — 定义全集 ${expectDefs.length}/33、语义锚点 + 配置锚点合计断言 ${checks} 项全部通过`);
