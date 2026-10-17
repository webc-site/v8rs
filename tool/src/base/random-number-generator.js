// utils/random-number-generator.{h,cc} → v8-base/src/random_number_generator.rs
// 规则: xorshift128+/MurmurHash3 常数与移位序列机械提取; SetSeed/Next* 抽样判定式
import { readV8, writeSrc, banner, lines } from "../../lib.js";

const h = readV8("src/base/utils/random-number-generator.h");
const cc = readV8("src/base/utils/random-number-generator.cc");

// 关键常数 (kMultiplier/kAddend/kMask 与 MurmurHash3 乘数)
const constants = {
  multiplier: "0x5deece66d",
  addend: "0xb",
  mask: "0xffffffffffff",
  murmur1: "0xFF51AFD7ED558CCD",
  murmur2: "0xC4CEB9FE1A85EC53",
};
// NextInt 抽样判定: power-of-two 快路径 + 拒绝采样条件
const nextInt = lines(cc, 92, 106);
const nextSample = lines(cc, 150, 190);
// xorshift128+ 状态机
const xorshift = lines(h, 122, 136);

writeSrc(
  "v8-base",
  "random_number_generator.rs.skel",
  `${banner("src/base/random-number-generator.js", "src/base/utils/random-number-generator.h + .cc")}
// 常数: ${JSON.stringify(constants)}
// xorshift 区:
${xorshift.join("\n")}
// NextInt 区:
${nextInt.join("\n")}
// NextSample 区:
${nextSample.join("\n")}
`,
);
console.log("random-number-generator.js: 骨架已产出 v8-base/src/random_number_generator.rs.skel");
