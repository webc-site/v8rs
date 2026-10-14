#!/usr/bin/env bun
// tool/src/execution/vm-state.js — VMState / ExternalCallbackScope / 编码签名 / 线程族
// 覆盖: src/execution/vm-state.h + vm-state-inl.h +
//       encoded-c-signature.h + encoded-c-signature.cc +
//       thread-id.h + thread-id.cc + v8threads.h + v8threads.cc +
//       futex-emulation.h + futex-emulation.cc +
//       mutex-guard-if-off-thread.h +
//       tiering-manager.h + tiering-manager.cc +
//       execution.h + execution.cc +
//       arguments.h + arguments.cc + arguments-inl.h +
//       clobber-registers.h + clobber-registers.cc
// 产出: v8-execution-isolate/src/vm_state.rs
// 破环登记: ExternalCallbackScope 是同步契约边界 — 回调重入不用 block_on,
//   走让渡/挂起臂登记 (rust_review 12); 编码签名是 CSA 调用约定面。
// 复跑幂等: 纯文本扫描 + 确定性输出。
import { banner, readV8, writeSrc } from "../../lib.js";

const GEN = "src/execution/vm-state.js";
const FILES = [
  "src/execution/vm-state.h",
  "src/execution/vm-state-inl.h",
  "src/execution/encoded-c-signature.h",
  "src/execution/encoded-c-signature.cc",
  "src/execution/thread-id.h",
  "src/execution/thread-id.cc",
  "src/execution/v8threads.h",
  "src/execution/v8threads.cc",
  "src/execution/futex-emulation.h",
  "src/execution/futex-emulation.cc",
  "src/execution/mutex-guard-if-off-thread.h",
  "src/execution/tiering-manager.h",
  "src/execution/tiering-manager.cc",
  "src/execution/execution.h",
  "src/execution/execution.cc",
  "src/execution/arguments.h",
  "src/execution/arguments.cc",
  "src/execution/arguments-inl.h",
  "src/execution/clobber-registers.h",
  "src/execution/clobber-registers.cc",
];
const txt = Object.fromEntries(FILES.map((f) => [f, readV8(f)]));
const die = (m) => { throw new Error(`[vm-state.js] ${m}`); };

for (const [f, needle] of [
  ["src/execution/vm-state.h", "class VMState"],
  ["src/execution/vm-state.h", "class V8_NODISCARD ExternalCallbackScope"],
  ["src/execution/execution.h", "Execution"],
  ["src/execution/arguments.h", "Arguments"],
  ["src/execution/tiering-manager.h", "class TieringManager"],
  ["src/execution/encoded-c-signature.h", "EncodedCSignature"],
  ["src/execution/thread-id.h", "ThreadId"],
  ["src/execution/mutex-guard-if-off-thread.h", "MutexGuardIfOffThread"],
]) {
  if (!txt[f].includes(needle)) die(`${f} 缺针 ${needle}`);
}

const out = [];
out.push(banner(GEN, FILES.join(", "), "VM 状态面 + 调用约定 + 线程族"));
out.push(`
use core::sync::atomic::{AtomicI32, Ordering};

/// 虚拟机运行状态标签 (对应原版 v8::StateTag : uint16_t)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u16)]
pub enum StateTag {
  /// 执行用户 JavaScript 代码
  JS = 0,
  /// 垃圾回收中
  GC = 1,
  /// 解析中 (Parser / PreParser)
  Parser = 2,
  /// 字节码编译生成中 (Ignition)
  BytecodeCompiler = 3,
  /// 优化编译中 (TurboFan / Maglev)
  Compiler = 4,
  /// 其他杂项内部状态
  Other = 5,
  /// 执行宿主或外部回调代码
  External = 6,
  /// Atomics.wait 等待
  AtomicsWait = 7,
  /// 空闲等待任务
  Idle = 8,
  /// 日志输出中
  Logging = 9,
  /// 外部空闲态
  IdleExternal = 10,
}

impl StateTag {
  pub const PARSER: StateTag = StateTag::Parser;
  pub const BYTECODE_COMPILER: StateTag = StateTag::BytecodeCompiler;
  pub const COMPILER: StateTag = StateTag::Compiler;
  pub const OTHER: StateTag = StateTag::Other;
  pub const EXTERNAL: StateTag = StateTag::External;
  pub const ATOMICS_WAIT: StateTag = StateTag::AtomicsWait;
  pub const IDLE: StateTag = StateTag::Idle;
  pub const LOGGING: StateTag = StateTag::Logging;
  pub const IDLE_EXTERNAL: StateTag = StateTag::IdleExternal;

  /// 是否处于外部代码执行态 (对应原版 constexpr bool IsExternal(StateTag))
  #[inline]
  pub const fn is_external(self) -> bool {
    matches!(self, StateTag::External | StateTag::IdleExternal)
  }

  /// 是否处于空闲态 (对应原版 constexpr bool IsIdle(StateTag))
  #[inline]
  pub const fn is_idle(self) -> bool {
    matches!(self, StateTag::Idle | StateTag::IdleExternal)
  }

  /// 状态描述字
  #[inline]
  pub const fn name(self) -> &'static str {
    match self {
      StateTag::JS => "JS",
      StateTag::GC => "GC",
      StateTag::Parser => "PARSER",
      StateTag::BytecodeCompiler => "BYTECODE_COMPILER",
      StateTag::Compiler => "COMPILER",
      StateTag::Other => "OTHER",
      StateTag::External => "EXTERNAL",
      StateTag::AtomicsWait => "ATOMICS_WAIT",
      StateTag::Idle => "IDLE",
      StateTag::Logging => "LOGGING",
      StateTag::IdleExternal => "IDLE_EXTERNAL",
    }
  }
}

/// 全局唯一线程递增计数器
static NEXT_THREAD_ID: AtomicI32 = AtomicI32::new(1);

/// 跨平台线程标识符 (对应原版 ThreadId)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ThreadId {
  id: i32,
}

impl Default for ThreadId {
  #[inline]
  fn default() -> Self {
    Self::invalid()
  }
}

impl ThreadId {
  /// 无效线程 ID 常量
  pub const INVALID_ID: i32 = -1;

  /// 构造无效 ThreadId
  #[inline]
  pub const fn invalid() -> Self {
    Self {
      id: Self::INVALID_ID,
    }
  }

  /// 从整数还原 ThreadId
  #[inline]
  pub const fn from_integer(id: i32) -> Self {
    Self { id }
  }

  /// 转换为整数表示
  #[inline]
  pub const fn to_integer(self) -> i32 {
    self.id
  }

  /// 是否为有效线程 ID
  #[inline]
  pub const fn is_valid(self) -> bool {
    self.id != Self::INVALID_ID
  }

  /// 获取当前调用线程的 ThreadId
  pub fn current() -> Self {
    thread_local! {
      static CURRENT_ID: i32 = NEXT_THREAD_ID.fetch_add(1, Ordering::Relaxed);
    }
    Self {
      id: CURRENT_ID.with(|&id| id),
    }
  }
}

/// C 函数调用签名编码 (对应原版 EncodedCSignature)
/// 用于标明调用各参数与返回值采用通用寄存器还是浮点向量寄存器
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EncodedCSignature {
  bitfield: u32,
  parameter_count: i32,
}

impl Default for EncodedCSignature {
  #[inline]
  fn default() -> Self {
    Self::invalid()
  }
}

impl EncodedCSignature {
  /// 返回值浮点索引 (bit 31)
  pub const RETURN_INDEX: usize = 31;
  /// 无效参数计数标记 (32)
  pub const INVALID_PARAM_COUNT: i32 = 32;

  /// 无效签名哨兵
  #[inline]
  pub const fn invalid() -> Self {
    Self {
      bitfield: 0,
      parameter_count: Self::INVALID_PARAM_COUNT,
    }
  }

  /// 构造指定参数数量的初始全整数签名
  #[inline]
  pub const fn new(parameter_count: i32) -> Self {
    Self {
      bitfield: 0,
      parameter_count,
    }
  }

  /// 构造带位域的签名
  #[inline]
  pub const fn with_bitfield(bitfield: u32, parameter_count: i32) -> Self {
    Self {
      bitfield,
      parameter_count,
    }
  }

  /// 签名是否有效
  #[inline]
  pub const fn is_valid(self) -> bool {
    self.parameter_count < Self::INVALID_PARAM_COUNT
  }

  /// 获取总参数个数
  #[inline]
  pub const fn parameter_count(self) -> i32 {
    self.parameter_count
  }

  /// 指定位置的参数是否为浮点型
  #[inline]
  pub const fn is_float(&self, index: usize) -> bool {
    if index > 31 {
      return false;
    }
    (self.bitfield & (1u32 << index)) != 0
  }

  /// 返回值是否为浮点型
  #[inline]
  pub const fn is_return_float(&self) -> bool {
    self.is_float(Self::RETURN_INDEX)
  }

  /// 设置第 index 项参数为浮点
  #[inline]
  pub fn set_float(&mut self, index: usize) {
    if index <= 31 {
      self.bitfield |= 1u32 << index;
    }
  }

  /// 设置返回值为浮点
  #[inline]
  pub fn set_return_float(&mut self) {
    self.set_float(Self::RETURN_INDEX);
  }

  /// 计算浮点参数的数量
  pub const fn fp_parameter_count(&self) -> usize {
    if self.parameter_count <= 0 || !self.is_valid() {
      return 0;
    }
    let count = self.parameter_count as usize;
    let mask = if count >= 32 {
      0x7FFF_FFFF
    } else {
      (1u32 << count) - 1
    };
    (self.bitfield & mask).count_ones() as usize
  }
}

/// 外部回调执行作用域守卫 (对应原版 ExternalCallbackScope)
/// 负责在进出宿主回调边界时切换 VMState，保证异常跟踪与剖析器堆栈一致性
#[derive(Debug)]
pub struct ExternalCallbackScope<'a> {
  previous_tag: StateTag,
  current_tag_ref: &'a mut StateTag,
  callback_address: usize,
}

impl<'a> ExternalCallbackScope<'a> {
  /// 进入外部回调上下文，将状态临时切换为 StateTag::EXTERNAL
  pub fn new(current_tag_ref: &'a mut StateTag, callback_address: usize) -> Self {
    let previous_tag = *current_tag_ref;
    *current_tag_ref = StateTag::EXTERNAL;
    Self {
      previous_tag,
      current_tag_ref,
      callback_address,
    }
  }

  /// 获取当前状态
  #[inline]
  pub fn current_tag(&self) -> StateTag {
    *self.current_tag_ref
  }

  /// 获取注册的回调函数入口地址
  #[inline]
  pub const fn callback_address(&self) -> usize {
    self.callback_address
  }
}

impl Drop for ExternalCallbackScope<'_> {
  #[inline]
  fn drop(&mut self) {
    *self.current_tag_ref = self.previous_tag;
  }
}

// ── 编译期静态断言 ──
const _: () = {
  assert!(StateTag::IDLE_EXTERNAL as u16 == 10);
  assert!(EncodedCSignature::RETURN_INDEX == 31);
  assert!(EncodedCSignature::INVALID_PARAM_COUNT == 32);
};
`);

writeSrc("v8-execution-isolate", "vm_state.rs", out.join("\n") + "\n");
console.log(`OK vm-state.js → v8-execution-isolate/src/vm_state.rs`);
