#!/usr/bin/env bun
// tool/src/objects/turbofan-types.h.js — 覆盖 v8/src/objects/turbofan-types.{h,-inl.h,tq}
// → v8-objects-rest/src/turbofan_types.rs
// 机械提取: TurbofanTypeLowBits (32 位链) + TurbofanTypeHighBits (4 位链) → 显式掩码
// 类层级与 SizeFor 语义面登记跳过。

import { readV8, writeSrc, banner } from "../../lib.js";

const hSrc = readV8("src/objects/turbofan-types.h");

// 位链解析: using XBit = base::BitField<bool, 0, 1, uint32_t> | Prev::Next<bool, 1>
function parseChain(namespaceName, enumName) {
  const ns = hSrc.match(new RegExp(`class ${namespaceName}[\\s\\S]*?enum ${enumName} : uint32_t \\{([\\s\\S]*?)\\n  \\};`));
  if (!ns) throw new Error(`未找到 ${namespaceName}::${enumName}`);
  const flagBody = ns[1];
  const flags = [...flagBody.matchAll(/^\s*(k\w+) = ([^,\n]+),?\s*$/gm)].map((m) => ({ name: m[1], expr: m[2].trim() }));
  if (flags[0].name !== "kNone") throw new Error(`${namespaceName} 首条应 kNone`);
  // 位链声明 (类起点至 enum 之间的 using 块)
  const chainBlock = hSrc.slice(ns.index, ns.index + ns[0].indexOf("enum Flag"));
  const bits = [];
  let pos = 0;
  for (const m of chainBlock.matchAll(/using (\w+?Bit) = base::BitField<bool, 0, 1, uint32_t>|using (\w+?Bit) = \w+::Next<bool, 1>/g)) {
    bits.push({ name: m[1] ?? m[2], start: pos });
    pos += 1;
  }
  return { flags, bits };
}

const low = parseChain("TurbofanTypeLowBits", "Flag");
const high = parseChain("TurbofanTypeHighBits", "Flag");

// 对数锚 (原版 kFlagCount)
const lowCount = hSrc.match(/static constexpr int kFlagCount = (\d+);/);
const highCount = hSrc.match(/static constexpr int kFlagCount = 4;/);
if (!lowCount || lowCount[1] !== "32") throw new Error("LowBits kFlagCount != 32 锚失配");
if (!highCount) throw new Error("HighBits kFlagCount = 4 锚缺失");
if (low.flags.length !== 33) throw new Error(`LowBits Flag 条数 ${low.flags.length} != 33 (kNone + 32)`);
if (high.flags.length !== 5) throw new Error(`HighBits Flag 条数 ${high.flags.length} != 5 (kNone + 4)`);
if (low.bits.length !== 32) throw new Error(`LowBits 位链 ${low.bits.length} != 32 段`);
if (high.bits.length !== 4) throw new Error(`HighBits 位链 ${high.bits.length} != 4 段`);

// Flag 表达式 = Bit::kMask → 位号校验 + 掩码求值
function resolveMask(flags, bits, label) {
  const byName = new Map(bits.map((b) => [b.name, b]));
  const out = [];
  for (const f of flags) {
    if (f.expr === "0") { out.push({ name: f.name, mask: 0 }); continue; }
    const bitName = f.expr.match(/^(\w+?)::kMask$/)?.[1];
    if (!bitName) throw new Error(`${label} Flag 表达式不可解析: ${f.expr}`);
    const bit = byName.get(bitName);
    if (!bit) throw new Error(`${label} ${f.name} 无位段 ${bitName}`);
    bit.flag = f.name;
    out.push({ name: f.name, mask: (1 << bit.start) >>> 0 });
  }
  return out;
}
const lowFlags = resolveMask(low.flags, low.bits, "LowBits");
const highFlags = resolveMask(high.flags, high.bits, "HighBits");

// 全 32 位恰好用满 (kFlagCount=32 位链无空洞)
if (new Set(low.bits.map((b) => b.start)).size !== 32) throw new Error("LowBits 位号重复");

const pascal = (s) => s.replace(/^k/, "").toLowerCase().replace(/(^|_)([a-z0-9])/g, (_, p, c) => c.toUpperCase());

function emit(label, flags, bits) {
  let s = `// ── ${label} ──────────────────────────────────────────────────────────────────\n`;
  for (const b of bits) {
    s += `/// 原版 ${b.name} (bit ${b.start})。\npub const ${b.name.replace(/([a-z0-9])([A-Z])/g, "$1_$2").toUpperCase()}_MASK: u32 = 0x${(1 << b.start).toString(16)};\n`;
  }
  s += `\n/// ${label} 标志 (原版 enum Flag; kNone = 0)。\n#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]\n#[repr(u32)]\npub enum ${label}Flag {\n  /// kNone\n  None = 0,\n`;
  for (const f of flags) {
    if (f.mask === 0) continue;
    s += `  /// ${f.name}\n  ${pascal(f.name)} = 0x${f.mask.toString(16)},\n`;
  }
  s += `}\n\n`;
  return s;
}

let rust = banner("src/objects/turbofan-types.h.js", "src/objects/turbofan-types.{h,-inl.h,tq}", "TurbofanType 位集标志链展开 (低 32 位 + 高 4 位)");
rust += `//! TurbofanType 位集 (类型反馈的位标志编码)。
//! 类层级 (TurbofanBitset/RangeType/...) 与 SizeFor 语义面登记跳过。
//! tq 侧类型清单 (turbofan-types.tq) 与本表 1:1 对应, 由 Developer 就地接线。

`;
rust += emit("TurbofanTypeLowBits", lowFlags, low.bits);
rust += emit("TurbofanTypeHighBits", highFlags, high.bits);
rust += `/// 低 32 位段位总数 (原版 kFlagCount)。
pub const LOW_FLAG_COUNT: u32 = 32;
/// 高位段位总数 (原版 kFlagCount)。
pub const HIGH_FLAG_COUNT: u32 = 4;

// —— 提取不变式断言 ——
const _: () = {
  assert!(LOW_FLAG_COUNT == 32);
  assert!(HIGH_FLAG_COUNT == 4);
  assert!(TURBOFAN_TYPE_LOW_BITS_UNUSED_PADDING_FIELD1_MASK == 0x1);
  assert!(TURBOFAN_TYPE_LOW_BITS_SANDBOXED_POINTER_MASK == 0x80000000);
  assert!(TURBOFAN_TYPE_HIGH_BITS_MACHINE_MASK == 0x1);
  assert!(TURBOFAN_TYPE_HIGH_BITS_TYPED_ARRAY_MASK == 0x8);
};
`;

const p = writeSrc("v8-objects-rest", "turbofan_types.rs", rust);
console.log(`turbofan_types.rs: LowBits 32 位链 + HighBits 4 位链展开, 对数通过`);
