#!/usr/bin/env bun
// tool/src/objects/api-callbacks.h.js — 覆盖 v8/src/objects/api-callbacks.{h,tq}
// AccessorInfoFlags / InterceptorInfoFlags 位域 (tq bitfield struct 展开),
// InterceptorInfo::Flag 掩码枚举, 回调列表 X-macro 展开
// → v8-objects-rest/src/api_callbacks.rs

import { readV8, writeSrc, banner } from "../../lib.js";

const hSrc = readV8("src/objects/api-callbacks.h");
const tqSrc = readV8("src/objects/api-callbacks.tq");

// —— tq bitfield struct 解析: name: type: N bit; → {name,type,size,offset,mask} ——
function parseBitfield(structName) {
  const m = tqSrc.match(new RegExp(`bitfield struct ${structName} extends (uint\\d+) \\{([^}]*)\\}`));
  if (!m) throw new Error(`未找到 bitfield struct ${structName}`);
  const fields = [];
  let off = 0;
  for (const fm of m[2].matchAll(/(\w+):\s*([\w|]+):\s*(\d+)\s*bit/g)) {
    const size = Number(fm[3]);
    fields.push({ name: fm[1], type: fm[2], size, offset: off, mask: ((1 << size) - 1) << off });
    off += size;
  }
  if (fields.length === 0) throw new Error(`${structName} 字段为空`);
  return { base: m[1], fields, bits: off };
}

const accFlags = parseBitfield("AccessorInfoFlags");
const itcFlags = parseBitfield("InterceptorInfoFlags");
if (accFlags.fields.length !== 5) throw new Error(`AccessorInfoFlags 字段数不符: 期望 5, 得 ${accFlags.fields.length}`);
if (itcFlags.fields.length !== 6) throw new Error(`InterceptorInfoFlags 字段数不符: 期望 6, 得 ${itcFlags.fields.length}`);

// —— InterceptorInfo::Flag 枚举 (h 侧, kXxx = XxxBit::kMask / kNone) ——
const flagMatch = hSrc.match(/enum Flag : uint32_t \{([\s\S]*?)\n  \};/);
if (!flagMatch) throw new Error("未找到 InterceptorInfo::Flag 枚举");
const flagEntries = [...flagMatch[1].matchAll(/k(\w+) = ([^,\n]+)[,\n]/g)].map((m) => ({ name: m[1], rhs: m[2].trim() }));
if (flagEntries.length !== 7) throw new Error(`Flag 条数不符: 期望 7 (含 kNone), 得 ${flagEntries.length}`);
// kNone 之外每个 Flag 对应一个 tq 位域字段 (snake_case)
const snake = (s) => s.replace(/([a-z0-9])([A-Z])/g, "$1_$2").toLowerCase();
for (const fe of flagEntries.slice(1)) {
  const f = itcFlags.fields.find((x) => x.name === snake(fe.name));
  if (!f) throw new Error(`Flag ${fe.name} 无对应 InterceptorInfoFlags 字段`);
  fe.mask = f.mask;
}
if (flagEntries[0].name !== "None") throw new Error("Flag 首项应为 kNone");

// —— 回调列表 X-macro ——
// X-macro 展开 (递归处理嵌套宏引用, 如 INDEXED 首行引 COMMON)
const listSrc = (name, seen = new Set()) => {
  if (seen.has(name)) throw new Error(`宏循环引用: ${name}`);
  seen.add(name);
  const m = hSrc.match(new RegExp(`#define ${name}\\(V\\)([\\s\\S]*?)\\n\\n`));
  if (!m) throw new Error(`未找到 ${name}`);
  const out = [];
  const body = m[1].replace(/\\/g, " ");
  for (const line of body.split("\n")) {
    const t = line.trim();
    if (!t) continue;
    const nested = t.match(/^(\w+)\(V\)$/);
    if (nested) out.push(...listSrc(nested[1], seen));
    else for (const x of t.matchAll(/V\((\w+),\s*(\w+)\)/g)) out.push({ upper: x[1], lower: x[2] });
  }
  return out;
};
const commonList = listSrc("COMMON_INTERCEPTOR_INFO_CALLBACK_LIST");
const indexedList = listSrc("INDEXED_INTERCEPTOR_INFO_CALLBACK_LIST");
if (commonList.length !== 7) throw new Error(`COMMON 列表条数不符: 期望 7, 得 ${commonList.length}`);
if (indexedList.length !== 9) throw new Error(`INDEXED 列表条数不符: 期望 9, 得 ${indexedList.length}`);
const indexedOnly = indexedList.filter((x) => !commonList.some((c) => c.upper === x.upper));
if (indexedOnly.length !== 2) throw new Error(`INDEXED 独有条数不符: 期望 2, 得 ${indexedOnly.length}`);

// —— InterceptorInfo::kFlagCount ——
const flagCount = Number(hSrc.match(/static constexpr int kFlagCount = (\d+);/)?.[1] ?? 0);
if (flagCount !== 6) throw new Error(`kFlagCount 不符: ${flagCount}`);

console.log(`AccessorInfoFlags ${accFlags.fields.length} 字段/${accFlags.bits} 位, InterceptorInfoFlags ${itcFlags.fields.length} 字段/${itcFlags.bits} 位, Flag ${flagEntries.length}`);

// —— Rust 生成 ——
const emitBitfield = (name, bf, doc) => {
  let s = `/// ${doc}\n#[repr(transparent)]\n#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]\npub struct ${name}(pub u32);\n\nimpl ${name} {\n`;
  for (const f of bf.fields) {
    const C = f.name.replace(/(^|_)(\w)/g, (_, a, b) => b.toUpperCase());
    s += `  /// ${f.name}: ${f.type}, 位 ${f.offset}..${f.offset + f.size} (宽 ${f.size})\n`;
    s += `  pub const ${C}_SHIFT: u32 = ${f.offset};\n`;
    s += `  pub const ${C}_SIZE: u32 = ${f.size};\n`;
    s += `  pub const ${C}_MASK: u32 = 0x${f.mask.toString(16)};\n`;
  }
  s += `
  /// 原始位串
  #[inline]
  #[must_use]
  pub const fn bits(self) -> u32 {
    self.0
  }
`;
  for (const f of bf.fields) {
    const C = f.name.replace(/(^|_)(\w)/g, (_, a, b) => b.toUpperCase());
    const sn = f.name;
    if (f.type === "bool") {
      s += `
  /// 读取 ${sn} 位 (原版 ${name.toLowerCase().replace(/flags/, "")}${sn})
  #[inline]
  #[must_use]
  pub const fn ${sn}(self) -> bool {
    self.0 & Self::${C}_MASK != 0
  }
`;
    } else {
      s += `
  /// 读取 ${sn} 字段 (宽 ${f.size}, 原版 ${sn})
  #[inline]
  #[must_use]
  pub const fn get_${sn}(self) -> u32 {
    (self.0 & Self::${C}_MASK) >> Self::${C}_SHIFT
  }
`;
    }
  }
  s += `}\n\n`;
  return s;
};

let rust = banner("src/objects/api-callbacks.h.js", "src/objects/api-callbacks.{h,tq}", "API 回调对象: 位域与回调列表");

rust += "\n" + emitBitfield("AccessorInfoFlags", accFlags, "AccessorInfo 标志位 (tq bitfield struct AccessorInfoFlags extends " + accFlags.base + ")");
rust += emitBitfield("InterceptorInfoFlags", itcFlags, "InterceptorInfo 标志位 (tq bitfield struct InterceptorInfoFlags extends " + itcFlags.base + ")");

rust += `/// InterceptorInfo::Flag 掩码枚举 (原版 enum Flag : uint32_t)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct InterceptorFlag(pub u32);

impl InterceptorFlag {
  /// kNone
  pub const NONE: Self = Self(0);
`;
for (const fe of flagEntries.slice(1)) {
  rust += `  /// k${fe.name} = 对应位域掩码\n  pub const ${fe.name.replace(/(^|_)(\w)/g, (_, a, b) => b.toUpperCase())}: Self = Self(0x${fe.mask.toString(16)});\n`;
}
rust += `
  /// 位串
  #[inline]
  #[must_use]
  pub const fn bits(self) -> u32 {
    self.0
  }

  /// 合并
  #[inline]
  #[must_use]
  pub const fn union(self, other: Self) -> Self {
    Self(self.0 | other.0)
  }

  /// 含判定
  #[inline]
  #[must_use]
  pub const fn contains(self, other: Self) -> bool {
    self.0 & other.0 == other.0
  }
}

/// 位域总数 (原版 InterceptorInfo::kFlagCount)
pub const INTERCEPTOR_FLAG_COUNT: usize = ${flagCount};

/// 命名/索引拦截器共有回调 (原版 COMMON_INTERCEPTOR_INFO_CALLBACK_LIST 展开)
pub const COMMON_INTERCEPTOR_CALLBACKS: [&str; ${commonList.length}] = [
${commonList.map((c) => `  "${c.upper}",`).join("\n")}
];

/// 索引拦截器全量回调 (原版 INDEXED_INTERCEPTOR_INFO_CALLBACK_LIST 展开)
pub const INDEXED_INTERCEPTOR_CALLBACKS: [&str; ${indexedList.length}] = [
${indexedList.map((c) => `  "${c.upper}",`).join("\n")}
];

/// 索引拦截器独有回调 (IndexOf / IterableToList)
pub const INDEXED_ONLY_CALLBACKS: [&str; ${indexedOnly.length}] = [
${indexedOnly.map((c) => `  "${c.upper}",`).join("\n")}
];

/// 副作用类型别名 (原版 type SideEffectType extends int32, 枚举本体在 debug-interface.h)
pub type SideEffectType = i32;

// —— 提取不变式断言 ——
const _: () = {
  assert!(INTERCEPTOR_FLAG_COUNT == 6);
  assert!(InterceptorFlag::NONE.bits() == 0);
  assert!(InterceptorFlag::NON_MASKING.contains(InterceptorFlag::NON_MASKING));
  assert!(InterceptorInfoFlags::default().bits() == 0);
  assert!(AccessorInfoFlags::IS_SLOPPY_MASK == 0x1);
  assert!(COMMON_INTERCEPTOR_CALLBACKS.len() == 7);
  assert!(INDEXED_ONLY_CALLBACKS.len() == 2);
};
`;

const p = writeSrc("v8-objects-rest", "api_callbacks.rs", rust);
console.log(`written ${p}`);
