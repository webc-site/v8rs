#!/usr/bin/env bun
// tool/src/compiler/js-heap-broker-inl.h.js
// 从 v8/src/compiler/js-heap-broker-inl.h
// 提取 FindRootIndex 排除规则与三个嵌套互斥守卫的 V8_INLINE 语义
// 生成 v8-compiler-frontend/src/js_heap_broker_inl.rs.skel
// (js-heap-broker.h/.cc 主族已有 tool/src/compiler/js-heap-broker.h.js, 本脚本只做 -inl 差集)
import { readV8, writeSrc, banner } from "../../lib.js";

const h = readV8("src/compiler/js-heap-broker-inl.h");

// —— 1. 锚点断言 ——
const anchors = [
  ["std::optional<RootIndex> JSHeapBroker::FindRootIndex(HeapObjectRef object) {", "FindRootIndex 定义"],
  ["bool known_in_ro_space =", "RO 空间判定双路径 (静态根 vs HeapLayout)"],
  ["StaticReadOnlyRoot::kLastAllocatedRoot;", "静态根上界比较"],
  ["HeapLayout::InReadOnlySpace(raw_obj);", "动态 RO 空间判定"],
  ["if (!known_in_ro_space && object.IsJSReceiver()) return {};", "JSReceiver 排除规则"],
  ["if (root_index_map_.Lookup(raw_obj, &root_index)) {", "RootIndexMap 查表"],
  ["V8_INLINE\nJSHeapBroker::RecursiveMutexGuardIfNeeded::RecursiveMutexGuardIfNeeded(", "RecursiveMutexGuard V8_INLINE"],
  ["V8_INLINE JSHeapBroker::MapUpdaterGuardIfNeeded::MapUpdaterGuardIfNeeded(", "MapUpdaterGuard V8_INLINE"],
  ["V8_INLINE JSHeapBroker::BoilerplateMigrationGuardIfNeeded::", "BoilerplateMigrationGuard V8_INLINE"],
  ["(*mutex_depth_address_)++;", "深度计数递增"],
  ["&broker->map_updater_mutex_depth_", "MapUpdater 深度地址"],
  ["&broker->boilerplate_migration_mutex_depth_", "Boilerplate 深度地址"],
];
for (const [lit, name] of anchors) {
  if (!h.includes(lit)) throw new Error(`js-heap-broker-inl.h 锚点缺失: ${name}`);
}

// —— 2. 真解析: 三个守卫的成员初始化次序 ——
const guardSrc = h.slice(h.indexOf("V8_INLINE\nJSHeapBroker::RecursiveMutexGuardIfNeeded"));
const guardMembers = [...guardSrc.matchAll(/(\w+)_\(([^)]+)\)/g)].map((m) => m[1]);
if (guardMembers.length < 3) throw new Error(`守卫成员提取不足: ${guardMembers}`);

// —— 3. 守卫类清单 ——
const guards = ["RecursiveMutexGuardIfNeeded", "MapUpdaterGuardIfNeeded", "BoilerplateMigrationGuardIfNeeded"];

const body = `${banner("src/compiler/js-heap-broker-inl.h.js", "src/compiler/js-heap-broker-inl.h", "broker 内联件: FindRootIndex 排除规则 + 递归互斥守卫")}

impl JSHeapBroker {
    /// 根索引反查 — 两条排除规则:
    /// 1. RO 空间判定: 静态根 (V8_STATIC_ROOTS_BOOL) 用
    ///    IsInMainCageBase() && ptr <= StaticReadOnlyRoot::kLastAllocatedRoot;
    ///    否则 HeapLayout::InReadOnlySpace
    /// 2. 非 RO 且 IsJSReceiver → 无根 (无根常量是 JSReceiver, 无 JSReceiver 在 RO)
    /// 3. root_index_map_ 查表命中才 Some
    /// v8: src/compiler/js-heap-broker-inl.h JSHeapBroker::FindRootIndex
    pub fn find_root_index(&self, object: HeapObjectRef) -> Option<RootIndex>;
}

/// 递归互斥守卫 — depth==0 才真正上锁, 否则只递增计数
/// 成员初始化: ${guardMembers.join(" → ")}; 构造时 (*mutex_depth_address_)++
/// v8: src/compiler/js-heap-broker-inl.h JSHeapBroker::RecursiveMutexGuardIfNeeded
pub struct RecursiveMutexGuardIfNeeded {
    mutex_depth_address: *mut i32,
    initial_mutex_depth: i32,
    /// local_isolate + mutex + depth==0 才 acquire
    mutex_guard: ParkedMutexGuard,
}

/// MapUpdater 访问守卫 — 递归转发到 broker->isolate()->map_updater_access()
/// v8: src/compiler/js-heap-broker-inl.h JSHeapBroker::MapUpdaterGuardIfNeeded
pub struct MapUpdaterGuardIfNeeded { inner: RecursiveMutexGuardIfNeeded }

/// Boilerplate 迁移守卫 — 递归转发到 boilerplate_migration_access()
/// v8: src/compiler/js-heap-broker-inl.h JSHeapBroker::BoilerplateMigrationGuardIfNeeded
pub struct BoilerplateMigrationGuardIfNeeded { inner: RecursiveMutexGuardIfNeeded }

// —— 对数基准 ——
// 守卫类 ${guards.length} / 守卫成员序 ${guardMembers.length} 项 / 锚点 ${anchors.length}
`;

const p = writeSrc("v8-compiler-frontend", "js_heap_broker_inl.rs.skel", body);
console.log(`✓ js-heap-broker-inl: 守卫 ${guards.length} / 成员序 ${guardMembers.length} / 锚点 ${anchors.length}`);
