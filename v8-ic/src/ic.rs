use strum::{Display, EnumIter, EnumString};

use crate::error::Result;

/// 命名属性查找所有权类型 (bool 底, 来源: ic.h:22)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Display, EnumIter, EnumString)]
pub enum NamedPropertyType {
  #[strum(serialize = "kNotOwn")]
  NotOwn = 0,
  #[strum(serialize = "kOwn")]
  Own = 1,
}

impl NamedPropertyType {
  #[inline]
  pub const fn is_own(&self) -> bool {
    matches!(self, Self::Own)
  }
}

/// 调用栈帧类型 (u8 底, 来源: ic.h:24)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Display, EnumIter, EnumString)]
pub enum CallerFrameType {
  #[strum(serialize = "kAny")]
  Any = 0,
  #[strum(serialize = "kBaseline")]
  Baseline = 1,
}

impl CallerFrameType {
  #[inline]
  pub const fn is_baseline(&self) -> bool {
    matches!(self, Self::Baseline)
  }
}

/// 键值存储 Map 检查模式 (来源: ic.h:320)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Display, EnumIter, EnumString)]
pub enum KeyedStoreCheckMap {
  #[strum(serialize = "kDontCheckMap")]
  DontCheckMap = 0,
  #[strum(serialize = "kCheckMap")]
  CheckMap = 1,
}

impl KeyedStoreCheckMap {
  #[inline]
  pub const fn should_check_map(&self) -> bool {
    matches!(self, Self::CheckMap)
  }
}

/// 键值存储数组长度递增模式 (来源: ic.h:322)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Display, EnumIter, EnumString)]
pub enum KeyedStoreIncrementLength {
  #[strum(serialize = "kDontIncrementLength")]
  DontIncrementLength = 0,
  #[strum(serialize = "kIncrementLength")]
  IncrementLength = 1,
}

impl KeyedStoreIncrementLength {
  #[inline]
  pub const fn should_increment_length(&self) -> bool {
    matches!(self, Self::IncrementLength)
  }
}

/// 元素类型转换迁移模式 (来源: ic.h:324)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Display, EnumIter, EnumString)]
pub enum TransitionMode {
  #[strum(serialize = "kNoTransition")]
  NoTransition = 0,
  #[strum(serialize = "kTransitionToDouble")]
  TransitionToDouble = 1,
  #[strum(serialize = "kTransitionToObject")]
  TransitionToObject = 2,
}

impl TransitionMode {
  #[inline]
  pub const fn has_transition(&self) -> bool {
    !matches!(self, Self::NoTransition)
  }
}

/// 内联缓存生命周期状态 (对齐 V8 InlineCacheState)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Display, EnumIter, EnumString)]
pub enum InlineCacheState {
  Uninitialized,
  Premonomorphic,
  Monomorphic,
  RecomputeHandler,
  Polymorphic,
  Megamorphic,
  Generic,
}

impl InlineCacheState {
  #[inline]
  pub const fn is_monomorphic(&self) -> bool {
    matches!(self, Self::Monomorphic)
  }

  #[inline]
  pub const fn is_polymorphic(&self) -> bool {
    matches!(self, Self::Polymorphic)
  }

  #[inline]
  pub const fn is_megamorphic(&self) -> bool {
    matches!(self, Self::Megamorphic)
  }

  #[inline]
  pub const fn is_generic(&self) -> bool {
    matches!(self, Self::Generic)
  }
}

/// IC 体系分类种类
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Display, EnumIter, EnumString)]
pub enum IcKind {
  LoadIC,
  LoadGlobalIC,
  KeyedLoadIC,
  StoreIC,
  StoreGlobalIC,
  KeyedStoreIC,
  StoreInArrayLiteralIC,
}

/// IC 核心状态机模型 (来源: ic.h:29)
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IC {
  state: InlineCacheState,
  old_state: InlineCacheState,
  kind: IcKind,
  caller_frame_type: CallerFrameType,
}

impl IC {
  #[inline]
  pub const fn new(kind: IcKind, caller_frame_type: CallerFrameType) -> Self {
    Self {
      state: InlineCacheState::Uninitialized,
      old_state: InlineCacheState::Uninitialized,
      kind,
      caller_frame_type,
    }
  }

  #[inline]
  pub const fn state(&self) -> InlineCacheState {
    self.state
  }

  #[inline]
  pub const fn old_state(&self) -> InlineCacheState {
    self.old_state
  }

  #[inline]
  pub const fn kind(&self) -> IcKind {
    self.kind
  }

  #[inline]
  pub const fn caller_frame_type(&self) -> CallerFrameType {
    self.caller_frame_type
  }

  #[inline]
  pub const fn is_any_load(&self) -> bool {
    matches!(
      self.kind,
      IcKind::LoadIC | IcKind::LoadGlobalIC | IcKind::KeyedLoadIC
    )
  }

  #[inline]
  pub const fn is_any_store(&self) -> bool {
    matches!(
      self.kind,
      IcKind::StoreIC
        | IcKind::StoreGlobalIC
        | IcKind::KeyedStoreIC
        | IcKind::StoreInArrayLiteralIC
    )
  }

  /// 状态机跃迁更新
  pub fn update_state(&mut self, next_state: InlineCacheState) -> Result<()> {
    self.old_state = self.state;
    self.state = next_state;
    Ok(())
  }

  /// 标记需要重新计算 Handler
  pub fn mark_recompute_handler(&mut self) {
    self.old_state = self.state;
    self.state = InlineCacheState::RecomputeHandler;
  }
}

/// 反馈向量隔离窄 trait
pub trait IcFeedbackEnv {
  fn read_feedback_slot(&self, slot_id: usize) -> Option<u64>;
  fn write_feedback_slot(&mut self, slot_id: usize, val: u64);
}

/// Map 结构布局隔离窄 trait
pub trait IcMapEnv {
  fn instance_type(&self, map_address: u64) -> u16;
  fn is_deprecated(&self, map_address: u64) -> bool;
}
