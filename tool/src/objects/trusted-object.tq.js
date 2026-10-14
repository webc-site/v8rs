#!/usr/bin/env bun
// tool/src/objects/trusted-object.tq.js — 受信对象 tq 布局生成
// 组→文件映射 (本脚本管辖):
//   v8/src/objects/trusted-object.tq → v8-objects-rest/src/trusted_object_tq.rs
//   (trusted-object.{h,-inl.h} 归 tool/src/objects/trusted-object{,-inl}.h.js → v8-objects-arrays/trusted*.skel;
//    本脚本只落 tq 侧类层级与沙箱条件字段)
import { readV8, writeSrc, banner } from "../../lib.js";

const tq = readV8("src/objects/trusted-object.tq");
const h = readV8("src/objects/trusted-object.h");

// —— tq 类层级解析 ——
const classes = [...tq.matchAll(/^(@[\w]+\s*\n)*extern class (\w+) extends (\w+)(?: \{([\s\S]*?)\})?/gm)].map((m) => {
  const annots = (m[0].match(/^@(\w+)/gm) || []).map((a) => a.slice(1));
  return { name: m[2], base: m[3], annots, fields: (m[4] ?? "").split("\n").map((l) => l.trim()).filter((l) => l && !l.startsWith("//")) };
});
if (classes.length !== 2) throw new Error(`trusted-object.tq 类应 2, 实得 ${classes.length}: ${classes.map((c) => c.name)}`);
if (classes[0].name !== "TrustedObject" || classes[0].base !== "HeapObject") throw new Error("TrustedObject extends HeapObject 漂移");
if (classes[1].name !== "ExposedTrustedObject" || classes[1].base !== "TrustedObject") throw new Error("ExposedTrustedObject 继承漂移");
if (classes[0].annots.join() !== "abstract,cppObjectLayoutDefinition") throw new Error(`TrustedObject 注解漂移: ${classes[0].annots}`);
if (classes[1].annots.join() !== "abstract,cppObjectLayoutDefinition") throw new Error(`ExposedTrustedObject 注解漂移: ${classes[1].annots}`);
// 条件字段: @if(V8_ENABLE_SANDBOX) self_indirect_pointer: TrustedPointer;
if (classes[1].fields.join() !== "@if(V8_ENABLE_SANDBOX) self_indirect_pointer: TrustedPointer;")
  throw new Error(`ExposedTrustedObject 条件字段漂移: ${classes[1].fields}`);

// —— h 侧跨源核对 (布局契约) ——
if (!/V8_OBJECT class TrustedObject : public HeapObject \{/.test(h) && !/class TrustedObject : public HeapObject/.test(h))
  throw new Error("trusted-object.h TrustedObject 声明缺失");
if (!/class ExposedTrustedObject : public TrustedObject/.test(h)) throw new Error("ExposedTrustedObject 声明缺失");

const body = `${banner("src/objects/trusted-object.tq.js", "src/objects/trusted-object.tq (+ trusted-object.h 跨源核对)", "非压缩沙箱关: 自间接指针字段不存在")}
//! 受信对象 tq 布局 (原版 trusted-object.tq)
//!
//! 类层级: HeapObject → TrustedObject (abstract) → ExposedTrustedObject (abstract).
//! ExposedTrustedObject 在沙箱开时增加 self_indirect_pointer (自引用句柄,
//! 供间接指针表反查); 基线沙箱关 → 头与 TrustedObject 同为 8 字节.

/// tq 类层级 (注解 + 条件字段 1:1)
pub const TRUSTED_OBJECT_TQ_CLASSES: [(&str, &str, &str, &str); ${classes.length}] = [
  // (类, 基类, 注解, 条件字段)
  ("TrustedObject", "HeapObject", "@abstract @cppObjectLayoutDefinition", ""),
  ("ExposedTrustedObject", "TrustedObject", "@abstract @cppObjectLayoutDefinition", "@if(V8_ENABLE_SANDBOX) self_indirect_pointer: TrustedPointer"),
];

/// 沙箱开时自间接指针偏移 (ExposedTrustedObject 头内, 紧跟 map;
/// 基线沙箱关: 字段不存在, 本常量仅登记生成侧语义)
pub const K_SELF_INDIRECT_POINTER_OFFSET_SANDBOX_ON: usize = 8;

/// 取沙箱开时的自间接指针槽语义 (原版 self_indirect_pointer 字段)
pub fn self_indirect_pointer(/* this */) {
  todo!("就地修订: 沙箱开读 IndirectPointerHandle (kSelfIndirectPointerTag); 关配置不存在")
}
`;

const p = writeSrc("v8-objects-rest", "trusted_object_tq.rs", body);
console.log(`trusted-object.tq.js: 已生成 ${p} (类 ${classes.length}, 条件字段 ${classes[1].fields.length})`);
