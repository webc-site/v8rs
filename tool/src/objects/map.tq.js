#!/usr/bin/env bun
// tool/src/objects/map.tq.js — 覆盖 v8/src/objects/map.tq (单文件族)
// → v8-objects-maps/src/map_tq.rs
// 机械提取: MapBits1/2/3 + ExtendedMapBitsEx 位段表 (tq 声明序 = 位序) +
//   Map 布局字段表 (TAGGED_SIZE_8_BYTES 分支展开) + ExtendedMap + macro 清单
// 对数 oracle: clang -E 预处理 v8/src/objects/map.h (Bits1/2/3 C++ 镜像 1:1;
//   目标配置: 压缩指针 K_TAGGED_SIZE=4, WASM 开, INTL 开)

import { execFileSync } from "node:child_process";
import { mkdirSync, writeFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { tmpdir } from "node:os";
import { readV8, writeSrc, banner, root } from "../../lib.js";

const tq = readV8("src/objects/map.tq");

let checks = 0;
const must = (c, m) => {
  checks++;
  if (!c) throw new Error(`锚点失配: ${m}`);
};

// ── clang -E 对数基准 (原版侧 = 编译器预处理产物; 缺失头自动空 stub) ──────
const ppShim = join(tmpdir(), "v8rs-r3o4-pp-shim");
function clangE(rel) {
  const args = [
    "-E", "-P", "-x", "c++", "-std=c++20", "-I", "v8", "-I", "v8/include",
    "-I", ppShim, "-DV8_INTL_SUPPORT=1", "-DV8_ENABLE_WEBASSEMBLY=1", "-",
  ];
  for (let round = 0; round < 48; round++) {
    try {
      return execFileSync("clang", args, {
        input: `#include "${rel}"\n`,
        encoding: "utf8",
        cwd: root,
        maxBuffer: 512 * 1024 * 1024,
      });
    } catch (e) {
      const err = String(e.stderr ?? "");
      const m = err.match(/fatal error: '([^']+)' file not found/);
      if (!m) throw new Error(`clang -E ${rel} 失败: ${err.slice(0, 300)}`);
      const stub = join(ppShim, m[1]);
      mkdirSync(dirname(stub), { recursive: true });
      writeFileSync(stub, "#pragma once\n");
    }
  }
  throw new Error(`clang -E ${rel}: stub 轮数超限`);
}

// ── 产物 fmt 归一 (与 cargo +nightly fmt 同配置; 幂等) ────────────────────
function fmtNormalize(p) {
  const rustfmt = execFileSync("rustup", ["which", "--toolchain", "nightly", "rustfmt"],
    { encoding: "utf8", cwd: root }).trim();
  execFileSync(rustfmt, ["--edition", "2024", "--config-path", join(root, "rustfmt.toml"), p],
    { cwd: root });
}


const mapHpp = clangE("src/objects/map.h");

// C++ 侧 Bits 链名 (pp 产物内 struct BitsN 体, 声明序 = 位序)
function cppBits(structName) {
  const seg = mapHpp.slice(mapHpp.indexOf(`struct ${structName} : BitFieldGroup<${structName},`));
  const body = seg.slice(0, seg.indexOf("\n  };"));
  return [...body.matchAll(/using (\w+) =\s*(?:base::BitField|\w+Bits?::Next)</g)].map((m) => m[1]);
}
const cppBits1 = cppBits("Bits1");
const cppBits2 = cppBits("Bits2");
const cppBits3 = cppBits("Bits3");
must(cppBits1.length === 7, `pp Bits1 链长 ${cppBits1.length} != 7`);
must(cppBits2.length === 3, `pp Bits2 链长 ${cppBits2.length} != 3`);
must(cppBits3.length === 12, `pp Bits3 链长 ${cppBits3.length} != 12`);
must(cppBits3[0] === "EnumLengthBits" && cppBits3[1] === "NumberOfOwnDescriptorsBits",
  `pp Bits3 首两段失配: ${cppBits3.slice(0, 2)}`);

// ── tq bitfield struct 解析 ───────────────────────────────────────────────
function tqBits(structName) {
  const re = new RegExp(`bitfield struct ${structName} extends (\\w+) \\{([\\s\\S]*?)\\n\\}`);
  const m = tq.match(re);
  if (!m) throw new Error(`未找到 tq bitfield struct ${structName}`);
  const fields = [...m[2].matchAll(/^\s{2}(\w+): ([\w<>, ]+): (\d+) bit;$/gm)]
    .map((x) => ({ name: x[1], type: x[2].trim(), len: parseInt(x[3], 10) }));
  if (fields.length === 0) throw new Error(`${structName} 字段解析为空`);
  let pos = 0;
  for (const f of fields) {
    f.start = pos;
    pos += f.len;
  }
  return { width: m[1], usedBits: pos, fields };
}
const bits1 = tqBits("MapBits1");
const bits2 = tqBits("MapBits2");
const bits3 = tqBits("MapBits3");
const bitsEx = tqBits("ExtendedMapBitsEx");

// tq ↔ C++ 位名对数 (is_callable → IsCallableBit; elements_kind → ElementsKindBits)
const camel = (s) => s.replace(/_([a-z0-9])/g, (_, c) => c.toUpperCase()).replace(/^./, (c) => c.toUpperCase());
function crossCheck(tqStruct, cppChain, label) {
  must(tqStruct.fields.length === cppChain.length,
    `${label} 段数失配: tq ${tqStruct.fields.length} vs pp ${cppChain.length}`);
  tqStruct.fields.forEach((f, i) => {
    const expect = camel(f.name) + (f.type === "bool" ? "Bit" : "Bits");
    must(expect === cppChain[i], `${label}[${i}] 位名失配: ${expect} vs pp ${cppChain[i]}`);
  });
}
crossCheck(bits1, cppBits1, "MapBits1");
crossCheck(bits2, cppBits2, "MapBits2");
crossCheck(bits3, cppBits3, "MapBits3");

// 位宽自洽 (uint8 容器 8 位: MapBits1 占 7 位留 1 保留位 / uint32 容器 32 位)
must(bits1.width === "uint8" && bits1.usedBits === 7, `MapBits1 位宽 ${bits1.usedBits} != 7 (1 保留位)`);
must(bits2.width === "uint8" && bits2.usedBits === 8, `MapBits2 位宽 ${bits2.usedBits} != 8`);
must(bits3.width === "uint32" && bits3.usedBits === 32, `MapBits3 位宽 ${bits3.usedBits} != 32`);
must(bitsEx.width === "uint8" && bitsEx.usedBits === 8, `ExtendedMapBitsEx 位宽 ${bitsEx.usedBits} != 8`);
// 原版 static_assert 锚: EnumLength/NumberOfOwnDescriptors 各 10 位 (kDescriptorIndexBitCount)
must(bits3.fields[0].len === 10 && bits3.fields[1].len === 10, "Bits3 前两段应各 10 位");

// ── extern class Map 布局字段 (@if 分支按目标配置展开) ────────────────────
const mapCls = tq.match(/extern class Map extends HeapObject \{([\s\S]*?)\n\}/);
if (!mapCls) throw new Error("未找到 extern class Map");
// 剔除 macro 体, 只留字段声明行 (含 @if 前缀)
// 锚 "instance_size_in_words: uint8;" — 避开 used_or_unused_instance_size_in_words 内部子串
const fieldZone = mapCls[1].slice(mapCls[1].indexOf("instance_size_in_words: uint8;") - 2);
const fieldRe = /^ {2}(@if\((\w+)\) |@ifnot\((\w+)\) )?([\w]+)(?:\[[^\]]+\])?: ([^;]+);$/gm;
const TAGGED_SIZE_8 = false; // 目标配置: 压缩指针 (v8-common K_TAGGED_SIZE=4)
const WASM = true; // -DV8_ENABLE_WEBASSEMBLY=1
const layout = [];
for (const m of fieldZone.matchAll(fieldRe)) {
  const [, attrAll, ifCond, ifnotCond, name, ty] = m;
  if (ifCond === "TAGGED_SIZE_8_BYTES" && !TAGGED_SIZE_8) continue;
  if (ifnotCond === "V8_ENABLE_WEBASSEMBLY" && WASM) continue;
  layout.push({ name, type: ty.replace(/\s+/g, " ").replace(/\s*\|\s*/g, " | ").trim(), cond: attrAll ? attrAll.trim() : "" });
}
must(layout.length === 14, `Map 布局字段 ${layout.length} != 14 (压缩+WASM 配置, padding 剔除)`);
must(layout[0].name === "instance_size_in_words" && layout[0].type === "uint8", "Map 首字段失配");
must(layout[1].name === "inobject_properties_start_or_constructor_function_index", "第 2 字段失配");
must(layout[2].name === "used_or_unused_instance_size_in_words", "第 3 字段失配");
must(layout[3].name === "visitor_id" && layout[4].name === "instance_type", "instance_type 位次失配");
must(layout[5].name === "bit_field" && layout[6].name === "bit_field2" && layout[7].name === "bit_field3",
  "bit_field 三连位次失配");
must(layout[8].name === "prototype", "prototype 位次失配");
must(layout[10].type.includes("WasmStruct"), "instance_descriptors 应含 WasmStruct (WASM 开)");
must(layout[11].type.includes("Map"), "dependent_code 应含 Map 联合 (WASM 开)");
must(layout[13].name === "transitions_or_prototype_info", "尾字段失配");
// C++ 镜像对数: map.h TaggedMember 声明序中 tagged 尾字段
must(mapHpp.includes("TaggedMember<UnionOf<JSPrototype, Map, Null>> prototype_;") ||
  mapHpp.includes("prototype_;"), "pp 缺 prototype_ 成员锚");

// ── ExtendedMapKind / ExtendedMap ─────────────────────────────────────────
const emk = tq.match(/extern enum ExtendedMapKind extends uint8 \{ (\w+), \.\.\.\}/);
must(emk !== null && emk[1] === "kJSInterceptorMap", "ExtendedMapKind 首项失配");
const extMap = tq.match(/extern class ExtendedMap extends Map \{([\s\S]*?)\n\}/);
must(extMap !== null && extMap[1].includes("bit_field_ex: ExtendedMapBitsEx"), "ExtendedMap 字段失配");

// ── macro 清单 (类内 3 + @export 3 + extern 1) ───────────────────────────
const classMacros = [...mapCls[1].matchAll(/^ {2}macro (\w+)\(/gm)].map((m) => m[1]);
must(JSON.stringify(classMacros) === JSON.stringify(["PrototypeInfo", "PrototypeSharedClosureInfo", "IsUndetectable"]),
  `类内 macro 失配: ${JSON.stringify(classMacros)}`);
for (const m of ["LoadMapPrototypeInfo", "LoadMapPrototypeSharedClosureInfo", "IsSimpleObjectMap"])
  must(tq.includes(`macro ${m}(`), `缺 @export macro ${m}`);
must(tq.includes("extern macro IsSpecialReceiverInstanceType(InstanceType): bool;"),
  "缺 extern macro IsSpecialReceiverInstanceType");

// ── 产物 ─────────────────────────────────────────────────────────────────
function bitsBlock(struct, prefix, rustName) {
  let out = `// ── ${rustName} (tq 位序 1:1; 容器 ${struct.width}, 占 ${struct.usedBits} 位) ──\n`;
  for (const f of struct.fields) {
    // BigInt 防符号溢出 (construction_counter @29 3bit)
    const mask = ((1n << BigInt(f.len)) - 1n) << BigInt(f.start);
    const CONST = f.name.toUpperCase();
    out += `/// ${f.name}: ${f.type} (${f.len} bit @${f.start})。\npub const ${prefix}${CONST}_SHIFT: u32 = ${f.start};\npub const ${prefix}${CONST}_MASK: ${struct.width === "uint8" ? "u8" : "u32"} = 0x${mask.toString(16)};\n`;
  }
  return out;
}

let rust = banner("src/objects/map.tq.js", "src/objects/map.tq",
  "Map tq 位段/布局字段表 (clang -E map.h 对数, 压缩+WASM 配置)");
rust += `//! Map 的 Torque 侧布局镜像: 位段表 + 字段表 + macro 清单。
//! 目标配置: 压缩指针 (K_TAGGED_SIZE=4, TAGGED_SIZE_8_BYTES 关 → 无 optional_padding),
//! WASM 开 (instance_descriptors/dependent_code 取联合类型分支), INTL 开。

${bitsBlock(bits1, "MAP_BITS1_", "MapBits1")}
${bitsBlock(bits2, "MAP_BITS2_", "MapBits2")}
${bitsBlock(bits3, "MAP_BITS3_", "MapBits3")}
${bitsBlock(bitsEx, "EXTENDED_MAP_BITS_EX_", "ExtendedMapBitsEx")}

/// ExtendedMapKind 首个已定义变体 (开放枚举; 全量见 v8_common::globals::ExtendedMapKind)。
pub const EXTENDED_MAP_KIND_JS_INTERCEPTOR_MAP: u8 = 0;

/// tq extern class Map 布局字段表 (压缩+WASM 配置展开, 顺序 1:1)。
pub const MAP_LAYOUT_FIELDS: [(&str, &str); ${layout.length}] = [
${layout.map((f) => `  ("${f.name}", "${f.type}"),`).join("\n")}
];

/// TAGGED_SIZE_8_BYTES 分支的 optional_padding 是否存在 (压缩配置 = 否)。
pub const OPTIONAL_PADDING_PRESENT: bool = false;

/// 类内 macro 清单 (macro PrototypeInfo / PrototypeSharedClosureInfo / IsUndetectable)。
pub const MAP_CLASS_MACROS: [&str; ${classMacros.length}] = [
${classMacros.map((m) => `  "${m}",`).join("\n")}
];

/// @export macro 清单 (+ extern macro IsSpecialReceiverInstanceType)。
pub const MAP_EXPORT_MACROS: [&str; 4] = [
  "LoadMapPrototypeInfo",
  "LoadMapPrototypeSharedClosureInfo",
  "IsSimpleObjectMap",
  "IsSpecialReceiverInstanceType",
];

// —— 提取不变式断言 ——
const _: () = {
  assert!(MAP_LAYOUT_FIELDS.len() == ${layout.length});
  assert!((MAP_BITS1_IS_CALLABLE_MASK == 0x01) && (MAP_BITS1_IS_EXTENDED_MAP_MASK == 0x40));
  assert!((MAP_BITS2_ELEMENTS_KIND_SHIFT == 2) && (MAP_BITS2_ELEMENTS_KIND_MASK == 0xfc));
  assert!((MAP_BITS3_ENUM_LENGTH_MASK == 0x3ff) && (MAP_BITS3_CONSTRUCTION_COUNTER_SHIFT == 29));
  assert!((EXTENDED_MAP_BITS_EX_MAP_KIND_MASK == 0x07) && (EXTENDED_MAP_BITS_EX_MAP_SIZE_IN_WORDS_SHIFT == 3));
  assert!(!OPTIONAL_PADDING_PRESENT);
};
`;

const p = writeSrc("v8-objects-maps", "map_tq.rs", rust);
fmtNormalize(p);
console.log(`map_tq.rs: 位段 ${bits1.fields.length}+${bits2.fields.length}+${bits3.fields.length}+${bitsEx.fields.length} (pp 镜像 1:1), 布局字段 ${layout.length}, macro ${classMacros.length}+4, 断言 ${checks} 项通过 → ${p}`);
