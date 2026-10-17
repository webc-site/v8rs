#!/usr/bin/env bun
// tool/src/builtins/arm64/builtins-arm64.cc.js — 机械提取 src/builtins/arm64/builtins-arm64.cc → v8-builtins-arm64/src/builtins_arm64.rs
// 产物为 C++ 规则代码骨架: 内建登记宏清单/枚举直译/类骨架, 函数体不展开;
// 语义实现不在脚本职责内, 由 Developer 修订环补齐。
import { spawnSync } from "node:child_process";
import { join } from "node:path";
import { readV8, writeSrc, banner, root } from "../../../lib.js";

// rustfmt 归一 (与聚合基线脚本一致), 保证门禁 fmt 检查通过
function fmt(body) {
  const r = spawnSync("rustfmt", ["--edition", "2024", "--config-path", join(root, "rustfmt.toml")], { input: body, encoding: "utf8" });
  if (r.status !== 0) throw new Error("rustfmt 归一失败: " + r.stderr);
  return r.stdout;
}

const V8RELS = ["src/builtins/arm64/builtins-arm64.cc"];
const CRATE = "v8-builtins-arm64";
const MODULE = "builtins_arm64.rs";
// 对数基准 (生成时核定; 原版漂移即 throw; 各源文件合计)
const EXPECT = {"builtins":0,"generates":71,"enums":2,"classes":4};
const ANCHORS = {"src/builtins/arm64/builtins-arm64.cc":["OsrSourceTier"]};

function maskCc(s) {
  let out = ""; let i = 0; const n = s.length;
  while (i < n) {
    const c = s[i];
    if (c === "/" && s[i + 1] === "/") { while (i < n && s[i] !== "\n") { out += " "; i++; } }
    else if (c === "/" && s[i + 1] === "*") {
      out += "  "; i += 2;
      while (i < n && !(s[i] === "*" && s[i + 1] === "/")) { out += s[i] === "\n" ? "\n" : " "; i++; }
      if (i < n) { out += "  "; i += 2; }
    } else if (c === '"' || c === "'") {
      const q = c; out += q; i++;
      while (i < n && s[i] !== q) {
        if (s[i] === "\\") { out += "  "; i += 2; } else { out += s[i] === "\n" ? "\n" : " "; i++; }
      }
      if (i < n) { out += q; i++; }
    } else { out += c; i++; }
  }
  return out;
}

function extractCc(src) {
  const m = maskCc(src);
  const builtins = [];
  for (const mm of m.matchAll(/\b(TF_BUILTIN|BUILTIN|TS_BUILTIN)\(\s*(\w+)\s*(?:,\s*([\w:<>]+))?/g)) {
    builtins.push({ macro: mm[1], name: mm[2], cls: mm[3] ?? "" });
  }
  const gens = [];
  for (const mm of m.matchAll(/Builtins::Generate_(\w+)\s*\(/g)) gens.push(mm[1]);
  const enums = [];
  for (const mm of m.matchAll(/\benum\s+(?:class\s+)?(\w+)\s*(?::\s*([\w: ]+?))?\s*\{([^{}]*)\}/g)) {
    const items = mm[3].split(",").map((s) => s.replace(/\s+/g, " ").trim()).filter(Boolean);
    enums.push({ name: mm[1], base: (mm[2] ?? "").trim(), items });
  }
  const classes = [];
  for (const mm of m.matchAll(/\bclass\s+(\w+)\s*(?::\s*public\s+([\w:<>]+))?\s*\{/g)) {
    classes.push({ name: mm[1], base: mm[2] ?? "" });
  }
  return { builtins, gens, enums, classes };
}

function rustStr(s) { return s.replace(/\\/g, "\\\\").replace(/"/g, '\\"'); }
function variantRust(v) {
  const mm = v.match(/^(\w+)\s*(?:=\s*(.+))?$/);
  if (!mm) return null;
  const name = mm[1].replace(/^k/, "").split(/[^A-Za-z0-9]+/).filter(Boolean)
    .map((w) => w.charAt(0).toUpperCase() + w.slice(1)).join("");
  return { name, val: (mm[2] ?? "").trim(), raw: mm[1] };
}

// —— 提取与对数守卫 ——
const builtins = [], gens = [], enums = [], classes = [];
for (const rel of V8RELS) {
  const src = readV8(rel);
  for (const a of ANCHORS[rel] ?? []) {
    if (!src.includes(a)) throw new Error(rel + " 锚点失配: " + a);
  }
  const r = extractCc(src);
  builtins.push(...r.builtins);
  gens.push(...r.gens);
  enums.push(...r.enums);
  classes.push(...r.classes);
}
if (builtins.length !== EXPECT.builtins) throw new Error("应 " + EXPECT.builtins + " 个内建登记 实际 " + builtins.length);
if (gens.length !== EXPECT.generates) throw new Error("应 " + EXPECT.generates + " 个 Generate_ 条目 实际 " + gens.length);
if (enums.length !== EXPECT.enums) throw new Error("应 " + EXPECT.enums + " 个枚举 实际 " + enums.length);
if (classes.length !== EXPECT.classes) throw new Error("应 " + EXPECT.classes + " 个类 实际 " + classes.length);

// —— Rust 产物 ——
let rust = banner("src/builtins/arm64/builtins-arm64.cc.js", V8RELS.join(" + "), "C++ 规则骨架 — 内建登记/枚举直译/类清单, 函数体不展开");
if (V8RELS.length > 1) rust += "//! 同名族合并脚本 (" + V8RELS.join(", ") + ")\n";
rust += "//! 任务映射: " + CRATE + "; 聚合基线脚本另产同源模块 (并存不覆盖)\n\n";

rust += "/// 内建实现种类 (汇编体推迟面占位: 登记原版 cc 面信息, 现波不填)\n#[derive(Debug, Clone, Copy)]\npub struct CcBuiltin {\n  pub macro_name: &'static str,\n  pub name: &'static str,\n  pub assembler: &'static str,\n}\n\n";
rust += "/// 内建登记清单 (共 " + builtins.length + " 条) — 汇编体推迟面占位\n///\n/// 空表是设计态而非漏填: 原版 `LIST_ASSEMBLER_BUILTINS` 类宏登记属汇编体波,\n/// 随 Generate_* 函数体填波一并派生; 名面权威清单见 ASM_GENERATE_ENTRIES。\n/// 推迟面台账见 readme/zh.md。\n";
rust += "pub const CC_BUILTINS: &[CcBuiltin] = &[\n";
for (const b of builtins) rust += '  CcBuiltin { macro_name: "' + b.macro + '", name: "' + b.name + '", assembler: "' + b.cls + '" },\n';
rust += "];\n\n";
if (gens.length) {
  rust += "/// 平台汇编入口 Builtins::Generate_<名> 清单 (共 " + gens.length + " 条)\npub const ASM_GENERATE_ENTRIES: &[&str] = &[\n";
  for (const g of gens) rust += '  "' + g + '",\n';
  rust += "];\n\n";
}
for (const e of enums) {
  const structName = e.name.replace(/^k/, "").split(/[^A-Za-z0-9]+/).filter(Boolean).map((w) => w.charAt(0).toUpperCase() + w.slice(1)).join("");
  rust += "/// 原版 enum " + e.name + (e.base ? " : " + e.base : "") + " 直译 (变体不带值, 语义由 Developer 修订)\n";
  rust += "#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, strum::Display, strum::EnumIter, strum::EnumString)]\n#[strum(serialize_all = \"PascalCase\")]\npub enum " + structName + " {\n";
  const seen = new Set();
  for (const item of e.items) {
    const v = variantRust(item);
    if (!v || seen.has(v.name)) continue;
    seen.add(v.name);
    rust += "  /// " + item + "\n  " + v.name + ",\n";
  }
  rust += "}\n\n";
}
if (classes.length) {
  rust += "/// 类骨架 (名, 基类)\npub const CLASSES: &[(&str, &str)] = &[\n";
  for (const c of classes) rust += '  ("' + c.name + '", "' + c.base + '"),\n';
  rust += "];\n\n";
}
rust += "// —— 对数断言 ——\nconst _: () = {\n";
rust += builtins.length === 0 ? "  assert!(CC_BUILTINS.is_empty());\n" : ("  assert!(CC_BUILTINS.len() == " + builtins.length + ");\n");
rust += "  assert!(ASM_GENERATE_ENTRIES.len() == " + gens.length + ");\n";
if (enums.length) rust += "  assert!(CLASSES.len() == " + classes.length + ");\n";
rust += "};\n";

const p = writeSrc(CRATE, MODULE, fmt(rust));
console.log("[" + V8RELS.join("+") + "] builtins=" + builtins.length + " generates=" + gens.length + " enums=" + enums.length + " classes=" + classes.length + " -> " + CRATE + "/src/" + MODULE);
