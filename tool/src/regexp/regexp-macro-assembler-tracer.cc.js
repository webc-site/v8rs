#!/usr/bin/env bun
// tool/src/regexp/regexp-macro-assembler-tracer.cc.js — v8/src/regexp/regexp-macro-assembler-tracer.{h,cc}
//   → v8-regexp-compiler/src/macro_assembler_tracer_generated.rs
// 机械提取: h 覆盖面方法签名清单 + cc 逐方法日志格式串 (PrintF 首参量,
//   相邻字符串字面量拼接) + label[%08x] 约定 + PrintablePrinter 可打印规则 +
//   位图/nibble 表打印布局;
// 对数断言: cc 定义数 == h 声明 override 数, 格式串计数, kRows 16 锚。
import { banner, readV8, writeSrc } from "../../lib.js";

const h = readV8("src/regexp/regexp-macro-assembler-tracer.h");
const cc = readV8("src/regexp/regexp-macro-assembler-tracer.cc");

// ── h 声明面: override 方法 (带分号结尾的声明 + 内联体) ──────────────────
const decls = [...h.matchAll(/^  (?:[\w:<>&*,\s]+?)\s(\w+)\([^;{]*\)\s*(?:const\s*)?(?:override)(?:;|\s*\{)/gm)].map((r) => r[1]);
const OVERRIDES = new Set(decls);
if (OVERRIDES.has("RecordComment") === false) throw new Error("内联 override RecordComment 缺失");
if (OVERRIDES.size < 55) throw new Error(`h override 声明应 ≥55, 实数 ${OVERRIDES.size}`);

// ── cc 定义面: 逐方法体提取 + PrintF 格式串 ─────────────────────────────
const heads = [...cc.matchAll(/RegExpMacroAssemblerTracer::([~\w]+)\(/g)];
if (heads.length < 55) throw new Error(`cc 定义应 ≥55, 实数 ${heads.length}`);
const methods = heads.map((m, i) => {
  const end = i + 1 < heads.length ? heads[i + 1].index : cc.length;
  return { name: m[1].replace(/^~/, "").replace(/^RegExpMacroAssemblerTracer$/, "Constructor"), body: cc.slice(m.index, end) };
});
// h 声明的 override 都有 cc 定义 (内联直通的在 h 内实现, 不在 cc)
const defined = new Set(methods.map((m) => m.name));
const missing = [...OVERRIDES].filter((n) => !defined.has(n));
// 内联直通白名单 (h 内直接转发, cc 无定义)
const INLINE_PASSTHROUGH = [
  "SkipUntilBitInTableUseSimd", "SkipUntilCharAndUseSimd", "SkipUntilOneOfMasked3UseSimd",
  "RecordComment", "masm", "set_fail_label", "prologue_pushes_fail_label",
];
for (const n of missing) {
  if (!INLINE_PASSTHROUGH.includes(n)) throw new Error(`cc 缺 ${n} 定义`);
}

// ── PrintF 格式串 (相邻字面量拼接, 解 C 转义) ───────────────────────────
const cUnescape = (s) => s.replace(/\\(.)/g, (_, c) => (c === "n" ? "\n" : c === "t" ? "\t" : c));
const formats = new Map();
for (const m of methods) {
  const pf = m.body.match(/PrintF\(\s*((?:"(?:[^"\\]|\\.)*"\s*)+)/);
  if (pf) {
    const joined = [...pf[1].matchAll(/"((?:[^"\\]|\\.)*)"/g)].map((r) => r[1]).join("");
    formats.set(m.name, cUnescape(joined));
  }
}
if (formats.size < 40) throw new Error(`PrintF 格式串应 ≥40, 实数 ${formats.size}`);
// label 约定
let labelCount = 0;
for (const f of formats.values()) labelCount += (f.match(/label\[%08x\]/g) || []).length;
if (labelCount < 20) throw new Error(`label[%08x] 出现应 ≥20, 实数 ${labelCount}`);
// 人工锚: 5 条
const EXPECT_FMT = {
  Bind: "label[%08x]: (Bind)\n",
  GoTo: " GoTo(label[%08x]);\n\n",
  Backtrack: " Backtrack();\n",
  PushRegister: " PushRegister(register=%d, %s);\n",
  GetCode: " GetCode('%s', '%s');\n",
};
for (const [name, want] of Object.entries(EXPECT_FMT)) {
  if (formats.get(name) !== want) throw new Error(`${name} 格式漂移: ${JSON.stringify(formats.get(name))}`);
}
// Succeed 条件后缀
if (!methods.find((m) => m.name === "Succeed").body.includes('" [restart for global match]"')) {
  throw new Error("Succeed 重启后缀漂移");
}
// Fail 无换行特例
if (formats.get("Fail") !== " Fail();") throw new Error("Fail 无换行特例漂移");

// ── PrintablePrinter 规则 (cc:165-184) ───────────────────────────────────
if (!cc.includes("if (character_ >= ' ' && character_ <= '~')")) throw new Error("可打印区间规则漂移");
if (!cc.includes("buffer_[0] = '(';")) throw new Error("括号包裹规则漂移");
// 推进打印 "check stack limit" 二态
if (!cc.includes('"check stack limit"')) throw new Error("栈检查文案漂移");
// 前后向二态
if (!cc.includes('read_backward ? "backward" : "forward"')) throw new Error("前后向文案漂移");
// unicode 二态
if (!cc.includes('unicode ? "unicode" : "non-unicode"')) throw new Error("unicode 文案漂移");

// ── 位图/nibble 表布局 (cc:356-384) ─────────────────────────────────────
if (!cc.includes("static_assert(RegExpMacroAssembler::kTableSize == 128);")) throw new Error("kTableSize 128 断言漂移");
if (!cc.includes("static constexpr int kRows = 16;")) throw new Error("kRows 16 漂移");
if (!cc.includes("static_assert(kRows * kBitsPerByte == RegExpMacroAssembler::kTableSize);")) {
  throw new Error("kRows×8==128 断言漂移");
}
if (!cc.includes('"  +----------------\\n"')) throw new Error("nibble 表分隔线漂移");
if (!cc.includes('"\\n--+----------------"')) throw new Error("nibble 表表头线漂移");
// 位图行折行规则: 每 32 位折行
if (!cc.includes("if (i % 32 == 31 && i != kTableMask)")) throw new Error("位图折行规则漂移");
// 构造器打印实现名
if (!cc.includes("ImplementationToString(assembler_->Implementation())")) throw new Error("构造器实现名打印漂移");

console.log(
  `regexp-macro-assembler-tracer: h override ${OVERRIDES.size}, cc 定义 ${methods.length}, ` +
    `PrintF 格式串 ${formats.size}, label[%08x] ${labelCount} ✓`,
);

// ── 生成 Rust ──────────────────────────────────────────────────────────────
const fmtEntries = [...formats.entries()];
const rustEscape = (s) => s.replace(/\\/g, "\\\\").replace(/"/g, '\\"').replace(/\n/g, "\\n").replace(/\t/g, "\\t");
const body = `${banner(
  "src/regexp/regexp-macro-assembler-tracer.cc.js",
  "src/regexp/regexp-macro-assembler-tracer.{h,cc}",
  "宏汇编装饰器日志格式串表 (PrintF 格式 1:1); 装饰器实现面待 Developer 就地修订",
)}
/// 逐方法日志格式串 (cc PrintF 首参量, 相邻字面量已拼接; %08x/%d/%s 为插值位)
pub const TRACER_FORMATS: [(&str, &str); ${fmtEntries.length}] = [
${fmtEntries.map(([n, f]) => `  ("${n}", "${rustEscape(f)}"),`).join("\n")}
];

// ── 约定常量 ──
/// 标签打印约定: label[指针低 32 位, 8 位十六进制] (cc:33-35 LabelToInt)
pub const K_LABEL_FORMAT: &str = "label[%08x]";
/// 栈检查文案 (PushRegister 的 StackCheckFlag::kCheckStackLimit 分支)
pub const K_CHECK_STACK_LIMIT_TEXT: &str = "check stack limit";
/// 读方向二态 (CheckNotBackReference 系)
pub const K_READ_BACKWARD_TEXT: &str = "backward";
pub const K_READ_FORWARD_TEXT: &str = "forward";
/// unicode 二态 (CheckNotBackReferenceIgnoreCase)
pub const K_UNICODE_TEXT: &str = "unicode";
pub const K_NON_UNICODE_TEXT: &str = "non-unicode";
/// Succeed 全局匹配重启后缀
pub const K_RESTART_SUFFIX: &str = " [restart for global match]";
/// 可打印字符显示区间 (' '..'~', 括号包裹) (cc:170-175)
pub const K_PRINTABLE_MIN: u16 = 0x20;
pub const K_PRINTABLE_MAX: u16 = 0x7e;
/// 可打印包裹括号
pub const K_PRINTABLE_WRAP: [char; 2] = ['(', ')'];

// ── 位图/nibble 表布局 (PrintTables, cc:358-384) ──
/// 位图总尺寸 (kTableSize 断言 128)
pub const K_TABLE_SIZE: usize = 128;
/// nibble 表行数 (kRows = 16 = 128/8)
pub const K_TABLE_ROWS: usize = 16;
/// 每行位数 (i % 32 == 31 折行, 4 行 × 32)
pub const K_BITS_PER_ROW: usize = 32;
/// nibble 表头分隔线
pub const K_NIBBLE_HEADER_RULE: &str = "\\n--+----------------";
/// nibble 表体分隔线
pub const K_NIBBLE_BODY_RULE: &str = "  +----------------\\n";
/// 位图显示字符 (置位/清位)
pub const K_BITMAP_SET_CHAR: char = 'X';
pub const K_BITMAP_CLEAR_CHAR: char = '.';

const _: () = assert!(TRACER_FORMATS.len() == ${fmtEntries.length});
const _: () = assert!(K_TABLE_ROWS * 8 == K_TABLE_SIZE);
`;

writeSrc("v8-regexp-compiler", "macro_assembler_tracer_generated.rs", body);
console.log("written: v8-regexp-compiler/src/macro_assembler_tracer_generated.rs");
