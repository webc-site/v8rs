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
    SwitchStatement, ThisExpression, Throw, TryCatchStatement, UnaryOperation, VariableDeclaration,
    VariableProxy, WhileStatement,
  },
  value::AstValue,
};

use crate::{
  bytecode_array_builder::{
    BytecodeArray, BytecodeArrayBuilder, ConstantPoolEntry, InterpFunction, ToBooleanMode,
  },
  bytecode_label::{BytecodeLabel, BytecodeLoopHeader},
  bytecode_register::Register,
  error::{Error, Result},
};

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
    AstNode::Throw(t) => collect_var_refs(&t.exception, refs),
    _ => {}
  }
}

/// 真实的 AST 字节码生成器 (BytecodeGenerator)
pub struct BytecodeGenerator {
  builder: BytecodeArrayBuilder,
  locals: HashMap<String, Register>,
  loop_stack: Vec<LoopScope>,
  is_top_level: bool,
  captured_vars: HashSet<String>,
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
      is_top_level,
      captured_vars,
    }
  }

  /// 编译通用 AST 节点生成 BytecodeArray
  pub fn generate(ast: &AstNode) -> Result<BytecodeArray> {
    let mut generator = Self::new(0, 0, true, HashSet::default());
    if let AstNode::Block(b) = ast {
      generator.hoist_declarations(&b.statements)?;
      for stmt in &b.statements {
        generator.visit_node(stmt)?;
      }
    } else {
      generator.visit_node(ast)?;
    }
    generator.builder.load_undefined()?;
    generator.builder.return_op()?;
    Ok(generator.builder.build())
  }

  /// 编译函数字面量 AST 生成 BytecodeArray (包含 receiver 与形参)
  pub fn generate_function(literal: &FunctionLiteral) -> Result<BytecodeArray> {
    Self::generate_function_with_locals(literal).map(|(b, _)| b)
  }

  /// 编译函数字面量 AST 生成 BytecodeArray 以及当前函数内所有声明的局部变量与形参名
  pub fn generate_function_with_locals(
    literal: &FunctionLiteral,
  ) -> Result<(BytecodeArray, Vec<String>)> {
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
      generator.locals.insert(param_name.clone(), reg);
      if generator.captured_vars.contains(&param_name) {
        generator.builder.load_accumulator_with_register(reg)?;
        generator.builder.store_global(&param_name, 0)?;
      }
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
    for stmt in &literal.body {
      collect_local_defs(stmt, &mut my_defs);
    }
    let captured_locals: Vec<String> = my_defs
      .into_iter()
      .filter(|name| generator.captured_vars.contains(name))
      .collect();
    Ok((generator.builder.build(), captured_locals))
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
      AstNode::BreakStatement(_) => self.visit_break(),
      AstNode::ContinueStatement(_) => self.visit_continue(),
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
      AstNode::Throw(t) => self.visit_throw(t),
      _ => Ok(()),
    }
  }

  fn hoist_declarations(&mut self, stmts: &[AstNode]) -> Result<()> {
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
      let (child_bytecode, captured_locals) = Self::generate_function_with_locals(fun)?;
      let uses_arguments = child_bytecode
        .constants()
        .iter()
        .any(|entry| matches!(entry, ConstantPoolEntry::String(s) if s == "arguments"));
      let interp_func = Arc::new(InterpFunction {
        name: fun.name.as_ref().map(|n| n.to_string_lossy()),
        params: fun.params.iter().map(|p| p.to_string_lossy()).collect(),
        captured_locals,
        bytecode_array: child_bytecode,
        uses_arguments,
      });
      self
        .builder
        .load_constant(ConstantPoolEntry::Function(interp_func))?;

      if let Some(name) = &fun.name {
        let s = name.to_string_lossy();
        if !self.is_top_level && !self.captured_vars.contains(&s) {
          let reg = if let Some(&r) = self.locals.get(&s) {
            r
          } else {
            let r = self.builder.allocate_register();
            self.locals.insert(s.clone(), r);
            r
          };
          self.builder.store_accumulator_in_register(reg)?;
        }
        self.builder.store_global(&s, 0)?;
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
      if !self.locals.contains_key(&name) {
        let reg = self.builder.allocate_register();
        self.locals.insert(name, reg);
      }
    }
    Ok(())
  }

  fn visit_block(&mut self, block: &Block) -> Result<()> {
    for stmt in &block.statements {
      self.visit_node(stmt)?;
    }
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
        if num.fract() == 0.0 && *num >= (i32::MIN as f64) && *num <= (i32::MAX as f64) {
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
          .load_constant(ConstantPoolEntry::String(s.to_string_lossy()))?;
      }
      AstValue::ConsString(cs) => {
        self
          .builder
          .load_constant(ConstantPoolEntry::String(cs.to_string()))?;
      }
      AstValue::BigInt(bi) => {
        self
          .builder
          .load_constant(ConstantPoolEntry::BigInt(bi.as_str().to_string()))?;
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
          if self.is_top_level || self.captured_vars.contains(&var_name) {
            self.builder.store_global(&var_name, 0)?;
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
            if self.is_top_level || self.captured_vars.contains(&var_name) {
              self.builder.store_global(&var_name, 0)?;
            }
          } else {
            self.builder.load_accumulator_with_register(reg)?;
            let old_reg = self.builder.allocate_register();
            self.builder.store_accumulator_in_register(old_reg)?;
            self.builder.count_operation(count.op, 0)?;
            self.builder.store_accumulator_in_register(reg)?;
            if self.is_top_level || self.captured_vars.contains(&var_name) {
              self.builder.store_global(&var_name, 0)?;
            }
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
    let end_label = self.builder.new_label();
    if let Some(else_stmt) = &stmt.else_statement {
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
    self.builder.return_op()?;
    Ok(())
  }

  fn visit_break(&mut self) -> Result<()> {
    if let Some(scope) = self.loop_stack.last() {
      self.builder.jump(scope.break_label)?;
      Ok(())
    } else {
      Err(Error::UnsupportedAstNode("break outside of loop"))
    }
  }

  fn visit_continue(&mut self) -> Result<()> {
    for scope in self.loop_stack.iter().rev() {
      if let Some(target) = &scope.continue_target {
        match target {
          LoopContinueTarget::Header(header) => {
            self.builder.jump_loop(header, 0, 0)?;
          }
          LoopContinueTarget::Label(label) => {
            self.builder.jump(*label)?;
          }
        }
        return Ok(());
      }
    }
    Err(Error::UnsupportedAstNode("continue outside of loop"))
  }

  fn assign_accumulator_to_target(&mut self, target: &AstNode) -> Result<()> {
    match target {
      AstNode::VariableProxy(vp) => {
        let var_name = vp.name.to_string_lossy();
        if let Some(&reg) = self.locals.get(&var_name) {
          self.builder.store_accumulator_in_register(reg)?;
          if self.is_top_level || self.captured_vars.contains(&var_name) {
            self.builder.store_global(&var_name, 0)?;
          }
        } else {
          self.builder.store_global(&var_name, 0)?;
        }
        Ok(())
      }
      AstNode::VariableDeclaration(decl) => {
        if let Some(var) = &decl.var {
          let var_name = var.to_string_lossy();
          if !self.is_top_level {
            let reg = if let Some(&r) = self.locals.get(&var_name) {
              r
            } else {
              let r = self.builder.allocate_register();
              self.locals.insert(var_name.clone(), r);
              r
            };
            self.builder.store_accumulator_in_register(reg)?;
          }
          if self.is_top_level || self.captured_vars.contains(&var_name) {
            self.builder.store_global(&var_name, 0)?;
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
      let reg = if let Some(&r) = self.locals.get(&name) {
        r
      } else {
        let r = self.builder.allocate_register();
        self.locals.insert(name, r);
        r
      };
      self.builder.store_accumulator_in_register(reg)?;
    }

    for s in &stmt.catch_block.statements {
      self.visit_node(s)?;
    }

    self.builder.bind(exit_label);
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
    self.builder.create_empty_object_literal()?;
    let obj_reg = self.builder.allocate_register();
    self.builder.store_accumulator_in_register(obj_reg)?;

    for prop in &obj.properties {
      if !prop.is_computed
        && let Some(name) = Self::extract_property_name(&prop.key)
      {
        self.visit_node(&prop.value)?;
        self.builder.set_named_property(obj_reg, &name, 0)?;
        continue;
      }
      let val_reg = self.builder.allocate_register();
      self.visit_node(&prop.value)?;
      self.builder.store_accumulator_in_register(val_reg)?;

      let key_reg = self.builder.allocate_register();
      self.visit_node(&prop.key)?;
      self.builder.store_accumulator_in_register(key_reg)?;

      self.builder.load_accumulator_with_register(val_reg)?;
      self.builder.set_keyed_property(obj_reg, key_reg, 0)?;
      self.builder.free_register(key_reg);
      self.builder.free_register(val_reg);
    }
    self.builder.load_accumulator_with_register(obj_reg)?;
    self.builder.free_register(obj_reg);
    Ok(())
  }

  fn visit_function_literal(&mut self, func: &FunctionLiteral) -> Result<()> {
    let (child_bytecode, captured_locals) = Self::generate_function_with_locals(func)?;
    let uses_arguments = child_bytecode
      .constants()
      .iter()
      .any(|entry| matches!(entry, ConstantPoolEntry::String(s) if s == "arguments"));
    let interp_func = Arc::new(InterpFunction {
      name: func.name.as_ref().map(|n| n.to_string_lossy()),
      params: func.params.iter().map(|p| p.to_string_lossy()).collect(),
      captured_locals,
      bytecode_array: child_bytecode,
      uses_arguments,
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
