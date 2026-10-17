// 由 tool/src/wasm/local-decl-encoder.h.js 从 v8/src/wasm/local-decl-encoder.{h,cc} 生成; 修订就地改, 复跑覆盖
// 局部声明编码器: Emit/AddLocals/Size 1:1 转写

use crate::{
  leb_helper::{LebHelper, MAX_VAR_INT32_SIZE},
  value_type::{ValueType, ValueTypeBits},
  wasm_constants::{K_SHARED_FLAG_CODE, ValueTypeCode},
  wasm_opcodes::SigDef,
};

/// 局部声明: (数量, 类型)
pub type LocalDecl = (u32, ValueType);

/// 局部声明编码器 (对应 class LocalDeclEncoder)
pub struct LocalDeclEncoder {
  /// 函数签名 (const FunctionSig* sig_)
  sig_: Option<SigDef>,
  /// ZoneVector<std::pair<uint32_t, ValueType>> local_decls_
  local_decls_: Vec<LocalDecl>,
  /// 局部总数 size_t total_
  total_: usize,
}

/// 把 LEB 写入结果拷进目标缓冲, 返回写入字节数 (write_u32v/write_i32v 共用暂存)
fn write_leb(dest: &mut [u8], pos: usize, bytes: &[u8]) -> usize {
  dest[pos..pos + bytes.len()].copy_from_slice(bytes);
  bytes.len()
}

impl LocalDeclEncoder {
  /// 对应 LocalDeclEncoder(zone, sig = nullptr)
  #[must_use]
  pub const fn new() -> Self {
    Self {
      sig_: None,
      local_decls_: Vec::new(),
      total_: 0,
    }
  }

  /// 对应 Emit(uint8_t* buffer) -> size_t; 编码局部声明前缀
  /// v8: src/wasm/local-decl-encoder.cc LocalDeclEncoder::Emit
  pub fn emit(&self, buffer: &mut [u8]) -> usize {
    let mut pos = 0usize;
    let mut scratch = [0u8; MAX_VAR_INT32_SIZE];
    let n = LebHelper::write_u32v(&mut scratch, self.local_decls_.len() as u32);
    pos += write_leb(buffer, pos, &scratch[..n]);
    for &(count, ty) in &self.local_decls_ {
      let n = LebHelper::write_u32v(&mut scratch, count);
      pos += write_leb(buffer, pos, &scratch[..n]);
      buffer[pos] = ty.value_type_code() as u8;
      pos += 1;
      if ty.encoding_needs_shared() {
        buffer[pos] = K_SHARED_FLAG_CODE;
        pos += 1;
      }
      if ty.encoding_needs_exact() {
        buffer[pos] = ValueTypeCode::Exact as u8;
        pos += 1;
      }
      if ty.encoding_needs_heap_type() {
        let n = LebHelper::write_i32v(&mut scratch, ty.heap_type().code());
        pos += write_leb(buffer, pos, &scratch[..n]);
      }
    }
    debug_assert_eq!(self.size(), pos);
    pos
  }

  /// 对应 AddLocals(count, type) -> uint32_t; 返回新局部起始索引
  /// v8: src/wasm/local-decl-encoder.cc LocalDeclEncoder::AddLocals
  pub fn add_locals(&mut self, mut count: u32, type_: ValueType) -> u32 {
    let params = self.sig_.as_ref().map_or(0, |s| s.params.len());
    let result = (self.total_ + params) as u32;
    self.total_ += count as usize;
    if let Some(&(last_count, last_type)) = self.local_decls_.last()
      && last_type == type_
    {
      count += last_count;
      self.local_decls_.pop();
    }
    self.local_decls_.push((count, type_));
    result
  }

  /// 对应 Size() -> size_t (locals 计数字节 + 各 (reps 字节 + Opcode + 修饰位))
  /// v8: src/wasm/local-decl-encoder.cc LocalDeclEncoder::Size
  /// 注: heap_type() 仅在 needs_heap_type 时求值, 与原版三元运算符的短路一致 (数值类型 bits&0b11==0, 调 heap_type 触发 debug_assert)
  #[must_use]
  pub fn size(&self) -> usize {
    let mut size = LebHelper::sizeof_u32v(self.local_decls_.len() as u32);
    for &(count, ty) in &self.local_decls_ {
      size += LebHelper::sizeof_u32v(count) + 1;
      if ty.encoding_needs_shared() {
        size += 1;
      }
      if ty.encoding_needs_exact() {
        size += 1;
      }
      if ty.encoding_needs_heap_type() {
        size += LebHelper::sizeof_i32v(ty.heap_type().code());
      }
    }
    size
  }

  /// 对应 has_sig()
  #[must_use]
  pub const fn has_sig(&self) -> bool {
    self.sig_.is_some()
  }

  /// 对应 set_sig(s)
  pub const fn set_sig(&mut self, s: SigDef) {
    self.sig_ = Some(s);
  }
}

impl Default for LocalDeclEncoder {
  fn default() -> Self {
    Self::new()
  }
}
