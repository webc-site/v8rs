#!/usr/bin/env bun
// tool/src/objects/struct-inl.h.js — -inl 语义合并指引生成
// 组→文件映射 (本脚本管辖):
//   v8/src/objects/struct-inl.h → 合并进 v8-objects-arrays/src/struct_.rs
//   (Rust 无 inl; 产物 struct_inl.rs.skel 为逐定义合并指引, 最终以就地修订 src 为准)
// 落地基线 (AA-2R 修订): Tagged<X> inherent impl; 槽访问 = Relaxed 原子 + 写屏障桩
//   (fixed_array_base::write_barrier_stub 先例); AccessorComponent 复用 core objects 枚举。
import { readV8, writeSrc, banner } from "../../lib.js";

const inl = readV8("src/objects/struct-inl.h");

let checks = 0;
const must = (cond, msg) => {
  checks++;
  if (!cond) throw new Error(`锚点失配: ${msg}`);
};
const has = (s, sub, where) => {
  if (!s.includes(sub)) throw new Error(`锚点失配: ${where} 无 "${sub.slice(0, 64)}"`);
  checks++;
};
const eq = (actual, expected, msg) => {
  checks++;
  if (actual !== expected) throw new Error(`锚点失配: ${msg} (期望 ${expected}, 实得 ${actual})`);
};
const countOf = (s, re) => (s.match(re) || []).length;

// ── 1. 定义全集对数 (机械提取 类::方法, 双向严格核对) ────────────────────
const defs = [...new Set([...inl.matchAll(/(\w+)(?:<[^<>]*>)?::(\w+)\s*\(/g)].map((m) => `${m[1]}::${m[2]}`))];
const expectDefs = [
  "Struct::Struct",
  "Tuple2::Tuple2", "Tuple2::value1", "Tuple2::set_value1", "Tuple2::value2", "Tuple2::set_value2",
  "AccessorPair::AccessorPair", "AccessorPair::get", "AccessorPair::set",
  "AccessorPair::getter", "AccessorPair::set_getter", "AccessorPair::setter", "AccessorPair::set_setter",
  "AccessorPair::SetComponents", "AccessorPair::Equals",
  "ClassPositions::ClassPositions", "ClassPositions::start", "ClassPositions::end",
  "ForInEnumeratorHolder::ForInEnumeratorHolder", "ForInEnumeratorHolder::enum_cache_map",
  "ForInEnumeratorHolder::named_keys", "ForInEnumeratorHolder::elements_length",
  "ForInEnumeratorHolder::cache_length",
];
eq(expectDefs.length, 23, "指引清单条目数");
for (const d of expectDefs) must(defs.includes(d), `定义 ${d} 缺失: ${defs}`);
// 全集恰为 23 + 外部调用 Smi::FromInt (构造参数编码, 非本族定义) — 双向严格
const extras = defs.filter((d) => !expectDefs.includes(d));
eq(extras.length, 1, "非清单定义数 (应仅外部 Smi::FromInt 调用)");
must(extras[0] === "Smi::FromInt", `额外项应为 Smi::FromInt, 实得 ${extras}`);

// ── 2. 重载族额外对数 (出现次数 = 定义 + 调用, 实测锚定) ─────────────────
eq(countOf(inl, /value1\(/g), 6, "value1( 出现次数 (定义 3 + set_value1 定义 3)");
eq(countOf(inl, /set_getter\(/g), 5, "set_getter( 出现次数 (定义 2 + 调用 3)");
eq(countOf(inl, /\.load\(\)/g), 10, "TaggedMember .load() 读次数");
eq(countOf(inl, /\(this, value/g), 9, "带宿主 store 次数 (store 4 + Relaxed 2 + Release 3)");
eq(countOf(inl, /\.Relaxed_Store\(this/g), 2, "Relaxed_Store 带宿主次数");
eq(countOf(inl, /\.Release_Store\(this/g), 3, "Release_Store 带宿主次数");
eq(countOf(inl, /Smi::FromInt/g), 2, "Smi::FromInt 编码次数 (start/end)");
eq(countOf(inl, /IsNull\(/g), 2, "IsNull 跳空判定次数 (SetComponents)");

// ── 3. 语义锚点 ──────────────────────────────────────────────────────────
has(inl, "Struct::Struct(Tagged<ReadOnly<Map>> map) : HeapObject(map) {}", "Struct 构造");
has(inl, "value1_(witness, value1),", "Tuple2 构造 TaggedMember 初始化");
has(inl, "Tagged<Object> Tuple2::value1() const { return value1_.load(); }", "Tuple2::value1");
has(inl, "value1_.Relaxed_Store(this, value, mode);", "Relaxed 写族");
has(inl, "value1_.Release_Store(this, value, mode);", "Release 写族");
has(inl, "return component == ACCESSOR_GETTER ? getter() : setter();", "AccessorPair::get 分派");
has(inl, "getter_(witness.roots().null_value()),", "AccessorPair 初始 null");
has(inl, "if (!IsNull(getter)) set_getter(getter);", "SetComponents 跳 null");
has(inl, "return (getter() == getter_value) && (setter() == setter_value);", "Equals");
has(inl, "start_(Smi::FromInt(start)),", "ClassPositions Smi 编码");
has(inl, "return start_.load().value();", "ClassPositions::start 解包");
has(inl, "elements_length_(elements_length),", "ForInEnumeratorHolder 非 witness 初始化");

// ── 4. 合并指引产物 ──────────────────────────────────────────────────────
const rows = expectDefs
  .map((d) => {
    const [cls, m] = d.split("::");
    const rs = /^[A-Z]/.test(m) ? m.replace(/([A-Z])/g, "_$1").toLowerCase() : m.toLowerCase();
    return `// ${d} → impl Tagged<${cls}>::${m === cls ? "new" : rs}`;
  })
  .join("\n");

const body = `${banner("src/objects/struct-inl.h.js", "src/objects/struct-inl.h", "-inl 合并指引; Rust 无 inl, 语义并入 struct_.rs 就地修订")}
//! -inl 合并指引 (23 个定义, 构造/tag 重载映射登记)
//!
//! 命名与语义映射:
//!   1. X::X(构造) → X::new(map, ...) 头结构体构造; AllocationWitness/ReadOnlyRoots
//!      未落地 → map 由调用方显式传入 (偏差登记); roots().null_value() → Tagged::NULL
//!      哨兵 (roots 落地后替换); Smi::FromInt → Smi::from_int
//!   2. 普通 load/store → Relaxed 原子读 + store(this,value,mode) → Relaxed 写 + 屏障桩
//!      (C++ 普通字段访问在并发标记配置下即 V8_ATOMIC_OBJECT_FIELD_WRITES=Relaxed,
//!      同 core TaggedMember / fixed_array 先例)
//!   3. Relaxed_Load/Relaxed_Store/Acquire_Load/Release_Store → 方法后缀
//!      _relaxed/_acquire/_release (tag 参数重载消解)
//!   4. AccessorPair::get/set(component) → 分派 getter/setter; AccessorComponent
//!      复用 v8_objects_core::objects::AccessorComponent (ACCESSOR_GETTER →
//!      AccessorGetter)
//!   5. IsNull(v) → v.is_null() (Tagged::NULL 哨兵, roots 落地后替换)
//!   6. ClassPositions start_/end_ → Smi::from_int 编码写入; start()/end() 返回
//!      i32 (= .load().value() 解包)
//!   7. DECL_PRINTER/DECL_VERIFIER/BriefPrintDetails (L4 诊断) 与
//!      StructBodyDescriptor (heap 层 BodyDescriptor) → 推迟不落地 (票单)
//!   8. AccessorPair::Copy/GetComponent (Isolate/Handle/NativeContext) → 推迟 (票单)
${rows}
`;

const p = writeSrc("v8-objects-arrays", "struct_inl.rs.skel", body);
console.log(
  `struct-inl.h.js: 合并指引 ${p} — 定义全集 ${expectDefs.length}/23 (双向严格, 额外仅 Smi::FromInt)、` +
    `重载/槽访问对数锚定 (load 10/store 9/FromInt 2/IsNull 2)、断言 ${checks} 项全部通过`,
);
