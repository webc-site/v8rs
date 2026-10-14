// MinorMarkSweepCollector 语义面 (纯规则子集)。
//
// 族: v8/src/heap/minor-mark-sweep.{h,cc,inl.h}
// 分工: 本模块挂载 MarkingItem::SlotsType 与 kMaxParallelTasks 常量。
//       YoungGenerationRememberedSetsMarkingWorklist / MinorMarkSweepCollector
//       CollectGarbage / StartMarking / RequestGC / TearDown /
//       YoungGenerationRootMarkingVisitor (依赖 MutablePage/SlotSet/TypedSlotSet/
//       Heap/EphemeronTable/Sweeper/IndexGenerator) 已登记 readme 台账, 待 heap-core
//       冻结后启动。

/// v8: src/heap/minor-mark-sweep.h MarkingItem::SlotsType
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum SlotsType {
  /// kRegularSlots
  RegularSlots = 0,
  /// kTypedSlots
  TypedSlots = 1,
}

/// v8: src/heap/minor-mark-sweep.h MinorMarkSweepCollector::kMaxParallelTasks
pub const K_MAX_PARALLEL_TASKS: usize = 8;

// —— 编译期不变式 ——
const _: () = {
  assert!(SlotsType::RegularSlots as u8 == 0);
  assert!(SlotsType::TypedSlots as u8 == 1);
  assert!(K_MAX_PARALLEL_TASKS == 8);
};
