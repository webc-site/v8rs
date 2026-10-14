// 由 tool/src/codegen/interface-descriptors.h.js 从 v8/src/codegen/interface-descriptors.{h,-inl.h,cc} 生成; 修订就地改, 复跑覆盖
// 接口描述符枚举与骨架

// 语义面: 描述符默认寄存器/初始化/派生类登记表 (inl/cc) 登记跳过。

/// 栈参数序 (原 enum class StackArgumentOrder)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum StackArgumentOrder {
  /// 首参在栈最高地址
  HighToLow = 0,
  /// 首参在栈最低地址
  LowToHigh = 1,
}

/// 原 kDefault = kHighToLow (手写汇编/builtin 历史默认)
pub const STACK_ARGUMENT_ORDER_DEFAULT: StackArgumentOrder = StackArgumentOrder::HighToLow;
/// JS 调用约定使用 kLowToHigh
pub const STACK_ARGUMENT_ORDER_JS: StackArgumentOrder = StackArgumentOrder::LowToHigh;

/// 原参数索引生成宏族 (展开为每描述符 enum ParameterIndices):
// - DEFINE_PARAMETERS_ENTRY
// - DEFINE_PARAMETERS
// - DEFINE_PARAMETERS_NO_CONTEXT
// - DEFINE_PARAMETERS_VARARGS
// - DEFINE_PARAMETERS_NO_CONTEXT_VARARGS

/// 原 CallInterfaceDescriptor::Flag
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DescriptorFlag {
  /// kNoFlags
  NoFlags,
  /// kNoContext
  NoContext,
  /// kNoStackScan
  NoStackScan,
  /// kAllowVarArgs
  AllowVarArgs,
  /// kCalleeSaveRegisters
  CalleeSaveRegisters,
}

/// 原 CallInterfaceDescriptor 类面骨架
pub struct CallInterfaceDescriptor;

impl CallInterfaceDescriptor {
  // 原签名: static constexpr auto DefaultRegisterArray()
  // 原签名: static constexpr auto DefaultDoubleRegisterArray()
  // 原签名: static constexpr auto DefaultReturnRegisterArray()
  // 原签名: static constexpr auto DefaultReturnDoubleRegisterArray()
  // 原签名: static constexpr Register ContextRegister()
  // TODO(dev): 依平台寄存器配置补默认数组。
}

// —— 提取不变式断言 ——
const _: () = {
  assert!(STACK_ARGUMENT_ORDER_DEFAULT == StackArgumentOrder::HighToLow);
};
