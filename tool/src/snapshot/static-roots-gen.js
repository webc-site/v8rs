#!/usr/bin/env bun
// tool/src/snapshot/static-roots-gen.js
// 机械提取 v8/src/snapshot/static-roots-gen.{h,cc} 的生成器门面

import { readV8, writeSrc, banner } from "../../lib.js";

const hSrc = readV8("src/snapshot/static-roots-gen.h");
const ccSrc = readV8("src/snapshot/static-roots-gen.cc");

// 1. 门面 (h): write / write_text
const clsM = hSrc.match(/class StaticRootsTableGen \{([\s\S]*?)\n\};/);
if (!clsM) throw new Error("未找到 StaticRootsTableGen 类体");
const methods = [...clsM[1].matchAll(/static void (\w+)\(/g)].map((m) => m[1]);
if (methods.length !== 2) throw new Error(`方法数不符: 期望 2, 实得 ${methods.length}: ${methods.join(",")}`);

// 2. cc 输出形态 (机械提取): 头守卫 defname 由输出路径推导, include guard 自动生成
const defnameM = ccSrc.match(/std::string defname = file_str\.substr\(file_str\.find_last_of\("\/\\\\"\) \+ 1\);/);
const ofstreamM = ccSrc.includes("std::ofstream out(file, std::ios::binary)");
if (!defnameM || !ofstreamM) throw new Error("输出形态登记异常");
console.log(`方法 ${methods.length} + 输出形态 2`);

let rust = banner("src/snapshot/static-roots-gen.js", "src/snapshot/static-roots-gen.{h,cc}", "静态根表生成器");
rust += `
/// StaticRootsTableGen (原版 AllStatic 门面): 从运行中 isolate 落静态根表
/// (V8_STATIC_ROOTS 构建产物, mksnapshot 专用)。
/// 方法面:
/// - \`write\` (原版 write): 落 C++ 头 (根偏移表; include guard 由输出文件名
///   推导 defname, 绝对值常量可跨文件复制)
/// - \`write_text\` (原版 write_text): 落可读文本版
/// 输出经 std::ofstream 二进制写; 具体表内容由 Developer 对齐 roots 产物落。
`;

const p = writeSrc("v8-snapshot", "static_roots_gen.rs", rust);
console.log(`written ${p}`);
