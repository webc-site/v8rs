#!/usr/bin/env bun
// tool/src/compiler/osr.js
// v8/src/compiler/osr.h + osr.cc
// → v8-compiler-passes/src/osr.rs.skel
// OSR 辅助: 解析 OsrHelper 字段/方法与 FirstStackSlotIndex 公式。
import { readV8, writeSrc, banner } from "../../lib.js";

const h = readV8("src/compiler/osr.h");
const cc = readV8("src/compiler/osr.cc");

const need = (cond, msg) => {
  if (!cond) throw new Error(`osr: ${msg}`);
};

for (const [s, name] of [
  ["class OsrHelper {", "OsrHelper 定义"],
  ["// TurboFan environments do not contain the context.", "环境不含 context 注释"],
  ["return 1 + parameter_count;  // receiver + params", "FirstStackSlotIndex 公式"],
  ["OsrHelper::OsrHelper(OptimizedCompilationInfo* info)", "cc ctor"],
  ["void OsrHelper::SetupFrame(Frame* frame)", "cc SetupFrame"],
]) need(h.includes(s) || cc.includes(s), `哨兵缺失 ${name}`);

// 公式真解析: FirstStackSlotIndex = 1 + parameter_count (体前有注释行)
const fssi = h.match(/static int FirstStackSlotIndex\(int parameter_count\) \{\s*(?:\/\/[^\n]*\n\s*)*return ([^;]+);/);
need(fssi && fssi[1].trim() === "1 + parameter_count", `FirstStackSlotIndex 体 ${fssi && fssi[1]}`);

// cc: SetupFrame 用的源 (真解析取数)
const unopt = cc.match(/parameter_count_\((\w+->)/);
const slots = cc.match(/stack_slot_count_\((\w+::)/);
need(unopt && slots, "cc 初始化列表解析失败");

const body = `${banner("src/compiler/osr.js", "src/compiler/osr.h + osr.cc", "OSR (栈上替换) 帧布局辅助")}
//! osr — OSR 编译的帧准备
//!
//! FirstStackSlotIndex = 1 + parameter_count (receiver + 参数;
//! TurboFan 环境不含 context)。UnoptimizedFrameSlots 返回未优化帧槽位数。

/// 原版 OsrHelper — OSR 细节封装
pub struct OsrHelper {
  /// parameter_count_
  pub parameter_count: usize,
  /// stack_slot_count_
  pub stack_slot_count: usize,
}

impl OsrHelper {
  /// 原版 FirstStackSlotIndex — 首栈槽的环境索引 (receiver + params)
  /// v8: src/compiler/osr.h OsrHelper::FirstStackSlotIndex
  pub fn first_stack_slot_index(parameter_count: i32) -> i32 {
    1 + parameter_count
  }

  /// 原版 UnoptimizedFrameSlots — 未优化帧的栈槽数
  /// v8: src/compiler/osr.h OsrHelper::UnoptimizedFrameSlots
  pub fn unoptimized_frame_slots(&self) -> usize {
    self.stack_slot_count
  }

  /// 原版 SetupFrame — 依 OSR 值栈信息准备 frame
  /// v8: src/compiler/osr.cc OsrHelper::SetupFrame
  pub fn setup_frame(&self, frame: &mut Frame) {
    todo!("v8: src/compiler/osr.cc OsrHelper::SetupFrame")
  }
}
`;

const p = writeSrc("v8-compiler-passes", "osr.rs.skel", body);
console.log(`written ${p}\n公式 FirstStackSlotIndex=1+parameter_count 核对通过; cc init: parameter_count=${unopt[1]} stack_slot_count=${slots[1]}`);
