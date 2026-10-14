#!/usr/bin/env bun
// tool/src/wasm/function-body-decoder.h.js
// 机械提取 v8/src/wasm/function-body-decoder.{h,cc}: FunctionBody/BodyLocalDecls/迭代器骨架

import { readV8, writeSrc, banner } from "../../lib.js";

const hSrc = readV8("src/wasm/function-body-decoder.h");
const ccSrc = readV8("src/wasm/function-body-decoder.cc");

const fb = hSrc.match(/struct FunctionBody\s*\{([\s\S]*?)\n\};/);
if (!fb) throw new Error("未找到 FunctionBody");
const fbFields = [...fb[1].matchAll(/^\s*(?:const\s+)?[\w:<>,\*&\s]+?\b(\w+);/gm)].map((m) => m[1]);
if (fbFields.length !== 4 || fbFields.join() !== "sig,offset,start,end") {
  throw new Error(`FunctionBody 字段漂移: ${fbFields}`);
}

const lt = hSrc.match(/enum class LoadTransformationKind : uint8_t \{([^}]*)\};/);
if (!lt) throw new Error("未找到 LoadTransformationKind");
const ltk = lt[1].split(",").map((s) => s.trim()).filter(Boolean);
if (ltk.length !== 3 || ltk[0] !== "kSplat") throw new Error("LoadTransformationKind 漂移");

// cc 侧函数锚点
for (const fn of ["ValidateFunctionBody", "DecodeLocalDecls", "BytecodeIterator"]) {
  if (!ccSrc.includes(fn) && !hSrc.includes(fn)) throw new Error(`锚点缺失: ${fn}`);
}

console.log(`FunctionBody 字段: ${fbFields.join(", ")}`);

let rust = banner(
  "src/wasm/function-body-decoder.h.js",
  "src/wasm/function-body-decoder.{h,cc}",
  "函数体解码入口: FunctionBody 布局 + LoadTransformationKind + 迭代器骨架",
);

rust += `
/// 函数体定位 (对应 struct FunctionBody; 字段直译: sig/offset/start/end)
#[derive(Debug, Clone, Copy)]
pub struct FunctionBody {
  /// 函数签名 (const FunctionSig* sig)
  pub sig: Option<crate::wasm_opcodes::SigDef>,
  /// 模块字节内偏移 (错误报告定位)
  pub offset: u32,
  /// 函数体起始指针 (以 wire bytes 偏移表示)
  pub start: u32,
  /// 函数体结束
  pub end: u32,
}

/// load 变换种类 (对应 enum class LoadTransformationKind : uint8_t)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum LoadTransformationKind {
  /// kSplat
  Splat = 0,
  /// kExtend
  Extend = 1,
  /// kZeroExtend
  ZeroExtend = 2,
}

/// 局部声明解码结果 (对应 struct BodyLocalDecls)
#[derive(Debug, Clone, Default)]
pub struct BodyLocalDecls {
  /// (数量, 类型) 序列
  pub locals: Vec<(u32, crate::value_type::ValueType)>,
  /// 局部声明段之后的解码位置
  pub decoded_offset: u32,
}

/// 函数体校验 (对应 ValidateFunctionBody, 骨架)
pub fn validate_function_body(_body: &FunctionBody) -> Result<(), crate::error::WasmError> {
  unimplemented!("骨架: 参照原版 function-body-decoder.cc ValidateFunctionBody 修订")
}

/// 解码局部声明 (对应 DecodeLocalDecls, 不校验, 骨架)
pub fn decode_local_decls(_body: &FunctionBody) -> BodyLocalDecls {
  unimplemented!("骨架: 参照原版 DecodeLocalDecls 修订")
}

/// 字节码迭代器 (对应 class BytecodeIterator, 骨架: opcode/offset 两种视图)
pub struct BytecodeIterator {
  pub pc: u32,
  pub end: u32,
}

impl BytecodeIterator {
  /// 对应 reset(pc)
  pub fn reset(&mut self, pc: u32) {
    self.pc = pc;
  }

  /// 对应 has()
  #[must_use]
  pub fn has(&self) -> bool {
    self.pc < self.end
  }

  /// 对应 opcode() (骨架)
  pub fn opcode(&self) -> crate::wasm_opcodes::WasmOpcode {
    unimplemented!("骨架: LEB 前进与 opcode 读取待修订")
  }

  /// 对应 pc_to_next()
  pub fn pc_to_next(&mut self) {
    unimplemented!("骨架: 参照原版 pc_to_next 修订")
  }
}

/// SIMD 硬件支持检查 (对应 CheckHardwareSupportsSimd, 骨架)
pub fn check_hardware_supports_simd() -> bool {
  unimplemented!("骨架: 参照原版修订 (x64/arm64 均默认支持)")
}

// —— 提取不变式断言 ——
const _: () = {
  assert!(LoadTransformationKind::Splat as u8 == 0);
  assert!(LoadTransformationKind::ZeroExtend as u8 == 2);
};
`;
;

const p = writeSrc("v8-wasm-core", "function_body_decoder.rs", rust);
console.log(`written ${p}`);
