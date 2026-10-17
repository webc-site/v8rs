#!/usr/bin/env bun
// tool/src/snapshot/code-serializer.js
// 机械提取 v8/src/snapshot/code-serializer.{h,cc} 的代码缓存头布局与健全性检查结果。
// cc 1034 行超大只提取常量面。

import { readV8, writeSrc, banner } from "../../lib.js";

const hSrc = readV8("src/snapshot/code-serializer.h");
const ccSrc = readV8("src/snapshot/code-serializer.cc");

const K_UINT32_SIZE = 4;
const MAGIC_NUMBER_OFFSET = 0; // 对齐 snapshot_data.rs

// 1. SourceHash 尺寸 = kSizeOfSha256Digest (32 字节)
const shaM = hSrc.match(/static constexpr uint32_t kSize = kSizeOfSha256Digest;/);
if (!shaM) throw new Error("未找到 SourceHash::kSize");
const SHA256_DIGEST = 32;

// 2. 头布局偏移链 (逐条机械展开)
//    kVersionHashOffset = kMagicNumberOffset + kUInt32Size
//    kSourceHashOffset  = kVersionHashOffset + kUInt32Size
//    kFlagHashOffset    = kSourceHashOffset + kSizeOfSha256Digest
//    kReadOnlySnapshotChecksumOffset = kFlagHashOffset + kUInt32Size
//    kPayloadLengthOffset = kReadOnlySnapshotChecksumOffset + kUInt32Size
//    kTrustedPayloadLengthOffset = kPayloadLengthOffset + kUInt32Size
//    kChecksumOffset = kTrustedPayloadLengthOffset + kUInt32Size
//    kUnalignedHeaderSize = kChecksumOffset + kUInt32Size
//    kHeaderSize = POINTER_SIZE_ALIGN(kUnalignedHeaderSize)
const layout = {
  versionHash: MAGIC_NUMBER_OFFSET + K_UINT32_SIZE,
  sourceHash: MAGIC_NUMBER_OFFSET + K_UINT32_SIZE * 2,
  flagHash: MAGIC_NUMBER_OFFSET + K_UINT32_SIZE * 2 + SHA256_DIGEST,
  roChecksum: MAGIC_NUMBER_OFFSET + K_UINT32_SIZE * 3 + SHA256_DIGEST,
  payloadLength: MAGIC_NUMBER_OFFSET + K_UINT32_SIZE * 4 + SHA256_DIGEST,
  trustedPayloadLength: MAGIC_NUMBER_OFFSET + K_UINT32_SIZE * 5 + SHA256_DIGEST,
  checksum: MAGIC_NUMBER_OFFSET + K_UINT32_SIZE * 6 + SHA256_DIGEST,
  unalignedHeaderSize: MAGIC_NUMBER_OFFSET + K_UINT32_SIZE * 7 + SHA256_DIGEST,
};
layout.headerSize = (layout.unalignedHeaderSize + 7) & ~7; // POINTER_SIZE_ALIGN
const layoutNames = [
  "kVersionHashOffset",
  "kSourceHashOffset",
  "kFlagHashOffset",
  "kReadOnlySnapshotChecksumOffset",
  "kPayloadLengthOffset",
  "kTrustedPayloadLengthOffset",
  "kChecksumOffset",
  "kUnalignedHeaderSize",
  "kHeaderSize",
];
for (const n of layoutNames) {
  if (!hSrc.includes(`static constexpr uint32_t ${n}`)) throw new Error(`未找到布局常量 ${n}`);
}

// 3. static_assert: 健全性检查结果数 == 9 (h)
const assertM = hSrc.match(/static_assert\(static_cast<int>\(SerializedCodeSanityCheckResult::kLast\) == (\d+)\);/);
if (!assertM) throw new Error("未找到 kLast == 9 断言");
if (assertM[1] !== "9") throw new Error(`kLast 断言异常: ${assertM[1]}`);

// 4. cc 侧 CachedDataVersionTag / ClearRehashableAndReconstructableData 等 (机械清点)
const ccFns = [...ccSrc.matchAll(/^(?:void|bool|AlignedCachedData\*|base::Vector<const uint8_t>|std::unique_ptr<[\w:]+>)\s+(?:CodeSerializer::)?(\w+)\(/gm)].map((m) => m[1]);
if (ccFns.length < 3) throw new Error(`cc 函数清点过少: ${ccFns.length}`);
const ccLines = ccSrc.split("\n").length;
console.log(`头布局 ${layoutNames.length} 槽 + SHA256 ${SHA256_DIGEST}B + 断言 kLast==9 + cc ${ccLines} 行`);

let rust = banner("src/snapshot/code-serializer.js", "src/snapshot/code-serializer.{h,cc}", "代码缓存序列化头布局");
rust += `
/// SerializedCodeData 头布局 (原版 code-serializer.h, uint32 槽; 例外: 256 位源哈希)。
/// 偏移链机械展开 (基点 kMagicNumberOffset = ${MAGIC_NUMBER_OFFSET}):
pub mod serialized_code_data_layout {
  /// kVersionHashOffset = kMagicNumberOffset + kUInt32Size
  pub const VERSION_HASH_OFFSET: u32 = ${layout.versionHash};
  /// kSourceHashOffset = kVersionHashOffset + kUInt32Size
  pub const SOURCE_HASH_OFFSET: u32 = ${layout.sourceHash};
  /// kFlagHashOffset = kSourceHashOffset + kSizeOfSha256Digest
  pub const FLAG_HASH_OFFSET: u32 = ${layout.flagHash};
  /// kReadOnlySnapshotChecksumOffset
  pub const READ_ONLY_SNAPSHOT_CHECKSUM_OFFSET: u32 = ${layout.roChecksum};
  /// kPayloadLengthOffset
  pub const PAYLOAD_LENGTH_OFFSET: u32 = ${layout.payloadLength};
  /// kTrustedPayloadLengthOffset (双载荷: untrusted + trusted)
  pub const TRUSTED_PAYLOAD_LENGTH_OFFSET: u32 = ${layout.trustedPayloadLength};
  /// kChecksumOffset
  pub const CHECKSUM_OFFSET: u32 = ${layout.checksum};
  /// kUnalignedHeaderSize
  pub const UNALIGNED_HEADER_SIZE: u32 = ${layout.unalignedHeaderSize};
  /// kHeaderSize = POINTER_SIZE_ALIGN(kUnalignedHeaderSize)
  pub const HEADER_SIZE: u32 = ${layout.headerSize};

  /// SourceHash 尺寸 = kSizeOfSha256Digest
  pub const SOURCE_HASH_SIZE: usize = ${SHA256_DIGEST};
}

/// 健全性检查结果 (原版 SerializedCodeSanityCheckResult, typedef 自
/// v8::ScriptCompiler::CachedData::CompatibilityCheckResult, 9 档):
/// 原版 static_assert(kLast == 9) — 档位定义在公开 API 头 (v8.h), 由 Developer
/// 对齐 v8 crate 落枚举; 本模块登记断言基准。
pub const SANITY_CHECK_RESULT_COUNT: usize = 9;

/// cc 侧 (机械登记, ${ccLines} 行, 语义由 Developer 落):
/// ${ccFns.join(", ")}
/// 主入口: CodeSerializer::Serialize → 头填充 (魔数/版本哈希/旗标哈希/RO 校验和/
/// 双载荷长度/校验和) → UntrustedPayload + TrustedPayload 双段。
`;

const p = writeSrc("v8-snapshot", "code_serializer.rs", rust);
console.log(`written ${p}`);
