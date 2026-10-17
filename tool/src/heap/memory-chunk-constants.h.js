#!/usr/bin/env bun
// tool/src/heap/memory-chunk-constants.h.js — v8/src/heap/memory-chunk-constants.h → v8-heap-core/src/memory_chunk.rs (常量区基线)
// 规则:
//   ① clang -E 目标配置 (V8_COMPRESS_POINTERS/V8_ENABLE_SANDBOX/V8_EXTERNAL_CODE_SPACE) 展开后对数,
//      防 #ifdef V8_ENABLE_SANDBOX 分支漏计 — 目标配置 sandbox 开, 9 个常量全在分支内
//   ② 常量值用 v8-common 上游常量在 Rust 侧重算 (1<<18 页等), 脚本产 .skel 台账含逐项原版行号+实测值
import { spawnSync } from "node:child_process";
import { mkdirSync, readFileSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { dirname, join } from "node:path";
import { banner, readV8, root, writeSrc } from "../../lib.js";

const V8 = join(root, "v8");
const REL = "src/heap/memory-chunk-constants.h";
const src = readV8(REL);
const STUB = join(tmpdir(), "v8rs-heap-stub");
mkdirSync(STUB, { recursive: true });

// clang -E 目标配置展开; 缺失 include 以空桩头收敛 (预处理只关心 #if/#include, 不需要类型)
function preprocess(file) {
  const out = join(tmpdir(), "v8rs-heap-mcc.ii");
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

// ── ① 源码侧常量全集 (9 个, 定义序) ────────────────────────────────────────
const CONSTS = [
  "kPagesInMainCage", "kPagesInCodeCage", "kPagesInTrustedCage",
  "kMainCageMetadataOffset", "kTrustedSpaceMetadataOffset", "kCodeRangeMetadataOffset",
  "kMetadataPointerTableSizeLog2", "kMetadataPointerTableSize", "kMetadataPointerTableSizeMask",
];
const names = [...src.matchAll(/static constexpr size_t (k\w+) =/g)].map((m) => m[1]);
if (names.join(",") !== CONSTS.join(",")) fail("常量名序", names.join(","), CONSTS.join(","));
n++;
for (const c of CONSTS) {
  if (count(src, new RegExp(`static constexpr size_t ${c} =`, "g")) !== 1) fail(`常量 ${c} 计数`, count(src, new RegExp(`static constexpr size_t ${c} =`, "g")), 1);
  n++;
}
if (count(src, /^#ifdef V8_ENABLE_SANDBOX/gm) !== 1) fail("sandbox 守护块", count(src, /^#ifdef V8_ENABLE_SANDBOX/gm), 1);
n++;

// ── ② clang -E 展开侧对数: 目标配置 sandbox 开 → 9 常量全体保留 ───────────
// 展开产物剥注释且混入上游头 constexpr, 以类体为界截区 (class MemoryChunkConstants → 首个 };)
const expanded = preprocess(REL);
const clsStart = expanded.indexOf("class MemoryChunkConstants final : public AllStatic {");
if (clsStart < 0) fail("展开类体锚", clsStart, "found");
n++;
const clsEnd = expanded.indexOf("};", clsStart);
const clsRegion = clsStart >= 0 && clsEnd > clsStart ? expanded.slice(clsStart, clsEnd) : "";
if (count(clsRegion, /static constexpr size_t k/g) !== 9) fail("展开后类体常量数", count(clsRegion, /static constexpr size_t k/g), 9);
n++;
for (const c of CONSTS) {
  if (count(clsRegion, new RegExp(`static constexpr size_t ${c} =`, "g")) !== 1) fail(`展开后常量 ${c}`, count(clsRegion, new RegExp(`static constexpr size_t ${c} =`, "g")), 1);
  n++;
}

// ── ③ 值重算 (目标配置: 页 1<<18; 主 cage 4GB; code range/trusted 用 v8-common 常量式) ──
const K_PAGE = 1 << 18; // kRegularPageSize (v8-internal.h:120 kPageSizeBits=18)
const MAIN = 2 ** 32 / K_PAGE; // kPtrComprCageReservationSize = 4GB → 16384
if (MAIN !== 16384) fail("主 cage 页数实测", MAIN, 16384);
n++;

// ── ④ rs 侧落地锚 (HC-1R: v8-heap-core/src/memory_chunk.rs 常量区) ─────────
// 目的: 防止 Rust 侧常量名/值式漂移; 逐常量断言 pub const 定义 + 双行号 doc 精确计数。
const RS = "v8-heap-core/src/memory_chunk.rs";
const rs = readFileSync(join(root, RS), "utf8");
const rsLine = (re, label, want = 1) => {
  const got = (rs.match(new RegExp(re, "g")) || []).length;
  if (got !== want) fail(`rs ${label}`, got, want);
  n++;
};
const rsHas = (needle, label) => {
  if (!rs.includes(needle)) fail(`rs ${label}`, "missing", needle);
  n++;
};
// 9 常量逐名: Rust 侧 K_PascalCase → SCREAMING_SNAKE
const rsNames = CONSTS.map((c) => "K_" + c.slice(1).replace(/([a-z0-9])([A-Z])/g, "$1_$2").toUpperCase());
for (let i = 0; i < CONSTS.length; i++) rsLine(`pub const ${rsNames[i]}: usize =`, `常量定义 ${CONSTS[i]}`);
// 双行号 doc: 9 项逐项 constants.h:NN + 节头 constants.h:15-38 → 恰 10 处
rsLine("constants\\.h:[0-9][0-9]", "constants.h 双行号 doc 面", 10);
// 值式承接: 主 cage 用 v8-common 上游常量 (与展开侧 2^32/2^18 同源), 表容量走 bit_width64
rsHas("K_PTR_COMPR_CAGE_RESERVATION_SIZE / K_REGULAR_PAGE_SIZE", "主 cage 页数式");
rsHas("v8_common::K_MAXIMAL_TRUSTED_RANGE_SIZE / K_REGULAR_PAGE_SIZE", "trusted cage 页数式");
rsHas("bit_width64((K_PAGES_IN_MAIN_CAGE + K_PAGES_IN_CODE_CAGE + K_PAGES_IN_TRUSTED_CAGE) as u64)", "表容量位宽式");
rsHas("1 << K_METADATA_POINTER_TABLE_SIZE_LOG2", "表容量式");
rsHas("K_METADATA_POINTER_TABLE_SIZE - 1", "表掩码式");
// sandbox 分支: 目标配置恒开 → Rust 侧无条件落地 (无 cfg 门) 的一致性锚
if (/#\[[^\]]*cfg[^\]]*K_PAGES_IN_MAIN_CAGE/.test(rs)) fail("rs 常量误加 cfg 门", "found", "none");
n++;

const RS_ARM = `
// ══ rs 落地锚 (v8-heap-core/src/memory_chunk.rs, HC-1R) ══
// 9 常量逐名 (SCREAMING_SNAKE, 均 usize, 无 cfg 门 = 目标配置 sandbox 恒开):
${rsNames.map((r, i) => `// [${i}] ${CONSTS[i].padEnd(31)} → pub const ${r}`).join("\n")}
// 双行号 doc 面: constants.h:NN 共 10 处 (9 项逐项 + 节头 15-38)
// 值式同源: 主 cage = K_PTR_COMPR_CAGE_RESERVATION_SIZE / K_REGULAR_PAGE_SIZE;
//   trusted = v8_common::K_MAXIMAL_TRUSTED_RANGE_SIZE / K_REGULAR_PAGE_SIZE;
//   表位宽 = bit_width64(主+码+信); 表容量 = 1<<位宽; 掩码 = 容量-1
// 码 cage 页数随架构 (x64 2048 / arm64 1024) — Rust 侧 const 式与原版 globals 同源, 不写死。
`;

// ── ④c rs 侧实测锚代入 (HC-1R2a 增量: skel 金标准注释须与 rs 台账同源) ──
if (count(rs, /2\^32 \/ 2\^18 = 16384/g) !== 1) fail("rs 主 cage 实测锚", 0, 1);
n++;
if (count(rs, /2\^30 \/ 2\^18 = 4096/g) !== 1) fail("rs trusted 实测锚", 0, 1);
n++;

const skel = `${banner("src/heap/memory-chunk-constants.h.js", REL, "MemoryChunkConstants 9 常量台账 (目标配置 sandbox 开, 全量落地)")
}
// ══ MemoryChunkConstants (原版 memory-chunk-constants.h:15-38) → v8-heap-core/src/memory_chunk.rs ══
// 逐项: 名 + 原版行号 + 展开实测式 (Rust 侧以 v8-common K_REGULAR_PAGE_SIZE 等重算)
${CONSTS.map((c, i) => `// [${i}] ${c.padEnd(31)} h:${src.split(String.fromCharCode(10)).findIndex((l) => l.includes(`size_t ${c} =`)) + 1}`).join("\n")}
// 实测锚: kPagesInMainCage = 2^32 / 2^18 = 16384; kTrustedSpaceMetadataOffset = 16384;
// kPagesInTrustedCage = 2^30 / 2^18 = 4096; kPagesInCodeCage = K_MAXIMAL_CODE_RANGE_SIZE / 2^18 (x64 与 arm64 值异,
//   Rust 侧按 v8-common K_MAXIMAL_CODE_RANGE_SIZE const 式随架构取, 与原版 globals 同源);
// kMetadataPointerTableSizeLog2 = BitWidth(主+码+信) ; kMetadataPointerTableSize = 1<<log2 ; Mask = size-1
// ── rs 侧落地锚 (HC-1R2a 实测台账) ──
// Landed @ v8-heap-core/src/memory_chunk.rs (MemoryChunkConstants 区): 9 常量全量 pub const 落地
//   (constants.h:17-37 沙箱守护目标配置开 → 无剥除臂); 测试面 chunk_constants_gold 金标准 16384/4096/32768。
`;
writeSrc("v8-heap-core", "memory_chunk_constants_h.skel", skel + RS_ARM);
console.log(`OK memory-chunk-constants.h.js: 对数断言 ${n} 条全过 (原版侧 23 + rs 落地锚 16 + rs 实测锚代入 2); 常量全集 9 项逐名序 + clang -E 展开侧 9 项 + 类锚 1 + sandbox 守护 1 + 主 cage 页数实测 16384; rs 侧 9 常量定义 + 双行号 doc 10 + 值式同源 5 + 无 cfg 门 1 + 实测锚 2 (16384/4096); 产物 v8-heap-core/src/memory_chunk_constants_h.skel`);
