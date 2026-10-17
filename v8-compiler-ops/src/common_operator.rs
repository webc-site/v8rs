// 由 tool/src/compiler/common-operator.h.js 从 v8/src/compiler/common-operator.h + common-operator.cc 生成; 修订就地改, 复跑覆盖

//! CommonOperatorBuilder 与 Common 算子族
//!
//! 对应原版 common-operator.h / common-operator.cc。
//! 包含静态单例缓存与 Zone 分配器，覆盖 Dead, Start, End, Parameter, Branch,
//! Switch, IfTrue, IfFalse, Return, Deoptimize, EffectPhi, Loop, Merge 等核心算子。

use core::result::Result;
use std::sync::LazyLock;

use v8_compiler_graph::{
  FeedbackSource, IrOpcode, MachineRepresentation, Operator, Operator1, OperatorCounts, Properties,
};
use v8_zone::Zone;

use crate::Error;

/// 分支语义 (BranchSemantics)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(u8)]
pub enum BranchSemantics {
  JS = 0,
  Machine = 1,
  #[default]
  Unspecified = 2,
}

/// 分支预测提示 (BranchHint)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(u8)]
pub enum BranchHint {
  #[default]
  None = 0,
  True = 1,
  False = 2,
}

impl BranchHint {
  /// 反转分支提示
  pub const fn negate(self) -> Self {
    match self {
      Self::None => Self::None,
      Self::True => Self::False,
      Self::False => Self::True,
    }
  }
}

/// 分支算子参数 (BranchParameters)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct BranchParameters {
  pub semantics: BranchSemantics,
  pub hint: BranchHint,
}

impl BranchParameters {
  pub const fn new(semantics: BranchSemantics, hint: BranchHint) -> Self {
    Self { semantics, hint }
  }
}

/// Deoptimize 原因枚举 (由 deoptimize-reason.h 抽取)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum DeoptimizeReason {
  OutOfBounds,
  ArrayBufferWasDetached,
  ArrayLengthChanged,
  BigIntTooBig,
  CouldNotGrowElements,
  CowArrayElementsChanged,
  DeoptimizeNow,
  DivisionByZero,
  Hole,
  HoleOrUndefined,
  InstanceMigrationFailed,
  InsufficientTypeFeedbackForCall,
  InsufficientTypeFeedbackForForIn,
  KeyedAccessChanged,
  LoopSpeculationFailed,
  LostPrecision,
  LostPrecisionOrNaN,
  MinusZero,
  NoInitialElement,
  NotABigInt,
  NotABigInt64,
  NotAHeapNumber,
  NotAJavaScriptObject,
  NotANumber,
  NotANumberOrBoolean,
  NotANumberOrUndefined,
  NotANumberOrOddball,
  NotASmi,
  NotAString,
  NotAStringOrStringWrapper,
  NotAStringOrOddball,
  NotASymbol,
  NotAdditiveSafeInteger,
  NotAnArrayIndex,
  NotDetectableReceiver,
  NotInt32,
  NotUint32,
  OSREarlyExit,
  Overflow,
  PrepareForOnStackReplacement,
  Smi,
  StoreToConstant,
  StringTooLarge,
  UnexpectedContextExtension,
  Unknown,
  ValueMismatch,
  WrongCallTarget,
  WrongConstructor,
  WrongEnumIndices,
  WrongFeedbackCell,
  WrongInstanceType,
  WrongMap,
  WrongMapDynamic,
  WrongName,
  WrongValue,
  MapDeprecated,
  PrototypeChange,
  PropertyCellChange,
  FieldTypeConstChange,
  FieldTypeChange,
  FieldRepresentationChange,
  InitialMapChange,
  ContextCellChange,
  EmptyContextExtensionChange,
  WeakObjects,
  Debugger,
  Testing,
  ExceptionCaught,
  EagerDeopt,
  FrameValueMaterialized,
}

/// Deoptimize 算子参数 (DeoptimizeParameters)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DeoptimizeParameters {
  pub reason: DeoptimizeReason,
  pub feedback: FeedbackSource,
}

impl DeoptimizeParameters {
  pub const fn new(reason: DeoptimizeReason, feedback: FeedbackSource) -> Self {
    Self { reason, feedback }
  }
}

/// Parameter 算子参数
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ParameterInfo {
  pub index: i32,
  pub debug_name: Option<&'static str>,
}

impl ParameterInfo {
  pub const fn new(index: i32, debug_name: Option<&'static str>) -> Self {
    Self { index, debug_name }
  }
}

/// IfValue 算子参数
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct IfValueParameters {
  pub value: i32,
  pub comparison_order: i32,
  pub hint: BranchHint,
}

impl IfValueParameters {
  pub const fn new(value: i32, comparison_order: i32, hint: BranchHint) -> Self {
    Self {
      value,
      comparison_order,
      hint,
    }
  }
}

/// 全局共享 Common 算子缓存 (镜像原版 CommonOperatorGlobalCache)
pub struct CommonOperatorGlobalCache {
  pub plug: Operator,
  pub dead: Operator,
  pub unreachable: Operator,
  pub if_true: Operator,
  pub if_false: Operator,
  pub if_success: Operator,
  pub if_exception: Operator,
  pub throw: Operator,
  pub terminate: Operator,
  pub loop_exit: Operator,
  pub loop_exit_effect: Operator,
  pub checkpoint: Operator,
  pub finish_region: Operator,
  pub retain: Operator,
  // 常用分支
  pub branch_js_none: Operator1<BranchParameters>,
  pub branch_js_true: Operator1<BranchParameters>,
  pub branch_js_false: Operator1<BranchParameters>,
  pub branch_machine_none: Operator1<BranchParameters>,
  pub branch_machine_true: Operator1<BranchParameters>,
  pub branch_machine_false: Operator1<BranchParameters>,
  pub branch_unspecified_none: Operator1<BranchParameters>,
  pub branch_unspecified_true: Operator1<BranchParameters>,
  pub branch_unspecified_false: Operator1<BranchParameters>,
  // 常用返回
  pub return_1: Operator,
  pub return_2: Operator,
  pub return_3: Operator,
  pub return_4: Operator,
  // 常用 End
  pub end_1: Operator,
  pub end_2: Operator,
  pub end_3: Operator,
  pub end_4: Operator,
  // 常用 EffectPhi
  pub effect_phi_1: Operator,
  pub effect_phi_2: Operator,
  pub effect_phi_3: Operator,
  // 常用 Loop
  pub loop_1: Operator1<FeedbackSource>,
  pub loop_2: Operator1<FeedbackSource>,
  // 常用 Merge
  pub merge_1: Operator,
  pub merge_2: Operator,
  pub merge_3: Operator,
  pub merge_4: Operator,
}

#[inline]
fn unwrap_op<T>(res: Result<T, v8_compiler_graph::Error>) -> T {
  match res {
    Ok(v) => v,
    Err(_) => unreachable!(),
  }
}

impl CommonOperatorGlobalCache {
  fn new() -> Self {
    Self {
      plug: unwrap_op(Operator::new(
        IrOpcode::Plug as u16,
        Properties::NONE,
        "Plug",
        OperatorCounts::new(0, 0, 0, 1, 0, 0),
      )),
      dead: unwrap_op(Operator::new(
        IrOpcode::Dead as u16,
        Properties::FOLDABLE,
        "Dead",
        OperatorCounts::new(0, 0, 0, 1, 1, 1),
      )),
      unreachable: unwrap_op(Operator::new(
        IrOpcode::Unreachable as u16,
        Properties::FOLDABLE,
        "Unreachable",
        OperatorCounts::new(0, 1, 1, 1, 1, 0),
      )),
      if_true: unwrap_op(Operator::new(
        IrOpcode::IfTrue as u16,
        Properties::KONTROL,
        "IfTrue",
        OperatorCounts::new(0, 0, 1, 0, 0, 1),
      )),
      if_false: unwrap_op(Operator::new(
        IrOpcode::IfFalse as u16,
        Properties::KONTROL,
        "IfFalse",
        OperatorCounts::new(0, 0, 1, 0, 0, 1),
      )),
      if_success: unwrap_op(Operator::new(
        IrOpcode::IfSuccess as u16,
        Properties::KONTROL,
        "IfSuccess",
        OperatorCounts::new(0, 0, 1, 0, 0, 1),
      )),
      if_exception: unwrap_op(Operator::new(
        IrOpcode::IfException as u16,
        Properties::KONTROL,
        "IfException",
        OperatorCounts::new(0, 1, 1, 1, 1, 1),
      )),
      throw: unwrap_op(Operator::new(
        IrOpcode::Throw as u16,
        Properties::KONTROL,
        "Throw",
        OperatorCounts::new(0, 1, 1, 0, 0, 1),
      )),
      terminate: unwrap_op(Operator::new(
        IrOpcode::Terminate as u16,
        Properties::KONTROL,
        "Terminate",
        OperatorCounts::new(0, 1, 1, 0, 0, 1),
      )),
      loop_exit: unwrap_op(Operator::new(
        IrOpcode::LoopExit as u16,
        Properties::KONTROL,
        "LoopExit",
        OperatorCounts::new(0, 0, 2, 0, 0, 1),
      )),
      loop_exit_effect: unwrap_op(Operator::new(
        IrOpcode::LoopExitEffect as u16,
        Properties::NO_THROW,
        "LoopExitEffect",
        OperatorCounts::new(0, 1, 1, 0, 1, 0),
      )),
      checkpoint: unwrap_op(Operator::new(
        IrOpcode::Checkpoint as u16,
        Properties::KONTROL,
        "Checkpoint",
        OperatorCounts::new(0, 1, 1, 0, 1, 0),
      )),
      finish_region: unwrap_op(Operator::new(
        IrOpcode::FinishRegion as u16,
        Properties::KONTROL,
        "FinishRegion",
        OperatorCounts::new(1, 1, 0, 1, 1, 0),
      )),
      retain: unwrap_op(Operator::new(
        IrOpcode::Retain as u16,
        Properties::KONTROL,
        "Retain",
        OperatorCounts::new(1, 1, 0, 0, 1, 0),
      )),
      branch_js_none: Self::make_branch(BranchSemantics::JS, BranchHint::None),
      branch_js_true: Self::make_branch(BranchSemantics::JS, BranchHint::True),
      branch_js_false: Self::make_branch(BranchSemantics::JS, BranchHint::False),
      branch_machine_none: Self::make_branch(BranchSemantics::Machine, BranchHint::None),
      branch_machine_true: Self::make_branch(BranchSemantics::Machine, BranchHint::True),
      branch_machine_false: Self::make_branch(BranchSemantics::Machine, BranchHint::False),
      branch_unspecified_none: Self::make_branch(BranchSemantics::Unspecified, BranchHint::None),
      branch_unspecified_true: Self::make_branch(BranchSemantics::Unspecified, BranchHint::True),
      branch_unspecified_false: Self::make_branch(BranchSemantics::Unspecified, BranchHint::False),
      return_1: Self::make_return(1),
      return_2: Self::make_return(2),
      return_3: Self::make_return(3),
      return_4: Self::make_return(4),
      end_1: Self::make_end(1),
      end_2: Self::make_end(2),
      end_3: Self::make_end(3),
      end_4: Self::make_end(4),
      effect_phi_1: Self::make_effect_phi(1),
      effect_phi_2: Self::make_effect_phi(2),
      effect_phi_3: Self::make_effect_phi(3),
      loop_1: Self::make_loop(1),
      loop_2: Self::make_loop(2),
      merge_1: Self::make_merge(1),
      merge_2: Self::make_merge(2),
      merge_3: Self::make_merge(3),
      merge_4: Self::make_merge(4),
    }
  }

  fn make_branch(semantics: BranchSemantics, hint: BranchHint) -> Operator1<BranchParameters> {
    unwrap_op(Operator1::new(
      IrOpcode::Branch as u16,
      Properties::KONTROL,
      "Branch",
      OperatorCounts::new(1, 0, 1, 0, 0, 2),
      BranchParameters::new(semantics, hint),
    ))
  }

  fn make_return(value_input_count: u32) -> Operator {
    unwrap_op(Operator::new(
      IrOpcode::Return as u16,
      Properties::NO_THROW,
      "Return",
      OperatorCounts::new(value_input_count + 1, 1, 1, 0, 0, 1),
    ))
  }

  fn make_end(control_input_count: u32) -> Operator {
    unwrap_op(Operator::new(
      IrOpcode::End as u16,
      Properties::KONTROL,
      "End",
      OperatorCounts::new(0, 0, control_input_count, 0, 0, 0),
    ))
  }

  fn make_effect_phi(effect_input_count: u32) -> Operator {
    unwrap_op(Operator::new(
      IrOpcode::EffectPhi as u16,
      Properties::KONTROL,
      "EffectPhi",
      OperatorCounts::new(0, effect_input_count, 1, 0, 1, 0),
    ))
  }

  fn make_loop(control_input_count: u32) -> Operator1<FeedbackSource> {
    unwrap_op(Operator1::new(
      IrOpcode::Loop as u16,
      Properties::KONTROL,
      "Loop",
      OperatorCounts::new(0, 0, control_input_count, 0, 0, 1),
      FeedbackSource::invalid(),
    ))
  }

  fn make_merge(control_input_count: u32) -> Operator {
    unwrap_op(Operator::new(
      IrOpcode::Merge as u16,
      Properties::KONTROL,
      "Merge",
      OperatorCounts::new(0, 0, control_input_count, 0, 0, 1),
    ))
  }
}

unsafe impl Send for CommonOperatorGlobalCache {}
unsafe impl Sync for CommonOperatorGlobalCache {}

static COMMON_CACHE: LazyLock<CommonOperatorGlobalCache> =
  LazyLock::new(CommonOperatorGlobalCache::new);

/// CommonOperatorBuilder
pub struct CommonOperatorBuilder<'z, 'a> {
  zone: &'z Zone<'a>,
}

impl<'z, 'a> CommonOperatorBuilder<'z, 'a> {
  /// 创建构建器
  pub fn new(zone: &'z Zone<'a>) -> Self {
    Self { zone }
  }

  /// 底层 Zone 引用
  pub fn zone(&self) -> &'z Zone<'a> {
    self.zone
  }

  // —— 静态缓存算子 ——

  /// Plug
  #[inline]
  pub fn plug(&self) -> &'static Operator {
    &COMMON_CACHE.plug
  }

  /// Dead
  #[inline]
  pub fn dead(&self) -> &'static Operator {
    &COMMON_CACHE.dead
  }

  /// Unreachable
  #[inline]
  pub fn unreachable(&self) -> &'static Operator {
    &COMMON_CACHE.unreachable
  }

  /// IfTrue
  #[inline]
  pub fn if_true(&self) -> &'static Operator {
    &COMMON_CACHE.if_true
  }

  /// IfFalse
  #[inline]
  pub fn if_false(&self) -> &'static Operator {
    &COMMON_CACHE.if_false
  }

  /// IfSuccess
  #[inline]
  pub fn if_success(&self) -> &'static Operator {
    &COMMON_CACHE.if_success
  }

  /// IfException
  #[inline]
  pub fn if_exception(&self) -> &'static Operator {
    &COMMON_CACHE.if_exception
  }

  /// Throw
  #[inline]
  pub fn throw(&self) -> &'static Operator {
    &COMMON_CACHE.throw
  }

  /// Terminate
  #[inline]
  pub fn terminate(&self) -> &'static Operator {
    &COMMON_CACHE.terminate
  }

  /// LoopExit
  #[inline]
  pub fn loop_exit(&self) -> &'static Operator {
    &COMMON_CACHE.loop_exit
  }

  /// LoopExitEffect
  #[inline]
  pub fn loop_exit_effect(&self) -> &'static Operator {
    &COMMON_CACHE.loop_exit_effect
  }

  /// Checkpoint
  #[inline]
  pub fn checkpoint(&self) -> &'static Operator {
    &COMMON_CACHE.checkpoint
  }

  /// FinishRegion
  #[inline]
  pub fn finish_region(&self) -> &'static Operator {
    &COMMON_CACHE.finish_region
  }

  /// Retain
  #[inline]
  pub fn retain(&self) -> &'static Operator {
    &COMMON_CACHE.retain
  }

  /// Start 算子
  pub fn start(&self, value_output_count: u32) -> Result<&'z Operator, Error> {
    let op = Operator::new(
      IrOpcode::Start as u16,
      Properties::FOLDABLE | Properties::NO_THROW,
      "Start",
      OperatorCounts::new(0, 0, 0, value_output_count, 1, 1),
    )?;
    Ok(self.zone.make(op)?)
  }

  /// End 算子
  pub fn end(&self, control_input_count: u32) -> Result<&'z Operator, Error> {
    match control_input_count {
      1 => Ok(&COMMON_CACHE.end_1),
      2 => Ok(&COMMON_CACHE.end_2),
      3 => Ok(&COMMON_CACHE.end_3),
      4 => Ok(&COMMON_CACHE.end_4),
      _ => {
        let op = Operator::new(
          IrOpcode::End as u16,
          Properties::KONTROL,
          "End",
          OperatorCounts::new(0, 0, control_input_count, 0, 0, 0),
        )?;
        Ok(self.zone.make(op)?)
      }
    }
  }

  /// Branch 算子
  pub fn branch(&self, hint: BranchHint, semantics: BranchSemantics) -> &'static Operator {
    match (semantics, hint) {
      (BranchSemantics::JS, BranchHint::None) => &COMMON_CACHE.branch_js_none.base,
      (BranchSemantics::JS, BranchHint::True) => &COMMON_CACHE.branch_js_true.base,
      (BranchSemantics::JS, BranchHint::False) => &COMMON_CACHE.branch_js_false.base,
      (BranchSemantics::Machine, BranchHint::None) => &COMMON_CACHE.branch_machine_none.base,
      (BranchSemantics::Machine, BranchHint::True) => &COMMON_CACHE.branch_machine_true.base,
      (BranchSemantics::Machine, BranchHint::False) => &COMMON_CACHE.branch_machine_false.base,
      (BranchSemantics::Unspecified, BranchHint::None) => {
        &COMMON_CACHE.branch_unspecified_none.base
      }
      (BranchSemantics::Unspecified, BranchHint::True) => {
        &COMMON_CACHE.branch_unspecified_true.base
      }
      (BranchSemantics::Unspecified, BranchHint::False) => {
        &COMMON_CACHE.branch_unspecified_false.base
      }
    }
  }

  /// Parameter 算子 (值输入 1 即 Start 节点)
  pub fn parameter(
    &self,
    index: i32,
    debug_name: Option<&'static str>,
  ) -> Result<&'z Operator, Error> {
    let op = Operator1::new(
      IrOpcode::Parameter as u16,
      Properties::PURE,
      "Parameter",
      OperatorCounts::new(1, 0, 0, 1, 0, 0),
      ParameterInfo::new(index, debug_name),
    )?;
    let allocated = self.zone.make(op)?;
    Ok(&allocated.base)
  }

  /// Switch 算子
  pub fn switch_op(&self, control_output_count: u32) -> Result<&'z Operator, Error> {
    let op = Operator::new(
      IrOpcode::Switch as u16,
      Properties::KONTROL,
      "Switch",
      OperatorCounts::new(1, 0, 1, 0, 0, control_output_count),
    )?;
    Ok(self.zone.make(op)?)
  }

  /// Return 算子
  pub fn return_op(&self, value_input_count: u32) -> Result<&'z Operator, Error> {
    match value_input_count {
      1 => Ok(&COMMON_CACHE.return_1),
      2 => Ok(&COMMON_CACHE.return_2),
      3 => Ok(&COMMON_CACHE.return_3),
      4 => Ok(&COMMON_CACHE.return_4),
      _ => {
        let op = Operator::new(
          IrOpcode::Return as u16,
          Properties::NO_THROW,
          "Return",
          OperatorCounts::new(value_input_count + 1, 1, 1, 0, 0, 1),
        )?;
        Ok(self.zone.make(op)?)
      }
    }
  }

  /// Deoptimize 算子
  pub fn deoptimize(
    &self,
    reason: DeoptimizeReason,
    feedback: FeedbackSource,
  ) -> Result<&'z Operator, Error> {
    let op = Operator1::new(
      IrOpcode::Deoptimize as u16,
      Properties::FOLDABLE | Properties::NO_THROW,
      "Deoptimize",
      OperatorCounts::new(1, 1, 1, 0, 0, 1),
      DeoptimizeParameters::new(reason, feedback),
    )?;
    let allocated = self.zone.make(op)?;
    Ok(&allocated.base)
  }

  /// DeoptimizeIf 算子
  pub fn deoptimize_if(
    &self,
    reason: DeoptimizeReason,
    feedback: FeedbackSource,
  ) -> Result<&'z Operator, Error> {
    let op = Operator1::new(
      IrOpcode::DeoptimizeIf as u16,
      Properties::FOLDABLE | Properties::NO_THROW,
      "DeoptimizeIf",
      OperatorCounts::new(2, 1, 1, 0, 1, 1),
      DeoptimizeParameters::new(reason, feedback),
    )?;
    let allocated = self.zone.make(op)?;
    Ok(&allocated.base)
  }

  /// DeoptimizeUnless 算子
  pub fn deoptimize_unless(
    &self,
    reason: DeoptimizeReason,
    feedback: FeedbackSource,
  ) -> Result<&'z Operator, Error> {
    let op = Operator1::new(
      IrOpcode::DeoptimizeUnless as u16,
      Properties::FOLDABLE | Properties::NO_THROW,
      "DeoptimizeUnless",
      OperatorCounts::new(2, 1, 1, 0, 1, 1),
      DeoptimizeParameters::new(reason, feedback),
    )?;
    let allocated = self.zone.make(op)?;
    Ok(&allocated.base)
  }

  /// EffectPhi 算子
  pub fn effect_phi(&self, effect_input_count: u32) -> Result<&'z Operator, Error> {
    match effect_input_count {
      1 => Ok(&COMMON_CACHE.effect_phi_1),
      2 => Ok(&COMMON_CACHE.effect_phi_2),
      3 => Ok(&COMMON_CACHE.effect_phi_3),
      _ => {
        let op = Operator::new(
          IrOpcode::EffectPhi as u16,
          Properties::KONTROL,
          "EffectPhi",
          OperatorCounts::new(0, effect_input_count, 1, 0, 1, 0),
        )?;
        Ok(self.zone.make(op)?)
      }
    }
  }

  /// Loop 算子
  pub fn loop_op(
    &self,
    control_input_count: u32,
    feedback: FeedbackSource,
  ) -> Result<&'z Operator, Error> {
    if !feedback.is_valid() {
      match control_input_count {
        1 => return Ok(&COMMON_CACHE.loop_1.base),
        2 => return Ok(&COMMON_CACHE.loop_2.base),
        _ => {}
      }
    }
    let op = Operator1::new(
      IrOpcode::Loop as u16,
      Properties::KONTROL,
      "Loop",
      OperatorCounts::new(0, 0, control_input_count, 0, 0, 1),
      feedback,
    )?;
    let allocated = self.zone.make(op)?;
    Ok(&allocated.base)
  }

  /// Merge 算子
  pub fn merge(&self, control_input_count: u32) -> Result<&'z Operator, Error> {
    match control_input_count {
      1 => Ok(&COMMON_CACHE.merge_1),
      2 => Ok(&COMMON_CACHE.merge_2),
      3 => Ok(&COMMON_CACHE.merge_3),
      4 => Ok(&COMMON_CACHE.merge_4),
      _ => {
        let op = Operator::new(
          IrOpcode::Merge as u16,
          Properties::KONTROL,
          "Merge",
          OperatorCounts::new(0, 0, control_input_count, 0, 0, 1),
        )?;
        Ok(self.zone.make(op)?)
      }
    }
  }

  /// Phi 算子
  pub fn phi(
    &self,
    rep: MachineRepresentation,
    value_input_count: u32,
  ) -> Result<&'z Operator, Error> {
    let op = Operator1::new(
      IrOpcode::Phi as u16,
      Properties::PURE,
      "Phi",
      OperatorCounts::new(value_input_count, 0, 1, 1, 0, 0),
      rep,
    )?;
    let allocated = self.zone.make(op)?;
    Ok(&allocated.base)
  }

  /// IfValue 算子
  pub fn if_value(
    &self,
    value: i32,
    comparison_order: i32,
    hint: BranchHint,
  ) -> Result<&'z Operator, Error> {
    let op = Operator1::new(
      IrOpcode::IfValue as u16,
      Properties::KONTROL,
      "IfValue",
      OperatorCounts::new(0, 0, 1, 0, 0, 1),
      IfValueParameters::new(value, comparison_order, hint),
    )?;
    let allocated = self.zone.make(op)?;
    Ok(&allocated.base)
  }

  /// IfDefault 算子
  pub fn if_default(&self, hint: BranchHint) -> Result<&'z Operator, Error> {
    let op = Operator1::new(
      IrOpcode::IfDefault as u16,
      Properties::KONTROL,
      "IfDefault",
      OperatorCounts::new(0, 0, 1, 0, 0, 1),
      hint,
    )?;
    let allocated = self.zone.make(op)?;
    Ok(&allocated.base)
  }
}
