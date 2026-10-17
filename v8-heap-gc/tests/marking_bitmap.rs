// 1:1 转写 v8/test/unittests/heap/marking-bitmap-unittest.cc (非原子面)
// 原 AtomicBitmapTest 实例化在原子路径未落地前与非原子语义等价, 合并到同一实现验证。
use aok::{OK, Void};
use v8_heap_gc::*;

#[ctor::ctor(unsafe)]
fn _log_init() {
  log_init::init();
}

// MarkBit::CellType 全 1 (std::numeric_limits<MarkBit::CellType>::max())
const K_MARKED_CELL: MarkBitCell = usize::MAX;
// 低半字全置位 (kMarkedCell >> (bits/2))
const K_LOWER_HALF_MARKED_CELL: MarkBitCell = K_MARKED_CELL >> (K_BITS_PER_CELL / 2);
// 高半字全置位 (~kLowerHalfMarkedCell)
const K_HIGHER_HALF_MARKED_CELL: MarkBitCell = !K_LOWER_HALF_MARKED_CELL;
// 全 0 cell (kWhiteCell)
const K_WHITE_CELL: MarkBitCell = 0;

fn all_bytes_of(bm: &MarkingBitmap) -> [u8; K_SIZE] {
  let mut out = [0u8; K_SIZE];
  let mut i = 0;
  for &cell in &bm.cells {
    for b in cell.to_ne_bytes() {
      out[i] = b;
      i += 1;
    }
  }
  out
}

// TEST_F(NonAtomicBitmapTest, IsZeroInitialized)
#[test]
fn is_zero_initialized() -> Void {
  let bm = MarkingBitmap::new();
  for byte in all_bytes_of(&bm) {
    assert_eq!(byte, 0);
  }
  OK
}

// TEST_F(NonAtomicBitmapTest, Cells)
#[test]
fn cells() -> Void {
  let mut bm = MarkingBitmap::new();
  bm.cells[1] = K_MARKED_CELL;
  let raw = all_bytes_of(&bm);
  for i in 0..K_BYTES_PER_CELL as usize {
    assert_eq!(raw[K_BYTES_PER_CELL as usize + i], 0xFF);
  }
  OK
}

// TEST_F(NonAtomicBitmapTest, CellsCount)
#[test]
fn cells_count() -> Void {
  let mut bm = MarkingBitmap::new();
  bm.cells[K_CELLS_COUNT - 1] = K_MARKED_CELL;
  let raw = all_bytes_of(&bm);
  for (i, byte) in raw.iter().enumerate() {
    if i >= K_SIZE - K_BYTES_PER_CELL as usize {
      assert_eq!(*byte, 0xFF);
    } else {
      assert_eq!(*byte, 0);
    }
  }
  OK
}

// TEST_F(NonAtomicBitmapTest, IsClean)
#[test]
fn is_clean() -> Void {
  let mut bm = MarkingBitmap::new();
  assert!(bm.is_clean());
  bm.cells[0] = K_MARKED_CELL;
  assert!(!bm.is_clean());
  OK
}

// TEST_F(Atomic/NonAtomicBitmapTest, Clear)
#[test]
fn clear() -> Void {
  let mut bm = MarkingBitmap::new();
  for cell in bm.cells.iter_mut() {
    *cell = K_MARKED_CELL;
  }
  bm.clear();
  for &cell in &bm.cells {
    assert_eq!(cell, 0);
  }
  OK
}

// TEST_F(Atomic/NonAtomicBitmapTest, ClearRange1)
#[test]
fn clear_range1() -> Void {
  let mut bm = MarkingBitmap::new();
  bm.cells[0] = K_MARKED_CELL;
  bm.cells[1] = K_MARKED_CELL;
  bm.cells[2] = K_MARKED_CELL;
  bm.clear_range(0, K_BITS_PER_CELL + K_BITS_PER_CELL / 2);
  assert_eq!(bm.cells[0], K_WHITE_CELL);
  assert_eq!(bm.cells[1], K_HIGHER_HALF_MARKED_CELL);
  assert_eq!(bm.cells[2], K_MARKED_CELL);
  OK
}

// TEST_F(Atomic/NonAtomicBitmapTest, ClearRange2)
#[test]
fn clear_range2() -> Void {
  let mut bm = MarkingBitmap::new();
  bm.cells[0] = K_MARKED_CELL;
  bm.cells[1] = K_MARKED_CELL;
  bm.cells[2] = K_MARKED_CELL;
  bm.clear_range(K_BITS_PER_CELL, K_BITS_PER_CELL + K_BITS_PER_CELL / 2);
  assert_eq!(bm.cells[0], K_MARKED_CELL);
  assert_eq!(bm.cells[1], K_HIGHER_HALF_MARKED_CELL);
  assert_eq!(bm.cells[2], K_MARKED_CELL);
  OK
}

// TEST_F(Atomic/NonAtomicBitmapTest, SetAndClearRange)
#[test]
fn set_and_clear_range() -> Void {
  for i in 0..3u32 {
    let mut bm = MarkingBitmap::new();
    bm.set_range(i, K_BITS_PER_CELL + i);
    assert_eq!(bm.cells[0], K_ALL_BITS_SET_IN_CELL_VALUE << i);
    assert_eq!(bm.cells[1], (1usize << i) - 1);
    bm.clear_range(i, K_BITS_PER_CELL + i);
    assert_eq!(bm.cells[0], 0);
    assert_eq!(bm.cells[1], 0);
  }
  OK
}

// TEST_F(NonAtomicBitmapTest, ClearMultipleRanges)
#[test]
fn clear_multiple_ranges() -> Void {
  let mut bm = MarkingBitmap::new();
  bm.set_range(0, K_BITS_PER_CELL * 3);
  assert!(bm.all_bits_set_in_range(0, K_BITS_PER_CELL));

  bm.clear_range(K_BITS_PER_CELL / 2, K_BITS_PER_CELL);
  bm.clear_range(K_BITS_PER_CELL, K_BITS_PER_CELL + K_BITS_PER_CELL / 2);
  bm.clear_range(K_BITS_PER_CELL * 2 + 8, K_BITS_PER_CELL * 2 + 16);
  bm.clear_range(K_BITS_PER_CELL * 2 + 24, K_BITS_PER_CELL * 3);

  assert_eq!(bm.cells[0], K_LOWER_HALF_MARKED_CELL);
  assert!(bm.all_bits_set_in_range(0, K_BITS_PER_CELL / 2));
  assert!(bm.all_bits_clear_in_range(K_BITS_PER_CELL / 2, K_BITS_PER_CELL));

  assert_eq!(bm.cells[1], K_HIGHER_HALF_MARKED_CELL);
  assert!(bm.all_bits_clear_in_range(K_BITS_PER_CELL, K_BITS_PER_CELL + K_BITS_PER_CELL / 2));
  assert!(bm.all_bits_set_in_range(K_BITS_PER_CELL + K_BITS_PER_CELL / 2, K_BITS_PER_CELL * 2));

  assert_eq!(bm.cells[2], 0x00FF_00FF);
  assert!(bm.all_bits_set_in_range(K_BITS_PER_CELL * 2, K_BITS_PER_CELL * 2 + 8));
  assert!(bm.all_bits_clear_in_range(K_BITS_PER_CELL * 2 + 8, K_BITS_PER_CELL * 2 + 16));
  assert!(bm.all_bits_set_in_range(K_BITS_PER_CELL * 2 + 16, K_BITS_PER_CELL * 2 + 24));
  assert!(bm.all_bits_clear_in_range(K_BITS_PER_CELL * 2 + 24, K_BITS_PER_CELL * 3));
  OK
}

// TEST_F(NonAtomicBitmapTest, TransitionMarkBit)
#[test]
fn transition_mark_bit() -> Void {
  let bm = MarkingBitmap::new();
  let mut cells = bm.cells;
  for position in [K_BITS_PER_CELL - 2, K_BITS_PER_CELL - 1, K_BITS_PER_CELL] {
    let mark_bit = bm.mark_bit_from_index(position);
    assert!(!mark_bit.get(&cells));
    assert!(mark_bit.set(&mut cells));
    assert!(mark_bit.get(&cells));
    assert!(mark_bit.clear(&mut cells));
    assert!(!mark_bit.get(&cells));
  }
  OK
}
