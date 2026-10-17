// 由 tool/src/ast/scopes.cc.js 从 v8/src/ast/scopes.cc 生成; 修订就地改, 复跑覆盖
// 作用域解析定义头对数清单

/// scopes.cc 定义头清单 (对数基准, 共 126 项)。
/// 类 → 定义数: Scope=75, DeclarationScope=35, ModuleScope=3, ClassScope=12, UnresolvedList=1

// Scope::Scope(Zone* zone, ScopeType scope_type) : outer_scope_(nullptr), variables_(zone), scope_type_(scope_type)
// Scope::Scope(Zone* zone, Scope* outer_scope, ScopeType scope_type) : outer_scope_(outer_scope), variables_(zone), scope_type_(scope_type)
// Variable* Scope::DeclareHomeObjectVariable(AstValueFactory* ast_value_factory)
// Variable* Scope::DeclareStaticHomeObjectVariable( AstValueFactory* ast_value_factory)
// DeclarationScope::DeclarationScope(Zone* zone, AstValueFactory* ast_value_factory, REPLMode repl_mode) : Scope(zone, repl_mode == REPLMode::kYes ? REPL_MODE_SCOPE : SCRIPT_SCOPE), function_kind_(repl_mode == REPLMode::kYes ? FunctionKind::kAsyncFunction : FunctionKind::kNormalFunction), params_(4, zone)
// void DeclarationScope::SwitchScriptScopeToREPLMode()
// DeclarationScope::DeclarationScope(Zone* zone, Scope* outer_scope, ScopeType scope_type, FunctionKind function_kind) : Scope(zone, outer_scope, scope_type), function_kind_(function_kind), params_(4, zone)
// ModuleScope::ModuleScope(DeclarationScope* script_scope, AstValueFactory* avfactory) : DeclarationScope(avfactory->single_parse_zone(), script_scope, MODULE_SCOPE, FunctionKind::kModule), module_descriptor_( avfactory->single_parse_zone()->New<SourceTextModuleDescriptor>( avfactory->single_parse_zone()))
// ModuleScope::ModuleScope(Handle<ScopeInfo> scope_info, AstValueFactory* avfactory) : DeclarationScope(avfactory->single_parse_zone(), MODULE_SCOPE, avfactory, scope_info), module_descriptor_(nullptr)
// ClassScope::ClassScope(Zone* zone, Scope* outer_scope, bool is_anonymous) : Scope(zone, outer_scope, CLASS_SCOPE), rare_data_and_is_parsing_heritage_(nullptr)
// ClassScope::ClassScope(IsolateT* isolate, Zone* zone, AstValueFactory* ast_value_factory, Handle<ScopeInfo> scope_info) : Scope(zone, CLASS_SCOPE, ast_value_factory, scope_info), rare_data_and_is_parsing_heritage_(nullptr)
// Scope::Scope(Zone* zone, ScopeType scope_type, AstValueFactory* ast_value_factory, Handle<ScopeInfo> scope_info) : outer_scope_(nullptr), variables_(zone), scope_info_(scope_info), scope_type_(scope_type)
// DeclarationScope::DeclarationScope(Zone* zone, ScopeType scope_type, AstValueFactory* ast_value_factory, Handle<ScopeInfo> scope_info) : Scope(zone, scope_type, ast_value_factory, scope_info), function_kind_(scope_info->function_kind()), params_(0, zone)
// Scope::Scope(Zone* zone, const AstRawString* catch_variable_name, MaybeAssignedFlag maybe_assigned, Handle<ScopeInfo> scope_info) : outer_scope_(nullptr), variables_(zone), scope_info_(scope_info), scope_type_(CATCH_SCOPE)
// void DeclarationScope::SetDefaults()
// void Scope::SetDefaults()
// bool Scope::HasSimpleParameters()
// void DeclarationScope::set_should_eager_compile()
// bool Scope::is_debug_evaluate_scope() const
// Scope* Scope::DeserializeScopeChain( IsolateT* isolate, Zone* zone, Tagged<ScopeInfo> scope_info, DeclarationScope* script_scope, AstValueFactory* ast_value_factory, DeserializationMode deserialization_mode, Tagged<Script> script, Tagged<ScopeInfo> eval_outer_info, ParseInfo* parse_info)
// void Scope::SetScriptScopeInfo(IsolateT* isolate, DeclarationScope* script_scope)
// DeclarationScope* Scope::AsDeclarationScope()
// const DeclarationScope* Scope::AsDeclarationScope() const
// ModuleScope* Scope::AsModuleScope()
// const ModuleScope* Scope::AsModuleScope() const
// ClassScope* Scope::AsClassScope()
// const ClassScope* Scope::AsClassScope() const
// void DeclarationScope::DeclareSloppyBlockFunction( SloppyBlockFunctionStatement* sloppy_block_function)
// void DeclarationScope::HoistSloppyBlockFunctions(AstNodeFactory* factory)
// void DeclarationScope::TakeUnresolvedReferencesFromParent()
// bool DeclarationScope::Analyze(ParseInfo* info)
// void DeclarationScope::DeclareThis(AstValueFactory* ast_value_factory)
// void DeclarationScope::DeclareArguments(AstValueFactory* ast_value_factory)
// void DeclarationScope::DeclareDefaultFunctionVariables( AstValueFactory* ast_value_factory)
// Variable* DeclarationScope::DeclareFunctionVar(const AstRawString* name, Scope* cache)
// Variable* DeclarationScope::DeclareGeneratorObjectVar( const AstRawString* name)
// Scope* Scope::FinalizeBlockScope()
// void DeclarationScope::AddLocal(Variable* var)
// Variable* Scope::LookupInScopeInfo(const AstRawString* name, Scope* cache)
// Variable* DeclarationScope::DeclareParameter(const AstRawString* name, VariableMode mode, bool is_optional, bool is_rest, AstValueFactory* ast_value_factory, int position)
// void DeclarationScope::RecordParameter(bool is_rest)
// Variable* Scope::DeclareLocal(const AstRawString* name, VariableMode mode, VariableKind kind, bool* was_added, InitializationFlag init_flag)
// Variable* Scope::DeclareVariable( Declaration* declaration, const AstRawString* name, int pos, VariableMode mode, VariableKind kind, InitializationFlag init, bool* was_added, bool* sloppy_mode_block_scope_function_redefinition, bool* ok)
// Variable* Scope::DeclareVariableName(const AstRawString* name, VariableMode mode, bool* was_added, VariableKind kind)
// Variable* Scope::DeclareCatchVariableName(const AstRawString* name)
// void Scope::AddUnresolved(VariableProxy* proxy)
// Variable* DeclarationScope::DeclareDynamicGlobal(const AstRawString* name, VariableKind kind, Scope* cache)
// void Scope::DeleteUnresolved(VariableProxy* var)
// Variable* Scope::NewTemporary(const AstRawString* name)
// Variable* Scope::NewTemporary(const AstRawString* name, MaybeAssignedFlag maybe_assigned)
// Declaration* DeclarationScope::CheckConflictingVarDeclarations( bool* allowed_catch_binding_var_redeclaration)
// bool DeclarationScope::HasConflictingArgumentsVarDeclaration( AstValueFactory* ast_value_factory)
// const AstRawString* Scope::FindVariableDeclaredIn(Scope* scope, VariableMode mode_limit)
// void DeclarationScope::DeserializeReceiver(AstValueFactory* ast_value_factory)
// bool DeclarationScope::AllocateVariables(ParseInfo* info)
// bool Scope::HasReceiverToDeserialize() const
// bool Scope::HasThisReference() const
// bool Scope::AllowsLazyParsingWithoutUnresolvedVariables( const Scope* outer) const
// bool DeclarationScope::AllowsLazyCompilation() const
// int Scope::ContextChainLength(Scope* scope) const
// int Scope::ContextChainLengthUntilOutermostSloppyEval() const
// DeclarationScope* Scope::GetDeclarationScope()
// DeclarationScope* Scope::GetNonEvalDeclarationScope()
// const DeclarationScope* Scope::GetClosureScope() const
// DeclarationScope* Scope::GetClosureScope()
// bool Scope::NeedsScopeInfo() const
// bool Scope::ShouldBanArguments()
// DeclarationScope* Scope::GetReceiverScope()
// DeclarationScope* Scope::GetConstructorScope()
// Scope* Scope::GetHomeObjectScope()
// DeclarationScope* Scope::GetScriptScope()
// Scope* Scope::GetOuterScopeWithContext()
// void Scope::ForEach(FunctionType callback)
// bool Scope::IsConstructorScope() const
// bool Scope::IsOuterScopeOf(Scope* other) const
// bool Scope::IsOuterScopeUpToClosureScopeOf(Scope* scope) const
// void Scope::AnalyzePartially(DeclarationScope* max_outer_scope, AstNodeFactory* ast_node_factory, UnresolvedList* new_unresolved_list, bool maybe_in_arrowhead)
// void DeclarationScope::ResetAfterPreparsing(AstValueFactory* ast_value_factory, bool aborted)
// bool Scope::IsSkippableFunctionScope()
// void Scope::SavePreparseData(Parser* parser)
// void DeclarationScope::SavePreparseDataForDeclarationScope(Parser* parser)
// void DeclarationScope::AnalyzePartially(Parser* parser, AstNodeFactory* ast_node_factory, bool maybe_in_arrowhead)
// void Scope::RewriteReplGlobalVariables()
// void DeclarationScope::PrintParameters()
// void Scope::Print(int n)
// void Scope::CheckScopePositions()
// void Scope::CheckZones()
// Variable* Scope::NonLocal(const AstRawString* name, VariableMode mode)
// void Scope::ForceDynamicLookup(VariableProxy* proxy)
// Variable* Scope::Lookup(VariableProxy* proxy, Scope* scope, Scope* outer_scope_end, int* access_position, Scope* cache_scope, bool force_context_allocation)
// Variable* Scope::LookupWith(VariableProxy* proxy, Scope* scope, Scope* outer_scope_end, int* access_position, Scope* cache_scope, bool force_context_allocation)
// Variable* Scope::LookupSloppyEval(VariableProxy* proxy, Scope* scope, Scope* outer_scope_end, int* access_position, Scope* cache_scope, bool force_context_allocation)
// void Scope::ResolveVariable(VariableProxy* proxy)
// void Scope::ResolveTo(VariableProxy* proxy, Variable* var, int access_position)
// void Scope::ResolvePreparsedVariable(VariableProxy* proxy, Scope* scope, Scope* end)
// bool Scope::ResolveVariablesRecursively(Scope* end)
// void Scope::MarkMaybeAssignedIfEval(Variable* var)
// bool Scope::MustAllocate(Variable* var)
// bool Scope::MustAllocateInContext(Variable* var)
// void Scope::AllocateStackSlot(Variable* var)
// void Scope::AllocateHeapSlot(Variable* var)
// void DeclarationScope::AllocateParameterLocals()
// void DeclarationScope::AllocateParameter(Variable* var, int index)
// void DeclarationScope::AllocateReceiver()
// void Scope::AllocateNonParameterLocal(Variable* var)
// void Scope::AllocateNonParameterLocalsAndDeclaredGlobals()
// void DeclarationScope::AllocateLocals()
// void ModuleScope::AllocateModuleVariables()
// int Scope::UniqueIdInScript() const
// void Scope::AllocateVariablesRecursively()
// void Scope::AllocateScopeInfosRecursively( IsolateT* isolate, MaybeHandle<ScopeInfo> outer_scope, std::unordered_map<int, IndirectHandle<ScopeInfo>>& scope_infos_to_reuse)
// void DeclarationScope::RecalcPrivateNameContextChain()
// void DeclarationScope::RecordNeedsPrivateNameContextChainRecalc()
// void DeclarationScope::AllocateScopeInfos(ParseInfo* parse_info, DirectHandle<Script> script, IsolateT* isolate)
// int Scope::ContextLocalCount() const
// Variable* ClassScope::DeclarePrivateName(const AstRawString* name, VariableMode mode, IsStaticFlag is_static_flag, bool* was_added)
// Variable* ClassScope::LookupLocalPrivateName(const AstRawString* name)
// UnresolvedList::Iterator ClassScope::GetUnresolvedPrivateNameTail()
// void ClassScope::ResetUnresolvedPrivateNameTail(UnresolvedList::Iterator tail)
// void ClassScope::MigrateUnresolvedPrivateNameTail( AstNodeFactory* ast_node_factory, UnresolvedList::Iterator tail)
// Variable* ClassScope::LookupPrivateNameInScopeInfo(const AstRawString* name)
// Variable* ClassScope::LookupPrivateName(VariableProxy* proxy)
// bool ClassScope::ResolvePrivateNames(ParseInfo* info)
// VariableProxy* ClassScope::ResolvePrivateNamesPartially()
// Variable* ClassScope::DeclareBrandVariable(AstValueFactory* ast_value_factory, IsStaticFlag is_static_flag, int class_token_pos)
// Variable* ClassScope::DeclareClassVariable(AstValueFactory* ast_value_factory, const AstRawString* name, int class_token_pos)

pub mod scopes_cc_skel {
  /// 变量分配入口锚点 (原版 AllocateVariables / AllocateParameterLocals)
  pub const ALLOC_ANCHORS: [&str; 2] = ["Scope::AllocateVariables", "DeclarationScope::AllocateParameterLocals"];
  pub const DEF_TOTAL: usize = 126;
}
