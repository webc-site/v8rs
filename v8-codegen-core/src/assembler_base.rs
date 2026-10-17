use core::ptr::null_mut;

use crate::error::{Error, Result};

pub const K_MINIMAL_BUFFER_SIZE: usize = 128;
pub const K_DEFAULT_BUFFER_SIZE: usize = 4 * 1024;
pub const K_MAXIMAL_BUFFER_SIZE: usize = 512 * 1024 * 1024;
pub const K_ONE_MB: usize = 1024 * 1024;

/// 内建调用跳转模式 (对应原版 v8::internal::BuiltinCallJumpMode)
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum BuiltinCallJumpMode {
  #[default]
  Absolute,
  PCRelative,
  Indirect,
  ForMksnapshot,
}

/// 平台无关汇编器配置 (对应原版 v8::internal::AssemblerOptions)
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AssemblerOptions {
  pub record_reloc_info_for_serialization: bool,
  pub enable_root_relative_access: bool,
  pub enable_simulator_code: bool,
  pub isolate_independent_code: bool,
  pub builtin_call_jump_mode: BuiltinCallJumpMode,
  pub use_pc_relative_calls_and_jumps_for_mksnapshot: bool,
  pub generating_embedded_builtin: bool,
  pub code_range_base: usize,
  pub collect_win64_unwind_info: bool,
  pub emit_code_comments: bool,
  pub is_wasm: bool,
}

impl Default for AssemblerOptions {
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
      builtin_call_jump_mode: BuiltinCallJumpMode::Absolute,
      use_pc_relative_calls_and_jumps_for_mksnapshot: false,
      generating_embedded_builtin: false,
      code_range_base: 0,
      collect_win64_unwind_info: false,
      emit_code_comments: false,
      is_wasm: false,
    }
  }
}

/// 缓冲增长策略 (对应原版 v8::internal::AssemblerBase::BufferGrowthStrategy)
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BufferGrowthStrategy {
  Double,
  DoubleCapped1MB,
}

/// 根据增长策略计算新缓冲区容量
pub fn compute_new_buffer_size(old_size: usize, strategy: BufferGrowthStrategy) -> Result<usize> {
  let initial = if old_size < K_MINIMAL_BUFFER_SIZE {
    K_MINIMAL_BUFFER_SIZE
  } else {
    old_size
  };

  let new_size = match strategy {
    BufferGrowthStrategy::Double => initial.saturating_mul(2),
    BufferGrowthStrategy::DoubleCapped1MB => {
      let double_size = initial.saturating_mul(2);
      let capped = initial.saturating_add(K_ONE_MB);
      double_size.min(capped)
    }
  };

  if new_size > K_MAXIMAL_BUFFER_SIZE {
    return Err(Error::BufferOverflow {
      requested: new_size,
      max: K_MAXIMAL_BUFFER_SIZE,
    });
  }

  Ok(new_size)
}

/// 汇编指令存储缓冲
#[derive(Debug)]
pub struct AssemblerBuffer {
  buffer: Vec<u8>,
}

impl AssemblerBuffer {
  pub fn new(capacity: usize) -> Self {
    let cap = capacity.max(K_MINIMAL_BUFFER_SIZE);
    Self {
      buffer: Vec::with_capacity(cap),
    }
  }

  #[inline]
  pub fn as_slice(&self) -> &[u8] {
    &self.buffer
  }

  #[inline]
  pub fn capacity(&self) -> usize {
    self.buffer.capacity()
  }

  #[inline]
  pub fn len(&self) -> usize {
    self.buffer.len()
  }

  #[inline]
  pub fn is_empty(&self) -> bool {
    self.buffer.is_empty()
  }

  /// 依据增长策略扩展容量
  pub fn grow(&mut self, strategy: BufferGrowthStrategy) -> Result<()> {
    let new_size = compute_new_buffer_size(self.buffer.capacity(), strategy)?;
    let additional = new_size.saturating_sub(self.buffer.len());
    self.buffer.reserve(additional);
    Ok(())
  }

  #[inline]
  pub fn push(&mut self, byte: u8) {
    self.buffer.push(byte);
  }

  #[inline]
  pub fn extend_from_slice(&mut self, slice: &[u8]) {
    self.buffer.extend_from_slice(slice);
  }

  #[inline]
  pub fn into_inner(self) -> Vec<u8> {
    self.buffer
  }

  #[inline]
  pub fn set_byte_at(&mut self, pos: usize, val: u8) {
    if pos < self.buffer.len() {
      self.buffer[pos] = val;
    }
  }

  #[inline]
  pub fn write_bytes_at(&mut self, pos: usize, bytes: &[u8]) {
    if pos + bytes.len() <= self.buffer.len() {
      self.buffer[pos..pos + bytes.len()].copy_from_slice(bytes);
    }
  }

  #[inline]
  pub fn byte_at(&self, pos: usize) -> u8 {
    self.buffer[pos]
  }

  #[inline]
  pub fn read_bytes_at<const N: usize>(&self, pos: usize) -> [u8; N] {
    let mut arr = [0u8; N];
    arr.copy_from_slice(&self.buffer[pos..pos + N]);
    arr
  }
}

impl Default for AssemblerBuffer {
  fn default() -> Self {
    Self::new(K_DEFAULT_BUFFER_SIZE)
  }
}

/// 平台无关的基础汇编器基类 (对应原版 v8::internal::AssemblerBase)
#[derive(Debug)]
pub struct AssemblerBase {
  options: AssemblerOptions,
  buffer: AssemblerBuffer,
  enabled_cpu_features: u64,
  predictable_code_size: bool,
}

impl AssemblerBase {
  pub fn new(options: AssemblerOptions) -> Self {
    Self {
      options,
      buffer: AssemblerBuffer::default(),
      enabled_cpu_features: 0,
      predictable_code_size: false,
    }
  }

  pub fn with_buffer(options: AssemblerOptions, buffer: AssemblerBuffer) -> Self {
    Self {
      options,
      buffer,
      enabled_cpu_features: 0,
      predictable_code_size: false,
    }
  }

  #[inline]
  pub const fn options(&self) -> &AssemblerOptions {
    &self.options
  }

  #[inline]
  pub fn pc_offset(&self) -> usize {
    self.buffer.len()
  }

  #[inline]
  pub const fn predictable_code_size(&self) -> bool {
    self.predictable_code_size
  }

  #[inline]
  pub fn set_predictable_code_size(&mut self, val: bool) {
    self.predictable_code_size = val;
  }

  #[inline]
  pub const fn enabled_cpu_features(&self) -> u64 {
    self.enabled_cpu_features
  }

  #[inline]
  pub fn enable_cpu_feature(&mut self, feature_bit: u8) {
    if feature_bit < 64 {
      self.enabled_cpu_features |= 1u64 << feature_bit;
    }
  }

  #[inline]
  pub const fn is_cpu_feature_enabled(&self, feature_bit: u8) -> bool {
    if feature_bit < 64 {
      (self.enabled_cpu_features & (1u64 << feature_bit)) != 0
    } else {
      false
    }
  }

  /// 预留所需剩余字节容量
  pub fn ensure_space(&mut self, space: usize) -> Result<()> {
    let available = self.buffer.capacity().saturating_sub(self.buffer.len());
    if available < space {
      let needed = self.buffer.len().saturating_add(space);
      if needed > K_MAXIMAL_BUFFER_SIZE {
        return Err(Error::BufferOverflow {
          requested: needed,
          max: K_MAXIMAL_BUFFER_SIZE,
        });
      }
      while self.buffer.capacity().saturating_sub(self.buffer.len()) < space {
        self.buffer.grow(BufferGrowthStrategy::Double)?;
      }
    }
    Ok(())
  }

  /// 输出 1 字节
  pub fn emit_u8(&mut self, val: u8) -> Result<()> {
    self.ensure_space(1)?;
    self.buffer.push(val);
    Ok(())
  }

  /// 输出小端 2 字节
  pub fn emit_u16(&mut self, val: u16) -> Result<()> {
    self.ensure_space(2)?;
    self.buffer.extend_from_slice(&val.to_le_bytes());
    Ok(())
  }

  /// 输出小端 4 字节
  pub fn emit_u32(&mut self, val: u32) -> Result<()> {
    self.ensure_space(4)?;
    self.buffer.extend_from_slice(&val.to_le_bytes());
    Ok(())
  }

  /// 输出小端 8 字节
  pub fn emit_u64(&mut self, val: u64) -> Result<()> {
    self.ensure_space(8)?;
    self.buffer.extend_from_slice(&val.to_le_bytes());
    Ok(())
  }

  /// 输出字节切片
  pub fn emit_bytes(&mut self, bytes: &[u8]) -> Result<()> {
    self.ensure_space(bytes.len())?;
    self.buffer.extend_from_slice(bytes);
    Ok(())
  }

  #[inline]
  pub fn buffer_slice(&self) -> &[u8] {
    self.buffer.as_slice()
  }

  /// v8: assembler-inl.h AssemblerBase::buffer_start() — 缓冲首地址
  /// (仅取地址值使用; 经此指针写回缓冲即违反借用契约, 调用方只读)
  #[inline]
  #[must_use]
  pub fn buffer_start(&self) -> *mut u8 {
    self.buffer.as_slice().as_ptr() as *mut u8
  }

  /// v8: assembler-inl.h AssemblerBase::buffer_size() — 缓冲总容量
  #[inline]
  #[must_use]
  pub fn buffer_capacity(&self) -> usize {
    self.buffer.capacity()
  }

  /// v8: assembler-inl.h AssemblerBase::instruction_size() — 已发射字节数
  #[inline]
  #[must_use]
  pub fn instruction_size(&self) -> usize {
    self.pc_offset()
  }

  #[inline]
  pub fn take_buffer(self) -> Vec<u8> {
    self.buffer.into_inner()
  }

  #[inline]
  pub fn set_byte_at(&mut self, pos: usize, val: u8) {
    self.buffer.set_byte_at(pos, val);
  }

  #[inline]
  pub fn write_bytes_at(&mut self, pos: usize, bytes: &[u8]) {
    self.buffer.write_bytes_at(pos, bytes);
  }

  #[inline]
  pub fn byte_at(&self, pos: usize) -> u8 {
    self.buffer.byte_at(pos)
  }

  #[inline]
  pub fn read_i32_at(&self, pos: usize) -> i32 {
    i32::from_le_bytes(self.buffer.read_bytes_at::<4>(pos))
  }

  #[inline]
  pub fn write_i32_at(&mut self, pos: usize, val: i32) {
    self.buffer.write_bytes_at(pos, &val.to_le_bytes());
  }
}
