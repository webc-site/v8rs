#!/usr/bin/env bun
// tool/src/execution/protectors.js — Protectors 有效位表
// 覆盖: src/execution/protectors.h + protectors.cc + protectors-inl.h
// 产出: v8-execution-isolate/src/protectors.rs
// 规则: PROTECTOR_LIST V( 22 条全量 → 位标志枚举 + 有效位检查面。
// 复跑幂等: 纯文本扫描 + 确定性输出。
import { banner, readV8, writeSrc } from "../../lib.js";

const GEN = "src/execution/protectors.js";
const FILES = ["src/execution/protectors.h", "src/execution/protectors.cc", "src/execution/protectors-inl.h"];
const txt = Object.fromEntries(FILES.map((f) => [f, readV8(f)]));
const die = (m) => { throw new Error(`[protectors.js] ${m}`); };
const count = (s, re) => (s.match(re) ?? []).length;

const h = txt["src/execution/protectors.h"];
// ① V( 行 22 条 (grep 实测 ^  V()
const n = count(h, /^  V\(/gm);
if (n !== 22) die(`Protector V( 数=${n}, 期望 22`);
// ② 首尾代表针
for (const needle of ["V(ArrayBufferDetaching, ArrayBufferDetachingProtector,", "V(TypedArraySpeciesLookupChain, TypedArraySpeciesProtector,"]) {
  if (!h.includes(needle)) die(`缺针 ${needle}`);
}
// ③ 类面针
if (!h.includes("class Protectors")) die("缺 class Protectors");

const macroBlock = h.slice(h.indexOf("#define DECLARED_PROTECTORS_ON_ISOLATE"), h.indexOf("#define DECLARE_PROTECTOR_ON_ISOLATE"));
const items = [...macroBlock.matchAll(/V\(\s*(\w+),\s*(\w+),[\s\\]*(\w+)\s*\)/g)].map((m) => ({
  name: m[1],
  rootIndex: m[2],
  cell: m[3],
}));

if (items.length !== 22) die(`解析项数=${items.length}, 期望 22`);

const toSnakeCase = (str) =>
  str.replace(/([a-z0-9])([A-Z])/g, "$1_$2").toLowerCase();

const out = [];
out.push(banner(GEN, FILES.join(", "), "Protectors 22 有效位管理"));
out.push(`
/// 保护器有效标记 (对应原版 kProtectorValid)
pub const PROTECTOR_VALID: i32 = 1;
/// 保护器失效标记 (对应原版 kProtectorInvalid)
pub const PROTECTOR_INVALID: i32 = 0;

/// 22 个保护器枚举 (顺序与原版 DECLARED_PROTECTORS_ON_ISOLATE 严格 1:1)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum Protector {`);

items.forEach((item, idx) => {
  out.push(`  /// 原版 ${item.name} (RootIndex: ${item.rootIndex})`);
  out.push(`  ${item.name} = ${idx},`);
});

out.push(`}

impl Protector {
  /// 全部 22 项保护器列表
  pub const ALL: [Protector; 22] = [`);
for (const item of items) {
  out.push(`    Protector::${item.name},`);
}
out.push(`  ];

  /// 对应位掩码
  #[inline]
  pub const fn mask(self) -> u32 {
    1 << (self as u8)
  }

  /// 保护器名称
  #[inline]
  pub const fn name(self) -> &'static str {
    match self {`);
for (const item of items) {
  out.push(`      Protector::${item.name} => "${item.name}",`);
}
out.push(`    }
  }
}

/// 保护器有效位管理状态机 (22 项位图压缩)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Protectors {
  bits: u32,
}

impl Default for Protectors {
  #[inline]
  fn default() -> Self {
    Self::new()
  }
}

impl Protectors {
  /// 全有效位掩码 (低 22 位全 1)
  pub const ALL_VALID_MASK: u32 = (1 << 22) - 1;

  /// 初始化保护器：所有保护器默认完整有效
  #[inline]
  pub const fn new() -> Self {
    Self {
      bits: Self::ALL_VALID_MASK,
    }
  }

  /// 检查特定保护器是否依然完整有效 (对应原版 Is##name##Intact)
  #[inline]
  pub const fn is_intact(&self, protector: Protector) -> bool {
    (self.bits & protector.mask()) != 0
  }

  /// 将特定保护器置为失效 (对应原版 Invalidate##name)
  #[inline]
  pub fn invalidate(&mut self, protector: Protector) {
    self.bits &= !protector.mask();
  }

  /// 是否全部 22 个保护器均完整
  #[inline]
  pub const fn is_all_intact(&self) -> bool {
    self.bits == Self::ALL_VALID_MASK
  }

  /// 获取当前位图原始值
  #[inline]
  pub const fn raw_bits(&self) -> u32 {
    self.bits
  }
`);

for (const item of items) {
  const snake = toSnakeCase(item.name);
  out.push(`  /// 原版 Is${item.name}Intact`);
  out.push(`  #[inline]`);
  out.push(`  pub const fn is_${snake}_intact(&self) -> bool {`);
  out.push(`    self.is_intact(Protector::${item.name})`);
  out.push(`  }`);
  out.push(``);
  out.push(`  /// 原版 Invalidate${item.name}`);
  out.push(`  #[inline]`);
  out.push(`  pub fn invalidate_${snake}(&mut self) {`);
  out.push(`    self.invalidate(Protector::${item.name});`);
  out.push(`  }`);
  out.push(``);
}

out.push(`}

// ── 编译期静态断言 ──
const _: () = {
  assert!(Protector::ALL.len() == 22);
  assert!(Protectors::ALL_VALID_MASK == 0x003F_FFFF);
};
`);

writeSrc("v8-execution-isolate", "protectors.rs", out.join("\n") + "\n");
console.log(`OK protectors.js: ${items.length} 项 → v8-execution-isolate/src/protectors.rs`);
