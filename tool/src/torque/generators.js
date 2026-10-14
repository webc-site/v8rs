#!/usr/bin/env bun
// v8/src/torque 生成器组 → 直出
//   torque-code-generator.{h,cc}     → torque_code_generator.rs
//   csa-generator.{h,cc}             → csa_generator.rs
//   cc-generator.{h,cc}              → cc_generator.rs
//   tsa-generator.{h,cc}             → tsa_generator.rs (TSAGenerator 类在 .cc 中; 原版唯一
//                                        AstVisitor 继承者, trait impl 骨架在此)
//   class-debug-reader-generator.cc  → class_debug_reader_generator.rs
//   instance-type-generator.cc       → instance_type_generator.rs
//   cpp-builder.{h,cc}               → cpp_builder.rs
// 继承树映射: TorqueCodeGenerator 基类 → 公共字段结构体 + 后端结构体组合;
// ostream 输出 → String 缓冲 (emit 系列返回生成文本或写入 File.buf)。
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
  // scope 栈: ns=命名空间/extern 块, cls=class/struct 块 (成员继续收集), fn=函数体 (不收集)
  const scope = [];
  let unit = "", startLine = 0;
  const top = () => scope[scope.length - 1] ?? "ns";
  const fin = () => {
    const s = unit.trim().replace(/\s+/g, " ").replace(/;$/, "");
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
        const t = top();
        const u = unit.trim();
        // 类头判据: class/struct 关键字且无 `(` (排除函数签名与 template <class T> 前缀)
        const isClassHead = !u.includes("(") && /^(class|struct)\b/.test(u);
        if (t === "ns") {
          if (u === "" || /^(namespace|extern)\b/.test(u)) { scope.push("ns"); unit = ""; continue; }
          // 类/结构块: 不收类头签名, 成员方法继续收集 (P1-3: tsa-generator.cc 类内联方法)
          if (isClassHead) { scope.push("cls"); unit = ""; continue; }
          fin(); scope.push("fn"); unit = ""; continue;
        }
        if (t === "cls") {
          if (u !== "") fin(); // 类内方法定义: 出条后入函数体
          // 嵌套类/结构继续按类块收集 (template <T> class X : Y { 也归 cls)
          scope.push(!u.includes("(") && /\b(class|struct)\b/.test(u) ? "cls" : "fn");
          unit = "";
          continue;
        }
        scope.push("fn");
      } else if (ch === "}") {
        scope.pop();
      } else if (top() === "ns" || top() === "cls") {
        if (!unit.trim()) startLine = i + 1;
        unit += ch;
      }
    }
    // `;` 结尾单元单独出条 (P2-3: 防多条声明粘连 1 条), 无 `(` 的纯声明仅清空
    if (/;\s*$/.test(raw) && top() !== "fn") {
      if (unit.includes("(")) fin();
      unit = "";
    }
  }
  return out;
}

function sigBlock(f, note = "签名清单 (填体正交任务源)") {
  const s = readV8(f);
  let dd = `// ── 原版 ${f} ──\n`;
  const sigs = topSigs(s);
  // 防静默: 清单条数按文件行数比例校验 (< 行数/200 视为抽取异常)
  const lineCount = s.split("\n").length;
  if (f.endsWith(".cc") && sigs.length < lineCount / 200) {
    console.error(`[torque] ${f} ${lineCount} 行仅抽到 ${sigs.length} 条签名 (< 行数/200) — noCmt/topSigs 异常, 防静默检查`);
  }
  for (const x of sigs) dd += `// L${x.line}: ${x.sig}\n`;
  return dd;
}

const CAMEL1 = /([a-z0-9])([A-Z])/g, CAMEL2 = /([A-Z]+)([A-Z][a-z])/g;
const snake = (s) => s.replaceAll(CAMEL1, "$1_$2").replaceAll(CAMEL2, "$1_$2").toLowerCase();

// ast 节点清单 (TsaGenerator trait impl 用)
function allNodeKinds() {
  const src = readV8("src/torque/ast.h");
  const noComment = src.replace(/\/\*[\s\S]*?\*\//g, "").replace(/\/\/.*$/gm, "");
  const expandList = (name, text) => {
    const re = new RegExp(`#define\\s+${name}\\(V\\)([\\s\\S]*?)(?=\\n\\n|#define|struct|class|enum)`);
    const m = text.match(re);
    if (!m) return [];
    return [...m[1].matchAll(/V\((\w+)\)/g)].map((mm) => mm[1]);
  };
  const exprKinds = expandList("AST_EXPRESSION_NODE_KIND_LIST", noComment);
  const typeExprKinds = expandList("AST_TYPE_EXPRESSION_NODE_KIND_LIST", noComment);
  const stmtKinds = expandList("AST_STATEMENT_NODE_KIND_LIST", noComment);
  const typeDeclKinds = expandList("AST_TYPE_DECLARATION_NODE_KIND_LIST", noComment);
  const declOnly = expandList("AST_DECLARATION_NODE_KIND_LIST", noComment).filter((k) => !typeDeclKinds.includes(k));
  return [...exprKinds, ...typeExprKinds, ...stmtKinds, ...typeDeclKinds, ...declOnly, "Identifier", "TryHandler", "ClassBody"];
}

// ===========================================================================
// torque-code-generator.{h,cc} → torque_code_generator.rs
// ===========================================================================
function genCodeGenerator() {
  const relH = "src/torque/torque-code-generator.h";
  let d = banner("src/torque/generators.js", relH, "代码生成器基类 (公共字段 + 指令发射骨架)");
  d += `
use crate::error::{Error, Result};
use crate::instructions::{BlockId, DefinitionLocation, Instruction};
use crate::source_positions::SourcePosition;
use crate::utils::Stack;
use std::collections::BTreeMap;

// C++ 持 const ControlFlowGraph& + ostream* → Rust: cfg 经方法传入, 输出走 String
// 缓冲 (out/out_decls 合并由调用方拆分, 登记偏差)。
`;

  d += `/// 原版 TorqueCodeGenerator 基类字段 1:1\n#[derive(Debug, Default)]\npub struct TorqueCodeGenerator {\n  /// 原版 out_ (生成代码缓冲)\n  pub out: String,\n  /// 原版 out_decls_ (声明区缓冲)\n  pub out_decls: String,\n  pub fresh_id: usize,\n  pub previous_position: SourcePosition,\n  pub location_map: BTreeMap<DefinitionLocation, String>,\n}\n\n`;
  d += `impl TorqueCodeGenerator {\n  /// 原版 DefinitionToVariable\n  #[must_use]\n  pub fn definition_to_variable(&self, location: &DefinitionLocation) -> Option<&str> {\n    // 原版: phi → phi_bb{id}_{index}; 其余查表\n    if location.is_phi() {\n      return None; // TODO(填体): phi 命名需 cfg 块 id\n    }\n    self.location_map.get(location).map(String::as_str)\n  }\n\n`;
  d += `  /// 原版 SetDefinitionVariable\n  pub fn set_definition_variable(&mut self, definition: DefinitionLocation, str_: String) {\n    self.location_map.insert(definition, str_);\n  }\n\n`;
  d += `  /// 原版 FreshNodeName\n  #[must_use]\n  pub fn fresh_node_name(&mut self) -> String {\n    let n = self.fresh_id;\n    self.fresh_id += 1;\n    format!("tmp{n}")\n  }\n\n`;
  d += `  /// 原版 FreshCatchName\n  #[must_use]\n  pub fn fresh_catch_name(&mut self) -> String {\n    let n = self.fresh_id;\n    self.fresh_id += 1;\n    format!("catch{n}")\n  }\n\n`;
  d += `  /// 原版 FreshLabelName\n  #[must_use]\n  pub fn fresh_label_name(&mut self) -> String {\n    let n = self.fresh_id;\n    self.fresh_id += 1;\n    format!("label{n}")\n  }\n\n`;
  d += `  /// 原版 BlockName\n  #[must_use]\n  pub fn block_name(block: BlockId) -> String {\n    format!("block{block}")\n  }\n\n`;
  d += `  /// 原版 IsEmptyInstruction (静态)\n  #[must_use]\n  pub fn is_empty_instruction(instruction: &Instruction) -> bool {\n    // TODO(填体)\n    let _ = instruction;\n    false\n  }\n\n`;
  d += `  /// 原版 EmitInstruction (后端无关部分直接发射, 后端相关虚分派)\n  pub fn emit_instruction(&mut self, instruction: &Instruction, stack: &mut Stack<String>) -> Result<()> {\n    // TODO(填体): 后端无关 Peek/Poke/DeleteRange 直发; 其余转后端\n    let _ = (instruction, stack);\n    Err(Error::Unimplemented)\n  }\n\n`;
  d += `  /// 原版 EmitSourcePosition (纯虚 → 后端实现)\n  pub fn emit_source_position(&mut self, pos: SourcePosition, always_emit: bool) -> Result<()> {\n    // TODO(填体)\n    let _ = (pos, always_emit);\n    Err(Error::Unimplemented)\n  }\n`;
  d += `}\n\n`;
  d += sigBlock("src/torque/torque-code-generator.cc", "签名清单");
  writeSrc("v8-torque", "torque_code_generator.rs", d);
}

// ===========================================================================
// csa-generator / cc-generator
// ===========================================================================
function genBackendGenerators() {
  let d = banner("src/torque/generators.js", "src/torque/csa-generator.h", "CSA 后端生成器");
  d += `
use crate::cfg::ControlFlowGraph;
use crate::declarable::BuiltinKind;
use crate::error::{Error, Result};
use crate::instructions::{BlockId, DefinitionLocation, Instruction};
use crate::source_positions::SourcePosition;
use crate::torque_code_generator::TorqueCodeGenerator;
use crate::types::{TypeId, TypeVector, VisitResult};
use crate::utils::Stack;

// C++ CSAGenerator : TorqueCodeGenerator → 组合 (base 字段 + 自有字段)。
`;

  d += `/// 原版 CSAGenerator::ARGUMENTS_VARIABLE_STRING\npub const ARGUMENTS_VARIABLE_STRING: &str = "arguments";\n\n`;
  d += `/// 原版 CSAGenerator 1:1\n#[derive(Debug, Default)]\npub struct CsaGenerator {\n  pub base: TorqueCodeGenerator,\n  pub linkage: Option<BuiltinKind>,\n}\n\n`;
  d += `impl CsaGenerator {\n  /// 原版 EmitGraph\n  pub fn emit_graph(&mut self, cfg: &ControlFlowGraph, parameters: Stack<String>) -> Result<Option<Stack<String>>> {\n    // TODO(填体)\n    let _ = (cfg, parameters);\n    Err(Error::Unimplemented)\n  }\n\n`;
  d += `  /// 原版 EmitCSAValue (静态)\n  pub fn emit_csa_value(result: &VisitResult, values: &Stack<String>, out: &mut String) -> Result<()> {\n    // TODO(填体)\n    let _ = (result, values, out);\n    Err(Error::Unimplemented)\n  }\n\n`;
  d += `  /// 原版 EmitSourcePosition (override)\n  pub fn emit_source_position(&mut self, pos: SourcePosition, always_emit: bool) -> Result<()> {\n    // TODO(填体)\n    let _ = (pos, always_emit);\n    Err(Error::Unimplemented)\n  }\n\n`;
  d += `  /// 原版 PreCallableExceptionPreparation\n  pub fn pre_callable_exception_preparation(&mut self, catch_block: Option<BlockId>) -> Result<String> {\n    // TODO(填体)\n    let _ = catch_block;\n    Err(Error::Unimplemented)\n  }\n\n`;
  d += `  /// 原版 PostCallableExceptionPreparation\n  pub fn post_callable_exception_preparation(&mut self, catch_name: &str, return_type: TypeId, catch_block: Option<BlockId>, stack: &mut Stack<String>, exception_object_definition: Option<DefinitionLocation>) -> Result<()> {\n    // TODO(填体)\n    let _ = (catch_name, return_type, catch_block, stack, exception_object_definition);\n    Err(Error::Unimplemented)\n  }\n\n`;
  d += `  /// 原版 ProcessArgumentsCommon\n  pub fn process_arguments_common(&mut self, parameter_types: &TypeVector, constexpr_arguments: Vec<String>, stack: &mut Stack<String>) -> Result<Vec<String>> {\n    // TODO(填体)\n    let _ = (parameter_types, constexpr_arguments, stack);\n    Err(Error::Unimplemented)\n  }\n\n`;
  d += `  /// 原版 EmitBlock\n  pub fn emit_block(&mut self, cfg: &ControlFlowGraph, block: BlockId) -> Result<Stack<String>> {\n    // TODO(填体)\n    let _ = (cfg, block);\n    Err(Error::Unimplemented)\n  }\n\n`;
  d += `  /// 原版 EmitInstruction(T) × 22 (后端相关逐指令重载, 见 .cc 清单)\n  pub fn emit_instruction(&mut self, cfg: &ControlFlowGraph, instruction: &Instruction, stack: &mut Stack<String>) -> Result<()> {\n    // TODO(填体): 逐指令分支 (TORQUE_BACKEND_DEPENDENT_INSTRUCTION_LIST)\n    let _ = (cfg, instruction, stack);\n    Err(Error::Unimplemented)\n  }\n`;
  d += `}\n\n`;
  d += sigBlock("src/torque/csa-generator.cc", "签名清单 (1085 行, CSA 发射大头)");
  writeSrc("v8-torque", "csa_generator.rs", d);

  d = banner("src/torque/generators.js", "src/torque/cc-generator.h", "CC 后端生成器");
  d += `
use crate::cfg::ControlFlowGraph;
use crate::error::{Error, Result};
use crate::instructions::{BlockId, DefinitionLocation, Instruction};
use crate::source_positions::SourcePosition;
use crate::torque_code_generator::TorqueCodeGenerator;
use crate::types::{TypeVector, VisitResult};
use crate::utils::Stack;

// C++ CCGenerator : TorqueCodeGenerator → 组合。
`;

  d += `/// 原版 CCGenerator 1:1\n#[derive(Debug, Default)]\npub struct CcGenerator {\n  pub base: TorqueCodeGenerator,\n  pub is_cc_debug: bool,\n}\n\n`;
  d += `impl CcGenerator {\n  /// 原版 EmitGraph\n  pub fn emit_graph(&mut self, cfg: &ControlFlowGraph, parameters: Stack<String>) -> Result<Option<Stack<String>>> {\n    // TODO(填体)\n    let _ = (cfg, parameters);\n    Err(Error::Unimplemented)\n  }\n\n`;
  d += `  /// 原版 EmitCCValue (静态)\n  pub fn emit_cc_value(result: &VisitResult, values: &Stack<String>, out: &mut String) -> Result<()> {\n    // TODO(填体)\n    let _ = (result, values, out);\n    Err(Error::Unimplemented)\n  }\n\n`;
  d += `  /// 原版 EmitSourcePosition (override)\n  pub fn emit_source_position(&mut self, pos: SourcePosition, always_emit: bool) -> Result<()> {\n    // TODO(填体)\n    let _ = (pos, always_emit);\n    Err(Error::Unimplemented)\n  }\n\n`;
  d += `  /// 原版 EmitGoto\n  pub fn emit_goto(&mut self, destination: BlockId, stack: &mut Stack<String>, indentation: &str) -> Result<()> {\n    // TODO(填体)\n    let _ = (destination, stack, indentation);\n    Err(Error::Unimplemented)\n  }\n\n`;
  d += `  /// 原版 ProcessArgumentsCommon\n  pub fn process_arguments_common(&mut self, parameter_types: &TypeVector, constexpr_arguments: Vec<String>, stack: &mut Stack<String>) -> Result<Vec<String>> {\n    // TODO(填体)\n    let _ = (parameter_types, constexpr_arguments, stack);\n    Err(Error::Unimplemented)\n  }\n\n`;
  d += `  /// 原版 EmitBlock\n  pub fn emit_block(&mut self, cfg: &ControlFlowGraph, block: BlockId) -> Result<Stack<String>> {\n    // TODO(填体)\n    let _ = (cfg, block);\n    Err(Error::Unimplemented)\n  }\n\n`;
  d += `  /// 原版 EmitInstruction(T) × 22 (后端相关逐指令重载, 见 .cc 清单)\n  pub fn emit_instruction(&mut self, cfg: &ControlFlowGraph, instruction: &Instruction, stack: &mut Stack<String>) -> Result<()> {\n    // TODO(填体)\n    let _ = (cfg, instruction, stack);\n    Err(Error::Unimplemented)\n  }\n\n`;
  d += `  /// 原版 exception_object_definition_location (cc 侧辅助)\n  pub fn exception_object_definition_location(&self, def: Option<DefinitionLocation>) -> Result<Option<DefinitionLocation>> {\n    // TODO(填体)\n    let _ = def;\n    Err(Error::Unimplemented)\n  }\n`;
  d += `}\n\n`;
  d += sigBlock("src/torque/cc-generator.cc", "签名清单");
  writeSrc("v8-torque", "cc_generator.rs", d);
}

// ===========================================================================
// tsa-generator + AstVisitor impl 骨架
// ===========================================================================
function genTsaGenerator() {
  const kinds = allNodeKinds();
  let d = banner("src/torque/generators.js", "src/torque/tsa-generator.h", "TSA 生成器 (TSAGenerator 类在 .cc; 原版唯一 AstVisitor 继承者)");
  d += `
use crate::ast::{Ast, NodeId};
use crate::ast_visitor::AstVisitor;
use crate::error::{Error, Result};

// 原版 tsa-generator.h 仅声明 GenerateTSA; TSAGenerator 类 (继承 AstVisitor) 定义于
// tsa-generator.cc, 此处上提为公开结构体并给出 trait impl 骨架 (60 节点逐一 Err 占位)。
`;

  d += `/// 原版 GenerateTSA\npub fn generate_tsa(ast: &mut Ast, output_directory: &str) -> Result<()> {\n  // TODO(填体): TSAGenerator 构造 + Run\n  let _ = (ast, output_directory);\n  Err(Error::Unimplemented)\n}\n\n`;

  d += `/// 原版 TSAGenerator (tsa-generator.cc 内定义, 继承 AstVisitor<TSAGenerator>)\n#[derive(Debug, Default)]\npub struct TsaGenerator {\n  pub out: String,\n  pub out_decls: String,\n}\n\n`;
  d += `impl AstVisitor for TsaGenerator {\n`;
  for (const k of kinds) {
    d += `  fn visit_${snake(k)}(&mut self, _ast: &mut Ast, _node: NodeId) -> Result<Option<NodeId>> {\n    // TODO(填体): 原版 TSAGenerator::Visit${k}\n    Err(Error::Unimplemented)\n  }\n\n`;
  }
  d += `}\n\n`;
  d += `// 原版 TSAGenerator 自有方法 (GenerateClass/GenerateStruct 等) 见 .cc 清单;\n// 类内枚举/字段随填体上提。\n`;
  d += sigBlock("src/torque/tsa-generator.cc", "签名清单 (1808 行)");
  writeSrc("v8-torque", "tsa_generator.rs", d);
}

// ===========================================================================
// class-debug-reader-generator.cc / instance-type-generator.cc
// ===========================================================================
function genAuxGenerators() {
  let d = banner("src/torque/generators.js", "src/torque/class-debug-reader-generator.cc", "类调试读取器生成器 (仅 .cc)");
  d += `
use crate::error::{Error, Result};

// 原版无独立头文件; 入口由 implementation_visitor.rs 的 GenerateClassDebugReaders 调用。
/// 原版 GenerateClassDebugReader 系列入口
pub fn generate_class_debug_readers(output_directory: &str) -> Result<()> {
  // TODO(填体)
  let _ = output_directory;
  Err(Error::Unimplemented)
}
`;
  d += sigBlock("src/torque/class-debug-reader-generator.cc", "签名清单");
  writeSrc("v8-torque", "class_debug_reader_generator.rs", d);

  d = banner("src/torque/generators.js", "src/torque/instance-type-generator.cc", "InstanceType 生成器 (仅 .cc)");
  d += `
use crate::error::{Error, Result};

// 原版无独立头文件; 入口由 implementation_visitor.rs 的 GenerateInstanceTypes 调用。
/// 原版 GenerateInstanceTypes 系列
pub fn generate_instance_types(output_directory: &str) -> Result<()> {
  // TODO(填体)
  let _ = output_directory;
  Err(Error::Unimplemented)
}
`;
  d += sigBlock("src/torque/instance-type-generator.cc", "签名清单");
  writeSrc("v8-torque", "instance_type_generator.rs", d);
}

// ===========================================================================
// cpp-builder.{h,cc} → cpp_builder.rs (C++ 代码构建辅助, 大部分可机械实现)
// ===========================================================================
function genCppBuilder() {
  const relH = "src/torque/cpp-builder.h";
  const s = noCmt(readV8(relH));
  let d = banner("src/torque/generators.js", relH, "C++ 代码构建辅助 (类/函数/文件缓冲)");
  d += `
use crate::error::{Error, Result};
use crate::source_positions::SourcePosition;
use std::collections::BTreeSet;
use std::mem;

// 原版面向 ostream 的构建器 → Rust String 缓冲; include 语义随生成器定型复核
// (Rust 版产出 Rust 代码, cpp-builder 主要服务 cc-debug 输出路径)。
`;

  d += `/// 原版 TemplateParameter 1:1\n#[derive(Debug, Clone, Default)]\npub struct TemplateParameter {\n  pub name: String,\n  pub type_: String,\n}\n\n`;
  d += `impl TemplateParameter {\n  #[must_use]\n  pub fn named(name: String) -> Self {\n    Self { name, type_: String::new() }\n  }\n\n`;
  d += `  #[must_use]\n  pub fn typed(type_: String, name: String) -> Self {\n    Self { name, type_ }\n  }\n`;
  d += `}\n\n`;

  d += `/// 原版 cpp::Class 1:1\n#[derive(Debug, Clone, Default)]\npub struct Class {\n  template_parameters: Vec<TemplateParameter>,\n  name: String,\n}\n\n`;
  d += `impl Class {\n  #[must_use]\n  pub fn new(name: String) -> Self {\n    Self { template_parameters: Vec::new(), name }\n  }\n\n`;
  d += `  #[must_use]\n  pub fn with_template_parameters(template_parameters: Vec<TemplateParameter>, name: String) -> Self {\n    Self { template_parameters, name }\n  }\n\n`;
  d += `  /// 原版 GetName\n  #[must_use]\n  pub fn name(&self) -> &str {\n    &self.name\n  }\n\n`;
  d += `  /// 原版 GetTemplateParameters\n  #[must_use]\n  pub fn template_parameters(&self) -> &[TemplateParameter] {\n    &self.template_parameters\n  }\n`;
  d += `}\n\n`;

  // FUNCTION_FLAG_LIST X-macro 展开
  const flagList = s.match(/#define FUNCTION_FLAG_LIST\(V\) \\\n((?:\s+V\(\w+, \s*0x[0-9a-fA-F]+\)[ \t]*\\\n)+)/);
  const flags = [...(flagList?.[1] ?? "").matchAll(/V\((\w+), (0x[0-9a-fA-F]+)\)/g)].map(([, n, v]) => [n, v]);
  if (!flags.length) throw new Error("FUNCTION_FLAG_LIST 未提取");
  d += `/// 原版 FUNCTION_FLAG_LIST X-macro 展开 (Function::FunctionFlag)\n#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]\n#[repr(u32)]\npub enum FunctionFlag {\n`;
  for (const [n, v] of flags) d += `  ${n} = ${v},\n`;
  d += `}\n\n`;
  d += `/// 原版 base::Flags<FunctionFlag> → 位掩码 u32\npub type FunctionFlags = u32;\n\n`;

  d += `/// 原版 Function::Parameter 1:1\n#[derive(Debug, Clone, Default)]\npub struct Parameter {\n  pub type_: String,\n  pub name: String,\n  pub default_value: String,\n}\n\n`;
  d += `impl Parameter {\n  #[must_use]\n  pub fn new(type_: String, name: String, default_value: String) -> Self {\n    Self { type_, name, default_value }\n  }\n}\n\n`;

  d += `/// 原版 cpp::Function 1:1 (owning_class 指针 → Option<String> 类名)\n#[derive(Debug, Clone, Default)]\npub struct Function {\n  pub pos: SourcePosition,\n  pub owning_class: Option<String>,\n  pub description: String,\n  name: String,\n  return_type: String,\n  parameters: Vec<Parameter>,\n  flags: FunctionFlags,\n}\n\n`;
  d += `impl Function {\n  pub const K_AUTOMATIC_INDENTATION: i32 = -1;\n\n`;
  d += `  /// 原版 Function(name)\n  #[must_use]\n  pub fn new(name: String) -> Self {\n    Self { pos: SourcePosition::invalid(), owning_class: None, description: String::new(), name, return_type: String::new(), parameters: Vec::new(), flags: 0 }\n  }\n\n`;
  d += `  /// 原版 Function(owner, name)\n  #[must_use]\n  pub fn in_class(owner: &Class, name: String) -> Self {\n    let mut f = Self::new(name);\n    f.owning_class = Some(owner.name().to_owned());\n    f\n  }\n\n`;
  d += `  /// 原版 DefaultGetter\n  #[must_use]\n  pub fn default_getter(return_type: String, owner: &Class, name: String) -> Self {\n    let mut f = Self::in_class(owner, name);\n    f.return_type = return_type;\n    f.set_flag(FunctionFlag::Inline, true);\n    f.set_flag(FunctionFlag::Const, true);\n    f\n  }\n\n`;
  d += `  /// 原版 DefaultSetter\n  #[must_use]\n  pub fn default_setter(owner: &Class, name: String, parameter_type: String, parameter_name: String) -> Self {\n    let mut f = Self::in_class(owner, name);\n    f.return_type = "void".to_owned();\n    f.parameters.push(Parameter::new(parameter_type, parameter_name, String::new()));\n    f.set_flag(FunctionFlag::Inline, true);\n    f\n  }\n\n`;
  d += `  /// 原版 SetFlag\n  pub fn set_flag(&mut self, flag: FunctionFlag, value: bool) {\n    if value {\n      self.flags |= flag as u32;\n    } else {\n      self.flags &= !(flag as u32);\n    }\n  }\n\n`;
  d += `  /// 原版 HasFlag\n  #[must_use]\n  pub fn has_flag(&self, flag: FunctionFlag) -> bool {\n    (self.flags & flag as u32) == flag as u32\n  }\n\n`;
  // ACCESSOR 展开SetXxx/IsXxx
  for (const [n] of flags) {
    d += `  /// 原版 Set${n} / Is${n}\n  pub fn set_${snake(n)}(&mut self, v: bool) {\n    self.set_flag(FunctionFlag::${n}, v);\n  }\n\n  #[must_use]\n  pub fn is_${snake(n)}(&self) -> bool {\n    self.has_flag(FunctionFlag::${n})\n  }\n\n`;
  }
  d += `  /// 原版 SetDescription / SetName / SetReturnType\n  pub fn set_description(&mut self, description: String) {\n    self.description = description;\n  }\n\n  pub fn set_name(&mut self, name: String) {\n    self.name = name;\n  }\n\n  pub fn set_return_type(&mut self, return_type: String) {\n    self.return_type = return_type;\n  }\n\n`;
  d += `  /// 原版 AddParameter\n  pub fn add_parameter(&mut self, type_: String, name: String, default_value: String) {\n    self.parameters.push(Parameter::new(type_, name, default_value));\n  }\n\n`;
  d += `  /// 原版 InsertParameter\n  pub fn insert_parameter(&mut self, index: usize, type_: String, name: String, default_value: String) {\n    self.parameters.insert(index, Parameter::new(type_, name, default_value));\n  }\n\n`;
  d += `  /// 原版 GetParameters\n  #[must_use]\n  pub fn parameters(&self) -> &[Parameter] {\n    &self.parameters\n  }\n\n`;
  d += `  /// 原版 GetParameterNames\n  #[must_use]\n  pub fn parameter_names(&self) -> Vec<&str> {\n    self.parameters.iter().map(|p| p.name.as_str()).collect()\n  }\n\n`;
  d += `  /// 原版 PrintDeclaration\n  pub fn print_declaration(&self, stream: &mut String, indentation: i32) -> Result<()> {\n    // TODO(填体)\n    let _ = (stream, indentation);\n    Err(Error::Unimplemented)\n  }\n\n`;
  d += `  /// 原版 PrintDefinition\n  pub fn print_definition(&self, stream: &mut String, builder: &dyn Fn(&mut String), indentation: i32) -> Result<()> {\n    // TODO(填体)\n    let _ = (stream, builder, indentation);\n    Err(Error::Unimplemented)\n  }\n\n`;
  d += `  /// 原版 PrintBeginDefinition / PrintEndDefinition / PrintInlineDefinition\n  pub fn print_begin_definition(&self, stream: &mut String, indentation: i32) -> Result<()> {\n    // TODO(填体)\n    let _ = (stream, indentation);\n    Err(Error::Unimplemented)\n  }\n\n  pub fn print_end_definition(&self, stream: &mut String, indentation: i32) -> Result<()> {\n    // TODO(填体)\n    let _ = (stream, indentation);\n    Err(Error::Unimplemented)\n  }\n`;
  d += `}\n\n`;

  d += `/// 原版 cpp::File 1:1 (ostream* → String 缓冲; namespace 栈保序)\n#[derive(Debug, Default)]\npub struct File {\n  pub buf: String,\n  includes: BTreeSet<String>,\n  namespaces: Vec<String>,\n}\n\n`;
  d += `impl File {\n  /// 原版 BeginIncludeGuard\n  pub fn begin_include_guard(&mut self, name: &str) {\n    self.buf.push_str("#ifndef ");\n    self.buf.push_str(name);\n    self.buf.push_str("\\n#define ");\n    self.buf.push_str(name);\n    self.buf.push('\\n');\n  }\n\n`;
  d += `  /// 原版 EndIncludeGuard\n  pub fn end_include_guard(&mut self, name: &str) {\n    self.buf.push_str("#endif  // ");\n    self.buf.push_str(name);\n    self.buf.push('\\n');\n  }\n\n`;
  d += `  /// 原版 BeginNamespace (name 版)\n  pub fn begin_namespace(&mut self, name: &str) {\n    self.buf.push_str("namespace ");\n    self.buf.push_str(name);\n    self.buf.push_str(" {\\n");\n    self.namespaces.push(name.to_owned());\n  }\n\n`;
  d += `  /// 原版 BeginNamespace (name0, name1) 嵌套版\n  pub fn begin_namespace2(&mut self, name0: &str, name1: &str) {\n    self.begin_namespace(name0);\n    self.begin_namespace(name1);\n  }\n\n`;
  d += `  /// 原版 EndNamespace (name 版)\n  pub fn end_namespace(&mut self, name: &str) -> Result<()> {\n    let top = self.namespaces.pop().ok_or(Error::Unimplemented)?;\n    if top != name {\n      return Err(Error::Unimplemented);\n    }\n    self.buf.push_str("}  // namespace ");\n    self.buf.push_str(name);\n    self.buf.push('\\n');\n    Ok(())\n  }\n\n`;
  d += `  /// 原版 EndNamespace (name0, name1) 嵌套版\n  pub fn end_namespace2(&mut self, name0: &str, name1: &str) -> Result<()> {\n    self.end_namespace(name1)?;\n    self.end_namespace(name0)\n  }\n\n`;
  d += `  /// 原版 AddInclude\n  pub fn add_include(&mut self, include: String) {\n    self.includes.insert(include);\n  }\n\n`;
  d += `  /// 原版 operator<< → write\n  pub fn write(&mut self, s: &str) {\n    self.buf.push_str(s);\n  }\n\n`;
  d += `  /// include 区渲染 (原版在 PrintDeclaration 前集中输出)\n  pub fn flush_includes(&mut self) {\n    // TODO(填体): 输出位置与原版对齐\n    for inc in mem::take(&mut self.includes) {\n      self.buf.push_str("#include \\"");\n      self.buf.push_str(&inc);\n      self.buf.push_str("\\"\\n");\n    }\n  }\n`;
  d += `}\n\n`;

  d += `/// 原版 IncludeGuardScope (RAII → 显式 begin/end 对, 登记偏差)\n#[derive(Debug)]\npub struct IncludeGuardScope {\n  name: String,\n  active: bool,\n}\n\nimpl IncludeGuardScope {\n  #[must_use]\n  pub fn begin(file: &mut File, name: String) -> Self {\n    file.begin_include_guard(&name);\n    Self { name, active: true }\n  }\n\n  pub fn end(self, file: &mut File) {\n    if self.active {\n      file.end_include_guard(&self.name);\n    }\n  }\n}\n`;

  d += `\n` + sigBlock("src/torque/cpp-builder.cc", "签名清单");
  writeSrc("v8-torque", "cpp_builder.rs", d);
}

// ===========================================================================
// 主流程
// ===========================================================================
genCodeGenerator();
genBackendGenerators();
genTsaGenerator();
genAuxGenerators();
genCppBuilder();
console.log("== v8-torque 生成器组直出完毕: torque_code_generator + csa/cc/tsa + class_debug_reader + instance_type + cpp_builder ==");
