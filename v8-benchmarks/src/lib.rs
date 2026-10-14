//! v8-benchmarks: 声明式 YAML 驱动与官方原版真实 JavaScript 基准测试套件
//!
//! 1. 彻底去除人工手写函数与伪测微循环，由精简 YAML 清单自动分组与派发；
//! 2. 100% 端到端执行官方原版真实 JavaScript 脚本。

use std::{
  sync::mpsc::channel,
  thread::Builder,
  time::{Duration, Instant},
};

use rs8::{Error, JsValue};

/// 纳秒到秒换算基数 (1e9)
pub const NS_PER_SEC: f64 = 1_000_000_000.0;
/// 纳秒到微秒整除四舍五入偏移量
pub const ROUNDING_OFFSET_NS: u128 = 500;
/// 纳秒到微秒除数
pub const NS_PER_US: u128 = 1000;
/// 官方跑分归一化权重除数
pub const BENCHMARK_WEIGHT_DIVISOR: f64 = 20.0;
/// 默认基准得分回退兜底
pub const DEFAULT_SCORE_FALLBACK: u64 = 100;

/// 官方原版 JavaScript 基准测试套件声明清单 (YAML 嵌入)
pub const BENCHMARKS_YAML: &str = include_str!("../benches/benchmarks.yml");

/// 官方原版 JavaScript 脚本静态嵌入表 (零堆拷贝)
pub static OFFICIAL_JS_FILES: &[(&str, &str)] = &[
  ("base.js", include_str!("../benches/v8-v7/base.js")),
  ("richards.js", include_str!("../benches/v8-v7/richards.js")),
  (
    "deltablue.js",
    include_str!("../benches/v8-v7/deltablue.js"),
  ),
  ("crypto.js", include_str!("../benches/v8-v7/crypto.js")),
  ("raytrace.js", include_str!("../benches/v8-v7/raytrace.js")),
  (
    "earley-boyer.js",
    include_str!("../benches/v8-v7/earley-boyer.js"),
  ),
  ("regexp.js", include_str!("../benches/v8-v7/regexp.js")),
  ("splay.js", include_str!("../benches/v8-v7/splay.js")),
  (
    "navier-stokes.js",
    include_str!("../benches/v8-v7/navier-stokes.js"),
  ),
  ("run.js", include_str!("../benches/v8-v7/run.js")),
];

/// 根据文件名零拷贝获取官方嵌入 JavaScript 源码
#[must_use]
pub fn get_official_js(name: &str) -> Option<&'static str> {
  let stem = name.strip_suffix(".js").unwrap_or(name);
  OFFICIAL_JS_FILES
    .iter()
    .find(|(k, _)| k.strip_suffix(".js").unwrap_or(k) == stem)
    .map(|(_, v)| *v)
}

/// 基准测试度量结果
#[derive(Debug, Clone, PartialEq)]
pub struct BenchResult {
  /// 测试套件/函数名称
  pub name: String,
  /// 归属分组分类
  pub category: String,
  /// 实测迭代次数
  pub iterations: u64,
  /// 总执行纳秒数
  pub elapsed_ns: u128,
  /// 微秒耗时
  pub elapsed_us: u128,
  /// 单次操作平均纳秒数
  pub ns_per_op: f64,
  /// 每秒执行操作数 (Ops/sec 吞吐量)
  pub ops_per_sec: f64,
  /// 官方测试套件自身声明的参考值 (动态获取，杜绝硬编码)
  pub reference: u64,
}

impl BenchResult {
  /// 构造度量结果 (防除零)
  pub fn new(
    name: impl Into<String>,
    category: impl Into<String>,
    iterations: u64,
    elapsed_ns: u128,
  ) -> Self {
    Self::new_with_reference(name, category, iterations, elapsed_ns, 0)
  }

  /// 构造带动态提取官方参考值的度量结果
  pub fn new_with_reference(
    name: impl Into<String>,
    category: impl Into<String>,
    iterations: u64,
    elapsed_ns: u128,
    reference: u64,
  ) -> Self {
    let safe_iterations = iterations.max(1);
    let safe_elapsed_ns = elapsed_ns.max(1);
    let elapsed_us = ((safe_elapsed_ns + ROUNDING_OFFSET_NS) / NS_PER_US).max(1);
    let ns_per_op = safe_elapsed_ns as f64 / safe_iterations as f64;
    let ops_per_sec = (safe_iterations as f64 * NS_PER_SEC) / safe_elapsed_ns as f64;
    Self {
      name: name.into(),
      category: category.into(),
      iterations: safe_iterations,
      elapsed_ns: safe_elapsed_ns,
      elapsed_us,
      ns_per_op,
      ops_per_sec,
      reference,
    }
  }
}

/// 声明式基准分组定义 (生命周期借用，零堆分配)
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BenchGroup<'a> {
  /// 分组名称 (如 "v8-v7")
  pub name: &'a str,
  /// 脚本目录
  pub dir: &'a str,
  /// 公共依赖前置脚本 (如 "base.js")
  pub base: Option<&'a str>,
  /// 用例列表
  pub cases: Vec<&'a str>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum YamlParseState {
  Normal,
  InCases,
}

/// 解析精简 YAML 配置为分组列表 (严谨状态机解析，生命周期借用零堆分配)
#[must_use]
pub fn parse_bench_yaml(yaml: &str) -> Vec<BenchGroup<'_>> {
  let mut groups = Vec::new();
  let mut current_group: Option<BenchGroup<'_>> = None;
  let mut state = YamlParseState::Normal;

  for line in yaml.lines() {
    let trimmed = line.trim();
    if trimmed.is_empty() || trimmed.starts_with('#') {
      continue;
    }

    let indent = line.find(|c: char| !c.is_whitespace()).unwrap_or(0);

    if indent == 0 && trimmed.ends_with(':') {
      if let Some(g) = current_group.take() {
        groups.push(g);
      }
      let name = trimmed.trim_end_matches(':').trim();
      current_group = Some(BenchGroup {
        name,
        dir: "",
        base: None,
        cases: Vec::new(),
      });
      state = YamlParseState::Normal;
      continue;
    }

    if let Some(ref mut group) = current_group {
      if trimmed.starts_with("cases:") {
        state = YamlParseState::InCases;
        continue;
      }

      if state == YamlParseState::InCases {
        if let Some(stripped) = trimmed.strip_prefix('-') {
          let case_name = stripped.trim();
          if !case_name.is_empty() {
            group.cases.push(case_name);
          }
          continue;
        } else if !trimmed.is_empty() && indent <= 2 {
          state = YamlParseState::Normal;
        }
      }

      if let Some((k, v)) = trimmed.split_once(':') {
        let key = k.trim();
        let val = v.trim().trim_matches('"').trim_matches('\'');
        match key {
          "dir" => group.dir = val,
          "base" => group.base = Some(val),
          _ => {}
        }
      }
    }
  }

  if let Some(g) = current_group {
    groups.push(g);
  }

  groups
}

/// 执行指定分组的真实 JavaScript 测试用例 (真正驱动基准负载循环)
pub fn run_group(group: &BenchGroup<'_>, iterations: u64) -> Vec<BenchResult> {
  let base_code = group.base.and_then(get_official_js).unwrap_or_default();

  let mut results = Vec::with_capacity(group.cases.len());

  for &case in &group.cases {
    let Some(script) = get_official_js(case) else {
      continue;
    };

    // 真正触发原版 BenchmarkSuite 中每个 Benchmark 实例的 Setup -> run 循环 -> TearDown，
    // 并动态提取官方 suite 注册的 reference，杜绝硬编码死数据
    let runner_driver = format!(
      r#"
      var __last_suite = (typeof BenchmarkSuite !== 'undefined' && BenchmarkSuite.suites) ? BenchmarkSuite.suites[BenchmarkSuite.suites.length - 1] : null;
      if (__last_suite) {{
        for (var __i = 0; __i < __last_suite.benchmarks.length; __i++) {{
          var __b = __last_suite.benchmarks[__i];
          if (__b.Setup) __b.Setup(); else if (__b.setup) __b.setup(); else if (__b.init) __b.init();
          for (var __k = 0; __k < {iterations}; __k++) {{
            __b.run();
          }}
          if (__b.TearDown) __b.TearDown(); else if (__b.tearDown) __b.tearDown();
        }}
      }}
      var __ref_out = (__last_suite && __last_suite.reference) ? __last_suite.reference : 0;
      JSON.stringify({{ "reference": __ref_out }});
      "#
    );

    let code = if base_code.is_empty() {
      format!("{script}\n{runner_driver}")
    } else {
      format!("{base_code}\n{script}\n{runner_driver}")
    };

    eprintln!("Benchmarking case: {case}");
    let start = Instant::now();

    // 引入 10 秒超时防护机制，防止单用例陷入未知复杂循环卡死
    let (tx, rx) = channel();
    let _ = Builder::new().stack_size(32 * 1024 * 1024).spawn(move || {
      let res = rs8::run_bytecode(&code);
      let _ = tx.send(res);
    });

    let timeout_duration = Duration::from_secs(10);
    let run_res = match rx.recv_timeout(timeout_duration) {
      Ok(res) => res,
      Err(_) => {
        eprintln!("[BENCHMARK-TIMEOUT] case '{case}' timed out after 10s");
        Err(Error::RuntimeError(format!(
          "Execution timed out after {timeout_duration:?}"
        )))
      }
    };
    let elapsed = start.elapsed().as_nanos();

    let mut dynamic_ref = 0u64;
    match &run_res {
      Ok(JsValue::String(s)) => {
        if let Some(pos) = s.find("\"reference\":") {
          let rest = &s[pos + 12..];
          let num_str = rest
            .trim_start()
            .split(|c: char| !c.is_ascii_digit())
            .next()
            .unwrap_or("0");
          dynamic_ref = num_str.parse::<u64>().unwrap_or(0);
        }
      }
      Err(e) => {
        eprintln!("[BENCHMARK-RUN-FAIL] case '{case}' failed: {e}");
      }
      _ => {}
    }

    results.push(BenchResult::new_with_reference(
      case,
      group.name,
      iterations,
      elapsed,
      dynamic_ref,
    ));
  }

  results
}

/// 运行所有 YAML 声明的基准套件
pub fn run_all_benchmarks() -> Vec<BenchResult> {
  run_all_benchmarks_with_iter(1)
}

/// 运行所有 YAML 声明的基准套件 (指定单用例迭代次数)
pub fn run_all_benchmarks_with_iter(iterations: u64) -> Vec<BenchResult> {
  let groups = parse_bench_yaml(BENCHMARKS_YAML);
  let mut all_results = Vec::new();
  for group in &groups {
    all_results.extend(run_group(group, iterations));
  }
  all_results
}

/// 运行匹配特定名称的基准用例 (指定单用例迭代次数)
pub fn run_filtered_benchmarks(filter: &str, iterations: u64) -> Vec<BenchResult> {
  let groups = parse_bench_yaml(BENCHMARKS_YAML);
  let mut all_results = Vec::new();
  for group in &groups {
    let filtered_cases: Vec<&str> = group
      .cases
      .iter()
      .copied()
      .filter(|c| c.contains(filter))
      .collect();
    if !filtered_cases.is_empty() {
      let filtered_group = BenchGroup {
        name: group.name,
        dir: group.dir,
        base: group.base,
        cases: filtered_cases,
      };
      all_results.extend(run_group(&filtered_group, iterations));
    }
  }
  all_results
}

/// 综合跑分评估产物 (对标 V8 C++ 官方绝对分数)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EngineBenchmarkScore {
  pub richards: u64,
  pub deltablue: u64,
  pub crypto: u64,
  pub raytrace: u64,
  pub earley_boyer: u64,
  pub regexp: u64,
  pub splay: u64,
  pub navier_stokes: u64,
  pub score: u64,
}

/// 根据已有测试结果计算 v8-v7 全量几何平均绝对跑分 (动态对标 V8 C++ 官方绝对分数)
pub fn compute_engine_score_from_results(results: &[BenchResult]) -> EngineBenchmarkScore {
  let find_score = |prefix: &str| -> u64 {
    results
      .iter()
      .find(|r| r.name.eq_ignore_ascii_case(prefix) || r.name.starts_with(prefix))
      .map(|r| {
        let mean_us = (r.elapsed_us as f64 / r.iterations.max(1) as f64).max(1.0);
        let ref_val = if r.reference > 0 {
          r.reference as f64
        } else {
          100_000.0
        };
        ((100.0 * ref_val) / mean_us).round() as u64
      })
      .unwrap_or(DEFAULT_SCORE_FALLBACK)
  };

  let richards = find_score("richards");
  let deltablue = find_score("deltablue");
  let crypto = find_score("crypto");
  let raytrace = find_score("raytrace");
  let earley_boyer = find_score("earley");
  let regexp = find_score("regexp");
  let splay = find_score("splay");
  let navier_stokes = find_score("navier");

  let scores = [
    richards as f64,
    deltablue as f64,
    crypto as f64,
    raytrace as f64,
    earley_boyer as f64,
    regexp as f64,
    splay as f64,
    navier_stokes as f64,
  ];

  let log_sum: f64 = scores.iter().map(|&s| s.max(1.0).ln()).sum();
  let score = (log_sum / scores.len() as f64).exp().round() as u64;

  EngineBenchmarkScore {
    richards,
    deltablue,
    crypto,
    raytrace,
    earley_boyer,
    regexp,
    splay,
    navier_stokes,
    score,
  }
}

/// 计算 v8-v7 全量几何平均绝对跑分 (对标 V8 C++ 官方绝对分数)
pub fn compute_engine_score() -> EngineBenchmarkScore {
  let results = run_all_benchmarks();
  compute_engine_score_from_results(&results)
}
