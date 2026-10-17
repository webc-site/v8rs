// 由 tool/src/ast/scopes.h.js 从 v8/src/ast/scopes.h 生成; 修订就地改, 复跑覆盖
// 作用域枚举与骨架

// 语义面: Scope/DeclarationScope 变量解析 (~1700 行) 登记跳过,
// 由手写 scopes.rs 权威覆盖; 本文件登记枚举与布局登记面。

/// 反序列化模式 (原 enum class DeserializationMode)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum DeserializationMode {
  /// 含变量
  IncludingVariables = 0,
  /// 仅作用域
  ScopesOnly = 1,
}

/// 稀疏变量槽 (原 enum class RareVariable = offsetof(RareData, 字段)):
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum RareVariable {
  /// kThisFunction = offsetof(RareData, this_function)
  ThisFunction = 0,
  /// kGeneratorObject = offsetof(RareData, generator_object)
  GeneratorObject = 0,
}
// 注: 槽位偏移依赖 RareData 内存布局 (this_function/generator_object 两 Variable* 槽),
// Rust 侧建议以显式槽位数组替代 offsetof 语义, 由 Developer 裁定。

/// 作用域可用位标记 (原 kAvailableBits)
pub const AVAILABLE_BITS: i32 = 1;

/// 原 RareData — 两槽稀疏变量表
pub struct RareData;

/// 原 class Scope 骨架 (变量登记/查找/反序列化) — 手写 scopes.rs 权威。
pub struct Scope;

// —— 提取不变式断言 ——
const _: () = {
  assert!(AVAILABLE_BITS == 1);
};
