#!/usr/bin/env bun
// tool/src/compiler/turboshaft/value-numbering-reducer.h.js
// 机械提取 v8/src/compiler/turboshaft/value-numbering-reducer.h
// → v8-turboshaft/src/value_numbering_reducer.rs.skel
//
// 提取面: Entry 结构; kMaxEpoch; GVN 效果白名单; epoch 失效协议;
// ScopeCounter/DisableValueNumbering 域管理。
import { readV8, writeSrc, banner } from "../../../lib.js";

const h = readV8("src/compiler/turboshaft/value-numbering-reducer.h");

// 1. 哨兵
for (const [re, name] of [
  [/class ValueNumberingReducer : public Next \{/, "reducer 类"],
  [/TURBOSHAFT_REDUCER_BOILERPLATE\(ValueNumbering\)/, "boilerplate"],
  [/static constexpr int kMaxEpoch = std::numeric_limits<int>::max\(\);/, "kMaxEpoch"],
  [/struct Entry \{/, "Entry 结构"],
  [/OpIndex value;/, "value 字段"],
  [/Entry\* depth_neighboring_entry = nullptr;/, "深度邻接链"],
  [/bool IsEmpty\(\) const \{ return hash == 0; \}/, "空判 hash==0"],
  [/class ScopeCounter \{/, "ScopeCounter"],
  [/class DisableValueNumbering \{/, "DisableValueNumbering"],
  [/next_is_bottom_of_assembler_stack<Next>::value \|\|/, "栈底静态断言"],
]) {
  if (!re.test(h)) throw new Error(`哨兵缺失: ${name}`);
}

// 2. GVN 效果白名单
const wlM = h.match(/bool EffectsAllowGVN\(OpEffects effects\) \{\s*return effects\.IsSubsetOf\(OpEffects\(\)([\s\S]*?)\);/);
if (!wlM) throw new Error("未找到效果白名单");
const whitelist = [...wlM[1].matchAll(/\.(\w+)\(\)/g)].map((m) => m[1]);
if (whitelist.length !== 4) throw new Error(`白名单 ${whitelist.length} != 4`);

// 3. 特例
const deoptIf = /std::is_same_v<Op, DeoptimizeIfOp>/.test(h);
const mapLoad = /offsetof\(HeapObject, map_\)/.test(h);

let rust = banner(
  "src/compiler/turboshaft/value-numbering-reducer.h.js",
  "src/compiler/turboshaft/value-numbering-reducer.h",
  "GVN: 开放寻址哈希 + 支配树深度链表",
);

rust += `
/// 原版 Entry — 开放寻址表项 (hash==0 为空; 深度邻接单链表支持
/// 整深度批量摘除; epoch 避免全表清零)
/// v8: value-numbering-reducer.h ValueNumberingReducer::Entry
#[derive(Clone, Copy)]
pub struct Entry {
  pub value: OpIndex,
  pub block: BlockIndex,
  pub hash: usize,
  pub depth_neighboring_entry: Option<usize>,
  /// kMaxEpoch = i32::MAX
  pub epoch: i32,
}

/// GVN 效果白名单 (IsSubsetOf):
${whitelist.map((w) => `/// - CanDependOnChecks 等: ${w}`).join("\n")}
/// 即 ${whitelist.join(" + ")}
/// v8: value-numbering-reducer.h EffectsAllowGVN
pub fn effects_allow_gvn(effects: OpEffects) -> bool { todo!() }

/// 特例:
/// - DeoptimizeIfOp: 尽管含 CanDeopt 也允许 GVN
/// - LoadOp: offset == offsetof(HeapObject, map_) 且无 index 的
///   map 载入不 GVN (string map 可能被 GC 改写, epoch 不递增)
/// v8: value-numbering-reducer.h EffectsAllowGVN(const Op&)
pub const GVN_EXCEPTION_COUNT: usize = ${deoptIf && mapLoad ? 2 : 0};

/// 原版 ScopeCounter / DisableValueNumbering — 有意复制指令时
/// 关闭 GVN 的域 (构造进, 析构出)
/// v8: value-numbering-reducer.h ScopeCounter
pub struct DisableValueNumbering<'r>(pub &'r mut ScopeCounter);
pub struct ScopeCounter { scopes: i32 }

/// 原版 ValueNumberingReducer — 依赖支配序迭代: 下降时入表,
/// 回升时按 depths_heads_ 链表整深度摘除; 可写操作递增 current_epoch_
/// 使旧表项整体失效。
/// v8: src/compiler/turboshaft/value-numbering-reducer.h
pub struct ValueNumberingReducer;
`;

const p = writeSrc("v8-turboshaft", "value_numbering_reducer.rs.skel", rust);
console.log(`written ${p}`);
console.log(`白名单=${whitelist.length} 特例=${(deoptIf ? 1 : 0) + (mapLoad ? 1 : 0)}`);
