// macros.h → v8-base/src/macros.rs 转换脚本
// 规则: 运行期语义函数 (RoundDown/RoundUp/IsAligned/make_uint64/is_inbounds)
// 机械提取; 纯预处理宏 (CONCAT/EXPAND/V8PRIxPTR 等) Rust 无对应, 登记不迁移
import { readV8, writeSrc, banner, lines } from "../../lib.js";

const src = readV8("src/base/macros.h");
const runtime = {
  makeUint64: lines(src, 408, 410),
  roundDown: lines(src, 412, 425),
  roundUp: lines(src, 427, 443),
  isAligned: lines(src, 445, 464),
  isInbounds: lines(src, 466, 486),
  arraysize: lines(src, 68, 90),
};

writeSrc(
  "v8-base",
  "macros.rs.skel",
  `${banner("src/base/macros.js", "src/base/macros.h")}\n` +
    Object.entries(runtime).map(([k, v]) => `// ---- ${k} ----\n${v.join("\n")}`).join("\n\n"),
);
console.log("macros.js: 骨架已产出 v8-base/src/macros.rs.skel");
