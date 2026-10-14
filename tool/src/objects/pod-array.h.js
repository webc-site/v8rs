#!/usr/bin/env bun
// tool/src/objects/pod-array.h.js — 覆盖 v8/src/objects/pod-array.{h,-inl.h} (全新族)
// → v8-objects-arrays/src/pod_array.rs
// 机械提取: PodArrayBase 6 方法语义 + length 换算 + PodArray/TrustedPodArray New 签名
//   + 字节长溢出检查规则。
// 配置登记: 64 位非压缩 (kTaggedSize=8)。

import { readV8, writeSrc, banner } from "../../lib.js";

const h = readV8("src/objects/pod-array.h");
const inl = readV8("src/objects/pod-array-inl.h");

let checks = 0;
const must = (c, m) => {
  checks++;
  if (!c) throw new Error(`锚点失配: ${m}`);
};

// ── 1. h: 类层次 ───────────────────────────────────────────────────────
must(h.includes("template <class T, class Super>\nclass PodArrayBase : public Super {"),
  "PodArrayBase 模板基类锚缺失");
must(h.includes("class PodArray : public PodArrayBase<T, ByteArray> {"), "PodArray 继承锚缺失");
must(h.includes("class TrustedPodArray : public PodArrayBase<T, TrustedByteArray> {"),
  "TrustedPodArray 继承锚缺失");
must(h.includes("V8_IT_REUSE_PARENT;"), "V8_IT_REUSE_PARENT 锚缺失");

// 方法清单与语义锚 (含 length 声明共 7 项)
const baseMethods = [...h.matchAll(/^  (\w[\w<>,&* ]*) (\w+)\(([^)]*)\)/gm)]
  .map((m) => ({ ret: m[1].trim(), name: m[2], params: m[3].replace(/\s+/g, " ").trim() }))
  .filter((m) => m.name !== "New");
must(JSON.stringify(baseMethods.map((m) => m.name)) ===
  JSON.stringify(["copy_out", "copy_in", "matches", "matches", "get", "set", "length"]),
  `PodArrayBase 方法应 7 (copy_out/copy_in/matches×2/get/set/length), 实得 ${JSON.stringify(baseMethods.map((m) => m.name))}`);
must(h.includes("MemCopy(result, &this->values()[index * sizeof(T)], length * sizeof(T));"),
  "copy_out 语义锚缺失");
must(h.includes("MemCopy(&this->values()[index * sizeof(T)], buffer, length * sizeof(T));"),
  "copy_in 语义锚缺失");
must(h.includes("return memcmp(this->begin(), buffer, length * sizeof(T)) == 0;"),
  "matches(offset=0) 语义锚缺失");
must(h.includes("return memcmp(this->begin() + sizeof(T) * offset, buffer,"),
  "matches(offset) 语义锚缺失");
must(h.includes("DCHECK_LT(index, this->length().value());"), "下标 DCHECK 锚缺失");
must(h.includes("inline SafeHeapObjectSize length() const;"), "length 声明锚缺失");

// New 重载 (Isolate kYoung 默认 / LocalIsolate kOld); 参数跨行允许
const newSigs = [...h.matchAll(/static (?:Direct)?Handle<(\w+<T>)> New\(\s*(\w+)\* isolate,\s*uint32_t length(?:,\s*([^)]*?))?\)/gm)]
  .map((m) => [m[1], m[2], (m[3] ?? "").replace(/\s+/g, " ").trim()]);
must(newSigs.length === 4, `New 重载应 4, 实得 ${newSigs.length}: ${JSON.stringify(newSigs)}`);
must(h.includes("AllocationType allocation = AllocationType::kYoung"), "kYoung 默认锚缺失");
must(h.includes("AllocationType allocation = AllocationType::kOld"), "kOld 默认锚缺失");

// ── 2. -inl: length 换算 + New 实现 ────────────────────────────────────
must(inl.includes("return SafeHeapObjectSize(Super::length().value() / sizeof(T));"),
  "length 换算锚缺失");
must(inl.includes("checked_byte_length *= sizeof(T);"), "字节长乘法锚缺失");
must(inl.includes("CHECK(checked_byte_length.AssignIfValid(&byte_length));"), "溢出检查锚缺失");
must(inl.includes("isolate->factory()->NewByteArray(byte_length, allocation)"), "NewByteArray 锚缺失");
must(inl.includes("isolate->factory()->NewTrustedByteArray(byte_length)"), "NewTrustedByteArray 锚缺失");
const inlDefs = [...inl.matchAll(/^[\w:<>& ]*(PodArrayBase<T, Super>|PodArray<T>|TrustedPodArray<T>)::(\w+)\(/gm)]
  .map((m) => `${m[1]}::${m[2]}`);
must(inlDefs.length === 5, `-inl 定义应 5 (length + New×4), 实得 ${inlDefs.length}: ${JSON.stringify(inlDefs)}`);

// ── 3. 生成 ────────────────────────────────────────────────────────────
const body = `${banner("src/objects/pod-array.h.js", "src/objects/pod-array.{h,-inl.h}", "PodArray 族: memcpy 语义基座 + 字节长换算")}
//! PodArray: 任意可 memcpy 的 C++ 类型 (POD) 装箱到 ByteArray / TrustedByteArray。
//! Rust 侧以泛型承载: PodArrayBase<T, Super> → 参数化宿主字节数组。

/// 元素数量换算 (原版 PodArrayBase::length = Super 字节长 / sizeof(T))。
#[inline]
#[must_use]
pub const fn pod_length(super_byte_length: usize, elem_size: usize) -> usize {
  super_byte_length / elem_size
}

/// 字节长换算 + u32 溢出检查 (原版 New: CheckedNumeric<Length> *= sizeof(T),
/// AssignIfValid 失败即 CHECK 崩溃 → Rust 侧返回 None 由调用方决定)。
#[inline]
#[must_use]
pub const fn pod_byte_length(length: u32, elem_size: usize) -> Option<u32> {
  let bytes = (length as u64) * (elem_size as u64);
  if bytes > u32::MAX as u64 { None } else { Some(bytes as u32) }
}

/// PodArrayBase 方法清单 (memcpy 族 + length 换算; 语义就地修订进宿主实现)。
/// v8: src/objects/pod-array.h — 下标均 DCHECK 越界后走 values() 偏移:
pub const POD_ARRAY_BASE_METHODS: &[(&str, &str)] = &[
  ("copy_out", "MemCopy(result, &values[index * sizeof(T)], length * sizeof(T))"),
  ("copy_in", "MemCopy(&values[index * sizeof(T)], buffer, length * sizeof(T))"),
  ("matches(0)", "memcmp(begin, buffer, length * sizeof(T)) == 0"),
  ("matches(offset)", "memcmp(begin + sizeof(T) * offset, buffer, length * sizeof(T)) == 0"),
  ("get", "copy_out(index, &result, 1) 单元素读"),
  ("set", "copy_in(index, &value, 1) 单元素写"),
  ("length", "Super::length().value() / sizeof(T) 元素数换算"),
];

/// New 重载清单 (4 形态; 分配类型登记)。
pub const POD_ARRAY_NEW_SIGNATURES: &[(&str, &str, &str)] = &[
  ("PodArray<T>", "Isolate", "AllocationType::kYoung (默认) → factory NewByteArray"),
  ("PodArray<T>", "LocalIsolate", "AllocationType::kOld (默认) → factory NewByteArray"),
  ("TrustedPodArray<T>", "Isolate", "factory NewTrustedByteArray"),
  ("TrustedPodArray<T>", "LocalIsolate", "factory NewTrustedByteArray"),
];

// —— 提取不变式断言 ——
const _: () = {
  assert!(pod_length(100, 4) == 25);
  assert!(pod_length(7, 4) == 1);
  assert!(matches!(pod_byte_length(10, 8), Some(80)));
  assert!(pod_byte_length(u32::MAX, 8).is_none());
  assert!(matches!(pod_byte_length(0, 8), Some(0)));
  assert!(POD_ARRAY_BASE_METHODS.len() == 7);
  assert!(POD_ARRAY_NEW_SIGNATURES.len() == 4);
};
`;

const p = writeSrc("v8-objects-arrays", "pod_array.rs", body);
console.log(
  `pod-array.h.js: ${p} — 基座方法 ${baseMethods.length}、New 重载 ${newSigs.length}、-inl ${inlDefs.length}、断言 ${checks} 项通过`,
);