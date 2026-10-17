#!/usr/bin/env bun
// tool/src/builtins/string-startswith.tq.js — 机械提取 v8/src/builtins/string-startswith.tq → v8-builtins-string/src/string_startswith_tq.rs
// \u4EA7\u7269\u4E3A Torque \u58F0\u660E\u9AA8\u67B6 (namespace/type/const/macro/builtin/struct/enum/class), \u51FD\u6570\u4F53\u4E0D\u5C55\u5F00;
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

const V8REL = "src/builtins/string-startswith.tq";
const CRATE = "v8-builtins-string";
const MODULE = "string_startswith_tq.rs";
// \u5BF9\u6570\u57FA\u51C6 (\u751F\u6210\u65F6\u6838\u5B9A; \u539F\u7248\u6F02\u79FB\u5373 throw)
const EXPECT = {"total":2,"byKind":{"namespace":1,"builtin":1}};
const ANCHORS = ["namespace string","StringPrototypeStartsWith"];

// \u2014\u2014 Torque \u9876\u5C42\u58F0\u660E\u63D0\u53D6: \u6CE8\u91CA/\u5B57\u7B26\u4E32\u63A9\u7801 + \u62EC\u53F7\u6DF1\u5EA6\u72B6\u6001\u673A \u2014\u2014
function maskTorque(s) {
  let out = ""; let i = 0; const n = s.length;
  while (i < n) {
    const c = s[i];
    if (c === "/" && s[i + 1] === "/") { while (i < n && s[i] !== "\n") { out += " "; i++; } }
    else if (c === "/" && s[i + 1] === "*") {
      out += "  "; i += 2;
      while (i < n && !(s[i] === "*" && s[i + 1] === "/")) { out += s[i] === "\n" ? "\n" : " "; i++; }
      if (i < n) { out += "  "; i += 2; }
    } else if (c === "'" || c === '"') {
      const q = c; out += q; i++;
      while (i < n && s[i] !== q) {
        if (s[i] === "\\") { out += "  "; i += 2; } else { out += s[i] === "\n" ? "\n" : " "; i++; }
      }
      if (i < n) { out += q; i++; }
    } else { out += c; i++; }
  }
  return out;
}

function extractDecls(src) {
  const m = maskTorque(src);
  const n = m.length;
  const pa = new Int32Array(n + 1);
  const bb = new Int32Array(n + 1);
  let a = 0, b = 0;
  for (let i = 0; i < n; i++) {
    if (m[i] === "(") a++; else if (m[i] === ")") a--;
    else if (m[i] === "{") b++; else if (m[i] === "}") b--;
    pa[i + 1] = a; bb[i + 1] = b;
  }
  const re = /(?:^|\n)[ \t]*((?:@[A-Za-z_][A-Za-z0-9_]*[ \t]*(?:\([^)]*\))?[ \t]*\n?[ \t]*)*)(?:(extern|transitioning|javascript|torque)[ \t]+)*(namespace|type|const|enum|struct|class|macro|builtin|import)\b/g;
  const hits = [];
  let mm;
  while ((mm = re.exec(m)) !== null) {
    const kwStart = mm.index + mm[0].length - mm[3].length;
    hits.push({ kw: mm[3], kwStart, annos: [...(mm[1] ?? "").matchAll(/@([A-Za-z_][A-Za-z0-9_]*)/g)].map((x) => x[1]), mods: [...(mm[2] ?? "").matchAll(/\b(extern|transitioning|javascript|torque)\b/g)].map((x) => x[1]) });
  }
  const decls = [];
  let nsLevel = 0;
  let skipUntil = -1;
  for (const h of hits) {
    if (h.kwStart <= skipUntil) continue;
    if (h.kw === "namespace") {
      let braceAt = -1;
      for (let i = h.kwStart; i < n; i++) if (m[i] === "{" && bb[i] === nsLevel) { braceAt = i; break; }
      if (braceAt < 0) continue;
      const nameM = m.slice(h.kwStart, braceAt).match(/namespace\s+([A-Za-z_][A-Za-z0-9_]*)/);
      decls.push({ kw: "namespace", name: nameM?.[1] ?? "", sig: "namespace " + (nameM?.[1] ?? ""), annos: [], mods: [] });
      nsLevel = bb[braceAt + 1];
      continue;
    }
    if (bb[h.kwStart] !== nsLevel) continue;
    let end = -1, isBrace = false;
    for (let i = h.kwStart; i < n; i++) {
      if (m[i] === ";" && bb[i] === nsLevel && pa[i] === pa[h.kwStart]) { end = i; break; }
      if (m[i] === "{" && bb[i] === nsLevel) { end = i; isBrace = true; break; }
    }
    if (end < 0) continue;
    const sig = src.slice(h.kwStart, end).replace(/\s+/g, " ").trim();
    const nmm = sig.match(new RegExp("\\b" + h.kw + "\\s+([A-Za-z_][A-Za-z0-9_]*(?:::[A-Za-z_][A-Za-z0-9_]*)?)"));
    decls.push({ kw: h.kw, name: nmm?.[1] ?? "", sig, annos: h.annos, mods: h.mods, js: h.mods.includes("javascript") });
    if (isBrace) {
      let d = 0, j = end;
      for (; j < n; j++) {
        if (m[j] === "{") d++;
        else if (m[j] === "}") { d--; if (d === 0) break; }
      }
      skipUntil = j;
    }
  }
  return decls;
}

// \u2014\u2014 \u63D0\u53D6\u4E0E\u5BF9\u6570\u5B88\u536B \u2014\u2014
const src = readV8(V8REL);
for (const anchor of ANCHORS) {
  if (!src.includes(anchor)) throw new Error(V8REL + " \u951A\u70B9\u5931\u914D: " + anchor);
}
const decls = extractDecls(src);
const byKind = {};
for (const d of decls) byKind[d.kw] = (byKind[d.kw] ?? 0) + 1;
if (decls.length !== EXPECT.total) {
  throw new Error(V8REL + " \u5E94 " + EXPECT.total + " \u6761\u58F0\u660E \u5B9E\u9645 " + decls.length);
}
for (const [k, n] of Object.entries(EXPECT.byKind)) {
  const got = byKind[k] ?? 0;
  if (got !== n) throw new Error(V8REL + " \u5E94 " + n + " \u4E2A " + k + " \u5B9E\u9645 " + got);
}

// \u2014\u2014 \u5E38\u91CF\u5206\u7C7B: C++ \u7B26\u53F7\u5F15\u7528 / \u6570\u503C\u00B7\u5B57\u7B26\u4E32\u5B57\u9762\u91CF\u76F4\u8BD1 \u2014\u2014
function rustStr(s) { return s.replace(/\\/g, "\\\\").replace(/"/g, '\\"'); }
function constRustName(k) { return k.replace(/^k/, "").replace(/([a-z0-9])([A-Z])/g, "$1_$2").replace(/([A-Z]+)([A-Z][a-z])/g, "$1_$2").toUpperCase(); }
const cppGen = [];
const literals = [];
for (const d of decls) {
  const g = d.sig.match(/generates '([^']+)'/);
  if (g) cppGen.push([d.name, g[1]]);
  if (d.kw !== "const" || g) continue;
  const num = d.sig.match(/^const (\w+):\s*[\w:. <>]+?=\s*(-?\d+|0[xX][0-9a-fA-F]+)$/);
  const str = d.sig.match(/^const (\w+):\s*[\w:. <>]+?=\s*'([^']*)'$/);
  if (num) literals.push({ name: num[1], expr: num[2], ty: "i64" });
  else if (str) literals.push({ name: str[1], expr: '"' + rustStr(str[2]) + '"', ty: "&'static str" });
}

// \u2014\u2014 Rust \u4EA7\u7269 \u2014\u2014
let rust = banner("src/builtins/string-startswith.tq.js", V8REL, "Torque \u58F0\u660E\u9AA8\u67B6 \u2014 \u58F0\u660E\u6E05\u5355/\u5E38\u91CF\u76F4\u8BD1/C++ \u7B26\u53F7\u5F15\u7528, \u51FD\u6570\u4F53\u4E0D\u5C55\u5F00");
rust += "//! \u4EFB\u52A1\u6620\u5C04: " + CRATE + "; \u805A\u5408\u57FA\u7EBF\u811A\u672C\u53E6\u4EA7\u540C\u6E90\u6A21\u5757 (\u5E76\u5B58\u4E0D\u8986\u76D6)\n\n";
rust += "/// Torque \u58F0\u660E\u6761\u76EE (kind: namespace/type/const/macro/builtin/struct/enum/class/import)\n";
rust += "#[derive(Debug, Clone, Copy)]\npub struct TqDecl {\n  pub kind: &'static str,\n  pub name: &'static str,\n  /// \u539F\u7248\u7B7E\u540D (\u7A7A\u767D\u89C4\u6574; \u542B labels/js-implicit \u7B49\u4FEE\u9970)\n  pub sig: &'static str,\n}\n\n";
rust += "/// \u539F\u7248\u9876\u5C42\u58F0\u660E\u6E05\u5355 (\u5171 " + decls.length + " \u6761)\npub const DECLS: &[TqDecl] = &[\n";
for (const d of decls) {
  rust += '  ("' + d.kw + '", "' + rustStr(d.name) + '", "' + rustStr(d.sig.slice(0, 240)) + (d.sig.length > 240 ? "\u2026" : "") + '"),\n';
}
rust += "];\n\n";
if (cppGen.length) {
  rust += "/// constexpr ... generates 'C++ \u7B26\u53F7' \u5F15\u7528\u767B\u8BB0 (\u673A\u68B0\u767B\u8BB0, \u4E0D\u731C\u503C)\npub const CPP_GENERATES: &[(&str, &str)] = &[\n";
  for (const [n, c] of cppGen) rust += '  ("' + rustStr(n) + '", "' + rustStr(c) + '"),\n';
  rust += "];\n\n";
}
if (literals.length) {
  rust += "/// \u5B57\u9762\u91CF\u5E38\u91CF\u76F4\u8BD1 (\u539F\u7248 const <\u540D>: <\u7C7B\u578B> = <\u5B57\u9762\u91CF>)\n";
  for (const l of literals) rust += "pub const " + constRustName(l.name) + ": " + l.ty + " = " + l.expr + "; // \u539F\u7248: " + rustStr(l.name) + "\n";
  rust += "\n";
}
rust += "// \u2014\u2014 \u5BF9\u6570\u65AD\u8A00 \u2014\u2014\nconst _: () = {\n  assert!(DECLS.len() == " + decls.length + ");\n";
for (const [k, n] of Object.entries(EXPECT.byKind)) {
  rust += '  assert!(DECLS.iter().filter(|d| d.kind == "' + k + '").count() == ' + n + ');\n';
}
rust += "};\n";

const p = writeSrc(CRATE, MODULE, fmt(rust));
console.log("[" + V8REL + "] DECLS=" + decls.length + " (" + Object.entries(byKind).map(([k, v]) => k + ":" + v).join(" ") + ") -> " + p);
