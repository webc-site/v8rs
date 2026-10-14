#!/usr/bin/env bun
// tool/src/wasm/wasm-code-pointer-table.h.js
// 机械提取 v8/src/wasm/wasm-code-pointer-table.{h,cc} 的表常量与布局标记

import { readV8, writeSrc, banner } from "../../lib.js";

const hSrc = readV8("src/wasm/wasm-code-pointer-table.h");
const ccSrc = readV8("src/wasm/wasm-code-pointer-table.cc");

// 1. Entry 写保护标记
const writeProtectedMatch = hSrc.match(
  /static constexpr bool IsWriteProtected = (true|false);/,
);
if (!writeProtectedMatch) throw new Error("未找到 IsWriteProtected");
const isWriteProtected = writeProtectedMatch[1] === "true";

// 2. 无效句柄 (u32 域内的 -1)
const invalidHandleMatch = hSrc.match(/static constexpr Handle kInvalidHandle = -1;/);
if (!invalidHandleMatch) throw new Error("未找到 kInvalidHandle");

// 3. 重试标记 freelist (length=0xffffffff, next=0)
const retryMatch = hSrc.match(
  /static constexpr FreelistHead kRetryMarker = FreelistHead\(0x([0-9a-fA-F]+), (\d+)\);/,
);
if (!retryMatch) throw new Error("未找到 kRetryMarker");
const retryLength = parseInt(retryMatch[1], 16);
const retryNext = parseInt(retryMatch[2], 10);
if (retryLength !== 0xffffffff || retryNext !== 0) {
  throw new Error(`kRetryMarker 不符: len=${retryLength} next=${retryNext}`);
}

// 4. sandbox 布局: entrypoint_(Address=8) 后接 signature_hash_(u64)
const hasSandboxField = hSrc.includes("#ifdef V8_ENABLE_SANDBOX\n  uint64_t signature_hash_;");
if (!hasSandboxField) throw new Error("未找到 signature_hash_ 字段");
const entrySizeNoSandbox = 8; // sizeof(Address)
const entrySizeSandbox = 16; // Address + uint64_t
const offsetOfSignatureHashSandbox = 8; // offsetof(entry, signature_hash_)

// 5. cc 侧: 表初始化容量检查点 (只登记无具体魔数的结构)
const initializeRef = ccSrc.match(/void WasmCodePointerTable::Initialize\(\)/);
if (!initializeRef) throw new Error("未找到 WasmCodePointerTable::Initialize");

console.log(
  `wasm-code-pointer-table: writeProtected=${isWriteProtected} entry(sandbox)=${entrySizeSandbox}`,
);

// 6. 生成 Rust
let rust = banner(
  "src/wasm/wasm-code-pointer-table.h.js",
  "src/wasm/wasm-code-pointer-table.{h,cc}",
  "WasmCodePointerTable 布局与哨兵常量",
);

rust += `
/// 表项是否写保护 (原版 WasmCodePointerTableEntry::IsWriteProtected)
pub const IS_WRITE_PROTECTED: bool = ${isWriteProtected};

/// 无效句柄 (原版 kInvalidHandle = -1, 即 u32 最大值)
pub const INVALID_HANDLE: u32 = u32::MAX;

/// freelist 重试标记 length (原版 kRetryMarker length)
pub const RETRY_MARKER_LENGTH: u32 = 0xFFFF_FFFF;

/// freelist 重试标记 next (原版 kRetryMarker next)
pub const RETRY_MARKER_NEXT: u32 = 0;

/// 表项布局参数 (sandbox 关: 仅 entrypoint_; sandbox 开: + signature_hash_)
pub mod entry_layout {
  /// entrypoint_ 字段宽 (std::atomic<Address>)
  pub const ENTRYPOINT_SIZE: usize = 8;

  /// sandbox 开启时表项总宽 (Address + u64)
  pub const ENTRY_SIZE_SANDBOX: usize = ${entrySizeSandbox};

  /// sandbox 关闭时表项总宽
  pub const ENTRY_SIZE_NO_SANDBOX: usize = ${entrySizeNoSandbox};

  /// sandbox 开启时 signature_hash_ 偏移 (原版 kOffsetOfSignatureHash = offsetof)
  pub const OFFSET_OF_SIGNATURE_HASH: usize = ${offsetOfSignatureHashSandbox};
}

// —— 提取不变式断言 ——
const _: () = {
  assert!(INVALID_HANDLE == 0xFFFF_FFFF);
  assert!(entry_layout::OFFSET_OF_SIGNATURE_HASH == entry_layout::ENTRYPOINT_SIZE);
  assert!(entry_layout::ENTRY_SIZE_SANDBOX == 16);
};
`;

const p = writeSrc("v8-wasm-core", "wasm_code_pointer_table.rs", rust);
console.log(`written ${p}`);
