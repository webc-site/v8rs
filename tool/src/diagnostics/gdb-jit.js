#!/usr/bin/env bun
// tool/src/diagnostics/gdb-jit.js — 从 v8/src/diagnostics/gdb-jit.cc 提取 ELF/Mach-O 构造纯逻辑面:
// Writer 缓冲原语针、ELFHeader/ELFSectionHeader/MachO* 记录字段布局 (64 位)、
// ELFSection/MachOSection 类型与标志枚举、ELF ident 字节、GDB JIT 注册 ABI 结构 (JITCodeEntry/JITDescriptor)。
// 生成 v8-diagnostics/src/gdb_jit_data.rs (布局表 + 常量; 构造流程见 gdb_jit.rs)
// 复跑幂等: 纯文本扫描 + 源码序输出, 无时间戳。ia32 32 位分支推迟 (V8 ia32 面已冻结)。
import { readV8, writeSrc, banner } from "../../lib.js";

const src = readV8("src/diagnostics/gdb-jit.cc");
let n = 0;
const die = (m) => { throw new Error(`[gdb-jit.js] ${m}`); };
const need = (s, where = "gdb-jit.cc") => { if (!src.includes(s)) die(`${where} 缺针 ${s}`); n++; };

// Writer 原语锚定
need("capacity_(1024)", "Writer 初始容量");
need("while (capacity_ < pos) capacity_ *= 2;", "Writer 扩容");
need("void WriteULEB128(uintptr_t value)", "ULEB 写");
need("void WriteSLEB128(intptr_t value)", "SLEB 写");

// 64 位预处理器: 丢弃 IA32 分支保留 #else, 保留 X64 分支体
function preprocess(body) {
  return body
    .replace(/#if V8_TARGET_ARCH_IA32[\s\S]*?#else/g, "")
    .replace(/#if V8_TARGET_ARCH_X64/g, "")
    .replace(/#if V8_TARGET_ARCH_IA32\b[\s\S]*?#endif/g, "")
    .replace(/#endif/g, "")
    .replace(/#elif[^\n]*/g, "");
}

// 解析 C++ struct → 字段 (rust 类型, 名, 字节数)，并计算 64 位对齐布局
const WIRE = { uint8_t: ["u8", 1], uint16_t: ["u16", 2], uint32_t: ["u32", 4], uint64_t: ["u64", 8], uintptr_t: ["usize", 8], Address: ["usize", 8] };
function parseFields(name) {
  const m = src.match(new RegExp(`struct ${name} \\{([\\s\\S]*?)\\n\\s*\\};`));
  if (!m) die(`struct ${name} 未找到`);
  const fields = [];
  for (const line of preprocess(m[1]).split("\n")) {
    const t = line.trim();
    if (!t || t.startsWith("//")) continue;
    let fm = t.match(/^(?:const )?(uint8_t|uint16_t|uint32_t|uint64_t|uintptr_t|Address)\s+(\w+);$/);
    if (fm) { const sz = WIRE[fm[1]][1]; fields.push([WIRE[fm[1]][0], fm[2], sz, sz]); continue; }
    fm = t.match(/^uint8_t (\w+)\[(\d+)\];$/);
    if (fm) { fields.push([`[u8; ${fm[2]}]`, fm[1], parseInt(fm[2]), 1]); continue; }
    fm = t.match(/^char (\w+)\[(\d+)\];$/);
    if (fm) { fields.push([`[u8; ${fm[2]}]`, fm[1], parseInt(fm[2]), 1]); continue; }
    fm = t.match(/^(\w+)\* (\w+);$/); // JITCodeEntry* 等指针 → usize
    if (fm) { fields.push(["usize", fm[2], 8, 8]); continue; }
    die(`struct ${name} 无法解析行: ${JSON.stringify(t)}`);
  }
  // 布局: C 对齐规则; 总尺寸含尾部填充到最大字段对齐 (sizeof 语义, CreateSlotsHere 步长)
  let off = 0;
  let maxAlign = 1;
  const layout = [];
  for (const [ty, fname, size, align] of fields) {
    off = Math.ceil(off / align) * align;
    layout.push([fname, off, size, ty]);
    off += size;
    maxAlign = Math.max(maxAlign, align);
  }
  const total = Math.ceil(off / maxAlign) * maxAlign;
  n++;
  return { layout, total };
}

const STRUCTS = {
  ElfHeader: "ELFHeader",
  ElfSectionHeader: "ELFSectionHeader",
  MachOHeader: "MachOHeader",
  MachOSegmentCommand: "MachOSegmentCommand",
  MachOSectionHeader: "MachOSectionHeader",
  JitCodeEntry: "JITCodeEntry",
  JitDescriptor: "JITDescriptor",
};
const layouts = Object.fromEntries(Object.entries(STRUCTS).map(([rs, cpp]) => [rs, parseFields(cpp)]));

// ELF/Mach-O 枚举
function parseEnum(re, what, expect) {
  const m = src.match(re);
  if (!m) die(`枚举 ${what} 未找到`);
  const items = m[1].split(",").map((s) => s.trim()).filter((s) => s && !s.startsWith("//"));
  const values = [];
  let prev;
  for (const it of items) {
    const fm = it.match(/^(\w+)(?:\s*=\s*(0x[0-9a-fA-F]+|\d+)u?)?$/);
    if (!fm) die(`枚举 ${what} 无法解析: ${JSON.stringify(it)}`);
    const val = fm[2] === undefined ? (prev === undefined ? 0 : prev + 1) : parseInt(fm[2], fm[2].startsWith("0x") ? 16 : 10);
    prev = val;
    values.push([fm[1], val]);
  }
  if (values.length !== expect) die(`枚举 ${what} 项数 ${values.length} != ${expect}`);
  n++;
  return values;
}
const elfTypes = parseEnum(/enum Type \{(\s*\n\s*TYPE_NULL[\s\S]*?)\};/, "ELFSection::Type", 17);
const machoTypes = parseEnum(/enum Type \{(\s*S_REGULAR[\s\S]*?)\};/, "MachOSection::Type", 5);
const lcCmds = parseEnum(/enum MachOLoadCommandCmd \{([\s\S]*?)\};/, "MachOLoadCommandCmd", 2);
const jitActions = parseEnum(/enum JITAction \{([^}]*)\};/, "JITAction", 3);
// 值断言: parseEnum 仅数项数, 名单/值漂移 (如 TYPE_NULL 改值、JITAction 换序) 仍能通过 → 钉死首项与全序基线
if (elfTypes[0][0] !== "TYPE_NULL" || elfTypes[0][1] !== 0) die(`ELF Type 首项漂移 ${elfTypes[0]}`);
if (machoTypes[0][0] !== "S_REGULAR" || machoTypes[0][1] !== 0) die(`MachO Type 首项漂移 ${machoTypes[0]}`);
const EXPECTED_JIT = [["JIT_NOACTION", 0], ["JIT_REGISTER_FN", 1], ["JIT_UNREGISTER_FN", 2]];
for (const [i, [nm, vv]] of jitActions.entries()) {
  if (EXPECTED_JIT[i][0] !== nm || EXPECTED_JIT[i][1] !== vv) die(`JITAction[${i}] 漂移 ${nm}=${vv}`);
}

// 标志/特殊值
const elfFlags = [["SHF_WRITE", 1], ["SHF_ALLOC", 2], ["SHF_EXEC", 4]];
need("enum Flags { FLAG_WRITE = 1, FLAG_ALLOC = 2, FLAG_EXEC = 4 };", "ELFSection::Flags");
need("enum SpecialIndexes { INDEX_ABSOLUTE = 0xFFF1 };", "INDEX_ABSOLUTE");
const lc32 = lcCmds.find(([x]) => x === "LC_SEGMENT_32")[1];
const lc64 = lcCmds.find(([x]) => x === "LC_SEGMENT_64")[1];
if (lc32 !== 0x1 || lc64 !== 0x19) die(`LC_SEGMENT 值漂移 ${lc32}/${lc64}`);

// ELF ident (ELFCLASS64 LE 分支) 与 WriteHeader 关键常量
const ident64m = src.match(/const uint8_t ident\[16\] = \{0x7F, 'E', 'L', 'F', 2, 1, 1, 0,\s*\n\s*0,\s*0,\s*0,\s*0,\s*0, 0, 0, 0\};/);
if (!ident64m) die("ELFCLASS64 ident 未找到");
n++;
need("header->type = 1;", "ET_REL");
need("header->machine = 62;", "x64 machine");
need("header->version = 1;", "EV_CURRENT");
need("header->sht_strtab_index = 1;", "shstrndx");
need("cmd->maxprot = 7;", "PROT_ALL");
need("header->ncmds = 1;", "MH ncmds");
need("JITDescriptor __jit_debug_descriptor = {1, 0, nullptr, nullptr};", "descriptor 初始值");
const machoMagic = parseInt(src.match(/header->magic = (0xFEEDFACF)u;/)[1], 16);
// machoCpu 原为无锚硬编码 (源漂移仍通过); 补源码值针后派生。sub/filetype 同: 解析值须等于基线
if (!src.includes("header->cputype = 7 | 0x01000000;")) die("gdb-jit.cc 缺针 cputype = 7 | 0x01000000 (CPU_TYPE_X86_64)");
const machoCpu = 7 | 0x01000000;
const machoSub = parseInt(src.match(/header->cpusubtype = (\d+);/)[1]);
if (machoSub !== 3) die(`cpusubtype 漂移 ${machoSub} != 3 (CPU_SUBTYPE_I386_ALL)`);
const machoFiletype = parseInt(src.match(/header->filetype = (0x\w+);/)[1], 16);
if (machoFiletype !== 0x1) die(`filetype 漂移 ${machoFiletype} != 0x1 (MH_OBJECT)`);

const out = [];
out.push(banner("src/diagnostics/gdb-jit.js", "src/diagnostics/gdb-jit.cc"));
out.push(`//! GDB-JIT 镜像构造纯数据面 (基线生成; Writer/镜像拼装/注册流程见 gdb_jit.rs)。`);
out.push(`const _: () = assert!(size_of::<usize>() == 8);`);
out.push(``);
out.push(`// ── Writer 缓冲参数 ──`);
out.push(`pub const INITIAL_CAPACITY: usize = 1024;`);
out.push(``);
out.push(`// ── ELF ident / 头字段值 ──`);
out.push(`pub const ELF64_IDENT: [u8; 16] = [0x7F, b'E', b'L', b'F', 2, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0];`);
out.push(`pub const ELF_TYPE_REL: u16 = 1;`);
out.push(`pub const ELF_MACHINE_X64: u16 = 62;`);
out.push(`pub const ELF_VERSION: u32 = 1;`);
out.push(`pub const ELF_SHT_STRTAB_INDEX: u16 = 1;`);
out.push(``);
out.push(`// ── Mach-O 头常量 (x64 分支) ──`);
out.push(`pub const MACHO_MAGIC_64: u32 = ${machoMagic};`);
out.push(`pub const MACHO_CPU_TYPE_X86_64: u32 = ${machoCpu};`);
out.push(`pub const MACHO_CPU_SUBTYPE_ALL: u32 = ${machoSub};`);
out.push(`pub const MACHO_FILETYPE_OBJECT: u32 = ${machoFiletype};`);
out.push(`pub const MACHO_PROT_ALL: u32 = 7;`);
out.push(`pub const MACHO_NCMDS: u32 = 1;`);
out.push(`pub const LC_SEGMENT_32: u32 = ${lc32};`);
out.push(`pub const LC_SEGMENT_64: u32 = ${lc64};`);
out.push(``);
out.push(`// ── ELFSection::Type (${elfTypes.length} 项) ──`);
const pascalEnum = (s) => s.split("_").slice(1).map((w) => w[0] + w.slice(1).toLowerCase()).join("");
out.push(`#[derive(Debug, Clone, Copy, PartialEq, Eq)]`);
out.push(`#[repr(u32)]`);
out.push(`pub enum ElfSectionType {`);
for (const [name, val] of elfTypes) out.push(`  ${pascalEnum(name)} = ${val},`);
out.push(`}`);
out.push(``);
out.push(`// ── ELFSection::Flags / SpecialIndexes ──`);
for (const [name, val] of elfFlags) out.push(`pub const ${name}: u64 = ${val};`);
out.push(`pub const ELF_INDEX_ABSOLUTE: u16 = 0xFFF1;`);
out.push(``);
out.push(`// ── MachOSection::Type (${machoTypes.length} 项) ──`);
for (const [name, val] of machoTypes) out.push(`pub const ${name}: u32 = ${val};`);
out.push(``);
out.push(`// ── GDB JIT 注册 ABI (extern "C") ──`);
out.push(`#[derive(Debug, Clone, Copy, PartialEq, Eq)]`);
out.push(`#[repr(u32)]`);
out.push(`pub enum JitAction {`);
for (const [name, val] of jitActions) out.push(`  ${pascalEnum(name)} = ${val},`);
out.push(`}`);
out.push(`pub const JIT_DESCRIPTOR_VERSION: u32 = 1;`);
out.push(``);
out.push(`// ── 记录布局表 (64 位): [字段, 偏移, 大小] (SIZE = sizeof, 含尾部填充) ──`);
for (const [rs, { layout, total }] of Object.entries(layouts)) {
  out.push(`pub const ${rs.toUpperCase()}_FIELDS: &[(&str, usize, usize)] = &[`);
  for (const [fname, off, size] of layout) out.push(`  ("${fname}", ${off}, ${size}),`);
  out.push(`];`);
  out.push(`pub const ${rs.toUpperCase()}_SIZE: usize = ${total};`);
}
out.push(``);

writeSrc("v8-diagnostics", "gdb_jit_data.rs", out.join("\n").trimEnd() + "\n");
console.log(`OK gdb-jit.js: 对数 ${n} (struct 7 + 枚举 4 组 + ident + 针 ~12) → v8-diagnostics/src/gdb_jit_data.rs`);
for (const [rs, { layout, total }] of Object.entries(layouts)) {
  console.log(`  ${rs}: ${layout.length} 字段 size=${total}`);
}
console.log(`  elfTypes=${elfTypes.length} machoTypes=${machoTypes.length} lc=${lc32}/${lc64} jitActions=${jitActions.length}`);
