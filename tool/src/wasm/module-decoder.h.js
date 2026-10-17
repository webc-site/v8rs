#!/usr/bin/env bun
// tool/src/wasm/module-decoder.h.js
// 机械提取 v8/src/wasm/module-decoder.{h,cc}: DecodingMethod 枚举与特殊段映射表。
// (module-decoder-impl.h 为解码器实现, 语义侧留给 Developer)

import { readV8, writeSrc, banner } from "../../lib.js";

const hSrc = readV8("src/wasm/module-decoder.h");
const implSrc = readV8("src/wasm/module-decoder-impl.h");
const ccSrc = readV8("src/wasm/module-decoder.cc");

const dm = hSrc.match(/enum class DecodingMethod \{([^}]*)\};/);
if (!dm) throw new Error("未找到 DecodingMethod");
const methods = dm[1].split(",").map((s) => s.trim()).filter(Boolean);
if (methods.length !== 5 || methods[0] !== "kSync" || methods[4] !== "kDeserialize") {
  throw new Error(`DecodingMethod 漂移: ${methods}`);
}

// kSpecialSections: {StaticCharVector(kXxxString), kYyySectionCode}
const special = [...implSrc.matchAll(
  /\{base::StaticCharVector\((k\w+String)\),\s*(k\w+SectionCode)\}/g,
)].map((m) => ({ str: m[1], code: m[2] }));
if (special.length !== 10) throw new Error(`kSpecialSections ${special.length} 条, 期望 10`);
if (special[0].str !== "kNameString" || special[special.length - 1].str !== "kBuildIdString") {
  throw new Error("kSpecialSections 锚点漂移");
}

// 入口函数锚点
for (const fn of ["DecodeWasmModule", "DecodeCustomSections", "ValidateFunctions", "SectionName"]) {
  if (!hSrc.includes(fn)) throw new Error(`入口锚点缺失: ${fn}`);
}
if (!ccSrc.includes("SectionName(SectionCode code)")) throw new Error("cc SectionName 定义缺失");

console.log(`DecodingMethod ${methods.length}, 特殊段 ${special.length}`);

let rust = banner(
  "src/wasm/module-decoder.h.js",
  "src/wasm/module-decoder.{h,cc}",
  "模块解码入口: DecodingMethod 枚举 + 特殊段名映射表",
);

rust += `
/// 解码方式 (对应 enum class DecodingMethod)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum DecodingMethod {
`;

const dmNames = methods.map((m) => m.replace(/^k/, ""));
dmNames.forEach((n, i) => {
  rust += `  /// k${n}\n  ${n.replace(/([a-z0-9])([A-Z])/g, "$1_$2").toUpperCase()} = ${i},\n`;
});

rust += `}

/// 特殊自定义段名 → 段码映射 (对应 kSpecialSections, ${special.length} 条)
/// 段码记号保留原名 (SectionCode 定义在 wasm-constants 侧)
#[rustfmt::skip]
pub static SPECIAL_SECTIONS: &[(&str, &str)] = &[
`;

const strConst = (s) => s.replace(/^k(.*)String$/, "$1"); // kNameString → Name
for (const s of special) {
  rust += `  ("${strConst(s.str)}", "${s.code}"),\n`;
}

rust += `];

/// 识别特殊自定义段 (对应 IdentifyUnknownSectionInternal 的表驱动部分, 骨架)
pub fn identify_special_section(name: &str) -> Option<&'static str> {
  SPECIAL_SECTIONS.iter().find(|(n, _)| *n == name).map(|(_, code)| *code)
}

/// 段名 (对应 SectionName(SectionCode), 骨架)
pub fn section_name(_code: u8) -> &'static str {
  unimplemented!("骨架: 参照原版 module-decoder.cc SectionName 段码表修订")
}

/// 解码模块 (对应 DecodeWasmModule 三重载, 骨架)
pub fn decode_wasm_module(
  _method: DecodingMethod,
  _enabled_features: crate::wasm_features::WasmEnabledFeatures,
  _wire_bytes: &[u8],
) -> crate::error::Result<()> {
  unimplemented!("骨架: 参照原版 module-decoder.cc / module-decoder-impl.h 修订")
}

/// 解码全部自定义段 (对应 DecodeCustomSections, 骨架)
pub fn decode_custom_sections(_wire_bytes: &[u8]) -> Vec<(u32, u32)> {
  unimplemented!("骨架: 返回 CustomSectionOffset 列表, 待修订")
}

/// 校验全部函数体 (对应 ValidateFunctions, 骨架)
pub fn validate_functions(_module: &(), _enabled_features: crate::wasm_features::WasmEnabledFeatures) -> Result<(), crate::error::WasmError> {
  unimplemented!("骨架: 参照原版修订")
}

// —— 提取不变式断言 ——
const _: () = {
  assert!(DecodingMethod::SYNC == 0);
  assert!(DecodingMethod::ASYNC_STREAM == 3);
  assert!(DecodingMethod::DESERIALIZE == 4);
  assert!(SPECIAL_SECTIONS.len() == 10);
  assert!(SPECIAL_SECTIONS[0].0 == "Name");
  assert!(SPECIAL_SECTIONS[0].1 == "kNameSectionCode");
  assert!(identify_special_section("Name") == Some("kNameSectionCode"));
};
`;
;

const p = writeSrc("v8-wasm-core", "module_decoder.rs", rust);
console.log(`written ${p}`);
