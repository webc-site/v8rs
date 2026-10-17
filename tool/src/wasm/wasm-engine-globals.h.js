#!/usr/bin/env bun
// tool/src/wasm/wasm-engine-globals.h.js
// 机械提取 v8/src/wasm/wasm-engine-globals.h 的进程级引擎访问函数骨架

import { readV8, writeSrc, banner } from "../../lib.js";

const hSrc = readV8("src/wasm/wasm-engine-globals.h");

const getters = [...hSrc.matchAll(/V8_EXPORT_PRIVATE\s+(\w+)\*\s+(Get\w+)\(\);/g)];
const EXPECTED = 5;
if (getters.length !== EXPECTED) {
  throw new Error(`引擎访问函数条数不符: ${getters.length}, 期望 ${EXPECTED}`);
}

console.log(`wasm-engine-globals.h: getters=${getters.length}`);

let rust = banner(
  "src/wasm/wasm-engine-globals.h.js",
  "src/wasm/wasm-engine-globals.h",
  "进程级 Wasm 引擎全局访问函数骨架 (签名骨架)",
);

rust += `
/// 进程共享引擎单例访问函数 (对应原版 wasm-engine-globals.h 的 5 个 Get*)
/// 返回类型以原版类名登记; 语义实现待 Developer 修订
`;

for (const [, cls, name] of getters) {
  rust += `/// 原版 ${cls}* ${name}()\npub fn ${name.replace(/^Get/, "get_").replace(/([a-z0-9])([A-Z])/g, "$1_$2").toLowerCase()}() -> *mut ${cls} {\n  unimplemented!("登记骨架: 见 v8/src/wasm/wasm-engine.cc")\n}\n\n`;
}

rust += `// —— 提取不变式断言 ——
const _: () = {
  // 5 个访问函数 1:1 登记
};
`;

const p = writeSrc("v8-wasm-core", "wasm_engine_globals.rs", rust);
console.log(`written ${p}`);
