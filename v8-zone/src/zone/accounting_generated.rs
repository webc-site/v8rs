// 由 tool/src/zone/accounting-allocator.h.js 从 v8/src/zone/accounting-allocator.{h,cc} 生成; 修订就地改, 复跑覆盖
// 段记账分配器骨架

/// AccountingAllocator 签名骨架 (语义见 crate 内手写 zone/accounting.rs)
pub mod accounting_allocator_skel {
  /// 分配段 (原版 AllocateSegment; 尺寸夹在 [kMinimumSegmentSize, kMaximumSegmentSize])
  pub fn allocate_segment(_bytes: usize) -> usize {
    unimplemented!("骨架")
  }
  /// 回收段 (原版 ReturnSegment; DEBUG 下先 zap)
  pub fn return_segment(_segment: usize) {
    unimplemented!("骨架")
  }
  /// 追踪虚面 (原版 Trace* 三件套, 默认空实现, 精确统计构建覆写)
  pub const TRACE_IMPLS: [&str; 3] = ["TraceZoneCreationImpl", "TraceZoneDestructionImpl", "TraceAllocateSegmentImpl"];
  /// 平台分支登记: 32 位宿主缩小保留区间; VirtualMemory 保留 + 重试
  pub const PLATFORM_BRANCHES: [&str; 3] = ["32bit-reservation", "VirtualMemory", "retry"];
}

// accounting-allocator.cc 定义头清单 (对数基准, 4 项):
// AccountingAllocator::AccountingAllocator() : AccountingAllocator(nullptr)
// AccountingAllocator::AccountingAllocator(Isolate* isolate) : isolate_(isolate)
// Segment* AccountingAllocator::AllocateSegment(size_t requested_bytes)
// void AccountingAllocator::ReturnSegment(Segment* segment)
