#![forbid(unsafe_code)]
#![cfg_attr(docsrs, feature(doc_cfg))]

mod error;
pub use error::{Error, Result};

mod ast;
pub use ast::{
  AbstractTypeDeclaration, Annotation, AnnotationParameter, AssertKind, AssertStatement,
  AssignmentExpression, AssumeTypeImpossibleExpression, Ast, AstNode, AstNodeKind,
  BasicTypeExpression, BitFieldDeclaration, BitFieldStructDeclaration, BlockStatement,
  BreakStatement, CallExpression, CallMethodExpression, ClassBody, ClassDeclaration,
  ClassFieldExpression, ClassFieldIndexInfo, ConditionalAnnotation, ConditionalAnnotationType,
  ConditionalExpression, ConstDeclaration, ContinueStatement, CppIncludeDeclaration, DebugKind,
  DebugStatement, DereferenceExpression, ElementAccessExpression, EnumDescription, EnumEntryDesc,
  ExpressionStatement, ExternConstDeclaration, ExternalBuiltinDeclaration,
  ExternalMacroDeclaration, ExternalRuntimeDeclaration, FieldAccessExpression,
  FloatingPointLiteralExpression, ForLoopStatement, FunctionTypeExpression,
  GenericCallableDeclaration, GenericParameter, GenericParameters, GenericTypeDeclaration,
  GotoStatement, HandlerKind, Identifier, IdentifierExpression, IfStatement, ImplicitKind,
  ImplicitParameters, IncludeSelector, IncrementDecrementExpression, IncrementDecrementOperator,
  InstanceTypeConstraints, IntegerLiteral, IntegerLiteralExpression, IntrinsicCallExpression,
  IntrinsicDeclaration, K_THIS_PARAMETER_NAME, LabelAndTypes, LabelAndTypesVector,
  LogicalAndExpression, LogicalOrExpression, NameAndExpression, NameAndTypeExpression,
  NamespaceDeclaration, NewExpression, NodeHeader, NodeId, ParameterList,
  PrecomputedTypeExpression, ReturnStatement, SpecializationDeclaration, SpreadExpression,
  StatementExpression, StringLiteralExpression, StructDeclaration, StructExpression,
  StructFieldExpression, TailCallStatement, TorqueBuiltinDeclaration, TorqueMacroDeclaration,
  TryHandler, TryLabelExpression, TypeAliasDeclaration, TypeswitchCase, TypeswitchStatement,
  UnionTypeExpression, VarDeclarationStatement, WhileStatement,
};

mod ast_visitor;
pub use ast_visitor::AstVisitor;

mod cc_generator;
pub use cc_generator::CcGenerator;

mod cfg;
pub use cfg::{Block, CfgAssembler, CfgAssemblerScopedTemporaryBlock, ControlFlowGraph};

mod class_debug_reader_generator;
pub use class_debug_reader_generator::generate_class_debug_readers;

mod constants;
pub use constants::{
  ANNOTATION_ABSTRACT, ANNOTATION_CPP_ACQUIRE_LOAD, ANNOTATION_CPP_OBJECT_LAYOUT_DEFINITION,
  ANNOTATION_CPP_RELAXED_LOAD, ANNOTATION_CPP_RELAXED_STORE, ANNOTATION_CPP_RELEASE_STORE,
  ANNOTATION_CPP_SCOPE, ANNOTATION_CUSTOM_CPP_CLASS, ANNOTATION_CUSTOM_INTERFACE_DESCRIPTOR,
  ANNOTATION_CUSTOM_MAP, ANNOTATION_CUSTOM_WEAK_MARKING, ANNOTATION_DO_NOT_GENERATE_CAST,
  ANNOTATION_DO_NOT_GENERATE_CPP_CLASS, ANNOTATION_DO_NOT_GENERATE_INSTANCE_TYPE_CHECK,
  ANNOTATION_EXPORT, ANNOTATION_HAS_SAME_INSTANCE_TYPE_AS_PARENT,
  ANNOTATION_HIGHEST_INSTANCE_TYPE_WITHIN_PARENT, ANNOTATION_IF, ANNOTATION_IFNOT,
  ANNOTATION_INCREMENT_USE_COUNTER, ANNOTATION_INSTANCE_TYPE_VALUE,
  ANNOTATION_LOWEST_INSTANCE_TYPE_WITHIN_PARENT, ANNOTATION_RESERVE_BITS_IN_INSTANCE_TYPE,
  ANNOTATION_SAME_ENUM_VALUE_AS, ANNOTATION_SUPPORTS_TSA, ANNOTATION_USE_PARENT_TYPE_CHECKER,
  ARGUMENTS_TYPE_STRING, AbstractTypeFlag, AbstractTypeFlags, BINT_TYPE_STRING, BOOL_TYPE_STRING,
  BUILTIN_POINTER_TYPE_STRING, CHAR8_TYPE_STRING, CHAR16_TYPE_STRING, CONST_FLOAT64_TYPE_STRING,
  CONST_INT31_TYPE_STRING, CONST_INT32_TYPE_STRING, CONST_REFERENCE_TYPE_STRING,
  CONST_SLICE_TYPE_STRING, CONST_STRING_TYPE_STRING, CONSTEXPR_BOOL_TYPE_STRING,
  CONSTEXPR_INSTANCE_TYPE_TYPE_STRING, CONSTEXPR_INTPTR_TYPE_STRING, CONSTEXPR_STRING_TYPE_STRING,
  CONSTEXPR_TYPE_PREFIX, CONTEXT_TYPE_STRING, CPPHEAPPTR_TYPE_STRING, ClassFlag, ClassFlags,
  DISPATCH_HANDLE_TYPE_STRING, EXTERNALPTR_TYPE_STRING, FIXED_ARRAY_BASE_TYPE_STRING,
  FLOAT16_RAW_BITS_TYPE_STRING, FLOAT32_TYPE_STRING, FLOAT64_OR_UNDEFINED_OR_HOLE_TYPE_STRING,
  FLOAT64_TYPE_STRING, FieldSynchronization, GENERIC_TYPE_INSTANTIATION_NAMESPACE_STRING,
  HEAP_OBJECT_TYPE_STRING, INT8_TYPE_STRING, INT16_TYPE_STRING, INT31_TYPE_STRING,
  INT32_TYPE_STRING, INT64_TYPE_STRING, INTEGER_LITERAL_TYPE_STRING, INTPTR_TYPE_STRING,
  JS_FUNCTION_TYPE_STRING, JSANY_TYPE_STRING, JSOBJECT_TYPE_STRING, LAZY_TYPE_STRING,
  MAP_TYPE_STRING, MUTABLE_REFERENCE_TYPE_STRING, MUTABLE_SLICE_TYPE_STRING,
  NATIVE_CONTEXT_TYPE_STRING, NEVER_TYPE_STRING, NO_CONTEXT_TYPE_STRING, NUMBER_TYPE_STRING,
  OBJECT_TYPE_STRING, PROTECTEDPTR_TYPE_STRING, RAWPTR_TYPE_STRING, SMI_TAGGED_TYPE_STRING,
  SMI_TYPE_STRING, STATIC_ASSERT_MACRO_STRING, STRING_TYPE_STRING, STRONG_TAGGED_TYPE_STRING,
  StructFlag, StructFlags, TAGGED_TYPE_STRING, TAGGED_ZERO_PATTERN_TYPE_STRING,
  TORQUE_INTERNAL_NAMESPACE_STRING, TRUSTEDPTR_TYPE_STRING, UINT8_TYPE_STRING, UINT16_TYPE_STRING,
  UINT31_TYPE_STRING, UINT32_TYPE_STRING, UINT64_TYPE_STRING, UINTPTR_TYPE_STRING,
  UNINITIALIZED_HEAP_OBJECT_TYPE_STRING, UNINITIALIZED_ITERATOR_TYPE_STRING, VOID_TYPE_STRING,
  WEAK_HEAP_OBJECT, WEAK_TYPE_STRING, get_constexpr_name, get_non_constexpr_name,
  is_constexpr_name,
};

mod context;
pub use context::{
  CurrentScopeGuard, SourceFileScope, SourcePositionScope, absolute_path, add_source, build_flag,
  current_scope, current_source_file, current_source_position, error_continue, error_continue_at,
  file_relative_to_v8_root_exists, get_source_file_map, get_source_id, lint, lint_at, replace_ast,
  replace_language_server_data, replace_source_file_map, replace_type_oracle, report_error,
  set_target_architecture, tagged_size, take_ast, take_messages, with_ast,
  with_language_server_data, with_type_oracle, with_type_oracle_mut,
};

mod cpp_builder;
pub use cpp_builder::{
  Class, File, Function, FunctionFlag, FunctionFlags, IncludeGuardScope, Parameter,
  TemplateParameter,
};

mod csa_generator;
pub use csa_generator::{ARGUMENTS_VARIABLE_STRING, CsaGenerator};

mod declarable;
pub use declarable::{
  BuiltinData, BuiltinFlag, BuiltinFlags, BuiltinKind, CallableCommon, Declarable, DeclarableData,
  DeclarableId, DeclarableKind, ExternConstantData, ExternMacroData, GenericCallableData,
  GenericTypeData, IntrinsicData, MacroCommon, MethodData, NamespaceConstantData, NamespaceData,
  OutputType, QualifiedName, RuntimeFunctionData, ScopeData, SpecializationRequester,
  TorqueMacroData, TypeAliasData, TypeConstraint, ValueCommon, add_callable_specialization,
  add_declarable, add_type_specialization, builtin_declaration_flags, callable_body,
  callable_common_of, callable_declaration_parameter_types, compute_constraints, current_namespace,
  display_builtin, display_callable, display_generic_callable, display_parameter_types,
  display_runtime_function, display_type_vector, find_constraint_violation, generic_constraints,
  generic_declaration_node, generic_name, generic_parameters, get_callable_specialization,
  get_type_specialization, identifier_value, infer_specialization_types, is_builtin_declaration,
  is_intrinsic_declaration, is_macro_declaration, kind_is_scope, lookup_shallow, make_declarable,
  make_specialization_requester, namespace_is_default, namespace_is_test, resolve_type_alias,
  scope_lookup, set_specialization_requester, type_declaration_name,
};

mod declarable_cc_generated;
pub use declarable_cc_generated::{declarable_cc_skel, declarable_kinds};

mod declaration_visitor;
pub use declaration_visitor::{DeclarationVisitor, PredeclarationVisitor, get_or_create_namespace};

mod declarations;
pub use declarations::{
  BREAK_LABEL_NAME, BuiltinSpec, CATCH_LABEL_NAME, CONTINUE_LABEL_NAME, Declarations,
  FROM_CONSTEXPR_MACRO_NAME, MACRO_END_LABEL_NAME, MacroSpec, NEXT_CASE_LABEL_NAME,
};

mod declarations_cc_generated;
pub use declarations_cc_generated::declarations_cc_skel;

mod earley_parser;
pub use earley_parser::{
  Action, Grammar, GrammarTables, InputPosition, Item, ItemId, ItemKey, Lexer, LexerResult,
  MatchedInput, ParseResult, ParseResultIterator, ParseResultTypeId, ParseResultValue,
  PatternFunction, Rule, RuleId, Symbol, SymbolId, default_action, item_matched_input,
  parse_tokens, run_earley_algorithm, yield_false, yield_matched_input, yield_true,
};

mod global_context;
pub use global_context::{
  CppInclude, GlobalContext, KytheInlineMetadata, PerFileStreams, TargetArchitecture,
};

mod implementation_visitor;
pub use implementation_visitor::{
  Arguments, Binding, BindingId, BindingValue, BindingsManager, BindingsManagersScope,
  BlockBindings, BreakContinueActivator, ImplementationVisitor, InitializerResults,
  LayoutForInitialization, LocalLabel, LocalValue, LocationReference, MacroInliningScope,
  NextUniqueBindingIndex, StackScope, is_compatible_signature, report_all_unused_macros,
};

mod instance_type_generator;
pub use instance_type_generator::generate_instance_types;

mod instructions;
pub use instructions::{
  AbortInstruction, AbortKind, BlockId, BranchInstruction, CallBuiltinInstruction,
  CallBuiltinPointerInstruction, CallCsaMacroAndBranchInstruction, CallCsaMacroInstruction,
  CallIntrinsicInstruction, CallRuntimeInstruction, ConstexprBranchInstruction, DefinitionLocation,
  DefinitionLocationKind, DeleteRangeInstruction, GotoExternalInstruction, GotoInstruction,
  InsnRef, Instruction, InstructionKind, LoadBitFieldInstruction, LoadReferenceInstruction,
  MakeLazyNodeInstruction, NamespaceConstantInstruction, PeekInstruction, PokeInstruction,
  PrintErrorInstruction, PushBuiltinPointerInstruction, PushUninitializedInstruction,
  ReturnInstruction, StoreBitFieldInstruction, StoreReferenceInstruction, UnsafeCastInstruction,
  abort_kind_to_string,
};

mod kythe_data;
pub use kythe_data::{KytheConsumer, KytheData, KytheEntity, KytheKind, KythePosition};

mod layout_loader;
pub use layout_loader::{import_cpp_layouts, verify_cpp_layouts};

mod parameter_difference;
pub use parameter_difference::ParameterDifference;

mod runtime_macro_shims;

mod runtime_support;

mod server_data;
pub use server_data::{DefinitionsMap, LanguageServerData, Symbols, SymbolsMap};

mod source_positions;
pub use source_positions::{
  LineAndColumn, SourceFileMap, SourceId, SourcePosition, position_as_string,
};

mod torque;
pub use torque::run;

mod torque_code_generator;
pub use torque_code_generator::TorqueCodeGenerator;

mod torque_compiler;
pub use torque_compiler::{
  TorqueCompilationUnit, TorqueCompilerOptions, TorqueCompilerResult, compile_torque,
  compile_torque_files, compile_torque_for_kythe,
};

mod torque_parser;
pub use torque_parser::{
  EnumEntry, ExpressionWithSource, GRAMMAR_TOKENS, TorqueGrammar, TypeList, make_call,
  parse_torque, parse_torque_class_fields, parse_torque_expression, parse_torque_type_expression,
  yield_decrement, yield_default_parameter_list, yield_empty_label_types_vec, yield_increment,
};

mod tsa_generator;
pub use tsa_generator::{TsaGenerator, generate_tsa};

mod type_inference;
pub use type_inference::TypeArgumentInference;

mod type_oracle;
pub use type_oracle::TypeOracle;

mod type_oracle_cc_generated;
pub use type_oracle_cc_generated::{type_oracle_cc_skel, type_oracle_surface};

mod type_visitor;
pub use type_visitor::TypeVisitor;

mod type_visitor_cc_generated;
pub use type_visitor_cc_generated::{type_visitor_cc_skel, type_visitor_surface};

mod types;
pub use types::{
  AbstractType, AggregateCommon, BitField, BitFieldStructType, BuiltinPointerType, ClassType,
  Field, LabelDeclaration, LabelDefinition, MaybeSpecializationKey, NameAndType, NameAndTypeVector,
  ObjectSlotKind, ParameterMode, ParameterTypes, Signature, SpecializationKey, StructType,
  StructTypeClassificationFlag, TopType, Type, TypeChecker, TypeData, TypeHandleKind, TypeId,
  TypeKind, TypeVector, UnionType, VisitResult, combine_object_slot_kind, common_supertype,
  compute_name, compute_type_vector, extract_simple_field_array_size, is_32_bit_integral_type,
  is_allowed_as_bit_field, is_any_unsigned_integer, is_assignable_from,
  is_pointer_size_integral_type, is_subtype_of, lower_parameter_types, lower_type,
  lowered_slot_count, project_struct_field, size_of, subtract_type,
};

mod types_cc_generated;
pub use types_cc_generated::types_cc_skel;

mod utils;
pub use utils::{
  BottomOffset, Deduplicator, K_BASE_NAMESPACE_NAME, K_TEST_NAMESPACE_NAME, MessageKind,
  ResidueClass, Stack, StackRange, TorqueMessage, Worklist, camelify_string,
  capify_string_with_underscores, dashify_string, file_uri_decode, is_lower_camel_case,
  is_snake_case, is_upper_camel_case, is_valid_namespace_const_name, is_valid_type_name,
  replace_file_contents_if_different, snakeify_string, starts_with_single_underscore,
  string_literal_quote, string_literal_unquote, underlinify_path,
};

mod ls;
pub use ls::{
  CompilerHandler, Diagnostic, DiagnosticSeverity, DidChangeWatchedFilesParams,
  DidChangeWatchedFilesRegistrationOptions, FileEvent, FileListParams, InitializeParams,
  InitializeResult, JsonArray, JsonMember, JsonObject, JsonParserResult, JsonPosition, JsonValue,
  Location, Logger, Message, MessageHandler, PublishDiagnosticsParams, Range, Registration,
  RegistrationParams, ResponseError, SaveOptions, ServerCapabilities, SymbolKind,
  TextDocumentIdentifier, TextDocumentPositionParams, TextDocumentSyncOptions, TorqueFileList,
  WatchKind, parse_json, read_message, run_language_server, serialize_to_string, write_message,
};
