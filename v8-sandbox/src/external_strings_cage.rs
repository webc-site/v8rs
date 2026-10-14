// 由 tool/src/sandbox/external-strings-cage.h.js 从 v8/src/sandbox/external-strings-cage.{h,cc} 生成; 修订就地改, 复跑覆盖
// ExternalStringsCage 布局常量

/// 笼内容最大尺寸 (原 kMaxContentsSize = (1<<33) + GB)
pub const MAX_CONTENTS_SIZE: usize = 9663676416;

/// 守护区尺寸 (原 kGuardRegionSize = 1 << 34)
pub const GUARD_REGION_SIZE: usize = 17179869184;

/// 原 class ExternalStringsCage 方法面:
// - Deleter
// - cage_
// - operator
// - ExternalStringsCage
// - GetInstance
// - InitializeOncePerProcess
// - TearDown
// - CHECK_LE
// - Seal
// - reservation_region
// - CHECK
// - GetAllocSize
// - AllocateRaw
// - Free

pub struct ExternalStringsCage;

// TODO(dev): 补笼初始化/指针进出笼实现。

// —— 提取不变式断言 ——
const _: () = {
  assert!(MAX_CONTENTS_SIZE == 9663676416);
  assert!(GUARD_REGION_SIZE == 17179869184);
};
