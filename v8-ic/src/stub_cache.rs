use strum::{Display, EnumIter, EnumString};

pub const PRIMARY_TABLE_BITS: usize = 12;
pub const PRIMARY_TABLE_SIZE: usize = 1 << PRIMARY_TABLE_BITS; // 4096

pub const SECONDARY_TABLE_BITS: usize = 10;
pub const SECONDARY_TABLE_SIZE: usize = 1 << SECONDARY_TABLE_BITS; // 1024

pub const MAX_IC_INFO: usize = 4096;

/// 缓存表选择枚举 (来源: stub-cache.h:52)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Display, EnumIter, EnumString)]
pub enum CacheTable {
  #[strum(serialize = "kPrimary")]
  Primary,
  #[strum(serialize = "kSecondary")]
  Secondary,
}

/// 接收者/持有者查找结果 (来源: call-optimization.h:57)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Display, EnumIter, EnumString)]
pub enum HolderLookup {
  #[strum(serialize = "kHolderNotFound")]
  NotFound = 0,
  #[strum(serialize = "kHolderIsReceiver")]
  IsReceiver = 1,
  #[strum(serialize = "kHolderFound")]
  Found = 2,
}

/// StubCache 词条 (key, value, map 组合三元组, 对齐 stub-cache.h:34)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct StubCacheEntry {
  pub key: u64,
  pub value: u64,
  pub map: u64,
}

/// 巨态内联缓存表模型 (来源: stub-cache.h:32)
#[derive(Debug)]
pub struct StubCache {
  primary: Box<[StubCacheEntry; PRIMARY_TABLE_SIZE]>,
  secondary: Box<[StubCacheEntry; SECONDARY_TABLE_SIZE]>,
}

impl Default for StubCache {
  fn default() -> Self {
    Self::new()
  }
}

impl StubCache {
  pub fn new() -> Self {
    Self {
      primary: Box::new([StubCacheEntry::default(); PRIMARY_TABLE_SIZE]),
      secondary: Box::new([StubCacheEntry::default(); SECONDARY_TABLE_SIZE]),
    }
  }

  #[inline]
  pub fn primary_offset(name_hash: u32, map_hash: u32) -> usize {
    ((name_hash ^ map_hash) as usize) & (PRIMARY_TABLE_SIZE - 1)
  }

  #[inline]
  pub fn secondary_offset(name_hash: u32, map_hash: u32) -> usize {
    let raw = (name_hash ^ (map_hash.wrapping_mul(17))) as usize;
    raw & (SECONDARY_TABLE_SIZE - 1)
  }

  pub fn get(&self, name_hash: u32, map_hash: u32) -> Option<u64> {
    let p_idx = Self::primary_offset(name_hash, map_hash);
    let p_entry = &self.primary[p_idx];
    if p_entry.key == (name_hash as u64) && p_entry.map == (map_hash as u64) {
      return Some(p_entry.value);
    }

    let s_idx = Self::secondary_offset(name_hash, map_hash);
    let s_entry = &self.secondary[s_idx];
    if s_entry.key == (name_hash as u64) && s_entry.map == (map_hash as u64) {
      return Some(s_entry.value);
    }

    None
  }

  pub fn set(&mut self, name_hash: u32, map_hash: u32, handler: u64) {
    let p_idx = Self::primary_offset(name_hash, map_hash);
    let old_p = self.primary[p_idx];

    // Primary 发生冲突时，将被置换者降级推入 Secondary 表
    if old_p.key != 0 && old_p.map != 0 {
      let s_idx = Self::secondary_offset(old_p.key as u32, old_p.map as u32);
      self.secondary[s_idx] = old_p;
    }

    self.primary[p_idx] = StubCacheEntry {
      key: name_hash as u64,
      value: handler,
      map: map_hash as u64,
    };
  }

  pub fn clear(&mut self) {
    for e in self.primary.iter_mut() {
      *e = StubCacheEntry::default();
    }
    for e in self.secondary.iter_mut() {
      *e = StubCacheEntry::default();
    }
  }
}

/// IC 追踪元数据 (来源: ic-stats.h:31)
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct ICInfo {
  pub type_name: String,
  pub function_name: String,
  pub script_offset: i32,
  pub script_name: String,
  pub line_num: i32,
  pub column_num: i32,
  pub is_constructor: bool,
  pub is_optimized: bool,
  pub state: String,
  pub map_address: u64,
  pub is_dictionary_map: bool,
  pub number_of_own_descriptors: u32,
  pub instance_type: String,
}

/// IC 运行期追踪统计器 (来源: ic-stats.h:53)
#[derive(Debug, Default)]
pub struct ICStats {
  pub enabled: bool,
  pub records: Vec<ICInfo>,
}

impl ICStats {
  pub fn new() -> Self {
    Self::default()
  }

  pub fn begin(&mut self) -> bool {
    self.enabled = true;
    true
  }

  pub fn end(&mut self) {
    self.enabled = false;
  }

  pub fn record(&mut self, info: ICInfo) {
    if self.enabled && self.records.len() < MAX_IC_INFO {
      self.records.push(info);
    }
  }

  pub fn dump(&self) -> String {
    format!("ICStats: recorded {} events", self.records.len())
  }
}

/// 函数调用优化推导结构 (来源: call-optimization.h:17)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct CallOptimization {
  pub constant_function: Option<u64>,
  pub is_simple_api_call: bool,
  pub accept_any_receiver: bool,
}

impl CallOptimization {
  #[inline]
  pub const fn is_constant_call(&self) -> bool {
    self.constant_function.is_some()
  }

  #[inline]
  pub const fn is_simple_api_call(&self) -> bool {
    self.is_simple_api_call
  }

  #[inline]
  pub const fn accept_any_receiver(&self) -> bool {
    self.accept_any_receiver
  }
}
