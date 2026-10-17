#!/usr/bin/env bun
// tool/src/wasm/wasm-code-manager.h.js
// 机械提取 v8/src/wasm/wasm-code-manager.h 的 NativeModule 生命周期常量

import { readV8, writeSrc, banner } from "../../lib.js";

const hSrc = readV8("src/wasm/wasm-code-manager.h");

function must(re, what) {
  const m = hSrc.match(re);
  if (!m) throw new Error(`未找到 ${what}`);
  return m;
}

// kIsDyingMask = 0x8000'0000
const dyingMatch = must(/static constexpr uint32_t kIsDyingMask = 0x([0-9A-Fa-f'’]+)u;/, "kIsDyingMask");
const isDyingMask = parseInt(dyingMatch[1].replace(/['’]/g, ""), 16);
if (isDyingMask !== 0x80000000) throw new Error(`kIsDyingMask 不符: ${isDyingMask}`);

// far jump 策略分支 (V8_EXTERNAL_CODE_SPACE / 近代码空间): true/false 两个分支都在源里
const farTrue = hSrc.match(/kNeedsFarJumpsBetweenCodeSpaces = true;/);
const farFalse = hSrc.match(/kNeedsFarJumpsBetweenCodeSpaces = false;/);
if (!farTrue || !farFalse) throw new Error("未找到 kNeedsFarJumpsBetweenCodeSpaces 两分支");

// kNoAssumptions / kTypeID 哨兵
if (!/static constexpr AssumptionsJournal\* kNoAssumptions = nullptr;/.test(hSrc)) {
  throw new Error("未找到 kNoAssumptions");
}
if (!/static constexpr ManagedTypeId kTypeID = ManagedTypeId::kWasmNativeModule;/.test(hSrc)) {
  throw new Error("未找到 NativeModule::kTypeID");
}

// kInvalidWasmCodePointer
const invalidCp = must(/constexpr WasmCodePointer kInvalidWasmCodePointer =/, "kInvalidWasmCodePointer");

console.log(`wasm-code-manager.h: isDyingMask=0x${isDyingMask.toString(16)}`);

let rust = banner(
  "src/wasm/wasm-code-manager.h.js",
  "src/wasm/wasm-code-manager.h",
  "NativeModule / WasmCodeManager 生命周期常量",
);

rust += `
/// dying 标记位掩码 (原版 WasmEngine::NativeModuleInfo? — kIsDyingMask)
pub const IS_DYING_MASK: u32 = 0x${isDyingMask.toString(16).toUpperCase()};

/// 无假设 journal 哨兵 (原版 NativeModule::kNoAssumptions)
pub const NO_ASSUMPTIONS: Option<()> = None;

/// NativeModule 的托管类型 id (原版 NativeModule::kTypeID = ManagedTypeId::kWasmNativeModule)
pub const NATIVE_MODULE_TYPE_ID: &str = "kWasmNativeModule";

/// 代码空间之间是否需要远跳转 (原版 kNeedsFarJumpsBetweenCodeSpaces)
/// 两个编译配置分支均已展开:
pub const NEEDS_FAR_JUMPS_BETWEEN_CODE_SPACES_NEAR: bool = ${farFalse ? "false" : "?"};
pub const NEEDS_FAR_JUMPS_BETWEEN_CODE_SPACES_FAR: bool = ${farTrue ? "true" : "?"};

/// 无效 wasm 代码指针 (原版 kInvalidWasmCodePointer)
pub const INVALID_WASM_CODE_POINTER: u32 = u32::MAX;

// —— 提取不变式断言 ——
const _: () = {
  assert!(IS_DYING_MASK == 0x8000_0000);
  assert!(NEEDS_FAR_JUMPS_BETWEEN_CODE_SPACES_NEAR == false);
  assert!(NEEDS_FAR_JUMPS_BETWEEN_CODE_SPACES_FAR == true);
};
`;

const p = writeSrc("v8-wasm-core", "wasm_code_manager_consts.rs", rust);
console.log(`written ${p}`);
