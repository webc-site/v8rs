// v8-regexp-exec: RegExpStack — v8/src/regexp/regexp-stack.{h,cc} 1:1
// 由 tool/src/regexp/regexp-stack.h.js / regexp-stack.cc.js 对数审计
// (骨架: regexp_stack.rs.skel; 本文件为 Developer 就地修订版, 复跑脚本仅重生成骨架)
//
// 原版语义: per-Isolate 的 irregexp 回溯栈。线程安全结构原版无锁
// (regexp-stack.{h,cc} 实测 Mutex/条件变量 0 处): 类型 !Send + !Sync
// (ThreadLocal 含裸指针), 线程切换必须显式走 archive_stack/restore_stack 搬迁。
// 沙箱硬件分支 (V8_ENABLE_SANDBOX_HARDWARE_SUPPORT) 按默认配置展开排除, 见票单。

use std::{
  cell::UnsafeCell,
  mem::{offset_of, size_of},
  ptr::{copy, copy_nonoverlapping, from_ref, null_mut, slice_from_raw_parts_mut},
};

use v8_base::logging::{check_op_eq, v8_fatal};
use v8_common::{Address, K_INT32_SIZE, K_NULL_ADDRESS, KB, MB};

/// 原版 kSlotSize (h:57-61): PPC64/S390X 分支取 kSystemPointerSize,
/// 其余 (x64/arm64 默认配置) 取 kInt32Size — 本仓先展开默认分支
pub const K_SLOT_SIZE: usize = K_INT32_SIZE;
/// 原版 kStackLimitSlackSlotCount (h:64) — limit 之下可连续 push 的槽数
pub const K_STACK_LIMIT_SLACK_SLOT_COUNT: usize = 32;
/// 原版 kStackLimitSlackSize (h:65-66)
pub const K_STACK_LIMIT_SLACK_SIZE: usize = K_STACK_LIMIT_SLACK_SLOT_COUNT * K_SLOT_SIZE;
/// 原版 kMaximumStackSize (h:98) = 64*MB — EnsureCapacity 上限。
/// 实测上游 v8 无 regexp_max_stack_size flag (flag-definitions.h 0 处), 上限即该常量;
/// 超限返回 K_NULL_ADDRESS 的语义 1:1 (见票单申报)
pub const K_MAXIMUM_STACK_SIZE: usize = 64 * MB;
/// 原版 kStaticStackSize (h:114) — 静态预留区, 保证栈恒可用、免可用性检查
const K_STATIC_STACK_SIZE: usize = KB;
/// 原版 kMinimumDynamicStackSize (h:126) — 动态栈最小值
const K_MINIMUM_DYNAMIC_STACK_SIZE: usize = 2 * KB;
/// 原版 kMemoryTop (h:107-108): (Address)(uintptr_t)-1 — 线程局部失效后的伪 limit
const K_MEMORY_TOP: Address = usize::MAX;

// 原版 static_assert ×3 (h:118/119/127) → 编译期 const assert
const _: () = assert!(K_STATIC_STACK_SIZE >= 2 * K_STACK_LIMIT_SLACK_SIZE);
const _: () = assert!(K_STATIC_STACK_SIZE <= K_MAXIMUM_STACK_SIZE);
const _: () = assert!(K_MINIMUM_DYNAMIC_STACK_SIZE == 2 * K_STATIC_STACK_SIZE);

/// 原版 Stack::ThreadLocal (h:131-160) — 动态内存/大小/limit 快照,
/// 线程切换时整体归档/恢复 (即原版 memcpy 的对象)。
/// repr(C) 固定字段序, 归档逐字节搬迁与 offset 常量对数依赖此布局。
/// 不变式 (h:136-139): memory_size_>0 时 memory_/memory_top_/stack_pointer_ 非空,
/// memory_top_ = memory_ + memory_size_, memory_ <= stack_pointer_ <= memory_top_。
#[repr(C)]
#[derive(Clone, Copy)]
struct ThreadLocal {
  /// 栈区首 (静态区或动态缓冲)
  memory_: *mut u8,
  /// 栈区顶
  memory_top_: *mut u8,
  /// 栈区大小 (字节)
  memory_size_: usize,
  /// 栈指针 (原版由生成码直写; Rust 侧代码生成面经 thread_local_address+offset 读写)
  stack_pointer_: *mut u8,
  /// 增长检查 limit (低于此须先扩容)
  limit_: Address,
  /// memory_ 是否为持有的动态缓冲 (须释放)
  owns_memory_: bool,
}

/// 原版 kThreadLocalSize (h:161) = sizeof(ThreadLocal) — 归档块大小
pub const K_THREAD_LOCAL_SIZE: usize = size_of::<ThreadLocal>();
/// 原版 kMemoryTopOffset (h:165) — 生成码访存偏移 (代码生成面消费)
pub const K_MEMORY_TOP_OFFSET: usize = offset_of!(ThreadLocal, memory_top_);
/// 原版 kStackPointerOffset (h:166-167)
pub const K_STACK_POINTER_OFFSET: usize = offset_of!(ThreadLocal, stack_pointer_);
/// 原版 kLimitOffset (h:172)
pub const K_LIMIT_OFFSET: usize = offset_of!(ThreadLocal, limit_);

impl ThreadLocal {
  /// 原版 h:140-148 字段默认值 (构造缺省态, limit_ = kNullAddress)
  const INIT: Self = Self {
    memory_: null_mut(),
    memory_top_: null_mut(),
    memory_size_: 0,
    stack_pointer_: null_mut(),
    limit_: K_NULL_ADDRESS,
    owns_memory_: false,
  };

  /// 原版 ThreadLocal::ResetToStaticStack (cc:82-92) —
  /// 释放动态栈并回落到 Stack 拥有的静态预留区
  fn reset_to_static_stack(&mut self, static_stack: *mut u8) {
    self.delete_dynamic_stack();
    self.memory_ = static_stack;
    // SAFETY: static_stack 由 Box<[u8; K_STATIC_STACK_SIZE]> 提供, 其后必有整区字节
    self.memory_top_ = unsafe { static_stack.add(K_STATIC_STACK_SIZE) };
    self.memory_size_ = K_STATIC_STACK_SIZE;
    self.stack_pointer_ = self.memory_top_;
    self.limit_ = (static_stack as Address) + K_STACK_LIMIT_SLACK_SIZE;
    self.owns_memory_ = false;
  }

  /// 原版 ThreadLocal::FreeAndInvalidate (cc:94-104) —
  /// 释放后写入失效值, 防误用旧内存区 (limit 取伪顶 kMemoryTop)
  fn free_and_invalidate(&mut self) {
    self.delete_dynamic_stack();
    self.memory_ = null_mut();
    self.memory_top_ = null_mut();
    self.memory_size_ = 0;
    self.stack_pointer_ = null_mut();
    self.limit_ = K_MEMORY_TOP;
  }

  /// 原版 ThreadLocal::NewDynamicStack (cc:107-127) —
  /// 默认 (非沙箱硬件) 分支 NewArray<uint8_t>(size): 精确 size 字节。
  /// 原版 release 配置不初始化, Rust 侧零初始化 (语义超集, 见票单),
  /// Box<[u8]> 定长分配, 归还按 memory_size_ 重建 (delete_dynamic_stack)
  fn new_dynamic_stack(size: usize) -> *mut u8 {
    let buf: Box<[u8]> = vec![0u8; size].into_boxed_slice();
    Box::into_raw(buf).cast::<u8>()
  }

  /// 原版 ThreadLocal::DeleteDynamicStack (cc:129-140)
  fn delete_dynamic_stack(&mut self) {
    if self.owns_memory_ {
      // SAFETY: owns_memory_ ⇔ memory_/memory_size_ 恰为 new_dynamic_stack 交还的
      // Box<[u8]> 裸指针与长度且未归还过; 立即复位 owns_memory_ 防双释放
      unsafe {
        drop(Box::<[u8]>::from_raw(slice_from_raw_parts_mut(
          self.memory_,
          self.memory_size_,
        )));
      }
      self.owns_memory_ = false;
    }
  }

  /// 原版 ThreadLocal::ResetToStaticStackIfEmpty (h:151-153) — 空栈 (sp==顶) 时回落
  fn reset_to_static_stack_if_empty(&mut self, static_stack: *mut u8) {
    if self.stack_pointer_ == self.memory_top_ {
      self.reset_to_static_stack(static_stack);
    }
  }
}

/// 原版 regexp::Stack (h:49-208) — irregexp 回溯栈内存 (线程亲和)。
///
/// 静态预留区经 Box 定址, 故 Stack 值可安全 move (原版对象本身禁拷贝/赋值,
/// h:51-52 的 deleted copy 由 Rust 移动语义 + Drop 承接)。
/// ThreadLocal 含裸指针 → 类型天然 !Send + !Sync, 线程切换必须显式
/// archive_stack/restore_stack 交接所有权。
pub struct Stack {
  /// 原版 static_stack_ (h:123) — 无动态栈时使用的静态预留区
  static_stack_: Box<UnsafeCell<[u8; K_STATIC_STACK_SIZE]>>,
  thread_local_: UnsafeCell<ThreadLocal>,
}

impl Stack {
  /// 原版 Stack::New (cc:29-46): 默认配置即 `new Stack()` (沙箱硬件页分配分支排除,
  /// 见票单); 原版 Stack::Delete (cc:48-60) 由 Drop + Box 释放承接
  pub fn new() -> Stack {
    let static_stack = Box::new(UnsafeCell::new([0u8; K_STATIC_STACK_SIZE]));
    let stack = Stack {
      static_stack_: static_stack,
      thread_local_: UnsafeCell::new(ThreadLocal::INIT),
    };
    let ss = stack.static_stack_ptr();
    // SAFETY: 刚构造, INIT 缺省态无持有内存 (owns_memory_=false, 回落不触发释放);
    // ss 指向 stack.static_stack_ 拥有的 Box 堆块
    unsafe { (*stack.thread_local_.get()).reset_to_static_stack(ss) };
    stack
  }

  /// 读 ThreadLocal 快照 (Copy 值, 不留引用 — UnsafeCell 读取纪律)
  /// SAFETY 契约: 本类型 !Send/!Sync, &self 即线程独占视图, 读与短可变借用不并发
  #[inline]
  fn tl(&self) -> ThreadLocal {
    // SAFETY: 同上; 快照按值复制, 字段皆为指针/整数值位模式
    unsafe { *self.thread_local_.get() }
  }

  /// 静态预留区首地址
  #[inline]
  fn static_stack_ptr(&self) -> *mut u8 {
    // UnsafeCell::get 仅派生指针; Box 堆块在 Stack 生命周期内地址稳定
    self.static_stack_.get().cast::<u8>()
  }

  /// 原版 begin (h:68-70) — 栈区首地址
  #[inline]
  pub fn begin(&self) -> Address {
    self.tl().memory_ as Address
  }

  /// 原版 end (h:71-76) — 栈区顶地址 (含不变式 DCHECK ×2)
  pub fn end(&self) -> Address {
    let tl = self.tl();
    debug_assert_ne!(0, tl.memory_size_);
    debug_assert_eq!(
      tl.memory_top_ as Address,
      tl.memory_ as Address + tl.memory_size_
    );
    tl.memory_top_ as Address
  }

  /// 原版 memory_top (h:77) = end()
  #[inline]
  pub fn memory_top(&self) -> Address {
    self.end()
  }

  /// 原版 stack_pointer (h:79-81)
  #[inline]
  pub fn stack_pointer(&self) -> Address {
    self.tl().stack_pointer_ as Address
  }

  /// 原版 memory_size (h:83)
  #[inline]
  pub fn memory_size(&self) -> usize {
    self.tl().memory_size_
  }

  /// 原版 EnsureCapacity (cc:142-163) — 预留至少 size 字节并返回新栈顶;
  /// size=0 取默认/最小缓冲; 超 K_MAXIMUM_STACK_SIZE 返回 K_NULL_ADDRESS。
  /// 复用规则 1:1: 旧内容整体拷入新缓冲顶部 (栈自顶向下生长, 保 sp_top_delta 连续),
  /// limit 重置为新基址 + slack
  pub fn ensure_capacity(&self, size: usize) -> Address {
    if size > K_MAXIMUM_STACK_SIZE {
      return K_NULL_ADDRESS;
    }
    // SAFETY: 线程亲和 &self 即独占可变视图 (等价原版 Stack* 直写字段);
    // 可变借用仅限本块, 已交出的 Address 为 usize 值非引用
    unsafe {
      let tl = &mut *self.thread_local_.get();
      if tl.memory_size_ < size {
        let new_size = size.max(K_MINIMUM_DYNAMIC_STACK_SIZE);
        let new_memory = ThreadLocal::new_dynamic_stack(new_size);
        if tl.memory_size_ > 0 {
          // 原版 cc:149-150: MemCopy(new + size - old, old, old)
          copy(
            tl.memory_,
            new_memory.add(new_size - tl.memory_size_),
            tl.memory_size_,
          );
          tl.delete_dynamic_stack();
        }
        // 原版 cc:153: delta = sp_top_delta() (拷贝/释放之后、字段改写之前)
        let delta = tl.stack_pointer_.offset_from(tl.memory_top_);
        tl.memory_ = new_memory;
        tl.memory_top_ = new_memory.add(new_size);
        tl.memory_size_ = new_size;
        tl.stack_pointer_ = tl.memory_top_.offset(delta);
        tl.limit_ = (new_memory as Address) + K_STACK_LIMIT_SLACK_SIZE;
        tl.owns_memory_ = true;
      }
      tl.memory_top_ as Address
    }
  }

  /// 原版 ArchiveSpacePerThread (h:90-92) — 每线程归档字节数
  #[inline]
  pub const fn archive_space_per_thread() -> i32 {
    K_THREAD_LOCAL_SIZE as i32
  }

  /// 原版 Stack::ArchiveStack (cc:62-75) — 归档 ThreadLocal 块并回落静态区。
  /// 归档前强制转动态栈 (cc:63-70): 静态区随 Stack 实例搬迁, 归档/恢复依赖
  /// memory_ 指向定址后备缓冲。
  ///
  /// # Safety
  /// `to` 须指向 ≥ `K_THREAD_LOCAL_SIZE` 字节的合法可写区且与本栈内存不重叠
  /// (归档缓冲由线程切换方持有 — 原版 char* 契约的 Rust 等价)。
  /// 返回归档区末尾 (to + K_THREAD_LOCAL_SIZE)。
  pub unsafe fn archive_stack(&self, to: *mut u8) -> *mut u8 {
    if !self.tl().owns_memory_ {
      let need = self.tl().memory_size_ + 1;
      self.ensure_capacity(need);
      debug_assert!(self.tl().owns_memory_);
    }
    let tl = self.tl();
    // SAFETY: 契约保证 to 区大小与不重叠; 源为栈上快照 (与 thread_local_ 同字节)
    unsafe {
      copy_nonoverlapping(from_ref(&tl).cast::<u8>(), to, K_THREAD_LOCAL_SIZE);
    }
    let ss = self.static_stack_ptr();
    // SAFETY: 同 tl 读写纪律 (线程亲和短借用)
    unsafe {
      // 原版 cc:73 `thread_local_ = ThreadLocal(this)`: 经临时对象构造 (缺省态
      // owns_memory_=false, Delete 为空操作) 后整体赋值 —— 动态缓冲所有权已随
      // 归档快照移交, 此处不得释放
      let tl_mut = &mut *self.thread_local_.get();
      *tl_mut = ThreadLocal::INIT;
      tl_mut.reset_to_static_stack(ss);
    }
    // SAFETY: 契约保证 to 区 ≥ K_THREAD_LOCAL_SIZE
    unsafe { to.add(K_THREAD_LOCAL_SIZE) }
  }

  /// 原版 Stack::RestoreStack (cc:77-80) — 从归档块原位恢复 ThreadLocal。
  ///
  /// # Safety
  /// `from` 须指向 ≥ `K_THREAD_LOCAL_SIZE` 字节、内容为 `archive_stack` 写出的合法快照;
  /// 且恢复前当前 ThreadLocal 处缺省/已回落态, 否则旧动态缓冲被覆盖所有权而泄漏
  /// (原版 MemCopy 直覆盖同构风险, 由线程切换纪律保证)。
  /// 返回消费区末尾 (from + K_THREAD_LOCAL_SIZE)。
  pub unsafe fn restore_stack(&self, from: *mut u8) -> *mut u8 {
    // SAFETY: 契约保证 from 区大小; 目标为对齐 ThreadLocal 槽位, 逐字节覆盖等价 MemCopy
    unsafe {
      copy_nonoverlapping(
        from,
        self.thread_local_.get().cast::<u8>(),
        K_THREAD_LOCAL_SIZE,
      );
      from.add(K_THREAD_LOCAL_SIZE)
    }
  }

  /// 原版 FreeThreadResources (h:95) — 线程资源释放: 动态栈归还并回落静态区
  pub fn free_thread_resources(&self) {
    let ss = self.static_stack_ptr();
    // SAFETY: 同 tl 读写纪律
    unsafe { (*self.thread_local_.get()).reset_to_static_stack(ss) };
  }

  /// 原版 Stack::thread_local_address (h:174-176) —
  /// 原版 ExternalReference/宏汇编 friend 访存基址; Rust 收敛为 pub:
  /// 代码生成侧经 K_MEMORY_TOP_OFFSET/K_STACK_POINTER_OFFSET/K_LIMIT_OFFSET 读写块内字段
  #[inline]
  pub fn thread_local_address(&self) -> Address {
    // UnsafeCell::get 仅派生指针 (Box 无关, thread_local_ 随 Stack 存储);
    // 由此派生的访问保持 Stacked Borrows 写权限
    self.thread_local_.get() as Address
  }

  /// 原版 Stack::sp_top_delta (h:179-185) — 栈指针相对栈顶的位置无关偏移 (≤0)
  fn sp_top_delta(&self) -> isize {
    let tl = self.tl();
    let result = (tl.stack_pointer_ as isize) - (tl.memory_top_ as isize);
    debug_assert!(result <= 0);
    result
  }

  /// 原版 Stack::ResetIfEmpty (h:189) — 空栈时回落静态区 (h:151-153 转发)
  fn reset_if_empty(&self) {
    let ss = self.static_stack_ptr();
    // SAFETY: 同 tl 读写纪律
    unsafe {
      (*self.thread_local_.get()).reset_to_static_stack_if_empty(ss);
    }
  }

  /// 原版 Stack::IsValid (h:192) — 线程局部存储未被失效
  #[inline]
  fn is_valid(&self) -> bool {
    !self.tl().memory_.is_null()
  }
}

impl Drop for Stack {
  /// 原版 ~Stack (cc:27): FreeAndInvalidate
  fn drop(&mut self) {
    // SAFETY: &mut self 独占; 失效后字段不再访问
    unsafe { (*self.thread_local_.get()).free_and_invalidate() };
  }
}

impl Default for Stack {
  /// 与 new() 同义 (缺省即静态预留区初态)
  #[inline]
  fn default() -> Stack {
    Stack::new()
  }
}

/// 原版 regexp::StackScope (V8_NODISCARD, h:29-44) —
/// 控制 growing stack 生命周期的 RAII 域: 构造记录 sp_top_delta 基线,
/// 析构校验栈纪律 (原版 CHECK_EQ) 并做空栈回落
#[must_use = "StackScope 的存活期即栈生命周期域, 立即丢弃将失去 reset 语义"]
pub struct StackScope<'a> {
  regexp_stack_: &'a Stack,
  old_sp_top_delta_: isize,
}

impl<'a> StackScope<'a> {
  /// 原版 StackScope(Isolate*) (cc:14-18): isolate->regexp_stack() 取栈并 DCHECK 有效。
  /// Isolate 尚未冻结 (objects L3 前), 以 &Stack 参数注入 (glossary 占位规则)
  pub fn new(regexp_stack: &'a Stack) -> Self {
    debug_assert!(regexp_stack.is_valid());
    let old_sp_top_delta_ = regexp_stack.sp_top_delta();
    Self {
      regexp_stack_: regexp_stack,
      old_sp_top_delta_,
    }
  }

  /// 原版 StackScope::stack() (h:39)
  #[inline]
  pub fn stack(&self) -> &'a Stack {
    self.regexp_stack_
  }
}

impl Drop for StackScope<'_> {
  /// 原版 ~StackScope (cc:20-23): CHECK_EQ(old_sp_top_delta_, now) + ResetIfEmpty。
  /// CHECK 失败走 v8-base 冻结件 fatal (abort 语义, 非 panic 路径, 见票单)
  fn drop(&mut self) {
    let now = self.regexp_stack_.sp_top_delta();
    if let Some(msg) = check_op_eq(&self.old_sp_top_delta_, &now, "sp_top_delta") {
      v8_fatal(format_args!("{msg}"));
    }
    self.regexp_stack_.reset_if_empty();
  }
}
