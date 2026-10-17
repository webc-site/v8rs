#!/usr/bin/env bun
// tool/src/base/numbers/bignum.h.js — 族: src/base/numbers/bignum.h + .cc
// 覆盖: Bignum (28 位 bigit 定容大数) — 常量 6 + 公有方法 26 + 私有 8
// → v8-base/src/numbers/bignum.rs.skel

import { readV8, writeSrc, banner } from "../../../lib.js";

const hSrc = readV8("src/base/numbers/bignum.h");
const ccSrc = readV8("src/base/numbers/bignum.cc");

// —— 锚点断言 ——
if (!hSrc.includes("class V8_BASE_EXPORT Bignum {")) throw new Error("缺 Bignum 类");
if (!hSrc.includes("static const int kMaxSignificantBits = 3584;")) throw new Error("缺位宽上限");
if (!hSrc.includes("using Chunk = uint32_t;")) throw new Error("缺 Chunk");
if (!hSrc.includes("static const int kBigitSize = 28;")) throw new Error("缺 bigit 位宽");
if (!hSrc.includes("static const int kBigitCapacity = kMaxSignificantBits / kBigitSize;")) throw new Error("缺容量公式");
if (!hSrc.includes("int BigitLength() const { return used_digits_ + exponent_; }")) throw new Error("缺 BigitLength 内联");
if (!hSrc.includes("// The Bignum's value equals value(bigits_) * 2^(exponent_ * kBigitSize).")) throw new Error("缺值语义注释");

// —— 真解析: cc 方法定义对数 ——
const defs = new Set([...ccSrc.matchAll(/^(?:void|uint16_t|int|bool|Bignum::Chunk) Bignum::(\w+)\(/gm)].map((m) => m[1]));
const expected = [
  "AssignUInt16", "AssignUInt64", "AssignBignum", "AssignDecimalString", "AssignHexString",
  "AssignPowerUInt16", "AddUInt64", "AddBignum", "SubtractBignum", "Square",
  "ShiftLeft", "MultiplyByUInt32", "MultiplyByUInt64", "MultiplyByPowerOfTen",
  "DivideModuloIntBignum", "ToHexString", "Compare", "PlusCompare",
  "Align", "Clamp", "IsClamped", "Zero", "BigitsShiftLeft", "BigitAt", "SubtractTimes",
];
for (const e of expected) if (!defs.has(e)) throw new Error(`cc 缺定义 ${e}`);
// v8 怪癖: AddUInt16 在 .h 声明但 .cc 无定义 (死声明, 亦无调用)
if (!hSrc.includes("void AddUInt16(uint16_t operand);")) throw new Error("h 缺 AddUInt16 声明");
if (defs.has("AddUInt16")) throw new Error("AddUInt16 不应有定义");
// bigit 容量 = 3584 / 28 = 128
const capacity = 3584 / 28;
if (capacity !== 128) throw new Error("容量公式异常");
console.log(`bignum: cc 定义 ${expected.length}, bigit 28 位 × ${capacity} 容量, Comba 乘法`);

let rust = banner(
  "src/base/numbers/bignum.h.js",
  "src/base/numbers/bignum.h + src/base/numbers/bignum.cc",
  "定容大整数 (dtoa 精确算法用, 禁拷贝)",
);

rust += `
//! v8: src/base/numbers/bignum.h/.cc

// v8: bignum.h 常量
/// 最大精确位: 2^3584 > 10^1000
pub const MAX_SIGNIFICANT_BITS: i32 = 3584;
/// bigit 位宽 28 (双精度尾数恰装两 chunk, 支持 Comba 乘法)
pub const BIGIT_SIZE: i32 = 28;
/// bigit 掩码 (1<<28)-1
pub const BIGIT_MASK: u32 = (1 << 28) - 1;
/// 容量 = 3584/28 = 128 chunk (栈上定容, 不增长)
pub const BIGIT_CAPACITY: usize = ${capacity};
/// Chunk = u32; DoubleChunk = u64
pub type Chunk = u32;

/// v8: bignum.h Bignum
/// 值 = value(bigits_) * 2^(exponent_ * kBigitSize); used_digits_ 记有效 bigit 数;
/// bigits_buffer_[128] 栈上定容 + Vector 越界检查包装; 禁拷贝 (静态数组不可移)
pub struct Bignum {
    bigits: [Chunk; BIGIT_CAPACITY],
    used_digits: i32,
    exponent: i32,
}

impl Bignum {
    /// v8: Bignum() — Zero()
    pub fn new() -> Self;
    // —— 赋值 ——
    /// v8: AssignUInt16 / AssignUInt64 / AssignBignum
    pub fn assign_uint16(&mut self, value: u16);
    pub fn assign_uint64(&mut self, value: u64);
    pub fn assign_bignum(&mut self, other: &Bignum);
    /// v8: AssignDecimalString / AssignHexString (十进制 ASCII 逐位乘加)
    pub fn assign_decimal_string(&mut self, value: &[u8]);
    pub fn assign_hex_string(&mut self, value: &[u8]);
    /// v8: AssignPowerUInt16 — 幂 via 反复平方法 (平方 + 乘)
    pub fn assign_power_uint16(&mut self, base: u16, exponent: i32);
    // —— 算术 ——
    // v8: AddUInt16 — 头文件声明但 .cc 无定义无调用 (v8 死声明), Rust 侧登记跳过
    /// v8: AddUInt64 / AddBignum (先 Align 指数)
    pub fn add_uint64(&mut self, operand: u64);
    pub fn add_bignum(&mut self, other: &Bignum);
    /// v8: SubtractBignum — 前置 this >= other
    pub fn subtract_bignum(&mut self, other: &Bignum);
    /// v8: Square — Comba 乘法
    pub fn square(&mut self);
    /// v8: ShiftLeft / BigitsShiftLeft (by < kBigitSize)
    pub fn shift_left(&mut self, shift_amount: i32);
    /// v8: MultiplyByUInt32 / MultiplyByUInt64 / MultiplyByPowerOfTen / Times10
    pub fn multiply_by_uint32(&mut self, factor: u32);
    pub fn multiply_by_uint64(&mut self, factor: u64);
    /// v8: MultiplyByPowerOfTen — 小幂查 kSmallPowersOfTen, 大幂 assign+乘
    pub fn multiply_by_power_of_ten(&mut self, exponent: i32);
    /// v8: DivideModuloIntBignum — 返回商 (0..65535), this 变余数; 最坏 O(this/other)
    pub fn divide_modulo_int_bignum(&mut self, other: &Bignum) -> u16;
    /// v8: ToHexString
    pub fn to_hex_string(&self, buffer: &mut [u8]) -> bool;
    // —— 比较 ——
    /// v8: Compare — 按符号位对齐 BigitLength 后逐 bigit 比较
    pub fn compare(a: &Bignum, b: &Bignum) -> std::cmp::Ordering;
    /// v8: Equal / LessEqual / Less — Compare 包装
    pub fn equal(a: &Bignum, b: &Bignum) -> bool;
    pub fn less_equal(a: &Bignum, b: &Bignum) -> bool;
    pub fn less(a: &Bignum, b: &Bignum) -> bool;
    /// v8: PlusCompare / PlusEqual / PlusLessEqual / PlusLess — a+b 与 c 比较 (不物化和)
    pub fn plus_compare(a: &Bignum, b: &Bignum, c: &Bignum) -> std::cmp::Ordering;
    pub fn plus_equal(a: &Bignum, b: &Bignum, c: &Bignum) -> bool;
    pub fn plus_less_equal(a: &Bignum, b: &Bignum, c: &Bignum) -> bool;
    pub fn plus_less(a: &Bignum, b: &Bignum, c: &Bignum) -> bool;
    // —— 私有 ——
    // v8: EnsureCapacity (>capacity 则 UNREACHABLE) / Align / Clamp / IsClamped / Zero
    // v8: BigitLength = used_digits_ + exponent_ (含指数隐藏位) / BigitAt (越界返回 0)
    // v8: SubtractTimes — 减 other×factor (除法内核)
}
`;

writeSrc("v8-base", "numbers/bignum.rs.skel", rust);
console.log("written v8-base/src/numbers/bignum.rs.skel");
