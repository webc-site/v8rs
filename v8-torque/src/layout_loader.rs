// 由 tool/src/torque/misc.js 从 v8/src/torque/layout-loader.h 生成; 修订就地改, 复跑覆盖
// C++ 布局 JSON 加载/校验

use crate::error::{Error, Result};

/// 原版 VerifyCppLayouts: metagen layout JSON 逐记录比对 @cppObjectLayoutDefinition
/// 类的 base/base size/header size/全部字段; 不一致即 Torque 错误。
/// 必须在 TypeOracle::FinalizeAggregateTypes 之后运行。
pub fn verify_cpp_layouts(layout_json_path: &str) -> Result<()> {
  // TODO(填体)
  let _ = layout_json_path;
  Err(Error::Unimplemented)
}

/// 原版 ImportCppLayouts: 用 layout JSON 填充 @cppObjectLayoutDefinition 类字段;
/// 必须在解析之后、CompileCurrentAst 之前运行。
/// positions_path 可选: 有则诊断指向 C++ 成员, 无则指向 .tq 类声明。
pub fn import_cpp_layouts(layout_json_path: &str, positions_path: &str) -> Result<()> {
  // TODO(填体)
  let _ = (layout_json_path, positions_path);
  Err(Error::Unimplemented)
}

// ── 原版 src/torque/layout-loader.cc ──
// 签名清单 (1844 行, 校验逻辑大头)
// L35: const JsonValue* Lookup(const JsonObject& object, const std::string& key)
// L41: const JsonValue& Require(const JsonObject& object, const std::string& key, const std::string& context)
// L50: size_t RequireSize(const JsonObject& object, const std::string& key, const std::string& context)
// L67: const std::string& RequireString(const JsonObject& object, const std::string& key, const std::string& context)
// L78: std::string TorqueClassNameFromCppName(const std::string& cpp_name, const std::string& context)
// L88: std::string TorqueBitFieldStructName(const JsonObject& storage, const std::string& context)
// L157: std::string TorqueBitFieldName(const std::string& cpp_name, size_t num_bits, const std::string& context)
// L176: std::string TorqueTypeName(const std::string& cpp_name)
// L191: std::vector<std::string> SplitQualification(std::string* name)
// L229: bool AnnotationAllowedOn(const AnnotationSpec& spec, AnnotationOn on)
// L236: const JsonObject* FindAnnotation(const JsonObject& record, std::string_view name, const std::string& context)
// L256: bool HasAnnotation(const JsonObject& record, std::string_view name, const std::string& context)
// L261: const std::string* AnnotationArgument(const JsonObject& record, std::string_view name, const std::string& context)
// L270: void ValidateAnnotations(const JsonObject& record, AnnotationOn on, const std::string& context)
// L307: std::string TorqueClassName(const JsonObject& record)
// L312: std::string TorqueFieldName(const JsonObject& record, const std::string& context)
// L340: std::string_view CategoryName(StorageCategory category)
// L357: std::optional<StorageCategory> CategoryOfStorageKind(const std::string& kind)
// L371: StorageCategory CategoryOfTorqueType(const Type* type)
// L391: std::string_view SynchronizationName(FieldSynchronization synchronization)
// L402: std::optional<size_t> ConstantIndexExtent(const Field& field)
// L411: void FlattenUnion(TypeExpression* type, std::vector<std::string>* members)
// L413: std::string TypeExpressionKey(TypeExpression* type)
// L448: void FlattenUnion(TypeExpression* type, std::vector<std::string>* members)
// L459: std::string ExpressionKey(Expression* expr)
// L523: std::string IndexKey(const ClassFieldIndexInfo& index)
// L530: std::string TorqueSuperName(TypeExpression* super)
// L535: std::string FieldKey(const ClassFieldExpression& field)
// L550: JsonValue LoadLayoutDocument(const std::string& layout_json_path)
// L578: void VerifyConfig(const JsonValue& document_value, const std::string& layout_json_path)
// L615: RecordIndexes IndexRecords(const JsonValue& document_value)
// L655: bool OptionalBool(const JsonObject& record, const std::string& key, const std::string& context)
// L666: FieldSynchronization ReadSynchronization(const JsonObject& record, const std::string& context)
// L679: const JsonObject* LookupTail(const JsonObject& record, const std::string& name)
// L701: TailRepresentation GetTailRepresentation(const JsonObject& class_record, const JsonObject* tail_record, const std::string& context)
// L764: const JsonArray& RequireFields(const JsonObject& group, const std::string& name)
// L773: const JsonObject& RequireFieldObject(const JsonValue& field, const std::string& name)
// L781: std::vector<const JsonObject*> CollectFields( const std::vector<const JsonObject*>& groups, const std::string& name)
// L793: public: void Run(const std::string& layout_json_path)
// L807: private: void VerifyClass(const JsonObject& record)
// L829: void VerifyBase(const ClassType* type, const JsonObject& record, const std::string& name)
// L868: void VerifyFields(const ClassType* type, const std::vector<const JsonObject*>& groups, const JsonObject& record, const std::string& name)
// L961: void VerifyField(const std::string& class_name, const Field& field, const JsonObject& record)
// L1024: void VerifyBitFields(const std::string& context, const Field& field, const JsonObject& record)
// L1087: void VerifyTail(const std::string& class_name, const Field& field, const JsonObject& class_record, const JsonObject& record)
// L1133: void VerifyCategory(const std::string& context, const Field& field, const JsonObject& record, const std::string& key)
// L1155: void VerifyFlags(const std::string& context, const Field& field, const JsonObject& record)
// L1186: public: void Run(const std::string& layout_json_path, const std::string& positions_path)
// L1207: void CollectBitFieldStructs(const std::vector<Declaration*>& declarations)
// L1219: void ImportBitFieldStructs(const JsonObject& record, const std::string& cpp_name, SourcePosition fallback)
// L1270: bool BitFieldStructsEquivalent( const BitFieldStructDeclaration& decl, TypeExpression* parent, const std::vector<BitFieldDeclaration>& built)
// L1301: static std::string BitFieldKey(const BitFieldDeclaration& field)
// L1308: TypeExpression* BitFieldParent(const std::string& context, const JsonObject& storage)
// L1316: std::vector<BitFieldDeclaration> BitFieldEntries( const std::string& context, const JsonObject& storage, const std::vector<SourcePosition>* positions = nullptr)
// L1353: private: void LoadPositions(const std::string& path)
// L1386: SourcePosition MemberPosition(const std::string& class_cpp_name, const std::string& member_cpp_name, SourcePosition fallback)
// L1396: void Import(const std::vector<Declaration*>& declarations)
// L1409: void ImportClass(ClassDeclaration* decl)
// L1471: std::vector<ClassFieldExpression> BuildFields( const std::string& class_name, const JsonObject& class_record, const std::vector<const JsonObject*>& groups, const JsonObject* tail_record, TailRepresentation tail_representation, SourcePosition default_position, const std::vector<SourcePosition>* positions)
// L1532: bool FieldsEquivalent(const std::string& class_name, const std::vector<ClassFieldExpression>& declared, const std::vector<ClassFieldExpression>& built)
// L1560: ClassFieldExpression BuildField(const std::string& class_name, const JsonObject& record, SourcePosition position)
// L1592: ClassFieldExpression BuildTail(const std::string& class_name, const JsonObject& class_record, const JsonObject& record, SourcePosition position)
// L1615: TypeExpression* BuildType(const std::string& context, const JsonObject& record, const std::string& key)
// L1624: TypeExpression* NamedType(const std::string& name, TypeExpression* argument = nullptr)
// L1636: TypeExpression* DeriveType(const std::string& context, const JsonObject& storage)
// L1679: TypeExpression* BuildFieldType(const std::string& context, const JsonObject& record, const std::string& storage_key)
// L1699: const char* ClassTemplateTorqueName(const std::string& name, const JsonObject& type)
// L1731: TypeExpression* BuildTypeExpression(const std::string& context, const JsonObject& type)
// L1800: SourcePosition ParsePosition(const std::string& position)
// L1835: void VerifyCppLayouts(const std::string& layout_json_path)
// L1839: void ImportCppLayouts(const std::string& layout_json_path, const std::string& positions_path)
