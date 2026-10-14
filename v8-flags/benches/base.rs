// v8-flags 核心评测操作 trait 与各对比实现
// 评测覆盖操作: 按名查找 (命中/未命中)、flag 字符串解析 (--a=b / --no-a /
// 多 flag 批量 / 未知 flag 错误路径)、默认值重置
// 红线 (todo 10.2): flag 语义自研, 不引 clap; 查找基线用 std HashMap 手写路径,
// 对照 v8rs 自身的 phf 静态表路径 (crate 内 NAME_MAP)
// 本文件同时是独立 bench 目标 (cargo 自动发现), 单独编译时部分项未被使用属预期

#[cfg(feature = "bench-hashmap")]
use std::collections::HashMap;

#[cfg(feature = "bench-hashmap")]
use v8_flags::ALL_ENTRIES;
use v8_flags::{FlagId, Flags, find_flag_by_name};

/// 名字归一化 ('_' 视同 '-', 与 lookup::norm_name 语义一致)
#[cfg(feature = "bench-hashmap")]
fn norm_name(name: &str) -> String {
  name.replace('_', "-")
}

/// 评测实现统一接口: 每个对比实现一个 impl, 评测 id 基于 NAME 生成
pub trait FlagBench: Default {
  /// 实现名 (criterion 评测 id 的 function 段)
  const NAME: &'static str;

  /// 按名查找 → 主 flag 下标
  fn lookup(&self, name: &str) -> Option<FlagId>;

  /// 解析 flag 字符串 (空格分隔, 走命令行路径)
  fn parse(&mut self, s: &str) -> v8_flags::Result<()>;

  /// 全量重置为默认值
  fn reset(&mut self);
}

/// v8rs 原生路径: phf 编译期完美散列静态表 (NAME_MAP) + 自研解析
#[derive(Default)]
pub struct V8rsPhf {
  flags: Flags,
}

impl FlagBench for V8rsPhf {
  const NAME: &str = "v8rs-phf";

  #[inline]
  fn lookup(&self, name: &str) -> Option<FlagId> {
    find_flag_by_name(name)
  }

  #[inline]
  fn parse(&mut self, s: &str) -> v8_flags::Result<()> {
    self.flags.set_flags_from_string(s)
  }

  #[inline]
  fn reset(&mut self) {
    self.flags = Flags::new();
  }
}

/// std HashMap 手写动态查找基线 (feature bench-hashmap)
/// 只参与查找对比; 解析/批量/重置无对比库 (不引 clap), 由 V8rsPhf 单实现测量,
/// parse/reset 仅为满足 trait 接口保留
#[cfg(feature = "bench-hashmap")]
pub struct StdHashMap {
  map: HashMap<String, FlagId>,
  flags: Flags,
}

#[cfg(feature = "bench-hashmap")]
impl Default for StdHashMap {
  fn default() -> Self {
    Self::build()
  }
}

#[cfg(feature = "bench-hashmap")]
impl StdHashMap {
  /// 从生成表全条目 (含别名) 构建归一化名 → 主 flag 下标动态表
  pub fn build() -> Self {
    let map = ALL_ENTRIES
      .iter()
      .map(|e| (norm_name(e.name), e.flag_index))
      .collect();
    Self {
      map,
      flags: Flags::new(),
    }
  }
}

#[cfg(feature = "bench-hashmap")]
impl FlagBench for StdHashMap {
  const NAME: &str = "std-hashmap";

  #[inline]
  fn lookup(&self, name: &str) -> Option<FlagId> {
    self.map.get(name).copied()
  }

  #[inline]
  fn parse(&mut self, s: &str) -> v8_flags::Result<()> {
    self.flags.set_flags_from_string(s)
  }

  #[inline]
  fn reset(&mut self) {
    self.flags = Flags::new();
  }
}
