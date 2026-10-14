#!/usr/bin/env bun
// tool/src/wasm/pgo.h.js
// 机械提取 v8/src/wasm/pgo.{h,cc} 的 ProfileInformation 骨架与导出函数签名

import { readV8, writeSrc, banner } from "../../lib.js";

const hSrc = readV8("src/wasm/pgo.h");
const ccSrc = readV8("src/wasm/pgo.cc");

if (!/class ProfileInformation/.test(hSrc)) throw new Error("未找到 ProfileInformation");

// cc 侧导出函数
if (!ccSrc.includes("void DumpProfileToFile(")) throw new Error("未找到 DumpProfileToFile");
if (!ccSrc.includes("LoadProfileFromFile(")) throw new Error("未找到 LoadProfileFromFile");

console.log("pgo.h: skeleton");

let rust = banner(
  "src/wasm/pgo.h.js",
  "src/wasm/pgo.{h,cc}",
  "Wasm PGO 配置信息骨架 (签名骨架, 语义面待 Developer 修订)",
);

rust += `
/// 剖析信息: 已执行函数集 + 已 tier-up 函数集 (对应原版 ProfileInformation)
/// 原版禁止拷贝; 语义实现待 Developer 修订
pub struct ProfileInformation {
  executed_functions: Vec<u32>,
  tiered_up_functions: Vec<u32>,
}

impl ProfileInformation {
  /// 原版构造: ProfileInformation(std::vector<uint32_t>, std::vector<uint32_t>)
  #[must_use]
  pub fn new(executed_functions: Vec<u32>, tiered_up_functions: Vec<u32>) -> Self {
    Self { executed_functions, tiered_up_functions }
  }

  /// 原版 executed_functions()
  #[must_use]
  pub fn executed_functions(&self) -> &[u32] {
    &self.executed_functions
  }

  /// 原版 tiered_up_functions()
  #[must_use]
  pub fn tiered_up_functions(&self) -> &[u32] {
    &self.tiered_up_functions
  }
}

/// 导出剖析到文件
/// 原版签名: void DumpProfileToFile(const WasmModule* module,
///   base::Vector<const uint8_t> wire_bytes,
///   std::atomic<uint32_t>* tiering_budget_array) — 语义面待 Developer 修订
pub fn dump_profile_to_file() {
  unimplemented!("登记骨架: 见 v8/src/wasm/pgo.cc")
}

/// 从文件载入剖析
/// 原版签名: unique_ptr<ProfileInformation> LoadProfileFromFile(
///   const WasmModule*, base::Vector<const uint8_t>) — 语义面待 Developer 修订
pub fn load_profile_from_file() -> Option<ProfileInformation> {
  unimplemented!("登记骨架: 见 v8/src/wasm/pgo.cc")
}
`;

const p = writeSrc("v8-wasm-core", "pgo.rs", rust);
console.log(`written ${p}`);
