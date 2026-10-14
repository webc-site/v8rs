#!/usr/bin/env bun
// tool/src/base/cpu/cpu-s390.cc.js — 族: src/base/cpu/cpu-s390.cc
// 覆盖: CPU::DetectFeatures() 空实现 (s390x 无运行时探测)
// → v8-base/src/cpu_s390.rs.skel

import { readV8, writeSrc, banner } from "../../../lib.js";

const src = readV8("src/base/cpu/cpu-s390.cc");

// —— 锚点断言 ——
if (!src.includes("void CPU::DetectFeatures() {}")) throw new Error("缺空 DetectFeatures");
const implCount = (src.match(/CPU::/g) || []).length;
if (implCount !== 1) throw new Error(`CPU 定义数 ${implCount} != 1`);
console.log("cpu-s390: DetectFeatures 1 (空实现, 无特性探测)");

let rust = banner(
  "src/base/cpu/cpu-s390.cc.js",
  "src/base/cpu/cpu-s390.cc",
  "s390x 特性探测空实现",
);

rust += `
//! v8: src/base/cpu/cpu-s390.cc

impl Cpu {
    /// v8: cpu-s390.cc DetectFeatures — 空实现 (s390x 无需运行时探测)
    #[cfg(target_arch = "s390x")]
    fn detect_features(&mut self) {}
}
`;

writeSrc("v8-base", "cpu_s390.rs.skel", rust);
console.log("written v8-base/src/cpu_s390.rs.skel");
