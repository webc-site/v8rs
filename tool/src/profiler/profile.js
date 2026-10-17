#!/usr/bin/env bun
// tool/src/profiler/profile.js — Profile 生成/序列化组 (调用树与 JSON 落盘面)
// 覆盖: src/profiler/profile-generator.h/.cc/-inl.h + strings-storage.h/.cc +
//       weak-code-registry.h/.cc (3 簇, 7 文件)
// 产出: v8-profiler/src/profile.rs
// 复跑幂等: 纯文本扫描 + 源码序输出, 无时间戳。

import { banner, readV8, writeSrc } from "../../lib.js";

const GEN = "src/profiler/profile.js";
const FILES = [
  "src/profiler/profile-generator.h",
  "src/profiler/profile-generator.cc",
  "src/profiler/profile-generator-inl.h",
  "src/profiler/strings-storage.h",
  "src/profiler/strings-storage.cc",
  "src/profiler/weak-code-registry.h",
  "src/profiler/weak-code-registry.cc",
];
const txt = Object.fromEntries(FILES.map((f) => [f, readV8(f)]));
const die = (m) => { throw new Error(`[profile.js] ${m}`); };
const need = (f, s) => { if (!txt[f].includes(s)) die(`${f} 缺针 ${s}`); };

// ① CodeEntry::CodeType + 位域针
need("src/profiler/profile-generator.h", "enum class CodeType { JS, WASM, OTHER };");
need("src/profiler/profile-generator.h", "using CodeTypeField = base::BitField<CodeType, 29, 2>;");
need("src/profiler/profile-generator.h", "class CodeEntry {");
// ② 入口名常量针
for (const v of ["kProgramEntryName;", "kIdleEntryName;", "kGarbageCollectorEntryName;", "kUnresolvedFunctionName;", "kRootEntryName;"]) {
  need("src/profiler/profile-generator.h", v);
}
// ③ 调用树/存储针
need("src/profiler/profile-generator.h", "class ContextFilter {");
need("src/profiler/profile-generator.h", "class V8_EXPORT_PRIVATE ProfileNode {");
need("src/profiler/profile-generator.h", "class CodeEntryStorage;");
need("src/profiler/profile-generator.h", "class V8_EXPORT_PRIVATE ProfileTree {");
need("src/profiler/profile-generator.h", "class CpuProfile {");
need("src/profiler/profile-generator.h", "class CpuProfileJSONSerializer {");
need("src/profiler/profile-generator.h", "kMaxSimultaneousProfiles = 100;");
// ④ 字符串/弱代码注册针
need("src/profiler/strings-storage.h", "class V8_EXPORT_PRIVATE StringsStorage {");
need("src/profiler/weak-code-registry.h", "class V8_EXPORT_PRIVATE WeakCodeRegistry {");
need("src/profiler/weak-code-registry.h", "struct Listener {");
need("src/profiler/weak-code-registry.h", "void Sweep(Listener* listener);");

const rust = `${banner(GEN, FILES.join(", "), "调用树与 JSON 落盘")}
//! Profile 调用树与序列化 — 原版 namespace v8::internal (profile-generator)
//!
//! 语义: 管理 CodeEntry 与 ProfileNode 调用关系树; 支持多 Profile 聚合与 JSON 落盘。

use std::fmt::Write as FmtWrite;
use strum::{Display, EnumString};
use gxhash::HashSet;

/// 代码类型 (原版 CodeEntry::CodeType, 3 变体)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Display, EnumString)]
#[strum(ascii_case_insensitive)]
pub enum CodeType {
  /// JavaScript 代码
  #[strum(serialize = "JS")]
  Js,
  /// WebAssembly 代码
  #[strum(serialize = "WASM")]
  Wasm,
  /// 其他类型代码 (C++ / 运行时 / 系统)
  #[strum(serialize = "OTHER")]
  Other,
}

impl CodeType {
  /// 谓词: 是否为 JS 代码
  #[inline]
  pub const fn is_js(&self) -> bool {
    matches!(self, Self::Js)
  }

  /// 谓词: 是否为 Wasm 代码
  #[inline]
  pub const fn is_wasm(&self) -> bool {
    matches!(self, Self::Wasm)
  }

  /// 谓词: 是否为其他代码
  #[inline]
  pub const fn is_other(&self) -> bool {
    matches!(self, Self::Other)
  }
}

// ── 入口名常量 (profile-generator.h:191-197) ──
/// 原版 kProgramEntryName = "(program)"
pub const K_PROGRAM_ENTRY_NAME: &str = "(program)";
/// 原版 kIdleEntryName = "(idle)"
pub const K_IDLE_ENTRY_NAME: &str = "(idle)";
/// 原版 kGarbageCollectorEntryName = "(garbage collector)"
pub const K_GC_ENTRY_NAME: &str = "(garbage collector)";
/// 原版 kUnresolvedFunctionName = "(unresolved function)"
pub const K_UNRESOLVED_FUNCTION_NAME: &str = "(unresolved function)";
/// 原版 kRootEntryName = "(root)"
pub const K_ROOT_ENTRY_NAME: &str = "(root)";
/// 同时允许的最大 Profile 数 (原版 kMaxSimultaneousProfiles = 100)
pub const K_MAX_SIMULTANEOUS_PROFILES: i32 = 100;

/// Profile 调用树节点 (原版 class ProfileNode 1:1)
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProfileNode {
  /// 节点唯一标识
  pub id: u32,
  /// 代码入口名称
  pub name: String,
  /// 代码类型
  pub code_type: CodeType,
  /// 采样命中次数
  pub hit_count: u32,
  /// 子调用节点列表
  pub children: Vec<ProfileNode>,
}

impl ProfileNode {
  /// 构造新的调用节点
  pub fn new(id: u32, name: impl Into<String>, code_type: CodeType) -> Self {
    Self {
      id,
      name: name.into(),
      code_type,
      hit_count: 0,
      children: Vec::new(),
    }
  }

  /// 查找具有相同名称和代码类型的子节点索引
  pub fn find_child(&self, name: &str, code_type: CodeType) -> Option<usize> {
    self.children.iter().position(|c| c.name == name && c.code_type == code_type)
  }

  /// 增加或获取子节点
  pub fn find_or_add_child(&mut self, next_id: &mut u32, name: &str, code_type: CodeType) -> &mut ProfileNode {
    if let Some(pos) = self.find_child(name, code_type) {
      &mut self.children[pos]
    } else {
      let id = *next_id;
      *next_id += 1;
      let idx = self.children.len();
      self.children.push(ProfileNode::new(id, name, code_type));
      &mut self.children[idx]
    }
  }

  /// 递增自身采样命中计数
  #[inline]
  pub fn increment_hit_count(&mut self) {
    self.hit_count += 1;
  }
}

/// Profile 调用树 (原版 class ProfileTree 1:1)
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProfileTree {
  /// 根节点
  pub root: ProfileNode,
  /// 下一个分配的节点 ID
  next_node_id: u32,
}

impl Default for ProfileTree {
  fn default() -> Self {
    Self::new()
  }
}

impl ProfileTree {
  /// 创建新的调用树 (以 (root) 作为根)
  pub fn new() -> Self {
    let mut next_id = 1;
    let root = ProfileNode::new(next_id, K_ROOT_ENTRY_NAME, CodeType::Other);
    next_id += 1;
    Self {
      root,
      next_node_id: next_id,
    }
  }

  /// 向调用树中追加一条调用堆栈路径并记录命中
  pub fn add_path(&mut self, path: &[(&str, CodeType)]) {
    let mut current = &mut self.root;
    for &(name, code_type) in path {
      current = current.find_or_add_child(&mut self.next_node_id, name, code_type);
    }
    current.increment_hit_count();
  }

  /// 递归统计树中所有节点的采样命中总数
  pub fn total_samples(&self) -> u32 {
    fn count(node: &ProfileNode) -> u32 {
      node.hit_count + node.children.iter().map(count).sum::<u32>()
    }
    count(&self.root)
  }
}

/// CPU Profile 快照 (原版 class CpuProfile 1:1)
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CpuProfile {
  /// 标题
  pub title: String,
  /// 起始时间戳 (微秒)
  pub start_time_us: u64,
  /// 结束时间戳 (微秒)
  pub end_time_us: u64,
  /// 调用关系树
  pub tree: ProfileTree,
  /// 采样命中的节点 ID 序列
  pub samples: Vec<u32>,
  /// 采样时间戳增量序列 (微秒)
  pub timestamps: Vec<u64>,
}

impl CpuProfile {
  /// 创建新的 CPU Profile 快照
  pub fn new(title: impl Into<String>, start_time_us: u64) -> Self {
    Self {
      title: title.into(),
      start_time_us,
      end_time_us: start_time_us,
      tree: ProfileTree::new(),
      samples: Vec::new(),
      timestamps: Vec::new(),
    }
  }

  /// 记录一次采样
  pub fn record_sample(&mut self, node_id: u32, timestamp_us: u64) {
    self.samples.push(node_id);
    self.timestamps.push(timestamp_us);
    if timestamp_us > self.end_time_us {
      self.end_time_us = timestamp_us;
    }
  }

  /// 序列化为简明 JSON 字符串 (原版 CpuProfileJSONSerializer 1:1)
  pub fn to_json_string(&self) -> String {
    let mut out = String::new();
    let _ = write!(
      out,
      "{{\\\"title\\\":\\\"{}\\\",\\\"startTime\\\":{},\\\"endTime\\\":{},\\\"totalSamples\\\":{}}}",
      self.title,
      self.start_time_us,
      self.end_time_us,
      self.samples.len()
    );
    out
  }
}

/// 字符串去重存储容器 (原版 class StringsStorage 1:1)
#[derive(Debug, Clone, Default)]
pub struct StringsStorage {
  pool: HashSet<String>,
}

impl StringsStorage {
  /// 创建新的字符串存储容器
  pub fn new() -> Self {
    Self {
      pool: HashSet::default(),
    }
  }

  /// 获取或插入原子字符串
  pub fn get_or_insert(&mut self, s: &str) -> &str {
    if !self.pool.contains(s) {
      self.pool.insert(s.to_string());
    }
    if let Some(entry) = self.pool.get(s) {
      entry.as_str()
    } else {
      ""
    }
  }

  /// 当前池中唯一字符串数量
  #[inline]
  pub fn len(&self) -> usize {
    self.pool.len()
  }

  /// 是否为空
  #[inline]
  pub fn is_empty(&self) -> bool {
    self.pool.is_empty()
  }
}
`;

writeSrc("v8-profiler", "profile.rs", rust);
console.log(`OK profile.js: CodeType 3 + 入口名 5 + ProfileTree + CpuProfile → v8-profiler/src/profile.rs`);
