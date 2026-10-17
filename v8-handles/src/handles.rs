//! 由 tool/src/handles/handles.js 从 v8/src/handles/{handles.h,handles.cc,handles-inl.h} 生成
//! 底层存储: thread_local 分块地址数组 (HandleArena), 语义对齐原版
//! HandleScopeImplementer 的 blocks/spare 数据布局 (该族推迟转写, 见 README)。
//!
//! SAFETY 总约: SlotPtr 指向的句柄槽仅在其所属 HandleScope 关闭前有效 ——
//! 与原版 "scope 删除后访问句柄为 UB" 约定一致; 块内存由 Arena/容器持有, 存活期内地址稳定。
use std::{
  cell::RefCell,
  marker::PhantomData,
  mem::{size_of, swap},
  ptr::{NonNull, null_mut},
};

use v8_base::small_vector::SmallVector;
use v8_common::{Address, K_SMI_TAG_MASK, KB};

/// 原 kTaggedNullAddress (handles.h): direct-handle 空值哨兵
pub const K_TAGGED_NULL_ADDRESS: Address = 0x1;
/// 原 kHandleBlockSize (handle-scope-implementer.h): KB - 2, 单页恰好容纳
pub const K_HANDLE_BLOCK_SIZE: usize = KB - 2;
/// 原 HandleScope::kCheckHandleThreshold (handles.h): --check-handle-count 门限
pub const K_CHECK_HANDLE_THRESHOLD: usize = 48 * 1024;
/// 原 kHandleZapValue (common/globals.h:1154, 64 位): debug 构建句柄回收区填充值
pub const K_HANDLE_ZAP_VALUE: Address = 0x1baddead0baddeaf;
/// 原 kPersistentHandleZapValue (common/globals.h:1156): persistent 块回收填充值,
/// 与句柄 zap 值刻意不同 (误用会在 zap 断言测试暴露)
pub const K_PERSISTENT_HANDLE_ZAP_VALUE: Address = 0x1baffed66baffedf;

// 单一真相对账: 本 crate 常量必须与 v8_common (globals 转写) 逐位一致
const _: () = assert!(K_HANDLE_ZAP_VALUE as u64 == v8_common::K_HANDLE_ZAP_VALUE);
const _: () =
  assert!(K_PERSISTENT_HANDLE_ZAP_VALUE as u64 == v8_common::K_PERSISTENT_HANDLE_ZAP_VALUE);

/// 句柄槽位指针, 对应原版 `Address*`
pub type SlotPtr = *mut Address;
/// 句柄块: 连续 K_HANDLE_BLOCK_SIZE 个槽位, 对应原版 NewArray<Address>(kHandleBlockSize)
pub type Block = [Address; K_HANDLE_BLOCK_SIZE];

// ----------------------------------------------------------------------------
// 幻类型对象层 (v8-objects-core 并行开发未冻结, 本 crate 自带最小面, 禁依赖之)

/// 任意 tagged 值, 对应原版 class Object
pub struct Object;
/// 堆对象, 对应原版 class HeapObject
pub struct HeapObject;
/// Smi 立即数, 对应原版 class Smi
pub struct Smi;

/// tagged 指针的幻类型包装 (原 src/objects/tagged.h 的最小内聚面)
/// 手动 impl Copy/Clone: derive 会强加 `T: Copy` bound, 而幻类型参数不参与复制
#[derive(Debug, PartialEq, Eq)]
pub struct Tagged<T> {
  ptr: Address,
  _m: PhantomData<fn() -> T>,
}

impl<T> Clone for Tagged<T> {
  fn clone(&self) -> Self {
    *self
  }
}
impl<T> Copy for Tagged<T> {}

impl<T> Tagged<T> {
  /// 由原始地址构造, 不做类型检查 (原 Tagged<T>(Address))
  pub const fn from_ptr(ptr: Address) -> Self {
    Self {
      ptr,
      _m: PhantomData,
    }
  }

  /// 原始地址 (原 .ptr())
  pub const fn ptr(self) -> Address {
    self.ptr
  }
}

impl Tagged<Smi> {
  /// Smi 编码: 31 位值左移 1 位 (kSmiTag = 0);
  /// `v as u32` 位模式重解释 + 移位高位自然丢弃, 负数不触发溢出 panic
  pub const fn smi(v: i32) -> Self {
    Self {
      ptr: ((v as u32) << 1) as Address,
      _m: PhantomData,
    }
  }

  /// Smi 解码 (原 Tagged<Smi>::value()): 低 32 位转 i32 后算术右移 1 位 (负 Smi 复原)
  pub const fn value(self) -> i32 {
    (self.ptr as u32 as i32) >> 1
  }
}

/// 是否 Smi (低位 tag == kSmiTag == 0)
pub const fn is_smi(ptr: Address) -> bool {
  ptr & K_SMI_TAG_MASK == 0
}

// ----------------------------------------------------------------------------
// HandleScopeData (原 include/v8-internal.h)

/// 每线程句柄作用域数据: next/limit 推进指针 + level/sealed_level 深度
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct HandleScopeData {
  /// 下一个空闲槽 (原 next)
  pub next: SlotPtr,
  /// 当前块槽位上界 (原 limit)
  pub limit: SlotPtr,
  /// 未关闭作用域数 (原 level)
  pub level: i32,
  /// 封印深度 (原 sealed_level)
  pub sealed_level: i32,
}

impl HandleScopeData {
  /// 原 Initialize(): 全零态
  pub const fn initialize() -> Self {
    Self {
      next: null_mut(),
      limit: null_mut(),
      level: 0,
      sealed_level: 0,
    }
  }
}

// ----------------------------------------------------------------------------
// HandleArena: 分块地址数组, HandleScopeImplementer 的最小分块面

/// 当前线程句柄竞技场: blocks(已提交块) + spare(备用块) + 作用域数据
/// 布局 1:1 对应原版 isolate->handle_scope_data() 与 implementer->blocks()/spare_
pub(crate) struct HandleArena {
  /// 已提交块首指针 (原 DetachableVector<Address*> blocks_)
  blocks: Vec<NonNull<Address>>,
  /// 备用块 (原 spare_): DeleteExtensions 回收的块优先复用
  spare: Option<Box<Block>>,
  /// 作用域数据 (原 handle_scope_data_)
  pub(crate) data: HandleScopeData,
  /// persistent 作用域首句柄 (原 last_handle_before_persistent_block_)
  last_handle_before_persistent_block: Option<SlotPtr>,
}

impl HandleArena {
  fn new() -> Self {
    Self {
      blocks: Vec::new(),
      spare: None,
      data: HandleScopeData::initialize(),
      last_handle_before_persistent_block: None,
    }
  }

  /// 原 HandleScopeImplementer::GetSpareOrNewBlock: 有备用块则复用, 否则新分配
  fn get_spare_or_new_block(&mut self) -> NonNull<Address> {
    let block: Box<Block> = self
      .spare
      .take()
      .unwrap_or_else(|| Box::new([0; K_HANDLE_BLOCK_SIZE]));
    // SAFETY: Box 指针非空
    unsafe { NonNull::new_unchecked(Box::into_raw(block) as *mut Address) }
  }

  /// 原 HandleScope::Extend: 为当前作用域扩展槽位, 返回可用的 result 槽
  fn extend(&mut self) -> SlotPtr {
    let result = self.data.next;
    // 顶层必须存在未封印作用域 (原 Utils::ApiCheck fatal)
    debug_assert_ne!(
      self.data.level, self.data.sealed_level,
      "Cannot create a handle without a HandleScope"
    );
    // 最后一块尚有余量则直接收作 limit —— 作用域屏障后的快速建域 (原注释)
    if let Some(&back) = self.blocks.last() {
      // SAFETY: back 为块首, 块容量 K_HANDLE_BLOCK_SIZE, add 不越界
      let limit = unsafe { back.as_ptr().add(K_HANDLE_BLOCK_SIZE) };
      if self.data.limit != limit {
        self.data.limit = limit;
        debug_assert!(
          (limit as usize - self.data.next as usize) / size_of::<Address>() < K_HANDLE_BLOCK_SIZE
        );
      }
    }
    // 仍无空槽: 分配新块扩展当前作用域
    if result == self.data.limit {
      let block = self.get_spare_or_new_block();
      self.blocks.push(block);
      // SAFETY: 块首 + 容量 K_HANDLE_BLOCK_SIZE 得块尾
      self.data.limit = unsafe { block.as_ptr().add(K_HANDLE_BLOCK_SIZE) };
      return block.as_ptr();
    }
    result
  }

  /// 原 HandleScope::CreateHandle: 在当前作用域写入 value, 返回槽位
  fn create_handle(&mut self, value: Address) -> SlotPtr {
    let result = self.data.next;
    if result == self.data.limit {
      let slot = self.extend();
      self.create_handle_unchecked(slot, value)
    } else {
      self.create_handle_unchecked(result, value)
    }
  }

  /// 原 HandleScope::CreateHandleUnchecked: 写入并推进 next
  fn create_handle_unchecked(&mut self, result: SlotPtr, value: Address) -> SlotPtr {
    debug_assert!(result < self.data.limit);
    // SAFETY: result < limit 且指向本 Arena 持有块内的空槽
    unsafe {
      *result = value;
      self.data.next = result.add(1);
    }
    result
  }

  /// 原 HandleScope::CloseScope (handles-inl.h:222): 回退 next/limit 至先前状态, level--
  fn close_scope(&mut self, prev_next: SlotPtr, prev_limit: SlotPtr) {
    // 原 std::swap(current->next, prev_next): 留存关闭时以关闭时刻 next 作 zap 上界
    #[cfg(debug_assertions)]
    let next_at_close = self.data.next;
    self.data.next = prev_next;
    self.data.level -= 1;
    if self.data.limit != prev_limit {
      self.data.limit = prev_limit;
      self.delete_extensions();
      // limit 回退: zap 上界取回退后的 prev_limit
      #[cfg(debug_assertions)]
      self.zap_range(self.data.next, prev_limit);
    } else {
      #[cfg(debug_assertions)]
      self.zap_range(self.data.next, next_at_close);
    }
  }

  /// 原 HandleScopeImplementer::DeleteExtensions (handle-scope-implementer-inl.h:63):
  /// 回收 prev_limit 之后的扩展块进 spare, break/pop 两分支均 zap 回收区
  fn delete_extensions(&mut self) {
    let prev_limit = self.data.limit;
    while let Some(&back) = self.blocks.last() {
      // SAFETY: 块首 + 容量得块尾
      let block_limit = unsafe { back.as_ptr().add(K_HANDLE_BLOCK_SIZE) };
      // SealHandleScope 可能使 prev_limit 指向块内 (原注释) —— 用地址比较规避无关指针比较 UB
      if (back.as_ptr() as usize) < prev_limit as usize
        && prev_limit as usize <= block_limit as usize
      {
        #[cfg(debug_assertions)]
        self.zap_range(prev_limit, block_limit);
        break;
      }
      self.blocks.pop();
      #[cfg(debug_assertions)]
      self.zap_range(back.as_ptr(), block_limit);
      // 原 ReturnBlock: 块归还 spare, 旧 spare 释放
      // SAFETY: 块首来自 Box::into_raw, 此刻 Arena 唯一持有
      let block = unsafe { Box::from_raw(back.as_ptr() as *mut Block) };
      let _ = self.spare.replace(block);
    }
  }

  /// 原 HandleScope::NumberOfHandles: (块数-1)*块大小 + 末块已用槽数
  fn number_of_handles(&self) -> usize {
    let Some((&back, _)) = self.blocks.split_last() else {
      return 0;
    };
    (self.blocks.len() - 1) * K_HANDLE_BLOCK_SIZE
      + (self.data.next as usize - back.as_ptr() as usize) / size_of::<Address>()
  }

  /// 原 HandleScopeImplementer::IterateThis (handle-scope-implementer.cc:76):
  /// 逆序遍历除末块外全部块 —— persistent 分界块截至分界点 (闭区间判定, 块尾也算),
  /// 其余整块; 末块存活句柄截至 data.next
  pub(crate) fn iterate_this(&self, mut visit: impl FnMut(*const Address, *const Address)) {
    let Some((&back, rest)) = self.blocks.split_last() else {
      return;
    };
    let mut found_block_before_persistent = false;
    for &block in rest.iter().rev() {
      let start = block.as_ptr();
      // SAFETY: 块首 + 容量 K_HANDLE_BLOCK_SIZE
      let end = unsafe { start.add(K_HANDLE_BLOCK_SIZE) };
      // 分界块闭区间判定 (含块尾): lhb <= 块尾 && lhb >= 块首 —— 地址比较规避无关指针比较 UB
      let boundary = self.last_handle_before_persistent_block.filter(|&lhb| {
        !lhb.is_null() && (lhb as usize) <= (end as usize) && (lhb as usize) >= (start as usize)
      });
      if let Some(lhb) = boundary {
        visit(start, lhb);
        debug_assert!(!found_block_before_persistent);
        found_block_before_persistent = true;
      } else {
        visit(start, end);
      }
    }
    // 原版 DCHECK: persistent 作用域开启且分界指针非空时, 分界块必在遍历中命中
    debug_assert_eq!(
      self.has_persistent_scope()
        && self
          .last_handle_before_persistent_block
          .is_some_and(|p| !p.is_null()),
      found_block_before_persistent
    );
    // 末块存活句柄截至 next (可能不足整块)
    visit(back.as_ptr(), self.data.next);
  }

  /// 原 HandleScope::ZapRange (ENABLE_LOCAL_HANDLE_ZAPPING, debug 构建启用):
  /// 半开区间 [start, end) 填充 zap 值
  #[cfg(debug_assertions)]
  fn zap_range(&self, start: SlotPtr, end: SlotPtr) {
    debug_assert!((end as usize - start as usize) / size_of::<Address>() <= K_HANDLE_BLOCK_SIZE);
    zap_range(start, end, K_HANDLE_ZAP_VALUE);
  }

  // ---- persistent 作用域支撑 (原 HandleScopeImplementer 私有面, 见 persistent_handles.rs) ----

  /// 原 BeginPersistentScope: 记录 persistent 块之前的最后句柄位置
  pub(crate) fn begin_persistent_scope(&mut self) {
    debug_assert!(self.last_handle_before_persistent_block.is_none());
    self.last_handle_before_persistent_block = Some(self.data.next);
  }

  /// 原 HasPersistentScope
  pub(crate) fn has_persistent_scope(&self) -> bool {
    self.last_handle_before_persistent_block.is_some()
  }

  /// 取新块并入 blocks (原 PersistentHandlesScope 构造内 GetSpareOrNewBlock + push_back),
  /// 返回 (块首, 块尾)
  pub(crate) fn push_persistent_block(&mut self) -> (SlotPtr, SlotPtr) {
    let block = self.get_spare_or_new_block();
    self.blocks.push(block);
    let start = block.as_ptr();
    // SAFETY: 块首 + 容量 K_HANDLE_BLOCK_SIZE
    let limit = unsafe { start.add(K_HANDLE_BLOCK_SIZE) };
    (start, limit)
  }

  /// 原 HandleScopeImplementer::DetachPersistent: 移出自 first_block 起的全部块 (逆序弹出,
  /// 首尾对调使末块为可能半满的块), 返回 (块集, 沿用 block_next)
  pub(crate) fn take_persistent_blocks(
    &mut self,
    first_block: SlotPtr,
  ) -> (Vec<NonNull<Address>>, SlotPtr) {
    debug_assert!(self.has_persistent_scope());
    let block_next = self.data.next;
    let mut taken = Vec::new();
    while let Some(back) = self.blocks.pop() {
      taken.push(back);
      if back.as_ptr() == first_block {
        break;
      }
    }
    debug_assert_eq!(taken.last().map(|b| b.as_ptr()), Some(first_block));
    // 首尾对调: taken 弹出序为 [末块(可能半满), ..., first_block], 对调后
    // blocks 尾为 first_block 满块, 半满块居首 —— 原注释语义
    let n = taken.len();
    if n > 1 {
      taken.swap(0, n - 1);
    }
    self.last_handle_before_persistent_block = None;
    (taken, block_next)
  }
}

thread_local! {
  /// 当前线程句柄竞技场 (模拟 isolate 的 handle_scope_data + implementer 分块, 线程即隔离边界)
  static ARENA: RefCell<HandleArena> = RefCell::new(HandleArena::new());
}

/// debug 构建句柄回收区填充内核 (原 ZapRange 族: arena / local / persistent 共用)
#[cfg(debug_assertions)]
pub(crate) fn zap_range(start: SlotPtr, end: SlotPtr, value: Address) {
  let mut p = start;
  while p != end {
    // SAFETY: p 遍历 [start, end) 均在所属容器持有块内
    unsafe {
      *p = value;
      p = p.add(1);
    }
  }
}

/// 独占借用当前线程竞技场 (对应原版 isolate->handle_scope_data() 的当前线程绑定)
pub(crate) fn with_arena<R>(f: impl FnOnce(&mut HandleArena) -> R) -> R {
  ARENA.with(|a| f(&mut a.borrow_mut()))
}

// ----------------------------------------------------------------------------
// HandleScope (原 V8_NODISCARD HandleScope)

/// 栈分配的局部句柄作用域: 构造即 Enter, Drop 即 Leave; 嵌套时内层分配,
/// 关闭后 next/limit 回退, 外层从回退点继续 (原类注释 1:1)
#[must_use]
pub struct HandleScope {
  prev_next: SlotPtr,
  prev_limit: SlotPtr,
}

impl HandleScope {
  /// 进入作用域 (原 HandleScope(Isolate*)): 保存 prev_next/prev_limit, level++
  pub fn new() -> Self {
    let mut scope = Self {
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

  /// 当前已分配句柄数 (原 static NumberOfHandles(Isolate*))
  pub fn number_of_handles() -> usize {
    with_arena(|a| a.number_of_handles())
  }

  /// 遍历当前线程全部存活句柄槽区间 [start, end) (原 implementer->Iterate(RootVisitor*)
  /// 落到 IterateThis 的线程本位收窄; RootVisitor 归 heap 层, 以槽区间回调暴露)
  pub fn iterate_handles(visit: impl FnMut(*const Address, *const Address)) {
    with_arena(|a| a.iterate_this(visit));
  }

  /// 创建槽位并写入 value (原 private static CreateHandle)
  pub(crate) fn create_handle(value: Address) -> SlotPtr {
    with_arena(|a| a.create_handle(value))
  }

  /// 关闭当前作用域 (句柄全部失效), 在父作用域为参数值重分配一个句柄并返回
  /// (原 CloseAndEscape)
  pub fn close_and_escape<T>(&mut self, handle_value: Handle<T>) -> Handle<T> {
    // 空句柄安全路径 (原版 *handle_value 解引用 null 属 UB, 此处收窄):
    // 仅关闭并重开作用域, 不在父作用域分配
    if handle_value.is_null() {
      with_arena(|a| a.close_scope(self.prev_next, self.prev_limit));
      self.reopen();
      return Handle::null();
    }
    // 先取值再关闭 (原 Tagged<T> value = *handle_value): close 后槽位进入回收区, 防读悬垂
    let value = handle_value.value().ptr();
    // 抛弃当前作用域内全部句柄
    with_arena(|a| {
      debug_assert!(a.data.level > a.data.sealed_level);
      a.close_scope(self.prev_next, self.prev_limit)
    });
    // 在父作用域分配一个句柄
    let location = Self::create_handle(value);
    // 重新初始化当前作用域, 备再次关闭或继续使用 (原注释)
    self.reopen();
    Handle::from_location(location)
  }

  /// 重新初始化当前作用域 (原 CloseAndEscape 尾部惯用法):
  /// 以当前 next/limit 为新 prev, level++
  fn reopen(&mut self) {
    with_arena(|a| {
      self.prev_next = a.data.next;
      self.prev_limit = a.data.limit;
      a.data.level += 1;
    });
  }

  /// 关闭作用域, limit 回退到先前状态 (原 private static CloseScope)
  pub(crate) fn close_scope(prev_next: SlotPtr, prev_limit: SlotPtr) {
    with_arena(|a| a.close_scope(prev_next, prev_limit));
  }
}

impl Default for HandleScope {
  fn default() -> Self {
    Self::new()
  }
}

impl Drop for HandleScope {
  fn drop(&mut self) {
    Self::close_scope(self.prev_next, self.prev_limit);
  }
}

// ----------------------------------------------------------------------------
// SealHandleScope (原 debug-only 实现, 保留 debug 语义, 类型常开)

/// 封印当前 HandleScope: 收紧 limit 至 next, 使新句柄只能在新作用域中创建 (原注释)
pub struct SealHandleScope {
  prev_limit: SlotPtr,
  prev_sealed_level: i32,
}

impl SealHandleScope {
  /// 封印 (原 SealHandleScope(Isolate*) debug 分支)
  pub fn new() -> Self {
    with_arena(|a| {
      let scope = Self {
        prev_limit: a.data.limit,
        prev_sealed_level: a.data.sealed_level,
      };
      // 收紧作用域使无显式 HandleScope 时无法分配 (原注释)
      a.data.limit = a.data.next;
      a.data.sealed_level = a.data.level;
      scope
    })
  }
}

impl Default for SealHandleScope {
  fn default() -> Self {
    Self::new()
  }
}

impl Drop for SealHandleScope {
  fn drop(&mut self) {
    // 恢复状态以重新允许句柄分配 (原注释)
    with_arena(|a| {
      debug_assert_eq!(a.data.next, a.data.limit);
      a.data.limit = self.prev_limit;
      debug_assert_eq!(a.data.level, a.data.sealed_level);
      a.data.sealed_level = self.prev_sealed_level;
    });
  }
}

// ----------------------------------------------------------------------------
// Handle<T> (原 template<class T> class Handle final : HandleBase)

/// 对象引用的间接层包装: GC 搬迁下保持有效; 仅在 HandleScope 存活期内有效 (原类注释)。
/// 刻意不提供默认相等/散列 —— location 与对象身份语义有歧义,
/// 显式用 equals (location) / is_identical_to (对象身份) (原注释)。
/// 手动 impl Copy/Clone: derive 会强加 `T: Copy` bound, 幻类型参数不参与复制
pub struct Handle<T = Object> {
  location: SlotPtr,
  _m: PhantomData<fn() -> T>,
}

impl<T> Clone for Handle<T> {
  fn clone(&self) -> Self {
    *self
  }
}
impl<T> Copy for Handle<T> {}

impl<T> Handle<T> {
  /// 空句柄 (原 Handle() / Handle<T>::null())
  pub const fn null() -> Self {
    Self {
      location: null_mut(),
      _m: PhantomData,
    }
  }

  /// 由槽位构造 (原 explicit Handle(Address*))
  pub const fn from_location(location: SlotPtr) -> Self {
    Self {
      location,
      _m: PhantomData,
    }
  }

  /// 为对象分配新句柄, 不做规范化 (原 static New(Tagged<T>, Isolate*));
  /// scope 参数证明作用域存活 (Isolate 推迟的惯用法映射)
  pub fn new(value: Tagged<T>, _scope: &HandleScope) -> Self {
    Self::from_location(HandleScope::create_handle(value.ptr()))
  }

  /// 指向槽位的原始地址, 仅供散列, 禁解引用 (原 address());
  /// 指针转整数不可在 const eval 展开, 保持运行时函数
  pub fn address(&self) -> Address {
    self.location as Address
  }

  /// 槽位指针 (原 location())
  pub const fn location(&self) -> SlotPtr {
    self.location
  }

  /// 是否空句柄 (原 is_null())
  pub fn is_null(&self) -> bool {
    self.location.is_null()
  }

  /// 是否与另一句柄指向同一对象: 同槽或槽值相等 (原 is_identical_to(HandleBase&))
  pub fn is_identical_to(&self, that: &Self) -> bool {
    if self.location == that.location {
      return true;
    }
    if self.location.is_null() || that.location.is_null() {
      return false;
    }
    // SAFETY: 双方非空且在作用域存活约定内
    unsafe { *self.location == *that.location }
  }

  /// location 相等 (原 equals)
  pub fn equals(&self, other: &Self) -> bool {
    self.address() == other.address()
  }

  /// 读句柄指向的值 (原 operator*): 双重解引用的第一重
  pub fn value(&self) -> Tagged<T> {
    debug_assert!(!self.location.is_null());
    // SAFETY: 非空槽位, 作用域存活约定内
    Tagged::from_ptr(unsafe { *self.location })
  }

  /// 原地改写句柄值, 同 location 的所有间接句柄可见 (原 PatchValue)
  pub fn patch_value(&self, new_value: Tagged<T>) {
    debug_assert!(!self.location.is_null());
    // SAFETY: 非空槽位, 作用域存活约定内
    unsafe { *self.location = new_value.ptr() }
  }

  /// 无检查类型换型, 复用同一 location (原 UncheckedCast<To>(IndirectHandle<From>))
  pub fn unchecked_cast<U>(self) -> Handle<U> {
    Handle::from_location(self.location)
  }

  /// 上转到顶层对象句柄 (原 is_subtype_v<S, Object> 的隐式上转惯用法)
  pub fn as_object(self) -> Handle<Object> {
    self.unchecked_cast()
  }
}

impl<T> Default for Handle<T> {
  fn default() -> Self {
    Self::null()
  }
}

/// 原型别名: 非 direct 配置下 IndirectHandle 即 Handle
pub type IndirectHandle<T = Object> = Handle<T>;

// ----------------------------------------------------------------------------
// DirectHandle<T> (原 !V8_ENABLE_DIRECT_HANDLE 分支: Handle 的包装)

/// 无 (用户可见) 中间层的对象引用 —— 保守栈扫描关闭时是 IndirectHandle 的包装,
/// 可由 Handle 隐式构造, 不隐式转回 (原类注释)。
/// 手动 impl Copy/Clone/Default: derive 会强加 `T: Copy`/`T: Default` bound
pub struct DirectHandle<T = Object> {
  handle: Handle<T>,
}

impl<T> Clone for DirectHandle<T> {
  fn clone(&self) -> Self {
    *self
  }
}
impl<T> Copy for DirectHandle<T> {}

impl<T> DirectHandle<T> {
  /// 空直连句柄 (原 static null())
  pub const fn null() -> Self {
    Self {
      handle: Handle::null(),
    }
  }

  /// 由地址分配槽位构造 (原 static FromAddress(Isolate*, Address));
  /// scope 参数证明作用域存活 (Isolate 推迟的惯用法映射)
  pub fn from_address(object: Address, _scope: &HandleScope) -> Self {
    Self {
      handle: Handle::from_location(HandleScope::create_handle(object)),
    }
  }

  /// 由槽位构造 (原 static FromSlot, 非 direct 分支: IndirectHandle<T>(slot))
  pub const fn from_slot(slot: SlotPtr) -> Self {
    Self {
      handle: Handle::from_location(slot),
    }
  }

  /// 分配新直连句柄 (原 static New(Tagged<T>, Isolate*))
  pub fn new(value: Tagged<T>, scope: &HandleScope) -> Self {
    Self {
      handle: Handle::new(value, scope),
    }
  }

  /// 由间接句柄构造 (原 DirectHandle(IndirectHandle<S>) 上转)
  pub const fn from_handle(handle: Handle<T>) -> Self {
    Self { handle }
  }

  /// 底层间接句柄 (原 indirect_handle(DirectHandle) 自由函数)
  pub const fn indirect(self) -> Handle<T> {
    self.handle
  }

  /// 是否空 (原 is_null())
  pub fn is_null(&self) -> bool {
    self.handle.is_null()
  }

  /// 槽位原始地址 (原 address())
  pub fn address(&self) -> Address {
    self.handle.address()
  }

  /// 读值 (原 operator*)
  pub fn value(&self) -> Tagged<T> {
    self.handle.value()
  }

  /// location 相等 (原 equals)
  pub fn equals(&self, other: &Self) -> bool {
    self.handle.equals(&other.handle)
  }

  /// 对象身份相等 (原 is_identical_to)
  pub fn is_identical_to(&self, other: &Self) -> bool {
    self.handle.is_identical_to(&other.handle)
  }

  /// 改写值: 效果等同句柄赋值, 经底层 PatchValue 实现 (原 SetValue 注释)
  pub fn set_value(&mut self, new_value: Tagged<T>) {
    self.handle.patch_value(new_value);
  }

  /// 无检查类型换型 (原 UncheckedCast)
  pub fn unchecked_cast<U>(self) -> DirectHandle<U> {
    DirectHandle {
      handle: self.handle.unchecked_cast(),
    }
  }
}

impl<T> Default for DirectHandle<T> {
  fn default() -> Self {
    Self::null()
  }
}

// ----------------------------------------------------------------------------
// 离栈容器 (原 DirectHandleVector / DirectHandleSmallVector, 非 direct 配置即普通 vector)

/// DirectHandle 的 vector 容器 (原 template<class T> class DirectHandleVector)
pub struct DirectHandleVector<T> {
  backing: Vec<DirectHandle<T>>,
}

impl<T> DirectHandleVector<T> {
  /// 原构造 (Isolate* 参数在此配置下无作用)
  pub const fn new() -> Self {
    Self {
      backing: Vec::new(),
    }
  }

  /// 指定容量构造 (原 DirectHandleVector(Isolate*, size_t n))
  pub fn with_len(n: usize) -> Self {
    Self {
      backing: vec![DirectHandle::null(); n],
    }
  }

  /// 元素数 (原 size())
  pub fn len(&self) -> usize {
    self.backing.len()
  }

  /// 是否为空 (原 empty())
  pub fn is_empty(&self) -> bool {
    self.backing.is_empty()
  }

  /// 预留容量 (原 reserve)
  pub fn reserve(&mut self, n: usize) {
    self.backing.reserve(n);
  }

  /// 收缩容量 (原 shrink_to_fit)
  pub fn shrink_to_fit(&mut self) {
    self.backing.shrink_to_fit();
  }

  /// 索引访问 (原 operator[])
  pub fn get(&self, n: usize) -> Option<&DirectHandle<T>> {
    self.backing.get(n)
  }

  /// 尾元素 (原 back())
  pub fn back(&self) -> Option<&DirectHandle<T>> {
    self.backing.last()
  }

  /// 首元素 (原 front())
  pub fn front(&self) -> Option<&DirectHandle<T>> {
    self.backing.first()
  }

  /// 追加 (原 push_back)
  pub fn push(&mut self, x: DirectHandle<T>) {
    self.backing.push(x);
  }

  /// 弹出 (原 pop_back)
  pub fn pop(&mut self) -> Option<DirectHandle<T>> {
    self.backing.pop()
  }

  /// 清空 (原 clear)
  pub fn clear(&mut self) {
    self.backing.clear();
  }

  /// 重设尺寸 (原 resize)
  pub fn resize(&mut self, n: usize) {
    self.backing.resize(n, DirectHandle::null());
  }

  /// 交换 (原 swap: vector 内容整体交换)
  pub fn swap(&mut self, other: &mut Self) {
    swap(&mut self.backing, &mut other.backing);
  }

  /// 底层切片
  pub fn as_slice(&self) -> &[DirectHandle<T>] {
    &self.backing
  }
}

impl<T> Default for DirectHandleVector<T> {
  fn default() -> Self {
    Self::new()
  }
}

/// DirectHandle 的小向量容器 (原 template<class T, size_t kSize> DirectHandleSmallVector)
pub struct DirectHandleSmallVector<T, const N: usize> {
  backing: SmallVector<DirectHandle<T>, N>,
}

impl<T, const N: usize> DirectHandleSmallVector<T, N> {
  /// 内联槽位数 (原 kInlineSize)
  pub const K_INLINE_SIZE: usize = N;

  /// 原构造 (Isolate* 参数在此配置下无作用)
  pub fn new() -> Self {
    Self {
      backing: SmallVector::new(),
    }
  }

  /// 元素数 (原 size())
  pub fn len(&self) -> usize {
    self.backing.size()
  }

  /// 是否为空 (原 empty())
  pub fn is_empty(&self) -> bool {
    self.backing.is_empty()
  }

  /// 尾元素 (原 back())
  pub fn back(&self) -> Option<&DirectHandle<T>> {
    self.backing.back()
  }

  /// 首元素 (原 front())
  pub fn front(&self) -> Option<&DirectHandle<T>> {
    self.backing.front()
  }

  /// 索引访问 (原 operator[])
  pub fn get(&self, n: usize) -> Option<&DirectHandle<T>> {
    self.backing.at(n)
  }

  /// 追加 (原 push_back)
  pub fn push(&mut self, x: DirectHandle<T>) {
    self.backing.push_back(x);
  }

  /// 弹出 count 个 (原 pop_back(size_t count = 1))
  pub fn pop(&mut self, count: usize) {
    self.backing.pop_back(count);
  }

  /// 追加切片 (原 insert(first, last) 惯用法)
  pub fn extend_from_slice(&mut self, src: &[DirectHandle<T>]) {
    self.backing.extend_from_slice(src);
  }

  /// 清空 (原 clear)
  pub fn clear(&mut self) {
    self.backing.clear();
  }

  /// 重设尺寸 (原 resize)
  pub fn resize(&mut self, new_size: usize) {
    self.backing.resize(new_size);
  }

  /// 底层切片
  pub fn as_slice(&self) -> &[DirectHandle<T>] {
    self.backing.as_slice()
  }
}

impl<T, const N: usize> Default for DirectHandleSmallVector<T, N> {
  fn default() -> Self {
    Self::new()
  }
}
