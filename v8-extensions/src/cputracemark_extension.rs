// 由 tool/src/extensions/cputracemark-extension.h.js 从 v8/src/extensions/cputracemark-extension.{h,cc} 生成; 修订就地改, 复跑覆盖
// CpuTraceMark 扩展元数据 + 签名骨架

//! CpuTraceMark 扩展 — 原版 namespace v8::internal::CpuTraceMarkExtension
//!
//! 语义: 通过 cpuid 叶子 0x4711|(param<<16) 标记 CPU 轨迹点 (仅 IA32/X64 有实际副作用)。
//! arm64 目标: 原版函数体在非 IA32/X64 下为空 (整段 #if 剥除) — 本模块登记为空操作。

/// CpuTraceMark 扩展注册名 (原版 v8::Extension 构造第一参)
pub const CPUTRACEMARK_EXTENSION_NAME: &str = "v8/cpumark";

/// 注入 JS 的源码模板 (原版 BuildSource, SNPrintF 格式串)
pub const CPUTRACEMARK_SOURCE_FORMAT: &str = "native function %s();";

/// 栈上源码缓冲容量 (原版 char buffer_[50])
pub const CPUTRACEMARK_BUFFER_SIZE: usize = 50;

/// Mark 参数错误文案 (原版 ThrowError 字面量)
pub const ERR_FIRST_PARAM: &str = "First parameter to cputracemark() must be a unsigned int32.";

/// cpuid 魔数叶子 (原版 0x4711 | (param << 16))
pub const CPUID_LEAF_BASE: u32 = 0x4711;

// —— 提取不变式断言 ——
const _: () = {
  assert!(CPUTRACEMARK_BUFFER_SIZE == 50);
  assert!(CPUID_LEAF_BASE == 0x4711);
};

// —— 方法签名骨架 (原版声明 3 / 定义 2, 1:1) ——
// public override: GetNativeFunctionTemplate(isolate, name) -> FunctionTemplate
// private static: Mark(info) — 校验 info[0] 为 uint32 后发 cpuid
// private static: BuildSource(buf, size, fun_name) -> *const c_char

// 平台分支台账:
//   cc #if V8_HOST_ARCH_IA32 || V8_HOST_ARCH_X64 → x64/arm64 目标中仅 x64 展开
//     (clang 路径: "cpuid" : "=a"(dummy) : "a"(leaf) : "ecx","edx","ebx")
//   __i386__ && __pic__ 变体 (push %%ebx) — 32 位 PIC 专属, arm64/x64 目标跳过
//   MSVC 路径 (#else / 无 msvc 构建) — 本迁移不涉
//   arm64: 无 cpuid 汇编, Mark 为参数校验后直返 — 空实现登记
