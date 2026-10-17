#!/usr/bin/env bun
// tool/src/heap/local-factory.h.js
// 机械提取 v8/src/heap/local-factory.{h,cc,-inl.h}:
// LocalFactory (并发线程工厂) 定制点台账: ACCESSOR_INFO_ROOT_LIST 展开 +
// 4 个定制点 (AllocateRaw/CanAllocateInReadOnlySpace=false/
// EmptyStringRootIsInitialized=true/AllocationTypeForInPlaceInternalizableString) +
// 2 个 UNREACHABLE 错误方法。

import { readV8, writeSrc, banner } from "../../lib.js";

const h = readV8("src/heap/local-factory.h");
const cc = readV8("src/heap/local-factory.cc");
const inl = readV8("src/heap/local-factory-inl.h");

for (const a of [
  "class V8_EXPORT_PRIVATE LocalFactory : public FactoryBase<LocalFactory> {",
  "explicit LocalFactory(Isolate* isolate);",
  "ReadOnlyRoots read_only_roots() const { return roots_; }",
  "#define ROOT_ACCESSOR(Type, name, CamelName) inline DirectHandle<Type> name();",
  "ACCESSOR_INFO_ROOT_LIST(ROOT_ACCESSOR)",
  "DirectHandle<Object> NewInvalidStringLengthError() { UNREACHABLE(); }",
  "DirectHandle<Object> NewRangeError(MessageTemplate template_index) {",
  "Tagged<HeapObject> AllocateRaw(int size, AllocationType allocation,",
  "inline bool CanAllocateInReadOnlySpace() { return false; }",
  "inline bool EmptyStringRootIsInitialized() { return true; }",
  "inline AllocationType AllocationTypeForInPlaceInternalizableString();",
  "void ProcessNewScript(DirectHandle<Script> script,",
  "ReadOnlyRoots roots_;",
  "bool a_script_was_added_to_the_script_list_ = false;",
]) {
  if (!h.includes(a)) throw new Error(`local-factory.h 锚缺失: ${a}`);
}
for (const a of [
  "LocalFactory::LocalFactory(Isolate* isolate)",
  "Tagged<HeapObject> LocalFactory::AllocateRaw(",
  "void LocalFactory::ProcessNewScript(",
]) {
  if (!cc.includes(a)) throw new Error(`local-factory.cc 锚缺失: ${a}`);
}
if (!inl.includes("AllocationType LocalFactory::AllocationTypeForInPlaceInternalizableString()")) {
  throw new Error("local-factory-inl.h AllocationTypeForInPlaceInternalizableString 缺失");
}
console.log("LocalFactory: 定制点 4 + UNREACHABLE 2 + 根访问面");

let rust = banner(
  "src/heap/local-factory.h.js",
  "src/heap/local-factory.{h,cc,-inl.h}",
  "并发线程工厂: FactoryBase 定制点台账",
);

rust += `
/// 并发线程工厂骨架 (对应 class LocalFactory : public FactoryBase<LocalFactory>)。
/// 解析/编译后台线程用; 错误路径 UNREACHABLE (解析器保证不产生错误)。
/// v8: src/heap/local-factory.h LocalFactory
pub struct LocalFactorySkeleton {
  /// 对应 ReadOnlyRoots roots_ (主线程快照, 后台只读)
  pub roots: (),
}

impl LocalFactorySkeleton {
  /// 对应 CanAllocateInReadOnlySpace: 恒 false (只读空间只在启动期由主线程分配)
  /// v8: src/heap/local-factory.h LocalFactory::CanAllocateInReadOnlySpace
  #[must_use]
  pub fn can_allocate_in_read_only_space() -> bool {
    false
  }

  /// 对应 EmptyStringRootIsInitialized: 恒 true (构造时已初始化)
  /// v8: src/heap/local-factory.h LocalFactory::EmptyStringRootIsInitialized
  #[must_use]
  pub fn empty_string_root_is_initialized() -> bool {
    true
  }

  /// 对应 NewInvalidStringLengthError: 后台线程不可能走到 → UNREACHABLE
  /// v8: src/heap/local-factory.h LocalFactory::NewInvalidStringLengthError
  pub fn new_invalid_string_length_error() -> ! {
    unreachable!("LocalFactory: 解析器不允许产生错误")
  }

  /// 对应 NewRangeError: 同上 UNREACHABLE
  /// v8: src/heap/local-factory.h LocalFactory::NewRangeError
  pub fn new_range_error(_template_index: u32) -> ! {
    unreachable!("LocalFactory: 解析器不允许产生错误")
  }

  /// 对应 AllocateRaw: 经 LocalHeap 分配 (后台线程)
  /// v8: src/heap/local-factory.cc LocalFactory::AllocateRaw
  pub fn allocate_raw(&mut self, _size: i32) -> u64 {
    unimplemented!("骨架: LocalHeap::AllocateRaw 待 local-heap 族就位");
  }
}

// —— 提取不变式断言 ——
const _: () = {
  assert!(!LocalFactorySkeleton::can_allocate_in_read_only_space());
  assert!(LocalFactorySkeleton::empty_string_root_is_initialized());
};
`;

const p = writeSrc("v8-heap-core", "local_factory.rs.skel", rust);
console.log(`written ${p}`);
