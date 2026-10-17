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
  ACCESSOR_FLAG_KEY, BytecodeDispatcher, BytecodeGenerator, Error as V8InterpreterError,
  GETTER_PREFIX, GlobalsTable, LENGTH_KEY, NAME_KEY, NativeFn as InterpNativeFn, PROTO_KEY,
  SETTER_PREFIX, Value as InterpreterValue, fits_smi, format_getter_key, format_setter_key,
  is_internal_key,
};

use crate::{
  error::{Error, Result},
  runtime::{
    ast::{AssignOp, BinaryOp, Expr, ObjectKey, PropKind, Stmt, UnaryOp},
    environment::Environment,
    interpreter::Interpreter,
    value::{FunctionData, JsValue, NativeFn, PROTOTYPE_KEY, SharedFunctionInfo},
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
    Stmt::Labeled { body, .. } => {
      collect_stmt_vars(body, locals);
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

/// 对象字面量属性键的最终落表键: accessor 换成 `@@get:`/`@@set:` 伪键 (与
/// Object.defineProperty、AST 求值器同一套约定), 数据属性原样
#[inline]
fn accessor_key(name: &str, kind: PropKind) -> Cow<'_, str> {
  match kind {
    PropKind::Data => Cow::Borrowed(name),
    PropKind::Get => Cow::Owned(format_getter_key(name)),
    PropKind::Set => Cow::Owned(format_setter_key(name)),
  }
}

/// 尝试将运行时表达式编译为 v8-ast 语法节点 (带作用域定义域检查)
/// 合成 `new $$RegExp(pattern, flags)` AST 节点 (C-④a 一处定义): BC 链正则字面量的
/// 唯一落形 —— `Expr::RegExp` 解析产物与快照态 `JsValue::RegExp` 字面量两臂共走,
/// 每跳执行即新实例 (规范 per-eval), 重编译税由引擎 `RegExpCore` 缓存吸收
fn regexp_new_ast(pattern: &str, flags: &str) -> AstNode {
  let callee = AstNode::VariableProxy(AstNodeFactory::new_variable_proxy(
    0,
    AstRawString::from_utf8("$$RegExp"),
    VariableProxyBindingMode::Bound,
    HoleCheckMode::Elided,
  ));
  let arg0 = AstNode::Literal(AstNodeFactory::new_literal(
    0,
    AstValue::String(AstRawString::from_utf8(pattern)),
  ));
  let arg1 = AstNode::Literal(AstNodeFactory::new_literal(
    0,
    AstValue::String(AstRawString::from_utf8(flags)),
  ));
  AstNode::CallNew(AstNodeFactory::new_call_new(
    0,
    callee,
    vec![arg0, arg1],
    SpreadPosition::NoSpread,
  ))
}

pub fn expr_to_v8_ast_scoped(
  expr: &Expr,
  locals: &HashSet<&str>,
  globals: &HashSet<&str>,
) -> Option<AstNode> {
  match expr {
    // 解构暂态在解析出口已展开, 正常不可达; 残形交降级路径而非 panic
    Expr::AssignPattern { .. } => None,
    // C-④a: 正则字面量在 BC 链合成 `new $$RegExp(pattern, flags)` —— 每跳新建实例
    // (规范 per-eval), 编译税由 RegExpCore 进程级缓存吸收; 与旧 Literal(RegExp) 合成分工:
    // 解析产物走本臂, 快照态 RegExp 值仍走 Literal 臂 (同一合成体)
    Expr::RegExp { pattern, flags } => Some(regexp_new_ast(pattern, flags)),
    Expr::Literal(val) => match val {
      JsValue::Number(n) => {
        if fits_smi(*n) {
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
      // 数组空位 (战役 C ③): 引擎生成器认 AstValue::TheHole → LdaTheHole, 直取哨兵入 accumulator
      JsValue::TheHole => Some(AstNode::Literal(AstNodeFactory::new_literal(
        0,
        AstValue::TheHole,
      ))),
      JsValue::RegExp(re) => Some(regexp_new_ast(&re.core.pattern, &re.core.flags)),
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
      let mut prop_nodes = Vec::with_capacity(entries.len() + 1);
      // 哨兵键排在最前: 字面量自身的落表现在走 DefineOwnProperty, 它的"同名 accessor 摘除"要看
      // 得到这个哨兵 (追加在尾部时, 构建期的数据键定义永远看不见 flag, 覆盖语义就失效)
      let has_accessor = entries.iter().any(|p| !matches!(p.kind, PropKind::Data));
      if has_accessor {
        prop_nodes.push(ObjectLiteralProperty::new(
          ObjectPropertyKind::Constant,
          AstNode::Literal(AstNodeFactory::new_literal(
            0,
            AstValue::String(AstRawString::from_utf8(ACCESSOR_FLAG_KEY)),
          )),
          AstNode::Literal(AstNodeFactory::new_literal(0, AstValue::Boolean(true))),
          false,
        ));
      }
      for p in entries {
        let (key_node, is_computed) = match (&p.key, p.kind) {
          (ObjectKey::Ident(s), kind) => (
            AstNode::Literal(AstNodeFactory::new_literal(
              0,
              AstValue::String(AstRawString::from_utf8(&accessor_key(s, kind))),
            )),
            false,
          ),
          // 计算键 accessor: 伪键前缀接在求值后的键上就行, 而"求值后的键"本身就是一个计算键,
          // 于是 `"@@get:" + k` 用一条 Add 表达式表达, 属性形态仍走 is_computed —— 原先这里
          // return None, 一个 `get [k](){}` 就让整脚本回退到 AST 求值器
          (ObjectKey::Computed(expr), kind) => {
            let key_node = expr_to_v8_ast_scoped(expr, locals, globals)?;
            let prefix = match kind {
              PropKind::Get => Some(GETTER_PREFIX),
              PropKind::Set => Some(SETTER_PREFIX),
              PropKind::Data => None,
            };
            let node = match prefix {
              Some(p) => AstNode::BinaryOperation(AstNodeFactory::new_binary_operation(
                0,
                AstBinaryOp::Add,
                AstNode::Literal(AstNodeFactory::new_literal(
                  0,
                  AstValue::String(AstRawString::from_utf8(p)),
                )),
                key_node,
              )),
              None => key_node,
            };
            (node, true)
          }
        };
        let val_node = expr_to_v8_ast_scoped(&p.value, locals, globals)?;
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
        is_arrow: shared.is_arrow,
        pretenure: false,
        requires_instance_members_initializer: false,
        has_static_private_methods_or_accessors: false,
        preparse_data: None,
        src: shared.src.clone(),
        rest_param: shared.rest_param.clone().map(Box::new),
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
/// 标签附加到可标记目标 (循环/switch/block/if); 其余语句 (var/表达式等死标签) 静默丢弃
/// —— 它们体内不可能出现 break/continue L, 标签无 observable 语义
fn attach_labels(node: &mut AstNode, labels: &[String]) {
  let dst: &mut Vec<String> = match node {
    AstNode::WhileStatement(s) => &mut s.labels,
    AstNode::DoWhileStatement(s) => &mut s.labels,
    AstNode::ForStatement(s) => &mut s.labels,
    AstNode::ForInStatement(s) => &mut s.labels,
    AstNode::Block(s) => &mut s.labels,
    AstNode::IfStatement(s) => &mut s.labels,
    AstNode::SwitchStatement(s) => &mut s.labels,
    _ => return,
  };
  *dst = labels.to_vec();
}

pub fn stmt_to_v8_ast_scoped(
  stmt: &Stmt,
  locals: &HashSet<&str>,
  globals: &HashSet<&str>,
) -> Option<AstNode> {
  match stmt {
    Stmt::Expr(expr) => expr_to_v8_ast_scoped(expr, locals, globals),
    // 解构/for-of 暂态在 parse 出口已展开, 理论上永不到此; 交回 None 走 AST 回落保平安
    Stmt::VarPattern { .. } | Stmt::ForOf { .. } => None,

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
        is_arrow: shared.is_arrow,
        pretenure: false,
        requires_instance_members_initializer: false,
        has_static_private_methods_or_accessors: false,
        preparse_data: None,
        src: shared.src.clone(),
        rest_param: shared.rest_param.clone().map(Box::new),
      };

      Some(AstNode::FunctionDeclaration(
        AstNodeFactory::new_function_declaration(0, Some(func_lit)),
      ))
    }

    Stmt::Break(label) => {
      let mut br = AstNodeFactory::new_break_statement(0, 0);
      br.label = label.clone();
      Some(AstNode::BreakStatement(br))
    }

    Stmt::Continue(label) => {
      let mut ct = AstNodeFactory::new_continue_statement(0, 0);
      ct.label = label.clone();
      Some(AstNode::ContinueStatement(ct))
    }

    Stmt::Labeled { labels, body } => {
      let mut node = stmt_to_v8_ast_scoped(body, locals, globals)?;
      // is_var 的 for-in 包成 Block[decl, forin]: 标签挂内层 ForIn (continue L 要有循环层接)
      if matches!(**body, Stmt::ForIn { is_var: true, .. }) {
        if let AstNode::Block(b) = &mut node
          && let Some(last) = b.statements.last_mut()
        {
          attach_labels(last, labels);
        }
      } else {
        attach_labels(&mut node, labels);
      }
      Some(node)
    }

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
      finally_block,
    } => {
      let mut try_stmts = Vec::with_capacity(block.len());
      for s in block {
        try_stmts.push(stmt_to_v8_ast_scoped(s, locals, globals)?);
      }
      let try_block = AstNodeFactory::new_block(0, try_stmts, false);

      let var = catch_param.as_deref().map(AstRawString::from_utf8);

      let core = match catch_block {
        Some(cb) => {
          let mut catch_stmts = Vec::with_capacity(cb.len());
          for s in cb {
            catch_stmts.push(stmt_to_v8_ast_scoped(s, locals, globals)?);
          }
          let catch_b = AstNodeFactory::new_block(0, catch_stmts, false);
          AstNode::TryCatchStatement(AstNodeFactory::new_try_catch_statement(
            0,
            try_block,
            catch_b,
            var,
            CatchPrediction::Caught,
          ))
        }
        // 无 catch 时不能凭空造一个空的 TryCatch: 空 catch 会把异常吃掉, 让 finally 之外
        // 的处理器永远收不到它 (对标原版: try/finally 直接生成 TryFinallyStatement)
        None => AstNode::Block(try_block),
      };

      let fin_stmts = match finally_block {
        Some(fb) if !fb.is_empty() => {
          let mut v = Vec::with_capacity(fb.len());
          for s in fb {
            v.push(stmt_to_v8_ast_scoped(s, locals, globals)?);
          }
          v
        }
        _ => return Some(core),
      };

      Some(AstNode::TryFinallyStatement(
        AstNodeFactory::new_try_finally_statement(
          0,
          AstNodeFactory::new_block(0, vec![core], false),
          AstNodeFactory::new_block(0, fin_stmts, false),
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
///
/// 无执行上下文可用, 因此转换出的字节码函数只挂一张私有空表; 由字节码运行回写宿主的转换
/// 必须走 `interpreter_value_to_js_value_with_globals`, 否则函数回调时看不到全局
#[must_use]
pub fn interpreter_value_to_js_value(val: &InterpreterValue) -> JsValue {
  let detached = GlobalsTable::default();
  interpreter_value_to_js_value_with_globals(val, &detached)
}

/// 同上, 并让转换出的字节码函数携带其所属全局表句柄 (对标原版 JSFunction 自带 NativeContext)
#[must_use]
pub fn interpreter_value_to_js_value_with_globals(
  val: &InterpreterValue,
  globals: &GlobalsTable,
) -> JsValue {
  // 空 HashSet 不分配, 因此无需为标量另开快路径
  let mut visited = HashSet::default();
  interp_to_js_with_visited(val, &mut visited, 0, globals)
}

fn interp_to_js_with_visited(
  val: &InterpreterValue,
  visited: &mut HashSet<usize>,
  depth: usize,
  globals: &GlobalsTable,
) -> JsValue {
  if depth > 16 {
    return JsValue::Undefined;
  }
  match val {
    InterpreterValue::Undefined => JsValue::Undefined,
    // 洞哨兵反向同样 1:1 (战役 C ③): 此前塌 Undefined 会抹掉洞的存在性
    InterpreterValue::TheHole => JsValue::TheHole,
    InterpreterValue::Null => JsValue::Null,
    InterpreterValue::Boolean(b) => JsValue::Boolean(*b),
    InterpreterValue::Smi(n) => JsValue::Number(*n as f64),
    InterpreterValue::Number(n) => JsValue::Number(*n),
    InterpreterValue::String(s) => JsValue::String(s.clone()),
    InterpreterValue::Symbol(s) => JsValue::Symbol(s.clone()),
    InterpreterValue::BigInt(s) => JsValue::String(s.clone().into()),
    InterpreterValue::Array(arr) => {
      let ptr = Arc::as_ptr(arr) as usize;
      if !visited.insert(ptr) {
        return JsValue::new_array(Vec::new());
      }
      let guard = arr.read();
      let mut list = Vec::with_capacity(guard.elements.len());
      for item in guard.elements.iter() {
        list.push(interp_to_js_with_visited(item, visited, depth + 1, globals));
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
        let _ = js_obj.set_property(k, interp_to_js_with_visited(v, visited, depth + 1, globals));
      }
      visited.remove(&ptr);
      js_obj
    }
    // C-2b 第二刀: 引擎正则变体回宿主 = 内核 Arc 浅拷贝 + 袋逐键转换。旧"哨兵对象重编译
    // 还原"路径退役: 不重编译正则体, lastIndex 与 expando 遮蔽都经同一 Arc 内核直连
    InterpreterValue::RegExp(core, bag) => {
      let ptr = Arc::as_ptr(core) as usize;
      if !visited.insert(ptr) {
        return JsValue::regexp_from_core(Arc::clone(core), HashMap::default());
      }
      let mut props = HashMap::default();
      for (k, v) in bag.read().iter() {
        // Construct 挂的原型链槽与 `@@` 内部键不回搬宿主袋 (旧还原臂同口径):
        // 宿主实例的原型面由宿主模型自持, 袋里只该有用户 expando
        if k == PROTO_KEY || is_internal_key(k) {
          continue;
        }
        props.insert(
          k.clone(),
          interp_to_js_with_visited(v, visited, depth + 1, globals),
        );
      }
      visited.remove(&ptr);
      JsValue::regexp_from_core(Arc::clone(core), props)
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
          None,
          entries,
          globals,
          visited,
          depth,
        )
      }
    }
    InterpreterValue::Function(closure) => {
      let entries = detach_props(&closure.props);
      let native = NativeFn::InterpFunction(Arc::clone(closure), Arc::clone(globals));
      if entries.is_empty() {
        JsValue::Native(native)
      } else {
        interp_callable_to_js(
          native,
          closure.func.name.clone(),
          closure.func.params.clone(),
          closure.func.src.clone(),
          entries,
          globals,
          visited,
          depth,
        )
      }
    }
  }
}

/// 属性表整体脱锁复制: 属性值可能指回本函数, 持锁递归会自堵
fn detach_props(
  props: &RwLock<HashMap<String, InterpreterValue>>,
) -> Vec<(String, InterpreterValue)> {
  props
    .read()
    .iter()
    .map(|(k, v)| (k.clone(), v.clone()))
    .collect()
}

/// 解释器函数值 → 宿主原生支撑的函数对象, 携带其已脱锁的自有属性
#[allow(clippy::too_many_arguments)]
fn interp_callable_to_js(
  native: NativeFn,
  name: Option<String>,
  params: Vec<String>,
  src: Option<String>,
  entries: Vec<(String, InterpreterValue)>,
  globals: &GlobalsTable,
  visited: &mut HashSet<usize>,
  depth: usize,
) -> JsValue {
  let properties: HashMap<String, JsValue> = entries
    .into_iter()
    .map(|(k, v)| {
      (
        k,
        interp_to_js_with_visited(&v, visited, depth + 1, globals),
      )
    })
    .collect();
  JsValue::Function(Arc::new(FunctionData {
    shared: Arc::new(SharedFunctionInfo::new_with_src(
      name,
      params,
      Vec::new(),
      src,
      None,
    )),
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
    // 洞哨兵双向 1:1 直通 (战役 C ③): 不再降级 Undefined, 数组洞跨桥保身份
    JsValue::TheHole => Some(InterpreterValue::TheHole),
    JsValue::Null => Some(InterpreterValue::Null),
    JsValue::Boolean(b) => Some(InterpreterValue::Boolean(*b)),
    JsValue::Number(n) => {
      if fits_smi(*n) {
        Some(InterpreterValue::Smi(*n as i32))
      } else {
        Some(InterpreterValue::Number(*n))
      }
    }
    JsValue::BigInt(n) => {
      let mut buf = Buffer::new();
      Some(InterpreterValue::BigInt(buf.format(*n).to_string()))
    }
    JsValue::String(s) => Some(InterpreterValue::new_string(s.clone())),
    JsValue::Symbol(s) => Some(InterpreterValue::Symbol(s.clone())),
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
    // 洞哨兵双向 1:1 直通 (战役 C ③): 不再降级 Undefined, 数组洞跨桥保身份
    JsValue::TheHole => Some(InterpreterValue::TheHole),
    JsValue::Null => Some(InterpreterValue::Null),
    JsValue::Boolean(b) => Some(InterpreterValue::Boolean(*b)),
    JsValue::Number(n) => {
      if fits_smi(*n) {
        Some(InterpreterValue::Smi(*n as i32))
      } else {
        Some(InterpreterValue::Number(*n))
      }
    }
    JsValue::BigInt(n) => {
      let mut buf = Buffer::new();
      Some(InterpreterValue::BigInt(buf.format(*n).to_string()))
    }
    JsValue::String(s) => Some(InterpreterValue::new_string(s.clone())),
    JsValue::Symbol(s) => Some(InterpreterValue::Symbol(s.clone())),
    JsValue::Array(arr) => {
      let ptr = Arc::as_ptr(arr) as usize;
      if !visited.insert(ptr) {
        return Some(InterpreterValue::new_array());
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
      Some(InterpreterValue::new_array_with_elements(res))
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
        // __proto__/constructor/prototype 必须一并带过去: 宿主 Error.prototype 上的 constructor
        // 一旦被丢, `thrown.constructor.name` 这类规范要求的反射就全塌 (visited 集合保证成环链终止)
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
      if let NativeFn::InterpFunction(closure, _) = native {
        return Some(InterpreterValue::Function(Arc::clone(closure)));
      }
      let native_clone = native.clone();
      let interp_fn = InterpNativeFn::from_context(native_clone, |nat, _receiver, args| {
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
      // 宿主 JS 函数过桥后走引擎 Native 臂 (不派生 length/name): 合成面在此预置形参位/名字,
      // 否则 prelude 函数 (hasOwn/raw/groupBy/assign…) 的 `.length` 在 BC 链全 undefined
      props
        .entry(LENGTH_KEY.to_string())
        .or_insert_with(|| InterpreterValue::Smi(func_data.shared.params.len() as i32));
      props.entry(NAME_KEY.to_string()).or_insert_with(|| {
        InterpreterValue::new_string(func_data.shared.name.clone().unwrap_or_default())
      });
      let props_arc = Arc::new(RwLock::new(props));

      if let Some(ref native) = func_data.native_impl {
        let interp_fn = if let NativeFn::InterpNative(orig) = native {
          orig.clone()
        } else {
          let native_clone = native.clone();
          InterpNativeFn::from_context(native_clone, |nat, _receiver, args| {
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
        let interp_fn =
          InterpNativeFn::from_context(func_data_clone, |func_data_clone, receiver, args| {
            let js_args: Vec<JsValue> = args.iter().map(JsValue::from).collect();
            // 调用点的 this 必须带过去: 宿主预置脚本里的 Object.prototype.toString 这类方法只靠
            // receiver 取接收者。receiver 为 Undefined 时保留 None, 沿用宿主自身的 this 解析
            let this_opt = match receiver {
              InterpreterValue::Undefined => None,
              other => Some(JsValue::from(other)),
            };
            // 派生实例的 global 必须吃函数的词法闭包链: 体内 `this.<内建>` 的原型回落
            // (proto_prop) 靠它, 空环境会让整个原型面在 BC 回调里失明
            let mut interp =
              Interpreter::for_cross_engine_call(Arc::clone(&func_data_clone.closure));
            match interp.invoke_function(
              &JsValue::Function(Arc::clone(func_data_clone)),
              this_opt,
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
      // C-2b 第二刀: 引擎 `Value::RegExp` 直持 `Arc<RegExpCore>` —— 过桥只拷一个指针,
      // pattern/flags/六布尔/lastIndex/exec/test/toString 全部由引擎读臂就地从内核供给
      // (一处定义在引擎 regexp.rs), 旧"物化成普通对象 + @@split 哨兵 + @@shadow 墓碑 +
      // 伪键名册"整套代管机器退役。expando 袋仍逐键过桥转换 (与 Native props 同形,
      // 袋的身份合一挂账 ④ 删桥)
      let ptr = Arc::as_ptr(re) as usize;
      if !visited.insert(ptr) {
        return Some(InterpreterValue::new_regexp(Arc::clone(&re.core)));
      }
      let mut bag = HashMap::default();
      for (k, v) in re.props.read().iter() {
        if let Some(iv) = js_value_to_interp_with_visited(v, visited, depth + 1) {
          bag.insert(k.clone(), iv);
        }
      }
      visited.remove(&ptr);
      Some(InterpreterValue::RegExp(
        Arc::clone(&re.core),
        Arc::new(RwLock::new(bag)),
      ))
    }
  }
}

/// 尝试以 Ignition 扁平字节码数组流调度执行语句序列 (支持外部全局环境双向同步)
///
/// 自建一张空全局表, 用于一次性执行与单元测试; 常驻执行请用
/// [`try_execute_with_bytecode_in`], 由调用方持有表以复用绑定与引用身份。
#[inline]
pub fn try_execute_with_bytecode_and_env(
  stmts: &[Stmt],
  global_env: Option<&Environment>,
) -> Option<Result<JsValue>> {
  try_execute_with_bytecode_in(stmts, global_env, &GlobalsTable::default())
}

/// 在全局表 `globals` 上执行语句序列
///
/// 表常驻 = 原版的 NativeContext: 段与段之间不再把上千个内建深转换一遍, 函数/对象也不会
/// 因往返宿主而换成新副本 (`thrown.constructor === ErrCtor` 这类身份比较全靠它)。
pub fn try_execute_with_bytecode_in(
  stmts: &[Stmt],
  global_env: Option<&Environment>,
  globals: &GlobalsTable,
) -> Option<Result<JsValue>> {
  let mut locals = HashSet::default();
  collect_stmts_vars(stmts, &mut locals);

  let mut dispatcher = BytecodeDispatcher::with_globals(Arc::clone(globals));

  // 若传入外部词法环境，将其中标量与对象全局变量注入字节码全局表 (逐层回溯父作用域，确保 arguments 与闭包变量完整导入)
  if let Some(env) = global_env {
    let mut curr_env = Some(env);
    while let Some(curr) = curr_env {
      let guard = curr.values.read();
      for (k, v) in guard.iter() {
        if !dispatcher.has_global(k)
          && let Some(interp_val) = js_value_to_interpreter_value(v)
        {
          dispatcher.set_global(k, interp_val);
        }
      }
      curr_env = curr.parent.as_deref();
    }
  }

  if let Some(re) = dispatcher.get_global("RegExp") {
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
      let table = dispatcher.global_table();
      // 执行成功后，把本次变更的全局同步写回宿主环境 (先脱锁复制再批处理写,
      // 免得持宿主写锁期间做深转换; 函数值必须带上这张表, 宿主回调它时才看得见全局)
      if let Some(env) = global_env {
        let mut batch: Vec<(String, JsValue)> = Vec::new();
        {
          let guard = table.read();
          for k in dispatcher.dirty_globals() {
            if let Some(v) = guard.get(k) {
              batch.push((
                k.clone(),
                interpreter_value_to_js_value_with_globals(v, table),
              ));
            }
          }
        }
        let mut guard = env.values.write();
        for (k, v) in batch {
          guard.insert(k, v);
        }
      }
      Some(Ok(interpreter_value_to_js_value_with_globals(&val, table)))
    }
    Err(e) => Some(Err(Error::RuntimeError(e.to_string()))),
  }
}

/// 尝试以 Ignition 扁平字节码数组流调度执行语句序列
#[inline]
pub fn try_execute_with_bytecode(stmts: &[Stmt]) -> Option<Result<JsValue>> {
  try_execute_with_bytecode_and_env(stmts, None)
}
