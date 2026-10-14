#!/usr/bin/env bun
// tool/src/codegen/heap-object-list.h.js
// 机械展开 v8/src/codegen/heap-object-list.h 的 X-macro 对象清单:
// HEAP_MUTABLE_IMMOVABLE_OBJECT_LIST / HEAP_IMMUTABLE_IMMOVABLE_OBJECT_LIST
// → v8-codegen-core/src/heap_object_list.rs

import { readV8, writeSrc, banner } from "../../lib.js";

const src = readV8("src/codegen/heap-object-list.h");

// 行续接拼接后按宏体提取 V(camle, underscore, Class) 三元组
const flat = src.replace(/\\\r?\n/g, " ");

function macroEntries(name) {
  const re = new RegExp(`#define ${name}\\(V\\)([\\s\\S]*?)(?=#define |#endif)`);
  const m = flat.match(re);
  if (!m) throw new Error(`未找到宏 ${name}`);
  const entries = [...m[1].matchAll(/V\(([^)]+)\)/g)].map((x) =>
    x[1].split(",").map((s) => s.trim()),
  );
  return entries;
}

// BUILTINS_WITH_SFI_OBJECT_LIST 依赖外部 BUILTINS_WITH_SFI_LIST_GENERATOR
// (torque 生成), 无法本地展开 → 登记
const hasExternalList = flat.includes("BUILTINS_WITH_SFI_LIST_GENERATOR(");
if (!hasExternalList) throw new Error("缺 BUILTINS_WITH_SFI_LIST_GENERATOR 登记");

const mutableList = macroEntries("HEAP_MUTABLE_IMMOVABLE_OBJECT_LIST");
const immutableList = macroEntries("HEAP_IMMUTABLE_IMMOVABLE_OBJECT_LIST");

// 对数 (原版侧独立计数)
const EXPECT_MUTABLE = 15; // 15 个 protector (BUILTINS_WITH_SFI 挂载行为引用行, 不展开)
const EXPECT_IMMUTABLE = 0; // 下面先实数, 再核对
if (mutableList.length !== EXPECT_MUTABLE)
  throw new Error(`MUTABLE 清单 ${mutableList.length} != ${EXPECT_MUTABLE}`);
if (immutableList.length < 50)
  throw new Error(`IMMUTABLE 清单过少: ${immutableList.length}`);
// 首尾对数
if (mutableList[0][0] !== "ArrayIteratorProtector") throw new Error("MUTABLE 首项不符");
if (immutableList[0][0] !== "AllocationSiteWithoutWeakNextMap")
  throw new Error("IMMUTABLE 首项不符");

function fmtTable(name, list, note) {
  const rows = list
    .map(([camel, underscore, cls]) => `  (${JSON.stringify(camel)}, ${JSON.stringify(underscore)}, ${JSON.stringify(cls)}),`)
    .join("\n");
  return `/// ${note}\npub const ${name}: &[(&str, &str, &str)] = &[\n${rows}\n];\n`;
}

let rust = banner(
  "src/codegen/heap-object-list.h.js",
  "src/codegen/heap-object-list.h",
  "堆不可移动对象清单 (X-macro 机械展开)",
);

rust += `
// 三元组语义: (CamelName, underscore_name, class_name)。
// BUILTINS_WITH_SFI_OBJECT_LIST 由 BUILTINS_WITH_SFI_LIST_GENERATOR(torque 生成)驱动,
// 本地无法展开 — 登记跳过, MUTABLE 表不含该挂载点展开。

${fmtTable("HEAP_MUTABLE_IMMOVABLE_OBJECT_LIST", mutableList, "可变不可移动对象清单 (protector 族)")}

${fmtTable("HEAP_IMMUTABLE_IMMOVABLE_OBJECT_LIST", immutableList, "不可变不可移动对象清单 (maps/strings/哨兵族)")}

// —— 提取不变式断言 ——
const _: () = {
  assert!(HEAP_MUTABLE_IMMOVABLE_OBJECT_LIST.len() == ${mutableList.length});
  assert!(HEAP_IMMUTABLE_IMMOVABLE_OBJECT_LIST.len() == ${immutableList.length});
  assert!(HEAP_MUTABLE_IMMOVABLE_OBJECT_LIST[0].0 == "ArrayIteratorProtector");
  assert!(HEAP_IMMUTABLE_IMMOVABLE_OBJECT_LIST[0].0 == "AllocationSiteWithoutWeakNextMap");
  // 三元组三列非空
  for MUT in HEAP_MUTABLE_IMMOVABLE_OBJECT_LIST { assert!(!MUT.0.is_empty() && !MUT.1.is_empty()); }
  for IM in HEAP_IMMUTABLE_IMMOVABLE_OBJECT_LIST { assert!(!IM.0.is_empty() && !IM.1.is_empty()); }
};
`;

const p = writeSrc("v8-codegen-core", "heap_object_list.rs", rust);
console.log(
  `heap-object-list: MUTABLE ${mutableList.length} / IMMUTABLE ${immutableList.length} (基准待复核) → ${p}`,
);
