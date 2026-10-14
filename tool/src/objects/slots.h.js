#!/usr/bin/env bun
// tool/src/objects/slots.h.js — 全宽槽位族生成
// 组→文件映射 (本脚本管辖):
//   v8/src/objects/slots.h + slots-inl.h + slots-atomic-inl.h → v8-objects-core/src/slots.rs
//   (压缩族 compressed-slots.{h,-inl.h} 归 compressed-slots.h.js → compressed_slots.rs;
//    visitors.rs 里的最小槽位 newtype 占位由本模块接管升级 — 修订环合并)
// 规则面: SlotBase 算术/比较面 + 4 个 Full* 槽类方法面 + 外指/CppHeap/间接指针槽
//   + UnalignedSlot/AtomicSlot 迭代器适配 + Memset 族; 语义 (原子序/表解析) 留骨架
import { readV8, writeSrc, banner } from "../../lib.js";

const h = readV8("src/objects/slots.h");
const inl = readV8("src/objects/slots-inl.h");
const atomic = readV8("src/objects/slots-atomic-inl.h");

let checks = 0;
const need = (cond, msg) => { checks++; if (!cond) throw new Error(msg); };

// —— SlotBase 面 ——
need(/static constexpr size_t kSlotDataSize = sizeof\(Data\);/.test(h), "kSlotDataSize 常量缺失");
need(/static constexpr size_t kSlotDataAlignment = SlotDataAlignment;/.test(h), "kSlotDataAlignment 常量缺失");
const slotOps = [...h.matchAll(/^  (Subclass&|Subclass|bool|size_t) operator([+\-]=]|==|!=|<|<=|>|>=|\+\+|\-\-|\+|\-)?/gm)].length;
need(slotOps >= 14, `SlotBase 运算符面应 ≥14, 实得 ${slotOps}`);

// —— 槽类清单 (h 侧顶层 class) ——
const classes = [...h.matchAll(/^class (\w+)[^;\n]*$/gm)].map((m) => m[1]).filter((n) => n !== "Reference");
const CLASSES = ["SlotBase", "FullObjectSlot", "FullMaybeObjectSlot", "FullHeapObjectSlot", "UnalignedSlot",
  "ExternalPointerSlot", "CppHeapPointerSlot", "IndirectPointerSlot", "WriteProtectedSlot"];
if (classes.join() !== CLASSES.join()) throw new Error(`槽类清单漂移: ${classes}`);

// —— 各类 kCanBeWeak ——
need(/class FullObjectSlot[\s\S]{0,1200}static constexpr bool kCanBeWeak = false;/.test(h), "FullObjectSlot kCanBeWeak 应 false");
need(/class FullMaybeObjectSlot[\s\S]{0,1200}static constexpr bool kCanBeWeak = true;/.test(h), "FullMaybeObjectSlot kCanBeWeak 应 true");

// —— FullObjectSlot 方法面 (声明顺序) ——
const foBlock = h.slice(h.indexOf("class FullObjectSlot"), h.indexOf("// A FullMaybeObjectSlot"));
const foMethods = [...foBlock.matchAll(/^\s*(?:static )?inline [\w:<>, ]*?(\w+)\(/gm)].map((m) => m[1]);
for (const need2 of ["contains_map_value", "Relaxed_ContainsMapValue", "load", "store", "store_map", "load_map",
  "Acquire_Load", "Relaxed_Load", "Relaxed_Load_Raw", "RawToTagged", "Relaxed_Store", "Release_Store",
  "Relaxed_CompareAndSwap", "Release_CompareAndSwap"])
  need(foMethods.includes(need2), `FullObjectSlot 缺 ${need2}`);

// —— ExternalPointerSlot 序列化面 + 句柄面 ——
const epsBlock = h.slice(h.indexOf("class ExternalPointerSlot"), h.indexOf("// Similar to ExternalPointerSlot"));
for (const fn of ["init_lazily_initialized", "init", "Relaxed_LoadHandle", "Relaxed_StoreHandle", "Release_StoreHandle",
  "load", "store", "load_tag", "GetAndClearContentForSerialization", "RestoreContentAfterSerialization",
  "ReplaceContentWithIndexForSerialization", "GetContentAsIndexAfterDeserialization", "ExactTagIsKnown", "exact_tag", "tag_range"])
  need(epsBlock.includes(`${fn}(`), `ExternalPointerSlot 缺 ${fn}`);
need(/HasExternalPointerHandle\(\) const \{\s*\n\s*return V8_ENABLE_SANDBOX_BOOL \|\| tag_range\(\) == kArrayBufferExtensionTag \|\|\s*\n\s*tag_range\(\) == kWaiterQueueNodeTag;/.test(h),
  "HasExternalPointerHandle 三条件缺失");

// —— CppHeapPointerSlot / IndirectPointerSlot 面 ——
const chpsBlock = h.slice(h.indexOf("class CppHeapPointerSlot"), h.indexOf("// An IndirectPointerSlot"));
for (const fn of ["init", "Relaxed_LoadHandle", "Release_StoreHandle", "Relaxed_StoreHandle", "store", "load",
  "GetAndClearContentForSerialization", "RestoreContentAfterSerialization"])
  need(chpsBlock.includes(`${fn}(`) || chpsBlock.includes(`inline ${fn}(`), `CppHeapPointerSlot 缺 ${fn}`);
const ipsBlock = h.slice(h.indexOf("class IndirectPointerSlot"), h.indexOf("class WritableJitAllocation"));
for (const fn of ["load", "store", "Relaxed_Load", "Relaxed_Load_AllowUnpublished", "Acquire_Load", "Relaxed_Store",
  "Release_Store", "Relaxed_LoadHandle", "Acquire_LoadHandle", "Relaxed_StoreHandle", "Release_StoreHandle", "IsEmpty", "ResolveHandle"])
  need(ipsBlock.includes(`${fn}(`), `IndirectPointerSlot 缺 ${fn}`);
need(ipsBlock.includes("enum TagCheckStrictness { kRequireExactMatch, kAllowUnpublishedEntries };"), "TagCheckStrictness 枚举漂移");

// —— 自由函数 ——
for (const fn of ["CopyTagged", "MemsetTagged", "Relaxed_MemsetTagged", "MemsetPointer"])
  need(new RegExp(`inline void ${fn}\\(`).test(h), `自由函数 ${fn} 缺失`);

// —— slots-inl.h 定义面 ——
const inlDefs = {};
for (const cls of ["FullObjectSlot", "FullMaybeObjectSlot", "FullHeapObjectSlot", "ExternalPointerSlot", "CppHeapPointerSlot", "IndirectPointerSlot"]) {
  inlDefs[cls] = [...inl.matchAll(new RegExp(`(?:^|\\n)(?:template <[^>]+>\\n)?[\\w:<>&* ]*${cls}::(\\w+)\\(`, "g"))].map((m) => m[1]).length;
}
const totalDefs = Object.values(inlDefs).reduce((a, b) => a + b, 0);
need(totalDefs >= 60, `slots-inl.h 定义应 ≥60, 实得 ${totalDefs}`);
// 语义锚: 非压缩 RawToTagged 恒等; WriteProtected 走 jit 写授权
need(/Tagged<Object> FullObjectSlot::RawToTagged\(PtrComprCageBase cage_base,\s*\n\s*Address raw\) \{\s*\n\s*return Tagged<Object>\(raw\);/.test(inl), "RawToTagged 恒等实现缺失");
need(/jit_allocation_\.WriteHeaderSlot\(this->address\(\), value, kRelaxedStore\);/.test(inl), "WriteProtectedSlot 写授权语义缺失");

// —— slots-atomic-inl.h: AtomicSlot ——
need(/class AtomicSlot : public SlotBase<AtomicSlot, Tagged_t>/.test(atomic), "AtomicSlot 声明缺失");
const atomicRefOps = [...atomic.matchAll(/^    (?:Reference&|void|bool) operator([=(]|==|<)/gm)].length;
need(atomicRefOps >= 4, `AtomicSlot::Reference 运算符应 ≥4, 实得 ${atomicRefOps}`);

const body = `${banner("src/objects/slots.h.js", "src/objects/slots.h + slots-inl.h + slots-atomic-inl.h", "非压缩基线 Tagged_t=usize; 压缩族在 compressed_slots.rs")}
//! 全宽槽位族 (原版 slots.{h,-inl.h} + slots-atomic-inl.h)
//!
//! 槽 (Slot) = 指向堆内 tagged 字段的裸地址包装. 非压缩基线下
//! \`TData = Address = usize\`, 压缩变体见 compressed_slots.rs.

/// 槽位数据大小 (原版 kSlotDataSize = sizeof(Data))
pub type SlotData = usize;

/// 空槽地址哨兵 (原版 kNullAddress)
pub const K_NULL_ADDRESS: usize = 0;

// ===== SlotBase 算术面 (运算符 ${slotOps} 个: ++/-- ×4, 比较 ×6, +-/+=/-= ×6, 地址/取位置) =====
/// 槽基类公共面 (Rust 侧由各槽 newtype 实现; 步长 = kSlotDataSize)
///
/// - 前后缀 ++/--: ptr_ ± kSlotDataSize
/// - 比较: 裸地址比较
/// - a - b = (ptr 差) / kSlotDataSize; a ± i = ptr ± i*kSlotDataSize
pub fn slot_base_ops_doc() {}

// ===== Full* 槽类 (${CLASSES.length - 5} 个) =====
/// FullObjectSlot 方法面 (原版声明 ${foMethods.length} 个)
pub const FULL_OBJECT_SLOT_METHODS: [&str; ${foMethods.length}] = [${foMethods.map((n) => `"${n}"`).join(", ")}];

/// FullObjectSlot 读 (非压缩: 裸 usize → Tagged; 原版 operator*)
pub fn full_object_slot_load(/* slot: &FullObjectSlot */) /* -> Tagged<Object> */ {
  todo!("就地修订: 非压缩直接包 *location(); 压缩走 DecompressTagged")
}

/// FullObjectSlot 原子松读 (原版 Relaxed_Load: AsAtomicPointer::Relaxed_Load)
pub fn full_object_slot_relaxed_load(/* slot */) /* -> Tagged<Object> */ {
  todo!("就地修订: 原子松加载后包 Tagged")
}

/// FullObjectSlot 原子释放存 (原版 Release_Store)
pub fn full_object_slot_release_store(/* slot, value */) {
  todo!("就地修订: AsAtomicPointer::Release_Store(location(), value.ptr())")
}

/// FullObjectSlot 比较交换 (原版 Relaxed_/Release_CompareAndSwap: 返回实际值)
pub fn full_object_slot_compare_and_swap(/* slot, old, target, ordering */) /* -> Tagged<Object> */ {
  todo!("就地修订: AsAtomicPointer 双序 CAS, 结果包 Tagged")
}

/// map 值比较 (原版 contains_map_value: load_map().ptr() 裸值比; Relaxed 版原子读)
pub fn full_object_slot_contains_map_value(/* slot, raw_value, relaxed: bool */) -> bool {
  todo!("就地修订: relaxed ? AsAtomicPointer::Relaxed_Load(location()) : load_map().ptr()")
}

/// map 槽读写 (原版 store_map/load_map: V8_MAP_PACKING 时走 Pack/Unpack)
pub fn full_object_slot_map_access(/* slot, map 或读取 */) {
  todo!("就地修订: 基线无 MAP_PACKING, 直接 store/load + UncheckedCast<Map>")
}

/// FullMaybeObjectSlot (kCanBeWeak=true; 读/写/CAS 同上, 值类型 Tagged<MaybeObject>)
pub fn full_maybe_object_slot_family() {
  todo!("就地修订: ${inlDefs.FullMaybeObjectSlot} 个定义 1:1 (operator*/load/store ×2/Relaxed_Load ×2/Relaxed_Load_Raw/RawToTagged/Relaxed_Store/Release_CAS)")
}

/// FullHeapObjectSlot (Tagged<HeapObjectReference>; ToHeapObject 断言强标记位)
pub fn full_heap_object_slot_to_heap_object(/* slot */) /* -> Tagged<HeapObject> */ {
  todo!("就地修订: DCHECK(HAS_STRONG_HEAP_OBJECT_TAG) 后 Cast<HeapObject>")
}

// ===== 外部指针槽 (沙箱表句柄 / 裸指针双形态) =====
/// 外指槽句柄判定 (原版 HasExternalPointerHandle: 沙箱开或 ArrayBufferExtension/WaiterQueueNode tag)
pub fn external_pointer_slot_has_handle(/* slot: &ExternalPointerSlot */) -> bool {
  todo!("就地修订: 三条件 (SANDBOX_BOOL / kArrayBufferExtensionTag / kWaiterQueueNodeTag)")
}

/// 外指槽读 (沙箱开: EPT.Get(handle, tag_range); 关: 非对齐裸读)
pub fn external_pointer_slot_load(/* slot, isolate: IsolateForSandbox */) /* -> Address */ {
  todo!("就地修订: 按沙箱开关分支")
}

/// 外指槽初始化 (原版 init: 沙箱开 AllocateAndInitializeEntry + Release_StoreHandle 建立先表后柄顺序)
pub fn external_pointer_slot_init(/* slot, isolate, host, value, tag */) {
  todo!("就地修订: 表项先行, Release_Store 句柄防重排")
}

/// 外指槽序列化面 (GetAndClear/Restore/ReplaceWithIndex/GetAsIndex, 原版 4 法)
pub fn external_pointer_slot_serialization_family() {
  todo!("就地修订: 读出内容后置空句柄/裸值; 恢复逆操作; 只读序列化器就地换 index")
}

// ===== CppHeap 指针槽 =====
/// CppHeap 槽面 (init/句柄三法/裸读写/序列化二法, 非压缩走裸原子指针)
pub fn cpp_heap_pointer_slot_family() {
  todo!("就地修订: ${inlDefs.CppHeapPointerSlot} 个定义; 压缩开走 CppHeapPointerTable 句柄")
}

// ===== 间接指针槽 (沙箱受信对象引用) =====
/// 间接指针槽加载 (原版 load: Relaxed_LoadHandle → ResolveHandle; 空句柄 → Smi::zero())
pub fn indirect_pointer_slot_load(/* slot, isolate: IsolateForSandbox */) /* -> Tagged<Object> */ {
  todo!("就地修订: !handle → Smi::zero(); 否则按 tag_range 选表解析")
}

/// 句柄解析严格度 (原版 enum TagCheckStrictness: 精确匹配 / 允许未发布项)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TagCheckStrictness {
  /// kRequireExactMatch
  RequireExactMatch,
  /// kAllowUnpublishedEntries (发布窗口期)
  AllowUnpublishedEntries,
}

/// 句柄解析 (原版 ResolveHandle: kCodeTag 走 code 指针表, 其余走 trusted 指针表)
pub fn indirect_pointer_slot_resolve_handle(/* slot, handle, isolate, strictness */) /* -> Tagged<Object> */ {
  todo!("就地修订: 按 tag_range.first 选表; AllowUnpublished 走 GetMaybeUnpublished")
}

/// 受信对象存入 (原版 Relaxed_/Release_Store: 读 value 的 self_indirect_pointer_ 句柄)
pub fn indirect_pointer_slot_store(/* slot, value: Tagged<ExposedTrustedObject>, ordering */) {
  todo!("就地修订: handle = value->ReadField(offsetof(self_indirect_pointer_)); 非沙箱 UNREACHABLE")
}

// ===== 迭代器适配 =====
/// UnalignedSlot<T> (对齐 1 的随机访问迭代器, Reference 代理非对齐读写; 压缩期 [u]intptr/double 字段专用)
pub struct UnalignedSlot<T> { _ty: core::marker::PhantomData<T> }

/// AtomicSlot (Tagged_t 元素原子读写适配器, std::sort 等 STL 入口用; Reference 全走 AsAtomicTagged)
pub struct AtomicSlot { pub address: usize }

impl AtomicSlot {
  /// 解引用 (原版 operator*: 包 Reference, 读走 Relaxed_Load)
  pub fn reference_load(&self) -> u32 {
    todo!("就地修订: AsAtomicTagged::Relaxed_Load; 非压缩 Tagged_t=usize")
  }
  /// 下标访问 (原版 operator[]: address + i * kTaggedSize)
  pub fn index(&self, i: usize) -> usize {
    todo!("就地修订: self.address + i * K_TAGGED_SIZE")
  }
}

// ===== 自由函数 (slots-inl.h 尾部 4 族) =====
/// CopyTagged: 源到目标拷贝 tagged 字 (区间不重叠, 双侧 kTaggedSize 对齐)
pub fn copy_tagged(/* dst: Address, src: Address, num_tagged: u32 */) {
  todo!("就地修订: 逐字拷贝")
}

/// MemsetTagged (裸指针版 + SlotBase 模板版): 填 Tagged<MaybeObject>
pub fn memset_tagged(/* start, value, count */) {
  todo!("就地修订: 压缩开先 CompressAny(value.ptr()) 再 fill_n")
}

/// Relaxed_MemsetTagged: 松弛原子填充
pub fn relaxed_memset_tagged(/* start, value, count */) {
  todo!("就地修订: 同上但走 Relaxed_Memset")
}

/// MemsetPointer: FullObjectSlot 填 Tagged<Object>
pub fn memset_pointer(/* start: FullObjectSlot, value, count */) {
  todo!("就地修订: fill_n(start.location(), count, value.ptr())")
}

// ===== 写保护槽 =====
/// WriteProtectedSlot<SlotT> (JIT 写授权下的槽包装; Relaxed_Store 必须走 WritableJitAllocation)
pub struct WriteProtectedSlot { pub address: usize }

impl WriteProtectedSlot {
  /// 松弛读 (原版直接转发宿主槽)
  pub fn relaxed_load(&self) -> usize {
    todo!("就地修订: 转发 SlotT::Relaxed_Load")
  }
  /// 松弛写 (原版 jit_allocation_.WriteHeaderSlot(address, value, kRelaxedStore))
  pub fn relaxed_store(&self, _value: usize) {
    todo!("就地修订: 经 WritableJitAllocation 写授权路径")
  }
}

/// slots-inl.h 定义对数 (按类: ${Object.entries(inlDefs).map(([k, v]) => `${k} ${v}`).join(", ")})
pub const SLOT_INL_DEF_COUNTS: [(&str, usize); ${Object.keys(inlDefs).length}] = [
${Object.entries(inlDefs).map(([k, v]) => `  ("${k}", ${v}),`).join("\n")}
];
`;

const p = writeSrc("v8-objects-core", "slots.rs", body);
console.log(`slots.h.js: 已生成 ${p} (must ${checks}: 类 ${classes.length}, FullObjectSlot 方法 ${foMethods.length}, inl 定义 ${totalDefs})`);
