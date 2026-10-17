#!/usr/bin/env bun
// tool/src/sandbox/external-strings-cage.h.js
// 机械提取 v8/src/sandbox/external-strings-cage.{h,cc} — 外部字符串笼布局常量。
// 产物: v8-sandbox/src/external_strings_cage.rs

import { readV8, writeSrc, banner } from "../../lib.js";

const H_REL = "src/sandbox/external-strings-cage.h";
const CC_REL = "src/sandbox/external-strings-cage.cc";
const h = readV8(H_REL);
readV8(CC_REL); // 家族覆盖锚

const maxM = h.match(/static constexpr size_t kMaxContentsSize = ([^;]+);/);
if (!maxM) throw new Error("未找到 kMaxContentsSize");
if (!maxM[1].includes("<< 33")) throw new Error("kMaxContentsSize 表达式锚不符");
const guardM = h.match(/static constexpr size_t kGuardRegionSize = size_t\{1\} << (\d+);/);
if (!guardM) throw new Error("未找到 kGuardRegionSize");
const classM = h.match(/class V8_EXPORT_PRIVATE ExternalStringsCage final\s*\{([\s\S]*?)\n\};/);
if (!classM) throw new Error("未找到 ExternalStringsCage 类");
const methods = [...classM[1].matchAll(/^\s*[\w:<>,&*\s]+?\b(\w+)\(/gm)].map((m) => m[1]);
if (methods.length < 4) throw new Error(`方法面不足: ${methods.length}`);

// GB = 1024^3
const maxContents = 2n ** 33n + 1024n ** 3n;
const guardRegion = 2n ** BigInt(guardM[1]);

let rust = banner("src/sandbox/external-strings-cage.h.js", "src/sandbox/external-strings-cage.{h,cc}", "ExternalStringsCage 布局常量");
rust += `
/// 笼内容最大尺寸 (原 kMaxContentsSize = (1<<33) + GB)
pub const MAX_CONTENTS_SIZE: usize = ${maxContents};

/// 守护区尺寸 (原 kGuardRegionSize = 1 << ${guardM[1]})
pub const GUARD_REGION_SIZE: usize = ${guardRegion};

/// 原 class ExternalStringsCage 方法面:
${[...new Set(methods)].map((s) => `// - ${s}`).join("\n")}

pub struct ExternalStringsCage;

// TODO(dev): 补笼初始化/指针进出笼实现。

// —— 提取不变式断言 ——
const _: () = {
  assert!(MAX_CONTENTS_SIZE == ${maxContents});
  assert!(GUARD_REGION_SIZE == ${guardRegion});
};
`;

const p = writeSrc("v8-sandbox", "external_strings_cage.rs", rust);
console.log(`written ${p} (常量 2, 方法 ${[...new Set(methods)].length})`);
