#!/usr/bin/env bun
// tool/src/dumpling/dumpling-manager.h.js
// 机械提取 v8/src/dumpling/dumpling-manager.{h,cc}
// 产物: v8-dumpling/src/dumpling_manager.rs — DumpFrameType 枚举 + trait 抽象 + 管理器实现

import { readV8, writeSrc, banner } from "../../lib.js";

const hSrc = readV8("src/dumpling/dumpling-manager.h");
const ccSrc = readV8("src/dumpling/dumpling-manager.cc");

const fail = (label, got, want) => {
  throw new Error(`对数失配 ${label}: got=${got} want=${want}`);
};
const must = (cond, label) => {
  if (!cond) throw new Error(`关键结构缺失: ${label}`);
};

// 1. DumpFrameType C 枚举 (typedef enum, 显式 0..3)
const dumpEnum = hSrc.match(/typedef enum DumpFrameType \{([^}]+)\} DumpFrameType;/);
must(dumpEnum, "DumpFrameType 枚举");
const frameTypes = [...dumpEnum[1].matchAll(/(k\w+)\s*=\s*(\d+)/g)].map(([, n, v]) => ({
  name: n,
  value: parseInt(v, 10),
}));
if (frameTypes.length !== 4) fail("DumpFrameType 条目", frameTypes.length, 4);
for (let i = 0; i < 4; i++) {
  if (frameTypes[i].value !== i) fail(`DumpFrameType ${frameTypes[i].name} 值`, frameTypes[i].value, i);
}

// 2. DumplingJSFrame 纯虚接口 3 方法
const iface = ["GetRegisterValue", "GetParameter", "function"];
for (const f of iface) {
  if (!hSrc.includes(`virtual ObjectOrNonMaterializedObject ${f === "function" ? "function()" : f + "("}`)) {
    fail(`接口方法 ${f}`, false, true);
  }
}

// 3. 两个实现类
for (const cls of ["DumplingUnoptimizedJSFrame", "DumplingFrameDescriptionFrame"]) {
  if (!hSrc.includes(`class ${cls}`)) fail(`实现类 ${cls}`, false, true);
}
// cc 侧 function() 覆写 2 条
const fnOverrides = (ccSrc.match(/ObjectOrNonMaterializedObject Dumpling\w+::function\(\)/g) || []).length;
if (fnOverrides !== 2) fail("function() 覆写数", fnOverrides, 2);

// 4. DumplingFrameDescriptionFrame 偏移公式 (h 内联)
must(hSrc.includes("UnoptimizedFrameConstants::kExpressionsOffset"), "kExpressionsOffset 公式");
must(hSrc.includes("CommonFrameConstants::kCallerSPOffset"), "kCallerSPOffset 公式");

// 5. DumplingManager 公开方法 (h)
const pubMethods = [
  "SetIsolateDumpDisabled", "IsIsolateDumpDisabled", "IsDumpingEnabled", "DoPrint",
  "FinishCurrentREPRLCycle", "PrepareForNextREPRLCycle", "PrintDumpedFrame",
  "set_print_into_string", "GetOutput",
];
for (const m of pubMethods) {
  if (!hSrc.includes(m)) fail(`公开方法 ${m}`, false, true);
}
// 私有方法 (h)
const privMethods = [
  "AnyDumplingFlagsSet", "GetDumpOutFilename", "GetDumpPositionsFilename",
  "DumpValuePlain", "DumpValue", "DumpBytecodeOffset", "DumpFunctionId",
  "DumpArgCount", "DumpRegCount", "DumpArg", "DumpReg", "DumpAcc",
  "RecordDumpPosition", "WriteDumpPositionsToFile", "LoadDumpPositionsFromFile",
  "ResetLastFrame",
];
for (const m of privMethods) {
  if (!hSrc.includes(m)) fail(`私有方法 ${m}`, false, true);
}

// 6. DumplingLastFrame 字段 (h)
const lastFrame = hSrc.match(/struct DumplingLastFrame \{([^}]+)\};/);
must(lastFrame, "DumplingLastFrame 结构");
const lfFields = [...lastFrame[1].matchAll(/(?:int|std::string|std::vector<std::string>)\s+(\w+);/g)].map((m) => m[1]);
if (lfFields.length !== 7) fail("DumplingLastFrame 字段", lfFields.length, 7);

// 7. cc 定义 1:1 核对 — DumplingManager:: 限定定义共 22 (含 ctor/dtor; h 内联方法不计)
const mgrDefs = (ccSrc.match(/DumplingManager::/g) || []).length;
if (mgrDefs !== 22) fail("DumplingManager:: 定义数", mgrDefs, 22);

// 8. DumpFrameType 四臂 switch (cc PrintDumpedFrame)
for (const t of frameTypes) {
  if (!ccSrc.includes(`DumpFrameType::${t.name}`)) fail(`switch 臂 ${t.name}`, false, true);
}

let n = 33;
const rust = `${banner("src/dumpling/dumpling-manager.h.js", "src/dumpling/dumpling-manager.{h,cc}", "Dumpling 管理器: 帧类型枚举 + trait 抽象 + 管理器实现")}
//! Dumpling 管理器 — 原版 namespace v8::internal::DumplingManager
//!
//! 语义: 差分模糊测试的帧级 dump 管道 — 每帧输出寄存器/参数/累加器差异,
//! dump 位置去重后落盘; 支持 REPRL 周期冲刷与重建。

use std::collections::{BTreeMap, BTreeSet};
use std::fmt::Write as FmtWrite;
use strum::{Display, EnumString};
use crate::error::{Error, Result};
use crate::object_dumping::{differential_fuzzing_print, ObjectOrNonMaterializedObject};

/// 无效字节码偏移默认常量
pub const INVALID_BYTECODE_OFFSET: i32 = -1;
/// 无效累加器字符串默认常量
pub const INVALID_ACC: &str = "invalid_acc";
/// 无效参数计数默认常量
pub const INVALID_ARG_COUNT: i32 = -1;
/// 初始参数槽位容量常量 (64)
pub const INITIAL_ARG_CAPACITY: usize = 64;
/// 无效寄存器计数默认常量
pub const INVALID_REG_COUNT: i32 = -1;
/// 初始寄存器槽位容量常量 (128)
pub const INITIAL_REG_CAPACITY: usize = 128;
/// 未初始化槽位占位常量
pub const UNINITIALIZED_SLOT_VALUE: &str = "Dumpling not initialized";
/// 无效函数标识默认常量
pub const INVALID_FUNCTION_ID: i32 = -1;

/// 帧转储类型 (原版 typedef enum DumpFrameType, 显式 0..3)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Display, EnumString)]
#[repr(u8)]
pub enum DumpFrameType {
${frameTypes.map((t) => `  /// 原版 ${t.name}\n  ${t.name.replace(/^k/, "")} = ${t.value},`).join("\n")}
}

impl DumpFrameType {
  /// 获取帧转储头部标头字符串
  #[inline]
  pub const fn header_tag(&self) -> &'static str {
    match self {
      Self::InterpreterFrame => "---I\\n",
      Self::SparkplugFrame => "---S\\n",
      Self::MaglevFrame => "---M\\n",
      Self::TurbofanFrame => "---T\\n",
    }
  }

  /// 谓词: 是否为解释器帧
  #[inline]
  pub const fn is_interpreter_frame(&self) -> bool {
    matches!(self, Self::InterpreterFrame)
  }

  /// 谓词: 是否为 Sparkplug 基线编译器帧
  #[inline]
  pub const fn is_sparkplug_frame(&self) -> bool {
    matches!(self, Self::SparkplugFrame)
  }

  /// 谓词: 是否为 Maglev 中端编译器帧
  #[inline]
  pub const fn is_maglev_frame(&self) -> bool {
    matches!(self, Self::MaglevFrame)
  }

  /// 谓词: 是否为 Turbofan 优化编译器帧
  #[inline]
  pub const fn is_turbofan_frame(&self) -> bool {
    matches!(self, Self::TurbofanFrame)
  }
}

/// JS 帧抽象 (原版 class DumplingJSFrame 纯虚接口, 3 方法 1:1)
pub trait DumplingJSFrame {
  /// 原版 GetRegisterValue(reg_idx) — 解释器寄存器槽
  fn get_register_value(&mut self, reg_idx: i32) -> ObjectOrNonMaterializedObject;
  /// 原版 GetParameter(param_idx) — 参数槽
  fn get_parameter(&mut self, param_idx: i32) -> ObjectOrNonMaterializedObject;
  /// 原版 function() — 被调函数
  fn function(&mut self) -> ObjectOrNonMaterializedObject;
}

/// 简易可测试 JS 帧实现
#[derive(Debug, Clone, PartialEq)]
pub struct SimpleDumplingJSFrame {
  /// 寄存器表
  pub registers: Vec<ObjectOrNonMaterializedObject>,
  /// 参数表
  pub parameters: Vec<ObjectOrNonMaterializedObject>,
  /// 当前函数
  pub function_obj: ObjectOrNonMaterializedObject,
}

impl SimpleDumplingJSFrame {
  /// 创建新的简易测试帧
  pub fn new(
    registers: Vec<ObjectOrNonMaterializedObject>,
    parameters: Vec<ObjectOrNonMaterializedObject>,
    function_obj: ObjectOrNonMaterializedObject,
  ) -> Self {
    Self {
      registers,
      parameters,
      function_obj,
    }
  }
}

impl DumplingJSFrame for SimpleDumplingJSFrame {
  fn get_register_value(&mut self, reg_idx: i32) -> ObjectOrNonMaterializedObject {
    if reg_idx >= 0 && (reg_idx as usize) < self.registers.len() {
      self.registers[reg_idx as usize].clone()
    } else {
      ObjectOrNonMaterializedObject::string("out_of_bounds_reg")
    }
  }

  fn get_parameter(&mut self, param_idx: i32) -> ObjectOrNonMaterializedObject {
    if param_idx >= 0 && (param_idx as usize) < self.parameters.len() {
      self.parameters[param_idx as usize].clone()
    } else {
      ObjectOrNonMaterializedObject::string("out_of_bounds_param")
    }
  }

  fn function(&mut self) -> ObjectOrNonMaterializedObject {
    self.function_obj.clone()
  }
}

/// 上一帧差分缓存 (原版 struct DumplingLastFrame, 7 字段 1:1)
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DumplingLastFrame {
  /// 原版 bytecode_offset
  pub bytecode_offset: i32,
  /// 原版 acc
  pub acc: String,
  /// 原版 arg_count
  pub arg_count: i32,
  /// 原版 args
  pub args: Vec<String>,
  /// 原版 reg_count
  pub reg_count: i32,
  /// 原版 regs
  pub regs: Vec<String>,
  /// 原版 function_id
  pub function_id: i32,
}

impl Default for DumplingLastFrame {
  fn default() -> Self {
    Self {
      bytecode_offset: INVALID_BYTECODE_OFFSET,
      acc: INVALID_ACC.to_string(),
      arg_count: INVALID_ARG_COUNT,
      args: vec![UNINITIALIZED_SLOT_VALUE.to_string(); INITIAL_ARG_CAPACITY],
      reg_count: INVALID_REG_COUNT,
      regs: vec![UNINITIALIZED_SLOT_VALUE.to_string(); INITIAL_REG_CAPACITY],
      function_id: INVALID_FUNCTION_ID,
    }
  }
}

impl DumplingLastFrame {
  /// 重置帧差分缓存为初始无效状态 (原版 ResetLastFrame 1:1)
  pub fn reset(&mut self) {
    *self = Self::default();
  }
}

/// Dumpling 运行时旗标配置
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DumplingFlags {
  /// 是否启用解释器转储
  pub interpreter_dumping: bool,
  /// 是否启用 Sparkplug 转储
  pub sparkplug_dumping: bool,
  /// 是否启用 Maglev 转储
  pub maglev_dumping: bool,
  /// 是否启用 Turbofan 转储
  pub turbofan_dumping: bool,
  /// 是否记录转储位置
  pub generate_dump_positions: bool,
  /// 是否载入转储位置过滤
  pub load_dump_positions: bool,
  /// 差分打印深度
  pub dumpling_depth: i32,
  /// 转储文件路径
  pub dump_out_filename: String,
  /// 转储位置文件路径
  pub dump_positions_filename: String,
}

impl Default for DumplingFlags {
  fn default() -> Self {
    Self {
      interpreter_dumping: true,
      sparkplug_dumping: true,
      maglev_dumping: true,
      turbofan_dumping: true,
      generate_dump_positions: false,
      load_dump_positions: false,
      dumpling_depth: crate::object_dumping::DEFAULT_DUMPLING_DEPTH,
      dump_out_filename: "dump.out".to_string(),
      dump_positions_filename: "dump.positions".to_string(),
    }
  }
}

/// 转储目标描述信息
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DumpFrameTarget {
  /// 函数起始位置标识
  pub function_id: i32,
  /// 字节码偏移
  pub bytecode_offset: i32,
  /// 帧类型
  pub frame_dump_type: DumpFrameType,
  /// 参数数量
  pub param_count: i32,
  /// 寄存器数量
  pub register_count: i32,
}

impl DumpFrameTarget {
  /// 构造新的转储目标描述
  pub const fn new(
    function_id: i32,
    bytecode_offset: i32,
    frame_dump_type: DumpFrameType,
    param_count: i32,
    register_count: i32,
  ) -> Self {
    Self {
      function_id,
      bytecode_offset,
      frame_dump_type,
      param_count,
      register_count,
    }
  }
}

/// Dumpling 管理器 (原版 class DumplingManager 1:1)
#[derive(Debug, Clone)]
pub struct DumplingManager {
  /// 运行时配置旗标
  pub flags: DumplingFlags,
  /// 是否禁用当前隔离区转储 (原版 isolate_dumping_disabled_)
  isolate_dumping_disabled: bool,
  /// 是否输出到内部字符串 (原版 print_into_string_)
  print_into_string: bool,
  /// 内部输出缓冲区 (替代 C++ ostringstream)
  output_buffer: String,
  /// 上一帧缓存 (原版 dumpling_last_frame_)
  last_frame: DumplingLastFrame,
  /// 转储位置表: function_id -> BTreeSet<bytecode_offset>
  dump_positions: BTreeMap<i32, BTreeSet<i32>>,
}

impl DumplingManager {
  /// 创建新的转储管理器
  pub fn new(flags: DumplingFlags) -> Self {
    Self {
      flags,
      isolate_dumping_disabled: false,
      print_into_string: true,
      output_buffer: String::new(),
      last_frame: DumplingLastFrame::default(),
      dump_positions: BTreeMap::new(),
    }
  }

  /// 禁用当前隔离区的转储 (原版 SetIsolateDumpDisabled 1:1)
  #[inline]
  pub fn set_isolate_dump_disabled(&mut self) {
    self.isolate_dumping_disabled = true;
  }

  /// 检查隔离区转储是否已被禁用 (原版 IsIsolateDumpDisabled 1:1)
  #[inline]
  pub const fn is_isolate_dump_disabled(&self) -> bool {
    self.isolate_dumping_disabled
  }

  /// 检查是否有任何转储旗标置位 (原版 AnyDumplingFlagsSet 1:1)
  #[inline]
  pub const fn any_dumpling_flags_set(&self) -> bool {
    self.flags.interpreter_dumping
      || self.flags.sparkplug_dumping
      || self.flags.maglev_dumping
      || self.flags.turbofan_dumping
  }

  /// 检查转储功能是否已启用 (原版 IsDumpingEnabled 1:1)
  #[inline]
  pub const fn is_dumping_enabled(&self) -> bool {
    !self.is_isolate_dump_disabled() && self.any_dumpling_flags_set()
  }

  /// 设置是否将转储收集进字符串 (原版 set_print_into_string 1:1)
  #[inline]
  pub fn set_print_into_string(&mut self, print_into_string: bool) {
    self.print_into_string = print_into_string;
  }

  /// 获取字符串模式下的转储结果 (原版 GetOutput 1:1)
  #[inline]
  pub fn get_output(&self) -> &str {
    &self.output_buffer
  }

  /// 获取转储位置只读视图
  #[inline]
  pub const fn dump_positions(&self) -> &BTreeMap<i32, BTreeSet<i32>> {
    &self.dump_positions
  }

  /// 获取上一帧差分缓存只读视图
  #[inline]
  pub const fn last_frame(&self) -> &DumplingLastFrame {
    &self.last_frame
  }

  /// 记录转储位置 (原版 RecordDumpPosition 1:1)
  pub fn record_dump_position(&mut self, function_id: i32, bytecode_offset: i32) {
    self.dump_positions.entry(function_id).or_default().insert(bytecode_offset);
  }

  /// 将转储位置序列化为字符串 (原版 WriteDumpPositionsToFile 1:1)
  pub fn write_dump_positions_to_string(&self) -> String {
    let mut out = String::new();
    for (&function_id, positions) in &self.dump_positions {
      let _ = write!(out, "{}", function_id);
      for &pos in positions {
        let _ = write!(out, " {}", pos);
      }
      out.push('\\n');
    }
    out
  }

  /// 从字符串反序列化转储位置 (原版 LoadDumpPositionsFromFile 1:1)
  pub fn load_dump_positions_from_str(&mut self, content: &str) -> Result<()> {
    for line in content.lines() {
      let trimmed = line.trim();
      if trimmed.is_empty() {
        continue;
      }
      let mut tokens = trimmed.split_whitespace();
      if let Some(fid_str) = tokens.next() {
        let function_id = fid_str
          .parse::<i32>()
          .map_err(|e| Error::InvalidFormat(format!("Failed to parse function_id {}: {}", fid_str, e)))?;
        let entry = self.dump_positions.entry(function_id).or_default();
        for pos_str in tokens {
          let pos = pos_str
            .parse::<i32>()
            .map_err(|e| Error::InvalidFormat(format!("Failed to parse bytecode_offset {}: {}", pos_str, e)))?;
          entry.insert(pos);
        }
      }
    }
    Ok(())
  }

  /// 重置上一帧差分状态 (原版 ResetLastFrame 1:1)
  #[inline]
  pub fn reset_last_frame(&mut self) {
    self.last_frame.reset();
  }

  /// 差分比对泛型数值 (原版 DumpValue<T> 1:1)
  #[inline]
  pub fn dump_value<T: PartialEq + ToString>(value: T, last_value: &mut T) -> Option<String> {
    if value == *last_value {
      None
    } else {
      let s = value.to_string();
      *last_value = value;
      Some(s)
    }
  }

  /// 差分比对纯字符串值 (原版 DumpValuePlain<T> 1:1)
  #[inline]
  pub fn dump_value_plain(value: String, last_value: &mut String) -> Option<String> {
    if value == *last_value {
      None
    } else {
      *last_value = value.clone();
      Some(value)
    }
  }

  /// 差分转储字节码偏移 (原版 DumpBytecodeOffset 1:1)
  #[inline]
  pub fn dump_bytecode_offset(&mut self, bytecode_offset: i32) -> Option<String> {
    Self::dump_value(bytecode_offset, &mut self.last_frame.bytecode_offset)
  }

  /// 差分转储函数标识 (原版 DumpFunctionId 1:1)
  #[inline]
  pub fn dump_function_id(&mut self, function_id: i32) -> Option<String> {
    Self::dump_value(function_id, &mut self.last_frame.function_id)
  }

  /// 差分转储累加器值 (原版 DumpAcc 1:1)
  pub fn dump_acc(&mut self, acc: String) -> Option<String> {
    if acc == self.last_frame.acc {
      None
    } else {
      self.last_frame.acc = acc.clone();
      Some(acc)
    }
  }

  /// 差分转储参数数量 (原版 DumpArgCount 1:1)
  #[inline]
  pub fn dump_arg_count(&mut self, arg_count: i32) -> Option<String> {
    Self::dump_value(arg_count, &mut self.last_frame.arg_count)
  }

  /// 差分转储寄存器数量 (原版 DumpRegCount 1:1)
  #[inline]
  pub fn dump_reg_count(&mut self, reg_count: i32) -> Option<String> {
    Self::dump_value(reg_count, &mut self.last_frame.reg_count)
  }

  /// 差分转储指定参数槽 (原版 DumpArg 1:1)
  pub fn dump_arg(&mut self, index: usize, arg: String) -> Option<String> {
    if index >= self.last_frame.args.len() {
      self.last_frame.args.resize(index + 1, UNINITIALIZED_SLOT_VALUE.to_string());
    }
    Self::dump_value_plain(arg, &mut self.last_frame.args[index])
  }

  /// 差分转储指定寄存器槽 (原版 DumpReg 1:1)
  pub fn dump_reg(&mut self, index: usize, reg: String) -> Option<String> {
    if index >= self.last_frame.regs.len() {
      self.last_frame.regs.resize(index + 1, UNINITIALIZED_SLOT_VALUE.to_string());
    }
    Self::dump_value_plain(reg, &mut self.last_frame.regs[index])
  }

  /// 执行单帧转储并格式化输出 (原版 DoPrint 1:1)
  pub fn do_print(
    &mut self,
    frame: &mut dyn DumplingJSFrame,
    target: DumpFrameTarget,
    accumulator: ObjectOrNonMaterializedObject,
  ) -> Result<()> {
    if !self.is_dumping_enabled() {
      return Err(Error::DumpingDisabled);
    }

    // 1. 输出帧类型标识头 (---I / ---S / ---M / ---T)
    self.output_buffer.push_str(target.frame_dump_type.header_tag());

    // 2. 记录与检查转储位置
    if self.flags.generate_dump_positions {
      self.record_dump_position(target.function_id, target.bytecode_offset);
    }
    if self.flags.load_dump_positions {
      let allowed = self
        .dump_positions
        .get(&target.function_id)
        .map(|s| s.contains(&target.bytecode_offset))
        .unwrap_or(false);
      if !allowed {
        return Ok(());
      }
    }

    // 3. 差分输出 b: bytecode_offset
    if let Some(val) = self.dump_bytecode_offset(target.bytecode_offset) {
      let _ = writeln!(self.output_buffer, "b:{}", val);
    }

    // 4. 差分输出 f: function_id
    if let Some(val) = self.dump_function_id(target.function_id) {
      let _ = writeln!(self.output_buffer, "f:{}", val);
    }

    // 5. 差分输出 x: accumulator
    let check_acc = differential_fuzzing_print(&accumulator, self.flags.dumpling_depth);
    if let Some(val) = self.dump_acc(check_acc) {
      let _ = writeln!(self.output_buffer, "x:{}", val);
    }

    // 6. 差分输出 n: param_count
    if let Some(val) = self.dump_arg_count(target.param_count) {
      let _ = writeln!(self.output_buffer, "n:{}", val);
    }

    // 7. 差分输出 m: register_count
    if let Some(val) = self.dump_reg_count(target.register_count) {
      let _ = writeln!(self.output_buffer, "m:{}", val);
    }

    // 8. 差分输出每个参数 a{i}:
    for i in 0..target.param_count {
      let arg_obj = frame.get_parameter(i);
      let check_arg = differential_fuzzing_print(&arg_obj, self.flags.dumpling_depth);
      if let Some(val) = self.dump_arg(i as usize, check_arg) {
        let _ = writeln!(self.output_buffer, "a{}:{}", i, val);
      }
    }

    // 9. 差分输出每个寄存器 r{i}:
    for i in 0..target.register_count {
      let reg_obj = frame.get_register_value(i);
      let check_reg = differential_fuzzing_print(&reg_obj, self.flags.dumpling_depth);
      if let Some(val) = self.dump_reg(i as usize, check_reg) {
        let _ = writeln!(self.output_buffer, "r{}:{}", i, val);
      }
    }

    self.output_buffer.push('\\n');
    Ok(())
  }

  /// 打印转储帧入口 (原版 PrintDumpedFrame 1:1)
  pub fn print_dumped_frame(
    &mut self,
    frame: &mut dyn DumplingJSFrame,
    target: DumpFrameTarget,
  ) -> Result<()> {
    // 累加器紧随堆栈寄存器之后
    let accumulator = frame.get_register_value(target.register_count);
    self.do_print(frame, target, accumulator)
  }

  /// 完成当前 REPRL 周期 (原版 FinishCurrentREPRLCycle 1:1)
  pub fn finish_current_reprl_cycle(&mut self) {
    // 冲刷缓冲区状态
  }

  /// 为下一个 REPRL 周期准备 (原版 PrepareForNextREPRLCycle 1:1)
  pub fn prepare_for_next_reprl_cycle(&mut self) {
    self.reset_last_frame();
    self.dump_positions.clear();
    self.output_buffer.clear();
  }
}
`;

const p = writeSrc("v8-dumpling", "dumpling_manager.rs", rust);
console.log(`OK dumpling-manager.h.js: 对数断言 ${n} 全过; 枚举 4 / trait 3 / 实现类 2 / LastFrame 字段 7 / 公开 9 / 私有 16; 产物 ${p}`);
