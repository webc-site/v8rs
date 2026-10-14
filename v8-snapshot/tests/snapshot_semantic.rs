// Wave 9 round1 审查新增: 纯可测语义面对抗用例 (边界 / 往返对偶 / 坏输入 / 全量覆盖)。
// 归属 snapshot_semantic.rs (原版 test-serialize.cc 的 UNINITIALIZED_TEST 族依赖 Isolate, 本层无独立 gtest 对应,
// 按 readme §3「补充纯层往返/位域测」并入)。

use aok::{OK, Void};
use v8_snapshot::{
  FIXED_RAW_DATA_COUNT,
  HOT_OBJECT_COUNT,
  ROOT_ARRAY_CONSTANTS_COUNT,
  // 线上格式标签
  SerializationTag,
  // 字节码体系
  SerializerBytecode,
  SnapshotByteSink,
  SnapshotByteSource,
  UINT30_MAX,
  UNUSED_SERIALIZER_BYTE_CODES,
  // 位域 / 位集
  bitset,
  blob_layout,
  builtin_lookup_entry,
  // 校验和
  checksum,
  encode,
  encoded_external_reference,
  encoded_tagged,
  layout_description,
  serialized_code_data_layout,
  serializer_reference_bits,
  // 布局
  snapshot_blob_layout,
  snapshot_data_layout,
};

#[ctor::ctor(unsafe)]
fn _log_init() {
  log_init::init();
}

/// Uint30 游程编解码对偶: 边界值逐一往返 (覆盖 1/2/3/4 字节分阶 + 上界 UINT30_MAX)。
/// 原版 snapshot-source-sink.cc PutUint30 / .h GetUint30 对偶。
#[test]
fn test_uint30_boundary_roundtrip() -> Void {
  let boundary: [u32; 15] = [
    0,
    1,
    2,
    63,
    64,
    8191,
    8192,
    16383,
    16384,
    (1 << 22) - 1,
    1 << 22,
    (1 << 28) - 1,
    1 << 28,
    UINT30_MAX - 1,
    UINT30_MAX,
  ];
  for &val in &boundary {
    let mut sink = SnapshotByteSink::new();
    sink.put_uint30(val, "u30");
    let bytes = sink.position();
    // 编码字节数须落在 1..=4 且与值宽匹配
    assert!(
      (1..=4).contains(&bytes),
      "uint30 {val} 编码宽度越界: {bytes}"
    );
    let mut source = SnapshotByteSource::new(sink.data());
    assert_eq!(source.get_uint30(), val, "uint30 {val} 往返失真");
    // 单值编解码后游标恰停在末尾
    assert_eq!(source.position(), bytes);
    assert!(!source.has_more());
  }
  OK
}

/// Uint30 交错多值往返 + 游标推进记账: 写入流与读取流位置严格同步 (原版 sink_/source_ 共用游程语义)。
#[test]
fn test_uint30_interleaved_stream() -> Void {
  let seq: Vec<u32> = (0..200u32)
    .map(|i| match i % 4 {
      0 => i,
      1 => (i * 131) % 1024,
      2 => (1 << 20) + i,
      _ => UINT30_MAX - i,
    })
    .collect();
  let mut sink = SnapshotByteSink::new();
  for &v in &seq {
    sink.put_uint30(v, "seq");
  }
  let mut source = SnapshotByteSource::new(sink.data());
  for &v in &seq {
    assert_eq!(source.get_uint30(), v);
  }
  assert_eq!(source.position(), sink.position());
  assert!(!source.has_more());
  OK
}

/// Uint30 尾部截断安全读: 声明 4 字节宽但缓冲不足时, 缺失高位视作 0 (Rust 边界安全化, 原版依赖尾部富余)。
/// 断言不 panic 且停在确定值 (坏输入面)。
#[test]
fn test_uint30_truncated_read_safe() -> Void {
  // 末位长度码声明 bytes=2, 但只有 1 字节存在 → 高字节视作 0
  let buf = [0x01u8];
  let mut source = SnapshotByteSource::new(&buf);
  let got = source.get_uint30();
  // (answer=0x01) → bytes=(0x01&3)+1=2 → mask 0xFFFF → (0x01 & 0xFFFF)>>2 = 0
  assert_eq!(got, 0);
  // 游标越过末尾不得 panic (契约: 调用方负责边界)
  assert_eq!(source.position(), 2);
  assert!(!source.has_more());
  OK
}

/// SerializationTag: 遍历全部 256 字节, 断言 from_byte 命中恰 59 个且每个命中值与码位自反,
/// 锁定 59 标签完整性 + 判别式两两互异 + from_byte 全覆盖 (原版 value-serializer.cc enum 全表)。
#[test]
fn test_serialization_tag_full_coverage() -> Void {
  let mut hits: Vec<u8> = Vec::new();
  for b in 0..=255u8 {
    if let Some(tag) = SerializationTag::from_byte(b) {
      // 自反: 标签判别式须等于反查字节
      assert_eq!(tag as u8, b, "from_byte({b:#04x}) 回指标签判别式不符");
      hits.push(b);
    }
  }
  // 命中数 == 声明总数 == 59
  assert_eq!(hits.len(), SerializationTag::COUNT);
  assert_eq!(SerializationTag::COUNT, 59);
  // 判别式互异 (hits 天然唯一, 因 0..=255 单调; 命中数==59 已蕴含 59 个不同字节)
  let mut uniq = hits.clone();
  uniq.sort_unstable();
  uniq.dedup();
  assert_eq!(uniq.len(), 59, "SerializationTag 判别式存在重复");

  // 未占用字节须拒绝: 0x01/0x02/0x7F/0x60/0x2F 均非标签 (原版保留/空闲码位)
  for b in [1u8, 2, 0x7F, 0x60, 0x2F] {
    assert_eq!(SerializationTag::from_byte(b), None, "{b:#04x} 不应是标签");
  }
  OK
}

/// 空闲字节码表 1:1 复原: 期望集 = {0x23..0x3f} ∪ {0x98..0xff}, 共 133;
/// 并交叉校验任一空闲码都不是命名 Bytecode 判别式 (原版 UNUSED_SERIALIZER_BYTE_CODES X-macro)。
#[test]
fn test_unused_bytecodes_table_exact() -> Void {
  let mut expected: Vec<u8> = Vec::new();
  for x in 0x23u16..=0x3F {
    expected.push(x as u8);
  }
  for x in 0x98u16..=0xFF {
    expected.push(x as u8);
  }
  assert_eq!(expected.len(), 133);
  assert_eq!(UNUSED_SERIALIZER_BYTE_CODES.len(), expected.len());
  for (i, &v) in UNUSED_SERIALIZER_BYTE_CODES.iter().enumerate() {
    assert_eq!(v, expected[i], "空闲码表第 {i} 项不符");
    // 空闲码必须与命名枚举互斥
    assert!(
      SerializerBytecode::from_repr(v).is_none(),
      "空闲码 {v:#04x} 不应命中命名 Bytecode"
    );
  }
  OK
}

/// Bytecode from_repr: 命名码回指 + 段内偏移码 (非命名) 返回 None + COUNT=36 + 全判别式往返。
#[test]
fn test_serializer_bytecode_from_repr() -> Void {
  // 关键命名锚
  assert_eq!(
    SerializerBytecode::from_repr(0x00),
    Some(SerializerBytecode::NewObject)
  );
  assert_eq!(
    SerializerBytecode::from_repr(0x04),
    Some(SerializerBytecode::Backref)
  );
  assert_eq!(
    SerializerBytecode::from_repr(0x22),
    Some(SerializerBytecode::ProtectedPointerPrefix)
  );
  assert_eq!(
    SerializerBytecode::from_repr(0x40),
    Some(SerializerBytecode::RootArrayConstants)
  );
  assert_eq!(
    SerializerBytecode::from_repr(0x60),
    Some(SerializerBytecode::FixedRawData)
  );
  assert_eq!(
    SerializerBytecode::from_repr(0x80),
    Some(SerializerBytecode::FixedRepeatRoot)
  );
  assert_eq!(
    SerializerBytecode::from_repr(0x90),
    Some(SerializerBytecode::HotObject)
  );
  // 段内偏移码 (kNewObject 空间编码 0x01..0x03 与各段基址之上偏移) 非命名
  for interior in [
    0x01u8, 0x02, 0x03, 0x41, 0x5F, 0x61, 0x7F, 0x81, 0x8F, 0x91, 0x97,
  ] {
    assert_eq!(
      SerializerBytecode::from_repr(interior),
      None,
      "{interior:#04x} 应为段内偏移码"
    );
  }
  // 全 36 命名判别式往返
  for val in 0..=255u8 {
    if let Some(bc) = SerializerBytecode::from_repr(val) {
      assert_eq!(bc as u8, val);
    }
  }
  assert_eq!(SerializerBytecode::COUNT, 36);
  OK
}

/// 段编解码对偶 (FixedRawData/FixedRepeatRoot/HotObject/RootArrayConstant): 全定义域双射。
/// 原版 BytecodeValueEncoder<>::Encode/Decode。
#[test]
fn test_segment_codec_bijection() -> Void {
  // NewObject 空间段 0x00..0x03
  for space in 0..encode::NUMBER_OF_SNAPSHOT_SPACES as u8 {
    let bc = encode::encode_new_object(space);
    assert_eq!(bc, space);
    assert_eq!(encode::decode_new_object(bc), space);
  }
  // FixedRawData 段: 长度 [1,32]
  for size in 1..=FIXED_RAW_DATA_COUNT as i32 {
    let bc = encode::encode_fixed_raw_data(size);
    assert!((0x60..=0x7F).contains(&bc));
    assert_eq!(encode::decode_fixed_raw_data(bc), size);
  }
  // FixedRepeatRoot 段: count [2,17]
  for count in 2..=17 {
    let bc = encode::encode_fixed_repeat_root(count);
    assert!((0x80..=0x8F).contains(&bc));
    assert_eq!(encode::decode_fixed_repeat_root(bc), count);
  }
  // HotObject 段: index [0,7]
  for idx in 0..HOT_OBJECT_COUNT as i32 {
    let bc = encode::encode_hot_object(idx);
    assert!((0x90..=0x97).contains(&bc));
  }
  // RootArrayConstant 段: index [0,31]
  for r in 0..ROOT_ARRAY_CONSTANTS_COUNT as i32 {
    let bc = encode::encode_root_array_constant(r);
    assert!((0x40..=0x5F).contains(&bc));
  }
  // VariableRepeatRootCount: 变长下限两侧往返
  for count in [18i32, 100, 4096, i32::MAX / 2] {
    let enc = encode::encode_variable_repeat_root(count);
    assert_eq!(encode::decode_variable_repeat_root(enc), count);
  }
  OK
}

/// ro::BitSet 索引映射跨 chunk 边界 (原版 chunk_index/bit_index/bit_mask, uint8 chunk)。
#[test]
fn test_bitset_index_mapping() -> Void {
  for i in [0usize, 1, 7, 8, 9, 15, 16, 63, 64, 255, 256, 1023] {
    assert_eq!(bitset::chunk_index(i), i / 8);
    assert_eq!(bitset::bit_index(i), i % 8);
    assert_eq!(bitset::bit_mask(i), 1u32 << (i % 8));
  }
  OK
}

/// SerializerReference 组合位打包往返: 对齐原版 ctor `TypeBits::encode(type) | ValueBits::encode(value)`。
/// 全 4 类型 × 值边界 (0 / 1 / 上界 2^30-1)。
#[test]
fn test_serializer_reference_bitpack() -> Void {
  use serializer_reference_bits::{
    TYPE_MASK, VALUE_MASK, decode_type, decode_value, encode_type, encode_value,
  };
  for t in 0..4u8 {
    for &v in &[0u32, 1, 7, (1 << 30) - 1] {
      let packed = encode_type(t) | encode_value(v);
      assert_eq!(decode_type(packed), t, "类型往返失真 t={t}");
      assert_eq!(decode_value(packed), v, "值往返失真 v={v}");
    }
  }
  // TypeBits(2) 与 ValueBits(30) 无重叠且恰覆盖 32 位 (原版 TypeBits + TypeBits::Next)
  assert_eq!(TYPE_MASK & VALUE_MASK, 0);
  assert_eq!(TYPE_MASK | VALUE_MASK, u32::MAX);
  OK
}

/// EncodedExternalReference 位域恰覆盖 32 位且互斥 (原版 tag(8)+is_api(1)+index(23), 判别式保序)。
/// 注: 本用例仅锁原版正确面; EncodedTagged 位序问题另见缺陷台账。
#[test]
fn test_encoded_external_reference_masks() -> Void {
  let tag = encoded_external_reference::TAG_MASK;
  let api = encoded_external_reference::IS_API_REFERENCE_MASK;
  let idx = encoded_external_reference::INDEX_MASK;
  assert_eq!(tag | api | idx, u32::MAX);
  assert_eq!(tag & api, 0);
  assert_eq!(api & idx, 0);
  assert_eq!(tag & idx, 0);
  // 位宽与偏移关系: is_api 紧跟 tag 上沿, index 再上移 1
  assert_eq!(api, 1 << 8);
  assert!(idx & (1 << 9) != 0);
  OK
}

/// EncodedTagged 位宽分割与互斥 (仅锁原版「page_index+offset=32、掩码不重叠」不变式;
/// 不锁具体位序 —— 位序失真记为缺陷台账, 见 W9SNAPR round1)。
#[test]
fn test_encoded_tagged_width_partition() -> Void {
  assert_eq!(
    encoded_tagged::OFFSET_BITS + encoded_tagged::PAGE_INDEX_BITS,
    encoded_tagged::SIZE * 8
  );
  assert_eq!(
    encoded_tagged::OFFSET_MASK & encoded_tagged::PAGE_INDEX_MASK,
    0
  );
  assert_eq!(
    encoded_tagged::OFFSET_MASK | encoded_tagged::PAGE_INDEX_MASK,
    u32::MAX
  );
  OK
}

/// 快照 blob 头布局偏移链连续性 (原版 snapshot.cc 匿名 namespace, 逐项 = 前项 + 槽宽)。
#[test]
fn test_snapshot_blob_offset_chain() -> Void {
  use snapshot_blob_layout as m;
  assert_eq!(m::NUMBER_OF_CONTEXTS_OFFSET, 0);
  assert_eq!(m::REHASHABILITY_OFFSET, m::NUMBER_OF_CONTEXTS_OFFSET + 4);
  assert_eq!(m::CHECKSUM_OFFSET, m::REHASHABILITY_OFFSET + 4);
  assert_eq!(
    m::READ_ONLY_SNAPSHOT_CHECKSUM_OFFSET,
    m::CHECKSUM_OFFSET + 4
  );
  assert_eq!(
    m::VERSION_STRING_OFFSET,
    m::READ_ONLY_SNAPSHOT_CHECKSUM_OFFSET + 4
  );
  assert_eq!(m::VERSION_STRING_LENGTH, 64);
  assert_eq!(
    m::STARTUP_OFFSET_OFFSET,
    m::VERSION_STRING_OFFSET + m::VERSION_STRING_LENGTH
  );
  assert_eq!(m::SHARED_HEAP_OFFSET_OFFSET, m::STARTUP_OFFSET_OFFSET + 4);
  assert_eq!(
    m::FIRST_CONTEXT_OFFSET_OFFSET,
    m::SHARED_HEAP_OFFSET_OFFSET + 4
  );
  // RO 快照起点 POINTER_SIZE_ALIGN(8) 对齐 (原版 ReadOnlySnapshotOffset)
  for n in 0..24u32 {
    let off = m::read_only_snapshot_offset(n);
    let raw = m::FIRST_CONTEXT_OFFSET_OFFSET + n * 4;
    assert_eq!(off, (raw + 7) & !7, "RO 偏移未 8 对齐 n={n}");
    assert_eq!(off % 8, 0);
    assert_eq!(m::context_snapshot_offset_offset(n), raw);
  }
  OK
}

/// 数据 blob 与代码缓存头链 (原版 snapshot-data.h + code-serializer.h)。
#[test]
fn test_data_and_code_header_chain() -> Void {
  use serialized_code_data_layout as c;
  use snapshot_data_layout as d;
  // SnapshotData: [魔数][载荷长度] → 头 8 字节
  assert_eq!(d::UINT32_SIZE, 4);
  assert_eq!(d::PAYLOAD_LENGTH_OFFSET, d::UINT32_SIZE);
  assert_eq!(d::HEADER_SIZE, d::PAYLOAD_LENGTH_OFFSET + d::UINT32_SIZE);
  // SerializedCodeData 偏移链 (基点 kMagicNumberOffset=0)
  assert_eq!(c::VERSION_HASH_OFFSET, 4);
  assert_eq!(c::SOURCE_HASH_OFFSET, c::VERSION_HASH_OFFSET + 4);
  assert_eq!(
    c::FLAG_HASH_OFFSET,
    c::SOURCE_HASH_OFFSET + c::SOURCE_HASH_SIZE as u32
  );
  assert_eq!(c::SOURCE_HASH_SIZE, 32); // kSizeOfSha256Digest
  assert_eq!(
    c::READ_ONLY_SNAPSHOT_CHECKSUM_OFFSET,
    c::FLAG_HASH_OFFSET + 4
  );
  assert_eq!(
    c::PAYLOAD_LENGTH_OFFSET,
    c::READ_ONLY_SNAPSHOT_CHECKSUM_OFFSET + 4
  );
  assert_eq!(
    c::TRUSTED_PAYLOAD_LENGTH_OFFSET,
    c::PAYLOAD_LENGTH_OFFSET + 4
  );
  assert_eq!(c::CHECKSUM_OFFSET, c::TRUSTED_PAYLOAD_LENGTH_OFFSET + 4);
  assert_eq!(c::UNALIGNED_HEADER_SIZE, c::CHECKSUM_OFFSET + 4);
  assert_eq!(c::HEADER_SIZE, (c::UNALIGNED_HEADER_SIZE + 7) & !7);
  assert_eq!(c::HEADER_SIZE, 64);
  OK
}

/// 内嵌 blob 数据区偏移链 (原版 embedded-data.h, 以 table_size 参数化避免伪常量)。
#[test]
fn test_embedded_blob_layout_chain() -> Void {
  use blob_layout as m;
  assert_eq!(m::EMBEDDED_BLOB_DATA_HASH_OFFSET, 0);
  assert_eq!(m::EMBEDDED_BLOB_DATA_HASH_SIZE, 8); // kSizetSize (64 位)
  assert_eq!(m::embedded_blob_code_hash_offset(), 8);
  assert_eq!(m::isolate_hash_offset(), 16);
  assert_eq!(m::layout_description_table_offset(), 24);
  assert_eq!(m::FIRST_HASHED_DATA_OFFSET, m::isolate_hash_offset()); // =16
  assert_eq!(m::RAW_CODE_OFFSET, 0);
  // 表容量参数化链 (取一具体 kTableSize 验证组合)
  let ts: u32 = 137;
  assert_eq!(
    m::layout_description_table_size(ts),
    layout_description::SIZE * ts
  );
  assert_eq!(
    m::builtin_lookup_entry_table_offset(ts),
    m::layout_description_table_offset() + m::layout_description_table_size(ts)
  );
  assert_eq!(
    m::builtin_lookup_entry_table_size(ts),
    builtin_lookup_entry::SIZE * ts
  );
  assert_eq!(
    m::fixed_data_size(ts),
    m::builtin_lookup_entry_table_offset(ts) + m::builtin_lookup_entry_table_size(ts)
  );
  assert_eq!(m::raw_metadata_offset(ts), m::fixed_data_size(ts));
  // 表项尺寸 1:1 (LayoutDescription 3×u32 / BuiltinLookupEntry 2×u32)
  assert_eq!(layout_description::SIZE, 12);
  assert_eq!(builtin_lookup_entry::SIZE, 8);
  OK
}

/// 校验和 (无 zlib / Fletcher-32 分支) 独立模型对照 + 已知向量, 锁模算术与不回绕。
/// 注: 目标构建若启用 zlib 则走 adler32 (见缺陷台账), 本用例仅锁已落分支的内部一致性。
#[test]
fn test_checksum_fletcher32_model() -> Void {
  // 已知向量
  assert_eq!(checksum(&[0x42]), 0x0042_0042);
  assert_eq!(checksum(&[0xFF, 0x01]), 0x01FF_0100);
  assert_eq!(checksum(&[]), 0);

  // 独立 Fletcher-32 参考实现对照 (镜像原版 snapshot-utils.cc #else 分支), 覆盖大输入回绕边界
  let f32 = |payload: &[u8]| -> u32 {
    let mut s1: u32 = 0;
    let mut s2: u32 = 0;
    for &b in payload {
      s1 = (s1 + u32::from(b)) % 65535;
      s2 = (s2 + s1) % 65535;
    }
    (s2 << 16) | s1
  };

  // 递增图案 + 全 0xFF 饱和图案 (逼近 65535 回绕) 两路对照
  let ramp: Vec<u8> = (0..2048u32).map(|i| (i & 0xFF) as u8).collect();
  assert_eq!(checksum(&ramp), f32(&ramp));
  let saturated = vec![0xFFu8; 400];
  assert_eq!(checksum(&saturated), f32(&saturated));

  // 单字节翻转敏感性
  let a = ramp.clone();
  let mut b = ramp.clone();
  b[777] ^= 0x80;
  assert_eq!(checksum(&a), f32(&a));
  assert_ne!(checksum(&a), checksum(&b));
  OK
}
