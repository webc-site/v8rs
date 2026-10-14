#!/usr/bin/env bun
// v8/src/torque/instructions.{h,cc} + cfg.{h,cc} → 直出
//   instructions.h/.cc → instructions.rs (25 条指令全量: kind + 结构体 + 包装 enum)
//   cfg.h/.cc          → cfg.rs          (Block/ControlFlowGraph/CfgAssembler)
// 继承树映射: InstructionBase 虚函数 + unique_ptr 克隆 → Rust enum Instruction
// (判别即 kind); Block* → BlockId; Stack<const Type*> → Stack<TypeId>。
import { readV8, writeSrc, banner } from "../../lib.js";

// ---------- 通用提取器 (脚本内本地副本) ----------
// 单遍词法化 FSM: 注释与字符串/字符字面量同层判定 (字符串内 // 不误删),
// 换行 1:1 保留 → 行号稳定; 支持 \' \" 转义、行尾续行、R"( raw string
const noCmt = (s) => {
  let out = "", i = 0;
  const n = s.length;
  while (i < n) {
    const c = s[i];
    if (c === "/" && s[i + 1] === "*") {
      const end = s.indexOf("*/", i + 2);
      const seg = s.slice(i, end < 0 ? n : end + 2);
      for (const ch of seg) if (ch === "\n") out += "\n";
      i += seg.length;
    } else if (c === "/" && s[i + 1] === "/") {
      while (i < n && s[i] !== "\n") i++;
    } else if (c === '"' || (c === "R" && /^R"[^ ()\\\t\v\f\n]*\(/.test(s.slice(i, i + 64)))) {
      const raw = c === "R" ? /^R"([^ ()\\\t\v\f\n]*)\(/.exec(s.slice(i, i + 64))[0] : "";
      if (raw) {
        const close = ")" + raw.slice(2, -1) + '"';
        const end = s.indexOf(close, i + raw.length);
        const body = s.slice(i + raw.length, end < 0 ? n : end);
        for (const ch of body) if (ch === "\n") out += "\n";
        out += '""';
        i = end < 0 ? n : end + close.length;
      } else {
        out += '""';
        i++;
        while (i < n && s[i] !== '"' && s[i] !== "\n") {
          if (s[i] === "\\") {
            if (s[i + 1] === "\n") out += "\n";
            i += 2;
          } else i++;
        }
        if (s[i] === '"') i++;
      }
    } else if (c === "'") {
      out += "''";
      i++;
      while (i < n && s[i] !== "'" && s[i] !== "\n") {
        i += s[i] === "\\" ? 2 : 1;
      }
      if (s[i] === "'") i++;
    } else {
      out += c;
      i++;
    }
  }
  return out;
};

function topSigs(text) {
  const out = [];
  let ns = 0, fn = 0, unit = "", startLine = 0, live = true;
  const fin = () => {
    const s = unit.trim().replace(/\s+/g, " ");
    if (s.includes("(") && !/^(namespace|using|typedef|friend|enum|struct|class|template|static_assert|if|for|while|switch|else|do|try|catch|return|case|default|#)/.test(s) && /[)]\s*(const)?\s*(override)?\s*(noexcept)?\s*$/.test(s)) {
      out.push({ sig: s, line: startLine });
    }
  };
  for (const [i, raw] of noCmt(text).split("\n").entries()) {
    if (/^\s*#/.test(raw)) continue;
    // 顶层全大写宏行 (EXPORT_CONTEXTUAL_VARIABLE(...) / DISABLE_CFI_ICALL) 不入签名
    if (/^[A-Z][A-Z0-9_]*(\(.*\))?\s*;?\s*$/.test(raw.trim())) continue;
    for (const ch of raw) {
      if (ch === "{") {
        if (fn === 0) {
          const u = unit.trim();
          if (u === "" || /^(namespace|extern)\b/.test(u)) { ns++; unit = ""; live = true; continue; }
          if (live) fin();
          unit = ""; live = false; fn++;
        } else fn++;
      } else if (ch === "}") {
        if (fn > 0) fn--; else ns--;
      } else if (fn === 0 && live) {
        if (!unit.trim()) startLine = i + 1;
        unit += ch;
      }
    }
    if (fn === 0) {
      if (/;\s*$/.test(raw) && !unit.includes("(")) unit = "";
      live = true;
    }
  }
  return out;
}

function classBlock(header, text, from = 0) {
  const re = new RegExp(`(?:class|struct|enum class)\\s+(?:V8_EXPORT_PRIVATE\\s+|final\\s+)*${header}(?:\\s+final)?[^\\{;]*`);
  const m = text.slice(from).match(re);
  if (!m) return null;
  const headStart = from + m.index;
  const open = text.indexOf("{", headStart);
  if (open < 0) return null;
  let depth = 0;
  for (let j = open; j < text.length; j++) {
    if (text[j] === "{") depth++;
    else if (text[j] === "}") { depth--; if (depth === 0) return { head: m[0].replace(/\s+/g, " "), body: text.slice(open + 1, j) }; }
  }
  return null;
}

function memberFields(body) {
  const out = [];
  for (const raw of body.split("\n")) {
    const line = raw.trim();
    // 括号任一侧出现 → 方法签名或其续行, 跳过
    if (!line.endsWith(";") || line.includes("(") || line.includes(")")) continue;
    if (/^(DEFINE_|DECLARE_|friend|using|typedef|enum|static)/.test(line) || line.startsWith("//")) continue;
    const m = line.slice(0, -1).match(/^(.+?)\s*\b(\w+)$/);
    if (m && !/[<>=]/.test(m[2])) out.push({ type: m[1].replace(/\s+/g, " "), name: m[2] });
  }
  return out;
}

function sigBlock(f, note = "签名清单 (填体正交任务源)") {
  const s = readV8(f);
  let dd = `// ── 原版 ${f} ──\n`;
  const sigs = topSigs(s);
  if (sigs.length === 0 && f.endsWith(".cc") && s.length > 400) {
    console.error(`[torque] ${f} 非空但抽到 0 条签名 — noCmt/topSigs 异常, 防静默检查`);
  }
  for (const x of sigs) dd += `// L${x.line}: ${x.sig}\n`;
  return dd;
}

const snake = (s) => s.replaceAll(/([a-z0-9])([A-Z])/g, "$1_$2").replaceAll(/([A-Z]+)([A-Z][a-z])/g, "$1_$2").toLowerCase();

// ===========================================================================
// instructions.{h,cc} → instructions.rs
// ===========================================================================
function genInstructions() {
  const relH = "src/torque/instructions.h";
  const ih = noCmt(readV8(relH));

  // X-macro 双清单展开
  const agnosticAll = [...ih.matchAll(/V\((\w+Instruction)\)/g)].map((x) => x[1]);
  const agnosticList = ih.match(/TORQUE_BACKEND_AGNOSTIC_INSTRUCTION_LIST\(V\) \\\n((?:\s+V\(\w+\)[ \t]*\\?\n)+)/);
  const backendList = ih.match(/TORQUE_BACKEND_DEPENDENT_INSTRUCTION_LIST\(V\) \\\n((?:\s+V\(\w+\)[ \t]*\\?\n)+)/);
  const grab = (m) => (m ? [...m[1].matchAll(/V\((\w+)\)/g)].map((x) => x[1]) : []);
  const agn = grab(agnosticList), dep = grab(backendList);
  const all = [...agn, ...dep];
  if (agn.length + dep.length !== agnosticAll.length) console.error(`指令数不齐: agn=${agn.length} dep=${dep.length} total=${agnosticAll.length}`);

  // 字段类型机械映射
  const T = (t) => t.trim().replaceAll(/\s+/g, " ")
    .replace(/std::optional<(.*?)>/g, "Option<$1>").replace(/std::vector<(.*?)>/g, "Vec<$1>")
    .replace(/std::string\b/g, "String").replace(/\bsize_t\b/g, "usize").replace(/\bbool\b/g, "bool")
    .replace(/BottomOffset/g, "BottomOffset").replace(/StackRange/g, "StackRange")
    .replace(/const Type\*/g, "TypeId").replace(/const BuiltinPointerType\*/g, "TypeId")
    .replace(/FieldSynchronization/g, "FieldSynchronization").replace(/BitField\b/g, "BitField")
    .replace(/\b(Macro|Builtin|RuntimeFunction|Intrinsic|NamespaceConstant)\*/g, "DeclarableId")
    .replace(/\bBlock\*/g, "BlockId").replace(/const/g, "").trim()
    .replace(/^Kind$/, "AbortKind");
  const FNAME = { type: "r#type", macro: "r#macro" };

  let d = banner("src/torque/cfg.js", relH, `指令集全量 (${all.length} 条: 后端无关 ${agn.length} + 后端相关 ${dep.length})`);
  d += `
use crate::declarable::DeclarableId;
use crate::error::{Error, Result};
use crate::source_positions::SourcePosition;
use crate::constants::FieldSynchronization;
use crate::cfg::ControlFlowGraph;
use crate::types::{BitField, TypeId, TypeVector};
use crate::utils::{BottomOffset, Stack, StackRange, Worklist};

// C++ InstructionBase 虚函数 + unique_ptr 克隆 → Rust enum Instruction (判别即 kind);
// InstructionKind 标签 enum 供 kind()/mnemonic() 使用 (变体名去 Instruction 后缀);
// Block* → BlockId; Stack<const Type*> → Stack<TypeId>; 指令在块内序号 InsnRef。
`;

  d += `/// 块索引 (对应 C++ Block*)\npub type BlockId = u32;\n\n`;
  d += `/// 指令稳定序号 (定义位置引用; 填体时进入块时分配)\npub type InsnRef = u32;\n\n`;

  // InstructionKind (变体名去 Instruction 后缀, 避 clippy enum_variant_names)
  d += `/// 原版 enum InstructionKind 1:1 (变体名去公共后缀 Instruction)\n#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]\n#[repr(u8)]\npub enum InstructionKind {\n`;
  for (const n of all) d += `  ${n.replace(/Instruction$/, "")},\n`;
  d += `}\n\n`;

  // DefinitionLocation
  {
    const blk = classBlock("DefinitionLocation", ih);
    const kindBody = blk.body.match(/enum class Kind\s*\{([^}]*)\}/);
    const kinds = kindBody ? [...kindBody[1].matchAll(/\bk(\w+)/g)].map((x) => x[1]) : [];
    d += `/// DefinitionLocation::Kind 1:1\n#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]\n#[repr(u8)]\npub enum DefinitionLocationKind { ${kinds.map((k) => k.replace(/^k/, "")).join(", ")} }\n\n`;
    d += `/// DefinitionLocation 1:1: (kind, 块/指令定位, 槽位)\n#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]\npub struct DefinitionLocation {\n  pub kind: DefinitionLocationKind,\n  pub block: Option<BlockId>,\n  pub instruction: Option<InsnRef>,\n  pub index: usize,\n}\n\n`;
    d += `impl DefinitionLocation {\n  /// 原版 Parameter(index)\n  #[must_use]\n  pub fn parameter(index: usize) -> Self {\n    Self { kind: DefinitionLocationKind::Parameter, block: None, instruction: None, index }\n  }\n\n`;
    d += `  /// 原版 Phi(block, index)\n  #[must_use]\n  pub fn phi(block: BlockId, index: usize) -> Self {\n    Self { kind: DefinitionLocationKind::Phi, block: Some(block), instruction: None, index }\n  }\n\n`;
    d += `  /// 原版 Instruction(instruction, index)\n  #[must_use]\n  pub fn insn(instruction: InsnRef, index: usize) -> Self {\n    Self { kind: DefinitionLocationKind::Instruction, block: None, instruction: Some(instruction), index }\n  }\n\n`;
    d += `  #[must_use]\n  pub fn is_valid(self) -> bool { self.kind != DefinitionLocationKind::Invalid }\n\n`;
    d += `  #[must_use]\n  pub fn is_parameter(self) -> bool { self.kind == DefinitionLocationKind::Parameter }\n\n`;
    d += `  #[must_use]\n  pub fn is_phi(self) -> bool { self.kind == DefinitionLocationKind::Phi }\n\n`;
    d += `  #[must_use]\n  pub fn is_instruction(self) -> bool { self.kind == DefinitionLocationKind::Instruction }\n`;
    d += `}\n\n`;
  }

  // 每条指令结构体: 字段提取 + pos
  for (const n of all) {
    const blk = classBlock(n, ih);
    if (!blk) { console.error(`缺失指令 ${n}`); continue; }
    const fs = memberFields(blk.body);
    d += `/// ${n} 1:1\n#[derive(Debug, Clone, PartialEq, Eq)]\npub struct ${n} {\n  /// 原版 InstructionBase::pos\n  pub pos: SourcePosition,\n`;
    for (const f of fs) d += `  pub ${FNAME[f.name] ?? f.name}: ${T(f.type)},\n`;
    d += `}\n\n`;
  }

  // AbortKind
  {
    const blk = classBlock("Kind", ih.slice(ih.indexOf("struct AbortInstruction")));
    const kinds = blk ? [...blk.body.matchAll(/\bk(\w+)/g)].map((x) => x[1]) : [];
    d += `/// AbortInstruction::Kind 1:1\n#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]\n#[repr(u8)]\npub enum AbortKind { ${kinds.map((k) => k.replace(/^k/, "")).join(", ")} }\n\n`;
    d += `/// 原版 AbortInstruction::KindToString\n#[must_use]\npub fn abort_kind_to_string(kind: AbortKind) -> &'static str {\n  match kind {\n    AbortKind::DebugBreak => "kDebugBreak",\n    AbortKind::Unreachable => "kUnreachable",\n    AbortKind::AssertionFailure => "kAssertionFailure",\n  }\n}\n\n`;
  }

  // Instruction 包装 enum + 分派方法
  const short = (n) => n.replace(/Instruction$/, "");
  d += `/// 全指令合拢: Block 内指令序列元素 (判别 = kind; C++ Clone/Assign 由 derive(Clone) 承担)\n#[derive(Debug, Clone, PartialEq, Eq)]\npub enum Instruction {\n`;
  for (const n of all) d += `  ${short(n)}(${n}),\n`;
  d += `}\n\n`;
  d += `impl Instruction {\n`;
  d += `  /// 原版 kind()\n  #[must_use]\n  pub fn kind(&self) -> InstructionKind {\n    match self {\n`;
  for (const n of all) d += `      Self::${short(n)}(_) => InstructionKind::${short(n)},\n`;
  d += `    }\n  }\n\n`;
  d += `  /// 原版 Mnemonic()\n  #[must_use]\n  pub fn mnemonic(&self) -> &'static str {\n    match self {\n`;
  for (const n of all) d += `      Self::${short(n)}(_) => "${n}",\n`;
  d += `    }\n  }\n\n`;
  d += `  /// 原版 IsBlockTerminator (逐指令常量语义)\n  #[must_use]\n  pub fn is_block_terminator(&self) -> bool {\n    match self {\n`;
  d += `      Self::CallCsaMacroAndBranch(_) => true,\n      Self::Branch(_) | Self::ConstexprBranch(_) | Self::Goto(_) | Self::GotoExternal(_) | Self::Return(_) => true,\n`;
  d += `      Self::CallBuiltin(i) => i.is_tailcall,\n      Self::CallBuiltinPointer(i) => i.is_tailcall,\n      Self::CallRuntime(i) => i.is_tailcall,\n`;
  d += `      Self::Abort(i) => i.kind != AbortKind::DebugBreak,\n      _ => false,\n`;
  d += `    }\n  }\n\n`;
  d += `  /// 原版 AppendSuccessorBlocks\n  pub fn append_successor_blocks(&self, block_list: &mut Vec<BlockId>) {\n    match self {\n`;
  d += `      Self::CallCsaMacro(i) => {\n        if let Some(b) = i.catch_block {\n          block_list.push(b);\n        }\n      }\n`;
  d += `      Self::CallCsaMacroAndBranch(i) => {\n        if let Some(b) = i.catch_block {\n          block_list.push(b);\n        }\n        if let Some(b) = i.return_continuation {\n          block_list.push(b);\n        }\n        block_list.extend(i.label_blocks.iter().copied());\n      }\n`;
  d += `      Self::CallBuiltin(i) => {\n        if let Some(b) = i.catch_block {\n          block_list.push(b);\n        }\n      }\n      Self::CallRuntime(i) => {\n        if let Some(b) = i.catch_block {\n          block_list.push(b);\n        }\n      }\n`;
  d += `      Self::Branch(i) => {\n        block_list.push(i.if_true);\n        block_list.push(i.if_false);\n      }\n      Self::ConstexprBranch(i) => {\n        block_list.push(i.if_true);\n        block_list.push(i.if_false);\n      }\n      Self::Goto(i) => block_list.push(i.destination),\n      _ => {}\n    }\n  }\n\n`;
  d += `  /// 原版 TypeInstruction (栈类型化, 语义填体)\n  pub fn type_instruction(&self, stack: &mut Stack<TypeId>, cfg: &ControlFlowGraph) -> Result<()> {\n    // TODO(填体): 25 条指令逐一类型化\n    let _ = (stack, cfg);\n    Err(Error::Unimplemented)\n  }\n\n`;
  d += `  /// 原版 RecomputeDefinitionLocations (定义位置重算, 语义填体)\n  pub fn recompute_definition_locations(&self, locations: &mut Stack<DefinitionLocation>, worklist: &mut Worklist<BlockId>) -> Result<()> {\n    // TODO(填体)\n    let _ = (locations, worklist);\n    Err(Error::Unimplemented)\n  }\n\n`;
  d += `  /// 原版 InvalidateTransientTypes (自由函数于 InstructionBase)\n  pub fn invalidate_transient_types(&self, stack: &mut Stack<TypeId>) -> Result<()> {\n    // TODO(填体)\n    let _ = stack;\n    Err(Error::Unimplemented)\n  }\n`;
  d += `}\n\n`;

  // GetValueDefinition 族 (逐指令)
  d += `// 原版逐指令 GetValueDefinition/GetExceptionObjectDefinition (定义位置查询, 填体):\n`;
  const valueDefFns = [
    ["PushUninitializedInstruction", "get_value_definition", "DefinitionLocation"],
    ["PushBuiltinPointerInstruction", "get_value_definition", "DefinitionLocation"],
    ["LoadReferenceInstruction", "get_value_definition", "DefinitionLocation"],
    ["LoadBitFieldInstruction", "get_value_definition", "DefinitionLocation"],
    ["StoreBitFieldInstruction", "get_value_definition", "DefinitionLocation"],
    ["MakeLazyNodeInstruction", "get_value_definition", "DefinitionLocation"],
    ["UnsafeCastInstruction", "get_value_definition", "DefinitionLocation"],
    ["CallIntrinsicInstruction", "get_value_definition_count", "usize"],
    ["CallIntrinsicInstruction", "get_value_definition", "DefinitionLocation"],
    ["NamespaceConstantInstruction", "get_value_definition_count", "usize"],
    ["NamespaceConstantInstruction", "get_value_definition", "DefinitionLocation"],
    ["CallCsaMacroInstruction", "get_value_definition_count", "usize"],
    ["CallCsaMacroInstruction", "get_value_definition", "DefinitionLocation"],
    ["CallCsaMacroInstruction", "get_exception_object_definition", "Option<DefinitionLocation>"],
    ["CallCsaMacroAndBranchInstruction", "get_label_count", "usize"],
    ["CallCsaMacroAndBranchInstruction", "get_label_value_definition_count", "usize"],
    ["CallCsaMacroAndBranchInstruction", "get_label_value_definition", "DefinitionLocation"],
    ["CallCsaMacroAndBranchInstruction", "get_value_definition_count", "usize"],
    ["CallCsaMacroAndBranchInstruction", "get_value_definition", "DefinitionLocation"],
    ["CallCsaMacroAndBranchInstruction", "get_exception_object_definition", "Option<DefinitionLocation>"],
    ["CallBuiltinInstruction", "get_value_definition_count", "usize"],
    ["CallBuiltinInstruction", "get_value_definition", "DefinitionLocation"],
    ["CallBuiltinInstruction", "get_exception_object_definition", "Option<DefinitionLocation>"],
    ["CallBuiltinPointerInstruction", "get_value_definition_count", "usize"],
    ["CallBuiltinPointerInstruction", "get_value_definition", "DefinitionLocation"],
    ["CallRuntimeInstruction", "get_value_definition_count", "usize"],
    ["CallRuntimeInstruction", "get_value_definition", "DefinitionLocation"],
    ["CallRuntimeInstruction", "get_exception_object_definition", "Option<DefinitionLocation>"],
  ];
  for (const [cls, m, ret] of valueDefFns) {
    d += `impl ${cls} {\n  /// 原版 ${cls}::${m}\n  pub fn ${m}(&self) -> Result<${ret}> {\n    // TODO(填体)\n    Err(Error::Unimplemented)\n  }\n}\n\n`;
  }
  d += `// CallCsaMacroAndBranchInstruction::GetLabelValueDefinition(label, index) 原双参 →\n// 填体时并入 get_label_value_definition (参数登记于此)。\n`;

  d += sigBlock("src/torque/instructions.cc", "签名清单");
  writeSrc("v8-torque", "instructions.rs", d);
}

// ===========================================================================
// cfg.{h,cc} → cfg.rs
// ===========================================================================
function genCfg() {
  const relH = "src/torque/cfg.h";
  const s = noCmt(readV8(relH));
  let d = banner("src/torque/cfg.js", relH, "CFG: Block/ControlFlowGraph/CfgAssembler");
  d += `
use crate::error::{Error, Result};
use crate::instructions::{BlockId, DefinitionLocation, Instruction};
use crate::types::{TypeId, TypeVector};
use crate::utils::{BottomOffset, Stack, StackRange, Worklist};

// C++ Block* 互联 → BlockId (arena 存于 ControlFlowGraph.blocks, 原版 std::list
// 稳定性由 Vec + 下标承担); cfg_ 回指针删除 (登记偏差, 由持有方补齐)。
`;

  d += `/// 原版 Block 1:1 (cfg_ 回指针删除)\n#[derive(Debug, Clone)]\npub struct Block {\n  id: BlockId,\n  instructions: Vec<Instruction>,\n  input_types: Option<Stack<TypeId>>,\n  input_definitions: Option<Stack<DefinitionLocation>>,\n  is_deferred: bool,\n}\n\n`;
  d += `impl Block {\n  /// 原版构造\n  #[must_use]\n  pub fn new(id: BlockId, input_types: Option<Stack<TypeId>>, is_deferred: bool) -> Self {\n    Self { id, instructions: Vec::new(), input_types, input_definitions: None, is_deferred }\n  }\n\n`;
  d += `  /// 原版 Add\n  pub fn add(&mut self, instruction: Instruction) {\n    debug_assert!(!self.is_complete());\n    self.instructions.push(instruction);\n  }\n\n`;
  d += `  #[must_use]\n  pub fn has_input_types(&self) -> bool { self.input_types.is_some() }\n\n`;
  d += `  /// 原版 InputTypes (前置条件: HasInputTypes())\n  #[must_use]\n  pub fn input_types(&self) -> &Stack<TypeId> {\n    self.input_types.as_ref().expect("input_types 已由 has_input_types 校验")\n  }\n\n`;
  d += `  /// 原版 SetInputTypes (与已登记类型不匹配则报错)\n  pub fn set_input_types(&mut self, input_types: &Stack<TypeId>) -> Result<()> {\n    // TODO(填体): 不匹配路径 ReportError\n    self.input_types = Some(input_types.clone());\n    Ok(())\n  }\n\n`;
  d += `  /// 原版 Retype: 沿指令重放类型化\n  pub fn retype(&mut self) -> Result<()> {\n    // TODO(填体)\n    Err(Error::Unimplemented)\n  }\n\n`;
  d += `  #[must_use]\n  pub fn instructions(&self) -> &[Instruction] { &self.instructions }\n\n`;
  d += `  #[must_use]\n  pub fn instructions_mut(&mut self) -> &mut Vec<Instruction> { &mut self.instructions }\n\n`;
  d += `  /// 原版 IsComplete: 末条指令为块终结\n  #[must_use]\n  pub fn is_complete(&self) -> bool {\n    self.instructions.last().is_some_and(Instruction::is_block_terminator)\n  }\n\n`;
  d += `  #[must_use]\n  pub const fn id(&self) -> BlockId { self.id }\n\n`;
  d += `  #[must_use]\n  pub const fn is_deferred(&self) -> bool { self.is_deferred }\n\n`;
  d += `  /// 原版 MergeInputDefinitions (不一致槽位折为 Phi)\n  pub fn merge_input_definitions(&mut self, input_definitions: &Stack<DefinitionLocation>, worklist: &mut Worklist<BlockId>) -> Result<()> {\n    // TODO(填体): Phi 合并语义\n    let _ = (input_definitions, worklist);\n    Err(Error::Unimplemented)\n  }\n\n`;
  d += `  #[must_use]\n  pub fn has_input_definitions(&self) -> bool { self.input_definitions.is_some() }\n\n`;
  d += `  /// 原版 InputDefinitions (前置条件: HasInputDefinitions())\n  #[must_use]\n  pub fn input_definitions(&self) -> &Stack<DefinitionLocation> {\n    self.input_definitions.as_ref().expect("input_definitions 已由 has_input_definitions 校验")\n  }\n\n`;
  d += `  /// 原版 IsDead\n  #[must_use]\n  pub fn is_dead(&self) -> bool { !self.has_input_definitions() }\n`;
  d += `}\n\n`;

  d += `/// 原版 ControlFlowGraph 1:1\n#[derive(Debug)]\npub struct ControlFlowGraph {\n  blocks: Vec<Block>,\n  start: BlockId,\n  placed_blocks: Vec<BlockId>,\n  end: Option<BlockId>,\n  return_type: Option<TypeVector>,\n  next_block_id: usize,\n}\n\n`;
  d += `impl ControlFlowGraph {\n  /// 原版构造: 建入口块并放置\n  #[must_use]\n  pub fn new(input_types: Stack<TypeId>) -> Self {\n    let mut g = Self {\n      blocks: Vec::new(),\n      start: 0,\n      placed_blocks: Vec::new(),\n      end: None,\n      return_type: None,\n      next_block_id: 0,\n    };\n    let b = g.new_block(Some(input_types), false);\n    g.place_block(b);\n    g\n  }\n\n`;
  d += `  /// 原版 NewBlock\n  pub fn new_block(&mut self, input_types: Option<Stack<TypeId>>, is_deferred: bool) -> BlockId {\n    let id = self.next_block_id as BlockId;\n    self.next_block_id += 1;\n    self.blocks.push(Block::new(id, input_types, is_deferred));\n    id\n  }\n\n`;
  d += `  /// 原版 PlaceBlock\n  pub fn place_block(&mut self, block: BlockId) {\n    self.placed_blocks.push(block);\n  }\n\n`;
  d += `  /// 原版 UnplaceBlockIf\n  pub fn unplace_block_if(&mut self, predicate: impl Fn(BlockId) -> bool) {\n    self.placed_blocks.retain(|b| !predicate(*b));\n  }\n\n`;
  d += `  #[must_use]\n  pub const fn start(&self) -> BlockId { self.start }\n\n`;
  d += `  #[must_use]\n  pub const fn end(&self) -> Option<BlockId> { self.end }\n\n`;
  d += `  pub fn set_end(&mut self, end: BlockId) {\n    self.end = Some(end);\n  }\n\n`;
  d += `  /// 原版 SetReturnType (不一致即编译错误)\n  pub fn set_return_type(&mut self, t: TypeVector) -> Result<()> {\n    if self.return_type.is_none() {\n      self.return_type = Some(t);\n      return Ok(());\n    }\n    // TODO(填体): 与既有返回型比对, 不一致 → Error::Message\n    if self.return_type.as_ref() != Some(&t) {\n      return Err(Error::Unimplemented);\n    }\n    Ok(())\n  }\n\n`;
  d += `  /// 原版 blocks(): 已放置块序\n  #[must_use]\n  pub fn blocks(&self) -> &[BlockId] { &self.placed_blocks }\n\n`;
  d += `  #[must_use]\n  pub fn block(&self, id: BlockId) -> &Block { &self.blocks[id as usize] }\n\n`;
  d += `  #[must_use]\n  pub fn block_mut(&mut self, id: BlockId) -> &mut Block { &mut self.blocks[id as usize] }\n\n`;
  d += `  /// 原版 NumberOfBlockIds\n  #[must_use]\n  pub fn number_of_block_ids(&self) -> usize { self.next_block_id }\n\n`;
  d += `  /// 原版 ParameterCount\n  #[must_use]\n  pub fn parameter_count(&self) -> usize {\n    self.block(self.start).input_types().size()\n  }\n`;
  d += `}\n\n`;

  d += `/// 原版 CfgAssembler 1:1\n#[derive(Debug)]\npub struct CfgAssembler {\n  current_stack: Stack<TypeId>,\n  cfg: ControlFlowGraph,\n  current_block: BlockId,\n}\n\n`;
  d += `impl CfgAssembler {\n  /// 原版构造\n  #[must_use]\n  pub fn new(input_types: Stack<TypeId>) -> Self {\n    let current_stack = input_types.clone();\n    let cfg = ControlFlowGraph::new(input_types);\n    let current_block = cfg.start();\n    Self { current_stack, cfg, current_block }\n  }\n\n`;
  d += `  /// 原版 Result() (收尾 + 优化 + 输入定义计算)\n  pub fn result(&mut self) -> Result<ControlFlowGraph> {\n    // TODO(填体): OptimizeCfg + ComputeInputDefinitions\n    if !self.current_block_is_complete() {\n      self.cfg.set_end(self.current_block);\n    }\n    Err(Error::Unimplemented)\n  }\n\n`;
  d += `  /// 原版 NewBlock\n  pub fn new_block(&mut self, input_types: Option<Stack<TypeId>>, is_deferred: bool) -> BlockId {\n    self.cfg.new_block(input_types, is_deferred)\n  }\n\n`;
  d += `  #[must_use]\n  pub fn current_block_is_complete(&self) -> bool {\n    self.cfg.block(self.current_block).is_complete()\n  }\n\n`;
  d += `  /// 原版 CfgIsComplete\n  #[must_use]\n  pub fn cfg_is_complete(&self) -> bool {\n    self.cfg.blocks().iter().all(|b| {\n      self.cfg.end() == Some(*b) || self.cfg.block(*b).is_complete()\n    })\n  }\n\n`;
  d += `  /// 原版 Emit: 先类型化再入块\n  pub fn emit(&mut self, instruction: Instruction) -> Result<()> {\n    // TODO(填体): type_instruction 消费栈\n    self.cfg.block_mut(self.current_block).add(instruction);\n    Ok(())\n  }\n\n`;
  d += `  #[must_use]\n  pub fn current_stack(&self) -> &Stack<TypeId> { &self.current_stack }\n\n`;
  d += `  #[must_use]\n  pub fn top_range(&self, slot_count: usize) -> StackRange {\n    self.current_stack.top_range(slot_count)\n  }\n\n`;
  d += `  /// 原版 Bind\n  pub fn bind(&mut self, block: BlockId) -> Result<()> {\n    // TODO(填体)\n    let _ = block;\n    Err(Error::Unimplemented)\n  }\n\n`;
  d += `  /// 原版 Goto\n  pub fn goto(&mut self, block: BlockId) -> Result<()> {\n    // TODO(填体)\n    let _ = block;\n    Err(Error::Unimplemented)\n  }\n\n`;
  d += `  /// 原版 Goto(block, preserved_slots): 保留顶部槽位版\n  pub fn goto_preserved(&mut self, block: BlockId, preserved_slots: usize) -> Result<StackRange> {\n    // TODO(填体)\n    let _ = (block, preserved_slots);\n    Err(Error::Unimplemented)\n  }\n\n`;
  d += `  /// 原版 Branch (栈顶 bool 弹出后分支)\n  pub fn branch(&mut self, if_true: BlockId, if_false: BlockId) -> Result<()> {\n    // TODO(填体)\n    let _ = (if_true, if_false);\n    Err(Error::Unimplemented)\n  }\n\n`;
  d += `  /// 原版 DeleteRange\n  pub fn delete_range(&mut self, range: StackRange) {\n    self.current_stack.delete_range(range);\n  }\n\n`;
  d += `  /// 原版 DropTo\n  pub fn drop_to(&mut self, new_level: BottomOffset) {\n    let n = self.current_stack.above_top().offset - new_level.offset;\n    self.current_stack.pop_many(n);\n  }\n\n`;
  d += `  /// 原版 Peek(range, type)\n  pub fn peek(&mut self, range: StackRange, type_: Option<TypeId>) -> Result<StackRange> {\n    // TODO(填体)\n    let _ = (range, type_);\n    Err(Error::Unimplemented)\n  }\n\n`;
  d += `  /// 原版 Poke(destination, origin, type)\n  pub fn poke(&mut self, destination: StackRange, origin: StackRange, type_: Option<TypeId>) -> Result<()> {\n    // TODO(填体)\n    let _ = (destination, origin, type_);\n    Err(Error::Unimplemented)\n  }\n\n`;
  d += `  /// 原版 Print\n  pub fn print(&mut self, s: String) -> Result<()> {\n    // TODO(填体): PrintErrorInstruction 发射\n    let _ = s;\n    Err(Error::Unimplemented)\n  }\n\n`;
  d += `  /// 原版 AssertionFailure\n  pub fn assertion_failure(&mut self, message: String) -> Result<()> {\n    // TODO(填体)\n    let _ = message;\n    Err(Error::Unimplemented)\n  }\n\n`;
  d += `  /// 原版 Unreachable\n  pub fn unreachable(&mut self) -> Result<()> {\n    // TODO(填体)\n    Err(Error::Unimplemented)\n  }\n\n`;
  d += `  /// 原版 DebugBreak\n  pub fn debug_break(&mut self) -> Result<()> {\n    // TODO(填体)\n    Err(Error::Unimplemented)\n  }\n\n`;
  d += `  /// 原版 OptimizeCfg\n  pub fn optimize_cfg(&mut self) {\n    // TODO(填体)\n  }\n\n`;
  d += `  /// 原版 ComputeInputDefinitions\n  pub fn compute_input_definitions(&mut self) {\n    // TODO(填体)\n  }\n`;
  d += `}\n\n`;

  d += `/// 原版 CfgAssemblerScopedTemporaryBlock — Rust 借用模型下改显式 enter/exit 对\n/// (原版 RAII swap current_block/current_stack; Drop 守卫留填体设计)\n#[derive(Debug)]\npub struct CfgAssemblerScopedTemporaryBlock {\n  pub saved_stack: Stack<TypeId>,\n  pub saved_block: BlockId,\n}\n\nimpl CfgAssemblerScopedTemporaryBlock {\n  /// 原版构造 (swap 进临时块)\n  pub fn enter(_assembler: &mut CfgAssembler, block: BlockId) -> Result<Self> {\n    // TODO(填体): swap 语义\n    let _ = block;\n    Err(Error::Unimplemented)\n  }\n\n  /// 原版析构 (swap 回)\n  pub fn exit(self, _assembler: &mut CfgAssembler) {\n    // TODO(填体)\n    let _ = self;\n  }\n}\n`;

  d += `\n` + sigBlock("src/torque/cfg.cc", "签名清单");
  writeSrc("v8-torque", "cfg.rs", d);
}

// ===========================================================================
// 主流程
// ===========================================================================
genInstructions();
genCfg();
console.log("== v8-torque CFG 组直出完毕: instructions.rs + cfg.rs ==");
