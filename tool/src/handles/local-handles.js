#!/usr/bin/env bun
// tool/src/handles/local-handles.js — local-handles.h + local-handles-inl.h + local-handles.cc
// → v8-handles/src/local_handles.rs
// LocalHeap 尚未建 (依赖 crate 未冻结): 以 LocalHeapRef 枚举承载原版 is_main_thread 二分,
// Main 分支转发全局 HandleScope arena, Background 分支走 LocalHandles 自有分块。
import { readV8, writeSrc } from "../../lib.js";

const h = readV8("src/handles/local-handles.h");
const inl = readV8("src/handles/local-handles-inl.h");
const cc = readV8("src/handles/local-handles.cc");

// ---- 1. API 面对数 ----
const localHandlesApi = [
  "LocalHandles()", "~LocalHandles()", "Iterate(RootVisitor* visitor)", "Contains(Address* location)",
  "AddBlock()", "RemoveUnusedBlocks()", "ZapRange(Address* start, Address* end)",
];
for (const sig of localHandlesApi) if (!(h + cc).includes(sig.split("(")[0])) throw new Error(`LocalHandles ${sig} 未命中`);

const scopeApi = [
  "LocalHandleScope(LocalIsolate* local_isolate)", "LocalHandleScope(LocalHeap* local_heap)",
  "~LocalHandleScope()", "CloseAndEscape(HandleType<T> handle_value)", "GetHandle(LocalHeap* local_heap, Address value)",
  "CloseScope(LocalHeap* local_heap", "CloseMainThreadScope(LocalHeap*", "OpenMainThreadScope(LocalHeap*",
  "GetMainThreadHandle(LocalHeap*", "VerifyMainThreadScope()",
];
for (const sig of scopeApi) if (!(h + inl + cc).includes(sig.split("(")[0])) throw new Error(`LocalHandleScope ${sig} 未命中`);

// 数据成员对数
if (!h.includes("HandleScopeData scope_;")) throw new Error("LocalHandles.scope_ 未命中");
if (!h.includes("std::vector<Address*> blocks_;")) throw new Error("LocalHandles.blocks_ 未命中");
if (!/LocalHeap\* local_heap_;\s*Address\* prev_limit_;\s*Address\* prev_next_;/.test(h.replace(/\n\s*/g, " "))) {
  throw new Error("LocalHandleScope 成员布局未命中");
}

// ---- 2. 生成 Rust 初版 ----
const body = `//! 由 tool/src/handles/local-handles.js 从 v8/src/handles/local-handles{.h,-inl.h,.cc} 生成
//! LocalHeap 依赖 crate 未冻结 (推迟): 以 LocalHeapRef 承载原版 is_main_thread 二分 ——
//! Main 分支转发全局 HandleScope arena (原主线程路径), Background 分支走 LocalHandles 自有分块。
//! RootVisitor 推迟: Iterate 以槽区间回调 [start, end) 形式暴露。
use std::ptr::NonNull;

use crate::handles::{
  with_arena, Handle, HandleScope, HandleScopeData, SlotPtr, Tagged, K_HANDLE_BLOCK_SIZE,
  K_HANDLE_ZAP_VALUE,
};

/// 后台线程句柄容器 (原 class LocalHandles): 挂在 LocalHeap 上的作用域数据 + 自有块集
pub struct LocalHandles {
  /// 作用域数据 (原 scope_)
  pub(crate) scope: HandleScopeData,
  /// 自有块集 (原 std::vector<Address*> blocks_)
  blocks: Vec<NonNull<Address>>,
}

impl LocalHandles {
  /// 原 LocalHandles(): scope_.Initialize()
  pub fn new() -> Self {
    Self { scope: HandleScopeData::initialize(), blocks: Vec::new() }
  }

  /// 遍历所有存活句柄槽区间 [start, end) (原 Iterate(RootVisitor*)):
  /// 非末块整块访问, 末块截至 next
  pub fn iterate(&self, mut visit: impl FnMut(*const Address, *const Address)) {
    for (i, &block) in self.blocks.iter().enumerate() {
      let start = block.as_ptr();
      // SAFETY: 块首 + 容量 K_HANDLE_BLOCK_SIZE
      let end = if i + 1 < self.blocks.len() {
        unsafe { start.add(K_HANDLE_BLOCK_SIZE) }
      } else {
        self.scope.next
      };
      visit(start, end);
    }
  }

  /// 槽位是否属于本容器 (原 DEBUG Contains): 块序无保证 → 全量线性 (原注释);
  /// 末块特判上界为 next (可能不足整块)
  pub fn contains(&self, location: SlotPtr) -> bool {
    let Some(&back) = self.blocks.last() else { return false; };
    for &block in &self.blocks {
      let lower = block.as_ptr();
      // SAFETY: 块首 + 容量 K_HANDLE_BLOCK_SIZE; 末块上界改用 next
      let upper = unsafe {
        if block != back {
          lower.add(K_HANDLE_BLOCK_SIZE)
        } else {
          self.scope.next
        }
      };
      if lower <= location && location < upper {
        return true;
      }
    }
    false
  }

  /// 追加新块并切换 next/limit (原 private AddBlock)
  fn add_block(&mut self) -> SlotPtr {
    debug_assert_eq!(self.scope.next, self.scope.limit);
    let block: Box<[Address; K_HANDLE_BLOCK_SIZE]> = Box::new([0; K_HANDLE_BLOCK_SIZE]);
    // SAFETY: Box 指针非空
    let start = unsafe { NonNull::new_unchecked(Box::into_raw(block) as *mut Address) };
    self.blocks.push(start);
    self.scope.next = start.as_ptr();
    // SAFETY: 块首 + 容量
    self.scope.limit = unsafe { start.as_ptr().add(K_HANDLE_BLOCK_SIZE) };
    start.as_ptr()
  }

  /// 回收不再使用的块 (原 private RemoveUnusedBlocks): 块尾非当前 limit 的块全部释放
  fn remove_unused_blocks(&mut self) {
    while let Some(&back) = self.blocks.last() {
      // SAFETY: 块首 + 容量
      let block_limit = unsafe { back.as_ptr().add(K_HANDLE_BLOCK_SIZE) };
      if block_limit == self.scope.limit {
        break;
      }
      self.blocks.pop();
      // SAFETY: 块首来自 Box::into_raw, 此刻容器唯一持有
      drop(unsafe { Box::from_raw(back.as_ptr() as *mut [Address; K_HANDLE_BLOCK_SIZE]) });
    }
  }

  /// debug 构建回收区填充 (原 ZapRange → HandleScope::ZapRange)
  #[cfg(debug_assertions)]
  fn zap_range(start: SlotPtr, end: SlotPtr) {
    let mut p = start;
    while p != end {
      // SAFETY: p 遍历 [start, end) 均在容器持有块内
      unsafe {
        *p = K_HANDLE_ZAP_VALUE;
        p = p.add(1);
      }
    }
  }
}

impl Default for LocalHandles {
  fn default() -> Self {
    Self::new()
  }
}

impl Drop for LocalHandles {
  fn drop(&mut self) {
    // 原 ~LocalHandles(): limit 置空使全部块可回收
    self.scope.limit = std::ptr::null_mut();
    self.remove_unused_blocks();
    debug_assert!(self.blocks.is_empty());
  }
}

/// LocalHeap 引用的最小承载 (原 is_main_thread 二分)
pub enum LocalHeapRef<'a> {
  /// 主线程: 转发全局 HandleScope arena (原 GetMainThreadHandle/OpenMainThreadScope 路径)
  Main,
  /// 后台线程: 绑定 LocalHandles (原 local_heap->handles() 路径)
  Background(&'a mut LocalHandles),
}

/// 局部句柄作用域 (原 class V8_NODISCARD LocalHandleScope)
#[must_use]
pub struct LocalHandleScope<'a> {
  /// 宿主 (原 local_heap_)
  heap: LocalHeapRef<'a>,
  prev_next: SlotPtr,
  prev_limit: SlotPtr,
}

impl<'a> LocalHandleScope<'a> {
  /// 进入作用域 (原 LocalHandleScope(LocalHeap*)): 主线程转发 OpenMainThreadScope,
  /// 后台线程保存 prev 并 level++
  pub fn new(heap: LocalHeapRef<'a>) -> Self {
    match heap {
      LocalHeapRef::Main => Self::open_main_thread_scope(),
      LocalHeapRef::Background(ref handles) => {
        let mut scope = Self {
          heap,
          prev_next: handles.scope.next,
          prev_limit: handles.scope.limit,
        };
        if let LocalHeapRef::Background(handles) = &mut scope.heap {
          handles.scope.level += 1;
        }
        scope
      }
    }
  }

  /// 在给定宿主上取槽 (原 static GetHandle(LocalHeap*, Address)):
  /// 主线程转发 GetMainThreadHandle, 后台线程走本地块
  pub fn get_handle(heap: &mut LocalHeapRef, value: crate::handles::Address) -> SlotPtr {
    match heap {
      LocalHeapRef::Main => Self::get_main_thread_handle(value),
      LocalHeapRef::Background(handles) => {
        let result = if handles.scope.next == handles.scope.limit {
          handles.add_block()
        } else {
          handles.scope.next
        };
        debug_assert!(result < handles.scope.limit);
        // SAFETY: result 指向容器持有块内空槽
        unsafe {
          handles.scope.next = result.add(1);
          *result = value;
        }
        result
      }
    }
  }

  /// 关闭并逃逸 (原 CloseAndEscape): 抛弃当前作用域句柄, 在父作用域重分配,
  /// 随后重新初始化当前作用域 (原注释)
  pub fn close_and_escape<T>(&mut self, handle_value: Handle<T>) -> Handle<T> {
    let value = handle_value.value().ptr();
    match &mut self.heap {
      LocalHeapRef::Main => {
        Self::close_main_thread_scope(self.prev_next, self.prev_limit);
        let location = HandleScope::create_handle(value);
        with_arena(|a| {
          self.prev_next = a.data.next;
          self.prev_limit = a.data.limit;
          a.data.level += 1;
        });
        Handle::from_location(location)
      }
      LocalHeapRef::Background(handles) => {
        Self::close_scope(handles, self.prev_next, self.prev_limit);
        let location = Self::get_handle(&mut LocalHeapRef::Background(handles), value);
        self.prev_next = handles.scope.next;
        self.prev_limit = handles.scope.limit;
        handles.scope.level += 1;
        Handle::from_location(location)
      }
    }
  }

  /// 原主线程路径: GetMainThreadHandle —— 即 HandleScope::CreateHandle
  pub(crate) fn get_main_thread_handle(value: crate::handles::Address) -> SlotPtr {
    HandleScope::create_handle(value)
  }

  /// 原主线程路径: OpenMainThreadScope —— 保存 prev 并 level++
  fn open_main_thread_scope() -> Self {
    let mut scope = Self {
      heap: LocalHeapRef::Main,
      prev_next: std::ptr::null_mut(),
      prev_limit: std::ptr::null_mut(),
    };
    with_arena(|a| {
      scope.prev_next = a.data.next;
      scope.prev_limit = a.data.limit;
      a.data.level += 1;
    });
    scope
  }

  /// 原主线程路径: CloseMainThreadScope —— 即 HandleScope::CloseScope
  pub(crate) fn close_main_thread_scope(prev_next: SlotPtr, prev_limit: SlotPtr) {
    HandleScope::close_scope(prev_next, prev_limit);
  }

  /// 原后台路径: CloseScope —— 回退 next/limit, level--, 回收多余块
  fn close_scope(handles: &mut LocalHandles, prev_next: SlotPtr, prev_limit: SlotPtr) {
    let old_limit = handles.scope.limit;
    handles.scope.next = prev_next;
    handles.scope.limit = prev_limit;
    handles.scope.level -= 1;
    let mut limit = old_limit;
    if old_limit != handles.scope.limit {
      handles.remove_unused_blocks();
      limit = handles.scope.limit;
    }
    #[cfg(debug_assertions)]
    LocalHandles::zap_range(handles.scope.next, limit);
  }
}

impl Drop for LocalHandleScope<'_> {
  fn drop(&mut self) {
    match &mut self.heap {
      LocalHeapRef::Main => Self::close_main_thread_scope(self.prev_next, self.prev_limit),
      LocalHeapRef::Background(handles) => Self::close_scope(handles, self.prev_next, self.prev_limit),
    }
  }
}
`;

const p = writeSrc("v8-handles", "local_handles.rs", body);
console.log(`✔ v8-handles/src/local_handles.rs (${body.split("\n").length} 行)`);
console.log(`  API 对数: LocalHandles ${localHandlesApi.length} 面, LocalHandleScope ${scopeApi.length} 面全部命中`);
