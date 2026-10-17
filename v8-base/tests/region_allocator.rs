//! v8-base/src/region_allocator.rs 集成测试 — 1:1 对应 region-allocator-unittest.cc (16 用例)
//! kAllocationFailure 哨兵 → Option::None; 0 返回值 → None

use std::collections::BTreeSet;

use aok::{OK, Void};
use v8_base::{
  address_region::AddressRegion,
  random_number_generator::RandomNumberGenerator,
  region_allocator::{AllocationStrategy, RegionAllocator, RegionState},
};

#[ctor::ctor(unsafe)]
fn _log_init() {
  log_init::init();
}

const KB: usize = 1024;
const MB: usize = 1024 * 1024;

fn is_aligned(addr: usize, align: usize) -> bool {
  addr.is_multiple_of(align)
}

#[test]
fn region_allocator_test_simple_allocate_region_at() -> Void {
  let page_size = 4 * KB;
  let page_count = 16usize;
  let size = page_size * page_count;
  let begin = page_size * 153;
  let end = begin + size;

  let mut ra = RegionAllocator::new(begin, size, page_size).unwrap();

  // 逐页分配整个区间
  let mut address = begin;
  while address < end {
    assert_eq!(end - address, ra.free_size());
    assert!(ra.allocate_region_at(address, page_size, RegionState::Allocated));
    address += page_size;
  }

  // 无空闲区间, 分配必失败
  assert_eq!(0, ra.free_size());
  assert_eq!(None, ra.allocate_region(page_size));

  // 释放一页后可再分配
  assert_eq!(Some(page_size), ra.free_region(begin));
  assert_eq!(page_size, ra.free_size());
  assert!(ra.allocate_region_at(begin, page_size, RegionState::Allocated));

  // 释放全部页
  address = begin;
  while address < end {
    assert_eq!(Some(page_size), ra.free_region(address));
    address += page_size;
  }

  // 整区空闲, 可整体分配
  assert_eq!(size, ra.free_size());
  assert_eq!(Some(begin), ra.allocate_region(size));
  OK
}

#[test]
fn region_allocator_test_simple_allocate_region() -> Void {
  let page_size = 4 * KB;
  let page_count = 16usize;
  let size = page_size * page_count;
  let begin = page_size * 153;
  let end = begin + size;

  let mut ra = RegionAllocator::new(begin, size, page_size).unwrap();

  for i in 0..page_count {
    assert_eq!(size - page_size * i, ra.free_size());
    let address = ra.allocate_region(page_size);
    assert_eq!(Some(begin + page_size * i), address);
  }

  assert_eq!(0, ra.free_size());
  assert_eq!(None, ra.allocate_region(page_size));

  // 释放后可原地重新分配
  let mut address = begin;
  while address < end {
    assert_eq!(Some(page_size), ra.free_region(address));
    assert_eq!(Some(address), ra.allocate_region(page_size));
    address += page_size;
  }
  assert_eq!(0, ra.free_size());
  OK
}

#[test]
fn region_allocator_test_simple_allocate_aligned_region() -> Void {
  let page_size = 4 * KB;
  let page_count = 16usize;
  let size = page_size * page_count;
  let begin = page_size * 153;

  let mut ra = RegionAllocator::new(begin, size, page_size).unwrap();

  // 不同对齐的分配须真正对齐
  let alignments = [
    page_size,
    page_size * 8,
    page_size,
    page_size * 4,
    page_size * 2,
    page_size * 2,
    page_size * 4,
    page_size * 2,
  ];
  for &alignment in &alignments {
    let address = ra.allocate_aligned_region(page_size, alignment);
    assert!(address.is_some());
    assert!(is_aligned(address.unwrap(), alignment));
  }
  assert_eq!(8 * page_size, ra.free_size());
  OK
}

#[test]
fn region_allocator_test_allocate_region_random() -> Void {
  let page_size = 8 * KB;
  let page_count_log = 16usize;
  let page_count = 1usize << page_count_log;
  let size = page_size * page_count;
  let begin = 153 * MB;
  let end = begin + size;

  // 固定 seed (原版取 GTEST random_seed)
  let mut rng = RandomNumberGenerator::with_seed(42);
  let mut ra = RegionAllocator::new(begin, size, page_size).unwrap();

  let mut allocated_pages = BTreeSet::new();
  // 分配页数达到此值前必须观察到随机化
  let randomization_limit = ra.max_load_for_randomization() / page_size;
  assert!(randomization_limit < page_count);

  let mut last_address = begin;
  let mut saw_randomized_pages = false;

  for i in 0..page_count {
    // 原版 NextBytes(&random, 8): 全宽 u64
    let address = ra
      .allocate_region_randomized(|| rng.next_int64() as u64 as usize, page_size)
      .expect("分配必成功");
    assert!(is_aligned(address, page_size));
    assert!(begin <= address && address < end);
    assert!(!allocated_pages.contains(&address));
    allocated_pages.insert(address);

    saw_randomized_pages |= address < last_address;
    last_address = address;

    if i == randomization_limit {
      // 到此点必须已有随机化证据
      assert!(saw_randomized_pages);
    }
  }

  assert_eq!(0, ra.free_size());
  assert_eq!(None, ra.allocate_region(page_size));
  OK
}

#[test]
fn region_allocator_test_allocate_big_regions() -> Void {
  let page_size = 4 * KB;
  let page_count_log = 10usize;
  let page_count = (1usize << page_count_log) - 1;
  let size = page_size * page_count;
  let begin = page_size * 153;

  let mut ra = RegionAllocator::new(begin, size, page_size).unwrap();

  // |0|1|22|3333|... 递增尺寸分配
  for i in 0..page_count_log {
    let address = ra.allocate_region(page_size * (1usize << i));
    assert_eq!(Some(begin + page_size * ((1usize << i) - 1)), address);
  }

  assert_eq!(0, ra.free_size());
  assert_eq!(None, ra.allocate_region(page_size));

  for i in 0..page_count_log {
    let size = page_size * (1usize << i);
    let address = begin + page_size * ((1usize << i) - 1);
    assert_eq!(Some(size), ra.free_region(address));
    assert_eq!(Some(address), ra.allocate_region(size));
  }
  assert_eq!(0, ra.free_size());
  OK
}

#[test]
fn region_allocator_test_merge_left_to_right_coalecsing_regions() -> Void {
  let page_size = 4 * KB;
  let page_count_log = 10usize;
  let page_count = 1usize << page_count_log;
  let size = page_size * page_count;
  let begin = page_size * 153;

  let mut ra = RegionAllocator::new(begin, size, page_size).unwrap();

  assert_eq!(Some(begin), ra.allocate_region(page_size));
  for i in 0..page_count_log {
    let address = ra.allocate_region(page_size * (1usize << i));
    assert_eq!(Some(begin + page_size * (1usize << i)), address);
  }

  assert_eq!(0, ra.free_size());
  assert_eq!(None, ra.allocate_region(page_size));

  // 释放两个相邻区间后应能分配更大区间
  let mut current_size = page_size;
  for _ in 0..page_count_log {
    assert_eq!(Some(current_size), ra.free_region(begin));
    assert_eq!(Some(current_size), ra.free_region(begin + current_size));
    current_size += current_size;
    assert_eq!(Some(begin), ra.allocate_region(current_size));
  }
  assert_eq!(0, ra.free_size());
  OK
}

#[test]
fn region_allocator_test_merge_right_to_left_coalecsing_regions() -> Void {
  let page_size = 4 * KB;
  let page_count_log = 10usize;
  let page_count = 1usize << page_count_log;
  let size = page_size * page_count;
  let begin = page_size * 153;

  let mut ra = RegionAllocator::new(begin, size, page_size).unwrap();

  for i in 0..page_count {
    assert_eq!(Some(begin + page_size * i), ra.allocate_region(page_size));
  }

  assert_eq!(0, ra.free_size());
  assert_eq!(None, ra.allocate_region(page_size));

  // 偶数页从左到右释放
  let mut i = 0;
  while i < page_count {
    assert_eq!(Some(page_size), ra.free_region(begin + page_size * i));
    i += 2;
  }

  // 奇数页从右到左释放, 每次恰能容纳一个双页
  let mut i = 1;
  while i < page_count {
    let address = begin + page_size * (page_count - i);
    assert_eq!(Some(page_size), ra.free_region(address));
    assert_eq!(Some(address - page_size), ra.allocate_region(page_size * 2));
    assert_eq!(None, ra.allocate_region(page_size * 2));
    i += 2;
  }

  // 释放全部双页
  let mut i = 0;
  while i < page_count {
    assert_eq!(Some(page_size * 2), ra.free_region(begin + page_size * i));
    i += 2;
  }

  assert_eq!(size, ra.free_size());
  assert_eq!(Some(begin), ra.allocate_region(size));
  OK
}

#[test]
fn region_allocator_test_fragmentation() -> Void {
  let page_size = 64 * KB;
  let page_count = 9usize;
  let size = page_size * page_count;
  let begin = page_size * 153;

  let mut ra = RegionAllocator::new(begin, size, page_size).unwrap();

  for i in 0..page_count {
    assert_eq!(Some(begin + page_size * i), ra.allocate_region(page_size));
  }

  assert_eq!(0, ra.free_size());
  assert_eq!(None, ra.allocate_region(page_size));

  // 按序释放并核对区间条目数 (合并效果)
  let testcase = [
    (0usize, 9usize), // x........
    (2, 9),           // x.x......
    (4, 9),           // x.x.x....
    (6, 9),           // x.x.x.x..
    (8, 9),           // x.x.x.x.x
    (1, 7),           // xxx.x.x.x
    (7, 5),           // xxx.x.xxx
    (3, 3),           // xxxxx.xxx
    (5, 1),           // xxxxxxxxx
  ];
  assert_eq!(page_count, testcase.len());

  assert_eq!(page_count, ra.all_regions_len());
  for &(page_index_to_free, expected_page_count) in &testcase {
    let address = begin + page_size * page_index_to_free;
    assert_eq!(Some(page_size), ra.free_region(address));
    assert_eq!(expected_page_count, ra.all_regions_len());
  }

  assert_eq!(size, ra.free_size());
  assert_eq!(Some(begin), ra.allocate_region(size));
  OK
}

#[test]
fn region_allocator_test_find_region() -> Void {
  let page_size = 4 * KB;
  let page_count = 16usize;
  let size = page_size * page_count;
  let begin = page_size * 153;
  let end = begin + size;

  let mut ra = RegionAllocator::new(begin, size, page_size).unwrap();

  let mut address = begin;
  while address < end {
    assert_eq!(end - address, ra.free_size());
    assert!(ra.allocate_region_at(address, page_size, RegionState::Allocated));
    address += page_size;
  }

  assert_eq!(0, ra.free_size());
  assert_eq!(None, ra.allocate_region(page_size));

  // 区间外请求返回 end 迭代器 → None
  assert_eq!(None, ra.find_region(begin - 1));
  assert_eq!(None, ra.find_region(begin - page_size));
  assert_eq!(None, ra.find_region(begin / 2));
  assert_eq!(None, ra.find_region(end));
  assert_eq!(None, ra.find_region(end + page_size));
  assert_eq!(None, ra.find_region(end * 2));

  let mut address = begin;
  while address < end {
    let Some((region_start, _region_size, _)) = ra.find_region(address) else {
      panic!("区间内地址必能定位: {address:#x}");
    };
    let expected_start = address - address % page_size; // RoundDown
    assert_eq!(expected_start, region_start);
    assert!(region_start <= address);
    assert!(address < region_start + page_size);
    address += page_size / 4;
  }
  OK
}

#[test]
fn region_allocator_test_trim_region() -> Void {
  let page_size = 4 * KB;
  let page_count = 64usize;
  let size = page_size * page_count;
  let begin = page_size * 153;

  let mut ra = RegionAllocator::new(begin, size, page_size).unwrap();

  let address = begin + 13 * page_size;
  let mut size = 37 * page_size;
  let mut free_size = size_of_total(page_count) - size;
  assert!(ra.allocate_region_at(address, size, RegionState::Allocated));

  let mut trim_size = page_size;
  loop {
    assert_eq!(size, ra.check_region(address));
    assert_eq!(free_size, ra.free_size());

    trim_size = trim_size.min(size);
    size -= trim_size;
    free_size += trim_size;
    assert_eq!(Some(trim_size), ra.trim_region(address, size));
    trim_size *= 2;
    if size == 0 {
      break;
    }
  }

  assert_eq!(size_of_total(page_count), ra.free_size());
  assert_eq!(Some(begin), ra.allocate_region(size_of_total(page_count)));
  OK
}

/// 整区大小 (kSize)
const fn size_of_total(page_count: usize) -> usize {
  4 * KB * page_count
}

#[test]
fn region_allocator_test_free_region_reports_merged_free_region() -> Void {
  let page_size = 4 * KB;
  let page_count = 8usize;
  let size = page_size * page_count;
  let begin = page_size * 153;

  let mut ra = RegionAllocator::new(begin, size, page_size).unwrap();

  for i in 0..page_count {
    assert_eq!(Some(begin + page_size * i), ra.allocate_region(page_size));
  }

  // 相邻已分配之间的释放页自成一个空闲区间
  let (freed, free_region) = ra.free_region_with_free(begin + 3 * page_size).unwrap();
  assert_eq!(page_size, freed);
  assert_eq!(
    AddressRegion::new(begin + 3 * page_size, page_size),
    free_region
  );

  // 释放邻接页合并 (任意一侧)
  let (freed, free_region) = ra.free_region_with_free(begin + 4 * page_size).unwrap();
  assert_eq!(page_size, freed);
  assert_eq!(
    AddressRegion::new(begin + 3 * page_size, 2 * page_size),
    free_region
  );
  let (freed, free_region) = ra.free_region_with_free(begin + 2 * page_size).unwrap();
  assert_eq!(page_size, freed);
  assert_eq!(
    AddressRegion::new(begin + 2 * page_size, 3 * page_size),
    free_region
  );

  // 释放两个空闲区之间的页: 三区合一
  let (freed, free_region) = ra.free_region_with_free(begin + 6 * page_size).unwrap();
  assert_eq!(page_size, freed);
  assert_eq!(
    AddressRegion::new(begin + 6 * page_size, page_size),
    free_region
  );
  let (freed, free_region) = ra.free_region_with_free(begin + 5 * page_size).unwrap();
  assert_eq!(page_size, freed);
  assert_eq!(
    AddressRegion::new(begin + 2 * page_size, 5 * page_size),
    free_region
  );

  // 无可释放: 返回 0 (原版出参保持原值 → None 语义)
  assert_eq!(None, ra.free_region(begin + 5 * page_size));
  OK
}

#[test]
fn region_allocator_test_try_grow_region() -> Void {
  let page_size = 4 * KB;
  let page_count = 60usize;
  let size = page_size * page_count;
  let begin = page_size * 153;

  let mut ra = RegionAllocator::new(begin, size, page_size).unwrap();

  // 在区间中部分配主区域
  let address = begin + 10 * page_size;
  let size = 10 * page_size;
  assert!(ra.allocate_region_at(address, size, RegionState::Allocated));

  // 增长到距末尾一页
  assert!(ra.try_grow_region(address, 49 * page_size));
  assert_eq!(11 * page_size, ra.free_size());

  // 在主区域前后分配
  assert_eq!(Some(begin), ra.allocate_region(10 * page_size));
  assert_eq!(Some(begin + 59 * page_size), ra.allocate_region(page_size));

  // 重新释放周围区域
  assert_eq!(Some(10 * page_size), ra.free_region(begin));
  assert_eq!(10 * page_size, ra.free_size());

  assert_eq!(Some(page_size), ra.free_region(begin + 59 * page_size));
  assert_eq!(11 * page_size, ra.free_size());

  // 越界增长必失败
  assert!(!ra.try_grow_region(address, 51 * page_size));
  assert_eq!(Some(49 * page_size), ra.free_region(address));
  assert_eq!(60 * page_size, ra.free_size());
  OK
}

#[test]
fn region_allocator_test_try_grow_region_limited_by_other_region() -> Void {
  let page_size = 4 * KB;
  let page_count = 60usize;
  let size = page_size * page_count;
  let begin = page_size * 153;

  let mut ra = RegionAllocator::new(begin, size, page_size).unwrap();

  let address = begin + 10 * page_size;
  let size = 10 * page_size;
  assert!(ra.allocate_region_at(address, size, RegionState::Allocated));

  // 布置增长屏障: 主区域最多 30 页
  assert!(ra.allocate_region_at(
    begin + 40 * page_size,
    10 * page_size,
    RegionState::Allocated
  ));

  // 增长到 31 页必失败
  assert!(!ra.try_grow_region(address, 31 * page_size));

  // 增长到 30 页成功
  assert!(ra.try_grow_region(address, 30 * page_size));
  assert_eq!(Some(30 * page_size), ra.free_region(address));
  assert_eq!(50 * page_size, ra.free_size());
  OK
}

#[test]
fn region_allocator_test_allocat_region_with_growing_hint() -> Void {
  let page_size = 4 * KB;
  let page_count = 80usize;
  let size = page_size * page_count;
  let begin = page_size * 153;

  let mut ra = RegionAllocator::new(begin, size, page_size).unwrap();

  // 10 free | 10 allocated | 20 free | 10 allocated | 30 free
  assert!(ra.allocate_region_at(
    begin + 10 * page_size,
    10 * page_size,
    RegionState::Allocated
  ));
  assert!(ra.allocate_region_at(
    begin + 40 * page_size,
    10 * page_size,
    RegionState::Allocated
  ));

  // 常规分配落在第一个足够大的空闲区
  assert_eq!(Some(begin), ra.allocate_region(10 * page_size));
  assert_eq!(Some(10 * page_size), ra.free_region(begin));

  // LargestFit 策略落在最大空闲区
  assert_eq!(
    Some(begin + 50 * page_size),
    ra.allocate_region_with_strategy(10 * page_size, AllocationStrategy::LargestFit)
  );
  assert!(ra.try_grow_region(begin + 50 * page_size, 30 * page_size));
  assert!(!ra.try_grow_region(begin + 50 * page_size, 31 * page_size));
  OK
}

#[test]
fn region_allocator_test_allocate_excluded() -> Void {
  let page_size = 4 * KB;
  let page_count = 64usize;
  let size = page_size * page_count;
  let begin = page_size * 153;

  let mut ra = RegionAllocator::new(begin, size, page_size).unwrap();

  let address = begin + 13 * page_size;
  let size = 37 * page_size;
  assert!(ra.allocate_region_at(address, size, RegionState::Excluded));

  // Excluded 区不可分配也不空闲
  assert!(!ra.is_free(address, size));
  assert!(!ra.allocate_region_at(address, size, RegionState::Allocated));
  let (region_begin, _region_size, state) = ra.find_region(address).unwrap();
  assert_eq!(address, region_begin);
  assert_eq!(RegionState::Excluded, state);

  // Excluded 区不能释放也不能裁剪
  assert_eq!(None, ra.free_region(address));
  assert_eq!(None, ra.trim_region(address, page_size));
  OK
}

#[test]
fn region_allocator_test_get_largest_free_region_size() -> Void {
  let page_size = 4 * KB;
  let page_count = 10usize;
  let size = page_size * page_count;
  let begin = 0;

  let mut ra = RegionAllocator::new(begin, size, page_size).unwrap();
  assert_eq!(10 * page_size, ra.get_largest_free_region_size());

  assert!(ra.allocate_region_at(8 * page_size, page_size, RegionState::Allocated));
  assert_eq!(8 * page_size, ra.get_largest_free_region_size());

  assert!(ra.allocate_region_at(page_size, page_size, RegionState::Allocated));
  assert_eq!(6 * page_size, ra.get_largest_free_region_size());

  assert!(ra.allocate_region_at(5 * page_size, page_size, RegionState::Allocated));
  assert_eq!(3 * page_size, ra.get_largest_free_region_size());

  assert_eq!(Some(page_size), ra.free_region(5 * page_size));
  assert_eq!(6 * page_size, ra.get_largest_free_region_size());
  OK
}
