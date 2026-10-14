#!/usr/bin/env bun
// tool/src/wasm/wasm-code-pointer-table-inl.h.js
// 机械提取 v8/src/wasm/wasm-code-pointer-table-inl.h 的写作用域理由串、
// freelist 状态机 (retry marker CAS 自旋) 与签名哈希防线。
// 表尺寸常量在 v8-wasm-core/src/wasm_code_pointer_table.rs (wasm-code-pointer-table.h.js)。

import { readV8, writeSrc, banner } from "../../lib.js";

const src = readV8("src/wasm/wasm-code-pointer-table-inl.h");

// ---------- 1. 写作用域理由串 ----------
const writeReasons = [...src.matchAll(/WriteScope write_scope\("([^"]+)"\)/g)].map((m) => m[1]);
if (writeReasons.join("|") !== "WasmCodePointerTable write|WasmCodePointerTable write|write free list entry") {
  throw new Error(`写作用域理由串异常: ${writeReasons.join("|")}`);
}
const tableWriteCount = writeReasons.filter((r) => r === "WasmCodePointerTable write").length;
if (tableWriteCount !== 2) throw new Error(`表写理由串 ${tableWriteCount} != 2`);

// ---------- 2. 签名哈希防线 ----------
if (!src.includes("SBXCHECK_EQ(signature_hash_, signature_hash);")) {
  throw new Error("更新时签名一致性 SBXCHECK 缺失");
}
if (!src.includes("signature_hash_ = kInvalidWasmSignatureHash;")) {
  throw new Error("freelist 项签名复位缺失");
}

// ---------- 3. freelist 状态机 ----------
// 3a. 读侧自旋等待 retry marker 消失
if (!src.includes("if (IsRetryMarker(freelist)) {")) throw new Error("retry marker 自旋缺失");
if (!src.includes("// The retry marker will only be stored for a short amount of time. We can")) {
  throw new Error("retry marker 短暂语义注释缺失");
}
// 3b. CAS 占据 (kRetryMarker)
if (!src.includes("kRetryMarker")) throw new Error("kRetryMarker CAS 缺失");
// 3c. 长度随分配递减
if (!src.includes("FreelistHead new_freelist_head(next_freelist_entry,")) throw new Error("分配后 freelist 长度递减缺失");
// 3d. 链接时长度相加, release CAS
if (!src.includes("FreelistHead(freelist_to_link.next(),\n                            freelist_to_link.length() + current_head.length());")) {
  throw new Error("链接时长 度相加缺失");
}
if (!src.includes("std::memory_order_release")) throw new Error("链接 release 序缺失");

// ---------- 4. 内存序清单 ----------
const relaxed = [...src.matchAll(/std::memory_order_relaxed/g)].length;
const acquire = [...src.matchAll(/std::memory_order_acquire/g)].length;
if (relaxed !== 7) throw new Error(`relaxed 序 ${relaxed} != 7`);
if (acquire !== 1) throw new Error(`acquire 序 ${acquire} != 1`);

console.log(`wasm-code-pointer-table-inl: 理由串=${writeReasons.length} relaxed=${relaxed} acquire=${acquire} release=1`);

// ---------- 生成 Rust ----------
const rust = `${banner(
  "src/wasm/wasm-code-pointer-table-inl.h.js",
  "src/wasm/wasm-code-pointer-table-inl.h",
  "代码指针表条目内联语义与 freelist 状态机 (尺寸常量在 wasm_code_pointer_table.rs)",
)}
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
`;

const p = writeSrc("v8-wasm-objects", "wasm_code_pointer_table_inl.rs", rust);
console.log(`written ${p}`);
