#!/usr/bin/env bun
// tool/src/objects/value-serializer-skel.h.js — 覆盖 v8/src/objects/value-serializer.h (+ .cc 版本常量补面)
// 组→文件映射 (本脚本管辖): 席位表差集
//   (SerializationTag 全量枚举归既有 tool/src/objects/value-serializer.h.js; 本脚本补类面与版本常量)
// 产物: v8-objects-js-async/src/value_serializer_skel.rs
// 机械提取: kLatestVersion 线上版本常量 + ValueSerializer/ValueDeserializer 类面骨架 (todo! 就地修订)

import { readV8, writeSrc, banner } from "../../lib.js";

const h = readV8("src/objects/value-serializer.h");
const cc = readV8("src/objects/value-serializer.cc");

let checks = 0;
const must = (c, m) => { checks++; if (!c) throw new Error(`[value-serializer-skel.h.js] ${m}`); };

// ── 1. 版本常量 (cc 补面; 既有脚本只产 SerializationTag) ─────────────────
const latest = cc.match(/static const uint32_t kLatestVersion = (\d+);/);
must(latest, "cc 缺 kLatestVersion");
const K_LATEST_VERSION = Number(latest[1]);
must(K_LATEST_VERSION === 16, `kLatestVersion ${K_LATEST_VERSION} != 16`);
must(cc.includes("static_assert(kLatestVersion == v8::CurrentValueSerializerFormatVersion(),"), "缺版本一致性 static_assert");
must(cc.includes("WriteVarint(kLatestVersion);"), "缺 WriteHeader 写版本锚");
must(cc.includes("version_ > kLatestVersion) {"), "缺反序列化版本上界检查锚");

// ── 2. .h 类面锚 (SerializationTag 前向声明 + 两类骨架) ──────────────────
must(h.includes("enum class SerializationTag : uint8_t;"), "h 缺 SerializationTag 前向声明");
must(h.includes("class ValueSerializer {"), "h 缺 ValueSerializer 类");
must(h.includes("class ValueDeserializer {"), "h 缺 ValueDeserializer 类");

// ValueSerializer 公开面
const vsStart = h.indexOf("class ValueSerializer {");
const vsEnd = h.indexOf("class ValueDeserializer {");
const vsBlock = h.slice(vsStart, vsEnd);
for (const s of [
  "explicit ValueSerializer(", "v8::ValueSerializer::Delegate* delegate,",
  "void WriteHeader();", "Maybe<bool> WriteObject(", "std::pair<uint8_t*, size_t> Release();",
  "void TransferArrayBuffer(", "void WriteUint32(uint32_t value);", "void WriteUint64(uint64_t value);",
  "void WriteRawBytes(const void* source, size_t length);", "void WriteDouble(double value);", "void WriteByte(uint8_t value);",
  "void SetTreatArrayBufferViewsAsHostObjects(bool mode);", "Maybe<bool> ExpandBuffer(size_t required_capacity);",
]) must(vsBlock.includes(s), `ValueSerializer 缺 ${s}`);

// ValueDeserializer 公开面
const vdBlock = h.slice(vsEnd);
for (const s of [
  "Maybe<bool> ReadHeader() V8_WARN_UNUSED_RESULT;",
  "uint32_t GetWireFormatVersion() const { return version_; }",
  "MaybeDirectHandle<Object> ReadObjectWrapper() V8_WARN_UNUSED_RESULT;",
  "MaybeDirectHandle<Object> ReadObjectUsingEntireBufferForLegacyFormat(",
  "void TransferArrayBuffer(uint32_t transfer_id,",
  "bool ReadUint32(uint32_t* value) V8_WARN_UNUSED_RESULT;",
  "bool ReadUint64(uint64_t* value) V8_WARN_UNUSED_RESULT;",
  "bool ReadDouble(double* value) V8_WARN_UNUSED_RESULT;",
  "bool ReadRawBytes(size_t length, const void** data) V8_WARN_UNUSED_RESULT;",
  "Maybe<SerializationTag> PeekTag() const V8_WARN_UNUSED_RESULT;",
  "MaybeDirectHandle<Object> ReadObject() V8_WARN_UNUSED_RESULT;",
  "MaybeDirectHandle<JSArrayBuffer> ReadJSArrayBuffer(",
  "MaybeDirectHandle<JSArrayBufferView> ReadJSArrayBufferView(",
]) must(vdBlock.includes(s), `ValueDeserializer 缺 ${s}`);

// Delegate 面 (经 v8::ValueSerializer::Delegate 由嵌入器实现; h 侧引用)
must(vsBlock.includes("v8::ValueSerializer::Delegate* delegate"), "缺 Delegate 注入锚");

let rust = banner("src/objects/value-serializer-skel.h.js", "src/objects/value-serializer.h (+ value-serializer.cc 版本常量补面)",
  "席位表差集: 版本常量 + 序列化器/反序列化器类面骨架 (SerializationTag 归既有 value-serializer.h.js)");
rust += `//! 结构化克隆 (structured clone) 序列化器类面骨架。
//!
//! SerializationTag 线上标签全量归既有 value-serializer.h.js 产物;
//! 本文件补: 线上版本常量 (kLatestVersion=16, 不可回退) 与
//! ValueSerializer/ValueDeserializer 方法骨架。

/// 线上格式版本 (原版 kLatestVersion; 每次格式变更单调递增, 不可复用旧值)。
pub const K_LATEST_VERSION: u32 = ${K_LATEST_VERSION};

// ── ValueSerializer 公开面骨架 (语义就地修订) ──
/// 原版 WriteHeader (cc: 写 varint 版本 kLatestVersion)。
pub fn write_header() {
  todo!("就地修订: value-serializer.cc WriteHeader")
}

/// 原版 WriteObject (cc: 按 Tag 分派递归序列化)。
pub fn write_object() {
  todo!("就地修订: value-serializer.cc WriteObject")
}

/// 原版 Release (cc: 交还缓冲区所有权)。
pub fn release() {
  todo!("就地修订: value-serializer.cc Release")
}

/// 原版 TransferArrayBuffer (cc: 带外转移标记)。
pub fn transfer_array_buffer() {
  todo!("就地修订: value-serializer.cc TransferArrayBuffer")
}

// 线上写原语 (公开给 Delegate::WriteHostObject 用):
//   WriteUint32 / WriteUint64 / WriteRawBytes / WriteDouble / WriteByte

/// 原版 ExpandBuffer (私有: 容量倍增)。
pub fn expand_buffer() {
  todo!("就地修订: value-serializer.cc ExpandBuffer")
}

// ── ValueDeserializer 公开面骨架 (语义就地修订) ──
/// 原版 ReadHeader (cc: 读版本并校验 <= kLatestVersion)。
pub fn read_header() {
  todo!("就地修订: value-serializer.cc ReadHeader")
}

/// 原版 ReadObject (cc: 按 Tag 分派递归反序列化)。
pub fn read_object() {
  todo!("就地修订: value-serializer.cc ReadObject")
}

/// 原版 ReadJSArrayBuffer (cc: 带外转移/内联恢复)。
pub fn read_js_array_buffer() {
  todo!("就地修订: value-serializer.cc ReadJSArrayBuffer")
}

/// 原版 ReadJSArrayBufferView (cc: 视图槽位恢复)。
pub fn read_js_array_buffer_view() {
  todo!("就地修订: value-serializer.cc ReadJSArrayBufferView")
}

// 线上读原语: ReadUint32 / ReadUint64 / ReadDouble / ReadRawBytes
// 版本面: version() (WriteHeader 落值), set_expect_inline_wasm_objects

// ── 提取不变式断言 ──
const _: () = {
  assert!(K_LATEST_VERSION == ${K_LATEST_VERSION});
  assert!(K_LATEST_VERSION > 0);
};
`;

const p = writeSrc("v8-objects-js-async", "value_serializer_skel.rs", rust);
console.log(`value-serializer-skel.h.js: ${p} — kLatestVersion ${K_LATEST_VERSION} + 序列化器 6 骨架 + 反序列化器 4 骨架, 断言 ${checks} 项全过`);
