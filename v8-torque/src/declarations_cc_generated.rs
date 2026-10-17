// 由 tool/src/torque/declarations.cc.js 从 v8/src/torque/declarations.cc 生成; 修订就地改, 复跑覆盖
// 声明表 .cc 方法定义头对数清单

/// torque declarations.cc 方法定义头清单 (对数基准, 修订 declarations.rs 时对照)。
/// 类 → 方法定义数: std=6, Declarations=26
/// std::vector<Declarable*> Declarations::LookupGlobalScope( const QualifiedName& name)
/// const TypeAlias* Declarations::LookupTypeAlias(const QualifiedName& name)
/// const Type* Declarations::LookupType(const QualifiedName& name)
/// const Type* Declarations::LookupType(const Identifier* name)
/// std::optional<const Type*> Declarations::TryLookupType( const QualifiedName& name)
/// const Type* Declarations::LookupGlobalType(const QualifiedName& name)
/// Builtin* Declarations::FindSomeInternalBuiltinWithType( const BuiltinPointerType* type)
/// Value* Declarations::LookupValue(const QualifiedName& name)
/// Macro* Declarations::TryLookupMacro(const std::string& name, const TypeVector& types)
/// std::optional<Builtin*> Declarations::TryLookupBuiltin( const QualifiedName& name)
/// std::vector<GenericCallable*> Declarations::LookupGeneric( const std::string& name)
/// GenericCallable* Declarations::LookupUniqueGeneric(const QualifiedName& name)
/// GenericType* Declarations::LookupUniqueGenericType(const QualifiedName& name)
/// GenericType* Declarations::LookupGlobalUniqueGenericType( const std::string& name)
/// std::optional<GenericType*> Declarations::TryLookupGenericType( const QualifiedName& name)
/// Namespace* Declarations::DeclareNamespace(const std::string& name)
/// TypeAlias* Declarations::DeclareType(const Identifier* name, const Type* type)
/// TypeAlias* Declarations::PredeclareTypeAlias(const Identifier* name, TypeDeclaration* type, bool redeclaration)
/// TorqueMacro* Declarations::CreateTorqueMacro( std::string external_name, std::string readable_name, bool exported_to_csa, Signature signature, std::optional<Statement*> body, bool is_user_defined)
/// ExternMacro* Declarations::CreateExternMacro( std::string name, std::string external_assembler_name, Signature signature)
/// Macro* Declarations::DeclareMacro( const std::string& name, bool accessible_from_csa, std::optional<std::string> external_assembler_name, const Signature& signature, std::optional<Statement*> body, std::optional<std::string> op, bool is_user_defined)
/// Method* Declarations::CreateMethod(AggregateType* container_type, const std::string& name, Signature signature, Statement* body)
/// Intrinsic* Declarations::CreateIntrinsic(const std::string& name, const Signature& signature)
/// Intrinsic* Declarations::DeclareIntrinsic(const std::string& name, const Signature& signature)
/// Builtin* Declarations::CreateBuiltin( std::string external_name, std::string readable_name, Builtin::Kind kind, Builtin::Flags flags, Signature signature, std::optional<std::string> use_counter_name, std::optional<Statement*> body)
/// RuntimeFunction* Declarations::DeclareRuntimeFunction( const std::string& name, const Signature& signature)
/// ExternConstant* Declarations::DeclareExternConstant(Identifier* name, const Type* type, std::string value)
/// NamespaceConstant* Declarations::DeclareNamespaceConstant(Identifier* name, const Type* type, Expression* body)
/// GenericCallable* Declarations::DeclareGenericCallable( const std::string& name, GenericCallableDeclaration* ast_node)
/// GenericType* Declarations::DeclareGenericType( const std::string& name, GenericTypeDeclaration* ast_node)
/// std::string Declarations::GetGeneratedCallableName( const std::string& name, const TypeVector& specialized_types)
/// Macro* Declarations::DeclareOperator(const std::string& name, Macro* m)
pub mod declarations_cc_skel {
  /// 定义头总数对数守卫
  pub const DEF_COUNT: usize = 32;
}
