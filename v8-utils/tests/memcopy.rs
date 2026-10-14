// 迁移自 v8/test/cctest/test-utils.cc 的 TEST(MemMove)
// (同文件其余用例属 base/conversions/cctest 基建, 见 README 映射表)
// 断言规约: 与 std::memmove 全组合对比逐字节一致。
use std::sync::atomic::Ordering;

use aok::{OK, Void};
use v8_utils::memcopy::{copy_chars, mem_move, relaxed_memset_u8, relaxed_memset_u32};

#[ctor::ctor(unsafe)]
fn _log_init() {
  log_init::init();
}

const AREA_SIZE: usize = 512;

/// TestMemMove: 以相同初值的两块区域分别走实现与标准库对比
fn test_mem_move(
  area1: &mut [u8],
  area2: &mut [u8],
  src_offset: usize,
  dest_offset: usize,
  length: usize,
) {
  for (i, (a, b)) in area1.iter_mut().zip(area2.iter_mut()).enumerate() {
    *a = (i & 0xFF) as u8;
    *b = (i & 0xFF) as u8;
  }
  // SAFETY: area1 内 [src, src+len) 与 [dst, dst+len) 均在界内 (调用方组合保证)
  unsafe {
    mem_move(
      area1.as_mut_ptr().add(dest_offset),
      area1.as_ptr().add(src_offset),
      length,
    );
  }
  area2.copy_within(src_offset..src_offset + length, dest_offset);
  assert_eq!(
    area1, area2,
    "MemMove(): src {src_offset}, dst {dest_offset}, len {length}"
  );
}

// MemMove: 33×33×129 全组合
#[test]
fn mem_move_all() -> Void {
  let mut area1 = vec![0u8; AREA_SIZE];
  let mut area2 = vec![0u8; AREA_SIZE];

  const MIN_OFFSET: usize = 32;
  const MAX_OFFSET: usize = 64;
  const MAX_LENGTH: usize = 128;
  const _: () = assert!(MAX_OFFSET + MAX_LENGTH < AREA_SIZE);

  for src_offset in MIN_OFFSET..=MAX_OFFSET {
    for dst_offset in MIN_OFFSET..=MAX_OFFSET {
      for length in 0..=MAX_LENGTH {
        test_mem_move(&mut area1, &mut area2, src_offset, dst_offset, length);
      }
    }
  }
  OK
}

// 补充: CopyChars u8→u16 零扩展 (关键路径, 上游无独立单测)
#[test]
fn copy_chars_zero_extend() -> Void {
  let src: &[u8] = &[0x00, 0x7F, 0x80, 0xFF];
  let mut dst = [0u16; 4];
  copy_chars(&mut dst, src);
  assert_eq!(dst, [0x0000, 0x007F, 0x0080, 0x00FF]);

  let src16: &[u16] = &[0x1234, 0xABCD];
  let mut dst8 = [0u8; 2];
  copy_chars(&mut dst8, src16); // 高位截断 (上游 static_assert 整型域)
  assert_eq!(dst8, [0x34, 0xCD]);
  OK
}

// 补充: 不可重叠拷贝族 (MemCopy/CopyWords/CopyTagged/CopyBytes) 与 copy_chars_raw
#[test]
fn nonoverlapping_copy_family() -> Void {
  use v8_utils::memcopy::{copy_bytes, copy_chars_raw, copy_tagged, copy_words, mem_copy};

  // MemCopy: 字节拷贝
  let src = [1u8, 2, 3, 4, 5];
  let mut dst = [0u8; 5];
  // SAFETY: 两数组不重叠, 长度一致
  unsafe { mem_copy(dst.as_mut_ptr(), src.as_ptr(), 5) };
  assert_eq!(dst, src);

  // CopyWords: 系统字 (usize) 拷贝
  let wsrc = [0x1111_1111_1111_1111usize, 0x2222_2222_2222_2222];
  let mut wdst = [0usize; 2];
  // SAFETY: 不重叠, 对齐由数组保证
  unsafe { copy_words(wdst.as_mut_ptr(), wsrc.as_ptr(), 2) };
  assert_eq!(wdst, wsrc);

  // CopyTagged: u32 槽拷贝
  let tsrc = [0xDEAD_BEEFu32, 0x1];
  let mut tdst = [0u32; 2];
  // SAFETY: 不重叠, 对齐由数组保证
  unsafe { copy_tagged(tdst.as_mut_ptr(), tsrc.as_ptr(), 2) };
  assert_eq!(tdst, tsrc);

  // CopyBytes
  let mut bdst = [0u8; 3];
  // SAFETY: 不重叠
  unsafe { copy_bytes(bdst.as_mut_ptr(), [9u8, 8, 7].as_ptr(), 3) };
  assert_eq!(bdst, [9, 8, 7]);

  // copy_chars_raw: u16 → u8 高位截断的裸指针形态
  let mut rdst = [0u8; 3];
  // SAFETY: 不重叠, count 个元素均在界内
  unsafe { copy_chars_raw(rdst.as_mut_ptr(), [0x0102u16, 0x0304, 0x0506].as_ptr(), 3) };
  assert_eq!(rdst, [0x02, 0x04, 0x06]);
  OK
}

// 补充: Relaxed_Memset 逐元素原子填充 (上游 is_integral 模板, u8/u32 形态抽查)
#[test]
fn relaxed_memset_fills() -> Void {
  use std::sync::atomic::AtomicU32;

  let mut buf8 = [0u8; 4];
  // SAFETY: buf8 起连续 4 字节可写
  unsafe { relaxed_memset_u8(buf8.as_mut_ptr(), 0xAB, 4) };
  assert_eq!(buf8, [0xAB; 4]);

  let mut buf32 = [0u32; 3];
  // SAFETY: buf32 对齐且连续 3 元素可写
  unsafe { relaxed_memset_u32(buf32.as_mut_ptr(), 0x0102_0304, 3) };
  // 以原子读回 (与写入同一 relaxed 语义)
  let check: Vec<u32> = buf32
    .iter()
    .map(|p| {
      // SAFETY: &u32 → *const u32, AtomicU32 与 u32 布局位级兼容 (std 保证)
      unsafe { AtomicU32::from_ptr(p as *const u32 as *mut u32) }.load(Ordering::Relaxed)
    })
    .collect();
  assert_eq!(check, vec![0x0102_0304; 3]);
  OK
}
