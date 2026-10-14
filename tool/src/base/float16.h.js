#!/usr/bin/env bun
// tool/src/base/float16.h.js — 族: src/base/float16.h
// 覆盖: Float16 位容器 (fp16 库换算) — Read/Write/FromFloat32/FromBits/ToFloat32
// → v8-base/src/float16.rs.skel

import { readV8, writeSrc, banner } from "../../lib.js";

const src = readV8("src/base/float16.h");

// —— 锚点断言 ——
if (!src.includes("class Float16 {")) throw new Error("缺 Float16 类");
if (!src.includes("#include \"third_party/fp16/src/include/fp16.h\"")) throw new Error("缺 fp16 三方库 include");
if (!src.includes("base::ReadUnalignedValue<uint16_t>(source)")) throw new Error("缺非对齐读");
if (!src.includes("fp16_ieee_from_fp32_value(f32)")) throw new Error("缺 f32→bits 换算");
if (!src.includes("fp16_ieee_to_fp32_value(bits_)")) throw new Error("缺 bits→f32 换算");
if (!src.includes("static_assert(sizeof(Float16) == sizeof(uint16_t));")) throw new Error("缺尺寸断言");

// —— 真解析: 方法面计数 ——
const methods = ["Read", "Write", "FromFloat32", "FromBits", "get_bits", "ToFloat32"];
for (const m of methods) if (!src.includes(m)) throw new Error(`缺方法 ${m}`);
const pubCount = (src.match(/^ (static )?\w[\w:<>* ]* \w+\(/gm) || []).length;
console.log(`float16: 方法 ${methods.length}, 位宽断言 1 (sizeof == 2)`);

let rust = banner(
  "src/base/float16.h.js",
  "src/base/float16.h",
  "IEEE 半精度位容器 (换算委托 fp16 三方库)",
);

rust += `
//! v8: src/base/float16.h

/// v8: src/base/float16.h Float16
/// 纯 uint16_t bits_ 位容器 (sizeof == 2); 换算走 third_party/fp16
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Float16 {
    bits: u16,
}

impl Float16 {
    /// v8: Float16() — bits_(0)
    pub const fn new() -> Self;

    /// v8: Float16::Read — ReadUnalignedValue<u16>(source) (地址非对齐安全)
    pub fn read(source: usize) -> Self;

    /// v8: Float16::Write — WriteUnalignedValue<u16>(destination, bits_)
    pub fn write(&self, destination: usize);

    /// v8: Float16::FromFloat32 — fp16_ieee_from_fp32_value
    pub fn from_float32(f32: f32) -> Self;

    /// v8: Float16::FromBits
    pub const fn from_bits(bits: u16) -> Self;

    /// v8: Float16::get_bits
    pub const fn get_bits(self) -> u16;

    /// v8: Float16::ToFloat32 — fp16_ieee_to_fp32_value
    pub fn to_float32(self) -> f32;
}

// v8: 换算内核 fp16_ieee_from_fp32_value / fp16_ieee_to_fp32_value 属三方库,
//     Rust 侧待 Developer 定: 引 half crate 或手写位级换算, 登记跳过
`;

writeSrc("v8-base", "float16.rs.skel", rust);
console.log("written v8-base/src/float16.rs.skel");
