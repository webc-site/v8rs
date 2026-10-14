// 由 tool/src/trap-handler/handler-outside.cc.js 从 v8/src/trap-handler/handler-outside.cc 生成; 修订就地改, 复跑覆盖

//! 处理器外的注册/查询 API; 不运行在信号上下文, 但操作处理器读取的数据结构
//! 规则区原文行区间: ["gNext","consts","verify","register","release","sandbox","covered","recovered","enable","landingPad"]
use std::{
  process::abort,
  sync::atomic::{AtomicBool, Ordering},
};

use crate::{
  TrappingInstructionData,
  error::{Error, Result},
  internal::{CodeEntry, CodeProtectionInfo, CoveredMemRec, SandboxRec, th_check},
  register_default_trap_handler,
  shared::{
    COVERED_MEMORY, LANDING_PAD, METADATA, Metadata, RECOVERED_TRAP_COUNT, SANDBOX_RECORDS,
    TrapHandlerGuard,
  },
};

/// handler-outside.cc:46 kInitialCodeObjectSize
const K_INITIAL_CODE_OBJECT_SIZE: usize = 1024;
/// handler-outside.cc:47 kCodeObjectGrowthFactor
const K_CODE_OBJECT_GROWTH_FACTOR: usize = 2;

/// handler-outside.cc:362 g_is_trap_handler_enabled
/// (原版普通 bool; Rust static 需原子, Relaxed 与普通读写代码生成等价)
pub(crate) static IS_TRAP_HANDLER_ENABLED: AtomicBool = AtomicBool::new(false);
/// handler-outside.cc:363 g_can_enable_trap_handler (原版 atomic<bool> relaxed)
pub(crate) static CAN_ENABLE_TRAP_HANDLER: AtomicBool = AtomicBool::new(true);

/// handler-outside.cc:56 IsDisjoint (debug 校验): 两代码范围不相交
fn is_disjoint(a: &CodeProtectionInfo, b: &CodeProtectionInfo) -> bool {
  a.base >= b.base.wrapping_add(b.size) || b.base >= a.base.wrapping_add(a.size)
}

/// handler-outside.cc:66 VerifyCodeRangeIsDisjoint (debug 断言级)
fn verify_disjoint(table: &[CodeEntry], data: &CodeProtectionInfo) -> bool {
  table
    .iter()
    .all(|e| e.code_info.as_deref().is_none_or(|b| is_disjoint(data, b)))
}

/// handler-outside.cc:72 ValidateCodeObjects 的自由链表校验子集 (debug 级):
/// 从游标起每步必在界内且成链 (无环)、链上槽必须空闲,
/// 且链可达槽数 == 表内全部空闲槽数 (无不可达空槽)
fn validate_free_list(md: &Metadata) -> bool {
  let mut visited = 0;
  let mut i = md.next;
  while i != md.table.len() {
    if i >= md.table.len() || visited > md.table.len() {
      return false;
    }
    if md.table[i].code_info.is_some() {
      return false;
    }
    i = md.table[i].next_free;
    visited += 1;
  }
  let idle = md.table.iter().filter(|e| e.code_info.is_none()).count();
  visited == idle
}

/// handler-outside.cc:114 RegisterHandlerData
/// 注册代码对象的受保护指令表, 返回索引供 release_handler_data 反注册。
/// 原版 malloc 失败 abort —— Rust 分配失败语言级终止, 语义一致;
/// 表达 int 索引上限时返回 Err (原版 kInvalidIndex 路径)
pub fn register_handler_data(
  base: usize,
  size: usize,
  trapping_instructions: &[TrappingInstructionData],
) -> Result<i32> {
  let data = CodeProtectionInfo {
    base,
    size,
    instructions: trapping_instructions.into(),
  };

  let _guard = TrapHandlerGuard::new();
  let mut md = METADATA.lock();

  debug_assert!(verify_disjoint(&md.table, &data));

  let i = md.next;
  // :155 索引必须可容纳于 i32 (返回值契约)
  const INT_MAX: usize = i32::MAX as usize;

  // :158 游标 == 表长 → 无空洞, 扩容
  if i == md.table.len() {
    let new_size = if md.table.is_empty() {
      K_INITIAL_CODE_OBJECT_SIZE
    } else {
      md.table.len() * K_CODE_OBJECT_GROWTH_FACTOR
    }
    .min(INT_MAX);
    // :168 已到 int 上限无法再扩
    if new_size == md.table.len() {
      return Err(Error::CodeObjectTableFull);
    }
    let old_len = md.table.len();
    md.table.resize_with(new_size, || CodeEntry {
      code_info: None,
      next_free: 0,
    });
    // :184 新槽 next_free = j+1, 游标推进直接取用 (游标 == 表长表示无空洞)
    for (j, e) in md.table.iter_mut().enumerate().skip(old_len) {
      e.next_free = j + 1;
    }
  }

  debug_assert!(md.table[i].code_info.is_none());

  // :193 从槽内取出下一个插入位
  md.next = md.table[i].next_free;
  md.table[i].code_info = Some(Box::new(data));
  // :198 kEnableSlowChecks 路径: 注册后自由链表仍须一致
  debug_assert!(validate_free_list(&md));
  Ok(i as i32)
}

/// handler-outside.cc:121 ReleaseHandlerData
/// 反注册: 摘除数据, 槽位回到自由链表头 (LIFO 复用)
pub fn release_handler_data(index: i32) {
  // :210 无效索引直接返回
  if index == crate::K_INVALID_INDEX {
    return;
  }
  debug_assert!(index >= 0);
  // :213 负索引为调用错误: debug 崩溃对齐原版 TH_DCHECK;
  // release 下原版是数组负下标 UB, 此处安全化为直接返回 (禁 panic 红线)
  if index < 0 {
    return;
  }
  // 界外索引: 原版数组越界为 UB, 此处安全化为 no-op (禁 panic 红线; 测试契约见
  // release_out_of_range_noop — 误释放不得破坏既有注册)
  {
    let _guard = TrapHandlerGuard::new();
    let md = METADATA.lock();
    if index as usize >= md.table.len() {
      return;
    }
  }

  let data = {
    let _guard = TrapHandlerGuard::new();
    let mut md = METADATA.lock();
    let i = index as usize;
    let data = md.table[i].code_info.take();
    // :223 释放槽成为下一个插入位
    md.table[i].next_free = md.next;
    md.next = i;
    // :227 kEnableSlowChecks 路径: 释放后自由链表仍须一致
    debug_assert!(validate_free_list(&md));
    data
  };
  // :233 释放的必须是合法注册数据 (二次释放为程序错误)
  debug_assert!(data.is_some());
}

/// handler-outside.cc:238 RegisterV8Sandbox
/// 注册 sandbox 范围; 之后注册的覆盖内存必须整体落在 sandbox 内。
/// 原版 malloc 失败返回 false; Rust 分配失败语言级终止, 无 false 路径
pub fn register_v8_sandbox(base: usize, size: usize) {
  let _guard = TrapHandlerGuard::new();
  let mut recs = SANDBOX_RECORDS.lock();
  // :242 重复注册同 base 为程序错误 (debug 断言)
  debug_assert!(recs.iter().all(|r| r.base != base));
  recs.push(SandboxRec { base, size });
}

/// handler-outside.cc:262 UnregisterV8Sandbox
pub fn unregister_v8_sandbox(base: usize, size: usize) {
  let _guard = TrapHandlerGuard::new();
  let mut recs = SANDBOX_RECORDS.lock();
  // :275 未注册即反注册 → 契约破坏, 崩溃
  let Some(pos) = recs.iter().position(|r| r.base == base) else {
    abort()
  };
  // :276 尺寸必须精确匹配
  th_check!(recs[pos].size == size);
  recs.remove(pos);
}

/// handler-outside.cc:286 RegisterCoveredMemory
/// 注册陷阱处理器应覆盖的内存区间 (wasm 内存含保护区间 / WasmNull 载荷)
pub fn register_covered_memory(base: usize, reserved_size: usize) {
  let _guard = TrapHandlerGuard::new();
  // :289 base+size 回绕检查
  th_check!(
    base
      .checked_add(reserved_size)
      .is_some_and(|end| end > base)
  );

  // :291 第一道校验: 整体落在任一已注册 sandbox 内 (若存在)
  {
    let sandboxes = SANDBOX_RECORDS.lock();
    if !sandboxes.is_empty() {
      let within = sandboxes
        .iter()
        .any(|s| base >= s.base && base.wrapping_add(reserved_size) <= s.base.wrapping_add(s.size));
      th_check!(within);
    }
  }

  let mut mems = COVERED_MEMORY.lock();
  // :307 第二道校验: 与既有覆盖内存不得重叠
  for m in mems.iter() {
    let disjoint =
      base >= m.base.wrapping_add(m.size) || base.wrapping_add(reserved_size) <= m.base;
    th_check!(disjoint);
  }
  mems.push(CoveredMemRec {
    base,
    size: reserved_size,
  });
}

/// handler-outside.cc:328 UnregisterCoveredMemory
pub fn unregister_covered_memory(base: usize, reserved_size: usize) {
  let _guard = TrapHandlerGuard::new();
  let mut mems = COVERED_MEMORY.lock();
  // :339 反注册必须精确匹配既有记录
  let Some(pos) = mems.iter().position(|m| m.base == base) else {
    abort()
  };
  th_check!(mems[pos].size == reserved_size);
  mems.remove(pos);
}

/// handler-outside.cc:349 GetRecoveredTrapCount
pub fn get_recovered_trap_count() -> usize {
  RECOVERED_TRAP_COUNT.load(Ordering::Relaxed)
}

/// handler-outside.cc:365 EnableTrapHandler
/// use_v8_handler: true → 由本 crate 安装 posix 信号处理器;
/// false → 嵌入器自行安装。仅允许启用一次, 且先于任何 is_trap_handler_enabled。
/// 原版重复启用 IMMEDIATE_CRASH; 此处转为 Err (可恢复使用错误, 错误集中 error.rs)
pub fn enable_trap_handler(use_v8_handler: bool) -> Result<bool> {
  // :370 原子换出 can_enable; 已被查询/启用过即拒绝
  let can_enable = CAN_ENABLE_TRAP_HANDLER.swap(false, Ordering::Relaxed);
  if !can_enable {
    return Err(Error::EnableTwice);
  }
  if !crate::TRAP_HANDLER_SUPPORTED {
    return Ok(false);
  }

  // :378 warm-up TrapHandlerGuard 的 TLS: 避免首次访问落在信号处理器内
  let _guard = TrapHandlerGuard::new();

  if use_v8_handler {
    let ok = register_default_trap_handler();
    IS_TRAP_HANDLER_ENABLED.store(ok, Ordering::Relaxed);
    Ok(ok)
  } else {
    IS_TRAP_HANDLER_ENABLED.store(true, Ordering::Relaxed);
    Ok(true)
  }
}

/// trap-handler.h:165 IsTrapHandlerEnabled
/// 查询即冻结: 之后禁止再启用 (晚启用会破坏按禁用假设生成的代码/对象)
pub fn is_trap_handler_enabled() -> bool {
  debug_assert!(!IS_TRAP_HANDLER_ENABLED.load(Ordering::Relaxed) || crate::TRAP_HANDLER_SUPPORTED);
  // :171 先测后写, 避免无条件写的争用
  if CAN_ENABLE_TRAP_HANDLER.load(Ordering::Relaxed) {
    CAN_ENABLE_TRAP_HANDLER.store(false, Ordering::Relaxed);
  }
  IS_TRAP_HANDLER_ENABLED.load(Ordering::Relaxed)
}

/// trap-handler.h:163 SetLandingPad
/// 设置故障接管后的续行地址
pub fn set_landing_pad(landing_pad: usize) {
  LANDING_PAD.store(landing_pad, Ordering::SeqCst);
}
