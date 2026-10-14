// 由 tool/src/flags/flags.h.js 从 v8/src/flags/flags.h 生成; 修订就地改, 复跑覆盖
// 存储层: 对应原版 FlagValue<T> + FlagValues(v8_flags 单例) + SetBy 运行时状态

use std::borrow::Cow as StdCow;

use strum::Display;

use crate::{
  defs,
  error::{Error, Result},
  lookup::FlagId,
};

/// flag 字符串值 (原版 const char*; 默认值 Borrowed 静态串, 命令行设置 Owned)
pub type Cow = StdCow<'static, str>;

/// 单个 flag 的值 (原版 FlagValue<T> 模板脱虚后的全集, 见 flags.h static_assert:
/// 算术类型 + std::optional<bool> + const char*)
#[derive(Clone, Debug, PartialEq)]
pub enum FlagValue {
  Bool(bool),
  MaybeBool(Option<bool>),
  Int(i32),
  Uint(u32),
  Uint64(u64),
  Float(f64),
  SizeT(usize),
  Str(Option<Cow>),
}

/// flag 类型 (原版 flags-impl.h Flag::FlagType; Display 输出与 flags.cc Type2String 一致)
#[derive(Copy, Clone, Debug, Display, PartialEq, Eq)]
pub enum FlagType {
  #[strum(serialize = "bool")]
  Bool,
  #[strum(serialize = "maybe_bool")]
  MaybeBool,
  #[strum(serialize = "int")]
  Int,
  #[strum(serialize = "uint")]
  Uint,
  #[strum(serialize = "uint64")]
  Uint64,
  #[strum(serialize = "float")]
  Float,
  #[strum(serialize = "size_t")]
  SizeT,
  #[strum(serialize = "string")]
  Str,
}

/// 类型标签 (编译期统计用; PartialEq 判等非 const)
#[derive(Copy, Clone, Debug)]
pub(crate) enum TyTag {
  Bool,
  MaybeBool,
  Int,
  Uint,
  Uint64,
  Float,
  SizeT,
  Str,
}

impl FlagType {
  /// 类型名 (flags.cc Type2String: bool)
  /// 类型名 (flags.cc Type2String)
  #[inline]
  pub const fn as_str(self) -> &'static str {
    match self {
      Self::Bool => "bool",
      Self::MaybeBool => "maybe_bool",
      Self::Int => "int",
      Self::Uint => "uint",
      Self::Uint64 => "uint64",
      Self::Float => "float",
      Self::SizeT => "size_t",
      Self::Str => "string",
    }
  }
  /// 类型标签
  #[inline]
  pub(crate) const fn tag(self) -> TyTag {
    match self {
      Self::Bool => TyTag::Bool,
      Self::MaybeBool => TyTag::MaybeBool,
      Self::Int => TyTag::Int,
      Self::Uint => TyTag::Uint,
      Self::Uint64 => TyTag::Uint64,
      Self::Float => TyTag::Float,
      Self::SizeT => TyTag::SizeT,
      Self::Str => TyTag::Str,
    }
  }
}

impl TyTag {
  /// 标签判等 (const 上下文可用)
  #[inline]
  pub(crate) const fn is(self, other: TyTag) -> bool {
    matches!(
      (self, other),
      (TyTag::Bool, TyTag::Bool)
        | (TyTag::MaybeBool, TyTag::MaybeBool)
        | (TyTag::Int, TyTag::Int)
        | (TyTag::Uint, TyTag::Uint)
        | (TyTag::Uint64, TyTag::Uint64)
        | (TyTag::Float, TyTag::Float)
        | (TyTag::SizeT, TyTag::SizeT)
        | (TyTag::Str, TyTag::Str)
    )
  }
}

impl FlagValue {
  /// 值所属类型
  #[inline]
  pub fn ty(&self) -> FlagType {
    match self {
      Self::Bool(_) => FlagType::Bool,
      Self::MaybeBool(_) => FlagType::MaybeBool,
      Self::Int(_) => FlagType::Int,
      Self::Uint(_) => FlagType::Uint,
      Self::Uint64(_) => FlagType::Uint64,
      Self::Float(_) => FlagType::Float,
      Self::SizeT(_) => FlagType::SizeT,
      Self::Str(_) => FlagType::Str,
    }
  }
  /// 按目标类型取 bool 值 (类型不符返回 None)
  #[inline]
  pub fn as_bool(&self) -> Option<bool> {
    match self {
      Self::Bool(v) => Some(*v),
      _ => None,
    }
  }
}

/// 值的设置来源 (原版 Flag::SetBy)
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum SetBy {
  Default,
  WeakImplication,
  Implication,
  CommandLine,
}

/// 是否任一种蕴含来源
#[inline]
pub const fn is_any_implication(s: SetBy) -> bool {
  matches!(s, SetBy::WeakImplication | SetBy::Implication)
}

/// flag 处理模式 (原版 flags.h FlagProcessingMode, 控制矛盾/错误的处置)
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum FlagProcessingMode {
  /// Flag errors lead to abnormal termination (via Abort). This is typically the default behavior as it clearly indicates flag misconfigurations.
  AbortOnError,
  /// Flag errors lead to termination via Exit(-1). This is useful for automated bug detection systems that may set custom flags. This way, flag misconfigurations are not treated as crashes but only as failed executions.
  ExitOnError,
  /// Flag contradictions are explicitly ignored. This can be useful when fuzzing with random flags that are likely to contradict each other. In these cases, the testcase will still be executed and the last specification of each flag will be used.
  IgnoreContradictions,
}

/// 全部 flag 的运行时状态 (原版 FlagValues v8_flags 单例 + Flag 运行时字段合一)
///
/// 原版 Flag::valptr_ 指回 v8_flags 字段; 本实现值就存在本结构内,
/// 静态元数据 (名/类型/默认值/只读) 在 defs::FLAGS, 按下标一一对应。
/// readonly flag (原版 valptr_==nullptr) 的值恒为默认值, 变更请求被拒。
#[derive(Debug)]
pub struct Flags {
  /// 当前值 (NUM_FLAGS 主 flag, 顺序=defs::FLAGS); crate 内直访 (save/hash 热路径)
  pub(crate) values: Box<[FlagValue]>,
  /// 设置来源 (原版 Flag::set_by_)
  set_by: Box<[SetBy]>,
  /// 蕴含来源名 (原版 Flag::implied_by_; 显式设置为 None; 等值蕴含也登记, 是元数据)
  implied_by: Box<[Option<&'static str>]>,
  /// 蕴含改值边 (原版 ImplicationProcessor::implied_by_map_): premise 名 (剥 '!') →
  /// 因该 premise 真实改值的 flag 列表。仅 is_change 时登记 — 与 implied_by 分离,
  /// 等值蕴含不进重置图 (否则 premise 日后改值会误重置从未被改过的 flag)
  pub(crate) implied_by_map: Vec<(&'static str, Vec<FlagId>)>,
  /// 冻结标志 (原版 FlagList::FreezeFlags 的 flags_frozen)
  frozen: bool,
}

impl Default for Flags {
  #[inline]
  fn default() -> Self {
    Self::new()
  }
}

impl Flags {
  /// 全默认值构造 (原版 FlagValues 默认构造)
  pub fn new() -> Self {
    Self {
      values: defs::FLAGS.iter().map(|d| (d.default)()).collect(),
      set_by: vec![SetBy::Default; defs::NUM_FLAGS].into_boxed_slice(),
      implied_by: vec![None; defs::NUM_FLAGS].into_boxed_slice(),
      implied_by_map: Vec::new(),
      frozen: false,
    }
  }

  /// 是否冻结 (原版 FlagList::IsFrozen)
  #[inline]
  pub fn is_frozen(&self) -> bool {
    self.frozen
  }

  /// 冻结 flag 值 (原版 FreezeFlags; 原版另做内存页保护, 本层仅拒写)
  pub fn freeze(&mut self) {
    self.frozen = true;
  }

  /// flag 名 (原始下划线形式)
  #[inline]
  pub fn name(&self, id: FlagId) -> &'static str {
    defs::FLAGS[id as usize].name
  }

  /// flag 静态描述
  #[inline]
  pub(crate) fn def(&self, id: FlagId) -> &'static defs::FlagDef {
    &defs::FLAGS[id as usize]
  }

  // ── 类型化读取 (defs.rs 生成访问器的底层原语; id 由生成器保证类型匹配) ──
  #[inline]
  pub(crate) fn get_bool(&self, id: FlagId) -> bool {
    match &self.values[id as usize] {
      FlagValue::Bool(v) => *v,
      _ => false,
    }
  }
  #[inline]
  pub(crate) fn get_mb(&self, id: FlagId) -> Option<bool> {
    match &self.values[id as usize] {
      FlagValue::MaybeBool(v) => *v,
      _ => None,
    }
  }
  #[inline]
  pub(crate) fn get_i32(&self, id: FlagId) -> i32 {
    match self.values[id as usize] {
      FlagValue::Int(v) => v,
      _ => 0,
    }
  }
  #[inline]
  pub(crate) fn get_u32(&self, id: FlagId) -> u32 {
    match self.values[id as usize] {
      FlagValue::Uint(v) => v,
      _ => 0,
    }
  }
  #[inline]
  pub(crate) fn get_u64(&self, id: FlagId) -> u64 {
    match self.values[id as usize] {
      FlagValue::Uint64(v) => v,
      _ => 0,
    }
  }
  #[inline]
  pub(crate) fn get_f64(&self, id: FlagId) -> f64 {
    match self.values[id as usize] {
      FlagValue::Float(v) => v,
      _ => 0.0,
    }
  }
  #[inline]
  pub(crate) fn get_usize(&self, id: FlagId) -> usize {
    match self.values[id as usize] {
      FlagValue::SizeT(v) => v,
      _ => 0,
    }
  }
  #[inline]
  pub(crate) fn get_str(&self, id: FlagId) -> Option<&str> {
    match &self.values[id as usize] {
      FlagValue::Str(v) => v.as_deref(),
      _ => None,
    }
  }

  /// 值的通用只读视图
  #[inline]
  pub fn value(&self, id: FlagId) -> &FlagValue {
    &self.values[id as usize]
  }

  /// 默认值
  #[inline]
  pub fn default_value(&self, id: FlagId) -> FlagValue {
    (defs::FLAGS[id as usize].default)()
  }

  // ── 类型化默认值 (implication 条件里 f.default_xxx(id) 用; 按生成表实际使用收口) ──
  /// bool 默认值
  #[inline]
  pub fn default_bool(&self, id: FlagId) -> bool {
    match (defs::FLAGS[id as usize].default)() {
      FlagValue::Bool(v) => v,
      _ => false,
    }
  }
  /// i32 默认值
  #[inline]
  pub fn default_i32(&self, id: FlagId) -> i32 {
    match (defs::FLAGS[id as usize].default)() {
      FlagValue::Int(v) => v,
      _ => 0,
    }
  }
  /// 字符串默认值 (借用切片; 默认值恒为静态 Borrowed 串)
  #[inline]
  pub fn default_str(&self, id: FlagId) -> Option<&'static str> {
    match (defs::FLAGS[id as usize].default)() {
      FlagValue::Str(Some(StdCow::Borrowed(s))) => Some(s),
      _ => None,
    }
  }

  /// 当前值是否等于默认值 (原版 Flag::IsDefault)
  pub fn is_default(&self, id: FlagId) -> bool {
    self.values[id as usize] == (defs::FLAGS[id as usize].default)()
  }

  /// 设置来源
  #[inline]
  pub fn set_by(&self, id: FlagId) -> SetBy {
    self.set_by[id as usize]
  }

  /// 蕴含来源名 (非蕴含设置为 None)
  #[inline]
  pub fn implied_by(&self, id: FlagId) -> Option<&'static str> {
    self.implied_by[id as usize]
  }

  /// 冲突检查开关 (原版 Flag::ShouldCheckFlagContradictions; 由处理模式推导)
  pub fn should_check_contradictions(&self) -> Result<bool> {
    Ok(self.processing_mode()? != FlagProcessingMode::IgnoreContradictions)
  }

  /// 处理模式 (原版 FlagList::GetFlagProcessingMode: 读 flag_processing_mode 串 + fuzzing 豁免)
  pub fn processing_mode(&self) -> Result<FlagProcessingMode> {
    let Some(mode) = self.flag_processing_mode() else {
      return Ok(FlagProcessingMode::IgnoreContradictions); // 默认忽略矛盾 (历史行为)
    };
    match mode {
      "exit-on-error" => Ok(FlagProcessingMode::ExitOnError),
      "abort-on-error" => {
        // 历史行为: --fuzzing 下关闭矛盾检查
        if self.fuzzing() {
          Ok(FlagProcessingMode::IgnoreContradictions)
        } else {
          Ok(FlagProcessingMode::AbortOnError)
        }
      }
      "ignore-contradictions" => Ok(FlagProcessingMode::IgnoreContradictions),
      other => Err(Error::InvalidProcessingMode(other.to_string())),
    }
  }

  /// 变更门禁 (原版 Flag::CheckFlagChange 1:1)
  ///
  /// 返回是否执行值写入; 弱蕴含被强蕴含/命令行压制的场合返回 false。
  /// 矛盾检查开启且违反规则时返回 Err (原版 FlagError 直接终止进程)。
  pub(crate) fn check_flag_change(
    &mut self,
    id: FlagId,
    new_set_by: SetBy,
    change: bool,
    implied_by: Option<&'static str>,
  ) -> Result<bool> {
    let idx = id as usize;
    let def = &defs::FLAGS[idx];
    let cur = self.set_by[idx];
    // 弱蕴含输给已生效的强蕴含/命令行
    if new_set_by == SetBy::WeakImplication
      && matches!(cur, SetBy::Implication | SetBy::CommandLine)
    {
      return Ok(false);
    }
    if self.should_check_contradictions()? {
      // 只读 flag 不得变值
      if change && def.readonly {
        return match implied_by {
          None => Err(Error::ReadonlyConflict { name: def.name }),
          Some(by) => Err(Error::ReadonlyImplied { name: def.name, by }),
        };
      }
      // bool 类仅在值实际变化时报冲突; 非 bool 类重复显式/蕴含不同值即报
      let is_bool = matches!(def.ty, FlagType::Bool | FlagType::MaybeBool);
      if change {
        match (cur, new_set_by) {
          (SetBy::WeakImplication, SetBy::WeakImplication) => {
            return Err(Error::WeakImplications {
              name: def.name,
              a: self.implied_by[idx],
              b: implied_by,
            });
          }
          (SetBy::Implication, SetBy::Implication) => {
            return Err(Error::Implications {
              name: def.name,
              a: self.implied_by[idx],
              b: implied_by,
            });
          }
          (SetBy::CommandLine, SetBy::Implication) => {
            return Err(if is_bool {
              Error::ImpliedVsExplicit {
                name: def.name,
                by: implied_by.unwrap_or(""),
              }
            } else {
              Error::ImpliedButExplicit {
                name: def.name,
                by: implied_by.unwrap_or(""),
              }
            });
          }
          (SetBy::CommandLine, SetBy::CommandLine) => {
            return Err(if is_bool {
              Error::CmdBothTrueFalse { name: def.name }
            } else {
              Error::CmdMultiple { name: def.name }
            });
          }
          _ => {}
        }
      }
    }
    if change && def.readonly {
      // 不检查矛盾的场合: 只读 flag 静默忽略变更
      return Ok(false);
    }
    self.set_by[idx] = new_set_by;
    if is_any_implication(new_set_by) {
      self.implied_by[idx] = implied_by;
    } else {
      self.implied_by[idx] = None;
    }
    Ok(change)
  }

  /// 值写入 (已过 check_flag_change 门禁; 只读恒不写入)
  #[inline]
  pub(crate) fn write_value(&mut self, id: FlagId, v: FlagValue) {
    if !defs::FLAGS[id as usize].readonly {
      self.values[id as usize] = v;
    }
  }

  /// 通用赋值 (显式设置语义 = 命令行; defs.rs 生成的 set_xxx 与解析器共用)
  pub(crate) fn assign(&mut self, id: FlagId, v: FlagValue) -> Result<()> {
    self.ensure_writable()?;
    let idx = id as usize;
    let change = self.values[idx] != v;
    if self.check_flag_change(id, SetBy::CommandLine, change, None)? {
      self.write_value(id, v);
    }
    Ok(())
  }

  /// 冻结后拒绝一切赋值 (原版 FlagValue::operator= → ResetFlagHash 的 CHECK)
  #[inline]
  fn ensure_writable(&self) -> Result<()> {
    if self.frozen {
      Err(Error::Frozen)
    } else {
      Ok(())
    }
  }

  // ── 类型化赋值 (defs.rs 生成访问器底层) ──
  /// Bool 赋值 (显式语义)
  #[inline]
  pub(crate) fn assign_bool(&mut self, id: FlagId, v: bool) -> Result<()> {
    self.assign(id, FlagValue::Bool(v))
  }
  /// MaybeBool 赋值 (显式语义)
  #[inline]
  pub(crate) fn assign_mb(&mut self, id: FlagId, v: Option<bool>) -> Result<()> {
    self.assign(id, FlagValue::MaybeBool(v))
  }
  /// Int 赋值 (显式语义)
  #[inline]
  pub(crate) fn assign_i32(&mut self, id: FlagId, v: i32) -> Result<()> {
    self.assign(id, FlagValue::Int(v))
  }
  /// Uint 赋值 (显式语义)
  #[inline]
  pub(crate) fn assign_u32(&mut self, id: FlagId, v: u32) -> Result<()> {
    self.assign(id, FlagValue::Uint(v))
  }
  /// Uint64 赋值 (显式语义)
  #[inline]
  pub(crate) fn assign_u64(&mut self, id: FlagId, v: u64) -> Result<()> {
    self.assign(id, FlagValue::Uint64(v))
  }
  /// Float 赋值 (显式语义)
  #[inline]
  pub(crate) fn assign_f64(&mut self, id: FlagId, v: f64) -> Result<()> {
    self.assign(id, FlagValue::Float(v))
  }
  /// SizeT 赋值 (显式语义)
  #[inline]
  pub(crate) fn assign_usize(&mut self, id: FlagId, v: usize) -> Result<()> {
    self.assign(id, FlagValue::SizeT(v))
  }
  /// 字符串赋值 (None = 清空)
  #[inline]
  pub(crate) fn assign_str(&mut self, id: FlagId, v: Option<Cow>) -> Result<()> {
    self.assign(id, FlagValue::Str(v))
  }

  /// 重置为默认值 (原版 Flag::Reset; SetBy 归 Default)
  pub fn reset(&mut self, id: FlagId) -> Result<()> {
    let d = (defs::FLAGS[id as usize].default)();
    let change = self.values[id as usize] != d;
    if self.check_flag_change(id, SetBy::Default, change, None)? {
      self.write_value(id, d);
    }
    Ok(())
  }

  /// 允许重新覆盖 (原版 Flag::AllowOverwriting)
  #[inline]
  pub fn allow_overwriting(&mut self, id: FlagId) {
    self.set_by[id as usize] = SetBy::Default;
  }
}
