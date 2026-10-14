#!/usr/bin/env bun
// tool/src/compiler/backend/register-allocation.js
// v8/src/compiler/backend/register-allocation.h → v8-compiler-backend/src/register_allocation.rs.skel
// 寄存器分配公共工具面: RegisterKind 枚举 + 配置查询 + 栈槽字宽表 (真解析 switch)
import { readV8, writeSrc, banner } from "../../../lib.js";

const h = readV8("src/compiler/backend/register-allocation.h");

// ── 对数断言 ────────────────────────────────────────────────────────────────
let checks = 0;
const must = (c, msg) => { checks++; if (!c) throw new Error(`对数失配: ${msg}`); };
const count = (s, re) => (s.match(re) || []).length;

must(h.includes("enum class RegisterKind { kGeneral, kDouble, kSimd128, kSimd256 };"),
  "h RegisterKind 4 值");
must(count(h, /^inline [\w:* ]+\w+\(/gm) === 5, "h inline 函数 5");
for (const fn of ["RegisterKindFor", "GetRegisterCount", "GetAllocatableRegisterCount",
  "GetAllocatableRegisterCodes", "ByteWidthForStackSlot"])
  must(h.includes(fn), `h 函数 ${fn}`);
must(count(h, /UNREACHABLE\(\);/g) === 8, "h UNREACHABLE ×8 (3 组各 2 + 字宽表 2)");

// ── 真解析: ByteWidthForStackSlot 的 rep → 字宽映射 ─────────────────────────
const fnSrc = h.match(/inline int ByteWidthForStackSlot\(MachineRepresentation rep\) \{[\s\S]*?\n\}/)[0];
const cases = [];
{
  // 逐 case 归组: 连续 case 共享下一 return
  let pending = [];
  for (const line of fnSrc.split("\n")) {
    const cm = line.match(/case MachineRepresentation::k(\w+):/);
    if (cm) { pending.push(cm[1]); continue; }
    if (/UNREACHABLE\(\);/.test(line)) { pending = []; continue; }  // 不可达组不产映射
    const rm = line.match(/return (k\w+);/);
    if (rm) { const sizeName = rm[1]; for (const rep of pending) cases.push([rep, sizeName]); pending = []; }
  }
  if (pending.length) throw new Error("ByteWidthForStackSlot 存在无 return 的 case 组");
}
if (cases.length !== 17) throw new Error(`字宽表 ${cases.length} != 17`);
const sizeGroups = {};
for (const [rep, size] of cases) (sizeGroups[size] ??= []).push(rep);
must(sizeGroups.kSystemPointerSize.length === 13, `指针宽组 13 (bit/w8/w16/w32/f16/f32/sandboxed 7 + tagged 族 6), 实得 ${sizeGroups.kSystemPointerSize.length}`);
must(sizeGroups.kDoubleSize.length === 2, "双宽组 2 (Word64/Float64)");
must(sizeGroups.kSimd128Size.length === 1, "Simd128 组 1");
must(sizeGroups.kSimd256Size.length === 1, "Simd256 组 1");
// UNREACHABLE 组 (kNone/kMapWord/kIndirectPointer/kFloat16RawBits) 不产映射
must(count(fnSrc, /case MachineRepresentation::/g) === 21, "case 总数 21 (17 映射 + 4 不可达)");

// ── 产物 ────────────────────────────────────────────────────────────────────
let table = "";
for (const [size, reps] of Object.entries(sizeGroups)) {
  table += `/// v8: ByteWidthForStackSlot → ${size}\n`;
  for (const rep of reps) table += `pub const SLOT_WIDTH_${rep.toUpperCase()}: usize = ${size};\n`;
  table += "\n";
}
const skel = `${banner("src/compiler/backend/register-allocation.js",
  "src/compiler/backend/register-allocation.h",
  "寄存器分配公共工具: RegisterKind/配置查询/栈槽字宽表")}
//! 台账: RegisterKind 4 值 (kGeneral/kDouble/kSimd128/kSimd256);
//!   RegisterKindFor = Simd256→Simd128→浮点→通用判定链;
//!   GetRegisterCount/GetAllocatableRegisterCount/GetAllocatableRegisterCodes
//!   = RegisterConfiguration 按 kind 分派 (Simd256 UNREACHABLE);
//!   ByteWidthForStackSlot 字宽表 (实测 ${cases.length} rep 有映射, 4 rep UNREACHABLE)。

/// v8: RegisterKind — 寄存器类别
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum RegisterKind {
  /// kGeneral
  General = 0,
  /// kDouble
  Double = 1,
  /// kSimd128
  Simd128 = 2,
  /// kSimd256
  Simd256 = 3,
}

/// v8: RegisterKindFor — 表示法 → 寄存器类别判定链
// pub fn register_kind_for(rep: MachineRepresentation) -> RegisterKind
/// v8: GetRegisterCount — 配置查询 (Simd256 不可达)
// pub fn get_register_count(config: &RegisterConfiguration, kind: RegisterKind) -> i32
/// v8: GetAllocatableRegisterCount — 可分配数查询
// pub fn get_allocatable_register_count(config: &RegisterConfiguration, kind: RegisterKind) -> i32
/// v8: GetAllocatableRegisterCodes — 可分配码表查询
// pub fn get_allocatable_register_codes(config: &RegisterConfiguration, kind: RegisterKind) -> &[i32]
/// v8: ByteWidthForStackSlot — 栈槽字宽 (kNone/kMapWord/kIndirectPointer/kFloat16RawBits 不可达)
// pub fn byte_width_for_stack_slot(rep: MachineRepresentation) -> usize

${table}
// —— 提取不变式断言 (字宽表实测: 指针宽 12 / 双宽 2 / Simd128 1 / Simd256 1) ——
const _: () = {
  assert!(SLOT_WIDTH_BIT == SLOT_WIDTH_WORD32); // 同组同宽实测
  assert!(SLOT_WIDTH_WORD64 == SLOT_WIDTH_FLOAT64);
  assert!(SLOT_WIDTH_TAGGED == SLOT_WIDTH_POINTER);
};
`;

writeSrc("v8-compiler-backend", "register_allocation.rs.skel", skel);
console.log(`register-allocation.js: 对数 ${checks} 条全过; RegisterKind 4; inline 函数 5; ` +
  `字宽表 17 rep (指针宽 13/双宽 2/Simd128 1/Simd256 1, 不可达 4); 产物 register_allocation.rs.skel`);
