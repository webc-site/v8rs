// 由 tool/src/common.js 从 v8/src/common/checks.h 生成; 修订就地改, 复跑覆盖
//
// 修订 (Developer): 宏 → Rust 断言映射落位。目标配置非 ENABLE_SLOW_DCHECK:
// SLOW_DCHECK → 空操作 (checks.h:21); DCHECK_WITH_SANDBOX_ACCESS →
// debug_assert (sandbox 访问例外对象 L3 接入, 登记裁剪)。
//
// 修订 (审查环 round1, D5): 裁剪理由落成可调用符号 (此前仅注释声明) —
// 4 个被裁宏均有 1:1 函数实体, 调用点可 grep, 语义不凭注释悬空。
//
// 修订 (审查环 round2, F1): SLOW_DCHECK 族入参改惰性闭包 — 原版禁用分支
// `((void)0)` 不求值条件, bool 值参在调用点强求值违背该语义;
// DCHECK_WITH_SANDBOX_ACCESS 族同因 (release 分支同样 `((void)0)` 空操作,
// checks.h:38-42) 一并惰性化。

use crate::internal::K_HEAP_OBJECT_TAG_MASK;

/// 地址按堆对象 2 字节 tag 对齐 (原 DCHECK_TAG_ALIGNED, checks.h:44-45)
#[inline]
pub fn dcheck_tag_aligned(address: usize) {
  debug_assert!(address & K_HEAP_OBJECT_TAG_MASK == 0);
}

/// 尺寸按堆对象 tag 对齐 (原 DCHECK_SIZE_TAG_ALIGNED, checks.h:47-48)
#[inline]
pub fn dcheck_size_tag_aligned(size: usize) {
  debug_assert!(size & K_HEAP_OBJECT_TAG_MASK == 0);
}

/// 原 SLOW_DCHECK (checks.h:16-23) — 目标配置无 ENABLE_SLOW_DCHECKS →
/// 编译期即空操作 (原版 else 分支 `((void)0)`, 条件不求值)。
/// 审查环 round2 (F1): 入参改惰性闭包 — `bool` 值参会在调用点强求值,
/// 与原版宏禁用分支"条件不求值"相悖 (副作用/开销泄漏)
#[inline]
pub fn slow_dcheck(_condition: impl FnOnce() -> bool) {}

/// 原 SLOW_DCHECK_IMPLIES (checks.h:19/22) — 同上空操作, 双参均惰性
#[inline]
pub fn slow_dcheck_implies(_lhs: impl FnOnce() -> bool, _rhs: impl FnOnce() -> bool) {}

/// 原 DCHECK_WITH_SANDBOX_ACCESS (checks.h:26-42) — release 分支空操作;
/// DEBUG 分支 = AllowSandboxAccess RAII 包裹的 DCHECK, 断言本体即
/// debug_assert; sandbox 访问例外对象为差分日志设施, L3 sandbox 接入
/// (登记裁剪: RAII 包裹层不落位, 断言语义 1:1)
/// 审查环 round2 (F1): 入参改惰性闭包 (同 slow_dcheck 族) — release
/// 空操作分支条件不得求值 (原版宏语义, 副作用/开销不泄漏)
#[inline]
pub fn dcheck_with_sandbox_access(condition: impl FnOnce() -> bool) {
  debug_assert!(condition());
}

/// 原 DCHECK_WITH_SANDBOX_ACCESS_AND_MSG_AND_LOC (checks.h:32-37) —
/// 同上映射; msg/loc 仅 debug 失败时参与打印, Rust panic 消息由
/// debug_assert 调用点自带 (登记)
/// 审查环 round2 (F1): 入参改惰性闭包, 理由同上
#[inline]
pub fn dcheck_with_sandbox_access_and_msg(condition: impl FnOnce() -> bool) {
  debug_assert!(condition());
}
