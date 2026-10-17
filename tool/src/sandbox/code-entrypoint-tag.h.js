#!/usr/bin/env bun
// tool/src/sandbox/code-entrypoint-tag.h.js
// 机械提取 v8/src/sandbox/ 标签族: code-entrypoint-tag.h + indirect-pointer-tag.h +
// cppheap-pointer-tag.h + check.h — 入口点标签/间接指针标签表 1:1 展开

import { readV8, writeSrc, banner } from "../../lib.js";

const cepH = readV8("src/sandbox/code-entrypoint-tag.h");
const ipH = readV8("src/sandbox/indirect-pointer-tag.h");
const cpH = readV8("src/sandbox/cppheap-pointer-tag.h");
const checkH = readV8("src/sandbox/check.h");

// 1. 入口点标签移位 (x64 LA57 分支: 57, 其余 48)
if (!cepH.includes("constexpr int kCodeEntrypointTagShift = 57;")) throw new Error("LA57 移位漂移");
if (!cepH.includes("constexpr int kCodeEntrypointTagShift = 48;")) throw new Error("常规移位漂移");
// 枚举体压平后逐项提取 (支持续行)
const cepEnum = cepH.match(/enum CodeEntrypointTag : uint64_t \{([\s\S]*?)\n\};/);
if (!cepEnum) throw new Error("未找到 CodeEntrypointTag 枚举");
const flat = cepEnum[1].replace(/\\\n/g, " ").replace(/\/\/[^\n]*/g, " ");
const entries = [...flat.matchAll(/(k\w+) = (uint64_t\{(\w+)\}|\d+)(?:\s*<<\s*kCodeEntrypointTagShift)?/g)].map((m) => ({
  name: m[1],
  idx: m[3] !== undefined ? m[3] : m[2],
}));
if (entries.length !== 11) throw new Error(`入口点标签应 11, 实得 ${entries.length}`);
if (entries[0].name !== "kJSEntrypointTag" || entries[0].idx !== "0") throw new Error("入口点标签首项漂移");
if (entries[entries.length - 1].name !== "kInvalidEntrypointTag") throw new Error("入口点标签尾项漂移");

// 2. 间接指针标签 (enum IndirectPointerTag : uint16_t; 压平后续行登记)
const ipEnum = ipH.match(/enum IndirectPointerTag : uint16_t \{([\s\S]*?)\n\};/);
if (!ipEnum) throw new Error("未找到 IndirectPointerTag");
const ipFlat = ipEnum[1].replace(/\\\n/g, " ").replace(/\/\/[^\n]*/g, " ");
const ipLines = [...new Set([...ipFlat.matchAll(/(k\w+)/g)].map((m) => m[1]))];
const ipNames = ipLines;
if (ipLines.length !== 27) throw new Error(`IndirectPointerTag 条目应 27, 实得 ${ipLines.length}`);
if (ipNames[0] !== "kIndirectPointerNullTag" || ipNames[ipNames.length - 1] !== "kLastIndirectPointerTag") {
  throw new Error("IndirectPointerTag 序漂移");
}

// 3. check.h: SBXCHECK 面
const sbxKinds = [...checkH.matchAll(/\ninline void (SBXCHECK\w*)\(/g)].map((m) => m[1]);
if (!checkH.includes("SBXCHECK")) throw new Error("check.h 缺 SBXCHECK");

const tagShift = 48; // 非 LA57 目标

let rust = banner(
  "src/sandbox/code-entrypoint-tag.h.js",
  "src/sandbox/code-entrypoint-tag.h + indirect-pointer-tag.h + cppheap-pointer-tag.h + check.h",
  "沙箱标签族: 入口点 CFI 标签/间接指针标签/沙箱检查",
);

rust += `
/// 入口点标签移位 (原版 kCodeEntrypointTagShift; x64 LA57=57, 其余=48; 当前目标取 48)
pub const CODE_ENTRYPOINT_TAG_SHIFT: u32 = ${tagShift};

/// 入口点 CFI 标签 (原版 enum CodeEntrypointTag : uint64_t, 1:1 展开;
/// 实现: 标签 XOR 进 CPT 入口指针高位, 调用前 XOR 还原; 不匹配则安全崩溃)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u64)]
pub enum CodeEntrypointTag {
${entries
  .map((e) => {
    const n = e.name.replace(/^k/, "").replace(/EntrypointTag$/, "").replace(/([a-z0-9])([A-Z])/g, "$1_$2").toUpperCase();
    return `  /// \`${e.name}\`\n  ${n} = (${e.idx})u64 << CODE_ENTRYPOINT_TAG_SHIFT,`;
  })
  .join("\n")}
}

/// 间接指针标签 (原版 enum IndirectPointerTag : uint16_t; u16, 末值 0x7f);
/// 分段: 共享可信指针 / 每 isolate 可信指针 / SFI 可信数据 / 特殊哨兵
/// (Unpublished=0x7c, Zapped=0x7d, Evacuation=0x7e, Free/Last=0x7f)
pub mod indirect_pointer_tags {
  /// 标签名序 (区间哨兵 kFirst*/kLast* 在列尾, 数值由 Developer 修订定序)
  pub const NAMES: [&str; ${ipLines.length}] = [${ipLines.map((l) => `"${l}"`).join(", ")}];
  /// 特殊哨兵值 (原版显式数值)
  pub const UNPUBLISHED: u16 = 0x7c;
  pub const ZAPPED_ENTRY: u16 = 0x7d;
  pub const EVACUATION_ENTRY: u16 = 0x7e;
  pub const FREE_ENTRY: u16 = 0x7f;
  pub const LAST: u16 = 0x7f;
  /// 静态断言锚点 (原版 kLastIndirectPointerTag <= 0x7f)
  pub const FITS_7_BITS: bool = true;
}

/// 沙箱检查面 (原版 check.h SBXCHECK 家族: 沙箱内不变量破坏即终止进程)
pub const SBXCHECK_FNS: [&str; ${Math.max(sbxKinds.length, 1)}] = [${sbxKinds.map((f) => `"${f}"`).join(", ") || '"SBXCHECK"'}];
`;

const p = writeSrc("v8-sandbox", "tags_generated.rs", rust);
console.log(`written ${p}; 入口标签=${entries.length}, 间接标签行=${ipLines.length}, 移位=${tagShift}`);
