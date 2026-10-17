// flags.h → v8-base/src/flags.rs 转换脚本
// 规则: Flags<EnumT, BitfieldT> 类模板的运算符集机械展开为 Rust trait 方法清单
import { readV8, writeSrc, banner, lines } from "../../lib.js";

const src = readV8("src/base/flags.h");
const ops = lines(src, 58, 108); // set/contains/with/without/bitwise 运算符块
const ctor = lines(src, 30, 46); // 构造函数块

writeSrc(
  "v8-base",
  "flags.rs.skel",
  `${banner("src/base/flags.js", "src/base/flags.h")}// 构造区: ${ctor.join(" | ")}\n// 运算区: ${ops.join(" | ")}\n`,
);
console.log("flags.js: 骨架已产出 v8-base/src/flags.rs.skel");
