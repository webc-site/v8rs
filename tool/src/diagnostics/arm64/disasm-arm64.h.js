#!/usr/bin/env bun
// tool/src/diagnostics/arm64/disasm-arm64.h.js
// 机械提取 v8/src/diagnostics/arm64/disasm-arm64.{h,cc} 的规则面:
//   12 张 NEONFormatMap 静态格式表, barrier options 4x4, 2 张 Visit 内
//   mnemonics[] 助记索引表 (含取索引公式注释), disasm-arm64.h 类骨架签名。
// 语义面: 各 Visit* 解码体 / Substitute* 字段替换逻辑 (~4600 行) 不属规则代码,
//   登记跳过, 由 Developer 修订补齐。
// 产物: v8-codegen-arm64/src/disasm_arm64_generated.rs
//   (同族 instructions-arm64 存手写产物, 本脚本落 _generated 旁路, 禁覆写)

import { readV8, writeSrc, banner } from "../../../lib.js";

const H_REL = "src/diagnostics/arm64/disasm-arm64.h";
const CC_REL = "src/diagnostics/arm64/disasm-arm64.cc";
const hSrc = readV8(H_REL);
const cc = readV8(CC_REL);

// —— 按函数切块, 记录每个 static 所属的 Visit 函数 ——
const ccLines = cc.split("\n");
const fnRe = /^(?:void|int|const char\*|std::string) DisassemblingDecoder::(\w+)\(/;
let curFn = "(file-scope)";
const blocks = []; // {fn, start, end}
for (let i = 0; i < ccLines.length; i++) {
  const m = ccLines[i].match(fnRe);
  if (m) {
    if (blocks.length) blocks[blocks.length - 1].end = i;
    curFn = m[1];
    blocks.push({ fn: curFn, start: i, end: ccLines.length });
  }
}
const fnAt = (lineIdx) => {
  for (const b of blocks) if (lineIdx >= b.start && lineIdx < b.end) return b.fn;
  return "(file-scope)";
};

// 1. NEONFormatMap 静态实例: static const NEONFormatMap <name> = {{bits...}, {NF_...}};
const mapRe = /static const NEONFormatMap (\w+)\s*=\s*\{\s*\{([\d,\s]*?)\}\s*,\s*\{([^}]*)\}/g;
const formatMaps = [];
for (const m of cc.matchAll(mapRe)) {
  const bits = m[2].split(",").map((s) => parseInt(s.trim(), 10)).filter((n) => !Number.isNaN(n));
  const formats = [...m[3].matchAll(/NF_(\w+)/g)].map((s) => `NF_${s[1]}`);
  formatMaps.push({ name: m[1], bits, formats, fn: fnAt(cc.slice(0, m.index).split("\n").length - 1) });
}
if (formatMaps.length !== 12) throw new Error(`NEONFormatMap 实例条数不符: ${formatMaps.length} != 12`);
for (const fm of formatMaps) {
  if (fm.formats.length === 0) throw new Error(`NEONFormatMap ${fm.name} 格式列表为空`);
}

// 2. barrier options[4][4]
const optM = cc.match(/static const char\* const options\[4\]\[4\]\s*=\s*\{([\s\S]*?)\}\};/);
if (!optM) throw new Error("未找到 barrier options 表");
const optFlat = [...optM[1].matchAll(/"([^"]*)"/g)].map((s) => s[1]);
if (optFlat.length !== 16) throw new Error(`options 表元素数不符: ${optFlat.length} != 16`);
const optRows = [0, 1, 2, 3].map((i) => optFlat.slice(i * 4, i * 4 + 4));
if (optRows.some((r) => r.length !== 4)) throw new Error("options 表行切分失败");

// 3. Visit 函数内 mnemonics[] 助记索引表
const mnRe = /static const char\* mnemonics\[\]\s*=\s*\{([\s\S]*?)\};/g;
const mnemonicTables = [];
for (const m of cc.matchAll(mnRe)) {
  const line = cc.slice(0, m.index).split("\n").length - 1;
  const entries = [...m[1].matchAll(/"([^"]*)"|(nullptr)/g)].map((s) => (s[1] !== undefined ? s[1] : null));
  // 取索引公式可能在表前或表后, 上下各搜 60 行
  let formula = "";
  for (const dir of [-1, 1]) {
    for (let i = line + dir; i > line - 60 && i < line + 60 && i >= 0 && i < ccLines.length; i += dir) {
      if (/unsigned index =/.test(ccLines[i])) {
        let expr = ccLines[i].replace(/^\s*/, "");
        let j = i;
        while (!expr.includes(";") && j < ccLines.length - 1 && j > 0) {
          j += dir;
          expr += " " + ccLines[j].trim();
        }
        formula = expr.replace(/;\s*$/, "").replace(/instr->/g, "instr.");
        break;
      }
    }
    if (formula) break;
  }
  mnemonicTables.push({ fn: fnAt(line), entries, formula });
}
if (mnemonicTables.length !== 2) throw new Error(`mnemonics[] 表条数不符: ${mnemonicTables.length} != 2`);
for (const t of mnemonicTables) {
  if (t.entries.length < 16) throw new Error(`${t.fn} mnemonics 条数异常: ${t.entries.length}`);
  if (!t.formula) throw new Error(`${t.fn} 未定位到取索引公式`);
}

// 4. disasm-arm64.h: Substitute*/Format/Append* 方法签名骨架 + Visit 委托宏计数
const declSigs = [...hSrc.matchAll(/^\s*(?:virtual\s+)?((?:void|int|const char\*)\s+\w+\([^;]*\));/gm)].map(
  (m) => m[1].replace(/\s+/g, " ").trim(),
);
const visitMacroM = hSrc.match(/#define DECLARE\(A\) void Visit##A\(Instruction\* instr\);([\s\S]*?)VISITOR_LIST\(/);
if (!declSigs.length) throw new Error("disasm-arm64.h 未提取到方法签名");
if (!visitMacroM) throw new Error("disasm-arm64.h 未找到 VISITOR_LIST 声明宏锚");

// —— 生成 Rust ——
let rust = banner("src/diagnostics/arm64/disasm-arm64.h.js", "src/diagnostics/arm64/disasm-arm64.{h,cc}", "arm64 反汇编规则面: NEON 格式表/助记索引表/barrier 选项/骨架签名");
rust += `
// 原文件 #if V8_TARGET_ARCH_ARM64 门控 (arm64 专用), 本产物即 arm64 目标展开。
// 语义面登记跳过: DisassemblingDecoder::Visit*/Substitute* 解码替换实现
// (~4600 行) 不属机械规则代码, 由 Developer 修订补齐。
// NEONFormat (NF_*) 枚举本体在 instructions-arm64.h, 由
// tool/src/codegen/arm64/instructions-arm64.h.js 展开, 此处表值以名字登记。

/// NEON 格式映射 (原 struct NEONFormatMap): 按 bits 位取索引查 formats 表
#[derive(Debug, Clone, Copy)]
pub struct NeonFormatMap {
  /// 参与索引的指令位号 (原 forms_mask 初始化列表)
  pub bits: &'static [usize],
  /// NF_* 格式名 (原 NEONFormat 枚举值, 名字与
  /// crate 内 instructions_arm64_generated::NeonFormat 对应)
  pub formats: &'static [&'static str],
}

/// 原文件内 12 张 static const NEONFormatMap 实例 (含归属函数)
`;

for (const fm of formatMaps) {
  const bits = `[${fm.bits.join(", ")}]`;
  const fmts = fm.formats.map((f) => `"${f}"`).join(", ");
  rust += `/// 原位 ${fm.fn} 内 static ${fm.name}\npub static ${fm.name.toUpperCase()}: NeonFormatMap = NeonFormatMap { bits: &${bits}, formats: &[${fmts}] };\n\n`;
}

rust += `/// 屏障选项表 (原 SubstituteBarrierField 内 options[4][4], 索引 [domain][type])
#[rustfmt::skip]
pub static BARRIER_OPTIONS: [[&str; 4]; 4] = [
${optRows.map((r) => `  [${r.map((s) => `"${s}"`).join(", ")}],`).join("\n")}
];

/// Visit 函数内助记索引表 (原 static const char* mnemonics[32], nullptr 以 None 登记)
pub struct MnemonicTable {
  /// 归属的 DisassemblingDecoder::Visit 函数
  pub owner: &'static str,
  /// 取索引公式 (原文转录, instr-> 改 instr.)
  pub index_expr: &'static str,
  /// 助记符, None 即原 nullptr (外层 switch 特判)
  pub mnemonics: &'static [Option<&'static str>],
}

`;

for (const t of mnemonicTables) {
  rust += `/// 原 ${t.fn} 内助记表, 索引: \`${t.formula}\`
pub static ${t.fn.toUpperCase()}_MNEMONICS: MnemonicTable = MnemonicTable {
  owner: "${t.fn}",
  index_expr: "${t.formula.replace(/"/g, '\\"')}",
  mnemonics: [
${t.entries.map((e) => `    ${e === null ? "None" : `Some("${e}")`},`).join("\n")}
  ],
};

`;
}

rust += `// —— disasm-arm64.h 类面骨架 (DisassemblingDecoder/PrintDisassembler 签名) ——
// 原版经 DECLARE(A) × VISITOR_LIST 展开为全部 Visit<A>(); 此处登记声明锚。
// TODO(dev): 按 DecoderVisitor 框架补 trait 与实现。
${declSigs.map((s) => `// 原签名: ${s}`).join("\n")}

// —— 提取不变式断言 ——
const _: () = {
  assert!(BARRIER_OPTIONS.len() == 4);
  assert!(BARRIER_OPTIONS[0][0] == "sy (0b0000)");
  assert!(BARRIER_OPTIONS[3][3] == "sy");
${formatMaps
  .slice(0, 3)
  .map((fm) => `  assert!(!${fm.name.toUpperCase()}.formats.is_empty());`)
  .join("\n")}
${mnemonicTables
  .map((t) => `  assert!(${t.fn.toUpperCase()}_MNEMONICS.mnemonics.iter().filter(|m| m.is_some()).count() >= 24);`)
  .join("\n")}
};
`;

const p = writeSrc("v8-codegen-arm64", "disasm_arm64_generated.rs", rust);
console.log(`written ${p}`);
console.log(
  `NEONFormatMap ${formatMaps.length} 张, barrier 4x4, mnemonics 表 ${mnemonicTables.length} 张 (${mnemonicTables.map((t) => `${t.fn}:${t.entries.length}`).join(",")}), 签名 ${declSigs.length} 条`,
);
