#!/usr/bin/env bun
// tool/src/objects/instruction-stream.h.js — 指令流对象生成
// 组→文件映射 (本脚本管辖):
//   v8/src/objects/instruction-stream.h + instruction-stream.cc + instruction-stream-inl.h
//   → v8-objects-rest/src/instruction_stream.rs
// 基准配置: 非压缩 (kProtectedPointerSize=kTaggedSize=8) / 恒常池关 / 代码校验器关
import { readV8, writeSrc, banner } from "../../lib.js";

const h = readV8("src/objects/instruction-stream.h");
const cc = readV8("src/objects/instruction-stream.cc");
const inl = readV8("src/objects/instruction-stream-inl.h");

let checks = 0;
const need = (cond, msg) => { checks++; if (!cond) throw new Error(msg); };

// —— ISTREAM_FIELDS X-macro ——
const xma = h.match(/#define ISTREAM_FIELDS\(V\)([\s\S]*?)DEFINE_FIELD_OFFSET_CONSTANTS/);
need(!!xma, "ISTREAM_FIELDS 未找到");
const clean = (xma ? xma[1] : "").replace(/\\\n/g, " ").replace(/\/\*[\s\S]*?\*\//g, " ").replace(/\/\/[^\n]*/g, " ");
const raw = [...clean.matchAll(/V\((\w+),\s*([^)]+?)\)/g)].map((m) => ({ name: m[1], expr: m[2].trim() }));
need(raw.length === 7, `ISTREAM_FIELDS 应 7 项, 实得 ${raw.length}`);
// 求值: base = sizeof(TrustedObject) = 8 (沙箱关)
// kCodeOffset 8 / kRelocationInfoOffset 8 / kDataStart 0 / kBodySizeOffset 4 / kConstantPoolOffsetOffset 0 (关) / kUnalignedSize PAD / kHeaderSize 0
let cursor = 8;
const layout = [];
for (const f of raw) {
  let size;
  if (f.name === "kCodeOffset" || f.name === "kRelocationInfoOffset") size = 8;
  else if (f.name === "kBodySizeOffset") size = 4;
  else if (f.expr === "0") size = 0;
  else if (/V8_EMBEDDED_CONSTANT_POOL_BOOL \? kIntSize : 0/.test(f.expr)) size = 0;
  else if (/OBJECT_POINTER_PADDING/.test(f.expr)) size = "PAD";
  else throw new Error(`ISTREAM 尺寸不可解析: ${f.name} = ${f.expr}`);
  if (size === "PAD") {
    const pad = ((cursor + 7) & ~7) - cursor;
    layout.push({ ...f, offset: cursor, size: pad });
    cursor += pad;
  } else {
    layout.push({ ...f, offset: cursor, size });
    cursor += size;
  }
}
const kHeaderSize = layout[layout.length - 1].offset;
need(kHeaderSize === 32, `kHeaderSize 应 32 (非压缩), 实得 ${kHeaderSize}`);
need(layout[0].offset === 8 && layout[1].offset === 16, "kCodeOffset@8 / kRelocationInfoOffset@16 漂移");
need(layout[3].offset === 24, `kBodySizeOffset 应 24, 实得 ${layout[3].offset}`);

// —— 常量与公式 ——
need(/static constexpr int kMetadataAlignment = kIntSize;/.test(h), "kMetadataAlignment=4 缺失");
need(/static constexpr bool kOnHeapBodyIsContiguous = true;/.test(h), "kOnHeapBodyIsContiguous 缺失");
need(/static constexpr bool kOffHeapBodyIsContiguous = false;/.test(h), "kOffHeapBodyIsContiguous 缺失");
need(/static_assert\(kCodeAlignment >= kHeaderSize\);/.test(h), "kCodeAlignment ≥ kHeaderSize 断言缺失");
need(/kCodeAlignmentMinusCodeHeader =\s*\n\s*kCodeAlignment - kHeaderSize;/.test(h), "kCodeAlignmentMinusCodeHeader 公式缺失");

// —— 方法面 ——
for (const fn of ["instruction_start", "code", "raw_code", "TryGetCode", "TryGetCodeUnchecked", "constant_pool",
  "relocation_info", "unchecked_relocation_info", "relocation_start", "relocation_end", "relocation_size",
  "body_end", "SizeFor", "FromTargetAddress", "FromEntryAddress", "Relocate", "Initialize",
  "Finalize", "IsFullyInitialized"])
  need(new RegExp(`\\b${fn}\\(`).test(h), `InstructionStream 方法缺 ${fn}`);
need(h.includes("DECL_PRIMITIVE_GETTER(body_size, uint32_t)"), "InstructionStream 缺 DECL_PRIMITIVE_GETTER(body_size)");

// —— inl/cc 定义面 ——
const inlDefs = (inl.match(/InstructionStream::\w+/g) || []).length;
need(inlDefs >= 25, `instruction-stream-inl.h 定义引用应 ≥25, 实得 ${inlDefs}`);
need(cc.includes("InstructionStream::Relocate("), "cc Relocate 缺失");
need(cc.includes("InstructionStream::Initialize(") || cc.includes("InstructionStream::WriteBarrierPromise InstructionStream::RelocateFromDesc("), "cc RelocateFromDesc/Initialize 缺失");
// FromTargetAddress 语义: 目标地址 - kHeaderSize (经 HeapObject::FromAddress 补 tag)
need(/address - InstructionStream::kHeaderSize/.test(inl), "FromTargetAddress 头回退公式缺失");

const body = `${banner("src/objects/instruction-stream.h.js", "src/objects/instruction-stream.{h,cc,-inl.h}", "基准: 非压缩/恒常池关/校验器关")}
//! 指令流对象 (原版 instruction-stream.{h,cc,-inl.h})
//!
//! InstructionStream 是信任对象但住在 code space (非 trusted space):
//! 头部只读 (写保护页), 体 = 指令段 + 对齐填充 + 元数据段.

// ===== 布局 (ISTREAM_FIELDS, ${raw.length} 项) =====
/// 字段偏移表 (基准配置求值)
pub const ISTREAM_FIELDS: [(&str, usize, usize, &str); ${layout.length}] = [
${layout.map((f) => `  ("${f.name}", ${f.offset}, ${f.size}, "${f.expr.replace(/"/g, "'")}"),`).join("\n")}
];

/// 头大小 (基准配置 kHeaderSize)
pub const K_HEADER_SIZE: usize = ${kHeaderSize};

/// 元数据段对齐 (原版 kMetadataAlignment = kIntSize)
pub const K_METADATA_ALIGNMENT: usize = 4;

/// 头邻接性: 堆上体连续 true / 离堆体连续 false (Code::metadata_start 依赖此对)
pub const K_ON_HEAP_BODY_IS_CONTIGUOUS: bool = true;
pub const K_OFF_HEAP_BODY_IS_CONTIGUOUS: bool = false;

/// 尾部填充 (原版 TrailingPaddingSizeFor: RoundUp<kCodeAlignment>(kHeaderSize+body_size) - kHeaderSize - body_size)
pub const fn trailing_padding_size_for(body_size: usize, k_code_alignment: usize) -> usize {
  k_header_size_padded(K_HEADER_SIZE + body_size, k_code_alignment) - K_HEADER_SIZE - body_size
}

/// 按体大小求对象尺寸 (原版 SizeFor: kHeaderSize + body_size + TrailingPaddingSizeFor)
pub const fn size_for(body_size: usize, k_code_alignment: usize) -> usize {
  K_HEADER_SIZE + body_size + trailing_padding_size_for(body_size, k_code_alignment)
}

const fn k_header_size_padded(v: usize, k_code_alignment: usize) -> usize {
  (v + k_code_alignment - 1) / k_code_alignment * k_code_alignment
}

/// 条目地址对齐补头 (原版 kCodeAlignmentMinusCodeHeader = kCodeAlignment - kHeaderSize;
/// MemoryChunk 首部一次性垫该值使入口对 kCodeAlignment 对齐)
pub const fn k_code_alignment_minus_code_header(k_code_alignment: usize) -> usize {
  k_code_alignment - K_HEADER_SIZE
}

// ===== 方法面 =====
/// 指令起点 (原版 instruction_start: address() + kHeaderSize)
pub fn instruction_start(/* this */) -> usize {
  todo!("就地修订: self 地址 + K_HEADER_SIZE")
}

/// 关联 Code (原版 code(AcquireLoadTag): 受保护槽读)
pub fn code(/* this */) /* -> Tagged<Code> */ {
  todo!("就地修订: acquire 读 kCodeOffset 受保护槽, Smi::zero() 视为未初始化")
}

/// 可失败取 Code (原版 TryGetCode: Smi::zero() → false)
pub fn try_get_code(/* this, out */) -> bool {
  todo!("就地修订: raw 非 Smi 才 Cast")
}

/// 重定位信息 (原版 relocation_info: kRelocationInfoOffset 受保护槽)
pub fn relocation_info(/* this */) /* -> Tagged<TrustedByteArray> */ {
  todo!("就地修订: 受保护槽读")
}

/// 重定位区间 (原版 relocation_start/end/size: 委托重定位字节数组)
pub fn relocation_span(/* this */) -> (usize, usize, i32) {
  todo!("就地修订: (info 地址区间, length)")
}

/// 从代码目标地址还原对象 (原版 FromTargetAddress: address - kHeaderSize + kHeapObjectTag)
pub fn from_target_address(address: usize) -> usize {
  address - K_HEADER_SIZE + 1 /* kHeapObjectTag */
}

/// 从存放地址的位置还原对象 (原版 FromEntryAddress: 解引用一次再走 FromTargetAddress)
pub fn from_entry_address(location_of_address: usize) -> usize {
  let _ = location_of_address;
  todo!("就地修订: 读槽后转发 from_target_address")
}

/// 整体重定位 (原版 cc Relocate: WritableJitAllocation 授权下逐 RelocInfo 平移 delta)
pub fn relocate(/* this, jit_allocation, delta */) {
  todo!("就地修订: WritableRelocIterator 迭代 APPLY_RELOC 模式")
}

/// 从 CodeDesc 装配 (原版 Initialize: 头部 + 指令拷贝 + 元数据布局)
pub fn initialize(/* self, map, body_size, constant_pool_offset, reloc_info */) {
  todo!("就地修订: cc 35 起完整装配序列 (写保护下)") 
}

/// 完成装配 (原版 Finalize: 反向填 Code 槽 + 重定位)
pub fn finalize(/* this, code, reloc_info, desc, heap */) {
  todo!("就地修订: SetTaggedField + RelocateFromDesc")
}

/// 全初始化检查 (原版 IsFullyInitialized)
pub fn is_fully_initialized(/* this */) -> bool {
  todo!("就地修订: 按 body_size 尾部哨兵判定")
}

/// 写屏障承诺 (原版 WriteBarrierPromise: DEBUG 下登记延迟写屏障地址集合)
pub struct WriteBarrierPromise { _private: () }
`;

const p = writeSrc("v8-objects-rest", "instruction_stream.rs", body);
console.log(`instruction-stream.h.js: 已生成 ${p} (must ${checks}: 布局 ${layout.length} 项 K_HEADER_SIZE=${kHeaderSize}, inl 引用 ${inlDefs})`);
