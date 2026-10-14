// v8-regexp-exec: RegExpUtils — v8/src/regexp/regexp-utils.h 族 1:1 对数
// 由 tool/src/regexp/regexp-utils.h.js 对数审计 (骨架: regexp_utils.rs.skel)
//
// 原版 regexp::Utils (AllStatic) 共 8 个静态方法。本席落地其纯逻辑面
// (参数注入替代 Isolate/句柄依赖, glossary "L3 前占位" 规则):
// - AdvanceStringIndex → advance_string_index (String 注入为 UTF-16 码元切片)
// - IsMatchedCapture   → is_matched_capture  (match_info 注入为捕获寄存器组 + 计数)
// - capture_start_index/capture_end_index 索引公式 (regexp-match-info.h:60-65)
//
// 推迟登记 (Isolate/DirectHandle 硬耦合面, 待 objects-js L3, 与 REG-1b 双引用):
// - GenericCaptureGetter  (factory NewSubString/empty_string)
// - SetLastIndex / GetLastIndex (JSReceiver 属性存取)
// - RegExpExec            (Execution::Call / TypeError 抛出)
// - IsUnmodifiedRegExp    (Map/Protectors/描述符检查)
// - SetAdvancedStringIndex (h:54, 句柄版 AdvanceStringIndex: DirectHandle<JSReceiver> 属性写)
use v8_common::K_MAX_SAFE_INTEGER_UINT64;

/// 占位标记 (待 objects-js / v8-execution L3 对接)
pub struct IsolateMarker;

/// UTF-16 前导代理下界 (原版 regexp-utils.cc:193 硬编码 0xD800)
const LEAD_LOW: u16 = 0xD800;
/// UTF-16 前导代理上界 (原版 regexp-utils.cc:193 硬编码 0xDBFF)
const LEAD_HIGH: u16 = 0xDBFF;
/// UTF-16 尾随代理下界 (原版 regexp-utils.cc:196 硬编码 0xDC00)
const TRAIL_LOW: u16 = 0xDC00;
/// UTF-16 尾随代理上界 (原版 regexp-utils.cc:196 硬编码 0xDFFF)
const TRAIL_HIGH: u16 = 0xDFFF;

/// 原版 RegExpMatchInfo::capture_start_index (regexp-match-info.h:60-62)
#[inline]
pub const fn capture_start_index(capture: i32) -> i32 {
  capture * 2
}

/// 原版 RegExpMatchInfo::capture_end_index (regexp-match-info.h:63-65)
#[inline]
pub const fn capture_end_index(capture: i32) -> i32 {
  capture * 2 + 1
}

/// 原版 Utils::IsMatchedCapture (regexp-utils.cc:42-55) —
/// 判定捕获组 `capture` 是否参与本次匹配。match_info 参数注入:
/// `captures` 为捕获寄存器组 (Smi 值, -1 = 未命中), `register_count` 即
/// number_of_capture_registers。原版依赖对象内直读越界未定义;
/// Rust 以 get 防御: 越界 (含负索引回绕) 一律按未命中处理。
pub fn is_matched_capture(captures: &[i32], register_count: i32, capture: i32) -> bool {
  // 哨兵: -1 恒未命中 (cc:44, GenericCaptureGetter 等失败指示)
  if capture == -1 {
    return false;
  }
  let start = capture_start_index(capture);
  // 越出寄存器组 (cc:47)
  if start >= register_count {
    return false;
  }
  let end = capture_end_index(capture);
  // SAFETY-free: get 承接原版 capture(index) 直读 (regexp-match-info-inl.h:42)
  let (Some(&match_start), Some(&match_end)) = (
    captures.get(usize::try_from(start).unwrap_or(usize::MAX)),
    captures.get(usize::try_from(end).unwrap_or(usize::MAX)),
  ) else {
    return false;
  };
  match_start != -1 && match_end != -1
}

/// 原版 Utils::AdvanceStringIndex (regexp-utils.cc:187-203) —
/// https://tc39.es/ecma262/#sec-advancestringindex。String 参数注入为
/// UTF-16 码元切片: unicode 且 index 处为合法代理对 (lead+trail) 时前进 2,
/// 否则前进 1 (含 index ≥ 长度的越尾情形)。
pub fn advance_string_index(code_units: &[u16], index: u64, unicode: bool) -> u64 {
  // 原版 DCHECK_LE(index, kMaxSafeInteger) (cc:189)
  debug_assert!(index <= K_MAX_SAFE_INTEGER_UINT64);
  let string_length = code_units.len() as u64;
  if unicode && index < string_length {
    // index < len 保证直接索引不越界
    let first = code_units[index as usize];
    if (LEAD_LOW..=LEAD_HIGH).contains(&first) && index + 1 < string_length {
      let second = code_units[index as usize + 1];
      if (TRAIL_LOW..=TRAIL_HIGH).contains(&second) {
        return index + 2;
      }
    }
  }
  index + 1
}
