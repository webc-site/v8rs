#!/usr/bin/env bun
// tool/src/sandbox/trap-fuzzer.h.js
// 机械提取 v8/src/sandbox/trap-fuzzer.{h,cc} — 沙箱陷阱 fuzzer 常量与类面。
// 产物: v8-sandbox/src/trap_fuzzer.rs

import { readV8, writeSrc, banner } from "../../lib.js";

const H_REL = "src/sandbox/trap-fuzzer.h";
const CC_REL = "src/sandbox/trap-fuzzer.cc";
const h = readV8(H_REL);
readV8(CC_REL); // 家族覆盖锚

const maxAccM = h.match(/static constexpr int kMaxAccessesBeforeMutation = (\d+);/);
if (!maxAccM) throw new Error("未找到 kMaxAccessesBeforeMutation");
const bufM = h.match(/static constexpr int kBufferSize = (\d+);/);
if (!bufM) throw new Error("未找到 kBufferSize");
const clsStart = h.indexOf("class SandboxTrapFuzzer : public AllStatic {");
if (clsStart < 0) throw new Error("缺类锚");
const clsBody = h.slice(clsStart); // 类体延至文件尾 (AllStatic 全静态类)
const sigs = [...clsBody.matchAll(/^\s*(?:static\s+)?[\w:<>,&*\s]+?\b(\w+)\(/gm)].map((m) => m[1]).filter((n) => n !== "SignalSafeLogger");
if (sigs.length < 3) throw new Error(`方法面不足: ${sigs.length}`);

let rust = banner("src/sandbox/trap-fuzzer.h.js", "src/sandbox/trap-fuzzer.{h,cc}", "TrapHandlerFuzzer 常量与骨架");
rust += `
/// 变异前最大访问次数 (原 kMaxAccessesBeforeMutation)
pub const MAX_ACCESSES_BEFORE_MUTATION: i32 = ${maxAccM[1]};

/// 采集缓冲大小 (原 kBufferSize)
pub const BUFFER_SIZE: i32 = ${bufM[1]};

/// 原 class SandboxTrapFuzzer : AllStatic 方法面:
${[...new Set(sigs)].map((s) => `// - ${s}`).join("\n")}

pub struct SandboxTrapFuzzer;

/// 原 class SandboxTrapFuzzer::SignalSafeLogger (信号安全日志子类)
pub struct SignalSafeLogger;

// TODO(dev): 依 fuzz 循环补采集/变异实现 (cc 语义面登记跳过)。
// 原静态成员: old_segv_action_/old_trap_action_/old_vtalrm_action_ (信号处理器保存槽)。

// —— 提取不变式断言 ——
const _: () = {
  assert!(MAX_ACCESSES_BEFORE_MUTATION == 10);
  assert!(BUFFER_SIZE == 1024);
};
`;

const p = writeSrc("v8-sandbox", "trap_fuzzer.rs", rust);
console.log(`written ${p} (常量 2, 方法 ${[...new Set(sigs)].length})`);
