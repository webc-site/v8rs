#!/usr/bin/env bun
// v8/src/torque/ast.h → v8-torque/src/ast.rs 直出
// X-macro 节点清单机械展开为 AstNodeKind + AstNode enum; AST 节点结构体字段机械提取;
// 节点指针一律 NodeId arena 索引; 类别层级 (Expression/Statement/Declaration/TypeExpression)
// 以 AstNodeKind 谓词保留 (对应 AstNodeClassCheck::IsInstanceOf)。
import { readV8, writeSrc, banner } from "../../lib.js";

const src = readV8("src/torque/ast.h");

// ---------- 通用小工具 ----------
const stripComments = (s) =>
  s
    .replace(/\/\*[\s\S]*?\*\//g, "")
    .replace(/\/\/.*$/gm, "")
    .replace(/[ \t]+$/gm, "");

function blockAfter(header, text, from = 0) {
  // 词边界匹配: header 后不能紧跟标识符字符 (防 "struct Annotation" 前缀命中 AnnotationParameter)
  let i = from;
  for (;;) {
    i = text.indexOf(header, i);
    if (i < 0) return null;
    const after = text[i + header.length] ?? " ";
    if (!/[A-Za-z0-9_]/.test(after)) {
      const open = text.indexOf("{", i + header.length);
      if (open < 0) return null;
      let depth = 0;
      for (let j = open; j < text.length; j++) {
        if (text[j] === "{") depth++;
        else if (text[j] === "}") {
          depth--;
          if (depth === 0) return { start: open + 1, end: j, body: text.slice(open + 1, j) };
        }
      }
      return null;
    }
    i += header.length;
  }
}

function expandList(name, text) {
  const re = new RegExp(`#define\\s+${name}\\(V\\)([\\s\\S]*?)(?=\\n\\n|#define|struct|class|enum)`);
  const m = text.match(re);
  if (!m) return [];
  const items = [];
  for (const mm of m[1].matchAll(/V\((\w+)\)/g)) items.push(mm[1]);
  return items;
}

const CAMEL1 = /([a-z0-9])([A-Z])/g, CAMEL2 = /([A-Z]+)([A-Z][a-z])/g;
const snake = (s) =>
  s.replaceAll(CAMEL1, "$1_$2").replaceAll(CAMEL2, "$1_$2").toLowerCase();

// C++ 类型 → 骨架 Rust 类型 (节点指针一律 NodeId)
function rustType(t) {
  let s = t.trim().replaceAll(/\s+/g, " ");
  let opt = false, vec = false;
  for (;;) {
    if (s.startsWith("std::optional<") && s.endsWith(">")) {
      opt = true; s = s.slice("std::optional<".length, -1); continue;
    }
    if (s.startsWith("std::vector<") && s.endsWith(">")) {
      vec = true; s = s.slice("std::vector<".length, -1); continue;
    }
    break;
  }
  if (s.endsWith("*")) s = "NodeId";
  const prim = {
    "std::string": "String", "std::string_view": "String", double: "f64",
    float: "f32", bool: "bool", int: "i32", "int32_t": "i32",
    "uint8_t": "u8", "uint32_t": "u32", "size_t": "usize",
    "const char*": "&'static str", "SourcePosition": "SourcePosition",
    "SourceId": "SourceId", "IntegerLiteral": "IntegerLiteral",
    "AbstractTypeFlags": "AbstractTypeFlags", "ClassFlags": "ClassFlags",
    "StructFlags": "StructFlags", "FieldSynchronization": "FieldSynchronization",
    "InstanceTypeConstraints": "InstanceTypeConstraints",
  };
  let r = prim[s] ?? s;
  if (r === "type") r = "NodeId";
  // 类内枚举重命名: DebugStatement::Kind → DebugKind; EnumDescription::Entry → EnumEntryDesc
  if (r === "Kind") r = "DebugKind";
  if (r === "Entry") r = "EnumEntryDesc";
  if (vec) r = `Vec<${r}>`;
  if (opt) r = `Option<${r}>`;
  return r;
}

// 从结构体体内提取字段行 "TYPE name;" / "TYPE name = 默认值;"
function extractFields(body) {
  const out = [];
  for (const raw of body.split("\n")) {
    const line = raw.trim();
    if (!line.endsWith(";")) continue;
    const pre = line.slice(0, -1);
    // 只跳函数形态行: 含函数体 `{`, 或 `(` 先于 `=` (方法/构造/返回语句);
    // 保留带 `=` 的字段默认初始化行 (如 SourcePosition x = SourcePosition::Invalid())
    const eq = pre.indexOf("=");
    const par = pre.indexOf("(");
    if (pre.includes("{") || (par >= 0 && (eq < 0 || par < eq))) continue;
    if (/^(DEFINE_|static|using|typedef|friend|template|enum)/.test(line)) continue;
    const decl = (eq >= 0 ? pre.slice(0, eq) : pre).trim(); // 去 C++ 默认初始化
    const m = decl.match(/^(.+?)\s*\b(\w+)$/s);
    if (!m) continue;
    out.push({ type: rustType(m[1]), name: m[2] });
  }
  return out;
}

// Rust 字段名修正 (关键字)
// super 不能作 raw identifier (保留字), ClassDeclaration.super → super_class
const FIELD_RENAME = { type: "r#type", enum: "r#enum", ref: "r#ref", super: "super_class" };
const fname = (n) => FIELD_RENAME[n] ?? n;

// 提取类内 enum class Name { kA, kB };
function extractInnerEnums(body, owner) {
  const out = [];
  const re = /enum class (\w+)\s*\{([^}]*)\}/g;
  for (const m of body.matchAll(re)) {
    const variants = [...m[2].matchAll(/k(\w+)/g)].map((x) => x[1]);
    // 内嵌枚举名: Kind → DebugKind; AssertKind/HandlerKind 等自带语义名直接用
    let short;
    if (m[1] === "Kind") short = owner.replace(/Statement$/, "") + "Kind";
    else if (m[1].endsWith("Kind")) short = m[1];
    else short = owner + m[1];
    out.push({ name: short, variants });
  }
  return out;
}

const pascal = (v) => snake(v).replace(/(^|_)(\w)/g, (_, __, c) => c.toUpperCase());

// ---------- 1. X-macro 清单 → AstNodeKind ----------
const exprKinds = expandList("AST_EXPRESSION_NODE_KIND_LIST", src);
const typeExprKinds = expandList("AST_TYPE_EXPRESSION_NODE_KIND_LIST", src);
const stmtKinds = expandList("AST_STATEMENT_NODE_KIND_LIST", src);
const typeDeclKinds = expandList("AST_TYPE_DECLARATION_NODE_KIND_LIST", src);
const declKinds = expandList("AST_DECLARATION_NODE_KIND_LIST", src);
const declOnly = declKinds.filter((k) => !typeDeclKinds.includes(k));
const callableKinds = [
  "TorqueMacroDeclaration", "TorqueBuiltinDeclaration", "ExternalMacroDeclaration",
  "ExternalBuiltinDeclaration", "ExternalRuntimeDeclaration", "IntrinsicDeclaration",
  "GenericCallableDeclaration", "SpecializationDeclaration",
];
// AST_NODE_KIND_LIST 是组合宏, 显式展开: 五个子清单 + 3 个独立项
const allKinds = [...exprKinds, ...typeExprKinds, ...stmtKinds, ...typeDeclKinds, ...declOnly, "Identifier", "TryHandler", "ClassBody"];

let out = banner("src/torque/ast.js", "src/torque/ast.h", "AST 节点体系全量 (60 kind 1:1)");
out += `
use crate::constants::{AbstractTypeFlags, ClassFlags, FieldSynchronization, StructFlags};
use crate::source_positions::{SourceId, SourcePosition};
use std::collections::{BTreeMap, BTreeSet};

// 层级映射: C++ 继承树 → NodeId 引用 + AstNode enum。
// AstNode Kind 标签 → AstNodeKind; Expression/Statement/Declaration/TypeExpression
// 四个中间层以 is_* 谓词保留 (对应 AstNodeClassCheck::IsInstanceOf)。
// 原版 IntegerLiteral (src/numbers/integer-literal.h) 在此最小组件化。
`;

out += `/// 原版 IntegerLiteral 最小建模 (原版 src/numbers/integer-literal.h: 符号 + 十进制数字串)\n#[derive(Debug, Clone, PartialEq, Eq)]\npub struct IntegerLiteral {\n  pub negative: bool,\n  pub digits: String,\n}\n\n`;

out += `/// AstNode::Kind 1:1 (${allKinds.length} 变体; 变体名 = 节点类名)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum AstNodeKind {
`;
for (const k of allKinds) out += `  ${k},\n`;
out += `}\n\n`;

const K = (k) => `Self::${k}`;
out += `impl AstNodeKind {\n`;
out += `  /// 对应 AST_EXPRESSION_NODE_KIND_LIST\n  #[must_use]\n  pub fn is_expression(self) -> bool { matches!(self, ${exprKinds.map(K).join(" | ")}) }\n\n`;
out += `  /// 对应 AST_TYPE_EXPRESSION_NODE_KIND_LIST\n  #[must_use]\n  pub fn is_type_expression(self) -> bool { matches!(self, ${typeExprKinds.map(K).join(" | ")}) }\n\n`;
out += `  /// 对应 AST_STATEMENT_NODE_KIND_LIST\n  #[must_use]\n  pub fn is_statement(self) -> bool { matches!(self, ${stmtKinds.map(K).join(" | ")}) }\n\n`;
out += `  /// 对应 AST_DECLARATION_NODE_KIND_LIST (首行展开 AST_TYPE_DECLARATION_NODE_KIND_LIST, 18 kind)\n  #[must_use]\n  pub fn is_declaration(self) -> bool { matches!(self, ${[...typeDeclKinds, ...declKinds].map(K).join(" | ")}) }\n\n`;
out += `  /// CallableDeclaration 子树 (含宏/内建/运行时/内联函数/泛型/特化声明)\n  #[must_use]\n  pub fn is_callable(self) -> bool { matches!(self, ${callableKinds.map(K).join(" | ")}) }\n`;
out += `}\n\n`;

// ---------- 2. 顶层 enum ----------
const topEnums = [
  ["IncrementDecrementOperator", ["Increment", "Decrement"]],
  // ImplicitKind 首变体即原版默认 (ParameterList::implicit_kind = kNoImplicit)
  ["ImplicitKind", ["NoImplicit", "JsImplicit", "Implicit"]],
  ["IncludeSelector", ["Any", "Csa", "Tsa"]],
  ["ConditionalAnnotationType", ["Positive", "Negative"]],
];
for (const [name, vs] of topEnums) {
  // Default 结构体引用的枚举取首变体 (ConditionalAnnotation / ParameterList)
  const needsDefault = name === "ConditionalAnnotationType" || name === "ImplicitKind";
  const derives = needsDefault
    ? "Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default"
    : "Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash";
  out += `/// ${name} 1:1\n#[derive(${derives})]\n#[repr(u8)]\npub enum ${name} {\n`;
  for (let vi = 0; vi < vs.length; vi++) {
    const tag = needsDefault && vi === 0 ? "  #[default]\n" : "";
    out += `${tag}  ${vs[vi]},\n`;
  }
  out += `}\n\n`;
}

// ---------- 3. 辅助结构体 (非 AstNode) ----------
const auxStructs = [
  "NameAndExpression", "TypeswitchCase", "ParameterList",
  "NameAndTypeExpression", "ImplicitParameters", "StructFieldExpression",
  "BitFieldDeclaration", "ConditionalAnnotation", "AnnotationParameter",
  "Annotation", "ClassFieldIndexInfo", "ClassFieldExpression", "LabelAndTypes",
  "GenericParameter", "InstanceTypeConstraints",
];
const noComment = stripComments(src);
const innerEnums = [];
for (const name of auxStructs) {
  const blk = blockAfter(`struct ${name}`, noComment);
  if (!blk) { console.error(`缺失结构体: ${name}`); continue; }
  for (const e of extractInnerEnums(blk.body, name)) innerEnums.push(e);
  if (name === "InstanceTypeConstraints") {
    out += `/// ${name} 1:1 (原版默认 value=-1, num_flags_bits=-1)\n#[derive(Debug, Clone, PartialEq, Eq)]\npub struct ${name} {\n  pub value: i32,\n  pub num_flags_bits: i32,\n}\n\nimpl Default for ${name} {\n  fn default() -> Self {\n    Self { value: -1, num_flags_bits: -1 }\n  }\n}\n\n`;
    continue;
  }
  const fields = extractFields(blk.body);
  // Default 语义登记: ParameterList 全字段默认 1:1 (implicit_kind=kNoImplicit,
  // implicit_kind_pos=Invalid, implicit_count=0, has_varargs=false, arguments_variable=""),
  // 由 derive(Default) + ImplicitKind/SourcePosition 的 Default 承担
  out += name === "ParameterList"
    ? `/// ${name} 1:1 (Default 语义 1:1, 对应原版 ParameterList::Empty)\n#[derive(Debug, Clone, Default)]\npub struct ${name} {\n`
    : `/// ${name} 1:1\n#[derive(Debug, Clone, Default)]\npub struct ${name} {\n`;
  for (const f of fields) out += `  pub ${fname(f.name)}: ${f.type},\n`;
  out += `}\n\n`;
}
// EnumDescription::Entry (嵌套结构体)
out += `/// 原版 EnumDescription::Entry 1:1\n#[derive(Debug, Clone, Default)]\npub struct EnumEntryDesc {\n  pub name: String,\n  pub alias_entry: String,\n}\n\n`;
out += `/// EnumDescription 1:1 (嵌套 Entry 抽出为 EnumEntryDesc, 登记偏差)\n#[derive(Debug, Clone, Default)]\npub struct EnumDescription {\n  pub pos: SourcePosition,\n  pub name: String,\n  pub constexpr_generates: String,\n  pub is_open: bool,\n  pub entries: Vec<EnumEntryDesc>,\n}\n\n`;

// ---------- 4. 节点内 enum ----------
for (const kind of allKinds) {
  const blk = blockAfter(`struct ${kind}`, noComment)
    ?? blockAfter(`class ${kind}`, noComment);
  if (!blk) { console.error(`缺失节点: ${kind}`); continue; }
  for (const e of extractInnerEnums(blk.body, kind)) innerEnums.push(e);
}
for (const e of innerEnums) {
  // ConditionalAnnotationType 被 Default 结构体引用 → 首变体 #[default]
  const needsDefault = e.name === "ConditionalAnnotationType";
  const derives = needsDefault
    ? "Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default"
    : "Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash";
  out += `/// ${e.name} (原版类内 enum) 1:1\n#[derive(${derives})]\n#[repr(u8)]\npub enum ${e.name} {\n`;
  for (let vi = 0; vi < e.variants.length; vi++) {
    const v = e.variants[vi];
    const tag = needsDefault && vi === 0 ? "  #[default]\n" : "";
    out += `${tag}  ${pascal(v)},\n`;
  }
  out += `}\n\n`;
}

// ---------- 5. AST 节点叶子结构体 ----------
// C++ 继承链: struct X : Y → parentOf[X] = Y (ast.h 节点声明区)
const parentOf = new Map();
for (const m of noComment.matchAll(/(?:struct|class)\s+(\w+)\s*:\s*([\w:]+)\s*\{/g)) {
  parentOf.set(m[1], m[2]);
}
// 基类字段上提 (登记决策: 平铺上提, 不引 Common 结构体 — ast.rs 节点为纯数据结构体,
// 平铺保持构造直白; declarable.rs 的 CallableCommon 模式针对私有实现字段组合, 场景不同)
// 中间层基类数据字段: TypeDeclaration.name (ast.h:869) / CallableDeclaration 5 字段
// (ast.h:988-992) / MacroDeclaration.op (ast.h:1002) / BuiltinDeclaration.javascript_linkage
// (ast.h:1076); 根基类 (AstNode/Expression/Statement/Declaration/TypeExpression) 仅
// kind/pos → 已由 NodeHeader 承担, 不上提。
const HOIST_BASES = new Set(["TypeDeclaration", "CallableDeclaration", "MacroDeclaration", "BuiltinDeclaration"]);
const baseFields = new Map();
for (const b of HOIST_BASES) {
  const blk = blockAfter(`struct ${b}`, noComment);
  if (!blk) { console.error(`缺失基类: ${b}`); continue; }
  baseFields.set(b, extractFields(blk.body));
}
const inheritedFields = (kind) => {
  const out = [];
  for (let p = parentOf.get(kind); p && HOIST_BASES.has(p); p = parentOf.get(p)) {
    out.unshift(...(baseFields.get(p) ?? []));
  }
  return out;
};

const nodeStructs = [];
for (const kind of allKinds) {
  const blk = blockAfter(`struct ${kind}`, noComment)
    ?? blockAfter(`class ${kind}`, noComment);
  if (!blk) { console.error(`缺失节点: ${kind}`); continue; }
  nodeStructs.push({ kind, fields: [...inheritedFields(kind), ...extractFields(blk.body)] });
}
out += `/// 每个节点共用头: kind 标签 + 源位置 (对应 C++ AstNode 基类)\n#[derive(Debug, Clone, Copy, PartialEq, Eq)]\npub struct NodeHeader {\n  pub kind: AstNodeKind,\n  pub pos: SourcePosition,\n}\n\n`;
for (const n of nodeStructs) {
  out += `/// ${n.kind} 1:1 (指针字段 → NodeId)\n#[derive(Debug, Clone)]\npub struct ${n.kind} {\n`;
  for (const f of n.fields) out += `  pub ${fname(f.name)}: ${f.type},\n`;
  out += `}\n\n`;
}

// ---------- 6. Ast 容器 (arena) ----------
out += `/// 原版 using GenericParameters = std::vector<GenericParameter>\npub type GenericParameters = Vec<GenericParameter>;\n\n`;
out += `/// 原版 using LabelAndTypesVector = std::vector<LabelAndTypes>\npub type LabelAndTypesVector = Vec<LabelAndTypes>;\n\n`;
out += `/// 节点索引: 对应 C++ AstNode*\npub type NodeId = u32;\n\n`;
out += `/// 全部叶子节点合拢; 类别层级用 AstNodeKind 谓词查询 (Box 控制枚举尺寸)\n#[derive(Debug, Clone)]\npub enum AstNode {\n`;
for (const n of nodeStructs) out += `  ${n.kind}(Box<${n.kind}>),\n`;
out += `}\n\n`;
out += `/// Ast 容器: C++ 用 unique_ptr 存活 + 裸指针互联, Rust 改 arena (Vec + NodeId)\n#[derive(Debug, Default)]\npub struct Ast {\n  pub declarations: Vec<NodeId>,\n  pub nodes: Vec<(NodeHeader, AstNode)>,\n  pub declared_imports: BTreeMap<SourceId, BTreeSet<SourceId>>,\n  pub enum_descriptions: Vec<EnumDescription>,\n}\n\n`;
out += `impl Ast {\n`;
out += `  #[must_use]\n  pub fn new() -> Self { Self::default() }\n\n`;
out += `  /// 原版 Ast::AddNode: 入 arena, 返回 NodeId (kind/pos 随 header 登记)\n  pub fn add_node(&mut self, header: NodeHeader, node: AstNode) -> NodeId {\n    self.nodes.push((header, node));\n    (self.nodes.len() - 1) as NodeId\n  }\n\n`;
out += `  #[must_use]\n  pub fn node(&self, id: NodeId) -> &(NodeHeader, AstNode) {\n    &self.nodes[id as usize]\n  }\n\n`;
out += `  /// 原版 DeclareImportForCurrentFile (CurrentSourcePosition 上下文 → 显式传参)\n  pub fn declare_import_for_current_file(&mut self, current: SourceId, import_id: SourceId) {\n    self.declared_imports.entry(current).or_default().insert(import_id);\n  }\n\n`;
out += `  /// 原版 AddEnumDescription\n  pub fn add_enum_description(&mut self, description: EnumDescription) {\n    debug_assert!(!description.name.is_empty());\n    self.enum_descriptions.push(description);\n  }\n\n`;
out += `  #[must_use]\n  pub fn enum_descriptions(&self) -> &[EnumDescription] { &self.enum_descriptions }\n`;
out += `}\n`;
out += `
// 原版自由构造函数 (MakeNode/MakeCallExpression/MakeFieldAccessExpression 等) 经
// CurrentAst + CurrentSourcePosition 上下文建节点; Rust 版由 parser 动作持有
// &mut Ast + SourcePosition 显式构造, 不设独立自由函数 (登记偏差)。

// 原版 kThisParameterName`;
out += `\npub const K_THIS_PARAMETER_NAME: &str = "this";\n`;

writeSrc("v8-torque", "ast.rs", out);
console.log(`ast.h → ${nodeStructs.length} 节点, ${allKinds.length} kind, ${auxStructs.length} 辅助结构体, ${innerEnums.length} 内嵌枚举`);
