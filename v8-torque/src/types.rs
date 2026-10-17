// 由 tool/src/torque/types.js 从 v8/src/torque/types.h 生成; 修订就地改, 复跑覆盖
// Type 体系全量 (7 kind + 载荷结构体 + 签名/字段/标签)

use std::collections::BTreeSet;

use crate::{
  ast::{AstNode, ClassFieldIndexInfo, NodeId},
  constants::{
    AbstractTypeFlag, AbstractTypeFlags, ClassFlag, ClassFlags, FieldSynchronization,
    NEVER_TYPE_STRING, VOID_TYPE_STRING,
  },
  declarable::DeclarableId,
  error::{Error, Result},
  global_context::GlobalContext,
  source_positions::SourcePosition,
  utils::{ResidueClass, StackRange},
};

// 继承树映射: TypeBase(kKind) → TypeKind 七变体; C++ const Type* → TypeId (arena 索引,
// 由 type_oracle.rs 的 TypeOracle 存储); TypeOracle 承担 interning (去重 + 指针同一性
// → TypeId 相等性)。AggregateType 中间层字段抽为 AggregateCommon, Struct/Class 持有。
/// TypeBase::Kind 1:1 (7 变体)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum TypeKind {
  TopType,
  AbstractType,
  BuiltinPointerType,
  UnionType,
  BitFieldStructType,
  StructType,
  ClassType,
}

/// 原版 using TypeVector = std::vector<const Type*>
pub type TypeVector = Vec<TypeId>;

/// 原版 using NameAndTypeVector = std::vector<NameAndType>
pub type NameAndTypeVector = Vec<NameAndType>;

/// 类型 arena 索引 (对应 C++ const Type*)
pub type TypeId = u32;

/// Type::HandleKind 1:1
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum TypeHandleKind {
  Indirect,
  Direct,
}

/// ObjectSlotKind 1:1
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum ObjectSlotKind {
  NoPointer,
  StrongPointer,
  MaybeObjectPointer,
  CustomWeakPointer,
}

/// ParameterMode 1:1
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum ParameterMode {
  ProcessImplicit,
  IgnoreImplicit,
}

/// StructType::ClassificationFlag 1:1
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum StructTypeClassificationFlag {
  Empty,
  StrongTagged,
  WeakTagged,
  Untagged,
}

/// 原版 Combine(a, b): 强/弱指针槽合并
#[must_use]
pub fn combine_object_slot_kind(a: ObjectSlotKind, b: ObjectSlotKind) -> Option<ObjectSlotKind> {
  if a == b {
    return Some(a);
  }
  let (lo, hi) = if (a as u8) < (b as u8) {
    (a, b)
  } else {
    (b, a)
  };
  if matches!(
    (lo, hi),
    (
      ObjectSlotKind::StrongPointer,
      ObjectSlotKind::MaybeObjectPointer
    )
  ) {
    return Some(ObjectSlotKind::MaybeObjectPointer);
  }
  None
}

/// TypeChecker 1:1
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct TypeChecker {
  pub r#type: String,
  pub weak_ref_to: String,
}

/// NameAndType 1:1
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct NameAndType {
  pub name: String,
  pub r#type: TypeId,
}

/// BitField 1:1
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct BitField {
  pub pos: SourcePosition,
  pub name_and_type: NameAndType,
  pub offset: i32,
  pub num_bits: i32,
}

/// LabelDefinition 1:1
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct LabelDefinition {
  pub name: String,
  pub parameters: NameAndTypeVector,
}

/// LabelDeclaration 1:1
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct LabelDeclaration {
  pub name: NodeId,
  pub types: TypeVector,
}

/// ParameterTypes 1:1
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ParameterTypes {
  pub types: TypeVector,
  pub var_args: bool,
}

/// 原版 SpecializationKey<T> (两实例合一: generic 指针 → DeclarableId)
#[derive(Debug, Clone, Default)]
pub struct SpecializationKey {
  pub generic: DeclarableId,
  pub specialized_types: TypeVector,
}

/// 原版 MaybeSpecializationKey
pub type MaybeSpecializationKey = Option<SpecializationKey>;

/// Field 1:1 (aggregate 指针 → TypeId)
#[derive(Debug, Clone, Default)]
pub struct Field {
  pub pos: SourcePosition,
  pub aggregate: TypeId,
  pub index: Option<ClassFieldIndexInfo>,
  pub name_and_type: NameAndType,
  /// 字节偏移; 未知 (索引字段之后 / 不支持堆上布局) 为 None
  pub offset: Option<usize>,
  pub custom_weak_marking: bool,
  pub const_qualified: bool,
  pub index_is_constant: bool,
  pub synchronization: FieldSynchronization,
}

/// Signature 1:1 (构造函数字段 + 默认值)
#[derive(Debug, Clone, Default)]
pub struct Signature {
  pub parameter_names: Vec<NodeId>,
  pub arguments_variable: Option<String>,
  pub parameter_types: ParameterTypes,
  pub implicit_count: usize,
  pub return_type: Option<TypeId>,
  pub labels: Vec<LabelDeclaration>,
  pub transitioning: bool,
}

impl Signature {
  /// 原版 types()
  #[must_use]
  pub fn types(&self) -> &TypeVector {
    &self.parameter_types.types
  }

  /// 原版 ExplicitCount
  #[must_use]
  pub fn explicit_count(&self) -> usize {
    self.parameter_types.types.len() - self.implicit_count
  }

  /// 原版 GetImplicitTypes
  #[must_use]
  pub fn get_implicit_types(&self) -> &[TypeId] {
    &self.parameter_types.types[..self.implicit_count]
  }

  /// 原版 GetExplicitTypes
  #[must_use]
  pub fn get_explicit_types(&self) -> &[TypeId] {
    &self.parameter_types.types[self.implicit_count..]
  }

  /// 原版 Signature::HasSameTypesAs (types.cc:1143-1167):
  /// mode 决定比较全参数还是显式参数; var_args / return_type / labels 逐项类型须一致
  /// (labels 只比 types, 不比 name, types.cc:1161-1165)
  #[must_use]
  pub fn has_same_types_as(&self, other: &Self, mode: ParameterMode) -> bool {
    // v8: src/torque/types.cc:1143-1167
    let (compare_types, other_compare_types) = match mode {
      ParameterMode::ProcessImplicit => (self.types().as_slice(), other.types().as_slice()),
      ParameterMode::IgnoreImplicit => (self.get_explicit_types(), other.get_explicit_types()),
    };
    if compare_types != other_compare_types
      || self.parameter_types.var_args != other.parameter_types.var_args
      || self.return_type != other.return_type
    {
      return false;
    }
    if self.labels.len() != other.labels.len() {
      return false;
    }
    self
      .labels
      .iter()
      .zip(other.labels.iter())
      .all(|(l, o)| l.types == o.types)
  }

  /// 原版 Signature::HasContextParameter (types.cc:1169-1178):
  /// types()[0] 与 Context/NoContext 类型同一 (指针同一 → TypeId 同一)。
  /// 依赖面台账: context_type/no_context_type 走 get_builtin_type
  /// (Declarations 全局名查询, 见 type_oracle.rs get_builtin_type 台账注),
  /// 接线波前运行期传播 Err。
  pub fn has_context_parameter(&self, oracle: &TypeOracle) -> Result<bool> {
    // v8: src/torque/types.cc:1169-1178
    first_type_is_context(oracle, self.types())
  }
}

/// 原版 BuiltinPointerType::HasContextParameter (types.cc:1180-1182)
impl BuiltinPointerType {
  pub fn has_context_parameter(&self, oracle: &TypeOracle) -> Result<bool> {
    // v8: src/torque/types.cc:1180-1182
    first_type_is_context(oracle, &self.parameter_types)
  }
}

/// 原版 FirstTypeIsContext (types.cc:1169-1174, 匿名 ns): 首参数为 Context/NoContext
fn first_type_is_context(oracle: &TypeOracle, parameter_types: &[TypeId]) -> Result<bool> {
  // v8: src/torque/types.cc:1169-1174
  let Some(&first) = parameter_types.first() else {
    return Ok(false);
  };
  Ok(first == oracle.context_type()? || first == oracle.no_context_type()?)
}

/// VisitResult 1:1: 类型 + (constexpr 值 | 栈区间) 二选一
#[derive(Debug, Clone, Default)]
pub struct VisitResult {
  pub ty: Option<TypeId>,
  pub constexpr_value: Option<String>,
  pub stack_range: Option<StackRange>,
}

impl VisitResult {
  /// 原版 IsOnStack
  #[must_use]
  pub fn is_on_stack(&self) -> bool {
    self.stack_range.is_some()
  }

  /// 原版 VisitResult::NeverResult (types.cc:1251-1255): type_ = GetNeverType。
  /// 依赖面台账: never_type 走 get_builtin_type (type_oracle.rs 台账注),
  /// 接线波前运行期传播 Err。
  pub fn never_result(oracle: &TypeOracle) -> Result<Self> {
    // v8: src/torque/types.cc:1251-1255
    Ok(Self {
      ty: Some(oracle.never_type()?),
      constexpr_value: None,
      stack_range: None,
    })
  }

  /// 原版 VisitResult::TopTypeResult (types.cc:1257-1262):
  /// type_ = TypeOracle::GetTopType(reason, from_type) (每次新建, 不去重)
  #[must_use]
  pub fn top_type_result(oracle: &mut TypeOracle, top_reason: String, from_type: TypeId) -> Self {
    // v8: src/torque/types.cc:1257-1262 + type-oracle.h:160-165 GetTopType
    Self {
      ty: Some(oracle.get_top_type(top_reason, from_type)),
      constexpr_value: None,
      stack_range: None,
    }
  }
}

/// 原版 VisitResultVector (std::vector 派生类 → Vec + 自由函数)
#[must_use]
pub fn compute_type_vector(v: &[VisitResult]) -> TypeVector {
  v.iter().filter_map(|r| r.ty).collect()
}

/// Type 公共头: TypeBase(kind) + Type 基类字段 1:1
/// (aliases_ 在原版即 Type 基类成员, mutable set → BTreeSet, types.cc:24 拷贝构造不复制)
#[derive(Debug, Clone)]
pub struct Type {
  pub kind: TypeKind,
  /// 子类型父链 (nullptr → None)
  pub parent: Option<TypeId>,
  pub id: usize,
  pub specialized_from: MaybeSpecializationKey,
  /// 原版 mutable std::set<std::string> aliases_ (Type 基类成员)
  pub aliases: BTreeSet<String>,
  /// 按变体载荷 (虚函数分派由 TypeData 判别承担)
  pub data: TypeData,
}

impl Type {
  // 原 TypeBase::IsXxx 谓词 1:1
  #[must_use]
  pub fn is_top_type(&self) -> bool {
    self.kind == TypeKind::TopType
  }

  #[must_use]
  pub fn is_abstract_type(&self) -> bool {
    self.kind == TypeKind::AbstractType
  }

  #[must_use]
  pub fn is_builtin_pointer_type(&self) -> bool {
    self.kind == TypeKind::BuiltinPointerType
  }

  #[must_use]
  pub fn is_union_type(&self) -> bool {
    self.kind == TypeKind::UnionType
  }

  #[must_use]
  pub fn is_bit_field_struct_type(&self) -> bool {
    self.kind == TypeKind::BitFieldStructType
  }

  #[must_use]
  pub fn is_struct_type(&self) -> bool {
    self.kind == TypeKind::StructType
  }

  #[must_use]
  pub fn is_class_type(&self) -> bool {
    self.kind == TypeKind::ClassType
  }

  /// 原版 IsAggregateType
  #[must_use]
  pub fn is_aggregate_type(&self) -> bool {
    self.is_struct_type() || self.is_class_type()
  }

  /// 原版 Type::StructSupertype (types.cc:136-143): 自父链 (含自身) 首个 StructType。
  /// arena 不变量: Type.id 与 types 下标同步 (push_type 同值双写)
  #[must_use]
  pub fn struct_supertype(&self, oracle: &TypeOracle) -> Option<TypeId> {
    // v8: src/torque/types.cc:136-143
    let mut current = Some(self.id as TypeId);
    while let Some(t) = current {
      let ty = &oracle.types[t as usize];
      if ty.is_struct_type() {
        return Some(t);
      }
      current = ty.parent;
    }
    None
  }

  /// 原版 Type::IsVoidOrNever (types.h:131)
  #[must_use]
  pub fn is_void_or_never(&self) -> bool {
    self.is_void() || self.is_never()
  }

  /// 原版 IsVoid (IsAbstractName(VOID_TYPE_STRING))
  #[must_use]
  pub fn is_void(&self) -> bool {
    self.is_abstract_name(VOID_TYPE_STRING)
  }

  /// 原版 IsNever (IsAbstractName(NEVER_TYPE_STRING), 仅比对自身抽象名, types.cc:178-181)
  #[must_use]
  pub fn is_never(&self) -> bool {
    self.is_abstract_name(NEVER_TYPE_STRING)
  }

  /// 原版 IsConstexpr (types.h:136-139 / 275-279 / 445): Abstract 查 flags 位,
  /// Union 沿父链, 其余 false
  #[must_use]
  pub fn is_constexpr(&self, oracle: &TypeOracle) -> bool {
    match &self.data {
      TypeData::Abstract(a) => a.flags & AbstractTypeFlag::Constexpr as u32 != 0,
      TypeData::Union(_) => self
        .parent
        .map(|p| oracle.types[p as usize].is_constexpr(oracle))
        .unwrap_or(false),
      _ => false,
    }
  }

  /// 原版 IsTransient (types.h:140 / 320-322 / 436-443 / 689)
  #[must_use]
  pub fn is_transient(&self, oracle: &TypeOracle) -> bool {
    match &self.data {
      TypeData::Abstract(a) => a.flags & AbstractTypeFlag::Transient as u32 != 0,
      TypeData::Union(u) => u
        .types
        .iter()
        .any(|&m| oracle.types[m as usize].is_transient(oracle)),
      TypeData::Class(c) => c.flags & ClassFlag::Transient as u32 != 0,
      _ => false,
    }
  }

  /// 原版 NonConstexprVersion (types.h:141 / 281-287 / 447-451):
  /// 载荷优先, 非 constexpr 取自身, constexpr 沿父链
  #[must_use]
  pub fn non_constexpr_version(&self, oracle: &TypeOracle) -> Option<TypeId> {
    match &self.data {
      TypeData::Abstract(a) => a.non_constexpr_version.or_else(|| {
        if self.is_constexpr(oracle) {
          self
            .parent
            .and_then(|p| oracle.types[p as usize].non_constexpr_version(oracle))
        } else {
          Some(self.id as TypeId)
        }
      }),
      TypeData::Union(_) => {
        if self.is_constexpr(oracle) {
          self
            .parent
            .and_then(|p| oracle.types[p as usize].non_constexpr_version(oracle))
        } else {
          Some(self.id as TypeId)
        }
      }
      _ => Some(self.id as TypeId),
    }
  }

  /// 原版 IsAbstractName (types.cc:178-181): 仅自身抽象名比对, 不沿父链
  #[must_use]
  pub fn is_abstract_name(&self, name: &str) -> bool {
    matches!(&self.data, TypeData::Abstract(a) if a.name == name)
  }

  /// 原版 ToString (types.cc:38-57): 空别名走 ComputeName(ToExplicitString);
  /// 单别名直出; 多别名 "a0 (aka. a1, a2)"
  #[must_use]
  pub fn to_display_string(&self, cx: &GlobalContext, oracle: &TypeOracle) -> String {
    if self.aliases.is_empty() {
      return compute_name(
        cx,
        oracle,
        &self.to_explicit_string(cx, oracle),
        &self.specialized_from,
      );
    }
    if self.aliases.len() == 1 {
      return self.aliases.iter().next().cloned().unwrap_or_default();
    }
    let mut result = String::new();
    for (i, alias) in self.aliases.iter().enumerate() {
      match i {
        0 => result.push_str(alias),
        1 => {
          result.push_str(" (aka. ");
          result.push_str(alias);
        }
        _ => {
          result.push_str(", ");
          result.push_str(alias);
        }
      }
    }
    result.push(')');
    result
  }

  /// 原版 SimpleName (types.cc:59-73): SimpleNameImpl + 特化实参逐个 "_" 拼接;
  /// 有别名取首别名
  #[must_use]
  pub fn simple_name(&self, cx: &GlobalContext, oracle: &TypeOracle) -> String {
    if self.aliases.is_empty() {
      let mut result = self.simple_name_impl(cx, oracle);
      if let Some(key) = &self.specialized_from {
        for t in &key.specialized_types {
          result.push('_');
          result.push_str(&oracle.types[*t as usize].simple_name(cx, oracle));
        }
      }
      return result;
    }
    self.aliases.iter().next().cloned().unwrap_or_default()
  }

  /// 原版 AddAlias (types.h:150)
  pub fn add_alias(&mut self, alias: String) {
    self.aliases.insert(alias);
  }

  /// 原版 GetGeneratedTypeName (types.cc:183-190): 空名/TNode<> → ReportError
  pub fn get_generated_type_name(&self, cx: &GlobalContext, oracle: &TypeOracle) -> Result<String> {
    let result = self.get_generated_type_name_impl(cx, oracle)?;
    if result.is_empty() || result == "TNode<>" {
      return Err(Error::Message(format!(
        "Generated type is required for type '{}'. Use 'generates' clause in definition.",
        self.to_display_string(cx, oracle)
      )));
    }
    Ok(result)
  }

  /// 原版 GetGeneratedTNodeTypeName (types.cc:192-199): 空名或 constexpr → ReportError
  pub fn get_generated_tnode_type_name(
    &self,
    cx: &GlobalContext,
    oracle: &TypeOracle,
  ) -> Result<String> {
    let result = self.get_generated_tnode_type_name_impl(cx, oracle)?;
    if result.is_empty() || self.is_constexpr(oracle) {
      return Err(Error::Message(format!(
        "Generated TNode type is required for type '{}'. Use 'generates' clause in definition.",
        self.to_display_string(cx, oracle)
      )));
    }
    Ok(result)
  }

  /// 原版 ToExplicitString 虚分派 (各 kind 覆写, types.h:180)
  #[must_use]
  fn to_explicit_string(&self, cx: &GlobalContext, oracle: &TypeOracle) -> String {
    match &self.data {
      TypeData::Top(t) => {
        format!(
          "inaccessible {}",
          t.source_type
            .map(|s| oracle.types[s as usize].to_display_string(cx, oracle))
            .unwrap_or_default()
        )
      }
      TypeData::Abstract(a) => a.name.clone(),
      TypeData::BuiltinPointer(b) => {
        let params = b
          .parameter_types
          .iter()
          .map(|&t| oracle.types[t as usize].to_display_string(cx, oracle))
          .collect::<Vec<_>>()
          .join(", ");
        format!(
          "builtin ({params}) => {}",
          oracle.types[b.return_type as usize].to_display_string(cx, oracle)
        )
      }
      TypeData::Union(u) => {
        let members = u
          .types
          .iter()
          .map(|&t| oracle.types[t as usize].to_display_string(cx, oracle))
          .collect::<Vec<_>>()
          .join(" | ");
        format!("({members})")
      }
      TypeData::BitFieldStruct(b) => format!("bitfield struct {}", b.name(cx)),
      TypeData::Struct(s) => format!("struct {}", s.decl_name(cx)),
      TypeData::Class(c) => format!("class {}", c.aggregate.name),
    }
  }

  /// 原版 SimpleNameImpl 虚分派 (各 kind 覆写, types.cc:59-73 引用点)
  #[must_use]
  fn simple_name_impl(&self, cx: &GlobalContext, oracle: &TypeOracle) -> String {
    match &self.data {
      TypeData::Top(_) => "TopType".to_owned(),
      TypeData::Abstract(_) => {
        // constexpr: "constexpr_" + 非 constexpr版本 SimpleName (types.h:309-318);
        // 无非 constexpr 版本时原版 ReportError, 此处以名兜底 (数据由 oracle 构造保证)
        if self.is_constexpr(oracle) {
          let base = self
            .non_constexpr_version(oracle)
            .map(|nc| oracle.types[nc as usize].simple_name(cx, oracle))
            .unwrap_or_else(|| self.abstract_name().unwrap_or_default().to_owned());
          return format!("constexpr_{base}");
        }
        self.abstract_name().unwrap_or_default().to_owned()
      }
      TypeData::BuiltinPointer(b) => {
        let mut result = String::from("BuiltinPointer");
        for &t in &b.parameter_types {
          result.push('_');
          result.push_str(&oracle.types[t as usize].simple_name(cx, oracle));
        }
        result.push('_');
        result.push_str(&oracle.types[b.return_type as usize].simple_name(cx, oracle));
        result
      }
      TypeData::Union(u) => u
        .types
        .iter()
        .map(|&t| oracle.types[t as usize].simple_name(cx, oracle))
        .collect::<Vec<_>>()
        .join("_OR_"),
      TypeData::BitFieldStruct(b) => b.name(cx).to_owned(),
      TypeData::Struct(s) => s.decl_name(cx).to_owned(),
      TypeData::Class(c) => c.aggregate.name.clone(),
    }
  }

  /// 原版 GetGeneratedTypeNameImpl 虚分派; Top/BuiltinPointer/Union 原版 UNREACHABLE → Err
  fn get_generated_type_name_impl(
    &self,
    cx: &GlobalContext,
    oracle: &TypeOracle,
  ) -> Result<String> {
    match &self.data {
      TypeData::Abstract(a) => {
        // Lazy<T> 特例 (std::function<TNode<T>()>) 依赖泛型匹配, 未填体, 走常轨
        if a.generated_type.is_empty() {
          return self
            .parent
            .map(|p| oracle.types[p as usize].get_generated_type_name(cx, oracle))
            .unwrap_or_else(|| {
              Err(Error::Message(
                "abstract type without parent has no generated type name".into(),
              ))
            });
        }
        if self.is_constexpr(oracle) {
          Ok(a.generated_type.clone())
        } else {
          Ok(format!("TNode<{}>", a.generated_type))
        }
      }
      TypeData::Struct(s) => Ok(s.generated_type_name.clone()),
      TypeData::Class(c) => {
        if self.is_constexpr(oracle) {
          Ok(c.generates.clone())
        } else {
          Ok(format!("TNode<{}>", c.generates))
        }
      }
      TypeData::BitFieldStruct(_) => self
        .parent
        .map(|p| oracle.types[p as usize].get_generated_type_name(cx, oracle))
        .unwrap_or_else(|| {
          Err(Error::Message(
            "bitfield struct without parent has no generated type name".into(),
          ))
        }),
      _ => Err(Error::Message(
        "this type kind has no generated type name".into(),
      )),
    }
  }

  /// 原版 GetGeneratedTNodeTypeNameImpl 虚分派; Aggregate 原版 UNREACHABLE → Err
  fn get_generated_tnode_type_name_impl(
    &self,
    cx: &GlobalContext,
    oracle: &TypeOracle,
  ) -> Result<String> {
    match &self.data {
      TypeData::Top(t) => t
        .source_type
        .map(|s| oracle.types[s as usize].get_generated_tnode_type_name(cx, oracle))
        .unwrap_or_else(|| {
          Err(Error::Message(
            "top type without source has no TNode name".into(),
          ))
        }),
      TypeData::Abstract(a) => {
        if a.generated_type.is_empty() {
          return self
            .parent
            .map(|p| oracle.types[p as usize].get_generated_tnode_type_name(cx, oracle))
            .unwrap_or_else(|| {
              Err(Error::Message(
                "abstract type without parent has no TNode name".into(),
              ))
            });
        }
        Ok(a.generated_type.clone())
      }
      TypeData::Class(c) => Ok(c.generates.clone()),
      TypeData::BitFieldStruct(_) => self
        .parent
        .map(|p| oracle.types[p as usize].get_generated_tnode_type_name(cx, oracle))
        .unwrap_or_else(|| {
          Err(Error::Message(
            "bitfield struct without parent has no TNode name".into(),
          ))
        }),
      _ => Err(Error::Message(
        "this type kind has no generated TNode type name".into(),
      )),
    }
  }

  /// Abstract 载荷名 (仅 Abstract kind 有值)
  fn abstract_name(&self) -> Option<&str> {
    match &self.data {
      TypeData::Abstract(a) => Some(&a.name),
      _ => None,
    }
  }
}

/// 按变体载荷 (C++ 派生类私有字段 1:1)
#[derive(Debug, Clone)]
pub enum TypeData {
  Top(TopType),
  Abstract(AbstractType),
  BuiltinPointer(BuiltinPointerType),
  Union(UnionType),
  BitFieldStruct(BitFieldStructType),
  Struct(StructType),
  Class(ClassType),
}

/// 原版 TopType 1:1
#[derive(Debug, Clone, Default)]
pub struct TopType {
  pub reason: String,
  pub source_type: Option<TypeId>,
}

/// 原版 AbstractType 1:1 (aliases_ 上移 Type 基类)
#[derive(Debug, Clone, Default)]
pub struct AbstractType {
  pub flags: AbstractTypeFlags,
  pub name: String,
  pub generated_type: String,
  pub non_constexpr_version: Option<TypeId>,
  /// 原版 mutable constexpr_version_
  pub constexpr_version: Option<TypeId>,
}

/// 原版 BuiltinPointerType 1:1
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct BuiltinPointerType {
  pub parameter_types: TypeVector,
  pub return_type: TypeId,
  pub function_pointer_type_id: usize,
}

/// 原版 UnionType 1:1 (std::set<Type*, TypeLess> → BTreeSet<TypeId>, 有序确定性)
#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
pub struct UnionType {
  pub types: BTreeSet<TypeId>,
}

/// 原版 BitFieldStructType 1:1 (decl 指针 → NodeId, namespace 指针 → DeclarableId)
#[derive(Debug, Clone, Default)]
pub struct BitFieldStructType {
  pub namespace_: Option<DeclarableId>,
  pub decl: NodeId,
  pub fields: Vec<BitField>,
}

impl BitFieldStructType {
  /// 原版 name() (types.h:530): decl->name->value
  #[must_use]
  pub fn name<'a>(&self, cx: &'a GlobalContext) -> &'a str {
    let AstNode::BitFieldStructDeclaration(d) = &cx.ast.node(self.decl).1 else {
      return "";
    };
    let AstNode::Identifier(id) = &cx.ast.node(d.name).1 else {
      return "";
    };
    &id.value
  }
}

/// 原版 AggregateType 中间层公共字段 (is_finalized/fields/namespace/name/methods)
#[derive(Debug, Clone, Default)]
pub struct AggregateCommon {
  pub is_finalized: bool,
  pub fields: Vec<Field>,
  pub namespace_: Option<DeclarableId>,
  pub name: String,
  pub methods: Vec<DeclarableId>,
}

/// 原版 StructType 1:1
#[derive(Debug, Clone, Default)]
pub struct StructType {
  pub aggregate: AggregateCommon,
  pub decl: NodeId,
  pub generated_type_name: String,
}

impl StructType {
  /// 载荷级取名: decl->name->value (构造期 name_ 同源, types.cc:507;
  /// SimpleNameImpl/ToExplicitString 覆写点 types.cc:573/602)
  #[must_use]
  pub fn decl_name<'a>(&self, cx: &'a GlobalContext) -> &'a str {
    let AstNode::StructDeclaration(d) = &cx.ast.node(self.decl).1 else {
      return "";
    };
    let AstNode::Identifier(id) = &cx.ast.node(d.name).1 else {
      return "";
    };
    &id.value
  }
}

/// 原版 ClassType 1:1
#[derive(Debug, Clone, Default)]
pub struct ClassType {
  pub aggregate: AggregateCommon,
  pub header_size: usize,
  pub size: ResidueClass,
  pub flags: ClassFlags,
  pub generates: String,
  pub decl: NodeId,
  pub alias: Option<DeclarableId>,
  pub own_instance_type: Option<i32>,
  pub instance_type_range: Option<(i32, i32)>,
}

// ── 原版 src/torque/types.cc ──
// L24: Type::Type(const Type& other) V8_NOEXCEPT : TypeBase(other), parent_(other.parent_), aliases_(), id_(TypeOracle::FreshTypeId()), constexpr_version_(other.constexpr_version_)
// L30: Type::Type(TypeBase::Kind kind, const Type* parent, MaybeSpecializationKey specialized_from) : TypeBase(kind), parent_(parent), id_(TypeOracle::FreshTypeId()), specialized_from_(specialized_from), constexpr_version_(nullptr)
// L38: std::string Type::ToString() const
// L59: std::string Type::SimpleName() const
// L73: std::string Type::GetHandleTypeName(HandleKind kind, const std::string& type_name) const
// L86: std::string Type::HandlifiedCppTypeName(HandleKind kind) const
// L95: std::string Type::TagglifiedCppTypeName() const
// L104: bool Type::IsSubtypeOf(const Type* supertype) const
// L118: std::string Type::GetConstexprGeneratedTypeName() const
// L127: std::optional<const ClassType*> Type::ClassSupertype() const
// L136: std::optional<const StructType*> Type::StructSupertype() const
// L145: std::optional<const AggregateType*> Type::AggregateSupertype() const
// L155: const Type* Type::CommonSupertype(const Type* a, const Type* b)
// L170: int Type::Depth() const
// L178: bool Type::IsAbstractName(const std::string& name) const
// L183: std::string Type::GetGeneratedTypeName() const
// L192: std::string Type::GetGeneratedTNodeTypeName() const
// L201: std::string AbstractType::GetGeneratedTypeNameImpl() const
// L218: std::string AbstractType::GetGeneratedTNodeTypeNameImpl() const
// L223: std::vector<TypeChecker> AbstractType::GetTypeCheckers() const
// L241: std::string BuiltinPointerType::ToExplicitString() const
// L249: std::string BuiltinPointerType::SimpleNameImpl() const
// L259: std::string UnionType::ToExplicitString() const
// L274: std::string UnionType::SimpleNameImpl() const
// L288: void UnionType::InsertGeneratedTNodeTypeName(std::set<std::string>& names, const Type* t)
// L299: std::string UnionType::GetGeneratedTNodeTypeNameImpl() const
// L332: std::string UnionType::GetRuntimeType() const
// L342: void UnionType::InsertConstexprGeneratedTypeName(std::set<std::string>& names, const Type* t)
// L353: std::string UnionType::GetConstexprGeneratedTypeName() const
// L388: std::string UnionType::GetDebugType() const
// L390: void UnionType::RecomputeParent()
// L402: void UnionType::Subtract(const Type* t)
// L414: const Type* SubtractType(const Type* a, const Type* b)
// L420: std::string BitFieldStructType::ToExplicitString() const
// L424: const BitField& BitFieldStructType::LookupField(const std::string& name) const
// L433: void AggregateType::CheckForDuplicateFields() const
// L462: std::vector<const AggregateType*> AggregateType::GetHierarchy() const
// L477: bool AggregateType::HasField(const std::string& name) const
// L490: const Field& AggregateType::LookupFieldInternal(const std::string& name) const
// L502: const Field& AggregateType::LookupField(const std::string& name) const
// L507: StructType::StructType(Namespace* nspace, const StructDeclaration* decl, MaybeSpecializationKey specialized_from) : AggregateType(Kind::kStructType, nullptr, nspace, decl->name->value, specialized_from), decl_(decl)
// L520: std::string StructType::GetGeneratedTypeNameImpl() const
// L524: size_t StructType::PackedSize() const
// L532: StructType::Classification StructType::ClassifyContents() const
// L550: std::string Type::ComputeName(const std::string& basename, MaybeSpecializationKey specialized_from)
// L573: std::string StructType::SimpleNameImpl() const
// L576: std::optional<const Type*> Type::MatchUnaryGeneric(const Type* type, GenericType* generic)
// L589: std::vector<Method*> AggregateType::Methods(const std::string& name) const
// L602: std::string StructType::ToExplicitString() const
// L604: void StructType::Finalize() const
// L615: ClassType::ClassType(const Type* parent, Namespace* nspace, const std::string& name, ClassFlags flags, const std::string& generates, const ClassDeclaration* decl, const TypeAlias* alias) : AggregateType(Kind::kClassType, parent, nspace, name), size_(ResidueClass::Unknown()), flags_(flags), generates_(generates), decl_(decl), alias_(alias)
// L626: std::string ClassType::GetGeneratedTNodeTypeNameImpl() const
// L630: std::string ClassType::GetGeneratedTypeNameImpl() const
// L635: std::string ClassType::ToExplicitString() const
// L637: bool ClassType::AllowInstantiation() const
// L641: void ClassType::Finalize() const
// L651: std::vector<Field> ClassType::ComputeAllFields() const
// L662: std::vector<Field> ClassType::ComputeHeaderFields() const
// L674: std::vector<Field> ClassType::ComputeArrayFields() const
// L688: void ClassType::InitializeInstanceTypes( std::optional<int> own, std::optional<std::pair<int, int>> range) const
// L696: std::optional<int> ClassType::OwnInstanceType() const
// L701: std::optional<std::pair<int, int>> ClassType::InstanceTypeRange() const
// L707: void ComputeSlotKindsHelper(std::vector<ObjectSlotKind>* slots, size_t start_offset, const std::vector<Field>& fields)
// L750: std::vector<ObjectSlotKind> ClassType::ComputeHeaderSlotKinds() const
// L760: std::optional<ObjectSlotKind> ClassType::ComputeArraySlotKind() const
// L774: bool ClassType::HasNoPointerSlotsExceptMap() const
// L788: bool ClassType::HasDynamicIndexedFieldsIncludingInParents() const
// L798: const Field* ClassType::GetFieldPreceding(size_t field_index) const
// L808: const ClassType* ClassType::GetClassDeclaringField(const Field& f) const
// L815: std::string ClassType::GetSliceMacroName(const Field& field) const
// L821: void ClassType::GenerateAccessors()
// L929: void ClassType::GenerateSliceAccessor(size_t field_index)
// L1062: bool ClassType::HasStaticSize() const
// L1067: SourceId ClassType::AttributedToFile() const
// L1076: void PrintSignature(std::ostream& os, const Signature& sig, bool with_names)
// L1109: std::ostream& operator<<(std::ostream& os, const NameAndType& name_and_type)
// L1116: std::ostream& operator<<(std::ostream& os, const Field& field)
// L1124: std::ostream& operator<<(std::ostream& os, const Signature& sig)
// L1129: std::ostream& operator<<(std::ostream& os, const TypeVector& types)
// L1134: std::ostream& operator<<(std::ostream& os, const ParameterTypes& p)
// L1143: bool Signature::HasSameTypesAs(const Signature& other, ParameterMode mode) const
// L1169: bool FirstTypeIsContext(const std::vector<const Type*>& parameter_types)
// L1176: bool Signature::HasContextParameter() const
// L1180: bool BuiltinPointerType::HasContextParameter() const
// L1184: bool IsAssignableFrom(const Type* to, const Type* from)
// L1190: bool operator<(const Type& a, const Type& b)
// L1192: VisitResult ProjectStructField(VisitResult structure, const std::string& fieldname)
// L1212: void AppendLoweredTypes(const Type* type, std::vector<const Type*>* result)
// L1225: TypeVector LowerType(const Type* type)
// L1231: size_t LoweredSlotCount(const Type* type)
// L1233: TypeVector LowerParameterTypes(const TypeVector& parameters)
// L1241: TypeVector LowerParameterTypes(const ParameterTypes& parameter_types, size_t arg_count)
// L1251: VisitResult VisitResult::NeverResult()
// L1257: VisitResult VisitResult::TopTypeResult(std::string top_reason, const Type* from_type)
// L1264: std::tuple<size_t, std::string> Field::GetFieldSizeInformation() const
// L1274: size_t Type::AlignmentLog2() const
// L1279: size_t AbstractType::AlignmentLog2() const
// L1324: size_t StructType::AlignmentLog2() const
// L1336: void Field::ValidateAlignment(ResidueClass at_offset) const
// L1356: std::optional<std::tuple<size_t, std::string>> SizeOf(const Type* type)
// L1427: bool IsAnyUnsignedInteger(const Type* type)
// L1435: bool IsAllowedAsBitField(const Type* type)
// L1447: bool IsPointerSizeIntegralType(const Type* type)
// L1452: bool Is32BitIntegralType(const Type* type)
// L1458: std::optional<NameAndType> ExtractSimpleFieldArraySize( const ClassType& class_type, Expression* array_size)
// L1470: std::string Type::GetRuntimeType() const
// L1490: std::string Type::GetDebugType() const

// ── types.cc 核心自由函数 Rust 签名 (oracle 显式传参, 填体) ──
use crate::type_oracle::TypeOracle;

/// 原版 Type::IsSubtypeOf (types.cc:104-116) — 虚分派语义展开:
/// sub 为 Union 时全成员成立 (types.h:420-425); 否则 sup 为 Top 恒真 /
/// sub 为 never 恒真 / sup 为 Union 任一成员成立 (types.h:427-434) / 父链上溯
#[must_use]
pub fn is_subtype_of(oracle: &TypeOracle, subtype: TypeId, supertype: TypeId) -> bool {
  // UnionType::IsSubtypeOf 覆写优先 (C++ 虚分派按 this 类型)
  if let TypeData::Union(members) = &oracle.types[subtype as usize].data {
    return members
      .types
      .iter()
      .all(|&m| is_subtype_of(oracle, m, supertype));
  }
  let sup = &oracle.types[supertype as usize];
  if sup.is_top_type() {
    return true;
  }
  if oracle.types[subtype as usize].is_never() {
    return true;
  }
  if let TypeData::Union(members) = &sup.data {
    return members
      .types
      .iter()
      .any(|&m| is_subtype_of(oracle, subtype, m));
  }
  let mut current = Some(subtype);
  while let Some(t) = current {
    if t == supertype {
      return true;
    }
    current = oracle.types[t as usize].parent;
  }
  false
}

/// 原版 Type::ComputeName (types.cc:550-571):
/// 无特化 → basename; 特化 → "Base<a, b>" (引用泛型分支需 Declarations 查询, 登记未填体)
#[must_use]
pub fn compute_name(
  cx: &GlobalContext,
  oracle: &TypeOracle,
  basename: &str,
  specialized_from: &MaybeSpecializationKey,
) -> String {
  let Some(key) = specialized_from else {
    return basename.to_owned();
  };
  let args = key
    .specialized_types
    .iter()
    .map(|&t| oracle.types[t as usize].to_display_string(cx, oracle))
    .collect::<Vec<_>>()
    .join(", ");
  format!("{basename}<{args}>")
}

/// 原版 IsAssignableFrom
pub fn is_assignable_from(oracle: &TypeOracle, to: TypeId, from: TypeId) -> bool {
  // TODO(填体)
  let _ = (oracle, to, from);
  false
}

/// 原版 LowerType
pub fn lower_type(oracle: &TypeOracle, type_: TypeId) -> Result<TypeVector> {
  // TODO(填体)
  let _ = (oracle, type_);
  Err(Error::Unimplemented)
}

/// 原版 LoweredSlotCount
pub fn lowered_slot_count(oracle: &TypeOracle, type_: TypeId) -> Result<usize> {
  // TODO(填体)
  let _ = (oracle, type_);
  Err(Error::Unimplemented)
}

/// 原版 LowerParameterTypes(TypeVector)
pub fn lower_parameter_types(oracle: &TypeOracle, parameters: &TypeVector) -> Result<TypeVector> {
  // TODO(填体)
  let _ = (oracle, parameters);
  Err(Error::Unimplemented)
}

/// 原版 SizeOf
pub fn size_of(oracle: &TypeOracle, type_: TypeId) -> Result<Option<(usize, String)>> {
  // TODO(填体)
  let _ = (oracle, type_);
  Err(Error::Unimplemented)
}

/// 原版 IsAnyUnsignedInteger
#[must_use]
pub fn is_any_unsigned_integer(oracle: &TypeOracle, type_: TypeId) -> bool {
  // TODO(填体)
  let _ = (oracle, type_);
  false
}

/// 原版 IsAllowedAsBitField
#[must_use]
pub fn is_allowed_as_bit_field(oracle: &TypeOracle, type_: TypeId) -> bool {
  // TODO(填体)
  let _ = (oracle, type_);
  false
}

/// 原版 IsPointerSizeIntegralType
#[must_use]
pub fn is_pointer_size_integral_type(oracle: &TypeOracle, type_: TypeId) -> bool {
  // TODO(填体)
  let _ = (oracle, type_);
  false
}

/// 原版 Is32BitIntegralType
#[must_use]
pub fn is_32_bit_integral_type(oracle: &TypeOracle, type_: TypeId) -> bool {
  // TODO(填体)
  let _ = (oracle, type_);
  false
}

/// 原版 SubtractType
pub fn subtract_type(oracle: &TypeOracle, a: TypeId, b: TypeId) -> Result<TypeId> {
  // TODO(填体)
  let _ = (oracle, a, b);
  Err(Error::Unimplemented)
}

/// 原版 Type::CommonSupertype
pub fn common_supertype(oracle: &TypeOracle, a: TypeId, b: TypeId) -> Result<TypeId> {
  // TODO(填体)
  let _ = (oracle, a, b);
  Err(Error::Unimplemented)
}

/// 原版 ProjectStructField
pub fn project_struct_field(
  oracle: &TypeOracle,
  structure: VisitResult,
  fieldname: &str,
) -> Result<VisitResult> {
  // TODO(填体)
  let _ = (oracle, structure, fieldname);
  Err(Error::Unimplemented)
}

/// 原版 ExtractSimpleFieldArraySize
pub fn extract_simple_field_array_size(
  oracle: &TypeOracle,
  class_type: TypeId,
  array_size: NodeId,
) -> Result<Option<NameAndType>> {
  // TODO(填体)
  let _ = (oracle, class_type, array_size);
  Err(Error::Unimplemented)
}
