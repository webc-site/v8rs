// 由 tool/src/torque/generators.js 从 v8/src/torque/class-debug-reader-generator.cc 生成; 修订就地改, 复跑覆盖
// 类调试读取器生成器 (仅 .cc)

use crate::error::{Error, Result};

// 原版无独立头文件; 入口由 implementation_visitor.rs 的 GenerateClassDebugReaders 调用。
/// 原版 GenerateClassDebugReader 系列入口
pub fn generate_class_debug_readers(output_directory: &str) -> Result<()> {
  // TODO(填体)
  let _ = output_directory;
  Err(Error::Unimplemented)
}
// ── 原版 src/torque/class-debug-reader-generator.cc ──
// L28: public: ValueTypeFieldIterator(const Type* type, size_t index) : type_(type), index_(index)
// L38: const Result operator*() const
// L58: ValueTypeFieldIterator& operator++()
// L62: bool operator==(const ValueTypeFieldIterator& other) const
// L65: bool operator!=(const ValueTypeFieldIterator& other) const
// L77: public: explicit ValueTypeFieldsRange(const Type* type) : type_(type)
// L79: ValueTypeFieldIterator begin()
// L80: ValueTypeFieldIterator end()
// L106: public: explicit DebugFieldType(const Field& field) : name_and_type_(field.name_and_type), pos_(field.pos)
// L109: DebugFieldType(const NameAndType& name_and_type, const SourcePosition& pos) : name_and_type_(name_and_type), pos_(pos)
// L112: bool IsTagged() const
// L120: std::string GetValueType(TypeStorage storage) const
// L137: std::string GetOriginalType(TypeStorage storage) const
// L164: std::string GetTypeString(TypeStorage storage) const
// L180: size_t GetSize() const
// L191: std::string GetAddressGetter()
// L207: void GenerateFieldAddressAccessor(const Field& field, const std::string& class_name, std::ostream& h_contents, std::ostream& cc_contents)
// L251: void GenerateFieldValueAccessor(const Field& field, const std::string& class_name, std::ostream& h_contents, std::ostream& cc_contents)
// L351: void GenerateGetPropsChunkForField(const Field& field, std::ostream& get_props_impl, std::string class_name)
// L446: void GenerateClassDebugReader(const ClassType& type, std::ostream& h_contents, std::ostream& cc_contents, std::ostream& visitor, std::unordered_set<const ClassType*>* done)
// L522: void ImplementationVisitor::GenerateClassDebugReaders( const std::string& output_directory)
