#!/usr/bin/env bun
// tool/src/wasm/code-space-access.cc.js
// 机械提取 v8/src/wasm/code-space-access.cc 的写保护作用域理由串与转发骨架。
// 槽位布局常量在头侧 (v8-wasm-core/src/code_space_access.rs.skel 由 code-space-access.h.js 生成)。

import { readV8, writeSrc, banner } from "../../lib.js";

const ccSrc = readV8("src/wasm/code-space-access.cc");

// 1. 写保护理由串 (构造转发到 RwxMemoryWriteScope)
const REASON = "For wasm::CodeSpaceWriteScope.";
if (!ccSrc.includes(`rwx_write_scope_("${REASON}")`)) {
  throw new Error("未找到 rwx 写保护理由串转发");
}

// 2. 全文件只有这一个构造函数, 无其他规则面
const ctors = [...ccSrc.matchAll(/CodeSpaceWriteScope::CodeSpaceWriteScope\(/g)];
if (ctors.length !== 1) throw new Error(`构造函数 ${ctors.length} != 1`);

console.log("code-space-access: 理由串=1 构造=1");

// 3. 生成 Rust
const rust = `${banner(
  "src/wasm/code-space-access.cc.js",
  "src/wasm/code-space-access.cc",
  "wasm 代码空间写保护作用域理由串与骨架",
)}
//! wasm 代码空间写保护: 构造时进入 RWX 写作用域, 析构时恢复。

/// 写保护理由串 (透传给底层 RwxMemoryWriteScope, 出现在崩溃诊断中)
/// v8: src/wasm/code-space-access.cc CodeSpaceWriteScope::CodeSpaceWriteScope
pub const CODE_SPACE_WRITE_REASON: &str = "${REASON}";

/// 代码空间写保护作用域 (骨架)
/// v8: src/wasm/code-space-access.cc CodeSpaceWriteScope
pub struct CodeSpaceWriteScope {
    /// 底层 RWX 写作用域 (理由串透传; 类型待 Developer 修订对接 v8-common 写保护件)
    pub inner: RwxMemoryWriteScope,
}

impl CodeSpaceWriteScope {
    /// v8: src/wasm/code-space-access.cc CodeSpaceWriteScope::CodeSpaceWriteScope
    pub fn new() -> Self {
        todo!("就地修订: 转发 RwxMemoryWriteScope::new(CODE_SPACE_WRITE_REASON)")
    }
}

// —— 提取不变式断言 ——
const _: () = {
    assert_eq!(CODE_SPACE_WRITE_REASON, "For wasm::CodeSpaceWriteScope.");
};
`;

const p = writeSrc("v8-wasm-objects", "code_space_access_cc.rs", rust);
console.log(`written ${p}`);
