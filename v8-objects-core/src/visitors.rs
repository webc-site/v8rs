// 由 tool/src/objects/visitors.h.js 从 v8/src/objects/visitors.h + visitors.cc(RootName) 生成; 修订就地改, 复跑覆盖
// visitor 骨架 trait 建模; Code/Reloc/沙箱槽方法推迟, 登记见脚本头

//! Visitor 体系骨架: GC 与序列化遍历根/对象指针的抽象访客。

use v8_common::GarbageCollector;

use crate::{
  error::{Error, Result},
  heap_object::HeapObject,
  tagged::Tagged,
};

// ── 槽位占位 (骨架寻址语义; 完整压缩槽族归 slots 层, 见脚本头登记) ──────────
macro_rules! slot_newtype {
  ($name:ident, $doc:literal, $step:expr) => {
    #[doc = $doc]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct $name {
      /// 槽位所在地址 (未打 tag 的裸地址)。
      pub address: usize,
    }
    impl $name {
      /// 以地址构造。
      #[inline]
      #[must_use]
      pub const fn new(address: usize) -> Self {
        Self { address }
      }
      /// 相邻下一槽 (原版 p + 1; 槽宽: FullObjectSlot 为 kSystemPointerSize=8,
      /// 压缩槽为 kTaggedSize=4)。
      #[inline]
      #[must_use]
      pub const fn next(self) -> Self {
        Self {
          address: self.address + $step,
        }
      }
    }
  };
}
slot_newtype!(FullObjectSlot, "全宽 (解压缩) tagged 对象槽。", 8);
slot_newtype!(
  OffHeapObjectSlot,
  "堆外压缩 tagged 对象槽 (字符串转发表等)。",
  4
);
slot_newtype!(ObjectSlot, "堆对象内强引用槽。", 4);
slot_newtype!(MaybeObjectSlot, "堆对象内可能弱引用槽。", 4);

// ── Root/SyncTag (ROOT_ID_LIST 机械展开, 28 项) ──────────────
/// 访客同步标记 (原版 VisitorSynchronization::SyncTag)。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum SyncTag {
  Bootstrapper,
  Builtins,
  ClientHeap,
  CodeFlusher,
  CompilationCache,
  Debug,
  Extensions,
  EternalHandles,
  ExternalStringsTable,
  GlobalHandles,
  HandleScope,
  IdentityMap,
  MicroTasks,
  ReadOnlyRootList,
  Relocatable,
  RetainMaps,
  SharedHeapObjectCache,
  SharedStructTypeRegistry,
  SmiRootList,
  StackRoots,
  StartupObjectCache,
  StringTable,
  StrongRootList,
  StrongRoots,
  ThreadManager,
  TracedHandles,
  WeakRoots,
  WriteBarrier,
  /// 标记总数 (原版 kNumberOfSyncTags)。
  NumberOfSyncTags,
}

/// GC/序列化根类别 (原版 enum Root)。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum Root {
  Bootstrapper,
  Builtins,
  ClientHeap,
  CodeFlusher,
  CompilationCache,
  Debug,
  Extensions,
  EternalHandles,
  ExternalStringsTable,
  GlobalHandles,
  HandleScope,
  IdentityMap,
  MicroTasks,
  ReadOnlyRootList,
  Relocatable,
  RetainMaps,
  SharedHeapObjectCache,
  SharedStructTypeRegistry,
  SmiRootList,
  StackRoots,
  StartupObjectCache,
  StringTable,
  StrongRootList,
  StrongRoots,
  ThreadManager,
  TracedHandles,
  WeakRoots,
  WriteBarrier,
  /// 根总数 (原版 kNumberOfRoots)。
  NumberOfRoots,
}

/// 根的人类可读名 (原版 RootVisitor::RootName, visitors.cc)。
#[inline]
#[must_use]
pub const fn root_name(root: Root) -> Option<&'static str> {
  match root {
    Root::Bootstrapper => Some("(Bootstrapper)"),
    Root::Builtins => Some("(Builtins)"),
    Root::ClientHeap => Some("(Client heap)"),
    Root::CodeFlusher => Some("(Code flusher)"),
    Root::CompilationCache => Some("(Compilation cache)"),
    Root::Debug => Some("(Debugger)"),
    Root::Extensions => Some("(Extensions)"),
    Root::EternalHandles => Some("(Eternal handles)"),
    Root::ExternalStringsTable => Some("(External strings)"),
    Root::GlobalHandles => Some("(Global handles)"),
    Root::HandleScope => Some("(Handle scope)"),
    Root::IdentityMap => Some("(Identity map)"),
    Root::MicroTasks => Some("(Micro tasks)"),
    Root::ReadOnlyRootList => Some("(Read-only roots)"),
    Root::Relocatable => Some("(Relocatable)"),
    Root::RetainMaps => Some("(Retain maps)"),
    Root::SharedHeapObjectCache => Some("(Shareable object cache)"),
    Root::SharedStructTypeRegistry => Some("(SharedStruct type registry)"),
    Root::SmiRootList => Some("(Smi roots)"),
    Root::StackRoots => Some("(Stack roots)"),
    Root::StartupObjectCache => Some("(Startup object cache)"),
    Root::StringTable => Some("(Internalized strings)"),
    Root::StrongRootList => Some("(Strong root list)"),
    Root::StrongRoots => Some("(Strong roots)"),
    Root::ThreadManager => Some("(Thread manager)"),
    Root::TracedHandles => Some("(Traced handles)"),
    Root::WeakRoots => Some("(Weak roots)"),
    Root::WriteBarrier => Some("(Write barrier)"),
    _ => None,
  }
}

// ── RootVisitor (visitors.h:66-128; C++ 虚函数 → trait) ──────────────────
/// 根指针访客抽象 (GC 与序列化/反序列化; 可选修改根区间内的指针)。
pub trait RootVisitor {
  /// 遍历 [start, end) 半开区间的全宽指针数组; 返回时值可已被修改。
  fn visit_root_pointers(
    &mut self,
    root: Root,
    description: Option<&str>,
    start: FullObjectSlot,
    end: FullObjectSlot,
  ) -> Result<()>;

  /// 单指针便捷遍历 (默认转区间; 原版默认体)。
  #[inline]
  fn visit_root_pointer(
    &mut self,
    root: Root,
    description: Option<&str>,
    p: FullObjectSlot,
  ) -> Result<()> {
    self.visit_root_pointers(root, description, p, p.next())
  }

  /// 堆外压缩指针区间 (默认不可达; 原版 UNREACHABLE → 禁 panic 落 Err)。
  #[inline]
  fn visit_compressed_root_pointers(
    &mut self,
    _root: Root,
    _description: Option<&str>,
    _start: OffHeapObjectSlot,
    _end: OffHeapObjectSlot,
  ) -> Result<()> {
    Err(Error::Unsupported(
      "VisitCompressedRootPointers: 默认不可达",
    ))
  }

  /// 正在运行的 Code 对象及其指令流 (默认按栈根遍历; 原版默认体)。
  #[inline]
  fn visit_running_code(
    &mut self,
    code_slot: FullObjectSlot,
    istream_or_smi_zero_slot: FullObjectSlot,
  ) -> Result<()> {
    // 对多数访客, 运行中代码与栈上其它指针无异。
    self.visit_root_pointer(Root::StackRoots, None, istream_or_smi_zero_slot)?;
    self.visit_root_pointer(Root::StackRoots, None, code_slot)
  }

  /// 序列化流位置标记 (默认空; 原版默认体)。
  #[inline]
  fn synchronize(&mut self, _tag: SyncTag) {}

  /// 触发本访客的回收器类型 (默认 MARK_COMPACTOR; 保守栈扫描分流用,
  /// 原版 enum GarbageCollector 由 v8-common 承载)。
  #[inline]
  #[must_use]
  fn collector(&self) -> GarbageCollector {
    GarbageCollector::MarkCompactor
  }
}

// ── ObjectVisitor (visitors.h:134-220; 核心槽位方法子集) ─────────────────
/// 对象指针访客抽象 (GC 与序列化; 可选修改对象内的指针)。
pub trait ObjectVisitor {
  /// 遍历宿主对象 [start, end) 强引用槽区间。
  fn visit_pointers(
    &mut self,
    host: Tagged<HeapObject>,
    start: ObjectSlot,
    end: ObjectSlot,
  ) -> Result<()>;

  /// 遍历宿主对象 [start, end) 可能弱引用槽区间。
  fn visit_maybe_pointers(
    &mut self,
    host: Tagged<HeapObject>,
    start: MaybeObjectSlot,
    end: MaybeObjectSlot,
  ) -> Result<()>;

  /// 自定义弱指针 (GC 须忽略, 其余访客按强指针处理; 原版默认体转强遍历)。
  #[inline]
  fn visit_custom_weak_pointers(
    &mut self,
    host: Tagged<HeapObject>,
    start: ObjectSlot,
    end: ObjectSlot,
  ) -> Result<()> {
    self.visit_pointers(host, start, end)
  }

  /// 单强引用槽便捷遍历 (原版默认体)。
  #[inline]
  fn visit_pointer(&mut self, host: Tagged<HeapObject>, p: ObjectSlot) -> Result<()> {
    self.visit_pointers(host, p, p.next())
  }

  /// 单可能弱引用槽便捷遍历 (原版默认体)。
  #[inline]
  fn visit_maybe_pointer(&mut self, host: Tagged<HeapObject>, p: MaybeObjectSlot) -> Result<()> {
    self.visit_maybe_pointers(host, p, p.next())
  }

  /// 单自定义弱槽便捷遍历 (原版默认体)。
  #[inline]
  fn visit_custom_weak_pointer(&mut self, host: Tagged<HeapObject>, p: ObjectSlot) -> Result<()> {
    self.visit_custom_weak_pointers(host, p, p.next())
  }

  /// ephemeron 表项 (键值各遍历一次; 原版默认体)。
  #[inline]
  fn visit_ephemeron(
    &mut self,
    host: Tagged<HeapObject>,
    _index: i32,
    key: ObjectSlot,
    value: ObjectSlot,
  ) -> Result<()> {
    self.visit_pointer(host, key)?;
    self.visit_pointer(host, value)
  }

  /// 宿主的 Map 指针 (默认不可达; 原版 UNREACHABLE → 禁 panic 落 Err)。
  #[inline]
  fn visit_map_pointer(&mut self, _host: Tagged<HeapObject>) -> Result<()> {
    Err(Error::Unsupported("VisitMapPointer: 默认不可达"))
  }
}
