#!/usr/bin/env bun
// tool/src/wasm/baseline/liftoff-assembler.cc.js
// 机械提取 v8/src/wasm/baseline/liftoff-assembler{.cc,-inl.h} 的
// 溢出槽偏移规则、平台无关 emit 分派规则、merge 布尔枚举与兼容性判定

import { readV8, writeSrc, banner } from "../../../lib.js";

const ccSrc = readV8("src/wasm/baseline/liftoff-assembler.cc");
const inlSrc = readV8("src/wasm/baseline/liftoff-assembler-inl.h");
// 公式来源在 .h (只读对账, 产物仍归本族脚本)
const hSrc = readV8("src/wasm/baseline/liftoff-assembler.h");

function must(src, re, what) {
  const m = src.match(re);
  if (!m) throw new Error(`未找到 ${what}`);
  return m;
}

// —— 1. .cc: 平台相关类别常量 ——
must(ccSrc, /constexpr ValueKind LiftoffAssembler::kIntPtrKind;/, "kIntPtrKind 定义");
must(ccSrc, /constexpr ValueKind LiftoffAssembler::kSmiKind;/, "kSmiKind 定义");
// 64 位平台 (指针压缩开) 下: kIntPtrKind=kI64, kSmiKind=kI32
must(hSrc, /kSystemPointerSize == kInt32Size/, "kIntPtrKind 条件 (.h)");
must(hSrc, /kTaggedSize == kInt32Size/, "kSmiKind 条件 (.h)");

// —— 2. .cc: merge 布尔枚举 (4 组 8 值) ——
const mergeEnums = [
  ["MergeKeepStackSlots", ["K_KEEP_STACK_SLOTS", "K_TURN_STACK_SLOTS_INTO_REGISTERS"], ["kKeepStackSlots = true", "kTurnStackSlotsIntoRegisters = false"]],
  ["MergeAllowConstants", ["K_CONSTANTS_ALLOWED", "K_CONSTANTS_NOT_ALLOWED"], ["kConstantsAllowed = true", "kConstantsNotAllowed = false"]],
  ["MergeAllowRegisters", ["K_REGISTERS_ALLOWED", "K_REGISTERS_NOT_ALLOWED"], ["kRegistersAllowed = true", "kRegistersNotAllowed = false"]],
  ["ReuseRegisters", ["K_REUSE_REGISTERS", "K_NO_REUSE_REGISTERS"], ["kReuseRegisters = true", "kNoReuseRegisters = false"]],
];
for (const [name, , variants] of mergeEnums) {
  must(ccSrc, new RegExp(`enum ${name} : bool \\{\\s*${variants[0]},\\s*${variants[1]}\\s*\\};`), name);
}

// —— 3. .cc: 兼容槽类型判定 (仅 DEBUG) ——
must(ccSrc, /bool CompatibleStackSlotTypes\(ValueKind a, ValueKind b\) \{/, "CompatibleStackSlotTypes");
must(ccSrc, /return a == b \|\| \(is_reference\(a\) && is_reference\(b\)\);/, "兼容规则体");

// —— 4. .cc: 关键流程锚 ——
for (const a of [
  "void InitMergeRegion(LiftoffAssembler::CacheState* target_state,",
  "LiftoffAssembler::CacheState LiftoffAssembler::MergeIntoNewState(",
  "void LiftoffAssembler::CacheState::Steal(CacheState& source) {",
  "void LiftoffAssembler::CacheState::Split(const CacheState& source) {",
  "void LiftoffAssembler::CacheState::DefineSafepoint(",
  "void LiftoffAssembler::MergeStackWith(CacheState& target, uint32_t arity,",
  "void LiftoffAssembler::MergeFullStackWith(CacheState& target) {",
  "void LiftoffAssembler::PrepareCall(const ValueKindSig* sig,",
  "void LiftoffAssembler::FinishCall(const ValueKindSig* sig,",
  "void LiftoffAssembler::MoveToReturnLocations(",
  "void LiftoffAssembler::MoveToReturnLocationsMultiReturn(",
  "bool LiftoffAssembler::ValidateCacheState() const {",
  "void LiftoffAssembler::SpillRegister(LiftoffRegister reg) {",
  "void LiftoffAssembler::set_num_locals(uint32_t num_locals) {",
]) {
  if (!ccSrc.includes(a)) throw new Error(`流程锚缺失: ${a}`);
}

// —— 5. .cc: CacheState 方法/定义清单 (col-0 签名, 跨行签名两种解析等量复核) ——
const ccFns = [...ccSrc.matchAll(/^[A-Za-z_][^;=\n]*?\b(\w*)::([A-Za-z_~]\w*)\s*\(/gm)].map((m) => `${m[1]}::${m[2]}`);
const ccFnsSpan = [...ccSrc.matchAll(/^[A-Za-z_][^;=]*?\b(\w*)::([A-Za-z_~]\w*)\s*\(/gm)].map((m) => `${m[1]}::${m[2]}`);
if (ccFns.length !== ccFnsSpan.length) throw new Error(`跨行签名不一致: ${ccFns.length}/${ccFnsSpan.length}`);
if (ccFns.length !== 34) throw new Error(`liftoff-assembler.cc 定义数不符: ${ccFns.length}, 期望 34`);

// —— 6. -inl.h: 溢出槽偏移规则 ——
must(inlSrc, /int LiftoffAssembler::NextSpillOffset\(ValueKind kind, int top_spill_offset\) \{/, "NextSpillOffset");
must(inlSrc, /int offset = top_spill_offset \+ SlotSizeForType\(kind\);/, "偏移累加");
must(inlSrc, /offset = RoundUp\(offset, SlotSizeForType\(kind\)\);/, "对齐规则");
must(inlSrc, /int LiftoffAssembler::TopSpillOffset\(\) const \{/, "TopSpillOffset");
must(inlSrc, /\? StaticStackFrameSize\(\)\s*: cache_state_\.stack_state\.back\(\)\.offset\(\);/, "空栈回退规则");

// —— 7. -inl.h: ptrsize emit 分派族 (kSystemPointerSize==8 → i64) ——
const ptrsizeEmits = [...inlSrc.matchAll(/void LiftoffAssembler::(emit_ptrsize_\w+)\(/g)].map((m) => m[1]);
const EXPECTED_PTRSIZE = [
  "emit_ptrsize_add", "emit_ptrsize_sub", "emit_ptrsize_and", "emit_ptrsize_shri",
  "emit_ptrsize_addi", "emit_ptrsize_muli", "emit_ptrsize_set_cond",
  "emit_ptrsize_cond_jumpi", // 32 位平台专属 (V8_TARGET_ARCH_32_BIT 段)
];
if (ptrsizeEmits.join(",") !== EXPECTED_PTRSIZE.join(",")) {
  throw new Error(`ptrsize 族不符: ${ptrsizeEmits}`);
}
for (const e of EXPECTED_PTRSIZE.slice(0, 7)) {
  must(inlSrc, new RegExp(`void LiftoffAssembler::${e}\\([\\s\\S]*?if constexpr \\(kSystemPointerSize == 8\\) \\{[\\s\\S]*?emit_i64_`), `${e} i64 分派`);
}
// cond_jumpi 为 32 位平台专属转发 (无 i64 分派体)
must(inlSrc, /#ifdef V8_TARGET_ARCH_32_BIT[\s\S]*?emit_ptrsize_cond_jumpi[\s\S]*?emit_i32_cond_jumpi/, "cond_jumpi 32 位专属");

// —— 8. -inl.h: bailout 首因保留规则 ——
must(inlSrc, /void LiftoffAssembler::bailout\(LiftoffBailoutReason reason,\s*const char\* detail\) \{/, "bailout 签名");
must(inlSrc, /if \(bailout_reason_ != kNoReason\) return;/, "首因保留");

// —— 9. -inl.h: 栈推入族锚 ——
for (const a of [
  "void LiftoffAssembler::PushRegister(ValueKind kind, LiftoffRegister reg) {",
  "void LiftoffAssembler::PushException() {",
  "void LiftoffAssembler::PushConstant(ValueKind kind, int32_t i32_const) {",
  "void LiftoffAssembler::PushStack(ValueKind kind) {",
  "void LiftoffAssembler::PopToFixedRegister(LiftoffRegister reg) {",
]) {
  if (!inlSrc.includes(a)) throw new Error(`推入族锚缺失: ${a}`);
}

console.log(
  `liftoff-assembler{.cc,-inl.h}: 定义=${ccFns.length} merge 枚举=${mergeEnums.length} ptrsize=${ptrsizeEmits.length}`,
);

// —— 10. 生成 Rust ——
let rust = banner(
  "src/wasm/baseline/liftoff-assembler.cc.js",
  "src/wasm/baseline/liftoff-assembler{.cc,-inl.h}",
  "Liftoff 汇编器平台无关层: 溢出槽规则 / merge 枚举 / ptrsize 分派",
);

rust += `
//! 平台无关规则; 平台发射语义由 Developer 就地修订。

/// 溢出槽下一偏移 (原版 NextSpillOffset 规则):
/// offset = top + slot_size; 需对齐的类别 (s128 / 引用) 再向上取整到自身槽宽。
/// v8: src/wasm/baseline/liftoff-assembler-inl.h NextSpillOffset
#[must_use]
pub const fn next_spill_offset(top_spill_offset: i32, slot_size: i32, needs_alignment: bool) -> i32 {
  let offset = top_spill_offset + slot_size;
  if (needs_alignment) {
    // RoundUp(offset, slot_size)
    offset.div_ceil(slot_size) * slot_size
  } else {
    offset
  }
}

/// 平台相关值类别 (64 位平台取值, 指针压缩开启时 kTaggedSize=4)
/// v8: src/wasm/baseline/liftoff-assembler.cc kIntPtrKind / kSmiKind
pub const K_INT_PTR_IS_I64: bool = true;  // kSystemPointerSize(8) != kInt32Size(4)
pub const K_SMI_IS_I64: bool = false;    // kTaggedSize(4) == kInt32Size(4)

/// merge 策略布尔枚举族 (原版 enum X : bool)
/// v8: src/wasm/baseline/liftoff-assembler.cc InitMergeRegion 参数面
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum MergeKeepStackSlots {
  /// kKeepStackSlots
  K_KEEP_STACK_SLOTS = 1,
  /// kTurnStackSlotsIntoRegisters
  K_TURN_STACK_SLOTS_INTO_REGISTERS = 0,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum MergeAllowConstants {
  /// kConstantsAllowed
  K_CONSTANTS_ALLOWED = 1,
  /// kConstantsNotAllowed
  K_CONSTANTS_NOT_ALLOWED = 0,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum MergeAllowRegisters {
  /// kRegistersAllowed
  K_REGISTERS_ALLOWED = 1,
  /// kRegistersNotAllowed
  K_REGISTERS_NOT_ALLOWED = 0,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum ReuseRegisters {
  /// kReuseRegisters
  K_REUSE_REGISTERS = 1,
  /// kNoReuseRegisters
  K_NO_REUSE_REGISTERS = 0,
}

/// 槽类型兼容判定 (原版 CompatibleStackSlotTypes, DEBUG 校验):
/// 同类兼容; 引用类别内部 (ref / ref null) 互覆兼容。
/// v8: src/wasm/baseline/liftoff-assembler.cc CompatibleStackSlotTypes
#[must_use]
pub const fn compatible_stack_slot_types(same: bool, both_reference: bool) -> bool {
  same || both_reference
}

/// ptrsize 发射分派 (64 位平台: 全部落 i64 发射器)
/// v8: src/wasm/baseline/liftoff-assembler-inl.h emit_ptrsize_* 族
pub const K_PTRSIZE_EMITS: &[&str] = &[
`;

for (const e of ptrsizeEmits) rust += `  "${e}",\n`;
rust += `];

/// .cc 定义清单 (col-0 签名, Class::member)
/// v8: src/wasm/baseline/liftoff-assembler.cc
pub const CC_FN_DEFS: &[&str] = &[
`;
for (const f of ccFns) rust += `  "${f}",\n`;
rust += `];

// —— 提取不变式断言 ——
const _: () = {
  assert!(K_PTRSIZE_EMITS.len() == ${ptrsizeEmits.length});
  assert!(next_spill_offset(0, 16, true) == 16);
  assert!(next_spill_offset(8, 16, true) == 16);   // 8 → 对齐到 16
  assert!(next_spill_offset(4, 4, false) == 8);    // i32 不对齐直加
  assert!(next_spill_offset(9, 4, true) == 12);
  assert!(MergeKeepStackSlots::K_KEEP_STACK_SLOTS as u8 == 1);
  assert!(compatible_stack_slot_types(false, true));
  assert!(!compatible_stack_slot_types(false, false));
  assert!(K_INT_PTR_IS_I64 && !K_SMI_IS_I64);
};
`;

const p = writeSrc("v8-wasm-liftoff", "liftoff_assembler_impl.rs.skel", rust);
console.log(`written ${p}`);
