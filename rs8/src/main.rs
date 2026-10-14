//! rs8 — V8 sample shell (d8) pure Rust CLI
//!
//! 支持 CLI 执行 JavaScript 文件、读取版本号及作为 js-engine-benchmark 对标执行入口。

use rs8::{ExecMode, init_d8_platform, runtime::Interpreter};
#[global_allocator]
static ALLOC: rusty_alloc_api::RustyAlloc = rusty_alloc_api::RustyAlloc;

use std::{env, fs, process::exit};

const VERSION: &str = env!("CARGO_PKG_VERSION");

fn print_help() {
  println!("rs8: V8 JavaScript Shell (pure Rust remake)");
  println!("Usage: rs8 [options] [file.js] [-- [arguments]]\n");
  println!("Options:");
  println!("  --version, -v      Print rs8 version and exit");
  println!("  --help, -h         Print this help message and exit");
  println!("  --shell            Run interactive shell");
  println!("  --print-bytecode-coverage  打印主链路 (Ignition 字节码) 实测覆盖率");
  println!("  --no-bytecode      强制走 AST 求值器, 用于差分对照");
  println!("  --bytecode-only    只允许字节码引擎, 不支持即报错");
}

fn main() {
  let _platform = init_d8_platform();
  let args: Vec<String> = env::args().collect();

  if args.len() <= 1 {
    print_help();
    return;
  }

  let mut script_paths: Vec<&str> = Vec::new();
  let mut eval_code: Option<&str> = None;
  let mut mode = ExecMode::default();
  let mut print_coverage = false;
  let mut iter = args[1..].iter();

  while let Some(arg) = iter.next() {
    match arg.as_str() {
      "--version" | "-v" => {
        println!("rs8 {VERSION} (v8rs)");
        return;
      }
      "--help" | "-h" => {
        print_help();
        return;
      }
      "-e" | "--eval" => {
        if let Some(code) = iter.next() {
          eval_code = Some(code);
        }
      }
      "--print-bytecode-coverage" => print_coverage = true,
      "--no-bytecode" => mode = ExecMode::AstOnly,
      "--bytecode-only" => mode = ExecMode::BytecodeOnly,
      arg if arg.starts_with('-') => {
        // 忽略其他 V8 旗标，如 --expose-gc, --predictable 等
      }
      _ => {
        script_paths.push(arg);
      }
    }
  }

  if eval_code.is_none() && script_paths.is_empty() {
    print_help();
    return;
  }

  // 单个 Interpreter 贯穿 -e 与全部脚本, 保证全局作用域与 d8 一致并累计覆盖率
  let mut interp = Interpreter::new();
  interp.mode = mode;

  if let Some(code) = eval_code {
    if let Err(err) = interp.run(code) {
      eprintln!("rs8: {err}");
      exit(1);
    }
  }

  for path in &script_paths {
    match fs::read_to_string(path) {
      Ok(content) => {
        if let Err(err) = interp.run(&content) {
          eprintln!("rs8: {err}");
          exit(1);
        }
      }
      Err(err) => {
        eprintln!("rs8: Error reading file '{path}': {err}");
        exit(1);
      }
    }
  }

  if print_coverage {
    interp.print_coverage();
  }
}
