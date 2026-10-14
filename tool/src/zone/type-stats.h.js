#!/usr/bin/env bun
// tool/src/zone/type-stats.h.js
// 机械提取 v8/src/zone/type-stats.{h,cc}: 精确 zone 统计协议 (V8_ENABLE_PRECISE_ZONE_STATS)

import { readV8, writeSrc, banner } from "../../lib.js";

const hSrc = readV8("src/zone/type-stats.h");
const ccSrc = readV8("src/zone/type-stats.cc");

// 前置: 双模态 (精确统计开 → 完整类; 关 → 无类定义, 引用处自行空转)
if (!hSrc.includes("#ifdef V8_ENABLE_PRECISE_ZONE_STATS")) throw new Error("缺 PRECISE 分支");
if (!hSrc.includes("#endif  // V8_ENABLE_PRECISE_ZONE_STATS")) throw new Error("缺 PRECISE 收尾");

// 精确模式接口面
const fns = [...hSrc.matchAll(/\n  void (Add\w+)\(size_t bytes\)/g)].map((m) => m[1]);
if (fns.join() !== "AddAllocated,AddDeallocated") throw new Error(`统计接口漂移: ${fns.join()}`);
// 输出面 (stdout 打印注释锚定)
if (!/Prints recorded statisticts to stdout/.test(hSrc)) throw new Error("缺输出注释锚点");

// cc 侧: 不完整类型记 1 字节 (sizeof(char))
if (!ccSrc.includes("kIsIncomplete") && !hSrc.includes("kIsIncomplete")) throw new Error("不完整类型分支漂移");

let rust = banner("src/zone/type-stats.h.js", "src/zone/type-stats.{h,cc}", "Zone 类型统计骨架");

rust += `
/// 精确 zone 统计分支 (原版 #ifdef V8_ENABLE_PRECISE_ZONE_STATS; 默认关闭 → 空实现)
pub mod type_stats_branch {
  /// 当前目标: x64/arm64 常规构建关闭精确统计
  pub const PRECISE_ZONE_STATS_ENABLED: bool = false;
  /// 接口面 (对数)
  pub const ADD_METHODS: [&str; ${fns.length}] = [${fns.map((f) => `"${f}"`).join(", ")}];
}

/// 统计条目 (原版 StatsEntry)
#[derive(Debug, Clone, Copy, Default)]
pub struct StatsEntry {
  /// 分配次数 (原版 allocation_count)
  pub allocation_count: u64,
  /// 累计分配字节 (原版 allocated_bytes)
  pub allocated_bytes: u64,
  /// 累计释放字节 (原版 deallocated_bytes)
  pub deallocated_bytes: u64,
  /// 实例宽 (不完整类型/void 记 sizeof(char)=1, 原版 kIsIncomplete 分支)
  pub instance_size: usize,
}

/// TypeStats 签名骨架 (语义见手写 zone/type_stats.rs)
pub mod type_stats_skel {
  use super::StatsEntry;

  /// 按类型标签累加分配 (原版 AddAllocated<TypeTag>)
  pub fn add_allocated(_bytes: usize) {
    unimplemented!("骨架")
  }
  /// 按类型标签累加释放 (原版 AddDeallocated<TypeTag>)
  pub fn add_deallocated(_bytes: usize) {
    unimplemented!("骨架")
  }
  /// 汇总打印 (原版 Print)
  pub fn print() -> String {
    unimplemented!("骨架")
  }
  /// 存储协议: type_index → StatsEntry 无序表 (原版 map_)
  pub fn entry_table() -> Vec<(std::any::TypeId, StatsEntry)> {
    unimplemented!("骨架")
  }
}
`;

const p = writeSrc("v8-zone", "zone/type_stats_generated.rs", rust);
console.log(`written ${p}; 接口=${fns.length}, HashMap锚点=true`);
