#!/usr/bin/env bun
// tool/src/interpreter/bytecode-source-info.h.js
// 机械提取 v8/src/interpreter/bytecode-source-info.{h,cc} 的源码位置信息骨架
// → v8-interpreter/src/bytecode_source_info.rs
import { readV8, writeSrc, banner } from "../../lib.js";

const hSrc = readV8("src/interpreter/bytecode-source-info.h");
const ccSrc = readV8("src/interpreter/bytecode-source-info.cc");

for (const [src, needle, name] of [
  [hSrc, "static const int kUninitializedPosition = -1;", "kUninitializedPosition"],
  [hSrc, "enum class PositionType : uint8_t { kNone, kExpression, kStatement };", "PositionType 枚举"],
  [hSrc, "void MakeStatementPosition(int source_position, bool is_breakable = true)", "MakeStatementPosition"],
  [ccSrc, "std::ostream& operator<<(std::ostream& os, const BytecodeSourceInfo& info)", "operator<< 定义"],
]) {
  if (!src.includes(needle)) throw new Error(`结构锚缺失: ${name}`);
}

// —— 机械提取: 去注释 + 类块 + 成员声明 ——
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

const cls = classBlocks(stripComments(hSrc)).find((b) => b.name === "BytecodeSourceInfo");
if (!cls) throw new Error("类缺失: BytecodeSourceInfo");
const decls = members(cls.body);
if (decls.length !== 17) throw new Error(`成员对数不符: 期望 17 实得 ${decls.length}`);

let rust = banner("src/interpreter/bytecode-source-info.h.js", "src/interpreter/bytecode-source-info.{h,cc}", "字节码源码位置信息骨架");

rust += `
/// 位置类别 (原版私有 enum class PositionType : uint8_t)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(u8)]
pub enum PositionType {
  #[default]
  None = 0,
  Expression = 1,
  Statement = 2,
}

/// 未初始化位置哨兵 (原版 kUninitializedPosition)
pub const UNINITIALIZED_POSITION: i32 = -1;

/// 原版 class BytecodeSourceInfo 成员声明 (机械 1:1 提取, ${decls.length} 条)
pub const BYTECODE_SOURCE_INFO_DECLARATIONS: &[&str] = &[
`;
for (const d of decls) rust += `  "[${d.vis}/${d.kind}] ${d.sig}",\n`;
rust += `];

/// 源码位置信息 (原版 BytecodeSourceInfo)
#[derive(Debug, Clone, Copy, Default)]
pub struct BytecodeSourceInfo {
  position_type: PositionType,
  source_position: i32,
  is_breakable: bool,
}

impl PartialEq for BytecodeSourceInfo {
  #[inline]
  fn eq(&self, other: &Self) -> bool {
    self.position_type == other.position_type && self.source_position == other.source_position
  }
}
impl Eq for BytecodeSourceInfo {}

impl BytecodeSourceInfo {
  #[inline]
  pub const fn none() -> Self {
    Self {
      position_type: PositionType::None,
      source_position: UNINITIALIZED_POSITION,
      is_breakable: true,
    }
  }

  #[inline]
  pub const fn statement(source_position: i32, is_breakable: bool) -> Self {
    Self {
      position_type: PositionType::Statement,
      source_position,
      is_breakable,
    }
  }

  #[inline]
  pub const fn expression(source_position: i32) -> Self {
    Self {
      position_type: PositionType::Expression,
      source_position,
      is_breakable: true,
    }
  }

  #[inline]
  pub const fn source_position(self) -> i32 {
    self.source_position
  }

  #[inline]
  pub const fn is_statement(self) -> bool {
    matches!(self.position_type, PositionType::Statement)
  }

  #[inline]
  pub const fn is_expression(self) -> bool {
    matches!(self.position_type, PositionType::Expression)
  }

  #[inline]
  pub const fn is_valid(self) -> bool {
    self.source_position != UNINITIALIZED_POSITION
  }

  #[inline]
  pub fn set_invalid(&mut self) {
    self.position_type = PositionType::None;
    self.source_position = UNINITIALIZED_POSITION;
  }

  #[inline]
  pub fn make_statement_position(&mut self, source_position: i32, is_breakable: bool) {
    self.position_type = PositionType::Statement;
    self.source_position = source_position;
    self.is_breakable = is_breakable;
  }

  #[inline]
  pub fn make_expression_position(&mut self, source_position: i32) {
    self.position_type = PositionType::Expression;
    self.source_position = source_position;
    self.is_breakable = true;
  }

  #[inline]
  pub fn force_expression_position(&mut self, source_position: i32) {
    self.position_type = PositionType::Expression;
    self.source_position = source_position;
  }

  #[inline]
  pub const fn is_breakable(self) -> bool {
    self.is_breakable
  }
}

// —— 提取不变式断言 ——
const _: () = {
  assert!(BYTECODE_SOURCE_INFO_DECLARATIONS.len() == 17);
  assert!(UNINITIALIZED_POSITION == -1);
};
`;

const p = writeSrc("v8-interpreter", "bytecode_source_info.rs", rust);
console.log(`written ${p} (${decls.length} declarations)`);
