// 迁移规约:
//   number-string-cache-inl.h:28/116 GetEntryFor 哈希公式
//     (Smi: value & mask; Double: lo ^ hi & mask)
//   number-string-cache-inl.h:41-96/135-188 Get/Set 命中与覆写 +
//     full-size 切换段 (isolate/heap::SetXxxCache → 参数化)
//   number-string-cache.cc:12-47 GetUsedEntriesCount
//   number-string-cache.h:27-160 常量与布局 (kInitialSize/kMaxCapacity/
//     SizeFor/OffsetOfElementAt + static_assert)
//   unittests 侧无独立用例文件 (随 heap/factory 测试) → 规约自原版直迁
use aok::{OK, Void};
use v8_objects_core::{
  smi::Smi,
  tagged::{Object, Tagged},
};
use v8_objects_strings::number_string_cache::{
  DOUBLE_INITIAL_SIZE, DOUBLE_MAX_CAPACITY, DoubleStringCache, INITIAL_SIZE, SmiStringCache,
  double_offset_of_element_at, double_size_for,
};

#[ctor::ctor(unsafe)]
fn init() {
  log_init::init();
}

/// 堆对象样值 (非 Smi)。
fn str_obj(id: usize) -> Tagged<Object> {
  Tagged::new(0x1_0001 + id * 0x10)
}

/// 规约 (inl.h:28): Smi 定位 = 数值位型 & (capacity-1); 负数走位型。
#[test]
fn smi_entry_location_formula() -> Void {
  let cache = SmiStringCache::new(128);
  assert_eq!(cache.capacity(), 128);
  assert_eq!(cache.get_entry_for(Smi::from_int(5)), 5);
  assert_eq!(cache.get_entry_for(Smi::from_int(128)), 0);
  assert_eq!(cache.get_entry_for(Smi::from_int(129)), 1);
  // 负数: static_cast<uint32_t>(-1) = 0xFFFFFFFF → & 127 = 127
  assert_eq!(cache.get_entry_for(Smi::from_int(-1)), 127);
  OK
}

/// 规约 (inl.h:41-64 Get + 67-96 Set): 键命中返回值; 冲突覆写;
/// 未命中键返回 None (原版 undefined)。
#[test]
fn smi_get_set_roundtrip_and_overwrite() -> Void {
  let mut cache = SmiStringCache::new(INITIAL_SIZE);
  assert_eq!(cache.get_used_entries_count(), 0);

  let number = Smi::from_int(42);
  let s = str_obj(1);
  let entry = cache.get_entry_for(number);
  assert_eq!(cache.get(entry, number), None, "初始未命中");

  cache.set(entry, number, s);
  assert_eq!(cache.get(entry, number), Some(s));
  assert_eq!(cache.get_used_entries_count(), 1);

  // 同槽异键: 覆写 (原版 Set 语义: potentially overwriting)
  let other = Smi::from_int(42 + 128); // 同哈希槽
  assert_eq!(cache.get_entry_for(other), entry);
  cache.set(entry, other, str_obj(2));
  assert_eq!(cache.get(entry, number), None, "旧键被覆写");
  assert_eq!(cache.get(entry, other), Some(str_obj(2)));
  OK
}

/// 规约 (inl.h:78-92): 初始容量 + 槽已占 + full_size → 重建全量表
/// (不复制旧条目, 原版 TODO 同款), 条目重算; None (MemorySaver) 直写。
#[test]
fn smi_full_size_switch_rebuilds_without_copy() -> Void {
  let mut cache = SmiStringCache::new(INITIAL_SIZE);
  let n1 = Smi::from_int(7);
  let e1 = cache.get_entry_for(n1);
  cache.set(e1, n1, str_obj(1));

  // 冲突触发重建: 旧条目不迁移
  let n2 = Smi::from_int(7 + 128);
  let e2 = cache.get_entry_for(n2);
  assert_eq!(e1, e2, "同槽冲突前提");
  let used_entry = cache.set_with_growth(e2, n2, str_obj(2), Some(256));
  assert_eq!(cache.capacity(), 256);
  assert_eq!(cache.get_used_entries_count(), 1, "重建不复制旧条目");
  assert_eq!(cache.get(used_entry, n2), Some(str_obj(2)));
  assert_eq!(
    cache.get(cache.get_entry_for(n1), n1),
    None,
    "旧键已随旧表丢弃"
  );

  // MemorySaver 模式 (None): 不重建, 直接覆写
  let n3 = Smi::from_int(9);
  let e3 = cache.get_entry_for(n3);
  let used3 = cache.set_with_growth(e3, n3, str_obj(3), None);
  assert_eq!(cache.capacity(), 256);
  assert_eq!(used3, e3);
  OK
}

/// 规约 (number-string-cache.cc:12 GetUsedEntriesCount + Clear)。
#[test]
fn smi_used_count_and_clear() -> Void {
  let mut cache = SmiStringCache::new(128);
  for i in 1..=10 {
    let n = Smi::from_int(i);
    cache.set(cache.get_entry_for(n), n, str_obj(i as usize));
  }
  assert_eq!(cache.get_used_entries_count(), 10);
  cache.clear();
  assert_eq!(cache.get_used_entries_count(), 0);
  OK
}

/// 规约 (inl.h:116): Double 定位 = (lo ^ hi) & (capacity-1)。
#[test]
fn double_entry_location_formula() -> Void {
  assert_eq!(
    DoubleStringCache::entry_for_bits(0x0000_0001_0000_0000, 128),
    1
  );
  assert_eq!(
    DoubleStringCache::entry_for_bits(0x0000_0001_0000_0003, 128),
    2
  );
  assert_eq!(
    DoubleStringCache::entry_for_bits(u64::from(u32::MAX) << 32 | 1, 128),
    126
  );
  // 实例透传
  let cache = DoubleStringCache::new(DOUBLE_INITIAL_SIZE);
  assert_eq!(cache.get_entry_for(0x5), 5);
  OK
}

/// 规约 (inl.h:135-188): 位型键命中/覆写 + full-size 切换重建。
#[test]
fn double_get_set_and_full_size_switch() -> Void {
  let mut cache = DoubleStringCache::new(DOUBLE_INITIAL_SIZE);
  let bits: u64 = 0x4009_21FB_5444_2D18; // π 位型
  let s = str_obj(1);
  let entry = cache.get_entry_for(bits);
  assert_eq!(cache.get(entry, bits), None);
  cache.set(entry, bits, s);
  assert_eq!(cache.get(entry, bits), Some(s));
  assert_eq!(cache.get_used_entries_count(), 1);

  // 冲突覆写 (同槽异键: 构造 lo^hi 相同)
  let bits2 = bits ^ 0x0000_0001_0000_0001; // hi^lo 不变? hi+1,lo-1 → xor 不变
  assert_eq!(cache.get_entry_for(bits2), entry);
  cache.set(entry, bits2, str_obj(2));
  assert_eq!(cache.get(entry, bits), None);
  assert_eq!(cache.get(entry, bits2), Some(str_obj(2)));

  // full-size 切换: 重建不复制
  let bits3 = bits ^ 0x0000_0002_0000_0002;
  let entry3 = cache.get_entry_for(bits3);
  let used = cache.set_with_growth(entry3, bits3, str_obj(3), Some(DOUBLE_INITIAL_SIZE * 2));
  assert_eq!(cache.capacity(), DOUBLE_INITIAL_SIZE * 2);
  assert_eq!(cache.get_used_entries_count(), 1, "重建不复制旧条目");
  assert_eq!(cache.get(used, bits3), Some(str_obj(3)));
  OK
}

/// 规约 (h:158 static_assert + SizeFor/OffsetOfElementAt 布局公式)。
#[test]
fn double_layout_constants() -> Void {
  assert_eq!(INITIAL_SIZE, 128);
  assert_eq!(DOUBLE_INITIAL_SIZE, 128);
  assert_eq!(DOUBLE_MAX_CAPACITY, 0x2000, "kTaggedSize=4 配置取 0x2000");
  // Offset = header(kTaggedSize) + capacity 字段(u32) + index * 12
  assert_eq!(double_offset_of_element_at(0), 8);
  assert_eq!(double_offset_of_element_at(1), 20);
  // SizeFor = OBJECT_POINTER_ALIGN(offset); 原版 static_assert 保留在模块内
  assert_eq!(double_size_for(2), 32);
  assert!(double_size_for(DOUBLE_MAX_CAPACITY as usize) < 128 * 1024);
  OK
}
