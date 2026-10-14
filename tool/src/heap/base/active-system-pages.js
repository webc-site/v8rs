#!/usr/bin/env bun
// tool/src/heap/base/active-system-pages.js — active_system_pages.rs 初版生成
// 组→文件映射 (本脚本管辖):
//   v8/src/heap/base/active-system-pages.h + active-system-pages.cc → v8-heap-base/src/active_system_pages.rs
// 规则:
//   - kMaxPages 机械提取验算 (64, 恰好 u64 位宽)
//   - Add/Reduce/Clear/Size 位算术公式原文提取为注释与骨架
import { readV8, writeSrc, banner } from "../../../lib.js";

const h = readV8("src/heap/base/active-system-pages.h");
const cc = readV8("src/heap/base/active-system-pages.cc");

// ---- 常量提取 ----
const kMaxPages = h.match(/static constexpr size_t kMaxPages = (\d+);/);
if (!kMaxPages || Number(kMaxPages[1]) !== 64) throw new Error(`kMaxPages 应为 64, 实得 ${kMaxPages}`);

// ---- 锚点断言 ----
for (const fn of ["Init", "Add", "Reduce", "Clear", "Size"]) {
  if (!new RegExp(`size_t ${fn}\\(`).test(h)) throw new Error(`active-system-pages.h: ${fn} 缺失`);
}
// Add 的掩码公式证据
if (!/RoundDown\(start, page_size\) >> page_size_bits/.test(cc)) throw new Error("cc: Add 起始页位公式不符");
if (!/RoundUp\(end, page_size\) >> page_size_bits/.test(cc)) throw new Error("cc: Add 结束页位公式不符");
if (!/\(\(uint64_t\{1\} << bits\) - 1\) << start_page_bit/.test(cc)) throw new Error("cc: Add 掩码公式不符");

const body = `${banner("src/heap/base/active-system-pages.js", "src/heap/base/active-system-pages.h + active-system-pages.cc", "堆页上系统页活跃位集 (u64)")}
//! ActiveSystemPages: 单个堆页内系统页的活跃位图, u64 承载, kMaxPages=${kMaxPages[1]}。

/// 原最大可跟踪系统页数 (机械提取验算: ${kMaxPages[1]} = u64 位宽)。
pub const K_MAX_PAGES: usize = ${kMaxPages[1]};

/// 原 ActiveSystemPages (active-system-pages.h:17)。
pub struct ActiveSystemPages {
  /// 原私有 bitset_t value_; u64 位对齐承载。
  value: u64,
}

impl Default for ActiveSystemPages {
  fn default() -> Self {
    Self { value: 0 }
  }
}

impl ActiveSystemPages {
  pub fn new() -> Self { Self::default() }
  /// 原 Init (cc:15-24): 清空后加 header 覆盖页。
  pub fn init(&mut self, header_size: usize, page_size_bits: usize, user_page_size: usize) -> usize { todo!("就地修订") }
  /// 原 Add (cc:26-49): 区间页置位, 返回新增页数。
  /// start_page_bit = RoundDown(start)>>bits; end_page_bit = RoundUp(end)>>bits;
  /// mask = bits==64 ? ~0 : ((1<<bits)-1)<<start_page_bit (原文公式提取)。
  pub fn add(&mut self, start: usize, end: usize, page_size_bits: usize) -> usize { todo!("就地修订") }
  /// 原 Reduce (cc:51-56): 以子集替换, 返回移除页数。
  pub fn reduce(&mut self, updated_value: ActiveSystemPages) -> usize { todo!("就地修订") }
  /// 原 Clear (cc:58-62): 全清, 返回移除页数。
  pub fn clear(&mut self) -> usize { todo!("就地修订") }
  /// 原 Size (cc:64-68): 活跃页数 × 页大小。
  pub fn size(&self, page_size_bits: usize) -> usize { todo!("就地修订") }
}
`;

const p = writeSrc("v8-heap-base", "active_system_pages.rs", body);
console.log(`active-system-pages.js: ${p} (kMaxPages=${kMaxPages[1]} + 方法 5 项 + 掩码公式 3 处核对通过)`);
