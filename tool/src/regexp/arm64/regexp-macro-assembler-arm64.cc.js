#!/usr/bin/env bun
// tool/src/regexp/arm64/regexp-macro-assembler-arm64.cc.js — v8/src/regexp/arm64/regexp-macro-assembler-arm64.cc
//   → v8-regexp-arm64/src/regexp_macro_assembler_arm64.cc.rs.skel
// 真解析: cc:24-46 寄存器角色约定表 (x19..x28/w 变体逐条) + 88 顶层方法定义全集
//   + 帧布局 static_assert 群与 frame_layout.rs 同源交叉 + SIMD 常量组
//   + kNumCachedRegisters=16 (x0-x7 成对缓存捕获寄存器) 不变式;
// 对数: 定义/断言/SIMD/角色表计数, 缺则 throw
import { readFileSync } from "node:fs";
import { join } from "node:path";
import { banner, readV8, root, writeSrc } from "../../../lib.js";

const cc = readV8("src/regexp/arm64/regexp-macro-assembler-arm64.cc");
const h = readV8("src/regexp/arm64/regexp-macro-assembler-arm64.h");

let n = 0;
const must = (c, m) => { n++; if (!c) throw new Error(m); };
const cnt = (s, re) => (s.match(re) || []).length;

// ── 寄存器角色约定表 (cc:24-46 注释, 真解析逐条) ──────────────────────────
const roleBlock = cc.match(/This assembler uses the following register assignment convention:([\s\S]*?)The remaining registers are free/);
if (!roleBlock) throw new Error("寄存器约定注释区漂移");
const roles = [...roleBlock[1].matchAll(/- (x\d+\/x\d+|x\d+|w\d+|x\d+\/fp|sp|fp)\s+:\s+([^\n]+(?:\n {2,}[^\n-][^\n]*)*)/g)]
  .map((m) => [m[1], m[2].replace(/\s+/g, " ").trim()]);
if (roles.length !== 13) throw new Error(`寄存器角色应 13, 实得 ${roles.length}: ${roles.map((r) => r[0])}`);
const RV = Object.fromEntries(roles);
if (!RV["x23"].includes("backtrack stack")) throw new Error("x23 回溯栈顶约定漂移");
if (!RV["w22"].includes("Currently loaded character")) throw new Error("w22 当前字符约定漂移");
if (!RV["w21"].includes("Current position")) throw new Error("w21 当前位置约定漂移");
if (!RV["x28"].includes("Output array")) throw new Error("x28 输出数组约定漂移");
// 关键断言: backtrack_stackpointer() 访问器 == x23 (cc:1516)
must(cc.includes("static_assert(backtrack_stackpointer() == x23);"), "x23 访问器断言缺失");

// ── 方法定义全集 (88 顶层 / 93 引用含重载) ───────────────────────────────
const refs = cnt(cc, /\bRegExpMacroAssemblerARM64::~?\w+\(/g);
const defs = cnt(cc, /^[A-Za-z_][A-Za-z0-9_:<>,&* ]* RegExpMacroAssemblerARM64::~?[A-Za-z_]+\(/gm);
if (refs !== 93 || defs !== 88) throw new Error(`定义应 88/引用 93, 实得 ${defs}/${refs}`);
for (const m of ["AdvanceRegister", "Backtrack", "Bind", "CheckCharacter", "LoadCurrentCharacterUnchecked", "GetCode", "Succeed", "Fail", "GoTo", "PushBacktrack", "CallCheckStackGuardState", "CheckStackGuardState"]) {
  must(new RegExp(`\\bRegExpMacroAssemblerARM64::${m}\\(`).test(cc), `关键方法 ${m} 缺失`);
}
must(cc.includes("return NativeRegExpMacroAssembler::CheckStackGuardState("), "Native CheckStackGuardState 转发缺失");

// ── kNumCachedRegisters (h:205 = 16 = x0..x7 成对缓存捕获寄存器) ─────────
const ncr = Number(h.match(/static constexpr int kNumCachedRegisters = (\d+);/)?.[1]);
if (ncr !== 16) throw new Error(`kNumCachedRegisters 应 16, 实得 ${ncr}`);
must(cc.includes("static_assert(kNumCachedRegisters <= 16);") && cc.includes("static_assert((kNumCachedRegisters % 2) == 0);"), "缓存寄存器不变式断言缺失");
must(cnt(cc, /DCHECK_EQ\(kNumCachedRegisters, cached_registers\.Count\(\) \* 2\);/g) === 2, "缓存计数 DCHECK ×2 缺失");
// 小叶帧免栈检查阈值 (cc:1540)
must(cc.includes("static constexpr int kMaxRegistersWithoutStackCheck = 32;"), "kMaxRegistersWithoutStackCheck=32 缺失");
must(cc.includes("kMaxRegistersWithoutStackCheck * kWRegSize <="), "叶帧阈值 static_assert 缺失");

// ── SIMD 常量组: 4×kVectorSize=16 (int) + 1×kQRegSize ────────────────────
must(cnt(cc, /static constexpr int kVectorSize = 16;/g) === 4, "kVectorSize=16 应 4 组");
must(cnt(cc, /static constexpr int kVectorSize = kQRegSize;/g) === 1, "kVectorSize=kQRegSize 应 1 组");
must(cnt(cc, /static constexpr int kCheckPositionOffset = -1;/g) === 4, "kCheckPositionOffset=-1 应 4 组");
must(cc.includes("static constexpr int kWRegPerXReg = kXRegSize / kWRegSize;"), "kWRegPerXReg 缺失");
must(cc.includes("static constexpr int kCharsPerLoad = 4;"), "kCharsPerLoad=4 缺失");

// ── 帧布局交叉: cc 断言群 ↔ frame_layout.rs 数值 ─────────────────────────
const fl = readFileSync(join(root, "v8-regexp-arm64", "src", "frame_layout.rs"), "utf8");
const FV = {};
for (const m of fl.matchAll(/pub const (\w+): i32 = (-?\d+);/g)) FV[m[1]] = Number(m[2]);
for (const [k, want] of [
  ["FRAME_TYPE_OFFSET", -8], ["ISOLATE_OFFSET", -24], ["DIRECT_CALL_OFFSET", -32],
  ["NUM_OUTPUT_REGISTERS_OFFSET", -40], ["INPUT_STRING_OFFSET", -48],
  ["NUM_CALLEE_SAVED_REGISTERS", 10], ["PADDING_AFTER_FRAME_TYPE", 8], ["INITIAL_BUFFER_SIZE", 1024],
]) {
  if (FV[k] !== want) throw new Error(`frame_layout ${k} 应 ${want}, 实得 ${FV[k]}`);
}
// cc 侧对应断言在场
for (const a of [
  "kFrameTypeOffset == kFramePointerOffset - kSystemPointerSize",
  "kIsolateOffset ==",
  "kDirectCallOffset == kIsolateOffset - kSystemPointerSize",
  "kNumOutputRegistersOffset ==",
  "kInputStringOffset ==",
]) must(cc.includes(a), `帧断言 ${a} 缺失`);

// ── 栈结构注释锚 (cc:52-96) ───────────────────────────────────────────────
for (const a of ["fp[104]    Address regexp", "fp[96]     isolate", "fp[16..88] r19-r28", "fp[8]      lr", "fp[0]      fp", "fp[-8]     frame marker"]) {
  must(cc.includes(a), `栈结构注释锚 ${a} 缺失`);
}

console.log(`arm64 .cc: 寄存器角色 ${roles.length}, 定义 ${defs}/引用 ${refs}, 帧断言 5+, SIMD 4+1, frame_layout 交叉 8 项, 断言 ${n} 条全过 ✓`);

// ── 生成 Rust ─────────────────────────────────────────────────────────────
const rust = `${banner(
  "src/regexp/arm64/regexp-macro-assembler-arm64.cc.js",
  "src/regexp/arm64/regexp-macro-assembler-arm64.cc",
  "arm64 宏汇编实现面台账: 寄存器角色表 + 定义全集 + SIMD/叶帧常量 (帧数值同源 frame_layout.rs)",
)}
//! RegExpMacroAssemblerARM64 — 原生码生成器。帧数值见 frame_layout.rs (已交叉)。

/// 寄存器角色约定 (cc:24-46 逐条; w=32 位视图)
pub const REGISTER_ROLES: &[(&str, &str)] = &[
${roles.map(([r, d]) => `  ("${r}", "${d.replace(/"/g, "'")}"),`).join("\n")}
];

/// 成对缓存捕获寄存器数 (x0-x7 = 16 半槽, h:205; ≤16 且 2 整除)
pub const K_NUM_CACHED_REGISTERS: usize = ${ncr};
/// 小叶帧免入口栈检查阈值 (寄存器数 ≤32 即 32×4B=128B 帧)
pub const K_MAX_REGISTERS_WITHOUT_STACK_CHECK: i32 = 32;
/// SIMD 向量宽 (v 寄存器 16B; 4 处 int + 1 处 kQRegSize)
pub const K_VECTOR_SIZE: i32 = 16;
/// SIMD 前瞻位置槽偏移 (4 处)
pub const K_CHECK_POSITION_OFFSET: i32 = -1;
/// 每 SIMD 加载字符数 (uc16 ×4)
pub const K_CHARS_PER_LOAD: i32 = 4;
/// w 寄存器每 x 寄存器数 (8/4)
pub const K_W_REG_PER_X_REG: i32 = 2;

const _: () = assert!(K_NUM_CACHED_REGISTERS <= 16);
const _: () = assert!(K_NUM_CACHED_REGISTERS % 2 == 0);

// ── 定义处置表 (88 顶层; 签名见 cc) ──
// /// v8: cc ctor — num_saved_registers_ 初始化 + kNumCachedRegisters 不变式
// /// v8: cc AdvanceRegister/Read/WriteCurrentPositionFromRegister — 缓存槽
//   (x0-x7 成对, start_reg < kNumCachedRegisters 走缓存, 余走栈) 双通道
// /// v8: cc CheckPosition 系 — w21 字节偏移约定 (非字符偏移!) + w22 预载前提
// /// v8: cc Succeed/Fail/GoTo/Backtrack — backtrack (x23) 压弹协议
// /// v8: cc CallCheckStackGuardState/CheckStackGuardState — Native 层转发
// /// v8: cc 叶帧优化 — kMaxRegistersWithoutStackCheck 阈值下免入口栈检查
// /// v8: cc 大小写折叠 — Orr(wN, wN, 0x20) ASCII 小写化通道
`;
writeSrc("v8-regexp-arm64", "regexp_macro_assembler_arm64.cc.rs.skel", rust);
console.log("written: v8-regexp-arm64/src/regexp_macro_assembler_arm64.cc.rs.skel");
