#!/usr/bin/env bun
// tool/src/compiler/turboshaft/typeswitch.h.js
// 机械提取 v8/src/compiler/turboshaft/typeswitch.h
// → v8-turboshaft/src/typeswitch.rs.skel
//
// 提取面: MatchCase 特化清单; TypeswitchBuilder::ControlState 枚举;
// MatchState 缓存三件套; 顺序匹配语义。
import { readV8, writeSrc, banner } from "../../../lib.js";

const h = readV8("src/compiler/turboshaft/typeswitch.h");

// 1. MatchCase 特化清单
const cases = [...h.matchAll(/^struct MatchCase<([\w:, <>.]+)> \{/gm)].map((m) => m[1].trim());
if (cases.length !== 14) throw new Error(`MatchCase 特化 ${cases.length} != 14`);
for (const must of ["Smi", "HeapNumber", "String", "JSReceiver", "Object", "Union<Ts...>"]) {
  if (!cases.some((c) => c === must)) throw new Error(`MatchCase 缺 ${must}`);
}

// 2. ControlState 枚举
const csM = h.match(/enum ControlState : int \{([^}]+)\}/);
if (!csM) throw new Error("未找到 ControlState");
const states = [...csM[1].replace(/\/\/[^\n]*/g, "").matchAll(/\bk(\w+)\b/g)].map((m) => m[1]);
if (states.length !== 5) throw new Error(`ControlState ${states.length} != 5`);

// 3. 哨兵
for (const [re, name] of [
  [/class MatchState \{/, "MatchState"],
  [/V<HeapObject> value\(\)/, "value 缓存"],
  [/OptionalV<Map> map_;/, "map 缓存"],
  [/OptionalV<Word32> instance_type_;/, "instance_type 缓存"],
  [/class TypeswitchBuilder \{/, "TypeswitchBuilder"],
  [/if \(!bound\) FATAL\("CASE of TYPESWITCH is unreachable"\)/, "不可达 FATAL"],
  [/can_be_smi_ = base::tmp::contains_v<types, Smi> \|\|/, "Smi 预判"],
  [/struct AssemblerWrapper/, "AssemblerWrapper"],
]) {
  if (!re.test(h)) throw new Error(`哨兵缺失: ${name}`);
}

let rust = banner(
  "src/compiler/turboshaft/typeswitch.h.js",
  "src/compiler/turboshaft/typeswitch.h",
  "TYPESWITCH: 按对象类型顺序分派 (Torque 语义)",
);

rust += `
/// 原版 TypeswitchBuilder::ControlState — 两遍构建状态机
/// v8: typeswitch.h TypeswitchBuilder::ControlState
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum ControlState {
  /// kUninitialized
  Uninitialized = 0,
  /// kCollectCases: 第一遍收集 case + 分配入口块
  CollectCases = 1,
  /// kEmitBranches: 第二遍前段发类型检查与条件跳转
  EmitBranches = 2,
  /// kEmitCases: 第二遍后段绑定块, 发 case 体
  EmitCases = 3,
  /// kDone
  Done = 4,
}

/// 原版 MatchState — case 间共享的缓存 (Smi case 已在外层剥离,
/// value 恒为 HeapObject)
/// v8: typeswitch.h MatchState
pub struct MatchState {
  pub value: V<HeapObject>,
  /// map_ 惰性 LoadMapField
  pub map: OptionV<Map>,
  /// instance_type_ 惰性 LoadInstanceTypeField(map)
  pub instance_type: OptionV<Word32>,
}

/// 原版 MatchCase<T> 特化面 (${cases.length} 个):
${cases.map((c) => `/// - MatchCase<${c}>`).join("\n")}
/// 每个特化提供 static Match(Asm, state, target);
/// Union<Ts...> 自动展开为逐成员 MatchCase。
/// 语义: case 按定义序检查; 全不可达的 CASE 报错,
/// 尾部宽 case (如 Object) 允许部分类型不可达。
/// v8: src/compiler/turboshaft/typeswitch.h MatchCase
pub trait MatchCase {
  /// v8: typeswitch.h MatchCase<T>::Match
  fn match(asm: &mut Assembler, state: &mut MatchState, target: Block);
}

/// 原版 TypeswitchBuilder — BeginCase 按 ControlState 三段分派;
/// can_be_smi_ = 类型集含 Smi/Object 时设 smi_target_
/// v8: typeswitch.h TypeswitchBuilder
pub struct TypeswitchBuilder {
  pub control_state: ControlState,
}
`;

const p = writeSrc("v8-turboshaft", "typeswitch.rs.skel", rust);
console.log(`written ${p}`);
console.log(`MatchCase=${cases.length} ControlState=${states.length}`);
