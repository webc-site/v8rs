#!/usr/bin/env bun
// tool/src/base/platform/memory-protection-key.js — 覆盖 v8/src/base/platform/memory-protection-key.{h,cc}
// PKU 内存保护键: Permission 枚举 + 常量 + 11 静态方法签名 (V8_HAS_PKU_SUPPORT)
// → v8-base-platform/src/memory_protection_key.rs.skel

import { readV8, writeSrc, banner } from "../../../lib.js";

const hSrc = readV8("src/base/platform/memory-protection-key.h");
const ccSrc = readV8("src/base/platform/memory-protection-key.cc");

// 1. 门面: 整族包在 V8_HAS_PKU_SUPPORT 内 (目标 darwin arm64 无 PKU — 登记态)
if (!hSrc.includes("#if V8_HAS_PKU_SUPPORT") || !ccSrc.includes("#if V8_HAS_PKU_SUPPORT")) {
  throw new Error("V8_HAS_PKU_SUPPORT 门面缺失");
}

// 2. 常量: 哨兵 -1 / 默认键 0 / 键容量 16 / 每键位数 2 (x64)
const noKey = hSrc.match(/kNoMemoryProtectionKey = (-\d+);/)?.[1];
const defKey = hSrc.match(/kDefaultProtectionKey = (\d+);/)?.[1];
if (noKey !== "-1" || defKey !== "0") throw new Error(`保护键哨兵不符: ${noKey}/${defKey}`);
const maxKeys = ccSrc.match(/kMaxAvailableKeys = (\d+);/)?.[1];
const keysArray = ccSrc.match(/std::array<bool, kMaxAvailableKeys> g_active_keys/);
if (maxKeys !== "16" || !keysArray) throw new Error(`kMaxAvailableKeys 不符: ${maxKeys}`);
const bitsPerKey = ccSrc.match(/kBitsPerKey = (\d+);/)?.[1];
if (bitsPerKey !== "2") throw new Error(`kBitsPerKey 不符: ${bitsPerKey}`);

// 3. Permission 枚举: 值与 glibc PKEY_* 对齐
const permBody = hSrc.match(/enum Permission \{([^}]*)\};/);
if (!permBody) throw new Error("未找到 Permission 枚举");
const perms = [...permBody[1].matchAll(/k(\w+) = (\d+)/g)].map((m) => [m[1], Number(m[2])]);
if (perms.length !== 3) throw new Error(`Permission 条数不符: ${perms.length}, 期望 3`);
if (perms[0][0] !== "NoRestrictions" || perms[1][0] !== "DisableAccess" || perms[2][0] !== "DisableWrite") {
  throw new Error(`Permission 锚点不符: ${perms.map((p) => p[0])}`);
}
if (perms[0][1] !== 0 || perms[1][1] !== 1 || perms[2][1] !== 2) throw new Error("Permission 值不符");
if (!hSrc.includes("kDisableAccess == PKEY_DISABLE_ACCESS") || !hSrc.includes("kDisableWrite == PKEY_DISABLE_WRITE")) {
  throw new Error("PKEY glibc 对齐 static_assert 缺失");
}

// 4. 方法面 11 个 (GetKeyPermission 返回型为裸 Permission, 无 static 前缀)
const methods = [...hSrc.matchAll(/^  (?:static )?(?:bool|void|int|uint32_t|Permission) (\w+)\(/gm)].map((m) => m[1]);
const expected = ["HasMemoryProtectionKeyAPIs", "TestKeyAllocation", "AllocateKey", "RegisterExternallyAllocatedKey", "FreeKey", "SetPermissionsAndKey", "SetPermissionsForKey", "GetKeyPermission", "ComputeRegisterMaskForPermissionSwitch", "SetDefaultPermissionsForAllKeysInSignalHandler", "SetKeyForCurrentThreadsStack"];
for (const m of expected) {
  if (!hSrc.includes(`${m}(`)) throw new Error(`memory-protection-key.h 缺方法: ${m}`);
}
if (methods.length !== expected.length) throw new Error(`方法面计数不符: ${methods.length}, 期望 ${expected.length}`);

// 5. 实现侧: 弱符号五件套 + 活跃键簿记 + 寄存器掩码 x64 限定
const weakCount = (ccSrc.match(/V8_WEAK;/g) || []).length;
if (weakCount !== 5) throw new Error(`弱符号数不符: ${weakCount}, 期望 5 (pkey_mprotect/get/set/alloc/free)`);
for (const a of ["if (!pkey_mprotect) return false;", "CHECK_LT(key, kMaxAvailableKeys);", "g_active_keys[key] = true;", "return permissions << (key * kBitsPerKey);", "FATAL(\"Unsupported architecture\");", "PROT_READ | PROT_WRITE", "flags |= PROT_GROWSDOWN;"]) {
  if (!ccSrc.includes(a)) throw new Error(`memory-protection-key.cc 缺锚点: ${a}`);
}
// PROT_* 对齐断言 4 条
const protAsserts = (ccSrc.match(/PagePermissions::k\w+\) == PROT_\w+/g) || []).length;
if (protAsserts !== 4) throw new Error(`PROT 对齐断言数不符: ${protAsserts}, 期望 4`);

console.log(`memory-protection-key: 方法 ${methods.length}, Permission ${perms.length}, 弱符号 ${weakCount}, 活跃键容量 ${maxKeys}`);

let rust = banner("src/base/platform/memory-protection-key.js", "src/base/platform/memory-protection-key.{h,cc}", "PKU 内存保护键 (V8_HAS_PKU_SUPPORT 门面; 目标 darwin 无 PKU — 登记骨架)");

rust += `
/// PKU 保护键权限 (原版 enum MemoryProtectionKey::Permission)
/// v8: src/base/platform/memory-protection-key.h Permission
/// 值与 glibc 对齐: kDisableAccess == PKEY_DISABLE_ACCESS(1),
/// kDisableWrite == PKEY_DISABLE_WRITE(2) — 无执行位, 键不能撤执行权
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum Permission {
  /// kNoRestrictions = 0
  NoRestrictions = 0,
  /// kDisableAccess = 1 (PKEY_DISABLE_ACCESS)
  DisableAccess = 1,
  /// kDisableWrite = 2 (PKEY_DISABLE_WRITE)
  DisableWrite = 2,
}

/// 无键哨兵 (原版 kNoMemoryProtectionKey; pkey_alloc 失败返回值, pkey_mprotect(-1) 退化为 mprotect)
/// v8: src/base/platform/memory-protection-key.h kNoMemoryProtectionKey
pub const NO_MEMORY_PROTECTION_KEY: i32 = -1;
/// 默认键 (原版 kDefaultProtectionKey — 用于解除键关联)
/// v8: src/base/platform/memory-protection-key.h kDefaultProtectionKey
pub const DEFAULT_PROTECTION_KEY: i32 = 0;
/// 键容量 (原版 cc 43 行注释: 16 keys on x64, 8 keys on arm64)
/// v8: src/base/platform/memory-protection-key.cc kMaxAvailableKeys
pub const MAX_AVAILABLE_KEYS: usize = 16;
/// 每键寄存器位数 (原版 x64 PKRU: 2 bit/键)
/// v8: src/base/platform/memory-protection-key.cc kBitsPerKey
pub const BITS_PER_KEY: i32 = 2;

// —— 活跃键簿记 (原版 std::array<bool, 16> g_active_keys) ——
/// v8: src/base/platform/memory-protection-key.cc g_active_keys
/// AllocateKey/RegisterExternallyAllocatedKey 置 true, FreeKey 清 false

// —— 11 静态方法签名骨架 (供 Developer 补语义) ——
/// v8: src/base/platform/memory-protection-key.cc HasMemoryProtectionKeyAPIs
/// 廉价探测: pkey_mprotect 非空即真 (其余四符号必齐, CHECK)
/// v8: src/base/platform/memory-protection-key.cc TestKeyAllocation
/// 昂贵探测: alloc+free 一轮 (两次系统调用)
/// v8: src/base/platform/memory-protection-key.cc AllocateKey
/// pkey_alloc(0, kNoRestrictions); 成功后 CHECK_LT(key,16) 并簿记
/// v8: src/base/platform/memory-protection-key.cc RegisterExternallyAllocatedKey
/// 外部键登记 (信号处理器全键默认权限依赖此簿记)
/// v8: src/base/platform/memory-protection-key.cc FreeKey
/// pkey_free(key)==0 后清簿记
/// v8: src/base/platform/memory-protection-key.cc SetPermissionsAndKey
/// pkey_mprotect(region, prot, key)==0; 失败仅容 ENOMEM (CHECK)
/// v8: src/base/platform/memory-protection-key.cc SetPermissionsForKey
/// pkey_set(key, permissions)==0
/// v8: src/base/platform/memory-protection-key.cc GetKeyPermission
/// pkey_get(key), 值域限 Permission 三态 (CHECK)
/// v8: src/base/platform/memory-protection-key.cc ComputeRegisterMaskForPermissionSwitch
/// x64: permissions << (key * 2); 其余架构 FATAL — 用于生成切权限代码
/// v8: src/base/platform/memory-protection-key.cc SetDefaultPermissionsForAllKeysInSignalHandler
/// 异步信号安全: 活跃键统一 kDisableWrite (needs_full_access 时 kNoRestrictions)
/// v8: src/base/platform/memory-protection-key.cc SetKeyForCurrentThreadsStack
/// pthread_getattr_np 取栈界; limit_address 页对齐截断;
/// pkey_mprotect(PROT_READ|PROT_WRITE) 失败重试 +PROT_GROWSDOWN (主线程栈)

// —— PagePermissions ↔ PROT_* 对齐 (原版 4 条 static_assert) ——
/// v8: src/base/platform/memory-protection-key.cc GetProtectionFromMemoryPermission
/// kNoAccess==PROT_NONE / kRead==PROT_READ / kWrite==PROT_WRITE / kExecute==PROT_EXEC

// —— 提取不变式断言 ——
const _: () = {
  assert_eq!(NO_MEMORY_PROTECTION_KEY, -1);
  assert_eq!(DEFAULT_PROTECTION_KEY, 0);
  assert_eq!(Permission::DisableAccess as i32, 1);
  assert_eq!(Permission::DisableWrite as i32, 2);
  assert_eq!(MAX_AVAILABLE_KEYS, 16);
  assert_eq!(BITS_PER_KEY, 2);
  // x64 掩码: key=0 → 值原样 (0 移位), kBitsPerKey=2 与 PKRU 2bit/键一致
  assert_eq!((1 << (0 * BITS_PER_KEY)), 1);
};
`;

const p = writeSrc("v8-base-platform", "memory_protection_key.rs.skel", rust);
console.log(`written ${p}`);
