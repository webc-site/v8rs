//! v8-benchmarks CLI: 运行细粒度函数级基准测试并输出性能回归报告

use std::{env::current_exe, fs::metadata, thread::Builder};
#[global_allocator]
static ALLOC: rusty_alloc_api::RustyAlloc = rusty_alloc_api::RustyAlloc;

use std::env;

use v8_benchmarks::{compute_engine_score_from_results, run_all_benchmarks};

fn main() {
  let builder = Builder::new().stack_size(64 * 1024 * 1024);
  let handler = builder
    .spawn(|| {
      let json_mode = env::args().any(|arg| arg == "--json");
      let filter = env::args()
        .skip(1)
        .find(|arg| !arg.starts_with('-'));
      let results = if let Some(ref f) = filter {
        v8_benchmarks::run_filtered_benchmarks(f, 1)
      } else {
        run_all_benchmarks()
      };
      let suite = compute_engine_score_from_results(&results);

  if json_mode {
    println!("{{");
    println!("  \"engine\": \"rs8\",");
    println!("  \"version\": \"{}\",", env!("CARGO_PKG_VERSION"));
    println!("  \"suite\": {{");
    println!("    \"Richards\": {},", suite.richards);
    println!("    \"DeltaBlue\": {},", suite.deltablue);
    println!("    \"Crypto\": {},", suite.crypto);
    println!("    \"RayTrace\": {},", suite.raytrace);
    println!("    \"EarleyBoyer\": {},", suite.earley_boyer);
    println!("    \"RegExp\": {},", suite.regexp);
    println!("    \"Splay\": {},", suite.splay);
    println!("    \"NavierStokes\": {},", suite.navier_stokes);
    let bin_size = current_exe()
      .ok()
      .and_then(|p| {
        p.parent()
          .map(|d| d.join("rs8"))
          .filter(|p| p.exists())
          .or(Some(p))
      })
      .and_then(|p| metadata(p).ok())
      .map(|m| m.len())
      .unwrap_or(0);
    println!("    \"BinarySize\": {},", bin_size);
    println!("    \"Score\": {}", suite.score);
    println!("  }},");
    println!("  \"benchmarks\": [");
    for (i, r) in results.iter().enumerate() {
      let comma = if i + 1 < results.len() { "," } else { "" };
      println!(
        "    {{\"name\": \"{}\", \"category\": \"{}\", \"iterations\": {}, \"elapsed_us\": {}, \"ns_per_op\": {:.2}, \"ops_per_sec\": {:.0}}}{}",
        r.name, r.category, r.iterations, r.elapsed_us, r.ns_per_op, r.ops_per_sec, comma
      );
    }
    println!("  ]\n}}");
  } else {
    println!("基准用例测试结果:");
    for r in &results {
      println!(
        "{:<16} 迭代: {:>4} 耗时: {:>10.2} ns/op 吞吐量: {:>8.0} ops/sec",
        r.name, r.iterations, r.ns_per_op, r.ops_per_sec
      );
    }
    println!("几何平均综合得分: {}", suite.score);
  }
})
  .unwrap();
  handler.join().unwrap();
}
