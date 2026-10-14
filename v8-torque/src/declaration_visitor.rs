// 由 tool/src/torque/visitor.js 从 v8/src/torque/declaration-visitor.h 生成; 修订就地改, 复跑覆盖
// 预声明 + 声明访问器 (静态方法族 → 关联函数)

use crate::{
  ast::{Ast, AstNode, NodeId},
  constants::{FLOAT32_TYPE_STRING, FLOAT64_TYPE_STRING},
  context::{
    CurrentScopeGuard, SourcePositionScope, current_scope, current_source_position, error_continue,
    error_continue_at, get_source_file_map, report_error, with_language_server_data,
    with_type_oracle, with_type_oracle_mut,
  },
  declarable::{
    BuiltinFlag, BuiltinFlags, BuiltinKind, DeclarableData, DeclarableId, DeclarableKind,
    NamespaceData, QualifiedName, SpecializationRequester, add_callable_specialization,
    builtin_declaration_flags, callable_body, callable_common_of, display_generic_callable,
    display_type_vector, generic_declaration_node, generic_name, generic_parameters,
    get_callable_specialization, identifier_value, infer_specialization_types,
    is_builtin_declaration, is_intrinsic_declaration, is_macro_declaration, make_declarable,
    make_specialization_requester, resolve_type_alias, set_specialization_requester,
  },
  declarations::{BuiltinSpec, Declarations, MacroSpec},
  error::{Error, Result},
  global_context::GlobalContext,
  source_positions::SourcePosition,
  type_oracle::TypeOracle,
  type_visitor::TypeVisitor,
  types::{
    MaybeSpecializationKey, ParameterMode, Signature, SpecializationKey, TypeId, is_subtype_of,
  },
};

// 原版 PredeclarationVisitor/DeclarationVisitor 全静态 (CurrentScope 上下文) →
// Rust 关联函数, 上下文以 (Ast, GlobalContext) 显式传参; 返回值从指针改为 id。
//
// 填体约定 (登记偏差): 声明阶段对 AST 只读, 节点数据一律经 cx.ast 读取
// (declarable/declarations 两段的助手函数均已依赖 cx.ast); 冻结签名保留的
// `ast: &mut Ast` 参数在体内仅作透传递归, 不作为读取通道 —— 避免调用方以
// `(&mut cx, &mut cx.ast)` 触发拆分借用冲突, 调用方可为 ast 传入占位实例。

/// 取声明节点整体位置 (原版 decl->pos)
fn decl_position(cx: &GlobalContext, decl: NodeId) -> SourcePosition {
  cx.ast.node(decl).0.pos
}

/// 取可调用声明族 (6 kind + SpecializationDeclaration) 的 name 标识符节点
/// (原版 declaration->name / decl->name, 供 name->value 与 name->pos)
fn callable_declaration_name(cx: &GlobalContext, decl: NodeId) -> Result<NodeId> {
  // v8: src/torque/ast.h CallableDeclaration::name
  Ok(match &cx.ast.node(decl).1 {
    AstNode::TorqueMacroDeclaration(d) => d.name,
    AstNode::TorqueBuiltinDeclaration(d) => d.name,
    AstNode::ExternalMacroDeclaration(d) => d.name,
    AstNode::ExternalBuiltinDeclaration(d) => d.name,
    AstNode::ExternalRuntimeDeclaration(d) => d.name,
    AstNode::IntrinsicDeclaration(d) => d.name,
    AstNode::SpecializationDeclaration(d) => d.name,
    _ => {
      return Err(Error::Message(
        "internal: not a callable declaration".into(),
      ));
    }
  })
}

/// 取类型声明族 (5 kind) 的 name 标识符节点 (原版 decl->name)
fn type_declaration_name_node(cx: &GlobalContext, decl: NodeId) -> Result<NodeId> {
  // v8: src/torque/ast.h TypeDeclaration::name
  Ok(match &cx.ast.node(decl).1 {
    AstNode::AbstractTypeDeclaration(d) => d.name,
    AstNode::TypeAliasDeclaration(d) => d.name,
    AstNode::BitFieldStructDeclaration(d) => d.name,
    AstNode::ClassDeclaration(d) => d.name,
    AstNode::StructDeclaration(d) => d.name,
    _ => return Err(Error::Message("internal: not a type declaration".into())),
  })
}

/// 取可调用声明族 return_type 类型表达式节点 (原版 decl->return_type)。
/// 报错定位用 (CreateBuiltin 的 JS 返回值错误挂 return_type 位置)。
fn callable_declaration_return_type_pos(
  cx: &GlobalContext,
  decl: NodeId,
) -> Result<SourcePosition> {
  let node = match &cx.ast.node(decl).1 {
    AstNode::TorqueBuiltinDeclaration(d) => d.return_type,
    AstNode::ExternalBuiltinDeclaration(d) => d.return_type,
    _ => return Err(Error::Message("internal: not a builtin declaration".into())),
  };
  Ok(cx.ast.node(node).0.pos)
}

/// 取可调用声明族 parameters.types 类型表达式节点表 (原版 decl->parameters.types)。
fn callable_declaration_parameter_type_nodes(
  cx: &GlobalContext,
  decl: NodeId,
) -> Result<Vec<NodeId>> {
  Ok(match &cx.ast.node(decl).1 {
    AstNode::TorqueBuiltinDeclaration(d) => d.parameters.types.clone(),
    AstNode::ExternalBuiltinDeclaration(d) => d.parameters.types.clone(),
    _ => return Err(Error::Message("internal: not a builtin declaration".into())),
  })
}

/// 原版 operator<<(std::ostream&, SourcePosition) (source-positions.h:116-121):
/// "https://crsrc.org/c/v8/{PathFromV8Root}?l={line+1}&c={col+1}"。
/// 登记偏差: 类型段未提供该流助手, 就近局部实现 (路径不可得时以空串兜底,
/// 原版 PathFromV8Root 对未登记源文件 CHECK 崩溃, Rust 不允许 panic)。
fn source_position_display(pos: SourcePosition) -> String {
  let path = get_source_file_map()
    .path_from_v8_root(pos.source)
    .unwrap_or_default();
  format!(
    "https://crsrc.org/c/v8/{path}?l={}&c={}",
    pos.start.line + 1,
    pos.start.column + 1
  )
}

/// 原版 StructType::DynamicCast 沿父链探测 (types.cc:136-143 Type::StructSupertype)。
/// 登记偏差: 类型段未提供该方法, 就近局部实现; CreateBuiltin 仅消费其存在性。
fn has_struct_supertype(oracle: &TypeOracle, type_: TypeId) -> bool {
  let mut current = Some(type_);
  while let Some(id) = current {
    let t = &oracle.types[id as usize];
    if t.is_struct_type() {
      return true;
    }
    current = t.parent;
  }
  false
}

/// 原版 PrintSignature(os, sig, with_names=true) (types.cc:1076-1107)。
/// 登记偏差: 类型段未填体, 就近局部实现 (与 declarable.rs 显示助手同源)。
fn print_signature_display(cx: &GlobalContext, oracle: &TypeOracle, sig: &Signature) -> String {
  // v8: src/torque/types.cc PrintSignature
  let mut result = String::from("(");
  for (i, t) in sig.parameter_types.types.iter().enumerate() {
    if i == 0 && sig.implicit_count != 0 {
      result.push_str("implicit ");
    }
    if sig.implicit_count > 0 && sig.implicit_count == i {
      result.push_str(")(");
    } else if i > 0 {
      result.push_str(", ");
    }
    if !sig.parameter_names.is_empty() && i < sig.parameter_names.len() {
      result.push_str(&identifier_value(&cx.ast, sig.parameter_names[i]));
      result.push_str(": ");
    }
    result.push_str(&oracle.types[*t as usize].to_display_string(cx, oracle));
  }
  if sig.parameter_types.var_args {
    if !sig.parameter_names.is_empty() {
      result.push_str(", ");
    }
    result.push_str("...");
  }
  result.push(')');
  result.push_str(": ");
  result.push_str(
    &sig
      .return_type
      .map(|ret| oracle.types[ret as usize].to_display_string(cx, oracle))
      .unwrap_or_default(),
  );
  if sig.labels.is_empty() {
    return result;
  }
  result.push_str(" labels ");
  for (i, label) in sig.labels.iter().enumerate() {
    if i > 0 {
      result.push_str(", ");
    }
    result.push_str(&identifier_value(&cx.ast, label.name));
    if !label.types.is_empty() {
      result.push('(');
      result.push_str(&display_type_vector(cx, oracle, &label.types));
      result.push(')');
    }
  }
  result
}

/// 冻结签名 key 为 Option 别名 (MaybeSpecializationKey), 体内解包。
fn unwrap_key(key: &MaybeSpecializationKey) -> Result<&SpecializationKey> {
  key
    .as_ref()
    .ok_or_else(|| Error::Message("internal: specialization key missing".into()))
}

/// 原版 GetOrCreateNamespace (declaration-visitor.cc:17-25)
pub fn get_or_create_namespace(cx: &mut GlobalContext, name: &str) -> Result<DeclarableId> {
  // v8: src/torque/declaration-visitor.cc GetOrCreateNamespace
  let scope =
    current_scope().ok_or_else(|| Error::Message("internal: CurrentScope is null".into()))?;
  let found = Declarations::try_lookup_shallow(cx, scope, &QualifiedName::plain(name.to_owned()))?;
  let existing_namespaces = Declarations::filter_declarables(cx, &found, DeclarableKind::Namespace);
  if existing_namespaces.is_empty() {
    return Declarations::declare_namespace(cx, name);
  }
  // 原版 DCHECK_EQ(1, size)
  debug_assert_eq!(existing_namespaces.len(), 1);
  Ok(existing_namespaces[0])
}

/// 原版 PredeclarationVisitor
pub struct PredeclarationVisitor;

impl PredeclarationVisitor {
  /// 原版 Predeclare(Ast) — 入口 (declaration-visitor.h:24-27)
  pub fn predeclare(cx: &mut GlobalContext, ast: &mut Ast) -> Result<()> {
    // v8: src/torque/declaration-visitor.h PredeclarationVisitor::Predeclare(Ast*)
    let _ = ast; // 声明阶段只读 AST, 节点数据走 cx.ast (见文件头约定)
    let default_namespace = cx
      .default_namespace()
      .ok_or_else(|| Error::Message("internal: default namespace missing".into()))?;
    let _scope_guard = CurrentScopeGuard::new(Some(default_namespace));
    let top_level: Vec<NodeId> = cx.ast.declarations.clone();
    for child in top_level {
      Self::predeclare_decl(cx, ast, child)?;
    }
    Ok(())
  }

  /// 原版 ResolvePredeclarations (declaration-visitor.cc:473-483)
  pub fn resolve_predeclarations(cx: &mut GlobalContext) -> Result<()> {
    // v8: src/torque/declaration-visitor.cc PredeclarationVisitor::ResolvePredeclarations
    let count = cx.all_declarables().len() as DeclarableId;
    for id in 0..count {
      if !cx.declarable(id).is_type_alias() {
        continue;
      }
      let (parent_scope, position) = {
        let d = cx.declarable(id);
        (d.parent_scope, d.position)
      };
      let _scope_guard = CurrentScopeGuard::new(parent_scope);
      let _position_guard = SourcePositionScope::new(position);
      // 原版 alias->Resolve() 惰性计算并缓存 (错误经 Error().Throw() 中止)
      resolve_type_alias(cx, id)?;
    }
    Ok(())
  }

  /// 原版 Predeclare(Declaration*) — 按 kind 分派 (declaration-visitor.cc:27-46)
  fn predeclare_decl(cx: &mut GlobalContext, ast: &mut Ast, decl: NodeId) -> Result<()> {
    // v8: src/torque/declaration-visitor.cc PredeclarationVisitor::Predeclare(Declaration*)
    let _ = ast; // 见文件头约定
    let pos = decl_position(cx, decl);
    let _position_guard = SourcePositionScope::new(pos);
    match &cx.ast.node(decl).1 {
      // AST_TYPE_DECLARATION_NODE_KIND_LIST (5 kind) → Predeclare(TypeDeclaration*)
      AstNode::AbstractTypeDeclaration(d) => {
        let name = d.name;
        Self::predeclare_type_declaration(cx, name, decl, pos)
      }
      AstNode::TypeAliasDeclaration(d) => {
        let name = d.name;
        Self::predeclare_type_declaration(cx, name, decl, pos)
      }
      AstNode::BitFieldStructDeclaration(d) => {
        let name = d.name;
        Self::predeclare_type_declaration(cx, name, decl, pos)
      }
      AstNode::ClassDeclaration(d) => {
        let name = d.name;
        Self::predeclare_type_declaration(cx, name, decl, pos)
      }
      AstNode::StructDeclaration(d) => {
        let name = d.name;
        Self::predeclare_type_declaration(cx, name, decl, pos)
      }
      AstNode::NamespaceDeclaration(d) => {
        let ns_name = d.name.clone();
        let children = d.declarations.clone();
        let scope_id = get_or_create_namespace(cx, &ns_name)?;
        let _scope_guard = CurrentScopeGuard::new(Some(scope_id));
        for child in children {
          Self::predeclare_decl(cx, ast, child)?;
        }
        Ok(())
      }
      AstNode::GenericCallableDeclaration(d) => {
        let inner = d.declaration;
        let name_node = callable_declaration_name(cx, inner)?;
        let name_value = identifier_value(&cx.ast, name_node);
        Declarations::declare_generic_callable(cx, &name_value, decl)?;
        Ok(())
      }
      AstNode::GenericTypeDeclaration(d) => {
        let inner = d.declaration;
        let name_node = type_declaration_name_node(cx, inner)?;
        let name_value = identifier_value(&cx.ast, name_node);
        Declarations::declare_generic_type(cx, &name_value, decl)?;
        Ok(())
      }
      _ => {
        // 原版 default: break —— 仅处理类型声明、命名空间与泛型
        Ok(())
      }
    }
  }

  /// 原版 Predeclare(TypeDeclaration*) (declaration-visitor.h:36-44, Struct 同体)
  fn predeclare_type_declaration(
    cx: &mut GlobalContext,
    name_node: NodeId,
    decl: NodeId,
    pos: SourcePosition,
  ) -> Result<()> {
    // v8: src/torque/declaration-visitor.h PredeclarationVisitor::Predeclare(TypeDeclaration*)
    let alias = Declarations::predeclare_type_alias(cx, name_node, decl, false)?;
    let name_pos = cx.ast.node(name_node).0.pos;
    {
      let d = cx.declarable_mut(alias);
      d.set_position(pos);
      d.set_identifier_position(name_pos);
    }
    if cx.collect_kythe_data {
      // 登记: KytheData 段未填体 (原版 KytheData::AddTypeDefinition)
      return Err(Error::Unimplemented);
    }
    Ok(())
  }
}

/// 原版 DeclarationVisitor
pub struct DeclarationVisitor;

impl DeclarationVisitor {
  /// 原版 Visit(Ast) — 入口 (declaration-visitor.h:66-69)
  pub fn visit_ast(cx: &mut GlobalContext, ast: &mut Ast) -> Result<()> {
    // v8: src/torque/declaration-visitor.h DeclarationVisitor::Visit(Ast*)
    let _ = ast; // 见文件头约定
    let default_namespace = cx
      .default_namespace()
      .ok_or_else(|| Error::Message("internal: default namespace missing".into()))?;
    let _scope_guard = CurrentScopeGuard::new(Some(default_namespace));
    let top_level: Vec<NodeId> = cx.ast.declarations.clone();
    for child in top_level {
      Self::visit_decl(cx, ast, child)?;
    }
    Ok(())
  }

  /// 原版 Visit(Declaration*) — 按 kind 分派 (declaration-visitor.cc:48-59)
  pub fn visit_decl(cx: &mut GlobalContext, ast: &mut Ast, decl: NodeId) -> Result<Option<TypeId>> {
    // v8: src/torque/declaration-visitor.cc DeclarationVisitor::Visit(Declaration*)
    let _ = ast; // 见文件头约定
    let pos = decl_position(cx, decl);
    let _position_guard = SourcePositionScope::new(pos);
    match &cx.ast.node(decl).1 {
      // AST_DECLARATION_NODE_KIND_LIST (18 kind) 逐一对位原版重载
      AstNode::AbstractTypeDeclaration(d) => {
        let name = d.name;
        Self::visit_type_declaration(cx, name)
      }
      AstNode::TypeAliasDeclaration(d) => {
        let name = d.name;
        Self::visit_type_declaration(cx, name)
      }
      AstNode::BitFieldStructDeclaration(d) => {
        let name = d.name;
        Self::visit_type_declaration(cx, name)
      }
      AstNode::ClassDeclaration(d) => {
        let name = d.name;
        Self::visit_type_declaration(cx, name)
      }
      AstNode::StructDeclaration(d) => {
        let name = d.name;
        Self::visit_type_declaration(cx, name)
      }
      AstNode::GenericCallableDeclaration(_) | AstNode::GenericTypeDeclaration(_) => {
        // 原版空实现: PredeclarationVisitor 已处理
        Ok(None)
      }
      AstNode::NamespaceDeclaration(d) => {
        let ns_name = d.name.clone();
        let children = d.declarations.clone();
        // 原版 Visit(NamespaceDeclaration*) 头文件内联: 入命名空间作用域后递归
        let scope_id = get_or_create_namespace(cx, &ns_name)?;
        let _scope_guard = CurrentScopeGuard::new(Some(scope_id));
        for child in children {
          Self::visit_decl(cx, ast, child)?;
        }
        Ok(None)
      }
      AstNode::SpecializationDeclaration(d) => {
        let name_node = d.name;
        let generic_arg_exprs = d.generic_parameters.clone();
        let body = d.body;
        Self::visit_specialization(cx, decl, name_node, &generic_arg_exprs, body, pos)?;
        Ok(None)
      }
      AstNode::ExternConstDeclaration(d) => {
        let name_node = d.name;
        let type_node = d.r#type;
        let literal = d.literal.clone();
        // v8: src/torque/declaration-visitor.cc Visit(ExternConstDeclaration*)
        let type_ = with_type_oracle_mut(|oracle| TypeVisitor::compute_type(oracle, type_node))?;
        let is_constexpr =
          with_type_oracle(|oracle| oracle.types[type_ as usize].is_constexpr(oracle));
        if !is_constexpr {
          let disp =
            with_type_oracle(|oracle| oracle.types[type_ as usize].to_display_string(cx, oracle));
          return Err(report_error(format!(
            "extern constants must have constexpr type, but found: \"{disp}\"\n"
          )));
        }
        Declarations::declare_extern_constant(cx, name_node, type_, literal)?;
        if cx.collect_kythe_data {
          // 登记: KytheData 段未填体 (原版 KytheData::AddConstantDefinition)
          return Err(Error::Unimplemented);
        }
        Ok(None)
      }
      AstNode::ConstDeclaration(d) => {
        let name_node = d.name;
        let type_node = d.r#type;
        let expression = d.expression;
        // v8: src/torque/declaration-visitor.cc Visit(ConstDeclaration*)
        let type_ = with_type_oracle_mut(|oracle| TypeVisitor::compute_type(oracle, type_node))?;
        Declarations::declare_namespace_constant(cx, name_node, type_, expression)?;
        if cx.collect_kythe_data {
          // 登记: KytheData 段未填体 (原版 KytheData::AddConstantDefinition)
          return Err(Error::Unimplemented);
        }
        Ok(None)
      }
      AstNode::CppIncludeDeclaration(d) => {
        let path = d.include_path.clone();
        let selector = d.include_selector;
        // v8: src/torque/declaration-visitor.cc Visit(CppIncludeDeclaration*)
        cx.add_cpp_include(path, selector);
        Ok(None)
      }
      AstNode::TorqueMacroDeclaration(d) => {
        let name_node = d.name;
        let op = d.op.clone();
        let export_to_csa = d.export_to_csa;
        let supports_tsa = d.supports_tsa;
        let body = d.body;
        // v8: src/torque/declaration-visitor.cc Visit(TorqueMacroDeclaration*)
        let name_value = identifier_value(&cx.ast, name_node);
        let signature = TypeVisitor::make_signature(decl)?;
        let spec = MacroSpec {
          name: name_value.clone(),
          accessible_from_csa: export_to_csa,
          external_assembler_name: None,
          op,
          is_user_defined: true,
        };
        let macro_ = Declarations::declare_macro(cx, spec, signature, body)?;
        let name_pos = cx.ast.node(name_node).0.pos;
        {
          let d = cx.declarable_mut(macro_);
          d.set_identifier_position(name_pos);
          d.set_position(pos);
          if supports_tsa {
            // 原版 SetSupportsTSA(true)
            if let DeclarableData::TorqueMacro(m) = &mut d.data {
              m.macro_.supports_tsa = true;
            }
          }
        }
        if cx.collect_kythe_data {
          // 登记: KytheData 段未填体 (原版 KytheData::AddFunctionDefinition)
          return Err(Error::Unimplemented);
        }
        Ok(None)
      }
      AstNode::TorqueBuiltinDeclaration(d) => {
        let name_node = d.name;
        let use_counter_name = d.use_counter_name.clone();
        let body = d.body;
        // v8: src/torque/declaration-visitor.cc Visit(TorqueBuiltinDeclaration*)
        let name_value = identifier_value(&cx.ast, name_node);
        let signature = TypeVisitor::make_signature(decl)?;
        let native_context = with_type_oracle(|oracle| oracle.native_context_type())?;
        let context = with_type_oracle(|oracle| oracle.context_type())?;
        if use_counter_name.is_some() {
          let first = signature.parameter_types.types.first().copied();
          if first.is_none_or(|t| t != native_context && t != context) {
            // 登记偏差: 原版空参时仍流式读 types[0] (越界), Rust 以空显示安全兜底
            let disp = first.map_or_else(String::new, |t| {
              with_type_oracle(|oracle| oracle.types[t as usize].to_display_string(cx, oracle))
            });
            return Err(report_error(format!(
              "@incrementUseCounter requires the builtin's first parameter to be of type Context or NativeContext, but found type {disp}"
            )));
          }
        }
        let builtin = Self::create_builtin(
          cx,
          decl,
          &name_value,
          &name_value,
          signature,
          use_counter_name,
          body,
        )?;
        let name_pos = cx.ast.node(name_node).0.pos;
        {
          let d = cx.declarable_mut(builtin);
          d.set_identifier_position(name_pos);
          d.set_position(pos);
        }
        if cx.collect_kythe_data {
          // 登记: KytheData 段未填体 (原版 KytheData::AddFunctionDefinition)
          return Err(Error::Unimplemented);
        }
        Declarations::declare(cx, &name_value, builtin)?;
        Ok(None)
      }
      AstNode::ExternalMacroDeclaration(d) => {
        let name_node = d.name;
        let external_assembler_name = d.external_assembler_name.clone();
        let op = d.op.clone();
        // v8: src/torque/declaration-visitor.cc Visit(ExternalMacroDeclaration*)
        let name_value = identifier_value(&cx.ast, name_node);
        let signature = TypeVisitor::make_signature(decl)?;
        let spec = MacroSpec {
          name: name_value.clone(),
          accessible_from_csa: true,
          external_assembler_name: Some(external_assembler_name),
          op,
          is_user_defined: true,
        };
        let macro_ = Declarations::declare_macro(cx, spec, signature, None)?;
        let name_pos = cx.ast.node(name_node).0.pos;
        {
          let d = cx.declarable_mut(macro_);
          d.set_identifier_position(name_pos);
          d.set_position(pos);
        }
        if cx.collect_kythe_data {
          // 登记: KytheData 段未填体 (原版 KytheData::AddFunctionDefinition)
          return Err(Error::Unimplemented);
        }
        Ok(None)
      }
      AstNode::ExternalBuiltinDeclaration(d) => {
        let name_node = d.name;
        // v8: src/torque/declaration-visitor.cc Visit(ExternalBuiltinDeclaration*)
        let name_value = identifier_value(&cx.ast, name_node);
        let signature = TypeVisitor::make_signature(decl)?;
        let builtin =
          Self::create_builtin(cx, decl, &name_value, &name_value, signature, None, None)?;
        let name_pos = cx.ast.node(name_node).0.pos;
        cx.declarable_mut(builtin).set_identifier_position(name_pos);
        Declarations::declare(cx, &name_value, builtin)?;
        Ok(None)
      }
      AstNode::ExternalRuntimeDeclaration(d) => {
        let name_node = d.name;
        // v8: src/torque/declaration-visitor.cc Visit(ExternalRuntimeDeclaration*)
        let name_value = identifier_value(&cx.ast, name_node);
        let signature = TypeVisitor::make_signature(decl)?;
        if signature.parameter_types.types.is_empty() {
          return Err(report_error(
            "Missing parameters for runtime function, at least the context parameter is required."
              .to_owned(),
          ));
        }
        let first = signature.parameter_types.types[0];
        let context = with_type_oracle(|oracle| oracle.context_type())?;
        let no_context = with_type_oracle(|oracle| oracle.no_context_type())?;
        if first != context && first != no_context {
          let disp =
            with_type_oracle(|oracle| oracle.types[first as usize].to_display_string(cx, oracle));
          return Err(report_error(format!(
            "first parameter to runtime functions has to be the context and have type Context or NoContext, but found type {disp}"
          )));
        }
        let strong_tagged = with_type_oracle(|oracle| oracle.strong_tagged_type())?;
        let void_type = with_type_oracle(|oracle| oracle.void_type())?;
        let never_type = with_type_oracle(|oracle| oracle.never_type())?;
        // 原版 return_type 恒非空 (parser 保证); None 视为 internal
        let return_type = signature
          .return_type
          .ok_or_else(|| Error::Message("internal: signature without return type".into()))?;
        let return_ok = with_type_oracle(|oracle| {
          is_subtype_of(oracle, return_type, strong_tagged)
            || return_type == void_type
            || return_type == never_type
        });
        if !return_ok {
          let disp = with_type_oracle(|oracle| {
            oracle.types[return_type as usize].to_display_string(cx, oracle)
          });
          return Err(report_error(format!(
            "runtime functions can only return strong tagged values, but found type {disp}"
          )));
        }
        for parameter_type in signature.parameter_types.types.clone() {
          let ok = with_type_oracle(|oracle| is_subtype_of(oracle, parameter_type, strong_tagged));
          if !ok {
            let disp = with_type_oracle(|oracle| {
              oracle.types[parameter_type as usize].to_display_string(cx, oracle)
            });
            return Err(report_error(format!(
              "runtime functions can only take strong tagged parameters, but found type {disp}"
            )));
          }
        }
        let function = Declarations::declare_runtime_function(cx, &name_value, signature)?;
        let name_pos = cx.ast.node(name_node).0.pos;
        {
          let d = cx.declarable_mut(function);
          d.set_identifier_position(name_pos);
          d.set_position(pos);
        }
        if cx.collect_kythe_data {
          // 登记: KytheData 段未填体 (原版 KytheData::AddFunctionDefinition)
          return Err(Error::Unimplemented);
        }
        Ok(None)
      }
      AstNode::IntrinsicDeclaration(d) => {
        let name_node = d.name;
        // v8: src/torque/declaration-visitor.cc Visit(IntrinsicDeclaration*)
        let name_value = identifier_value(&cx.ast, name_node);
        let signature = TypeVisitor::make_signature(decl)?;
        Declarations::declare_intrinsic(cx, &name_value, signature)?;
        Ok(None)
      }
      _ => {
        // 原版 default: UNIMPLEMENTED()
        Err(Error::Unimplemented)
      }
    }
  }

  /// 原版 Visit(TypeDeclaration*/StructDeclaration*) (declaration-visitor.h:76-83):
  /// 查型触发类型计算, 保证未用类型的错误也被报告
  fn visit_type_declaration(cx: &GlobalContext, name_node: NodeId) -> Result<Option<TypeId>> {
    // v8: src/torque/declaration-visitor.h DeclarationVisitor::Visit(TypeDeclaration*)
    Ok(Some(Declarations::lookup_type_identifier(cx, name_node)?))
  }

  /// 原版 Visit(SpecializationDeclaration*) (declaration-visitor.cc:267-329)
  fn visit_specialization(
    cx: &mut GlobalContext,
    decl: NodeId,
    name_node: NodeId,
    generic_arg_exprs: &[NodeId],
    body: NodeId,
    pos: SourcePosition,
  ) -> Result<()> {
    // v8: src/torque/declaration-visitor.cc DeclarationVisitor::Visit(SpecializationDeclaration*)
    let name_value = identifier_value(&cx.ast, name_node);
    let generic_list = Declarations::lookup_generic(cx, &name_value)?;
    let signature_with_types = TypeVisitor::make_signature(decl)?;
    // 原版 TypeVisitor::ComputeTypeVector(decl->generic_parameters)
    let types =
      with_type_oracle_mut(|oracle| TypeVisitor::compute_type_vector(oracle, generic_arg_exprs))?;

    let mut matching_generic: Option<DeclarableId> = None;
    for generic in generic_list.clone() {
      // 该推断仅为触发泛型实参的约束检查 (原版注释)
      let inference = with_type_oracle_mut(|oracle| {
        infer_specialization_types(cx, oracle, generic, &types, &[])
      })?;
      if inference.has_failed() {
        continue;
      }
      let key = Some(SpecializationKey {
        generic,
        specialized_types: types.clone(),
      });
      let generic_signature_with_types = Self::make_specialized_signature(cx, &key)?;
      if signature_with_types
        .has_same_types_as(&generic_signature_with_types, ParameterMode::IgnoreImplicit)
      {
        if let Some(previous) = matching_generic {
          let a = display_generic_callable(cx, previous)?;
          let b = display_generic_callable(cx, generic)?;
          return Err(report_error(format!(
            "specialization of {name_value} is ambigous, it matches more than one generic declaration ({a} and {b})"
          )));
        }
        matching_generic = Some(generic);
      }
    }

    let Some(matching_generic) = matching_generic else {
      if generic_list.is_empty() {
        return Err(report_error(format!(
          "no generic defined with the name {name_value}"
        )));
      }
      // 登记偏差: 原版在流式循环内即时算候选签名, Rust 先顺序算出再拼串
      // (避免 oracle 重入: make_specialized_signature 内部会触发 oracle 可变借用)
      let mut candidates = Vec::new();
      for generic in generic_list.clone() {
        let key = Some(SpecializationKey {
          generic,
          specialized_types: types.clone(),
        });
        candidates.push(Self::make_specialized_signature(cx, &key)?);
      }
      let sig_disp =
        with_type_oracle(|oracle| print_signature_display(cx, oracle, &signature_with_types));
      let mut stream = format!(
        "specialization of {name_value} doesn't match any generic declaration\nspecialization signature:\n  {sig_disp}\ncandidates are:"
      );
      let candidate_displays = with_type_oracle(|oracle| {
        candidates
          .iter()
          .map(|sig| format!("\n  {}", print_signature_display(cx, oracle, sig)))
          .collect::<Vec<_>>()
      });
      for disp in candidate_displays {
        stream.push_str(&disp);
      }
      return Err(report_error(stream));
    };

    if cx.collect_language_server_data {
      let token = cx.ast.node(name_node).0.pos;
      let definition = cx.declarable(matching_generic).identifier_position();
      with_language_server_data(|data| data.add_definition(token, definition));
    }

    let generic_declaration = generic_declaration_node(cx, matching_generic)?;
    let key = Some(SpecializationKey {
      generic: matching_generic,
      specialized_types: types,
    });
    Self::specialize(cx, &key, generic_declaration, Some(decl), Some(body), pos)?;
    Ok(())
  }

  /// 原版 CreateBuiltin (declaration-visitor.cc:61-160):
  /// 校验 + flags 组装 + Declarations::CreateBuiltin 登记
  pub fn create_builtin(
    cx: &mut GlobalContext,
    decl: NodeId,
    external_name: &str,
    readable_name: &str,
    signature: Signature,
    use_counter_name: Option<String>,
    body: Option<NodeId>,
  ) -> Result<DeclarableId> {
    // v8: src/torque/declaration-visitor.cc DeclarationVisitor::CreateBuiltin
    let (javascript, varargs, has_custom_interface_descriptor, supports_tsa, _decl_use_counter) =
      builtin_declaration_flags(cx, decl)?;
    let kind = if !javascript {
      BuiltinKind::Stub
    } else if varargs {
      BuiltinKind::VarArgsJavaScript
    } else {
      BuiltinKind::FixedArgsJavaScript
    };

    if varargs && !javascript {
      // 原版 Error(...) 不 Throw: 记录后继续校验
      let decl_name = {
        let name_node = callable_declaration_name(cx, decl)?;
        identifier_value(&cx.ast, name_node)
      };
      error_continue(format!(
        "Rest parameters require {decl_name} to be a JavaScript builtin"
      ));
    }

    // 原版 return_type 恒非空 (parser 保证); None 视为 internal
    let return_type = signature
      .return_type
      .ok_or_else(|| Error::Message("internal: signature without return type".into()))?;

    let is_external_builtin =
      matches!(&cx.ast.node(decl).1, AstNode::ExternalBuiltinDeclaration(_));

    if javascript {
      let jsany = with_type_oracle(|oracle| oracle.jsany_type())?;
      let return_ok = with_type_oracle(|oracle| is_subtype_of(oracle, return_type, jsany));
      if !return_ok {
        let return_type_pos = callable_declaration_return_type_pos(cx, decl)?;
        error_continue_at(
          "Return type of JavaScript-linkage builtins has to be JSAny.".to_owned(),
          return_type_pos,
        );
      }
      // 校验参数类型: torque 定义的 JS builtin 参数须为 JSAny 的父类型
      // ("extern javascript" 例外, 原版注释保留: 其常用于 tailcall, 需带
      //  JS 调用约定的隐式参数; CSA::CallJSBuiltin 会在 C++ 编译期再校验)
      if !is_external_builtin {
        let parameter_type_nodes = callable_declaration_parameter_type_nodes(cx, decl)?;
        for (i, parameter_type) in signature
          .parameter_types
          .types
          .iter()
          .copied()
          .enumerate()
          .skip(signature.implicit_count)
        {
          let ok = with_type_oracle(|oracle| is_subtype_of(oracle, jsany, parameter_type));
          if !ok {
            let pos = parameter_type_nodes
              .get(i)
              .map_or(pos_of_decl(cx, decl), |node| cx.ast.node(*node).0.pos);
            error_continue_at(
              "Parameters of JavaScript-linkage builtins have to be a supertype of JSAny."
                .to_owned(),
              pos,
            );
          }
        }
      }
    }

    for i in 0..signature.parameter_types.types.len() {
      let parameter_type = signature.parameter_types.types[i];
      let parameter_name = signature
        .parameter_names
        .get(i)
        .map_or_else(String::new, |node| identifier_value(&cx.ast, *node));
      let parameter_display = with_type_oracle(|oracle| {
        oracle.types[parameter_type as usize].to_display_string(cx, oracle)
      });
      let is_struct = with_type_oracle(|oracle| has_struct_supertype(oracle, parameter_type));
      if is_struct {
        error_continue(format!(
          "Builtin do not support structs as arguments, but argument {parameter_name} has type {parameter_display}."
        ));
      }
      let is_float = with_type_oracle(|oracle| {
        let t = &oracle.types[parameter_type as usize];
        t.is_abstract_name(FLOAT32_TYPE_STRING) || t.is_abstract_name(FLOAT64_TYPE_STRING)
      });
      if is_float && !has_custom_interface_descriptor {
        error_continue(format!(
          "Builtin {external_name} needs a custom interface descriptor, because it uses type {parameter_display} for argument {parameter_name}. One reason being that the default descriptor defines xmm0 to be the first floating point argument register, which is current used as scratch on ia32 and cannot be allocated."
        ));
      }
    }

    let return_is_struct = with_type_oracle(|oracle| has_struct_supertype(oracle, return_type));
    if return_is_struct && javascript {
      let disp =
        with_type_oracle(|oracle| oracle.types[return_type as usize].to_display_string(cx, oracle));
      error_continue(format!(
        "Builtins with JS linkage cannot return structs, but the return type is {disp}."
      ));
    }

    let void_type = with_type_oracle(|oracle| oracle.void_type())?;
    if return_type == void_type {
      error_continue("Builtins cannot have return type void.".to_owned());
    }

    let mut flags: BuiltinFlags = BuiltinFlag::None as BuiltinFlags;
    if has_custom_interface_descriptor {
      flags |= BuiltinFlag::CustomInterfaceDescriptor as BuiltinFlags;
    }
    if supports_tsa {
      flags |= BuiltinFlag::SupportsTsa as BuiltinFlags;
    }
    let builtin = Declarations::create_builtin(
      cx,
      BuiltinSpec {
        external_name: external_name.to_owned(),
        readable_name: readable_name.to_owned(),
        kind,
        flags,
        use_counter_name,
      },
      signature,
      body,
    )?;
    // TODO(v8:12261): Recheck this.
    // builtin->SetIdentifierPosition(decl->name->pos); (原版即注释)
    Ok(builtin)
  }

  /// 原版 MakeSpecializedSignature (declaration-visitor.cc:371-380)
  pub fn make_specialized_signature(
    cx: &mut GlobalContext,
    key: &MaybeSpecializationKey,
  ) -> Result<Signature> {
    // v8: src/torque/declaration-visitor.cc DeclarationVisitor::MakeSpecializedSignature
    let key = unwrap_key(key)?;
    let generic_parent_scope = cx.declarable(key.generic).parent_scope;
    let _generic_scope_guard = CurrentScopeGuard::new(generic_parent_scope);
    // 原版在栈上构造匿名 Namespace("_tmp") 不入 arena; Rust 版登记进 arena
    // (登记偏差: 语义对齐 —— 临时命名空间不挂在任何父作用域名字表上,
    //  仅作为本次签名计算期 CurrentScope 的别名容器, 事后不可达)
    let tmp_namespace = make_declarable(
      DeclarableKind::Namespace,
      DeclarableData::Namespace(NamespaceData {
        name: "_tmp".to_owned(),
      }),
      true,
    );
    let tmp_id = cx.register_declarable(tmp_namespace);
    let _tmp_scope_guard = CurrentScopeGuard::new(Some(tmp_id));
    let key_ref = Some(key.clone());
    Self::declare_specialized_types(cx, &key_ref)?;
    let declaration = generic_declaration_node(cx, key.generic)?;
    TypeVisitor::make_signature(declaration)
  }

  /// 原版 SpecializeImplicit (declaration-visitor.cc:382-402)
  pub fn specialize_implicit(
    cx: &mut GlobalContext,
    key: &MaybeSpecializationKey,
  ) -> Result<DeclarableId> {
    // v8: src/torque/declaration-visitor.cc DeclarationVisitor::SpecializeImplicit
    let key = unwrap_key(key)?;
    let body = callable_body(cx, key.generic)?;
    let declaration = generic_declaration_node(cx, key.generic)?;
    if body.is_none() && !is_intrinsic_declaration(cx, declaration) {
      let name = generic_name(cx, key.generic)?;
      let generic_position = cx.declarable(key.generic).position;
      let types_display =
        with_type_oracle(|oracle| display_type_vector(cx, oracle, &key.specialized_types));
      return Err(report_error(format!(
        "missing specialization of {name} with types <{types_display}> declared at {}",
        source_position_display(generic_position)
      )));
    }
    // 原版 SpecializationRequester{CurrentSourcePosition::Get(), CurrentScope::Get(), ""}
    let requester = make_specialization_requester(
      cx,
      current_source_position(),
      current_scope(),
      String::new(),
    );
    let generic_parent_scope = cx.declarable(key.generic).parent_scope;
    let _generic_scope_guard = CurrentScopeGuard::new(generic_parent_scope);
    let key_ref = Some(key.clone());
    let result = Self::specialize(
      cx,
      &key_ref,
      declaration,
      None,
      body,
      current_source_position(),
    )?;
    cx.declarable_mut(result).is_user_defined = false;
    let readable_name = callable_common_of(cx.declarable(result))
      .map(|c| c.readable_name.clone())
      .ok_or_else(|| Error::Message("internal: specialization result not callable".into()))?;
    let requester = SpecializationRequester {
      name: readable_name,
      ..requester
    };
    set_specialization_requester(cx, result, &requester)?;
    let _callable_scope_guard = CurrentScopeGuard::new(Some(result));
    Self::declare_specialized_types(cx, &key_ref)?;
    Ok(result)
  }

  /// 原版 Specialize (declaration-visitor.cc:404-471)
  pub fn specialize(
    cx: &mut GlobalContext,
    key: &MaybeSpecializationKey,
    declaration: NodeId,
    explicit_specialization: Option<NodeId>,
    body: Option<NodeId>,
    position: SourcePosition,
  ) -> Result<DeclarableId> {
    // v8: src/torque/declaration-visitor.cc DeclarationVisitor::Specialize
    let key = unwrap_key(key)?;
    let _position_guard = SourcePositionScope::new(position);
    let generic_parameters = generic_parameters(cx, key.generic)?;
    if generic_parameters.len() != key.specialized_types.len() {
      let declaration_name = {
        let name_node = callable_declaration_name(cx, declaration)?;
        identifier_value(&cx.ast, name_node)
      };
      return Err(report_error(format!(
        "number of template parameters ({}) to intantiation of generic {declaration_name} doesnt match the generic's declaration ({})",
        key.specialized_types.len(),
        generic_parameters.len()
      )));
    }
    if get_callable_specialization(cx, key.generic, &key.specialized_types).is_some() {
      let name = generic_name(cx, key.generic)?;
      let types_display =
        with_type_oracle(|oracle| display_type_vector(cx, oracle, &key.specialized_types));
      return Err(report_error(format!(
        "cannot redeclare specialization of {name} with types <{types_display}>"
      )));
    }

    let type_signature = match explicit_specialization {
      Some(explicit_decl) => TypeVisitor::make_signature(explicit_decl)?,
      None => {
        let key_ref = Some(key.clone());
        Self::make_specialized_signature(cx, &key_ref)?
      }
    };

    let declaration_name = {
      let name_node = callable_declaration_name(cx, declaration)?;
      identifier_value(&cx.ast, name_node)
    };
    let generated_name =
      Declarations::get_generated_callable_name(cx, &declaration_name, &key.specialized_types)?;
    let types_display =
      with_type_oracle(|oracle| display_type_vector(cx, oracle, &key.specialized_types));
    let readable_name = format!("{declaration_name}<{types_display}>");

    let callable = if is_macro_declaration(cx, declaration) {
      let body_node =
        body.ok_or_else(|| Error::Message("internal: macro specialization without body".into()))?;
      Declarations::create_torque_macro(
        cx,
        &generated_name,
        &readable_name,
        false,
        type_signature,
        Some(body_node),
        true,
      )?
    } else if is_intrinsic_declaration(cx, declaration) {
      Declarations::create_intrinsic(cx, &declaration_name, type_signature)?
    } else {
      if !is_builtin_declaration(cx, declaration) {
        return Err(Error::Message(
          "internal: specialization of unsupported declaration kind".into(),
        ));
      }
      // 原版 BuiltinDeclaration::cast + TorqueBuiltinDeclaration::DynamicCast
      let use_counter_name = match &cx.ast.node(declaration).1 {
        AstNode::TorqueBuiltinDeclaration(d) => d.use_counter_name.clone(),
        AstNode::ExternalBuiltinDeclaration(_) => None,
        _ => return Err(Error::Message("internal: not a builtin declaration".into())),
      };
      let body_node = body
        .ok_or_else(|| Error::Message("internal: builtin specialization without body".into()))?;
      // 原版 GetContext().MakeUniqueName(generated_name) (当前线程上下文桥)
      let unique_external = cx.make_unique_name(&generated_name);
      Self::create_builtin(
        cx,
        declaration,
        &unique_external,
        &readable_name,
        type_signature,
        use_counter_name,
        Some(body_node),
      )?
    };

    let identifier_position = match explicit_specialization {
      Some(explicit_decl) => {
        let name_node = callable_declaration_name(cx, explicit_decl)?;
        cx.ast.node(name_node).0.pos
      }
      None => {
        let name_node = callable_declaration_name(cx, declaration)?;
        cx.ast.node(name_node).0.pos
      }
    };
    cx.declarable_mut(callable)
      .set_identifier_position(identifier_position);
    if cx.collect_kythe_data && cx.declarable(callable).kind != DeclarableKind::Intrinsic {
      // 登记: KytheData 段未填体 (原版 KytheData::AddFunctionDefinition)
      return Err(Error::Unimplemented);
    }
    with_type_oracle_mut(|oracle| {
      add_callable_specialization(cx, oracle, key.generic, &key.specialized_types, callable)
    })?;
    Ok(callable)
  }

  /// 原版 DeclareSpecializedTypes (declaration-visitor.cc:351-369, 私有)
  fn declare_specialized_types(cx: &mut GlobalContext, key: &MaybeSpecializationKey) -> Result<()> {
    // v8: src/torque/declaration-visitor.cc DeclarationVisitor::DeclareSpecializedTypes
    let key = unwrap_key(key)?;
    let parameters = generic_parameters(cx, key.generic)?;
    let generic_parameter_count = parameters.len();
    if generic_parameter_count != key.specialized_types.len() {
      let name = generic_name(cx, key.generic)?;
      return Err(report_error(format!(
        "Wrong generic argument count for specialization of \"{name}\", expected: {generic_parameter_count}, actual: {}",
        key.specialized_types.len()
      )));
    }
    for (i, type_) in key.specialized_types.iter().enumerate() {
      let generic_type_name = parameters[i].name;
      let alias = Declarations::declare_type(cx, generic_type_name, *type_)?;
      // 原版 SetIsUserDefined(false)
      cx.declarable_mut(alias).is_user_defined = false;
    }
    Ok(())
  }
}

/// 原版 CreateBuiltin 参数定位兜底: 参数类型节点缺失时退回声明整体位置
fn pos_of_decl(cx: &GlobalContext, decl: NodeId) -> SourcePosition {
  cx.ast.node(decl).0.pos
}

// ── 原版 src/torque/declaration-visitor.h ──

// ── 原版 src/torque/declaration-visitor.cc ──
// L17: Namespace* GetOrCreateNamespace(const std::string& name)
// L27: void PredeclarationVisitor::Predeclare(Declaration* decl)
// L48: void DeclarationVisitor::Visit(Declaration* decl)
// L61: Builtin* DeclarationVisitor::CreateBuiltin( BuiltinDeclaration* decl, std::string external_name, std::string readable_name, Signature signature, std::optional<std::string> use_counter_name, std::optional<Statement*> body)
// L162: void DeclarationVisitor::Visit(ExternalBuiltinDeclaration* decl)
// L170: void DeclarationVisitor::Visit(ExternalRuntimeDeclaration* decl)
// L210: void DeclarationVisitor::Visit(ExternalMacroDeclaration* decl)
// L221: void DeclarationVisitor::Visit(TorqueBuiltinDeclaration* decl)
// L242: void DeclarationVisitor::Visit(TorqueMacroDeclaration* decl)
// L254: void DeclarationVisitor::Visit(IntrinsicDeclaration* decl)
// L259: void DeclarationVisitor::Visit(ConstDeclaration* decl)
// L267: void DeclarationVisitor::Visit(SpecializationDeclaration* decl)
// L331: void DeclarationVisitor::Visit(ExternConstDeclaration* decl)
// L347: void DeclarationVisitor::Visit(CppIncludeDeclaration* decl)
// L351: void DeclarationVisitor::DeclareSpecializedTypes( const SpecializationKey<GenericCallable>& key)
// L371: Signature DeclarationVisitor::MakeSpecializedSignature( const SpecializationKey<GenericCallable>& key)
// L382: Callable* DeclarationVisitor::SpecializeImplicit( const SpecializationKey<GenericCallable>& key)
// L404: Callable* DeclarationVisitor::Specialize( const SpecializationKey<GenericCallable>& key, CallableDeclaration* declaration, std::optional<const SpecializationDeclaration*> explicit_specialization, std::optional<Statement*> body, SourcePosition position)
// L473: void PredeclarationVisitor::ResolvePredeclarations()
