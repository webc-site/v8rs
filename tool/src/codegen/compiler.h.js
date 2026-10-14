#!/usr/bin/env bun
// tool/src/codegen/compiler.h.js
// 机械提取 v8/src/codegen/compiler.{h,cc} —
// Compiler 命名空间静态接口 + CompilationJob 状态机 + 每层作业类骨架。
// cc 4772 行管线为语义面, 登记跳过 (规则 10)。
// 产物: v8-codegen-core/src/compiler_generated.rs

import { readV8, writeSrc, banner } from "../../lib.js";

const H_REL = "src/codegen/compiler.h";
const CC_REL = "src/codegen/compiler.cc";
const h = readV8(H_REL);
readV8(CC_REL); // 家族覆盖锚

// 1. Compiler 静态接口 (class V8_EXPORT_PRIVATE Compiler : public AllStatic)
const clsM = h.match(/class V8_EXPORT_PRIVATE Compiler : public AllStatic\s*\{([\s\S]*?)\n\};/);
if (!clsM) throw new Error("未找到 Compiler 类");
const fns = [...clsM[1].matchAll(/^\s*(?:V8_WARN_UNUSED_RESULT\s+)?static\s+[\w:<>,&*\s]+?\b(\w+)\(/gm)].map((m) => m[1]);
if (fns.length < 10) throw new Error(`Compiler 静态接口不足: ${fns.length}`);
if (!clsM[1].includes("enum ClearExceptionFlag { KEEP_EXCEPTION, CLEAR_EXCEPTION };")) throw new Error("缺 ClearExceptionFlag 锚");

// 2. CompilationJob 状态机
const stM = h.match(/enum class State\s*\{([^}]+)\};/);
if (!stM) throw new Error("未找到 State 枚举");
const states = [...stM[1].matchAll(/k(\w+)/g)].map((m) => "k" + m[1]);
if (states.length !== 5) throw new Error(`State 条数不符: ${states.length} != 5`);
const statusM = h.match(/enum Status\s*\{([^}]+)\};/);
if (!statusM) throw new Error("未找到 Status 枚举");
const statuses = [...statusM[1].matchAll(/([A-Z_]+)[,\s]/g)].map((m) => m[1]);
if (statuses.join() !== "SUCCEEDED,FAILED,RETRY_ON_MAIN_THREAD") throw new Error(`Status 条目异常: ${statuses.join()}`);

// 3. 派生作业类登记
const jobs = [...h.matchAll(/class (\w*(?:Turbofan|Maglev|Baseline)\w*CompilationJob)\b/g)].map((m) => m[1]);
if (jobs.length < 3) throw new Error(`作业类不足: ${jobs.length}`);

let rust = banner("src/codegen/compiler.h.js", "src/codegen/compiler.{h,cc}", "编译器接口与作业状态机骨架");
rust += `
// 语义面: 编译管线 (cc ${readV8(CC_REL).split("\n").length} 行) 登记跳过。

/// 异常清除策略 (原 enum ClearExceptionFlag)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum ClearExceptionFlag {
  KeepException = 0,
  ClearException = 1,
}

/// 作业状态机 (原 enum class State)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u8)]
pub enum State {
${states.map((s, i) => `  /// k${s}\n  ${s} = ${i},`).join("\n")}
}

/// 作业结果 (原 enum Status)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Status {
  Succeeded = 0,
  Failed = 1,
  RetryOnMainThread = 2,
}

/// 原 class Compiler : AllStatic 静态接口 (${[...new Set(fns)].length} 条):
${[...new Set(fns)].map((s) => `// - ${s}`).join("\n")}

/// 派生作业类登记:
${[...new Set(jobs)].map((s) => `// - class ${s}`).join("\n")}

/// 原 class CompilationJob 骨架
pub struct CompilationJob;

// TODO(dev): 补 Prepare/Execute/Finalize 三相与状态迁移。

// —— 提取不变式断言 ——
const _: () = {
  assert!(State::ReadyToPrepare as u8 == 0);
  assert!(Status::RetryOnMainThread as u8 == 2);
};
`;

const p = writeSrc("v8-codegen-core", "compiler_generated.rs", rust);
console.log(`written ${p} (接口 ${[...new Set(fns)].length}, 状态 ${states.length}, 作业 ${[...new Set(jobs)].length})`);
