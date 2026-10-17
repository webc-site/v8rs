use std::str::FromStr;

use aok::{OK, Void};
use log::info;
use v8_profiler::{
  AllocationTraceNode, CodeEventType, CodeType, CpuProfile, Error, HeapGraphEdge,
  HeapGraphEdgeType, HeapGraphNode, HeapSnapshot, IsNativeObject, K_GC_ENTRY_NAME,
  K_IDLE_ENTRY_NAME, K_MAX_SIMULTANEOUS_PROFILES, K_PROGRAM_ENTRY_NAME, K_ROOT_ENTRY_NAME,
  K_UNRESOLVED_FUNCTION_NAME, MAX_FRAMES_COUNT, MAX_FRAMES_COUNT_LOG2, MarkEntryAccessed,
  ProfileTree, ProfilerStats, ProfilerStatsReason, RecordCEntryFrame, RegisterState, SIGPROF,
  StringsStorage, TickSample, USE_SIGNALS,
  linux_arm64::{FP_REG, LR_REG},
  linux_x64::{RBP, RIP, RSP},
};

#[ctor::ctor(unsafe)]
fn _log_init() {
  log_init::init();
}

#[test]
fn test_sampler_constants_and_state() -> Void {
  info!("> test_sampler_constants_and_state");
  assert_eq!(MAX_FRAMES_COUNT_LOG2, 8);
  assert_eq!(MAX_FRAMES_COUNT, 255);
  const {
    assert!(USE_SIGNALS);
  }
  assert_eq!(SIGPROF, 27);

  // 验证寄存器提取索引
  assert_eq!(RBP, 10);
  assert_eq!(RSP, 15);
  assert_eq!(RIP, 16);
  assert_eq!(FP_REG, 29);
  assert_eq!(LR_REG, 30);

  // 验证采样寄存器状态
  let regs = RegisterState {
    pc: 0x1000,
    sp: 0x2000,
    fp: 0x2010,
    lr: Some(0x1008),
  };
  assert_eq!(regs.pc, 0x1000);
  assert_eq!(regs.sp, 0x2000);
  assert_eq!(regs.fp, 0x2010);
  assert_eq!(regs.lr, Some(0x1008));
  OK
}

#[test]
fn test_profile_tree_and_json() -> Void {
  info!("> test_profile_tree_and_json");
  assert_eq!(K_PROGRAM_ENTRY_NAME, "(program)");
  assert_eq!(K_IDLE_ENTRY_NAME, "(idle)");
  assert_eq!(K_GC_ENTRY_NAME, "(garbage collector)");
  assert_eq!(K_UNRESOLVED_FUNCTION_NAME, "(unresolved function)");
  assert_eq!(K_ROOT_ENTRY_NAME, "(root)");
  assert_eq!(K_MAX_SIMULTANEOUS_PROFILES, 100);

  // 1. CodeType 枚举与谓词
  let code_js = CodeType::Js;
  assert!(code_js.is_js());
  assert!(!code_js.is_wasm());
  assert_eq!(code_js.to_string(), "JS");
  assert_eq!(
    CodeType::from_str("WASM").map_err(|e| Error::InvalidState(e.to_string()))?,
    CodeType::Wasm
  );

  // 2. ProfileTree 构造与路径追加
  let mut tree = ProfileTree::new();
  assert_eq!(tree.root.name, K_ROOT_ENTRY_NAME);

  tree.add_path(&[
    (K_PROGRAM_ENTRY_NAME, CodeType::Other),
    ("main", CodeType::Js),
    ("render", CodeType::Js),
  ]);
  tree.add_path(&[
    (K_PROGRAM_ENTRY_NAME, CodeType::Other),
    ("main", CodeType::Js),
    ("render", CodeType::Js),
  ]);
  tree.add_path(&[
    (K_PROGRAM_ENTRY_NAME, CodeType::Other),
    ("main", CodeType::Js),
    ("compute", CodeType::Wasm),
  ]);

  assert_eq!(tree.total_samples(), 3);

  // 3. CpuProfile 快照与序列化
  let mut profile = CpuProfile::new("test-profile", 1_000_000);
  profile.record_sample(2, 1_000_100);
  profile.record_sample(3, 1_000_200);
  assert_eq!(profile.samples.len(), 2);
  assert_eq!(profile.end_time_us, 1_000_200);

  let json = profile.to_json_string();
  assert!(json.contains("\"title\":\"test-profile\""));
  assert!(json.contains("\"totalSamples\":2"));

  // 4. StringsStorage 字符串去重
  let mut storage = StringsStorage::new();
  let s1 = storage.get_or_insert("hello");
  assert_eq!(s1, "hello");
  assert_eq!(storage.len(), 1);
  let s2 = storage.get_or_insert("hello");
  assert_eq!(s2, "hello");
  assert_eq!(storage.len(), 1); // 再次插入未增加
  storage.get_or_insert("world");
  assert_eq!(storage.len(), 2);
  OK
}

#[test]
fn test_cpu_profiler_and_tick_sample() -> Void {
  info!("> test_cpu_profiler_and_tick_sample");
  // 1. CodeEventType
  let ev = CodeEventType::CodeCreation;
  assert!(ev.is_creation());
  assert!(!ev.is_relocation_or_deletion());
  let ev_del = CodeEventType::CodeDelete;
  assert!(ev_del.is_relocation_or_deletion());
  assert!(!ev_del.is_creation());

  // 2. RecordCEntryFrame
  assert_eq!(
    RecordCEntryFrame::IncludeCEntryFrame.to_string(),
    "IncludeCEntryFrame"
  );

  // 3. ProfilerStats 统计器
  let mut stats = ProfilerStats::new();
  stats.record_failure(ProfilerStatsReason::IsolateNotLocked);
  stats.record_failure(ProfilerStatsReason::IsolateNotLocked);
  stats.record_failure(ProfilerStatsReason::TickBufferFull);
  assert_eq!(stats.get_count(ProfilerStatsReason::IsolateNotLocked), 2);
  assert_eq!(stats.get_count(ProfilerStatsReason::TickBufferFull), 1);
  assert_eq!(stats.get_count(ProfilerStatsReason::NullPC), 0);

  // 4. TickSample
  let mut sample = TickSample::new(0x4000, 0x8000, 0x8020, 500);
  assert_eq!(sample.pc, 0x4000);
  assert_eq!(sample.sp, 0x8000);
  assert_eq!(sample.fp, 0x8020);
  assert_eq!(sample.timestamp_us, 500);
  sample.add_frame(0x4010);
  sample.add_frame(0x4020);
  assert_eq!(sample.frame_count(), 2);
  OK
}

#[test]
fn test_heap_snapshot_and_trace() -> Void {
  info!("> test_heap_snapshot_and_trace");
  // 1. HeapGraphEdgeType 谓词与转换
  let edge_weak = HeapGraphEdgeType::Weak;
  assert!(edge_weak.is_weak());
  assert!(!edge_weak.is_internal_or_hidden());
  let edge_internal = HeapGraphEdgeType::Internal;
  assert!(edge_internal.is_internal_or_hidden());
  assert!(!edge_internal.is_weak());

  // 2. MarkEntryAccessed & IsNativeObject
  assert_eq!(MarkEntryAccessed::No as i32, 0);
  assert_eq!(MarkEntryAccessed::Yes as i32, 1);
  assert_eq!(IsNativeObject::No as i32, 0);
  assert_eq!(IsNativeObject::Yes as i32, 1);

  // 3. HeapSnapshot 拓扑构造
  let mut snapshot = HeapSnapshot::new("heap-snapshot-1");
  let mut node1 = HeapGraphNode::new(1, "Root", 64);
  let node2 = HeapGraphNode::new(2, "Array", 128);

  node1.add_edge(HeapGraphEdge::new(HeapGraphEdgeType::Element, "0", 2));
  snapshot.add_node(node1);
  snapshot.add_node(node2);

  assert_eq!(snapshot.node_count(), 2);
  assert_eq!(snapshot.total_shallow_size(), 192);
  assert_eq!(snapshot.total_edge_count(), 1);

  // 4. AllocationTraceNode 分配追踪
  let mut alloc_node = AllocationTraceNode::new(10, "ArrayBuffer");
  alloc_node.record_allocation(1024);
  alloc_node.record_allocation(2048);
  assert_eq!(alloc_node.allocation_count, 2);
  assert_eq!(alloc_node.allocation_size, 3072);
  OK
}
