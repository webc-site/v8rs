//! 原版 v8/src/torque/types.h|types.cc 的 Type 体系语义校验
//! (上游 v8/test/unittests/torque 无 types 专属用例, 断言规约直接取自源码行号)
//!
//! 断言规约清单 (先规约后断言, 强度守恒):
//! - types_is_subtype_of_basic: sup 为 Top 恒真 / sub 为 never 恒真 / 否则沿父链上溯
//!   (types.cc:104-116)
//! - types_is_subtype_of_union_directions: 并集作子型须全成员成立 (types.h:420-425),
//!   并集作父型须任一成员成立 (types.h:427-434)
//! - types_to_display_string_alias_forms: 空别名走 ComputeName(ToExplicitString);
//!   单别名直出; 多别名 "a0 (aka. a1, a2)" (types.cc:38-57)
//! - types_simple_name_specialization_suffix: 无别名时 SimpleNameImpl 逐项追加 "_X"
//!   (types.cc:59-73)
//! - types_compute_name_specialized_forms: None → basename; 非引用泛型 → "Base<a, b>"
//!   (types.cc:550-571; const/mutable 引用泛型分支须 Declarations 泛型查找, 未填体)
//! - types_to_explicit_string_per_kind: Top "inaccessible X" (types.h:247-250);
//!   Abstract 名 (types.h:272); BuiltinPointer "builtin (a) => r" (types.cc:241-247);
//!   Union "(a | b)" (types.cc:259-268); Class "class X" (types.cc:635)
//! - types_aggregate_decl_names: "bitfield struct <decl->name->value>" (types.h:530 +
//!   types.cc:420); "struct <decl->name->value>" (types.cc:573/602, 与构造期 name_ 同源
//!   types.cc:507)
//! - types_generated_type_names: Abstract 非 constexpr → "TNode<X>"、空 generated → 父链
//!   (types.cc:183-192/201-221); Class → "TNode<generates>" (types.cc:626-633);
//!   Struct 直出 generated_type_name (types.cc:520), 空串 → ReportError 走 Err 通道
//!   (types.cc:183-190); Struct 的 TNode 名原版 UNREACHABLE (types.h:555) → Err
//! - types_constexpr_version_linkage: GetAbstractType 回填 constexpr_version_
//!   (type-oracle.h:30-33); NonConstexprVersion (types.h:281-287);
//!   constexpr SimpleName 前缀 (types.h:305-313); constexpr 的 TNode 名报错
//!   (types.cc:192-199)
//! - types_is_transient: Abstract kTransient / Union 任一成员 / Class kTransient
//!   (types.h:140/437-443/689)
//! - types_combine_object_slot_kind: 同值原样、强+可空 → 可空、其余 None (types.h:666-674)
//! - types_signature_implicit_split: ExplicitCount / GetImplicitTypes / GetExplicitTypes
//!   (types.h:891-910)

use std::collections::BTreeSet;

use aok::{OK, Void};
use log::info;
use v8_torque::{
  AbstractTypeFlag, Ast, AstNode, AstNodeKind, BitFieldStructDeclaration, ClassFlag, DeclarableId,
  Error, GlobalContext, Identifier, NodeHeader, NodeId, ObjectSlotKind, ParameterTypes, Signature,
  SpecializationKey, StructDeclaration, Type, TypeData, TypeId, TypeKind, TypeOracle, UnionType,
  combine_object_slot_kind, compute_name, is_subtype_of,
};

#[ctor::ctor(unsafe)]
fn _log_init() {
  log_init::init();
}

/// 原版节点头最小构造 (pos/kind 在被测路径不参与判定)
fn header(kind: AstNodeKind) -> NodeHeader {
  NodeHeader {
    kind,
    pos: Default::default(),
  }
}

/// 原版 Identifier 节点入 arena, 返回 NodeId (types.h:Identifier::value)
fn add_identifier(ast: &mut Ast, value: &str) -> NodeId {
  ast.add_node(
    header(AstNodeKind::Identifier),
    AstNode::Identifier(Box::new(Identifier {
      value: value.to_owned(),
    })),
  )
}

/// 原版 StructDeclaration 节点入 arena (fields/methods 在被测路径不参与判定)
fn add_struct_declaration(ast: &mut Ast, name: &str) -> NodeId {
  let name_id = add_identifier(ast, name);
  ast.add_node(
    header(AstNodeKind::StructDeclaration),
    AstNode::StructDeclaration(Box::new(StructDeclaration {
      name: name_id,
      flags: 0,
      methods: Vec::new(),
      fields: Vec::new(),
    })),
  )
}

/// 原版 TypeOracle::GetAbstractType (parent/name/flags/generated)
fn add_abstract(
  oracle: &mut TypeOracle,
  parent: Option<TypeId>,
  name: &str,
  flags: u32,
  generated: &str,
) -> TypeId {
  oracle.get_abstract_type(
    parent,
    name.to_owned(),
    flags,
    generated.to_owned(),
    None,
    None,
  )
}

/// 原版 TypeOracle::GetUnionType 的 interning 尚未填体 (type_oracle.rs:206-213),
/// 测试按同构语义直填 arena: TypeId = 下标, 成员集 = BTreeSet (原 std::set 同为确定序)
fn add_union(oracle: &mut TypeOracle, members: &[TypeId], parent: Option<TypeId>) -> TypeId {
  let id = oracle.types.len();
  oracle.types.push(Type {
    kind: TypeKind::UnionType,
    parent,
    id,
    specialized_from: None,
    aliases: BTreeSet::default(),
    data: TypeData::Union(UnionType {
      types: members.iter().copied().collect(),
    }),
  });
  id as TypeId
}

/// 原版 Type::AddAlias (types.h:150, const 方法 + mutable aliases_)
fn add_alias(oracle: &mut TypeOracle, id: TypeId, alias: &str) {
  oracle.types[id as usize].add_alias(alias.to_owned());
}

fn empty_cx() -> GlobalContext {
  GlobalContext::new(Ast::new())
}

/// 原版 Type::IsSubtypeOf 基类三分支 + 父链上溯
#[test]
fn types_is_subtype_of_basic() -> Void {
  info!("> types_is_subtype_of_basic");
  let mut oracle = TypeOracle::default();
  let object = add_abstract(&mut oracle, None, "Object", 0, "Object");
  let js_object = add_abstract(&mut oracle, Some(object), "JSObject", 0, "JSObject");
  let never = add_abstract(&mut oracle, None, "never", 0, "");
  let top = oracle.get_top_type("test only".to_owned(), js_object);

  // 自身命中 (原 while 首轮 subtype == supertype)
  assert!(is_subtype_of(&oracle, object, object));
  assert!(is_subtype_of(&oracle, js_object, js_object));
  // 父链上溯
  assert!(is_subtype_of(&oracle, js_object, object));
  assert!(!is_subtype_of(&oracle, object, js_object));
  // sup 为 Top 恒真 (types.cc:105)
  assert!(is_subtype_of(&oracle, object, top));
  // sub 为 never 恒真 (types.cc:106)
  assert!(is_subtype_of(&oracle, never, js_object));
  assert!(!is_subtype_of(&oracle, js_object, never));
  // IsAbstractName / IsNever 仅比对自身抽象名, 不沿父链 (types.cc:178-181)
  assert!(oracle.types[never as usize].is_never());
  assert!(oracle.types[js_object as usize].is_abstract_name("JSObject"));
  assert!(!oracle.types[js_object as usize].is_never());
  OK
}

/// 并集的双向语义 (types.h:420-434)
#[test]
fn types_is_subtype_of_union_directions() -> Void {
  info!("> types_is_subtype_of_union_directions");
  let mut oracle = TypeOracle::default();
  let object = add_abstract(&mut oracle, None, "Object", 0, "Object");
  let js_object = add_abstract(&mut oracle, Some(object), "JSObject", 0, "JSObject");
  let number = add_abstract(&mut oracle, None, "Number", 0, "Number");
  let string = add_abstract(&mut oracle, None, "String", 0, "String");
  let sup_union = add_union(&mut oracle, &[object, number], None);
  let sub_union = add_union(&mut oracle, &[js_object, object], None);
  let mixed_union = add_union(&mut oracle, &[js_object, number], None);

  // 并集作 sup: 任一成员即可 (UnionType::IsSupertypeOf)
  assert!(is_subtype_of(&oracle, js_object, sup_union));
  assert!(is_subtype_of(&oracle, number, sup_union));
  assert!(!is_subtype_of(&oracle, string, sup_union));
  // 并集作 sub: 全成员皆可 (UnionType::IsSubtypeOf)
  assert!(is_subtype_of(&oracle, sub_union, object));
  assert!(!is_subtype_of(&oracle, mixed_union, object));
  // 并集 → Top: 逐成员命中 Top 恒真
  let top = oracle.get_top_type("test only".to_owned(), string);
  assert!(is_subtype_of(&oracle, mixed_union, top));
  // 并集 → 并集: 子并集的每个成员各须命中 sup 并集的任一成员 (types.h:420 + L427)
  assert!(is_subtype_of(&oracle, sub_union, sup_union));
  assert!(is_subtype_of(&oracle, mixed_union, sup_union));
  let bad_union = add_union(&mut oracle, &[js_object, string], None);
  assert!(!is_subtype_of(&oracle, bad_union, sup_union));
  OK
}

/// 原版 Type::ToString 的别名三形态 (types.cc:38-57)
#[test]
fn types_to_display_string_alias_forms() -> Void {
  info!("> types_to_display_string_alias_forms");
  let cx = empty_cx();
  let mut oracle = TypeOracle::default();
  let foo = add_abstract(&mut oracle, None, "Foo", 0, "Foo");
  // 无别名: ToExplicitString + ComputeName(specialized_from=None → 原样)
  assert_eq!(
    oracle.types[foo as usize].to_display_string(&cx, &oracle),
    "Foo"
  );
  // 单别名直出 (types.cc:42)
  add_alias(&mut oracle, foo, "Bar");
  assert_eq!(
    oracle.types[foo as usize].to_display_string(&cx, &oracle),
    "Bar"
  );
  // 双别名 "a0 (aka. a1)" (types.cc:44-51, 集合序即字典序)
  add_alias(&mut oracle, foo, "Baz");
  assert_eq!(
    oracle.types[foo as usize].to_display_string(&cx, &oracle),
    "Bar (aka. Baz)"
  );
  // 三别名 "a0 (aka. a1, a2)"
  add_alias(&mut oracle, foo, "Qux");
  assert_eq!(
    oracle.types[foo as usize].to_display_string(&cx, &oracle),
    "Bar (aka. Baz, Qux)"
  );
  // SimpleName 取首别名 (types.cc:72)
  assert_eq!(oracle.types[foo as usize].simple_name(&cx, &oracle), "Bar");
  OK
}

/// 原版 Type::SimpleName 特化后缀 (types.cc:59-73)
#[test]
fn types_simple_name_specialization_suffix() -> Void {
  info!("> types_simple_name_specialization_suffix");
  let cx = empty_cx();
  let mut oracle = TypeOracle::default();
  let i32 = add_abstract(&mut oracle, None, "int32", 0, "int32");
  let generic: DeclarableId = 0;
  let key = Some(SpecializationKey {
    generic,
    specialized_types: vec![i32],
  });
  let container =
    oracle.get_abstract_type(None, "Container".to_owned(), 0, String::new(), None, key);
  // SimpleNameImpl + "_" + 每个特化实参 SimpleName
  assert_eq!(
    oracle.types[container as usize].simple_name(&cx, &oracle),
    "Container_int32"
  );
  // ToString → ComputeName("Container", key) (types.cc:40 + L563-570)
  assert_eq!(
    oracle.types[container as usize].to_display_string(&cx, &oracle),
    "Container<int32>"
  );
  OK
}

/// 原版 Type::ComputeName 静态入口 (types.cc:550-571)
#[test]
fn types_compute_name_specialized_forms() -> Void {
  info!("> types_compute_name_specialized_forms");
  let cx = empty_cx();
  let mut oracle = TypeOracle::default();
  let i32 = add_abstract(&mut oracle, None, "int32", 0, "int32");
  let f64 = add_abstract(&mut oracle, None, "float64", 0, "float64");
  let generic: DeclarableId = 0;
  assert_eq!(compute_name(&cx, &oracle, "Base", &None), "Base");
  let one = Some(SpecializationKey {
    generic,
    specialized_types: vec![i32],
  });
  assert_eq!(compute_name(&cx, &oracle, "Base", &one), "Base<int32>");
  let two = Some(SpecializationKey {
    generic,
    specialized_types: vec![i32, f64],
  });
  assert_eq!(
    compute_name(&cx, &oracle, "Base", &two),
    "Base<int32, float64>"
  );
  OK
}

/// 各 kind 的 ToExplicitString / SimpleNameImpl 形态 (types.h:247-272 +
/// types.cc:241/259/274/635)
#[test]
fn types_to_explicit_string_per_kind() -> Void {
  info!("> types_to_explicit_string_per_kind");
  let cx = empty_cx();
  let mut oracle = TypeOracle::default();
  let alpha = add_abstract(&mut oracle, None, "Alpha", 0, "Alpha");
  let beta = add_abstract(&mut oracle, None, "Beta", 0, "Beta");
  let void_ = add_abstract(&mut oracle, None, "void", 0, "void");
  assert_eq!(
    oracle.types[alpha as usize].to_display_string(&cx, &oracle),
    "Alpha"
  );

  // Union: "(a | b)", 成员按 arena 下标序 (原 std::set<Type*, TypeLess>)
  let union = add_union(&mut oracle, &[alpha, beta], None);
  assert_eq!(
    oracle.types[union as usize].to_display_string(&cx, &oracle),
    "(Alpha | Beta)"
  );
  // Union SimpleName: "A_OR_B" (types.cc:274-284)
  assert_eq!(
    oracle.types[union as usize].simple_name(&cx, &oracle),
    "Alpha_OR_Beta"
  );

  // BuiltinPointer: "builtin (params) => ret" + SimpleName 下划线链 (types.cc:241-260)
  let fn_ptr = oracle.get_builtin_pointer_type(vec![alpha, beta], void_);
  assert_eq!(
    oracle.types[fn_ptr as usize].to_display_string(&cx, &oracle),
    "builtin (Alpha, Beta) => void"
  );
  assert_eq!(
    oracle.types[fn_ptr as usize].simple_name(&cx, &oracle),
    "BuiltinPointer_Alpha_Beta_void"
  );

  // Class: "class X" (types.cc:635) / SimpleName = aggregate name (types.h:645)
  let decl: NodeId = 0;
  let class = oracle.get_class_type(
    None,
    "Klass".to_owned(),
    0,
    "JSKlass".to_owned(),
    decl,
    None,
  );
  assert_eq!(
    oracle.types[class as usize].to_display_string(&cx, &oracle),
    "class Klass"
  );
  assert_eq!(
    oracle.types[class as usize].simple_name(&cx, &oracle),
    "Klass"
  );

  // Top: "inaccessible <source ToString>" (types.h:247-250) / SimpleName "TopType"
  let top = oracle.get_top_type("bad cast".to_owned(), alpha);
  assert_eq!(
    oracle.types[top as usize].to_display_string(&cx, &oracle),
    "inaccessible Alpha"
  );
  assert_eq!(
    oracle.types[top as usize].simple_name(&cx, &oracle),
    "TopType"
  );
  OK
}

/// BitFieldStructType::name / StructType::decl_name 走 Ast 的 decl->name->value
/// (types.h:530 + types.cc:573/507/602/420)
#[test]
fn types_aggregate_decl_names() -> Void {
  info!("> types_aggregate_decl_names");
  let mut ast = Ast::new();
  let bf_name = add_identifier(&mut ast, "kTestBitFields");
  let bf_decl = ast.add_node(
    header(AstNodeKind::BitFieldStructDeclaration),
    AstNode::BitFieldStructDeclaration(Box::new(BitFieldStructDeclaration {
      name: bf_name,
      parent: 0,
      fields: Vec::new(),
      cpp_scope: None,
    })),
  );
  let st_decl = add_struct_declaration(&mut ast, "TestStruct");
  let cx = GlobalContext::new(ast);

  let mut oracle = TypeOracle::default();
  let bf = oracle.get_bit_field_struct_type(None, bf_decl);
  let struct_ty = oracle.get_struct_type(st_decl, None);

  // 载荷级取名 (修复点: BitFieldStructType::name / StructType::decl_name)
  let TypeData::BitFieldStruct(payload) = &oracle.types[bf as usize].data else {
    panic!("测试构造保证 BitFieldStruct 载荷");
  };
  assert_eq!(payload.name(&cx), "kTestBitFields");
  let TypeData::Struct(payload) = &oracle.types[struct_ty as usize].data else {
    panic!("测试构造保证 Struct 载荷");
  };
  assert_eq!(payload.decl_name(&cx), "TestStruct");

  assert_eq!(
    oracle.types[bf as usize].to_display_string(&cx, &oracle),
    "bitfield struct kTestBitFields"
  );
  assert_eq!(
    oracle.types[bf as usize].simple_name(&cx, &oracle),
    "kTestBitFields"
  );
  assert_eq!(
    oracle.types[struct_ty as usize].to_display_string(&cx, &oracle),
    "struct TestStruct"
  );
  assert_eq!(
    oracle.types[struct_ty as usize].simple_name(&cx, &oracle),
    "TestStruct"
  );
  OK
}

/// GetGeneratedTypeName / GetGeneratedTNodeTypeName 主链路 (types.cc:183-230/520/626-633)
#[test]
fn types_generated_type_names() -> Void {
  info!("> types_generated_type_names");
  let mut ast = Ast::new();
  let st_decl = add_struct_declaration(&mut ast, "Bare");
  let cx = GlobalContext::new(ast);
  let mut oracle = TypeOracle::default();
  let number = add_abstract(&mut oracle, None, "Number", 0, "Number");
  let child = add_abstract(&mut oracle, Some(number), "Numeric", 0, "");
  // Abstract 非 constexpr → TNode<generated> (types.cc:215)
  assert_eq!(
    oracle.types[number as usize].get_generated_type_name(&cx, &oracle)?,
    "TNode<Number>"
  );
  // 空 generated → 走父链 (types.cc:211-213)
  assert_eq!(
    oracle.types[child as usize].get_generated_type_name(&cx, &oracle)?,
    "TNode<Number>"
  );
  // TNode 名即 generated 本体 (types.cc:218-221)
  assert_eq!(
    oracle.types[number as usize].get_generated_tnode_type_name(&cx, &oracle)?,
    "Number"
  );

  // Class → TNode<generates> / generates 本体 (types.cc:626-633)
  let decl: NodeId = 0;
  let class = oracle.get_class_type(
    None,
    "Klass".to_owned(),
    0,
    "JSKlass".to_owned(),
    decl,
    None,
  );
  assert_eq!(
    oracle.types[class as usize].get_generated_type_name(&cx, &oracle)?,
    "TNode<JSKlass>"
  );
  assert_eq!(
    oracle.types[class as usize].get_generated_tnode_type_name(&cx, &oracle)?,
    "JSKlass"
  );

  // Struct: generated_type_name 空 → 原版 ReportError → Err 通道 (types.cc:183-190)
  let struct_ty = oracle.get_struct_type(st_decl, None);
  let err = oracle.types[struct_ty as usize]
    .get_generated_type_name(&cx, &oracle)
    .expect_err("空 generated_type_name 原版必 ReportError");
  match err {
    Error::Message(msg) => {
      assert!(msg.starts_with("Generated type is required for type 'struct Bare'"));
      assert!(msg.contains("Use 'generates' clause in definition."));
    }
    other => panic!("测试构造保证 Message 错误通道: {other:?}"),
  }
  // 回填 generated 后直出本体, 不套 TNode (types.cc:520)
  if let TypeData::Struct(payload) = &mut oracle.types[struct_ty as usize].data {
    payload.generated_type_name = "BareLayout".to_owned();
  }
  assert_eq!(
    oracle.types[struct_ty as usize].get_generated_type_name(&cx, &oracle)?,
    "BareLayout"
  );
  // Struct 的 TNode 名原版落 AggregateType 层 UNREACHABLE (types.h:555) → Err
  assert!(
    oracle.types[struct_ty as usize]
      .get_generated_tnode_type_name(&cx, &oracle)
      .is_err()
  );
  OK
}

/// constexpr 双向链接 + NonConstexprVersion + constexpr SimpleName (types.h:275-313)
#[test]
fn types_constexpr_version_linkage() -> Void {
  info!("> types_constexpr_version_linkage");
  let cx = empty_cx();
  let mut oracle = TypeOracle::default();
  let nc = add_abstract(&mut oracle, None, "int32", 0, "int32");
  let c = oracle.get_abstract_type(
    Some(nc),
    "constexpr int32".to_owned(),
    AbstractTypeFlag::Constexpr as u32,
    "int32".to_owned(),
    Some(nc),
    None,
  );
  // IsConstexpr 由 flags 位判别 (types.h:275-279)
  assert!(oracle.types[c as usize].is_constexpr(&oracle));
  assert!(!oracle.types[nc as usize].is_constexpr(&oracle));
  // GetAbstractType 回填非 constexpr 版本的 constexpr_version_ (type-oracle.h:30-33)
  let TypeData::Abstract(payload) = &oracle.types[nc as usize].data else {
    panic!("测试构造保证 Abstract 载荷");
  };
  assert_eq!(payload.constexpr_version, Some(c));
  // NonConstexprVersion: 载荷优先, 非 constexpr 取自身 (types.h:281-287)
  assert_eq!(
    oracle.types[c as usize].non_constexpr_version(&oracle),
    Some(nc)
  );
  assert_eq!(
    oracle.types[nc as usize].non_constexpr_version(&oracle),
    Some(nc)
  );
  // constexpr SimpleName = "constexpr_" + 非 constexpr 版 SimpleName (types.h:305-313)
  assert_eq!(
    oracle.types[c as usize].simple_name(&cx, &oracle),
    "constexpr_int32"
  );
  assert_eq!(oracle.types[nc as usize].simple_name(&cx, &oracle), "int32");
  // constexpr 的 generated 名去 TNode 包裹 (types.cc:215)
  assert_eq!(
    oracle.types[c as usize].get_generated_type_name(&cx, &oracle)?,
    "int32"
  );
  // GetGeneratedTNodeTypeName 对 constexpr 报错 (types.cc:192-199)
  assert!(
    oracle.types[c as usize]
      .get_generated_tnode_type_name(&cx, &oracle)
      .is_err()
  );
  OK
}

/// IsTransient 三处覆写 (types.h:140/437-443/689)
#[test]
fn types_is_transient() -> Void {
  info!("> types_is_transient");
  let mut oracle = TypeOracle::default();
  let transient = add_abstract(
    &mut oracle,
    None,
    "Transient",
    AbstractTypeFlag::Transient as u32,
    "",
  );
  let plain = add_abstract(&mut oracle, None, "Plain", 0, "Plain");
  assert!(oracle.types[transient as usize].is_transient(&oracle));
  assert!(!oracle.types[plain as usize].is_transient(&oracle));
  // Union: 任一成员 transient 即 transient, 全非则 false (types.h:437-443)
  let mixed = add_union(&mut oracle, &[plain, transient], None);
  let calm = add_union(&mut oracle, &[plain, plain], None);
  assert!(oracle.types[mixed as usize].is_transient(&oracle));
  assert!(!oracle.types[calm as usize].is_transient(&oracle));
  // Class: kTransient 位 (types.h:689)
  let decl: NodeId = 0;
  let transient_class = oracle.get_class_type(
    None,
    "TClass".to_owned(),
    ClassFlag::Transient as u32,
    "JST".to_owned(),
    decl,
    None,
  );
  let plain_class =
    oracle.get_class_type(None, "PClass".to_owned(), 0, "JP".to_owned(), decl, None);
  assert!(oracle.types[transient_class as usize].is_transient(&oracle));
  assert!(!oracle.types[plain_class as usize].is_transient(&oracle));
  OK
}

/// 原版 Combine(ObjectSlotKind, ObjectSlotKind) (types.h:666-674)
#[test]
fn types_combine_object_slot_kind() -> Void {
  info!("> types_combine_object_slot_kind");
  use ObjectSlotKind::{CustomWeakPointer, MaybeObjectPointer, NoPointer, StrongPointer};
  assert_eq!(
    combine_object_slot_kind(NoPointer, NoPointer),
    Some(NoPointer)
  );
  assert_eq!(
    combine_object_slot_kind(StrongPointer, StrongPointer),
    Some(StrongPointer)
  );
  // 强指针 + 可空指针 → 可空指针 (原 min/max 判别, 双向一致)
  assert_eq!(
    combine_object_slot_kind(StrongPointer, MaybeObjectPointer),
    Some(MaybeObjectPointer)
  );
  assert_eq!(
    combine_object_slot_kind(MaybeObjectPointer, StrongPointer),
    Some(MaybeObjectPointer)
  );
  // 其余组合 → nullopt
  assert_eq!(combine_object_slot_kind(NoPointer, StrongPointer), None);
  assert_eq!(
    combine_object_slot_kind(CustomWeakPointer, MaybeObjectPointer),
    None
  );
  OK
}

/// Signature 隐式/显式参数切分 (types.h:891-910)
#[test]
fn types_signature_implicit_split() -> Void {
  info!("> types_signature_implicit_split");
  let mut oracle = TypeOracle::default();
  let context = add_abstract(&mut oracle, None, "Context", 0, "Context");
  let a = add_abstract(&mut oracle, None, "Alpha", 0, "Alpha");
  let b = add_abstract(&mut oracle, None, "Beta", 0, "Beta");
  let sig = Signature {
    parameter_types: ParameterTypes {
      types: vec![context, a, b],
      var_args: false,
    },
    implicit_count: 1,
    return_type: Some(a),
    ..Default::default()
  };
  assert_eq!(sig.types().as_slice(), &[context, a, b]);
  assert_eq!(sig.explicit_count(), 2);
  assert_eq!(sig.get_implicit_types(), &[context][..]);
  assert_eq!(sig.get_explicit_types(), &[a, b][..]);
  OK
}
