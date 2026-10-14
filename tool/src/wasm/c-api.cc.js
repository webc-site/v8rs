#!/usr/bin/env bun
// tool/src/wasm/c-api.cc.js (族合一: c-api.cc + c-api.h)
// 大 .cc 规则面: 外部种类映射、名字缓冲上限、计数器表上限、栈捕获深度、
// vec 布局兼容断言、托管类型 id; 另出 WASM_EXPORT 方法签名骨架。
// 头侧枚举在 c_api.rs.skel (c-api.h.js)。

import { readV8, writeSrc, banner } from "../../lib.js";

const src = readV8("src/wasm/c-api.cc");

// ---------- 1. 外部种类映射 (6 种: exact-function 归并到 function) ----------
const kindBlock = src.match(/own<ExternType> GetImportExportType[\s\S]*?switch \(kind\) \{([\s\S]*?)\n  \}/);
if (!kindBlock) throw new Error("外部种类 switch 缺失");
const kinds = [...kindBlock[1].matchAll(/case i::wasm::(kExternal\w+):/g)].map((m) => m[1]);
if (kinds.join(",") !== "kExternalFunction,kExternalExactFunction,kExternalTable,kExternalMemory,kExternalGlobal,kExternalTag") {
  throw new Error(`外部种类序异常: ${kinds.join(",")}`);
}

// ---------- 2. 名字缓冲上限 ----------
const maxName = src.match(/static const int kMaxNameSize = (\d+);/);
if (!maxName || maxName[1] !== "64") throw new Error("kMaxNameSize != 64");

// ---------- 3. 计数器表上限 ----------
const maxCounters = src.match(/static const unsigned kMaxCounters = (\d+);/);
if (!maxCounters || maxCounters[1] !== "512") throw new Error("kMaxCounters != 512");
if (!src.includes("if (counters_in_use_ == kMaxCounters) return nullptr;")) {
  throw new Error("计数器满拒绝缺失");
}

// ---------- 4. 名字盒/值盒尺寸 (反序列化) ----------
const nameBox = src.match(/constexpr int kNameBoxSize = (\d+);/);
if (!nameBox || nameBox[1] !== "64") throw new Error("kNameBoxSize != 64");
const valueBox = src.match(/constexpr int kValueBoxSize = (\d+);/);
if (!valueBox || valueBox[1] !== "13") throw new Error("kValueBoxSize != 13");

// ---------- 5. 栈捕获深度 ----------
const stackLimit = src.match(/constexpr int kStackLimit = (\d+);/);
if (!stackLimit || stackLimit[1] !== "10") throw new Error("kStackLimit != 10");

// ---------- 6. 托管类型 id ----------
const typeIds = [...src.matchAll(/static constexpr i::ManagedTypeId kTypeID =\s*(i::ManagedTypeId::\w+);/g)].map((m) => m[1]);
if (typeIds.length !== 2) throw new Error(`托管类型 id ${typeIds.length} != 2`);

// ---------- 7. 表最大长截断到 kMaxUInt32 ----------
if (!src.includes("// truncate the actual declared maximum to kMaxUint32.")) {
  throw new Error("表最大长截断注释缺失");
}

// ---------- 8. vec 布局兼容断言 (除 GC_MOLE 外生效) ----------
if (!src.includes('static_assert(sizeof(wasm_##name##_vec_t) == sizeof(vec<Name>),')) {
  throw new Error("vec 布局断言缺失");
}
if (!src.includes('#ifdef V8_GC_MOLE')) throw new Error("GC_MOLE 豁免缺失");

// ---------- 9. WASM_EXPORT 方法台账 ----------
const exports = [...src.matchAll(/^WASM_EXPORT auto ([\w:]+)\(/gm)].map((m) => m[1]);
if (exports.length < 60) throw new Error(`WASM_EXPORT 台账 ${exports.length} < 60`);

console.log(`c-api: 外部种类=${kinds.length} 托管id=${typeIds.length} WASM_EXPORT=${exports.length}`);

// ---------- 生成 Rust ----------
let skel = "";
const byOwner = new Map();
for (const e of exports) {
  const [owner, method] = e.split("::");
  if (!byOwner.has(owner)) byOwner.set(owner, []);
  byOwner.get(owner).push(method);
}
for (const [owner, methods] of byOwner) {
  skel += `\n/// ${owner} WASM_EXPORT 方法骨架 (${methods.length} 个)\n`;
  for (const m of methods) {
    skel += `/// v8: src/wasm/c-api.cc ${owner}::${m}\n`;
  }
  skel += `pub fn ${owner.toLowerCase()}_api_methods() {\n    todo!("就地修订: ${owner} ${methods.length} 个 WASM_EXPORT")\n}\n`;
}

const rust = `${banner(
  "src/wasm/c-api.cc.js",
  "src/wasm/c-api.{h,cc}",
  "C API 外部种类映射与缓冲上限 + WASM_EXPORT 方法签名骨架 (枚举在 c_api.rs.skel)",
)}
//! wasm-c-api 兼容层规则面。

/// 外部种类映射: exact-function 归并 function, 共 6 种外部形态
/// v8: src/wasm/c-api.cc GetImportExportType
#[rustfmt::skip]
pub static EXTERNAL_KIND_MAP: &[(&str, &str)] = &[
    ("kExternalFunction", "FuncType"),
    ("kExternalExactFunction", "FuncType (归并 function)"),
    ("kExternalTable", "TableType"),
    ("kExternalMemory", "MemoryType"),
    ("kExternalGlobal", "GlobalType"),
    ("kExternalTag", "TagType"),
];

/// 名字复制缓冲上限 (含 NUL)
/// v8: src/wasm/c-api.cc kMaxNameSize
pub const K_MAX_NAME_SIZE: i32 = ${maxName[1]};
/// 计数器表上限 (满后拒绝新计数器)
/// v8: src/wasm/c-api.cc kMaxCounters
pub const K_MAX_COUNTERS: u32 = ${maxCounters};
/// 反序列化名字盒尺寸
/// v8: src/wasm/c-api.cc kNameBoxSize
pub const K_NAME_BOX_SIZE: i32 = ${nameBox[1]};
/// 反序列化值盒尺寸
/// v8: src/wasm/c-api.cc kValueBoxSize
pub const K_VALUE_BOX_SIZE: i32 = ${valueBox[1]};

/// Store 构造时开启未捕获异常栈捕获, 深度 10, Overview 模式 (为 trap 生成栈)
/// v8: src/wasm/c-api.cc Store::make
pub const K_STACK_LIMIT: i32 = ${stackLimit[1]};
pub const STACK_CAPTURE_MODE: &str = "kOverview";

/// 托管类型 id (HostData / FuncData)
/// v8: src/wasm/c-api.cc kTypeID
#[rustfmt::skip]
pub static MANAGED_TYPE_IDS: &[&str] = &[
${typeIds.map((t) => `    "${t}",\n`).join("")}];

/// 表最大长对外截断到 kMaxUInt32
/// v8: src/wasm/c-api.cc wasm_table_type_maximum 或等价处
pub const TABLE_MAX_TRUNCATE: &str = "maximum_length_u64().value_or(kMaxUInt32) 钳 kMaxUInt32";

/// vec<T> 与 wasm_xxx_vec_t 布局兼容断言 (V8_GC_MOLE 构建下豁免)
/// v8: src/wasm/c-api.cc ASSERT_VEC_BASE_SIZE
pub const VEC_LAYOUT_ASSERT: &str = "sizeof(wasm_##name##_vec_t) == sizeof(vec<Name>) (GC_MOLE 除外)";

// —— 提取不变式断言 ——
const _: () = {
    assert_eq!(EXTERNAL_KIND_MAP.len(), 6);
    assert_eq!(K_MAX_NAME_SIZE, 64);
    assert_eq!(K_MAX_COUNTERS, 512);
    assert_eq!(K_NAME_BOX_SIZE, 64);
    assert_eq!(K_VALUE_BOX_SIZE, 13);
    assert_eq!(K_STACK_LIMIT, 10);
    assert_eq!(MANAGED_TYPE_IDS.len(), 2);
};
${skel}`;

const p = writeSrc("v8-wasm-core", "c_api_cc.rs", rust);
console.log(`written ${p}`);
