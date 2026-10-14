use crate::{
  constants::DEFAULT_STACK_SIZE,
  error::{Error, Result},
};

/// 模拟器内存空间 (包含模拟栈与动态映射区, 100% Safe Rust)
#[derive(Debug)]
pub struct SimulatorMemory {
  /// 模拟栈基址 (栈底, 高地址)
  stack_base: u64,
  /// 模拟栈内存缓冲区
  stack_data: Vec<u8>,
  /// 代码区基址与内容
  code_base: u64,
  code_data: Vec<u8>,
}

impl Default for SimulatorMemory {
  fn default() -> Self {
    Self::new()
  }
}

impl SimulatorMemory {
  /// 创建标准 1MB 模拟栈空间的内存管理器
  pub fn new() -> Self {
    let stack_base = 0x0000_7FFF_FFFF_0000;
    let stack_data = vec![0u8; DEFAULT_STACK_SIZE];

    Self {
      stack_base,
      stack_data,
      code_base: 0x0000_0001_0000_0000,
      code_data: Vec::new(),
    }
  }

  /// 获取初始栈指针位置 (16 字节对齐)
  pub const fn initial_sp(&self) -> u64 {
    self.stack_base
  }

  /// 获取当前栈大小
  #[inline]
  pub fn stack_size(&self) -> usize {
    self.stack_data.len()
  }

  /// 加载可执行指令切片到代码区 (支持增量多函数加载)
  pub fn load_code(&mut self, base: u64, instructions: &[u8]) {
    if self.code_data.is_empty() {
      self.code_base = base;
      self.code_data = instructions.to_vec();
      return;
    }

    if base < self.code_base {
      let shift = (self.code_base - base) as usize;
      let mut new_data = vec![0u8; shift + self.code_data.len()];
      new_data[shift..].copy_from_slice(&self.code_data);
      self.code_base = base;
      self.code_data = new_data;
    }

    let offset = (base - self.code_base) as usize;
    let end = offset + instructions.len();
    if end > self.code_data.len() {
      self.code_data.resize(end, 0);
    }
    self.code_data[offset..end].copy_from_slice(instructions);
  }

  /// 校验内存地址或栈指针对齐 (例如 16 字节对齐)
  #[inline]
  pub fn check_alignment(&self, address: u64, alignment: usize) -> Result<()> {
    if alignment > 0 && !address.is_multiple_of(alignment as u64) {
      Err(Error::UnalignedAccess { address, alignment })
    } else {
      Ok(())
    }
  }

  /// 获取只读内存切片
  fn get_slice(&self, address: u64, size: usize) -> Result<&[u8]> {
    let stack_top = self.stack_base - self.stack_data.len() as u64;
    if address >= stack_top && address.saturating_add(size as u64) <= self.stack_base {
      let offset = (address - stack_top) as usize;
      if let Some(slice) = self.stack_data.get(offset..offset + size) {
        return Ok(slice);
      }
    }

    let code_end = self.code_base + self.code_data.len() as u64;
    if address >= self.code_base && address.saturating_add(size as u64) <= code_end {
      let offset = (address - self.code_base) as usize;
      if let Some(slice) = self.code_data.get(offset..offset + size) {
        return Ok(slice);
      }
    }

    Err(Error::MemoryOutOfBounds { address, size })
  }

  /// 获取可写内存切片
  fn get_mut_slice(&mut self, address: u64, size: usize) -> Result<&mut [u8]> {
    let stack_top = self.stack_base - self.stack_data.len() as u64;
    if address >= stack_top && address.saturating_add(size as u64) <= self.stack_base {
      let offset = (address - stack_top) as usize;
      if let Some(slice) = self.stack_data.get_mut(offset..offset + size) {
        return Ok(slice);
      }
    }

    let code_end = self.code_base + self.code_data.len() as u64;
    if address >= self.code_base && address.saturating_add(size as u64) <= code_end {
      let offset = (address - self.code_base) as usize;
      if let Some(slice) = self.code_data.get_mut(offset..offset + size) {
        return Ok(slice);
      }
    }

    Err(Error::MemoryOutOfBounds { address, size })
  }

  /// 读取 8 位无符号整数
  pub fn read_u8(&self, address: u64) -> Result<u8> {
    let slice = self.get_slice(address, 1)?;
    Ok(slice[0])
  }

  /// 读取 16 位小端无符号整数
  pub fn read_u16(&self, address: u64) -> Result<u16> {
    let slice = self.get_slice(address, 2)?;
    let mut bytes = [0u8; 2];
    bytes.copy_from_slice(slice);
    Ok(u16::from_le_bytes(bytes))
  }

  /// 读取 32 位小端无符号整数 (指令读取常用)
  pub fn read_u32(&self, address: u64) -> Result<u32> {
    let slice = self.get_slice(address, 4)?;
    let mut bytes = [0u8; 4];
    bytes.copy_from_slice(slice);
    Ok(u32::from_le_bytes(bytes))
  }

  /// 读取 64 位小端无符号整数 (指针/数值读取常用)
  pub fn read_u64(&self, address: u64) -> Result<u64> {
    let slice = self.get_slice(address, 8)?;
    let mut bytes = [0u8; 8];
    bytes.copy_from_slice(slice);
    Ok(u64::from_le_bytes(bytes))
  }

  /// 写入 8 位无符号整数
  pub fn write_u8(&mut self, address: u64, val: u8) -> Result<()> {
    let slice = self.get_mut_slice(address, 1)?;
    slice[0] = val;
    Ok(())
  }

  /// 写入 16 位小端无符号整数
  pub fn write_u16(&mut self, address: u64, val: u16) -> Result<()> {
    let slice = self.get_mut_slice(address, 2)?;
    slice.copy_from_slice(&val.to_le_bytes());
    Ok(())
  }

  /// 写入 32 位小端无符号整数
  pub fn write_u32(&mut self, address: u64, val: u32) -> Result<()> {
    let slice = self.get_mut_slice(address, 4)?;
    slice.copy_from_slice(&val.to_le_bytes());
    Ok(())
  }

  /// 写入 64 位小端无符号整数
  pub fn write_u64(&mut self, address: u64, val: u64) -> Result<()> {
    let slice = self.get_mut_slice(address, 8)?;
    slice.copy_from_slice(&val.to_le_bytes());
    Ok(())
  }
}
