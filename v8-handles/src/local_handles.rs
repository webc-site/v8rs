//! 由 tool/src/handles/local-handles.js 从 v8/src/handles/local-handles{.h,-inl.h,.cc} 生成
//! LocalHeap 依赖 crate 未冻结 (推迟): 以 LocalHeapRef 承载原版 is_main_thread 二分 ——
//! Main 分支转发全局 HandleScope arena (原主线程路径), Background 分支走 LocalHandles 自有分块,
//! 后端为裸指针 (原 local_heap 裸指针拓扑: 同宿主可挂多个嵌套作用域)。
//! RootVisitor 推迟: Iterate 以槽区间回调 [start, end) 形式暴露。
use std::ptr::{NonNull, null_mut};

use v8_common::Address;

use crate::handles::{
  Handle, HandleScope, HandleScopeData, K_HANDLE_BLOCK_SIZE, SlotPtr, with_arena,
};
#[cfg(debug_assertions)]
use crate::handles::{K_HANDLE_ZAP_VALUE, zap_range};

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
    Self {
      scope: HandleScopeData::initialize(),
      blocks: Vec::new(),
    }
  }

  /// 遍历所有存活句柄槽区间 [start, end) (原 Iterate(RootVisitor*)):
  /// 非末块整块访问, 末块截至 next
  pub fn iterate(&self, mut visit: impl FnMut(*const Address, *const Address)) {
    let Some((&back, rest)) = self.blocks.split_last() else {
      return;
    };
    for &block in rest {
      let start = block.as_ptr();
      // SAFETY: 块首 + 容量 K_HANDLE_BLOCK_SIZE
      let end = unsafe { start.add(K_HANDLE_BLOCK_SIZE) };
      visit(start, end);
    }
    // 末块存活句柄截至 next (可能不足整块)
    visit(back.as_ptr(), self.scope.next);
  }

  /// 槽位是否属于本容器 (原 DEBUG Contains): 块序无保证 → 全量线性 (原注释);
  /// 末块特判上界为 next (可能不足整块)
  pub fn contains(&self, location: SlotPtr) -> bool {
    let Some(&back) = self.blocks.last() else {
      return false;
    };
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

  /// 取槽写值 (原 LocalHandleScope::GetHandle 后台分支, 容器内聚)
  fn get_handle(&mut self, value: Address) -> SlotPtr {
    let result = if self.scope.next == self.scope.limit {
      self.add_block()
    } else {
      self.scope.next
    };
    debug_assert!(result < self.scope.limit);
    // SAFETY: result 指向容器持有块内空槽
    unsafe {
      self.scope.next = result.add(1);
      *result = value;
    }
    result
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

  /// debug 构建回收区填充 (原 ZapRange → HandleScope::ZapRange, 共用内核)
  #[cfg(debug_assertions)]
  fn zap_range(start: SlotPtr, end: SlotPtr) {
    zap_range(start, end, K_HANDLE_ZAP_VALUE);
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
    self.scope.limit = null_mut();
    self.remove_unused_blocks();
    debug_assert!(self.blocks.is_empty());
  }
}

/// LocalHeap 引用的最小承载 (原 is_main_thread 二分)。
/// Background 后端为裸指针: 原版 local_heap 即裸指针拓扑, 同一宿主可挂多个嵌套作用域
/// (安全引用后端会拒绝 &mut 别名)。
#[derive(Clone, Copy)]
pub enum LocalHeapRef {
  /// 主线程: 转发全局 HandleScope arena (原 GetMainThreadHandle/OpenMainThreadScope 路径)
  Main,
  /// 后台线程: 绑定 LocalHandles (原 local_heap->handles() 路径)
  Background {
    /// 宿主容器 (存活约定由 from_raw 调用方承担)
    handles: *mut LocalHandles,
  },
}

impl LocalHeapRef {
  /// 主线程后端关联常量
  pub const MAIN: Self = Self::Main;

  /// 由 LocalHandles 裸指针构造后台后端
  ///
  /// # Safety
  ///
  /// 调用方保证指针有效且存活覆盖全部派生作用域, 且不与其他可变借用并发
  pub unsafe fn from_raw(handles: *mut LocalHandles) -> Self {
    Self::Background { handles }
  }

  /// 后台容器可变引用
  ///
  /// # Safety
  ///
  /// from_raw 构造约定下指针有效
  unsafe fn handles_mut(&mut self) -> Option<&mut LocalHandles> {
    match self {
      Self::Main => None,
      // SAFETY: 构造约定存活
      Self::Background { handles } => Some(unsafe { &mut **handles }),
    }
  }
}

/// 局部句柄作用域 (原 class V8_NODISCARD LocalHandleScope)
#[must_use]
pub struct LocalHandleScope {
  /// 宿主 (原 local_heap_)
  heap: LocalHeapRef,
  prev_next: SlotPtr,
  prev_limit: SlotPtr,
}

impl LocalHandleScope {
  /// 进入作用域 (原 LocalHandleScope(LocalHeap*)): 主线程转发 OpenMainThreadScope,
  /// 后台线程保存 prev 并 level++
  pub fn new(heap: LocalHeapRef) -> Self {
    match heap {
      LocalHeapRef::Main => Self::open_main_thread_scope(),
      // SAFETY: 构造约定指针存活覆盖作用域
      LocalHeapRef::Background { handles } => unsafe {
        let lh = &mut *handles;
        let scope = Self {
          heap,
          prev_next: lh.scope.next,
          prev_limit: lh.scope.limit,
        };
        lh.scope.level += 1;
        scope
      },
    }
  }

  /// 在作用域内为 tagged 值分配句柄 (原 GetHandle + Handle 构造惯用法)
  pub fn allocate<T>(&mut self, value: Address) -> Handle<T> {
    Handle::from_location(Self::get_handle(&mut self.heap, value))
  }

  /// 在给定宿主上取槽 (原 static GetHandle(LocalHeap*, Address)):
  /// 主线程转发 GetMainThreadHandle, 后台线程走本地块
  pub fn get_handle(heap: &mut LocalHeapRef, value: Address) -> SlotPtr {
    match heap {
      LocalHeapRef::Main => Self::get_main_thread_handle(value),
      // SAFETY: 构造约定指针存活
      LocalHeapRef::Background { .. } => unsafe {
        heap
          .handles_mut()
          .expect("Background 分支必有容器")
          .get_handle(value)
      },
    }
  }

  /// 关闭并逃逸 (原 CloseAndEscape): 抛弃当前作用域句柄, 在父作用域重分配,
  /// 随后重新初始化当前作用域 (原注释); 空句柄安全返回 null (原版 UB 路径收窄)
  pub fn close_and_escape<T>(&mut self, handle_value: Handle<T>) -> Handle<T> {
    // 先取值再关闭 (原 Tagged<T> value = *handle_value): 防读悬垂;
    // 空句柄仅关闭重开, 不在父作用域分配
    let value = (!handle_value.is_null()).then(|| handle_value.value().ptr());
    // 抛弃当前作用域内全部句柄
    match self.heap {
      LocalHeapRef::Main => Self::close_main_thread_scope(self.prev_next, self.prev_limit),
      // SAFETY: 构造约定指针存活覆盖作用域
      LocalHeapRef::Background { handles } => unsafe {
        Self::close_scope(&mut *handles, self.prev_next, self.prev_limit);
      },
    }
    // 在父作用域分配一个句柄
    let location = value.map(|v| match self.heap {
      LocalHeapRef::Main => Self::get_main_thread_handle(v),
      // SAFETY: 构造约定指针存活
      LocalHeapRef::Background { handles } => unsafe { (*handles).get_handle(v) },
    });
    // 重新初始化当前作用域, 备再次关闭或继续使用 (原注释)
    self.reopen();
    location.map_or_else(Handle::null, Handle::from_location)
  }

  /// 重新初始化当前作用域 (原 CloseAndEscape 尾部惯用法):
  /// 以当前 next/limit 为新 prev, level++
  fn reopen(&mut self) {
    match self.heap {
      LocalHeapRef::Main => with_arena(|a| {
        self.prev_next = a.data.next;
        self.prev_limit = a.data.limit;
        a.data.level += 1;
      }),
      // SAFETY: 构造约定指针存活覆盖作用域
      LocalHeapRef::Background { handles } => unsafe {
        let lh = &mut *handles;
        self.prev_next = lh.scope.next;
        self.prev_limit = lh.scope.limit;
        lh.scope.level += 1;
      },
    }
  }

  /// 原主线程路径: GetMainThreadHandle —— 即 HandleScope::CreateHandle
  pub(crate) fn get_main_thread_handle(value: Address) -> SlotPtr {
    HandleScope::create_handle(value)
  }

  /// 原主线程路径: OpenMainThreadScope —— 保存 prev 并 level++
  fn open_main_thread_scope() -> Self {
    let mut scope = Self {
      heap: LocalHeapRef::Main,
      prev_next: null_mut(),
      prev_limit: null_mut(),
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
    if old_limit != prev_limit {
      handles.remove_unused_blocks();
      // limit 回退: zap 上界取回退后的 prev_limit
      #[cfg(debug_assertions)]
      LocalHandles::zap_range(handles.scope.next, prev_limit);
    } else {
      #[cfg(debug_assertions)]
      LocalHandles::zap_range(handles.scope.next, old_limit);
    }
  }
}

impl Drop for LocalHandleScope {
  fn drop(&mut self) {
    match self.heap {
      LocalHeapRef::Main => Self::close_main_thread_scope(self.prev_next, self.prev_limit),
      // SAFETY: 构造约定指针存活覆盖作用域
      LocalHeapRef::Background { handles } => unsafe {
        Self::close_scope(&mut *handles, self.prev_next, self.prev_limit);
      },
    }
  }
}
