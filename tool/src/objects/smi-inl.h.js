#!/usr/bin/env bun
// tool/src/objects/smi-inl.h.js — Smi 内联空壳登记
// 组→文件映射 (本脚本管辖):
//   v8/src/objects/smi-inl.h → v8-objects-core/src/smi_inl.rs
//   (smi.h 主体与 objects.cc 的 kPowersOf10 表归 tool/src/objects/smi.h.js → smi.rs + smi.rs.skel)
// 规则面: smi-inl.h 是原版遗留空壳 (TODO(ishell): remove this file), 无任何定义 —
//   本脚本断言其为空并登记差集, 防止"空文件被误判为遗漏"
import { readV8, writeSrc, banner } from "../../lib.js";

const inl = readV8("src/objects/smi-inl.h");
const smiH = readV8("src/objects/smi.h");

// —— 空壳断言: 命名空间内零定义 ——
const ns = inl.match(/namespace (?:v8|internal) \{[\s\S]*?\}  \/\/ namespace (?:v8|internal)/g);
if (!ns || ns.length === 0) throw new Error("未找到命名空间块");
const inner = inl.slice(inl.indexOf("namespace internal"), inl.lastIndexOf("}  // namespace internal"));
const defs = [...inner.matchAll(/^(?:template <[^>]+>\s*)?\w[\w:<>*& ]*\s+\w+::\w+\(|^(?:inline|constexpr)\s+\w/gm)].length;
if (defs !== 0) throw new Error(`smi-inl.h 应为空壳 (0 定义), 实得 ${defs} — 上游已填充, 需重估席位`);
if (!inl.includes("// TODO(ishell): remove this file")) throw new Error("TODO(ishell) 移除标记缺失");

// —— smi.h 锚点核对 (差集归属面) ——
for (const anchor of ["class Smi", "kSmiShift", "static inline constexpr Tagged<Smi> zero()", "IsValid"])
  if (!smiH.includes(anchor) && !new RegExp(anchor).test(smiH)) throw new Error(`smi.h 缺锚点 ${anchor}`);

const body = `${banner("src/objects/smi-inl.h.js", "src/objects/smi-inl.h", "空壳登记: 0 定义 (原版 TODO 待删文件)")}
//! Smi 内联差集登记 (原版 smi-inl.h)
//!
//! 原版该文件仅剩 include 骨架与 \`TODO(ishell): remove this file\`,
//! 命名空间内 0 定义 — Smi 全部规则面落在 smi.h (→ smi.rs / smi.rs.skel).
//! 本模块存在意义: 函数级对账 (12.1) 时把该文件计为"已处理/空",
//! 避免误报遗漏.

/// 登记标记: smi-inl.h 为空壳 (对账时按 0 项计)
pub const SMI_INL_DEFINITIONS: usize = 0;
`;

const p = writeSrc("v8-objects-core", "smi_inl.rs", body);
console.log(`smi-inl.h.js: 已生成 ${p} (定义 ${defs}, 空壳断言通过)`);
