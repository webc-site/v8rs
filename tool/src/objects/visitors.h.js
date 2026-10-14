#!/usr/bin/env bun
// tool/src/objects/visitors.h.js — 覆盖 visitors.h 同名族骨架:
//   v8/src/objects/visitors.h + visitors.cc (RootName) + visitors-inl.h (骨架外延)
//   → v8-objects-core/src/visitors.rs
//
// 映射登记 (C++ → Rust):
//   ROOT_ID_LIST X-macro → enum SyncTag + enum Root (同名变体 1:1) + ROOT_NAMES
//     描述字符串表 (RootName, visitors.cc:12)
//   class RootVisitor (虚函数大户) → trait RootVisitor (默认方法 = C++ 虚默认体;
//     glossary: 虚函数大户 trait 建模; 具体访客闭集落地后可叠 enum_dispatch)
//   class ObjectVisitor → trait ObjectVisitor (核心槽位方法子集)
//   FullObjectSlot/OffHeapObjectSlot/ObjectSlot/MaybeObjectSlot → 最小地址 newtype
//     占位 (globals.rs 登记的槽位族 L3 落位, 此处仅承载骨架所需寻址语义)
//   VisitRunningCode/Synchronize/collector 默认体 → 默认方法 1:1
//
// 范围裁剪登记 (推迟):
//   - VisitInstructionStreamPointer/VisitRelocInfo/VisitCodeTarget 等 Code/RelocInfo
//     依赖方法 (visitors.h:148-219): 归 v8-objects-code / v8-objects-trusted
//   - VisitExternalPointer/VisitCppHeapPointer/VisitIndirectPointer/
//     VisitProtectedPointer/VisitTrustedPointerTableEntry/VisitJSDispatchTableEntry
//     (沙箱句柄槽依赖): 归 v8-sandbox
//   - ObjectVisitorWithCageBases (指针压缩 cage 缓存): 归 v8-heap-gc
//   - ClientRootVisitor/ClientObjectVisitor 模板包装 (共享堆 GC): 归 v8-heap-gc
//   - 槽位压缩编解码 (cage base 依赖): 归 slots 层 (globals.rs 已登记 L3)
import { banner, readV8, writeSrc } from "../../lib.js";

const src = readV8("src/objects/visitors.h");
const cc = readV8("src/objects/visitors.cc");

const mustContain = (s, sub, file) => {
  if (!s.includes(sub)) throw new Error(`锚点失配: ${file} 无 "${sub.slice(0, 48)}..."`);
};
mustContain(src, "#define ROOT_ID_LIST(V)", "visitors.h");
mustContain(src, "class RootVisitor {", "visitors.h");
mustContain(src, "class ObjectVisitor {", "visitors.h");
mustContain(cc, "const char* RootVisitor::RootName(Root root)", "visitors.cc");

// 机械提取 ROOT_ID_LIST 变体: V(kName, "(desc)")
const listStart = src.indexOf("#define ROOT_ID_LIST(V)");
const listEnd = src.indexOf("\n\n", listStart);
const listBody = src.slice(listStart, listEnd);
const entries = [...listBody.matchAll(/V\((\w+),\s*"([^"]+)"\)/g)].map((m) => ({
  variant: m[1],
  desc: m[2],
}));
if (entries.length < 20) throw new Error(`ROOT_ID_LIST 提取 ${entries.length} 项, 疑似失配`);
// 对数锚点: 首尾变体
if (entries[0].variant !== "kBootstrapper" || entries.at(-1).variant !== "kWriteBarrier") {
  throw new Error(`ROOT_ID_LIST 首尾锚点失配: ${entries[0].variant}..${entries.at(-1).variant}`);
}

// 变体名去 k 前缀 → PascalCase (kHandleScope → HandleScope)
function pascal(v) {
  return v
    .replace(/^k/, "")
    .split(/(?=[A-Z])/)
    .map((p) => p)
    .join("");
}
const variants = entries.map((e) => pascal(e.variant));
// 对数: visitors.cc 的 RootName 即 ROOT_ID_LIST(ROOT_CASE) 展开 (宏驱动, 无字面分支)
mustContain(cc, "ROOT_ID_LIST(ROOT_CASE)", "visitors.cc");

const enumBlock = variants.map((v) => `  ${v},`).join("\n");
const nameTable = entries
  .map((e) => `      Root::${pascal(e.variant)} => Some("${e.desc}"),`)
  .join("\n");

const body = `${banner("src/objects/visitors.h.js", "src/objects/visitors.h + visitors.cc(RootName)", "visitor 骨架 trait 建模; Code/Reloc/沙箱槽方法推迟, 登记见脚本头")}
//! Visitor 体系骨架: GC 与序列化遍历根/对象指针的抽象访客。

use crate::error::{Error, Result};
use crate::heap_object::HeapObject;
use crate::tagged::Tagged;
use v8_common::GarbageCollector;

// ── 槽位占位 (骨架寻址语义; 完整压缩槽族归 slots 层, 见脚本头登记) ──────────
macro_rules! slot_newtype {
  ($name:ident, $doc:literal) => {
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
      /// 相邻下一槽 (原版 p + 1; 槽宽 kTaggedSize=4, 压缩配置)。
      #[inline]
      #[must_use]
      pub const fn next(self) -> Self {
        Self { address: self.address + 4 }
      }
    }
  };
}
slot_newtype!(FullObjectSlot, "全宽 (解压缩) tagged 对象槽。");
slot_newtype!(OffHeapObjectSlot, "堆外压缩 tagged 对象槽 (字符串转发表等)。");
slot_newtype!(ObjectSlot, "堆对象内强引用槽。");
slot_newtype!(MaybeObjectSlot, "堆对象内可能弱引用槽。");

// ── Root/SyncTag (ROOT_ID_LIST 机械展开, ${entries.length} 项) ──────────────
/// 访客同步标记 (原版 VisitorSynchronization::SyncTag)。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum SyncTag {
${enumBlock}
  /// 标记总数 (原版 kNumberOfSyncTags)。
  NumberOfSyncTags,
}

/// GC/序列化根类别 (原版 enum Root)。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum Root {
${enumBlock}
  /// 根总数 (原版 kNumberOfRoots)。
  NumberOfRoots,
}

/// 根的人类可读名 (原版 RootVisitor::RootName, visitors.cc)。
#[inline]
#[must_use]
pub const fn root_name(root: Root) -> Option<&'static str> {
  match root {
${nameTable}
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
    Err(Error::Unsupported("VisitCompressedRootPointers: 默认不可达"))
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
  fn visit_pointers(&mut self, host: Tagged<HeapObject>, start: ObjectSlot, end: ObjectSlot) -> Result<()>;

  /// 遍历宿主对象 [start, end) 可能弱引用槽区间。
  fn visit_maybe_pointers(&mut self, host: Tagged<HeapObject>, start: MaybeObjectSlot, end: MaybeObjectSlot) -> Result<()>;

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
`;

writeSrc("v8-objects-core", "visitors.rs", body);
console.log(
  `visitors.rs: Root/SyncTag ${variants.length} 变体 + RootName 表 + RootVisitor/ObjectVisitor trait (默认方法 1:1); ` +
    "Code/沙箱槽方法推迟已登记",
);
