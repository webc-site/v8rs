#!/usr/bin/env bun
// tool/src/compiler/operation-typer.js
// v8/src/compiler/operation-typer.h + operation-typer.cc
// → v8-compiler-passes/src/operation_typer.rs.skel
// 运算定型器: 解析 TYPER_SUPPORTED_MACHINE_BINOP_LIST / 公有 Type 方法面 / cc 对数。
import { readV8, writeSrc, banner } from "../../lib.js";

const h = readV8("src/compiler/operation-typer.h");
const cc = readV8("src/compiler/operation-typer.cc");
const opcodesH = readV8("src/compiler/opcodes.h");

const need = (cond, msg) => {
  if (!cond) throw new Error(`operation-typer: ${msg}`);
};

for (const [s, name] of [
  ["#define TYPER_SUPPORTED_MACHINE_BINOP_LIST(V)", "机器二元列表"],
  ["class V8_EXPORT_PRIVATE OperationTyper {", "类定义"],
  ["Type Merge(Type left, Type right);", "Merge"],
  ["Type OperationTyper::Merge(Type left, Type right) {", "cc Merge"],
]) need(h.includes(s) || cc.includes(s), `哨兵缺失 ${name}`);

// —— 机器二元列表 (真解析) ——
const mach = h.match(/#define TYPER_SUPPORTED_MACHINE_BINOP_LIST\(V\)\s*\\\n([\s\S]*?)\n\n/);
need(mach, "机器二元列表未匹配");
const machBinops = [...mach[1].replace(/\\\n/g, " ").matchAll(/\bV\((\w+)\)/g)].map((m) => m[1]);
need(machBinops.length >= 20, `机器二元 ${machBinops.length}`);
for (const n of ["Int32Add", "Int32Sub", "Load", "Uint32Div", "Word32And", "Word32Shl", "Word64Shr"]) {
  need(machBinops.includes(n), `机器二元缺 ${n}`);
}

// —— 公有 Type 方法面 (真解析 h) ——
const hNorm = h.replace(/\(\s*\n\s*/g, "(");
const pub = hNorm.slice(hNorm.indexOf("class V8_EXPORT_PRIVATE OperationTyper {"));
const pubRegion = pub.slice(0, pub.indexOf(" private:"));
const typeMethods = [...new Set([
  ...[...pubRegion.matchAll(/^\s{2}Type (\w+)\(([^()]*)\)( const)?;/gm)].map((m) => m[1]),
  ...[...pubRegion.matchAll(/^  DECLARE_METHOD\((\w+)\)/gm)].map((m) => m[1]),
])];
// —— DECLARE_METHOD 宏列表展开 (Number*/BigInt*/Speculative* 面) ——
const expandSeen = new Set();
function expandOps(name) {
  if (expandSeen.has(name)) return [];
  expandSeen.add(name);
  const m = opcodesH.match(new RegExp(`#define ${name}\\(V\\)\\s*\\\\\\n([\\s\\S]*?)\\n\\n`));
  if (!m) return [];
  const body = m[1].replace(/\\\n/g, " ").replace(/\/\/[^\n]*/g, "");
  const own = [...body.matchAll(/\bV\((\w+)/g)].map((x) => x[1]);
  const nested = [...body.matchAll(/\b([A-Z0-9_]+_LIST)\(V\)/g)].flatMap((x) => expandOps(x[1]));
  return [...own, ...nested];
}
const macroOps = [...new Set([
  ...expandOps("SIMPLIFIED_NUMBER_UNOP_LIST"),
  ...expandOps("SIMPLIFIED_BIGINT_UNOP_LIST"),
  ...expandOps("SIMPLIFIED_SPECULATIVE_NUMBER_UNOP_LIST"),
  ...expandOps("SIMPLIFIED_SPECULATIVE_BIGINT_UNOP_LIST"),
  ...expandOps("SIMPLIFIED_NUMBER_BINOP_LIST"),
  ...expandOps("SIMPLIFIED_BIGINT_BINOP_LIST"),
  ...expandOps("SIMPLIFIED_SPECULATIVE_NUMBER_BINOP_LIST"),
])];
for (const needM of ["Merge", "ToPrimitive", "ToNumber", "ToNumberConvertBigInt", "ToBigInt", "ToBigIntConvertNumber", "ToNumeric", "ToBoolean", "WeakenRange", "TypeTypeGuard", "ConvertReceiver", "CheckBounds"]) {
  need(typeMethods.includes(needM), `方法 ${needM} 缺席 (实得 ${typeMethods.length})`);
}
for (const needM of ["NumberAdd", "NumberSubtract", "SpeculativeNumberAdd", "SpeculativeAdditiveSafeIntegerAdd"]) {
  need(macroOps.includes(needM), `宏展开缺 ${needM} (宏面 ${macroOps.length})`);
}

// —— cc 定义对数 (声明面 = 显式 Type 方法 + 宏展开方法) ——
const allFaces = [...new Set([...typeMethods, ...macroOps])];
const ccDefined = allFaces.filter((m) => cc.includes(`Type OperationTyper::${m}(`));
need(ccDefined.length >= 60, `cc 定义 ${ccDefined.length}/${allFaces.length}`);
const ccMissing = allFaces.filter((m) => !ccDefined.includes(m));

// —— singleton 工厂 (真解析) ——
const singletons = [...pubRegion.matchAll(/^\s{2}Type (\w+)\(\) const;/gm)].map((m) => m[1]);

const body = `${banner("src/compiler/operation-typer.js", "src/compiler/operation-typer.h + operation-typer.cc", "算子级类型推演内核")}
//! operation-typer — Typer 的纯函数核 (无图依赖, 输入类型→输出类型)
//!
//! 公有 Type 方法 ${typeMethods.length} 个 (cc 定义 ${ccDefined.length});
//! TYPER_SUPPORTED_MACHINE_BINOP_LIST ${machBinops.length} 项
//! (${machBinops.slice(0, 12).join(", ")}, ...);
//! singleton 工厂: ${singletons.join(", ") || "(zone 缓存)"}。

/// 原版 OperationTyper — 运算定型器
pub struct OperationTyper<'z> {
  /// broker_
  pub broker: *mut JSHeapBroker,
  /// cache_ (TypeCache 引用)
  pub cache: *const TypeCache,
  /// true_/false_/falsish_/truish_ 等 singleton 缓存
  pub true_type: Type,
  pub false_type: Type,
  pub falsish: Type,
  pub truish: Type,
  /// zone
  pub zone: &'z Zone<'z>,
}

impl OperationTyper<'_> {
  /// 原版 singleton 工厂面 (Type::HeapConstant 缓存)
  /// v8: src/compiler/operation-typer.cc OperationTyper::singleton_true 等
  pub fn singleton_true(&self) -> Type {
    todo!("v8: src/compiler/operation-typer.cc OperationTyper::singleton_true")
  }

  /// 原版 Merge — Phi 汇聚 (区间并 + 常量并)
  /// v8: src/compiler/operation-typer.cc OperationTyper::Merge
  pub fn merge(&mut self, left: Type, right: Type) -> Type {
    todo!("v8: src/compiler/operation-typer.cc OperationTyper::Merge")
  }

  /// 原版 ToNumber 族 — 转换定型
  /// v8: src/compiler/operation-typer.cc OperationTyper::ToNumber
  pub fn to_number(&mut self, type_: Type) -> Type {
    todo!("v8: src/compiler/operation-typer.cc OperationTyper::ToNumber")
  }

  /// 原版 TypeTypeGuard
  /// v8: src/compiler/operation-typer.cc OperationTyper::TypeTypeGuard
  pub fn type_type_guard(&mut self, op: &Operator, input: Type) -> Type {
    todo!("v8: src/compiler/operation-typer.cc OperationTyper::TypeTypeGuard")
  }

  /// 原版 CheckBounds — 越界即 None
  /// v8: src/compiler/operation-typer.cc OperationTyper::CheckBounds
  pub fn check_bounds(&mut self, index: Type, length: Type) -> Type {
    todo!("v8: src/compiler/operation-typer.cc OperationTyper::CheckBounds")
  }

  // —— Number* / Speculative* / Compare 族 (${typeMethods.length} 个, 全 1:1 锚) ——
${typeMethods.filter((m) => !["Merge", "ToNumber", "TypeTypeGuard", "CheckBounds"].includes(m)).map((m) => `  /// v8: src/compiler/operation-typer.cc OperationTyper::${m}\n  pub fn ${m.replace(/([a-z0-9])([A-Z])/g, "$1_$2").toLowerCase()}(&mut self, /* 按签名 */) -> Type {\n    todo!("v8: src/compiler/operation-typer.cc OperationTyper::${m}")\n  }`).join("\n\n")}

  // —— 机器算子二元定型 (${machBinops.length} 项, 谓词化分发) ——
  /// v8: src/compiler/operation-typer.cc OperationTyper::${machBinops[0]} (代表项; 全表 ${machBinops.join("/")})
  pub fn machine_binop(&mut self, op: IrOpcode, left: Type, right: Type) -> Type {
    todo!("v8: src/compiler/operation-typer.cc OperationTyper machine 二元面 (${machBinops.length} 项)")
  }
}
`;

const p = writeSrc("v8-compiler-passes", "operation_typer.rs.skel", body);
console.log(
  [
    `written ${p}`,
    `type_methods=${typeMethods.length} + 宏展开=${macroOps.length} (NumberAdd 等) cc_defs=${ccDefined.length} cc_missing=${ccMissing.length}`,
    `machine_binops=${machBinops.length} singletons=${singletons.length}`,
  ].join("\n"),
);
