#!/usr/bin/env bun
// tool/src/init/v8.js
// 机械提取 v8/src/init/v8.{h,cc} 的 V8StartupState 状态机枚举与 V8 门面签名骨架

import { readV8, writeSrc, banner } from "../../lib.js";

const hSrc = readV8("src/init/v8.h");
const ccSrc = readV8("src/init/v8.cc");

// 1. 提取 V8StartupState 枚举 (v8.cc 匿名 namespace 内)
const stateMatch = ccSrc.match(/enum class V8StartupState \{([^}]+)\};/);
if (!stateMatch) throw new Error("未找到 V8StartupState 枚举");

const rawStates = stateMatch[1]
  .split(",")
  .map((s) => s.trim())
  .filter(Boolean);
if (rawStates.length !== 9) {
  throw new Error(`V8StartupState 变体数不符: 期望 9, 实得 ${rawStates.length}`);
}

// kPlatformInitializing → PlatformInitializing: 去前缀, 首字母大写, 其余保留
const toPascalCase = (name) => {
  const t = name.replace(/^k/, "");
  return t.charAt(0).toUpperCase() + t.slice(1);
};

const states = rawStates.map((s, idx) => ({ raw: s, rustName: toPascalCase(s), repr: idx }));
console.log(`提取 V8StartupState ${states.length} 变体`);

// 2. 提取 V8 类 public 静态方法签名 (v8.h)
const v8ClassMatch = hSrc.match(/class V8 : public AllStatic \{([\s\S]*?)\n\};/);
if (!v8ClassMatch) throw new Error("未找到 V8 类体");
const classBody = v8ClassMatch[1];

const pubSigs = [];
const sigLines = classBody.split("\n");
// 逐行拼接跨行签名: 以 "static"/"[noreturn]" 开头到 "(" 所在行为止
for (let i = 0; i < sigLines.length; i++) {
  const line = sigLines[i].trim();
  if (!/^(static |\[\[noreturn\]\]|V8_EXPORT_PRIVATE)/.test(line)) continue;
  let full = line;
  // 无 "(" 却已以 ";" 结尾 → 纯声明 (kNoOOMDetails 等), 直接跳过不吞行
  if (!full.includes("(") && full.includes(";")) continue;
  // 跨行签名: 参数表未闭合 (缺 ")") 时继续吞行
  while (!/\)\s*(?:const\s*)?[;{=]?\s*$/.test(full) && i + 1 < sigLines.length) {
    full += " " + sigLines[++i].trim();
  }
  const m = full.match(/(?:static\s+.*?\s)?\**(\w+)\s*\(/);
  if (m) pubSigs.push({ full, name: m[1] });
}
if (pubSigs.length !== 9) {
  throw new Error(`V8 public 签名数不符: 期望 9, 实得 ${pubSigs.length}: ${pubSigs.map((s) => s.name).join(",")}`);
}
console.log(`提取 V8 签名 ${pubSigs.length} 条`);

// 3. kNoOOMDetails / kHeapOOM (v8.cc 定义) — 仅登记原文, 不造类型
const oomDefs = [...ccSrc.matchAll(/const OOMDetails V8::(k\w+)\{([^}]*)\};/g)].map(
  (m) => ({ name: m[1], init: m[2] }),
);
if (oomDefs.length !== 2) throw new Error(`OOM 常量数不符: 期望 2, 实得 ${oomDefs.length}`);

let rust = banner("src/init/v8.js", "src/init/v8.{h,cc}", "V8 全局初始化门面与启动状态机");
rust += `
/// V8 进程级启动/关闭状态机 (原版 v8.cc 匿名 namespace 内 enum class V8StartupState)
/// 状态严格按序推进: 每次 AdvanceStartupState 只允许 +1 步, 乱序 FATAL。
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum V8StartupState {
`;
for (const s of states) {
  rust += `  /// ${s.raw}\n  ${s.rustName} = ${s.repr},\n`;
}
rust += `}

impl V8StartupState {
  /// 状态总数 (原版共 9 态)
  pub const COUNT: usize = ${states.length};
}

`;

// V8 签名骨架
rust += `/// V8 全局门面 (原版 class V8 : AllStatic, 全静态方法)
/// 签名骨架: 语义实现由 Developer 就地修订
pub struct V8;

impl V8 {
`;
for (const s of pubSigs) {
  rust += `  /// 原版签名: ${s.full.replace(/\s+/g, " ")}\n`;
}
rust += `  // (骨架占位: 具体方法由 Developer 按上列原版签名补齐)

  /// OOM 详情常量 (原版 v8.cc 定义, 类型为 v8::OOMDetails, 由 Developer 落型):
`;
for (const o of oomDefs) {
  rust += `  /// V8::${o.name} 初始化式 { ${o.init} }\n`;
}
rust += `}

// —— 提取不变式断言 ——
const _: () = {
  assert!(V8StartupState::COUNT == 9);
  assert!(V8StartupState::Idle as u8 == 0);
  assert!(V8StartupState::PlatformDisposed as u8 == 8);
};
`;

const p = writeSrc("v8-init", "v8.rs", rust);
console.log(`written ${p}`);
