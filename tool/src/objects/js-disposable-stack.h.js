#!/usr/bin/env bun
// tool/src/objects/js-disposable-stack.h.js — JSDisposableStackBase 枚举/位域提取
// 组→文件映射 (本脚本管辖): v8/src/objects/js-disposable-stack.{h,-inl.h,cc,tq}
//   → v8-objects-js-core/src/js_disposable_stack.rs.skel
// 配置登记: 64 位非压缩 (kTaggedSize=8, JSObject 头 = 3 tagged = 24)。
import { readV8, writeSrc, banner } from "../../lib.js";

const h = readV8("src/objects/js-disposable-stack.h");
const inl = readV8("src/objects/js-disposable-stack-inl.h");
const cc = readV8("src/objects/js-disposable-stack.cc");
const tq = readV8("src/objects/js-disposable-stack.tq");

let checks = 0;
const must = (c, m) => {
  checks++;
  if (!c) throw new Error(`锚点失配: ${m}`);
};
const has = (s, sub, f) => {
  checks++;
  if (!s.includes(sub)) throw new Error(`锚点失配: ${f} 无 "${sub.slice(0, 64)}"`);
};

// ── 1. 四个枚举 (tq extern enum 真解析 + h 对照) ──────────────────────────
function tqEnum(name) {
  const seg = tq.slice(tq.indexOf(`extern enum ${name}`));
  return seg.slice(seg.indexOf("{") + 1, seg.indexOf("}"))
    .split(",").map((s) => s.trim()).filter(Boolean);
}
must(JSON.stringify(tqEnum("DisposableStackState")) === JSON.stringify(["kDisposed", "kPending"]), "DisposableStackState 失配");
must(JSON.stringify(tqEnum("DisposeMethodCallType")) === JSON.stringify(["kValueIsReceiver", "kValueIsArgument"]), "DisposeMethodCallType 失配");
must(JSON.stringify(tqEnum("DisposeMethodHint")) === JSON.stringify(["kSyncDispose", "kAsyncDispose"]), "DisposeMethodHint 失配");
must(JSON.stringify(tqEnum("DisposableStackResourcesType")) === JSON.stringify(["kAllSync", "kAtLeastOneAsync"]), "DisposableStackResourcesType 失配");
has(h, "enum class DisposableStackState { kDisposed, kPending };", "h DisposableStackState");
has(h, "enum class DisposeMethodCallType { kValueIsReceiver = 0, kValueIsArgument = 1 };", "h DisposeMethodCallType");
has(h, "enum class DisposeMethodHint { kSyncDispose = 0, kAsyncDispose = 1 };", "h DisposeMethodHint");

// ── 2. status 位域 (h + tq 对数; inl torque_asserts 5 项) ──────────────────
has(h, "using StateBit = base::BitField<DisposableStackState, 0, 1>;", "h StateBit");
has(h, "using NeedsAwaitBit = base::BitField<bool, 1, 1>;", "h NeedsAwaitBit");
has(h, "using HasAwaitedBit = base::BitField<bool, 2, 1>;", "h HasAwaitedBit");
has(h, "using SuppressedErrorCreatedBit = base::BitField<bool, 3, 1>;", "h SuppressedErrorCreatedBit");
has(h, "using LengthBits = base::BitField<int32_t, 4, 27>;", "h LengthBits");
const bf = tq.slice(tq.indexOf("bitfield struct DisposableStackStatus"));
const bfF = [...bf.slice(0, bf.indexOf("}")).matchAll(/(\w+): \w+.*?: (\d+) bit/g)].map((m) => [m[1], +m[2]]);
must(JSON.stringify(bfF) === JSON.stringify([
  ["state", 1], ["needs_await", 1], ["has_awaited", 1], ["suppressed_error_created", 1], ["length", 27],
]), `tq 位域失配: ${JSON.stringify(bfF)}`);
must(bfF.reduce((a, [, b]) => a + b, 0) === 31, "status 总位宽应 31");
must((inl.match(/^CHECK_BITFIELD\(/gm) || []).length === 5, "inl CHECK_BITFIELD 使用应 5 处");

// ── 3. 资源槽编码 + Context 槽位枚举 ─────────────────────────────────────
has(h, "using DisposeCallTypeBit =\n    base::BitField<DisposeMethodCallType, 0, 1, uint32_t>;", "h DisposeCallTypeBit");
has(h, "using DisposeHintBit = DisposeCallTypeBit::Next<DisposeMethodHint, 1>;", "h DisposeHintBit");
has(tq, "// In the stack, [i] is the value that the initializer evaluates to", "tq 槽说明");
has(h, "enum class AsyncDisposableStackContextSlots {\n    kStack = Context::MIN_CONTEXT_SLOTS,\n    kOuterPromise,\n    kLength,\n  };", "h AsyncDisposableStackContextSlots");
has(h, "enum class AsyncDisposeFromSyncDisposeContextSlots {\n    kMethod = Context::MIN_CONTEXT_SLOTS,\n    kLength,\n  };", "h AsyncDisposeFromSyncDisposeContextSlots");
has(inl, "int stack_type =\n      DisposeCallTypeBit::encode(type) | DisposeHintBit::encode(hint);", "inl 槽编码");

// ── 4. 字段 (4) 与类 ──────────────────────────────────────────────────────
const seg = tq.slice(tq.indexOf("extern class JSDisposableStackBase"));
const tqF = [...seg.slice(0, seg.indexOf("\n}")).matchAll(/^  (\w+):\s*([^;]+);/gm)]
  .map((m) => [m[1], m[2].replace(/\s+/g, " ").trim()]);
must(JSON.stringify(tqF) === JSON.stringify([
  ["stack", "FixedArray"], ["status", "SmiTagged<DisposableStackStatus>"],
  ["error", "Object|Hole"], ["error_message", "Object|Hole"],
]), `tq 字段失配: ${JSON.stringify(tqF)}`);
has(h, "TaggedMember<FixedArray> stack_;", "h stack_");
has(h, "TaggedMember<Smi> status_ V8_TQ_TYPE(SmiTagged<DisposableStackStatus>);", "h status_");
has(h, "class JSSyncDisposableStack final : public JSDisposableStackBase", "h Sync");
has(h, "class JSAsyncDisposableStack final : public JSDisposableStackBase", "h Async");
has(cc, "NextDisposeAsyncIteration", "cc NextDisposeAsyncIteration");

// ── 5. 64 位布局 ──────────────────────────────────────────────────────────
const T = 8;
const K_HEADER = 3 * T + 4 * T; // 24 + 4 字段
must(K_HEADER === 56, `布局推导失配 ${K_HEADER}`);

const body = `${banner("src/objects/js-disposable-stack.h.js", "src/objects/js-disposable-stack.{h,-inl.h,cc,tq}", "JSDisposableStackBase 枚举/status 位域/资源槽编码骨架")}
//! 差集骨架: JSDisposableStackBase (explicit resource management) + Sync/Async 两子类。
//! 配置: 64 位非压缩 (kTaggedSize=8)。dispose 语义面留 Developer。

/// v8: src/objects/js-disposable-stack.h DisposableStackState
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DisposableStackState { Disposed = 0, Pending = 1 }

/// v8: src/objects/js-disposable-stack.h DisposeMethodCallType
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DisposeMethodCallType { ValueIsReceiver = 0, ValueIsArgument = 1 }

/// v8: src/objects/js-disposable-stack.h DisposeMethodHint
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DisposeMethodHint { SyncDispose = 0, AsyncDispose = 1 }

/// v8: src/objects/js-disposable-stack.h DisposableStackResourcesType
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DisposableStackResourcesType { AllSync = 0, AtLeastOneAsync = 1 }

// ── status_ 位域 (uint31 Smi; h BitField = tq DisposableStackStatus, inl torque_asserts 校平) ──
pub const K_STATE_MASK: u32 = 0x1;
pub const K_NEEDS_AWAIT_SHIFT: u32 = 1;
pub const K_NEEDS_AWAIT_MASK: u32 = 0x1 << 1;
pub const K_HAS_AWAITED_SHIFT: u32 = 2;
pub const K_HAS_AWAITED_MASK: u32 = 0x1 << 2;
pub const K_SUPPRESSED_ERROR_CREATED_SHIFT: u32 = 3;
pub const K_SUPPRESSED_ERROR_CREATED_MASK: u32 = 0x1 << 3;
/// v8: src/objects/js-disposable-stack.h LengthBits (int32, 27 位)
pub const K_LENGTH_SHIFT: u32 = 4;
pub const K_LENGTH_BITS: u32 = 27;
pub const K_LENGTH_MASK: u32 = ((1 << 27) - 1) << 4;

/// v8: src/objects/js-disposable-stack.h DisposeCallTypeBit(bit0) + DisposeHintBit(bit1):
///   stack_[i+2] = SmiTag(encode(type) | encode(hint)) — 资源三元组第三槽编码
pub const K_DISPOSE_CALL_TYPE_MASK: u32 = 0x1;
pub const K_DISPOSE_HINT_SHIFT: u32 = 1;
pub const K_DISPOSE_HINT_MASK: u32 = 0x1 << 1;

/// v8: src/objects/js-disposable-stack.h JSDisposableStackBase: stack_ offset=24 (FixedArray,
///   三元组 [value, method, type|hint]), status_ 32 (Smi), error_ 40 (Object|Hole),
///   error_message_ 48 (Object|Hole)
pub const K_JS_DISPOSABLE_STACK_BASE_HEADER_SIZE: usize = ${K_HEADER};

/// v8: src/objects/js-disposable-stack.h AsyncDisposableStackContextSlots (kStack/kOuterPromise/kLength)
/// v8: src/objects/js-disposable-stack.h AsyncDisposeFromSyncDisposeContextSlots (kMethod/kLength)
pub struct JSDisposableStackBase { /* stack_, status_, error_, error_message_ */ }
`;

const p = writeSrc("v8-objects-js-core", "js_disposable_stack.rs.skel", body);
console.log(
  `js-disposable-stack.h.js: ${p} — 枚举 ${4} (2+2+2+2 值)、status 位域 ${bfF.length}/5 (31 位)、` +
    `资源槽编码 2 位、字段 ${tqF.length}/4、布局 ${K_HEADER}、断言 ${checks} 项通过`,
);
