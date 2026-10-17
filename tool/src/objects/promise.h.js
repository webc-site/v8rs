#!/usr/bin/env bun
// tool/src/objects/promise.h.js — 覆盖 v8/src/objects/promise.{h,-inl.h,tq}
// Promise 结构族: PromiseReaction Type 枚举 / tq 形态表 / ObjectTraits 偏移登记
// → v8-objects-js-async/src/promise.rs

import { readV8, writeSrc, banner } from "../../lib.js";

const hSrc = readV8("src/objects/promise.h");
const inlSrc = readV8("src/objects/promise-inl.h");
const tqSrc = readV8("src/objects/promise.tq");

// 1. PromiseReaction::Type 枚举
const typeBody = hSrc.match(/enum Type \{([^}]*)\};/);
if (!typeBody) throw new Error("未找到 PromiseReaction::Type 枚举");
const types = [...typeBody[1].matchAll(/k(\w+)/g)].map((m) => m[1]);
if (types.join(",") !== "Fulfill,Reject") throw new Error(`Type 锚点不符: ${types}`);

// 2. tq 形态表
const tqClasses = [...tqSrc.matchAll(/extern class (\w+) extends (\w+)/g)].map((m) => ({ name: m[1], base: m[2] }));
if (tqClasses.length !== 6) throw new Error(`tq 类数不符: ${tqClasses.length}, 期望 6`);
if (tqClasses[0].name !== "PromiseCapability" || tqClasses[1].name !== "PromiseReaction" || tqClasses[2].name !== "PromiseReactionJobTask")
  throw new Error("tq 类序锚点不符");

// 3. ObjectTraits 偏移登记 (h 侧 offsetof 项)
const traitFields = [...hSrc.matchAll(/static constexpr int k(\w+) =\s*\n?\s*offsetof\((\w+), (\w+_)\)/g)].map((m) => ({
  name: m[1], cls: m[2], field: m[3],
}));
if (traitFields.length !== 5) throw new Error(`traits 偏移项数异常: ${traitFields.length}, 期望 5`);

// 4. inl 访问器面对数
const inlCount = [...new Set([...inlSrc.matchAll(/Promise\w+::(\w+)\(/g)].map((m) => m[1]))];
if (inlCount.length < 10) throw new Error(`inl 访问器数异常: ${inlCount.length}`);

console.log(`Promise tq 类 ${tqClasses.length}, Type ${types.length}, traits ${traitFields.length}, inl 访问器 ${inlCount.length}`);

let rust = banner("src/objects/promise.h.js", "src/objects/promise.{h,-inl.h,tq}", "Promise 结构族");

rust += `
/// 反应类型 (原版 PromiseReaction::Type)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum PromiseReactionType {
  /// kFulfill
  Fulfill = 0,
  /// kReject
  Reject = 1,
}

/// Promise 结构形态表 (原版 tq extern class 继承序)
pub const PROMISE_CLASSES: [(&str, &str); ${tqClasses.length}] = [
${tqClasses.map((c) => `  ("${c.name}", "${c.base}"),`).join("\n")}
];

/// ObjectTraits 偏移登记 (数值依赖布局求解器, 由 Developer 接线)
/// (trait 项, 所属类, 字段名)
pub const TRAIT_OFFSETS: [(&str, &str, &str); ${traitFields.length}] = [
${traitFields.map((t) => `  ("${t.name}", "${t.cls}", "${t.field}"),`).join("\n")}
];

// —— 签名骨架 ——
// PromiseCapability: promise(JSReceiver|Undefined) / resolve(JSAny) / reject(JSAny)
// PromiseReaction: next / reject_handler / fulfill_handler / promise_or_capability
// PromiseReactionJobTask(@abstract): argument / context / handler / promise_or_capability
// PromiseFulfillReactionJobTask / PromiseRejectReactionJobTask: 空扩展
// PromiseResolveThenableJobTask: context / promise_to_resolve / thenable / then
// V8_ENABLE_CONTINUATION_PRESERVED_EMBEDDER_DATA 分支字段登记于 TRAIT_OFFSETS
pub const PROMISE_SIGNATURES: [&str; 6] = [
  "capability_promise_resolve_reject",
  "reaction_next_handlers",
  "reaction_job_task_argument_context_handler",
  "fulfill_reject_job_tasks",
  "resolve_thenable_job",
  "verify_printer",
];

// —— 提取不变式断言 ——
const _: () = {
  assert!(PROMISE_CLASSES.len() == 6);
  assert!(PROMISE_CLASSES[0].0 == "PromiseCapability");
  assert!(PROMISE_CLASSES[2].0 == "PromiseReactionJobTask");
  assert!(TRAIT_OFFSETS.len() == ${traitFields.length});
  assert!(PromiseReactionType::Fulfill as u8 == 0);
};
`;

const p = writeSrc("v8-objects-js-async", "promise.rs", rust);
console.log(`written ${p}`);
