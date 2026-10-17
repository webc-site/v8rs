#!/usr/bin/env bun
// tool/src/numbers/hash-seed.h.js
// 机械提取 v8/src/numbers/hash-seed.{h,cc} 的 .cc 残余面 —
// 默认种子数据 (RAPIDHASH_DEFAULT_SECRET 展开) + 数组索引哈希派生算法
// (modular_inverse/derive_multiplier, V8_ENABLE_SEEDED_ARRAY_INDEX_HASH 分支)。
// 注: .h 常量面已由 tool/src/numbers/hash-seed.cc.js 落 hash_seed.rs;
//     本脚本按家族合并规则补 .cc 段, 落独立文件避覆写。
// 产物: v8-numbers/src/hash_seed_cc_generated.rs

import { readV8, writeSrc, banner } from "../../lib.js";

const CC_REL = "src/numbers/hash-seed.cc";
const H_REL = "src/numbers/hash-seed.h";
const SECRET_REL = "third_party/rapidhash-v8/secret.h";
readV8(CC_REL); // 家族覆盖锚
const h = readV8(H_REL);
const secret = readV8(SECRET_REL);

// 默认 secret 三元组
const secM = secret.match(/RAPIDHASH_DEFAULT_SECRET\[3\] = \{([^}]+)\};/);
if (!secM) throw new Error("未找到 RAPIDHASH_DEFAULT_SECRET");
const secrets = [...secM[1].matchAll(/0x[0-9a-fA-F]+/g)].map((s) => BigInt(s[0]));
if (secrets.length !== 3) throw new Error(`secret 条数不符: ${secrets.length} != 3`);

// kSecretsCount 锚
const cntM = h.match(/static constexpr int kSecretsCount = (\d+);/);
if (!cntM || cntM[1] !== "3") throw new Error("kSecretsCount 锚不符");
// 位宽实值取自 src/objects/name.h (kArrayIndexValueBits), 派生校验依赖之
const nameH = readV8("src/objects/name.h");
const bitsM = nameH.match(/static constexpr int kArrayIndexValueBits = (\d+);/);
if (!bitsM) throw new Error("未找到 kArrayIndexValueBits");
const K_ARRAY_INDEX_VALUE_BITS = Number(bitsM[1]);
if (K_ARRAY_INDEX_VALUE_BITS !== 24 && K_ARRAY_INDEX_VALUE_BITS !== 26) {
  throw new Error(`kArrayIndexValueBits 值异常: ${K_ARRAY_INDEX_VALUE_BITS}`);
}

// 派生算法 (机械转录, uint32 模 2^32 运算)
function modularInverse(m) {
  let x = ((3 * m) ^ 2) >>> 0;
  for (let i = 0; i < 4; i++) x = Math.imul(x, 2 - Math.imul(m, x)) >>> 0;
  return x >>> 0;
}
const truncate = (s) => Number(s & ((1n << BigInt(K_ARRAY_INDEX_VALUE_BITS)) - 1n)) >>> 0;
const deriveMultiplier = (s) => (truncate(s) | 1) >>> 0;
// 自洽校验: (m * m_inv) & mask === 1
function isModularInverse(m, mInv) {
  return (Math.imul(m, mInv) & ((1 << K_ARRAY_INDEX_VALUE_BITS) - 1)) === 1;
}
const derived = secrets.map((s) => {
  const m = deriveMultiplier(s);
  const inv = modularInverse(m);
  if (!isModularInverse(m, inv)) throw new Error(`模逆自洽校验失败: secret 0x${s.toString(16)}`);
  return { m, inv };
});

let rust = banner("src/numbers/hash-seed.h.js", "src/numbers/hash-seed.{h,cc}", "默认哈希种子数据与派生算法");
rust += `
// 注: 种子布局常量由 hash_seed.rs (tool/src/numbers/hash-seed.cc.js) 覆盖。

/// 原 RAPIDHASH_DEFAULT_SECRET[3] 展开
pub const RAPIDHASH_DEFAULT_SECRET: [u64; 3] = [
${secrets.map((s) => `  0x${s.toString(16).toUpperCase()}_u64,`).join("\n")}
];

/// 数组索引哈希截断位宽 (原 Name::kArrayIndexValueBits, src/objects/name.h)
pub const ARRAY_INDEX_VALUE_BITS: u32 = ${K_ARRAY_INDEX_VALUE_BITS};
/// 截断掩码 (原 Name::kArrayIndexValueMask)
pub const ARRAY_INDEX_VALUE_MASK: u32 = (1 << ${K_ARRAY_INDEX_VALUE_BITS}) - 1;

/// 原 kDefaultSeed (V8_ENABLE_SEEDED_ARRAY_INDEX_HASH 分支, 含派生乘数):
/// m = truncate(secret) | 1; m_inv = modular_inverse(m) mod 2^26
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct HashSeedData {
  pub seed: u64,
  pub secrets: [u64; 3],
  pub m1: u32,
  pub m1_inv: u32,
  pub m2: u32,
  pub m2_inv: u32,
  pub m3: u32,
  pub m3_inv: u32,
}

/// 原版 kDefaultSeed 终态 (seed = 0, secrets 取默认三元组)
pub const DEFAULT_SEED: HashSeedData = HashSeedData {
  seed: 0,
  secrets: RAPIDHASH_DEFAULT_SECRET,
  m1: 0x${derived[0].m.toString(16).toUpperCase()},
  m1_inv: 0x${derived[0].inv.toString(16).toUpperCase()},
  m2: 0x${derived[1].m.toString(16).toUpperCase()},
  m2_inv: 0x${derived[1].inv.toString(16).toUpperCase()},
  m3: 0x${derived[2].m.toString(16).toUpperCase()},
  m3_inv: 0x${derived[2].inv.toString(16).toUpperCase()},
};

/// 原 modular_inverse (Newton 迭代, 模 2^32)
#[inline]
#[must_use]
pub const fn modular_inverse(m: u32) -> u32 {
  let mut x = (3u32.wrapping_mul(m)) ^ 2;
  x = x.wrapping_mul(2u32.wrapping_sub(m.wrapping_mul(x)));
  x = x.wrapping_mul(2u32.wrapping_sub(m.wrapping_mul(x)));
  x = x.wrapping_mul(2u32.wrapping_sub(m.wrapping_mul(x)));
  x.wrapping_mul(2u32.wrapping_sub(m.wrapping_mul(x)))
}

/// 原 is_modular_inverse 校验
#[inline]
#[must_use]
pub const fn is_modular_inverse(m: u32, m_inv: u32) -> bool {
  (m.wrapping_mul(m_inv) & ARRAY_INDEX_VALUE_MASK) == 1
}

// 原版 HashSeed::InitializeRoots 语义面 (随机种子/标志覆盖/根写入) 登记跳过。

// —— 提取不变式断言 ——
const _: () = {
  assert!(RAPIDHASH_DEFAULT_SECRET[0] == 0x2d358dccaa6c78a5);
  assert!(DEFAULT_SEED.m1 & 1 == 1);
  assert!(is_modular_inverse(DEFAULT_SEED.m1, DEFAULT_SEED.m1_inv));
  assert!(is_modular_inverse(DEFAULT_SEED.m2, DEFAULT_SEED.m2_inv));
  assert!(is_modular_inverse(DEFAULT_SEED.m3, DEFAULT_SEED.m3_inv));
};
`;

const p = writeSrc("v8-numbers", "hash_seed_cc_generated.rs", rust);
console.log(`written ${p} (secret 3, 派生对 3)`);
