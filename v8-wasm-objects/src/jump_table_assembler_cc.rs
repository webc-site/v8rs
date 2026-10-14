// 由 tool/src/wasm/jump-table-assembler.cc.js 从 v8/src/wasm/jump-table-assembler.cc 生成; 修订就地改, 复跑覆盖
// 各架构跳转表槽位指令模板字节与目标地址布局 (尺寸常量在 jump_table_assembler_consts.rs)

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
pub const CET_ENDBR64: u32 = 0xfa1e0ff3;
/// CET_IBT 前置块的对齐 nop 指令字 (补齐 8 字节对齐)
pub const CET_ALIGN_NOP: u32 = 0x00401f0f;

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
    ("ia32", "edi"),
    ("arm", "r4"),
    ("arm64", "x8"),
    ("s390x", "r7"),
    ("mips64", "t0"),
    ("loong64", "t0"),
    ("ppc64", "r15"),
];

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
