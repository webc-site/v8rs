//! 实验性正则编译器: RegExpTree → experimental bytecode
//! (v8/src/regexp/experimental/experimental-compiler.h + .cc 1:1 转写)。
//!
//! 公开入口 (v8: experimental-compiler.h:24,28 `ExperimentalCompiler` 两个 static):
//! - [`can_be_handled`] ← h:24 `CanBeHandled(Tree*, Flags, int)`
//! - [`compile`]        ← h:28 `Compile(Tree*, Flags, Zone*)`
//!
//! 签名映射登记 (原版 C++ ↔ Rust, 语义不变):
//! - `Zone*` 形参删除 — 原版 zone 裸分配容器 (ZoneList/ZoneLinkedList/ZoneMap,
//!   cc:257,530,612-614) 在 Rust 侧为栈上所有权容器 (Vec/VecDeque/RemapMap),
//!   无分配器边界; `Compile` 返回 `ZoneList<Instruction>` (cc:1219-1222) ↔
//!   `Vec<Instruction>`。
//! - 全局 `v8_flags.` 访问 (cc:187,599,619,1030,1046,1179-1180) → 显式只读形参
//!   `&V8Flags` (v8-ast `with_flags` 同款注入先例; 无全局可变状态, 可测两条 flag 路径)。
//! - `CanBeHandled` 的 `capture_count` 形参原版声明但体内从不读取 (cc:30-35), 1:1 保留。
//! - Visitor `void* VisitX(node, void*)` (C++ 虚函数) ↔ `v8_regexp_syntax::ast::Visitor`
//!   trait, `Output = ()`; 返回值原版全部为 nullptr, 从不消费。
//! - AST 裸指针队列 `ZoneLinkedList<Lookaround*>` (cc:1197) ↔ `VecDeque<*const
//!   Lookaround>`; filter 队列 `Tree*` (cc:525) ↔ [`BFNode`] 三态裸指针枚举 (仅
//!   Quantifier/Capture/Lookaround 会被入队, 与原版 `Accept` 二次派发语义全等)。
//!   两队列指针均源自本次编译调用持有的 `&Tree` (调用区间内 AST 稳定), 解引用点
//!   带 SAFETY 注记 — 与原版 zone 裸指针同一存活不变量的 Rust 承接。
//! - `Label` union (cc:247-250) ↔ `LabelState` 枚举; "MUST be bound before scope end"
//!   (cc:224-225 注释) 由 Drop 上的 debug_assert 承接 (cc:231-234 DCHECK_EQ(BOUND))。
//! - `CharacterRanges` 原版就地改写 AST 区间表 (cc:741 Canonicalize 直接作用于
//!   node->ranges()); Rust 侧 AST 为不可变共享引用, 改为 clone 后规范化 —
//!   产码逐字相同, 仅"编译动作不再回写 AST"这一无消费方的副作用消失。
//! - 金标准/语义锚: 原版 v8 侧无 experimental-compiler 专有用例
//!   (实测 `v8/test/unittests/regexp/` 与 cctest 0 命中, 引擎经 mjsunit
//!   regexp-experimental.js 间接消费), 测试按源码语义逐行推演手工构造向量。

use std::{collections::VecDeque, ptr::from_ref};

use v8_flags::Flags as V8Flags;
use v8_regexp_syntax::{
  Alternative, Assertion, AssertionType, Atom, BackReference, Capture, CharacterRange, ClassRanges,
  ClassSetExpression, ClassSetOperand, Disjunction, Empty, Group, Interval, K_INFINITY, Lookaround,
  LookaroundType, Quantifier, QuantifierType, RegExpFlags, StackLimiter, Text, Tree, Visitor,
  char_range::K_MAX_CODE_POINT,
  flags_generated::{DOTALL, GLOBAL, LINEAR, MULTILINE, STICKY, UNICODE},
};

use super::bytecode::{Instruction, Opcode, Payload};

// ── 常量 (v8: experimental-compiler.cc:20-25, 匿名 namespace) ────────────────

/// UTF-16 支持上限 (v8: cc:22 `constexpr base::uc32 kMaxSupportedCodepoint = 0xFFFFu`;
/// TODO(mbid, v8:10765) 原版尚未支持 UTF-16 码点)
const K_MAX_SUPPORTED_CODEPOINT: u32 = 0xFFFF;
// 原版 static_assert(kMaxSupportedCodepoint <= uc16::max) (cc:765-766) 承接:
const _: () = assert!(K_MAX_SUPPORTED_CODEPOINT <= u16::MAX as u32);
// 原版 #ifdef DEBUG kMaxCodePoint = 0x10ffff (cc:24) — Rust 复用 syntax crate
// 常量 K_MAX_CODE_POINT, debug_assert 通道使用, 不重复定义。

/// 原版量词复制因子预算 (v8: cc:106 `kMaxReplicationFactor = 16`)
const K_MAX_REPLICATION_FACTOR: i32 = 16;

/// 锚定探测递归预算 (v8: regexp-nodes.h:345 `Node::kRecursionBudget = 200`,
/// 调用点 cc:550,634,637; Node 属 v8-regexp-compiler 面, 常量随调用点落地于此)
const K_RECURSION_BUDGET: i32 = 200;

/// 原版 StackLimiter(kMaxInt) (cc:466,501,1044); kMaxInt = INT_MAX
const K_MAX_INT: i32 = i32::MAX;

// ── CanBeHandledVisitor (v8: cc:27-213 匿名 namespace) ──────────────────────

/// 可行性检查访问器 (v8: cc:27 `class CanBeHandledVisitor final : private Visitor`)
struct CanBeHandledVisitor<'f> {
  /// 复制因子 (v8: cc:209 `replication_factor_ = 1`)
  replication_factor: i32,
  /// 累积结果 (v8: cc:211 `result_ = true`)
  result: bool,
  /// v8: cc:212 `flags_` + cc:206 `Flags flags() const` 访问器 (调用点 cc:162,180);
  /// Rust 侧字段直读, 不单设 getter
  flags: RegExpFlags,
  /// 原版读全局 v8_flags (cc:187); Rust 显式注入
  v8_flags: &'f V8Flags,
}

/// 允许的标志白名单 (v8: cc:43-45 `kAllowedFlags = kGlobal | kSticky | kMultiline |
/// kDotAll | kLinear`); 不含 kUnicode ⟹ 原版 static_assert (cc:47-48)
/// `kSupportsUnicode == IsUnicode(kAllowedFlags)` 两侧同为 false
/// (experimental.h:65 `kSupportsUnicode = false`)
const K_ALLOWED_FLAGS: RegExpFlags = RegExpFlags(GLOBAL | STICKY | MULTILINE | DOTALL | LINEAR);

// 原版 static_assert (cc:47-48) 承接: 白名单不含 u/v, 与 kSupportsUnicode=false 一致
const _: () = assert!(K_ALLOWED_FLAGS.0 & UNICODE == 0);

impl CanBeHandledVisitor<'_> {
  /// v8: cc:30-35 `static bool Check(Tree*, Flags, int)` — capture_count 在
  /// 原版函数体中从未被引用 (逐行核对), 故此处不接收
  fn check(tree: &Tree, flags: RegExpFlags, v8_flags: &V8Flags) -> bool {
    if !Self::are_suitable_flags(flags) {
      return false;
    }
    // cc:38 `explicit CanBeHandledVisitor(Flags flags) : flags_(flags)` + cc:209-212
    // 成员初值 (replication_factor_ = 1, result_ = true); Rust 侧聚合体直构
    let mut visitor = CanBeHandledVisitor {
      replication_factor: 1,
      result: true,
      flags,
      v8_flags,
    };
    tree.accept(&mut visitor);
    visitor.result
  }

  /// v8: cc:40-50 `static bool AreSuitableFlags(Flags)`
  fn are_suitable_flags(flags: RegExpFlags) -> bool {
    // 原版 `(flags & ~kAllowedFlags) == 0`; Rust Not 按 NUM_FLAGS 位掩码取反
    // (flags.rs:163-169), 对合法 flag 域与 C++ u16 全位取反结果一致
    (flags & !K_ALLOWED_FLAGS) == RegExpFlags::NONE
  }
}

impl Visitor for CanBeHandledVisitor<'_> {
  type Output = ();

  /// v8: cc:52-60 VisitDisjunction — 逐分支递归, 失败即停
  fn visit_disjunction(&mut self, node: &Disjunction) -> Self::Output {
    for alt in &node.alternatives {
      alt.accept(self);
      if !self.result {
        return;
      }
    }
  }

  /// v8: cc:62-70 VisitAlternative
  fn visit_alternative(&mut self, node: &Alternative) -> Self::Output {
    for child in &node.nodes {
      child.accept(self);
      if !self.result {
        return;
      }
    }
  }

  /// v8: cc:72 VisitClassRanges — 恒可
  fn visit_class_ranges(&mut self, _node: &ClassRanges) -> Self::Output {}

  /// v8: cc:74-78 VisitClassSetOperand — 含字符串 (/v sets) 即拒
  fn visit_class_set_operand(&mut self, node: &ClassSetOperand) -> Self::Output {
    self.result = !node.has_strings();
  }

  /// v8: cc:79-83 VisitClassSetExpression — 类运算恒拒
  fn visit_class_set_expression(&mut self, _node: &ClassSetExpression) -> Self::Output {
    self.result = false;
  }

  /// v8: cc:84 VisitAssertion — 恒可
  fn visit_assertion(&mut self, _node: &Assertion) -> Self::Output {}

  /// v8: cc:86 VisitAtom — 恒可
  fn visit_atom(&mut self, _node: &Atom) -> Self::Output {}

  /// v8: cc:88-96 VisitText
  fn visit_text(&mut self, node: &Text) -> Self::Output {
    for el in &node.elements {
      el.tree.accept(self);
      if !self.result {
        return;
      }
    }
  }

  /// v8: cc:98-154 VisitQuantifier — 复制因子预算 + POSSESSIVE 拒绝
  fn visit_quantifier(&mut self, node: &Quantifier) -> Self::Output {
    // 先排除单节点即超预算的 min/max (cc:111-116), 同时护住后续乘法不溢出
    if node.min > K_MAX_REPLICATION_FACTOR
      || (node.max != K_INFINITY && node.max > K_MAX_REPLICATION_FACTOR)
    {
      self.result = false;
      return;
    }

    // cc:120 保存现场 (result_ 保持 true 时恢复; 拒绝路径不恢复, 访问已终止)
    let before_replication_factor = self.replication_factor;

    // cc:122-132 local_replication
    let local_replication = if node.max == K_INFINITY {
      if node.min > 0 && node.min_match > 0 {
        // 可归约为非空 plus (cc:124-126)
        node.min.max(1)
      } else {
        node.min + 1
      }
    } else {
      node.max
    };

    // cc:134-138: 上界守卫保证乘积 ≤ 16*17, 无溢出 (原版注释 cc:108-111 同义)
    self.replication_factor *= local_replication;
    if self.replication_factor > K_MAX_REPLICATION_FACTOR {
      self.result = false;
      return;
    }

    // cc:140-149 switch: POSSESSIVE 拒绝 (TODO(mbid,v8:10765) 广度优先下未定支持)
    match node.quantifier_type {
      QuantifierType::Greedy | QuantifierType::NonGreedy => {}
      QuantifierType::Possessive => {
        self.result = false;
        return;
      }
    }

    node.body.accept(self);
    self.replication_factor = before_replication_factor;
  }

  /// v8: cc:156-159 VisitCapture
  fn visit_capture(&mut self, node: &Capture) -> Self::Output {
    node.body.accept(self);
  }

  /// v8: cc:161-177 VisitGroup — modifier 标志须全在白名单内
  fn visit_group(&mut self, node: &Group) -> Self::Output {
    if self.flags != node.flags {
      // 顶层不支持的标志, 经 modifier 同样不支持 (cc:162-169 TODO(pthier))
      if !Self::are_suitable_flags(node.flags) {
        self.result = false;
        return;
      }
    }
    node.body.accept(self);
  }

  /// v8: cc:179-195 VisitLookaround
  fn visit_lookaround(&mut self, node: &Lookaround) -> Self::Output {
    // 环视与 global/sticky 冲突: 续搜起点非 0, lookbehind 须从串首跑 (cc:583-592 注释)
    if self.flags.is_global() || self.flags.sticky() {
      self.result = false;
      return;
    }
    // capture_group_opt 关: 拒绝全部 lookahead 与含捕获的 lookbehind (cc:185-191)
    if !self.v8_flags.experimental_regexp_engine_capture_group_opt()
      && (node.lookaround_type == LookaroundType::Lookahead || node.capture_count > 0)
    {
      self.result = false;
      return;
    }
    node.body.accept(self);
  }

  /// v8: cc:197-201 VisitBackReference — 无回溯不可实现, 恒拒
  fn visit_back_reference(&mut self, _node: &BackReference) -> Self::Output {
    self.result = false;
  }

  /// v8: cc:203 VisitEmpty — 恒可
  fn visit_empty(&mut self, _node: &Empty) -> Self::Output {}
}

/// 判定 tree 能否编入实验引擎 (v8: experimental-compiler.h:24 + cc:217-220
/// `ExperimentalCompiler::CanBeHandled`)
///
/// `capture_count` 形参为原版签名 1:1 保留 (h:24); `Check` 体 (cc:30-35) 从不读取。
#[must_use]
pub fn can_be_handled(
  tree: &Tree,
  flags: RegExpFlags,
  capture_count: i32,
  v8_flags: &V8Flags,
) -> bool {
  // v8: cc:219 直接转发 CanBeHandledVisitor::Check (capture_count 原版即透传不用)
  let _ = capture_count;
  CanBeHandledVisitor::check(tree, flags, v8_flags)
}

// ── Label (v8: cc:222-251 匿名 namespace) ───────────────────────────────────

/// 标签状态: 未绑定 ⇒ 补丁链头; 已绑定 ⇒ 落点下标
/// (v8: cc:244-250 `enum { UNBOUND, BOUND }` + union{unbound_patch_list_begin_, bound_index_})
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum LabelState {
  /// 未绑定; 值为 FORK/JMP/FILTER_CHILD 补丁链头 (经 payload.pc 串链), -1 为空
  Unbound { patch_list_begin: i32 },
  /// 已绑定 (绑定后只读)
  Bound { index: i32 },
}

/// 字节码标签 (v8: cc:228-251 `struct Label`) — 出作用域前必须 Bind
/// (cc:224-226 注释 "The address *must* be bound with `Bind`"; cc:231-234 析构 DCHECK)。
/// 原版禁拷贝禁移动 (cc:238-239); Rust 侧全部经 `&mut` 传递, 值不复制。
struct Label {
  state: LabelState,
}

impl Label {
  /// v8: cc:230 `Label() = default` — UNBOUND + patch_list_begin_ = -1
  fn new() -> Self {
    Self {
      state: LabelState::Unbound {
        patch_list_begin: -1,
      },
    }
  }
}

impl Drop for Label {
  /// v8: cc:231-234 `~Label()` DCHECK_EQ(state_, BOUND); DCHECK_GE(bound_index_, 0)
  fn drop(&mut self) {
    debug_assert!(
      matches!(self.state, LabelState::Bound { index } if index >= 0),
      "v8 cc:231-234: Label 出作用域前必须 Bind"
    );
  }
}

// ── BytecodeAssembler (v8: cc:253-376) ──────────────────────────────────────

/// 指令流装配器 (v8: cc:253 `class BytecodeAssembler`)
/// 原版持 (Zone*, ZoneList<Instruction> code_(0, zone)); Rust 侧 Vec 自增容,
/// zone 形参按模块头登记删除 (TODO(mbid,v8:10765) 容量上界估算 cc:255 原版亦未实现)。
struct BytecodeAssembler {
  /// v8: cc:375 `code_`
  code: Vec<Instruction>,
}

impl BytecodeAssembler {
  /// 产出指令流 (v8: cc:259 `ZoneList<Instruction> IntoCode() &&`)
  fn into_code(self) -> Vec<Instruction> {
    self.code
  }

  /// v8: cc:261 `Accept()`
  fn accept(&mut self) {
    self.code.push(Instruction::accept());
  }

  /// v8: cc:263-265 `Assertion(Assertion::Type)`
  fn assertion(&mut self, t: AssertionType) {
    self.code.push(Instruction::assertion(t));
  }

  /// v8: cc:267-269 `ClearRegister(int32_t)`
  fn clear_register(&mut self, register_index: i32) {
    self.code.push(Instruction::clear_register(register_index));
  }

  /// v8: cc:271-273 `ConsumeRange(base::uc16, base::uc16)`
  fn consume_range(&mut self, from: u16, to: u16) {
    self.code.push(Instruction::consume_range(from, to));
  }

  /// v8: cc:275 `ConsumeAnyChar()`
  fn consume_any_char(&mut self) {
    self.code.push(Instruction::consume_any_char());
  }

  /// v8: cc:277-279 `RangeCount(int32_t)`
  fn range_count(&mut self, num_ranges: i32) {
    self.code.push(Instruction::range_count(num_ranges));
  }

  /// v8: cc:281-283 `Fork(Label&)`
  fn fork(&mut self, target: &mut Label) {
    self.labelled_instr(Opcode::Fork, target);
  }

  /// v8: cc:285-287 `Jmp(Label&)`
  fn jmp(&mut self, target: &mut Label) {
    self.labelled_instr(Opcode::Jmp, target);
  }

  /// v8: cc:289-291 `SetRegisterToCp(int32_t)`
  fn set_register_to_cp(&mut self, register_index: i32) {
    self
      .code
      .push(Instruction::set_register_to_cp(register_index));
  }

  /// v8: cc:293 `BeginLoop()`
  fn begin_loop(&mut self) {
    self.code.push(Instruction::begin_loop());
  }

  /// v8: cc:295 `EndLoop()`
  fn end_loop(&mut self) {
    self.code.push(Instruction::end_loop());
  }

  /// v8: cc:297-301 `StartLookaround(int, bool, Lookaround::Type)`
  fn start_lookaround(&mut self, lookaround_index: i32, is_positive: bool, type_: LookaroundType) {
    self.code.push(Instruction::start_lookaround(
      lookaround_index,
      is_positive,
      type_,
    ));
  }

  /// v8: cc:303 `EndLookaround()`
  fn end_lookaround(&mut self) {
    self.code.push(Instruction::end_lookaround());
  }

  /// v8: cc:305-307 `WriteLookaroundTable(int)`
  fn write_lookaround_table(&mut self, index: i32) {
    self.code.push(Instruction::write_look_table(index));
  }

  /// v8: cc:309-311 `ReadLookaroundTable(int, bool, Lookaround::Type)`
  fn read_lookaround_table(&mut self, index: i32, is_positive: bool, type_: LookaroundType) {
    self
      .code
      .push(Instruction::read_look_table(index, is_positive, type_));
  }

  /// v8: cc:313-315 `SetQuantifierToClock(int32_t)`
  fn set_quantifier_to_clock(&mut self, quantifier_id: i32) {
    self
      .code
      .push(Instruction::set_quantifier_to_clock(quantifier_id));
  }

  /// v8: cc:317-319 `FilterQuantifier(int32_t)`
  fn filter_quantifier(&mut self, quantifier_id: i32) {
    self
      .code
      .push(Instruction::filter_quantifier(quantifier_id));
  }

  /// v8: cc:321-323 `FilterGroup(int32_t)`
  fn filter_group(&mut self, group_id: i32) {
    self.code.push(Instruction::filter_group(group_id));
  }

  /// v8: cc:325-327 `FilterLookaround(int32_t)`
  fn filter_lookaround(&mut self, lookaround_id: i32) {
    self
      .code
      .push(Instruction::filter_lookaround(lookaround_id));
  }

  /// v8: cc:329-331 `FilterChild(Label&)`
  fn filter_child(&mut self, target: &mut Label) {
    self.labelled_instr(Opcode::FilterChild, target);
  }

  /// v8: cc:333-350 `Bind(Label&)` — 回打补丁链并置 Bound
  fn bind(&mut self, target: &mut Label) {
    // DCHECK_EQ(target.state_, UNBOUND) (cc:334)
    let LabelState::Unbound {
      ref mut patch_list_begin,
    } = target.state
    else {
      // 100% 不可达: 全部 Bind 调用点传入新建或从未复用的标签 (原版 cc:334 同 DCHECK)
      unreachable!("v8 cc:334: Bind 只接受 UNBOUND 标签")
    };
    let index = self.code.len() as i32;
    let mut p = *patch_list_begin;
    while p != -1 {
      let inst = &mut self.code[p as usize];
      // DCHECK(inst.opcode == FORK || JMP || FILTER_CHILD) (cc:340-342)
      debug_assert!(matches!(
        inst.opcode,
        Opcode::Fork | Opcode::Jmp | Opcode::FilterChild
      ));
      // 原版读 payload.pc 再覆写 (cc:344-345); Rust tagged payload 同槽读写
      let Payload::Pc(next) = inst.payload else {
        // 100% 不可达: FORK/JMP/FILTER_CHILD 三 opcode 的 payload 由 bytecode.rs
        // 工厂静态配对为 Pc (h:172-184,241-246)
        unreachable!("v8 cc:344: 带标签指令 payload 必为 pc")
      };
      p = next;
      inst.payload = Payload::Pc(index);
    }
    target.state = LabelState::Bound { index };
  }

  /// v8: cc:352 `Fail()`
  fn fail(&mut self) {
    self.code.push(Instruction::fail());
  }

  /// v8: cc:355-372 `LabelledInstrImpl(Instruction::Opcode, Label&)`
  fn labelled_instr(&mut self, op: Opcode, target: &mut Label) {
    let pc = match target.state {
      LabelState::Bound { index } => index,
      LabelState::Unbound { patch_list_begin } => {
        let new_list_begin = self.code.len() as i32;
        // DCHECK_GE(new_list_begin, 0) (cc:364) — usize→i32 转换非负天然成立,
        // 指令流超 2^31 条在 V8 原版同样不可达 (zone 容量)
        debug_assert!(new_list_begin >= 0);
        target.state = LabelState::Unbound {
          patch_list_begin: new_list_begin,
        };
        patch_list_begin
      }
    };
    // 原版以 opcode 成员赋值 + payload.pc 直写 (cc:356-358,360,366);
    // Rust 经 bytecode.rs 工厂构造合法配对 (禁自造 IR — 复用约束)
    let inst = match op {
      Opcode::Fork => Instruction::fork(pc),
      Opcode::Jmp => Instruction::jmp(pc),
      Opcode::FilterChild => Instruction::filter_child(pc),
      _ => unreachable!("v8 cc:282,286,330: LabelledInstrImpl 仅 FORK/JMP/FILTER_CHILD"),
    };
    self.code.push(inst);
  }
}

// ── 重映射表 (v8: cc:1199-1200 ZoneMap<int,int>; 语义 = std::map try_emplace/at) ──

/// 稠密化重映射 (v8: cc:1178-1190 `RemapQuantifier`/`RemapLookaround` 的
/// `map.try_emplace(id, map.size())` 通道)。非语义绑定小表, Vec 线性查找
/// (引擎内区量/环视数量 ≤ 预算级, 无需哈希; Cargo 依赖预置无 gxhash, 按红线不加)
#[derive(Default)]
struct RemapMap {
  entries: Vec<(i32, i32)>,
}

impl RemapMap {
  /// `try_emplace(id, size()).first->second` — 无则按当前规模插入新号
  fn get_or_insert(&mut self, id: i32) -> i32 {
    if let Some(&(_, v)) = self.entries.iter().find(|&&(k, _)| k == id) {
      return v;
    }
    let v = self.entries.len() as i32;
    self.entries.push((id, v));
    v
  }

  /// `std::map::at` 语义 (v8: cc:460,497; 缺键原版 terminate) —
  /// 100% 不可达: filter 相入队的 id 全部产生自主编译相的 Remap* 插入
  fn at(&self, id: i32) -> i32 {
    self
      .entries
      .iter()
      .find(|&&(k, _)| k == id)
      .map(|&(_, v)| v)
      .unwrap_or(id)
  }
}

// ── FilterGroupsCompileVisitor (v8: cc:378-542) ─────────────────────────────

/// 队列元素节点三态 (v8: cc:521-525 `BFEntry{Label label; Tree* node}` 的 Tree*;
/// 只有 Quantifier/Capture/Lookaround 三种节点会被入队 — VisitQuantifier/Capture/
/// Lookaround 的排队分支, 二次派发 Accept 到对应 Visit*, Rust 以枚举承接)。
/// 裸指针承接原版 zone 裸指针 (同 CompileVisitor.lookarounds 的 SAFETY 不变量:
/// 指针源自本次编译持有的 &Tree, 存活期覆盖整个编译调用)。
enum BFNode {
  Quantifier(*const Quantifier),
  Capture(*const Capture),
  Lookaround(*const Lookaround),
}

/// 广度优先 filter 队列项 (v8: cc:519-525 `class BFEntry`)
struct BFEntry {
  /// v8: cc:523 `Label label;` (BFEntry 构造即默认新建, cc:521)
  label: Label,
  /// v8: cc:524 `Tree* node;`
  node: BFNode,
}

/// FILTER_* 指令编译器 (v8: cc:378 `class FilterGroupsCompileVisitor final : private Visitor`)
struct FilterGroupsCompileVisitor<'a> {
  /// v8: cc:529 `BytecodeAssembler& assembler_;`
  assembler: &'a mut BytecodeAssembler,
  /// v8: cc:530 `ZoneLinkedList<BFEntry> nodes_;` (emplace_back/front/pop_front → 队列)
  nodes: VecDeque<BFEntry>,
  /// v8: cc:535 `can_compile_node_;` 弹队后 true, 编译一个节点后 false
  can_compile_node: bool,
  /// v8: cc:537 `quantifier_id_remapping_;` (cc:1202 注释: 主编译相产物, 恒可用)
  quantifier_id_remapping: &'a RemapMap,
  /// v8: cc:541 `lookaround_id_remapping_;` 环视 id 稠密化 (解析期优化可留空洞)
  lookaround_id_remapping: &'a RemapMap,
}

impl FilterGroupsCompileVisitor<'_> {
  /// v8: cc:380-411 `static void CompileFilter(Zone*, Tree*, BytecodeAssembler&,
  /// ZoneMap&, ZoneMap&)` — cc:384-395 大注释: 用 FILTER_GROUP/FILTER_QUANTIFIER/
  /// FILTER_CHILD 编码只含组与量词的简化 AST, 广度优先逐节点编译并入队子节点。
  /// (zone 形参按模块头登记删除; 原版 private ctor cc:414-422 = assembler 引用 +
  ///  nodes_ 空表 + can_compile_node_ = false (cc:420) → Rust 聚合体直构)
  fn compile_filter(
    tree: &Tree,
    assembler: &mut BytecodeAssembler,
    quantifier_id_remapping: &RemapMap,
    lookaround_id_remapping: &RemapMap,
  ) {
    let mut visitor = FilterGroupsCompileVisitor {
      assembler,
      nodes: VecDeque::new(),
      can_compile_node: false,
      quantifier_id_remapping,
      lookaround_id_remapping,
    };

    tree.accept(&mut visitor);

    // cc:402-410: while 非空 → front Bind → can_compile_node_=true → node Accept →
    // pop_front。Rust 先 pop 持有 entry 再 Bind: entry 出队后其 label 无人再读,
    // 与原版在队列内改写 label 等价 (pop 先后不影响 BFS 序)
    while let Some(mut entry) = visitor.nodes.pop_front() {
      visitor.assembler.bind(&mut entry.label);
      visitor.can_compile_node = true;
      match entry.node {
        // SAFETY: BFNode 指针由 visit_* 排队分支从本次编译的 &Tree 借用产生
        // (visit 参数源自 compile_filter 的 tree), tree 由调用方持有且借用区间
        // 覆盖本函数全程 — 与原版 zone Tree* 队列 (cc:524) 同一存活不变量。
        BFNode::Quantifier(node) => visitor.visit_quantifier(unsafe { &*node }),
        BFNode::Capture(node) => visitor.visit_capture(unsafe { &*node }),
        BFNode::Lookaround(node) => visitor.visit_lookaround(unsafe { &*node }),
      }
    }
  }

  /// v8: cc:454-475 VisitQuantifier (filter AST 节点排队/编译)
  fn enqueue_child(&mut self, node: BFNode) {
    // 原版先 emplace_back(node) 再 FilterChild(nodes_.back().label) (cc:470-471);
    // Rust 先建 label→FilterChild→入队, 补丁链状态逐值相同 (label 仅这一个引用点)
    let mut label = Label::new();
    self.assembler.filter_child(&mut label);
    self.nodes.push_back(BFEntry { label, node });
  }
}

impl Visitor for FilterGroupsCompileVisitor<'_> {
  type Output = ();

  /// v8: cc:424-429 VisitDisjunction — 全分支直接递归 (无 result 短路)
  fn visit_disjunction(&mut self, node: &Disjunction) -> Self::Output {
    for alt in &node.alternatives {
      alt.accept(self);
    }
  }

  /// v8: cc:431-436 VisitAlternative
  fn visit_alternative(&mut self, node: &Alternative) -> Self::Output {
    for alt in &node.nodes {
      alt.accept(self);
    }
  }

  /// v8: cc:438 VisitClassRanges — 叶, 无 filter 语义
  fn visit_class_ranges(&mut self, _node: &ClassRanges) -> Self::Output {}

  /// v8: cc:440-442 VisitClassSetOperand
  fn visit_class_set_operand(&mut self, _node: &ClassSetOperand) -> Self::Output {}

  /// v8: cc:444-446 VisitClassSetExpression
  fn visit_class_set_expression(&mut self, _node: &ClassSetExpression) -> Self::Output {}

  /// v8: cc:448 VisitAssertion
  fn visit_assertion(&mut self, _node: &Assertion) -> Self::Output {}

  /// v8: cc:450 VisitAtom
  fn visit_atom(&mut self, _node: &Atom) -> Self::Output {}

  /// v8: cc:452 VisitText — filter 相不下钻文本 (主编译相已消费)
  fn visit_text(&mut self, _node: &Text) -> Self::Output {}

  /// v8: cc:454-475 VisitQuantifier
  fn visit_quantifier(&mut self, node: &Quantifier) -> Self::Output {
    if node.max == 0 {
      return;
    }

    if self.can_compile_node {
      self
        .assembler
        .filter_quantifier(self.quantifier_id_remapping.at(node.index));
      self.can_compile_node = false;
      node.body.accept(self);
    } else {
      // TODO(v8:10765) 原版 cc:464-465: kMaxInt 深度传递, 待改真实栈溢出处理
      // node->CaptureRegisters == body 递归 (regexp-ast.cc Tree::CaptureRegisters)
      if node
        .body
        .capture_registers(StackLimiter::new(K_MAX_INT))
        .is_empty()
      {
        return;
      }
      self.enqueue_child(BFNode::Quantifier(from_ref(node)));
    }
  }

  /// v8: cc:477-488 VisitCapture
  fn visit_capture(&mut self, node: &Capture) -> Self::Output {
    if self.can_compile_node {
      self.assembler.filter_group(node.index);
      self.can_compile_node = false;
      node.body.accept(self);
    } else {
      self.enqueue_child(BFNode::Capture(from_ref(node)));
    }
  }

  /// v8: cc:490-493 VisitGroup — 直接下钻
  fn visit_group(&mut self, node: &Group) -> Self::Output {
    node.body.accept(self);
  }

  /// v8: cc:495-510 VisitLookaround
  fn visit_lookaround(&mut self, node: &Lookaround) -> Self::Output {
    if self.can_compile_node {
      self
        .assembler
        .filter_lookaround(self.lookaround_id_remapping.at(node.index));
      self.can_compile_node = false;
      node.body.accept(self);
    } else {
      if node
        .body
        .capture_registers(StackLimiter::new(K_MAX_INT))
        .is_empty()
      {
        return;
      }
      self.enqueue_child(BFNode::Lookaround(from_ref(node)));
    }
  }

  /// v8: cc:512-514 VisitBackReference — CanBeHandled 已拒, filter 相亦空转
  fn visit_back_reference(&mut self, _node: &BackReference) -> Self::Output {}

  /// v8: cc:516 VisitEmpty
  fn visit_empty(&mut self, _node: &Empty) -> Self::Output {}
}

// ── CompileVisitor (v8: cc:544-1215) ────────────────────────────────────────

/// RegExpTree → 字节码主编译访问器
/// (v8: cc:544 `class CompileVisitor : private Visitor`; 逐字段映射见下)
struct CompileVisitor<'f> {
  // v8: cc:1193 `Zone* zone_;` — 模块头登记删除 (Rust 所有权容器无分配器边界)
  /// 主编译相遇到的环视队列 (v8: cc:1197 `ZoneLinkedList<Lookaround*> lookarounds_;`,
  /// 主表达式编译完后 BFS 出队逐个 CompileLookaround)。
  /// 原版存 zone 裸指针, Rust 用 `*const Lookaround` 同构承接: 指针源自入队时
  /// `tree.accept` 借用链上的节点, `compile` 调用区间内 tree 全程存活 (解引用点有 SAFETY 注记)。
  lookarounds: VecDeque<*const Lookaround>,
  /// v8: cc:1199 `std::optional<ZoneMap<int,int>> quantifier_id_remapping_;`
  /// (仅 capture_group_opt 开时 Some, cc:619-621)
  quantifier_id_remapping: Option<RemapMap>,
  /// v8: cc:1200 `ZoneMap<int,int> lookaround_id_remapping_;`
  lookaround_id_remapping: RemapMap,
  /// v8: cc:1202 `BytecodeAssembler assembler_;`
  assembler: BytecodeAssembler,
  /// v8: cc:1203 `reverse_;` 反相编译 (lookahead 的匹配自动机整体倒放)
  reverse: bool,
  /// v8: cc:1208 `ignore_captures_;` 不产 SET_REGISTER_TO_CP (匹配自动机捕获无人消费)
  ignore_captures: bool,
  /// v8: cc:1214 `ignore_lookarounds_;` 不再入队环视 (捕获自动机相, 子环视已入过队)
  ignore_lookarounds: bool,
  /// 原版读全局 v8_flags (cc:599,619,1030,1046,1179); Rust 显式注入
  v8_flags: &'f V8Flags,
}

impl<'f> CompileVisitor<'f> {
  /// v8: cc:610-622 `explicit CompileVisitor(Zone*)` — reverse/ignore_* 初值 false,
  /// capture_group_opt 开时 emplace 量词重映射表
  fn new(v8_flags: &'f V8Flags) -> Self {
    let mut quantifier_id_remapping = None;
    if v8_flags.experimental_regexp_engine_capture_group_opt() {
      quantifier_id_remapping = Some(RemapMap::default());
    }
    Self {
      lookarounds: VecDeque::new(),
      quantifier_id_remapping,
      lookaround_id_remapping: RemapMap::default(),
      assembler: BytecodeAssembler { code: Vec::new() },
      reverse: false,
      ignore_captures: false,
      ignore_lookarounds: false,
      v8_flags,
    }
  }

  /// 编译入口 (v8: cc:546-607 `static ZoneList<Instruction> Compile(Tree*, Flags, Zone*)`)
  fn compile(tree: &Tree, flags: RegExpFlags, v8_flags: &'f V8Flags) -> Vec<Instruction> {
    let mut compiler = Self::new(v8_flags);

    if !flags.sticky() && !tree.is_certainly_anchored_at_start(K_RECURSION_BUDGET) {
      // cc:551-555: 非锚定匹配可在任意位置起 → 前导 /.*?/ (非贪婪星跳过任意前缀)
      compiler.compile_non_greedy_star(&mut |slf: &mut Self| slf.assembler.consume_any_char());
    }

    // cc:558-561: 捕获寄存器 0=串首, 1=串尾
    compiler.assembler.set_register_to_cp(0);
    tree.accept(&mut compiler);
    compiler.assembler.set_register_to_cp(1);
    compiler.assembler.accept();

    // cc:563-597 大注释 (环视两相编译, 保留原文语义摘要): 每个环视编译成两段字节码 —
    // 1) 无捕获匹配自动机 (lookahead 反相) 2) 捕获自动机 (lookbehind 反相, 无捕获可空)。
    // 主表达式编译中环视不即时编译, 入队延后 BFS 出队; 段落由 START/END_LOOKAROUND
    // 界分, 切分点在 WRITE_LOOKAROUND_TABLE 之后。此路线排除 global/sticky 与环视共存
    // (续搜起点非 0, lookbehind 须从串首起跑)。
    while let Some(node) = compiler.lookarounds.pop_front() {
      // SAFETY: 指针源自本函数 `tree.accept` 派发产生的节点引用 (原版 cc:1197
      // zone 裸指针队列同构), tree 在本调用区间内全程存活, 出队时节点必有效。
      compiler.compile_lookaround(unsafe { &*node });
    }

    if v8_flags.experimental_regexp_engine_capture_group_opt() {
      // cc:599-604: filter 指令相 (读 remap 表, 主编译相已填满)。
      // 原版 cc:553 同传 assembler_ 与两张 remap 表: 三个借用指向互不相交字段,
      // Rust 侧按字段拆分同时借用。
      let empty_map = RemapMap::default();
      FilterGroupsCompileVisitor::compile_filter(
        tree,
        &mut compiler.assembler,
        compiler
          .quantifier_id_remapping
          .as_ref()
          .unwrap_or(&empty_map),
        &compiler.lookaround_id_remapping,
      );
    }

    // cc:606 `return std::move(compiler.assembler_).IntoCode();`
    compiler.assembler.into_code()
  }

  /// v8: cc:625-659 `CompileLookaround(Lookaround*)` — 两相字节码生成
  fn compile_lookaround(&mut self, lookaround: &Lookaround) {
    // cc:627-628 第一段: START_LOOKAROUND (id 稠密化两次 Remap 同值, cc:1186-1190)
    let remapped = self.remap_lookaround(lookaround.index);
    self
      .assembler
      .start_lookaround(remapped, lookaround.is_positive, lookaround.lookaround_type);

    // cc:632-639: 体不锚定 → 段首加 /.*?/ 使自动机跑遍整个输入
    // (lookahead 查"锚定串尾", lookbehind 查"锚定串首")
    if (lookaround.lookaround_type == LookaroundType::Lookahead
      && !lookaround
        .body
        .is_certainly_anchored_at_end(K_RECURSION_BUDGET))
      || (lookaround.lookaround_type == LookaroundType::Lookbehind
        && !lookaround
          .body
          .is_certainly_anchored_at_start(K_RECURSION_BUDGET))
    {
      self.compile_non_greedy_star(&mut |slf: &mut Self| slf.assembler.consume_any_char());
    }

    // cc:641-645: 匹配自动机 — lookahead 反相; 全程忽略捕获寄存器
    self.reverse = lookaround.lookaround_type == LookaroundType::Lookahead;
    self.ignore_captures = true;
    lookaround.body.accept(self);
    self.ignore_captures = false;

    // cc:647: 写环视结果表 (id 与 cc:627 同值 — Remap 幂等)
    let remapped = self.remap_lookaround(lookaround.index);
    self.assembler.write_lookaround_table(remapped);

    // cc:649-656: 第二段捕获自动机 — 仅正环视且含捕获时生成; lookbehind 反相
    if lookaround.capture_count > 0 && lookaround.is_positive {
      self.reverse = lookaround.lookaround_type == LookaroundType::Lookbehind;
      // 子环视已在匹配自动机相入过队, 本相不再入队
      self.ignore_lookarounds = true;
      lookaround.body.accept(self);
      self.ignore_lookarounds = false;
    }

    self.assembler.end_lookaround();
  }

  /// v8: cc:661-708 `template <class F> CompileDisjunction(int alt_num, F&& gen_alt)` —
  /// cc:667-687 大注释: a1|…|an 编为 FORK tail1 <a1> JMP end / tail1: FORK tail2 …
  /// 最后分支直落; FORK 线程优先级低于派生线程 ⟹ a1 优先级最高 (回溯语义一致)。
  /// gen_alt(i) 生成第 i 分支; Rust 以 `FnMut(&mut Self, i32)` 承接 C++ lambda 捕获 this。
  fn compile_disjunction(&mut self, alt_num: i32, gen_alt: &mut impl FnMut(&mut Self, i32)) {
    if alt_num == 0 {
      // cc:689-693: 空析取恒不匹配
      self.assembler.fail();
      return;
    }

    let mut end = Label::new();

    for i in 0..alt_num - 1 {
      let mut tail = Label::new();
      self.assembler.fork(&mut tail);
      gen_alt(self, i);
      self.assembler.jmp(&mut end);
      self.assembler.bind(&mut tail);
    }

    gen_alt(self, alt_num - 1);

    self.assembler.bind(&mut end);
  }

  /// v8: cc:736-789 `CompileCharacterRanges(ZoneList<CharacterRange>*, bool negated)` —
  /// cc:737-740 注释: 字符类编为互不相交区间; 单区间 = 1 条 CONSUME_RANGE,
  /// 多区间 = RANGE_COUNT + n 条 CONSUME_RANGE。
  /// 原版就地 Canonicalize 改写 AST 区间表; Rust 侧 AST 共享不可变, 入参为调用点
  /// clone 副本 (产码逐字相同, 见模块头登记)。
  fn compile_character_ranges(&mut self, mut ranges: Vec<CharacterRange>, negated: bool) {
    CharacterRange::canonicalize(&mut ranges);
    if negated {
      // cc:742-749: k 区间规范化并集的补集 ≤ k+1 区间
      let mut negated_ranges = Vec::with_capacity(ranges.len() + 1);
      CharacterRange::negate(&ranges, &mut negated_ranges);
      debug_assert!(negated_ranges.len() <= ranges.len() + 1);
      ranges = negated_ranges;
    }

    if ranges.is_empty() {
      // cc:752-755: 空类恒不匹配
      self.assembler.fail();
      return;
    }

    if ranges.len() > 1 {
      // cc:757-760: 多区间需要 RANGE_COUNT
      self.assembler.range_count(ranges.len() as i32);
    }

    for r in &ranges {
      // cc:762-766: 仅支持 uc16 边界可达的码点域 (static_assert 已在常量区承接)
      let from = r.from;
      let to = r.to;

      // cc:770-777: [^\x00-\uFFFF] 特例 — 非 unicode 模式下无字符可匹配, 恒失败
      if from > K_MAX_SUPPORTED_CODEPOINT && to > K_MAX_SUPPORTED_CODEPOINT {
        debug_assert!(negated);
        self.assembler.fail();
        return;
      }

      debug_assert!(from <= K_MAX_SUPPORTED_CODEPOINT);
      let from_uc16 = from as u16;

      // DCHECK_IMPLIES(to > kMaxSupportedCodepoint, to == kMaxCodePoint) (cc:783)
      debug_assert!(to <= K_MAX_SUPPORTED_CODEPOINT || to == K_MAX_CODE_POINT);
      // cc:784-785: std::min(to, kMaxSupportedCodepoint) 截断为 uc16
      let to_uc16 = to.min(K_MAX_SUPPORTED_CODEPOINT) as u16;

      self.assembler.consume_range(from_uc16, to_uc16);
    }
  }

  /// v8: cc:822-832 `ClearRegisters(Interval)`
  fn clear_registers(&mut self, indices: Interval) {
    if indices.is_empty() {
      return;
    }
    // cc:824-825 DCHECK: from 偶 / to 奇 (捕获寄存器成对)
    debug_assert_eq!(indices.from() % 2, 0);
    debug_assert_eq!(indices.to() % 2, 1);
    // cc:826-831: 只需清 begin 寄存器 (begin 未定义 ⟹ 捕获未定义, 不看 end)
    for i in (indices.from()..=indices.to()).step_by(2) {
      self.assembler.clear_register(i);
    }
  }

  /// v8: cc:834-861 `template <class F> CompileGreedyStar(F&&)` — cc:837-849 注释:
  /// begin: FORK end; BEGIN_LOOP; <body>; END_LOOP; JMP begin; end:
  /// (FORK 线程优先级低 ⟹ 贪婪)
  fn compile_greedy_star(&mut self, emit_body: &mut impl FnMut(&mut Self)) {
    let mut begin = Label::new();
    let mut end = Label::new();

    self.assembler.bind(&mut begin);
    self.assembler.fork(&mut end);
    self.assembler.begin_loop();
    emit_body(self);
    self.assembler.end_loop();
    self.assembler.jmp(&mut begin);

    self.assembler.bind(&mut end);
  }

  /// v8: cc:863-891 `template <class F> CompileNonGreedyStar(F&&)` — cc:864-876 注释:
  /// FORK body; JMP end; body: BEGIN_LOOP; <body>; END_LOOP; FORK body; end:
  fn compile_non_greedy_star(&mut self, emit_body: &mut impl FnMut(&mut Self)) {
    let mut body = Label::new();
    let mut end = Label::new();

    self.assembler.fork(&mut body);
    self.assembler.jmp(&mut end);

    self.assembler.bind(&mut body);
    self.assembler.begin_loop();
    emit_body(self);
    self.assembler.end_loop();
    self.assembler.fork(&mut body);

    self.assembler.bind(&mut end);
  }

  /// v8: cc:893-924 `CompileGreedyRepetition(emit_body, max_repetition_num)` —
  /// max_repetition_num 次 {FORK end; BEGIN_LOOP; <body>; END_LOOP} 后 Bind end。
  /// cc:913-914 注释: BEGIN/END_LOOP 因可选重复的体不可匹配空串
  fn compile_greedy_repetition(
    &mut self,
    emit_body: &mut impl FnMut(&mut Self),
    max_repetition_num: i32,
  ) {
    let mut end = Label::new();
    for _ in 0..max_repetition_num {
      self.assembler.fork(&mut end);
      self.assembler.begin_loop();
      emit_body(self);
      self.assembler.end_loop();
    }
    self.assembler.bind(&mut end);
  }

  /// v8: cc:926-968 `CompileNonGreedyRepetition(emit_body, max_repetition_num)` —
  /// max_repetition_num 次 {FORK body_i; JMP end; body_i: BEGIN_LOOP; <body>; END_LOOP}
  fn compile_non_greedy_repetition(
    &mut self,
    emit_body: &mut impl FnMut(&mut Self),
    max_repetition_num: i32,
  ) {
    let mut end = Label::new();
    for _ in 0..max_repetition_num {
      let mut body = Label::new();
      self.assembler.fork(&mut body);
      self.assembler.jmp(&mut end);

      self.assembler.bind(&mut body);
      self.assembler.begin_loop();
      emit_body(self);
      self.assembler.end_loop();
    }
    self.assembler.bind(&mut end);
  }

  /// v8: cc:984-1005 `CompileNonNullableGreedyPlus(emit_body)` —
  /// begin: <body>; FORK end; JMP begin; end: (体非空可 ⟹ 不复制体, 嵌套 plus 线性)
  fn compile_non_nullable_greedy_plus(&mut self, emit_body: &mut impl FnMut(&mut Self)) {
    let mut begin = Label::new();
    let mut end = Label::new();

    self.assembler.bind(&mut begin);
    emit_body(self);

    self.assembler.fork(&mut end);
    self.assembler.jmp(&mut begin);
    self.assembler.bind(&mut end);
  }

  /// v8: cc:1007-1024 `CompileNonNullableNonGreedyPlus(emit_body)` —
  /// begin: <body>; FORK begin
  fn compile_non_nullable_non_greedy_plus(&mut self, emit_body: &mut impl FnMut(&mut Self)) {
    let mut begin = Label::new();

    self.assembler.bind(&mut begin);
    emit_body(self);

    self.assembler.fork(&mut begin);
  }

  /// v8: cc:1178-1184 `RemapQuantifier(int id)` — flag 开时才被调用 (cc:1046,1179)
  fn remap_quantifier(&mut self, id: i32) -> i32 {
    debug_assert!(self.v8_flags.experimental_regexp_engine_capture_group_opt());
    let map = self
      .quantifier_id_remapping
      .get_or_insert_with(RemapMap::default);
    map.get_or_insert(id)
  }

  /// v8: cc:1186-1190 `RemapLookaround(int id)` — 环视 id 稠密化
  fn remap_lookaround(&mut self, id: i32) -> i32 {
    self.lookaround_id_remapping.get_or_insert(id)
  }
}

impl Visitor for CompileVisitor<'_> {
  type Output = ();

  /// v8: cc:710-715 VisitDisjunction
  fn visit_disjunction(&mut self, node: &Disjunction) -> Self::Output {
    let alts = &node.alternatives;
    // 原版 lambda 捕获 this+alts 按下标 Accept; Rust 闭包经形参取 &mut Self,
    // alt_num = alts.length() (Vec 与 ZoneList::length 等长)
    self.compile_disjunction(alts.len() as i32, &mut |slf: &mut Self, i| {
      alts[i as usize].accept(slf);
    });
  }

  /// v8: cc:717-729 VisitAlternative — reverse 相逆序下钻
  fn visit_alternative(&mut self, node: &Alternative) -> Self::Output {
    if self.reverse {
      for child in node.nodes.iter().rev() {
        child.accept(self);
      }
    } else {
      for child in &node.nodes {
        child.accept(self);
      }
    }
  }

  /// v8: cc:731-734 VisitAssertion
  fn visit_assertion(&mut self, node: &Assertion) -> Self::Output {
    self.assembler.assertion(node.assertion_type);
  }

  /// v8: cc:791-794 VisitClassRanges — ranges(zone_) 返回副本 (标准集在
  /// syntax crate 构造时已展开, ast.rs:307-316), Rust 侧同款 clone
  fn visit_class_ranges(&mut self, node: &ClassRanges) -> Self::Output {
    self.compile_character_ranges(node.set.ranges.clone(), node.is_negated());
  }

  /// v8: cc:796-801 VisitClassSetOperand — TODO(v8:11935) 字符串不支持;
  /// DCHECK(!has_strings) 承接
  fn visit_class_set_operand(&mut self, node: &ClassSetOperand) -> Self::Output {
    debug_assert!(!node.has_strings());
    self.compile_character_ranges(node.ranges.clone(), false);
  }

  /// v8: cc:803-806 VisitClassSetExpression — CanBeHandled 恒拒 (cc:79-83),
  /// 到此 100% 不可达 (原版 UNREACHABLE())
  fn visit_class_set_expression(&mut self, _node: &ClassSetExpression) -> Self::Output {
    unreachable!("v8 cc:805 UNREACHABLE(): ClassSetExpression 已被 CanBeHandled 拒绝")
  }

  /// v8: cc:808-820 VisitAtom — reverse 相倒序逐字符 CONSUME_RANGE
  fn visit_atom(&mut self, node: &Atom) -> Self::Output {
    if self.reverse {
      for &c in node.data.iter().rev() {
        self.assembler.consume_range(c, c);
      }
    } else {
      for &c in &node.data {
        self.assembler.consume_range(c, c);
      }
    }
  }

  /// v8: cc:1026-1122 VisitQuantifier — 量词 lowering 主通道
  fn visit_quantifier(&mut self, node: &Quantifier) -> Self::Output {
    // cc:1027-1033: flag 开且 max==0 → 完全不产体 (SET_QUANTIFIER_TO_CLOCK 亦不产,
    // filter 相 cc:455-457 同款跳过)
    if self.v8_flags.experimental_regexp_engine_capture_group_opt() && node.max == 0 {
      return;
    }

    // cc:1043-1044 (TODO(mbid,v8:1037 优化注释 cc:1037-1042 保留原义:
    // 并非每次重复都需捕获/清寄存器, 原版未实现):
    let body_registers = node.body.capture_registers(StackLimiter::new(K_MAX_INT));

    // cc:1045-1053 emit_body: flag 开 → SET_QUANTIFIER_TO_CLOCK(重映射 id);
    // 关 → 先清体内寄存器再下钻
    let mut emit_body = |slf: &mut Self| {
      if slf.v8_flags.experimental_regexp_engine_capture_group_opt() {
        let id = slf.remap_quantifier(node.index);
        slf.assembler.set_quantifier_to_clock(id);
      } else {
        slf.clear_registers(body_registers);
      }
      node.body.accept(slf);
    };

    // cc:1055-1057: 可归约为非空 plus ⟹ 嵌套 {n,} 少复制一遍体 (线性化)
    let can_be_reduced_to_non_nullable_plus =
      node.min > 0 && node.max == K_INFINITY && node.min_match > 0;

    if can_be_reduced_to_non_nullable_plus {
      // cc:1065-1069: 必重复 min-1 次, 最后一次并入 plus 循环复用
      for _ in 0..node.min - 1 {
        emit_body(self);
      }
      // cc:1073-1086 switch (POSSESSIVE 已被 CanBeHandled cc:144-148 拒绝)
      match node.quantifier_type {
        QuantifierType::Greedy => self.compile_non_nullable_greedy_plus(&mut emit_body),
        QuantifierType::NonGreedy => self.compile_non_nullable_non_greedy_plus(&mut emit_body),
        QuantifierType::Possessive => {
          unreachable!("v8 cc:1075 UNREACHABLE(): POSSESSIVE 已被 CanBeHandled 拒绝")
        }
      }
    } else {
      // cc:1088-1094: <body>+ → <body><body>*, <body>{n,} → <body>{n}<body>*
      for _ in 0..node.min {
        emit_body(self);
      }
      // cc:1097-1118
      match node.quantifier_type {
        QuantifierType::Greedy => {
          if node.max == K_INFINITY {
            self.compile_greedy_star(&mut emit_body);
          } else {
            debug_assert_ne!(node.max, K_INFINITY);
            self.compile_greedy_repetition(&mut emit_body, node.max - node.min);
          }
        }
        QuantifierType::NonGreedy => {
          if node.max == K_INFINITY {
            self.compile_non_greedy_star(&mut emit_body);
          } else {
            debug_assert_ne!(node.max, K_INFINITY);
            self.compile_non_greedy_repetition(&mut emit_body, node.max - node.min);
          }
        }
        QuantifierType::Possessive => {
          unreachable!("v8 cc:1099 UNREACHABLE(): POSSESSIVE 已被 CanBeHandled 拒绝")
        }
      }
    }
  }

  /// v8: cc:1124-1139 VisitCapture — reverse 相 start/end 寄存器互换
  fn visit_capture(&mut self, node: &Capture) -> Self::Output {
    if self.ignore_captures {
      // cc:1125-1127: 跳过 SET_REGISTER_TO_CP, 仅下钻
      node.body.accept(self);
    } else {
      let index = node.index;
      let start_register = Capture::start_register(index);
      let end_register = Capture::end_register(index);

      self.assembler.set_register_to_cp(if self.reverse {
        end_register
      } else {
        start_register
      });
      node.body.accept(self);
      self.assembler.set_register_to_cp(if self.reverse {
        start_register
      } else {
        end_register
      });
    }
  }

  /// v8: cc:1141-1144 VisitGroup — flag 决策已在 CanBeHandled, 直接下钻
  fn visit_group(&mut self, node: &Group) -> Self::Output {
    node.body.accept(self);
  }

  /// v8: cc:1146-1156 VisitLookaround — 主表达式只读结果表; 入队待编。
  /// 队列存裸指针 (v8: cc:1197 `ZoneLinkedList<Lookaround*>` 同构承接): `node`
  /// 借用自 `tree.accept` 借用链, `compile` 调用区间内 AST 全程存活, 出队点
  /// 解引用见 [`CompileVisitor::compile`] 的 SAFETY 注记。
  fn visit_lookaround(&mut self, node: &Lookaround) -> Self::Output {
    let remapped = self.remap_lookaround(node.index);
    self
      .assembler
      .read_lookaround_table(remapped, node.is_positive, node.lookaround_type);

    if !self.ignore_lookarounds {
      self.lookarounds.push_back(from_ref(node));
    }
  }

  /// v8: cc:1158-1160 VisitBackReference — CanBeHandled 恒拒 (cc:197-201),
  /// 到此 100% 不可达 (原版 UNREACHABLE())
  fn visit_back_reference(&mut self, _node: &BackReference) -> Self::Output {
    unreachable!("v8 cc:1159 UNREACHABLE(): BackReference 已被 CanBeHandled 拒绝")
  }

  /// v8: cc:1162 VisitEmpty — 无指令
  fn visit_empty(&mut self, _node: &Empty) -> Self::Output {}

  /// v8: cc:1164-1176 VisitText — reverse 相逆序下钻元素
  fn visit_text(&mut self, node: &Text) -> Self::Output {
    if self.reverse {
      for el in node.elements.iter().rev() {
        el.tree.accept(self);
      }
    } else {
      for el in &node.elements {
        el.tree.accept(self);
      }
    }
  }
}

/// 编译 tree 为实验引擎字节码程序 (v8: experimental-compiler.h:28 + cc:1219-1222
/// `ExperimentalCompiler::Compile`)。前提: 引擎可处理 (见 [`can_be_handled`],
/// 原版由调用方 experimental.cc 保证, 编译器内部 UNREACHABLE 分支即该契约)。
#[must_use]
pub fn compile(tree: &Tree, flags: RegExpFlags, v8_flags: &V8Flags) -> Vec<Instruction> {
  CompileVisitor::compile(tree, flags, v8_flags)
}
