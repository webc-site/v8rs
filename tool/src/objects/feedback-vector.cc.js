#!/usr/bin/env bun
// tool/src/objects/feedback-vector.cc.js — 覆盖 v8/src/objects/feedback-vector.cc
//   (.h/-inl.h/.tq 与 feedback-cell 族归既有席 feedback-vector.h.js, 本脚本只做差集)
//   → v8-objects-js-func/src/feedback_vector_cc.rs
// 规则代码: FeedbackSlotKind 显示名表 (Kind2String, oracle: clang -E) + cc 方法清单。
// 注意 kSetKeyedSloppy/Strict 显示名是 StoreKeyedSloppy/Strict (原版刻意为之)。

import { spawnSync } from "node:child_process";
import { existsSync, mkdirSync, readFileSync, writeFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { root, writeSrc, banner } from "../../lib.js";

// ── clang -E oracle ────────────────────────────────────────────────────────
const STUB = join(root, "target", "agent_R3O6", "clang-stub");
const CACHE = join(root, "target", "agent_R3O6", "clang-E");
function clangE(rel) {
  const cache = join(CACHE, rel.replaceAll("/", "_") + ".i");
  if (existsSync(cache)) return readFileSync(cache, "utf8");
  mkdirSync(STUB, { recursive: true });
  mkdirSync(CACHE, { recursive: true });
  for (let i = 0; i < 96; i++) {
    const r = spawnSync(
      "clang",
      ["-E", "-x", "c++", "-std=c++20", "-I", "v8", "-I", "v8/include", "-I", STUB, "-P", join("v8", rel)],
      { cwd: root, encoding: "utf8", maxBuffer: 1 << 28 },
    );
    const miss = r.stderr?.match(/fatal error: '([^']+)' file not found/);
    if (!miss) {
      if (r.status !== 0) throw new Error(`clang -E ${rel} 失败: ${r.stderr?.slice(0, 400)}`);
      writeFileSync(cache, r.stdout);
      return r.stdout;
    }
    const p = join(STUB, miss[1]);
    mkdirSync(dirname(p), { recursive: true });
    writeFileSync(p, "");
  }
  throw new Error(`clang -E ${rel}: stub 迭代超限`);
}

let checks = 0;
const must = (c, m) => {
  checks++;
  if (!c) throw new Error(`锚点失配: ${m}`);
};

const iSrc = clangE("src/objects/feedback-vector.cc");

// ── 1. Kind2String 显示名表 (oracle: -E 尾部区段 switch 明文) ──────────────
const tail = iSrc.slice(iSrc.lastIndexOf("\nnamespace v8 {"));
const k2sAt = tail.indexOf("const char* FeedbackMetadata::Kind2String(FeedbackSlotKind kind) {");
must(k2sAt >= 0, "-E 尾部缺 Kind2String");
const k2sEnd = tail.indexOf("UNREACHABLE", k2sAt);
const k2sBody = tail.slice(k2sAt, k2sEnd);
const names = [...k2sBody.matchAll(/case FeedbackSlotKind::(\w+):\s*return "(\w+)";/g)].map((m) => [m[1], m[2]]);
must(names.length === 26, `Kind2String case ${names.length} != 26`);
must(new Set(names.map(([k]) => k)).size === 26, "Kind2String case 名重复");
must(names.find(([k]) => k === "kInvalid")?.[1] === "Invalid", "kInvalid 显示名失配");
// 原版刻意别名: kSetKeyedSloppy/Strict 显示 Store*
must(names.find(([k]) => k === "kSetKeyedSloppy")?.[1] === "StoreKeyedSloppy", "kSetKeyedSloppy 应显示 StoreKeyedSloppy");
must(names.find(([k]) => k === "kSetKeyedStrict")?.[1] === "StoreKeyedStrict", "kSetKeyedStrict 应显示 StoreKeyedStrict");
must(names.find(([k]) => k === "kJumpLoop")?.[1] === "JumpLoop", "kJumpLoop 显示名失配");
must(!names.some(([k]) => k === "kKindsCount"), "KindsCount 不应出现在 switch");

// ── 2. cc 方法清单 (oracle: -E 尾部区段) ──────────────────────────────────
const defs = [...new Set([...tail.matchAll(/^[A-Za-z_][\w:<>,&* ]*?\b(\w+)::(~?\w+)\(/gm)].map((m) => `${m[1]}::${m[2]}`))];
must(defs.length >= 60, `cc 方法定义 ${defs.length} < 60`);
const byCls = {};
for (const d of defs) {
  const c = d.split("::")[0];
  byCls[c] = (byCls[c] ?? 0) + 1;
}
for (const [cls, min] of [["FeedbackNexus", 35], ["FeedbackVector", 10], ["FeedbackMetadata", 5], ["FeedbackVectorSpec", 1], ["ClosureFeedbackCellArray", 1]]) {
  must((byCls[cls] ?? 0) >= min, `${cls} 方法 ${(byCls[cls] ?? 0)} < ${min}`);
}
for (const key of ["FeedbackVectorSpec::AddSlot", "FeedbackMetadata::GetKind", "FeedbackNexus::ConfigureUninitialized", "FeedbackNexus::ic_state", "FeedbackVector::ClearSlots", "FeedbackNexus::Clear", "FeedbackNexus::ic_state"]) {
  must(tail.includes(key), `缺 ${key}`);
}

// ── 3. 产物 ────────────────────────────────────────────────────────────────
const body = `${banner("src/objects/feedback-vector.cc.js", "src/objects/feedback-vector.cc", "FeedbackSlotKind 显示名表 + cc 方法清单骨架 (h/inl/tq 差集归 feedback-vector.h.js 席)")}
//! 差集骨架: feedback-vector.cc 的规则代码。
//! IC 状态机 / megamorphic 处理 / OSR 语义面留 Developer。

/// v8: src/objects/feedback-vector.cc FeedbackMetadata::Kind2String 显示名表
///   (FeedbackSlotKind::kXxx → 字符串); kSetKeyedSloppy/Strict 显示 Store* 为原版刻意别名
pub const FEEDBACK_SLOT_KIND_NAMES: &[(&str, &str); ${names.length}] = &[
${names.map(([k, v]) => `  ("${k}", "${v}"),`).join("\n")}
];

// ── cc 方法清单 (${defs.length} 个定义, oracle: clang -E 尾部区段) ──
/// v8: src/objects/feedback-vector.cc
pub const FEEDBACK_VECTOR_CC_METHODS: &[&str] = &[
${defs.map((d) => `  "${d}",`).join("\n")}
];

/// v8: src/objects/feedback-vector.cc FeedbackNexus — 槽位反馈读写的中心抽象
///   (ic_state / Configure* / ComputeUnreliableReceiverMap 语义面留锚)
/// v8: src/objects/feedback-vector.cc FeedbackVector::ClearSlots — 逐槽重置 (含 weak 引用清弱)
/// v8: src/objects/feedback-vector.cc FeedbackVectorSpec::AddSlot — GetSlotSize(kind) 决定槽宽补 kInvalid

// ── 对数不变式 ──
const fn str_eq(a: &str, b: &str) -> bool {
  let (a, b) = (a.as_bytes(), b.as_bytes());
  if a.len() != b.len() {
    return false;
  }
  let mut i = 0;
  while i < a.len() {
    if a[i] != b[i] {
      return false;
    }
    i += 1;
  }
  true
}

const fn contains_pair(t: &[(&str, &str)], k: &str, n: &str) -> bool {
  let mut i = 0;
  while i < t.len() {
    if str_eq(t[i].0, k) && str_eq(t[i].1, n) {
      return true;
    }
    i += 1;
  }
  false
}

const _: () = {
  assert!(FEEDBACK_SLOT_KIND_NAMES.len() == 26);
  let n0 = FEEDBACK_SLOT_KIND_NAMES[0];
  assert!(str_eq(n0.0, "kInvalid") && str_eq(n0.1, "Invalid"));
  let n1 = FEEDBACK_SLOT_KIND_NAMES[1];
  assert!(str_eq(n1.0, "kCall") && str_eq(n1.1, "Call"));
  assert!(contains_pair(FEEDBACK_SLOT_KIND_NAMES, "kSetKeyedSloppy", "StoreKeyedSloppy"));
  assert!(contains_pair(FEEDBACK_SLOT_KIND_NAMES, "kDefineKeyedOwnPropertyInLiteral", "DefineKeyedOwnPropertyInLiteral"));
  assert!(FEEDBACK_VECTOR_CC_METHODS.len() >= 60);
};
`;

const p = writeSrc("v8-objects-js-func", "feedback_vector_cc.rs", body);
console.log(
  `${p}: Kind2String ${names.length}/26 (含 Store* 别名 2), cc 方法 ${defs.length} ` +
    `(Nexus ${byCls.FeedbackNexus}/Vector ${byCls.FeedbackVector}/Metadata ${byCls.FeedbackMetadata}), 断言 ${checks} 项通过`,
);
