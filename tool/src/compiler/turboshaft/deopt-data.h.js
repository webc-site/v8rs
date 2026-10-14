#!/usr/bin/env bun
// tool/src/compiler/turboshaft/deopt-data.h.js
// 机械提取 v8/src/compiler/turboshaft/deopt-data.h
// → v8-turboshaft/src/deopt_data.rs.skel
//
// 提取面: FrameStateData::Instr 枚举 (9 变体, 树前序编码) + Builder 方法表
// (AddXxx) + 字段面。语义实现禁编造。
import { readV8, writeSrc, banner } from "../../../lib.js";

const h = readV8("src/compiler/turboshaft/deopt-data.h");

// 1. Instr 枚举 (树前序指令)
const instrM = h.match(/enum class Instr : uint8_t\s*\{([\s\S]*?)\};/);
if (!instrM) throw new Error("未找到 FrameStateData::Instr");
const instrVariants = [...instrM[1].replace(/\/\/[^\n]*/g, "").matchAll(/\bk(\w+)\b/g)].map((m) => m[1]);
const expect = ["Input", "UnusedRegister", "DematerializedObject", "DematerializedObjectReference",
  "ArgumentsElements", "ArgumentsLength", "RestLength", "DematerializedStringConcat",
  "DematerializedStringConcatReference"];
if (JSON.stringify(instrVariants) !== JSON.stringify(expect)) {
  throw new Error(`Instr ${JSON.stringify(instrVariants)} != 预期 9 项`);
}

// 2. Builder 方法表 (AddXxx, 保序)
const builderM = h.match(/class Builder\s*\{([\s\S]*?)\n  \};/);
if (!builderM) throw new Error("未找到 FrameStateData::Builder");
const addMethods = [...builderM[1].matchAll(/^\s*void (Add\w+)\(/gm)].map((m) => m[1]);
const expectAdd = ["AddParentFrameState", "AddInput", "AddUnusedRegister", "AddDematerializedObjectReference",
  "AddDematerializedObject", "AddDematerializedStringConcat", "AddDematerializedStringConcatReference",
  "AddArgumentsElements", "AddArgumentsLength"];
for (const m of expectAdd) {
  if (!addMethods.includes(m)) throw new Error(`Builder 缺方法: ${m}`);
}

// 3. 其余结构面哨兵
for (const [re, label] of [
  [/struct FrameStateData\s*\{/, "FrameStateData"],
  [/The data is encoded as a pre-traversal of a tree\./, "前序编码注释锚"],
  [/class Builder/, "Builder"],
  [/bool inlined_ = false;/, "inlined_ 字段"],
  [/inlined_ = true;/, "AddParentFrameState 置 inlined_"],
]) {
  if (!re.test(h)) throw new Error(`哨兵缺失: ${label}`);
}

// 4. 生成 Rust
let rust = banner(
  "src/compiler/turboshaft/deopt-data.h.js",
  "src/compiler/turboshaft/deopt-data.h",
  "去优化帧状态: 前序编码指令与 Builder 骨架",
);
rust += `
/// v8: src/compiler/turboshaft/deopt-data.h FrameStateData::Instr
/// 树前序编码指令 (注释保留原版操作数说明):
${instrM[1].split("\n").filter((l) => l.trim().startsWith("k")).map((l) => `/// ${l.trim()}`).join("\n")}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Instr {
${expect.map((v, i) => `  /// k${v}\n  ${v.replace(/^./, (c) => c.toLowerCase())} = ${i},`).join("\n")}
}

/// v8: src/compiler/turboshaft/deopt-data.h FrameStateData
pub struct FrameStateData {
  /// instructions_ — Instr 序列
  pub instructions: Vec<Instr>,
  /// machine_types_ — kInput 的机器类型
  pub machine_types: Vec<()>,
  /// int_operands_ — Dematerialized/ArgumentsElements 的整型操作数
  pub int_operands: Vec<u32>,
  /// inlined_ — 有 parent (kInput 前唯一)
  pub inlined: bool,
}

impl FrameStateData {
  /// v8: src/compiler/turboshaft/deopt-data.h FrameStateData::Builder
  /// Build → (inputs, FrameStateData); inlined 时 inputs 首位是 parent
  pub fn builder() -> FrameStateDataBuilder {
    unimplemented!("签名骨架, Developer 修订")
  }
}

/// v8: src/compiler/turboshaft/deopt-data.h FrameStateData::Builder
/// 方法面 (原版 ${addMethods.length} AddXxx):
${addMethods.map((m) => `/// - ${m}`).join("\n")}
pub struct FrameStateDataBuilder;

impl FrameStateDataBuilder {
`;
for (const m of addMethods) {
  const snakeName = m.replace(/^Add/, "").replace(/([a-z0-9])([A-Z])/g, "$1_$2").toLowerCase();
  rust += `  /// 原版 void ${m}(...)
  /// v8: src/compiler/turboshaft/deopt-data.h FrameStateData::Builder::${m}
  pub fn add_${snakeName}(&mut self) {
    unimplemented!("签名骨架, Developer 修订")
  }
`;
}
rust += `}

// —— 提取不变式断言 ——
const _: () = {
  assert!(Instr::Input as u8 == 0);
  assert!(Instr::DematerializedStringConcatReference as u8 == 8);
};
`;

const p = writeSrc("v8-turboshaft", "deopt_data.rs.skel", rust);
console.log(`written ${p}`);
console.log(`Instr=${expect.length} Builder方法=${addMethods.length} 哨兵=5`);
