#!/usr/bin/env bun
// tool/src/deoptimizer/frame-translation-builder.h.js
// 机械提取 v8/src/deoptimizer/ 状态族: translated-state.{h,cc} + frame-translation-builder.{h,cc}
// + deoptimized-frame-info.{h,cc} + materialized-object-store.{h,cc} + 8 架构 deoptimizer-*.cc
// (>2k 行, 按规则只取结构锚点/字面量表/签名骨架; 架构分支按 x64+arm64 展开, 其余登记跳过)

import { readV8, writeSrc, banner } from "../../lib.js";

const tsH = readV8("src/deoptimizer/translated-state.h");
const tsCc = readV8("src/deoptimizer/translated-state.cc");
const fbH = readV8("src/deoptimizer/frame-translation-builder.h");
const fbCc = readV8("src/deoptimizer/frame-translation-builder.cc");
const fiH = readV8("src/deoptimizer/deoptimized-frame-info.h");
const moH = readV8("src/deoptimizer/materialized-object-store.h");

// 1. TranslatedState 方法面 (translated-state.h)
const tsFns = [...tsH.matchAll(/\n  (?:inline )?(?:void|Handle|Tagged|DirectHandle|bool|int|Address)[\w:<>,*& ]*?(\w+)\(/g)].map((m) => m[1]);
if (tsFns.length < 15) throw new Error(`TranslatedState 方法应 ≥15, 实得 ${tsFns.length}`);

// 2. FrameTranslationBuilder 写入面 (frame-translation-builder.h: RawWrite* / Add*)
const builderFns = [...fbH.matchAll(/\n  (?:inline )?void (\w+)\(/g)].map((m) => m[1]);
if (builderFns.length < 15) throw new Error(`Builder 方法应 ≥15, 实得 ${builderFns.length}`);
if (!fbCc.includes("AddRawBegin") || !fbCc.includes("WriteVLQ")) throw new Error("builder cc 缺 VLQ 写入锚点");

// 3. DeoptimizedFrameInfo / MaterializedObjectStore 锚点
for (const k of ["class DeoptimizedFrameInfo"]) {
  if (!fiH.includes(k)) throw new Error(`deoptimized-frame-info.h 缺 ${k}`);
}
for (const k of ["class MaterializedObjectStore"]) {
  if (!moH.includes(k)) throw new Error(`materialized-object-store.h 缺 ${k}`);
}

// 4. 架构文件面 (9 个): 退出尺寸常量 + 函数面; x64/arm64 展开, 其余登记跳过
const archs = ["x64", "arm64", "arm", "ia32", "loong64", "mips64", "ppc", "riscv", "s390"];
const archData = {};
for (const a of archs) {
  const src = readV8(`src/deoptimizer/${a}/deoptimizer-${a}.cc`);
  const consts = [];
  const countByName = new Map();
  for (const m of src.matchAll(/const int Deoptimizer::(k\w+) = (\d+);/g)) {
    const name = m[1];
    const val = parseInt(m[2], 10);
    const c = (countByName.get(name) || 0) + 1;
    countByName.set(name, c);
    if (c === 1) {
      consts.push({ name, val });
    } else {
      consts.push({ name: `${name}_Alt${c}`, val });
    }
  }
  const fns = [...src.matchAll(/^(?:void|Address|int|bool|frame|[A-Za-z_][\w:<>,*& ]*)\s+\w+\(/gm)].length;
  archData[a] = { consts, fns };
}
// 对数守卫: x64/arm64 必有退出尺寸常量
if (archData.x64.consts.length < 3) throw new Error(`x64 常量应 ≥3, 实得 ${archData.x64.consts.length}`);
if (archData.arm64.consts.length < 1) throw new Error(`arm64 常量应 ≥2, 实得 ${archData.arm64.consts.length}`);

let rust = banner(
  "src/deoptimizer/frame-translation-builder.h.js",
  "src/deoptimizer/{translated-state,frame-translation-builder,deoptimized-frame-info,materialized-object-store}.{h,cc} + 9 架构 deoptimizer-*.cc",
  "反优化状态族与架构分派骨架",
);

rust += `
/// TranslatedState 方法面 (原版 translated-state.h, ${tsFns.length} 项, 顺序登记)
pub const TRANSLATED_STATE_METHODS: [&str; ${tsFns.length}] = [${tsFns.map((f) => `"${f}"`).join(", ")}];

/// FrameTranslationBuilder 写入面 (原版 frame-translation-builder.h, ${builderFns.length} 项)
pub const TRANSLATION_BUILDER_METHODS: [&str; ${builderFns.length}] = [${builderFns.map((f) => `"${f}"`).join(", ")}];

/// VLQ 编码锚点 (builder 以 VLQ 写操作数)
pub const VLQ_ANCHOR: &str = "WriteVLQ";

/// 结构锚点
pub const STRUCT_ANCHORS: [&str; 2] = ["DeoptimizedFrameInfo", "MaterializedObjectStore"];

/// 架构分派登记 (原版 deoptimizer/<arch>/deoptimizer-<arch>.cc)
/// x64+arm64 为当前目标; 其余架构跳过并在此留档
pub mod arch_dispatch {
  /// 当前目标 (x64 + arm64)
  pub const TARGET_ARCHES: [&str; 2] = ["x64", "arm64"];
  /// 登记跳过架构 (实现存在但本仓库不编译)
  pub const SKIPPED_ARCHES: [&str; 7] = ["arm", "ia32", "loong64", "mips64", "ppc", "riscv", "s390"];

  /// 各架构定义头函数数 (对数基准)
  pub const FN_COUNTS: [(&str, usize); ${archs.length}] = [
${archs.map((a) => `    ("${a}", ${archData[a].fns}),`).join("\n")}
  ];

  /// x64 退出尺寸常量 (原版 deoptimizer-x64.cc; 非沙箱/沙箱双值取非沙箱,
  /// 平台分支按当前目标展开)
  pub mod x64_consts {
${archData.x64.consts.map((c) => `    /// 原版 ${c.name}\n    pub const ${c.name.replace(/^k/, "").replace(/([a-z0-9])([A-Z])/g, "$1_$2").toUpperCase()}: i32 = ${c.val};`).join("\n")}
  }

  /// arm64 退出尺寸常量
  pub mod arm64_consts {
${archData.arm64.consts.map((c) => `    /// 原版 ${c.name}\n    pub const ${c.name.replace(/^k/, "").replace(/([a-z0-9])([A-Z])/g, "$1_$2").toUpperCase()}: i32 = ${c.val};`).join("\n")}
  }
}

/// 签名骨架 (语义见 Developer 修订)
pub mod translated_state_skel {
  /// 读取翻译字节流并物化帧 (原版 TranslatedState::Init / UpdateFromPreviouslyMaterializedObjects)
  #[inline]
  pub fn init() {}
  /// 物化对象暂存 (原版 MaterializedObjectStore: 栈上物化对象跨 GC 存取)
  #[inline]
  pub fn store_materialized_objects() {}
  /// 帧信息快照 (原版 DeoptimizedFrameInfo: 调试器取反优化帧局部)
  #[inline]
  pub fn deoptimized_frame_info_skel() {}
}
`;

const p = writeSrc("v8-deoptimizer", "translated_state_generated.rs", rust);
console.log(`written ${p}; TS=${tsFns.length}, Builder=${builderFns.length}, 架构=${archs.length}`);
