// 由 tool/src/wasm/wasm-js.cc.js 从 v8/src/wasm/wasm-js.{h,cc} 生成; 修订就地改, 复跑覆盖
// WebAssembly JS API 安装表与 JS Tag 签名 + API 回调签名骨架 (头侧在 wasm_js.rs.skel)

//! WebAssembly JS API 绑定规则面。

/// 内建对象名常量
/// v8: src/wasm/wasm-js.cc kName_Wasm*
#[rustfmt::skip]
pub static OBJECT_NAMES: &[(&str, &str)] = &[
    ("kName_WasmGlobalObject", "WebAssembly.Global"),
    ("kName_WasmMemoryObject", "WebAssembly.Memory"),
    ("kName_WasmInstanceObject", "WebAssembly.Instance"),
    ("kName_WasmTableObject", "WebAssembly.Table"),
    ("kName_WasmExceptionPackage", "WebAssembly.Exception"),
];

/// WebAssembly 命名空间方法 (compile/validate/instantiate 恒装; streaming 对与 promising 另装)
/// v8: src/wasm/wasm-js.cc WasmJs::Install / InstallConditional
#[rustfmt::skip]
pub static NAMESPACE_METHODS: &[(&str, &str)] = &[
    ("compile", "1"),
    ("validate", "1"),
    ("instantiate", "1"),
    ("compileStreaming", "1 (流式)"),
    ("instantiateStreaming", "1 (流式)"),
    ("promising", "1 (WasmFX)"),
];

/// proto 方法/getter 安装表 (目标, 属性名, 回调)
/// v8: src/wasm/wasm-js.cc WasmJs::Install / InstallModule
#[rustfmt::skip]
pub static API_INSTALLS: &[(&str, &str, &str)] = &[
    ("webassembly", "compile", "WebAssemblyCompile"),
    ("webassembly", "validate", "WebAssemblyValidate"),
    ("webassembly", "instantiate", "WebAssemblyInstantiate"),
    ("table_proto", "grow", "WebAssemblyTableGrow"),
    ("table_proto", "set", "WebAssemblyTableSet"),
    ("table_proto", "get", "WebAssemblyTableGet"),
    ("memory_proto", "grow", "WebAssemblyMemoryGrow"),
    ("memory_proto", "toFixedLengthBuffer", "WebAssemblyMemoryToFixedLengthBuffer"),
    ("memory_proto", "toResizableBuffer", "WebAssemblyMemoryToResizableBuffer"),
    ("global_proto", "valueOf", "WebAssemblyGlobalValueOf"),
    ("exception_proto", "getArg", "WebAssemblyExceptionGetArg"),
    ("exception_proto", "is", "WebAssemblyExceptionIs"),
    ("module_constructor", "imports", "WebAssemblyModuleImports"),
    ("module_constructor", "exports", "WebAssemblyModuleExports"),
    ("module_constructor", "customSections", "WebAssemblyModuleCustomSections"),
    ("webassembly", "compileStreaming", "WebAssemblyCompileStreaming"),
    ("webassembly", "instantiateStreaming", "WebAssemblyInstantiateStreaming"),
    ("webassembly", "promising", "WebAssemblyPromising"),
    ("instance_proto", "exports", "WebAssemblyInstanceGetExports"),
    ("table_proto", "length", "WebAssemblyTableGetLength"),
    ("memory_proto", "buffer", "WebAssemblyMemoryGetBuffer"),
    ("exception_proto", "stack", "WebAssemblyExceptionGetStack"),
];

/// Global.value 为 getter+setter 成对安装
/// v8: src/wasm/wasm-js.cc WasmJs::Install
pub const GLOBAL_VALUE_ACCESSOR_PAIR: (&str, &str) =
  ("WebAssemblyGlobalGetValue", "WebAssemblyGlobalSetValue");

/// 错误构造器 (CompileError/LinkError/RuntimeError)
/// v8: src/wasm/wasm-js.cc WasmJs::Install
#[rustfmt::skip]
pub static ERROR_CONSTRUCTORS: &[(&str, &str)] = &[
    ("CompileError", "WASM_COMPILE_ERROR_FUNCTION_INDEX"),
    ("LinkError", "WASM_LINK_ERROR_FUNCTION_INDEX"),
    ("RuntimeError", "WASM_RUNTIME_ERROR_FUNCTION_INDEX"),
];

/// JS Tag 签名: 0 返回 + 1 个 externref 参数; 安装时 canonical index 从 0 起
/// v8: src/wasm/wasm-js.cc kWasmExceptionTagSignature / kInitialCanonicalTypeIndex
pub const JS_TAG_SIGNATURE: &str = "() -> (externref)";
pub const JS_TAG_INITIAL_CANONICAL_INDEX: u32 = 0;

/// 只读属性组合: DONT_ENUM | READ_ONLY
/// v8: src/wasm/wasm-js.cc ro_attributes
pub const RO_ATTRIBUTES: &str = "DONT_ENUM | READ_ONLY";

/// WebAssembly.Exception 私有字段: tag/values 两个 symbol (描述符预留 2)
/// v8: src/wasm/wasm-js.cc WasmJs::Install
#[rustfmt::skip]
pub static EXCEPTION_PRIVATE_FIELDS: &[&str] = &["wasm_exception_tag_symbol", "wasm_exception_values_symbol"];

/// 异步 resolver 的全局句柄常量声明 (6 处: Promise/Module/Imports 跨 5 类 resolver)
/// v8: src/wasm/wasm-js.cc AsyncCompilationResolver 系
pub const RESOLVER_GLOBAL_HANDLES: usize = 6;

// —— 提取不变式断言 ——
const _: () = {
  assert!(OBJECT_NAMES.len() == 5);
  assert!(NAMESPACE_METHODS.len() == 6);
  assert!(API_INSTALLS.len() == 22);
  assert!(ERROR_CONSTRUCTORS.len() == 3);
  assert!(EXCEPTION_PRIVATE_FIELDS.len() == 2);
  assert!(JS_TAG_INITIAL_CANONICAL_INDEX == 0);
  assert!(RESOLVER_GLOBAL_HANDLES == 6);
};

/// v8: src/wasm/wasm-js.cc WebAssembly* 回调清单 (对应原版 32 项 ExternalReference)
pub const fn webassembly_api_callbacks() -> &'static [&'static str] {
  crate::WASM_JS_EXTERNAL_REFERENCES
}
