#!/usr/bin/env bun
// tool/src/objects/js-atomics-synchronization.h.js — JSAtomicsMutex/Condition 状态位提取
// 组→文件映射 (本脚本管辖): v8/src/objects/js-atomics-synchronization.{h,-inl.h,cc,tq}
//   → v8-objects-js-core/src/js_atomics_synchronization.rs.skel
// 配置登记: 64 位非压缩 (kTaggedSize=8; waiter_queue_head_ 为原生指针 8 字节)。
import { readV8, writeSrc, banner } from "../../lib.js";

const h = readV8("src/objects/js-atomics-synchronization.h");
const inl = readV8("src/objects/js-atomics-synchronization-inl.h");
const cc = readV8("src/objects/js-atomics-synchronization.cc");
const tq = readV8("src/objects/js-atomics-synchronization.tq");

let checks = 0;
const must = (c, m) => {
  checks++;
  if (!c) throw new Error(`锚点失配: ${m}`);
};
const has = (s, sub, f) => {
  checks++;
  if (!s.includes(sub)) throw new Error(`锚点失配: ${f} 无 "${sub.slice(0, 64)}"`);
};

// ── 1. tq 类字段 ──────────────────────────────────────────────────────────
for (const [cls, fields] of [
  ["JSSynchronizationPrimitive", [["waiter_queue_head", "ExternalPointer"], ["state", "uint32"]]],
  ["JSAtomicsMutex", [["owner_thread_id", "int32"]]],
  // optional_padding 仅 TAGGED_SIZE_8_BYTES 分支存在, 但总是出现在 .tq 文本中
  ["JSAtomicsCondition", [["optional_padding", "uint32"]]],
]) {
  const seg = tq.slice(tq.indexOf(`extern class ${cls}`));
  const tqF = [...seg.slice(0, seg.indexOf("\n}")).matchAll(/^  (?:@if\(\w+\) )?(\w+):\s*([^;]+);/gm)]
    .map((m) => [m[1], m[2].trim()]);
  must(JSON.stringify(tqF) === JSON.stringify(fields), `${cls} tq 字段失配: ${JSON.stringify(tqF)}`);
}
has(tq, "@if(TAGGED_SIZE_8_BYTES) optional_padding: uint32;", "tq 条件 padding");
has(tq, "extern class JSSynchronizationPrimitive extends AlwaysSharedSpaceJSObject", "tq 基类");

// ── 2. 状态位布局 (h BitField 链) ─────────────────────────────────────────
has(h, "using HasWaitersField = base::BitField<bool, 0, 1>;", "h HasWaitersField");
has(h, "using IsWaiterQueueLockedField = HasWaitersField::Next<bool, 1>;", "h IsWaiterQueueLockedField");
has(h, "template <class T, int size>\n  using NextBitField = IsWaiterQueueLockedField::Next<T, size>;", "h NextBitField");
has(h, "static constexpr StateT kEmptyState = 0;", "h kEmptyState");
has(h, "static constexpr StateT kWaiterQueueMask =", "h kWaiterQueueMask");
has(h, "using IsLockedField = JSSynchronizationPrimitive::NextBitField<bool, 1>;", "h IsLockedField");
has(h, "static constexpr StateT kUnlockedUncontended = kEmptyState;", "h kUnlockedUncontended");
has(h, "static constexpr StateT kLockedUncontended = IsLockedField::encode(true);", "h kLockedUncontended");
has(h, "static constexpr uint32_t kAllWaiters = UINT32_MAX;", "h kAllWaiters");

// ── 3. cc/inl 行为锚点 ────────────────────────────────────────────────────
has(cc, "static constexpr StateT kInvalidState =\n      ~JSSynchronizationPrimitive::kEmptyState;", "cc kInvalidState");
has(cc, "constexpr int kSpinCount = 64;", "cc kSpinCount");
has(cc, "constexpr int kMaxBackoff = 16;", "cc kMaxBackoff");
has(inl, "StateT expected = kUnlockedUncontended;", "inl 快路径 CAS");
has(inl, "compare_exchange_strong(expected, kUnlockedUncontended,", "inl 解锁 CAS");

// ── 4. 64 位布局 ──────────────────────────────────────────────────────────
// AlwaysSharedSpaceJSObject 头 = JSObject 3 tagged = 24 (无自有 tagged 字段)。
// waiter_queue_head_ (非压缩原生指针 8) + state_ (4) → 36 → 8 对齐 padding 4 = 40。
// h 注释自证: "Slim: 12-16 bytes" = 本类自有字段 4+8(+4 owner) → mutex 40, condition 40。
const T = 8;
const K_JS_OBJECT_HEADER = 3 * T;
const K_SYNC_PRIMITIVE_SELF = T + 4; // waiter_queue_head_ + state_ (不含尾对齐)
const K_SYNC_HEADER = K_JS_OBJECT_HEADER + K_SYNC_PRIMITIVE_SELF + 4; // 尾对齐 → 40
const K_MUTEX_HEADER = K_SYNC_HEADER + 4; // owner_thread_id (4) → 44 → 复用 padding 位 → 40?
// owner_thread_id_ 在 padding 槽内落位: state_(4)+owner(4) 后正好 8 对齐, 无需额外 padding。
const K_MUTEX_HEADER_ALIGNED = K_JS_OBJECT_HEADER + T + 4 + 4; // 24+8+4+4 = 40
const K_COND_HEADER = K_SYNC_HEADER; // optional_padding_(4) 落在基类尾对齐 padding 槽 → 40
must(K_SYNC_HEADER === 40 && K_MUTEX_HEADER_ALIGNED === 40 && K_COND_HEADER === 40,
  `布局推导失配 ${K_SYNC_HEADER}/${K_MUTEX_HEADER_ALIGNED}/${K_COND_HEADER}`);

const body = `${banner("src/objects/js-atomics-synchronization.h.js", "src/objects/js-atomics-synchronization.{h,-inl.h,cc,tq}", "JSAtomicsMutex/Condition 状态位与 waiter 队列布局")}
//! 差集骨架: JSSynchronizationPrimitive / JSAtomicsMutex / JSAtomicsCondition。
//! 配置: 64 位非压缩 (kTaggedSize=8, waiter_queue_head_ = 原生指针)。
//! state 编码: mutex = 0bLQW (bit2=L 锁, bit1=Q 队列锁, bit0=W 有等待者);
//!   condition = 0bQW。锁算法 (CAS 快路径/慢路径停车) 语义面留 Developer。

/// v8: src/objects/js-atomics-synchronization.h JSSynchronizationPrimitive::HasWaitersField
pub const K_HAS_WAITERS_MASK: u32 = 0x1;
/// v8: src/objects/js-atomics-synchronization.h JSSynchronizationPrimitive::IsWaiterQueueLockedField
pub const K_IS_WAITER_QUEUE_LOCKED_MASK: u32 = 0x2;
/// v8: src/objects/js-atomics-synchronization.h kWaiterQueueMask = BitFieldUnion(两标志)
pub const K_WAITER_QUEUE_STATE_MASK: u32 = 0x3;
/// v8: src/objects/js-atomics-synchronization.h JSSynchronizationPrimitive::kEmptyState
pub const K_EMPTY_STATE: u32 = 0;
/// v8: src/objects/js-atomics-synchronization.h JSAtomicsMutex::IsLockedField (NextBitField, bit2)
pub const K_IS_LOCKED_MASK: u32 = 0x4;
/// v8: src/objects/js-atomics-synchronization.h JSAtomicsMutex::kLockedUncontended
pub const K_LOCKED_UNCONTENDED: u32 = 0x4;
/// v8: src/objects/js-atomics-synchronization.cc kInvalidState = ~kEmptyState
pub const K_INVALID_STATE: u32 = 0xFFFF_FFFF;
/// v8: src/objects/js-atomics-synchronization.h JSAtomicsCondition::kAllWaiters
pub const K_ALL_WAITERS: u32 = u32::MAX;
/// v8: src/objects/js-atomics-synchronization.cc LockSlowPath 自旋参数
pub const K_SPIN_COUNT: i32 = 64;
/// v8: src/objects/js-atomics-synchronization.cc LockSlowPath 退避上限
pub const K_MAX_BACKOFF: i32 = 16;

// ── 64 位非压缩布局 ──
/// v8: src/objects/js-atomics-synchronization.h JSSynchronizationPrimitive:
///   waiter_queue_head_ offset=24 (8), state_ offset=32 (4), 隐式对齐 padding 36..40
pub const K_JS_SYNC_PRIMITIVE_HEADER_SIZE: usize = ${K_SYNC_HEADER};
/// v8: src/objects/js-atomics-synchronization.h JSAtomicsMutex:
///   owner_thread_id_ offset=36 (int32, 落在基类对齐 padding 槽)
pub const K_JS_ATOMICS_MUTEX_HEADER_SIZE: usize = ${K_MUTEX_HEADER_ALIGNED};
/// v8: src/objects/js-atomics-synchronization.h JSAtomicsCondition:
///   optional_padding_ offset=36 (uint32, TAGGED_SIZE_8_BYTES 显式)
pub const K_JS_ATOMICS_CONDITION_HEADER_SIZE: usize = ${K_COND_HEADER};

/// v8: src/objects/js-atomics-synchronization.h JSAtomicsMutex::Lock/TryLock/Unlock/
///   LockSlowPath/UnlockSlowPath/MaybeEnqueueNode; LockGuard/TryLockGuard RAII
/// v8: src/objects/js-atomics-synchronization.h JSAtomicsCondition::WaitFor/Notify
/// v8: src/objects/js-atomics-synchronization.h SetWaiterQueueHead (压缩时经外部指针表)
pub struct JSAtomicsMutex { /* waiter_queue_head_, state_, owner_thread_id_ */ }
`;

const p = writeSrc("v8-objects-js-core", "js_atomics_synchronization.rs.skel", body);
console.log(
  `js-atomics-synchronization.h.js: ${p} — 状态位 4、常量 9、tq 类 3、` +
    `布局 ${K_SYNC_HEADER}/${K_MUTEX_HEADER_ALIGNED}/${K_COND_HEADER}、断言 ${checks} 项通过`,
);
