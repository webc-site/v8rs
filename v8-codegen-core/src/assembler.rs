// 由 tool/src/codegen/assembler.h.js 从 v8/src/codegen/assembler.{h,cc};src/codegen/assembler-inl.h;src/codegen/assembler-arch.h 生成; 修订就地改, 复跑覆盖
// 汇编器基类常量/枚举/选项默认值/缓冲区接口/签名骨架

// assembler-arch.h 平台派发: x64 → assembler-x64.h, arm64 → assembler-arm64.h
// (各自架构 crate 承接); 其余架构 (ia32/arm/ppc/mips64/loong64/s390/riscv) 登记跳过。

use std::ptr::null_mut;

/// 原版 kMaximalBufferSize = 512 * MB
pub const K_MAXIMAL_BUFFER_SIZE: i32 = 536870912;
/// 原版 kMinimalBufferSize = 128 (须 >= 2 * 平台 kGap)
pub const K_MINIMAL_BUFFER_SIZE: i32 = 128;
/// 原版 kDefaultBufferSize = 4 * KB
pub const K_DEFAULT_BUFFER_SIZE: i32 = 4096;

/// 原版 CodeComment::kIndentWidth
pub const K_INDENT_WIDTH: i32 = 2;

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
    Self::default_for(null_mut())
  }
}

impl AssemblerOptions {
  /// 原版 AssemblerOptions::Default(isolate) (assembler.cc:56):
  /// 返回安全有效的默认选项
  #[must_use]
  pub const fn default_for(_isolate: *mut ()) -> Self {
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

/// 原版 class AssemblerBuffer: 可增长代码缓冲接口
pub trait AssemblerBuffer {
  /// 缓冲起点
  fn start(&self) -> *mut u8;
  /// 缓冲大小
  fn size(&self) -> i32;
  /// 返回增长后的新缓冲 (旧数据仍可读, 不再写)
  fn grow(&self, new_size: i32) -> Box<dyn AssemblerBuffer>;
}

/// 原版 DefaultAssemblerBuffer (assembler.cc:95): 自持可增长堆缓冲
pub struct DefaultAssemblerBuffer {
  /// 原版 buffer_ (base::UniqueArray<uint8_t>)
  data: Vec<u8>,
}

impl DefaultAssemblerBuffer {
  /// v8: assembler.cc:97-104 构造: size = max(kMinimalBufferSize, size)
  #[must_use]
  pub fn new(size: i32) -> Self {
    let cap = size.max(K_MINIMAL_BUFFER_SIZE) as usize;
    // 原版 NewForOverwrite 内容未定义; DEBUG 构建 ZapCode 填 0xff
    let mut data = vec![0u8; cap];
    if cfg!(debug_assertions) {
      data.fill(0xff);
    }
    Self { data }
  }
}

impl AssemblerBuffer for DefaultAssemblerBuffer {
  fn start(&self) -> *mut u8 {
    self.data.as_ptr() as *mut u8
  }

  fn size(&self) -> i32 {
    self.data.len() as i32
  }

  /// v8: assembler.cc:109-112 Grow: DCHECK_LT(size, new_size) 后新建缓冲
  fn grow(&self, new_size: i32) -> Box<dyn AssemblerBuffer> {
    debug_assert!((self.size() as i64) < new_size as i64, "只增不减");
    Box::new(Self::new(new_size))
  }
}

/// 原版 ExternalAssemblerBufferImpl (assembler.cc:118): 外部固定缓冲, 不可增长
pub struct ExternalAssemblerBufferImpl {
  /// 原版 start_
  start_: *mut u8,
  /// 原版 size_
  size_: i32,
}

impl AssemblerBuffer for ExternalAssemblerBufferImpl {
  fn start(&self) -> *mut u8 {
    self.start_
  }

  fn size(&self) -> i32 {
    self.size_
  }

  /// v8: assembler.cc:127-129 Grow: 原版 FATAL("Cannot grow external assembler buffer")
  fn grow(&self, _new_size: i32) -> Box<dyn AssemblerBuffer> {
    unreachable!("Cannot grow external assembler buffer (原版 FATAL)")
  }
}

/// 原版 ExternalAssemblerBuffer(buffer, size) (assembler.cc:165)
///
/// # Safety
/// `buffer` 起 `size` 字节须为调用方保有的合法可写内存, 生命周期覆盖使用该缓冲的全部装配过程。
#[must_use]
pub unsafe fn external_assembler_buffer(buffer: *mut u8, size: i32) -> Box<dyn AssemblerBuffer> {
  debug_assert!(size >= 0, "外部缓冲大小非负");
  Box::new(ExternalAssemblerBufferImpl { start_: buffer, size_ })
}

/// 原版 NewAssemblerBuffer(size) (assembler.cc:170)
#[must_use]
pub fn new_assembler_buffer(size: i32) -> Box<dyn AssemblerBuffer> {
  debug_assert!(size >= K_MINIMAL_BUFFER_SIZE && size <= K_MAXIMAL_BUFFER_SIZE);
  Box::new(DefaultAssemblerBuffer::new(size))
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
    Self {
      indirect_pointer_tag_: K_INDIRECT_POINTER_NULL_TAG,
    }
  }

  /// 原版 ForIndirectPointerSlot(tag)
  #[inline]
  #[must_use]
  pub const fn for_indirect_pointer_slot(tag: u32) -> Self {
    Self {
      indirect_pointer_tag_: tag,
    }
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
      jump_optimization_info_: null_mut(),
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
