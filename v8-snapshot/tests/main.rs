use aok::{OK, Void};
use v8_snapshot::{
  CODE_ALIGNMENT_ARM64, CODE_ALIGNMENT_X64, DataDirective, EmbeddedTargetArch, EmbeddedTargetOs,
  FIXED_RAW_DATA_COUNT, HOT_OBJECT_COUNT, METADATA_ALIGNMENT, ROOT_ARRAY_CONSTANTS_COUNT,
  SerializationTag, SerializerFlags, SnapshotByteSink, SnapshotByteSource, SnapshotSpace,
  SpecialValueType, bitset, checksum, encode, encoded_external_reference, encoded_tagged, framing,
  get_uncompressed_size, serializer_reference_bits, snapshot_blob_layout, snapshot_data_layout,
};

#[ctor::ctor(unsafe)]
fn _log_init() {
  log_init::init();
}

#[test]
fn test_checksum() -> Void {
  // 空载荷校验和为 0
  assert_eq!(checksum(&[]), 0);

  // 单字节校验和
  let single = [0x42_u8];
  let cs_single = checksum(&single);
  assert_eq!(cs_single & 0xFFFF, 0x42);
  assert_eq!((cs_single >> 16) & 0xFFFF, 0x42);

  // 确定性验证
  let payload = b"V8 Snapshot Fletcher-32 Checksum Test Payload";
  let cs1 = checksum(payload);
  let cs2 = checksum(payload);
  assert_eq!(cs1, cs2);
  assert_ne!(cs1, 0);

  // 差异敏感性
  let mut payload_altered = *payload;
  payload_altered[0] ^= 1;
  assert_ne!(checksum(&payload_altered), cs1);

  OK
}

#[test]
fn test_byte_source_sink_roundtrip() -> Void {
  let mut sink = SnapshotByteSink::new();

  // 1. 基础单字节与批量填充
  sink.put(0xAA, "header");
  sink.put_n(3, 0x55, "padding");

  // 2. 游程 uint30 (边界分阶: 1字节, 2字节, 3字节, 4字节)
  let test_u30 = [
    0u32,
    1,
    63,
    64,
    16383,
    16384,
    (1 << 22) - 1,
    1 << 22,
    (1 << 30) - 1,
  ];
  for &val in &test_u30 {
    sink.put_uint30(val, "u30");
  }

  // 3. 原生 uint32
  sink.put_uint32(0xDEAD_BEEF, "magic");
  sink.put_raw(b"RAWBYTES", "raw");

  let encoded = sink.data();
  let mut source = SnapshotByteSource::new(encoded);

  // 验证基础单字节
  assert!(source.has_more());
  assert_eq!(source.peek(), 0xAA);
  assert_eq!(source.get(), 0xAA);

  let mut pad = [0u8; 3];
  source.copy_raw(&mut pad);
  assert_eq!(pad, [0x55, 0x55, 0x55]);

  // 验证 uint30
  for &expected in &test_u30 {
    let actual = source.get_uint30();
    assert_eq!(actual, expected);
  }

  // 验证 uint32 与 raw
  assert_eq!(source.get_uint32(), 0xDEAD_BEEF);
  let mut raw_buf = [0u8; 8];
  source.copy_raw(&mut raw_buf);
  assert_eq!(&raw_buf, b"RAWBYTES");

  assert!(!source.has_more());

  OK
}

#[test]
fn test_compression_framing() -> Void {
  let uncompressed_len = 1048576_u32; // 1MB
  let framed = framing::write_length_prefix(uncompressed_len);
  assert_eq!(framed.len(), 4);
  assert_eq!(get_uncompressed_size(&framed), uncompressed_len);

  // 带压缩载荷的切片拆分
  let mut complete_blob = framed;
  complete_blob.extend_from_slice(b"fake compressed stream body");

  let (len, body) = framing::split(&complete_blob).unwrap();
  assert_eq!(len, uncompressed_len);
  assert_eq!(body, b"fake compressed stream body");

  // 头部长度不足时安全返回 None
  assert!(framing::split(&[1, 2, 3]).is_none());

  OK
}

#[test]
fn test_blob_and_data_layout() -> Void {
  // 快照 blob 头布局不变量校验
  assert_eq!(snapshot_blob_layout::NUMBER_OF_CONTEXTS_OFFSET, 0);
  assert_eq!(snapshot_blob_layout::REHASHABILITY_OFFSET, 4);
  assert_eq!(snapshot_blob_layout::CHECKSUM_OFFSET, 8);
  assert_eq!(snapshot_blob_layout::READ_ONLY_SNAPSHOT_CHECKSUM_OFFSET, 12);
  assert_eq!(snapshot_blob_layout::VERSION_STRING_OFFSET, 16);
  assert_eq!(snapshot_blob_layout::VERSION_STRING_LENGTH, 64);
  assert_eq!(snapshot_blob_layout::STARTUP_OFFSET_OFFSET, 80);
  assert_eq!(snapshot_blob_layout::SHARED_HEAP_OFFSET_OFFSET, 84);
  assert_eq!(snapshot_blob_layout::FIRST_CONTEXT_OFFSET_OFFSET, 88);

  // 8 字节对齐 read_only_snapshot_offset
  assert_eq!(snapshot_blob_layout::read_only_snapshot_offset(0), 88);
  assert_eq!(snapshot_blob_layout::read_only_snapshot_offset(1), 96);
  assert_eq!(snapshot_blob_layout::read_only_snapshot_offset(2), 96);
  assert_eq!(snapshot_blob_layout::read_only_snapshot_offset(3), 104);

  // 上下文偏移索引
  assert_eq!(snapshot_blob_layout::context_snapshot_offset_offset(0), 88);
  assert_eq!(snapshot_blob_layout::context_snapshot_offset_offset(1), 92);
  assert_eq!(snapshot_blob_layout::context_snapshot_offset_offset(2), 96);

  // 数据 blob 布局
  assert_eq!(snapshot_data_layout::UINT32_SIZE, 4);
  assert_eq!(snapshot_data_layout::PAYLOAD_LENGTH_OFFSET, 4);
  assert_eq!(snapshot_data_layout::HEADER_SIZE, 8);

  OK
}

#[test]
fn test_serializer_flags() -> Void {
  let flags = SerializerFlags::DEFAULT;
  assert_eq!(flags.bits(), 0);
  assert!(!flags.contains(SerializerFlags::ALLOW_UNKNOWN_EXTERNAL_REFERENCES_FOR_TESTING));

  let flags_with = flags.with(SerializerFlags::ALLOW_UNKNOWN_EXTERNAL_REFERENCES_FOR_TESTING);
  assert!(flags_with.contains(SerializerFlags::ALLOW_UNKNOWN_EXTERNAL_REFERENCES_FOR_TESTING));

  let flags_without =
    flags_with.without(SerializerFlags::ALLOW_UNKNOWN_EXTERNAL_REFERENCES_FOR_TESTING);
  assert!(!flags_without.contains(SerializerFlags::ALLOW_UNKNOWN_EXTERNAL_REFERENCES_FOR_TESTING));

  OK
}

#[test]
fn test_serialization_tags_roundtrip() -> Void {
  // 验证已知 Tag 码位双向往返
  let tags = [
    SerializationTag::True,
    SerializationTag::False,
    SerializationTag::Null,
    SerializationTag::Undefined,
    SerializationTag::Date,
    SerializationTag::RegExp,
    SerializationTag::ArrayBuffer,
    SerializationTag::Version,
  ];

  for tag in tags {
    let byte = tag as u8;
    assert_eq!(SerializationTag::from_byte(byte), Some(tag));
  }

  // 非法码位拒绝解析
  assert_eq!(SerializationTag::from_byte(1), None);
  assert_eq!(SerializationTag::from_byte(2), None);
  assert_eq!(SerializationTag::from_byte(0x7F), None);

  OK
}

#[test]
fn test_embedded_data_alignment() -> Void {
  const {
    assert!(CODE_ALIGNMENT_X64.is_power_of_two());
    assert!(CODE_ALIGNMENT_ARM64.is_power_of_two());
    assert!(METADATA_ALIGNMENT.is_power_of_two());
  };

  // 数据指令尺寸比例一致性
  assert_eq!(DataDirective::Byte.size() * 4, DataDirective::Long.size());
  assert_eq!(DataDirective::Long.size() * 2, DataDirective::Quad.size());
  assert_eq!(DataDirective::Quad.size() * 2, DataDirective::Octa.size());

  // 目标架构与操作系统枚举互斥
  assert_ne!(
    EmbeddedTargetArch::X64 as u8,
    EmbeddedTargetArch::Arm64 as u8
  );
  assert_ne!(EmbeddedTargetOs::Mac as u8, EmbeddedTargetOs::Win as u8);

  OK
}

#[test]
fn test_serializer_bytecode_segment_encode_decode() -> Void {
  // SpaceEncoder<kNewObject>: 0x00..0x03, 逐空间往返
  for space in 0..encode::NUMBER_OF_SNAPSHOT_SPACES as u8 {
    let bc = encode::encode_new_object(space);
    assert_eq!(bc, space);
    assert_eq!(encode::decode_new_object(bc), space);
  }
  // SpaceEncoder<kBackref>: 0x04..0x07
  assert_eq!(encode::encode_backref(3), 0x07);

  // FixedRawDataWithSize: 段基址 0x60, 长度 [1,32] 双射
  for size in 1..=FIXED_RAW_DATA_COUNT as i32 {
    let bc = encode::encode_fixed_raw_data(size);
    assert_eq!(bc, 0x60 + (size - 1) as u8);
    assert_eq!(encode::decode_fixed_raw_data(bc), size);
  }

  // FixedRepeatRootWithCount: 段基址 0x80, count [2,17]
  for count in 2..=17 {
    let bc = encode::encode_fixed_repeat_root(count);
    assert_eq!(encode::decode_fixed_repeat_root(bc), count);
  }

  // HotObject: 段基址 0x90, index [0,7]
  for idx in 0..HOT_OBJECT_COUNT as i32 {
    assert_eq!(encode::encode_hot_object(idx), 0x90 + idx as u8);
  }

  // RootArrayConstant 段: 编码到 [0x40, 0x40+kRootArrayConstantsCount)
  assert_eq!(encode::encode_root_array_constant(0), 0x40);
  assert_eq!(
    encode::encode_root_array_constant(ROOT_ARRAY_CONSTANTS_COUNT as i32 - 1),
    0x40 + 31
  );

  // VariableRepeatRootCount 往返
  let count = 100;
  let enc = encode::encode_variable_repeat_root(count);
  assert_eq!(encode::decode_variable_repeat_root(enc), count);

  OK
}

#[test]
fn test_snapshot_space_and_reference_bits() -> Void {
  // SnapshotSpace 值连续且索引可用 (原版 enum class : uint8_t 直接下标)
  for (i, space) in SnapshotSpace::ALL.iter().enumerate() {
    assert_eq!(*space as u8, i as u8);
    assert_eq!(SnapshotSpace::from_repr(*space as u8), Some(*space));
  }
  assert_eq!(SnapshotSpace::from_repr(4), None);

  // SpecialValueType 枚举互斥性
  assert_ne!(
    SpecialValueType::BuiltinReference as u8,
    SpecialValueType::OffHeapBackingStore as u8
  );

  // SerializerReference 位打包: TypeBits(2) + ValueBits(30) 无重叠, 往返保真
  let t: u8 = 0b11;
  let v: u32 = (1 << 30) - 1;
  let packed =
    serializer_reference_bits::encode_type(t) | serializer_reference_bits::encode_value(v);
  assert_eq!(serializer_reference_bits::decode_type(packed), t);
  assert_eq!(serializer_reference_bits::decode_value(packed), v);
  // 类型与值掩码互斥
  assert_eq!(
    serializer_reference_bits::TYPE_MASK & serializer_reference_bits::VALUE_MASK,
    0
  );

  OK
}

#[test]
fn test_read_only_bits_and_bitset() -> Void {
  // ro::BitSet 索引映射 (uint8 chunk)
  assert_eq!(bitset::chunk_index(0), 0);
  assert_eq!(bitset::chunk_index(8), 1);
  assert_eq!(bitset::bit_index(9), 1);
  assert_eq!(bitset::bit_mask(9), 1 << 1);

  // EncodedTagged 位域分割: page_index + offset 覆盖全 32 位
  assert_eq!(
    encoded_tagged::OFFSET_BITS + encoded_tagged::PAGE_INDEX_BITS,
    encoded_tagged::SIZE * 8
  );
  assert_eq!(
    encoded_tagged::OFFSET_MASK & encoded_tagged::PAGE_INDEX_MASK,
    0
  );

  // EncodedExternalReference tag(8)+is_api(1)+index(23) 无重叠, 覆盖 32 位
  let masks = [
    encoded_external_reference::TAG_MASK,
    encoded_external_reference::IS_API_REFERENCE_MASK,
    encoded_external_reference::INDEX_MASK,
  ];
  let union = masks[0] | masks[1] | masks[2];
  assert_eq!(union, u32::MAX);
  assert_eq!(masks[0] & masks[1], 0);
  assert_eq!(masks[1] & masks[2], 0);

  OK
}
