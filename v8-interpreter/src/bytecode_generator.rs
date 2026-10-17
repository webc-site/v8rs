// 由 tool/src/interpreter/bytecode-generator.h.js 从 v8/src/interpreter/bytecode-generator.{h,cc} 生成; 修订就地改, 复跑覆盖
// 真实的 Ignition BytecodeGenerator AST 遍历器与字节码生成器

use std::sync::Arc;

use gxhash::{HashMap, HashSet};
use v8_ast::{
  AstAssignOp, AstCompareOp,
  ast::{
    ArrayLiteral, Assignment, AstBinaryOp, AstCompoundAssignOp, AstNode, AstUnaryOp,
    BinaryOperation, Block, Call, CallNew, CompareOperation, CompoundAssignment, Conditional,
    CountOperation, DoWhileStatement, ForInStatement, ForStatement, FunctionDeclaration,
    FunctionLiteral, IfStatement, Literal, NaryOperation, ObjectLiteral, Property, ReturnStatement,
    SwitchStatement, ThisExpression, Throw, TryCatchStatement, TryFinallyStatement, UnaryOperation,
    VariableDeclaration, VariableProxy, WhileStatement,
  },
  value::AstValue,
};

use crate::{
  bytecode_array_builder::{
    BytecodeArray, BytecodeArrayBuilder, ConstantPoolEntry, InterpFunction, ToBooleanMode,
  },
  bytecode_dispatcher::fits_smi,
  bytecode_label::{BytecodeLabel, BytecodeLoopHeader},
  bytecode_register::Register,
  error::{Error, Result},
};

/// 函数字面量编译产物 (12.40): (字节码数组, 被内层捕获的本地名, rest 形参落地 (收集起点, 局部寄存器号))
pub type CompiledFunction = (BytecodeArray, Vec<String>, Option<(usize, i32)>);

/// 表达式类型提示位族 (原版 enum TypeHint : uint8_t)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum TypeHint {
  /// 布尔 (1 << 0)
  Boolean = 1 << 0,
  /// 内部化字符串 (1 << 1)
  InternalizedString = 1 << 1,
  /// 字符串 = 内部化 | (1 << 2)
  String = (1 << 1) | (1 << 2),
  /// 布尔 | 字符串
  Any = (1 << 0) | (1 << 1) | (1 << 2),
  /// 未知 (全位)
  Unknown = 0xFF,
}

impl TypeHint {
  /// hint2 是否与 hint1 相同或为其子型
  #[inline]
  #[must_use]
  pub const fn is_same_or_sub_type(hint1: Self, hint2: Self) -> bool {
    hint1 as u8 == (hint1 as u8 | hint2 as u8)
  }

  /// 是否字符串型
  #[inline]
  #[must_use]
  pub const fn is_string_type(hint: Self) -> bool {
    Self::is_same_or_sub_type(Self::String, hint)
  }

  /// 类型提示 -> ToBoolean 模式
  #[inline]
  #[must_use]
  pub const fn to_boolean_mode(self) -> ToBooleanMode {
    match self {
      Self::Boolean => ToBooleanMode::AlreadyBoolean,
      _ => ToBooleanMode::ConvertToBoolean,
    }
  }
}

/// 比较落穿分支 (原版 enum class TestFallthrough)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum TestFallthrough {
  Then = 0,
  Else = 1,
  None = 2,
}

/// 累加器保存模式 (原版 enum class AccumulatorPreservingMode)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum AccumulatorPreservingMode {
  None = 0,
  Preserve = 1,
}

enum LoopContinueTarget {
  Header(BytecodeLoopHeader),
  Label(BytecodeLabel),
}

struct LoopScope {
  break_label: BytecodeLabel,
  continue_target: Option<LoopContinueTarget>,
  /// 标记语句标签 (break/continue L 按名在本栈上从内向外匹配)
  labels: Vec<String>,
  /// 裸 break/continue 能否停在本层: 循环/switch=true, 被标记的 block/if 伪层=false
  /// (node: 裸 break 要穿透标记块找最近循环)
  bare_breakable: bool,
}

/// break/continue 的真实落点
///
/// 穿出 finally 时不能一步跳到目标: 要先跑完各层收尾语句, 所以目标本身得存进续体里等副本生成。
#[derive(Clone, Copy, PartialEq, Eq)]
enum JumpDest {
  /// 普通跳转标签 (循环出口、for 的增量段)
  Label(BytecodeLabel),
  /// 循环头回跳 (while 的 continue 走 JumpLoop)
  Header(BytecodeLoopHeader),
}

impl JumpDest {
  fn emit(self, builder: &mut BytecodeArrayBuilder) -> Result<()> {
    match self {
      Self::Label(label) => builder.jump(label),
      Self::Header(header) => builder.jump_loop(&header, 0, 0),
    }
  }
}

/// 一层 try-finally 为某个跳转目标准备的收尾副本
struct JumpContinuation {
  /// 跑完收尾语句后要去的真实目标
  dest: JumpDest,
  /// 本层副本的入口标签
  entry: BytecodeLabel,
  /// 更外层还要穿过一层 finally 时, 副本末尾跳它的续体入口; 否则直接落到 `dest`
  next: Option<BytecodeLabel>,
}

/// 一层 try-finally 的跳出续体
///
/// 原版为 return/break/continue 各备一组 token/result 续体寄存器, 由
/// `JumpContinuation` 复用同一份 finalization block。这里没有续体寄存器, 改为多发射一份
/// finally 副本: 返回点先 `Star saved_reg` 再 `Jump return_label`, 副本末尾以 `Lda saved_reg`
/// 接 `Return`; break/continue 则跳进按目标登记的 `continuations` 副本, 副本末尾接着跳向外层
/// 续体或真实目标。嵌套 try-finally 共享同一个 `saved_reg`, 内层副本末尾跳向外层入口。
struct FinallyFrame {
  /// try 体内出现 return 才有: (返回续体入口, 承接返回值的寄存器)
  ret: Option<(BytecodeLabel, Register)>,
  /// 进入本层 try 体时的循环层数; 跳转目标的层号小于它, 说明这次跳转穿出本层 finally
  loop_depth: usize,
  /// break/continue 穿出本层的续体, 每个跳转目标一份
  continuations: Vec<JumpContinuation>,
}

/// 语句序列里是否出现属于当前函数的 `return`
///
/// 不下钻函数体: 内层函数的 `return` 与外层帧的 finally 无关。只有确实出现 return 时,
/// try-finally 才多发射一份副本, 否则白付一份字节码体积。
fn contains_return_in(statements: &[AstNode]) -> bool {
  statements.iter().any(contains_return)
}

fn contains_return(node: &AstNode) -> bool {
  match node {
    AstNode::ReturnStatement(_) => true,
    AstNode::Block(b) => contains_return_in(&b.statements),
    AstNode::IfStatement(s) => {
      contains_return(&s.then_statement)
        || s
          .else_statement
          .as_ref()
          .is_some_and(|stmt| contains_return(stmt))
    }
    AstNode::WhileStatement(s) => contains_return(&s.body),
    AstNode::DoWhileStatement(s) => contains_return(&s.body),
    AstNode::ForStatement(s) => contains_return(&s.body),
    AstNode::ForInStatement(s) => contains_return(&s.body),
    AstNode::SwitchStatement(s) => s
      .cases
      .iter()
      .any(|case| contains_return_in(&case.statements)),
    AstNode::TryCatchStatement(s) => {
      contains_return_in(&s.try_block.statements) || contains_return_in(&s.catch_block.statements)
    }
    AstNode::TryFinallyStatement(s) => {
      contains_return_in(&s.try_block.statements) || contains_return_in(&s.finally_block.statements)
    }
    _ => false,
  }
}

fn collect_nested_functions<'a>(node: &'a AstNode, funcs: &mut Vec<&'a FunctionLiteral>) {
  match node {
    AstNode::FunctionDeclaration(decl) => {
      if let Some(fun) = &decl.fun {
        funcs.push(fun);
        for stmt in &fun.body {
          collect_nested_functions(stmt, funcs);
        }
      }
    }
    AstNode::FunctionLiteral(fun) => {
      funcs.push(fun);
      for stmt in &fun.body {
        collect_nested_functions(stmt, funcs);
      }
    }
    AstNode::Block(b) => {
      for s in &b.statements {
        collect_nested_functions(s, funcs);
      }
    }
    AstNode::ExpressionStatement(e) => collect_nested_functions(&e.expression, funcs),
    AstNode::IfStatement(s) => {
      collect_nested_functions(&s.condition, funcs);
      collect_nested_functions(&s.then_statement, funcs);
      if let Some(e) = &s.else_statement {
        collect_nested_functions(e, funcs);
      }
    }
    AstNode::WhileStatement(s) => {
      collect_nested_functions(&s.cond, funcs);
      collect_nested_functions(&s.body, funcs);
    }
    AstNode::DoWhileStatement(s) => {
      collect_nested_functions(&s.cond, funcs);
      collect_nested_functions(&s.body, funcs);
    }
    AstNode::ForStatement(s) => {
      if let Some(i) = &s.init {
        collect_nested_functions(i, funcs);
      }
      if let Some(c) = &s.cond {
        collect_nested_functions(c, funcs);
      }
      if let Some(n) = &s.next {
        collect_nested_functions(n, funcs);
      }
      collect_nested_functions(&s.body, funcs);
    }
    AstNode::ForInStatement(s) => {
      collect_nested_functions(&s.each, funcs);
      collect_nested_functions(&s.subject, funcs);
      collect_nested_functions(&s.body, funcs);
    }
    AstNode::ForOfStatement(s) => {
      collect_nested_functions(&s.each, funcs);
      collect_nested_functions(&s.subject, funcs);
      collect_nested_functions(&s.body, funcs);
    }
    AstNode::SwitchStatement(s) => {
      collect_nested_functions(&s.tag, funcs);
      for clause in &s.cases {
        if let Some(l) = &clause.label {
          collect_nested_functions(l, funcs);
        }
        for stmt in &clause.statements {
          collect_nested_functions(stmt, funcs);
        }
      }
    }
    AstNode::ReturnStatement(s) => {
      if let Some(e) = &s.expression {
        collect_nested_functions(e, funcs);
      }
    }
    AstNode::Assignment(a) => {
      collect_nested_functions(&a.target, funcs);
      collect_nested_functions(&a.value, funcs);
    }
    AstNode::CompoundAssignment(ca) => {
      collect_nested_functions(&ca.target, funcs);
      collect_nested_functions(&ca.value, funcs);
    }
    AstNode::BinaryOperation(b) => {
      collect_nested_functions(&b.left, funcs);
      collect_nested_functions(&b.right, funcs);
    }
    AstNode::NaryOperation(n) => {
      collect_nested_functions(&n.first, funcs);
      for sub in &n.subsequent {
        collect_nested_functions(sub, funcs);
      }
    }
    AstNode::CompareOperation(c) => {
      collect_nested_functions(&c.left, funcs);
      collect_nested_functions(&c.right, funcs);
    }
    AstNode::UnaryOperation(u) => collect_nested_functions(&u.expression, funcs),
    AstNode::CountOperation(c) => collect_nested_functions(&c.expression, funcs),
    AstNode::Conditional(c) => {
      collect_nested_functions(&c.condition, funcs);
      collect_nested_functions(&c.then_expression, funcs);
      collect_nested_functions(&c.else_expression, funcs);
    }
    AstNode::Property(p) => {
      collect_nested_functions(&p.object, funcs);
      collect_nested_functions(&p.key, funcs);
    }
    AstNode::Call(c) => {
      collect_nested_functions(&c.expression, funcs);
      for arg in &c.arguments {
        collect_nested_functions(arg, funcs);
      }
    }
    AstNode::CallNew(cn) => {
      collect_nested_functions(&cn.expression, funcs);
      for arg in &cn.arguments {
        collect_nested_functions(arg, funcs);
      }
    }
    AstNode::ArrayLiteral(a) => {
      for elem in &a.values {
        collect_nested_functions(elem, funcs);
      }
    }
    AstNode::ObjectLiteral(o) => {
      for prop in &o.properties {
        collect_nested_functions(&prop.key, funcs);
        collect_nested_functions(&prop.value, funcs);
      }
    }
    AstNode::TryCatchStatement(s) => {
      for stmt in &s.try_block.statements {
        collect_nested_functions(stmt, funcs);
      }
      for stmt in &s.catch_block.statements {
        collect_nested_functions(stmt, funcs);
      }
    }
    AstNode::TryFinallyStatement(s) => {
      for stmt in &s.try_block.statements {
        collect_nested_functions(stmt, funcs);
      }
      for stmt in &s.finally_block.statements {
        collect_nested_functions(stmt, funcs);
      }
    }
    AstNode::Throw(t) => collect_nested_functions(&t.exception, funcs),
    _ => {}
  }
}

fn collect_local_defs(node: &AstNode, defs: &mut HashSet<String>) {
  match node {
    AstNode::VariableDeclaration(decl) => {
      if let Some(var) = &decl.var {
        defs.insert(var.to_string_lossy());
      }
    }
    AstNode::FunctionDeclaration(decl) => {
      if let Some(fun) = &decl.fun
        && let Some(name) = &fun.name
      {
        defs.insert(name.to_string_lossy());
      }
    }
    AstNode::Block(b) => {
      for s in &b.statements {
        collect_local_defs(s, defs);
      }
    }
    AstNode::IfStatement(s) => {
      collect_local_defs(&s.then_statement, defs);
      if let Some(e) = &s.else_statement {
        collect_local_defs(e, defs);
      }
    }
    AstNode::WhileStatement(s) => collect_local_defs(&s.body, defs),
    AstNode::DoWhileStatement(s) => collect_local_defs(&s.body, defs),
    AstNode::ForStatement(s) => {
      if let Some(i) = &s.init {
        collect_local_defs(i, defs);
      }
      collect_local_defs(&s.body, defs);
    }
    AstNode::ForInStatement(s) => {
      collect_local_defs(&s.each, defs);
      collect_local_defs(&s.body, defs);
    }
    AstNode::ForOfStatement(s) => {
      collect_local_defs(&s.each, defs);
      collect_local_defs(&s.body, defs);
    }
    AstNode::SwitchStatement(s) => {
      for clause in &s.cases {
        for stmt in &clause.statements {
          collect_local_defs(stmt, defs);
        }
      }
    }
    AstNode::TryCatchStatement(s) => {
      for stmt in &s.try_block.statements {
        collect_local_defs(stmt, defs);
      }
      for stmt in &s.catch_block.statements {
        collect_local_defs(stmt, defs);
      }
    }
    AstNode::TryFinallyStatement(s) => {
      for stmt in &s.try_block.statements {
        collect_local_defs(stmt, defs);
      }
      for stmt in &s.finally_block.statements {
        collect_local_defs(stmt, defs);
      }
    }
    _ => {}
  }
}

fn collect_var_refs(node: &AstNode, refs: &mut HashSet<String>) {
  match node {
    AstNode::VariableProxy(vp) => {
      refs.insert(vp.name.to_string_lossy());
    }
    AstNode::Block(b) => {
      for s in &b.statements {
        collect_var_refs(s, refs);
      }
    }
    AstNode::ExpressionStatement(e) => collect_var_refs(&e.expression, refs),
    AstNode::IfStatement(s) => {
      collect_var_refs(&s.condition, refs);
      collect_var_refs(&s.then_statement, refs);
      if let Some(e) = &s.else_statement {
        collect_var_refs(e, refs);
      }
    }
    AstNode::WhileStatement(s) => {
      collect_var_refs(&s.cond, refs);
      collect_var_refs(&s.body, refs);
    }
    AstNode::DoWhileStatement(s) => {
      collect_var_refs(&s.cond, refs);
      collect_var_refs(&s.body, refs);
    }
    AstNode::ForStatement(s) => {
      if let Some(i) = &s.init {
        collect_var_refs(i, refs);
      }
      if let Some(c) = &s.cond {
        collect_var_refs(c, refs);
      }
      if let Some(n) = &s.next {
        collect_var_refs(n, refs);
      }
      collect_var_refs(&s.body, refs);
    }
    AstNode::ForInStatement(s) => {
      collect_var_refs(&s.each, refs);
      collect_var_refs(&s.subject, refs);
      collect_var_refs(&s.body, refs);
    }
    AstNode::ForOfStatement(s) => {
      collect_var_refs(&s.each, refs);
      collect_var_refs(&s.subject, refs);
      collect_var_refs(&s.body, refs);
    }
    AstNode::SwitchStatement(s) => {
      collect_var_refs(&s.tag, refs);
      for clause in &s.cases {
        if let Some(l) = &clause.label {
          collect_var_refs(l, refs);
        }
        for stmt in &clause.statements {
          collect_var_refs(stmt, refs);
        }
      }
    }
    AstNode::ReturnStatement(s) => {
      if let Some(e) = &s.expression {
        collect_var_refs(e, refs);
      }
    }
    AstNode::Assignment(a) => {
      collect_var_refs(&a.target, refs);
      collect_var_refs(&a.value, refs);
    }
    AstNode::CompoundAssignment(ca) => {
      collect_var_refs(&ca.target, refs);
      collect_var_refs(&ca.value, refs);
    }
    AstNode::BinaryOperation(b) => {
      collect_var_refs(&b.left, refs);
      collect_var_refs(&b.right, refs);
    }
    AstNode::NaryOperation(n) => {
      collect_var_refs(&n.first, refs);
      for sub in &n.subsequent {
        collect_var_refs(sub, refs);
      }
    }
    AstNode::CompareOperation(c) => {
      collect_var_refs(&c.left, refs);
      collect_var_refs(&c.right, refs);
    }
    AstNode::UnaryOperation(u) => collect_var_refs(&u.expression, refs),
    AstNode::CountOperation(c) => collect_var_refs(&c.expression, refs),
    AstNode::Conditional(c) => {
      collect_var_refs(&c.condition, refs);
      collect_var_refs(&c.then_expression, refs);
      collect_var_refs(&c.else_expression, refs);
    }
    AstNode::Property(p) => {
      collect_var_refs(&p.object, refs);
      if p.is_computed {
        collect_var_refs(&p.key, refs);
      }
    }
    AstNode::Call(c) => {
      collect_var_refs(&c.expression, refs);
      for arg in &c.arguments {
        collect_var_refs(arg, refs);
      }
    }
    AstNode::CallNew(cn) => {
      collect_var_refs(&cn.expression, refs);
      for arg in &cn.arguments {
        collect_var_refs(arg, refs);
      }
    }
    AstNode::ArrayLiteral(a) => {
      for elem in &a.values {
        collect_var_refs(elem, refs);
      }
    }
    AstNode::ObjectLiteral(o) => {
      for prop in &o.properties {
        if prop.is_computed {
          collect_var_refs(&prop.key, refs);
        }
        collect_var_refs(&prop.value, refs);
      }
    }
    AstNode::FunctionDeclaration(decl) => {
      if let Some(fun) = &decl.fun {
        for s in &fun.body {
          collect_var_refs(s, refs);
        }
      }
    }
    AstNode::FunctionLiteral(fun) => {
      for s in &fun.body {
        collect_var_refs(s, refs);
      }
    }
    AstNode::TryCatchStatement(s) => {
      for stmt in &s.try_block.statements {
        collect_var_refs(stmt, refs);
      }
      for stmt in &s.catch_block.statements {
        collect_var_refs(stmt, refs);
      }
    }
    AstNode::TryFinallyStatement(s) => {
      for stmt in &s.try_block.statements {
        collect_var_refs(stmt, refs);
      }
      for stmt in &s.finally_block.statements {
        collect_var_refs(stmt, refs);
      }
    }
    AstNode::Throw(t) => collect_var_refs(&t.exception, refs),
    _ => {}
  }
}

/// 真实的 AST 字节码生成器 (BytecodeGenerator)
pub struct BytecodeGenerator {
  builder: BytecodeArrayBuilder,
  locals: HashMap<String, Register>,
  loop_stack: Vec<LoopScope>,
  /// 正在生成且带 finally 的 try 帧栈, 栈顶即 `return` 应交付续体的那一层
  finally_stack: Vec<FinallyFrame>,
  is_top_level: bool,
  captured_vars: HashSet<String>,
  /// 正在生成的语句其值是否会被下游读取 (即程序/块的最后一条语句)
  ///
  /// 对标原版 `IsAccumulatorLive`: 只有值存活时, `if` 这类语句才需要把两条分支的完成值
  /// 落到累加器里; 否则跳过分支时留下的条件值会被误当成语句值 (`eval("2; if(false){}else{3}")`)。
  /// 非最后一条语句传 `false`, 免得给热循环里的每个 `if` 白加两条 `LdaUndefined`。
  in_completion: bool,
}

impl BytecodeGenerator {
  #[inline]
  #[must_use]
  pub fn new(
    parameter_count: usize,
    locals_count: usize,
    is_top_level: bool,
    captured_vars: HashSet<String>,
  ) -> Self {
    Self {
      builder: BytecodeArrayBuilder::new(parameter_count, locals_count),
      locals: HashMap::default(),
      loop_stack: Vec::new(),
      finally_stack: Vec::new(),
      is_top_level,
      captured_vars,
      in_completion: false,
    }
  }

  /// 把名字登记到寄存器槽, 返回 `None` 表示它由环境单元承载
  ///
  /// 原版给被捕获变量开的是 context slot, 不是寄存器: 我们此前同一变量既进寄存器又镜像到
  /// 全局表, 内层函数改了环境里的值, 外层寄存器仍是旧值 (`x = x + 5` 之后外层读到旧 x)。
  /// 被捕获名压根不入寄存器表, 读写点自然全部落到 `LdaGlobal`/`StaGlobal` 的环境链上。
  fn bind_local(&mut self, name: &str) -> Option<Register> {
    if self.captured_vars.contains(name) {
      return None;
    }
    if let Some(&r) = self.locals.get(name) {
      return Some(r);
    }
    let r = self.builder.allocate_register();
    self.locals.insert(name.to_string(), r);
    Some(r)
  }

  /// 依次生成语句序列, 只有序列末条在其上游存活时才标记为完成值位置
  fn visit_statements(&mut self, statements: &[AstNode]) -> Result<()> {
    let live = self.in_completion;
    let last = statements.len().saturating_sub(1);
    for (i, stmt) in statements.iter().enumerate() {
      self.in_completion = live && i == last;
      self.visit_node(stmt)?;
    }
    self.in_completion = live;
    Ok(())
  }

  /// 编译通用 AST 节点生成 BytecodeArray
  pub fn generate(ast: &AstNode) -> Result<BytecodeArray> {
    let mut generator = Self::new(0, 0, true, HashSet::default());
    let mut completion_value = false;
    if let AstNode::Block(b) = ast {
      generator.hoist_global_vars(&b.statements)?;
      generator.hoist_declarations(&b.statements)?;
      // 脚本完成值取自末条语句留下的累加器: 只有本身能产出完成值的语句 (表达式、块、if)
      // 才走这条路, 其余 (var/函数声明/循环) 按原版语义返回 undefined
      completion_value = matches!(
        b.statements.last(),
        Some(AstNode::ExpressionStatement(_))
          | Some(AstNode::Block(_))
          | Some(AstNode::IfStatement(_))
      );
      generator.in_completion = completion_value;
      generator.visit_statements(&b.statements)?;
    } else {
      generator.visit_node(ast)?;
    }
    if !completion_value {
      generator.builder.load_undefined()?;
    }
    generator.builder.return_op()?;
    Ok(generator.builder.build())
  }

  /// 编译函数字面量 AST 生成 BytecodeArray (包含 receiver 与形参)
  pub fn generate_function(literal: &FunctionLiteral) -> Result<BytecodeArray> {
    Self::generate_function_with_locals(literal).map(|(b, ..)| b)
  }

  /// 编译函数字面量 AST 生成 BytecodeArray 以及当前函数内所有声明的局部变量与形参名;
  /// 第三个返回值是 rest 形参的 (收集起点, 局部寄存器号), 交给 InterpFunction 进帧落地
  pub fn generate_function_with_locals(literal: &FunctionLiteral) -> Result<CompiledFunction> {
    let mut nested_funcs = Vec::new();
    for stmt in &literal.body {
      collect_nested_functions(stmt, &mut nested_funcs);
    }

    let mut captured_vars = HashSet::default();
    for f in nested_funcs {
      let mut local_defs = HashSet::default();
      for p in &f.params {
        local_defs.insert(p.to_string_lossy());
      }
      // rest 名是内层函数自带绑定, 不能被当成对外层同名变量的捕获引用 (12.40)
      if let Some((r, _)) = f.rest_param.as_deref() {
        local_defs.insert(r.clone());
      }
      for stmt in &f.body {
        collect_local_defs(stmt, &mut local_defs);
      }

      let mut var_refs = HashSet::default();
      for stmt in &f.body {
        collect_var_refs(stmt, &mut var_refs);
      }

      for r in var_refs {
        if !local_defs.contains(&r) {
          captured_vars.insert(r);
        }
      }
    }

    let param_count = literal.params.len();
    // 对标 V8: parameter_count 包含隐式 receiver (this)
    let mut generator = Self::new(param_count + 1, 0, false, captured_vars);

    // 绑定 receiver 到 "this" 符号
    generator
      .locals
      .insert("this".to_string(), Register::receiver());

    for (i, param) in literal.params.iter().enumerate() {
      // 对标 V8: 参数 0 为 receiver，形参从 1 开始编码
      let param_name = param.to_string_lossy();
      let reg = Register::from_parameter_index((i + 1) as i32);
      if generator.captured_vars.contains(&param_name) {
        // 被捕获的形参只落环境单元, 不留寄存器副本 (见 bind_local)
        generator.builder.load_accumulator_with_register(reg)?;
        generator.builder.store_global(&param_name, 0)?;
      } else {
        generator.locals.insert(param_name, reg);
      }
    }

    // rest 形参 (12.40): 专属局部寄存器由分发器在进帧时刻写入实参尾段快照;
    // 被子函数捕获时与捕获形参同路 —— 体首先取寄存器值再落环境单元, 不留寄存器副本
    let mut rest_binding = None;
    if let Some((rname, start)) = literal.rest_param.as_deref() {
      let rname = rname.clone();
      let reg = generator.builder.allocate_register();
      if generator.captured_vars.contains(&rname) {
        generator.builder.load_accumulator_with_register(reg)?;
        generator.builder.store_global(&rname, 0)?;
      } else {
        generator.locals.insert(rname, reg);
      }
      rest_binding = Some((*start, reg.index()));
    }

    generator.hoist_declarations(&literal.body)?;

    for stmt in &literal.body {
      generator.visit_node(stmt)?;
    }

    generator.builder.load_undefined()?;
    generator.builder.return_op()?;
    let mut my_defs = HashSet::default();
    for p in &literal.params {
      my_defs.insert(p.to_string_lossy());
    }
    // rest 名同样算本函数自带绑定 (被内层引用时要进 captured_locals 名单, 12.40)
    if let Some((r, _)) = literal.rest_param.as_deref() {
      my_defs.insert(r.clone());
    }
    for stmt in &literal.body {
      collect_local_defs(stmt, &mut my_defs);
    }
    let captured_locals: Vec<String> = my_defs
      .into_iter()
      .filter(|name| generator.captured_vars.contains(name))
      .collect();
    Ok((generator.builder.build(), captured_locals, rest_binding))
  }

  /// 遍历 AST 节点
  pub fn visit_node(&mut self, node: &AstNode) -> Result<()> {
    match node {
      AstNode::Block(b) => self.visit_block(b),
      AstNode::ExpressionStatement(e) => self.visit_node(&e.expression),
      AstNode::IfStatement(s) => self.visit_if_statement(s),
      AstNode::WhileStatement(s) => self.visit_while_statement(s),
      AstNode::DoWhileStatement(s) => self.visit_do_while_statement(s),
      AstNode::ForStatement(s) => self.visit_for_statement(s),
      AstNode::ReturnStatement(s) => self.visit_return_statement(s),
      AstNode::BreakStatement(s) => self.visit_break(s.label.clone()),
      AstNode::ContinueStatement(s) => self.visit_continue(s.label.clone()),
      AstNode::VariableDeclaration(decl) => self.visit_variable_declaration(decl),
      AstNode::FunctionDeclaration(f) => self.visit_function_declaration(f),
      AstNode::EmptyStatement(_) => Ok(()),

      AstNode::Assignment(a) => self.visit_assignment(a),
      AstNode::CompoundAssignment(ca) => self.visit_compound_assignment(ca),
      AstNode::BinaryOperation(b) => self.visit_binary_operation(b),
      AstNode::NaryOperation(n) => self.visit_nary_operation(n),
      AstNode::CompareOperation(c) => self.visit_compare_operation(c),
      AstNode::UnaryOperation(u) => self.visit_unary_operation(u),
      AstNode::CountOperation(c) => self.visit_count_operation(c),
      AstNode::Literal(l) => self.visit_literal(l),
      AstNode::VariableProxy(v) => self.visit_variable_proxy(v),
      AstNode::Conditional(c) => self.visit_conditional(c),
      AstNode::Property(p) => self.visit_property(p),
      AstNode::Call(c) => self.visit_call(c),
      AstNode::CallNew(cn) => self.visit_call_new(cn),
      AstNode::ArrayLiteral(a) => self.visit_array_literal(a),
      AstNode::ObjectLiteral(o) => self.visit_object_literal(o),
      AstNode::FunctionLiteral(f) => self.visit_function_literal(f),
      AstNode::ThisExpression(t) => self.visit_this_expression(t),
      AstNode::SwitchStatement(s) => self.visit_switch_statement(s),
      AstNode::ForInStatement(s) => self.visit_for_in_statement(s),
      AstNode::TryCatchStatement(s) => self.visit_try_catch_statement(s),
      AstNode::TryFinallyStatement(s) => self.visit_try_finally_statement(s),
      AstNode::Throw(t) => self.visit_throw(t),
      _ => Ok(()),
    }
  }

  /// 顶层 var 提升: 把尚未存在的全局补成 undefined
  ///
  /// 对标原版 GlobalDeclarationInstantiation 的 "属性已存在则跳过": 宿主预置内建和上一段脚本
  /// 写入的全局不能被 `var` 重新声明抹掉, 所以只在取值为 undefined 时写入 (此时写入是 no-op
  /// 或补建)。函数作用域不需要这一步 —— 寄存器栈本就整片预置为 Undefined
  fn hoist_global_vars(&mut self, stmts: &[AstNode]) -> Result<()> {
    let mut defs = HashSet::default();
    for stmt in stmts {
      collect_local_defs(stmt, &mut defs);
    }
    // 函数声明名由 declare_function 直接落值, 不能再被 undefined 抢先占位
    for stmt in stmts {
      if let AstNode::FunctionDeclaration(decl) = stmt
        && let Some(fun) = &decl.fun
        && let Some(name) = &fun.name
      {
        defs.remove(&name.to_string_lossy());
      }
    }
    for name in defs {
      self.builder.load_global_inside_typeof(&name, 0)?;
      let declare = self.builder.new_label();
      let skip = self.builder.new_label();
      self.builder.jump_if_undefined(declare)?;
      self.builder.jump(skip)?;
      self.builder.bind(declare);
      self.builder.load_undefined()?;
      self.builder.store_global(&name, 0)?;
      self.builder.bind(skip);
    }
    Ok(())
  }

  fn hoist_declarations(&mut self, stmts: &[AstNode]) -> Result<()> {
    // var 提升预绑槽 (寄存器冲撞根因): 整个函数作用域的 var (含循环/条件体里的) 必须在
    // 生成任何代码前拿到寄存器槽。否则循环头先行分配的临时寄存器会在语句间隙被释放复用,
    // 循环体里晚绑的 var 与字节码里永久刻死的临时槽同号, 条件每轮回边重写用户变量
    // (`for(;c<3;){var w=…}` 里 Star 临时槽写回 c 的值, w 读回计数器末值)
    if !self.is_top_level {
      let mut defs = HashSet::default();
      for stmt in stmts {
        collect_local_defs(stmt, &mut defs);
      }
      // 排序保证槽号对同一源码确定可复现 (HashSet 迭代序随进程种子变化)
      let mut names: Vec<String> = defs.into_iter().collect();
      names.sort();
      for name in &names {
        self.bind_local(name);
      }
    }
    for stmt in stmts {
      match stmt {
        AstNode::FunctionDeclaration(decl) => {
          self.declare_function(decl)?;
        }
        AstNode::VariableDeclaration(decl) => {
          self.declare_variable(decl)?;
        }
        AstNode::Block(b) => {
          self.hoist_declarations(&b.statements)?;
        }
        _ => {}
      }
    }
    Ok(())
  }

  fn declare_function(&mut self, decl: &FunctionDeclaration) -> Result<()> {
    if let Some(fun) = &decl.fun {
      let (child_bytecode, captured_locals, rest_binding) =
        Self::generate_function_with_locals(fun)?;
      let uses_arguments = child_bytecode
        .constants()
        .iter()
        .any(|entry| matches!(entry, ConstantPoolEntry::String(s) if &**s == "arguments"));
      let interp_func = Arc::new(InterpFunction {
        name: fun.name.as_ref().map(|n| n.to_string_lossy()),
        params: fun.params.iter().map(|p| p.to_string_lossy()).collect(),
        captured_locals,
        bytecode_array: child_bytecode,
        uses_arguments,
        is_arrow: fun.is_arrow,
        src: fun.src.clone(),
        rest_binding,
      });
      self
        .builder
        .load_constant(ConstantPoolEntry::Function(interp_func))?;

      if let Some(name) = &fun.name {
        let s = name.to_string_lossy();
        // 顶层没有帧环境, 名字一律落全局表; 函数内未被子函数捕获的名字才占寄存器
        let in_reg = if self.is_top_level {
          None
        } else {
          self.bind_local(&s)
        };
        match in_reg {
          Some(reg) => self.builder.store_accumulator_in_register(reg)?,
          None => self.builder.store_global(&s, 0)?,
        }
      }
    }
    Ok(())
  }

  fn declare_variable(&mut self, decl: &VariableDeclaration) -> Result<()> {
    if self.is_top_level {
      return Ok(());
    }
    if let Some(var) = &decl.var {
      let name = var.to_string_lossy();
      self.bind_local(&name);
    }
    Ok(())
  }

  fn visit_block(&mut self, block: &Block) -> Result<()> {
    if block.labels.is_empty() {
      return self.visit_statements(&block.statements);
    }
    // 标记块: break L 的落点 = 块尾; 裸 break 不停靠 (bare_breakable=false 照常外传)
    let break_label = self.builder.new_label();
    self.loop_stack.push(LoopScope {
      break_label,
      continue_target: None,
      labels: block.labels.clone(),
      bare_breakable: false,
    });
    self.visit_statements(&block.statements)?;
    self.loop_stack.pop();
    self.builder.bind(break_label);
    Ok(())
  }

  fn visit_variable_declaration(&mut self, decl: &VariableDeclaration) -> Result<()> {
    self.declare_variable(decl)
  }

  fn visit_variable_proxy(&mut self, vp: &VariableProxy) -> Result<()> {
    let name = vp.name.to_string_lossy();
    if let Some(&reg) = self.locals.get(&name) {
      self.builder.load_accumulator_with_register(reg)?;
    } else {
      self.builder.load_global(&name, 0)?;
    }
    Ok(())
  }

  fn visit_literal(&mut self, lit: &Literal) -> Result<()> {
    match &lit.value {
      AstValue::Smi(val) => self.builder.load_literal_smi(*val)?,
      AstValue::Number(num) => {
        if fits_smi(*num) {
          self.builder.load_literal_smi(*num as i32)?;
        } else {
          self
            .builder
            .load_constant(ConstantPoolEntry::Number(*num))?;
        }
      }
      AstValue::Boolean(b) => self.builder.load_boolean(*b)?,
      AstValue::String(s) => {
        self
          .builder
          .load_constant(ConstantPoolEntry::new_string(s.to_string_lossy()))?;
      }
      AstValue::ConsString(cs) => {
        self
          .builder
          .load_constant(ConstantPoolEntry::new_string(cs.to_string()))?;
      }
      AstValue::BigInt(bi) => {
        self
          .builder
          .load_constant(ConstantPoolEntry::BigInt(bi.as_str().into()))?;
      }
      AstValue::Null => self.builder.load_null()?,
      AstValue::Undefined => self.builder.load_undefined()?,
      AstValue::TheHole => self.builder.load_the_hole()?,
    }
    Ok(())
  }

  fn extract_property_name(key: &AstNode) -> Option<String> {
    match key {
      AstNode::Literal(lit) => match &lit.value {
        AstValue::String(s) => Some(s.to_string_lossy()),
        AstValue::ConsString(cs) => Some(cs.to_string()),
        _ => None,
      },
      AstNode::VariableProxy(vp) => Some(vp.name.to_string_lossy()),
      _ => None,
    }
  }

  fn visit_assignment(&mut self, assign: &Assignment) -> Result<()> {
    match &*assign.target {
      AstNode::VariableProxy(vp) => {
        let var_name = vp.name.to_string_lossy();
        if let Some(&reg) = self.locals.get(&var_name) {
          if assign.op == AstAssignOp::Assign {
            self.visit_node(&assign.value)?;
            self.builder.store_accumulator_in_register(reg)?;
          } else if let Some(bin_op) = assign.op.binary_op() {
            let lhs_reg = self.builder.allocate_register();
            self.builder.load_accumulator_with_register(reg)?;
            self.builder.store_accumulator_in_register(lhs_reg)?;
            self.visit_node(&assign.value)?;
            self.builder.binary_operation(bin_op, lhs_reg, 0)?;
            self.builder.free_register(lhs_reg);
            self.builder.store_accumulator_in_register(reg)?;
          }
        } else {
          if assign.op == AstAssignOp::Assign {
            self.visit_node(&assign.value)?;
            self.builder.store_global(&var_name, 0)?;
          } else if let Some(bin_op) = assign.op.binary_op() {
            let lhs_reg = self.builder.allocate_register();
            self.builder.load_global(&var_name, 0)?;
            self.builder.store_accumulator_in_register(lhs_reg)?;
            self.visit_node(&assign.value)?;
            self.builder.binary_operation(bin_op, lhs_reg, 0)?;
            self.builder.free_register(lhs_reg);
            self.builder.store_global(&var_name, 0)?;
          }
        }
        Ok(())
      }
      AstNode::Property(prop) => {
        let named_prop = if !prop.is_computed {
          Self::extract_property_name(&prop.key)
        } else {
          None
        };

        if assign.op == AstAssignOp::Assign {
          let obj_reg = self.builder.allocate_register();
          self.visit_node(&prop.object)?;
          self.builder.store_accumulator_in_register(obj_reg)?;

          if let Some(ref name) = named_prop {
            self.visit_node(&assign.value)?;
            self.builder.set_named_property(obj_reg, name, 0)?;
          } else {
            let key_reg = self.builder.allocate_register();
            self.visit_node(&prop.key)?;
            self.builder.store_accumulator_in_register(key_reg)?;
            self.visit_node(&assign.value)?;
            self.builder.set_keyed_property(obj_reg, key_reg, 0)?;
            self.builder.free_register(key_reg);
          }
          self.builder.free_register(obj_reg);
        } else if let Some(bin_op) = assign.op.binary_op() {
          let obj_reg = self.builder.allocate_register();
          self.visit_node(&prop.object)?;
          self.builder.store_accumulator_in_register(obj_reg)?;

          let lhs_reg = self.builder.allocate_register();
          if let Some(ref name) = named_prop {
            self.builder.load_named_property(obj_reg, name, 0)?;
            self.builder.store_accumulator_in_register(lhs_reg)?;
            self.visit_node(&assign.value)?;
            self.builder.binary_operation(bin_op, lhs_reg, 0)?;
            self.builder.free_register(lhs_reg);
            self.builder.set_named_property(obj_reg, name, 0)?;
          } else {
            let key_reg = self.builder.allocate_register();
            self.visit_node(&prop.key)?;
            self.builder.store_accumulator_in_register(key_reg)?;
            self.builder.load_accumulator_with_register(key_reg)?;
            self.builder.load_keyed_property(obj_reg, 0)?;
            self.builder.store_accumulator_in_register(lhs_reg)?;
            self.visit_node(&assign.value)?;
            self.builder.binary_operation(bin_op, lhs_reg, 0)?;
            self.builder.free_register(lhs_reg);
            self.builder.set_keyed_property(obj_reg, key_reg, 0)?;
            self.builder.free_register(key_reg);
          }
          self.builder.free_register(obj_reg);
        }
        Ok(())
      }
      _ => Err(Error::UnsupportedAstNode("non-variable assignment target")),
    }
  }

  fn visit_compound_assignment(&mut self, ca: &CompoundAssignment) -> Result<()> {
    let bin_op = match ca.op {
      AstCompoundAssignOp::AssignAdd => AstBinaryOp::Add,
      AstCompoundAssignOp::AssignSub => AstBinaryOp::Sub,
      AstCompoundAssignOp::AssignMul => AstBinaryOp::Mul,
      AstCompoundAssignOp::AssignDiv => AstBinaryOp::Div,
      AstCompoundAssignOp::AssignMod => AstBinaryOp::Mod,
      AstCompoundAssignOp::AssignExp => AstBinaryOp::Exp,
      AstCompoundAssignOp::AssignBitOr => AstBinaryOp::BitOr,
      AstCompoundAssignOp::AssignBitAnd => AstBinaryOp::BitAnd,
      AstCompoundAssignOp::AssignBitXor => AstBinaryOp::BitXor,
      AstCompoundAssignOp::AssignShl => AstBinaryOp::Shl,
      AstCompoundAssignOp::AssignSar => AstBinaryOp::Sar,
      AstCompoundAssignOp::AssignShr => AstBinaryOp::Shr,
      AstCompoundAssignOp::AssignOr => AstBinaryOp::Or,
      AstCompoundAssignOp::AssignAnd => AstBinaryOp::And,
      AstCompoundAssignOp::AssignNullish => AstBinaryOp::Nullish,
    };

    match &*ca.target {
      AstNode::VariableProxy(vp) => {
        let var_name = vp.name.to_string_lossy();
        if let Some(&reg) = self.locals.get(&var_name) {
          let lhs_reg = self.builder.allocate_register();
          self.builder.load_accumulator_with_register(reg)?;
          self.builder.store_accumulator_in_register(lhs_reg)?;
          self.visit_node(&ca.value)?;
          self.builder.binary_operation(bin_op, lhs_reg, 0)?;
          self.builder.free_register(lhs_reg);
          self.builder.store_accumulator_in_register(reg)?;
          if self.is_top_level {
            self.builder.store_global(&var_name, 0)?;
          }
        } else {
          let lhs_reg = self.builder.allocate_register();
          self.builder.load_global(&var_name, 0)?;
          self.builder.store_accumulator_in_register(lhs_reg)?;
          self.visit_node(&ca.value)?;
          self.builder.binary_operation(bin_op, lhs_reg, 0)?;
          self.builder.free_register(lhs_reg);
          self.builder.store_global(&var_name, 0)?;
        }
        Ok(())
      }
      AstNode::Property(prop) => {
        let named_prop = if !prop.is_computed {
          Self::extract_property_name(&prop.key)
        } else {
          None
        };

        let obj_reg = self.builder.allocate_register();
        self.visit_node(&prop.object)?;
        self.builder.store_accumulator_in_register(obj_reg)?;

        let lhs_reg = self.builder.allocate_register();
        if let Some(ref name) = named_prop {
          self.builder.load_named_property(obj_reg, name, 0)?;
          self.builder.store_accumulator_in_register(lhs_reg)?;
          self.visit_node(&ca.value)?;
          self.builder.binary_operation(bin_op, lhs_reg, 0)?;
          self.builder.free_register(lhs_reg);
          self.builder.set_named_property(obj_reg, name, 0)?;
        } else {
          let key_reg = self.builder.allocate_register();
          self.visit_node(&prop.key)?;
          self.builder.store_accumulator_in_register(key_reg)?;
          self.builder.load_accumulator_with_register(key_reg)?;
          self.builder.load_keyed_property(obj_reg, 0)?;
          self.builder.store_accumulator_in_register(lhs_reg)?;
          self.visit_node(&ca.value)?;
          self.builder.binary_operation(bin_op, lhs_reg, 0)?;
          self.builder.free_register(lhs_reg);
          self.builder.set_keyed_property(obj_reg, key_reg, 0)?;
          self.builder.free_register(key_reg);
        }
        self.builder.free_register(obj_reg);
        Ok(())
      }
      _ => Err(Error::UnsupportedAstNode(
        "non-variable compound assignment",
      )),
    }
  }

  fn visit_binary_operation(&mut self, binop: &BinaryOperation) -> Result<()> {
    match binop.op {
      AstBinaryOp::Comma => {
        self.visit_node(&binop.left)?;
        self.visit_node(&binop.right)?;
        return Ok(());
      }
      AstBinaryOp::Or => {
        let end_label = self.builder.new_label();
        self.visit_node(&binop.left)?;
        self
          .builder
          .jump_if_true(ToBooleanMode::ConvertToBoolean, end_label)?;
        self.visit_node(&binop.right)?;
        self.builder.bind(end_label);
        return Ok(());
      }
      AstBinaryOp::And => {
        let end_label = self.builder.new_label();
        self.visit_node(&binop.left)?;
        self
          .builder
          .jump_if_false(ToBooleanMode::ConvertToBoolean, end_label)?;
        self.visit_node(&binop.right)?;
        self.builder.bind(end_label);
        return Ok(());
      }
      AstBinaryOp::Nullish => {
        let end_label = self.builder.new_label();
        let is_null_or_undef = self.builder.new_label();
        self.visit_node(&binop.left)?;
        self.builder.jump_if_null(is_null_or_undef)?;
        self.builder.jump_if_undefined(is_null_or_undef)?;
        self.builder.jump(end_label)?;
        self.builder.bind(is_null_or_undef);
        self.visit_node(&binop.right)?;
        self.builder.bind(end_label);
        return Ok(());
      }
      _ => {}
    }

    let lhs_reg = self.builder.allocate_register();
    self.visit_node(&binop.left)?;
    self.builder.store_accumulator_in_register(lhs_reg)?;
    self.visit_node(&binop.right)?;
    self.builder.binary_operation(binop.op, lhs_reg, 0)?;
    self.builder.free_register(lhs_reg);
    Ok(())
  }

  fn visit_nary_operation(&mut self, nary: &NaryOperation) -> Result<()> {
    let lhs_reg = self.builder.allocate_register();
    self.visit_node(&nary.first)?;
    self.builder.store_accumulator_in_register(lhs_reg)?;
    for next in &nary.subsequent {
      self.visit_node(next)?;
      self.builder.binary_operation(nary.op, lhs_reg, 0)?;
      self.builder.store_accumulator_in_register(lhs_reg)?;
    }
    self.builder.load_accumulator_with_register(lhs_reg)?;
    self.builder.free_register(lhs_reg);
    Ok(())
  }

  fn visit_compare_operation(&mut self, comp: &CompareOperation) -> Result<()> {
    let lhs_reg = self.builder.allocate_register();
    self.visit_node(&comp.left)?;
    self.builder.store_accumulator_in_register(lhs_reg)?;
    self.visit_node(&comp.right)?;
    self.builder.compare_operation(comp.op, lhs_reg, 0)?;
    self.builder.free_register(lhs_reg);
    Ok(())
  }

  fn visit_unary_operation(&mut self, unop: &UnaryOperation) -> Result<()> {
    if unop.op == AstUnaryOp::Delete {
      if let AstNode::Property(ref p) = *unop.expression {
        let obj_reg = self.builder.allocate_register();
        self.visit_node(&p.object)?;
        self.builder.store_accumulator_in_register(obj_reg)?;
        self.visit_node(&p.key)?;
        self.builder.delete_property_sloppy(obj_reg)?;
        self.builder.free_register(obj_reg);
        return Ok(());
      }
      self.builder.load_true()?;
      return Ok(());
    }
    if unop.op == AstUnaryOp::Void {
      self.visit_node(&unop.expression)?;
      self.builder.load_undefined()?;
      return Ok(());
    }
    // typeof 的自由变量走 LdaGlobalInsideTypeof: 未声明名要得到 "undefined" 而非 ReferenceError
    if unop.op == AstUnaryOp::TypeOf
      && let AstNode::VariableProxy(ref vp) = *unop.expression
    {
      let name = vp.name.to_string_lossy();
      if !self.locals.contains_key(&name) {
        self.builder.load_global_inside_typeof(name, 0)?;
        self.builder.type_of(0)?;
        return Ok(());
      }
    }
    self.visit_node(&unop.expression)?;
    if unop.op == AstUnaryOp::TypeOf {
      self.builder.type_of(0)?;
    } else {
      self.builder.unary_operation(unop.op, 0)?;
    }
    Ok(())
  }

  fn visit_count_operation(&mut self, count: &CountOperation) -> Result<()> {
    match &*count.expression {
      AstNode::VariableProxy(vp) => {
        let var_name = vp.name.to_string_lossy();
        if let Some(&reg) = self.locals.get(&var_name) {
          if count.is_prefix {
            self.builder.load_accumulator_with_register(reg)?;
            self.builder.count_operation(count.op, 0)?;
            self.builder.store_accumulator_in_register(reg)?;
          } else {
            self.builder.load_accumulator_with_register(reg)?;
            let old_reg = self.builder.allocate_register();
            self.builder.store_accumulator_in_register(old_reg)?;
            self.builder.count_operation(count.op, 0)?;
            self.builder.store_accumulator_in_register(reg)?;
            self.builder.load_accumulator_with_register(old_reg)?;
            self.builder.free_register(old_reg);
          }
        } else {
          if count.is_prefix {
            self.builder.load_global(&var_name, 0)?;
            self.builder.count_operation(count.op, 0)?;
            self.builder.store_global(&var_name, 0)?;
          } else {
            self.builder.load_global(&var_name, 0)?;
            let old_reg = self.builder.allocate_register();
            self.builder.store_accumulator_in_register(old_reg)?;
            self.builder.count_operation(count.op, 0)?;
            self.builder.store_global(&var_name, 0)?;
            self.builder.load_accumulator_with_register(old_reg)?;
            self.builder.free_register(old_reg);
          }
        }
        Ok(())
      }
      AstNode::Property(prop) => {
        let named_prop = if !prop.is_computed {
          Self::extract_property_name(&prop.key)
        } else {
          None
        };

        let obj_reg = self.builder.allocate_register();
        self.visit_node(&prop.object)?;
        self.builder.store_accumulator_in_register(obj_reg)?;

        let key_reg = if named_prop.is_none() {
          let kr = self.builder.allocate_register();
          self.visit_node(&prop.key)?;
          self.builder.store_accumulator_in_register(kr)?;
          Some(kr)
        } else {
          None
        };

        if let Some(ref name) = named_prop {
          self.builder.load_named_property(obj_reg, name, 0)?;
        } else if let Some(kr) = key_reg {
          self.builder.load_accumulator_with_register(kr)?;
          self.builder.load_keyed_property(obj_reg, 0)?;
        }

        let old_reg = if !count.is_prefix {
          let r = self.builder.allocate_register();
          self.builder.store_accumulator_in_register(r)?;
          Some(r)
        } else {
          None
        };

        self.builder.count_operation(count.op, 0)?;

        if let Some(ref name) = named_prop {
          self.builder.set_named_property(obj_reg, name, 0)?;
        } else if let Some(kr) = key_reg {
          let val_reg = self.builder.allocate_register();
          self.builder.store_accumulator_in_register(val_reg)?;
          self.builder.load_accumulator_with_register(val_reg)?;
          self.builder.set_keyed_property(obj_reg, kr, 0)?;
          self.builder.free_register(val_reg);
        }

        if let Some(r) = old_reg {
          self.builder.load_accumulator_with_register(r)?;
          self.builder.free_register(r);
        }

        if let Some(kr) = key_reg {
          self.builder.free_register(kr);
        }
        self.builder.free_register(obj_reg);
        Ok(())
      }
      _ => Err(Error::UnsupportedAstNode("non-variable count target")),
    }
  }

  fn visit_if_statement(&mut self, stmt: &IfStatement) -> Result<()> {
    self.visit_node(&stmt.condition)?;
    // 标记 if: break L 落到整条 if 之后 (then/else 都算穿出); 裸 break 不停靠
    let label_guard = if stmt.labels.is_empty() {
      None
    } else {
      let l = self.builder.new_label();
      self.loop_stack.push(LoopScope {
        break_label: l,
        continue_target: None,
        labels: stmt.labels.clone(),
        bare_breakable: false,
      });
      Some(l)
    };
    let end_label = self.builder.new_label();
    // 值存活时每条分支先清成 undefined: 分支体没有完成值 (空块、纯声明) 时, 累加器里留下的
    // 条件值会被当成 if 语句的值 (test262 language/statements/if/cptn-else-false-nrml.js)
    let live = self.in_completion;
    if live {
      let else_label = self.builder.new_label();
      self
        .builder
        .jump_if_false(ToBooleanMode::ConvertToBoolean, else_label)?;
      self.builder.load_undefined()?;
      self.visit_node(&stmt.then_statement)?;
      self.builder.jump(end_label)?;
      self.builder.bind(else_label);
      self.builder.load_undefined()?;
      if let Some(else_stmt) = &stmt.else_statement {
        self.visit_node(else_stmt)?;
      }
      self.builder.bind(end_label);
    } else if let Some(else_stmt) = &stmt.else_statement {
      let else_label = self.builder.new_label();
      self
        .builder
        .jump_if_false(ToBooleanMode::ConvertToBoolean, else_label)?;
      self.visit_node(&stmt.then_statement)?;
      self.builder.jump(end_label)?;
      self.builder.bind(else_label);
      self.visit_node(else_stmt)?;
      self.builder.bind(end_label);
    } else {
      self
        .builder
        .jump_if_false(ToBooleanMode::ConvertToBoolean, end_label)?;
      self.visit_node(&stmt.then_statement)?;
      self.builder.bind(end_label);
    }
    if let Some(l) = label_guard {
      self.loop_stack.pop();
      self.builder.bind(l);
    }
    Ok(())
  }

  fn visit_while_statement(&mut self, stmt: &WhileStatement) -> Result<()> {
    let mut loop_header = BytecodeLoopHeader::new();
    let end_label = self.builder.new_label();

    self.builder.bind_loop_header(&mut loop_header);
    self.visit_node(&stmt.cond)?;
    self
      .builder
      .jump_if_false(ToBooleanMode::ConvertToBoolean, end_label)?;

    self.loop_stack.push(LoopScope {
      break_label: end_label,
      continue_target: Some(LoopContinueTarget::Header(loop_header)),
      labels: stmt.labels.clone(),
      bare_breakable: true,
    });

    self.visit_node(&stmt.body)?;

    self.loop_stack.pop();

    self.builder.jump_loop(&loop_header, 0, 0)?;
    self.builder.bind(end_label);
    Ok(())
  }

  fn visit_do_while_statement(&mut self, stmt: &DoWhileStatement) -> Result<()> {
    let mut loop_header = BytecodeLoopHeader::new();
    let end_label = self.builder.new_label();
    let continue_label = self.builder.new_label();

    self.builder.bind_loop_header(&mut loop_header);

    self.loop_stack.push(LoopScope {
      break_label: end_label,
      continue_target: Some(LoopContinueTarget::Label(continue_label)),
      labels: stmt.labels.clone(),
      bare_breakable: true,
    });

    self.visit_node(&stmt.body)?;

    self.loop_stack.pop();

    self.builder.bind(continue_label);
    self.visit_node(&stmt.cond)?;
    self
      .builder
      .jump_if_false(ToBooleanMode::ConvertToBoolean, end_label)?;
    self.builder.jump_loop(&loop_header, 0, 0)?;
    self.builder.bind(end_label);
    Ok(())
  }

  fn visit_for_statement(&mut self, stmt: &ForStatement) -> Result<()> {
    if let Some(init) = &stmt.init {
      self.visit_node(init)?;
    }

    let mut loop_header = BytecodeLoopHeader::new();
    let end_label = self.builder.new_label();
    let continue_label = self.builder.new_label();

    self.builder.bind_loop_header(&mut loop_header);

    if let Some(cond) = &stmt.cond {
      self.visit_node(cond)?;
      self
        .builder
        .jump_if_false(ToBooleanMode::ConvertToBoolean, end_label)?;
    }

    self.loop_stack.push(LoopScope {
      break_label: end_label,
      continue_target: Some(LoopContinueTarget::Label(continue_label)),
      labels: stmt.labels.clone(),
      bare_breakable: true,
    });

    self.visit_node(&stmt.body)?;

    self.loop_stack.pop();

    self.builder.bind(continue_label);
    if let Some(next) = &stmt.next {
      self.visit_node(next)?;
    }

    self.builder.jump_loop(&loop_header, 0, 0)?;
    self.builder.bind(end_label);
    Ok(())
  }

  fn visit_return_statement(&mut self, stmt: &ReturnStatement) -> Result<()> {
    if let Some(expr) = &stmt.expression {
      self.visit_node(expr)?;
    } else {
      self.builder.load_undefined()?;
    }
    // 落在 try-finally 里时不能直接离帧: 先把返回值存进共享续体寄存器, 再跳进最近一层
    // finally 的返回副本, 由它跑完收尾语句后交付返回值 (原版靠 JumpContinuation 做同一件事)
    match self.finally_stack.last().and_then(|frame| frame.ret) {
      Some((return_label, saved_reg)) => {
        self.builder.store_accumulator_in_register(saved_reg)?;
        self.builder.jump(return_label)?;
      }
      None => self.builder.return_op()?,
    }
    Ok(())
  }

  /// `break` / `break L`: 裸 break 停最近可裸跳层 (循环/switch), 带名按标签从内向外找
  fn visit_break(&mut self, label: Option<String>) -> Result<()> {
    let pos = match &label {
      None => self.loop_stack.iter().rposition(|s| s.bare_breakable),
      Some(l) => self
        .loop_stack
        .iter()
        .rposition(|s| s.labels.iter().any(|n| n == l)),
    };
    match pos {
      Some(pos) => {
        let dest = JumpDest::Label(self.loop_stack[pos].break_label);
        self.jump_to(dest, pos)
      }
      None => Err(Error::UnsupportedAstNode("break outside of loop")),
    }
  }

  /// `continue` / `continue L`: 只有迭代语句层能接住 (continue_target Some)
  fn visit_continue(&mut self, label: Option<String>) -> Result<()> {
    for pos in (0..self.loop_stack.len()).rev() {
      if let Some(target) = &self.loop_stack[pos].continue_target {
        let hit = match &label {
          None => true,
          Some(l) => self.loop_stack[pos].labels.iter().any(|n| n == l),
        };
        if hit {
          let dest = match target {
            LoopContinueTarget::Header(header) => JumpDest::Header(*header),
            LoopContinueTarget::Label(lbl) => JumpDest::Label(*lbl),
          };
          return self.jump_to(dest, pos);
        }
      }
    }
    Err(Error::UnsupportedAstNode("continue outside of loop"))
  }

  /// 跳到 `dest`, 途中经过的每一层 try-finally 都先执行其收尾语句
  ///
  /// 目标循环比某层 finally 更外层 (层号小于该帧的 `loop_depth`) 就算穿出这层, 被穿过的帧是
  /// 栈顶的一段后缀。按原版 `JumpScope` 的做法把它们的副本串成续体链: 站点跳进最内层副本,
  /// 副本跑完跳向更外一层, 最外层副本才落到真实目标 —— 收尾语句的次序必须由内向外。
  /// 因此登记顺序也得由外向内 (每层的 `next` 是已建好的更外一层入口), 而站点要跳的是
  /// 链上最后建的那个 (最内层) 入口。
  fn jump_to(&mut self, dest: JumpDest, loop_index: usize) -> Result<()> {
    let crossed: Vec<usize> = (0..self.finally_stack.len())
      .filter(|&depth| self.finally_stack[depth].loop_depth > loop_index)
      .collect();
    let mut next: Option<BytecodeLabel> = None;
    for depth in crossed {
      next = Some(self.continuation_entry(depth, dest, next));
    }
    match next {
      Some(label) => self.builder.jump(label)?,
      None => dest.emit(&mut self.builder)?,
    }
    Ok(())
  }

  /// 取 `depth` 层里目标为 `dest` 的续体入口, 没有就新建一份副本
  fn continuation_entry(
    &mut self,
    depth: usize,
    dest: JumpDest,
    next: Option<BytecodeLabel>,
  ) -> BytecodeLabel {
    if let Some(cont) = self.finally_stack[depth]
      .continuations
      .iter()
      .find(|cont| cont.dest == dest)
    {
      return cont.entry;
    }
    let entry = self.builder.new_label();
    self.finally_stack[depth]
      .continuations
      .push(JumpContinuation { dest, entry, next });
    entry
  }

  fn assign_accumulator_to_target(&mut self, target: &AstNode) -> Result<()> {
    match target {
      AstNode::VariableProxy(vp) => {
        let var_name = vp.name.to_string_lossy();
        match self.locals.get(&var_name).copied() {
          Some(reg) => self.builder.store_accumulator_in_register(reg)?,
          None => self.builder.store_global(&var_name, 0)?,
        }
        Ok(())
      }
      AstNode::VariableDeclaration(decl) => {
        if let Some(var) = &decl.var {
          let var_name = var.to_string_lossy();
          let in_reg = if self.is_top_level {
            None
          } else {
            self.bind_local(&var_name)
          };
          match in_reg {
            Some(reg) => self.builder.store_accumulator_in_register(reg)?,
            None => self.builder.store_global(&var_name, 0)?,
          }
        }
        Ok(())
      }
      AstNode::Assignment(assign) => self.assign_accumulator_to_target(&assign.target),
      AstNode::Property(prop) => {
        let val_reg = self.builder.allocate_register();
        self.builder.store_accumulator_in_register(val_reg)?;

        let named_prop = if !prop.is_computed {
          Self::extract_property_name(&prop.key)
        } else {
          None
        };
        let obj_reg = self.builder.allocate_register();
        self.visit_node(&prop.object)?;
        self.builder.store_accumulator_in_register(obj_reg)?;

        if let Some(ref name) = named_prop {
          self.builder.load_accumulator_with_register(val_reg)?;
          self.builder.set_named_property(obj_reg, name, 0)?;
        } else {
          let key_reg = self.builder.allocate_register();
          self.visit_node(&prop.key)?;
          self.builder.store_accumulator_in_register(key_reg)?;
          self.builder.load_accumulator_with_register(val_reg)?;
          self.builder.set_keyed_property(obj_reg, key_reg, 0)?;
          self.builder.free_register(key_reg);
        }
        self.builder.free_register(obj_reg);
        self.builder.free_register(val_reg);
        Ok(())
      }
      _ => Err(Error::UnsupportedAstNode("unsupported assignment target")),
    }
  }

  fn visit_switch_statement(&mut self, stmt: &SwitchStatement) -> Result<()> {
    self.visit_node(&stmt.tag)?;
    let tag_reg = self.builder.allocate_register();
    self.builder.store_accumulator_in_register(tag_reg)?;

    let break_label = self.builder.new_label();
    self.loop_stack.push(LoopScope {
      break_label,
      continue_target: None,
      labels: stmt.labels.clone(),
      bare_breakable: true,
    });

    let case_labels: Vec<BytecodeLabel> = stmt
      .cases
      .iter()
      .map(|_| self.builder.new_label())
      .collect();
    let mut default_idx = None;

    for (i, clause) in stmt.cases.iter().enumerate() {
      if let Some(ref label_node) = clause.label {
        self.visit_node(label_node)?;
        self
          .builder
          .compare_operation(AstCompareOp::EqStrict, tag_reg, 0)?;
        self
          .builder
          .jump_if_true(ToBooleanMode::AlreadyBoolean, case_labels[i])?;
      } else {
        default_idx = Some(i);
      }
    }

    self.builder.free_register(tag_reg);

    if let Some(idx) = default_idx {
      self.builder.jump(case_labels[idx])?;
    } else {
      self.builder.jump(break_label)?;
    }

    for (i, clause) in stmt.cases.iter().enumerate() {
      self.builder.bind(case_labels[i]);
      for s in &clause.statements {
        self.visit_node(s)?;
      }
    }

    self.loop_stack.pop();
    self.builder.bind(break_label);
    Ok(())
  }

  fn visit_throw(&mut self, stmt: &Throw) -> Result<()> {
    self.visit_node(&stmt.exception)?;
    self.builder.throw_op()?;
    Ok(())
  }

  fn visit_for_in_statement(&mut self, stmt: &ForInStatement) -> Result<()> {
    self.visit_node(&stmt.subject)?;
    let end_label = self.builder.new_label();
    self.builder.jump_if_undefined(end_label)?;
    self.builder.jump_if_null(end_label)?;

    let receiver = self.builder.allocate_register();
    self.builder.store_accumulator_in_register(receiver)?;

    let cache_type = self.builder.allocate_register();
    let cache_array = self.builder.allocate_register();
    let cache_length = self.builder.allocate_register();

    self.builder.for_in_enumerate(receiver)?;
    self.builder.for_in_prepare(cache_type, 0)?;

    let index = self.builder.allocate_register();
    self.builder.load_literal_smi(0)?;
    self.builder.store_accumulator_in_register(index)?;

    let mut loop_header = BytecodeLoopHeader::new();
    let continue_label = self.builder.new_label();

    self.builder.bind_loop_header(&mut loop_header);
    self
      .builder
      .jump_if_for_in_done(index, cache_length, end_label)?;

    self.builder.for_in_next(receiver, index, cache_type, 0)?;
    self.builder.jump_if_undefined(continue_label)?;

    self.assign_accumulator_to_target(&stmt.each)?;

    self.loop_stack.push(LoopScope {
      break_label: end_label,
      continue_target: Some(LoopContinueTarget::Label(continue_label)),
      labels: stmt.labels.clone(),
      bare_breakable: true,
    });

    self.visit_node(&stmt.body)?;

    self.loop_stack.pop();

    self.builder.bind(continue_label);
    self.builder.for_in_step(index)?;
    self.builder.jump_loop(&loop_header, 0, 0)?;

    self.builder.bind(end_label);

    self.builder.free_register(index);
    self.builder.free_register(cache_length);
    self.builder.free_register(cache_array);
    self.builder.free_register(cache_type);
    self.builder.free_register(receiver);
    Ok(())
  }

  fn visit_try_catch_statement(&mut self, stmt: &TryCatchStatement) -> Result<()> {
    let try_start = self.builder.current_bytecode_offset();

    for s in &stmt.try_block.statements {
      self.visit_node(s)?;
    }
    let try_end = self.builder.current_bytecode_offset();

    let exit_label = self.builder.new_label();
    self.builder.jump(exit_label)?;

    let catch_target = self.builder.current_bytecode_offset();
    if try_end > try_start {
      self
        .builder
        .mark_handler_entry(try_start, try_end, catch_target);
    }

    if let Some(ref var_name) = stmt.variable {
      let name = var_name.to_string_lossy();
      match self.bind_local(&name) {
        Some(reg) => self.builder.store_accumulator_in_register(reg)?,
        None => self.builder.store_global(&name, 0)?,
      }
    }

    for s in &stmt.catch_block.statements {
      self.visit_node(s)?;
    }

    self.builder.bind(exit_label);
    Ok(())
  }

  /// try/finally: 按出口路径各发射一份 finally 字节码, 实现原版 finalization block
  ///
  /// 原版用 token/result 续体寄存器 (JumpContinuation) 让正常、异常、返回各条路径共用一份
  /// finally; 现阶段没有续体寄存器, 所以每路发射一份 —— 单次执行只走其一, 语义等价。异常入口
  /// 先把累加器里的待抛值存入私有寄存器, 跑完 finally 再 Load + Throw 原样上抛, 使
  /// `try { throw x } finally {}` 与外层 catch 都能拿到原值; 返回入口把返回值留在续体寄存器里
  /// 过完 finally 再 `Return`, 于是 `try { return 1 } finally { return 3 }` 得到 3;
  /// break/continue 穿出本层时跳进按目标登记的续体副本, 收尾语句跑完再接着跳向外层续体或真实落点。
  /// 异常与正常两份副本无条件生成 (两条出口必然存在), 返回副本与 break/continue 续体只在
  /// 对应出口出现时才生成。
  fn visit_try_finally_statement(&mut self, stmt: &TryFinallyStatement) -> Result<()> {
    let try_start = self.builder.current_bytecode_offset();

    // try 体里出现 return 才需要续体寄存器与返回副本, 否则不付这份字节码体积
    let ret = if contains_return_in(&stmt.try_block.statements) {
      // 嵌套层复用外层的续体寄存器: 值只在其中一个槽里过一遍所有 finally
      let saved_reg = match self.finally_stack.last().and_then(|outer| outer.ret) {
        Some((_, reg)) => reg,
        None => self.builder.allocate_register(),
      };
      Some((self.builder.new_label(), saved_reg))
    } else {
      None
    };
    self.finally_stack.push(FinallyFrame {
      ret,
      loop_depth: self.loop_stack.len(),
      continuations: Vec::new(),
    });

    self.visit_statements(&stmt.try_block.statements)?;
    let try_end = self.builder.current_bytecode_offset();

    // 本层续体在此出栈: 副本里再出现的 `return` 属于 finally 自身, 外层帧仍在栈上继续接管
    let mut frame = self
      .finally_stack
      .pop()
      .ok_or(Error::UnsupportedAstNode("try-finally 帧不配平"))?;

    // 正常路径要落进 finally (而不是像 catch 那样跳过), 所以跳转目标是 finally 的正常入口
    let normal_entry = self.builder.new_label();
    self.builder.jump(normal_entry)?;

    let handler_target = self.builder.current_bytecode_offset();
    if try_end > try_start {
      self
        .builder
        .mark_handler_entry(try_start, try_end, handler_target);
    }
    let pending_reg = self.builder.allocate_register();
    self.builder.store_accumulator_in_register(pending_reg)?;
    self.emit_finally_body(&stmt.finally_block)?;
    self.builder.load_accumulator_with_register(pending_reg)?;
    self.builder.throw_op()?;
    self.builder.free_register(pending_reg);

    self.builder.bind(normal_entry);
    self.emit_finally_body(&stmt.finally_block)?;

    // 正常出口后面还排着返回与 break/continue 的副本, 收尾语句跑完必须跳过它们, 否则会顺路
    // 再执行一遍别人的出口
    if frame.ret.is_some() || !frame.continuations.is_empty() {
      let exit_label = self.builder.new_label();
      self.builder.jump(exit_label)?;

      if let Some((return_label, saved_reg)) = frame.ret {
        self.builder.bind(return_label);
        self.emit_finally_body(&stmt.finally_block)?;
        // 外层还包着 finally 就继续向外交付, 由最外层的副本真正离帧
        match self.finally_stack.last().and_then(|outer| outer.ret) {
          Some((outer_label, _)) => self.builder.jump(outer_label)?,
          None => {
            self.builder.load_accumulator_with_register(saved_reg)?;
            self.builder.return_op()?;
            self.builder.free_register(saved_reg);
          }
        }
      }

      // break/continue 的续体副本: 收尾语句跑完接外层续体, 只有最外一层的副本落到真实目标
      while let Some(cont) = frame.continuations.pop() {
        self.builder.bind(cont.entry);
        self.emit_finally_body(&stmt.finally_block)?;
        match cont.next {
          Some(next_label) => self.builder.jump(next_label)?,
          None => cont.dest.emit(&mut self.builder)?,
        }
      }

      self.builder.bind(exit_label);
    }
    Ok(())
  }

  /// 发射一份 finally 收尾语句 (正常、异常、返回、break/continue 各路径共用其副本)
  fn emit_finally_body(&mut self, block: &Block) -> Result<()> {
    for s in &block.statements {
      self.visit_node(s)?;
    }
    Ok(())
  }

  fn visit_conditional(&mut self, cond: &Conditional) -> Result<()> {
    let else_label = self.builder.new_label();
    let end_label = self.builder.new_label();

    self.visit_node(&cond.condition)?;
    self
      .builder
      .jump_if_false(ToBooleanMode::ConvertToBoolean, else_label)?;

    self.visit_node(&cond.then_expression)?;
    self.builder.jump(end_label)?;

    self.builder.bind(else_label);
    self.visit_node(&cond.else_expression)?;

    self.builder.bind(end_label);
    Ok(())
  }

  fn visit_property(&mut self, prop: &Property) -> Result<()> {
    let obj_reg = self.builder.allocate_register();
    self.visit_node(&prop.object)?;
    self.builder.store_accumulator_in_register(obj_reg)?;

    if !prop.is_computed
      && let Some(name) = Self::extract_property_name(&prop.key)
    {
      self.builder.load_named_property(obj_reg, &name, 0)?;
      self.builder.free_register(obj_reg);
      return Ok(());
    }

    self.visit_node(&prop.key)?;
    self.builder.load_keyed_property(obj_reg, 0)?;
    self.builder.free_register(obj_reg);
    Ok(())
  }

  fn visit_call(&mut self, call: &Call) -> Result<()> {
    if let AstNode::Property(prop) = &*call.expression {
      let receiver_reg = self.builder.allocate_register();
      self.visit_node(&prop.object)?;
      self.builder.store_accumulator_in_register(receiver_reg)?;

      let callable_reg = self.builder.allocate_register();
      if !prop.is_computed
        && let Some(name) = Self::extract_property_name(&prop.key)
      {
        self.builder.load_named_property(receiver_reg, &name, 0)?;
      } else {
        self.visit_node(&prop.key)?;
        self.builder.load_keyed_property(receiver_reg, 0)?;
      }
      self.builder.store_accumulator_in_register(callable_reg)?;

      let arg_count = call.arguments.len();
      match arg_count {
        0 => {
          self
            .builder
            .call_property(callable_reg, receiver_reg, &[], 0)?;
        }
        1 => {
          let arg1 = self.builder.allocate_register();
          self.visit_node(&call.arguments[0])?;
          self.builder.store_accumulator_in_register(arg1)?;
          self
            .builder
            .call_property(callable_reg, receiver_reg, &[arg1], 0)?;
          self.builder.free_register(arg1);
        }
        2 => {
          let arg1 = self.builder.allocate_register();
          self.visit_node(&call.arguments[0])?;
          self.builder.store_accumulator_in_register(arg1)?;

          let arg2 = self.builder.allocate_register();
          self.visit_node(&call.arguments[1])?;
          self.builder.store_accumulator_in_register(arg2)?;

          self
            .builder
            .call_property(callable_reg, receiver_reg, &[arg1, arg2], 0)?;
          self.builder.free_register(arg2);
          self.builder.free_register(arg1);
        }
        _ => {
          let mut arg_regs = Vec::with_capacity(arg_count + 1);
          let first_reg = self.builder.allocate_register();
          arg_regs.push(first_reg);
          for _ in 1..=arg_count {
            arg_regs.push(self.builder.allocate_register());
          }
          self.builder.load_accumulator_with_register(receiver_reg)?;
          self.builder.store_accumulator_in_register(first_reg)?;
          for (i, arg) in call.arguments.iter().enumerate() {
            self.visit_node(arg)?;
            self
              .builder
              .store_accumulator_in_register(arg_regs[i + 1])?;
          }
          self
            .builder
            .call_property(callable_reg, first_reg, &arg_regs[1..], 0)?;
          for reg in arg_regs.into_iter().rev() {
            self.builder.free_register(reg);
          }
        }
      }

      self.builder.free_register(callable_reg);
      self.builder.free_register(receiver_reg);
      Ok(())
    } else {
      let callable_reg = self.builder.allocate_register();
      self.visit_node(&call.expression)?;
      self.builder.store_accumulator_in_register(callable_reg)?;

      let arg_count = call.arguments.len();
      match arg_count {
        0 => {
          self.builder.call_undefined_receiver(callable_reg, &[], 0)?;
        }
        1 => {
          let arg1 = self.builder.allocate_register();
          self.visit_node(&call.arguments[0])?;
          self.builder.store_accumulator_in_register(arg1)?;
          self
            .builder
            .call_undefined_receiver(callable_reg, &[arg1], 0)?;
          self.builder.free_register(arg1);
        }
        2 => {
          let arg1 = self.builder.allocate_register();
          self.visit_node(&call.arguments[0])?;
          self.builder.store_accumulator_in_register(arg1)?;

          let arg2 = self.builder.allocate_register();
          self.visit_node(&call.arguments[1])?;
          self.builder.store_accumulator_in_register(arg2)?;

          self
            .builder
            .call_undefined_receiver(callable_reg, &[arg1, arg2], 0)?;
          self.builder.free_register(arg2);
          self.builder.free_register(arg1);
        }
        _ => {
          let mut arg_regs = Vec::with_capacity(arg_count);
          for _ in 0..arg_count {
            arg_regs.push(self.builder.allocate_register());
          }
          for (i, arg) in call.arguments.iter().enumerate() {
            self.visit_node(arg)?;
            self.builder.store_accumulator_in_register(arg_regs[i])?;
          }
          self
            .builder
            .call_undefined_receiver(callable_reg, &arg_regs, 0)?;
          for reg in arg_regs.into_iter().rev() {
            self.builder.free_register(reg);
          }
        }
      }

      self.builder.free_register(callable_reg);
      Ok(())
    }
  }

  fn visit_call_new(&mut self, cn: &CallNew) -> Result<()> {
    let constructor_reg = self.builder.allocate_register();
    self.visit_node(&cn.expression)?;
    self
      .builder
      .store_accumulator_in_register(constructor_reg)?;

    let arg_count = cn.arguments.len();
    if arg_count == 0 {
      self.builder.construct(constructor_reg, &[], 0)?;
    } else {
      let mut arg_regs = Vec::with_capacity(arg_count);
      for _ in 0..arg_count {
        arg_regs.push(self.builder.allocate_register());
      }
      for (i, arg) in cn.arguments.iter().enumerate() {
        self.visit_node(arg)?;
        self.builder.store_accumulator_in_register(arg_regs[i])?;
      }
      self.builder.construct(constructor_reg, &arg_regs, 0)?;
      for reg in arg_regs.into_iter().rev() {
        self.builder.free_register(reg);
      }
    }

    self.builder.free_register(constructor_reg);
    Ok(())
  }

  fn visit_array_literal(&mut self, arr: &ArrayLiteral) -> Result<()> {
    if arr.values.is_empty() {
      self.builder.create_empty_array_literal(0)?;
      return Ok(());
    }
    self.builder.create_empty_array_literal(0)?;
    let arr_reg = self.builder.allocate_register();
    self.builder.store_accumulator_in_register(arr_reg)?;

    let idx_reg = self.builder.allocate_register();
    for (i, val) in arr.values.iter().enumerate() {
      self.builder.load_literal_smi(i as i32)?;
      self.builder.store_accumulator_in_register(idx_reg)?;
      self.visit_node(val)?;
      self.builder.store_in_array_literal(arr_reg, idx_reg, 0)?;
    }
    self.builder.free_register(idx_reg);
    self.builder.load_accumulator_with_register(arr_reg)?;
    self.builder.free_register(arr_reg);
    Ok(())
  }

  fn visit_object_literal(&mut self, obj: &ObjectLiteral) -> Result<()> {
    if obj.properties.is_empty() {
      self.builder.create_empty_object_literal()?;
      return Ok(());
    }
    // 属性数在生成期已知, 交给 Flag8 让执行期一次铺好表的容量。原版这里是
    // `CreateObjectLiteral(常量池里的属性名数组, literal_index, 注解)`, 对象容量由 Map 的
    // in-object 槽位带出来; 本端口没有 FixedArray 常量, 属性名仍逐个走 Define* 臂, 唯独把
    // 容量这一项补上 —— 否则逐属性 insert 会在中途 reserve_rehash (profile 里占整条链 11%)
    self
      .builder
      .create_object_literal(0, 0, obj.properties.len().min(u8::MAX as usize) as u8)?;
    let obj_reg = self.builder.allocate_register();
    self.builder.store_accumulator_in_register(obj_reg)?;

    for prop in &obj.properties {
      if !prop.is_computed
        && let Some(name) = Self::extract_property_name(&prop.key)
      {
        self.visit_node(&prop.value)?;
        self
          .builder
          .define_named_property_in_literal(obj_reg, &name, 0)?;
        continue;
      }
      let val_reg = self.builder.allocate_register();
      self.visit_node(&prop.value)?;
      self.builder.store_accumulator_in_register(val_reg)?;

      let key_reg = self.builder.allocate_register();
      self.visit_node(&prop.key)?;
      self.builder.store_accumulator_in_register(key_reg)?;

      self.builder.load_accumulator_with_register(val_reg)?;
      self.builder.define_keyed_property(obj_reg, key_reg, 0)?;
      self.builder.free_register(key_reg);
      self.builder.free_register(val_reg);
    }
    self.builder.load_accumulator_with_register(obj_reg)?;
    self.builder.free_register(obj_reg);
    Ok(())
  }

  fn visit_function_literal(&mut self, func: &FunctionLiteral) -> Result<()> {
    let (child_bytecode, captured_locals, rest_binding) =
      Self::generate_function_with_locals(func)?;
    let uses_arguments = child_bytecode
      .constants()
      .iter()
      .any(|entry| matches!(entry, ConstantPoolEntry::String(s) if &**s == "arguments"));
    let interp_func = Arc::new(InterpFunction {
      name: func.name.as_ref().map(|n| n.to_string_lossy()),
      params: func.params.iter().map(|p| p.to_string_lossy()).collect(),
      captured_locals,
      bytecode_array: child_bytecode,
      uses_arguments,
      is_arrow: func.is_arrow,
      src: func.src.clone(),
      rest_binding,
    });
    self
      .builder
      .load_constant(ConstantPoolEntry::Function(interp_func))?;
    Ok(())
  }

  fn visit_function_declaration(&mut self, _decl: &FunctionDeclaration) -> Result<()> {
    // 内部函数声明已在作用域入口预分析提升发射，遍历时跳过避免重复执行
    Ok(())
  }

  fn visit_this_expression(&mut self, _this_expr: &ThisExpression) -> Result<()> {
    let reg = self
      .locals
      .get("this")
      .copied()
      .unwrap_or_else(Register::receiver);
    self.builder.load_accumulator_with_register(reg)?;
    Ok(())
  }
}

// —— 提取不变式断言 ——
const _: () = {
  assert!(TypeHint::String as u8 == 6);
  assert!(TypeHint::Any as u8 == 7);
  assert!(TypeHint::is_same_or_sub_type(
    TypeHint::Any,
    TypeHint::Boolean
  ));
  assert!(!TypeHint::is_same_or_sub_type(
    TypeHint::Boolean,
    TypeHint::Any
  ));
  assert!(TypeHint::is_string_type(TypeHint::InternalizedString));
  assert!(matches!(
    TypeHint::Boolean.to_boolean_mode(),
    ToBooleanMode::AlreadyBoolean
  ));
};
