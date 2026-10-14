#!/usr/bin/env bun
// tool/src/roots/roots.h.js — 从 v8/src/roots/roots.h 提取并校验 RootIndex 与根表边界
// 生成: v8-roots/src/root_index.rs
import { spawnSync } from "node:child_process";
import { mkdirSync, readFileSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { dirname, join } from "node:path";
import { banner, readV8, root, writeSrc } from "../../lib.js";

const V8 = join(root, "v8");
const REL = "src/roots/roots.h";
const rawSrc = readV8(REL);

// ── 1. clang -E 宏预处理展开 ───────────────────────────────────────────────
const STUB = join(tmpdir(), "v8rs-roots-stub");
mkdirSync(STUB, { recursive: true });

function preprocess(file) {
  const out = join(tmpdir(), "v8rs-roots-expanded.ii");
  for (let i = 0; i < 200; i++) {
    const r = spawnSync(
      "clang",
      [
        "-E",
        "-P",
        "-x",
        "c++",
        "-std=c++20",
        "-DV8_COMPRESS_POINTERS",
        "-DV8_ENABLE_SANDBOX",
        "-DV8_EXTERNAL_CODE_SPACE",
        "-DV8_INTL_SUPPORT",
        "-DV8_ENABLE_WEBASSEMBLY",
        "-I",
        ".",
        "-I",
        "include",
        "-I",
        STUB,
        file,
        "-o",
        out,
      ],
      { cwd: V8, encoding: "utf8" }
    );
    if (r.status === 0) return readFileSync(out, "utf8");
    const miss = (r.stderr.match(/fatal error: '([^']+)' file not found/) || [])[1];
    if (!miss) throw new Error(`clang -E ${file} 失败:\n${r.stderr}`);
    const p = join(STUB, miss);
    mkdirSync(dirname(p), { recursive: true });
    writeFileSync(p, "#pragma once\n");
  }
  throw new Error(`clang -E ${file} 桩头迭代超限`);
}

console.log("正在执行 clang -E 宏预处理...");
const expanded = preprocess(REL);

const nameStub = join(tmpdir(), "v8rs-extract-names.cc");
writeFileSync(
  nameStub,
  `#include "src/roots/roots.h"
const char* const roots_names_table[] = {
#define ROOT_NAME(type, name, CamelName) #name,
  ROOT_LIST(ROOT_NAME)
#undef ROOT_NAME
};
`
);
const nameExpanded = preprocess(nameStub);
const nameMatch = nameExpanded.match(/roots_names_table\[\]\s*=\s*\{([\s\S]*?)\};/);
if (!nameMatch) throw new Error("未找到 roots_names_table 展开结果");
const rawRootNames = nameMatch[1]
  .split(",")
  .map((s) => s.trim().replace(/^"|"$/g, ""))
  .filter(Boolean);


// ── 2. 提取 RootIndex 枚举列表 ─────────────────────────────────────────────
const enumStart = expanded.indexOf("enum class RootIndex : uint16_t {");
if (enumStart === -1) throw new Error("未找到 enum class RootIndex");
const lengthIdx = expanded.indexOf("kRootListLength", enumStart);
if (lengthIdx === -1) throw new Error("未找到 kRootListLength");

const beforeLength = expanded.slice(enumStart, lengthIdx);
const rawEntries = beforeLength
  .slice(beforeLength.indexOf("{") + 1)
  .split(",")
  .map((s) => s.trim())
  .filter(Boolean);

// C++ kXxx → Rust PascalCase 变体 (upper camel case 惯例; 值/顺序 1:1 不变)
// 消歧: 同基名冲突时全小写源得基名, 含大写源 (如 kA_string / kpercentSign_string) 加 Uppercase 前缀
const rustVariant = (n) =>
  n.replace(/^k/, "").split("_").map((w) => (w ? w[0].toUpperCase() + w.slice(1) : w)).join("");
const baseCount = new Map();
for (const n of rawEntries) {
  const b = rustVariant(n);
  baseCount.set(b, (baseCount.get(b) ?? 0) + 1);
}
const variantOf = (n) => {
  const b = rustVariant(n);
  const body = n.slice(1);
  return baseCount.get(b) > 1 && body !== body.toLowerCase() ? "Uppercase" + b : b;
};

const ENTRIES_COUNT = rawEntries.length;
console.log(`已提取 RootIndex 枚举项数: ${ENTRIES_COUNT}`);

// ── 3. 提取并核对各分区计数 ─────────────────────────────────────────────────
function countPlusOnes(name) {
  const start = expanded.indexOf(name + " =", enumStart);
  if (start === -1) throw new Error(`未找到分区计数 ${name}`);
  const end = expanded.indexOf(",", start);
  const chunk = expanded.slice(start, end);
  return (chunk.match(/\+1/g) || []).length;
}

const READ_ONLY_COUNT = countPlusOnes("kReadOnlyRootsCount");
const IMMORTAL_IMMOVABLE_COUNT = countPlusOnes("kImmortalImmovableRootsCount") + READ_ONLY_COUNT;
const MUTABLE_COUNT = countPlusOnes("kMutableRootsCount");
const SMI_COUNT = ENTRIES_COUNT - READ_ONLY_COUNT - MUTABLE_COUNT;
const SINGLE_CHAR_STRING_COUNT = countPlusOnes("kSingleCharacterStringRootsCount");
const NAME_FOR_PROTECTOR_COUNT = countPlusOnes("kNameForProtectorCount");
const TRUSTED_OBJECT_MAP_COUNT = countPlusOnes("kTrustedObjectMapRootsCount");
const SERIALIZABLE_TRUSTED_OBJECT_MAP_COUNT = countPlusOnes("kSerializableTrustedObjectMapRootsCount");

const fail = (label, got, want) => {
  throw new Error(`对数失配 ${label}: got=${got} want=${want}`);
};

if (ENTRIES_COUNT !== 1157) fail("总根数 (kEntriesCount)", ENTRIES_COUNT, 1157);
if (READ_ONLY_COUNT !== 1044) fail("只读根数 (kReadOnlyRootsCount)", READ_ONLY_COUNT, 1044);
if (IMMORTAL_IMMOVABLE_COUNT !== 1123) fail("不可移动根数 (kImmortalImmovableRootsCount)", IMMORTAL_IMMOVABLE_COUNT, 1123);
if (MUTABLE_COUNT !== 105) fail("可变根数 (kMutableRootsCount)", MUTABLE_COUNT, 105);
if (SMI_COUNT !== 8) fail("Smi根数 (SMI_ROOT_LIST)", SMI_COUNT, 8);
if (SINGLE_CHAR_STRING_COUNT !== 256) fail("单字符字符串根数", SINGLE_CHAR_STRING_COUNT, 256);
if (NAME_FOR_PROTECTOR_COUNT !== 12) fail("保护符名称根数", NAME_FOR_PROTECTOR_COUNT, 12);
if (TRUSTED_OBJECT_MAP_COUNT !== 27) fail("受信任对象Map根数", TRUSTED_OBJECT_MAP_COUNT, 27);
if (SERIALIZABLE_TRUSTED_OBJECT_MAP_COUNT !== 7) fail("可序列化受信任对象Map根数", SERIALIZABLE_TRUSTED_OBJECT_MAP_COUNT, 7);

// 边界断言
const FIRST_READ_ONLY = 0;
const LAST_READ_ONLY = READ_ONLY_COUNT - 1; // 1043
const FIRST_STRONG = READ_ONLY_COUNT; // 1044
const LAST_STRONG = FIRST_STRONG + MUTABLE_COUNT - 1; // 1148
const FIRST_SMI = LAST_STRONG + 1; // 1149
const LAST_SMI = ENTRIES_COUNT - 1; // 1156

if (rawEntries[FIRST_READ_ONLY] !== "kUndefinedValue") fail("FIRST_READ_ONLY", rawEntries[FIRST_READ_ONLY], "kUndefinedValue");
if (rawEntries[LAST_READ_ONLY] !== "kCppGCManagedBaseMap") fail("LAST_READ_ONLY", rawEntries[LAST_READ_ONLY], "kCppGCManagedBaseMap");
if (rawEntries[FIRST_STRONG] !== "kArgumentsIteratorAccessor") fail("FIRST_STRONG", rawEntries[FIRST_STRONG], "kArgumentsIteratorAccessor");
if (rawEntries[IMMORTAL_IMMOVABLE_COUNT - 1] !== "kEmptyWasmDispatchTable") fail("LAST_IMMORTAL_IMMOVABLE", rawEntries[IMMORTAL_IMMOVABLE_COUNT - 1], "kEmptyWasmDispatchTable");
if (rawEntries[LAST_STRONG] !== "kErrorStackSetterSharedFun") fail("LAST_STRONG", rawEntries[LAST_STRONG], "kErrorStackSetterSharedFun");
if (rawEntries[FIRST_SMI] !== "kLastScriptId") fail("FIRST_SMI", rawEntries[FIRST_SMI], "kLastScriptId");
if (rawEntries[LAST_SMI] !== "kInterpreterEntryReturnPCOffset") fail("LAST_SMI", rawEntries[LAST_SMI], "kInterpreterEntryReturnPCOffset");

const asciiNulIdx = rawEntries.indexOf("kascii_nul_string");
if (asciiNulIdx !== 354) fail("kascii_nul_string index", asciiNulIdx, 354);
if (rawEntries[asciiNulIdx + 255] !== "klatin1_ff_string") fail("klatin1_ff_string", rawEntries[asciiNulIdx + 255], "klatin1_ff_string");

const constructorStrIdx = rawEntries.indexOf("kconstructor_string");
if (constructorStrIdx !== 1019) fail("kconstructor_string index", constructorStrIdx, 1019);
if (rawEntries[constructorStrIdx + 11] !== "kis_concat_spreadable_symbol") fail("kis_concat_spreadable_symbol", rawEntries[constructorStrIdx + 11], "kis_concat_spreadable_symbol");

const atomRegExpDataMapIdx = rawEntries.indexOf("kAtomRegExpDataMap");
if (atomRegExpDataMapIdx !== 100) fail("kAtomRegExpDataMap index", atomRegExpDataMapIdx, 100);

const bytecodeArrayMapIdx = rawEntries.indexOf("kBytecodeArrayMap");
if (bytecodeArrayMapIdx !== 120) fail("kBytecodeArrayMap index", bytecodeArrayMapIdx, 120);

const proxyRevokeIdx = rawEntries.indexOf("kProxyRevokeSharedFun");
if (proxyRevokeIdx !== 1089) fail("kProxyRevokeSharedFun index", proxyRevokeIdx, 1089);
const BUILTINS_WITH_SFI_COUNT = countPlusOnes("kLastBuiltinWithSfiRoot");
if (BUILTINS_WITH_SFI_COUNT !== 28) fail("BUILTINS_WITH_SFI_COUNT", BUILTINS_WITH_SFI_COUNT, 28);

const nanValueIdx = rawEntries.indexOf("kNanValue");
if (nanValueIdx !== 187) fail("kNanValue index", nanValueIdx, 187);
const smiMaxValuePlusOneIdx = rawEntries.indexOf("kSmiMaxValuePlusOne");
if (smiMaxValuePlusOneIdx !== 196) fail("kSmiMaxValuePlusOne index", smiMaxValuePlusOneIdx, 196);
const jsSharedArrayMapIdx = rawEntries.indexOf("kJSSharedArrayMap");
if (jsSharedArrayMapIdx !== 153) fail("kJSSharedArrayMap index", jsSharedArrayMapIdx, 153);

if (rawRootNames.length !== ENTRIES_COUNT) fail("根名称数量 (RootsTable::root_names_)", rawRootNames.length, ENTRIES_COUNT);
if (rawRootNames[0] !== "undefined_value") fail("rawRootNames[0]", rawRootNames[0], "undefined_value");
if (rawRootNames[ENTRIES_COUNT - 1] !== "interpreter_entry_return_pc_offset") fail("rawRootNames[last]", rawRootNames[ENTRIES_COUNT - 1], "interpreter_entry_return_pc_offset");

console.log("所有根数量与分区范围校验 100% 通过！");

// ── 4. 生成 v8-roots/src/root_index.rs ─────────────────────────────────────
const code = `${banner("src/roots/roots.h.js", "src/roots/roots.h", "RootIndex 1:1 对齐 V8 原版定义")}
use core::mem::transmute;

use crate::error::{Error, Result};

/// V8 全局固定根对象索引枚举 (1:1 镜像 C++ \`v8::internal::RootIndex\`)
///
/// 包含所有只读根 (Read-Only Roots)、可变强根 (Mutable Strong Roots) 与 Smi 根。
/// 底层表现为 \`u16\` 连续编号。
#[repr(u16)]
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum RootIndex {
${rawEntries.map((name, i) => `  ${variantOf(name)} = ${i},`).join("\n")}
}

impl RootIndex {
  /// 总根对象槽位数 (kRootListLength / kEntriesCount)
  pub const K_ENTRIES_COUNT: usize = ${ENTRIES_COUNT};

  /// 只读根对象总数 (kReadOnlyRootsCount)
  pub const K_READ_ONLY_ROOTS_COUNT: usize = ${READ_ONLY_COUNT};

  /// 不可移动根对象总数 (含只读根与可变不可移动根)
  pub const K_IMMORTAL_IMMOVABLE_ROOTS_COUNT: usize = ${IMMORTAL_IMMOVABLE_COUNT};

  /// 可变强根对象总数 (kMutableRootsCount)
  pub const K_MUTABLE_ROOTS_COUNT: usize = ${MUTABLE_COUNT};

  /// Smi 标记根对象总数 (SMI_ROOT_LIST)
  pub const K_SMI_ROOTS_COUNT: usize = ${SMI_COUNT};

  /// 单字符内部化字符串根数
  pub const K_SINGLE_CHARACTER_STRING_ROOTS_COUNT: usize = ${SINGLE_CHAR_STRING_COUNT};

  /// 用于 Protector 快速检查的标识符名称根数
  pub const K_NAME_FOR_PROTECTOR_COUNT: usize = ${NAME_FOR_PROTECTOR_COUNT};

  /// 受信任对象 Map 根数
  pub const K_TRUSTED_OBJECT_MAP_ROOTS_COUNT: usize = ${TRUSTED_OBJECT_MAP_COUNT};

  /// 可序列化受信任对象 Map 根数
  pub const K_SERIALIZABLE_TRUSTED_OBJECT_MAP_ROOTS_COUNT: usize = ${SERIALIZABLE_TRUSTED_OBJECT_MAP_COUNT};

  // ── 区域首尾索引常量 ──
  pub const K_FIRST_ROOT: Self = Self::${rawEntries[0]};
  pub const K_LAST_ROOT: Self = Self::${rawEntries[ENTRIES_COUNT - 1]};

  pub const K_FIRST_READ_ONLY_ROOT: Self = Self::${rawEntries[FIRST_READ_ONLY]};
  pub const K_LAST_READ_ONLY_ROOT: Self = Self::${rawEntries[LAST_READ_ONLY]};

  pub const K_FIRST_STRONG_ROOT: Self = Self::${rawEntries[FIRST_STRONG]};
  pub const K_LAST_STRONG_ROOT: Self = Self::${rawEntries[LAST_STRONG]};

  pub const K_FIRST_STRONG_OR_READ_ONLY_ROOT: Self = Self::${rawEntries[0]};
  pub const K_LAST_STRONG_OR_READ_ONLY_ROOT: Self = Self::${rawEntries[LAST_STRONG]};

  pub const K_FIRST_IMMORTAL_IMMOVABLE_ROOT: Self = Self::${rawEntries[0]};
  pub const K_LAST_IMMORTAL_IMMOVABLE_ROOT: Self = Self::${rawEntries[IMMORTAL_IMMOVABLE_COUNT - 1]};

  pub const K_FIRST_SMI_ROOT: Self = Self::${rawEntries[FIRST_SMI]};
  pub const K_LAST_SMI_ROOT: Self = Self::${rawEntries[LAST_SMI]};

  pub const K_FIRST_SINGLE_CHARACTER_STRING: Self = Self::${rawEntries[asciiNulIdx]};
  pub const K_LAST_SINGLE_CHARACTER_STRING: Self = Self::${rawEntries[asciiNulIdx + 255]};

  pub const K_FIRST_NAME_FOR_PROTECTOR: Self = Self::${rawEntries[constructorStrIdx]};
  pub const K_LAST_NAME_FOR_PROTECTOR: Self = Self::${rawEntries[constructorStrIdx + NAME_FOR_PROTECTOR_COUNT - 1]};

  pub const K_FIRST_TRUSTED_OBJECT_MAP: Self = Self::${rawEntries[atomRegExpDataMapIdx]};
  pub const K_LAST_TRUSTED_OBJECT_MAP: Self = Self::${rawEntries[atomRegExpDataMapIdx + TRUSTED_OBJECT_MAP_COUNT - 1]};

  pub const K_FIRST_SERIALIZABLE_TRUSTED_OBJECT_MAP: Self = Self::${rawEntries[bytecodeArrayMapIdx]};
  pub const K_LAST_SERIALIZABLE_TRUSTED_OBJECT_MAP: Self = Self::${rawEntries[bytecodeArrayMapIdx + SERIALIZABLE_TRUSTED_OBJECT_MAP_COUNT - 1]};

  pub const K_FIRST_BUILTIN_WITH_SFI_ROOT: Self = Self::${rawEntries[proxyRevokeIdx]};
  pub const K_LAST_BUILTIN_WITH_SFI_ROOT: Self = Self::${rawEntries[proxyRevokeIdx + BUILTINS_WITH_SFI_COUNT - 1]};
  pub const K_BUILTINS_WITH_SFI_ROOTS_COUNT: usize = ${BUILTINS_WITH_SFI_COUNT};

  pub const K_FIRST_HEAP_NUMBER_ROOT: Self = Self::${variantOf(rawEntries[nanValueIdx])};
  pub const K_LAST_HEAP_NUMBER_ROOT: Self = Self::${variantOf(rawEntries[smiMaxValuePlusOneIdx])};

  pub const K_FIRST_JS_RECEIVER_MAP_ROOT: Self = Self::${variantOf(rawEntries[jsSharedArrayMapIdx])};

  /// TDZ Hole 别名 (若未启用独立 TDZ Hole 则映射至 TheHoleValue)
  pub const K_TDZ_HOLE_VALUE: Self = Self::${variantOf("kTheHoleValue")};

  /// 从原始 u16 整数构造 RootIndex
  #[inline]
  pub const fn from_u16(val: u16) -> Option<Self> {
    if (val as usize) < Self::K_ENTRIES_COUNT {
      // SAFETY: 枚举定义包含 0..1157 连续所有变体，val < K_ENTRIES_COUNT 保证有效
      Some(unsafe { transmute::<u16, Self>(val) })
    } else {
      None
    }
  }

  /// 从原始 usize 整数构造 RootIndex
  #[inline]
  pub const fn from_usize(val: usize) -> Option<Self> {
    if val < Self::K_ENTRIES_COUNT {
      // SAFETY: 枚举定义包含 0..1157 连续所有变体
      Some(unsafe { transmute::<u16, Self>(val as u16) })
    } else {
      None
    }
  }

  /// 转换为 u16 整数值
  #[inline]
  pub const fn as_u16(self) -> u16 {
    self as u16
  }

  /// 转换为 usize 索引值
  #[inline]
  pub const fn as_usize(self) -> usize {
    self as usize
  }

  /// 判断当前根是否位于只读区
  #[inline]
  pub const fn is_read_only(self) -> bool {
    (self as usize) <= Self::K_LAST_READ_ONLY_ROOT as usize
  }

  /// 判断当前根是否为不可移动根 (Immortal Immovable)
  #[inline]
  pub const fn is_immortal_immovable(self) -> bool {
    (self as usize) <= Self::K_LAST_IMMORTAL_IMMOVABLE_ROOT as usize
  }

  /// 判断当前根是否属于受信任对象 Map 列表
  #[inline]
  pub const fn is_in_trusted_object_map_list(self) -> bool {
    let val = self as usize;
    val >= Self::K_FIRST_TRUSTED_OBJECT_MAP as usize && val <= Self::K_LAST_TRUSTED_OBJECT_MAP as usize
  }

  /// 判断当前根是否属于可序列化受信任对象 Map 列表
  #[inline]
  pub const fn is_in_serializable_trusted_object_map_list(self) -> bool {
    let val = self as usize;
    val >= Self::K_FIRST_SERIALIZABLE_TRUSTED_OBJECT_MAP as usize
      && val <= Self::K_LAST_SERIALIZABLE_TRUSTED_OBJECT_MAP as usize
  }

  /// 判断当前根是否为堆数字常量 (HeapNumber)
  #[inline]
  pub const fn is_heap_number(self) -> bool {
    let val = self as usize;
    val >= Self::K_FIRST_HEAP_NUMBER_ROOT as usize
      && val <= Self::K_LAST_HEAP_NUMBER_ROOT as usize
  }

  /// 判断当前根是否为可变强根 (Strong Mutable Root)
  #[inline]
  pub const fn is_strong(self) -> bool {
    let val = self as usize;
    val >= Self::K_FIRST_STRONG_ROOT as usize && val <= Self::K_LAST_STRONG_ROOT as usize
  }

  /// 判断当前根是否为 Smi 标记根
  #[inline]
  pub const fn is_smi(self) -> bool {
    let val = self as usize;
    val >= Self::K_FIRST_SMI_ROOT as usize && val <= Self::K_LAST_SMI_ROOT as usize
  }

  /// 根据单字节字符 code (0..=255) 计算其在根表中的 RootIndex
  #[inline]
  pub const fn single_character_string_index(c: u8) -> Self {
    let idx = Self::K_FIRST_SINGLE_CHARACTER_STRING as u16 + c as u16;
    // SAFETY: K_FIRST_SINGLE_CHARACTER_STRING=354, 354 + 255 = 609 = K_LAST_SINGLE_CHARACTER_STRING, 恒在范围内
    unsafe { transmute::<u16, Self>(idx) }
  }

  /// 获取根名称字符串字面量
  #[inline]
  pub const fn name(self) -> &'static str {
    ROOT_NAMES[self as usize]
  }
}

impl TryFrom<usize> for RootIndex {
  type Error = Error;

  #[inline]
  fn try_from(val: usize) -> Result<Self> {
    Self::from_usize(val).ok_or(Error::InvalidRootIndex(val))
  }
}

impl TryFrom<u16> for RootIndex {
  type Error = Error;

  #[inline]
  fn try_from(val: u16) -> Result<Self> {
    Self::from_u16(val).ok_or(Error::InvalidRootIndex(val as usize))
  }
}

impl From<RootIndex> for usize {
  #[inline]
  fn from(idx: RootIndex) -> Self {
    idx as usize
  }
}

impl From<RootIndex> for u16 {
  #[inline]
  fn from(idx: RootIndex) -> Self {
    idx as u16
  }
}

/// 全局根名称静态表 (1:1 严格对齐 C++ RootsTable::root_names_)
pub static ROOT_NAMES: [&str; RootIndex::K_ENTRIES_COUNT] = [
${rawRootNames.map((n) => `  "${n}",`).join("\n")}
];
`;

const target = writeSrc("v8-roots", "root_index.rs", code);
console.log(`已成功生成并写入: ${target}`);
