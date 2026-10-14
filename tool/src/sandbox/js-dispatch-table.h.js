#!/usr/bin/env bun
// tool/src/sandbox/js-dispatch-table.h.js
// 机械提取 v8/src/sandbox/js-dispatch-table-inl.h (+ globals.h 布局常量锚) —
// JSDispatchTable 内联访问面: 布局常量双分支 + 访问器骨架。
// 注: 基表 external-entity-table.js 已覆盖分段表基类; 本脚本只覆盖 -inl 残余。
// 产物: v8-sandbox/src/js_dispatch_table.rs

import { readV8, writeSrc, banner } from "../../lib.js";

const INL_REL = "src/sandbox/js-dispatch-table-inl.h";
const inl = readV8(INL_REL);
const globals = readV8("src/common/globals.h"); // 布局常量来源

// globals.h 布局常量双分支 (x64-LA57 vs 其他)
const la57 = globals.match(
  /#if V8_TARGET_ARCH_X64 && V8_SUPPORT_LA57\s*\nconstexpr int kJSDispatchTableEntrySize = (\d+);\s*\nconstexpr int kJSDispatchTableEntrySizeLog2 = (\d+);\s*\n#else\s*\nconstexpr int kJSDispatchTableEntrySize = (\d+);\s*\nconstexpr int kJSDispatchTableEntrySizeLog2 = (\d+);/,
);
if (!la57) throw new Error("globals.h 未找到 kJSDispatchTableEntrySize 双分支");
if (la57[1] !== "32" || la57[3] !== "16") throw new Error("EntrySize 分支值与预期不符");
const nullHandleM = globals.match(/constexpr JSDispatchHandle kNullJSDispatchHandle\(0\);/);
if (!nullHandleM) throw new Error("未找到 kNullJSDispatchHandle");

// inl 访问器面
const accessors = [...inl.matchAll(/^(?:template <[\s\S]*?>\s*\n)?[A-Za-z].*?\b(\w+)\(JSDispatchHandle/gm)].map((m) => m[1]);
if (!inl.includes("void JSDispatchEntry::MakeJSDispatchEntry(")) throw new Error("inl 缺 MakeJSDispatchEntry 锚");
const entryMethods = [...inl.matchAll(/JSDispatchEntry::(\w+)\(/g)].map((m) => m[1]);
const encM = [...inl.matchAll(/k(?:CodeObject|Entrypoint|ParameterCount)Offsets?|encoded_word_/g)].length;
if (encM < 3) throw new Error("inl 缺编码布局锚");

let rust = banner("src/sandbox/js-dispatch-table.h.js", "src/sandbox/js-dispatch-table-inl.h", "JSDispatchTable 内联访问骨架");
rust += `
// 布局常量实值来自 src/common/globals.h (双目标分支); 表基类由
// tool/src/sandbox/external-entity-table.js 覆盖。

/// 原 JSDispatchHandle = StrongAlias<uint32_t>
pub type JSDispatchHandle = u32;

/// 原 kNullJSDispatchHandle
pub const NULL_JS_DISPATCH_HANDLE: JSDispatchHandle = 0;

/// 表项大小双分支:
/// - x64 + LA57 (五级页表): ${la57[1]} 字节 (log2 = ${la57[2]})
/// - 其他目标 (arm64 / x64 非 LA57): ${la57[3]} 字节 (log2 = ${la57[4]})
pub mod layout {
  pub const ENTRY_SIZE_LA57: usize = ${la57[1]};
  pub const ENTRY_SIZE_LOG2_LA57: u32 = ${la57[2]};
  pub const ENTRY_SIZE: usize = ${la57[3]};
  pub const ENTRY_SIZE_LOG2: u32 = ${la57[4]};
}

/// 原 JSDispatchTable 访问器面 (inl 侧 ${accessors.length} 处句柄入参方法族):
${[...new Set(accessors)].map((s) => `// - ${s}`).join("\n")}

/// 原 JSDispatchEntry 方法注册表 (inl 侧 ${entryMethods.length} 处定义):
${[...new Set(entryMethods)].map((s) => `// - ${s}`).join("\n")}

/// 原 MakeJSDispatchEntry 载荷编码双分支:
/// - x64+LA57: payload = parameter_count (object 存独立 code_object_ 字)
/// - 其他目标: payload = ((object - kObjectPointerOffset) << kObjectPointerShift)
///             | (parameter_count & kParameterCountMask)
/// 语义实现登记跳过。
pub struct JSDispatchEntry;

// TODO(dev): 按 encoded_word_ 布局补 GetCodePointer/SetCodeAndEntrypointPointer。

// —— 提取不变式断言 ——
const _: () = {
  assert!(layout::ENTRY_SIZE_LA57 == 32);
  assert!(layout::ENTRY_SIZE == 16);
  assert!(layout::ENTRY_SIZE_LOG2_LA57 == 5);
  assert!(layout::ENTRY_SIZE_LOG2 == 4);
};
`;

const p = writeSrc("v8-sandbox", "js_dispatch_table.rs", rust);
console.log(`written ${p} (访问器 ${[...new Set(accessors)].length}, Entry 方法 ${[...new Set(entryMethods)].length}, 布局双分支 2+2)`);
