// 迁移规约:
//   visitors.h:20-62 ROOT_ID_LIST → Root/SyncTag 28 变体 (含描述字符串锚点)
//   visitors.cc:12-25 RootVisitor::RootName
//   visitors.h:66-128 RootVisitor 默认方法体 (visit_root_pointer 转区间 /
//     visit_running_code 双指针 / collector 默认 MARK_COMPACTOR)
//   visitors.h:134-220 ObjectVisitor 默认方法体 (visit_pointer 转区间 /
//     visit_custom_weak 默认转强 / visit_ephemeron 键值各遍历 /
//     visit_map_pointer 默认不可达)
//   ClientRootVisitor/ClientObjectVisitor/cage bases (共享堆 GC): 归 v8-heap-gc
use aok::{OK, Void};
use v8_common::GarbageCollector;
use v8_objects_core::{
  Error, Result,
  heap_object::HeapObject,
  tagged::{Object, Tagged},
  visitors::{
    FullObjectSlot, MaybeObjectSlot, ObjectSlot, ObjectVisitor, OffHeapObjectSlot, Root,
    RootVisitor, SyncTag, root_name,
  },
};

#[ctor::ctor(unsafe)]
fn init() {
  log_init::init();
}

/// 规约 (visitors.h:20-48 + visitors.cc RootName):
/// 根类别 28 个 + 哨兵; 首尾名称锚点。
#[test]
fn root_enum_and_names() -> Void {
  assert_eq!(Root::Bootstrapper as u8, 0);
  assert_eq!(Root::WriteBarrier as u8, 27);
  assert_eq!(Root::NumberOfRoots as u8, 28);
  assert_eq!(SyncTag::Bootstrapper as u8, 0);
  assert_eq!(SyncTag::NumberOfSyncTags as u8, 28);

  assert_eq!(root_name(Root::Bootstrapper), Some("(Bootstrapper)"));
  assert_eq!(root_name(Root::ReadOnlyRootList), Some("(Read-only roots)"));
  assert_eq!(root_name(Root::WriteBarrier), Some("(Write barrier)"));
  assert_eq!(root_name(Root::NumberOfRoots), None); // 哨兵无名
  OK
}

/// 规约 (骨架槽位寻址): FullObjectSlot 步进 = kSystemPointerSize (8,
/// slots.h:101-105 全宽槽); 堆内/堆外压缩槽步进 = kTaggedSize (压缩配置 4)。
#[test]
fn slot_address_arithmetic() -> Void {
  let s = FullObjectSlot::new(0x1000);
  assert_eq!(s.next().address, 0x1008);
  let os = ObjectSlot::new(8);
  assert_eq!(os.next().address, 12);
  let ms = MaybeObjectSlot::new(0);
  assert_eq!(ms.next().address, 4);
  let ohs = OffHeapObjectSlot::new(0);
  assert_eq!(ohs.next().address, 4);
  OK
}

/// 记录型 RootVisitor: 观察默认方法委派。
#[derive(Default)]
struct RecordingRootVisitor {
  ranges: usize,
  syncs: Vec<SyncTag>,
}

impl RootVisitor for RecordingRootVisitor {
  fn visit_root_pointers(
    &mut self,
    _root: Root,
    _description: Option<&str>,
    start: FullObjectSlot,
    end: FullObjectSlot,
  ) -> Result<()> {
    assert_eq!(end.address, start.address + 8); // 单指针便捷遍历恰 1 槽 (全宽 8B)
    self.ranges += 1;
    Ok(())
  }

  fn synchronize(&mut self, tag: SyncTag) {
    self.syncs.push(tag);
  }
}

/// 规约 (visitors.h:76-79 默认体): VisitRootPointer → VisitRootPointers(p, p+1)。
#[test]
fn root_visitor_default_delegation() -> Void {
  let mut v = RecordingRootVisitor::default();
  let slot = FullObjectSlot::new(0x2000);
  v.visit_root_pointer(Root::StackRoots, Some("stack"), slot)
    .unwrap();
  assert_eq!(v.ranges, 1);

  // VisitRunningCode: istream 与 code 各按栈根遍历一次 (visitors.h:105-111)
  v.visit_running_code(slot, slot.next()).unwrap();
  assert_eq!(v.ranges, 3);

  // Synchronize 默认空 — 子类覆盖后可见 (visitors.h:115-116)
  v.synchronize(SyncTag::Debug);
  assert_eq!(v.syncs, vec![SyncTag::Debug]);

  // collector 默认 MARK_COMPACTOR (visitors.h:125-127)
  assert_eq!(v.collector(), GarbageCollector::MarkCompactor);
  OK
}

/// 记录型 ObjectVisitor: 观察默认方法委派。
#[derive(Default)]
struct CountingObjectVisitor {
  strong: usize,
}

impl ObjectVisitor for CountingObjectVisitor {
  fn visit_pointers(
    &mut self,
    _host: Tagged<HeapObject>,
    start: ObjectSlot,
    end: ObjectSlot,
  ) -> Result<()> {
    assert_eq!(end.address, start.address + 4);
    self.strong += 1;
    Ok(())
  }

  fn visit_maybe_pointers(
    &mut self,
    _host: Tagged<HeapObject>,
    start: MaybeObjectSlot,
    end: MaybeObjectSlot,
  ) -> Result<()> {
    assert_eq!(end.address, start.address + 4);
    self.strong += 1;
    Ok(())
  }
}

/// 规约 (visitors.h:155-175 默认体): 单指针/自定义弱/ephemeron 委派链。
#[test]
fn object_visitor_default_delegation() -> Void {
  let host: Tagged<HeapObject> = Tagged::new(0x1);
  let mut v = CountingObjectVisitor::default();

  v.visit_pointer(host, ObjectSlot::new(0x10)).unwrap();
  assert_eq!(v.strong, 1);

  // 自定义弱指针默认按强指针处理 (visitors.h:155-158)
  v.visit_custom_weak_pointers(host, ObjectSlot::new(0x20), ObjectSlot::new(0x24))
    .unwrap();
  assert_eq!(v.strong, 2);

  // ephemeron: 键值各遍历一次 (visitors.h:171-175)
  v.visit_ephemeron(host, 3, ObjectSlot::new(0x30), ObjectSlot::new(0x40))
    .unwrap();
  assert_eq!(v.strong, 4);

  // maybe 槽位便捷遍历 (visitors.h:163-166)
  v.visit_maybe_pointer(host, MaybeObjectSlot::new(0x50))
    .unwrap();
  assert_eq!(v.strong, 5);

  // Map 指针默认不可达 (visitors.h:219 UNREACHABLE → 禁 panic 落 Err)
  assert!(matches!(
    v.visit_map_pointer(host),
    Err(Error::Unsupported(_))
  ));
  OK
}

/// 规约 (visitors.h:97-101): 压缩根指针区间默认不可达。
#[test]
fn compressed_roots_default_unreachable() -> Void {
  let mut v = RecordingRootVisitor::default();
  let r = v.visit_compressed_root_pointers(
    Root::StringTable,
    None,
    OffHeapObjectSlot::new(0),
    OffHeapObjectSlot::new(4),
  );
  assert!(matches!(r, Err(Error::Unsupported(_))));
  OK
}

/// 占位: Tagged<Object> 槽语义由 slots 层承接 (登记), 此处仅锚定类型可构造。
#[test]
fn tagged_object_placeholder() -> Void {
  let o: Tagged<Object> = Tagged::NULL;
  assert!(o.is_null());
  OK
}
