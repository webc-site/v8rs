// 由 tool/src/snapshot/snapshot-source-sink.js 从 v8/src/snapshot/snapshot-source-sink.{h,cc} 生成; 修订就地改, 复跑覆盖
// 快照字节源/字节汇

use log::trace;

/// v8: src/snapshot/snapshot-source-sink.h SnapshotByteSource
/// 快照/builtin 文件读取源 (内存所有权归调用方, 借用切片)。
#[derive(Debug, Clone, Copy)]
pub struct SnapshotByteSource<'a> {
  data: &'a [u8],
  length: usize,
  position: usize,
}

impl<'a> SnapshotByteSource<'a> {
  /// v8: src/snapshot/snapshot-source-sink.h SnapshotByteSource::SnapshotByteSource
  #[inline]
  #[must_use]
  pub const fn new(payload: &'a [u8]) -> Self {
    Self {
      data: payload,
      length: payload.len(),
      position: 0,
    }
  }

  /// v8: src/snapshot/snapshot-source-sink.h SnapshotByteSource::Reset
  #[inline]
  pub fn reset(&mut self, payload: &'a [u8]) {
    self.data = payload;
    self.length = payload.len();
    self.position = 0;
  }

  /// v8: src/snapshot/snapshot-source-sink.h SnapshotByteSource::HasMore
  #[inline]
  #[must_use]
  pub const fn has_more(&self) -> bool {
    self.position < self.length
  }

  /// v8: src/snapshot/snapshot-source-sink.h SnapshotByteSource::Get
  /// 越界为程序契约错误 (原版 DCHECK), 返回已确认存在的字节。
  #[inline]
  #[must_use]
  pub fn get(&mut self) -> u8 {
    debug_assert!(self.position < self.length, "SnapshotByteSource::Get 越界");
    let b = self.data[self.position];
    self.position += 1;
    b
  }

  /// v8: src/snapshot/snapshot-source-sink.h SnapshotByteSource::Peek
  #[inline]
  #[must_use]
  pub fn peek(&self) -> u8 {
    debug_assert!(self.position < self.length, "SnapshotByteSource::Peek 越界");
    self.data[self.position]
  }

  /// v8: src/snapshot/snapshot-source-sink.h SnapshotByteSource::Advance
  #[inline]
  pub fn advance(&mut self, by: usize) {
    self.position += by;
  }

  /// v8: src/snapshot/snapshot-source-sink.h SnapshotByteSource::CopyRaw
  /// 把 `to` 长度对应的原始字节拷入, 游标前移。
  pub fn copy_raw(&mut self, to: &mut [u8]) {
    debug_assert!(self.position + to.len() <= self.length, "CopyRaw 越界");
    to.copy_from_slice(&self.data[self.position..self.position + to.len()]);
    self.position += to.len();
  }

  /// v8: src/snapshot/snapshot-source-sink.h SnapshotByteSource::GetUint30
  /// 游程编码解码 (与 `put_uint30` 对偶, 逐字节 | 后一次掩码, 无分支惩罚)。
  /// 高位读满 4 字节; 若剩余不足, 缺失字节视作 0 — 掩码只保留 `bytes` 低字节,
  /// 结果与原版逐字节读等价 (原版依赖缓冲区尾部富余, 此处对边界安全化)。
  #[inline]
  #[must_use]
  pub fn get_uint30(&mut self) -> u32 {
    let mut answer: u32 = 0;
    for k in 0..4 {
      let p = self.position + k;
      let b: u8 = if p < self.length { self.data[p] } else { 0 };
      answer |= (b as u32) << (8 * k);
    }
    let bytes = (answer & 3) + 1;
    self.advance(bytes as usize);
    let mask = u32::MAX >> (32 - (bytes << 3));
    (answer & mask) >> 2
  }

  /// v8: src/snapshot/snapshot-source-sink.h SnapshotByteSource::GetUint32
  /// 按宿主字节序读 4 字节 (原版 memcpy, 小端平台直读; 此处小端序列化)。
  #[inline]
  #[must_use]
  pub fn get_uint32(&mut self) -> u32 {
    debug_assert!(self.position + 4 <= self.length, "GetUint32 越界");
    let mut buf = [0u8; 4];
    self.copy_raw(&mut buf);
    u32::from_le_bytes(buf)
  }

  /// v8: src/snapshot/snapshot-source-sink.cc SnapshotByteSource::GetBlob
  /// 先 `get_uint30` 读长度, 返回载荷切片引用并前移游标 (返回长度, out 数据)。
  pub fn get_blob(&mut self) -> (usize, &'a [u8]) {
    let size = self.get_uint30() as usize;
    debug_assert!(self.position + size <= self.length, "GetBlob 越界");
    let blob = &self.data[self.position..self.position + size];
    self.advance(size);
    (size, blob)
  }

  /// v8: src/snapshot/snapshot-source-sink.h SnapshotByteSource::position
  #[inline]
  #[must_use]
  pub const fn position(&self) -> usize {
    self.position
  }

  /// v8: src/snapshot/snapshot-source-sink.h SnapshotByteSource::set_position
  #[inline]
  pub fn set_position(&mut self, position: usize) {
    self.position = position;
  }

  /// v8: src/snapshot/snapshot-source-sink.h SnapshotByteSource::data
  #[inline]
  #[must_use]
  pub const fn data(&self) -> &'a [u8] {
    self.data
  }

  /// v8: src/snapshot/snapshot-source-sink.h SnapshotByteSource::length
  #[inline]
  #[must_use]
  pub const fn length(&self) -> usize {
    self.length
  }
}

/// v8: src/snapshot/snapshot-source-sink.h SnapshotByteSink
/// 字节汇 (落地存储, Vec<u8>)。原版带 `description` 调试标签, 此处 trace 记录。
#[derive(Debug, Default, Clone)]
pub struct SnapshotByteSink {
  data: Vec<u8>,
}

impl SnapshotByteSink {
  /// v8: src/snapshot/snapshot-source-sink.h SnapshotByteSink::SnapshotByteSink
  #[inline]
  #[must_use]
  pub fn new() -> Self {
    Self::default()
  }

  /// 预分配初始容量 (原版 explicit SnapshotByteSink(int initial_size))。
  #[inline]
  #[must_use]
  pub fn with_capacity(initial_size: usize) -> Self {
    Self {
      data: Vec::with_capacity(initial_size),
    }
  }

  /// v8: src/snapshot/snapshot-source-sink.h SnapshotByteSink::Put
  #[inline]
  pub fn put(&mut self, b: u8, description: &str) {
    trace!("Sink.Put {b:#04x} {description}");
    self.data.push(b);
  }

  /// v8: src/snapshot/snapshot-source-sink.cc SnapshotByteSink::PutN
  #[inline]
  pub fn put_n(&mut self, number_of_bytes: usize, v: u8, description: &str) {
    trace!("Sink.PutN {number_of_bytes}x{v:#04x} {description}");
    self.data.resize(self.data.len() + number_of_bytes, v);
  }

  /// v8: src/snapshot/snapshot-source-sink.cc SnapshotByteSink::PutUint30
  /// 游程编码 (必须 < 2^30): 左移 2 位腾出低 2 位长度码, 按值宽写 1..4 字节。
  pub fn put_uint30(&mut self, integer: u32, description: &str) {
    debug_assert!(integer < (1 << 30), "PutUint30 超出 uint30 范围");
    let mut v = integer << 2;
    let mut bytes: u32 = 1;
    if v > 0xFF {
      bytes = 2;
    }
    if v > 0xFFFF {
      bytes = 3;
    }
    if v > 0xFF_FFFF {
      bytes = 4;
    }
    v |= bytes - 1;
    trace!("Sink.PutUint30 {integer} bytes={bytes} {description}");
    self.put((v & 0xFF) as u8, "IntPart1");
    if bytes > 1 {
      self.put(((v >> 8) & 0xFF) as u8, "IntPart2");
    }
    if bytes > 2 {
      self.put(((v >> 16) & 0xFF) as u8, "IntPart3");
    }
    if bytes > 3 {
      self.put(((v >> 24) & 0xFF) as u8, "IntPart4");
    }
  }

  /// v8: src/snapshot/snapshot-source-sink.h SnapshotByteSink::PutUint32
  #[inline]
  pub fn put_uint32(&mut self, integer: u32, description: &str) {
    self.put_raw(&integer.to_le_bytes(), description);
  }

  /// v8: src/snapshot/snapshot-source-sink.cc SnapshotByteSink::PutRaw
  #[inline]
  pub fn put_raw(&mut self, data: &[u8], description: &str) {
    trace!("Sink.PutRaw {} 字节 {description}", data.len());
    self.data.extend_from_slice(data);
  }

  /// v8: src/snapshot/snapshot-source-sink.cc SnapshotByteSink::Append
  pub fn append(&mut self, other: &Self) {
    self.data.extend_from_slice(&other.data);
  }

  /// v8: src/snapshot/snapshot-source-sink.h SnapshotByteSink::Position
  #[inline]
  #[must_use]
  pub fn position(&self) -> usize {
    self.data.len()
  }

  /// v8: src/snapshot/snapshot-source-sink.h SnapshotByteSink::data
  #[inline]
  #[must_use]
  pub fn data(&self) -> &[u8] {
    &self.data
  }
}

/// Uint30 编码可表示的最大值 (原版 CHECK_LT(integer, 1UL << 30))。
pub const UINT30_MAX: u32 = (1 << 30) - 1;

// 条件分支登记: #ifdef V8_COMPRESS_POINTERS x1 — CopySlots 的 Tagged_t (4 字节压缩槽)
// 重载仅在压缩指针构建存在, 依赖真实堆槽类型, 推迟 (见 readme/zh.md)。
// CopySlots 面向 Address*/AtomicWord 槽, 依赖 v8-common 的 AtomicWord 与 Relaxed_Store,
// 归堆/内存语义族, 登记推迟。
