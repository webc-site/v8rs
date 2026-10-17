#!/usr/bin/env bun
// tool/src/execution/frames.js — 栈帧类型表 + 帧面
// 覆盖: src/execution/frames.h + frames.cc + frames-inl.h + frame-constants.h
// 产出: v8-execution-frames/src/frames.rs
// 规则: STACK_FRAME_TYPE_LIST V( 23 条全量枚举 + wasm 条件项 + 附属枚举与帧栈遍历
// 复跑幂等: 纯文本扫描 + 固定排序 + 确定性输出。
import { banner, readV8, writeSrc } from "../../lib.js";

const GEN = "src/execution/frames.js";
const FILES = [
  "src/execution/frames.h",
  "src/execution/frames.cc",
  "src/execution/frames-inl.h",
  "src/execution/frame-constants.h",
];
const txt = Object.fromEntries(FILES.map((f) => [f, readV8(f)]));
const die = (m) => { throw new Error(`[frames.js] ${m}`); };
const count = (s, re) => (s.match(re) ?? []).length;

const h = txt["src/execution/frames.h"];
const listRegion = h.slice(h.indexOf("#define STACK_FRAME_TYPE_LIST"), h.indexOf("// Abstract base class for all stack frames."));
const unescaped = listRegion.replace(/\\\r?\n/g, "\n");
const plainItems = [...unescaped.matchAll(/\bV\(\s*(\w+)\s*,\s*(\w+)\s*\)/g)].map((m) => ({
  name: m[1],
  structName: m[2],
  isWasm: false,
}));
if (plainItems.length !== 23) die(`STACK_FRAME_TYPE_LIST 纯 V( 项数=${plainItems.length}, 期望 23`);

const wasmItems = [...unescaped.matchAll(/\bIF_WASM\w*\(\s*V\s*,\s*(\w+)\s*,\s*(\w+)\s*\)/g)].map((m) => ({
  name: m[1],
  structName: m[2],
  isWasm: true,
}));
if (wasmItems.length !== 11) die(`STACK_FRAME_TYPE_LIST wasm 项数=${wasmItems.length}, 期望 11`);

const allItems = [...plainItems, ...wasmItems];

const toPascalCase = (str) =>
  str.toLowerCase().replace(/(?:^|_)([a-z])/g, (_, g) => g.toUpperCase());

const out = [];
out.push(banner(GEN, FILES.join(", "), "栈帧类型表 34 项 (23 标准 + 11 wasm) 与栈帧遍历抽象"));
out.push(`
/// 栈帧类型 (对应原版 StackFrame::Type)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum StackFrameType {
  /// 无帧类型标记
  NoFrameType = 0,`);

let idx = 1;
for (const item of allItems) {
  const pascal = toPascalCase(item.name);
  out.push(`  /// 原版 ${item.name} (${item.structName})`);
  out.push(`  ${pascal} = ${idx},`);
  idx++;
}

out.push(`  /// 帧类型总数哨兵 (对应原版 NUMBER_OF_TYPES)
  NumberOfTypes = ${idx},
  /// 手工构造帧标记 (对应原版 MANUAL)
  Manual = ${idx + 1},
}

impl StackFrameType {
  pub const COUNT: usize = ${idx - 1};

  /// 全部具体帧类型清单 (排除 NoFrameType, NumberOfTypes, Manual)
  pub const ALL: [StackFrameType; ${idx - 1}] = [`);

for (const item of allItems) {
  out.push(`    StackFrameType::${toPascalCase(item.name)},`);
}

out.push(`  ];

  /// 是否为严格 JavaScriptFrame (可安全下转型为 JavaScriptFrame, 对应原版 IsJavaScript)
  #[inline]
  pub const fn is_javascript(self) -> bool {
    matches!(
      self,
      StackFrameType::Interpreted
        | StackFrameType::Baseline
        | StackFrameType::Maglev
        | StackFrameType::TurbofanJs
    )
  }

  /// 同 \`is_javascript\` (对应原版 is_javascript())
  #[inline]
  pub const fn is_js(self) -> bool {
    self.is_javascript()
  }

  /// 是否属于广义 JS 相关帧 (包含 BuiltinContinuation 族)
  #[inline]
  pub const fn is_js_family(self) -> bool {
    self.is_javascript()
      || matches!(
        self,
        StackFrameType::JavascriptBuiltinContinuation
          | StackFrameType::JavascriptBuiltinContinuationWithCatch
      )
  }

  /// 是否为严格 WebAssembly 基础帧 (对应原版 is_wasm())
  #[inline]
  pub const fn is_wasm(self) -> bool {
    matches!(
      self,
      StackFrameType::Wasm | StackFrameType::WasmInterpreterEntry
    )
  }

  /// 是否属于广义 WebAssembly 帧家族 (包含 11 种全部 WASM 帧)
  #[inline]
  pub const fn is_wasm_family(self) -> bool {
    matches!(
      self,`);

for (const item of wasmItems) {
  out.push(`      | StackFrameType::${toPascalCase(item.name)}`);
}

out.push(`    )
  }

  /// 是否为严格 ExitFrame (进入 C++ 运行时, 对应原版 is_exit())
  #[inline]
  pub const fn is_exit(self) -> bool {
    matches!(self, StackFrameType::Exit)
  }

  /// 是否属于广义退出帧家族 (包含内建与 API 回调退出帧)
  #[inline]
  pub const fn is_exit_family(self) -> bool {
    matches!(
      self,
      StackFrameType::Exit
        | StackFrameType::BuiltinExit
        | StackFrameType::ApiCallbackExit
        | StackFrameType::ApiConstructExit
        | StackFrameType::ApiNamedAccessorExit
        | StackFrameType::ApiIndexedAccessorExit
        | StackFrameType::WasmExit
    )
  }

  /// 对应原版大写标识符
  #[inline]
  pub const fn name(self) -> &'static str {
    match self {
      StackFrameType::NoFrameType => "NO_FRAME_TYPE",`);

for (const item of allItems) {
  out.push(`      StackFrameType::${toPascalCase(item.name)} => "${item.name}",`);
}

out.push(`      StackFrameType::NumberOfTypes => "NUMBER_OF_TYPES",
      StackFrameType::Manual => "MANUAL",
    }
  }
}

/// 内建调用续延模式 (对应原版 BuiltinContinuationMode)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum BuiltinContinuationMode {
  /// 普通机器码桩续延
  Stub = 0,
  /// JavaScript 续延
  JavaScript = 1,
  /// 带异常捕获的 JavaScript 续延
  JavaScriptWithCatch = 2,
  /// 异常处理分发续延
  JavaScriptHandleException = 3,
}

/// 栈帧信息精度模式 (对应原版 FrameInfoKind)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum FrameInfoKind {
  /// 精确模式
  Precise = 0,
  /// 保守模式
  Conservative = 1,
}

/// JS 入口标记 (对应原版 JsFrameMarker, Smi 编码以防 GC 栈扫描误判为堆指针)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum JsFrameMarker {
  /// 内部 JS 入口帧 ((0 << kSmiTagSize) | kSmiTag = 0)
  InnerJsEntryFrame = 0,
  /// 最外层 JS 入口帧 ((1 << kSmiTagSize) | kSmiTag = 2)
  OutermostJsEntryFrame = 2,
}

/// 栈帧指针与遍历抽象
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StackFrame {
  /// 帧基址指针 (Frame Pointer)
  fp: usize,
  /// 栈顶指针 (Stack Pointer)
  sp: usize,
  /// 栈帧类型
  frame_type: StackFrameType,
}

impl StackFrame {
  /// 构造栈帧描述
  #[inline]
  pub const fn new(fp: usize, sp: usize, frame_type: StackFrameType) -> Self {
    Self { fp, sp, frame_type }
  }

  /// 获取 FP
  #[inline]
  pub const fn fp(&self) -> usize {
    self.fp
  }

  /// 获取 SP
  #[inline]
  pub const fn sp(&self) -> usize {
    self.sp
  }

  /// 获取帧类型
  #[inline]
  pub const fn frame_type(&self) -> StackFrameType {
    self.frame_type
  }

  /// 计算 caller 的标准 FP 偏移位置 (以指针步长 8 为基准)
  #[inline]
  pub const fn caller_fp_address(&self) -> usize {
    self.fp
  }

  /// 计算 caller 的标准返回地址 (PC) 槽偏移位置
  #[inline]
  pub const fn caller_pc_address(&self) -> usize {
    self.fp + 8
  }

  /// 计算 caller 的标准栈顶 (SP) 偏移位置
  #[inline]
  pub const fn caller_sp_address(&self) -> usize {
    self.fp + 16
  }
}

// ── 编译期静态断言 ──
const _: () = {
  assert!(StackFrameType::COUNT == ${idx - 1});
  assert!(StackFrameType::NoFrameType as u8 == 0);
  assert!(BuiltinContinuationMode::JavaScriptWithCatch as u8 == 2);
  assert!(JsFrameMarker::InnerJsEntryFrame as u8 == 0);
  assert!(JsFrameMarker::OutermostJsEntryFrame as u8 == 2);
  assert!(StackFrameType::Interpreted.is_javascript());
  assert!(!StackFrameType::JavascriptBuiltinContinuation.is_javascript());
  assert!(StackFrameType::JavascriptBuiltinContinuation.is_js_family());
  assert!(StackFrameType::Exit.is_exit());
  assert!(!StackFrameType::BuiltinExit.is_exit());
  assert!(StackFrameType::BuiltinExit.is_exit_family());
};
`);

writeSrc("v8-execution-frames", "frames.rs", out.join("\n") + "\n");
console.log(`OK frames.js: 帧类型 ${allItems.length} 项 → v8-execution-frames/src/frames.rs`);
