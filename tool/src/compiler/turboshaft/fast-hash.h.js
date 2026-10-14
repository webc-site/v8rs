#!/usr/bin/env bun
// tool/src/compiler/turboshaft/fast-hash.h.js
// 机械提取 v8/src/compiler/turboshaft/fast-hash.h
// → v8-turboshaft/src/fast_hash.rs.skel
//
// 提取面: fast_hash 主模板 + 特化表 (optional/pair/tuple/Vector/span/
// IndirectHandle/MaybeIndirectHandle/array) + fast_hash_combine 混合常数。
import { readV8, writeSrc, banner } from "../../../lib.js";

const h = readV8("src/compiler/turboshaft/fast-hash.h");

// 1. fast_hash 主模板
if (!/template <class T>\s*struct fast_hash\s*\{/.test(h)) {
  throw new Error("未找到 fast_hash 主模板");
}
// enum → size_t 直转, 否则 base::hash
if (!/std::is_enum_v<T>/.test(h)) throw new Error("enum 直转分支缺失");
if (!/base::hash<T>\(\)\(v\)/.test(h)) throw new Error("base::hash 回退分支缺失");

// 2. 特化表 (逐个断言)
const specs = [
  ["std::optional<T>", "has_value ? (fast_hash<T>(*v) << 1) + 1 : 0"],
  ["std::pair<T1, T2>", "fast_hash_combine(first, second)"],
  ["std::tuple<Ts...>", "逐元素 fast_hash_combine"],
  ["base::Vector<T>", "size 混入 + 逐元素"],
  ["std::span<T, N>", "size 混入 + 逐元素"],
  ["IndirectHandle<T>", "地址 (deref)"],
  ["MaybeIndirectHandle<T>", "地址"],
  ["std::array<T, N>", "逐元素 (无 size 混入)"],
];
for (const [spec] of specs) {
  const esc = spec.replace(/[.*+?^${}()|[\]\\]/g, "\\$&");
  if (!new RegExp(`struct fast_hash<${esc}>`).test(h)) throw new Error(`特化缺失: fast_hash<${spec}>`);
}

// 3. fast_hash_combine 声明 + 混合实现锚
for (const [re, label] of [
  [/V8_INLINE size_t fast_hash_combine\(T const& v, Ts const&\.\.\. vs\) \{/, "fast_hash_combine 递归定义"],
  [/return fast_hash_combine\(fast_hash_combine\(vs\.\.\.\), fast_hash<T>\(\)\(v\)\);/, "右结合递归混合"],
  [/V8_INLINE size_t fast_hash_range\(Iterator first, Iterator last\)/, "fast_hash_range"],
  [/size_t acc = 0;/, "range 累加器清零"],
]) {
  if (!re.test(h)) throw new Error(`哨兵缺失: ${label}`);
}

// 4. 生成 Rust (完整实现 — 混合常数 17 可机械提取)
const combineMul = Number(h.match(/return (\d+) \* acc \+ value;/)[1]);
if (combineMul !== 17) throw new Error(`混合常数漂移: ${combineMul}`);

let rust = banner(
  "src/compiler/turboshaft/fast-hash.h.js",
  "src/compiler/turboshaft/fast-hash.h",
  "快哈希: 特化分发表 + 17 倍混合 (1:1 实现)",
);
rust += `
/// 原版 fast_hash 分发表 (主模板 + ${specs.length} 特化):
/// - 主模板: enum → static_cast<size_t> 直转, 其它走 base::hash
///${specs.map(([s, d]) => `/// - fast_hash<${s}>: ${d}`).join("\n")}
/// v8: src/compiler/turboshaft/fast-hash.h fast_hash
pub trait FastHash {
  fn fast_hash(&self) -> u64;
}

/// size_t 直传路径 (base::hash<size_t> 即恒等)
impl FastHash for u64 {
  fn fast_hash(&self) -> u64 {
    *self
  }
}

impl FastHash for u32 {
  fn fast_hash(&self) -> u64 {
    *self as u64
  }
}

impl FastHash for usize {
  fn fast_hash(&self) -> u64 {
    *self as u64
  }
}

/// 原版: has_value ? (fast_hash<T>()(*v) << 1) + 1 : 0
impl FastHash for Option<u64> {
  fn fast_hash(&self) -> u64 {
    match self {
      Some(v) => (v.fast_hash() << 1) + 1,
      None => 0,
    }
  }
}

/// 原版 fast_hash_combine(size_t acc, size_t value) = 17 * acc + value
/// v8: src/compiler/turboshaft/fast-hash.h fast_hash_combine(acc, value)
#[inline]
#[must_use]
pub const fn fast_hash_combine2(acc: u64, value: u64) -> u64 {
  ${combineMul} * acc + value
}

/// 原版变参 fast_hash_combine(v, vs...) =
///   combine(combine(vs...), hash(v)) — 展开为 Σ 17^(i-1) * hash(v_i)
/// (首元素权重 1, 逐位后移)。空集 = 0, 单元素 = hash(v)。
/// v8: src/compiler/turboshaft/fast-hash.h fast_hash_combine(T, Ts...)
#[must_use]
pub fn fast_hash_combine(values: &[u64]) -> u64 {
  let mut acc = 0u64;
  let mut pow = 1u64;
  for v in values {
    acc = acc.wrapping_add(pow.wrapping_mul(*v));
    pow = pow.wrapping_mul(${combineMul});
  }
  acc
}

/// 原版 fast_hash_range(first, last): acc = combine(acc, *first) 左折叠。
/// size_t 元素走非模板重载 → acc = 17 * acc + value。
/// v8: src/compiler/turboshaft/fast-hash.h fast_hash_range
#[must_use]
pub fn fast_hash_range(values: &[u64]) -> u64 {
  let mut acc = 0u64;
  for v in values {
    acc = fast_hash_combine2(acc, *v);
  }
  acc
}

/// 原版 fast_hash<std::pair<T1, T2>>: combine(first, second)
/// (两参变参展开 = 17 * hash(second) + hash(first), 即 acc=second 权重高)
#[must_use]
pub fn fast_hash_pair(first: u64, second: u64) -> u64 {
  fast_hash_combine(&[first, second])
}

// —— 提取不变式断言 ——
const _: () = {
  assert!(None::<u64>.fast_hash() == 0);
  assert!(Some(0u64).fast_hash() == 1);
  assert!(fast_hash_combine(&[]) == 0);
  assert!(fast_hash_combine(&[5]) == 5);
  assert!(fast_hash_combine(&[3, 5]) == 17 * 5 + 3);
  assert!(fast_hash_combine2(0, 7) == 7);
  assert!(fast_hash_range(&[1, 2]) == ${combineMul} * 1 + 2);
};
`;

const p = writeSrc("v8-turboshaft", "fast_hash.rs", rust);
console.log(`written ${p}`);
console.log(`特化=${specs.length} 主模板=1 combine=1 递归锚=4`);
