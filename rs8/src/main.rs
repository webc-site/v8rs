//! rs8 — V8 sample shell (d8) pure Rust CLI
//!
//! 支持 CLI 执行 JavaScript 文件、读取版本号及作为 js-engine-benchmark 对标执行入口。

use rs8::{init_d8_platform, runtime::Interpreter};
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
      arg if arg.starts_with('-') => {
        // 忽略其他 V8 旗标，如 --expose-gc, --predictable 等
      }
      _ => {
        script_paths.push(arg);
      }
    }
  }

  if let Some(code) = eval_code {
    if let Err(err) = rs8::run_js(code) {
      eprintln!("rs8: {err}");
      exit(1);
    }
    return;
  }

  if !script_paths.is_empty() {
    let mut interpreter = Interpreter::new();
    for path in script_paths {
      match fs::read_to_string(path) {
        Ok(content) => {
          if let Err(err) = interpreter.run(&content) {
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
  } else {
    print_help();
  }
}
