#!/usr/bin/env bun
// tool/src/diagnostics/etw-jit-win.h.js
// 机械提取 v8/src/diagnostics/etw-jit-win.{h,cc} — ETW JIT 事件接入面。
// Windows 平台模块; kEtwRundown 位掩码 + 静态开关 + 接口函数签名骨架。
// 产物: v8-diagnostics/src/etw_jit_win.rs

import { readV8, writeSrc, banner } from "../../lib.js";

const H_REL = "src/diagnostics/etw-jit-win.h";
const CC_REL = "src/diagnostics/etw-jit-win.cc";
const h = readV8(H_REL);
const cc = readV8(CC_REL);

const rundownM = h.match(/constexpr uint32_t kEtwRundown = ([^;]+);/);
if (!rundownM) throw new Error("未找到 kEtwRundown");
const rundown = rundownM[1].trim();
if (!/^(0x[0-9a-fA-F]+|\d+)$/.test(rundown)) throw new Error(`kEtwRundown 非字面量: ${rundown}`);
if (!h.includes("static_assert(kEtwRundown > JitCodeEventOptions::kLastJitCodeEventOption);")) {
  throw new Error("kEtwRundown static_assert 锚缺失");
}
if (!h.includes("has_active_etw_tracing_session_or_custom_filter")) throw new Error("缺静态开关锚");

const sigs = [...h.matchAll(/^\s*((?:void)\s+\w+\([^;]*\));/gm)].map((m) => m[1].replace(/\s+/g, " ").trim());
if (sigs.length < 6) throw new Error(`ETWJITInterface 函数签名不足: ${sigs.length}`);
if (!cc.includes("kEtwControlCaptureState")) throw new Error("etw-jit-win.cc 缺 ControlState 使用锚");

let rust = banner("src/diagnostics/etw-jit-win.h.js", "src/diagnostics/etw-jit-win.{h,cc}", "ETW JIT 接入骨架");
rust += `
// Windows 平台模块 (ETW), 接入时建议 #[cfg(windows)]。
// 语义面: 事件发送/元数据注册/回调实现登记跳过, 由 Developer 修订补齐。

/// Rundown 事件位掩码 (原 kEtwRundown; CaptureState 回调触发时对事件 ID 加此位)
pub const ETW_RUNDOWN: u32 = ${rundown};

/// 原 ETWJITInterface::has_active_etw_tracing_session_or_custom_filter (原子开关)
pub static HAS_ACTIVE_ETW_TRACING_SESSION_OR_CUSTOM_FILTER: std::sync::atomic::AtomicBool =
    std::sync::atomic::AtomicBool::new(false);

/// 原 namespace ETWJITInterface 接口函数签名骨架
pub struct EtwJitInterface;

impl EtwJitInterface {
${sigs.map((s) => `  // 原签名: ${s}`).join("\n")}
}

// —— 提取不变式断言 ——
const _: () = {
  assert!(ETW_RUNDOWN == 0xF0000000);
};
`;

const p = writeSrc("v8-diagnostics", "etw_jit_win.rs", rust);
console.log(`written ${p} (签名 ${sigs.length} 条)`);
