// 由 tool/src/codegen/aligned-slot-allocator.h.js 从 v8/src/codegen/aligned-slot-allocator.{h,cc} 生成; 修订就地改, 复跑覆盖
// AlignedSlotAllocator 常量/布局/签名 (算法体机械直译)

/// 对齐槽分配器: 按 1/2/4 槽组分配, 组内首槽按组大小对齐;
/// 复用对齐 padding 片段以节槽。槽 = 指针大小单位。
/// kSlotSize (原版 kSlotSize = kSystemPointerSize, 64 位目标 = 8)
pub const SLOT_SIZE: i32 = 8;

/// 无效槽哨兵 (原版 kInvalidSlot)
pub const INVALID_SLOT: i32 = -1;

/// 原版 AlignedSlotAllocator (字段布局机械对应 next1_/next2_/next4_/size_)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AlignedSlotAllocator {
  /// 1 槽片段起点, 无则 INVALID_SLOT
  pub next1: i32,
  /// 2 对齐 2 槽片段起点, 无则 INVALID_SLOT
  pub next2: i32,
  /// 4 对齐下一 4 槽组起点, 恒有效
  pub next4: i32,
  /// 槽区大小
  pub size: i32,
}

impl Default for AlignedSlotAllocator {
  fn default() -> Self {
    Self {
      next1: INVALID_SLOT,
      next2: INVALID_SLOT,
      next4: 0,
      size: 0,
    }
  }
}

impl AlignedSlotAllocator {
  /// 原版 IsValid (私有静态): slot > kInvalidSlot
  #[inline]
  #[must_use]
  pub const fn is_valid(slot: i32) -> bool {
    slot > INVALID_SLOT
  }

  /// 原版 NumSlotsForWidth: (bytes + kSlotSize - 1) / kSlotSize
  #[inline]
  #[must_use]
  pub const fn num_slots_for_width(bytes: i32) -> i32 {
    (bytes + SLOT_SIZE - 1) / SLOT_SIZE
  }

  /// 原版 NextSlot(int n) const — n 限 1/2/4
  /// 机械直译自 aligned-slot-allocator.cc NextSlot
  #[must_use]
  pub const fn next_slot(&self, n: i32) -> i32 {
    if n <= 1 && Self::is_valid(self.next1) {
      return self.next1;
    }
    if n <= 2 && Self::is_valid(self.next2) {
      return self.next2;
    }
    self.next4
  }

  /// 原版 Allocate(int n) — n 限 1/2/4, 可插 padding; 返回按 n 对齐的起点
  /// 机械直译自 aligned-slot-allocator.cc Allocate (result 初值改写为 match 返回值, 消 unused_assignments)
  pub fn allocate(&mut self, n: i32) -> i32 {
    let result = match n {
      1 => {
        if Self::is_valid(self.next1) {
          let result = self.next1;
          self.next1 = INVALID_SLOT;
          result
        } else if Self::is_valid(self.next2) {
          let result = self.next2;
          self.next1 = result + 1;
          self.next2 = INVALID_SLOT;
          result
        } else {
          let result = self.next4;
          self.next1 = result + 1;
          self.next2 = result + 2;
          self.next4 += 4;
          result
        }
      }
      2 => {
        if Self::is_valid(self.next2) {
          let result = self.next2;
          self.next2 = INVALID_SLOT;
          result
        } else {
          let result = self.next4;
          self.next2 = result + 2;
          self.next4 += 4;
          result
        }
      }
      4 => {
        let result = self.next4;
        self.next4 += 4;
        result
      }
      _ => unreachable!("n 必须为 1/2/4"),
    };
    debug_assert!(Self::is_valid(result), "原版 DCHECK(IsValid(result))");
    self.size = self.size.max(result + n);
    result
  }

  /// 原版 AllocateUnaligned(int n) — 在 size_ 尾部保留 n 槽并复位片段;
  /// n=0 可用于切分槽区 (tagged/untagged 分区)。
  /// 机械直译自 aligned-slot-allocator.cc AllocateUnaligned
  pub fn allocate_unaligned(&mut self, n: i32) -> i32 {
    let result = self.size;
    self.size += n;
    match self.size & 3 {
      0 => {
        self.next1 = INVALID_SLOT;
        self.next2 = INVALID_SLOT;
        self.next4 = self.size;
      }
      1 => {
        self.next1 = self.size;
        self.next2 = self.size + 1;
        self.next4 = self.size + 3;
      }
      2 => {
        self.next1 = INVALID_SLOT;
        self.next2 = self.size;
        self.next4 = self.size + 2;
      }
      _ => {
        self.next1 = self.size;
        self.next2 = INVALID_SLOT;
        self.next4 = self.size + 1;
      }
    }
    result
  }

  /// 原版 Align(int n) — 对齐槽区, 返回 padding 槽数 (n 为 <=4 的 2 的幂)
  /// 机械直译自 aligned-slot-allocator.cc Align
  pub fn align(&mut self, n: i32) -> i32 {
    let mask = n - 1;
    let misalignment = self.size & mask;
    let padding = (n - misalignment) & mask;
    self.allocate_unaligned(padding);
    padding
  }
}

// —— 提取不变式断言 ——
const _: () = {
  assert!(SLOT_SIZE == 8);
  assert!(INVALID_SLOT == -1);
  assert!(AlignedSlotAllocator::num_slots_for_width(8) == 1);
  assert!(AlignedSlotAllocator::num_slots_for_width(9) == 2);
};
