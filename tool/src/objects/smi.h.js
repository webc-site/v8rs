#!/usr/bin/env bun
// tool/src/objects/smi.h.js — smi.rs 差集骨架生成
// 组→文件映射 (本脚本管辖):
//   v8/src/objects/smi.h + smi-inl.h → v8-objects-core/src/smi.rs (差集)
//   v8/src/objects/objects.cc Smi::LexicographicCompare (6482-6559) → 同上
// 产物: v8-objects-core/src/smi.rs.skel (差集骨架; smi.rs 已有落地实现, 直出会覆盖修订)
// 规则: kPowersOf10 表从 objects.cc 机械提取; Smi 移位常量沿用 tagged_constants
import { readV8, writeSrc, banner } from "../../lib.js";

const smiH = readV8("src/objects/smi.h");
const objectsCc = readV8("src/objects/objects.cc");

// 锚点断言: smi.h 静态方法清单
const methods = [...smiH.matchAll(/static (?:inline )?(?:constexpr )?(?:inline )?(?:constexpr )?\S[\w<>:* ,]*\s(\w+)\(/g)].map((m) => m[1]);
for (const need of ["ToUint32Smi", "ToInt", "ToUInt", "IsValid", "FromInt", "FromUInt", "FromIntptr", "From31BitPattern", "To31BitPattern", "FromEnum", "zero", "uninitialized_deserialization_value", "LexicographicCompare"])
  if (!methods.includes(need)) throw new Error(`smi.h 方法 ${need} 缺失: ${methods}`);

// objects.cc: kPowersOf10 静态表机械提取
const tableMatch = objectsCc.match(
  /static const uint32_t kPowersOf10\[\] = \{([\s\S]*?)\};/,
);
if (!tableMatch) throw new Error("kPowersOf10 表提取失败");
// 表项含 "10 * 1000" 形式乘积, 按项求积 (仅数字与 * 的安全求值)
const powers = tableMatch[1]
  .split(",")
  .map((s) => s.trim())
  .filter((s) => s.length > 0)
  .map((s) => {
    if (!/^[\d\s*]+$/.test(s)) throw new Error(`非法表项: ${s}`);
    return s.split("*").reduce((acc, f) => acc * Number(f.trim()), 1);
  });
if (powers.length !== 10) throw new Error(`kPowersOf10 应为 10 项, 实得 ${powers.length}`);
// 对数核对: 10 的幂
powers.forEach((v, i) => {
  if (v !== 10 ** i) throw new Error(`kPowersOf10[${i}] 应为 ${10 ** i}, 实得 ${v}`);
});

const body = `${banner("src/objects/smi.h.js", "src/objects/smi.h + objects.cc LexicographicCompare", "smi.rs 差集骨架, 合并后就地修订")}
//! 差集骨架: 转换辅助 + 字典序比较

impl Smi {
  /// Tagged<Object> → int (对应 ToInt)
  pub const fn to_int(object: Tagged<Object>) -> i32;
  /// 正 Smi → u32 (对应 ToUInt, DCHECK >= 0)
  pub const fn to_uint(object: Tagged<Object>) -> u32;
  /// u32 → Smi (对应 FromUInt)
  pub const fn from_uint(value: u32) -> Tagged<Self>;
  /// intptr → Smi (对应 FromIntptr, 超界返回 None)
  pub const fn from_intptr(value: i64) -> Option<Tagged<Self>>;
  /// 枚举 → Smi (对应 FromEnum)
  pub const fn from_enum(value: i32) -> Tagged<Self>;
  /// 未初始化反序列化哨兵 = kNullAddress (对应 uninitialized_deserialization_value)
  pub const fn uninitialized_deserialization_value() -> Tagged<Self>;
  /// 字典序比较两个 Smi 的十进制字符串表示 (对应 LexicographicCompare;
  /// 原版 isolate 参数仅用于禁 JS 执行, 与算法无关, 省略)
  pub const fn lexicographic_compare(x: Tagged<Self>, y: Tagged<Self>) -> Tagged<Self>;
}

/// 10 的幂表 (objects.cc 机械提取)
const POWERS_OF_10: [u32; 10] = [${powers.join(", ")}];
`;

const p = writeSrc("v8-objects-core", "smi.rs.skel", body);
console.log(`smi.h.js: 差集骨架 ${p} (方法 ${methods.length} 项, kPowersOf10 ${powers.length} 项核对通过)`);
