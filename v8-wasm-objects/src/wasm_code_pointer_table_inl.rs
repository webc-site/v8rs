// 由 tool/src/wasm/wasm-code-pointer-table-inl.h.js 从 v8/src/wasm/wasm-code-pointer-table-inl.h 生成; 修订就地改, 复跑覆盖
// 代码指针表条目内联语义与 freelist 状态机 (尺寸常量在 wasm_code_pointer_table.rs)

//! WasmCodePointerTable 条目读写与无锁分配。

/// 表写作用域理由串 (条目初始化/更新 共 2 处)
/// v8: src/wasm/wasm-code-pointer-table-inl.h SetEntrypointAndSignature / AllocateAndInitializeEntry
pub const WRITE_SCOPE_TABLE: &str = "WasmCodePointerTable write";
/// freelist 链接写作用域理由串
/// v8: src/wasm/wasm-code-pointer-table-inl.h LinkFreelist
pub const WRITE_SCOPE_FREELIST: &str = "write free list entry";

/// 更新条目时签名哈希必须与既有一致 (sandbox 防线)
/// v8: src/wasm/wasm-code-pointer-table-inl.h UpdateCodePointerEntry
pub const UPDATE_SIGNATURE_CHECK: &str = "SBXCHECK_EQ(old_signature_hash, signature_hash)";
/// freelist 项签名复位为无效哈希
/// v8: src/wasm/wasm-code-pointer-table-inl.h MakeFreelistEntry
pub const FREELIST_SIGNATURE_RESET: &str = "kInvalidWasmSignatureHash";

/// freelist 状态机步骤
/// v8: src/wasm/wasm-code-pointer-table-inl.h TryAllocateFromFreelist / ReadFreelistHead / LinkFreelist
#[rustfmt::skip]
pub static FREELIST_PROTOCOL: &[&str] = &[
    "读 freelist 头 (acquire); 遇 retry marker 自旋重试",
    "CAS 头 -> retry marker 以独占",
    "取 next 条目, 新头长度 = 旧长度 - 1",
    "非原子分配路径: 段内自取 (双检锁 + 段分配互斥)",
    "LinkFreelist: 末条目指旧头, CAS(release) 换新头, 长度相加",
    "FreeEntry: 单条目自链 (长度 1), 需写作用域",
];

// —— 提取不变式断言 ——
const _: () = {
    assert_eq!(WRITE_SCOPE_TABLE, "WasmCodePointerTable write");
    assert_eq!(WRITE_SCOPE_FREELIST, "write free list entry");
    assert_eq!(FREELIST_PROTOCOL.len(), 6);
};
