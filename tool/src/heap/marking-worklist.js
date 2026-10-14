#!/usr/bin/env bun
// tool/src/heap/marking-worklist.js
// 机械提取 v8/src/heap/marking-worklist{.h,.cc,-inl.h} 族:
// MarkingWorklists / ::Local 常量与签名骨架

import { readV8, writeSrc, banner } from "../../lib.js";

const hSrc = readV8("src/heap/marking-worklist.h");
const ccSrc = readV8("src/heap/marking-worklist.cc");
const inlSrc = readV8("src/heap/marking-worklist-inl.h");

const anchors = [
  [hSrc, "struct ContextWorklistPair {"],
  [hSrc, "class V8_EXPORT_PRIVATE MarkingWorklists final {"],
  [hSrc, "static constexpr Address kSharedContext = 0;"],
  [hSrc, "static constexpr Address kOtherContext = 8;"],
  [hSrc, "class V8_EXPORT_PRIVATE MarkingWorklists::Local final {"],
  [hSrc, "static constexpr Address kSharedContext = MarkingWorklists::kSharedContext;"],
  [hSrc, "void CreateContextWorklists(const std::vector<Address>& contexts);"],
  [hSrc, "void ReleaseContextWorklists();"],
  [hSrc, "void ShareWork();"],
  [hSrc, "void MergeOnHold();"],
  [ccSrc, "void MarkingWorklists::Clear() {"],
  [ccSrc, "void MarkingWorklists::CreateContextWorklists("],
  [inlSrc, "void MarkingWorklists::Local::PublishCppHeapObjects() {"],
];
for (const [src, a] of anchors) if (!src.includes(a)) throw new Error(`锚点缺失: ${a}`);

const ccFns = ["MarkingWorklists::Clear", "MarkingWorklists::Print", "MarkingWorklists::IsEmpty", "MarkingWorklists::CreateContextWorklists", "MarkingWorklists::ReleaseContextWorklists", "MarkingWorklists::PrintWorklist", "MarkingWorklists::Local::Local", "MarkingWorklists::Local::Publish", "MarkingWorklists::Local::IsEmpty", "MarkingWorklists::Local::IsWrapperEmpty", "MarkingWorklists::Local::ShareWork", "MarkingWorklists::Local::PublishWork", "MarkingWorklists::Local::MergeOnHold"];
for (const f of ccFns) if (!ccSrc.includes(f)) throw new Error(`cc 缺 ${f}`);
console.log(`marking-worklist: cc 函数 ${ccFns.length}, 假地址常量 2, inl Push/Pop/PushOnHold/PopOnHold/PublishCppHeapObjects`);

let rust = banner(
  "src/heap/marking-worklist.js",
  "src/heap/marking-worklist.h, src/heap/marking-worklist.cc, src/heap/marking-worklist-inl.h",
  "全局标记工作表集合与线程本地视图",
);

rust += `/// v8: src/heap/marking-worklist.h ContextWorklistPair
pub struct ContextWorklistPair {
  pub context: Address,
  pub worklist: Box<MarkingWorklist>,
}

/// v8: src/heap/marking-worklist.h MarkingWorklists
pub struct MarkingWorklists {
  /// v8: default_ — 绝大多数对象
  pub default_: MarkingWorklist,
  /// v8: on_hold_ — 并发标记对新生成对象 bail out (避免黑分配)
  pub on_hold: MarkingWorklist,
  /// v8: other_ — 归属未被测量 context 的对象
  pub other: MarkingWorklist,
  /// v8: context_worklists_
  pub context_worklists: Vec<ContextWorklistPair>,
}

impl MarkingWorklists {
  /// v8: MarkingWorklists::kSharedContext — 未归属任何 context 的假地址
  pub const K_SHARED_CONTEXT: Address = 0;
  /// v8: MarkingWorklists::kOtherContext — 归属未测量 context 的假地址
  pub const K_OTHER_CONTEXT: Address = 8;

  /// v8: MarkingWorklists::CreateContextWorklists — 标记开始时按 contexts 建表
  pub fn create_context_worklists(&mut self, contexts: &[Address]) { todo!() }
  /// v8: MarkingWorklists::ReleaseContextWorklists — 结束时全部须为空
  pub fn release_context_worklists(&mut self) { todo!() }
  /// v8: MarkingWorklists::IsPerContextMode — !context_worklists_.empty()
  pub fn is_per_context_mode(&self) -> bool { !self.context_worklists.is_empty() }
  /// v8: MarkingWorklists::Clear / Print / IsEmpty / PrintWorklist
}

/// v8: src/heap/marking-worklist.h MarkingWorklists::Local
/// 线程本地视图; active_ 直接内嵌 Local 实例避免间接层
/// (不变式: active_owner 为空壳, 内容已移入 active_)。
pub struct MarkingWorklistsLocal {
  /// v8: global / active / active_owner / active_context / cpp_marking_state_
  pub fields_todo: (),
}
impl MarkingWorklistsLocal {
  pub const K_SHARED_CONTEXT: Address = 0;
  pub const K_OTHER_CONTEXT: Address = 8;

  /// v8: Local::Push / Pop (默认表)
  /// v8: Local::PushOnHold / PopOnHold
  /// v8: Local::Publish — 本地段推回全局
  /// v8: Local::IsEmpty / IsWrapperEmpty
  /// v8: Local::ShareWork — 全局空则发布 active (含 shared)
  /// v8: Local::PublishWork — 非 per-context 模式发布
  /// v8: Local::MergeOnHold — on_hold 并回 shared
  /// v8: Local::PublishCppHeapObjects (-inl.h)
  fn todo_iface(&mut self) { todo!() }
}
`;

const p = writeSrc("v8-heap-gc", "marking_worklist.rs.skel", rust);
console.log(`written ${p}`);
