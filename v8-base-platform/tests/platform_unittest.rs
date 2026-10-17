//! platform-unittest.cc 1:1 迁移（darwin 可见部分）。规约见 tests/SPEC.md。
// 原版 C++ `for (int i = 0; ...)` 下标循环按 1:1 直译，静默对应风格 lint（不影响断言）。
use std::{
  cell::UnsafeCell, ffi::c_void, hint::black_box, process::id, ptr::null_mut,
  slice::from_raw_parts, sync::Arc,
};

use aok::{OK, Void};
use log::info;
use v8_base_platform::{
  LocalStorageKey, MemoryPermission, OS, Options, Stack, StackSlot, Thread,
  create_thread_local_key, delete_thread_local_key, get_existing_thread_local, get_thread_local,
  has_thread_local, set_thread_local,
};

#[ctor::ctor(unsafe)]
fn _log_init() {
  log_init::init();
}

// cc:29-43: Windows 默认页大小 4096, 非 win/ppc64 分支 kMaxPageSize = 16384
#[cfg(target_os = "windows")]
const K_MAX_PAGE_SIZE: usize = 4096;
#[cfg(not(target_os = "windows"))]
const K_MAX_PAGE_SIZE: usize = 16384;

#[cfg_attr(target_os = "windows", repr(C, align(4096)))]
#[cfg_attr(not(target_os = "windows"), repr(C, align(16384)))]
struct PageAligned([u8; K_MAX_PAGE_SIZE]);

// cc:45-47 alignas(kMaxPageSize) const char kArray[kMaxPageSize] = "Lorem ...";
const K_TEXT: &[u8] =
  b"Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua.";
static K_ARRAY: PageAligned = {
  let mut a = [0u8; K_MAX_PAGE_SIZE]; // 原版 char 数组剩余部分补零
  let mut i = 0;
  while i < K_TEXT.len() {
    a[i] = K_TEXT[i];
    i += 1;
  }
  PageAligned(a)
};

// TEST(OS, GetCurrentProcessId) cc:49-58（V8_OS_POSIX 分支）
#[test]
fn os_get_current_process_id() -> Void {
  info!("> os_get_current_process_id");
  // cc:51 EXPECT_EQ(static_cast<int>(getpid()), OS::GetCurrentProcessId())
  assert_eq!(id() as i32, OS::get_current_process_id());
  OK
}

// TEST(OS, RemapPages) cc:60-78
#[test]
fn os_remap_pages() -> Void {
  if OS::is_remap_page_supported() {
    // cc:62 const size_t size = OS::AllocatePageSize();
    let size = OS::allocate_page_size();
    assert!(
      size <= K_MAX_PAGE_SIZE,
      "cc:63 ASSERT_TRUE(size <= kMaxPageSize)"
    ); // cc:63
    let data = &K_ARRAY.0 as *const u8 as *const c_void; // cc:64

    // cc:67-69 Target mapping.
    let remapped = unsafe {
      OS::allocate(
        null_mut(),
        size,
        OS::allocate_page_size(),
        MemoryPermission::ReadWrite,
      )
    };
    assert!(remapped.is_ok(), "cc:70 ASSERT_TRUE(remapped_data)");
    let remapped_data = remapped.unwrap(); // is_ok 后 100% 安全

    // cc:72-73 EXPECT_TRUE(OS::RemapPages(...kReadExecute))
    let remap_ok = unsafe {
      OS::remap_pages(
        data as *mut c_void,
        size,
        remapped_data,
        MemoryPermission::ReadExecute,
      )
    };
    assert!(remap_ok.is_ok(), "cc:72 EXPECT_TRUE(RemapPages)");

    // cc:74 EXPECT_EQ(0, memcmp(remapped_data, data, size))
    let remapped_bytes = unsafe { from_raw_parts(remapped_data as *const u8, size) };
    let data_bytes = unsafe { from_raw_parts(data as *const u8, size) };
    assert!(remapped_bytes.cmp(data_bytes).is_eq(), "cc:74 memcmp == 0");

    // cc:76 OS::Free(remapped_data, size)
    assert!(unsafe { OS::free(remapped_data, size) }.is_ok());
  }
  OK
}

// cc:81-107 [SKIP] OS.GetSharedLibraryAddresses —— V8_TARGET_OS_LINUX 专属, darwin 原版不编译
// cc:109-138 [SKIP] OS.SignalSafeMapsParserForCurrentProcess —— 同上 Linux-only
// cc:142-239 [SKIP] OS.SignalSafeMapsParserForCustomMapsFile —— Linux-only + makedev/proc maps
// cc:241-270 [SKIP] OS.SignalSafeMapsParserCustomName —— Linux-only + VirtualAddressSpace

// Rust 侧强化补充（原版 cc 无对应用例）：回收的 OS RNG 入口
// （platform-posix.cc:355-360 SetRandomMmapSeed / cc:363-395 GetRandomMmapAddr）
// 与 gc_fake_mmap 存取（cc:276-278 / cc:785-787）。
#[test]
fn os_random_mmap_and_fake_mmap() -> Void {
  info!("> os_random_mmap_and_fake_mmap");
  // 同种子 → 同地址序列（确定性，无随机源断言）
  OS::set_random_mmap_seed(42);
  let a1 = OS::get_random_mmap_addr() as usize;
  let a2 = OS::get_random_mmap_addr() as usize;
  OS::set_random_mmap_seed(42);
  assert_eq!(a1, OS::get_random_mmap_addr() as usize, "seed 后序列可复现");
  assert_eq!(a2, OS::get_random_mmap_addr() as usize, "第二地址可复现");
  // 掩码 + 页对齐（原版 cc:388-395 截断规则）
  #[cfg(any(
    target_arch = "x86_64",
    all(target_arch = "aarch64", target_os = "macos")
  ))]
  const ADDR_MASK: usize = 0x3FFF_FFFF_F000;
  #[cfg(all(target_arch = "aarch64", target_os = "linux"))]
  const ADDR_MASK: usize = 0x3F_FFFF_F000;
  let page = OS::allocate_page_size();
  for a in [a1, a2] {
    assert_eq!(a & (page - 1), 0, "hint 地址页对齐");
    #[cfg(any(target_arch = "x86_64", target_arch = "aarch64"))]
    assert_eq!(a & !ADDR_MASK, 0, "hint 地址落在掩码内");
  }
  // initialize/get_gc_fake_mmap_file 往返
  OS::initialize(Some("/tmp/v8rs-gc-fake-mmap"));
  assert_eq!(OS::get_gc_fake_mmap_file(), Some("/tmp/v8rs-gc-fake-mmap"));
  OS::initialize(None);
  assert_eq!(OS::get_gc_fake_mmap_file(), None);
  OK
}

/// 原版 fixture ThreadLocalStorageTest (cc:275-322)。
struct TlsKeys(UnsafeCell<Vec<LocalStorageKey>>);
// SAFETY: 主线程与 worker 的访问遵循原版构造→Run→Join→析构的时序, 无并发窗口。
unsafe impl Sync for TlsKeys {}

// Run() cc:288-314；在当前线程执行原版全部 CHECK。
fn run_tls_checks(keys: &[LocalStorageKey]) {
  const K_SIZE: usize = 32; // keys_[32] cc:321
  assert_eq!(keys.len(), K_SIZE);
  for &key in keys {
    assert!(!has_thread_local(key), "cc:290 CHECK(!HasThreadLocal)");
  }
  for (i, &key) in keys.iter().enumerate() {
    unsafe { set_thread_local(key, (i + 1) as *mut c_void) }; // GetValue(i) = i+1, cc:317
  }
  for &key in keys {
    assert!(has_thread_local(key), "cc:296 CHECK(HasThreadLocal)");
  }
  for (i, &key) in keys.iter().enumerate() {
    assert_eq!(i + 1, get_thread_local(key) as usize, "cc:299 CHECK_EQ");
    assert_eq!(
      i + 1,
      get_existing_thread_local(key) as usize,
      "cc:300 CHECK_EQ"
    );
  }
  for (i, &key) in keys.iter().enumerate() {
    unsafe { set_thread_local(key, (K_SIZE - i) as *mut c_void) }; // GetValue(31-i), cc:303
  }
  for &key in keys {
    assert!(has_thread_local(key), "cc:306 CHECK(HasThreadLocal)");
  }
  for (i, &key) in keys.iter().enumerate() {
    assert_eq!(
      K_SIZE - i,
      get_thread_local(key) as usize,
      "cc:309 CHECK_EQ(GetValue(arraysize-i-1), GetThreadLocal)"
    );
    assert_eq!(
      K_SIZE - i,
      get_existing_thread_local(key) as usize,
      "cc:310 CHECK_EQ(GetValue(arraysize-i-1), GetExistingThreadLocal)"
    );
  }
}

// TEST_F(ThreadLocalStorageTest, DoTest) cc:326-330
#[test]
fn thread_local_storage_test_do_test() -> Void {
  // fixture 构造 cc:277-281: 32 个 key
  let mut keys = Vec::with_capacity(32);
  for _ in 0..32 {
    let k = create_thread_local_key();
    assert!(k.is_ok(), "CreateThreadLocalKey 应成功");
    keys.push(k.unwrap());
  }
  let keys = Arc::new(TlsKeys(UnsafeCell::new(keys)));

  // cc:327 Run() —— 先在当前(主)线程执行
  run_tls_checks(unsafe { &*keys.0.get() });

  // cc:328 CHECK(Start()) —— worker 内再执行 Run()
  let worker_keys = Arc::clone(&keys);
  let mut thread = Thread::new(Options::new("ThreadLocalStorageTest"), move || {
    run_tls_checks(unsafe { &*worker_keys.0.get() });
  });
  assert!(thread.start().is_ok(), "cc:328 CHECK(Start())");
  // cc:329 Join()；worker 断言 panic → join Err → 此处红
  assert!(
    thread.join().is_ok(),
    "cc:329 Join: worker CHECK 应全部成立"
  );

  // fixture 析构 cc:282-286
  for &k in unsafe { &*keys.0.get() } {
    delete_thread_local_key(k);
  }
  OK
}

// TEST(StackTest, GetStackStart) cc:332
#[test]
fn stack_test_get_stack_start() -> Void {
  assert_ne!(
    Stack::get_stack_start().value(),
    0,
    "cc:332 EXPECT_NE(nullptr, GetStackStart())"
  );
  OK
}

// TEST(StackTest, GetCurrentStackPosition) cc:334-336
#[test]
fn stack_test_get_current_stack_position() -> Void {
  assert_ne!(
    Stack::get_current_stack_position().value(),
    0,
    "cc:335 EXPECT_NE(nullptr, GetCurrentStackPosition())"
  );
  OK
}

// TEST(StackTest, StackVariableInBounds) cc:339-347
#[test]
fn stack_test_stack_variable_in_bounds() -> Void {
  let mut dummy: u8 = 0; // 原版 void* dummy; 取其地址
  black_box(&mut dummy);
  let start = Stack::get_stack_start().value();
  let current = Stack::get_current_stack_position().value();
  let slot =
    Stack::get_real_stack_address_for_slot(StackSlot::from_ptr(&dummy as *const u8 as *const ()))
      .value();
  assert!(
    start >= current,
    "cc:341 ASSERT_GE(GetStackStart(), GetCurrentStackPosition())"
  );
  assert!(start >= slot, "cc:343 EXPECT_GE(GetStackStart(), slot)");
  assert!(
    current <= slot,
    "cc:345 EXPECT_LE(GetCurrentStackPosition(), slot)"
  );
  OK
}

// TEST_F(SetDataReadOnlyTest, SetDataReadOnly) cc:352-369
// [SKIP] 含 ASSERT_DEATH_IF_SUPPORTED x2 (cc:367-368): 写只读页需真实 SIGSEGV, glossary 规定跳过;
// 且对本测试进程静态页 mprotect 会危及同页其他静态数据。理由已登记 SPEC.md 跳过表。

// TEST_F(PlatformTest, StackAlignment) cc:387-410
// [SKIP] 依赖 v8::Isolate / JS 执行 (v8::TestWithIsolate), 非 L0 v8-base-platform 范围。已登记。
