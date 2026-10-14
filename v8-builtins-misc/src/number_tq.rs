// 由 tool/src/builtins/number.tq.js 从 v8/src/builtins/number.tq 生成; 修订就地改, 复跑覆盖
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

/// 原版顶层声明清单 (共 14 条)
pub const DECLS: &[TqDecl] = &[
  ("enum", "Operation", "enum Operation extends uint31"),
  ("namespace", "runtime", "namespace runtime"),
  ("namespace", "number", "namespace number"),
  (
    "const",
    "log2",
    "const log2: int32 = 31 - math::Word32Clz(Signed(n) | 1)",
  ),
  (
    "const",
    "tableEntry",
    "const tableEntry: uint64 = log10OffsetsTable[Convert<intptr>(log2)]",
  ),
  (
    "const",
    "digitCount",
    "const digitCount: uint64 = (Convert<uint64>(n) + tableEntry) >>> 32",
  ),
  (
    "const",
    "string",
    "const string = AllocateNonEmptySeqOneByteString(length)",
  ),
  (
    "const",
    "lengthIntptr",
    "const lengthIntptr = Convert<intptr>(Signed(length))",
  ),
  ("const", "rawChars", "const rawChars = &string.chars"),
  ("const", "isNegative", "const isNegative: bool = x < 0"),
  (
    "const",
    "strSeq",
    "const strSeq = AllocateNonEmptySeqOneByteString(Unsigned(length))",
  ),
  (
    "const",
    "x",
    "const x = ThisNumberValue(receiver, 'Number.prototype.toString')",
  ),
  ("const", "radix", "const radix: JSAny = arguments[0]"),
  (
    "const",
    "radixNumber",
    "const radixNumber: Number = radix == Undefined ? 10 : ToInteger_Inline(radix)",
  ),
];

// —— 对数断言 ——
const _: () = {
  assert!(DECLS.len() == 14);
  assert!(DECLS.iter().filter(|d| d.kind == "enum").count() == 1);
  assert!(DECLS.iter().filter(|d| d.kind == "namespace").count() == 2);
  assert!(DECLS.iter().filter(|d| d.kind == "const").count() == 11);
};
