// 由 tool/src/heap/base/active-system-pages.js 从 v8/src/heap/base/active-system-pages.h + active-system-pages.cc 生成; 修订就地改, 复跑覆盖
// 堆页上系统页活跃位集 (u64)

//! ActiveSystemPages: 单个堆页内系统页的活跃位图, u64 承载, kMaxPages=64。

/// 原最大可跟踪系统页数 (机械提取验算: 64 = u64 位宽)。
pub const K_MAX_PAGES: usize = 64;

/// 原 ActiveSystemPages (active-system-pages.h:17)。
#[derive(Default)]
pub struct ActiveSystemPages {
  /// 原私有 bitset_t value_; u64 位对齐承载。
  value: u64,
}

/// RoundDown (base/bits.h): 向下对齐。
const fn round_down(value: usize, align: usize) -> usize {
  value & !(align - 1)
}

/// RoundUp (base/bits.h): 向上对齐。
const fn round_up(value: usize, align: usize) -> usize {
  (value + align - 1) & !(align - 1)
}

impl ActiveSystemPages {
  pub fn new() -> Self {
    Self::default()
  }

  /// 原 Init (cc:15-24): 清空后加 header 覆盖页 (user_page_size 仅 DEBUG 校验)。
  pub fn init(
    &mut self,
    header_size: usize,
    page_size_bits: usize,
    user_page_size: usize,
  ) -> usize {
    let page_size = 1usize << page_size_bits;
    debug_assert!(round_up(user_page_size, page_size) >> page_size_bits <= K_MAX_PAGES);
    self.clear();
    self.add(0, header_size, page_size_bits)
  }

  /// 原 Add (cc:26-49): 区间页置位, 返回新增页数。
  /// start_page_bit = RoundDown(start)>>bits; end_page_bit = RoundUp(end)>>bits;
  /// mask = bits==64 ? ~0 : ((1<<bits)-1)<<start_page_bit (原文公式提取)。
  pub fn add(&mut self, start: usize, end: usize, page_size_bits: usize) -> usize {
    let page_size = 1usize << page_size_bits;
    debug_assert!(start <= end);
    debug_assert!(end <= K_MAX_PAGES * page_size);
    // 必须传页大小的 log2, 防止拿整页大小误当位宽。
    debug_assert!(page_size_bits < usize::BITS as usize);

    let start_page_bit = round_down(start, page_size) >> page_size_bits;
    let end_page_bit = round_up(end, page_size) >> page_size_bits;
    debug_assert!(start_page_bit <= end_page_bit);

    let bits = end_page_bit - start_page_bit;
    debug_assert!(bits <= K_MAX_PAGES);
    let mask = if bits == K_MAX_PAGES {
      u64::MAX
    } else {
      ((1u64 << bits) - 1) << start_page_bit
    };
    let added_pages = !self.value & mask;
    self.value |= mask;
    added_pages.count_ones() as usize
  }

  /// 原 Reduce (cc:51-56): 以子集替换, 返回移除页数。
  pub fn reduce(&mut self, updated_value: ActiveSystemPages) -> usize {
    debug_assert!(self.value | updated_value.value == self.value);
    let removed_pages = self.value & !updated_value.value;
    self.value = updated_value.value;
    removed_pages.count_ones() as usize
  }

  /// 原 Clear (cc:58-62): 全清, 返回移除页数。
  pub fn clear(&mut self) -> usize {
    let removed_pages = self.value.count_ones() as usize;
    self.value = 0;
    removed_pages
  }

  /// 原 Size (cc:64-68): 活跃页数 × 页大小。
  pub fn size(&self, page_size_bits: usize) -> usize {
    debug_assert!(page_size_bits < usize::BITS as usize);
    self.value.count_ones() as usize * (1usize << page_size_bits)
  }
}
