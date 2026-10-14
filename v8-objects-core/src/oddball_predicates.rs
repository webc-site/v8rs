// 由 tool/src/objects/oddball-predicates.h.js 从 v8/src/objects/oddball-predicates.{h,-inl.h} 生成; 修订就地改, 复跑覆盖
// Oddball/Hole 类型谓词表 (V8_ENABLE_TDZ_HOLE 关闭分支)

/// 谓词函数名全表 (原版 IS_TYPE_FUNCTION_DECL 展开序: ODDBALL_LIST → HOLE_LIST → 直书项)
/// 每项对应 inl 侧 bool Is<Type>(Tagged<Object>) 实现
pub const PREDICATES: [&str; 19] = [
  "Undefined",
  "Null",
  "True",
  "False",
  "TheHole",
  "PropertyCellHole",
  "HashTableHole",
  "PromiseHole",
  "ExceptionHole",
  "TerminationException",
  "UninitializedHole",
  "ArgumentsMarker",
  "OptimizedOut",
  "StaleRegister",
  "SelfReferenceMarker",
  "BasicBlockCountersMarker",
  "TdzHole",
  "UndefinedContextCell",
  "NullOrUndefined",
];

/// Oddball 项 (ODDBALL_LIST)
pub const ODDBALL_PREDICATES: [&str; 4] = ["Undefined", "Null", "True", "False"];

/// Hole 项 (HOLE_LIST, TDZ 关闭分支)
pub const HOLE_PREDICATES: [&str; 13] = [
  "TheHole",
  "PropertyCellHole",
  "HashTableHole",
  "PromiseHole",
  "DisabledTdzHole",
  "ExceptionHole",
  "TerminationException",
  "UninitializedHole",
  "ArgumentsMarker",
  "OptimizedOut",
  "StaleRegister",
  "SelfReferenceMarker",
  "BasicBlockCountersMarker",
];

// —— 签名骨架 (原版公开接口, 供 Developer 补语义) ——
// 每谓词两个重载: bool Is<Type>(Tagged<Object>, EarlyReadOnlyRoots) / bool Is<Type>(Tagged<Object>)
// inl 实现形态: 位比较 (oddball.kind()/roots 判等), 语义待 Developer 就地接根表
pub const PREDICATE_OVERLOADS: usize = 2;

// —— 提取不变式断言 ——
const _: () = {
  assert!(PREDICATES.len() == 19);
  assert!(ODDBALL_PREDICATES.len() == 4);
};
