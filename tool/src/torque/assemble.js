#!/usr/bin/env bun
// v8-torque lib.rs / ls.rs 装配: 扫描各模块 pub 项生成导出清单 (其余 9 个脚本之后跑)
// crate AGENTS.md: 禁 pub mod (对外), 用 pub use 模块::{项}; ls 子模块 crate 内转发。
import { readdirSync, readFileSync, writeFileSync } from "node:fs";
import { join } from "node:path";

const root = join(import.meta.dir, "..", "..", "..");
const srcDir = join(root, "v8-torque", "src");
const lsDir = join(srcDir, "ls");

const harvest = (path) =>
  [...readFileSync(path, "utf8").matchAll(/^pub (?:struct|enum|fn|const|type|trait) (\w+)/gm)].map(
    (m) => m[1],
  );

const mods = readdirSync(srcDir)
  .filter((f) => f.endsWith(".rs") && !["lib.rs", "error.rs", "ls.rs"].includes(f))
  .map((f) => f.slice(0, -3))
  .sort();
const lsMods = readdirSync(lsDir)
  .filter((f) => f.endsWith(".rs"))
  .map((f) => f.slice(0, -3))
  .sort();

// 冲突检查
const seen = {};
for (const m of mods) {
  for (const it of harvest(join(srcDir, `${m}.rs`))) {
    if (seen[it]) throw new Error(`导出名冲突: ${it} @ ${seen[it]} 与 ${m}`);
    seen[it] = m;
  }
}
for (const m of lsMods) {
  for (const it of harvest(join(lsDir, `${m}.rs`))) {
    if (seen[it]) throw new Error(`导出名冲突: ${it} @ ${seen[it]} 与 ls/${m}`);
    seen[it] = m;
  }
}

const body = [
  "#![cfg_attr(docsrs, feature(doc_cfg))]",
  "",
  "mod error;",
  "pub use error::{Error, Result};",
];
for (const m of mods) {
  const items = harvest(join(srcDir, `${m}.rs`));
  body.push(`\nmod ${m};`);
  if (items.length) body.push(`pub use ${m}::{${items.join(", ")}};`);
}
const lsItems = lsMods.flatMap((m) => harvest(join(lsDir, `${m}.rs`)));
body.push("\nmod ls;");
body.push(`pub use ls::{${lsItems.join(", ")}};`);
writeFileSync(join(srcDir, "lib.rs"), `${body.join("\n")}\n`);

const lsLines = ["// LS 子模块装配: crate 内转发 (crate AGENTS.md: 禁 pub mod 对外)"];
for (const m of lsMods) {
  const items = harvest(join(lsDir, `${m}.rs`));
  lsLines.push(`pub(crate) mod ${m};`);
  if (items.length) lsLines.push(`pub use ${m}::{${items.join(", ")}};`);
}
writeFileSync(join(srcDir, "ls.rs"), `${lsLines.join("\n")}\n`);
console.log(`== v8-torque 装配完毕: ${mods.length} 模块 + ${lsMods.length} ls 子模块, ${Object.keys(seen).length} 个导出项 ==`);
