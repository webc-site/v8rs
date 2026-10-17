#!/usr/bin/env bun
// tool/src/wasm/struct-types.h.js
// 机械提取 v8/src/wasm/struct-types.h 的结构体/数组/cont 类型签名与字段布局算法常量

import { readV8, writeSrc, banner } from "../../lib.js";

const hSrc = readV8("src/wasm/struct-types.h");

// 1. 类签名锚 (8 个类型)
const classes = [
  "class StructTypeBase : public ZoneObject",
  "class StructType : public StructTypeBase",
  "class CanonicalStructType : public StructTypeBase",
  "class ArrayTypeBase : public ZoneObject",
  "class ArrayType : public ArrayTypeBase",
  "class CanonicalArrayType : public ArrayTypeBase",
  "class ContType : public ZoneObject",
  "class CanonicalContType : public ZoneObject",
];
for (const c of classes) {
  if (!hSrc.includes(c)) throw new Error(`类缺失: ${c}`);
}

// 2. field_count_ 为 u16 且有上限断言
if (!hSrc.includes("static_assert(kV8MaxWasmStructFields < std::numeric_limits<uint16_t>::max());")) {
  throw new Error("未找到 field_count_ u16 上限断言");
}
if (!hSrc.includes("const uint16_t field_count_;")) throw new Error("未找到 uint16_t field_count_");

// 3. Align 公式: RoundUp(offset, min(alignment, shared ? kDoubleSize : kTaggedSize))
const alignMatch = hSrc.match(
  /return RoundUp\(\s*offset,\s*std::min\(alignment,\s*static_cast<uint32_t>\(is_shared \? kDoubleSize : kTaggedSize\)\)\);/,
);
if (!alignMatch) throw new Error("未找到 Align 公式");

// 4. 首字段偏移规则 (descriptor 非合并时预留 kTaggedSize)
if (!hSrc.includes("return is_descriptor() ? kTaggedSize : 0;")) {
  throw new Error("未找到 descriptor 首字段预留 kTaggedSize");
}

// 5. gap 填充算法锚
for (const anchor of [
  "uint32_t gap_position = 0;",
  "uint32_t gap_size = 0;",
  "if (field_size <= gap_size)",
  "if (gap_before > gap_after)",
  "offset = RoundUp(offset, kTaggedSize);",
]) {
  if (!hSrc.includes(anchor)) throw new Error(`gap 算法锚缺失: ${anchor}`);
}

// 6. builder 枚举
const enumMatch = hSrc.match(
  /enum ComputeOffsets : bool \{\s*kComputeOffsets = true,\s*kUseProvidedOffsets = false\s*\};/,
);
if (!enumMatch) throw new Error("未找到 ComputeOffsets 枚举");

// 7. 首字段可作原型判定
const protoMatch = hSrc.match(
  /bool first_field_can_be_prototype\(\) const \{\s*return is_descriptor_ && field_count_ > 0 && !mutabilities_\[0\] &&\s*reps_\[0\]\.is_reference_to\(GenericKind::kExtern\);/,
);
if (!protoMatch) throw new Error("未找到 first_field_can_be_prototype 条件");

// 8. 成员布局: field_count_(u16) is_descriptor_(bool) is_shared_(SharedFlag) field_offsets_(ptr) reps_(ptr) mutabilities_(ptr)
for (const m of [
  "const uint16_t field_count_;",
  "const bool is_descriptor_;",
  "const SharedFlag is_shared_;",
  "uint32_t* const field_offsets_;",
  "const ValueTypeBase* const reps_;",
  "const bool* const mutabilities_;",
]) {
  if (!hSrc.includes(m)) throw new Error(`成员缺失: ${m}`);
}

console.log(`struct-types: 类=${classes.length} 枚举=${enumMatch ? 1 : 0}`);

// 9. 生成 Rust
let rust = banner(
  "src/wasm/struct-types.h.js",
  "src/wasm/struct-types.h",
  "struct/array/cont wasm 类型骨架与字段偏移布局算法",
);

rust += `
//! GC 类型系统的三对类型 (模块相对 / canonical) 与布局计算。
/// v8: src/wasm/struct-types.h StructTypeBase

/// 对齐上限: shared 结构体按 f64(8) 对齐, 否则按 tagged(压缩 4 / 非压缩 8)
/// v8: src/wasm/struct-types.h StructTypeBase::Align
pub fn align_field(offset: u32, alignment: u32, is_shared: bool, tagged_size: u32) -> u32 {
    let cap = if is_shared { k_double_size } else { tagged_size };
    let align = alignment.min(cap);
    offset.div_ceil(align) * align
}

/// 非合并 descriptor 的首字段预留 (kTaggedSize); 合并 descriptors 时为 0
/// v8: src/wasm/struct-types.h field_offset(index == 0)
pub const fn first_field_reserved(is_descriptor: bool, merged_descriptors: bool, tagged_size: u32) -> u32 {
    if merged_descriptors { 0 } else if is_descriptor { tagged_size } else { 0 }
}

/// gap 填充布局器 (保持子类型安全: 公共前缀必得公共偏移)
/// v8: src/wasm/struct-types.h InitializeOffsets
pub fn initialize_offsets(sizes: &[u32], is_shared: bool, tagged_size: u32, out_offsets: &mut [u32]) {
    let n = sizes.len();
    if n == 0 { return; }
    let mut offset = 0usize;
    let mut gap_position = 0usize;
    let mut gap_size = 0usize;
    for i in 1..n {
        let field_size = sizes[i] as usize;
        if field_size <= gap_size {
            let aligned_gap = align_field(gap_position as u32, field_size as u32, is_shared, tagged_size) as usize;
            let gap_before = aligned_gap - gap_position;
            let aligned_gap_size = gap_size - gap_before;
            if field_size <= aligned_gap_size {
                out_offsets[i - 1] = aligned_gap as u32;
                let gap_after = aligned_gap_size - field_size;
                if gap_before > gap_after {
                    gap_size = gap_before;
                } else {
                    gap_position = aligned_gap + field_size;
                    gap_size = gap_after;
                }
                continue;
            }
        }
        let old_offset = offset;
        offset = align_field(offset as u32, field_size as u32, is_shared, tagged_size) as usize;
        let gap = offset - old_offset;
        if gap > gap_size {
            gap_size = gap;
            gap_position = old_offset;
        }
        out_offsets[i - 1] = offset as u32;
        offset += field_size;
    }
    offset = align_field(offset as u32, tagged_size, false, tagged_size) as usize;
    out_offsets[n - 1] = offset as u32;
}

/// builder 是否计算偏移
/// v8: src/wasm/struct-types.h BuilderImpl::ComputeOffsets
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ComputeOffsets {
    /// v8: kComputeOffsets = true
    Compute,
    /// v8: kUseProvidedOffsets = false
    UseProvided,
}

/// 首字段可作 JS 原型判定: descriptor 且首字段不可变 extern 引用
/// v8: src/wasm/struct-types.h first_field_can_be_prototype
pub fn first_field_can_be_prototype(
    is_descriptor: bool,
    field_count: usize,
    first_mutability: bool,
    first_is_extern_ref: bool,
) -> bool {
    is_descriptor && field_count > 0 && !first_mutability && first_is_extern_ref
}

/// v8: src/wasm/struct-types.h kDoubleSize (f64 宽度)
pub const k_double_size: u32 = 8;

// —— 提取不变式断言 ——
const _: () = {
    assert_eq!(k_double_size, 8);
    assert_eq!(first_field_reserved(true, false, 4), 4);
    assert_eq!(first_field_reserved(false, false, 4), 0);
};
`;

const p = writeSrc("v8-wasm-core", "struct_types.rs.skel", rust);
console.log(`written ${p}`);
