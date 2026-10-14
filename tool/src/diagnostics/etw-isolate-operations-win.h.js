#!/usr/bin/env bun
// tool/src/diagnostics/etw-isolate-operations-win.h.js
// 机械提取 v8/src/diagnostics/etw-isolate-operations-win.{h,cc} —
// ETW isolate 操作虚接口面 (签名骨架 + 单例登记)。
// Windows 平台模块; 虚方法实现登记跳过。
// 产物: v8-diagnostics/src/etw_isolate_operations_win.rs

import { readV8, writeSrc, banner } from "../../lib.js";

const H_REL = "src/diagnostics/etw-isolate-operations-win.h";
const CC_REL = "src/diagnostics/etw-isolate-operations-win.cc";
const h = readV8(H_REL);
readV8(CC_REL); // 家族覆盖锚

const classM = h.match(/class V8_EXPORT_PRIVATE EtwIsolateOperations\s*\{([\s\S]*?)\n\};/);
if (!classM) throw new Error("未找到 EtwIsolateOperations 类");
const sigs = [...classM[1].matchAll(/^\s*(?:virtual\s+)?([A-Za-z_:<>][^;]*\([^;]*\));/gm)]
  .map((m) => m[1].replace(/\s+/g, " ").trim())
  .filter((s) => !s.startsWith("EtwIsolateOperations("));
const virtualCount = [...classM[1].matchAll(/virtual /g)].length;
if (virtualCount !== 6) throw new Error(`虚方法条数不符: ${virtualCount} != 6`);
if (!h.includes("static EtwIsolateOperations* Instance();")) throw new Error("缺 Instance 单例锚");

let rust = banner("src/diagnostics/etw-isolate-operations-win.h.js", "src/diagnostics/etw-isolate-operations-win.{h,cc}", "ETW isolate 操作接口骨架");
rust += `
// Windows 平台模块 (ETW), 接入时建议 #[cfg(windows)]。
// 语义面: 虚方法实现 (中断/代码事件句柄/URL 过滤回调) 登记跳过。

/// 原 class EtwIsolateOperations — 可注入虚接口 (测试替换 Instance)
pub trait EtwIsolateOperations {
${sigs.filter((s) => !s.startsWith("static ")).map((s) => `  // 原签名: ${s}`).join("\n")}
}

/// 原 EtwIsolateOperations::Instance() 全局单例槽
pub static INSTANCE: std::sync::Mutex<Option<()>> = std::sync::Mutex::new(None);

// 单例操作原签名:
${sigs.filter((s) => s.startsWith("static ")).map((s) => `// ${s}`).join("\n")}

// —— 提取不变式断言 ——
const _: () = {
  // 虚方法 ${virtualCount} 条 + Instance/SetInstanceForTesting 2 静态
};
`;

const p = writeSrc("v8-diagnostics", "etw_isolate_operations_win.rs", rust);
console.log(`written ${p} (虚方法 ${virtualCount}, 静态 ${sigs.length - virtualCount})`);
