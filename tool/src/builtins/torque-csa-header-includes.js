#!/usr/bin/env bun
// tool/src/builtins/torque-csa-header-includes.js — 机械提取 v8/src/builtins/torque-csa-header-includes.h
// 产物: include 依赖登记表 (Torque 生成的 CSA 头公共前置)。
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

const H = "src/builtins/torque-csa-header-includes.h";
// 对数基准 (生成时核定): include 条目数
const EXPECT = { includes: 6 };

const src = readV8(H);
const includes = [...src.matchAll(/^#include "([^"]+)"/gm)].map((m) => m[1]);
if (includes.length !== EXPECT.includes) throw new Error(`应 ${EXPECT.includes} 条 include 实际 ${includes.length}`);

let rust = banner("src/builtins/torque-csa-header-includes.js", H, "include 依赖登记 (Torque 生成 CSA 头的公共前置)");
rust += "//! 任务映射: v8-builtins-misc; 聚合基线另归 v8-builtins-core (并存不覆盖)\n\n";

rust += "/// 原版 include 清单 (顺序保持)\npub const INCLUDES: &[&str] = &[\n";
for (const i of includes) rust += `  "${i}",\n`;
rust += "];\n\n";

rust += "/// 该头本身无规则代码 (仅 include 聚合), 迁移语义为零\npub const RULE_CODE_ITEMS: usize = 0;\n\n";

rust += "// —— 对数断言 ——\nconst _: () = {\n";
rust += `  assert!(INCLUDES.len() == ${includes.length});\n`;
rust += "  assert!(INCLUDES[0] == \"src/builtins/builtins-promise.h\");\n";
rust += "};\n";

const p = writeSrc("v8-builtins-misc", "torque_csa_header_includes.rs", fmt(rust));
console.log(`[torque-csa-header-includes.h] includes=${includes.length} -> ${p}`);
