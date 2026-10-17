#!/usr/bin/env bun
// tool/src/diagnostics/objects-printer.js — 从 v8/src/diagnostics/objects-printer.cc 提取零耦合字符串工具面:
// PrintDouble 四分支格式 (含 kMin/kMaxSafeInteger 边界, 取自 common/globals.h),
// JSIteratorHelperState / JSIteratorZipHelperMode 枚举→字符串映射 (逐项对数)。
// HeapObjectPrint / *Verify 重耦合 Tagged 面对象打印/校验主体推迟 (L3 objects + execution)。
// 生成 v8-diagnostics/src/objects_print_data.rs; 行为面见 objects_print.rs
// 复跑幂等: 纯文本扫描 + 源码序输出, 无时间戳。
import { readV8, writeSrc, banner } from "../../lib.js";

const printer = readV8("src/diagnostics/objects-printer.cc");
const globals = readV8("src/common/globals.h");
const conversions = readV8("src/numbers/conversions-inl.h");

let n = 0;
const die = (m) => { throw new Error(`[objects-printer.js] ${m}`); };
const need = (src, s, where) => { if (!src.includes(s)) die(`${where} 缺针 ${s}`); n++; };

// ── PrintDouble 四分支锚定 ──
need(printer, "void PrintDouble(std::ostream& os, double val) {", "objects-printer.cc");
need(printer, 'if (i::IsMinusZero(val)) {\n    os << "-0.0";', "objects-printer.cc");
need(printer, "val == DoubleToInteger(val) && val >= kMinSafeInteger &&", "objects-printer.cc");
need(printer, 'os << i << ".0";', "objects-printer.cc");
need(printer, 'os << val << " (0x" << std::hex << base::double_to_uint64(val) << std::dec', "objects-printer.cc");
// DoubleToInteger 语义锚定 (ToIntegerOrInfinity)
need(conversions, "if (std::isnan(x) || x == 0.0) return 0;", "conversions-inl.h");
need(conversions, "if (!std::isfinite(x)) return x;", "conversions-inl.h");
need(conversions, "return ((x > 0) ? std::floor(x) : std::ceil(x)) + 0.0;", "conversions-inl.h");

// ── 安全整数边界 ──
const safeRaw = globals.match(/constexpr uint64_t kMaxSafeIntegerUint64 = \(uint64_t\{1\} << (\d+)\) - 1;/);
if (!safeRaw) die("kMaxSafeIntegerUint64 未找到");
const SAFE_BITS = parseInt(safeRaw[1]);
n++;
if (SAFE_BITS !== 53) die(`kMaxSafeIntegerUint64 位移 ${SAFE_BITS} != 53`);

// ── 枚举→字符串映射逐项对数 ──
function parseMap(enumName) {
  const re = new RegExp(`const char\\* ${enumName}ToString\\(${enumName} \\w+\\) \\{([\\s\\S]*?)\\n\\}`);
  const m = printer.match(re);
  if (!m) die(`${enumName}ToString 未找到`);
  const pairs = [...m[1].matchAll(/case .*::(k\w+):\s*\n\s*return "([^"]+)";/g)].map((x) => [x[1], x[2]]);
  return pairs;
}
const helper = parseMap("JSIteratorHelperState");
if (helper.length !== 4) die(`JSIteratorHelperStateToString 项数 ${helper.length} != 4`);
const zip = parseMap("JSIteratorZipHelperMode");
if (zip.length !== 3) die(`JSIteratorZipHelperModeToString 项数 ${zip.length} != 3`);
n += 2;
need(printer, 'constexpr char kUnavailableString[] = "unavailable";', "objects-printer.cc");

const pascal = (k) => k.slice(1)[0] + k.slice(2);

const out = [];
out.push(banner("src/diagnostics/objects-printer.js", "src/diagnostics/objects-printer.cc;src/common/globals.h;src/numbers/conversions-inl.h"));
out.push(`//! 对象打印零耦合字符串工具面 (基线生成; 行为面见 objects_print.rs)。`);
out.push(``);
out.push(`// ── PrintDouble 边界/字面 ──`);
out.push(`pub const MAX_SAFE_INTEGER_BITS: u32 = ${SAFE_BITS};`);
out.push(`pub const MAX_SAFE_INTEGER_F64: f64 = ((1u64 << MAX_SAFE_INTEGER_BITS) - 1) as f64;`);
out.push(`pub const MIN_SAFE_INTEGER_F64: f64 = -MAX_SAFE_INTEGER_F64;`);
out.push(`pub const MINUS_ZERO_STR: &str = "-0.0";`);
out.push(`pub const NAN_SUFFIX: &str = " (0x";`);
out.push(``);
// 名字表: 单行超 rustfmt max_width(100) 则逐元素换行, 与 Rust 面格式对齐
const nameTable = (ty, name, vals) => {
  const oneLine = `pub const ${name}: ${ty} = [${vals.map((v) => `"${v}"`).join(", ")}];`;
  if (oneLine.length <= 100) { out.push(oneLine); return; }
  out.push(`pub const ${name}: ${ty} = [`);
  for (const v of vals) out.push(`  "${v}",`);
  out.push(`];`);
};
out.push(`// ── JSIteratorHelperState → 名 (${helper.length} 项) ──`);
out.push(`#[derive(Debug, Clone, Copy, PartialEq, Eq)]`);
out.push(`pub enum JsIteratorHelperState {`);
for (const [k] of helper) out.push(`  ${pascal(k)},`);
out.push(`}`);
nameTable(`[&str; ${helper.length}]`, "ITER_HELPER_STATE_NAMES", helper.map(([, v]) => v));
out.push(``);
out.push(`// ── JSIteratorZipHelperMode → 名 (${zip.length} 项) ──`);
out.push(`#[derive(Debug, Clone, Copy, PartialEq, Eq)]`);
out.push(`pub enum JsIteratorZipHelperMode {`);
for (const [k] of zip) out.push(`  ${pascal(k)},`);
out.push(`}`);
nameTable(`[&str; ${zip.length}]`, "ITER_ZIP_HELPER_MODE_NAMES", zip.map(([, v]) => v));

writeSrc("v8-diagnostics", "objects_print_data.rs", out.join("\n").trimEnd() + "\n");
console.log(`OK objects-printer.js: 对数 ${n} (PrintDouble 针 6 + 边界 1 + 映射 2 组 + 未用串 1) → v8-diagnostics/src/objects_print_data.rs`);
console.log(`  helper=${JSON.stringify(helper.map(([, v]) => v))}`);
console.log(`  zip=${JSON.stringify(zip.map(([, v]) => v))}`);
