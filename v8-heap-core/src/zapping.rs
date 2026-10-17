// 由 tool/src/heap/zapping.h.js 从 v8/src/heap/zapping.h + src/common/globals.h 生成; 修订就地改, 复跑覆盖
// 堆填充 (zapping) 判定与 zap 值常量

/// kZapValue (src/common/globals.h, 64 位目标)
pub const K_ZAP_VALUE: u64 = 0xdeadbeedbeadbeef;
/// kCodeZapValue (src/common/globals.h)
pub const K_CODE_ZAP_VALUE: u64 = 0xbadc0de;

/// 是否需要填充垃圾 (对应 ShouldZapGarbage; DEBUG 或 VERIFY_HEAP+flag)。
/// 展开按 release + VERIFY_HEAP 关闭取值: false; debug 构建恒 true (登记于注释)。
#[inline]
#[must_use]
pub const fn should_zap_garbage() -> bool {
  // #ifdef DEBUG → true
  // #else #ifdef VERIFY_HEAP → v8_flags.verify_heap
  false
}

/// zap 值选择 (对应 ZapValue: clear_free_memory ? kClearedFreeMemoryValue : kZapValue)
#[inline]
#[must_use]
pub const fn zap_value(clear_free_memory: bool) -> u64 {
  K_ZAP_VALUE
}

/// 填充常规内存块 (对应 ZapBlock, 骨架)
pub fn zap_block(_start: usize, _size_in_bytes: usize, _zap_value: u64) {
  unimplemented!("骨架: 参照原版 zapping.cc ZapBlock 修订")
}

/// 填充代码块 (对应 ZapCodeBlock, 使用 kCodeZapValue, 骨架)
pub fn zap_code_block(_start: usize, _size_in_bytes: i32) {
  unimplemented!("骨架: 参照原版 zapping.cc ZapCodeBlock 修订")
}

// —— 提取不变式断言 ——
const _: () = {
  assert!(K_ZAP_VALUE == 0xdeadbeedbeadbeef);
  assert!(!should_zap_garbage());
};
