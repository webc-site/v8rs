#!/usr/bin/env bun
// tool/src/wasm/wasm-disassembler.h.js
// 机械提取 v8/src/wasm/wasm-disassembler.h + wasm-disassembler-impl.h 的
// 反汇编器配置常量

import { readV8, writeSrc, banner } from "../../lib.js";

const hSrc = readV8("src/wasm/wasm-disassembler.h");
const implSrc = readV8("src/wasm/wasm-disassembler-impl.h");

// 1. impl.h 配置开关 (DevTools 兼容行为, true 为 legacy)
const skipFnTypes = implSrc.match(/constexpr bool kSkipFunctionTypesInTypeSection = (true|false);/);
if (!skipFnTypes) throw new Error("未找到 kSkipFunctionTypesInTypeSection");
const skipDataNames = implSrc.match(/constexpr bool kSkipDataSegmentNames = (true|false);/);
if (!skipDataNames) throw new Error("未找到 kSkipDataSegmentNames");
const indicesAsComments = implSrc.match(/constexpr IndexAsComment kIndicesAsComments = NamesProvider::(k\w+);/);
if (!indicesAsComments) throw new Error("未找到 kIndicesAsComments");
if (indicesAsComments[1] !== "kIndexAsComment") {
  throw new Error(`kIndicesAsComments 指向异常: ${indicesAsComments[1]}`);
}

// 2. 行缓冲宽
const bufSize = implSrc.match(/static constexpr size_t kBufferSize = (\d+);/);
if (!bufSize) throw new Error("未找到 kBufferSize");
const bufferSize = parseInt(bufSize[1], 10);
if (bufferSize !== 20) throw new Error(`kBufferSize 不符: ${bufferSize}`);

// 3. RecGroup 无效哨兵
const invalidRecGroup = implSrc.match(/static constexpr uint32_t kInvalid = ~0u;/);
if (!invalidRecGroup) throw new Error("未找到 RecGroup::kInvalid");

// 4. FunctionHeader 枚举
const fhMatch = implSrc.match(/enum FunctionHeader : bool \{ kSkipHeader = false, kPrintHeader = true \};/);
if (!fhMatch) throw new Error("未找到 FunctionHeader");

// 5. h 侧入口存在性
if (!/void Disassemble\(/.test(hSrc)) throw new Error("未找到 Disassemble 入口");

console.log(`wasm-disassembler: bufferSize=${bufferSize}`);

let rust = banner(
  "src/wasm/wasm-disassembler.h.js",
  "src/wasm/wasm-disassembler.h + wasm-disassembler-impl.h",
  "反汇编器行为配置常量",
);

rust += `
/// 类型段是否跳过函数类型打印 (原版 kSkipFunctionTypesInTypeSection; true 为 DevTools legacy 行为)
pub const SKIP_FUNCTION_TYPES_IN_TYPE_SECTION: bool = ${skipFnTypes[1]};

/// 是否打印索引注释 (原版 kIndicesAsComments = NamesProvider::kIndexAsComment)
pub const INDICES_AS_COMMENTS: bool = true;

/// 是否跳过数据段名打印 (原版 kSkipDataSegmentNames)
pub const SKIP_DATA_SEGMENT_NAMES: bool = ${skipDataNames[1]};

/// 反汇编行缓冲宽 (原版 MultiLineStringBuilder::kBufferSize; 恰容一个 uint64)
pub const BUFFER_SIZE: usize = ${bufferSize};

/// 无效递归组哨兵 (原版 RecGroup::kInvalid = ~0u)
pub const INVALID_REC_GROUP: u32 = u32::MAX;

/// 函数头打印策略 (对应原版 FunctionHeader : bool)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(bool)]
pub enum FunctionHeader {
  /// kSkipHeader
  SkipHeader = false,
  /// kPrintHeader
  PrintHeader = true,
}

// —— 提取不变式断言 ——
const _: () = {
  assert!(BUFFER_SIZE == 20);
  assert!(INVALID_REC_GROUP == 0xFFFF_FFFF);
  assert!(SKIP_FUNCTION_TYPES_IN_TYPE_SECTION);
  assert!(SKIP_DATA_SEGMENT_NAMES);
};
`;

const p = writeSrc("v8-wasm-core", "wasm_disassembler_consts.rs", rust);
console.log(`written ${p}`);
