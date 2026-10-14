//! opcodes-unittest.cc 1:1 移植 + 属性表抽样对数
//! (谓词全量遍历交叉验证 / 助记符 / 位表抽样 / 精确 context 分区)

use aok::{OK, Void};
use log::info;
use strum::{EnumCount, IntoEnumIterator};
use v8_compiler_graph::{
  ExactContext, IrOpcode, OP_ATOMIC, OP_BASIC_BLOCK_BEGIN, OP_COMMON, OP_COMPARISON, OP_CONSTANT,
  OP_CONTROL, OP_EXACT_CONTEXT_PARAM, OP_EXACT_CONTEXT_TRUE, OP_FRAME_STATE_INPUT,
  OP_GRAPH_TERMINATOR, OP_INLINEE, OP_JS, OP_MERGE, OP_PHI, OP_SIMD128, OPCODE_PROPS,
};

#[ctor::ctor(unsafe)]
fn _log_init() {
  log_init::init();
}

/// IrOpcodeTest.IsCommonOpcode — 全量: 表位 ⟺ [Start, StaticAssert] 区间
#[test]
fn ir_opcode_test_is_common() -> Void {
  for op in IrOpcode::iter() {
    let in_range =
      (IrOpcode::Start as u16) <= (op as u16) && (op as u16) <= (IrOpcode::StaticAssert as u16);
    assert_eq!(in_range, op.is_common(), "{}", op.mnemonic());
    assert_eq!(
      OPCODE_PROPS[op as usize] & OP_COMMON != 0,
      in_range,
      "表位 {}",
      op.mnemonic()
    );
  }
  OK
}

/// IrOpcodeTest.IsControlOpcode — 全量: 表位 ⟺ [Start, End] 区间
#[test]
fn ir_opcode_test_is_control() -> Void {
  for op in IrOpcode::iter() {
    let in_range = (IrOpcode::Start as u16) <= (op as u16) && (op as u16) <= (IrOpcode::End as u16);
    assert_eq!(in_range, op.is_control(), "{}", op.mnemonic());
    assert_eq!(
      OPCODE_PROPS[op as usize] & OP_CONTROL != 0,
      in_range,
      "表位 {}",
      op.mnemonic()
    );
  }
  OK
}

/// IrOpcodeTest.IsJsOpcode — 全量: 表位 ⟺ [JSEqual, JSDebugger] 区间
#[test]
fn ir_opcode_test_is_js() -> Void {
  for op in IrOpcode::iter() {
    let in_range =
      (IrOpcode::JSEqual as u16) <= (op as u16) && (op as u16) <= (IrOpcode::JSDebugger as u16);
    assert_eq!(in_range, op.is_js(), "{}", op.mnemonic());
    assert_eq!(
      OPCODE_PROPS[op as usize] & OP_JS != 0,
      in_range,
      "表位 {}",
      op.mnemonic()
    );
  }
  OK
}

/// IrOpcodeTest.IsConstantOpcode — 全量: JS 级 + 机器级常量并集
#[test]
fn ir_opcode_test_is_constant() -> Void {
  for op in IrOpcode::iter() {
    assert_eq!(
      op.is_constant(),
      OPCODE_PROPS[op as usize] & OP_CONSTANT != 0,
      "{}",
      op.mnemonic()
    );
  }
  // 常量表成员抽样
  for name in [
    IrOpcode::ExternalConstant,
    IrOpcode::NumberConstant,
    IrOpcode::PointerConstant,
    IrOpcode::HeapConstant,
    IrOpcode::TrustedHeapConstant,
    IrOpcode::Int32Constant,
    IrOpcode::Int64Constant,
    IrOpcode::TaggedIndexConstant,
    IrOpcode::Float32Constant,
    IrOpcode::Float64Constant,
    IrOpcode::CompressedHeapConstant,
    IrOpcode::RelocatableInt32Constant,
    IrOpcode::RelocatableInt64Constant,
  ] {
    assert!(name.is_constant(), "{} 应为常量", name.mnemonic());
  }
  assert!(!IrOpcode::Phi.is_constant());
  OK
}

/// IrOpcodeTest.IsComparisonOpcode — 三表并集全量交叉
#[test]
fn ir_opcode_test_is_comparison() -> Void {
  for op in IrOpcode::iter() {
    assert_eq!(
      op.is_comparison(),
      OPCODE_PROPS[op as usize] & OP_COMPARISON != 0,
      "{}",
      op.mnemonic()
    );
  }
  // 并集成员计数: JS(6) + SIMPLIFIED(20) + MACHINE(16) = 42
  assert_eq!(42, IrOpcode::iter().filter(|o| o.is_comparison()).count());
  assert!(
    IrOpcode::JSEqual.is_comparison()
      && IrOpcode::NumberEqual.is_comparison()
      && IrOpcode::Word32Equal.is_comparison()
  );
  assert!(!IrOpcode::JSAdd.is_comparison());
  OK
}

/// IrOpcodeTest.Mnemonic — 全量: 变体名 == 助记符
#[test]
fn ir_opcode_test_mnemonic() -> Void {
  for op in IrOpcode::iter() {
    assert_eq!(op.mnemonic(), <IrOpcode as Into<&str>>::into(op));
  }
  assert_eq!(IrOpcode::mnemonic_of(0), "Start");
  assert_eq!(IrOpcode::mnemonic_of(u16::MAX), "UnknownOpcode");
  OK
}

/// 变体总数与关键判别值 (X-macro 展开对数; C 预处理器交叉核对于生成脚本)
#[test]
fn ir_opcode_test_count_and_discriminants() -> Void {
  assert_eq!(1036, IrOpcode::COUNT);
  assert_eq!(1035, IrOpcode::LAST_U16);
  assert_eq!(IrOpcode::LAST, IrOpcode::JSDebugger);
  assert_eq!(0, IrOpcode::Start as u16);
  assert_eq!(21, IrOpcode::End as u16);
  // 各顶层段首
  assert!((IrOpcode::ExternalConstant as u16) > (IrOpcode::End as u16)); // COMMON 段在后
  info!("count ok: {}", IrOpcode::COUNT);
  OK
}

/// 构建配置抽样: wasm 开 / SIMD256 关 / 硬件沙箱关 / CPED 开
#[test]
fn ir_opcode_test_build_config() -> Void {
  assert_eq!(
    Some(IrOpcode::JSWasmCall),
    IrOpcode::from_repr(IrOpcode::JSWasmCall as u16)
  );
  assert!(IrOpcode::from_repr(9999).is_none());
  // SIMD128 开: F64x2Splat 在; SIMD256 关: F64x4Min 不在 (编译期即无此变体)
  assert!(IrOpcode::F64x2Splat.is_simd128());
  assert!(IrOpcode::I8x16Splat.is_simd128()); // SIMD128 表中的无条件项
  // CPED 开: 两变体存在且属 simplified 段
  assert!(
    (IrOpcode::GetContinuationPreservedEmbedderData as u16) > (IrOpcode::StaticAssert as u16)
  );
  assert!((IrOpcode::SetContinuationPreservedEmbedderData as u16) < (IrOpcode::Word32Clz as u16));
  OK
}

/// phi / merge / 投影 / 终结 / 内联 / 原子 / 反馈 / 机器常量 抽样
#[test]
fn ir_opcode_test_small_predicates() -> Void {
  assert!(IrOpcode::Phi.is_phi() && IrOpcode::EffectPhi.is_phi() && !IrOpcode::Merge.is_phi());
  assert!(IrOpcode::Merge.is_merge() && IrOpcode::Loop.is_merge() && !IrOpcode::Branch.is_merge());
  // IfTrue..IfDefault 区间含 IfSuccess/IfException/IfValue (原版区间语义)
  assert!(IrOpcode::IfTrue.is_if_projection() && IrOpcode::IfDefault.is_if_projection());
  assert!(IrOpcode::IfSuccess.is_if_projection() && IrOpcode::IfException.is_if_projection());
  assert!(!IrOpcode::Merge.is_if_projection());
  assert!((IrOpcode::IfTrue as u16) < (IrOpcode::IfFalse as u16));
  assert!((IrOpcode::IfValue as u16) < (IrOpcode::IfDefault as u16));
  assert!(IrOpcode::Return.is_graph_terminator() && IrOpcode::Throw.is_graph_terminator());
  assert!(!IrOpcode::End.is_graph_terminator());
  assert!(IrOpcode::JSCall.is_inlinee() && IrOpcode::JSConstruct.is_inlinee());
  assert!(IrOpcode::Word32AtomicLoad.is_atomic_op() && !IrOpcode::Word32And.is_atomic_op());
  assert!(
    IrOpcode::Int32Constant.is_machine_constant() && !IrOpcode::HeapConstant.is_machine_constant()
  );
  assert!(IrOpcode::JSAdd.is_feedback_collecting() && IrOpcode::JSCall.is_feedback_collecting());
  assert!(!IrOpcode::JSCallRuntime.is_feedback_collecting());
  assert_eq!(
    11,
    IrOpcode::iter()
      .filter(|o| o.is_basic_block_begin())
      .count()
  );
  assert!(OPCODE_PROPS[IrOpcode::Start as usize] & OP_BASIC_BLOCK_BEGIN != 0);
  OK
}

/// FrameState 输入面 (operator-properties.cc HasFrameStateInput 的 opcode 面)
#[test]
fn ir_opcode_test_frame_state_input() -> Void {
  let count = IrOpcode::iter()
    .filter(|o| o.has_frame_state_input())
    .count();
  assert_eq!(92, count, "HasFrameStateInput 真组计数 (脚本对数 92)");
  assert!(IrOpcode::JSAdd.has_frame_state_input());
  assert!(
    IrOpcode::Checkpoint.has_frame_state_input() && IrOpcode::FrameState.has_frame_state_input()
  );
  assert!(!IrOpcode::JSStrictEqual.has_frame_state_input());
  assert!(!IrOpcode::Start.has_frame_state_input());
  assert!(IrOpcode::JSCallRuntime.frame_state_input_needs_param());
  assert!(!IrOpcode::JSCallRuntime.has_frame_state_input());
  assert_eq!(
    OPCODE_PROPS[IrOpcode::JSAdd as usize] & OP_FRAME_STATE_INPUT,
    OP_FRAME_STATE_INPUT
  );
  OK
}

/// NeedsExactContext 四组分区 (69/30/2/23; 全量计数)
#[test]
fn ir_opcode_test_exact_context_partition() -> Void {
  let mut any_c = 0;
  let mut exact = 0;
  let mut param = 0;
  let mut unreachable = 0;
  for op in IrOpcode::iter() {
    match op.exact_context() {
      ExactContext::AnyContext => any_c += 1,
      ExactContext::Exact => exact += 1,
      ExactContext::NeedsParam => param += 1,
      ExactContext::Unreachable => unreachable += 1,
      ExactContext::NotJs => {}
    }
    assert_eq!(
      op.exact_context() == ExactContext::Exact,
      OPCODE_PROPS[op as usize] & OP_EXACT_CONTEXT_TRUE != 0,
      "{}",
      op.mnemonic()
    );
    assert_eq!(
      op.exact_context() == ExactContext::NeedsParam,
      OPCODE_PROPS[op as usize] & OP_EXACT_CONTEXT_PARAM != 0,
      "{}",
      op.mnemonic()
    );
  }
  assert_eq!((any_c, exact, param, unreachable), (69, 30, 2, 23));
  OK
}

/// 顶层段计数 (与 C 预处理器金标准一致)
#[test]
fn ir_opcode_test_top_level_counts() -> Void {
  // is_common 区间 = CONTROL(22) + COMMON(47) = 69
  assert_eq!(22, IrOpcode::iter().filter(|o| o.is_control()).count());
  assert_eq!(69, IrOpcode::iter().filter(|o| o.is_common()).count());
  assert_eq!(124, IrOpcode::iter().filter(|o| o.is_js()).count());
  assert_eq!(287, IrOpcode::iter().filter(|o| o.is_simd128()).count());
  assert_eq!(27, IrOpcode::iter().filter(|o| o.is_atomic_op()).count());
  assert_eq!(
    8,
    IrOpcode::iter().filter(|o| o.is_machine_constant()).count()
  );
  assert_eq!(
    51,
    IrOpcode::iter()
      .filter(|o| o.is_feedback_collecting())
      .count()
  );
  OK
}

/// OP_* 位常量正交性 (无重叠别名)
#[test]
fn ir_opcode_test_prop_bits_orthogonal() -> Void {
  // 段位互斥: CONTROL 与 COMMON 可并存 (common 区间含 control), JS/SIMPLIFIED/MACHINE 与段位互斥
  for op in IrOpcode::iter() {
    let p = OPCODE_PROPS[op as usize];
    let js = p & OP_JS != 0;
    let simd = p & OP_SIMD128 != 0;
    assert!(!(js && simd), "{} 同时 JS 与 SIMD128", op.mnemonic());
    assert_eq!(op.is_graph_terminator(), p & OP_GRAPH_TERMINATOR != 0);
    assert_eq!(op.is_inlinee(), p & OP_INLINEE != 0);
    assert_eq!(op.is_merge(), p & OP_MERGE != 0);
    assert_eq!(op.is_phi(), p & OP_PHI != 0);
    assert!(OP_ATOMIC.count_ones() == 1);
  }
  OK
}
