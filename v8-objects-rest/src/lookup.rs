// 由 tool/src/objects/lookup.h.js 从 v8/src/objects/lookup.{h,-inl.h} 生成; 修订就地改, 复跑覆盖
// LookupIterator 配置位/状态机/拦截器状态枚举
//! LookupIterator: 属性查找迭代器的配置与状态机。

// ── Configuration 位标志 (原版 enum Configuration) ─────────────────────────
/// 拦截器位 (bit0)。
pub const K_INTERCEPTOR: u32 = 1 << 0;
/// 原型链位 (bit1)。
pub const K_PROTOTYPE_CHAIN: u32 = 1 << 1;
/// 便捷组合常量 (机械展开自原版表达式)。
pub const OWN_SKIP_INTERCEPTOR: u32 = 0x0; // 0
pub const OWN: u32 = 0x1; // kInterceptor
pub const PROTOTYPE_CHAIN_SKIP_INTERCEPTOR: u32 = 0x2; // kPrototypeChain
pub const PROTOTYPE_CHAIN: u32 = 0x3; // kPrototypeChain | kInterceptor
pub const DEFAULT: u32 = 0x3; // PROTOTYPE_CHAIN

// ── State 状态机 (原版 enum State, 顺序值; 共 12 态) ───────────────
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum State {
  /// NOT_FOUND
  NotFound = 0,
  /// STRING_LOOKUP_START_OBJECT
  StringLookupStartObject = 1,
  /// TYPED_ARRAY_INDEX_NOT_FOUND
  TypedArrayIndexNotFound = 2,
  /// ACCESS_CHECK
  AccessCheck = 3,
  /// INTERCEPTOR
  Interceptor = 4,
  /// JSPROXY
  Jsproxy = 5,
  /// ACCESSOR
  Accessor = 6,
  /// DATA
  Data = 7,
  /// WASM_OBJECT
  WasmObject = 8,
  /// MODULE_NAMESPACE
  ModuleNamespace = 9,
  /// TRANSITION
  Transition = 10,
  /// BEFORE_PROPERTY = INTERCEPTOR (显式别名)
  BeforeProperty = 4,
}

impl State {
  /// 顺序态总数 (不含显式别名 BEFORE_PROPERTY)。
  pub const COUNT: usize = 11;

  /// 数值反查 (BEFORE_PROPERTY 与 INTERCEPTOR 同值, 返回 INTERCEPTOR)。
  #[inline]
  #[must_use]
  pub const fn from_repr(v: u8) -> Option<Self> {
    match v {
      0 => Some(Self::NotFound),
      1 => Some(Self::StringLookupStartObject),
      2 => Some(Self::TypedArrayIndexNotFound),
      3 => Some(Self::AccessCheck),
      4 => Some(Self::Interceptor),
      5 => Some(Self::Jsproxy),
      6 => Some(Self::Accessor),
      7 => Some(Self::Data),
      8 => Some(Self::WasmObject),
      9 => Some(Self::ModuleNamespace),
      10 => Some(Self::Transition),
      _ => None,
    }
  }
}

// ── InterceptorState (原版 enum class, 顺序值) ─────────────────────────────
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum InterceptorState {
  /// kUninitialized
  Uninitialized = 0,
  /// kSkipNonMasking
  Skipnonmasking = 1,
  /// kSkipNonMaskingOwnProperty
  Skipnonmaskingownproperty = 2,
  /// kProcessNonMasking
  Processnonmasking = 3,
}

// ── ConcurrentLookupIterator::Result (三态) ────────────────────────────────
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum ConcurrentResult {
  /// kPresent
  Present = 0,
  /// kNotPresent
  Notpresent = 1,
  /// kGaveUp
  Gaveup = 2,
}

// —— 提取不变式断言 ——
const _: () = {
  assert!(K_INTERCEPTOR == 1 && K_PROTOTYPE_CHAIN == 2);
  assert!(PROTOTYPE_CHAIN == 3 && DEFAULT == 3 && OWN == 1);
  assert!(State::COUNT == 11);
  assert!(State::NotFound as u8 == 0);
  assert!(State::BeforeProperty as u8 == State::Interceptor as u8);
  assert!(State::Transition as u8 == 11);
};
