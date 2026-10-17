#!/usr/bin/env bun
// tool/src/objects/shared-function-info.cc.js — 覆盖 v8/src/objects/shared-function-info.{cc,tq}
//   (.h/-inl.h 规则代码归既有席 shared-function-info.h.js + 手写 shared_function_info.rs, 本脚本只做差集)
//   → v8-objects-js-func/src/shared_function_info_cc.rs
// 规则代码: tq 布局位段表 (SharedFunctionInfoFlags/Flags2, 与 .h BitField 链 clang -E 对数)
//   + tq extern class 清单 + cc 方法清单 + tq @export macro 语义 (formal parameter count)。

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

const tq = readV8("src/objects/shared-function-info.tq");
const iSrc = clangE("src/objects/shared-function-info.cc");

// ── 1. oracle: .h BitField 链 (clang -E 明文) → shift/width 表 ──────────────
// 链: FunctionKindBits = base::BitField<FunctionKind, 0, 5, uint32_t>; 后续 X = Prev::Next<T, W>
function chainFrom(firstAlias, firstShift, firstWidth) {
  const nexts = new Map();
  for (const m of iSrc.matchAll(/using (\w+)\s*=\s*(\w+)::Next<[\w:]+,\s*(\d+)>;/g)) {
    if (!nexts.has(m[2])) nexts.set(m[2], { name: m[1], width: Number(m[3]) });
  }
  const out = [{ name: firstAlias, shift: firstShift, width: firstWidth }];
  let cur = firstAlias;
  while (nexts.has(cur)) {
    const nx = nexts.get(cur);
    out.push({ name: nx.name, shift: out.at(-1).shift + out.at(-1).width, width: nx.width });
    cur = nx.name;
  }
  return out;
}
const hFlags = chainFrom("FunctionKindBits", 0, 5);
const hFlags2 = chainFrom("ClassScopeHasPrivateBrandBit", 0, 1);
must(hFlags.length === 17, `flags 链 ${hFlags.length} != 17`);
must(hFlags.at(-1).name === "IsHoistedInContextBit" && hFlags.at(-1).shift === 29, "flags 链尾应 IsHoistedInContextBit@29");
must(hFlags.reduce((a, s) => a + s.width, 0) === 30, "flags 总宽应 30");
must(hFlags2.length === 6 && hFlags2.at(-1).name === "FunctionContextIndependentCompiledBit", "flags2 链应 6 段");
must(hFlags2.reduce((a, s) => a + s.width, 0) === 8, "flags2 总宽应 8");

// ── 2. tq 位段表 → 与 oracle 对数 (段序/宽度 1:1) ──────────────────────────
function tqBitfield(name) {
  const m = tq.match(new RegExp(`bitfield struct ${name} extends \\w+ \\{([\\s\\S]*?)\\n\\}`));
  if (!m) throw new Error(`tq 缺 bitfield struct ${name}`);
  const segs = [];
  for (const line of m[1].split("\n")) {
    const s = line.replace(/\/\/.*$/, "").trim();
    const mm = s.match(/^(\w+): [\w<>|]+: (\d+) bit;$/);
    if (mm) segs.push({ name: mm[1], width: Number(mm[2]) });
  }
  return segs;
}
const tqFlags = tqBitfield("SharedFunctionInfoFlags");
const tqFlags2 = tqBitfield("SharedFunctionInfoFlags2");
must(tqFlags.length === 17, `tq flags 段数 ${tqFlags.length} != 17`);
must(tqFlags2.length === 6, `tq flags2 段数 ${tqFlags2.length} != 6`);
// 名字期望表 (tq snake ↔ .h BitField 别名), 与 oracle 双向对数
const EXPECT_FLAGS = [
  ["function_kind", "FunctionKindBits"], ["is_native", "IsNativeBit"], ["is_strict", "IsStrictBit"],
  ["function_syntax_kind", "FunctionSyntaxKindBits"], ["is_class_constructor", "IsClassConstructorBit"],
  ["has_duplicate_parameters", "HasDuplicateParametersBit"], ["allow_lazy_compilation", "AllowLazyCompilationBit"],
  ["function_map_index", "FunctionMapIndexBits"], ["disabled_optimization_reason", "DisabledOptimizationReasonBits"],
  ["requires_instance_members_initializer", "RequiresInstanceMembersInitializerBit"],
  ["construct_as_builtin", "ConstructAsBuiltinBit"],
  ["name_should_print_as_anonymous", "NameShouldPrintAsAnonymousBit"],
  ["has_reported_binary_coverage", "HasReportedBinaryCoverageBit"], ["is_top_level", "IsTopLevelBit"],
  ["properties_are_final", "PropertiesAreFinalBit"],
  ["private_name_lookup_skips_outer_class", "PrivateNameLookupSkipsOuterClassBit"],
  ["is_hoisted_in_context", "IsHoistedInContextBit"],
];
const EXPECT_FLAGS2 = [
  ["class_scope_has_private_brand", "ClassScopeHasPrivateBrandBit"],
  ["has_static_private_methods_or_accessors", "HasStaticPrivateMethodsOrAccessorsBit"],
  ["is_sparkplug_compiling", "IsSparkplugCompilingBit"], ["maglev_compilation_failed", "MaglevCompilationFailedBit"],
  ["cached_tiering_decision", "CachedTieringDecisionBits"],
  ["function_context_independent_compiled", "FunctionContextIndependentCompiledBit"],
];
for (let i = 0; i < 17; i++) {
  must(tqFlags[i].name === EXPECT_FLAGS[i][0] && tqFlags[i].width === hFlags[i].width, `flags 段[${i}] ${JSON.stringify(tqFlags[i])} 失配`);
  must(hFlags[i].name === EXPECT_FLAGS[i][1], `flags .h 链[${i}] ${hFlags[i].name} != ${EXPECT_FLAGS[i][1]}`);
}
for (let i = 0; i < 6; i++) {
  must(tqFlags2[i].name === EXPECT_FLAGS2[i][0] && tqFlags2[i].width === hFlags2[i].width, `flags2 段[${i}] 失配`);
  must(hFlags2[i].name === EXPECT_FLAGS2[i][1], `flags2 .h 链[${i}] ${hFlags2[i].name} != ${EXPECT_FLAGS2[i][1]}`);
}

// tq 累计 shift
const withShift = (segs) => {
  let s = 0;
  return segs.map((g) => {
    const r = { ...g, shift: s };
    s += g.width;
    return r;
  });
};
const flagsSegs = withShift(tqFlags);
const flags2Segs = withShift(tqFlags2);
for (let i = 0; i < 17; i++) must(flagsSegs[i].shift === hFlags[i].shift, `flags 段[${i}] shift 失配`);
for (let i = 0; i < 6; i++) must(flags2Segs[i].shift === hFlags2[i].shift, `flags2 段[${i}] shift 失配`);

// ── 3. tq extern class 清单 + SharedFunctionInfo 字段 ─────────────────────
const tqCls = [...tq.matchAll(/^@cppObjectLayoutDefinition\nextern class (\w+)/gm)].map((m) => m[1]);
must(tqCls.length === 10, `tq @cppObjectLayoutDefinition 类 ${tqCls.length} != 10`);
must(tqCls.join(",") === "PreparseData,InterpreterData,SharedFunctionInfo,SharedFunctionInfoWrapper,UncompiledData,UncompiledDataWithoutPreparseData,UncompiledDataWithPreparseData,UncompiledDataWithoutPreparseDataWithJob,UncompiledDataWithPreparseDataAndJob,OnHeapBasicBlockProfilerData", "tq 类清单失配");
function tqFields(cls) {
  const at = tq.search(new RegExp(`^extern class ${cls}\\b`, "m"));
  if (at < 0) throw new Error(`tq 缺类 ${cls}`);
  const body = tq.slice(at);
  const end = body.indexOf("\n}");
  const seg = body.slice(0, end);
  return [...seg.matchAll(/^  (?:@\w+\s+)?(\w+): [^;]+;/gm)].map((m) => m[1]);
}
const sfiFields = tqFields("SharedFunctionInfo");
must(sfiFields.length === 15, `SFI tq 字段 ${sfiFields.length} != 15:\n${sfiFields}`);
must(sfiFields.join(",") === "trusted_function_data,untrusted_function_data,name_or_scope_info,outer_scope_info_or_feedback_metadata,script,length,formal_parameter_count,function_token_offset,expected_nof_properties,flags2,flags,function_literal_id,unique_id,age,feedback_slot", "SFI tq 字段清单失配");
const clsFieldCounts = tqCls.map((c) => [c, tqFields(c).length]);
must(clsFieldCounts.find(([c]) => c === "SharedFunctionInfo")[1] === 15, "SFI 字段计数失配");

// tq @export macro: kDontAdaptArgumentsSentinel 语义锚
must(tq.includes("macro LoadSharedFunctionInfoFormalParameterCountWithoutReceiver"), "tq 缺 WithoutReceiver macro");
must(tq.includes("macro IsSharedFunctionInfoDontAdaptArguments"), "tq 缺 DontAdapt macro");
must(/kDontAdaptArgumentsSentinel/.test(tq), "tq 缺 kDontAdaptArgumentsSentinel");
// 哨兵值 oracle: globals.h 明文 constexpr uint16_t kDontAdaptArgumentsSentinel = 0
const sent = iSrc.match(/constexpr uint16_t kDontAdaptArgumentsSentinel = (\d+);/);
must(sent && sent[1] === "0", "kDontAdaptArgumentsSentinel 应为 0");

// ── 4. cc 方法清单 (oracle: -E 尾部用户区段) ──────────────────────────────
const tail = iSrc.slice(iSrc.lastIndexOf("\nnamespace v8 {"));
const defs = [...tail.matchAll(/^[A-Za-z_][\w:<>,&* ]*?\b(\w+)::(~?\w+)\(/gm)].map((m) => `${m[1]}::${m[2]}`);
const unique = [...new Set(defs)];
must(unique.length >= 40, `cc 方法定义 ${unique.length} < 40`);
const sfiMethods = unique.filter((d) => d.startsWith("SharedFunctionInfo::"));
must(sfiMethods.length >= 35, `SharedFunctionInfo:: 方法 ${sfiMethods.length} < 35`);
for (const key of ["SharedFunctionInfo::Hash", "SharedFunctionInfo::SetScript", "SharedFunctionInfo::UniqueIdInScript", "SharedFunctionInfo::DisableOptimization", "SharedFunctionInfo::StartPosition", "SharedFunctionInfo::EndPosition", "ScriptIterator::ScriptIterator"]) {
  must(tail.includes(key), `cc 缺 ${key}`);
}

// ── 5. 产物 ────────────────────────────────────────────────────────────────
const seg = (s) => `("${s.name}", ${s.shift}, ${s.width})`;
const body = `${banner("src/objects/shared-function-info.cc.js", "src/objects/shared-function-info.{cc,tq}", "tq 布局位段/类清单 + cc 方法清单 + formal-parameter-count macro 语义 (h 差集归 shared-function-info.h.js 席)")}
//! 差集骨架: shared-function-info.cc + .tq 的规则代码。
//! 位段经 .h BitField 链 (clang -E) 对数; 大函数体语义面留 Developer。

/// v8: src/objects/shared-function-info.tq bitfield struct SharedFunctionInfoFlags extends uint32
///   (段名, shift, width) — 与 .h FunctionKindBits..IsHoistedInContextBit 链 1:1, 总宽 30
pub const SFI_TQ_FLAGS_SEGMENTS: [(&str, u8, u8); ${flagsSegs.length}] = [
${flagsSegs.map((s) => `  ${seg(s)},`).join("\n")}
];

/// v8: src/objects/shared-function-info.tq bitfield struct SharedFunctionInfoFlags2 extends uint8
///   总宽 8 (uint8 全占)
pub const SFI_TQ_FLAGS2_SEGMENTS: [(&str, u8, u8); ${flags2Segs.length}] = [
${flags2Segs.map((s) => `  ${seg(s)},`).join("\n")}
];

/// v8: src/objects/shared-function-info.tq @cppObjectLayoutDefinition 类清单 (类名, 字段数)
pub const SFI_TQ_CLASSES: &[(&str, usize)] = &[
${clsFieldCounts.map(([c, n]) => `  ("${c}", ${n}),`).join("\n")}
];

/// v8: src/objects/shared-function-info.tq extern class SharedFunctionInfo 字段序 (布局序)
pub const SFI_TQ_SFI_FIELDS: &[&str] = &[
${sfiFields.map((f) => `  "${f}",`).join("\n")}
];

// ── cc 方法清单 (${unique.length} 个定义, oracle: clang -E 尾部区段) ──
/// v8: src/objects/shared-function-info.cc
pub const SHARED_FUNCTION_INFO_CC_METHODS: &[&str] = &[
${unique.map((d) => `  "${d}",`).join("\n")}
];

// ── tq @export macro 语义 (真实现) ──
/// v8: src/objects/shared-function-info.tq const kDontAdaptArgumentsSentinel
pub const K_DONT_ADAPT_ARGUMENTS_SENTINEL: u16 = 0;

/// v8: src/objects/shared-function-info.tq LoadSharedFunctionInfoFormalParameterCountWithoutReceiver
///   参数不适配哨兵时原样返回, 否则扣去 receiver 槽位 (kJSArgcReceiverSlots = 1)
#[inline]
#[must_use]
pub const fn formal_parameter_count_without_receiver(formal_parameter_count: u16) -> u16 {
  if formal_parameter_count != K_DONT_ADAPT_ARGUMENTS_SENTINEL {
    formal_parameter_count - 1
  } else {
    formal_parameter_count
  }
}

/// v8: src/objects/shared-function-info.tq LoadSharedFunctionInfoFormalParameterCountWithReceiver
#[inline]
#[must_use]
pub const fn formal_parameter_count_with_receiver(formal_parameter_count: u16) -> u16 {
  formal_parameter_count
}

/// v8: src/objects/shared-function-info.tq IsSharedFunctionInfoDontAdaptArguments
#[inline]
#[must_use]
pub const fn is_dont_adapt_arguments(formal_parameter_count: u16) -> bool {
  formal_parameter_count == K_DONT_ADAPT_ARGUMENTS_SENTINEL
}

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
  assert!(SFI_TQ_FLAGS_SEGMENTS.len() == 17);
  let s0 = SFI_TQ_FLAGS_SEGMENTS[0];
  assert!(str_eq(s0.0, "function_kind") && s0.1 == 0 && s0.2 == 5);
  let s3 = SFI_TQ_FLAGS_SEGMENTS[3];
  assert!(str_eq(s3.0, "function_syntax_kind") && s3.1 == 7 && s3.2 == 3);
  let s4 = SFI_TQ_FLAGS_SEGMENTS[4];
  assert!(str_eq(s4.0, "is_class_constructor") && s4.1 == 10 && s4.2 == 1);
  let s16 = SFI_TQ_FLAGS_SEGMENTS[16];
  assert!(str_eq(s16.0, "is_hoisted_in_context") && s16.1 == 29 && s16.2 == 1);
  assert!(width_sum(&SFI_TQ_FLAGS_SEGMENTS) == 30);
  assert!(SFI_TQ_FLAGS2_SEGMENTS.len() == 6);
  let f2 = SFI_TQ_FLAGS2_SEGMENTS[4];
  assert!(str_eq(f2.0, "cached_tiering_decision") && f2.1 == 4 && f2.2 == 3);
  assert!(width_sum(&SFI_TQ_FLAGS2_SEGMENTS) == 8);
  assert!(SFI_TQ_CLASSES.len() == 10);
  assert!(SFI_TQ_SFI_FIELDS.len() == 15);
  assert!(str_eq(SFI_TQ_SFI_FIELDS[0], "trusted_function_data"));
  assert!(str_eq(SFI_TQ_SFI_FIELDS[14], "feedback_slot"));
  assert!(SHARED_FUNCTION_INFO_CC_METHODS.len() >= 40);
  assert!(K_DONT_ADAPT_ARGUMENTS_SENTINEL == 0);
  assert!(formal_parameter_count_without_receiver(3) == 2);
  assert!(formal_parameter_count_without_receiver(0) == 0);
  assert!(is_dont_adapt_arguments(0));
  assert!(!is_dont_adapt_arguments(4));
};
`;

const p = writeSrc("v8-objects-js-func", "shared_function_info_cc.rs", body);
console.log(
  `${p}: flags 位段 ${flagsSegs.length}/17+${flags2Segs.length}/6 (总宽 30/8, .h 链对数), tq 类 ${tqCls.length}/10, ` +
    `SFI 字段 ${sfiFields.length}/15, cc 方法 ${unique.length}, 断言 ${checks} 项通过`,
);
