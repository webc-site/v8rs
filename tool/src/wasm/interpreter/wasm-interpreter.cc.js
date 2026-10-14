#!/usr/bin/env bun
// tool/src/wasm/interpreter/wasm-interpreter.cc.js
// 机械提取 v8/src/wasm/interpreter/wasm-interpreter.cc 的分发表骨架:
// 指令表布局 (小/大 handler + 掩码回绕)、越界计数枚举、符号位常量、宏与函数清单。
// 大 .cc: 只取规则面, 函数体签名骨架 todo!() 就地修订。

import { readV8, writeSrc, banner } from "../../../lib.js";

const ccSrc = readV8("src/wasm/interpreter/wasm-interpreter.cc");
const handlersH = readV8("src/wasm/interpreter/instruction-handlers.h");

function must(src, re, what) {
  const m = src.match(re);
  if (!m) throw new Error(`未找到 ${what}`);
  return m;
}

// —— 1. X-macro 名单 (复用 instruction-handlers.h 的宏链, 对数锚同 clang -E 1690 脚本) ——
function macroBody(src, name) {
  const start = src.indexOf(`#define ${name}`);
  if (start < 0) throw new Error(`未找到宏 ${name}`);
  const ls = src.slice(start).split("\n");
  const out = [];
  for (let i = 1; i < ls.length; i++) {
    const cont = /\\\s*$/.test(ls[i]);
    out.push(ls[i].replace(/\\\s*$/, ""));
    if (!cont) break;
  }
  return out.join("\n");
}
const stripCmt = (s) => s.replace(/\/\/.*$/gm, "");
function paramNames(name) {
  const b = stripCmt(macroBody(handlersH, name));
  const names = [...b.matchAll(/V\(__VA_ARGS__ __VA_OPT__\(, \) (\w+)\)/g)].map((m) => m[1]);
  const total = (b.match(/V\(__VA_ARGS__/g) || []).length;
  if (total !== names.length) throw new Error(`${name} 条目解析不全: ${names.length}/${total}`);
  return names;
}
const regular = paramNames("FOREACH_LOAD_STORE_REGULAR_INSTR_HANDLER");
const superInstr = paramNames("FOREACH_LOAD_STORE_SUPER_INSTR_HANDLER");
// 字面计数 (族宏组成: LOAD_STORE = REGULAR + SUPER, 无变体展开)
if (regular.length !== 74) throw new Error(`REGULAR 计数不符: ${regular.length}, 期望 74`);
if (superInstr.length !== 22) throw new Error(`SUPER 计数不符: ${superInstr.length}, 期望 22`);
// 与 1690 clang -E 锚脚本 (instruction-handlers.h.js) 的分布交叉复核:
// _Mem64 总数 = |REGULAR| + |SUPER| + |ATOMIC|×1 … 变体分布由该脚本保证, 此处只锁字面面
must(handlersH, /FOREACH_LOAD_STORE_REGULAR_INSTR_HANDLER\(V __VA_OPT__\(, \) __VA_ARGS__\) \\\n\s*FOREACH_LOAD_STORE_SUPER_INSTR_HANDLER\(V __VA_OPT__\(, \) __VA_ARGS__\)/, "LOAD_STORE=REGULAR+SUPER 组成");

// —— 2. BoundsCheckedHandlersCounter 枚举 (仅 BOUNDS_CHECKS 开启时存在) ——
must(ccSrc, /enum BoundsCheckedHandlersCounter \{\s*#define ITEM_ENUM_DEFINE\(name\) name##counter,/, "计数枚举头");
must(ccSrc, /FOREACH_LOAD_STORE_INSTR_HANDLER\(ITEM_ENUM_DEFINE\)/, "计数枚举展开");
// FOREACH_LOAD_STORE_INSTR_HANDLER(V) = REGULAR(V) + SUPER(V) 字面展开 → 计数项 96
const loadStoreHandlers = [...regular, ...superInstr];
if (loadStoreHandlers.length !== 96) throw new Error(`load/store 计数项不符: ${loadStoreHandlers.length}`);
must(ccSrc, /kTotalItems/, "kTotalItems 哨兵");

// —— 3. 指令表布局 ——
must(ccSrc, /PWasmOp\* kInstructionTable\[kInstructionTableSize\] = \{/, "kInstructionTable 定义");
must(ccSrc, /CHECK_LE\(kInstructionCount, kInstructionTableSize\);/, "表容量检查");
// MUSTTAIL 尾分派: 大 id 以掩码回绕落表
must(ccSrc, /MUSTTAIL return kInstructionTable\[ReadFnId\(code\) & kInstructionTableMask\]\(/, "MUSTTAIL 分派锚");
// 小段 (压缩字节码) 用 builtin _s 版本填充, 大段用 _l 版本
must(ccSrc, /Builtin::k##name##_s/, "_s builtin 锚");
must(ccSrc, /Builtin::k##name##_l/, "_l builtin 锚");
// 追踪名表
must(ccSrc, /char const\* kInstructionHandlerNames\[kInstructionTableSize\];/, "kInstructionHandlerNames");
// trap 处理器落地页
must(ccSrc, /trap_handler::SetLandingPad\(reinterpret_cast<Address>\(TrapMemOutOfBounds\)\);/, "SetLandingPad");

// —— 4. 符号位常量 ——
must(ccSrc, /constexpr uint32_t kFloat32SignBitMask = uint32_t\{1\} << 31;/, "kFloat32SignBitMask");
must(ccSrc, /constexpr uint64_t kFloat64SignBitMask = uint64_t\{1\} << 63;/, "kFloat64SignBitMask");

// —— 5. unwind 哨兵 ——
must(ccSrc, /static constexpr PWasmOp\* s_unwind_func_addr = HandlersBase::s2s_Unwind;/, "s_unwind_func_addr");

// —— 6. 宏清单 (分派发射宏族) ——
const macroNames = [...ccSrc.matchAll(/^#define\s+(\w+)/gm)].map((m) => m[1]);
if (macroNames.length !== 132) throw new Error(`宏计数不符: ${macroNames.length}, 期望 132`);
for (const required of [
  "EMIT_INSTR_HANDLER", "EMIT_INSTR_HANDLER_WITH_PC", "START_EMIT_INSTR_HANDLER",
  "END_EMIT_INSTR_HANDLER", "EMIT_MEM64_INSTR_HANDLER",
  "EMIT_MULTI_MEM64_INSTR_HANDLER", "EMIT_MEM_MEM_INSTR_HANDLER",
  "DECLARE_MEM64_VARIANTS", "DECLARE_ALL_MEM_VARIANTS", "NextOp",
]) {
  if (!macroNames.includes(required)) throw new Error(`关键宏缺失: ${required}`);
}

// —— 7. 顶层函数定义清单 (col-0 签名; 括号配平后以 `{` 判定, 滤掉前向声明) ——
const fnDefs = [];
{
  const re = /^[A-Za-z_][^;=]*?\b(\w*)::([A-Za-z_~]\w*)\s*\(/gm;
  for (const m of ccSrc.matchAll(re)) {
    // 括号配平找闭括号, 其后首个非空白字符为 '{' 才算定义 (';' 为前向声明)
    let depth = 0, i = ccSrc.indexOf("(", m.index + m[0].length - 1);
    for (; i < ccSrc.length; i++) {
      const c = ccSrc[i];
      if (c === "(") depth++;
      else if (c === ")") { depth--; if (depth === 0) break; }
    }
    const rest = ccSrc.slice(i + 1).match(/^\s*(.)/);
    // ';' → 前向声明, 滤掉; '{' / ':'(初始化列表) / 限定符 → 定义
    if (!rest) throw new Error(`无法判定 ${m[2]} 定义/声明`);
    if (rest[1] !== ";") fnDefs.push([m[1], m[2]]);
  }
}
const fnCount = fnDefs.length;
if (fnCount !== 83) throw new Error(`函数定义计数不符: ${fnCount}, 期望 83`);

console.log(
  `wasm-interpreter.cc: 宏=${macroNames.length} 函数定义=${fnCount} load/store=${loadStoreHandlers.length}`,
);

// —— 8. 生成 Rust ——
let rust = banner(
  "src/wasm/interpreter/wasm-interpreter.cc.js",
  "src/wasm/interpreter/wasm-interpreter.cc",
  "Drumbrake 指令分发表骨架 (大 .cc, 语义面 Developer 就地修订)",
);

rust += `
//! 分发表规则: 表长 4096, MUSTTAIL 尾分派, 大 id 经表掩码回绕落表。

/// 指令表长 (原版 kInstructionTableSize, 2^12)
pub const K_INSTRUCTION_TABLE_SIZE: usize = 4096;

/// 指令表掩码 (原版 kInstructionTableMask)
pub const K_INSTRUCTION_TABLE_MASK: u32 = (K_INSTRUCTION_TABLE_SIZE as u32) - 1;

/// f32 符号位掩码 (原版 kFloat32SignBitMask)
pub const K_FLOAT32_SIGN_BIT_MASK: u32 = 1 << 31;

/// f64 符号位掩码 (原版 kFloat64SignBitMask)
pub const K_FLOAT64_SIGN_BIT_MASK: u64 = 1 << 63;

/// 越界检查 handler 计数枚举 (原版 BoundsCheckedHandlersCounter;
/// 仅 V8_DRUMBRAKE_BOUNDS_CHECKS 开启时编译, 项 = REGULAR+SUPER 字面展开)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u16)]
pub enum BoundsCheckedHandler {
`;

for (const [i, h] of loadStoreHandlers.entries()) {
  rust += `  /// ${h}counter\n  ${h.toUpperCase()}_COUNTER = ${i},\n`;
}
rust += `  /// kTotalItems\n  K_TOTAL_ITEMS = ${loadStoreHandlers.length},
}

impl BoundsCheckedHandler {
  /// 计数项总数 (原版 kTotalItems)
  pub const K_COUNT: u16 = ${loadStoreHandlers.length};
}

/// 指令表槽位 (规则面: 表内既可指向 C++ handler 也可指向 asm builtin)
/// v8: src/wasm/interpreter/wasm-interpreter.cc kInstructionTable
pub enum InstructionTableSlot {
  /// 未初始化 (BOUNDS_CHECKS 关时 load/store 槽先置空, InitInstructionTableOnce 填 builtin)
  Uninit,
  /// C++ 处理函数 (小/大尺寸类别)
  Handler { small: bool },
  /// asm builtin 入口 (_s 压缩版 / _l 大版)
  Builtin { small: bool },
}

/// 指令表 (定长 4096; 大 id 回绕由 K_INSTRUCTION_TABLE_MASK 保证)
/// v8: src/wasm/interpreter/wasm-interpreter.cc kInstructionTable
pub type InstructionTable = [InstructionTableSlot; K_INSTRUCTION_TABLE_SIZE];

/// 表初始化次序规则 (无 BOUNDS_CHECKS 缺省配置):
/// 1) 小段: LOAD_STORE 槽留给 builtin (InitInstructionTableOnce), 其余 NO_BOUNDSCHECK C++ handler;
/// 2) 大段: 同构, builtin 取 _l 版本;
/// 3) InitTrapHandlersOnce 注册 trap_handler 数据, SetLandingPad 指向 TrapMemOutOfBounds。
/// v8: InitInstructionTableOnce / InitTrapHandlersOnce / WasmInterpreter 构造
pub fn init_instruction_table_rule() {
  todo!("就地修订: builtin blob 解包与 trap_handler 注册依赖 Isolate/EmbeddedData")
}

/// 尾分派读取规则 (原版 MUSTTAIL 链):
/// fn_id = ReadFnId(code); slot = fn_id & kInstructionTableMask; MUSTTAIL 跳转。
/// v8: src/wasm/interpreter/wasm-interpreter.cc NextOp 宏族
pub fn tail_dispatch_rule() {
  todo!("就地修订: 与 ReadFnId 编码 (尺寸位 + id) 联动")
}

/// 顶层函数定义清单 (col-0 签名, 按源序; Class::member 对)
/// v8: src/wasm/interpreter/wasm-interpreter.cc
pub const TOP_LEVEL_FN_DEFS: &[(&str, &str)] = &[
`;

for (const [cls, name] of fnDefs) rust += `  ("${cls}", "${name}"),\n`;
rust += `];

/// 分派发射宏清单 (原版 #define 全量)
pub const DISPATCH_MACROS: &[&str] = &[
`;
for (const n of macroNames) rust += `  "${n}",\n`;
rust += `];

// —— 提取不变式断言 ——
const _: () = {
  assert!(K_INSTRUCTION_TABLE_MASK == 4095);
  assert!(BoundsCheckedHandler::K_COUNT as usize == ${loadStoreHandlers.length});
  assert!(DISPATCH_MACROS.len() == ${macroNames.length});
  assert!(TOP_LEVEL_FN_DEFS.len() == ${fnCount});
  assert!(K_FLOAT32_SIGN_BIT_MASK == 0x8000_0000);
  assert!(K_FLOAT64_SIGN_BIT_MASK == 0x8000_0000_0000_0000);
};
`;

const p = writeSrc("v8-wasm-interp", "wasm_interpreter_dispatch.rs.skel", rust);
console.log(`written ${p}`);
