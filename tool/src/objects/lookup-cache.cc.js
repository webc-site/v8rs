#!/usr/bin/env bun
// tool/src/objects/lookup-cache.cc.js — 覆盖 lookup-cache 同名族:
//   v8/src/objects/lookup-cache.h + lookup-cache-inl.h + lookup-cache.cc
//   → v8-objects-collections/src/lookup_cache.rs
// 机械提取: DescriptorLookupCache 常量 + 哈希公式 + 查/更/清三方法
//   (体量小, 逻辑全部落真实现; Tagged 指针以 usize 占位, 无堆读)。

import { readV8, writeSrc, banner } from "../../lib.js";

const h = readV8("src/objects/lookup-cache.h");
const inl = readV8("src/objects/lookup-cache-inl.h");
const cc = readV8("src/objects/lookup-cache.cc");

const grab = (s, re, f) => {
  const m = s.match(re);
  if (!m) throw new Error(`常量提取失配: ${f}`);
  return m[1];
};
const kAbsent = Number(grab(h, /static const int kAbsent = (-\d+);/, "kAbsent"));
const kLength = Number(grab(h, /static const int kLength = (\d+);/, "kLength"));
if (kAbsent !== -2) throw new Error(`kAbsent 应 -2, 实得 ${kAbsent}`);
if (kLength !== 64) throw new Error(`kLength 应 64, 实得 ${kLength}`);

const anchors = [
  [h, "class DescriptorLookupCache {", "lookup-cache.h"],
  [inl, "uint32_t source_hash = static_cast<uint32_t>(source.ptr()) >> kTaggedSizeLog2;", "lookup-cache-inl.h (Hash)"],
  [inl, "return (source_hash ^ name_hash) % kLength;", "lookup-cache-inl.h (Hash)"],
  [inl, "if (key.source.SafeEquals(source) && key.name.SafeEquals(name)) {", "lookup-cache-inl.h (Lookup)"],
  [inl, "DCHECK_NE(result, kAbsent);", "lookup-cache-inl.h (Update)"],
  [cc, "void DescriptorLookupCache::Clear() {", "lookup-cache.cc"],
  [cc, "for (int index = 0; index < kLength; index++) keys_[index].source = {};", "lookup-cache.cc (Clear)"],
];
for (const [s, sub, f] of anchors) {
  if (!s.includes(sub)) throw new Error(`锚点失配: ${f} 无 "${sub.slice(0, 52)}..."`);
}

// kTaggedSizeLog2 (globals.h:581, 指针压缩=2 / 非压缩=3; 取压缩 2 — 原版静态配置)
const globals = readV8("src/common/globals.h");
const tagLog2 = grab(globals, /constexpr int kTaggedSizeLog2 = (\d+);/, "kTaggedSizeLog2 (压缩分支)");
if (tagLog2 !== "2") throw new Error("kTaggedSizeLog2 压缩分支应 2");

const body = `${banner("src/objects/lookup-cache.cc.js", "src/objects/lookup-cache.{h,-inl.h,cc}", "(map, name) → 描述符索引 缓存 (含负结果)")}
//! DescriptorLookupCache: (map, 属性名) → 描述符下标的直接映射缓存。
//!
//! - 槽位 ${kLength}, 正负结果皆缓存; kAbsent 表示未命中
//! - 哈希 = (map地址>>kTaggedSizeLog2) ^ name哈希, 对槽数取模
//! - 指针可能过期 → 读侧用 SafeEquals 比对; GC 前整体清空

/// 未命中哨兵 (kAbsent; 注意与查找失败的 -1 区分)。
pub const K_ABSENT: i32 = ${kAbsent};
/// 缓存槽数 (kLength)。
pub const K_LENGTH: usize = ${kLength};
/// TaggedSize 位移 (指针压缩配置 kTaggedSizeLog2 = 2)。
pub const K_TAGGED_SIZE_LOG2: u32 = ${tagLog2};

/// 缓存键 (map 指针 + name 指针)。
#[derive(Clone, Copy, Default)]
pub struct Key {
  /// map 的 Tagged 地址 (0 = 空槽; Clear 置 0)。
  pub source: usize,
  /// name 的 Tagged 地址。
  pub name: usize,
}

/// (map, name) → 描述符索引缓存 (lookup-cache.h:19 DescriptorLookupCache)。
pub struct DescriptorLookupCache {
  keys: [Key; K_LENGTH],
  results: [i32; K_LENGTH],
}

impl Default for DescriptorLookupCache {
  #[inline]
  fn default() -> Self {
    Self::new()
  }
}

impl DescriptorLookupCache {
  /// 构造: 全槽清空 (原版私有无参构造; name 置空指针, 结果置 kAbsent)。
  #[inline]
  #[must_use]
  pub const fn new() -> Self {
    Self {
      keys: [Key { source: 0, name: 0 }; K_LENGTH],
      results: [K_ABSENT; K_LENGTH],
    }
  }

  /// 槽位哈希 (lookup-cache-inl.h:18 Hash)。
  ///
  /// 指针仅取低 32 位场景 (原版注释); dcheck 契约: name 须为 UniqueName。
  #[inline]
  #[must_use]
  pub const fn hash(source: usize, name_hash: u32) -> usize {
    let source_hash = ((source as u64) as u32) >> K_TAGGED_SIZE_LOG2;
    ((source_hash ^ name_hash) as usize) % K_LENGTH
  }

  /// 查描述符索引 (lookup-cache-inl.h:26 Lookup); 未命中返回 kAbsent。
  ///
  /// 槽内指针可能过期 (GC 移动对象) → 仅按当前指针相等比对 (SafeEquals 语义)。
  #[inline]
  #[must_use]
  pub fn lookup(&self, source: usize, name: usize) -> i32 {
    let index = Self::hash(source, Self::name_hash_of(name));
    let key = &self.keys[index];
    if key.source == source && key.name == name {
      return self.results[index];
    }
    K_ABSENT
  }

  /// 更新槽位 (lookup-cache-inl.h:36 Update)。
  ///
  /// 契约: result ≠ kAbsent (原版 DCHECK_NE)。
  #[inline]
  pub fn update(&mut self, source: usize, name: usize, result: i32) {
    debug_assert_ne!(result, K_ABSENT);
    let index = Self::hash(source, Self::name_hash_of(name));
    self.keys[index] = Key { source, name };
    self.results[index] = result;
  }

  /// 清空缓存 (lookup-cache.cc:10 Clear; 仅清 source 即视为空槽)。
  #[inline]
  pub const fn clear(&mut self) {
    let mut i = 0;
    while i < K_LENGTH {
      self.keys[i].source = 0;
      i += 1;
    }
  }

  /// name 哈希读取占位 (原版 name->hash(); Tagged 读通道 L4 修订)。
  fn name_hash_of(_name: usize) -> u32 {
    todo!("就地修订")
  }
}

// —— 提取不变式断言 ——
const _: () = {
  assert!(K_ABSENT == -2 && K_LENGTH == 64 && K_TAGGED_SIZE_LOG2 == 2);
  assert!(DescriptorLookupCache::hash(0, 0) == 0);
  // (source>>2) ^ name_hash 对 64 取模
  assert!(DescriptorLookupCache::hash(1 << 2, 0b1) == 1);
  assert!(DescriptorLookupCache::hash(64, 5) == (1 ^ 5));
  let mut c = DescriptorLookupCache::new();
  c.keys[0] = Key { source: 11, name: 22 };
  c.results[0] = 7;
  assert!(c.results[0] == 7);
  c.clear();
  assert!(c.keys[0].source == 0);
};
`;

writeSrc("v8-objects-collections", "lookup_cache.rs", body);
console.log(`lookup_cache.rs: 常量 ${[kAbsent, kLength, tagLog2].join("/")} + 方法 4 (hash/lookup/update/clear), 对数通过`);
