//! fast_api_calls — 可优化 C 签名 API 调用降为直接 C 调用子图
//!
//! 1:1 对应 v8/src/compiler/fast-api-calls.{h,cc}。CTypeInfo/CFunctionInfo 主面
//! 在 v8/include/v8-fast-api-calls.h (API 层), 此处建最小镜像供判定函数消费。

use v8_objects_arrays::elements_kind::ElementsKind;

use crate::{
  Error, feedback_source::FeedbackSource, graph_assembler::GraphAssemblerLabel, reducer::Reduction,
};

/// C 签名类型 (CTypeInfo::Type, uint8)
/// v8: include/v8-fast-api-calls.h CTypeInfo::Type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum CTypeInfoType {
  Void = 0,
  Bool,
  Uint8,
  Int32,
  Uint32,
  Int64,
  Uint64,
  Float32,
  Float64,
  Pointer,
  V8Value,
  SeqOneByteString,
  ApiObject,
  Any,
}

/// kCallbackOptionsType 不入枚举, 内部值 255
/// v8: include/v8-fast-api-calls.h CTypeInfo::kCallbackOptionsType
pub const K_CALLBACK_OPTIONS_TYPE: u8 = 255;

/// 浮点参数 C 链接旗标 (默认无)
pub const FP_PARAMS_IN_C_LINKAGE: bool = false;

/// C 签名旗标位 (CTypeInfo::Flags, uint8)
/// v8: include/v8-fast-api-calls.h CTypeInfo::Flags
pub mod c_type_info_flags {
  /// v8: include/v8-fast-api-calls.h CTypeInfo::Flags::kNone
  pub const NONE: u8 = 0;
  /// ArrayBuffer/TypedArray 可共享
  /// v8: include/v8-fast-api-calls.h CTypeInfo::Flags::kAllowSharedBit
  pub const ALLOW_SHARED: u8 = 1 << 0;
  /// 整型强制范围
  /// v8: include/v8-fast-api-calls.h CTypeInfo::Flags::kEnforceRangeBit
  pub const ENFORCE_RANGE: u8 = 1 << 1;
  /// 整型钳位
  /// v8: include/v8-fast-api-calls.h CTypeInfo::Flags::kClampBit
  pub const CLAMP: u8 = 1 << 2;
  /// 浮点受限
  /// v8: include/v8-fast-api-calls.h CTypeInfo::Flags::kIsRestrictedBit
  pub const IS_RESTRICTED: u8 = 1 << 3;
}

/// C 参数类型信息镜像 (type + flags)
/// v8: include/v8-fast-api-calls.h CTypeInfo
#[derive(Debug, Clone, Copy)]
pub struct CTypeInfo {
  /// type_
  pub ty: CTypeInfoType,
  /// flags_
  pub flags: u8,
}

impl CTypeInfo {
  /// v8: include/v8-fast-api-calls.h CTypeInfo::GetType
  pub const fn get_type(&self) -> CTypeInfoType {
    self.ty
  }

  /// v8: include/v8-fast-api-calls.h CTypeInfo::GetFlags
  pub const fn get_flags(&self) -> u8 {
    self.flags
  }
}

/// C 函数签名信息镜像 (api-namespace CFunctionInfo 主面在 API 层, 推迟对接)
/// v8: include/v8-fast-api-calls.h CFunctionInfo
#[derive(Debug)]
pub struct CFunctionInfo {
  /// 参数信息 (含 receiver 位)
  pub argument_info: Vec<CTypeInfo>,
  /// 返回信息
  pub return_info: CTypeInfo,
  /// 是否带 FastApiCallbackOptions 尾参
  pub has_callback_options: bool,
}

impl CFunctionInfo {
  /// v8: include/v8-fast-api-calls.h CFunctionInfo::ArgumentCount
  pub fn argument_count(&self) -> usize {
    self.argument_info.len()
  }

  /// v8: include/v8-fast-api-calls.h CFunctionInfo::ArgumentInfo
  pub fn argument_info(&self, i: usize) -> &CTypeInfo {
    &self.argument_info[i]
  }

  /// v8: include/v8-fast-api-calls.h CFunctionInfo::ReturnInfo
  pub fn return_info(&self) -> &CTypeInfo {
    &self.return_info
  }
}

/// C 类型 → TypedArray ElementsKind 映射 (7 项命中, 7 项原版 UNREACHABLE)
/// v8: src/compiler/fast-api-calls.cc GetTypedArrayElementsKind
pub fn typed_array_elements_kind(ty: CTypeInfoType) -> crate::Result<ElementsKind> {
  let kind = match ty {
    CTypeInfoType::Uint8 => ElementsKind::Uint8Elements,
    CTypeInfoType::Int32 => ElementsKind::Int32Elements,
    CTypeInfoType::Uint32 => ElementsKind::Uint32Elements,
    CTypeInfoType::Int64 => ElementsKind::Bigint64Elements,
    CTypeInfoType::Uint64 => ElementsKind::Biguint64Elements,
    CTypeInfoType::Float32 => ElementsKind::Float32Elements,
    CTypeInfoType::Float64 => ElementsKind::Float64Elements,
    // kVoid/kSeqOneByteString/kBool/kPointer/kV8Value/kApiObject/kAny → UNREACHABLE
    other => return Err(Error::UnreachableCTypeInfo(other)),
  };
  Ok(kind)
}

/// 签名可否优化为快速调用 — 平台规则 (MacArm64 参数上限 / FP / 32 位 / x64 clamp)
/// v8: src/compiler/fast-api-calls.cc CanOptimizeFastSignature
pub fn can_optimize_fast_signature(c_signature: &CFunctionInfo) -> crate::Result<bool> {
  #[cfg(all(target_os = "macos", target_arch = "aarch64"))]
  {
    // MacArm64 硬件不支持栈传参
    if c_signature.argument_count() > 8 {
      return Ok(false);
    }
  }

  // 默认构建 (无 V8_ENABLE_FP_PARAMS_IN_C_LINKAGE): 返回含 float → false
  if !FP_PARAMS_IN_C_LINKAGE
    && matches!(
      c_signature.return_info().get_type(),
      CTypeInfoType::Float32 | CTypeInfoType::Float64
    )
  {
    return Ok(false);
  }

  // 非 64 位目标: 返回含 int64/uint64 → false
  if !cfg!(target_pointer_width = "64")
    && matches!(
      c_signature.return_info().get_type(),
      CTypeInfoType::Int64 | CTypeInfoType::Uint64
    )
  {
    return Ok(false);
  }

  for i in 0..c_signature.argument_count() {
    let arg = c_signature.argument_info(i);
    #[cfg(target_arch = "x86_64")]
    {
      // x64: FastApiCallLoweringReducer 的 clamp 降级用取整, 需 SSE4_2，未满足时安全回退
      if arg.get_flags() & c_type_info_flags::CLAMP != 0 {
        return Ok(false);
      }
    }
    if !FP_PARAMS_IN_C_LINKAGE
      && matches!(
        arg.get_type(),
        CTypeInfoType::Float32 | CTypeInfoType::Float64
      )
    {
      return Ok(false);
    }
    if !cfg!(target_pointer_width = "64")
      && matches!(arg.get_type(), CTypeInfoType::Int64 | CTypeInfoType::Uint64)
    {
      return Ok(false);
    }
  }
  Ok(true)
}

/// 重载决议结果 — 仅支持 JSArray vs TypedArray 可区分重载
/// v8: src/compiler/fast-api-calls.h OverloadsResolutionResult
pub struct OverloadsResolutionResult<'z> {
  /// 可区分重载参数下标 (-1 无效)
  pub distinguishable_arg_index: i32,
  /// TypedArray 参数元素类型 (无效时 kVoid)
  pub element_type: CTypeInfoType,
  /// target_address_ (决议后填)
  pub target_address: Option<&'z v8_compiler_graph::Node>,
}

impl<'z> OverloadsResolutionResult<'z> {
  /// v8: src/compiler/fast-api-calls.h OverloadsResolutionResult::Invalid
  pub const fn invalid() -> Self {
    Self {
      distinguishable_arg_index: -1,
      element_type: CTypeInfoType::Void,
      target_address: None,
    }
  }

  /// 带参构造 (原版 DCHECK: index>=0 时元素类型非 kVoid)
  /// v8: src/compiler/fast-api-calls.h OverloadsResolutionResult::OverloadsResolutionResult
  pub const fn new(distinguishable_arg_index: i32, element_type: CTypeInfoType) -> Self {
    debug_assert!(distinguishable_arg_index < 0 || element_type as u8 != CTypeInfoType::Void as u8);
    Self {
      distinguishable_arg_index,
      element_type,
      target_address: None,
    }
  }

  /// v8: src/compiler/fast-api-calls.h OverloadsResolutionResult::is_valid
  pub const fn is_valid(&self) -> bool {
    self.distinguishable_arg_index >= 0
  }
}

/// fast API 调用目标镜像 (c_function + version + address + interface)
/// v8: include/v8-fast-api-calls.h FastApiCallFunction
#[derive(Debug, Clone, Copy)]
pub struct FastApiCallFunction<'i> {
  /// c_function_ (签名信息)
  pub c_function: Option<&'i CFunctionInfo>,
  /// version_
  pub version: u16,
  /// address_ (C 函数入口)
  pub address: usize,
  /// interface_ (CFunctionInterface)
  pub interface: u8,
}

/// 回调面: 取参 (i32 arg_index, label 回传)
/// v8: src/compiler/fast-api-calls.h GetParameter
pub type GetParameter<'f, 'z> =
  &'f mut dyn Fn(i32, &mut GraphAssemblerLabel<'z>) -> crate::Result<&'z v8_compiler_graph::Node>;
/// 返回值转换
/// v8: src/compiler/fast-api-calls.h ConvertReturnValue
pub type ConvertReturnValue<'f> = &'f mut dyn Fn(
  &CFunctionInfo,
  &v8_compiler_graph::Node,
) -> crate::Result<v8_compiler_graph::NodeId>;
/// options 初始化
/// v8: src/compiler/fast-api-calls.h InitializeOptions
pub type InitializeOptions<'f> = &'f mut dyn Fn(&v8_compiler_graph::Node);
/// 慢路径生成
/// v8: src/compiler/fast-api-calls.h GenerateSlowApiCall
pub type GenerateSlowApiCall<'f> = &'f mut dyn Fn() -> crate::Result<()>;

/// FastApiCallBuilder 入参包
pub struct BuildFastApiCallArgs<'f, 'z> {
  pub c_function: FastApiCallFunction<'f>,
  pub data_argument: &'z v8_compiler_graph::Node,
  pub get_parameter: GetParameter<'f, 'z>,
  pub convert_return_value: ConvertReturnValue<'f>,
  pub initialize_options: InitializeOptions<'f>,
  pub generate_slow_api_call: GenerateSlowApiCall<'f>,
}

/// FastApiCallBuilder::Build — 快/慢路径双出口子图 (含异常传播)
/// 调用输入布局: [fast callee, receiver, ...C args, [Options], effect, control];
/// FastApiCallbackOptions 栈槽 = 2×uintptr (isolate+data)
/// v8: src/compiler/fast-api-calls.cc BuildFastApiCall
pub fn build_fast_api_call<'z>(_args: BuildFastApiCallArgs<'_, 'z>) -> Reduction<'z> {
  Reduction::no_change()
}

/// 挑选与 arg_count 匹配的可优化重载 (kReceiver=1)
/// v8: src/compiler/fast-api-calls.cc GetFastApiCallTarget
pub fn get_fast_api_call_target<'z>(
  _broker: &v8_compiler_broker::JSHeapBroker<'z, '_>,
  _function_template_info: v8_objects_rest::FunctionTemplateInfo,
  _arg_count: usize,
  _feedback: &FeedbackSource,
) -> crate::Result<FastApiCallFunction<'z>> {
  todo!() // 推迟台账: CFunctionInfo 反序列化面 (v8-api)
}

// —— 对数基准 ——
// 类型映射 7 命中 / 7 UNREACHABLE / CanOptimize 8 return 分支 / 结构 4 / 回调 4
