#!/usr/bin/env bun
// tool/src/init/bootstrapper.js
// 机械提取 v8/src/init/bootstrapper.{h,cc} — cc 超 2k 行 (7191), 按规则只提取
// 头签名骨架 + 常量; cc 侧不展开实现。

import { readV8, writeSrc, banner } from "../../lib.js";

const hSrc = readV8("src/init/bootstrapper.h");
const ccSrc = readV8("src/init/bootstrapper.cc");

// 1. 常量: kDefaultContextIndex = 0 (h 内 CreateEnvironmentForTesting)
const defCtxM = hSrc.match(/static constexpr int kDefaultContextIndex = (\d+);/);
if (!defCtxM) throw new Error("未找到 kDefaultContextIndex");

// 2. SourceCodeCache 方法签名 (h)
const sccM = hSrc.match(/class SourceCodeCache final \{([\s\S]*?)\n\};/);
if (!sccM) throw new Error("未找到 SourceCodeCache 类体");
const sccSigs = [...sccM[1].matchAll(/(?:void|bool)\s+(\w+)\(([^)]*)\)/g)].map(
  (m) => ({ name: m[1], args: m[2].replace(/\s+/g, " ").trim() }),
);
if (sccSigs.length !== 4) {
  throw new Error(`SourceCodeCache 签名数不符: 期望 4, 实得 ${sccSigs.length}`);
}

// 3. Bootstrapper 公开方法签名 (h)
const bsM = hSrc.match(/class Bootstrapper final \{([\s\S]*?)\n private:/);
if (!bsM) throw new Error("未找到 Bootstrapper 类体");
const bsSigs = [
  ...bsM[1].matchAll(
    /(?:static\s+)?(?:void|bool|int|char\*|DirectHandle<NativeContext>|DirectHandle<JSGlobalProxy>|SourceCodeCache\*)\s+(\w+)\(/g,
  ),
].map((m) => m[1]);
if (bsSigs.length !== 15) {
  throw new Error(`Bootstrapper 签名数不符: 期望 15, 实得 ${bsSigs.length}: ${bsSigs.join(",")}`);
}
console.log(`提取 SourceCodeCache ${sccSigs.length} + Bootstrapper ${bsSigs.length} 签名`);

// 4. cc 规模登记 (超 2k 行规则: 只登记, 不展开)
const ccLines = ccSrc.split("\n").length;
if (ccLines < 7000) throw new Error(`bootstrapper.cc 行数异常: ${ccLines}`);

let rust = banner("src/init/bootstrapper.js", "src/init/bootstrapper.{h,cc}", "全局上下文引导 (超大 cc, 仅签名骨架)");
rust += `
/// Bootstrapper: 创建 JavaScript 全局上下文的公共入口;
/// 原版 cc 共 ${ccLines} 行 (>2k), 按提取规则仅登记签名骨架, 实现由 Developer 落。

/// 默认上下文索引 (原版 kDefaultContextIndex)
pub const DEFAULT_CONTEXT_INDEX: i32 = ${defCtxM[1]};

/// SourceCodeCache: FixedArray 存 (OneByteString, SharedFunctionInfo) 对,
/// 将原生扩展代码文件名映射到预编译函数。签名骨架:
`;
for (const s of sccSigs) {
  rust += `/// - \`${s.name}(${s.args})\`\n`;
}
rust += `
/// Bootstrapper 公开接口签名骨架 (原版 bootstrapper.h, 共 ${bsSigs.length} 条):
`;
for (const s of bsSigs) {
  rust += `/// - \`${s}\`\n`;
}
rust += `
/// Bootstrapper::IsActive 契约: nesting_ != 0 即引导进行中;
/// ArchiveSpacePerThread/ArchiveState/RestoreState/FreeThreadResources 为线程抢占支撑。
`;

const p = writeSrc("v8-init", "bootstrapper.rs", rust);
console.log(`written ${p}`);
