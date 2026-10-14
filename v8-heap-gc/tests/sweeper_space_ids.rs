// W11HGR 回归: 钉死 AllocationSpace 判别值与清扫空间计数 1:1
// (v8/src/common/globals.h:1488-1513 枚举连续: RO=0, NEW=1, OLD=2, CODE=3,
// SHARED=4, TRUSTED=5, SHARED_TRUSTED=6, NEW_LO=7, LO=8; 原版不存在 CODE_MAP_SPACE)。
// 兼防 Sweeper 常量再次臆造 (缺陷 D2 回归)。
use aok::{OK, Void};
use v8_heap_gc::{
  K_CODE_SPACE, K_FIRST_SWEEPABLE_SPACE, K_LAST_SWEEPABLE_SPACE, K_LO_SPACE, K_NEW_SPACE,
  K_NUMBER_OF_MAJOR_SWEEPING_SPACES, K_NUMBER_OF_SWEEPING_SPACES, K_OLD_SPACE, K_SHARED_SPACE,
  K_SHARED_TRUSTED_SPACE, K_TRUSTED_SPACE, K_ZAP_TAGGED,
};

#[ctor::ctor(unsafe)]
fn _log_init() {
  log_init::init();
}

/// AllocationSpace 判别值逐一对照 globals.h 枚举序 (原版无 CODE_MAP_SPACE)。
#[test]
fn allocation_space_discriminants_1to1() -> Void {
  assert_eq!(K_NEW_SPACE, 1);
  assert_eq!(K_OLD_SPACE, 2);
  assert_eq!(K_CODE_SPACE, 3);
  assert_eq!(K_SHARED_SPACE, 4);
  assert_eq!(K_TRUSTED_SPACE, 5);
  assert_eq!(K_SHARED_TRUSTED_SPACE, 6);
  // NEW_LO_SPACE=7 不参与 GC 清扫面; LO_SPACE=8 (globals.h:1500)
  assert_eq!(K_LO_SPACE, 8);
  // 可清扫区间: FIRST_SWEEPABLE=NEW 至 LAST_SWEEPABLE=SHARED_TRUSTED
  assert_eq!(K_FIRST_SWEEPABLE_SPACE, K_NEW_SPACE);
  assert_eq!(K_LAST_SWEEPABLE_SPACE, K_SHARED_TRUSTED_SPACE);
  OK
}

/// 清扫空间计数 (sweeper.h kNumberOfSweepingSpaces = LAST-FIRST+1;
/// sweeper.cc:135 kNumberOfMajorSweepingSpaces = 上者-1, 剥 NEW_SPACE)。
#[test]
fn sweeping_space_counts() -> Void {
  assert_eq!(
    K_NUMBER_OF_SWEEPING_SPACES,
    K_LAST_SWEEPABLE_SPACE - K_FIRST_SWEEPABLE_SPACE + 1
  );
  assert_eq!(K_NUMBER_OF_SWEEPING_SPACES, 6);
  assert_eq!(
    K_NUMBER_OF_MAJOR_SWEEPING_SPACES,
    K_NUMBER_OF_SWEEPING_SPACES - 1
  );
  assert_eq!(K_NUMBER_OF_MAJOR_SWEEPING_SPACES, 5);
  OK
}

/// kZapTagged = kZapValue(0xdeadbeedbeadbeef) 截断到 Tagged_t(u32, 指针压缩)
/// (v8: src/heap/sweeper.cc:54 AtomicZapBlock)。
#[test]
fn zap_tagged_truncation() -> Void {
  const K_ZAP_VALUE: u64 = 0xdeadbeedbeadbeef;
  assert_eq!(K_ZAP_TAGGED, K_ZAP_VALUE as u32);
  OK
}
