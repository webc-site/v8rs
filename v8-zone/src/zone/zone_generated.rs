// 由 tool/src/zone/zone.h.js 从 v8/src/zone/zone.{h,cc} 生成; 修订就地改, 复跑覆盖
// Zone 分配器基线

/// Zone 分配常量 (原版 zone.h)
pub mod zone_const {
  /// New() 返回指针按 8 字节对齐 (ASan/MIPS 均要求; 原版 kAlignmentInBytes)
  pub const ALIGNMENT_IN_BYTES: usize = 8;
  /// 段最小 8KB (原版 kMinimumSegmentSize = 8 * KB)
  pub const MINIMUM_SEGMENT_SIZE: usize = 8192;
  /// 段最大 32KB (原版 kMaximumSegmentSize = 32 * KB)
  pub const MAXIMUM_SEGMENT_SIZE: usize = 32768;
}

/// Zone 协议骨架 (语义见 crate 内手写 zone/core.rs)
pub mod zone_skel {
  /// 当前段空闲区为半开区间 [position, limit) (原版注释)
  pub fn allocate(_size: usize) -> usize {
    unimplemented!("骨架")
  }
  /// 扩容: 当前段不足时开新段 (原版 Expand / ExpandAndAllocate, NOINLINE)
  pub fn expand(_size: usize) {
    unimplemented!("骨架")
  }
  /// 释放段回分配器 (原版 ReleaseSegment)
  pub fn release_segment(_segment: usize) {
    unimplemented!("骨架")
  }
  /// 统计: allocation_size_ / segment_bytes_allocated_ 双原子计数
  pub const ATOMIC_COUNTERS: [&str; 2] = ["allocation_size", "segment_bytes_allocated"];
}

// zone.cc 定义头清单 (对数基准, 8 项):
// void* Zone::AsanNew(size_t size)
// void Zone::Reset()
// bool Zone::Contains(const void* ptr) const
// ZoneSnapshot Zone::Snapshot() const { return ZoneSnapshot{this}; }  void Zone::DeleteAll()
// void Zone::DeleteAll()
// void Zone::ReleaseSegment(Segment* segment)
// void Zone::Expand(size_t size)
// void* Zone::ExpandAndAllocate(size_t size)
