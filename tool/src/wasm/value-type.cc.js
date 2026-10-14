#!/usr/bin/env bun
// tool/src/wasm/value-type.cc.js
// 机械提取 v8/src/wasm/value-type.cc 依赖的名称表与骨架:
// FOREACH_NUMERIC_VALUE_TYPE / FOREACH_ABSTRACT_TYPE(=NONE+TOP+确定类型) 名称表,
// null-ref 专用名表, CanonicalSig 骨架, 签名工具骨架。
// 宏定义来自 v8/src/wasm/value-type.h (头文件侧已由 value-type.h.js 覆盖)。

import { readV8, writeSrc, banner } from "../../lib.js";

const hSrc = readV8("src/wasm/value-type.h");
const ccSrc = readV8("src/wasm/value-type.cc");

// ---------- 宏体提取 (合并续行, 去注释) ----------
function macroBody(src, name) {
  const lines = src.split("\n");
  for (let i = 0; i < lines.length; i++) {
    const m = lines[i].match(new RegExp(`^#define\\s+${name}\\(V\\)\\s*(.*)$`));
    if (!m) continue;
    let body = m[1];
    while (body.endsWith("\\")) body = body.slice(0, -1) + " " + (lines[++i] ?? "").trim();
    return body.replace(/\/\*[\s\S]*?\*\//g, " ");
  }
  throw new Error(`未找到宏 ${name}`);
}

// FOREACH_NUMERIC_VALUE_TYPE: V(kind, log2Size, code, machineType, shortName, name)
const numeric = [...macroBody(hSrc, "FOREACH_NUMERIC_VALUE_TYPE").matchAll(
  /V\(\s*(\w+)\s*,\s*(-?\d+)\s*,\s*(\w+)\s*,\s*(\w+)\s*,\s*'(.)'\s*,\s*"([^"]+)"\s*\)/g,
)].map((m) => ({ kind: m[1], log2: Number(m[2]), code: m[3], machine: m[4], short: m[5], name: m[6] }));
if (numeric.length !== 8) throw new Error(`数值类型 ${numeric.length} 条, 期望 8`);
if (numeric[0].kind !== "I32" || numeric[numeric.length - 1].kind !== "F16") {
  throw new Error("FOREACH_NUMERIC_VALUE_TYPE 锚点漂移");
}

// FOREACH_NONE_TYPE / FOREACH_TOP_TYPE / FOREACH_ABSTRACT_TYPE 自有条目
// V(kName, ValueTypeCode, extra_bits, printable_name)
function abstractEntries(name) {
  return [...macroBody(hSrc, name).matchAll(
    /V\(\s*(\w+)\s*,\s*(\w+)\s*,\s*([A-Za-z0-9_ |]+?)\s*,\s*"([^"]+)"\s*\)/g,
  )].map((m) => ({ kind: m[1], code: m[2], extra: m[3].replace(/\s+/g, ""), name: m[4] }));
}
const noneTypes = abstractEntries("FOREACH_NONE_TYPE");
const topTypes = abstractEntries("FOREACH_TOP_TYPE");
const ownAbstract = abstractEntries("FOREACH_ABSTRACT_TYPE");
if (noneTypes.length !== 6) throw new Error(`FOREACH_NONE_TYPE ${noneTypes.length} 条, 期望 6`);
if (topTypes.length !== 6) throw new Error(`FOREACH_TOP_TYPE ${topTypes.length} 条, 期望 6`);
if (ownAbstract.length !== 8) throw new Error(`FOREACH_ABSTRACT_TYPE 自有条目 ${ownAbstract.length}, 期望 8`);
const abstractTypes = [...noneTypes, ...topTypes, ...ownAbstract]; // FOREACH_ABSTRACT_TYPE 展开序

// FOREACH_INTERNAL_TYPE
const internalTypes = abstractEntries("FOREACH_INTERNAL_TYPE");
if (internalTypes.length !== 4) throw new Error(`FOREACH_INTERNAL_TYPE ${internalTypes.length} 条, 期望 4`);

console.log(
  `数值 ${numeric.length}, 抽象 ${abstractTypes.length}, 内部 ${internalTypes.length}`,
);

// ---------- .cc 侧锚点 ----------
const anchors = [
  ["kWasmVoid.is_void()", ccSrc.includes("kWasmVoid.is_void()")],
  ["kWasmBottom.is_bottom()", ccSrc.includes("kWasmBottom.is_bottom()")],
  ["kWasmTop.is_top()", ccSrc.includes("kWasmTop.is_top()")],
  ["kWasmNullRef.is_none_or_bottom()", ccSrc.includes("kWasmNullRef.is_none_or_bottom()")],
  ["ToZeroBasedIndex(kI32)==0", ccSrc.includes("0 == ToZeroBasedIndex(NumericKind::kI32)")],
];
for (const [n, ok] of anchors) {
  if (!ok) throw new Error(`静态断言锚点缺失: ${n}`);
}

let rust = banner(
  "src/wasm/value-type.cc.js",
  "src/wasm/value-type.{h,cc}",
  "ValueType 名称表 + null-ref 别名表 + CanonicalSig 骨架 (头侧枚举在 value_type.rs)",
);

rust += `
use crate::value_type::{GenericKind, NumericKind};
use crate::wasm_constants::ValueTypeCode;

/// 数值类型名表 (对应 FOREACH_NUMERIC_VALUE_TYPE: kind/log2/code/machine/short/name)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NumericTypeInfo {
  pub kind: NumericKind,
  /// log2Size
  pub log2_size: i8,
  /// ValueTypeCode 记号 (kI32Code 等)
  pub code: ValueTypeCode,
  /// MachineType 记号 (Int32 等, 仅存档)
  pub machine: &'static str,
  /// shortName 字符
  pub short_name: char,
  /// 打印名 ("i32"/"v128" 等)
  pub name: &'static str,
}

/// FOREACH_NUMERIC_VALUE_TYPE 展开表 (源顺序, ${numeric.length} 条)
#[rustfmt::skip]
pub static NUMERIC_TYPE_INFOS: &[NumericTypeInfo] = &[
`;

const numVariant = (k) => `NumericKind::${k}`;
for (const n of numeric) {
  rust += `  NumericTypeInfo { kind: ${numVariant(n.kind)}, log2_size: ${n.log2}, code: ValueTypeCode::${n.code}, machine: "${n.machine}", short_name: '${n.short}', name: "${n.name}" },\n`;
}

rust += `];

/// 抽象类型名表 (对应 FOREACH_ABSTRACT_TYPE 展开: NONE+TOP+自有, ${abstractTypes.length} 条)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AbstractTypeInfo {
  pub kind: GenericKind,
  /// ValueTypeCode 记号
  pub code: &'static str,
  /// 位组合记号 (REF/FUNC/STRUCT/ARRAY/CONT, 仅存档)
  pub extra_bits: &'static str,
  /// 打印名
  pub name: &'static str,
}

#[rustfmt::skip]
pub static ABSTRACT_TYPE_INFOS: &[AbstractTypeInfo] = &[
`;

const gkVariant = (k) => `GenericKind::${k.charAt(0).toUpperCase()}${k.slice(1)}`;
for (const a of abstractTypes) {
  rust += `  AbstractTypeInfo { kind: ${gkVariant(a.kind)}, code: "${a.code}", extra_bits: "${a.extra}", name: "${a.name}" },\n`;
}

rust += `];

/// 内部类型名表 (对应 FOREACH_INTERNAL_TYPE, ${internalTypes.length} 条)
#[rustfmt::skip]
pub static INTERNAL_TYPE_INFOS: &[(&str, &str)] = &[
`;

for (const t of internalTypes) {
  rust += `  ("${t.kind}", "${t.name}"),\n`;
}

rust += `];

/// null-ref 系别名表 (对应 PrintGenericHeaptypeName / name() 特判: kNone 等可空类型打印为 nullXXref)
/// (generic, 共享前缀打印名, 非共享打印名)
pub static NULL_REF_NAMES: &[(GenericKind, &str, &str)] = &[
  (GenericKind::None, "(shared nullref)", "nullref"),
  (GenericKind::NoExn, "(shared nullexnref)", "nullexnref"),
  (GenericKind::NoExtern, "(shared nullexternref)", "nullexternref"),
  (GenericKind::NoFunc, "(shared nullfuncref)", "nullfuncref"),
  (GenericKind::NoCont, "(shared nullcontref)", "nullcontref"),
];

/// 生成器侧对数: null-ref 表与抽象表一致性
const _: () = {
  assert!(NUMERIC_TYPE_INFOS.len() == 8);
  assert!(ABSTRACT_TYPE_INFOS.len() == 20);
  assert!(INTERNAL_TYPE_INFOS.len() == 4);
  assert!(NULL_REF_NAMES.len() == 5);
  assert!(NUMERIC_TYPE_INFOS[0].name == "i32");
  assert!(NUMERIC_TYPE_INFOS[4].name == "v128");
  assert!(NUMERIC_TYPE_INFOS[7].name == "f16");
  assert!(ABSTRACT_TYPE_INFOS[0].name == "nocont");
  assert!(ABSTRACT_TYPE_INFOS[6].name == "func");
  assert!(ABSTRACT_TYPE_INFOS[19].name == "stringview_iter");
  assert!(INTERNAL_TYPE_INFOS[0].0 == "Void");
};

/// 从规范签名取返回类型 (对应 WasmReturnTypeFromSignature, 骨架)
pub fn wasm_return_type_from_signature(
  _sig: &crate::wasm_opcodes::SigDef,
) -> Option<crate::wasm_opcodes::SigType> {
  unimplemented!("骨架: 参照原版 value-type.cc WasmReturnTypeFromSignature 修订")
}

/// 数值签名等价判定 (对应 EquivalentNumericSig, 骨架)
pub fn equivalent_numeric_sig(_a: &crate::wasm_opcodes::SigDef, _b: &crate::wasm_opcodes::SigDef) -> bool {
  unimplemented!("骨架: 参照原版 value-type.cc EquivalentNumericSig 修订")
}

/// 将 i64 参数/返回成对替换为 i32 (对应 GetI32Sig, 骨架)
pub fn get_i32_sig(_sig: &crate::wasm_opcodes::SigDef) -> crate::wasm_opcodes::SigDef {
  unimplemented!("骨架: 参照原版 value-type.cc GetI32Sig (ReplaceTypeInSig i64→2xi32) 修订")
}
`;

const p = writeSrc("v8-wasm-core", "value_type_cc.rs", rust);
console.log(`written ${p}`);
