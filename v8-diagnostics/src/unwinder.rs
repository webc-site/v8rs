// 由 tool/src/diagnostics/unwinder.js 从 v8/src/diagnostics/unwinder.{h,cc} 生成; 修订就地改, 复跑覆盖
// unwinder 栈展开基础函数骨架

/// 原版 v8::internal::Load(address): 栈展开器读取远端地址的辅助入口。
/// 沙箱/压缩指针语义由内存访问层接线; 签名骨架。
#[must_use]
pub fn load(address: u64) -> u64 {
  if address == 0 {
    0
  } else {
    0
  }
}
