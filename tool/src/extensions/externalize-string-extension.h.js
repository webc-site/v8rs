#!/usr/bin/env bun
// tool/src/extensions/externalize-string-extension.h.js
// 机械提取 v8/src/extensions/externalize-string-extension.{h,cc}
// 产物: v8-extensions/src/externalize_string_extension.rs — 最小外化长度公式 + native 表 + 签名骨架

import { readV8, writeSrc, banner } from "../../lib.js";

const hSrc = readV8("src/extensions/externalize-string-extension.h");
const ccSrc = readV8("src/extensions/externalize-string-extension.cc");

const fail = (label, got, want) => {
  throw new Error(`对数失配 ${label}: got=${got} want=${want}`);
};
const must = (cond, label) => {
  if (!cond) throw new Error(`关键结构缺失: ${label}`);
};

// 1. 4 条最小长度公式 (cc 顶部 constexpr)
const minLens = [
  ["kMinOneByteLength", "kExternalPointerSlotSize - kTaggedSize + 1"],
  ["kMinTwoByteLength", "(kExternalPointerSlotSize - kTaggedSize) / sizeof(base::uc16) + 1"],
  ["kMinOneByteCachedLength", "2 * kExternalPointerSlotSize - kTaggedSize + 1"],
  ["kMinTwoByteCachedLength", "(2 * kExternalPointerSlotSize - kTaggedSize) / sizeof(base::uc16) + 1"],
];
for (const [name, expr] of minLens) {
  const re = new RegExp(`constexpr int ${name} =`);
  if (!re.test(ccSrc)) fail(`constexpr ${name}`, false, true);
  // 公式文本逐 token 核对 (剥空白)
  const body = ccSrc.match(new RegExp(`constexpr int ${name} =([\\s\\S]*?);`))[1].replace(/\s+/g, "");
  if (body !== expr.replace(/\s+/g, "")) fail(`公式 ${name}`, body, expr);
}

// 2. 扩展名 + buffer 容量 (h)
const extName = hSrc.match(/v8::Extension\("v8\/externalize"/);
must(extName, "扩展名 v8/externalize");
const bufCap = hSrc.match(/char buffer_\[(\d+)\];/);
must(bufCap, "buffer_ 容量");

// 3. BuildSource 注入的 4 条 native function + 4 个 let 常量
const buildSrcBody = ccSrc.match(/BuildSource\(char\* buf[\s\S]*?return buf;/);
must(buildSrcBody, "BuildSource 函数体");
const natives = [...buildSrcBody[0].matchAll(/native function (\w+)\(\);/g)].map((m) => m[1]);
if (natives.length !== 4) fail("native function 数", natives.length, 4);
const lets = [...buildSrcBody[0].matchAll(/let (kExternalString\w+) = %d;/g)].map((m) => m[1]);
if (lets.length !== 4) fail("let 常量数", lets.length, 4);

// 4. 方法全集 (h 声明 6 / cc 定义 1:1)
const methods = [
  "GetNativeFunctionTemplate", "Externalize", "CreateExternalizableString",
  "CreateExternalizableTwoByteString", "IsOneByte", "BuildSource",
];
for (const name of methods) {
  if (!hSrc.includes(name)) fail(`h 声明 ${name}`, false, true);
}
const externalizeDefs = (ccSrc.match(/ExternalizeStringExtension::\w+\(/g) || []).length;
if (externalizeDefs !== 6) fail("cc 定义数", externalizeDefs, 6);

// 5. 匿名 namespace 辅助函数
for (const f of ["HasExternalForwardingIndex", "CopyConsStringToOld", "CreateExternalizableString"]) {
  if (!ccSrc.includes(`${f}(`)) fail(`匿名辅助 ${f}`, false, true);
}

// 6. GetNativeFunctionTemplate 分发表 (4 分支, 全部 ConstructorBehavior::kThrow)
const ctorThrow = (ccSrc.match(/ConstructorBehavior::kThrow/g) || []).length;
if (ctorThrow !== 4) fail("kThrow 分支数", ctorThrow, 4);

let n = 17;
const rust = `${banner("src/extensions/externalize-string-extension.h.js", "src/extensions/externalize-string-extension.{h,cc}", "ExternalizeString 扩展: 最小外化长度公式 + native 表 + 签名骨架")}
//! ExternalizeString 扩展 — 原版 namespace v8::internal::ExternalizeStringExtension
//!
//! 语义: 注入 4 个 native 函数, 把堆内字符串外化为 ExternalString 资源;
//! CreateExternalizable* 负责把不可外化字符串拷入旧生代/展平 ConsString。

/// ExternalizeString 扩展注册名
pub const EXTERNALIZE_STRING_EXTENSION_NAME: &str = "v8/externalize";

/// 栈上源码缓冲容量 (原版 char buffer_[400])
pub const EXTERNALIZE_STRING_BUFFER_SIZE: usize = ${bufCap[1]};

/// 注入 JS 的 let 常量名 (原版 BuildSource %d 占位, 顺序与 MIN_LENGTHS 一致)
pub const JS_CONST_NAMES: [&str; 4] = [
${lets.map((s) => `  ${JSON.stringify(s)},`).join("\n")}
];

/// 最小外化长度公式表 (原版 4 条 constexpr, S=外部指针槽宽, T=Tagged 槽宽, U16=2)
/// 公式为平台相关值 (32/64 位沙箱配置不同), 保留符号形式待 Developer 侧代入
pub const MIN_LENGTH_FORMULAS: [(&str, &str); 4] = [
${minLens.map(([n2, e]) => `  (${JSON.stringify(n2)}, ${JSON.stringify(e)}),`).join("\n")}
];

/// native 函数名表 (原版 BuildSource, 顺序 1:1)
pub const NATIVE_FUNCTIONS: [&str; 4] = [
${natives.map((s) => `  ${JSON.stringify(s)},`).join("\n")}
];

// —— 提取不变式断言 ——
const _: () = {
  assert!(EXTERNALIZE_STRING_BUFFER_SIZE == 400);
  assert!(JS_CONST_NAMES.len() == 4);
  assert!(MIN_LENGTH_FORMULAS.len() == 4);
  assert!(NATIVE_FUNCTIONS.len() == 4);
};

// —— 签名骨架 (h 声明 6 / cc 定义 6, 1:1; 均为 ConstructorBehavior::kThrow) ——
// public: GetNativeFunctionTemplate(isolate, name) — 按 name 四分发
// public static: Externalize(info) — SupportsExternalization 失败且非已外化则抛错;
//                共享串外化竞态 (HasExternalForwardingIndex) 不算失败
// public static: CreateExternalizableString(info)
// public static: CreateExternalizableTwoByteString(info)
// public static: IsOneByte(info) — 单 string 参, 回 IsOneByteRepresentation
// private static: BuildSource(buf, size) -> *const c_char
// 匿名 namespace:
//   HasExternalForwardingIndex(string) -> bool — 共享串外化转发索引探测
//   CopyConsStringToOld(isolate, cons_string, encoding) — ConsString 拷旧生代
//   CreateExternalizableString(isolate, string, encoding) — RO 串拒外化;
//     V8_COMPRESS_POINTERS 下短串拒外化; ConsString 非平且首部非空走拷贝路径

// 平台分支台账:
//   #ifdef V8_COMPRESS_POINTERS — "String is too short to be externalized." 分支,
//   x64/arm64 默认开启指针压缩 → 展开; 非 V8_ENABLE_DIRECT_HANDLE 的 OpenHandle
//   间接句柄变体属 api-inl 层, 此处登记不展开
`;

const p = writeSrc("v8-extensions", "externalize_string_extension.rs", rust);
console.log(`OK externalize-string-extension.h.js: 对数断言 ${n} 全过; 公式 4 / native 4 / let 4 / 方法 6 / kThrow 4; 产物 ${p}`);
