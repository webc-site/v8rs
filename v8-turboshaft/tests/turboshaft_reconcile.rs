//! W11TSR3 round3 对抗对拍: graph op_id 跨边界批量随机 / index 区间算术 / operations Kind 与 may_throw / abstract_rep 跨件引用
//! v8: src/compiler/turboshaft/graph.h op_id_count/div_ceil + op_id_capacity/floor
//!     + index.h OpIndex id/offset/hash/valid/IsExternalId/EncodeExternalId (release 布局, offset 掩码 no-op 等价)
//!     + operations.h MayThrow 集 + 各 *Op::Kind enum 判别序

use aok::{OK, Void};
use log::info;
use v8_turboshaft::{
  BlockIndex, ChangeOrDeoptKind, FloatBinopKind, Graph, OpIndex, Opcode, Origin,
  OverflowCheckedBinopKind, RegisterRepresentation, Simd128ShuffleKind, Word32PairBinopKind,
  abstract_rep,
};

#[ctor::ctor(unsafe)]
fn _log_init() {
  log_init::init();
}

/// 固定种子 LCG (线性同余), 仅造数; 与 kSlotsPerId=2 无关
struct Lcg {
  state: u64,
}

impl Lcg {
  const fn new(seed: u64) -> Self {
    Self { state: seed }
  }
  /// 产 [0, 2^31) 的 u32
  fn next_u32(&mut self) -> u32 {
    self.state = self
      .state
      .wrapping_mul(6364136223846793005)
      .wrapping_add(1442695040888963407);
    (self.state >> 33) as u32
  }
}

/// 面 1 · op_id_count(op_id_capacity) 跨奇偶边界的批量随机对拍 (count=ceil / capacity=floor)
/// 手推公式: count(n)=(n+1)/2 (ceil), capacity(n)=n/2 (floor); 逐值锁定, 且校验 checksum 与奇数分岔计数
#[test]
fn graph_op_id_batch_random_reconcile() -> Void {
  info!("> graph op_id 批量随机对拍");
  let mut rng = Lcg::new(0x9e37_79b9_7f4a_7c15);
  let mut g = Graph::new(Origin::PureTurboshaft);
  let mut count_sum: u64 = 0;
  let mut cap_sum: u64 = 0;
  let mut odd_diverge = 0u32; // count≠(n/2) 的奇数槽次数 (ceil 与 floor 分岔)

  // 直接逐值对拍 (size 与 capacity 独立扫边界)
  for _ in 0..4096 {
    let n = rng.next_u32() as usize;
    // op_id_count: div_ceil(n,2) — 手推 (n+1)/2
    g.operations.slot_count = n;
    let got_count = g.op_id_count() as u64;
    let hand_count = (n as u64).div_ceil(2);
    assert_eq!(got_count, hand_count, "op_id_count 违 div_ceil 手推: n={n}");
    count_sum = count_sum.wrapping_add(got_count);
    if n % 2 == 1 {
      assert_ne!(got_count, (n as u64) / 2, "奇数 n 应 ceil≠floor: n={n}");
      odd_diverge += 1;
    }
    // op_id_capacity: floor(n,2) — 手推 n/2
    g.operations.slot_capacity = n;
    let got_cap = g.op_id_capacity() as u64;
    let hand_cap = (n as u64) / 2;
    assert_eq!(got_cap, hand_cap, "op_id_capacity 违 floor 手推: n={n}");
    cap_sum = cap_sum.wrapping_add(got_cap);
  }
  // 分岔确实被大量触发 (确保覆盖奇偶边界, 非退化)
  assert!(odd_diverge > 1500, "奇数边界覆盖不足: {odd_diverge}");
  // checksum 独立复算 (与实现走不同累加序, 再对拍一次)
  let mut rng2 = Lcg::new(0x9e37_79b9_7f4a_7c15);
  let mut c2 = 0u64;
  let mut k2 = 0u64;
  for _ in 0..4096 {
    let n = rng2.next_u32() as u64;
    c2 = c2.wrapping_add(n.div_ceil(2));
    k2 = k2.wrapping_add(n / 2);
  }
  assert_eq!(count_sum, c2, "op_id_count checksum 漂移");
  assert_eq!(cap_sum, k2, "op_id_capacity checksum 漂移");
  OK
}

/// 面 1 · 模拟逐 op 发射 (每 op 占 2..5 槽, 容量翻倍增长) 下的 count/capacity 连续性
/// 校验: op_id_count 随发射单调不减, 且恒等于运行 size 的 div_ceil; capacity 恒等于运行 cap 的 floor
#[test]
fn graph_op_id_emission_monotonic() -> Void {
  info!("> graph op_id 发射单调性对拍");
  let mut rng = Lcg::new(0xdead_beef_0123_4567);
  let mut g = Graph::new(Origin::PureTurboshaft);
  let mut prev_count = 0u32;
  let mut size = 0usize;
  let mut cap = 0usize;
  for _ in 0..2048 {
    let slots = 2 + (rng.next_u32() % 4) as usize; // 每 op 至少 kSlotsPerId=2
    size += slots;
    while cap < size {
      cap = if cap == 0 { 64 } else { cap.saturating_mul(2) };
    }
    g.operations.slot_count = size;
    g.operations.slot_capacity = cap;
    let c = g.op_id_count();
    assert_eq!(
      c as u64,
      (size as u64).div_ceil(2),
      "发射中 count 违公式: size={size}"
    );
    assert!(c >= prev_count, "op_id_count 非单调: {prev_count}→{c}");
    prev_count = c;
    assert_eq!(
      g.op_id_capacity() as u64,
      (cap as u64) / 2,
      "发射中 capacity 违 floor: cap={cap}"
    );
  }
  OK
}

/// 面 2 · OpIndex 区间算术 release 等价对拍: id()/offset()/valid()/is_external_id()
/// 原版 offset() release 面返回裸 offset_; Rust 恒施加 & UNMASK_GENERATION_MASK (清 bit1)。
/// 合法 offset 恒为 8 的倍数 (或外部 = 8k+1), bit1 恒 0 → 掩码为 no-op, 逐值锁定该等价性。
#[test]
fn opindex_offset_mask_is_release_noop() -> Void {
  info!("> OpIndex offset 掩码 release no-op 对拍");
  let mut rng = Lcg::new(0x51ed_2701_9abc_def1);
  let mut checked = 0u32;
  for _ in 0..4096 {
    // 合法 op 起始 offset: 8 的倍数 (slot 对齐), bit1=0
    let offset = rng.next_u32() & !7u32;
    let idx = OpIndex::from_offset(offset);
    // 掩码 no-op: offset() == 裸 offset
    assert_eq!(
      idx.offset(),
      offset,
      "offset 掩码改变裸值 (bit1 应为 0): {offset}"
    );
    // id() == offset/8/2 == offset/16 (原版 offset_/sizeof/kSlotsPerId)
    assert_eq!(idx.id(), offset / 16, "id 违 offset/16: {offset}");
    assert_eq!(idx.hash(), idx.id(), "hash 应恒等于 id");
    // 8 的倍数 → 非 external, valid (除 INVALID)
    assert!(idx.valid(), "对齐 offset 应 valid");
    assert!(!idx.is_external_id(), "8 的倍数非外部 id: {offset}");
    checked += 1;
  }
  assert_eq!(checked, 4096);
  OK
}

/// 面 2 · 外部 id 编解码区间算术: encode_external_id → is_external_id/decode/id 空间折叠
/// 原版 EncodeExternalId: offset_=id*8+1; DecodeExternalId: offset_/8; id(): offset_/8/2
#[test]
fn opindex_external_id_reconcile() -> Void {
  info!("> OpIndex 外部 id 编解码对拍");
  let mut rng = Lcg::new(0xcafe_f00d_1234_5678);
  for _ in 0..2048 {
    let id = rng.next_u32() & 0x0fff_ffff; // id < 2^28 → raw=id*8+1 < 2^31, 不溢出 u32
    let ext = OpIndex::encode_external_id(id);
    // raw = id*8+1
    assert_eq!(ext.offset(), id * 8 + 1, "外部 raw 违 id*8+1");
    assert!(ext.is_external_id(), "外部 flag bit0=1 应识别");
    assert_eq!(ext.decode_external_id(), id, "decode 逆不还原");
    // id() = raw/8/2 = (id*8+1)/8/2 = id/2 (floor) — 外部 id 复用紧凑 id 空间
    assert_eq!(
      ext.id(),
      (id as u64 / 2) as u32,
      "外部 id() 空间折叠违 id/2"
    );
    // raw=id*8+1 恒非 INVALID (u32::MAX), 故 valid 为真
    assert!(ext.valid(), "外部编码 raw=id*8+1 不应等于 INVALID");
  }
  OK
}

/// 面 2 · OpIndex/BlockIndex 序算术 (原版 operator< 按 offset_; Rust derive Ord 按裸 u32; 一致)
/// 遍历序: 支配块 id 更小 (BlockIndex 注释); OpIndex 按 offset 单调
#[test]
fn opindex_blockindex_ordering() -> Void {
  info!("> OpIndex/BlockIndex 序单调对拍");
  let mut rng = Lcg::new(0x0bad_c0de_dead_beef);
  // OpIndex: 收集 8 倍数 offset 排序, 验证 Ord 与 offset 比较一致
  let mut offs: Vec<u32> = (0..64).map(|_| rng.next_u32() & !7u32).collect();
  offs.sort_unstable();
  for w in offs.windows(2) {
    let (a, b) = (w[0], w[1]);
    let ia = OpIndex::from_offset(a);
    let ib = OpIndex::from_offset(b);
    assert_eq!(ia < ib, a < b, "OpIndex 序违 offset 序");
    assert_eq!(ia.cmp(&ib), a.cmp(&b), "OpIndex cmp 违 offset cmp");
  }
  // BlockIndex: 支配序 (id 递增) 与 Ord 一致
  let mut ids: Vec<u32> = (0..64).map(|_| rng.next_u32() & 0xffff).collect();
  ids.sort_unstable();
  for w in ids.windows(2) {
    let (a, b) = (BlockIndex::new(w[0]), BlockIndex::new(w[1]));
    assert_eq!(a < b, w[0] < w[1], "BlockIndex 序违 id 序");
  }
  // INVALID 哨兵: valid=false, id/hash 不 panic (原版 hash 允许 Invalid)
  let inv = OpIndex::INVALID;
  assert!(!inv.valid(), "INVALID 应 valid=false");
  let _ = inv.id();
  let _ = inv.hash();
  assert_eq!(BlockIndex::INVALID.id(), u32::MAX);
  assert!(!BlockIndex::INVALID.valid());
  OK
}

/// 面 3 · operations 语义: MayThrow 集 (原版 TURBOSHAFT_THROWING_OPERATIONS_LIST 共 8 项)
#[test]
fn opcode_may_throw_set_matches_original() -> Void {
  info!("> Opcode::may_throw 集对拍原版 8 项");
  let throwing = [
    Opcode::GenericBinop,
    Opcode::GenericUnop,
    Opcode::ToNumberOrNumeric,
    Opcode::LoadDictionaryField,
    Opcode::StringLocaleCompareIntl,
    Opcode::StringToCaseIntl,
    Opcode::Call,
    Opcode::FastApiCall,
  ];
  for op in throwing {
    assert!(op.may_throw(), "{op:?} 原版在 throwing 列表内");
  }
  // 反例 (原版 default → false)
  let not_throwing = [
    Opcode::Goto,
    Opcode::Switch,
    Opcode::Load,
    Opcode::Store,
    Opcode::Phi,
    Opcode::Constant,
    Opcode::WordBinop,
    Opcode::Change,
    Opcode::Comparison,
    Opcode::DidntThrow,
  ];
  for op in not_throwing {
    assert!(!op.may_throw(), "{op:?} 不应 may_throw");
  }
  OK
}

/// 面 3 · Kind enum 判别序 + from_repr 完备性 (逐变体 round-trip, 越界回退首变体)
/// 对拍原版 enum class Kind 声明序 (FloatBinop/ChangeOrDeopt/Simd128Shuffle/Word32Pair/OverflowChecked)
#[test]
fn kind_enums_from_repr_roundtrip() -> Void {
  info!("> Kind 判别序与 from_repr 完备性对拍");
  // FloatBinopKind (9): kAdd..kAtan2
  let fb = [
    FloatBinopKind::Add,
    FloatBinopKind::Mul,
    FloatBinopKind::Min,
    FloatBinopKind::Max,
    FloatBinopKind::Sub,
    FloatBinopKind::Div,
    FloatBinopKind::Mod,
    FloatBinopKind::Power,
    FloatBinopKind::Atan2,
  ];
  for (i, k) in fb.iter().enumerate() {
    assert_eq!(*k as u8, i as u8, "FloatBinopKind 判别序漂移 @{i}");
    assert_eq!(
      FloatBinopKind::from_repr(i as u8),
      *k,
      "FloatBinopKind from_repr 违 @{i}"
    );
  }
  assert_eq!(
    FloatBinopKind::from_repr(fb.len() as u8),
    FloatBinopKind::Add,
    "越界回退首变体"
  );

  // ChangeOrDeoptKind (13)
  let cd = [
    ChangeOrDeoptKind::Uint32ToInt32,
    ChangeOrDeoptKind::Int64ToInt32,
    ChangeOrDeoptKind::Int64ToAdditiveSafeInteger,
    ChangeOrDeoptKind::Uint64ToInt32,
    ChangeOrDeoptKind::Uint64ToInt64,
    ChangeOrDeoptKind::Float64ToInt32,
    ChangeOrDeoptKind::Float64ToUint32,
    ChangeOrDeoptKind::Int32ToUint64,
    ChangeOrDeoptKind::Int64ToUint64,
    ChangeOrDeoptKind::Float64ToUint64,
    ChangeOrDeoptKind::Float64ToAdditiveSafeInteger,
    ChangeOrDeoptKind::Float64ToInt64,
    ChangeOrDeoptKind::Float64NotHole,
  ];
  for (i, k) in cd.iter().enumerate() {
    assert_eq!(*k as u8, i as u8, "ChangeOrDeoptKind 判别序漂移 @{i}");
    assert_eq!(
      ChangeOrDeoptKind::from_repr(i as u8),
      *k,
      "ChangeOrDeoptKind from_repr 违 @{i}"
    );
  }
  assert_eq!(
    ChangeOrDeoptKind::from_repr(cd.len() as u8),
    ChangeOrDeoptKind::Uint32ToInt32
  );

  // Simd128ShuffleKind (5)
  let sh = [
    Simd128ShuffleKind::I8x1,
    Simd128ShuffleKind::I8x2,
    Simd128ShuffleKind::I8x4,
    Simd128ShuffleKind::I8x8,
    Simd128ShuffleKind::I8x16,
  ];
  for (i, k) in sh.iter().enumerate() {
    assert_eq!(*k as u8, i as u8, "Simd128ShuffleKind 判别序漂移 @{i}");
    assert_eq!(
      Simd128ShuffleKind::from_repr(i as u8),
      *k,
      "Simd128ShuffleKind from_repr 违 @{i}"
    );
  }
  assert_eq!(
    Simd128ShuffleKind::from_repr(sh.len() as u8),
    Simd128ShuffleKind::I8x1
  );

  // Word32PairBinopKind (6)
  let wp = [
    Word32PairBinopKind::Add,
    Word32PairBinopKind::Sub,
    Word32PairBinopKind::Mul,
    Word32PairBinopKind::ShiftLeft,
    Word32PairBinopKind::ShiftRightArithmetic,
    Word32PairBinopKind::ShiftRightLogical,
  ];
  for (i, k) in wp.iter().enumerate() {
    assert_eq!(*k as u8, i as u8, "Word32PairBinopKind 判别序漂移 @{i}");
    assert_eq!(
      Word32PairBinopKind::from_repr(i as u8),
      *k,
      "Word32PairBinopKind from_repr 违 @{i}"
    );
  }

  // OverflowCheckedBinopKind (3)
  let oc = [
    OverflowCheckedBinopKind::SignedAdd,
    OverflowCheckedBinopKind::SignedMul,
    OverflowCheckedBinopKind::SignedSub,
  ];
  for (i, k) in oc.iter().enumerate() {
    assert_eq!(
      *k as u8, i as u8,
      "OverflowCheckedBinopKind 判别序漂移 @{i}"
    );
    assert_eq!(
      OverflowCheckedBinopKind::from_repr(i as u8),
      *k,
      "OverflowCheckedBinopKind from_repr 违 @{i}"
    );
  }
  OK
}

/// 面 3 · 跨件引用: index.rs abstract_rep → representations.rs RegisterRepresentation 一致
/// v_traits<T>::rep 映射 (Any/None=nullrep→None; Smi→Tagged; 其余按名)
#[test]
fn abstract_rep_crossref_matches_representations() -> Void {
  info!("> abstract_rep 跨 representations 引用对拍");
  assert_eq!(abstract_rep("Any"), None);
  assert_eq!(abstract_rep("None"), None);
  assert_eq!(
    abstract_rep("Compressed"),
    Some(RegisterRepresentation::Compressed)
  );
  assert_eq!(abstract_rep("Word32"), Some(RegisterRepresentation::Word32));
  assert_eq!(abstract_rep("Word64"), Some(RegisterRepresentation::Word64));
  assert_eq!(
    abstract_rep("Float32"),
    Some(RegisterRepresentation::Float32)
  );
  assert_eq!(
    abstract_rep("Float64"),
    Some(RegisterRepresentation::Float64)
  );
  assert_eq!(abstract_rep("Smi"), Some(RegisterRepresentation::Tagged));
  assert_eq!(
    abstract_rep("Simd128"),
    Some(RegisterRepresentation::Simd128)
  );
  assert_eq!(
    abstract_rep("Simd256"),
    Some(RegisterRepresentation::Simd256)
  );
  assert_eq!(abstract_rep("Unknown"), None);
  OK
}
