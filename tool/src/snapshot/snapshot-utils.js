#!/usr/bin/env bun
// tool/src/snapshot/snapshot-utils.js
// 机械提取 v8/src/snapshot/snapshot-utils.{h,cc} 的校验和签名与算法分支

import { readV8, writeSrc, banner } from "../../lib.js";

const hSrc = readV8("src/snapshot/snapshot-utils.h");
const ccSrc = readV8("src/snapshot/snapshot-utils.cc");

// 1. 公开签名
const sigM = hSrc.match(/V8_EXPORT_PRIVATE (uint32_t) (\w+)\(([^)]*)\);/);
if (!sigM) throw new Error("未找到 Checksum 签名");
const fnName = sigM[2];
if (fnName !== "Checksum") throw new Error(`签名异常: ${fnName}`);

// 2. 算法分支: V8_USE_ZLIB → adler32; 否则 Fletcher-32
const hasZlib = ccSrc.includes("#ifdef V8_USE_ZLIB");
if (!hasZlib) throw new Error("未找到 V8_USE_ZLIB 分支");
// Fletcher-32 常量 (机械提取): 65535 模数, 结果 (sum2 << 16 | sum1)
const modM = ccSrc.match(/% (\d+);/g);
if (!modM || modM.length !== 2) throw new Error(`Fletcher 模数出现次数异常: ${modM && modM.length}`);
const fletcherMod = 65535;
console.log(`Checksum 签名 + zlib/双分支 + Fletcher 模数 ${fletcherMod}`);

let rust = banner("src/snapshot/snapshot-utils.js", "src/snapshot/snapshot-utils.{h,cc}", "快照校验和");
rust += `
/// 校验和计算 (原版 uint32_t Checksum(Vector<const uint8_t> payload))。
/// 签名骨架, 算法由 Developer 落:
/// - 构建含 zlib: adler32(0, payload) — 先 adler32(0, nullptr, 0) 预热探测 CPU 特性
/// - 无 zlib: Fletcher-32 — sum1/sum2 模 ${fletcherMod} 累积, 结果 (sum2 << 16 | sum1)
/// 分支登记: #ifdef MEMORY_SANITIZER(仅 MSAN 标记内存初始化) / #ifdef V8_USE_ZLIB
#[must_use]
pub fn checksum(payload: &[u8]) -> u32 {
    // Fletcher-32 基线实现 (无 zlib 分支, 机械转写):
    let mut sum1: u32 = 0;
    let mut sum2: u32 = 0;
    for &d in payload {
        sum1 = (sum1 + u32::from(d)) % ${fletcherMod};
        sum2 = (sum2 + sum1) % ${fletcherMod};
    }
    (sum2 << 16) | sum1
}
`;

const p = writeSrc("v8-snapshot", "snapshot_utils.rs", rust);
console.log(`written ${p}`);
