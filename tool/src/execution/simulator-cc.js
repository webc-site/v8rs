#!/usr/bin/env bun
// v8/src/execution/simulator-base.cc + arm64/simulator-logic-arm64.cc → v8-execution-simulator/src/simulator_cc.rs
// 提取规则面: SimulatorBase 静态生命周期协议 + half(float16) 桥接 + FP NaN 位模式常量 + 指令语义方法清单
import { readV8, writeSrc, banner } from "../../lib.js";

const GEN = "src/execution/simulator-cc.js";
const FILES = [
  "src/execution/simulator-base.cc",
  "src/execution/arm64/simulator-logic-arm64.cc",
];
const txt = Object.fromEntries(FILES.map((f) => [f, readV8(f)]));

const base = txt[FILES[0]];
const logic = txt[FILES[1]];

// ---- 对数: simulator-base.cc 静态方法清单 ----
const baseMethods = [...base.matchAll(/void SimulatorBase::(\w+)\(/g)].map((m) => m[1]);
const must = (cond, what) => { if (!cond) throw new Error(`对数失败: ${what}`); };
must(baseMethods.includes("InitializeOncePerProcess"), "SimulatorBase::InitializeOncePerProcess");
must(baseMethods.includes("GlobalTearDown"), "SimulatorBase::GlobalTearDown");
// 静态成员: redirection_mutex_/redirection_/i_cache_mutex_/i_cache_ 四件套
const statics = [...base.matchAll(/^(base::Mutex\*|Redirection\*|base::CustomMatcherHashMap\*) SimulatorBase::(\w+) = nullptr;/gm)];
must(statics.length === 4, `SimulatorBase 静态成员 4 件, 实得 ${statics.length}`);

// ---- 对数: simulator-logic-arm64.cc 指令语义方法清单 ----
// 形如 `void Simulator::ld1(...)` / `double Simulator::FPToDouble(...)`, 语义助手族 (Visit 分发在 simulator-arm64.cc)
const methods = [...logic.matchAll(/^\w[\w:<>, ]*\s+Simulator::(\w+)\(/gm)].map((m) => m[1]);
must(methods.length >= 300, `Simulator 语义方法 >= 300, 实得 ${methods.length}`);
must(methods.includes("ld1") && methods.includes("FPToDouble"), "ld1/FPToDouble 语义助手在表");

// ---- 对数: half 类与 FP16 位模式 ----
must(/class half \{/.test(logic), "half 类 (fp16 桥接)");
must(logic.includes("fp16_ieee_from_fp32_value"), "fp16_ieee_from_fp32_value");
must(logic.includes("fp16_ieee_to_fp32_value"), "fp16_ieee_to_fp32_value");
const fp16Consts = {
  kFP16DefaultNaN: logic.match(/kFP16DefaultNaN\s*=\s*(0x[0-9a-fA-F]+)/)?.[1],
  kHQuietNanMask: logic.match(/kHQuietNanMask\s*=\s*(0x[0-9a-fA-F]+)/)?.[1],
};
// 这两个常量定义在 simulator-arm64.h, logic 只消费; 消费点必须存在
must(logic.includes("kFP16DefaultNaN") && logic.includes("kHQuietNanMask"), "FP16 NaN 常量消费点");

// ---- 对数: CachePage 与 ICache 协议 (base.cc) ----
must(base.includes("CachePage"), "CachePage ICache 页");
must(base.includes("ICacheMatch"), "ICacheMatch 匹配器");
must(base.includes("DeleteChain"), "Redirection::DeleteChain");

const body = `${banner(GEN, FILES.join(", "), "SimulatorBase 生命周期 + simulator-logic 指令语义清单 (规则面基线, 函数体 todo)")}
//! SimulatorBase 与 arm64 指令语义模拟器的规则面。
//!
//! simulator-base.cc: 进程级单例协议 (重定向链/指令缓存, 互斥锁四件套);
//! simulator-logic-arm64.cc: Visit 分发 + FP half 桥接, 函数体归语义轮填体。

// ══ SimulatorBase (simulator-base.cc) ══

/// 静态成员四件套: redirection_mutex / redirection / i_cache_mutex / i_cache
/// (cc:14-25; Rust 版收敛为 OnceLock 进程单例, 字段保留语义名)
pub const REDIRECTION_MUTEX: &str = "redirection_mutex_";
pub const REDIRECTION: &str = "redirection_";
pub const I_CACHE_MUTEX: &str = "i_cache_mutex_";
pub const I_CACHE: &str = "i_cache_";

/// InitializeOncePerProcess (cc:27-38): 首次调用建锁 + CustomMatcherHashMap(ICacheMatch)
pub fn initialize_once_per_process() { todo!("就地修订: cc:27-38, OnceLock 收敛") }

/// GlobalTearDown (cc:40-58): 删锁/删重定向链/逐 CachePage 释放 i_cache
pub fn global_tear_down() { todo!("就地修订: cc:40-58") }

// ══ half 桥接 (simulator-logic-arm64.cc:16-49) ══

/// FP16 位模式: 默认 NaN 与 quiet-NaN 掩码 (定义在 simulator-arm64.h, 此处消费锚)
pub const K_FP16_DEFAULT_NAN: u16 = ${fp16Consts.kFP16DefaultNaN ?? "todo_fp16_default_nan"};
pub const K_H_QUIET_NAN_MASK: u16 = ${fp16Consts.kHQuietNanMask ?? "todo_h_quiet_nan_mask"};

/// half(float16) 桥接: fp16_ieee_from_fp32_value / fp16_ieee_to_fp32_value (cc:20-21/24)
pub struct Half { pub bits: u16 }

impl Half {
  /// half(float) — fp32 → fp16 位模式
  pub fn from_f32(v: f32) -> Self { todo!("就地修订: fp16_ieee_from_fp32_value") }
  /// operator float() — fp16 位模式 → fp32
  pub fn to_f32(self) -> f32 { todo!("就地修订: fp16_ieee_to_fp32_value") }
  /// operator-() — 符号位翻转 (cc:27 bits ^ 0x8000)
  pub fn neg(self) -> Self { Self { bits: self.bits ^ 0x8000 } }
  /// ToQuietNaN (cc:35-37 bits | kHQuietNanMask)
  pub fn to_quiet_nan(self) -> Self { Self { bits: self.bits | K_H_QUIET_NAN_MASK } }
}

// ══ 指令语义助手面 (simulator-logic-arm64.cc, 共 ${methods.length} 个 Simulator:: 方法) ══

/// 语义方法名全表 (函数体归填体轮, 表本身供覆盖率与分派骨架对数)
pub const LOGIC_METHODS: &[&str] = &[
${methods.map((v) => `  "${v}",`).join("\n")}
];
`;

writeSrc("v8-execution-simulator", "simulator_cc.rs", body);
console.log(`simulator-cc.js: SimulatorBase 4 静态 + ${baseMethods.length} 方法 / half 桥接 / 语义助手 ${methods.length} ✓`);
