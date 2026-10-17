#!/usr/bin/env bun
// tool/src/compiler/turboshaft/access-builder.h.js
// 机械提取 v8/src/compiler/turboshaft/access-builder.h
// → v8-turboshaft/src/access_builder.rs.skel
//
// 提取面: AccessBuilderTS 静态工厂方法表 (ForXxx) + 宏展开区
// (DECL_FIELD_ACCESS/DECL_ELEMENT_ACCESS) + FieldAccessTS/ElementAccessTS
// 类型别名。语义实现禁编造。
import { readV8, writeSrc, banner } from "../../../lib.js";

const h = readV8("src/compiler/turboshaft/access-builder.h");

// 1. 宏区: 两个声明宏 (字段访问 / 元素访问)
const fieldMacro = h.match(/#define (TF_FIELD_ACCESS)\(\w+, \w+, \w+\)\s*\\[\s\S]*?static FieldAccessTS/);
if (!fieldMacro) throw new Error("未找到字段访问宏 (DECL_FIELD_ACCESS)");
const elemMacro = h.match(/#define (TF_ELEMENT_ACCESS)\(\w+, \w+, \w+\)\s*\\[\s\S]*?static ElementAccessTS/);
if (!elemMacro) throw new Error("未找到元素访问宏");

// 宏展开调用计数 (宏名(驼峰名) 形式)
const fieldMacroName = fieldMacro[1];
const elemMacroName = elemMacro[1];
const fieldCalls = [...h.matchAll(new RegExp(`${fieldMacroName}\\((\\w+)\\)`, "g"))].filter((m) => m[0].includes(fieldMacroName + "(") && !m[0].startsWith("#"));
const elemCalls = [...h.matchAll(new RegExp(`${elemMacroName}\\((\\w+)\\)`, "g"))];
// 过滤宏定义行 (#define 自身)
const fieldUses = [...h.matchAll(new RegExp(`^\\s*${fieldMacroName}\\(\\w+\\)`, "gm"))].length;
const elemUses = [...h.matchAll(new RegExp(`^\\s*${elemMacroName}\\(\\w+\\)`, "gm"))].length;

// 2. 静态方法面 (含宏展开外的直接定义)
const methods = [...h.matchAll(/static (?:FieldAccessTS|ElementAccessTS)[^;{]*?(For\w+)\(/g)].map((m) => m[1]);
const uniqMethods = [...new Set(methods)];
// 对数: 直接定义方法 ≥ 宏调用 (部分方法由宏展开)
if (uniqMethods.length < fieldUses + elemUses) {
  throw new Error(`For 方法 ${uniqMethods.length} < 宏展开 ${fieldUses}+${elemUses}`);
}

// 3. 关键锚 (从原文核对的方法子集)
const anchors = ["ForMap", "ForFeedbackVectorLength", "ForCellMaybeValue", "ForPropertyCellValue",
  "ForJSArrayLength", "ForContextSlot", "ForSymbolFlags", "ForFixedArrayElement"];
for (const a of anchors) {
  if (!h.includes(a)) throw new Error(`锚方法缺失: ${a}`);
}

// 4. 类型别名与基类面
for (const [re, label] of [
  [/struct FieldAccessTS : public compiler::FieldAccess/, "FieldAccessTS 基类"],
  [/struct ElementAccessTS : public compiler::ElementAccess/, "ElementAccessTS 基类"],
  [/class AccessBuilderTS : public AllStatic/, "AccessBuilderTS : AllStatic"],
  [/using JSReceiverOrNull = UnionOf<JSReceiver, Null>;/, "JSReceiverOrNull 别名"],
  [/using HeapNumberOrOddball = Union<HeapNumber, Oddball>;/, "HeapNumberOrOddball 别名"],
  [/class AccessBuilderTS;/, "前向声明"],
]) {
  if (!re.test(h)) throw new Error(`哨兵缺失: ${label}`);
}

// 5. 生成 Rust
let rust = banner(
  "src/compiler/turboshaft/access-builder.h.js",
  "src/compiler/turboshaft/access-builder.h",
  "访问描述符工厂: 字段/元素访问方法面登记",
);
rust += `
/// v8: src/compiler/turboshaft/access-builder.h AccessBuilderTS
///
/// 全静态工厂: 产出 FieldAccessTS/ElementAccessTS (TS 侧带类型参数 T)。
/// 方法面: ${uniqMethods.length} 个 ForXxx (直接定义) + ${fieldUses} 字段宏 + ${elemUses} 元素宏展开。
/// 宏: ${fieldMacroName} / ${elemMacroName}
pub struct AccessBuilderTs;

impl AccessBuilderTs {
`;
for (const m of uniqMethods) {
  const snakeName = m.replace(/^For/, "").replace(/([a-z0-9])([A-Z])/g, "$1_$2").toLowerCase();
  rust += `  /// 原版 static ... ${m}(...)
  /// v8: src/compiler/turboshaft/access-builder.h AccessBuilderTS::${m}
  pub fn ${snakeName}() -> () {
    unimplemented!("签名骨架, Developer 修订")
  }
`;
}
rust += `}

// —— 提取不变式断言 ——
const _: () = {
  assert!(${uniqMethods.length} > 0); // For 方法面非空
};
`;

const p = writeSrc("v8-turboshaft", "access_builder.rs.skel", rust);
console.log(`written ${p}`);
console.log(`类=AccessBuilderTS For方法=${uniqMethods.length} 宏展开=${fieldUses}(字段)+${elemUses}(元素) 锚=${anchors.length}`);
