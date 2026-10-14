#!/usr/bin/env bun
// tool/src/wasm/code-space-access.h.js
// 机械提取 v8/src/wasm/code-space-access.{h,cc} 的 CodeSpaceWriteScope 契约

import { readV8, writeSrc, banner } from "../../lib.js";

const hSrc = readV8("src/wasm/code-space-access.h");
const ccSrc = readV8("src/wasm/code-space-access.cc");

// 1. h: 类签名与实现策略
if (!hSrc.includes("class V8_NODISCARD CodeSpaceWriteScope final")) {
  throw new Error("未找到 CodeSpaceWriteScope");
}
if (!hSrc.includes("RwxMemoryWriteScope rwx_write_scope_;")) {
  throw new Error("未找到 rwx_write_scope_ 成员 (组合实现)");
}
for (const strat of [
  "APRR/MAP_JIT",
  "Intel PKU",
  "{mprotect()} between R-X and RWX",
]) {
  if (!hSrc.includes(strat)) throw new Error(`策略注释缺失: ${strat}`);
}

// 2. cc: reason 字符串
const reasonMatch = ccSrc.match(/rwx_write_scope_\("([^"]*)"\)/);
if (!reasonMatch) throw new Error("未找到 reason 实参");
const reason = reasonMatch[1];

// 3. 删除拷贝 (RAII)
if (!hSrc.includes("CodeSpaceWriteScope(const CodeSpaceWriteScope&) = delete;")) {
  throw new Error("未找到删除拷贝构造");
}

console.log(`code-space-access: reason="${reason}"`);

// 4. 生成 Rust
let rust = banner(
  "src/wasm/code-space-access.h.js",
  "src/wasm/code-space-access.{h,cc}",
  "CodeSpaceWriteScope W^X 写域契约",
);

rust += `
//! wasm 代码空间写保护作用域: 组合复用 RwxMemoryWriteScope, 自身无状态。
/// v8: src/wasm/code-space-access.h CodeSpaceWriteScope

/// 传给底层 RwxMemoryWriteScope 的 reason 字符串
/// v8: src/wasm/code-space-access.cc CodeSpaceWriteScope::CodeSpaceWriteScope
pub const CODE_SPACE_WRITE_SCOPE_REASON: &str = "${reason}";

/// 三种平台策略枚举 (原版注释归档, 运行时按平台择一)
/// v8: src/wasm/code-space-access.h 类注释
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CodeSpaceWriteImplementation {
    /// Apple Silicon: APRR/MAP_JIT, 线程本地, 真 W^X
    AppleMprrMapJit,
    /// Intel PKU: 保护键切换写权限, 线程本地, 可执行权限不可收回
    IntelPku,
    /// 兜底: mprotect 在 R-X 与 RWX 间切换, 进程级, 慢
    Mprotect,
}

/// RAII 域骨架: 进入取写权限, 离开恢复 (可嵌套计数)
/// v8: src/wasm/code-space-access.h CodeSpaceWriteScope
pub struct CodeSpaceWriteScope {
    /// v8: 成员 rwx_write_scope_ (组合)
    rwx_write_scope: RwxMemoryWriteScope,
}

// —— 提取不变式断言 ——
const _: () = {
    assert_eq!(CODE_SPACE_WRITE_SCOPE_REASON, "For wasm::CodeSpaceWriteScope.");
};
`;

const p = writeSrc("v8-wasm-core", "code_space_access.rs.skel", rust);
console.log(`written ${p}`);
