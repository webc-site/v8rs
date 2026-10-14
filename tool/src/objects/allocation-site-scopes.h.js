#!/usr/bin/env bun
// tool/src/objects/allocation-site-scopes.h.js — 覆盖 v8/src/objects/allocation-site-scopes.{h,-inl.h}
// → v8-objects-js-async/src/allocation_site_scopes.rs
// 机械提取: AllocationSiteContext/AllocationSiteUsageContext 类面骨架 (嵌套字面量遍历域)

import { readV8, writeSrc, banner } from "../../lib.js";

const h = readV8("src/objects/allocation-site-scopes.h");
const inl = readV8("src/objects/allocation-site-scopes-inl.h");

let checks = 0;
const must = (c, m) => { checks++; if (!c) throw new Error(`[allocation-site-scopes.h.js] ${m}`); };

// ── 1. AllocationSiteContext (基类: 遍历嵌套 boilerplate 的游标域) ────────
must(h.includes("class AllocationSiteContext {"), "h 缺 AllocationSiteContext");
const ctxMembers = [...h.matchAll(/^  (\w+)\*? (\w+);$/gm)].map((m) => [m[1], m[2]]);
// isolate_/top_/current_ 3 字段 (private 段)
must(h.includes("  Isolate* isolate_;") && h.includes("  Handle<AllocationSite> top_;") && h.includes("  Handle<AllocationSite> current_;"),
  "AllocationSiteContext 成员漂移");
const ctxMethods = [...h.matchAll(/^  (?:explicit |inline |static |bool |void |DirectHandle<\w+> |Isolate\* )*?(\w+)\(/gm)].map((m) => m[1]);
for (const m of ["AllocationSiteContext", "top", "current", "ShouldCreateMemento", "isolate", "update_current_site", "InitializeTraversal"]) {
  must(ctxMethods.includes(m), `AllocationSiteContext 缺方法 ${m}`);
}

// ── 2. AllocationSiteUsageContext (子类: AllocationMemento 制造面) ────────
must(h.includes("class AllocationSiteUsageContext : public AllocationSiteContext {"), "h 缺 AllocationSiteUsageContext");
must(h.includes("static const bool kCopying = true;"), "kCopying != true");
for (const m of ["EnterNewScope", "ExitScope", "ShouldCreateMemento"]) {
  must(new RegExp(`inline \\S+\\s*${m}\\(`).test(h) || h.includes(`${m}(`), `UsageContext 缺 ${m}`);
}
must(h.includes("  Handle<AllocationSite> top_site_;") && h.includes("  bool activated_;"), "UsageContext 成员漂移");

// ── 3. -inl.h 实现面锚 ───────────────────────────────────────────────────
for (const s of [
  "void AllocationSiteContext::InitializeTraversal(Handle<AllocationSite> site) {",
  "Handle<AllocationSite> AllocationSiteUsageContext::EnterNewScope() {",
  "void AllocationSiteUsageContext::ExitScope(",
  "bool AllocationSiteUsageContext::ShouldCreateMemento(",
  "AllocationSite::CanTrack(object->map()->instance_type())",
  "AllocationSite::ShouldTrack(object->GetElementsKind())",
  "v8_flags.allocation_site_pretenuring",
]) must(inl.includes(s), `-inl.h 缺 ${s}`);

let rust = banner("src/objects/allocation-site-scopes.h.js", "src/objects/allocation-site-scopes.{h,-inl.h}",
  "分配站点作用域: 嵌套字面量复制的 site 游标域骨架");
rust += `//! AllocationSiteContext / AllocationSiteUsageContext: 遍历并复制嵌套
//! boilerplate 时的 AllocationSite 游标 + AllocationMemento 制造决策面。

// ── AllocationSiteContext (基类, 3 私有字段: isolate/top/current) ──
pub const ALLOCATION_SITE_CONTEXT_MEMBERS: [(&str, &str); 3] = [
  ("isolate_", "Isolate*"),
  ("top_", "Handle<AllocationSite>"),
  ("current_", "Handle<AllocationSite>"),
];

// ── AllocationSiteUsageContext (子类, 2 私有字段) ──
pub const ALLOCATION_SITE_USAGE_CONTEXT_MEMBERS: [(&str, &str); 2] = [
  ("top_site_", "Handle<AllocationSite>"),
  ("activated_", "bool"),
];

/// 复制模式标记 (原版 kCopying = true)。
pub const K_COPYING: bool = true;

// ── 类面骨架 (语义就地修订) ──
/// 原版 AllocationSiteContext::InitializeTraversal (inl: top/current 双 Handle 起游标)。
pub fn allocation_site_context_initialize_traversal() {
  todo!("就地修订: allocation-site-scopes-inl.h InitializeTraversal")
}

/// 原版 AllocationSiteUsageContext::EnterNewScope (inl: 沿 nested_site 推进 current)。
pub fn allocation_site_usage_context_enter_new_scope() {
  todo!("就地修订: allocation-site-scopes-inl.h EnterNewScope")
}

/// 原版 AllocationSiteUsageContext::ExitScope (inl: DCHECK 对齐 boilerplate)。
pub fn allocation_site_usage_context_exit_scope() {
  todo!("就地修订: allocation-site-scopes-inl.h ExitScope")
}

/// 原版 AllocationSiteUsageContext::ShouldCreateMemento
/// (inl: activated_ && CanTrack(instance_type) && (pretenuring || ShouldTrack(elements_kind)))。
pub fn allocation_site_usage_context_should_create_memento() {
  todo!("就地修订: allocation-site-scopes-inl.h ShouldCreateMemento")
}

// ── 提取不变式断言 ──
const _: () = {
  assert!(ALLOCATION_SITE_CONTEXT_MEMBERS.len() == 3);
  assert!(ALLOCATION_SITE_USAGE_CONTEXT_MEMBERS.len() == 2);
  assert!(K_COPYING);
};
`;

const p = writeSrc("v8-objects-js-async", "allocation_site_scopes.rs", rust);
console.log(`allocation-site-scopes.h.js: ${p} — 2 类 5 字段 + 4 方法骨架 + kCopying, 断言 ${checks} 项全过`);
