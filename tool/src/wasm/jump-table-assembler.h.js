#!/usr/bin/env bun
// tool/src/wasm/jump-table-assembler.h.js
// 机械提取 v8/src/wasm/jump-table-assembler.h 的跳转表槽位布局常量
// 平台展开: x64 (含 CET_IBT 两分支) + arm64 (含 CFI 两分支); 其余架构登记跳过

import { readV8, writeSrc, banner } from "../../lib.js";

const hSrc = readV8("src/wasm/jump-table-assembler.h");
const K_INSTR_SIZE_ARM64 = 4;

function segBetween(startMarker, endMarker) {
  const b = hSrc.indexOf(startMarker);
  if (b < 0) throw new Error(`未找到 ${startMarker}`);
  const e = hSrc.indexOf(endMarker, b + startMarker.length);
  if (e < 0) throw new Error(`未找到 ${endMarker}`);
  return hSrc.slice(b, e);
}

// —— x64: CET_IBT 开/关两分支 (数值常量), 行宽在 #endif 之后符号引用槽宽
const x64seg = segBetween("#if V8_TARGET_ARCH_X64", "#elif V8_TARGET_ARCH_IA32");
function x64vals(withCet) {
  const branch = withCet
    ? x64seg.slice(x64seg.indexOf("#ifdef V8_ENABLE_CET_IBT"), x64seg.indexOf("#else  // V8_ENABLE_CET_IBT"))
    : x64seg.slice(x64seg.indexOf("#else  // V8_ENABLE_CET_IBT"), x64seg.indexOf("#endif"));
  const num = (name) => {
    const m = branch.match(new RegExp(`static constexpr int ${name} = (\\d+);`));
    if (!m) throw new Error(`x64 未找到 ${name} (cet=${withCet})`);
    return parseInt(m[1], 10);
  };
  const slotSize = num("kJumpTableSlotSize");
  const markerSize = num("kJumpTableSlotEntryMarkerSize");
  const sym = (name) => {
    const m = x64seg.match(new RegExp(`static constexpr int ${name} = ([^;]+);`));
    if (!m) throw new Error(`x64 未找到 ${name}`);
    const e = m[1].trim().replace(/kJumpTableSlotSize/g, String(slotSize)).replace(/\s/g, "");
    if (!/^\d+$/.test(e)) throw new Error(`x64 ${name} 无法折叠: ${e}`);
    return parseInt(e, 10);
  };
  return {
    slotSize,
    markerSize,
    lineSize: sym("kJumpTableLineSize"),
    farSlotSize: sym("kFarJumpTableSlotSize"),
    lazySlotSize: sym("kLazyCompileTableSlotSize"),
  };
}
const x64cetOn = x64vals(true);
const x64cetOff = x64vals(false);

// —— arm64: CFI 开/关两分支 (kInstrSize 符号常量)
const arm64seg = segBetween("#elif V8_TARGET_ARCH_ARM64", "#elif V8_TARGET_ARCH_S390X");
const arm64CfiOn = arm64seg.match(/#if V8_ENABLE_CONTROL_FLOW_INTEGRITY([\s\S]*?)#else/)[1];
const arm64CfiOff = arm64seg.match(/#else([\s\S]*?)#endif/)[1];
function arm64vals(src, withCfi) {
  const val = (name, searchSrc) => {
    const m = (searchSrc ?? src).match(new RegExp(`static constexpr int ${name} = ([^;]+);`));
    if (!m) throw new Error(`arm64 未找到 ${name} (cfi=${withCfi})`);
    const e = m[1].trim().replace(/kInstrSize/g, String(K_INSTR_SIZE_ARM64)).replace(/\s/g, "");
    if (!/^\d+([*+]\d+)*$/.test(e)) throw new Error(`arm64 ${name} 无法折叠: ${e}`);
    const folded = e.includes("*") ? e.split("*").reduce((a, b) => Number(a) * Number(b)) : Number(e);
    if (!Number.isInteger(folded)) throw new Error(`arm64 ${name} 折叠异常: ${e}`);
    return folded;
  };
  return {
    lineSize: val("kJumpTableLineSize"),
    slotSize: val("kJumpTableSlotSize"),
    // far/lazy 槽宽在 CFI 分支外共享
    farSlotSize: val("kFarJumpTableSlotSize", arm64seg),
    lazySlotSize: val("kLazyCompileTableSlotSize", arm64seg),
  };
}
const arm64on = arm64vals(arm64CfiOn, true);
const arm64off = arm64vals(arm64CfiOff, false);

// 交叉校验 (原版侧人工对数值)
if (x64cetOn.slotSize !== 16 || x64cetOn.markerSize !== 8) throw new Error("x64 CET 分支值不符");
if (x64cetOff.slotSize !== 8 || x64cetOff.markerSize !== 0) throw new Error("x64 非 CET 分支值不符");
if (x64cetOn.lineSize !== 16 || x64cetOff.lineSize !== 8) throw new Error("x64 行宽不符");
if (x64cetOn.farSlotSize !== 16 || x64cetOn.lazySlotSize !== 10) throw new Error("x64 far/lazy 槽宽不符");
if (arm64on.lineSize !== 8 || arm64on.slotSize !== 8) throw new Error("arm64 CFI 分支值不符");
if (arm64off.lineSize !== 4 || arm64off.slotSize !== 4) throw new Error("arm64 非 CFI 分支值不符");
if (arm64on.farSlotSize !== 16 || arm64on.lazySlotSize !== 16) throw new Error("arm64 far/lazy 槽宽不符");

console.log(`jump-table: x64(cet)=${x64cetOn.slotSize}/${x64cetOff.slotSize} arm64(cfi)=${arm64on.slotSize}/${arm64off.slotSize}`);

function emit(name, v, note) {
  const marker = v.markerSize !== undefined
    ? `  /// 原版 kJumpTableSlotEntryMarkerSize\n  pub const JUMP_TABLE_SLOT_ENTRY_MARKER_SIZE: i32 = ${v.markerSize};\n`
    : "";
  return `/// ${name}${note}\npub mod ${name} {\n  /// 原版 kJumpTableLineSize\n  pub const JUMP_TABLE_LINE_SIZE: i32 = ${v.lineSize};\n  /// 原版 kJumpTableSlotSize\n  pub const JUMP_TABLE_SLOT_SIZE: i32 = ${v.slotSize};\n${marker}  /// 原版 kFarJumpTableSlotSize\n  pub const FAR_JUMP_TABLE_SLOT_SIZE: i32 = ${v.farSlotSize};\n  /// 原版 kLazyCompileTableSlotSize\n  pub const LAZY_COMPILE_TABLE_SLOT_SIZE: i32 = ${v.lazySlotSize};\n}\n\n`;
}

let rustBody = "";
rustBody += emit("x64_cet_ibt", { ...x64cetOn }, " (V8_TARGET_ARCH_X64 + V8_ENABLE_CET_IBT)");
rustBody += emit("x64", { ...x64cetOff }, " (V8_TARGET_ARCH_X64, 无 CET_IBT)");
rustBody += emit("arm64_cfi", { ...arm64on }, " (V8_TARGET_ARCH_ARM64 + V8_ENABLE_CONTROL_FLOW_INTEGRITY; kInstrSize=4)");
rustBody += emit("arm64", { ...arm64off }, " (V8_TARGET_ARCH_ARM64, 无 CFI)");

const rust =
  banner(
    "src/wasm/jump-table-assembler.h.js",
    "src/wasm/jump-table-assembler.h",
    "跳转表槽位布局常量 (x64+arm64 分支展开; 其余架构跳过: ia32/arm/s390x/ppc/mips64/riscv)",
  ) +
  "\n" +
  rustBody +
  `// —— 提取不变式断言 ——
const _: () = {
  assert!(x64_cet_ibt::JUMP_TABLE_SLOT_SIZE == 16);
  assert!(x64::JUMP_TABLE_SLOT_SIZE == 8);
  assert!(x64::JUMP_TABLE_LINE_SIZE == 8);
  assert!(arm64_cfi::JUMP_TABLE_SLOT_SIZE == 8);
  assert!(arm64::JUMP_TABLE_SLOT_SIZE == 4);
  assert!(arm64::LAZY_COMPILE_TABLE_SLOT_SIZE == 16);
};
`;

const p = writeSrc("v8-wasm-core", "jump_table_assembler_consts.rs", rust);
console.log(`written ${p}`);
