//! MapWord 语义面 (map-word.h / map-word-inl.h / objects.cc IsMapOrForwarded)
//! 钉死配置: V8_EXTERNAL_CODE_SPACE 开 → 转发 = Smi 编码 kObjectAlignment 块差值。

use aok::{OK, Void};
use v8_common::K_OBJECT_ALIGNMENT;
use v8_objects_core::{
  heap_object::HeapObject, instance_type::InstanceType, map::Map, map_word::MapWord, smi::Smi,
  tagged::Tagged,
};
use v8_zone::{AccountingAllocator, Zone};

#[ctor::ctor(unsafe)]
fn init() {
  log_init::init();
}

#[test]
fn test_map_word_from_map_roundtrip() -> Void {
  // 原版 map-word-inl.h: FromMap/ToMap (默认非打包直存)
  let map = Tagged::<Map>::new(0x2001);
  let map_word = MapWord::from_map(map);

  assert_eq!(map_word.ptr(), 0x2001);
  assert!(!map_word.is_forwarding_address());
  assert_eq!(map_word.to_map(), map);

  OK
}

#[test]
fn test_map_word_forwarding_address() -> Void {
  // 原版 map-word-inl.h ECS 分支: 转发 = Smi((object - host) / kObjectAlignment)
  let host = Tagged::<HeapObject>::new(0x1001);
  let target = Tagged::<HeapObject>::new(0x3001);
  let map_word = MapWord::from_forwarding_address(host, target);

  let diff = target.ptr() - host.ptr();
  assert_eq!(
    map_word.ptr(),
    Smi::from_int31((diff / K_OBJECT_ALIGNMENT) as i32).ptr()
  );
  // ECS: Smi 编码 (bit0=0) → 判定为转发
  assert!(map_word.is_forwarding_address());

  // 还原目标 = host.address() + Smi 值 × kObjectAlignment + kHeapObjectTag
  assert_eq!(map_word.to_forwarding_address(host), target);

  // 同址转发 (差 0 → Smi::zero())
  let same = MapWord::from_forwarding_address(host, host);
  assert_eq!(same.ptr(), 0);
  assert_eq!(same.to_forwarding_address(host), host);

  // 正常 Map 指针 (低 1 位 01) 不是转发地址
  let normal = MapWord::from_map(Tagged::<Map>::new(0x2001));
  assert!(!normal.is_forwarding_address());

  // 打包判定: 默认配置恒 false (V8_MAP_PACKING 未启用)
  assert!(!MapWord::is_packed(0));

  OK
}

#[test]
fn test_map_word_equality() -> Void {
  // 原版 operator==: 全值比较
  let a = MapWord::from_map(Tagged::<Map>::new(0x2001));
  let b = MapWord::from_map(Tagged::<Map>::new(0x2001));
  let c = MapWord::from_map(Tagged::<Map>::new(0x4001));
  assert_eq!(a, b);
  assert_ne!(a, c);

  OK
}

#[test]
fn test_map_word_is_map_or_forwarded() -> Void {
  let allocator = AccountingAllocator::default();
  let zone = Zone::new(&allocator, "map-word-test");

  // 元 Map: 自指 + InstanceType::Map
  // SAFETY: 先占位分配, 再回填自指首字段; zone 生命周期覆盖断言期
  let meta = {
    let placeholder = Tagged::<Map>::new(0);
    let tagged =
      Tagged::allocate_in_zone(&zone, Map::new(placeholder, InstanceType::Map, 48)).unwrap();
    // SAFETY: zone 内指针有效, 独占访问
    unsafe {
      (*tagged.as_mut_ptr()).header.map = tagged.cast();
    }
    tagged
  };

  // meta 自指且 InstanceType::Map → 判定为 Map
  // SAFETY: meta 指向 zone 内有效 Map, 无并发访问
  assert!(unsafe { MapWord::is_map_or_forwarded(meta) });

  // 健康 map (元 map 为 Map 类型) → 判定为 Map (map_word.ToMap() 即 meta, 类型 Map)
  let js_map = Map::new(meta, InstanceType::JSObject, 32);
  let tagged = Tagged::allocate_in_zone(&zone, js_map).unwrap();
  // SAFETY: zone 生命周期覆盖断言期
  assert!(unsafe { MapWord::is_map_or_forwarded(tagged) });

  // 元 map 非法 (实例类型非 Map) → 判定失败
  let fake = Tagged::allocate_in_zone(&zone, Map::new(meta, InstanceType::JSObject, 32)).unwrap();
  let suspect = Tagged::allocate_in_zone(
    &zone,
    Map::new(Tagged::<Map>::new(0), InstanceType::JSObject, 32),
  )
  .unwrap();
  // SAFETY: zone 内指针有效, 只改首字段
  unsafe {
    (*suspect.as_mut_ptr()).header.map = fake.cast();
  }
  // SAFETY: 同上
  assert!(!unsafe { MapWord::is_map_or_forwarded(suspect) });

  // 元 Map 槽位被 GC 转发 (低 2 位 00) → 判定为转发
  // SAFETY: 指针有效, 只改首字段
  unsafe {
    (*suspect.as_mut_ptr()).header.map = Tagged::<Map>::new(0x5000);
  }
  // SAFETY: 同上
  assert!(unsafe { MapWord::is_map_or_forwarded(suspect) });

  OK
}
