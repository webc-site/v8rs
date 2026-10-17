// 由 tool/src/diagnostics/basic-block-profiler.js 从 v8/src/diagnostics/basic-block-profiler.{h,cc} 生成; 修订就地改, 复跑覆盖
// 基本块profiler: 数据布局 + 签名骨架

use std::{
  collections::{BTreeSet, HashMap},
  fmt::{Display, Formatter, Result as FmtResult, Write as _},
  sync::{Mutex, OnceLock},
};

/// 原版 BlockCallees: 块内被调 builtin 集合 (Builtin 编码占位)
pub type BlockCallees = BTreeSet<u32>;
/// 原版 BuiltinCallees: 块 id → 被调集合
pub type BuiltinCallees = HashMap<i32, BlockCallees>;
/// 原版 BuiltinCallMap: builtin → 其调用图
pub type BuiltinCallMap = HashMap<u32, BuiltinCallees>;

/// 原版 class BasicBlockProfilerData:
/// 以逆后序块号索引的计数/分支/元数据 (字段机械对应)
#[derive(Debug, Clone, Default)]
pub struct BasicBlockProfilerData {
  /// 原版 block_ids_
  block_ids_: Vec<i32>,
  /// 原版 counts_
  counts_: Vec<u32>,
  /// 原版 branches_: (true_block_id, false_block_id)
  branches_: Vec<(i32, i32)>,
  /// 原版 function_name_
  function_name_: String,
  /// 原版 schedule_
  schedule_: String,
  /// 原版 code_
  code_: String,
  /// 原版 hash_ (默认 0)
  hash_: i32,
}

pub const BLOCK_MARKER: &str = "block";
pub const BLOCK_HINT_MARKER: &str = "block_hint";
pub const BUILTIN_HASH_MARKER: &str = "builtin_hash";

impl BasicBlockProfilerData {
  /// 原版构造 BasicBlockProfilerData(n_blocks)
  #[must_use]
  pub fn new(n_blocks: usize) -> Self {
    Self {
      block_ids_: vec![0; n_blocks],
      counts_: vec![0; n_blocks],
      branches_: Vec::new(),
      function_name_: String::new(),
      schedule_: String::new(),
      code_: String::new(),
      hash_: 0,
    }
  }

  /// 原版 n_blocks(): block_ids_.size() == counts_.size() 前提
  #[inline]
  #[must_use]
  pub fn n_blocks(&self) -> usize {
    debug_assert_eq!(self.block_ids_.len(), self.counts_.len());
    self.block_ids_.len()
  }

  /// 原版 counts()
  #[inline]
  #[must_use]
  pub fn counts(&self) -> &[u32] {
    &self.counts_
  }

  /// 原版 block_ids()
  #[inline]
  #[must_use]
  pub fn block_ids(&self) -> &[i32] {
    &self.block_ids_
  }

  /// 原版 branches()
  #[inline]
  #[must_use]
  pub fn branches(&self) -> &[(i32, i32)] {
    &self.branches_
  }

  /// 原版 function_name()
  #[inline]
  #[must_use]
  pub fn function_name(&self) -> &str {
    &self.function_name_
  }

  /// 原版 schedule()
  #[inline]
  #[must_use]
  pub fn schedule(&self) -> &str {
    &self.schedule_
  }

  /// 原版 code()
  #[inline]
  #[must_use]
  pub fn code(&self) -> &str {
    &self.code_
  }

  /// 原版 hash()
  #[inline]
  #[must_use]
  pub const fn hash(&self) -> i32 {
    self.hash_
  }

  /// 原版 SetBlockId(offset, id)
  pub fn set_block_id(&mut self, offset: usize, id: i32) {
    if let Some(target) = self.block_ids_.get_mut(offset) {
      *target = id;
    }
  }

  /// 原版 SetHash(hash)
  pub fn set_hash(&mut self, hash: i32) {
    self.hash_ = hash;
  }

  /// 原版 AddBranch(true_block_id, false_block_id)
  pub fn add_branch(&mut self, true_block_id: i32, false_block_id: i32) {
    self.branches_.push((true_block_id, false_block_id));
  }

  /// 原版 SetCode(os)
  pub fn set_code(&mut self, os: &str) {
    self.code_ = os.to_string();
  }

  /// 原版 SetFunctionName(name)
  pub fn set_function_name(&mut self, name: &str) {
    self.function_name_ = name.to_string();
  }

  /// 原版 SetSchedule(os)
  pub fn set_schedule(&mut self, os: &str) {
    self.schedule_ = os.to_string();
  }

  /// 原版 ResetCounts
  pub fn reset_counts(&mut self) {
    self.counts_.fill(0);
  }

  /// 原版 CopyToJSHeap(isolate):
  /// JS 堆对象分配属于深水区 (依赖 v8-isolate / v8-heap 内部堆布局);
  /// 纯逻辑面安全返回 0, 登记推迟台账。
  #[inline]
  pub fn copy_to_js_heap(&self, _isolate: *mut ()) -> u32 {
    0
  }

  /// 原版 Log(isolate, os)
  pub fn log(&self, _isolate: *mut (), os: &mut String) {
    let mut any_nonzero = false;
    for (&block_id, &count) in self.block_ids_.iter().zip(self.counts_.iter()) {
      if count > 0 {
        any_nonzero = true;
        let _ = writeln!(
          os,
          "{BLOCK_MARKER}\t{}\t{block_id}\t{count}",
          self.function_name_
        );
      }
    }
    if any_nonzero {
      for &(true_id, false_id) in &self.branches_ {
        let _ = writeln!(
          os,
          "{BLOCK_HINT_MARKER}\t{}\t{true_id}\t{false_id}",
          self.function_name_
        );
      }
      let _ = writeln!(
        os,
        "{BUILTIN_HASH_MARKER}\t{}\t{}",
        self.function_name_, self.hash_
      );
    }
  }

  /// 块计数自增 (辅助统计聚集)
  pub fn increment_count(&mut self, offset: usize) {
    if let Some(target) = self.counts_.get_mut(offset) {
      *target = target.saturating_add(1);
    }
  }

  /// 设置指定块计数
  pub fn set_count(&mut self, offset: usize, count: u32) {
    if let Some(target) = self.counts_.get_mut(offset) {
      *target = count;
    }
  }
}

impl Display for BasicBlockProfilerData {
  fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
    if self.counts_.iter().all(|&c| c == 0) {
      return Ok(());
    }
    let name = if self.function_name_.is_empty() {
      "unknown function"
    } else {
      &self.function_name_
    };
    if !self.schedule_.is_empty() {
      let b0 = self.counts_.first().copied().unwrap_or(0);
      writeln!(f, "schedule for {name} (B0 entered {b0} times)")?;
      writeln!(f, "{}", self.schedule_)?;
    }
    writeln!(f, "block counts for {name}:")?;
    let mut pairs: Vec<(usize, u32)> = self.counts_.iter().copied().enumerate().collect();
    pairs.sort_by(|(left_idx, left_cnt), (right_idx, right_cnt)| {
      right_cnt
        .cmp(left_cnt)
        .then_with(|| left_idx.cmp(right_idx))
    });
    for (idx, cnt) in pairs {
      if cnt == 0 {
        break;
      }
      writeln!(f, "block B{idx} : {cnt}")?;
    }
    writeln!(f)?;
    if !self.code_.is_empty() {
      writeln!(f, "{}", self.code_)?;
    }
    Ok(())
  }
}

/// 原版 class BasicBlockProfiler: 全局单例 (Get) + 数据列表 (互斥保护)
#[derive(Default)]
pub struct BasicBlockProfiler {
  /// 原版 data_list_
  data_list_: Vec<BasicBlockProfilerData>,
}

impl BasicBlockProfiler {
  /// 原版 Get(): 进程级单例
  pub fn get() -> &'static Mutex<Self> {
    static INSTANCE: OnceLock<Mutex<BasicBlockProfiler>> = OnceLock::new();
    INSTANCE.get_or_init(|| Mutex::new(Self::default()))
  }

  /// 原版 NewData(n_blocks): 追加并返回索引
  pub fn new_data(&mut self, n_blocks: usize) -> usize {
    self.data_list_.push(BasicBlockProfilerData::new(n_blocks));
    self.data_list_.len() - 1
  }

  /// 原版 ResetCounts(isolate)
  pub fn reset_counts(&mut self, _isolate: *mut ()) {
    for data in &mut self.data_list_ {
      data.reset_counts();
    }
  }

  /// 原版 HasData(isolate)
  #[must_use]
  pub fn has_data(&self, _isolate: *mut ()) -> bool {
    !self.data_list_.is_empty()
  }

  /// 原版 Print(isolate, os)
  pub fn print(&self, _isolate: *mut (), os: &mut String) {
    os.push_str("---- Start Profiling Data ----\n");
    for data in &self.data_list_ {
      let _ = write!(os, "{data}");
    }
    os.push_str("---- End Profiling Data ----\n");
  }

  /// 原版 Log(isolate, os)
  pub fn log(&self, _isolate: *mut (), os: &mut String) {
    for data in &self.data_list_ {
      data.log(_isolate, os);
    }
  }

  /// 原版 GetCoverageBitmap(isolate)
  #[must_use]
  pub fn get_coverage_bitmap(&self, _isolate: *mut ()) -> Vec<bool> {
    let total_blocks: usize = self
      .data_list_
      .iter()
      .map(BasicBlockProfilerData::n_blocks)
      .sum();
    let mut out = Vec::with_capacity(total_blocks);
    for data in &self.data_list_ {
      out.extend(data.counts().iter().map(|&c| c > 0));
    }
    out
  }

  /// 原版 GetBuiltinsBlockCount(isolate): 全 builtin 基本块总数快速零分配计数
  #[must_use]
  pub fn get_builtins_block_count(&self, _isolate: *mut ()) -> u32 {
    self
      .data_list_
      .iter()
      .fold(0u32, |acc, d| acc.saturating_add(d.n_blocks() as u32))
  }

  /// 原版 ForEachExecutedBlockAndReset(isolate, callback)
  pub fn for_each_executed_block_and_reset<F>(&mut self, _isolate: *mut (), mut callback: F)
  where
    F: FnMut(u32),
  {
    let mut block_index: u32 = 0;
    for data in &mut self.data_list_ {
      for c in &mut data.counts_ {
        if *c > 0 {
          callback(block_index);
          *c = 0;
        }
        block_index = block_index.wrapping_add(1);
      }
    }
  }

  /// 原版 data_list()
  #[inline]
  #[must_use]
  pub fn data_list(&self) -> &[BasicBlockProfilerData] {
    &self.data_list_
  }

  /// 原版 data_list_mut()
  #[inline]
  #[must_use]
  pub fn data_list_mut(&mut self) -> &mut [BasicBlockProfilerData] {
    &mut self.data_list_
  }
}

/// 原版 class BuiltinsCallGraph: builtin 调用图
#[derive(Debug, Clone)]
pub struct BuiltinsCallGraph {
  /// 原版 builtin_call_map_
  builtin_call_map_: BuiltinCallMap,
  /// 原版 all_hash_matched_
  all_hash_matched_: bool,
}

impl Default for BuiltinsCallGraph {
  fn default() -> Self {
    Self {
      builtin_call_map_: BuiltinCallMap::default(),
      all_hash_matched_: true,
    }
  }
}

impl BuiltinsCallGraph {
  /// 原版 Get(): 单例
  pub fn get() -> &'static Mutex<Self> {
    static INSTANCE: OnceLock<Mutex<BuiltinsCallGraph>> = OnceLock::new();
    INSTANCE.get_or_init(|| Mutex::new(Self::default()))
  }

  /// 原版 AddBuiltinCall(caller, callee, block_id)
  pub fn add_builtin_call(&mut self, caller: u32, callee: u32, block_id: i32) {
    self
      .builtin_call_map_
      .entry(caller)
      .or_default()
      .entry(block_id)
      .or_default()
      .insert(callee);
  }

  /// 原版 GetBuiltinCallees(builtin)
  #[must_use]
  pub fn get_builtin_callees(&self, builtin: u32) -> Option<&BuiltinCallees> {
    self.builtin_call_map_.get(&builtin)
  }

  /// 原版 all_hash_matched()
  #[inline]
  #[must_use]
  pub const fn all_hash_matched(&self) -> bool {
    self.all_hash_matched_
  }

  /// 原版 set_all_hash_matched(v)
  #[inline]
  pub const fn set_all_hash_matched(&mut self, v: bool) {
    self.all_hash_matched_ = v;
  }
}
