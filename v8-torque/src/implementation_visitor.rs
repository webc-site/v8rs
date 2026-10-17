// 由 tool/src/torque/visitor.js 从 v8/src/torque/implementation-visitor.h 生成; 修订就地改, 复跑覆盖
// 实现访问器 (CFG 构建核心) + LocationReference 五态

use std::collections::{BTreeMap, BTreeSet};

use crate::{
  ast::NodeId,
  cfg::CfgAssembler,
  constants::FieldSynchronization,
  declarable::{DeclarableId, OutputType},
  error::{Error, Result},
  global_context::GlobalContext,
  instructions::BlockId,
  source_positions::{SourceId, SourcePosition},
  type_inference::TypeArgumentInference,
  type_oracle::TypeOracle,
  types::{Signature, TypeId, TypeVector, VisitResult},
  utils::{BottomOffset, replace_file_contents_if_different},
};

// C++ Binding<T> 模板 (LocalValue/LocalLabel 两实例) → Binding + BindingKind 两变体;
// BindingsManager 按名字查绑定; 上下文变量 (ValueBindingsManager/LabelBindingsManager/
// CurrentCallable/CurrentFileStreams/CurrentReturnValue) → ImplementationVisitor 字段
// + 显式传参 (登记偏差, 填体时定形)。
/// 原版 LocationReference — l-value 五态和 (C++ 多字段互斥 → enum)
#[derive(Debug, Clone)]
pub enum LocationReference {
  /// VariableAccess: 可赋值栈区间 (+ 绑定回引)
  Variable {
    variable: VisitResult,
    binding: Option<BindingId>,
  },
  /// Temporary: 不可赋值临时值 ({description} 供错误消息)
  Temporary {
    temporary: VisitResult,
    temporary_description: String,
  },
  /// HeapReference: 堆引用 (tagged 值 + 内指针偏移) + 同步语义
  HeapReference {
    heap_reference: VisitResult,
    synchronization: FieldSynchronization,
  },
  /// HeapSlice: 堆数组引用 (tagged 值 + 偏移 + 元素数)
  HeapSlice { heap_slice: VisitResult },
  /// ArrayAccess/FieldAccess: eval/assign 函数调用对
  Call {
    eval_function: String,
    assign_function: String,
    call_arguments: Vec<VisitResult>,
  },
}

impl LocationReference {
  /// 原版 VariableAccess
  #[must_use]
  pub fn variable_access(variable: VisitResult, binding: Option<BindingId>) -> Self {
    Self::Variable { variable, binding }
  }

  /// 原版 Temporary
  #[must_use]
  pub fn temporary(temporary: VisitResult, description: String) -> Self {
    Self::Temporary {
      temporary,
      temporary_description: description,
    }
  }

  /// 原版 HeapReference
  #[must_use]
  pub fn heap_reference(
    heap_reference: VisitResult,
    synchronization: FieldSynchronization,
  ) -> Self {
    Self::HeapReference {
      heap_reference,
      synchronization,
    }
  }

  /// 原版 HeapSlice
  #[must_use]
  pub fn heap_slice(heap_slice: VisitResult) -> Self {
    Self::HeapSlice { heap_slice }
  }

  /// 原版 ArrayAccess
  #[must_use]
  pub fn array_access(base: VisitResult, offset: VisitResult) -> Self {
    Self::Call {
      eval_function: "[]".to_owned(),
      assign_function: "[]=".to_owned(),
      call_arguments: vec![base, offset],
    }
  }

  /// 原版 FieldAccess
  #[must_use]
  pub fn field_access(object: VisitResult, fieldname: &str) -> Self {
    Self::Call {
      eval_function: format!(".{fieldname}"),
      assign_function: format!(".{fieldname}="),
      call_arguments: vec![object],
    }
  }

  /// 原版 IsVariable
  #[must_use]
  pub fn is_variable(&self) -> bool {
    matches!(self, Self::Variable { .. })
  }

  /// 原版 IsTemporary
  #[must_use]
  pub fn is_temporary(&self) -> bool {
    matches!(self, Self::Temporary { .. })
  }

  /// 原版 IsHeapReference
  #[must_use]
  pub fn is_heap_reference(&self) -> bool {
    matches!(self, Self::HeapReference { .. })
  }

  /// 原版 IsHeapSlice
  #[must_use]
  pub fn is_heap_slice(&self) -> bool {
    matches!(self, Self::HeapSlice { .. })
  }

  /// 原版 IsCall
  #[must_use]
  pub fn is_call(&self) -> bool {
    matches!(self, Self::Call { .. })
  }
}

/// 绑定 arena 索引
pub type BindingId = u32;

/// 原版 Binding<LocalValue>/<LocalLabel> → BindingKind 两变体
#[derive(Debug, Clone)]
pub enum BindingValue {
  Value(LocalValue),
  Label(LocalLabel),
}

/// 原版 Binding<T> 1:1 (IS-A T → value 字段承载; manager_/previous_binding_ 由
/// BindingsManager arena + 影子栈承担, 登记偏差)
#[derive(Debug, Clone)]
pub struct Binding {
  pub name: String,
  pub value: BindingValue,
  pub declaration_position: SourcePosition,
  pub used: bool,
  pub written: bool,
  pub unique_index: u64,
}

/// 原版 BindingsManager<T> 1:1 (value/label 两实例由 BindingKind 区分)
#[derive(Debug, Clone, Default)]
pub struct BindingsManager {
  arena: Vec<Binding>,
  bindings: BTreeMap<String, BindingId>,
}

impl BindingsManager {
  /// 原版 TryLookup
  #[must_use]
  pub fn try_lookup(&self, name: &str) -> Option<BindingId> {
    self.bindings.get(name).copied()
  }

  /// 原版 Binding 构造登记 (Binding::TryCreate)
  pub fn declare_binding(&mut self, name: String, binding: Binding) -> BindingId {
    let id = (self.arena.len()) as BindingId;
    self.arena.push(binding);
    self.bindings.insert(name, id);
    id
  }

  #[must_use]
  pub fn binding(&self, id: BindingId) -> &Binding {
    &self.arena[id as usize]
  }

  #[must_use]
  pub fn binding_mut(&mut self, id: BindingId) -> &mut Binding {
    &mut self.arena[id as usize]
  }
}

/// 原版 LocalValue 1:1 (值 / 惰性 / 不可访问三态)
#[derive(Debug, Clone, Default)]
pub struct LocalValue {
  pub value: Option<LocationReference>,
  /// 惰性求值 (原版 std::function<LocationReference()>)
  pub lazy: Option<fn() -> LocationReference>,
  pub inaccessible_explanation: String,
}

impl LocalValue {
  /// 原版 IsAccessibleNonLazy
  #[must_use]
  pub fn is_accessible_non_lazy(&self) -> bool {
    self.value.is_some()
  }

  /// 原版 GetLocationReference
  pub fn get_location_reference(&self, binding: BindingId) -> Result<LocationReference> {
    // TODO(填体): 三态解析 + 不可访问报错
    let _ = binding;
    Err(Error::Unimplemented)
  }
}

/// 原版 LocalLabel 1:1
#[derive(Debug, Clone, Default)]
pub struct LocalLabel {
  pub block: Option<BlockId>,
  pub parameter_types: TypeVector,
}

/// 原版 BlockBindings<LocalValue> 1:1 (类字段初始化; 管理器引用由调用方持有)
#[derive(Debug, Clone, Default)]
pub struct BlockBindings {
  pub bindings: Vec<BindingId>,
}

/// 原版 Arguments 1:1
#[derive(Debug, Clone, Default)]
pub struct Arguments {
  pub parameters: Vec<VisitResult>,
  pub labels: Vec<BindingId>,
}

/// 原版 IsCompatibleSignature (重载兼容判定)
#[must_use]
pub fn is_compatible_signature(sig: &Signature, types: &TypeVector, label_count: usize) -> bool {
  // TODO(填体)
  let _ = (sig, types, label_count);
  false
}

/// 原版 next_unique_binding_index (全局计数 → 计数器参数/驱动持有, 登记偏差)
pub type NextUniqueBindingIndex = u64;

/// 原版 ImplementationVisitor 私有字段 1:1 (null_stream_ 删除 — Rust 无流占位)
#[derive(Debug, Default)]
pub struct ImplementationVisitor {
  pub assembler: Option<CfgAssembler>,
  pub is_dry_run: bool,
  /// 位域表达式来源表 (表达式 → 所属位域结构体标识符)
  pub bitfield_expressions: BTreeMap<NodeId, NodeId>,
  /// InlineMacro 内联中的宏集合 (警告用)
  pub inlining_macros: BTreeSet<DeclarableId>,
  /// debug 宏输出文件缓冲 (原版 stringstream)
  pub debug_macros_cc: String,
  pub debug_macros_h: String,
  pub output_type: OutputType,
  /// 原版 ValueBindingsManager / LabelBindingsManager 上下文
  pub value_bindings: BindingsManager,
  pub label_bindings: BindingsManager,
  /// 原版 CurrentCallable / CurrentReturnValue 上下文
  pub current_callable: Option<DeclarableId>,
  pub current_return_value: Option<VisitResult>,
}

impl ImplementationVisitor {
  /// 原版 SetDryRun
  pub fn set_dry_run(&mut self, is_dry_run: bool) {
    self.is_dry_run = is_dry_run;
  }

  /// 原版 GenerateBuiltinDefinitionsAndInterfaceDescriptors(output_directory)
  pub fn generate_builtin_definitions_and_interface_descriptors(
    &mut self,
    cx: &mut GlobalContext,
    output_directory: &str,
  ) -> Result<()> {
    // TODO(填体)
    let _ = (cx, output_directory);
    Err(Error::Unimplemented)
  }

  /// 原版 GenerateBitFields(output_directory)
  pub fn generate_bit_fields(
    &mut self,
    cx: &mut GlobalContext,
    output_directory: &str,
  ) -> Result<()> {
    // TODO(填体)
    let _ = (cx, output_directory);
    Err(Error::Unimplemented)
  }

  /// 原版 GenerateClassDefinitions(output_directory)
  pub fn generate_class_definitions(
    &mut self,
    cx: &mut GlobalContext,
    output_directory: &str,
  ) -> Result<()> {
    // TODO(填体)
    let _ = (cx, output_directory);
    Err(Error::Unimplemented)
  }

  /// 原版 GenerateInstanceTypes(output_directory)
  pub fn generate_instance_types(
    &mut self,
    cx: &mut GlobalContext,
    output_directory: &str,
  ) -> Result<()> {
    // TODO(填体)
    let _ = (cx, output_directory);
    Err(Error::Unimplemented)
  }

  /// 原版 GenerateEnumVerifiers(output_directory)
  pub fn generate_enum_verifiers(
    &mut self,
    cx: &mut GlobalContext,
    output_directory: &str,
  ) -> Result<()> {
    // TODO(填体)
    let _ = (cx, output_directory);
    Err(Error::Unimplemented)
  }

  /// 原版 GenerateClassDebugReaders(output_directory)
  pub fn generate_class_debug_readers(
    &mut self,
    cx: &mut GlobalContext,
    output_directory: &str,
  ) -> Result<()> {
    // TODO(填体)
    let _ = (cx, output_directory);
    Err(Error::Unimplemented)
  }

  /// 原版 GenerateExportedMacrosAssembler(output_directory)
  pub fn generate_exported_macros_assembler(
    &mut self,
    cx: &mut GlobalContext,
    output_directory: &str,
  ) -> Result<()> {
    // TODO(填体)
    let _ = (cx, output_directory);
    Err(Error::Unimplemented)
  }

  /// 原版 GenerateCSATypes(output_directory)
  pub fn generate_csa_types(
    &mut self,
    cx: &mut GlobalContext,
    output_directory: &str,
  ) -> Result<()> {
    // TODO(填体)
    let _ = (cx, output_directory);
    Err(Error::Unimplemented)
  }

  /// 原版 Visit(Expression*) → VisitResult
  pub fn visit_expression(&mut self, cx: &mut GlobalContext, expr: NodeId) -> Result<VisitResult> {
    // TODO(填体)
    let _ = (cx, expr);
    Err(Error::Unimplemented)
  }

  /// 原版 Visit(Statement*) → const Type*
  pub fn visit_statement(&mut self, cx: &mut GlobalContext, stmt: NodeId) -> Result<TypeId> {
    // TODO(填体)
    let _ = (cx, stmt);
    Err(Error::Unimplemented)
  }

  /// 原版 GetLocationReference(Expression*)
  pub fn get_location_reference(
    &mut self,
    cx: &mut GlobalContext,
    location: NodeId,
  ) -> Result<LocationReference> {
    // TODO(填体)
    let _ = (cx, location);
    Err(Error::Unimplemented)
  }

  /// 原版 GenerateCall(callable, this, parameters, specialization_types, tail_call)
  pub fn generate_call(
    &mut self,
    cx: &mut GlobalContext,
    callable: DeclarableId,
    this_parameter: Option<LocationReference>,
    parameters: Arguments,
    specialization_types: &TypeVector,
    tail_call: bool,
  ) -> Result<VisitResult> {
    // TODO(填体)
    let _ = (
      cx,
      callable,
      this_parameter,
      parameters,
      specialization_types,
      tail_call,
    );
    Err(Error::Unimplemented)
  }

  /// 原版 InlineMacro(macro, this_reference, arguments, label_blocks)
  pub fn inline_macro(
    &mut self,
    cx: &mut GlobalContext,
    macro_: DeclarableId,
    this_reference: Option<LocationReference>,
    arguments: &[VisitResult],
    label_blocks: &[BlockId],
  ) -> Result<VisitResult> {
    // TODO(填体)
    let _ = (cx, macro_, this_reference, arguments, label_blocks);
    Err(Error::Unimplemented)
  }

  /// 原版 VisitAllDeclarables()
  pub fn visit_all_declarables(&mut self, cx: &mut GlobalContext) -> Result<()> {
    // TODO(填体)
    let _ = cx;
    Err(Error::Unimplemented)
  }

  /// 原版 Visit(Declarable*, file)
  pub fn visit_declarable(
    &mut self,
    cx: &mut GlobalContext,
    declarable: DeclarableId,
    file: Option<SourceId>,
  ) -> Result<()> {
    // TODO(填体)
    let _ = (cx, declarable, file);
    Err(Error::Unimplemented)
  }

  /// 原版 GenerateImplementation(dir)
  pub fn generate_implementation(&mut self, cx: &mut GlobalContext, dir: &str) -> Result<()> {
    // TODO(填体)
    let _ = (cx, dir);
    Err(Error::Unimplemented)
  }

  /// 原版 GetCommonType(left, right)
  pub fn get_common_type(&self, cx: &TypeOracle, left: TypeId, right: TypeId) -> Result<TypeId> {
    // TODO(填体)
    let _ = (cx, left, right);
    Err(Error::Unimplemented)
  }

  /// 原版 GenerateCopy(to_copy)
  pub fn generate_copy(&mut self, to_copy: &VisitResult) -> Result<VisitResult> {
    // TODO(填体)
    let _ = to_copy;
    Err(Error::Unimplemented)
  }

  /// 原版 GenerateImplicitConvert(destination_type, source)
  pub fn generate_implicit_convert(
    &mut self,
    cx: &TypeOracle,
    destination_type: TypeId,
    source: VisitResult,
  ) -> Result<VisitResult> {
    // TODO(填体)
    let _ = (cx, destination_type, source);
    Err(Error::Unimplemented)
  }

  /// 原版 GenerateBranch(condition, true_block, false_block)
  pub fn generate_branch(
    &mut self,
    condition: &VisitResult,
    true_block: BlockId,
    false_block: BlockId,
  ) -> Result<()> {
    // TODO(填体)
    let _ = (condition, true_block, false_block);
    Err(Error::Unimplemented)
  }

  /// 原版 GenerateBoolConstant(constant)
  pub fn generate_bool_constant(&mut self, constant: bool) -> Result<VisitResult> {
    // TODO(填体)
    let _ = constant;
    Err(Error::Unimplemented)
  }

  /// 原版 InferSpecializationTypes(generic, explicit_specialization_types, explicit_arguments)
  pub fn infer_specialization_types(
    &self,
    cx: &TypeOracle,
    generic: DeclarableId,
    explicit_specialization_types: &TypeVector,
    explicit_arguments: &TypeVector,
  ) -> Result<TypeArgumentInference> {
    // TODO(填体)
    let _ = (
      cx,
      generic,
      explicit_specialization_types,
      explicit_arguments,
    );
    Err(Error::Unimplemented)
  }

  /// 原版 WriteFile (dry-run 时跳过)
  pub fn write_file(&self, file: &str, content: &str) -> Result<()> {
    if self.is_dry_run {
      return Ok(());
    }
    replace_file_contents_if_different(file, content)?;
    Ok(())
  }
}

/// 原版 StackScope — 栈高度作用域守卫 (Yield 存活槽位); Rust 借用模型下
/// 改显式 begin/yield/close 三步 (登记偏差)。
#[derive(Debug)]
pub struct StackScope {
  pub base: BottomOffset,
  pub closed: bool,
}

impl StackScope {
  /// 原版构造 (记录当前栈高)
  #[must_use]
  pub fn begin(assembler: &CfgAssembler) -> Self {
    Self {
      base: assembler.current_stack().above_top(),
      closed: false,
    }
  }

  /// 原版 Yield (v 存活, 其余删除)
  pub fn yield_result(
    &mut self,
    assembler: &mut CfgAssembler,
    result: VisitResult,
  ) -> Result<VisitResult> {
    // TODO(填体)
    let _ = (assembler, result);
    Err(Error::Unimplemented)
  }

  /// 原版 Close
  pub fn close(&mut self, assembler: &mut CfgAssembler) -> Result<()> {
    // TODO(填体)
    let _ = assembler;
    self.closed = true;
    Ok(())
  }
}

/// 原版 BreakContinueActivator — break/continue 标号绑定对
#[derive(Debug)]
pub struct BreakContinueActivator {
  pub break_binding: BindingId,
  pub continue_binding: BindingId,
}

/// 原版 BindingsManagersScope — 绑定管理器作用域 (隐藏既有绑定)
#[derive(Debug, Default)]
pub struct BindingsManagersScope;

/// 原版 MacroInliningScope — 内联宏登记作用域
#[derive(Debug, Default)]
pub struct MacroInliningScope;

/// 原版 InitializerResults 1:1
#[derive(Debug, Clone, Default)]
pub struct InitializerResults {
  pub names: Vec<NodeId>,
  pub field_value_map: BTreeMap<String, VisitResult>,
}

/// 原版 LayoutForInitialization 1:1
#[derive(Debug, Clone, Default)]
pub struct LayoutForInitialization {
  pub array_lengths: BTreeMap<String, VisitResult>,
  pub offsets: BTreeMap<String, VisitResult>,
  pub size: VisitResult,
}

/// 原版 ReportAllUnusedMacros
pub fn report_all_unused_macros(cx: &GlobalContext) -> Result<()> {
  // TODO(填体)
  let _ = cx;
  Err(Error::Unimplemented)
}

// ── 原版 src/torque/implementation-visitor.h ──

// ── 原版 src/torque/implementation-visitor.cc ──
// L39: std::string Base64Encode(std::string_view input)
// L52: std::string KytheSourceFilePath(SourceId file)
// L69: void EmitKytheInlineMetadata( std::ostream& stream, const std::vector<GlobalContext::KytheInlineMetadata>& rules)
// L121: VisitResult ImplementationVisitor::Visit(Expression* expr)
// L134: const Type* ImplementationVisitor::Visit(Statement* stmt)
// L153: void ImplementationVisitor::BeginGeneratedFiles()
// L219: void ImplementationVisitor::EndGeneratedFiles()
// L243: void ImplementationVisitor::BeginDebugMacrosFile()
// L279: void ImplementationVisitor::EndDebugMacrosFile()
// L296: void ImplementationVisitor::Visit(NamespaceConstant* decl)
// L329: void ImplementationVisitor::Visit(TypeAlias* alias)
// L357: VisitResult ImplementationVisitor::InlineMacro( Macro* macro, std::optional<LocationReference> this_reference, const std::vector<VisitResult>& arguments, const std::vector<Block*>& label_blocks)
// L477: void ImplementationVisitor::VisitMacroCommon(Macro* macro)
// L616: void ImplementationVisitor::Visit(TorqueMacro* macro)
// L620: void ImplementationVisitor::Visit(Method* method)
// L627: std::string AddParameter(size_t i, Builtin* builtin, Stack<std::string>* parameters, Stack<const Type*>* parameter_types, BlockBindings<LocalValue>* parameter_bindings, bool mark_as_used)
// L650: void ImplementationVisitor::Visit(Builtin* builtin)
// L852: const Type* ImplementationVisitor::Visit(VarDeclarationStatement* stmt)
// L857: const Type* ImplementationVisitor::Visit( VarDeclarationStatement* stmt, BlockBindings<LocalValue>* block_bindings)
// L907: const Type* ImplementationVisitor::Visit(TailCallStatement* stmt)
// L911: VisitResult ImplementationVisitor::Visit(ConditionalExpression* expr)
// L951: VisitResult ImplementationVisitor::Visit(LogicalOrExpression* expr)
// L992: VisitResult ImplementationVisitor::Visit(LogicalAndExpression* expr)
// L1042: VisitResult ImplementationVisitor::Visit(IncrementDecrementExpression* expr)
// L1055: VisitResult ImplementationVisitor::Visit(AssignmentExpression* expr)
// L1073: VisitResult ImplementationVisitor::Visit(FloatingPointLiteralExpression* expr)
// L1081: VisitResult ImplementationVisitor::Visit(IntegerLiteralExpression* expr)
// L1090: VisitResult ImplementationVisitor::Visit(AssumeTypeImpossibleExpression* expr)
// L1103: VisitResult ImplementationVisitor::Visit(StringLiteralExpression* expr)
// L1109: VisitResult ImplementationVisitor::GetBuiltinCode(Builtin* builtin)
// L1123: VisitResult ImplementationVisitor::Visit(LocationExpression* expr)
// L1128: VisitResult ImplementationVisitor::Visit(FieldAccessExpression* expr)
// L1139: const Type* ImplementationVisitor::Visit(GotoStatement* stmt)
// L1169: const Type* ImplementationVisitor::Visit(IfStatement* stmt)
// L1258: const Type* ImplementationVisitor::Visit(WhileStatement* stmt)
// L1281: const Type* ImplementationVisitor::Visit(BlockStatement* block)
// L1298: const Type* ImplementationVisitor::Visit(DebugStatement* stmt)
// L1325: std::string FormatAssertSource(const std::string& str)
// L1342: const Type* ImplementationVisitor::Visit(AssertStatement* stmt)
// L1399: const Type* ImplementationVisitor::Visit(ExpressionStatement* stmt)
// L1404: const Type* ImplementationVisitor::Visit(ReturnStatement* stmt)
// L1451: VisitResult ImplementationVisitor::Visit(TryLabelExpression* expr)
// L1527: VisitResult ImplementationVisitor::Visit(StatementExpression* expr)
// L1531: InitializerResults ImplementationVisitor::VisitInitializerResults( const ClassType* class_type, const std::vector<NameAndExpression>& initializers)
// L1557: LocationReference ImplementationVisitor::GenerateFieldReference( VisitResult object, const Field& field, const ClassType* class_type, bool treat_optional_as_indexed)
// L1587: LocationReference ImplementationVisitor::GenerateFieldReferenceForInit( VisitResult object, const Field& field, const LayoutForInitialization& layout)
// L1611: void ImplementationVisitor::InitializeClass( const ClassType* class_type, VisitResult allocate_result, const InitializerResults& initializer_results, const LayoutForInitialization& layout)
// L1638: VisitResult ImplementationVisitor::GenerateArrayLength( Expression* array_length, Namespace* nspace, const std::map<std::string, LocalValue>& bindings)
// L1658: VisitResult ImplementationVisitor::GenerateArrayLength(VisitResult object, const Field& field)
// L1691: VisitResult ImplementationVisitor::GenerateArrayLength( const ClassType* class_type, const InitializerResults& initializer_results, const Field& field)
// L1714: LayoutForInitialization ImplementationVisitor::GenerateLayoutForInitialization( const ClassType* class_type, const InitializerResults& initializer_results)
// L1761: VisitResult ImplementationVisitor::Visit(NewExpression* expr)
// L1838: const Type* ImplementationVisitor::Visit(BreakStatement* stmt)
// L1848: const Type* ImplementationVisitor::Visit(ContinueStatement* stmt)
// L1858: const Type* ImplementationVisitor::Visit(ForLoopStatement* stmt)
// L1910: VisitResult ImplementationVisitor::Visit(SpreadExpression* expr)
// L1916: void ImplementationVisitor::GenerateImplementation(const std::string& dir)
// L1969: cpp::Function ImplementationVisitor::GenerateMacroFunctionDeclaration( Macro* macro)
// L2010: cpp::Function ImplementationVisitor::GenerateFunction( cpp::Class* owner, const std::string& name, const Signature& signature, const NameVector& parameter_names, bool pass_code_assembler_state, std::vector<std::string>* generated_parameter_names)
// L2098: void FailCallableLookup( const std::string& reason, const QualifiedName& name, const TypeVector& parameter_types, const std::vector<Binding<LocalLabel>*>& labels, const std::vector<Signature>& candidates, const std::vector<std::pair<GenericCallable*, std::string>> inapplicable_generics)
// L2131: Callable* GetOrCreateSpecialization( const SpecializationKey<GenericCallable>& key)
// L2142: std::optional<Binding<LocalValue>*> ImplementationVisitor::TryLookupLocalValue( const std::string& name)
// L2147: std::optional<Binding<LocalLabel>*> ImplementationVisitor::TryLookupLabel( const std::string& name)
// L2152: Binding<LocalLabel>* ImplementationVisitor::LookupLabel( const std::string& name)
// L2159: Block* ImplementationVisitor::LookupSimpleLabel(const std::string& name)
// L2172: bool ImplementationVisitor::TestLookupCallable( const QualifiedName& name, const TypeVector& parameter_types)
// L2178: TypeArgumentInference ImplementationVisitor::InferSpecializationTypes( GenericCallable* generic, const TypeVector& explicit_specialization_types, const TypeVector& explicit_arguments)
// L2305: Method* ImplementationVisitor::LookupMethod( const std::string& name, const AggregateType* receiver_type, const Arguments& arguments, const TypeVector& specialization_types)
// L2315: const Type* ImplementationVisitor::GetCommonType(const Type* left, const Type* right)
// L2329: VisitResult ImplementationVisitor::GenerateCopy(const VisitResult& to_copy)
// L2337: VisitResult ImplementationVisitor::Visit(StructExpression* expr)
// L2397: VisitResult ImplementationVisitor::GenerateSetBitField( const Type* bitfield_struct_type, const BitField& bitfield, VisitResult bitfield_struct, VisitResult value, bool starts_as_zero)
// L2407: LocationReference ImplementationVisitor::GetLocationReference( Expression* location)
// L2426: LocationReference ImplementationVisitor::GetLocationReference( FieldAccessExpression* expr)
// L2432: LocationReference ImplementationVisitor::GenerateFieldAccess( LocationReference reference, const std::string& fieldname, bool ignore_stuct_field_constness, std::optional<SourcePosition> pos)
// L2549: LocationReference ImplementationVisitor::GetLocationReference( ElementAccessExpression* expr)
// L2561: LocationReference ImplementationVisitor::GenerateReferenceToItemInHeapSlice( LocationReference slice, VisitResult index)
// L2575: LocationReference ImplementationVisitor::GetLocationReference( IdentifierExpression* expr)
// L2659: LocationReference ImplementationVisitor::GetLocationReference( DereferenceExpression* expr)
// L2670: VisitResult ImplementationVisitor::GenerateFetchFromLocation( const LocationReference& reference)
// L2722: void ImplementationVisitor::GenerateAssignToLocation( const LocationReference& reference, const VisitResult& assignment_value)
// L2803: VisitResult ImplementationVisitor::GeneratePointerCall( Expression* callee, const Arguments& arguments, bool is_tailcall)
// L2856: void ImplementationVisitor::AddCallParameter( Callable* callable, VisitResult parameter, const Type* parameter_type, std::vector<VisitResult>* converted_arguments, StackRange* argument_range, std::vector<std::string>* constexpr_arguments, bool inline_macro)
// L2878: std::pair<std::string, std::string> GetClassInstanceTypeRange( const ClassType* class_type)
// L2906: VisitResult ImplementationVisitor::GenerateCall( Callable* callable, std::optional<LocationReference> this_reference, Arguments arguments, const TypeVector& specialization_types, bool is_tailcall)
// L3367: VisitResult ImplementationVisitor::GenerateCall( const QualifiedName& callable_name, Arguments arguments, const TypeVector& specialization_types, bool is_tailcall)
// L3377: VisitResult ImplementationVisitor::Visit(CallExpression* expr, bool is_tailcall)
// L3432: VisitResult ImplementationVisitor::Visit(CallMethodExpression* expr)
// L3467: VisitResult ImplementationVisitor::Visit(IntrinsicCallExpression* expr)
// L3479: void ImplementationVisitor::GenerateBranch(const VisitResult& condition, Block* true_block, Block* false_block)
// L3487: VisitResult ImplementationVisitor::GenerateBoolConstant(bool constant)
// L3493: void ImplementationVisitor::GenerateExpressionBranch(Expression* expression, Block* true_block, Block* false_block)
// L3503: VisitResult ImplementationVisitor::GenerateImplicitConvert( const Type* destination_type, VisitResult source)
// L3546: StackRange ImplementationVisitor::GenerateLabelGoto( LocalLabel* label, std::optional<StackRange> arguments)
// L3551: std::vector<Binding<LocalLabel>*> ImplementationVisitor::LabelsFromIdentifiers( const std::vector<Identifier*>& names)
// L3571: StackRange ImplementationVisitor::LowerParameter( const Type* type, const std::string& parameter_name, Stack<std::string>* lowered_parameters)
// L3589: void ImplementationVisitor::LowerLabelParameter( const Type* type, const std::string& parameter_name, std::vector<std::string>* lowered_parameters)
// L3604: std::string ImplementationVisitor::ExternalLabelName( const std::string& label_name)
// L3609: std::string ImplementationVisitor::ExternalLabelParameterName( const std::string& label_name, size_t i)
// L3614: std::string ImplementationVisitor::ExternalParameterName( const std::string& name)
// L3619: bool IsCompatibleSignature(const Signature& sig, const TypeVector& types, size_t label_count)
// L3637: std::optional<Block*> ImplementationVisitor::GetCatchBlock()
// L3645: void ImplementationVisitor::GenerateCatchBlock( std::optional<Block*> catch_block)
// L3668: void ImplementationVisitor::VisitAllDeclarables()
// L3700: void ImplementationVisitor::Visit(Declarable* declarable, std::optional<SourceId> file)
// L3735: std::string MachineTypeString(const Type* type)
// L3748: void ImplementationVisitor::GenerateBuiltinDefinitionsAndInterfaceDescriptors( const std::string& output_directory)
// L3898: bool IsPointerSection(FieldSectionType type)
// L3905: std::string ToString(FieldSectionType type)
// L4069: void ImplementationVisitor::GenerateBitFields( const std::string& output_directory)
// L4223: void CppClassGenerator::GenerateCppObjectLayoutDefinitionAsserts()
// L4283: SourcePosition CppClassGenerator::Position()
// L4287: void ImplementationVisitor::GenerateClassDefinitions( const std::string& output_directory)
// L4320: void ImplementationVisitor::GenerateEnumVerifiers( const std::string& output_directory)
// L4365: void ImplementationVisitor::GenerateExportedMacrosAssembler( const std::string& output_directory)
// L4458: void CollectAllFields(const std::string& path, const Field& field, std::vector<std::string>& result)
// L4474: void ImplementationVisitor::GenerateCSATypes( const std::string& output_directory)
// L4519: void ReportAllUnusedMacros()
