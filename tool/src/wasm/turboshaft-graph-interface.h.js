#!/usr/bin/env bun
// tool/src/wasm/turboshaft-graph-interface.h.js
// 机械提取 v8/src/wasm/turboshaft-graph-interface.{h,cc,-inl.h} 的图构建常量与位置编码

import { readV8, writeSrc, banner } from "../../lib.js";

const hSrc = readV8("src/wasm/turboshaft-graph-interface.h");
const ccSrc = readV8("src/wasm/turboshaft-graph-interface.cc");
const inlSrc = readV8("src/wasm/turboshaft-graph-interface-inl.h");

// 1. h: 入口签名与 Validation 别名
if (!hSrc.includes("V8_EXPORT_PRIVATE void BuildTSGraph(")) throw new Error("未找到 BuildTSGraph");
if (!hSrc.includes("using Validation = base::StrongAlias<struct ValidationFlagTag, bool>;")) {
  throw new Error("未找到 Validation 别名");
}
if (!hSrc.includes("class V8_EXPORT_PRIVATE WasmGraphBuilderBase {")) {
  throw new Error("未找到 WasmGraphBuilderBase");
}

// 2. cc: 编译模式四态
const modeMatch = ccSrc.match(/enum Mode \{\s*kRegular,\s*kInlinedUnhandled,\s*kInlinedWithCatch,\s*kInlinedTailCall\s*\};/);
if (!modeMatch) throw new Error("未找到 Mode 四态");

// 3. cc: 异常检查三态
if (!ccSrc.includes("enum class CheckForException { kNo, kCatchInThisFrame, kCatchInParentFrame };")) {
  throw new Error("未找到 CheckForException");
}

// 4. cc: 分支提示模式
if (!ccSrc.includes("enum class BranchHintingMode {\n  kNone,\n  kModuleProvided,\n  kStress,\n};")) {
  throw new Error("未找到 BranchHintingMode");
}

// 5. cc: 压力器位游标 (random_seed 32 位逐位)
if (!ccSrc.includes("int bit = v8_flags.random_seed & (1 << cursor_);")) {
  throw new Error("未找到压力器取位");
}
if (!ccSrc.includes("cursor_ = (cursor_ + 1) & 31;")) throw new Error("未找到游标回绕");

// 6. cc: 解码契约
if (!ccSrc.includes("static constexpr bool kUsesPoppedArgs = true;")) {
  throw new Error("未找到 kUsesPoppedArgs=true");
}

// 7. cc: 宽算术栈槽按架构
if (!ccSrc.includes("kTurboshaftNeedsStackSlotForWideArithmetic =")) {
  throw new Error("未找到宽算术架构门");
}

// 8. cc: br_table 快路径参数
const brTable = [
  ["kMaxComparesPerTarget", "static constexpr int32_t kMaxComparesPerTarget = 2;"],
  ["kMaxTargets", "static constexpr uint32_t kMaxTargets = 3;"],
  ["kMaxTableCount", "static constexpr int32_t kMaxTableCount = 20;"],
];
for (const [n, lit] of brTable) {
  if (!ccSrc.includes(lit)) throw new Error(`br_table 参数缺失: ${n}`);
}

// 9. cc: 批量内存操作内联上限 (copy 112 / fill 144)
const inlineBytes = [...ccSrc.matchAll(/kMaxInlineBytes = (\d+);/g)].map((m) => m[1]);
if (inlineBytes.join(",") !== "112,144") throw new Error(`内联上限异常: ${inlineBytes}`);

// 10. cc: 位置编码位域
const posMatch = ccSrc.match(
  /kPositionFieldShift = (\d+);[\s\S]*?kPositionFieldSize = (\d+);[\s\S]*?kInliningIdFieldSize = (\d+);[\s\S]*?kNoInliningId = (\d+);/,
);
if (!posMatch) throw new Error("未找到位置编码位域");
const [shift, posSize, inlSize, noInl] = posMatch.slice(1).map((v) => parseInt(v, 10));
if (!ccSrc.includes("sizeof(compiler::turboshaft::OperationStorageSlot) ==\n                1 << kPositionFieldShift")) {
  throw new Error("未找到槽宽断言");
}

// 11. cc: phi 初始输入容量与 deopt 输入上限
if (!ccSrc.includes("kInitialInputCapacityPerPhi = 2;")) throw new Error("未找到 phi 容量");
if (!ccSrc.includes("constexpr size_t max_deopt_input_count = 500;")) {
  throw new Error("未找到 deopt 输入上限");
}

// 12. inl: builtin 指针 = Smi 常量
if (!inlSrc.includes('static_assert(std::is_same_v<Smi, BuiltinPtr>, "BuiltinPtr must be Smi")')) {
  throw new Error("未找到 BuiltinPtr=Smi 断言");
}
if (!inlSrc.includes("__ SmiConstant(Smi::FromInt(static_cast<int>(builtin)));")) {
  throw new Error("未找到 Smi 常量构造");
}

console.log(
  `turboshaft-graph-interface: 位域(shift=${shift},pos=${posSize},inl=${inlSize},noId=${noInl}) 内联上限=${inlineBytes}`,
);

// 13. 生成 Rust
let rust = banner(
  "src/wasm/turboshaft-graph-interface.h.js",
  "src/wasm/turboshaft-graph-interface.{h,cc,-inl.h}",
  "Turboshaft wasm 图构建: 模式枚举与 OpIndex 位置编码",
);

rust += `
//! wasm 字节码 → Turboshaft 图的构建接口 (优化层)。
/// v8: src/wasm/turboshaft-graph-interface.cc TurboshaftGraphBuildingInterface

/// 编译模式 (顶层/内联三态)
/// v8: src/wasm/turboshaft-graph-interface.cc enum Mode
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GraphBuildMode {
    /// v8: kRegular — 非内联顶层函数
    Regular,
    /// v8: kInlinedUnhandled — 内联且异常未处理
    InlinedUnhandled,
    /// v8: kInlinedWithCatch — 内联且本帧捕获
    InlinedWithCatch,
    /// v8: kInlinedTailCall — 内联尾调用
    InlinedTailCall,
}

/// 异常检查位置
/// v8: enum class CheckForException
pub const CHECK_EXCEPTION_VALUES: &[&str] = &["kNo", "kCatchInThisFrame", "kCatchInParentFrame"];

/// 分支提示来源 (--stress-wasm-branch-hinting 逐位取 random_seed)
/// v8: enum class BranchHintingMode + BranchHintingStresser
pub const BRANCH_HINTING_MODES: &[&str] = &["kNone", "kModuleProvided", "kStress"];
/// v8: 压力器游标 32 位回绕
pub const STRESSER_CURSOR_MASK: i32 = 31;

/// v8: kUsesPoppedArgs = true
pub const USES_POPPED_ARGS: bool = true;

/// br_table 快路径参数
/// v8: src/wasm/turboshaft-graph-interface.cc kMaxComparesPerTarget 等
pub mod br_table {
    /// v8: kMaxComparesPerTarget = 2
    pub const MAX_COMPARES_PER_TARGET: i32 = 2;
    /// v8: kMaxTargets = 3
    pub const MAX_TARGETS: u32 = 3;
    /// v8: kMaxTableCount = 20
    pub const MAX_TABLE_COUNT: i32 = 20;
}

/// 批量内存操作内联上限 (64 位 + 全非对齐支持 + 常量 size)
/// v8: kMaxInlineBytes (memory.copy / memory.fill)
pub const MAX_INLINE_MEMCPY_BYTES: u32 = 112;
pub const MAX_INLINE_MEMFILL_BYTES: u32 = 144;

/// OpIndex 位置编码: wasm 位置 + 内联 id 打包进节点 origin
/// v8: PositionField / InliningIdField 位域
pub mod position_encoding {
    /// v8: kPositionFieldShift (槽宽 8 字节的 log2)
    pub const SHIFT: u32 = ${shift};
    /// v8: kPositionFieldSize (kV8MaxWasmFunctionSize < 2^23)
    pub const POSITION_BITS: u32 = ${posSize};
    /// v8: kInliningIdFieldSize
    pub const INLINING_ID_BITS: u32 = ${inlSize};
    /// v8: kNoInliningId = 63
    pub const NO_INLINING_ID: u8 = ${noInl};

    /// v8: WasmPositionToOpIndex 打包
    pub fn encode(position: u32, inlining_id: u8) -> u32 {
        (position << SHIFT) | (inlining_id as u32)
    }
}

/// phi 输入初始容量
/// v8: kInitialInputCapacityPerPhi
pub const INITIAL_PHI_INPUT_CAPACITY: u32 = 2;

/// deopt 单输入上限
/// v8: max_deopt_input_count
pub const MAX_DEOPT_INPUT_COUNT: usize = 500;

/// builtin 指针以 Smi 编码 (BuiltinPtr = Smi)
/// v8: src/wasm/turboshaft-graph-interface-inl.h GetBuiltinPointerTarget
pub const BUILTIN_PTR_IS_SMI: bool = true;

// —— 提取不变式断言 ——
const _: () = {
    assert_eq!(position_encoding::SHIFT, 3);
    assert_eq!(position_encoding::POSITION_BITS, 23);
    assert_eq!(position_encoding::INLINING_ID_BITS, 6);
    assert_eq!(position_encoding::NO_INLINING_ID, 63);
    assert_eq!(1 << position_encoding::SHIFT, 8);
    assert_eq!(br_table::MAX_TABLE_COUNT, 20);
    assert_eq!(MAX_INLINE_MEMCPY_BYTES, 112);
    assert_eq!(MAX_INLINE_MEMFILL_BYTES, 144);
    assert_eq!(position_encoding::encode(1, 0), 8);
};
`;

const p = writeSrc("v8-wasm-turbofan", "turboshaft_graph_interface.rs.skel", rust);
console.log(`written ${p}`);
