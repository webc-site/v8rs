// 由 tool/src/snapshot/snapshot-utils.js 从 v8/src/snapshot/snapshot-utils.{h,cc} 生成; 修订就地改, 复跑覆盖
// 快照校验和

/// v8: src/snapshot/snapshot-utils.cc Checksum
/// 校验和计算 (原版 uint32_t Checksum(Vector<const uint8_t> payload))。
/// 无 zlib 分支为 Fletcher-32: sum1/sum2 模 65535 累积, 结果 `sum2 << 16 | sum1`。
/// 分支登记: `#ifdef MEMORY_SANITIZER` 仅标记内存初始化 / `#ifdef V8_USE_ZLIB` 走 adler32。
#[must_use]
pub fn checksum(payload: &[u8]) -> u32 {
  // Fletcher-32 基线 (无 zlib, 机械转写): 单次遍历, 无 collect。
  let mut sum1: u32 = 0;
  let mut sum2: u32 = 0;
  for &d in payload {
    sum1 = (sum1 + u32::from(d)) % 65535;
    sum2 = (sum2 + sum1) % 65535;
  }
  (sum2 << 16) | sum1
}
