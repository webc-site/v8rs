//! opmask-unittest.cc 1:1 移植 — MaskBuilder 全掩码/部分掩码匹配语义
//! v8: test/unittests/compiler/turboshaft/opmask-unittest.cc

use aok::{OK, Void};
use log::info;
use v8_turboshaft::{MaskBuilder, OpMask, OpMaskField, Opcode};

#[ctor::ctor(unsafe)]
fn _log_init() {
  log_init::init();
}

/// v8: opmask-unittest.cc Opcode::kConstant 借位说明 — 测试无法扩枚举,
/// 复用 kConstant (原版注释保留)
const OPCODE: u8 = Opcode::Constant.index();

/// v8: opmask-unittest.cc MyFakeOp::Kind
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u16)]
enum Kind {
  A = 0x0000,
  B = 0x0001,
  C = 0x0100,
  D = 0x11F8,
  E = 0xFFFF,
}

/// v8: opmask-unittest.cc MyFakeOp — FixedArityOperationT<0>: 基座 Operation
/// 占 4 字节 (opcode@0 + sizes), 载荷字段自偏移 4 起: kind u16@4, value u16@6
#[derive(Debug, Clone, Copy)]
struct MyFakeOp {
  kind: Kind,
  value: u16,
}

impl MyFakeOp {
  /// v8: opmask-unittest.cc MyFakeOp::MyFakeOp
  const fn new(kind: Kind, value: u16) -> Self {
    Self { kind, value }
  }

  /// 原版按操作前 8 字节 (小端) 比较: 此处按布局显式拼头
  /// opcode@0 | kind@4 | value@6
  const fn head(self) -> u64 {
    (OPCODE as u64) | ((self.kind as u64) << (4 * 8)) | ((self.value as u64) << (6 * 8))
  }

  /// v8: opmask-unittest.cc op.Is<Mask>() — Operation::Is 语义面
  fn is(self, m: &OpMask) -> bool {
    m.matches(self.head())
  }
}

/// v8: opmask-unittest.cc FIELD(MyFakeOp, kind) / FIELD(MyFakeOp, value)
static FULL_FIELDS: [OpMaskField; 2] = [
  OpMaskField {
    name: "kind",
    offset: 4,
    size: 2,
  },
  OpMaskField {
    name: "value",
    offset: 6,
    size: 2,
  },
];

/// v8: opmask-unittest.cc FIELD(MyFakeOp, kind) (MyFakeKindMask 单字段)
static KIND_FIELDS: [OpMaskField; 1] = [OpMaskField {
  name: "kind",
  offset: 4,
  size: 2,
}];

/// v8: opmask-unittest.cc MyFakeMask = MaskBuilder<MyFakeOp, kind, value>
static MY_FAKE_MASK: MaskBuilder = MaskBuilder {
  fields: &FULL_FIELDS,
};

/// v8: opmask-unittest.cc MyFakeKindMask = MaskBuilder<MyFakeOp, kind>
static MY_FAKE_KIND_MASK: MaskBuilder = MaskBuilder {
  fields: &KIND_FIELDS,
};

/// v8: opmask-unittest.cc MyFakeMask::For<Kind, value>
const fn full(kind: Kind, value: u64) -> OpMask {
  MY_FAKE_MASK.r#for(OPCODE, &[kind as u64, value])
}

/// v8: opmask-unittest.cc MyFakeKindMask::For<Kind>
const fn kind_only(kind: Kind) -> OpMask {
  MY_FAKE_KIND_MASK.r#for(OPCODE, &[kind as u64])
}

/// v8: opmask-unittest.cc MaskList<kA0..kB11F8> — 10 个全掩码族
static MASK_LIST: [(&str, OpMask); 10] = [
  ("kA0", full(Kind::A, 0)),
  ("kB0", full(Kind::B, 0)),
  ("kC0", full(Kind::C, 0)),
  ("kD0", full(Kind::D, 0)),
  ("kA1", full(Kind::A, 1)),
  ("kC1", full(Kind::C, 1)),
  ("kB0100", full(Kind::B, 0x0100)),
  ("kD0100", full(Kind::D, 0x0100)),
  ("kA11F8", full(Kind::A, 0x11F8)),
  ("kB11F8", full(Kind::B, 0x11F8)),
];

/// v8: opmask-unittest.cc Check<Expected>(op) — 恰有 Expected 匹配 (None 即全不匹配)
fn check(op: MyFakeOp, expected: Option<&str>) {
  for (name, m) in MASK_LIST.iter() {
    assert_eq!(
      op.is(m),
      expected == Some(*name),
      "op {:?}/{:#x} vs {}",
      op.kind,
      op.value,
      name
    );
  }
}

/// OpmaskTest.FullMask — 全字段掩码精确匹配
#[test]
fn opmask_test_full_mask() -> Void {
  info!("> OpmaskTest.FullMask");

  check(MyFakeOp::new(Kind::A, 0), Some("kA0"));
  check(MyFakeOp::new(Kind::B, 0), Some("kB0"));
  check(MyFakeOp::new(Kind::C, 1), Some("kC1"));
  check(MyFakeOp::new(Kind::B, 0x0100), Some("kB0100"));
  check(MyFakeOp::new(Kind::D, 0x0100), Some("kD0100"));
  check(MyFakeOp::new(Kind::A, 0x11F8), Some("kA11F8"));

  // 原版: 以下不应匹配任何掩码
  check(MyFakeOp::new(Kind::E, 0), None);
  check(MyFakeOp::new(Kind::E, 0x11F8), None);
  check(MyFakeOp::new(Kind::A, 2), None);
  check(MyFakeOp::new(Kind::D, 0xF811), None);
  check(MyFakeOp::new(Kind::A, 0x0100), None);

  OK
}

/// OpmaskTest.PartialMask — 仅 kind 字段的部分掩码跨 value 匹配
#[test]
fn opmask_test_partial_mask() -> Void {
  info!("> OpmaskTest.PartialMask");

  let ka = kind_only(Kind::A);
  let kc = kind_only(Kind::C);

  for v in [0u16, 1, 2, 0x0100, 0x0101, 0x11F8] {
    let op = MyFakeOp::new(Kind::A, v);
    assert!(op.is(&ka), "kind-only kA 应匹配 value {:#x}", v);
    assert!(!op.is(&kc), "kind-only kC 不应匹配 value {:#x}", v);
  }

  for v in [0u16, 1, 2, 0x0100, 0x0101, 0x11F8] {
    let op = MyFakeOp::new(Kind::C, v);
    assert!(!op.is(&ka), "kind-only kA 不应匹配 value {:#x}", v);
    assert!(op.is(&kc), "kind-only kC 应匹配 value {:#x}", v);
  }

  OK
}
