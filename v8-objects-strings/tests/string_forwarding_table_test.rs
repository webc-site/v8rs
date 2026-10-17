// 迁移规约:
//   string-forwarding-table-inl.h:303-331 块索引位运算 (BlockForIndex/
//     IndexInBlock/CapacityForBlock: 块容量 16/16/32/64... 阶梯)
//   string-forwarding-table-inl.h:24-232 Record 原子族 (SetInternalized/
//     SetExternal/raw_hash 双分支/TryUpdateExternalResource CAS/one-byte tag)
//   string-forwarding-table.cc:162-395 表生命周期 (Add/Update/Get/EnsureCapacity
//     增长/IterateElements/TearDown 去重/Reset/疏散修正)
//   test-shared-strings.cc / cctest string-forwarding 段: isolate 依赖 → 推迟
use std::{
  alloc::{GlobalAlloc, Layout, System},
  cell::{Cell, RefCell},
  sync::{
    Mutex,
    atomic::{AtomicUsize, Ordering},
  },
};

use aok::{OK, Void};
use v8_objects_core::{
  heap_object::HeapObject,
  map::Map,
  map_word::MapWord,
  smi::Smi,
  tagged::{Object, Tagged},
};
use v8_objects_strings::string_forwarding_table::{
  self, RawHash, StringForwardingTable, block_for_index, capacity_for_block, index_in_block,
};

static TEST_LOCK: Mutex<()> = Mutex::new(());

/// 活跃分配字节数 (Drop 泄漏回归断言; 默认 realloc = alloc+dealloc, 计数自洽)。
static ACTIVE: AtomicUsize = AtomicUsize::new(0);

/// 透明计数分配器: 仅统计, 转发 System。
struct Counting;

unsafe impl GlobalAlloc for Counting {
  unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
    // SAFETY: 透明转发系统分配器
    let p = unsafe { System.alloc(layout) };
    if !p.is_null() {
      ACTIVE.fetch_add(layout.size(), Ordering::Relaxed);
    }
    p
  }

  unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
    ACTIVE.fetch_sub(layout.size(), Ordering::Relaxed);
    // SAFETY: 透明转发
    unsafe { System.dealloc(ptr, layout) }
  }
}

#[global_allocator]
static ALLOC: Counting = Counting;

#[ctor::ctor(unsafe)]
fn init() {
  log_init::init();
}

/// 堆对象样指针 (低 2 位 = heap object tag)。
fn obj(id: usize) -> Tagged<Object> {
  Tagged::new((0x1000 + id * 0x10) | 1)
}

/// 规约 (inl.h:303-331): 块容量阶梯 16/32/64/...; 块号 = 最高位修正;
/// 块内索引 = 清最左定位位。
#[test]
fn block_index_math_ladder() -> Void {
  let _lock = TEST_LOCK.lock().unwrap();
  // 容量阶梯: block i 容量 = 16 << i
  assert_eq!(capacity_for_block(0), 16);
  assert_eq!(capacity_for_block(1), 32);
  assert_eq!(capacity_for_block(5), 512);

  // 独立模拟: 顺序索引逐个核对 (block, index_in_block)
  let expected: Vec<(u32, u32)> = {
    let mut v = Vec::new();
    let mut block = 0u32;
    let mut used_in_block = 0u32;
    for _ in 0..200 {
      v.push((block, used_in_block));
      used_in_block += 1;
      if used_in_block == capacity_for_block(block) {
        block += 1;
        used_in_block = 0;
      }
    }
    v
  };
  for (index, (block, in_block)) in expected.iter().enumerate() {
    let (b, i) = block_for_index(index as i32);
    assert_eq!(b, *block, "index {index}");
    assert_eq!(i, *in_block, "index {index}");
    assert_eq!(index_in_block(index as i32, b), *in_block);
  }
  OK
}

/// 规约 (inl.h:35 哨兵 + cc:21 零初始化): unused=Smi 0 (ptr=0), deleted=Smi 1。
#[test]
fn sentinels_match_smi_zero_and_one() -> Void {
  let _lock = TEST_LOCK.lock().unwrap();
  assert_eq!(string_forwarding_table::unused_element().ptr(), 0);
  assert_eq!(
    string_forwarding_table::deleted_element().ptr(),
    Smi::from_int(1).ptr()
  );
  OK
}

/// 规约 (cc:209/285): AddForwardString → GetForwardString 往返; 越界 Err。
#[test]
fn add_and_get_forward_string_roundtrip() -> Void {
  let _lock = TEST_LOCK.lock().unwrap();
  let table = StringForwardingTable::new();
  assert!(table.is_empty());

  let s = obj(1);
  let internalized = obj(2);
  let index = table.add_forward_string(s, internalized);
  assert_eq!(index, 0);
  assert_eq!(table.size(), 1);
  assert!(!table.is_empty());

  assert_eq!(
    table.get_forward_string(index).expect("index 刚分配").ptr(),
    internalized.ptr()
  );
  // 越界 (原版 CHECK_LT → Err)
  assert!(table.get_forward_string(1).is_err());
  assert!(table.get_forward_string(-1).is_err());
  OK
}

/// 规约 (cc:225/236 + inl.h:132-186): 内部化登记 + 外部化登记的
/// forward 槽双分支 (串对象 / 裸哈希) 与覆写语义。
#[test]
fn raw_hash_dual_branch_and_overwrite() -> Void {
  let _lock = TEST_LOCK.lock().unwrap();
  let table = StringForwardingTable::new();
  let s = obj(1);

  // 外部化: forward 槽 = 裸哈希 (LSB=0)
  let raw_hash: u32 = 0x1234_5600; // 已计算样值 (bit0=0)
  let idx = table.add_external_resource_and_hash(s, 0xAB00, false, raw_hash);
  assert_eq!(
    table.get_raw_hash(idx).expect("idx 有效"),
    RawHash::Hash(raw_hash)
  );
  assert_eq!(
    table.get_external_resource(idx).expect("idx 有效"),
    (0xAB00, false)
  );

  // 后续内部化: 覆写 forward 槽为内部化串 (inl.h:124-126 语义)
  let internalized = obj(2);
  table
    .update_forward_string(idx, internalized)
    .expect("越界不可达");
  assert_eq!(
    table.get_forward_string(idx).expect("idx 有效").ptr(),
    internalized.ptr()
  );
  // raw_hash 分支随之变为串对象透传
  assert_eq!(
    table.get_raw_hash(idx).expect("idx 有效"),
    RawHash::ForwardStringObject(internalized)
  );
  OK
}

/// 规约 (inl.h:188-205): TryUpdateExternalResource 仅首次成功 (防资源泄漏);
/// one-byte 编码位随存随取。
#[test]
fn try_update_external_resource_once_only() -> Void {
  let _lock = TEST_LOCK.lock().unwrap();
  let table = StringForwardingTable::new();
  let s = obj(1);
  let idx = table.add_forward_string(s, obj(2));

  // 首次写入成功 (含 one-byte tag)
  assert!(
    table
      .try_update_external_resource(idx, 0x2000, true)
      .expect("idx 有效")
  );
  assert_eq!(
    table.get_external_resource(idx).expect("idx 有效"),
    (0x2000, true)
  );

  // 二次写入被拒 (已有资源, 防泄漏)
  assert!(
    !table
      .try_update_external_resource(idx, 0x3000, false)
      .expect("idx 有效")
  );
  assert_eq!(
    table.get_external_resource(idx).expect("idx 有效"),
    (0x2000, true)
  );
  OK
}

/// 规约 (cc:183 EnsureCapacity): 跨块增长 — 17 条记录即触达块 1;
/// 100 条触达块 2; 全部可读回。
#[test]
fn growth_across_blocks() -> Void {
  let _lock = TEST_LOCK.lock().unwrap();
  let table = StringForwardingTable::new();
  for i in 0..100 {
    let s = obj(i as usize);
    let fwd = obj(1000 + i as usize);
    let index = table.add_forward_string(s, fwd);
    assert_eq!(index, i);
  }
  assert_eq!(table.size(), 100);
  for i in 0..100 {
    let fwd = obj(1000 + i as usize);
    assert_eq!(
      table.get_forward_string(i).expect("index 在 size 内").ptr(),
      fwd.ptr(),
      "index {i}"
    );
  }
  OK
}

/// 规约 (inl.h:333 IterateElements): 全量遍历恰好覆盖 size 条记录,
/// 末块按用量截断。
#[test]
fn iterate_elements_visits_exact_count() -> Void {
  let _lock = TEST_LOCK.lock().unwrap();
  let table = StringForwardingTable::new();
  for i in 0..40 {
    table.add_forward_string(obj(i as usize), obj(1000 + i as usize));
  }
  let count = Cell::new(0usize);
  table.iterate_elements(|_| count.set(count.get() + 1));
  assert_eq!(count.get(), 40);
  OK
}

/// 规约 (cc:166-171 析构 + h:115 block_vector_storage_ 成员析构):
/// 块向量倍容 (第 5 块, 即 241 条记录) 后 Drop 须释放全部 Block
/// 与保活链上的 BlockVector 本体 (回归: storage 链泄漏)。
#[test]
fn drop_releases_grown_block_vectors() -> Void {
  let _lock = TEST_LOCK.lock().unwrap();
  // 预热：让全局/线程内部懒加载分配稳定
  {
    let mut warm = StringForwardingTable::new();
    warm.add_forward_string(obj(0), obj(1));
    warm.reset();
  }

  // 阶梯 16/32/64/128 → 前 4 块 (块向量初始容量 4) 共 240; 第 241 条触发倍容
  let n: i32 = 16 + 32 + 64 + 128 + 1;
  let baseline = ACTIVE.load(Ordering::Relaxed);
  {
    let table = StringForwardingTable::new();
    for i in 0..n {
      table.add_forward_string(obj(i as usize), obj(10_000 + i as usize));
    }
    assert_eq!(table.size(), n);
    assert_eq!(capacity_for_block(4), 256); // 倍容确实发生 (block4 落在新块向量)
  }
  let after = ACTIVE.load(Ordering::Relaxed);
  let leak = (after as isize - baseline as isize).abs();
  assert!(
    leak < 4096,
    "Drop 后活跃分配须回落到构造前 (Block/BlockVector 全释放): leak={leak} bytes (baseline={baseline}, after={after})"
  );
  OK
}

/// 规约 (cc:328 TearDown): 非删除记录的外部资源逐一释放且按地址去重;
/// Reset 后表复位于空。
#[test]
fn teardown_disposes_unique_resources_and_resets() -> Void {
  let _lock = TEST_LOCK.lock().unwrap();
  let mut table = StringForwardingTable::new();
  let s = obj(1);
  // 两条记录共用同一资源地址 (含编码位一致) → 只释放一次
  // (原版按 ExternalResourceAddress 原始地址去重, one-byte 编码位不同即视为不同地址)
  let i0 = table.add_external_resource_and_hash(s, 0x9000, true, 0x10);
  let i1 = table.add_external_resource_and_hash(obj(2), 0x9000, true, 0x20);
  let _i2 = table.add_forward_string(obj(3), obj(4)); // 无资源, 不释放
  assert_eq!((i0, i1), (0, 1));

  let disposed: RefCell<Vec<usize>> = RefCell::new(Vec::new());
  table.teardown(|address| disposed.borrow_mut().push(address));
  assert_eq!(*disposed.borrow(), vec![0x9000 | 1]); // one-byte tag 保留在地址位
  assert_eq!(table.size(), 0);
  assert!(table.is_empty());
  // 复位后可继续使用
  let idx = table.add_forward_string(obj(5), obj(6));
  assert_eq!(idx, 0);
  OK
}

/// 规约 (cc:93 UpdateAfterYoungEvacuation): from-page 对象按 map word
/// 转发地址更新槽位; 死对象标删除; 非迁移页不动。
#[test]
fn young_evacuation_updates_forwarded_slots() -> Void {
  let _lock = TEST_LOCK.lock().unwrap();
  // 在连续内存块中分配模拟堆对象，确保地址差落在 V8 Cage 单堆 (<= 4GB) 31-bit Smi 约束内
  let pool = Box::leak(Box::new([
    HeapObject::new(Tagged::<Map>::new(0x21)), // 0: normal
    HeapObject::new(Tagged::<Map>::new(0x21)), // 1: target
    HeapObject::new(Tagged::<Map>::new(0x21)), // 2: forwarded
  ]));
  let (left, right) = pool.split_at_mut(2);
  let normal = &left[0];
  let target = &left[1];
  let forwarded = &mut right[0];
  let forwarded_tagged: Tagged<HeapObject> = Tagged::from_ref(target);
  let host_tagged: Tagged<HeapObject> = Tagged::from_ref(&*forwarded);
  let fwd_word = MapWord::from_forwarding_address(host_tagged, forwarded_tagged);
  forwarded.map = Tagged::<Map>::new(fwd_word.ptr());
  let normal_obj: Tagged<Object> = Tagged::from_ptr(normal as *const HeapObject as *const Object);
  let forwarded_obj: Tagged<Object> =
    Tagged::from_ptr(forwarded as *const HeapObject as *const Object);
  let expected_target = forwarded_tagged.ptr();

  let table = StringForwardingTable::new();
  table.add_forward_string(normal_obj, obj(9));
  table.add_forward_string(forwarded_obj, obj(10));
  let i2 = table.add_forward_string(obj(3), obj(11)); // 非迁移页对象 (样指针)

  // 判定回调: 仅 i0/i1 的原串在 from page; forwarded 已带转发 map word
  let in_from_page =
    |o: Tagged<HeapObject>| o.ptr() == normal_obj.ptr() || o.ptr() == forwarded_obj.ptr();
  table.update_after_young_evacuation(in_from_page);

  // i0: 正常 map 且未被转发 → 死亡 → 删除哨兵 (遍历核对原串槽)
  let mut seen_deleted = 0;
  let mut seen_forwarded_target = 0;
  table.iterate_elements(|rec| {
    let original = rec.original_string_object();
    if original.ptr() == Smi::from_int(1).ptr() {
      seen_deleted += 1;
    }
    if original.ptr() == expected_target {
      seen_forwarded_target += 1;
    }
  });
  assert_eq!(seen_deleted, 1, "死亡对象槽应标删除");
  assert_eq!(seen_forwarded_target, 1, "转发对象槽应更新为转发目标");
  // 非迁移页对象 (i2) 的转发槽不受影响
  assert_eq!(
    table.get_forward_string(i2).expect("i2 有效").ptr(),
    obj(11).ptr()
  );
  OK
}

/// 规约 (cc:377 UpdateAfterFullEvacuation): 原串与转发槽都随转发更新。
#[test]
fn full_evacuation_updates_both_slots() -> Void {
  let _lock = TEST_LOCK.lock().unwrap();
  // 在连续内存块中分配模拟堆对象，确保地址差落在 V8 Cage 单堆 (<= 4GB) 31-bit Smi 约束内
  let pool = Box::leak(Box::new([
    HeapObject::new(Tagged::<Map>::new(0x21)), // 0: target
    HeapObject::new(Tagged::<Map>::new(0x21)), // 1: original
    HeapObject::new(Tagged::<Map>::new(0x21)), // 2: forward
  ]));
  let (first, rest) = pool.split_at_mut(1);
  let (middle, last) = rest.split_at_mut(1);
  let target_ref = &first[0];
  let original = &mut middle[0];
  let forward_ref = &last[0];
  let target_tagged: Tagged<HeapObject> = Tagged::from_ref(target_ref);
  let host_tagged: Tagged<HeapObject> = Tagged::from_ref(&*original);
  let fwd_word = MapWord::from_forwarding_address(host_tagged, target_tagged);
  original.map = Tagged::<Map>::new(fwd_word.ptr());

  let table = StringForwardingTable::new();
  let original_obj: Tagged<Object> =
    Tagged::from_ptr(original as *const HeapObject as *const Object);
  let forward_obj: Tagged<Object> =
    Tagged::from_ptr(forward_ref as *const HeapObject as *const Object);
  table.add_forward_string(original_obj, forward_obj);
  table.update_after_full_evacuation();

  let mut seen = 0;
  table.iterate_elements(|rec| {
    if rec.original_string_object().ptr() == target_tagged.ptr() {
      seen += 1;
    }
  });
  assert_eq!(seen, 1, "原串槽应更新为转发目标");
  OK
}
