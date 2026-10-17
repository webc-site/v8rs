use aok::{OK, Void};
use log::info;
use v8_diagnostics::{
  AsPrintableStatistics, BasicStats, CompilationStatistics, printable_statistics,
};

#[ctor::ctor(unsafe)]
fn _log_init() {
  log_init::init();
}

#[test]
fn test_basic_stats_accumulation() -> Void {
  let mut s1 = BasicStats {
    delta_us: 1000,
    total_allocated_bytes_: 512,
    max_allocated_bytes_: 256,
    absolute_max_allocated_bytes_: 256,
    input_graph_size_: 10,
    output_graph_size_: 15,
    function_name_: "fn_small".to_string(),
  };

  let s2 = BasicStats {
    delta_us: 2000,
    total_allocated_bytes_: 1024,
    max_allocated_bytes_: 1000,
    absolute_max_allocated_bytes_: 2048,
    input_graph_size_: 20,
    output_graph_size_: 35,
    function_name_: "fn_big".to_string(),
  };

  s1.accumulate(&s2);
  assert_eq!(s1.delta_us, 3000);
  assert_eq!(s1.total_allocated_bytes_, 1536);
  assert_eq!(s1.absolute_max_allocated_bytes_, 2048);
  assert_eq!(s1.max_allocated_bytes_, 1000);
  assert_eq!(s1.function_name_, "fn_big");
  assert_eq!(s1.input_graph_size_, 30);
  assert_eq!(s1.output_graph_size_, 50);

  // 累加绝对值更小的对象，不应冲掉最大值与函数名
  let s3 = BasicStats {
    delta_us: 500,
    total_allocated_bytes_: 100,
    max_allocated_bytes_: 50,
    absolute_max_allocated_bytes_: 100,
    input_graph_size_: 5,
    output_graph_size_: 5,
    function_name_: "fn_smaller".to_string(),
  };
  s1.accumulate(&s3);
  assert_eq!(s1.absolute_max_allocated_bytes_, 2048);
  assert_eq!(s1.function_name_, "fn_big");

  info!("test_basic_stats_accumulation ok");
  OK
}

#[test]
fn test_basic_stats_as_json() -> Void {
  let stats = BasicStats {
    delta_us: 1500,
    total_allocated_bytes_: 4096,
    max_allocated_bytes_: 2048,
    absolute_max_allocated_bytes_: 8192,
    input_graph_size_: 100,
    output_graph_size_: 120,
    function_name_: "main".to_string(),
  };

  let json = stats.as_json();
  assert_eq!(
    json,
    r#"{"function_name":"main","total_allocated_bytes":4096,"max_allocated_bytes":2048,"absolute_max_allocated_bytes":8192}"#
  );

  info!("test_basic_stats_as_json ok");
  OK
}

#[test]
fn test_compilation_statistics_phases_and_totals() -> Void {
  let mut cs = CompilationStatistics::new();

  let phase_stat1 = BasicStats {
    delta_us: 5000,
    total_allocated_bytes_: 1024,
    max_allocated_bytes_: 512,
    absolute_max_allocated_bytes_: 512,
    input_graph_size_: 50,
    output_graph_size_: 60,
    function_name_: "foo".to_string(),
  };

  let phase_stat2 = BasicStats {
    delta_us: 3000,
    total_allocated_bytes_: 2048,
    max_allocated_bytes_: 1024,
    absolute_max_allocated_bytes_: 1024,
    input_graph_size_: 60,
    output_graph_size_: 70,
    function_name_: "foo".to_string(),
  };

  cs.record_phase_stats("parsing", "scanner", &phase_stat1);
  cs.record_phase_stats("parsing", "parser", &phase_stat2);
  cs.record_phase_kind_stats("parsing", &phase_stat1);
  cs.record_phase_kind_stats("parsing", &phase_stat2);
  cs.record_total_stats(&phase_stat1);
  cs.record_total_stats(&phase_stat2);

  assert_eq!(cs.phase_map().len(), 2);
  assert_eq!(cs.phase_kind_map().len(), 1);
  assert_eq!(cs.total_stats().count_, 2);
  assert_eq!(cs.total_stats().base.delta_us, 8000);
  assert_eq!(cs.total_stats().base.total_allocated_bytes_, 3072);

  let scanner = cs.phase_map().get("scanner").unwrap();
  assert_eq!(scanner.insert_order_, 0);
  assert_eq!(scanner.phase_kind_name_, "parsing");
  assert_eq!(scanner.base.delta_us, 5000);

  let parser = cs.phase_map().get("parser").unwrap();
  assert_eq!(parser.insert_order_, 1);
  assert_eq!(parser.phase_kind_name_, "parsing");
  assert_eq!(parser.base.delta_us, 3000);

  info!("test_compilation_statistics_phases_and_totals ok");
  OK
}

#[test]
fn test_printable_statistics_formats() -> Void {
  let mut cs = CompilationStatistics::new();

  let stat = BasicStats {
    delta_us: 10_000, // 10 ms
    total_allocated_bytes_: 2000,
    max_allocated_bytes_: 1000,
    absolute_max_allocated_bytes_: 1000,
    input_graph_size_: 100,
    output_graph_size_: 150,
    function_name_: "compute".to_string(),
  };

  cs.record_phase_stats("codegen", "instruction_select", &stat);
  cs.record_phase_kind_stats("codegen", &stat);
  cs.record_total_stats(&stat);

  // 人读格式
  let printable_human = AsPrintableStatistics {
    compiler: "TurboFan",
    s: &cs,
    machine_output: false,
  };
  let human_output = printable_statistics(&printable_human);
  assert!(human_output.contains("TurboFan phase"));
  assert!(human_output.contains("instruction_select"));
  assert!(human_output.contains("codegen"));
  assert!(human_output.contains("totals"));
  assert!(human_output.contains("compute"));

  // 机器格式
  let printable_machine = AsPrintableStatistics {
    compiler: "TurboFan",
    s: &cs,
    machine_output: true,
  };
  let machine_output = printable_statistics(&printable_machine);
  assert!(machine_output.contains("\"TurboFan_codegen_time\"=10.000"));
  assert!(machine_output.contains("\"TurboFan_codegen_space\"=2000"));
  assert!(machine_output.contains("\"TurboFan_totals_count\"=1"));

  info!("test_printable_statistics_formats ok");
  OK
}

#[test]
fn test_edge_zero_safety() -> Void {
  // 零耗时、零分配、零图规模极限边界
  let cs_empty = CompilationStatistics::new();
  let printable = AsPrintableStatistics {
    compiler: "Ignition",
    s: &cs_empty,
    machine_output: false,
  };
  let output = printable_statistics(&printable);
  assert!(output.contains("totals"));

  let printable_mach = AsPrintableStatistics {
    compiler: "Ignition",
    s: &cs_empty,
    machine_output: true,
  };
  let mach_out = printable_statistics(&printable_mach);
  assert!(mach_out.contains("\"Ignition_totals_count\"=0"));

  // output_graph_size == 0 分支 (15 空格)
  let mut cs_zero_graph = CompilationStatistics::new();
  let zero_stat = BasicStats {
    delta_us: 0,
    total_allocated_bytes_: 0,
    max_allocated_bytes_: 0,
    absolute_max_allocated_bytes_: 0,
    input_graph_size_: 0,
    output_graph_size_: 0,
    function_name_: String::new(),
  };
  cs_zero_graph.record_phase_stats("opt", "inline", &zero_stat);
  cs_zero_graph.record_phase_kind_stats("opt", &zero_stat);
  cs_zero_graph.record_total_stats(&zero_stat);

  let out_zero = printable_statistics(&AsPrintableStatistics {
    compiler: "Opt",
    s: &cs_zero_graph,
    machine_output: false,
  });
  assert!(out_zero.contains("inline"));

  info!("test_edge_zero_safety ok");
  OK
}
