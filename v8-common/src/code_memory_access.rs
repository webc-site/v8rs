// 由 tool/src/common.js 从 v8/src/common/code-memory-access.h/.cc/-inl.h 生成;
// 修订就地改, 复跑覆盖
//
// 修订 (Developer): ThreadIsolation 完整语义 — JitPage 注册表 (BTreeMap 区间
// 注册/注销/合并拆分, kSplitThreshold = 0x40000), WritableJitAllocation /
// WritableJitPage / WritableFreeSpace 写通道, RwxMemoryWriteScope 三平台分支
// (macOS arm64 pthread_jit_write_protect_np / Linux x64 pkey 运行时探测 /
// 其余 no-op)。原版 CHECK 失败 → Result 落 error.rs (登记)。
//
// 修订 (审查环 R1): WritableJumpTablePair::lookup 改查找语义 (调用方先注册,
// Split 后 CHECK(Contains), -inl.h:115-143); 零尺寸拒绝 CHECK_GT
// (cc:200-201, 339); CopyCode memmove 语义 (-inl.h:260-265); debug+enforce
// 构造复位写作用域 (-inl.h:64-69); IsSupported 第三分支恒 false 无断言
// (-inl.h:385-394); flags 收 pub(crate); 补 Expand (cc:318-320) /
// IsPKUWritable (cc:28-32) / RegisterExternallyAllocatedKey (cc:115-116);
// TearDown 不复位 ALLOCATOR/PKEY (cc:149-164)。
//
// 修订 (审查环 R3): 对齐原版页级锁并发语义 (K5) — 注册表 map 值改
// Arc<JitPage> (原版 .h:324-325 "store pointers in the map since we want to
// use the entries without keeping the map locked"), JitPage 内含 per-page
// 锁 (对齐 JitPage::mutex_, .h:307); 注册表锁仅查找/插入/删除瞬间持有,
// JitPageReference / WritableJitPage / WritableJumpTablePair 持页锁 guard
// 至析构 (对齐 page_lock_, .h:294 / -inl.h:126-131)。页尺寸为原子镜像:
// 变更均持页锁, 注册表重叠检查免页锁读取 — 注册路径不被无关页页锁串行化。
// 错误路径注册表一致性 (K6) — SplitJitPageLocked 切口校验提前到任何注册表
// 变更前 (原版 cc:598 CHECK 即时终止进程无残留, Rust 落 Result 后先变更后
// 校验会让 mid/tail 带病入表不可回滚); RegisterJitAllocations 改先全量校验
// 再统一落表 (原版逐条 CHECK 失败即终止, Rust 中途失败会留部分注册)。
// StartOfJitAllocationAt 外层 Err 补 error 日志 (与内层间隙路径同纪律)。
//
// 修订 (审查环 round1): D1 同族 — RegisterJitAllocations total_size 改
// checked 累加 (debug 构建 sum 溢出 panic 违禁, 落 AddrOverflow);
// D3 — WritableJitAllocation / WritableJitPage 字段序对齐原版成员逆序析构
// (page_ref_ 先于 write_scope_ 释放), enforce 析构的页引用改为写作用域
// 存活期间显式 drop (对齐原版 emplace 进成员的时序);
// 补遗漏 — RwxMemoryWriteScope::memory_protection_key (cc:24-26)。
// 裁剪登记: ForPatchableBaselineJIT (-inl.h:89-101, V8_ENABLE_SPARKPLUG_PLUS
// 关); StlAllocator + operator==/!= (.h:217-243, 600-610, C++ STL 分配器
// 胶水, Rust 注册表为普通静态 BTreeMap 无需分配器层)。
//
// 修订 (审查环 round2): F3 — UnregisterAllocationsExcept 迭代夹紧消除
// BTreeMap::range(start > end) panic (原版 keep_after ≥ keep_before 恒序,
// cc:391-409; Rust range 起点大于终点无条件 panic, release 亦然);
// F4 — UnregisterRange / UnregisterAllocationsExcept 左切/右切/pred 偏差
// 登记成文 (函数注释); F5 — page_ref()/page_ref_mut() 虚构原版引证纠正为
// "Rust 扩展访问器" 登记。注册表不变式: 注册路径 checked 校验保证
// addr + size ≤ page_addr + page_size ≤ usize::MAX (register_jit_page
// cc:447 CheckForRegionOverlap + check_allocation cc:336-341), 故注册表内
// 键加法不回绕, C++ Address 回绕语义无需 wrapping 镜像。
//
// 修订 (审查环 round2 findings): F2 — Initialize 的 enable 清零检查按
// config::V8_HAS_PKU_JIT_WRITE_PROTECT 常量门控, 还原原版 #if 平台结构
// (cc:95-104: 非 pku 平台不评估, 初始化成功); F3 — expand 与注册表重叠
// 检查的 TOCTOU 以文档契约登记 (expand 不得与 register 并发; 注册表锁
// 加锁修法与 "注册表锁 → 页锁" 全序反序会死锁, 见 JitPageRef::expand);
// F4 — 补 write_value_relaxed (原版 -inl.h:249-258 RelaxedStoreTag 变体)
// 与 write_header_slot 的 offset 模板参 (const 泛型承载)。
//
// 修订 (审查环 round4): F8 — RwxMemoryWriteScope 线程绑定 (!Send + !Sync):
// 原版删除拷贝构造 (h:99-100) 且自声明析构抑制移动, 权限窗口仅作用于
// 构造线程; 零字段 Rust 载体派生 Send, 跨线程 drop 会在他线程执行
// SetExecutable 而使构造线程永久残留可写权限。PhantomData<*const ()> 收紧,
// 同 PerThreadAssertScope (assert_scope round3 F7) 纪律。

use core::{
  marker::PhantomData,
  mem::transmute,
  ops::Bound,
  ptr,
  sync::atomic::{
    AtomicBool, AtomicI32, AtomicU8, AtomicU16, AtomicU32, AtomicU64, AtomicUsize, Ordering,
  },
};
use std::{collections::BTreeMap, sync::Arc};

use parking_lot::{Mutex, MutexGuard};
use ptr::NonNull;

use crate::{
  error::{Error, Result},
  globals::{
    K_CLEARED_FREE_MEMORY_VALUE, K_TAGGED_SIZE,
    config::{V8_HAS_PKU_JIT_WRITE_PROTECT, V8_HEAP_USE_PKU_JIT_WRITE_PROTECT},
  },
};

/// 原版 ThreadIsolation::JitAllocationType (code-memory-access.h:143-149)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum JitAllocationType {
  InstructionStream,
  WasmCode,
  WasmJumpTable,
  WasmFarJumpTable,
  WasmLazyCompileTable,
}

/// 原版 WritableJitAllocation::JitAllocationSource (code-memory-access.h:451-454)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
enum JitAllocationSource {
  Register,
  Lookup,
}

/// 原版 JitAllocation (code-memory-access.h:245-255)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct JitAllocation {
  size: usize,
  ty: JitAllocationType,
}

impl JitAllocation {
  const fn new(size: usize, ty: JitAllocationType) -> Self {
    Self { size, ty }
  }
  /// 原版 Size()
  pub const fn size(&self) -> usize {
    self.size
  }
  /// 原版 Type()
  pub const fn ty(&self) -> JitAllocationType {
    self.ty
  }
}

/// 原版 PageAllocator::Permission 的使用子集 (include/v8-platform.h)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum PagePermissions {
  NoAccess,
  ReadWrite,
  ReadWriteExecute,
}

/// 原版 v8::ThreadIsolatedAllocator (include/v8-platform.h) 的 Rust 面 —
/// 由嵌入方提供受 pkey 保护的元数据分配器 (L3 接入点)
pub trait ThreadIsolatedAllocator: Send + Sync {
  /// 原版 Allocate(size)
  fn allocate(&self, size: usize) -> Option<NonNull<u8>>;
  /// 原版 Free(ptr)
  fn free(&self, ptr: NonNull<u8>);
  /// 原版 Pkey() — 仅 pkey 平台有效, 其余 -1
  fn pkey(&self) -> i32;
}

/// 原版 v8_flags 子集 (L3 flags crate 接管前, 原子布尔承载; 默认值 = V8 默认)
/// crate 内模块, 经 lib.rs 平铺 pub use 导出 (禁 pub mod)
pub(crate) mod flags {
  use core::sync::atomic::{AtomicBool, Ordering};

  static JITLESS: AtomicBool = AtomicBool::new(false);
  static FORCE_MEMORY_PROTECTION_KEYS: AtomicBool = AtomicBool::new(false);
  static MEMORY_PROTECTION_KEYS: AtomicBool = AtomicBool::new(true);

  pub fn jitless() -> bool {
    JITLESS.load(Ordering::Relaxed)
  }
  pub fn set_jitless(v: bool) {
    JITLESS.store(v, Ordering::Relaxed);
  }
  pub fn force_memory_protection_keys() -> bool {
    FORCE_MEMORY_PROTECTION_KEYS.load(Ordering::Relaxed)
  }
  pub fn set_force_memory_protection_keys(v: bool) {
    FORCE_MEMORY_PROTECTION_KEYS.store(v, Ordering::Relaxed);
  }
  pub fn memory_protection_keys() -> bool {
    MEMORY_PROTECTION_KEYS.load(Ordering::Relaxed)
  }
  pub fn set_memory_protection_keys(v: bool) {
    MEMORY_PROTECTION_KEYS.store(v, Ordering::Relaxed);
  }
}

// ── 平台宏展开 (code-memory-access.h:43-66) ───────────────────────────────

/// 原版 V8_HAS_PKU_JIT_WRITE_PROTECT 分支 THREAD_ISOLATION_ALIGN_SZ (0x1000),
/// 其余 0 (code-memory-access.h:43-66)。门控条件为 build_config.h:30-37 的
/// 宿主平台判定 (V8_HOST_ARCH_X64); Rust 以 cfg! 目标三元组承载, v8rs 用法
/// 宿主=目标恒成立 (交叉编译差异登记 readme) — 经
/// `config::V8_HAS_PKU_JIT_WRITE_PROTECT` 单点展开 (审查环 R3 偏差修正)
pub const THREAD_ISOLATION_ALIGN_SZ: usize = if V8_HAS_PKU_JIT_WRITE_PROTECT {
  0x1000
} else {
  0
};

/// 原版 THREAD_ISOLATION_FILL_PAGE_SZ(size) (code-memory-access.h:55-64)
pub const fn thread_isolation_fill_page_sz(size: usize) -> usize {
  if THREAD_ISOLATION_ALIGN_SZ == 0 {
    0
  } else {
    (THREAD_ISOLATION_ALIGN_SZ - (size & (THREAD_ISOLATION_ALIGN_SZ - 1)))
      % THREAD_ISOLATION_ALIGN_SZ
  }
}

// 原版 static_assert (code-memory-access.h:347) sizeof(trusted_data_) == ALIGN_SZ
// — TrustedData 在 Rust 为独立静态项, 不落位 (登记)。

/// 原版 ThreadIsolation::kSplitThreshold (code-memory-access.cc:544)
pub const K_SPLIT_THRESHOLD: usize = 0x40000;

// ── pkey FFI (libc 0.2 未导出 glibc pkey_*, 自行声明) ──────────────────────

/// 原版 base::MemoryProtectionKey 权限值
mod pkey {
  #[cfg(all(target_os = "linux", target_arch = "x86_64"))]
  use core::ffi::c_void;
  use core::sync::atomic::{AtomicI32, Ordering};

  pub const K_NO_RESTRICTIONS: u32 = 0;
  pub const K_DISABLE_WRITE: u32 = 1; // PKEY_DISABLE_WRITE
  pub const K_NO_KEY: i32 = -1;

  /// 外部分配器 pkey 登记 (原版 RegisterExternallyAllocatedKey,
  /// code-memory-access.cc:115-116) — 供信号处理器引用; 本 crate 无
  /// 信号处理器路径, 仅静态登记 (L3 v8-base memory-protection-key 接管)
  static EXTERNALLY_ALLOCATED_KEY: AtomicI32 = AtomicI32::new(K_NO_KEY);

  pub fn register_externally_allocated_key(key: i32) {
    EXTERNALLY_ALLOCATED_KEY.store(key, Ordering::Relaxed);
  }

  #[cfg(all(target_os = "linux", target_arch = "x86_64"))]
  mod ffi {
    use core::ffi::{c_int, c_uint, c_void};

    unsafe extern "C" {
      pub fn pkey_alloc(flags: c_uint, access_rights: c_uint) -> c_int;
      pub fn pkey_free(key: c_int) -> c_int;
      pub fn pkey_mprotect(addr: *mut c_void, len: usize, prot: i32, pkey: c_int) -> c_int;
      pub fn pkey_set(key: c_int, rights: c_uint) -> c_int;
      pub fn pkey_get(key: c_int) -> c_uint;
    }
  }

  /// 原版 HasMemoryProtectionKeyAPIs() — pkey_alloc 探测, 成功即释放
  #[cfg(all(target_os = "linux", target_arch = "x86_64"))]
  pub fn has_memory_protection_key_apis() -> bool {
    // SAFETY: glibc 稳定 ABI, 探测用
    let key = unsafe { ffi::pkey_alloc(0, 0) };
    if key != -1 {
      unsafe { ffi::pkey_free(key) };
      true
    } else {
      false
    }
  }
  #[cfg(not(all(target_os = "linux", target_arch = "x86_64")))]
  pub const fn has_memory_protection_key_apis() -> bool {
    false
  }

  /// 原版 SetPermissionsForKey(pkey, kNoRestrictions/kDisableWrite)
  #[cfg(all(target_os = "linux", target_arch = "x86_64"))]
  pub fn set_permissions_for_key(key: i32, rights: u32) {
    // SAFETY: glibc 稳定 ABI
    unsafe { ffi::pkey_set(key, rights) };
  }
  #[cfg(not(all(target_os = "linux", target_arch = "x86_64")))]
  pub const fn set_permissions_for_key(_key: i32, _rights: u32) {}

  /// 原版 GetKeyPermission(pkey)
  #[cfg(all(target_os = "linux", target_arch = "x86_64"))]
  pub fn get_key_permission(key: i32) -> u32 {
    // SAFETY: glibc 稳定 ABI
    unsafe { ffi::pkey_get(key) }
  }
  #[cfg(not(all(target_os = "linux", target_arch = "x86_64")))]
  pub const fn get_key_permission(_key: i32) -> u32 {
    K_NO_RESTRICTIONS
  }

  /// 原版 SetPermissionsAndKey(ranges, permissions, pkey) — pkey_mprotect
  #[cfg(all(target_os = "linux", target_arch = "x86_64"))]
  pub fn set_permissions_and_key(addr: usize, size: usize, prot: i32, key: i32) -> bool {
    // SAFETY: glibc 稳定 ABI; 调用方保证 [addr, addr+size) 为本进程映射区
    unsafe { ffi::pkey_mprotect(addr as *mut c_void, size, prot, key) != -1 }
  }
  #[cfg(not(all(target_os = "linux", target_arch = "x86_64")))]
  pub const fn set_permissions_and_key(_addr: usize, _size: usize, _prot: i32, _key: i32) -> bool {
    false
  }
}

/// POSIX prot 值 (PROT_READ=1, PROT_WRITE=2, PROT_EXEC=4, 全平台一致)
const PROT_NONE: i32 = 0;
const PROT_READ_WRITE_EXECUTE: i32 = 0b111;

// ── RwxMemoryWriteScope (code-memory-access.h:90-122, -inl.h:29-39, 323-396) ─

/// RWX 内存写权限作用域 — 可重入并发 (每线程权限切换)
///
/// 审查环 round4 (F8): 线程绑定 — 原版作用域删除拷贝构造 (h:99-100) 且
/// 自声明析构抑制移动构造, 实体栈绑定不可跨线程迁移; 权限切换仅作用于
/// 构造线程 (h:72-89 "Only current thread is affected")。Rust 零字段载体
/// 会派生 Send, 作用域可被 move 至他线程 drop, 使构造线程永久残留
/// SetWritable 权限窗口 (原版不可能发生的权限泄漏)。以
/// `PhantomData<*const ()>` 收紧为 !Send + !Sync, 与 PerThreadAssertScope
/// (round3 F7) 同族纪律。
pub struct RwxMemoryWriteScope {
  _priv: (),
  /// 线程绑定标记 — *const () 非 Send, 类型随之 !Send + !Sync
  _thread_bound: PhantomData<*const ()>,
}

impl RwxMemoryWriteScope {
  /// 原版构造 (comment 仅注明用途)
  #[inline]
  pub fn new(_comment: &str) -> Self {
    if !flags::jitless() || flags::force_memory_protection_keys() {
      Self::set_writable();
    }
    Self {
      _priv: (),
      _thread_bound: PhantomData,
    }
  }

  /// 原版 IsSupported() — 当前配置是否支持快速写保护 (-inl.h:385-394 三分支)
  #[inline]
  pub fn is_supported() -> bool {
    if cfg!(all(target_os = "macos", target_arch = "aarch64")) {
      // pthread JIT 分支: 恒真
      true
    } else if cfg!(all(target_os = "linux", target_arch = "x86_64")) {
      // pku 分支
      ThreadIsolation::initialized() && ThreadIsolation::pkey_is_available()
    } else {
      // 原版第三分支恒 false, 无断言 — 非 target 平台未 Initialize 也不炸
      false
    }
  }

  /// 原版 memory_protection_key() (code-memory-access.h:106-107, .cc:24-26) —
  /// 原版仅在 V8_HAS_PKU_JIT_WRITE_PROTECT 配置下暴露 (h:106-110); Rust 恒
  /// 提供: 非 pku 构建 PKEY 静态保持 K_NO_KEY (-1), 取值语义等价 (登记)
  #[inline]
  pub fn memory_protection_key() -> i32 {
    ThreadIsolation::pkey()
  }

  /// 原版 IsPKUWritable (code-memory-access.cc:28-32) — 仅 pku 配置有意义:
  /// 当前 pkey 权限已解除写禁 (即处于可写窗口) 才真
  #[inline]
  pub fn is_pku_writable() -> bool {
    if !V8_HAS_PKU_JIT_WRITE_PROTECT || !Self::is_supported() {
      return false;
    }
    pkey::get_key_permission(ThreadIsolation::pkey()) == pkey::K_NO_RESTRICTIONS
  }

  /// 原版 SetWritable() — mac: SetJitWriteProtected(0); pku: 解除写禁
  #[inline]
  fn set_writable() {
    if cfg!(all(target_os = "macos", target_arch = "aarch64")) {
      #[cfg(all(target_os = "macos", target_arch = "aarch64"))]
      // SAFETY: 系统调用, 仅影响当前线程 JIT 权限
      unsafe {
        libc::pthread_jit_write_protect_np(0)
      }
    } else if Self::is_supported() {
      let pkey = ThreadIsolation::pkey();
      debug_assert!(pkey::get_key_permission(pkey) != pkey::K_NO_RESTRICTIONS);
      pkey::set_permissions_for_key(pkey, pkey::K_NO_RESTRICTIONS);
    }
  }

  /// 原版 SetExecutable()
  #[inline]
  fn set_executable() {
    if cfg!(all(target_os = "macos", target_arch = "aarch64")) {
      #[cfg(all(target_os = "macos", target_arch = "aarch64"))]
      // SAFETY: 系统调用, 仅影响当前线程 JIT 权限
      unsafe {
        libc::pthread_jit_write_protect_np(1)
      }
    } else if Self::is_supported() {
      let pkey = ThreadIsolation::pkey();
      debug_assert!(pkey::get_key_permission(pkey) == pkey::K_NO_RESTRICTIONS);
      pkey::set_permissions_for_key(pkey, pkey::K_DISABLE_WRITE);
    }
  }
}

impl Drop for RwxMemoryWriteScope {
  #[inline]
  fn drop(&mut self) {
    if !flags::jitless() || flags::force_memory_protection_keys() {
      Self::set_executable();
    }
  }
}

/// 原版 NopRwxMemoryWriteScope (code-memory-access.h:615-621)
pub struct NopRwxMemoryWriteScope;

impl NopRwxMemoryWriteScope {
  #[inline]
  pub fn new(_comment: &str) -> Self {
    Self
  }
}

/// 原版 RwxMemoryWriteScopeForTesting (code-memory-access.h:626-637) —
/// 仅消内联差异, 语义同 RwxMemoryWriteScope (登记为别名)
pub type RwxMemoryWriteScopeForTesting = RwxMemoryWriteScope;

/// 原版 CFIMetadataWriteScope (code-memory-access.h:639-644) —
/// pthread JIT 分支元数据不受保护 → Nop, 其余 → RwxMemoryWriteScope
#[cfg(all(target_os = "macos", target_arch = "aarch64"))]
pub type CFIMetadataWriteScope = NopRwxMemoryWriteScope;
#[cfg(not(all(target_os = "macos", target_arch = "aarch64")))]
pub type CFIMetadataWriteScope = RwxMemoryWriteScope;

/// 原版 DiscardSealedMemoryScope (code-memory-access.h:646-650) —
/// V8_ENABLE_MEMORY_SEALING 关 → Nop
pub type DiscardSealedMemoryScope = NopRwxMemoryWriteScope;

// ── 注册表存储 (原版 ThreadIsolation::TrustedData, code-memory-access.h:332-345)
// ─────────────────────────────────────────────────────────────────────────────

/// 页内分配表 — 原版 JitPage::allocations_ (code-memory-access.h:308-311),
/// 由页锁保护
#[derive(Default)]
struct PageState {
  allocations: BTreeMap<usize, JitAllocation>,
}

/// 原版 JitPage (code-memory-access.h:301-317) — mutex_ 保护 allocations_ 与
/// size_; Rust 拆两层: allocations 由页锁保护, size 为原子镜像 — 变更均持
/// 页锁同步, 注册表重叠检查免页锁读取 (K5: 不相交页注册不得被无关页页锁
/// 串行化; 原版 GetSize (cc:246-248) 经 JitPageReference 取页锁读 size,
/// 注册路径会被任意邻页页锁持有者阻塞)
struct JitPage {
  /// 页尺寸镜像 — 权威值: 页锁临界区内读写; 弱一致读: 注册表重叠检查。
  /// 并发变更面: split/shrink/merge/unregister 均持注册表锁 (与检查路径互斥),
  /// 仅 Expand (cc:318-320) 持页锁不持注册表锁 — round2-F3 契约:
  /// expand 不得与 register_jit_page 并发 (见 JitPageRef::expand SAFETY)
  size: AtomicUsize,
  /// 页锁 (对齐 JitPage::mutex_, .h:307) — 保护 allocations
  state: Mutex<PageState>,
}

impl JitPage {
  const fn new(size: usize) -> Self {
    Self {
      size: AtomicUsize::new(size),
      state: Mutex::new(PageState {
        allocations: BTreeMap::new(),
      }),
    }
  }

  /// 页锁 guard — 生命周期抹平为 'static: Mutex 的真实存续由持有方同结构体
  /// 保存的 Arc<JitPage> 保证, 且 guard 字段声明在前先析构 (解锁时页仍存活)
  fn lock_state(&self) -> MutexGuard<'static, PageState> {
    // SAFETY: 仅抹平 guard 内部自持的 mutex 引用生命周期; 全部构造点
    // (JitPageRef) 均以同值 Arc 字段保证 self 存续至 guard 析构之后
    unsafe { transmute(self.state.lock()) }
  }
}

/// 原版 JitPageMap (code-memory-access.h:326-328) — 有序区间表;
/// map 存 Arc 以在不持注册表锁的情况下使用条目 (原版注释 .h:324-325)
type JitPageMap = BTreeMap<usize, Arc<JitPage>>;

static JIT_PAGES: Mutex<JitPageMap> = Mutex::new(JitPageMap::new());
static INITIALIZED: AtomicBool = AtomicBool::new(false);
static PKEY: AtomicI32 = AtomicI32::new(pkey::K_NO_KEY);
static ALLOCATOR: Mutex<Option<&'static dyn ThreadIsolatedAllocator>> = Mutex::new(None);

/// 原版 JitPageReference (code-memory-access.h:261-299) — 持页锁的页引用:
/// 页锁 guard 持有至析构 (对齐 page_lock_, .h:294), 注册表锁已由查找路径
/// 释放 — 持本引用期间他线程可注册/查询任意不相交页 (K5)
pub struct JitPageRef {
  /// 页锁 guard, 持有至析构 — 字段居首: 析构先放锁后减 Arc 引用,
  /// 保证解锁时 Mutex 仍存活
  _page_lock: MutexGuard<'static, PageState>,
  /// 页句柄 — Arc 保证页存续 (原版 jit_page_ 裸指针 + 注册表纪律,
  /// Rust 以引用计数收紧: 页移出注册表后引用持有者仍安全, 降级为空页快照)
  page: Arc<JitPage>,
  address: usize,
}

impl JitPageRef {
  /// 构造: 须持注册表锁调用 — clone Arc + 取页锁; 注册表锁由调用方在引用
  /// 构造完成后释放
  fn locked(pages: &JitPageMap, page_addr: usize) -> Self {
    // 键必在: 调用方持注册表锁, page_addr 来自同一临界区内的查找/拆分
    let page = pages.get(&page_addr).unwrap().clone();
    let _page_lock = page.lock_state();
    Self {
      _page_lock,
      page,
      address: page_addr,
    }
  }

  /// 原版 Expand (code-memory-access.cc:318-320)
  ///
  /// 并发契约 (round2-F3 登记): 本方法只持页锁不持注册表锁, 而
  /// register_jit_page 的重叠检查免页锁经 `JitPage::size` Relaxed 读邻页
  /// 尺寸 — 二者并发存在 TOCTOU: 检查线程可能读到扩张前旧尺寸, 使本应
  /// Err(RegionOverlap) 的注册混入扩张区间。原版经 GetSize 取页锁天然
  /// 串行化 (cc:246-248); Rust K5 免锁优化将该串行化让渡给调用方契约:
  /// **expand 不得与 register_jit_page (一切触发注册表重叠检查的注册路径)
  /// 并发**, 两者须由上层按页粒度互斥或串行调用。
  /// 备选修法 (a) expand 变更时加注册表锁不可行: 持页锁再取注册表锁与
  /// 既有 "注册表锁 → 页锁" 全序 (lookup/split 路径在注册表锁内取候选页
  /// 页锁) 反序, 会死锁 — 故取契约登记 (b)
  ///
  /// SAFETY: 调用方保证变更期间无并发 register; fetch_add 单调增长且页键
  /// (起始地址) 不变, 契约成立时不破坏注册表区间不变式
  pub fn expand(&mut self, offset: usize) {
    self.page.size.fetch_add(offset, Ordering::Relaxed);
  }

  /// 原版 Address()
  pub const fn address(&self) -> usize {
    self.address
  }

  /// 原版 Size() (code-memory-access.cc:306-308)
  pub fn size(&self) -> usize {
    self.page.size.load(Ordering::Relaxed)
  }

  /// 原版 End()
  pub fn end(&self) -> usize {
    self.address + self.size()
  }

  /// 原版 Empty() (code-memory-access.h:287)
  pub fn empty(&self) -> bool {
    self._page_lock.allocations.is_empty()
  }

  /// 注册前校验 (原版 RegisterAllocation 的 CHECK 族, cc:334-343) —
  /// 不落表, 供单条注册与批量注册两段式复用
  fn check_allocation(&self, addr: usize, size: usize) -> Result<()> {
    // CFI 视角数据不可信, 越界检查安全敏感
    if addr < self.address {
      return Err(Error::AllocOutOfPage {
        addr,
        size,
        page: self.address,
      });
    }
    let offset = addr - self.address;
    let Some(end_offset) = offset.checked_add(size) else {
      return Err(Error::AddrOverflow { addr, size });
    };
    // 原版 CHECK_GT(end_offset, offset) (cc:339) — 拒绝零尺寸分配
    if end_offset <= offset {
      return Err(Error::ZeroSize { addr, size });
    }
    // 原版 CHECK_GT(size_, offset) + CHECK_GE(size_, end_offset) (cc:340-341)
    let page_size = self.page.size.load(Ordering::Relaxed);
    if page_size <= offset || page_size < end_offset {
      return Err(Error::AllocOutOfPage {
        addr,
        size,
        page: self.address,
      });
    }
    check_for_region_overlap(
      &self._page_lock.allocations,
      JitAllocation::size,
      addr,
      size,
    )
  }

  /// 落表 (校验后) — 供单条注册与批量注册两段式复用
  fn insert_allocation(&mut self, addr: usize, alloc: JitAllocation) {
    self._page_lock.allocations.insert(addr, alloc);
  }

  /// 原版 RegisterAllocation (code-memory-access.cc:330-346)
  pub fn register_allocation(
    &mut self,
    addr: usize,
    size: usize,
    ty: JitAllocationType,
  ) -> Result<JitAllocation> {
    self.check_allocation(addr, size)?;
    let alloc = JitAllocation::new(size, ty);
    self.insert_allocation(addr, alloc);
    Ok(alloc)
  }

  /// 原版 LookupAllocation (code-memory-access.cc:348-357)
  pub fn lookup_allocation(
    &self,
    addr: usize,
    size: usize,
    ty: JitAllocationType,
  ) -> Result<JitAllocation> {
    let allocs = &self._page_lock.allocations;
    let Some(a) = allocs.get(&addr) else {
      return Err(Error::JitAllocationMismatch { addr, size });
    };
    if a.size() != size || a.ty() != ty {
      return Err(Error::JitAllocationMismatch { addr, size });
    }
    Ok(*a)
  }

  /// 原版 Contains (code-memory-access.cc:359-365)
  pub fn contains(&self, addr: usize, size: usize, ty: JitAllocationType) -> bool {
    let allocs = &self._page_lock.allocations;
    match allocs.get(&addr) {
      Some(a) => a.size() == size && a.ty() == ty,
      None => false,
    }
  }

  /// 原版 UnregisterAllocation (code-memory-access.cc:367-371)
  pub fn unregister_allocation(&mut self, addr: usize) -> Result<()> {
    let allocs = &mut self._page_lock.allocations;
    if allocs.remove(&addr).is_none() {
      return Err(Error::JitAllocationMismatch { addr, size: 0 });
    }
    Ok(())
  }

  /// 原版 UnregisterRange (code-memory-access.cc:373-384)
  ///
  /// 审查环 round2 (F4 登记偏差): 左切校验为本移植加严 — 原版只对
  /// lower_bound(start) 起的分配做 AllocationIsBehindRange CHECK
  /// (cc:282-294), 键 < start 且跨过 start 的分配不受检、原样保留 (其内存
  /// 被部分释放而注册表仍记全尺寸, 属上游 CFI 弱点); Rust 对齐
  /// CHECK_LE(allocation_end, range_end) 同族语义, 左切即 Err(SplitFreeRange),
  /// 注册表不动。上游 UnregisterRange 调用方 (WritableJitPage::FreeRange,
  /// -inl.h:293-297) 恒按分配边界释放, 该偏差对合法调用序列不可观测。
  pub fn unregister_range(&mut self, start: usize, size: usize) -> Result<()> {
    let range_end = start
      .checked_add(size)
      .ok_or(Error::AddrOverflow { addr: start, size })?;
    let allocs = &mut self._page_lock.allocations;

    // 校验左侧分配不被切分
    if let Some((&prev_addr, prev)) = allocs.range(..start).next_back()
      && prev_addr + prev.size() > start
    {
      return Err(Error::SplitFreeRange { addr: start, size });
    }

    let mut to_remove = Vec::new();
    for (&k, a) in allocs.range(start..) {
      // 原版 AllocationIsBehindRange (code-memory-access.cc:282-294)
      if k >= range_end {
        break;
      }
      // CHECK_LE(allocation_end, range_end) — 释放范围切进分配中间 → 终止
      if k + a.size() > range_end {
        return Err(Error::SplitFreeRange { addr: start, size });
      }
      to_remove.push(k);
    }
    for k in to_remove {
      allocs.remove(&k);
    }
    Ok(())
  }

  /// 原版 UnregisterAllocationsExcept (code-memory-access.cc:386-416)
  ///
  /// 审查环 round2 (F4 登记偏差, 3 处加严/修正, 上游 API 全仓零调用方):
  /// 1. 左切: 原版 keep_before 前移一条 (cc:395-398) 后, 该分配进入
  ///    keep 匹配环 — 不在 keep 清单即被静默弃注册 (CFI 弱点); Rust
  ///    Err(SplitFreeRange), 注册表不动。
  /// 2. 右切: 原版对 [start, range_end) 内跨过 range_end 的分配同样静默
  ///    弃注册 (无 CHECK); Rust Err(SplitFreeRange), 与 UnregisterRange 的
  ///    CHECK_LE(allocation_end, range_end) (cc:292) 同族。
  /// 3. start 前的分配恒保留: 原版仅 [begin, pred) 无条件保留, pred 本身
  ///    须命中 keep[0] 否则弃注册; Rust 恒保留全部 [..start)。
  ///
  /// 原版 CHECK_EQ(keep_iterator, keep.end()) (cc:410) → Err(KeepListMismatch)。
  pub fn unregister_allocations_except(
    &mut self,
    start: usize,
    size: usize,
    keep: &[usize],
  ) -> Result<()> {
    let range_end = start
      .checked_add(size)
      .ok_or(Error::AddrOverflow { addr: start, size })?;
    let allocs = &mut self._page_lock.allocations;

    // 校验左侧分配不被切分
    if let Some((&prev_addr, prev)) = allocs.range(..start).next_back()
      && prev_addr + prev.size() > start
    {
      return Err(Error::SplitFreeRange { addr: start, size });
    }

    let mut kept: BTreeMap<usize, JitAllocation> = BTreeMap::new();

    // 1. 保留 start 之前的所有分配 (code-memory-access.cc:394)
    for (&k, v) in allocs.range(..start) {
      kept.insert(k, *v);
    }

    // 2. [start, range_end) 区间内仅保留 keep 清单中的分配 (顺序匹配, cc:400-410)
    let mut keep_iter = keep.iter().copied();
    let mut expected_next_keep = keep_iter.next();
    for (&k, v) in allocs.range(start..range_end) {
      if k + v.size() > range_end {
        return Err(Error::SplitFreeRange { addr: start, size });
      }
      if Some(k) == expected_next_keep {
        kept.insert(k, *v);
        expected_next_keep = keep_iter.next();
      }
    }

    // 原版 CHECK_EQ(keep_iterator, keep.end())
    if expected_next_keep.is_some() {
      return Err(Error::KeepListMismatch);
    }

    // 3. 保留 range_end 之后的所有分配 (code-memory-access.cc:412)
    for (&k, v) in allocs.range(range_end..) {
      kept.insert(k, *v);
    }

    self._page_lock.allocations = kept;
    Ok(())
  }

  /// 原版 AllocationContaining (code-memory-access.cc:423-432)
  pub fn allocation_containing(&self, inner_pointer: usize) -> Result<(usize, JitAllocation)> {
    let allocs = &self._page_lock.allocations;
    let Some((&k, a)) = allocs.range(..=inner_pointer).next_back() else {
      return Err(Error::JitAllocationMismatch {
        addr: inner_pointer,
        size: 0,
      });
    };
    let offset = inner_pointer - k;
    if a.size() <= offset {
      return Err(Error::JitAllocationMismatch {
        addr: inner_pointer,
        size: a.size(),
      });
    }
    Ok((k, *a))
  }

  /// 原版 StartOfAllocationAt (code-memory-access.cc:418-421)
  pub fn start_of_allocation_at(&self, inner_pointer: usize) -> Result<usize> {
    self.allocation_containing(inner_pointer).map(|(k, _)| k)
  }

  /// 原版 EndOfLastAllocation (code-memory-access.cc:434-440)
  pub fn end_of_last_allocation(&self) -> usize {
    let allocs = &self._page_lock.allocations;
    match allocs.iter().next_back() {
      Some((&k, a)) => k + a.size(),
      None => self.address,
    }
  }
}

/// 原版 CheckForRegionOverlap (code-memory-access.cc:254-280) —
/// CFI 视角数据不可信, 重叠即安全违规
///
/// 并发契约 (round2-F3 登记): 本检查免页锁经 `JitPage::size` 原子镜像读
/// 邻页尺寸 (K5: 不相交页注册不得被邻页页锁串行化); JitPageRef::expand
/// 只持页锁变更同字段, 与本检查并发存在 TOCTOU — expand 不得与本检查的
/// 调用路径 (register_jit_page) 并发 (见 JitPageRef::expand 契约)
fn check_for_region_overlap<T>(
  map: &BTreeMap<usize, T>,
  entry_size: impl Fn(&T) -> usize,
  addr: usize,
  size: usize,
) -> Result<()> {
  // CHECK_GE(addr + size, addr)
  if addr.checked_add(size).is_none() {
    return Err(Error::AddrOverflow { addr, size });
  }
  // 与后一条目重叠: CHECK_LE(size, next_addr - addr)
  if let Some((&next_addr, _)) = map.range((Bound::Excluded(addr), Bound::Unbounded)).next()
    && next_addr - addr < size
  {
    return Err(Error::RegionOverlap { addr, size });
  }
  // 与前一条目重叠: CHECK_LE(GetSize(prev), addr - prev_addr)
  if let Some((&prev_addr, prev)) = map.range(..=addr).next_back() {
    let offset = addr - prev_addr;
    if entry_size(prev) > offset {
      return Err(Error::RegionOverlap { addr, size });
    }
  }
  Ok(())
}

/// 原版 TryLookupJitPageLocked (code-memory-access.cc:196-242) —
/// 覆盖 [addr, addr+size) 的页, 必要时合并相邻页; 返回页起始地址。
/// 须持注册表锁; 候选页页锁在覆盖判定与合并期间持有 (对齐原版
/// JitPageReference 持页锁贯穿, 函数返回前释放)
fn try_lookup_jit_page_locked(
  pages: &mut JitPageMap,
  addr: usize,
  size: usize,
) -> Result<Option<usize>> {
  let end = addr
    .checked_add(size)
    .ok_or(Error::AddrOverflow { addr, size })?;
  // 原版 CHECK_GT(end, addr) (code-memory-access.cc:200-201) — 拒绝零尺寸
  // (CFI: data untrusted), 静默接受会使空区间伪装成合法覆盖
  if end <= addr {
    return Err(Error::ZeroSize { addr, size });
  }

  // upper_bound(addr) 前移一条 = 候选页 (cc:203-211)
  let Some((&page_addr, page)) = pages.range(..=addr).next_back() else {
    return Ok(None);
  };
  // Arc 脱表: 后续 &mut pages 操作不与页锁借用冲突
  let page = page.clone();
  let mut state = page.state.lock();
  let mut covered_end = page_addr + page.size.load(Ordering::Relaxed);
  if covered_end <= addr {
    return Ok(None);
  }
  if covered_end >= end {
    return Ok(Some(page_addr));
  }

  // 跨页: 依次合并相邻页 (cc:224-235)
  let next_keys: Vec<usize> = pages
    .range((Bound::Excluded(page_addr), Bound::Unbounded))
    .map(|(&k, _)| k)
    .collect();
  for k in next_keys {
    if covered_end >= end {
      break;
    }
    // 原版 CHECK_EQ(next_page.Address(), jit_page.End()) — 页必须邻接
    if k != covered_end {
      return Err(Error::JitPageNotFound { addr, size });
    }
    // 键必在: next_keys 取自同一注册表锁临界区, 期间仅本函数变更注册表
    let next = pages.remove(&k).unwrap();
    let mut next_state = next.state.lock();
    let next_size = next.size.load(Ordering::Relaxed);
    covered_end += next_size;
    // 原版 Merge (cc:322-328)
    page.size.fetch_add(next_size, Ordering::Relaxed);
    next.size.store(0, Ordering::Relaxed);
    state.allocations.append(&mut next_state.allocations);
    drop(next_state);
    drop(next); // 表项移除 + 分配搬空 → 最后引用释放即原版 Delete (cc:232);
    // 他线程仍持页引用时 Arc 延迟析构, 降级为空页快照 (原版为 UAF)
  }

  if covered_end < end {
    return Ok(None);
  }
  Ok(Some(page_addr))
}

/// 原版 LookupJitPageLocked (code-memory-access.cc:167-173)
fn lookup_jit_page_locked(pages: &mut JitPageMap, addr: usize, size: usize) -> Result<usize> {
  try_lookup_jit_page_locked(pages, addr, size)?.ok_or(Error::JitPageNotFound { addr, size })
}

/// 原版 SplitJitPageLocked (code-memory-access.cc:578-604) —
/// 切出 [addr, addr+size), 返回该页地址。须持注册表锁; 候选页页锁贯穿
/// 校验 + 收缩 (对齐原版 JitPageReference 持页锁贯穿 Split)
fn split_jit_page_locked(pages: &mut JitPageMap, addr: usize, size: usize) -> Result<usize> {
  let page_addr = lookup_jit_page_locked(pages, addr, size)?;
  // 键必在: 同一注册表锁临界区; Arc 脱表使后续 insert 不与页锁借用冲突
  let page = pages.get(&page_addr).unwrap().clone();
  let mut state = page.state.lock();

  let page_size = page.size.load(Ordering::Relaxed);
  let head_size = addr - page_addr;
  let tail_size = page_size - size - head_size;

  // K6: 切口校验提前 — 任何注册表变更前完成。原版 defense-in-depth CHECK
  // (cc:597-598) 在双收缩后即时终止进程, 无可观测残留; Rust 落 Result 后
  // 先变更后校验会让 tail/mid 带病入表不可回滚。原版 CHECK 作用于双收缩后
  // 残部 (键 < addr 的分配), 此处按 [..addr) 键域等价计算, 语义 1:1。
  if head_size > 0 {
    // 纵深防御: 切口不得落在代码对象中间
    let last_end = state
      .allocations
      .range(..addr)
      .next_back()
      .map_or(page_addr, |(&k, a)| k + a.size());
    if last_end > addr {
      return Err(Error::SplitFreeRange {
        addr: page_addr,
        size,
      });
    }
  }

  // 校验全通过 → 拆分落表 (cc:587-601)
  if tail_size > 0 {
    let mut tail = JitPage::new(tail_size);
    shrink_locked(&mut state, &page, page_addr, addr + size, &mut tail);
    pages.insert(addr + size, Arc::new(tail));
  }
  if head_size > 0 {
    let mut mid = JitPage::new(size);
    shrink_locked(&mut state, &page, page_addr, addr, &mut mid);
    pages.insert(addr, Arc::new(mid));
  }
  Ok(addr)
}

/// 原版 JitPageReference::Shrink (code-memory-access.cc:310-316) —
/// 页收缩至 new_end, [new_end, 原末尾) 的分配移交 tail。
/// 前置: 调用方持有 page 的页锁 (state guard); tail 为新建页未入表,
/// 无人竞争其页锁
fn shrink_locked(
  state: &mut PageState,
  page: &JitPage,
  page_addr: usize,
  new_end: usize,
  tail: &mut JitPage,
) {
  debug_assert!(new_end >= page_addr, "shrink_locked new_end 小于起始页地址");
  let mut tail_state = tail.state.lock();
  tail_state.allocations = state.allocations.split_off(&new_end);
  page.size.store(new_end - page_addr, Ordering::Relaxed);
}

// ── ThreadIsolation (code-memory-access.h:137-379) ────────────────────────

/// JIT 内存线程隔离 — 注册表 + 写通道 + 平台写保护 (AllStatic)
pub struct ThreadIsolation;

impl ThreadIsolation {
  /// 原版 Enabled() (code-memory-access.cc:42-48) — 仅 pkey 配置可为真
  #[inline]
  pub fn enabled() -> bool {
    V8_HEAP_USE_PKU_JIT_WRITE_PROTECT && ALLOCATOR.lock().is_some()
  }

  /// 原版 Initialize (code-memory-access.cc:74-146)
  pub fn initialize(allocator: Option<&'static dyn ThreadIsolatedAllocator>) -> Result<()> {
    INITIALIZED.store(true, Ordering::Relaxed);

    let mut enable = allocator.is_some();
    if flags::jitless() {
      enable = flags::force_memory_protection_keys();
    }
    debug_assert!(
      !flags::force_memory_protection_keys() || allocator.is_some(),
      "force_memory_protection_keys 需要线程隔离分配器",
    );

    // 原版 THREAD_SANITIZER 分支 (cc:87-93): TSan 构建关, 不落位
    //
    // 审查环 round2 (F2): 原版本检查整段包在 #if V8_HAS_PKU_JIT_WRITE_PROTECT
    // (cc:95-104) — 非 pku 平台不评估, enable 保持初值 (allocator 在即真),
    // 初始化成功; 无条件评估时非 pku 平台 has_memory_protection_key_apis
    // 恒假 → enable 被清零 → force_memory_protection_keys 恒落
    // Err(ForcedPkeyUnsupported)。以 config::V8_HAS_PKU_JIT_WRITE_PROTECT
    // 常量门控还原平台结构
    if V8_HAS_PKU_JIT_WRITE_PROTECT
      && (!flags::memory_protection_keys() || !pkey::has_memory_protection_key_apis())
    {
      // 原版分支内 DCHECK_IMPLIES 对 (cc:98-103)
      debug_assert!(!flags::force_memory_protection_keys() || flags::memory_protection_keys());
      debug_assert!(
        !flags::force_memory_protection_keys() || pkey::has_memory_protection_key_apis()
      );
      enable = false;
    }

    if enable {
      // 原版 allocator_ 赋值在 #if 外 (cc:113); pkey 读取与外部 key 登记在
      // #if V8_HAS_PKU_JIT_WRITE_PROTECT 内 (cc:111-119) — 非 pku 平台
      // 不触碰 pkey (round2-F2 平台结构还原)
      if let Some(alloc) = allocator {
        *ALLOCATOR.lock() = Some(alloc);
      }
      if V8_HAS_PKU_JIT_WRITE_PROTECT {
        let Some(alloc) = allocator else {
          // jitless + force 无分配器路径: 原版此处解引用 nullptr 即崩溃,
          // Rust 落错 (登记)
          return Err(Error::ForcedPkeyUnsupported);
        };
        let pkey_val = alloc.pkey();
        PKEY.store(pkey_val, Ordering::Relaxed);
        // 原版 cc:115-116 — 登记外部 pkey, 供信号处理器引用 (L3 接管, 此处静态登记)
        pkey::register_externally_allocated_key(pkey_val);
      }
    }

    // 原版 jit_pages_ 追踪内存在关断使能时也要分配 (cc:120-126) —
    // Rust 注册表为静态, 无需分配 (登记)

    // 原版 CHECK_IMPLIES(v8_flags.force_memory_protection_keys, enable)
    if flags::force_memory_protection_keys() && !enable {
      return Err(Error::ForcedPkeyUnsupported);
    }
    if !enable {
      return Ok(());
    }

    // 原版 pkey 分支的页对齐校验与 trusted_data_ 写保护
    // (cc:134-145) 依赖分配器元数据区, L3 接入 (登记)
    Ok(())
  }

  /// 原版 InitializeThreadIsolation (thread-isolation-unittest.cc:11-16) —
  /// 测试不提供 pkey 分配器, 强制关 force_memory_protection_keys
  pub fn initialize_for_tests(
    allocator: Option<&'static dyn ThreadIsolatedAllocator>,
  ) -> Result<()> {
    flags::set_force_memory_protection_keys(false);
    Self::initialize(allocator)
  }

  /// 原版 TearDown (code-memory-access.cc:149-164) —
  /// 仅校验并清空页表; 原版保留 ALLOCATOR/PKEY (trusted_data_ 只删 jit_pages_),
  /// Rust 对齐不复位 (登记)
  pub fn tear_down() -> Result<()> {
    if !Self::initialized() {
      return Err(Error::TearDownState);
    }
    // 原版 pkey 分支恢复 trusted_data_ 读写权限 — Rust 静态无保护页, 不落位
    let mut pages = JIT_PAGES.lock();
    if !pages.is_empty() {
      return Err(Error::TearDownState);
    }
    pages.clear();
    Ok(())
  }

  /// 原版 initialized() (code-memory-access.h:215)
  pub fn initialized() -> bool {
    INITIALIZED.load(Ordering::Relaxed)
  }

  /// 原版 pkey() (code-memory-access.h:211) — 仅 pkey 配置有意义
  pub fn pkey() -> i32 {
    PKEY.load(Ordering::Relaxed)
  }

  /// 原版 PkeyIsAvailable() (code-memory-access.h:212)
  pub fn pkey_is_available() -> bool {
    Self::pkey() != pkey::K_NO_KEY
  }

  /// 原版 RegisterJitPage (code-memory-access.cc:443-451)
  pub fn register_jit_page(address: usize, size: usize) -> Result<()> {
    let _scope = CFIMetadataWriteScope::new("Adding new executable memory.");
    let mut pages = JIT_PAGES.lock();
    // K5: 邻页尺寸经原子镜像读取 (原版 GetSize (cc:246-248) 取页锁读 size,
    // 注册路径会被无关页页锁持有者串行化) — 注册表锁仅此瞬间持有
    check_for_region_overlap(
      &*pages,
      |p: &Arc<JitPage>| p.size.load(Ordering::Relaxed),
      address,
      size,
    )?;
    pages.insert(address, Arc::new(JitPage::new(size)));
    Ok(())
  }

  /// 原版 UnregisterJitPage (code-memory-access.cc:453-494)
  pub fn unregister_jit_page(address: usize, size: usize) -> Result<()> {
    let _scope = CFIMetadataWriteScope::new("Removing executable memory.");
    let mut pages = JIT_PAGES.lock();
    let page_addr = lookup_jit_page_locked(&mut pages, address, size)?;

    let to_free_end = address.checked_add(size).ok_or(Error::AddrOverflow {
      addr: address,
      size,
    })?;
    // 键必在: 同一注册表锁临界区; Arc 脱表使后续 insert/remove 不与页锁借用冲突
    let page = pages.get(&page_addr).unwrap().clone();
    let mut state = page.state.lock();
    let jit_page_end = page_addr + page.size.load(Ordering::Relaxed);

    if to_free_end < jit_page_end {
      // 释放尾部之后还有残余 → 收缩并登记尾页 (cc:468-476)
      let tail_size = jit_page_end - to_free_end;
      let mut tail = JitPage::new(tail_size);
      shrink_locked(&mut state, &page, page_addr, to_free_end, &mut tail);
      pages.insert(to_free_end, Arc::new(tail));
    }
    debug_assert_eq!(to_free_end, page_addr + page.size.load(Ordering::Relaxed));

    if address == page_addr {
      // 移除区域起点 → 整页移出注册表 (drop 即原版 Delete, cc:480-483);
      // 他线程仍持页引用时 Arc 延迟析构, 降级为空页快照 (原版为 UAF)
      drop(state);
      pages.remove(&address);
    } else {
      // 只收缩: 移除部分不落表, drop 即原版 Delete(tail) (cc:484-491)
      debug_assert!(address > page_addr);
      let mut tail = JitPage::new(size);
      shrink_locked(&mut state, &page, page_addr, address, &mut tail);
    }
    Ok(())
  }

  /// 原版 MakeExecutable (code-memory-access.cc:497-508)
  pub fn make_executable(address: usize, size: usize) -> Result<bool> {
    // 原版 DCHECK(Enabled()) — 违反时 C++ 直接 UNREACHABLE, 此处落错 (登记)
    if !Self::enabled() {
      return Err(Error::NotSupported("MakeExecutable 需要 pkey 配置"));
    }
    Ok(pkey::set_permissions_and_key(
      address,
      size,
      PROT_READ_WRITE_EXECUTE,
      Self::pkey(),
    ))
  }

  /// 原版 LookupJitPage → JitPageRef (code-memory-access.cc:176-180)
  pub fn lookup_jit_page(addr: usize, size: usize) -> Result<JitPageRef> {
    let mut pages = JIT_PAGES.lock();
    let page_addr = lookup_jit_page_locked(&mut pages, addr, size)?;
    let page_ref = JitPageRef::locked(&pages, page_addr);
    // K5: 注册表锁仅查找瞬间持有, 页锁由引用持有至析构
    drop(pages);
    Ok(page_ref)
  }

  /// 原版 LookupWritableJitPage (code-memory-access.cc:183-186)
  pub fn lookup_writable_jit_page(addr: usize, size: usize) -> Result<WritableJitPage> {
    WritableJitPage::new(addr, size)
  }

  /// 原版 RegisterJitAllocation (code-memory-access.cc:511-516)
  pub fn register_jit_allocation(
    addr: usize,
    size: usize,
    ty: JitAllocationType,
    enforce_write_api: bool,
  ) -> Result<WritableJitAllocation> {
    WritableJitAllocation::with_page(
      addr,
      size,
      ty,
      JitAllocationSource::Register,
      enforce_write_api,
    )
  }

  /// 原版 RegisterInstructionStreamAllocation (code-memory-access.cc:519-523)
  pub fn register_instruction_stream_allocation(
    addr: usize,
    size: usize,
    enforce_write_api: bool,
  ) -> Result<WritableJitAllocation> {
    Self::register_jit_allocation(
      addr,
      size,
      JitAllocationType::InstructionStream,
      enforce_write_api,
    )
  }

  /// 原版 LookupJitAllocation (code-memory-access.cc:526-531)
  pub fn lookup_jit_allocation(
    addr: usize,
    size: usize,
    ty: JitAllocationType,
    enforce_write_api: bool,
  ) -> Result<WritableJitAllocation> {
    WritableJitAllocation::with_page(
      addr,
      size,
      ty,
      JitAllocationSource::Lookup,
      enforce_write_api,
    )
  }

  /// 原版 RegisterJitAllocations (code-memory-access.cc:534-553)
  pub fn register_jit_allocations(
    start: usize,
    sizes: &[usize],
    ty: JitAllocationType,
  ) -> Result<()> {
    let _scope = CFIMetadataWriteScope::new("Register bulk allocations.");
    // 审查环 round1 (D1 同族): 原版 size_t 累加回绕 (cc:539-542) 后续 CHECK
    // 兜底; Rust debug 构建求和溢出会 panic (违禁) → checked 累加落 Err
    let total_size = sizes
      .iter()
      .try_fold(0usize, |acc, &s| acc.checked_add(s))
      .ok_or(Error::AddrOverflow {
        addr: start,
        size: usize::MAX,
      })?;
    let mut pages = JIT_PAGES.lock();
    let page_addr = if total_size >= K_SPLIT_THRESHOLD {
      split_jit_page_locked(&mut pages, start, total_size)?
    } else {
      lookup_jit_page_locked(&mut pages, start, total_size)?
    };
    let mut page = JitPageRef::locked(&pages, page_addr);
    // K5: 注册表锁在此释放, 页锁由页引用贯穿两段式 (对齐原版
    // JitPageReference 持页锁贯穿 split/lookup 与逐条 RegisterAllocation)
    drop(pages);
    // K6 同族: 原版逐条 RegisterAllocation CHECK 失败即终止进程, 无残留
    // (cc:549-552); Rust 落 Result 后中途失败会留部分注册 → 先全量校验
    // (区间合法 / 不与既有分配重叠; 新分配互相衔接由零尺寸拒绝 + 游标
    // 单调递增保证), 全部通过后统一落表
    let mut cursor = start;
    for &sz in sizes {
      page.check_allocation(cursor, sz)?;
      cursor += sz;
    }
    let mut cursor = start;
    for &sz in sizes {
      page.insert_allocation(cursor, JitAllocation::new(sz, ty));
      cursor += sz;
    }
    Ok(())
  }

  /// 原版 RegisterJitAllocationForTesting (code-memory-access.cc:555-558)
  pub fn register_jit_allocation_for_testing(obj: usize, size: usize) -> Result<()> {
    Self::register_jit_allocation(obj, size, JitAllocationType::InstructionStream, false)?;
    Ok(())
  }

  /// 原版 UnregisterJitAllocationForTesting (code-memory-access.cc:561-564)
  pub fn unregister_jit_allocation_for_testing(addr: usize, size: usize) -> Result<()> {
    let mut pages = JIT_PAGES.lock();
    let page_addr = lookup_jit_page_locked(&mut pages, addr, size)?;
    let mut page = JitPageRef::locked(&pages, page_addr);
    drop(pages);
    page.unregister_allocation(addr)
  }

  /// 原版 UnregisterWasmAllocation (code-memory-access.cc:567-570)
  pub fn unregister_wasm_allocation(addr: usize, size: usize) -> Result<()> {
    let _scope = CFIMetadataWriteScope::new("UnregisterWasmAllocation");
    Self::unregister_jit_allocation_for_testing(addr, size)
  }

  /// 原版 StartOfJitAllocationAt (code-memory-access.cc:622-630) —
  /// 原 CHECK 失败 → None + error 日志 (登记)
  pub fn start_of_jit_allocation_at(inner_pointer: usize) -> Option<usize> {
    let _scope = CFIMetadataWriteScope::new("StartOfJitAllocationAt");
    let mut pages = JIT_PAGES.lock();
    // P2-2: 外层 Err (AddrOverflow — inner_pointer + 1 回绕; ZeroSize — 由
    // 常量尺寸 1 不触发, 防御保留) 与内层间隙路径同纪律: log::error + None,
    // 不再静默吞错
    let page_addr = match try_lookup_jit_page_locked(&mut pages, inner_pointer, 1) {
      Ok(Some(page_addr)) => page_addr,
      Ok(None) => return None,
      Err(e) => {
        log::error!("StartOfJitAllocationAt: {e}");
        return None;
      }
    };
    let page = JitPageRef::locked(&pages, page_addr);
    drop(pages);
    match page.start_of_allocation_at(inner_pointer) {
      Ok(start) => Some(start),
      Err(e) => {
        log::error!("StartOfJitAllocationAt: {e}");
        None
      }
    }
  }

  /// 原版 CanLookupStartOfJitAllocationAt (code-memory-access.cc:667-694) —
  /// 信号处理器内调用, 全链路 try_lock 防死锁
  pub fn can_lookup_start_of_jit_allocation_at(inner_pointer: usize) -> bool {
    let _scope = CFIMetadataWriteScope::new("CanLookupStartOfJitAllocationAt");
    // 原版 cc:670-677 — try 锁注册表防与持锁线程互等
    let Some(pages) = JIT_PAGES.try_lock() else {
      return false;
    };
    // upper_bound 前移一条 = 候选页, 上界页命中即可, 无页内含校验 (cc:679-686)
    let Some((_, page)) = pages.range(..=inner_pointer).next_back() else {
      return false;
    };
    // 原版 cc:688-693 — 页 try_lock 成功即真 (guard 随即释放);
    // K5: 页锁独立于注册表锁, 持页引用者只挡本页
    page.state.try_lock().is_some()
  }

  /// 原版 WriteProtectMemory (code-memory-access.cc:633-645)
  pub fn write_protect_memory(addr: usize, size: usize, _perms: PagePermissions) -> Result<bool> {
    if !Self::enabled() {
      return Ok(true);
    }
    Ok(pkey::set_permissions_and_key(
      addr,
      size,
      PROT_NONE,
      Self::pkey(),
    ))
  }

  /// 原版 LookupJumpTableAllocations (code-memory-access.cc:716-721)
  /// V8_ENABLE_WEBASSEMBLY 目标配置开, Rust 无条件编译 (登记)
  pub fn lookup_jump_table_allocations(
    jump_table_address: usize,
    jump_table_size: usize,
    far_jump_table_address: usize,
    far_jump_table_size: usize,
  ) -> Result<WritableJumpTablePair> {
    WritableJumpTablePair::lookup(
      jump_table_address,
      jump_table_size,
      far_jump_table_address,
      far_jump_table_size,
    )
  }
}

// MutexGuard 内表视图说明: check_for_region_overlap 直接取 &*pages

// ── WritableJitAllocation (code-memory-access.h:384-492, -inl.h:41-87, 103-278)

/// JIT 分配写通道 — 所有可执行内存写入须经此对象 (CFI 校验点)
///
/// 审查环 round1 (D3): 字段序对齐原版成员逆序析构 — 声明序 address_,
/// write_scope_, page_ref_, allocation_, enforce_write_api_ → 析构序
/// page_ref_ (放页锁) 先于 write_scope_ (SetExecutable)。Rust 按声明序
/// drop, 故 page_ref 声明在 _write_scope 之前。
pub struct WritableJitAllocation {
  address: usize,
  page_ref: Option<JitPageRef>,
  _write_scope: Option<RwxMemoryWriteScope>,
  allocation: JitAllocation,
  enforce_write_api: bool,
}

impl WritableJitAllocation {
  /// 原版私有构造 (code-memory-access-inl.h:51-74)
  fn with_page(
    addr: usize,
    size: usize,
    ty: JitAllocationType,
    source: JitAllocationSource,
    enforce_write_api: bool,
  ) -> Result<Self> {
    // 先建写作用域再查页 — 查页会取保护内存中的锁 (原版注释)
    let write_scope = RwxMemoryWriteScope::new("WritableJitAllocation");
    let mut page_ref = ThreadIsolation::lookup_jit_page(addr, size)?;
    let allocation = match source {
      JitAllocationSource::Register => page_ref.register_allocation(addr, size, ty)?,
      JitAllocationSource::Lookup => page_ref.lookup_allocation(addr, size, ty)?,
    };
    // 原版 -inl.h:64-69: debug + enforce 时构造即复位写作用域 (write_scope_.reset()),
    // 逐写经 write_scope_for_api_enforcement 建细粒度作用域; release 全程持权不变
    let write_scope = if cfg!(debug_assertions) && enforce_write_api {
      drop(write_scope);
      None
    } else {
      Some(write_scope)
    };
    Ok(Self {
      address: addr,
      page_ref: Some(page_ref),
      _write_scope: write_scope,
      allocation,
      enforce_write_api,
    })
  }

  /// 原版私有构造 (code-memory-access-inl.h:76-81) — 非可执行内存
  fn plain(addr: usize, allocation: JitAllocation, enforce_write_api: bool) -> Self {
    Self {
      address: addr,
      page_ref: None,
      _write_scope: None,
      allocation,
      enforce_write_api,
    }
  }

  /// 原版 ForNonExecutableMemory (code-memory-access-inl.h:84-87)
  pub fn for_non_executable_memory(addr: usize, size: usize, ty: JitAllocationType) -> Self {
    Self::plain(addr, JitAllocation::new(size, ty), false)
  }

  /// 原版 ForInstructionStream (code-memory-access.cc:697-703) —
  /// 原版入参 Tagged<InstructionStream>, 对象层 L3 接入前以 (地址, 尺寸) 承载
  pub fn for_instruction_stream(address: usize, size: usize) -> Result<Self> {
    let write_scope = RwxMemoryWriteScope::new("ForInstructionStream");
    let page_ref = ThreadIsolation::lookup_jit_page(address, size)?;
    let allocation =
      page_ref.lookup_allocation(address, size, JitAllocationType::InstructionStream)?;
    Ok(Self {
      address,
      page_ref: Some(page_ref),
      _write_scope: Some(write_scope),
      allocation,
      enforce_write_api: false,
    })
  }

  /// 原版 WriteScopeForApiEnforcement (code-memory-access-inl.h:103-111) —
  /// debug + enforce 时逐次开写作用域
  #[inline]
  fn write_scope_for_api_enforcement(&self) -> Option<RwxMemoryWriteScope> {
    if cfg!(debug_assertions) && self.enforce_write_api {
      return Some(RwxMemoryWriteScope::new("WriteScopeForApiEnforcement"));
    }
    None
  }

  /// 原版 address()
  pub const fn address(&self) -> usize {
    self.address
  }

  /// 原版 size()
  pub const fn size(&self) -> usize {
    self.allocation.size()
  }

  /// 原版 page_ref() — 原版为私有访问器 (code-memory-access.h:469),
  /// Rust 升 pub 供测试/L3 探测页引用 (登记: 访问级别放宽)
  pub fn page_ref(&self) -> Option<&JitPageRef> {
    self.page_ref.as_ref()
  }

  /// Rust 扩展访问器 (原版无对应, 登记) — 可变页引用的测试/L3 通道
  pub fn page_ref_mut(&mut self) -> Option<&mut JitPageRef> {
    self.page_ref.as_mut()
  }

  /// 原版 CopyCode (code-memory-access-inl.h:260-265) —
  /// base::CopyBytes → MemCopy 为 memmove 语义, 源/目的允许重叠
  /// (自修改代码搬运场景), 用 ptr::copy 对齐
  pub fn copy_code(&self, dst_offset: usize, src: &[u8]) {
    assert!(
      dst_offset
        .checked_add(src.len())
        .is_some_and(|end| end <= self.size()),
      "copy_code 越界",
    );
    let _scope = self.write_scope_for_api_enforcement();
    // SAFETY: 注册表保证 [address, address+size) 为本线程已解锁的可写 JIT 区;
    // ptr::copy 重叠安全, 与原版 memmove 一致
    unsafe {
      ptr::copy(
        src.as_ptr(),
        (self.address + dst_offset) as *mut u8,
        src.len(),
      );
    }
  }

  /// 原版 CopyData (code-memory-access-inl.h:267-272)
  pub fn copy_data(&self, dst_offset: usize, src: &[u8]) {
    self.copy_code(dst_offset, src);
  }

  /// 原版 ClearBytes (code-memory-access-inl.h:274-278)
  pub fn clear_bytes(&self, offset: usize, len: usize) {
    assert!(
      offset
        .checked_add(len)
        .is_some_and(|end| end <= self.size()),
      "clear_bytes 越界",
    );
    let _scope = self.write_scope_for_api_enforcement();
    // SAFETY: 注册表保证范围内可写
    unsafe {
      ptr::write_bytes((self.address + offset) as *mut u8, 0, len);
    }
  }

  /// 原版 WriteHeaderSlot<T, offset> 原始值分支 (code-memory-access-inl.h:147-161)
  /// — offset 模板参 (round2-F4) 以 const 泛型承载, 写点 address + OFFSET;
  /// Tagged/ReleaseStore/RelaxedStore/ProtectedPointer 变体依赖
  /// TaggedField/HeapObject → L3 接入 (登记)
  pub fn write_header_slot<T: Copy, const OFFSET: usize>(&self, value: T) {
    assert!(
      OFFSET
        .checked_add(size_of::<T>())
        .is_some_and(|end| end <= self.size()),
      "write_header_slot 越界",
    );
    let _scope = self.write_scope_for_api_enforcement();
    // SAFETY: 注册表保证 [address, address+size) 可写且容得下 OFFSET + T
    unsafe {
      ((self.address + OFFSET) as *mut T).write_unaligned(value);
    }
  }

  /// 原版 WriteUnalignedValue<T> (code-memory-access-inl.h:230-238)
  pub fn write_unaligned_value<T: Copy>(&self, address: usize, value: T) {
    assert!(address >= self.address, "write_unaligned_value 地址下溢");
    assert!(
      (address - self.address)
        .checked_add(size_of::<T>())
        .is_some_and(|end| end <= self.size()),
      "write_unaligned_value 越界",
    );
    let _scope = self.write_scope_for_api_enforcement();
    // SAFETY: 范围校验见上
    unsafe {
      (address as *mut T).write_unaligned(value);
    }
  }

  /// 原版 WriteValue<T> (code-memory-access-inl.h:240-247)
  pub fn write_value<T: Copy>(&self, address: usize, value: T) {
    assert!(address >= self.address, "write_value 地址下溢");
    assert!(
      (address - self.address)
        .checked_add(size_of::<T>())
        .is_some_and(|end| end <= self.size()),
      "write_value 越界",
    );
    let _scope = self.write_scope_for_api_enforcement();
    // SAFETY: 范围校验见上
    unsafe {
      (address as *mut T).write(value);
    }
  }

  /// 原版 WriteValue<T>(address, value, RelaxedStoreTag)
  /// (code-memory-access-inl.h:249-258) — `std::atomic<T>::store(relaxed)`
  /// 的 Rust 面: 1/2/4/8 字节经对应原子类型 (宿主天然原子宽度); 其余宽度
  /// 原版走 libatomic/库级锁, Rust 无对应 → 未对齐普通写 (登记: 目标调用面
  /// Tagged_t/Address 均为 4/8 字节, 该分支仅为完备性兜底)
  ///
  /// SAFETY: 调用方契约同原版 std::atomic<T> — address 须按 T 对齐且落在
  /// 注册表解锁的可写区内 (debug 构建对齐校验兜底)
  pub fn write_value_relaxed<T: Copy>(&self, address: usize, value: T) {
    assert!(address >= self.address, "write_value_relaxed 地址下溢");
    assert!(
      (address - self.address)
        .checked_add(size_of::<T>())
        .is_some_and(|end| end <= self.size()),
      "write_value_relaxed 越界",
    );
    let _scope = self.write_scope_for_api_enforcement();
    // SAFETY: 范围校验见上; 原子分支的对齐契约由 debug_assert 兜底
    unsafe {
      match size_of::<T>() {
        1 => {
          AtomicU8::from_ptr(address as *mut u8).store(
            ptr::read_unaligned(&value as *const T as *const u8),
            Ordering::Relaxed,
          );
        }
        2 => {
          debug_assert!(address & 1 == 0);
          AtomicU16::from_ptr(address as *mut u16).store(
            ptr::read_unaligned(&value as *const T as *const u16),
            Ordering::Relaxed,
          );
        }
        4 => {
          debug_assert!(address & 3 == 0);
          AtomicU32::from_ptr(address as *mut u32).store(
            ptr::read_unaligned(&value as *const T as *const u32),
            Ordering::Relaxed,
          );
        }
        8 => {
          debug_assert!(address & 7 == 0);
          AtomicU64::from_ptr(address as *mut u64).store(
            ptr::read_unaligned(&value as *const T as *const u64),
            Ordering::Relaxed,
          );
        }
        _ => (address as *mut T).write_unaligned(value),
      }
    }
  }
}

impl Drop for WritableJitAllocation {
  /// 原版析构 (code-memory-access-inl.h:41-49) — debug + enforce 时
  /// 析构体重开写作用域, 成员逆序析构中 page_ref_ (页锁位于保护内存,
  /// 解锁须持写权限) 先于重开的 write_scope_ 释放。Rust Drop 体先于字段
  /// drop 执行, 故页引用在体内显式 take + drop, 使其在写作用域存活期间
  /// 释放 (1:1 对齐原版 emplace 进成员的时序)
  fn drop(&mut self) {
    if cfg!(debug_assertions) && self.enforce_write_api && self.page_ref.is_some() {
      let _scope = RwxMemoryWriteScope::new("~WritableJitAllocation");
      drop(self.page_ref.take());
    }
  }
}

// 原版 UpdateWasmCodePointer (code-memory-access.cc:707-713) — wasm 代码指针表
// 属 v8-sandbox 层, L3+ 接入 (登记), 表落位后补方法。

// ── WritableJitPage (code-memory-access.h:536-554, -inl.h:280-297) ─────────

/// 整页写通道 — 一次性解锁, 免逐分配切换权限
///
/// 审查环 round1 (D3): 字段序对齐原版成员逆序析构 — 声明序 write_scope_,
/// page_ref_ → 析构序 page_ref_ (放页锁) 先于 write_scope_ (SetExecutable)。
pub struct WritableJitPage {
  page_ref: JitPageRef,
  _write_scope: RwxMemoryWriteScope,
}

impl WritableJitPage {
  /// 原版构造 (code-memory-access-inl.h:282-284)
  pub fn new(addr: usize, size: usize) -> Result<Self> {
    let write_scope = RwxMemoryWriteScope::new("WritableJitPage");
    let page_ref = ThreadIsolation::lookup_jit_page(addr, size)?;
    Ok(Self {
      page_ref,
      _write_scope: write_scope,
    })
  }

  /// 原版 LookupAllocationContaining (code-memory-access-inl.h:286-291) —
  /// 4 参构造形态, 不再加锁
  pub fn lookup_allocation_containing(&self, addr: usize) -> Result<WritableJitAllocation> {
    let (alloc_addr, alloc) = self.page_ref.allocation_containing(addr)?;
    Ok(WritableJitAllocation::plain(alloc_addr, alloc, false))
  }

  /// 原版 FreeRange (code-memory-access-inl.h:293-297) —
  /// 原版方法非 const (注销分配), 页锁 guard 已由页引用持有
  pub fn free_range(&mut self, addr: usize, size: usize) -> Result<WritableFreeSpace> {
    self.page_ref.unregister_range(addr, size)?;
    assert!(
      size <= i32::MAX as usize,
      "WritableFreeSpace 尺寸超出 i32 范围"
    );
    Ok(WritableFreeSpace {
      address: addr,
      size: size as i32,
      executable: true,
    })
  }

  /// 原版 Empty()
  pub fn empty(&self) -> bool {
    self.page_ref.empty()
  }

  /// Rust 扩展访问器 — 原版 page_ref_ 为私有成员 (code-memory-access.h:553),
  /// 无公开 getter; 供测试/L3 探测页状态 (登记: 虚构引证纠正, 审查环 round2 F5)
  pub fn page_ref(&self) -> &JitPageRef {
    &self.page_ref
  }

  /// Rust 扩展访问器 (原版无对应, 登记) — 页级注销操作的测试/L3 通道
  pub fn page_ref_mut(&mut self) -> &mut JitPageRef {
    &mut self.page_ref
  }

  /// 原版 UnregisterAllocationsExcept (code-memory-access.h:278)
  pub fn unregister_allocations_except(
    &mut self,
    start: usize,
    size: usize,
    keep: &[usize],
  ) -> Result<()> {
    self
      .page_ref
      .unregister_allocations_except(start, size, keep)
  }
}

// ── WritableFreeSpace (code-memory-access.h:499-529, -inl.h:299-321, cc:756-766)

/// 空闲区写通道
pub struct WritableFreeSpace {
  address: usize,
  size: i32,
  executable: bool,
}

impl WritableFreeSpace {
  /// 原版 ForNonExecutableMemory (code-memory-access-inl.h:301-305)
  pub fn for_non_executable_memory(addr: usize, size: usize) -> Self {
    assert!(
      size <= i32::MAX as usize,
      "WritableFreeSpace 尺寸超出 i32 范围"
    );
    Self {
      address: addr,
      size: size as i32,
      executable: false,
    }
  }

  /// 原版 Address()
  pub const fn address(&self) -> usize {
    self.address
  }

  /// 原版 Size()
  pub const fn size(&self) -> i32 {
    self.size
  }

  /// 原版 Executable()
  pub const fn executable(&self) -> bool {
    self.executable
  }

  /// 原版 WriteHeaderSlot<T, offset>(Tagged<T>, RelaxedStoreTag)
  /// (code-memory-access-inl.h:311-321) — offset 模板参 (round2-F4) 以
  /// const 泛型承载; TaggedField 依赖 L3, 原始值以未对齐写承载 (登记)
  pub fn write_header_slot<T: Copy, const OFFSET: usize>(&self, value: T) {
    assert!(
      self.size >= 0
        && OFFSET
          .checked_add(size_of::<T>())
          .is_some_and(|end| end <= self.size as usize),
      "write_header_slot 越界",
    );
    // SAFETY: 空闲区归调用方所有且已严格校验尺寸
    unsafe {
      ((self.address + OFFSET) as *mut T).write_unaligned(value);
    }
  }

  /// 原版 ClearTagged<offset>(count) (code-memory-access.cc:756-762) —
  /// 以 kClearedFreeMemoryValue 填充 count 个 tagged 槽
  pub fn clear_tagged<const OFFSET: usize>(&self, count: usize) {
    let bytes = count
      .checked_mul(K_TAGGED_SIZE)
      .expect("clear_tagged 乘法溢出");
    assert!(
      self.size >= 0
        && OFFSET
          .checked_add(bytes)
          .is_some_and(|end| end <= self.size as usize),
      "clear_tagged 越界",
    );
    let start = self.address + OFFSET;
    // 原版 MemsetTagged — kClearedFreeMemoryValue = 0 → 全零。
    // P2-3: 非 0 时原版按值填充 (MemsetTagged), Rust 仅实现全零路径 →
    // 编译期锁定常量, 常量变更即编译失败, 杜绝静默 no-op
    const _: () = assert!(K_CLEARED_FREE_MEMORY_VALUE == 0);
    // SAFETY: 空闲区归调用方所有, count * kTaggedSize 字节可写
    unsafe {
      ptr::write_bytes(start as *mut u8, 0, bytes);
    }
  }
}

// ── WritableJumpTablePair (code-memory-access.h:558-596, cc:716-752) ───────

/// wasm 双跳表写通道 — 跳表与远跳表需同时解锁
/// (V8_ENABLE_WEBASSEMBLY 目标配置开, 无条件编译, 登记)
///
/// 审查环 R1-D3: 字段序对齐原版成员逆序析构 — 声明序 writable_jump_table_,
/// writable_far_jump_table_, write_scope_, jump_table_pages_ (.h:587-593,
/// .h:584-586 注释明言顺序依赖) → 析构序 jump_table_pages_ (放双页锁) →
/// write_scope_ (SetExecutable 恢复权限) → far → jt; Rust 按声明序 drop,
/// 故声明 (pages, _write_scope, far_jump_table, jump_table)
pub struct WritableJumpTablePair {
  /// 双页引用各持本页页锁至析构 (K5, 对齐原版 -inl.h:126-131 / .h:591-593
  /// jump_table_pages_ 两 JitPageReference) — for_testing 非注册形态
  /// 无页引用 → None; 字段居首, 析构先放页锁, 与原版成员逆序析构一致
  jump_table_pages: Option<(JitPageRef, JitPageRef)>,
  /// 原版成员恒存在 (.h:590 non-optional); debug 构造后权限窗口经
  /// SetExecutable 闭合 (-inl.h:138-142), 析构经 SetWritable 重开 (cc:723-731)
  _write_scope: RwxMemoryWriteScope,
  far_jump_table: WritableJitAllocation,
  jump_table: WritableJitAllocation,
}

/// 切分后页必以 addr 为起点, 对该页做 Contains 校验
/// (原版 JitPageReference::Contains, code-memory-access.cc:359-365)
fn page_contains_locked(
  pages: &JitPageMap,
  addr: usize,
  size: usize,
  ty: JitAllocationType,
) -> bool {
  let Some(page) = pages.get(&addr) else {
    return false;
  };
  let state = page.state.lock();
  match state.allocations.get(&addr) {
    Some(a) => a.size() == size && a.ty() == ty,
    None => false,
  }
}

impl WritableJumpTablePair {
  /// 原版构造 (code-memory-access-inl.h:115-143) —
  /// 查找已注册分配语义: 调用方须先 RegisterJitAllocation 登记两个跳表
  /// (wasm-code-manager.cc:1763 先注册, :1809 再 LookupJumpTableAllocations),
  /// 此处 SplitJitPages 后逐表 CHECK(Contains(...)), 未注册/尺寸类型不符 → Err
  /// (原 CHECK 崩溃 → Result, 登记)。注册形态不另立 API: 注册是调用方职责。
  fn lookup(
    jump_table_address: usize,
    jump_table_size: usize,
    far_jump_table_address: usize,
    far_jump_table_size: usize,
  ) -> Result<Self> {
    let write_scope = RwxMemoryWriteScope::new("WritableJumpTablePair");
    // 原版 SplitJitPages(far, size, jt, size) — 自动换序保证升序 (cc:700-713);
    // 注册表锁贯穿双拆分 (原版 SplitJitPages 持 map 锁, cc:617-618)
    let mut pages = JIT_PAGES.lock();
    // CHECK_LE(addr1 + size1, addr2) — 无重叠
    let (addr1, size1, addr2, size2) = if jump_table_address > far_jump_table_address {
      (
        far_jump_table_address,
        far_jump_table_size,
        jump_table_address,
        jump_table_size,
      )
    } else {
      (
        jump_table_address,
        jump_table_size,
        far_jump_table_address,
        far_jump_table_size,
      )
    };
    if addr1.checked_add(size1).ok_or(Error::AddrOverflow {
      addr: addr1,
      size: size1,
    })?
      > addr2
    {
      return Err(Error::RegionOverlap {
        addr: addr1,
        size: size1,
      });
    }
    split_jit_page_locked(&mut pages, addr1, size1)?;
    split_jit_page_locked(&mut pages, addr2, size2)?;
    // 原版 -inl.h:131-136: CHECK(Contains(...)) — 分配必须已在注册表且
    // 地址/尺寸/类型逐一相符; 切分后页起点即分配起点, 按键直查
    if !page_contains_locked(
      &pages,
      jump_table_address,
      jump_table_size,
      JitAllocationType::WasmJumpTable,
    ) || !page_contains_locked(
      &pages,
      far_jump_table_address,
      far_jump_table_size,
      JitAllocationType::WasmFarJumpTable,
    ) {
      return Err(Error::JitAllocationMismatch {
        addr: jump_table_address,
        size: jump_table_size,
      });
    }
    // 原版成员走四参私有构造 (-inl.h:76-81): 不持页引用, 仅携分配元数据;
    // 双页引用 (各持页锁) 由本结构体持至析构 — 先构造引用再放注册表锁
    // (对齐原版 jump_table_pages_ = (far, jt) 序, -inl.h:128-130)
    let far_ref = JitPageRef::locked(&pages, far_jump_table_address);
    let jt_ref = JitPageRef::locked(&pages, jump_table_address);
    drop(pages);
    let jump_table = WritableJitAllocation::plain(
      jump_table_address,
      JitAllocation::new(jump_table_size, JitAllocationType::WasmJumpTable),
      true,
    );
    let far_jump_table = WritableJitAllocation::plain(
      far_jump_table_address,
      JitAllocation::new(far_jump_table_size, JitAllocationType::WasmFarJumpTable),
      true,
    );
    // 原版 -inl.h:138-142: DEBUG 下仅闭权限窗口 (write_scope_.SetExecutable()),
    // 作用域对象本身存活至析构; 逐写经分配对象的细粒度作用域
    if cfg!(debug_assertions) {
      RwxMemoryWriteScope::set_executable();
    }
    Ok(Self {
      jump_table_pages: Some((far_ref, jt_ref)),
      _write_scope: write_scope,
      far_jump_table,
      jump_table,
    })
  }

  /// 原版 jump_table()
  pub const fn jump_table(&self) -> &WritableJitAllocation {
    &self.jump_table
  }

  /// 原版 far_jump_table()
  pub const fn far_jump_table(&self) -> &WritableJitAllocation {
    &self.far_jump_table
  }

  /// 原版 ForTesting (code-memory-access.cc:745-752) — 非可执行形态
  pub fn for_testing(
    jump_table_address: usize,
    jump_table_size: usize,
    far_jump_table_address: usize,
    far_jump_table_size: usize,
  ) -> Self {
    let jump_table = WritableJitAllocation::for_non_executable_memory(
      jump_table_address,
      jump_table_size,
      JitAllocationType::WasmJumpTable,
    );
    let far_jump_table = WritableJitAllocation::for_non_executable_memory(
      far_jump_table_address,
      far_jump_table_size,
      JitAllocationType::WasmFarJumpTable,
    );
    Self {
      // ForTestingTag 构造 (cc:733-740) — 非注册形态, 无页引用
      jump_table_pages: None,
      jump_table,
      far_jump_table,
      _write_scope: RwxMemoryWriteScope::new("for testing"),
    }
  }
}

impl Drop for WritableJumpTablePair {
  /// 原版析构 (code-memory-access.cc:723-731) — debug + 持页引用时重开写权限
  /// 以释放页引用; Rust 字段按声明序 drop, 体内显式 take + drop 保证页引用
  /// 在写作用域存活期间释放 (1:1 对齐 WritableJitAllocation 及原版时序)
  fn drop(&mut self) {
    if cfg!(debug_assertions) && self.jump_table_pages.is_some() {
      let _scope = RwxMemoryWriteScope::new("~WritableJumpTablePair");
      drop(self.jump_table_pages.take());
    }
  }
}
