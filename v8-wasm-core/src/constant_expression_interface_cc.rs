// 由 tool/src/wasm/constant-expression-interface.cc.js 从 v8/src/wasm/constant-expression-interface.cc 生成; 修订就地改, 复跑覆盖
// 常量表达式求值操作表 (骨架在 constant_expression_interface.rs.skel)

//! 常量表达式接口: 解码回调求值。

/// 单目操作表 (2 项): extern.convert_any / any.convert_extern (JS<->wasm 对象包装)
/// v8: src/wasm/constant-expression-interface.cc UnOp
#[rustfmt::skip]
pub static UN_OPS: &[&str] = &[
    "kExprExternConvertAny",
    "kExprAnyConvertExtern",
];

/// 双目操作表 (6 项): 全部回绕算术 (Add/Sub/Mul x I32/I64)
/// v8: src/wasm/constant-expression-interface.cc BinOp
#[rustfmt::skip]
pub static BIN_OPS: &[&str] = &[
    "kExprI32Add",
    "kExprI32Sub",
    "kExprI32Mul",
    "kExprI64Add",
    "kExprI64Sub",
    "kExprI64Mul",
];

/// i31ref 的 Smi 编码: 接受越界输入, 手工构造 Smi
/// 31 位 Smi: raw << (kSmiTagSize + kSmiShiftSize);
/// 32 位 Smi: (raw << (kSmiTagSize + kSmiShiftSize + 1)) >> 1 (算术右移符号扩展次高位)
/// v8: src/wasm/constant-expression-interface.cc RefI31
pub const I31_SMI_SHIFT_31BIT: &str = "raw << (kSmiTagSize + kSmiShiftSize)";
pub const I31_SMI_SHIFT_32BIT: &str = "(raw << (kSmiTagSize + kSmiShiftSize + 1)) as intptr >> 1";

/// end 提前停机: 常量表达式长度未知, 遇 end 将解码终点设为 pc+1
/// v8: src/wasm/constant-expression-interface.cc DoReturn
pub const END_EARLY_STOP: &str = "set_end(pc + 1)";

/// RefFunc 在无 isolate (仅校验) 时只登记函数为 declared
/// v8: src/wasm/constant-expression-interface.cc RefFunc
pub const REFFUNC_DECLARE_ONLY: &str = "functions[i].declared = true";

/// ref.null 按类型选 wasm_null / 普通 null
/// v8: src/wasm/constant-expression-interface.cc RefNull
pub const NULL_VARIANT_RULE: &str = "use_wasm_null ? wasm_null : null_value";

// —— 提取不变式断言 ——
const _: () = {
  assert!(UN_OPS.len() == 2);
  assert!(BIN_OPS.len() == 6);
};
