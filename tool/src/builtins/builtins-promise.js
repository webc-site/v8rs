#!/usr/bin/env bun
// tool/src/builtins/builtins-promise.js — v8-builtins-promise 基线生成
// 来源: promise*.tq(13) + async/generator/iterator/microtask .cc/.h/.tq + weak/finalization
// → v8-builtins-promise/src/{promise, async_gen, iterator}.rs
// (重建版: 原文件被同名族脚本误覆盖, 依审计 files/anchors 清单与产物 mod 结构还原)
import { readFileSync, writeFileSync } from "node:fs";
import { join } from "node:path";
import { root } from "../../lib.js";
import { fmt, groupRun } from "./builtins-lib.js";

const crate = "v8-builtins-promise";
const groups = [
  { mod: "promise", files: { cc: ["builtins-promise-gen.cc"], h: ["builtins-promise-gen.h", "builtins-promise.h"], tq: ["promise-abstract-operations.tq", "promise-all.tq", "promise-all-element-closure.tq", "promise-any.tq", "promise-constructor.tq", "promise-finally.tq", "promise-jobs.tq", "promise-misc.tq", "promise-race.tq", "promise-reaction-job.tq", "promise-resolve.tq", "promise-then.tq", "promise-try.tq", "promise-withresolvers.tq", "aggregate-error.tq", "suppressed-error.tq", "weak-ref.tq", "finalization-registry.tq"] }, anchors: [["PromiseBuiltinsAssembler::AllocateJSPromise", "builtins-promise-gen.cc"]] },
  { mod: "async_gen", files: { cc: ["builtins-async-gen.cc", "builtins-async-function-gen.cc", "builtins-async-generator-gen.cc", "builtins-async-iterator-gen.cc", "builtins-async-module.cc", "builtins-async-disposable-stack.cc", "builtins-generator-gen.cc", "builtins-microtask-queue-gen.cc", "builtins-disposable-stack.cc"], h: ["builtins-async-gen.h"], tq: ["generator.tq", "array-from-async.tq"] }, anchors: [["Async", "builtins-async-gen.cc"]] },
  { mod: "iterator", files: { cc: ["builtins-iterator-gen.cc"], h: ["builtins-iterator-gen.h", "builtins-iterator.h", "builtins-iterator-inl.h"], tq: ["iterator.tq", "iterator-helpers.tq", "iterator-from.tq"] }, anchors: [["Iterator", "builtins-iterator-gen.cc"]] },
];

groupRun("builtins-promise.js", crate, groups);

// 清障 (agent-allow-a): 承接面全量挂 pub mod — 骨架 pub 项经公开路径可达, 即非死代码;
// 去 #[allow(dead_code)] (禁 allow 绕过), lib.rs 不再 Builtin 单项二次导出
for (const g of groups) {
  const p = join(root, crate, "src", `${g.mod}.rs`);
  writeFileSync(p, readFileSync(p, "utf8").replaceAll("#[allow(dead_code)]\n", ""));
}
const mods = groups.map((g) => g.mod);
const libBody = [
  `// 由 tool/src/builtins/builtins-promise.js 生成; 来源 v8/src/builtins 下 ${mods.length} 文件`,
  "// 模块索引",
  "// 修订就地改, 复跑覆盖; 只含规则代码, 不含语义",
  "",
  mods.map((m) => `pub mod ${m};`).join("\n"),
  "mod error;",
  "pub use error::{Error, Result};",
  "",
].join("\n");
writeFileSync(join(root, crate, "src", "lib.rs"), fmt(libBody));
