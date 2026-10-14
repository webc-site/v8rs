#!/usr/bin/env bun
// tool/src/objects/map.cc.js — 覆盖 v8/src/objects/map.cc 的 Map 自身基础语义子集
//   → v8-objects-core/src/map.rs 的 @cc-semantics 区段 (区段标记替换, 幂等;
//     不触碰 map.h.js 生成的其余内容)
//
// 转写区段登记 (map.cc 实测行号):
//   69-559   Map::GetVisitorId 大 switch (X-macro 机械展开:
//            TYPED_ARRAYS 构造器族 + SIMPLE_HEAP_OBJECT_LIST2 +
//            CONCRETE_TRUSTED_OBJECT_TYPE_LIST2 尾部宏块; IF_WASM/IF_INTL
//            按默认配置展开, V8_TEMPORAL_SUPPORT 默认关剔除)
//   2742-2757 ToString(VisitorId) → visitor_id_to_str
//
// 语义缺口登记 (get_visitor_id 内):
//   - JS_PROMISE_TYPE 分支 (map.cc:297-301): 依赖 COMPRESS_POINTERS_BOOL 与
//     JSObject::GetEmbedderFieldCount (JSObject 头部布局) → 落 Err,
//     归 v8-objects-js-core 承接
//   - 默认 JSObject 快族分支的 CHECK_EQ(0, embedder count) (map.cc:386):
//     空间位置断言依赖同上, 剔除断言保留返回值
//   - 尾部 FATAL (map.cc:556-558): 穷举 match 消灭 (编译期保证全覆盖);
//            InstanceType 若有新增未映射变体, 编译失败即暴露
//   - string 头部分支 (map.cc:74-100): kStringRepresentationMask/
//            kStringEncodingMask 判定 + IsShortcutCandidate 1:1
//   - JS_API_OBJECT 前置分支 (map.cc:102-104): InstanceTypeChecker::IsJSApiObject
//
// 跳过区段登记 (推迟, 归属见 map.h.js 头注释):
//   map.cc 其余全部 (isolate/transition/DescriptorArray 依赖:
//   Copy/Normalize/TryUpdate/Hash/NextFreeFieldStorageLocation/
//   ShouldCheckForReadOnlyElementsInPrototypeChain/MapPrint/...)
import { banner, readV8 } from "../../lib.js";
import { readFileSync, writeFileSync } from "node:fs";
import { join } from "node:path";

const root = join(import.meta.dir, "../../..");
const cc = readV8("src/objects/map.cc");
const instanceTypeRs = readFileSync(join(root, "v8-objects-core/src/instance_type.rs"), "utf8");
const target = join(root, "v8-objects-core/src/map.rs");

// ── 锚点 ────────────────────────────────────────────────────────────────
for (const [sub] of [
  ["VisitorId Map::GetVisitorId(Tagged<Map> map) {"],
  ["const char* ToString(VisitorId visitor_id) {"],
  ["SIMPLE_HEAP_OBJECT_LIST2(CASE)"],
]) {
  if (!cc.includes(sub)) throw new Error(`map.cc 锚点失配: "${sub.slice(0, 44)}..."`);
}

// ── InstanceType 变体名集合 + 别名表 (校验 case 标签可解析) ────────────────
const variants = new Set([...instanceTypeRs.matchAll(/^  (\w+) = \d+,$/gm)].map((m) => m[1]));
// 别名: pub const JS_FUNCTION_TYPE: InstanceType = InstanceType::JSFunctionWithPrototype;
// (C++ TYPE 名与 Rust 变体名不同名时的桥)
const aliases = new Map(
  [...instanceTypeRs.matchAll(/^pub const (\w+): InstanceType = InstanceType::(\w+);$/gm)].map((m) => [m[1], m[2]]),
);

// TYPE_UPPER_CASE → Rust 变体名 (与 instance_type.h.js 生成规则一致:
// 分段 PascalCase; 全大写段仅首字母大写, JS 段保留, 尾部 _TYPE 剥离)
function typeUpperToVariant(t) {
  let name = t.replace(/_TYPE$/, "");
  const segs = name.split("_").filter(Boolean);
  const conv = segs
    .map((s) => {
      if (s === "JS") return "JS";
      if (/^[A-Z0-9]+$/.test(s)) {
        // 纯大写段: UINT8 → Uint8, CLAMPED → Clamped, BIGINT64 → Bigint64
        return s[0] + s.slice(1).toLowerCase();
      }
      return s;
    })
    .join("");
  return conv;
}

// 校验函数: 先别名表, 再直转; 报错时列出无法解析的标签
function resolve(label) {
  if (aliases.has(label)) return aliases.get(label);
  const v = typeUpperToVariant(label);
  if (!variants.has(v)) throw new Error(`case ${label} → ${v} 不在 instance_type.rs 变体集`);
  return v;
}

// ── 解析 GetVisitorId switch 体 (map.cc:106-555) ─────────────────────────
function sliceBetween(s, from, to) {
  const i = s.indexOf(from);
  if (i < 0) throw new Error(`找不到 "${from.slice(0, 40)}"`);
  const j = s.indexOf(to, i);
  if (j < 0) throw new Error(`找不到 "${to.slice(0, 40)}"`);
  return s.slice(i, j);
}
let switchBody = sliceBetween(cc, "switch (instance_type) {", "std::string name = ToString(map->instance_type());");

// 1) 展开 TYPED_ARRAYS 构造器宏块 (map.cc:259-262)
const taEntries = readV8("src/objects/elements-kind.h")
  .match(/#define TYPED_ARRAYS_BASE\(V\)([\s\S]*?)\n\n/)[1];
const taNames = [...taEntries.matchAll(/V\((\w+), \w+, (\w+),/g)].map((m) => [m[1], m[2]]);
switchBody = switchBody.replace(
  /#define TYPED_ARRAY_CONSTRUCTORS_SWITCH[\s\S]*?#undef TYPED_ARRAY_CONSTRUCTORS_SWITCH/,
  taNames.map(([, TYPE]) => `  case ${TYPE}_TYPED_ARRAY_CONSTRUCTOR_TYPE:`).join("\n"),
);

// 2) 展开 #ifdef V8_TEMPORAL_SUPPORT (默认关 → 剔除)
switchBody = switchBody.replace(/#ifdef V8_TEMPORAL_SUPPORT[\s\S]*?#endif\s*\/\/ V8_TEMPORAL_SUPPORT/g, "");
// 3) IF_INTL / V8_INTL_SUPPORT (默认开 → 保留内容)
switchBody = switchBody.replace(/#ifdef V8_INTL_SUPPORT\n/g, "").replace(/#endif\s*\/\/ V8_INTL_SUPPORT/g, "");
// 4) IF_WASM / V8_ENABLE_WEBASSEMBLY (默认开 → 保留内容)
switchBody = switchBody.replace(/#if V8_ENABLE_WEBASSEMBLY\n/g, "").replace(/#endif\s*\/\/ V8_ENABLE_WEBASSEMBLY/g, "");

// 5) 展开 SIMPLE_HEAP_OBJECT_LIST2(CASE) / CONCRETE_TRUSTED_OBJECT_TYPE_LIST2(CASE)
const listMacros = readV8("src/objects/object-list-macros.h");
function generatorEntries(def) {
  const flat = listMacros.match(def)[1].replace(/\\\n/g, " ");
  const out = [...flat.matchAll(/APPLY\(V,\s*(\w+),\s*(\w+)\)/g)].map((m) => [m[1], m[2]]);
  for (const m of flat.matchAll(/IF_WASM\(APPLY, V,\s*(\w+),\s*(\w+)\)/g)) out.push([m[1], m[2]]);
  return out;
}
const simple2 = generatorEntries(/#define SIMPLE_HEAP_OBJECT_LIST_GENERATOR\(APPLY, V\)([\s\S]*?)\n\n/);
const trusted2 = generatorEntries(/#define CONCRETE_TRUSTED_OBJECT_LIST_GENERATOR\(APPLY, V\)([\s\S]*?)\n\n/);
const byVariant = new Map();
const macroTail = [...simple2, ...trusted2]
  .map(([Name, NAME]) => {
    const variant = resolve(NAME);
    // 尾部宏块并入覆盖账 (Rust 穷举 match 同样包含这些臂)
    if (!byVariant.has(variant)) byVariant.set(variant, new Set());
    byVariant.get(variant).add(`Visit${Name}`);
    return `      InstanceType::${variant} => VisitorId::Visit${Name},`;
  })
  .join("\n");
switchBody = switchBody.replace(
  /#define CASE\(TypeCamelCase, TYPE_UPPER_CASE\)[\s\S]*?#undef CASE/,
  macroTail,
);

// 6) 提取 case 组 → return 映射
const lines = switchBody.split("\n");
const groups = []; // { labels: [TYPE_UPPER], visitor: "VisitFoo" | special }
let cur = null;
let specials = 0;
for (const raw of lines) {
  const line = raw.trim().replace(/^\[\[unlikely\]]\s*/, "");
  const cm = line.match(/^case (\w+):(?: \{)?$/);
  if (cm) {
    if (cm[1] === "JS_PROMISE_TYPE") {
      // 特殊块: 依赖 embedder 字段数, 不产出直接映射
      specials += 1;
      continue;
    }
    if (!cur) cur = { labels: [], visitor: null };
    cur.labels.push(cm[1]);
    continue;
  }
  const rm = line.match(/^return kVisit(\w+);$/);
  if (rm && cur) {
    cur.visitor = `Visit${rm[1]}`;
    groups.push(cur);
    cur = null;
    continue;
  }
  // 特殊块: JS_PROMISp / CHECK 语句体 (case ... : { 语句 }) — 单独登记
  if (/^case JS_PROMISE_TYPE: \{/.test(line)) {
    specials += 1;
    continue;
  }
  if (/^CHECK_EQ\(0, JSObject::GetEmbedderFieldCount\(map\)\);/.test(line)) {
    continue; // 空间断言剔除 (语义缺口登记)
  }
  if (/has_raw_data_fields|kVisitJSObjectFast|kVisitJSObject;/.test(line) && cur === null && !/^case/.test(line)) {
    continue; // JS_PROMISE 块内语句 (specials 计数承接)
  }
  if (line === "{" || line === "}") continue;
  if (/^\/\//.test(line) || line === "") continue;
  if (/^\[\[unlikely\]]/.test(line)) continue;
  if (/^#/.test(line)) continue;
}
if (specials !== 1) throw new Error(`JS_PROMISE 特殊块计数 ${specials} != 1`);
if (cur) throw new Error("case 组解析残留未闭合");

// 解析校验 + 变体聚合 (byVariant 已在宏尾部展开时建账)
const arms = [];
for (const g of groups) {
  const vis = g.visitor;
  for (const label of g.labels) {
    if (label === "JS_PROMISE_TYPE") throw new Error("JS_PROMISE 须走特殊分支");
    const variant = resolve(label);
    if (!byVariant.has(variant)) byVariant.set(variant, new Set());
    const set = byVariant.get(variant);
    set.add(vis);
    if (set.size > 1) throw new Error(`${variant} 映射到多个 visitor: ${[...set].join(",")}`);
  }
}
// 对数锚点: 关键映射
const expect = {
  HeapNumber: "VisitHeapNumber",
  Map: "VisitMap",
  Oddball: "VisitOddball",
  Symbol: "VisitSymbol",
  Filler: "VisitFiller",
  ByteArray: "VisitByteArray",
  FixedArray: "VisitFixedArray",
  Code: "VisitCode",
};
for (const [v, w] of Object.entries(expect)) {
  if (byVariant.get(v) !== w && ![...byVariant.get(v) ?? []].includes(w)) {
    throw new Error(`对数失败: ${v} → ${[...(byVariant.get(v) ?? [])].join(",")} (期望含 ${w})`);
  }
}

// 未映射变体 (编译穷举需要显式分支) — 应只有 JS_PROMISE;
// string 变体 (< FIRST_NONSTRING_TYPE) 走表示法分支, 不入大表。
const variantValue = new Map([...instanceTypeRs.matchAll(/^  (\w+) = (\d+),$/gm)].map((m) => [m[1], Number(m[2])]));
const firstNonstring = variantValue.get(aliases.get("FIRST_NONSTRING_TYPE"));
if (!firstNonstring) throw new Error("FIRST_NONSTRING_TYPE 别名解析失败");
// 抽象/占位实例类型: 运行时不可能成为 map 的 instance_type (原版走尾部
// FATAL) — 穷举 match 需显式臂, 落 Err 登记。
const PLACEHOLDER_TYPES = [
  "WasmFunctionData",
  "BaseNameDictionary",
  "Dictionary",
  "ObjectHashTableBase",
  "OrderedHashTable",
  "SmallOrderedHashTableImpl",
  "JSLastDummyApiObject",
  "JSObjectWithEmbedderSlots",
  "OrderedHashTableIterator",
];
const unmapped = [...variants].filter((v) => variantValue.get(v) >= firstNonstring && !byVariant.has(v));
const expectUnmapped = [...PLACEHOLDER_TYPES, "JSPromise"].sort();
const unmappedSorted = unmapped.slice().sort();
if (unmappedSorted.join() !== expectUnmapped.join()) {
  throw new Error(`未映射变体集合失配: ${unmappedSorted.join(", ")}`);
}

const matchArms = [...byVariant.entries()]
  .sort((a, b) => a[0].localeCompare(b[0]))
  .map(([variant, vis]) => `      InstanceType::${variant} => Ok(VisitorId::${[...vis][0]}),`)
  .join("\n");

// —— visitor 名单从 map.rs 反解 (与 map.h.js 同源, 单一事实源) ——
const mapRs = readFileSync(target, "utf8");
const enumM = mapRs.match(/pub enum VisitorId \{([\s\S]*?)\n\}/);
if (!enumM) throw new Error("map.rs 缺 VisitorId 枚举 (先跑 map.h.js)");
const allVisitorNames = [...enumM[1].matchAll(/^  Visit(\w+),$/gm)]
  .map((x) => x[1])
  .filter((n) => n !== "orIdCount"); // 排除哨兵 VisitorIdCount 的前缀误匹配
if (allVisitorNames.length !== 144) throw new Error(`VisitorId 名单 ${allVisitorNames.length} != 144`);

// string 变体 (< FIRST_NONSTRING_TYPE) 走表示法分支 — 穷举 match 需要显式臂
const stringVariants = [...variants]
  .filter((v) => variantValue.get(v) < firstNonstring)
  .sort((a, b) => a.localeCompare(b));
const stringArm = stringVariants.map((v) => `InstanceType::${v}`).join(" | ");

const semantics = `${banner("src/objects/map.cc.js", "src/objects/map.cc (GetVisitorId + ToString)", "转写/跳过区段登记见脚本头注释")}
use crate::error::Result; // Error 由 map.h 头部导入承接
use crate::instance_type::{
  is_shortcut_candidate, K_CONS_STRING_TAG, K_EXTERNAL_STRING_TAG, K_ONE_BYTE_STRING_TAG,
  K_SEQ_STRING_TAG, K_SLICED_STRING_TAG, K_STRING_ENCODING_MASK, K_STRING_REPRESENTATION_MASK,
  K_THIN_STRING_TAG,
};
use crate::instance_type_checker as itc;

// ── Map::GetVisitorId (map.cc:69-559, 大表机械展开) ───────────────────────
// 逐 case 对齐原版; JS_PROMISE 分支依赖 JSObject::GetEmbedderFieldCount
// (JSObject 头部布局) 落 Err, 归 v8-objects-js-core 承接。
#[inline]
pub fn get_visitor_id(map: &Map) -> Result<VisitorId> {
  let ty = map.instance_type();

  // JS API 对象前置分支 (map.cc:102-104; JS_API_* 判别值均 >= FIRST_NONSTRING_TYPE)
  if itc::is_js_api_object(ty) {
    return Ok(VisitorId::VisitJSApiObject);
  }

  let instance_type = ty as u16;
  match ty {
      // string 表示法分派 (map.cc:74-100; string 族先于大表, 原版 if 前置)
      ${stringArm} => {
        let rep = (instance_type as u32) & K_STRING_REPRESENTATION_MASK;
        let one_byte = (instance_type as u32 & K_STRING_ENCODING_MASK) == K_ONE_BYTE_STRING_TAG;
        match rep {
          K_SEQ_STRING_TAG => Ok(if one_byte {
            VisitorId::VisitSeqOneByteString
          } else {
            VisitorId::VisitSeqTwoByteString
          }),
          K_CONS_STRING_TAG => Ok(if is_shortcut_candidate(instance_type) {
            VisitorId::VisitShortcutCandidate
          } else {
            VisitorId::VisitConsString
          }),
          K_SLICED_STRING_TAG => Ok(VisitorId::VisitSlicedString),
          K_EXTERNAL_STRING_TAG => Ok(VisitorId::VisitExternalString),
          K_THIN_STRING_TAG => Ok(VisitorId::VisitThinString),
          // 表示法 tag 仅定义 0/1/2/3/5 (4/6/7 未用); 原版 UNREACHABLE → 禁 panic 落 Err。
          _ => Err(Error::Unsupported("GetVisitorId: 非法 string 表示法 tag")),
        }
      }
${matchArms}
      // JS_PROMISE (map.cc:297-301): COMPRESS_POINTERS_BOOL && embedder 字段数 > 0
      // → VisitJSObject, 否则 VisitJSObjectFast — 依赖 JSObject 头部布局。
      InstanceType::JSPromise => {
        Err(Error::Unsupported("GetVisitorId: JS_PROMISE embedder 字段判定归 v8-objects-js-core"))
      }
      // 抽象/占位实例类型: 运行时不可能成为 map 的 instance_type
      // (原版尾部 FATAL → 禁 panic 落 Err)。
      ${PLACEHOLDER_TYPES.map((v) => `InstanceType::${v}`).join(" | ")} => {
        Err(Error::Unsupported("GetVisitorId: 抽象/占位实例类型"))
      }
  }
}

// ── ToString(VisitorId) (map.cc:2742-2757) ───────────────────────────────
/// 访客 id 名 (哨兵变体返回哨兵名)。
#[inline]
#[must_use]
pub fn visitor_id_to_str(visitor_id: VisitorId) -> Option<&'static str> {
  match visitor_id {
${allVisitorNames.map((n) => `    VisitorId::Visit${n} => Some("${n}"),`).join("\n")}
    VisitorId::DataOnlyVisitorIdCount => Some("DataOnlyVisitorIdCount"),
    VisitorId::VisitorIdCount => Some("VisitorIdCount"),
  }
}
`;

// 区段标记替换 (幂等)
const BEGIN = "// @cc-semantics-begin";
const END = "// @cc-semantics-end";
const bi = mapRs.indexOf(BEGIN);
const ei = mapRs.indexOf(END);
if (bi < 0 || ei < 0) throw new Error("map.rs 缺少 @cc-semantics 区段标记 (先跑 map.h.js)");
const next = mapRs.slice(0, bi + BEGIN.length) + "\n" + semantics + "\n" + mapRs.slice(ei);
writeFileSync(target, next);
console.log(
  `map.rs @cc-semantics 区段已注入: GetVisitorId ${byVariant.size} 变体映射 ` +
    `(锚点 8/8 过, 未映射=JSPromise 1 个登记推迟) + visitor_id_to_str ${allVisitorNames.length} 名`,
);
