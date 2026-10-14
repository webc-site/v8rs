// 由 tool/src/objects/compilation-cache-table.cc.js 从 v8/src/objects/compilation-cache-table.{h,-inl.h,cc} 生成; 修订就地改, 复跑覆盖
// 编译缓存表: script/eval/regexp 三类缓存规则面

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
pub const SCRIPT_CACHE_KEY_HASH: usize = 0;
/// 弱 Script 指针槽。
pub const SCRIPT_CACHE_KEY_WEAK_SCRIPT: usize = 1;
/// 布局长度。
pub const SCRIPT_CACHE_KEY_END: usize = 2;

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
///
/// v8: src/objects/compilation-cache-table.cc CompilationCacheTable::LookupScript
pub fn lookup_script(_table: u64, _src: u64, _isolate: u64) -> u64 {
  todo!(
    "L4: 需 WeakFixedArray 弱 Script 键 + Script/SharedFunctionInfo 存活校验 + HashTable<Shape> 全模型"
  )
}
/// 写 script 缓存条目 (PutScript)。
///
/// v8: src/objects/compilation-cache-table.cc CompilationCacheTable::PutScript
pub fn put_script(_cache: u64, _src: u64, _value: u64, _isolate: u64) -> u64 {
  todo!("L4: 需 Factory 建 WeakFixedArray 键 + HashTable Add/EnsureCapacity + GC 写屏障")
}
/// 查 eval 缓存 (LookupEval → InfoCellPair)。
///
/// v8: src/objects/compilation-cache-table.cc CompilationCacheTable::LookupEval
pub fn lookup_eval(
  _table: u64,
  _src: u64,
  _shared: u64,
  _language_mode: u32,
  _position: i32,
) -> u64 {
  // eval_hash const fn 已落上方; 实体查找需 CowFixedArray 键比较 + JSFunction/InfoCellPair
  todo!("L4: 需 eval_hash 下游 HashTable<Shape> 查找 + SharedFunctionInfo/JSFunction 对象模型")
}
/// 为既有 eval 条目补记 JSFunction (UpdateEval)。
///
/// v8: src/objects/compilation-cache-table.cc CompilationCacheTable::UpdateEval
pub fn update_eval(
  _table: u64,
  _src: u64,
  _outer_info: u64,
  _js_function: u64,
  _language_mode: u32,
  _position: i32,
) {
  todo!("L4: 需 WeakFixedArray JSFunctions 追加 + JSFunction/SharedFunctionInfo 全模型")
}
/// 写 eval 缓存条目 (PutEval)。
///
/// v8: src/objects/compilation-cache-table.cc CompilationCacheTable::PutEval
pub fn put_eval(
  _cache: u64,
  _src: u64,
  _outer_info: u64,
  _js_function: u64,
  _position: i32,
) -> u64 {
  todo!("L4: 需 Factory 建 CowFixedArray 键 + 首插哈希/代数占位 (kHashGenerations) + HashTable Add")
}
/// 查 regexp 缓存 (LookupRegExp; 返回 RegExpData)。
///
/// v8: src/objects/compilation-cache-table.cc CompilationCacheTable::LookupRegExp
pub fn lookup_regexp(_table: u64, _source: u64, _flags: u32) -> u64 {
  // regexp_hash const fn 已落上方; 实体查找需 RegExpDataWrapper 键比较 (字符串内容级) → L4
  todo!("L4: 需 regexp_hash 下游 HashTable 查找 + RegExpDataWrapper 源串等价比较")
}
/// 写 regexp 缓存条目 (PutRegExp)。
///
/// v8: src/objects/compilation-cache-table.cc CompilationCacheTable::PutRegExp
pub fn put_regexp(_isolate: u64, _cache: u64, _src: u64, _flags: u32, _value: u64) -> u64 {
  todo!("L4: 需 Factory 建 RegExpDataWrapper 键值 + HashTable<Shape> Add/EnsureCapacity")
}
/// 按 value 移除条目 (Remove)。
///
/// v8: src/objects/compilation-cache-table.cc CompilationCacheTable::Remove
pub fn remove(_table: u64, _value: u64) {
  todo!("L4: 需 HashTable<Shape> 全表遍历按 value 匹配 + DeleteEntry (依赖基座 hash_table 面)")
}
/// 按 entry 移除 (RemoveEntry)。
///
/// v8: src/objects/compilation-cache-table.cc CompilationCacheTable::RemoveEntry
pub fn remove_entry(_table: u64, _entry: u32) {
  todo!("L4: 需 HashTable<Shape>::DeleteEntry (the_hole 清槽 + 计数) 基座实现")
}
/// 扩容保障 (EnsureScriptTableCapacity)。
///
/// v8: src/objects/compilation-cache-table.cc CompilationCacheTable::EnsureScriptTableCapacity
pub fn ensure_script_table_capacity(_isolate: u64, _cache: u64) -> u64 {
  todo!("L4: 需 HashTable<Shape> EnsureCapacity → Factory 扩容重哈希 (依赖基座 hash_table 面)")
}

// compilation-cache-table.cc 方法面 (对数基准, 10 项):
// CompilationCacheTable::EnsureScriptTableCapacity
// CompilationCacheTable::LookupEval
// CompilationCacheTable::LookupRegExp
// CompilationCacheTable::LookupScript
// CompilationCacheTable::PutEval
// CompilationCacheTable::PutRegExp
// CompilationCacheTable::PutScript
// CompilationCacheTable::Remove
// CompilationCacheTable::RemoveEntry
// CompilationCacheTable::UpdateEval

// —— 提取不变式断言 ——
const _: () = {
  assert!(K_PREFIX_SIZE == 0 && K_ENTRY_SIZE == 3 && K_HASH_GENERATIONS == 10);
  assert!(K_MATCH_NEEDS_HOLE_CHECK && !K_DO_HASH_SPREADING && K_HASH_BITS == 0);
  assert!(K_PRIMARY_VALUE_OFFSET == 1 && K_EVAL_JS_FUNCTIONS_VALUE_OFFSET == 2);
  assert!(
    SCRIPT_CACHE_KEY_HASH == 0 && SCRIPT_CACHE_KEY_WEAK_SCRIPT == 1 && SCRIPT_CACHE_KEY_END == 2
  );
  assert!(EVAL_CACHE_KEY_LENGTH == 4 && EVAL_CACHE_KEY_SOURCE == 1);
  assert!(regexp_hash(10, 3) == 13);
  assert!(eval_hash(0x1234, Some(0x0F0F), true, 7) == (0x1234 ^ 0x0F0F ^ 0x8000) + 7);
  assert!(eval_hash(5, None, false, 1) == 6);
};
