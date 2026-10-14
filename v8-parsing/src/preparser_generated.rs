// 由 tool/src/parsing/preparser.cc.js 从 v8/src/parsing/preparser.{h,cc} + preparse-data.{h,cc,-impl.h} + preparser-logger.h 生成; 修订就地改, 复跑覆盖
// 预解析族骨架

/// 预解析器结构锚点 (原版 preparser.h: 惰性解析, 只记函数边界与跳过区)
pub const PREPARSER_ANCHORS: [&str; 3] = ["PreParser", "PreParserIdentifier", "PreParserExpression"];

/// 预解析数据结构锚点 (原版 preparse-data.h/-impl.h)
pub const PREPARSE_DATA_ANCHORS: [&str; 3] = ["PreParserData", "UnresolvedData", "PreparseDataBuilder"];

/// 预解析日志器 (原版 preparser-logger.h)
pub const PREPARSER_LOGGER_ANCHOR: bool = true;

// preparser.cc PreParser 定义头清单 (7 项):
// PreParserIdentifier PreParser::GetIdentifier() const
// PreParser::PreParseResult PreParser::PreParseProgram()
// PreParser::PreParseResult PreParser::PreParseFunction( int function_literal_id, const AstRawString* function_name, FunctionKind kind, FunctionSyntaxKind function_syntax_kind, DeclarationScope* function_scope, int* use_counts, ProducedPreparseData** produced_preparse_data)
// PreParser::Expression PreParser::ParseFunctionLiteral( Identifier function_name, Scanner::Location function_name_location, FunctionNameValidity function_name_validity, FunctionKind kind, int function_token_pos, FunctionSyntaxKind function_syntax_kind, LanguageMode language_mode, ZonePtrList<const AstRawString>* arguments_for_wrapped_function)
// void PreParser::ParseStatementListAndLogFunction( int function_literal_id, PreParserFormalParameters* formals)
// PreParserBlock PreParser::BuildParameterInitializationBlock( const PreParserFormalParameters& parameters)
// bool PreParser::IdentifierEquals(const PreParserIdentifier& identifier, const AstRawString* other)

// preparse-data.cc 定义头清单 (15 项):
// PreparseDataBuilder::PreparseDataBuilder(Zone* zone, PreparseDataBuilder* parent_builder, std::vector<void*>* children_buffer) : parent_(parent_builder), byte_data_(), children_buffer_(children_buffer), function_scope_(nullptr), function_length_(-1), num_inner_functions_(0), num_inner_with_data_(0), bailed_out_(false), has_data_(false)
// bool PreparseDataBuilder::HasInnerFunctions() const
// bool PreparseDataBuilder::HasData() const { return !bailed_out_ && has_data_; }  bool PreparseDataBuilder::HasDataForParent() const
// bool PreparseDataBuilder::HasDataForParent() const
// void PreparseDataBuilder::AddChild(PreparseDataBuilder* child)
// void PreparseDataBuilder::FinalizeChildren(Zone* zone)
// bool PreparseDataBuilder::ScopeNeedsData(Scope* scope)
// bool PreparseDataBuilder::SaveDataForSkippableFunction( PreparseDataBuilder* builder)
// void PreparseDataBuilder::SaveScopeAllocationData(DeclarationScope* scope, Parser* parser)
// void PreparseDataBuilder::SaveDataForScope(Scope* scope)
// void PreparseDataBuilder::SaveDataForVariable(Variable* var)
// void PreparseDataBuilder::SaveDataForInnerScopes(Scope* scope)
// Handle<PreparseData> PreparseDataBuilder::Serialize(Isolate* isolate)
// Handle<PreparseData> PreparseDataBuilder::Serialize(LocalIsolate* isolate)
// ZonePreparseData* PreparseDataBuilder::Serialize(Zone* zone)
