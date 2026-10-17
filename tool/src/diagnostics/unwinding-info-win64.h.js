#!/usr/bin/env bun
// tool/src/diagnostics/unwinding-info-win64.h.js
// 机械提取 v8/src/diagnostics/unwinding-info-win64.{h,cc} 的常量/枚举/位宽规则面:
//   Windows X64 分支 (kPushRbp* / kRbpPrefix*) 与 Windows ARM64 分支
//   (kMaxFunctionLength / UnwindOp8Bit / shift-mask 常量族), 按双目标展开登记。
// 语义面: XdataEncoder / GetUnwindInfoForBuiltinFunctions 等 ABI 编码实现登记跳过。
// 产物: v8-diagnostics/src/unwinding_info_win64.rs

import { readV8, writeSrc, banner } from "../../lib.js";

const H_REL = "src/diagnostics/unwinding-info-win64.h";
const CC_REL = "src/diagnostics/unwinding-info-win64.cc";
const h = readV8(H_REL);
const cc = readV8(CC_REL);

function needConst(src, name, expect) {
  const m = src.match(new RegExp(`(?:static const|constexpr)[^(]*\\b${name}\\s*=\\s*([^;]+);`));
  if (!m) throw new Error(`未找到常量 ${name}`);
  return m[1].trim();
}
function evalExpr(src, expr, seen = new Set()) {
  const e = expr.replace(/\s+/g, "");
  if (/^-?(0x[0-9a-fA-F]+|\d+)$/.test(e)) return Number(e);
  if (seen.has(e)) throw new Error(`常量循环引用: ${expr}`);
  seen.add(e);
  // 括号内先求
  let out = e.replace(/\(([^()]+)\)/g, (_, inner) => String(evalExpr(src, inner, new Set(seen))));
  // 常量名替换 (限一轮, 嵌套由递归处理)
  for (let i = 0; i < 8 && /[A-Za-z_]/.test(out); i++) {
    const before = out;
    out = out.replace(/[A-Za-z_]\w*/g, (name) => {
      if (/^-?(0x[0-9a-fA-F]+|\d+)$/.test(out)) return out;
      const m = src.match(new RegExp(`(?:static const(?:expr)?[^({;]*|constexpr int)\\s*${name}\\s*=\\s*([^;]+);`));
      if (!m) throw new Error(`常量未定义: ${name} (于 ${expr})`);
      return `(${evalExpr(src, m[1].trim(), new Set(seen))})`;
    });
    if (out === before) break;
  }
  if (/^[()\-0-9xXa-fA-F+*|<>]+$/.test(out)) {
    // 仅算术表达式, 用 Function 求值
    const js = out.replace(/\|\|/g, "||");
    // eslint 禁 eval 语义之外的编造 — 此处只做整数字面量算术
    const val = new Function(`"use strict"; return (${js});`)();
    if (typeof val !== "number" || !Number.isFinite(val)) throw new Error(`表达式求值失败: ${expr} → ${out}`);
    return val;
  }
  throw new Error(`表达式含未识别记号: ${expr} → ${out}`);
}

// —— 公共 (双平台) ——
const osPageSize = evalExpr(h, needConst(h, "kOSPageSize"));
const defaultRuntimeFunctionCount = evalExpr(h, needConst(h, "kDefaultRuntimeFunctionCount"));

// —— V8_OS_WIN_X64 分支 ——
const x64 = {
  pushRbp: evalExpr(h, needConst(h, "kPushRbpInstructionLength")),
  movRbpRsp: evalExpr(h, needConst(h, "kMovRbpRspInstructionLength")),
  rbpPrefixCodes: evalExpr(h, needConst(h, "kRbpPrefixCodes")),
  rbpPrefixLength: evalExpr(h, needConst(h, "kRbpPrefixLength")),
};

// —— V8_OS_WIN_ARM64 分支 ——
const maxFunctionLength = evalExpr(h, needConst(h, "kMaxFunctionLength"));
const unwindOpM = cc.match(/enum UnwindOp8Bit\s*\{([^}]+)\};/);
if (!unwindOpM) throw new Error("未找到 UnwindOp8Bit 枚举");
const unwindOps = unwindOpM[1]
  .split(",")
  .map((s) => s.trim())
  .filter(Boolean)
  .map((s) => {
    const m = s.match(/(\w+)\s*=\s*(0x[0-9a-fA-F]+|\d+)/);
    if (!m) throw new Error(`UnwindOp8Bit 条目异常: ${s}`);
    return { name: m[1], value: Number(m[2]) };
  });
if (unwindOps.length !== 7) throw new Error(`UnwindOp8Bit 条数不符: ${unwindOps.length} != 7`);

// ARM64 分支 shift/mask 常量族 (kDefaultNumberOfUnwindCodeWords .. kFunctionLengthMask 等)
const arm64Consts = [
  ["kNumberOfUnwindCodes", 90],
  ["kMaxExceptionThunkSize", 91],
  ["kDefaultNumberOfUnwindCodeWords", 222],
].map(([name]) => ({ name, value: evalExpr(cc, needConst(cc, name)) }));
const functionLengthShift = evalExpr(cc, needConst(cc, "kFunctionLengthShiftSize"));
const functionLengthMaskExpr = needConst(cc, "kFunctionLengthMask");
const functionLengthMask = evalExpr(cc, functionLengthMaskExpr);
const allocStackShift = evalExpr(cc, needConst(cc, "kAllocStackShiftSize"));
const allocStackMask = evalExpr(cc, needConst(cc, "kAllocStackShiftMask"));
if (functionLengthMask !== (1 << functionLengthShift) - 1) throw new Error("kFunctionLengthMask 与 ShiftSize 不自洽");
if (allocStackMask !== (1 << allocStackShift) - 1) throw new Error("kAllocStackShiftMask 与 ShiftSize 不自洽");

let rust = banner("src/diagnostics/unwinding-info-win64.h.js", "src/diagnostics/unwinding-info-win64.{h,cc}", "Win64/WinARM64 展开信息常量面");
rust += `
// Windows 专用模块 (V8_OS_WIN_X64 / V8_OS_WIN_ARM64 双分支, 均 x64/arm64 目标内),
// 接入 crate 时建议整体 #[cfg(windows)]。语义面 (XdataEncoder/ABI 编码) 登记跳过。

/// 原 CRASH_HANDLER_FUNCTION_NAME 宏
pub const CRASH_HANDLER_FUNCTION_NAME: &str = "CrashForExceptionInNonABICompliantCodeRange";

/// OS 页大小 (原 kOSPageSize)
pub const OS_PAGE_SIZE: usize = ${osPageSize};

/// RUNTIME_FUNCTION 默认数 (原 kDefaultRuntimeFunctionCount; X64 一条覆盖 4GB)
pub const DEFAULT_RUNTIME_FUNCTION_COUNT: u32 = ${defaultRuntimeFunctionCount};

// —— V8_OS_WIN_X64 分支 ——

/// 原 kPushRbpInstructionLength
pub const X64_PUSH_RBP_INSTRUCTION_LENGTH: usize = ${x64.pushRbp};
/// 原 kMovRbpRspInstructionLength
pub const X64_MOV_RBP_RSP_INSTRUCTION_LENGTH: usize = ${x64.movRbpRsp};
/// 原 kRbpPrefixCodes
pub const X64_RBP_PREFIX_CODES: usize = ${x64.rbpPrefixCodes};
/// 原 kRbpPrefixLength = push + mov
pub const X64_RBP_PREFIX_LENGTH: usize = ${x64.rbpPrefixLength};

// —— V8_OS_WIN_ARM64 分支 ——

/// 18 位无符号编码函数长度 (除 2 个恒 0 LSB), 原 kMaxFunctionLength
pub const ARM64_MAX_FUNCTION_LENGTH: usize = ${maxFunctionLength};

/// ARM64 unwind 8 位操作码 (原 enum UnwindOp8Bit)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum UnwindOp8Bit {
${unwindOps.map((o) => `  /// 原 \`${o.name}\`\n  ${o.name.replace(/^Op/, "")} = 0x${o.value.toString(16).toUpperCase()},`).join("\n")}
}

impl UnwindOp8Bit {
  /// 原 Combine8BitUnwindCodes: 四个 8 位操作码打包为一个 UNWIND_CODE 字
  #[inline]
  #[must_use]
  pub const fn combine(c0: Self, c1: Self, c2: Self, c3: Self) -> u32 {
    c0 as u32 | ((c1 as u32) << 8) | ((c2 as u32) << 16) | ((c3 as u32) << 24)
  }
}

/// 原 UNWIND_CODE typedef
pub type UnwindCode = u32;

${arm64Consts.map((c) => `/// 原 ${c.name}\npub const ${c.name.replace(/^k/, "ARM64_").toUpperCase()}: usize = ${c.value};`).join("\n")}
/// 原 kFunctionLengthShiftSize
pub const ARM64_FUNCTION_LENGTH_SHIFT_SIZE: usize = ${functionLengthShift};
/// 原 kFunctionLengthMask
pub const ARM64_FUNCTION_LENGTH_MASK: usize = 0x${functionLengthMask.toString(16).toUpperCase()};
/// 原 kAllocStackShiftSize
pub const ARM64_ALLOC_STACK_SHIFT_SIZE: usize = ${allocStackShift};
/// 原 kAllocStackShiftMask
pub const ARM64_ALLOC_STACK_SHIFT_MASK: usize = 0x${allocStackMask.toString(16).toUpperCase()};

// —— 提取不变式断言 ——
const _: () = {
  assert!(OS_PAGE_SIZE == 4096);
  assert!(DEFAULT_RUNTIME_FUNCTION_COUNT == 1);
  assert!(X64_RBP_PREFIX_LENGTH == ${x64.rbpPrefixLength});
  assert!(ARM64_MAX_FUNCTION_LENGTH == ${maxFunctionLength});
  assert!(ARM64_FUNCTION_LENGTH_MASK == (1 << ARM64_FUNCTION_LENGTH_SHIFT_SIZE) - 1);
  assert!(ARM64_ALLOC_STACK_MASK == (1 << ARM64_ALLOC_STACK_SHIFT_SIZE) - 1);
  assert!(UnwindOp8Bit::End as u32 == 0xE4);
  assert!(UnwindOp8Bit::combine(UnwindOp8Bit::Nop, UnwindOp8Bit::Nop, UnwindOp8Bit::Nop, UnwindOp8Bit::Nop) == 0xE3E3E3E3);
};
`;

const p = writeSrc("v8-diagnostics", "unwinding_info_win64.rs", rust);
console.log(`written ${p} (UnwindOp8Bit ${unwindOps.length} 条, x64 ${4} 常量, arm64 ${arm64Consts.length + 4} 常量)`);
