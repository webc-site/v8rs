#!/usr/bin/env bun
// tool/src/execution/simulator.js — ARM64 模拟器常量与基类契约提取
// 覆盖: src/execution/simulator.h + simulator-base.h + arm64/simulator-arm64.h/.cc
// 产出: v8-execution-simulator/src/constants.rs
// 复跑幂等: 纯文本扫描 + 确定性输出。
import { banner, readV8, writeSrc } from "../../lib.js";

const GEN = "src/execution/simulator.js";
const FILES = [
  "src/execution/simulator.h",
  "src/execution/simulator-base.h",
  "src/execution/arm64/simulator-arm64.h",
  "src/execution/arm64/simulator-arm64.cc",
];
const txt = Object.fromEntries(FILES.map((f) => [f, readV8(f)]));
const die = (m) => { throw new Error(`[simulator.js] ${m}`); };

const hBase = txt["src/execution/simulator-base.h"];
const hSim = txt["src/execution/arm64/simulator-arm64.h"];
const ccSim = txt["src/execution/arm64/simulator-arm64.cc"];

// 守卫原版特征针
if (!hBase.includes("class SimulatorBase {")) die("simulator-base.h 缺 SimulatorBase 类针");
if (!hSim.includes("class Simulator : public DecoderVisitor, public SimulatorBase")) die("simulator-arm64.h 缺 Simulator 类针");
if (!ccSim.includes("kEndOfSimAddress = nullptr;")) die("simulator-arm64.cc 缺 kEndOfSimAddress");
if (!ccSim.includes("set_dreg_bits(i, 0x7FF000007F800001UL);")) die("simulator-arm64.cc 缺 NaN 初始化针");

const out = [];
out.push(banner(GEN, FILES.join(", "), "ARM64 模拟器常量与执行边界 (kEndOfSimAddress, 寄存器初始值, 栈与标志位)"));
out.push(`
/// 模拟器结束执行地址哨兵 (对应原版 Simulator::kEndOfSimAddress = nullptr 即 0)
pub const K_END_OF_SIM_ADDRESS: u64 = 0;

/// 模拟器默认栈空间大小 (1 MiB)
pub const DEFAULT_STACK_SIZE: usize = 1024 * 1024;

/// AAPCS64 栈指针 16 字节对齐
pub const STACK_ALIGNMENT: usize = 16;

/// 浮点寄存器默认 NaN 双精度位模式 (对应原版 0x7FF000007F800001UL)
pub const FP_DEFAULT_NAN_BITS: u64 = 0x7FF0_0000_7F80_0001;

/// NZCV 条件标志位掩码 (bit 31..28)
pub const N_FLAG_MASK: u32 = 0x8000_0000;
pub const Z_FLAG_MASK: u32 = 0x4000_0000;
pub const C_FLAG_MASK: u32 = 0x2000_0000;
pub const V_FLAG_MASK: u32 = 0x1000_0000;

// ── 编译期静态断言 ──
const _: () = {
  assert!(K_END_OF_SIM_ADDRESS == 0);
  assert!(STACK_ALIGNMENT == 16);
  assert!(DEFAULT_STACK_SIZE == 1048576);
  assert!(FP_DEFAULT_NAN_BITS == 0x7FF0_0000_7F80_0001);
  assert!(N_FLAG_MASK == 0x8000_0000);
  assert!(Z_FLAG_MASK == 0x4000_0000);
  assert!(C_FLAG_MASK == 0x2000_0000);
  assert!(V_FLAG_MASK == 0x1000_0000);
};
`);

writeSrc("v8-execution-simulator", "constants.rs", out.join("\n") + "\n");
console.log(`OK simulator.js: 边界常量提取 → v8-execution-simulator/src/constants.rs`);
