// pointer-with-payload.h → v8-base/src/pointer_with_payload.rs 转换脚本
// 规则: kAvailableBits 对齐推导 + 掩码运算机械展开
import { readV8, writeSrc, banner, lines } from "../../lib.js";

const src = readV8("src/base/pointer-with-payload.h");
const traits = lines(src, 12, 17); // kAvailableBits 推导
const masks = lines(src, 100, 112); // kPayloadMask/kPointerMask/断言

writeSrc(
  "v8-base",
  "pointer_with_payload.rs.skel",
  `${banner("src/base/pointer-with-payload.js", "src/base/pointer-with-payload.h")}// 掩码区: ${masks.join(" | ")}\n// trait 区: ${traits.join(" | ")}\n`,
);
console.log("pointer-with-payload.js: 骨架已产出 v8-base/src/pointer_with_payload.rs.skel");
