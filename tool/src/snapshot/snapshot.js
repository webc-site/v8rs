#!/usr/bin/env bun
// tool/src/snapshot/snapshot.js
// 机械提取 v8/src/snapshot/snapshot.{h,cc} 的 SerializerFlag 位标志、
// Snapshot 门面签名与快照 blob 头布局 (cc 内匿名 namespace)

import { readV8, writeSrc, banner } from "../../lib.js";

const hSrc = readV8("src/snapshot/snapshot.h");
const ccSrc = readV8("src/snapshot/snapshot.cc");

// 1. SerializerFlag 位标志 (1 << n, 带注释)
const flagM = hSrc.match(/enum SerializerFlag \{([\s\S]*?)\n  \};/);
if (!flagM) throw new Error("未找到 SerializerFlag 枚举");
const flags = [...flagM[1].matchAll(/k(\w+)\s*=\s*1 << (\d+)/g)].map((m) => ({
  raw: `k${m[1]}`,
  rustName: m[1],
  bit: parseInt(m[2], 10),
}));
if (flags.length !== 4) {
  throw new Error(`SerializerFlag 条数不符: 期望 4, 实得 ${flags.length}: ${flags.map((f) => f.raw).join(",")}`);
}
flags.forEach((f, i) => {
  if (f.bit !== i) throw new Error(`SerializerFlag 位序断裂: ${f.raw} = ${f.bit}`);
});

// 2. kDefaultSerializerFlags = {}
const defM = hSrc.match(/static constexpr SerializerFlags kDefaultSerializerFlags =\s*\{\};/);
if (!defM) throw new Error("未找到 kDefaultSerializerFlags");

// 3. Snapshot 静态方法面 (h)
const snapM = hSrc.match(/class Snapshot : public AllStatic \{([\s\S]*?)\n\};/);
if (!snapM) throw new Error("未找到 Snapshot 类体");
const snapMethods = [...snapM[1].matchAll(/static\s+[\w:<>*&]+\s+(\w+)\(/g)].map((m) => m[1]);
if (snapMethods.length !== 15) {
  throw new Error(`Snapshot 方法数不符: 期望 15, 实得 ${snapMethods.length}: ${snapMethods.join(",")}`);
}

// 4. SnapshotCreatorImpl 常量
const ctxIdxM = hSrc.match(/static constexpr size_t kDefaultContextIndex = (\d+);/);
if (!ctxIdxM) throw new Error("未找到 kDefaultContextIndex");
const defCtx = parseInt(ctxIdxM[1], 10);
const firstAddtl = defCtx + 1; // 原版 kFirstAddtlContextIndex = kDefaultContextIndex + 1

// 5. cc 头布局常量 (匿名 namespace, ContextSerializerData 头)
const layoutNames = [
  "kNumberOfContextsOffset",
  "kRehashabilityOffset",
  "kChecksumOffset",
  "kReadOnlySnapshotChecksumOffset",
  "kVersionStringOffset",
  "kVersionStringLength",
  "kStartupOffsetOffset",
  "kSharedHeapOffsetOffset",
  "kFirstContextOffsetOffset",
];
const K_UINT32_SIZE = 4;
// 机械展开偏移链 (原版表达式逐条核对)
const layout = {
  kNumberOfContextsOffset: 0,
  kRehashabilityOffset: 0 + K_UINT32_SIZE,
  kChecksumOffset: 0 + K_UINT32_SIZE + K_UINT32_SIZE,
  kReadOnlySnapshotChecksumOffset: 0 + K_UINT32_SIZE * 3,
  kVersionStringOffset: 0 + K_UINT32_SIZE * 4,
  kVersionStringLength: 64,
  kStartupOffsetOffset: 0 + K_UINT32_SIZE * 4 + 64,
  kSharedHeapOffsetOffset: 0 + K_UINT32_SIZE * 4 + 64 + K_UINT32_SIZE,
  kFirstContextOffsetOffset: 0 + K_UINT32_SIZE * 4 + 64 + K_UINT32_SIZE * 2,
};
for (const n of layoutNames) {
  if (!ccSrc.includes(`static const uint32_t ${n}`)) throw new Error(`未找到头布局常量 ${n}`);
}
// 版本串长度对数
const verM = ccSrc.match(/static const uint32_t kVersionStringLength = (\d+);/);
if (!verM || parseInt(verM[1], 10) !== layout.kVersionStringLength) {
  throw new Error("kVersionStringLength 对数失败");
}
console.log(`SerializerFlag ${flags.length} + Snapshot 方法 ${snapMethods.length} + 头布局 ${layoutNames.length} 槽`);

let rust = banner("src/snapshot/snapshot.js", "src/snapshot/snapshot.{h,cc}", "快照门面与 blob 头布局");
rust += `
/// SerializerFlag 位标志 (原版 enum SerializerFlag, base::Flags 包装)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct SerializerFlags(u8);

impl SerializerFlags {
`;
for (const f of flags) {
  rust += `  /// ${f.raw}\n  pub const ${f.rustName.toUpperCase()}: u8 = 1 << ${f.bit};\n`;
}
rust += `
  /// 原版 kDefaultSerializerFlags = {} — 全默认空标志
  pub const DEFAULT: Self = Self(0);

  #[inline]
  #[must_use]
  pub const fn contains(self, flag: u8) -> bool {
    (self.0 & flag) != 0
  }

  #[inline]
  #[must_use]
  pub const fn bits(self) -> u8 {
    self.0
  }
}

/// Snapshot 门面 (原版 class Snapshot : AllStatic, 全静态)。方法面 (共 ${snapMethods.length} 项):
`;
for (const m of snapMethods) rust += `/// - \`${m}\`\n`;
rust += `
/// 语义分组:
/// - 序列化: ClearReconstructableDataForSerialization / Create
/// - 反序列化: Initialize / NewContextFromSnapshot
/// - 测试: SerializeDeserializeAndVerifyForTesting
/// - 辅助: HasContextSnapshot / EmbedsScript / GetExpectedChecksum / CalculateChecksum /
///   VerifyChecksum / ExtractRehashability / ExtractReadOnlySnapshotChecksum / VersionIsValid
/// - 快照源实现方: DefaultSnapshotBlob / ShouldVerifyChecksum (DEBUG 另有 SnapshotIsValid)

/// SnapshotCreatorImpl 上下文槽索引 (原版 constexpr)
pub const DEFAULT_CONTEXT_INDEX: usize = ${defCtx};
pub const FIRST_ADDTL_CONTEXT_INDEX: usize = ${firstAddtl};

/// 快照 blob 头布局 (原版 snapshot.cc 匿名 namespace, uint32 槽, 小端):
/// [0] 上下文数 | [1] 可重散列标志 | [2] 校验和 | [3] RO 快照校验和
/// [4..67] 64 字节版本串 | [68] startup 偏移 | [69] 共享堆偏移 | [70..] 各上下文偏移
pub mod snapshot_blob_layout {
  pub const NUMBER_OF_CONTEXTS_OFFSET: u32 = ${layout.kNumberOfContextsOffset};
  pub const REHASHABILITY_OFFSET: u32 = ${layout.kRehashabilityOffset};
  pub const CHECKSUM_OFFSET: u32 = ${layout.kChecksumOffset};
  pub const READ_ONLY_SNAPSHOT_CHECKSUM_OFFSET: u32 = ${layout.kReadOnlySnapshotChecksumOffset};
  pub const VERSION_STRING_OFFSET: u32 = ${layout.kVersionStringOffset};
  pub const VERSION_STRING_LENGTH: u32 = ${layout.kVersionStringLength};
  pub const STARTUP_OFFSET_OFFSET: u32 = ${layout.kStartupOffsetOffset};
  pub const SHARED_HEAP_OFFSET_OFFSET: u32 = ${layout.kSharedHeapOffsetOffset};
  pub const FIRST_CONTEXT_OFFSET_OFFSET: u32 = ${layout.kFirstContextOffsetOffset};

  /// RO 快照紧跟头后, POINTER_SIZE_ALIGN(8) 对齐
  /// (原版 ReadOnlySnapshotOffset(num_contexts))
  #[inline]
  #[must_use]
  pub const fn read_only_snapshot_offset(num_contexts: u32) -> u32 {
    (FIRST_CONTEXT_OFFSET_OFFSET + num_contexts * 4 + 7) & !7
  }

  /// 原版 ContextSnapshotOffsetOffset(index)
  #[inline]
  #[must_use]
  pub const fn context_snapshot_offset_offset(index: u32) -> u32 {
    FIRST_CONTEXT_OFFSET_OFFSET + index * 4
  }
}

// —— 提取不变式断言 ——
const _: () = {
  assert!(SerializerFlags::ALLOW_UNKNOWN_EXTERNAL_REFERENCES_FOR_TESTING == 1);
  assert!(SerializerFlags::ALLOW_ACTIVE_ISOLATE_FOR_TESTING == 2);
  assert!(SerializerFlags::RECONSTRUCT_READ_ONLY_AND_SHARED_OBJECT_CACHES_FOR_TESTING == 4);
  assert!(SerializerFlags::ALLOW_SERIALIZING_ALL_TRUSTED_OBJECTS == 8);
  assert!(DEFAULT_CONTEXT_INDEX == 0);
  assert!(FIRST_ADDTL_CONTEXT_INDEX == 1);
  assert!(snapshot_blob_layout::VERSION_STRING_LENGTH == 64);
  assert!(snapshot_blob_layout::FIRST_CONTEXT_OFFSET_OFFSET == 72);
  assert!(snapshot_blob_layout::read_only_snapshot_offset(0) == 72);
  assert!(snapshot_blob_layout::read_only_snapshot_offset(2) == 80);
};
`;

const p = writeSrc("v8-snapshot", "snapshot.rs", rust);
console.log(`written ${p}`);
