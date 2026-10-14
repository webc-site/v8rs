//! graph_assembler — 在 SeaOf 图上以 effect/control 传递方式组装子图
//!
//! 1:1 对应 v8/src/compiler/graph-assembler.{h,cc}。Label 状态机
//! (merged_count/bindings) + MergeState/Bind/Goto/Branch 为纯面直译;
//! loop/LoopExit 标记与 inline reducer 挂钩推迟 (台账)。

use v8_compiler_graph::{Graph, IrOpcode, MachineRepresentation, Node, NodeId, NodeProperties};
use v8_compiler_ops::{BranchHint, BranchSemantics, CommonOperatorBuilder};
use v8_zone::Zone;

use crate::js_graph::JSGraph;

/// 节点引用别名 (原版 Node*)
pub type NodeRef<'z> = &'z Node;

/// Label 类型
/// v8: src/compiler/graph-assembler.h GraphAssemblerLabelType
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GraphAssemblerLabelType {
  /// 延迟块 (冷路径)
  Deferred,
  NonDeferred,
  /// 循环头 (需 Phi 回边合并)
  Loop,
}

/// Label — 组装器基本块句柄 (effect/control + 变量 phi 绑定)
///
/// 原版模板参 VarCount 是容量优化 (SmallVector vs array), Rust 侧统一 Vec。
/// v8: src/compiler/graph-assembler.h GraphAssemblerLabel
#[derive(Debug)]
pub struct GraphAssemblerLabel<'z> {
  /// is_bound_
  pub is_bound: bool,
  /// type_
  pub ty: GraphAssemblerLabelType,
  /// loop_nesting_level_
  pub loop_nesting_level: i32,
  /// merged_count_
  pub merged_count: i32,
  /// effect_
  pub effect: Option<NodeRef<'z>>,
  /// control_
  pub control: Option<NodeRef<'z>>,
  /// bindings_ (变量 phi)
  pub bindings: Vec<Option<NodeRef<'z>>>,
  /// representations_
  pub representations: Vec<MachineRepresentation>,
}

impl<'z> GraphAssemblerLabel<'z> {
  /// 构造: 记录当前循环层级, bindings 初始化为 None
  /// v8: src/compiler/graph-assembler.h GraphAssemblerLabel::GraphAssemblerLabel
  pub fn new(
    ty: GraphAssemblerLabelType,
    loop_nesting_level: i32,
    representations: Vec<MachineRepresentation>,
  ) -> Self {
    let bindings = vec![None; representations.len()];
    Self {
      is_bound: false,
      ty,
      loop_nesting_level,
      merged_count: 0,
      effect: None,
      control: None,
      bindings,
      representations,
    }
  }

  /// v8: src/compiler/graph-assembler.h GraphAssemblerLabel::Count
  pub fn count(&self) -> usize {
    self.representations.len()
  }

  /// v8: src/compiler/graph-assembler.h GraphAssemblerLabel::IsUsed
  pub fn is_used(&self) -> bool {
    self.merged_count > 0
  }

  /// v8: src/compiler/graph-assembler.cc GraphAssemblerLabel::PhiAt
  /// (原版 DCHECK(IsBound) && index < Count)
  pub fn phi_at(&self, index: usize) -> Option<NodeRef<'z>> {
    debug_assert!(self.is_bound, "PhiAt 要求已 Bind");
    debug_assert!(index < self.count(), "PhiAt 越界");
    self.bindings[index]
  }

  /// v8: src/compiler/graph-assembler.h GraphAssemblerLabel::IsDeferred
  pub const fn is_deferred(&self) -> bool {
    matches!(self.ty, GraphAssemblerLabelType::Deferred)
  }

  /// v8: src/compiler/graph-assembler.h GraphAssemblerLabel::IsLoop
  pub const fn is_loop(&self) -> bool {
    matches!(self.ty, GraphAssemblerLabelType::Loop)
  }

  /// v8: src/compiler/graph-assembler.h GraphAssemblerLabel::IsBound
  pub const fn is_bound(&self) -> bool {
    self.is_bound
  }

  /// SetBound — DCHECK(!IsBound)
  /// v8: src/compiler/graph-assembler.h GraphAssemblerLabel::SetBound
  fn set_bound(&mut self) {
    debug_assert!(!self.is_bound, "SetBound 不得重复");
    self.is_bound = true;
  }
}

/// GraphAssembler — effect/control 游标 + 块合并状态机
/// v8: src/compiler/graph-assembler.h GraphAssembler
pub struct GraphAssembler<'g, 'z, 'a> {
  /// mcgraph_ (JS 门面对象)
  pub jsgraph: &'g mut JSGraph<'z, 'a>,
  /// temp_zone_
  pub temp_zone: &'z Zone<'a>,
  /// default_branch_semantics_
  pub default_branch_semantics: BranchSemantics,
  /// effect_
  effect: Option<NodeRef<'z>>,
  /// control_
  control: Option<NodeRef<'z>>,
  /// loop_nesting_level_ (最外层 0)
  loop_nesting_level: i32,
  /// mark_loop_exits_
  mark_loop_exits: bool,
}

impl<'g, 'z, 'a> GraphAssembler<'g, 'z, 'a> {
  /// v8: src/compiler/graph-assembler.cc GraphAssembler::GraphAssembler
  pub fn new(
    jsgraph: &'g mut JSGraph<'z, 'a>,
    temp_zone: &'z Zone<'a>,
    default_branch_semantics: BranchSemantics,
    mark_loop_exits: bool,
  ) -> Self {
    Self {
      jsgraph,
      temp_zone,
      default_branch_semantics,
      effect: None,
      control: None,
      loop_nesting_level: 0,
      mark_loop_exits,
    }
  }

  /// v8: src/compiler/graph-assembler.h GraphAssembler::graph
  pub fn graph(&self) -> &Graph<'z, 'a> {
    &self.jsgraph.graph
  }

  /// v8: src/compiler/graph-assembler.h GraphAssembler::common
  pub fn common(&self) -> &CommonOperatorBuilder<'z, 'a> {
    &self.jsgraph.common
  }

  /// v8: src/compiler/graph-assembler.h GraphAssembler::temp_zone
  pub fn zone(&self) -> &'z Zone<'a> {
    self.temp_zone
  }

  /// v8: src/compiler/graph-assembler.h GraphAssembler::control
  pub const fn control(&self) -> Option<NodeRef<'z>> {
    self.control
  }

  /// v8: src/compiler/graph-assembler.h GraphAssembler::effect
  pub const fn effect(&self) -> Option<NodeRef<'z>> {
    self.effect
  }

  /// 当前块活跃 = control 非空 (被 Goto/Unreachable 终止后须再 Bind)
  /// v8: src/compiler/graph-assembler.h GraphAssembler::HasActiveBlock
  pub const fn has_active_block(&self) -> bool {
    self.control.is_some()
  }

  /// v8: src/compiler/graph-assembler.h GraphAssembler::start
  pub fn start(&self) -> Option<NodeRef<'z>> {
    self.graph().start()
  }

  /// v8: src/compiler/graph-assembler.cc GraphAssembler::Reset
  pub fn reset(&mut self) {
    self.effect = None;
    self.control = None;
  }

  /// v8: src/compiler/graph-assembler.cc GraphAssembler::InitializeEffectControl
  pub fn initialize_effect_control(&mut self, effect: NodeRef<'z>, control: NodeRef<'z>) {
    self.effect = Some(effect);
    self.control = Some(control);
  }

  /// MakeLabelFor — 记录当前循环层级
  /// v8: src/compiler/graph-assembler.h GraphAssembler::MakeLabelFor
  pub fn make_label_for(
    &self,
    ty: GraphAssemblerLabelType,
    reps: Vec<MachineRepresentation>,
  ) -> GraphAssemblerLabel<'z> {
    GraphAssemblerLabel::new(ty, self.loop_nesting_level, reps)
  }

  /// v8: src/compiler/graph-assembler.h GraphAssembler::MakeLabel
  pub fn make_label(&self, reps: Vec<MachineRepresentation>) -> GraphAssemblerLabel<'z> {
    self.make_label_for(GraphAssemblerLabelType::NonDeferred, reps)
  }

  /// v8: src/compiler/graph-assembler.h GraphAssembler::MakeLoopLabel
  pub fn make_loop_label(&self, reps: Vec<MachineRepresentation>) -> GraphAssemblerLabel<'z> {
    self.make_label_for(GraphAssemblerLabelType::Loop, reps)
  }

  /// v8: src/compiler/graph-assembler.h GraphAssembler::MakeDeferredLabel
  pub fn make_deferred_label(&self, reps: Vec<MachineRepresentation>) -> GraphAssemblerLabel<'z> {
    self.make_label_for(GraphAssemblerLabelType::Deferred, reps)
  }

  /// AddNode — 原版先过 inline reducers / node_changed_callback (推迟), 现直通
  /// v8: src/compiler/graph-assembler.cc GraphAssembler::AddNode
  pub fn add_node(&mut self, node: NodeRef<'z>) -> NodeRef<'z> {
    self.update_effect_control_with(node);
    node
  }

  /// UpdateEffectControlWith — 按算子输出类别推进游标
  /// v8: src/compiler/graph-assembler.h GraphAssembler::UpdateEffectControlWith
  pub fn update_effect_control_with(&mut self, node: NodeRef<'z>) {
    if node.op().effect_output_count() > 0 {
      self.effect = Some(node);
    }
    if node.op().control_output_count() > 0 {
      self.control = Some(node);
    }
  }

  /// MergeState — 汇入 label: 首跳存三元组, 二次建 Merge/EffectPhi/Phi, 追加扩输入
  /// v8: src/compiler/graph-assembler.h GraphAssembler::MergeState
  pub fn merge_state(
    &mut self,
    label: &mut GraphAssemblerLabel<'z>,
    vars: &[NodeRef<'z>],
  ) -> crate::Result<()> {
    debug_assert_eq!(vars.len(), label.count(), "MergeState 变量数不符");
    // loop-exit 标记 (mark_loop_exits 特性) 依赖 LoopExitValue 算子 — 推迟
    if label.loop_nesting_level != self.loop_nesting_level {
      debug_assert!(self.mark_loop_exits, "loop-exit 标记未启用");
      debug_assert!(!label.is_loop(), "loop → loop 跳转不支持");
      todo!() // 推迟台账: common()->LoopExitValue 算子 (v8-compiler-ops)
    }
    if label.is_loop() {
      todo!() // 推迟台账: NodeProperties::MergeControlToEnd (v8-compiler-graph)
    }
    debug_assert!(!label.is_bound());
    let control = self.control.expect("MergeState 要求 control 非空");
    let effect = self.effect.expect("MergeState 要求 effect 非空");
    let merged_count = label.merged_count;
    if merged_count == 0 {
      label.control = Some(control);
      label.effect = Some(effect);
      for (i, &var) in vars.iter().enumerate() {
        label.bindings[i] = Some(var);
      }
    } else if merged_count == 1 {
      let old_control = label.control.expect("merged_count==1 必有 control");
      let old_effect = label.effect.expect("merged_count==1 必有 effect");
      let merge = self
        .graph()
        .new_node(self.common().merge(2)?, &[old_control, control], false)?;
      label.control = Some(merge);
      label.effect = Some(self.graph().new_node(
        self.common().effect_phi(2)?,
        &[old_effect, effect, merge],
        false,
      )?);
      for (i, &var) in vars.iter().enumerate() {
        let old_var = label.bindings[i].expect("bindings 齐");
        let op = self.common().phi(label.representations[i], 2)?;
        label.bindings[i] = Some(self.graph().new_node(op, &[old_var, var, merge], false)?);
      }
    } else {
      // 追加输入并扩算子 arity
      let merge = label.control.expect("merged>1 必有 merge");
      debug_assert_eq!(merge.opcode(), IrOpcode::Merge);
      merge.append_input(self.zone(), control)?;
      let new_merge_op = self.common().merge((merged_count + 1) as u32)?;
      NodeProperties::change_op(merge, new_merge_op);

      let effect_phi = label.effect.expect("merged>1 必有 effect phi");
      debug_assert_eq!(effect_phi.opcode(), IrOpcode::EffectPhi);
      effect_phi.replace_input(merged_count as usize, Some(effect));
      effect_phi.append_input(self.zone(), merge)?;
      let new_efp_op = self.common().effect_phi((merged_count + 1) as u32)?;
      NodeProperties::change_op(effect_phi, new_efp_op);

      for (i, &var) in vars.iter().enumerate() {
        let phi = label.bindings[i].expect("bindings 齐");
        debug_assert_eq!(phi.opcode(), IrOpcode::Phi);
        phi.replace_input(merged_count as usize, Some(var));
        phi.append_input(self.zone(), merge)?;
        let new_phi_op = self
          .common()
          .phi(label.representations[i], (merged_count + 1) as u32)?;
        NodeProperties::change_op(phi, new_phi_op);
        // 原版 typed phi 类型合并 (Type::Union) — NodeProperties 类型面未导出
        // 推迟台账: is_typed/get_type/set_type (v8-compiler-graph)
      }
    }
    label.merged_count += 1;
    Ok(())
  }

  /// Bind — 落位 label; merged==1 且非 loop 时插 Merge(1) 占位
  /// v8: src/compiler/graph-assembler.h GraphAssembler::Bind
  pub fn bind(&mut self, label: &mut GraphAssemblerLabel<'z>) -> crate::Result<()> {
    debug_assert!(
      self.control.is_none() && self.effect.is_none(),
      "Bind 要求游标清空"
    );
    debug_assert!(label.merged_count > 0, "Bind 要求至少一次汇入");
    debug_assert_eq!(
      label.loop_nesting_level, self.loop_nesting_level,
      "跨层 Bind"
    );
    self.control = label.control;
    self.effect = label.effect;
    label.set_bound();
    if label.merged_count > 1 || label.is_loop() {
      let control = self.control.expect("bind control");
      let effect = self.effect.expect("bind effect");
      self.add_node(control);
      self.add_node(effect);
      for binding in label.bindings.iter().take(label.count()) {
        let phi = binding.expect("bindings 齐");
        self.add_node(phi);
      }
    } else {
      // 无控制节点时插 Merge(1) 占位, 供后续 pass 起点
      let control = self.control.expect("bind control");
      let merge1 = self
        .graph()
        .new_node(self.common().merge(1)?, &[control], false)?;
      self.control = Some(merge1);
    }
    Ok(())
  }

  /// Goto — 汇入后清空游标
  /// v8: src/compiler/graph-assembler.h GraphAssembler::Goto
  pub fn goto_(
    &mut self,
    label: &mut GraphAssemblerLabel<'z>,
    vars: &[NodeRef<'z>],
  ) -> crate::Result<()> {
    debug_assert!(
      self.control.is_some() && self.effect.is_some(),
      "Goto 要求游标非空"
    );
    self.merge_state(label, vars)?;
    self.control = None;
    self.effect = None;
    Ok(())
  }

  /// BranchImpl — Branch + IfTrue/IfFalse 双汇入, 游标清空
  /// v8: src/compiler/graph-assembler.h GraphAssembler::BranchImpl
  pub fn branch_impl(
    &mut self,
    semantics: BranchSemantics,
    condition: NodeRef<'z>,
    if_true: &mut GraphAssemblerLabel<'z>,
    if_false: &mut GraphAssemblerLabel<'z>,
    hint: BranchHint,
    vars: &[NodeRef<'z>],
  ) -> crate::Result<()> {
    debug_assert!(self.control.is_some(), "Branch 要求 control 非空");
    let control = self.control.expect("branch control");
    let branch_op = self.common().branch(hint, semantics);
    let branch = self
      .graph()
      .new_node(branch_op, &[condition, control], false)?;
    self.control = Some(
      self
        .graph()
        .new_node(self.common().if_true(), &[branch], false)?,
    );
    self.merge_state(if_true, vars)?;
    self.control = Some(
      self
        .graph()
        .new_node(self.common().if_false(), &[branch], false)?,
    );
    self.merge_state(if_false, vars)?;
    self.control = None;
    self.effect = None;
    Ok(())
  }

  /// Branch — deferred 差异自动定 hint
  /// v8: src/compiler/graph-assembler.h GraphAssembler::Branch
  pub fn branch(
    &mut self,
    condition: NodeRef<'z>,
    if_true: &mut GraphAssemblerLabel<'z>,
    if_false: &mut GraphAssemblerLabel<'z>,
    vars: &[NodeRef<'z>],
  ) -> crate::Result<()> {
    let hint = match (if_true.is_deferred(), if_false.is_deferred()) {
      (true, false) => BranchHint::False,
      (false, true) => BranchHint::True,
      _ => BranchHint::None,
    };
    self.branch_impl(
      self.default_branch_semantics,
      condition,
      if_true,
      if_false,
      hint,
      vars,
    )
  }

  /// GotoIf(hint) — 跳入 label, 顺流留在 IfFalse 分支
  /// v8: src/compiler/graph-assembler.h GraphAssembler::GotoIf
  pub fn goto_if(
    &mut self,
    condition: NodeRef<'z>,
    label: &mut GraphAssemblerLabel<'z>,
    hint: BranchHint,
    vars: &[NodeRef<'z>],
  ) -> crate::Result<()> {
    let control = self.control.expect("GotoIf 要求 control 非空");
    let branch_op = self.common().branch(hint, self.default_branch_semantics);
    let branch = self
      .graph()
      .new_node(branch_op, &[condition, control], false)?;
    self.control = Some(
      self
        .graph()
        .new_node(self.common().if_true(), &[branch], false)?,
    );
    self.merge_state(label, vars)?;
    let if_false = self
      .graph()
      .new_node(self.common().if_false(), &[branch], false)?;
    self.control = Some(self.add_node(if_false));
    Ok(())
  }

  /// GotoIfNot(hint) — 跳入 label 走 IfFalse 分支
  /// v8: src/compiler/graph-assembler.h GraphAssembler::GotoIfNot
  pub fn goto_if_not(
    &mut self,
    condition: NodeRef<'z>,
    label: &mut GraphAssemblerLabel<'z>,
    hint: BranchHint,
    vars: &[NodeRef<'z>],
  ) -> crate::Result<()> {
    let control = self.control.expect("GotoIfNot 要求 control 非空");
    let branch_op = self.common().branch(hint, self.default_branch_semantics);
    let branch = self
      .graph()
      .new_node(branch_op, &[condition, control], false)?;
    self.control = Some(
      self
        .graph()
        .new_node(self.common().if_false(), &[branch], false)?,
    );
    self.merge_state(label, vars)?;
    let if_true = self
      .graph()
      .new_node(self.common().if_true(), &[branch], false)?;
    self.control = Some(self.add_node(if_true));
    Ok(())
  }

  /// GotoIf 默认 hint: 目标 deferred → kFalse
  /// v8: src/compiler/graph-assembler.h GraphAssembler::GotoIf (无 hint 重载)
  pub fn goto_if_default(
    &mut self,
    condition: NodeRef<'z>,
    label: &mut GraphAssemblerLabel<'z>,
    vars: &[NodeRef<'z>],
  ) -> crate::Result<()> {
    let hint = if label.is_deferred() {
      BranchHint::False
    } else {
      BranchHint::None
    };
    self.goto_if(condition, label, hint, vars)
  }

  /// GotoIfNot 默认 hint: 目标 deferred → kTrue
  /// v8: src/compiler/graph-assembler.h GraphAssembler::GotoIfNot (无 hint 重载)
  pub fn goto_if_not_default(
    &mut self,
    condition: NodeRef<'z>,
    label: &mut GraphAssemblerLabel<'z>,
    vars: &[NodeRef<'z>],
  ) -> crate::Result<()> {
    let hint = if label.is_deferred() {
      BranchHint::True
    } else {
      BranchHint::None
    };
    self.goto_if_not(condition, label, hint, vars)
  }

  /// v8: src/compiler/graph-assembler.h GraphAssembler::MachineBranch
  pub fn machine_branch(
    &mut self,
    condition: NodeRef<'z>,
    if_true: &mut GraphAssemblerLabel<'z>,
    if_false: &mut GraphAssemblerLabel<'z>,
    hint: BranchHint,
    vars: &[NodeRef<'z>],
  ) -> crate::Result<()> {
    self.branch_impl(
      BranchSemantics::Machine,
      condition,
      if_true,
      if_false,
      hint,
      vars,
    )
  }

  /// v8: src/compiler/graph-assembler.h GraphAssembler::JSBranch
  pub fn js_branch(
    &mut self,
    condition: NodeRef<'z>,
    if_true: &mut GraphAssemblerLabel<'z>,
    if_false: &mut GraphAssemblerLabel<'z>,
    hint: BranchHint,
    vars: &[NodeRef<'z>],
  ) -> crate::Result<()> {
    self.branch_impl(
      BranchSemantics::JS,
      condition,
      if_true,
      if_false,
      hint,
      vars,
    )
  }

  /// ConnectUnreachableToEnd — Unreachable 连 End (MergeControlToEnd 推迟)
  /// v8: src/compiler/graph-assembler.cc GraphAssembler::ConnectUnreachableToEnd
  pub fn connect_unreachable_to_end(&mut self) -> crate::Result<()> {
    todo!() // 推迟台账: NodeProperties::MergeControlToEnd (v8-compiler-graph)
  }

  /// LoopScope — RAII 循环域 (ControlScope 机制未接入)
  /// v8: src/compiler/graph-assembler.h GraphAssembler::LoopScope
  pub fn loop_scope(&mut self) -> crate::Result<LoopScope<'_, 'z, 'a>> {
    todo!() // 推迟台账: ControlScope 族 (graph-assembler.cc 深水面)
  }
}

/// 循环域 RAII (进入 ++loop_nesting_level, 退出回退)
/// v8: src/compiler/graph-assembler.h GraphAssembler::LoopScope
pub struct LoopScope<'g, 'z, 'a> {
  /// gasm_
  pub gasm: &'g mut GraphAssembler<'g, 'z, 'a>,
}

/// JSGraphAssembler — JS 侧扩展: broker + 单例常量缓存
/// v8: src/compiler/graph-assembler.h JSGraphAssembler
pub struct JSGraphAssembler<'g, 'z, 'a> {
  /// 基类组装器
  pub assembler: GraphAssembler<'g, 'z, 'a>,
  /// broker_
  pub broker: &'g v8_compiler_broker::JSHeapBroker<'z, 'a>,
  /// outermost_catch_scope_ / catch_scope_ (CatchScope 推迟)
  pub _catch_scope_deferred: (),
}

impl<'g, 'z, 'a> JSGraphAssembler<'g, 'z, 'a> {
  /// 单例常量面 (JSGRAPH_SINGLETON_CONSTANT_LIST 23 项 + IF_WASM 2 项):
  /// allocate_in_old_generation_stub / allocate_in_young_generation_stub /
  /// big_int_map / boolean_map / empty_string / external_object_map / false_ /
  /// fixed_array_map / fixed_double_array_map / weak_fixed_array_map /
  /// context_cell_map / heap_number_map / minus_one / nan / no_context / null /
  /// one / the_hole / to_number_builtin / plain_primitive_to_number_builtin /
  /// true_ / undefined / zero (+ wasm 分配桩 2 项)
  /// v8: src/compiler/graph-assembler.h JSGRAPH_SINGLETON_CONSTANT_LIST
  pub fn singleton_constant(&mut self, _name: &'static str) -> crate::Result<NodeId> {
    todo!() // 推迟台账: 根常量/CodeStub 接入 (v8-roots / v8-codegen)
  }
}

// —— 对数基准 ——
// LabelType 3 / 单例表 23+2 / cc 定义面 127 (纯面直译以上, 深水面台账登记)
