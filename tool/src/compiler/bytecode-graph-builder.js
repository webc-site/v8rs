#!/usr/bin/env bun
// v8/src/compiler/bytecode-graph-builder.h + bytecode-graph-builder.cc (建图纯面)
// → v8-compiler-graph/src/build_graph.rs 基线
//
// 台账锚定: 登记名 `NativeContext::BuildGraph` = BuildGraphFromBytecode (cc:4885;
// 本 checkout 旧名 BytecodeGraphBuilder::BuildGraph 已改 CreateGraph, cc:1183) —
// 入口把 broker->target_native_context() (cc:4894) 直传 builder, 建图全程建立/使用
// native context 节点 (cc:1158/1163)。只落地不耦合 JSHeapBroker / JSGraph /
// BytecodeAnalysis / CommonOperatorBuilder 的纯图子集: EnsureInputBufferSize
// (cc:4671) 扩容公式、CreateGraph 的 Start/End 骨架 (cc:1191-1193/1207-1211)、
// NewPhi/NewEffectPhi (cc:4798/4806, 同形合一; common()->Phi/EffectPhi/Start/End
// 算子构造属 v8-compiler-ops, 按 crate 惯例作参注入)。
// 硬耦合面 (Environment 依赖追踪 / Visit* 字节码 handler 族 / 异常 handler 表 /
// jsgraph 常量节点 / MakeNode 上下文与依赖布线 / Merge* 参数算子重建) 推迟见 readme 台账。
// 对数: 结构锚 12 项必现 + 对数断言 10 项逐字面计数 (字面串匹配, 不用正则转义)
// + 行号锚 14 项 (本 checkout 定位行号, 防台账行号漂移)。
import { readV8, writeSrc, banner } from "../../lib.js";

const h = readV8("src/compiler/bytecode-graph-builder.h");
const cc = readV8("src/compiler/bytecode-graph-builder.cc");

// 字面串出现次数 (原版侧计数, 免正则转义陷阱)
const count = (src, needle) => src.split(needle).length - 1;

const presence = [
  [h, "void BuildGraphFromBytecode(", "建图入口声明"],
  [cc, "broker->target_native_context()", "native context 为纲锚证"],
  [cc, "builder.CreateGraph();", "入口委托 CreateGraph"],
  [cc, "static const int kInputBufferSizeIncrement = 64;", "缓冲扩容步长常量"],
  [cc, "size = size + kInputBufferSizeIncrement + input_buffer_size_;", "扩容公式逐字"],
  [cc, "input_buffer_ = local_zone()->AllocateArray<Node*>(size);", "zone bump 扩容"],
  [cc, "return input_buffer_;", "缓冲基址返回"],
  [cc, "graph()->SetStart(graph()->NewNode(common()->Start(start_output_arity)));", "CreateGraph 头 SetStart 路"],
  [cc, "DCHECK_NE(0u, exit_controls_.size());", "CreateGraph 尾非空断言"],
  [cc, "Node** const inputs = &exit_controls_.front();", "exit_controls 连续输入取基址"],
  [cc, "Node* end = graph()->NewNode(common()->End(input_count), input_count, inputs);", "CreateGraph 尾 End 建节点"],
  [cc, "graph()->SetEnd(end);", "SetEnd 回写"],
];
for (const [src, needle, name] of presence) {
  if (!src.includes(needle)) throw new Error(`结构锚缺失: ${name}`);
}

const tally = [
  [cc, "int input_buffer_size_;", 1, "容量字段"],
  [cc, "Node** input_buffer_;", 1, "缓冲字段"],
  [cc, "Node** BytecodeGraphBuilder::EnsureInputBufferSize(int size)", 1, "扩容函数定义"],
  [cc, "void BytecodeGraphBuilder::CreateGraph()", 1, "建图骨架定义"],
  [cc, "native_context_node_", 6, "native context 节点面 (声明/初始化/存取器 DCHECK/返回/创建写)"],
  [cc, "Node* BytecodeGraphBuilder::NewPhi(", 1, "Phi 构造面"],
  [cc, "Node* BytecodeGraphBuilder::NewEffectPhi(", 1, "EffectPhi 构造面"],
  [cc, "std::fill_n(buffer, count, input);", 2, "输入填充 (Phi/EffectPhi 同形)"],
  [cc, "buffer[count] = control;", 2, "control 尾置 (Phi/EffectPhi 同形)"],
  [cc, "graph()->NewNode(phi_op, count + 1, buffer, true);", 2, "incomplete 建节点 (Phi/EffectPhi 同形)"],
];
for (const [src, needle, want, name] of tally) {
  const got = count(src, needle);
  if (got !== want) throw new Error(`对数不符: ${name} 期望 ${want} 实得 ${got}`);
}

// 行号锚: 逐锚核对本 checkout 的实际行号, 防注释/台账行号漂移 (出现序对齐)
const linesOf = (src, needle) => {
  const out = [];
  let i = src.indexOf(needle);
  while (i !== -1) {
    out.push(src.slice(0, i).split("\n").length);
    i = src.indexOf(needle, i + 1);
  }
  return out;
};
const lineAnchors = [
  [cc, "static const int kInputBufferSizeIncrement = 64;", [384], "增量常量行号"],
  [cc, "Node** BytecodeGraphBuilder::EnsureInputBufferSize(int size)", [4671], "Ensure 定义行号"],
  [cc, "size = size + kInputBufferSizeIncrement + input_buffer_size_;", [4673], "扩容公式行号"],
  [cc, "void BytecodeGraphBuilder::CreateGraph()", [1183], "CreateGraph 定义行号"],
  [cc, "graph()->SetStart(graph()->NewNode(common()->Start(start_output_arity)));", [1193], "SetStart 路行号"],
  [cc, "DCHECK_NE(0u, exit_controls_.size());", [1207], "尾部非空断言行号"],
  [cc, "graph()->SetEnd(end);", [1211], "SetEnd 行号"],
  [cc, "Node* BytecodeGraphBuilder::NewPhi(", [4798], "NewPhi 定义行号"],
  [cc, "Node* BytecodeGraphBuilder::NewEffectPhi(", [4806], "NewEffectPhi 定义行号"],
  [cc, "std::fill_n(buffer, count, input);", [4801, 4810], "fill_n 双现行号"],
  [cc, "buffer[count] = control;", [4802, 4811], "control 尾置双现行号"],
  [cc, "graph()->NewNode(phi_op, count + 1, buffer, true);", [4803, 4812], "incomplete 建节点双现行号"],
  [cc, "void BuildGraphFromBytecode(", [4885], "入口定义行号"],
  [cc, "broker->target_native_context()", [4894], "target_native_context 行号"],
];
for (const [src, needle, want, name] of lineAnchors) {
  const got = linesOf(src, needle);
  if (got.join() !== want.join()) throw new Error(`行号锚不符: ${name} 期望 ${want} 实得 ${got}`);
}

const body = `${banner("src/compiler/bytecode-graph-builder.js", "src/compiler/bytecode-graph-builder.h + bytecode-graph-builder.cc")}
//! BytecodeGraphBuilder 建图纯面 — native context 为纲的建图入口的图构建原语
//!
//! 锚定: 台账名 \`NativeContext::BuildGraph\` = \`BuildGraphFromBytecode\` (cc:4885; 本 checkout
//! 旧名 BytecodeGraphBuilder::BuildGraph 已改 CreateGraph, cc:1183) — 入口把
//! \`broker->target_native_context()\` (cc:4894) 直传 builder, 建图全程建立/使用
//! native context 节点 (cc:106/107/517/1108/1158-1166)。
//! 仅落地不耦合 JSHeapBroker / JSGraph / BytecodeAnalysis / CommonOperatorBuilder 的部分:
//! 输入缓冲扩容 (EnsureInputBufferSize cc:4671)、CreateGraph 的 Start/End 骨架 (cc:1191-1193/1207-1211)、
//! 输入聚合建节点 (NewPhi/NewEffectPhi cc:4798/4806, 同形合一, 算子按 crate 惯例作参注入)。
//! 硬耦合面 (Environment 依赖追踪 / Visit* 字节码 handler 族 / 异常 handler 表 / 上下文与
//! 反馈常量节点 / Merge* 参数算子重建) 推迟登记见 readme 台账。

use std::{mem::size_of, ptr, slice};

use v8_zone::Zone;

use crate::{Error, Graph, Node, Operator, Result};

/// 原版 kInputBufferSizeIncrement (cc:384)
pub const K_INPUT_BUFFER_SIZE_INCREMENT: usize = 64;

/// 原版 BytecodeGraphBuilder 输入缓冲字段 (cc:512-513) — 建节点期的输入聚合区
///
/// 扩容时从 zone 重新 bump 分配、旧缓冲弃用 (原版 local_zone_ 无释放语义)。
/// 槽位类型 Node* 按引用宽度对齐落地为 \`&Node\` 槽。
pub struct InputBuffer<'z> {
  /// input_buffer_
  buffer: *mut &'z Node,
  /// input_buffer_size_
  size: usize,
}

impl<'z> InputBuffer<'z> {
  /// 原版构造初始化 (cc:1104-1105): size 0 / buffer 空
  pub const fn new() -> Self {
    Self {
      buffer: ptr::null_mut(),
      size: 0,
    }
  }

  /// 原版 EnsureInputBufferSize (cc:4671-4678) — 扩容公式 \`需求 + 64 + 旧容量\` 逐字一致
  pub fn ensure(&mut self, zone: &Zone<'_>, size: usize) -> Result<()> {
    if size > self.size {
      let new = size + K_INPUT_BUFFER_SIZE_INCREMENT + self.size;
      let buf = zone.allocate_bytes_ref(new * size_of::<&'z Node>())?;
      // SAFETY: zone bump 分配至 reset 前地址稳定 (同 Node/OOL 分配纪律); 槽宽=引用宽
      self.buffer = buf.as_mut_ptr().cast();
      self.size = new;
    }
    Ok(())
  }

  /// 原版 input_buffer_size_ (容量而非已用槽数)
  pub fn size(&self) -> usize {
    self.size
  }

  /// 原版 \`std::fill_n(buffer, count, input)\` 路 (cc:4801/4810)
  pub fn fill(&mut self, count: usize, value: &'z Node) {
    debug_assert!(count <= self.size, "输入缓冲越容量");
    // SAFETY: [0,count) 在容量内; ptr::write 只初始化不读旧值
    unsafe {
      for i in 0..count {
        ptr::write(self.buffer.add(i), value);
      }
    }
  }

  /// 原版 \`buffer[index] = value\` 路 (cc:4802/4811)
  pub fn set_at(&mut self, index: usize, value: &'z Node) {
    debug_assert!(index < self.size, "输入缓冲越容量");
    // SAFETY: index 在容量内
    unsafe { ptr::write(self.buffer.add(index), value) };
  }

  /// 取前 \`count\` 个已初始化槽 (原版直接把 Node** 递给 NewNode 的路)
  ///
  /// 契约: [0,count) 已由 fill/set_at 写入 (与原版调用方同一纪律, 违者 UB)。
  pub fn view(&self, count: usize) -> &[&'z Node] {
    debug_assert!(count <= self.size);
    // SAFETY: 契约保证 [0,count) 全部初始化且指向存活节点
    unsafe { slice::from_raw_parts(self.buffer as *const &'z Node, count) }
  }
}

impl Default for InputBuffer<'_> {
  fn default() -> Self {
    Self::new()
  }
}

/// 原版 CreateGraph 头部 (cc:1191-1193) — \`SetStart(NewNode(Start(arity)))\` 建图路
///
/// arity 公式 (\`StartNode::OutputArityForFormalParameterCount\`) 与 \`common()->Start\`
/// 构造属 v8-compiler-ops (common-operator.h), start_op 作参注入。
pub fn create_start<'z, 'a>(graph: &mut Graph<'z, 'a>, start_op: &Operator) -> Result<&'z Node> {
  let start = graph.new_node(start_op, &[], false)?;
  graph.set_start(start);
  Ok(start)
}

/// 原版 CreateGraph 尾部 (cc:1207-1211) — exit_controls 非空作输入 \`NewNode(End) + SetEnd\`
///
/// 原版 DCHECK_NE(0u, exit_controls_.size()) 按 crate 惯例映射 Err;
/// \`common()->End(input_count)\` 构造属 v8-compiler-ops, end_op 作参注入。
pub fn create_end<'z, 'a>(
  graph: &mut Graph<'z, 'a>,
  end_op: &Operator,
  exit_controls: &[&Node],
) -> Result<&'z Node> {
  if exit_controls.is_empty() {
    return Err(Error::CountOutOfRange {
      what: "exit_controls",
      val: 0,
    });
  }
  let end = graph.new_node(end_op, exit_controls, false)?;
  graph.set_end(end);
  Ok(end)
}

/// 原版 NewPhi / NewEffectPhi (cc:4798-4813) — count 份输入 + 尾置 control, incomplete 建节点
///
/// 两原版函数仅 \`common()->Phi(kTagged, n)\` / \`common()->EffectPhi(n)\` 构造点之差
/// (属 v8-compiler-ops), 图构建形全同, 合一落地; phi_op 作参注入。
pub fn new_phi<'z>(
  graph: &Graph<'z, '_>,
  buffer: &mut InputBuffer<'z>,
  phi_op: &Operator,
  count: usize,
  input: &'z Node,
  control: &'z Node,
) -> Result<&'z Node> {
  buffer.ensure(graph.zone(), count + 1)?;
  buffer.fill(count, input);
  buffer.set_at(count, control);
  let inputs = buffer.view(count + 1);
  graph.new_node(phi_op, inputs, true)
}
`;

const p = writeSrc("v8-compiler-graph", "build_graph.rs", body);
console.log(`written ${p} (结构锚 ${presence.length} + 对数断言 ${tally.length} + 行号锚 ${lineAnchors.length} 全过)`);
