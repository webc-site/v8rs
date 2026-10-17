#!/usr/bin/env bun
// tool/src/interpreter/bytecode-node.h.js
// 机械提取 src/interpreter/bytecode-node.{h,cc} 的字节码节点骨架 (操作数上限 5)
// → v8-interpreter/src/bytecode_node.rs
import { readV8, writeSrc, banner } from "../../lib.js";
const hSrc = readV8("src/interpreter/bytecode-node.h");
const ccSrc = readV8("src/interpreter/bytecode-node.cc");
// 结构锚
for (const [src, needle, name] of [
  [hSrc, "class V8_EXPORT_PRIVATE BytecodeNode final {", "BytecodeNode"],
  [hSrc, "uint32_t operands_[Bytecodes::kMaxOperands];", "operands_ \u5b9a\u957f\u6570\u7ec4"],
  [ccSrc, "#include \"src/interpreter/bytecode-node.h\"", "cc \u5f15\u5165\u951a"],
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
const CLASS_TABLES = [["BytecodeNode", "BYTECODE_NODE_DECLARATIONS", 25, 0]];
const emitTables = [];
for (const [className, constName, expected, occurrence] of CLASS_TABLES) {
  const hits = blocks.filter((b) => b.name === className);
  const idx = occurrence ?? 0;
  if (hits.length <= idx) throw new Error(`类缺失: ${className} (#${idx})`);
  const decls = members(hits[idx].body);
  if (decls.length !== expected) throw new Error(`${className} 对数不符: 期望 ${expected} 实得 ${decls.length}`);
  emitTables.push({ constName, decls, className });
}

let rust = banner("src/interpreter/bytecode-node.h.js", "src/interpreter/bytecode-node.{h,cc}", "");

rust += `
/// 单节点操作数容量 (原版 operands_[5], 对齐 Bytecodes::kMaxOperands)
pub const NODE_OPERAND_CAPACITY: usize = 5;
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
/// 字节码节点 (承载操作码、操作数列表、缩放尺度与源码位置)
#[derive(Debug, Clone, Copy)]
pub struct BytecodeNode {
  bytecode: crate::bytecodes::Bytecode,
  operand_count: usize,
  operand_scale: crate::bytecode_operands::OperandScale,
  source_info: crate::bytecode_source_info::BytecodeSourceInfo,
  operands: [u32; NODE_OPERAND_CAPACITY],
}

impl PartialEq for BytecodeNode {
  #[inline]
  fn eq(&self, other: &Self) -> bool {
    if self.bytecode != other.bytecode || self.source_info != other.source_info {
      return false;
    }
    for i in 0..self.operand_count {
      if self.operands[i] != other.operands[i] {
        return false;
      }
    }
    true
  }
}
impl Eq for BytecodeNode {}

impl Default for BytecodeNode {
  fn default() -> Self {
    Self {
      bytecode: crate::bytecodes::Bytecode::Illegal,
      operand_count: 0,
      operand_scale: crate::bytecode_operands::OperandScale::Single,
      source_info: crate::bytecode_source_info::BytecodeSourceInfo::none(),
      operands: [0; NODE_OPERAND_CAPACITY],
    }
  }
}

impl BytecodeNode {
  #[inline]
  pub fn new(bytecode: crate::bytecodes::Bytecode, source_info: crate::bytecode_source_info::BytecodeSourceInfo) -> crate::error::Result<Self> {
    let expected = bytecode.number_of_operands();
    if expected != 0 {
      return Err(crate::error::Error::InvalidOperandCount {
        expected,
        got: 0,
      });
    }
    Ok(Self {
      bytecode,
      operand_count: 0,
      operand_scale: crate::bytecode_operands::OperandScale::Single,
      source_info,
      operands: [0; NODE_OPERAND_CAPACITY],
    })
  }

  #[inline]
  pub fn with_operands(
    bytecode: crate::bytecodes::Bytecode,
    operands: &[u32],
    source_info: crate::bytecode_source_info::BytecodeSourceInfo,
  ) -> crate::error::Result<Self> {
    let expected = bytecode.number_of_operands();
    if expected != operands.len() {
      return Err(crate::error::Error::InvalidOperandCount {
        expected,
        got: operands.len(),
      });
    }

    let mut node_operands = [0u32; NODE_OPERAND_CAPACITY];
    let mut scale = crate::bytecode_operands::OperandScale::Single;

    for (i, &op) in operands.iter().enumerate() {
      node_operands[i] = op;
      let op_type = bytecode.get_operand_type(i);
      let op_scale = op_type.scale_for_operand(op);
      if (op_scale as u8) > (scale as u8) {
        scale = op_scale;
      }
    }

    Ok(Self {
      bytecode,
      operand_count: operands.len(),
      operand_scale: scale,
      source_info,
      operands: node_operands,
    })
  }

  #[inline]
  pub const fn bytecode(&self) -> crate::bytecodes::Bytecode {
    self.bytecode
  }

  #[inline]
  pub const fn operand_count(&self) -> usize {
    self.operand_count
  }

  #[inline]
  pub const fn operand_scale(&self) -> crate::bytecode_operands::OperandScale {
    self.operand_scale
  }

  #[inline]
  pub const fn source_info(&self) -> crate::bytecode_source_info::BytecodeSourceInfo {
    self.source_info
  }

  #[inline]
  pub fn set_source_info(&mut self, source_info: crate::bytecode_source_info::BytecodeSourceInfo) {
    self.source_info = source_info;
  }

  #[inline]
  pub fn operand(&self, index: usize) -> crate::error::Result<u32> {
    if index < self.operand_count {
      Ok(self.operands[index])
    } else {
      Err(crate::error::Error::OperandIndexOutOfRange {
        index,
        count: self.operand_count,
      })
    }
  }

  #[inline]
  pub fn set_operand(&mut self, index: usize, operand: u32) -> crate::error::Result<()> {
    if index < self.operand_count {
      self.operands[index] = operand;
      let op_type = self.bytecode.get_operand_type(index);
      let op_scale = op_type.scale_for_operand(operand);
      if (op_scale as u8) > (self.operand_scale as u8) {
        self.operand_scale = op_scale;
      }
      Ok(())
    } else {
      Err(crate::error::Error::OperandIndexOutOfRange {
        index,
        count: self.operand_count,
      })
    }
  }

  #[inline]
  pub fn update_operand0(&mut self, operand0: u32) -> crate::error::Result<()> {
    self.set_operand(0, operand0)
  }

  #[inline]
  pub const fn operands(&self) -> &[u32; NODE_OPERAND_CAPACITY] {
    &self.operands
  }
}

// —— 提取不变式断言 ——
const _: () = {
  assert!(BYTECODE_NODE_DECLARATIONS.len() == 25);
  assert!(NODE_OPERAND_CAPACITY == 5);
};
`;

const p = writeSrc("v8-interpreter", "bytecode_node.rs", rust);
console.log(`written ${p} (${emitTables.map((t) => t.constName + "=" + t.decls.length).join(", ")})`);
