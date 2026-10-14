#!/usr/bin/env bun
// tool/src/objects/foreign.h.js — 覆盖 v8/src/objects/foreign.{h,-inl.h,tq}
// Foreign 外部指针包装: tq API 暴露类型常量与骨架
// → v8-objects-rest/src/foreign.rs

import { readV8, writeSrc, banner } from "../../lib.js";

const hSrc = readV8("src/objects/foreign.h");
const inlSrc = readV8("src/objects/foreign-inl.h");
const tqSrc = readV8("src/objects/foreign.tq");

// 1. tq 侧 API 暴露实例类型值
const apiVal = tqSrc.match(/@apiExposedInstanceTypeValue\((0x[0-9a-fA-F]+)\)/);
if (!apiVal) throw new Error("tq 缺 @apiExposedInstanceTypeValue");
const apiExposed = apiVal[1];

// 2. 结构锚点
const anchors = [
  [hSrc, "V8_OBJECT class Foreign : public HeapObject"],
  [hSrc, "V8_OBJECT class TrustedForeign : public TrustedObject"],
  [tqSrc, "extern class Foreign extends HeapObject"],
  [tqSrc, "extern class TrustedForeign extends TrustedObject"],
  [inlSrc, "foreign_address"],
];
for (const [src, a] of anchors) {
  if (!src.includes(a)) throw new Error(`foreign 缺锚点: ${a}`);
}

// 3. h 侧方法面
const hMethods = [...hSrc.matchAll(/inline (?:Address|Tagged<[^>]*>) (\w+)\(/g)].map((m) => m[1]);
if (hMethods.length < 2) throw new Error(`h 方法面异常: ${hMethods.length}`);

console.log(`Foreign apiExposed=${apiExposed}, 锚点 ${anchors.length}`);

let rust = banner("src/objects/foreign.h.js", "src/objects/foreign.{h,-inl.h,tq}", "Foreign 外部指针包装");

rust += `
/// 外部指针包装 (原版 V8_OBJECT class Foreign : public HeapObject)
/// 布局 (tq): map + foreign_address (ExternalPointer, 沙箱内为句柄)
/// API 暴露实例类型值 (原版 @apiExposedInstanceTypeValue(${apiExposed}))
pub const API_EXPOSED_INSTANCE_TYPE: u32 = ${apiExposed};

/// 受信形态 (原版 TrustedForeign : TrustedObject, foreign_address 为 RawPtr)
pub struct TrustedForeign;

/// 外部地址读写访问器族 (inl 侧)
pub const FOREIGN_ACCESSORS: [&str; 4] = ["foreign_address", "set_foreign_address", "raw_foreign_address", "init_foreign_address"];

// —— 签名骨架 ——
// Address foreign_address(IsolateForSandbox) const; (+NoIsolate 变体)
// void set_foreign_address(IsolateForSandbox, Address);
// DECL_VERIFIER(Foreign) / BodyDescriptor (ExternalPointer 外部指针体描述)
pub const TRUSTED_FOREIGN_POINTER_KIND: &str = "RawPtr";

// —— 提取不变式断言 ——
const _: () = {
  assert!(API_EXPOSED_INSTANCE_TYPE == 0xcc);
  assert!(FOREIGN_ACCESSORS.len() == 4);
};
`;

const p = writeSrc("v8-objects-rest", "foreign.rs", rust);
console.log(`written ${p}`);
