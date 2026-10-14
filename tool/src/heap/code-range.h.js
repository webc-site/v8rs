#!/usr/bin/env bun
// tool/src/heap/code-range.h.js
// 机械提取 v8/src/heap/code-range.{h,cc}: 代码区间 cage。
// 规则: GetWritableReservedAreaSize 公式 / kRadiusInMB 三目 / InitReservation
// 下限钳制 / CodeRangeAddressHint 最近释放表 + RedZones 台账。

import { readV8, writeSrc, banner } from "../../lib.js";

const h = readV8("src/heap/code-range.h");
const cc = readV8("src/heap/code-range.cc");

for (const a of [
  "class V8_EXPORT_PRIVATE RedZones final {",
  "void Initialize(base::BoundedPageAllocator* allocator);",
  "bool TryAdd(base::AddressRegion region);",
  "bool TryRemove(base::AddressRegion needle);",
  "void Reset() {",
  "size_t num_red_zones() const { return red_zones_.size(); }",
  "class CodeRangeAddressHint {",
  "V8_EXPORT_PRIVATE Address GetAddressHint(size_t code_range_size,",
  "V8_EXPORT_PRIVATE void NotifyFreedCodeRange(Address code_range_start,",
  "std::unordered_map<size_t, std::vector<Address>> recently_freed_;",
  "class CodeRange final : public VirtualMemoryCage {",
  "static size_t GetWritableReservedAreaSize();",
  "bool InitReservation(v8::PageAllocator* page_allocator, size_t requested,",
  "V8_EXPORT_PRIVATE void Free();",
  "uint8_t* RemapEmbeddedBuiltins(Isolate* isolate,",
  "static base::AddressRegion GetPreferredRegion(size_t radius_in_megabytes,",
  "std::atomic<uint8_t*> embedded_blob_code_copy_{nullptr};",
  "RedZones red_zones_;",
]) {
  if (!h.includes(a)) throw new Error(`code-range.h 锚点缺失: ${a}`);
}
for (const a of [
  "size_t CodeRange::GetWritableReservedAreaSize() {",
  "return kReservedCodeRangePages * MemoryAllocator::GetCommitPageSize();",
  "if (requested <= kMinimumCodeRangeSize) {",
  "requested = kMinimumCodeRangeSize;",
  "constexpr size_t kRadiusInMB =",
  "kMaxPCRelativeCodeRangeInMB > 1024 ? kMaxPCRelativeCodeRangeInMB : 4096;",
  "auto preferred_region = GetPreferredRegion(kRadiusInMB, kPageSize);",
  "const size_t required_writable_area_size = GetWritableReservedAreaSize();",
  "CHECK_LE(required_writable_area_size, kPageSize);",
  "Address CodeRangeAddressHint::GetAddressHint(",
  "void CodeRangeAddressHint::NotifyFreedCodeRange(",
  "bool CodeRange::InitReservation(",
]) {
  if (!cc.includes(a)) throw new Error(`code-range.cc 锚点缺失: ${a}`);
}
console.log("CodeRange: RedZones 5 + AddressHint 2 + CodeRange 方法 6 + 规则公式 4");

let rust = banner(
  "src/heap/code-range.h.js",
  "src/heap/code-range.{h,cc}",
  "代码区间 cage: RW 保留区/优先域半径/最近释放地址表",
);

rust += `
/// 可写保留区大小 (对应 GetWritableReservedAreaSize =
/// kReservedCodeRangePages * GetCommitPageSize(); Win64 崩溃处理器需要)。
/// v8: src/heap/code-range.cc CodeRange::GetWritableReservedAreaSize
#[must_use]
pub fn writable_reserved_area_size(reserved_code_range_pages: usize, commit_page_size: usize) -> usize {
  reserved_code_range_pages * commit_page_size
}

/// InitReservation 下限钳制 (requested <= kMinimumCodeRangeSize → 取下限)。
/// v8: src/heap/code-range.cc CodeRange::InitReservation
#[must_use]
pub fn clamp_requested_size(requested: usize, minimum_code_range_size: usize) -> usize {
  if requested <= minimum_code_range_size {
    minimum_code_range_size
  } else {
    requested
  }
}

/// 近距代码域搜索半径 (对应 kRadiusInMB = max(kMaxPCRelativeCodeRangeInMB, 4096)
/// 的三目: > 1024 用原值, 否则 4096)。
/// v8: src/heap/code-range.cc CodeRange::InitReservation
#[must_use]
pub fn radius_in_mb(max_pc_relative_code_range_in_mb: usize) -> usize {
  if max_pc_relative_code_range_in_mb > 1024 {
    max_pc_relative_code_range_in_mb
  } else {
    4096
  }
}

/// 最近释放代码区间地址表 (对应 class CodeRangeAddressHint)。
/// size → 最近释放地址数组; 近距关闭时按 size 复用, 否则随机。
/// v8: src/heap/code-range.h CodeRangeAddressHint
#[derive(Debug, Default)]
pub struct CodeRangeAddressHintSkeleton {
  recently_freed: std::collections::HashMap<usize, Vec<u64>>,
}

impl CodeRangeAddressHintSkeleton {
  /// 对应 GetAddressHint: 同 size 有最近释放地址则弹出复用, 否则由调用方随机
  /// v8: src/heap/code-range.cc CodeRangeAddressHint::GetAddressHint
  pub fn get_address_hint(&mut self, code_range_size: usize) -> Option<u64> {
    self.recently_freed.get_mut(&code_range_size).and_then(|v| v.pop())
  }

  /// 对应 NotifyFreedCodeRange: 记入 recently_freed_
  /// v8: src/heap/code-range.cc CodeRangeAddressHint::NotifyFreedCodeRange
  pub fn notify_freed_code_range(&mut self, code_range_start: u64, code_range_size: usize) {
    self.recently_freed.entry(code_range_size).or_default().push(code_range_start);
  }
}

/// 红区集合 (对应 class RedZones final): 借底层 BoundedPageAllocator 管理。
/// v8: src/heap/code-range.h RedZones
#[derive(Debug, Default)]
pub struct RedZonesSkeleton {
  pub red_zones: Vec<(u64, u64)>, // (begin, end)
}

impl RedZonesSkeleton {
  /// 对应 TryAdd: 与管理区相交且未含于现有红区 → 加入
  /// v8: src/heap/code-range.cc RedZones::TryAdd
  pub fn try_add(&mut self, region: (u64, u64), managed: (u64, u64)) -> bool {
    let (rb, re) = region;
    let (mb, me) = managed;
    if re <= mb || rb >= me {
      return false; // 与管理区不相交
    }
    if self.red_zones.iter().any(|(b, e)| rb < *e && *b < re) {
      return false; // 已在红区
    }
    self.red_zones.push(region);
    true
  }

  /// 对应 num_red_zones
  pub fn num_red_zones(&self) -> usize {
    self.red_zones.len()
  }

  /// 对应 Reset: 红区清空, 分配器脱钩 (不动实际内存)
  pub fn reset(&mut self) {
    self.red_zones.clear();
  }
}

// —— 提取不变式断言 ——
const _: () = {
  assert_eq!(writable_reserved_area_size(2, 4096), 8192);
  assert_eq!(clamp_requested_size(1024, 4096), 4096);
  assert_eq!(clamp_requested_size(8192, 4096), 8192);
  assert_eq!(radius_in_mb(128), 4096);
  assert_eq!(radius_in_mb(2048), 2048);
  let mut hint = CodeRangeAddressHintSkeleton::default();
  assert_eq!(hint.get_address_hint(4096), None);
  hint.notify_freed_code_range(0x1000, 4096);
  assert_eq!(hint.get_address_hint(4096), Some(0x1000));
  assert_eq!(hint.get_address_hint(4096), None);
  let mut rz = RedZonesSkeleton::default();
  assert!(rz.try_add((10, 20), (0, 100)));
  assert!(!rz.try_add((15, 25), (0, 100))); // 重叠拒绝
  assert!(!rz.try_add((200, 300), (0, 100))); // 管理区外拒绝
  assert_eq!(rz.num_red_zones(), 1);
  rz.reset();
  assert_eq!(rz.num_red_zones(), 0);
};
`;

const p = writeSrc("v8-heap-core", "code_range.rs.skel", rust);
console.log(`written ${p}`);
