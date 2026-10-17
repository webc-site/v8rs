//! v8/src/base/region-allocator.h/cc 的转写 — 纯逻辑地址区间分配器
//! 原版 std::set 裸指针双有序集 → BTreeMap(key=region.end) + BTreeSet((size, begin)):
//! - all_regions: 全部区间按地址有序 (原版 AddressEndOrder)
//! - free_regions: 空闲区间按 (size, begin) 有序 (原版 SizeAddressOrder, best-fit)
//!
//! 算法: best-fit + 合并策略 — 分配挑最小合适空闲区, 释放时与邻接空闲区合并
//! 本类型不做任何实际内存预留; 非线程安全 (原版同款约束)
//! 差异登记: 原版 FreeRegion/TrimRegion 的 AddressRegion* 出参 (sandbox 层使用)
//! 未迁移, 如需再加查询接口; AllocateRegion(rng,...) 的 rng 参数以闭包承接

use std::{
  collections::{BTreeMap, BTreeSet},
  ops::Bound,
};

use log::debug;

use crate::{Result, address_region::AddressRegion, error::Error};

/// 空闲率低于全区大小的此比例时停止随机化尝试 (kMaxLoadFactorForRandomization)
const MAX_LOAD_FACTOR_FOR_RANDOMIZATION: f64 = 0.40;

/// 随机地址分配的最大尝试次数 (kMaxRandomizationAttempts)
const MAX_RANDOMIZATION_ATTEMPTS: u32 = 3;

/// 区间状态 (原版 RegionState)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RegionState {
  /// 可分配
  Free,
  /// 已从更大区域切出且不可分配 (kExcluded)
  Excluded,
  /// 已分配并由 RegionAllocator 管理 (kAllocated)
  Allocated,
}

/// 分配策略 (原版 AllocationStrategy)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum AllocationStrategy {
  /// 第一个足够大的空闲区 (默认)
  #[default]
  FirstFit,
  /// 最大的空闲区
  LargestFit,
}

/// 分割/合并回调 (原版 SplitMergeCallback)
pub type SplitMergeCallback = Box<dyn FnMut(usize, usize) + Send>;

/// 内部区间: begin + size + 状态
#[derive(Debug, Clone, Copy)]
struct Region {
  begin: usize,
  size: usize,
  state: RegionState,
}

impl Region {
  const fn end(&self) -> usize {
    self.begin + self.size
  }
  const fn contains_range(&self, address: usize, size: usize) -> bool {
    let offset = address.wrapping_sub(self.begin);
    offset < self.size && offset + size <= self.size
  }
  const fn is_free(&self) -> bool {
    matches!(self.state, RegionState::Free)
  }
}

/// 区间分配器: 管理 [address, address+size) 内的已用/空闲区间
/// 最小分配单位 page_size, 请求大小向上取整到 page_size
/// (回调字段不实现 Debug, 与原版无打印需求一致)
pub struct RegionAllocator {
  /// 整个管理区
  whole_region: AddressRegion,
  /// 全区页数 (size / page_size)
  region_size_in_pages: usize,
  /// 空闲量低于此值时停止随机化
  max_load_for_randomization: usize,
  /// 空闲区间总大小
  free_size: usize,
  /// 最小区间大小 (2 的幂)
  page_size: usize,
  /// 全部区间, key = end (地址有序; 区间互不重叠故 end 唯一)
  all_regions: BTreeMap<usize, Region>,
  /// 空闲区间按 (size, begin) 有序 (best-fit 查找)
  free_regions: BTreeSet<(usize, usize)>,
  /// 区间被切分时回调 [start, start+size)
  on_split: Option<SplitMergeCallback>,
  /// 区间合并时回调 [start, start+size)
  on_merge: Option<SplitMergeCallback>,
}

impl RegionAllocator {
  /// 原版 kAllocationFailure: 分配失败哨兵 (Rust 侧以 Option 表达, 常量供下游对齐)
  pub const ALLOCATION_FAILURE: usize = usize::MAX;

  /// 构造 (原版 CHECK 以 Result 承接)
  pub fn new(address: usize, size: usize, page_size: usize) -> Result<Self> {
    if address >= address.wrapping_add(size) {
      return Err(Error::InvalidArg("region begin 必须小于 end"));
    }
    if !page_size.is_power_of_two() {
      return Err(Error::InvalidArg("page_size 必须是 2 的幂"));
    }
    if !size.is_multiple_of(page_size) || !address.is_multiple_of(page_size) {
      return Err(Error::InvalidArg(
        "region 的 begin/size 必须按 page_size 对齐",
      ));
    }
    let whole_region = AddressRegion::new(address, size);
    let region = Region {
      begin: address,
      size,
      state: RegionState::Free,
    };
    let mut all_regions = BTreeMap::new();
    let mut free_regions = BTreeSet::new();
    free_regions.insert((size, address));
    all_regions.insert(region.end(), region);
    Ok(Self {
      region_size_in_pages: size / page_size,
      max_load_for_randomization: (size as f64 * MAX_LOAD_FACTOR_FOR_RANDOMIZATION) as usize,
      free_size: size,
      whole_region,
      page_size,
      all_regions,
      free_regions,
      on_split: None,
      on_merge: None,
    })
  }

  /// on_split 回调: 已有区间被切分出 [start, start+size) 新区间
  pub fn set_on_split_callback(&mut self, callback: SplitMergeCallback) {
    self.on_split = Some(callback);
  }

  /// on_merge 回调: [start, start+size) 内的全部区间合并为一个
  pub fn set_on_merge_callback(&mut self, callback: SplitMergeCallback) {
    self.on_merge = Some(callback);
  }

  /// 分配 size 大小区间 (须按 page_size 对齐), 成功返回起始地址
  pub fn allocate_region(&mut self, size: usize) -> Option<usize> {
    self.allocate_region_with_strategy(size, AllocationStrategy::FirstFit)
  }

  /// 同上, 指定分配策略
  pub fn allocate_region_with_strategy(
    &mut self,
    size: usize,
    strategy: AllocationStrategy,
  ) -> Option<usize> {
    debug_assert_ne!(size, 0);
    debug_assert_eq!(size % self.page_size, 0);

    let region = match strategy {
      AllocationStrategy::FirstFit => self.free_list_find_region(size),
      AllocationStrategy::LargestFit => self.free_list_find_largest_region(size),
    }?;

    let begin = region.begin;
    let mut end = region.end();
    if region.size != size {
      // 切分后目标区间是前半 (原版 Split 忽略返回值, region 保持前半)
      let (front, _) = self.split(end, size);
      end = front;
    }
    debug_assert_eq!(begin % self.page_size, 0);
    // 原版 region-allocator.cc:170 DCHECK_EQ(region->size(), size)
    debug_assert_eq!(self.all_regions[&end].size, size);

    // 标记为已用 (先移出空闲表再改状态, 保持空闲表断言成立)
    self.free_list_remove_region(end);
    self
      .all_regions
      .get_mut(&end)
      .expect("split 后区间必在")
      .state = RegionState::Allocated;
    Some(begin)
  }

  /// 分配并随机化区间位置 (原版 AllocateRegion(rng, size)):
  /// rng 每次调用产生一个随机 usize (承接 RandomNumberGenerator::NextBytes)
  pub fn allocate_region_randomized(
    &mut self,
    mut rng: impl FnMut() -> usize,
    size: usize,
  ) -> Option<usize> {
    if self.free_size >= self.max_load_for_randomization {
      // 空间足够, 尝试随机化地址
      for _ in 0..MAX_RANDOMIZATION_ATTEMPTS {
        let random = rng();
        let random_offset = self.page_size * (random % self.region_size_in_pages);
        let address = self.whole_region.begin() + random_offset;
        if self.allocate_region_at(address, size, RegionState::Allocated) {
          return Some(address);
        }
      }
      // 回退到空闲表分配
    }
    self.allocate_region(size)
  }

  /// 在 requested_address 处分配 size 区间 (两者须按 page_size 对齐);
  /// 默认标记 Allocated, 也可标记 Excluded 以保留区间不被分配
  pub fn allocate_region_at(
    &mut self,
    requested_address: usize,
    size: usize,
    region_state: RegionState,
  ) -> bool {
    debug_assert_eq!(requested_address % self.page_size, 0);
    debug_assert_ne!(size, 0);
    debug_assert_eq!(size % self.page_size, 0);
    debug_assert_ne!(region_state, RegionState::Free);

    let requested_end = requested_address.wrapping_add(size);
    // 超出总预留或加法溢出则失败
    if requested_end > self.whole_region.end() || requested_end < requested_address {
      return false;
    }

    let Some(mut end) = self.find_region_key(requested_address) else {
      return false;
    };
    let region = self.all_regions[&end];
    if !region.is_free() || region.end() < requested_end {
      return false;
    }
    // 找到覆盖请求区间的空闲区间
    if region.begin != requested_address {
      // 在 requested_address 边界切分, 后半承接请求区间 (原版取 Split 返回值)
      let new_size = requested_address - region.begin;
      let (_, back) = self.split(end, new_size);
      end = back;
    }
    if self.all_regions[&end].end() != requested_end {
      // 在 requested_end 边界切分 (原版 Split 返回值丢弃, region 指针留在前半)
      let (front, _) = self.split(end, size);
      end = front;
    }
    debug_assert_eq!(self.all_regions[&end].begin, requested_address);
    debug_assert_eq!(self.all_regions[&end].size, size);

    // 标记目标状态
    self.free_list_remove_region(end);
    self.all_regions.get_mut(&end).expect("区间必在").state = region_state;
    true
  }

  /// 分配 size 大小、按 alignment 对齐的区间 (size/alignment 须为 page_size 倍数)
  pub fn allocate_aligned_region(&mut self, size: usize, alignment: usize) -> Option<usize> {
    debug_assert_eq!(size % self.page_size, 0);
    debug_assert_eq!(alignment % self.page_size, 0);
    debug_assert!(alignment >= self.page_size);

    let padded_size = size + alignment - self.page_size;
    // best-fit 找 padded_size; 空间不足时退而找 size (仍可能满足对齐)
    let region = self
      .free_list_find_region(padded_size)
      .or_else(|| self.free_list_find_region(size))?;

    let mut end = region.end();
    if region.begin % alignment != 0 {
      let start = region.begin.div_ceil(alignment) * alignment;
      if start + size > region.end() {
        return None;
      }
      let (_, back) = self.split(end, start - region.begin);
      end = back;
      debug_assert_eq!(self.all_regions[&end].begin % alignment, 0);
    }

    if self.all_regions[&end].size != size {
      // 原版 Split(region, size) 后 region 指针留在前半, 取 front key
      let (front, _) = self.split(end, size);
      end = front;
    }
    debug_assert_eq!(self.all_regions[&end].size, size);

    let begin = self.all_regions[&end].begin;
    self.free_list_remove_region(end);
    self.all_regions.get_mut(&end).expect("区间必在").state = RegionState::Allocated;
    Some(begin)
  }

  /// 尝试在 hint 处分配 (按 alignment 对齐), 失败则回退常规分配
  /// (原版 AllocateRegion(hint, size, alignment) 重载)
  pub fn allocate_region_with_hint(
    &mut self,
    hint: usize,
    size: usize,
    alignment: usize,
  ) -> Option<usize> {
    debug_assert_eq!(alignment % self.page_size, 0);
    debug_assert_eq!(hint % alignment, 0);

    if hint != 0
      && self.contains_range(hint, size)
      && self.allocate_region_at(hint, size, RegionState::Allocated)
    {
      return Some(hint);
    }
    if alignment <= self.page_size {
      // 原版 TODO: 此处可考虑随机化版本
      self.allocate_region(size)
    } else {
      self.allocate_aligned_region(size, alignment)
    }
  }

  /// 释放 address 处的区间, 返回被释放区间大小 (无匹配返回 None)
  pub fn free_region(&mut self, address: usize) -> Option<usize> {
    self.trim_region(address, 0)
  }

  /// FreeRegion 的带出参形态 (原版 `AddressRegion* free_region`):
  /// 额外返回与邻接空闲区合并后的完整空闲区间
  pub fn free_region_with_free(&mut self, address: usize) -> Option<(usize, AddressRegion)> {
    self.trim_region_with_free(address, 0)
  }

  /// 缩减 address 处已分配区间到 new_size, 返回释放的大小 (失败返回 None);
  /// new_size 为 0 即完全释放; 须按 page_size 对齐且 <= 当前大小
  pub fn trim_region(&mut self, address: usize, new_size: usize) -> Option<usize> {
    self
      .trim_region_with_free(address, new_size)
      .map(|(n, _)| n)
  }

  /// TrimRegion 的带出参形态 (原版 `AddressRegion* free_region`):
  /// 返回 (释放大小, 合并后的空闲区间)
  pub fn trim_region_with_free(
    &mut self,
    address: usize,
    new_size: usize,
  ) -> Option<(usize, AddressRegion)> {
    debug_assert_eq!(new_size % self.page_size, 0);

    let end = self.find_region_key(address)?;
    if self.all_regions[&end].begin != address
      || self.all_regions[&end].state != RegionState::Allocated
    {
      return None;
    }

    let mut end = end;
    if new_size > 0 {
      let (_, back) = self.split(end, new_size);
      end = back;
    }
    let size = self.all_regions[&end].size;
    self.all_regions.get_mut(&end).expect("区间必在").state = RegionState::Free;

    // 与周围空闲区间合并
    if self.all_regions[&end].end() != self.whole_region.end() {
      // 后继区间必存在
      let next_end = *self.all_regions.range(end..).nth(1).expect("后继必在").0;
      if self.all_regions[&next_end].is_free() {
        // next 将在合并中消失, 先移出空闲表
        self.free_list_remove_region(next_end);
        self.merge(end, next_end);
        // 合并产物以 next_end 为 key
        end = next_end;
      }
    }
    if new_size == 0 && self.all_regions[&end].begin != self.whole_region.begin() {
      // 前驱区间必存在
      let prev_end = *self
        .all_regions
        .range(..end)
        .next_back()
        .expect("前驱必在")
        .0;
      if self.all_regions[&prev_end].is_free() {
        // prev 大小将变化, 需重插空闲表
        self.free_list_remove_region(prev_end);
        // 合并产物以 merged.end() == end 为 key (原版 prev 迭代器节点原地扩容),
        // 故 end 保持不变
        self.merge(prev_end, end);
      }
    }
    self.free_list_add_region(end);
    let region = self.all_regions[&end];
    Some((size, AddressRegion::new(region.begin, region.size)))
  }

  /// 尝试把 address 处区间增长到 new_size
  pub fn try_grow_region(&mut self, address: usize, new_size: usize) -> bool {
    debug_assert_eq!(new_size % self.page_size, 0);

    let Some(end) = self.find_region_key(address) else {
      return false;
    };
    let region = self.all_regions[&end];
    if region.begin != address || region.state != RegionState::Allocated {
      return false;
    }
    debug_assert!(region.size < new_size);

    if !self.allocate_region_at(region.end(), new_size - region.size, RegionState::Allocated) {
      return false;
    }
    // 新分配的后继区间与原区间合并
    let next_end = *self.all_regions.range(end..).nth(1).expect("后继必在").0;
    self.merge(end, next_end);
    true
  }

  /// address 处若有已分配 (含 Excluded) 区间返回其大小, 否则 0
  pub fn check_region(&self, address: usize) -> usize {
    let Some(end) = self.find_region_key(address) else {
      return 0;
    };
    let region = self.all_regions[&end];
    if region.begin != address || region.is_free() {
      return 0;
    }
    region.size
  }

  /// [address, address+size) 是否完全空闲
  pub fn is_free(&self, address: usize, size: usize) -> bool {
    // 原版 CHECK(contains(address, size)): release 亦执行
    assert!(self.contains_range(address, size), "查询区间越界");
    let Some(end) = self.find_region_key(address) else {
      return true;
    };
    let region = self.all_regions[&end];
    region.is_free() && region.contains_range(address, size)
  }

  /// 全区起始地址
  pub const fn begin(&self) -> usize {
    self.whole_region.begin()
  }

  /// 全区结束地址
  pub const fn end(&self) -> usize {
    self.whole_region.end()
  }

  /// 全区大小
  pub const fn size(&self) -> usize {
    self.whole_region.size()
  }

  /// 是否包含 address
  pub const fn contains(&self, address: usize) -> bool {
    self.whole_region.contains_address(address)
  }

  /// 是否包含 [address, address+size)
  pub const fn contains_range(&self, address: usize, size: usize) -> bool {
    self.whole_region.contains_region(address, size)
  }

  /// 尚未分配的总大小
  pub const fn free_size(&self) -> usize {
    self.free_size
  }

  /// 最大空闲区间的大小
  pub fn get_largest_free_region_size(&self) -> usize {
    // 空闲表按 (size, begin) 有序, 末位即最大
    self.free_regions.last().map_or(0, |&(size, _)| size)
  }

  /// 分配地址对齐与大小粒度
  pub const fn page_size(&self) -> usize {
    self.page_size
  }

  /// 全部区间条目数 (原版 FRIEND_TEST(Fragmentation) 的 all_regions_.size())
  #[must_use]
  pub fn all_regions_len(&self) -> usize {
    self.all_regions.len()
  }

  /// 随机化停止阈值 (原版 FRIEND_TEST(AllocateRegionRandom) 的
  /// max_load_for_randomization_)
  #[must_use]
  pub const fn max_load_for_randomization(&self) -> usize {
    self.max_load_for_randomization
  }

  /// 定位包含 address 的区间 (原版 FRIEND_TEST(FindRegion/AllocateExcluded)):
  /// 区间外返回 None, 否则返回 (begin, size, state)
  #[must_use]
  pub fn find_region(&self, address: usize) -> Option<(usize, usize, RegionState)> {
    let end = self.find_region_key(address)?;
    let r = self.all_regions[&end];
    Some((r.begin, r.size, r.state))
  }

  /// 打印区间状态 (原版 Print(std::ostream&), 以 log::debug 输出)
  pub fn print(&self) {
    debug!(
      "RegionAllocator: [{:#x}, {:#x}) size: {:#x} free_size: {:#x} page_size: {:#x}",
      self.begin(),
      self.end(),
      self.size(),
      self.free_size(),
      self.page_size
    );
    debug!("all regions:");
    for region in self.all_regions.values() {
      debug!(
        "  [{:#x}, {:#x}) size: {:#x}, {}",
        region.begin,
        region.end(),
        region.size,
        region_state_to_string(region.state)
      );
    }
    debug!("free regions:");
    for &(size, begin) in &self.free_regions {
      debug!(
        "  [{:#x}, {:#x}) size: {:#x}, free",
        begin,
        begin + size,
        size
      );
    }
  }

  // ---- 内部: 空闲表与区间操作 ----

  /// 定位包含 address 的区间, 返回其 end key (无则 None)
  fn find_region_key(&self, address: usize) -> Option<usize> {
    if !self.whole_region.contains_address(address) {
      return None;
    }
    // 区间按 end 排序; 第一个 end > address 的区间即包含 address
    self
      .all_regions
      .range((Bound::Excluded(address), Bound::Unbounded))
      .next()
      .map(|(end, _)| *end)
  }

  /// 加入空闲表并累计 free_size
  fn free_list_add_region(&mut self, end: usize) {
    let region = self.all_regions[&end];
    self.free_size += region.size;
    self.free_regions.insert((region.size, region.begin));
  }

  /// 移出空闲表并回减 free_size (调用方保证在表内且空闲)
  fn free_list_remove_region(&mut self, end: usize) {
    let region = self.all_regions[&end];
    debug_assert!(region.is_free());
    debug_assert!(region.size <= self.free_size);
    self.free_size -= region.size;
    self.free_regions.remove(&(region.size, region.begin));
  }

  /// best-fit: 最小的 size' >= size 空闲区间
  fn free_list_find_region(&self, size: usize) -> Option<Region> {
    self
      .free_regions
      .range((size, 0)..)
      .next()
      .map(|&(s, b)| Region {
        begin: b,
        size: s,
        state: RegionState::Free,
      })
  }

  /// 最大的 size' >= size 空闲区间
  /// (原版 FreeListFindLargestRegion 按序遍历取严格更大, 同尺寸 tie 保留 begin 最小)
  fn free_list_find_largest_region(&self, size: usize) -> Option<Region> {
    let &(max_size, _) = self.free_regions.range((size, 0)..).next_back()?;
    // 无比 max_size 更大的空闲区, 再取该尺寸下 begin 最小的
    let &(_, begin) = self.free_regions.range((max_size, 0)..).next()?;
    Some(Region {
      begin,
      size: max_size,
      state: RegionState::Free,
    })
  }

  /// 把 end key 处区间切分为 new_size + 余量, 返回 (前半 end key, 后半 end key)
  fn split(&mut self, end: usize, new_size: usize) -> (usize, usize) {
    debug_assert_eq!(new_size % self.page_size, 0);
    debug_assert_ne!(new_size, 0);
    let region = self.all_regions[&end];
    debug_assert!(region.size > new_size);
    debug_assert!(!matches!(region.state, RegionState::Excluded));

    if let Some(cb) = self.on_split.as_mut() {
      cb(region.begin, new_size);
    }

    let state = region.state;
    let new_region = Region {
      begin: region.begin + new_size,
      size: region.size - new_size,
      state,
    };
    if region.is_free() {
      // 更新大小前先移出空闲表
      self.free_list_remove_region(end);
    }
    let mut front = region;
    front.size = new_size;
    // 原 key (end) 移交给后半, 前半换新 key
    self.all_regions.insert(new_region.end(), new_region);
    self.all_regions.insert(front.end(), front);
    if state == RegionState::Free {
      self.free_list_add_region(front.end());
      self.free_list_add_region(new_region.end());
    }
    (front.end(), new_region.end())
  }

  /// 把相邻两区间 (prev 在前, next 在后) 合并为一个, 删除 next
  fn merge(&mut self, prev_end: usize, next_end: usize) {
    let prev = self.all_regions[&prev_end];
    let next = self.all_regions[&next_end];
    debug_assert_eq!(prev.end(), next.begin);

    if let Some(cb) = self.on_merge.as_mut() {
      cb(prev.begin, prev.size + next.size);
    }

    let mut merged = prev;
    merged.size += next.size;
    self.all_regions.remove(&prev_end);
    self.all_regions.remove(&next_end);
    self.all_regions.insert(merged.end(), merged);
    // next 必然不在空闲表 (调用方先行移出)
    debug_assert!(!self.free_regions.contains(&(next.size, next.begin)));
  }
}

/// 区间状态文案 (原版 RegionStateToString)
const fn region_state_to_string(state: RegionState) -> &'static str {
  match state {
    RegionState::Free => "free",
    RegionState::Excluded => "excluded",
    RegionState::Allocated => "used",
  }
}
