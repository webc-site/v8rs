// 由 tool/src/builtins/generate-bytecodes-builtins-list.js 从 v8/src/builtins/generate-bytecodes-builtins-list.cc 生成; 修订就地改, 复跑覆盖
// 常量直译 + 函数签名骨架 (字节码内建清单生成器)
//! 任务映射: v8-builtins-misc; 聚合基线另归 v8-builtins-core (并存不覆盖)

/// 非法字节码 handler 哨兵 (原版 const int kIllegalBytecodeHandler)
pub const K_ILLEGALBYTECODEHANDLER: i32 = -1; // 原版: kIllegalBytecodeHandler
pub const K_ILLEGALBYTECODEHANDLERENCODING: i32 = 255; // 原版: kIllegalBytecodeHandlerEncoding

/// 偏移表大小 (原版 kTableSize = BytecodeOperands::kOperandScaleCount * Bytecodes::kBytecodeCount; 依赖字节码计数, 不在此硬编)
pub const K_TABLE_SIZE_EXPR: &str =
  "BytecodeOperands::kOperandScaleCount * Bytecodes::kBytecodeCount";

/// 函数签名骨架 (名, 返回种类, 原版行号)
pub const FUNCTIONS: &[(&str, &str, u32)] = &[
  ("WriteBytecode", "void", 17),
  ("WriteHeader", "void", 45),
  ("main", "int", 117),
];

/// 该 .cc 为构建期工具: 展开字节码内建清单头 (BUILTIN_LIST_BYTECODE_HANDLERS), 运行时语义不在迁移范围
pub const IS_BUILD_TIME_TOOL: bool = true;

// —— 对数断言 ——
const _: () = {
  assert!(K_ILLEGALBYTECODEHANDLER == -1);
  assert!(K_ILLEGALBYTECODEHANDLERENCODING == 255);
  assert!(FUNCTIONS.len() == 3);
};
