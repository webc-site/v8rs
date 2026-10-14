// 迁移规约:
//   property-details-unittest.cc TEST(PropertyDetailsTest, AsByte):
//     kind×constness×attrs (NoCell) 全组合 to_byte/from_byte 相等
//   property-details-unittest.cc TEST(PropertyDetailsTest, ExceedMaxEnumerationIndex):
//     EXPECT_DEATH — 禁 panic 语义, 跳过 (登记: Rust set_index 以掩码截断 +
//     debug_assert, 非崩溃路径)
//   property-details.h:309-588 构造/取放函数语义自证 (fast/dictionary 双布局,
//     AsSmi 符号扩展, 位段边界)
use aok::{OK, Void};
use v8_objects_core::{
  property_details::{
    DESCRIPTOR_POINTER_MASK, DICTIONARY_STORAGE_MASK, OFFSET_IN_WORDS_MASK,
    PropertyAttributes as Attrs, PropertyCellType as Cell, PropertyConstness as Constness,
    PropertyDetails as PD, PropertyKind as Kind, PropertyLocation as Loc, Representation as Rep,
    generalize_constness, location_is_generalizable_to,
  },
  smi::Smi,
};

#[ctor::ctor(unsafe)]
fn init() {
  log_init::init();
}

/// 规约 (property-details-unittest.cc:53-62 AsByte):
/// NoCell 且字典序号 0 的详情 → to_byte/from_byte 无损。
#[test]
fn property_details_as_byte() -> Void {
  for kind in [Kind::Data, Kind::Accessor] {
    for attrs in 0u8..8 {
      let d = PD::for_global_dictionary(kind, Attrs(attrs), Cell::NO_CELL, 0);
      let from = PD::from_byte(d.to_byte());
      assert_eq!(d, from, "kind={kind:?} attrs={attrs}");
    }
  }
  OK
}

/// 规约 (property-details.h:314-336 构造族):
/// 字典布局 kind/attributes/constness/dictionary_index/cell_type 逐字段回读。
#[test]
fn for_dictionary_layout() -> Void {
  let d = PD::for_dictionary(Kind::Accessor, Attrs::DONT_ENUM, Constness::Const, 42);
  assert_eq!(d.kind(), Kind::Accessor);
  assert_eq!(d.attributes(), Attrs::DONT_ENUM);
  assert_eq!(d.constness(), Constness::Const);
  assert_eq!(d.dictionary_index(), 42);
  assert_eq!(d.cell_type(), Cell::NO_CELL);
  assert_eq!(d.location(), Loc::Field);

  let g = PD::for_global_dictionary(Kind::Data, Attrs::READ_ONLY, Cell::Constant, 7);
  assert_eq!(g.cell_type(), Cell::Constant);
  assert_eq!(g.dictionary_index(), 7);
  // 全局字典详情 Constness 固定 kMutable (原版构造注释)
  assert_eq!(g.constness(), Constness::Mutable);
  OK
}

/// 规约 (property-details.h:338-352 for_fast_field):
/// fast 布局 location/representation/offset_in_words/in_object 逐字段回读。
#[test]
fn for_fast_field_layout() -> Void {
  let d = PD::for_fast_field(
    Kind::Data,
    Attrs::NONE,
    Loc::Field,
    Constness::Const,
    Rep::double(),
    4 * 3,
    true,
  );
  assert_eq!(d.location(), Loc::Field);
  assert_eq!(d.representation(), Rep::double());
  assert_eq!(d.field_offset(), 3); // 字节偏移 12 → 3 words
  assert!(d.is_in_object());
  assert_eq!(d.field_width_in_words(), 1);
  assert_eq!(d.storage_location().offset_in_words, 3);
  assert!(d.storage_location().is_in_object);

  let ooo = PD::for_fast_field(
    Kind::Data,
    Attrs::NONE,
    Loc::Field,
    Constness::Mutable,
    Rep::tagged(),
    4 * 2,
    false,
  );
  assert!(!ooo.is_in_object());
  assert!(!ooo.storage_location().is_in_object);
  OK
}

/// 规约 (property-details.h:380-416 更新族):
/// set_pointer/set_cell_type/set_index/copy_with_* 返回新值不改旧值。
#[test]
fn update_family_is_functional() -> Void {
  let base = PD::for_fast(
    Kind::Data,
    Attrs::NONE,
    Loc::Descriptor,
    Constness::Mutable,
    Rep::smi(),
  );
  let p = base.set_pointer(5);
  assert_eq!(p.pointer(), 5);
  assert_ne!(base.pointer(), 5);
  assert_eq!(p.to_raw() & DESCRIPTOR_POINTER_MASK, 5 << 9);

  let i = base.set_index(9);
  assert_eq!(i.dictionary_index(), 9);
  let c = base.set_cell_type(Cell::InTransition);
  assert_eq!(c.cell_type(), Cell::InTransition);

  let r = base.copy_with_representation(Rep::heap_object());
  assert_eq!(r.representation(), Rep::heap_object());
  let k = base.copy_with_constness(Constness::Const);
  assert_eq!(k.constness(), Constness::Const);
  let a = base.copy_add_attributes(Attrs::READ_ONLY);
  assert!(a.is_read_only());
  assert!(base.is_enumerable() && base.is_configurable());
  OK
}

/// 规约 (property-details-inl.h:19-24):
/// from_smi/as_smi 往返 — 前置: bit31 镜像 bit30 ("upper 2 bits have the
/// same value"), 31 位 smi 才能承载第 31 位。
#[test]
fn smi_roundtrip_keeps_bit31() -> Void {
  for raw in [
    0u32,
    1,
    0b111,
    (1 << 30) | (1 << 31),
    ((1 << 30) | (1 << 31)) | 123,
  ] {
    let d = PD::from_raw(raw);
    let smi = d.as_smi();
    assert_eq!(PD::from_smi(smi), d, "raw={raw:#x}");
    assert_eq!(Smi::value(smi) as u32, raw, "raw={raw:#x}");
  }
  // 空构造 = Smi 0 (property.cc:59)
  assert_eq!(PD::empty(), PD::from_smi(Smi::from_int(0)));
  OK
}

/// 规约 (property-details.h:478-505 static_assert 组):
/// DictionaryStorage 末位恰为 30; InObject 末位 30; 前三字段共用首字节。
#[test]
fn bitfield_layout_anchors() -> Void {
  assert_eq!(DICTIONARY_STORAGE_MASK, 0x7FFF_FF00);
  assert_eq!(OFFSET_IN_WORDS_MASK, 0x3FF8_0000);
  // kind(bit0) | constness(bit1) | attributes(bits2-4) 首字节自洽
  let byte =
    PD::for_global_dictionary(Kind::Accessor, Attrs::DONT_DELETE, Cell::NO_CELL, 0).to_byte();
  assert_eq!(byte, 0b0001_0001); // kind=1 | attrs(4)<<2
  assert!(PD::is_valid_index(DICTIONARY_STORAGE_MASK >> 8));
  assert!(!PD::is_valid_index((DICTIONARY_STORAGE_MASK >> 8) + 1));
  OK
}

/// 规约 (property-details.h:590-605 泛化谓词):
/// kField 更泛化; kMutable 更泛化; GeneralizeConstness 短路。
#[test]
fn generalization_predicates() -> Void {
  assert!(location_is_generalizable_to(Loc::Descriptor, Loc::Field));
  assert!(!location_is_generalizable_to(Loc::Field, Loc::Descriptor));
  let gc = generalize_constness;
  assert_eq!(gc(Constness::Const, Constness::Mutable), Constness::Mutable);
  assert_eq!(gc(Constness::Mutable, Constness::Const), Constness::Mutable);
  assert_eq!(gc(Constness::Const, Constness::Const), Constness::Const);
  OK
}

/// 规约 (property-details.h:267-307 FieldStorageLocation):
/// First: 对象内有空位则内置; Next: 内置装满外移 PropertyArray[2], 外置递增。
#[test]
fn field_storage_location_transitions() -> Void {
  use v8_objects_core::property_details::FieldStorageLocation as FSL;
  let first_in = FSL::first(3, 8);
  assert_eq!(first_in.offset_in_words, 3);
  assert!(first_in.is_in_object);

  let first_out = FSL::first(8, 8);
  assert_eq!(
    first_out.offset_in_words,
    FSL::FIRST_OUT_OF_OBJECT_OFFSET_IN_WORDS
  );
  assert!(!first_out.is_in_object);

  // 内置下一字段: 偏移 +1 仍 < 8 → 保持内置
  let next_in = first_in.next(8, 1);
  assert_eq!(next_in.offset_in_words, 4);
  assert!(next_in.is_in_object);

  // 偏移推进到 7, 再 +1 恰等于对象大小 → 外移
  let last = FSL {
    offset_in_words: 7,
    is_in_object: true,
  };
  let moved = last.next(8, 1);
  assert_eq!(moved.offset_in_words, 2);
  assert!(!moved.is_in_object);

  // 外置字段: 递增不回迁
  let ooo = FSL {
    offset_in_words: 2,
    is_in_object: false,
  };
  assert_eq!(ooo.next(8, 1).offset_in_words, 3);
  OK
}
