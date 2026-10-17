// 由 tool/src/objects/string.tq.js 从 v8/src/objects/string.tq + 锚 src/objects/string.h 生成; 修订就地改, 复跑覆盖
// String 类族: 表示标签/实例类型位段/层级规则面

//! String 的 Torque 侧规则面。
//!
//! instance_type 低 7 位 (@reserveBitsInInstanceType(7)):
//! - bit0-2: representation (StringRepresentationTag)
//! - bit3: is_one_byte; bit4: is_uncached; bit5: is_shared; bit6: is_not_internalized

// ── StringRepresentationTag (string.tq:54-60, 顺序值) ──────────────────
/// 字符串表示法标签 (instance_type bit0-2)。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u32)]
pub enum StringRepresentationTag {
  /// 顺序字符串。
  SeqStringTag = 0,
  /// 拼接字符串 (first/second)。
  ConsStringTag = 1,
  /// 外部字符串 (resource/resource_data 指针)。
  ExternalStringTag = 2,
  /// 切片字符串 (parent/offset)。
  SlicedStringTag = 3,
  /// 薄字符串 (actual 指向内部化串)。
  ThinStringTag = 4,
}

/// 表示法标签位宽。
pub const K_STRING_REPRESENTATION_BITS: u32 = 3;

// ── StringInstanceType 位段 (string.tq:62-68) ─────────────────────────
/// is_one_byte 移位 (bit3)。
pub const IS_ONE_BYTE_SHIFT: u32 = 3;
/// is_uncached 移位 (bit4; ExternalString 无 resource_data)。
pub const IS_UNCACHED_SHIFT: u32 = 4;
/// is_shared 移位 (bit5)。
pub const IS_SHARED_SHIFT: u32 = 5;
/// is_not_internalized 移位 (bit6)。
pub const IS_NOT_INTERNALIZED_SHIFT: u32 = 6;
/// 位段总宽 (与 @reserveBitsInInstanceType(7) 对齐)。
pub const K_STRING_INSTANCE_TYPE_RESERVED_BITS: u32 = 7;
/// 掩码: 7 位全部。
pub const K_STRING_INSTANCE_TYPE_MASK: u16 = 0b0111_1111;

// ── 类族层级 (string.tq extern class 清单, 11 个) ─────────
/// String 类族层级表 (名, 直接基类)。
pub const STRING_CLASS_HIERARCHY: [(&str, &str); 11] = [
  ("String", "Name"),
  ("ConsString", "String"),
  ("ExternalString", "String"),
  ("ExternalOneByteString", "ExternalString"),
  ("ExternalTwoByteString", "ExternalString"),
  ("InternalizedString", "String"),
  ("SeqString", "String"),
  ("SeqOneByteString", "SeqString"),
  ("SeqTwoByteString", "SeqString"),
  ("SlicedString", "String"),
  ("ThinString", "String"),
];

// ── 字段布局规则 (string.tq 各 class) ─────────────────────────────────
/// String 专属字段: length (int32)。
pub const STRING_FIELDS: [&str; 1] = ["length"];
/// ConsString 字段: first, second (second.length == 0 即扁平)。
pub const CONS_STRING_FIELDS: [&str; 2] = ["first", "second"];
/// ExternalString 字段: resource, resource_data (uncached 时缺后者)。
pub const EXTERNAL_STRING_FIELDS: [&str; 2] = ["resource", "resource_data"];
/// SlicedString 字段: parent, offset(Smi)。
pub const SLICED_STRING_FIELDS: [&str; 2] = ["parent", "offset"];
/// ThinString 字段: actual (InternalizedString)。
pub const THIN_STRING_FIELDS: [&str; 1] = ["actual"];
/// SeqOneByteString / SeqTwoByteString 变长负载字段。
pub const SEQ_STRING_CHARS_FIELD: &str = "chars";

// ── 长度上限制度 (string.h:552 / v8-primitive.h:129-130) ──────────────
/// 字符串长度上限 = v8::String::kMaxLength (64 位非 LOWER_LIMITS: (1<<29)-24)。
/// StaticAssertStringLengthFitsSmi (string.tq:360) 静态断言其可入 Smi。
pub const K_MAX_LENGTH: u32 = 536870888;

// —— 提取不变式断言 ——
const _: () = {
  assert!(StringRepresentationTag::SeqStringTag as u32 == 0);
  assert!(StringRepresentationTag::ThinStringTag as u32 == 4);
  assert!(K_STRING_REPRESENTATION_BITS == 3);
  assert!(IS_ONE_BYTE_SHIFT == 3 && IS_UNCACHED_SHIFT == 4 && IS_SHARED_SHIFT == 5 && IS_NOT_INTERNALIZED_SHIFT == 6);
  assert!(K_STRING_INSTANCE_TYPE_RESERVED_BITS == 7);
  assert!(K_STRING_INSTANCE_TYPE_MASK == 0x7F);
  assert!(STRING_CLASS_HIERARCHY.len() == 11);
  assert!(STRING_CLASS_HIERARCHY[0] == ("String", "Name"));
  assert!(K_MAX_LENGTH == (1 << 29) - 24);
  assert!(CONS_STRING_FIELDS.len() == 2 && SLICED_STRING_FIELDS[1] == "offset");
};
