// 由 tool/src/wasm/module-instantiate.cc.js 从 v8/src/wasm/module-instantiate.{h,cc} 生成; 修订就地改, 复跑覆盖
// 实例化阶段序与导入校验错误消息表 + InstanceBuilder 方法签名骨架

//! 模块实例化规则面。

/// 实例化阶段序: Build 总控 -> Phase1 (可失败) -> Phase1_Infallible -> Phase2
/// v8: src/wasm/module-instantiate.cc InstanceBuilder
#[rustfmt::skip]
pub static INSTANTIATE_PHASES: &[&str] = &[
    "Build",
    "Build_Phase1",
    "Build_Phase1_Infallible",
    "Build_Phase2",
];

/// JS 函数导入包装签名: 机器签名跳过第一个隐式参数 (param_offset = 1)
/// v8: src/wasm/module-instantiate.cc 包装签名构造
pub const JS_WRAPPER_PARAM_OFFSET: u32 = 1;

/// 导入校验 LinkError 消息表 (%s 前缀统一为导入名 "module.field")
/// v8: src/wasm/module-instantiate.cc ProcessImports 系
#[rustfmt::skip]
pub static IMPORT_LINK_ERRORS: &[&str] = &[
    "%s: import not found",
    "%s: function import requires a callable",
    "%s: imported function does not match the expected type",
    "%s: table import requires a WebAssembly.Table",
    "table import %d is smaller than initial %u, got %u",
    "table import %d has no maximum length; required: %",
    "table import %d has a larger maximum size %",
    "cannot import %s table as %s",
    "%s: imported table does not match the expected type",
    "%s: imported global does not match the expected mutability",
    "%s: imported global does not match the expected type",
    "%s: global import of type v128 must be a WebAssembly.Global",
    "%s: imported mutable global must be a WebAssembly.Global object",
    "%s: %s",
    "%s: global import must be a number, valid Wasm reference, or ",
    "%s: tag import requires a WebAssembly.Tag",
    "%s: imported tag does not match the expected type",
    "%s: memory import must be a WebAssembly.Memory object",
    "cannot import %s memory as %s",
    "%s: memory import has %u pages which is smaller than the declared ",
    "%s: memory import has no maximum limit, expected at most %u",
    "%s: memory import has a larger maximum size %u than the ",
    "%s: mismatch in shared state of memory, declared = %d, imported = ",
];

/// 导入种类处理器 (5 类)
/// v8: src/wasm/module-instantiate.cc InstanceBuilder
#[rustfmt::skip]
pub static IMPORT_PROCESSORS: &[&str] = &[
    "ProcessImportedFunction — 可调用 + 类型匹配",
    "ProcessImportedTable — WebAssembly.Table + 类型匹配",
    "ProcessImportedWasmGlobalObject — WebAssembly.Global (含 v128 强制)",
    "ProcessImportedGlobal — 数值/合法引用",
    "ProcessImportedMemories — 页数下限/最大上限/共享态三查",
];

/// 内存导入三查错误语义
/// v8: src/wasm/module-instantiate.cc ProcessImportedMemories
#[rustfmt::skip]
pub static MEMORY_IMPORT_CHECKS: &[&str] = &[
    "导入页数 < 声明最小页数",
    "声明有最大但导入无最大",
    "导入最大 > 声明最大",
    "共享态不一致 (declared vs imported)",
];

/// 内存条目数上限防线: kV8MaxWasmMemories <= kMaxUInt32
/// v8: src/wasm/module-instantiate.cc static_assert
pub const MEMORIES_LIMIT_GUARD: &str = "kV8MaxWasmMemories <= kMaxUInt32";

// —— 提取不变式断言 ——
const _: () = {
    assert_eq!(INSTANTIATE_PHASES.len(), 4);
    assert_eq!(JS_WRAPPER_PARAM_OFFSET, 1);
    assert_eq!(IMPORT_LINK_ERRORS.len(), 23);
    assert_eq!(IMPORT_PROCESSORS.len(), 5);
    assert_eq!(MEMORY_IMPORT_CHECKS.len(), 4);
};

// —— 方法签名骨架 (就地修订 todo) ——
/// v8: src/wasm/module-instantiate.cc InstanceBuilder::Build
pub fn instance_builder_build() {
    todo!("InstanceBuilder::Build: 前置 Isolate/Handles/Factory 与 WasmInstanceObject 终装面; 解锁 v8-execution-isolate+v8-handles+v8-wasm-objects")
}

/// v8: src/wasm/module-instantiate.cc InstanceBuilder::Build_Phase1
pub fn instance_builder_build_phase1() {
    todo!("InstanceBuilder::Build_Phase1: 前置 Isolate/Handles/Factory 与 WasmInstanceObject 终装面; 解锁 v8-execution-isolate+v8-handles+v8-wasm-objects")
}

/// v8: src/wasm/module-instantiate.cc InstanceBuilder::Build_Phase1_Infallible
pub fn instance_builder_build_phase1_infallible() {
    todo!("InstanceBuilder::Build_Phase1_Infallible: 前置 Isolate/Handles/Factory 与 WasmInstanceObject 终装面; 解锁 v8-execution-isolate+v8-handles+v8-wasm-objects")
}

/// v8: src/wasm/module-instantiate.cc InstanceBuilder::Build_Phase2
pub fn instance_builder_build_phase2() {
    todo!("InstanceBuilder::Build_Phase2: 前置 Isolate/Handles/Factory 与 WasmInstanceObject 终装面; 解锁 v8-execution-isolate+v8-handles+v8-wasm-objects")
}

/// v8: src/wasm/module-instantiate.cc InstanceBuilder::ExecuteStartFunction
pub fn instance_builder_executestartfunction() {
    todo!("InstanceBuilder::ExecuteStartFunction: 前置 Isolate/Handles/Factory 与 WasmInstanceObject 终装面; 解锁 v8-execution-isolate+v8-handles+v8-wasm-objects")
}

/// v8: src/wasm/module-instantiate.cc InstanceBuilder::LookupImport
pub fn instance_builder_lookupimport() {
    todo!("InstanceBuilder::LookupImport: 前置 WebAssembly JS 导入对象 (Table/Memory/Global/Tag) 校验面; 解锁 v8-wasm-js+v8-objects-js-core")
}

/// v8: src/wasm/module-instantiate.cc InstanceBuilder::LoadDataSegments
pub fn instance_builder_loaddatasegments() {
    todo!("InstanceBuilder::LoadDataSegments: 前置堆内存/全局裸指针写入面; 解锁 v8-heap-core+v8-wasm-objects")
}

/// v8: src/wasm/module-instantiate.cc InstanceBuilder::WriteGlobalValue
pub fn instance_builder_writeglobalvalue() {
    todo!("InstanceBuilder::WriteGlobalValue: 前置堆内存/全局裸指针写入面; 解锁 v8-heap-core+v8-wasm-objects")
}

/// v8: src/wasm/module-instantiate.cc InstanceBuilder::SanitizeImports
pub fn instance_builder_sanitizeimports() {
    todo!("InstanceBuilder::SanitizeImports: 前置 WebAssembly JS 导入对象 (Table/Memory/Global/Tag) 校验面; 解锁 v8-wasm-js+v8-objects-js-core")
}

/// v8: src/wasm/module-instantiate.cc InstanceBuilder::ProcessImportedFunction
pub fn instance_builder_processimportedfunction() {
    todo!("InstanceBuilder::ProcessImportedFunction: 前置 WebAssembly JS 导入对象 (Table/Memory/Global/Tag) 校验面; 解锁 v8-wasm-js+v8-objects-js-core")
}

/// v8: src/wasm/module-instantiate.cc InstanceBuilder::ProcessImportedTable
pub fn instance_builder_processimportedtable() {
    todo!("InstanceBuilder::ProcessImportedTable: 前置 WebAssembly JS 导入对象 (Table/Memory/Global/Tag) 校验面; 解锁 v8-wasm-js+v8-objects-js-core")
}

/// v8: src/wasm/module-instantiate.cc InstanceBuilder::ProcessImportedWasmGlobalObject
pub fn instance_builder_processimportedwasmglobalobject() {
    todo!("InstanceBuilder::ProcessImportedWasmGlobalObject: 前置 WebAssembly JS 导入对象 (Table/Memory/Global/Tag) 校验面; 解锁 v8-wasm-js+v8-objects-js-core")
}

/// v8: src/wasm/module-instantiate.cc InstanceBuilder::ProcessImportedGlobal
pub fn instance_builder_processimportedglobal() {
    todo!("InstanceBuilder::ProcessImportedGlobal: 前置 WebAssembly JS 导入对象 (Table/Memory/Global/Tag) 校验面; 解锁 v8-wasm-js+v8-objects-js-core")
}

/// v8: src/wasm/module-instantiate.cc InstanceBuilder::ProcessImports
pub fn instance_builder_processimports() {
    todo!("InstanceBuilder::ProcessImports: 前置 WebAssembly JS 导入对象 (Table/Memory/Global/Tag) 校验面; 解锁 v8-wasm-js+v8-objects-js-core")
}

/// v8: src/wasm/module-instantiate.cc InstanceBuilder::ProcessImportedMemories
pub fn instance_builder_processimportedmemories() {
    todo!("InstanceBuilder::ProcessImportedMemories: 前置 WebAssembly JS 导入对象 (Table/Memory/Global/Tag) 校验面; 解锁 v8-wasm-js+v8-objects-js-core")
}

/// v8: src/wasm/module-instantiate.cc InstanceBuilder::GetRawUntaggedGlobalPtr
pub fn instance_builder_getrawuntaggedglobalptr() {
    todo!("InstanceBuilder::GetRawUntaggedGlobalPtr: 前置堆内存/全局裸指针写入面; 解锁 v8-heap-core+v8-wasm-objects")
}

/// v8: src/wasm/module-instantiate.cc InstanceBuilder::InitGlobals
pub fn instance_builder_initglobals() {
    todo!("InstanceBuilder::InitGlobals: 前置堆内存/全局裸指针写入面; 解锁 v8-heap-core+v8-wasm-objects")
}

/// v8: src/wasm/module-instantiate.cc InstanceBuilder::AllocateMemory
pub fn instance_builder_allocatememory() {
    todo!("InstanceBuilder::AllocateMemory: 前置堆内存/全局裸指针写入面; 解锁 v8-heap-core+v8-wasm-objects")
}

/// v8: src/wasm/module-instantiate.cc InstanceBuilder::ProcessExports
pub fn instance_builder_processexports() {
    todo!("InstanceBuilder::ProcessExports: 前置实例表段/标签初始化面; 解锁 v8-wasm-objects+v8-objects-arrays")
}

/// v8: src/wasm/module-instantiate.cc InstanceBuilder::SetTableInitialValues
pub fn instance_builder_settableinitialvalues() {
    todo!("InstanceBuilder::SetTableInitialValues: 前置实例表段/标签初始化面; 解锁 v8-wasm-objects+v8-objects-arrays")
}

/// v8: src/wasm/module-instantiate.cc InstanceBuilder::LoadTableSegments
pub fn instance_builder_loadtablesegments() {
    todo!("InstanceBuilder::LoadTableSegments: 前置实例表段/标签初始化面; 解锁 v8-wasm-objects+v8-objects-arrays")
}

/// v8: src/wasm/module-instantiate.cc InstanceBuilder::InitializeTags
pub fn instance_builder_initializetags() {
    todo!("InstanceBuilder::InitializeTags: 前置实例表段/标签初始化面; 解锁 v8-wasm-objects+v8-objects-arrays")
}

