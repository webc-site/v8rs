#!/usr/bin/env bun
// tool/src/wasm/baseline/arm64/liftoff-assembler-arm64-inl.h.js
// 机械提取 v8/src/wasm/baseline/arm64/liftoff-assembler-arm64-inl.h 的 arm64 liftoff 约定

import { readV8, writeSrc, banner } from "../../../../lib.js";

const src = readV8("src/wasm/baseline/arm64/liftoff-assembler-arm64-inl.h");
const frameSrc = readV8("src/execution/arm64/frame-constants-arm64.h");

// 1. 帧布局注释 (liftoff 帧槽自上而下)
for (const anchor of [
  "| return addr (lr)   |",
  "| previous frame (fp)|",
  "| StackFrame::WASM   |",
  "|     instance       |",
  "|     feedback vector|",
  "optional padding slot to keep the stack 16 byte aligned",
]) {
  if (!src.includes(anchor)) throw new Error(`帧布局注释缺失: ${anchor}`);
}

// 2. 帧常量 (frame-constants-arm64.h)
const fb = frameSrc.match(/kFeedbackVectorOffset = (\d+) \* kSystemPointerSize;/);
const ib = frameSrc.match(/kInstanceDataOffset = (\d+) \* kSystemPointerSize;/);
if (!fb || !ib) throw new Error("未找到 arm64 WasmLiftoffFrameConstants");
const feedbackOffset = parseInt(fb[1], 10) * 8;
const instanceOffset = parseInt(ib[1], 10) * 8;

// 3. GetRegFromType: kind → W/X/S/D/Q (switch 内 7 个 case)
const regTypeMatch = src.match(/inline CPURegister GetRegFromType\([\s\S]*?switch \(kind\) \{([\s\S]*?)default:/);
if (!regTypeMatch) throw new Error("未找到 GetRegFromType");
const kindCases = [...regTypeMatch[1].matchAll(/case (k\w+):/g)].map((m) => m[1]);
if (kindCases.join(",") !== "kI32,kI64,kRef,kRefNull,kF32,kF64,kS128") {
  throw new Error(`GetRegFromType case 异常: ${kindCases.join(",")}`);
}
// kI64/kRef/kRefNull 共用 fallthrough 组到 X
for (const [kind, expr] of [
  ["kI32", "reg.gp().W();"],
  ["kF32", "reg.fp().S();"],
  ["kF64", "reg.fp().D();"],
  ["kS128", "reg.fp().Q();"],
]) {
  if (!regTypeMatch[1].includes(`case ${kind}:\n      return ${expr}`)) {
    throw new Error(`GetRegFromType 映射异常: ${kind}`);
  }
}
if (!regTypeMatch[1].includes("case kI64:\n    case kRef:\n    case kRefNull:\n      return reg.gp().X();")) {
  throw new Error("GetRegFromType 引用族映射异常");
}

// 4. 寄存器列表奇偶填充
if (!src.includes("if ((list.Count() & 1) != 0) list.set(padreg);")) {
  throw new Error("未找到 padreg 奇偶填充");
}
if (!src.includes("if ((list.Count() & 1) != 0) list.set(fp_scratch);")) {
  throw new Error("未找到 fp_scratch 奇偶填充");
}

// 5. AcquireByType 按类型取临时寄存器
if (!src.includes("inline CPURegister AcquireByType(UseScratchRegisterScope* temps,")) {
  throw new Error("未找到 AcquireByType");
}

// 6. 方法实现规模
const methods = (src.match(/LiftoffAssembler::/g) || []).length;
if (methods < 400) throw new Error(`LiftoffAssembler 方法 ${methods} < 400`);

console.log(
  `liftoff-assembler-arm64-inl: kind映射=${kindCases.length} 帧偏移(instance)=${instanceOffset} 方法=${methods}`,
);

// 7. 生成 Rust
let rust = banner(
  "src/wasm/baseline/arm64/liftoff-assembler-arm64-inl.h.js",
  "src/wasm/baseline/arm64/liftoff-assembler-arm64-inl.h",
  "arm64 Liftoff 汇编层约定 (帧布局/kind→寄存器宽/奇偶填充)",
);

rust += `
//! arm64 平台 Liftoff 桩: 帧布局与类型化寄存器访问。
/// v8: src/wasm/baseline/arm64/liftoff-assembler-arm64-inl.h namespace liftoff

/// liftoff 帧槽 (fp 之下; 之上为返回地址/参数区, 栈 16 字节对齐含填充槽)
/// v8: src/wasm/baseline/arm64/liftoff-assembler-arm64-inl.h 帧布局注释
pub mod frame {
    /// v8: slot -1 StackFrame::WASM 标记
    pub const WASM_FRAME_MARKER_SLOT: i32 = -1;
    /// v8: slot -2 instance
    pub const INSTANCE_SLOT: i32 = -2;
    /// v8: slot -3 feedback vector
    pub const FEEDBACK_VECTOR_SLOT: i32 = -3;
    /// v8: slot -4 起 value stack 槽
    pub const FIRST_VALUE_SLOT: i32 = -4;
}

/// fp 相对字节偏移 (来自 WasmLiftoffFrameConstants, 系统指针宽 8)
/// v8: src/execution/arm64/frame-constants-arm64.h WasmLiftoffFrameConstants
pub const INSTANCE_DATA_OFFSET: i32 = ${instanceOffset};
pub const FEEDBACK_VECTOR_OFFSET: i32 = ${feedbackOffset};

/// ValueKind → arm64 寄存器宽度
/// v8: src/wasm/baseline/arm64/liftoff-assembler-arm64-inl.h GetRegFromType
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RegWidth {
    /// v8: kI32 → gp().W() (32 位)
    W,
    /// v8: kI64/kRef/kRefNull → gp().X() (64 位)
    X,
    /// v8: kF32 → fp().S()
    S,
    /// v8: kF64 → fp().D()
    D,
    /// v8: kS128 → fp().Q()
    Q,
}

pub fn reg_width(kind: ValueKind) -> RegWidth {
    match kind {
        ValueKind::I32 => RegWidth::W,
        ValueKind::I64 | ValueKind::Ref | ValueKind::RefNull => RegWidth::X,
        ValueKind::F32 => RegWidth::S,
        ValueKind::F64 => RegWidth::D,
        ValueKind::S128 => RegWidth::Q,
    }
}

/// 寄存器列表奇偶填充: GP 补 padreg, FP 补 fp_scratch (调用约定要求偶数个)
/// v8: src/wasm/baseline/arm64/liftoff-assembler-arm64-inl.h PadRegList / PadVRegList
pub fn pad_reg_list(count: usize) -> bool {
    (count & 1) != 0
}

/// 临时寄存器按类型获取 (W/X/S/D/Q)
/// v8: src/wasm/baseline/arm64/liftoff-assembler-arm64-inl.h AcquireByType
pub const ACQUIRE_BY_TYPE: bool = true;

/// LiftoffAssembler 方法实现数 (本架构)
/// v8: LiftoffAssembler::* 全部特化
pub const ASSEMBLER_METHODS_COUNT: usize = ${methods};

// —— 提取不变式断言 ——
const _: () = {
    assert_eq!(INSTANCE_DATA_OFFSET, 16);
    assert_eq!(FEEDBACK_VECTOR_OFFSET, 24);
    assert_eq!(frame::INSTANCE_SLOT, -2);
    assert_eq!(reg_width(ValueKind::I32), RegWidth::W);
    assert_eq!(reg_width(ValueKind::S128), RegWidth::Q);
    assert!(pad_reg_list(3));
    assert!(!pad_reg_list(4));
    assert!(ASSEMBLER_METHODS_COUNT >= 400);
};
`;

const p = writeSrc("v8-wasm-liftoff", "liftoff_assembler_arm64.rs.skel", rust);
console.log(`written ${p}`);
