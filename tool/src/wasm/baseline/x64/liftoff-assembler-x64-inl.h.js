#!/usr/bin/env bun
// tool/src/wasm/baseline/x64/liftoff-assembler-x64-inl.h.js
// 机械提取 v8/src/wasm/baseline/x64/liftoff-assembler-x64-inl.h 的 x64 liftoff 约定

import { readV8, writeSrc, banner } from "../../../../lib.js";

const src = readV8("src/wasm/baseline/x64/liftoff-assembler-x64-inl.h");
const frameSrc = readV8("src/execution/x64/frame-constants-x64.h");

// 1. 第二 scratch 寄存器约定
if (!src.includes("constexpr Register kScratchRegister2 = r11;")) {
  throw new Error("未找到 kScratchRegister2=r11");
}
if (!src.includes("constexpr DoubleRegister kScratchDoubleReg2 = xmm14;")) {
  throw new Error("未找到 kScratchDoubleReg2=xmm14");
}

// 2. scratch 不得与缓存寄存器重叠 (双 static_assert)
const overlapAsserts = (src.match(/scratch registers must not be used as cache registers/g) || []).length;
if (overlapAsserts !== 2) throw new Error(`scratch 排除断言 ${overlapAsserts} != 2`);

// 3. 栈槽寻址: rbp 负偏移
if (!src.includes("return Operand(rbp, -offset);")) throw new Error("未找到 rbp 负偏移寻址");

// 4. 实例数据/OSR 槽位操作数
if (!src.includes("constexpr Operand kInstanceDataOperand =")) {
  throw new Error("未找到 kInstanceDataOperand");
}
if (!src.includes("constexpr Operand kOSRTargetSlot = GetStackSlot(kOSRTargetOffset);")) {
  throw new Error("未找到 kOSRTargetSlot");
}

// 5. 帧常量 (来自 frame-constants-x64.h)
const fb = frameSrc.match(/kFeedbackVectorOffset = (\d+) \* kSystemPointerSize;/);
const ib = frameSrc.match(/kInstanceDataOffset = (\d+) \* kSystemPointerSize;/);
if (!fb || !ib) throw new Error("未找到 x64 WasmLiftoffFrameConstants");
const feedbackOffset = parseInt(fb[1], 10) * 8;
const instanceOffset = parseInt(ib[1], 10) * 8;

// 6. 内存操作数: uint31 门槛
if (!src.includes("if (is_uint31(offset_imm)) {")) throw new Error("未找到 uint31 门槛");

// 7. CPU feature 宏
if (!src.includes("#define RETURN_FALSE_IF_MISSING_CPU_FEATURE(name)")) {
  throw new Error("未找到 RETURN_FALSE_IF_MISSING_CPU_FEATURE");
}

// 8. 方法实现规模
const methods = (src.match(/LiftoffAssembler::/g) || []).length;
if (methods < 400) throw new Error(`LiftoffAssembler 方法 ${methods} < 400`);

// 9. 缩放因子约定
if (!src.includes("ScaleFactor scale_factor = times_1")) throw new Error("未找到默认缩放 times_1");

console.log(
  `liftoff-assembler-x64-inl: scratch2=r11/xmm14 帧偏移(instance)=${instanceOffset} 方法=${methods}`,
);

// 10. 生成 Rust
let rust = banner(
  "src/wasm/baseline/x64/liftoff-assembler-x64-inl.h.js",
  "src/wasm/baseline/x64/liftoff-assembler-x64-inl.h",
  "x64 Liftoff 汇编层约定 (scratch/栈槽/内存操作数)",
);

rust += `
//! x64 平台 Liftoff 桩: 寄存器与帧布局约定。
/// v8: src/wasm/baseline/x64/liftoff-assembler-x64-inl.h namespace liftoff

/// 第二 scratch 寄存器 (第一为 kScratchRegister, x64 上是 r10)
/// v8: src/wasm/baseline/x64/liftoff-assembler-x64-inl.h kScratchRegister2
pub const SCRATCH_REGISTER_2: &str = "r11";
/// v8: kScratchDoubleReg2
pub const SCRATCH_DOUBLE_REG_2: &str = "xmm14";

/// scratch 寄存器绝不进 liftoff 缓存寄存器组 (GP/FP 各一条断言)
/// v8: static_assert "scratch registers must not be used as cache registers"
pub const SCRATCH_NEVER_CACHED: bool = true;

/// 栈槽寻址: [rbp - offset]
/// v8: src/wasm/baseline/x64/liftoff-assembler-x64-inl.h GetStackSlot
pub const STACK_SLOT_BASE: &str = "rbp";
pub fn stack_slot_operand(offset: i32) -> /* Operand */ i32 {
    -offset
}

/// liftoff 帧槽偏移 (fp 之下, 按系统指针宽计)
/// v8: src/execution/x64/frame-constants-x64.h WasmLiftoffFrameConstants
pub const FEEDBACK_VECTOR_OFFSET: i32 = ${feedbackOffset};
pub const INSTANCE_DATA_OFFSET: i32 = ${instanceOffset};

/// 实例数据槽操作数 (编译期常量)
/// v8: kInstanceDataOperand
pub const INSTANCE_DATA_OPERAND: &str = "[rbp - ${instanceOffset}]";
/// OSR 目标槽
/// v8: kOSRTargetSlot
pub const OSR_TARGET_SLOT_OPERAND: &str = "[rbp - kOSRTargetOffset]";

/// 内存操作数: 偏移立即可用条件 is_uint31 (超出走 64 位加法序列)
/// v8: GetMemOp is_uint31 分支
pub const MEM_OFFSET_IMM_BITS: u32 = 31;
/// 索引寄存器默认缩放 1
/// v8: GetMemOp scale_factor = times_1
pub const DEFAULT_SCALE_FACTOR: u32 = 1;

/// 缺 CPU 特性时桩返回 false (调用方 bail out)
/// v8: RETURN_FALSE_IF_MISSING_CPU_FEATURE
pub const MISSING_CPU_FEATURE_BAILS_OUT: bool = true;

/// LiftoffAssembler 方法实现数 (本架构)
/// v8: LiftoffAssembler::* 全部特化
pub const ASSEMBLER_METHODS_COUNT: usize = ${methods};

// —— 提取不变式断言 ——
const _: () = {
    assert_eq!(SCRATCH_REGISTER_2, "r11");
    assert_eq!(SCRATCH_DOUBLE_REG_2, "xmm14");
    assert_eq!(INSTANCE_DATA_OFFSET, 16);
    assert_eq!(FEEDBACK_VECTOR_OFFSET, 24);
    assert_eq!(stack_slot_operand(16), -16);
    assert!(ASSEMBLER_METHODS_COUNT >= 400);
};
`;

const p = writeSrc("v8-wasm-liftoff", "liftoff_assembler_x64.rs.skel", rust);
console.log(`written ${p}`);
