#!/usr/bin/env bun
// tool/src/objects/elements-kind.cc.js — 覆盖 v8/src/objects/elements-kind.cc (+elements-kind.h 只读锚)
// → v8-objects-arrays/src/elements_kind.rs
// 机械提取: ElementsKind 全枚举 (真成员+别名) / shift-size 转换表 /
//   ElementsKindToString / kFastElementsKindSequence / 谓词族 / 转换函数
// 对数 oracle: clang -E 两遍 (默认 = 非压缩 / -D V8_COMPRESS_POINTERS = 压缩, crate 主轨取压缩)
//   产物侧配置登记: 压缩 (kTaggedSizeLog2=2, SYSTEM_POINTER_ELEMENTS=PACKED_DOUBLE_ELEMENTS)

import { execFileSync } from "node:child_process";
import { mkdirSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";
import { readV8, writeSrc, banner, root } from "../../lib.js";

// ── clang -E oracle (带缺失 generated 头 stub 自愈循环) ──────────────────
const stubDir = join(tmpdir(), "v8rs-clang-stub");
function clangE(rel, defines = []) {
  for (let i = 0; i < 40; i++) {
    let out;
    try {
      out = execFileSync(
        "clang",
        [
          "-E", "-x", "c++", join(root, "v8", rel), "-std=c++20",
          "-I", join(root, "v8"), "-I", join(root, "v8", "include"), "-I", stubDir,
          ...defines.map((d) => `-D${d}`),
        ],
        { encoding: "utf8", maxBuffer: 512 * 1024 * 1024, stdio: ["ignore", "pipe", "pipe"] },
      );
      return out;
    } catch (e) {
      const err = String(e.stderr || e.message);
      const m = err.match(/fatal error: '([^']+)' file not found/);
      if (!m) throw new Error(`clang -E ${rel} 失败:\n${err.slice(0, 800)}`);
      const miss = m[1];
      mkdirSync(join(stubDir, miss, ".."), { recursive: true });
      writeFileSync(join(stubDir, miss), "");
    }
  }
  throw new Error(`clang -E ${rel} stub 循环超限`);
}

// 取 .i 中属于指定 v8 源文件自身的行 (linemarker 切片)
function sliceSelf(iText, relSuffix) {
  const outLines = [];
  let cur = "";
  for (const ln of iText.split("\n")) {
    const m = ln.match(/^# \d+ "([^"]+)"/);
    if (m) {
      cur = m[1];
      continue;
    }
    if (cur.endsWith(relSuffix)) outLines.push(ln);
  }
  return outLines.join("\n");
}

// ── 提取计数与锚点断言 ───────────────────────────────────────────────────
let checks = 0;
const must = (cond, msg) => {
  checks++;
  if (!cond) throw new Error(`锚点失配: ${msg}`);
};

const iPlain = clangE("src/objects/elements-kind.cc");
const iComp = clangE("src/objects/elements-kind.cc", ["V8_COMPRESS_POINTERS"]);

// 1. 枚举块 (取 .i 第一处完整定义)
function extractEnum(iText) {
  const start = iText.indexOf("enum ElementsKind : uint8_t {");
  must(start >= 0, ".i 缺 enum ElementsKind");
  const end = iText.indexOf("};", start);
  const body = iText.slice(start + "enum ElementsKind : uint8_t {".length, end);
  const items = body
    .split("\n")
    .map((l) => l.trim())
    .filter((l) => l && !l.startsWith("//"))
    .join(" ")
    .split(",")
    .map((s) => s.trim())
    .filter(Boolean);
  return items.map((s) => {
    const m = s.match(/^([A-Z0-9_]+)(?:\s*=\s*([A-Z0-9_]+))?$/);
    if (!m) throw new Error(`枚举项无法解析: "${s}"`);
    return { name: m[1], init: m[2] ?? null };
  });
}
const enumPlain = extractEnum(iPlain);
const enumComp = extractEnum(iComp);
must(enumPlain.length === enumComp.length, `两遍枚举数不一致: ${enumPlain.length} vs ${enumComp.length}`);
must(enumPlain.length === 58, `枚举项应 58 (44 真成员 + 13 别名 + SYSTEM_POINTER), 实得 ${enumPlain.length}`);

// 符号求值: 真成员顺序递增, 带 init 且值 < next 视为别名
function resolve(items) {
  const env = new Map();
  const real = [];
  const alias = [];
  let next = 0;
  for (const it of items) {
    if (it.init === null) {
      env.set(it.name, next);
      real.push({ name: it.name, value: next });
      next++;
    } else {
      const v = env.get(it.init);
      if (v === undefined) throw new Error(`别名 ${it.name} = ${it.init} 无法求值`);
      env.set(it.name, v);
      alias.push({ name: it.name, value: v, init: it.init });
    }
  }
  return { env, real, alias };
}
const rPlain = resolve(enumPlain);
const rComp = resolve(enumComp);
must(rPlain.real.length === 44, `真成员应 44, 实得 ${rPlain.real.length}`);
must(rPlain.alias.length === 14, `别名应 14 (13 + SYSTEM_POINTER), 实得 ${rPlain.alias.length}`);
for (const a of rPlain.alias) {
  const c = rComp.alias.find((x) => x.name === a.name);
  if (a.name === "SYSTEM_POINTER_ELEMENTS") {
    must(a.init === "PACKED_ELEMENTS" && c.init === "PACKED_DOUBLE_ELEMENTS",
      `SYSTEM_POINTER_ELEMENTS 分支失配: 默认=${a.init} 压缩=${c.init}`);
  } else {
    must(c.value === a.value, `别名 ${a.name} 两遍值不一致`);
  }
}
must(rPlain.env.get("LAST_ELEMENTS_KIND") === 41, "LAST_ELEMENTS_KIND != 41");
must(rPlain.env.get("WASM_ARRAY_ELEMENTS") === 42 && rPlain.env.get("NO_ELEMENTS") === 43,
  "WASM_ARRAY/NO_ELEMENTS 值失配");

// 2. kTaggedSizeLog2 两遍取值 (压缩分支直接常量; 非压缩分支符号链 → kSystemPointerSizeLog2)
function taggedLog2(iText) {
  const direct = iText.match(/constexpr int kTaggedSizeLog2 = (\d+);/);
  if (direct) return parseInt(direct[1], 10);
  const sym = iText.match(/constexpr int kTaggedSizeLog2 = kSystemPointerSizeLog2;/);
  must(sym, "kTaggedSizeLog2 定义形态失配");
  const sp = iText.match(/kSystemPointerSizeLog2 = (\d+);/);
  must(sp, "kSystemPointerSizeLog2 数值缺失");
  return parseInt(sp[1], 10);
}
must(taggedLog2(iPlain) === 3, "非压缩 kTaggedSizeLog2 != 3");
must(taggedLog2(iComp) === 2, "压缩 kTaggedSizeLog2 != 2");

// 3. typed ctype 序列 (Shifts/Sizes 数组 init 均来自同一宏序)
const ctypeSeq = [...iPlain.matchAll(/size_to_shift\(sizeof\((\w+)\)\)/g)].map((m) => m[1]);
const sizesIdx = iPlain.indexOf("kTypedArrayAndRabGsabTypedArrayElementsKindSizes");
const sizesBlock = iPlain.slice(sizesIdx, iPlain.indexOf(";", sizesIdx));
const sizeSeq = [...sizesBlock.matchAll(/sizeof\((\w+)\)/g)].map((x) => x[1]);
must(ctypeSeq.length === 24, `ctype 序列应 24, 实得 ${ctypeSeq.length}`);
must(JSON.stringify(ctypeSeq) === JSON.stringify(sizeSeq), "Shifts/Sizes ctype 序列不一致");
const CTYPE_BYTES = { uint8_t: 1, int8_t: 1, uint16_t: 2, int16_t: 2, uint32_t: 4, int32_t: 4, uint64_t: 8, int64_t: 8, float: 4, double: 8 };
const byteSeq = ctypeSeq.map((t) => CTYPE_BYTES[t] ?? (() => { throw new Error(`未知 ctype ${t}`); })());
const shiftOf = (b) => Math.log2(b); // 1/2/4/8 → 0/1/2/3
must(shiftOf(8) === 3 && shiftOf(1) === 0, "shift 换算失配");
const TABLE_SIZE = 1 << Math.ceil(Math.log2(24)); // bit_ceil(24) = 32
must(TABLE_SIZE === 32, `bit_ceil(24) != 32`);

// 4. ElementsKindToShiftSize switch (.h 展开) — case→return 分组对数
const shiftFn = iPlain.slice(
  iPlain.indexOf("inline constexpr int ElementsKindToShiftSize(ElementsKind elements_kind)"),
  iPlain.indexOf("inline constexpr int ElementsKindToByteSize"),
);
must(shiftFn.includes("}"), "ElementsKindToShiftSize 提取失败");
const switchMap = new Map();
for (const m of shiftFn.matchAll(/case ([A-Z0-9_]+):(?:\s*case ([A-Z0-9_]+):)*\s*return ([^;]+);/g)) {
  const ret = m[3].trim();
  const names = [m[1], ...m[0].matchAll(/case ([A-Z0-9_]+):/g)].map((x) => x[1]);
  for (const n of names) switchMap.set(n, ret);
}
must(switchMap.size === 44, `switch 覆盖 kind 应 44 (含 WASM/NO 的 UNREACHABLE 臂), 实得 ${switchMap.size}`);
for (const [i, t] of ctypeSeq.entries()) {
  const kindName = enumPlain[18 + i].name; // typed 段自 UINT8_ELEMENTS (序 18) 起连续 24 项
  const got = switchMap.get(kindName);
  must(got === String(shiftOf(CTYPE_BYTES[t])), `${kindName} shift 对数失配: ${got}`);
}
must([...switchMap.values()].every((v) => v === "kTaggedSizeLog2" || v === "CONSTEXPR_UNREACHABLE()" || /^\d$/.test(v)),
  "switch return 出现非预期值");

// 5. ElementsKindToString switch (.cc 展开) — 宏 PRINT_NAME 展开为相邻字符串字面量
const strFn = iPlain.slice(
  iPlain.indexOf("const char* ElementsKindToString(ElementsKind kind)"),
  iPlain.indexOf("const ElementsKind kFastElementsKindSequence"),
);
const nameMap = [];
for (const m of strFn.matchAll(/case ([A-Z0-9_]+):\s*return ("(?:[^"]*)"(?:\s*"(?:[^"]*)")*);/g)) {
  const s = m[2].match(/"([^"]*)"/g).map((x) => x.slice(1, -1)).join("");
  nameMap.push([m[1], s]);
}
must(nameMap.length === 44, `ToString 映射应 44, 实得 ${nameMap.length}`);
for (const [k, s] of nameMap) {
  const v = rPlain.env.get(k);
  must(v !== undefined, `ToString kind ${k} 不在枚举`);
  if (v >= 18 && v <= 41) {
    // typed 段: PRINT_NAME 宏 = #TYPE "ELEMENTS" → 无下划线 (V8 原版怪癖, mjsunit 同款)
    must(s === `${k.replace("_ELEMENTS", "")}ELEMENTS`, `ToString typed 段失配: ${k} -> ${s}`);
  } else {
    must(k === s, `ToString 自反失配: ${k} -> ${s}`);
  }
}

// 6. kFastElementsKindSequence 锚
const seqArr = iPlain.match(/kFastElementsKindSequence\[kFastElementsKindCount\] = \{\s*PACKED_SMI_ELEMENTS,\s*HOLEY_SMI_ELEMENTS,\s*PACKED_DOUBLE_ELEMENTS,\s*HOLEY_DOUBLE_ELEMENTS,\s*PACKED_ELEMENTS,\s*HOLEY_ELEMENTS/);
must(seqArr, "kFastElementsKindSequence 内容锚失配");

// 7. 源文本存在性锚 (手写模板函数的行为锚)
const ccSrc = readV8("src/objects/elements-kind.cc");
const hSrc = readV8("src/objects/elements-kind.h");
must(ccSrc.includes("return to_kind == PACKED_ELEMENTS || to_kind == HOLEY_ELEMENTS;"), "HOLEY_DOUBLE 转换目标锚失配");
must(ccSrc.includes("*a_out = HOLEY_ELEMENTS;"), "UnionElementsKindUptoSize HOLEY 合流锚失配");
must(hSrc.includes("*a_out = std::max(a, b);"), "UnionElementsKindUptoPackedness max 锚失配");
must(hSrc.includes("return (from_kind == PACKED_SMI_ELEMENTS) ? PACKED_ELEMENTS : HOLEY_ELEMENTS;"), "FastSmiToObject 锚失配");
must(hSrc.includes("(GetHoleyElementsKind(from_kind) == to_kind) ||"), "IsSimpleMapChangeTransition 锚失配");

// ── 命名换算 ─────────────────────────────────────────────────────────────
// PACKED_SMI_ELEMENTS → PackedSmiElements (Rust 枚举 PascalCase)
const camel = (s) =>
  s.toLowerCase().split("_").map((p) => p[0].toUpperCase() + p.slice(1)).join("");
// IsTypedArrayOrRabGsabTypedArrayElementsKind → is_typed_array_or_rab_gsab_typed_array_elements_kind
const snake = (s) =>
  s.replace(/([a-z0-9])([A-Z])/g, "$1_$2").replace(/([A-Z]+)([A-Z][a-z])/g, "$1_$2").toLowerCase();

// 谓词族: 从 .h 展开 (ek.i 的 elements-kind.h 切片) 机械翻译
const hSelf = sliceSelf(iPlain, "src/objects/elements-kind.h");
function extractPredicates() {
  const preds = [];
  // 签名允许 "(\n    ElementsKind kind)" 跨行形态
  const re = /(?:constexpr inline bool|inline constexpr bool|constexpr bool) (Is\w+)\(\s*ElementsKind\s+kind\s*\) \{([\s\S]*?)\n\}/g;
  for (const m of hSelf.matchAll(re)) preds.push({ name: m[1], body: m[2] });
  return preds;
}
const predicates = extractPredicates();
must(predicates.length >= 31, `谓词应 >=31, 实得 ${predicates.length}`);
must(predicates.some((p) => p.name === "IsTypedArrayOrRabGsabTypedArrayElementsKind"),
  "IsTypedArrayOrRabGsabTypedArrayElementsKind 缺失 (跨行签名)");
must(predicates.some((p) => p.name === "IsFastElementsKind"), "IsFastElementsKind 缺失");
must(predicates.some((p) => p.name === "IsHoleyElementsKind"), "IsHoleyElementsKind 缺失");

const V = (name) => `ElementsKind::${camel(name)}`;
function rustExpr(cond, env, aliasNames) {
  let e = cond;
  // static_assert / DCHECK 行剔除
  e = e.replace(/static_assert\([^;]*;\s*/g, "").replace(/DCHECK_IMPLIES\([^;]*;\s*/g, "");
  e = e.trim();
  // IsXxx(kind) → self.is_xxx()
  e = e.replace(/\b(Is\w+)\(kind\)/g, (_, f) => `self.${snake(f)}()`);
  // base::IsInRange(kind, A, B) → 数值范围 (enum range pattern 不稳定, 统一 u8 比较;
  //   下界为 0 时省略 (u8 恒 >= 0, 避免 useless_comparison 警告))
  e = e.replace(/base::IsInRange\(kind,\s*([A-Z0-9_]+),\s*([A-Z0-9_]+)\)/g, (_, a, b) => {
    for (const x of [a, b]) must(env.get(x) !== undefined, `IsInRange 边界 ${x} 无法求值`);
    const lo = env.get(a);
    const hi = env.get(b);
    if (lo === 0) return `(self as u8) <= ${hi}`;
    return `(self as u8) >= ${lo} && (self as u8) <= ${hi}`;
  });
  // kind == X / kind != X (真成员 → matches!; 别名 → u8 数值比较)
  e = e.replace(/kind == ([A-Z0-9_]+)/g, (_, x) =>
    aliasNames.has(x) ? `(self as u8) == ${x}` : `matches!(self, ${V(x)})`);
  e = e.replace(/kind != ([A-Z0-9_]+)/g, (_, x) =>
    aliasNames.has(x) ? `(self as u8) != ${x}` : `!matches!(self, ${V(x)})`);
  // kind <= X (别名常量 → 数值)
  e = e.replace(/kind <= ([A-Z0-9_]+)/g, (_, x) => {
    must(env.get(x) !== undefined, `kind <= ${x} 无法求值`);
    return `(self as u8) <= ${env.get(x)}`;
  });
  // kind % 2 奇偶 (位与实现, 避开 is_multiple_of 建议)
  e = e.replace(/kind % 2 == 1/g, "(self as u8) & 1 == 1");
  e = e.replace(/kind % 2 == 0/g, "(self as u8) & 1 == 0");
  // 收尾: self 方法形态
  e = e.replace(/\bkind\b/g, "self");
  // 残留 C++ 结构检查 (白名单: 翻译后不得再出现 C++ 侧 token)
  if (/Is[A-Z]\w*\(|base::|std::|DCHECK|static_assert|CONSTEXPR|return |const bool|\bkind\b/.test(e)) {
    throw new Error(`谓词翻译残留: ${e}`);
  }
  return e;
}
const predLines = [];
const aliasNameSet = new Set(rPlain.alias.map((a) => a.name));
for (const p of predicates) {
  if (p.name === "IsFastOrNonextensibleOrSealedElementsKind") {
    // 结构特例: const bool result = kind <= HOLEY_SEALED_ELEMENTS; DCHECK...; return result;
    must(p.body.includes("kind <= HOLEY_SEALED_ELEMENTS"), "IsFastOrNonextensibleOrSealed 锚失配");
    predLines.push(`  /// v8: src/objects/elements-kind.h ${p.name}\n  pub const fn ${snake(p.name)}(self) -> bool {\n    (self as u8) <= ${rPlain.env.get("HOLEY_SEALED_ELEMENTS")}\n  }`);
    continue;
  }
  const ret = p.body.match(/return ([\s\S]*?);\s*$/);
  if (!ret) throw new Error(`${p.name} 无 return`);
  const expr = rustExpr(ret[1], rPlain.env, aliasNameSet);
  predLines.push(`  /// v8: src/objects/elements-kind.h ${p.name}\n  pub const fn ${snake(p.name)}(self) -> bool {\n    ${expr}\n  }`);
}

// ── 生成 Rust ────────────────────────────────────────────────────────────
const members = rPlain.real.map((r) => `  ${camel(r.name)},`);
const aliasConsts = rPlain.alias
  .filter((a) => a.name !== "SYSTEM_POINTER_ELEMENTS")
  .map((a) => `/// v8: src/objects/elements-kind.h ${a.name} = ${a.init}\npub const ${a.name}: u8 = ${a.value};`);

const toStringArms = nameMap.map(([k]) => `      ElementsKind::${camel(k)} => "${k}",`);

// to_shift_size match: typed 段按表, tagged 段按 kTaggedSizeLog2, wasm/no 特例
const shiftArms = rPlain.real.map((r) => {
  const got = switchMap.get(r.name);
  let val;
  if (got === "kTaggedSizeLog2") val = "K_TAGGED_SIZE_LOG2";
  else if (got === "CONSTEXPR_UNREACHABLE()") val = "K_TAGGED_SIZE_LOG2"; // 原版不可达; 偏差登记: 按 tagged 对齐返回
  else if (got !== undefined) val = got;
  else {
    must(r.name === "WASM_ARRAY_ELEMENTS" || r.name === "NO_ELEMENTS", `${r.name} 无 shift 映射`);
    val = "K_TAGGED_SIZE_LOG2"; // 原版 CONSTEXPR_UNREACHABLE(); 偏差登记: 按 tagged 对齐返回
  }
  return `      ElementsKind::${camel(r.name)} => ${val},`;
});

const shiftsTable = [...byteSeq.map((b) => String(shiftOf(b))), ...Array(TABLE_SIZE - 24).fill("0")].join(", ");
const sizesTable = [...byteSeq.map(String), ...Array(TABLE_SIZE - 24).fill("0")].join(", ");

let rust = banner("src/objects/elements-kind.cc.js", "src/objects/elements-kind.cc (+elements-kind.h 只读锚)", "ElementsKind 全枚举与转换表; 对数 clang -E 两遍 (默认/压缩)");
rust += `//! ElementsKind: 元素种类枚举与转换规则。
//! 配置: 指针压缩开 (crate 主轨, 与 v8_common::K_TAGGED_SIZE_LOG2=2 对齐;
//! clang -E 两遍对数: 非压缩分支 kTaggedSizeLog2=3 / SYSTEM_POINTER_ELEMENTS=PACKED_ELEMENTS)。

use v8_common::K_TAGGED_SIZE_LOG2;

/// 真成员数 (含 WASM_ARRAY/NO_ELEMENTS; 不含 14 别名)。
pub const K_REAL_KIND_COUNT: u8 = ${rPlain.real.length};
/// kElementsKindCount = LAST_ELEMENTS_KIND - FIRST_ELEMENTS_KIND + 1。
pub const K_ELEMENTS_KIND_COUNT: usize = ${rPlain.env.get("LAST_ELEMENTS_KIND") - rPlain.env.get("FIRST_ELEMENTS_KIND") + 1};
/// kFastElementsKindCount。
pub const K_FAST_ELEMENTS_KIND_COUNT: usize = 6;
/// kFastElementsKindPackedToHoley = HOLEY_SMI - PACKED_SMI。
pub const K_FAST_ELEMENTS_KIND_PACKED_TO_HOLEY: u8 = 1;
/// kElementsKindBits (map bit_field2 位段宽)。
pub const K_ELEMENTS_KIND_BITS: u32 = 6;
/// kFastElementsKindBits。
pub const K_FAST_ELEMENTS_KIND_BITS: u32 = 3;
/// TypedArrayAndRabGsabTypedArrayElementsKindTableSize = bit_ceil(12+12)。
pub const TYPED_ARRAY_AND_RAB_GSAB_TABLE_SIZE: usize = ${TABLE_SIZE};

const _: () = assert!((1 << K_ELEMENTS_KIND_BITS) > ${rPlain.env.get("LAST_ELEMENTS_KIND")});
const _: () = assert!((1 << (K_ELEMENTS_KIND_BITS - 1)) <= ${rPlain.env.get("LAST_ELEMENTS_KIND")});

/// v8: src/objects/elements-kind.h enum ElementsKind : uint8_t
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(u8)]
pub enum ElementsKind {
${members.join("\n")}
}

${aliasConsts.join("\n\n")}

/// SYSTEM_POINTER_ELEMENTS (压缩配置别名 → PACKED_DOUBLE_ELEMENTS;
/// 非压缩分支为 PACKED_ELEMENTS, clang -E 双分支对数)。
pub const SYSTEM_POINTER_ELEMENTS: ElementsKind = ElementsKind::PackedDoubleElements;

impl ElementsKind {
  /// 首个 kind (PACKED_SMI_ELEMENTS)。
  pub const FIRST: Self = Self::PackedSmiElements;
  /// 最后一个 kind (RAB_GSAB_FLOAT16_ELEMENTS)。
  pub const LAST: Self = Self::RabGsabFloat16Elements;

  /// 真成员全集 (按枚举值升序)。
  pub const ALL: [Self; ${rPlain.real.length}] = [
${rPlain.real.map((r) => `    Self::${camel(r.name)},`).join("\n")}
  ];

  /// v8: src/objects/elements-kind.h GetInitialFastElementsKind
  pub const INITIAL_FAST: Self = Self::PackedSmiElements;

  /// 数值 → 枚举 (0..=43 之外返回 None)。
  #[must_use]
  pub const fn from_u8(v: u8) -> Option<Self> {
    if v <= ${rPlain.real.length - 1} {
      Some(Self::ALL[v as usize])
    } else {
      None
    }
  }

${predLines.join("\n\n")}

  /// v8: src/objects/elements-kind.h ElementsKindToShiftSize
  /// (WASM_ARRAY/NO_ELEMENTS 原版 CONSTEXPR_UNREACHABLE; 偏差: 按 tagged 对齐返回)。
  #[must_use]
  pub const fn to_shift_size(self) -> u32 {
    match self {
${shiftArms.join("\n")}
    }
  }

  /// v8: src/objects/elements-kind.h ElementsKindToByteSize
  #[must_use]
  pub const fn to_byte_size(self) -> u32 {
    1 << self.to_shift_size()
  }

  /// v8: src/objects/elements-kind.cc ElementsKindToString
  #[must_use]
  pub const fn to_str(self) -> &'static str {
    match self {
${toStringArms.join("\n")}
    }
  }

  /// v8: src/objects/elements-kind.cc GetDefaultHeaderSizeForElementsKind
  /// (OFFSET_OF_DATA_START(FixedArray) 取 crate 压缩主轨 K_HEADER_SIZE=8, 减 kHeapObjectTag=1)。
  #[must_use]
  pub const fn default_header_size(self) -> usize {
    if self.is_typed_array_or_rab_gsab_typed_array_elements_kind() {
      0
    } else {
      crate::fixed_array_base::K_HEADER_SIZE - v8_objects_core::tagged_constants::HEAP_OBJECT_TAG
    }
  }

  /// v8: src/objects/elements-kind.cc GetSequenceIndexFromFastElementsKind
  #[must_use]
  pub const fn sequence_index_from_fast(self) -> usize {
    match self {
      Self::PackedSmiElements => 0,
      Self::HoleySmiElements => 1,
      Self::PackedDoubleElements => 2,
      Self::HoleyDoubleElements => 3,
      Self::PackedElements => 4,
      Self::HoleyElements => 5,
      _ => ${rPlain.env.get("LAST_FAST_ELEMENTS_KIND") + 1}, // 原版 UNREACHABLE; 非 fast 返回越界哨兵
    }
  }

  /// v8: src/objects/elements-kind.cc GetFastElementsKindFromSequenceIndex
  #[must_use]
  pub const fn fast_from_sequence_index(i: usize) -> Self {
    FAST_ELEMENTS_KIND_SEQUENCE[i % K_FAST_ELEMENTS_KIND_COUNT]
  }

  /// v8: src/objects/elements-kind.cc GetNextTransitionElementsKind
  #[must_use]
  pub const fn next_transition(self) -> Self {
    Self::fast_from_sequence_index(self.sequence_index_from_fast() + 1)
  }

  /// v8: src/objects/elements-kind.h GetPackedElementsKind
  #[must_use]
  pub const fn packed(self) -> Self {
    match self {
      Self::HoleySmiElements => Self::PackedSmiElements,
      Self::HoleyDoubleElements => Self::PackedDoubleElements,
      Self::HoleyElements => Self::PackedElements,
      k => k,
    }
  }

  /// v8: src/objects/elements-kind.h GetHoleyElementsKind
  #[must_use]
  pub const fn holey(self) -> Self {
    match self {
      Self::PackedSmiElements => Self::HoleySmiElements,
      Self::PackedDoubleElements => Self::HoleyDoubleElements,
      Self::PackedElements => Self::HoleyElements,
      Self::PackedNonextensibleElements => Self::HoleyNonextensibleElements,
      k => k,
    }
  }

  /// v8: src/objects/elements-kind.h FastSmiToObjectElementsKind
  #[must_use]
  pub const fn fast_smi_to_object(self) -> Self {
    match self {
      Self::PackedSmiElements => Self::PackedElements,
      _ => Self::HoleyElements,
    }
  }

  /// v8: src/objects/elements-kind.h GetCorrespondingRabGsabElementsKind
  #[must_use]
  pub const fn corresponding_rab_gsab(self) -> Self {
    let v = (self as u8)
      .wrapping_sub(FIRST_FIXED_TYPED_ARRAY_ELEMENTS_KIND)
      .wrapping_add(FIRST_RAB_GSAB_FIXED_TYPED_ARRAY_ELEMENTS_KIND);
    match Self::from_u8(v) {
      Some(k) => k,
      None => Self::RabGsabFloat16Elements,
    }
  }

  /// v8: src/objects/elements-kind.h GetCorrespondingNonRabGsabElementsKind
  #[must_use]
  pub const fn corresponding_non_rab_gsab(self) -> Self {
    let v = (self as u8)
      .wrapping_sub(FIRST_RAB_GSAB_FIXED_TYPED_ARRAY_ELEMENTS_KIND)
      .wrapping_add(FIRST_FIXED_TYPED_ARRAY_ELEMENTS_KIND);
    match Self::from_u8(v) {
      Some(k) => k,
      None => Self::Float16Elements,
    }
  }
}

/// v8: src/objects/elements-kind.cc kFastElementsKindSequence
pub const FAST_ELEMENTS_KIND_SEQUENCE: [ElementsKind; 6] = [
  ElementsKind::PackedSmiElements,
  ElementsKind::HoleySmiElements,
  ElementsKind::PackedDoubleElements,
  ElementsKind::HoleyDoubleElements,
  ElementsKind::PackedElements,
  ElementsKind::HoleyElements,
];

/// v8: src/objects/elements-kind.cc kTypedArrayAndRabGsabTypedArrayElementsKindShifts
/// (索引 = kind - FIRST_FIXED_TYPED_ARRAY_ELEMENTS_KIND; 尾部 bit_ceil 填充 0)。
pub const TYPED_ARRAY_AND_RAB_GSAB_SHIFTS: [u8; ${TABLE_SIZE}] = [${shiftsTable}];

/// v8: src/objects/elements-kind.cc kTypedArrayAndRabGsabTypedArrayElementsKindSizes
pub const TYPED_ARRAY_AND_RAB_GSAB_SIZES: [u8; ${TABLE_SIZE}] = [${sizesTable}];

/// v8: src/objects/elements-kind.h UnionElementsKindUptoPackedness
#[must_use]
pub fn union_elements_kind_upto_packedness(a: &mut ElementsKind, b: ElementsKind) -> bool {
  match *a {
    ElementsKind::HoleySmiElements | ElementsKind::PackedSmiElements => {
      if matches!(b, ElementsKind::PackedSmiElements | ElementsKind::HoleySmiElements) {
        if (*a as u8) < (b as u8) {
          *a = b;
        }
        true
      } else {
        false
      }
    }
    ElementsKind::PackedElements | ElementsKind::HoleyElements => {
      if matches!(b, ElementsKind::PackedElements | ElementsKind::HoleyElements) {
        if (*a as u8) < (b as u8) {
          *a = b;
        }
        true
      } else {
        false
      }
    }
    ElementsKind::PackedDoubleElements | ElementsKind::HoleyDoubleElements => {
      if matches!(b, ElementsKind::PackedDoubleElements | ElementsKind::HoleyDoubleElements) {
        if (*a as u8) < (b as u8) {
          *a = b;
        }
        true
      } else {
        false
      }
    }
    _ => false,
  }
}

/// v8: src/objects/elements-kind.cc UnionElementsKindUptoSize
#[must_use]
pub fn union_elements_kind_upto_size(a: &mut ElementsKind, b: ElementsKind) -> bool {
  match *a {
    ElementsKind::PackedSmiElements => match b {
      ElementsKind::PackedSmiElements
      | ElementsKind::HoleySmiElements
      | ElementsKind::PackedElements
      | ElementsKind::HoleyElements => {
        *a = b;
        true
      }
      _ => false,
    },
    ElementsKind::HoleySmiElements => match b {
      ElementsKind::PackedSmiElements | ElementsKind::HoleySmiElements => {
        *a = ElementsKind::HoleySmiElements;
        true
      }
      ElementsKind::PackedElements | ElementsKind::HoleyElements => {
        *a = ElementsKind::HoleyElements;
        true
      }
      _ => false,
    },
    ElementsKind::PackedElements => match b {
      ElementsKind::PackedSmiElements | ElementsKind::PackedElements => {
        *a = ElementsKind::PackedElements;
        true
      }
      ElementsKind::HoleySmiElements | ElementsKind::HoleyElements => {
        *a = ElementsKind::HoleyElements;
        true
      }
      _ => false,
    },
    ElementsKind::HoleyElements => match b {
      ElementsKind::PackedSmiElements
      | ElementsKind::HoleySmiElements
      | ElementsKind::PackedElements
      | ElementsKind::HoleyElements => {
        *a = ElementsKind::HoleyElements;
        true
      }
      _ => false,
    },
    ElementsKind::PackedDoubleElements => match b {
      ElementsKind::PackedDoubleElements | ElementsKind::HoleyDoubleElements => {
        *a = b;
        true
      }
      _ => false,
    },
    ElementsKind::HoleyDoubleElements => match b {
      ElementsKind::PackedDoubleElements | ElementsKind::HoleyDoubleElements => {
        *a = ElementsKind::HoleyDoubleElements;
        true
      }
      _ => false,
    },
    _ => false,
  }
}

/// v8: src/objects/elements-kind.cc IsMoreGeneralElementsKindTransition
#[must_use]
pub const fn is_more_general_elements_kind_transition(from: ElementsKind, to: ElementsKind) -> bool {
  if !from.is_fast_elements_kind() {
    return false;
  }
  if !(to.is_fast_elements_kind() || matches!(to, ElementsKind::DictionaryElements)) {
    return false;
  }
  match from {
    ElementsKind::PackedSmiElements => !matches!(to, ElementsKind::PackedSmiElements),
    ElementsKind::HoleySmiElements => !matches!(
      to,
      ElementsKind::PackedSmiElements | ElementsKind::HoleySmiElements
    ),
    ElementsKind::PackedDoubleElements => !matches!(
      to,
      ElementsKind::PackedSmiElements
        | ElementsKind::HoleySmiElements
        | ElementsKind::PackedDoubleElements
    ),
    ElementsKind::HoleyDoubleElements => matches!(
      to,
      ElementsKind::PackedElements | ElementsKind::HoleyElements
    ),
    ElementsKind::PackedElements => matches!(to, ElementsKind::HoleyElements),
    _ => false,
  }
}

/// v8: src/objects/elements-kind.h GetMoreGeneralElementsKind
#[must_use]
pub const fn get_more_general_elements_kind(from: ElementsKind, to: ElementsKind) -> ElementsKind {
  if is_more_general_elements_kind_transition(from, to) {
    to
  } else {
    from
  }
}

/// v8: src/objects/elements-kind.h IsSimpleMapChangeTransition
#[must_use]
pub const fn is_simple_map_change_transition(from: ElementsKind, to: ElementsKind) -> bool {
  ((from.holey() as u8) == (to as u8))
    || (from.is_smi_elements_kind() && to.is_object_elements_kind())
}

// —— 提取不变式断言 (clang -E 对数; const 兼容: 枚举比较走 u8 / matches!) ——
const _: () = {
  assert!(ElementsKind::ALL.len() == ${rPlain.real.length});
  assert!(ElementsKind::PackedSmiElements as u8 == 0);
  assert!(ElementsKind::RabGsabFloat16Elements as u8 == 41);
  assert!(ElementsKind::WasmArrayElements as u8 == 42);
  assert!(ElementsKind::NoElements as u8 == 43);
  assert!(LAST_ELEMENTS_KIND == 41);
  assert!(FIRST_FIXED_TYPED_ARRAY_ELEMENTS_KIND == 18);
  assert!(LAST_FIXED_TYPED_ARRAY_ELEMENTS_KIND == 29);
  assert!(FIRST_RAB_GSAB_FIXED_TYPED_ARRAY_ELEMENTS_KIND == 30);
  assert!(LAST_RAB_GSAB_FIXED_TYPED_ARRAY_ELEMENTS_KIND == 41);
  assert!(matches!(SYSTEM_POINTER_ELEMENTS, ElementsKind::PackedDoubleElements));
  assert!(K_ELEMENTS_KIND_COUNT == 42);
  assert!(TYPED_ARRAY_AND_RAB_GSAB_TABLE_SIZE == 32);
  assert!(TYPED_ARRAY_AND_RAB_GSAB_SHIFTS.len() == 32);
  assert!(TYPED_ARRAY_AND_RAB_GSAB_SHIFTS[0] == 0); // uint8
  assert!(TYPED_ARRAY_AND_RAB_GSAB_SHIFTS[10] == 3); // float64 (序: ...Float32=9, Float64=10, Float16=11)
  assert!(TYPED_ARRAY_AND_RAB_GSAB_SIZES[10] == 8);
  assert!(TYPED_ARRAY_AND_RAB_GSAB_SHIFTS[22] == 3); // rab_gsab float64
  assert!(TYPED_ARRAY_AND_RAB_GSAB_SHIFTS[11] == 1); // float16
  assert!(matches!(FAST_ELEMENTS_KIND_SEQUENCE[0], ElementsKind::PackedSmiElements));
  assert!(ElementsKind::Uint8Elements.to_byte_size() == 1);
  assert!(ElementsKind::Float64Elements.to_byte_size() == 8);
  assert!(ElementsKind::PackedSmiElements.to_shift_size() == K_TAGGED_SIZE_LOG2);
  assert!(ElementsKind::PackedElements.to_str().len() == 15);
  assert!(ElementsKind::NoElements.to_str().len() == 11);
  assert!(ElementsKind::Uint8Elements.to_str().as_bytes()[0] == b'U');
  assert!(matches!(ElementsKind::HoleyElements.packed(), ElementsKind::PackedElements));
  assert!(matches!(ElementsKind::PackedNonextensibleElements.holey(), ElementsKind::HoleyNonextensibleElements));
  assert!(ElementsKind::HoleySmiElements.sequence_index_from_fast() == 1);
  assert!(matches!(ElementsKind::PackedSmiElements.next_transition(), ElementsKind::HoleySmiElements));
  assert!(matches!(ElementsKind::HoleyElements.next_transition(), ElementsKind::PackedSmiElements));
  assert!(matches!(ElementsKind::Uint8Elements.corresponding_rab_gsab(), ElementsKind::RabGsabUint8Elements));
  assert!(matches!(ElementsKind::RabGsabFloat16Elements.corresponding_non_rab_gsab(), ElementsKind::Float16Elements));
  assert!(is_simple_map_change_transition(ElementsKind::PackedSmiElements, ElementsKind::HoleySmiElements));
  assert!(!is_more_general_elements_kind_transition(ElementsKind::HoleyElements, ElementsKind::PackedElements));
  assert!(is_more_general_elements_kind_transition(ElementsKind::PackedSmiElements, ElementsKind::DictionaryElements));
};
`;

const p = writeSrc("v8-objects-arrays", "elements_kind.rs", rust);
console.log(
  `elements_kind.rs: 枚举成员 ${rPlain.real.length}+别名 ${rPlain.alias.length}、谓词 ${predicates.length}、` +
    `ToString ${nameMap.length}、shift 表 ${TABLE_SIZE} (实项 24)、sequence 6; 断言 ${checks} 项通过; 两遍 clang -E 对数一致`,
);
