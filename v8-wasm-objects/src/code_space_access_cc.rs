// 由 tool/src/wasm/code-space-access.cc.js 从 v8/src/wasm/code-space-access.cc 生成; 修订就地改, 复跑覆盖
// wasm 代码空间写保护作用域理由串与骨架

//! wasm 代码空间写保护: 构造时进入 RWX 写作用域, 析构时恢复。

use v8_common::RwxMemoryWriteScope;

/// 写保护理由串 (透传给底层 RwxMemoryWriteScope, 出现在崩溃诊断中)
/// v8: src/wasm/code-space-access.cc CodeSpaceWriteScope::CodeSpaceWriteScope
pub const CODE_SPACE_WRITE_REASON: &str = "For wasm::CodeSpaceWriteScope.";

/// 代码空间写保护作用域 (对接 v8-common 写保护件)
/// v8: src/wasm/code-space-access.cc CodeSpaceWriteScope
/// 原版 h:43 V8_NODISCARD — 作用域对象即时 drop 会静默失去写保护窗口, 编译期禁用
#[must_use = "CodeSpaceWriteScope 即时 drop 会立刻恢复只读, 失去保护窗口 (原版 V8_NODISCARD)"]
pub struct CodeSpaceWriteScope {
  /// 底层 RWX 写作用域; Drop 自动恢复权限 (原版成员析构语义)
  pub inner: RwxMemoryWriteScope,
}

impl CodeSpaceWriteScope {
  /// v8: src/wasm/code-space-access.cc:11-12 CodeSpaceWriteScope::CodeSpaceWriteScope
  #[must_use = "CodeSpaceWriteScope 即时 drop 会立刻恢复只读, 失去保护窗口 (原版 V8_NODISCARD)"]
  pub fn new() -> Self {
    Self {
      inner: RwxMemoryWriteScope::new(CODE_SPACE_WRITE_REASON),
    }
  }
}

impl Default for CodeSpaceWriteScope {
  fn default() -> Self {
    Self::new()
  }
}

// —— 提取不变式断言 ——
// 注: 本版工具链 const 上下文不支持 assert_eq! (非 const assert_failed), 折叠为 assert! 等式
const _: () = {
  assert!(CODE_SPACE_WRITE_REASON.len() == 30);
};
