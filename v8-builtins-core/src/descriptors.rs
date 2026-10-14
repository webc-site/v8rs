// 由 tool/src/builtins/builtins-core.js 生成; 来源 v8/src/builtins 下 2 文件
// 内建名 → descriptor 名映射骨架
// 修订就地改, 复跑覆盖; 只含规则代码, 不含语义

/// 内建 descriptor 名 (Builtin_<Name>_InterfaceDescriptor)
pub fn descriptor_name(builtin: &str) -> String {
  format!("Builtin_{builtin}_InterfaceDescriptor")
}

/// builtins.cc 登记内建数: 0
pub const CC_BUILTIN_COUNT: usize = 0;
