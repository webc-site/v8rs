#!/usr/bin/env bun
// tool/src/init/icu_util.js
// 机械提取 v8/src/init/icu_util.{h,cc} 的 ICU 数据模式宏与初始化签名

import { readV8, writeSrc, banner } from "../../lib.js";

const hSrc = readV8("src/init/icu_util.h");
const ccSrc = readV8("src/init/icu_util.cc");

// 1. 宏机械展开: ICU_UTIL_DATA_FILE / ICU_UTIL_DATA_STATIC (cc 内 #define)
const fileM = ccSrc.match(/#define ICU_UTIL_DATA_FILE (\d+)/);
const staticM = ccSrc.match(/#define ICU_UTIL_DATA_STATIC (\d+)/);
if (!fileM || !staticM) throw new Error("未找到 ICU_UTIL_DATA_* 宏定义");
const dataFile = parseInt(fileM[1], 10);
const dataStatic = parseInt(staticM[1], 10);

// 2. ICU_UTIL_DATA_IMPL 构建选择: cc 顶部按构建宏分支; 目标配置 (x64+arm64,
// V8_INTL_SUPPORT) 走 file 模式, 记录选择与替代分支
const implZone = ccSrc.slice(0, ccSrc.indexOf("namespace v8 {"));
const implM = implZone.match(/#define ICU_UTIL_DATA_IMPL (\d+)/);
const implValue = implM ? parseInt(implM[1], 10) : dataFile; // 未显式定义时按构建默认 file
console.log(`ICU_UTIL_DATA_IMPL 展开为 ${implValue} (file=${dataFile}, static=${dataStatic})`);

// 3. 头文件公开签名: InitializeICU / InitializeICUDefaultLocation
const pubSigs = [...hSrc.matchAll(/V8_WARN_UNUSED_RESULT\s+(bool)\s+(\w+)\(([^)]*)\)/gs)].map(
  (m) => ({ ret: m[1], name: m[2], args: m[3].replace(/\s+/g, " ").trim() }),
);
if (pubSigs.length !== 2) {
  throw new Error(`icu_util 公开签名数不符: 期望 2, 实得 ${pubSigs.length}`);
}
console.log(`提取公开签名 ${pubSigs.length} 条: ${pubSigs.map((s) => s.name).join(", ")}`);

// 4. cc 内 #if 平台分支登记 (机械清点, 不造语义)
const branches = [...ccSrc.matchAll(/^#(if|ifdef|elif)\b(.*)$/gm)].map((m) =>
  m[2].trim(),
);
if (branches.length < 5) throw new Error(`条件分支过少, 疑解析错位: ${branches.length}`);

let rust = banner("src/init/icu_util.js", "src/init/icu_util.{h,cc}", "ICU 数据表初始化");
rust += `
/// ICU 快照数据装载模式 (原版 cc 内宏机械展开)
pub const ICU_UTIL_DATA_FILE: i32 = ${dataFile};
pub const ICU_UTIL_DATA_STATIC: i32 = ${dataStatic};

/// ICU_UTIL_DATA_IMPL 构建期选择: 目标配置 (V8_INTL_SUPPORT + 文件模式) 展开为
/// ICU_UTIL_DATA_FILE(${dataFile}); 静态链接模式取 ICU_UTIL_DATA_STATIC(${dataStatic}),
/// 本 crate 按 file 模式展开, 其他配置跳过并在此登记。
pub const ICU_UTIL_DATA_IMPL: i32 = ${implValue};

/// 公开 API 签名骨架 (原版 icu_util.h):
`;
for (const s of pubSigs) {
  rust += `/// - \`${s.ret} ${s.name}(${s.args})\` — V8_WARN_UNUSED_RESULT\n`;
}
rust += `
/// 平台分支登记 (原版 cc 内条件编译, 语义实现由 Developer 落):
`;
for (const b of branches) {
  rust += `/// #if ${b}\n`;
}
rust += `
/// 全局驻留 (POSIX 分支): g_icu_data_file (MemoryMappedFile) / 其他平台:
/// g_icu_data_ptr — 由 Developer 按目标平台二选一。
`;

const p = writeSrc("v8-init", "icu_util.rs", rust);
console.log(`written ${p}`);
