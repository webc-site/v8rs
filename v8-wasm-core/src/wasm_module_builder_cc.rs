// 由 tool/src/wasm/wasm-module-builder.cc.js 从 v8/src/wasm/wasm-module-builder.cc 生成; 修订就地改, 复跑覆盖
// 模块二进制发射协议 (骨架在 wasm_module_builder.rs.skel)

//! wasm 模块构建器的二进制编码规则。

/// 节协议: 节码 u8 + 填充 varint 占位; 节体完成后回填 (offset - start - kPaddedVarInt32Size)
/// v8: src/wasm/wasm-module-builder.cc EmitSection / FixupSection
pub const SECTION_PROTOCOL: &str =
  "write_u8(code) + reserve_u32v 占位; 修补 = offset - start - kPaddedVarInt32Size";

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
pub const GC_PREFIX_ASSERT: &str =
  "(kExprArrayNewFixed >> 8) == kGCPrefix && (kExprArrayNewFixed & 0x80) == 0";

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
  assert!(PREFIX_ENCODING.len() == 3);
  assert!(TYPE_SECTION_FIELDS.len() == 4);
};
