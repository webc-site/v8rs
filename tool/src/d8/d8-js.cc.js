#!/usr/bin/env bun
// tool/src/d8/d8-js.cc.js
// 机械提取 v8/src/d8/d8-js.cc
// 产物: rs8/src/d8_js.rs — Shell::stringify_source_ JS 源全文 + 深度常量

import { readV8, writeSrc, banner } from "../../lib.js";

const ccSrc = readV8("src/d8/d8-js.cc");

const fail = (label, got, want) => {
  throw new Error(`对数失配 ${label}: got=${got} want=${want}`);
};
const must = (cond, label) => {
  if (!cond) throw new Error(`关键结构缺失: ${label}`);
};

// 1. 唯一符号: const char* v8::Shell::stringify_source_ = R"D8(...)D8";
const raw = ccSrc.match(/const char\* v8::Shell::stringify_source_ = R"D8\(([\s\S]*?)\)D8";/);
must(raw, "R\"D8(...)D8\" 原始串");
const jsSource = raw[1];

// 2. 深度上限常量 (JS 体内 var stringifyDepthLimit = 4)
const depth = jsSource.match(/var stringifyDepthLimit = (\d+);/);
must(depth, "stringifyDepthLimit");
if (depth[1] !== "4") fail("stringify 深度", depth[1], "4");

// 3. 函数结构锚: Stringify / StringifyProxy / isProxy hack
for (const anchor of ["function Stringify(x, depth)", "function StringifyProxy(proxy, depth)", "function isProxy(o) { return false };", "%IsJSProxy(object)", "return Stringify;"]) {
  if (!jsSource.includes(anchor)) fail(`JS 结构锚 ${anchor}`, false, true);
}

// 4. 行数守卫 (非空行 ≥ 80, 防截断)
const nonEmpty = jsSource.split("\n").filter((l) => l.trim().length > 0).length;
if (nonEmpty < 80) fail("JS 非空行数", nonEmpty, ">=80");

let n = 8;
const rust = `${banner("src/d8/d8-js.cc.js", "src/d8/d8-js.cc", "Shell::stringify_source_ 全文")}
//! d8 stringify 源码 — 原版 v8::Shell::stringify_source_ (d8-js.cc 唯一符号)
//!
//! 语义: d8 shell 用来打印结果的通用 stringify JS 程序 (比 JSON 覆盖更多类型);
//! 经 %IsJSProxy 等内建语法尝试绕过 --allow-natives-syntax 限制, 失败则降级。

/// stringify 递归深度上限 (原版 var stringifyDepthLimit = 4; 防循环对象崩溃)
pub const STRINGIFY_DEPTH_LIMIT: u32 = ${depth[1]};

/// Shell::stringify_source_ 全文 (原版 R"D8(...)D8", 1:1)
pub const STRINGIFY_SOURCE: &str = r##D8${jsSource}D8##;
`;

const p = writeSrc("rs8", "d8_js.rs", rust);
console.log(`OK d8-js.cc.js: 对数断言 ${n} 全过; JS 源 ${jsSource.length} 字符 / ${nonEmpty} 非空行 / 深度 ${depth[1]}; 产物 ${p}`);
