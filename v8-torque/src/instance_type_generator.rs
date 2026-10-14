// 由 tool/src/torque/generators.js 从 v8/src/torque/instance-type-generator.cc 生成; 修订就地改, 复跑覆盖
// InstanceType 生成器 (仅 .cc)

use crate::error::{Error, Result};

// 原版无独立头文件; 入口由 implementation_visitor.rs 的 GenerateInstanceTypes 调用。
/// 原版 GenerateInstanceTypes 系列
pub fn generate_instance_types(output_directory: &str) -> Result<()> {
  // TODO(填体)
  let _ = output_directory;
  Err(Error::Unimplemented)
}
// ── 原版 src/torque/instance-type-generator.cc ──
// L17: explicit InstanceTypeTree(const ClassType* type) : type(type), start(INT_MAX), end(INT_MIN), value(-1), num_values(0), num_own_values(0)
// L35: std::unique_ptr<InstanceTypeTree> BuildInstanceTypeTree()
// L77: void PropagateInstanceTypeConstraints(InstanceTypeTree* root)
// L112: int SelectOwnValues(InstanceTypeTree* root, int start_value)
// L127: constexpr bool operator()(const InstanceTypeTree* a, const InstanceTypeTree* b) const
// L142: int SolveInstanceTypeConstraints( std::unique_ptr<InstanceTypeTree> root, int start_value, std::vector<std::unique_ptr<InstanceTypeTree>>* destination)
// L270: std::unique_ptr<InstanceTypeTree> SolveInstanceTypeConstraints( std::unique_ptr<InstanceTypeTree> root)
// L277: std::unique_ptr<InstanceTypeTree> AssignInstanceTypes()
// L312: void PrintInstanceTypes(InstanceTypeTree* root, std::ostream& definitions, std::ostream& values, std::ostream& debug_reader_classes_single, std::ostream& debug_reader_classes_multiple, std::ostream& debug_reader_classes_range, std::ostream& it_list_single, std::ostream& it_list_multiple, std::ostream& it_list_range, const std::string& indent)
// L388: void ImplementationVisitor::GenerateInstanceTypes( const std::string& output_directory)
