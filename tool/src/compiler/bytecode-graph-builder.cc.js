#!/usr/bin/env bun
// tool/src/compiler/bytecode-graph-builder.cc.js
// 从 v8/src/compiler/bytecode-graph-builder.h + bytecode-graph-builder.cc
// 宏展开 BYTECODE_CASE 派发表 (BYTECODE_LIST → Visit<Name>), 提取类成员/位旗标/方法面
// 生成 v8-compiler-frontend/src/bytecode_graph_builder.rs.skel
import { readV8, writeSrc, banner } from "../../lib.js";

const h = readV8("src/compiler/bytecode-graph-builder.h");
const cc = readV8("src/compiler/bytecode-graph-builder.cc");

// —— 1. 锚点断言 ——
const anchors = [
  ["enum class BytecodeGraphBuilderFlag : uint8_t {", "旗标枚举"],
  ["kSkipFirstStackAndTierupCheck = 1 << 0", "旗标 bit0"],
  ["kAnalyzeEnvironmentLiveness = 1 << 1", "旗标 bit1"],
  ["kBailoutOnUninitialized = 1 << 2", "旗标 bit2"],
  ["#define BYTECODE_CASE(name, ...)", "BYTECODE_CASE 宏"],
  ["BYTECODE_LIST(BYTECODE_CASE, BYTECODE_CASE)", "派发表双展开"],
  ["void BytecodeGraphBuilder::VisitSingleBytecode()", "主循环访问器"],
  ["class BytecodeGraphBuilder::Environment : public ZoneObject", "Environment 嵌套类"],
  ["static const int kInputBufferSizeIncrement = 64;", "输入缓冲增量"],
  ["SHORT_STAR_BYTECODE_LIST(SHORT_STAR_VISITOR)", "short-star 宏展开"],
];
for (const [lit, name] of anchors) {
  if (!h.includes(lit) && !cc.includes(lit)) {
    throw new Error(`bytecode-graph-builder 锚点缺失: ${name}`);
  }
}

// —— 2. 位旗标逐项提取 (定义在 .h) ——
const flagBlock = h.match(/enum class BytecodeGraphBuilderFlag : uint8_t \{([\s\S]*?)\};/)[1];
const flags = [...flagBlock.matchAll(/(\w+) = (1 << \d+)/g)].map((m) => [m[1], m[2]]);
if (flags.length !== 3) throw new Error(`旗标数量异常: ${flags.length}`);

// —— 3. 派发表宏展开: Visit<Name>() 定义集合 (BYTECODE_CASE 展开产物) ——
const visitors = [...new Set(
  [...cc.matchAll(/void BytecodeGraphBuilder::Visit(\w+)\(\)/g)].map((m) => m[1]),
)];
// 排除非派发入口 (VisitSingleBytecode/VisitBytecodes 是驱动, 非 case 目标)
const dispatch = visitors.filter((n) => n !== "SingleBytecode" && n !== "Bytecodes");

// —— 4. cc 定义面方法名全表 ——
const methods = [...new Set(
  [...cc.matchAll(/^\w[\w:<>*, ]*\bBytecodeGraphBuilder::(\w+)\(/gm)].map((m) => m[1]),
)];

// —— 5. 常量提取 ——
const consts = [
  ["K_INPUT_BUFFER_SIZE_INCREMENT", 64, "kInputBufferSizeIncrement"],
  ["K_BINARY_OPERATION_HINT_INDEX", 1, "kBinaryOperationHintIndex"],
  ["K_COMPARE_OPERATION_HINT_INDEX", 1, "kCompareOperationHintIndex"],
  ["K_COUNT_OPERATION_HINT_INDEX", 0, "kCountOperationHintIndex"],
];

const rustName = (s) => s.replace(/([a-z0-9])([A-Z])/g, "$1_$2").toLowerCase();

const dispatchArms = dispatch
  .map((n) => `    // Bytecode::k${n[0].toLowerCase() + n.slice(1)} → Visit${n}`)
  .join("\n");

const methodSigs = methods
  .filter((m) => !m.startsWith("Visit") && m !== "BytecodeGraphBuilder")
  .map(
    (m) =>
      `  /// v8: src/compiler/bytecode-graph-builder.cc BytecodeGraphBuilder::${m}\n  pub fn ${rustName(m)}(&mut self) -> crate::Result<()> {\n    todo!() // 推迟台账: BGB 执行面\n  }`,
  )
  .join("\n\n");

const visitorFns = dispatch
  .map(
    (n) =>
      `  /// v8: src/compiler/bytecode-graph-builder.cc BytecodeGraphBuilder::Visit${n} (BYTECODE_CASE 展开)\n  fn visit_${rustName(n)}(&mut self) {\n    todo!() // 推迟台账: BGB 字节码访问器\n  }`,
  )
  .join("\n\n");

const body = `${banner("src/compiler/bytecode-graph-builder.cc.js", "src/compiler/bytecode-graph-builder.h + bytecode-graph-builder.cc", "字节码→SeaOf 图构建器: 派发表 + 环境绑定 + 框架构建")}

//! BytecodeGraphBuilder — 遍历 BytecodeArray, 经 Environment 寄存器绑定建 SeaOf 图。

bitflags::bitflags! {
  /// BGB 行为旗标
  /// v8: src/compiler/bytecode-graph-builder.cc BytecodeGraphBuilderFlag
  #[derive(Debug, Clone, Copy, PartialEq, Eq)]
  pub struct BytecodeGraphBuilderFlags: u8 {
    /// 跳过首个 stack/tierup 检查 (OSR)
    const SKIP_FIRST_STACK_AND_TIERUP_CHECK = 1 << 0;
    /// 分析环境活性
    const ANALYZE_ENVIRONMENT_LIVENESS = 1 << 1;
    /// 未初始化变量触发 bailout
    const BAILOUT_ON_UNINITIALIZED = 1 << 2;
  }
}

${consts.map(([n, v, orig]) => `/// v8: src/compiler/bytecode-graph-builder.cc ${orig}\npub const ${n}: i32 = ${v};`).join("\n")}

/// Environment — 字节码偏移处的 SSA 环境 (累加器/寄存器 → Node 绑定 + FrameState)
/// v8: src/compiler/bytecode-graph-builder.cc BytecodeGraphBuilder::Environment
pub struct Environment {
  /// bytecode_offset
  pub bytecode_offset: i32,
  /// 累加器绑定
  pub accumulator: Option<NodeRef>,
}

/// BytecodeGraphBuilder — 主构建器
/// v8: src/compiler/bytecode-graph-builder.cc BytecodeGraphBuilder
pub struct BytecodeGraphBuilder {
  /// 旗标位集
  pub flags: BytecodeGraphBuilderFlags,
}

impl BytecodeGraphBuilder {
${methodSigs}

  // —— BYTECODE_CASE 派发表 (VisitSingleBytecode 内 switch, 宏展开) ——
  /// 派发目标清单 (${dispatch.length} 个 Visit<Name>, 宏先展开产物)
${dispatchArms}

${visitorFns}
}

// —— 对数基准 ——
// 旗标 ${flags.length} / 派发 Visit ${dispatch.length} / cc 定义面 ${methods.length}
// 派发表双槽 BYTECODE_LIST(name, ...) → Visit##name; SHORT_STAR_BYTECODE_LIST 经 FromShortStar 同构
`;

const p = writeSrc("v8-compiler-frontend", "bytecode_graph_builder.rs.skel", body);
console.log(`OK ${p}`);
