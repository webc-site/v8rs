#!/usr/bin/env bun
// tool/src/execution/stack-guard.js — StackGuard 中断表
// 覆盖: src/execution/stack-guard.h + stack-guard.cc
// 产出: v8-execution-isolate/src/stack_guard.rs
// 规则: INTERRUPT_LIST 12 项全量 (名/id/level 三元组) → InterruptFlag 位枚举 + 中断管理。
// 破环登记: 中断触发 GC/装码 → 经 Isolate 回调, StackGuard 自身只存标志位与限值。
// 复跑幂等: 纯文本扫描 + 确定性输出。
import { banner, readV8, writeSrc } from "../../lib.js";

const GEN = "src/execution/stack-guard.js";
const FILES = ["src/execution/stack-guard.h", "src/execution/stack-guard.cc"];
const txt = Object.fromEntries(FILES.map((f) => [f, readV8(f)]));
const die = (m) => { throw new Error(`[stack-guard.js] ${m}`); };
const count = (s, re) => (s.match(re) ?? []).length;

const h = txt["src/execution/stack-guard.h"];
const listRegion = h.slice(h.indexOf("#define INTERRUPT_LIST"), h.indexOf("#define V(NAME, Name, id, interrupt_level)                   \\"));
// ① 中断项: 行首 V( 12 条
const headV = count(listRegion, /^  V\(/gm);
if (headV !== 12) die(`INTERRUPT_LIST 行首 V( 数=${headV}, 期望 12`);
const items = [...listRegion.matchAll(/V\((\w+),\s*(\w+),\s*(\d+),[\s\\]*InterruptLevel::(\w+)\)/g)].map((m) => ({
  name: m[1],
  methodName: m[2],
  id: Number(m[3]),
  level: m[4],
}));
if (items.length !== 12) die(`INTERRUPT_LIST 三元组数=${items.length}, 期望 12`);
// ② id 连续 0..11
const ids = items.map((m) => m.id).sort((a, b) => a - b);
if (ids.join(",") !== "0,1,2,3,4,5,6,7,8,9,10,11") die(`中断 id 不连续: ${ids}`);
// ③ 代表针 + 等级枚举针
for (const needle of ["V(TERMINATE_EXECUTION, TerminateExecution, 0, InterruptLevel::kNoGC)", "enum class InterruptLevel { kNoGC, kNoHeapWrites, kAnyEffect }", "enum InterruptFlag : uint32_t", "class V8_EXPORT_PRIVATE V8_NODISCARD StackGuard final {"]) {
  if (!h.includes(needle)) die(`缺针 ${needle}`);
}
// ④ 三等级各自至少 1 项
for (const lv of ["kNoGC", "kNoHeapWrites", "kAnyEffect"]) {
  if (count(listRegion, new RegExp(lv, "g")) < 1) die(`等级 ${lv} 无项`);
}

const toSnakeCase = (str) =>
  str.replace(/([a-z0-9])([A-Z])/g, "$1_$2").toLowerCase();

const out = [];
out.push(banner(GEN, FILES.join(", "), "StackGuard 中断表 12 项与栈限防护"));
out.push(`
use core::sync::atomic::{AtomicU32, AtomicUsize, Ordering};

/// 中断处理效果等级 (对应原版 InterruptLevel)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum InterruptLevel {
  /// 不触发 GC (kNoGC)
  NoGC = 0,
  /// 允许读取堆，但不写入堆 (kNoHeapWrites)
  NoHeapWrites = 1,
  /// 任意副作用，包括 GC 和装载代码 (kAnyEffect)
  AnyEffect = 2,
}

impl InterruptLevel {
  /// 是否允许在该等级下执行 target_level 要求的操作
  #[inline]
  pub const fn allows(self, target_level: InterruptLevel) -> bool {
    (target_level as u8) <= (self as u8)
  }
}

/// 12 项中断原因标志枚举与位定义 (对应原版 InterruptFlag : uint32_t)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum InterruptType {`);

for (const item of items) {
  out.push(`  /// id=${item.id}, level=${item.level}`);
  out.push(`  ${item.methodName} = ${item.id},`);
}

out.push(`}

impl InterruptType {
  /// 中断标志位掩码 (1 << id)
  #[inline]
  pub const fn mask(self) -> u32 {
    1 << (self as u8)
  }

  /// 对应中断等级
  #[inline]
  pub const fn level(self) -> InterruptLevel {
    match self {`);

for (const item of items) {
  const lv = item.level.replace(/^k/, "");
  out.push(`      InterruptType::${item.methodName} => InterruptLevel::${lv},`);
}

out.push(`    }
  }

  /// 原版大写名称
  #[inline]
  pub const fn name(self) -> &'static str {
    match self {`);

for (const item of items) {
  out.push(`      InterruptType::${item.methodName} => "${item.name}",`);
}

out.push(`    }
  }
}

/// 中断标志位常量集合
pub mod interrupt_flags {
`);

for (const item of items) {
  out.push(`  /// 原版 ${item.name} = 1 << ${item.id}`);
  out.push(`  pub const ${item.name}: u32 = 1 << ${item.id};`);
}

out.push(`
  /// 所有 12 项中断组合掩码 (对应原版 ALL_INTERRUPTS)
  pub const ALL_INTERRUPTS: u32 = (1 << 12) - 1;

  /// 根据中断等级计算过滤掩码 (对应原版 InterruptLevelMask)
  #[inline]
  pub const fn interrupt_level_mask(level: super::InterruptLevel) -> u32 {
    let mut mask = 0u32;`);

for (const item of items) {
  const lv = item.level.replace(/^k/, "");
  out.push(`    if (super::InterruptLevel::${lv} as u8) <= (level as u8) {`);
  out.push(`      mask |= ${item.name};`);
  out.push(`    }`);
}

out.push(`    mask
  }
}

/// 栈限与中断哨兵管理 (StackGuard)
#[derive(Debug)]
pub struct StackGuard {
  /// JS 栈限 (触发中断检查的当前限制)
  jslimit: AtomicUsize,
  /// 真实 JS 栈限 (底线)
  real_jslimit: AtomicUsize,
  /// C++ 栈限
  climit: AtomicUsize,
  /// 真实 C++ 栈限
  real_climit: AtomicUsize,
  /// 当前处于激活状态的中断原因位图
  interrupt_flags: AtomicU32,
}

impl Default for StackGuard {
  #[inline]
  fn default() -> Self {
    Self::new()
  }
}

impl StackGuard {
  /// 中断哨兵标记地址 (对应原版 kInterruptLimit = ~0)
  pub const INTERRUPT_LIMIT: usize = usize::MAX;

  /// 新建 StackGuard 实例
  pub const fn new() -> Self {
    Self {
      jslimit: AtomicUsize::new(Self::INTERRUPT_LIMIT),
      real_jslimit: AtomicUsize::new(0),
      climit: AtomicUsize::new(Self::INTERRUPT_LIMIT),
      real_climit: AtomicUsize::new(0),
      interrupt_flags: AtomicU32::new(0),
    }
  }

  /// 设置真实栈限制 (由运行时根据当前线程栈顶和栈大小初始化)
  pub fn set_stack_limit(&self, limit: usize) {
    self.real_jslimit.store(limit, Ordering::Release);
    self.real_climit.store(limit, Ordering::Release);
    // 若无中断待处理，同步更新当前限值
    if self.interrupt_flags.load(Ordering::Acquire) == 0 {
      self.jslimit.store(limit, Ordering::Release);
      self.climit.store(limit, Ordering::Release);
    }
  }

  /// 当前生效的 jslimit
  #[inline]
  pub fn jslimit(&self) -> usize {
    self.jslimit.load(Ordering::Acquire)
  }

  /// 真实底线 real_jslimit
  #[inline]
  pub fn real_jslimit(&self) -> usize {
    self.real_jslimit.load(Ordering::Acquire)
  }

  /// 当前生效的 climit
  #[inline]
  pub fn climit(&self) -> usize {
    self.climit.load(Ordering::Acquire)
  }

  /// 真实底线 real_climit
  #[inline]
  pub fn real_climit(&self) -> usize {
    self.real_climit.load(Ordering::Acquire)
  }

  /// 请求中断 (设置标志并下压栈限至 INTERRUPT_LIMIT 强制进入运行时拦截)
  pub fn request_interrupt(&self, flag: InterruptType) {
    self.interrupt_flags.fetch_or(flag.mask(), Ordering::SeqCst);
    self.jslimit.store(Self::INTERRUPT_LIMIT, Ordering::SeqCst);
    self.climit.store(Self::INTERRUPT_LIMIT, Ordering::SeqCst);
  }

  /// 清除特定中断
  pub fn clear_interrupt(&self, flag: InterruptType) {
    let old = self.interrupt_flags.fetch_and(!flag.mask(), Ordering::SeqCst);
    // 若所有中断已清空，还原为真实底线
    if (old & !flag.mask()) == 0 {
      let r_js = self.real_jslimit.load(Ordering::Acquire);
      let r_c = self.real_climit.load(Ordering::Acquire);
      self.jslimit.store(r_js, Ordering::Release);
      self.climit.store(r_c, Ordering::Release);
    }
  }

  /// 检查特定中断是否处于 pending 状态
  #[inline]
  pub fn check_interrupt(&self, flag: InterruptType) -> bool {
    (self.interrupt_flags.load(Ordering::Acquire) & flag.mask()) != 0
  }

  /// 检查是否存在任何中断请求
  #[inline]
  pub fn has_any_interrupt(&self) -> bool {
    self.interrupt_flags.load(Ordering::Acquire) != 0
  }

  /// 检查特定等级及以下是否有中断请求
  #[inline]
  pub fn has_interrupt_for_level(&self, level: InterruptLevel) -> bool {
    let mask = interrupt_flags::interrupt_level_mask(level);
    (self.interrupt_flags.load(Ordering::Acquire) & mask) != 0
  }

  /// 当前挂起的中断掩码
  #[inline]
  pub fn pending_interrupts(&self) -> u32 {
    self.interrupt_flags.load(Ordering::Acquire)
  }

  /// 检查给定的栈指针是否发生溢出 (低于 real_jslimit)
  #[inline]
  pub fn is_stack_overflow(&self, current_sp: usize) -> bool {
    let limit = self.real_jslimit.load(Ordering::Acquire);
    limit != 0 && current_sp < limit
  }
`);

for (const item of items) {
  const snake = toSnakeCase(item.methodName);
  out.push(`  /// 原版 Check${item.methodName}`);
  out.push(`  #[inline]`);
  out.push(`  pub fn check_${snake}(&self) -> bool {`);
  out.push(`    self.check_interrupt(InterruptType::${item.methodName})`);
  out.push(`  }`);
  out.push(``);
  out.push(`  /// 原版 Request${item.methodName}`);
  out.push(`  #[inline]`);
  out.push(`  pub fn request_${snake}(&self) {`);
  out.push(`    self.request_interrupt(InterruptType::${item.methodName});`);
  out.push(`  }`);
  out.push(``);
  out.push(`  /// 原版 Clear${item.methodName}`);
  out.push(`  #[inline]`);
  out.push(`  pub fn clear_${snake}(&self) {`);
  out.push(`    self.clear_interrupt(InterruptType::${item.methodName});`);
  out.push(`  }`);
  out.push(``);
}

out.push(`}

// ── 编译期静态断言 ──
const _: () = {
  assert!(interrupt_flags::ALL_INTERRUPTS == 0xFFF);
  assert!(interrupt_flags::interrupt_level_mask(InterruptLevel::NoGC) == 0x041);
  assert!(interrupt_flags::interrupt_level_mask(InterruptLevel::NoHeapWrites) == 0xD73);
  assert!(interrupt_flags::interrupt_level_mask(InterruptLevel::AnyEffect) == 0xFFF);
};
`);

writeSrc("v8-execution-isolate", "stack_guard.rs", out.join("\n") + "\n");
console.log(`OK stack-guard.js: 中断 12 项 id 0..11 → v8-execution-isolate/src/stack_guard.rs`);
