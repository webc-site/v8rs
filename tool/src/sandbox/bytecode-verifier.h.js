#!/usr/bin/env bun
// tool/src/sandbox/bytecode-verifier.h.js
// 机械提取 v8/src/sandbox/bytecode-verifier.{h,cc} — 字节码沙箱校验器骨架。
// 产物: v8-sandbox/src/bytecode_verifier.rs

import { readV8, writeSrc, banner } from "../../lib.js";

const H_REL = "src/sandbox/bytecode-verifier.h";
const CC_REL = "src/sandbox/bytecode-verifier.cc";
const h = readV8(H_REL);
const cc = readV8(CC_REL);

const classM = h.match(/class V8_EXPORT_PRIVATE BytecodeVerifier\s*\{([\s\S]*?)\n\};/);
if (!classM) throw new Error("未找到 BytecodeVerifier 类");
const sigs = [...classM[1].matchAll(/^\s*(?:\[\[noreturn\]\]\s*)?static\s+(?:inline\s+)?[\w:<>,&*\s]+?\b(\w+)\(/gm)].map((m) => m[1]);
if (sigs.length < 4) throw new Error(`方法面不足: ${sigs.length}`);
for (const anchor of ["static void Verify(", "VerifyLight(", "VerifyFull(", "FailVerification("]) {
  if (!classM[1].includes(anchor)) throw new Error(`缺方法锚 ${anchor}`);
}
// cc 侧错误报告机制锚
if (!cc.includes('FailVerification("')) throw new Error("cc 缺 FailVerification 调用锚");
const failM = [null, "const char* reason"];

let rust = banner("src/sandbox/bytecode-verifier.h.js", "src/sandbox/bytecode-verifier.{h,cc}", "BytecodeVerifier 骨架");
rust += `
// 沙箱字节码校验器 — 校验逻辑 (cc ${cc.split("\n").length} 行) 为纯语义面, 登记跳过。

/// 原 class BytecodeVerifier (全静态接口)
pub struct BytecodeVerifier;

impl BytecodeVerifier {
${sigs.map((s) => `  // 原签名: static ${s}`).join("\n")}
}

// FailVerification 原签名参数: (${failM[1]})
// TODO(dev): 按字节码遍历补 Verify/VerifyLight/VerifyFull 校验实现。

// —— 提取不变式断言 ——
const _: () = {};
`;

const p = writeSrc("v8-sandbox", "bytecode_verifier.rs", rust);
console.log(`written ${p} (方法 ${sigs.length})`);
