#!/usr/bin/env bun
// tool/src/diagnostics/etw-jit-metadata-win.h.js
// 机械提取 v8/src/diagnostics/etw-jit-metadata-win.{h,cc} — ETW 事件元数据常量面。
// Windows 平台模块; kEtwControl*/事件 ID/keyword 常量为规则面;
// str_bytes/EventFields 等 constexpr 元编程模板属语义面, 登记跳过。
// 产物: v8-diagnostics/src/etw_jit_metadata_win.rs

import { readV8, writeSrc, banner } from "../../lib.js";

const H_REL = "src/diagnostics/etw-jit-metadata-win.h";
const CC_REL = "src/diagnostics/etw-jit-metadata-win.cc";
const h = readV8(H_REL);
readV8(CC_REL); // 家族覆盖锚: .cc 一并登记

const consts = [
  ["kEtwControlDisable", "u32"],
  ["kEtwControlEnable", "u32"],
  ["kEtwControlCaptureState", "u32"],
  ["kJScriptRuntimeKeyword", "u64"],
  ["kSourceLoadEventID", "u16"],
  ["kMethodLoadEventID", "u16"],
  ["kSourceDCStartEventID", "u16"],
  ["kMethodDCStartEventID", "u16"],
];
const values = consts.map(([name]) => {
  const m = h.match(new RegExp(`constexpr u(?:int)?(?:32|64|16)_t ${name}\\s*=\\s*([^;]+);`));
  if (!m) throw new Error(`未找到常量 ${name}`);
  const v = m[1].trim();
  if (!/^(0x[0-9a-fA-F]+|\d+)$/.test(v)) throw new Error(`常量 ${name} 非字面量: ${v}`);
  return { name, type: consts.find(([n]) => n === name)[1], value: v };
});
if (values.length !== 8) throw new Error(`ETW 元数据常量条数不符: ${values.length} != 8`);

// TRACE_LEVEL 依赖 win 头, 登记为符号
const traceLevelM = h.match(/constexpr unsigned char kTraceLevel = ([A-Z_]+);/);
if (!traceLevelM) throw new Error("未找到 kTraceLevel");

let rust = banner("src/diagnostics/etw-jit-metadata-win.h.js", "src/diagnostics/etw-jit-metadata-win.{h,cc}", "ETW JIT 事件元数据常量");
rust += `
// Windows 平台模块 (ETW), 接入时建议 #[cfg(windows)]。
// 语义面: str_bytes/MakeStrBytes/JoinBytes/Field/Header/JoinFields/EventFields/
// EventMetadata constexpr 模板元编程不属规则代码, 登记跳过。

${values
  .map(
    ({ name, type, value }) =>
      `/// 原 ${name}\npub const ${name.replace(/^k/, "").toUpperCase()}: ${type} = ${value};`,
  )
  .join("\n")}

/// 原 kTraceLevel = TRACE_LEVEL_INFORMATION (值由 Windows SDK 头定义, 接入时绑定)
pub const TRACE_LEVEL: u8 = TRACE_LEVEL_INFORMATION;

// —— 提取不变式断言 ——
const _: () = {
  assert!(ETW_CONTROL_DISABLE == 0);
  assert!(ETW_CONTROL_ENABLE == 1);
  assert!(ETW_CONTROL_CAPTURE_STATE == 2);
  assert!(JSCRIPT_RUNTIME_KEYWORD == 1);
  assert!(SOURCE_LOAD_EVENT_ID == 41);
  assert!(METHOD_LOAD_EVENT_ID == 9);
  assert!(SOURCE_DC_START_EVENT_ID == 39);
  assert!(METHOD_DC_START_EVENT_ID == 5);
};
`;

const p = writeSrc("v8-diagnostics", "etw_jit_metadata_win.rs", rust);
console.log(`written ${p} (常量 ${values.length} 条)`);
