#!/usr/bin/env bun
// tool/src/builtins/builtins-utils-inl.js — 机械提取 v8/src/builtins/builtins-utils-inl.h
// 产物: BuiltinArguments 内联方法签名骨架 (4 条, 函数体不展开)。
// 任务映射: v8-builtins-misc (聚合基线脚本将其归于 v8-builtins-core, 两处并存不覆盖)。
import { spawnSync } from "node:child_process";
import { join } from "node:path";
import { readV8, writeSrc, banner, root } from "../../lib.js";

// rustfmt 归一 (与聚合基线脚本一致)
function fmt(body) {
  const r = spawnSync("rustfmt", ["--edition", "2024", "--config-path", join(root, "rustfmt.toml")], { input: body, encoding: "utf8" });
  if (r.status !== 0) throw new Error("rustfmt 归一失败: " + r.stderr);
  return r.stdout;
}

const H = "src/builtins/builtins-utils-inl.h";
// 对数基准 (生成时核定)
const EXPECT = { methods: 4 };
const ANCHORS = ["BuiltinArguments::atOrUndefined", "BuiltinArguments::new_target"];

const src = readV8(H);
for (const a of ANCHORS) if (!src.includes(a)) throw new Error(H + " 锚点失配: " + a);

// 方法签名: 返回类型 + BuiltinArguments::名( 参数 (多行签名拼接至 ")" 前)
const sigs = [];
const re = /^(Handle<[\w]+>|DirectHandle<[\w]+>)\s+BuiltinArguments::(\w+)\(/gm;
let m;
while ((m = re.exec(src)) !== null) {
  // 签名延伸到参数表右括号 (含 const 尾注)
  let depth = 0, j = src.indexOf("(", m.index);
  let end = j;
  for (; j < src.length; j++) {
    if (src[j] === "(") depth++;
    else if (src[j] === ")") { depth--; if (depth === 0) { end = j + 1; break; } }
  }
  let sig = src.slice(m.index, end).replace(/\s+/g, " ").trim();
  const tail = src.slice(end, end + 8);
  if (tail.startsWith(" const")) sig += " const";
  sigs.push({ ret: m[1], name: m[2], sig });
}
if (sigs.length !== EXPECT.methods) throw new Error(`应 ${EXPECT.methods} 个 BuiltinArguments 方法 实际 ${sigs.length}`);

let rust = banner("src/builtins/builtins-utils-inl.js", H, "BuiltinArguments 内联方法签名骨架, 函数体不展开");
rust += "//! 任务映射: v8-builtins-misc; 聚合基线另归 v8-builtins-core (并存不覆盖)\n\n";

rust += "/// 方法签名条目\n#[derive(Debug, Clone, Copy)]\npub struct MethodSig {\n  pub ret: &'static str,\n  pub name: &'static str,\n  /// 原版签名 (空白规整)\n  pub sig: &'static str,\n}\n\n";
rust += `/// BuiltinArguments 内联方法清单 (共 ${sigs.length} 条)\npub const METHODS: &[MethodSig] = &[\n`;
for (const s of sigs) {
  const esc = s.sig.replace(/\\/g, "\\\\").replace(/"/g, "\\\"");
  rust += `  MethodSig { ret: "${s.ret}", name: "${s.name}", sig: "${esc}" },\n`;
}
rust += "];\n\n";

rust += "// —— 对数断言 ——\nconst _: () = {\n";
rust += `  assert!(METHODS.len() == ${sigs.length});\n`;
rust += "  assert!(METHODS[0].name == \"atOrUndefined\");\n";
rust += "};\n";

const p = writeSrc("v8-builtins-misc", "builtins_utils_inl.rs", fmt(rust));
console.log(`[builtins-utils-inl.h] methods=${sigs.length} -> ${p}`);
