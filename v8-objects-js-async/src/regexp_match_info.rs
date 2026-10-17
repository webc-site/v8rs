// 由 tool/src/objects/regexp-match-info.h.js 从 v8/src/objects/regexp-match-info.{-inl.h,cc,tq} 生成; 修订就地改, 复跑覆盖
// 正则匹配结果槽: 具名字段 + 变长 capture 寄存器对
//! RegExpMatchInfo: 正则上次匹配的捕获组边界寄存器 (FixedArrayBase 派生)。
//!
//! 布局: 3 具名头字段 + 变长 Smi 槽 (捕获组 i 的 start/end 位于 objects[i*2] / objects[i*2+1]);
//! 容量 = (capture_count + 1) * 2 (组 0 = 整体匹配)。

// ── 布局 (tq, 源码序) ──
pub const REGEXP_MATCH_INFO_FIELDS: [(&str, &str); 3] = [
  ("number_of_capture_registers", "Smi"),
  ("last_subject", "String"),
  ("last_input", "Object"),
];
/// 变长槽 (tq objects[length]: Smi; 捕获寄存器对)。
pub const REGEXP_MATCH_INFO_VAR_SLOT: &str = "objects[length]: Smi";

// ── capture 槽位公式 (tq 宏 1:1) ──
/// 捕获组 i 的 start 寄存器下标 (原版 GetStartOfCapture: i * 2)。
#[inline]
#[must_use]
pub const fn start_of_capture(i: usize) -> usize {
  i * 2
}

/// 捕获组 i 的 end 寄存器下标 (原版 GetEndOfCapture: i * 2 + 1)。
#[inline]
#[must_use]
pub const fn end_of_capture(i: usize) -> usize {
  i * 2 + 1
}

/// 所需容量 (原版 JSRegExp::RegistersForCaptureCount: (capture_count + 1) * 2)。
#[inline]
#[must_use]
pub const fn registers_for_capture_count(capture_count: usize) -> usize {
  (capture_count + 1) * 2
}

// ── .cc 签名骨架 (深水推迟台账, 依 readme/zh.md) ──
/// v8: src/objects/regexp-match-info.cc RegExpMatchInfo::New。
pub fn new() {
  todo!(
    "RegExpMatchInfo::New: 依赖 Isolate + FixedArray alloc + MemsetTagged + 空 subject/input; 解锁 v8-execution-isolate + v8-objects-arrays"
  )
}

/// v8: src/objects/regexp-match-info.cc RegExpMatchInfo::ReserveCaptures。
pub fn reserve_captures() {
  todo!(
    "RegExpMatchInfo::ReserveCaptures: 依赖 Isolate + CopyElements + 新建 Handle + 平移; 解锁 v8-execution-isolate + v8-objects-arrays + v8-handles"
  )
}

// ── 提取不变式断言 ── (const 上下文无 PartialEq<&str>, 用字节循环 const fn)
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
  assert!(REGEXP_MATCH_INFO_FIELDS.len() == 3);
  assert!(str_eq(
    REGEXP_MATCH_INFO_FIELDS[0].0,
    "number_of_capture_registers"
  ));
  assert!(start_of_capture(0) == 0);
  assert!(end_of_capture(0) == 1);
  assert!(start_of_capture(3) == 6);
  assert!(end_of_capture(3) == 7);
  assert!(registers_for_capture_count(2) == 6);
  assert!(registers_for_capture_count(0) == 2);
};
