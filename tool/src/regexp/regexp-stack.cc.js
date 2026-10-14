#!/usr/bin/env bun
// tool/src/regexp/regexp-stack.cc.js — v8/src/regexp/regexp-stack.cc → v8-regexp-exec/src/regexp_stack.rs (实现族)
// 规则: 13 条方法定义全集 + EnsureCapacity/Archive/动态栈分配释放语义锚逐条对数
//   + 沙箱硬件分支 (#ifdef V8_ENABLE_SANDBOX_HARDWARE_SUPPORT) 计数登记 — 默认配置全部排除落地
// 产物: v8-regexp-exec/src/regexp_stack_cc.skel (定义→落地映射表, 审计专用; 落地以就地修订为准)
//   + 落地对数核对 (缺项 throw 列票单)
import { readFileSync } from "node:fs";
import { join } from "node:path";
import { banner, readV8, root, writeSrc } from "../../lib.js";

const cc = readV8("src/regexp/regexp-stack.cc");

let assertCount = 0;
const fail = (label, got, want) => {
  throw new Error(`对数失配: ${label} got=${JSON.stringify(got)} want=${JSON.stringify(want)}`);
};
const count = (src, re) => (src.match(re) || []).length;

// 1) 方法定义全集 (13 条: StackScope ×2 + Stack 构造/析构/New/Delete/Archive/Restore/EnsureCapacity
//    + ThreadLocal ×4)
const DEFS = [...cc.matchAll(/\b((?:StackScope|Stack(?:::ThreadLocal)?)::(~?\w+))\s*\(/g)].map((m) => m[1]);
const DEF_TABLE = {
  "StackScope::StackScope": 1,
  "StackScope::~StackScope": 1,
  "Stack::Stack": 1,
  "Stack::~Stack": 1,
  "Stack::New": 1,
  "Stack::Delete": 1,
  "Stack::ArchiveStack": 1,
  "Stack::RestoreStack": 1,
  "Stack::ThreadLocal::ResetToStaticStack": 1,
  "Stack::ThreadLocal::FreeAndInvalidate": 1,
  "Stack::ThreadLocal::NewDynamicStack": 1,
  "Stack::ThreadLocal::DeleteDynamicStack": 1,
  "Stack::EnsureCapacity": 1,
};
const DEF_TOTAL = Object.keys(DEF_TABLE).length; // 13
if (DEFS.length !== DEF_TOTAL) fail("cc 方法定义总数", DEFS.length, DEF_TOTAL);
assertCount++;
const byDef = {};
for (const d of DEFS) byDef[d] = (byDef[d] || 0) + 1;
for (const [d, want] of Object.entries(DEF_TABLE)) {
  if (byDef[d] !== want) fail(`定义 ${d}`, byDef[d], want);
  assertCount++;
}

// 2) 沙箱硬件分支: #ifdef ×4 (New/Delete/NewDynamicStack/DeleteDynamicStack),
//    含 #endif 尾注合计 7 处指令; 默认配置 (非沙箱硬件) 全部排除落地
const SANDBOX_IFDEFS = 4;
if (count(cc, /#ifdef V8_ENABLE_SANDBOX_HARDWARE_SUPPORT/g) !== SANDBOX_IFDEFS)
  fail("沙箱 #ifdef 组数", count(cc, /#ifdef V8_ENABLE_SANDBOX_HARDWARE_SUPPORT/g), SANDBOX_IFDEFS);
assertCount++;
const SANDBOX_DIRECTIVES = 7; // 4×#ifdef + 3×#endif 尾注
if (count(cc, /V8_ENABLE_SANDBOX_HARDWARE_SUPPORT/g) !== SANDBOX_DIRECTIVES)
  fail("沙箱硬件宏指令总数", count(cc, /V8_ENABLE_SANDBOX_HARDWARE_SUPPORT/g), SANDBOX_DIRECTIVES);
assertCount++;
// 非默认面 (落地排除, 见票单): VirtualAddressSpace / SandboxHardwareSupport / PagePermissions 各 ≥1
for (const [re, label] of [
  [/VirtualAddressSpace/g, "VirtualAddressSpace 引用"],
  [/SandboxHardwareSupport::RegisterUnsafeSandboxExtensionMemory/g, "沙箱注册调用"],
  [/PagePermissions::kReadWrite/g, "页权限参数"],
]) {
  if (count(cc, re) < 1) fail(`${label} 应存在 (排除分支)`, count(cc, re), ">=1");
  assertCount++;
}

// 3) 语义锚逐条对数 (EnsureCapacity 上限/最小值/顶拷贝/limit/复用 + Archive 强制动态 + MemCopy)
for (const [re, want, label] of [
  [/if \(size > kMaximumStackSize\) return kNullAddress;/g, 1, "上限 → kNullAddress"],
  [/if \(size < kMinimumDynamicStackSize\) size = kMinimumDynamicStackSize;/g, 1, "最小动态栈钳制"],
  [/new_memory \+ size - thread_local_\.memory_size_/g, 1, "旧内容拷入新内存顶部"],
  [/kStackLimitSlackSize;/g, 2, "limit = 基址 + slack (静态/动态两处)"],
  [/thread_local_\.memory_top_ \+ delta;/g, 1, "sp 复用 delta 迁移"],
  [/EnsureCapacity\(thread_local_\.memory_size_ \+ 1\)/g, 1, "归档前强制转动态栈"],
  [/MemCopy\(/g, 3, "MemCopy ×3 (archive/restore/顶拷贝)"],
  [/NewArray<uint8_t>\(size\)/g, 1, "非沙箱动态分配"],
  [/DeleteArray\(memory_\)/g, 1, "非沙箱动态释放"],
  [/thread_local_\.owns_memory_/g, 3, "owns_memory_ 判定 (archive 前检查/DCHECK/delete)"],
  [/thread_local_ = ThreadLocal\(this\);/g, 1, "归档后重建 ThreadLocal (回落静态)"],
]) {
  if (count(cc, re) !== want) fail(label, count(cc, re), want);
  assertCount++;
}
// CHECK/DCHECK 纪律锚 (StackScope 析构 CHECK_EQ + 构造 DCHECK IsValid 等)
if (count(cc, /\bCHECK_EQ\b/g) !== 1) fail("CHECK_EQ 总数", count(cc, /\bCHECK_EQ\b/g), 1);
assertCount++;
if (count(cc, /\bCHECK_LT\b/g) !== 1) fail("CHECK_LT 总数", count(cc, /\bCHECK_LT\b/g), 1);
assertCount++;
if (count(cc, /\bDCHECK/g) !== 3) fail("DCHECK 族总数", count(cc, /\bDCHECK/g), 3);
assertCount++;

// ── 产物: 定义→落地映射骨架 ────────────────────────────────────────────────
const skel = `${banner(
  "src/regexp/regexp-stack.cc.js",
  "src/regexp/regexp-stack.cc",
  "13 定义处置表 (审计专用): 落地以 regexp_stack.rs 就地修订为准",
)}
//! regexp-stack.cc → regexp_stack.rs 映射 (${DEF_TOTAL} 定义, 0 缺项):
//! - StackScope(Isolate*) → StackScope::new(&Stack) 参数注入; ~StackScope → Drop
//!   (原版 CHECK_EQ 走 v8_base::logging check_op_eq + v8_fatal, 非 panic 路径)
//! - Stack()/~Stack → Stack::new() / Drop; New/Delete → new()→Box 交付 + drop 承接
//!   (沙箱页分配分支 ${SANDBOX_IFDEFS} 组 #ifdef 排除, 见票单)
//! - ThreadLocal::ResetToStaticStack/IfEmpty/FreeAndInvalidate/NewDynamicStack/DeleteDynamicStack
//!   → 同名 snake_case 私有方法 (动态栈 = Box<[u8]> 精确 size 分配; 零初始化超集语义见票单)
//! - EnsureCapacity: 上限 kNullAddress / max(最小动态栈) / 顶拷贝 / delta 复用 / limit+slack 1:1
//! - ArchiveStack/RestoreStack → pub unsafe fn (char* 契约 → *mut u8 + SAFETY 块)
`;
writeSrc("v8-regexp-exec", "regexp_stack_cc.skel", skel);

// ── 落地对数核对 ───────────────────────────────────────────────────────────
const landed = readFileSync(join(root, "v8-regexp-exec", "src", "regexp_stack.rs"), "utf8");
const LAND_TABLE = [
  [/if size > K_MAXIMUM_STACK_SIZE \{\n      return K_NULL_ADDRESS;/g, 1, "上限 → K_NULL_ADDRESS"],
  [/\.max\(K_MINIMUM_DYNAMIC_STACK_SIZE\)/g, 1, "最小动态栈钳制"],
  [/new_memory\.add\(new_size - tl\.memory_size_\)/g, 1, "旧内容拷入新内存顶部"],
  [/\+ K_STACK_LIMIT_SLACK_SIZE/g, 2, "limit = 基址 + slack ×2"],
  [/offset_from\(tl\.memory_top_\)/g, 1, "delta 快照 (sp_top_delta)"],
  [/tl\.memory_top_\.offset\(delta\)/g, 1, "sp delta 迁移"],
  [/self\.ensure_capacity\(need\)/g, 1, "归档前强制转动态栈"],
  [/copy_nonoverlapping\(/g, 2, "归档/恢复 MemCopy 等价 ×2"],
  [/Box::into_raw\(buf\)/g, 1, "NewDynamicStack → Box 定长"],
  [/Box::<\[u8\]>::from_raw/g, 1, "DeleteDynamicStack → Box 归还"],
  [/reset_to_static_stack_if_empty/g, 2, "IfEmpty 定义+调用"],
  [/free_and_invalidate/g, 2, "~Stack → Drop 调 FreeAndInvalidate (定义+调用)"],
  [/check_op_eq\(&self\.old_sp_top_delta_/g, 1, "CHECK_EQ 承接"],
  [/v8_fatal\(format_args!/g, 1, "fatal 非 panic 路径"],
  [/self\.tl\(\)\.memory_size_ \+ 1/g, 1, "memory_size_+1 增长量"],
];
const missing = [];
for (const [re, want, label] of LAND_TABLE) {
  const got = count(landed, re);
  if (got !== want) missing.push(`${label}: got=${got} want=${want}`);
  assertCount++;
}
if (missing.length) throw new Error(`regexp_stack.rs 落地对数缺项 (cc 侧, 票单):\n  ${missing.join("\n  ")}`);

console.log(
  `regexp-stack.cc.js: 对数断言 ${assertCount} 条全过; 原版 ${DEF_TOTAL} 方法定义全集逐项命中; ` +
    `沙箱硬件分支 ${SANDBOX_DIRECTIVES} 指令 (排除落地); MemCopy ×3 / owns_memory_ ×3 / ` +
    `limit+slack ×2 语义锚 1:1; 落地 15 锚命中, 缺项 0; 产物 v8-regexp-exec/src/regexp_stack_cc.skel`,
);
