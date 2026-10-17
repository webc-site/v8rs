// 1:1 移植自 v8/test/unittests/codegen/safepoint-table-unittest.cc (W11CG)
// 原版 Assembler 缓冲面以 Vec<u8> 承载; FindEntry 的隐式起点面在 Rust 以 None 表达。

use core::mem::size_of;

use aok::{OK, Void};
use log::info;
use v8_codegen_core::{
  BitVector, GrowableBitVector, K_HEADER_SIZE, K_NO_DEOPT_INDEX, K_NO_TRAMPOLINE_PC,
  SafepointEntry, SafepointTable, SafepointTableBuilder, compare_and_create_xor_patch,
  decode_safepoint_entry, encode_safepoint_entry,
};

#[ctor::ctor(unsafe)]
fn _log_init() {
  log_init::init();
}

#[test]
fn test_create_patch() -> Void {
  info!("测试 CompareAndCreateXorPatch 各差异形态");
  const K_SIZE: usize = 200;
  assert!(K_SIZE >= size_of::<usize>() * 3 * 8);

  let empty = GrowableBitVector::new();
  let mut u = GrowableBitVector::new();
  let mut v = GrowableBitVector::new();
  let mut w = GrowableBitVector::new();
  u.add(80);
  v.add(80);
  w.add(5);

  let (patch, common) = compare_and_create_xor_patch(&u, &v);
  assert_eq!(u32::MAX, common);
  assert!(patch.is_none());

  // 与空向量的差异
  let (patch, common) = compare_and_create_xor_patch(&empty, &v);
  assert_eq!(80u32, common);
  let patch = patch.expect("差异存在");
  assert_eq!(1, patch.length());
  assert!(patch.contains(0));

  // 空向量对非空向量的差异
  let (patch, common) = compare_and_create_xor_patch(&u, &empty);
  assert_eq!(80u32, common);
  let patch = patch.expect("差异存在");
  assert_eq!(1, patch.length());
  assert!(patch.contains(0));

  // 开头的 0->1 差异
  let (patch, common) = compare_and_create_xor_patch(&empty, &w);
  assert_eq!(5u32, common);
  let patch = patch.expect("差异存在");
  assert_eq!(1, patch.length());
  assert!(patch.contains(0));

  // 第二字中段的差异
  v.add(81);
  v.add(191);
  let (patch, common) = compare_and_create_xor_patch(&u, &v);
  assert_eq!(81u32, common);
  let patch = patch.expect("差异存在");
  assert_eq!(192 - 81, patch.length());
  assert!(patch.contains(81 - 81));
  assert!(patch.contains(191 - 81));

  // 尾部全同、中段小差异
  v.add(83);
  u.add(191);
  let (patch, common) = compare_and_create_xor_patch(&u, &v);
  assert_eq!(81u32, common);
  let patch = patch.expect("差异存在");
  assert_eq!(3, patch.length());
  assert!(patch.contains(81 - 81));
  assert!(patch.contains(83 - 81));

  // 第二字开头的 1->0 差异
  u.add(64);
  let (patch, common) = compare_and_create_xor_patch(&u, &v);
  assert_eq!(64u32, common);
  let patch = patch.expect("差异存在");
  assert_eq!(84 - 64, patch.length());
  assert!(patch.contains(64 - 64));
  assert!(!patch.contains(80 - 64), "u 与 v 共有该位");
  assert!(patch.contains(81 - 64));
  assert!(patch.contains(83 - 64));

  // 双方含极高位置
  u.add(1023);
  v.add(1023);
  let (patch, common) = compare_and_create_xor_patch(&u, &v);
  assert_eq!(64u32, common);
  let patch = patch.expect("差异存在");
  assert_eq!(84 - 64, patch.length());
  assert!(patch.contains(64 - 64));
  assert!(patch.contains(83 - 64));
  OK
}

#[test]
fn test_no_tagged_slots() -> Void {
  info!("测试 NoTaggedSlots: 全同条目去重为空表, 解码器可处理");
  let mut builder = SafepointTableBuilder::new(8, false, false);
  // 若干无 tagged 槽的 safepoint, 全部可去重为空条目
  let mut sp1 = SafepointEntry::new(10);
  sp1.reset_tagged_slots(0);
  builder.add_entry(sp1);
  let mut sp2 = SafepointEntry::new(20);
  sp2.reset_tagged_slots(0);
  builder.add_entry(sp2);

  let encoded = builder.emit();
  let table = SafepointTable::parse(&encoded)?;
  assert_eq!(8, table.stack_slots);
  assert_eq!(K_HEADER_SIZE as u32, table.byte_length);
  // 解码器须能处理空条目: 原版返回隐式起点 (pc=1, 全零位图); Rust 面以 None 表达
  let entry = table.find_entry(1);
  if let Some(e) = entry {
    assert!(!e.tagged_slots.iter().any(|b| *b));
  } else {
    assert!(table.entries.is_empty());
  }
  OK
}

#[test]
fn test_basic() -> Void {
  info!("测试 Basic: 6 槽表, 槽 5 置位条目与空条目");
  const K_STACK_SLOT_COUNT: u32 = 6;
  let mut builder = SafepointTableBuilder::new(K_STACK_SLOT_COUNT, false, false);
  let mut sp1 = SafepointEntry::new(0x12f);
  sp1.set_tagged_slot(5);
  builder.add_entry(sp1);
  let sp2 = SafepointEntry::new(0x143);
  // 无 tagged 槽
  builder.add_entry(sp2);

  let encoded = builder.emit();
  let flip_index = |original_index: u32| K_STACK_SLOT_COUNT - 1 - original_index;

  let table = SafepointTable::parse(&encoded)?;
  assert_eq!(K_STACK_SLOT_COUNT, table.stack_slots);
  let entry = table.find_entry(0x12f).expect("覆盖条目存在");
  assert_eq!(0x12f, entry.pc);
  // 原断言 tagged_slots()[0] == 1 << flip_index(5) (翻转域位 0); 本面存原槽号位图
  assert_eq!(0, flip_index(5));
  assert!(entry.is_tagged_slot(5));
  assert!(!entry.is_tagged_slot(0));

  let table = SafepointTable::parse(&encoded)?;
  let entry = table.find_entry(0x143).expect("覆盖条目存在");
  assert_eq!(0x143, entry.pc);
  assert!(!entry.is_tagged_slot(5));
  OK
}

struct Data {
  pc: u32,
  deopt: i32,
  trampoline: i32,
  bits: &'static [u32],
}

#[test]
fn test_bigger() -> Void {
  info!("测试 Bigger: 17 槽 mjsunit 去重缺陷样本与回退查找");
  const K_STACK_SLOT_COUNT: u32 = 17;
  let k_no_deopt = K_NO_DEOPT_INDEX;
  let k_no_trampoline = K_NO_TRAMPOLINE_PC;
  // 取自暴露缺陷的 mjsunit 用例
  let test_data: [Data; 15] = [
    Data {
      pc: 0xdc,
      deopt: 3,
      trampoline: 0x6bd,
      bits: &[8, 13],
    },
    Data {
      pc: 0x21a,
      deopt: 4,
      trampoline: 0x6c1,
      bits: &[13],
    },
    Data {
      pc: 0x2c4,
      deopt: 5,
      trampoline: 0x6c5,
      bits: &[13],
    },
    Data {
      pc: 0x2e5,
      deopt: k_no_deopt,
      trampoline: k_no_trampoline,
      bits: &[13, 15],
    },
    Data {
      pc: 0x304,
      deopt: 6,
      trampoline: 0x6c9,
      bits: &[13, 14, 15],
    },
    Data {
      pc: 0x325,
      deopt: 7,
      trampoline: 0x6cd,
      bits: &[13],
    },
    Data {
      pc: 0x369,
      deopt: 8,
      trampoline: 0x6d1,
      bits: &[8, 13],
    },
    Data {
      pc: 0x3f8,
      deopt: k_no_deopt,
      trampoline: k_no_trampoline,
      bits: &[8, 13],
    },
    Data {
      pc: 0x44b,
      deopt: k_no_deopt,
      trampoline: k_no_trampoline,
      bits: &[8, 13, 15],
    },
    Data {
      pc: 0x468,
      deopt: 9,
      trampoline: 0x6d5,
      bits: &[8, 13],
    },
    Data {
      pc: 0x490,
      deopt: 10,
      trampoline: 0x6d9,
      bits: &[13],
    },
    Data {
      pc: 0x51d,
      deopt: k_no_deopt,
      trampoline: k_no_trampoline,
      bits: &[13],
    },
    Data {
      pc: 0x585,
      deopt: k_no_deopt,
      trampoline: k_no_trampoline,
      bits: &[13, 14],
    },
    Data {
      pc: 0x5ba,
      deopt: 11,
      trampoline: 0x6dd,
      bits: &[13],
    },
    Data {
      pc: 0x5df,
      deopt: 12,
      trampoline: 0x6e1,
      bits: &[13],
    },
  ];

  let mut builder = SafepointTableBuilder::new(K_STACK_SLOT_COUNT, false, true);
  let mut last_updated = 0usize;
  for data in &test_data {
    let mut sp = SafepointEntry::new(data.pc);
    for &bit in data.bits {
      sp.set_tagged_slot(bit as usize);
    }
    builder.add_entry(sp);
    if data.deopt != k_no_deopt {
      last_updated =
        builder.update_deoptimization_info(data.pc, data.trampoline, last_updated, data.deopt);
    }
  }

  let encoded = builder.emit();
  let table = SafepointTable::parse(&encoded)?;

  // 原版 flip_index 边界断言
  let flip_index = |original_index: u32| K_STACK_SLOT_COUNT - 1 - original_index;
  assert_eq!(0, flip_index(K_STACK_SLOT_COUNT - 1));
  assert_eq!(K_STACK_SLOT_COUNT - 1, flip_index(0));

  for data in &test_data {
    assert_eq!(K_STACK_SLOT_COUNT, table.stack_slots);
    let entry = table.find_entry(data.pc).expect("覆盖条目存在");
    assert_eq!(data.pc, entry.pc, "SCOPED_TRACE pc");
    if data.deopt != k_no_deopt {
      assert_eq!(data.deopt, entry.deopt_index);
      assert_eq!(data.trampoline, entry.trampoline_pc);
    }
    for &bit in data.bits {
      assert!(
        entry.is_tagged_slot(bit as usize),
        "槽 {bit} 应置位 pc={:#x}",
        data.pc
      );
    }
  }

  // 去重逻辑: 两 safepoint/trampoline 之间的 pc 回退到前一个 safepoint/trampoline
  let fallbacks: [(u32, u32); 8] = [
    (0xdd, 0xdc),   // 恰在既有 PC 之后
    (0x2c3, 0x21a), // 恰在下个 PC 之前
    (0x600, 0x5df), // 末个 PC 之后
    (0x6bd, 0xdc),  // Trampoline
    (0x6c0, 0xdc),  // 两 trampoline 之间
    (0x6c1, 0x21a), // 下个 trampoline
    (0x6dc, 0x490), // 必须回退过两个无 trampoline 的条目
    (0x700, 0x5df), // 超过末个 trampoline
  ];
  for (lookup, returned) in fallbacks {
    let entry = table
      .find_entry(lookup)
      .unwrap_or_else(|| panic!("回退查找缺失 lookup={lookup:#x}"));
    assert_eq!(returned, entry.pc, "回退 pc 不符 lookup={lookup:#x}");
  }
  OK
}

#[test]
fn test_decode_safepoint_entry() -> Void {
  info!("测试 DecodeSafepointEntry 全量/紧凑/跳字节形态 (跨用例累积 tagged_slots)");
  const K_SIZE: usize = 16;
  let mut tagged_slots = [0u8; K_SIZE];
  // 这些用例不独立: tagged_slots 向量逐用例传递以测更新行为
  let cases: &[(&[u8], &[u8])] = &[
    // 全量编码, 置 3 字节
    (
      &[0b0000_0110, 0b00001111, 0b00111100, 0b11110000],
      &[0b00001111, 0b00111100, 0b11110000],
    ),
    // 零字节输入表示无变化
    (&[0], &[0b00001111, 0b00111100, 0b11110000]),
    // 紧凑编码翻两比特, 无跳比特
    (&[0b0000_1101], &[0b00001100, 0b00111100, 0b11110000]),
    // 紧凑编码翻两比特, 跳 11 比特 (编码为 4)
    (
      &[0b0001_0011, 0b0000_1101],
      &[0b00001100, 0b00100100, 0b11110000],
    ),
    // 同一两比特翻回, 无跳比特
    (
      &[0b1000_0001, 0b1100_0000, 0b0000_0001],
      &[0b00001100, 0b00111100, 0b11110000],
    ),
  ];
  for (input, expected) in cases {
    // update_tagged_slots=true: 解码并比对期望位图
    let mut ptr = 0usize;
    decode_safepoint_entry::<true>(input, &mut ptr, &mut tagged_slots)?;
    assert_eq!(input.len(), ptr, "解码器不应读太远");
    for i in 0..expected.len() {
      assert_eq!(tagged_slots[i], expected[i]);
    }
    // update_tagged_slots=false: 消费同量字节但不改输出
    let mut no_update = 0usize;
    decode_safepoint_entry::<false>(input, &mut no_update, &mut tagged_slots)?;
    assert_eq!(ptr, no_update);
    for i in 0..expected.len() {
      assert_eq!(tagged_slots[i], expected[i]);
    }
  }
  OK
}

#[test]
fn test_encode_safepoint_entry() -> Void {
  info!("测试 EncodeSafepointEntry 单字节/双字节/跳比特形态 (记号: 撇号分隔载荷与 tag 比特)");
  let test = |stack_slot_count: u32,
              common_prefix: u32,
              bits_length: usize,
              bit_list: &[usize],
              result_bytes: &[u8]| {
    let bits = BitVector::from_bits(bits_length, bit_list);
    let mut out: Vec<u8> = Vec::new();
    encode_safepoint_entry(stack_slot_count, common_prefix, Some(&bits), &mut out);
    assert_eq!(result_bytes.len(), out.len());
    for i in 0..result_bytes.len() {
      assert_eq!(result_bytes[i], out[i], "第 {i} 字节不符");
    }
  };
  // 单字节 xor 掩码
  test(10, 9, 1, &[0], &[0b0000_0101]);
  test(10, 8, 1, &[0], &[0b0000_1001]);
  test(10, 5, 3, &[0, 2], &[0b0101_0001]);
  // 双字节 xor 掩码
  test(10, 0, 9, &[0, 3, 4, 5, 8], &[0b1100_1001, 0b0001_0011]);
  // 跳 9 比特 (编码为 2) 一字节 + xor 掩码一字节
  test(10, 0, 1, &[0], &[0b0000_1011, 0b0000_0101]);
  // 跳 53 比特 (编码为 46) 两字节 + xor 掩码两字节
  test(60, 1, 6, &[0, 5], &[0b1011_1011, 0b0_1, 0b1000_0101, 0b0_1]);
  OK
}
