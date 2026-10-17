#!/usr/bin/env bun
// v8/src/torque/declarable.{h,cc} + declarations.{h,cc} + parameter-difference.h
//   + type-inference.{h,cc} → 直出
//   declarable.h/.cc          → declarable.rs          (Declarable 体系全量 12 kind)
//   declarations.h/.cc        → declarations.rs        (声明表常量 + 查询/创建接口)
//   parameter-difference.h    → parameter_difference.rs
//   type-inference.{h,cc}     → type_inference.rs
// 继承树映射: Declarable(kind) → DeclarableData 十二变体; Scope 内 declarations_ 多态
// 指针 → DeclarableId arena (存储在 global_context.rs); C++ 模板 GenericDeclarable<S,D>
// 去模板: GenericCallable/GenericType 各自持表; CurrentScope 上下文 → 显式传参。
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
  const re = new RegExp(`(?:class|struct)\\s+(?:V8_EXPORT_PRIVATE\\s+|final\\s+)*${header}(?:\\s+final)?[^\\{;]*`);
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

function memberSigs(body) {
  const out = [];
  for (const [i, raw] of body.split("\n").entries()) {
    const line = raw.trim();
    if (!line.endsWith(";") || !line.includes("(")) continue;
    if (/^(DEFINE_|DECLARE_|friend|using|template\b)/.test(line) || line.startsWith("//")) continue;
    out.push({ sig: line.replace(/\s+/g, " "), line: i + 1 });
  }
  return out;
}

function sigBlock(f, note = "签名清单 (填体正交任务源)") {
  const rel = f.startsWith("src/") ? f : `src/torque/${f}`;
  const s = readV8(rel);
  let dd = `// ── 原版 ${rel} ──\n`;
  const sigs = topSigs(s);
  if (sigs.length === 0 && f.endsWith(".cc") && s.length > 400) {
    console.error(`[torque] ${f} 非空但抽到 0 条签名 — noCmt/topSigs 异常, 防静默检查`);
  }
  for (const x of sigs) dd += `// L${x.line}: ${x.sig}\n`;
  return dd;
}

// ===========================================================================
// declarable.{h,cc} → declarable.rs
// ===========================================================================
function genDeclarable() {
  const relH = "src/torque/declarable.h";
  const dh = noCmt(readV8(relH));

  let d = banner("src/torque/declarable.js", relH, "Declarable 体系全量 (12 kind + 载荷 + Scope 存储)");
  d += `
use crate::ast::{GenericParameters, NodeId};
use crate::error::{Error, Result};
use crate::source_positions::SourcePosition;
use crate::types::{Signature, TypeId, TypeVector, VisitResult};
use std::collections::BTreeMap;

// 继承树映射: Declarable(kind) → DeclarableData 十二变体; Scope 存储抽为 ScopeData
// (Namespace 与 Callable 共有); C++ 模板 GenericDeclarable<S,D> 去模板, 两个具体类
// 各自持特化表; 特化键 TypeVector 用 BTreeMap 保确定性序。
`;

  // Declarable::Kind 全量 (无作用域 enum, 专用正则)
  {
    const m = dh.slice(dh.indexOf("class Declarable")).match(/enum Kind\s*\{([^}]*)\}/);
    if (!m) throw new Error("Declarable::Kind 未提取");
    const vs = [...m[1].matchAll(/\bk(\w+)/g)].map((x) => x[1]);
    d += `/// Declarable::Kind 1:1 (${vs.length} 变体)\n#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]\n#[repr(u8)]\npub enum DeclarableKind {\n`;
    for (const v of vs) d += `  ${v.replace(/^k/, "")},\n`;
    d += `}\n\n`;
  }

  d += `/// 声明 arena 索引 (对应 C++ Declarable*)\npub type DeclarableId = u32;\n\n`;

  d += `/// 原版 QualifiedName 1:1\n#[derive(Debug, Clone, Default)]\npub struct QualifiedName {\n  pub namespace_qualification: Vec<String>,\n  pub name: String,\n}\n\n`;
  d += `impl QualifiedName {\n  /// 原版 explicit QualifiedName(name) — 无限定名\n  #[must_use]\n  pub fn plain(name: String) -> Self {\n    Self { namespace_qualification: Vec::new(), name }\n  }\n\n`;
  d += `  /// 原版 Parse\n  #[must_use]\n  pub fn parse(qualified_name: &str) -> Self {\n    // TODO(填体): 按 :: 切分\n    let _ = qualified_name;\n    Self::default()\n  }\n\n`;
  d += `  #[must_use]\n  pub fn has_namespace_qualification(&self) -> bool {\n    !self.namespace_qualification.is_empty()\n  }\n\n`;
  d += `  /// 原版 DropFirstNamespaceQualification\n  #[must_use]\n  pub fn drop_first_namespace_qualification(&self) -> Self {\n    Self {\n      namespace_qualification: self.namespace_qualification.iter().skip(1).cloned().collect(),\n      name: self.name.clone(),\n    }\n  }\n`;
  d += `}\n\n`;

  d += `/// 原版 SpecializationRequester 1:1 (特化成因, 供错误报告)\n#[derive(Debug, Clone, Default)]\npub struct SpecializationRequester {\n  pub position: SourcePosition,\n  pub scope: Option<DeclarableId>,\n  pub name: String,\n}\n\n`;
  d += `impl SpecializationRequester {\n  /// 原版 None()\n  #[must_use]\n  pub fn none() -> Self {\n    Self { position: SourcePosition::invalid(), scope: None, name: String::new() }\n  }\n\n`;
  d += `  #[must_use]\n  pub fn is_none(&self) -> bool {\n    self.position == SourcePosition::invalid() && self.scope.is_none() && self.name.is_empty()\n  }\n`;
  d += `}\n\n`;

  // 类内枚举
  d += `/// Builtin::Kind 1:1\n#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]\n#[repr(u8)]\npub enum BuiltinKind {\n  #[default]\n  Stub,\n  FixedArgsJavaScript,\n  VarArgsJavaScript,\n}\n\n`;
  d += `/// Builtin::Flag 位标志 1:1 (base::Flags → u32)\n#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]\n#[repr(u32)]\npub enum BuiltinFlag { None = 0, CustomInterfaceDescriptor = 1 << 0, SupportsTsa = 1 << 1 }\n\n`;
  d += `/// 原版 using Builtin::Flags = base::Flags<Builtin::Flag>\npub type BuiltinFlags = u32;\n\n`;
  d += `/// 原版 OutputType 1:1\n#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]\n#[repr(u8)]\npub enum OutputType {\n  #[default]\n  Csa,\n  CcDebug,\n}\n\n`;

  // 载荷结构体 (继承链公共部分抽出)
  d += `/// 原版 Scope 私有存储 (declarations_ + requester_), Namespace/Callable 共有\n#[derive(Debug, Clone, Default)]\npub struct ScopeData {\n  pub declarations: BTreeMap<String, Vec<DeclarableId>>,\n  pub requester: SpecializationRequester,\n}\n\n`;
  d += `/// 原版 Callable 私有字段 (external_name/readable_name/signature/returns/body)\n#[derive(Debug, Clone, Default)]\npub struct CallableCommon {\n  pub external_name: String,\n  pub readable_name: String,\n  pub signature: Signature,\n  pub returns: usize,\n  pub body: Option<NodeId>,\n}\n\n`;
  d += `/// 原版 Macro 私有字段 (used/supports_tsa)\n#[derive(Debug, Clone, Default)]\npub struct MacroCommon {\n  pub callable: CallableCommon,\n  pub used: bool,\n  pub supports_tsa: bool,\n}\n\n`;
  d += `/// 原版 Value 私有字段 (type/name/value)\n#[derive(Debug, Clone, Default)]\npub struct ValueCommon {\n  pub type_: Option<TypeId>,\n  pub name: NodeId,\n  pub value: Option<VisitResult>,\n}\n\n`;

  d += `/// 按变体载荷 (C++ 派生类私有字段 1:1)\n#[derive(Debug, Clone)]\npub enum DeclarableData {\n  Namespace(NamespaceData),\n  TorqueMacro(TorqueMacroData),\n  ExternMacro(ExternMacroData),\n  Method(MethodData),\n  Builtin(BuiltinData),\n  RuntimeFunction(RuntimeFunctionData),\n  Intrinsic(IntrinsicData),\n  GenericCallable(GenericCallableData),\n  GenericType(GenericTypeData),\n  TypeAlias(TypeAliasData),\n  ExternConstant(ExternConstantData),\n  NamespaceConstant(NamespaceConstantData),\n}\n\n`;
  d += `/// 原版 Namespace 1:1\n#[derive(Debug, Clone, Default)]\npub struct NamespaceData {\n  pub name: String,\n}\n\n`;
  d += `/// 原版 TorqueMacro 1:1\n#[derive(Debug, Clone, Default)]\npub struct TorqueMacroData {\n  pub macro_: MacroCommon,\n  pub exported_to_csa: bool,\n}\n\n`;
  d += `/// 原版 ExternMacro 1:1\n#[derive(Debug, Clone, Default)]\npub struct ExternMacroData {\n  pub macro_: MacroCommon,\n  pub external_assembler_name: String,\n}\n\n`;
  d += `/// 原版 Method 1:1 (aggregate_type 指针 → TypeId)\n#[derive(Debug, Clone, Default)]\npub struct MethodData {\n  pub macro_: MacroCommon,\n  pub aggregate_type: Option<TypeId>,\n}\n\n`;
  d += `/// 原版 Builtin 1:1\n#[derive(Debug, Clone, Default)]\npub struct BuiltinData {\n  pub callable: CallableCommon,\n  pub kind: BuiltinKind,\n  pub flags: BuiltinFlags,\n  pub use_counter_name: Option<String>,\n}\n\n`;
  d += `/// 原版 RuntimeFunction 1:1\n#[derive(Debug, Clone, Default)]\npub struct RuntimeFunctionData {\n  pub callable: CallableCommon,\n}\n\n`;
  d += `/// 原版 Intrinsic 1:1\n#[derive(Debug, Clone, Default)]\npub struct IntrinsicData {\n  pub callable: CallableCommon,\n}\n\n`;
  d += `/// 原版 GenericCallableDeclarable (模板 GenericDeclarable<Callable*, ...> 去模板)\n#[derive(Debug, Clone, Default)]\npub struct GenericCallableData {\n  pub name: String,\n  pub declaration: NodeId,\n  pub specializations: BTreeMap<TypeVector, DeclarableId>,\n  pub constraints: Option<Vec<TypeConstraint>>,\n}\n\n`;
  d += `/// 原版 GenericTypeDeclarable (模板 GenericDeclarable<const Type*, ...> 去模板)\n#[derive(Debug, Clone, Default)]\npub struct GenericTypeData {\n  pub name: String,\n  pub declaration: NodeId,\n  pub specializations: BTreeMap<TypeVector, TypeId>,\n  pub constraints: Option<Vec<TypeConstraint>>,\n}\n\n`;
  d += `/// 原版 TypeAlias 1:1\n#[derive(Debug, Clone, Default)]\npub struct TypeAliasData {\n  pub being_resolved: bool,\n  pub delayed: Option<NodeId>,\n  pub type_: Option<TypeId>,\n  pub redeclaration: bool,\n  pub declaration_position: SourcePosition,\n}\n\n`;
  d += `/// 原版 ExternConstant 1:1\n#[derive(Debug, Clone, Default)]\npub struct ExternConstantData {\n  pub value: ValueCommon,\n}\n\n`;
  d += `/// 原版 NamespaceConstant 1:1\n#[derive(Debug, Clone, Default)]\npub struct NamespaceConstantData {\n  pub value: ValueCommon,\n  pub external_name: String,\n  pub body: NodeId,\n}\n\n`;

  d += `/// Declarable 公共头 (kind + 作用域链 + 源位置 + 数据载荷) 1:1\n#[derive(Debug, Clone)]\npub struct Declarable {\n  pub kind: DeclarableKind,\n  /// 原版 parent_scope_ (构造时取 CurrentScope::Get() → 由创建方显式传入)\n  pub parent_scope: Option<DeclarableId>,\n  pub position: SourcePosition,\n  pub identifier_position: SourcePosition,\n  pub is_user_defined: bool,\n  /// IsScope() (Namespace 或 Callable) 为真时持有 Scope 存储\n  pub scope: Option<ScopeData>,\n  pub data: DeclarableData,\n}\n\n`;
  d += `impl Default for Declarable {\n  fn default() -> Self {\n    Self {\n      kind: DeclarableKind::Namespace,\n      parent_scope: None,\n      position: SourcePosition::invalid(),\n      identifier_position: SourcePosition::invalid(),\n      is_user_defined: true,\n      scope: None,\n      data: DeclarableData::Namespace(NamespaceData::default()),\n    }\n  }\n}\n\n`;
  d += `impl Declarable {\n`;
  d += `  // 原 IsXxx 谓词 1:1 (全部由 kind 判别)\n  #[must_use]\n  pub fn is_namespace(&self) -> bool { self.kind == DeclarableKind::Namespace }\n\n`;
  d += `  /// 原版 IsMacro: TorqueMacro 或 ExternMacro\n  #[must_use]\n  pub fn is_macro(&self) -> bool { self.is_torque_macro() || self.is_extern_macro() }\n\n`;
  d += `  /// 原版 IsTorqueMacro: kTorqueMacro 或 kMethod\n  #[must_use]\n  pub fn is_torque_macro(&self) -> bool {\n    matches!(self.kind, DeclarableKind::TorqueMacro | DeclarableKind::Method)\n  }\n\n`;
  d += `  #[must_use]\n  pub fn is_method(&self) -> bool { self.kind == DeclarableKind::Method }\n\n`;
  d += `  #[must_use]\n  pub fn is_extern_macro(&self) -> bool { self.kind == DeclarableKind::ExternMacro }\n\n`;
  d += `  #[must_use]\n  pub fn is_intrinsic(&self) -> bool { self.kind == DeclarableKind::Intrinsic }\n\n`;
  d += `  #[must_use]\n  pub fn is_builtin(&self) -> bool { self.kind == DeclarableKind::Builtin }\n\n`;
  d += `  #[must_use]\n  pub fn is_runtime_function(&self) -> bool { self.kind == DeclarableKind::RuntimeFunction }\n\n`;
  d += `  #[must_use]\n  pub fn is_generic_callable(&self) -> bool { self.kind == DeclarableKind::GenericCallable }\n\n`;
  d += `  #[must_use]\n  pub fn is_generic_type(&self) -> bool { self.kind == DeclarableKind::GenericType }\n\n`;
  d += `  #[must_use]\n  pub fn is_type_alias(&self) -> bool { self.kind == DeclarableKind::TypeAlias }\n\n`;
  d += `  #[must_use]\n  pub fn is_extern_constant(&self) -> bool { self.kind == DeclarableKind::ExternConstant }\n\n`;
  d += `  #[must_use]\n  pub fn is_namespace_constant(&self) -> bool { self.kind == DeclarableKind::NamespaceConstant }\n\n`;
  d += `  /// 原版 IsValue: ExternConstant 或 NamespaceConstant\n  #[must_use]\n  pub fn is_value(&self) -> bool { self.is_extern_constant() || self.is_namespace_constant() }\n\n`;
  d += `  /// 原版 IsCallable: 全部可调用类别\n  #[must_use]\n  pub fn is_callable(&self) -> bool {\n    matches!(\n      self.kind,\n      DeclarableKind::TorqueMacro\n        | DeclarableKind::ExternMacro\n        | DeclarableKind::Method\n        | DeclarableKind::Builtin\n        | DeclarableKind::RuntimeFunction\n        | DeclarableKind::Intrinsic\n    )\n  }\n\n`;
  d += `  /// 原版 IsScope: Namespace 或 Callable (scope 存储是否在位)\n  #[must_use]\n  pub fn is_scope(&self) -> bool { self.is_namespace() || self.is_callable() }\n\n`;
  d += `  /// 原版 Position\n  #[must_use]\n  pub fn position(&self) -> SourcePosition { self.position }\n\n`;
  d += `  /// 原版 SetPosition\n  pub fn set_position(&mut self, position: SourcePosition) {\n    self.position = position;\n  }\n\n`;
  d += `  /// 原版 IdentifierPosition: 无标识符位置时回退整体位置\n  #[must_use]\n  pub fn identifier_position(&self) -> SourcePosition {\n    if self.identifier_position.source.is_valid() {\n      self.identifier_position\n    } else {\n      self.position\n    }\n  }\n\n`;
  d += `  /// 原版 SetIdentifierPosition\n  pub fn set_identifier_position(&mut self, position: SourcePosition) {\n    self.identifier_position = position;\n  }\n\n`;
  d += `  /// 原版 PrefixNameForCCOutput\n  #[must_use]\n  pub fn prefix_name_for_cc_output(name: &str) -> String {\n    format!("TqRuntime{name}")\n  }\n\n`;
  d += `  /// 原版 PrefixNameForCCDebugOutput\n  #[must_use]\n  pub fn prefix_name_for_cc_debug_output(name: &str) -> String {\n    format!("TqDebug{name}")\n  }\n`;
  d += `}\n\n`;

  d += `/// 原版 Scope::LookupShallow (限定名浅查; scope: &mut GlobalContext 由调用方持有,\n/// 此处以数据访问闭包抽象 — 填体时定形)\npub fn lookup_shallow() -> Result<Vec<DeclarableId>> {\n  // TODO(填体)\n  Err(Error::Unimplemented)\n}\n\n`;
  d += `/// 原版 Scope::Lookup (深查, 递归父链)\npub fn scope_lookup() -> Result<Vec<DeclarableId>> {\n  // TODO(填体)\n  Err(Error::Unimplemented)\n}\n\n`;
  d += `/// 原版 CurrentNamespace (沿父链找 Namespace) — 上下文 → 显式传参版\npub fn current_namespace() -> Result<DeclarableId> {\n  // TODO(填体)\n  Err(Error::Unimplemented)\n}\n\n`;
  d += `/// 原版 TypeConstraint 1:1\n#[derive(Debug, Clone, Default)]\npub struct TypeConstraint {\n  pub upper_bound: Option<TypeId>,\n}\n\n`;
  d += `impl TypeConstraint {\n  /// 原版 Unconstrained\n  #[must_use]\n  pub fn unconstrained() -> Self {\n    Self { upper_bound: None }\n  }\n\n`;
  d += `  /// 原版 SubtypeConstraint\n  #[must_use]\n  pub fn subtype_constraint(upper_bound: TypeId) -> Self {\n    Self { upper_bound: Some(upper_bound) }\n  }\n\n`;
  d += `  /// 原版 IsViolated\n  pub fn is_violated(&self, type_: TypeId) -> Result<Option<String>> {\n    // TODO(填体)\n    let _ = type_;\n    Err(Error::Unimplemented)\n  }\n`;
  d += `}\n\n`;
  d += `/// 原版 FindConstraintViolation\npub fn find_constraint_violation(types: &TypeVector, constraints: &[TypeConstraint]) -> Result<Option<String>> {\n  // TODO(填体)\n  let _ = (types, constraints);\n  Err(Error::Unimplemented)\n}\n\n`;
  d += `/// 原版 ComputeConstraints (scope 上下文 → 显式 DeclarableId + 泛型参数)\npub fn compute_constraints(scope: DeclarableId, parameters: GenericParameters) -> Result<Vec<TypeConstraint>> {\n  // TODO(填体)\n  let _ = (scope, parameters);\n  Err(Error::Unimplemented)\n}\n`;

  // 泛型参数类型: GenericParameters 在 ast.rs (原版 ast.h using)
  d += `\n// 填体提示: GenericParameters = Vec<GenericParameter>, 定义在 crate::ast。\n`;

  d += `\n` + sigBlock("declarable.cc", "签名清单");
  writeSrc("v8-torque", "declarable.rs", d);
}

// ===========================================================================
// declarations.{h,cc} → declarations.rs
// ===========================================================================
function genDeclarations() {
  const relH = "src/torque/declarations.h";
  const s = noCmt(readV8(relH));
  let d = banner("src/torque/declarations.js", relH, "声明表常量真转换 + Declarations 查询/创建接口");
  d += `
use crate::declarable::{BuiltinFlags, BuiltinKind, DeclarableId, DeclarableKind, QualifiedName};
use crate::error::{Error, Result};
use crate::global_context::GlobalContext;
use crate::ast::NodeId;
use crate::types::{Signature, TypeId, TypeVector};

// 原版 Declarations 全静态 (CurrentScope 上下文) → Rust 关联函数, 上下文链
// (GlobalContext 持 declarations arena) 由调用方显式传参; 具体参数形随填体定形。
`;

  d += `// 常量 1:1 (char* → &str)\n`;
  for (const m of s.matchAll(/static constexpr const char\* const (\w+) = "([^"]*)";|static constexpr const char\* (\w+) = "([^"]*)";/g)) {
    const name = m[1] ?? m[3], val = m[2] ?? m[4];
    d += `pub const ${name.replace(/^k/, "").replace(/([a-z0-9])([A-Z])/g, "$1_$2").replace(/([A-Z]+)([A-Z][a-z])/g, "$1_$2").toUpperCase()}: &str = "${val}";\n`;
  }
  d += `\n`;

  d += `/// 原版 Declarations (全静态 → Rust 关联函数; 上下文链 = GlobalContext 持\n/// declarations arena, 首参显式传入; 原签名逐条见 impl 内注释)\npub struct Declarations;\n\n`;
  d += `impl Declarations {\n`;
  const methods = [
    ["try_lookup", "TryLookup(name)", "pub fn try_lookup(cx: &GlobalContext, scope: DeclarableId, name: &QualifiedName) -> Result<Vec<DeclarableId>>", "Err(Error::Unimplemented)"],
    ["try_lookup_shallow", "TryLookupShallow(name)", "pub fn try_lookup_shallow(cx: &GlobalContext, scope: DeclarableId, name: &QualifiedName) -> Result<Vec<DeclarableId>>", "Err(Error::Unimplemented)"],
    ["lookup", "Lookup(name)", "pub fn lookup(cx: &GlobalContext, scope: DeclarableId, name: &QualifiedName) -> Result<Vec<DeclarableId>>", "Err(Error::Unimplemented)"],
    ["lookup_global_scope", "LookupGlobalScope(name)", "pub fn lookup_global_scope(cx: &GlobalContext, name: &QualifiedName) -> Result<DeclarableId>", "Err(Error::Unimplemented)"],
    ["lookup_type_alias", "LookupTypeAlias(name)", "pub fn lookup_type_alias(cx: &GlobalContext, name: &QualifiedName) -> Result<DeclarableId>", "Err(Error::Unimplemented)"],
    ["lookup_type_name", "LookupType(name: QualifiedName)", "pub fn lookup_type_name(cx: &GlobalContext, name: &QualifiedName) -> Result<TypeId>", "Err(Error::Unimplemented)"],
    ["lookup_type_identifier", "LookupType(identifier)", "pub fn lookup_type_identifier(cx: &GlobalContext, identifier: NodeId) -> Result<TypeId>", "Err(Error::Unimplemented)"],
    ["try_lookup_type", "TryLookupType(name)", "pub fn try_lookup_type(cx: &GlobalContext, name: &QualifiedName) -> Result<Option<TypeId>>", "Err(Error::Unimplemented)"],
    ["lookup_global_type", "LookupGlobalType(name)", "pub fn lookup_global_type(cx: &GlobalContext, name: &QualifiedName) -> Result<TypeId>", "Err(Error::Unimplemented)"],
    ["find_some_internal_builtin_with_type", "FindSomeInternalBuiltinWithType(type)", "pub fn find_some_internal_builtin_with_type(cx: &GlobalContext, type_: TypeId) -> Result<DeclarableId>", "Err(Error::Unimplemented)"],
    ["lookup_value", "LookupValue(name)", "pub fn lookup_value(cx: &GlobalContext, name: &QualifiedName) -> Result<DeclarableId>", "Err(Error::Unimplemented)"],
    ["try_lookup_macro", "TryLookupMacro(name, types)", "pub fn try_lookup_macro(cx: &GlobalContext, name: &str, types: &TypeVector) -> Result<Option<DeclarableId>>", "Err(Error::Unimplemented)"],
    ["try_lookup_builtin", "TryLookupBuiltin(name)", "pub fn try_lookup_builtin(cx: &GlobalContext, name: &QualifiedName) -> Result<Option<DeclarableId>>", "Err(Error::Unimplemented)"],
    ["lookup_generic", "LookupGeneric(name)", "pub fn lookup_generic(cx: &GlobalContext, name: &str) -> Result<Vec<DeclarableId>>", "Err(Error::Unimplemented)"],
    ["lookup_unique_generic", "LookupUniqueGeneric(name)", "pub fn lookup_unique_generic(cx: &GlobalContext, name: &QualifiedName) -> Result<DeclarableId>", "Err(Error::Unimplemented)"],
    ["lookup_unique_generic_type", "LookupUniqueGenericType(name)", "pub fn lookup_unique_generic_type(cx: &GlobalContext, name: &QualifiedName) -> Result<DeclarableId>", "Err(Error::Unimplemented)"],
    ["lookup_global_unique_generic_type", "LookupGlobalUniqueGenericType(name)", "pub fn lookup_global_unique_generic_type(cx: &GlobalContext, name: &str) -> Result<DeclarableId>", "Err(Error::Unimplemented)"],
    ["try_lookup_generic_type", "TryLookupGenericType(name)", "pub fn try_lookup_generic_type(cx: &GlobalContext, name: &QualifiedName) -> Result<Option<DeclarableId>>", "Err(Error::Unimplemented)"],
    ["declare_namespace", "DeclareNamespace(name)", "pub fn declare_namespace(cx: &mut GlobalContext, name: &str) -> Result<DeclarableId>", "Err(Error::Unimplemented)"],
    ["declare_type", "DeclareType(name, type)", "pub fn declare_type(cx: &mut GlobalContext, name: NodeId, type_: TypeId) -> Result<DeclarableId>", "Err(Error::Unimplemented)"],
    ["predeclare_type_alias", "PredeclareTypeAlias(name, type, redeclaration)", "pub fn predeclare_type_alias(cx: &mut GlobalContext, name: NodeId, decl: NodeId, redeclaration: bool) -> Result<DeclarableId>", "Err(Error::Unimplemented)"],
    ["create_torque_macro", "CreateTorqueMacro(external_name, readable_name, exported_to_csa, signature, body, is_user_defined)", "pub fn create_torque_macro(cx: &mut GlobalContext, external_name: &str, readable_name: &str, exported_to_csa: bool, signature: Signature, body: Option<NodeId>, is_user_defined: bool) -> Result<DeclarableId>", "Err(Error::Unimplemented)"],
    ["create_extern_macro", "CreateExternMacro(name, external_assembler_name, signature)", "pub fn create_extern_macro(cx: &mut GlobalContext, name: &str, external_assembler_name: &str, signature: Signature) -> Result<DeclarableId>", "Err(Error::Unimplemented)"],
    ["declare_macro", "DeclareMacro(name, accessible_from_csa, external_assembler_name, signature, body, op, is_user_defined)", "pub fn declare_macro(cx: &mut GlobalContext, spec: MacroSpec, signature: Signature, body: Option<NodeId>) -> Result<DeclarableId>", "Err(Error::Unimplemented)"],
    ["create_method", "CreateMethod(class_type, name, signature, body)", "pub fn create_method(cx: &mut GlobalContext, class_type: TypeId, name: &str, signature: Signature, body: NodeId) -> Result<DeclarableId>", "Err(Error::Unimplemented)"],
    ["create_intrinsic", "CreateIntrinsic(name, signature)", "pub fn create_intrinsic(cx: &mut GlobalContext, name: &str, signature: Signature) -> Result<DeclarableId>", "Err(Error::Unimplemented)"],
    ["declare_intrinsic", "DeclareIntrinsic(name, signature)", "pub fn declare_intrinsic(cx: &mut GlobalContext, name: &str, signature: Signature) -> Result<DeclarableId>", "Err(Error::Unimplemented)"],
    ["create_builtin", "CreateBuiltin(external_name, readable_name, kind, flags, signature, use_counter_name, body)", "pub fn create_builtin(cx: &mut GlobalContext, spec: BuiltinSpec, signature: Signature, body: Option<NodeId>) -> Result<DeclarableId>", "Err(Error::Unimplemented)"],
    ["declare_runtime_function", "DeclareRuntimeFunction(name, signature)", "pub fn declare_runtime_function(cx: &mut GlobalContext, name: &str, signature: Signature) -> Result<DeclarableId>", "Err(Error::Unimplemented)"],
    ["declare_extern_constant", "DeclareExternConstant(name, type, value)", "pub fn declare_extern_constant(cx: &mut GlobalContext, name: NodeId, type_: TypeId, value: String) -> Result<DeclarableId>", "Err(Error::Unimplemented)"],
    ["declare_namespace_constant", "DeclareNamespaceConstant(name, type, body)", "pub fn declare_namespace_constant(cx: &mut GlobalContext, name: NodeId, type_: TypeId, body: NodeId) -> Result<DeclarableId>", "Err(Error::Unimplemented)"],
    ["declare_generic_callable", "DeclareGenericCallable(name, ast_node)", "pub fn declare_generic_callable(cx: &mut GlobalContext, name: &str, ast_node: NodeId) -> Result<DeclarableId>", "Err(Error::Unimplemented)"],
    ["declare_generic_type", "DeclareGenericType(name, ast_node)", "pub fn declare_generic_type(cx: &mut GlobalContext, name: &str, ast_node: NodeId) -> Result<DeclarableId>", "Err(Error::Unimplemented)"],
    ["declare_operator", "DeclareOperator(name, m)", "pub fn declare_operator(cx: &mut GlobalContext, name: &str, m: DeclarableId) -> Result<DeclarableId>", "Err(Error::Unimplemented)"],
    ["get_generated_callable_name", "GetGeneratedCallableName(name, specialized_types)", "pub fn get_generated_callable_name(cx: &mut GlobalContext, name: &str, specialized_types: &TypeVector) -> Result<String>", "Err(Error::Unimplemented)"],
  ];
  // 从签名串提取参数名, 桩体统一 let _ 消未用变量
  const paramsOf = (sig) => {
    const m = sig.match(/\(([\s\S]*)\)\s*->/);
    if (!m) return [];
    return m[1].split(",")
      .map((p) => p.trim().split(":")[0].trim())
      .filter((p) => p && p !== "self" && !p.startsWith("&") && p !== "mut");
  };
  for (const [m, orig, sig, body] of methods) {
    const ps = paramsOf(sig);
    const bind = ps.length ? (ps.length === 1 ? `    let _ = ${ps[0]};\n` : `    let _ = (${ps.join(", ")});\n`) : "";
    d += `  /// 原版 ${orig}\n  ${sig} {\n    // TODO(填体)\n${bind}    ${body}\n  }\n\n`;
  }
  d += `  /// 原版 FilterDeclarables<T> (模板) → 按种类过滤\n  #[must_use]\n  pub fn filter_declarables(\n    cx: &GlobalContext,\n    list: &[DeclarableId],\n    kind: DeclarableKind,\n  ) -> Vec<DeclarableId> {\n    list\n      .iter()\n      .copied()\n      .filter(|id| cx.declarable(*id).kind == kind)\n      .collect()\n  }\n`;
  d += `}\n\n`;
  d += `/// declare_macro 参数束 (原版 7 参 → 束; too_many_arguments 治理)\n#[derive(Debug, Clone, Default)]\npub struct MacroSpec {\n  pub name: String,\n  pub accessible_from_csa: bool,\n  pub external_assembler_name: Option<String>,\n  pub op: Option<String>,\n  pub is_user_defined: bool,\n}\n\n`;
  d += `/// create_builtin 参数束 (原版 5 参 → 束)\n#[derive(Debug, Clone, Default)]\npub struct BuiltinSpec {\n  pub external_name: String,\n  pub readable_name: String,\n  pub kind: BuiltinKind,\n  pub flags: BuiltinFlags,\n  pub use_counter_name: Option<String>,\n}\n\n`;
  d += `// 原版 TypeConstraint 相关: 见 declarable.rs (compute_constraints/find_constraint_violation)\n`;
  d += `// 原版 SpecializationKey<GenericCallable> → crate::types::SpecializationKey (DeclarableId 泛化)\n`;
  d += `// 原版 FilterDeclarables 泛型按 DeclarableKind 过滤, 泛型 T 退化由调用方 kind 判别承担。\n`;

  d += `\n` + sigBlock("declarations.cc", "签名清单");
  writeSrc("v8-torque", "declarations.rs", d);
}

// ===========================================================================
// parameter-difference.h → parameter_difference.rs
// ===========================================================================
function genParameterDifference() {
  const relH = "src/torque/parameter-difference.h";
  let d = banner("src/torque/declarable.js", relH, "参数差集 (重载决议比较器)");
  d += `
use crate::types::{TypeId, TypeVector};

// 原版 ParameterDifference 1:1: 逐参数比较调用实参型与签名形参型的差集;
// None = 需要隐式转换, Some = 签名中的超型。
#[derive(Debug, Clone, Default)]
pub struct ParameterDifference {
  pub difference: Vec<Option<TypeId>>,
}

impl ParameterDifference {
  /// 原版构造: 逐位 AddParameter(to[i], from[i])
  #[must_use]
  pub fn new(to: &TypeVector, from: &TypeVector) -> Self {
    // TODO(填体): AddParameter (IsSubtypeOf / IsAssignableFrom 判定)
    let _ = (to, from);
    Self::default()
  }

  /// 原版 StrictlyBetterThan: 严格优于全部备选才当选
  #[must_use]
  pub fn strictly_better_than(&self, other: &Self) -> bool {
    // TODO(填体)
    let _ = other;
    false
  }
}
`;
  writeSrc("v8-torque", "parameter_difference.rs", d);
}

// ===========================================================================
// type-inference.{h,cc} → type_inference.rs
// ===========================================================================
function genTypeInference() {
  const relH = "src/torque/type-inference.h";
  const relCc = "src/torque/type-inference.cc";
  let d = banner("src/torque/declarable.js", relH, "类型实参推断");
  d += `
use crate::ast::{GenericParameter, NodeId};
use crate::error::{Error, Result};
use crate::types::{TypeId, TypeVector};
use std::collections::BTreeMap;

// 原版 TypeArgumentInference 1:1 — 泛型实参推断 (见原版头注释):
// 形参类型表达式 (NodeId) 与实参型 (TypeId) 匹配; 显式给出的实参忽略不一致约束;
// 函数/联合形参不产生约束。推断在相应 scope 内实例化 (上下文 → 构造时传参)。
#[derive(Debug, Clone, Default)]
pub struct TypeArgumentInference {
  pub num_explicit: usize,
  pub type_parameter_from_name: BTreeMap<String, usize>,
  pub inferred: Vec<Option<TypeId>>,
  pub failure_reason: Option<String>,
}

impl TypeArgumentInference {
  /// 原版构造 (type_parameters: 泛型形参表, term_parameters: 项形参类型表达式)
  #[must_use]
  pub fn new(
    type_parameters: &[GenericParameter],
    explicit_type_arguments: &TypeVector,
    term_parameters: &[NodeId],
    term_argument_types: &[Option<TypeId>],
  ) -> Self {
    // TODO(填体): Match 循环
    let _ = (type_parameters, explicit_type_arguments, term_parameters, term_argument_types);
    Self::default()
  }

  #[must_use]
  pub fn has_failed(&self) -> bool {
    self.failure_reason.is_some()
  }

  /// 原版 GetFailureReason (前置条件: HasFailed())
  #[must_use]
  pub fn failure_reason(&self) -> &str {
    self.failure_reason.as_deref().unwrap_or_default()
  }

  /// 原版 GetResult
  pub fn get_result(&self) -> Result<TypeVector> {
    // TODO(填体)
    Err(Error::Unimplemented)
  }

  /// 原版 Fail
  pub fn fail(&mut self, reason: String) {
    self.failure_reason = Some(reason);
  }

  /// 原版 Match (私有)
  pub fn match_parameter(&mut self, parameter: NodeId, argument_type: Option<TypeId>) {
    // TODO(填体)
    let _ = (parameter, argument_type);
  }

  /// 原版 MatchGeneric (私有)
  pub fn match_generic(&mut self, parameter: NodeId, argument_type: Option<TypeId>) {
    // TODO(填体)
    let _ = (parameter, argument_type);
  }
}
`;
  d += `\n` + sigBlock(relCc, "签名清单");
  d += `\n// 填体提示: 推断结果经 GenericCallable::GetSpecialization (specializations 表) 消费。\n`;
  writeSrc("v8-torque", "type_inference.rs", d);
}

// ===========================================================================
// 主流程
// ===========================================================================
genDeclarable();
genDeclarations();
genParameterDifference();
genTypeInference();
console.log("== v8-torque 声明层直出完毕: declarable.rs + declarations.rs + parameter_difference.rs + type_inference.rs ==");
