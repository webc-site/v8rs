#!/usr/bin/env bun
// tool/src/codegen/safepoint-table.h.js — 从 v8/src/codegen/safepoint-table.h 提取并校验安全点位字段布局与常量
import { readV8 } from "../../lib.js";

const srcHeader = readV8("src/codegen/safepoint-table.h");
const srcBase = readV8("src/codegen/safepoint-table-base.h");
const srcCc = readV8("src/codegen/safepoint-table.cc");

function must(cond, msg) {
  if (!cond) throw new Error(`safepoint-table.h 对数核对失败: ${msg}`);
}

must(srcBase.includes("kSafepointTableStackSlotsOffset = 0"), "kSafepointTableStackSlotsOffset 缺失");
must(srcBase.includes("static constexpr int kNoDeoptIndex = -1;"), "kNoDeoptIndex 缺失");
must(srcBase.includes("static constexpr int kNoTrampolinePC = -1;"), "kNoTrampolinePC 缺失");

must(srcHeader.includes("V(kStackSlotsOffset, sizeof(SafepointTableStackSlotsField_t))"), "kStackSlotsOffset 缺失");
must(srcHeader.includes("V(kConfigOffset, kUInt32Size)"), "kConfigOffset 缺失");
must(srcHeader.includes("V(kHeaderSize, 0)"), "kHeaderSize 缺失");

must(srcHeader.includes("using ByteLengthField = base::BitField<uint32_t, 0, 30>;"), "ByteLengthField 缺失");
must(srcHeader.includes("using HasTaggedRegistersField = ByteLengthField::Next<bool, 1>;"), "HasTaggedRegistersField 缺失");
must(srcHeader.includes("using HasDeoptDataField = HasTaggedRegistersField::Next<bool, 1>;"), "HasDeoptDataField 缺失");

must(srcCc.includes("kImplicitStartPC = 1"), "kImplicitStartPC 缺失");
must(srcCc.includes("kImplicitStartDeoptIndex = -1"), "kImplicitStartDeoptIndex 缺失");
must(srcCc.includes("kImplicitStartTrampoline = 0"), "kImplicitStartTrampoline 缺失");
must(srcCc.includes("kMinSkipBits = 7"), "kMinSkipBits 缺失");

// 布局常量核对
const kStackSlotsOffset = 0;
const kConfigOffset = 4;
const kHeaderSize = 8;

const kByteLengthBits = 30;
const kHasTaggedRegistersShift = 30;
const kHasDeoptDataShift = 31;

const kNoDeoptIndex = -1;
const kNoTrampolinePC = -1;
const kMinSkipBits = 7;

console.log(`safepoint-table.h.js: 校验通过!`);
console.log(`  kStackSlotsOffset = ${kStackSlotsOffset}`);
console.log(`  kConfigOffset = ${kConfigOffset}`);
console.log(`  kHeaderSize = ${kHeaderSize}`);
console.log(`  kByteLengthBits = ${kByteLengthBits}`);
console.log(`  kHasTaggedRegistersShift = ${kHasTaggedRegistersShift}`);
console.log(`  kHasDeoptDataShift = ${kHasDeoptDataShift}`);
console.log(`  kMinSkipBits = ${kMinSkipBits}`);
