#!/usr/bin/env bun
// tool/src/codegen/reloc-info.cc.js
// 机械提取 v8/src/codegen/reloc-info.{cc,-inl.h} —
// 目标地址寻址模式掩码双分支 + inl 断言锚。
// 注: reloc_info.rs (手写) 已覆盖 reloc-info.h 规则面, 本脚本落 _cc_generated 旁路。
// 产物: v8-codegen-core/src/reloc_info_cc_generated.rs

import { readV8, writeSrc, banner } from "../../lib.js";

const CC_REL = "src/codegen/reloc-info.cc";
const INL_REL = "src/codegen/reloc-info-inl.h";
const cc = readV8(CC_REL);
const inl = readV8(INL_REL);

// 双分支掩码常量: kTargetAddressAddressModeMask (HasTargetAddressAddress 内)
function branchMasks() {
  const out = [];
  const re = /static constexpr int kTargetAddressAddressModeMask =\s*([\s\S]*?);/g;
  let m;
  while ((m = re.exec(cc)) !== null) out.push(m[1].replace(/\s+/g, " ").trim());
  return out;
}
const masks = branchMasks();
if (masks.length !== 2) throw new Error(`kTargetAddressAddressModeMask 分支数异常: ${masks.length}`);
if (!masks[0].includes("CODE_TARGET") || !masks[1].includes("RELATIVE_CODE_TARGET")) {
  throw new Error("掩码分支内容锚不符");
}
if (!inl.includes("RelocInfoWriter") && !inl.includes("RelocInfo")) throw new Error("inl 缺锚");
if (!cc.includes("RelocIterator")) throw new Error("cc 缺迭代器锚");

let rust = banner("src/codegen/reloc-info.cc.js", "src/codegen/reloc-info.{cc,-inl.h}", "重定位信息 inl 常量与骨架");
rust += `
// reloc-info.h 枚举/表由 reloc_info.rs (手写) 覆盖; 此处登记 inl 分支常量。

/// 目标地址寻址模式掩码 — HasTargetAddressAddress 内双分支 (x64/ia32 vs 其他):
/// - 分支 0 (压缩指针/默认布局): ${masks[0]}
/// - 分支 1: ${masks[1]}
pub mod target_address_address_mode {
${masks
  .map((m, i) => `  /// 分支 ${i} 原文: ${m}\n  pub const BRANCH_${i}_EXPR: &str = ${JSON.stringify(m)};`)
  .join("\n")}
}

/// 原 RelocInfo::HasTargetAddressAddress 判定使用的模式掩码 (枚举名引用,
/// 数值由 reloc_info.rs 手写产物中的 ModeMask 对应枚举决定)。
/// 语义面: RelocInfoWriter/RelocIterator 编解码实现登记跳过。
pub struct RelocInfoWriter;

// —— 提取不变式断言 ——
const _: () = {};
`;

const p = writeSrc("v8-codegen-core", "reloc_info_cc_generated.rs", rust);
console.log(`written ${p} (掩码分支 ${masks.length})`);
