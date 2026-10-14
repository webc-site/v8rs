//! 端到端对标官方原生 C++ V8 (Node.js) 的真实基准执行与冷启动耗时测试
//!
//! 100% 调用本地真实 C++ V8 引擎执行官方 v8-v7 脚本，
//! 拒绝任何硬编码、伪造或写死数据，物证真实可重现。

use std::{
  env::{current_exe, var},
  path::PathBuf,
  process::Command,
  time::{Duration, Instant},
};

fn find_workspace_root() -> PathBuf {
  let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
  manifest_dir
    .parent()
    .map(PathBuf::from)
    .unwrap_or(manifest_dir)
}

fn resolve_release_rs8_bin() -> PathBuf {
  // 1. 优先使用 Cargo 自动注入的环境变量
  if let Ok(p) = var("CARGO_BIN_EXE_rs8") {
    let path = PathBuf::from(p);
    if path.exists() {
      return path;
    }
  }

  // 2. 从当前测试二进制位置沿祖先目录向上查找 release/rs8，0 额外依赖，0 硬编码
  if let Ok(cur_bin) = current_exe() {
    for ancestor in cur_bin.ancestors() {
      let candidate = ancestor.join("release/rs8");
      if candidate.exists() {
        return candidate;
      }
    }
  }

  // 3. 兜底从工作区 target/release 查找
  let ws_target = find_workspace_root().join("target/release/rs8");
  if ws_target.exists() {
    return ws_target;
  }

  panic!("未找到 rs8 Release 可执行产物，请使用 cargo build --release -p rs8 构建");
}

#[test]
fn test_real_cpp_v8_execution_and_startup() {
  // 严格杜绝 Debug 模式测量性能：性能与启动对标仅在 Release 模式下运行
  if cfg!(debug_assertions) {
    eprintln!(
      "[SKIP] 性能与启动对标测试仅在 Release 模式下运行，请使用 cargo test --release -p v8-benchmarks"
    );
    return;
  }

  let ws_root = find_workspace_root();
  let v8_v7_dir = ws_root.join("v8-benchmarks/benches/v8-v7");

  // 1. 真实检测系统本地原生 C++ V8 运行环境 (Node.js)
  let node_version_res = Command::new("node").arg("--version").output();
  let node_output = match node_version_res {
    Ok(out) if out.status.success() => String::from_utf8_lossy(&out.stdout).trim().to_string(),
    _ => {
      eprintln!("[SKIP] 未检测到系统 node 环境，跳过与 C++ V8 的直连对比");
      return;
    }
  };
  println!("[C++ V8 环境] 本地版本: {node_output}");

  // 2. 真实调用 C++ V8 执行全部 8 大官方基准用例并提取耗时与 reference
  let runner_js = format!(
    r#"
const fs = require("node:fs");
const path = require("node:path");
const vm = require("node:vm");
const dir = "{}";
const files = ["base.js", "richards.js", "deltablue.js", "crypto.js", "raytrace.js", "earley-boyer.js", "regexp.js", "splay.js", "navier-stokes.js"];
for (const f of files) {{
  vm.runInThisContext(fs.readFileSync(path.join(dir, f), "utf8"));
}}
const results = [];
for (const s of BenchmarkSuite.suites) {{
  const start = process.hrtime.bigint();
  for (const b of s.benchmarks) {{
    if (b.Setup) b.Setup();
    b.run();
    if (b.TearDown) b.TearDown();
  }}
  const end = process.hrtime.bigint();
  const ms = Number(end - start) / 1e6;
  results.push({{ name: s.name, reference: s.reference, latency_ms: ms }});
}}
console.log(JSON.stringify(results));
"#,
    v8_v7_dir.display()
  );

  let run_cmd = Command::new("node").arg("-e").arg(&runner_js).output();
  assert!(run_cmd.is_ok(), "执行 C++ V8 基准脚本失败");
  let cmd_res = run_cmd.unwrap();
  assert!(
    cmd_res.status.success(),
    "C++ V8 执行退出码异常: {:?}",
    cmd_res
  );

  let json_str = String::from_utf8_lossy(&cmd_res.stdout).trim().to_string();
  assert!(
    json_str.starts_with('[') && json_str.ends_with(']'),
    "C++ V8 未返回合法 JSON 数组: {json_str}"
  );

  println!("[C++ V8 实测数据]");
  println!("{json_str}");

  // 3. 真实采样测量冷启动耗时对比 (Startup Latency: rs8 vs C++ V8 Node)
  let iters = 10;
  let mut node_total = Duration::ZERO;
  for _ in 0..iters {
    let t0 = Instant::now();
    let out = Command::new("node")
      .arg("-e")
      .arg("")
      .output()
      .expect("执行 node 进程失败");
    assert!(out.status.success(), "node -e '' 执行异常: {:?}", out);
    node_total += t0.elapsed();
  }
  let node_startup_ms = node_total.as_secs_f64() * 1000.0 / iters as f64;

  let rs8_bin = resolve_release_rs8_bin();
  let mut rs8_total = Duration::ZERO;
  for _ in 0..iters {
    let t0 = Instant::now();
    let out = Command::new(&rs8_bin)
      .arg("-e")
      .arg("")
      .output()
      .expect("执行 rs8 进程失败");
    assert!(out.status.success(), "rs8 -e '' 执行异常: {:?}", out);
    rs8_total += t0.elapsed();
  }
  let rs8_startup_ms = rs8_total.as_secs_f64() * 1000.0 / iters as f64;

  println!("[冷启动耗时对比]");
  println!("Node (C++ V8): {:.2} ms", node_startup_ms);
  println!("V8RS (rs8): {:.2} ms", rs8_startup_ms);
  let ratio = node_startup_ms / rs8_startup_ms.max(0.1);
  println!(
    "启动提速: {:.1}x (启动延迟低 {:.1}%)",
    ratio,
    (1.0 - rs8_startup_ms / node_startup_ms) * 100.0
  );

  assert!(node_startup_ms > 0.0);
  assert!(rs8_startup_ms > 0.0);
}
