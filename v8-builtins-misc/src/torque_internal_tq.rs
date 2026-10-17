// 由 tool/src/builtins/torque-internal.tq.js 从 v8/src/builtins/torque-internal.tq 生成; 修订就地改, 复跑覆盖
// Torque 声明骨架 — 声明清单/常量直译/C++ 符号引用, 函数体不展开
//! 任务映射: v8-builtins-misc; 聚合基线脚本另产同源模块 (并存不覆盖)

/// Torque 声明条目 (kind: namespace/type/const/macro/builtin/struct/enum/class/import)
#[derive(Debug, Clone, Copy)]
pub struct TqDecl {
  pub kind: &'static str,
  pub name: &'static str,
  /// 原版签名 (空白规整; 含 labels/js-implicit 等修饰)
  pub sig: &'static str,
}

/// 原版顶层声明清单 (共 21 条)
pub const DECLS: &[TqDecl] = &[
  (
    "type",
    "MutableSlice",
    "type MutableSlice<T: type> extends torque_internal::Slice<T, &T>",
  ),
  (
    "type",
    "ConstSlice",
    "type ConstSlice<T: type> extends torque_internal::Slice<T, const &T>",
  ),
  (
    "macro",
    "Subslice",
    "macro Subslice<T: type>( slice: ConstSlice<T>, start: intptr, length: intptr): ConstSlice<T>labels OutOfBounds",
  ),
  (
    "macro",
    "Subslice",
    "macro Subslice<T: type>( slice: MutableSlice<T>, start: intptr, length: intptr): MutableSlice<T>labels OutOfBounds",
  ),
  ("namespace", "unsafe", "namespace unsafe"),
  (
    "macro",
    "AddOffset",
    "macro AddOffset<T: type>(ref: &T, offset: intptr): &T",
  ),
  (
    "macro",
    "AddOffset",
    "macro AddOffset<T: type>(ref: const &T, offset: intptr): const &T",
  ),
  ("namespace", "torque_internal", "namespace torque_internal"),
  (
    "macro",
    "GCUnsafeRawPtr",
    "macro GCUnsafeRawPtr(): RawPtr<T>",
  ),
  (
    "const",
    "object",
    "const object: HeapObject|TaggedZeroPattern",
  ),
  ("const", "offset", "const offset: intptr"),
  ("namespace", "unsafe", "namespace unsafe"),
  (
    "const",
    "ref",
    "const ref = NewReference<T>(ref.object, ref.offset)",
  ),
  ("const", "i", "const i: intptr = Convert<intptr>(index)"),
  ("const", "i", "const i: intptr = Convert<intptr>(index)"),
  ("const", "i", "const i: intptr = Convert<intptr>(index)"),
  (
    "const",
    "end",
    "const end = this.offset + TimesSizeOf<T>(this.length)",
  ),
  (
    "const",
    "start",
    "const start = this.offset + TimesSizeOf<T>(startIndex)",
  ),
  (
    "const",
    "end",
    "const end = this.offset + TimesSizeOf<T>(endIndex)",
  ),
  ("namespace", "unsafe", "namespace unsafe"),
  (
    "const",
    "result",
    "const result = unsafe::NewReference<T>(this.object, this.start)",
  ),
];

// —— 对数断言 ——
const _: () = {
  assert!(DECLS.len() == 21);
  assert!(DECLS.iter().filter(|d| d.kind == "type").count() == 2);
  assert!(DECLS.iter().filter(|d| d.kind == "macro").count() == 5);
  assert!(DECLS.iter().filter(|d| d.kind == "namespace").count() == 4);
  assert!(DECLS.iter().filter(|d| d.kind == "const").count() == 10);
};
