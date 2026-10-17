// 由 tool/src/builtins/builtins-core.js 生成; 来源 v8/src/builtins 下 1 文件
// 模块索引
// 修订就地改, 复跑覆盖; 只含规则代码, 不含语义

#![forbid(unsafe_code)]

use v8_codegen_core::{CallInterfaceDescriptorData, CallInterfaceDescriptorFlags};
mod descriptors;
mod infra;
mod kind;
mod table;

pub use descriptors::{CC_BUILTIN_COUNT, descriptor_name};
pub use infra::TQ_INFRA_DECLS;
pub use kind::{
  BUILTIN_LIST_BASE_TIER0_USE_COUNT, BUILTIN_LIST_BASE_TIER1_USE_COUNT,
  BUILTIN_LIST_INTL_USE_COUNT, BUILTIN_LIST_TEMPORAL_USE_COUNT, BuiltinKind,
};
pub use table::{
  BUILTIN_TABLE, BUILTIN_TABLE_LEN, count_by_kind, find_builtin, get_builtin, is_tier0,
};
mod error;
pub use error::{Error, Result};

/// 桥接: 从 BuiltinKind 映射到调用接口描述符数据 (真实消费 BuiltinKind)
pub fn get_builtin_descriptor(kind: BuiltinKind) -> v8_codegen_core::CallInterfaceDescriptorData {
  let mut data = CallInterfaceDescriptorData::new();
  match kind {
    BuiltinKind::TFJ | BuiltinKind::TFJTSA => {
      data.flags = CallInterfaceDescriptorFlags::ALLOW_VAR_ARGS;
      data.param_count = v8_codegen_core::K_JS_BUILTIN_REGISTER_PARAMS;
    }
    BuiltinKind::TFC | BuiltinKind::TFCTSA | BuiltinKind::TFS => {
      data.flags = CallInterfaceDescriptorFlags::CALLEE_SAVE_REGISTERS;
      data.param_count = v8_codegen_core::K_MAX_BUILTIN_REGISTER_PARAMS;
    }
    BuiltinKind::CPP | BuiltinKind::ASM | BuiltinKind::BCH | BuiltinKind::TFH => {
      data.param_count = 1;
    }
  }
  data
}
