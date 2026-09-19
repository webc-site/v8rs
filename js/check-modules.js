#!/usr/bin/env -S bun

// 流水线第二阶段：cargo check 全量模块，把不通过模块显式记入剔除清单并生成 mod.rs
import { existsSync, readdirSync, writeFileSync } from "node:fs";
import { join } from "node:path";

export const root_dir = join(import.meta.dirname, ".."),
  gen_dir = join(root_dir, "v8rs", "src", "v8"),
  build_dir = join(root_dir, "cpp2rust", "build"),
  check_result = join(build_dir, "check-result.json");

const allow_li = [
  "#![allow(",
  "  clippy::all,",
  "  ambiguous_glob_reexports,",
  "  dead_code,",
  "  non_camel_case_types,",
  "  non_snake_case,",
  "  non_upper_case_globals,",
  "  unreachable_code,",
  "  unused_imports,",
  "  unused_labels,",
  "  unused_must_use,",
  "  unused_parens,",
  "  unused_variables",
  ")]\n",
];

// 写 mod.rs：仅纳入 passed 模块
export const write_mod_rs = (mods) => {
  const body = mods.map((m) => "pub mod " + m + ";\npub use " + m + "::*;");
  writeFileSync(join(gen_dir, "mod.rs"), allow_li.join("") + "\n" + body.join("\n") + "\n");
};

// 跑一轮 cargo check，返回模块名 → 首个编译错误摘要
export const cargo_check_errors = async () => {
  const proc = Bun.spawn(["cargo", "check", "--package", "v8rs", "--message-format=json"], {
    cwd: root_dir,
    stdout: "pipe",
    stderr: "pipe",
  });
  const output = await new Response(proc.stdout).text();
  const stderr = await new Response(proc.stderr).text();
  const code = await proc.exited;

  const errors = new Map();
  let cargo_broken = false;
  for (const line of output.split("\n")) {
    if (!line.trim()) continue;
    let msg;
    try {
      msg = JSON.parse(line);
    } catch {
      continue;
    }
    if (msg.reason === "compiler-message" && msg.message?.level === "error") {
      const text = msg.message.message || "";
      let hit = false;
      for (const span of msg.message.spans || []) {
        const file = span.file_name || "";
        if (file.includes("v8rs/src/v8/")) {
          const mod = file.split("v8rs/src/v8/")[1]?.replace(/\.rs$/, "");
          if (mod) {
            hit = true;
            if (!errors.has(mod)) errors.set(mod, text);
          }
        }
      }
      if (!hit && !errors.has("__outside__")) errors.set("__outside__", text);
    }
  }
  // cargo 自身失败（如依赖缺失）：错误都定位不到 v8 模块，属于流水线故障而非模块问题
  if (code !== 0 && errors.size === 0) {
    cargo_broken = true;
    console.error("cargo check 失败且无模块级错误定位:\n" + stderr.slice(0, 4000));
  }
  return { errors, code, cargo_broken };
};

// 不动点剔除：直到没有新的模块报错为止；每个模块的剔除原因写入结果
export const check_and_filter = async (candidates) => {
  const dropped = new Map();
  let current = [...candidates];
  while (true) {
    write_mod_rs(current);
    const { errors, cargo_broken } = await cargo_check_errors();
    if (cargo_broken) {
      console.error("cargo check 环境性失败，终止流水线（不剔除任何模块）");
      process.exit(2);
    }
    errors.delete("__outside__");
    const bad = current.filter((m) => errors.has(m));
    if (bad.length === 0) {
      // 剩余错误若非本目录模块导致，则视为已收敛
      if (errors.size > 0) {
        console.warn("警告: 存在不属于 v8rs/src/v8 的编译错误，可能影响下游:");
        for (const [k, v] of errors) console.warn("  " + k + ": " + v);
      }
      return { kept: current, dropped };
    }
    for (const m of bad) dropped.set(m, errors.get(m));
    const bad_set = new Set(bad);
    current = current.filter((m) => !bad_set.has(m));
    console.log("本轮剔除 " + bad.length + " 个编译不通过模块: " + bad.join(", "));
  }
};

if (import.meta.main) {
  if (!existsSync(gen_dir)) {
    console.error("gen_dir 不存在: " + gen_dir);
    process.exit(1);
  }
  const candidates = readdirSync(gen_dir)
    .filter((f) => f.endsWith(".rs") && f !== "mod.rs")
    .map((f) => f.replace(/\.rs$/, ""))
    .sort();
  const { kept, dropped } = await check_and_filter(candidates);
  const { mkdirSync } = await import("node:fs");
  mkdirSync(build_dir, { recursive: true });
  writeFileSync(
    check_result,
    JSON.stringify({ kept, dropped: Object.fromEntries(dropped) }, null, 2)
  );
  console.log("check 完成: 通过 " + kept.length + " / 候选 " + candidates.length);
}
