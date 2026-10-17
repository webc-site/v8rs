#!/usr/bin/env bun
// tool/src/objects/prototype-info.tq.js — 覆盖 v8/src/objects/prototype-info.tq (单文件族)
// → v8-objects-maps/src/prototype_info_tq.rs
// 机械提取: PrototypeInfoFlags 位段 + PrototypeInfo tq 布局字段表 (含尾随数组)
// 对数 oracle: clang -E 预处理 v8/src/objects/prototype-info.h
//   (C++ TaggedMember 声明序 1:1; 压缩指针 K_TAGGED_SIZE=4)

import { execFileSync } from "node:child_process";
import { mkdirSync, writeFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { tmpdir } from "node:os";
import { readV8, writeSrc, banner, root } from "../../lib.js";

const tq = readV8("src/objects/prototype-info.tq");

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


const hpp = clangE("src/objects/prototype-info.h");

// ── 1. PrototypeInfoFlags (should_be_fast: 1 bit, uint31 容器) ────────────
const flagsTq = tq.match(/bitfield struct PrototypeInfoFlags extends uint31 \{([\s\S]*?)\n\}/);
if (!flagsTq) throw new Error("未找到 tq PrototypeInfoFlags");
const flagBits = [...flagsTq[1].matchAll(/^ {2}(\w+): ([\w<>']+): (\d+) bit;$/gm)]
  .map((m) => ({ name: m[1], type: m[2], len: parseInt(m[3], 10) }));
must(flagBits.length === 1 && flagBits[0].name === "should_be_fast" && flagBits[0].len === 1,
  `PrototypeInfoFlags 失配: ${JSON.stringify(flagBits)}`);
// pp 侧位链锚 (ShouldBeFastBit = bit 0, 1 位)
must(hpp.includes("using ShouldBeFastBit = base::BitField<bool, 0, 1, uint32_t>;"),
  "pp 缺 ShouldBeFastBit 声明锚");

// ── 2. kPrototypeInfoCachedHandlerCount (tq const ↔ pp 枚举展开) ─────────
must(tq.includes("const kPrototypeInfoCachedHandlerCount: constexpr int31"), "缺 tq 常量声明");
// pp: enum CachedHandlerIndex 展开 → kCachedHandlerCount = 2
const enumSeg = hpp.slice(hpp.indexOf("enum CachedHandlerIndex"));
const enumBody = enumSeg.slice(0, enumSeg.indexOf("};"));
const enumNames = [...enumBody.matchAll(/^\s*(k\w+)(?: = \d+)?,?$/gm)].map((m) => m[1]);
must(JSON.stringify(enumNames) === JSON.stringify([
  "kLoadNonExistentHandlerDefault", "kLoadNonExistentHandlerWithLookupOnReceiver", "kCachedHandlerCount",
]), `pp CachedHandlerIndex 失配: ${JSON.stringify(enumNames)}`);
const CACHED_HANDLER_COUNT = 2;

// ── 3. tq 布局字段表 (7 标量 + 1 尾数组 ↔ pp TaggedMember 序 1:1) ─────────
const cls = tq.match(/extern class PrototypeInfo extends Struct \{([\s\S]*?)\n\}/);
if (!cls) throw new Error("未找到 tq PrototypeInfo");
const fields = [...cls[1].matchAll(/^ {2}(\w+)(?:\[([^\]]+)\])?: ([^;]+);$/gm)]
  .map((m) => ({ name: m[1], extent: m[2] ?? "", type: m[3].replace(/\s+/g, " ").trim() }));
must(fields.length === 8, `tq 字段数 ${fields.length} != 8`);
must(fields[0].name === "module_namespace" && fields[1].name === "prototype_users", "前两字段失配");
must(fields[3].name === "registry_slot" && fields[3].type === "Smi", "registry_slot 失配");
must(fields[4].name === "bit_field" && fields[4].type === "SmiTagged<PrototypeInfoFlags>", "bit_field 失配");
must(fields[7].name === "cached_handler" && fields[7].extent === "kPrototypeInfoCachedHandlerCount",
  "尾数组失配");

// pp TaggedMember 序 (多行声明折叠后再匹配; V8_TQ_EXTENT_NAME 数组保留在成员上)
const piSeg = hpp.slice(hpp.indexOf("class PrototypeInfo : public Struct"));
const piBody = piSeg.slice(0, piSeg.indexOf("V8_OBJECT_END")).replace(/\n\s+/g, " ");
const members = [...piBody.matchAll(/TaggedMember<([\w<>, ]+?)> (\w+)/g)]
  .map((m) => [m[2].replace(/_$/, ""), m[1].replace(/\s+/g, " ")]);
// 数组成员名 cached_handler_ (.extent 截断) — 取 8 个
must(members.length >= 8, `pp 成员数 ${members.length} < 8`);
for (let i = 0; i < 8; i++) {
  must(fields[i].name === members[i][0], `tq/pp 名失配 @${i}: ${fields[i].name} vs ${members[i][0]}`);
}

// ── 4. 压缩配置偏移 (Struct 头 = HeapObject map word) ─────────────────────
const T = 4;
const offsets = fields.map((_, i) => T + i * T); // 4..32
const K_CACHED_HANDLER_OFFSET = offsets[7];
must(K_CACHED_HANDLER_OFFSET === 32, `cached_handler 偏移 ${K_CACHED_HANDLER_OFFSET} != 32`);
// 尺寸 = 头 + 8 tagged + 2 cached_handler
const K_OBJECT_SIZE = T + (fields.length + CACHED_HANDLER_COUNT) * T;
must(K_OBJECT_SIZE === 44, `对象尺寸 ${K_OBJECT_SIZE} != 44`);

let rust = banner("src/objects/prototype-info.tq.js", "src/objects/prototype-info.tq",
  "PrototypeInfo tq 布局 (clang -E prototype-info.h 对数, 压缩配置)");
rust += `//! PrototypeInfo 的 Torque 侧布局镜像: 标志位段 + 字段表 + 偏移
//! (压缩 K_TAGGED_SIZE=4; 语义面/PrototypeUsers 归 prototype-info.h 侧脚本)。

/// PrototypeInfoFlags: should_be_fast 位段 (1 bit @0, SmiTagged 容器)。
pub const SHOULD_BE_FAST_SHIFT: u32 = 0;
/// should_be_fast 掩码。
pub const SHOULD_BE_FAST_MASK: u32 = 0x1;

/// 缓存 handler 数 (原版 kCachedHandlerCount; Default + WithLookupOnReceiver)。
pub const CACHED_HANDLER_COUNT: usize = ${CACHED_HANDLER_COUNT};
/// kLoadNonExistentHandlerDefault 序值。
pub const LOAD_NON_EXISTENT_HANDLER_DEFAULT: usize = 0;
/// kLoadNonExistentHandlerWithLookupOnReceiver 序值。
pub const LOAD_NON_EXISTENT_HANDLER_WITH_LOOKUP_ON_RECEIVER: usize = 1;

/// tq extern class PrototypeInfo 字段表 (顺序 1:1; 尾数组含 extent 名)。
pub const PROTOTYPE_INFO_FIELDS: [(&str, &str); ${fields.length}] = [
${fields.map((f) => `  ("${f.name}${f.extent ? `[${f.extent}]` : ""}", "${f.type.replace(/\s*\|\s*/g, " | ")}"),`).join("\n")}
];

/// module_namespace 偏移 (tagged)。
pub const K_MODULE_NAMESPACE_OFFSET: usize = ${offsets[0]};
/// prototype_users 偏移 (tagged)。
pub const K_PROTOTYPE_USERS_OFFSET: usize = ${offsets[1]};
/// prototype_chain_enum_cache 偏移 (tagged)。
pub const K_PROTOTYPE_CHAIN_ENUM_CACHE_OFFSET: usize = ${offsets[2]};
/// registry_slot 偏移 (tagged Smi)。
pub const K_REGISTRY_SLOT_OFFSET: usize = ${offsets[3]};
/// bit_field 偏移 (tagged Smi, 内嵌 PrototypeInfoFlags)。
pub const K_BIT_FIELD_OFFSET: usize = ${offsets[4]};
/// derived_maps 偏移 (tagged)。
pub const K_DERIVED_MAPS_OFFSET: usize = ${offsets[5]};
/// prototype_shared_closure_info 偏移 (tagged)。
pub const K_PROTOTYPE_SHARED_CLOSURE_INFO_OFFSET: usize = ${offsets[6]};
/// cached_handler 数组起点偏移 (tagged x kCachedHandlerCount)。
pub const K_CACHED_HANDLER_OFFSET: usize = ${K_CACHED_HANDLER_OFFSET};
/// 定长对象尺寸 (头 + 8 字段 + 2 缓存 handler)。
pub const K_OBJECT_SIZE: usize = ${K_OBJECT_SIZE};

// —— 提取不变式断言 ——
const _: () = {
  assert!(PROTOTYPE_INFO_FIELDS.len() == 8);
  assert!((SHOULD_BE_FAST_MASK == 0x1) && (SHOULD_BE_FAST_SHIFT == 0));
  assert!((CACHED_HANDLER_COUNT == 2) && (LOAD_NON_EXISTENT_HANDLER_WITH_LOOKUP_ON_RECEIVER == 1));
  assert!((K_CACHED_HANDLER_OFFSET == 32) && (K_OBJECT_SIZE == 44));
};
`;

const p = writeSrc("v8-objects-maps", "prototype_info_tq.rs", rust);
fmtNormalize(p);
console.log(`prototype_info_tq.rs: 位段 1 + 字段 ${fields.length} (pp 序 1:1), cached_handler@${K_CACHED_HANDLER_OFFSET}, 断言 ${checks} 项通过 → ${p}`);
