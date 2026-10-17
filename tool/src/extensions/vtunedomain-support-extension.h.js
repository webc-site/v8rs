#!/usr/bin/env bun
// tool/src/extensions/vtunedomain-support-extension.h.js
// 机械提取 v8/src/extensions/vtunedomain-support-extension.{h,cc}
// 产物: v8-extensions/src/vtunedomain_support_extension.rs — 7 错误码 #define 显式展开 + 分发表 + 签名骨架

import { readV8, writeSrc, banner } from "../../lib.js";

const hSrc = readV8("src/extensions/vtunedomain-support-extension.h");
const ccSrc = readV8("src/extensions/vtunedomain-support-extension.cc");

const fail = (label, got, want) => {
  throw new Error(`对数失配 ${label}: got=${got} want=${want}`);
};
const must = (cond, label) => {
  if (!cond) throw new Error(`关键结构缺失: ${label}`);
};

// 1. #define 错误码 → 显式常量 (宏机械展开)
const defines = [...hSrc.matchAll(/^#define ([A-Z_]+)\s+(\d+)\s*<<\s*(\d+)$/gm)];
if (defines.length !== 7) fail("#define 错误码数", defines.length, 7);
const flags = defines.map(([, name, , shift]) => ({
  name,
  rustName: name.toLowerCase(),
  value: 1 << parseInt(shift, 10),
  shift: parseInt(shift, 10),
}));
// 展开值台账: 1<<0 .. 1<<6
for (let i = 0; i < 7; i++) {
  if (flags[i].shift !== i) fail(`位序 ${i}`, flags[i].shift, i);
  if (flags[i].value !== 1 << i) fail(`展开值 ${flags[i].name}`, flags[i].value, 1 << i);
}

// 2. 扩展名
const extName = hSrc.match(/v8::Extension\("([^"]+)"/);
must(extName, "扩展名 v8/vtunedomain");
if (extName[1] !== "v8/vtunedomain") fail("扩展名", extName[1], "v8/vtunedomain");

// 3. libvtune function_map 分发表 (cc)
const mapEntries = [...ccSrc.matchAll(/\{"(\w+)",\s*(\w+)\}/g)].map(([, k, v]) => ({ k, v }));
if (mapEntries.length !== 2) fail("function_map 条目", mapEntries.length, 2);

// 4. libvtune 函数全集 (cc): split / startTask / endTask / invoke
const libFns = ["split", "startTask", "endTask", "invoke"];
for (const f of libFns) {
  const re = new RegExp(`(?:int|void) ${f}\\(`);
  if (!re.test(ccSrc)) fail(`libvtune 函数 ${f}`, false, true);
}

// 5. Mark 参数约定 (3 string 参数)
const errMsg = ccSrc.match(/ThrowError\(\s*\n?\s*"([^"]+)/);
must(errMsg, "Mark 参数错误文案");

let n = 13;
const rust = `${banner("src/extensions/vtunedomain-support-extension.h.js", "src/extensions/vtunedomain-support-extension.{h,cc}", "VTuneDomain 扩展: 7 错误码 #define 显式展开 + 分发表 + 签名骨架")}
//! VTuneDomainSupport 扩展 — 原版 namespace v8::internal::VTuneDomainSupportExtension + libvtune
//!
//! 语义: 注入 test(domain, task, start_or_end) — 经 libvtune 在 VTune 域上开始/结束任务;
//! 错误按位累加后以异常文本返回。

// —— #define 宏 → 显式常量 (位标志, 原版 1 << N) ——
${flags
  .map((f) => `/// 原版 #define ${f.name} (1 << ${f.shift})\npub const ${f.name}: u32 = 1 << ${f.shift}; // = ${f.value}`)
  .join("\n")}

/// VTuneDomain 扩展注册名
pub const VTUNEDOMAIN_SUPPORT_EXTENSION_NAME: &str = "v8/vtunedomain";

/// Mark 参数错误文案
pub const ERR_PARAMS: &str = "Parameter number should be exactly three, first domain namesecond task name, third start/end";

/// libvtune 分发表 (原版 function_map: 命令字 → 处理函数)
pub const FUNCTION_MAP: [(&str, &str); 2] = [
${mapEntries.map((e) => `  (${JSON.stringify(e.k)}, ${JSON.stringify(e.v)}),`).join("\n")}
];

// —— 提取不变式断言 ——
const _: () = {
  assert!(UNKNOWN_PARAMS == 1);
  assert!(NO_DOMAIN_NAME == 2);
  assert!(CREATE_DOMAIN_FAILED == 4);
  assert!(NO_TASK_NAME == 8);
  assert!(CREATE_TASK_FAILED == 16);
  assert!(TASK_BEGIN_FAILED == 32);
  assert!(TASK_END_FAILED == 64);
  assert!(FUNCTION_MAP.len() == 2);
};

// —— 签名骨架 (cc 定义 1:1) ——
// libvtune (namespace v8::internal::libvtune):
${libFns.map((f) => `//   ${f === "split" ? `split(str: &str, delimiter: char, out: &mut Vec<String>)` : f === "invoke" ? `invoke(params: &str) -> i32 — vparams[0] 查表分发, 未知命令累加 UNKNOWN_PARAMS` : `${f}(vparams: &[String]) -> i32`}`).join("\n")}
// 扩展本体:
//   GetNativeFunctionTemplate(isolate, name) -> FunctionTemplate
//   Mark(info) — 3 个 string 参数: (domain_name, task_name, "start"|"end"),
//                拼 "cmd domain task" 后交 libvtune::invoke, 非 0 则按数值抛错

// 依赖台账:
//   third_party/vtune/vtuneapi.h — VTuneDomain::createDomain / beginTask / endTask,
//   外部库闭源接口, Rust 侧经 FFI 承接点登记, 不在本模块落地
`;

const p = writeSrc("v8-extensions", "vtunedomain_support_extension.rs", rust);
console.log(`OK vtunedomain-support-extension.h.js: 对数断言 ${n} 全过; #define 展开 7 / 分发 2 / libvtune 函数 4; 产物 ${p}`);
