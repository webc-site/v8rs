#!/usr/bin/env bun
// trap-handler.h → v8-trap-handler/src/lib.rs 转换脚本
// 规则: 公共 API 门面。提取 V8_TRAP_HANDLER_SUPPORTED 的平台矩阵 (host==target,
// android 排除), TrappingInstructionData, kInvalidIndex, API 清单 (Register/Release/
// Enable/IsEnabled/LandingPad/RecoveredTrapCount), 生成 crate lib.rs。
// IsTrapHandlerEnabled 的 inline 语义 (查询即冻结 can_enable) 落在 outside.rs,
// 此处仅 re-export。
import { readV8, writeSrc, banner, lines } from "../../lib.js";

const src = readV8("src/trap-handler/trap-handler.h");

// 圈定规则区原文行区间 (证据)
const regions = {
  supported: lines(src, 19, 53),
  api: lines(src, 110, 181),
};

// 对数核对: 关键声明必须仍在圈定区间内, 否则原文变更需重审脚本
function must(cond, msg) {
  if (!cond) throw new Error(`trap-handler.h 对数核对失败: ${msg}`);
}
must(
  regions.supported.join("\n").includes("V8_TRAP_HANDLER_SUPPORTED true"),
  "SUPPORTED 宏区间漂移"
);
must(src.includes("const int kInvalidIndex = -1;"), "kInvalidIndex 漂移");
must(
  regions.api.join("\n").includes("bool EnableTrapHandler(bool use_v8_handler)"),
  "EnableTrapHandler 签名漂移"
);
must(
  regions.api.join("\n").includes("void SetLandingPad(uintptr_t landing_pad)"),
  "SetLandingPad 签名漂移"
);

const body = `${banner("src/trap-handler/trap-handler.h.js", "src/trap-handler/trap-handler.h")}
//! wasm OOB 陷阱处理器: SIGSEGV/SIGBUS 拦截 → landing pad 跳转
//! 规则区原文行区间: ${JSON.stringify(Object.keys(regions))}
//!
//! posix 子集落地; windows (handler-inside-win/handler-outside-win) 与
//! simulator (handler-outside-simulator/trap-handler-simulator) 登记推迟
#![cfg_attr(docsrs, feature(doc_cfg))]

mod error;
mod inside;
mod inside_posix;
mod internal;
mod outside;
mod outside_posix;
mod shared;

pub use error::{Error, Result};
pub use inside::{is_accessed_memory_covered, is_fault_address_covered};
pub use outside::{
  enable_trap_handler, get_recovered_trap_count, is_trap_handler_enabled,
  register_covered_memory, register_handler_data, register_v8_sandbox,
  release_handler_data, set_landing_pad, unregister_covered_memory,
  unregister_v8_sandbox,
};
pub use outside_posix::{register_default_trap_handler, remove_trap_handler};

/// trap-handler.h:102 TrappingInstructionData
/// 陷入指令数据: 指令在代码对象内的偏移
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TrappingInstructionData {
  /// 指令距代码对象起点的偏移; wasm 代码 < 2GB, u32 足够
  pub instr_offset: u32,
}

/// trap-handler.h:108 kInvalidIndex: RegisterHandlerData 失败标识
pub const K_INVALID_INDEX: i32 = -1;

/// trap-handler.h:19-53 V8_TRAP_HANDLER_SUPPORTED 的 Rust 对应
/// (Rust 天然 host == target; android 因信号处理安全审查未过而被原版排除)
pub const TRAP_HANDLER_SUPPORTED: bool = cfg!(any(
  all(
    target_arch = "x86_64",
    any(target_os = "linux", target_os = "macos", target_os = "freebsd")
  ),
  all(target_arch = "aarch64", any(target_os = "linux", target_os = "macos")),
  all(target_arch = "riscv64", target_os = "linux"),
));
`;

const p = writeSrc("v8-trap-handler", "lib.rs", body);
console.log(
  `trap-handler.h.js: lib.rs 已产出 ${p} (API 8 项 + 常量 2 + 类型 1)`
);
