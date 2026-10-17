#!/usr/bin/env bun
// tool/src/maglev/s390/maglev-assembler-s390-inl.h.js
// S390 非本轮目标架构 (平台分支按 x64+arm64 展开, 规则九) → 跳过语义转换;
// 本脚本只做结构对数 + 跳过登记, 后续如启用该架构, 以此为复用底稿。

import { readV8, writeSrc, banner } from "../../../lib.js";

const GEN = "src/maglev/s390/maglev-assembler-s390-inl.h.js";
const die = (m) => {
  throw new Error(`[${GEN}] ${m}`);
};

const INL_REL = "src/maglev/s390/maglev-assembler-s390-inl.h";
const CC_REL = "src/maglev/s390/maglev-assembler-s390.cc";
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
if (inl.split("\n").length !== 1625) die(`-inl 行数 ${inl.split("\n").length} != 1625`);
if (cc.split("\n").length !== 607) die(`.cc 行数 ${cc.split("\n").length} != 607`);
if (inlNames.length !== 178) die(`-inl 定义 ${inlNames.length} != 178`);
if (ccNames.length !== 15) die(`.cc 定义 ${ccNames.length} != 15`);
if (!inl.includes("class MaglevAssembler::TemporaryRegisterScope")) {
  die("缺少 TemporaryRegisterScope 类定义锚");
}

let rust = banner(GEN, `${INL_REL} + ${CC_REL}`, "S390 MaglevAssembler 跳过登记 (非目标架构)");
rust += `
//! 规则九: 平台分支按 x64+arm64 展开, S390 跳过语义转换, 仅登记结构面。
//! 文件齐备、锚点全过; 启用该架构时以脚本对数为底稿展开签名骨架。

pub static REGISTERED_FILES: &[(&str, usize, usize)] = &[
  ("src/maglev/s390/maglev-assembler-s390-inl.h", 1625, 178),
  ("src/maglev/s390/maglev-assembler-s390.cc", 607, 15),
];

const _: () = {
  assert!(REGISTERED_FILES.len() == 2);
  assert!(REGISTERED_FILES[0].1 == 1625);
  assert!(REGISTERED_FILES[1].1 == 607);
};
`;

const p = writeSrc("v8-maglev-backend", "s390/maglev_assembler_s390.rs", rust);
console.log(`OK ${GEN}: inl=${inlNames.length} cc=${ccNames.length} → ${p}`);
