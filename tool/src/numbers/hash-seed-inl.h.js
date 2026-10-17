#!/usr/bin/env bun
// tool/src/numbers/hash-seed-inl.h.js
// 机械提取 v8/src/numbers/hash-seed-inl.h: HashSeed 内联构造/访问器签名骨架
// (Data 布局本体在 hash-seed.h, 默认种子已由 tool/src/numbers/hash-seed.cc.js 产出)

import { readV8, writeSrc, banner } from "../../lib.js";

const inlSrc = readV8("src/numbers/hash-seed-inl.h");
const hSrc = readV8("src/numbers/hash-seed.h");

// 1. -inl 构造重载对数: Isolate / LocalIsolate / ReadOnlyRoots 三路 + Default
const ctors = inlSrc.match(/inline HashSeed::HashSeed\(([^)]+)\)/g);
if (!ctors || ctors.length !== 3) throw new Error(`HashSeed 内联构造应 3, 实得 ${ctors ? ctors.length : 0}`);

// 2. 访问器清单
const accessors = ["seed", "secret"];
const seededAccessors = ["m1", "m1_inv", "m2", "m2_inv", "m3", "m3_inv"];
for (const a of [...accessors, ...seededAccessors]) {
  if (!new RegExp(`inline [\\w:* ]+? HashSeed::${a}\\(\\)`).test(inlSrc)) {
    throw new Error(`hash-seed-inl.h 未找到访问器 ${a}`);
  }
}

// 3. V8_ENABLE_SEEDED_ARRAY_INDEX_HASH 分支登记 (x64/arm64 默认关闭 → 弃用组注释登记)
if (!inlSrc.includes("#ifdef V8_ENABLE_SEEDED_ARRAY_INDEX_HASH")) {
  throw new Error("未找到 V8_ENABLE_SEEDED_ARRAY_INDEX_HASH 分支");
}

// 4. Data 布局对数 (hash-seed.h)
const dataFields = hSrc.match(/struct Data\s*\{([\s\S]*?)\n\s*\};/);
if (!dataFields) throw new Error("hash-seed.h 未找到 struct Data");
const hasSeed = /uint64_t seed;/.test(dataFields[1]);
const hasSecrets = /uint64_t secrets\[kSecretsCount\];/.test(dataFields[1]);
if (!hasSeed || !hasSecrets) throw new Error("Data 布局漂移: seed/secrets 缺失");

let rust = banner("src/numbers/hash-seed-inl.h.js", "src/numbers/hash-seed-inl.h", "HashSeed 内联访问骨架");

rust += `
/// HashSeed 内联层签名骨架 (原版 hash-seed-inl.h)。
/// Data 布局: { uint64_t seed; uint64_t secrets[kSecretsCount]; ... }
/// 实例只指向只读根或 kDefaultData, 不持所有者。
pub mod hash_seed_inl_skel {
  /// 种子值 (原版 seed(): data_->seed)
  pub fn seed(_data: usize) -> u64 {
    unimplemented!("骨架")
  }

  /// 密钥组指针 (原版 secret(): data_->secrets)
  pub fn secret(_data: usize) -> *const u64 {
    unimplemented!("骨架")
  }

  /// 从 Isolate 构造 (原版 HashSeed(Isolate*) → 委派 ReadOnlyRoots)
  pub fn from_isolate(_isolate_key: usize) -> usize {
    unimplemented!("骨架")
  }

  /// 从 LocalIsolate 构造 (原版 HashSeed(LocalIsolate*) → 委派 ReadOnlyRoots)
  pub fn from_local_isolate(_isolate_key: usize) -> usize {
    unimplemented!("骨架")
  }

  /// 从只读根构造 (原版 HashSeed(ReadOnlyRoots)); 对齐断言:
  /// data_ 按 alignof(HashSeed::Data) 对齐
  pub fn from_read_only_roots(_roots: usize) -> usize {
    unimplemented!("骨架")
  }

  /// 默认种子 (原版 Default() → kDefaultData; 默认值见表 hash_seed.rs)
  pub fn default_seed() -> usize {
    unimplemented!("骨架")
  }

  // —— 平台分支登记: V8_ENABLE_SEEDED_ARRAY_INDEX_HASH (x64/arm64 默认关闭) ——
  // 启用时另有 m1/m1_inv/m2/m2_inv/m3/m3_inv 六个访问器 (取 secrets[0] 低
  // kArrayIndexValueBits 位的奇数及其模 2^k 的逆), 本目标跳过。
  pub const SEEDED_ARRAY_INDEX_HASH_ENABLED: bool = false;
}
`;

const p = writeSrc("v8-numbers", "hash_seed_inl_generated.rs", rust);
console.log(`written ${p}; 构造=${ctors.length}, 访问器=${accessors.length + seededAccessors.length}`);
