// 由 tool/src/flags/flags-impl.h.js 从 v8/src/flags/flags-impl.h 生成; 修订就地改, 复跑覆盖
// 查找层: FlagHelpers 名字归一比较 + 按名查找

use std::{cmp::Ordering, str::from_utf8};

use crate::defs;

/// 主 flag 下标 (0..NUM_FLAGS, 顺序=原版 flags[] 即 defs::FLAGS)
pub type FlagId = u32;

/// 名字归一化字节 ('_' 视同 '-'; 原版 FlagHelpers::NormalizeChar)
#[inline]
pub(crate) const fn norm_byte(ch: u8) -> u8 {
  if ch == b'_' { b'-' } else { ch }
}

/// 归一化名字比较 (原版 FlagHelpers::FlagNamesCmp: 逐字节归一比较, 先耗尽者小)
/// 返回 Ordering (原版返回 -1/0/1)
pub fn cmp_names(a: &str, b: &str) -> Ordering {
  a.bytes().map(norm_byte).cmp(b.bytes().map(norm_byte))
}

/// 名字相等 (归一化后; 原版 FlagHelpers::EqualNames)
#[inline]
pub fn equal_names(a: &str, b: &str) -> bool {
  cmp_names(a, b) == Ordering::Equal
}

/// a 与 b 前缀相等且 b 以空白起始后缀收尾 (原版 FlagHelpers::EqualNameWithSuffix)
/// 蕴含条件名形如 "max_opt < 3" 时按 "max_opt" 命中
pub fn equal_name_with_suffix(a: &str, b: &str) -> bool {
  match b.find(char::is_whitespace) {
    Some(cut) => equal_names(a, &b[..cut]),
    None => equal_names(a, b),
  }
}

/// 归一化 (运行时) 名字查找 (原版 FindFlagByName: 精确匹配, 可查不存在的名)
pub fn find_flag_by_name(name: &str) -> Option<FlagId> {
  defs::NAME_MAP.get(norm_name(name).as_str()).copied()
}

/// 蕴含条件名查找 (原版 FindImplicationFlagByName: 允许 "flag < N" 后缀;
/// 生成表内目标必然存在, 缺失即数据漂移)
pub fn find_implication_flag(name: &str) -> Option<FlagId> {
  let head = match name.find(char::is_whitespace) {
    Some(cut) => &name[..cut],
    None => name,
  };
  defs::NAME_MAP.get(norm_name(head).as_str()).copied()
}

/// 名字归一化 ('_'→'-'); 常见名落栈缓冲, 超长才堆分配
fn norm_name(name: &str) -> NormBuf {
  const STACK_CAP: usize = 128; // 最长 flag 名 < 128 字节 (生成表实测 62)
  if name.len() <= STACK_CAP {
    let mut buf = [0u8; STACK_CAP];
    buf[..name.len()].copy_from_slice(name.as_bytes());
    for b in &mut buf[..name.len()] {
      *b = norm_byte(*b);
    }
    // 归一化后名均为 ASCII flag 名, from_utf8 必然成功
    NormBuf::Stack(buf, name.len())
  } else {
    NormBuf::Heap(name.replace('_', "-"))
  }
}

/// 归一化名字缓冲 (栈优先)
enum NormBuf {
  Stack([u8; 128], usize),
  Heap(String),
}

impl NormBuf {
  #[inline]
  fn as_str(&self) -> &str {
    match self {
      // 归一化仅替换 ASCII 字节, UTF-8 结构不变
      Self::Stack(buf, len) => from_utf8(&buf[..*len]).unwrap_or_default(),
      Self::Heap(s) => s,
    }
  }
}
