#!/usr/bin/env bun
// v8/src/flags/flags-impl.h → v8-flags/src/lookup.rs (名字归一/查找层基线)
// 提取规则:
//   - FlagHelpers::NormalizeChar ('_'→'-') / FlagNamesCmp (归一化逐字节, 终止符最小) /
//     EqualNames / EqualNameWithSuffix (允许 b 以空白起始后缀收尾, 如 "max_opt < 3") → Rust 函数
//   - FindFlagByName / FindImplicationFlagByName 查找语义 (二分搜索 → phf 精确查表等价)
//   - Flag::FlagType / SetBy 变体核对 (与 value.rs 类型表一致)
// 原版二分搜索依赖名字唯一 (归一化后); 生成器已做重名检测, 故 phf 精确匹配等价
import { readV8, writeSrc, banner } from "../../lib.js";

const CRATE = "v8-flags";
const GEN = "src/flags/flags-impl.h.js";
const src = readV8("src/flags/flags-impl.h");

// ---- 提取: FlagType 变体 (TYPE_XXX) ----
const tyEnum = src.match(/enum FlagType \{([\s\S]*?)\n  \};/);
if (!tyEnum) throw new Error("flags-impl.h Flag::FlagType 未找到");
const types = [...tyEnum[1].matchAll(/\bTYPE_([A-Z0-9_]+)/g)].map((m) => m[1]);
if (types.length !== 8) throw new Error(`FlagType 变体数异常: ${types.length} ${types}`);

// ---- 提取: SetBy 变体 ----
const setBy = src.match(/enum class SetBy \{([^}]+)\}/);
if (!setBy) throw new Error("flags-impl.h SetBy 未找到");
const setBys = [...setBy[1].matchAll(/k([A-Za-z]+)/g)].map((m) => m[1]);
if (setBys.length !== 4) throw new Error(`SetBy 变体数异常: ${setBys.length}`);

// ---- 提取: NormalizeChar 实现 (核对本脚本转写依据) ----
const norm = src.match(/static constexpr char NormalizeChar\(char ch\) \{ return ch == '_' \? '-' : ch; \}/);
if (!norm) throw new Error("NormalizeChar 实现已变, 需人工核对");

// ---- 提取: EqualNameWithSuffix 语义 (b 允许空白后缀) ----
const suffix = src.match(/EqualNameWithSuffix/);
if (!suffix) throw new Error("EqualNameWithSuffix 未找到");

const out = [
  banner(GEN, "src/flags/flags-impl.h", "查找层: FlagHelpers 名字归一比较 + 按名查找"),
  `use crate::defs;`,
  ``,
  `/// 主 flag 下标 (0..NUM_FLAGS, 顺序=原版 flags[] 即 defs::FLAGS)`,
  `pub type FlagId = u32;`,
  ``,
  `/// 名字归一化字节 ('_' 视同 '-'; 原版 FlagHelpers::NormalizeChar)`,
  `#[inline]`,
  `pub(crate) const fn norm_byte(ch: u8) -> u8 {`,
  `  if ch == b'_' { b'-' } else { ch }`,
  `}`,
  ``,
  `/// 归一化名字比较 (原版 FlagHelpers::FlagNamesCmp: 逐字节归一比较, 先耗尽者小)`,
  `/// 返回 Ordering (原版返回 -1/0/1)`,
  `pub fn cmp_names(a: &str, b: &str) -> std::cmp::Ordering {`,
  `  a.bytes().map(norm_byte).cmp(b.bytes().map(norm_byte))`,
  `}`,
  ``,
  `/// 名字相等 (归一化后; 原版 FlagHelpers::EqualNames)`,
  `#[inline]`,
  `pub fn equal_names(a: &str, b: &str) -> bool {`,
  `  cmp_names(a, b) == std::cmp::Ordering::Equal`,
  `}`,
  ``,
  `/// a 与 b 前缀相等且 b 以空白起始后缀收尾 (原版 FlagHelpers::EqualNameWithSuffix)`,
  `/// 蕴含条件名形如 "max_opt < 3" 时按 "max_opt" 命中`,
  `pub fn equal_name_with_suffix(a: &str, b: &str) -> bool {`,
  `  match b.find(char::is_whitespace) {`,
  `    Some(cut) => equal_names(a, &b[..cut]),`,
  `    None => equal_names(a, b),`,
  `  }`,
  `}`,
  ``,
  `/// 归一化 (运行时) 名字查找 (原版 FindFlagByName: 精确匹配, 可查不存在的名)`,
  `pub fn find_flag_by_name(name: &str) -> Option<FlagId> {`,
  `  defs::NAME_MAP.get(norm_name(name).as_str()).copied()`,
  `}`,
  ``,
  `/// 蕴含条件名查找 (原版 FindImplicationFlagByName: 允许 "flag < N" 后缀;`,
  `/// 生成表内目标必然存在, 缺失即数据漂移)`,
  `pub fn find_implication_flag(name: &str) -> Option<FlagId> {`,
  `  let head = match name.find(char::is_whitespace) {`,
  `    Some(cut) => &name[..cut],`,
  `    None => name,`,
  `  };`,
  `  defs::NAME_MAP.get(norm_name(head).as_str()).copied()`,
  `}`,
  ``,
  `/// 名字归一化 ('_'→'-'); 常见名落栈缓冲, 超长才堆分配`,
  `fn norm_name(name: &str) -> NormBuf {`,
  `  const STACK_CAP: usize = 128; // 最长 flag 名 < 128 字节 (生成表实测 62)`,
  `  if name.len() <= STACK_CAP {`,
  `    let mut buf = [0u8; STACK_CAP];`,
  `    buf[..name.len()].copy_from_slice(name.as_bytes());`,
  `    for b in &mut buf[..name.len()] {`,
  `      *b = norm_byte(*b);`,
  `    }`,
  `    // 归一化后名均为 ASCII flag 名, from_utf8 必然成功`,
  `    NormBuf::Stack(buf, name.len())`,
  `  } else {`,
  `    NormBuf::Heap(name.replace('_', "-"))`,
  `  }`,
  `}`,
  ``,
  `/// 归一化名字缓冲 (栈优先)`,
  `enum NormBuf {`,
  `  Stack([u8; 128], usize),`,
  `  Heap(String),`,
  `}`,
  ``,
  `impl NormBuf {`,
  `  #[inline]`,
  `  fn as_str(&self) -> &str {`,
  `    match self {`,
  `      // 归一化仅替换 ASCII 字节, UTF-8 结构不变`,
  `      Self::Stack(buf, len) => std::str::from_utf8(&buf[..*len]).unwrap_or_default(),`,
  `      Self::Heap(s) => s,`,
  `    }`,
  `  }`,
  `}`,
  ``,
];

writeSrc(CRATE, "lookup.rs", out.join("\n") + "\n");
console.log(`lookup.rs: ${types.length} FlagType, ${setBys.length} SetBy (核对通过)`);
