#!/usr/bin/env bun
// tool/src/wasm/wasm-serialization.h.js
// 机械提取 v8/src/wasm/wasm-serialization.{h,cc} 的序列化头部布局常量与函数编码标记

import { readV8, writeSrc, banner } from "../../lib.js";

const hSrc = readV8("src/wasm/wasm-serialization.h");
const ccSrc = readV8("src/wasm/wasm-serialization.cc");

// 1. 头部布局: [0] magic [1] version hash [2] CPU features [3] flag hash
//    [4] enabled features [5] compile time imports (变长)
// kVersionHashOffset = kUInt32Size = 4
const versionHashMatch = hSrc.match(/static constexpr size_t kVersionHashOffset = kUInt32Size;/);
if (!versionHashMatch) throw new Error("未找到 kVersionHashOffset");
const K_UINT32_SIZE = 4; // 原版 kUInt32Size
const headerWords = 5; // magic/version/features/flagenabled/flags — 变长 imports 之前
const HEADER_DOC = [
  "magic number (uint32_t)",
  "version hash (uint32_t)",
  "supported CPU features (uint32_t)",
  "flag hash (uint32_t)",
  "enabled features (via flags and OT) (uint32_t)",
];

// 2. .cc 函数编码标记
function mustConstU8(name) {
  const m = ccSrc.match(new RegExp(`constexpr uint8_t ${name} = (\\d+);`));
  if (!m) throw new Error(`未找到 ${name}`);
  return parseInt(m[1], 10);
}
const lazyFunction = mustConstU8("kLazyFunction");
const eagerFunction = mustConstU8("kEagerFunction");
const turboFanFunction = mustConstU8("kTurboFanFunction");
if (!(lazyFunction < eagerFunction && eagerFunction < turboFanFunction)) {
  throw new Error("函数编码标记顺序不符");
}

// 3. magic number 源头: SerializedData::kMagicNumber (src/snapshot/snapshot-data.h)
//    = 0xC0DE0000 ^ ExternalReferenceTable::kSize, 依赖 STATS_COUNTER 宏链,
//    本脚本不折叠具体值, 只登记公式 (禁编造数值)。
const magicRef = ccSrc.match(/writer->Write\(SerializedData::kMagicNumber\);/);
if (!magicRef) throw new Error("未找到 kMagicNumber 写入点");
const magicFormula = hSrc.match(/kMagicNumber/);
if (!magicFormula && !readV8("src/snapshot/snapshot-data.h").match(/kMagicNumber =\s*\n?\s*0xC0DE0000 \^ ExternalReferenceTable::kSize;/)) {
  throw new Error("snapshot-data.h 中 kMagicNumber 公式变更, 需人工对数");
}

console.log(
  `wasm-serialization: headerWords=${headerWords} lazy=${lazyFunction} eager=${eagerFunction} turbofan=${turboFanFunction}`,
);

// 4. 生成 Rust
let rust = banner(
  "src/wasm/wasm-serialization.h.js",
  "src/wasm/wasm-serialization.{h,cc}",
  "Native module 序列化头部布局与函数编码标记",
);

rust += `
/// 序列化头部的定长字段布局 (字偏移), 之后为变长 compile-time imports
/// 原版布局注释: [0] magic [1] version hash [2] CPU features [3] flag hash
/// [4] enabled features [5..] compile time imports (变长)
pub mod header {
  /// 单个 uint32 字段的字节宽 (原版 kUInt32Size)
  pub const UINT32_SIZE: usize = ${K_UINT32_SIZE};

  /// version hash 字段字节偏移 (原版 kVersionHashOffset = kUInt32Size)
  pub const VERSION_HASH_OFFSET: usize = ${K_UINT32_SIZE};

  /// CPU features 字段字节偏移
  pub const CPU_FEATURES_OFFSET: usize = ${K_UINT32_SIZE * 2};

  /// flag hash 字段字节偏移
  pub const FLAG_HASH_OFFSET: usize = ${K_UINT32_SIZE * 3};

  /// enabled features 字段字节偏移
  pub const ENABLED_FEATURES_OFFSET: usize = ${K_UINT32_SIZE * 4};

  /// compile time imports 字段起始字节偏移 (变长区)
  pub const COMPILE_TIME_IMPORTS_OFFSET: usize = ${K_UINT32_SIZE * 5};

  /// 变长区之前的定长字段数
  pub const FIXED_HEADER_WORDS: usize = ${headerWords};
}

${HEADER_DOC.map((d, i) => `/// 头部字 ${i}: ${d}`).join("\n")}

/// 序列化魔数 (原版 SerializedData::kMagicNumber)
/// 登记公式: 0xC0DE0000 ^ ExternalReferenceTable::kSize
/// (kSize 依赖 STATS_COUNTER_NATIVE_CODE_LIST 宏链, 数值待 Developer 侧以
///  编译器产物折叠; 禁在此编造)
pub const MAGIC_NUMBER_FORMULA: &str = "0xC0DE0000 ^ ExternalReferenceTable::kSize";

/// 序列化魔数前缀 (公式中的固定部分)
pub const MAGIC_NUMBER_PREFIX: u32 = 0xC0DE_0000;

/// 函数编码标记: 懒函数 (原版 wasm-serialization.cc kLazyFunction)
pub const LAZY_FUNCTION: u8 = ${lazyFunction};

/// 函数编码标记: 急切函数 (原版 kEagerFunction)
pub const EAGER_FUNCTION: u8 = ${eagerFunction};

/// 函数编码标记: TurboFan 函数 (原版 kTurboFanFunction)
pub const TURBOFAN_FUNCTION: u8 = ${turboFanFunction};

// —— 提取不变式断言 ——
const _: () = {
  assert!(header::VERSION_HASH_OFFSET == 4);
  assert!(header::COMPILE_TIME_IMPORTS_OFFSET == 20);
  assert!(header::FIXED_HEADER_WORDS == 5);
  assert!(LAZY_FUNCTION < EAGER_FUNCTION && EAGER_FUNCTION < TURBOFAN_FUNCTION);
};
`;

const p = writeSrc("v8-wasm-core", "wasm_serialization_consts.rs", rust);
console.log(`written ${p}`);
