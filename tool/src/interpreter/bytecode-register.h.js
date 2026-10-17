#!/usr/bin/env bun
// tool/src/interpreter/bytecode-register.h.js
// 机械提取 src/interpreter/bytecode-register.{h,cc} 的解释器寄存器编码公式与骨架
// → v8-interpreter/src/bytecode_register.rs
//
// 帧常数依赖项 (InterpreterFrameConstants/StandardFrameConstants, 属 v8-execution-frames
// crate) 以参数注入, 公式原样保留; kSystemPointerSize = 8 (src/common/globals.h)。
import { readV8, writeSrc, banner } from "../../lib.js";
const hSrc = readV8("src/interpreter/bytecode-register.h");
const ccSrc = readV8("src/interpreter/bytecode-register.cc");
// 结构锚
for (const [src, needle, name] of [
  [hSrc, "constexpr int32_t ToOperand() const {", "ToOperand \u516c\u5f0f"],
  [hSrc, "static constexpr Register FromShortStar(Bytecode bytecode) {", "FromShortStar"],
  [hSrc, "static constexpr int kInvalidIndex = kMaxInt;", "kInvalidIndex"],
  [hSrc, "constexpr std::optional<Bytecode> TryToShortStar() const {", "TryToShortStar"],
  [ccSrc, "std::string Register::ToString() const", "ToString \u5b9a\u4e49"],
]) {
  if (!src.includes(needle)) throw new Error(`结构锚缺失: ${name}`);
}

// —— 机械提取: 去注释 + 类块 + 成员声明 (对数守卫内嵌) ——
function stripComments(src) {
  return src.replace(/\/\*[\s\S]*?\*\//g, "").replace(/^\s*\/\/.*$/gm, "").replace(/^\s*#.*$/gm, "");
}
function classBlocks(src) {
  const out = [];
  const re = /\b(class|struct)\s+(?:[A-Z][A-Z_0-9]*\s+)?(\w+)[^{;]*\{/g;
  let m;
  while ((m = re.exec(src)) !== null) {
    let depth = 1, i = re.lastIndex;
    while (depth > 0 && i < src.length) {
      if (src[i] === "{") depth++;
      else if (src[i] === "}") depth--;
      i++;
    }
    out.push({ name: m[2], kind: m[1], body: src.slice(re.lastIndex, i - 1) });
  }
  return out;
}
function members(body) {
  const marked = body
    .replace(/\bpublic\s*:/g, "\x01public:")
    .replace(/\bprivate\s*:/g, "\x01private:")
    .replace(/\bprotected\s*:/g, "\x01protected:");
  const out = [];
  let vis = "private";
  let paren = 0, start = 0, i = 0;
  const emit = (seg) => {
    const visM = [...seg.matchAll(/\x01(public|private|protected):/g)];
    if (visM.length) vis = visM[visM.length - 1][1];
    seg = seg.replace(/\x01(?:public|private|protected):/g, " ").trim().replace(/\s+/g, " ");
    if (!seg) return;
    if (/^(enum|template|typedef|using|static_assert|friend)\b/.test(seg)) return;
    if (/DISALLOW|NON_EXPORTED|^#/.test(seg)) return;
    out.push({ sig: seg, vis, kind: /constexpr|static const/.test(seg) ? "const" : (/\(/.test(seg) ? "method" : "field") });
  };
  while (i < marked.length) {
    const c = marked[i];
    if (c === "(") paren++;
    else if (c === ")") paren = Math.max(0, paren - 1);
    else if (paren === 0 && c === ";") {
      emit(marked.slice(start, i));
      start = i + 1;
    } else if (paren === 0 && c === "{") {
      const sigEnd = i;
      let d = 1;
      i++;
      while (d > 0 && i < marked.length) {
        if (marked[i] === "{") d++;
        else if (marked[i] === "}") d--;
        i++;
      }
      emit(marked.slice(start, sigEnd));
      start = i;
      continue;
    }
    i++;
  }
  emit(marked.slice(start));
  return out;
}

const stripped = stripComments(hSrc);
const blocks = classBlocks(stripped);
const CLASS_TABLES = [["Register", "REGISTER_DECLARATIONS", 43, 0], ["RegisterList", "REGISTER_LIST_DECLARATIONS", 12, 0]];
const emitTables = [];
for (const [className, constName, expected, occurrence] of CLASS_TABLES) {
  const hits = blocks.filter((b) => b.name === className);
  const idx = occurrence ?? 0;
  if (hits.length <= idx) throw new Error(`类缺失: ${className} (#${idx})`);
  const decls = members(hits[idx].body);
  if (decls.length !== expected) throw new Error(`${className} 对数不符: 期望 ${expected} 实得 ${decls.length}`);
  emitTables.push({ constName, decls, className });
}

let rust = banner("src/interpreter/bytecode-register.h.js", "src/interpreter/bytecode-register.{h,cc}", "");

rust += `
use crate::bytecodes::{Bytecode, SHORT_STAR_COUNT};

/// 非法寄存器索引 (原版 kInvalidIndex = kMaxInt)
pub const INVALID_INDEX: i32 = i32::MAX;

/// 帧偏移 -> 寄存器索引 (原版 OffsetFromFPToRegisterIndex; kSystemPointerSize = 8)
#[inline]
#[must_use]
pub const fn offset_from_fp_to_register_index(offset: i32, register_file_from_fp: i32) -> i32 {
  (register_file_from_fp - offset) / 8
}

/// 寄存器索引 -> 字节码操作数编码 (原版 Register::ToOperand; register_file_start_offset =
/// OffsetFromFPToRegisterIndex(0))
#[inline]
#[must_use]
pub const fn to_operand(register_file_start_offset: i32, index: i32) -> i32 {
  register_file_start_offset - index
}

/// 操作数编码 -> 寄存器索引 (原版 Register::FromOperand)
#[inline]
#[must_use]
pub const fn from_operand(register_file_start_offset: i32, operand: i32) -> i32 {
  register_file_start_offset - operand
}

/// ShortStar 字节码 -> 寄存器索引 (原版 Register::FromShortStar; star0_repr = Bytecode::Star0)
#[inline]
#[must_use]
pub const fn from_short_star(star0_repr: u8, bytecode: Bytecode) -> i32 {
  star0_repr as i32 - bytecode as i32
}

/// 寄存器索引 -> ShortStar 字节码 (原版 Register::TryToShortStar; star0_repr = Bytecode::Star0)
#[inline]
#[must_use]
pub const fn try_to_short_star(star0_repr: u8, index: i32) -> Option<Bytecode> {
  if index >= 0 && index < SHORT_STAR_COUNT as i32 {
    Bytecode::from_repr((star0_repr as i32 - index) as u8)
  } else {
    None
  }
}

/// 操作数编码尺寸 (原版 Register::SizeOfOperand; i8/i16 阈值 -128/127/-32768/32767)
#[inline]
#[must_use]
pub const fn size_of_operand(operand: i32) -> crate::bytecode_operands::OperandSize {
  use crate::bytecode_operands::OperandSize;
  if operand >= -128 && operand <= 127 {
    OperandSize::Byte
  } else if operand >= -32768 && operand <= 32767 {
    OperandSize::Short
  } else {
    OperandSize::Quad
  }
}

/// 解释器寄存器类型定义
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Register {
  index: i32,
}

impl Register {
  #[inline]
  pub const fn new(index: i32) -> Self {
    Self { index }
  }

  #[inline]
  pub const fn invalid_value() -> Self {
    Self { index: INVALID_INDEX }
  }

  #[inline]
  pub const fn index(self) -> i32 {
    self.index
  }

  #[inline]
  pub const fn is_valid(self) -> bool {
    self.index != INVALID_INDEX
  }

  #[inline]
  pub const fn is_parameter(self) -> bool {
    self.index < 0
  }

  #[inline]
  pub const fn from_parameter_index(index: i32) -> Self {
    Self { index: -1 - index }
  }

  #[inline]
  pub const fn to_parameter_index(self) -> i32 {
    -1 - self.index
  }

  #[inline]
  pub const fn receiver() -> Self {
    Self::from_parameter_index(0)
  }

  #[inline]
  pub const fn is_receiver(self) -> bool {
    self.to_parameter_index() == 0
  }

  #[inline]
  pub const fn to_operand(self) -> i32 {
    -4 - self.index
  }

  #[inline]
  pub const fn from_operand(operand: i32) -> Self {
    Self { index: -4 - operand }
  }

  #[inline]
  pub const fn from_short_star(bytecode: Bytecode) -> Self {
    Self {
      index: (Bytecode::Star0 as i32) - (bytecode as i32),
    }
  }

  #[inline]
  pub fn try_to_short_star(self) -> Option<Bytecode> {
    try_to_short_star(Bytecode::Star0 as u8, self.index)
  }

  #[inline]
  pub const fn size_of_operand(self) -> crate::bytecode_operands::OperandSize {
    size_of_operand(self.to_operand())
  }
}

/// 连续寄存器列表
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct RegisterList {
  first_reg_index: i32,
  register_count: usize,
}

impl RegisterList {
  #[inline]
  pub const fn new() -> Self {
    Self {
      first_reg_index: INVALID_INDEX,
      register_count: 0,
    }
  }

  #[inline]
  pub const fn single(reg: Register) -> Self {
    Self {
      first_reg_index: reg.index(),
      register_count: 1,
    }
  }

  #[inline]
  pub const fn from_range(first_reg: Register, count: usize) -> Self {
    Self {
      first_reg_index: first_reg.index(),
      register_count: count,
    }
  }

  #[inline]
  pub const fn register_count(&self) -> usize {
    self.register_count
  }

  #[inline]
  pub const fn is_empty(&self) -> bool {
    self.register_count == 0
  }

  #[inline]
  pub const fn first_register(&self) -> Register {
    if self.register_count == 0 {
      Register::new(0)
    } else {
      Register::new(self.first_reg_index)
    }
  }

  #[inline]
  pub const fn last_register(&self) -> Register {
    if self.register_count == 0 {
      Register::new(0)
    } else {
      Register::new(self.first_reg_index + (self.register_count as i32) - 1)
    }
  }

  #[inline]
  pub const fn get(&self, index: usize) -> Option<Register> {
    if index < self.register_count {
      Some(Register::new(self.first_reg_index + (index as i32)))
    } else {
      None
    }
  }

  #[inline]
  pub fn register_at(&self, index: usize) -> crate::error::Result<Register> {
    if index < self.register_count {
      Ok(Register::new(self.first_reg_index + (index as i32)))
    } else {
      Err(crate::error::Error::InvalidRegisterCount {
        expected: self.register_count,
        got: index,
      })
    }
  }

  #[inline]
  pub fn truncate(&self, new_count: usize) -> crate::error::Result<Self> {
    if new_count <= self.register_count {
      Ok(Self {
        first_reg_index: self.first_reg_index,
        register_count: new_count,
      })
    } else {
      Err(crate::error::Error::InvalidRegisterCount {
        expected: self.register_count,
        got: new_count,
      })
    }
  }

  #[inline]
  pub fn pop_left(&self) -> crate::error::Result<Self> {
    if self.register_count > 0 {
      Ok(Self {
        first_reg_index: self.first_reg_index + 1,
        register_count: self.register_count - 1,
      })
    } else {
      Err(crate::error::Error::EmptyRegisterList)
    }
  }
}
`;

for (const { constName, decls, className } of emitTables) {
  rust += `
/// 原版 class/struct ${className} 成员声明 (机械 1:1 提取, ${decls.length} 条)
pub const ${constName}: &[&str] = &[
`;
  for (const d of decls) rust += `  "[${d.vis}/${d.kind}] ${d.sig}",
`;
  rust += `];
`;
}

rust += `
// —— 提取不变式断言 ——
const _: () = {
  assert!(REGISTER_DECLARATIONS.len() == 43);
  assert!(REGISTER_LIST_DECLARATIONS.len() == 12);
  assert!(INVALID_INDEX == i32::MAX);
  assert!(size_of_operand(0) as u8 == crate::bytecode_operands::OperandSize::Byte as u8);
  assert!(size_of_operand(1000) as u8 == crate::bytecode_operands::OperandSize::Short as u8);
  assert!(size_of_operand(100000) as u8 == crate::bytecode_operands::OperandSize::Quad as u8);
};
`;

const p = writeSrc("v8-interpreter", "bytecode_register.rs", rust);
console.log(`written ${p} (${emitTables.map((t) => t.constName + "=" + t.decls.length).join(", ")})`);
