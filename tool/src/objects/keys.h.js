#!/usr/bin/env bun
// tool/src/objects/keys.h.js — 覆盖 v8/src/objects/keys.{h,cc}
// → v8-objects-rest/src/keys.rs
// 机械提取: AddKeyConversion / GetKeysConversion / KeyCollectionMode / IndexedOrNamed

import { readV8, writeSrc, banner } from "../../lib.js";

const hSrc = readV8("src/objects/keys.h");
const src = hSrc.replace(/\/\*[\s\S]*?\*\//g, "").replace(/^[ \t]*\/\/.*$/gm, "");

function splitTop(body) {
  return body
    .split(",")
    .map((s) => s.trim())
    .filter(Boolean);
}

// AddKeyConversion (非 class 枚举, 顺序值)
const addKey = splitTop(src.match(/enum AddKeyConversion\s*\{([^}]+)\}/)[1]);
if (addKey.length !== 2 || addKey[0] !== "DO_NOT_CONVERT" || addKey[1] !== "CONVERT_TO_ARRAY_INDEX") {
  throw new Error("AddKeyConversion 条目失配");
}

// GetKeysConversion: 显式值 = v8 公开枚举值
const gkcRaw = splitTop(src.match(/enum class GetKeysConversion\s*\{([\s\S]*?)\n\};/)[1]);
if (gkcRaw.length !== 3) throw new Error(`GetKeysConversion 条数 ${gkcRaw.length} != 3`);
const gkc = gkcRaw.map((e) => {
  const m = e.match(/^(k\w+)\s*=\s*static_cast<int>\(v8::KeyConversionMode::(k\w+)\)$/);
  if (!m) throw new Error(`GetKeysConversion 条目不可解析: ${e}`);
  if (m[1] !== m[2]) throw new Error(`GetKeysConversion 内外名失配: ${e}`);
  return m[1];
});
// v8::KeyConversionMode 值锚 (include/v8-object.h; 注意顺序 kConvertToString 在前)
const v8Header = readV8("include/v8-object.h");
const kcm = v8Header.match(/enum class KeyConversionMode\s*\{([^}]+)\}/);
if (!kcm) throw new Error("未找到 v8::KeyConversionMode");
const kcmEntries = splitTop(kcm[1].replace(/\/\/.*$/gm, ""));
if (kcmEntries.join(",") !== "kConvertToString,kKeepNumbers,kNoNumbers") {
  throw new Error(`KeyConversionMode 条目失配: ${kcmEntries.join(",")}`);
}
// keys.h GetKeysConversion 条目顺序 (kKeepNumbers, kConvertToString, kNoNumbers)
// 与 v8 公开枚举位置 (kConvertToString=0, kKeepNumbers=1, kNoNumbers=2) 不同,
// 值取 v8 侧位置 1:1:
const v8Val = (name) => kcmEntries.indexOf(name);
const gkcValues = gkc.map((name) => v8Val(name));
if (gkcValues.join(",") !== "1,0,2") throw new Error(`GetKeysConversion 值失配: ${gkcValues.join(",")}`);

// KeyCollectionMode: 显式值 = v8 公开枚举值
const kcmRaw = splitTop(src.match(/enum class KeyCollectionMode\s*\{([\s\S]*?)\n\};/)[1]);
if (kcmRaw.length !== 2) throw new Error(`KeyCollectionMode 条数 ${kcmRaw.length} != 2`);
const kcm2 = kcmRaw.map((e) => {
  const m = e.match(/^(k\w+)\s*=\s*static_cast<int>\(v8::KeyCollectionMode::(k\w+)\)$/);
  if (!m) throw new Error(`KeyCollectionMode 条目不可解析: ${e}`);
  if (m[1] !== m[2]) throw new Error(`KeyCollectionMode 内外名失配: ${e}`);
  return m[1];
});
const kcmMode = v8Header.match(/enum class KeyCollectionMode\s*\{([^}]+)\}/);
if (!kcmMode) throw new Error("未找到 v8::KeyCollectionMode");
const modeEntries = splitTop(kcmMode[1].replace(/\/\/.*$/gm, ""));
if (modeEntries.join(",") !== "kOwnOnly,kIncludePrototypes") {
  throw new Error(`v8::KeyCollectionMode 条目失配: ${modeEntries.join(",")}`);
}
const kcm2Values = kcm2.map((name) => modeEntries.indexOf(name));
if (kcm2Values.join(",") !== "0,1") throw new Error(`KeyCollectionMode 值失配: ${kcm2Values.join(",")}`);

// IndexedOrNamed (KeyAccumulator 私有, 顺序值)
const ion = splitTop(src.match(/enum IndexedOrNamed\s*\{([^}]+)\}/)[1]);
if (ion.join(",") !== "kIndexed,kNamed") throw new Error("IndexedOrNamed 条目失配");

const pascal = (s) => s.replace(/^k/, "").toLowerCase().replace(/(^|_)([a-z0-9])/g, (_, p, c) => c.toUpperCase());

let rust = banner("src/objects/keys.h.js", "src/objects/keys.{h,cc}", "键收集枚举 (v8 公开枚举值 1:1 对齐)");
rust += `//! KeyAccumulator 键收集配置枚举。

/// 原版 enum AddKeyConversion (顺序值)。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum AddKeyConversion {
  /// DO_NOT_CONVERT
  DoNotConvert = 0,
  /// CONVERT_TO_ARRAY_INDEX
  ConvertToArrayIndex = 1,
}

/// 原版 enum class GetKeysConversion (值 = v8::KeyConversionMode)。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum GetKeysConversion {
`;
gkc.forEach((name, i) => {
  rust += `  /// ${name}\n  ${pascal(name)} = ${gkcValues[i]},\n`;
});
rust += `}

/// 原版 enum class KeyCollectionMode (值 = v8::KeyCollectionMode)。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum KeyCollectionMode {
`;
kcm2.forEach((name, i) => {
  rust += `  /// ${name}\n  ${pascal(name)} = ${kcm2Values[i]},\n`;
});
rust += `}

/// KeyAccumulator 私有分类 (原版 enum IndexedOrNamed)。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum IndexedOrNamed {
  /// kIndexed
  Indexed = 0,
  /// kNamed
  Named = 1,
}

// —— 提取不变式断言 ——
const _: () = {
  assert!(GetKeysConversion::KeepNumbers as u8 == 1);
  assert!(GetKeysConversion::ConvertToString as u8 == 0);
  assert!(GetKeysConversion::NoNumbers as u8 == 2);
  assert!(KeyCollectionMode::OwnOnly as u8 == 0);
  assert!(KeyCollectionMode::IncludePrototypes as u8 == 1);
};
`;

const p = writeSrc("v8-objects-rest", "keys.rs", rust);
console.log(`keys.rs: 4 枚举 2+3+2+2 变体, v8.h 公开枚举锚对数通过`);
