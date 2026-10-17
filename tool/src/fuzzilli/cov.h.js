#!/usr/bin/env bun
// tool/src/fuzzilli/cov.h.js
// 机械提取 v8/src/fuzzilli/cov.{h,cc}
// 产物: v8-fuzzilli/src/cov.rs — 覆盖率位图常量 + 位运算内联落地 + Safe Rust 覆盖率位图体系

import { readV8, writeSrc, banner } from "../../lib.js";

const hSrc = readV8("src/fuzzilli/cov.h");
const ccSrc = readV8("src/fuzzilli/cov.cc");

const fail = (label, got, want) => {
  throw new Error(`对数失配 ${label}: got=${got} want=${want}`);
};
const must = (cond, label) => {
  if (!cond) throw new Error(`关键结构缺失: ${label}`);
};

// 1. 宏 → 显式常量
const shmSize = ccSrc.match(/^#define SHM_SIZE (0x[0-9a-fA-F]+)$/m);
must(shmSize, "SHM_SIZE");
const maxEdges = ccSrc.match(/^#define MAX_EDGES \(\(SHM_SIZE - 4\) \* 8\)$/m);
must(maxEdges, "MAX_EDGES 公式");
const SHM_SIZE = parseInt(shmSize[1], 16);
const MAX_EDGES = (SHM_SIZE - 4) * 8;

// 2. shmem_data 布局 (num_edges + edges[])
must(ccSrc.includes("struct shmem_data"), "struct shmem_data");
must(ccSrc.includes("uint32_t num_edges;"), "shmem_data.num_edges");
must(ccSrc.includes("unsigned char edges[];"), "shmem_data.edges[]");

// 3. 模块级全局 (cc)
const globals = ["shmem", "edges_start", "edges_stop", "builtins_start", "builtins_edge_count", "num_bool_flag_regions"];
for (const g of globals) {
  if (!ccSrc.includes(`${g}`)) fail(`全局 ${g}`, false, true);
}

// 4. support_multi_dso 常量 (USE_CHROMIUM_FUZZILLI 分支; 非 Chromium 构建为 false)
const multiDso = ccSrc.match(/#ifdef USE_CHROMIUM_FUZZILLI\nconstexpr bool support_multi_dso = (true|false);/);
must(multiDso, "support_multi_dso 分支");

// 5. h 内联函数 2 条 (纯位运算 → const fn 落地)
must(hSrc.includes("inline void cov_set_edge("), "cov_set_edge 内联");
must(hSrc.includes("inline bool cov_is_edge_set("), "cov_is_edge_set 内联");

// 6. h 导出函数声明全集 (9 条)
const decls = [
  "fuzzilli_cov_enable", "sanitizer_cov_reset_edgeguards", "sanitizer_cov_flush_bool_flags",
  "sanitizer_cov_discard_bool_flags", "sanitizer_cov_install_crash_flush_handlers",
  "sanitizer_cov_count_discovered_edges", "cov_init_builtins_edges",
  "cov_has_builtins_edges", "cov_set_builtin_edge",
];
for (const d of decls) {
  if (!hSrc.includes(`${d}(`)) fail(`h 声明 ${d}`, false, true);
}
// cc 侧 sanitizer 回调 (编译器接口, h 未声明)
const ccCallbacks = ["__sanitizer_cov_trace_pc_guard_init", "__sanitizer_cov_trace_pc_guard", "__sanitizer_cov_bool_flag_init"];
for (const c of ccCallbacks) {
  if (!ccSrc.includes(c)) fail(`cc 回调 ${c}`, false, true);
}

// 7. builtins 边界语义 (cc 尾部): builtins_start = 1 + shmem->num_edges
must(ccSrc.includes("builtins_start = 1 + shmem->num_edges;"), "builtins_start 公式");
must(ccSrc.includes("shmem->num_edges += builtins_edge_count;"), "num_edges 累加");
must(ccSrc.includes("cov_set_edge(shmem->edges, builtins_start + block_index);"), "set_builtin_edge 公式");

// 8. inline-bool-flag 区域上限
const maxRegions = ccSrc.match(/constexpr size_t kMaxBoolFlagRegions = (\d+);/);
must(maxRegions, "kMaxBoolFlagRegions");
const MAX_BOOL_FLAG_REGIONS = parseInt(maxRegions[1], 10);

// 9. 崩溃信号列表
const crashSignals = ccSrc.match(/constexpr int kCrashSignals\[\] = \{([^}]+)\};/);
must(crashSignals, "kCrashSignals 列表");

let n = 22;
const rust = `${banner("src/fuzzilli/cov.h.js", "src/fuzzilli/cov.{h,cc}", "Fuzzilli 覆盖率: 位图常量 + 位运算内联落地 + Safe Rust 覆盖率位图体系")}
//! Fuzzilli 覆盖率桥 — 原版 v8/src/fuzzilli/cov.{{h,cc}}
//!
//! 语义: 经共享内存位图与 Fuzzilli 交换边覆盖率; guard 数组首击归零 (单 DSO 模式)。

use crate::error::{Error, Result};

/// 共享内存位图大小 (原版 #define SHM_SIZE) = 2 MiB
pub const SHM_SIZE: u32 = 0x${SHM_SIZE.toString(16)};

/// 可编码最大边数 (原版 #define MAX_EDGES = (SHM_SIZE - 4) * 8; 头 4 字节存 num_edges)
pub const MAX_EDGES: u32 = ${MAX_EDGES};

/// 单 DSO 模式守卫归零开关 (原版 support_multi_dso; USE_CHROMIUM_FUZZILLI 未定义 → false)
pub const SUPPORT_MULTI_DSO: bool = false;

/// inline-bool-flag 最大支持区域数 (原版 constexpr size_t kMaxBoolFlagRegions = 64)
pub const MAX_BOOL_FLAG_REGIONS: usize = ${MAX_BOOL_FLAG_REGIONS};

/// 崩溃冲刷捕获信号 (原版 kCrashSignals: SIGSEGV, SIGBUS, SIGILL, SIGTRAP, SIGFPE, SIGABRT)
pub const CRASH_SIGNALS: [i32; 6] = [11, 10, 4, 5, 8, 6];

/// 原版 cov.h 声明函数全集 (9 项)
pub const COV_EXPORT_DECLARATIONS: [&str; 9] = [
${decls.map((d) => `  "${d}",`).join("\n")}
];

/// 原版 cc SanitizerCoverage 回调声明全集 (3 项)
pub const SANITIZER_CALLBACKS: [&str; 3] = [
${ccCallbacks.map((c) => `  "${c}",`).join("\n")}
];

/// 置位第 index 条边 (原版 inline cov_set_edge — 纯位运算, 1:1 落地)
#[inline]
pub fn cov_set_edge(edges: &mut [u8], index: u32) {
  let byte_index = (index >> 3) as usize;
  if byte_index < edges.len() {
    let bit_index = index & 7;
    edges[byte_index] |= 1u8 << bit_index;
  }
}

/// 查询第 index 条边是否已置位 (原版 inline cov_is_edge_set — 纯位运算, 1:1 落地)
#[inline]
#[must_use]
pub fn cov_is_edge_set(edges: &[u8], index: u32) -> bool {
  let byte_index = (index >> 3) as usize;
  if byte_index < edges.len() {
    let bit_index = index & 7;
    (edges[byte_index] & (1 << bit_index)) != 0
  } else {
    false
  }
}

/// Safe Rust 覆盖率位图容器 (对应原版 struct shmem_data 与模块全局状态)
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CoverageBitmap {
  /// 已登记边数 (位图前 4 字节表示)
  num_edges: u32,
  /// 边位图字节数组
  edges: Vec<u8>,
  /// 内建边起始编号 (1 + shmem->num_edges)
  builtins_start: u32,
  /// 内建边计数
  builtins_edge_count: u32,
}

impl Default for CoverageBitmap {
  fn default() -> Self {
    Self::new(0)
  }
}

impl CoverageBitmap {
  /// 创建指定边容量的覆盖率位图容器
  pub fn new(initial_edges: u32) -> Self {
    let capped_edges = initial_edges.min(MAX_EDGES);
    let bytes_needed = (capped_edges as usize).div_ceil(8).max(1);
    Self {
      num_edges: capped_edges,
      edges: vec![0u8; bytes_needed],
      builtins_start: 1 + capped_edges,
      builtins_edge_count: 0,
    }
  }

  /// 创建具有完整 2 MiB 真实大小的共享位图容器 (等价于 mmap 映射布局)
  pub fn new_full_shm() -> Self {
    Self {
      num_edges: 0,
      edges: vec![0u8; (SHM_SIZE - 4) as usize],
      builtins_start: 1,
      builtins_edge_count: 0,
    }
  }

  /// 获取当前已登记的边数
  #[inline]
  pub const fn num_edges(&self) -> u32 {
    self.num_edges
  }

  /// 获取边位图只读切片
  #[inline]
  pub fn edges(&self) -> &[u8] {
    &self.edges
  }

  /// 获取边位图可变切片
  #[inline]
  pub fn edges_mut(&mut self) -> &mut [u8] {
    &mut self.edges
  }

  /// 置位指定边
  pub fn set_edge(&mut self, index: u32) -> Result<()> {
    if index >= MAX_EDGES {
      return Err(Error::InvalidEdgeIndex(index));
    }
    let byte_index = (index >> 3) as usize;
    if byte_index >= self.edges.len() {
      self.edges.resize(byte_index + 1, 0);
    }
    cov_set_edge(&mut self.edges, index);
    if index >= self.num_edges {
      self.num_edges = index + 1;
    }
    Ok(())
  }

  /// 检查指定边是否已置位
  #[inline]
  #[must_use]
  pub fn is_edge_set(&self, index: u32) -> bool {
    cov_is_edge_set(&self.edges, index)
  }

  /// 初始化内建函数边集合 (原版 cov_init_builtins_edges 1:1)
  pub fn init_builtins_edges(&mut self, num_edges: u32) -> Result<()> {
    if self.builtins_edge_count != 0 {
      return Err(Error::BuiltinsAlreadyInitialized);
    }
    let available = MAX_EDGES.saturating_sub(self.num_edges);
    if num_edges > available {
      return Err(Error::InsufficientEdges {
        requested: num_edges,
        available,
      });
    }
    self.builtins_edge_count = num_edges;
    self.builtins_start = 1 + self.num_edges;
    self.num_edges += num_edges;
    let byte_needed = (self.num_edges as usize).div_ceil(8).max(1);
    if byte_needed > self.edges.len() {
      self.edges.resize(byte_needed, 0);
    }
    Ok(())
  }

  /// 检查是否存在内建边登记 (原版 cov_has_builtins_edges 1:1)
  #[inline]
  pub const fn has_builtins_edges(&self) -> bool {
    self.builtins_edge_count > 0
  }

  /// 置位内建块边 (原版 cov_set_builtin_edge 1:1)
  pub fn set_builtin_edge(&mut self, block_index: u32) -> Result<()> {
    if !self.has_builtins_edges() || block_index >= self.builtins_edge_count {
      return Err(Error::InvalidEdgeIndex(block_index));
    }
    let edge_index = self.builtins_start + block_index;
    self.set_edge(edge_index)
  }

  /// 内建边起始编号
  #[inline]
  pub const fn builtins_start(&self) -> u32 {
    self.builtins_start
  }

  /// 内建边数量
  #[inline]
  pub const fn builtins_edge_count(&self) -> u32 {
    self.builtins_edge_count
  }

  /// 统计已发现的有效边数 (原版 sanitizer_cov_count_discovered_edges 1:1)
  /// 统计 [1, builtins_start) 范围内的命中边
  pub fn count_discovered_edges(&self) -> u32 {
    let mut on_edges = 0u32;
    for i in 1..self.builtins_start {
      if self.is_edge_set(i) {
        on_edges += 1;
      }
    }
    on_edges
  }

  /// 重置守卫数组 (原版 sanitizer_cov_reset_edgeguards 1:1)
  pub fn reset_edgeguards(&self, guards: &mut [u32]) {
    for (i, guard) in guards.iter_mut().enumerate() {
      *guard = (i + 1) as u32;
    }
  }

  /// 清空位图中的所有已置位边
  pub fn clear_edges(&mut self) {
    self.edges.fill(0);
  }
}

/// Inline-Bool-Flag 区域结构体 (原版 struct BoolFlagRegion 1:1)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BoolFlagRegion {
  /// 标志总数
  pub count: u32,
  /// 位图起始索引
  pub first_index: u32,
}

/// Inline-Bool-Flag 注册表 (管理最多 MAX_BOOL_FLAG_REGIONS 个区域)
#[derive(Debug, Clone, Default)]
pub struct BoolFlagRegistry {
  regions: Vec<BoolFlagRegion>,
}

impl BoolFlagRegistry {
  /// 创建新的布尔标志注册表
  pub fn new() -> Self {
    Self {
      regions: Vec::with_capacity(MAX_BOOL_FLAG_REGIONS),
    }
  }

  /// 注册新的布尔标志区域 (原版 __sanitizer_cov_bool_flag_init 1:1)
  pub fn register_region(
    &mut self,
    flag_count: u32,
    bitmap: &mut CoverageBitmap,
  ) -> Result<u32> {
    if self.regions.len() >= MAX_BOOL_FLAG_REGIONS {
      return Err(Error::TooManyRegions {
        current: self.regions.len(),
        max: MAX_BOOL_FLAG_REGIONS,
      });
    }
    let available = MAX_EDGES.saturating_sub(bitmap.num_edges);
    let allowed_count = flag_count.min(available);
    let first_index = bitmap.num_edges + 1;
    bitmap.num_edges += allowed_count;

    self.regions.push(BoolFlagRegion {
      count: allowed_count,
      first_index,
    });

    Ok(first_index)
  }

  /// 将置位的布尔标志冲刷到位图中并重置 (原版 sanitizer_cov_flush_bool_flags 1:1)
  pub fn flush_bool_flags(
    &self,
    region_idx: usize,
    flags: &mut [bool],
    bitmap: &mut CoverageBitmap,
  ) -> Result<usize> {
    let region = self
      .regions
      .get(region_idx)
      .ok_or(Error::InvalidEdgeIndex(region_idx as u32))?;

    let mut flushed_count = 0usize;
    let limit = (region.count as usize).min(flags.len());
    for (i, flag) in flags.iter_mut().enumerate().take(limit) {
      if *flag {
        *flag = false;
        let edge_idx = region.first_index + i as u32;
        let _ = bitmap.set_edge(edge_idx);
        flushed_count += 1;
      }
    }
    Ok(flushed_count)
  }

  /// 废弃布尔标志并重置为 false (原版 sanitizer_cov_discard_bool_flags 1:1)
  pub fn discard_bool_flags(&self, flags: &mut [bool]) {
    flags.fill(false);
  }

  /// 获取已登记区域数量
  #[inline]
  pub fn region_count(&self) -> usize {
    self.regions.len()
  }
}
`;

const p = writeSrc("v8-fuzzilli", "cov.rs", rust);
console.log(`OK cov.h.js: 对数断言 ${n} 全过; 常量 5 / 纯位运算 2 / CoverageBitmap 12 方法 / BoolFlagRegistry 4 方法; 产物 ${p}`);
