#!/usr/bin/env bun
// tool/src/objects/js-regexp-string-iterator.h.js — 覆盖 js-regexp-string-iterator 同名族:
//   v8/src/objects/js-regexp-string-iterator.h + -inl.h (+ .tq 布局源)
//   → v8-objects-strings/src/js_regexp_string_iterator.rs
//
// 映射登记 (C++ → Rust):
//   JSRegExpStringIterator (V8_OBJECT JSObject 子类) → 字段结构体占位
//     (JSReceiver/String 具体类型归后续 crate → Tagged<Object> 占位,
//      登记遗留: 集成时替换为强类型 TaggedMember)
//   TaggedMember<JSReceiver>/TaggedMember<String>/TaggedMember<Smi> →
//     pub 字段 (load/store 内联访问器 → 直读直写)
//   BitField 链 DoneBit(0,1)→GlobalBit(1,1)→UnicodeBit(2,1) → 显式 SHIFT 常量
//   Smi::From31BitPattern(value) → Smi::from_int (31 位内等价)
//   BOOL_ACCESSORS(done/global/unicode) → 位读写方法
//   WriteBarrierMode 参数: 写屏障归 heap 层 → 省略 (占位结构体无堆绑定)
//
// 跳过区段登记 (推迟):
//   JSObject 头部布局 (map/properties/elements) 与字段堆偏移: 归
//     v8-objects-js-core; 本模块只落字段三元组与标志位语义
//   @@iterator 协议运行时行为 (%RegExpStringIteratorPrototype%.next):
//     L4 builtins (js-regexp-string-iterator.tq 语义)
//   DECL_PRINTER/DECL_VERIFIER: L4 diagnostics
import { banner, readV8, writeSrc } from "../../lib.js";

const h = readV8("src/objects/js-regexp-string-iterator.h");
const inl = readV8("src/objects/js-regexp-string-iterator-inl.h");

const anchors = [
  [h, "using DoneBit = base::BitField<bool, 0, 1>;", "h"],
  [h, "using GlobalBit = DoneBit::Next<bool, 1>;", "h"],
  [h, "using UnicodeBit = GlobalBit::Next<bool, 1>;", "h"],
  [inl, "int JSRegExpStringIterator::flags() const { return flags_.load().value(); }", "inl"],
  [inl, "flags_.store(this, Smi::From31BitPattern(value));", "inl"],
  [inl, "BOOL_ACCESSORS(JSRegExpStringIterator, flags, done, DoneBit::kShift)", "inl"],
];
for (const [s, sub, f] of anchors) {
  if (!s.includes(sub)) throw new Error(`锚点失配: ${f} 无 "${sub.slice(0, 52)}..."`);
}

// BitField 链机械展开: Done(0,1) → Global(1,1) → Unicode(2,1)
const fields = [
  { name: "DONE", shift: 0, size: 1 },
  { name: "GLOBAL", shift: 1, size: 1 },
  { name: "UNICODE", shift: 2, size: 1 },
];
// 校验链连续
{
  let next = 0;
  for (const f of fields) {
    if (f.shift !== next) throw new Error(`BitField 链断裂: ${f.name}`);
    next = f.shift + f.size;
  }
}
const bitConsts = fields
  .map(
    (f) => `/// ${f.name} 位偏移 (${f.name}Bit::kShift)。
pub const ${f.name}_SHIFT: u32 = ${f.shift};
/// ${f.name} 位宽。
pub const ${f.name}_SIZE: u32 = ${f.size};
/// ${f.name} 掩码。
pub const ${f.name}_MASK: u32 = ((1u32 << ${f.size}) - 1) << ${f.shift};`,
  )
  .join("\n\n");

const accessors = fields
  .map(
    (f) => `  /// [[${f.name == "DONE" ? "Done" : f.name.charAt(0) + f.name.slice(1).toLowerCase()}]] 内部属性 (BOOL_ACCESSORS ${f.name.toLowerCase()})。
  #[inline]
  #[must_use]
  pub const fn ${f.name.toLowerCase()}(&self) -> bool {
    (Self::flags_bits(self.flags) & ${f.name}_MASK) != 0
  }

  /// 置 [[${f.name == "DONE" ? "Done" : f.name.charAt(0) + f.name.slice(1).toLowerCase()}]] 属性。
  #[inline]
  pub fn set_${f.name.toLowerCase()}(&mut self, value: bool) {
    let bits = Self::flags_bits(self.flags);
    let updated = if value {
      bits | ${f.name}_MASK
    } else {
      bits & !${f.name}_MASK
    };
    self.flags = Smi::from_int(updated as i32);
  }`,
  )
  .join("\n\n");

const body = `${banner("src/objects/js-regexp-string-iterator.h.js", "src/objects/js-regexp-string-iterator.{h,-inl.h}", "对象字段三元组 + 标志位语义 1:1; JSObject 头布局与 @@iterator 运行时推迟 (登记见脚本头)")}
//! JSRegExpStringIterator: \`String.prototype.matchAll\` 返回的迭代器对象。
//!
//! 字段布局占位 (JSObject 头部与堆偏移归 v8-objects-js-core, 登记见
//! 脚本头); 本模块落地字段三元组与 flags 位段语义, 运行时行为
//! (@@iterator 协议 next) 推迟 L4。

use v8_objects_core::smi::Smi;
use v8_objects_core::tagged::{Object, Tagged};

// BitField 链布局自检: 三位互不重叠
const _: () = assert!(DONE_MASK & GLOBAL_MASK == 0);
const _: () = assert!(DONE_MASK & UNICODE_MASK == 0);
const _: () = assert!(GLOBAL_MASK & UNICODE_MASK == 0);

// ── flags 位段 (BitField 链机械展开: Done(0,1)→Global(1,1)→Unicode(2,1)) ──
${bitConsts}

/// JSRegExpStringIterator 对象 (字段占位形态)。
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct JSRegExpStringIterator {
  /// 被迭代的 RegExp 对象 (TaggedMember<JSReceiver>; JSReceiver 占位 Object)。
  pub iterating_reg_exp: Tagged<Object>,
  /// 被迭代的字符串 (TaggedMember<String>; String 占位 Object)。
  pub iterated_string: Tagged<Object>,
  /// 标志位 Smi (SmiTagged<JSRegExpStringIteratorFlags>)。
  pub flags: Tagged<Smi>,
}

impl JSRegExpStringIterator {
  /// 全零构造 (工厂分配语义归 L4; 测试/占位用)。
  #[inline]
  #[must_use]
  pub const fn new(
    iterating_reg_exp: Tagged<Object>,
    iterated_string: Tagged<Object>,
    flags: Tagged<Smi>,
  ) -> Self {
    Self {
      iterating_reg_exp,
      iterated_string,
      flags,
    }
  }

  /// flags 的整型位型 (inl.h:39 flags(): Smi 解包)。
  #[inline]
  const fn flags_bits(flags: Tagged<Smi>) -> u32 {
    Smi::value(flags) as u32
  }

  /// 读 flags 整型值 (原版 flags())。
  #[inline]
  #[must_use]
  pub const fn flags(&self) -> i32 {
    Smi::value(self.flags)
  }

  /// 写 flags 整型值 (inl.h:41 set_flags: From31BitPattern → from_int)。
  #[inline]
  pub fn set_flags(&mut self, value: i32) {
    self.flags = Smi::from_int(value);
  }

  /// 读被迭代的 RegExp 对象 (iterating_reg_exp)。
  #[inline]
  #[must_use]
  pub const fn iterating_reg_exp(&self) -> Tagged<Object> {
    self.iterating_reg_exp
  }

  /// 写被迭代的 RegExp 对象 (set_iterating_reg_exp; 写屏障归 heap 层)。
  #[inline]
  pub fn set_iterating_reg_exp(&mut self, value: Tagged<Object>) {
    self.iterating_reg_exp = value;
  }

  /// 读被迭代字符串 (iterated_string)。
  #[inline]
  #[must_use]
  pub const fn iterated_string(&self) -> Tagged<Object> {
    self.iterated_string
  }

  /// 写被迭代字符串 (set_iterated_string)。
  #[inline]
  pub fn set_iterated_string(&mut self, value: Tagged<Object>) {
    self.iterated_string = value;
  }

${accessors}
}
`;

writeSrc("v8-objects-strings", "js_regexp_string_iterator.rs", body);
console.log("js_regexp_string_iterator.rs: 位段 3 项 + 访问器族; 运行时行为推迟登记");
