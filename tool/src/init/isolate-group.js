#!/usr/bin/env bun
// tool/src/init/isolate-group.js
// 机械提取 v8/src/init/isolate-group.{h,cc} 的隔离组常量与签名骨架

import { readV8, writeSrc, banner } from "../../lib.js";

const hSrc = readV8("src/init/isolate-group.h");
const ccSrc = readV8("src/init/isolate-group.cc");

// 1. 常量: kReadOnlyOrSharedEntryIsolateSentinel = -1 (uintptr 语义 → u64::MAX)
const sentinelM = hSrc.match(/kReadOnlyOrSharedEntryIsolateSentinel = (-\d+);/);
if (!sentinelM) throw new Error("未找到 kReadOnlyOrSharedEntryIsolateSentinel");
const sentinel = BigInt.asUintN(64, BigInt(sentinelM[1]));

// 2. CanCreateNewGroups: COMPRESS_POINTERS_IN_MULTIPLE_CAGES_BOOL 构建期值
//    目标配置 (x64+arm64, 单 cage 默认) 展开 false; 多 cage 配置 true — 登记于注释
const canNewM = hSrc.match(/static constexpr bool CanCreateNewGroups\(\) \{\s*return (\w+);/);
if (!canNewM) throw new Error("未找到 CanCreateNewGroups");

// 3. 公开静态方法签名 (h)
const classM = hSrc.match(/class V8_EXPORT_PRIVATE IsolateGroup final \{([\s\S]*?)\n\};/);
if (!classM) throw new Error("未找到 IsolateGroup 类体");
const staticSigs = [...classM[1].matchAll(/static\s+(?:IsolateGroup\*|void)\s+(\w+)\(([^)]*)\)/g)].map(
  (m) => ({ ret: m[0].split(/\s+/).slice(0, -2).join(" "), name: m[1], args: m[2].replace(/\s+/g, " ").trim() }),
);
if (staticSigs.length < 4) throw new Error(`静态签名过少: ${staticSigs.length}`);

// 4. cc 内实现函数清点 (机械登记)
const ccFns = [...ccSrc.matchAll(/^(?:void|IsolateGroup\*|Isolate\s?\*|uint32_t|size_t)\s+IsolateGroup::(\w+)/gm)].map((m) => m[1]);
if (ccFns.length < 8) throw new Error(`IsolateGroup:: 实现函数过少: ${ccFns.length}`);

let rust = banner("src/init/isolate-group.js", "src/init/isolate-group.{h,cc}", "隔离组: 指针压缩笼共享单元");
rust += `
/// IsolateGroup: 一组共享同一指针压缩笼 (pointer cage) 的 Isolate。
/// 压缩关闭时退化为全局单组; JS 对象只能在同组 Isolate 间传递 (API 用户责任)。

/// BasePageTableEntry 哨兵: 元数据可被任意 Isolate 读 (RO/共享页) 时,
/// isolate_ 字段置此值 (原版 constexpr = -1, uintptr 语义)。
pub const READ_ONLY_OR_SHARED_ENTRY_ISOLATE_SENTINEL: u64 = 0x${sentinel.toString(16)};

/// 能否新建隔离组 = COMPRESS_POINTERS_IN_MULTIPLE_CAGES_BOOL:
/// 目标配置 (x64+arm64, 默认单共享笼) 展开 false; 多笼构建 true (构建期决定, 此处登记)。
pub const CAN_CREATE_NEW_GROUPS: bool = false;

/// 公开静态 API 签名骨架 (原版 isolate-group.h):
`;
for (const s of staticSigs) {
  rust += `/// - \`${s.ret} ${s.name}(${s.args})\`\n`;
}
rust += `
/// cc 实现 (机械提取 IsolateGroup:: 方法名, 共 ${ccFns.length} 个):
/// ${ccFns.join(", ")}
`;

const p = writeSrc("v8-init", "isolate_group.rs", rust);
console.log(`written ${p}`);
