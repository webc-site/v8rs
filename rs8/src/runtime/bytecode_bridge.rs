//! Ignition 字节码与虚拟机架构桥接器
//!
//! 将 rs8 语法树编译为扁平连续的 Ignition BytecodeArray，
//! 并由 BytecodeDispatcher 寄存器虚拟机直接分派求值，彻底打破 AST Walker 递归瓶颈。

use std::{borrow::Cow, sync::Arc};

use gxhash::{HashMap, HashSet};
use itoa::Buffer;
use parking_lot::RwLock;
use v8_ast::{
  CatchPrediction, VariableMode,
  ast::{
    AstAssignOp, AstBinaryOp, AstCompareOp, AstCountOp, AstNode, AstNodeBase, AstNodeFactory,
    AstNodeType, AstUnaryOp, CallType, CaseClause, EagerCompileHint, FunctionLiteral,
    FunctionSyntaxKind, HoleCheckMode, NO_SOURCE_POSITION, ObjectLiteralProperty,
    ObjectPropertyKind, ReturnType, SpreadPosition, VariableProxyBindingMode,
  },
  value::{AstRawString, AstValue},
};
use v8_interpreter::{
  BytecodeDispatcher, BytecodeGenerator, Error as V8InterpreterError, NativeFn as InterpNativeFn,
  Value as InterpreterValue,
};

use crate::{
  error::{Error, Result},
  runtime::{
    ast::{AssignOp, BinaryOp, Expr, ObjectKey, Stmt, UnaryOp},
    environment::Environment,
    interpreter::Interpreter,
    value::{
      EXEC_KEY, FLAGS_KEY, FunctionData, GLOBAL_KEY, JsValue, LAST_INDEX_KEY, MATCH_KEY, NativeFn,
      PROTOTYPE_KEY, REPLACE_KEY, SOURCE_KEY, SPLIT_KEY, SharedFunctionInfo, TEST_KEY,
    },
  },
};

/// 收集单个语句中所有声明与赋值的局部变量名 (零堆分配，纯切片借用)
fn collect_stmt_vars<'a>(stmt: &'a Stmt, locals: &mut HashSet<&'a str>) {
  match stmt {
    Stmt::VarDecl { name, .. } => {
      locals.insert(name.as_str());
    }
    Stmt::VarDecls(decls) => {
      for (name, _) in decls {
        locals.insert(name.as_str());
      }
    }
    Stmt::FunctionDecl(shared) => {
      if let Some(ref name) = shared.name {
        locals.insert(name.as_str());
      }
    }
    Stmt::Block(inner) => {
      collect_stmts_vars(inner, locals);
    }
    Stmt::If {
      then_branch,
      else_branch,
      ..
    } => {
      collect_stmt_vars(then_branch, locals);
      if let Some(eb) = else_branch {
        collect_stmt_vars(eb, locals);
      }
    }
    Stmt::While { body, .. } | Stmt::DoWhile { body, .. } => {
      collect_stmt_vars(body, locals);
    }
    Stmt::For { init, body, .. } => {
      if let Some(in_stmt) = init {
        collect_stmt_vars(in_stmt, locals);
      }
      collect_stmt_vars(body, locals);
    }
    Stmt::Expr(expr) => {
      collect_expr_assigned_vars(expr, locals);
    }
    Stmt::ForIn { variable, body, .. } => {
      locals.insert(variable.as_str());
      collect_stmt_vars(body, locals);
    }
    Stmt::Switch { cases, .. } => {
      for c in cases {
        collect_stmts_vars(&c.body, locals);
      }
    }
    Stmt::Try {
      block,
      catch_param,
      catch_block,
      finally_block,
    } => {
      collect_stmts_vars(block, locals);
      if let Some(param) = catch_param {
        locals.insert(param.as_str());
      }
      if let Some(cb) = catch_block {
        collect_stmts_vars(cb, locals);
      }
      if let Some(fb) = finally_block {
        collect_stmts_vars(fb, locals);
      }
    }
    Stmt::Throw(expr) => {
      collect_expr_assigned_vars(expr, locals);
    }
    _ => {}
  }
}

/// 收集语句列表中所有声明与赋值的局部变量名 (零堆分配)
fn collect_stmts_vars<'a>(stmts: &'a [Stmt], locals: &mut HashSet<&'a str>) {
  for s in stmts {
    collect_stmt_vars(s, locals);
  }
}

#[inline]
fn collect_expr_assigned_vars<'a>(expr: &'a Expr, locals: &mut HashSet<&'a str>) {
  if let Expr::Assign { target, .. } = expr
    && let Expr::Identifier(name) = &**target
  {
    locals.insert(name.as_str());
  }
}

/// 尝试将运行时表达式编译为 v8-ast 语法节点 (带作用域定义域检查)
pub fn expr_to_v8_ast_scoped(
  expr: &Expr,
  locals: &HashSet<&str>,
  globals: &HashSet<&str>,
) -> Option<AstNode> {
  match expr {
    Expr::Literal(val) => match val {
      JsValue::Number(n) => {
        if n.fract() == 0.0 && *n >= (i32::MIN as f64) && *n <= (i32::MAX as f64) {
          Some(AstNode::Literal(AstNodeFactory::new_literal(
            0,
            AstValue::Smi(*n as i32),
          )))
        } else {
          Some(AstNode::Literal(AstNodeFactory::new_literal(
            0,
            AstValue::Number(*n),
          )))
        }
      }
      JsValue::Boolean(b) => Some(AstNode::Literal(AstNodeFactory::new_literal(
        0,
        AstValue::Boolean(*b),
      ))),
      JsValue::String(s) => Some(AstNode::Literal(AstNodeFactory::new_literal(
        0,
        AstValue::String(AstRawString::from_utf8(s)),
      ))),
      JsValue::Null => Some(AstNode::Literal(AstNodeFactory::new_literal(
        0,
        AstValue::Null,
      ))),
      JsValue::Undefined => Some(AstNode::Literal(AstNodeFactory::new_literal(
        0,
        AstValue::Undefined,
      ))),
      JsValue::RegExp(re) => {
        let callee = AstNode::VariableProxy(AstNodeFactory::new_variable_proxy(
          0,
          AstRawString::from_utf8("$$RegExp"),
          VariableProxyBindingMode::Bound,
          HoleCheckMode::Elided,
        ));
        let arg0 = AstNode::Literal(AstNodeFactory::new_literal(
          0,
          AstValue::String(AstRawString::from_utf8(&re.pattern)),
        ));
        let arg1 = AstNode::Literal(AstNodeFactory::new_literal(
          0,
          AstValue::String(AstRawString::from_utf8(&re.flags)),
        ));
        Some(AstNode::CallNew(AstNodeFactory::new_call_new(
          0,
          callee,
          vec![arg0, arg1],
          SpreadPosition::NoSpread,
        )))
      }
      _ => None,
    },

    Expr::Identifier(name) => Some(AstNode::VariableProxy(AstNodeFactory::new_variable_proxy(
      0,
      AstRawString::from_utf8(name),
      VariableProxyBindingMode::Bound,
      HoleCheckMode::Elided,
    ))),

    Expr::Member { object, property } => {
      let obj_node = expr_to_v8_ast_scoped(object, locals, globals)?;
      let key_node = AstNode::Literal(AstNodeFactory::new_literal(
        0,
        AstValue::String(AstRawString::from_utf8(property)),
      ));
      Some(AstNode::Property(AstNodeFactory::new_property(
        0, obj_node, key_node, false, false,
      )))
    }

    Expr::Index { object, index } => {
      let obj_node = expr_to_v8_ast_scoped(object, locals, globals)?;
      let idx_node = expr_to_v8_ast_scoped(index, locals, globals)?;
      Some(AstNode::Property(AstNodeFactory::new_property(
        0, obj_node, idx_node, true, false,
      )))
    }

    Expr::Call { callee, args } => {
      let callee_node = expr_to_v8_ast_scoped(callee, locals, globals)?;
      let mut arg_nodes = Vec::with_capacity(args.len());
      for a in args {
        arg_nodes.push(expr_to_v8_ast_scoped(a, locals, globals)?);
      }
      Some(AstNode::Call(AstNodeFactory::new_call(
        0,
        callee_node,
        arg_nodes,
        CallType::OrdinaryCall,
        SpreadPosition::NoSpread,
      )))
    }

    Expr::New { callee, args } => {
      let callee_node = expr_to_v8_ast_scoped(callee, locals, globals)?;
      let mut arg_nodes = Vec::with_capacity(args.len());
      for a in args {
        arg_nodes.push(expr_to_v8_ast_scoped(a, locals, globals)?);
      }
      Some(AstNode::CallNew(AstNodeFactory::new_call_new(
        0,
        callee_node,
        arg_nodes,
        SpreadPosition::NoSpread,
      )))
    }

    Expr::Array(items) => {
      let mut item_nodes = Vec::with_capacity(items.len());
      for it in items {
        item_nodes.push(expr_to_v8_ast_scoped(it, locals, globals)?);
      }
      Some(AstNode::ArrayLiteral(AstNodeFactory::new_array_literal(
        0, item_nodes,
      )))
    }

    Expr::Object(entries) => {
      let mut prop_nodes = Vec::with_capacity(entries.len());
      for (k, v) in entries {
        let (key_node, is_computed) = match k {
          ObjectKey::Ident(s) => (
            AstNode::Literal(AstNodeFactory::new_literal(
              0,
              AstValue::String(AstRawString::from_utf8(s)),
            )),
            false,
          ),
          ObjectKey::Computed(expr) => (expr_to_v8_ast_scoped(expr, locals, globals)?, true),
        };
        let val_node = expr_to_v8_ast_scoped(v, locals, globals)?;
        prop_nodes.push(ObjectLiteralProperty::new(
          ObjectPropertyKind::Constant,
          key_node,
          val_node,
          is_computed,
        ));
      }
      Some(AstNode::ObjectLiteral(AstNodeFactory::new_object_literal(
        0, prop_nodes,
      )))
    }

    Expr::This => Some(AstNode::ThisExpression(
      AstNodeFactory::new_this_expression(0),
    )),

    Expr::FunctionExpr(shared) => {
      let mut params = Vec::with_capacity(shared.params.len());
      for p in &shared.params {
        params.push(AstRawString::from_utf8(p));
      }

      let mut body = Vec::with_capacity(shared.body.len());
      for s in &shared.body {
        body.push(stmt_to_v8_ast_scoped(s, locals, globals)?);
      }

      Some(AstNode::FunctionLiteral(FunctionLiteral {
        base: AstNodeBase::new(0, AstNodeType::FunctionLiteral),
        name: shared.name.as_deref().map(AstRawString::from_utf8),
        inferred_name: None,
        scope: None,
        params,
        body,
        syntax_kind: FunctionSyntaxKind::AnonymousExpression,
        compile_hint: EagerCompileHint::ShouldLazyCompile,
        is_async: false,
        is_generator: false,
        parameter_count: shared.params.len() as i32,
        function_length: shared.params.len() as i32,
        expected_property_count: 0,
        function_token_position: NO_SOURCE_POSITION,
        suspend_count: 0,
        function_literal_id: 0,
        has_duplicate_parameters: false,
        has_braces: true,
        pretenure: false,
        requires_instance_members_initializer: false,
        has_static_private_methods_or_accessors: false,
        preparse_data: None,
      }))
    }

    Expr::Binary { op, left, right } => {
      let lhs = expr_to_v8_ast_scoped(left, locals, globals)?;
      let rhs = expr_to_v8_ast_scoped(right, locals, globals)?;

      match op {
        BinaryOp::Add => Some(AstNode::BinaryOperation(
          AstNodeFactory::new_binary_operation(0, AstBinaryOp::Add, lhs, rhs),
        )),
        BinaryOp::Sub => Some(AstNode::BinaryOperation(
          AstNodeFactory::new_binary_operation(0, AstBinaryOp::Sub, lhs, rhs),
        )),
        BinaryOp::Mul => Some(AstNode::BinaryOperation(
          AstNodeFactory::new_binary_operation(0, AstBinaryOp::Mul, lhs, rhs),
        )),
        BinaryOp::Div => Some(AstNode::BinaryOperation(
          AstNodeFactory::new_binary_operation(0, AstBinaryOp::Div, lhs, rhs),
        )),
        BinaryOp::Mod => Some(AstNode::BinaryOperation(
          AstNodeFactory::new_binary_operation(0, AstBinaryOp::Mod, lhs, rhs),
        )),
        BinaryOp::BitAnd => Some(AstNode::BinaryOperation(
          AstNodeFactory::new_binary_operation(0, AstBinaryOp::BitAnd, lhs, rhs),
        )),
        BinaryOp::BitOr => Some(AstNode::BinaryOperation(
          AstNodeFactory::new_binary_operation(0, AstBinaryOp::BitOr, lhs, rhs),
        )),
        BinaryOp::BitXor => Some(AstNode::BinaryOperation(
          AstNodeFactory::new_binary_operation(0, AstBinaryOp::BitXor, lhs, rhs),
        )),
        BinaryOp::Shl => Some(AstNode::BinaryOperation(
          AstNodeFactory::new_binary_operation(0, AstBinaryOp::Shl, lhs, rhs),
        )),
        BinaryOp::Shr => Some(AstNode::BinaryOperation(
          AstNodeFactory::new_binary_operation(0, AstBinaryOp::Sar, lhs, rhs),
        )),
        BinaryOp::Sar => Some(AstNode::BinaryOperation(
          AstNodeFactory::new_binary_operation(0, AstBinaryOp::Shr, lhs, rhs),
        )),
        BinaryOp::LogicalAnd => Some(AstNode::BinaryOperation(
          AstNodeFactory::new_binary_operation(0, AstBinaryOp::And, lhs, rhs),
        )),
        BinaryOp::LogicalOr => Some(AstNode::BinaryOperation(
          AstNodeFactory::new_binary_operation(0, AstBinaryOp::Or, lhs, rhs),
        )),

        BinaryOp::Lt => Some(AstNode::CompareOperation(
          AstNodeFactory::new_compare_operation(0, AstCompareOp::LessThan, lhs, rhs),
        )),
        BinaryOp::Lte => Some(AstNode::CompareOperation(
          AstNodeFactory::new_compare_operation(0, AstCompareOp::LessThanEq, lhs, rhs),
        )),
        BinaryOp::Gt => Some(AstNode::CompareOperation(
          AstNodeFactory::new_compare_operation(0, AstCompareOp::GreaterThan, lhs, rhs),
        )),
        BinaryOp::Gte => Some(AstNode::CompareOperation(
          AstNodeFactory::new_compare_operation(0, AstCompareOp::GreaterThanEq, lhs, rhs),
        )),
        BinaryOp::Eq => Some(AstNode::CompareOperation(
          AstNodeFactory::new_compare_operation(0, AstCompareOp::Eq, lhs, rhs),
        )),
        BinaryOp::NotEq => Some(AstNode::CompareOperation(
          AstNodeFactory::new_compare_operation(0, AstCompareOp::NotEq, lhs, rhs),
        )),
        BinaryOp::StrictEq => Some(AstNode::CompareOperation(
          AstNodeFactory::new_compare_operation(0, AstCompareOp::EqStrict, lhs, rhs),
        )),
        BinaryOp::StrictNotEq => Some(AstNode::CompareOperation(
          AstNodeFactory::new_compare_operation(0, AstCompareOp::NotEqStrict, lhs, rhs),
        )),
        BinaryOp::In => Some(AstNode::CompareOperation(
          AstNodeFactory::new_compare_operation(0, AstCompareOp::In, lhs, rhs),
        )),
        BinaryOp::InstanceOf => Some(AstNode::CompareOperation(
          AstNodeFactory::new_compare_operation(0, AstCompareOp::InstanceOf, lhs, rhs),
        )),
        BinaryOp::Comma => Some(AstNode::BinaryOperation(
          AstNodeFactory::new_binary_operation(0, AstBinaryOp::Comma, lhs, rhs),
        )),
      }
    }

    Expr::Unary { op, expr } => {
      let inner = expr_to_v8_ast_scoped(expr, locals, globals)?;
      match op {
        UnaryOp::Neg => Some(AstNode::UnaryOperation(
          AstNodeFactory::new_unary_operation(0, AstUnaryOp::Sub, inner),
        )),
        UnaryOp::Pos => Some(AstNode::UnaryOperation(
          AstNodeFactory::new_unary_operation(0, AstUnaryOp::Add, inner),
        )),
        UnaryOp::Not => Some(AstNode::UnaryOperation(
          AstNodeFactory::new_unary_operation(0, AstUnaryOp::Not, inner),
        )),
        UnaryOp::BitNot => Some(AstNode::UnaryOperation(
          AstNodeFactory::new_unary_operation(0, AstUnaryOp::BitNot, inner),
        )),
        UnaryOp::PreInc => Some(AstNode::CountOperation(
          AstNodeFactory::new_count_operation(0, AstCountOp::Inc, inner, true),
        )),
        UnaryOp::PostInc => Some(AstNode::CountOperation(
          AstNodeFactory::new_count_operation(0, AstCountOp::Inc, inner, false),
        )),
        UnaryOp::PreDec => Some(AstNode::CountOperation(
          AstNodeFactory::new_count_operation(0, AstCountOp::Dec, inner, true),
        )),
        UnaryOp::PostDec => Some(AstNode::CountOperation(
          AstNodeFactory::new_count_operation(0, AstCountOp::Dec, inner, false),
        )),
        UnaryOp::Void => Some(AstNode::UnaryOperation(
          AstNodeFactory::new_unary_operation(0, AstUnaryOp::Void, inner),
        )),
        UnaryOp::Delete => Some(AstNode::UnaryOperation(
          AstNodeFactory::new_unary_operation(0, AstUnaryOp::Delete, inner),
        )),
      }
    }

    Expr::Assign { target, op, value } => {
      let target_node = expr_to_v8_ast_scoped(target, locals, globals)?;
      let value_node = expr_to_v8_ast_scoped(value, locals, globals)?;
      let v8_op = match op {
        AssignOp::Assign => AstAssignOp::Assign,
        AssignOp::AddAssign => AstAssignOp::AssignAdd,
        AssignOp::SubAssign => AstAssignOp::AssignSub,
        AssignOp::MulAssign => AstAssignOp::AssignMul,
        AssignOp::DivAssign => AstAssignOp::AssignDiv,
        AssignOp::ModAssign => AstAssignOp::AssignMod,
        AssignOp::BitAndAssign => AstAssignOp::AssignBitAnd,
        AssignOp::BitOrAssign => AstAssignOp::AssignBitOr,
        AssignOp::BitXorAssign => AstAssignOp::AssignBitXor,
        AssignOp::ShlAssign => AstAssignOp::AssignShl,
        AssignOp::ShrAssign => AstAssignOp::AssignSar,
        AssignOp::SarAssign => AstAssignOp::AssignShr,
      };
      Some(AstNode::Assignment(AstNodeFactory::new_assignment(
        0,
        v8_op,
        target_node,
        value_node,
      )))
    }

    Expr::Conditional {
      condition,
      consequent,
      alternate,
    } => {
      let cond_node = expr_to_v8_ast_scoped(condition, locals, globals)?;
      let then_node = expr_to_v8_ast_scoped(consequent, locals, globals)?;
      let else_node = expr_to_v8_ast_scoped(alternate, locals, globals)?;
      Some(AstNode::Conditional(AstNodeFactory::new_conditional(
        0, cond_node, then_node, else_node,
      )))
    }

    Expr::Typeof(inner) => {
      let inner_node = expr_to_v8_ast_scoped(inner, locals, globals)?;
      Some(AstNode::UnaryOperation(
        AstNodeFactory::new_unary_operation(0, AstUnaryOp::TypeOf, inner_node),
      ))
    }
  }
}

/// 尝试将运行时表达式编译为 v8-ast 语法节点
#[inline]
pub fn expr_to_v8_ast(expr: &Expr) -> Option<AstNode> {
  let empty_locals = HashSet::default();
  let empty_globals = HashSet::default();
  expr_to_v8_ast_scoped(expr, &empty_locals, &empty_globals)
}

/// 尝试将运行时语句编译为 v8-ast 语法节点 (带符号定义域检查)
pub fn stmt_to_v8_ast_scoped(
  stmt: &Stmt,
  locals: &HashSet<&str>,
  globals: &HashSet<&str>,
) -> Option<AstNode> {
  match stmt {
    Stmt::Expr(expr) => expr_to_v8_ast_scoped(expr, locals, globals),

    Stmt::VarDecl { name, init } => {
      let decl = AstNode::VariableDeclaration(AstNodeFactory::new_variable_declaration(
        0,
        Some(AstRawString::from_utf8(name)),
        VariableMode::Var,
      ));
      if let Some(e) = init {
        let target = AstNode::VariableProxy(AstNodeFactory::new_variable_proxy(
          0,
          AstRawString::from_utf8(name),
          VariableProxyBindingMode::Bound,
          HoleCheckMode::Elided,
        ));
        let value = expr_to_v8_ast_scoped(e, locals, globals)?;
        let assign = AstNode::Assignment(AstNodeFactory::new_assignment(
          0,
          AstAssignOp::Assign,
          target,
          value,
        ));
        Some(AstNode::Block(AstNodeFactory::new_block(
          0,
          vec![decl, assign],
          false,
        )))
      } else {
        Some(decl)
      }
    }

    Stmt::VarDecls(decls) => {
      let mut stmts = Vec::with_capacity(decls.len() * 2);
      for (name, init) in decls {
        let decl = AstNode::VariableDeclaration(AstNodeFactory::new_variable_declaration(
          0,
          Some(AstRawString::from_utf8(name)),
          VariableMode::Var,
        ));
        stmts.push(decl);
        if let Some(e) = init {
          let target = AstNode::VariableProxy(AstNodeFactory::new_variable_proxy(
            0,
            AstRawString::from_utf8(name),
            VariableProxyBindingMode::Bound,
            HoleCheckMode::Elided,
          ));
          let value = expr_to_v8_ast_scoped(e, locals, globals)?;
          stmts.push(AstNode::Assignment(AstNodeFactory::new_assignment(
            0,
            AstAssignOp::Assign,
            target,
            value,
          )));
        }
      }
      Some(AstNode::Block(AstNodeFactory::new_block(0, stmts, false)))
    }

    Stmt::Block(stmts) => {
      let mut v8_stmts = Vec::with_capacity(stmts.len());
      for s in stmts {
        v8_stmts.push(stmt_to_v8_ast_scoped(s, locals, globals)?);
      }
      Some(AstNode::Block(AstNodeFactory::new_block(
        0, v8_stmts, false,
      )))
    }

    Stmt::If {
      cond,
      then_branch,
      else_branch,
    } => {
      let cond_node = expr_to_v8_ast_scoped(cond, locals, globals)?;
      let then_node = stmt_to_v8_ast_scoped(then_branch, locals, globals)?;
      let else_node = if let Some(eb) = else_branch {
        Some(stmt_to_v8_ast_scoped(eb, locals, globals)?)
      } else {
        None
      };
      Some(AstNode::IfStatement(AstNodeFactory::new_if_statement(
        0, cond_node, then_node, else_node,
      )))
    }

    Stmt::While { cond, body } => {
      let cond_node = expr_to_v8_ast_scoped(cond, locals, globals)?;
      let body_node = stmt_to_v8_ast_scoped(body, locals, globals)?;
      Some(AstNode::WhileStatement(
        AstNodeFactory::new_while_statement(0, cond_node, body_node),
      ))
    }

    Stmt::DoWhile { body, cond } => {
      let body_node = stmt_to_v8_ast_scoped(body, locals, globals)?;
      let cond_node = expr_to_v8_ast_scoped(cond, locals, globals)?;
      Some(AstNode::DoWhileStatement(
        AstNodeFactory::new_do_while_statement(0, body_node, cond_node),
      ))
    }

    Stmt::For {
      init,
      cond,
      step,
      body,
    } => {
      let init_node = if let Some(i) = init {
        Some(stmt_to_v8_ast_scoped(i, locals, globals)?)
      } else {
        None
      };
      let cond_node = if let Some(c) = cond {
        Some(expr_to_v8_ast_scoped(c, locals, globals)?)
      } else {
        None
      };
      let step_node = if let Some(s) = step {
        Some(expr_to_v8_ast_scoped(s, locals, globals)?)
      } else {
        None
      };
      let body_node = stmt_to_v8_ast_scoped(body, locals, globals)?;
      Some(AstNode::ForStatement(AstNodeFactory::new_for_statement(
        0, init_node, cond_node, step_node, body_node,
      )))
    }

    Stmt::Return(opt_expr) => {
      let ret_val = if let Some(e) = opt_expr {
        Some(expr_to_v8_ast_scoped(e, locals, globals)?)
      } else {
        None
      };
      Some(AstNode::ReturnStatement(
        AstNodeFactory::new_return_statement(0, ret_val, ReturnType::Normal),
      ))
    }

    Stmt::FunctionDecl(shared) => {
      let mut params = Vec::with_capacity(shared.params.len());
      for p in &shared.params {
        params.push(AstRawString::from_utf8(p));
      }

      let mut body = Vec::with_capacity(shared.body.len());
      for s in &shared.body {
        body.push(stmt_to_v8_ast_scoped(s, locals, globals)?);
      }

      let func_lit = FunctionLiteral {
        base: AstNodeBase::new(0, AstNodeType::FunctionLiteral),
        name: shared.name.as_deref().map(AstRawString::from_utf8),
        inferred_name: None,
        scope: None,
        params,
        body,
        syntax_kind: FunctionSyntaxKind::Declaration,
        compile_hint: EagerCompileHint::ShouldLazyCompile,
        is_async: false,
        is_generator: false,
        parameter_count: shared.params.len() as i32,
        function_length: shared.params.len() as i32,
        expected_property_count: 0,
        function_token_position: NO_SOURCE_POSITION,
        suspend_count: 0,
        function_literal_id: 0,
        has_duplicate_parameters: false,
        has_braces: true,
        pretenure: false,
        requires_instance_members_initializer: false,
        has_static_private_methods_or_accessors: false,
        preparse_data: None,
      };

      Some(AstNode::FunctionDeclaration(
        AstNodeFactory::new_function_declaration(0, Some(func_lit)),
      ))
    }

    Stmt::Break => Some(AstNode::BreakStatement(
      AstNodeFactory::new_break_statement(0, 0),
    )),

    Stmt::Continue => Some(AstNode::ContinueStatement(
      AstNodeFactory::new_continue_statement(0, 0),
    )),

    Stmt::Empty => Some(AstNode::EmptyStatement(
      AstNodeFactory::new_empty_statement(0),
    )),

    Stmt::Throw(expr) => {
      let expr_node = expr_to_v8_ast_scoped(expr, locals, globals)?;
      Some(AstNode::Throw(AstNodeFactory::new_throw(0, expr_node)))
    }

    Stmt::Switch {
      discriminant,
      cases,
    } => {
      let tag_node = expr_to_v8_ast_scoped(discriminant, locals, globals)?;
      let mut v8_cases = Vec::with_capacity(cases.len());
      for c in cases {
        let label_node = if let Some(ref test_expr) = c.test {
          Some(expr_to_v8_ast_scoped(test_expr, locals, globals)?)
        } else {
          None
        };
        let mut body_nodes = Vec::with_capacity(c.body.len());
        for s in &c.body {
          body_nodes.push(stmt_to_v8_ast_scoped(s, locals, globals)?);
        }
        v8_cases.push(CaseClause::new(label_node, body_nodes));
      }
      Some(AstNode::SwitchStatement(
        AstNodeFactory::new_switch_statement(0, tag_node, v8_cases),
      ))
    }

    Stmt::ForIn {
      variable,
      is_var,
      object,
      body,
    } => {
      let decl = if *is_var {
        Some(AstNode::VariableDeclaration(
          AstNodeFactory::new_variable_declaration(
            0,
            Some(AstRawString::from_utf8(variable)),
            VariableMode::Var,
          ),
        ))
      } else {
        None
      };
      let each_node = AstNode::VariableProxy(AstNodeFactory::new_variable_proxy(
        0,
        AstRawString::from_utf8(variable),
        VariableProxyBindingMode::Bound,
        HoleCheckMode::Elided,
      ));
      let subject_node = expr_to_v8_ast_scoped(object, locals, globals)?;
      let body_node = stmt_to_v8_ast_scoped(body, locals, globals)?;
      let for_in_node = AstNode::ForInStatement(AstNodeFactory::new_for_in_statement(
        0,
        each_node,
        subject_node,
        body_node,
        None,
      ));
      if let Some(d) = decl {
        Some(AstNode::Block(AstNodeFactory::new_block(
          0,
          vec![d, for_in_node],
          false,
        )))
      } else {
        Some(for_in_node)
      }
    }

    Stmt::Try {
      block,
      catch_param,
      catch_block,
      finally_block: _,
    } => {
      let mut try_stmts = Vec::with_capacity(block.len());
      for s in block {
        try_stmts.push(stmt_to_v8_ast_scoped(s, locals, globals)?);
      }
      let try_block = AstNodeFactory::new_block(0, try_stmts, false);

      let mut catch_stmts = Vec::new();
      if let Some(cb) = catch_block {
        catch_stmts.reserve(cb.len());
        for s in cb {
          catch_stmts.push(stmt_to_v8_ast_scoped(s, locals, globals)?);
        }
      }
      let catch_b = AstNodeFactory::new_block(0, catch_stmts, false);

      let var = catch_param.as_deref().map(AstRawString::from_utf8);

      Some(AstNode::TryCatchStatement(
        AstNodeFactory::new_try_catch_statement(
          0,
          try_block,
          catch_b,
          var,
          CatchPrediction::Caught,
        ),
      ))
    }
  }
}

/// 尝试将运行时语句编译为 v8-ast 语法节点
#[inline]
pub fn stmt_to_v8_ast(stmt: &Stmt) -> Option<AstNode> {
  let empty_locals = HashSet::default();
  let empty_globals = HashSet::default();
  stmt_to_v8_ast_scoped(stmt, &empty_locals, &empty_globals)
}

impl From<&InterpreterValue> for JsValue {
  fn from(val: &InterpreterValue) -> Self {
    interpreter_value_to_js_value(val)
  }
}

impl From<InterpreterValue> for JsValue {
  fn from(val: InterpreterValue) -> Self {
    interpreter_value_to_js_value(&val)
  }
}

/// 将解释器原生值转换为 JsValue (带防环与深度预算保护)
#[must_use]
pub fn interpreter_value_to_js_value(val: &InterpreterValue) -> JsValue {
  match val {
    InterpreterValue::Undefined | InterpreterValue::TheHole => JsValue::Undefined,
    InterpreterValue::Null => JsValue::Null,
    InterpreterValue::Boolean(b) => JsValue::Boolean(*b),
    InterpreterValue::Smi(n) => JsValue::Number(*n as f64),
    InterpreterValue::Number(n) => JsValue::Number(*n),
    InterpreterValue::String(s) => JsValue::String(s.clone()),
    InterpreterValue::BigInt(s) => JsValue::String(s.clone()),
    _ => {
      let mut visited = HashSet::default();
      interp_to_js_with_visited(val, &mut visited, 0)
    }
  }
}

fn interp_to_js_with_visited(
  val: &InterpreterValue,
  visited: &mut HashSet<usize>,
  depth: usize,
) -> JsValue {
  if depth > 16 {
    return JsValue::Undefined;
  }
  match val {
    InterpreterValue::Undefined | InterpreterValue::TheHole => JsValue::Undefined,
    InterpreterValue::Null => JsValue::Null,
    InterpreterValue::Boolean(b) => JsValue::Boolean(*b),
    InterpreterValue::Smi(n) => JsValue::Number(*n as f64),
    InterpreterValue::Number(n) => JsValue::Number(*n),
    InterpreterValue::String(s) => JsValue::String(s.clone()),
    InterpreterValue::BigInt(s) => JsValue::String(s.clone()),
    InterpreterValue::Array(arr) => {
      let ptr = Arc::as_ptr(arr) as usize;
      if !visited.insert(ptr) {
        return JsValue::new_array(Vec::new());
      }
      let guard = arr.read();
      let mut list = Vec::with_capacity(guard.len());
      for item in guard.iter() {
        list.push(interp_to_js_with_visited(item, visited, depth + 1));
      }
      visited.remove(&ptr);
      JsValue::new_array(list)
    }
    InterpreterValue::Object(obj) => {
      let ptr = Arc::as_ptr(obj) as usize;
      if !visited.insert(ptr) {
        return JsValue::new_object();
      }
      let guard = obj.read();
      let js_obj = JsValue::new_object();
      for (k, v) in guard.iter() {
        let _ = js_obj.set_property(k, interp_to_js_with_visited(v, visited, depth + 1));
      }
      visited.remove(&ptr);
      js_obj
    }
    // 带自有属性的函数必须落成 JsValue::Function: JsValue::Native 没有属性槽,
    // 挂在函数上的方法 (如测试框架的 assert.sameValue) 会在回写宿主环境时丢失
    InterpreterValue::Native(native, props) => {
      let entries = detach_props(props);
      if entries.is_empty() {
        JsValue::Native(NativeFn::InterpNative(native.clone()))
      } else {
        interp_callable_to_js(
          NativeFn::InterpNative(native.clone()),
          None,
          Vec::new(),
          entries,
          visited,
          depth,
        )
      }
    }
    InterpreterValue::Function(func, props) => {
      let entries = detach_props(props);
      if entries.is_empty() {
        JsValue::Native(NativeFn::InterpFunction(Arc::clone(func)))
      } else {
        interp_callable_to_js(
          NativeFn::InterpFunction(Arc::clone(func)),
          func.name.clone(),
          func.params.clone(),
          entries,
          visited,
          depth,
        )
      }
    }
  }
}

/// 属性表整体脱锁复制: 属性值可能指回本函数, 持锁递归会自堵
fn detach_props(
  props: &Arc<RwLock<HashMap<String, InterpreterValue>>>,
) -> Vec<(String, InterpreterValue)> {
  props
    .read()
    .iter()
    .map(|(k, v)| (k.clone(), v.clone()))
    .collect()
}

/// 解释器函数值 → 宿主原生支撑的函数对象, 携带其已脱锁的自有属性
fn interp_callable_to_js(
  native: NativeFn,
  name: Option<String>,
  params: Vec<String>,
  entries: Vec<(String, InterpreterValue)>,
  visited: &mut HashSet<usize>,
  depth: usize,
) -> JsValue {
  let properties: HashMap<String, JsValue> = entries
    .into_iter()
    .map(|(k, v)| (k, interp_to_js_with_visited(&v, visited, depth + 1)))
    .collect();
  JsValue::Function(Arc::new(FunctionData {
    shared: Arc::new(SharedFunctionInfo::new(name, params, Vec::new())),
    closure: Arc::new(Environment::default()),
    properties: Arc::new(RwLock::new(properties)),
    native_impl: Some(native),
  }))
}

/// 尝试将 JsValue 转换为解释器原生值
#[must_use]
pub fn js_value_to_interpreter_value(val: &JsValue) -> Option<InterpreterValue> {
  match val {
    JsValue::Undefined => Some(InterpreterValue::Undefined),
    JsValue::Null => Some(InterpreterValue::Null),
    JsValue::Boolean(b) => Some(InterpreterValue::Boolean(*b)),
    JsValue::Number(n) => {
      if n.fract() == 0.0 && *n >= (i32::MIN as f64) && *n <= (i32::MAX as f64) {
        Some(InterpreterValue::Smi(*n as i32))
      } else {
        Some(InterpreterValue::Number(*n))
      }
    }
    JsValue::BigInt(n) => {
      let mut buf = Buffer::new();
      Some(InterpreterValue::BigInt(buf.format(*n).to_string()))
    }
    JsValue::String(s) => Some(InterpreterValue::String(s.clone())),
    JsValue::Symbol(_) => Some(InterpreterValue::Undefined),
    _ => {
      let mut visited = HashSet::default();
      js_value_to_interp_with_visited(val, &mut visited, 0)
    }
  }
}

fn js_value_to_interp_with_visited(
  val: &JsValue,
  visited: &mut HashSet<usize>,
  depth: usize,
) -> Option<InterpreterValue> {
  if depth > 16 {
    return Some(InterpreterValue::Object(Arc::new(RwLock::new(
      HashMap::default(),
    ))));
  }
  match val {
    JsValue::Undefined => Some(InterpreterValue::Undefined),
    JsValue::Null => Some(InterpreterValue::Null),
    JsValue::Boolean(b) => Some(InterpreterValue::Boolean(*b)),
    JsValue::Number(n) => {
      if n.fract() == 0.0 && *n >= (i32::MIN as f64) && *n <= (i32::MAX as f64) {
        Some(InterpreterValue::Smi(*n as i32))
      } else {
        Some(InterpreterValue::Number(*n))
      }
    }
    JsValue::BigInt(n) => {
      let mut buf = Buffer::new();
      Some(InterpreterValue::BigInt(buf.format(*n).to_string()))
    }
    JsValue::String(s) => Some(InterpreterValue::String(s.clone())),
    JsValue::Symbol(_) => Some(InterpreterValue::Undefined),
    JsValue::Array(arr) => {
      let ptr = Arc::as_ptr(arr) as usize;
      if !visited.insert(ptr) {
        return Some(InterpreterValue::Array(Arc::new(RwLock::new(Vec::new()))));
      }
      let guard = arr.read();
      let mut res = Vec::with_capacity(guard.len());
      for item in guard.iter() {
        if let Some(iv) = js_value_to_interp_with_visited(item, visited, depth + 1) {
          res.push(iv);
        } else {
          res.push(InterpreterValue::Undefined);
        }
      }
      visited.remove(&ptr);
      Some(InterpreterValue::Array(Arc::new(RwLock::new(res))))
    }
    JsValue::Object(obj) => {
      let ptr = Arc::as_ptr(obj) as usize;
      if !visited.insert(ptr) {
        return Some(InterpreterValue::Object(Arc::new(RwLock::new(
          HashMap::default(),
        ))));
      }
      let guard = obj.read();
      let mut map = HashMap::default();
      for (k, v) in guard.iter() {
        if k == "__proto__" || k == "constructor" || k == "prototype" {
          continue;
        }
        if let Some(iv) = js_value_to_interp_with_visited(v, visited, depth + 1) {
          map.insert(k.clone(), iv);
        }
      }
      visited.remove(&ptr);
      Some(InterpreterValue::Object(Arc::new(RwLock::new(map))))
    }
    JsValue::Native(native) => {
      if let NativeFn::InterpNative(orig) = native {
        return Some(InterpreterValue::new_native(orig.clone()));
      }
      if let NativeFn::InterpFunction(func) = native {
        return Some(InterpreterValue::Function(
          Arc::clone(func),
          Arc::new(RwLock::new(HashMap::default())),
        ));
      }
      let native_clone = native.clone();
      let interp_fn = InterpNativeFn::from_context(native_clone, |nat, args| {
        let js_args: Vec<JsValue> = args.iter().map(JsValue::from).collect();
        match nat.call(&js_args) {
          Ok(ret) => Ok(js_value_to_interpreter_value(&ret).unwrap_or(InterpreterValue::Undefined)),
          Err(e) => Err(V8InterpreterError::ExecutionError(e.to_string())),
        }
      });
      Some(InterpreterValue::new_native(interp_fn))
    }
    JsValue::Function(func_data) => {
      let mut props = HashMap::default();
      let p_guard = func_data.properties.read();
      for (k, v) in p_guard.iter() {
        if let Some(iv) = js_value_to_interp_with_visited(v, visited, depth + 1) {
          props.insert(k.clone(), iv);
        }
      }
      if !props.contains_key(PROTOTYPE_KEY) {
        props.insert(PROTOTYPE_KEY.to_string(), InterpreterValue::new_object());
      }
      let props_arc = Arc::new(RwLock::new(props));

      if let Some(ref native) = func_data.native_impl {
        let interp_fn = if let NativeFn::InterpNative(orig) = native {
          orig.clone()
        } else {
          let native_clone = native.clone();
          InterpNativeFn::from_context(native_clone, |nat, args| {
            let js_args: Vec<JsValue> = args.iter().map(JsValue::from).collect();
            match nat.call(&js_args) {
              Ok(ret) => {
                Ok(js_value_to_interpreter_value(&ret).unwrap_or(InterpreterValue::Undefined))
              }
              Err(e) => Err(V8InterpreterError::ExecutionError(e.to_string())),
            }
          })
        };
        Some(InterpreterValue::new_native_with_props(
          interp_fn, props_arc,
        ))
      } else {
        let func_data_clone = Arc::clone(func_data);
        let interp_fn = InterpNativeFn::from_context(func_data_clone, |func_data_clone, args| {
          let js_args: Vec<JsValue> = args.iter().map(JsValue::from).collect();
          let mut interp = Interpreter::new();
          match interp.invoke_function(
            &JsValue::Function(Arc::clone(func_data_clone)),
            None,
            &js_args,
          ) {
            Ok(ret) => {
              Ok(js_value_to_interpreter_value(&ret).unwrap_or(InterpreterValue::Undefined))
            }
            Err(e) => Err(V8InterpreterError::ExecutionError(e.to_string())),
          }
        });
        Some(InterpreterValue::new_native_with_props(
          interp_fn, props_arc,
        ))
      }
    }
    JsValue::RegExp(re) => {
      let obj = InterpreterValue::new_object();
      let _ = obj.set_property(SOURCE_KEY, InterpreterValue::String(re.pattern.clone()));
      let _ = obj.set_property(FLAGS_KEY, InterpreterValue::String(re.flags.clone()));
      let _ = obj.set_property(GLOBAL_KEY, InterpreterValue::Boolean(re.global));
      let _ = obj.set_property(LAST_INDEX_KEY, InterpreterValue::Smi(0));
      let re_clone = Arc::clone(re);
      let exec_fn = InterpNativeFn::from_context(re_clone, |re_clone, args| {
        let text_cow = match args.first() {
          Some(InterpreterValue::String(s)) => Cow::Borrowed(s.as_str()),
          Some(other) => Cow::Owned(other.to_string_lossy()),
          None => Cow::Borrowed(""),
        };
        let text = text_cow.as_ref();
        let from_pos = args
          .get(1)
          .map(|v| v.to_i32().max(0) as usize)
          .unwrap_or_else(|| {
            if re_clone.global {
              *re_clone.last_index.read()
            } else {
              0
            }
          });
        if from_pos > text.len() {
          if re_clone.global {
            *re_clone.last_index.write() = 0;
          }
          return Ok(InterpreterValue::Null);
        }
        match re_clone.compiled.captures_from_pos(text, from_pos) {
          Ok(Some(caps)) => {
            let mut list = Vec::with_capacity(caps.len());
            for m in caps.iter() {
              if let Some(mat) = m {
                list.push(InterpreterValue::String(mat.as_str().to_string()));
              } else {
                list.push(InterpreterValue::Undefined);
              }
            }
            if let Some(whole) = caps.get(0)
              && re_clone.global
            {
              *re_clone.last_index.write() = whole.end();
            }
            Ok(InterpreterValue::new_array_with_elements(list))
          }
          _ => {
            if re_clone.global {
              *re_clone.last_index.write() = 0;
            }
            Ok(InterpreterValue::Null)
          }
        }
      });
      let _ = obj.set_property(EXEC_KEY, InterpreterValue::new_native(exec_fn));

      let re_rep = Arc::clone(re);
      let replace_fn = InterpNativeFn::from_context(re_rep, |re_rep, args| {
        let text_cow = match args.first() {
          Some(InterpreterValue::String(s)) => Cow::Borrowed(s.as_str()),
          Some(other) => Cow::Owned(other.to_string_lossy()),
          None => Cow::Borrowed(""),
        };
        let text = text_cow.as_ref();
        let repl_cow = match args.get(1) {
          Some(InterpreterValue::String(s)) => Cow::Borrowed(s.as_str()),
          Some(other) => Cow::Owned(other.to_string_lossy()),
          None => Cow::Borrowed(""),
        };
        let repl = repl_cow.as_ref();
        let res = if re_rep.global {
          re_rep.compiled.replace_all(text, repl)
        } else {
          re_rep.compiled.replace(text, repl)
        };
        Ok(InterpreterValue::String(res.into_owned()))
      });
      let _ = obj.set_property(REPLACE_KEY, InterpreterValue::new_native(replace_fn));

      let re_split = Arc::clone(re);
      let split_fn = InterpNativeFn::from_context(re_split, |re_split, args| {
        let text_cow = match args.first() {
          Some(InterpreterValue::String(s)) => Cow::Borrowed(s.as_str()),
          Some(other) => Cow::Owned(other.to_string_lossy()),
          None => Cow::Borrowed(""),
        };
        let text = text_cow.as_ref();
        let limit = match args.get(1) {
          Some(InterpreterValue::Undefined) | None => None,
          Some(v) => {
            let n = v.to_u32();
            if n == u32::MAX {
              None
            } else {
              Some(n as usize)
            }
          }
        };
        let iter = re_split.compiled.split(text);
        let parts: Vec<InterpreterValue> = if let Some(lim) = limit {
          let mut list = Vec::with_capacity(lim.min(text.len() + 1));
          for p in iter.take(lim) {
            list.push(InterpreterValue::String(p.unwrap_or("").to_string()));
          }
          list
        } else {
          iter
            .map(|p| InterpreterValue::String(p.unwrap_or("").to_string()))
            .collect()
        };
        Ok(InterpreterValue::new_array_with_elements(parts))
      });
      let _ = obj.set_property(SPLIT_KEY, InterpreterValue::new_native(split_fn));

      let re_match = Arc::clone(re);
      let match_fn = InterpNativeFn::from_context(re_match, |re_match, args| {
        let text_cow = match args.first() {
          Some(InterpreterValue::String(s)) => Cow::Borrowed(s.as_str()),
          Some(other) => Cow::Owned(other.to_string_lossy()),
          None => Cow::Borrowed(""),
        };
        let text = text_cow.as_ref();
        if re_match.global {
          let mut results = Vec::new();
          let mut cur_pos = 0;
          while cur_pos <= text.len() {
            if let Ok(Some(mat)) = re_match.compiled.find_from_pos(text, cur_pos) {
              results.push(InterpreterValue::String(mat.as_str().to_string()));
              if mat.end() == cur_pos {
                cur_pos += 1;
              } else {
                cur_pos = mat.end();
              }
            } else {
              break;
            }
          }
          if results.is_empty() {
            Ok(InterpreterValue::Null)
          } else {
            Ok(InterpreterValue::new_array_with_elements(results))
          }
        } else {
          match re_match.compiled.captures(text) {
            Ok(Some(caps)) => {
              let mut list = Vec::with_capacity(caps.len());
              for m in caps.iter() {
                if let Some(mat) = m {
                  list.push(InterpreterValue::String(mat.as_str().to_string()));
                } else {
                  list.push(InterpreterValue::Undefined);
                }
              }
              Ok(InterpreterValue::new_array_with_elements(list))
            }
            _ => Ok(InterpreterValue::Null),
          }
        }
      });
      let _ = obj.set_property(MATCH_KEY, InterpreterValue::new_native(match_fn));

      let re_test = Arc::clone(re);
      let test_fn = InterpNativeFn::from_context(re_test, |re_test, args| {
        let text_cow = match args.first() {
          Some(InterpreterValue::String(s)) => Cow::Borrowed(s.as_str()),
          Some(other) => Cow::Owned(other.to_string_lossy()),
          None => Cow::Borrowed(""),
        };
        let is_match = re_test
          .compiled
          .is_match(text_cow.as_ref())
          .unwrap_or(false);
        Ok(InterpreterValue::Boolean(is_match))
      });
      let _ = obj.set_property(TEST_KEY, InterpreterValue::new_native(test_fn));
      Some(obj)
    }
  }
}

/// 尝试以 Ignition 扁平字节码数组流调度执行语句序列 (支持外部全局环境双向同步)
pub fn try_execute_with_bytecode_and_env(
  stmts: &[Stmt],
  global_env: Option<&Environment>,
) -> Option<Result<JsValue>> {
  let mut locals = HashSet::default();
  collect_stmts_vars(stmts, &mut locals);

  let mut dispatcher = BytecodeDispatcher::new();

  // 若传入外部词法环境，将其中标量与对象全局变量注入字节码全局表 (逐层回溯父作用域，确保 arguments 与闭包变量完整导入)
  if let Some(env) = global_env {
    let mut curr_env = Some(env);
    while let Some(curr) = curr_env {
      let guard = curr.values.read();
      for (k, v) in guard.iter() {
        if dispatcher.get_global(k).is_none()
          && let Some(interp_val) = js_value_to_interpreter_value(v)
        {
          dispatcher.set_global(k.clone(), interp_val);
        }
      }
      curr_env = curr.parent.as_deref();
    }
  }

  if let Some(re) = dispatcher.get_global("RegExp").cloned() {
    dispatcher.set_global("$$RegExp", re);
  }

  let mut v8_stmts = Vec::with_capacity(stmts.len());
  let last_idx = stmts.len().saturating_sub(1);
  let empty_globals = HashSet::default();
  for (idx, s) in stmts.iter().enumerate() {
    if idx == last_idx
      && let Stmt::Expr(expr) = s
    {
      let ret_expr = expr_to_v8_ast_scoped(expr, &locals, &empty_globals)?;
      let ret_ast = AstNode::ReturnStatement(AstNodeFactory::new_return_statement(
        0,
        Some(ret_expr),
        ReturnType::Normal,
      ));
      v8_stmts.push(ret_ast);
    } else {
      let node = stmt_to_v8_ast_scoped(s, &locals, &empty_globals)?;
      v8_stmts.push(node);
    }
  }

  let root_ast = AstNode::Block(AstNodeFactory::new_block(0, v8_stmts, false));
  let bytecode = match BytecodeGenerator::generate(&root_ast) {
    Ok(b) => b,
    Err(e) => return Some(Err(Error::RuntimeError(e.to_string()))),
  };

  match dispatcher.execute(&bytecode) {
    Ok(val) => {
      // 执行成功后，若有新全局变量写入，同步写回宿主环境 (单次写锁批处理，消除逐键竞争)
      if let Some(env) = global_env {
        let mut guard = env.values.write();
        for (k, v) in dispatcher.globals() {
          guard.insert(k.clone(), JsValue::from(v));
        }
      }
      Some(Ok(JsValue::from(val)))
    }
    Err(e) => Some(Err(Error::RuntimeError(e.to_string()))),
  }
}

/// 尝试以 Ignition 扁平字节码数组流调度执行语句序列
#[inline]
pub fn try_execute_with_bytecode(stmts: &[Stmt]) -> Option<Result<JsValue>> {
  try_execute_with_bytecode_and_env(stmts, None)
}
