#!/usr/bin/env bun
// tool/src/base/ios-headers.h.js — 族: src/base/ios-headers.h
// 覆盖: iOS 非公开 SDK 的 mach_vm_remap / mach_vm_map 声明
// → v8-base/src/ios_headers.rs.skel

import { readV8, writeSrc, banner } from "../../lib.js";

const src = readV8("src/base/ios-headers.h");

// —— 锚点断言 ——
if (!src.includes("#include <mach/mach.h>")) throw new Error("缺 mach include");
if (!src.includes("#include <mach/vm_map.h>")) throw new Error("缺 vm_map include");
if (!src.includes("__BEGIN_DECLS")) throw new Error("缺 C 链接声明");

// —— 真解析: mach 函数清单, 对数 ——
const fns = [...src.matchAll(/^(\w+) mach_(\w+)\(/gm)].map((m) => `mach_${m[2]}`);
if (fns.join(",") !== "mach_vm_remap,mach_vm_map") throw new Error(`mach 函数 ${fns}`);
console.log("ios-headers: extern C 声明 2 (mach_vm_remap / mach_vm_map)");

let rust = banner(
  "src/base/ios-headers.h.js",
  "src/base/ios-headers.h",
  "iOS 私有 mach API 声明 (内存分配支持)",
);

rust += `
//! v8: src/base/ios-headers.h — 仅 iOS 目标, 非 公共 SDK 头补声明

// v8: kern_return_t mach_vm_remap(target_task, target_address, size, mask, flags,
//     src_task, src_address, copy, cur_protection, max_protection, inheritance)
// v8: kern_return_t mach_vm_map(target_task, address, size, mask, flags, object,
//     offset, copy, cur_protection, max_protection, inheritance)
// Rust: #[cfg(target_os = "ios")] extern "C" 声明, liblibc 未覆盖时 Developer 手写,
//       非目标平台登记跳过
`;

writeSrc("v8-base", "ios_headers.rs.skel", rust);
console.log("written v8-base/src/ios_headers.rs.skel");
