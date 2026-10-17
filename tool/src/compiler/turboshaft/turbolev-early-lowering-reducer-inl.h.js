#!/usr/bin/env bun
// tool/src/compiler/turboshaft/turbolev-early-lowering-reducer-inl.h.js
// 机械提取 v8/src/compiler/turboshaft/turbolev-early-lowering-reducer-inl.h
// → v8-turboshaft/src/turbolev_early_lowering_reducer.rs.skel
//
// 提取面: TurbolevEarlyLoweringReducer helper 方法面 (maglev 提前降低);
// CheckInstanceType 的静态根优化锚。
import { readV8, writeSrc, banner } from "../../../lib.js";

const h = readV8("src/compiler/turboshaft/turbolev-early-lowering-reducer-inl.h");

// 1. 哨兵
for (const [re, name] of [
  [/class TurbolevEarlyLoweringReducer : public Next \{/, "reducer 类"],
  [/TURBOSHAFT_REDUCER_BOILERPLATE\(TurbolevEarlyLowering\)/, "boilerplate"],
  [/void CheckInstanceType\(V<Object> input/, "CheckInstanceType"],
  [/V<InternalizedString> CheckedInternalizedString\(/, "CheckedInternalizedString"],
  [/V8_STATIC_ROOTS_BOOL/, "静态根开关"],
  [/UniqueMapOfInstanceType/, "唯一 map 优化"],
  [/UniqueMapRangeOfInstanceTypeRange/, "map 区间优化"],
  [/kIsNotStringMask | kIsNotInternalizedMask/, "字符串位掩码"],
  [/kThinStringTagBit/, "ThinString 位"],
]) {
  if (!re.test(h)) throw new Error(`哨兵缺失: ${name}`);
}

// 2. 公开 helper 方法面
const helpers = [...h.matchAll(/^\s{2}(?:void|V<[\w<>, ]+>|std::optional<[\w<> ]+>)\s+(\w+)\(/gm)]
  .map((m) => m[1])
  .filter((n) => !n.startsWith("Turbolev"));

let rust = banner(
  "src/compiler/turboshaft/turbolev-early-lowering-reducer-inl.h.js",
  "src/compiler/turboshaft/turbolev-early-lowering-reducer-inl.h",
  "Turbolev 前置降低: maglev 操作的提前降低 helper",
);

rust += `
/// 原版 TurbolevEarlyLoweringReducer — TurbolevGraphBuildingPhase 期间
/// 降低部分 maglev 操作的 helper 集 (非正式 REDUCE 面, 见类注释)。
/// v8: src/compiler/turboshaft/turbolev-early-lowering-reducer-inl.h
pub struct TurbolevEarlyLoweringReducer;

impl TurbolevEarlyLoweringReducer {
  /// v8: CheckInstanceType(input, frame_state, feedback, first, last, check_smi)
  /// - check_smi 先 DeoptimizeIf(IsSmi)
  /// - first == last 且 V8_STATIC_ROOTS_BOOL:
  ///     UniqueMapOfInstanceType -> RootEqual 直接判
  /// - 区间且静态根: UniqueMapRangeOfInstanceTypeRange ->
  ///     压缩 map 的 Uint32LessThanOrEqual 区间判
  /// - 否则 LoadMapField + LoadInstanceTypeField + 区间比较
  pub fn check_instance_type(&mut self) { todo!() }

  /// 其余 helper 面 (${helpers.length} 个):
${helpers.map((x) => `  /// v8: turbolev-early-lowering-reducer-inl.h ${x}`).join("\n")}
  pub fn helper(&mut self) { todo!() }
}

/// 位掩码常量依赖 (语义锚):
/// - kIsNotStringMask | kIsNotInternalizedMask: 判非内化字符串
/// - kThinStringTagBit (popcount==1): 判 ThinString, 取 actual 字段
`;

const p = writeSrc("v8-turboshaft", "turbolev_early_lowering_reducer.rs.skel", rust);
console.log(`written ${p}`);
console.log(`helper=${helpers.length}`);
