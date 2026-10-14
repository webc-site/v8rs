#!/usr/bin/env bun
// tool/src/maglev/loong64/maglev-assembler-loong64-inl.h.js
// LoongArch64 非本轮目标架构 (平台分支按 x64+arm64 展开, 规则九) → 跳过语义转换;
// 本脚本只做结构对数 + 跳过登记, 后续如启用该架构, 以此为复用底稿。

import { readV8, writeSrc, banner } from "../../../lib.js";

const GEN = "src/maglev/loong64/maglev-assembler-loong64-inl.h.js";
const die = (m) => {
  throw new Error(`[${GEN}] ${m}`);
};

const INL_REL = "src/maglev/loong64/maglev-assembler-loong64-inl.h";
const CC_REL = "src/maglev/loong64/maglev-assembler-loong64.cc";
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
if (inl.split("\n").length !== 1943) die(`-inl 行数 ${inl.split("\n").length} != 1943`);
if (cc.split("\n").length !== 721) die(`.cc 行数 ${cc.split("\n").length} != 721`);
if (inlNames.length !== 195) die(`-inl 定义 ${inlNames.length} != 195`);
if (ccNames.length !== 15) die(`.cc 定义 ${ccNames.length} != 15`);
if (!inl.includes("class MaglevAssembler::TemporaryRegisterScope")) {
  die("缺少 TemporaryRegisterScope 类定义锚");
}

let rust = banner(GEN, `${INL_REL} + ${CC_REL}`, "LoongArch64 MaglevAssembler 跳过登记 (非目标架构)");
rust += `
//! 规则九: 平台分支按 x64+arm64 展开, LoongArch64 跳过语义转换, 仅登记结构面。
//! 文件齐备、锚点全过; 启用该架构时以脚本对数为底稿展开签名骨架。

pub static REGISTERED_FILES: &[(&str, usize, usize)] = &[
  ("src/maglev/loong64/maglev-assembler-loong64-inl.h", 1943, 195),
  ("src/maglev/loong64/maglev-assembler-loong64.cc", 721, 15),
];

const _: () = {
  assert!(REGISTERED_FILES.len() == 2);
  assert!(REGISTERED_FILES[0].1 == 1943);
  assert!(REGISTERED_FILES[1].1 == 721);
};
`;

const p = writeSrc("v8-maglev-backend", "loong64/maglev_assembler_loong64.rs", rust);
console.log(`OK ${GEN}: inl=${inlNames.length} cc=${ccNames.length} → ${p}`);
