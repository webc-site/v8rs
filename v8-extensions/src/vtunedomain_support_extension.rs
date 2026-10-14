// 由 tool/src/extensions/vtunedomain-support-extension.h.js 从 v8/src/extensions/vtunedomain-support-extension.{h,cc} 生成; 修订就地改, 复跑覆盖
// VTuneDomain 扩展: 7 错误码 #define 显式展开 + 分发表 + 签名骨架

//! VTuneDomainSupport 扩展 — 原版 namespace v8::internal::VTuneDomainSupportExtension + libvtune
//!
//! 语义: 注入 test(domain, task, start_or_end) — 经 libvtune 在 VTune 域上开始/结束任务;
//! 错误按位累加后以异常文本返回。

// —— #define 宏 → 显式常量 (位标志, 原版 1 << N) ——
/// 原版 #define UNKNOWN_PARAMS (1 << 0)
pub const UNKNOWN_PARAMS: u32 = 1 << 0; // = 1
/// 原版 #define NO_DOMAIN_NAME (1 << 1)
pub const NO_DOMAIN_NAME: u32 = 1 << 1; // = 2
/// 原版 #define CREATE_DOMAIN_FAILED (1 << 2)
pub const CREATE_DOMAIN_FAILED: u32 = 1 << 2; // = 4
/// 原版 #define NO_TASK_NAME (1 << 3)
pub const NO_TASK_NAME: u32 = 1 << 3; // = 8
/// 原版 #define CREATE_TASK_FAILED (1 << 4)
pub const CREATE_TASK_FAILED: u32 = 1 << 4; // = 16
/// 原版 #define TASK_BEGIN_FAILED (1 << 5)
pub const TASK_BEGIN_FAILED: u32 = 1 << 5; // = 32
/// 原版 #define TASK_END_FAILED (1 << 6)
pub const TASK_END_FAILED: u32 = 1 << 6; // = 64

/// VTuneDomain 扩展注册名
pub const VTUNEDOMAIN_SUPPORT_EXTENSION_NAME: &str = "v8/vtunedomain";

/// Mark 参数错误文案
pub const ERR_PARAMS: &str =
  "Parameter number should be exactly three, first domain namesecond task name, third start/end";

/// libvtune 分发表 (原版 function_map: 命令字 → 处理函数)
pub const FUNCTION_MAP: [(&str, &str); 2] = [("start", "startTask"), ("end", "endTask")];

// —— 提取不变式断言 ——
const _: () = {
  assert!(UNKNOWN_PARAMS == 1);
  assert!(NO_DOMAIN_NAME == 2);
  assert!(CREATE_DOMAIN_FAILED == 4);
  assert!(NO_TASK_NAME == 8);
  assert!(CREATE_TASK_FAILED == 16);
  assert!(TASK_BEGIN_FAILED == 32);
  assert!(TASK_END_FAILED == 64);
  assert!(FUNCTION_MAP.len() == 2);
};

// —— 签名骨架 (cc 定义 1:1) ——
// libvtune (namespace v8::internal::libvtune):
//   split(str: &str, delimiter: char, out: &mut Vec<String>)
//   startTask(vparams: &[String]) -> i32
//   endTask(vparams: &[String]) -> i32
//   invoke(params: &str) -> i32 — vparams[0] 查表分发, 未知命令累加 UNKNOWN_PARAMS
// 扩展本体:
//   GetNativeFunctionTemplate(isolate, name) -> FunctionTemplate
//   Mark(info) — 3 个 string 参数: (domain_name, task_name, "start"|"end"),
//                拼 "cmd domain task" 后交 libvtune::invoke, 非 0 则按数值抛错

// 依赖台账:
//   third_party/vtune/vtuneapi.h — VTuneDomain::createDomain / beginTask / endTask,
//   外部库闭源接口, Rust 侧经 FFI 承接点登记, 不在本模块落地
