#!/usr/bin/env bun
// tool/src/trap-handler/handler-inside-win.h.js
// 机械提取 v8/src/trap-handler/ 缺口族: handler-inside-win.{h,cc} +
// handler-outside-win.cc + handler-outside-simulator.cc + trap-handler-simulator.h
// Windows 向量异常与模拟器 ProbeMemory 协议: 常量/结构/签名骨架
// (posix 主路径已由 tool/src/trap-handler/ 既有脚本产出, 不重复)

import { readV8, writeSrc, banner } from "../../lib.js";

const insideWinH = readV8("src/trap-handler/handler-inside-win.h");
const insideWinCc = readV8("src/trap-handler/handler-inside-win.cc");
const outsideWinCc = readV8("src/trap-handler/handler-outside-win.cc");
const outSimCc = readV8("src/trap-handler/handler-outside-simulator.cc");
const simH = readV8("src/trap-handler/trap-handler-simulator.h");

// 1. 对数守卫
const must = [
  ["inside-win.h", insideWinH, "HandleWasmTrap(EXCEPTION_POINTERS*"],
  ["inside-win.h", insideWinH, "TryHandleWasmTrap(EXCEPTION_POINTERS*"],
  ["inside-win.cc", insideWinCc, "struct TEB {"],
  ["inside-win.cc", insideWinCc, "EXCEPTION_ACCESS_VIOLATION"],
  ["inside-win.cc", insideWinCc, "thread_local_storage_pointer"],
  ["inside-win.cc", insideWinCc, "TrapHandlerGuard::IsActiveOnCurrentThread()"],
  ["outside-win.cc", outsideWinCc, "AddVectoredExceptionHandler(first, HandleWasmTrap)"],
  ["outside-win.cc", outsideWinCc, "RemoveVectoredExceptionHandler(g_registered_handler)"],
  ["simulator.h", simH, "ProbeMemory(uintptr_t address, uintptr_t pc)"],
  ["simulator.cc", outSimCc, "v8_internal_simulator_ProbeMemory"],
  ["simulator.cc", outSimCc, "v8_simulator_probe_memory_continuation"],
];
for (const [file, src, k] of must) {
  if (!src.includes(k)) throw new Error(`${file} 未找到 ${k}`);
}

// 2. TEB 布局常量 (inside-win.cc 注释机械登记)
const tebM = insideWinCc.match(/PVOID reserved\[(\d+)\];\s*\n\s*PVOID thread_local_storage_pointer;/);
if (!tebM) throw new Error("TEB 布局漂移");
const tebReserved = parseInt(tebM[1], 10); // 11
if (tebReserved !== 11) throw new Error(`TEB 保留指针应 11, 实得 ${tebReserved}`);
if (!/0x58 for 64-bit platforms, and 0x2c for 32-bit platforms/.test(insideWinCc)) {
  throw new Error("TEB TLS 偏移注释漂移");
}

// 3. 恢复协议寄存器面 (x64/arm64 展开; 模拟器分支登记)
if (!insideWinCc.includes("exception->ContextRecord->Rip = gLandingPad;")) throw new Error("x64 Rip 协议漂移");
if (!insideWinCc.includes("exception->ContextRecord->R10 = fault_addr;")) throw new Error("x64 R10 协议漂移");
if (!insideWinCc.includes("exception->ContextRecord->Pc = gLandingPad;")) throw new Error("arm64 Pc 协议漂移");
if (!insideWinCc.includes("exception->ContextRecord->X16 = fault_addr;")) throw new Error("arm64 X16 协议漂移");
if (!insideWinCc.includes("exception->ContextRecord->Rax = gLandingPad;")) throw new Error("模拟器 Rax 协议漂移");
if (!insideWinCc.includes("uintptr_t simulated_ip = exception->ContextRecord->Rdx;")) {
  throw new Error("模拟器 Rdx 读取漂移");
}

// 4. ProbeMemory 汇编序列对数 (x64: 2 参寄存器分支; arm64: x0)
if (!outSimCc.includes('"  movb (%rcx), %al')) throw new Error("x64 win 传参序列漂移");
if (!outSimCc.includes('"  movb (%rdi), %al')) throw new Error("x64 sysv 传参序列漂移");
if (!outSimCc.includes('"  ldrb wzr, [x0]')) throw new Error("arm64 probe 序列漂移");
if (!outSimCc.includes('"  xorl %eax, %eax')) throw new Error("x64 清零返回漂移");
if (!outSimCc.includes('"  mov x0, xzr')) throw new Error("arm64 清零返回漂移");

// 5. 符号命名 (SYMBOL 宏: darwin 加 '_')
if (!/MACRO, "_" #name/.test(outSimCc.replace(/\s+/g, " "))) {
  if (!/SYMBOL\(name\) "_" #name/.test(outSimCc.replace(/\s+/g, " "))) throw new Error("SYMBOL 宏漂移");
}

// —— 产物: 4 个新模块 ——
const head = (v8rel, note) => banner("src/trap-handler/handler-inside-win.h.js", v8rel, note);

// inside-win: 异常上下文中的处理协议
let rust = head("src/trap-handler/handler-inside-win.{h,cc}", "Windows 向量异常处理协议");
rust += `
/// Windows 例外码 (winnt.h 显式展开)
pub mod win_exceptions {
  /// EXCEPTION_ACCESS_VIOLATION
  pub const ACCESS_VIOLATION: u32 = 0xC000_0005;
  /// EXCEPTION_CONTINUE_EXECUTION (-1: 已修复, 回到触发点)
  pub const CONTINUE_EXECUTION: i32 = -1;
  /// EXCEPTION_CONTINUE_SEARCH (0: 不处理, 继续搜寻)
  pub const CONTINUE_SEARCH: i32 = 0;
}

/// TEB 布局 (原版 struct TEB): 11 个保留指针后是 TLS 指针;
/// 64 位平台 TLS 偏移 0x58, 32 位 0x2c (x86/x64/ARM/ARM64 通用, 编进二进制不可改)
pub mod teb_layout {
  pub const RESERVED_POINTERS: usize = ${tebReserved};
  /// TLS 指针槽序号 (0-based, 第 ${tebReserved + 1} 个指针)
  pub const TLS_POINTER_SLOT: usize = ${tebReserved};
  /// 64 位偏移 (字节)
  pub const TLS_OFFSET_64BIT: usize = 0x58;
  /// 32 位偏移 (字节)
  pub const TLS_OFFSET_32BIT: usize = 0x2c;
}

/// 恢复协议寄存器面 (原版 TryHandleWasmTrap, x64/arm64 展开)
pub mod win_restore_protocol {
  /// x64: Rip ← landing pad; 故障地址走 R10
  pub const X64_RESTORE: [&str; 2] = ["Rip=gLandingPad", "R10=fault_addr"];
  /// arm64: Pc ← landing pad; 故障地址走 X16
  pub const ARM64_RESTORE: [&str; 2] = ["Pc=gLandingPad", "X16=fault_addr"];
  /// 模拟器分支: Rax ← landing pad; Rip ← probe_memory_continuation;
  /// simulated_ip 自第二参数寄存器 Rdx 读取
  pub const SIM_RESTORE: [&str; 3] = ["Rax=gLandingPad", "Rip=continuation", "simulated_ip=Rdx"];
}

/// Windows 处理链签名骨架 (语义见 crate 内手写 inside.rs)
pub mod handler_inside_win_skel {
  /// 拒处理快路径 (原版 TryHandleWasmTrap): 非 ACCESS_VIOLATION /
  /// TEB TLS 未分配 / TrapHandlerGuard 活跃 → false; 之后挂 guard 再判覆盖
  pub fn try_handle_wasm_trap(_exception: usize) -> bool {
    unimplemented!("骨架")
  }

  /// 向量异常入口 (原版 HandleWasmTrap): 处理→CONTINUE_EXECUTION, 否则 CONTINUE_SEARCH
  pub fn handle_wasm_trap(_exception: usize) -> i32 {
    unimplemented!("骨架")
  }
}
`;
const p1 = writeSrc("v8-trap-handler", "inside_win_generated.rs", rust);

// outside-win: 注册/注销
let rust2 = head("src/trap-handler/handler-outside-win.cc", "Windows 处理器注册");
rust2 += `
/// Windows 注册骨架 (原版 handler-outside-win.cc)
pub mod handler_outside_win_skel {
  /// AddVectoredExceptionHandler(first=TRUE, HandleWasmTrap) — 插队链首
  pub fn register_default_trap_handler() -> bool {
    unimplemented!("骨架")
  }

  /// RemoveVectoredExceptionHandler(句柄), 未注册则空转
  pub fn remove_trap_handler() {
    unimplemented!("骨架")
  }

  /// first 参数 (TRUE: 链首)
  pub const FIRST_HANDLER: u32 = 1;
}
`;
const p2 = writeSrc("v8-trap-handler", "outside_win_generated.rs", rust2);

// simulator: ProbeMemory 协议 + 汇编序列
let rust3 = head(
  "src/trap-handler/trap-handler-simulator.h + handler-outside-simulator.cc",
  "模拟器 ProbeMemory 探测协议",
);
rust3 += `
/// 模拟器探测协议 (V8_TRAP_HANDLER_VIA_SIMULATOR 下生效)
pub mod simulator_protocol {
  /// ProbeMemory 符号名 (darwin 前缀 '_', 其余裸名; 原版 SYMBOL 宏)
  pub const PROBE_MEMORY_SYMBOL: &str = if cfg!(target_os = "macos") {
    "_v8_internal_simulator_ProbeMemory"
  } else {
    "v8_internal_simulator_ProbeMemory"
  };
  /// 续接点符号名
  pub const CONTINUATION_SYMBOL: &str = if cfg!(target_os = "macos") {
    "_v8_simulator_probe_memory_continuation"
  } else {
    "v8_simulator_probe_memory_continuation"
  };

  /// ProbeMemory 语义: 对 address 做 1 字节读, 不可读则触发陷阱;
  /// 处理器按 pc 查 landing pad, 经 continuation 返回该 pad; 可读则返回 0
  pub const PROBE_READ_BYTES: usize = 1;

  /// x64 探测序列 (寄存器传参: win=%rcx, sysv=%rdi; 清零 %eax; ret;
  /// 另在 continuation 处独立 ret 防 Mac subsections 剥离)
  pub const X64_PROBE_STEPS: [&str; 3] = ["movb (%arg), %al", "xorl %eax, %eax", "ret"];
  /// arm64 探测序列 (x0 传参; wzr 读; 清零 x0; ret)
  pub const ARM64_PROBE_STEPS: [&str; 3] = ["ldrb wzr, [x0]", "mov x0, xzr", "ret"];

  /// 续接点语义: 处理器已把 landing pad 写入返回值寄存器 (%rax / x0), 直接 ret
  pub const CONTINUATION_STEPS: [&str; 1] = ["ret"];
}

/// ProbeMemory 签名骨架 (原版 trap-handler-simulator.h 声明, asm 符号绑定)
pub fn probe_memory(_address: usize, _pc: usize) -> usize {
  unimplemented!("骨架")
}
`;
const p3 = writeSrc("v8-trap-handler", "simulator_generated.rs", rust3);

console.log(`written ${p1}, ${p2}, ${p3}; TEB保留=${tebReserved}, 协议=3架构`);
