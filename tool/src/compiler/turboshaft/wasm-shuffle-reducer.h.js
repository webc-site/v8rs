#!/usr/bin/env bun
// tool/src/compiler/turboshaft/wasm-shuffle-reducer.h.js
// 机械提取 v8/src/compiler/turboshaft/wasm-shuffle-reducer.{h,cc}
// → v8-turboshaft/src/wasm_shuffle_reducer.rs.skel
//
// 提取面: ShuffleSide 枚举; InRange 区间常量; DemandedByteAnalysis 常量
// (kMaxNumOperations/kMaxPassThruDepth); unary/binary_low_half_ops 表;
/// analyzer/reducer 结构面。
import { readV8, writeSrc, banner } from "../../../lib.js";

const h = readV8("src/compiler/turboshaft/wasm-shuffle-reducer.h");
const cc = readV8("src/compiler/turboshaft/wasm-shuffle-reducer.cc");

// 1. ShuffleSide
const ssM = h.match(/enum class ShuffleSide : uint8_t \{([^}]+)\}/);
if (!ssM) throw new Error("未找到 ShuffleSide");
const sides = [...ssM[1].replace(/\/\/[^\n]*/g, "").matchAll(/k(\w+)/g)].map((m) => m[1]);
if (sides.length !== 2) throw new Error(`ShuffleSide ${sides.length} != 2`);

// 2. 区间常量
const inRange = h.match(/constexpr uint8_t left_lower = (\d+);\s*\n\s*constexpr uint8_t left_upper = (\d+);\s*\n\s*constexpr uint8_t right_lower = (\d+);\s*\n\s*constexpr uint8_t right_upper = (\d+);/);
if (!inRange) throw new Error("未找到 InRange 区间常量");

// 3. 分析常量
const maxOps = h.match(/static constexpr int kMaxNumOperations = (\d+);/);
const maxDepth = h.match(/static constexpr uint8_t kMaxPassThruDepth = (\d+);/);
if (!maxOps || !maxDepth) throw new Error("未找到 DemandedByteAnalysis 常量");

// 4. 低半操作表 (Kind::kX 形式)
const unaryCount = [...h.match(/unary_low_half_ops = \{([\s\S]*?)\};/)[1].matchAll(/Kind::k(\w+)/g)].length;
const binCount = [...h.match(/binary_low_half_ops = \{([\s\S]*?)\};/)[1].matchAll(/Kind::k(\w+)/g)].length;
if (unaryCount < 10 || binCount < 3) throw new Error(`低半表 ${unaryCount}/${binCount}`);

// 5. 结构哨兵
for (const [re, name] of [
  [/class DemandedBytes \{/, "DemandedBytes"],
  [/class DemandedByteAnalysis \{/, "DemandedByteAnalysis"],
  [/struct DemandState \{/, "DemandState"],
  [/class MultiUserBits \{/, "MultiUserBits"],
  [/class WasmShuffleAnalyzer \{/, "analyzer"],
  [/struct DeinterleaveLoadCandidate \{/, "解交织候选"],
  [/class ShuffleWindow \{/, "ShuffleWindow"],
  [/struct DotCandidate \{/, "点积候选"],
  [/enum class SearchResult \{ kContinue, kAbort \};/, "搜索结果枚举"],
  [/class WasmShuffleReducer : public Next/, "reducer 类"],
]) {
  if (!re.test(h)) throw new Error(`哨兵缺失: ${name}`);
}

// 6. cc 方法面
const ccMethods = [...new Set([...cc.matchAll(/^std::optional<\w+> WasmShuffleAnalyzer::(\w+)\(|^void WasmShuffleAnalyzer::(\w+)\(|^bool WasmShuffleAnalyzer::(\w+)\(/gm)].map((m) => m[1] || m[2] || m[3]))];

let rust = banner(
  "src/compiler/turboshaft/wasm-shuffle-reducer.h.js",
  "src/compiler/turboshaft/wasm-shuffle-reducer.{h,cc}",
  "wasm shuffle 优化: 按需求字节裁剪与模式识别",
);

rust += `
/// 原版 ShuffleSide — 32 字节 shuffle 掩码的左右操作数侧
/// v8: wasm-shuffle-reducer.h ShuffleSide
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum ShuffleSide {
  /// kLeft: 字节 ${inRange[1]}..=${inRange[2]}
  Left = 0,
  /// kRight: 字节 ${inRange[3]}..=${inRange[4]}
  Right = 1,
}

/// 原版 DemandedByteAnalysis 常量:
/// - kMaxNumOperations = ${maxOps[1]} (分析规模上限, 超出放弃)
/// - kMaxPassThruDepth = ${maxDepth[1]} (穿透深度上限)
/// - 低半操作表: unary ${unaryCount} 个 / binary ${binCount} 个 Opcode
/// v8: wasm-shuffle-reducer.h DemandedByteAnalysis
pub const K_MAX_NUM_OPERATIONS: i32 = ${maxOps[1]};
pub const K_MAX_PASS_THRU_DEPTH: u8 = ${maxDepth[1]};

/// 原版结构面:
/// - DemandedBytes: 每输出字节的需求位集 (u32 掩码)
/// - DemandState / MultiUserBits: 穿透分析状态与多用户位
/// - WasmShuffleAnalyzer: 识别解交织载入 (DeinterleaveLoadCandidate +
///   ShuffleWindow 滑窗) 与点积 (DotCandidate); SearchResult{kContinue,kAbort}
/// - WasmShuffleReducer: 按 analyzer 结果重写 shuffle 链
///   (DeinterleaveLoadShuffle/DeinterleaveLoad 结构)
/// - cc analyzer 方法面 (${ccMethods.length} 个): ${ccMethods.join(", ")}
/// v8: src/compiler/turboshaft/wasm-shuffle-reducer.{h,cc}
pub struct WasmShuffleAnalyzer;
pub struct WasmShuffleReducer;
`;

const p = writeSrc("v8-turboshaft", "wasm_shuffle_reducer.rs.skel", rust);
console.log(`written ${p}`);
console.log(`ShuffleSide=${sides.length} 低半表=${unaryCount}/${binCount} cc方法=${ccMethods.length}`);
