#!/usr/bin/env bun
// tool/src/diagnostics/code-tracer.js
// 机械提取 v8/src/diagnostics/code-tracer.h: CodeTracer 重定向文件名规则 + Scope 结构
// → v8-diagnostics/src/code_tracer.rs

import { readV8, writeSrc, banner } from "../../lib.js";

const src = readV8("src/diagnostics/code-tracer.h");

// 对数: 文件名模板三态 + 成员
if (!src.includes('"code-%d-%d.asm"')) throw new Error("缺 isolate 文件名模板");
if (!src.includes('"code-%d.asm"')) throw new Error("缺全局文件名模板");
for (const f of ["file_", "scope_depth_", "ShouldRedirect", "OpenFile", "CloseFile", "file()"]) {
  if (!src.includes(f)) throw new Error(`缺 CodeTracer 成员 ${f}`);
}
if (!src.includes("class V8_NODISCARD Scope {")) throw new Error("缺 Scope");
if (!src.includes("class V8_NODISCARD StreamScope : public Scope")) throw new Error("缺 StreamScope");

let rust = banner(
  "src/diagnostics/code-tracer.h.js",
  "src/diagnostics/code-tracer.h",
  "CodeTracer 重定向与作用域骨架",
);

rust += `
/// 原版 CodeTracer: 代码追踪输出重定向 (stdout 或 code-<pid>[-<isolate>].asm)。
/// 规则机械对应:
/// - v8_flags.redirect_code_traces_to 非空 → 该文件
/// - isolate_id >= 0 → "code-<pid>-<isolate_id>.asm"
/// - 否则 → "code-<pid>.asm"
#[derive(Debug, Clone)]
pub struct CodeTracer {
  /// 原版 file_ (FILE*; None = 尚未打开)
  file_: Option<()>,
  /// 原版 scope_depth_
  scope_depth_: i32,
  /// 原版 filename_ (128 字节缓冲)
  filename_: String,
}

/// 文件名缓冲容量 (原版 filename_ 声明)
pub const FILENAME_BUFFER_SIZE: usize = 128;

impl CodeTracer {
  /// 原版构造 CodeTracer(isolate_id) — 文件名规则机械直译
  #[must_use]
  pub fn new(isolate_id: i32, redirect_to: Option<&str>, pid: u32) -> Self {
    let filename_ = if let Some(to) = redirect_to {
      to.chars().take(FILENAME_BUFFER_SIZE - 1).collect()
    } else if isolate_id >= 0 {
      format!("code-{pid}-{isolate_id}.asm")
    } else {
      format!("code-{pid}.asm")
    };
    Self { file_: None, scope_depth_: 0, filename_ }
  }

  /// 原版 ShouldRedirect — 骨架
  #[must_use]
  pub fn should_redirect(&self) -> bool {
    todo!("待 Developer 修订: 对齐 code-tracer.cc ShouldRedirect")
  }

  /// 原版 OpenFile — 骨架
  pub fn open_file(&mut self) {
    todo!("待 Developer 修订")
  }

  /// 原版 CloseFile — 骨架
  pub fn close_file(&mut self) {
    todo!("待 Developer 修订")
  }

  /// 原版 file()
  #[inline]
  #[must_use]
  pub const fn file(&self) -> Option<&()> {
    self.file_.as_ref()
  }
}

/// 原版 CodeTracer::Scope: RAII 开关追踪文件
pub struct CodeTracerScope<'a> {
  tracer_: &'a mut CodeTracer,
}

impl<'a> CodeTracerScope<'a> {
  /// 原版 Scope 构造: 即 OpenFile
  pub fn new(tracer: &'a mut CodeTracer) -> Self {
    tracer.open_file();
    Self { tracer_: tracer }
  }
}

impl Drop for CodeTracerScope<'_> {
  /// 原版 Scope 析构: CloseFile
  fn drop(&mut self) {
    self.tracer_.close_file();
  }
}

// —— 提取不变式断言 ——
const _: () = {
  assert!(FILENAME_BUFFER_SIZE == 128);
};
`;

const p = writeSrc("v8-diagnostics", "code_tracer.rs", rust);
console.log(`code-tracer: 文件名规则 3 / 成员 3 / Scope 2 → ${p}`);
