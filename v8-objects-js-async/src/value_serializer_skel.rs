// 由 tool/src/objects/value-serializer-skel.h.js 从 v8/src/objects/value-serializer.h (+ value-serializer.cc 版本常量补面) 生成; 修订就地改, 复跑覆盖
// 席位表差集: 版本常量 + 序列化器/反序列化器类面骨架 (SerializationTag 归既有 value-serializer.h.js)
//! 结构化克隆 (structured clone) 序列化器类面骨架。
//!
//! SerializationTag 线上标签全量归既有 value-serializer.h.js 产物
//! (v8-objects-rest 侧尚未挂载 lib, 此处仅落 kVersion/kPadding 两枚
//! 头部原语所需常量, 收敛待台账迁移项承接);
//! 本文件补: 线上版本常量 (kLatestVersion=16, 不可回退) 与
//! ValueSerializer/ValueDeserializer 缓冲区级线上原语 1:1 语义体。
//!
//! 十.3 红线: 序列化字节级语义自研转写, 禁 serde/sonic 类库替换。

use std::{
  cmp::max,
  mem::{size_of, size_of_val, take},
  ops::{BitOrAssign, Shl},
};

use crate::error::{Error, Result};

/// 线上格式版本 (原版 kLatestVersion; 每次格式变更单调递增, 不可复用旧值)。
pub const K_LATEST_VERSION: u32 = 16;

/// SerializationTag::kVersion = 0xFF (value-serializer.h; 头部版本 tag)。
pub const K_VERSION_TAG: u8 = 0xFF;
/// SerializationTag::kPadding = 0x00 (读侧跳填充)。
pub const K_PADDING_TAG: u8 = 0x00;

// ── ValueSerializer 缓冲面 (value-serializer.cc 字节原语 1:1) ──

/// 原版 ValueSerializer 的缓冲区字段面 (buffer_/buffer_size_/buffer_capacity_/
/// out_of_memory_/treat_array_buffer_views_as_host_objects_);
/// delegate_ ReallocateBufferMemory 走 Rust Vec 增长面 (base::Realloc 等价)。
pub struct ValueSerializerBuf {
  /// 原版 uint8_t* buffer_ + buffer_size_ (已写字节)。
  buffer: Vec<u8>,
  /// 原版 bool out_of_memory_。
  out_of_memory: bool,
  /// 原版 bool treat_array_buffer_views_as_host_objects_。
  treat_array_buffer_views_as_host_objects: bool,
}

impl ValueSerializerBuf {
  /// 原版 ctor 初始化面 (buffer_ = nullptr, buffer_size_ = 0, out_of_memory_ = false)。
  #[must_use]
  pub fn new() -> Self {
    Self {
      buffer: Vec::new(),
      out_of_memory: false,
      treat_array_buffer_views_as_host_objects: false,
    }
  }

  /// 原版 set_treat_array_buffer_views_as_host_objects (cc: SetTreatArrayBufferViewsAsHostObjects)。
  pub fn set_treat_array_buffer_views_as_host_objects(&mut self, mode: bool) {
    self.treat_array_buffer_views_as_host_objects = mode;
  }

  /// 原版 WriteHeader (cc: 写 varint 版本 kLatestVersion)。
  ///
  /// v8: src/objects/value-serializer.cc ValueSerializer::WriteHeader
  pub fn write_header(&mut self) {
    self.write_tag(K_VERSION_TAG);
    self.write_varint_u32(K_LATEST_VERSION);
  }

  /// 原版 WriteTag (cc: 单字节 tag 落流)。
  ///
  /// v8: src/objects/value-serializer.cc ValueSerializer::WriteTag
  pub fn write_tag(&mut self, tag: u8) {
    self.write_raw_bytes(&[tag]);
  }

  /// 原版 WriteVarint<T=uint32_t> (cc: base-128, 7 位一组, 末组清 MSB)。
  ///
  /// v8: src/objects/value-serializer.cc ValueSerializer::WriteVarint<uint32_t>
  pub fn write_varint_u32(&mut self, value: u32) {
    self.write_varint_impl(value as u64);
  }

  /// 原版 WriteVarint<T=uint64_t>。
  ///
  /// v8: src/objects/value-serializer.cc ValueSerializer::WriteVarint<uint64_t>
  pub fn write_varint_u64(&mut self, value: u64) {
    self.write_varint_impl(value);
  }

  /// 公共 varint 落码 (do/while 与原版 stack_buffer 逐字节等价)。
  ///
  /// v8: src/objects/value-serializer.cc ValueSerializer::WriteVarint<T>
  /// 原版 stack_buffer 尺寸 `sizeof(T) * 8 / 7 + 1` (u32→5 / u64→10);
  /// do/while 至少写一字节, 末字节清 MSB 后 WriteRawBytes。u32 经零位扩展走
  /// u64 通道与原版逐字节等价 (varint 只按有效位落码, 前导零不参与编码)。
  fn write_varint_impl(&mut self, mut value: u64) {
    // sizeof(u64) * 8 / 7 + 1 = 10, 覆盖 u32(≤5) / u64(≤10) 上界。
    const MAX_VARINT_BYTES: usize = u64::BITS as usize / 7 + 1;
    let mut stack_buffer = [0u8; MAX_VARINT_BYTES];
    let mut next_byte = 0usize;
    loop {
      // *next_byte = (value & 0x7F) | 0x80; next_byte++;
      stack_buffer[next_byte] = (value & 0x7F) as u8 | 0x80;
      next_byte += 1;
      // value >>= 7;
      value >>= 7;
      // while (value);
      if value == 0 {
        break;
      }
    }
    // *(next_byte - 1) &= 0x7F;
    stack_buffer[next_byte - 1] &= 0x7F;
    self.write_raw_bytes(&stack_buffer[..next_byte]);
  }

  /// 原版 WriteZigZag<int32_t> (cc: 符号位翻转后 varint)。
  ///
  /// v8: src/objects/value-serializer.cc ValueSerializer::WriteZigZag
  pub fn write_zigzag(&mut self, value: i32) {
    let encoded = ((value as u32) << 1) ^ (value >> 31) as u32;
    self.write_varint_u32(encoded);
  }

  /// 原版 WriteDouble (cc: 主机字节序 8 字节直落)。
  ///
  /// v8: src/objects/value-serializer.cc ValueSerializer::WriteDouble
  pub fn write_double(&mut self, value: f64) {
    self.write_raw_bytes(&value.to_ne_bytes());
  }

  /// 原版 WriteOneByteString (cc: varint(len) + 原始字节)。
  ///
  /// v8: src/objects/value-serializer.cc ValueSerializer::WriteOneByteString
  pub fn write_one_byte_string(&mut self, chars: &[u8]) {
    self.write_varint_u32(chars.len() as u32);
    self.write_raw_bytes(chars);
  }

  /// 原版 WriteTwoByteString (cc: varint(len*2) + 原始 16 位字节面, 主机序)。
  ///
  /// v8: src/objects/value-serializer.cc ValueSerializer::WriteTwoByteString
  pub fn write_two_byte_string(&mut self, chars: &[u16]) {
    self.write_varint_u32(size_of_val(chars) as u32);
    for c in chars {
      self.write_raw_bytes(&c.to_ne_bytes());
    }
  }

  /// 原版 WriteByte (cc)。
  ///
  /// v8: src/objects/value-serializer.cc ValueSerializer::WriteByte
  pub fn write_byte(&mut self, value: u8) {
    if self.reserve_raw_bytes(1).is_some() {
      self.buffer.push(value);
    }
  }

  /// 原版 WriteUint32 (cc: varint)。
  ///
  /// v8: src/objects/value-serializer.cc ValueSerializer::WriteUint32
  pub fn write_uint32(&mut self, value: u32) {
    self.write_varint_u32(value);
  }

  /// 原版 WriteUint64 (cc: varint)。
  ///
  /// v8: src/objects/value-serializer.cc ValueSerializer::WriteUint64
  pub fn write_uint64(&mut self, value: u64) {
    self.write_varint_u64(value);
  }

  /// 原版 WriteRawBytes (cc: reserve + memcpy)。
  ///
  /// v8: src/objects/value-serializer.cc ValueSerializer::WriteRawBytes
  pub fn write_raw_bytes(&mut self, source: &[u8]) {
    let length = source.len();
    if self.reserve_raw_bytes(length).is_some() && length > 0 {
      self.buffer.extend_from_slice(source);
    }
  }

  /// 原版 ReserveRawBytes (cc: 越容先扩, 返回写入起点)。
  ///
  /// v8: src/objects/value-serializer.cc ValueSerializer::ReserveRawBytes
  fn reserve_raw_bytes(&mut self, bytes: usize) -> Option<usize> {
    let old_size = self.buffer.len();
    let new_size = old_size + bytes;
    if new_size > self.buffer.capacity() && !self.expand_buffer(new_size) {
      return None;
    }
    Some(old_size)
  }

  /// 原版 ExpandBuffer (私有: 容量倍增)。
  ///
  /// v8: src/objects/value-serializer.cc ValueSerializer::ExpandBuffer
  /// 增长公式 1:1: requested = max(required, capacity*2) + 64;
  /// 分配失败 → out_of_memory_ 落真、返 false (原版 Nothing)。
  pub fn expand_buffer(&mut self, required_capacity: usize) -> bool {
    debug_assert!(required_capacity > self.buffer.capacity());
    let requested_capacity = max(required_capacity, self.buffer.capacity() * 2) + 64;
    if self
      .buffer
      .try_reserve(requested_capacity - self.buffer.len())
      .is_err()
    {
      self.out_of_memory = true;
      return false;
    }
    true
  }

  /// 原版 Release (cc: 交还缓冲区所有权)。
  ///
  /// v8: src/objects/value-serializer.cc ValueSerializer::Release
  /// 返回 (buffer, size) = 原版 pair<uint8_t*, size_t>; 交还后三字段清零。
  pub fn release(&mut self) -> Vec<u8> {
    let size = self.buffer.len();
    let mut result = take(&mut self.buffer);
    result.truncate(size);
    result.shrink_to_fit();
    result
  }

  /// out_of_memory_ 读面 (原版 IsOutOfMemory/ThrowIfOutOfMemory 判定入口)。
  #[must_use]
  pub fn is_out_of_memory(&self) -> bool {
    self.out_of_memory
  }

  /// v8: src/objects/value-serializer.cc ValueSerializer::WriteObject。
  pub fn write_object(&mut self, _object: ()) {
    todo!(
      "ValueSerializer::WriteObject: 依赖 Isolate + Handle<JSObject> + SerializationTag 分派 + 循环引用表; 解锁 v8-execution-isolate + v8-handles + v8-objects-js-core"
    )
  }
}

impl Default for ValueSerializerBuf {
  fn default() -> Self {
    Self::new()
  }
}

// ── ValueDeserializer 缓冲面 (value-serializer.cc 读原语 1:1) ──

/// 原版 ValueDeserializer 的读游标面 (next_/end_ → bytes+position, version_)。
pub struct ValueDeserializerBuf<'a> {
  /// 原版 next_..end_ 区间。
  bytes: &'a [u8],
  /// 原版 next_ 游标。
  position: usize,
  /// 原版 uint32_t version_ (ReadHeader 落值)。
  version: u32,
}

impl<'a> ValueDeserializerBuf<'a> {
  /// 原版 ctor (buffer 挂接, version_ = kLatestVersion? — 原构造 version_(0)?
  /// 采用原版字段初始值)。
  #[must_use]
  pub fn new(bytes: &'a [u8]) -> Self {
    Self {
      bytes,
      position: 0,
      version: 0,
    }
  }

  /// 原版 version() (WriteHeader 落值读出)。
  #[must_use]
  pub fn version(&self) -> u32 {
    self.version
  }

  /// 原版 ReadHeader (cc: 读版本并校验 <= kLatestVersion)。
  ///
  /// v8: src/objects/value-serializer.cc ValueDeserializer::ReadHeader
  /// 头字节为 kVersion 时消费之并 ReadVarintLoop 落版本号;
  /// 读取失败或高于 kLatestVersion → DataClone 版本错误 (原版 isolate Throw)。
  pub fn read_header(&mut self) -> Result<()> {
    if self.position < self.bytes.len() && self.bytes[self.position] == K_VERSION_TAG {
      self.read_tag().ok_or(Error::DataCloneVersionError)?;
      let Some(version) = self.read_varint_loop_u32() else {
        return Err(Error::DataCloneVersionError);
      };
      self.version = version;
      if version > K_LATEST_VERSION {
        return Err(Error::DataCloneVersionError);
      }
    }
    Ok(())
  }

  /// 原版 PeekTag (cc: 前瞻、跳 kPadding, 不移游标)。
  ///
  /// v8: src/objects/value-serializer.cc ValueDeserializer::PeekTag
  #[must_use]
  pub fn peek_tag(&self) -> Option<u8> {
    let mut peek_position = self.position;
    loop {
      if peek_position >= self.bytes.len() {
        return None;
      }
      let tag = self.bytes[peek_position];
      peek_position += 1;
      if tag != K_PADDING_TAG {
        return Some(tag);
      }
    }
  }

  /// 原版 ReadTag (cc: 消费并跳 kPadding)。
  ///
  /// v8: src/objects/value-serializer.cc ValueDeserializer::ReadTag
  pub fn read_tag(&mut self) -> Option<u8> {
    loop {
      if self.position >= self.bytes.len() {
        return None;
      }
      let tag = self.bytes[self.position];
      self.position += 1;
      if tag != K_PADDING_TAG {
        return Some(tag);
      }
    }
  }

  /// 原版 ConsumeTag (cc: 消费并与前瞻值 DCHECK 对齐)。
  ///
  /// v8: src/objects/value-serializer.cc ValueDeserializer::ConsumeTag
  pub fn consume_tag(&mut self, peeked_tag: u8) {
    if let Some(actual_tag) = self.read_tag() {
      debug_assert_eq!(actual_tag, peeked_tag);
    } else {
      debug_assert!(false, "原版 ToChecked: peek 后必有 tag");
    }
  }

  /// 原版 ReadVarint<uint32_t> (cc: 手工展开快路径, sizeof<=4)。
  ///
  /// v8: src/objects/value-serializer.cc ValueDeserializer::ReadVarint
  pub fn read_varint_u32(&mut self) -> Option<u32> {
    if self.position + (size_of::<u32>() + 1) >= self.bytes.len() {
      return self.read_varint_loop_u32();
    }
    #[cfg(debug_assertions)]
    let (previous_position, expected_value, expected_position) = {
      let previous_position = self.position;
      let expected_value = self.read_varint_loop_u32().unwrap_or(0);
      let expected_position = self.position;
      self.position = previous_position;
      (previous_position, expected_value, expected_position)
    };
    let mut value: u32 = 0;
    macro_rules! iteration_shifted {
      ($shift:expr) => {
        if $shift < std::mem::size_of::<u32>() * 8 {
          let byte = self.bytes[self.position];
          self.position += 1;
          if byte < 0x80 {
            value |= (byte as u32) << $shift;
            #[cfg(debug_assertions)]
            {
              debug_assert!(self.position <= self.bytes.len());
              debug_assert_eq!(self.position, expected_position);
              debug_assert_eq!(value, expected_value);
            }
            return Some(value);
          } else {
            value |= ((byte & 0x7F) as u32) << $shift;
          }
        }
      };
    }
    // Manually unroll the loop to achieve the best measured performance.
    iteration_shifted!(0);
    iteration_shifted!(7);
    iteration_shifted!(14);
    iteration_shifted!(21);
    iteration_shifted!(28);
    #[cfg(debug_assertions)]
    {
      debug_assert!(self.position <= self.bytes.len());
      debug_assert_eq!(self.position, expected_position);
      debug_assert_eq!(value, expected_value);
      let _ = previous_position;
    }
    Some(value)
  }

  /// 原版 ReadVarint<uint64_t> (cc: sizeof>4 直落 ReadVarintLoop)。
  ///
  /// v8: src/objects/value-serializer.cc ValueDeserializer::ReadVarint<uint64_t>
  pub fn read_varint_u64(&mut self) -> Option<u64> {
    self.read_varint_loop_u64()
  }

  /// 原版 ReadVarintLoop<uint32_t> (cc 回退环)。
  ///
  /// v8: src/objects/value-serializer.cc ValueDeserializer::ReadVarintLoop
  pub fn read_varint_loop_u32(&mut self) -> Option<u32> {
    self.read_varint_loop::<u32>()
  }

  /// 原版 ReadVarintLoop<uint64_t>。
  ///
  /// v8: src/objects/value-serializer.cc ValueDeserializer::ReadVarintLoop
  pub fn read_varint_loop_u64(&mut self) -> Option<u64> {
    self.read_varint_loop::<u64>()
  }

  /// 公共回退环 (逐字节 7 位累计; 超过 sizeof(T)*8 后与原版一致:
  /// 立即返回当前值且不再推进游标 — 原版 hard-fail CHECK 分支目标配置关)。
  fn read_varint_loop<T>(&mut self) -> Option<T>
  where
    T: Copy + Default + BitOrAssign + Shl<u32, Output = T> + From<u32> + Into<u64>,
  {
    let mut value = T::default();
    let mut shift = 0u32;
    loop {
      if self.position >= self.bytes.len() {
        return None;
      }
      let byte = self.bytes[self.position];
      let has_another_byte = byte & 0x80;
      if shift < (size_of::<T>() as u32) * 8 {
        value |= <T as From<u32>>::from((byte & 0x7F) as u32) << shift;
        shift += 7;
      } else {
        // For consistency with the fast unrolled loop: 读满 sizeof(T)+1 字节即返回。
        return Some(value);
      }
      self.position += 1;
      if has_another_byte == 0 {
        break;
      }
    }
    let _ = <T as Into<u64>>::into(value);
    Some(value)
  }

  /// 原版 ReadZigZag<int32_t> (cc: varint 后 zigzag 解码)。
  ///
  /// v8: src/objects/value-serializer.cc ValueDeserializer::ReadZigZag
  pub fn read_zigzag(&mut self) -> Option<i32> {
    let unsigned_value = self.read_varint_u32()?;
    Some(((unsigned_value >> 1) as i32) ^ -((unsigned_value & 1) as i32))
  }

  /// 原版 ReadDouble (cc: 8 字节 memcpy + NaN 规范化为 quiet NaN, 主机序)。
  ///
  /// v8: src/objects/value-serializer.cc ValueDeserializer::ReadDouble
  pub fn read_double(&mut self) -> Option<f64> {
    if size_of::<f64>() > self.bytes.len() - self.position {
      return None;
    }
    let mut bits = [0u8; 8];
    bits.copy_from_slice(&self.bytes[self.position..self.position + 8]);
    self.position += 8;
    let mut value = f64::from_ne_bytes(bits);
    if value.is_nan() {
      // std::numeric_limits<double>::quiet_NaN() 规范化。
      value = f64::NAN;
    }
    Some(value)
  }

  /// 原版 ReadRawBytes (cc: 界检后游标前移)。
  ///
  /// v8: src/objects/value-serializer.cc ValueDeserializer::ReadRawBytes(size)
  #[must_use]
  pub fn read_raw_bytes(&mut self, size: usize) -> Option<&'a [u8]> {
    if size > self.bytes.len() - self.position {
      return None;
    }
    let start = self.position;
    self.position += size;
    Some(&self.bytes[start..start + size])
  }

  /// 原版 ReadRawTwoBytes (cc: 2 字节对齐 + 界检 + size 偶数校验)。
  ///
  /// v8: src/objects/value-serializer.cc ValueDeserializer::ReadRawTwoBytes
  pub fn read_raw_two_bytes(&mut self, size: usize) -> Option<&'a [u8]> {
    let position_addr = self.bytes.as_ptr() as usize + self.position;
    if !position_addr.is_multiple_of(2)
      || size > self.bytes.len() - self.position
      || !size.is_multiple_of(size_of::<u16>())
    {
      return None;
    }
    let start = self.position;
    self.position += size;
    Some(&self.bytes[start..start + size])
  }

  /// 原版 ReadByte (cc: 界检单字节)。
  ///
  /// v8: src/objects/value-serializer.cc ValueDeserializer::ReadByte
  pub fn read_byte(&mut self) -> Option<u8> {
    if self.bytes.len() - self.position < size_of::<u8>() {
      return None;
    }
    let value = self.bytes[self.position];
    self.position += 1;
    Some(value)
  }

  /// 原版 ReadUint32 (cc: varint)。
  ///
  /// v8: src/objects/value-serializer.cc ValueDeserializer::ReadUint32
  pub fn read_uint32(&mut self) -> Option<u32> {
    self.read_varint_u32()
  }

  /// 原版 ReadUint64 (cc: varint)。
  ///
  /// v8: src/objects/value-serializer.cc ValueDeserializer::ReadUint64
  pub fn read_uint64(&mut self) -> Option<u64> {
    self.read_varint_u64()
  }

  /// 原版 ReadSizeT (cc: uint64 varint + 上界校验)。
  ///
  /// v8: src/objects/value-serializer.cc ValueDeserializer::ReadSizeT
  pub fn read_size_t(&mut self) -> Option<usize> {
    let value64 = self.read_varint_loop_u64()?;
    if value64 > usize::MAX as u64 {
      return None;
    }
    Some(value64 as usize)
  }

  /// v8: src/objects/value-serializer.cc ValueDeserializer::ReadObject。
  pub fn read_object(&mut self, _isolate: ()) {
    todo!(
      "ValueDeserializer::ReadObject: 依赖 Isolate + Handle<Object> + tag 分派 + back-ref 表; 解锁 v8-execution-isolate + v8-handles + v8-objects-js-core"
    )
  }

  /// v8: src/objects/value-serializer.cc ValueDeserializer::TransferArrayBuffer。
  pub fn transfer_array_buffer(&mut self, _transfer_id: u32, _array_buffer: ()) {
    todo!(
      "ValueDeserializer::TransferArrayBuffer: 依赖 JSArrayBuffer + Handle + 带外转移 id 表; 解锁 v8-objects-js-core + v8-handles"
    )
  }

  /// v8: src/objects/value-serializer.cc ValueDeserializer::ReadJSArrayBuffer。
  pub fn read_js_array_buffer(&mut self, _array_buffer: *mut (), _id: u32) {
    todo!(
      "ValueDeserializer::ReadJSArrayBuffer: 依赖 JSArrayBuffer 句柄槽 + BackingStore + Handle; 解锁 v8-objects-js-core + v8-handles + v8-execution-isolate"
    )
  }

  /// v8: src/objects/value-serializer.cc ValueDeserializer::ReadJSArrayBufferView。
  pub fn read_js_array_buffer_view(&mut self, _view: ()) {
    todo!(
      "ValueDeserializer::ReadJSArrayBufferView: 依赖 JSArrayBufferView 槽位恢复 + Handle; 解锁 v8-objects-js-core + v8-handles"
    )
  }
}

// ── 提取不变式断言 ──
const _: () = {
  assert!(K_LATEST_VERSION == 16);
  assert!(K_LATEST_VERSION > 0);
  assert!(K_VERSION_TAG == 0xFF);
  assert!(K_PADDING_TAG == 0x00);
};
