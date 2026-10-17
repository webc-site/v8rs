// 由 tool/src/wasm/c-api.cc.js 从 v8/src/wasm/c-api.{h,cc} 生成; 修订就地改, 复跑覆盖
// C API 外部种类映射与缓冲上限 + WASM_EXPORT 方法签名骨架 (枚举在 c_api.rs.skel)

//! wasm-c-api 兼容层规则面。

/// 外部种类映射: exact-function 归并 function, 共 6 种外部形态
/// v8: src/wasm/c-api.cc GetImportExportType
#[rustfmt::skip]
pub static EXTERNAL_KIND_MAP: &[(&str, &str)] = &[
    ("kExternalFunction", "FuncType"),
    ("kExternalExactFunction", "FuncType (归并 function)"),
    ("kExternalTable", "TableType"),
    ("kExternalMemory", "MemoryType"),
    ("kExternalGlobal", "GlobalType"),
    ("kExternalTag", "TagType"),
];

/// 名字复制缓冲上限 (含 NUL)
/// v8: src/wasm/c-api.cc kMaxNameSize
pub const K_MAX_NAME_SIZE: i32 = 64;
/// 计数器表上限 (满后拒绝新计数器)
/// v8: src/wasm/c-api.cc kMaxCounters
pub const K_MAX_COUNTERS: u32 = 512;
/// 反序列化名字盒尺寸
/// v8: src/wasm/c-api.cc kNameBoxSize
pub const K_NAME_BOX_SIZE: i32 = 64;
/// 反序列化值盒尺寸
/// v8: src/wasm/c-api.cc kValueBoxSize
pub const K_VALUE_BOX_SIZE: i32 = 13;

/// Store 构造时开启未捕获异常栈捕获, 深度 10, Overview 模式 (为 trap 生成栈)
/// v8: src/wasm/c-api.cc Store::make
pub const K_STACK_LIMIT: i32 = 10;
pub const STACK_CAPTURE_MODE: &str = "kOverview";

/// 托管类型 id (HostData / FuncData)
/// v8: src/wasm/c-api.cc kTypeID
#[rustfmt::skip]
pub static MANAGED_TYPE_IDS: &[&str] = &[
    "i::ManagedTypeId::kWasmManagedData",
    "i::ManagedTypeId::kWasmFuncData",
];

/// 表最大长对外截断到 kMaxUInt32
/// v8: src/wasm/c-api.cc wasm_table_type_maximum 或等价处
pub const TABLE_MAX_TRUNCATE: &str = "maximum_length_u64().value_or(kMaxUInt32) 钳 kMaxUInt32";

/// vec<T> 与 wasm_xxx_vec_t 布局兼容断言 (V8_GC_MOLE 构建下豁免)
/// v8: src/wasm/c-api.cc ASSERT_VEC_BASE_SIZE
pub const VEC_LAYOUT_ASSERT: &str = "sizeof(wasm_##name##_vec_t) == sizeof(vec<Name>) (GC_MOLE 除外)";

// —— 提取不变式断言 ——
const _: () = {
    assert_eq!(EXTERNAL_KIND_MAP.len(), 6);
    assert_eq!(K_MAX_NAME_SIZE, 64);
    assert_eq!(K_MAX_COUNTERS, 512);
    assert_eq!(K_NAME_BOX_SIZE, 64);
    assert_eq!(K_VALUE_BOX_SIZE, 13);
    assert_eq!(K_STACK_LIMIT, 10);
    assert_eq!(MANAGED_TYPE_IDS.len(), 2);
};

/// Config WASM_EXPORT 方法骨架 (1 个)
/// v8: src/wasm/c-api.cc Config::make
pub fn config_api_methods() {
    todo!("Config::wasm_config_new (1 个 WASM_EXPORT): 前置 wasm-c-api 句柄类型面 (c_api.rs.skel 挂载) 与 Isolate 创建链; 解锁 v8-execution-isolate+rs8-capi")
}

/// Engine WASM_EXPORT 方法骨架 (1 个)
/// v8: src/wasm/c-api.cc Engine::make
pub fn engine_api_methods() {
    todo!("Engine::wasm_engine_new (1 个 WASM_EXPORT): 前置 WasmEngine 编译调度面; 解锁 v8-execution-isolate+v8-wasm-js")
}

/// Store WASM_EXPORT 方法骨架 (1 个)
/// v8: src/wasm/c-api.cc Store::make
pub fn store_api_methods() {
    todo!("Store::wasm_store_new (1 个 WASM_EXPORT): 前置 Isolate 生命周期与栈捕获面; 解锁 v8-execution-isolate+v8-runtime")
}

/// ValType WASM_EXPORT 方法骨架 (2 个)
/// v8: src/wasm/c-api.cc ValType::copy
/// v8: src/wasm/c-api.cc ValType::kind
pub fn valtype_api_methods() {
    todo!("ValType::wasm_valtype_new/copy/kind (2 个 WASM_EXPORT): 前置 wasm-c-api 值类型句柄面; 解锁 rs8-capi")
}

/// ExternType WASM_EXPORT 方法骨架 (8 个)
/// v8: src/wasm/c-api.cc ExternType::copy
/// v8: src/wasm/c-api.cc ExternType::kind
/// v8: src/wasm/c-api.cc ExternType::func
/// v8: src/wasm/c-api.cc ExternType::func
/// v8: src/wasm/c-api.cc ExternType::table
/// v8: src/wasm/c-api.cc ExternType::table
/// v8: src/wasm/c-api.cc ExternType::memory
/// v8: src/wasm/c-api.cc ExternType::memory
pub fn externtype_api_methods() {
    todo!("ExternType::wasm_externtype_* 8 导出 (8 个 WASM_EXPORT): 前置外部类型句柄面 (依赖各 Type 骨架); 解锁 rs8-capi")
}

/// FuncType WASM_EXPORT 方法骨架 (4 个)
/// v8: src/wasm/c-api.cc FuncType::make
/// v8: src/wasm/c-api.cc FuncType::copy
/// v8: src/wasm/c-api.cc FuncType::params
/// v8: src/wasm/c-api.cc FuncType::results
pub fn functype_api_methods() {
    todo!("FuncType::wasm_functype_* 4 导出 (4 个 WASM_EXPORT): 前置 FunctionSig 面; 解锁 rs8-capi+v8-codegen-core")
}

/// TableType WASM_EXPORT 方法骨架 (4 个)
/// v8: src/wasm/c-api.cc TableType::make
/// v8: src/wasm/c-api.cc TableType::copy
/// v8: src/wasm/c-api.cc TableType::element
/// v8: src/wasm/c-api.cc TableType::limits
pub fn tabletype_api_methods() {
    todo!("TableType::wasm_tabletype_* 4 导出 (4 个 WASM_EXPORT): 前置 WasmTableDefinition 面; 解锁 rs8-capi+v8-wasm-objects")
}

/// MemoryType WASM_EXPORT 方法骨架 (3 个)
/// v8: src/wasm/c-api.cc MemoryType::make
/// v8: src/wasm/c-api.cc MemoryType::copy
/// v8: src/wasm/c-api.cc MemoryType::limits
pub fn memorytype_api_methods() {
    todo!("MemoryType::wasm_memorytype_* 3 导出 (3 个 WASM_EXPORT): 前置 WasmMemoryDefinition 面; 解锁 rs8-capi+v8-wasm-objects")
}

/// ImportType WASM_EXPORT 方法骨架 (5 个)
/// v8: src/wasm/c-api.cc ImportType::make
/// v8: src/wasm/c-api.cc ImportType::copy
/// v8: src/wasm/c-api.cc ImportType::module
/// v8: src/wasm/c-api.cc ImportType::name
/// v8: src/wasm/c-api.cc ImportType::type
pub fn importtype_api_methods() {
    todo!("ImportType::wasm_importtype_* 5 导出 (5 个 WASM_EXPORT): 前置 WasmImport 名称复制面; 解锁 rs8-capi")
}

/// ExportType WASM_EXPORT 方法骨架 (4 个)
/// v8: src/wasm/c-api.cc ExportType::make
/// v8: src/wasm/c-api.cc ExportType::copy
/// v8: src/wasm/c-api.cc ExportType::name
/// v8: src/wasm/c-api.cc ExportType::type
pub fn exporttype_api_methods() {
    todo!("ExportType::wasm_exporttype_* 4 导出 (4 个 WASM_EXPORT): 前置 WasmExport 名称复制面; 解锁 rs8-capi")
}

/// Ref WASM_EXPORT 方法骨架 (3 个)
/// v8: src/wasm/c-api.cc Ref::copy
/// v8: src/wasm/c-api.cc Ref::same
/// v8: src/wasm/c-api.cc Ref::get_host_info
pub fn ref_api_methods() {
    todo!("Ref::wasm_ref_* 3 导出 (3 个 WASM_EXPORT): 前置 JSReference 句柄表与 host_info 面; 解锁 rs8-capi+v8-handles")
}

/// Trap WASM_EXPORT 方法骨架 (3 个)
/// v8: src/wasm/c-api.cc Trap::copy
/// v8: src/wasm/c-api.cc Trap::make
/// v8: src/wasm/c-api.cc Trap::message
pub fn trap_api_methods() {
    todo!("Trap::wasm_trap_* 3 导出 (3 个 WASM_EXPORT): 前置 MessageHandler/trap 栈面; 解锁 v8-execution-messages+rs8-capi")
}

/// Foreign WASM_EXPORT 方法骨架 (2 个)
/// v8: src/wasm/c-api.cc Foreign::copy
/// v8: src/wasm/c-api.cc Foreign::make
pub fn foreign_api_methods() {
    todo!("Foreign::wasm_foreign_* 2 导出 (2 个 WASM_EXPORT): 前置 ManagedTypeId 托管数据面; 解锁 v8-heap-core+rs8-capi")
}

/// Module WASM_EXPORT 方法骨架 (9 个)
/// v8: src/wasm/c-api.cc Module::copy
/// v8: src/wasm/c-api.cc Module::validate
/// v8: src/wasm/c-api.cc Module::make
/// v8: src/wasm/c-api.cc Module::imports
/// v8: src/wasm/c-api.cc Module::exports
/// v8: src/wasm/c-api.cc Module::serialize
/// v8: src/wasm/c-api.cc Module::deserialize
/// v8: src/wasm/c-api.cc Module::share
/// v8: src/wasm/c-api.cc Module::obtain
pub fn module_api_methods() {
    todo!("Module::wasm_module_* 9 导出 (9 个 WASM_EXPORT): 前置编译/序列化全链 (WasmEngine+编译调度); 解锁 v8-wasm-js+v8-execution-isolate")
}

/// Extern WASM_EXPORT 方法骨架 (11 个)
/// v8: src/wasm/c-api.cc Extern::copy
/// v8: src/wasm/c-api.cc Extern::kind
/// v8: src/wasm/c-api.cc Extern::type
/// v8: src/wasm/c-api.cc Extern::func
/// v8: src/wasm/c-api.cc Extern::global
/// v8: src/wasm/c-api.cc Extern::table
/// v8: src/wasm/c-api.cc Extern::memory
/// v8: src/wasm/c-api.cc Extern::func
/// v8: src/wasm/c-api.cc Extern::global
/// v8: src/wasm/c-api.cc Extern::table
/// v8: src/wasm/c-api.cc Extern::memory
pub fn extern_api_methods() {
    todo!("Extern::wasm_extern_* 11 导出 (11 个 WASM_EXPORT): 前置 Extern 联合句柄面 (依赖 Func/Global/Table/Memory); 解锁 rs8-capi")
}

/// Func WASM_EXPORT 方法骨架 (7 个)
/// v8: src/wasm/c-api.cc Func::copy
/// v8: src/wasm/c-api.cc Func::make
/// v8: src/wasm/c-api.cc Func::make
/// v8: src/wasm/c-api.cc Func::type
/// v8: src/wasm/c-api.cc Func::param_arity
/// v8: src/wasm/c-api.cc Func::result_arity
/// v8: src/wasm/c-api.cc Func::call
pub fn func_api_methods() {
    todo!("Func::wasm_func_* 7 导出 (7 个 WASM_EXPORT): 前置导出包装调用与 FuncData 托管面; 解锁 v8-wasm-js+v8-execution-frames")
}

/// Global WASM_EXPORT 方法骨架 (4 个)
/// v8: src/wasm/c-api.cc Global::copy
/// v8: src/wasm/c-api.cc Global::make
/// v8: src/wasm/c-api.cc Global::type
/// v8: src/wasm/c-api.cc Global::get
pub fn global_api_methods() {
    todo!("Global::wasm_global_* 4 导出 (4 个 WASM_EXPORT): 前置全局读写裸指针面; 解锁 v8-wasm-objects+rs8-capi")
}

/// Table WASM_EXPORT 方法骨架 (7 个)
/// v8: src/wasm/c-api.cc Table::copy
/// v8: src/wasm/c-api.cc Table::make
/// v8: src/wasm/c-api.cc Table::type
/// v8: src/wasm/c-api.cc Table::get
/// v8: src/wasm/c-api.cc Table::set
/// v8: src/wasm/c-api.cc Table::size
/// v8: src/wasm/c-api.cc Table::grow
pub fn table_api_methods() {
    todo!("Table::wasm_table_* 7 导出 (7 个 WASM_EXPORT): 前置表 get/set/size/grow 面; 解锁 v8-wasm-objects+rs8-capi")
}

/// Memory WASM_EXPORT 方法骨架 (7 个)
/// v8: src/wasm/c-api.cc Memory::copy
/// v8: src/wasm/c-api.cc Memory::make
/// v8: src/wasm/c-api.cc Memory::type
/// v8: src/wasm/c-api.cc Memory::data
/// v8: src/wasm/c-api.cc Memory::data_size
/// v8: src/wasm/c-api.cc Memory::size
/// v8: src/wasm/c-api.cc Memory::grow
pub fn memory_api_methods() {
    todo!("Memory::wasm_memory_* 7 导出 (7 个 WASM_EXPORT): 前置内存 data/grow 面 (共享内存互斥); 解锁 v8-wasm-objects+v8-sandbox")
}

/// Instance WASM_EXPORT 方法骨架 (2 个)
/// v8: src/wasm/c-api.cc Instance::copy
/// v8: src/wasm/c-api.cc Instance::exports
pub fn instance_api_methods() {
    todo!("Instance::wasm_instance_* 2 导出 (2 个 WASM_EXPORT): 前置 InstanceBuilder::Build 全链; 解锁 v8-wasm-js+v8-execution-isolate")
}
