#!/usr/bin/env bun
// tool/src/wasm/jump-table-assembler.cc.js
// 机械提取 v8/src/wasm/jump-table-assembler.cc 各架构跳转表槽位指令模板与目标地址布局:
// x64/ia32 惰性编译槽、x64 跳转槽 (含 CET_IBT)、x64 远跳槽与 PatchFarJumpSlot 偏移、
// 各架构惰性编译函数索引寄存器 static_assert。
// 头侧槽位尺寸常量已在 v8-wasm-core/src/jump_table_assembler_consts.rs (jump-table-assembler.h.js)。

import { readV8, writeSrc, banner } from "../../lib.js";

const src = readV8("src/wasm/jump-table-assembler.cc");

// ---------- 1. 惰性编译函数索引寄存器 (每架构一处 static_assert) ----------
const regAsserts = [...src.matchAll(
  /static_assert\(kWasmCompileLazyFuncIndexRegister == (\w+)\)/g,
)].map((m) => m[1]);
if (regAsserts.join(",") !== "edi,r4,x8,r7,t0,t0,r15") {
  throw new Error(`惰性编译寄存器序异常: ${regAsserts.join(",")}`);
}
// 语义锚 (按源中架构段序): ia32=edi, arm=r4, arm64=x8, s390x=r7, mips64=t0, loong64=t0, ppc64=r15
// riscv64/riscv32 段用内联汇编, 无此断言; x64 段经 pushq 传参无需寄存器
const regArch = ["ia32", "arm", "arm64", "s390x", "mips64", "loong64", "ppc64"];

// ---------- 2. x64 惰性编译槽模板 (pushq func_index; near_jmp) ----------
if (!/const uint8_t inst\[kLazyCompileTableSlotSize\] = \{\s*0x68, 0, 0, 0, 0,\s*\/\/ pushq func_index\s*0xe9, 0, 0, 0, 0,\s*\/\/ near_jmp displacement\s*\};/.test(src)) {
  throw new Error("x64 惰性编译槽模板缺失");
}

// ---------- 3. x64 跳转槽模板 (near_jmp + int3*3, 8 字节原子写) ----------
if (!/uint8_t inst\[8\] = \{\s*0xe9, 0,\s*0,\s*0, 0,\s*\/\/ near_jmp displacement\s*0xcc, 0xcc, 0xcc,\s*\/\/ int3 \* 3\s*\};/.test(src)) {
  throw new Error("x64 跳转槽模板缺失");
}

// ---------- 4. x64 CET_IBT 前缀 (endbr64 + 8 字节对齐 nop) ----------
const cet = src.match(/uint32_t endbr_insn = (0x[0-9a-f]+);/);
if (!cet || cet[1] !== "0xfa1e0ff3") throw new Error("CET endbr 指令字缺失");
const cetNop = src.match(/uint32_t nop = (0x[0-9a-f]+);/);
if (!cetNop || cetNop[1] !== "0x00401f0f") throw new Error("CET 对齐 nop 缺失");

// ---------- 5. x64 远跳槽模板 (jmp [rip+2]; nop2; target 8B) ----------
if (!src.includes("0xff, 0x25, 0x02, 0, 0, 0,        // jmp [rip+0x2]")) {
  throw new Error("x64 远跳槽 jmp [rip+2] 模板缺失");
}

// ---------- 6. x64 PatchFarJumpSlot: 目标地址位于槽内 +8 偏移 ----------
if (!src.includes("jit_allocation.WriteValue(slot + kSystemPointerSize, target, kRelaxedStore);")) {
  throw new Error("x64 PatchFarJumpSlot +8 偏移缺失");
}
if (!src.includes("DCHECK(IsAligned(slot, kSystemPointerSize));")) {
  throw new Error("x64 PatchFarJumpSlot 指针对齐检查缺失");
}

// ---------- 7. ia32 惰性编译槽模板 (mov edi, func_index; near_jmp) ----------
if (!/0xbf, 0, 0, 0, 0,\s*\/\/ mov edi, func_index/.test(src)) {
  throw new Error("ia32 惰性编译槽模板缺失");
}

// ---------- 8. 远跳槽末尾存目标地址的架构 (riscv/s390x/ppc 家族: slot + size - ptr) ----------
const tailAddr = [...src.matchAll(/Address target_addr =\s*slot \+ kFarJumpTableSlotSize - ([\w() *+-]+);/g)].map((m) => m[1].replace(/\s+/g, ""));
if (tailAddr.join("|") !== "kSystemPointerSize|kSystemPointerSize|(2*kInstrSize)-kSystemPointerSize") {
  throw new Error(`远跳槽目标地址布局异常: ${tailAddr.join("|")}`);
}

// ---------- 9. 槽位尺寸恒等断言 (EmitFarJumpSlot 复用 EmitJumpSlot 的架构) ----------
const sizeEqs = [...src.matchAll(/static_assert\(kJumpTableSlotSize == kFarJumpTableSlotSize\)/g)];
if (sizeEqs.length !== 5) throw new Error(`槽位尺寸恒等断言 ${sizeEqs.length} != 5`);

console.log(`jump-table-assembler: 寄存器=${regAsserts.length} 尾部地址布局=${tailAddr.length} 尺寸恒等=${sizeEqs.length}`);

// ---------- 生成 Rust ----------
let rust = `${banner(
  "src/wasm/jump-table-assembler.cc.js",
  "src/wasm/jump-table-assembler.cc",
  "各架构跳转表槽位指令模板字节与目标地址布局 (尺寸常量在 jump_table_assembler_consts.rs)",
)}
//! 跳转表槽位的机器码模板: 生成期由汇编器逐架构填充。

/// x64 惰性编译槽指令前导: 0x68 = pushq imm32 (func_index)
/// v8: src/wasm/jump-table-assembler.cc EmitLazyCompileJumpSlot (x64)
pub const X64_LAZY_PUSH: u8 = 0x68;
/// 惰性编译槽跳转前导: 0xe9 = near_jmp rel32 (x64 与 ia32 共用)
/// v8: src/wasm/jump-table-assembler.cc EmitLazyCompileJumpSlot
pub const LAZY_NEAR_JMP: u8 = 0xe9;
/// ia32 惰性编译槽: 0xbf = mov edi, imm32 (func_index)
/// v8: src/wasm/jump-table-assembler.cc EmitLazyCompileJumpSlot (ia32)
pub const IA32_LAZY_MOV_EDI: u8 = 0xbf;

/// x64 跳转槽模板: near_jmp rel32 + 3 个 int3 填充, 共 8 字节一次原子写
/// v8: src/wasm/jump-table-assembler.cc EmitJumpSlot (x64)
pub const X64_JUMP_SLOT_TEMPLATE: [u8; 8] = [0xe9, 0, 0, 0, 0, 0xcc, 0xcc, 0xcc];
/// int3 陷阱填充字节
pub const INT3: u8 = 0xcc;

/// CET_IBT 使能时跳转槽前置 endbr64 指令字 (0xfa1e0ff3 小端)
/// v8: src/wasm/jump-table-assembler.cc EmitJumpSlot (x64, V8_ENABLE_CET_IBT)
pub const CET_ENDBR64: u32 = ${cet[1]};
/// CET_IBT 前置块的对齐 nop 指令字 (补齐 8 字节对齐)
pub const CET_ALIGN_NOP: u32 = ${cetNop[1]};

/// x64 远跳槽模板: jmp [rip+2] + 2 字节 nop + 8 字节目标地址
/// v8: src/wasm/jump-table-assembler.cc EmitFarJumpSlot (x64)
pub const X64_FAR_JUMP_SLOT_HEAD: [u8; 8] = [0xff, 0x25, 0x02, 0, 0, 0, 0x66, 0x90];
/// x64 PatchFarJumpSlot: 目标地址位于槽内指针大小 (8) 偏移处, 原子写
/// v8: src/wasm/jump-table-assembler.cc PatchFarJumpSlot (x64)
pub const X64_FAR_TARGET_OFFSET: usize = 8;

/// 惰性编译函数索引寄存器 (按架构)
/// v8: src/wasm/jump-table-assembler.cc static_assert(kWasmCompileLazyFuncIndexRegister == ...)
#[rustfmt::skip]
pub static LAZY_FUNC_INDEX_REGISTERS: &[(&str, &str)] = &[
`;

for (let i = 0; i < regAsserts.length; i++) {
  rust += `    ("${regArch[i]}", "${regAsserts[i]}"),\n`;
}

rust += `];

/// 远跳槽目标地址尾部布局: arm64/riscv64 直接尾随指针; s390x 额外预留 2 条指令后放指针
/// v8: src/wasm/jump-table-assembler.cc EmitFarJumpSlot (arm64/riscv64/s390x)
pub const FAR_TAIL_PTR_ONLY: &str = "arm64/riscv64: slot + kFarJumpTableSlotSize - kSystemPointerSize";
pub const FAR_TAIL_AFTER_INSTRS: &str = "s390x: slot + kFarJumpTableSlotSize - 2*kInstrSize - kSystemPointerSize";

// —— 提取不变式断言 ——
const _: () = {
    assert_eq!(X64_JUMP_SLOT_TEMPLATE[0], 0xe9);
    assert_eq!(X64_JUMP_SLOT_TEMPLATE[5], 0xcc);
    assert_eq!(X64_FAR_JUMP_SLOT_HEAD[0], 0xff);
    assert_eq!(X64_FAR_JUMP_SLOT_HEAD[1], 0x25);
    assert_eq!(X64_FAR_JUMP_SLOT_HEAD[6], 0x66);
    assert_eq!(CET_ENDBR64, 0xfa1e0ff3);
    assert_eq!(CET_ALIGN_NOP, 0x00401f0f);
    assert_eq!(X64_FAR_TARGET_OFFSET, 8);
    assert_eq!(LAZY_FUNC_INDEX_REGISTERS.len(), 7);
    assert_eq!(LAZY_FUNC_INDEX_REGISTERS[2], ("arm64", "x8"));
    assert_eq!(LAZY_FUNC_INDEX_REGISTERS[3], ("s390x", "r7"));
    assert_eq!(LAZY_FUNC_INDEX_REGISTERS[6], ("ppc64", "r15"));
};
`;

const p = writeSrc("v8-wasm-objects", "jump_table_assembler_cc.rs", rust);
console.log(`written ${p}`);
