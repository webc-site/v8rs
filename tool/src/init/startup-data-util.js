#!/usr/bin/env bun
// tool/src/init/startup-data-util.js
// 机械提取 v8/src/init/startup-data-util.{h,cc} 的外部快照装载签名与守卫分支

import { readV8, writeSrc, banner } from "../../lib.js";

const hSrc = readV8("src/init/startup-data-util.h");
const ccSrc = readV8("src/init/startup-data-util.cc");

// 1. 公开签名 (h): InitializeExternalStartupData / ...FromFile
const pubSigs = [...hSrc.matchAll(/^void\s+(\w+)\(([^)]*)\);/gm)].map(
  (m) => ({ name: m[1], args: m[2].replace(/\s+/g, " ").trim() }),
);
if (pubSigs.length !== 2) {
  throw new Error(`startup-data-util 公开签名数不符: 期望 2, 实得 ${pubSigs.length}`);
}
console.log(`公开签名 ${pubSigs.length} 条`);

// 2. cc 内部函数签名 (static 辅助)
const ccSigs = [...ccSrc.matchAll(/^(?:static\s+)?(?:void|bool|int|char.*?\*)\s+(\w+)\(/gm)].map((m) => m[1]);
if (ccSigs.length < 2) throw new Error(`cc 内部函数过少, 疑解析错位: ${ccSigs.length}`);

// 3. V8_USE_EXTERNAL_STARTUP_DATA 分支清点 (机械登记)
const extBranches = (ccSrc.match(/#ifdef V8_USE_EXTERNAL_STARTUP_DATA/g) || []).length;
if (extBranches !== 3) throw new Error(`外部快照分支数不符: 期望 3, 实得 ${extBranches}`);

let rust = banner(
  "src/init/startup-data-util.js",
  "src/init/startup-data-util.{h,cc}",
  "外部启动快照数据装载辅助",
);
rust += `
/// 独立二进制 (d8/cctest/unittest) 的外部启动数据装载便利层;
/// 嵌入方通常自管启动数据。整文件条件编译于 V8_USE_EXTERNAL_STARTUP_DATA。
/// 公开 API 签名骨架 (原版 startup-data-util.h):
`;
for (const s of pubSigs) {
  rust += `/// - \`void ${s.name}(${s.args})\`\n`;
}
rust += `
/// cc 内部辅助 (机械提取函数名): ${ccSigs.join(", ")}
/// 条件分支登记: #ifdef V8_USE_EXTERNAL_STARTUP_DATA x${extBranches};
/// 文件读取契约: fseek 定尾取 raw_size → fread 读入 → 不足即 Free 并置空。
`;

const p = writeSrc("v8-init", "startup_data_util.rs", rust);
console.log(`written ${p}`);
