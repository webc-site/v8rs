#!/usr/bin/env bun
// tool/src/codegen/code-reference.h.js
// 机械提取 v8/src/codegen/code-reference.{h,cc}: Kind 判别 + 访问器签名骨架
// → v8-codegen-core/src/code_reference.rs

import { readV8, writeSrc, banner } from "../../lib.js";

const hSrc = readV8("src/codegen/code-reference.h");
const ccSrc = readV8("src/codegen/code-reference.cc");

if (!hSrc.includes("enum class Kind { NONE, CODE, WASM_CODE, CODE_DESC }"))
  throw new Error("缺 Kind 枚举");
const accessors = ["constant_pool", "instruction_start", "instruction_end", "instruction_size",
  "relocation_start", "relocation_end", "relocation_size", "code_comments",
  "code_comments_size", "jump_table_info", "jump_table_info_size"];
for (const a of accessors) {
  if (!hSrc.includes(`${a}() const`) && !hSrc.includes(`${a}()`)) throw new Error(`缺访问器 ${a}`);
}
// .cc 以 DISPATCH(ret, method) 宏逐个生成访问器 — 对数: 恰好 11 个 DISPATCH
const dispatchRows = [...ccSrc.matchAll(/^DISPATCH\([\w: *]+, (\w+)\)$/gm)].map((x) => x[1]);
if (dispatchRows.length !== accessors.length)
  throw new Error(`DISPATCH 行 ${dispatchRows.length} != 访问器 ${accessors.length}`);
for (const a of accessors) {
  if (!dispatchRows.includes(a)) throw new Error(`.cc DISPATCH 缺 ${a}`);
}

let rust = banner(
  "src/codegen/code-reference.h.js",
  "src/codegen/code-reference.{h,cc}",
  "CodeReference 判别联合 + 访问器签名骨架",
);

rust += `
/// 原版 enum class Kind: NONE / CODE / WASM_CODE / CODE_DESC
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum Kind {
  NONE = 0,
  CODE = 1,
  WASM_CODE = 2,
  CODE_DESC = 3,
}

/// 原版 CodeReference: 判别联合 (null_/wasm_code_/code_desc_/code_)
/// 指针族以 usize 承载 (地址语义), 生命周期由调用方保证。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CodeReference {
  kind_: Kind,
  /// union 载荷 (指针宽度)
  payload: usize,
}

impl Default for CodeReference {
  /// 原版默认构造: NONE + null
  fn default() -> Self {
    Self { kind_: Kind::NONE, payload: 0 }
  }
}

impl CodeReference {
  /// 原版 CodeReference(wasm_code)
  #[must_use]
  pub const fn from_wasm_code(wasm_code: usize) -> Self {
    Self { kind_: Kind::WASM_CODE, payload: wasm_code }
  }

  /// 原版 CodeReference(code_desc)
  #[must_use]
  pub const fn from_code_desc(code_desc: usize) -> Self {
    Self { kind_: Kind::CODE_DESC, payload: code_desc }
  }

  /// 原版 CodeReference(code: Handle<Code>)
  #[must_use]
  pub const fn from_code(code: usize) -> Self {
    Self { kind_: Kind::CODE, payload: code }
  }

  /// 原版 is_null
  #[inline]
  #[must_use]
  pub const fn is_null(&self) -> bool {
    self.kind_ == Kind::NONE
  }

  /// 原版 is_code
  #[inline]
  #[must_use]
  pub const fn is_code(&self) -> bool {
    self.kind_ == Kind::CODE
  }

  /// 原版 is_wasm_code
  #[inline]
  #[must_use]
  pub const fn is_wasm_code(&self) -> bool {
    self.kind_ == Kind::WASM_CODE
  }

  // —— 访问器骨架 (union 读取需判别, 待 Developer) ——

${accessors
  .map(
    (a) => `  /// 原版 ${a}()
  pub fn ${a}(&self) -> ${a.startsWith("instruction_size") || a.includes("_size") ? "i32" : "usize"} {
    todo!("待 Developer 修订: 对齐 code-reference.cc ${a}")
  }`,
  )
  .join("\n\n")}

  /// 原版 as_code: 判别须为 CODE
  #[must_use]
  pub const fn as_code(&self) -> usize {
    debug_assert!(self.kind_ == Kind::CODE);
    self.payload
  }

  /// 原版 as_wasm_code: 判别须为 WASM_CODE
  #[must_use]
  pub const fn as_wasm_code(&self) -> usize {
    debug_assert!(self.kind_ == Kind::WASM_CODE);
    self.payload
  }
}
`;

const p = writeSrc("v8-codegen-core", "code_reference.rs", rust);
console.log(`code-reference: Kind 4 / 访问器 ${accessors.length} / 转换 2 → ${p}`);
