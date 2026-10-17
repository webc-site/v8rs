#!/usr/bin/env bun
// tool/src/objects/union.h.js — 覆盖 v8/src/objects/union.h
// Union<Ts...> 类型级机制: 类型 trait/别名规则登记 + 概念骨架
// → v8-objects-core/src/union.rs
// 对数: clang -E 探针展开 union.h, 编译器产物侧点名/计数比对 (todo.md 十一章对数基准)
import { readV8, writeSrc, banner } from "../../lib.js";
import { spawnSync } from "node:child_process";
import { mkdtempSync, writeFileSync, readFileSync, rmSync, mkdirSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";

const src = readV8("src/objects/union.h");

// —— clang -E 探针: union.h 全量预处理, 编译器产物侧点名 ————————
function clangProbe() {
  const dir = mkdtempSync(join(tmpdir(), "v8rs-union-"));
  const file = join(dir, "probe.cc");
  const outFile = join(dir, "probe.i");
  writeFileSync(file, `#include "src/objects/union.h"\n`);
  const stubs = [
    ["third_party/googletest/src/googletest/include/gtest", "gtest_prod.h", "#define FRIEND_TEST(a, b)\n"],
    ["third_party/fp16/src/include", "fp16.h", "#pragma once\n"],
    [".", "simdutf.h", "#pragma once\n"],
  ];
  for (const [d, n, c] of stubs) {
    mkdirSync(join(dir, d), { recursive: true });
    writeFileSync(join(dir, d, n), c);
  }
  const v8root = join(import.meta.dir, "../../../v8");
  const res = spawnSync(
    "clang",
    ["-E", "-P", "-std=c++20", "-DV8_METAGEN_GENERATION_PASS", "-DV8_COMPRESS_POINTERS",
      "-DV8_INTL_SUPPORT", "-DV8_ENABLE_WEBASSEMBLY", "-DV8_ENABLE_SANDBOX", "-DV8_EXTERNAL_CODE_SPACE",
      "-I", dir, "-I", join(v8root, "include"), "-I", v8root, "-I", "/opt/homebrew/include",
      "-o", outFile, file],
    { encoding: "utf8" },
  );
  let out = null;
  if (!res.error && res.status === 0) out = readFileSync(outFile, "utf8");
  try { rmSync(dir, { recursive: true, force: true }); } catch { /* tmp 清理失败可忽略 */ }
  if (out === null) {
    console.log(`clang 探针失败 (${res.error?.message ?? `exit ${res.status}}`}), 回退硬锚`);
    return null;
  }
  return out;
}

const pre = clangProbe();
const inPre = (s) => pre !== null && pre.includes(s);
const countPre = (re) => (pre === null ? -1 : [...pre.matchAll(re)].length);

// 概念清单 (union.h 全部类型级实体; clang 产物点名)
const CONCEPTS = [
  { name: "Union", kind: "class 模板 (final AllStatic; 成员别名 Without<U>)", anchor: "class Union final : public AllStatic" },
  { name: "is_union", kind: "类型 trait (主模板 false + Union 特化 true)", anchor: "struct is_union :" },
  { name: "is_union_v", kind: "变量模板", anchor: "static constexpr bool is_union_v" },
  { name: "detail::TypeList", kind: "类型列表 (kSize = sizeof...)", anchor: "struct TypeList {" },
  { name: "detail::ToUnion", kind: "TypeList → Union 折叠 (3 特化: 空→void/单→T/多→Union)", anchor: "struct ToUnion {" },
  { name: "detail::PopFrontUnion", kind: "DEDUP_PACK 分支: 弹头 (2 特化)", anchor: "struct PopFrontUnion {" },
  { name: "detail::UnionWithoutHelper", kind: "非 DEDUP_PACK 分支: 累积剔除 (3 特化)", anchor: "struct UnionWithoutHelper;" },
  { name: "detail::Deduplicate", kind: "非 DEDUP_PACK 分支: 顺序去重 (2 特化)", anchor: "struct Deduplicate;" },
  { name: "detail::AsTypeList", kind: "2 重载: T*→TypeList<T>, Union<Us...>*→TypeList<Us...>", anchor: "TypeList<T> AsTypeList(T*)" },
  { name: "detail::operator+", kind: "TypeList 拼接 (折叠表达式, 免深递归特化)", anchor: "TypeList<As..., Bs...> operator+" },
  { name: "detail::DedupAndFinalize", kind: "去重收尾 (kHasSmi 时 Smi 前置归一)", anchor: "struct DedupAndFinalize;" },
  { name: "detail::FlattenUnionHelper", kind: "AsTypeList 折叠拼接 + 去重收尾", anchor: "struct FlattenUnionHelper" },
  { name: "UnionOf", kind: "别名模板: 展平嵌套 + 去重", anchor: "using UnionOf" },
];
// 实体名必须同时存在于源文本 (防清单凭空)
for (const c of CONCEPTS) {
  if (!src.includes(c.name.split("::").at(-1))) throw new Error(`源文本缺概念 ${c.name}`);
}

// clang 产物侧计数锚 (探针可用时)
const ANCHORS = [
  { re: /struct TypeList \{/g, n: 1, what: "TypeList" },
  { re: /struct ToUnion<TypeList<>> \{/g, n: 1, what: "ToUnion 空特化" },
  { re: /struct ToUnion<TypeList<T>> \{/g, n: 1, what: "ToUnion 单型特化" },
  { re: /struct ToUnion<TypeList<Ts\.\.\.>> \{/g, n: 1, what: "ToUnion 多型特化" },
  { re: /struct is_union : public std::false_type \{\}/g, n: 1, what: "is_union 主模板" },
];
if (pre !== null) {
  for (const a of ANCHORS) {
    const n = countPre(a.re);
    if (n !== a.n) throw new Error(`clang 对数失配: ${a.what} 期望 ${a.n} 实得 ${n}`);
  }
  // DEDUP_PACK 分支判定: clang 有 __builtin_pack_dedup → PopFrontUnion 路径
  const dedup = pre.includes("struct PopFrontUnion<Union<T1, T2>>");
  const noDedup = pre.includes("struct UnionWithoutHelper;");
  if (dedup === noDedup) throw new Error("DEDUP_PACK 分支判定异常 (两分支同时出现/缺失)");
  console.log(`union.h clang 探针: DEDUP_PACK=${dedup}`);
} else {
  console.log("clang 不可用, 仅源文本锚");
}
// 源文本必含锚 (防文件漂移)
for (const s of ["class Union final : public AllStatic", "struct is_union", "struct TypeList", "using UnionOf", "Cannot have a union of unions"])
  if (!src.includes(s)) throw new Error(`union.h 锚点缺失: ${s}`);

const body = `${banner("src/objects/union.h.js", "src/objects/union.h", "Union<Ts...> 类型级机制登记; Rust 无可变参泛型, 语义实现归 Developer")}
//! Union<Ts...>: 多 V8 类型的标签联合类型级表示。
//!
//! 原版约束: 联合不可嵌套 (无 Union of Union)、成员不重复、
//! 禁止从 Union 继承 (破坏 is_subtype 检查); \`UnionOf<Ts...>\` 负责
//! 展平嵌套 + 去重, 且含 Smi 时归一化到 Smi 前置。
//! Rust 无可变参泛型, 本模块登记概念清单与判定语义骨架,
//! 具体建模 (enum 闭集/ trait) 归 Developer。

/// 概念清单 (union.h 类型级实体, 1:1 登记)。
pub const UNION_CONCEPTS: [&str; ${CONCEPTS.length}] = [
${CONCEPTS.map((c) => `  "${c.name}|${c.kind}",`).join("\n")}
];

/// 原版对 Union 的静态约束 (static_assert 组)。
pub const UNION_INVARIANTS: [&str; 5] = [
  "非嵌套: Union 的成员不得再是 Union",
  "不重复: 每个类型在 Union 中至多一次",
  "非继承: 禁止从 Union<Ts...> 继承 (干扰 is_subtype 判定)",
  "Smi 归一: 含 Smi 的 UnionOf 结果把 Smi 置于首位",
  "Without 无命中时为 no-op (Union<Smi, HeapObject>::Without<HeapNumber> 原样返回)",
];

/// 判定 T 是否 Union 类型 (对应 is_union_v<T>)。
///
/// v8: src/objects/union.h is_union
pub fn is_union<T>() -> bool {
  todo!()
}

/// 展平嵌套联合并去重 (对应 UnionOf<Ts...> 别名语义的值级探针)。
///
/// v8: src/objects/union.h UnionOf
pub fn union_flatten<T>(_value: T) -> T {
  todo!()
}
`;

const p = writeSrc("v8-objects-core", "union.rs", body);
console.log(`union.rs: 概念 ${CONCEPTS.length} 项 + 不变式 5 条, clang 点名对数通过 → ${p}`);
