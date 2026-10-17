#!/usr/bin/env bun
// tool/src/objects/string.cc.js — 覆盖 string.cc 规则代码区段:
//   v8/src/objects/string.cc
//   → v8-objects-strings/src/string.rs 的 @cc-semantics 区段 (标记替换, 幂等)
//
// 转写区段登记 (string.cc 实测行号):
//   589-623  PrefixForDebugPrint/SuffixForDebugPrint if 链 → match 机械展开
//   2198-2201 ExternalPayloadSize → 1:1
//   2226-2357 ConsStringIterator::Initialize/Continue/Search/NextLeaf →
//            结构体 + 内联助手在此落骨架, 算法体由 Developer 修订填充
//   2418-2444 位布局 static_assert 命名空间 → const assert 机械展开
//
// 跳过区段登记 (推迟, 归属):
//   - 44-126   SlowShare/MigrateExternalString(Factory/Heap): v8-heap-*
//   - 128-541  MakeThin/MakeExternal*/MarkForExternalizationDuringGC/
//              ComputeExternalStringMap (roots 表): v8-heap-* + v8-sandbox
//   - 669-693  ToArrayIndex(Address)/ToNumber: runtime 层 (conversions+factory)
//   - 780-1123 WriteToFlat/WriteToFlat2: Developer 修订承接 (算法区, 见 string.rs)
//   - 1248-1302 CalculateLineEnds(FixedArray): FixedArray 归 v8-objects-arrays
//   - 1526-1749 IndexOf(Object)/GetSubstitution (builder/异常): v8-runtime
//   - 2052-2090 SeqString::Truncate (heap NotifyObjectSizeChange): v8-heap-*
//   - 2115-2126 SeqStringVerify (VERIFY_HEAP): v8-diagnostics
//   - 2203-2224 FlatStringReader::PostGarbageCollection (Relocatable): v8-handles
//   - 2438-2444 IsNotInternalizedBit 等 5 个 static_assert: 保留 const assert
import { banner, readV8 } from "../../lib.js";
import { readFileSync, writeFileSync } from "node:fs";
import { join } from "node:path";

const root = join(import.meta.dir, "../../..");
const cc = readV8("src/objects/string.cc");

// ── 锚点 ────────────────────────────────────────────────────────────────
for (const [sub] of [
  ["const char* String::PrefixForDebugPrint() const {"],
  ["int ExternalString::ExternalPayloadSize() const {"],
  ["void ConsStringIterator::Initialize(Tagged<ConsString> cons_string,"],
  ["using RepresentationBits ="],
  ["static_assert(kStringRepresentationMask == RepresentationBits::kMask);"],
]) {
  if (!cc.includes(sub)) throw new Error(`string.cc 锚点失配: "${sub.slice(0, 44)}..."`);
}

const section = `${banner("src/objects/string.cc.js", "src/objects/string.cc (规则代码区段)")}

// ── 位布局 const assert (string.cc:2418-2444; StringInstanceType 位段) ─────
// representation 3 bit | is_one_byte 1 bit | is_uncached 1 bit | is_shared 1 bit
// | is_not_internalized 1 bit (string.tq bitfield struct 1:1)
const _: () = assert!(
  v8_objects_core::instance_type::K_STRING_REPRESENTATION_MASK == 0x7
);
const _: () = assert!(v8_objects_core::instance_type::K_STRING_ENCODING_MASK == 0x8);
const _: () = assert!(v8_objects_core::instance_type::K_TWO_BYTE_STRING_TAG == 0);
const _: () = assert!(v8_objects_core::instance_type::K_ONE_BYTE_STRING_TAG == 0x8);
const _: () = assert!(
  v8_objects_core::instance_type::K_UNCACHED_EXTERNAL_STRING_MASK == 0x10
);
const _: () = assert!(v8_objects_core::instance_type::K_UNCACHED_EXTERNAL_STRING_TAG == 0x10);
const _: () = assert!(v8_objects_core::instance_type::K_IS_NOT_INTERNALIZED_MASK == 0x40);
const _: () = assert!(v8_objects_core::instance_type::K_NOT_INTERNALIZED_TAG == 0x40);
const _: () = assert!(v8_objects_core::instance_type::K_INTERNALIZED_TAG == 0x0);
const _: () = assert!(v8_objects_core::instance_type::K_SHARED_STRING_MASK == 0x20);
const _: () = assert!(v8_objects_core::instance_type::K_SHARED_STRING_TAG == 0x20);

// ── 调试打印前后缀 (string.cc:589-623; if 链机械展开) ─────────────────────
/// 短打印前缀 (u 前缀 = 双字节表示)。
#[must_use]
pub fn prefix_for_debug_print(shape: StringShape) -> &'static str {
  if shape.is_two_byte() {
    if shape.is_internalized() {
      "u#"
    } else if shape.is_cons() {
      "uc\\""
    } else if shape.is_thin() {
      "u>\\""
    } else if shape.is_external() {
      "ue\\""
    } else {
      "u\\""
    }
  } else if shape.is_internalized() {
    "#"
  } else if shape.is_cons() {
    "c\\""
  } else if shape.is_thin() {
    ">\\""
  } else if shape.is_external() {
    "e\\""
  } else {
    "\\""
  }
}

/// 短打印后缀 (string.cc:619-623)。
#[must_use]
pub fn suffix_for_debug_print(shape: StringShape) -> &'static str {
  if shape.is_internalized() { "" } else { "\\"" }
}

// ── ConsStringIterator (string.h:1416-1468 + string.cc:2226-2357) ─────────
/// ConsString 树迭代器: 全迭代可重启栈式遍历 (string.h:1416)。
pub struct ConsStringIterator {
  /// 栈帧 (右遍历未做的 ConsString)。
  frames: [Option<Tagged<ConsString>>; Self::STACK_SIZE],
  /// 根节点。
  root: Option<Tagged<ConsString>>,
  /// 当前深度。
  depth: i32,
  /// 深度峰值 (爆栈判定)。
  maximum_depth: i32,
  /// 已消费字符数。
  consumed: u32,
}

impl ConsStringIterator {
  /// 栈容量 (string.h:1443)。
  pub const STACK_SIZE: usize = 32;
  /// 深度掩码 (kStackSize - 1, 2 的幂)。
  pub const DEPTH_MASK: i32 = Self::STACK_SIZE as i32 - 1;

  /// OffsetForDepth (string-inl.h:1663)。
  #[inline]
  const fn offset_for_depth(depth: i32) -> usize {
    (depth & Self::DEPTH_MASK) as usize
  }

  /// PushLeft (string-inl.h:1665)。
  #[inline]
  fn push_left(&mut self, string: Tagged<ConsString>) {
    self.frames[Self::offset_for_depth(self.depth)] = Some(string);
    self.depth += 1;
  }

  /// PushRight 原地更新栈顶 (string-inl.h:1669)。
  #[inline]
  fn push_right(&mut self, string: Tagged<ConsString>) {
    self.frames[Self::offset_for_depth(self.depth - 1)] = Some(string);
  }

  /// AdjustMaximumDepth (string-inl.h:1674)。
  #[inline]
  fn adjust_maximum_depth(&mut self) {
    if self.depth > self.maximum_depth {
      self.maximum_depth = self.depth;
    }
  }

  /// Pop (string-inl.h:1678)。
  #[inline]
  fn pop(&mut self) {
    debug_assert!(self.depth > 0);
    debug_assert!(self.depth <= self.maximum_depth);
    self.depth -= 1;
  }

  /// StackBlown: maximum_depth - depth == kStackSize (string.h:1454)。
  #[inline]
  fn stack_blown(&self) -> bool {
    self.maximum_depth - self.depth == Self::STACK_SIZE as i32
  }
}
`;

// 幂等替换 @cc-semantics 区段
const target = join(root, "v8-objects-strings/src/string.rs");
let rs = readFileSync(target, "utf8");
const BEGIN = "// @cc-semantics-begin";
const END = "// @cc-semantics-end";
const bi = rs.indexOf(BEGIN);
const ei = rs.indexOf(END);
if (bi < 0 || ei < 0) throw new Error("string.rs 缺少 @cc-semantics 标记");
rs = rs.slice(0, bi + BEGIN.length) + "\n" + section + "\n" + rs.slice(ei);
writeFileSync(target, rs);
console.log("@cc-semantics 区段已注入: const assert 11 + 前后缀表 + ConsStringIterator 骨架");
