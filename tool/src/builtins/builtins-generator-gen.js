#!/usr/bin/env bun
// tool/src/builtins/builtins-generator-gen.js — 机械提取 src/builtins/builtins-generator-gen.cc → v8-builtins-promise/src/builtins_generator_gen.rs
// \u4EA7\u7269\u4E3A C++ \u89C4\u5219\u4EE3\u7801\u9AA8\u67B6: \u5185\u5EFA\u767B\u8BB0\u5B8F\u6E05\u5355/\u679A\u4E3E\u76F4\u8BD1/\u7C7B\u9AA8\u67B6, \u51FD\u6570\u4F53\u4E0D\u5C55\u5F00;
// \u8BED\u4E49\u5B9E\u73B0\u4E0D\u5728\u811A\u672C\u804C\u8D23\u5185, \u7531 Developer \u4FEE\u8BA2\u73AF\u8865\u9F50\u3002
import { spawnSync } from "node:child_process";
import { join } from "node:path";
import { readV8, writeSrc, banner, root } from "../../lib.js";

// rustfmt \u5F52\u4E00 (\u4E0E\u805A\u5408\u57FA\u7EBF\u811A\u672C\u4E00\u81F4), \u4FDD\u8BC1\u95E8\u7981 fmt \u68C0\u67E5\u901A\u8FC7
function fmt(body) {
  const r = spawnSync("rustfmt", ["--edition", "2024", "--config-path", join(root, "rustfmt.toml")], { input: body, encoding: "utf8" });
  if (r.status !== 0) throw new Error("rustfmt \u5F52\u4E00\u5931\u8D25: " + r.stderr);
  return r.stdout;
}

const V8RELS = ["src/builtins/builtins-generator-gen.cc"];
const CRATE = "v8-builtins-promise";
const MODULE = "builtins_generator_gen.rs";
// \u5BF9\u6570\u57FA\u51C6 (\u751F\u6210\u65F6\u6838\u5B9A; \u539F\u7248\u6F02\u79FB\u5373 throw; \u5404\u6E90\u6587\u4EF6\u5408\u8BA1)
const EXPECT = {"builtins":7,"generates":0,"enums":0,"classes":1};
const ANCHORS = {"src/builtins/builtins-generator-gen.cc":["AsyncModuleEvaluate"]};

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

// \u2014\u2014 \u63D0\u53D6\u4E0E\u5BF9\u6570\u5B88\u536B \u2014\u2014
const builtins = [], gens = [], enums = [], classes = [];
for (const rel of V8RELS) {
  const src = readV8(rel);
  for (const a of ANCHORS[rel] ?? []) {
    if (!src.includes(a)) throw new Error(rel + " \u951A\u70B9\u5931\u914D: " + a);
  }
  const r = extractCc(src);
  builtins.push(...r.builtins);
  gens.push(...r.gens);
  enums.push(...r.enums);
  classes.push(...r.classes);
}
if (builtins.length !== EXPECT.builtins) throw new Error("\u5E94 " + EXPECT.builtins + " \u4E2A\u5185\u5EFA\u767B\u8BB0 \u5B9E\u9645 " + builtins.length);
if (gens.length !== EXPECT.generates) throw new Error("\u5E94 " + EXPECT.generates + " \u4E2A Generate_ \u6761\u76EE \u5B9E\u9645 " + gens.length);
if (enums.length !== EXPECT.enums) throw new Error("\u5E94 " + EXPECT.enums + " \u4E2A\u679A\u4E3E \u5B9E\u9645 " + enums.length);
if (classes.length !== EXPECT.classes) throw new Error("\u5E94 " + EXPECT.classes + " \u4E2A\u7C7B \u5B9E\u9645 " + classes.length);

// \u2014\u2014 Rust \u4EA7\u7269 \u2014\u2014
let rust = banner("src/builtins/builtins-generator-gen.js", V8RELS.join(" + "), "C++ \u89C4\u5219\u9AA8\u67B6 \u2014 \u5185\u5EFA\u767B\u8BB0/\u679A\u4E3E\u76F4\u8BD1/\u7C7B\u6E05\u5355, \u51FD\u6570\u4F53\u4E0D\u5C55\u5F00");
if (V8RELS.length > 1) rust += "//! \u540C\u540D\u65CF\u5408\u5E76\u811A\u672C (" + V8RELS.join(", ") + ")\n";
rust += "//! \u4EFB\u52A1\u6620\u5C04: " + CRATE + "; \u805A\u5408\u57FA\u7EBF\u811A\u672C\u53E6\u4EA7\u540C\u6E90\u6A21\u5757 (\u5E76\u5B58\u4E0D\u8986\u76D6)\n\n";

rust += "/// \u5185\u5EFA\u5B9E\u73B0\u79CD\u7C7B\n#[derive(Debug, Clone, Copy)]\npub struct CcBuiltin {\n  pub macro_name: &'static str,\n  pub name: &'static str,\n  pub assembler: &'static str,\n}\n\n";
rust += "/// \u5185\u5EFA\u767B\u8BB0\u6E05\u5355 (\u5171 " + builtins.length + " \u6761)\npub const CC_BUILTINS: &[CcBuiltin] = &[\n";
for (const b of builtins) rust += '  CcBuiltin { macro_name: "' + b.macro + '", name: "' + b.name + '", assembler: "' + b.cls + '" },\n';
rust += "];\n\n";
if (gens.length) {
  rust += "/// \u5E73\u53F0\u6C47\u7F16\u5165\u53E3 Builtins::Generate_<\u540D> \u6E05\u5355 (\u5171 " + gens.length + " \u6761)\npub const ASM_GENERATE_ENTRIES: &[&str] = &[\n";
  for (const g of gens) rust += '  "' + g + '",\n';
  rust += "];\n\n";
}
for (const e of enums) {
  const structName = e.name.replace(/^k/, "").split(/[^A-Za-z0-9]+/).filter(Boolean).map((w) => w.charAt(0).toUpperCase() + w.slice(1)).join("");
  rust += "/// \u539F\u7248 enum " + e.name + (e.base ? " : " + e.base : "") + " \u76F4\u8BD1 (\u53D8\u4F53\u4E0D\u5E26\u503C, \u8BED\u4E49\u7531 Developer \u4FEE\u8BA2)\n";
  rust += "#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]\npub enum " + structName + " {\n";
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
  rust += "/// \u7C7B\u9AA8\u67B6 (\u540D, \u57FA\u7C7B)\npub const CLASSES: &[(&str, &str)] = &[\n";
  for (const c of classes) rust += '  ("' + c.name + '", "' + c.base + '"),\n';
  rust += "];\n\n";
}
rust += "// \u2014\u2014 \u5BF9\u6570\u65AD\u8A00 \u2014\u2014\nconst _: () = {\n";
rust += "  assert!(CC_BUILTINS.len() == " + builtins.length + ");\n";
rust += "  assert!(ASM_GENERATE_ENTRIES.len() == " + gens.length + ");\n";
if (enums.length) rust += "  assert!(CLASSES.len() == " + classes.length + ");\n";
rust += "};\n";

const p = writeSrc(CRATE, MODULE, fmt(rust));
console.log("[" + V8RELS.join("+") + "] builtins=" + builtins.length + " generates=" + gens.length + " enums=" + enums.length + " classes=" + classes.length + " -> " + CRATE + "/src/" + MODULE);
