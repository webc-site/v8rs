#!/usr/bin/env bun
// v8/src/torque/types.{h,cc} + type-oracle.{h,cc} + type-visitor.{h,cc} → 直出
//   types.h/.cc        → types.rs        (Type 体系全量: 7 kind + 载荷 + 签名/字段)
//   type-oracle.h/.cc  → type_oracle.rs  (Type arena + interning + 查询接口)
//   type-visitor.h/.cc → type_visitor.rs (类型计算访问器接口)
// 继承树映射: TypeBase(kKind) → TypeKind 七变体; C++ const Type* → TypeId (arena 索引);
// TypeOracle 承担 interning (指针同一性 → TypeId 相等性); 上下文单例 → 驱动持有。
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
    else if (text[j] === "}") { depth--; if (depth === 0) {
      return { name: header, head: m[0].replace(/\s+/g, " "), body: text.slice(open + 1, j), start: headStart };
    } }
  }
  return null;
}

function memberFields(body) {
  const out = [];
  for (const raw of body.split("\n")) {
    const line = raw.trim();
    if (!line.endsWith(";") || line.includes("(")) continue;
    if (/^(DEFINE_|DECLARE_|friend|using|typedef|enum|static)/.test(line)) continue;
    if (line.startsWith("//")) continue;
    const m = line.slice(0, -1).match(/^(.+?)\s*\b(\w+)$/);
    if (m && !/[<>=]/.test(m[2])) out.push({ type: m[1].replace(/\s+/g, " "), name: m[2] });
  }
  return out;
}

function enumBody(name, text) {
  const blk = classBlock(name, text);
  if (!blk) return null;
  const vs = [...blk.body.matchAll(/\bk([A-Z]\w*)\b/g)].map((x) => x[1]);
  return { name, variants: [...new Set(vs)] };
}

function sigBlock(f, note = "签名清单 (填体正交任务源)") {
  const s = readV8(`src/torque/${f}`);
  let dd = `// ── 原版 src/torque/${f} ──\n`;
  const sigs = topSigs(s);
  if (sigs.length === 0 && f.endsWith(".cc") && s.length > 400) {
    console.error(`[torque] ${f} 非空但抽到 0 条签名 — noCmt/topSigs 异常, 防静默检查`);
  }
  for (const x of sigs) dd += `// L${x.line}: ${x.sig}\n`;
  return dd;
}

// ===========================================================================
// types.h → types.rs
// ===========================================================================
function genTypes() {
  const relH = "src/torque/types.h";
  const th = noCmt(readV8(relH));

  let d = banner("src/torque/types.js", relH, "Type 体系全量 (7 kind + 载荷结构体 + 签名/字段/标签)");
  d += `
use crate::ast::{ClassFieldIndexInfo, NodeId};
use crate::constants::{AbstractTypeFlags, ClassFlags, FieldSynchronization};
use crate::declarable::DeclarableId;
use crate::error::{Error, Result};
use crate::source_positions::SourcePosition;
use crate::utils::{ResidueClass, StackRange};
use std::collections::BTreeSet;

// 继承树映射: TypeBase(kKind) → TypeKind 七变体; C++ const Type* → TypeId (arena 索引,
// 由 type_oracle.rs 的 TypeOracle 存储); TypeOracle 承担 interning (去重 + 指针同一性
// → TypeId 相等性)。AggregateType 中间层字段抽为 AggregateCommon, Struct/Class 持有。
`;

  const kindEnum = enumBody("Kind", th.slice(th.indexOf("class TypeBase")));
  if (!kindEnum) throw new Error("TypeBase::Kind 未提取");
  d += `/// TypeBase::Kind 1:1 (${kindEnum.variants.length} 变体)\n#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]\n#[repr(u8)]\npub enum TypeKind {\n`;
  for (const v of kindEnum.variants) d += `  ${v.replace(/^k/, "")},\n`;
  d += `}\n\n`;

  d += `/// 原版 using TypeVector = std::vector<const Type*>\npub type TypeVector = Vec<TypeId>;\n\n`;
  d += `/// 原版 using NameAndTypeVector = std::vector<NameAndType>\npub type NameAndTypeVector = Vec<NameAndType>;\n\n`;
  d += `/// 类型 arena 索引 (对应 C++ const Type*)\npub type TypeId = u32;\n\n`;

  for (const [en, owner] of [["HandleKind", "Type"], ["ObjectSlotKind", ""], ["ParameterMode", ""], ["ClassificationFlag", "StructType"]]) {
    const e = enumBody(en, th);
    if (!e) { console.error(`缺失枚举 ${en}`); continue; }
    d += `/// ${owner ? owner + "::" : ""}${en} 1:1\n#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]\n#[repr(u8)]\npub enum ${owner}${en} {\n`;
    for (const v of e.variants) d += `  ${v.replace(/^k/, "")},\n`;
    d += `}\n\n`;
  }
  // ObjectSlotKind::Combine (自由内联函数, 平凡合并)
  d += `/// 原版 Combine(a, b): 强/弱指针槽合并\n#[must_use]\npub fn combine_object_slot_kind(a: ObjectSlotKind, b: ObjectSlotKind) -> Option<ObjectSlotKind> {\n  if a == b {\n    return Some(a);\n  }\n  let (lo, hi) = if (a as u8) < (b as u8) { (a, b) } else { (b, a) };\n  if matches!((lo, hi), (ObjectSlotKind::StrongPointer, ObjectSlotKind::MaybeObjectPointer)) {\n    return Some(ObjectSlotKind::MaybeObjectPointer);\n  }\n  None\n}\n\n`;

  // 简单结构体 (无虚函数): 字段真转换
  const simpleStructs = ["TypeChecker", "NameAndType", "BitField", "LabelDefinition", "LabelDeclaration", "ParameterTypes"];
  const R = (t) => t.trim().replaceAll(/\s+/g, " ")
    .replace(/std::optional<(.*?)>/g, "Option<$1>").replace(/std::vector<(.*?)>/g, "Vec<$1>")
    .replace(/std::string\b/g, "String").replace(/\bsize_t\b/g, "usize").replace(/\bbool\b/g, "bool")
    .replace(/\bint\b/g, "i32").replace(/const Type\*/g, "TypeId").replace(/Type\*/g, "TypeId")
    .replace(/const AggregateType\*/g, "TypeId").replace(/Identifier\*/g, "NodeId")
    .replace(/GenericType\*/g, "DeclarableId").replace(/TypeExpression\*/g, "NodeId")
    .replace(/const/g, "").replace(/&/g, "").replace(/\s+/g, " ").trim();
  for (const name of simpleStructs) {
    const blk = classBlock(name, th);
    if (!blk) { console.error(`缺失结构体 ${name}`); continue; }
    const fs = memberFields(blk.body);
    d += `/// ${name} 1:1\n#[derive(Debug, Clone, Default, PartialEq, Eq)]\npub struct ${name} {\n`;
    for (const f of fs) d += `  pub ${f.name === "type" ? "r#type" : f.name}: ${R(f.type).replace(/\btype\b/g, "TypeId").replace("r#TypeId", "r#type").replace(/\bNodeId\b/g, "NodeId")},\n`;
    d += `}\n\n`;
  }

  d += `/// 原版 SpecializationKey<T> (两实例合一: generic 指针 → DeclarableId)\n#[derive(Debug, Clone, Default)]\npub struct SpecializationKey {\n  pub generic: DeclarableId,\n  pub specialized_types: TypeVector,\n}\n\n`;
  d += `/// 原版 MaybeSpecializationKey\npub type MaybeSpecializationKey = Option<SpecializationKey>;\n\n`;

  // Field
  {
    const blk = classBlock("Field", th);
    d += `/// Field 1:1 (aggregate 指针 → TypeId)\n#[derive(Debug, Clone, Default)]\npub struct Field {\n  pub pos: SourcePosition,\n  pub aggregate: TypeId,\n  pub index: Option<ClassFieldIndexInfo>,\n  pub name_and_type: NameAndType,\n  /// 字节偏移; 未知 (索引字段之后 / 不支持堆上布局) 为 None\n  pub offset: Option<usize>,\n  pub custom_weak_marking: bool,\n  pub const_qualified: bool,\n  pub index_is_constant: bool,\n  pub synchronization: FieldSynchronization,\n}\n\n`;
  }

  // Signature
  {
    const blk = classBlock("Signature", th);
    d += `/// Signature 1:1 (构造函数字段 + 默认值)\n#[derive(Debug, Clone, Default)]\npub struct Signature {\n  pub parameter_names: Vec<NodeId>,\n  pub arguments_variable: Option<String>,\n  pub parameter_types: ParameterTypes,\n  pub implicit_count: usize,\n  pub return_type: Option<TypeId>,\n  pub labels: Vec<LabelDeclaration>,\n  pub transitioning: bool,\n}\n\n`;
    d += `impl Signature {\n`;
    d += `  /// 原版 types()\n  #[must_use]\n  pub fn types(&self) -> &TypeVector { &self.parameter_types.types }\n\n`;
  d += `  /// 原版 ExplicitCount\n  #[must_use]\n  pub fn explicit_count(&self) -> usize { self.parameter_types.types.len() - self.implicit_count }\n\n`;
  d += `  /// 原版 GetImplicitTypes\n  #[must_use]\n  pub fn get_implicit_types(&self) -> &[TypeId] { &self.parameter_types.types[..self.implicit_count] }\n\n`;
  d += `  /// 原版 GetExplicitTypes\n  #[must_use]\n  pub fn get_explicit_types(&self) -> &[TypeId] { &self.parameter_types.types[self.implicit_count..] }\n\n`;
  d += `  /// 原版 HasSameTypesAs (mode 决定隐式参数是否参与比较)\n  pub fn has_same_types_as(&self, other: &Self, mode: ParameterMode) -> bool {\n    // TODO(填体)\n    let _ = (other, mode);\n    false\n  }\n\n`;
  d += `  /// 原版 HasContextParameter\n  pub fn has_context_parameter(&self) -> bool {\n    // TODO(填体)\n    false\n  }\n`;
  d += `}\n\n`;
  }

  // VisitResult
  {
    d += `/// VisitResult 1:1: 类型 + (constexpr 值 | 栈区间) 二选一\n#[derive(Debug, Clone, Default)]\npub struct VisitResult {\n  pub ty: Option<TypeId>,\n  pub constexpr_value: Option<String>,\n  pub stack_range: Option<StackRange>,\n}\n\n`;
    d += `impl VisitResult {\n`;
    d += `  /// 原版 IsOnStack\n  #[must_use]\n  pub fn is_on_stack(&self) -> bool { self.stack_range.is_some() }\n\n`;
    d += `  /// 原版 NeverResult\n  #[must_use]\n  pub fn never_result() -> Self {\n    // TODO(填体)\n    Self::default()\n  }\n\n`;
    d += `  /// 原版 TopTypeResult\n  #[must_use]\n  pub fn top_type_result(top_reason: String, from_type: TypeId) -> Self {\n    // TODO(填体)\n    let _ = (top_reason, from_type);\n    Self::default()\n  }\n`;
    d += `}\n\n`;
    d += `/// 原版 VisitResultVector (std::vector 派生类 → Vec + 自由函数)\n#[must_use]\npub fn compute_type_vector(v: &[VisitResult]) -> TypeVector {\n  v.iter().filter_map(|r| r.ty).collect()\n}\n\n`;
  }

  // Type 层级 7 类: 公共头 + 载荷
  const typeClasses = ["TypeBase", "Type", "TopType", "AbstractType", "BuiltinPointerType", "UnionType", "BitFieldStructType", "AggregateType", "StructType", "ClassType"];
  const classSigs = {};
  for (const name of typeClasses) {
    const blk = classBlock(name, th);
    if (!blk) { console.error(`缺失类 ${name}`); continue; }
    classSigs[name] = blk;
  }

  d += `/// Type 公共头: TypeBase(kind) + Type 基类字段 1:1\n#[derive(Debug, Clone)]\npub struct Type {\n  pub kind: TypeKind,\n  /// 子类型父链 (nullptr → None)\n  pub parent: Option<TypeId>,\n  pub id: usize,\n  pub specialized_from: MaybeSpecializationKey,\n  /// 按变体载荷 (虚函数分派由 TypeData 判别承担)\n  pub data: TypeData,\n}\n\n`;
  d += `impl Type {\n`;
  d += `  // 原 TypeBase::IsXxx 谓词 1:1\n  #[must_use]\n  pub fn is_top_type(&self) -> bool { self.kind == TypeKind::TopType }\n\n`;
  d += `  #[must_use]\n  pub fn is_abstract_type(&self) -> bool { self.kind == TypeKind::AbstractType }\n\n`;
  d += `  #[must_use]\n  pub fn is_builtin_pointer_type(&self) -> bool { self.kind == TypeKind::BuiltinPointerType }\n\n`;
  d += `  #[must_use]\n  pub fn is_union_type(&self) -> bool { self.kind == TypeKind::UnionType }\n\n`;
  d += `  #[must_use]\n  pub fn is_bit_field_struct_type(&self) -> bool { self.kind == TypeKind::BitFieldStructType }\n\n`;
  d += `  #[must_use]\n  pub fn is_struct_type(&self) -> bool { self.kind == TypeKind::StructType }\n\n`;
  d += `  #[must_use]\n  pub fn is_class_type(&self) -> bool { self.kind == TypeKind::ClassType }\n\n`;
  d += `  /// 原版 IsAggregateType\n  #[must_use]\n  pub fn is_aggregate_type(&self) -> bool { self.is_struct_type() || self.is_class_type() }\n\n`;
  d += `  /// 原版 IsVoid (IsAbstractName(VOID_TYPE_STRING), 语义填体)\n  pub fn is_void(&self) -> bool {\n    // TODO(填体): 抽象类型名比对 (含父链)\n    false\n  }\n\n`;
  d += `  /// 原版 IsNever / IsBool / IsConstexprBool / IsFloat32 / IsFloat64 同族谓词 (填体)\n  pub fn is_never(&self) -> bool {\n    // TODO(填体)\n    false\n  }\n\n`;
  d += `  /// 原版 IsConstexpr\n  pub fn is_constexpr(&self) -> bool {\n    // TODO(填体): 抽象类型 flags & kConstexpr\n    false\n  }\n\n`;
  d += `  /// 原版 IsTransient\n  pub fn is_transient(&self) -> bool {\n    // TODO(填体)\n    false\n  }\n\n`;
  d += `  /// 原版 ToString (错误消息渲染)\n  pub fn to_display_string(&self) -> Result<String> {\n    // TODO(填体)\n    Err(Error::Unimplemented)\n  }\n\n`;
  d += `  /// 原版 GetGeneratedTypeName\n  pub fn get_generated_type_name(&self) -> Result<String> {\n    // TODO(填体)\n    Err(Error::Unimplemented)\n  }\n\n`;
  d += `  /// 原版 GetGeneratedTNodeTypeName\n  pub fn get_generated_tnode_type_name(&self) -> Result<String> {\n    // TODO(填体)\n    Err(Error::Unimplemented)\n  }\n\n`;
  d += `  /// 原版 AddAlias\n  pub fn add_alias(&mut self, alias: String) {\n    // TODO(填体): aliases_ 存于 AbstractType 载荷\n    let _ = alias;\n  }\n`;
  d += `}\n\n`;

  d += `/// 按变体载荷 (C++ 派生类私有字段 1:1)\n#[derive(Debug, Clone)]\npub enum TypeData {\n`;
  d += `  Top(TopType),\n  Abstract(AbstractType),\n  BuiltinPointer(BuiltinPointerType),\n  Union(UnionType),\n  BitFieldStruct(BitFieldStructType),\n  Struct(StructType),\n  Class(ClassType),\n}\n\n`;

  d += `/// 原版 TopType 1:1\n#[derive(Debug, Clone, Default)]\npub struct TopType {\n  pub reason: String,\n  pub source_type: Option<TypeId>,\n}\n\n`;
  d += `/// 原版 AbstractType 1:1\n#[derive(Debug, Clone, Default)]\npub struct AbstractType {\n  pub flags: AbstractTypeFlags,\n  pub name: String,\n  pub generated_type: String,\n  pub non_constexpr_version: Option<TypeId>,\n  /// 原版 mutable std::set<std::string> aliases_\n  pub aliases: BTreeSet<String>,\n  /// 原版 mutable constexpr_version_\n  pub constexpr_version: Option<TypeId>,\n}\n\n`;
  d += `/// 原版 BuiltinPointerType 1:1\n#[derive(Debug, Clone, PartialEq, Eq, Hash)]\npub struct BuiltinPointerType {\n  pub parameter_types: TypeVector,\n  pub return_type: TypeId,\n  pub function_pointer_type_id: usize,\n}\n\n`;
  d += `/// 原版 UnionType 1:1 (std::set<Type*, TypeLess> → BTreeSet<TypeId>, 有序确定性)\n#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]\npub struct UnionType {\n  pub types: BTreeSet<TypeId>,\n}\n\n`;
  d += `/// 原版 BitFieldStructType 1:1 (decl 指针 → NodeId, namespace 指针 → DeclarableId)\n#[derive(Debug, Clone, Default)]\npub struct BitFieldStructType {\n  pub namespace_: Option<DeclarableId>,\n  pub decl: NodeId,\n  pub fields: Vec<BitField>,\n}\n\n`;
  d += `/// 原版 AggregateType 中间层公共字段 (is_finalized/fields/namespace/name/methods)\n#[derive(Debug, Clone, Default)]\npub struct AggregateCommon {\n  pub is_finalized: bool,\n  pub fields: Vec<Field>,\n  pub namespace_: Option<DeclarableId>,\n  pub name: String,\n  pub methods: Vec<DeclarableId>,\n}\n\n`;
  d += `/// 原版 StructType 1:1\n#[derive(Debug, Clone, Default)]\npub struct StructType {\n  pub aggregate: AggregateCommon,\n  pub decl: NodeId,\n  pub generated_type_name: String,\n}\n\n`;
  d += `/// 原版 ClassType 1:1\n#[derive(Debug, Clone, Default)]\npub struct ClassType {\n  pub aggregate: AggregateCommon,\n  pub header_size: usize,\n  pub size: ResidueClass,\n  pub flags: ClassFlags,\n  pub generates: String,\n  pub decl: NodeId,\n  pub alias: Option<DeclarableId>,\n  pub own_instance_type: Option<i32>,\n  pub instance_type_range: Option<(i32, i32)>,\n}\n\n`;

  d += sigBlock("types.cc", "types.cc 函数签名清单 (填体正交任务源)");
  // types.cc 核心自由函数的 Rust 签名 (类型系统语义层)
  d += `\n// ── types.cc 核心自由函数 Rust 签名 (oracle 显式传参, 填体) ──\n`;
  d += `use crate::type_oracle::TypeOracle;\n\n`;
  d += `/// 原版 IsAssignableFrom\npub fn is_assignable_from(oracle: &TypeOracle, to: TypeId, from: TypeId) -> bool {\n  // TODO(填体)\n  let _ = (oracle, to, from);\n  false\n}\n\n`;
  d += `/// 原版 LowerType\npub fn lower_type(oracle: &TypeOracle, type_: TypeId) -> Result<TypeVector> {\n  // TODO(填体)\n  let _ = (oracle, type_);\n  Err(Error::Unimplemented)\n}\n\n`;
  d += `/// 原版 LoweredSlotCount\npub fn lowered_slot_count(oracle: &TypeOracle, type_: TypeId) -> Result<usize> {\n  // TODO(填体)\n  let _ = (oracle, type_);\n  Err(Error::Unimplemented)\n}\n\n`;
  d += `/// 原版 LowerParameterTypes(TypeVector)\npub fn lower_parameter_types(oracle: &TypeOracle, parameters: &TypeVector) -> Result<TypeVector> {\n  // TODO(填体)\n  let _ = (oracle, parameters);\n  Err(Error::Unimplemented)\n}\n\n`;
  d += `/// 原版 SizeOf\npub fn size_of(oracle: &TypeOracle, type_: TypeId) -> Result<Option<(usize, String)>> {\n  // TODO(填体)\n  let _ = (oracle, type_);\n  Err(Error::Unimplemented)\n}\n\n`;
  d += `/// 原版 IsAnyUnsignedInteger\n#[must_use]\npub fn is_any_unsigned_integer(oracle: &TypeOracle, type_: TypeId) -> bool {\n  // TODO(填体)\n  let _ = (oracle, type_);\n  false\n}\n\n`;
  d += `/// 原版 IsAllowedAsBitField\n#[must_use]\npub fn is_allowed_as_bit_field(oracle: &TypeOracle, type_: TypeId) -> bool {\n  // TODO(填体)\n  let _ = (oracle, type_);\n  false\n}\n\n`;
  d += `/// 原版 IsPointerSizeIntegralType\n#[must_use]\npub fn is_pointer_size_integral_type(oracle: &TypeOracle, type_: TypeId) -> bool {\n  // TODO(填体)\n  let _ = (oracle, type_);\n  false\n}\n\n`;
  d += `/// 原版 Is32BitIntegralType\n#[must_use]\npub fn is_32_bit_integral_type(oracle: &TypeOracle, type_: TypeId) -> bool {\n  // TODO(填体)\n  let _ = (oracle, type_);\n  false\n}\n\n`;
  d += `/// 原版 SubtractType\npub fn subtract_type(oracle: &TypeOracle, a: TypeId, b: TypeId) -> Result<TypeId> {\n  // TODO(填体)\n  let _ = (oracle, a, b);\n  Err(Error::Unimplemented)\n}\n\n`;
  d += `/// 原版 Type::CommonSupertype\npub fn common_supertype(oracle: &TypeOracle, a: TypeId, b: TypeId) -> Result<TypeId> {\n  // TODO(填体)\n  let _ = (oracle, a, b);\n  Err(Error::Unimplemented)\n}\n\n`;
  d += `/// 原版 ProjectStructField\npub fn project_struct_field(oracle: &TypeOracle, structure: VisitResult, fieldname: &str) -> Result<VisitResult> {\n  // TODO(填体)\n  let _ = (oracle, structure, fieldname);\n  Err(Error::Unimplemented)\n}\n\n`;
  d += `/// 原版 ExtractSimpleFieldArraySize\npub fn extract_simple_field_array_size(\n  oracle: &TypeOracle,\n  class_type: TypeId,\n  array_size: NodeId,\n) -> Result<Option<NameAndType>> {\n  // TODO(填体)\n  let _ = (oracle, class_type, array_size);\n  Err(Error::Unimplemented)\n}\n`;

  // 填体提示: is_void/is_never/is_constexpr 等谓词消费 crate::constants 的
  // VOID_TYPE_STRING/NEVER_TYPE_STRING/BOOL_TYPE_STRING 等常量与 AbstractTypeFlags。

  writeSrc("v8-torque", "types.rs", d);
}

// ===========================================================================
// type-oracle.{h,cc} → type_oracle.rs
// ===========================================================================
function genTypeOracle() {
  const relH = "src/torque/type-oracle.h";
  const toh = noCmt(readV8(relH));
  let d = banner("src/torque/types.js", relH, "TypeOracle: 类型 arena + interning + 查询 (静态单例 → 结构体)");
  d += `
use crate::ast::NodeId;
use crate::constants::{
  AbstractTypeFlags, ARGUMENTS_TYPE_STRING, BOOL_TYPE_STRING, BUILTIN_POINTER_TYPE_STRING, CONSTEXPR_BOOL_TYPE_STRING,
  CONSTEXPR_INTPTR_TYPE_STRING, CONSTEXPR_INSTANCE_TYPE_TYPE_STRING, CONSTEXPR_STRING_TYPE_STRING,
  CONST_FLOAT64_TYPE_STRING, CONST_INT31_TYPE_STRING, CONST_INT32_TYPE_STRING,
  ClassFlags, CONST_STRING_TYPE_STRING, CONTEXT_TYPE_STRING, CPPHEAPPTR_TYPE_STRING, DISPATCH_HANDLE_TYPE_STRING,
  EXTERNALPTR_TYPE_STRING, FIXED_ARRAY_BASE_TYPE_STRING, FLOAT64_OR_UNDEFINED_OR_HOLE_TYPE_STRING,
  FLOAT64_TYPE_STRING, HEAP_OBJECT_TYPE_STRING, INTPTR_TYPE_STRING, INT64_TYPE_STRING,
  INTEGER_LITERAL_TYPE_STRING, JSANY_TYPE_STRING, JSOBJECT_TYPE_STRING, JS_FUNCTION_TYPE_STRING,
  MAP_TYPE_STRING, NATIVE_CONTEXT_TYPE_STRING, NEVER_TYPE_STRING, NO_CONTEXT_TYPE_STRING,
  NUMBER_TYPE_STRING, OBJECT_TYPE_STRING, PROTECTEDPTR_TYPE_STRING, RAWPTR_TYPE_STRING,
  SMI_TYPE_STRING, STATIC_ASSERT_MACRO_STRING, STRING_TYPE_STRING, STRONG_TAGGED_TYPE_STRING,
  TAGGED_TYPE_STRING, TAGGED_ZERO_PATTERN_TYPE_STRING, TRUSTEDPTR_TYPE_STRING, UINTPTR_TYPE_STRING,
  UNINITIALIZED_HEAP_OBJECT_TYPE_STRING, UNINITIALIZED_ITERATOR_TYPE_STRING, VOID_TYPE_STRING,
};
use crate::declarable::DeclarableId;
use crate::error::{Error, Result};
use crate::types::{
  AbstractType, AggregateCommon, BitFieldStructType, BuiltinPointerType, ClassType,
  MaybeSpecializationKey, TopType, Type, TypeData, TypeId, TypeKind, TypeVector, UnionType,
  StructType,
};
use crate::utils::ResidueClass;
use crate::utils::Deduplicator;

// C++ ContextualClass 静态单例 → Rust 由编译驱动持有 TypeOracle 实例, 方法去 static。
// 原版分散的 unique_ptr 存储 (nominal/aggregate/bit_field_struct/top_types) 统一为
// 单一 types: Vec<Type> arena, 分 kind 列表保留索引 (登记偏差, 语义等价)。
`;

  const blk = classBlock("TypeOracle", toh);
  const ms = [...blk.body.matchAll(/^\s*(?:static\s+)?[\w:<>,&*\s]+?\b(\w+)\(/gm)].map((m) => m[1]);
  const fs = memberFields(blk.body);

  d += `/// 原版 TypeOracle — 全部静态方法 → 实例方法 (${ms.length} 个方法签名见原版)\n#[derive(Debug, Default)]\npub struct TypeOracle {\n`;
  d += `  /// 原版 function_pointer_types_ 去重存储 (键含参数/返回类型, 权益下标即 TypeId)\n  function_pointer_types: Deduplicator<BuiltinPointerType>,\n`;
  d += `  /// 原版 all_builtin_pointer_types_\n  all_builtin_pointer_types: Vec<TypeId>,\n`;
  d += `  /// interning slot → TypeId 平行表\n  function_pointer_type_ids: Vec<TypeId>,\n`;
  d += `  /// 原版 union_types_ 去重存储\n  union_types: Deduplicator<UnionType>,\n`;
  d += `  /// 统一 Type arena (TypeId = 下标)\n  pub types: Vec<Type>,\n`;
  d += `  /// 原版 aggregate_types_ 索引\n  pub aggregate_types: Vec<TypeId>,\n`;
  d += `  /// 原版 bit_field_struct_types_ 索引\n  pub bit_field_struct_types: Vec<TypeId>,\n`;
  d += `  /// 原版 top_types_ 索引\n  pub top_types: Vec<TypeId>,\n`;
  d += `  /// 原版 generic_type_instantiation_namespaces_\n  pub generic_type_instantiation_namespaces: Vec<DeclarableId>,\n`;
  d += `  next_type_id: usize,\n`;
  d += `}\n\n`;

  d += `impl TypeOracle {\n`;
  // 基础构造方法 (真实现: arena push)
  d += `  fn push_type(&mut self, kind: TypeKind, parent: Option<TypeId>, specialized_from: MaybeSpecializationKey, data: TypeData) -> TypeId {\n    let id = self.next_type_id;\n    self.next_type_id += 1;\n    self.types.push(Type { kind, parent, id, specialized_from, data });\n    (self.types.len() - 1) as TypeId\n  }\n\n`;
  d += `  /// 原版 FreshTypeId\n  pub fn fresh_type_id(&mut self) -> usize {\n    self.next_type_id += 1;\n    self.next_type_id - 1\n  }\n\n`;
  d += `  /// 原版 GetAbstractType\n  pub fn get_abstract_type(\n    &mut self,\n    parent: Option<TypeId>,\n    name: String,\n    flags: AbstractTypeFlags,\n    generated: String,\n    non_constexpr_version: Option<TypeId>,\n    specialized_from: MaybeSpecializationKey,\n  ) -> TypeId {\n    let data = TypeData::Abstract(AbstractType {\n      flags,\n      name: name.clone(),\n      generated_type: generated,\n      non_constexpr_version,\n      aliases: Default::default(),\n      constexpr_version: None,\n    });\n    let t = self.push_type(TypeKind::AbstractType, parent, specialized_from, data);\n    // 原版: 非 constexpr 版本回填 constexpr_version_\n    if let Some(nc) = non_constexpr_version\n      && let Some(nc_type) = self.types.get_mut(nc as usize)\n      && let TypeData::Abstract(a) = &mut nc_type.data\n    {\n      a.constexpr_version = Some(t);\n    }\n    t\n  }\n\n`;
  d += `  /// 原版 GetTopType\n  pub fn get_top_type(&mut self, reason: String, source_type: TypeId) -> TypeId {\n    let data = TypeData::Top(TopType { reason, source_type: Some(source_type) });\n    let t = self.push_type(TypeKind::TopType, None, None, data);\n    self.top_types.push(t);\n    t\n  }\n\n`;
  d += `  /// 原版 GetStructType\n  pub fn get_struct_type(&mut self, decl: NodeId, specialized_from: MaybeSpecializationKey) -> TypeId {\n    let data = TypeData::Struct(StructType { aggregate: AggregateCommon::default(), decl, generated_type_name: String::new() });\n    let t = self.push_type(TypeKind::StructType, None, specialized_from, data);\n    self.aggregate_types.push(t);\n    t\n  }\n\n`;
  d += `  /// 原版 GetBitFieldStructType\n  pub fn get_bit_field_struct_type(&mut self, parent: Option<TypeId>, decl: NodeId) -> TypeId {\n    let data = TypeData::BitFieldStruct(BitFieldStructType { namespace_: None, decl, fields: Vec::new() });\n    let t = self.push_type(TypeKind::BitFieldStructType, parent, None, data);\n    self.bit_field_struct_types.push(t);\n    t\n  }\n\n`;
  d += `  /// 原版 GetClassType\n  pub fn get_class_type(\n    &mut self,\n    parent: Option<TypeId>,\n    name: String,\n    flags: ClassFlags,\n    generates: String,\n    decl: NodeId,\n    alias: Option<DeclarableId>,\n  ) -> TypeId {\n    let data = TypeData::Class(ClassType {\n      aggregate: AggregateCommon { name, ..Default::default() },\n      header_size: 0,\n      size: ResidueClass::unknown(),\n      flags,\n      generates,\n      decl,\n      alias,\n      own_instance_type: None,\n      instance_type_range: None,\n    });\n    let t = self.push_type(TypeKind::ClassType, parent, None, data);\n    self.aggregate_types.push(t);\n    t\n  }\n\n`;
  d += `  /// 原版 GetBuiltinPointerType (去重 interning, slot → TypeId 平行表)\n  pub fn get_builtin_pointer_type(&mut self, argument_types: TypeVector, return_type: TypeId) -> TypeId {\n    let key = BuiltinPointerType {\n      function_pointer_type_id: usize::MAX,\n      parameter_types: argument_types,\n      return_type,\n    };\n    let slot = self.function_pointer_types.intern(key);\n    if let Some(&t) = self.function_pointer_type_ids.get(slot) {\n      return t;\n    }\n    let data = TypeData::BuiltinPointer(self.function_pointer_types.get(slot).clone());\n    // TODO(填体): parent 设为 BuiltinPtr 抽象型 (Declarations 查询)\n    let t = self.push_type(TypeKind::BuiltinPointerType, None, None, data);\n    self.all_builtin_pointer_types.push(t);\n    self.function_pointer_type_ids.push(t);\n    t\n  }\n\n`;
  d += `  /// 原版 GetUnionType(UnionType) — 单成员折叠 + 去重\n  pub fn intern_union_type(&mut self, union: UnionType) -> Result<TypeId> {\n    if union.types.len() == 1 {\n      return Ok(*union.types.iter().next().expect(\"单成员已在长度校验后成立\"));\n    }\n    let _ = self.union_types.intern(union);\n    // TODO(填体): 去重键 → TypeId 映射\n    Err(Error::Unimplemented)\n  }\n\n`;
  d += `  /// 原版 GetUnionType(a, b)\n  pub fn get_union_type(&mut self, a: TypeId, b: TypeId) -> Result<TypeId> {\n    // TODO(填体): 子型折叠 + Extend + interning\n    let _ = (a, b);\n    Err(Error::Unimplemented)\n  }\n\n`;
  // 查询类占位
  d += `  /// 原版 GetGenericTypeInstance\n  pub fn get_generic_type_instance(&mut self, generic_type: DeclarableId, arg_types: TypeVector) -> Result<TypeId> {\n    // TODO(填体)\n    let _ = (generic_type, arg_types);\n    Err(Error::Unimplemented)\n  }\n\n`;
  d += `  /// 原版 MatchReferenceGeneric (返回 (is_const, 被引用类型))\n  pub fn match_reference_generic(&self, reference_type: TypeId) -> Result<Option<(bool, TypeId)>> {\n    // TODO(填体)\n    let _ = reference_type;\n    Err(Error::Unimplemented)\n  }\n\n`;
  d += `  /// 原版 ImplicitlyConvertableFrom\n  pub fn implicitly_convertable_from(&self, to: TypeId, from: TypeId) -> Result<Option<TypeId>> {\n    // TODO(填体)\n    let _ = (to, from);\n    Err(Error::Unimplemented)\n  }\n\n`;
  d += `  /// 原版 GetClasses (拓扑有序)\n  pub fn get_classes(&self) -> Result<Vec<TypeId>> {\n    // TODO(填体)\n    Err(Error::Unimplemented)\n  }\n\n`;
  d += `  /// 原版 FinalizeAggregateTypes\n  pub fn finalize_aggregate_types(&mut self) -> Result<()> {\n    // TODO(填体)\n    Err(Error::Unimplemented)\n  }\n\n`;
  d += `  /// 原版 CreateGenericTypeInstantiationNamespace\n  pub fn create_generic_type_instantiation_namespace(&mut self) -> Result<DeclarableId> {\n    // TODO(填体)\n    Err(Error::Unimplemented)\n  }\n\n`;
  d += `  /// 原版 GetBuiltinType(name) 私有查询 (Declarations 全局查找)\n  fn get_builtin_type(&self, name: &str) -> Result<TypeId> {\n    // TODO(填体): Declarations::LookupGlobalType\n    let _ = name;\n    Err(Error::Unimplemented)\n  }\n`;

  // 40+ 个 get_X_type 便捷方法 — 机械生成 (全部转发 get_builtin_type)
  const convenience = [
    ["arguments_type", "ARGUMENTS_TYPE_STRING"], ["bool_type", "BOOL_TYPE_STRING"],
    ["constexpr_bool_type", "CONSTEXPR_BOOL_TYPE_STRING"], ["constexpr_string_type", "CONSTEXPR_STRING_TYPE_STRING"],
    ["constexpr_int_ptr_type", "CONSTEXPR_INTPTR_TYPE_STRING"], ["constexpr_instance_type_type", "CONSTEXPR_INSTANCE_TYPE_TYPE_STRING"],
    ["void_type", "VOID_TYPE_STRING"], ["raw_ptr_type", "RAWPTR_TYPE_STRING"],
    ["external_pointer_type", "EXTERNALPTR_TYPE_STRING"], ["cpp_heap_pointer_type", "CPPHEAPPTR_TYPE_STRING"],
    ["trusted_pointer_type", "TRUSTEDPTR_TYPE_STRING"], ["protected_pointer_type", "PROTECTEDPTR_TYPE_STRING"],
    ["dispatch_handle_type", "DISPATCH_HANDLE_TYPE_STRING"], ["map_type", "MAP_TYPE_STRING"],
    ["object_type", "OBJECT_TYPE_STRING"], ["heap_object_type", "HEAP_OBJECT_TYPE_STRING"],
    ["tagged_zero_pattern_type", "TAGGED_ZERO_PATTERN_TYPE_STRING"], ["jsany_type", "JSANY_TYPE_STRING"],
    ["jsobject_type", "JSOBJECT_TYPE_STRING"], ["tagged_type", "TAGGED_TYPE_STRING"],
    ["strong_tagged_type", "STRONG_TAGGED_TYPE_STRING"], ["smi_type", "SMI_TYPE_STRING"],
    ["const_string_type", "CONST_STRING_TYPE_STRING"], ["string_type", "STRING_TYPE_STRING"],
    ["number_type", "NUMBER_TYPE_STRING"], ["int_ptr_type", "INTPTR_TYPE_STRING"],
    ["uint_ptr_type", "UINTPTR_TYPE_STRING"], ["int64_type", "INT64_TYPE_STRING"],
    ["float64_type", "FLOAT64_TYPE_STRING"], ["float64_or_undefined_or_hole_type", "FLOAT64_OR_UNDEFINED_OR_HOLE_TYPE_STRING"],
    ["const_float64_type", "CONST_FLOAT64_TYPE_STRING"], ["integer_literal_type", "INTEGER_LITERAL_TYPE_STRING"],
    ["never_type", "NEVER_TYPE_STRING"], ["const_int31_type", "CONST_INT31_TYPE_STRING"],
    ["const_int32_type", "CONST_INT32_TYPE_STRING"], ["context_type", "CONTEXT_TYPE_STRING"],
    ["no_context_type", "NO_CONTEXT_TYPE_STRING"], ["native_context_type", "NATIVE_CONTEXT_TYPE_STRING"],
    ["js_function_type", "JS_FUNCTION_TYPE_STRING"], ["uninitialized_iterator_type", "UNINITIALIZED_ITERATOR_TYPE_STRING"],
    ["fixed_array_base_type", "FIXED_ARRAY_BASE_TYPE_STRING"], ["static_assert_macro", "STATIC_ASSERT_MACRO_STRING"],
  ];
  for (const [m, c] of convenience) {
    d += `\n  /// 原版 Get${m.split("_").map((x) => x.charAt(0).toUpperCase() + x.slice(1)).join("")} → GetBuiltinType(${c})\n  pub fn ${m}(&self) -> Result<TypeId> {\n    self.get_builtin_type(${c})\n  }\n`;
  }
  // 特殊路径的几个
  d += `\n  /// 原版 GetUninitializedHeapType (torque_internal 限定名)\n  pub fn uninitialized_heap_type(&self) -> Result<TypeId> {\n    self.get_builtin_type(UNINITIALIZED_HEAP_OBJECT_TYPE_STRING)\n  }\n`;
  d += `\n  /// 原版 GetUint64Type\n  pub fn uint64_type(&self) -> Result<TypeId> {\n    self.get_builtin_type("uint64")\n  }\n`;
  d += `\n  /// 原版 GetUint32Type\n  pub fn uint32_type(&self) -> Result<TypeId> {\n    self.get_builtin_type("uint32")\n  }\n`;
  d += `\n  /// 原版 GetUint31Type\n  pub fn uint31_type(&self) -> Result<TypeId> {\n    self.get_builtin_type("uint31")\n  }\n`;
  d += `\n  /// 原版 GetInt32Type\n  pub fn int32_type(&self) -> Result<TypeId> {\n    self.get_builtin_type("int32")\n  }\n`;
  d += `\n  /// 原版 GetInt16Type / GetUint16Type / GetInt8Type / GetUint8Type / GetBintType 同式 (字面量名直查)\n  pub fn int16_type(&self) -> Result<TypeId> {\n    self.get_builtin_type("int16")\n  }\n\n  pub fn uint16_type(&self) -> Result<TypeId> {\n    self.get_builtin_type("uint16")\n  }\n\n  pub fn int8_type(&self) -> Result<TypeId> {\n    self.get_builtin_type("int8")\n  }\n\n  pub fn uint8_type(&self) -> Result<TypeId> {\n    self.get_builtin_type("uint8")\n  }\n\n  pub fn bint_type(&self) -> Result<TypeId> {\n    self.get_builtin_type("bint")\n  }\n\n  pub fn builtin_pointer_type_name(&self) -> Result<TypeId> {\n    self.get_builtin_type(BUILTIN_POINTER_TYPE_STRING)\n  }\n`;
  // 泛型查询组
  d += `\n  /// 原版 GetReferenceGeneric / GetConstReferenceGeneric / GetMutableReferenceGeneric\n  pub fn reference_generic(&self, is_const: bool) -> Result<DeclarableId> {\n    // TODO(填体): Declarations::LookupUniqueGenericType(torque_internal::xxx)\n    let _ = is_const;\n    Err(Error::Unimplemented)\n  }\n\n  /// 原版 GetMutableSliceGeneric / GetConstSliceGeneric\n  pub fn slice_generic(&self, is_const: bool) -> Result<DeclarableId> {\n    // TODO(填体)\n    let _ = is_const;\n    Err(Error::Unimplemented)\n  }\n\n  /// 原版 GetWeakGeneric / GetSmiTaggedGeneric / GetLazyGeneric (全局唯一泛型)\n  pub fn global_unique_generic(&self, name: &str) -> Result<DeclarableId> {\n    // TODO(填体)\n    let _ = name;\n    Err(Error::Unimplemented)\n  }\n\n`;
  d += `  /// 原版 GetReferenceType 族\n  pub fn reference_type(&self, referenced_type: TypeId, is_const: bool) -> Result<TypeId> {\n    // TODO(填体)\n    let _ = (referenced_type, is_const);\n    Err(Error::Unimplemented)\n  }\n\n`;
  d += `  /// 原版 GetMutableSliceType / GetConstSliceType\n  pub fn slice_type(&self, referenced_type: TypeId, is_const: bool) -> Result<TypeId> {\n    // TODO(填体)\n    let _ = (referenced_type, is_const);\n    Err(Error::Unimplemented)\n  }\n\n`;
  d += `  /// 原版 AllBuiltinPointerTypes\n  #[must_use]\n  pub fn all_builtin_pointer_types(&self) -> &[TypeId] {\n    &self.all_builtin_pointer_types\n  }\n`;
  d += `}\n\n`;

  d += `\n// 填体提示: implicitly_convertable_from 消费 declarations::FROM_CONSTEXPR_MACRO_NAME;\n// reference_generic/slice_generic/global_unique_generic 消费 MUTABLE_/CONST_\n// REFERENCE_TYPE_STRING、MUTABLE_/CONST_SLICE_TYPE_STRING、LAZY_/SMI_TAGGED_/WEAK_TYPE_STRING。\n`;

  d += sigBlock("type-oracle.cc", "签名清单");
  writeSrc("v8-torque", "type_oracle.rs", d);
}

// ===========================================================================
// type-visitor.{h,cc} → type_visitor.rs
// ===========================================================================
function genTypeVisitor() {
  const relH = "src/torque/type-visitor.h";
  let d = banner("src/torque/types.js", relH, "TypeVisitor: 类型计算访问器 (静态方法族 → 关联函数)");
  d += `
use crate::ast::NodeId;
use crate::declarable::DeclarableId;
use crate::error::{Error, Result};
use crate::type_oracle::TypeOracle;
use crate::types::{MaybeSpecializationKey, Signature, TypeId, TypeVector};

// 原版 TypeVisitor 全静态 → Rust 关联函数; ComputeType 多重载 (按 decl 类型分派)
// 合一为 compute_type_decl (按节点 kind 分派, 登记偏差); 上下文 Scope → DeclarableId。
`;

  d += `pub struct TypeVisitor;\n\n`;
  d += `impl TypeVisitor {\n`;
  d += `  /// 原版 ComputeType(TypeExpression*) — 分派入口\n  pub fn compute_type(oracle: &TypeOracle, type_expression: NodeId) -> Result<TypeId> {\n    // TODO(填体)\n    let _ = (oracle, type_expression);\n    Err(Error::Unimplemented)\n  }\n\n`;
  d += `  /// 原版 ComputeTypeVector\n  pub fn compute_type_vector(oracle: &TypeOracle, v: &[NodeId]) -> Result<TypeVector> {\n    // TODO(填体)\n    let _ = (oracle, v);\n    Err(Error::Unimplemented)\n  }\n\n`;
  d += `  /// 原版 ComputeType(decl, specialized_from, requester) — 五个 decl 重载合一\n  pub fn compute_type_decl(\n    oracle: &TypeOracle,\n    decl: NodeId,\n    specialized_from: MaybeSpecializationKey,\n    specialization_requester: Option<DeclarableId>,\n  ) -> Result<TypeId> {\n    // TODO(填体): AbstractType/TypeAlias/BitFieldStruct/Struct/Class 五分支\n    let _ = (oracle, decl, specialized_from, specialization_requester);\n    Err(Error::Unimplemented)\n  }\n\n`;
  d += `  /// 原版 VisitClassFieldsAndMethods\n  pub fn visit_class_fields_and_methods(oracle: &TypeOracle, class_type: TypeId, class_declaration: NodeId) -> Result<()> {\n    // TODO(填体)\n    let _ = (oracle, class_type, class_declaration);\n    Err(Error::Unimplemented)\n  }\n\n`;
  d += `  /// 原版 VisitStructMethods\n  pub fn visit_struct_methods(oracle: &TypeOracle, struct_type: TypeId, struct_declaration: NodeId) -> Result<()> {\n    // TODO(填体)\n    let _ = (oracle, struct_type, struct_declaration);\n    Err(Error::Unimplemented)\n  }\n\n`;
  d += `  /// 原版 MakeSignature\n  pub fn make_signature(declaration: NodeId) -> Result<Signature> {\n    // TODO(填体)\n    let _ = declaration;\n    Err(Error::Unimplemented)\n  }\n\n`;
  d += `  /// 原版 ComputeTypeForStructExpression (返回 Struct 或 BitFieldStruct)\n  pub fn compute_type_for_struct_expression(\n    oracle: &TypeOracle,\n    type_expression: NodeId,\n    term_argument_types: &TypeVector,\n  ) -> Result<TypeId> {\n    // TODO(填体)\n    let _ = (oracle, type_expression, term_argument_types);\n    Err(Error::Unimplemented)\n  }\n`;
  d += `}\n\n`;
  d += sigBlock("type-visitor.h", "接口签名清单") + "\n";
  d += sigBlock("type-visitor.cc", "签名清单");
  writeSrc("v8-torque", "type_visitor.rs", d);
}

// ===========================================================================
// 主流程
// ===========================================================================
genTypes();
genTypeOracle();
genTypeVisitor();
console.log("== v8-torque 类型层直出完毕: types.rs + type_oracle.rs + type_visitor.rs ==");
