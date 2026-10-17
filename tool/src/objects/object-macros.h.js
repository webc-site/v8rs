#!/usr/bin/env bun
// tool/src/objects/object-macros.h.js — 覆盖 v8/src/objects/object-macros.h (~1207 行)
//   + objects-inl.h 的 DEF_CAST_TRAITS 调用表 (82-320 行)
//   → v8-objects-core/src/objects.rs 的 @object-macros 区段 (区段标记替换, 幂等)
//
// X-macro / 访问器宏 → Rust 映射登记 (机械展开为规则代码的部分 + 惯用法替代部分):
//   DEF_CAST_TRAITS(Type) 调用表 → CAST_TRAITS_TYPES 常量表 (供各对象族 crate 落地
//     impl Cast 时核对覆盖; 本 crate 仅对已有标记类型实现, 见 objects.rs 主体)
//   V8_IT_ABSTRACT/OWN_TYPE/FIXED_VALUE/FLAG_BITS/ORDER/NO_AUTO_CHECKER →
//     已由 instance-type.h.js 求解器消费 (instance_type.rs)
//   DECL_PRIMITIVE_ACCESSORS/INT_ACCESSORS/SMI_ACCESSORS/BOOL_ACCESSORS/
//     ACCESSORS_CHECKED/RELEASE_ACQUIRE_ACCESSORS/BIT_FIELD_ACCESSORS →
//     Rust 惯用法替代: struct 直接字段 + 关联 fn (无宏展开产物)
//   RELAXED_READ_FIELD/WRITE_FIELD/SEQ_CST_SWAP_FIELD 等原子字段宏 →
//     std::sync::atomic 按宽度选型 (归各对象族 crate 按需落地)
//   WRITE_BARRIER/CONDITIONAL_WRITE_BARRIER 宏族 → WriteBarrierMode 枚举已落
//     objects.rs; 屏障执行归 v8-heap-core
//   DECL_PRINTER/DECL_VERIFIER (OBJECT_PRINT) → 归 L4 v8-diagnostics
//   V8_OBJECT/V8_OBJECT_END (pack 压缩) → #[repr(C)] + 布局 static_assert (对象族)
//   TQ_FIELD_TYPE/DECL_FIELD_OFFSET_TQ (Torque 字段) → Torque 生成器输出 (v8-torque)
import { banner, readV8 } from "../../lib.js";
import { readFileSync, writeFileSync } from "node:fs";
import { join } from "node:path";

const root = join(import.meta.dir, "../../..");
const target = join(root, "v8-objects-core/src/objects.rs");

// 提取 objects-inl.h 的 DEF_CAST_TRAITS 调用 (含 #if 段照收, 由 WASM/INTL 配置注明)
const inl = readV8("src/objects/objects-inl.h");
const calls = [...inl.matchAll(/^DEF_CAST_TRAITS\((\w+)\)/gm)].map((m) => m[1]);
if (calls.length < 150) throw new Error(`DEF_CAST_TRAITS 提取数异常: ${calls.length}`);
// 去重保序 (HEAP_OBJECT_TRUSTED_TYPE_LIST/HOLE_LIST 展开项与显式项可能重名)
const seen = new Set();
const types = calls.filter((t) => (seen.has(t) ? false : (seen.add(t), true)));

const macros = readV8("src/objects/object-macros.h");
// 宏清单对数锚点
for (const a of ["#define DEF_CAST_TRAITS", "#define DECL_ACCESSORS", "#define SMI_ACCESSORS", "#define WRITE_BARRIER"]) {
  if (!macros.includes(a)) throw new Error(`object-macros.h 锚点失配: ${a}`);
}

const section = `${banner("src/objects/object-macros.h.js", "src/objects/object-macros.h + objects-inl.h DEF_CAST_TRAITS 表", `访问器宏 → Rust 惯用法映射登记 (见脚本头); DEF_CAST_TRAITS ${types.length} 项`)}
/// DEF_CAST_TRAITS 调用表 (objects-inl.h:82-320 机械提取, 含 WASM/INTL 配置项)。
/// 各对象族 crate 落地 \`impl Cast\` 时以此核对 1:1 覆盖。
pub const CAST_TRAITS_TYPES: &[&str] = &[
${types.map((t) => `  "${t}",`).join("\n")}
];
`;

const cur = readFileSync(target, "utf8");
const BEGIN = "// @object-macros-begin";
const END = "// @object-macros-end";
const bi = cur.indexOf(BEGIN);
const ei = cur.indexOf(END);
if (bi < 0 || ei < 0) throw new Error("objects.rs 缺少 @object-macros 区段标记 (先跑 objects.h.js)");
const next = cur.slice(0, bi + BEGIN.length) + "\n" + section + "\n" + cur.slice(ei);
writeFileSync(target, next);
console.log(`objects.rs @object-macros 区段已注入: CAST_TRAITS_TYPES ${types.length} 项`);
