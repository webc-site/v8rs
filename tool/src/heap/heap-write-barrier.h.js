#!/usr/bin/env bun
// tool/src/heap/heap-write-barrier.h.js
// 机械提取 v8/src/heap/heap-write-barrier.{h,cc,-inl.h}:
// WriteBarrier 静态类 For* 台账 + WriteBarrierMode 枚举 (定义于 src/objects/objects.h) +
// IsSkipWriteBarrierMode 规则 (mode <= UNSAFE_SKIP_WRITE_BARRIER) +
// kUninterestingPagesCanBeSkipped 常量 + FromCode 蹦床 5。
// 注: 现有 mark-compact.h.js / sweeper.h.js 属其他族 (禁碰)。

import { readV8, writeSrc, banner } from "../../lib.js";

const h = readV8("src/heap/heap-write-barrier.h");
const cc = readV8("src/heap/heap-write-barrier.cc");
const inl = readV8("src/heap/heap-write-barrier-inl.h");
const objectsH = readV8("src/objects/objects.h");

// —— WriteBarrierMode 枚举 (objects.h) 顺序+计数锚 ——
const wbm = objectsH.match(/enum WriteBarrierMode \{([\s\S]*?)\};/);
if (!wbm) throw new Error("objects.h WriteBarrierMode 缺失");
const modes = wbm[1].replace(/\/\/[^\n]*/g, "").split(",").map((s) => s.trim()).filter(Boolean);
if (modes.join() !== "SKIP_WRITE_BARRIER,SKIP_WRITE_BARRIER_SCOPE,SKIP_WRITE_BARRIER_FOR_GC,UNSAFE_SKIP_WRITE_BARRIER,UPDATE_EPHEMERON_KEY_WRITE_BARRIER,UPDATE_WRITE_BARRIER") {
  throw new Error(`WriteBarrierMode 漂移: ${modes}`);
}

// —— h 台账锚 ——
for (const a of [
  "class V8_EXPORT_PRIVATE V8_NODISCARD WriteBarrierModeScope final {",
  "class V8_EXPORT_PRIVATE WriteBarrier final {",
  "static void EphemeronKeyWriteBarrierFromCode(Address raw_object,",
  "static int MarkingFromCode(Address raw_host, Address raw_slot);",
  "static int IndirectPointerMarkingFromCode(Address raw_host, Address raw_slot,",
  "static int SharedMarkingFromCode(Address raw_host, Address raw_slot);",
  "static int SharedFromCode(Address raw_host, Address raw_slot);",
  "static inline WriteBarrierModeScope GetWriteBarrierModeForObject(",
  "static inline void ForEphemeronHashTable(Tagged<EphemeronHashTable> host,",
  "static inline void ForRelocInfo(Tagged<InstructionStream> host,",
  "static inline void ForArrayBufferExtension(Tagged<JSArrayBuffer> host,",
  "static inline void ForExternalPointer(",
  "static inline void ForIndirectPointer(",
  "static inline void ForProtectedPointer(",
  "static constexpr bool kUninterestingPagesCanBeSkipped = true;",
  "static inline bool IsSkipWriteBarrierMode(WriteBarrierMode mode) {",
  "static_assert(SKIP_WRITE_BARRIER == 0 && SKIP_WRITE_BARRIER_SCOPE == 1 &&",
  "return mode <= UNSAFE_SKIP_WRITE_BARRIER;",
  "static inline WriteBarrierMode ComputeWriteBarrierModeForObject(",
  "static inline void CombinedWriteBarrierInternal(",
  "static inline void CombinedWriteBarrierInternalForStickyMarkbits(",
]) {
  if (!h.includes(a)) throw new Error(`heap-write-barrier.h 锚点缺失: ${a}`);
}
// ForValue 3 重载计数
if ((h.match(/static inline void ForValue\(/g) || []).length < 3) throw new Error("ForValue 重载数漂移");
// —— cc / inl ——
for (const a of [
  "void WriteBarrier::EphemeronKeyWriteBarrierFromCode(",
  "int WriteBarrier::MarkingFromCode(",
  "int WriteBarrier::IndirectPointerMarkingFromCode(",
  "int WriteBarrier::SharedMarkingFromCode(",
  "int WriteBarrier::SharedFromCode(",
]) {
  if (!cc.includes(a)) throw new Error(`heap-write-barrier.cc 锚点缺失: ${a}`);
}
for (const a of [
  "WriteBarrierModeScope WriteBarrier::GetWriteBarrierModeForObject(",
  "WriteBarrierMode WriteBarrier::ComputeWriteBarrierModeForObject(",
  "if (v8_flags.disable_write_barriers) {",
  "if (chunk->IsMarking()) {",
  "if (HeapLayout::InYoungGeneration(chunk, object)) {",
  "bool WriteBarrier::IsMarking(",
  "void WriteBarrier::CombinedWriteBarrierInternalForStickyMarkbits(",
]) {
  if (!inl.includes(a)) throw new Error(`heap-write-barrier-inl.h 锚点缺失: ${a}`);
}
console.log(`WriteBarrier: 模式 ${modes.length} 值 + For* 台账 + FromCode 蹦床 5`);

let rust = banner(
  "src/heap/heap-write-barrier.h.js",
  "src/heap/heap-write-barrier.{h,cc,-inl.h}",
  "写屏障: For* 分发台账与模式判定",
);

rust += `
/// 写屏障模式 (对应 src/objects/objects.h enum WriteBarrierMode)。
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum WriteBarrierMode {
  /// 跳过 (静态消除, 新对象场景)
  SKIP_WRITE_BARRIER = 0,
  /// 跳过 (运行时消除, 经 GetWriteBarrierMode 判新生代)
  SKIP_WRITE_BARRIER_SCOPE = 1,
  /// GC 原子停顿期跳过 (GC 自管显式屏障)
  SKIP_WRITE_BARRIER_FOR_GC = 2,
  /// CSA/Turbofan 内跳过 (MemoryOptimizer 验证用)
  UNSAFE_SKIP_WRITE_BARRIER = 3,
  /// ephemeron 键专用屏障
  UPDATE_EPHEMERON_KEY_WRITE_BARRIER = 4,
  /// 常规屏障
  UPDATE_WRITE_BARRIER = 5,
}

/// 对应 IsSkipWriteBarrierMode: mode <= UNSAFE_SKIP_WRITE_BARRIER 即跳过型。
/// (static_assert 钉死 0/1/2/3 为四个 skip 值)
/// v8: src/heap/heap-write-barrier.h WriteBarrier::IsSkipWriteBarrierMode
#[must_use]
pub fn is_skip_write_barrier_mode(mode: WriteBarrierMode) -> bool {
  mode <= WriteBarrierMode::UNSAFE_SKIP_WRITE_BARRIER
}

/// 对应 kUninterestingPagesCanBeSkipped = true:
/// 宿主页无 PointersFromHereAreInteresting 标志时本机代码直接跳过后续处理。
pub const K_UNINTERESTING_PAGES_CAN_BE_SKIPPED: bool = true;

/// 写屏障静态类骨架 (对应 class WriteBarrier final)。
/// v8: src/heap/heap-write-barrier.h WriteBarrier
pub struct WriteBarrier;

impl WriteBarrier {
  /// 对应 ComputeWriteBarrierModeForObject 的四级判定:
  /// ① disable_write_barriers → SKIP; ② 页在标记 → UPDATE;
  /// ③ 对象在新生代 → SKIP_SCOPE; ④ 其余 → UPDATE
  /// v8: src/heap/heap-write-barrier-inl.h WriteBarrier::ComputeWriteBarrierModeForObject
  #[must_use]
  pub fn compute_mode_for_object(
    disable_write_barriers: bool,
    chunk_is_marking: bool,
    in_young_generation: bool,
  ) -> WriteBarrierMode {
    if disable_write_barriers {
      return WriteBarrierMode::SKIP_WRITE_BARRIER;
    }
    if chunk_is_marking {
      return WriteBarrierMode::UPDATE_WRITE_BARRIER;
    }
    if in_young_generation {
      return WriteBarrierMode::SKIP_WRITE_BARRIER_SCOPE;
    }
    WriteBarrierMode::UPDATE_WRITE_BARRIER
  }

  /// 对应 ForValue/ForRelocInfo/ForArrayBufferExtension/ForExternalPointer/
  /// ForIndirectPointer/ForProtectedPointer/ForEphemeronHashTable 分发面 —
  /// 全部汇入 CombinedWriteBarrierInternal
  /// v8: src/heap/heap-write-barrier-inl.h WriteBarrier::CombinedWriteBarrierInternal
  pub fn combined_write_barrier_internal(_host: u64, _slot: u64, _value: u64) {
    unimplemented!("骨架: marking/shared/old-to-new 分派待 GC 族就位");
  }

  /// 对应 MarkingFromCode (生成代码蹦床, 返回 barrier 语义码)
  /// v8: src/heap/heap-write-barrier.cc WriteBarrier::MarkingFromCode
  pub fn marking_from_code(_raw_host: u64, _raw_slot: u64) -> i32 {
    unimplemented!("骨架: 蹦床待 isolate/marking 就位");
  }
}

// —— 提取不变式断言 ——
const _: () = {
  assert_eq!(WriteBarrierMode::SKIP_WRITE_BARRIER as u8, 0);
  assert_eq!(WriteBarrierMode::UNSAFE_SKIP_WRITE_BARRIER as u8, 3);
  assert_eq!(WriteBarrierMode::UPDATE_EPHEMERON_KEY_WRITE_BARRIER as u8, 4);
  assert_eq!(WriteBarrierMode::UPDATE_WRITE_BARRIER as u8, 5);
  assert!(is_skip_write_barrier_mode(WriteBarrierMode::SKIP_WRITE_BARRIER));
  assert!(is_skip_write_barrier_mode(WriteBarrierMode::UNSAFE_SKIP_WRITE_BARRIER));
  assert!(!is_skip_write_barrier_mode(WriteBarrierMode::UPDATE_WRITE_BARRIER));
  assert!(!is_skip_write_barrier_mode(WriteBarrierMode::UPDATE_EPHEMERON_KEY_WRITE_BARRIER));
  assert_eq!(WriteBarrier::compute_mode_for_object(true, false, false), WriteBarrierMode::SKIP_WRITE_BARRIER);
  assert_eq!(WriteBarrier::compute_mode_for_object(false, true, true), WriteBarrierMode::UPDATE_WRITE_BARRIER);
  assert_eq!(WriteBarrier::compute_mode_for_object(false, false, true), WriteBarrierMode::SKIP_WRITE_BARRIER_SCOPE);
  assert_eq!(WriteBarrier::compute_mode_for_object(false, false, false), WriteBarrierMode::UPDATE_WRITE_BARRIER);
  assert!(K_UNINTERESTING_PAGES_CAN_BE_SKIPPED);
};
`;

const p = writeSrc("v8-heap-core", "heap_write_barrier.rs.skel", rust);
console.log(`written ${p}`);
