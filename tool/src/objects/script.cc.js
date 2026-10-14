#!/usr/bin/env bun
// tool/src/objects/script.cc.js — 覆盖 v8/src/objects/script.{-inl.h,cc,tq}
//   (script.h 枚举/PositionInfo 归既有席 script.h.js → v8-objects-rest/src/script.rs, 本脚本只做差集)
//   → v8-objects-js-func/src/script_cc.rs
// 规则代码: Script tq 字段布局 (64 位非压缩 offset 表) + ScriptFlags 位段 (与 .h BitField 链 clang -E 对数)
//   + ScriptOriginOptions 位标志 + inl/cc 方法清单。
// 目标配置: V8_SCRIPTORMODULE_LEGACY_LIFETIME 关 (script_or_modules 字段不落地)。

import { spawnSync } from "node:child_process";
import { existsSync, mkdirSync, readFileSync, writeFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { root, readV8, writeSrc, banner } from "../../lib.js";

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

const tq = readV8("src/objects/script.tq");
const ccI = clangE("src/objects/script.cc");
const inlI = clangE("src/objects/script-inl.h");

// ── 1. oracle: script.cc.i 中 Script 类成员 (clang -E 明文) ────────────────
const clsAt = ccI.indexOf("class Script : public Struct");
must(clsAt >= 0, "-E 缺 class Script");
const clsEnd = ccI.indexOf("\n};", clsAt);
const clsBody = ccI.slice(clsAt, clsEnd);  // V8_TQ_TYPE 在 -E 后展开为空, 尾随 'flags_ ;'
const hFields = [...clsBody.matchAll(/TaggedMember<[^;]+?>\s+(\w+)_?\s*(?:V8_TQ_TYPE\([^)]*\))?\s*;/g)]
  .map((m) => m[1].replace(/_$/, ""));
must(!hFields.includes("script_or_modules"), "legacy lifetime 字段不应在目标配置展开");
must(hFields.length === 19, `.h Script 字段 ${hFields.length} != 19:\n${hFields}`);
must(hFields.join(",") === "source,name,line_offset,column_offset,context_data,script_type,line_ends,id,eval_from_shared_or_wrapped_arguments,eval_from_position,eval_from_scope_info,infos,compiled_lazy_function_positions,flags,source_url,source_mapping_url,debug_id,host_defined_options,source_hash", "Script 字段清单失配");

// ── 2. tq 字段 (滤 legacy 分支) ↔ oracle 对数 ─────────────────────────────
const tqBody = tq.slice(tq.search(/^extern class Script extends Struct \{/m));
const tqSeg = tqBody.slice(0, tqBody.indexOf("\n}"));
const tqFields = [];
for (const line of tqSeg.split("\n")) {
  // 目标配置关 legacy lifetime: 同行 @if 字段整行丢弃
  if (line.includes("@if(V8_SCRIPTORMODULE_LEGACY_LIFETIME)")) continue;
  if (/^\s*@(if|cpp)/.test(line)) continue;
  const m = line.match(/^  (\w+):/);
  if (m) tqFields.push(m[1]);
}
must(tqFields.length === 19, `tq Script 字段 ${tqFields.length} != 19`);
must(JSON.stringify(tqFields) === JSON.stringify(hFields), "tq 字段序与 .h 成员序不一致");

// ScriptFlags 位段 (tq) ↔ .h BitField 链 (oracle) 对数
const flagSeg = tq.slice(tq.search(/bitfield struct ScriptFlags/));
const tqFlags = [...flagSeg.slice(0, flagSeg.indexOf("\n}")).matchAll(/^  (\w+): [\w<>|]+: (\d+) bit;/gm)].map((m) => ({ name: m[1], width: Number(m[2]) }));
must(tqFlags.length === 8, `tq ScriptFlags 段 ${tqFlags.length} != 8`);
const chainAt = ccI.indexOf("using CompilationKindBits =");
must(chainAt >= 0, "-E 缺 ScriptFlags BitField 链");
const chainSrc = ccI.slice(chainAt, ccI.indexOf("OuterLanguageModeBit", chainAt) + 200);
const hFlags = [{ name: "CompilationKindBits", shift: 0, width: 3 }];
{
  const nexts = [...chainSrc.matchAll(/using (\w+)\s*=\s*(\w+)::Next<[\w:]+,\s*(\d+)>;/g)];
  for (const m of nexts) {
    const prev = hFlags.at(-1);
    if (m[2] === prev.name) hFlags.push({ name: m[1], shift: prev.shift + prev.width, width: Number(m[3]) });
  }
}
must(hFlags.length === 8 && hFlags.at(-1).name === "OuterLanguageModeBit", `链 ${hFlags.length} 段异常`);
must(hFlags.at(-1).shift === 12 && hFlags.reduce((a, s) => a + s.width, 0) === 13, "ScriptFlags 总宽应 13 (uint31)");
const EXPECT_FLAGS = [
  ["compilation_kind", "CompilationKindBits"], ["compilation_state", "CompilationStateBit"],
  ["is_repl_mode", "IsReplModeBit"], ["origin_options", "OriginOptionsBits"],
  ["break_on_entry", "BreakOnEntryBit"], ["produce_compile_hints", "ProduceCompileHintsBit"],
  ["deserialized", "DeserializedBit"], ["outer_language_mode", "OuterLanguageModeBit"],
];
let fs = 0;
const flagsSegs = tqFlags.map((g) => {
  const r = { name: g.name, shift: fs, width: g.width };
  fs += g.width;
  return r;
});
for (let i = 0; i < 8; i++) {
  must(tqFlags[i].name === EXPECT_FLAGS[i][0], `tq flags[${i}] ${tqFlags[i].name} != ${EXPECT_FLAGS[i][0]}`);
  must(hFlags[i].name === EXPECT_FLAGS[i][1], `链[${i}] ${hFlags[i].name} != ${EXPECT_FLAGS[i][1]}`);
  must(tqFlags[i].width === hFlags[i].width && flagsSegs[i].shift === hFlags[i].shift, `flags[${i}] shift/width 失配`);
}

// ScriptOriginOptions 位标志 (oracle: -E 明文)
const ooo = ccI.match(/kIsSharedCrossOrigin = 1,\s*kIsOpaque = 1 << 1,\s*kIsWasm = 1 << 2,\s*kIsModule = 1 << 3/);
must(!!ooo, "ScriptOriginOptions 位标志 oracle 失配");

// ── 3. 布局 (64 位非压缩: map 8 + 19 tagged × 8) ──────────────────────────
const T = 8;
const offsets = tqFields.map((f, i) => [f, T + i * T]);
const headerSize = T + tqFields.length * T;
must(headerSize === 160, `Script header ${headerSize} != 160`);

// ── 4. inl / cc 方法清单 (oracle: 各自 -E 尾部区段) ───────────────────────
function tailMethods(src) {
  const tail = src.slice(src.lastIndexOf("\nnamespace v8 {"));
  return [...new Set([...tail.matchAll(/^[A-Za-z_][\w:<>,&* ]*?\b(\w+)::(~?\w+)\(/gm)].map((m) => `${m[1]}::${m[2]}`))];
}
const inlMethods = tailMethods(inlI).filter((d) => d.startsWith("Script::"));
must(inlMethods.length >= 40, `inl Script:: 方法 ${inlMethods.length} < 40`);
const ccMethods = tailMethods(ccI);
must(ccMethods.length >= 20, `cc 方法 ${ccMethods.length} < 20`);
for (const key of ["Script::GetPositionInfo", "Script::IsUserJavaScript", "GetPositionInfoSlow", "Script::GetPositionInfoInternal"]) {
  must(ccI.includes(key), `缺 ${key}`);
}

// ── 5. 产物 ────────────────────────────────────────────────────────────────
const body = `${banner("src/objects/script.cc.js", "src/objects/script.{-inl.h,cc,tq}", "Script tq 字段布局 + ScriptFlags 位段 + OriginOptions 位标志 + inl/cc 方法清单 (h 枚举归 script.h.js 席)")}
//! 差集骨架: Script 对象族的规则代码。布局按 64 位非压缩 (kTaggedSize=8)。
//! V8_SCRIPTORMODULE_LEGACY_LIFETIME 关: script_or_modules 字段不落地。

// ── Script tq 字段布局 (offset 表) ──
/// v8: src/objects/script.tq extern class Script (字段, offset)
pub const SCRIPT_TQ_FIELD_OFFSETS: &[(&str, u32)] = &[
${offsets.map(([f, o]) => `  ("${f}", ${o}),`).join("\n")}
];

/// v8: src/objects/script.h sizeof(Script) — map 8 + ${tqFields.length} tagged × 8 (非压缩)
pub const K_SCRIPT_HEADER_SIZE: usize = ${headerSize};

// ── ScriptFlags 位段 (SmiTagged 载体) ──
/// v8: src/objects/script.tq bitfield struct ScriptFlags extends uint31 (段名, shift, width)
pub const SCRIPT_TQ_FLAGS_SEGMENTS: [(&str, u8, u8); ${flagsSegs.length}] = [
${flagsSegs.map((s) => `  ("${s.name}", ${s.shift}, ${s.width}),`).join("\n")}
];

// ── ScriptOriginOptions 位标志 ──
/// v8: src/objects/script.h ScriptOriginOptions enum — origin_options 4 bit 载体
pub const K_IS_SHARED_CROSS_ORIGIN: i32 = 1;
/// v8: src/objects/script.h kIsOpaque
pub const K_IS_OPAQUE: i32 = 1 << 1;
/// v8: src/objects/script.h kIsWasm
pub const K_IS_WASM: i32 = 1 << 2;
/// v8: src/objects/script.h kIsModule
pub const K_IS_MODULE: i32 = 1 << 3;

// ── script-inl.h 方法清单 (${inlMethods.length} 个) ──
/// v8: src/objects/script-inl.h
pub const SCRIPT_INL_METHODS: &[&str] = &[
${inlMethods.map((d) => `  "${d}",`).join("\n")}
];

// ── script.cc 方法清单 (${ccMethods.length} 个) ──
/// v8: src/objects/script.cc
pub const SCRIPT_CC_METHODS: &[&str] = &[
${ccMethods.map((d) => `  "${d}",`).join("\n")}
];

// ── 对数不变式 ──
const fn width_sum(segs: &[(&str, u8, u8)]) -> u32 {
  let mut s = 0u32;
  let mut i = 0;
  while i < segs.len() {
    s += segs[i].2 as u32;
    i += 1;
  }
  s
}

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

const _: () = {
  assert!(SCRIPT_TQ_FIELD_OFFSETS.len() == 19);
  let f0 = SCRIPT_TQ_FIELD_OFFSETS[0];
  assert!(str_eq(f0.0, "source") && f0.1 == 8);
  let f13 = SCRIPT_TQ_FIELD_OFFSETS[13];
  assert!(str_eq(f13.0, "flags") && f13.1 == 112);
  let f18 = SCRIPT_TQ_FIELD_OFFSETS[18];
  assert!(str_eq(f18.0, "source_hash") && f18.1 == 152);
  assert!(K_SCRIPT_HEADER_SIZE == 160);
  assert!(SCRIPT_TQ_FLAGS_SEGMENTS.len() == 8);
  let g0 = SCRIPT_TQ_FLAGS_SEGMENTS[0];
  assert!(str_eq(g0.0, "compilation_kind") && g0.1 == 0 && g0.2 == 3);
  let g3 = SCRIPT_TQ_FLAGS_SEGMENTS[3];
  assert!(str_eq(g3.0, "origin_options") && g3.1 == 5 && g3.2 == 4);
  let g7 = SCRIPT_TQ_FLAGS_SEGMENTS[7];
  assert!(str_eq(g7.0, "outer_language_mode") && g7.1 == 12 && g7.2 == 1);
  assert!(width_sum(&SCRIPT_TQ_FLAGS_SEGMENTS) == 13);
  assert!(K_IS_SHARED_CROSS_ORIGIN == 1 && K_IS_OPAQUE == 2 && K_IS_WASM == 4 && K_IS_MODULE == 8);
  assert!(SCRIPT_INL_METHODS.len() >= 40);
  assert!(SCRIPT_CC_METHODS.len() >= 20);
};
`;

const p = writeSrc("v8-objects-js-func", "script_cc.rs", body);
console.log(
  `${p}: 字段 ${tqFields.length}/19 (header ${headerSize}), flags 位段 ${flagsSegs.length}/8 (总宽 13, .h 链对数), ` +
    `origin_options 4 标志, inl ${inlMethods.length} + cc ${ccMethods.length} 方法, 断言 ${checks} 项通过`,
);
