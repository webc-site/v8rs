#!/usr/bin/env bun
// tool/src/sandbox/sandboxable-thread.h.js
// 机械提取 v8/src/sandbox/sandboxable-thread.{h,cc} — 沙箱硬件支持线程骨架。
// 产物: v8-sandbox/src/sandboxable_thread.rs

import { readV8, writeSrc, banner } from "../../lib.js";

const H_REL = "src/sandbox/sandboxable-thread.h";
const CC_REL = "src/sandbox/sandboxable-thread.cc";
const h = readV8(H_REL);
const cc = readV8(CC_REL);

if (!h.includes("class V8_EXPORT_PRIVATE SandboxableThread : public v8::base::Thread")) {
  throw new Error("缺 SandboxableThread 类锚");
}
if (!h.includes("void Dispatch() override;")) throw new Error("缺 Dispatch 声明锚");
// cc: Dispatch 实现 + 平台初始化调用
const dispatchM = cc.match(/void SandboxableThread::Dispatch\(\)\s*\{([\s\S]*?)\n\}/);
if (!dispatchM) throw new Error("cc 缺 Dispatch 实现");
const initCall = dispatchM[1].match(/(\w+::\w+)\(/);
if (!initCall) throw new Error("Dispatch 缺平台初始化调用");

let rust = banner("src/sandbox/sandboxable-thread.h.js", "src/sandbox/sandboxable-thread.{h,cc}", "SandboxableThread 骨架");
rust += `
/// 原 class SandboxableThread : public v8::base::Thread
/// 运行 JS/WASM 的线程必须沙箱化: 线程启动前做硬件沙箱初始化。
/// - 原签名: explicit SandboxableThread(const Options&)
/// - 原签名: void Dispatch() override  (per-thread 硬件沙箱初始化后转 Run())
/// cc 侧 Dispatch 调用的平台初始化入口: \`${initCall[1]}\`
pub struct SandboxableThread;

// TODO(dev): 以线程 trait 补 Dispatch→初始化→Run 流程。

// —— 提取不变式断言 ——
const _: () = {};
`;

const p = writeSrc("v8-sandbox", "sandboxable_thread.rs", rust);
console.log(`written ${p}`);
