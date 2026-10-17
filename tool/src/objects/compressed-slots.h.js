#!/usr/bin/env bun
// tool/src/objects/compressed-slots.h.js — 压缩槽位族生成
// 组→文件映射 (本脚本管辖):
//   v8/src/objects/compressed-slots.h + compressed-slots-inl.h → v8-objects-core/src/compressed_slots.rs
// 规则面: 6 个槽位类的类型面 (TData/kCanBeWeak/方法签名清单) — 全体 #ifdef V8_COMPRESS_POINTERS 内,
//   本基线为非压缩配置, 类不落地仅登记; 方法语义 (压缩/解压经 V8HeapCompressionScheme) 留 doc + 骨架
import { readV8, writeSrc, banner } from "../../lib.js";

const h = readV8("src/objects/compressed-slots.h");
const inl = readV8("src/objects/compressed-slots-inl.h");

if (!/#ifdef V8_COMPRESS_POINTERS/.test(h) || !/#endif  \/\/ V8_COMPRESS_POINTERS/.test(h))
  throw new Error("compressed-slots.h 应整体位于 V8_COMPRESS_POINTERS 守卫内");

// —— 类清单与方法面 (按 class 声明序; 继承可跨行) ——
const classes = [...h.matchAll(/^class (\w+)\s*$|^class (\w+) : public [\w:<>, ]+\{$/gm)].map((m) => m[1] ?? m[2]).filter((n) => n !== "Reference");
if (classes.join() !== "CompressedObjectSlot,CompressedMaybeObjectSlot,CompressedHeapObjectSlot,OffHeapCompressedObjectSlotBase,OffHeapCompressedObjectSlot,OffHeapCompressedMaybeObjectSlot")
  throw new Error(`压缩槽位类清单漂移: ${classes}`);

// 每类方法签名计数 (声明侧: inline 方法)
function methodCount(cls) {
  const start = h.indexOf(`class ${cls}`);
  const next = h.indexOf("\nclass ", start + 1);
  const block = h.slice(start, next < 0 ? undefined : next);
  return [...block.matchAll(/^\s*(?:static )?inline .*?(\w+)\(/gm)].length;
}
const counts = Object.fromEntries(classes.map((c) => [c, methodCount(c)]));
const total = Object.values(counts).reduce((a, b) => a + b, 0);
if (total < 40) throw new Error(`声明方法总数应 ≥40, 实得 ${total}`);
// 锚点: 关键方法在列
for (const need of ["contains_map_value", "Relaxed_ContainsMapValue", "store_map", "load_map", "Relaxed_Load_Raw", "RawToTagged", "Release_CompareAndSwap", "ToHeapObject", "StoreHeapObject"])
  if (!new RegExp(`inline .*${need}\\(`).test(h)) throw new Error(`compressed-slots.h 缺方法 ${need}`);
// kCanBeWeak 常量核对: Object 槽 false / MaybeObject 槽 true / HeapObject 槽无
if (!/class CompressedObjectSlot[\s\S]{0,900}static constexpr bool kCanBeWeak = false;/.test(h)) throw new Error("CompressedObjectSlot kCanBeWeak 应 false");
if (!/class CompressedMaybeObjectSlot[\s\S]{0,900}static constexpr bool kCanBeWeak = true;/.test(h)) throw new Error("CompressedMaybeObjectSlot kCanBeWeak 应 true");
if (!/class OffHeapCompressedMaybeObjectSlot[\s\S]{0,900}static constexpr bool kCanBeWeak = true;/.test(h)) throw new Error("OffHeapCompressedMaybeObjectSlot kCanBeWeak 应 true");

// inl 侧: 压缩/解压调用点对数 (V8HeapCompressionScheme::Compress*/Decompress* 出现次数)
const compressCalls = [...inl.matchAll(/(?:TCompressionScheme|V8HeapCompressionScheme)::(Compress\w+|Decompress\w+)/g)].map((m) => m[1]);
const compressCount = compressCalls.filter((c) => c.startsWith("Compress")).length;
const decompressCount = compressCalls.length - compressCount;
if (compressCount < 10 || decompressCount < 15)
  throw new Error(`压缩/解压调用应 ≥10/≥15, 实得 ${compressCount}/${decompressCount}`);
// inl 定义数对账 (声明 vs 定义近似 1:1)
const defCount = [...inl.matchAll(/^(?:\w+<[^>]+>[\s\S]*?::|\w+)::\w+::/gm)].length;

const body = `${banner("src/objects/compressed-slots.h.js", "src/objects/compressed-slots.h + compressed-slots-inl.h", "非压缩基线: 类不落地, 登记类型面与压缩语义")}
//! 压缩槽位族 (原版 compressed-slots.{h,-inl.h})
//!
//! 原版 ${classes.length} 个类全部位于 \`#ifdef V8_COMPRESS_POINTERS\` 内,
//! \`TData = Tagged_t = uint32\` (压缩配置). 本基线为非压缩配置:
//! 槽位运行时由 slots.rs 的 Full* 族承载, 本模块登记压缩族类型面,
//! 供指针压缩开关 (V8_COMPRESS_POINTERS) 落地时 1:1 对齐.

/// 压缩槽位类清单 (声明顺序, 原版 ${classes.length} 类 / 方法声明 ${total} 个)
pub const COMPRESSED_SLOT_CLASSES: [(&str, &str, bool, usize); ${classes.length}] = [
  // (类名, 槽值语义, kCanBeWeak, 声明方法数)
${classes.map((c) => {
    const weak = /MaybeObject/.test(c) ? "true" : "false";
    const sem = c.includes("HeapObjectSlot") && !c.includes("Maybe") ? "Tagged<HeapObjectReference>" : c.includes("Maybe") ? "Tagged<MaybeObject>" : "Tagged<Object>";
    return `  ("${c}", "${sem}", ${weak}, ${counts[c]}),`;
  }).join("\n")}
];

/// 压缩方案的调用面 (compressed-slots-inl.h 机械计数:
/// Compress* ${compressCount} 处 / Decompress* ${decompressCount} 处)
pub const COMPRESSION_SCHEME_CALLS: (usize, usize) = (${compressCount}, ${decompressCount});

/// 压缩槽位读 (原版 load: 原子读 Tagged_t → DecompressTagged)
pub fn compressed_load(/* slot: &Self, cage_base: PtrComprCageBase */) /* -> Tagged<TObject> */ {
  todo!("就地修订: AsAtomicTagged 读 + V8HeapCompressionScheme::DecompressTagged")
}

/// 压缩槽位写 (原版 store: CompressObject → 裸写)
pub fn compressed_store(/* slot: &Self, value: Tagged<TObject> */) {
  todo!("就地修订: CompressObject + *location() 赋值")
}

/// 压缩槽位原子松读 (原版 Relaxed_Load)
pub fn compressed_relaxed_load(/* slot: &Self */) /* -> Tagged<TObject> */ {
  todo!("就地修订: AsAtomicTagged::Relaxed_Load + DecompressTagged")
}

/// 压缩槽位原子释放写 (原版 Release_Store)
pub fn compressed_release_store(/* slot: &Self, value: Tagged<TObject> */) {
  todo!("就地修订: CompressObject + AsAtomicTagged::Release_Store")
}

/// 压缩槽位释放比较交换 (原版 Release_CompareAndSwap: 双侧压缩, 结果解压)
pub fn compressed_release_compare_and_swap(/* slot, old, target */) /* -> Tagged<TObject> */ {
  todo!("就地修订: 双侧 CompressObject, AsAtomicTagged::Release_CompareAndSwap, 结果 DecompressTagged")
}

/// 压缩 map 值比较 (原版 contains_map_value: 不解压直接比裸值)
pub fn compressed_contains_map_value(/* slot, raw_value */) -> bool {
  todo!("就地修订: load_map().ptr() 裸值比较, 不经解压")
}
`;

const p = writeSrc("v8-objects-core", "compressed_slots.rs", body);
console.log(`compressed-slots.h.js: 已生成 ${p} (类 ${classes.length}, 方法声明 ${total}, Compress ${compressCount}/Decompress ${decompressCount}, inl 定义 ${defCount})`);
