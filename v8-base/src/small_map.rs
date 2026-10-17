//! v8/src/base/small-map.h 的 1:1 转写
//! 小规模内联数组 (kArraySize = N) + 超限退化为通用映射的两级存储。
//! 原版 `union { value_type array_[N]; NormalMap map_; }` + `size_ ==
//! kUsingFullMapSentinel` 哨兵 → Rust 枚举两分支 (借用安全等价)。
//! 登记差异:
//! - NormalMap 模板形参 / MapInit 子类构造钩子 (SubclassInit): gxhash 禁用且
//!   需排序遍历 → 全量段固定 BTreeMap, 不提供构造钩子。
//! - key_equal (comp 双向取反) 对 Ord 键即 ==, 线性段查找用 PartialEq。
//! - pair<iterator,bool> → Result<bool,(K,V)> (已存在时归还实参); 迭代句柄统一
//!   以 Option 引用/下标回传, 退化后裸迭代器失效语义保留 (见 insert 的转换注释)。
//! - N = 0 在原版为 static_assert 拒绝; Rust 侧 N=0 自然即时全量化, 不 panic。
//! - operator[]/at: 原版尾插后返回引用; Rust 兜底分支同样保证全函数无 panic。

use std::{
  collections::{BTreeMap, btree_map::Iter},
  fmt::{self, Debug, Formatter},
  mem::take,
  slice,
};

/// 内联数组容量默认值 (原版 kArraySize = 4)
pub const SMALL_MAP_DEFAULT_INLINE: usize = 4;

enum Storage<K, V, const N: usize> {
  /// array_[kArraySize] 线性段
  Inline { entries: Vec<(K, V)> },
  /// NormalMap map_ 全量段
  Full { map: BTreeMap<K, V> },
}

/// 原版 `SmallMap<NormalMap, kArraySize>`
pub struct SmallMap<K, V, const N: usize = SMALL_MAP_DEFAULT_INLINE> {
  storage: Storage<K, V, N>,
}

impl<K: Ord + Clone, V: Default + Clone, const N: usize> Default for SmallMap<K, V, N> {
  fn default() -> Self {
    Self::new()
  }
}

impl<K: Ord + Clone, V: Default + Clone, const N: usize> Clone for SmallMap<K, V, N> {
  /// 拷贝构造 = InitFrom(src) (深拷贝当前段)
  fn clone(&self) -> Self {
    let storage = match &self.storage {
      Storage::Inline { entries } => Storage::Inline {
        entries: entries.clone(),
      },
      Storage::Full { map } => Storage::Full { map: map.clone() },
    };
    Self { storage }
  }
}

impl<K: Ord + Clone, V: Default + Clone, const N: usize> SmallMap<K, V, N> {
  /// 编译期内联容量 (kArraySize)
  pub const K_ARRAY_SIZE: usize = N;

  /// SmallMap() — 空线性段
  pub fn new() -> Self {
    Self {
      storage: Storage::Inline {
        entries: Vec::with_capacity(N),
      },
    }
  }

  /// bool UsingFullMap() const
  #[inline]
  pub fn using_full_map(&self) -> bool {
    matches!(self.storage, Storage::Full { .. })
  }

  /// size_t size()
  #[inline]
  pub fn size(&self) -> usize {
    match &self.storage {
      Storage::Inline { entries } => entries.len(),
      Storage::Full { map } => map.len(),
    }
  }

  /// bool empty()
  #[inline]
  pub fn is_empty(&self) -> bool {
    self.size() == 0
  }

  /// iterator find(const key_type&) — end() 即 None; 解引用得 value_type&
  /// (原版 value_type* 解引用形态 → 键值双引用回传)
  pub fn find(&self, key: &K) -> Option<(&K, &V)> {
    match &self.storage {
      Storage::Full { map } => map.get_key_value(key),
      Storage::Inline { entries } => entries.iter().find(|(k, _)| k == key).map(|(k, v)| (k, v)),
    }
  }

  /// find 的可变形态 — 仅回传 data_type& (键不可经由 &mut 别名暴露, Rust 借用规则)
  pub fn get_mut(&mut self, key: &K) -> Option<&mut V> {
    match &mut self.storage {
      Storage::Full { map } => map.get_mut(key),
      Storage::Inline { entries } => {
        // 原版自后向前扫描; find 正向扫描, 对无重复键的表两者行为一致
        let idx = entries.iter().rposition(|(k, _)| k == key)?;
        Some(&mut entries[idx].1)
      }
    }
  }

  /// size_t count(const key_type&) — 0/1
  #[inline]
  pub fn count(&self, key: &K) -> usize {
    usize::from(self.find(key).is_some())
  }

  /// data_type& operator[](const key_type&) — 缺失即插入默认值 (原版三段式)
  pub fn at_mut(&mut self, key: &K) -> &mut V {
    // 原版: 线性段满且键缺失 → ConvertToRealMap 后走全量段
    let convert = matches!(&self.storage, Storage::Inline { entries } if entries.len() == N)
      && self.find(key).is_none();
    if convert {
      self.convert_to_real_map();
    }
    match &mut self.storage {
      Storage::Full { map } => map.entry(key.clone()).or_default(),
      Storage::Inline { entries } => match entries.iter().rposition(|(k, _)| k == key) {
        Some(i) => &mut entries[i].1,
        None => {
          let i = entries.len();
          entries.push((key.clone(), V::default()));
          &mut entries[i].1
        }
      },
    }
  }

  /// std::pair<iterator,bool> insert(value_type) —
  /// Ok(true) = 新插入; Err((K,V)) = 键已存在并归还实参 (既有项经 find 访问)。
  /// 原版在满表时 ConvertToRealMap: Invalidates all iterators!
  pub fn insert(&mut self, key: K, value: V) -> Result<bool, (K, V)> {
    if self.find(&key).is_some() {
      return Err((key, value));
    }
    if matches!(&self.storage, Storage::Inline { entries } if entries.len() == N) {
      self.convert_to_real_map();
    }
    match &mut self.storage {
      Storage::Full { map } => {
        map.insert(key, value);
        Ok(true)
      }
      Storage::Inline { entries } => {
        entries.push((key, value));
        Ok(true)
      }
    }
  }

  /// template <InputIterator> void insert(f, l) — 区间逐条插入 (已存在则跳过)
  pub fn insert_range(&mut self, iter: impl IntoIterator<Item = (K, V)>) {
    for (k, v) in iter {
      let _ = self.insert(k, v);
    }
  }

  /// pair<iterator,bool> try_emplace(key, Args...) — 既有项原值保留 (实参析构),
  /// 缺失则插入并回传引用 (原版 emplace/try_emplace 的 Rust 收敛形态)
  pub fn try_emplace(&mut self, key: K, value: V) -> &mut V {
    if self.find(&key).is_none() {
      let _ = self.insert(key.clone(), value);
    }
    match &mut self.storage {
      Storage::Full { map } => map.get_mut(&key).unwrap(),
      Storage::Inline { entries } => {
        let idx = entries.iter().rposition(|(k, _)| k == &key).unwrap();
        &mut entries[idx].1
      }
    }
  }

  /// std::pair<iterator,bool> emplace(args...) — Rust 侧参数即值, 等价 insert
  /// (登记差异: 原版就地构造 vs 移动插入无可观察差异)
  #[inline]
  pub fn emplace(&mut self, key: K, value: V) -> Result<bool, (K, V)> {
    self.insert(key, value)
  }

  /// iterator erase(position) — 线性段按下标删除并 swap-move 末元素入空位,
  /// 返回后继位置 (Some(同下标); None = 原版 end())
  pub fn erase_at(&mut self, index: usize) -> Option<usize> {
    match &mut self.storage {
      Storage::Full { .. } => {
        // 原版 map_.erase(iter) 以后继迭代器回传; Rust 全量段以键删除为准
        debug_assert!(false, "erase_at 仅用于线性段, 全量段请用 erase");
        None
      }
      Storage::Inline { entries } => {
        debug_assert!(index <= entries.len()); // 原版 CHECK_LE(i, size_)
        if index >= entries.len() {
          return None;
        }
        entries.swap_remove(index);
        if index != entries.len() {
          Some(index)
        } else {
          None
        }
      }
    }
  }

  /// size_t erase(const key_type&) — 0/1 (线性段借 erase_at 保持 swap-move 语义)
  pub fn erase(&mut self, key: &K) -> usize {
    match &mut self.storage {
      Storage::Full { map } => usize::from(map.remove(key).is_some()),
      Storage::Inline { entries } => {
        let idx = entries.iter().position(|(k, _)| k == key);
        match idx {
          Some(i) => {
            entries.swap_remove(i);
            1
          }
          None => 0,
        }
      }
    }
  }

  /// void clear() — 原版析构当前段并回退线性段 (size_ = 0)
  pub fn clear(&mut self) {
    self.storage = Storage::Inline {
      entries: Vec::with_capacity(N),
    };
  }

  /// V8_NOINLINE void ConvertToRealMap() — move 全部内联项入全量段
  fn convert_to_real_map(&mut self) {
    let Storage::Inline { entries } = &mut self.storage else {
      return;
    };
    let moved = take(entries);
    let mut map: BTreeMap<K, V> = BTreeMap::new();
    for (k, v) in moved {
      map.insert(k, v);
    }
    self.storage = Storage::Full { map };
  }

  /// begin()/end(): 只读遍历 (线性段插入序, 全量段键序 — 与原版两态一致)
  pub fn iter(&self) -> SmallMapIter<'_, K, V> {
    SmallMapIter {
      inner: match &self.storage {
        Storage::Inline { entries } => IterState::Inline(entries.iter()),
        Storage::Full { map } => IterState::Full(map.iter()),
      },
    }
  }
}

enum IterState<'a, K, V> {
  Inline(slice::Iter<'a, (K, V)>),
  Full(Iter<'a, K, V>),
}

/// 合并两态的 const_iterator (原版 SmallMap::iterator 的只读形态, Clone 支持
/// 后置 ++ 的旧位快照)
pub struct SmallMapIter<'a, K, V> {
  inner: IterState<'a, K, V>,
}

impl<'a, K, V> Clone for SmallMapIter<'a, K, V> {
  fn clone(&self) -> Self {
    Self {
      inner: match &self.inner {
        IterState::Inline(it) => IterState::Inline(it.clone()),
        IterState::Full(it) => IterState::Full(it.clone()),
      },
    }
  }
}

impl<'a, K, V> Iterator for SmallMapIter<'a, K, V> {
  type Item = (&'a K, &'a V);
  fn next(&mut self) -> Option<Self::Item> {
    match &mut self.inner {
      IterState::Inline(it) => it.next().map(|(k, v)| (k, v)),
      IterState::Full(it) => it.next(),
    }
  }
}

impl<K: Debug + Ord + Clone, V: Debug + Default + Clone, const N: usize> Debug
  for SmallMap<K, V, N>
{
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    f.debug_map().entries(self.iter()).finish()
  }
}
