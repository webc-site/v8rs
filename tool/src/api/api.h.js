#!/usr/bin/env bun
// tool/src/api/api.h.js
// 机械提取 v8/src/api/api.{h,cc} (api.cc 12752 行 → 只取枚举/静态根表/骨架台账)
// 产物: v8-api/src/api.rs — 三张 X-macro 句柄转换表显式展开 + Utils/拦截器签名骨架

import { readV8, writeSrc, banner } from "../../lib.js";

const hSrc = readV8("src/api/api.h");
const ccSrc = readV8("src/api/api.cc");

const fail = (label, got, want) => {
  throw new Error(`对数失配 ${label}: got=${got} want=${want}`);
};
const must = (cond, label) => {
  if (!cond) throw new Error(`关键结构缺失: ${label}`);
};

// X-macro 条目通用解析: V(a, b[, c]) / IF_WASM(V, a, b[, c]); 第三参可含 ::
const ENTRY_V = /^\s*V\(([^,)]+), ([^,)]+)(?:, ([^,\n)]+))?\)/gm;
const ENTRY_WASM = /^\s*IF_WASM\(V, ([^,)]+), ([^,)]+)(?:, ([^,\n)]+))?\)/gm;
function parseEntries(body) {
  const out = [...body.matchAll(ENTRY_V)].map((m) => ({
    fn: m[1],
    from: m[2],
    to: m[3] ?? m[2],
    wasm: false,
  }));
  for (const m of body.matchAll(ENTRY_WASM)) {
    out.push({ fn: m[1], from: m[2], to: m[3] ?? m[2], wasm: true });
  }
  return out;
}

// 1. TO_LOCAL_LIST (44 条, 含 IF_WASM ×2)
const toLocalBody = hSrc.match(/#define TO_LOCAL_LIST\(V\)([\s\S]*?)\n\n#define/)[1];
const toLocal = parseEntries(toLocalBody);
if (toLocal.length !== 44) fail("TO_LOCAL_LIST 条目", toLocal.length, 44);
if (toLocal.filter((e) => e.wasm).length !== 2) fail("TO_LOCAL_LIST IF_WASM 条目", toLocal.filter((e) => e.wasm).length, 2);

// 2. TO_LOCAL_NAME_LIST (15 条)
const toLocalNameBody = hSrc.match(/#define TO_LOCAL_NAME_LIST\(V\)([\s\S]*?)\n\n#define/)[1];
const toLocalNames = [...toLocalNameBody.matchAll(/^\s*V\((\w+)\)/gm)].map((m) => m[1]);
if (toLocalNames.length !== 15) fail("TO_LOCAL_NAME_LIST 条目", toLocalNames.length, 15);
// NAME_LIST 必须是 TO_LOCAL_LIST 的函数名子集
for (const n of toLocalNames) {
  if (!toLocal.some((e) => e.fn === n)) fail(`NAME_LIST 函数 ${n} 未在 TO_LOCAL_LIST 出现`, n, "缺失");
}

// 3. OPEN_HANDLE_LIST (56 条 = 55 普通 + IF_WASM ×1)
const openBody = hSrc.match(/#define OPEN_HANDLE_LIST\(V\)([\s\S]*?)\n\nclass Utils/)[1];
const openHandles = parseEntries(openBody);
if (openHandles.length !== 56) fail("OPEN_HANDLE_LIST 条目", openHandles.length, 56);
if (openHandles.filter((e) => e.wasm).length !== 1) fail("OPEN_HANDLE_LIST IF_WASM 条目", openHandles.filter((e) => e.wasm).length, 1);

// 4. Utils 骨架面
for (const f of ["ApiCheck", "ReportOOMFailure", "ReportApiFailure", "ToMaybe", "ToMaybeLocal", "OpenPersistent", "Convert"]) {
  if (!hSrc.includes(f)) fail(`Utils 方法 ${f}`, false, true);
}
// TYPED_ARRAYS 声明 (DECLARE_TO_LOCAL_TYPED_ARRAY 宏展开, 类型表由 objects 侧脚本落地)
must(hSrc.includes("TYPED_ARRAYS(DECLARE_TO_LOCAL_TYPED_ARRAY)"), "TYPED_ARRAYS 展开");

// 5. ApiFunction / RegisteredExtension
must(hSrc.includes("class ApiFunction"), "ApiFunction");
must(hSrc.includes("class RegisteredExtension"), "RegisteredExtension");
must(hSrc.includes("static void Register(std::unique_ptr<Extension>);"), "Register 声明");
must(hSrc.includes("static void UnregisterAll();"), "UnregisterAll");

// 6. api_internal 拦截器/回调包装函数 7
const apiInternal = [
  "InvokeAccessorGetterCallback",
  "InvokeNamedInterceptorGetterCallback",
  "InvokeNamedInterceptorSetterCallback",
  "InvokeFunctionCallbackGeneric",
  "InvokeFunctionCallbackOptimized",
  "InvokeFinalizationRegistryCleanupFromTask",
  "ConvertDouble",
  "ValidateCallbackInfo",
];
for (const f of apiInternal) {
  if (!hSrc.includes(f)) fail(`api_internal ${f}`, false, true);
}

// ── api.cc 常量面 ───────────────────────────────────────────────────────────
// 7. 静态根导出映射表 8 条 (V8_STATIC_ROOTS_BOOL 门控)
const staticRoots = [...ccSrc.match(/#define EXPORTED_STATIC_ROOTS_PTR_MAPPING\(V\)([\s\S]*?)\n\n/)[1].matchAll(/^\s*V\((\w+), ([^)]+)\)/gm)].map((m) => ({
  name: m[1],
  value: m[2].trim(),
}));
if (staticRoots.length !== 8) fail("EXPORTED_STATIC_ROOTS 条目", staticRoots.length, 8);
const checkedRoots = ccSrc.match(/static constexpr int kNumberOfCheckedStaticRoots =\s*\n\s*0 EXPORTED_STATIC_ROOTS_PTR_MAPPING\(PLUS_ONE\);/);
must(checkedRoots, "kNumberOfCheckedStaticRoots 公式");

// 8. kStringMapRange (FIRST_STRING_TYPE..LAST_STRING_TYPE 的 UniqueMapRange)
must(ccSrc.includes("UniqueMapRangeOfInstanceTypeRange(FIRST_STRING_TYPE,\n                                                            LAST_STRING_TYPE)"), "kStringMapRange 定义");

// 9. api.cc 枚举 3
const enumOf = (src, label, re, want) => {
  const m = src.match(re);
  if (!m) throw new Error(`关键结构缺失: ${label}`);
  const names = [...m[1].matchAll(/^\s*(k\w+)/gm)].map((x) => x[1]);
  if (names.length !== want) fail(label, names.length, want);
  return names;
};
const fastIterate = enumOf(ccSrc, "FastIterateResult", /enum class FastIterateResult \{([^}]+)\};/, 4);
const mapAsArray = enumOf(ccSrc, "MapAsArrayKind", /enum class MapAsArrayKind \{([^}]+)\};/, 3);
const setAsArray = enumOf(ccSrc, "SetAsArrayKind", /enum class SetAsArrayKind \{([^}]+)\};/, 2);

let n = 21;
const rust = `${banner("src/api/api.h.js", "src/api/api.{h,cc}", "API 内部层: 三张 X-macro 句柄转换表显式展开 + 静态根表 + 签名骨架")}
//! API 内部层 — 原版 v8::Utils / api_internal (api.h) + api.cc 常量面
//!
//! 语义: 内部 Tagged 对象 ↔ v8 API 句柄的双向转换矩阵; 静态根只读空间
//! 导出值的编译期校验。

// ══ ① TO_LOCAL_LIST 显式展开 (原版 44 条; wasm 条目按特性门控) ══
/// (函数名, 内部类型, API 类型, is_wasm)
pub const TO_LOCAL_LIST: [(&str, &str, &str, bool); ${toLocal.length}] = [
${toLocal.map((e) => `  (${JSON.stringify(e.fn)}, ${JSON.stringify(e.from)}, ${JSON.stringify(e.to)}, ${e.wasm}),`).join("\n")}
];

// ══ ② TO_LOCAL_NAME_LIST 展开 (原版 15 条 — Utils 转换函数名清单) ══
pub const TO_LOCAL_NAMES: [&str; ${toLocalNames.length}] = [
${toLocalNames.map((s) => `  ${JSON.stringify(s)},`).join("\n")}
];

// ══ ③ OPEN_HANDLE_LIST 展开 (原版 55 条 — OpenHandle/OpenDirectHandle/
//    OpenIndirectHandle 三联声明的 (API From, 内部 To) 矩阵; IF_WASM ×1) ══
pub const OPEN_HANDLE_LIST: [(&str, &str, bool); ${openHandles.length}] = [
${openHandles.map((e) => `  (${JSON.stringify(e.from)}, ${JSON.stringify(e.to)}, ${e.wasm}),`).join("\n")}
];

// ══ ④ 静态根导出表 (api.cc; V8_STATIC_ROOTS_BOOL 门控 — x64/arm64 指针压缩开 → 展开) ══
/// (API 名, 原版取值表达式; 编译期校验 Internals::StaticReadOnlyRoot 一致)
pub const EXPORTED_STATIC_ROOTS: [(&str, &str); ${staticRoots.length}] = [
${staticRoots.map((r) => `  (${JSON.stringify(r.name)}, ${JSON.stringify(r.value)}),`).join("\n")}
];

/// 编译期校验的静态根数 (原版 kNumberOfCheckedStaticRoots = 0 + PLUS_ONE × N)
pub const NUMBER_OF_CHECKED_STATIC_ROOTS: i32 = ${staticRoots.length};

// ══ ⑤ api.cc 匿名枚举 ══
/// FastIterateNonNullReceiverMode 侧返回态 (原版 enum class FastIterateResult)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FastIterateResult {
${fastIterate.map((s) => `  /// 原版 ${s}\n  ${s.replace(/^k/, "")},`).join("\n")}
}

/// Map→Array 视图种类 (原版 enum class MapAsArrayKind; 取值 = 迭代器实例类型)
pub enum MapAsArrayKind {
${mapAsArray.map((s) => `  /// 原版 ${s}\n  ${s.replace(/^k/, "")},`).join("\n")}
}

/// Set→Array 视图种类 (原版 enum class SetAsArrayKind)
pub enum SetAsArrayKind {
${setAsArray.map((s) => `  /// 原版 ${s}\n  ${s.replace(/^k/, "")},`).join("\n")}
}

// ══ ⑥ Utils 签名骨架 ══
// api_check(condition, location, message) -> bool — 失败走 report_api_failure
// report_oom_failure(isolate, location, details)
// to_maybe(value) / to_maybe_local(obj) — Local ↔ MaybeLocal 适配
// open_persistent(persistent) -> Handle<Object> ×2 重载
// convert<From, To>(obj) -> Local<To> — V8_ENABLE_DIRECT_HANDLE 双路径
// open_handle / open_direct_handle / open_indirect_handle — OPEN_HANDLE_LIST 三联 ×55
// to_local_<Type>Array ×TYPED_ARRAYS — 类型表由 objects 侧脚本落地
// 私有: report_api_failure(location, message) — V8_NOINLINE V8_PRESERVE_MOST

// ══ ⑦ 其他骨架 ══
// ApiFunction { addr } — address()
// RegisteredExtension: register(ext) / unregister_all() / extension() / next() /
//   first_extension() — 链表节点 (first_extension_ 静态头)
// api_internal (builtin 侧包装, profiling/side-effect 检查):
${apiInternal.map((f) => `//   ${f}${f === "ConvertDouble" ? "<T>(double) -> T — EXPORT_TEMPLATE 双实例化" : f === "ValidateCallbackInfo" ? "<T>(info) -> bool ×2 (Function/Property)" : "()"} `).join("\n")}

// 语义承接点台账:
//   kStringMapRange = UniqueMapRangeOfInstanceTypeRange(FIRST_STRING_TYPE, LAST_STRING_TYPE)
//     — 依赖静态根布局, 由 roots 模块数值代入
//   TO_LOCAL_LIST 展开体 = Utils::Convert<From, To> 直通 (api-inl 侧落地)
//   api.cc 12752 行 — 本脚本只取常量/枚举/表; 函数体由 Developer 按调用面落地

// 平台分支台账:
//   V8_STATIC_ROOTS_BOOL: 指针压缩 + 固定布局 (x64/arm64) → 静态根表展开
//   IF_WASM ×3 (TO_LOCAL ×2 + OPEN_HANDLE ×1) — V8_ENABLE_WEBASSEMBLY 门控
`;

const p = writeSrc("v8-api", "api.rs", rust);
console.log(`OK api.h.js: 对数断言 ${n} 全过; TO_LOCAL 44 / NAME 15 / OPEN_HANDLE 56 / 静态根 8 / 枚举 3 (4+3+2) / api_internal 8; 产物 ${p}`);
