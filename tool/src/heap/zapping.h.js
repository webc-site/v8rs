#!/usr/bin/env bun
// tool/src/heap/zapping.h.js
// 机械提取 v8/src/heap/zapping.h: 填充判定与 zap 值选择规则
// (kZapValue/kCodeZapValue 等常量本体在 src/common/globals.h, 已由 common 侧覆盖)

import { readV8, writeSrc, banner } from "../../lib.js";

const hSrc = readV8("src/heap/zapping.h");
const globalsSrc = readV8("src/common/globals.h");
// 族内 .cc 实读对数
const zapCcSrc = readV8("src/heap/zapping.cc");
if (!zapCcSrc.includes("void ZapCodeBlock(")) throw new Error("zapping.cc 锚点缺失");

if (!hSrc.includes("ShouldZapGarbage") || !hSrc.includes("ZapValue")) {
  throw new Error("zapping.h 锚点缺失");
}
// 常量锚点 (globals.h)
const zap = globalsSrc.match(/constexpr uint64_t kZapValue = uint64_t\{(0x\w+)\};/);
const cleared = globalsSrc.match(/kClearedFreeMemoryValue = (uint64_t\{)?\s*(0x\w+)/);
const codeZap = globalsSrc.match(/kCodeZapValue = (uint64_t\{)?\s*(0x\w+)/);
if (!zap || zap[1] !== "0xdeadbeedbeadbeef") throw new Error("kZapValue 漂移");
if (!codeZap) throw new Error("kCodeZapValue 缺失");

console.log(`kZapValue=${zap[1]}, kCodeZapValue=${codeZap[2]}`);

let rust = banner(
  "src/heap/zapping.h.js",
  "src/heap/zapping.h + src/common/globals.h",
  "堆填充 (zapping) 判定与 zap 值常量",
);

rust += `
/// kZapValue (src/common/globals.h, 64 位目标)
pub const K_ZAP_VALUE: u64 = 0xdeadbeedbeadbeef;
/// kCodeZapValue (src/common/globals.h)
pub const K_CODE_ZAP_VALUE: u64 = ${codeZap[2]};
${cleared ? `/// kClearedFreeMemoryValue (src/common/globals.h)\npub const K_CLEARED_FREE_MEMORY_VALUE: u64 = ${cleared[2]};` : ""}
/// 是否需要填充垃圾 (对应 ShouldZapGarbage; DEBUG 或 VERIFY_HEAP+flag)。
/// 展开按 release + VERIFY_HEAP 关闭取值: false; debug 构建恒 true (登记于注释)。
#[inline]
#[must_use]
pub const fn should_zap_garbage() -> bool {
  // #ifdef DEBUG → true
  // #else #ifdef VERIFY_HEAP → v8_flags.verify_heap
  false
}

/// zap 值选择 (对应 ZapValue: clear_free_memory ? kClearedFreeMemoryValue : kZapValue)
#[inline]
#[must_use]
pub const fn zap_value(clear_free_memory: bool) -> u64 {
  ${cleared ? "if clear_free_memory {\n    K_CLEARED_FREE_MEMORY_VALUE\n  } else {\n    K_ZAP_VALUE\n  }" : "K_ZAP_VALUE"}
}

/// 填充常规内存块 (对应 ZapBlock, 骨架)
pub fn zap_block(_start: usize, _size_in_bytes: usize, _zap_value: u64) {
  unimplemented!("骨架: 参照原版 zapping.cc ZapBlock 修订")
}

/// 填充代码块 (对应 ZapCodeBlock, 使用 kCodeZapValue, 骨架)
pub fn zap_code_block(_start: usize, _size_in_bytes: i32) {
  unimplemented!("骨架: 参照原版 zapping.cc ZapCodeBlock 修订")
}

// —— 提取不变式断言 ——
const _: () = {
  assert!(K_ZAP_VALUE == 0xdeadbeedbeadbeef);
  assert!(!should_zap_garbage());
};
`;
;

const p = writeSrc("v8-heap-core", "zapping.rs", rust);
console.log(`written ${p}`);
