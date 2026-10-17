//! operator.h/cc + operator-properties.h/cc 单测:
//! Properties 位/组合/ZeroIf*、Operator 计数与校验、
//! OperatorProperties 输入数计算 (value/effect/control/context/frame-state/total/bbb/needs_exact_context)

use aok::{OK, Void};
use log::info;
use v8_compiler_graph::{
  Error, ExactContext, IrOpcode, Operator, OperatorCounts, OperatorProperties, Properties,
};

#[ctor::ctor(unsafe)]
fn _log_init() {
  log_init::init();
}

/// 属性位与组合常量 (operator.h:40-54 逐位)
#[test]
fn operator_test_property_bits() -> Void {
  assert_eq!(1, Properties::COMMUTATIVE.bits());
  assert_eq!(2, Properties::ASSOCIATIVE.bits());
  assert_eq!(4, Properties::IDEMPOTENT.bits());
  assert_eq!(8, Properties::NO_READ.bits());
  assert_eq!(0x10, Properties::NO_WRITE.bits());
  assert_eq!(0x20, Properties::NO_THROW.bits());
  assert_eq!(0x40, Properties::NO_DEOPT.bits());
  // 组合 (kFoldable/kEliminatable/kKontrol/kPure)
  assert_eq!(
    Properties::NO_READ | Properties::NO_WRITE,
    Properties::FOLDABLE
  );
  assert_eq!(
    Properties::NO_DEOPT | Properties::NO_WRITE | Properties::NO_THROW,
    Properties::ELIMINATABLE
  );
  assert_eq!(
    Properties::NO_DEOPT | Properties::FOLDABLE | Properties::NO_THROW,
    Properties::KONTROL
  );
  assert_eq!(
    Properties::KONTROL | Properties::IDEMPOTENT,
    Properties::PURE
  );
  // contains = HasProperty 语义
  assert!(Properties::PURE.contains(Properties::FOLDABLE));
  assert!(!Properties::FOLDABLE.contains(Properties::COMMUTATIVE));
  assert!(Properties::NONE.bits() == 0);
  info!("props ok");
  OK
}

/// ZeroIfEliminatable / ZeroIfNoThrow / ZeroIfPure (operator.h:110-120)
#[test]
fn operator_test_zero_if() -> Void {
  assert_eq!(0, Properties::ELIMINATABLE.zero_if_eliminatable());
  assert_eq!(1, Properties::NONE.zero_if_eliminatable());
  assert_eq!(0, Properties::NO_THROW.zero_if_no_throw());
  assert_eq!(2, Properties::NONE.zero_if_no_throw());
  assert_eq!(0, Properties::PURE.zero_if_pure());
  assert_eq!(1, Properties::COMMUTATIVE.zero_if_pure());
  OK
}

/// Operator 构造 + 计数访问 (operator.cc:27-38; CheckRange → Err)
#[test]
fn operator_test_construction_and_counts() -> Void {
  let op = Operator::new(
    IrOpcode::Phi as u16,
    Properties::ASSOCIATIVE,
    "Phi",
    OperatorCounts::new(2, 1, 1, 1, 1, 1),
  )?;
  assert_eq!(IrOpcode::Phi as u16, op.opcode());
  assert_eq!("Phi", op.mnemonic());
  assert_eq!(2, op.value_input_count());
  assert_eq!(1, op.effect_input_count());
  assert_eq!(1, op.control_input_count());
  assert_eq!(1, op.value_output_count());
  assert_eq!(1, op.effect_output_count());
  assert_eq!(1, op.control_output_count());
  assert!(op.has_property(Properties::ASSOCIATIVE));
  // 越界 opcode → Err (Rust 加强: 原版仅 DCHECK)
  assert!(matches!(
    Operator::new(9999, Properties::NONE, "X", OperatorCounts::default()),
    Err(Error::OpcodeOutOfRange { .. })
  ));
  // effect_out 超 u8 → Err (原版 CheckRange<uint8_t>)
  assert!(matches!(
    Operator::new(
      0,
      Properties::NONE,
      "X",
      OperatorCounts::new(0, 0, 0, 0, 300, 0)
    ),
    Err(Error::CountOutOfRange { .. })
  ));
  // equals/hash 基类 = opcode 面
  let same = Operator::new(
    IrOpcode::Phi as u16,
    Properties::NONE,
    "Phi2",
    OperatorCounts::new(1, 0, 0, 1, 0, 0),
  )?;
  assert!(op.equals(&same)); // 仅 opcode
  assert_eq!(op.hash_code(), op.hash_code());
  OK
}

/// props_str — 属性名逗号串 (operator.cc PrintPropsTo, OPERATOR_PROPERTY_LIST 序)
#[test]
fn operator_test_props_str() -> Void {
  let op = Operator::new(
    0,
    Properties::COMMUTATIVE | Properties::NO_THROW,
    "X",
    OperatorCounts::default(),
  )?;
  assert_eq!("Commutative, NoThrow", op.props_str());
  assert_eq!(
    "",
    Operator::new(0, Properties::NONE, "X", OperatorCounts::default())?.props_str()
  );
  OK
}

/// HasContextInput / GetContextInputCount — ⟺ IsJsOpcode (operator-properties.cc:17-20)
#[test]
fn op_props_test_context_input() -> Void {
  let js = Operator::new(
    IrOpcode::JSAdd as u16,
    Properties::NONE,
    "JSAdd",
    OperatorCounts::new(2, 1, 1, 1, 1, 2),
  )?;
  let machine = Operator::new(
    IrOpcode::Int32Add as u16,
    Properties::NONE,
    "Add",
    OperatorCounts::new(2, 0, 0, 1, 0, 0),
  )?;
  assert!(OperatorProperties::has_context_input(&js));
  assert_eq!(1, OperatorProperties::get_context_input_count(&js));
  assert!(!OperatorProperties::has_context_input(&machine));
  assert_eq!(0, OperatorProperties::get_context_input_count(&machine));
  OK
}

/// HasFrameStateInput / GetFrameStateInputCount (operator-properties.cc:146-277)
#[test]
fn op_props_test_frame_state_input() -> Void {
  let mk = |oc: IrOpcode| {
    Operator::new(
      oc as u16,
      Properties::NONE,
      "x",
      OperatorCounts::new(0, 1, 1, 1, 1, 1),
    )
    .unwrap()
  };
  // 显式真组
  for oc in [
    IrOpcode::Checkpoint,
    IrOpcode::FrameState,
    IrOpcode::JSSetPrototypeProperties,
    IrOpcode::JSAdd,
    IrOpcode::JSCall,
    IrOpcode::JSGetIterator,
  ] {
    assert!(
      OperatorProperties::has_frame_state_input(&mk(oc))?,
      "{oc:?}"
    );
    assert_eq!(1, OperatorProperties::get_frame_state_input_count(&mk(oc))?);
  }
  // 显式假组 (严格相等不能 lazy deopt)
  assert!(!OperatorProperties::has_frame_state_input(&mk(
    IrOpcode::JSStrictEqual
  ))?);
  // 默认假 (非 JS)
  assert!(!OperatorProperties::has_frame_state_input(&mk(
    IrOpcode::Int32Add
  ))?);
  assert!(!OperatorProperties::has_frame_state_input(&mk(
    IrOpcode::Start
  ))?);
  // 参数化推迟: JSCallRuntime
  assert!(matches!(
    OperatorProperties::has_frame_state_input(&mk(IrOpcode::JSCallRuntime)),
    Err(Error::NeedsOpParam { .. })
  ));
  OK
}

/// GetTotalInputCount — value + context + frame_state + effect + control (operator-properties.cc:281-285)
#[test]
fn op_props_test_total_input_count() -> Void {
  // JSAdd: value 2 + context 1 + frame_state 1 + effect 1 + control 1 = 6
  let js_add = Operator::new(
    IrOpcode::JSAdd as u16,
    Properties::NONE,
    "JSAdd",
    OperatorCounts::new(2, 1, 1, 1, 1, 2),
  )?;
  assert_eq!(6, OperatorProperties::get_total_input_count(&js_add)?);
  // JSStrictEqual: value 2 + context 1 + frame_state 0 + effect 1 + control 1 = 5
  let eq = Operator::new(
    IrOpcode::JSStrictEqual as u16,
    Properties::NONE,
    "Eq",
    OperatorCounts::new(2, 1, 1, 1, 1, 2),
  )?;
  assert_eq!(5, OperatorProperties::get_total_input_count(&eq)?);
  // 机器: Word32Add 2 + 0 + 0 + 0 + 0 = 2
  let add = Operator::new(
    IrOpcode::Int32Add as u16,
    Properties::PURE,
    "Add",
    OperatorCounts::new(2, 0, 0, 1, 0, 0),
  )?;
  assert_eq!(2, OperatorProperties::get_total_input_count(&add)?);
  OK
}

/// IsBasicBlockBegin (operator-properties.cc:289-297)
#[test]
fn op_props_test_basic_block_begin() -> Void {
  let mk = |oc: IrOpcode| {
    Operator::new(oc as u16, Properties::NONE, "x", OperatorCounts::default()).unwrap()
  };
  for oc in [
    IrOpcode::Start,
    IrOpcode::End,
    IrOpcode::Dead,
    IrOpcode::Loop,
    IrOpcode::Merge,
    IrOpcode::IfTrue,
    IrOpcode::IfFalse,
    IrOpcode::IfSuccess,
    IrOpcode::IfException,
    IrOpcode::IfValue,
    IrOpcode::IfDefault,
  ] {
    assert!(OperatorProperties::is_basic_block_begin(&mk(oc)), "{oc:?}");
  }
  assert!(!OperatorProperties::is_basic_block_begin(&mk(
    IrOpcode::Branch
  )));
  assert!(!OperatorProperties::is_basic_block_begin(&mk(
    IrOpcode::JSAdd
  )));
  OK
}

/// NeedsExactContext 四组 (operator-properties.cc:23-143)
#[test]
fn op_props_test_needs_exact_context() -> Void {
  let mk = |oc: IrOpcode| {
    Operator::new(oc as u16, Properties::NONE, "x", OperatorCounts::default()).unwrap()
  };
  // 精确组
  for oc in [
    IrOpcode::JSDebugger,
    IrOpcode::JSLoadContext,
    IrOpcode::JSCreateClosure,
    IrOpcode::JSGeneratorStore,
  ] {
    assert!(OperatorProperties::needs_exact_context(&mk(oc))?, "{oc:?}");
  }
  // 任意组 (二元/一元/调用/构造 + 显式)
  for oc in [
    IrOpcode::JSAdd,
    IrOpcode::JSCall,
    IrOpcode::JSConstruct,
    IrOpcode::JSCloneObject,
    IrOpcode::JSGetIterator,
  ] {
    assert!(!OperatorProperties::needs_exact_context(&mk(oc))?, "{oc:?}");
  }
  // 参数化组
  for oc in [IrOpcode::JSCallRuntime, IrOpcode::JSCreateArguments] {
    assert!(
      matches!(
        OperatorProperties::needs_exact_context(&mk(oc)),
        Err(Error::NeedsOpParam { .. })
      ),
      "{oc:?}"
    );
  }
  // lazy 组 + 非 JS → Unreachable (原版 UNREACHABLE)
  assert!(matches!(
    OperatorProperties::needs_exact_context(&mk(IrOpcode::JSCreatePromise)),
    Err(Error::Unreachable { .. })
  ));
  assert!(matches!(
    OperatorProperties::needs_exact_context(&mk(IrOpcode::Start)),
    Err(Error::Unreachable { .. })
  ));
  OK
}

/// opcode 面与 ExactContext 枚举一致
#[test]
fn op_props_test_exact_context_enum() -> Void {
  assert_eq!(ExactContext::Exact, IrOpcode::JSDebugger.exact_context());
  assert_eq!(ExactContext::AnyContext, IrOpcode::JSAdd.exact_context());
  assert_eq!(
    ExactContext::NeedsParam,
    IrOpcode::JSCallRuntime.exact_context()
  );
  assert_eq!(
    ExactContext::Unreachable,
    IrOpcode::JSCreatePromise.exact_context()
  );
  assert_eq!(ExactContext::NotJs, IrOpcode::Start.exact_context());
  OK
}
