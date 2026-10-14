#!/usr/bin/env bun
// tool/src/compiler/pipeline-data-inl.h.js
// v8/src/compiler/pipeline-data-inl.h
// → v8-compiler-passes/src/pipeline_data_inl.rs.skel
// TF 管线数据: 解析 TFPipelineData 方法面与字段计数 (含 zone 删除族)。
import { readV8, writeSrc, banner } from "../../lib.js";

const h = readV8("src/compiler/pipeline-data-inl.h");

const need = (cond, msg) => {
  if (!cond) throw new Error(`pipeline-data-inl: ${msg}`);
};

for (const [s, name] of [
  ["class TFPipelineData {", "TFPipelineData"],
  ["void DeleteGraphZone() {", "DeleteGraphZone"],
  ["void DeleteInstructionZone() {", "DeleteInstructionZone"],
  ["void DeleteCodegenZone() {", "DeleteCodegenZone"],
  ["void DeleteRegisterAllocationZone() {", "DeleteRegisterAllocationZone"],
  ["void BeginPhaseKind(const char* phase_kind_name) {", "BeginPhaseKind"],
  ["void InitializeOsrHelper() {", "InitializeOsrHelper"],
  ["void AddTyperFlag(Typer::Flag flag) {", "AddTyperFlag"],
]) need(h.includes(s), `哨兵缺失 ${name}`);

// —— 方法面 (2 空格缩进, inline 体) ——
const methods = [...h.matchAll(/^  (?!\s)(?:void|bool|size_t|int|Node\*|Operator const\*|MaybeIndirectHandle<Code>) (\w+)\(([^()]*)\)(?: const)?\s*[;{]/gm)].map((m) => m[1]);
for (const needM of ["DeleteTyper", "DeleteGraphZone", "DeleteInstructionZone", "DeleteCodegenZone", "DeleteRegisterAllocationZone", "InitializeInstructionSequence", "InitializeFrameData", "InitializeRegisterAllocationData", "InitializeOsrHelper", "InitializeCodeGenerator", "BeginPhaseKind", "EndPhaseKind", "ChooseSpecializationContext", "set_code", "set_graph", "set_schedule", "reset_schedule"]) {
  need(methods.includes(needM), `方法 ${needM} 缺席 (实得 ${methods.length})`);
}

// —— 字段面计数 (private 尾区, 尾下划线命名) ——
const priv = h.slice(h.lastIndexOf(" private:"));
const fields = [...priv.matchAll(/^\s{2}[^\s/][^;{]*?(\w+_(?:_)?) *(?:= *[^;]+)?;/gm)].map((m) => m[1]);
need(fields.length >= 45, `字段数 ${fields.length}`);

// —— DeleteGraphZone 体语义 (zone 重置清关联指针) ——
const dgz = h.slice(h.indexOf("void DeleteGraphZone() {"), h.indexOf("void DeleteInstructionZone()"));
for (const s of ["graph_zone_", "source_positions_ = nullptr", "node_origins_ = nullptr"]) {
  need(dgz.includes(s), `DeleteGraphZone 缺 ${s}`);
}

const body = `${banner("src/compiler/pipeline-data-inl.h.js", "src/compiler/pipeline-data-inl.h", "TF 管线共享数据容器 (zone 族生命周期 + 各阶段产物挂点)")}
//! pipeline-data-inl — TFPipelineData: 全管线共享的可变状态包
//!
//! zone 族: graph / instruction / codegen / register-allocation 四区,
//! Delete*Zone 释放并清关联指针 (source_positions_/node_origins_ 随 graph zone 归零);
//! 方法面 ${methods.length} 个, 字段面 ${fields.length} 个。

/// 原版 TFPipelineData — 管线数据 (引用聚合, 非拥有者语义见各字段)
pub struct TFPipelineData<'z> {
${fields.map((f) => `  /// ${f}\n  pub ${f.replace(/_+$/, "")}: Placeholder,`).join("\n")}
  /// zone 族: graph_zone_ / instruction_zone_ / codegen_zone_ / register_allocation_zone_ (Scope)
  pub graph_zone: ZoneScope<'z>,
  pub instruction_zone: ZoneScope<'z>,
  pub codegen_zone: ZoneScope<'z>,
  pub register_allocation_zone: ZoneScope<'z>,
}

impl TFPipelineData<'_> {
  /// 原版 verify_graph / set_verify_graph / MayHaveUnverifiableGraph
  /// v8: src/compiler/pipeline-data-inl.h TFPipelineData::set_verify_graph
  pub fn set_verify_graph(&mut self, value: bool) {
    todo!("v8: src/compiler/pipeline-data-inl.h TFPipelineData::set_verify_graph")
  }

  /// 原版 DeleteGraphZone — 释放图区并清 source_positions_/node_origins_
  /// v8: src/compiler/pipeline-data-inl.h TFPipelineData::DeleteGraphZone
  pub fn delete_graph_zone(&mut self) {
    todo!("v8: src/compiler/pipeline-data-inl.h TFPipelineData::DeleteGraphZone")
  }

  /// 原版 DeleteInstructionZone / DeleteCodegenZone / DeleteRegisterAllocationZone — 同族
  /// v8: src/compiler/pipeline-data-inl.h TFPipelineData::DeleteInstructionZone
  pub fn delete_instruction_zone(&mut self) {
    todo!("v8: src/compiler/pipeline-data-inl.h TFPipelineData::DeleteInstructionZone (同族 DeleteCodegenZone/DeleteRegisterAllocationZone)")
  }

  /// 原版 InitializeInstructionSequence — 依 CallDescriptor 建指令序列
  /// v8: src/compiler/pipeline-data-inl.h TFPipelineData::InitializeInstructionSequence
  pub fn initialize_instruction_sequence(&mut self, call_descriptor: &CallDescriptor) {
    todo!("v8: src/compiler/pipeline-data-inl.h TFPipelineData::InitializeInstructionSequence")
  }

  /// 原版 InitializeFrameData — 帧数据
  /// v8: src/compiler/pipeline-data-inl.h TFPipelineData::InitializeFrameData
  pub fn initialize_frame_data(&mut self, call_descriptor: &CallDescriptor) {
    todo!("v8: src/compiler/pipeline-data-inl.h TFPipelineData::InitializeFrameData")
  }

  /// 原版 InitializeRegisterAllocationData — 寄存器分配数据
  /// v8: src/compiler/pipeline-data-inl.h TFPipelineData::InitializeRegisterAllocationData
  pub fn initialize_register_allocation_data(&mut self, config: &RegisterConfiguration) {
    todo!("v8: src/compiler/pipeline-data-inl.h TFPipelineData::InitializeRegisterAllocationData")
  }

  /// 原版 InitializeCodeGenerator — 代码生成器
  /// v8: src/compiler/pipeline-data-inl.h TFPipelineData::InitializeCodeGenerator
  pub fn initialize_code_generator(&mut self, linkage: &Linkage) {
    todo!("v8: src/compiler/pipeline-data-inl.h TFPipelineData::InitializeCodeGenerator")
  }

  /// 原版 BeginPhaseKind / EndPhaseKind — 相位类区间 (统计)
  /// v8: src/compiler/pipeline-data-inl.h TFPipelineData::BeginPhaseKind
  pub fn begin_phase_kind(&mut self, phase_kind_name: &'static str) {
    todo!("v8: src/compiler/pipeline-data-inl.h TFPipelineData::BeginPhaseKind")
  }

  /// 原版 ChooseSpecializationContext — 特化 context 选择
  /// v8: src/compiler/pipeline-data-inl.h TFPipelineData::ChooseSpecializationContext
  pub fn choose_specialization_context(&mut self) {
    todo!("v8: src/compiler/pipeline-data-inl.h TFPipelineData::ChooseSpecializationContext")
  }

  /// 原版 AddTyperFlag / DeleteTyper — typer 标志与销毁
  /// v8: src/compiler/pipeline-data-inl.h TFPipelineData::AddTyperFlag
  pub fn add_typer_flag(&mut self, flag: TyperFlags) {
    todo!("v8: src/compiler/pipeline-data-inl.h TFPipelineData::AddTyperFlag")
  }

  /// 原版 InitializeOsrHelper — OSR 辅助初始化
  /// v8: src/compiler/pipeline-data-inl.h TFPipelineData::InitializeOsrHelper
  pub fn initialize_osr_helper(&mut self) {
    todo!("v8: src/compiler/pipeline-data-inl.h TFPipelineData::InitializeOsrHelper")
  }
}
`;

const p = writeSrc("v8-compiler-passes", "pipeline_data_inl.rs.skel", body);
console.log(`written ${p}\nmethods=${methods.length} fields=${fields.length} (Delete 族 4 区核对通过)`);
