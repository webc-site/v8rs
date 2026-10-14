#!/usr/bin/env bun
// tool/src/heap/heap-visitor.h.js
// 机械提取 v8/src/heap/heap-visitor.{h,cc,-inl.h}:
// ① MaybeObjectSize (std::optional<size_t> 平替); ② TYPED_VISITOR_ID_LIST 68 项
// X-macro 展开; ③ TYPED_VISITOR_WITH_SLACK_ID_LIST 13+6(IF_WASM) 项;
// ④ HeapVisitor/ConcurrentHeapVisitor/NewSpaceVisitor 8 个 constexpr 定制开关。
// TRUSTED_VISITOR_ID_LIST 定义于 src/objects/map.h (另席), 此处只登记引用。

import { readV8, writeSrc, banner } from "../../lib.js";

const h = readV8("src/heap/heap-visitor.h");
const cc = readV8("src/heap/heap-visitor.cc");
const inl = readV8("src/heap/heap-visitor-inl.h");

// —— X-macro 列表展开 (反斜杠续行) ——
function extractList(name) {
  const re = new RegExp(`#define ${name}\\(V\\)([\\\\\\s\\S]*?)\\n\\n`);
  const m = h.match(re);
  if (!m) throw new Error(`${name} 未找到`);
  return [...m[1].matchAll(/V\((\w+)\)/g)].map((x) => x[1]);
}
const typed = extractList("TYPED_VISITOR_ID_LIST");
if (typed.length !== 68) throw new Error(`TYPED_VISITOR_ID_LIST ${typed.length}, 期望 68`);
if (typed[0] !== "AccessorInfo" || typed[67] !== "TurboshaftWord64SetType") {
  throw new Error(`typed 列表首尾漂移: ${typed[0]}/${typed[67]}`);
}
// typed 列表另含 11 个 IF_WASM 条目 + 尾部 SIMPLE_HEAP_OBJECT_LIST1(V) (objects/object-list-macros.h, 另席)
const typedWasm = (h.match(/IF_WASM\(V, (Wasm\w+)\)/g) || []).length;
if (typedWasm !== 11 + 6) throw new Error(`IF_WASM 总数 ${typedWasm}, 期望 17`);
if (!h.includes("SIMPLE_HEAP_OBJECT_LIST1(V)")) throw new Error("SIMPLE_HEAP_OBJECT_LIST1(V) 尾项缺失");

const slack = extractList("TYPED_VISITOR_WITH_SLACK_ID_LIST");
if (slack.length !== 13) throw new Error(`SLACK 列表 ${slack.length}, 期望 13`);
// IF_WASM 按 macro 作用域取 (slack 列表区内)
function extractWasm(name) {
  const re = new RegExp(`#define ${name}\\(V\\)([\\\\\\s\\S]*?)\\n\\n`);
  const m = h.match(re);
  if (!m) throw new Error(`${name} 区未找到`);
  return [...m[1].matchAll(/IF_WASM\(V, (\w+)\)/g)].map((x) => x[1]);
}
const slackWasm = extractWasm("TYPED_VISITOR_WITH_SLACK_ID_LIST");
if (slackWasm.length !== 6) throw new Error(`slack IF_WASM ${slackWasm.length}, 期望 6`);

// —— 类与定制开关锚 ——
for (const a of [
  "class MaybeObjectSize final {",
  "class HeapVisitor : public ObjectVisitorWithCageBases {",
  "class ConcurrentHeapVisitor : public HeapVisitor<ConcreteVisitor> {",
  "class NewSpaceVisitor : public ConcurrentHeapVisitor<ConcreteVisitor> {",
  "V8_INLINE static constexpr bool ShouldVisitMapPointer() { return true; }",
  "V8_INLINE static constexpr bool ShouldVisitReadOnlyMapPointer() {",
  "V8_INLINE static constexpr bool CanEncounterFillerOrFreeSpace() {",
  "V8_INLINE static constexpr bool ShouldUseUncheckedCast() { return false; }",
  "V8_INLINE static constexpr bool EnableConcurrentVisitation() { return false; }",
  "V8_INLINE static constexpr bool UsePrecomputedObjectSize() { return false; }",
  "V8_INLINE static constexpr bool ShouldVisitFullJSObject() { return false; }",
  "TYPED_VISITOR_ID_LIST(VISIT)",
  "TRUSTED_VISITOR_ID_LIST(VISIT)",
]) {
  if (!h.includes(a)) throw new Error(`heap-visitor.h 锚点缺失: ${a}`);
}
// NewSpaceVisitor: 不访 map 指针
if (!h.includes("V8_INLINE static constexpr bool ShouldVisitMapPointer() { return false; }")) {
  throw new Error("NewSpaceVisitor ShouldVisitMapPointer=false 缺失");
}
// cc: 弱链访问器 (VisitWeakList) 与显式实例化
for (const a of [
  "WeakListVisitor<T>::VisitLiveObject(heap, tail);",
  "template <>",
  "VisitWeakList<Context>",
  "VisitWeakList<AllocationSiteWithWeakNext>",
  "VisitWeakList<JSFinalizationRegistry>",
]) {
  if (!cc.includes(a)) throw new Error(`heap-visitor.cc 锚点缺失: ${a}`);
}
for (const a of [
  "size_t HeapVisitor<ConcreteVisitor>::Visit(",
  "VisitMapPointerIfNeeded",
  "VisitWithBodyDescriptor",
]) {
  if (!inl.includes(a)) throw new Error(`heap-visitor-inl.h 锚点缺失: ${a}`);
}
console.log(`HeapVisitor: typed ${typed.length}+${typedWasm}WASM + slack ${slack.length}+${slackWasm.length}WASM + 定制开关 8`);

let rust = banner(
  "src/heap/heap-visitor.h.js",
  "src/heap/heap-visitor.{h,cc,-inl.h}",
  "堆访问者: 类型分发 X-macro 展开与定制开关",
);

const snakeType = (s) => s.replace(/([a-z0-9])([A-Z])/g, "$1_$2").toUpperCase();
let visitFns = "";
typed.forEach((t) => {
  visitFns += `  /// 对应 VISIT(${t})\n  fn visit_${snakeType(t)}(&mut self, object: &mut ()) -> usize {\n    unimplemented!("骨架: ${t} BodyDescriptor 待对象族就位");\n  }\n\n`;
});
slack.forEach((t) => {
  visitFns += `  /// 对应 VISIT_WITH_SLACK(${t}) (JSObject slack 需全量访问时 ShouldVisitFullJSObject)\n  fn visit_${snakeType(t)}_with_slack(&mut self, object: &mut ()) -> usize {\n    unimplemented!("骨架: ${t} slack 访问待对象族就位");\n  }\n\n`;
});

rust += `
/// 对象大小可选值 (对应 class MaybeObjectSize final — std::optional<size_t> 平替)。
/// raw_size_ == 0 表 None; AssumeSize 在 debug 下断言非 0。
/// v8: src/heap/heap-visitor.h MaybeObjectSize
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct MaybeObjectSize {
  raw_size: usize,
}

impl MaybeObjectSize {
  /// 对应 explicit MaybeObjectSize(size_t): DCHECK_GT(size, 0)
  pub fn new(size: usize) -> Self {
    debug_assert!(size > 0);
    Self { raw_size: size }
  }

  /// 对应 MaybeObjectSize(): raw_size_(0)
  pub fn none() -> Self {
    Self { raw_size: 0 }
  }

  /// 对应 AssumeSize: DCHECK_GT(raw_size_, 0) 后返回
  pub fn assume_size(self) -> usize {
    debug_assert!(self.raw_size > 0);
    self.raw_size
  }

  /// 对应 IsNone: raw_size_ == 0
  pub fn is_none(self) -> bool {
    self.raw_size == 0
  }
}

/// 访问者定制开关 (对应 HeapVisitor CRTP 的 8 个 constexpr 谓词, 默认值)。
/// v8: src/heap/heap-visitor.h HeapVisitor
pub trait VisitorSwitches {
  const SHOULD_VISIT_MAP_POINTER: bool = true;
  const SHOULD_VISIT_READ_ONLY_MAP_POINTER: bool = true;
  const CAN_ENCOUNTER_FILLER_OR_FREE_SPACE: bool = true;
  const SHOULD_USE_UNCHECKED_CAST: bool = false;
  const ENABLE_CONCURRENT_VISITATION: bool = false;
  const USE_PRECOMPUTED_OBJECT_SIZE: bool = false;
  const SHOULD_VISIT_FULL_JS_OBJECT: bool = false;
}

/// 默认全开 (对应 HeapVisitor 基类默认)。
pub struct DefaultSwitches;
impl VisitorSwitches for DefaultSwitches {}

/// 并发访问者开关 (对应 ConcurrentHeapVisitor: 并发停用)。
pub struct ConcurrentSwitches;
impl VisitorSwitches for ConcurrentSwitches {}

/// 新生代访问者开关 (对应 NewSpaceVisitor: 不访 map 指针)。
pub struct NewSpaceSwitches;
impl VisitorSwitches for NewSpaceSwitches {
  const SHOULD_VISIT_MAP_POINTER: bool = false;
}

/// 类型分发访问者骨架 (对应 HeapVisitor<ConcreteVisitor> + 3 张 VISIT X-macro 表)。
/// v8: src/heap/heap-visitor.h HeapVisitor
pub trait HeapVisitorSkeleton {
${visitFns}  /// 对应 Visit(map, object, size) 主入口: 按 VisitorId 分发到上表
  fn visit(&mut self, _map: (), _object: &mut ()) -> usize {
    unimplemented!("骨架: VisitorId 分发待 instance-type 族就位");
  }
}

// —— X-macro 展开台账: TYPED_VISITOR_ID_LIST (68 直列 V() + 11 IF_WASM +
// 尾部 SIMPLE_HEAP_OBJECT_LIST1 展开; 后两者条件/另席, 此处登记直列部分) ——
pub const TYPED_VISITOR_IDS: &[&str] = &[
${typed.map((t) => `  "${t}",`).join("\n")}
];

/// TYPED_VISITOR_WITH_SLACK_ID_LIST (13 无条件 + 6 IF_WASM)。
pub const TYPED_VISITOR_WITH_SLACK_IDS: &[&str] = &[
${slack.map((t) => `  "${t}",`).join("\n")}
];
pub const TYPED_VISITOR_WITH_SLACK_IDS_WASM: &[&str] = &[
${slackWasm.map((t) => `  "${t}",`).join("\n")}
];
/// TRUSTED_VISITOR_ID_LIST 展开于 src/objects/map.h (CONCRETE_TRUSTED_OBJECT_TYPE_LIST1),
/// TYPED_VISITOR_ID_LIST 尾部的 SIMPLE_HEAP_OBJECT_LIST1 展开于 src/objects/object-list-macros.h,
/// 二者均另席覆盖; IF_WASM 11 条 (typed 列表内) 随 wasm 配置启用。


// —— 提取不变式断言 ——
const _: () = {
  assert_eq!(TYPED_VISITOR_IDS.len(), 68);
  assert_eq!(TYPED_VISITOR_IDS[0], "AccessorInfo");
  assert_eq!(TYPED_VISITOR_IDS[67], "WeakCell");
  assert_eq!(TYPED_VISITOR_WITH_SLACK_IDS.len(), 13);
  assert_eq!(TYPED_VISITOR_WITH_SLACK_IDS_WASM.len(), 6);
  let s = MaybeObjectSize::new(24);
  assert_eq!(s.assume_size(), 24);
  assert!(!s.is_none());
  assert!(MaybeObjectSize::none().is_none());
  assert!(DefaultSwitches::SHOULD_VISIT_MAP_POINTER);
  assert!(!NewSpaceSwitches::SHOULD_VISIT_MAP_POINTER);
};
`;

const p = writeSrc("v8-heap-core", "heap_visitor.rs.skel", rust);
console.log(`written ${p}`);
