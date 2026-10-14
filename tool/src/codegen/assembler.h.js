#!/usr/bin/env bun
// tool/src/codegen/assembler.h.js
// 机械提取 v8/src/codegen/assembler.{h,cc} (+assembler-inl.h, assembler-arch.h):
// 汇编器基类常量/枚举/选项默认值/缓冲区接口/签名骨架
// → v8-codegen-core/src/assembler.rs

import { readV8, writeSrc, banner } from "../../lib.js";

const hSrc = readV8("src/codegen/assembler.h");
const ccSrc = readV8("src/codegen/assembler.cc");
const inlSrc = readV8("src/codegen/assembler-inl.h");
const archSrc = readV8("src/codegen/assembler-arch.h");

const m = (re, what, src = hSrc) => {
  const r = src.match(re);
  if (!r) throw new Error(`未找到 ${what}`);
  return r;
};

// 常量对数
const maximal = m(/static constexpr int kMaximalBufferSize = (\d+) \* MB;/, "kMaximalBufferSize");
const minimal = m(/static constexpr int kMinimalBufferSize = (\d+);/, "kMinimalBufferSize");
const defSize = m(/static constexpr int kDefaultBufferSize = (\d+) \* KB;/, "kDefaultBufferSize");
if (maximal[1] !== "512") throw new Error("kMaximalBufferSize 异常");
if (minimal[1] !== "128") throw new Error("kMinimalBufferSize 异常");
if (defSize[1] !== "4") throw new Error("kDefaultBufferSize 异常");
const K_MAXIMAL_BUFFER_SIZE = 512 * 1024 * 1024;
const K_MINIMAL_BUFFER_SIZE = 128;
const K_DEFAULT_BUFFER_SIZE = 4 * 1024;

// 枚举对数: BuiltinCallJumpMode 4 / BufferGrowthStrategy 2 / CheckPolicy 2
const bcjm = m(/enum class BuiltinCallJumpMode \{([^}]*)\}/, "BuiltinCallJumpMode")[1];
const bcjmVariants = [...bcjm.matchAll(/k(\w+),/g)].map((x) => "k" + x[1]);
if (bcjmVariants.length !== 4) throw new Error(`BuiltinCallJumpMode ${bcjmVariants.length} != 4`);
for (const v of ["kDouble", "kDoubleCapped1MB"]) {
  if (!hSrc.includes(v)) throw new Error(`缺 BufferGrowthStrategy ${v}`);
}
for (const v of ["kCheckSupported", "kDontCheckSupported"]) {
  if (!hSrc.includes(v)) throw new Error(`缺 CheckPolicy ${v}`);
}
const kIndentWidth = m(/static const int kIndentWidth = (\d+);/, "kIndentWidth")[1];

// AssemblerOptions 字段清点 (对数): 12 个数据字段
const optFields = [
  "record_reloc_info_for_serialization", "enable_root_relative_access", "enable_simulator_code",
  "isolate_independent_code", "builtin_call_jump_mode",
  "use_pc_relative_calls_and_jumps_for_mksnapshot", "generating_embedded_builtin",
  "code_range_base", "collect_win64_unwind_info", "emit_code_comments", "is_wasm",
];
for (const f of optFields) {
  if (!hSrc.includes(`bool ${f}`) && !hSrc.includes(` ${f} =`)) throw new Error(`缺选项 ${f}`);
}

// .cc 关键方法
for (const want of ["AssemblerBase::AssemblerBase", "AssemblerOptions::Default",
  "NewAssemblerBuffer", "ExternalAssemblerBuffer"]) {
  if (!ccSrc.includes(want)) throw new Error(`assembler.cc 缺 ${want}`);
}
// arch 派发登记
if (!archSrc.includes("V8_TARGET_ARCH_X64") || !archSrc.includes("V8_TARGET_ARCH_ARM64"))
  throw new Error("assembler-arch.h 缺目标分支");

let rust = banner(
  "src/codegen/assembler.h.js",
  "src/codegen/assembler.{h,cc};src/codegen/assembler-inl.h;src/codegen/assembler-arch.h",
  "汇编器基类常量/枚举/选项默认值/缓冲区接口/签名骨架",
);

rust += `
// assembler-arch.h 平台派发: x64 → assembler-x64.h, arm64 → assembler-arm64.h
// (各自架构 crate 承接); 其余架构 (ia32/arm/ppc/mips64/loong64/s390/riscv) 登记跳过。

/// 原版 kMaximalBufferSize = 512 * MB
pub const K_MAXIMAL_BUFFER_SIZE: i32 = ${K_MAXIMAL_BUFFER_SIZE};
/// 原版 kMinimalBufferSize = 128 (须 >= 2 * 平台 kGap)
pub const K_MINIMAL_BUFFER_SIZE: i32 = ${K_MINIMAL_BUFFER_SIZE};
/// 原版 kDefaultBufferSize = 4 * KB
pub const K_DEFAULT_BUFFER_SIZE: i32 = ${K_DEFAULT_BUFFER_SIZE};

/// 原版 CodeComment::kIndentWidth
pub const K_INDENT_WIDTH: i32 = ${kIndentWidth};

/// 原版 enum class BuiltinCallJumpMode: builtin 调用/尾跳生成方式
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum BuiltinCallJumpMode {
  /// 绝对地址内嵌
  kAbsolute = 0,
  /// PC 相对 (目标须在 kMaxPCRelativeCodeRangeInMB 内)
  kPCRelative = 1,
  /// 间接: 从 builtin 入口表加载目标
  kIndirect = 2,
  /// 仅为 mksnapshot 生成 embedded builtins (待移除, 见 TODO v8:11527)
  kForMksnapshot = 3,
}

/// 原版 AssemblerBase::BufferGrowthStrategy
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum BufferGrowthStrategy {
  kDouble = 0,
  kDoubleCapped1MB = 1,
}

/// 原版 CpuFeatureScope::CheckPolicy
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum CheckPolicy {
  kCheckSupported = 0,
  kDontCheckSupported = 1,
}

/// 原版 struct AssemblerOptions (字段默认值机械对应)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AssemblerOptions {
  /// 序列化 (snapshot/wasm) 需要外部引用重定位; 短命码可关
  pub record_reloc_info_for_serialization: bool, // = true
  /// 根相对访问任意未标记地址; 仅短命码合法
  pub enable_root_relative_access: bool, // = false
  /// 模拟器专用序列 (USE_SIMULATOR_BOOL)
  pub enable_simulator_code: bool,
  /// isolate 无关常量 (经根数组间接)
  pub isolate_independent_code: bool, // = false
  /// builtin 调用/尾跳生成方式
  pub builtin_call_jump_mode: BuiltinCallJumpMode, // = kAbsolute
  /// mksnapshot 亦用 PC 相对调用 (序列化测试下回退间接)
  pub use_pc_relative_calls_and_jumps_for_mksnapshot: bool, // = false
  /// 生成 embedded builtin 且 isolate 不可用时置位
  pub generating_embedded_builtin: bool, // = false
  /// 代码区基址
  pub code_range_base: u64, // = 0
  /// 收集 Win64 unwind 信息
  pub collect_win64_unwind_info: bool, // = false
  /// 是否产出代码注释 (v8_flags.code_comments)
  pub emit_code_comments: bool,
  /// wasm 代码
  pub is_wasm: bool, // = false
}

impl Default for AssemblerOptions {
  /// 机械对应原版成员默认值 (emit_code_comments 取 flags 默认 = true;
  /// enable_simulator_code 取 USE_SIMULATOR_BOOL, x64/arm64 本地构建 = false)
  fn default() -> Self {
    Self {
      record_reloc_info_for_serialization: true,
      enable_root_relative_access: false,
      enable_simulator_code: false,
      isolate_independent_code: false,
      builtin_call_jump_mode: BuiltinCallJumpMode::kAbsolute,
      use_pc_relative_calls_and_jumps_for_mksnapshot: false,
      generating_embedded_builtin: false,
      code_range_base: 0,
      collect_win64_unwind_info: false,
      emit_code_comments: true,
      is_wasm: false,
    }
  }
}

impl AssemblerOptions {
  /// 原版 AssemblerOptions::Default(isolate) — 骨架
  pub fn default_for(_isolate: *mut ()) -> Self {
    todo!("待 Developer 修订: 对齐 assembler.cc AssemblerOptions::Default")
  }
}

/// 原版 class AssemblerBuffer: 可增长代码缓冲接口
pub trait AssemblerBuffer {
  /// 缓冲起点
  fn start(&self) -> *mut u8;
  /// 缓冲大小
  fn size(&self) -> i32;
  /// 返回增长后的新缓冲 (旧数据仍可读, 不再写)
  fn grow(&self, new_size: i32) -> Box<dyn AssemblerBuffer>;
}

/// 原版 ExternalAssemblerBuffer(buffer, size): 外部固定缓冲 — 骨架
pub fn external_assembler_buffer(_buffer: *mut u8, _size: i32) -> Box<dyn AssemblerBuffer> {
  todo!("待 Developer 修订: 对齐 assembler.cc ExternalAssemblerBuffer")
}

/// 原版 NewAssemblerBuffer(size): 新可增长缓冲 — 骨架
pub fn new_assembler_buffer(size: i32) -> Box<dyn AssemblerBuffer> {
  debug_assert!(size >= K_MINIMAL_BUFFER_SIZE && size <= K_MAXIMAL_BUFFER_SIZE);
  todo!("待 Developer 修订: 对齐 assembler.cc NewAssemblerBuffer")
}

/// 原版 class SlotDescriptor: HeapObject 槽描述 (直接/间接指针)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SlotDescriptor {
  /// 原版 indirect_pointer_tag_
  indirect_pointer_tag_: u32,
}

/// 原版 kIndirectPointerNullTag (sandbox 头; 编码占位待接线)
pub const K_INDIRECT_POINTER_NULL_TAG: u32 = 0;

impl SlotDescriptor {
  /// 原版 contains_direct_pointer: tag == kIndirectPointerNullTag
  #[inline]
  #[must_use]
  pub const fn contains_direct_pointer(&self) -> bool {
    self.indirect_pointer_tag_ == K_INDIRECT_POINTER_NULL_TAG
  }

  /// 原版 contains_indirect_pointer
  #[inline]
  #[must_use]
  pub const fn contains_indirect_pointer(&self) -> bool {
    self.indirect_pointer_tag_ != K_INDIRECT_POINTER_NULL_TAG
  }

  /// 原版 indirect_pointer_tag()
  #[inline]
  #[must_use]
  pub const fn indirect_pointer_tag(&self) -> u32 {
    self.indirect_pointer_tag_
  }

  /// 原版 ForDirectPointerSlot
  #[inline]
  #[must_use]
  pub const fn for_direct_pointer_slot() -> Self {
    Self { indirect_pointer_tag_: K_INDIRECT_POINTER_NULL_TAG }
  }

  /// 原版 ForIndirectPointerSlot(tag)
  #[inline]
  #[must_use]
  pub const fn for_indirect_pointer_slot(tag: u32) -> Self {
    Self { indirect_pointer_tag_: tag }
  }

  /// 原版 ForTrustedPointerSlot(tag): 沙箱开 = 间接, 否则直接
  /// (本目标 V8_ENABLE_SANDBOX = true, 机械取间接分支)
  #[inline]
  #[must_use]
  pub const fn for_trusted_pointer_slot(tag: u32) -> Self {
    Self::for_indirect_pointer_slot(tag)
  }
}

/// 原版 class AssemblerBase: 平台无关汇编器基类 (状态面骨架)
pub struct AssemblerBase {
  /// 原版 options_
  options_: AssemblerOptions,
  /// 原版 buffer_start_ / pc_
  buffer_start_: *mut u8,
  pc_: *mut u8,
  /// 原版 enabled_cpu_features_ (位集, u64 承载 CpuFeature)
  enabled_cpu_features_: u64,
  /// 原版 predictable_code_size_
  predictable_code_size_: bool,
  /// 原版 constant_pool_available_
  constant_pool_available_: bool,
  /// 原版 jump_optimization_info_
  jump_optimization_info_: *mut (),
  /// 原版 comment_depth_ (V8_CODE_COMMENTS)
  comment_depth_: i32,
}

impl AssemblerBase {
  /// 原版 pc_offset(): pc_ - buffer_start_
  #[inline]
  #[must_use]
  pub fn pc_offset(&self) -> i32 {
    (self.pc_ as usize - self.buffer_start_ as usize) as i32
  }

  /// 原版 current_pc()
  #[inline]
  #[must_use]
  pub fn current_pc(&self) -> usize {
    self.pc_ as usize
  }

  /// 原版 skip_bytes(num_bytes)
  #[inline]
  pub fn skip_bytes(&mut self, num_bytes: i32) {
    unsafe {
      self.pc_ = self.pc_.byte_offset(num_bytes as isize);
    }
  }

  /// 原版 options()
  #[inline]
  #[must_use]
  pub const fn options(&self) -> &AssemblerOptions {
    &self.options_
  }

  /// 原版 IsEnabled(f): 位集探测 (仅 DCHECK 用; 真探测用 IsSupported)
  #[inline]
  #[must_use]
  pub const fn is_enabled(&self, f: u8) -> bool {
    (self.enabled_cpu_features_ & (1u64 << f)) != 0
  }

  /// 原版 EnableCpuFeature(f)
  #[inline]
  pub const fn enable_cpu_feature(&mut self, f: u8) {
    self.enabled_cpu_features_ |= 1u64 << f;
  }

  /// 原版构造 (AssemblerBase(options, buffer)) — 骨架
  pub fn new(options: AssemblerOptions, buffer: Box<dyn AssemblerBuffer>) -> Self {
    let start = buffer.start();
    let _ = buffer;
    Self {
      options_: options,
      buffer_start_: start,
      pc_: start,
      enabled_cpu_features_: 0,
      predictable_code_size_: false,
      constant_pool_available_: false,
      jump_optimization_info_: std::ptr::null_mut(),
      comment_depth_: 0,
    }
  }

  // —— 其余成员 (RecordJSDispatchHandle / code_comments_writer_ / FinalizeCode 等)
  // 待 Developer 修订接线。
}

/// 原版 using CodeObjectRequired = StrongAlias<CodeObjectRequiredTag, bool>
pub type CodeObjectRequired = bool;

// —— 提取不变式断言 ——
const _: () = {
  assert!(K_MAXIMAL_BUFFER_SIZE == 536_870_912);
  assert!(K_MINIMAL_BUFFER_SIZE == 128);
  assert!(K_DEFAULT_BUFFER_SIZE == 4096);
  assert!(BuiltinCallJumpMode::kAbsolute as u8 == 0);
  assert!(BuiltinCallJumpMode::kForMksnapshot as u8 == 3);
  assert!(BufferGrowthStrategy::kDoubleCapped1MB as u8 == 1);
  assert!(CheckPolicy::kDontCheckSupported as u8 == 1);
  let d = AssemblerOptions::default();
  assert!(d.record_reloc_info_for_serialization && !d.is_wasm);
  assert!(d.builtin_call_jump_mode == BuiltinCallJumpMode::kAbsolute);
  assert!(SlotDescriptor::for_direct_pointer_slot().contains_direct_pointer());
  assert!(SlotDescriptor::for_indirect_pointer_slot(7).contains_indirect_pointer());
};
`;

const p = writeSrc("v8-codegen-core", "assembler.rs", rust);
console.log(
  `assembler: 常量 4 / 枚举 3 / 选项字段 ${optFields.length} / 接口 1 / 骨架方法 ~20 → ${p}`,
);
