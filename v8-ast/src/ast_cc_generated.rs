// 由 tool/src/ast/ast.cc.js 从 v8/src/ast/ast.cc + ast-value-factory.cc + modules.cc + variables.cc 生成; 修订就地改, 复跑覆盖
// AST 实现族定义头对数清单

// src/ast/ast.cc — Expression 定义头清单 (18 项):
// bool Expression::IsSmiLiteral() const
// bool Expression::IsNumberLiteral() const
// bool Expression::IsStringLiteral() const
// bool Expression::IsConsStringLiteral() const
// bool Expression::IsPropertyName() const
// bool Expression::IsNullLiteral() const
// bool Expression::IsBooleanLiteral() const
// bool Expression::IsTheHoleLiteral() const
// bool Expression::IsCompileTimeValue()
// bool Expression::IsUndefinedLiteral() const
// bool Expression::IsLiteralButNotNullOrUndefined() const
// bool Expression::ToBooleanIsTrue() const
// bool Expression::ToBooleanIsFalse() const
// bool Expression::IsPrivateName() const
// bool Expression::IsValidReferenceExpression() const
// bool Expression::IsAnonymousFunctionDefinition() const
// bool Expression::IsConciseMethodDefinition() const
// bool Expression::IsAccessorFunctionDefinition() const

// src/ast/ast.cc — FunctionLiteral 定义头清单 (14 项):
// void FunctionLiteral::set_raw_inferred_name(AstConsString* raw_inferred_name)
// Handle<String> FunctionLiteral::GetInferredName(Isolate* isolate)
// void FunctionLiteral::set_shared_function_info( Handle<SharedFunctionInfo> shared_function_info)
// bool FunctionLiteral::ShouldEagerCompile() const
// void FunctionLiteral::SetShouldEagerCompile()
// bool FunctionLiteral::AllowsLazyCompilation()
// int FunctionLiteral::start_position() const
// int FunctionLiteral::end_position() const { return scope()->end_position(); }  bool FunctionLiteral::is_hoisted_in_context() const
// bool FunctionLiteral::is_hoisted_in_context() const
// LanguageMode FunctionLiteral::language_mode() const
// FunctionKind FunctionLiteral::kind() const { return scope()->function_kind(); }  std::unique_ptr<char[]> FunctionLiteral::GetDebugName() const
// bool FunctionLiteral::private_name_lookup_skips_outer_class() const
// bool FunctionLiteral::class_scope_has_private_brand() const
// void FunctionLiteral::set_class_scope_has_private_brand(bool value)

// src/ast/ast.cc — AstNode 定义头清单 (2 项):
// IterationStatement* AstNode::AsIterationStatement()
// MaterializedLiteral* AstNode::AsMaterializedLiteral()

// src/ast/ast-value-factory.cc — AstRawString 定义头清单 (8 项):
// void AstRawString::Internalize(IsolateT* isolate)
// bool AstRawString::AsArrayIndex(uint32_t* index) const
// bool AstRawString::IsIntegerIndex() const
// bool AstRawString::IsOneByteEqualTo(const char* data) const
// uint16_t AstRawString::FirstCharacter() const
// bool AstRawString::Equal(const AstRawString* lhs, const AstRawString* rhs)
// int AstRawString::Compare(const AstRawString* lhs, const AstRawString* rhs)
// void AstRawString::Print() const { printf("%.*s", byte_length(), raw_data()); } #endif  // OBJECT_PRINT  template <typename IsolateT> Handle<String> AstConsString::Allocate(IsolateT* isolate) const

// src/ast/ast-value-factory.cc — AstValueFactory 定义头清单 (8 项):
// const AstRawString* AstValueFactory::GetOneByteStringInternal( base::Vector<const uint8_t> literal)
// const AstRawString* AstValueFactory::GetTwoByteStringInternal( base::Vector<const uint16_t> literal)
// const AstRawString* AstValueFactory::GetString( Tagged<String> literal, const SharedStringAccessGuardIfNeeded& access_guard)
// AstConsString* AstValueFactory::NewConsString()
// AstConsString* AstValueFactory::NewConsString(const AstRawString* str)
// AstConsString* AstValueFactory::NewConsString(const AstRawString* str1, const AstRawString* str2)
// void AstValueFactory::Internalize(IsolateT* isolate)
// const AstRawString* AstValueFactory::GetString( uint32_t raw_hash_field, bool is_one_byte, base::Vector<const uint8_t> literal_bytes)

// src/ast/modules.cc — SourceTextModuleDescriptor 定义头清单 (12 项):
// bool SourceTextModuleDescriptor::AddImport( const AstRawString* import_name, const AstRawString* local_name, const AstRawString* specifier, const ModuleImportPhase import_phase, const ImportAttributes* import_attributes, const Scanner::Location loc, const Scanner::Location specifier_loc, Zone* zone)
// bool SourceTextModuleDescriptor::AddStarImport( const AstRawString* local_name, const AstRawString* specifier, const ModuleImportPhase import_phase, const ImportAttributes* import_attributes, const Scanner::Location loc, const Scanner::Location specifier_loc, Zone* zone)
// void SourceTextModuleDescriptor::AddEmptyImport( const AstRawString* specifier, const ImportAttributes* import_attributes, const Scanner::Location specifier_loc, Zone* zone)
// void SourceTextModuleDescriptor::AddExport(const AstRawString* local_name, const AstRawString* export_name, Scanner::Location loc, Zone* zone)
// void SourceTextModuleDescriptor::AddExport( const AstRawString* import_name, const AstRawString* export_name, const AstRawString* specifier, const ImportAttributes* import_attributes, const Scanner::Location loc, const Scanner::Location specifier_loc, Zone* zone)
// void SourceTextModuleDescriptor::AddStarExport( const AstRawString* specifier, const ImportAttributes* import_attributes, const Scanner::Location loc, const Scanner::Location specifier_loc, Zone* zone)
// DirectHandle<FixedArray> SourceTextModuleDescriptor::SerializeRegularExports( IsolateT* isolate, Zone* zone) const
// void SourceTextModuleDescriptor::MakeIndirectExportsExplicit(Zone* zone)
// SourceTextModuleDescriptor::GetCellIndexKind(int cell_index)
// void SourceTextModuleDescriptor::AssignCellIndices()
// SourceTextModuleDescriptor::FindDuplicateExport(Zone* zone) const
// bool SourceTextModuleDescriptor::Validate( ModuleScope* module_scope, PendingCompilationErrorHandler* error_handler, Zone* zone)

// src/ast/variables.cc — Variable 定义头清单 (5 项):
// Variable::Variable(Variable* other) : scope_(other->scope_), name_(other->name_), local_if_not_shadowed_(nullptr), next_(nullptr), index_(other->index_), initializer_position_(other->initializer_position_), bit_field_(other->bit_field_)
// bool Variable::IsGlobalObjectProperty() const
// bool Variable::IsReplGlobal() const
// void Variable::RewriteLocationForRepl()
// void Variable::AssignHoleCheckBitmapIndex(ZoneVector<Variable*>& list, uint8_t next_index)

pub mod ast_cc_skel {
  /// 四文件定义头总数对数守卫
  pub const DEF_TOTAL: usize = 67;
}
