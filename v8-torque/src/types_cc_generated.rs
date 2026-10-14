// 由 tool/src/torque/types.cc.js 从 v8/src/torque/types.cc 生成; 修订就地改, 复跑覆盖
// Type 体系 .cc 方法定义头对数清单

/// torque types.cc 方法定义头清单 (对数基准, 供 Developer 修订 types.rs 对照)。
/// 类 → 方法定义数: std=44, Type=5, UnionType=4, BitFieldStructType=1, AggregateType=4, StructType=4, ClassType=11, Signature=2, BuiltinPointerType=1, VisitResult=2, AbstractType=1, Field=1
/// std::string Type::ToString() const
/// std::string Type::SimpleName() const
/// std::string Type::GetHandleTypeName(HandleKind kind, const std::string& type_name) const
/// std::string Type::HandlifiedCppTypeName(HandleKind kind) const
/// std::string Type::TagglifiedCppTypeName() const
/// bool Type::IsSubtypeOf(const Type* supertype) const
/// std::string Type::GetConstexprGeneratedTypeName() const
/// std::optional<const ClassType*> Type::ClassSupertype() const
/// std::optional<const StructType*> Type::StructSupertype() const
/// std::optional<const AggregateType*> Type::AggregateSupertype() const
/// const Type* Type::CommonSupertype(const Type* a, const Type* b)
/// int Type::Depth() const
/// bool Type::IsAbstractName(const std::string& name) const
/// std::string Type::GetGeneratedTypeName() const
/// std::string Type::GetGeneratedTNodeTypeName() const
/// std::string AbstractType::GetGeneratedTypeNameImpl() const
/// std::string AbstractType::GetGeneratedTNodeTypeNameImpl() const
/// std::vector<TypeChecker> AbstractType::GetTypeCheckers() const
/// std::string BuiltinPointerType::ToExplicitString() const
/// std::string BuiltinPointerType::SimpleNameImpl() const
/// std::string UnionType::ToExplicitString() const
/// std::string UnionType::SimpleNameImpl() const
/// void UnionType::InsertGeneratedTNodeTypeName(std::set<std::string>& names, const Type* t)
/// std::string UnionType::GetGeneratedTNodeTypeNameImpl() const
/// std::string UnionType::GetRuntimeType() const
/// void UnionType::InsertConstexprGeneratedTypeName(std::set<std::string>& names, const Type* t)
/// std::string UnionType::GetConstexprGeneratedTypeName() const
/// std::string UnionType::GetDebugType() const { return parent()->GetDebugType(); }  void UnionType::RecomputeParent()
/// void UnionType::RecomputeParent()
/// void UnionType::Subtract(const Type* t)
/// std::string BitFieldStructType::ToExplicitString() const
/// const BitField& BitFieldStructType::LookupField(const std::string& name) const
/// void AggregateType::CheckForDuplicateFields() const
/// std::vector<const AggregateType*> AggregateType::GetHierarchy() const
/// bool AggregateType::HasField(const std::string& name) const
/// const Field& AggregateType::LookupFieldInternal(const std::string& name) const
/// const Field& AggregateType::LookupField(const std::string& name) const
/// std::string StructType::GetGeneratedTypeNameImpl() const
/// size_t StructType::PackedSize() const
/// StructType::Classification StructType::ClassifyContents() const
/// std::string Type::ComputeName(const std::string& basename, MaybeSpecializationKey specialized_from)
/// std::string StructType::SimpleNameImpl() const { return decl_->name->value; }  // static std::optional<const Type*> Type::MatchUnaryGeneric(const Type* type, GenericType* generic)
/// std::optional<const Type*> Type::MatchUnaryGeneric(const Type* type, GenericType* generic)
/// std::vector<Method*> AggregateType::Methods(const std::string& name) const
/// std::string StructType::ToExplicitString() const { return "struct " + name(); }  void StructType::Finalize() const
/// void StructType::Finalize() const
/// std::string ClassType::GetGeneratedTNodeTypeNameImpl() const
/// std::string ClassType::GetGeneratedTypeNameImpl() const
/// std::string ClassType::ToExplicitString() const { return "class " + name(); }  bool ClassType::AllowInstantiation() const
/// bool ClassType::AllowInstantiation() const
/// void ClassType::Finalize() const
/// std::vector<Field> ClassType::ComputeAllFields() const
/// std::vector<Field> ClassType::ComputeHeaderFields() const
/// std::vector<Field> ClassType::ComputeArrayFields() const
/// void ClassType::InitializeInstanceTypes( std::optional<int> own, std::optional<std::pair<int, int>> range) const
/// std::optional<int> ClassType::OwnInstanceType() const
/// std::optional<std::pair<int, int>> ClassType::InstanceTypeRange() const
/// std::vector<ObjectSlotKind> ClassType::ComputeHeaderSlotKinds() const
/// std::optional<ObjectSlotKind> ClassType::ComputeArraySlotKind() const
/// bool ClassType::HasNoPointerSlotsExceptMap() const
/// bool ClassType::HasDynamicIndexedFieldsIncludingInParents() const
/// const Field* ClassType::GetFieldPreceding(size_t field_index) const
/// const ClassType* ClassType::GetClassDeclaringField(const Field& f) const
/// std::string ClassType::GetSliceMacroName(const Field& field) const
/// void ClassType::GenerateAccessors()
/// void ClassType::GenerateSliceAccessor(size_t field_index)
/// bool ClassType::HasStaticSize() const
/// SourceId ClassType::AttributedToFile() const
/// bool Signature::HasSameTypesAs(const Signature& other, ParameterMode mode) const
/// bool Signature::HasContextParameter() const
/// bool BuiltinPointerType::HasContextParameter() const
/// VisitResult VisitResult::NeverResult()
/// VisitResult VisitResult::TopTypeResult(std::string top_reason, const Type* from_type)
/// std::tuple<size_t, std::string> Field::GetFieldSizeInformation() const
/// size_t Type::AlignmentLog2() const
/// size_t AbstractType::AlignmentLog2() const
/// size_t StructType::AlignmentLog2() const
/// void Field::ValidateAlignment(ResidueClass at_offset) const
/// std::string Type::GetRuntimeType() const
/// std::string Type::GetDebugType() const
/// 类型体系骨架单测锚点 (机械登记)
pub mod types_cc_skel {
  /// 定义头总数对数守卫
  pub const DEF_COUNT: usize = 80;
}
