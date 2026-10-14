#!/usr/bin/env bun
// tool/src/diagnostics/eh-frame.js — 从 v8/src/diagnostics/eh-frame.{h,cc} + <arch>/eh-frame-<arch>.cc 提取
// EhFrameConstants (DWARF opcode/编码说明符/标签掩码/记录布局常量) 与 x64/arm64 架构参数,
// 生成 v8-diagnostics/src/eh_frame_data.rs (纯数据面; 记录布局含编译期派生)
// 复跑幂等: 纯文本扫描 + 固定序输出, 无时间戳。
import { readV8, writeSrc, banner } from "../../lib.js";

const h = readV8("src/diagnostics/eh-frame.h");
const cc = readV8("src/diagnostics/eh-frame.cc");
const x64 = readV8("src/diagnostics/x64/eh-frame-x64.cc");
const arm64 = readV8("src/diagnostics/arm64/eh-frame-arm64.cc");
const globals = readV8("src/common/globals.h");

let n = 0; // 对数计数
const die = (m) => { throw new Error(`[eh-frame.js] ${m}`); };
const need = (src, s, where) => { if (!src.includes(s)) die(`${where} 缺针 ${s}`); n++; };
function grab(src, re, what) {
  const m = src.match(re);
  if (!m) die(`未找到 ${what}`);
  return m[1];
}

// ── DwarfOpcodes 枚举 (10 项, 值须逐一相符) ──
const opcodeBlock = grab(h, /enum class DwarfOpcodes : uint8_t \{([\s\S]*?)\};/, "DwarfOpcodes");
// 值断言基线 (原 OPCODE_NAMES 仅名单成员, 值漂移可穿透 → 逐值钉死)
const EXPECTED_OPCODES = { kNop: 0, kAdvanceLoc1: 2, kAdvanceLoc2: 3, kAdvanceLoc4: 4,
  kRestoreExtended: 6, kSameValue: 8, kDefCfa: 12, kDefCfaRegister: 13, kDefCfaOffset: 14, kOffsetExtendedSf: 17 };
const opcodePairs = [...opcodeBlock.matchAll(/(k\w+)\s*=\s*(0x[0-9a-fA-F]+)\s*,/g)]
  .map((m) => [m[1], parseInt(m[2], 16)]);
if (opcodePairs.length !== 10) die(`DwarfOpcodes 项数 ${opcodePairs.length} != 10`);
for (const [name, val] of opcodePairs) {
  if (!(name in EXPECTED_OPCODES)) die(`DwarfOpcodes 意外项 ${name}`);
  if (EXPECTED_OPCODES[name] !== val) die(`DwarfOpcodes 值漂移 ${name}=${val} != ${EXPECTED_OPCODES[name]}`);
  n++;
}
n++;

// ── 编码说明符 (5 项) ──
const specBlock = grab(h, /enum DwarfEncodingSpecifiers : uint8_t \{([\s\S]*?)\};/, "DwarfEncodingSpecifiers");
const EXPECTED_SPECS = { kUData4: 0x03, kSData4: 0x0b, kPcRel: 0x10, kDataRel: 0x30, kOmit: 0xff };
const specPairs = [...specBlock.matchAll(/(k\w+)\s*=\s*(0x[0-9a-fA-F]+)\s*,/g)]
  .map((m) => [m[1], parseInt(m[2], 16)]);
if (specPairs.length !== 5) die(`DwarfEncodingSpecifiers 项数 ${specPairs.length} != 5`);
for (const [name, val] of specPairs) {
  if (!(name in EXPECTED_SPECS)) die(`意外说明符 ${name}`);
  if (EXPECTED_SPECS[name] !== val) die(`说明符值漂移 ${name}=${val} != ${EXPECTED_SPECS[name]}`);
}
n++;

// ── 标签/掩码 9 项 + 布局常量 8 项 + 占位符 ──
const intConsts = ["kLocationTag", "kLocationMask", "kLocationMaskSize", "kSavedRegisterTag",
  "kSavedRegisterMask", "kSavedRegisterMaskSize", "kFollowInitialRuleTag", "kFollowInitialRuleMask",
  "kFollowInitialRuleMaskSize", "kEhFrameTerminatorSize", "kInitialStateOffsetInCie",
  "kFdeVersionSize", "kFdeEncodingSpecifiersSize", "kEhFrameHdrVersion", "kEhFrameHdrSize"];
const ints = {};
// 值断言基线: 这些常量被 eh_frame.rs 行为面手工复用 (标签算术/落点 DCHECK), 仅锚名称不够, 须钉值
const EXPECTED_INTS = { kLocationTag: 1, kLocationMask: 63, kLocationMaskSize: 6,
  kSavedRegisterTag: 2, kSavedRegisterMask: 63, kSavedRegisterMaskSize: 6,
  kFollowInitialRuleTag: 3, kFollowInitialRuleMask: 63, kFollowInitialRuleMaskSize: 6,
  kEhFrameTerminatorSize: 4, kInitialStateOffsetInCie: 19, kFdeVersionSize: 1,
  kFdeEncodingSpecifiersSize: 3, kEhFrameHdrVersion: 1, kEhFrameHdrSize: 20 };
for (const name of intConsts) {
  const raw = grab(h, new RegExp(`static const int ${name} = (0x[0-9a-fA-F]+|\\d+);`), name);
  ints[name] = parseInt(raw, raw.startsWith("0x") ? 16 : 10);
  if (ints[name] !== EXPECTED_INTS[name]) die(`${name} 值漂移 ${ints[name]} != ${EXPECTED_INTS[name]}`);
  n++;
}
// FDE 内过程地址/大小编码偏移 = 2/3 × kInt32Size, kInt32Size 在 globals.h 定义
need(globals, "constexpr int kInt32Size = sizeof(int32_t);", "globals.h");
need(h, "static const int kProcedureAddressOffsetInFde = 2 * kInt32Size;", "eh-frame.h");
need(h, "static const int kProcedureSizeOffsetInFde = 3 * kInt32Size;", "eh-frame.h");
const PLACEHOLDER = grab(h, /static const uint32_t kInt32Placeholder = (0x[0-9a-fA-F]+);/, "kInt32Placeholder");
if (parseInt(PLACEHOLDER, 16) !== 0xdeadc0de) die(`kInt32Placeholder 值漂移 ${PLACEHOLDER}`);
n++;

// ── CIE/FDE 关键针 (生成侧行为由 eh_frame.rs 手工承接, 此处仅锚定源码未漂移) ──
need(cc, "static const int kCIEIdentifier = 0;", "eh-frame.cc");
need(cc, "static const int kCIEVersion = 3;", "eh-frame.cc");
need(cc, "static const int kAugmentationDataSize = 2;", "eh-frame.cc");
need(cc, "static const uint8_t kAugmentationString[] = {'z', 'L', 'R', 0};", "eh-frame.cc");
need(cc, "int padding_size = RoundUp(unpadded_size, kSystemPointerSize) - unpadded_size;", "eh-frame.cc");
// 空帧 (WriteEmptyEhFrame) 面
need(cc, "char dummy_data[EhFrameConstants::kEhFrameHdrSize - 4] = {0};", "eh-frame.cc");

// ── 架构参数: x64 / arm64 ──
function archParams(src, reg) {
  const codes = {};
  for (const m of src.matchAll(/static const int k(\w+)DwarfCode = (\d+);/g)) codes[m[1]] = parseInt(m[2]);
  const codeAlign = parseInt(grab(src, /kCodeAlignmentFactor = (-?\d+);/, "kCodeAlignmentFactor"));
  const dataAlign = parseInt(grab(src, /kDataAlignmentFactor = (-?\d+);/, "kDataAlignmentFactor"));
  return { codes, codeAlign, dataAlign, reg };
}
const x = archParams(x64, "x64");
const a = archParams(arm64, "arm64");
// x64: RA=rip(16); 初始态 = def_cfa(rsp, ptr) + ra 保存于 -ptr
if (x.reg !== "x64") die("arch 错位");
// 架构值断言: 对齐因子/DWARF 码被写出器行为面直接复用, 仅锚存在不够, 漂移必须显形
if (x.codeAlign !== 1 || x.dataAlign !== -8 || x.codes.Rip !== 16 || x.codes.Rsp !== 7)
  die(`x64 架构参数漂移 align=${x.codeAlign}/${x.dataAlign} ra=${x.codes.Rip} rsp=${x.codes.Rsp}`);
if (a.codeAlign !== 4 || a.dataAlign !== -8 || a.codes.Lr !== 30 || a.codes.Fp !== 29)
  die(`arm64 架构参数漂移 align=${a.codeAlign}/${a.dataAlign} lr=${a.codes.Lr} fp=${a.codes.Fp}`);
need(x64, "WriteULeb128(kRipDwarfCode);", "eh-frame-x64.cc");
need(x64, "SetBaseAddressRegisterAndOffset(rsp, kSystemPointerSize);", "eh-frame-x64.cc");
need(x64, "RecordRegisterSavedToStack(kRipDwarfCode, -kSystemPointerSize);", "eh-frame-x64.cc");
// arm64: RA=lr(30); 初始态 = def_cfa(fp, 0) + lr same_value
need(arm64, "WriteULeb128(kLrDwarfCode);", "eh-frame-arm64.cc");
need(arm64, "SetBaseAddressRegisterAndOffset(x29, 0);", "eh-frame-arm64.cc");
need(arm64, "RecordRegisterNotModified(x30);", "eh-frame-arm64.cc");
// kSystemPointerSize = sizeof(void*); wave1 仅 64 位, 由 Rust 编译期断言承接
need(globals, "constexpr int kSystemPointerSize = sizeof(void*);", "globals.h");
const SYS_PTR = "size_of::<usize>()";
n++;
// 64 位锚定: 校验本生成基线仅适用于 64 位目标
const SYS_PTR_CHECK = "const _: () = assert!(size_of::<usize>() == 8);";

const pascal = (k) => k.slice(1)[0] + k.slice(2); // kAdvanceLoc1 → AdvanceLoc1
const snake = (k) => k.slice(1).replace(/([a-z0-9])([A-Z])/g, "$1_$2").toUpperCase();

const out = [];
out.push(banner("src/diagnostics/eh-frame.js", "src/diagnostics/eh-frame.h;src/diagnostics/eh-frame.cc;src/diagnostics/x64/eh-frame-x64.cc;src/diagnostics/arm64/eh-frame-arm64.cc"));
out.push(`//! EH_FRAME 帧记录纯数据面 (基线生成; EH_FRAME 记录写出/读取行为见 eh_frame.rs)。`);
out.push(SYS_PTR_CHECK);
out.push(``);
out.push(`// ── DwarfOpcodes (${opcodePairs.length} 项) ──`);
out.push(`#[derive(Debug, Clone, Copy, PartialEq, Eq)]`);
out.push(`#[repr(u8)]`);
out.push(`pub enum DwarfOpcode {`);
for (const [name, val] of opcodePairs) out.push(`  ${pascal(name)} = ${val},`);
out.push(`}`);
out.push(``);
out.push(`// ── DwarfEncodingSpecifiers (${specPairs.length} 项) ──`);
for (const [name, val] of specPairs) out.push(`pub const ${snake(name)}: u8 = ${val};`);
out.push(``);
out.push(`// ── 指令标签/掩码 ──`);
for (const name of intConsts.slice(0, 9)) out.push(`pub const ${snake(name)}: i32 = ${ints[name]};`);
out.push(``);
out.push(`// ── 记录布局常量 (kInt32Size 编译期派生自 size_of::<i32>()) ──`);
out.push(`pub const INT32_SIZE: usize = size_of::<i32>();`);
out.push(`pub const PROCEDURE_ADDRESS_OFFSET_IN_FDE: usize = 2 * INT32_SIZE;`);
out.push(`pub const PROCEDURE_SIZE_OFFSET_IN_FDE: usize = 3 * INT32_SIZE;`);
for (const name of ["kEhFrameTerminatorSize", "kInitialStateOffsetInCie", "kFdeVersionSize",
  "kFdeEncodingSpecifiersSize", "kEhFrameHdrVersion", "kEhFrameHdrSize"]) {
  out.push(`pub const ${snake(name)}: usize = ${ints[name]};`);
}
out.push(``);
out.push(`// ── FDE/CIE 头字段值 (eh-frame.cc WriteCie/WriteFdeHeader) ──`);
out.push(`pub const INT32_PLACEHOLDER: u32 = ${PLACEHOLDER};`);
out.push(`pub const CIE_IDENTIFIER: i32 = 0;`);
out.push(`pub const CIE_VERSION: u8 = 3;`);
out.push(`pub const AUGMENTATION_DATA_SIZE: u32 = 2;`);
out.push(`pub const AUGMENTATION_STRING: [u8; 4] = [b'z', b'L', b'R', 0];`);
out.push(`pub const FDE_PAD_ALIGNMENT: usize = ${SYS_PTR};`); // RoundUp(unpadded, kSystemPointerSize)
out.push(`// ── 架构参数 (代码对齐 × 数据对齐 × RA 伪寄存器 DWARF 码 × CIE 初始态) ──`);
out.push(`#[derive(Debug, Clone, Copy, PartialEq, Eq)]`);
out.push(`pub struct EhFrameArchParams {`);
out.push(`  pub code_align: i32,`);
out.push(`  pub data_align: i32,`);
out.push(`  pub ra_code: u32,`);
out.push(`  pub cfa_reg_code: u32,`);
out.push(`  pub cfa_offset: i32,`);
out.push(`  pub saved_ra_offset: Option<i32>,`);
out.push(`  pub same_value_reg: Option<u32>,`);
out.push(`}`);
out.push(``);
out.push(`/// x64: def_cfa(rsp, 8); rip 保存于 -8`);
out.push(`pub const EH_FRAME_X64: EhFrameArchParams = EhFrameArchParams {`);
out.push(`  code_align: ${x.codeAlign},`);
out.push(`  data_align: ${x.dataAlign},`);
out.push(`  ra_code: ${x.codes.Rip},`);
out.push(`  cfa_reg_code: ${x.codes.Rsp},`);
out.push(`  cfa_offset: size_of::<usize>() as i32,`);
out.push(`  saved_ra_offset: Some(-(size_of::<usize>() as i32)),`);
out.push(`  same_value_reg: None,`);
out.push(`};`);
out.push(``);
out.push(`/// arm64: def_cfa(fp, 0); lr same_value`);
out.push(`pub const EH_FRAME_ARM64: EhFrameArchParams = EhFrameArchParams {`);
out.push(`  code_align: ${a.codeAlign},`);
out.push(`  data_align: ${a.dataAlign},`);
out.push(`  ra_code: ${a.codes.Lr},`);
out.push(`  cfa_reg_code: ${a.codes.Fp},`);
out.push(`  cfa_offset: 0,`);
out.push(`  saved_ra_offset: None,`);
out.push(`  same_value_reg: Some(${a.codes.Lr}),`);
out.push(`};`);

writeSrc("v8-diagnostics", "eh_frame_data.rs", out.join("\n").trimEnd() + "\n");
console.log(`OK eh-frame.js: 对数 ${n} (opcode 10 + 说明符 5 + 常量 15+针 + 架构 2) → v8-diagnostics/src/eh_frame_data.rs`);
console.log(`  x64 align=${x.codeAlign}/${x.dataAlign} ra=${x.codes.Rip} cfa=rsp(${x.codes.Rsp})+8 saved_ra=-8`);
console.log(`  arm64 align=${a.codeAlign}/${a.dataAlign} ra=${a.codes.Lr} cfa=fp(${a.codes.Fp})+0 same_value=lr(${a.codes.Lr})`);
console.log(`  布局: initial_state@${ints.kInitialStateOffsetInCie} hdr_size=${ints.kEhFrameHdrSize} terminator=${ints.kEhFrameTerminatorSize}`);
