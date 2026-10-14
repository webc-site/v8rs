#!/usr/bin/env bun
// tool/src/snapshot/startup-serializer.js
// 机械提取 v8/src/snapshot/startup-serializer.{h,cc} 的方法面与序列化次序契约

import { readV8, writeSrc, banner } from "../../lib.js";

const hSrc = readV8("src/snapshot/startup-serializer.h");
const ccSrc = readV8("src/snapshot/startup-serializer.cc");

// 1. StartupSerializer 方法面 (h)
const clsM = hSrc.match(/class V8_EXPORT_PRIVATE StartupSerializer : public RootsSerializer \{([\s\S]*?)\n private:/);
if (!clsM) throw new Error("未找到 StartupSerializer 类体");
const methods = [...clsM[1].matchAll(/^\s{2}(?:void|bool)\s+(\w+)\(/gm)].map((m) => m[1]);
if (methods.length !== 5) {
  throw new Error(`StartupSerializer 方法数不符: 期望 5, 实得 ${methods.length}: ${methods.join(",")}`);
}

// 2. SerializedHandleChecker 方法面
const chkM = hSrc.match(/class SerializedHandleChecker : public RootVisitor \{([\s\S]*?)\n\};/);
if (!chkM) throw new Error("未找到 SerializedHandleChecker 类体");
const chkMethods = [...chkM[1].matchAll(/^\s{2}(?:void|bool)\s+(\w+)\(/gm)].map((m) => m[1]);
if (chkMethods.length !== 3) {
  throw new Error(`SerializedHandleChecker 方法数不符: 期望 3, 实得 ${chkMethods.length}`);
}

// 3. cc 规模
const ccLines = ccSrc.split("\n").length;
console.log(`Startup ${methods.length} + Checker ${chkMethods.length} + cc ${ccLines} 行`);

let rust = banner("src/snapshot/startup-serializer.js", "src/snapshot/startup-serializer.{h,cc}", "启动序列化器");
rust += `
/// StartupSerializer (原版, 继承 RootsSerializer)。
/// 堆序列化次序契约 (原版注释机械登记):
/// 1) 强根 → 2) builtins 与字节码 handler → 3) 启动对象缓存 → 4) 弱引用 (字符串表等)
/// 方法面 (原版 h, 共 ${methods.length} 项):
`;
for (const m of methods) rust += `/// - \`${m}\`\n`;
rust += `
/// FinalizationRegistry 契约: 脏 FinalizationRegistry 列表为弱引用不入快照,
/// 启动期禁用 (CheckNoDirtyFinalizationRegistries)。

/// SerializedHandleChecker (原版): 校验全局/永恒句柄均已被序列化。
/// 方法面 (共 ${chkMethods.length} 项): ${chkMethods.join(", ")}

/// cc 规模: ${ccLines} 行 (语义由 Developer 落)。
`;

const p = writeSrc("v8-snapshot", "startup_serializer.rs", rust);
console.log(`written ${p}`);
