#!/usr/bin/env bun
// tool/src/objects/call-site-info.h.js — 覆盖 v8/src/objects/call-site-info.{h,cc}
// CallSiteInfo 位域链 / Flag 枚举 / Fields 枚举 / 延迟帧掩码
// 构建分支: V8_ENABLE_DRUMBRAKE 默认关闭, 按非 DRUMBRAKE 分支展开 (8 位变体注释登记)
// → v8-objects-js-func/src/call_site_info.rs

import { readV8, writeSrc, banner } from "../../lib.js";

const hSrc = readV8("src/objects/call-site-info.h");
const globalsSrc = readV8("src/common/globals.h");

// 1. BitField 链: 首项 base::BitField<T, off, size>, 后续 PrevBit::Next<T, size>
const chain = hSrc.match(/using IsWasmBit = base::BitField<[\s\S]*?kIsDeferredBaselineFrame = IsDeferredBaselineFrameBit::kMask,?/);
if (!chain) throw new Error("未找到 CallSiteInfo 位域链");

const fields = [];
let off = 0;
for (const line of chain[0].split("\n")) {
  const base = line.match(/using (\w+)Bit = base::BitField<[^,]+,\s*(\d+),\s*(\d+)/);
  const next = line.match(/using (\w+)Bit = (\w+)Bit::Next<[^,]+,\s*(\d+)>/);
  if (base) {
    const [, name, o, s] = base;
    if (Number(o) !== off) throw new Error(`位域 ${name} 偏移断裂: 期望 ${off}, 得 ${o}`);
    fields.push({ name, size: Number(s), offset: off });
    off += Number(s);
  } else if (next) {
    const [, name, prev, s] = next;
    const p = fields[fields.length - 1];
    if (!p || p.name !== prev) throw new Error(`位域 ${name} 前驱不符: ${prev}`);
    fields.push({ name, size: Number(s), offset: off });
    off += Number(s);
  }
}
for (const f of fields) f.mask = ((1 << f.size) - 1) << f.offset;
// 非 DRUMBRAKE: IsWasmInterpretedFrame 属 #if 分支, 移至末尾登记不产常量
const drumIdx = fields.findIndex((f) => f.name === "IsWasmInterpretedFrame");
if (drumIdx !== -1) fields.splice(drumIdx, 1)[0];
if (fields.length !== 7) throw new Error(`位域数不符: 期望 7 (非 DRUMBRAKE), 得 ${fields.length}`);
// DRUMBRAKE 第 8 位登记
const drumbrakeField = "IsWasmInterpretedFrame";

// 2. Flag 枚举: kNone + 每 Bit::kMask
const flagMatch = hSrc.match(/enum Flag : uint32_t \{([\s\S]*?)\n  \};/);
if (!flagMatch) throw new Error("未找到 Flag 枚举");
const flagNames = [...flagMatch[1].matchAll(/k(\w+) = /g)].map((m) => m[1]);
// 非 DRUMBRAKE 展开: 过滤 kIsWasmInterpretedFrame
const activeFlags = flagNames.filter((n) => n !== "IsWasmInterpretedFrame");
if (activeFlags.length !== 8) throw new Error(`Flag 条数不符: 期望 8 (含 kNone), 得 ${activeFlags.length}`);
if (flagNames.length !== 9) throw new Error(`Flag 原始条数异常: ${flagNames.length}, 期望 9 (8 + DRUMBRAKE 变体)`);

// 3. Fields 枚举 (延迟捕获裸数组布局)
const fieldsMatch = hSrc.match(/enum Fields \{([^}]*)\};/);
if (!fieldsMatch) throw new Error("未找到 Fields 枚举");
const fieldsEnum = [...fieldsMatch[1].matchAll(/k(\w+)/g)].map((m) => m[1]);
if (fieldsEnum.length !== 6) throw new Error(`Fields 条数不符: 期望 6, 得 ${fieldsEnum.length}`);

// 4. kUnknown = kNoSourcePosition (定义在 globals.h, 跨文件机械提取)
const noSourcePos = Number(globalsSrc.match(/constexpr int kNoSourcePosition = (-?\d+);/)?.[1]);
if (!Number.isFinite(noSourcePos)) throw new Error("未在 globals.h 找到 kNoSourcePosition");

// 5. kDeferredFlagsMask = kIsDeferredBaselineFrame 掩码
const deferredField = fields.find((f) => f.name === "IsDeferredBaselineFrame");
if (!deferredField) throw new Error("未找到 IsDeferredBaselineFrame 位域");
const deferredMask = deferredField.mask;

// 6. kFlagCount (非 DRUMBRAKE 分支, 取 #else 后那个)
const fcAll = [...hSrc.matchAll(/#if V8_ENABLE_DRUMBRAKE\s*\n\s*static constexpr int kFlagCount = (\d+);\s*\n#else\s*\n\s*static constexpr int kFlagCount = (\d+);/g)];
if (fcAll.length !== 1 || Number(fcAll[0][2]) !== 7) throw new Error("kFlagCount 非 DRUMBRAKE 分支应为 7 (且 #if/#else 对唯一)");

console.log(`CallSiteInfo 位域 ${fields.length}, Flag ${activeFlags.length} (含 kNone), Fields ${fieldsEnum.length}`);

let rust = banner(
  "src/objects/call-site-info.h.js",
  "src/objects/call-site-info.{h,cc}",
  "CallSiteInfo 栈帧标志位域 (V8_ENABLE_DRUMBRAKE 关闭分支展开, 开启时追加 IsWasmInterpretedFrame 位 7)",
);

const pascal = (s) => s.replace(/(^|_)(\w)/g, (_, a, b) => b.toUpperCase());

rust += `
/// CallSiteInfo 标志位域 (原版 IsWasmBit..IsDeferredBaselineFrameBit BitField 链)
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct CallSiteFlags(pub u32);

#[allow(dead_code)]
impl CallSiteFlags {
`;

for (const f of fields) {
  const C = pascal(f.name);
  rust += `  /// ${f.name}Bit: 位 ${f.offset}, 宽 ${f.size}\n  pub const ${C}_SHIFT: u32 = ${f.offset};\n  pub const ${C}_MASK: u32 = 0x${f.mask.toString(16)};\n`;
}
rust += `
  /// 原始位串
  #[inline]
  #[must_use]
  pub const fn bits(self) -> u32 {
    self.0
  }
`;

for (const f of fields) {
  const C = pascal(f.name);
  const sn = f.name.replace(/([a-z0-9])([A-Z])/g, "$1_$2").toLowerCase();
  rust += `
  /// 读取 ${f.name} 位 (原版 ${f.name}Bit)
  #[inline]
  #[must_use]
  pub const fn ${sn}(self) -> bool {
    self.0 & Self::${C}_MASK != 0
  }
`;
}

rust += `}

/// Flag 掩码枚举 (原版 enum Flag : uint32_t)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct CallSiteFlag(pub u32);

#[allow(dead_code)]
impl CallSiteFlag {
  /// kNone
  pub const NONE: Self = Self(0);
`;
for (const n of activeFlags.slice(1)) {
  const f = fields.find((x) => x.name === n);
  if (!f) throw new Error(`Flag ${n} 无对应位域`);
  rust += `  /// k${n} = ${n}Bit::kMask\n  pub const ${pascal(n)}: Self = Self(0x${f.mask.toString(16)});\n`;
}
rust += `
  /// 含判定
  #[inline]
  #[must_use]
  pub const fn contains(self, other: Self) -> bool {
    self.0 & other.0 == other.0
  }

  /// 合并
  #[inline]
  #[must_use]
  pub const fn union(self, other: Self) -> Self {
    Self(self.0 | other.0)
  }
}

/// 位域总数, 非 DRUMBRAKE 分支 (原版 kFlagCount; DRUMBRAKE 开启时为 8)
pub const CALL_SITE_FLAG_COUNT: usize = ${Number(fcAll[0][2])};

/// 延迟帧原始捕获数组布局 (原版 enum Fields)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum CallSiteFields {
`;
fieldsEnum.forEach((n, i) => {
  rust += `  /// k${n}\n  ${pascal(n)} = ${i},\n`;
});
rust += `}

impl CallSiteFields {
  /// 布局槽数 (原版 kCount)
  pub const COUNT: usize = ${fieldsEnum.length};
}

/// 未知源位置哨兵 (原版 kUnknown = kNoSourcePosition)
pub const CALL_SITE_UNKNOWN: i32 = ${noSourcePos};

/// 延迟标志掩码, 仅在裸捕获数组中有效 (原版 kDeferredFlagsMask)
pub const DEFERRED_FLAGS_MASK: u32 = 0x${deferredMask.toString(16)};

// —— 提取不变式断言 ——
const _: () = {
  assert!(CALL_SITE_FLAG_COUNT == 7);
  assert!(DEFERRED_FLAGS_MASK == 0x${deferredMask.toString(16)});
  assert!(CALL_SITE_UNKNOWN == ${noSourcePos});
  assert!(CallSiteFields::COUNT == 6);
  assert!(CallSiteFlags::default().bits() == 0);
};
`;

const p = writeSrc("v8-objects-js-func", "call_site_info.rs", rust);
console.log(`written ${p}`);
