//! 金标准对数测试: `OpProperties` / `NodeBase` 位域数据面 (自前任开发席草稿 test/properties.rs 迁移)。
//! 期望值独立抄自 v8/src/maglev/maglev-ir.h:1163-1373 (OpProperties 位域链/工厂/查询)
//! 与 :2033-2051 (NodeBase BitField64 段位链)。数值未做改动, 仅语法合法化。

// 本文件全部用例即"常量项 == 金标准字面量"的对数断言, 常量折叠正是用例语义本身。

use aok::{OK, Void};
use log::info;
use v8_maglev_ir::*;

#[ctor::ctor(unsafe)]
fn _log_init() {
  log_init::init();
}

// —— 位域布局 (maglev-ir.h:1357-1367 Next 位域链, 链起 kIsCallBit :1357; kUsedSize :1386) ——
#[test]
fn layout() -> Void {
  assert_eq!(IS_CALL_SHIFT, 0);
  assert_eq!(IS_CALL_WIDTH, 1);
  assert_eq!(IS_CALL_MASK, 0x1);
  assert_eq!(ATTACHED_DEOPT_INFO_SHIFT, 1);
  assert_eq!(ATTACHED_DEOPT_INFO_WIDTH, 3);
  assert_eq!(ATTACHED_DEOPT_INFO_MASK, 0xe);
  assert_eq!(CAN_THROW_SHIFT, 4);
  assert_eq!(CAN_THROW_WIDTH, 1);
  assert_eq!(CAN_THROW_MASK, 0x10);
  assert_eq!(CAN_READ_SHIFT, 5);
  assert_eq!(CAN_READ_MASK, 0x20);
  assert_eq!(CAN_WRITE_SHIFT, 6);
  assert_eq!(CAN_WRITE_MASK, 0x40);
  assert_eq!(CAN_ALLOCATE_SHIFT, 7);
  assert_eq!(CAN_ALLOCATE_MASK, 0x80);
  assert_eq!(NOT_IDEMPOTENT_SHIFT, 8);
  assert_eq!(NOT_IDEMPOTENT_MASK, 0x100);
  assert_eq!(VALUE_REPRESENTATION_SHIFT, 9);
  assert_eq!(VALUE_REPRESENTATION_WIDTH, 4);
  assert_eq!(VALUE_REPRESENTATION_MASK, 0x1e00);
  assert_eq!(IS_CONVERSION_SHIFT, 13);
  assert_eq!(IS_CONVERSION_MASK, 0x2000);
  assert_eq!(NEEDS_REGISTER_SNAPSHOT_SHIFT, 14);
  assert_eq!(NEEDS_REGISTER_SNAPSHOT_MASK, 0x4000);
  assert_eq!(PROPERTIES_USED_SIZE, 15);
  assert_eq!(PROPERTIES_SIZE, 16);
  // 原版 :1369-1370 kPureMask = kCanReadBit::kMask | kCanWriteBit::kMask | kCanAllocateBit::kMask
  assert_eq!(
    K_PURE_MASK,
    CAN_READ_MASK | CAN_WRITE_MASK | CAN_ALLOCATE_MASK
  );
  // 原版 :1371-1373 kPureValue = 三段 encode(false) 之或, 恒 0
  assert_eq!(K_PURE_VALUE, 0);

  // 掩码自洽: mask == ((1<<宽)-1)<<位移, 10 段两两不交, 并集恰为低 kUsedSize 位
  let fields: [(u32, u32, u32); 10] = [
    (IS_CALL_SHIFT, IS_CALL_WIDTH, IS_CALL_MASK),
    (
      ATTACHED_DEOPT_INFO_SHIFT,
      ATTACHED_DEOPT_INFO_WIDTH,
      ATTACHED_DEOPT_INFO_MASK,
    ),
    (CAN_THROW_SHIFT, CAN_THROW_WIDTH, CAN_THROW_MASK),
    (CAN_READ_SHIFT, CAN_READ_WIDTH, CAN_READ_MASK),
    (CAN_WRITE_SHIFT, CAN_WRITE_WIDTH, CAN_WRITE_MASK),
    (CAN_ALLOCATE_SHIFT, CAN_ALLOCATE_WIDTH, CAN_ALLOCATE_MASK),
    (
      NOT_IDEMPOTENT_SHIFT,
      NOT_IDEMPOTENT_WIDTH,
      NOT_IDEMPOTENT_MASK,
    ),
    (
      VALUE_REPRESENTATION_SHIFT,
      VALUE_REPRESENTATION_WIDTH,
      VALUE_REPRESENTATION_MASK,
    ),
    (IS_CONVERSION_SHIFT, IS_CONVERSION_WIDTH, IS_CONVERSION_MASK),
    (
      NEEDS_REGISTER_SNAPSHOT_SHIFT,
      NEEDS_REGISTER_SNAPSHOT_WIDTH,
      NEEDS_REGISTER_SNAPSHOT_MASK,
    ),
  ];
  let mut union = 0u32;
  for (shift, width, mask) in fields {
    assert_eq!(mask, ((1u32 << width) - 1) << shift);
    assert_eq!(union & mask, 0);
    union |= mask;
  }
  assert_eq!(union, (1u32 << PROPERTIES_USED_SIZE) - 1);
  OK
}

// —— 枚举编码值 ——
#[test]
fn enums() -> Void {
  // 原版 `enum class AttachedDeoptInfo` (maglev-ir.h:1350-1356), kNone 起连续分配
  assert_eq!(AttachedDeoptInfo::None as u32, 0);
  assert_eq!(AttachedDeoptInfo::Eager as u32, 1);
  assert_eq!(AttachedDeoptInfo::Lazy as u32, 2);
  assert_eq!(AttachedDeoptInfo::EagerAndLazy as u32, 3);
  assert_eq!(AttachedDeoptInfo::Checkpoint as u32, 4);
  // 原版 `enum class ValueRepresentation : uint8_t` (maglev-ir.h:921-930)
  assert_eq!(REPR_TAGGED, 0);
  assert_eq!(REPR_INT32, 1);
  assert_eq!(REPR_UINT32, 2);
  assert_eq!(REPR_FLOAT64, 3);
  assert_eq!(REPR_HOLEY_FLOAT64, 4);
  assert_eq!(REPR_INT_PTR, 5);
  assert_eq!(REPR_RAW_PTR, 6);
  assert_eq!(REPR_NONE, 7);
  // 草稿原式 `REPR_NONE+1 == 1<<WIDTH` 与锚不符: 原版 :921-930 共 8 个 repr (kTagged..kNone),
  // 位段宽 4 仅约束容量 8 ≤ 1<<4; 按锚校正为计数恒等 + 容量不等式。
  assert_eq!(REPR_NONE as u32 + 1, 8);
  assert!((REPR_NONE as u32 + 1) <= 1 << VALUE_REPRESENTATION_WIDTH);
  OK
}

/// 原版 `WithNewValueRepresentation` 中 repr 段的移位掩码 (maglev-ir.h:1340-1343)。
const FLOAT64_MASK_OF_REPR: u32 = (REPR_FLOAT64 as u32) << VALUE_REPRESENTATION_SHIFT;

// —— 工厂金标准数值 (25 外露 + 1 私有内联 + 1 带参; maglev-ir.h:1240-1335 + 私有 :1379-1381) ——
#[test]
fn factories() -> Void {
  let cases: [(&str, u32, OpProperties); 25] = [
    ("Pure", 0x0, OpProperties::PURE),
    ("Call", 0x1, OpProperties::CALL),
    ("EagerDeopt", 0x2, OpProperties::EAGER_DEOPT),
    ("LazyDeopt", 0x4, OpProperties::LAZY_DEOPT),
    ("DeoptCheckpoint", 0x8, OpProperties::DEOPT_CHECKPOINT),
    // CanThrow = kCanThrowBit | LazyDeopt | CanAllocate (:1256-1258)
    ("CanThrow", 0x94, OpProperties::CAN_THROW),
    ("CanRead", 0x20, OpProperties::CAN_READ),
    ("CanWrite", 0x40, OpProperties::CAN_WRITE),
    ("CanAllocate", 0x80, OpProperties::CAN_ALLOCATE),
    ("NotIdempotent", 0x100, OpProperties::NOT_IDEMPOTENT),
    ("TaggedValue", 0x0, OpProperties::TAGGED_VALUE),
    ("ExternalReference", 0xa00, OpProperties::EXTERNAL_REFERENCE),
    ("Int32", 0x200, OpProperties::INT32),
    ("Uint32", 0x400, OpProperties::UINT32),
    ("Float64", 0x600, OpProperties::FLOAT64),
    ("HoleyFloat64", 0x800, OpProperties::HOLEY_FLOAT64),
    ("IntPtr", 0xa00, OpProperties::INT_PTR),
    ("RawPtr", 0xc00, OpProperties::RAW_PTR),
    ("TrustedPointer", 0x0, OpProperties::TRUSTED_POINTER),
    ("ConversionNode", 0x2000, OpProperties::CONVERSION_NODE),
    // CanCallUserCode = AnySideEffects | LazyDeopt | CanThrow (:1315-1317)
    ("CanCallUserCode", 0xf4, OpProperties::CAN_CALL_USER_CODE),
    // GenericRuntimeOrBuiltinCall = Call | CanCallUserCode | NotIdempotent (:1323-1326)
    (
      "GenericRuntimeOrBuiltinCall",
      0x1f5,
      OpProperties::GENERIC_RUNTIME_OR_BUILTIN_CALL,
    ),
    ("JSCall", 0xf5, OpProperties::JSCALL),
    ("AnySideEffects", 0xe0, OpProperties::ANY_SIDE_EFFECTS),
    // DeferredCall = NeedsRegisterSnapshot (原版私有工厂 maglev-ir.h:1379-1381; :1327-1329 系 AnySideEffects)
    ("DeferredCall", 0x4000, OpProperties::DEFERRED_CALL),
  ];
  for (name, want, got) in cases {
    assert_eq!(got.to_u32(), want, "工厂 {} 值不符", name);
  }
  // ForValueRepresentation(repr) 与带 repr 的工厂一致 (:1308-1311)
  assert_eq!(
    OpProperties::for_value_representation(REPR_INT32),
    OpProperties::INT32
  );
  assert_eq!(
    OpProperties::for_value_representation(REPR_RAW_PTR),
    OpProperties::RAW_PTR
  );
  assert_eq!(
    OpProperties::for_value_representation(REPR_TAGGED),
    OpProperties::TAGGED_VALUE
  );
  info!("OK properties::factories 25 工厂金标准全过");
  OK
}

// —— 工厂间组合等式 (原版 `|` 展开 :1256-1342) ——
#[test]
fn combinations() -> Void {
  assert_eq!(
    OpProperties::ANY_SIDE_EFFECTS,
    OpProperties::CAN_READ
      .or(OpProperties::CAN_WRITE)
      .or(OpProperties::CAN_ALLOCATE),
  );
  assert_eq!(
    OpProperties::CAN_THROW,
    OpProperties::from_bits(CAN_THROW_MASK)
      .or(OpProperties::LAZY_DEOPT)
      .or(OpProperties::CAN_ALLOCATE),
  );
  assert_eq!(
    OpProperties::CAN_CALL_USER_CODE,
    OpProperties::ANY_SIDE_EFFECTS
      .or(OpProperties::LAZY_DEOPT)
      .or(OpProperties::CAN_THROW),
  );
  assert_eq!(
    OpProperties::JSCALL,
    OpProperties::CALL.or(OpProperties::CAN_CALL_USER_CODE),
  );
  assert_eq!(
    OpProperties::GENERIC_RUNTIME_OR_BUILTIN_CALL,
    OpProperties::CALL
      .or(OpProperties::CAN_CALL_USER_CODE)
      .or(OpProperties::NOT_IDEMPOTENT),
  );
  assert_eq!(OpProperties::EXTERNAL_REFERENCE, OpProperties::INT_PTR,);
  // WithNewValueRepresentation: 只改 repr 段 (:1340-1343)
  assert_eq!(
    OpProperties::CALL.with_new_value_representation(REPR_FLOAT64),
    OpProperties::from_bits(OpProperties::CALL.to_u32() | FLOAT64_MASK_OF_REPR),
  );
  assert_eq!(
    OpProperties::PURE.with_new_value_representation(REPR_NONE),
    OpProperties::for_value_representation(REPR_NONE),
  );
  // WithoutDeopt: 只清 attached_deopt_info 段 (:1344-1347)
  assert_eq!(
    OpProperties::CAN_THROW.without_deopt().to_u32(),
    CAN_THROW_MASK | CAN_ALLOCATE_MASK,
  );
  assert_eq!(OpProperties::PURE.without_deopt(), OpProperties::PURE);
  OK
}

// —— 查询谓词 (20 个, 逐项对照原版 C++ 语义 maglev-ir.h:1163-1228) ——
#[test]
fn queries() -> Void {
  let pure = OpProperties::PURE;
  assert!(pure.is_pure());
  assert!(!pure.is_call());
  assert!(!pure.is_any_call());
  assert!(!pure.can_throw());
  assert!(!pure.can_read());
  assert!(!pure.can_write());
  assert!(!pure.can_allocate());
  assert!(!pure.not_idempotent());
  assert_eq!(pure.value_representation(), REPR_TAGGED);
  assert!(pure.is_tagged());
  assert!(!pure.is_conversion());
  assert!(!pure.needs_register_snapshot());
  assert!(!pure.is_deferred_call());
  assert!(!pure.can_deopt());
  assert!(!pure.has_eager_deopt_info());
  assert!(!pure.is_required_when_unused());
  assert!(pure.can_participate_in_cse());

  let call = OpProperties::CALL;
  assert!(call.is_call());
  assert!(call.is_any_call());
  assert!(!call.is_deferred_call());
  // 草稿原式 `!call.is_pure()` 与锚不符: 原版 IsPure (:1220) 只看 kPureMask = read|write|allocate
  // (:1369-1370), 不含 is_call; Call()=0x1 三副作用位全空 → 按锚为真 (JSCall 才非纯)。
  assert!(call.is_pure());
  assert!(!OpProperties::JSCALL.is_pure());
  assert!(!call.can_throw());
  assert!(call.is_required_when_unused());
  assert!(call.can_participate_in_cse());

  // is_call()==false 但 is_deferred_call()==true → is_any_call()==true (:1169; :1170-1174 系 can_eager_deopt)
  let deferred = OpProperties::DEFERRED_CALL;
  assert!(!deferred.is_call());
  assert!(deferred.is_deferred_call());
  assert!(deferred.is_any_call());

  assert!(OpProperties::EAGER_DEOPT.can_eager_deopt());
  assert!(!OpProperties::EAGER_DEOPT.can_lazy_deopt());
  assert!(OpProperties::EAGER_DEOPT.has_eager_deopt_info());
  assert!(OpProperties::LAZY_DEOPT.can_lazy_deopt());
  assert!(!OpProperties::LAZY_DEOPT.has_eager_deopt_info());
  assert!(OpProperties::DEOPT_CHECKPOINT.is_deopt_checkpoint());
  assert!(OpProperties::DEOPT_CHECKPOINT.has_eager_deopt_info());
  assert!(!OpProperties::DEOPT_CHECKPOINT.can_deopt());
  // 草稿原式取 ATTACHED_DEOPT_INFO_MASK(=0xe, 解码 7 非法) 与注释 EagerAndLazy 自相矛盾;
  // 按锚校正: 原版 AttachedDeoptInfo::kEagerAndLazy = 3, encode 后 = 3 << 1 = 6
  // (maglev-ir.h:1350-1356 枚举体 + :1358 kAttachedDeoptInfoBits)。
  let both =
    OpProperties::from_bits((AttachedDeoptInfo::EagerAndLazy as u32) << ATTACHED_DEOPT_INFO_SHIFT);
  assert!(both.can_eager_deopt());
  assert!(both.can_lazy_deopt());
  assert!(both.can_deopt());
  assert!(!both.is_deopt_checkpoint());

  // 原版 :1190-1192 `can_throw() = kCanThrowBit::decode && can_lazy_deopt()` (:1187-1189 系 can_deopt)
  assert!(OpProperties::CAN_THROW.can_throw());
  assert!(!OpProperties::from_bits(CAN_THROW_MASK).can_throw());

  assert!(OpProperties::CAN_READ.can_read());
  assert!(!OpProperties::CAN_READ.can_write());
  assert!(OpProperties::CAN_WRITE.can_write());
  assert!(OpProperties::CAN_ALLOCATE.can_allocate());
  assert!(OpProperties::NOT_IDEMPOTENT.not_idempotent());
  assert!(!OpProperties::NOT_IDEMPOTENT.can_participate_in_cse());
  assert!(!OpProperties::CAN_WRITE.can_participate_in_cse());
  assert!(OpProperties::CONVERSION_NODE.is_conversion());
  assert_eq!(
    OpProperties::HOLEY_FLOAT64.value_representation(),
    REPR_HOLEY_FLOAT64,
  );
  assert!(!OpProperties::HOLEY_FLOAT64.is_tagged());
  assert!(OpProperties::TRUSTED_POINTER.is_tagged());

  // is_required_when_unused 的 conversion 分支不统计 is_any_call (:1221-1226)
  let conv_call = OpProperties::CONVERSION_NODE.or(OpProperties::CALL);
  assert!(!conv_call.is_required_when_unused());
  assert!(!OpProperties::CAN_READ.is_required_when_unused());
  assert!(OpProperties::CAN_WRITE.is_required_when_unused());
  OK
}

// —— NodeBase 位域 (encode/decode/update 恒等式; 段位锚 maglev-ir.h:2033-2051) ——
#[test]
fn node_base() -> Void {
  assert_eq!(NODE_OPCODE_SHIFT, 0);
  assert_eq!(NODE_OPCODE_WIDTH, 16);
  assert_eq!(NODE_INPUT_COUNT_SHIFT, 16);
  assert_eq!(NODE_INPUT_COUNT_WIDTH, 16);
  assert_eq!(NODE_OP_PROPERTIES_SHIFT, 32);
  assert_eq!(NODE_OP_PROPERTIES_WIDTH, 16);
  assert_eq!(NODE_NUM_TEMPORARIES_NEEDED_SHIFT, 48);
  assert_eq!(NODE_NUM_TEMPORARIES_NEEDED_WIDTH, 2);
  assert_eq!(NODE_NUM_DOUBLE_TEMPORARIES_NEEDED_SHIFT, 50);
  assert_eq!(NODE_NUM_DOUBLE_TEMPORARIES_NEEDED_WIDTH, 1);
  assert_eq!(NODE_RESERVED_FIELDS_SHIFT, 51);
  assert_eq!(NODE_RESERVED_FIELDS_WIDTH, 2);
  // 段间紧邻, 无空隙 (原版 Next<...> 前驱链)
  assert_eq!(
    NODE_OPCODE_SHIFT + NODE_OPCODE_WIDTH,
    NODE_INPUT_COUNT_SHIFT
  );
  assert_eq!(
    NODE_INPUT_COUNT_SHIFT + NODE_INPUT_COUNT_WIDTH,
    NODE_OP_PROPERTIES_SHIFT,
  );
  assert_eq!(
    NODE_OP_PROPERTIES_SHIFT + NODE_OP_PROPERTIES_WIDTH,
    NODE_NUM_TEMPORARIES_NEEDED_SHIFT,
  );
  assert_eq!(
    NODE_NUM_TEMPORARIES_NEEDED_SHIFT + NODE_NUM_TEMPORARIES_NEEDED_WIDTH,
    NODE_NUM_DOUBLE_TEMPORARIES_NEEDED_SHIFT,
  );
  assert_eq!(
    NODE_NUM_DOUBLE_TEMPORARIES_NEEDED_SHIFT + NODE_NUM_DOUBLE_TEMPORARIES_NEEDED_WIDTH,
    NODE_RESERVED_FIELDS_SHIFT,
  );
  // 原版 :2051 kMaxInputs = InputCountField::kMax
  assert_eq!(MAX_INPUTS, (1u64 << NODE_INPUT_COUNT_WIDTH) - 1);
  assert_eq!(MAX_INPUTS, 65535);

  let cases: [(u16, u64, OpProperties, u64, u64, u64); 6] = [
    (0, 0, OpProperties::PURE, 0, 0, 0),
    (
      12,
      5,
      OpProperties::CAN_THROW,
      2,
      1,
      12 | (5 << 16) | ((OpProperties::CAN_THROW.to_u32() as u64) << 32) | (2 << 48) | (1 << 50),
    ),
    (
      LAST_OPCODE,
      0,
      OpProperties::JSCALL,
      0,
      0,
      (LAST_OPCODE as u64) | ((OpProperties::JSCALL.to_u32() as u64) << 32),
    ),
    (
      7,
      MAX_INPUTS,
      OpProperties::GENERIC_RUNTIME_OR_BUILTIN_CALL,
      3,
      0,
      7 | (MAX_INPUTS << 16)
        | ((OpProperties::GENERIC_RUNTIME_OR_BUILTIN_CALL.to_u32() as u64) << 32)
        | (3 << 48),
    ),
    (
      65535,
      1,
      OpProperties::RAW_PTR,
      0,
      1,
      65535 | (1 << 16) | ((OpProperties::RAW_PTR.to_u32() as u64) << 32) | (1 << 50),
    ),
    (0, 3, OpProperties::PURE, 0, 0, 3 << 16),
  ];
  for (opcode, input_count, properties, num_temp, num_double_temp, bits) in cases {
    let enc = encode_node_bitfield(opcode, input_count, properties, num_temp, num_double_temp);
    assert_eq!(enc, bits);
    assert_eq!(decode_opcode(enc), opcode);
    assert_eq!(decode_input_count(enc), input_count);
    assert_eq!(decode_properties(enc), properties);
    assert_eq!(decode_num_temporaries(enc), num_temp);
    assert_eq!(decode_num_double_temporaries(enc), num_double_temp);
    // update 只改 properties 段 (原版 :2036-2037 OpPropertiesField::update)
    let upd = update_properties(enc, OpProperties::ANY_SIDE_EFFECTS);
    assert_eq!(decode_properties(upd), OpProperties::ANY_SIDE_EFFECTS);
    assert_eq!(
      upd & !NODE_OP_PROPERTIES_MASK,
      enc & !NODE_OP_PROPERTIES_MASK
    );
  }
  info!("OK properties.rs OpProperties/NodeBase 对数测试通过");
  OK
}

// —— r4 D1 回归: 构造入口 16 位截断 (成员 uint16_t, maglev-ir.h:1383; 构造 :1337) ——
#[test]
fn from_bits_truncates_to_16() -> Void {
  // C++ OpProperties(uint32_t) 构造即截: 1<<16 落 uint16_t 成员 → 0
  assert_eq!(OpProperties::from_bits(1 << 16).to_u32(), 0);
  assert_eq!(OpProperties::from_bits(0x1_ffff).to_u32(), 0xffff);
  // for_value_representation 超枚举域 (repr=64 → 1<<15 仍在 16 位内; repr=128 → 1<<16 截为 0)
  assert_eq!(
    OpProperties::from_bits(128 << VALUE_REPRESENTATION_SHIFT).to_u32(),
    0
  );
  // update 路径 (with_new_value_representation) 与 C++ release 一致:
  // encode 枚举支仅 DCHECK 不掩宽 (base/bit-field.h:84-102), 但写入经 uint16_t 成员同样截 16 位。
  // new_repr=16 时 16<<9 = 0x2000 在 16 位内 → 不截断, 与 C++ 相同置 is_conversion 位。
  let patched = OpProperties::PURE.with_new_value_representation(16);
  assert_eq!(patched.to_u32(), 0x2000);
  assert!(patched.is_conversion());
  OK
}
