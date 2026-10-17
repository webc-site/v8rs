#!/usr/bin/env bun
// tool/src/regexp/special-case.h.js
// 机械提取 v8/src/regexp/special-case.h 的大小写折叠模式与骨架
// (IgnoreSet/SupplementaryEquivalents 由 gen-regexp-special-case.cc 构建期生成,
//  属生成数据, 本脚本只登记接口骨架与 Mode 枚举)

import { readV8, writeSrc, banner } from "../../lib.js";

const hSrc = readV8("src/regexp/special-case.h");

const modeMatch = hSrc.match(/enum class Mode \{ kNonUnicode, kUnicode \};/);
if (!modeMatch) throw new Error("未找到 CaseFolding::Mode");

const eqKey = hSrc.match(/static UChar32 EquivalenceKey\(UChar32 c, Mode mode\)/);
if (!eqKey) throw new Error("未找到 EquivalenceKey 签名");

// V8_INTL_SUPPORT 条件 (整接口在其之下)
if (!hSrc.includes("#ifdef V8_INTL_SUPPORT")) throw new Error("未找到 V8_INTL_SUPPORT 条件");
// 构建期生成物说明
if (!hSrc.includes("gen-regexp-special-case.cc")) throw new Error("未找到生成器说明锚");

console.log("special-case.h: Mode=2 skeleton");

let rust = banner(
  "src/regexp/special-case.h.js",
  "src/regexp/special-case.h",
  "非 Unicode 忽略集合接口骨架 (数据由构建期生成器产出, 登记待 Developer 接线)",
);

rust += `
/// 大小写折叠模式 (对应原版 CaseFolding::Mode)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum CaseFoldingMode {
  /// kNonUnicode — "i" 模式, 输入为 UTF-16 码元
  NonUnicode = 0,
  /// kUnicode — "iu" 模式, 输入为码点
  Unicode = 1,
}

/// 等价键 (原版 CaseFolding::EquivalenceKey(UChar32, Mode))
/// 同键即互相匹配; 具体表由 gen-regexp-special-case.cc 构建期生成
/// (登记签名骨架; 语义面待 Developer 修订)
pub fn equivalence_key(_c: u32, _mode: CaseFoldingMode) -> u32 {
  unimplemented!("登记骨架: 数据源为 gen 产物 special-case.cc (构建期生成)")
}

/// 忽略集合 (原版 IgnoreSet — 仅构建期生成数据; 字符须只匹配自身)
/// 例: U+00DF (ß) 在集合内; U+212A (KELVIN) 不与 'k' 匹配于非 unicode 模式
pub struct IgnoreSet;

/// 补充平面等价集 (原版 SupplementaryEquivalentsSet — 构建期生成)
pub struct SupplementaryEquivalentsSet;

// —— 提取不变式断言 ——
const _: () = {
  assert!(CaseFoldingMode::Unicode as u8 == 1);
};
`;

const p = writeSrc("v8-regexp-compiler", "special_case.rs", rust);
console.log(`written ${p}`);
