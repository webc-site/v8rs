use std::ptr::null_mut;

use aok::{OK, Void};
use log::info;
use v8_diagnostics::{
  BLOCK_HINT_MARKER, BLOCK_MARKER, BUILTIN_HASH_MARKER, BasicBlockProfiler, BasicBlockProfilerData,
  BuiltinsCallGraph, VERIFY_REGISTRY, VERIFY_REGISTRY_LEN,
};

#[ctor::ctor(unsafe)]
fn _log_init() {
  log_init::init();
}

#[test]
fn test_basic_block_profiler_data_lifecycle() -> Void {
  let mut data = BasicBlockProfilerData::new(3);
  assert_eq!(data.n_blocks(), 3);
  assert_eq!(data.counts(), &[0, 0, 0]);
  assert_eq!(data.block_ids(), &[0, 0, 0]);

  data.set_block_id(0, 10);
  data.set_block_id(1, 20);
  data.set_block_id(2, 30);
  // 越界设置安全防 panic
  data.set_block_id(999, 40);

  data.set_function_name("Builtin_Test");
  data.set_schedule("B0 -> B1\nB1 -> B2");
  data.set_code("mov rax, 1\nret");
  data.set_hash(0x1234);
  data.add_branch(10, 20);

  assert_eq!(data.function_name(), "Builtin_Test");
  assert_eq!(data.schedule(), "B0 -> B1\nB1 -> B2");
  assert_eq!(data.code(), "mov rax, 1\nret");
  assert_eq!(data.hash(), 0x1234);
  assert_eq!(data.branches(), &[(10, 20)]);

  data.increment_count(0);
  data.increment_count(0);
  data.set_count(1, 5);
  assert_eq!(data.counts(), &[2, 5, 0]);

  // Log 格式断言
  let mut log_output = String::new();
  data.log(null_mut(), &mut log_output);
  assert!(log_output.contains(&format!("{BLOCK_MARKER}\tBuiltin_Test\t10\t2\n")));
  assert!(log_output.contains(&format!("{BLOCK_MARKER}\tBuiltin_Test\t20\t5\n")));
  assert!(log_output.contains(&format!("{BLOCK_HINT_MARKER}\tBuiltin_Test\t10\t20\n")));
  assert!(log_output.contains(&format!("{BUILTIN_HASH_MARKER}\tBuiltin_Test\t4660\n"))); // 0x1234 = 4660

  // Display 格式断言 (降序按 count 排序: B1: 5 次, B0: 2 次, B2 零次被裁断)
  let display_str = data.to_string();
  assert!(display_str.contains("schedule for Builtin_Test (B0 entered 2 times)"));
  assert!(display_str.contains("block counts for Builtin_Test:"));
  assert!(display_str.contains("block B1 : 5\nblock B0 : 2\n"));
  assert!(!display_str.contains("block B2 : 0"));

  // ResetCounts
  data.reset_counts();
  assert_eq!(data.counts(), &[0, 0, 0]);
  assert_eq!(data.to_string(), ""); // 全零计数无输出

  info!("test_basic_block_profiler_data_lifecycle ok");
  OK
}

#[test]
fn test_basic_block_profiler_aggregation() -> Void {
  let mut profiler = BasicBlockProfiler::default();
  assert!(!profiler.has_data(null_mut()));

  let idx0 = profiler.new_data(2);
  let idx1 = profiler.new_data(3);
  assert_eq!(idx0, 0);
  assert_eq!(idx1, 1);
  assert!(profiler.has_data(null_mut()));
  assert_eq!(profiler.get_builtins_block_count(null_mut()), 5);

  {
    let data_list = profiler.data_list_mut();
    data_list[0].set_function_name("f0");
    data_list[0].set_count(0, 10);
    data_list[0].set_count(1, 0);

    data_list[1].set_function_name("f1");
    data_list[1].set_count(0, 0);
    data_list[1].set_count(1, 20);
    data_list[1].set_count(2, 30);
  }

  // 覆盖率位图
  let bitmap = profiler.get_coverage_bitmap(null_mut());
  assert_eq!(bitmap, vec![true, false, false, true, true]);

  // 单次遍历收割并重置
  let mut executed_indices = Vec::new();
  profiler.for_each_executed_block_and_reset(null_mut(), &mut |idx| {
    executed_indices.push(idx);
  });
  assert_eq!(executed_indices, vec![0, 3, 4]);

  // 收割后全部归零
  let bitmap_after = profiler.get_coverage_bitmap(null_mut());
  assert_eq!(bitmap_after, vec![false, false, false, false, false]);

  info!("test_basic_block_profiler_aggregation ok");
  OK
}

#[test]
fn test_builtins_call_graph() -> Void {
  let mut graph = BuiltinsCallGraph::default();
  assert!(graph.all_hash_matched());

  graph.add_builtin_call(100, 200, 1);
  graph.add_builtin_call(100, 201, 1);
  graph.add_builtin_call(100, 300, 2);

  let callees = graph.get_builtin_callees(100).unwrap();
  assert_eq!(callees.len(), 2);

  let block1 = callees.get(&1).unwrap();
  assert!(block1.contains(&200));
  assert!(block1.contains(&201));
  assert!(!block1.contains(&300));

  let block2 = callees.get(&2).unwrap();
  assert!(block2.contains(&300));

  assert!(graph.get_builtin_callees(999).is_none());

  graph.set_all_hash_matched(false);
  assert!(!graph.all_hash_matched());

  info!("test_builtins_call_graph ok");
  OK
}

#[test]
fn test_verify_registry_presence() -> Void {
  assert_eq!(VERIFY_REGISTRY.len(), VERIFY_REGISTRY_LEN);
  assert_eq!(VERIFY_REGISTRY_LEN, 259);
  assert!(
    VERIFY_REGISTRY
      .iter()
      .any(|(c, m)| *c == "Object" && *m == "ObjectVerify")
  );
  assert!(
    VERIFY_REGISTRY
      .iter()
      .any(|(c, m)| *c == "HeapObject" && *m == "HeapObjectVerify")
  );
  assert!(
    VERIFY_REGISTRY
      .iter()
      .any(|(c, m)| *c == "JSObject" && *m == "JSObjectVerify")
  );

  info!("test_verify_registry_presence ok");
  OK
}
