#!/usr/bin/env bun
// tool/src/builtins/data-view-ops.js — 机械提取 v8/src/builtins/data-view-ops.h 的 DATAVIEW_OP_LIST X-macro
// 展开 DataViewOp 枚举 (21 变体, 显式顺序值) 与 ToString 映射; 语义实现由 Developer 修订环补齐。
// 任务映射: v8-builtins-misc (聚合基线脚本将其归于 v8-builtins-typedarray/buffer, 两处并存不覆盖)。
import { spawnSync } from "node:child_process";
import { join } from "node:path";
import { readV8, writeSrc, banner, root } from "../../lib.js";

// rustfmt 归一 (与聚合基线脚本一致)
function fmt(body) {
  const r = spawnSync("rustfmt", ["--edition", "2024", "--config-path", join(root, "rustfmt.toml")], { input: body, encoding: "utf8" });
  if (r.status !== 0) throw new Error("rustfmt 归一失败: " + r.stderr);
  return r.stdout;
}

const H = "src/builtins/data-view-ops.h";
// 对数基准 (生成时核定): 10 种操作 → 20 getter/setter 变体 + 1 byteLength
const EXPECT = { ops: 10, variants: 21 };

const src = readV8(H);
if (!src.includes("#define DATAVIEW_OP_LIST(V)")) throw new Error(H + " 未找到 DATAVIEW_OP_LIST");

// 宏体行收集 (续行 \)
const lines = src.split("\n");
const bodyLines = [];
let on = false;
for (const l of lines) {
  if (l.startsWith("#define DATAVIEW_OP_LIST(V)")) { on = true; continue; }
  if (on) {
    if (l.trimEnd().endsWith("\\")) { bodyLines.push(l); continue; }
    bodyLines.push(l);
    break;
  }
}
const ops = bodyLines.map((l) => (l.match(/V\((\w+)\)/) ?? [])[1]).filter(Boolean);
if (ops.length !== EXPECT.ops) throw new Error(`应 ${EXPECT.ops} 个 DataView 操作 实际 ${ops.length}`);

// X-macro 展开: kGet<Op>, kSet<Op> ×N + kByteLength (C++ enum 无显式值, 顺序即值)
const variants = [];
for (const op of ops) {
  variants.push({ name: `kGet${op}`, doc: `DataView.prototype.get${op}` });
  variants.push({ name: `kSet${op}`, doc: `DataView.prototype.set${op}` });
}
variants.push({ name: "kByteLength", doc: "get DataView.prototype.byteLength" });
if (variants.length !== EXPECT.variants) throw new Error(`应 ${EXPECT.variants} 个 DataViewOp 变体 实际 ${variants.length}`);

// —— Rust 产物 ——
let rust = banner("src/builtins/data-view-ops.js", H, "DATAVIEW_OP_LIST X-macro 机械展开 — 枚举变体带顺序值 + ToString 映射");
rust += "//! 任务映射: v8-builtins-misc; 聚合基线另归 v8-builtins-typedarray (并存不覆盖)\n\n";

rust += "/// DataView 已知导入操作名 — X-macro DATAVIEW_OP_LIST(V) 展开\n";
rust += "pub const DATAVIEW_OP_LIST: &[&str] = &[\n";
for (const op of ops) rust += `  "${op}",\n`;
rust += "];\n\n";

rust += `/// 原版 enum DataViewOp : uint8_t — X-macro 机械展开 (变体顺序即判别值, 共 ${variants.length} 个)\n`;
rust += "#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]\n#[repr(u8)]\npub enum DataViewOp {\n";
variants.forEach((v, i) => {
  rust += `  /// ${v.name} (${v.doc})\n  ${v.name.replace(/^k/, "")} = ${i},\n`;
});
rust += "}\n\n";

rust += "impl DataViewOp {\n  /// 变体总数\n  pub const COUNT: usize = " + variants.length + ";\n\n";
rust += "  /// 原版 ToString(DataViewOp) — 机械展开 switch 表\n  #[inline]\n  #[must_use]\n  pub const fn to_str(self) -> &'static str {\n    match self {\n";
for (const v of variants) {
  const rn = v.name.replace(/^k/, "");
  rust += `      Self::${rn} => "${v.doc}",\n`;
}
rust += "    }\n  }\n}\n\n";

rust += `/// 操作数 (不含 kByteLength)\npub const OP_COUNT: usize = ${ops.length};\n\n`;

rust += "// —— 对数断言 ——\nconst _: () = {\n";
rust += `  assert!(DATAVIEW_OP_LIST.len() == ${ops.length});\n`;
rust += `  assert!(DataViewOp::COUNT == ${variants.length});\n`;
rust += `  assert!(OP_COUNT == ${ops.length});\n`;
rust += "  assert!(DataViewOp::GetBigInt64 as u8 == 0);\n";
rust += `  assert!(DataViewOp::ByteLength as u8 == ${variants.length - 1});\n`;
rust += "  assert!(DataViewOp::SetUint32.to_str() == \"DataView.prototype.setUint32\");\n";
rust += "};\n";

const p = writeSrc("v8-builtins-misc", "data_view_ops.rs", fmt(rust));
console.log(`[data-view-ops.h] ops=${ops.length} variants=${variants.length} -> ${p}`);
