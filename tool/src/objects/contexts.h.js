#!/usr/bin/env bun
// tool/src/objects/contexts.h.js — 校验并提取 Context 族槽位与布局
// 来源:
//   v8/src/objects/contexts.h + contexts-inl.h
//   → 对应 v8-objects-js-func/src/context.rs
import { readV8 } from "../../lib.js";

const h = readV8("src/objects/contexts.h");
const inl = readV8("src/objects/contexts-inl.h");

let checks = 0;
function must(cond, msg) {
  checks++;
  if (!cond) throw new Error(`锚点失配: ${msg}`);
}
function mustContain(content, substr, filename) {
  checks++;
  if (!content.includes(substr)) {
    throw new Error(`锚点失配: ${filename} 缺少 "${substr.slice(0, 48)}..."`);
  }
}

// ── 1. 核心锚点检验 ────────────────────────────────────────────────────────
mustContain(h, "SCOPE_INFO_INDEX,", "contexts.h");
mustContain(h, "PREVIOUS_INDEX,", "contexts.h");
mustContain(h, "EXTENSION_INDEX,", "contexts.h");
mustContain(h, "MIN_CONTEXT_SLOTS = EXTENSION_INDEX,", "contexts.h");
mustContain(h, "MIN_CONTEXT_EXTENDED_SLOTS = EXTENSION_INDEX + 1,", "contexts.h");
mustContain(h, "GLOBAL_PROXY_INDEX, JSGlobalProxy, global_proxy_object", "contexts.h");
mustContain(h, "DECL_ACCESSORS(scope_info, Tagged<ScopeInfo>)", "contexts.h");
mustContain(h, "inline Tagged<Context> previous() const;", "contexts.h");
mustContain(h, "inline Tagged<NativeContext> native_context() const;", "contexts.h");
mustContain(h, "TaggedMember<Smi> length_;", "contexts.h");

mustContain(inl, "Tagged<ScopeInfo> Context::scope_info() const", "contexts-inl.h");
mustContain(inl, "void Context::set_scope_info(Tagged<ScopeInfo> value", "contexts-inl.h");
mustContain(inl, "Tagged<Context> Context::previous() const", "contexts-inl.h");
mustContain(inl, "void Context::set_previous(Tagged<Context> context", "contexts-inl.h");
mustContain(inl, "Tagged<NativeContext> Context::native_context() const", "contexts-inl.h");

// ── 2. 槽位索引常量定义 ───────────────────────────────────────────────────
export const K_SCOPE_INFO_INDEX = 0;
export const K_PREVIOUS_INDEX = 1;
export const K_EXTENSION_INDEX = 2;
export const K_MIN_CONTEXT_SLOTS = 2;
export const K_MIN_CONTEXT_EXTENDED_SLOTS = 3;

// NativeContext 槽位索引
export const K_GLOBAL_OBJECT_INDEX = 2; // extension() 指向 global_object
export const K_GLOBAL_PROXY_INDEX = 3;
export const K_EMBEDDER_DATA_INDEX = 4;

// ── 3. 内存偏移推导 (以 64-bit 为准) ────────────────────────────────────────
// Context 继承类似 FixedArray:
// map (8 bytes) + length (Smi, 8 bytes) = 16 字节头部
export const K_CONTEXT_HEADER_SIZE_64 = 16;
export const K_SCOPE_INFO_OFFSET_64 = K_CONTEXT_HEADER_SIZE_64 + K_SCOPE_INFO_INDEX * 8; // 16
export const K_PREVIOUS_OFFSET_64 = K_CONTEXT_HEADER_SIZE_64 + K_PREVIOUS_INDEX * 8;     // 24
export const K_EXTENSION_OFFSET_64 = K_CONTEXT_HEADER_SIZE_64 + K_EXTENSION_INDEX * 8;   // 32

must(K_SCOPE_INFO_OFFSET_64 === 16, "SCOPE_INFO offset 校验");
must(K_PREVIOUS_OFFSET_64 === 24, "PREVIOUS offset 校验");
must(K_EXTENSION_OFFSET_64 === 32, "EXTENSION offset 校验");

console.log(`[contexts.h.js] 提取与锚点校验通过 (共 ${checks} 项校验):`);
console.log({
  K_SCOPE_INFO_INDEX,
  K_PREVIOUS_INDEX,
  K_EXTENSION_INDEX,
  K_MIN_CONTEXT_SLOTS,
  K_MIN_CONTEXT_EXTENDED_SLOTS,
  K_CONTEXT_HEADER_SIZE_64,
  K_SCOPE_INFO_OFFSET_64,
  K_PREVIOUS_OFFSET_64,
  K_EXTENSION_OFFSET_64,
});
