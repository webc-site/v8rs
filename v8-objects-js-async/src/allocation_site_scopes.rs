// 由 tool/src/objects/allocation-site-scopes.h.js 从 v8/src/objects/allocation-site-scopes.{h,-inl.h} 生成; 修订就地改, 复跑覆盖
// 分配站点作用域: 嵌套字面量复制的 site 游标域骨架
//! AllocationSiteContext / AllocationSiteUsageContext: 遍历并复制嵌套
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

// ── 类面骨架 (深水推迟台账, 依 readme/zh.md) ──
/// v8: src/objects/allocation-site-scopes-inl.h AllocationSiteContext::InitializeTraversal。
pub fn allocation_site_context_initialize_traversal() {
  todo!(
    "AllocationSiteContext::InitializeTraversal: 依赖 Isolate + Handle<AllocationSite> top/current 双游标; 解锁 v8-handles + v8-execution-isolate + v8-objects-maps"
  )
}

/// v8: src/objects/allocation-site-scopes-inl.h AllocationSiteUsageContext::EnterNewScope。
pub fn allocation_site_usage_context_enter_new_scope() {
  todo!(
    "AllocationSiteUsageContext::EnterNewScope: 依赖 Handle<AllocationSite> + nested_site 推进; 解锁 v8-handles + v8-objects-maps"
  )
}

/// v8: src/objects/allocation-site-scopes-inl.h AllocationSiteUsageContext::ExitScope。
pub fn allocation_site_usage_context_exit_scope() {
  todo!(
    "AllocationSiteUsageContext::ExitScope: 依赖 boilerplate DCHECK + Handle 弹出; 解锁 v8-handles + v8-objects-maps"
  )
}

/// v8: src/objects/allocation-site-scopes-inl.h AllocationSiteUsageContext::ShouldCreateMemento。
pub fn allocation_site_usage_context_should_create_memento() {
  todo!(
    "AllocationSiteUsageContext::ShouldCreateMemento: 依赖 v8_flags.pretenuring_threshold + CanTrack(instance_type) + ShouldTrack(elements_kind); 解锁 v8-flags + v8-objects-maps"
  )
}

// ── 提取不变式断言 ──
const _: () = {
  assert!(ALLOCATION_SITE_CONTEXT_MEMBERS.len() == 3);
  assert!(ALLOCATION_SITE_USAGE_CONTEXT_MEMBERS.len() == 2);
  assert!(K_COPYING);
};
