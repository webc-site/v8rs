// 由 tool/src/regexp/regexp-bytecode-iterator.cc.js 从 v8/src/regexp/regexp-bytecode-iterator.cc 生成; 修订就地改, 复跑覆盖
// 迭代器生命周期/GC 移动协议骨架; Rust 侧借用切片天然免 GC 回调, 语义面就地修订

//! 原版协议 (cc 全文 63 行):
//! - 构造: 注册 GC epilogue 回调, DEBUG 下慢速推进验证 offset 落在字节码边界
//! - 析构: 注销回调 (严格配对)
//! - UpdatePointers: 缓冲基址被 GC 移动时, 以 cursor→end 距离在新基址上重建游标
//!
//! Rust 对策: `&[u8]` 借用使移动不可能, 回调协议整体消失;
//! 保留 offset 合法性验证 (advance 推进到 offset 且必须恰齐)。

/// 迭代器生命周期方法面 (原版 4 定义)
pub const ITERATOR_LIFECYCLE: [&str; 5] = [
  "new (bytecode)", // 委托 new (bytecode, 0)
  "new (bytecode, offset)",
  "drop",
  "update_pointers_callback",
];

/// offset 合法性验证: 从头 advance 到 offset, 必须恰好落在字节码边界
/// (DEBUG 慢断言协议, cc:27-36; Rust 侧保留为 debug_assert)
pub fn debug_verify_offset(bytecode: &[u8], offset: u32, size_of: impl Fn(u8) -> usize) -> bool {
  let mut cursor = 0usize;
  while cursor < offset as usize {
    cursor += size_of(bytecode[cursor]);
  }
  cursor == offset as usize
}

const _: () = assert!(ITERATOR_LIFECYCLE.len() == 4);
