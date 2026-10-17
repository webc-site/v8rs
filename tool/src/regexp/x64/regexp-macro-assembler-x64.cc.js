#!/usr/bin/env bun
// tool/src/regexp/x64/regexp-macro-assembler-x64.cc.js — v8/src/regexp/x64/regexp-macro-assembler-x64.cc
//   → v8-regexp-x64/src/regexp_macro_assembler_x64.cc.rs.skel
// 机械提取: 86 顶层方法定义全集 (Push 3 重载) + 帧偏移消费锚 (kStartIndexOffset 等
//   rbp 寻址) + 帧布局 static_assert 群 (SysV/Win 两态) 与 frame_layout.rs 同源交叉
//   + SIMD 常量 4 组 (kVectorSize 16/kCheckPositionOffset -1) + 标签协议 (success/backtrack);
// 对数: 定义数/断言群数/SIMD 组数; 缺则 throw
import { readFileSync } from "node:fs";
import { join } from "node:path";
import { banner, readV8, root, writeSrc } from "../../../lib.js";

const cc = readV8("src/regexp/x64/regexp-macro-assembler-x64.cc");

let n = 0;
const must = (c, m) => { n++; if (!c) throw new Error(m); };
const cnt = (s, re) => (s.match(re) || []).length;

// ── 方法定义全集 (86 顶层; Push 3 重载) ───────────────────────────────────
const defs = [...cc.matchAll(/\bRegExpMacroAssemblerX64::~?(\w+)\(/g)].map((m) => m[1]);
if (defs.length !== 91) throw new Error(`X64:: 引用应 91 (86 定义 + Push 重载), 实得 ${defs.length}`);
const defLines = cnt(cc, /^[A-Za-z_][A-Za-z0-9_:<>,&* ]* RegExpMacroAssemblerX64::~?[A-Za-z_]+\(/gm);
if (defLines !== 86) throw new Error(`顶层定义应 86, 实得 ${defLines}`);
const pushCount = defs.filter((d) => d === "Push").length;
if (pushCount !== 3) throw new Error(`Push 重载应 3, 实得 ${pushCount}`);
for (const m of ["AdvanceRegister", "Backtrack", "Bind", "CheckCharacter", "LoadCurrentCharacterUnchecked", "GetCode", "Succeed", "Fail", "GoTo", "PushBacktrack", "CheckStackGuardState", "CallCheckStackGuardState"]) {
  must(new RegExp(`\\bRegExpMacroAssemblerX64::${m}\\(`).test(cc), `关键方法 ${m} 缺失`);
}
// Native 层转发 (栈守卫/栈生长)
must(cc.includes("return NativeRegExpMacroAssembler::CheckStackGuardState("), "Native CheckStackGuardState 转发缺失");
must(cc.includes("// Call GrowStack(isolate)."), "GrowStack 调用注释锚缺失");

// ── 帧布局断言群 (cc:1523-1585 两态 static_assert; 与 frame_layout.rs 同源) ──
const asserts = [...cc.matchAll(/static_assert\(([^;]+)\);/g)].map((m) => m[1].trim());
if (asserts.length < 20) throw new Error(`帧断言应 ≥20, 实得 ${asserts.length}`);
for (const a of [
  "FAILURE == 0",
  "kFrameTypeOffset == -1 * kSystemPointerSize",
  "kNumCalleeSaveRegisters == 4",
  "kBackupRsiOffset == -2 * kSystemPointerSize",
  "kNumCalleeSaveRegisters == 2",
  "kBackupRbxOffset == -8 * kSystemPointerSize",
]) must(asserts.some((x) => x.replace(/\s+/g, " ").includes(a.replace(/\s+/g, " "))), `帧断言 ${a} 缺失`);
// 同源交叉: frame_layout.rs 的 SysV/Win 两态数值须满足 cc 断言 (x64 K=8)
const fl = readFileSync(join(root, "v8-regexp-x64", "src", "frame_layout.rs"), "utf8");
const num = (mod, name) => Number(fl.match(new RegExp(`mod ${mod} \\{[\\s\\S]*?${name}: i32 = (-?\\d+);`))?.[1]);
const K = 8;
const sv = { FT: num("sysv", "FRAME_TYPE_OFFSET"), ISI: num("sysv", "INPUT_STRING_OFFSET"), NOI: num("sysv", "NUM_OUTPUT_REGISTERS_OFFSET"), BK: num("sysv", "BACKUP_RBX_OFFSET") };
const wn = { FT: num("win", "FRAME_TYPE_OFFSET"), RSI: num("win", "BACKUP_RSI_OFFSET"), NOI: num("win", "NUM_OUTPUT_REGISTERS_OFFSET") };
if (sv.FT !== -K || wn.FT !== -K) throw new Error(`frame_layout FRAME_TYPE 应 -8, sysv=${sv.FT} win=${wn.FT}`);
if (wn.RSI !== -2 * K) throw new Error(`win BACKUP_RSI 应 -16, 实得 ${wn.RSI}`);
if (sv.BK !== sv.NOI - K) throw new Error(`sysv BACKUP_RBX = NUM_OUTPUT_REGISTERS-8 破坏: ${sv.BK} vs ${sv.NOI - K}`);
if (sv.NOI !== -7 * K || wn.NOI !== 7 * K) throw new Error(`NUM_OUTPUT_REGISTERS: sysv 应 -56, win 应 +56 (调用者栈参数区), 实得 ${sv.NOI}/${wn.NOI}`);
if (num("win", "INPUT_STRING_OFFSET") !== 2 * K) throw new Error("win INPUT_STRING 应 +16 (kFrameAlign)");

// ── SIMD 常量 4 组 (SkipUntil* 系 SIMD 通道) ─────────────────────────────
must(cnt(cc, /static constexpr int kVectorSize = 16;/g) === 5, `kVectorSize=16 应 5 组, 实得 ${cnt(cc, /static constexpr int kVectorSize = 16;/g)}`);
must(cnt(cc, /kCheckPositionOffset = -1;/g) === 4, `kCheckPositionOffset=-1 应 4 组, 实得 ${cnt(cc, /kCheckPositionOffset = -1;/g)}`);
// 第 5 组 SIMD (SkipUntilOneOfMasked3): xmm7..xmm11 现场保存 (rsp 让 6*kVectorSize)
must(cnt(cc, /movupd\(Operand\(rsp, \d \* kVectorSize\)/g) === 5, "xmm 现场保存 5 槽缺失");
must(cc.includes("static constexpr int kCharsPerLoad = 4;"), "kCharsPerLoad=4 缺失");
must(cc.includes("static constexpr int character_count = 4;"), "character_count=4 缺失");

// ── 寄存器/帧槽消费锚 (rbp 寻址面) ────────────────────────────────────────
for (const [a, label] of [
  ["movq(Operand(rbp, kStartIndexOffset),", "prologue 存 startIndex"],
  ["movq(Operand(rbp, kInputStartOffset), kCArgRegs[2]);", "prologue 存 inputStart (C ABI 第 3 参)"],
  ["__ movq(Operand(rbp, kRegExpStackBasePointerOffset), scratch);", "prologue 存正则栈基指针"],
  ["incq(Operand(rbp, kSuccessfulCapturesOffset));", "全局匹配成功计数自增"],
  ["cmpq(rax, Operand(rbp, kStringStartMinusOneOffset));", "串首-1 比对 (CheckPosition 系)"],
]) must(cc.includes(a), `帧槽锚 ${label} 缺失`);
must(cnt(cc, /kCArgRegs\[/g) >= 6, "C ABI 参数寄存器通道 ≥6 缺失");
must(cc.includes("success_label_(),") && cc.includes("backtrack_label_(),"), "ctor 标签初始化缺失");
must(cc.includes("success_label_.Unuse();") && cc.includes("backtrack_label_.Unuse();"), "dtor 标签 Unuse 缺失");
must(cc.includes("kCArgRegs") && cc.includes("kScratchRegister"), "x64 约定寄存器名缺失");

console.log(`x64 .cc: 定义 ${defLines} (Push ×${pushCount}), 帧断言 ${asserts.length}, SIMD 组 4×2, 消费锚 6, frame_layout 交叉 ${Object.keys(sv).length + Object.keys(wn).length} 项, 断言 ${n} 条全过 ✓`);

// ── 生成 Rust ─────────────────────────────────────────────────────────────
const rust = `${banner(
  "src/regexp/x64/regexp-macro-assembler-x64.cc.js",
  "src/regexp/x64/regexp-macro-assembler-x64.cc",
  "x64 宏汇编实现面台账: 定义全集 + 帧消费锚 + SIMD 通道常量 (帧数值同源 frame_layout.rs)",
)}
//! RegExpMacroAssemblerX64 — 原生码生成器 (MacroAssembler 直发)。
//! 帧布局数值不在此重复: 见 frame_layout.rs (本脚本已做 cc static_assert 群 ↔ rs 交叉)。

/// SIMD 向量宽 (SSE, 16 字节 = 8×uc16; SkipUntil* 系 4 处)
pub const K_VECTOR_SIZE: i32 = 16;
/// SIMD 前瞻位置槽偏移 (=-1, kCheckPositionOffset; 4 处)
pub const K_CHECK_POSITION_OFFSET: i32 = -1;
/// 每次 SIMD 加载的字符数 (uc16 ×4)
pub const K_CHARS_PER_LOAD: i32 = 4;
/// 单步推进字符数 (SIMD advance)
pub const CHARACTER_COUNT: i32 = 4;

// ── 定义处置表 (86 顶层; 签名见 cc, 落地按 frame_layout.rs + 本表) ──
// ctor/dtor: success_label_/backtrack_label_ 初始化 + Unuse
// prologue (SysV): kCArgRegs[0..3] → rbp 帧槽; kRegExpStackBasePointer 存 scratch
// 核心通道:
// /// v8: cc AdvanceRegister/ReadCurrentPositionFromRegister/WriteCurrentPositionToRegister
//   → rbp 相对寻址 register_location(kRegisterZeroOffset - reg*8)
// /// v8: cc CheckPosition/CheckAtStart/CheckNotAtStart → kStringStartMinusOneOffset 比对
// /// v8: cc Global 决胜 → kSuccessfulCapturesOffset 自增 + kNumOutputRegisters 减 1
// /// v8: cc CheckStackGuardState/Match/Execute/GrowStack — 外部引用入口 (C ABI kCArgRegs)
// /// v8: cc Push ×3 重载 — backtrack 栈 (rbx) 压 current_position/register/label
// /// v8: cc Succeed → success_label_ 绑定; 失败路径 backtrack_label_/fail_label_
// /// v8: cc static_assert(FAILURE == 0) — NativeRegExpMacroAssembler::Result 哨兵对齐
`;
writeSrc("v8-regexp-x64", "regexp_macro_assembler_x64.cc.rs.skel", rust);
console.log("written: v8-regexp-x64/src/regexp_macro_assembler_x64.cc.rs.skel");
