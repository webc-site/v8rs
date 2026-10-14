use std::str::FromStr;

use aok::{OK, Void};
use log::info;
use strum::IntoEnumIterator;
use v8_builtins_array::{
  ARRAY_TQ_DECLS, ArrayBuiltin, DEFAULT_MIN_GROWTH, Error, GROWABLE_TQ_DECLS, GrowableFixedArray,
  MAX_FIXED_ARRAY_CAPACITY, array_concat, array_pop, array_push,
};
use v8_builtins_core::{BUILTIN_TABLE_LEN, get_builtin};

#[ctor::ctor(unsafe)]
fn _log_init() {
  log_init::init();
}

#[test]
fn test_array_builtins_iteration_and_strum() -> Void {
  let mut count = 0;
  for b in ArrayBuiltin::iter() {
    count += 1;
    let s = b.to_string();
    assert_eq!(s, b.name());

    let parsed = ArrayBuiltin::from_str(&s)?;
    assert_eq!(parsed, b);

    // 验证描述符符号规范
    let desc = b.descriptor_name();
    assert_eq!(desc, format!("Builtin_{}_InterfaceDescriptor", b.name()));
  }
  assert_eq!(count, 33);

  info!("ArrayBuiltin 枚举遍历与 strum 反序列化验证通过 (共 33 项)");
  OK
}

#[test]
fn test_array_builtins_global_table_link() -> Void {
  // 验证每个数组内建均已在 v8-builtins-core 的全局 BUILTIN_TABLE 注册
  for b in ArrayBuiltin::iter() {
    let global_id = b.global_id().unwrap_or_else(|| {
      panic!("ArrayBuiltin {} 必须在全局 BUILTIN_TABLE 中存在", b.name());
    });
    assert!(global_id < BUILTIN_TABLE_LEN);

    let (kind, name, line) = get_builtin(global_id).expect("有效索引必定返回内建元数据");
    assert_eq!(name, b.name());
    assert!(!kind.is_empty());
    assert!(line > 0);
  }

  info!("ArrayBuiltin 全局内建注册表映射与关联验证通过");
  OK
}

#[test]
fn test_array_builtins_categories() -> Void {
  // 原型方法分类
  assert!(ArrayBuiltin::ArrayPrototypePop.is_prototype_method());
  assert!(ArrayBuiltin::ArrayPrototypePush.is_prototype_method());
  assert!(ArrayBuiltin::ArrayPrototypeFill.is_prototype_method());
  assert!(ArrayBuiltin::ArrayIteratorPrototypeNext.is_prototype_method());
  assert!(!ArrayBuiltin::ArrayConcat.is_prototype_method());

  // 搜索类方法
  assert!(ArrayBuiltin::ArrayIndexOf.is_search_method());
  assert!(ArrayBuiltin::ArrayIncludes.is_search_method());
  assert!(ArrayBuiltin::ArrayIncludesHoleyDoubles.is_search_method());
  assert!(ArrayBuiltin::ArrayIndexOfPackedDoubles.is_search_method());
  assert!(!ArrayBuiltin::ArrayShift.is_search_method());

  // 快克隆类方法
  assert!(ArrayBuiltin::CloneFastJSArray.is_fast_clone_method());
  assert!(ArrayBuiltin::CloneFastJSArrayFillingHoles.is_fast_clone_method());
  assert!(ArrayBuiltin::ExtractFastJSArray.is_fast_clone_method());
  assert!(!ArrayBuiltin::ArrayPop.is_fast_clone_method());

  // 构造类方法
  assert!(ArrayBuiltin::ArrayConstructor.is_constructor());
  assert!(ArrayBuiltin::ArrayConstructorImpl.is_constructor());
  assert!(ArrayBuiltin::ArrayNArgumentsConstructor.is_constructor());
  assert!(!ArrayBuiltin::ArrayPush.is_constructor());

  // 调度入口直接构造
  assert_eq!(array_concat(), ArrayBuiltin::ArrayConcat);
  assert_eq!(array_pop(), ArrayBuiltin::ArrayPop);
  assert_eq!(array_push(), ArrayBuiltin::ArrayPush);

  info!("ArrayBuiltin 语义分类判定逻辑验证通过");
  OK
}

#[test]
fn test_growable_fixed_array_lifecycle() -> Void {
  let mut arr: GrowableFixedArray<u64> = GrowableFixedArray::new();
  assert_eq!(arr.length(), 0);
  assert_eq!(arr.capacity(), 0);
  assert!(arr.is_empty());

  // 1. 扩容公式核算: new_capacity = c + (c >> 1) + 16
  assert_eq!(GrowableFixedArray::<u64>::next_capacity(0), 16);
  assert_eq!(GrowableFixedArray::<u64>::next_capacity(16), 40);
  assert_eq!(GrowableFixedArray::<u64>::next_capacity(40), 76);

  // 2. 逐个推送 20 个元素
  for i in 0..20 {
    arr.push(i * 10)?;
  }
  assert_eq!(arr.length(), 20);
  assert!(!arr.is_empty());
  // 第 1 次扩容为 16，填满后第 17 次扩容为 40
  assert_eq!(arr.capacity(), 40);

  // 3. 校验元素正确性
  for i in 0..20 {
    assert_eq!(arr.get(i), Some(&((i * 10) as u64)));
  }
  assert_eq!(arr.get(20), None);

  // 3.1 校验可变引用写入
  if let Some(val) = arr.get_mut(0) {
    *val = 999;
  }
  assert_eq!(arr.get(0), Some(&999));
  if let Some(val) = arr.get_mut(0) {
    *val = 0;
  }
  assert_eq!(arr.get(0), Some(&0));

  // 4. 收缩到紧凑状态
  arr.shrink_to_fit();
  assert_eq!(arr.capacity(), 20);
  assert_eq!(arr.length(), 20);
  assert_eq!(arr.as_slice().len(), 20);

  // 5. 清空
  arr.clear();
  assert_eq!(arr.length(), 0);
  assert!(arr.is_empty());

  info!("GrowableFixedArray 生命周期、扩容公式与紧凑化验证通过");
  OK
}

#[test]
fn test_growable_fixed_array_reserve_and_overflow() -> Void {
  let mut arr: GrowableFixedArray<i32> = GrowableFixedArray::with_capacity(30)?;
  // 预留 30: 0 -> 16 -> 40
  assert_eq!(arr.capacity(), 40);
  assert_eq!(arr.length(), 0);

  // 预留超大容量触发溢出错误
  let err = arr.reserve(MAX_FIXED_ARRAY_CAPACITY + 1);
  assert!(matches!(err, Err(Error::CapacityOverflow { .. })));

  info!("GrowableFixedArray 预留容量与超限防护验证通过");
  OK
}

#[test]
fn test_tq_declarations_metadata() -> Void {
  assert_eq!(ARRAY_TQ_DECLS.len(), 67);
  assert_eq!(GROWABLE_TQ_DECLS.len(), 2);
  assert_eq!(DEFAULT_MIN_GROWTH, 16);

  for (kind, name) in ARRAY_TQ_DECLS {
    assert!(!kind.is_empty());
    assert!(!name.is_empty());
  }

  info!("TQ 声明元数据条目健全性测试通过");
  OK
}

#[test]
fn test_growable_fixed_array_move_only_type() -> Void {
  struct MoveOnly(u64);

  let mut arr: GrowableFixedArray<MoveOnly> = GrowableFixedArray::new();
  arr.push(MoveOnly(42))?;
  assert_eq!(arr.length(), 1);
  assert_eq!(arr.get(0).map(|x| x.0), Some(42));

  info!("GrowableFixedArray 支持非 Clone/Default 泛型类型验证通过");
  OK
}
