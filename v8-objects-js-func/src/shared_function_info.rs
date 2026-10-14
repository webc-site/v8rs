// v8-objects-js-func/src/shared_function_info.rs
// 对应原版 v8/src/objects/shared-function-info.h + shared-function-info-inl.h

//! SharedFunctionInfo: 可在多个 JSFunction 实例间共享的函数元数据对象，
//! 包含函数代码指针/字节码槽位、参数数量、作用域信息以及 flags 位域。

use std::{
  mem,
  sync::atomic::{AtomicI32, AtomicU16, AtomicU32, Ordering},
};

use v8_objects_core::{
  heap_object::HeapObject,
  map::Map,
  tagged::{Object, Tagged},
};

/// 函数种类枚举 (对应原版 objects/function-kind.h)
#[repr(u8)]
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum FunctionKind {
  NormalFunction = 0,
  Module = 1,
  ModuleWithTopLevelAwait = 2,
  BaseConstructor = 3,
  DefaultBaseConstructor = 4,
  DefaultDerivedConstructor = 5,
  DerivedConstructor = 6,
  GetterFunction = 7,
  StaticGetterFunction = 8,
  SetterFunction = 9,
  StaticSetterFunction = 10,
  ArrowFunction = 11,
  AsyncArrowFunction = 12,
  AsyncFunction = 13,
  AsyncConciseMethod = 14,
  StaticAsyncConciseMethod = 15,
  AsyncConciseGeneratorMethod = 16,
  StaticAsyncConciseGeneratorMethod = 17,
  AsyncGeneratorFunction = 18,
  GeneratorFunction = 19,
  ConciseGeneratorMethod = 20,
  StaticConciseGeneratorMethod = 21,
  ConciseMethod = 22,
  StaticConciseMethod = 23,
  ClassMembersInitializerFunction = 24,
  ClassMembersInitializerFunctionPrecededByStatic = 25,
  ClassStaticInitializerFunction = 26,
  ClassStaticInitializerFunctionPrecededByMember = 27,
  Invalid = 28,
}

impl FunctionKind {
  /// 是否为可构造函数
  #[inline]
  pub const fn is_constructable(self) -> bool {
    (self as u8) <= (Self::DerivedConstructor as u8)
  }

  /// 是否为类构造函数
  #[inline]
  pub const fn is_class_constructor(self) -> bool {
    let val = self as u8;
    val >= (Self::BaseConstructor as u8) && val <= (Self::DerivedConstructor as u8)
  }

  /// 是否为箭头函数
  #[inline]
  pub const fn is_arrow_function(self) -> bool {
    let val = self as u8;
    val >= (Self::ArrowFunction as u8) && val <= (Self::AsyncArrowFunction as u8)
  }

  /// 是否为异步函数
  #[inline]
  pub const fn is_async(self) -> bool {
    let val = self as u8;
    val >= (Self::AsyncArrowFunction as u8) && val <= (Self::AsyncGeneratorFunction as u8)
  }

  /// 是否为生成器函数
  #[inline]
  pub const fn is_generator(self) -> bool {
    let val = self as u8;
    val >= (Self::AsyncConciseGeneratorMethod as u8)
      && val <= (Self::StaticConciseGeneratorMethod as u8)
  }
  /// 从数字安全转换
  #[inline]
  pub const fn from_u8(value: u8) -> Self {
    match value {
      0 => Self::NormalFunction,
      1 => Self::Module,
      2 => Self::ModuleWithTopLevelAwait,
      3 => Self::BaseConstructor,
      4 => Self::DefaultBaseConstructor,
      5 => Self::DefaultDerivedConstructor,
      6 => Self::DerivedConstructor,
      7 => Self::GetterFunction,
      8 => Self::StaticGetterFunction,
      9 => Self::SetterFunction,
      10 => Self::StaticSetterFunction,
      11 => Self::ArrowFunction,
      12 => Self::AsyncArrowFunction,
      13 => Self::AsyncFunction,
      14 => Self::AsyncConciseMethod,
      15 => Self::StaticAsyncConciseMethod,
      16 => Self::AsyncConciseGeneratorMethod,
      17 => Self::StaticAsyncConciseGeneratorMethod,
      18 => Self::AsyncGeneratorFunction,
      19 => Self::GeneratorFunction,
      20 => Self::ConciseGeneratorMethod,
      21 => Self::StaticConciseGeneratorMethod,
      22 => Self::ConciseMethod,
      23 => Self::StaticConciseMethod,
      24 => Self::ClassMembersInitializerFunction,
      25 => Self::ClassMembersInitializerFunctionPrecededByStatic,
      26 => Self::ClassStaticInitializerFunction,
      27 => Self::ClassStaticInitializerFunctionPrecededByMember,
      _ => Self::Invalid,
    }
  }
}

/// 函数语法种类 (对应原版 v8/src/objects/function-syntax-kind.h)
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum FunctionSyntaxKind {
  AnonymousExpression = 0,
  NamedExpression = 1,
  Declaration = 2,
  AccessorOrMethod = 3,
  Wrapped = 4,
}

impl FunctionSyntaxKind {
  /// 从 u8 安全转换
  #[inline]
  pub const fn from_u8(value: u8) -> Self {
    match value {
      0 => Self::AnonymousExpression,
      1 => Self::NamedExpression,
      2 => Self::Declaration,
      3 => Self::AccessorOrMethod,
      4 => Self::Wrapped,
      _ => Self::AnonymousExpression,
    }
  }
}

// ── Flags 位域掩码与移位常量 (由 tool/src/objects/shared-function-info.h.js 提取) ──
pub const K_FUNCTION_KIND_SHIFT: u32 = 0;
pub const K_FUNCTION_KIND_MASK: u32 = 0x1F;

pub const K_IS_NATIVE_SHIFT: u32 = 5;
pub const K_IS_NATIVE_MASK: u32 = 1 << K_IS_NATIVE_SHIFT;

pub const K_IS_STRICT_SHIFT: u32 = 6;
pub const K_IS_STRICT_MASK: u32 = 1 << K_IS_STRICT_SHIFT;

pub const K_FUNCTION_SYNTAX_KIND_SHIFT: u32 = 7;
pub const K_FUNCTION_SYNTAX_KIND_MASK: u32 = 0x7 << K_FUNCTION_SYNTAX_KIND_SHIFT;

pub const K_IS_CLASS_CONSTRUCTOR_SHIFT: u32 = 10;
pub const K_IS_CLASS_CONSTRUCTOR_MASK: u32 = 1 << K_IS_CLASS_CONSTRUCTOR_SHIFT;

pub const K_HAS_DUPLICATE_PARAMETERS_SHIFT: u32 = 11;
pub const K_HAS_DUPLICATE_PARAMETERS_MASK: u32 = 1 << K_HAS_DUPLICATE_PARAMETERS_SHIFT;

pub const K_ALLOW_LAZY_COMPILATION_SHIFT: u32 = 12;
pub const K_ALLOW_LAZY_COMPILATION_MASK: u32 = 1 << K_ALLOW_LAZY_COMPILATION_SHIFT;

pub const K_FUNCTION_MAP_INDEX_SHIFT: u32 = 13;
pub const K_FUNCTION_MAP_INDEX_MASK: u32 = 0x1F << K_FUNCTION_MAP_INDEX_SHIFT;

pub const K_DISABLED_OPTIMIZATION_REASON_SHIFT: u32 = 18;
pub const K_DISABLED_OPTIMIZATION_REASON_MASK: u32 = 0x0F << K_DISABLED_OPTIMIZATION_REASON_SHIFT;

pub const K_REQUIRES_INSTANCE_MEMBERS_INITIALIZER_SHIFT: u32 = 22;
pub const K_REQUIRES_INSTANCE_MEMBERS_INITIALIZER_MASK: u32 =
  1 << K_REQUIRES_INSTANCE_MEMBERS_INITIALIZER_SHIFT;

pub const K_CONSTRUCT_AS_BUILTIN_SHIFT: u32 = 23;
pub const K_CONSTRUCT_AS_BUILTIN_MASK: u32 = 1 << K_CONSTRUCT_AS_BUILTIN_SHIFT;

pub const K_NAME_SHOULD_PRINT_AS_ANONYMOUS_SHIFT: u32 = 24;
pub const K_NAME_SHOULD_PRINT_AS_ANONYMOUS_MASK: u32 = 1 << K_NAME_SHOULD_PRINT_AS_ANONYMOUS_SHIFT;

pub const K_HAS_REPORTED_BINARY_COVERAGE_SHIFT: u32 = 25;
pub const K_HAS_REPORTED_BINARY_COVERAGE_MASK: u32 = 1 << K_HAS_REPORTED_BINARY_COVERAGE_SHIFT;

pub const K_IS_TOP_LEVEL_SHIFT: u32 = 26;
pub const K_IS_TOP_LEVEL_MASK: u32 = 1 << K_IS_TOP_LEVEL_SHIFT;

pub const K_PROPERTIES_ARE_FINAL_SHIFT: u32 = 27;
pub const K_PROPERTIES_ARE_FINAL_MASK: u32 = 1 << K_PROPERTIES_ARE_FINAL_SHIFT;

pub const K_PRIVATE_NAME_LOOKUP_SKIPS_OUTER_CLASS_SHIFT: u32 = 28;
pub const K_PRIVATE_NAME_LOOKUP_SKIPS_OUTER_CLASS_MASK: u32 =
  1 << K_PRIVATE_NAME_LOOKUP_SKIPS_OUTER_CLASS_SHIFT;

pub const K_IS_HOISTED_IN_CONTEXT_SHIFT: u32 = 29;
pub const K_IS_HOISTED_IN_CONTEXT_MASK: u32 = 1 << K_IS_HOISTED_IN_CONTEXT_SHIFT;

// ── Flags2 位域常量 ──────────────────────────────────────────────────────────
pub const K_CLASS_SCOPE_HAS_PRIVATE_BRAND_MASK: u8 = 1 << 0;
pub const K_HAS_STATIC_PRIVATE_METHODS_OR_ACCESSORS_MASK: u8 = 1 << 1;
pub const K_IS_SPARKPLUG_COMPILING_MASK: u8 = 1 << 2;
pub const K_MAGLEV_COMPILATION_FAILED_MASK: u8 = 1 << 3;
pub const K_CACHED_TIERING_DECISION_MASK: u8 = 0x07 << 4;
pub const K_FUNCTION_CONTEXT_INDEPENDENT_COMPILED_MASK: u8 = 1 << 7;

/// SharedFunctionInfo 头部大小 (64 字节，8 words)
pub const K_HEADER_SIZE: usize = mem::size_of::<SharedFunctionInfo>();

/// SharedFunctionInfo 结构体
#[repr(C)]
#[derive(Debug)]
pub struct SharedFunctionInfo {
  /// Map 指针 (槽 0, offset 0)
  pub map: Tagged<Map>,
  /// 函数关联数据 (槽 1, offset 8, 对应 untrusted_function_data_)
  pub function_data: Tagged<Object>,
  /// 函数名称或 ScopeInfo (槽 2, offset 16, 对应 name_or_scope_info_)
  pub name_or_scope_info: Tagged<Object>,
  /// 外层 ScopeInfo 或 FeedbackMetadata (槽 3, offset 24)
  pub outer_scope_info_or_feedback_metadata: Tagged<Object>,
  /// 来源脚本 (槽 4, offset 32, 对应 script_)
  pub script: Tagged<HeapObject>,
  /// 函数形参长度 (槽 5 内部字段, offset 40)
  pub length: u16,
  /// 形式参数总数 (offset 42)
  pub formal_parameter_count: u16,
  /// function 关键字源码相对偏移 (offset 44)
  pub function_token_offset: u16,
  /// 预估属性数 (offset 46)
  pub expected_nof_properties: u8,
  /// flags2 标志位 (offset 47)
  pub flags2: u8,
  /// flags 原子标志位 (槽 6 内部字段, offset 48)
  pub flags: AtomicU32,
  /// 函数字面量标识 (offset 52)
  pub function_literal_id: AtomicI32,
  /// 脚本内唯一标识 (槽 7 内部字段, offset 56)
  pub unique_id: i32,
  /// 热度计数 / 代龄 (offset 60)
  pub age: AtomicU16,
  /// 反馈槽位编号 (offset 62)
  pub feedback_slot: AtomicU16,
}

const _: () = assert!(mem::size_of::<SharedFunctionInfo>() == 64);
const _: () = assert!(mem::align_of::<SharedFunctionInfo>() == 8);

impl SharedFunctionInfo {
  /// 创建新的 SharedFunctionInfo 实例
  #[inline]
  pub fn new(
    map: Tagged<Map>,
    function_data: Tagged<Object>,
    name_or_scope_info: Tagged<Object>,
    outer_scope_info: Tagged<Object>,
    script: Tagged<HeapObject>,
  ) -> Self {
    Self {
      map,
      function_data,
      name_or_scope_info,
      outer_scope_info_or_feedback_metadata: outer_scope_info,
      script,
      length: 0,
      formal_parameter_count: 0,
      function_token_offset: 0,
      expected_nof_properties: 0,
      flags2: 0,
      flags: AtomicU32::new(0),
      function_literal_id: AtomicI32::new(-1),
      unique_id: 0,
      age: AtomicU16::new(0),
      feedback_slot: AtomicU16::new(0),
    }
  }
}

/// SharedFunctionInfo 操作扩展 trait
pub trait SharedFunctionInfoExt {
  /// 获取 map
  fn map(&self) -> Tagged<Map>;

  /// 获取原始 flags
  fn flags(&self) -> u32;

  /// 设置原始 flags
  fn set_flags(&self, value: u32);

  /// 获取 flags2
  fn flags2(&self) -> u8;

  /// 设置 flags2
  fn set_flags2(&mut self, value: u8);

  /// 获取函数种类
  fn kind(&self) -> FunctionKind {
    let k = (self.flags() & K_FUNCTION_KIND_MASK) >> K_FUNCTION_KIND_SHIFT;
    FunctionKind::from_u8(k as u8)
  }

  /// 设置函数种类
  fn set_kind(&self, kind: FunctionKind) {
    let old = self.flags();
    let val = ((kind as u32) << K_FUNCTION_KIND_SHIFT) & K_FUNCTION_KIND_MASK;
    let mut new_flags = (old & !K_FUNCTION_KIND_MASK) | val;
    if kind.is_class_constructor() {
      new_flags |= K_IS_CLASS_CONSTRUCTOR_MASK;
    } else {
      new_flags &= !K_IS_CLASS_CONSTRUCTOR_MASK;
    }
    self.set_flags(new_flags);
  }

  /// 是否为原生函数
  fn is_native(&self) -> bool {
    (self.flags() & K_IS_NATIVE_MASK) != 0
  }

  /// 设置是否为原生函数
  fn set_native(&self, value: bool) {
    let old = self.flags();
    let new_flags = if value {
      old | K_IS_NATIVE_MASK
    } else {
      old & !K_IS_NATIVE_MASK
    };
    self.set_flags(new_flags);
  }

  /// 是否为严格模式
  fn is_strict(&self) -> bool {
    (self.flags() & K_IS_STRICT_MASK) != 0
  }

  /// 设置严格模式
  fn set_strict(&self, value: bool) {
    let old = self.flags();
    let new_flags = if value {
      old | K_IS_STRICT_MASK
    } else {
      old & !K_IS_STRICT_MASK
    };
    self.set_flags(new_flags);
  }

  /// 获取函数语法种类
  #[inline]
  fn syntax_kind(&self) -> FunctionSyntaxKind {
    let raw = (self.flags() & K_FUNCTION_SYNTAX_KIND_MASK) >> K_FUNCTION_SYNTAX_KIND_SHIFT;
    FunctionSyntaxKind::from_u8(raw as u8)
  }

  /// 设置函数语法种类
  #[inline]
  fn set_syntax_kind(&self, kind: FunctionSyntaxKind) {
    let old = self.flags();
    let raw = ((kind as u32) << K_FUNCTION_SYNTAX_KIND_SHIFT) & K_FUNCTION_SYNTAX_KIND_MASK;
    let new_flags = (old & !K_FUNCTION_SYNTAX_KIND_MASK) | raw;
    self.set_flags(new_flags);
  }

  /// 是否为包裹函数/脚本 (wrapped script/function)
  #[inline]
  fn is_wrapped(&self) -> bool {
    matches!(self.syntax_kind(), FunctionSyntaxKind::Wrapped)
  }

  /// 是否为类构造函数
  fn is_class_constructor(&self) -> bool {
    (self.flags() & K_IS_CLASS_CONSTRUCTOR_MASK) != 0
  }

  /// 设置是否为类构造函数
  fn set_class_constructor(&self, value: bool) {
    let old = self.flags();
    let new_flags = if value {
      old | K_IS_CLASS_CONSTRUCTOR_MASK
    } else {
      old & !K_IS_CLASS_CONSTRUCTOR_MASK
    };
    self.set_flags(new_flags);
  }

  /// 是否允许惰性编译
  fn allows_lazy_compilation(&self) -> bool {
    (self.flags() & K_ALLOW_LAZY_COMPILATION_MASK) != 0
  }

  /// 设置允许惰性编译
  fn set_allows_lazy_compilation(&self, value: bool) {
    let old = self.flags();
    let new_flags = if value {
      old | K_ALLOW_LAZY_COMPILATION_MASK
    } else {
      old & !K_ALLOW_LAZY_COMPILATION_MASK
    };
    self.set_flags(new_flags);
  }

  /// 是否为顶层代码（script/eval）
  fn is_toplevel(&self) -> bool {
    (self.flags() & K_IS_TOP_LEVEL_MASK) != 0
  }

  /// 设置是否为顶层代码
  fn set_toplevel(&self, value: bool) {
    let old = self.flags();
    let new_flags = if value {
      old | K_IS_TOP_LEVEL_MASK
    } else {
      old & !K_IS_TOP_LEVEL_MASK
    };
    self.set_flags(new_flags);
  }

  /// 是否可构造
  fn is_constructable(&self) -> bool {
    self.kind().is_constructable()
  }

  /// 是否为异步函数
  fn is_async(&self) -> bool {
    self.kind().is_async()
  }

  /// 是否为生成器函数
  fn is_generator(&self) -> bool {
    self.kind().is_generator()
  }

  /// 获取 script 槽位
  fn script(&self) -> Tagged<HeapObject>;

  /// 设置 script 槽位
  fn set_script(&mut self, script: Tagged<HeapObject>);

  /// 是否为用户定义的 JavaScript 代码 (非 native 且附带有效脚本)
  fn is_user_javascript(&self) -> bool {
    !self.is_native() && !self.script().is_null()
  }

  /// 获取代龄
  fn age(&self) -> u16;

  /// 设置代龄
  fn set_age(&self, value: u16);

  /// CAS 交换代龄
  fn compare_exchange_age(&self, expected: u16, new_age: u16) -> u16;
}

impl SharedFunctionInfoExt for SharedFunctionInfo {
  #[inline]
  fn map(&self) -> Tagged<Map> {
    self.map
  }

  #[inline]
  fn flags(&self) -> u32 {
    self.flags.load(Ordering::Relaxed)
  }

  #[inline]
  fn set_flags(&self, value: u32) {
    self.flags.store(value, Ordering::Relaxed);
  }

  #[inline]
  fn flags2(&self) -> u8 {
    self.flags2
  }

  #[inline]
  fn set_flags2(&mut self, value: u8) {
    self.flags2 = value;
  }

  #[inline]
  fn script(&self) -> Tagged<HeapObject> {
    self.script
  }

  #[inline]
  fn set_script(&mut self, script: Tagged<HeapObject>) {
    self.script = script;
  }

  #[inline]
  fn age(&self) -> u16 {
    self.age.load(Ordering::Relaxed)
  }

  #[inline]
  fn set_age(&self, value: u16) {
    self.age.store(value, Ordering::Relaxed);
  }

  #[inline]
  fn compare_exchange_age(&self, expected: u16, new_age: u16) -> u16 {
    match self
      .age
      .compare_exchange(expected, new_age, Ordering::Relaxed, Ordering::Relaxed)
    {
      Ok(v) | Err(v) => v,
    }
  }
}

impl SharedFunctionInfoExt for Tagged<SharedFunctionInfo> {
  #[inline]
  fn map(&self) -> Tagged<Map> {
    // SAFETY: Tagged 包装合法 SharedFunctionInfo 指针
    unsafe { (*self.as_ptr()).map() }
  }

  #[inline]
  fn flags(&self) -> u32 {
    // SAFETY: Tagged 包装合法 SharedFunctionInfo 指针
    unsafe { (*self.as_ptr()).flags() }
  }

  #[inline]
  fn set_flags(&self, value: u32) {
    // SAFETY: Tagged 包装合法 SharedFunctionInfo 指针
    unsafe { (*self.as_ptr()).set_flags(value) }
  }

  #[inline]
  fn flags2(&self) -> u8 {
    // SAFETY: Tagged 包装合法 SharedFunctionInfo 指针
    unsafe { (*self.as_ptr()).flags2() }
  }

  #[inline]
  fn set_flags2(&mut self, value: u8) {
    // SAFETY: Tagged 包装合法可变 SharedFunctionInfo 指针
    unsafe { (*self.as_mut_ptr()).set_flags2(value) }
  }

  #[inline]
  fn script(&self) -> Tagged<HeapObject> {
    // SAFETY: Tagged 包装合法 SharedFunctionInfo 指针
    unsafe { (*self.as_ptr()).script() }
  }

  #[inline]
  fn set_script(&mut self, script: Tagged<HeapObject>) {
    // SAFETY: Tagged 包装合法可变 SharedFunctionInfo 指针
    unsafe { (*self.as_mut_ptr()).set_script(script) }
  }

  #[inline]
  fn age(&self) -> u16 {
    // SAFETY: Tagged 包装合法 SharedFunctionInfo 指针
    unsafe { (*self.as_ptr()).age() }
  }

  #[inline]
  fn set_age(&self, value: u16) {
    // SAFETY: Tagged 包装合法 SharedFunctionInfo 指针
    unsafe { (*self.as_ptr()).set_age(value) }
  }

  #[inline]
  fn compare_exchange_age(&self, expected: u16, new_age: u16) -> u16 {
    // SAFETY: Tagged 包装合法 SharedFunctionInfo 指针
    unsafe { (*self.as_ptr()).compare_exchange_age(expected, new_age) }
  }
}
