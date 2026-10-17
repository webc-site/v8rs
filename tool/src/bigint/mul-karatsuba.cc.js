#!/usr/bin/env bun
// tool/src/bigint/mul-karatsuba.cc.js
// 机械提取 v8/src/bigint/ 乘法三族: mul-karatsuba.cc + mul-toom.cc + mul-fft.cc
// 阈值分派与算法定义头清单

import { readV8, writeSrc, banner } from "../../lib.js";

const karatsuba = readV8("src/bigint/mul-karatsuba.cc");
const toom = readV8("src/bigint/mul-toom.cc");
const fft = readV8("src/bigint/mul-fft.cc");
const inlH = readV8("src/bigint/bigint-inl.h");

// 1. config 阈值 (bigint-inl.h; 单一定义或 32/64 位成对)
function cfg(name, expected64) {
  const all = [...inlH.matchAll(new RegExp(`constexpr uint32_t ${name} = (\\d+);`, "g"))].map((x) => x[1]);
  if (all.length === 1) {
    if (parseInt(all[0], 10) !== expected64) throw new Error(`${name} 漂移: ${all[0]}`);
    return { single: parseInt(all[0], 10), bits32: null, bits64: parseInt(all[0], 10) };
  }
  if (all.length !== 2) throw new Error(`${name} 应 1 或 2 处, 实得 ${all.length}`);
  if (parseInt(all[1], 10) !== expected64) throw new Error(`${name} 64 位漂移: ${all[1]}`);
  return { single: null, bits32: parseInt(all[0], 10), bits64: parseInt(all[1], 10) };
}
const karatsubaT = cfg("kKaratsubaThreshold", 34);
const toomT = cfg("kToomThreshold", 210);
const fftT = cfg("kFftThreshold", 720);
const fftInnerT = cfg("kFftInnerThreshold", 200);
if (karatsubaT.bits64 !== 34) throw new Error("kKaratsubaThreshold 64 位漂移");
if (toomT.bits64 !== 210) throw new Error("kToomThreshold 64 位漂移");
if (fftT.bits64 !== 720) throw new Error("kFftThreshold 64 位漂移");

// 2. 递归结构对数
if ((karatsuba.match(/config::kKaratsubaThreshold/g) || []).length < 4) throw new Error("karatsuba 阈值引用不足");
// Toom 三分: 3 路切分
if (!toom.includes("/ 3") && !toom.includes("third")) throw new Error("toom 三分结构漂移");
// FFT 分块参数 (原版 kAsymmetricChunkingThreshold)
const asymM = fft.match(/static constexpr uint32_t kAsymmetricChunkingThreshold = (\d+);/);
if (!asymM || asymM[1] !== "100") throw new Error("kAsymmetricChunkingThreshold 漂移");

// 3. 定义头清单
function collectDefs(src) {
  const out = [];
  const ls = src.split("\n");
  for (let i = 0; i < ls.length; i++) {
    const l = ls[i];
    if (!/^(?:void|int|uint32_t|uint64_t|bool|static)\s/.test(l) || !/\(/.test(l)) continue;
    let sig = l;
    let j = i;
    while (!/\{\s*$|;\s*$/.test(sig) && j + 1 < ls.length) sig += " " + ls[++j].trim();
    sig = sig.replace(/\{\s*$/, "").trim();
    if (sig.endsWith(";")) continue;
    out.push(sig);
  }
  return out;
}
const kDefs = collectDefs(karatsuba);
const tDefs = collectDefs(toom);
const fDefs = collectDefs(fft);
if (kDefs.length < 3) throw new Error(`karatsuba 定义头应 ≥3, 实得 ${kDefs.length}`);
if (tDefs.length < 3) throw new Error(`toom 定义头应 ≥3, 实得 ${tDefs.length}`);
if (fDefs.length < 5) throw new Error(`fft 定义头应 ≥5, 实得 ${fDefs.length}`);

let rust = banner("src/bigint/mul-karatsuba.cc.js", "src/bigint/mul-{karatsuba,toom,fft}.cc (+bigint-inl.h config)", "乘法三族阈值与定义头清单");

rust += `
/// 乘法算法分派阈值 (原版 bigint-inl.h namespace config, 按平台展开; 64 位为当前目标)
pub mod mul_config {
  /// schoolbook → karatsuba 界 (平台无关单一值)
  pub const KARATSUBA_THRESHOLD: u32 = ${karatsubaT.bits64};
  /// karatsuba → toom 界 (x64 上 200-400 位区间两者接近)
  pub const TOOM_THRESHOLD: u32 = ${toomT.bits64};
  /// toom → fft 界 (32 位 ${fftT.bits32} / 64 位 ${fftT.bits64})
  pub const FFT_THRESHOLD_32: u32 = ${fftT.bits32};
  pub const FFT_THRESHOLD_64: u32 = ${fftT.bits64};
  /// fft 内层界 (内外两层 fft)
  pub const FFT_INNER_THRESHOLD: u32 = ${fftInnerT.bits64};

  /// 当前目标 (64 位) 生效组
  pub const ACTIVE: [&str; 4] = ["karatsuba=${karatsubaT.bits64}", "toom=${toomT.bits64}", "fft=${fftT.bits64}", "fft_inner=${fftInnerT.bits64}"];
}

/// FFT 非对称分块阈值 (原版 mul-fft.cc kAsymmetricChunkingThreshold)
pub const FFT_ASYMMETRIC_CHUNKING_THRESHOLD: u32 = ${asymM[1]};

// mul-karatsuba.cc 定义头清单 (${kDefs.length} 项):
${kDefs.map((s) => "// " + s).join("\n")}

// mul-toom.cc 定义头清单 (${tDefs.length} 项):
${tDefs.map((s) => "// " + s).join("\n")}

// mul-fft.cc 定义头清单 (${fDefs.length} 项):
${fDefs.map((s) => "// " + s).join("\n")}
`;

const p = writeSrc("v8-bigint", "mul_generated.rs", rust);
console.log(`written ${p}; karatsuba=${kDefs.length}, toom=${tDefs.length}, fft=${fDefs.length}`);
