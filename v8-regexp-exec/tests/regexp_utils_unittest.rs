//! RegExpUtils 族规约测试 (v8-regexp-exec)。
//!
//! 规约来源 (实测枚举): v8/test/unittests/regexp/ 下无 AdvanceStringIndex/IsMatchedCapture
//! 直接用例 (unittests/regexp 仅 regexp-unittest.cc/regexp-fuzzer.cc, grep 实测 0 命中;
//! 二者经 mjsunit/builtins 消费)。按纯函数黄金值 1:1 复刻 regexp-utils.cc:187-203 / 42-55
//! 与 regexp-match-info.h:60-65 索引公式的分支语义。
//! 跳过登记: GenericCaptureGetter/SetLastIndex/GetLastIndex/RegExpExec/
//! IsUnmodifiedRegExp/SetAdvancedStringIndex — Isolate/句柄硬耦合, 推迟 (见模块头登记)。

use aok::{OK, Void};
use v8_regexp_exec::regexp_utils::{
  advance_string_index, capture_end_index, capture_start_index, is_matched_capture,
};

#[ctor::ctor(unsafe)]
fn _log_init() {
  log_init::init();
}

/// AdvanceStringIndex 黄金值 (cc:187-203 全分支)
#[test]
fn advance_string_index_golden() -> Void {
  // 非 unicode: 恒 +1, 即便 index 处是完整代理对 (cc:191 unicode 短路)
  let pair = [0xD83Du16, 0xDE00u16];
  assert_eq!(advance_string_index(&pair, 0, false), 1);
  // unicode + 合法代理对 → +2
  assert_eq!(advance_string_index(&pair, 0, true), 2);
  // lead 在尾位 (index+1 >= len) → +1 (cc:193 第二条件)
  assert_eq!(advance_string_index(&pair[..1], 0, true), 1);
  // lead + 非 trail (lead/lead) → +1
  let bad = [0xD83Du16, 0xD83Du16];
  assert_eq!(advance_string_index(&bad, 0, true), 1);
  // 单独 trail 起始 → +1 (非 lead 区间)
  let trail = [0xDE00u16, 0x41u16];
  assert_eq!(advance_string_index(&trail, 0, true), 1);
  // BMP 字符 → +1
  assert_eq!(advance_string_index(&[0x0041u16, 0x0042u16], 0, true), 1);
  // index 越尾 (== len) → +1 (cc:191 首条件短路)
  assert_eq!(advance_string_index(&pair, 2, true), 3);
  assert_eq!(advance_string_index(&[], 0, true), 1);
  // 区间界: 0xD800/0xDBFF 均算 lead, 0xDC00/0xDFFF 均算 trail
  let edge = [0xD800u16, 0xDFFFu16, 0xDBFFu16, 0xDC00u16];
  assert_eq!(advance_string_index(&edge, 0, true), 2);
  assert_eq!(advance_string_index(&edge, 2, true), 4);
  // 界外: 0xDC00 非 lead → +1
  assert_eq!(advance_string_index(&edge[1..], 0, true), 1);
  OK
}

/// IsMatchedCapture + 索引公式 (cc:42-55; match-info.h:60-65)
#[test]
fn is_matched_capture_registers() -> Void {
  // 索引公式金标准: start=2c, end=2c+1
  assert_eq!(capture_start_index(0), 0);
  assert_eq!(capture_end_index(0), 1);
  assert_eq!(capture_start_index(3), 6);
  assert_eq!(capture_end_index(3), 7);

  // 寄存器组: 组0 [10,15] 命中, 组1 [-1,-1] 未命中, 组2 [3,4] 命中 (寄存器0=整体匹配)
  let captures = [10, 15, -1, -1, 3, 4];
  let count = 6;
  assert!(is_matched_capture(&captures, count, 0));
  assert!(!is_matched_capture(&captures, count, 1));
  assert!(is_matched_capture(&captures, count, 2));
  // 哨兵 -1 恒 false (cc:44)
  assert!(!is_matched_capture(&captures, count, -1));
  // start >= register_count → false (cc:47): 组3 start=6 >= 6
  assert!(!is_matched_capture(&captures, count, 3));
  // 半开区 (end 越数组尾, 契约要求偶数寄存器数) → get 防御 false
  let short = [10, 15, -1, -1, 3];
  assert!(!is_matched_capture(&short, 5, 2));
  // 仅 end 为 -1 → false (cc:54 双条件)
  let partial = [10, 15, 2, -1];
  assert!(!is_matched_capture(&partial, 4, 1));
  // 负索引 (非 -1 哨兵, 契约外) → false 而非崩溃
  assert!(!is_matched_capture(&captures, count, -2));
  OK
}
