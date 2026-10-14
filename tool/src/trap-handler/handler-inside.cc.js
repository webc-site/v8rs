#!/usr/bin/env bun
// handler-inside.cc → v8-trap-handler/src/inside.rs 转换脚本
// 规则: 信号处理器内的平台无关故障分类 (IsFaultAddressCovered /
// IsAccessedMemoryCovered)。两函数运行在信号上下文: 自旋锁 + 只读遍历,
// 无分配/无 panic; V8_ENABLE_DRUMBRAKE 分支不迁移 (解释器模式)。
// 原版仅信号上下文调用 (锁契约要求 guard 激活); Rust 侧为可测性暴露 pub 入口,
// pub 入口自持 guard 满足锁契约, 处理器内经 *_impl 调用 (guard 已在)。
import { readV8, writeSrc, banner, lines } from "../../lib.js";

const src = readV8("src/trap-handler/handler-inside.cc");

const regions = {
  fault: lines(src, 38, 87),
  accessed: lines(src, 89, 102),
};

function must(cond, msg) {
  if (!cond) throw new Error(`handler-inside.cc 对数核对失败: ${msg}`);
}
must(
  regions.fault.join("\n").includes("bool IsFaultAddressCovered(uintptr_t fault_addr)"),
  "IsFaultAddressCovered 区间漂移"
);
must(
  // 原文 2 处: DRUMBRAKE 分支 1 处 (不迁移) + 正常路径 1 处 (迁移)
  regions.fault.filter((l) => l.includes("gRecoveredTrapCount.store")).length === 2,
  "恢复计数写入点数不符 (DRUMBRAKE 分支不迁移)"
);
must(
  regions.accessed.join("\n").includes("bool IsAccessedMemoryCovered(uintptr_t addr)"),
  "IsAccessedMemoryCovered 区间漂移"
);

const body = `${banner("src/trap-handler/handler-inside.cc.js", "src/trap-handler/handler-inside.cc")}
//! 信号处理器内的平台无关故障分类; posix 路径由 inside_posix.rs 调用
//! 规则区原文行区间: ${JSON.stringify(Object.keys(regions))}
//! 本模块函数 async-signal-safe: 自旋锁 + 只读遍历, 无分配/无 panic
use std::sync::atomic::Ordering;

use crate::shared::{
  COVERED_MEMORY, METADATA, RECOVERED_TRAP_COUNT, TrapHandlerGuard,
};

/// handler-inside.cc:38 IsFaultAddressCovered 的纯查询实现
/// 查 fault_addr (PC) 是否落在已注册代码对象的受保护指令上; 命中则恢复计数 +1
pub(crate) fn is_fault_address_covered_impl(fault_addr: usize) -> bool {
  // handler-inside.cc:41 处理器内取锁有死锁风险, 由 Locked::lock 的
  // TrapHandlerGuard 契约兜底 (guard 未激活即崩溃, 不留死锁路径)
  let md = METADATA.lock();

  for e in &md.table {
    let Some(data) = e.code_info.as_deref() else { continue };
    if fault_addr >= data.base && fault_addr < data.base.wrapping_add(data.size) {
      // 命中代码对象; offset 必然 < 2GB (wasm 代码上限), u32 截断安全
      let offset = (fault_addr - data.base) as u32;
      debug_assert!(data.base + offset as usize == fault_addr);
      if data.instructions.iter().any(|i| i.instr_offset == offset) {
        // handler-inside.cc:77 命中受保护指令 → 恢复计数 +1。
        // 原版 load+store 两步非原子 (并发下丢计数); fetch_add 同为
        // lock-free 原子操作, async-signal-safe 且计数不丢
        RECOVERED_TRAP_COUNT.fetch_add(1, Ordering::Relaxed);
        return true;
      }
    }
  }
  false
}

/// handler-inside.cc:89 IsAccessedMemoryCovered 的纯查询实现
/// 查被访问地址是否落在已注册覆盖内存 (wasm 内存含保护区间) 内
pub(crate) fn is_accessed_memory_covered_impl(addr: usize) -> bool {
  let records = COVERED_MEMORY.lock();
  records.iter().any(|r| addr >= r.base && addr < r.base.wrapping_add(r.size))
}

/// PC 查询公共入口 (trap-handler-internal.h:137 声明的暴露形态)。
/// 原版仅在信号上下文调用; 此处供注册方/测试在处理器外查询,
/// 自持 guard 满足 MetadataLock 的锁契约
pub fn is_fault_address_covered(fault_addr: usize) -> bool {
  let _guard = TrapHandlerGuard::new();
  is_fault_address_covered_impl(fault_addr)
}

/// 被访问地址查询公共入口 (trap-handler-internal.h:141 声明的暴露形态)
pub fn is_accessed_memory_covered(addr: usize) -> bool {
  let _guard = TrapHandlerGuard::new();
  is_accessed_memory_covered_impl(addr)
}
`;

const p = writeSrc("v8-trap-handler", "inside.rs", body);
console.log(`handler-inside.cc.js: inside.rs 已产出 ${p} (查询 2 x {pub, impl})`);
