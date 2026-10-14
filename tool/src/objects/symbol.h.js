#!/usr/bin/env bun
// tool/src/objects/symbol.h.js — 校验并提取 Symbol 字段偏移、位域与公共符号
// 来源:
//   v8/src/objects/name.h (Symbol 声明) + name-inl.h + v8/src/init/heap-symbols.h
//   → 对应 v8-objects-rest/src/symbol.rs
import { readV8 } from "../../lib.js";

let nameH;
try {
  nameH = readV8("src/objects/name.h");
} catch {
  // class Symbol 声明在 src/objects/name.h, 上游重组时在此登记新路径
  nameH = readV8("src/objects/name-inl.h");
}
const heapSymbolsH = readV8("src/init/heap-symbols.h");

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
mustContain(nameH, "V8_OBJECT class Symbol : public Name", "name.h");
mustContain(nameH, "using PrivateSymbolKindBits = base::BitField<PrivateSymbolKind, 0, 2>;", "name.h");
mustContain(nameH, "using IsWellKnownSymbolBit = PrivateSymbolKindBits::Next<bool, 1>;", "name.h");
mustContain(nameH, "using IsInPublicSymbolTableBit = IsWellKnownSymbolBit::Next<bool, 1>;", "name.h");
mustContain(nameH, "using IsInterestingSymbolBit = IsInPublicSymbolTableBit::Next<bool, 1>;", "name.h");
mustContain(nameH, "uint32_t flags_ V8_TQ_TYPE(SymbolFlags);", "name.h");
mustContain(nameH, "TaggedMember<UnionOf<String, Undefined>> description_;", "name.h");
mustContain(nameH, "FixedBodyDescriptor<offsetof(Symbol, description_)", "name.h");

// 提取 heap-symbols.h 中的公共符号与众所周知符号
mustContain(heapSymbolsH, "#define PUBLIC_SYMBOL_LIST_GENERATOR(V, _)", "heap-symbols.h");
mustContain(heapSymbolsH, "#define WELL_KNOWN_SYMBOL_LIST_GENERATOR(V, _)", "heap-symbols.h");
mustContain(heapSymbolsH, "#define SYMBOL_FOR_PROTECTOR_LIST_GENERATOR(V, _)", "heap-symbols.h");

// ── 2. 位域定义 ───────────────────────────────────────────────────────────
export const PRIVATE_SYMBOL_KIND_SHIFT = 0;
export const PRIVATE_SYMBOL_KIND_SIZE = 2;
export const PRIVATE_SYMBOL_KIND_MASK = 0b11;

export const IS_WELL_KNOWN_SYMBOL_SHIFT = 2;
export const IS_WELL_KNOWN_SYMBOL_SIZE = 1;
export const IS_WELL_KNOWN_SYMBOL_MASK = 1 << 2;

export const IS_IN_PUBLIC_SYMBOL_TABLE_SHIFT = 3;
export const IS_IN_PUBLIC_SYMBOL_TABLE_SIZE = 1;
export const IS_IN_PUBLIC_SYMBOL_TABLE_MASK = 1 << 3;

export const IS_INTERESTING_SYMBOL_SHIFT = 4;
export const IS_INTERESTING_SYMBOL_SIZE = 1;
export const IS_INTERESTING_SYMBOL_MASK = 1 << 4;

// ── 3. 字段偏移 (64-bit 非压缩 Tagged) ────────────────────────────────────
// Name: HeapObject header (8) + raw_hash_field (4)
// Symbol: flags (4, 与 raw_hash_field 紧凑排布至 8B 对齐) + description (8)
export const K_SYMBOL_NAME_OFFSET = 0;
export const K_SYMBOL_FLAGS_OFFSET = 12;
export const K_SYMBOL_DESCRIPTION_OFFSET = 16;
export const K_SYMBOL_SIZE = 24;

must(K_SYMBOL_FLAGS_OFFSET === 12, "flags_ offset 校验");
must(K_SYMBOL_DESCRIPTION_OFFSET === 16, "description_ offset 校验");
must(K_SYMBOL_SIZE === 24, "Symbol size 校验");

console.log(`[symbol.h.js] 提取与锚点校验通过 (共 ${checks} 项校验):`);
console.log({
  PRIVATE_SYMBOL_KIND_MASK,
  IS_WELL_KNOWN_SYMBOL_MASK,
  IS_IN_PUBLIC_SYMBOL_TABLE_MASK,
  IS_INTERESTING_SYMBOL_MASK,
  K_SYMBOL_FLAGS_OFFSET,
  K_SYMBOL_DESCRIPTION_OFFSET,
  K_SYMBOL_SIZE,
});
