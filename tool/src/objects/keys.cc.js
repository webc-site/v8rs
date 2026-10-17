#!/usr/bin/env bun
// tool/src/objects/keys.cc.js — 覆盖 v8/src/objects/keys.cc
//   → v8-objects-collections/src/keys.rs
// 配置枚举 (AddKeyConversion 等) 归 v8-objects-rest/keys.rs (keys.h.js);
// 本脚本登记 .cc 方法面 (KeyAccumulator / FastKeyAccumulator) 与
// 匿名辅助规则 (ContainsOnlyValidKeys/AddKey/CombineKeys) + 宏展开 +
// GetKeys 主流程签名骨架 + todo!("就地修订")。

import { readV8, writeSrc, banner } from "../../lib.js";

const cc = readV8("src/objects/keys.cc");

// ── 方法面提取 + 数量 must 断言 ───────────────────────────────────────────
const keyAcc = [...new Set([...cc.matchAll(/KeyAccumulator::(\w+)\(/g)].map((m) => m[1]))].sort();
const fastKeyAcc = [...new Set([...cc.matchAll(/FastKeyAccumulator::(\w+)\(/g)].map((m) => m[1]))].sort();
if (keyAcc.length !== 29) throw new Error(`KeyAccumulator 方法应 29, 实得 ${keyAcc.length}: ${keyAcc.join(",")}`);
if (fastKeyAcc.length !== 10) throw new Error(`FastKeyAccumulator 方法应 10, 实得 ${fastKeyAcc.length}: ${fastKeyAcc.join(",")}`);
// 入口对数 (禁吞账)
for (const n of ["GetKeys", "AddKey", "AddKeys", "CollectKeys", "CollectOwnKeys", "GetOwnEnumPropertyKeys", "CollectOwnJSProxyKeys"]) {
  if (!keyAcc.includes(n)) throw new Error(`KeyAccumulator 缺 ${n}`);
}
for (const n of ["Prepare", "GetKeys", "GetKeysFast", "GetKeysSlow", "MayHaveSymbols"]) {
  if (!fastKeyAcc.includes(n)) throw new Error(`FastKeyAccumulator 缺 ${n}`);
}

// 匿名辅助 (namespace {} 内)
const anon = ["ContainsOnlyValidKeys", "AddKey", "CombineKeys"];
for (const n of anon) {
  if (!new RegExp(`^(?:static )?(?:bool|int|Handle<FixedArray>) ${n}\\(`, "m").test(cc)) {
    throw new Error(`keys.cc 匿名辅助缺 ${n}`);
  }
}

// 宏展开 (错误传播)
for (const m of ["RETURN_NOTHING_IF_NOT_SUCCESSFUL", "RETURN_FAILURE_IF_NOT_SUCCESSFUL"]) {
  if (!cc.includes(`#define ${m}`)) throw new Error(`keys.cc 缺宏 ${m}`);
}

// 规则锚
const anchors = [
  "if (!(IsName(e) || IsNumber(e))) return false;", // ContainsOnlyValidKeys
  "if (nof_descriptors == 0 && !may_have_elements) return prototype_chain_keys;", // CombineKeys 快路径
  "return FixedArray::RightTrimOrEmpty(isolate, combined_keys,", // 右裁剪
  "if (descs->GetKey(i) == key) return 0;", // AddKey 查重
];
for (const a of anchors) {
  if (!cc.includes(a)) throw new Error(`规则锚失配: ${a.slice(0, 48)}`);
}

const body = `${banner("src/objects/keys.cc.js", "src/objects/keys.cc", "键收集器方法面 (配置枚举归 v8-objects-rest/keys.rs)")}
//! KeyAccumulator / FastKeyAccumulator 的 .cc 方法面。
//!
//! - 配置枚举 (AddKeyConversion/GetKeysConversion/KeyCollectionMode/IndexedOrNamed)
//!   已由 keys.h.js 提取至 v8-objects-rest/src/keys.rs
//! - 快路径 (FastKeyAccumulator::GetKeysFast) 失败回退慢路径 (GetKeysSlow)
//! - 组键规则: 原型链键并入自有键, 去重依据 = receiver map 的描述符键

// ── 匿名辅助规则 (keys.cc:50-98 namespace) ────────────────────────────
/// 键集合有效性: 每项须为 Name 或 Number (ContainsOnlyValidKeys)。
pub const VALID_KEY_PREDICATE: &str = "IsName(e) || IsNumber(e)";
/// CombineKeys 快路径: 无自有描述符且无元素 → 直接复用原型链键数组。
pub const COMBINE_KEYS_FAST_PATH: &str = "nof_descriptors == 0 && !may_have_elements";
/// 组键尾部: 右裁剪到实际长度 (RightTrimOrEmpty)。
pub const COMBINE_KEYS_TRIM: &str = "RightTrimOrEmpty";
/// AddKey 查重: 描述符中已含该键则不计入 (返回 0)。
pub const ADD_KEY_DEDUPE: &str = "descs->GetKey(i) == key";

// ── KeyAccumulator 方法面 (${keyAcc.length} 项, 顺序登记) ────────────────
/// KeyAccumulator 全部 .cc 定义 (对数基准)。
pub const KEY_ACCUMULATOR_METHODS: [&str; ${keyAcc.length}] = [
${keyAcc.map((s) => `  "${s}",`).join("\n")}
];

// ── FastKeyAccumulator 方法面 (${fastKeyAcc.length} 项) ──────────────────
/// FastKeyAccumulator 全部 .cc 定义 (对数基准)。
pub const FAST_KEY_ACCUMULATOR_METHODS: [&str; ${fastKeyAcc.length}] = [
${fastKeyAcc.map((s) => `  "${s}",`).join("\n")}
];

// ── 主流程签名骨架 (就地修订; 参数为 Address/枚举值占位) ───────────────
/// 键收集入口 (KeyAccumulator::GetKeys; receiver + 配置 → 键数组)。
pub fn get_keys(_receiver: u64, _mode: u8, _convert: u8, _filter: u8) -> u64 { todo!("就地修订") }
/// 逐键添加 (AddKey; CONVERT_TO_ARRAY_INDEX 转换分支)。
pub fn add_key(_key: u64, _convert: u8) -> u32 { todo!("就地修订") }
/// 批量并入键 (AddKeys; 数组/类数组双形态)。
pub fn add_keys(_array_like: u64) -> u32 { todo!("就地修订") }
/// 收集 receiver 原型链键 (CollectKeys; 拦截器/代理/私有名分派)。
pub fn collect_keys(_receiver: u64) -> u32 { todo!("就地修订") }
/// 快路径主入口 (FastKeyAccumulator::GetKeys; 失败回退慢路径)。
pub fn get_keys_fast_entry(_receiver: u64, _mode: u8, _convert: u8, _filter: u8) -> u64 { todo!("就地修订") }

// —— 提取不变式断言 ——
const _: () = {
  assert!(KEY_ACCUMULATOR_METHODS.len() == ${keyAcc.length});
  assert!(FAST_KEY_ACCUMULATOR_METHODS.len() == ${fastKeyAcc.length});
  assert!(KEY_ACCUMULATOR_METHODS.contains(&"CollectOwnJSProxyKeys"));
  assert!(FAST_KEY_ACCUMULATOR_METHODS.contains(&"GetKeysFast"));
};
`;

writeSrc("v8-objects-collections", "keys.rs", body);
console.log(
  `keys.rs: KeyAccumulator ${keyAcc.length} + FastKeyAccumulator ${fastKeyAcc.length} + 匿名规则 4 + 骨架 5, 对数通过`,
);
