//! assert-scope 作用域进出配对校验
//!
//! PerThreadAssertScopeDebugOnly 在 release (DEBUG_BOOL=false) 下为空作用域
//! (原版 PerThreadAssertScopeEmpty, assert-scope.h:170-177) — 位不翻转。
//! 用例对 DebugOnly 作用域一律 profile 感知: debug 断言位翻转, release
//! 断言空作用域 no-op; 无条件行为用 *InRelease 变体承载。

use std::thread::scope;

use aok::{OK, Void};
use v8_common::{
  AllowCodeAllocation, AllowCodeDependencyChange, AllowGarbageCollection, AllowHandleAllocation,
  AllowHeapAllocation, DisallowCodeAllocation, DisallowCodeDependencyChange,
  DisallowGarbageCollection, DisallowGarbageCollectionInRelease, DisallowHandleAllocation,
  DisallowHeapAccess, DisallowHeapAllocation, K_INITIAL_VALUE, NoGarbageCollectionMutexGuard,
  PerIsolateAssertScope, PerIsolateAssertState, PerIsolateAssertType, PerThreadAsserts,
  per_thread_asserts_allow,
};

// [round3-F7] !Send 编译期锁 — 原版作用域删除拷贝构造且无移动构造
// (assert-scope.h:65-66), 栈绑定不可跨线程迁移; 若作用域载体重新派生
// Send, 下方断言即编译失败
static_assertions::assert_not_impl_all!(v8_common::DisallowGarbageCollection: Send);
static_assertions::assert_not_impl_all!(v8_common::DisallowGarbageCollectionInRelease: Send);
static_assertions::assert_not_impl_all!(v8_common::AllowGarbageCollection: Send);
static_assertions::assert_not_impl_all!(v8_common::AllowGarbageCollectionInRelease: Send);
static_assertions::assert_not_impl_all!(v8_common::AllowHeapAllocationInRelease: Send);
static_assertions::assert_not_impl_all!(v8_common::DisallowHeapAccess: Send);
static_assertions::assert_not_impl_all!(v8_common::DisallowHeapAccessIf: Send);
static_assertions::assert_not_impl_all!(v8_common::NoGarbageCollectionMutexGuard<'static, u32>: Send);

#[ctor::ctor(unsafe)]
fn _log_init() {
  log_init::init();
}

struct IsolateState {
  fields: [bool; 6],
}

impl PerIsolateAssertState for IsolateState {
  fn get(&self, ty: PerIsolateAssertType) -> bool {
    self.fields[ty as usize]
  }
  fn set(&mut self, ty: PerIsolateAssertType, enable: bool) {
    self.fields[ty as usize] = enable;
  }
}

/// 初始值: 除 HANDLE_USAGE_ON_ALL_THREADS 外全允许 (assert-scope.cc:17-18)
#[test]
fn assert_scope_initial_value() -> Void {
  assert_eq!(K_INITIAL_VALUE, u32::MAX - (1 << 5));
  // 标记位恒置
  assert_eq!(K_INITIAL_VALUE & 1, 1);
  // 所有类别默认允许
  assert!(per_thread_asserts_allow((1 << 1) | (1 << 2) | (1 << 3)));
  OK
}

/// scope 进出配对: Disallow 清位, 析构恢复
#[test]
fn assert_scope_disallow_allow_pairing() -> Void {
  const HEAP_ALLOC: PerThreadAsserts = 1 << 2;
  // 初始允许
  assert!(per_thread_asserts_allow(HEAP_ALLOC));
  if cfg!(debug_assertions) {
    {
      let _d = DisallowHeapAllocation::new();
      assert!(!per_thread_asserts_allow(HEAP_ALLOC));
      {
        // 嵌套例外: Allow 再放行
        let _a = AllowHeapAllocation::new();
        assert!(per_thread_asserts_allow(HEAP_ALLOC));
      }
      // 例外退出后恢复禁止
      assert!(!per_thread_asserts_allow(HEAP_ALLOC));
    }
    // 析构后恢复初始
    assert!(per_thread_asserts_allow(HEAP_ALLOC));

    const CODE_ALLOC: PerThreadAsserts = 1 << 7;
    {
      let _d = DisallowCodeAllocation::new();
      assert!(!per_thread_asserts_allow(CODE_ALLOC));
      // 其他位不受影响
      assert!(per_thread_asserts_allow(HEAP_ALLOC));
      let _a = AllowCodeAllocation::new();
      assert!(per_thread_asserts_allow(CODE_ALLOC));
    }
    assert!(per_thread_asserts_allow(CODE_ALLOC));
  } else {
    // release: DebugOnly = 空作用域, 位不动 (对齐原版 PerThreadAssertScopeEmpty)
    let _d = DisallowHeapAllocation::new();
    let _a = AllowHeapAllocation::new();
    assert!(per_thread_asserts_allow(HEAP_ALLOC));
  }
  OK
}

/// 组合位集: DisallowGarbageCollection = SAFEPOINTS | HEAP_ALLOCATION
#[test]
fn assert_scope_composite_bits() -> Void {
  const GC: PerThreadAsserts = (1 << 1) | (1 << 2);
  assert!(per_thread_asserts_allow(GC));
  if cfg!(debug_assertions) {
    {
      let _d = DisallowGarbageCollection::new();
      assert!(!per_thread_asserts_allow(GC));
      assert!(!per_thread_asserts_allow(1 << 1));
      assert!(!per_thread_asserts_allow(1 << 2));
      // 未覆盖位仍允许
      assert!(per_thread_asserts_allow(1 << 3));
    }
    assert!(per_thread_asserts_allow(GC));
    // AllowGarbageCollection 例外恢复
    {
      let _d = DisallowGarbageCollection::new();
      let _a = AllowGarbageCollection::new();
      assert!(per_thread_asserts_allow(GC));
    }
  } else {
    // release: DebugOnly 空作用域, 组合位不动
    let _d = DisallowGarbageCollection::new();
    assert!(per_thread_asserts_allow(GC));
  }
  // InRelease 变体不受 profile 影响 (原版 PerThreadAssertScope 直接实例化)
  {
    let _r = DisallowGarbageCollectionInRelease::new();
    assert!(!per_thread_asserts_allow(GC));
  }
  assert!(per_thread_asserts_allow(GC));
  OK
}

/// 多类型位集别名: DisallowHeapAccess 覆盖 4 位
#[test]
fn assert_scope_heap_access() -> Void {
  // CodeDependencyChange(6) | HandleDereference(4) | HandleAllocation(3) | HeapAllocation(2)
  const ACCESS: PerThreadAsserts = (1 << 6) | (1 << 4) | (1 << 3) | (1 << 2);
  if cfg!(debug_assertions) {
    {
      let _d = DisallowHeapAccess::new();
      assert!(!per_thread_asserts_allow(ACCESS));
      let _a = AllowCodeDependencyChange::new();
      // 单位例外不足以放行全部
      assert!(!per_thread_asserts_allow(ACCESS));
    }
    assert!(per_thread_asserts_allow(ACCESS));
    // HandleAllocation 单位例外由独立别名承载
    {
      let _d = DisallowHandleAllocation::new();
      assert!(!per_thread_asserts_allow(1 << 3));
      let _a = AllowHandleAllocation::new();
      assert!(per_thread_asserts_allow(1 << 3));
    }
  } else {
    // release: DebugOnly 空作用域
    let _d = DisallowHeapAccess::new();
    assert!(per_thread_asserts_allow(ACCESS));
  }
  OK
}

/// per-isolate 作用域: 构造保存旧值写 enable, 析构恢复
#[test]
fn assert_scope_per_isolate_pairing() -> Void {
  let mut isolate = IsolateState {
    fields: [true, true, true, true, true, true],
  };
  let ty = PerIsolateAssertType::JavascriptExecutionAssert;
  // RAII 进出配对: 作用域存活期间 isolate 被独占借用, 语义经边界读写校验
  {
    let _scope = PerIsolateAssertScope::<false>::new(&mut isolate, ty);
  }
  assert!(isolate.get(ty)); // 析构恢复
  // Allow 例外 (初值 true)
  {
    let _scope = PerIsolateAssertScope::<true>::new(&mut isolate, ty);
  }
  assert!(isolate.get(ty));
  // 先置 false, 验证 Allow 例外写回
  isolate.set(ty, false);
  {
    let _scope = PerIsolateAssertScope::<true>::new(&mut isolate, ty);
  }
  assert!(!isolate.get(ty)); // 恢复 false
  // Open/Close 静态语义 (assert-scope.cc:78-88)
  let was = PerIsolateAssertScope::<false>::open(&mut isolate, ty);
  assert!(!was); // Disallow 前为 false
  assert!(!PerIsolateAssertScope::<false>::is_allowed(&isolate, ty));
  PerIsolateAssertScope::<false>::close(&mut isolate, ty, was);
  assert!(!isolate.get(ty));
  // Allow 打开
  let was = PerIsolateAssertScope::<true>::open(&mut isolate, ty);
  assert!(!was);
  assert!(PerIsolateAssertScope::<true>::is_allowed(&isolate, ty));
  PerIsolateAssertScope::<true>::close(&mut isolate, ty, was);
  assert!(!isolate.get(ty));
  OK
}

/// 手动 Release 幂等: 提前恢复后 Drop 不再回写 (assert-scope.cc:50-55)
#[test]
fn assert_scope_release_idempotent() -> Void {
  const CODE_DEP: PerThreadAsserts = 1 << 6;
  let scope = DisallowCodeDependencyChange::new();
  if cfg!(debug_assertions) {
    assert!(!per_thread_asserts_allow(CODE_DEP));
    scope.release();
    assert!(per_thread_asserts_allow(CODE_DEP));
    scope.release();
    assert!(per_thread_asserts_allow(CODE_DEP));
    drop(scope);
    assert!(per_thread_asserts_allow(CODE_DEP));
  } else {
    // release: 空作用域 old_data 恒 K_CLEARED_VALUE, release 幂等 no-op
    scope.release();
    scope.release();
    drop(scope);
    assert!(per_thread_asserts_allow(CODE_DEP));
  }
  OK
}

/// [round3-F7] 跨线程隔离: 线程本位允许面互不串扰 (原版 thread_local,
/// assert-scope.cc:27) — A 线程作用域存续期间 B 线程位集不受影响;
/// 作用域 !Send (见 thread_bound_locks), 各线程只能作用自身 TLS
#[test]
fn assert_scope_cross_thread_isolation() -> Void {
  use std::sync::mpsc;
  const GC: PerThreadAsserts = (1 << 1) | (1 << 2);
  scope(|s| {
    let (a_ready_tx, a_ready_rx) = mpsc::channel::<()>();
    let (b_done_tx, b_done_rx) = mpsc::channel::<()>();
    s.spawn(move || {
      let _d = DisallowGarbageCollectionInRelease::new();
      assert!(!per_thread_asserts_allow(GC));
      a_ready_tx.send(()).unwrap();
      b_done_rx.recv().unwrap();
      // B 校验期间 A 的禁止仍生效
      assert!(!per_thread_asserts_allow(GC));
    });
    s.spawn(move || {
      a_ready_rx.recv().unwrap();
      // A 线程禁止期间, 本线程允许面不受影响
      assert!(per_thread_asserts_allow(GC));
      b_done_tx.send(()).unwrap();
    });
  });
  // 全线程退出后本线程仍允许
  assert!(per_thread_asserts_allow(GC));
  OK
}

/// [回归 round2-F6] NoGarbageCollectionMutexGuard::new 为**加锁**构造
/// (assert-scope.h:302-305 `guard_(mutex)`), 非接管已持锁;
/// Unlock/Lock 支持中途放锁/重锁周期 (assert-scope.h:307-314)
#[test]
fn assert_scope_no_gc_mutex_guard_lock_cycle() -> Void {
  use parking_lot::Mutex;

  let mutex = Mutex::new(7u32);
  let mut guard = NoGarbageCollectionMutexGuard::new(&mutex);
  // 构造即持锁: 他方 try_lock 必失败
  assert!(mutex.try_lock().is_none());
  guard.unlock();
  {
    let mut v = mutex.try_lock().expect("unlock 后必须可竞争");
    *v += 1;
  }
  // [round5-F10] Lock() 无参重锁构造时锁定的同一把锁 (assert-scope.h:311-314)
  guard.lock();
  assert!(mutex.try_lock().is_none());
  drop(guard);
  assert_eq!(*mutex.lock(), 8);
  OK
}

/// [回归 round5-F10] from_guard 接管路径: 父锁引用经 MutexGuard::mutex()
/// 恢复, unlock/lock 周期仍作用于同一把锁
#[test]
fn assert_scope_no_gc_mutex_guard_from_guard_lock_same_mutex() -> Void {
  use parking_lot::Mutex;

  let mutex = Mutex::new(1u32);
  let pre = mutex.lock();
  let mut guard = NoGarbageCollectionMutexGuard::from_guard(pre);
  assert!(mutex.try_lock().is_none());
  guard.unlock();
  drop(mutex.try_lock());
  guard.lock();
  assert!(mutex.try_lock().is_none());
  drop(guard);
  assert_eq!(*mutex.lock(), 1);
  OK
}

/// [round5] 同类型嵌套栈序压力: 同一位 (HEAP_ALLOCATION) 上 Allow→Disallow
/// 交替深嵌套, 逐层校验位面, 逆序逐层恢复 (原版 old_data_ 链, assert-scope.cc:
/// 31-55); 含中途 manual release 的层级重建与 64 轮循环交替
#[test]
fn assert_scope_nested_stack_order_stress() -> Void {
  const HEAP_ALLOC: PerThreadAsserts = 1 << 2;
  // InRelease 变体双 profile 恒活, 压力不随构建配置失真
  use v8_common::{
    AllowGarbageCollectionInRelease as Allow, DisallowGarbageCollectionInRelease as Disallow,
  };
  // 1) 手写 10 层交替嵌套: 进栈逐层断言, 收括号即逐层恢复
  {
    let _l0 = Allow::new(); // 初态允许, Allow 幂等
    assert!(per_thread_asserts_allow(HEAP_ALLOC));
    {
      let _l1 = Disallow::new();
      assert!(!per_thread_asserts_allow(HEAP_ALLOC));
      {
        let _l2 = Allow::new();
        assert!(per_thread_asserts_allow(HEAP_ALLOC));
        {
          let _l3 = Disallow::new();
          assert!(!per_thread_asserts_allow(HEAP_ALLOC));
          {
            let _l4 = Allow::new();
            assert!(per_thread_asserts_allow(HEAP_ALLOC));
            {
              let _l5 = Disallow::new();
              assert!(!per_thread_asserts_allow(HEAP_ALLOC));
              {
                let _l6 = Allow::new();
                assert!(per_thread_asserts_allow(HEAP_ALLOC));
                {
                  let _l7 = Disallow::new();
                  assert!(!per_thread_asserts_allow(HEAP_ALLOC));
                  {
                    let _l8 = Allow::new();
                    assert!(per_thread_asserts_allow(HEAP_ALLOC));
                    {
                      let _l9 = Disallow::new();
                      assert!(!per_thread_asserts_allow(HEAP_ALLOC));
                    }
                    // L9 退出 → 恢复 L8 的允许
                    assert!(per_thread_asserts_allow(HEAP_ALLOC));
                  }
                  assert!(!per_thread_asserts_allow(HEAP_ALLOC));
                }
                assert!(per_thread_asserts_allow(HEAP_ALLOC));
              }
              assert!(!per_thread_asserts_allow(HEAP_ALLOC));
            }
            assert!(per_thread_asserts_allow(HEAP_ALLOC));
          }
          assert!(!per_thread_asserts_allow(HEAP_ALLOC));
        }
        assert!(per_thread_asserts_allow(HEAP_ALLOC));
      }
      assert!(!per_thread_asserts_allow(HEAP_ALLOC));
    }
    assert!(per_thread_asserts_allow(HEAP_ALLOC));
  }
  assert!(per_thread_asserts_allow(HEAP_ALLOC));

  // 2) 中途 manual release: B 层提前恢复后, 其上新建层级以恢复后的值为基,
  //    析构序仍与原版 old_data_ 链一致 (assert-scope.cc:50-55 幂等)
  {
    let a = Disallow::new();
    assert!(!per_thread_asserts_allow(HEAP_ALLOC));
    let b = Allow::new();
    assert!(per_thread_asserts_allow(HEAP_ALLOC));
    b.release();
    assert!(!per_thread_asserts_allow(HEAP_ALLOC)); // release 恢复 a 的禁止
    b.release(); // 幂等: old_data 已清, 不再回写
    assert!(!per_thread_asserts_allow(HEAP_ALLOC));
    {
      // B 已 release, 本层 old_data = 禁止态; 退出恢复禁止 (非允许)
      let _c = Allow::new();
      assert!(per_thread_asserts_allow(HEAP_ALLOC));
    }
    assert!(!per_thread_asserts_allow(HEAP_ALLOC));
    drop(b); // 已 release, Drop 无二次回写
    assert!(!per_thread_asserts_allow(HEAP_ALLOC));
    drop(a);
    assert!(per_thread_asserts_allow(HEAP_ALLOC));
  }

  // 3) 64 轮循环交替: 每轮进出配对, 期望面以奇偶建模
  for i in 0..64u32 {
    if i % 2 == 0 {
      let _d = Disallow::new();
      assert!(!per_thread_asserts_allow(HEAP_ALLOC), "轮 {i}");
      // 嵌套一层例外再退出, 双重配对
      {
        let _a = Allow::new();
        assert!(per_thread_asserts_allow(HEAP_ALLOC), "轮 {i}");
      }
      assert!(!per_thread_asserts_allow(HEAP_ALLOC), "轮 {i}");
    } else {
      let _a = Allow::new();
      assert!(per_thread_asserts_allow(HEAP_ALLOC), "轮 {i}");
    }
    assert!(per_thread_asserts_allow(HEAP_ALLOC), "轮 {i} 收尾");
  }
  OK
}
