#!/usr/bin/env bun
// tool/src/maglev/riscv/maglev-assembler-riscv-inl.h.js
// RISCV64 非本轮目标架构 (平台分支按 x64+arm64 展开, 规则九) → 跳过语义转换;
// 本脚本只做结构对数 + 跳过登记, 后续如启用该架构, 以此为复用底稿。

import { readV8, writeSrc, banner } from "../../../lib.js";

const GEN = "src/maglev/riscv/maglev-assembler-riscv-inl.h.js";
const die = (m) => {
  throw new Error(`[${GEN}] ${m}`);
};

const INL_REL = "src/maglev/riscv/maglev-assembler-riscv-inl.h";
const CC_REL = "src/maglev/riscv/maglev-assembler-riscv.cc";
const inl = readV8(INL_REL);
const cc = readV8(CC_REL);

// —— 结构对数 (定义头: 返回类型 + MaglevAssembler::name(...) {) ————
function countDefs(src) {
  const folded = src.replace(/,\n[ \t]+/g, ", ").replace(/\(\n[ \t]+/g, "(");
  const re = /^((?:template\s*<[^\n>]*>\s*)?(?:inline\s+)?[A-Za-z_][\w:<>,&* \t]*?)MaglevAssembler::(\w+)\s*\(([^{};]*)\)\s*\{/gm;
  const names = [];
  let m;
  while ((m = re.exec(folded)) !== null) names.push(m[2]);
  return names;
}

const inlNames = countDefs(inl);
const ccNames = countDefs(cc);
if (inl.split("\n").length !== 2245) die(`-inl 行数 ${inl.split("\n").length} != 2245`);
if (cc.split("\n").length !== 692) die(`.cc 行数 ${cc.split("\n").length} != 692`);
if (inlNames.length !== 194) die(`-inl 定义 ${inlNames.length} != 194`);
if (ccNames.length !== 16) die(`.cc 定义 ${ccNames.length} != 16`);
if (!inl.includes("class MaglevAssembler::TemporaryRegisterScope")) {
  die("缺少 TemporaryRegisterScope 类定义锚");
}

let rust = banner(GEN, `${INL_REL} + ${CC_REL}`, "RISCV64 MaglevAssembler 跳过登记 (非目标架构)");
rust += `
//! 规则九: 平台分支按 x64+arm64 展开, RISCV64 跳过语义转换, 仅登记结构面。
//! 文件齐备、锚点全过; 启用该架构时以脚本对数为底稿展开签名骨架。

pub static REGISTERED_FILES: &[(&str, usize, usize)] = &[
  ("src/maglev/riscv/maglev-assembler-riscv-inl.h", 2245, 194),
  ("src/maglev/riscv/maglev-assembler-riscv.cc", 692, 16),
];

const _: () = {
  assert!(REGISTERED_FILES.len() == 2);
  assert!(REGISTERED_FILES[0].1 == 2245);
  assert!(REGISTERED_FILES[1].1 == 692);
};
`;

const p = writeSrc("v8-maglev-backend", "riscv/maglev_assembler_riscv.rs", rust);
console.log(`OK ${GEN}: inl=${inlNames.length} cc=${ccNames.length} → ${p}`);
