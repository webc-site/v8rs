#!/usr/bin/env bun
// v8/src/torque/ast-visitor.h + declaration-visitor.{h,cc} + implementation-visitor.{h,cc} → 直出
//   ast-visitor.h                → ast_visitor.rs            (CRTP → trait, 60 节点 visit_*)
//   declaration-visitor.h/.cc    → declaration_visitor.rs    (预声明 + 声明访问器)
//   implementation-visitor.h/.cc → implementation_visitor.rs (IR 构建核心)
// 继承树映射: AstVisitor<Derived> CRTP → Rust trait + visit_node 默认分派;
// Binding<T> 模板 → Binding + BindingKind 两变体; 上下文变量 (ValueBindingsManager/
// LabelBindingsManager/CurrentCallable/CurrentFileStreams/CurrentReturnValue) →
// ImplementationVisitor 字段 + 显式传参 (登记偏差)。
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
  const re = new RegExp(`(?:class|struct)\\s+(?:V8_EXPORT_PRIVATE\\s+|V8_NODISCARD\\s+|final\\s+)*${header}(?:\\s+final)?[^\\{;]*`);
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
    if (!line.endsWith(";") || line.includes("(")) continue;
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

const CAMEL1 = /([a-z0-9])([A-Z])/g, CAMEL2 = /([A-Z]+)([A-Z][a-z])/g;
const snake = (s) => s.replaceAll(CAMEL1, "$1_$2").replaceAll(CAMEL2, "$1_$2").toLowerCase();

// ===========================================================================
// ast-visitor.h → ast_visitor.rs
// ===========================================================================
function genAstVisitor() {
  const src = readV8("src/torque/ast.h");
  const noComment = src
    .replace(/\/\*[\s\S]*?\*\//g, "")
    .replace(/\/\/.*$/gm, "");
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
  const allKinds = [...exprKinds, ...typeExprKinds, ...stmtKinds, ...typeDeclKinds, ...declOnly, "Identifier", "TryHandler", "ClassBody"];

  let d = banner("src/torque/visitor.js", "src/torque/ast-visitor.h", "AST 访问器接口 (CRTP → trait, 60 节点)");
  d += `
use crate::ast::{Ast, AstNodeKind, NodeId};
use crate::error::Result;
use std::mem;

// C++ CRTP AstVisitor<Impl> → Rust trait: 每节点一个 visit_*, 由 visit_node 按
// kind 分派 (默认实现); 返回 Option<NodeId>: None = 删除节点 (对应原版返回 nullptr)。
// 实现方: TSAGenerator (原版唯一继承者, impl 见 tsa_generator.rs)。
`;

  d += `/// 原版 AstVisitor<Derived> — visit 方法族\npub trait AstVisitor {\n`;
  for (const k of allKinds) {
    d += `  /// 原版 Visit${k}\n  fn visit_${snake(k)}(&mut self, ast: &mut Ast, node: NodeId) -> Result<Option<NodeId>>;\n\n`;
  }
  d += `  /// 原版 Visit(node): kind 分派 (默认实现)\n  fn visit_node(&mut self, ast: &mut Ast, node: NodeId) -> Result<Option<NodeId>> {\n    let kind = ast.node(node).0.kind;\n    match kind {\n`;
  for (const k of allKinds) {
    d += `      AstNodeKind::${k} => self.visit_${snake(k)}(ast, node),\n`;
  }
  d += `    }\n  }\n\n`;
  d += `  /// 原版 Run: 遍历顶层声明, 原位替换\n  fn run(&mut self, ast: &mut Ast) -> Result<()> {\n    let decls = mem::take(&mut ast.declarations);\n    let mut out = Vec::with_capacity(decls.len());\n    for decl in decls {\n      if let Some(n) = self.visit_node(ast, decl)? {\n        out.push(n);\n      }\n    }\n    ast.declarations = out;\n    Ok(())\n  }\n`;
  d += `}\n`;
  writeSrc("v8-torque", "ast_visitor.rs", d);
}

// ===========================================================================
// declaration-visitor.{h,cc} → declaration_visitor.rs
// ===========================================================================
function genDeclarationVisitor() {
  const relH = "src/torque/declaration-visitor.h";
  let d = banner("src/torque/visitor.js", relH, "预声明 + 声明访问器 (静态方法族 → 关联函数)");
  d += `
use crate::ast::{Ast, NodeId};
use crate::declarable::DeclarableId;
use crate::error::{Error, Result};
use crate::global_context::GlobalContext;
use crate::source_positions::SourcePosition;
use crate::types::{MaybeSpecializationKey, Signature, TypeId};

// 原版 PredeclarationVisitor/DeclarationVisitor 全静态 (CurrentScope 上下文) →
// Rust 关联函数, 上下文以 (Ast, GlobalContext) 显式传参; 返回值从指针改为 id。
`;

  d += `/// 原版 GetOrCreateNamespace\npub fn get_or_create_namespace(cx: &mut GlobalContext, name: &str) -> Result<DeclarableId> {\n  // TODO(填体)\n  let _ = (cx, name);\n  Err(Error::Unimplemented)\n}\n\n`;

  d += `/// 原版 PredeclarationVisitor\npub struct PredeclarationVisitor;\n\nimpl PredeclarationVisitor {\n  /// 原版 Predeclare(Ast) — 入口\n  pub fn predeclare(cx: &mut GlobalContext, ast: &mut Ast) -> Result<()> {\n    // TODO(填体)\n    let _ = (cx, ast);\n    Err(Error::Unimplemented)\n  }\n\n`;
  d += `  /// 原版 ResolvePredeclarations\n  pub fn resolve_predeclarations(cx: &mut GlobalContext) -> Result<()> {\n    // TODO(填体)\n    let _ = cx;\n    Err(Error::Unimplemented)\n  }\n\n`;
  d += `  /// 原版 Predeclare(Declaration*) — 按 kind 分派\n  pub fn predeclare_decl(cx: &mut GlobalContext, ast: &mut Ast, decl: NodeId) -> Result<()> {\n    // TODO(填体): Namespace/Type/Struct/GenericType/GenericCallable 五分支\n    let _ = (cx, ast, decl);\n    Err(Error::Unimplemented)\n  }\n`;
  d += `}\n\n`;

  d += `/// 原版 DeclarationVisitor\npub struct DeclarationVisitor;\n\nimpl DeclarationVisitor {\n  /// 原版 Visit(Ast) — 入口\n  pub fn visit_ast(cx: &mut GlobalContext, ast: &mut Ast) -> Result<()> {\n    // TODO(填体)\n    let _ = (cx, ast);\n    Err(Error::Unimplemented)\n  }\n\n`;
  d += `  /// 原版 Visit(Declaration*) — 按 kind 分派\n  pub fn visit_decl(cx: &mut GlobalContext, ast: &mut Ast, decl: NodeId) -> Result<Option<TypeId>> {\n    // TODO(填体): 全部声明种类分支\n    let _ = (cx, ast, decl);\n    Err(Error::Unimplemented)\n  }\n\n`;
  d += `  /// 原版 CreateBuiltin\n  pub fn create_builtin(cx: &mut GlobalContext, decl: NodeId, external_name: &str, readable_name: &str, signature: Signature, use_counter_name: Option<String>, body: Option<NodeId>) -> Result<DeclarableId> {\n    // TODO(填体)\n    let _ = (cx, decl, external_name, readable_name, signature, use_counter_name, body);\n    Err(Error::Unimplemented)\n  }\n\n`;
  d += `  /// 原版 MakeSpecializedSignature\n  pub fn make_specialized_signature(cx: &mut GlobalContext, key: &MaybeSpecializationKey) -> Result<Signature> {\n    // TODO(填体)\n    let _ = (cx, key);\n    Err(Error::Unimplemented)\n  }\n\n`;
  d += `  /// 原版 SpecializeImplicit\n  pub fn specialize_implicit(cx: &mut GlobalContext, key: &MaybeSpecializationKey) -> Result<DeclarableId> {\n    // TODO(填体)\n    let _ = (cx, key);\n    Err(Error::Unimplemented)\n  }\n\n`;
  d += `  /// 原版 Specialize\n  pub fn specialize(cx: &mut GlobalContext, key: &MaybeSpecializationKey, declaration: NodeId, explicit_specialization: Option<NodeId>, body: Option<NodeId>, position: SourcePosition) -> Result<DeclarableId> {\n    // TODO(填体)\n    let _ = (cx, key, declaration, explicit_specialization, body, position);\n    Err(Error::Unimplemented)\n  }\n\n`;
  d += `  /// 原版 DeclareSpecializedTypes (私有)\n  pub fn declare_specialized_types(cx: &mut GlobalContext, key: &MaybeSpecializationKey) -> Result<()> {\n    // TODO(填体)\n    let _ = (cx, key);\n    Err(Error::Unimplemented)\n  }\n`;
  d += `}\n\n`;
  d += sigBlock("src/torque/declaration-visitor.h", "接口签名清单") + "\n";
  d += sigBlock("src/torque/declaration-visitor.cc", "签名清单");
  writeSrc("v8-torque", "declaration_visitor.rs", d);
}

// ===========================================================================
// implementation-visitor.{h,cc} → implementation_visitor.rs
// ===========================================================================
function genImplementationVisitor() {
  const relH = "src/torque/implementation-visitor.h";
  const s = noCmt(readV8(relH));
  let d = banner("src/torque/visitor.js", relH, "实现访问器 (CFG 构建核心) + LocationReference 五态");
  d += `
use crate::ast::NodeId;
use crate::cfg::CfgAssembler;
use crate::constants::FieldSynchronization;
use crate::declarable::{DeclarableId, OutputType};
use crate::error::{Error, Result};
use crate::global_context::GlobalContext;
use crate::instructions::BlockId;
use crate::source_positions::{SourceId, SourcePosition};
use crate::type_inference::TypeArgumentInference;
use crate::type_oracle::TypeOracle;
use crate::types::{Signature, TypeId, TypeVector, VisitResult};
use crate::utils::{BottomOffset, replace_file_contents_if_different};
use std::collections::{BTreeMap, BTreeSet};

// C++ Binding<T> 模板 (LocalValue/LocalLabel 两实例) → Binding + BindingKind 两变体;
// BindingsManager 按名字查绑定; 上下文变量 (ValueBindingsManager/LabelBindingsManager/
// CurrentCallable/CurrentFileStreams/CurrentReturnValue) → ImplementationVisitor 字段
// + 显式传参 (登记偏差, 填体时定形)。
`;

  // LocationReference 五态
  d += `/// 原版 LocationReference — l-value 五态和 (C++ 多字段互斥 → enum)\n#[derive(Debug, Clone)]\npub enum LocationReference {\n  /// VariableAccess: 可赋值栈区间 (+ 绑定回引)\n  Variable { variable: VisitResult, binding: Option<BindingId> },\n  /// Temporary: 不可赋值临时值 ({description} 供错误消息)\n  Temporary { temporary: VisitResult, temporary_description: String },\n  /// HeapReference: 堆引用 (tagged 值 + 内指针偏移) + 同步语义\n  HeapReference { heap_reference: VisitResult, synchronization: FieldSynchronization },\n  /// HeapSlice: 堆数组引用 (tagged 值 + 偏移 + 元素数)\n  HeapSlice { heap_slice: VisitResult },\n  /// ArrayAccess/FieldAccess: eval/assign 函数调用对\n  Call { eval_function: String, assign_function: String, call_arguments: Vec<VisitResult> },\n}\n\n`;
  d += `impl LocationReference {\n  /// 原版 VariableAccess\n  #[must_use]\n  pub fn variable_access(variable: VisitResult, binding: Option<BindingId>) -> Self {\n    Self::Variable { variable, binding }\n  }\n\n`;
  d += `  /// 原版 Temporary\n  #[must_use]\n  pub fn temporary(temporary: VisitResult, description: String) -> Self {\n    Self::Temporary { temporary, temporary_description: description }\n  }\n\n`;
  d += `  /// 原版 HeapReference\n  #[must_use]\n  pub fn heap_reference(heap_reference: VisitResult, synchronization: FieldSynchronization) -> Self {\n    Self::HeapReference { heap_reference, synchronization }\n  }\n\n`;
  d += `  /// 原版 HeapSlice\n  #[must_use]\n  pub fn heap_slice(heap_slice: VisitResult) -> Self {\n    Self::HeapSlice { heap_slice }\n  }\n\n`;
  d += `  /// 原版 ArrayAccess\n  #[must_use]\n  pub fn array_access(base: VisitResult, offset: VisitResult) -> Self {\n    Self::Call {\n      eval_function: "[]".to_owned(),\n      assign_function: "[]=".to_owned(),\n      call_arguments: vec![base, offset],\n    }\n  }\n\n`;
  d += `  /// 原版 FieldAccess\n  #[must_use]\n  pub fn field_access(object: VisitResult, fieldname: &str) -> Self {\n    Self::Call {\n      eval_function: format!(".{fieldname}"),\n      assign_function: format!(".{fieldname}="),\n      call_arguments: vec![object],\n    }\n  }\n\n`;
  d += `  /// 原版 IsVariable\n  #[must_use]\n  pub fn is_variable(&self) -> bool { matches!(self, Self::Variable { .. }) }\n\n`;
  d += `  /// 原版 IsTemporary\n  #[must_use]\n  pub fn is_temporary(&self) -> bool { matches!(self, Self::Temporary { .. }) }\n\n`;
  d += `  /// 原版 IsHeapReference\n  #[must_use]\n  pub fn is_heap_reference(&self) -> bool { matches!(self, Self::HeapReference { .. }) }\n\n`;
  d += `  /// 原版 IsHeapSlice\n  #[must_use]\n  pub fn is_heap_slice(&self) -> bool { matches!(self, Self::HeapSlice { .. }) }\n\n`;
  d += `  /// 原版 IsCall\n  #[must_use]\n  pub fn is_call(&self) -> bool { matches!(self, Self::Call { .. }) }\n`;
  d += `}\n\n`;

  // Binding 体系
  d += `/// 绑定 arena 索引\npub type BindingId = u32;\n\n`;
  d += `/// 原版 Binding<LocalValue>/<LocalLabel> → BindingKind 两变体\n#[derive(Debug, Clone)]\npub enum BindingValue {\n  Value(LocalValue),\n  Label(LocalLabel),\n}\n\n`;
  d += `/// 原版 Binding<T> 1:1 (IS-A T → value 字段承载; manager_/previous_binding_ 由\n/// BindingsManager arena + 影子栈承担, 登记偏差)\n#[derive(Debug, Clone)]\npub struct Binding {\n  pub name: String,\n  pub value: BindingValue,\n  pub declaration_position: SourcePosition,\n  pub used: bool,\n  pub written: bool,\n  pub unique_index: u64,\n}\n\n`;
  d += `/// 原版 BindingsManager<T> 1:1 (value/label 两实例由 BindingKind 区分)\n#[derive(Debug, Clone, Default)]\npub struct BindingsManager {\n  arena: Vec<Binding>,\n  bindings: BTreeMap<String, BindingId>,\n}\n\n`;
  d += `impl BindingsManager {\n  /// 原版 TryLookup\n  #[must_use]\n  pub fn try_lookup(&self, name: &str) -> Option<BindingId> {\n    self.bindings.get(name).copied()\n  }\n\n`;
  d += `  /// 原版 Binding 构造登记 (Binding::TryCreate)\n  pub fn declare_binding(&mut self, name: String, binding: Binding) -> BindingId {\n    let id = (self.arena.len()) as BindingId;\n    self.arena.push(binding);\n    self.bindings.insert(name, id);\n    id\n  }\n\n`;
  d += `  #[must_use]\n  pub fn binding(&self, id: BindingId) -> &Binding {\n    &self.arena[id as usize]\n  }\n\n`;
  d += `  #[must_use]\n  pub fn binding_mut(&mut self, id: BindingId) -> &mut Binding {\n    &mut self.arena[id as usize]\n  }\n`;
  d += `}\n\n`;

  // LocalValue / LocalLabel / BlockBindings / Arguments (字段按原版手写)
  d += `/// 原版 LocalValue 1:1 (值 / 惰性 / 不可访问三态)\n#[derive(Debug, Clone, Default)]\npub struct LocalValue {\n  pub value: Option<LocationReference>,\n  /// 惰性求值 (原版 std::function<LocationReference()>)\n  pub lazy: Option<fn() -> LocationReference>,\n  pub inaccessible_explanation: String,\n}\n\n`;
  d += `impl LocalValue {\n  /// 原版 IsAccessibleNonLazy\n  #[must_use]\n  pub fn is_accessible_non_lazy(&self) -> bool { self.value.is_some() }\n\n`;
  d += `  /// 原版 GetLocationReference\n  pub fn get_location_reference(&self, binding: BindingId) -> Result<LocationReference> {\n    // TODO(填体): 三态解析 + 不可访问报错\n    let _ = binding;\n    Err(Error::Unimplemented)\n  }\n}\n\n`;
  d += `/// 原版 LocalLabel 1:1\n#[derive(Debug, Clone, Default)]\npub struct LocalLabel {\n  pub block: Option<BlockId>,\n  pub parameter_types: TypeVector,\n}\n\n`;
  d += `/// 原版 BlockBindings<LocalValue> 1:1 (类字段初始化; 管理器引用由调用方持有)\n#[derive(Debug, Clone, Default)]\npub struct BlockBindings {\n  pub bindings: Vec<BindingId>,\n}\n\n`;
  d += `/// 原版 Arguments 1:1\n#[derive(Debug, Clone, Default)]\npub struct Arguments {\n  pub parameters: Vec<VisitResult>,\n  pub labels: Vec<BindingId>,\n}\n\n`;
  d += `/// 原版 IsCompatibleSignature (重载兼容判定)\n#[must_use]\npub fn is_compatible_signature(sig: &Signature, types: &TypeVector, label_count: usize) -> bool {\n  // TODO(填体)\n  let _ = (sig, types, label_count);\n  false\n}\n\n`;
  d += `/// 原版 next_unique_binding_index (全局计数 → 计数器参数/驱动持有, 登记偏差)\npub type NextUniqueBindingIndex = u64;\n\n`;

  // ImplementationVisitor 结构体 (私有字段 1:1)
  d += `/// 原版 ImplementationVisitor 私有字段 1:1 (null_stream_ 删除 — Rust 无流占位)\n#[derive(Debug, Default)]\npub struct ImplementationVisitor {\n  pub assembler: Option<CfgAssembler>,\n  pub is_dry_run: bool,\n  /// 位域表达式来源表 (表达式 → 所属位域结构体标识符)\n  pub bitfield_expressions: BTreeMap<NodeId, NodeId>,\n  /// InlineMacro 内联中的宏集合 (警告用)\n  pub inlining_macros: BTreeSet<DeclarableId>,\n  /// debug 宏输出文件缓冲 (原版 stringstream)\n  pub debug_macros_cc: String,\n  pub debug_macros_h: String,\n  pub output_type: OutputType,\n  /// 原版 ValueBindingsManager / LabelBindingsManager 上下文\n  pub value_bindings: BindingsManager,\n  pub label_bindings: BindingsManager,\n  /// 原版 CurrentCallable / CurrentReturnValue 上下文\n  pub current_callable: Option<DeclarableId>,\n  pub current_return_value: Option<VisitResult>,\n}\n\n`;

  d += `impl ImplementationVisitor {\n  /// 原版 SetDryRun\n  pub fn set_dry_run(&mut self, is_dry_run: bool) {\n    self.is_dry_run = is_dry_run;\n  }\n\n`;
  const methods = [
    ["generate_builtin_definitions_and_interface_descriptors", "GenerateBuiltinDefinitionsAndInterfaceDescriptors(output_directory)", "pub fn generate_builtin_definitions_and_interface_descriptors(&mut self, cx: &mut GlobalContext, output_directory: &str) -> Result<()>"],
    ["generate_bit_fields", "GenerateBitFields(output_directory)", "pub fn generate_bit_fields(&mut self, cx: &mut GlobalContext, output_directory: &str) -> Result<()>"],
    ["generate_class_definitions", "GenerateClassDefinitions(output_directory)", "pub fn generate_class_definitions(&mut self, cx: &mut GlobalContext, output_directory: &str) -> Result<()>"],
    ["generate_instance_types", "GenerateInstanceTypes(output_directory)", "pub fn generate_instance_types(&mut self, cx: &mut GlobalContext, output_directory: &str) -> Result<()>"],
    ["generate_enum_verifiers", "GenerateEnumVerifiers(output_directory)", "pub fn generate_enum_verifiers(&mut self, cx: &mut GlobalContext, output_directory: &str) -> Result<()>"],
    ["generate_class_debug_readers", "GenerateClassDebugReaders(output_directory)", "pub fn generate_class_debug_readers(&mut self, cx: &mut GlobalContext, output_directory: &str) -> Result<()>"],
    ["generate_exported_macros_assembler", "GenerateExportedMacrosAssembler(output_directory)", "pub fn generate_exported_macros_assembler(&mut self, cx: &mut GlobalContext, output_directory: &str) -> Result<()>"],
    ["generate_csa_types", "GenerateCSATypes(output_directory)", "pub fn generate_csa_types(&mut self, cx: &mut GlobalContext, output_directory: &str) -> Result<()>"],
    ["visit_expression", "Visit(Expression*) → VisitResult", "pub fn visit_expression(&mut self, cx: &mut GlobalContext, expr: NodeId) -> Result<VisitResult>"],
    ["visit_statement", "Visit(Statement*) → const Type*", "pub fn visit_statement(&mut self, cx: &mut GlobalContext, stmt: NodeId) -> Result<TypeId>"],
    ["get_location_reference", "GetLocationReference(Expression*)", "pub fn get_location_reference(&mut self, cx: &mut GlobalContext, location: NodeId) -> Result<LocationReference>"],
    ["generate_call", "GenerateCall(callable, this, parameters, specialization_types, tail_call)", "pub fn generate_call(&mut self, cx: &mut GlobalContext, callable: DeclarableId, this_parameter: Option<LocationReference>, parameters: Arguments, specialization_types: &TypeVector, tail_call: bool) -> Result<VisitResult>"],
    ["inline_macro", "InlineMacro(macro, this_reference, arguments, label_blocks)", "pub fn inline_macro(&mut self, cx: &mut GlobalContext, macro_: DeclarableId, this_reference: Option<LocationReference>, arguments: &[VisitResult], label_blocks: &[BlockId]) -> Result<VisitResult>"],
    ["visit_all_declarables", "VisitAllDeclarables()", "pub fn visit_all_declarables(&mut self, cx: &mut GlobalContext) -> Result<()>"],
    ["visit_declarable", "Visit(Declarable*, file)", "pub fn visit_declarable(&mut self, cx: &mut GlobalContext, declarable: DeclarableId, file: Option<SourceId>) -> Result<()>"],
    ["generate_implementation", "GenerateImplementation(dir)", "pub fn generate_implementation(&mut self, cx: &mut GlobalContext, dir: &str) -> Result<()>"],
    ["get_common_type", "GetCommonType(left, right)", "pub fn get_common_type(&self, cx: &TypeOracle, left: TypeId, right: TypeId) -> Result<TypeId>"],
    ["generate_copy", "GenerateCopy(to_copy)", "pub fn generate_copy(&mut self, to_copy: &VisitResult) -> Result<VisitResult>"],
    ["generate_implicit_convert", "GenerateImplicitConvert(destination_type, source)", "pub fn generate_implicit_convert(&mut self, cx: &TypeOracle, destination_type: TypeId, source: VisitResult) -> Result<VisitResult>"],
    ["generate_branch", "GenerateBranch(condition, true_block, false_block)", "pub fn generate_branch(&mut self, condition: &VisitResult, true_block: BlockId, false_block: BlockId) -> Result<()>"],
    ["generate_bool_constant", "GenerateBoolConstant(constant)", "pub fn generate_bool_constant(&mut self, constant: bool) -> Result<VisitResult>"],
    ["infer_specialization_types", "InferSpecializationTypes(generic, explicit_specialization_types, explicit_arguments)", "pub fn infer_specialization_types(&self, cx: &TypeOracle, generic: DeclarableId, explicit_specialization_types: &TypeVector, explicit_arguments: &TypeVector) -> Result<TypeArgumentInference>"],
  ];
  const paramsOf = (sig) => {
    const m = sig.match(/\(([\s\S]*)\)\s*->/);
    if (!m) return [];
    return m[1].split(",")
      .map((p) => p.trim().split(":")[0].trim())
      .filter((p) => p && p !== "self" && !p.startsWith("&") && p !== "mut");
  };
  for (const [m, orig, sig] of methods) {
    const ps = paramsOf(sig);
    const bind = ps.length ? (ps.length === 1 ? `    let _ = ${ps[0]};\n` : `    let _ = (${ps.join(", ")});\n`) : "";
    d += `  /// 原版 ${orig}\n  ${sig} {\n    // TODO(填体)\n${bind}    Err(Error::Unimplemented)\n  }\n\n`;
  }
  d += `  /// 原版 WriteFile (dry-run 时跳过)\n  pub fn write_file(&self, file: &str, content: &str) -> Result<()> {\n    if self.is_dry_run {\n      return Ok(());\n    }\n    replace_file_contents_if_different(file, content)?;
    Ok(())\n  }\n`;
  d += `}\n\n`;

  d += `/// 原版 StackScope — 栈高度作用域守卫 (Yield 存活槽位); Rust 借用模型下\n/// 改显式 begin/yield/close 三步 (登记偏差)。\n#[derive(Debug)]\npub struct StackScope {\n  pub base: BottomOffset,\n  pub closed: bool,\n}\n\nimpl StackScope {\n  /// 原版构造 (记录当前栈高)\n  #[must_use]\n  pub fn begin(assembler: &CfgAssembler) -> Self {\n    Self {\n      base: assembler.current_stack().above_top(),\n      closed: false,\n    }\n  }\n\n`;
  d += `  /// 原版 Yield (v 存活, 其余删除)\n  pub fn yield_result(&mut self, assembler: &mut CfgAssembler, result: VisitResult) -> Result<VisitResult> {\n    // TODO(填体)\n    let _ = (assembler, result);\n    Err(Error::Unimplemented)\n  }\n\n`;
  d += `  /// 原版 Close\n  pub fn close(&mut self, assembler: &mut CfgAssembler) -> Result<()> {\n    // TODO(填体)\n    let _ = assembler;\n    self.closed = true;\n    Ok(())\n  }\n`;
  d += `}\n\n`;

  d += `/// 原版 BreakContinueActivator — break/continue 标号绑定对\n#[derive(Debug)]\npub struct BreakContinueActivator {\n  pub break_binding: BindingId,\n  pub continue_binding: BindingId,\n}\n\n`;
  d += `/// 原版 BindingsManagersScope — 绑定管理器作用域 (隐藏既有绑定)\n#[derive(Debug, Default)]\npub struct BindingsManagersScope;\n\n`;
  d += `/// 原版 MacroInliningScope — 内联宏登记作用域\n#[derive(Debug, Default)]\npub struct MacroInliningScope;\n\n`;
  d += `/// 原版 InitializerResults 1:1\n#[derive(Debug, Clone, Default)]\npub struct InitializerResults {\n  pub names: Vec<NodeId>,\n  pub field_value_map: BTreeMap<String, VisitResult>,\n}\n\n`;
  d += `/// 原版 LayoutForInitialization 1:1\n#[derive(Debug, Clone, Default)]\npub struct LayoutForInitialization {\n  pub array_lengths: BTreeMap<String, VisitResult>,\n  pub offsets: BTreeMap<String, VisitResult>,\n  pub size: VisitResult,\n}\n\n`;
  d += `/// 原版 ReportAllUnusedMacros\npub fn report_all_unused_macros(cx: &GlobalContext) -> Result<()> {\n  // TODO(填体)\n  let _ = cx;\n  Err(Error::Unimplemented)\n}\n`;

  d += `\n` + sigBlock("src/torque/implementation-visitor.h", "接口签名清单") + "\n";
  d += sigBlock("src/torque/implementation-visitor.cc", "全量签名清单 (填体正交任务源)");
  writeSrc("v8-torque", "implementation_visitor.rs", d);
}

// ===========================================================================
// 主流程
// ===========================================================================
genAstVisitor();
genDeclarationVisitor();
genImplementationVisitor();
console.log("== v8-torque 访问器组直出完毕: ast_visitor.rs + declaration_visitor.rs + implementation_visitor.rs ==");
