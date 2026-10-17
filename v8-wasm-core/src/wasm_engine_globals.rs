// 由 tool/src/wasm/wasm-engine-globals.h.js 从 v8/src/wasm/wasm-engine-globals.h 生成; 修订就地改, 复跑覆盖
// 进程级 Wasm 引擎全局访问函数骨架 (签名骨架)

/// 进程共享引擎单例访问函数 (对应原版 wasm-engine-globals.h 的 5 个 Get*)
/// 返回类型以原版类名登记; 语义实现待 Developer 修订
/// 原版 WasmEngine* GetWasmEngine()
pub fn get_wasm_engine() -> *mut WasmEngine {
  unimplemented!("登记骨架: 见 v8/src/wasm/wasm-engine.cc")
}

/// 原版 WasmCodeManager* GetWasmCodeManager()
pub fn get_wasm_code_manager() -> *mut WasmCodeManager {
  unimplemented!("登记骨架: 见 v8/src/wasm/wasm-engine.cc")
}

/// 原版 WasmImportWrapperCache* GetWasmImportWrapperCache()
pub fn get_wasm_import_wrapper_cache() -> *mut WasmImportWrapperCache {
  unimplemented!("登记骨架: 见 v8/src/wasm/wasm-engine.cc")
}

/// 原版 WasmStackEntryWrapperCache* GetWasmStackEntryWrapperCache()
pub fn get_wasm_stack_entry_wrapper_cache() -> *mut WasmStackEntryWrapperCache {
  unimplemented!("登记骨架: 见 v8/src/wasm/wasm-engine.cc")
}

/// 原版 CanonicalTypeNamesProvider* GetCanonicalTypeNamesProvider()
pub fn get_canonical_type_names_provider() -> *mut CanonicalTypeNamesProvider {
  unimplemented!("登记骨架: 见 v8/src/wasm/wasm-engine.cc")
}

// —— 提取不变式断言 ——
const _: () = {
  // 5 个访问函数 1:1 登记
};
