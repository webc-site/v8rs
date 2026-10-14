// 对应 v8/src/codegen/safepoint-table.{h,cc} 平台无关编解码面 (手写语义体, W11CG 按原版 1:1 重写)
// 编码布局 (safepoint-table.h:222-238):
//   - uint32 stack_slot_count (dd, LE)
//   - uint32 config: ByteLengthField(位 0..30, 含头部) | HasTaggedRegisters(bit30) | HasDeoptData(bit31)
//   - 条目序列: uleb pc_delta → [deopt_delta uleb + trampoline_delta uleb | 0] → [uleb register_indexes]
//     → 栈槽位图 xor-patch (compact/full 双形态, 槽序按 stack_slot_count-1-index 翻转)

use crate::error::{Error, Result};

pub const K_STACK_SLOTS_OFFSET: usize = 0;
pub const K_CONFIG_OFFSET: usize = 4;
pub const K_HEADER_SIZE: usize = 8;
pub const K_NO_DEOPT_INDEX: i32 = -1;
pub const K_NO_TRAMPOLINE_PC: i32 = -1;
/// v8: safepoint-table.cc:29 kImplicitStartPC — PC 自 1 起, 使去重早期条目计为已初始化
pub const K_IMPLICIT_START_PC: u32 = 1;
/// v8: safepoint-table.cc:30 kImplicitStartDeoptIndex
pub const K_IMPLICIT_START_DEOPT_INDEX: i32 = -1;
/// v8: safepoint-table.cc:31 kImplicitStartTrampoline
pub const K_IMPLICIT_START_TRAMPOLINE: i32 = 0;
/// v8: safepoint-table.cc:96 kMinSkipBits — 少于 7 个跳过位时直接塞进 xor 掩码更高效
pub const K_MIN_SKIP_BITS: u32 = 7;

/// v8: base::BitField<uint32_t, 0, 30> ByteLengthField 掩码
const K_BYTE_LENGTH_MASK: u32 = (1 << 30) - 1;
/// v8: HasTaggedRegistersField (ByteLengthField::Next<bool, 1>) 位号
const K_HAS_TAGGED_REGISTERS_BIT: u32 = 30;
/// v8: HasDeoptDataField 位号
const K_HAS_DEOPT_DATA_BIT: u32 = 31;

/// kDataBits: BitVector 字宽位 (uintptr_t = 64 位目标)
const K_DATA_BITS: usize = usize::BITS as usize;
/// kDataBitShift
const K_DATA_BIT_SHIFT: u32 = usize::BITS.ilog2();

/// v8: src/codegen/safepoint-table.cc:73 emit_uleb — db 序列写入
pub fn write_uleb32(buf: &mut Vec<u8>, mut val: u32) {
  loop {
    let mut byte = (val & 0x7f) as u8;
    val >>= 7;
    if val != 0 {
      byte |= 0x80;
    }
    buf.push(byte);
    if val == 0 {
      break;
    }
  }
}

/// v8: src/codegen/safepoint-table.cc:80 read_uleb — 消费游标读取
/// (原版越界即 UB; Rust 侧以类型化错误兜底, 语义对合法表等价)
pub fn read_uleb32(buf: &[u8], offset: &mut usize) -> Result<u32> {
  let mut result = 0u32;
  let mut shift = 0;
  while *offset < buf.len() {
    let byte = buf[*offset];
    *offset += 1;
    result |= ((byte & 0x7f) as u32) << shift;
    if (byte & 0x80) == 0 {
      return Ok(result);
    }
    shift += 7;
    if shift >= 35 {
      return Err(Error::InvalidSafepointTable("ULEB32 overflow"));
    }
  }
  Err(Error::InvalidSafepointTable(
    "unexpected EOF reading ULEB32",
  ))
}

/// v8: src/utils/bit-vector.h BitVector — 定长位向量 (xor patch 结果)
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BitVector {
  /// 原版 data_begin_ 字数组
  data: Vec<usize>,
  /// 原版 length_ (位)
  length: usize,
}

impl BitVector {
  /// v8: bit-vector.h BitVector(length, zone) + Add(index) — 显式构造入口 (测试/patch 复用)
  #[must_use]
  pub fn from_bits(length: usize, bits: &[usize]) -> Self {
    let mut out = Self {
      data: vec![0usize; length.div_ceil(K_DATA_BITS)],
      length,
    };
    for &b in bits {
      out.add(b);
    }
    out
  }

  /// v8: bit-vector.h BitVector::Add — DCHECK_LT(index, length_)
  pub fn add(&mut self, index: usize) {
    debug_assert!(
      index < self.length,
      "原版 BitVector::Add DCHECK_LT(index, length_)"
    );
    if index < self.length {
      self.data[index / K_DATA_BITS] |= 1usize << (index % K_DATA_BITS);
    }
  }

  #[must_use]
  pub const fn length(&self) -> usize {
    self.length
  }

  /// 原版 Contains(index)
  #[must_use]
  pub fn contains(&self, index: usize) -> bool {
    index < self.length && (self.data[index / K_DATA_BITS] >> (index % K_DATA_BITS)) & 1 != 0
  }

  /// 原版 begin()/rbegin(): 置位下标升序 / 降序迭代 (EncodeSafepointEntry 用降序)
  pub fn set_bits_desc(&self) -> Vec<usize> {
    let mut out = Vec::new();
    for (wi, word) in self.data.iter().enumerate().rev() {
      let mut w = *word;
      while w != 0 {
        let bit = w.trailing_zeros() as usize;
        // 尾部越过 length 的字位不计 (合法 patch 尾字无越界置位)
        let index = wi * K_DATA_BITS + bit;
        if index < self.length {
          out.push(index);
        }
        w &= w - 1;
      }
    }
    out.reverse();
    out
  }
}

/// v8: src/utils/bit-vector.h GrowableBitVector — 只增不减位向量 (EntryBuilder 栈槽集)
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct GrowableBitVector {
  /// 原版 bits_.data_begin_
  data: Vec<usize>,
}

impl GrowableBitVector {
  #[must_use]
  pub const fn new() -> Self {
    Self { data: Vec::new() }
  }

  /// 原版 Add(index): 置位 (越字自动扩容)
  pub fn add(&mut self, index: usize) {
    let word = index / K_DATA_BITS;
    if self.data.len() <= word {
      self.data.resize(word + 1, 0);
    }
    self.data[word] |= 1usize << (index % K_DATA_BITS);
  }

  /// 原版 BitVector::UsedLength(): 最后置位 +1 (空向量 0)
  #[must_use]
  pub fn used_length(&self) -> usize {
    for (wi, word) in self.data.iter().enumerate().rev() {
      if *word != 0 {
        return wi * K_DATA_BITS + (K_DATA_BITS - word.leading_zeros() as usize);
      }
    }
    0
  }

  /// 原版 data_begin_[i] 读取 (越字视为 0, 对应 read_i < word_length 三元)
  #[must_use]
  fn word_at(&self, i: usize) -> usize {
    self.data.get(i).copied().unwrap_or(0)
  }

  /// 由 SafepEntry 位序 Vec<bool> 构造 (index 即位号)
  #[must_use]
  pub fn from_bools(bits: &[bool]) -> Self {
    let mut v = Self::new();
    for (i, &b) in bits.iter().enumerate() {
      if b {
        v.add(i);
      }
    }
    v
  }
}

/// v8: src/codegen/safepoint-table.cc:580 CompareAndCreateXorPatch —
/// 契约: 结果异或到 {a} 上自 {common_prefix_bits} 起得到 {b} 的置位集。
/// 返回 (patch, common_prefix_bits); 相同集时 (None, u32::MAX)。
#[must_use]
pub fn compare_and_create_xor_patch(
  v1: &GrowableBitVector,
  v2: &GrowableBitVector,
) -> (Option<BitVector>, u32) {
  let a_length = v1.used_length();
  let b_length = v2.used_length();
  let a_word_length = (a_length + K_DATA_BITS - 1) >> K_DATA_BIT_SHIFT;
  let b_word_length = (b_length + K_DATA_BITS - 1) >> K_DATA_BIT_SHIFT;
  let max_common_bits = a_length.min(b_length);
  let max_common_words = (max_common_bits + K_DATA_BITS - 1) >> K_DATA_BIT_SHIFT;
  let mut different_word = 0usize;
  while different_word < max_common_words
    && v1.word_at(different_word) == v2.word_at(different_word)
  {
    different_word += 1;
  }
  // 前缀全同: 在较长向量的剩余字里找非零
  if different_word == max_common_words {
    while different_word < b_word_length && v2.word_at(different_word) == 0 {
      different_word += 1;
    }
    while different_word < a_word_length && v1.word_at(different_word) == 0 {
      different_word += 1;
    }
  }
  // 交叠部分相同且长者仅尾零 → 视为相同集
  if different_word >= b_word_length && different_word >= a_word_length {
    return (None, u32::MAX);
  }
  // 找到差异字
  let a_word = if different_word < a_word_length {
    v1.word_at(different_word)
  } else {
    0
  };
  let b_word = if different_word < b_word_length {
    v2.word_at(different_word)
  } else {
    0
  };
  debug_assert_ne!(a_word, b_word, "差异字必须不同");
  let diff = a_word ^ b_word;
  let different_bit = diff.trailing_zeros() as usize;
  let common_prefix_bits = (different_word * K_DATA_BITS + different_bit) as u32;

  // 找末位差异: 长度不等取长者, 否则跳过相同尾字
  let result_end = if a_length != b_length {
    a_length.max(b_length)
  } else {
    let highest_bit = a_length - 1;
    let mut result_end_word = highest_bit >> K_DATA_BIT_SHIFT;
    while v1.word_at(result_end_word) == v2.word_at(result_end_word) {
      // 原版依赖: 非相同集必在 [0, result_end_word] 内有差异字
      debug_assert!(result_end_word > 0, "原版 DCHECK: 必存在差异字");
      result_end_word -= 1;
    }
    let diff = v1.word_at(result_end_word) ^ v2.word_at(result_end_word);
    debug_assert_ne!(diff, 0);
    let identical_tail = diff.leading_zeros() as usize;
    (result_end_word + 1) * K_DATA_BITS - identical_tail
  };

  // 分配并填充结果
  let suffix_length = result_end - common_prefix_bits as usize;
  debug_assert_ne!(suffix_length, 0);
  let mut result = BitVector {
    data: vec![0usize; suffix_length.div_ceil(K_DATA_BITS)],
    length: suffix_length,
  };
  let result_words = result.data.len();
  if different_bit == 0 {
    for (i, slot) in result.data.iter_mut().enumerate() {
      let read_i = different_word + i;
      *slot = v1.word_at(read_i) ^ v2.word_at(read_i);
    }
  } else {
    let a_word = if different_word < a_word_length {
      v1.word_at(different_word)
    } else {
      0
    };
    let b_word = if different_word < b_word_length {
      v2.word_at(different_word)
    } else {
      0
    };
    let mut carry = (a_word ^ b_word) >> different_bit;
    let left_shift = K_DATA_BITS - different_bit;
    for (i, slot) in result.data.iter_mut().enumerate() {
      let read_i = different_word + i + 1;
      let word = v1.word_at(read_i) ^ v2.word_at(read_i);
      *slot = carry | (word << left_shift);
      carry = word >> different_bit;
    }
  }
  // v8: safepoint-table.cc:678-687 DEBUG — patch 首末位必为需翻转位, 尾字越 length 位为 0
  debug_assert!(result.contains(0));
  debug_assert!(result.contains(suffix_length - 1));
  if suffix_length != result_words * K_DATA_BITS {
    debug_assert_eq!(
      result.data[result_words - 1] >> (suffix_length % K_DATA_BITS),
      0
    );
  }
  (Some(result), common_prefix_bits)
}

/// v8: src/codegen/safepoint-table.cc:99 EncodeSafepointEntry —
/// 栈槽 xor patch 双形态编码 (full: 长度头+尾部字节; compact: 7 位 LEB 位流), 写入 out (db)。
pub fn encode_safepoint_entry(
  stack_slot_count: u32,
  common_prefix: u32,
  patch: Option<&BitVector>,
  out: &mut Vec<u8>,
) {
  // v8: safepoint-table.cc:101-104 无差异即单字节 0
  let Some(patch) = patch else {
    out.push(0);
    return;
  };
  debug_assert!(common_prefix < stack_slot_count);
  debug_assert_ne!(patch.length(), 0);
  // 原版 flip: 额外 -1 使最高 index 映射到 0
  let flip = |index: u32| stack_slot_count - 1 - index;
  let tail_bits = stack_slot_count - common_prefix;
  let tail_bytes = tail_bits.div_ceil(8);
  let skipped_bits = tail_bits - patch.length() as u32;

  // 两种编码尺寸估算 (假设计数均可单 LEB 字节容纳)
  let byte_for_length = 1;
  let approx_bytes_full_encoding = byte_for_length + tail_bytes as i32;
  let mut bytes_for_skip = 0;
  let mut leb_payload_bits = patch.length() as i32;
  if skipped_bits >= K_MIN_SKIP_BITS {
    bytes_for_skip = 1;
  } else {
    leb_payload_bits += skipped_bits as i32;
  }
  let approx_bytes_compact_encoding = bytes_for_skip + (leb_payload_bits + 6) / 7;
  let bits = patch.set_bits_desc();

  // 全量编码
  if approx_bytes_full_encoding < approx_bytes_compact_encoding {
    write_uleb32(out, tail_bytes << 1);
    let mut end_of_current = 8i64;
    let mut current = 0u8;
    for idx in bits {
      let bit = flip(common_prefix + idx as u32) as i64;
      while bit >= end_of_current {
        out.push(current);
        current = 0;
        end_of_current += 8;
      }
      current |= 1 << (bit % 8);
    }
    out.push(current);
    return;
  }

  // 紧凑编码
  let skipped_bits = if skipped_bits >= K_MIN_SKIP_BITS {
    let encoded_skipped_bits = skipped_bits - K_MIN_SKIP_BITS;
    debug_assert_eq!(encoded_skipped_bits, (encoded_skipped_bits << 2) >> 2);
    write_uleb32(out, (encoded_skipped_bits << 2) | 0b11);
    skipped_bits
  } else {
    0
  };

  let mut current = 0b01u8;
  let mut start_of_current = skipped_bits as i64 - 2 /* tag 位宽 */;
  let mut end_of_current = start_of_current + 7;
  for idx in bits {
    let bit = flip(common_prefix + idx as u32) as i64;
    debug_assert!(bit < stack_slot_count as i64);
    debug_assert!(bit >= start_of_current);
    while bit >= end_of_current {
      out.push(0x80 | current);
      current = 0;
      start_of_current = end_of_current;
      end_of_current = start_of_current + 7;
    }
    current |= 1 << (bit - start_of_current);
  }
  out.push(current);
}

/// 安全字节读取 (原版 *ptr++ 直读; Rust 对畸形表返回错误)
fn read_byte(buf: &[u8], cursor: &mut usize) -> Result<u8> {
  let b = *buf
    .get(*cursor)
    .ok_or(Error::InvalidSafepointTable("unexpected EOF"))?;
  *cursor += 1;
  Ok(b)
}

/// v8: src/codegen/safepoint-table.cc:188 DecodeSafepointEntry<bool update_tagged_slots> —
/// 消费 patch 字节流, 异或累积进 tagged_slots 字节图; UPDATE=false 仅跳字节。
pub fn decode_safepoint_entry<const UPDATE: bool>(
  bytes: &[u8],
  ptr: &mut usize,
  tagged_slots: &mut [u8],
) -> Result<()> {
  let first_byte = *bytes
    .get(*ptr)
    .ok_or(Error::InvalidSafepointTable("unexpected EOF"))?;
  if first_byte == 0 {
    *ptr += 1;
    return Ok(());
  }
  let mut cursor = 0usize;

  // 不需解码时按编码形态跳过相应字节数
  if !UPDATE {
    if (first_byte & 1) == 0 {
      let num_bytes = read_uleb32(bytes, ptr)? >> 1;
      *ptr += num_bytes as usize;
      if *ptr > bytes.len() {
        return Err(Error::InvalidSafepointTable("full-encoding overrun"));
      }
      return Ok(());
    }
    if (first_byte & 0b11) == 0b11 {
      read_uleb32(bytes, ptr)?; // skip_bits
    }
    let mut byte = read_byte(bytes, ptr)?;
    while byte & 0x80 != 0 {
      byte = read_byte(bytes, ptr)?;
    }
    return Ok(());
  }

  // 全量编码: 逐 8 字节 (kSystemPointerSize) xor, 尾字节逐个 xor
  if (first_byte & 1) == 0 {
    let mut num_bytes = (read_uleb32(bytes, ptr)? >> 1) as usize;
    while num_bytes > 8 {
      let patch_bytes: [u8; 8] = bytes
        .get(*ptr..*ptr + 8)
        .ok_or(Error::InvalidSafepointTable("full-encoding overrun"))?
        .try_into()
        .map_err(|_| Error::InvalidSafepointTable("full-encoding overrun"))?;
      let patch = u64::from_le_bytes(patch_bytes);
      let current: &mut [u8; 8] = tagged_slots
        .get_mut(cursor..cursor + 8)
        .ok_or(Error::InvalidSafepointTable("tagged slots overrun"))?
        .try_into()
        .map_err(|_| Error::InvalidSafepointTable("tagged slots overrun"))?;
      *current = (u64::from_le_bytes(*current) ^ patch).to_le_bytes();
      *ptr += 8;
      cursor += 8;
      num_bytes -= 8;
    }
    while num_bytes > 0 {
      let b = read_byte(bytes, ptr)?;
      let slot = tagged_slots
        .get_mut(cursor)
        .ok_or(Error::InvalidSafepointTable("tagged slots overrun"))?;
      *slot ^= b;
      cursor += 1;
      num_bytes -= 1;
    }
    return Ok(());
  }

  // 紧凑编码
  let mut skip_bits = 0u32;
  if (first_byte & 0b11) == 0b11 {
    skip_bits = (read_uleb32(bytes, ptr)? >> 2) + K_MIN_SKIP_BITS;
    if skip_bits >= 8 {
      cursor += (skip_bits / 8) as usize;
      skip_bits %= 8;
    }
  }
  // 首字节特殊: 含 tag, 仅 5 位载荷
  let mut patch_byte = read_byte(bytes, ptr)?;
  let mut patch = ((patch_byte >> 2) as u32 & 0x1F) << skip_bits;
  let mut bits_in_patch = 5 + skip_bits;
  loop {
    // 累积足 8 位即异或落槽
    if bits_in_patch >= 8 {
      let slot = tagged_slots
        .get_mut(cursor)
        .ok_or(Error::InvalidSafepointTable("tagged slots overrun"))?;
      *slot ^= (patch & 0xff) as u8;
      cursor += 1;
      patch >>= 8;
      bits_in_patch -= 8;
    }
    // 无后续字节: 残余位非零才异或 (防越界空写)
    if patch_byte & 0x80 == 0 {
      if patch != 0 {
        let slot = tagged_slots
          .get_mut(cursor)
          .ok_or(Error::InvalidSafepointTable("tagged slots overrun"))?;
        *slot ^= (patch & 0xff) as u8;
      }
      break;
    }
    patch_byte = read_byte(bytes, ptr)?;
    patch |= ((patch_byte & 0x7F) as u32) << bits_in_patch;
    bits_in_patch += 7;
  }
  Ok(())
}

/// v8: src/codegen/safepoint-table.h:31 SafepointEntry (SafepointEntryBase 派生) —
/// 单 safepoint 解码结果: pc / deopt / trampoline / tagged 寄存器与栈槽位图。
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SafepointEntry {
  pub pc: u32,
  pub deopt_index: i32,
  pub trampoline_pc: i32,
  pub tagged_register_indexes: u32,
  pub tagged_slots: Vec<bool>,
}

impl SafepointEntry {
  /// 原版 Reset(pc): deopt/trampoline 归哨兵, 位图清空
  #[inline]
  #[must_use]
  pub fn new(pc: u32) -> Self {
    Self {
      pc,
      deopt_index: K_NO_DEOPT_INDEX,
      trampoline_pc: K_NO_TRAMPOLINE_PC,
      tagged_register_indexes: 0,
      tagged_slots: Vec::new(),
    }
  }

  /// 原版 ResetTaggedSlots(num): 清空为 num 槽位图 (位序 Vec<bool> 承载)
  pub fn reset_tagged_slots(&mut self, num_tagged_slots: u32) {
    let bytes = (num_tagged_slots as usize).div_ceil(8);
    self.tagged_slots = vec![false; bytes * 8];
    self.tagged_register_indexes = 0;
  }

  /// 位图置位 (原版测试侧 Add/GrowableBitVector 语义)
  #[inline]
  pub fn set_tagged_slot(&mut self, index: usize) {
    if self.tagged_slots.len() <= index {
      self.tagged_slots.resize(index + 1, false);
    }
    self.tagged_slots[index] = true;
  }

  /// 位图测位
  #[inline]
  #[must_use]
  pub fn is_tagged_slot(&self, index: usize) -> bool {
    self.tagged_slots.get(index).copied().unwrap_or(false)
  }
}

/// v8: safepoint-table.h:245 CompressedEntry — 提交期压缩条目
#[derive(Clone, Debug)]
struct CompressedEntry {
  pc: u32,
  register_indexes: u32,
  common_bits: u32,
  /// 原版 stack_indexes_tail (Zone BitVector*), None = 与前驱全同
  stack_indexes_tail: Option<BitVector>,
  trampoline: i32,
  deopt_index: i32,
}

/// v8: safepoint-table.h:168 SafepointTableBuilder —
/// DefineSafepoint/Commit 流式压缩 + Emit 终编码 (Emit 依赖面以字节 Vec 返回,
/// 原版 Align(nop 填充)/RecordComment 属汇编装配面, 由调用方缓冲承接)。
pub struct SafepointTableBuilder {
  pub stack_slots: u32,
  pub has_tagged_registers: bool,
  pub has_deopt_data: bool,
  entries: Vec<CompressedEntry>,
  /// 原版 previous_entry_->stack_indexes 旋转位 (Commit 比较基准)
  previous_bits: GrowableBitVector,
  /// 原版 entries_ 末 pc: DefineSafepoint DCHECK pc 递增序用
  last_pc: u32,
}

impl SafepointTableBuilder {
  pub fn new(stack_slots: u32, has_tagged_registers: bool, has_deopt_data: bool) -> Self {
    Self {
      stack_slots,
      has_tagged_registers,
      has_deopt_data,
      entries: Vec::new(),
      previous_bits: GrowableBitVector::new(),
      last_pc: 0,
    }
  }

  /// 原版 DefineSafepoint + ~Safepoint + Commit (safepoint-table.cc:445-463):
  /// 与前一全量向量做 xor-patch 后压缩入列; pc 必须严格递增。
  pub fn add_entry(&mut self, entry: SafepointEntry) {
    debug_assert!(entry.pc > self.last_pc, "原版 DCHECK: 条目按 pc 严格递增");
    self.last_pc = entry.pc;
    let current = GrowableBitVector::from_bools(&entry.tagged_slots);
    // v8: safepoint-table.cc:456 CompareAndCreateXorPatch(zone, current, previous, &common)
    let (stack_indexes, common_bits) = compare_and_create_xor_patch(&current, &self.previous_bits);
    debug_assert!(
      stack_indexes.is_none() == (common_bits == u32::MAX),
      "无差异后缀须全同前缀"
    );
    self.previous_bits = current;
    self.entries.push(CompressedEntry {
      pc: entry.pc,
      register_indexes: entry.tagged_register_indexes,
      common_bits,
      stack_indexes_tail: stack_indexes,
      trampoline: entry.trampoline_pc,
      deopt_index: entry.deopt_index,
    });
  }

  /// 原版 UpdateDeoptimizationInfo (safepoint-table.cc:465-479):
  /// 补写指定 pc 条目的 deopt/trampoline 并回写 has_deopt_data。
  pub fn update_deoptimization_info(
    &mut self,
    pc: u32,
    trampoline: i32,
    start: usize,
    deopt_index: i32,
  ) -> usize {
    debug_assert_ne!(trampoline, K_NO_TRAMPOLINE_PC);
    debug_assert_ne!(deopt_index, K_NO_DEOPT_INDEX);
    self.has_deopt_data = true;
    let mut index = start;
    while self.entries.get(index).is_some_and(|e| e.pc != pc) {
      index += 1;
    }
    // 原版 DCHECK any_of: pc 必须存在于 entries[start..]
    debug_assert!(
      self.entries.get(index).is_some(),
      "原版 DCHECK: pc 须在 entries[{start}..] 内"
    );
    if let Some(e) = self.entries.get_mut(index) {
      e.trampoline = trampoline;
      e.deopt_index = deopt_index;
    }
    index
  }

  /// 原版 SafepointTableBuilder::Emit (safepoint-table.cc:481-563) —
  /// 表字节流: 头部 dd(stack_slot_count)+dd(config), 条目 uleb 差分 + patch,
  /// 末回填 config (byte_length 含头部)。
  #[must_use]
  pub fn emit(&self) -> Vec<u8> {
    let mut out: Vec<u8> = Vec::new();
    // 表头 (原版 static_assert: kStackSlotsOffset/kConfigOffset 各 1 个 kIntSize)
    out.extend_from_slice(&self.stack_slots.to_le_bytes());
    out.extend_from_slice(&0u32.to_le_bytes()); // config 占位, 末尾 patch

    // v8: safepoint-table.cc:502-507 — implicit_start 的 trampoline/deopt 用
    // CompressedEntry 成员默认值 (kNoTrampolinePC/kNoDeoptIndex);
    // kImplicitStartTrampoline 仅用于 last_trampoline 累积器。
    let implicit_start = CompressedEntry {
      pc: K_IMPLICIT_START_PC,
      register_indexes: 0,
      common_bits: 0,
      stack_indexes_tail: None,
      trampoline: K_NO_TRAMPOLINE_PC,
      deopt_index: K_NO_DEOPT_INDEX,
    };
    let mut previous = &implicit_start;
    let mut last_deopt_index = K_IMPLICIT_START_DEOPT_INDEX;
    let mut last_trampoline = K_IMPLICIT_START_TRAMPOLINE;

    for entry in &self.entries {
      // 条目按 pc 有序 (原版 DCHECK_GT)
      debug_assert!(
        entry.pc > previous.pc,
        "原版 DCHECK_GT(entry.pc, previous.pc)"
      );

      // 除 pc 外与前驱全同的条目跳过 (可能全跳, 解码器可处理)
      if entry.deopt_index == previous.deopt_index
        && entry.register_indexes == previous.register_indexes
        && entry.stack_indexes_tail.is_none()
      {
        debug_assert_eq!(entry.trampoline, previous.trampoline);
        continue;
      }

      write_uleb32(&mut out, entry.pc - previous.pc);
      if self.has_deopt_data {
        if entry.trampoline == K_NO_TRAMPOLINE_PC {
          // 条目要么兼有 trampoline 与 deopt index, 要么皆无
          debug_assert_eq!(entry.deopt_index, K_NO_DEOPT_INDEX);
          out.push(0);
        } else {
          debug_assert_ne!(entry.deopt_index, K_NO_DEOPT_INDEX);
          debug_assert!(entry.deopt_index > previous.deopt_index, "deopt 索引递增");
          debug_assert!(entry.trampoline > previous.trampoline, "trampoline pc 递增");
          debug_assert!(
            entry.trampoline > self.entries.last().map_or(0, |e| e.pc) as i32,
            "trampoline pc 大于常规 pc"
          );
          write_uleb32(&mut out, (entry.deopt_index - last_deopt_index) as u32);
          last_deopt_index = entry.deopt_index;
          write_uleb32(&mut out, (entry.trampoline - last_trampoline) as u32);
          last_trampoline = entry.trampoline;
        }
      } else {
        debug_assert_eq!(K_NO_DEOPT_INDEX, entry.deopt_index);
        debug_assert_eq!(K_NO_TRAMPOLINE_PC, entry.trampoline);
      }
      if self.has_tagged_registers {
        write_uleb32(&mut out, entry.register_indexes);
      }

      // 栈槽位图 (编码槽序翻转: index=0 为 SP 侧起点)
      encode_safepoint_entry(
        self.stack_slots,
        entry.common_bits,
        entry.stack_indexes_tail.as_ref(),
        &mut out,
      );
      previous = entry;
    }

    // v8: safepoint-table.cc:555-562 config 回填 (byte_length 含头部)
    let byte_length = out.len() as u32;
    let mut config = byte_length & K_BYTE_LENGTH_MASK;
    if self.has_tagged_registers {
      config |= 1 << K_HAS_TAGGED_REGISTERS_BIT;
    }
    if self.has_deopt_data {
      config |= 1 << K_HAS_DEOPT_DATA_BIT;
    }
    out[K_CONFIG_OFFSET..K_CONFIG_OFFSET + 4].copy_from_slice(&config.to_le_bytes());
    out
  }
}

/// v8: src/codegen/safepoint-table.h:87 SafepointTable —
/// 嵌入 InstructionStream 的表访问器 (one-shot 消费; Rust 面 parse 即完整解码,
/// instruction_start 以 0 为基, pc 即 pc_offset)。
#[derive(Clone, Debug)]
pub struct SafepointTable {
  pub stack_slots: u32,
  pub byte_length: u32,
  pub has_tagged_registers: bool,
  pub has_deopt_data: bool,
  pub entries: Vec<SafepointEntry>,
}

impl SafepointTable {
  /// 原版构造 (safepoint-table.cc:53-68) + ResetIteration/Advance 全量解码:
  /// 头字段读 config 位域, 条目序列按 xor 累积还原位图。
  pub fn parse(bytes: &[u8]) -> Result<Self> {
    if bytes.len() < K_HEADER_SIZE {
      return Err(Error::InvalidSafepointTable("header too short"));
    }
    let stack_slots = u32::from_le_bytes(
      bytes[K_STACK_SLOTS_OFFSET..4]
        .try_into()
        .unwrap_or_default(),
    );
    let config = u32::from_le_bytes(bytes[K_CONFIG_OFFSET..8].try_into().unwrap_or_default());
    // v8: ByteLengthField::decode / HasTaggedRegistersField / HasDeoptDataField
    let byte_length = config & K_BYTE_LENGTH_MASK;
    let has_tagged_registers = (config >> K_HAS_TAGGED_REGISTERS_BIT) & 1 != 0;
    let has_deopt_data = (config >> K_HAS_DEOPT_DATA_BIT) & 1 != 0;
    if (byte_length as usize) > bytes.len() {
      return Err(Error::InvalidSafepointTable("byte_length exceeds buffer"));
    }
    let end = byte_length as usize;

    let mut ptr = K_HEADER_SIZE;
    let has_more = ptr < end;
    let mut entry = SafepointEntry::new(K_IMPLICIT_START_PC);
    entry.reset_tagged_slots(stack_slots);
    let mut last_deopt_index = K_IMPLICIT_START_DEOPT_INDEX;
    let mut last_trampoline = K_IMPLICIT_START_TRAMPOLINE;
    // v8: safepoint-table.cc:281 next_pc_ = kImplicitStartPC + read_uleb(&ptr_)
    let mut next_pc = if has_more {
      K_IMPLICIT_START_PC + read_uleb32(bytes, &mut ptr)?
    } else {
      u32::MAX
    };

    let mut entries = Vec::new();
    let mut slot_bytes = vec![0u8; (stack_slots as usize).div_ceil(8)];
    while ptr < end {
      // v8: safepoint-table.cc:285-306 Advance<true>
      entry.pc = next_pc;
      if has_deopt_data {
        let deopt_index_delta = read_uleb32(bytes, &mut ptr)?;
        if deopt_index_delta == 0 {
          entry.deopt_index = K_NO_DEOPT_INDEX;
          entry.trampoline_pc = K_NO_TRAMPOLINE_PC;
        } else {
          last_deopt_index += deopt_index_delta as i32;
          let trampoline_delta = read_uleb32(bytes, &mut ptr)?;
          last_trampoline += trampoline_delta as i32;
          entry.deopt_index = last_deopt_index;
          entry.trampoline_pc = last_trampoline;
        }
      }
      if has_tagged_registers {
        entry.tagged_register_indexes = read_uleb32(bytes, &mut ptr)?;
      }
      decode_safepoint_entry::<true>(bytes, &mut ptr, &mut slot_bytes)?;
      entry.tagged_slots = (0..stack_slots as usize)
        .map(|i| {
          let flipped = stack_slots as usize - 1 - i;
          (slot_bytes[flipped / 8] >> (flipped % 8)) & 1 != 0
        })
        .collect();
      next_pc = if ptr < end {
        next_pc + read_uleb32(bytes, &mut ptr)?
      } else {
        u32::MAX
      };
      entries.push(entry.clone());
    }

    Ok(Self {
      stack_slots,
      byte_length,
      has_tagged_registers,
      has_deopt_data,
      entries,
    })
  }

  /// 原版 FindEntryImpl (safepoint-table.cc:319-384) —
  /// 返回覆盖 pc 的条目 (末个 entry.pc <= pc; trampoline 精确命中优先, 命中即该条目)。
  /// pc 小于首个存储条目时原版返回隐式起点, Rust 面以 None 表达。
  #[must_use]
  pub fn find_entry(&self, pc: u32) -> Option<&SafepointEntry> {
    let mut best: Option<&SafepointEntry> = None;
    let mut trampoline_candidate: Option<&SafepointEntry> = None;
    for e in &self.entries {
      if e.trampoline_pc == pc as i32 {
        return Some(e);
      }
      if e.trampoline_pc != K_NO_TRAMPOLINE_PC {
        if pc as i32 >= e.trampoline_pc {
          trampoline_candidate = Some(e);
        }
        if trampoline_candidate.is_some() && e.trampoline_pc > pc as i32 {
          break;
        }
      }
      if e.pc <= pc {
        best = Some(e);
      } else {
        break;
      }
    }
    // v8: safepoint-table.cc:377-383 无候选即覆盖条目; 有候选则重启迭代命中该条目
    trampoline_candidate.or(best)
  }

  /// 原版 FindReturnPC (safepoint-table.cc:308-316) —
  /// trampoline 或 pc 精确命中的条目 pc; 未命中原版 UNREACHABLE, Rust 面 None。
  #[must_use]
  pub fn find_return_pc(&self, pc: u32) -> Option<u32> {
    for e in &self.entries {
      if e.trampoline_pc == pc as i32 || e.pc == pc {
        return Some(e.pc);
      }
    }
    None
  }
}
