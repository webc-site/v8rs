use aok::{OK, Void};
use v8_benchmarks::BenchResult;

#[test]
fn test_benchmarks_execution() -> Void {
  let groups = v8_benchmarks::parse_bench_yaml(v8_benchmarks::BENCHMARKS_YAML);
  assert!(!groups.is_empty(), "必须解析出基准分组");
  let test_group = v8_benchmarks::BenchGroup {
    name: groups[0].name,
    dir: groups[0].dir,
    base: groups[0].base,
    cases: vec!["richards"],
  };
  let results = v8_benchmarks::run_group(&test_group, 1);
  assert!(!results.is_empty(), "必须产出至少一项基准数据");
  for r in results {
    assert!(r.iterations > 0, "迭代次数必须大于 0");
    assert!(r.elapsed_us > 0, "耗时必须大于 0");
    assert!(r.ops_per_sec > 0.0, "吞吐必须大于 0");
    assert!(r.ns_per_op.is_finite(), "ns_per_op 必须有限");
    assert!(!r.name.is_empty(), "测试名不能为空");
    assert!(!r.category.is_empty(), "分组不能为空");
    assert!(r.reference > 0, "动态提取的官方 reference 必须大于 0");
  }
  OK
}

#[test]
fn test_bench_result_edge_cases() -> Void {
  let r = BenchResult::new("edge_case", "test", 0, 0);
  assert!(r.ns_per_op.is_finite(), "除零保护：ns_per_op 必须有限");
  assert!(r.ops_per_sec.is_finite(), "除零保护：ops_per_sec 必须有限");
  assert!(r.elapsed_us > 0, "最小耗时保底");
  OK
}
