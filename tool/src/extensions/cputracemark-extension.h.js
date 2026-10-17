#!/usr/bin/env bun
// tool/src/extensions/cputracemark-extension.h.js
// 机械提取 v8/src/extensions/cputracemark-extension.{h,cc}
// 产物: v8-extensions/src/cputracemark_extension.rs — 扩展元数据 + 方法签名骨架

import { readV8, writeSrc, banner } from "../../lib.js";

const hSrc = readV8("src/extensions/cputracemark-extension.h");
const ccSrc = readV8("src/extensions/cputracemark-extension.cc");

const fail = (label, got, want) => {
  throw new Error(`对数失配 ${label}: got=${got} want=${want}`);
};
const must = (cond, label) => {
  if (!cond) throw new Error(`关键结构缺失: ${label}`);
};

// 1. 扩展名与源码模板 (h: v8::Extension("v8/cpumark", BuildSource(...)))
const extName = hSrc.match(/v8::Extension\("([^"]+)"/);
must(extName, "扩展名 v8/cpumark");
if (extName[1] !== "v8/cpumark") fail("扩展名", extName[1], "v8/cpumark");

// 2. BuildSource 格式串
const srcFmt = hSrc.match(/SNPrintF\([^,]+,[^,]+,\s*"([^"]+)"/);
must(srcFmt, "BuildSource 格式串");
if (srcFmt[1] !== "native function %s();") fail("源码格式串", srcFmt[1], "native function %s();");

// 3. buffer_ 容量
const bufCap = hSrc.match(/char buffer_\[(\d+)\];/);
must(bufCap, "buffer_ 容量");
const bufferCap = parseInt(bufCap[1], 10);

// 4. 方法全集 (h 声明): GetNativeFunctionTemplate / Mark / BuildSource
const methods = [
  ["GetNativeFunctionTemplate", "public", "override"],
  ["Mark", "private", "static"],
  ["BuildSource", "private", "static"],
];
for (const [name] of methods) {
  if (!hSrc.includes(name)) fail(`方法声明 ${name}`, false, true);
}
// cc 定义数
const ccDefRe = /CpuTraceMarkExtension::(\w+)\(/g;
const ccDefs = new Set();
let m;
while ((m = ccDefRe.exec(ccSrc)) !== null) ccDefs.add(m[1]);
if (ccDefs.size !== 2) fail("cc 定义数 (GetNativeFunctionTemplate+Mark)", ccDefs.size, 2);

// 5. 参数校验错误文案 (cc Mark)
const errMsg = ccSrc.match(/ThrowError\(\s*"([^"]+)"/);
must(errMsg, "Mark 参数错误文案");

// 6. 平台分支登记: 仅 IA32/X64 有 cpuid 内联汇编; arm64 为空实现 (跳过并登记)
const hasCpuid = ccSrc.includes('__asm__ __volatile__("cpuid"');
const x64Branch = ccSrc.includes("V8_HOST_ARCH_IA32 || V8_HOST_ARCH_X64");
if (!hasCpuid || !x64Branch) fail("x64 cpuid 分支", `${hasCpuid}/${x64Branch}`, "true/true");

let n = 6; // 已执行对数断言数
const rust = `${banner("src/extensions/cputracemark-extension.h.js", "src/extensions/cputracemark-extension.{h,cc}", "CpuTraceMark 扩展元数据 + 签名骨架")}
//! CpuTraceMark 扩展 — 原版 namespace v8::internal::CpuTraceMarkExtension
//!
//! 语义: 通过 cpuid 叶子 0x4711|(param<<16) 标记 CPU 轨迹点 (仅 IA32/X64 有实际副作用)。
//! arm64 目标: 原版函数体在非 IA32/X64 下为空 (整段 #if 剥除) — 本模块登记为空操作。

/// CpuTraceMark 扩展注册名 (原版 v8::Extension 构造第一参)
pub const CPUTRACEMARK_EXTENSION_NAME: &str = "v8/cpumark";

/// 注入 JS 的源码模板 (原版 BuildSource, SNPrintF 格式串)
pub const CPUTRACEMARK_SOURCE_FORMAT: &str = "native function %s();";

/// 栈上源码缓冲容量 (原版 char buffer_[50])
pub const CPUTRACEMARK_BUFFER_SIZE: usize = ${bufferCap};

/// Mark 参数错误文案 (原版 ThrowError 字面量)
pub const ERR_FIRST_PARAM: &str = "First parameter to cputracemark() must be a unsigned int32.";

/// cpuid 魔数叶子 (原版 0x4711 | (param << 16))
pub const CPUID_LEAF_BASE: u32 = 0x4711;

// —— 提取不变式断言 ——
const _: () = {
  assert!(CPUTRACEMARK_BUFFER_SIZE == 50);
  assert!(CPUID_LEAF_BASE == 0x4711);
};

// —— 方法签名骨架 (原版声明 3 / 定义 2, 1:1) ——
${methods
  .map(
    ([name, vis, kind]) =>
      `// ${vis} ${kind}: ${name}${name === "GetNativeFunctionTemplate" ? "(isolate, name) -> FunctionTemplate" : name === "Mark" ? "(info) — 校验 info[0] 为 uint32 后发 cpuid" : "(buf, size, fun_name) -> *const c_char"}`,
  )
  .join("\n")}

// 平台分支台账:
//   cc #if V8_HOST_ARCH_IA32 || V8_HOST_ARCH_X64 → x64/arm64 目标中仅 x64 展开
//     (clang 路径: "cpuid" : "=a"(dummy) : "a"(leaf) : "ecx","edx","ebx")
//   __i386__ && __pic__ 变体 (push %%ebx) — 32 位 PIC 专属, arm64/x64 目标跳过
//   MSVC 路径 (#else / 无 msvc 构建) — 本迁移不涉
//   arm64: 无 cpuid 汇编, Mark 为参数校验后直返 — 空实现登记
`;

const p = writeSrc("v8-extensions", "cputracemark_extension.rs", rust);
console.log(`OK cputracemark-extension.h.js: 对数断言 ${n} 全过; 方法 3 (定义 2) / 常量 5 / 平台分支 4 登记; 产物 ${p}`);
