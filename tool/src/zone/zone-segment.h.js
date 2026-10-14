#!/usr/bin/env bun
// tool/src/zone/zone-segment.h.js
// 机械提取 v8/src/zone/zone-segment.{h,cc}: Segment 布局与 zap 协议

import { readV8, writeSrc, banner } from "../../lib.js";

const hSrc = readV8("src/zone/zone-segment.h");
const ccSrc = readV8("src/zone/zone-segment.cc");

// 对数守卫
for (const k of ["ZapContents", "ZapHeader", "kZapDeadByte = 0xcd", "size_ - sizeof(Segment)", "friend class AccountingAllocator"]) {
  if (!hSrc.includes(k)) throw new Error(`zone-segment.h 未找到 ${k}`);
}
if (!ccSrc.includes("kZapDeadByte")) throw new Error("cc 未实现 zap");
// DEBUG 专用 zap 字节
const zapM = hSrc.match(/static const unsigned char kZapDeadByte = (0x[0-9a-f]+);/);
if (!zapM || zapM[1] !== "0xcd") throw new Error("kZapDeadByte 漂移");

// 成员面 (zone_/next_/size_)
const members = ["Zone* zone_", "Segment* next_", "const size_t size_"];
for (const m of members) {
  if (!hSrc.includes(m)) throw new Error(`Segment 缺成员 ${m}`);
}

let rust = banner("src/zone/zone-segment.h.js", "src/zone/zone-segment.{h,cc}", "段布局与 zap 协议");

rust += `
/// Segment 布局常量 (原版 zone-segment.h)
pub mod segment_const {
  /// DEBUG 模式死内存 zap 字节 (原版 kZapDeadByte)
  pub const ZAP_DEAD_BYTE: u8 = 0xcd;

  /// 头部 = { zone_, next_, size_ } 三字段
  pub const HEADER_FIELDS: [&str; 3] = ["zone", "next", "size"];

  /// 容量 = 总大小 - 头部 (原版 capacity: size_ - sizeof(Segment))
  pub fn capacity(total_size: usize, header_size: usize) -> usize {
    total_size - header_size
  }
}

/// Segment 签名骨架 (语义见 crate 内手写 zone/segment.rs)
pub mod segment_skel {
  /// 内容 zap, 头部保留 (原版 ZapContents; DEBUG 填 0xcd)
  pub fn zap_contents() {
    unimplemented!("骨架")
  }
  /// 头部 zap, 段自此不可用 (原版 ZapHeader)
  pub fn zap_header() {
    unimplemented!("骨架")
  }
  /// 起始地址 = 头部之后 (原版 start: address(sizeof(Segment)))
  pub fn start() -> usize {
    unimplemented!("骨架")
  }
  /// 结束地址 = size_ 处 (原版 end)
  pub fn end() -> usize {
    unimplemented!("骨架")
  }
}
`;

const p = writeSrc("v8-zone", "zone/segment_generated.rs", rust);
console.log(`written ${p}; zap字节=${zapM[1]}`);
