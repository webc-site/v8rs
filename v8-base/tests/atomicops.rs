//! v8/test/unittests/base/atomicops-unittest.cc 的规约迁移
//!
//! 规约清单:
//! - TestAtomicIncrement (32/Word): +1/+2/+3/-3/-2/-1/-1/-4/+5 序列,
//!   邻接 prev_word/next_word 守卫值全程不变
//! - TestCompareAndSwap: 失败返回现值 / 成功改写 (含高低半字位型测试值)
//! - TestAtomicExchange: 换出旧值
//! - TestAtomicIncrementBounds: 32 位边界 (1<<16) 上下穿越
//! - TestStore (8/32/Word): Relaxed/Release 写 0xA5 底纹与 -1
//! - TestLoad (8/32/Word): Relaxed/Acquire 读回
//! - Relaxed_Memmove: 前向/后向重叠复制
//! - Relaxed_Memcmp: 等长 0 / 升序 <0 / 降序 >0 (全偏移扫描)
//!   + 补充高位字节 (≥0x80) 无符号序断言 (锁定 word 路径修复)

use std::{
  mem, ptr,
  sync::atomic::{AtomicI8, AtomicI32, AtomicI64},
};

use aok::{OK, Void};
use v8_base::atomicops::{Atomic8, AtomicInt, relaxed_memcmp, relaxed_memmove};

#[ctor::ctor(unsafe)]
fn _log_init() {
  log_init::init();
}

/// memset(pattern) 的等值: 按 width 字节铺 pattern 后按有符号宽度符号扩展
/// (与经窄型原子 store→load 往返后的观测值一致, 对应原版 int 直接比较)
fn fill(pattern: u8, width: usize) -> i64 {
  let mut v = 0u64;
  for _ in 0..width {
    v = (v << 8) | pattern as u64;
  }
  let bits = width * 8;
  ((v << (64 - bits)) as i64) >> (64 - bits)
}

/// 三槽守卫结构 (对应原版 struct { prev_word; count; next_word; })
struct Guarded<A> {
  prev_word: A,
  count: A,
  next_word: A,
}

fn test_atomic_increment32() {
  let s = Guarded {
    prev_word: AtomicI32::new(fill(0xFF, 4) as i32),
    count: AtomicI32::new(0),
    next_word: AtomicI32::new(fill(0xEE, 4) as i32),
  };
  let (prev_v, next_v) = (s.prev_word.relaxed_load(), s.next_word.relaxed_load());
  for (inc, after) in [
    (1, 1),
    (2, 3),
    (3, 6),
    (-3, 3),
    (-2, 1),
    (-1, 0),
    (-1, -1),
    (-4, -5),
    (5, 0),
  ] {
    assert_eq!(after, s.count.relaxed_atomic_increment(inc));
    assert_eq!(after, s.count.relaxed_load());
    assert_eq!(prev_v, s.prev_word.relaxed_load());
    assert_eq!(next_v, s.next_word.relaxed_load());
  }
}

fn test_atomic_increment_word() {
  let s = Guarded {
    prev_word: AtomicI64::new(fill(0xFF, 8)),
    count: AtomicI64::new(0),
    next_word: AtomicI64::new(fill(0xEE, 8)),
  };
  let (prev_v, next_v) = (s.prev_word.relaxed_load(), s.next_word.relaxed_load());
  for (inc, after) in [
    (1, 1),
    (2, 3),
    (3, 6),
    (-3, 3),
    (-2, 1),
    (-1, 0),
    (-1, -1),
    (-4, -5),
    (5, 0),
  ] {
    assert_eq!(after, s.count.relaxed_atomic_increment(inc));
    assert_eq!(after, s.count.relaxed_load());
    assert_eq!(prev_v, s.prev_word.relaxed_load());
    assert_eq!(next_v, s.next_word.relaxed_load());
  }
}

/// TEST(Atomicops, AtomicIncrement)
#[test]
fn atomicops_atomic_increment() -> Void {
  test_atomic_increment32();
  test_atomic_increment_word();
  OK
}

fn test_cas<A: AtomicCtor>(half_bits: u32) {
  let value = A::default_zero();
  let prev = value.relaxed_compare_and_swap(0, 1);
  assert_eq!(1, value.relaxed_load());
  assert_eq!(0, prev);

  // 高低半字同时非零的测试值
  let k_test_val = (1i64 << (half_bits - 2)) + 11;
  value.relaxed_store(k_test_val);
  let prev = value.relaxed_compare_and_swap(0, 5);
  assert_eq!(k_test_val, value.relaxed_load());
  assert_eq!(k_test_val, prev);

  value.relaxed_store(k_test_val);
  let prev = value.relaxed_compare_and_swap(k_test_val, 5);
  assert_eq!(5, value.relaxed_load());
  assert_eq!(k_test_val, prev);
}

/// AtomicInt 以具体 std 原子类型实现, 无 Default, 以本 trait 提供零值构造
trait AtomicCtor: AtomicInt + Sized {
  fn default_zero() -> Self;
}
impl AtomicCtor for AtomicI32 {
  fn default_zero() -> Self {
    Self::new(0)
  }
}
impl AtomicCtor for AtomicI64 {
  fn default_zero() -> Self {
    Self::new(0)
  }
}

/// TEST(Atomicops, CompareAndSwap)
#[test]
fn atomicops_compare_and_swap() -> Void {
  test_cas::<AtomicI32>(32);
  test_cas::<AtomicI64>(64);
  OK
}

fn test_exchange<A: AtomicCtor>(half_bits: u32) {
  let value = A::default_zero();
  let new_value = value.relaxed_exchange(1);
  assert_eq!(1, value.relaxed_load());
  assert_eq!(0, new_value);

  let k_test_val = (1i64 << (half_bits - 2)) + 11;
  value.relaxed_store(k_test_val);
  let new_value = value.relaxed_exchange(k_test_val);
  assert_eq!(k_test_val, value.relaxed_load());
  assert_eq!(k_test_val, new_value);

  value.relaxed_store(k_test_val);
  let new_value = value.relaxed_exchange(5);
  assert_eq!(5, value.relaxed_load());
  assert_eq!(k_test_val, new_value);
}

/// TEST(Atomicops, AtomicExchange)
#[test]
fn atomicops_atomic_exchange() -> Void {
  test_exchange::<AtomicI32>(32);
  test_exchange::<AtomicI64>(64);
  OK
}

/// TEST(Atomicops, AtomicIncrementBounds)
#[test]
fn atomicops_atomic_increment_bounds() -> Void {
  {
    let value = AtomicI32::new((1 << 16) - 1);
    let new_value = value.relaxed_atomic_increment(1);
    assert_eq!(1 << 16, value.relaxed_load());
    assert_eq!(value.relaxed_load(), new_value);
    value.relaxed_atomic_increment(-1);
    assert_eq!((1 << 16) - 1, value.relaxed_load());
  }
  {
    let value = AtomicI64::new((1 << 32) - 1);
    let new_value = value.relaxed_atomic_increment(1);
    assert_eq!(1 << 32, value.relaxed_load());
    assert_eq!(value.relaxed_load(), new_value);
    value.relaxed_atomic_increment(-1);
    assert_eq!((1 << 32) - 1, value.relaxed_load());
  }
  OK
}

/// TEST(Atomicops, Store) — Relaxed/Release × 0xA5 底纹 / -1
#[test]
fn atomicops_store() -> Void {
  {
    let k_val1 = fill(0xA5, 1);
    let k_val2 = -1;
    let value = AtomicI8::new(0);
    value.relaxed_store(k_val1);
    assert_eq!(k_val1, value.relaxed_load());
    value.relaxed_store(k_val2);
    assert_eq!(k_val2, value.relaxed_load());
  }
  {
    let k_val1 = fill(0xA5, 4);
    let k_val2 = -1;
    let value = AtomicI32::new(0);
    value.relaxed_store(k_val1);
    assert_eq!(k_val1, value.relaxed_load());
    value.relaxed_store(k_val2);
    assert_eq!(k_val2, value.relaxed_load());
    value.release_store(k_val1);
    assert_eq!(k_val1, value.relaxed_load());
    value.release_store(k_val2);
    assert_eq!(k_val2, value.relaxed_load());
  }
  {
    let k_val1 = fill(0xA5, 8);
    let k_val2 = -1;
    let value = AtomicI64::new(0);
    value.relaxed_store(k_val1);
    assert_eq!(k_val1, value.relaxed_load());
    value.relaxed_store(k_val2);
    assert_eq!(k_val2, value.relaxed_load());
    value.release_store(k_val1);
    assert_eq!(k_val1, value.relaxed_load());
    value.release_store(k_val2);
    assert_eq!(k_val2, value.relaxed_load());
  }
  OK
}

/// TEST(Atomicops, Load) — Relaxed/Acquire 读回
#[test]
fn atomicops_load() -> Void {
  {
    let k_val1 = fill(0xA5, 1);
    let k_val2 = -1;
    let value = AtomicI8::new(k_val1 as i8);
    assert_eq!(k_val1, value.relaxed_load());
    let value = AtomicI8::new(k_val2 as i8);
    assert_eq!(k_val2, value.relaxed_load());
  }
  for width in [4usize, 8] {
    let k_val1 = fill(0xA5, width);
    let k_val2 = -1i64;
    match width {
      4 => {
        let value = AtomicI32::new(k_val1 as i32);
        assert_eq!(k_val1, value.relaxed_load());
        let value = AtomicI32::new(k_val2 as i32);
        assert_eq!(k_val2, value.relaxed_load());
        let value = AtomicI32::new(k_val1 as i32);
        assert_eq!(k_val1, value.acquire_load());
        let value = AtomicI32::new(k_val2 as i32);
        assert_eq!(k_val2, value.acquire_load());
      }
      _ => {
        let value = AtomicI64::new(k_val1);
        assert_eq!(k_val1, value.relaxed_load());
        let value = AtomicI64::new(k_val2);
        assert_eq!(k_val2, value.relaxed_load());
        let value = AtomicI64::new(k_val1);
        assert_eq!(k_val1, value.acquire_load());
        let value = AtomicI64::new(k_val2);
        assert_eq!(k_val2, value.acquire_load());
      }
    }
  }
  OK
}

/// TEST(Atomicops, Relaxed_Memmove)
#[test]
fn atomicops_relaxed_memmove() -> Void {
  const LEN: usize = 6;
  {
    let mut arr: [Atomic8; LEN] = [0, 1, 2, 3, 4, 5];
    // SAFETY: arr 栈上存活; dst=arr+2 与 src=arr+3 重叠, 走后向复制
    unsafe {
      relaxed_memmove(
        ptr::from_mut(&mut arr).cast::<Atomic8>().add(2),
        ptr::from_ref(&arr).cast::<Atomic8>().add(3),
        2,
      )
    };
    let expected: [i8; LEN] = [0, 1, 3, 4, 4, 5];
    assert_eq!(arr, expected);
  }
  {
    let mut arr: [Atomic8; LEN] = [0, 1, 2, 3, 4, 5];
    // SAFETY: dst=arr+3 与 src=arr+2 重叠, 走前向复制
    unsafe {
      relaxed_memmove(
        ptr::from_mut(&mut arr).cast::<Atomic8>().add(3),
        ptr::from_ref(&arr).cast::<Atomic8>().add(2),
        2,
      )
    };
    let expected: [i8; LEN] = [0, 1, 2, 2, 3, 5];
    assert_eq!(arr, expected);
  }
  OK
}

/// TEST(Atomicops, Relaxed_Memcmp) — 全偏移扫描
#[test]
fn atomicops_relaxed_memcmp() -> Void {
  const LEN: usize = 50;
  let mut arr1 = [0u8; LEN];
  let mut arr1_same = [0u8; LEN];
  let mut arr2 = [0u8; LEN];
  for i in 0..LEN {
    arr1[i] = i as u8;
    arr1_same[i] = i as u8;
    arr2[i] = i as u8 + 1;
  }

  for offset in 0..LEN {
    let p1 = ptr::from_ref(&arr1[offset..]).cast::<Atomic8>();
    let p1s = ptr::from_ref(&arr1_same[offset..]).cast::<Atomic8>();
    let p2 = ptr::from_ref(&arr2[offset..]).cast::<Atomic8>();
    let len = LEN - offset;
    // SAFETY: 三个数组栈上存活, 各 len 字节可读
    unsafe {
      assert_eq!(0, relaxed_memcmp(p1, p1, len));
      assert_eq!(0, relaxed_memcmp(p1, p1s, len));
      assert!(relaxed_memcmp(p1, p2, len) < 0);
      assert!(relaxed_memcmp(p2, p1, len) > 0);
    }
  }
  OK
}

/// 补充: 高位字节 (≥0x80) 的无符号序 — 锁定 word 路径 MemcmpNotEqualFundamental
/// 按 uint8_t 比较的语义 (原版用例值域 0..49 不覆盖该路径)
#[test]
fn atomicops_relaxed_memcmp_unsigned_high_bytes() -> Void {
  // 对齐到 8 的栈缓冲: 0x01 vs 0x80, 首字节即分叉
  let a: [u64; 1] = [0x0000_0000_0000_0001];
  let b: [u64; 1] = [0x0000_0000_0000_0080];
  // SAFETY: 两数组已按 8 对齐, 8 字节可读
  let r = unsafe {
    relaxed_memcmp(
      ptr::from_ref(&a).cast::<Atomic8>(),
      ptr::from_ref(&b).cast::<Atomic8>(),
      mem::size_of::<u64>(),
    )
  };
  // 0x01 < 0x80 按无符号字节序 → a < b (若按 i8 符号序会得出相反结论)
  assert!(r < 0, "got {r}");
  OK
}
