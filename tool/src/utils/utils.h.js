#!/usr/bin/env bun
// tool/src/utils/utils.h.js
// 机械提取 v8/src/utils/utils.{h,cc} + utils-inl.h + memcopy.h + v8dll-main.cc:
// 通用工具面/页对齐协议/内存拷贝转发/DLL 入口

import { readV8, writeSrc, banner } from "../../lib.js";

const hSrc = readV8("src/utils/utils.h");
const ccSrc = readV8("src/utils/utils.cc");
const inlSrc = readV8("src/utils/utils-inl.h");
const memSrc = readV8("src/utils/memcopy.h");
const dllSrc = readV8("src/utils/v8dll-main.cc");

// 1. 内存拷贝转发三件
for (const k of ["MemMove", "MemCopy", "MemCopyAndSwitchEndianness"]) {
  if (!memSrc.includes(k)) throw new Error(`memcopy.h 缺 ${k}`);
}
if (!memSrc.includes("v8::base::MemCopy")) throw new Error("memcopy 转发缺失");

// 2. 页对齐协议 (utils.h RoundUpToPageSize)
if (!hSrc.includes("RoundUpToPageSize")) throw new Error("utils.h 缺 RoundUpToPageSize");
if (!/RoundUp only works when/.test(hSrc.replace(/\n/g, " "))) {
  if (!hSrc.includes("RoundUp only works when")) throw new Error("RoundUp 前置检查漂移");
}

// 3. cc 侧: 大缓冲上限等
const ccConsts = [...ccSrc.matchAll(/constexpr int k(\w+) = (\d+);/g)].map((m) => ({ name: m[1], val: m[2] }));

// 4. ARRAY_SET 宏 (utils.h)
if (!hSrc.includes("V(Name, (RoundUp<Alignment>(k##Name##Offset) - k##Name##Offset), hacker_name)")) {
  throw new Error("IS_FIELD_PADDING 宏漂移");
}

// 5. DLL 入口 (Windows 专用, 当前目标非 Windows)
if (!dllSrc.includes("DllMain") || !dllSrc.includes("V8_OS_WIN")) throw new Error("v8dll-main 结构漂移");

// 6. utils-inl 面
if (!inlSrc.includes("IsAddressAligned") && !/inline/.test(inlSrc)) throw new Error("utils-inl 面漂移");

let rust = banner(
  "src/utils/utils.h.js",
  "src/utils/utils.{h,cc,inl.h} + memcopy.h + v8dll-main.cc",
  "通用工具面与内存拷贝转发",
);

rust += `
/// 内存拷贝转发骨架 (原版 memcopy.h; 底座 v8::base::MemCopy/MemMove)
pub mod memcopy_skel {
  /// 拷贝 (原版 MemCopy; 平台最优实现转发)
  pub fn mem_copy(_dest: *mut u8, _src: *const u8, _size: usize) {
    unimplemented!("骨架")
  }
  /// 可重叠移动 (原版 MemMove)
  pub fn mem_move(_dest: *mut u8, _src: *const u8, _size: usize) {
    unimplemented!("骨架")
  }
  /// 拷贝并翻转字节序 (原版 MemCopyAndSwitchEndianness)
  pub fn mem_copy_switch_endianness(_dst: *mut u8, _src: *const u8, _num: usize, _elem_size: usize) {
    unimplemented!("骨架")
  }
}

/// 页对齐协议 (原版 utils.h RoundUpToPageSize)
pub mod utils_page_skel {
  /// RoundUp 前置: byte_length 加 page_size-1 不回绕 (原版注释)
  pub fn round_up_to_page_size(_byte_length: usize, _page_size: usize) -> usize {
    unimplemented!("骨架")
  }
}

/// 字段填充登记宏 (原版 IS_FIELD_PADDING: V(Name, RoundUp(Offset)-Offset, hacker_name))
pub const IS_FIELD_PADDING_PATTERN: &str = "V(Name, (RoundUp<Alignment>(k##Name##Offset) - k##Name##Offset), hacker_name)";

/// utils.cc 整型常量 (机械登记, ${ccConsts.length} 项)
pub mod utils_cc_consts {
${ccConsts.map((c) => `  /// 原版 k${c.name}\n  pub const ${c.name.replace(/([a-z0-9])([A-Z])/g, "$1_$2").toUpperCase()}: i32 = ${c.val};`).join("\n")}
}

/// DLL 入口 (原版 v8dll-main.cc; 仅 V8_OS_WIN 生效, 当前目标跳过并登记)
pub const DLL_MAIN_PLATFORM: &str = "windows-only; DllMain 空实现返回 1";
`;

const p = writeSrc("v8-utils", "utils_surface_generated.rs", rust);
console.log(`written ${p}; memcopy=3, cc常量=${ccConsts.length}`);
