#!/usr/bin/env bun
// tool/src/regexp/regexp-bytecode-iterator.cc.js — v8/src/regexp/regexp-bytecode-iterator.cc
//   → v8-regexp-exec/src/bytecode_iterator_cc_skel.rs
// 机械提取: 构造/析构/GC 回调协议 (AddGCEpilogueCallback/RemoveGCEpilogueCallback
//   配对) + UpdatePointers 距离保持算法锚 + DEBUG 慢断言的偏移合法性协议;
// 对数断言: 方法定义 4 / GC 回调配对 / 构造双形式。
import { banner, readV8, writeSrc } from "../../lib.js";

const cc = readV8("src/regexp/regexp-bytecode-iterator.cc");

// 方法定义 5: 构造×2 (重载) / 析构 / UpdatePointersCallback / UpdatePointers
const defs = [...cc.matchAll(/^(?:void )?BytecodeIterator::(~?\w+)\(/gm)].map((r) => r[1]);
if (defs.length !== 5) throw new Error(`方法定义应 5 (ctor×2/dtor/Callback/UpdatePointers), 实数 ${defs.length}`);
if (defs.filter((d) => d.startsWith("~")).length !== 1) throw new Error("析构应 1");
// ctor 双形式: 委托形式
if (!cc.includes("BytecodeIterator::BytecodeIterator(DirectHandle<TrustedByteArray> bytecode)\n    : BytecodeIterator(bytecode, 0) {}")) {
  throw new Error("ctor 委托形式漂移");
}
// GC epilogue 回调配对
if (!cc.includes("AddGCEpilogueCallback(\n      UpdatePointersCallback, this);") && !cc.includes("AddGCEpilogueCallback(UpdatePointersCallback, this)")) {
  throw new Error("GC epilogue 注册漂移");
}
if (!cc.includes("RemoveGCEpilogueCallback(\n      UpdatePointersCallback, this);") && !cc.includes("RemoveGCEpilogueCallback(UpdatePointersCallback, this)")) {
  throw new Error("GC epilogue 注销漂移");
}
// UpdatePointersCallback 静态转发
if (!cc.includes("reinterpret_cast<BytecodeIterator*>(iterator)->UpdatePointers();")) {
  throw new Error("回调静态转发漂移");
}
// 距离保持算法: cursor 相对 end 的距离在新缓冲上重建
if (!cc.includes("size_t distance_to_end = end_ - cursor_;")) throw new Error("距离保持算法锚缺失");
if (!cc.includes("cursor_ = end - distance_to_end;")) throw new Error("游标重建公式漂移");
// 仅当基址移动才更新
if (!cc.includes("if (start != start_)")) throw new Error("基址移动判定漂移");

// DEBUG 慢断言协议 (offset 不得指向字节码中部)
if (!cc.includes("#ifdef DEBUG")) throw new Error("DEBUG 门控缺失");
if (!cc.includes("while (cursor_ < start_with_offset) advance();")) throw new Error("慢推进循环漂移");
if (!cc.includes("DCHECK_EQ(cursor_, start_with_offset);")) throw new Error("偏移落点断言漂移");
if (!cc.includes("DCHECK_LT(offset, bytecode->ulength().value());")) throw new Error("偏移范围断言漂移");

console.log(`regexp-bytecode-iterator.cc: 方法定义 ${defs.length}, GC 回调配对 2, DEBUG 协议 3 锚 ✓`);

// ── 生成 Rust ──────────────────────────────────────────────────────────────
const body = `${banner(
  "src/regexp/regexp-bytecode-iterator.cc.js",
  "src/regexp/regexp-bytecode-iterator.cc",
  "迭代器生命周期/GC 移动协议骨架; Rust 侧借用切片天然免 GC 回调, 语义面就地修订",
)}
//! 原版协议 (cc 全文 63 行):
//! - 构造: 注册 GC epilogue 回调, DEBUG 下慢速推进验证 offset 落在字节码边界
//! - 析构: 注销回调 (严格配对)
//! - UpdatePointers: 缓冲基址被 GC 移动时, 以 cursor→end 距离在新基址上重建游标
//!
//! Rust 对策: \`&[u8]\` 借用使移动不可能, 回调协议整体消失;
//! 保留 offset 合法性验证 (advance 推进到 offset 且必须恰齐)。

/// 迭代器生命周期方法面 (原版 4 定义)
pub const ITERATOR_LIFECYCLE: [&str; ${defs.length}] = [
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
`;

writeSrc("v8-regexp-exec", "bytecode_iterator_cc_skel.rs", body);
console.log("written: v8-regexp-exec/src/bytecode_iterator_cc_skel.rs");
