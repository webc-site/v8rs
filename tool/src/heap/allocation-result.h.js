#!/usr/bin/env bun
// tool/src/heap/allocation-result.h.js — v8/src/heap/allocation-result.h → v8-heap-core/src/allocation.rs (AllocationOrigin + AllocationResult 区)
// 规则:
//   ① AllocationOrigin 枚举逐值锚定 (6 项, 名+值逐条) — 枚举值即 ABI, 逐值断言
//   ② AllocationResult 方法全集 10 + 字段 1 + sizeof 静态断言 1
//   ③ clang -E 目标配置展开侧: 枚举 6 项逐名在案, 类体方法/字段/断言计数核对
import { spawnSync } from "node:child_process";
import { mkdirSync, readFileSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { dirname, join } from "node:path";
import { banner, readV8, root, writeSrc } from "../../lib.js";

const V8 = join(root, "v8");
const REL = "src/heap/allocation-result.h";
const src = readV8(REL);
const STUB = join(tmpdir(), "v8rs-heap-stub");
mkdirSync(STUB, { recursive: true });
function preprocess(file) {
  const out = join(tmpdir(), "v8rs-heap-ar.ii");
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

// ── ① AllocationOrigin 逐值 (原版 h:15-22) ─────────────────────────────────
const ORIGIN = [
  ["kGeneratedCode", "0", 16],
  ["kRuntime", "1", 17],
  ["kGC", "2", 18],
  ["kFirstAllocationOrigin", "kGeneratedCode", 19],
  ["kLastAllocationOrigin", "kGC", 20],
  ["kNumberOfAllocationOrigins", "kLastAllocationOrigin + 1", 21],
];
const enumBody = src.slice(src.indexOf("enum class AllocationOrigin"), src.indexOf("};", src.indexOf("enum class AllocationOrigin")));
const gotEntries = [...enumBody.matchAll(/(\w+) = ([\w+ ]+)/g)].map((m) => [m[1], m[2].trim()]);
if (gotEntries.length !== 6) fail("枚举项数", gotEntries.length, 6);
n++;
ORIGIN.forEach(([name, val], i) => {
  const [gn, gv] = gotEntries[i] ?? ["-", "-"];
  if (gn !== name || gv !== val) fail(`枚举[${i}]`, `${gn}=${gv}`, `${name}=${val}`);
  n++;
});

// ── ② AllocationResult 方法全集 ────────────────────────────────────────────
const FNS = [
  ["Failure", /static AllocationResult Failure\(\) \{ return AllocationResult\(\); \}/, 1],
  ["FromObject", /static AllocationResult FromObject\(Tagged<HeapObject> heap_object\)/, 1],
  ["默认 ctor", /AllocationResult\(\) = default;/, 1],
  ["IsFailure", /bool IsFailure\(\) const \{ return object_\.is_null\(\); \}/, 1],
  ["To<T>", /bool To\(Tagged<T>\* obj\) const/, 1],
  ["ToObjectChecked", /Tagged<HeapObject> ToObjectChecked\(\) const/, 1],
  ["ToObject", /Tagged<HeapObject> ToObject\(\) const/, 1],
  ["ToAddress", /Address ToAddress\(\) const/, 1],
  ["operator bool", /explicit operator bool\(\) const \{ return !IsFailure\(\); \}/, 1],
  ["私有 ctor", /explicit AllocationResult\(Tagged<HeapObject> heap_object\)/, 1],
];
for (const [label, re, want] of FNS) {
  if (count(src, re) !== want) fail(`锚 ${label}`, count(src, re), want);
  n++;
}
if (count(src, /Tagged<HeapObject> object_;/) !== 1) fail("字段 object_", 0, 1);
n++;
if (count(src, /static_assert\(sizeof\(AllocationResult\) == kSystemPointerSize\);/) !== 1) fail("sizeof 断言", 0, 1);
n++;

// ── ③ 展开侧核对 ───────────────────────────────────────────────────────────
const expanded = preprocess(REL);
const clsStart = expanded.indexOf("class AllocationResult final {");
if (clsStart < 0) fail("展开类体锚", clsStart, "found");
n++;
const region = expanded.slice(clsStart, expanded.indexOf("};", clsStart));
const eStart = expanded.indexOf("enum class AllocationOrigin");
if (eStart < 0) fail("展开枚举锚", eStart, "found");
n++;
const eRegion = expanded.slice(eStart, expanded.indexOf("};", eStart));
if (count(eRegion, /k\w+ = /g) !== 6) fail("展开枚举项数", count(eRegion, /k\w+ = /g), 6);
n++;
for (const [name] of ORIGIN) {
  if (!eRegion.includes(name)) fail(`展开枚举缺名 ${name}`, true, false);
  n++;
}
if (count(region, /Tagged<HeapObject> object_;/) !== 1) fail("展开字段 object_", 0, 1);
n++;
if (count(expanded, /static_assert\(sizeof\(AllocationResult\) == kSystemPointerSize\);/) !== 1) fail("展开 sizeof 断言", count(expanded, /static_assert\(sizeof\(AllocationResult\) == kSystemPointerSize\);/), 1);
n++;

// ── ④ rs 侧落地锚 (HC-1R: v8-heap-core/src/allocation.rs 实态对数, 只增不减) ──
// 区界: 只截 AllocationStats 之前的 AllocationOrigin + AllocationResult 区, 防两文件行号串台。
const rs = readFileSync(join(root, "v8-heap-core", "src", "allocation.rs"), "utf8");
const rsRegion = rs.slice(0, rs.indexOf("AllocationStats (原版 allocation-stats.h"));
if (rsRegion.length < 100) fail("rs 区界锚 (AllocationStats 分隔)", rsRegion.length, ">100");
n++;
for (const [label, re, want] of [
  ["枚举声明", /pub enum AllocationOrigin \{/, 1],
  ["ABI 承接 repr(i32) (属性 1 + doc 1)", /#\[repr\(i32\)\]/, 2],
  ["GeneratedCode 值 0 (h:16, 承接 kGeneratedCode)", /GeneratedCode = 0,/, 1],
  ["Runtime 值 1 (h:17, 承接 kRuntime)", /Runtime = 1,/, 1],
  ["Gc 值 2 (h:18, 承接 kGC)", /Gc = 2,/, 1],
  ["别名 K_FIRST_ALLOCATION_ORIGIN (h:19, 承接 kFirstAllocationOrigin)", /pub const K_FIRST_ALLOCATION_ORIGIN: Self = Self::GeneratedCode;/, 1],
  ["别名 K_LAST_ALLOCATION_ORIGIN (h:20, 承接 kLastAllocationOrigin)", /pub const K_LAST_ALLOCATION_ORIGIN: Self = Self::Gc;/, 1],
  ["派生 K_NUMBER_OF_ALLOCATION_ORIGINS (h:21, 承接 kNumberOfAllocationOrigins)", /pub const K_NUMBER_OF_ALLOCATION_ORIGINS: usize = Self::K_LAST_ALLOCATION_ORIGIN as usize \+ 1;/, 1],
  ["类声明", /pub struct AllocationResult \{/, 1],
  ["字段 object_ (h:68)", /object_: Tagged<HeapObject>,/, 1],
  ["Failure (h:28)", /pub fn failure\(\) -> Self/, 1],
  ["FromObject (h:30)", /pub fn from_object\(heap_object: Tagged<HeapObject>\)/, 1],
  ["默认 ctor → 私有 null (h:36)", /fn null\(\) -> Self/, 1],
  ["Default impl 承接空 ctor", /impl Default for AllocationResult/, 1],
  ["IsFailure (h:38)", /pub fn is_failure\(&self\) -> bool/, 1],
  ["is_null 式 (h:38 实体)", /self\.object_\.is_null\(\)/, 1],
  ["To<T> (h:40)", /pub fn to<T>\(&self, obj: &mut Tagged<T>\) -> bool/, 1],
  ["Cast<T> → Tagged::cast (h:43)", /self\.object_\.cast::<T>\(\)/, 1],
  ["ToObjectChecked (h:47)", /pub fn to_object_checked\(&self\)/, 1],
  ["CHECK(!IsFailure()) → check_with_msg (h:48)", /check_with_msg\(!self\.is_failure\(\), "!IsFailure\(\)"\)/, 1],
  ["ToObject (h:52)", /pub fn to_object\(&self\)/, 1],
  ["ToAddress (h:57)", /pub fn to_address\(&self\) -> Address/, 1],
  ["operator bool → ok (h:62)", /pub fn ok\(&self\) -> bool/, 1],
  ["私有 ctor (h:65)", /object_: Tagged::NULL/, 1],
  ["sizeof 静态断言 (h:71)", /assert!\(size_of::<AllocationResult>\(\) == K_SYSTEM_POINTER_SIZE\)/, 1],
  ["DCHECK 面 (h:53/h:58) 用 debug_assert", /debug_assert!\(/, 2],
]) {
  const reG = new RegExp(re.source, "g");
  if (count(rsRegion, reG) !== want) fail(`rs 锚 ${label}`, count(rsRegion, reG), want);
  n++;
}
// 逐行 doc 锚: h:16..h:21 各 1 (枚举 6 项逐项双行号)
for (const [, , ln] of ORIGIN) {
  if (count(rsRegion, new RegExp(`h:${ln}\\b`, "g")) !== 1) fail(`rs 双行号 h:${ln}`, count(rsRegion, new RegExp(`h:${ln}\\b`, "g")), 1);
  n++;
}
// 禁面: 本仓源无 origin/value/success 成员 (票面差异申报) → Rust 侧不得凭空长出
for (const [label, re] of [["origin 字段 (源无)", /origin_?:/], ["value 字段 (源无)", /value_?:/], ["success 字段 (源无)", /success_?:/]]) {
  if (count(rsRegion, re) !== 0) fail(`rs 禁面 ${label}`, count(rsRegion, re), 0);
  n++;
}

const RS_ARM = `
// ── rs 侧落地锚 (HC-1R 实测台账) ──
// Landed @ v8-heap-core/src/allocation.rs (AllocationOrigin + AllocationResult 区, 26 锚 + 双行号 6 + 禁面 3):
//   AllocationOrigin: #[repr(i32)] 3 真值 (h:16/17/18) + 3 关联常量别名 (h:19/20/21, Rust 枚举禁重复判别式);
//   AllocationResult: 字段 object_ (h:68) + Failure/FromObject/null ctor/Default/is_failure/to<T>/
//     to_object_checked (check_with_msg = 原版 CHECK abort 面)/to_object/to_address (debug_assert = DCHECK 面)/ok;
//   sizeof==kSystemPointerSize 以 const _: () 恒检 (h:71);
//   禁面 0 检: 本仓 76 行旧版无 origin/value/success 成员 (票面差异申报, 见模块头注释)。
`;

const skel = `${banner("src/heap/allocation-result.h.js", REL, "AllocationOrigin 6 项逐值 + AllocationResult 10 方法台账 (全落地, 无 Isolate 耦合)")
}
// ══ AllocationOrigin (原版 h:15-22) → v8-heap-core/src/allocation.rs ══
${ORIGIN.map(([name, val, ln], i) => `// [${i}] ${name.padEnd(26)} = ${val.padEnd(25)} h:${ln}`).join("\n")}
// ══ AllocationResult (原版 h:26-69) → v8-heap-core/src/allocation.rs ══
// Failure h:28 / FromObject h:30 / 默认 ctor(=失败) h:36 / IsFailure h:38 / To<T> h:40-45 /
// ToObjectChecked h:47 / ToObject h:52 / ToAddress h:57 / operator bool h:62 / 私有 ctor h:65
// 字段: Tagged<HeapObject> object_ h:68; 布局断言 sizeof==kSystemPointerSize h:71
// 处置: 全部落地; To<T> 的 Cast<T> 以 v8-objects-core Tagged::cast + Cast trait 承接
${RS_ARM}`;
writeSrc("v8-heap-core", "allocation_result_h.skel", skel);
console.log(`OK allocation-result.h.js: 对数断言 ${n} 条全过 (原版/展开侧 30 + rs 落地锚 36); AllocationOrigin 6 项逐名逐值逐行 + 方法锚 10 + 字段/断言 2 + 展开侧 (枚举 6 项逐名 / 类体锚 / 字段 / sizeof==8 实测) 全对; rs 侧枚举/方法/字段 26 + 双行号 6 + 禁面 0 检 3; 产物 v8-heap-core/src/allocation_result_h.skel`);
