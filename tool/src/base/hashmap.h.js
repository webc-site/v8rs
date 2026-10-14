#!/usr/bin/env bun
// tool/src/base/hashmap.h.js — 族: src/base/hashmap.h
// 覆盖: TemplateHashMapImpl (开放寻址) + 3 匹配器 + 4 派生类/别名
// → v8-base/src/hashmap.rs.skel

import { readV8, writeSrc, banner } from "../../lib.js";

const src = readV8("src/base/hashmap.h");

// —— 锚点断言 ——
if (!src.includes("class DefaultAllocationPolicy {")) throw new Error("缺分配策略");
if (!src.includes("class TemplateHashMapImpl {")) throw new Error("缺主模板");
if (!src.includes("static const uint32_t kDefaultHashMapCapacity = 8;")) throw new Error("缺默认容量常量");
if (!src.includes("struct HashEqualityThenKeyMatcher {")) throw new Error("缺哈希先比匹配器");
if (!src.includes("struct KeyEqualityMatcher {")) throw new Error("缺键等匹配器");
if (!src.includes("class CustomMatcherTemplateHashMapImpl")) throw new Error("缺自定义匹配器派生");
if (!src.includes("class PointerTemplateHashMapImpl")) throw new Error("缺指针键派生");
if (!src.includes("class TemplateHashMap")) throw new Error("缺 STL 风格包装");
if (!src.includes("using HashMap = PointerTemplateHashMapImpl<DefaultAllocationPolicy>;")) throw new Error("缺 HashMap 别名");

// —— 真解析: 核心算法语义锚 ——
if (!src.includes("hash &= 0x7FFFFFFF;")) throw new Error("缺 Probe 掩码");
if (!src.includes("i = (i + 1) & (capacity() - 1);")) throw new Error("缺线性探测步进");
if (!src.includes("if (occupancy() + occupancy() / 4 >= capacity()) {")) throw new Error("缺 80% 扩容阈值");
if (!src.includes("Initialize(capacity() * 2);")) throw new Error("缺倍增扩容");
if (!src.includes("Entry* r = impl_.map_ + (q->hash() & (capacity() - 1));")) throw new Error("缺 Remove 回填环");
if (!src.includes("memset(impl_.map_, 0, capacity() * sizeof(Entry));")) throw new Error("缺 Clear");

// 方法面对数
const pubMethods = [
  "Lookup(const Key& key, uint32_t hash) const",
  "LookupOrInsert(const Key& key, uint32_t hash)",
  "InsertNew(const Key& key, uint32_t hash)",
  "Value Remove(const Key& key, uint32_t hash)",
  "Value Remove(Entry* entry)",
  "void Clear()",
  "void Invalidate()",
  "occupancy() const",
  "capacity() const",
  "Entry* Start() const",
  "Entry* Next(Entry* entry) const",
];
for (const m of pubMethods) if (!src.includes(m)) throw new Error(`缺方法 ${m}`);
console.log(`hashmap: 类 5 + 别名 2 (CustomMatcherHashMap/HashMap), 公有方法 ${pubMethods.length}, 容量常量 1`);

let rust = banner(
  "src/base/hashmap.h.js",
  "src/base/hashmap.h",
  "自研开放寻址哈希表 (避 STL 调试锁, 信号处理器安全)",
);

rust += `
//! v8: src/base/hashmap.h
//! 动机: STL unordered_map 调试构建用 mutex 池, 异步信号处理器下会死锁

/// v8: src/base/hashmap.h kDefaultHashMapCapacity = 8 (须 2 的幂)
pub const DEFAULT_HASHMAP_CAPACITY: u32 = 8;

/// v8: src/base/hashmap.h TemplateHashMapImpl<Key, Value, MatchFun, AllocationPolicy>
/// 开放寻址 + 线性探测:
///   Probe: hash &= 0x7FFFFFFF; i = hash & (cap-1); 冲突时 i = (i+1) & (cap-1)
///   FillEmptyEntry: placement-new; occupancy + occupancy/4 >= capacity 即 Resize(×2)
///   Remove: 开放寻址回填环 (Wikipedia Open addressing 算法), 避免断链
///   Clear: memset 0 (hash_and_exists_ 的 exists 位清空)
pub struct TemplateHashMapImpl<K, V, M, A> { /* Developer: entries + capacity + occupancy */ }

impl<K, V, M, A> TemplateHashMapImpl<K, V, M, A> {
    /// v8: Lookup — Probe 后 exists() ? entry : null
    pub fn lookup(&self, key: &K, hash: u32) -> Option<&Entry<K, V>>;
    /// v8: LookupOrInsert(key, hash) — 缺省值版
    /// v8: LookupOrInsert(key, hash, value_func) — 闭包造值版
    /// v8: LookupOrInsert(lookup_key, hash, key_func, value_func) — 异构键版
    pub fn lookup_or_insert(&mut self, key: &K, hash: u32) -> &mut Entry<K, V>;
    /// v8: InsertNew — 不查重, 直接填 Probe 命中的空位
    pub fn insert_new(&mut self, key: &K, hash: u32) -> &mut Entry<K, V>;
    /// v8: Remove(key, hash) / Remove(entry) — 回填环删除, 返回旧值
    pub fn remove(&mut self, key: &K, hash: u32) -> Option<V>;
    /// v8: Clear — 全部标记空, occupancy 归 0
    pub fn clear(&mut self);
    /// v8: Invalidate — 释放表并置不可用
    pub fn invalidate(&mut self);
    /// v8: occupancy — 非空条目数
    pub fn occupancy(&self) -> u32;
    /// v8: capacity — 装载因子上界 80%
    pub fn capacity(&self) -> u32;
    /// v8: Start/Next — C 式迭代对; Rust 侧用 iter() 迭代器代替
    pub fn iter(&self) -> impl Iterator<Item = &Entry<K, V>>;
}

/// v8: src/base/hashmap.h DefaultAllocationPolicy — base::Malloc/Free 数组分配
pub struct DefaultAllocationPolicy;

/// v8: src/base/hashmap.h HashEqualityThenKeyMatcher<Key, MatchFun>
/// hash 相等且 match(key1, key2) 才算命中 (先比哈希省贵键比较)
/// v8: KeyEqualityMatcher<Key> — 仅比 key1 == key2 (忽略 hash 形参)
/// v8: CustomMatcherTemplateHashMapImpl<void*,void*> → CustomMatcherHashMap
/// v8: PointerTemplateHashMapImpl<void*,void*> → HashMap (指针键直比)
/// v8: TemplateHashMap<Key,Value> — STL 风格 (value_type{first,second}, find(key, insert=false))
`;

writeSrc("v8-base", "hashmap.rs.skel", rust);
console.log("written v8-base/src/hashmap.rs.skel");
