#!/usr/bin/env bun
// tool/src/objects/compilation-cache-table.cc.js — 覆盖 compilation-cache-table 同名族:
//   v8/src/objects/compilation-cache-table.h + compilation-cache-table-inl.h + compilation-cache-table.cc
//   → v8-objects-collections/src/compilation_cache_table.rs
// 机械提取: CompilationCacheShape 布局 + kHashGenerations + 三类缓存哈希公式
//   (RegExpHash/EvalHash 纯计算段落 const fn; ScriptCacheKey::Index 枚举);
// 堆访问方法 (LookupScript/PutEval 等 10 个) 留签名骨架 + todo!("就地修订")。

import { readV8, writeSrc, banner } from "../../lib.js";

const h = readV8("src/objects/compilation-cache-table.h");
const inl = readV8("src/objects/compilation-cache-table-inl.h");
const cc = readV8("src/objects/compilation-cache-table.cc");

// ── 锚点 (must 断言) ──────────────────────────────────────────────────────
const anchors = [
  [h, "class CompilationCacheShape : public BaseShape<HashTableKey*> {", "compilation-cache-table.h"],
  [h, "static const int kPrefixSize = 0;", "compilation-cache-table.h"],
  [h, "static const int kEntrySize = 3;", "compilation-cache-table.h"],
  [h, "static const bool kMatchNeedsHoleCheck = true;", "compilation-cache-table.h"],
  [h, "static const bool kDoHashSpreading = false;", "compilation-cache-table.h"],
  [h, "static const uint32_t kHashBits = 0;", "compilation-cache-table.h"],
  [h, "static constexpr int kHashGenerations = 10;", "compilation-cache-table.h"],
  [inl, "return string->EnsureHash() + flags.value();", "compilation-cache-table-inl.h (RegExpHash)"],
  [inl, "if (is_strict(language_mode)) hash ^= 0x8000;", "compilation-cache-table-inl.h (EvalHash)"],
  [inl, "hash += position;", "compilation-cache-table-inl.h (EvalHash)"],
  [inl, "static_assert(CompilationCacheShape::kEntrySize == 3);", "compilation-cache-table-inl.h"],
  [cc, "CompilationCacheScriptLookupResult CompilationCacheTable::LookupScript(", "compilation-cache-table.cc"],
  [inl, "class ScriptCacheKey : public HashTableKey {", "compilation-cache-table-inl.h"],
  [cc, "class EvalCacheKey : public HashTableKey {", "compilation-cache-table.cc"],
];
for (const [s, sub, f] of anchors) {
  if (!s.includes(sub)) throw new Error(`锚点失配: ${f} 无 "${sub.slice(0, 52)}..."`);
}

// ScriptCacheKey::Index 枚举 (inl, 顺序值)
const idxM = inl.match(/enum Index \{\s*kHash,\s*kWeakScript,\s*kEnd,\s*\};/);
if (!idxM) throw new Error("ScriptCacheKey::Index 枚举失配");
const SCRIPT_CACHE_KEY_INDEX = { kHash: 0, kWeakScript: 1, kEnd: 2 };
if (SCRIPT_CACHE_KEY_INDEX.kEnd !== 2) throw new Error("ScriptCacheKey::kEnd != 2");

// Eval 缓存键 FixedArray 布局 (inl HashForObject: [shared, source, language, position], 长 4)
if (!inl.includes("DCHECK_EQ(4u, val->ulength().value());")) throw new Error("Eval 键长度锚失配");
if (!inl.includes("Cast<String>(val->get(1))")) throw new Error("Eval 键 source 槽位锚失配");

// cc 方法面 (数量 must 断言: 10 个)
const ccDefs = [...new Set([...cc.matchAll(/CompilationCacheTable::(\w+)\(/g)].map((m) => m[1]))].sort();
const EXPECTED = ["EnsureScriptTableCapacity", "LookupEval", "LookupRegExp", "LookupScript", "PutEval", "PutRegExp", "PutScript", "Remove", "RemoveEntry", "UpdateEval"].sort();
if (ccDefs.join(",") !== EXPECTED.join(",")) {
  throw new Error(`CompilationCacheTable 方法面漂移 (${ccDefs.length}): ${ccDefs.join(",")}`);
}
if (ccDefs.length !== 10) throw new Error(`cc 方法应 10, 实得 ${ccDefs.length}`);

// 键类归属: ScriptCacheKey 在 -inl.h, EvalCacheKey 在 .cc (regexp 无独立键类)
if (!inl.includes("class ScriptCacheKey : public HashTableKey {")) throw new Error("-inl.h 缺 ScriptCacheKey");
if (!cc.includes("class EvalCacheKey : public HashTableKey {")) throw new Error(".cc 缺 EvalCacheKey");

const body = `${banner("src/objects/compilation-cache-table.cc.js", "src/objects/compilation-cache-table.{h,-inl.h,cc}", "编译缓存表: script/eval/regexp 三类缓存规则面")}
//! CompilationCacheTable: HashTable<_, CompilationCacheShape>。
//!
//! - 每 entry 3 槽 (eval 缓存需要): [key, primary_value, eval_js_functions]
//! - script 缓存键 = WeakFixedArray(弱 Script 指针); eval 缓存键 = Number(哈希) /
//!   CowFixedArray[shared, source, language, position]; regexp 键值皆 RegExpDataWrapper
//! - eval 首插只记哈希 + 代数, 存活 kHashGenerations 次 GC 后由 Age 清退

// ── CompilationCacheShape (compilation-cache-table.h:41-49) ────────────
/// 前缀槽数。
pub const K_PREFIX_SIZE: usize = 0;
/// 每 entry 槽数 (eval 缓存需要 3 槽)。
pub const K_ENTRY_SIZE: usize = 3;
/// IsMatch 需处理 the_hole 删除槽。
pub const K_MATCH_NEEDS_HOLE_CHECK: bool = true;
/// 不启用大表哈希分散。
pub const K_DO_HASH_SPREADING: bool = false;
/// 哈希位数 (0 = 不分散)。
pub const K_HASH_BITS: u32 = 0;
/// eval 占位插入的存活代数 (kHashGenerations)。
pub const K_HASH_GENERATIONS: i32 = 10;

// ── entry 槽位 (compilation-cache-table-inl.h:29-53) ───────────────────
/// entry 内主值槽 (PrimaryValueAt = EntryToIndex + 1)。
pub const K_PRIMARY_VALUE_OFFSET: usize = 1;
/// entry 内 eval JSFunctions 弱数组槽 (+2; 仅 eval 缓存使用)。
pub const K_EVAL_JS_FUNCTIONS_VALUE_OFFSET: usize = 2;

// ── ScriptCacheKey (compilation-cache-table-inl.h:59-) ─────────────────
/// script 缓存键 WeakFixedArray 布局 (enum Index 顺序值)。
pub const SCRIPT_CACHE_KEY_HASH: usize = ${SCRIPT_CACHE_KEY_INDEX.kHash};
/// 弱 Script 指针槽。
pub const SCRIPT_CACHE_KEY_WEAK_SCRIPT: usize = ${SCRIPT_CACHE_KEY_INDEX.kWeakScript};
/// 布局长度。
pub const SCRIPT_CACHE_KEY_END: usize = ${SCRIPT_CACHE_KEY_INDEX.kEnd};

// ── eval 缓存键 FixedArray 布局 (inl HashForObject 尾段, 长 4) ────────
/// [shared, source, language, position] — 首插形态键为 Number(哈希)。
pub const EVAL_CACHE_KEY_LENGTH: usize = 4;
/// source 字符串槽。
pub const EVAL_CACHE_KEY_SOURCE: usize = 1;
/// 未检查 language_mode 槽。
pub const EVAL_CACHE_KEY_LANGUAGE: usize = 2;
/// position 槽。
pub const EVAL_CACHE_KEY_POSITION: usize = 3;

// ── 哈希公式 (纯计算段, const fn) ─────────────────────────────────────
/// regexp 缓存哈希 = EnsureHash(source) + flags (inl:109 RegExpHash)。
#[inline]
#[must_use]
pub const fn regexp_hash(source_hash: u32, flags: i32) -> u32 {
  source_hash.wrapping_add(flags as u32)
}

/// eval 缓存哈希 (inl:113 EvalHash)。
///
/// source 哈希起算; shared 含源码时异或 script 源哈希 (GC 存活关键);
/// strict 模式异或 0x8000 (LanguageModeSize==2); 终加 position。
#[inline]
#[must_use]
pub const fn eval_hash(
  source_hash: u32,
  script_source_hash: Option<u32>,
  is_strict: bool,
  position: i32,
) -> u32 {
  let hash = source_hash;
  let hash = match script_source_hash {
    Some(sh) => hash ^ sh,
    None => hash,
  };
  let hash = if is_strict { hash ^ 0x8000 } else { hash };
  hash.wrapping_add(position as u32)
}

// ── 堆访问签名骨架 (就地修订; 参数为 Address/索引占位) ─────────────────
/// 查 script 缓存条目 (LookupScript; 三态结果见 CompilationCacheScriptLookupResult)。
pub fn lookup_script(_table: u64, _src: u64, _isolate: u64) -> u64 { todo!("就地修订") }
/// 写 script 缓存条目 (PutScript)。
pub fn put_script(_cache: u64, _src: u64, _value: u64, _isolate: u64) -> u64 { todo!("就地修订") }
/// 查 eval 缓存 (LookupEval → InfoCellPair)。
pub fn lookup_eval(_table: u64, _src: u64, _shared: u64, _language_mode: u32, _position: i32) -> u64 { todo!("就地修订") }
/// 为既有 eval 条目补记 JSFunction (UpdateEval)。
pub fn update_eval(_table: u64, _src: u64, _outer_info: u64, _js_function: u64, _language_mode: u32, _position: i32) { todo!("就地修订") }
/// 写 eval 缓存条目 (PutEval)。
pub fn put_eval(_cache: u64, _src: u64, _outer_info: u64, _js_function: u64, _position: i32) -> u64 { todo!("就地修订") }
/// 查 regexp 缓存 (LookupRegExp; 返回 RegExpData)。
pub fn lookup_regexp(_table: u64, _source: u64, _flags: u32) -> u64 { todo!("就地修订") }
/// 写 regexp 缓存条目 (PutRegExp)。
pub fn put_regexp(_isolate: u64, _cache: u64, _src: u64, _flags: u32, _value: u64) -> u64 { todo!("就地修订") }
/// 按 value 移除条目 (Remove)。
pub fn remove(_table: u64, _value: u64) { todo!("就地修订") }
/// 按 entry 移除 (RemoveEntry)。
pub fn remove_entry(_table: u64, _entry: u32) { todo!("就地修订") }
/// 扩容保障 (EnsureScriptTableCapacity)。
pub fn ensure_script_table_capacity(_isolate: u64, _cache: u64) -> u64 { todo!("就地修订") }

// compilation-cache-table.cc 方法面 (对数基准, ${ccDefs.length} 项):
${ccDefs.map((s) => `// CompilationCacheTable::${s}`).join("\n")}

// —— 提取不变式断言 ——
const _: () = {
  assert!(K_PREFIX_SIZE == 0 && K_ENTRY_SIZE == 3 && K_HASH_GENERATIONS == 10);
  assert!(K_MATCH_NEEDS_HOLE_CHECK && !K_DO_HASH_SPREADING && K_HASH_BITS == 0);
  assert!(K_PRIMARY_VALUE_OFFSET == 1 && K_EVAL_JS_FUNCTIONS_VALUE_OFFSET == 2);
  assert!(SCRIPT_CACHE_KEY_HASH == 0 && SCRIPT_CACHE_KEY_WEAK_SCRIPT == 1 && SCRIPT_CACHE_KEY_END == 2);
  assert!(EVAL_CACHE_KEY_LENGTH == 4 && EVAL_CACHE_KEY_SOURCE == 1);
  assert!(regexp_hash(10, 3) == 13);
  assert!(eval_hash(0x1234, Some(0x0F0F), true, 7) == (0x1234 ^ 0x0F0F ^ 0x8000) + 7);
  assert!(eval_hash(5, None, false, 1) == 6);
};
`;

writeSrc("v8-objects-collections", "compilation_cache_table.rs", body);
console.log(
  `compilation_cache_table.rs: Shape 常量 6 + 槽位 2 + 键布局 7 + const fn 2 + 骨架 ${ccDefs.length}, 对数通过`,
);
