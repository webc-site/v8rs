// 由 tool/src/wasm/wasm-js.h.js 从 v8/src/wasm/wasm-js.{h,cc} 生成; 修订就地改, 复跑覆盖
// WebAssembly JS API 函数表与安装面

//! JS 侧 WebAssembly 对象的构建与回调表。
//! v8: src/wasm/wasm-js.h WasmJs

/// 全部 JS 回调 (原版 X-macro WASM_JS_EXTERNAL_REFERENCE_LIST 展开, 32 项)
/// v8: src/wasm/wasm-js.h WASM_JS_EXTERNAL_REFERENCE_LIST
pub const WASM_JS_EXTERNAL_REFERENCES: &[&str] = &[
  "WebAssemblyCompile",
  "WebAssemblyException",
  "WebAssemblyExceptionGetArg",
  "WebAssemblyExceptionGetStack",
  "WebAssemblyExceptionIs",
  "WebAssemblyGlobal",
  "WebAssemblyGlobalGetValue",
  "WebAssemblyGlobalSetValue",
  "WebAssemblyGlobalValueOf",
  "WebAssemblyInstance",
  "WebAssemblyInstanceGetExports",
  "WebAssemblyInstantiate",
  "WebAssemblyMemory",
  "WebAssemblyMemoryMapDescriptor",
  "WebAssemblyMemoryGetBuffer",
  "WebAssemblyMemoryGrow",
  "WebAssemblyMemoryMapDescriptorMap",
  "WebAssemblyMemoryMapDescriptorUnmap",
  "WebAssemblyMemoryToFixedLengthBuffer",
  "WebAssemblyMemoryToResizableBuffer",
  "WebAssemblyModule",
  "WebAssemblyModuleCustomSections",
  "WebAssemblyModuleExports",
  "WebAssemblyModuleImports",
  "WebAssemblyTable",
  "WebAssemblyTableGet",
  "WebAssemblyTableGetLength",
  "WebAssemblyTableGrow",
  "WebAssemblyTableSet",
  "WebAssemblyTag",
  "WebAssemblySuspending",
  "WebAssemblyValidate",
];

/// 构造器对象名 (挂在 WebAssembly 上)
/// v8: src/wasm/wasm-js.cc Install*
pub const CONSTRUCTOR_NAMES: &[&str] = &[
  "Module",
  "Instance",
  "Memory",
  "Table",
  "Global",
  "Tag",
  "Exception",
  "MemoryMapDescriptor",
  "Suspending",
  "Suspender",
  "SuspendError",
];

/// 静态方法名
/// v8: src/wasm/wasm-js.cc
pub const STATIC_METHOD_NAMES: &[&str] = &[
  "compile",
  "compileStreaming",
  "instantiate",
  "instantiateStreaming",
  "validate",
];

/// 实例访问器名 (getter/setter 共用键)
/// v8: src/wasm/wasm-js.cc
pub const ACCESSOR_NAMES: &[&str] = &[
  "imports",
  "exports",
  "customSections",
  "buffer",
  "length",
  "value",
  "valueOf",
  "get",
  "set",
  "grow",
  "getArg",
  "is",
  "stack",
  "map",
  "unmap",
  "toFixedLengthBuffer",
  "toResizableBuffer",
];

/// 值类型名字符串 (toString 输出/parse 输入)
/// v8: src/wasm/wasm-js.cc 类型名表
pub const VALUE_TYPE_NAMES: &[&str] = &[
  "anyfunc",
  "anyref",
  "arrayref",
  "eqref",
  "exnref",
  "externref",
  "stringref",
  "structref",
];

/// 安装阶段
/// v8: src/wasm/wasm-js.h WasmJs::PrepareForSnapshot / Install / InstallConditionalFeatures
pub const INSTALL_PHASES: &[&str] = &[
  "PrepareForSnapshot",
  "Install",
  "InstallConditionalFeatures",
];

/// 条件安装 (flag 门控): Module/MemoryControl/JSPromiseIntegration
/// v8: src/wasm/wasm-js.h WasmJs 私有 Install*
pub const CONDITIONAL_INSTALLERS: &[&str] = &[
  "InstallModule",
  "InstallMemoryControl",
  "InstallJSPromiseIntegration",
];

// —— 提取不变式断言 ——
const _: () = {
  assert!(WASM_JS_EXTERNAL_REFERENCES.len() == 32);
  assert!(VALUE_TYPE_NAMES.len() == 8);
  assert!(INSTALL_PHASES.len() == 3);
  assert!(CONDITIONAL_INSTALLERS.len() == 3);
};
