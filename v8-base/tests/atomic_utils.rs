//! v8/test/unittests/base/atomic-utils-unittest.cc 的规约迁移
//!
//! 规约清单 (每条原断言 ≥1 个 Rust 断言):
//! - AtomicValue Initial: 构造即初值 (Acquire 读)
//! - AtomicValue SetValue: Release 写后读回
//! - AtomicValue WithVoidStar: 裸指针存取往返
//! - AsAtomic8 CompareAndSwap_Sequential: 失败返回现值 / 成功改写
//! - AsAtomic8 CompareAndSwap_Concurrent: 8 字节 × 4 线程 × 10 次 CAS 自增
//! - AsAtomicWord Relaxed_SetBits_Sequential: 0xF0 底纹逐字节改 i
//! - AsAtomicWord Relaxed_SetBitsByMask_Sequential: 逐字节置 bit0
//! - AsAtomicWord Release_SetBits_Sequential: 同 Relaxed 版语义
//! - AsAtomicWord SetBits_Concurrent: 32 线程各置偶数位
//! - RelaxedMemset VariousTypesAndAlignments / ConstexprReplicatedWord:
//!   原版测试体实为 memcopy.h 的 Relaxed_Memset/ReplicateValueWord
//!   (本快照已迁至 memcopy.rs), 在此按原文件归属一并迁移

use std::{fmt, mem::size_of, ptr, sync::atomic::Ordering, thread};

use aok::{OK, Void};
use v8_base::{
  atomic_utils::{AsAtomic8, AsAtomicWord, AtomicValue, checked_decrement, checked_increment},
  memcopy::{RelaxedWord, relaxed_memset},
};

#[ctor::ctor(unsafe)]
fn _log_init() {
  log_init::init();
}

/// 原版 enum TestFlag : AtomicWord { kA, kB, kC } — 以 i64 承接
#[test]
fn atomic_value_initial() -> Void {
  let a: AtomicValue<i64> = AtomicValue::new(0);
  assert_eq!(0, a.value());
  OK
}

#[test]
fn atomic_value_set_value() -> Void {
  let a: AtomicValue<i64> = AtomicValue::new(1);
  a.set_value(2);
  assert_eq!(2, a.value());
  OK
}

#[test]
fn atomic_value_with_void_star() -> Void {
  let a: AtomicValue<*mut u8> = AtomicValue::new(ptr::null_mut());
  assert!(a.value().is_null());
  let mut target = 0u8;
  a.set_value(ptr::from_mut(&mut target));
  assert_eq!(ptr::from_mut(&mut target) as usize, a.value() as usize);
  OK
}

/// 跨线程传裸地址的 Send 包装
#[derive(Clone, Copy)]
struct Addr<T>(T);
unsafe impl<T> Send for Addr<T> {}

#[test]
fn as_atomic8_compare_and_swap_sequential() -> Void {
  let mut bytes = [0u8; 8];
  for (i, b) in bytes.iter_mut().enumerate() {
    *b = 0xF0 + i as u8;
  }
  // 老值不匹配 → 返回内存现值, 不改写
  for i in 0..8u8 {
    // SAFETY: bytes 栈上存活, u8 对齐恒成立, 仅本模块原子访问
    let prev = unsafe {
      AsAtomic8::release_compare_and_swap(ptr::from_ref(&bytes[i as usize]), i, 0xF7 + i)
    };
    assert_eq!(0xF0 + i, prev);
  }
  // 老值匹配 → 改写并返回旧值
  for i in 0..8u8 {
    // SAFETY: 同上
    let prev = unsafe {
      AsAtomic8::release_compare_and_swap(ptr::from_ref(&bytes[i as usize]), 0xF0 + i, 0xF7 + i)
    };
    assert_eq!(0xF0 + i, prev);
  }
  for (i, b) in bytes.iter().enumerate() {
    assert_eq!(0xF7 + i as u8, *b);
  }
  OK
}

#[test]
fn as_atomic8_compare_and_swap_concurrent() -> Void {
  const INCREMENTS: usize = 10;
  const BYTE_COUNT: usize = 8;
  const THREADS_PER_BYTE: usize = 4;
  let mut bytes = [0u8; BYTE_COUNT];
  let addrs: Vec<_> = bytes
    .iter_mut()
    .enumerate()
    .map(|(i, b)| {
      // SAFETY: 栈上存活; 仅本用例原子访问
      unsafe { AsAtomic8::relaxed_store(ptr::from_mut(b), i as u8) };
      Addr(ptr::from_mut(b) as usize)
    })
    .collect();

  thread::scope(|s| {
    for t in 0..BYTE_COUNT * THREADS_PER_BYTE {
      let byte = t / THREADS_PER_BYTE;
      let addr = addrs[byte];
      s.spawn(move || {
        let p = addr.0 as *mut u8;
        // 原版 Run(): for (i < increments_) Increment()
        for _ in 0..INCREMENTS {
          loop {
            // SAFETY: 指针来自 bytes 槽位, 存活覆盖整个 scope
            let b = unsafe { AsAtomic8::relaxed_load(p.cast_const()) };
            // SAFETY: 同上
            if unsafe { AsAtomic8::release_compare_and_swap(p.cast_const(), b, b.wrapping_add(1)) }
              == b
            {
              break;
            }
          }
        }
      });
    }
  });

  for (i, addr) in addrs.iter().enumerate() {
    // SAFETY: scope 已 join, 无并发访问
    let v = unsafe { AsAtomic8::relaxed_load(addr.0 as *const u8) };
    assert_eq!((i + INCREMENTS * THREADS_PER_BYTE) as u8, v);
  }
  OK
}

/// 0xF0 底纹检查 + 逐字节写 i (三个 SetBits 用例共用的骨架)
fn set_bits_pattern(mut set: impl FnMut(*const usize)) {
  const WORD_BYTES: usize = size_of::<usize>();
  let mut word = 0usize;
  for _ in 0..WORD_BYTES {
    word = (word << 8) | 0xF0;
  }
  for i in 0..WORD_BYTES {
    assert_eq!(0xF0, (word >> (i * 8)) & 0xFF);
  }
  // SAFETY: word 为本函数栈上存活 usize
  let addr = ptr::from_ref(&word);
  set(addr);
  // 检查写回值
  for i in 0..WORD_BYTES {
    assert_eq!(i, (word >> (i * 8)) & 0xFF);
  }
}

#[test]
fn as_atomic_word_relaxed_set_bits_sequential() -> Void {
  set_bits_pattern(|addr| {
    const WORD_BYTES: usize = size_of::<usize>();
    let mut mask = 0xFFusize;
    for i in 0..WORD_BYTES {
      let byte = i << (i * 8);
      // SAFETY: addr 指向存活 usize (骨架保证)
      unsafe { AsAtomicWord::relaxed_set_bits(addr, byte, mask) };
      mask <<= 8;
    }
  });
  OK
}

#[test]
fn as_atomic_word_relaxed_set_bits_by_mask_sequential() -> Void {
  const WORD_BYTES: usize = size_of::<usize>();
  let mut word = 0usize;
  for _ in 0..WORD_BYTES {
    word = (word << 8) | 0xF0;
  }
  for i in 0..WORD_BYTES {
    assert_eq!(0xF0, (word >> (i * 8)) & 0xFF);
  }
  let mut mask = 0x01usize;
  for _ in 0..WORD_BYTES {
    // SAFETY: word 栈上存活, 仅本模块原子访问
    unsafe { AsAtomicWord::relaxed_set_bits_mask(ptr::from_ref(&word), mask) };
    mask <<= 8;
  }
  for i in 0..WORD_BYTES {
    assert_eq!(0xF1, (word >> (i * 8)) & 0xFF);
  }
  OK
}

#[test]
fn as_atomic_word_release_set_bits_sequential() -> Void {
  set_bits_pattern(|addr| {
    const WORD_BYTES: usize = size_of::<usize>();
    let mut mask = 0xFFusize;
    for i in 0..WORD_BYTES {
      let byte = i << (i * 8);
      // SAFETY: addr 指向存活 usize (骨架保证)
      unsafe { AsAtomicWord::release_set_bits(addr, byte, mask) };
      mask <<= 8;
    }
  });
  OK
}

#[test]
fn as_atomic_word_set_bits_concurrent() -> Void {
  const BIT_COUNT: usize = usize::BITS as usize;
  const THREAD_COUNT: usize = BIT_COUNT / 2;
  let word = 0usize;
  // SAFETY: relaxed 原子写; word 存活覆盖 scope
  unsafe { AsAtomicWord::relaxed_store(ptr::from_ref(&word), 0) };
  let addr = Addr(ptr::from_ref(&word) as usize);

  thread::scope(|s| {
    for i in 0..THREAD_COUNT {
      s.spawn(move || {
        let bit = 1usize << (i * 2);
        // SAFETY: 指针来自 word, 存活覆盖 scope
        unsafe { AsAtomicWord::relaxed_set_bits(addr.0 as *const usize, bit, bit) };
      });
    }
  });

  // SAFETY: scope 已 join
  let mut actual = unsafe { AsAtomicWord::relaxed_load(addr.0 as *const usize) };
  for i in 0..BIT_COUNT {
    let expected = (i % 2 == 0) as usize;
    assert_eq!(expected, actual & 1);
    actual >>= 1;
  }
  OK
}

/// 原版 TestRelaxedMemsetHelper<T>: 64 槽缓冲 × offset 0..8 × count 0..32,
/// 区间内 == value、区间外 == sentinel
fn memset_helper<T>(value: T, sentinel: T)
where
  T: RelaxedWord + PartialEq + fmt::Debug,
{
  const BUFFER_SIZE: usize = 64;
  for offset in 0..8 {
    for count in 0..32 {
      if offset + count > BUFFER_SIZE {
        continue;
      }
      let mut buffer = [sentinel; BUFFER_SIZE];
      relaxed_memset(&mut buffer[offset..offset + count], value);
      for (i, b) in buffer.iter().enumerate() {
        if i >= offset && i < offset + count {
          assert_eq!(&value, b, "offset={offset} count={count} i={i}");
        } else {
          assert_eq!(&sentinel, b, "offset={offset} count={count} i={i}");
        }
      }
    }
  }
}

/// TEST(RelaxedMemset, VariousTypesAndAlignments)
#[test]
fn relaxed_memset_various_types_and_alignments() -> Void {
  memset_helper(0xABu8, 0x12);
  memset_helper(0xABCDu16, 0x1234);
  memset_helper(0xABCDEF01u32, 0x12345678);
  memset_helper(0xABCDEF0123456789u64, 0x1111222233334444);
  memset_helper(-1i8, 0);
  memset_helper(-123456i32, 789);
  OK
}

/// TEST(RelaxedMemset, ConstexprReplicatedWord):
/// static_assert ReplicateValueWord 对照 + 非对齐 memset 边界
#[test]
fn relaxed_memset_constexpr_replicated_word() -> Void {
  use v8_base::memcopy::RelaxedWord;
  // 原版 static_assert: 0xAB → 0xABABABABABABABAB (64 位宿主)
  assert_eq!(0xABu8.replicate_word(), 0xABAB_ABAB_ABAB_ABABu64 as usize);
  assert_eq!(
    0x1234u16.replicate_word(),
    0x1234_1234_1234_1234u64 as usize
  );

  const BUFFER_SIZE: usize = 32;
  let mut buffer = [0xFFu8; BUFFER_SIZE];
  relaxed_memset(&mut buffer[1..16], 0x42);
  assert_eq!(0xFF, buffer[0]);
  for b in &buffer[1..16] {
    assert_eq!(0x42, *b);
  }
  for b in &buffer[16..] {
    assert_eq!(0xFF, *b);
  }
  OK
}

/// CheckedIncrement/CheckedDecrement (atomic-utils.h:294/305, 原文件无 gtest
/// 覆盖 — 补充断言锁定环绕 debug 校验语义, 属加强)
#[test]
fn checked_counter_roundtrip() -> Void {
  use std::sync::atomic::AtomicU32;
  let c = AtomicU32::new(5);
  checked_increment(&c, 3, Ordering::SeqCst);
  assert_eq!(c.load(Ordering::SeqCst), 8);
  checked_decrement(&c, 8, Ordering::SeqCst);
  assert_eq!(c.load(Ordering::SeqCst), 0);
  OK
}
