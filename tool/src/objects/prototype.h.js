#!/usr/bin/env bun
// tool/src/objects/prototype.h.js — 覆盖 v8/src/objects/prototype.{h,-inl.h} (同名族合一)
// → v8-objects-maps/src/prototype.rs
// 机械提取: WhereToEnd 枚举 + PrototypeIterator 字段/方法签名骨架
//   (-inl 函数体留 todo! + /// v8: 锚)
// 对数 oracle: clang -E 预处理 v8/src/objects/prototype-inl.h (定义清单计数)

import { execFileSync } from "node:child_process";
import { mkdirSync, writeFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { tmpdir } from "node:os";
import { readV8, writeSrc, banner, root } from "../../lib.js";

const h = readV8("src/objects/prototype.h");
const inl = readV8("src/objects/prototype-inl.h");

let checks = 0;
const must = (c, m) => {
  checks++;
  if (!c) throw new Error(`锚点失配: ${m}`);
};

// ── clang -E 对数基准 (原版侧 = 编译器预处理产物; 缺失头自动空 stub) ──────
const ppShim = join(tmpdir(), "v8rs-r3o4-pp-shim");
function clangE(rel) {
  const args = [
    "-E", "-P", "-x", "c++", "-std=c++20", "-I", "v8", "-I", "v8/include",
    "-I", ppShim, "-DV8_INTL_SUPPORT=1", "-DV8_ENABLE_WEBASSEMBLY=1", "-",
  ];
  for (let round = 0; round < 48; round++) {
    try {
      return execFileSync("clang", args, {
        input: `#include "${rel}"\n`,
        encoding: "utf8",
        cwd: root,
        maxBuffer: 512 * 1024 * 1024,
      });
    } catch (e) {
      const err = String(e.stderr ?? "");
      const m = err.match(/fatal error: '([^']+)' file not found/);
      if (!m) throw new Error(`clang -E ${rel} 失败: ${err.slice(0, 300)}`);
      const stub = join(ppShim, m[1]);
      mkdirSync(dirname(stub), { recursive: true });
      writeFileSync(stub, "#pragma once\n");
    }
  }
  throw new Error(`clang -E ${rel}: stub 轮数超限`);
}

// ── 产物 fmt 归一 (与 cargo +nightly fmt 同配置; 幂等) ────────────────────
function fmtNormalize(p) {
  const rustfmt = execFileSync("rustup", ["which", "--toolchain", "nightly", "rustfmt"],
    { encoding: "utf8", cwd: root }).trim();
  execFileSync(rustfmt, ["--edition", "2024", "--config-path", join(root, "rustfmt.toml"), p],
    { cwd: root });
}


const inlpp = clangE("src/objects/prototype-inl.h");

// ── 1. WhereToEnd 枚举 (原文锚 + pp 锚) ──────────────────────────────────
must(h.includes("enum WhereToEnd { END_AT_NULL, END_AT_NON_HIDDEN };"), "原文 WhereToEnd 失配");
must(inlpp.includes("enum WhereToEnd { END_AT_NULL, END_AT_NON_HIDDEN };"), "pp WhereToEnd 失配");

// ── 2. 构造器/方法定义清单 (pp 列首定义行计数; 剥 __attribute__ + 去行首缩进) ─
const defs = [...inlpp.replace(/__attribute__\(\(.*?\)\)/g, "")
  .split("\n").map((l) => l.replace(/^\s+/, "")).join("\n")
  .matchAll(/^(?:[A-Za-z_][\w:<>,&* ]*? )?(PrototypeIterator::\w+)\(/gm)]
  .map((m) => m[1]);
must(defs.length === 10, `pp 定义数 ${defs.length} != 10 (4 构造器 + 6 方法)`);
const mult = {};
for (const d of defs) mult[d] = (mult[d] ?? 0) + 1;
for (const [k, v] of Object.entries({
  "PrototypeIterator::PrototypeIterator": 4,
  "PrototypeIterator::HasAccess": 1,
  "PrototypeIterator::Advance": 1,
  "PrototypeIterator::AdvanceIgnoringProxies": 1,
  "PrototypeIterator::AdvanceFollowingProxies": 1,
  "PrototypeIterator::AdvanceFollowingProxiesIgnoringAccessChecks": 1,
})) must(mult[k] === v, `pp ${k} 计数 ${mult[k] ?? 0} != ${v}`);

// ── 3. 字段清单 (原文锚) ─────────────────────────────────────────────────
for (const a of [
  "Isolate* isolate_;",
  "Tagged<JSPrototype> object_ = {};",
  "IndirectHandle<JSPrototype> handle_;",
  "WhereToEnd where_to_end_;",
  "bool is_at_end_;",
  "int seen_proxies_;",
]) must(h.includes(a), `原文缺字段锚: ${a}`);

// ── 4. 原文行号锚 (-inl 定义行, 供 /// v8: 锚) ────────────────────────────
const lineOf = {};
inl.split("\n").forEach((l, i) => {
  const m = l.match(/^(?:[A-Za-z_][\w:<>,&* ]*? )?PrototypeIterator::(\w+)\(/);
  if (m && lineOf[m[1]] === undefined) lineOf[m[1]] = i + 1;
});
for (const k of ["PrototypeIterator", "HasAccess", "Advance", "AdvanceIgnoringProxies",
  "AdvanceFollowingProxies", "AdvanceFollowingProxiesIgnoringAccessChecks"])
  must(lineOf[k] !== undefined, `原文缺行号锚: ${k}`);

let rust = banner("src/objects/prototype.h.js", "src/objects/prototype.{h,-inl.h}",
  "PrototypeIterator 枚举/字段/方法骨架 (clang -E 对数; 函数体留 todo!)");
rust += `//! PrototypeIterator: 统一访问任意对象的原型并走原型链。
//! 骨架基线; handlified (Isolate/Handle) 语义面留 Developer 修订。

/// 终止条件 (原版 enum WhereToEnd)。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum WhereToEnd {
  /// 走到 null 原型 (END_AT_NULL)。
  EndAtNull = 0,
  /// 走到首个非 hidden 原型 (END_AT_NON_HIDDEN, 跳过全局代理)。
  EndAtNonHidden = 1,
}

/// 迭代器字段骨架 (object_/handle_ 双轨: 原文 Tagged 与 IndirectHandle 二选一)。
pub struct PrototypeIterator {
  /// 原文 isolate_ (Isolate*; 语境句柄, 骨架期不落地)。
  /// 原文 object_ (Tagged<JSPrototype>, Tagged 轨)。
  /// 原文 handle_ (IndirectHandle<JSPrototype>, handlified 轨)。
  /// 终止条件。
  pub where_to_end: WhereToEnd,
  /// 是否已到链尾。
  pub is_at_end: bool,
  /// 已访问代理数 (防 __proto__ 递归)。
  pub seen_proxies: i32,
}

impl PrototypeIterator {
  /// v8: prototype-inl.h:${lineOf.PrototypeIterator ?? 0} — PrototypeIterator::PrototypeIterator(isolate, DirectHandle<JSReceiver>, where_to_start, where_to_end)
  pub fn new_handle(_isolate: &str, _receiver: usize, _where_to_start: u8, _where_to_end: WhereToEnd) -> Self {
    todo!()
  }

  /// v8: prototype-inl.h:${lineOf.Advance ?? 0} — void PrototypeIterator::Advance()
  ///
  /// 步进一格; JSProxy 直接置尾 (null)。
  pub fn advance(&mut self) {
    todo!()
  }

  /// v8: prototype-inl.h:${lineOf.AdvanceIgnoringProxies ?? 0} — void PrototypeIterator::AdvanceIgnoringProxies()
  ///
  /// 步进一格 (不做代理特判)。
  pub fn advance_ignoring_proxies(&mut self) {
    todo!()
  }

  /// v8: prototype-inl.h:${lineOf.AdvanceFollowingProxies ?? 0} — bool PrototypeIterator::AdvanceFollowingProxies()
  ///
  /// 返回 false iff JSProxy::GetPrototype 抛异常。
  pub fn advance_following_proxies(&mut self) -> bool {
    todo!()
  }
}

// —— 提取不变式断言 ——
const _: () = {
  assert!(WhereToEnd::EndAtNull as u8 == 0);
  assert!(WhereToEnd::EndAtNonHidden as u8 == 1);
};
`;

const p = writeSrc("v8-objects-maps", "prototype.rs", rust);
fmtNormalize(p);
console.log(`prototype.rs: WhereToEnd 2 值 + 字段 6 + 方法骨架 4/${defs.length} (pp 对数), 断言 ${checks} 项通过 → ${p}`);
