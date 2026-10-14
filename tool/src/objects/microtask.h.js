#!/usr/bin/env bun
// tool/src/objects/microtask.h.js — 覆盖 v8/src/objects/microtask.{h,-inl.h,tq}
// Microtask 族: 微任务抽象基与 CallbackTask/CallableTask/AsyncResumeTask 形态表
// → v8-objects-js-async/src/microtask.rs

import { readV8, writeSrc, banner } from "../../lib.js";

const hSrc = readV8("src/objects/microtask.h");
const inlSrc = readV8("src/objects/microtask-inl.h");
const tqSrc = readV8("src/objects/microtask.tq");

// 1. AsyncResumeTask::Kind 枚举 (含语义注释)
const kindBody = hSrc.match(/enum Kind \{([\s\S]*?)\n  \};/);
if (!kindBody) throw new Error("未找到 AsyncResumeTask::Kind 枚举");
const kinds = [...kindBody[1].matchAll(/k(\w+) = (\d+)/g)].map((m) => ({ name: m[1], repr: Number(m[2]) }));
if (kinds.length !== 2) throw new Error(`Kind 条数不符: ${kinds.length}, 期望 2`);
if (kinds[0].name !== "Yield" || kinds[0].repr !== 0) throw new Error("Kind 首项锚点不符");

// 2. 形态表 (tq extern class 序)
const tqClasses = [...tqSrc.matchAll(/extern class (\w+) extends (\w+)/g)].map((m) => ({ name: m[1], base: m[2] }));
if (tqClasses.length !== 4) throw new Error(`tq 类数不符: ${tqClasses.length}, 期望 4`);
if (tqClasses.map((c) => c.name).join(",") !== "Microtask,CallbackTask,CallableTask,AsyncResumeTask")
  throw new Error("tq 类序锚点不符");

// 3. ObjectTraits 偏移登记 (字段名, 数值依赖布局求解器, 不臆测)
const traitFields = [...hSrc.matchAll(/static constexpr int k(\w+) =\s*offsetof\((\w+), (\w+_)\);/g)].map((m) => ({
  name: m[1], cls: m[2], field: m[3],
}));
if (traitFields.length !== 4) throw new Error(`traits 偏移项数不符: ${traitFields.length}, 期望 4`);

// 4. inl 访问器面对数
const inlMethods = [...new Set([...inlSrc.matchAll(/::(set_\w+|\w+)\(\)/g)].map((m) => m[1]))];
if (inlMethods.length < 6) throw new Error(`inl 访问器数异常: ${inlMethods.length}`);

console.log(`Microtask tq 类 4, Kind ${kinds.length}, traits 偏移 ${traitFields.length}`);

let rust = banner("src/objects/microtask.h.js", "src/objects/microtask.{h,-inl.h,tq}", "Microtask 微任务族");

rust += `
/// 异步恢复任务类别 (原版 AsyncResumeTask::Kind)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum AsyncResumeKind {
  /// Async generator yield: resolve iterator result with done=false
  Yield = 0,
  /// Async function await: resume the generator with kNext
  AsyncFunctionAwait = 1,
}

/// 微任务形态表 (原版 tq extern class 继承序)
pub const MICROTASK_CLASSES: [(&str, &str); ${tqClasses.length}] = [
${tqClasses.map((c) => `  ("${c.name}", "${c.base}"),`).join("\n")}
];

/// ObjectTraits 偏移登记 (数值依赖布局求解器, 由 Developer 接线)
/// (trait 项, 所属类, 字段名)
pub const TRAIT_OFFSETS: [(&str, &str, &str); ${traitFields.length}] = [
${traitFields.map((t) => `  ("${t.name}", "${t.cls}", "${t.field}"),`).join("\n")}
];

/// 微任务抽象基标记 (V8_IT 标注类)
pub struct Microtask;

// —— 签名骨架 ——
// CallbackTask: callback(Foreign) + data(Object)
// CallableTask: callable(JSReceiver) + context(NativeContext)
// AsyncResumeTask: generator(JSGeneratorObject) + value(Object) + kind(Smi)
// 访问器: generator/value/kind 及各自 setter (写屏障), DECL_VERIFIER/PRINTER
// V8_ENABLE_CONTINUATION_PRESERVED_EMBEDDER_DATA 分支: continuation_preserved_embedder_data 偏移
pub const MICROTASK_SIGNATURES: [&str; 8] = [
  "callback_task_callback", "callback_task_data",
  "callable_task_callable", "callable_task_context",
  "async_resume_generator", "async_resume_value", "async_resume_kind",
  "verify_printer",
];

// —— 提取不变式断言 ——
const _: () = {
  assert!(MICROTASK_CLASSES.len() == 4);
  assert!(MICROTASK_CLASSES[3].0 == "AsyncResumeTask");
  assert!(TRAIT_OFFSETS.len() == 4);
  assert!(AsyncResumeKind::Yield as u8 == 0);
  assert!(AsyncResumeKind::AsyncFunctionAwait as u8 == 1);
};
`;

const p = writeSrc("v8-objects-js-async", "microtask.rs", rust);
console.log(`written ${p}`);
