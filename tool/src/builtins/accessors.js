#!/usr/bin/env bun
// tool/src/builtins/accessors.js — 机械提取 v8/src/builtins/accessors.{h,cc} 的 X-macro accessor 表
// 同名族合并脚本; 产物为规则代码 (X-macro 展开表 + COUNT 常量 + 方法登记), 语义实现由 Developer 修订环补齐。
// 任务映射: v8-builtins-misc (聚合基线脚本将其归于 v8-builtins-proxy-reflect/reflect_access, 两处并存不覆盖)。
import { spawnSync } from "node:child_process";
import { join } from "node:path";
import { readV8, writeSrc, banner, root } from "../../lib.js";

// rustfmt 归一 (与聚合基线脚本一致)
function fmt(body) {
  const r = spawnSync("rustfmt", ["--edition", "2024", "--config-path", join(root, "rustfmt.toml")], { input: body, encoding: "utf8" });
  if (r.status !== 0) throw new Error("rustfmt 归一失败: " + r.stderr);
  return r.stdout;
}

const H = "src/builtins/accessors.h";
const CC = "src/builtins/accessors.cc";
// 对数基准 (生成时核定): X-macro 表条数与 cc 侧方法出现次数
const EXPECT = { info: 13, getter: 0, setter: 4, callback: 2, methods: 39 };

function macroBody(src, name) {
  const lines = src.split("\n");
  const out = [];
  let on = false;
  for (const l of lines) {
    if (l.startsWith("#define " + name)) { on = true; out.push(l); continue; }
    if (on) {
      out.push(l);
      if (!l.trimEnd().endsWith("\\")) break;
    }
  }
  if (out.length === 0) throw new Error(H + " 未找到宏 " + name);
  return out.join("\n");
}

// —— 提取与对数守卫 ——
const h = readV8(H);
const cc = readV8(CC);

const infoMacro = macroBody(h, "ACCESSOR_INFO_LIST_GENERATOR");
const infoList = [...infoMacro.matchAll(/V\(_,\s*(\w+),\s*(\w+)/g)].map((m) => ({ lower: m[1], pascal: m[2] }));
if (infoList.length !== EXPECT.info) throw new Error("应 " + EXPECT.info + " 个 accessor info 实际 " + infoList.length);

const getterMacro = macroBody(h, "ACCESSOR_GETTER_LIST");
const getterList = [...getterMacro.matchAll(/V\(\s*(\w+)\s*\)/g)].map((m) => m[1]);
if (getterList.length !== EXPECT.getter) throw new Error("应 " + EXPECT.getter + " 个 accessor getter 实际 " + getterList.length);

const setterMacro = macroBody(h, "ACCESSOR_SETTER_LIST");
const setterList = [...setterMacro.matchAll(/V\(\s*(\w+)\s*\)/g)].map((m) => m[1]);
if (setterList.length !== EXPECT.setter) throw new Error("应 " + EXPECT.setter + " 个 accessor setter 实际 " + setterList.length);

const callbackMacro = macroBody(h, "ACCESSOR_CALLBACK_LIST_GENERATOR");
const callbackList = [...callbackMacro.matchAll(/V\(_,\s*(\w+)/g)].map((m) => m[1]);
if (callbackList.length !== EXPECT.callback) throw new Error("应 " + EXPECT.callback + " 个 accessor callback 实际 " + callbackList.length);

// cc 侧 Accessors::<方法>( 登记出现次数 (同名方法在 cc 内多次出现时按出现次数记)
const methods = [...cc.matchAll(/^[\w:<>,*&\s]*?\bAccessors::(\w+)\s*\(/gm)].map((m) => ({ name: m[1], line: cc.slice(0, m.index).split("\n").length }));
if (methods.length !== EXPECT.methods) throw new Error("应 " + EXPECT.methods + " 个 Accessors:: 方法出现 实际 " + methods.length);

// —— Rust 产物 ——
let rust = banner("src/builtins/accessors.js", "src/builtins/accessors.{h,cc}", "X-macro accessor 表机械展开 + 方法登记, 函数体不展开");
rust += "//! 同名族合并脚本 (accessors.h + accessors.cc)\n";
rust += "//! 任务映射: v8-builtins-misc; 聚合基线另归 v8-builtins-proxy-reflect (并存不覆盖)\n\n";

rust += "/// 预定义 accessor info 名单 — X-macro ACCESSOR_INFO_LIST_GENERATOR 展开 (V(_, name, Pascal, 效应标志…))\n";
rust += "pub const ACCESSOR_INFO_LIST: &[(&str, &str)] = &[\n";
for (const a of infoList) rust += `  ("${a.lower}", "${a.pascal}"),\n`;
rust += "];\n\n";

rust += "/// 独立 getter 名单 — X-macro ACCESSOR_GETTER_LIST 展开 (当前为空)\n";
rust += `pub const ACCESSOR_GETTER_LIST: &[&str] = &[${getterList.map((n) => `"${n}"`).join(", ")}];\n\n`;

rust += "/// 独立 setter 名单 — X-macro ACCESSOR_SETTER_LIST 展开\n";
rust += "pub const ACCESSOR_SETTER_LIST: &[&str] = &[\n";
for (const n of setterList) rust += `  "${n}",\n`;
rust += "];\n\n";

rust += "/// callback 名单 — X-macro ACCESSOR_CALLBACK_LIST_GENERATOR 展开\n";
rust += "pub const ACCESSOR_CALLBACK_LIST: &[&str] = &[\n";
for (const n of callbackList) rust += `  "${n}",\n`;
rust += "];\n\n";

// X-macro COUNT_ACCESSOR 展开: 0 + 每条 +1
rust += "/// 原版 static constexpr 计数 — X-macro COUNT_ACCESSOR(__VA_ARGS__) +1 机械展开\n";
rust += `pub const K_ACCESSOR_INFO_COUNT: i32 = ${infoList.length};\n`;
rust += `pub const K_ACCESSOR_GETTER_COUNT: i32 = ${getterList.length};\n`;
rust += `pub const K_ACCESSOR_SETTER_COUNT: i32 = ${setterList.length};\n`;
rust += `pub const K_ACCESSOR_CALLBACK_COUNT: i32 = ${callbackList.length};\n\n`;

rust += "/// accessors.cc 侧 Accessors::<方法>( 定义点登记 (名, 原版行号; 同名多次定义按出现次数记)\n";
rust += "pub const CC_METHODS: &[(&str, u32)] = &[\n";
for (const m of methods) rust += `  ("${m.name}", ${m.line}),\n`;
rust += "];\n\n";

rust += "// —— 对数断言 ——\nconst _: () = {\n";
rust += `  assert!(ACCESSOR_INFO_LIST.len() == ${infoList.length});\n`;
rust += `  assert!(ACCESSOR_GETTER_LIST.len() == ${getterList.length});\n`;
rust += `  assert!(ACCESSOR_SETTER_LIST.len() == ${setterList.length});\n`;
rust += `  assert!(ACCESSOR_CALLBACK_LIST.len() == ${callbackList.length});\n`;
rust += "  assert!(K_ACCESSOR_INFO_COUNT as usize == ACCESSOR_INFO_LIST.len());\n";
rust += "  assert!(K_ACCESSOR_SETTER_COUNT as usize == ACCESSOR_SETTER_LIST.len());\n";
rust += "  assert!(K_ACCESSOR_CALLBACK_COUNT as usize == ACCESSOR_CALLBACK_LIST.len());\n";
rust += `  assert!(CC_METHODS.len() == ${methods.length});\n`;
rust += "};\n";

const p = writeSrc("v8-builtins-misc", "accessors.rs", fmt(rust));
console.log(`[accessors.{h,cc}] info=${infoList.length} getter=${getterList.length} setter=${setterList.length} callback=${callbackList.length} methods=${methods.length} -> ${p}`);
