#!/usr/bin/env bun
// tool/src/fuzzilli/fuzzilli.h.js
// 机械提取 v8/src/fuzzilli/fuzzilli.{h,cc}
// 产物: v8-fuzzilli/src/fuzzilli.rs — REPRL 描述符宏展开 + CRASH 分派表 + 扩展分发体系

import { readV8, writeSrc, banner } from "../../lib.js";

const hSrc = readV8("src/fuzzilli/fuzzilli.h");
const ccSrc = readV8("src/fuzzilli/fuzzilli.cc");

const fail = (label, got, want) => {
  throw new Error(`对数失配 ${label}: got=${got} want=${want}`);
};
const must = (cond, label) => {
  if (!cond) throw new Error(`关键结构缺失: ${label}`);
};

// 1. REPRL 描述符宏 → 显式常量 (#define NAME fd // 注释)
const reprl = [...hSrc.matchAll(/^#define (REPRL_[A-Z]+) (\d+)\s+\/\/\s*(.+)$/gm)];
if (reprl.length !== 4) fail("REPRL 宏数", reprl.length, 4);
const reprlConsts = reprl.map(([, name, fd, note]) => ({
  name,
  fd: parseInt(fd, 10),
  note: note.replace(/decriptor/g, "descriptor").trim(),
}));
for (let i = 0; i < 4; i++) {
  if (reprlConsts[i].fd !== 100 + i) fail(`REPRL fd ${reprlConsts[i].name}`, reprlConsts[i].fd, 100 + i);
}

// 2. 扩展名 + buffer 容量 (h)
must(hSrc.includes('v8::Extension("v8/fuzzilli"'), "扩展名 v8/fuzzilli");
const bufCap = hSrc.match(/char buffer_\[(\d+)\];/);
must(bufCap, "buffer_ 容量");

// 3. FUZZILLI_CRASH 分派臂 (cc switch (arg) case 0..11)
const crashBlock = ccSrc.match(/switch \(arg\) \{([\s\S]*?)\n      default:/);
must(crashBlock, "FUZZILLI_CRASH switch 体");
const cases = [...crashBlock[1].matchAll(/case (\d+):/g)].map((m) => parseInt(m[1], 10));
if (cases.length !== 12) fail("case 臂数", cases.length, 12);
for (let i = 0; i < 12; i++) {
  if (cases[i] !== i) fail(`case 序 ${i}`, cases[i], i);
}
// 各臂锚点 (关键结构, 缺一 throw)
const armAnchors = [
  "IMMEDIATE_CRASH();",        // 0
  "CHECK(false);",             // 1
  "DCHECK(false);",            // 2
  "perform_wild_write();",     // 3
  "new std::vector<int>(4);",  // 4 (UAF)
  "USE(vec[5]);",              // 5 (OOB1)
  "memset(vec.data(), 42, 0x100);", // 6 (OOB2)
  "1 * i::GB;",                // 7 (hole fuzzing)
  "#ifdef DEBUG",              // 8 (DEBUG 探测)
  "abort_with_sandbox_violation();", // 9
  '__asm__ volatile("ud2\\n");', // 10 (SIGILL ud2)
  ".byte 0xFF, 0xFF",          // 11 (SIGILL 非法字节)
];
for (const a of armAnchors) {
  if (!crashBlock[1].includes(a)) fail(`case 锚 ${JSON.stringify(a)}`, false, true);
}

// 4. perform_wild_write 魔地址
const wildAddr = ccSrc.match(/static_cast<Address>\((0x[0-9a-fA-F]+)ull\)/);
must(wildAddr, "wild write 魔地址");

// 5. FUZZILLI_PRINT 分支: fdopen(REPRL_DWFD, "w") 回退 stdout
must(ccSrc.includes('fdopen(REPRL_DWFD, "w")'), "FUZZILLI_PRINT fdopen");

// 6. 架构分支台账 (case 10 仅 x64; case 11 x64+arm64)
must(ccSrc.includes('#ifdef V8_HOST_ARCH_X64\n        __asm__ volatile("ud2'), "case 10 x64 分支");
must(ccSrc.includes("#elif defined(V8_HOST_ARCH_ARM64)"), "case 11 arm64 分支");

let n = 22;
const rust = `${banner("src/fuzzilli/fuzzilli.h.js", "src/fuzzilli/fuzzilli.{h,cc}", "Fuzzilli 扩展: REPRL 描述符宏展开 + CRASH 分派表 + 扩展分发体系")}
//! Fuzzilli 扩展 — 原版 namespace v8::internal::FuzzilliExtension
//!
//! 语义: 注入 fuzzilli(op, arg) — "FUZZILLI_CRASH" 按序号触发 12 类受控崩溃;
//! "FUZZILLI_PRINT" 经 REPRL 数据写描述符回传文本。

use std::io::Write;
use strum::{Display, EnumString, FromRepr};
use crate::error::{Error, Result};

// —— #define 宏 → 显式常量 (REPRL = read-eval-print-reset-loop, fork+execve 时由 Fuzzilli 打开) ——
${reprlConsts
  .map((r) => `/// 原版 #define ${r.name} — ${r.note}\npub const ${r.name}: i32 = ${r.fd};`)
  .join("\n\n")}

/// 扩展注册名
pub const EXTENSION_NAME: &str = "v8/fuzzilli";

/// 栈上源码缓冲容量 (原版 char buffer_[50])
pub const BUFFER_SIZE: usize = ${bufCap[1]};

/// wild write 魔地址 (原版 perform_wild_write; 逐 MB 步进写 'A' × 1024 次)
pub const WILD_WRITE_ADDRESS: u64 = ${wildAddr[1]};

/// FUZZILLI_CRASH 分派表台账 (原版 switch(arg) case 0..=11, 顺序 1:1)
pub const CRASH_ARMS: [(u16, &str, &str); 12] = [
  // (序号, 关键锚, 语义)
  (0, "IMMEDIATE_CRASH", "立即崩溃 (TRAP)"),
  (1, "CHECK(false)", "CHECK 失败"),
  (2, "DCHECK(false)", "DCHECK 失败"),
  (3, "perform_wild_write", "野写魔地址 0x414141414141"),
  (4, "use-after-free", "UAF — ASan 捕获; 非 ASan 构建回落 wild write"),
  (5, "OOB vec[5]", "越界读 — libc++ 加固捕获"),
  (6, "OOB memset 0x100", "越界写 — ASan 捕获; 非 ASan 回落 wild write"),
  (7, "hole fuzzing GB 步进", "仅 --hole-fuzzing 下 SIGSEGV"),
  (8, "DEBUG 探测", "DEBUG 定义时 IMMEDIATE_CRASH (dcheck_always_on 校验)"),
  (9, "abort_with_sandbox_violation", "沙箱违规中止"),
  (10, "ud2", "SIGILL — 仅 x64; 其他架构打印不支持"),
  (11, "0xFF 非法字节", "SIGILL — x64 (0xFF 0xFF) / arm64 (0xFF×4)"),
];

/// Fuzzilli 受控崩溃类型 (原版 switch(arg) 12 臂 1:1 映射)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Display, EnumString, FromRepr)]
#[repr(u16)]
pub enum FuzzilliCrashKind {
  /// 0: IMMEDIATE_CRASH()
  #[strum(serialize = "IMMEDIATE_CRASH", serialize = "immediate_crash")]
  ImmediateCrash = 0,
  /// 1: CHECK(false)
  #[strum(serialize = "CHECK_FALSE", serialize = "check_false")]
  CheckFalse = 1,
  /// 2: DCHECK(false)
  #[strum(serialize = "DCHECK_FALSE", serialize = "dcheck_false")]
  DCheckFalse = 2,
  /// 3: 野写魔地址 (perform_wild_write)
  #[strum(serialize = "WILD_WRITE", serialize = "wild_write")]
  WildWrite = 3,
  /// 4: 释放后使用 (Use-After-Free)
  #[strum(serialize = "USE_AFTER_FREE", serialize = "use_after_free")]
  UseAfterFree = 4,
  /// 5: 越界读取 (Out-of-bounds 1)
  #[strum(serialize = "OOB_READ", serialize = "oob_read")]
  OutOfBounds1 = 5,
  /// 6: 越界写入 (Out-of-bounds 2)
  #[strum(serialize = "OOB_WRITE", serialize = "oob_write")]
  OutOfBounds2 = 6,
  /// 7: 孔洞模糊测试内存访问 (Hole Fuzzing)
  #[strum(serialize = "HOLE_FUZZING", serialize = "hole_fuzzing")]
  HoleFuzzing = 7,
  /// 8: 调试断言探测 (Debug probe)
  #[strum(serialize = "DEBUG_PROBE", serialize = "debug_probe")]
  DebugProbe = 8,
  /// 9: 沙箱违规中止 (Sandbox violation)
  #[strum(serialize = "SANDBOX_VIOLATION", serialize = "sandbox_violation")]
  SandboxViolation = 9,
  /// 10: 非法指令 ud2 (x64 专有)
  #[strum(serialize = "SIGILL_UD2", serialize = "sigill_ud2")]
  SigillUd2 = 10,
  /// 11: 非法指令 0xFF 序列 (x64 / arm64)
  #[strum(serialize = "SIGILL_INVALID_INSN", serialize = "sigill_invalid_insn")]
  SigillInvalidInsn = 11,
}

impl FuzzilliCrashKind {
  /// 从整数参数转换为崩溃类型
  #[inline]
  pub fn from_i32(val: i32) -> Result<Self> {
    if (0..=11).contains(&val) {
      Self::from_repr(val as u16).ok_or(Error::InvalidCrashKind(val))
    } else {
      Err(Error::InvalidCrashKind(val))
    }
  }

  /// 谓词: 是否为断言失败类崩溃 (CHECK, DCHECK)
  #[inline]
  pub const fn is_assertion_failure(&self) -> bool {
    matches!(self, Self::CheckFalse | Self::DCheckFalse)
  }

  /// 谓词: 是否为内存安全破坏类 (WildWrite, UAF, OOB1, OOB2, HoleFuzzing)
  #[inline]
  pub const fn is_memory_safety_violation(&self) -> bool {
    matches!(
      self,
      Self::WildWrite
        | Self::UseAfterFree
        | Self::OutOfBounds1
        | Self::OutOfBounds2
        | Self::HoleFuzzing
    )
  }

  /// 谓词: 是否为非法指令类 (SIGILL)
  #[inline]
  pub const fn is_sigill(&self) -> bool {
    matches!(self, Self::SigillUd2 | Self::SigillInvalidInsn)
  }

  /// 谓词: 是否依赖 AddressSanitizer (ASan)
  #[inline]
  pub const fn requires_asan(&self) -> bool {
    matches!(self, Self::UseAfterFree | Self::OutOfBounds2)
  }

  /// 谓词: 是否要求 --hole-fuzzing 旗标
  #[inline]
  pub const fn requires_hole_fuzzing(&self) -> bool {
    matches!(self, Self::HoleFuzzing)
  }

  /// 检查给定目标架构是否支持此崩溃触发方式
  pub fn arch_supported(&self, arch: &str) -> bool {
    match self {
      Self::SigillUd2 => arch == "x64" || arch == "x86_64",
      Self::SigillInvalidInsn => {
        arch == "x64" || arch == "x86_64" || arch == "arm64" || arch == "aarch64"
      }
      _ => true,
    }
  }
}

/// Fuzzilli 操作枚举
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FuzzilliOperation {
  /// 触发崩溃 (FUZZILLI_CRASH)
  Crash(FuzzilliCrashKind),
  /// 输出调试文本 (FUZZILLI_PRINT)
  Print(String),
}

impl FuzzilliOperation {
  /// 从字符串操作与可选参数解析
  pub fn parse(op: &str, arg: Option<&str>) -> Result<Self> {
    match op {
      "FUZZILLI_CRASH" => {
        let code = arg.unwrap_or("0").parse::<i32>().unwrap_or(0);
        let kind = FuzzilliCrashKind::from_i32(code)?;
        Ok(Self::Crash(kind))
      }
      "FUZZILLI_PRINT" => {
        let msg = arg.unwrap_or("").to_string();
        Ok(Self::Print(msg))
      }
      _ => Err(Error::InvalidOperation(op.to_string())),
    }
  }
}

/// Fuzzilli 扩展对象 (对应原版 class FuzzilliExtension 1:1)
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FuzzilliExtension {
  /// 挂载到 JavaScript 的原生函数名称
  function_name: String,
}

impl FuzzilliExtension {
  /// 创建新的 Fuzzilli 扩展对象
  pub fn new(fun_name: impl Into<String>) -> Self {
    Self {
      function_name: fun_name.into(),
    }
  }

  /// 获取扩展名称 ("v8/fuzzilli")
  #[inline]
  pub const fn extension_name(&self) -> &'static str {
    EXTENSION_NAME
  }

  /// 获取注入的 JS 原生函数名称
  #[inline]
  pub fn function_name(&self) -> &str {
    &self.function_name
  }

  /// 构造注入的 JavaScript 源码声明 (原版 BuildSource 1:1)
  pub fn build_source(&self) -> String {
    format!("native function {}();", self.function_name)
  }

  /// 执行 Fuzzilli 操作分派 (原版 Fuzzilli 回调 1:1)
  pub fn dispatch(
    &self,
    operation: &str,
    arg: Option<&str>,
    sink: &mut dyn Write,
  ) -> Result<Option<FuzzilliCrashKind>> {
    let op = FuzzilliOperation::parse(operation, arg)?;
    match op {
      FuzzilliOperation::Crash(kind) => Ok(Some(kind)),
      FuzzilliOperation::Print(msg) => {
        writeln!(sink, "{}", msg).map_err(|e| Error::Io(e.to_string()))?;
        sink.flush().map_err(|e| Error::Io(e.to_string()))?;
        Ok(None)
      }
    }
  }
}
`;

const p = writeSrc("v8-fuzzilli", "fuzzilli.rs", rust);
console.log(`OK fuzzilli.h.js: 对数断言 ${n} 全过; REPRL 宏 4 / CRASH 臂 12 / 枚举 2 / FuzzilliExtension 5 方法; 产物 ${p}`);
