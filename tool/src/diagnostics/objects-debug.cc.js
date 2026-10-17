#!/usr/bin/env bun
// tool/src/diagnostics/objects-debug.cc.js
// 机械提取 v8/src/diagnostics/objects-debug.cc — 对象 Verify 校验器注册面。
// 4291 行纯语义校验逻辑不属规则代码; 本脚本登记全量 <Class>::<Class>Verify
// 方法名注册表 (按源码序去重) + 关键展开宏锚, 供 Developer 按 registrations 补骨。
// 产物: v8-diagnostics/src/objects_debug_generated.rs

import { readV8, writeSrc, banner } from "../../lib.js";

const CC_REL = "src/diagnostics/objects-debug.cc";
const cc = readV8(CC_REL);

// 全量 Verify 方法定义: <Class>::<Class>Verify(
const defs = [...cc.matchAll(/^void (\w+)::(\w+Verify)\(/gm)].map((m) => ({ cls: m[1], method: m[2] }));
if (defs.length < 200) throw new Error(`Verify 定义条数异常: ${defs.length} (<200)`);

// 按源码序去重 (同一方法可能因 #if 分支多份定义)
const seen = new Set();
const registry = [];
for (const d of defs) {
  const key = `${d.cls}::${d.method}`;
  if (!seen.has(key)) {
    seen.add(key);
    registry.push(d);
  }
}
if (registry.length < 150) throw new Error(`去重后注册表条数异常: ${registry.length}`);

// 宏锚 (展开面登记)
for (const anchor of [
  "#define STRING_TYPE_CASE(TYPE, size, name, CamelName) case TYPE:",
  "#define MAKE_VERIFY_CASE(Name, TYPE)",
  "#define TYPED_ARRAY_CONSTRUCTORS_SWITCH(Type, type, TYPE, Ctype)",
]) {
  if (!cc.includes(anchor.slice(0, 40))) throw new Error(`缺宏锚: ${anchor.slice(0, 40)}`);
}

let rust = banner("src/diagnostics/objects-debug.cc.js", CC_REL, "对象 Verify 校验器注册表");
rust += `
// 语义面: 各 Verify 方法体 (~4300 行不变式检查) 登记跳过, 由 Developer 修订补齐。
// 原版关键展开宏: STRING_TYPE_CASE / MAKE_VERIFY_CASE / TYPED_ARRAY_CONSTRUCTORS_SWITCH。

/// 全量 <Class>::<Class>Verify 注册表 (源码序去重), 元组为 (类名, 方法名)
pub static VERIFY_REGISTRY: &[(&str, &str)] = &[
${registry.map((d) => `  ("${d.cls}", "${d.method}"),`).join("\n")}
];

/// 注册表条数
pub const VERIFY_REGISTRY_LEN: usize = ${registry.length};

// —— 提取不变式断言 ——
const _: () = {
  assert!(VERIFY_REGISTRY.len() == VERIFY_REGISTRY_LEN);
  assert!(VERIFY_REGISTRY[0].0.len() == 6);
};
`;

const p = writeSrc("v8-diagnostics", "objects_debug_generated.rs", rust);
console.log(`written ${p} (注册表 ${registry.length} 条, 原始定义 ${defs.length})`);
