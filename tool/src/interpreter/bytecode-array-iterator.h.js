#!/usr/bin/env bun
// tool/src/interpreter/bytecode-array-iterator.h.js
// 机械提取 src/interpreter/bytecode-array-iterator.{h,cc} 的字节码数组迭代器与跳转表偏移区间骨架
// → v8-interpreter/src/bytecode_array_iterator.rs
import { readV8, writeSrc, banner } from "../../lib.js";
const hSrc = readV8("src/interpreter/bytecode-array-iterator.h");
const ccSrc = readV8("src/interpreter/bytecode-array-iterator.cc");
// 结构锚
for (const [src, needle, name] of [
  [hSrc, "struct V8_EXPORT_PRIVATE JumpTableTargetOffset {", "JumpTableTargetOffset"],
  [hSrc, "int case_value;", "case_value \u5b57\u6bb5"],
  [hSrc, "class V8_EXPORT_PRIVATE BytecodeArrayIterator {", "BytecodeArrayIterator"],
  [ccSrc, "#include \"src/interpreter/bytecode-array-iterator.h\"", "cc \u5f15\u5165\u951a"],
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
const CLASS_TABLES = [["JumpTableTargetOffset", "JUMP_TABLE_TARGET_OFFSET_DECLARATIONS", 2, 0], ["JumpTableTargetOffsets", "JUMP_TABLE_TARGET_OFFSETS_DECLARATIONS", 9, 0], ["iterator", "JUMP_TABLE_OFFSETS_ITERATOR_DECLARATIONS", 10, 0], ["BytecodeArrayIterator", "BYTECODE_ARRAY_ITERATOR_DECLARATIONS", 75, 0]];
const emitTables = [];
for (const [className, constName, expected, occurrence] of CLASS_TABLES) {
  const hits = blocks.filter((b) => b.name === className);
  const idx = occurrence ?? 0;
  if (hits.length <= idx) throw new Error(`类缺失: ${className} (#${idx})`);
  const decls = members(hits[idx].body);
  if (decls.length !== expected) throw new Error(`${className} 对数不符: 期望 ${expected} 实得 ${decls.length}`);
  emitTables.push({ constName, decls, className });
}

let rust = banner("src/interpreter/bytecode-array-iterator.h.js", "src/interpreter/bytecode-array-iterator.{h,cc}", "");

rust += `
/// 跳转表偏移项 (原版 struct JumpTableTargetOffset)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct JumpTableTargetOffset {
  /// case 值
  pub case_value: i32,
  /// 目标字节码偏移
  pub target_offset: i32,
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
/// 字节码数组安全迭代器 (对应原版 BytecodeArrayIterator)
#[derive(Debug, Clone)]
pub struct BytecodeArrayIterator<'a> {
  bytes: &'a [u8],
  cursor: usize,
  current_offset: usize,
  current_bytecode: crate::bytecodes::Bytecode,
  operand_scale: crate::bytecode_operands::OperandScale,
  prefix_size: usize,
  done: bool,
}

impl<'a> BytecodeArrayIterator<'a> {
  pub fn new(bytes: &'a [u8]) -> Self {
    let mut it = Self {
      bytes,
      cursor: 0,
      current_offset: 0,
      current_bytecode: crate::bytecodes::Bytecode::Illegal,
      operand_scale: crate::bytecode_operands::OperandScale::Single,
      prefix_size: 0,
      done: bytes.is_empty(),
    };
    if !bytes.is_empty() {
      it.update_current_bytecode();
    }
    it
  }

  #[inline]
  pub const fn done(&self) -> bool {
    self.done
  }

  #[inline]
  pub const fn current_bytecode(&self) -> crate::bytecodes::Bytecode {
    self.current_bytecode
  }

  #[inline]
  pub const fn current_offset(&self) -> usize {
    self.current_offset
  }

  #[inline]
  pub const fn current_operand_scale(&self) -> crate::bytecode_operands::OperandScale {
    self.operand_scale
  }

  #[inline]
  pub fn current_bytecode_size_without_prefix(&self) -> usize {
    self.current_bytecode.size(self.operand_scale)
  }

  #[inline]
  pub fn current_bytecode_size(&self) -> usize {
    self.prefix_size + self.current_bytecode_size_without_prefix()
  }

  #[inline]
  pub fn next_offset(&self) -> usize {
    self.current_offset + self.current_bytecode_size()
  }

  pub fn next_bytecode(&self) -> crate::bytecodes::Bytecode {
    let next_cursor = self.cursor + self.current_bytecode_size_without_prefix();
    if next_cursor >= self.bytes.len() {
      return crate::bytecodes::Bytecode::Illegal;
    }
    let mut byte = self.bytes[next_cursor];
    if byte == (crate::bytecodes::Bytecode::Wide as u8) || byte == (crate::bytecodes::Bytecode::ExtraWide as u8) {
      if next_cursor + 1 < self.bytes.len() {
        byte = self.bytes[next_cursor + 1];
      } else {
        return crate::bytecodes::Bytecode::Illegal;
      }
    }
    crate::bytecodes::Bytecode::from_byte(byte).unwrap_or(crate::bytecodes::Bytecode::Illegal)
  }

  pub fn reset(&mut self) {
    self.cursor = 0;
    self.current_offset = 0;
    self.operand_scale = crate::bytecode_operands::OperandScale::Single;
    self.prefix_size = 0;
    self.done = self.bytes.is_empty();
    if !self.done {
      self.update_current_bytecode();
    }
  }

  pub fn advance_to(&mut self, offset: usize) -> crate::error::Result<()> {
    if self.current_offset > offset {
      return Err(crate::error::Error::InvalidBytecodeOffset(offset));
    }
    while !self.done && self.current_offset < offset {
      self.advance();
    }
    if !self.done && self.current_offset == offset {
      Ok(())
    } else {
      Err(crate::error::Error::InvalidBytecodeOffset(offset))
    }
  }

  pub fn set_offset(&mut self, offset: usize) -> crate::error::Result<()> {
    if self.current_offset > offset {
      self.reset();
    }
    self.advance_to(offset)
  }

  pub fn advance(&mut self) {
    if self.done {
      return;
    }
    let size_without = self.current_bytecode_size_without_prefix();
    self.cursor += size_without;
    if self.cursor >= self.bytes.len() {
      self.done = true;
    } else {
      self.update_current_bytecode();
    }
  }

  fn update_current_bytecode(&mut self) {
    if self.cursor >= self.bytes.len() {
      self.done = true;
      return;
    }
    self.current_offset = self.cursor;
    let mut byte = self.bytes[self.cursor];
    if byte == (crate::bytecodes::Bytecode::Wide as u8) {
      self.operand_scale = crate::bytecode_operands::OperandScale::Double;
      self.prefix_size = 1;
      self.cursor += 1;
      if self.cursor < self.bytes.len() {
        byte = self.bytes[self.cursor];
      } else {
        self.done = true;
        return;
      }
    } else if byte == (crate::bytecodes::Bytecode::ExtraWide as u8) {
      self.operand_scale = crate::bytecode_operands::OperandScale::Quadruple;
      self.prefix_size = 1;
      self.cursor += 1;
      if self.cursor < self.bytes.len() {
        byte = self.bytes[self.cursor];
      } else {
        self.done = true;
        return;
      }
    } else {
      self.operand_scale = crate::bytecode_operands::OperandScale::Single;
      self.prefix_size = 0;
    }

    self.current_bytecode = match crate::bytecodes::Bytecode::from_byte(byte) {
      Some(bc) => bc,
      None => crate::bytecodes::Bytecode::Illegal,
    };
  }

  fn read_operand_bytes(&self, operand_index: usize) -> crate::error::Result<(&'a [u8], crate::bytecode_operands::OperandType, crate::bytecode_operands::OperandSize)> {
    if self.done {
      return Err(crate::error::Error::UnexpectedEndOfInput);
    }
    let num_ops = self.current_bytecode.number_of_operands();
    if operand_index >= num_ops {
      return Err(crate::error::Error::OperandIndexOutOfRange {
        index: operand_index,
        count: num_ops,
      });
    }

    let op_type = self.current_bytecode.get_operand_type(operand_index);
    let op_size = op_type.size(self.operand_scale);
    let op_offset = self.current_bytecode.get_operand_offset(operand_index, self.operand_scale);
    let abs_start = self.cursor + op_offset;
    let abs_end = abs_start + (op_size as usize);

    if abs_end <= self.bytes.len() {
      Ok((&self.bytes[abs_start..abs_end], op_type, op_size))
    } else {
      Err(crate::error::Error::UnexpectedEndOfInput)
    }
  }

  pub fn get_unsigned_operand(&self, operand_index: usize) -> crate::error::Result<u32> {
    let (slice, _op_type, op_size) = self.read_operand_bytes(operand_index)?;
    match op_size {
      crate::bytecode_operands::OperandSize::None => Ok(0),
      crate::bytecode_operands::OperandSize::Byte => Ok(slice[0] as u32),
      crate::bytecode_operands::OperandSize::Short => Ok(u16::from_le_bytes([slice[0], slice[1]]) as u32),
      crate::bytecode_operands::OperandSize::Quad => Ok(u32::from_le_bytes([slice[0], slice[1], slice[2], slice[3]])),
    }
  }

  pub fn get_signed_operand(&self, operand_index: usize) -> crate::error::Result<i32> {
    let (slice, _op_type, op_size) = self.read_operand_bytes(operand_index)?;
    match op_size {
      crate::bytecode_operands::OperandSize::None => Ok(0),
      crate::bytecode_operands::OperandSize::Byte => Ok((slice[0] as i8) as i32),
      crate::bytecode_operands::OperandSize::Short => Ok((i16::from_le_bytes([slice[0], slice[1]])) as i32),
      crate::bytecode_operands::OperandSize::Quad => Ok(i32::from_le_bytes([slice[0], slice[1], slice[2], slice[3]])),
    }
  }

  #[inline]
  pub fn get_flag8_operand(&self, operand_index: usize) -> crate::error::Result<u8> {
    Ok(self.get_unsigned_operand(operand_index)? as u8)
  }

  #[inline]
  pub fn get_flag16_operand(&self, operand_index: usize) -> crate::error::Result<u16> {
    Ok(self.get_unsigned_operand(operand_index)? as u16)
  }

  #[inline]
  pub fn get_immediate_operand(&self, operand_index: usize) -> crate::error::Result<i32> {
    self.get_signed_operand(operand_index)
  }

  #[inline]
  pub fn get_unsigned_immediate_operand(&self, operand_index: usize) -> crate::error::Result<u32> {
    self.get_unsigned_operand(operand_index)
  }

  #[inline]
  pub fn get_register_operand(&self, operand_index: usize) -> crate::error::Result<crate::bytecode_register::Register> {
    let raw = self.get_signed_operand(operand_index)?;
    Ok(crate::bytecode_register::Register::from_operand(raw))
  }

  #[inline]
  pub fn get_register_list_operand(&self, operand_index: usize) -> crate::error::Result<crate::bytecode_register::RegisterList> {
    let first = self.get_register_operand(operand_index)?;
    let count = self.get_unsigned_operand(operand_index + 1)? as usize;
    Ok(crate::bytecode_register::RegisterList::from_range(first, count))
  }

  #[inline]
  pub fn get_register_pair_operand(&self, operand_index: usize) -> crate::error::Result<(crate::bytecode_register::Register, crate::bytecode_register::Register)> {
    let first = self.get_register_operand(operand_index)?;
    let second = crate::bytecode_register::Register::new(first.index() + 1);
    Ok((first, second))
  }

  #[inline]
  pub fn get_star_target_register(&self) -> crate::error::Result<crate::bytecode_register::Register> {
    if crate::bytecodes::Bytecodes::is_short_star(self.current_bytecode) {
      Ok(crate::bytecode_register::Register::from_short_star(self.current_bytecode))
    } else if self.current_bytecode == crate::bytecodes::Bytecode::Star {
      self.get_register_operand(0)
    } else {
      Err(crate::error::Error::InvalidBytecode(self.current_bytecode as u8))
    }
  }

  #[inline]
  pub fn get_register_operand_range(&self, operand_index: usize) -> crate::error::Result<u32> {
    let num_ops = self.current_bytecode.number_of_operands();
    if operand_index >= num_ops {
      return Err(crate::error::Error::OperandIndexOutOfRange {
        index: operand_index,
        count: num_ops,
      });
    }
    let op_type = self.current_bytecode.get_operand_type(operand_index);
    if op_type == crate::bytecode_operands::OperandType::RegList || op_type == crate::bytecode_operands::OperandType::RegOutList {
      self.get_unsigned_operand(operand_index + 1)
    } else if let Some(n) = crate::bytecodes::Bytecodes::number_of_registers_represented_by(op_type) {
      Ok(n)
    } else {
      Err(crate::error::Error::OperandIndexOutOfRange {
        index: operand_index,
        count: num_ops,
      })
    }
  }

  #[inline]
  pub fn get_runtime_id_operand(&self, operand_index: usize) -> crate::error::Result<u32> {
    self.get_unsigned_operand(operand_index)
  }

  #[inline]
  pub fn get_parameter(&self, parameter_index: i32) -> crate::bytecode_register::Register {
    crate::bytecode_register::Register::from_parameter_index(parameter_index + 1)
  }

  #[inline]
  pub fn get_relative_jump_target_offset(&self) -> crate::error::Result<i32> {
    let raw = self.get_unsigned_immediate_operand(0)? as i32;
    if self.current_bytecode == crate::bytecodes::Bytecode::JumpLoop {
      Ok(-raw)
    } else {
      Ok(raw)
    }
  }

  #[inline]
  pub fn get_absolute_offset(&self, relative_offset: i32) -> crate::error::Result<usize> {
    let target = (self.current_offset as isize) + (relative_offset as isize) + (self.prefix_size as isize);
    if target >= 0 {
      Ok(target as usize)
    } else {
      Err(crate::error::Error::InvalidJumpTarget(target))
    }
  }

  #[inline]
  pub fn get_jump_target_offset(&self) -> crate::error::Result<usize> {
    self.get_absolute_offset(self.get_relative_jump_target_offset()?)
  }
}

// —— 提取不变式断言 ——
const _: () = {
  assert!(JUMP_TABLE_TARGET_OFFSET_DECLARATIONS.len() == 2);
};
`;

const p = writeSrc("v8-interpreter", "bytecode_array_iterator.rs", rust);
console.log(`written ${p} (${emitTables.map((t) => t.constName + "=" + t.decls.length).join(", ")})`);
