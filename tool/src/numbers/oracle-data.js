#!/usr/bin/env bun
// ECMAScript 数值语义差分语料 — 以 bun (JSC) 为参考实现录制 golden corpus
// 产物 v8-numbers/tests/data/oracle-*.tsv:
//   parse.tsv      : in<TAB>op<TAB>radix<TAB>out_bits   (op = number|parsefloat|parseint)
//   to_string.tsv  : in_bits<TAB>op<TAB>arg<TAB>out     (op = tostring|tofixed|toprecision|toexponential|toradix)
// JSC 与 V8 的 Number/parseInt/parseFloat/Number::toString 均为规范唯一定义,
// 无实现分歧; 录制时抛错/undefined 的组合直接跳过
import { mkdirSync, writeFileSync } from "node:fs";
import { join } from "node:path";
import { root } from "../../lib.js";

// 固定种子 PRNG (mulberry32): 复跑幂等
let _seed = 0x9e3779b9;
const Math_random = () => {
  _seed |= 0; _seed = (_seed + 0x6d2b79f5) | 0;
  let t = Math.imul(_seed ^ (_seed >>> 15), 1 | _seed);
  t = (t + Math.imul(t ^ (t >>> 7), 61 | t)) ^ t;
  return ((t ^ (t >>> 14)) >>> 0) / 4294967296;
};
const f64 = new Float64Array(1);
const u64 = new BigUint64Array(f64.buffer);
const bitsOf = (v) => {
  f64[0] = v;
  return u64[0].toString(16);
};

// ---------------------------------------------------------------------------
// 解析侧语料
// ---------------------------------------------------------------------------

const fixedParseInputs = [
  "", " ", "\t", "\n", "  \t\n ", "0", "00", "000", "-0", "+0", "0.0", ".0", "0.",
  "1", "-1", "+1", "  -1  ", "  +1  ", "- 1", "+ 1", "--1", "1.5", "-1.5", ".5", "-.5",
  "+.5", "5.", "5.e3", ".e1", ".", "..", "1.2.3", "1e", "1e+", "1e-", "1e+2", "1e-2",
  "1E2", "1e1e2", "e1", "E5", "0x", "0x1", "0xFF", "0xff", "0X10", "0xg", "0b", "0b1",
  "0b12", "0B101", "0o", "0o7", "0O17", "0o8", "08", "081", "078", "07.7", "07e8",
  "0.001", "0.713", "NaN", "nan", "-NaN", "Infinity", "-Infinity", "+Infinity",
  "infinity", "InfinityX", "-Infinity ", " Infinity", "I", "-I", "1 ", "1x", "8q",
  "10e", "10e-", "1 2", "1_000", "４２", "1e309", "1e999", "-1e999", "1e-999",
  "0e99999999999999999999", "0e-99999999999999999999", "1.7976931348623157e308",
  "5e-324", "2.2250738585072011e-308", "2.2250738585072012e-308", "2.2250738585072013e-308",
  "1e-323", "4.9e-324", "24414062505131250.0", "24414062505131250.000000001",
  "9007199254740993", "18446744073709551616", "1e21", "1e20", "1e-6", "1e-7",
];

// 随机解析语料: 数字串×点×指数×符号×junk
function randDigit() { return String(Math.floor(Math_random() * 10)); }
function randDigits(n) { return Array.from({ length: n }, randDigit).join(""); }
function randParseInput() {
  const kind = Math.floor(Math_random() * 8);
  const d = () => randDigits(1 + Math.floor(Math_random() * 20));
  const junk = Math_random() < 0.3 ? ["", "x", "  ", " 1", "e", "z9"][Math.floor(Math_random() * 6)] : "";
  const sign = ["", "-", "+"][Math.floor(Math_random() * 3)];
  switch (kind) {
    case 0: return sign + d() + junk;
    case 1: return sign + d() + "." + d() + junk;
    case 2: return sign + "." + d() + junk;
    case 3: return sign + d() + "." + junk;
    case 4: return sign + d() + "e" + ["", "-", "+"][Math.floor(Math_random() * 3)] + d() + junk;
    case 5: return sign + d() + "." + d() + "E" + ["", "-"][Math.floor(Math_random() * 2)] + d() + junk;
    case 6: return "0x" + randDigits(4).replaceAll("9", "f") + junk;
    default: return sign + d() + "." + d();
  }
}

const parseRows = [];
function recordParse(input) {
  const enc = input.replaceAll("\\", "\\\\").replaceAll("\t", "\\t").replaceAll("\n", "\\n").replaceAll("\r", "\\r");
  parseRows.push([enc, "number", "0", bitsOf(Number(input))].join("\t"));
  parseRows.push([enc, "parsefloat", "0", bitsOf(parseFloat(input))].join("\t"));
  for (const radix of [0, 2, 8, 10, 16, 36]) {
    parseRows.push([enc, "parseint", String(radix), bitsOf(parseInt(input, radix))].join("\t"));
  }
}
for (const s of fixedParseInputs) recordParse(s);
for (let i = 0; i < 1500; i++) recordParse(randParseInput());

// ---------------------------------------------------------------------------
// 格式化侧语料
// ---------------------------------------------------------------------------

function randF64() {
  // 混合: 均匀位型 + 小整数 + 指数边界
  const kind = Math.floor(Math_random() * 4);
  if (kind === 0) {
    u64[0] = BigInt(Math.floor(Math_random() * 2 ** 64));
    return f64[0];
  }
  if (kind === 1) return Math.floor(Math_random() * 1e6) * (Math_random() < 0.5 ? -1 : 1);
  if (kind === 2) return Number((Math_random() * 2 - 1).toExponential(Math.floor(Math_random() * 17)));
  return [0, -0, 1, -1, 0.5, 1e21, 1e-7, 1e20, 4.5e-324, 1.7976931348623157e308][Math.floor(Math_random() * 10)];
}

const fmtRows = [];
function recordFormat(v) {
  if (!Number.isFinite(v)) return; // NaN/Inf 由 Rust 侧静态用例覆盖
  const b = bitsOf(v);
  fmtRows.push([b, "tostring", "0", v.toString()].join("\t"));
  for (const f of [0, 1, 5, 20]) {
    fmtRows.push([b, "tofixed", String(f), v.toFixed(f)].join("\t"));
  }
  for (const p of [1, 2, 8, 21]) {
    fmtRows.push([b, "toprecision", String(p), v.toPrecision(p)].join("\t"));
  }
  for (const e of [-1, 0, 3, 20]) {
    const out = e === -1 ? v.toExponential() : v.toExponential(e);
    fmtRows.push([b, "toexponential", String(e), out].join("\t"));
  }
  for (const r of [2, 8, 16, 36]) {
    fmtRows.push([b, "toradix", String(r), v.toString(r)].join("\t"));
  }
}
const fmtFixed = [
  0, -0, 1, -1, 0.5, -0.5, 1.5, 100, 1e6, 1e15, 1e16, 1e20, 1e21, 123456789,
  0.1, 0.3, 1 / 3, 2 / 3, 1e-6, 1e-7, 5e-324, 2.2250738585072014e-308,
  1.7976931348623157e308, 2147483647, -2147483648, 4294967295, 4294967296,
  0.000001, 0.0000001, 123.456, -123.456, 999999999999999999999,
];
for (const v of fmtFixed) recordFormat(v);
for (let i = 0; i < 1200; i++) recordFormat(randF64());

// ---------------------------------------------------------------------------
// xorshift128+ / MurmurHash3 / Math.pow 差分 (bun 侧按公开算法独立重实现)
// ---------------------------------------------------------------------------

const rngRows = [];
// MurmurHash3 u64 终结化 (标准常数)
function murmurHash3(h) {
  h ^= h >> 33n; h = BigInt.asUintN(64, h * 0xff51afd7ed558ccdn);
  h ^= h >> 33n; h = BigInt.asUintN(64, h * 0xc4ceb9fe1a85ec53n);
  h ^= h >> 33n;
  return h;
}
for (const seed of [0n, 1n, 12345n, 0xdeadbeefcafebaben]) {
  rngRows.push([`murmur3`, seed.toString(16), murmurHash3(seed).toString(16)].join("\t"));
}
// xorshift128+ (V8 RandomNumberGenerator::XorShift128, 与 math.tq 同源)
function xorshift128(s0, s1) {
  s0 = BigInt.asUintN(64, s0); s1 = BigInt.asUintN(64, s1);
  let x = s0, y = s1;
  const out = [];
  for (let i = 0; i < 64; i++) {
    const n0 = y;
    let n1 = x;
    n1 ^= BigInt.asUintN(64, n1 << 23n);
    n1 ^= n1 >> 17n;
    n1 ^= n0;
    n1 ^= n0 >> 26n;
    x = n0; y = n1;
    out.push(BigInt.asUintN(64, n0 + n1).toString(16));
  }
  return out;
}
for (const seed of [12345n, 1n]) {
  const s0 = murmurHash3(seed), s1 = murmurHash3(BigInt.asUintN(64, ~seed));
  const seq = xorshift128(s0, s1);
  rngRows.push(["state", seed.toString(16), `${s0.toString(16)},${s1.toString(16)}`].join("\t"));
  seq.forEach((v, i) => rngRows.push(["xorshift128", `${seed.toString(16)}:${i}`, v].join("\t")));
  // ToDouble: (v >> 11) / 2^53
  const td = (v) => String(Number(BigInt("0x" + v) >> 11n) / 2 ** 53);
  rngRows.push(["to_double", seq[0], td(seq[0])].join("\t"));
  rngRows.push(["to_double", seq[1], td(seq[1])].join("\t"));
}
// Math.pow 差分 (ECMAScript 语义: ±1 ** ±∞ = NaN)
const powRows = [];
const powCases = [
  [1, Infinity], [-1, Infinity], [1, -Infinity], [-1, -Infinity],
  [2, 2], [-3, 2], [2, 0.5], [4, 0.5], [-0, 0.5], [-Infinity, 0.5],
  [Infinity, 0.5], [2, 3], [NaN, 2], [2, NaN], [NaN, NaN], [-8, 1 / 3],
  [-8, 2 / 3], [0, 0], [-0, 3], [0, -1], [Infinity, -1], [-Infinity, 3],
  [-Infinity, 2.5], [1.0000000000001, Infinity], [0.9999999999999, Infinity],
  [1.5, 1000], [2, -1074], [10, 308], [10, 309], [10, -323], [10, -324],
];
for (const [x, y] of powCases) {
  powRows.push([bitsOf(x), bitsOf(y), bitsOf(Math.pow(x, y))].join("\t"));
}

// ---------------------------------------------------------------------------
// 落盘
// ---------------------------------------------------------------------------

const dir = join(root, "v8-numbers/tests/data");
mkdirSync(dir, { recursive: true });
writeFileSync(join(dir, "oracle-parse.tsv"), "in\top\tradix\tout_bits\n" + parseRows.join("\n") + "\n");
writeFileSync(join(dir, "oracle-fmt.tsv"), "in_bits\top\targ\tout\n" + fmtRows.join("\n") + "\n");
writeFileSync(join(dir, "oracle-rng.tsv"), "op\targ\tout\n" + rngRows.join("\n") + "\n");
writeFileSync(join(dir, "oracle-pow.tsv"), "x_bits\ty_bits\tout_bits\n" + powRows.join("\n") + "\n");
console.log(`parse rows: ${parseRows.length}, fmt rows: ${fmtRows.length}, rng rows: ${rngRows.length}, pow rows: ${powRows.length}`);
