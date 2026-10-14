// 由 tool/src/torque/declarations.js 从 v8/src/torque/declarations.h 生成; 修订就地改, 复跑覆盖
// 声明表常量真转换 + Declarations 查询/创建接口

use std::{cell::Cell, fmt::Display};

use crate::{
  ast::NodeId,
  context::{
    current_scope, report_error, with_language_server_data, with_type_oracle, with_type_oracle_mut,
  },
  declarable::{
    BuiltinData, BuiltinFlags, BuiltinKind, CallableCommon, DeclarableData, DeclarableId,
    DeclarableKind, ExternConstantData, ExternMacroData, GenericCallableData, GenericTypeData,
    IntrinsicData, MacroCommon, MethodData, NamespaceConstantData, NamespaceData, QualifiedName,
    RuntimeFunctionData, TorqueMacroData, TypeAliasData, ValueCommon, add_declarable,
    callable_common_of, identifier_value, lookup_shallow, make_declarable, resolve_type_alias,
    scope_lookup,
  },
  error::{Error, Result},
  global_context::GlobalContext,
  source_positions::SourcePosition,
  types::{Signature, TypeData, TypeId, TypeVector, VisitResult},
};

// 原版 Declarations 全静态 (CurrentScope 上下文) → Rust 关联函数, 上下文链
// (GlobalContext 持 declarations arena) 由调用方显式传参; 无 scope 参的函数走 CurrentScope 桥。
// 常量 1:1 (char* → &str)
pub const FROM_CONSTEXPR_MACRO_NAME: &str = "FromConstexpr";
pub const MACRO_END_LABEL_NAME: &str = "__macro_end";
pub const BREAK_LABEL_NAME: &str = "__break";
pub const CONTINUE_LABEL_NAME: &str = "__continue";
pub const CATCH_LABEL_NAME: &str = "__catch";
pub const NEXT_CASE_LABEL_NAME: &str = "__NextCase";

// ── 原版匿名命名空间助手 (declarations.cc:15-47) ──

/// 原版 EnsureNonempty (declarations.cc:17-24)
fn ensure_nonempty(
  list: Vec<DeclarableId>,
  name: &dyn Display,
  kind: &str,
) -> Result<Vec<DeclarableId>> {
  // v8: src/torque/declarations.cc EnsureNonempty
  if list.is_empty() {
    return Err(report_error(format!("there is no {kind} named {name}")));
  }
  Ok(list)
}

/// 原版 EnsureUnique (declarations.cc:26-35)
fn ensure_unique(list: Vec<DeclarableId>, name: &dyn Display, kind: &str) -> Result<DeclarableId> {
  // v8: src/torque/declarations.cc EnsureUnique
  if list.is_empty() {
    return Err(report_error(format!("there is no {kind} named {name}")));
  }
  if list.len() >= 2 {
    return Err(report_error(format!(
      "ambiguous reference to {kind} {name}"
    )));
  }
  Ok(list[0])
}

/// 原版 CheckAlreadyDeclared (declarations.cc:37-45)。
/// 登记偏差: 原版把 Scope* 当地址流进消息 (不可复现), 此处省略地址段;
/// const char* new_type 原版经流只打首字符 (t/c/r), Rust 打完整 kind 串系合理改良 (复审 TQR2 OBS-T2-2)
fn check_already_declared(
  cx: &GlobalContext,
  name: &str,
  new_type: &str,
  kinds: &[DeclarableKind],
) -> Result<()> {
  // v8: src/torque/declarations.cc CheckAlreadyDeclared
  let scope =
    current_scope().ok_or_else(|| Error::Message("internal: CurrentScope is null".into()))?;
  let declarations =
    Declarations::try_lookup_shallow(cx, scope, &QualifiedName::plain(name.to_owned()))?;
  let declared = declarations
    .into_iter()
    .any(|id| kinds.contains(&cx.declarable(id).kind));
  if declared {
    return Err(report_error(format!(
      "cannot redeclare {name} (type {new_type})"
    )));
  }
  Ok(())
}

/// 当前作用域 id (原版 CurrentScope::Get(), nullptr → internal 错误)
fn current_scope_id() -> Result<DeclarableId> {
  current_scope().ok_or_else(|| Error::Message("internal: CurrentScope is null".into()))
}

/// 原版 Declarations (全静态 → Rust 关联函数; 上下文链 = GlobalContext 持
/// declarations arena, 首参显式传入; 原签名逐条见 impl 内注释)
pub struct Declarations;

impl Declarations {
  /// 原版 TryLookup(name) (declarations.h:37-39): CurrentScope 深查
  pub fn try_lookup(
    cx: &GlobalContext,
    scope: DeclarableId,
    name: &QualifiedName,
  ) -> Result<Vec<DeclarableId>> {
    // v8: src/torque/declarations.h Declarations::TryLookup
    scope_lookup(cx, scope, name)
  }

  /// 原版 TryLookupShallow(name) (declarations.h:41-43): 仅本级
  pub fn try_lookup_shallow(
    cx: &GlobalContext,
    scope: DeclarableId,
    name: &QualifiedName,
  ) -> Result<Vec<DeclarableId>> {
    // v8: src/torque/declarations.h Declarations::TryLookupShallow
    lookup_shallow(cx, scope, name)
  }

  /// 原版 Lookup(name) (declarations.h:50-56): 空则报错
  pub fn lookup(
    cx: &GlobalContext,
    scope: DeclarableId,
    name: &QualifiedName,
  ) -> Result<Vec<DeclarableId>> {
    // v8: src/torque/declarations.h Declarations::Lookup
    let d = Self::try_lookup(cx, scope, name)?;
    if d.is_empty() {
      return Err(report_error(format!("cannot find \"{name}\"")));
    }
    Ok(d)
  }

  /// 原版 LookupGlobalScope(name) (declarations.cc:49-59): 默认命名空间起查
  pub fn lookup_global_scope(
    cx: &GlobalContext,
    name: &QualifiedName,
  ) -> Result<Vec<DeclarableId>> {
    // v8: src/torque/declarations.cc Declarations::LookupGlobalScope
    let default_namespace = cx
      .default_namespace()
      .ok_or_else(|| Error::Message("internal: default namespace missing".into()))?;
    let d = scope_lookup(cx, default_namespace, name)?;
    if d.is_empty() {
      return Err(report_error(format!(
        "cannot find \"{name}\" in global scope"
      )));
    }
    Ok(d)
  }

  /// 原版 LookupTypeAlias(name) (declarations.cc:61-65): CurrentScope 查 + type 唯一
  pub fn lookup_type_alias(cx: &GlobalContext, name: &QualifiedName) -> Result<DeclarableId> {
    // v8: src/torque/declarations.cc Declarations::LookupTypeAlias
    let scope = current_scope_id()?;
    let list = Self::filter_declarables(
      cx,
      &Self::lookup(cx, scope, name)?,
      DeclarableKind::TypeAlias,
    );
    ensure_unique(list, name, "type")
  }

  /// 原版 LookupType(name) (declarations.cc:67-69): 别名解析出类型
  pub fn lookup_type_name(cx: &GlobalContext, name: &QualifiedName) -> Result<TypeId> {
    // v8: src/torque/declarations.cc Declarations::LookupType(QualifiedName)
    let alias = Self::lookup_type_alias(cx, name)?;
    resolve_type_alias(cx, alias)
  }

  /// 原版 LookupType(identifier) (declarations.cc:71-78): 含 LS 定义登记
  pub fn lookup_type_identifier(cx: &GlobalContext, identifier: NodeId) -> Result<TypeId> {
    // v8: src/torque/declarations.cc Declarations::LookupType(Identifier*)
    let name = identifier_value(&cx.ast, identifier);
    let alias = Self::lookup_type_alias(cx, &QualifiedName::plain(name))?;
    if cx.collect_language_server_data {
      let token = cx.ast.node(identifier).0.pos;
      let declaration_position = match &cx.declarable(alias).data {
        DeclarableData::TypeAlias(data) => data.declaration_position,
        _ => SourcePosition::invalid(),
      };
      with_language_server_data(|data| data.add_definition(token, declaration_position));
    }
    resolve_type_alias(cx, alias)
  }

  /// 原版 TryLookupType(name) (declarations.cc:80-85): 空 → None
  pub fn try_lookup_type(cx: &GlobalContext, name: &QualifiedName) -> Result<Option<TypeId>> {
    // v8: src/torque/declarations.cc Declarations::TryLookupType
    let scope = current_scope_id()?;
    let decls = Self::filter_declarables(
      cx,
      &Self::try_lookup(cx, scope, name)?,
      DeclarableKind::TypeAlias,
    );
    if decls.is_empty() {
      return Ok(None);
    }
    let alias = ensure_unique(decls, name, "type")?;
    resolve_type_alias(cx, alias).map(Some)
  }

  /// 原版 LookupGlobalType(name) (declarations.cc:87-91)
  pub fn lookup_global_type(cx: &GlobalContext, name: &QualifiedName) -> Result<TypeId> {
    // v8: src/torque/declarations.cc Declarations::LookupGlobalType
    let list = Self::filter_declarables(
      cx,
      &Self::lookup_global_scope(cx, name)?,
      DeclarableKind::TypeAlias,
    );
    let alias = ensure_unique(list, name, "type")?;
    resolve_type_alias(cx, alias)
  }

  /// 原版 FindSomeInternalBuiltinWithType (declarations.cc:93-106)。
  /// 登记偏差: 原版返回 nullptr 表示未找到, Rust 以 Err(Message) 表达, 调用方按错误处理。
  pub fn find_some_internal_builtin_with_type(
    cx: &GlobalContext,
    type_: TypeId,
  ) -> Result<DeclarableId> {
    // v8: src/torque/declarations.cc Declarations::FindSomeInternalBuiltinWithType
    let (return_type, parameter_types) =
      with_type_oracle(|oracle| match &oracle.types[type_ as usize].data {
        TypeData::BuiltinPointer(b) => (b.return_type, b.parameter_types.clone()),
        // 原版 BuiltinPointerType::cast 契约, 非该类型时以不可命中哨兵表达
        _ => (u32::MAX, Vec::new()),
      });
    for (index, declarable) in cx.all_declarables().iter().enumerate() {
      if let DeclarableData::Builtin(builtin) = &declarable.data
        // 原版 !IsExternal() 即 body 有值
        && builtin.callable.body.is_some()
        && builtin.kind == BuiltinKind::Stub
        && builtin.callable.signature.return_type == Some(return_type)
        && builtin.callable.signature.parameter_types.types == parameter_types
      {
        return Ok(index as DeclarableId);
      }
    }
    Err(Error::Message(
      "no internal builtin found with the given type".into(),
    ))
  }

  /// 原版 LookupValue(name) (declarations.cc:108-110)
  pub fn lookup_value(cx: &GlobalContext, name: &QualifiedName) -> Result<DeclarableId> {
    // v8: src/torque/declarations.cc Declarations::LookupValue
    let scope = current_scope_id()?;
    let list = Self::lookup(cx, scope, name)?;
    let mut filtered = Self::filter_declarables(cx, &list, DeclarableKind::ExternConstant);
    filtered.extend(Self::filter_declarables(
      cx,
      &list,
      DeclarableKind::NamespaceConstant,
    ));
    ensure_unique(filtered, name, "value")
  }

  /// 原版 TryLookupMacro(name, types) (declarations.cc:112-122)
  pub fn try_lookup_macro(
    cx: &GlobalContext,
    name: &str,
    types: &TypeVector,
  ) -> Result<Option<DeclarableId>> {
    // v8: src/torque/declarations.cc Declarations::TryLookupMacro
    let scope = current_scope_id()?;
    let list = Self::try_lookup(cx, scope, &QualifiedName::plain(name.to_owned()))?;
    for id in list {
      let d = cx.declarable(id);
      if !d.is_macro() {
        continue;
      }
      let Some(callable) = callable_common_of(d) else {
        continue;
      };
      if callable.signature.get_explicit_types() == types.as_slice()
        && !callable.signature.parameter_types.var_args
      {
        return Ok(Some(id));
      }
    }
    Ok(None)
  }

  /// 原版 TryLookupBuiltin(name) (declarations.cc:124-130)
  pub fn try_lookup_builtin(
    cx: &GlobalContext,
    name: &QualifiedName,
  ) -> Result<Option<DeclarableId>> {
    // v8: src/torque/declarations.cc Declarations::TryLookupBuiltin
    let scope = current_scope_id()?;
    let builtins = Self::filter_declarables(
      cx,
      &Self::try_lookup(cx, scope, name)?,
      DeclarableKind::Builtin,
    );
    if builtins.is_empty() {
      return Ok(None);
    }
    ensure_unique(builtins, &name.name, "builtin").map(Some)
  }

  /// 原版 LookupGeneric(name) (declarations.cc:131-136)
  pub fn lookup_generic(cx: &GlobalContext, name: &str) -> Result<Vec<DeclarableId>> {
    // v8: src/torque/declarations.cc Declarations::LookupGeneric
    let scope = current_scope_id()?;
    let list = Self::filter_declarables(
      cx,
      &Self::lookup(cx, scope, &QualifiedName::plain(name.to_owned()))?,
      DeclarableKind::GenericCallable,
    );
    ensure_nonempty(list, &name, "generic callable")
  }

  /// 原版 LookupUniqueGeneric(name) (declarations.cc:138-141)
  pub fn lookup_unique_generic(cx: &GlobalContext, name: &QualifiedName) -> Result<DeclarableId> {
    // v8: src/torque/declarations.cc Declarations::LookupUniqueGeneric
    let scope = current_scope_id()?;
    let list = Self::filter_declarables(
      cx,
      &Self::lookup(cx, scope, name)?,
      DeclarableKind::GenericCallable,
    );
    ensure_unique(list, name, "generic callable")
  }

  /// 原版 LookupUniqueGenericType(name) (declarations.cc:143-146)
  pub fn lookup_unique_generic_type(
    cx: &GlobalContext,
    name: &QualifiedName,
  ) -> Result<DeclarableId> {
    // v8: src/torque/declarations.cc Declarations::LookupUniqueGenericType
    let scope = current_scope_id()?;
    let list = Self::filter_declarables(
      cx,
      &Self::lookup(cx, scope, name)?,
      DeclarableKind::GenericType,
    );
    ensure_unique(list, name, "generic type")
  }

  /// 原版 LookupGlobalUniqueGenericType(name) (declarations.cc:148-153)
  pub fn lookup_global_unique_generic_type(cx: &GlobalContext, name: &str) -> Result<DeclarableId> {
    // v8: src/torque/declarations.cc Declarations::LookupGlobalUniqueGenericType
    let list = Self::filter_declarables(
      cx,
      &Self::lookup_global_scope(cx, &QualifiedName::plain(name.to_owned()))?,
      DeclarableKind::GenericType,
    );
    ensure_unique(list, &name, "generic type")
  }

  /// 原版 TryLookupGenericType(name) (declarations.cc:155-160)
  pub fn try_lookup_generic_type(
    cx: &GlobalContext,
    name: &QualifiedName,
  ) -> Result<Option<DeclarableId>> {
    // v8: src/torque/declarations.cc Declarations::TryLookupGenericType
    let scope = current_scope_id()?;
    let results = Self::filter_declarables(
      cx,
      &Self::try_lookup(cx, scope, name)?,
      DeclarableKind::GenericType,
    );
    if results.is_empty() {
      return Ok(None);
    }
    ensure_unique(results, &name.name, "generic type").map(Some)
  }

  /// 原版 DeclareNamespace(name) (declarations.cc:162-164)
  pub fn declare_namespace(cx: &mut GlobalContext, name: &str) -> Result<DeclarableId> {
    // v8: src/torque/declarations.cc Declarations::DeclareNamespace
    let d = make_declarable(
      DeclarableKind::Namespace,
      DeclarableData::Namespace(NamespaceData {
        name: name.to_owned(),
      }),
      true,
    );
    let id = cx.register_declarable(d);
    Self::declare(cx, name, id)?;
    Ok(id)
  }

  /// 原版 DeclareType(name, type) (declarations.cc:166-170): 直接绑定已算好的类型
  pub fn declare_type(cx: &mut GlobalContext, name: NodeId, type_: TypeId) -> Result<DeclarableId> {
    // v8: src/torque/declarations.cc Declarations::DeclareType
    let value = identifier_value(&cx.ast, name);
    check_already_declared(cx, &value, "type", &[DeclarableKind::TypeAlias])?;
    let declaration_position = cx.ast.node(name).0.pos;
    let d = make_declarable(
      DeclarableKind::TypeAlias,
      DeclarableData::TypeAlias(TypeAliasData {
        being_resolved: Default::default(),
        delayed: Default::default(),
        type_: Cell::new(Some(type_)),
        redeclaration: true,
        declaration_position,
      }),
      true,
    );
    let id = cx.register_declarable(d);
    Self::declare(cx, &value, id)?;
    Ok(id)
  }

  /// 原版 PredeclareTypeAlias(name, type, redeclaration) (declarations.cc:172-179)
  pub fn predeclare_type_alias(
    cx: &mut GlobalContext,
    name: NodeId,
    decl: NodeId,
    redeclaration: bool,
  ) -> Result<DeclarableId> {
    // v8: src/torque/declarations.cc Declarations::PredeclareTypeAlias
    let value = identifier_value(&cx.ast, name);
    check_already_declared(cx, &value, "type", &[DeclarableKind::TypeAlias])?;
    let declaration_position = cx.ast.node(name).0.pos;
    let d = make_declarable(
      DeclarableKind::TypeAlias,
      DeclarableData::TypeAlias(TypeAliasData {
        being_resolved: Default::default(),
        delayed: Cell::new(Some(decl)),
        type_: Default::default(),
        redeclaration,
        declaration_position,
      }),
      true,
    );
    let id = cx.register_declarable(d);
    Self::declare(cx, &value, id)?;
    Ok(id)
  }

  /// 原版 CreateTorqueMacro (declarations.cc:181-188): external_name 唯一化; 仅登记不声明
  pub fn create_torque_macro(
    cx: &mut GlobalContext,
    external_name: &str,
    readable_name: &str,
    exported_to_csa: bool,
    signature: Signature,
    body: Option<NodeId>,
    is_user_defined: bool,
  ) -> Result<DeclarableId> {
    // v8: src/torque/declarations.cc Declarations::CreateTorqueMacro
    let external_name = cx.make_unique_name(external_name);
    check_macro_varargs(&signature)?;
    let d = make_declarable(
      DeclarableKind::TorqueMacro,
      DeclarableData::TorqueMacro(TorqueMacroData {
        macro_: MacroCommon {
          callable: CallableCommon {
            external_name,
            readable_name: readable_name.to_owned(),
            signature,
            returns: 0,
            body,
          },
          used: false,
          supports_tsa: false,
        },
        exported_to_csa,
      }),
      is_user_defined,
    );
    Ok(cx.register_declarable(d))
  }

  /// 原版 CreateExternMacro (declarations.cc:190-196): 仅登记不声明
  pub fn create_extern_macro(
    cx: &mut GlobalContext,
    name: &str,
    external_assembler_name: &str,
    signature: Signature,
  ) -> Result<DeclarableId> {
    // v8: src/torque/declarations.cc Declarations::CreateExternMacro
    check_macro_varargs(&signature)?;
    let d = make_declarable(
      DeclarableKind::ExternMacro,
      DeclarableData::ExternMacro(ExternMacroData {
        macro_: MacroCommon {
          callable: CallableCommon {
            external_name: name.to_owned(),
            readable_name: name.to_owned(),
            signature,
            returns: 0,
            body: None,
          },
          used: false,
          supports_tsa: false,
        },
        external_assembler_name: external_assembler_name.to_owned(),
      }),
      true,
    );
    Ok(cx.register_declarable(d))
  }

  /// 原版 DeclareMacro (declarations.cc:198-228)
  pub fn declare_macro(
    cx: &mut GlobalContext,
    spec: MacroSpec,
    signature: Signature,
    body: Option<NodeId>,
  ) -> Result<DeclarableId> {
    // v8: src/torque/declarations.cc Declarations::DeclareMacro
    let explicit_types = signature.get_explicit_types().to_vec();
    if let Some(existing_macro) = Self::try_lookup_macro(cx, &spec.name, &explicit_types)?
      && cx.declarable(existing_macro).parent_scope == current_scope()
    {
      return Err(report_error(format!(
        "cannot redeclare macro {} with identical explicit parameters",
        spec.name
      )));
    }
    let macro_ = match &spec.external_assembler_name {
      Some(external_assembler_name) => {
        Self::create_extern_macro(cx, &spec.name, external_assembler_name, signature.clone())?
      }
      None => Self::create_torque_macro(
        cx,
        &spec.name,
        &spec.name,
        spec.accessible_from_csa,
        signature.clone(),
        body,
        spec.is_user_defined,
      )?,
    };
    Self::declare(cx, &spec.name, macro_)?;
    if let Some(op) = &spec.op {
      if Self::try_lookup_macro(cx, op, &explicit_types)?.is_some() {
        return Err(report_error(format!(
          "cannot redeclare operator {} with identical explicit parameters",
          spec.name
        )));
      }
      Self::declare_operator(cx, op, macro_)?;
    }
    Ok(macro_)
  }

  /// 原版 CreateMethod (declarations.cc:230-239): 唯一化名 + 聚合类型登记
  pub fn create_method(
    cx: &mut GlobalContext,
    class_type: TypeId,
    name: &str,
    signature: Signature,
    body: NodeId,
  ) -> Result<DeclarableId> {
    // v8: src/torque/declarations.cc Declarations::CreateMethod
    check_macro_varargs(&signature)?;
    let simple_name =
      with_type_oracle(|oracle| oracle.types[class_type as usize].simple_name(cx, oracle));
    let generated_name = cx.make_unique_name(&format!("Method_{simple_name}_{name}"));
    let d = make_declarable(
      DeclarableKind::Method,
      DeclarableData::Method(MethodData {
        macro_: MacroCommon {
          callable: CallableCommon {
            external_name: generated_name,
            readable_name: name.to_owned(),
            signature,
            returns: 0,
            body: Some(body),
          },
          used: false,
          supports_tsa: false,
        },
        aggregate_type: Some(class_type),
      }),
      true,
    );
    let id = cx.register_declarable(d);
    with_type_oracle_mut(|oracle| {
      match &mut oracle.types[class_type as usize].data {
        // 原版 AggregateType::RegisterMethod
        TypeData::Class(c) => c.aggregate.methods.push(id),
        TypeData::Struct(s) => s.aggregate.methods.push(id),
        _ => {}
      }
    });
    Ok(id)
  }

  /// 原版 CreateIntrinsic (declarations.cc:241-246): 仅登记不声明
  pub fn create_intrinsic(
    cx: &mut GlobalContext,
    name: &str,
    signature: Signature,
  ) -> Result<DeclarableId> {
    // v8: src/torque/declarations.cc Declarations::CreateIntrinsic
    check_intrinsic_varargs(&signature)?;
    let d = make_declarable(
      DeclarableKind::Intrinsic,
      DeclarableData::Intrinsic(IntrinsicData {
        callable: CallableCommon {
          external_name: name.to_owned(),
          readable_name: name.to_owned(),
          signature,
          returns: 0,
          body: None,
        },
      }),
      true,
    );
    Ok(cx.register_declarable(d))
  }

  /// 原版 DeclareIntrinsic (declarations.cc:248-253)
  pub fn declare_intrinsic(
    cx: &mut GlobalContext,
    name: &str,
    signature: Signature,
  ) -> Result<DeclarableId> {
    // v8: src/torque/declarations.cc Declarations::DeclareIntrinsic
    let result = Self::create_intrinsic(cx, name, signature)?;
    Self::declare(cx, name, result)?;
    Ok(result)
  }

  /// 原版 CreateBuiltin (declarations.cc:255-263): 仅登记不声明
  pub fn create_builtin(
    cx: &mut GlobalContext,
    spec: BuiltinSpec,
    signature: Signature,
    body: Option<NodeId>,
  ) -> Result<DeclarableId> {
    // v8: src/torque/declarations.cc Declarations::CreateBuiltin
    let d = make_declarable(
      DeclarableKind::Builtin,
      DeclarableData::Builtin(BuiltinData {
        callable: CallableCommon {
          external_name: spec.external_name,
          readable_name: spec.readable_name,
          signature,
          returns: 0,
          body,
        },
        kind: spec.kind,
        flags: spec.flags,
        use_counter_name: spec.use_counter_name,
      }),
      true,
    );
    Ok(cx.register_declarable(d))
  }

  /// 原版 DeclareRuntimeFunction (declarations.cc:265-270)
  pub fn declare_runtime_function(
    cx: &mut GlobalContext,
    name: &str,
    signature: Signature,
  ) -> Result<DeclarableId> {
    // v8: src/torque/declarations.cc Declarations::DeclareRuntimeFunction
    check_already_declared(
      cx,
      name,
      "runtime function",
      &[DeclarableKind::RuntimeFunction],
    )?;
    let d = make_declarable(
      DeclarableKind::RuntimeFunction,
      DeclarableData::RuntimeFunction(RuntimeFunctionData {
        callable: CallableCommon {
          external_name: name.to_owned(),
          readable_name: name.to_owned(),
          signature,
          returns: 0,
          body: None,
        },
      }),
      true,
    );
    let id = cx.register_declarable(d);
    Self::declare(cx, name, id)?;
    Ok(id)
  }

  /// 原版 DeclareExternConstant (declarations.cc:272-278): 值直接内联 constexpr
  pub fn declare_extern_constant(
    cx: &mut GlobalContext,
    name: NodeId,
    type_: TypeId,
    value: String,
  ) -> Result<DeclarableId> {
    // v8: src/torque/declarations.cc Declarations::DeclareExternConstant
    let value_name = identifier_value(&cx.ast, name);
    check_already_declared(
      cx,
      &value_name,
      "constant",
      &[
        DeclarableKind::ExternConstant,
        DeclarableKind::NamespaceConstant,
      ],
    )?;
    let d = make_declarable(
      DeclarableKind::ExternConstant,
      DeclarableData::ExternConstant(ExternConstantData {
        value: ValueCommon {
          type_: Some(type_),
          name,
          value: Some(VisitResult {
            ty: Some(type_),
            constexpr_value: Some(value),
            stack_range: None,
          }),
        },
      }),
      true,
    );
    let id = cx.register_declarable(d);
    Self::declare(cx, &value_name, id)?;
    Ok(id)
  }

  /// 原版 DeclareNamespaceConstant (declarations.cc:280-290): external_name 唯一化
  pub fn declare_namespace_constant(
    cx: &mut GlobalContext,
    name: NodeId,
    type_: TypeId,
    body: NodeId,
  ) -> Result<DeclarableId> {
    // v8: src/torque/declarations.cc Declarations::DeclareNamespaceConstant
    let value_name = identifier_value(&cx.ast, name);
    check_already_declared(
      cx,
      &value_name,
      "constant",
      &[
        DeclarableKind::ExternConstant,
        DeclarableKind::NamespaceConstant,
      ],
    )?;
    let external_name = cx.make_unique_name(&value_name);
    let d = make_declarable(
      DeclarableKind::NamespaceConstant,
      DeclarableData::NamespaceConstant(NamespaceConstantData {
        value: ValueCommon {
          type_: Some(type_),
          name,
          value: None,
        },
        external_name,
        body,
      }),
      true,
    );
    let id = cx.register_declarable(d);
    Self::declare(cx, &value_name, id)?;
    Ok(id)
  }

  /// 原版 DeclareGenericCallable (declarations.cc:292-296)
  pub fn declare_generic_callable(
    cx: &mut GlobalContext,
    name: &str,
    ast_node: NodeId,
  ) -> Result<DeclarableId> {
    // v8: src/torque/declarations.cc Declarations::DeclareGenericCallable
    let d = make_declarable(
      DeclarableKind::GenericCallable,
      DeclarableData::GenericCallable(GenericCallableData {
        name: name.to_owned(),
        declaration: ast_node,
        specializations: Default::default(),
        constraints: None,
      }),
      true,
    );
    let id = cx.register_declarable(d);
    Self::declare(cx, name, id)?;
    Ok(id)
  }

  /// 原版 DeclareGenericType (declarations.cc:298-302)
  pub fn declare_generic_type(
    cx: &mut GlobalContext,
    name: &str,
    ast_node: NodeId,
  ) -> Result<DeclarableId> {
    // v8: src/torque/declarations.cc Declarations::DeclareGenericType
    let d = make_declarable(
      DeclarableKind::GenericType,
      DeclarableData::GenericType(GenericTypeData {
        name: name.to_owned(),
        declaration: ast_node,
        specializations: Default::default(),
        constraints: None,
      }),
      true,
    );
    let id = cx.register_declarable(d);
    Self::declare(cx, name, id)?;
    Ok(id)
  }

  /// 原版 DeclareOperator(name, m) (declarations.cc:313-316): 挂到默认命名空间
  pub fn declare_operator(
    cx: &mut GlobalContext,
    name: &str,
    m: DeclarableId,
  ) -> Result<DeclarableId> {
    // v8: src/torque/declarations.cc Declarations::DeclareOperator
    let default_namespace = cx
      .default_namespace()
      .ok_or_else(|| Error::Message("internal: default namespace missing".into()))?;
    add_declarable(cx, default_namespace, name, m)?;
    Ok(m)
  }

  /// 原版 GetGeneratedCallableName (declarations.cc:304-311): 逐特化类型 SimpleName 下划线拼接
  pub fn get_generated_callable_name(
    cx: &GlobalContext,
    name: &str,
    specialized_types: &TypeVector,
  ) -> Result<String> {
    // v8: src/torque/declarations.cc Declarations::GetGeneratedCallableName
    Ok(with_type_oracle(|oracle| {
      let mut result = name.to_owned();
      for type_ in specialized_types {
        result.push('_');
        result.push_str(&oracle.types[*type_ as usize].simple_name(cx, oracle));
      }
      result
    }))
  }

  /// 原版 Declare<T>(name, d) (declarations.h:136-145): 登记后挂当前作用域名字表。
  /// 填体新增 (原骨架未列出), 语义为头文件内联版直译。
  pub fn declare(
    cx: &mut GlobalContext,
    name: &str,
    declarable: DeclarableId,
  ) -> Result<DeclarableId> {
    // v8: src/torque/declarations.h Declarations::Declare
    let scope =
      current_scope().ok_or_else(|| Error::Message("internal: CurrentScope is null".into()))?;
    add_declarable(cx, scope, name, declarable)?;
    Ok(declarable)
  }

  /// 原版 FilterDeclarables<T> (模板) → 按种类过滤
  #[must_use]
  pub fn filter_declarables(
    cx: &GlobalContext,
    list: &[DeclarableId],
    kind: DeclarableKind,
  ) -> Vec<DeclarableId> {
    list
      .iter()
      .copied()
      .filter(|id| cx.declarable(*id).kind == kind)
      .collect()
  }
}

/// 原版 Macro 构造 varargs 检查 (declarable.h:398-401)
fn check_macro_varargs(signature: &Signature) -> Result<()> {
  // v8: src/torque/declarable.h Macro::Macro
  if signature.parameter_types.var_args {
    return Err(report_error(
      "Varargs are not supported for macros.".to_owned(),
    ));
  }
  Ok(())
}

/// 原版 Intrinsic 构造 varargs 检查 (declarable.h:560-562)
fn check_intrinsic_varargs(signature: &Signature) -> Result<()> {
  // v8: src/torque/declarable.h Intrinsic::Intrinsic
  if signature.parameter_types.var_args {
    return Err(report_error(
      "Varargs are not supported for intrinsics.".to_owned(),
    ));
  }
  Ok(())
}

/// declare_macro 参数束 (原版 7 参 → 束; too_many_arguments 治理)
#[derive(Debug, Clone, Default)]
pub struct MacroSpec {
  pub name: String,
  pub accessible_from_csa: bool,
  pub external_assembler_name: Option<String>,
  pub op: Option<String>,
  pub is_user_defined: bool,
}

/// create_builtin 参数束 (原版 5 参 → 束)
#[derive(Debug, Clone, Default)]
pub struct BuiltinSpec {
  pub external_name: String,
  pub readable_name: String,
  pub kind: BuiltinKind,
  pub flags: BuiltinFlags,
  pub use_counter_name: Option<String>,
}

// 原版 TypeConstraint 相关: 见 declarable.rs (compute_constraints/find_constraint_violation)
// 原版 SpecializationKey<GenericCallable> → crate::types::SpecializationKey (DeclarableId 泛化)
// 原版 FilterDeclarables 泛型按 DeclarableKind 过滤, 泛型 T 退化由调用方 kind 判别承担。

// ── 原版 src/torque/declarations.cc ──
// L49: std::vector<Declarable*> Declarations::LookupGlobalScope( const QualifiedName& name)
// L61: const TypeAlias* Declarations::LookupTypeAlias(const QualifiedName& name)
// L67: const Type* Declarations::LookupType(const QualifiedName& name)
// L71: const Type* Declarations::LookupType(const Identifier* name)
// L80: std::optional<const Type*> Declarations::TryLookupType( const QualifiedName& name)
// L87: const Type* Declarations::LookupGlobalType(const QualifiedName& name)
// L93: Builtin* Declarations::FindSomeInternalBuiltinWithType( const BuiltinPointerType* type)
// L108: Value* Declarations::LookupValue(const QualifiedName& name)
// L112: Macro* Declarations::TryLookupMacro(const std::string& name, const TypeVector& types)
// L124: std::optional<Builtin*> Declarations::TryLookupBuiltin( const QualifiedName& name)
// L131: std::vector<GenericCallable*> Declarations::LookupGeneric( const std::string& name)
// L138: GenericCallable* Declarations::LookupUniqueGeneric(const QualifiedName& name)
// L143: GenericType* Declarations::LookupUniqueGenericType(const QualifiedName& name)
// L148: GenericType* Declarations::LookupGlobalUniqueGenericType( const std::string& name)
// L155: std::optional<GenericType*> Declarations::TryLookupGenericType( const QualifiedName& name)
// L162: Namespace* Declarations::DeclareNamespace(const std::string& name)
// L166: TypeAlias* Declarations::DeclareType(const Identifier* name, const Type* type)
// L172: TypeAlias* Declarations::PredeclareTypeAlias(const Identifier* name, TypeDeclaration* type, bool redeclaration)
// L181: TorqueMacro* Declarations::CreateTorqueMacro( std::string external_name, std::string readable_name, bool exported_to_csa, Signature signature, std::optional<Statement*> body, bool is_user_defined)
// L190: ExternMacro* Declarations::CreateExternMacro( std::string name, std::string external_assembler_name, Signature signature)
// L198: Macro* Declarations::DeclareMacro( const std::string& name, bool accessible_from_csa, std::optional<std::string> external_assembler_name, const Signature& signature, std::optional<Statement*> body, std::optional<std::string> op, bool is_user_defined)
// L230: Method* Declarations::CreateMethod(AggregateType* class_type, const std::string& name, Signature signature, Statement* body)
// L241: Intrinsic* Declarations::CreateIntrinsic(const std::string& name, const Signature& signature)
// L248: Intrinsic* Declarations::DeclareIntrinsic(const std::string& name, const Signature& signature)
// L255: Builtin* Declarations::CreateBuiltin( std::string external_name, std::string readable_name, Builtin::Kind kind, Builtin::Flags flags, Signature signature, std::optional<std::string> use_counter_name, std::optional<Statement*> body)
// L265: RuntimeFunction* Declarations::DeclareRuntimeFunction( const std::string& name, const Signature& signature)
// L272: ExternConstant* Declarations::DeclareExternConstant(Identifier* name, const Type* type, std::string value)
// L280: NamespaceConstant* Declarations::DeclareNamespaceConstant(Identifier* name, const Type* type, Expression* body)
// L292: GenericCallable* Declarations::DeclareGenericCallable( const std::string& name, GenericCallableDeclaration* ast_node)
// L298: GenericType* Declarations::DeclareGenericType( const std::string& name, GenericTypeDeclaration* ast_node)
// L304: std::string Declarations::GetGeneratedCallableName( const std::string& name, const TypeVector& specialized_types)
// L313: Macro* Declarations::DeclareOperator(const std::string& name, Macro* m)
