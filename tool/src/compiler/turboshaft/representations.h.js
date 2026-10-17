#!/usr/bin/env bun
// tool/src/compiler/turboshaft/representations.h.js
// 机械提取 v8/src/compiler/turboshaft/representations.{h,cc}
// → v8-turboshaft/src/representations.rs
//
// 提取面: MaybeRegisterRepresentation / RegisterRepresentation / WordRepresentation /
// FloatRepresentation / MemoryRepresentation 五个枚举 + 各 switch 属性表
// (is_word/is_float/is_tagged_or_compressed/bit_width/machine_representation/
//  max_unsigned_value/is_signed/is_compressible_tagged/to_register_representation/
//  from_machine_representation/size_in_bytes_log2/as_str)。
//
// 目标配置: x64+arm64, V8_COMPRESS_POINTERS=1 (kTaggedSize=4, kTaggedSizeLog2=2,
// kSystemPointerSize=8, kSystemPointerSizeLog2=3)。
import { readV8, writeSrc, banner } from "../../../lib.js";

const hSrc = readV8("src/compiler/turboshaft/representations.h");
const ccSrc = readV8("src/compiler/turboshaft/representations.cc");

// ---------------------------------------------------------------------------
// 目标配置 (hard rule 9: 宏/平台分支显式展开)
// ---------------------------------------------------------------------------
const CONFIG = {
  V8_COMPRESS_POINTERS: true,
  kSystemPointerSize: 8,
  kTaggedSize: 4,
};
const TAGGED_SIZE_LOG2 = 2; // kTaggedSizeLog2, 压缩指针 kTaggedSize==4
const SYSTEM_POINTER_SIZE_LOG2 = 3;
const SYSTEM_POINTER_BITS = 64; // kSystemPointerSize==8 → bit_width(Tagged)=64

// ---------------------------------------------------------------------------
// 通用提取件
// ---------------------------------------------------------------------------

// 提取 `enum class Enum : uint8_t { ... };` 变体列表 (ownerClass 内首个)
function extractEnum(src, classRe, label) {
  const m = src.match(classRe);
  if (!m) throw new Error(`未找到 ${label} 枚举块`);
  const body = m[1].replace(/\/\/[^\n]*/g, ""); // 去行注释
  const variants = [];
  // 变体: kWord32, 或 kWord32 = static_cast<int>(...); 允许末项无逗号
  const itemRe = /k(\w+)\s*(?:=\s*([^,]+?))?\s*(?:,|$)/gm;
  let it;
  while ((it = itemRe.exec(body)) !== null) {
    variants.push({ raw: `k${it[1]}`, name: it[1], expr: it[2]?.trim() ?? null });
  }
  if (variants.length === 0) throw new Error(`${label} 枚举变体为空`);
  return variants;
}

// 解析 switch 体 → fallthrough 组 [{cases:[原始 case 文本...], result: 原始文本}]
function parseSwitch(body) {
  const groups = [];
  let pending = [];
  const tokRe = /case\s+([\w:()]+)\s*:|default\s*:|return\s+([^;]+);|\{/g;
  let t;
  while ((t = tokRe.exec(body)) !== null) {
    if (t[0] === "{") continue; // switch 头花括号
    if (t[1] !== undefined) {
      pending.push(t[1].trim());
    } else if (t[2] !== undefined) {
      if (pending.length === 0) throw new Error(`switch 组无 case: return ${t[2]}`);
      groups.push({ cases: pending, result: t[2].trim() });
      pending = [];
    }
  }
  return groups;
}

// 组表 → [rustCase, rustResult] 行; missing 映射即 throw (对数守卫)
function renderGroups(groups, caseMap, resultMap, label) {
  const out = [];
  for (const g of groups) {
    for (const c of g.cases) {
      const rhs0 = resultMap(g.result, label, c);
      // 结果形如 "2 // 注释" → 输出 "2, // 注释" (注释放逗号后, 防吞逗号)
      const cm2 = rhs0.match(/^([^/]+?)\s*\/\/\s*(.+)$/);
      const rhs = cm2 ? `${cm2[1].trim()}, // ${cm2[2].trim()}` : `${rhs0},`;
      const lhs = caseMap(c, label);
      out.push(`      ${lhs} => ${rhs}`);
    }
  }
  return out.join("\n");
}

const die = (msg, label) => {
  throw new Error(`[${label}] 未登记映射: ${msg}`);
};

// ---------------------------------------------------------------------------
// 1. 枚举变体
// ---------------------------------------------------------------------------
const maybeVariants = extractEnum(
  hSrc,
  /class MaybeRegisterRepresentation\s*\{[\s\S]*?enum class Enum : uint8_t\s*\{([^}]+)\}/,
  "MaybeRegisterRepresentation",
);
const regVariants = extractEnum(
  hSrc,
  /class RegisterRepresentation[\s\S]{0,600}?enum class Enum : uint8_t\s*\{([^}]+)\}/,
  "RegisterRepresentation",
);
const wordVariants = extractEnum(
  hSrc,
  /\nclass WordRepresentation : public RegisterRepresentation\s*\{[\s\S]*?enum class Enum : uint8_t\s*\{([^}]+)\}/,
  "WordRepresentation",
);
const floatVariants = extractEnum(
  hSrc,
  /\nclass FloatRepresentation : public RegisterRepresentation\s*\{[\s\S]*?enum class Enum : uint8_t\s*\{([^}]+)\}/,
  "FloatRepresentation",
);
const memVariants = extractEnum(
  hSrc,
  /class MemoryRepresentation\s*\{[\s\S]*?enum class Enum : uint8_t\s*\{([^}]+)\}/,
  "MemoryRepresentation",
);

// 对数: 原版侧条数 (硬编码自 representations.h 枚举原文, 解析失败即 throw 上抛)
if (maybeVariants.length !== 9) throw new Error(`MaybeRegisterRepresentation ${maybeVariants.length} != 9`);
if (regVariants.length !== 8) throw new Error(`RegisterRepresentation ${regVariants.length} != 8`);
if (wordVariants.length !== 2) throw new Error(`WordRepresentation ${wordVariants.length} != 2`);
if (floatVariants.length !== 2) throw new Error(`FloatRepresentation ${floatVariants.length} != 2`);
if (memVariants.length !== 23) throw new Error(`MemoryRepresentation ${memVariants.length} != 23`);
// RegisterRepresentation 是 MaybeRegisterRepresentation 前 8 项的同名子集
for (let i = 0; i < regVariants.length; i++) {
  if (regVariants[i].name !== maybeVariants[i].name) {
    throw new Error(`RegisterRepresentation[${i}] ${regVariants[i].name} != Maybe ${maybeVariants[i].name}`);
  }
}

// ---------------------------------------------------------------------------
// 2. switch 属性表提取
// ---------------------------------------------------------------------------
function switchBody(classLabel, fnName, src = hSrc) {
  // 在指定 class 文本块内取某成员函数的 switch 体
  const classRe = new RegExp(`class ${classLabel}[\\s\\S]*?\\n\\};`);
  const cm = src.match(classRe);
  if (!cm) throw new Error(`未找到 class ${classLabel}`);
  const fm = cm[0].match(new RegExp(`${fnName}\\([^)]*\\)[^\\{]*\\{([\\s\\S]*?)\\n  \\}`));
  if (!fm) throw new Error(`未找到 ${classLabel}::${fnName}`);
  return fm[1];
}

const rustVariant = (c) => c.replace(/^(?:[A-Za-z:]*::)*k?/, "").replace(/\(\)$/, "");

// MaybeRegisterRepresentation::IsWord / IsFloat / IsTaggedOrCompressed
function boolTable(cls, fn, expectedTrue, label) {
  const groups = parseSwitch(switchBody(cls, fn));
  const trueSet = new Set(expectedTrue);
  const rows = [];
  for (const g of groups) {
    const r = g.result === "true" ? "true" : g.result === "false" ? "false" : die(g.result, label);
    for (const c of g.cases) rows.push(`      ${cls}::${rustVariant(c)} => ${r},`);
  }
  const nTrue = rows.filter((r) => r.endsWith("true,")).length;
  const nFalse = rows.filter((r) => r.endsWith("false,")).length;
  if (nTrue !== expectedTrue.length) throw new Error(`${label} true 数 ${nTrue} != ${expectedTrue.length}`);
  if (nTrue + nFalse !== rows.length) throw new Error(`${label} 存在未分类行`);
  return rows.join("\n");
}

const MR = "MaybeRegisterRepresentation";
const isWordRows = boolTable(MR, "IsWord", ["kWord32", "kWord64"], "IsWord");
const isFloatRows = boolTable(MR, "IsFloat", ["kFloat32", "kFloat64"], "IsFloat");
const isTaggedRows = boolTable(MR, "IsTaggedOrCompressed", ["kTagged", "kCompressed"], "IsTaggedOrCompressed");

// bit_width: uint16_t 表
const bitWidthGroups = parseSwitch(switchBody(MR, "bit_width"));
let bitWidthRows = renderGroups(
  bitWidthGroups,
  (c) => {
    const v = rustVariant(c);
    if (v === "Tagged") return `${MR}::Tagged`;
    if (v === "Compressed") return `${MR}::Compressed`;
    return `${MR}::${v}`;
  },
  (r, _label, c) => {
    const n = r.trim();
    if (/^\d+$/.test(n)) return n;
    if (n === "kSystemPointerSize") {
      // 变体名判定走 rustVariant (c 为组内 case 集原文)
      if (c && /Compressed/.test(String(c))) return String(CONFIG.kTaggedSize * 8);
      return String(CONFIG.kSystemPointerSize * 8); // 字节→位
    }
    return die(r, "bit_width");
  },
  "bit_width",
);
const bitWidthExpect = { Word32: 32, Word64: 64, Float32: 32, Float64: 64, Tagged: 64, Compressed: 32, Simd128: 128, Simd256: 256 };
for (const g of bitWidthGroups) {
  const v = rustVariant(g.cases[0]);
  const n = Number(g.result);
  const expect = bitWidthExpect[v];
  if (expect !== undefined && expect !== n) {
    // Tagged/Compressed 原文为 kSystemPointerSize (字节) → ×8 对齐位宽
    if (!((v === "Tagged" || v === "Compressed") && g.result.includes("kSystemPointerSize"))) {
      throw new Error(`bit_width ${v}=${g.result} 与预期不符`);
    }
  }
}

// machine_representation 表 (→ MachineRepresentation 判别值, 用 v8-compiler-graph 枚举名)
const machRepGroups = parseSwitch(switchBody(MR, "machine_representation"));
let machRepRows = renderGroups(
  machRepGroups,
  (c) => `${MR}::${rustVariant(c)}`,
  (r) => {
    const m = r.match(/MachineRepresentation::k(\w+)/);
    if (!m) return die(r, "machine_representation");
    return `MachineRepresentation::${m[1]}`;
  },
  "machine_representation",
);
if (!machRepRows.includes("::None =>")) {
  machRepRows += `\n      ${MR}::None => MachineRepresentation::None,`;
}
if (!bitWidthRows.includes("::None =>")) {
  bitWidthRows += `\n      ${MR}::None => 0,`;
}

// MaxUnsignedValue
const maxUGroups = parseSwitch(switchBody(MR, "MaxUnsignedValue"));
let maxURows = renderGroups(
  maxUGroups,
  (c) => `${MR}::${rustVariant(c)}`,
  (r) => {
    if (r.includes("uint32_t")) return "u32::MAX as u64";
    if (r.includes("uint64_t")) return "u64::MAX";
    return die(r, "MaxUnsignedValue");
  },
  "MaxUnsignedValue",
);
maxURows += "\n      _ => 0,";

// MemoryRepresentation::IsSigned
const memSignedGroups = parseSwitch(switchBody("MemoryRepresentation", "IsSigned"));
const memSignedRows = renderGroups(
  memSignedGroups,
  (c) => `MemoryRepresentation::${rustVariant(c)}`,
  (r) => {
    if (r === "true") return "true";
    if (r === "false") return "false";
    return die(r, "MemIsSigned"); // UNREACHABLE 组已由 case 枚举覆盖
  },
  "MemIsSigned",
);
if (memSignedRows.split("=> true").length - 1 !== 4) throw new Error("IsSigned true 数 != 4");
const memSignedRowsWithFallback = memSignedRows + "\n      _ => false,";

// MemoryRepresentation::IsCompressibleTagged
const memCmpGroups = parseSwitch(switchBody("MemoryRepresentation", "IsCompressibleTagged"));
const memCmpRows = renderGroups(
  memCmpGroups,
  (c) => `MemoryRepresentation::${rustVariant(c)}`,
  (r) => (r === "true" ? "true" : r === "false" ? "false" : die(r, "IsCompressibleTagged")),
  "IsCompressibleTagged",
);
if (memCmpRows.split("=> true").length - 1 !== 3) throw new Error("IsCompressibleTagged true 数 != 3");

// MemoryRepresentation::ToRegisterRepresentation
const memToRegGroups = parseSwitch(switchBody("MemoryRepresentation", "ToRegisterRepresentation"));
const memToRegRows = renderGroups(
  memToRegGroups,
  (c) => `MemoryRepresentation::${rustVariant(c)}`,
  (r) => {
    const m = r.match(/RegisterRepresentation::(\w+)\(\)/);
    if (!m) return die(r, "MemToRegisterRepresentation");
    return `RegisterRepresentation::${m[1]}`;
  },
  "MemToRegisterRepresentation",
);

// MemoryRepresentation::SizeInBytesLog2
const sizeLog2Groups = parseSwitch(switchBody("MemoryRepresentation", "SizeInBytesLog2"));
const sizeLog2Rows = renderGroups(
  sizeLog2Groups,
  (c) => `MemoryRepresentation::${rustVariant(c)}`,
  (r) => {
    const n = r.trim();
    if (/^\d+$/.test(n)) return n;
    if (n === "kTaggedSizeLog2") return `${TAGGED_SIZE_LOG2} // kTaggedSizeLog2 (压缩指针)`;
    if (n === "kSystemPointerSizeLog2") return `${SYSTEM_POINTER_SIZE_LOG2} // kSystemPointerSizeLog2`;
    return die(r, "SizeInBytesLog2");
  },
  "SizeInBytesLog2",
);

// RegisterRepresentation::FromMachineRepresentation
const regFromMachGroups = parseSwitch(switchBody("RegisterRepresentation", "FromMachineRepresentation"));
let regFromMachRows = renderGroups(
  regFromMachGroups,
  (c) => {
    const m = c.match(/MachineRepresentation::k(\w+)/);
    if (!m) return die(c, "RegFromMachine case");
    if (["None", "Float16RawBits"].includes(m[1])) return `MachineRepresentation::${m[1]}`;
    return `MachineRepresentation::${m[1]}`;
  },
  (r) => {
    // WordPtr() 非变体: 64 位目标上 kSystemPointerSize==8 → Word64 别名
    const ptrFix = (n) => (n === "WordPtr" ? "Word64" : n);
    const m = r.match(/RegisterRepresentation::(\w+)\(\)/);
    if (m) return `RegisterRepresentation::${ptrFix(m[1])}`;
    // 类内简写: Word32() / Tagged() 等
    const bare = r.match(/^(\w+)\(\)$/);
    if (bare) return `RegisterRepresentation::${ptrFix(bare[1])}`;
    if (r.includes("WordPtr")) return "RegisterRepresentation::Word64";
    return die(r, "RegFromMachine result");
  },
  "RegFromMachine",
);

// MemoryRepresentation::FromMachineRepresentation
const memFromMachGroups = parseSwitch(switchBody("MemoryRepresentation", "FromMachineRepresentation"));
const memFromMachRows = renderGroups(
  memFromMachGroups,
  (c) => {
    const m = c.match(/MachineRepresentation::k(\w+)/);
    if (!m) return die(c, "MemFromMachine case");
    return `MachineRepresentation::${m[1]}`;
  },
  (r) => {
    const m = r.match(/MemoryRepresentation::(\w+)\(\)/);
    if (m) return `MemoryRepresentation::${m[1]}`;
    // 类内简写: Uint8() / AnyTagged() 等
    const bare = r.match(/^(\w+)\(\)$/);
    if (bare) return `MemoryRepresentation::${bare[1]}`;
    return die(r, "MemFromMachine result");
  },
  "MemFromMachine",
);

// MemoryRepresentation::ToMachineType → MachineType 构造 (v8-compiler-graph::machine_type::MachineType)
const memToMTGroups = parseSwitch(switchBody("MemoryRepresentation", "ToMachineType"));
const memToMTRows = renderGroups(
  memToMTGroups,
  (c) => `MemoryRepresentation::${rustVariant(c)}`,
  (r) => {
    const m = r.match(/MachineType::(\w+)\(\)/);
    if (m) {
      const name = m[1];
      const snake = name.replace(/([A-Z])/g, "_$1").toLowerCase().replace(/^_/, "");
      return `MachineType::${snake}()`;
    }
    return die(r, "ToMachineType");
  },
  "ToMachineType",
);

// as_str 名表 (representations.cc operator<<)
function nameTable(cc, enumName, ctorName, expect) {
  const groups = parseSwitch(cc);
  const rows = [];
  for (const g of groups) {
    for (const c of g.cases) {
      const v = rustVariant(c);
      const name = g.result.replace(/^os << "/, "").replace(/";?$/, "").replace(/"$/, "");
      rows.push(`      ${enumName}::${v} => "${name}",`);
    }
  }
  if (rows.length !== expect) throw new Error(`${ctorName} 名表 ${rows.length} != ${expect}`);
  return rows.join("\n");
}
const regNameRows = nameTable(ccSrc.match(/operator<<[\s\S]*?MaybeRegisterRepresentation rep\)\s*\{([\s\S]*?)\n\}/)[1], "MaybeRegisterRepresentation", "MaybeReg", 9);
const memNameRows = nameTable(ccSrc.match(/operator<<[\s\S]*?MemoryRepresentation rep\)\s*\{([\s\S]*?)\n\}/)[1], "MemoryRepresentation", "MemoryRepresentation", 23);

// ---------------------------------------------------------------------------
// 3. 生成 Rust
// ---------------------------------------------------------------------------
function enumDef(enumName, variants, doc) {
  let s = `/// ${doc}\n#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]\n#[repr(u8)]\npub enum ${enumName} {\n`;
  variants.forEach((v, i) => {
    s += `  /// ${v.raw}\n  ${v.name} = ${v.expr ? `/* ${v.expr} */ ` : ""}${v.expr ? "" : i},\n`;
  });
  s += "}\n";
  return s;
}

// —— 原版 UNREACHABLE 组在 Rust 侧以 panic! / 默认值承接 ——
if (!/\bNone\b/.test(bitWidthRows)) bitWidthRows += "      MaybeRegisterRepresentation::None => 0,\n";
if (!/\bNone\b/.test(machRepRows)) machRepRows += "      MaybeRegisterRepresentation::None => MachineRepresentation::None,\n";
if (!/_ =>/.test(maxURows)) maxURows += "      _ => 0,\n";
if (!/MachineRepresentation::None/.test(regFromMachRows)) regFromMachRows += "      _ => panic!(\"unsupported MachineRepresentation for RegisterRepresentation\"),\n";
const memFromMachRowsWithFallback = memFromMachRows + "\n      _ => panic!(\"unsupported MachineRepresentation for MemoryRepresentation\"),";

let rust = banner(
  "src/compiler/turboshaft/representations.h.js",
  "src/compiler/turboshaft/representations.{h,cc}",
  "表示体系: 寄存器/内存表示枚举与属性表 (x64+arm64, 压缩指针开)",
);

rust += `
// MachineRepresentation / MachineType 来自 v8-compiler-graph::machine_type
// (原版 src/codegen/machine-type.h)。
use v8_compiler_graph::{MachineRepresentation, MachineType};

`;

// MaybeRegisterRepresentation
rust += enumDef("MaybeRegisterRepresentation", maybeVariants,
  "原版 MaybeRegisterRepresentation — 可空寄存器表示 (含 kNone 哨兵)");
rust += `
impl MaybeRegisterRepresentation {
  /// 原版 kInvalid 哨兵 (static_cast<Enum>(-1)); Rust 侧以 Option 表达
  pub const INVALID: u8 = 255;

  /// 原版 IsWord
  #[inline]
  #[must_use]
  pub const fn is_word(self) -> bool {
    match self {
${isWordRows}
    }
  }

  /// 原版 IsFloat
  #[inline]
  #[must_use]
  pub const fn is_float(self) -> bool {
    match self {
${isFloatRows}
    }
  }

  /// 原版 IsTaggedOrCompressed
  #[inline]
  #[must_use]
  pub const fn is_tagged_or_compressed(self) -> bool {
    match self {
${isTaggedRows}
    }
  }

  /// 原版 bit_width (位宽; kSystemPointerSize=8 展开)
  #[inline]
  #[must_use]
  pub const fn bit_width(self) -> u16 {
    match self {
${bitWidthRows}
    }
  }

  /// 原版 machine_representation
  #[inline]
  #[must_use]
  pub const fn machine_representation(self) -> MachineRepresentation {
    match self {
${machRepRows}
    }
  }

  /// 原版 MaxUnsignedValue
  #[inline]
  #[must_use]
  pub const fn max_unsigned_value(self) -> u64 {
    match self {
${maxURows}
    }
  }

  /// 原版 operator<< 名 (representations.cc)
  #[must_use]
  pub const fn as_str(self) -> &'static str {
    match self {
${regNameRows}
    }
  }
}
`;

// RegisterRepresentation — Maybe 的非空子集
let regEnum = `/// 原版 RegisterRepresentation — 非空寄存器表示 (MaybeRegisterRepresentation 前 8 项子集)\n#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]\n#[repr(u8)]\npub enum RegisterRepresentation {\n`;
regVariants.forEach((v, i) => {
  regEnum += `  /// ${v.raw}\n  ${v.name} = ${i},\n`;
});
regEnum += "}\n";
rust += regEnum;
rust += `
impl RegisterRepresentation {
  /// 上采样为 Maybe 表示
  #[inline]
  #[must_use]
  pub const fn to_maybe(self) -> MaybeRegisterRepresentation {
    match self {
`;
regVariants.forEach((v) => {
  rust += `      RegisterRepresentation::${v.name} => MaybeRegisterRepresentation::${v.name},\n`;
});
rust += `    }
  }

  /// 原版 FromMachineRepresentation (kMapWord → Tagged, 不支持 map packing;
  /// kIndirectPointer/kSandboxedPointer → WordPtr)
  #[inline]
  #[must_use]
  pub const fn from_machine_representation(rep: MachineRepresentation) -> Self {
    match rep {
${regFromMachRows}
    }
  }

  /// 原版 FromMachineType
  #[inline]
  #[must_use]
  pub const fn from_machine_type(ty: MachineType) -> Self {
    Self::from_machine_representation(ty.representation())
  }

  /// 原版 MapTaggedToWord (压缩指针开: Tagged → Word32)
  #[inline]
  #[must_use]
  pub const fn map_tagged_to_word(self) -> Self {
    if self as u8 == RegisterRepresentation::Tagged as u8 {
      RegisterRepresentation::Word32
    } else {
      self
    }
  }
}
`;

// WordRepresentation / FloatRepresentation — 约束子类型 (注释登记, 不重复枚举)
rust += `
/// 原版 WordRepresentation — RegisterRepresentation 的 Word32/Word64 约束子类型;
/// Rust 侧不另立枚举, 以 RegisterRepresentation + 类型谓词表达。
impl RegisterRepresentation {
  /// 原版 WordRepresentation::MinSignedValue
  #[inline]
  #[must_use]
  pub const fn word_min_signed_value(self) -> i64 {
    match self {
      RegisterRepresentation::Word32 => i32::MIN as i64,
      _ => i64::MIN,
    }
  }

  /// 原版 WordRepresentation::MaxSignedValue
  #[inline]
  #[must_use]
  pub const fn word_max_signed_value(self) -> i64 {
    match self {
      RegisterRepresentation::Word32 => i32::MAX as i64,
      _ => i64::MAX,
    }
  }
}
`;

// MemoryRepresentation
rust += "\n";
rust += enumDef("MemoryRepresentation", memVariants, "原版 MemoryRepresentation — 内存加载/存储表示");
rust += `
impl MemoryRepresentation {
  /// 原版 IsSigned (整数表示符号性)
  #[inline]
  #[must_use]
  pub const fn is_signed(self) -> bool {
    match self {
${memSignedRowsWithFallback}
    }
  }

  /// 原版 IsCompressibleTagged (指针压缩相关 load/store 判定)
  #[inline]
  #[must_use]
  pub const fn is_compressible_tagged(self) -> bool {
    match self {
${memCmpRows}
    }
  }

  /// 原版 ToRegisterRepresentation
  #[inline]
  #[must_use]
  pub const fn to_register_representation(self) -> RegisterRepresentation {
    match self {
${memToRegRows}
    }
  }

  /// 原版 SizeInBytesLog2 (kTaggedSizeLog2=${TAGGED_SIZE_LOG2}, kSystemPointerSizeLog2=${SYSTEM_POINTER_SIZE_LOG2})
  #[inline]
  #[must_use]
  pub const fn size_in_bytes_log2(self) -> u8 {
    match self {
${sizeLog2Rows}
    }
  }

  /// 原版 SizeInBytes
  #[inline]
  #[must_use]
  pub const fn size_in_bytes(self) -> u8 {
    1 << self.size_in_bytes_log2()
  }

  /// 原版 FromMachineRepresentation (整型默认无符号)
  #[inline]
  #[must_use]
  pub const fn from_machine_representation(rep: MachineRepresentation) -> Self {
    match rep {
${memFromMachRowsWithFallback}
    }
  }

  /// 原版 ToMachineType
  #[inline]
  #[must_use]
  pub const fn to_machine_type(self) -> MachineType {
    match self {
${memToMTRows}
    }
  }

  /// 原版 operator<< 名 (representations.cc)
  #[must_use]
  pub const fn as_str(self) -> &'static str {
    match self {
${memNameRows}
    }
  }
}
`;

// 提取不变式断言
rust += `
// —— 提取不变式断言 ——
const _: () = {
  assert!(MaybeRegisterRepresentation::None as u8 == 8);
  assert!(RegisterRepresentation::Simd256 as u8 == 7);
  assert!(MemoryRepresentation::Simd256 as u8 == 22);
  assert!(MemoryRepresentation::Int8.size_in_bytes_log2() == 0);
  assert!(MemoryRepresentation::AnyTagged.size_in_bytes_log2() == ${TAGGED_SIZE_LOG2});
  assert!(MemoryRepresentation::Simd128.size_in_bytes_log2() == 4);
  assert!(MaybeRegisterRepresentation::Tagged.bit_width() == ${SYSTEM_POINTER_BITS});
  assert!(MaybeRegisterRepresentation::Compressed.bit_width() == ${CONFIG.kTaggedSize * 8});
  assert!(MemoryRepresentation::Int32.is_signed());
  assert!(!MemoryRepresentation::Uint32.is_signed());
  assert!(MemoryRepresentation::AnyTagged.is_compressible_tagged());
  assert!(!MemoryRepresentation::IndirectPointer.is_compressible_tagged());
};
`;

const p = writeSrc("v8-turboshaft", "representations.rs", rust);
console.log(`written ${p}`);
console.log(`枚举: Maybe=${maybeVariants.length} Reg=${regVariants.length} Word=${wordVariants.length} Float=${floatVariants.length} Mem=${memVariants.length}`);
console.log(`表: bitWidth=${bitWidthGroups.length}组 machRep=${machRepGroups.length}组 memSigned=${memSignedGroups.length}组 memCmp=${memCmpGroups.length}组 memToReg=${memToRegGroups.length}组 sizeLog2=${sizeLog2Groups.length}组 regFromMach=${regFromMachGroups.length}组 memFromMach=${memFromMachGroups.length}组 memToMT=${memToMTGroups.length}组`);
