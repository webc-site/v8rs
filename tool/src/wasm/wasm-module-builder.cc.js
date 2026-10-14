#!/usr/bin/env bun
// tool/src/wasm/wasm-module-builder.cc.js
// 机械提取 v8/src/wasm/wasm-module-builder.cc 的节发射/修补协议、
// 前缀操作码编码、堆类型编码与类型节字段序。
// 骨架在 v8-wasm-core/src/wasm_module_builder.rs.skel (wasm-module-builder.h.js)。

import { readV8, writeSrc, banner } from "../../lib.js";

const src = readV8("src/wasm/wasm-module-builder.cc");

// ---------- 1. 节发射/修补协议 ----------
if (!src.includes("// Emit a section code and the size as a padded varint that can be patched")) {
  throw new Error("节发射协议注释缺失");
}
if (!src.includes("buffer->write_u8(code);")) throw new Error("节码 u8 缺失");
if (!src.includes("return buffer->reserve_u32v();")) throw new Error("占位 varint 缺失");
if (!/patch_u32v\(start, static_cast<uint32_t>\(buffer->offset\(\) - start -\s*kPaddedVarInt32Size\)\);/.test(src)) {
  throw new Error("节长修补公式缺失");
}

// ---------- 2. 前缀操作码编码 (三轨) ----------
if (!src.includes("DCHECK_GT(opcode, 0xFF);")) throw new Error("前缀前提缺失");
if (!src.includes("DCHECK_EQ(kSimdPrefix, opcode >> 12);")) throw new Error("SIMD 前缀轨缺失");
if (!src.includes("body_.write_u32v(opcode & 0xFFF);")) throw new Error("SIMD 尾 12 位缺失");
if (!src.includes("body_.write_u8(opcode >> 8);      // Prefix.")) throw new Error("普通前缀轨缺失");
if (!src.includes("body_.write_u32v(opcode & 0xff);  // LEB encoded tail.")) throw new Error("LEB 尾轨缺失");

// ---------- 3. 堆类型编码: exact 前置 kExactCode ----------
if (!src.includes("if (type.encoding_needs_exact()) buffer->write_u8(kExactCode);")) {
  throw new Error("exact 堆类型编码缺失");
}

// ---------- 4. GC 前缀操作码断言 (ArrayNewFixed) ----------
if (!src.includes("static_assert((kExprArrayNewFixed >> 8) == kGCPrefix);")) {
  throw new Error("GC 前缀断言缺失");
}
if (!src.includes("static_assert((kExprArrayNewFixed & 0x80) == 0);")) {
  throw new Error("LEB 单字节断言缺失");
}

// ---------- 5. 模块前导 ----------
if (!src.includes("buffer->write_u32(kWasmMagic);")) throw new Error("magic 前导缺失");
if (!src.includes("buffer->write_u32(kWasmVersion);")) throw new Error("version 前导缺失");

// ---------- 6. 类型节字段序 (recgroup/supertype/describes/descriptor) ----------
for (const f of [
  "buffer->write_u32v(next_rec_group->size);",
  "buffer->write_u32v(type.supertype);",
  "buffer->write_u32v(type.describes);",
  "buffer->write_u32v(type.descriptor);",
]) {
  if (!src.includes(f)) throw new Error(`类型节字段缺失: ${f}`);
}

// ---------- 7. 初始化表达式: 字符串常量走 kExprStringConst & 0xFF ----------
if (!src.includes("buffer->write_u32v(kExprStringConst & 0xFF);")) {
  throw new Error("字符串常量编码缺失");
}

// ---------- 8. 直接 emit 的操作码上限 ----------
if (!src.includes("DCHECK_LE(opcode, 0xFF);")) throw new Error("单字节操作码上限缺失");

console.log("wasm-module-builder: 节协议=1 前缀轨=3 前导=2");

// ---------- 生成 Rust ----------
const rust = `${banner(
  "src/wasm/wasm-module-builder.cc.js",
  "src/wasm/wasm-module-builder.cc",
  "模块二进制发射协议 (骨架在 wasm_module_builder.rs.skel)",
)}
//! wasm 模块构建器的二进制编码规则。

/// 节协议: 节码 u8 + 填充 varint 占位; 节体完成后回填 (offset - start - kPaddedVarInt32Size)
/// v8: src/wasm/wasm-module-builder.cc EmitSection / FixupSection
pub const SECTION_PROTOCOL: &str = "write_u8(code) + reserve_u32v 占位; 修补 = offset - start - kPaddedVarInt32Size";

/// 前缀操作码三轨编码
/// v8: src/wasm/wasm-module-builder.cc WasmFunctionBuilder::EmitWithPrefix
#[rustfmt::skip]
pub static PREFIX_ENCODING: &[(&str, &str)] = &[
    ("<= 0xFF", "单字节直发"),
    ("> 0xFFFF (SIMD)", "kSimdPrefix 字节 + u32v(opcode & 0xFFF)"),
    ("0x100..=0xFFFF", "prefix 字节 (opcode >> 8) + u32v(opcode & 0xFF)"),
];

/// 堆类型编码: 需要 exact 时先写 kExactCode 再 LEB 编码 type.code()
/// v8: src/wasm/wasm-module-builder.cc WriteHeapType
pub const HEAP_TYPE_ENCODING: &str = "exact => kExactCode 前置 + write_i32v(code)";

/// GC 断言: kExprArrayNewFixed >> 8 == kGCPrefix 且低 8 位无符号扩展位 (& 0x80 == 0)
/// v8: src/wasm/wasm-module-builder.cc static_assert
pub const GC_PREFIX_ASSERT: &str = "(kExprArrayNewFixed >> 8) == kGCPrefix && (kExprArrayNewFixed & 0x80) == 0";

/// 模块前导: kWasmMagic + kWasmVersion (各 u32)
/// v8: src/wasm/wasm-module-builder.cc WriteTo
pub const MODULE_PRELUDE: &str = "u32(kWasmMagic) + u32(kWasmVersion)";

/// 类型节字段序: recgroup 尺寸 -> (supertype, describes, descriptor, 类型定义)
/// v8: src/wasm/wasm-module-builder.cc 类型节发射
#[rustfmt::skip]
pub static TYPE_SECTION_FIELDS: &[&str] = &[
    "next_rec_group->size",
    "type.supertype",
    "type.describes",
    "type.descriptor",
];

/// 初始化表达式中的字符串常量: kExprStringConst & 0xFF + 索引
/// v8: src/wasm/wasm-module-builder.cc 初始化表达式发射
pub const STRING_CONST_ENCODING: &str = "kExprStringConst & 0xFF + u32v(index)";

// —— 提取不变式断言 ——
const _: () = {
    assert_eq!(PREFIX_ENCODING.len(), 3);
    assert_eq!(TYPE_SECTION_FIELDS.len(), 4);
};
`;

const p = writeSrc("v8-wasm-core", "wasm_module_builder_cc.rs", rust);
console.log(`written ${p}`);
