#!/usr/bin/env bun
// tool/src/deoptimizer/translation-opcode.h.js
// 机械提取 v8/src/deoptimizer/translation-opcode.h:
// TRANSLATION_OPCODE_LIST 三段展开 (JS帧/帧/数据), 操作数计数表, 分类谓词

import { readV8, writeSrc, banner } from "../../lib.js";

const hSrc = readV8("src/deoptimizer/translation-opcode.h");

// 1. 三段表机械提取
function parseList(listName) {
  const m = hSrc.match(new RegExp(`#define ${listName}\\(V\\)([\\s\\S]*?)\\n\\n`));
  if (!m) throw new Error(`未找到 ${listName}`);
  const items = [];
  for (const line of m[1].split("\n")) {
    const t = line.trim();
    if (!t.startsWith("V(") && !t.startsWith("IF_WASM(V,")) continue;
    const isWasm = t.startsWith("IF_WASM");
    const mm = t.replace(/^IF_WASM\(V,\s*/, "").replace(/^V\(/, "").match(/^(\w+),\s*(\d+)\)/);    if (!mm) throw new Error(`${listName} 行解析失败: ${t}`);
    items.push({ name: mm[1], operands: parseInt(mm[2], 10), wasm: isWasm });
  }
  return items;
}

const jsFrames = parseList("TRANSLATION_JS_FRAME_OPCODE_LIST");
const frames = parseList("TRANSLATION_FRAME_OPCODE_LIST");
// 全表枚举 = JS 帧段 + 帧段 + 主表自有数据项 (原版 TRANSLATION_OPCODE_LIST 复用子表)
const ownData = parseList("TRANSLATION_OPCODE_LIST");
if (ownData.length !== 36) throw new Error(`主表自有数据项应 36, 实得 ${ownData.length}`);
const opcodes = [...jsFrames, ...frames, ...ownData];
if (opcodes.length !== 47) throw new Error(`全表应 47, 实得 ${opcodes.length}`);
// 首尾锚点
if (opcodes[0].name !== "INTERPRETED_FRAME_WITH_RETURN" || opcodes[opcodes.length - 1].name !== "MATCH_PREVIOUS_TRANSLATION") {
  throw new Error("操作码序漂移");
}
// 操作数上限断言 (原版 kMaxTranslationOperandCount = 6, 逐项 static_assert)
const maxOps = 6;
const over = opcodes.filter((o) => o.operands > maxOps);
if (over.length) throw new Error(`操作数越界: ${over.map((o) => o.name).join(",")}`);

// 2. 边界常量 (机械求值)
const kNumJsFrame = jsFrames.length;
const kNumFrame = kNumJsFrame + frames.length;
const kNumAll = opcodes.length;

function ident(name) {
  return name.toLowerCase().replace(/(^|_)(\w)/g, (_, __, c) => c.toUpperCase());
}

let rust = banner("src/deoptimizer/translation-opcode.h.js", "src/deoptimizer/translation-opcode.h", "翻译字节码操作码表");

rust += `
/// 翻译字节码操作码 (原版 enum class TranslationOpcode, TRANSLATION_OPCODE_LIST 1:1 展开;
/// 段序: JS 帧 → 帧 → 数据项)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum TranslationOpcode {
`;
for (const o of opcodes) {
  rust += `  /// 操作数 ${o.operands}${o.wasm ? " (仅 WebAssembly)" : ""}\n  ${ident(o.name)} = ${opcodes.indexOf(o)},\n`;
}
rust += `}

impl TranslationOpcode {
  /// JS 帧操作码数 (原版 kNumTranslationJsFrameOpcodes)
  pub const NUM_JS_FRAME: usize = ${kNumJsFrame};
  /// 帧操作码总数 = JS 帧 + 通用帧 (原版 kNumTranslationFrameOpcodes)
  pub const NUM_FRAME: usize = ${kNumFrame};
  /// 全部操作码数 (原版 kNumTranslationOpcodes)
  pub const NUM_ALL: usize = ${kNumAll};
  /// 单操作码操作数上限 (原版 kMaxTranslationOperandCount)
  pub const MAX_OPERAND_COUNT: usize = ${maxOps};

  /// 操作数计数表 (原版 TranslationOpcodeOperandCount)
  #[must_use]
  pub const fn operand_count(self) -> usize {
    match self {
${opcodes.map((o) => `      Self::${ident(o.name)} => ${o.operands},`).join("\n")}
    }
  }

  /// 是否帧操作码 (原版 IsTranslationFrameOpcode: < NUM_FRAME)
  #[must_use]
  pub const fn is_frame_opcode(self) -> bool {
    (self as usize) < Self::NUM_FRAME
  }

  /// 是否 JS 帧操作码 (原版 IsTranslationJsFrameOpcode: < NUM_JS_FRAME;
  /// 边界断言锚点 CONSTRUCT_CREATE_STUB_FRAME)
  #[must_use]
  pub const fn is_js_frame_opcode(self) -> bool {
    (self as usize) < Self::NUM_JS_FRAME
  }

  /// 是否解释器帧 (原版 IsTranslationInterpreterFrameOpcode)
  #[must_use]
  pub const fn is_interpreter_frame_opcode(self) -> bool {
    matches!(self, Self::InterpretedFrameWithReturn | Self::InterpretedFrameWithoutReturn)
  }

  /// 是否 BEGIN (原版 TranslationOpcodeIsBegin)
  #[must_use]
  pub const fn is_begin(self) -> bool {
    matches!(self, Self::BeginWithFeedback | Self::BeginWithoutFeedback)
  }
}
`;

// 3. WASM 分支登记 (IF_WASM 三项; x64/arm64 默认开 WASM)
const wasmItems = opcodes.filter((o) => o.wasm);
if (wasmItems.length !== 3) throw new Error(`IF_WASM 项应 3, 实得 ${wasmItems.length}`);

rust += `
/// WebAssembly 专属操作码 (原版 IF_WASM 展开, ${wasmItems.length} 项; x64/arm64 默认启用 WASM)
pub const WASM_OPCODES: [&str; ${wasmItems.length}] = [${wasmItems.map((o) => `"${o.name}"`).join(", ")}];
`;

const p = writeSrc("v8-deoptimizer", "translation_opcode_generated.rs", rust);
console.log(`written ${p}; JS帧=${jsFrames.length}, 帧=${frames.length}, 全表=${opcodes.length}`);
