// 由 tool/src/torque/declarable.cc.js 从 v8/src/torque/declarable.{h,cc} 生成; 修订就地改, 复跑覆盖
// 可声明实体族清单与定义头对数

/// torque declarable 实体族登记 (原版 declarable.h class 集, 19 项)
pub mod declarable_kinds {
  pub const KINDS: [&str; 19] = [
    "Declarable",
    "Scope",
    "Namespace",
    "Value",
    "NamespaceConstant",
    "ExternConstant",
    "Callable",
    "Macro",
    "ExternMacro",
    "TorqueMacro",
    "Method",
    "Builtin",
    "RuntimeFunction",
    "Intrinsic",
    "TypeConstraint",
    "GenericDeclarable",
    "GenericCallable",
    "GenericType",
    "TypeAlias",
  ];
}

/// declarable.cc 方法定义头清单 (对数基准, 修订 declarable.rs 时对照)。
/// 类 → 方法定义数: QualifiedName=1, std=3, GenericCallable=1, Namespace=2, TypeAlias=1
/// QualifiedName QualifiedName::Parse(std::string_view qualified_name)
/// std::vector<Declarable*> Scope::Lookup(const QualifiedName& name)
/// std::optional<std::string> TypeConstraint::IsViolated(const Type* type) const
/// TypeArgumentInference GenericCallable::InferSpecializationTypes( const TypeVector& explicit_specialization_types, const std::vector<std::optional<const Type*>>& arguments)
/// std::optional<Statement*> GenericCallable::CallableBody()
/// bool Namespace::IsDefaultNamespace() const
/// bool Namespace::IsTestNamespace() const { return name() == kTestNamespaceName; }  const Type* TypeAlias::Resolve() const
/// const Type* TypeAlias::Resolve() const
pub mod declarable_cc_skel {
  /// 定义头总数对数守卫
  pub const DEF_COUNT: usize = 8;
  /// operator<< 打印定义数
  pub const PRINT_OVERLOADS: usize = 5;
}
