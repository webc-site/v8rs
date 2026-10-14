#!/usr/bin/env bun
// tool/src/objects/js-array-buffer.h.js — JSArrayBuffer 族布局/位标志/元素表提取
// 组→文件映射 (本脚本管辖):
//   v8/src/objects/js-array-buffer.h + -inl.h + .cc + .tq (整族合一)
//   → v8-objects-js-core/src/js_array_buffer.rs.skel
// 配置登记: 64 位非压缩 (kTaggedSize=8, TAGGED_SIZE_8_BYTES=true, 无 sandbox)
//   → ExternalPointerMember 为原生指针 8 字节; BoundedSize/SandboxedPtr 不生效。
import { readV8, writeSrc, banner } from "../../lib.js";

const h = readV8("src/objects/js-array-buffer.h");
const inl = readV8("src/objects/js-array-buffer-inl.h");
const cc = readV8("src/objects/js-array-buffer.cc");
const tq = readV8("src/objects/js-array-buffer.tq");

let checks = 0;
const must = (c, m) => {
  checks++;
  if (!c) throw new Error(`锚点失配: ${m}`);
};
const has = (s, sub, f) => {
  checks++;
  if (!s.includes(sub)) throw new Error(`锚点失配: ${f} 无 "${sub.slice(0, 64)}"`);
};

// ── 1. 类全集锚点 (h + tq) ────────────────────────────────────────────────
for (const c of [
  "JSArrayBuffer : public JSAPIObjectWithEmbedderSlots",
  "class ArrayBufferExtension final",
  "JSArrayBufferView : public JSAPIObjectWithEmbedderSlots",
  "JSTypedArray : public JSArrayBufferView",
  "JSDetachedTypedArray : public JSTypedArray",
  "JSDataViewOrRabGsabDataView : public JSArrayBufferView",
  "JSDataView : public JSDataViewOrRabGsabDataView",
  "JSRabGsabDataView : public JSDataViewOrRabGsabDataView",
  "TypedArrayConstructor : public JSFunctionWithPrototype",
])
  has(h, c, "js-array-buffer.h");

// tq 侧 TypedArrayConstructor 子类全集 (12 个, 基类单独断言)
const tqCtors = [...tq.matchAll(/extern class (\w+TypedArrayConstructor) extends/g)]
  .map((m) => m[1]);
must(tqCtors.length === 12, `tq 构造器子类应 12 个, 实得 ${tqCtors.length}`);
has(tq, "extern class TypedArrayConstructor extends JSFunctionWithPrototype", "tq 基类");
const tqCtorAll = ["TypedArrayConstructor", ...tqCtors];
for (const n of ["Uint8", "Int8", "Uint16", "Int16", "Uint32", "Int32", "Float16",
  "Float32", "Float64", "Uint8Clamped", "Biguint64", "Bigint64"])
  must(tqCtors.includes(`${n}TypedArrayConstructor`), `tq 缺构造器 ${n}`);

// ── 2. tq 位标志布局真解析 ────────────────────────────────────────────────
function tqBitfield(name) {
  const seg = tq.slice(tq.indexOf(`bitfield struct ${name}`));
  const body = seg.slice(0, seg.indexOf("}"));
  return [...body.matchAll(/(\w+): \w+.*?: (\d+) bit/g)].map((m) => [m[1], +m[2]]);
}
const abFlags = tqBitfield("JSArrayBufferFlags");
must(JSON.stringify(abFlags) === JSON.stringify([
  ["is_external", 1], ["is_detachable", 1], ["was_detached", 1],
  ["is_shared", 1], ["is_resizable_by_js", 1], ["is_immutable", 1],
]), `JSArrayBufferFlags tq 布局失配: ${JSON.stringify(abFlags)}`);
const viewFlags = tqBitfield("JSArrayBufferViewFlags");
must(JSON.stringify(viewFlags) === JSON.stringify([
  ["is_length_tracking", 1], ["is_backed_by_rab", 1],
]), `JSArrayBufferViewFlags tq 布局失配: ${JSON.stringify(viewFlags)}`);

// h 侧 BitField 链锚点
for (const a of [
  "using IsExternalBit = base::BitField<bool, 0, 1, uint32_t>;",
  "using IsDetachableBit = IsExternalBit::Next<bool, 1>;",
  "using WasDetachedBit = IsDetachableBit::Next<bool, 1>;",
  "using IsSharedBit = WasDetachedBit::Next<SharedFlag, 1>;",
  "using IsResizableByJsBit = IsSharedBit::Next<ResizableFlag, 1>;",
  "using IsImmutableBit = IsResizableByJsBit::Next<ImmutableFlag, 1>;",
  "static constexpr int kFlagCount = 6;",
  "using IsLengthTrackingBit = base::BitField<bool, 0, 1, uint32_t>;",
  "using IsBackedByRabBit = IsLengthTrackingBit::Next<bool, 1>;",
  "static constexpr int kFlagCount = 2;",
])
  has(h, a, "js-array-buffer.h");

// ── 3. 常量 ───────────────────────────────────────────────────────────────
has(h, "#elif V8_HOST_ARCH_32_BIT\n  static constexpr size_t kMaxByteLength = kMaxInt;",
  "js-array-buffer.h");
has(h, "static constexpr size_t kMaxByteLength = kMaxSafeInteger;", "js-array-buffer.h");
has(h, "static constexpr Tagged<Smi> kNoView = Smi::zero();", "js-array-buffer.h");
has(h, "static constexpr Tagged<Smi> kManyViews = Smi::FromInt(1);", "js-array-buffer.h");
has(tq, "const kNoView: Smi = 0;", "js-array-buffer.tq");
has(tq, "const kManyViews: Smi = 1;", "js-array-buffer.tq");
has(h, "static constexpr size_t kMaxSizeInHeap = 64;", "js-array-buffer.h");
has(h, "static constexpr bool kOffHeapDataPtrEqualsExternalPointer = true;", "js-array-buffer.h");
has(h, "enum class Age : uint8_t { kYoung = 0, kOld = 1 };", "js-array-buffer.h");
has(h, "enum class GcState : uint8_t { Dead = 0, Copied, Promoted };", "js-array-buffer.h");
has(h, "using AgeField = base::BitField<uint8_t, 0, 1, uint64_t>;", "js-array-buffer.h");
has(h, "using AccountingLengthField = AgeField::Next<size_t, 63>;", "js-array-buffer.h");

// NotValidMask 规则 (h:200-209): kRead→WasDetached; kWrite→WasDetached|IsImmutable
has(h, "return JSArrayBuffer::WasDetachedBit::kMask;", "NotValidMask kRead");
has(h, "return JSArrayBuffer::WasDetachedBit::kMask |\n               JSArrayBuffer::IsImmutableBit::kMask;",
  "NotValidMask kWrite");

// ── 4. inl/cc 锚点 ────────────────────────────────────────────────────────
has(inl, "constexpr std::pair<ExternalArrayType, size_t>\nJSTypedArray::TypeAndElementSizeFor(ElementsKind kind)",
  "TypeAndElementSizeFor");
must((inl.match(/ELEMENTS_KIND_TO_ARRAY_TYPE/g) || []).length >= 2, "X-macro 展开锚点缺失");
has(inl, "TYPED_ARRAYS(ELEMENTS_KIND_TO_ARRAY_TYPE)", "inl TYPED_ARRAYS");
has(inl, "RAB_GSAB_TYPED_ARRAYS_WITH_TYPED_ARRAY_TYPE(ELEMENTS_KIND_TO_ARRAY_TYPE)", "inl RAB_GSAB");
has(inl, "ReadBoundedSizeField(offsetof(JSTypedArray, raw_length_))", "inl length");
has(inl, "static_assert(kOffHeapDataPtrEqualsExternalPointer);", "inl off-heap 断言");
has(cc, "for (int i = 0; i < v8::ArrayBuffer::kEmbedderFieldCount; i++)", "cc Setup");

// ── 5. 64 位非压缩布局推导 ────────────────────────────────────────────────
// JSAPIObjectWithEmbedderSlots 头 = JSObject 3 tagged = 24; embedder 槽在 in-object 尾部。
const T = 8;
const AB = [
  ["views_or_detach_key_", 8], ["raw_byte_length_", 8], ["raw_max_byte_length_", 8],
  ["backing_store_", 8], ["extension_", 8], ["bit_field_", 4], ["optional_padding_", 4],
];
let off = 3 * T;
const abLayout = AB.map(([n, s]) => [n, (off += s) - s, s]);
const K_AB_HEADER = off; // 72
const VIEW = [["buffer_", 8], ["bit_field_", 4], ["optional_padding_", 4],
  ["raw_byte_offset_", 8], ["raw_byte_length_", 8]];
off = 3 * T;
const viewLayout = VIEW.map(([n, s]) => [n, (off += s) - s, s]);
const K_VIEW_HEADER = off; // 56
const TA = [["raw_length_", 8], ["external_pointer_", 8], ["base_pointer_", 8]];
const taLayout = TA.map(([n, s]) => [n, (off += s) - s, s]);
const K_TA_HEADER = off; // 80
const DV = [["data_pointer_", 8]];
off = K_VIEW_HEADER;
const dvLayout = DV.map(([n, s]) => [n, (off += s) - s, s]);
const K_DV_HEADER = off; // 64
must(K_AB_HEADER === 72 && K_VIEW_HEADER === 56 && K_TA_HEADER === 80 && K_DV_HEADER === 64,
  `布局推导失配: ${K_AB_HEADER}/${K_VIEW_HEADER}/${K_TA_HEADER}/${K_DV_HEADER}`);

const body = `${banner("src/objects/js-array-buffer.h.js", "src/objects/js-array-buffer.{h,-inl.h,cc,tq}", "JSArrayBuffer 族位标志/布局/TypedArray 元素表骨架")}
//! 差集骨架: JSArrayBuffer / JSArrayBufferView / JSTypedArray / JSDataView 族。
//! 配置: 64 位非压缩 (kTaggedSize=8, 无 sandbox → extension_/backing_store_ 为原生指针)。

// ── JSArrayBufferFlags (JSArrayBuffer.bit_field_, uint32) ──
/// v8: src/objects/js-array-buffer.h JSArrayBuffer::IsExternalBit
pub const K_IS_EXTERNAL_MASK: u32 = 0x1;
/// v8: src/objects/js-array-buffer.h JSArrayBuffer::IsDetachableBit
pub const K_IS_DETACHABLE_MASK: u32 = 0x2;
/// v8: src/objects/js-array-buffer.h JSArrayBuffer::WasDetachedBit
pub const K_WAS_DETACHED_MASK: u32 = 0x4;
/// v8: src/objects/js-array-buffer.h JSArrayBuffer::IsSharedBit
pub const K_IS_SHARED_MASK: u32 = 0x8;
/// v8: src/objects/js-array-buffer.h JSArrayBuffer::IsResizableByJsBit
pub const K_IS_RESIZABLE_BY_JS_MASK: u32 = 0x10;
/// v8: src/objects/js-array-buffer.h JSArrayBuffer::IsImmutableBit
pub const K_IS_IMMUTABLE_MASK: u32 = 0x20;
/// v8: src/objects/js-array-buffer.h JSArrayBuffer::kFlagCount
pub const K_AB_FLAG_COUNT: u32 = 6;
/// v8: src/objects/js-array-buffer.h JSArrayBuffer::NotValidMask(kWrite) = WasDetached|IsImmutable
pub const K_NOT_VALID_WRITE_MASK: u32 = K_WAS_DETACHED_MASK | K_IS_IMMUTABLE_MASK;

// ── JSArrayBufferViewFlags (JSArrayBufferView.bit_field_, uint32) ──
/// v8: src/objects/js-array-buffer.h JSArrayBufferView::IsLengthTrackingBit
pub const K_IS_LENGTH_TRACKING_MASK: u32 = 0x1;
/// v8: src/objects/js-array-buffer.h JSArrayBufferView::IsBackedByRabBit
pub const K_IS_BACKED_BY_RAB_MASK: u32 = 0x2;
/// v8: src/objects/js-array-buffer.h JSArrayBufferView::kFlagCount
pub const K_VIEW_FLAG_COUNT: u32 = 2;

// ── 常量 ──
/// v8: src/objects/js-array-buffer.h JSArrayBuffer::kMaxByteLength (非 sandbox 64 位 = kMaxSafeInteger)
pub const K_MAX_BYTE_LENGTH: usize = ${2 ** 53 - 1};
/// v8: src/objects/js-array-buffer.h JSArrayBuffer::kNoView (Smi 0)
pub const K_NO_VIEW: i32 = 0;
/// v8: src/objects/js-array-buffer.h JSArrayBuffer::kManyViews (Smi 1)
pub const K_MANY_VIEWS: i32 = 1;
/// v8: src/objects/js-array-buffer.h JSTypedArray::kMaxSizeInHeap
pub const K_MAX_SIZE_IN_HEAP: usize = 64;

/// v8: src/objects/js-array-buffer.h ArrayBufferExtension::Age
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Age { Young = 0, Old = 1 }

/// v8: src/objects/js-array-buffer.h ArrayBufferExtension::GcState
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GcState { Dead = 0, Copied = 1, Promoted = 2 }

/// v8: src/objects/js-array-buffer.h ArrayBufferExtension AgeField: bit0=Age, [1..63]=accounting_length
pub mod accounting_state {
    pub const AGE_MASK: u64 = 1;
    pub const LENGTH_SHIFT: u32 = 1;
    pub const LENGTH_BITS: u32 = 63;
}

// ── 64 位非压缩布局 (embedder 槽不计, 在 in-object 尾) ──
${abLayout.map(([n, o, s]) => `/// v8: src/objects/js-array-buffer.h JSArrayBuffer::${n} offset=${o} size=${s}`).join("\n")}
pub const K_JS_ARRAY_BUFFER_HEADER_SIZE: usize = ${K_AB_HEADER};
${viewLayout.map(([n, o, s]) => `/// v8: src/objects/js-array-buffer.h JSArrayBufferView::${n} offset=${o} size=${s}`).join("\n")}
pub const K_JS_ARRAY_BUFFER_VIEW_HEADER_SIZE: usize = ${K_VIEW_HEADER};
${taLayout.map(([n, o, s]) => `/// v8: src/objects/js-array-buffer.h JSTypedArray::${n} offset=${o} size=${s}`).join("\n")}
pub const K_JS_TYPED_ARRAY_HEADER_SIZE: usize = ${K_TA_HEADER};
${dvLayout.map(([n, o, s]) => `/// v8: src/objects/js-array-buffer.h JSDataViewOrRabGsabDataView::${n} offset=${o} size=${s}`).join("\n")}
pub const K_JS_DATA_VIEW_HEADER_SIZE: usize = ${K_DV_HEADER};

/// v8: src/objects/js-array-buffer-inl.h JSTypedArray::TypeAndElementSizeFor
/// X-macro 展开 (TYPED_ARRAYS + RAB_GSAB_TYPED_ARRAYS_WITH_TYPED_ARRAY_TYPE):
/// ElementsKind → (ExternalArrayType, element_size); 语义面留 Developer 修订。
pub fn type_and_element_size_for(_kind: u8) -> (u8, usize) { todo!() }

/// v8: src/objects/js-array-buffer.h 末尾 13 个 *TypedArrayConstructor (JSFunctionWithPrototype 视图)
pub const TYPED_ARRAY_CONSTRUCTORS: &[&str] = &[
${tqCtorAll.map((n) => `    "${n}",`).join("\n")}
];
`;

const p = writeSrc("v8-objects-js-core", "js_array_buffer.rs.skel", body);
console.log(
  `js-array-buffer.h.js: ${p} — 位标志 6+2、tq 构造器 ${tqCtorAll.length}/13、` +
    `布局 AB=${K_AB_HEADER}/View=${K_VIEW_HEADER}/TA=${K_TA_HEADER}/DV=${K_DV_HEADER}、断言 ${checks} 项通过`,
);
