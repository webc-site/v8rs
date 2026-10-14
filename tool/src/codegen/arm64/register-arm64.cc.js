#!/usr/bin/env bun
// tool/src/codegen/arm64/register-arm64.cc.js
// 机械提取 v8/src/codegen/arm64/register-arm64.cc —
// VectorFormat 派生映射表 (半宽/倍宽/填充 Q/通道数/通道位宽/寄存器位宽)。
// 注: registers.rs (脚本产物) 覆盖 register-arm64.h, 本脚本落 _cc_generated 旁路。
// 产物: v8-codegen-arm64/src/registers_cc_generated.rs

import { readV8, writeSrc, banner } from "../../../lib.js";

const CC_REL = "src/codegen/arm64/register-arm64.cc";
const cc = readV8(CC_REL);

// 通用 switch 表提取: 函数名 → [(case 组, 返回值)]
function extractSwitch(fnName) {
  const re = new RegExp(`(?:VectorFormat|unsigned|int|int64_t) ${fnName}\\([^)]*\\)\\s*\\{([\\s\\S]*?)\\n\\}`);
  const m = cc.match(re);
  if (!m) throw new Error(`未找到函数 ${fnName}`);
  const body = m[1];
  // case 组后跟 return X; (可能多 case 共一 return)
  const groups = [];
  const re2 = /((?:\s*case k\w+:\n)+)\s*return ([^;]+);/g;
  let g;
  while ((g = re2.exec(body)) !== null) {
    const cases = [...g[1].matchAll(/case (k\w+):/g)].map((c) => c[1]);
    groups.push({ cases, ret: g[2].trim() });
  }
  if (!groups.length) throw new Error(`${fnName} 未提取到 case 表`);
  return groups;
}

const halfWidth = extractSwitch("VectorFormatHalfWidth");
const doubleWidth = extractSwitch("VectorFormatDoubleWidth");
const fillQ = extractSwitch("VectorFormatFillQ");
const laneBits = extractSwitch("LaneSizeInBitsFromFormat");
const laneCount = extractSwitch("LaneCountFromFormat");
const regBits = extractSwitch("RegisterSizeInBitsFromFormat");
for (const [name, groups] of [["HalfWidth", halfWidth], ["DoubleWidth", doubleWidth], ["LaneBits", laneBits], ["LaneCount", laneCount], ["RegBits", regBits]]) {
  const n = groups.reduce((a, g) => a + g.cases.length, 0);
  if (n < 5) throw new Error(`${name} case 覆盖不足: ${n}`);
}

// 常量锚 (kDRegSizeInBits 等来自 constants-arm64 覆盖面)
for (const anchor of ["kDRegSizeInBits", "kQRegSizeInBits", "kBRegSizeInBits"]) {
  if (!cc.includes(anchor)) throw new Error(`缺尺寸常量锚 ${anchor}`);
}

// 生成 match 表
const fmt = (name, groups, retType) => {
  let s = `/// 原 ${name} 的 switch 表展开\n#[rustfmt::skip]\npub fn ${name.replace(/^VectorFormat/, "").replace(/^([A-Z])/, (c) => c.toLowerCase())}_table(vform: crate::utils_generated::VectorFormat) -> Option<${retType}> {\n  use crate::utils_generated::VectorFormat as VF;\n  match vform {\n`;
  for (const g of groups) {
    s += `    ${g.cases.map((c) => `VF::${c}`).join(" | ")} => Some(${g.ret.replace(/k(Format\w+|BRegSizeInBits|HRegSizeInBits|SRegSizeInBits|DRegSizeInBits|QRegSizeInBits)/g, (mm, k) => (/Format/.test(k) ? `VF::${k}` : `crate::constants_generated::${k.replace(/^k/, "K_").replace(/([a-z0-9])([A-Z])/g, "$1_$2").toUpperCase()}`))}),\n`;
  }
  s += `    _ => None,\n  }\n}\n`;
  return s;
};

let rust = banner("src/codegen/arm64/register-arm64.cc.js", CC_REL, "VectorFormat 派生映射表");
rust += `
// 注: VectorFormat 枚举由 utils-arm64 覆盖面 (utils_generated) 提供;
// 尺寸常量由 constants-arm64 覆盖面 (constants_generated) 提供。
// 表中 default/UNREACHABLE 分支以 None 登记由调用方裁定。

${fmt("VectorFormatHalfWidth", halfWidth, "crate::utils_generated::VectorFormat")}
${fmt("VectorFormatDoubleWidth", doubleWidth, "crate::utils_generated::VectorFormat")}
${fmt("VectorFormatFillQ", fillQ, "crate::utils_generated::VectorFormat")}
${fmt("LaneSizeInBitsFromFormat", laneBits, "u32")}
${fmt("LaneCountFromFormat", laneCount, "i32")}
${fmt("RegisterSizeInBitsFromFormat", regBits, "u32")}

// 其余派生函数 (RegisterSizeInBytesFromFormat / LaneSizeInBytesFromFormat /
// LaneSizeInBytesLog2FromFormat / MaxLaneCountFromFormat / IsVectorFormat /
// MaxIntFromFormat / MinIntFromFormat / AreSameLaneSize) 可由上表组合, 语义面登记跳过。

// —— 提取不变式断言 ——
const _: () = {
  assert!(lane_size_in_bits_from_format_table(crate::utils_generated::VectorFormat::kFormat8B) == Some(8));
  assert!(lane_count_from_format_table(crate::utils_generated::VectorFormat::kFormat16B) == Some(16));
};
`;

const p = writeSrc("v8-codegen-arm64", "registers_cc_generated.rs", rust);
console.log(`written ${p} (表 ${[halfWidth, doubleWidth, fillQ, laneBits, laneCount, regBits].length} 张)`);
