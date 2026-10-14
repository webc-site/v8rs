#!/usr/bin/env bun
// tool/src/utils/hex-format.h.js
// 机械提取 v8/src/utils/hex-format.{h,cc}: 字节→十六进制格式化常量与骨架

import { readV8, writeSrc, banner } from "../../lib.js";

const hSrc = readV8("src/utils/hex-format.h");
const ccSrc = readV8("src/utils/hex-format.cc");

// 对数守卫
if (!/void FormatBytesToHex\(char\* formatted, size_t size_of_formatted,/.test(hSrc)) {
  throw new Error("hex-format.h 声明漂移");
}
// cc 侧防溢出 CHECK 界
const boundM = ccSrc.match(/CHECK_LT\(size_of_val, (0x[0-9a-fA-F]+)\);/);
if (!boundM || boundM[1] !== "0x20000000") throw new Error("防溢出上界漂移");
if (!ccSrc.includes("size_of_formatted >= (size_of_val * 2)")) throw new Error("缓冲宽关系漂移");
if (!ccSrc.includes('"%02x"')) throw new Error("十六进制格式串漂移");

let rust = banner("src/utils/hex-format.h.js", "src/utils/hex-format.{h,cc}", "字节十六进制格式化骨架");

rust += `
/// 字节→小写十六进制格式化 (原版 FormatBytesToHex)
pub mod hex_format_skel {
  /// 防溢出上界: 输入长 < 0x20000000 (格式化后 0x40000000 才溢出)
  pub const MAX_VAL_SIZE: usize = 0x2000_0000;

  /// 每字节产出 2 字符 (原版 size_of_formatted >= size_of_val * 2)
  pub const CHARS_PER_BYTE: usize = 2;

  /// 格式化骨架 (语义见手写 hex_format.rs; %02x 小写补零)
  pub fn format_bytes_to_hex(_val: &[u8], _formatted: &mut [u8]) {
    unimplemented!("骨架")
  }

  /// 输出缓冲宽需求
  #[must_use]
  pub const fn formatted_size(val_len: usize) -> usize {
    val_len * CHARS_PER_BYTE
  }
}
`;

const p = writeSrc("v8-utils", "hex_format_generated.rs", rust);
console.log(`written ${p}; 上界=0x20000000`);
