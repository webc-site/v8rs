#!/usr/bin/env bun
// tool/src/objects/prototype-info.h.js — 覆盖 v8/src/objects/prototype-info.{h,-inl.h} (同名族合一)
// → v8-objects-maps/src/prototype_info.rs
// 机械提取: CachedHandlerIndex 枚举 + UNREGISTERED + ShouldBeFastBit 位段 +
//   PrototypeUsers 槽位常量 + 字段表 + -inl 访问器签名清单
// 对数 oracle: clang -E 预处理 v8/src/objects/prototype-info.h 与 -inl.h
//   (枚举体/常量/TaggedMember 序/访问器定义计数全部取自编译器产物)

import { execFileSync } from "node:child_process";
import { mkdirSync, writeFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { tmpdir } from "node:os";
import { readV8, writeSrc, banner, root } from "../../lib.js";

const h = readV8("src/objects/prototype-info.h");

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
const inlpp = clangE("src/objects/prototype-info-inl.h");

// ── 1. CachedHandlerIndex 枚举 (pp 产物) ─────────────────────────────────
const enumSeg = hpp.slice(hpp.indexOf("enum CachedHandlerIndex"));
const enumBody = enumSeg.slice(0, enumSeg.indexOf("};"));
const enumNames = [...enumBody.matchAll(/^\s*(k\w+)(?: = (\d+))?,?$/gm)]
  .map((m) => ({ name: m[1], val: m[2] === undefined ? null : parseInt(m[2], 10) }));
must(enumNames.length === 3, `CachedHandlerIndex ${enumNames.length} != 3`);
must(enumNames[0].name === "kLoadNonExistentHandlerDefault" && (enumNames[0].val ?? 0) === 0,
  "首项失配");
must(enumNames[1].name === "kLoadNonExistentHandlerWithLookupOnReceiver" && (enumNames[1].val ?? 1) === 1,
  "次项失配");
must(enumNames[2].name === "kCachedHandlerCount", "计数哨兵失配");
// 原文锚 (显式 = 0 只标首项)
must(h.includes("kLoadNonExistentHandlerDefault = 0,"), "原文首项显式 0 锚缺失");

// ── 2. UNREGISTERED / PrototypeUsers 槽位常量 (pp 产物) ──────────────────
must(hpp.includes("static const int UNREGISTERED = -1;"), "pp 缺 UNREGISTERED");
for (const [name, val] of [
  ["kEmptySlotIndex", 0],
  ["kFirstIndex", 1],
  ["kNoEmptySlotsMarker", 0],
]) {
  const m = hpp.match(new RegExp(`static const int ${name} = (\\d+);`));
  must(m !== null && parseInt(m[1], 10) === val, `pp 缺 ${name}=${val}`);
}

// ── 3. ShouldBeFastBit 位段 (pp 产物) ────────────────────────────────────
must(hpp.includes("using ShouldBeFastBit = base::BitField<bool, 0, 1, uint32_t>;"),
  "pp 缺 ShouldBeFastBit 位链锚");

// ── 4. 字段表 (pp TaggedMember 序, 多行折叠) ─────────────────────────────
const piSeg = hpp.slice(hpp.indexOf("class PrototypeInfo : public Struct"));
const piBody = piSeg.slice(0, piSeg.indexOf("V8_OBJECT_END")).replace(/\n\s+/g, " ");
const members = [...piBody.matchAll(/TaggedMember<([\w<>, ]+?)> (\w+)(?: |\n|;)/g)]
  .map((m) => [m[2].replace(/_$/, ""), m[1]]);
must(members.length === 8, `pp 字段数 ${members.length} != 8`);
const MEMBER_TYPES = {
  module_namespace: "JSModuleNamespace | Undefined",
  prototype_users: "WeakArrayList | Zero",
  prototype_chain_enum_cache: "FixedArray | Zero | Undefined",
  registry_slot: "Smi",
  bit_field: "Smi",
  derived_maps: "WeakArrayList | Undefined",
  prototype_shared_closure_info: "PrototypeSharedClosureInfo | Undefined",
  cached_handler: "LoadHandler | Zero",
};
members.forEach(([n], i) => {
  must(Object.keys(MEMBER_TYPES)[i] === n, `pp 字段序失配 @${i}: ${n}`);
});

// ── 5. -inl.h 访问器签名清单 (pp 定义计数; 含宏展开与多行返回类型) ───────
// 列首定义行: "RET Class::method(" 或折行后的 "Class::method("
const inlDefs = [...inlpp.matchAll(/^(?:[A-Za-z_][\w:<>,&* ]*? )?((?:PrototypeInfo|PrototypeUsers)::\w+)\(/gm)]
  .map((m) => m[1]);
must(inlDefs.length === 28, `pp -inl 定义数 ${inlDefs.length} != 28`);
// 重载/宏展开多重计数 (derived_maps x2 / set_derived_maps x2 / ObjectCreateMap x2;
// should_be_fast_map 由 BOOL_ACCESSORS 宏展开补出)
const expectMult = {
  "PrototypeInfo::PrototypeInfo": 1,
  "PrototypeInfo::module_namespace": 1, "PrototypeInfo::set_module_namespace": 1,
  "PrototypeInfo::prototype_users": 1, "PrototypeInfo::set_prototype_users": 1,
  "PrototypeInfo::prototype_chain_enum_cache": 1, "PrototypeInfo::set_prototype_chain_enum_cache": 1,
  "PrototypeInfo::registry_slot": 1, "PrototypeInfo::set_registry_slot": 1,
  "PrototypeInfo::bit_field": 1, "PrototypeInfo::set_bit_field": 1,
  "PrototypeInfo::derived_maps": 2, "PrototypeInfo::set_derived_maps": 2,
  "PrototypeInfo::prototype_shared_closure_info": 1, "PrototypeInfo::set_prototype_shared_closure_info": 1,
  "PrototypeInfo::cached_handler": 1, "PrototypeInfo::set_cached_handler": 1,
  "PrototypeInfo::ObjectCreateMap": 2,
  "PrototypeInfo::SetObjectCreateMap": 1, "PrototypeInfo::GetDerivedMap": 1, "PrototypeInfo::AddDerivedMap": 1,
  "PrototypeInfo::should_be_fast_map": 1,
  "PrototypeUsers::MarkSlotEmpty": 1,
  "PrototypeUsers::empty_slot_index": 1, "PrototypeUsers::set_empty_slot_index": 1,
};
const mult = {};
for (const d of inlDefs) mult[d] = (mult[d] ?? 0) + 1;
for (const [k, v] of Object.entries(expectMult))
  must(mult[k] === v, `pp -inl ${k} 计数 ${mult[k] ?? 0} != ${v}`);

let rust = banner("src/objects/prototype-info.h.js", "src/objects/prototype-info.{h,-inl.h}",
  "PrototypeInfo 常量/位段/字段表 + -inl 签名清单 (clang -E 对数)");
rust += `//! PrototypeInfo: 原型 map 元数据容器 (访问器 + 派生 map 缓存 + 注册槽)。
//! tq 布局镜像归 crate::prototype_info_tq; 本模块落 C++ 侧规则代码。

/// 缓存 handler 槽位 (原版 enum CachedHandlerIndex)。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(usize)]
pub enum CachedHandlerIndex {
  /// 无访问检查、不在 receiver 上查找 (kLoadNonExistentHandlerDefault = 0)。
  LoadNonExistentHandlerDefault = 0,
  /// 需在 receiver 上查找 (kLoadNonExistentHandlerWithLookupOnReceiver = 1)。
  LoadNonExistentHandlerWithLookupOnReceiver = 1,
}

/// 未注册标记 (原版 PrototypeInfo::UNREGISTERED)。
pub const UNREGISTERED: i32 = -1;

/// ShouldBeFastBit: bit_field_ 的 should_be_fast 位 (1 bit @0, uint32 容器)。
pub const SHOULD_BE_FAST_BIT_SHIFT: u32 = 0;
/// ShouldBeFastBit 位宽。
pub const SHOULD_BE_FAST_BIT_BITS: u32 = 1;
/// ShouldBeFastBit 掩码。
pub const SHOULD_BE_FAST_BIT_MASK: u32 = 0x1;

/// 原型用户登记表槽位 (原版 PrototypeUsers, WeakArrayList 语义)。
pub mod prototype_users {
  /// 空槽链头槽位。
  pub const K_EMPTY_SLOT_INDEX: usize = 0;
  /// 首个可用槽位。
  pub const K_FIRST_INDEX: usize = 1;
  /// 无空槽标记值 (原版 kNoEmptySlotsMarker = Smi(0))。
  pub const K_NO_EMPTY_SLOTS_MARKER: i32 = 0;
}

/// C++ 成员字段表 (TaggedMember 声明序 1:1)。
pub const PROTOTYPE_INFO_MEMBERS: [(&str, &str); ${members.length}] = [
${members.map(([n]) => `  ("${n}_", "${MEMBER_TYPES[n]}"),`).join("\n")}
];

// —— -inl.h 签名清单 (28 项含重载与宏展开, pp 定义计数对数; 语义面留 Developer 修订) ——
${inlDefs.map((d) => `// ${d}`).join("\n")}

// —— 提取不变式断言 ——
const _: () = {
  assert!(CachedHandlerIndex::LoadNonExistentHandlerDefault as usize == 0);
  assert!(CachedHandlerIndex::LoadNonExistentHandlerWithLookupOnReceiver as usize == 1);
  assert!(UNREGISTERED == -1);
  assert!((SHOULD_BE_FAST_BIT_MASK == 0x1) && (SHOULD_BE_FAST_BIT_SHIFT == 0));
  assert!(PROTOTYPE_INFO_MEMBERS.len() == 8);
  assert!(prototype_users::K_FIRST_INDEX == 1);
};
`;

const p = writeSrc("v8-objects-maps", "prototype_info.rs", rust);
fmtNormalize(p);
console.log(`prototype_info.rs: 枚举 ${enumNames.length} + 位段 1 + 字段 ${members.length} + -inl 签名 ${inlDefs.length} (pp 对数), 断言 ${checks} 项通过 → ${p}`);
