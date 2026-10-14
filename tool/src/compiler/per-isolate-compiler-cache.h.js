#!/usr/bin/env bun
// tool/src/compiler/per-isolate-compiler-cache.h.js
// v8/src/compiler/per-isolate-compiler-cache.h
// → v8-compiler-passes/src/per_isolate_compiler_cache.rs.skel
// isolate 级编译缓存: 解析 snapshot 生命周期面与 Setup 惰性建域。
import { readV8, writeSrc, banner } from "../../lib.js";

const h = readV8("src/compiler/per-isolate-compiler-cache.h");

const need = (cond, msg) => {
  if (!cond) throw new Error(`per-isolate-compiler-cache: ${msg}`);
};

for (const [s, name] of [
  ["class PerIsolateCompilerCache : public ZoneObject {", "类定义"],
  ["bool HasSnapshot() const { return refs_snapshot_ != nullptr; }", "HasSnapshot"],
  ["refs_snapshot_ = zone_->New<RefsMap>(refs, zone_);", "SetSnapshot 拷贝构造"],
  ["static void Setup(Isolate* isolate) {", "Setup"],
  ['new Zone(isolate->allocator(), "Compiler zone")', "Compiler zone 名"],
]) need(h.includes(s), `哨兵缺失 ${name}`);

// 字段清单
const fields = [...h.matchAll(/^\s{2}(Zone\* const|RefsMap\*) (zone_|refs_snapshot_);/gm)];
need(fields.length === 2, `字段数 ${fields.length}`);

const body = `${banner("src/compiler/per-isolate-compiler-cache.h.js", "src/compiler/per-isolate-compiler-cache.h", "跨编译运行持久的 isolate 级 refs snapshot 容器")}
//! per-isolate-compiler-cache — 常见对象序列化数据跨编译运行复用
//!
//! 仅存 RefsMap snapshot (JSHeapBroker::InitializeRefsMap 消费);
//! SetSnapshot 用 RefsMap(RefsMap*, Zone*) 拷贝构造 (廉价快照语义)。

/// 原版 PerIsolateCompilerCache — isolate 级缓存 (挂 isolate->compiler_cache)
pub struct PerIsolateCompilerCache<'z> {
  /// zone_ ("Compiler zone")
  pub zone: &'z Zone<'z>,
  /// refs_snapshot_
  pub refs_snapshot: Option<RefsMap<'z>>,
}

impl PerIsolateCompilerCache<'_> {
  /// 原版 HasSnapshot
  /// v8: src/compiler/per-isolate-compiler-cache.h PerIsolateCompilerCache::HasSnapshot
  pub fn has_snapshot(&self) -> bool {
    self.refs_snapshot.is_some()
  }

  /// 原版 GetSnapshot — 取 snapshot (需 HasSnapshot)
  /// v8: src/compiler/per-isolate-compiler-cache.h PerIsolateCompilerCache::GetSnapshot
  pub fn get_snapshot(&mut self) -> &RefsMap<'_> {
    self.refs_snapshot.as_ref().expect("HasSnapshot")
  }

  /// 原版 SetSnapshot — 拷贝构造存入 (非空且此前无 snapshot)
  /// v8: src/compiler/per-isolate-compiler-cache.h PerIsolateCompilerCache::SetSnapshot
  pub fn set_snapshot(&mut self, refs: &RefsMap<'_>) {
    debug_assert!(self.refs_snapshot.is_none());
    self.refs_snapshot = Some(refs.clone_snapshot(self.zone));
  }

  /// 原版 Setup — isolate 上惰性建 "Compiler zone" + 缓存实例
  /// v8: src/compiler/per-isolate-compiler-cache.h PerIsolateCompilerCache::Setup
  pub fn setup(isolate: &mut Isolate) {
    todo!("v8: src/compiler/per-isolate-compiler-cache.h PerIsolateCompilerCache::Setup")
  }
}
`;

const p = writeSrc("v8-compiler-passes", "per_isolate_compiler_cache.rs.skel", body);
console.log(`written ${p}\nfields=${fields.length} (zone_, refs_snapshot_)`);
