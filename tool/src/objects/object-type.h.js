#!/usr/bin/env bun
// tool/src/objects/object-type.h.js — 覆盖 v8/src/objects/object-type.{h,cc}
// ObjectType 全量枚举 (clang -E 预处理展开, 对数基准 = 编译器产物)
// 构建配置: INTL 开 / WASM 开 / TEMPORAL 关 / METAGEN 空 / DRUMBRAKE 关
// → v8-objects-core/src/object_type.rs

import { readV8, writeSrc, banner } from "../../lib.js";
import { execFileSync } from "node:child_process";
import { join } from "node:path";
import { mkdirSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";

const hSrc = readV8("src/objects/object-type.h");

// —— clang -E 对数基准 ——
const shim = join(tmpdir(), "v8rs-tool-pp-shim");
mkdirSync(join(shim, "absl/numeric"), { recursive: true });
mkdirSync(join(shim, "metagen"), { recursive: true });
writeFileSync(
  join(shim, "absl/numeric/int128.h"),
  "#pragma once\n#include <cstdint>\nnamespace absl { using int128 = __int128; using uint128 = unsigned __int128; }\n",
);
writeFileSync(join(shim, "metagen/instance-types.h"), "#pragma once\n");

const ppOut = execFileSync(
  "clang",
  ["-E", "-P", "-x", "c++", "-std=c++20", "-I", "v8", "-I", "v8/include", "-I", shim,
    "-DV8_INTL_SUPPORT=1", "-DV8_ENABLE_WEBASSEMBLY=1", "-"],
  {
    input: '#include "src/objects/object-type.h"\n',
    encoding: "utf8",
    cwd: join(import.meta.dir, "../../.."),
    maxBuffer: 256 * 1024 * 1024,
  },
);

const enumBody = ppOut.match(/enum class ObjectType \{([\s\S]*?)\};/);
if (!enumBody) throw new Error("预处理产物中未找到 ObjectType 枚举");
const variants = [...new Set(enumBody[1].matchAll(/\bk(\w+)\b/g))].map((m) => m[1]);
// 展开体中 STRUCT_LIST 等以逗号平铺, 正则按出现序去重即可; 锚点对数
if (variants.length !== 386) throw new Error(`ObjectType 变体数不符: ${variants.length}, 期望 386`);
if (variants[0] !== "Object") throw new Error("首项锚点不符 (应为 Object)");
if (variants[4] !== "HeapObjectReference") throw new Error("第 5 项锚点不符");
if (!variants.includes("WasmExceptionTag")) throw new Error("缺 WasmExceptionTag (IF_WASM 分支)");
if (!variants.includes("JSCollator")) throw new Error("缺 JSCollator (INTL 分支)");
if (variants.includes("JSTemporalDuration")) throw new Error("TEMPORAL 应为关闭分支");

// cc 侧结构对数: CheckObjectType DEBUG 分支两组 switch (弱引用/强引用)
const ccSrc = readV8("src/objects/object-type.cc");
const weakSwitch = (ccSrc.match(/HAS_WEAK_HEAP_OBJECT_TAG\(raw_value\)/) ?? [])[0];
const strongSwitch = (ccSrc.match(/Tagged<Object> value\(raw_value\);/) ?? [])[0];
if (!weakSwitch || !strongSwitch) throw new Error("object-type.cc 结构锚点缺失");

console.log(`ObjectType ${variants.length} 变体 (clang 基准)`);

let rust = banner(
  "src/objects/object-type.h.js",
  "src/objects/object-type.{h,cc}",
  "ObjectType 检查标签枚举 (clang -E 全量展开, INTL+WASM 开, TEMPORAL/METAGEN/DRUMBRAKE 关)",
);

rust += `
/// 对象检查标签 (原版 enum class ObjectType, 顺序即基线序, 勿重排)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u16)]
pub enum ObjectType {
`;
variants.forEach((v, i) => {
  rust += `  /// k${v}\n  ${v} = ${i},\n`;
});
rust += `}

impl ObjectType {
  /// 变体总数 (对数基准: clang -E 展开项)
  pub const COUNT: usize = ${variants.length};

  /// 原版 TYPE_CASE 期望名 (#Name 字符串化)
  #[inline]
  #[must_use]
  pub const fn to_str(self) -> &'static str {
    match self {
`;
for (const v of variants) rust += `      Self::${v} => "${v}",\n`;
rust += `    }
  }
}

// —— 签名骨架 (原版 CheckObjectType, DEBUG 构建专用) ——
// Address CheckObjectType(Address raw_value, Address raw_type, Address raw_location);
// 弱引用路径: kHeapObjectReference 直接放行, 其余标签一律报错 (弱引用不可直接 cast)
// 强引用路径: 逐标签调用 Is<Name>(value) 判定, 通过返回 Smi(0), 失败 FATAL 定位
// 发布构建: UNREACHABLE (仅在 DEBUG 生效)
pub const CHECK_OBJECT_TYPE_DEBUG_ONLY: bool = true;

// —— 提取不变式断言 ——
const _: () = {
  assert!(ObjectType::COUNT == 386);
  assert!(ObjectType::Object as u16 == 0);
  assert!(ObjectType::HeapObjectReference as u16 == 4);
};
`;

const p = writeSrc("v8-objects-core", "object_type.rs", rust);
console.log(`written ${p}`);
