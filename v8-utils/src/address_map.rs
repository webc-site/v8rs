// 由 tool/src/utils/consts.js 从 v8/src/utils/address-map.h;src/utils/address-map.cc 生成; 修订就地改, 复跑覆盖

//! 地址 → 索引映射, 移植自 v8/src/utils/address-map。
//!
//! 上游 PointerToIndexHashMap 基于 base::TemplateHashMapImpl (开放寻址),
//! 并以 HeapObject 特化 + RootIndexMap 挂接对象系统;
//! Rust 版就地进行线性探测实现, 对象系统特化见 README 映射表。

// 哈希与扩容策略对齐 base::TemplateHashMapImpl (kDefaultHashMapCapacity=8,
// FillEmptyEntry 插入后 80% 阈值倍容); 哈希为上游低 32 位截断恒等。

/// 空槽标记 (TemplateHashMapImpl 的 kEmpty)
const EMPTY: u64 = u64::MAX;
/// 已删槽标记 (kRemoved)
const REMOVED: u64 = u64::MAX - 1;
/// 初始容量 (base::hashmap kDefaultHashMapCapacity)
const INITIAL_CAPACITY: usize = 8;

/// 指针/地址 → u32 索引映射 (PointerToIndexHashMap)
///
/// 键恒非 EMPTY/REMOVED 哨兵 (真实地址不可能取到两个最大值)。
#[derive(Debug, Clone, Default)]
pub struct PointerToIndexHashMap {
  keys: Vec<u64>,
  values: Vec<u32>,
  size: usize,
}

impl PointerToIndexHashMap {
  pub fn new() -> Self {
    Self::default()
  }

  pub fn len(&self) -> usize {
    self.size
  }

  pub fn is_empty(&self) -> bool {
    self.size == 0
  }

  /// Set(value, index): 覆盖或插入
  ///
  /// 上游语义: 插入后负载 >= 80% 时倍容 (hashmap.h FillEmptyEntry)。
  pub fn set(&mut self, value: u64, index: u32) {
    if self.keys.is_empty() {
      self.rehash(INITIAL_CAPACITY);
    }
    let h = Self::hash(value);
    let mask = self.keys.len() - 1;
    let mut i = h as usize & mask;
    loop {
      match self.keys[i] {
        // 空槽: 计数后落位
        EMPTY => {
          self.size += 1;
          self.keys[i] = value;
          self.values[i] = index;
          break;
        }
        // 复用已删槽: 占位数不变 (上游 occupancy_ 口径)
        REMOVED => {
          self.keys[i] = value;
          self.values[i] = index;
          break;
        }
        k if k == value => {
          self.values[i] = index;
          return;
        }
        _ => i = (i + 1) & mask,
      }
    }
    // 上游 FillEmptyEntry: 插入后检查 occupancy + occupancy/4 >= capacity
    if self.size + self.size / 4 >= self.keys.len() {
      self.rehash(self.keys.len() * 2);
    }
  }

  /// Get(value) -> Option<index>
  pub fn get(&self, value: u64) -> Option<u32> {
    if self.keys.is_empty() {
      return None;
    }
    let h = Self::hash(value);
    let mask = self.keys.len() - 1;
    let mut i = h as usize & mask;
    loop {
      match self.keys[i] {
        EMPTY => return None,
        k if k == value => return Some(self.values[i]),
        _ => i = (i + 1) & mask,
      }
    }
  }

  /// Hash: 上游 static_cast<uint32_t>(key) 低 32 位截断 (address-map.h:58)
  #[inline]
  fn hash(key: u64) -> u32 {
    key as u32
  }

  fn rehash(&mut self, new_len: usize) {
    let mut keys = vec![EMPTY; new_len];
    let mut values = vec![0u32; new_len];
    let mask = new_len - 1;
    'outer: for (&k, &v) in self.keys.iter().zip(&self.values) {
      if k == EMPTY || k == REMOVED {
        continue;
      }
      let mut i = Self::hash(k) as usize & mask;
      loop {
        if keys[i] == EMPTY {
          keys[i] = k;
          values[i] = v;
          continue 'outer;
        }
        i = (i + 1) & mask;
      }
    }
    self.keys = keys;
    self.values = values;
  }
}

/// AddressToIndexHashMap (Address 特化别名)
pub type AddressToIndexHashMap = PointerToIndexHashMap;
