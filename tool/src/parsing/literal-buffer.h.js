#!/usr/bin/env bun
// tool/src/parsing/literal-buffer.h.js
// 机械提取 v8/src/parsing/literal-buffer.h 与 literal-buffer.cc 中的常量与预分配尺寸

import { readV8, writeSrc, banner } from "../../lib.js";

const headerSrc = readV8("src/parsing/literal-buffer.h");

// 1. 提取容量与增长常量
const initCapMatch = headerSrc.match(/static constexpr size_t kInitialCapacity = (\d+);/);
if (!initCapMatch) throw new Error("未找到 kInitialCapacity");
const initialCapacity = parseInt(initCapMatch[1], 10);

const growthFactorMatch = headerSrc.match(/static constexpr size_t kGrowthFactor = (\d+);/);
if (!growthFactorMatch) throw new Error("未找到 kGrowthFactor");
const growthFactor = parseInt(growthFactorMatch[1], 10);

const maxGrowthMatch = headerSrc.match(/static constexpr size_t kMaxGrowth = (\d+) \* MB;/);
if (!maxGrowthMatch) throw new Error("未找到 kMaxGrowth");
const maxGrowthMb = parseInt(maxGrowthMatch[1], 10);
const maxGrowth = maxGrowthMb * 1024 * 1024;

// 2. 生成 Rust 代码
const out = `${banner(
  "src/parsing/literal-buffer.h.js",
  "src/parsing/literal-buffer.h",
  "LiteralBuffer 预分配尺寸与容量扩容常量与策略",
)}
/// 初始缓冲区容量 (${initialCapacity} 字节)
pub const K_INITIAL_CAPACITY: usize = ${initialCapacity};

/// 缓冲区增长因子 (${growthFactor} 倍)
pub const K_GROWTH_FACTOR: usize = ${growthFactor};

/// 最大单次增量 (${maxGrowthMb}MB = ${maxGrowth} 字节)
pub const K_MAX_GROWTH: usize = ${maxGrowth};

/// 计算新容量 (对应 LiteralBuffer::NewCapacity)
#[inline]
#[must_use]
pub const fn new_capacity(min_capacity: usize) -> usize {
  if min_capacity < (K_MAX_GROWTH / (K_GROWTH_FACTOR - 1)) {
    min_capacity * K_GROWTH_FACTOR
  } else {
    min_capacity + K_MAX_GROWTH
  }
}
`;

const target = writeSrc("v8-parsing", "literal_buffer_generated.rs", out);
console.log(`成功提取 LiteralBuffer 常量: init=${initialCapacity}, factor=${growthFactor}, maxGrowth=${maxGrowth}`);
console.log(`写入完成: ${target}`);
