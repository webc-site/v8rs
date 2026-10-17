// 由 tool/src/torque/types.js 从 v8/src/torque/type-oracle.h 生成; 修订就地改, 复跑覆盖
// TypeOracle: 类型 arena + interning + 查询 (静态单例 → 结构体)

use crate::{
  ast::NodeId,
  constants::{
    ARGUMENTS_TYPE_STRING, AbstractTypeFlags, BOOL_TYPE_STRING, BUILTIN_POINTER_TYPE_STRING,
    CONST_FLOAT64_TYPE_STRING, CONST_INT31_TYPE_STRING, CONST_INT32_TYPE_STRING,
    CONST_STRING_TYPE_STRING, CONSTEXPR_BOOL_TYPE_STRING, CONSTEXPR_INSTANCE_TYPE_TYPE_STRING,
    CONSTEXPR_INTPTR_TYPE_STRING, CONSTEXPR_STRING_TYPE_STRING, CONTEXT_TYPE_STRING,
    CPPHEAPPTR_TYPE_STRING, ClassFlags, DISPATCH_HANDLE_TYPE_STRING, EXTERNALPTR_TYPE_STRING,
    FIXED_ARRAY_BASE_TYPE_STRING, FLOAT64_OR_UNDEFINED_OR_HOLE_TYPE_STRING, FLOAT64_TYPE_STRING,
    HEAP_OBJECT_TYPE_STRING, INT64_TYPE_STRING, INTEGER_LITERAL_TYPE_STRING, INTPTR_TYPE_STRING,
    JS_FUNCTION_TYPE_STRING, JSANY_TYPE_STRING, JSOBJECT_TYPE_STRING, MAP_TYPE_STRING,
    NATIVE_CONTEXT_TYPE_STRING, NEVER_TYPE_STRING, NO_CONTEXT_TYPE_STRING, NUMBER_TYPE_STRING,
    OBJECT_TYPE_STRING, PROTECTEDPTR_TYPE_STRING, RAWPTR_TYPE_STRING, SMI_TYPE_STRING,
    STATIC_ASSERT_MACRO_STRING, STRING_TYPE_STRING, STRONG_TAGGED_TYPE_STRING, TAGGED_TYPE_STRING,
    TAGGED_ZERO_PATTERN_TYPE_STRING, TRUSTEDPTR_TYPE_STRING, UINTPTR_TYPE_STRING,
    UNINITIALIZED_HEAP_OBJECT_TYPE_STRING, UNINITIALIZED_ITERATOR_TYPE_STRING, VOID_TYPE_STRING,
  },
  declarable::DeclarableId,
  error::{Error, Result},
  types::{
    AbstractType, AggregateCommon, BitFieldStructType, BuiltinPointerType, ClassType,
    MaybeSpecializationKey, StructType, TopType, Type, TypeData, TypeId, TypeKind, TypeVector,
    UnionType,
  },
  utils::{Deduplicator, ResidueClass},
};

// C++ ContextualClass 静态单例 → Rust 由编译驱动持有 TypeOracle 实例, 方法去 static。
// 原版分散的 unique_ptr 存储 (nominal/aggregate/bit_field_struct/top_types) 统一为
// 单一 types: Vec<Type> arena, 分 kind 列表保留索引 (登记偏差, 语义等价)。
/// 原版 TypeOracle — 全部静态方法 → 实例方法 (169 个方法签名见原版)
#[derive(Debug, Default)]
pub struct TypeOracle {
  /// 原版 function_pointer_types_ 去重存储 (键含参数/返回类型, 权益下标即 TypeId)
  function_pointer_types: Deduplicator<BuiltinPointerType>,
  /// 原版 all_builtin_pointer_types_
  all_builtin_pointer_types: Vec<TypeId>,
  /// interning slot → TypeId 平行表
  function_pointer_type_ids: Vec<TypeId>,
  /// 原版 union_types_ 去重存储
  union_types: Deduplicator<UnionType>,
  /// 统一 Type arena (TypeId = 下标)
  pub types: Vec<Type>,
  /// 原版 aggregate_types_ 索引
  pub aggregate_types: Vec<TypeId>,
  /// 原版 bit_field_struct_types_ 索引
  pub bit_field_struct_types: Vec<TypeId>,
  /// 原版 top_types_ 索引
  pub top_types: Vec<TypeId>,
  /// 原版 generic_type_instantiation_namespaces_
  pub generic_type_instantiation_namespaces: Vec<DeclarableId>,
  next_type_id: usize,
}

impl TypeOracle {
  fn push_type(
    &mut self,
    kind: TypeKind,
    parent: Option<TypeId>,
    specialized_from: MaybeSpecializationKey,
    data: TypeData,
  ) -> TypeId {
    let id = self.next_type_id;
    self.next_type_id += 1;
    self.types.push(Type {
      kind,
      parent,
      id,
      specialized_from,
      aliases: Default::default(),
      data,
    });
    (self.types.len() - 1) as TypeId
  }

  /// 原版 FreshTypeId
  pub fn fresh_type_id(&mut self) -> usize {
    self.next_type_id += 1;
    self.next_type_id - 1
  }

  /// 原版 GetAbstractType
  pub fn get_abstract_type(
    &mut self,
    parent: Option<TypeId>,
    name: String,
    flags: AbstractTypeFlags,
    generated: String,
    non_constexpr_version: Option<TypeId>,
    specialized_from: MaybeSpecializationKey,
  ) -> TypeId {
    let data = TypeData::Abstract(AbstractType {
      flags,
      name: name.clone(),
      generated_type: generated,
      non_constexpr_version,
      constexpr_version: None,
    });
    let t = self.push_type(TypeKind::AbstractType, parent, specialized_from, data);
    // 原版: 非 constexpr 版本回填 constexpr_version_
    if let Some(nc) = non_constexpr_version
      && let Some(nc_type) = self.types.get_mut(nc as usize)
      && let TypeData::Abstract(a) = &mut nc_type.data
    {
      a.constexpr_version = Some(t);
    }
    t
  }

  /// 原版 GetTopType
  pub fn get_top_type(&mut self, reason: String, source_type: TypeId) -> TypeId {
    let data = TypeData::Top(TopType {
      reason,
      source_type: Some(source_type),
    });
    let t = self.push_type(TypeKind::TopType, None, None, data);
    self.top_types.push(t);
    t
  }

  /// 原版 GetStructType
  pub fn get_struct_type(
    &mut self,
    decl: NodeId,
    specialized_from: MaybeSpecializationKey,
  ) -> TypeId {
    let data = TypeData::Struct(StructType {
      aggregate: AggregateCommon::default(),
      decl,
      generated_type_name: String::new(),
    });
    let t = self.push_type(TypeKind::StructType, None, specialized_from, data);
    self.aggregate_types.push(t);
    t
  }

  /// 原版 GetBitFieldStructType
  pub fn get_bit_field_struct_type(&mut self, parent: Option<TypeId>, decl: NodeId) -> TypeId {
    let data = TypeData::BitFieldStruct(BitFieldStructType {
      namespace_: None,
      decl,
      fields: Vec::new(),
    });
    let t = self.push_type(TypeKind::BitFieldStructType, parent, None, data);
    self.bit_field_struct_types.push(t);
    t
  }

  /// 原版 GetClassType
  pub fn get_class_type(
    &mut self,
    parent: Option<TypeId>,
    name: String,
    flags: ClassFlags,
    generates: String,
    decl: NodeId,
    alias: Option<DeclarableId>,
  ) -> TypeId {
    let data = TypeData::Class(ClassType {
      aggregate: AggregateCommon {
        name,
        ..Default::default()
      },
      header_size: 0,
      size: ResidueClass::unknown(),
      flags,
      generates,
      decl,
      alias,
      own_instance_type: None,
      instance_type_range: None,
    });
    let t = self.push_type(TypeKind::ClassType, parent, None, data);
    self.aggregate_types.push(t);
    t
  }

  /// 原版 GetBuiltinPointerType (去重 interning, slot → TypeId 平行表)
  pub fn get_builtin_pointer_type(
    &mut self,
    argument_types: TypeVector,
    return_type: TypeId,
  ) -> TypeId {
    let key = BuiltinPointerType {
      function_pointer_type_id: usize::MAX,
      parameter_types: argument_types,
      return_type,
    };
    let slot = self.function_pointer_types.intern(key);
    if let Some(&t) = self.function_pointer_type_ids.get(slot) {
      return t;
    }
    let data = TypeData::BuiltinPointer(self.function_pointer_types.get(slot).clone());
    // TODO(填体): parent 设为 BuiltinPtr 抽象型 (Declarations 查询)
    let t = self.push_type(TypeKind::BuiltinPointerType, None, None, data);
    self.all_builtin_pointer_types.push(t);
    self.function_pointer_type_ids.push(t);
    t
  }

  /// 原版 GetUnionType(UnionType) — 单成员折叠 + 去重
  pub fn intern_union_type(&mut self, union: UnionType) -> Result<TypeId> {
    if union.types.len() == 1
      && let Some(&single) = union.types.iter().next()
    {
      return Ok(single);
    }
    let _ = self.union_types.intern(union);
    // TODO(填体): 去重键 → TypeId 映射
    Err(Error::Unimplemented)
  }

  /// 原版 GetUnionType(a, b)
  pub fn get_union_type(&mut self, a: TypeId, b: TypeId) -> Result<TypeId> {
    // TODO(填体): 子型折叠 + Extend + interning
    let _ = (a, b);
    Err(Error::Unimplemented)
  }

  /// 原版 GetGenericTypeInstance
  pub fn get_generic_type_instance(
    &mut self,
    generic_type: DeclarableId,
    arg_types: TypeVector,
  ) -> Result<TypeId> {
    // TODO(填体)
    let _ = (generic_type, arg_types);
    Err(Error::Unimplemented)
  }

  /// 原版 MatchReferenceGeneric (返回 (is_const, 被引用类型))
  pub fn match_reference_generic(&self, reference_type: TypeId) -> Result<Option<(bool, TypeId)>> {
    // TODO(填体)
    let _ = reference_type;
    Err(Error::Unimplemented)
  }

  /// 原版 ImplicitlyConvertableFrom
  pub fn implicitly_convertable_from(&self, to: TypeId, from: TypeId) -> Result<Option<TypeId>> {
    // TODO(填体)
    let _ = (to, from);
    Err(Error::Unimplemented)
  }

  /// 原版 GetClasses (拓扑有序)
  pub fn get_classes(&self) -> Result<Vec<TypeId>> {
    // TODO(填体)
    Err(Error::Unimplemented)
  }

  /// 原版 FinalizeAggregateTypes
  pub fn finalize_aggregate_types(&mut self) -> Result<()> {
    // TODO(填体)
    Err(Error::Unimplemented)
  }

  /// 原版 CreateGenericTypeInstantiationNamespace
  pub fn create_generic_type_instantiation_namespace(&mut self) -> Result<DeclarableId> {
    // TODO(填体)
    Err(Error::Unimplemented)
  }

  /// 原版 GetBuiltinType(name) 私有查询 (Declarations 全局查找)
  fn get_builtin_type(&self, name: &str) -> Result<TypeId> {
    // TODO(填体): Declarations::LookupGlobalType
    let _ = name;
    Err(Error::Unimplemented)
  }

  /// 原版 GetArgumentsType → GetBuiltinType(ARGUMENTS_TYPE_STRING)
  pub fn arguments_type(&self) -> Result<TypeId> {
    self.get_builtin_type(ARGUMENTS_TYPE_STRING)
  }

  /// 原版 GetBoolType → GetBuiltinType(BOOL_TYPE_STRING)
  pub fn bool_type(&self) -> Result<TypeId> {
    self.get_builtin_type(BOOL_TYPE_STRING)
  }

  /// 原版 GetConstexprBoolType → GetBuiltinType(CONSTEXPR_BOOL_TYPE_STRING)
  pub fn constexpr_bool_type(&self) -> Result<TypeId> {
    self.get_builtin_type(CONSTEXPR_BOOL_TYPE_STRING)
  }

  /// 原版 GetConstexprStringType → GetBuiltinType(CONSTEXPR_STRING_TYPE_STRING)
  pub fn constexpr_string_type(&self) -> Result<TypeId> {
    self.get_builtin_type(CONSTEXPR_STRING_TYPE_STRING)
  }

  /// 原版 GetConstexprIntPtrType → GetBuiltinType(CONSTEXPR_INTPTR_TYPE_STRING)
  pub fn constexpr_int_ptr_type(&self) -> Result<TypeId> {
    self.get_builtin_type(CONSTEXPR_INTPTR_TYPE_STRING)
  }

  /// 原版 GetConstexprInstanceTypeType → GetBuiltinType(CONSTEXPR_INSTANCE_TYPE_TYPE_STRING)
  pub fn constexpr_instance_type_type(&self) -> Result<TypeId> {
    self.get_builtin_type(CONSTEXPR_INSTANCE_TYPE_TYPE_STRING)
  }

  /// 原版 GetVoidType → GetBuiltinType(VOID_TYPE_STRING)
  pub fn void_type(&self) -> Result<TypeId> {
    self.get_builtin_type(VOID_TYPE_STRING)
  }

  /// 原版 GetRawPtrType → GetBuiltinType(RAWPTR_TYPE_STRING)
  pub fn raw_ptr_type(&self) -> Result<TypeId> {
    self.get_builtin_type(RAWPTR_TYPE_STRING)
  }

  /// 原版 GetExternalPointerType → GetBuiltinType(EXTERNALPTR_TYPE_STRING)
  pub fn external_pointer_type(&self) -> Result<TypeId> {
    self.get_builtin_type(EXTERNALPTR_TYPE_STRING)
  }

  /// 原版 GetCppHeapPointerType → GetBuiltinType(CPPHEAPPTR_TYPE_STRING)
  pub fn cpp_heap_pointer_type(&self) -> Result<TypeId> {
    self.get_builtin_type(CPPHEAPPTR_TYPE_STRING)
  }

  /// 原版 GetTrustedPointerType → GetBuiltinType(TRUSTEDPTR_TYPE_STRING)
  pub fn trusted_pointer_type(&self) -> Result<TypeId> {
    self.get_builtin_type(TRUSTEDPTR_TYPE_STRING)
  }

  /// 原版 GetProtectedPointerType → GetBuiltinType(PROTECTEDPTR_TYPE_STRING)
  pub fn protected_pointer_type(&self) -> Result<TypeId> {
    self.get_builtin_type(PROTECTEDPTR_TYPE_STRING)
  }

  /// 原版 GetDispatchHandleType → GetBuiltinType(DISPATCH_HANDLE_TYPE_STRING)
  pub fn dispatch_handle_type(&self) -> Result<TypeId> {
    self.get_builtin_type(DISPATCH_HANDLE_TYPE_STRING)
  }

  /// 原版 GetMapType → GetBuiltinType(MAP_TYPE_STRING)
  pub fn map_type(&self) -> Result<TypeId> {
    self.get_builtin_type(MAP_TYPE_STRING)
  }

  /// 原版 GetObjectType → GetBuiltinType(OBJECT_TYPE_STRING)
  pub fn object_type(&self) -> Result<TypeId> {
    self.get_builtin_type(OBJECT_TYPE_STRING)
  }

  /// 原版 GetHeapObjectType → GetBuiltinType(HEAP_OBJECT_TYPE_STRING)
  pub fn heap_object_type(&self) -> Result<TypeId> {
    self.get_builtin_type(HEAP_OBJECT_TYPE_STRING)
  }

  /// 原版 GetTaggedZeroPatternType → GetBuiltinType(TAGGED_ZERO_PATTERN_TYPE_STRING)
  pub fn tagged_zero_pattern_type(&self) -> Result<TypeId> {
    self.get_builtin_type(TAGGED_ZERO_PATTERN_TYPE_STRING)
  }

  /// 原版 GetJsanyType → GetBuiltinType(JSANY_TYPE_STRING)
  pub fn jsany_type(&self) -> Result<TypeId> {
    self.get_builtin_type(JSANY_TYPE_STRING)
  }

  /// 原版 GetJsobjectType → GetBuiltinType(JSOBJECT_TYPE_STRING)
  pub fn jsobject_type(&self) -> Result<TypeId> {
    self.get_builtin_type(JSOBJECT_TYPE_STRING)
  }

  /// 原版 GetTaggedType → GetBuiltinType(TAGGED_TYPE_STRING)
  pub fn tagged_type(&self) -> Result<TypeId> {
    self.get_builtin_type(TAGGED_TYPE_STRING)
  }

  /// 原版 GetStrongTaggedType → GetBuiltinType(STRONG_TAGGED_TYPE_STRING)
  pub fn strong_tagged_type(&self) -> Result<TypeId> {
    self.get_builtin_type(STRONG_TAGGED_TYPE_STRING)
  }

  /// 原版 GetSmiType → GetBuiltinType(SMI_TYPE_STRING)
  pub fn smi_type(&self) -> Result<TypeId> {
    self.get_builtin_type(SMI_TYPE_STRING)
  }

  /// 原版 GetConstStringType → GetBuiltinType(CONST_STRING_TYPE_STRING)
  pub fn const_string_type(&self) -> Result<TypeId> {
    self.get_builtin_type(CONST_STRING_TYPE_STRING)
  }

  /// 原版 GetStringType → GetBuiltinType(STRING_TYPE_STRING)
  pub fn string_type(&self) -> Result<TypeId> {
    self.get_builtin_type(STRING_TYPE_STRING)
  }

  /// 原版 GetNumberType → GetBuiltinType(NUMBER_TYPE_STRING)
  pub fn number_type(&self) -> Result<TypeId> {
    self.get_builtin_type(NUMBER_TYPE_STRING)
  }

  /// 原版 GetIntPtrType → GetBuiltinType(INTPTR_TYPE_STRING)
  pub fn int_ptr_type(&self) -> Result<TypeId> {
    self.get_builtin_type(INTPTR_TYPE_STRING)
  }

  /// 原版 GetUintPtrType → GetBuiltinType(UINTPTR_TYPE_STRING)
  pub fn uint_ptr_type(&self) -> Result<TypeId> {
    self.get_builtin_type(UINTPTR_TYPE_STRING)
  }

  /// 原版 GetInt64Type → GetBuiltinType(INT64_TYPE_STRING)
  pub fn int64_type(&self) -> Result<TypeId> {
    self.get_builtin_type(INT64_TYPE_STRING)
  }

  /// 原版 GetFloat64Type → GetBuiltinType(FLOAT64_TYPE_STRING)
  pub fn float64_type(&self) -> Result<TypeId> {
    self.get_builtin_type(FLOAT64_TYPE_STRING)
  }

  /// 原版 GetFloat64OrUndefinedOrHoleType → GetBuiltinType(FLOAT64_OR_UNDEFINED_OR_HOLE_TYPE_STRING)
  pub fn float64_or_undefined_or_hole_type(&self) -> Result<TypeId> {
    self.get_builtin_type(FLOAT64_OR_UNDEFINED_OR_HOLE_TYPE_STRING)
  }

  /// 原版 GetConstFloat64Type → GetBuiltinType(CONST_FLOAT64_TYPE_STRING)
  pub fn const_float64_type(&self) -> Result<TypeId> {
    self.get_builtin_type(CONST_FLOAT64_TYPE_STRING)
  }

  /// 原版 GetIntegerLiteralType → GetBuiltinType(INTEGER_LITERAL_TYPE_STRING)
  pub fn integer_literal_type(&self) -> Result<TypeId> {
    self.get_builtin_type(INTEGER_LITERAL_TYPE_STRING)
  }

  /// 原版 GetNeverType → GetBuiltinType(NEVER_TYPE_STRING)
  pub fn never_type(&self) -> Result<TypeId> {
    self.get_builtin_type(NEVER_TYPE_STRING)
  }

  /// 原版 GetConstInt31Type → GetBuiltinType(CONST_INT31_TYPE_STRING)
  pub fn const_int31_type(&self) -> Result<TypeId> {
    self.get_builtin_type(CONST_INT31_TYPE_STRING)
  }

  /// 原版 GetConstInt32Type → GetBuiltinType(CONST_INT32_TYPE_STRING)
  pub fn const_int32_type(&self) -> Result<TypeId> {
    self.get_builtin_type(CONST_INT32_TYPE_STRING)
  }

  /// 原版 GetContextType → GetBuiltinType(CONTEXT_TYPE_STRING)
  pub fn context_type(&self) -> Result<TypeId> {
    self.get_builtin_type(CONTEXT_TYPE_STRING)
  }

  /// 原版 GetNoContextType → GetBuiltinType(NO_CONTEXT_TYPE_STRING)
  pub fn no_context_type(&self) -> Result<TypeId> {
    self.get_builtin_type(NO_CONTEXT_TYPE_STRING)
  }

  /// 原版 GetNativeContextType → GetBuiltinType(NATIVE_CONTEXT_TYPE_STRING)
  pub fn native_context_type(&self) -> Result<TypeId> {
    self.get_builtin_type(NATIVE_CONTEXT_TYPE_STRING)
  }

  /// 原版 GetJsFunctionType → GetBuiltinType(JS_FUNCTION_TYPE_STRING)
  pub fn js_function_type(&self) -> Result<TypeId> {
    self.get_builtin_type(JS_FUNCTION_TYPE_STRING)
  }

  /// 原版 GetUninitializedIteratorType → GetBuiltinType(UNINITIALIZED_ITERATOR_TYPE_STRING)
  pub fn uninitialized_iterator_type(&self) -> Result<TypeId> {
    self.get_builtin_type(UNINITIALIZED_ITERATOR_TYPE_STRING)
  }

  /// 原版 GetFixedArrayBaseType → GetBuiltinType(FIXED_ARRAY_BASE_TYPE_STRING)
  pub fn fixed_array_base_type(&self) -> Result<TypeId> {
    self.get_builtin_type(FIXED_ARRAY_BASE_TYPE_STRING)
  }

  /// 原版 GetStaticAssertMacro → GetBuiltinType(STATIC_ASSERT_MACRO_STRING)
  pub fn static_assert_macro(&self) -> Result<TypeId> {
    self.get_builtin_type(STATIC_ASSERT_MACRO_STRING)
  }

  /// 原版 GetUninitializedHeapType (torque_internal 限定名)
  pub fn uninitialized_heap_type(&self) -> Result<TypeId> {
    self.get_builtin_type(UNINITIALIZED_HEAP_OBJECT_TYPE_STRING)
  }

  /// 原版 GetUint64Type
  pub fn uint64_type(&self) -> Result<TypeId> {
    self.get_builtin_type("uint64")
  }

  /// 原版 GetUint32Type
  pub fn uint32_type(&self) -> Result<TypeId> {
    self.get_builtin_type("uint32")
  }

  /// 原版 GetUint31Type
  pub fn uint31_type(&self) -> Result<TypeId> {
    self.get_builtin_type("uint31")
  }

  /// 原版 GetInt32Type
  pub fn int32_type(&self) -> Result<TypeId> {
    self.get_builtin_type("int32")
  }

  /// 原版 GetInt16Type / GetUint16Type / GetInt8Type / GetUint8Type / GetBintType 同式 (字面量名直查)
  pub fn int16_type(&self) -> Result<TypeId> {
    self.get_builtin_type("int16")
  }

  pub fn uint16_type(&self) -> Result<TypeId> {
    self.get_builtin_type("uint16")
  }

  pub fn int8_type(&self) -> Result<TypeId> {
    self.get_builtin_type("int8")
  }

  pub fn uint8_type(&self) -> Result<TypeId> {
    self.get_builtin_type("uint8")
  }

  pub fn bint_type(&self) -> Result<TypeId> {
    self.get_builtin_type("bint")
  }

  pub fn builtin_pointer_type_name(&self) -> Result<TypeId> {
    self.get_builtin_type(BUILTIN_POINTER_TYPE_STRING)
  }

  /// 原版 GetReferenceGeneric / GetConstReferenceGeneric / GetMutableReferenceGeneric
  pub fn reference_generic(&self, is_const: bool) -> Result<DeclarableId> {
    // TODO(填体): Declarations::LookupUniqueGenericType(torque_internal::xxx)
    let _ = is_const;
    Err(Error::Unimplemented)
  }

  /// 原版 GetMutableSliceGeneric / GetConstSliceGeneric
  pub fn slice_generic(&self, is_const: bool) -> Result<DeclarableId> {
    // TODO(填体)
    let _ = is_const;
    Err(Error::Unimplemented)
  }

  /// 原版 GetWeakGeneric / GetSmiTaggedGeneric / GetLazyGeneric (全局唯一泛型)
  pub fn global_unique_generic(&self, name: &str) -> Result<DeclarableId> {
    // TODO(填体)
    let _ = name;
    Err(Error::Unimplemented)
  }

  /// 原版 GetReferenceType 族
  pub fn reference_type(&self, referenced_type: TypeId, is_const: bool) -> Result<TypeId> {
    // TODO(填体)
    let _ = (referenced_type, is_const);
    Err(Error::Unimplemented)
  }

  /// 原版 GetMutableSliceType / GetConstSliceType
  pub fn slice_type(&self, referenced_type: TypeId, is_const: bool) -> Result<TypeId> {
    // TODO(填体)
    let _ = (referenced_type, is_const);
    Err(Error::Unimplemented)
  }

  /// 原版 AllBuiltinPointerTypes
  #[must_use]
  pub fn all_builtin_pointer_types(&self) -> &[TypeId] {
    &self.all_builtin_pointer_types
  }
}

// 填体提示: implicitly_convertable_from 消费 declarations::FROM_CONSTEXPR_MACRO_NAME;
// reference_generic/slice_generic/global_unique_generic 消费 MUTABLE_/CONST_
// REFERENCE_TYPE_STRING、MUTABLE_/CONST_SLICE_TYPE_STRING、LAZY_/SMI_TAGGED_/WEAK_TYPE_STRING。
// ── 原版 src/torque/type-oracle.cc ──
// L15: const std::vector<std::unique_ptr<AggregateType>>&TypeOracle::GetAggregateTypes()
// L21: const std::vector<std::unique_ptr<BitFieldStructType>>&TypeOracle::GetBitFieldStructTypes()
// L27: void TypeOracle::FinalizeAggregateTypes()
// L36: const Type* TypeOracle::GetGenericTypeInstance(GenericType* generic_type, TypeVector arg_types)
// L65: Namespace* TypeOracle::CreateGenericTypeInstantiationNamespace()
// L72: std::vector<const ClassType*> TypeOracle::GetClasses()
// L82: std::optional<const Type*> TypeOracle::MatchReferenceGeneric( const Type* reference_type, bool* is_const)
