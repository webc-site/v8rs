// 由 tool/src/wasm/names-provider.h.js 从 v8/src/wasm/names-provider.h 生成; 修订就地改, 复跑覆盖
// 名称提供器的显示行为枚举

/// 函数名展示行为 (对应原版 NamesProvider::FunctionNamesBehavior, enum : bool)
pub type FunctionNamesBehavior = bool;

/// kWasmInternal — 仅 name section 原始名
pub const WASM_INTERNAL: FunctionNamesBehavior = false;
/// kDevTools — 前缀 '$', 以导入/导出名回退, 缺省 "$funcN"
pub const DEV_TOOLS: FunctionNamesBehavior = true;

/// 是否将索引作为注释附带 (对应原版 NamesProvider::IndexAsComment, enum : bool)
pub type IndexAsComment = bool;

/// kDontPrintIndex
pub const DONT_PRINT_INDEX: IndexAsComment = false;
/// kIndexAsComment
pub const INDEX_AS_COMMENT: IndexAsComment = true;

// —— 提取不变式断言 ——
const _: () = {
  assert!(DEV_TOOLS);
  assert!(!DONT_PRINT_INDEX);
};
