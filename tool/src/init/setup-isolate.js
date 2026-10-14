#!/usr/bin/env bun
// tool/src/init/setup-isolate.js
// 机械提取 v8/src/init/setup-isolate.{h,-deserialize.cc,-full.cc} 的委托接口与两种链接实现差异

import { readV8, writeSrc, banner } from "../../lib.js";

const hSrc = readV8("src/init/setup-isolate.h");
const deserSrc = readV8("src/init/setup-isolate-deserialize.cc");
const fullSrc = readV8("src/init/setup-isolate-full.cc");

// 1. 提取 SetupIsolateDelegate 方法签名 (h 内 public + protected)
const classM = hSrc.match(/class V8_EXPORT_PRIVATE SetupIsolateDelegate \{([\s\S]*?)\n\};/);
if (!classM) throw new Error("未找到 SetupIsolateDelegate 类体");
const body = classM[1];

const sigRe = /(?:virtual\s+)?(?:static\s+)?(?:bool|void)\s+(\w+)\(([^)]*)\)/g;
const sigs = [];
let m;
while ((m = sigRe.exec(body)) !== null) {
  sigs.push({ name: m[1], args: m[2].replace(/\s+/g, " ").trim(), virtual: m[0].startsWith("virtual") });
}
if (sigs.length !== 7) {
  throw new Error(`SetupIsolateDelegate 签名数不符: 期望 7, 实得 ${sigs.length}: ${sigs.map((s) => s.name).join(",")}`);
}
console.log(`提取签名 ${sigs.length} 条`);

// 2. 两种链接实现的契约差异 (机械提取 CHECK/分支, 不造语义)
const deserChecks = [...deserSrc.matchAll(/CHECK_WITH_MSG\(!(\w+),\s*\n?\s*"([^"]+)"\)/g)].map(
  (x) => ({ flag: x[1], msg: x[2] }),
);
if (deserChecks.length !== 2) throw new Error(`deserialize 实现守卫数不符: 期望 2, 实得 ${deserChecks.length}`);

const fullBranches = [...fullSrc.matchAll(/(?:if \(!(\w+)\) \{|(\w+)\(isolate[^)]*\);)/g)]
  .map((x) => x[1] || x[2])
  .filter(Boolean);

let rust = banner(
  "src/init/setup-isolate.js",
  "src/init/setup-isolate.{h,-deserialize.cc,-full.cc}",
  "Isolate 初始化委托: 快照反序列化 / 全量构建双实现",
);
rust += `
/// SetupIsolateDelegate (原版 class SetupIsolateDelegate)
/// 抽象层: 组件 (builtins / 字节码 handler) 既可从快照反序列化, 也可从零构建。
/// 链接期二选一:
/// - setup-isolate-deserialize.cc: 只从快照装载
/// - setup-isolate-full.cc: 按标志从快照装载或现场构建 (mksnapshot 用)
pub trait SetupIsolateDelegate {
`;
for (const s of sigs.filter((x) => x.virtual)) {
  rust += `  /// 原版签名: ${s.virtual ? "virtual " : ""}${s.name}(${s.args})\n`;
}
rust += `  fn setup_heap(&mut self, isolate: &mut Isolate, create_heap_objects: bool) -> bool;
  fn setup_builtins(&mut self, isolate: &mut Isolate, compile_builtins: bool);
}
`;

// protected 静态方法骨架登记
for (const s of sigs.filter((x) => !x.virtual)) {
  rust += `
/// 原版 protected 静态签名: ${s.name}(${s.args}) — 由 Developer 落为关联函数`;
}
rust += `

/// deserialize 实现 (setup-isolate-deserialize.cc) 契约 — 机械提取 CHECK:
`;
for (const c of deserChecks) {
  rust += `/// - CHECK(!${c.flag}, "${c.msg}")\n`;
}
rust += `
/// full 实现 (setup-isolate-full.cc) 契约 — compile_builtins=false 时
/// CHECK(snapshot_available()); 否则走内部构建路径 (机械提取调用链): ${fullBranches.join(" → ")};
/// DEBUG 构建追加 DebugEvaluate::VerifyTransitiveBuiltins。
`;

const p = writeSrc("v8-init", "setup_isolate.rs", rust);
console.log(`written ${p}`);
