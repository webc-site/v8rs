#!/usr/bin/env bun
// tool/src/debug/synthetic-module.js — 覆盖 v8/src/objects/synthetic-module.{h,-inl.h,cc,tq}
// 组→文件映射 (本脚本管辖): 席位表 4 文件全族
// 产物: v8-debug/src/synthetic_module.rs
// 机械提取: SyntheticModule 4 字段布局 (h ↔ tq 交叉) + -inl.h 访问器面 + .cc 方法签名骨架

import { readV8, writeSrc, banner } from "../../lib.js";

const h = readV8("src/objects/synthetic-module.h");
const inl = readV8("src/objects/synthetic-module-inl.h");
const cc = readV8("src/objects/synthetic-module.cc");
const tq = readV8("src/objects/synthetic-module.tq");

let checks = 0;
const must = (c, m) => { checks++; if (!c) throw new Error(`[synthetic-module.js] ${m}`); };

// ── 1. 布局: h TaggedMember ↔ tq extern class 交叉 (4 字段) ──────────────
const hMembers = [...h.matchAll(/^  TaggedMember<([^;]+)> (\w+);$/gm)].map((m) => [m[2], m[1].trim()]);
must(hMembers.length === 4, `h TaggedMember 数 ${hMembers.length} != 4`);
must(hMembers.map((f) => f[0]).join(",") === "name_,export_names_,evaluation_steps_,host_defined_options_", "h 布局漂移");
must(hMembers[0][1] === "String" && hMembers[1][1] === "FixedArray" && hMembers[2][1] === "Foreign" && hMembers[3][1] === "Object",
  "h 成员类型漂移 (String/FixedArray/Foreign/Object)");

const cls = tq.match(/extern class SyntheticModule extends Module \{([\s\S]*?)\}/);
must(cls, "tq 缺 SyntheticModule");
const tqFields = [...cls[1].matchAll(/^  (\w+): ([^;]+);$/gm)].map((m) => [m[1], m[2].trim()]);
must(tqFields.length === 4, `tq 字段数 ${tqFields.length} != 4`);
must(tqFields.map((f) => f[0]).join(",") === "name,export_names,evaluation_steps,host_defined_options", "tq 布局漂移");
// 双侧逐字段对齐 (去下划线后)
for (let i = 0; i < 4; i++) {
  must(hMembers[i][0].replace(/_$/, "") === tqFields[i][0], `h/tq 字段 ${i} 名失配: ${hMembers[i][0]} vs ${tqFields[i][0]}`);
}

// ── 2. .h 方法面锚 ───────────────────────────────────────────────────────
for (const s of [
  "static Maybe<bool> SetExport(", "static void SetExportStrict(",
  "inline Tagged<String> name() const;", "inline Tagged<FixedArray> export_names() const;",
  "inline Tagged<Foreign> evaluation_steps() const;", "inline Tagged<Object> host_defined_options() const;",
  "static V8_WARN_UNUSED_RESULT MaybeHandle<Cell> ResolveExport(",
  "static V8_WARN_UNUSED_RESULT bool PrepareInstantiate(",
  "static V8_WARN_UNUSED_RESULT bool FinishInstantiate(",
  "static V8_WARN_UNUSED_RESULT MaybeDirectHandle<JSPromise> Evaluate(",
]) must(h.includes(s), `synthetic-module.h 缺 ${s}`);
must(h.includes("class SyntheticModule : public Module"), "h 继承漂移");

// ── 3. -inl.h 访问器面锚 (4 get + 4 set) ─────────────────────────────────
for (const s of ["SyntheticModule::name() const", "SyntheticModule::export_names() const",
  "SyntheticModule::evaluation_steps() const", "SyntheticModule::host_defined_options() const",
  "SyntheticModule::set_name(", "SyntheticModule::set_export_names(",
  "SyntheticModule::set_evaluation_steps(", "SyntheticModule::set_host_defined_options("]) {
  must(inl.includes(s), `synthetic-module-inl.h 缺 ${s}`);
}

// ── 4. .cc 方法签名骨架 (源码序提取) ──────────────────────────────────────
const ccSigs = [...cc.matchAll(/^(?:Maybe<bool>|MaybeHandle<\w+>|MaybeDirectHandle<\w+>|bool|void)\s+SyntheticModule::(\w+)\(/gm)]
  .map((m) => m[1]);
must(ccSigs.length === 6, `.cc 方法数 ${ccSigs.length} != 6`);
must(ccSigs.join(",") === "SetExport,SetExportStrict,ResolveExport,PrepareInstantiate,FinishInstantiate,Evaluate",
  `cc 方法序漂移: ${ccSigs.join(",")}`);
// cc 语义锚: 求值步骤经 Foreign 地址调用 + 状态迁移
must(cc.includes("module->evaluation_steps()->foreign_address<kSyntheticModuleTag>()"), "cc 缺 evaluation_steps Foreign 调用锚");
for (const st of ["SetStatus(kLinked)", "SetStatus(kEvaluating)", "SetStatus(kEvaluated)"]) {
  must(cc.includes(st), `cc 缺状态迁移 ${st}`);
}

const snake = (s) => s.replace(/([a-z0-9])([A-Z])/g, "$1_$2").toLowerCase();

let rust = banner("src/debug/synthetic-module.js", "src/objects/synthetic-module.{h,-inl.h,cc,tq}",
  "全族: Synthetic Module Record 布局 + 生命周期方法骨架");
rust += `//! SyntheticModule: 嵌入器构造的合成模块 (WebIDL Synthetic Module Record; debug 域)。
//!
//! 布局 4 槽 + 模块生命周期 (SetExport/ResolveExport/Instantiate/Evaluate) 骨架。

// ── 布局 (源码序; h TaggedMember 与 tq extern class 1:1) ──
pub const SYNTHETIC_MODULE_FIELDS: [(&str, &str); ${tqFields.length}] = [
`;
for (const f of tqFields) rust += `  ("${f[0]}", "${f[1]}"),\n`;
rust += `];

// ── .cc 方法签名骨架 (${ccSigs.length} 个, 源码序; 语义就地修订) ──
`;
for (const m of ccSigs) {
  rust += `/// 原版 SyntheticModule::${m} (synthetic-module.cc)。
pub fn ${snake(m)}() {
  todo!("就地修订: synthetic-module.cc ${m}")
}

`;
}
rust += `/// 编译期字符串相等 (str 的 const PartialEq 尚未稳定, 手写逐字节比较)
const fn str_eq(a: &str, b: &str) -> bool {
  let (a, b) = (a.as_bytes(), b.as_bytes());
  if a.len() != b.len() {
    return false;
  }
  let mut i = 0;
  while i < a.len() {
    if a[i] != b[i] {
      return false;
    }
    i += 1;
  }
  true
}

// ── 提取不变式断言 ──
const _: () = {
  assert!(SYNTHETIC_MODULE_FIELDS.len() == 4);
  assert!(str_eq(SYNTHETIC_MODULE_FIELDS[0].0, "name"));
  assert!(str_eq(SYNTHETIC_MODULE_FIELDS[0].1, "String"));
  assert!(str_eq(SYNTHETIC_MODULE_FIELDS[2].1, "Foreign"));
  assert!(str_eq(SYNTHETIC_MODULE_FIELDS[3].1, "Object"));
};
`;

const p = writeSrc("v8-debug", "synthetic_module.rs", rust);
console.log(`synthetic-module.js: ${p} — 布局 ${tqFields.length} 字段 (h/tq 双侧对齐)、.cc 方法 ${ccSigs.length}, 断言 ${checks} 项全过`);
