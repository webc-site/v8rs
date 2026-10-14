#!/usr/bin/env bun
// tool/src/heap/linear-allocation-area.h.js — v8/src/heap/linear-allocation-area.h → v8-heap-core/src/linear_allocation_area.rs
// 规则:
//   ① 方法/字段/静态断言全集逐锚计数 (源码侧正则) — 函数全集 21 个 (含 ctor ×2 / getter ×3 /
//      地址取址 ×4 / 私有 offset ×4 / Size / Verify / Reset 族 / 算术族)
//   ② clang -E 目标配置展开侧: DEBUG 未定义 → SLOW_DCHECK 0 处; 类体/字段/标准布局断言逐项在
//   ③ 布局金标准: sizeof = 3×kSystemPointerSize, offsetof = 0/8/16 (Rust tests 逐项断言)
import { spawnSync } from "node:child_process";
import { mkdirSync, readFileSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { dirname, join } from "node:path";
import { banner, readV8, root, writeSrc } from "../../lib.js";

const V8 = join(root, "v8");
const REL = "src/heap/linear-allocation-area.h";
const src = readV8(REL);
const STUB = join(tmpdir(), "v8rs-heap-stub");
mkdirSync(STUB, { recursive: true });
function preprocess(file) {
  const out = join(tmpdir(), "v8rs-heap-laa.ii");
  for (let i = 0; i < 200; i++) {
    const r = spawnSync("clang", ["-E", "-P", "-x", "c++", "-std=c++20",
      "-DV8_COMPRESS_POINTERS", "-DV8_ENABLE_SANDBOX", "-DV8_EXTERNAL_CODE_SPACE",
      "-I", ".", "-I", "include", "-I", STUB, file, "-o", out], { cwd: V8, encoding: "utf8" });
    if (r.status === 0) return readFileSync(out, "utf8");
    const miss = (r.stderr.match(/fatal error: '([^']+)' file not found/) || [])[1];
    if (!miss) throw new Error(`clang -E ${file} 硬失败:\n${r.stderr}`);
    const p = join(STUB, miss);
    mkdirSync(dirname(p), { recursive: true });
    writeFileSync(p, "#pragma once\n");
  }
  throw new Error(`clang -E ${file} 桩头迭代超限`);
}

let n = 0;
const fail = (label, got, want) => {
  throw new Error(`对数失配 ${label}: got=${got} want=${want}`);
};
const count = (s, re) => (s.match(re) || []).length;

// ── ① 函数全集 (21, 逐签名锚) ──────────────────────────────────────────────
const FNS = [
  ["默认 ctor", /LinearAllocationArea\(\) = default;/, 1],
  ["双参 ctor", /LinearAllocationArea\(Address top, Address limit\)/, 1],
  ["Reset", /void Reset\(Address top, Address limit\) \{/, 1],
  ["ResetStart", /void ResetStart\(\) \{ start_ = top_; \}/, 1],
  ["CanIncrementTop", /bool CanIncrementTop\(size_t bytes\) const/, 1],
  ["IncrementTop", /Address IncrementTop\(size_t bytes\)/, 1],
  ["DecrementTopIfAdjacent", /bool DecrementTopIfAdjacent\(Address new_top, size_t bytes\)/, 1],
  ["SetLimit", /void SetLimit\(Address limit\) \{/, 1],
  ["start()", /Address start\(\) const/, 1],
  ["top()", /Address top\(\) const/, 1],
  ["limit()", /Address limit\(\) const/, 1],
  ["top_address const", /const Address\* top_address\(\) const/, 1],
  ["top_address mut", /Address\* top_address\(\) \{/, 1],
  ["limit_address const", /const Address\* limit_address\(\) const/, 1],
  ["limit_address mut", /Address\* limit_address\(\) \{/, 1],
  ["Verify", /void Verify\(\) const \{/, 1],
  ["StartOffset", /static constexpr int StartOffset\(\)/, 1],
  ["TopOffset", /static constexpr int TopOffset\(\)/, 1],
  ["LimitOffset", /static constexpr int LimitOffset\(\)/, 1],
  ["Size", /static constexpr int Size\(\)/, 1],
  ["friend IsolateData", /friend class IsolateData;/, 1],
];
for (const [label, re, want] of FNS) {
  if (count(src, re) !== want) fail(`锚 ${label}`, count(src, re), want);
  n++;
}
// 字段 3 + 初始值 kNullAddress 3 + static_assert 1 (sizeof) + standard_layout 1
for (const [label, re, want] of [
  ["字段 start_", /Address start_ = kNullAddress;/, 1],
  ["字段 top_", /Address top_ = kNullAddress;/, 1],
  ["字段 limit_", /Address limit_ = kNullAddress;/, 1],
  ["sizeof 断言", /static_assert\(sizeof\(LinearAllocationArea\) == 3 \* kSystemPointerSize/, 1],
  ["standard_layout 断言", /static_assert\(std::is_standard_layout_v<LinearAllocationArea>/, 1],
]) {
  if (count(src, re) !== want) fail(label, count(src, re), want);
  n++;
}

// ── ② clang -E 展开侧 (目标配置: DEBUG 关 → Verify 体剥空) ────────────────
const expanded = preprocess(REL);
const clsStart = expanded.indexOf("class LinearAllocationArea final {");
if (clsStart < 0) fail("展开类体锚", clsStart, "found");
n++;
const clsEnd = expanded.indexOf("};", clsStart);
const region = expanded.slice(clsStart, clsEnd);
if (count(region, /SLOW_DCHECK/g) !== 0) fail("展开后 SLOW_DCHECK 数 (DEBUG 关应为 0)", count(region, /SLOW_DCHECK/g), 0);
n++;
if (count(region, /Verify\(\);/g) !== 10) fail("展开后 Verify() 调用点 (源码实测 10)", count(region, /Verify\(\);/g), 10);
n++;
for (const m of ["start_", "top_", "limit_"]) {
  if (count(region, new RegExp(`Address ${m} = kNullAddress;`, "g")) !== 1) fail(`展开字段 ${m}`, 0, 1);
  n++;
}
if (count(expanded, /is_standard_layout_v<LinearAllocationArea>/g) !== 1) fail("展开 standard_layout 断言", count(expanded, /is_standard_layout_v<LinearAllocationArea>/g), 1);
n++;

// 原版实测行号 (与 ① 锚表同序, 勘定自 linear-allocation-area.h)
const FN_LINES = [21, 22, 27, 34, 36, 41, 48, 61, 66, 70, 74, 78, 79, 80, 81, 83, 99, 102, 105, 108];
const skel = `${banner("src/heap/linear-allocation-area.h.js", REL, "LinearAllocationArea 21 方法 + 3 字段台账; 目标配置 DEBUG 关, Verify 为 cfg 门")
}
// ══ LinearAllocationArea (原版 linear-allocation-area.h:19-124) → v8-heap-core/src/linear_allocation_area.rs ══
// 全部落地 (纯结构, 无 Isolate 耦合); Verify 断言按原版 DEBUG 门 → Rust cfg!(debug_assertions)
${FNS.slice(0, 20).map(([label], i) => `// ${label.padEnd(24)} h:${FN_LINES[i]}`).join("\n")}
// 布局金标准 (tests/memory_chunk.rs 同组断言): sizeof = 3*kSystemPointerSize = 24;
//   StartOffset=0 / TopOffset=8 / LimitOffset=16 (8 平台, repr(C) 三 Address 字段)
// 不变量: start_ <= top_ <= limit_; top 对齐 kObjectAlignment (8GB 堆则 kObjectAlignment8GbHeap)
// 私有 offset 访问器原版 friend IsolateData 专用 → Rust 侧 pub const fn 供布局断言与上层 IsolateData 票消费
`;
writeSrc("v8-heap-core", "linear_allocation_area_h.skel", skel);
console.log(`OK linear-allocation-area.h.js: 对数断言 ${n} 条全过; 方法/结构锚 21 + 字段与断言锚 5 + 展开侧 (SLOW_DCHECK 剥空 0 / Verify 调用点 10 / 字段 ×3 / standard_layout 1); 产物 v8-heap-core/src/linear_allocation_area_h.skel`);
