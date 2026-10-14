#!/usr/bin/env bun
// tool/src/wasm/local-decl-encoder.h.js
// 机械提取 v8/src/wasm/local-decl-encoder.{h,cc}: LocalDeclEncoder 签名骨架

import { readV8, writeSrc, banner } from "../../lib.js";

const hSrc = readV8("src/wasm/local-decl-encoder.h");
const ccSrc = readV8("src/wasm/local-decl-encoder.cc");

// 头: 类方法清单
const cls = hSrc.match(/class V8_EXPORT_PRIVATE LocalDeclEncoder\s*\{([\s\S]*?)\n\};/);
if (!cls) throw new Error("未找到 LocalDeclEncoder 类体");
const body = cls[1];
const expectedMethods = ["Emit", "AddLocals", "Size", "has_sig", "set_sig"];
for (const m of expectedMethods) {
  if (!body.includes(m)) throw new Error(`方法缺失: ${m}`);
}

// cc: 方法定义锚点 (Emit/AddLocals/Size) 与 LEBHelper 依赖
const emitMethods = [...ccSrc.matchAll(/^(?:size_t|uint32_t) LocalDeclEncoder::(\w+)\(/gm)].map((m) => m[1]);
if (emitMethods.length !== 3 || emitMethods.join() !== "Emit,AddLocals,Size") {
  throw new Error(`cc 方法锚点漂移: ${emitMethods}`);
}
if (!ccSrc.includes("LEBHelper::write_u32v")) throw new Error("cc 未引用 LEBHelper::write_u32v");

let rust = banner(
  "src/wasm/local-decl-encoder.h.js",
  "src/wasm/local-decl-encoder.{h,cc}",
  "局部声明编码器签名骨架",
);

rust += `
/// 局部声明: (数量, 类型)
pub type LocalDecl = (u32, crate::value_type::ValueType);

/// 局部声明编码器 (对应 class LocalDeclEncoder, 签名骨架)
pub struct LocalDeclEncoder {
  /// 函数签名 (const FunctionSig* sig_)
  sig_: Option<crate::wasm_opcodes::SigDef>,
  /// ZoneVector<std::pair<uint32_t, ValueType>> local_decls_
  local_decls_: Vec<LocalDecl>,
  /// 局部总数 size_t total_
  total_: usize,
}

impl LocalDeclEncoder {
  /// 对应 LocalDeclEncoder(zone, sig = nullptr)
  #[must_use]
  pub fn new() -> Self {
    Self { sig_: None, local_decls_: Vec::new(), total_: 0 }
  }

  /// 对应 Emit(uint8_t* buffer) -> size_t; 编码局部声明前缀
  pub fn emit(&self, _buffer: &mut [u8]) -> usize {
    unimplemented!("骨架: 参照原版 local-decl-encoder.cc Emit 修订")
  }

  /// 对应 AddLocals(count, type) -> uint32_t; 返回新局部起始索引
  pub fn add_locals(&mut self, count: u32, type_: crate::value_type::ValueType) -> u32 {
    unimplemented!("骨架: 参照原版 AddLocals 修订")
  }

  /// 对应 Size() -> size_t
  #[must_use]
  pub fn size(&self) -> usize {
    unimplemented!("骨架: 参照原版 Size 修订")
  }

  /// 对应 has_sig()
  #[must_use]
  pub const fn has_sig(&self) -> bool {
    self.sig_.is_some()
  }

  /// 对应 set_sig(s)
  pub const fn set_sig(&mut self, s: crate::wasm_opcodes::SigDef) {
    self.sig_ = Some(s);
  }
}

impl Default for LocalDeclEncoder {
  fn default() -> Self {
    Self::new()
  }
}
`;

const p = writeSrc("v8-wasm-core", "local_decl_encoder.rs", rust);
console.log(`written ${p}`);
