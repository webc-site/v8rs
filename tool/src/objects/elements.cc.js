#!/usr/bin/env bun
// tool/src/objects/elements.cc.js — 覆盖 v8/src/objects/elements.cc
// → v8-objects-arrays/src/elements_impl.rs
// 机械提取: 顶层函数签名清单 (自由函数 + TypedElementsAccessor 特化 + 静态方法) +
//   访问器类清单 (每类类内方法计数); 函数体留 todo!() + /// v8: 锚
// 对数 oracle: clang -E + linemarker 切 elements.cc 自身行

import { execFileSync } from "node:child_process";
import { mkdirSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";
import { writeSrc, banner, root } from "../../lib.js";

// ── clang -E oracle (缺失 generated 头 stub 自愈) ────────────────────────
const stubDir = join(tmpdir(), "v8rs-clang-stub");
function clangE(rel, defines = []) {
  for (let i = 0; i < 40; i++) {
    try {
      return execFileSync(
        "clang",
        [
          "-E", "-x", "c++", join(root, "v8", rel), "-std=c++20",
          "-I", join(root, "v8"), "-I", join(root, "v8", "include"), "-I", stubDir,
          ...defines.map((d) => `-D${d}`),
        ],
        { encoding: "utf8", maxBuffer: 512 * 1024 * 1024, stdio: ["ignore", "pipe", "pipe"] },
      );
    } catch (e) {
      const err = String(e.stderr || e.message);
      const m = err.match(/fatal error: '([^']+)' file not found/);
      if (!m) throw new Error(`clang -E ${rel} 失败:\n${err.slice(0, 800)}`);
      mkdirSync(join(stubDir, m[1], ".."), { recursive: true });
      writeFileSync(join(stubDir, m[1]), "");
    }
  }
  throw new Error(`clang -E ${rel} stub 循环超限`);
}
function sliceSelf(iText, relSuffix) {
  const out = [];
  let cur = "";
  for (const ln of iText.split("\n")) {
    const m = ln.match(/^# \d+ "([^"]+)"/);
    if (m) {
      cur = m[1];
      continue;
    }
    if (cur.endsWith(relSuffix)) out.push(ln);
  }
  return out.join("\n");
}

let checks = 0;
const must = (cond, msg) => {
  checks++;
  if (!cond) throw new Error(`锚点失配: ${msg}`);
};

const self = sliceSelf(clangE("src/objects/elements.cc"), "src/objects/elements.cc");

// ── 1. 顶层函数清单 (token 法: "(" 前最后 token 为限定名) ────────────────
const SKIP = /^(template|using|namespace|static_assert|return|if|for|while|switch|case|__attribute__|DECLARE|CHECK|DCHECK|extern|typedef|enum|friend|public|private|protected|#)/;
const fns = [];
for (const ln of self.split("\n")) {
  if (!/^[A-Za-z_]/.test(ln)) continue;
  const bi = ln.indexOf("(");
  if (bi < 0) continue;
  if (SKIP.test(ln.trim())) continue;
  const last = ln.slice(0, bi).trim().split(/\s+/).pop();
  if (!/^[\w:]+(?:<[^()]*>)?(?:::[\w:]+)*$/.test(last)) continue;
  if (!/[a-z]/.test(last.replace(/<.*>/, ""))) continue;
  fns.push(last);
}
must(fns.length === 87, `顶层函数应 87, 实得 ${fns.length}`);
const freeFns = fns.filter((f) => !f.includes("::"));
const methodFns = fns.filter((f) => f.includes("::"));
must(freeFns.length === 20, `自由函数应 20, 实得 ${freeFns.length}`);
must(methodFns.length === 67, `方法定义应 67 (64 特化 + 3 静态), 实得 ${methodFns.length}`);
const specs = methodFns.filter((f) => f.startsWith("TypedElementsAccessor<"));
const statics = methodFns.filter((f) => f.startsWith("ElementsAccessor::"));
must(specs.length === 64, `TypedElementsAccessor 特化应 64, 实得 ${specs.length}`);
must(statics.length === 3, `ElementsAccessor 静态应 3, 实得 ${statics.length}`);
for (const n of ["ElementsAccessor::InitializeOncePerProcess", "ElementsAccessor::TearDown", "ElementsAccessor::Concat"])
  must(statics.includes(n), `缺 ${n}`);

// 与 elements.h 侧 4 个自由函数重叠 — 骨架归 elements.rs, 本模块只登记
const SHARED_WITH_H = ["ArrayConstructInitializeElements", "CopyFastNumberJSArrayElementsToTypedArray",
  "CopyTypedArrayElementsToTypedArray", "CopyTypedArrayElementsSlice"];
for (const n of SHARED_WITH_H) must(freeFns.includes(n), `elements.cc 缺 ${n}`);
const implOnlyFree = freeFns.filter((f) => !SHARED_WITH_H.includes(f));
must(implOnlyFree.length === 16, `本模块独有自由函数应 16, 实得 ${implOnlyFree.length}`);

// 特化分项 (method 名聚合)
const specByMethod = new Map();
for (const s of specs) {
  const kind = s.match(/<(.*?)>/)[1];
  const meth = s.split("::").pop();
  specByMethod.set(meth, (specByMethod.get(meth) ?? new Set()).add(kind));
}
for (const [m, kinds] of specByMethod) must(kinds.size <= 24, `特化 ${m} kind 数异常 ${kinds.size}`);

// ── 2. 访问器类清单 + 类内方法计数 (口径: class 行 → 下一 class/struct 行之间的
//        2 缩进含参声明行; 模板特化重名并入同名桶) ─────────────────────────
const lines = self.split("\n");
const classLineIdx = [];
for (let k = 0; k < lines.length; k++) {
  const cm = lines[k].match(/^(?:class|struct)\s+(\w+)/);
  if (cm) classLineIdx.push({ idx: k, name: cm[1] });
}
must(classLineIdx.length === 35, `class/struct 行应 35, 实得 ${classLineIdx.length}`);
const classMap = new Map();
for (let c = 0; c < classLineIdx.length; c++) {
  const start = classLineIdx[c].idx + 1;
  const end = c + 1 < classLineIdx.length ? classLineIdx[c + 1].idx : lines.length;
  let methods = 0;
  for (let k = start; k < end; k++) {
    const t = lines[k];
    if (/^ {2}(?:static\s+|inline\s+|virtual\s+|constexpr\s+|V8_WARN_UNUSED_RESULT\s+|typename\s+|const\s+)*[\w:<>~,&*\s]+?\b\w+\s*\(/.test(t)
      && !/^ {2}(?:if|for|while|switch|return|case|static_assert|DCHECK|CHECK|using|friend)\b/.test(t)
      && !t.trim().startsWith("//")) {
      methods++;
    }
  }
  classMap.set(classLineIdx[c].name, (classMap.get(classLineIdx[c].name) ?? 0) + methods);
}
const classes = [...classMap.entries()].map(([name, methods]) => ({ name, methods }));
must(classes.length === 32, `唯一类名应 32 (35 行 - 3 特化重名), 实得 ${classes.length}`);
for (const c of ["ElementsKindTraits", "InternalElementsAccessor", "ElementsAccessorBase",
  "DictionaryElementsAccessor", "FastElementsAccessor", "FastSmiOrObjectElementsAccessor",
  "TypedElementsAccessor", "SloppyArgumentsElementsAccessor", "StringWrapperElementsAccessor",
  "CopyBetweenBackingStoresImpl"])
  must(classes.some((x) => x.name === c), `类 ${c} 缺失`);
const totalClassMethods = classes.reduce((s, c) => s + c.methods, 0);
must(totalClassMethods > 300, `类内方法计数异常: ${totalClassMethods}`);

// ── 3. 关键实现锚 (InitializeOncePerProcess 表构建 / TearDown 全 kind 释放) ─
must(self.includes("elements_accessors_ = accessor_array;"), "InitializeOncePerProcess 表构建锚失配");
const tearDownDeletes = (self.match(/delete elements_accessors_\[/g) ?? []).length;
must(tearDownDeletes === 42, `TearDown delete 计数应 42 (= kElementsKindCount), 实得 ${tearDownDeletes}`);

// ── 生成 ────────────────────────────────────────────────────────────────
const specRows = specs.map((s) => {
  const kind = s.match(/<(.*?)>/)[1];
  const meth = s.split("::").pop();
  return `  ("${kind}", "${meth}"),`;
});
const classRows = classes.map((c) => `  ("${c.name}", ${c.methods}),`);

let rust = banner("src/objects/elements.cc.js", "src/objects/elements.cc", "elements.cc 签名骨架与语义实现; 对数 clang -E (87 顶层函数 / 35 类)");
rust += `//! elements.cc: 顶层转换/复制函数与 TypedElementsAccessor 特化清单。
//!
//! 语义实现面：
//! - [\`is_float16_raw_bits_zero\`]: float16 符号位掩码零判定；
//! - [\`is_identity_mapping\`] / [\`is_identity_mapping_slice\`]: 连续索引单次单向零分配恒等映射判定；
//! - [\`get_write_barrier_mode\`]: Smi/Double 元素的写屏障跳过决策；
//! - [\`sort_indices\`] / [\`sort_indices_slice\`]: 原位递增排序 (std::sort 对齐，Smi/HeapNumber 数值升序，Undefined 沉底)；
//! - 各类元素拷贝函数 ([\`copy_object_to_object_elements\`], [\`copy_double_to_double_elements\`],
//!   [\`copy_smi_to_double_elements\`], [\`copy_packed_smi_to_double_elements\`],
//!   [\`copy_object_to_double_elements\`], [\`copy_from_float16_to_f16\`],
//!   [\`copy_from_float16_to_f32\`], [\`copy_from_float16_to_f64\`])。
//!
//! 深水项与推迟登记项参见 \`readme/zh.md\` 推迟台账 (ELEM-ISO, ELEM-CSA, ELEM-DICT, ELEM-SLOW, ELEM-STATIC)。

use std::{cmp::Ordering, ptr};

use v8_objects_core::{
  heap_number::HeapNumber,
  instance_type_checker as itc,
  objects::{WriteBarrierMode, instance_type_of},
  smi::Smi,
  tagged::{Object, Tagged},
};

use crate::{
  elements_kind::ElementsKind,
  fixed_array::{FixedArray, TaggedArrayExt},
  primitive_array::{FixedDoubleArray, FixedDoubleArrayExt},
};

/// v8: src/objects/elements.cc 顶层自由函数 (elements.h 侧已建骨架的 4 个除外)。
pub const IMPL_ONLY_FREE_FUNCTIONS: [&str; ${implOnlyFree.length}] = [
${implOnlyFree.map((f) => `  "${f}",`).join("\n")}
];

/// v8: src/objects/elements.cc ElementsAccessor 静态方法 (3)。
pub const STATIC_METHODS: [&str; ${statics.length}] = [
${statics.map((f) => `  "${f}",`).join("\n")}
];

/// v8: src/objects/elements.cc TypedElementsAccessor 特化清单 (kind, method)。
pub const TYPED_SPECIALIZATIONS: [(&str, &str); ${specs.length}] = [
${specRows.join("\n")}
];

/// v8: src/objects/elements.cc 访问器类清单 (类名, 类内方法声明计数; 口径:
/// 预处理后类体内 2 缩进含参声明行, 多行签名计首行)。
pub const ACCESSOR_CLASSES: [(&str, usize); ${classes.length}] = [
${classRows.join("\n")}
];

// ── 语义已落地核心辅助函数 ──────────────────────────────────────────────────

/// v8: src/objects/elements.cc IsFloat16RawBitsZero
///
/// IEEE 754 规定 +0.0 与 -0.0 在比较时相等，但位模式不同 (0x0000 与 0x8000)。
/// 本函数检测去除符号位后是否为 0。
#[inline]
#[must_use]
pub const fn is_float16_raw_bits_zero(x: u16) -> bool {
  (x & !0x8000) == 0
}

/// 索引转换契约 trait (用于 is_identity_mapping)。
pub trait AsIndex {
  /// 转换为 usize 索引。
  fn as_index(&self) -> usize;
}

impl AsIndex for usize {
  #[inline]
  fn as_index(&self) -> usize {
    *self
  }
}

impl AsIndex for u8 {
  #[inline]
  fn as_index(&self) -> usize {
    *self as usize
  }
}

impl AsIndex for u16 {
  #[inline]
  fn as_index(&self) -> usize {
    *self as usize
  }
}

impl AsIndex for u32 {
  #[inline]
  fn as_index(&self) -> usize {
    *self as usize
  }
}

impl AsIndex for ElementsKind {
  #[inline]
  fn as_index(&self) -> usize {
    *self as usize
  }
}

/// v8: src/objects/elements.cc IsIdentityMapping
///
/// 检查映射表从指定索引开始是否为恒等映射 (mapping[i] == i)。
/// 纯迭代器单次遍历，提前终止，零堆分配。
#[inline]
#[must_use]
pub fn is_identity_mapping<T: AsIndex>(mapping: &[T], index: usize) -> bool {
  mapping
    .iter()
    .enumerate()
    .skip(index)
    .all(|(i, val)| val.as_index() == i)
}

/// v8: src/objects/elements.cc IsIdentityMapping (切片安全便利版)
#[inline]
#[must_use]
pub fn is_identity_mapping_slice(mapping: &[usize], index: usize) -> bool {
  is_identity_mapping(mapping, index)
}

/// v8: src/objects/elements.cc GetWriteBarrierMode
///
/// Smi 与 Double 类型元素无需写屏障 (直接使用 SkipWriteBarrier)。
/// 纯 Object 元素写操作需要更新写屏障 (UpdateWriteBarrier)。
#[inline]
#[must_use]
pub const fn get_write_barrier_mode(kind: ElementsKind) -> WriteBarrierMode {
  if kind.is_smi_elements_kind() || kind.is_double_elements_kind() {
    WriteBarrierMode::SkipWriteBarrier
  } else {
    WriteBarrierMode::UpdateWriteBarrier
  }
}

/// 提取元素索引的数值表示 (Smi 或 HeapNumber)。
/// 若为 Undefined / Null / 非数值则返回 None。
#[inline]
#[must_use]
pub fn extract_index_number(obj: Tagged<Object>) -> Option<f64> {
  if obj.is_smi() {
    Some(Smi::value(obj.cast()) as f64)
  } else if obj.is_heap_object() && instance_type_of(obj).is_some_and(itc::is_heap_number) {
    // SAFETY: 已校验为合法 HeapNumber 实例类型，内存对齐地址经 address() 获取
    let hn = unsafe { &*(obj.address() as *const HeapNumber) };
    Some(hn.value)
  } else {
    None
  }
}

/// 比较两个索引元素，对齐 V8 std::sort 的 lambda 行为：
/// 1. 若两者均为数值，按数值大小比较；
/// 2. 数值项优先于 Undefined/非数值项；
/// 3. Undefined 项沉底。
#[inline]
#[must_use]
pub fn compare_indices(a: Tagged<Object>, b: Tagged<Object>) -> Ordering {
  match (extract_index_number(a), extract_index_number(b)) {
    (Some(na), Some(nb)) => na.partial_cmp(&nb).unwrap_or(Ordering::Equal),
    (Some(_), None) => Ordering::Less,
    (None, Some(_)) => Ordering::Greater,
    (None, None) => Ordering::Equal,
  }
}

/// v8: src/objects/elements.cc SortIndices
///
/// 对 indices 数组前 \`sort_size\` 项执行原位递增排序 (std::sort 原版对齐)。
/// 排序规则与 V8 严格一致：
/// 1. Smi / HeapNumber 等数值类型按其数值大小升序排列；
/// 2. Undefined / 空位项沉底到末尾；
/// 3. 零堆分配，原位切片排序。
///
/// # Safety
/// \`indices\` 必须为有效合法的 FixedArray 实例，数据区必须已正确初始化。
pub unsafe fn sort_indices(indices: Tagged<FixedArray>, sort_size: u32) {
  if sort_size <= 1 {
    return;
  }
  let len = unsafe { indices.length() };
  let n = (sort_size.min(len)) as usize;
  if n <= 1 {
    return;
  }
  // SAFETY: 索引槽位连续且处于有效长度范围内，转换为可变切片原位排序
  unsafe {
    let slot_ptr = indices.element_slot(0) as *mut Tagged<Object>;
    let slice = std::slice::from_raw_parts_mut(slot_ptr, n);
    sort_indices_slice(slice);
  }
}

/// 对元素切片执行原位排序 (纯切片安全面)。
#[inline]
pub fn sort_indices_slice(slice: &mut [Tagged<Object>]) {
  if slice.len() <= 1 {
    return;
  }
  slice.sort_by(|&a, &b| compare_indices(a, b));
}

/// 对象元素拷贝配置参数。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CopyObjectElementsArgs {
  pub from_kind: ElementsKind,
  pub from_start: u32,
  pub to_kind: ElementsKind,
  pub to_start: u32,
  pub raw_copy_size: u32,
  pub hole_value: Tagged<Object>,
}

/// 复制至末尾并用 hole 填充的标记常量 (kMaxUInt32)。
pub const K_COPY_TO_END_AND_INITIALIZE_TO_HOLE: u32 = u32::MAX;

/// v8: src/objects/elements.cc CopyObjectToObjectElements
///
/// 将对象元素从 \`from_base\` 复制到 \`to_base\`：
/// - 支持 kCopyToEndAndInitializeToHole 模式 (自动用 \`args.hole_value\` 补齐尾部)；
/// - 根据 from_kind 与 to_kind 自动派发写屏障模式。
///
/// # Safety
/// from_base 与 to_base 必须有效且均为 FixedArray; 范围不越界。
pub unsafe fn copy_object_to_object_elements(
  from_base: Tagged<FixedArray>,
  to_base: Tagged<FixedArray>,
  args: CopyObjectElementsArgs,
) {
  let from_len = unsafe { from_base.length() };
  let to_len = unsafe { to_base.length() };
  debug_assert!(args.from_start <= from_len);
  debug_assert!(args.to_start <= to_len);

  let mut copy_size = args.raw_copy_size;
  if args.raw_copy_size == K_COPY_TO_END_AND_INITIALIZE_TO_HOLE {
    copy_size = (from_len.saturating_sub(args.from_start)).min(to_len.saturating_sub(args.to_start));
    let start = args.to_start + copy_size;
    if start < to_len {
      unsafe {
        to_base.fill(start, to_len, args.hole_value, WriteBarrierMode::SkipWriteBarrier);
      }
    }
  }

  if copy_size == 0 {
    return;
  }
  debug_assert!(copy_size + args.to_start <= to_len);
  debug_assert!(copy_size + args.from_start <= from_len);

  let mode = if args.from_kind.is_object_elements_kind() && args.to_kind.is_object_elements_kind() {
    WriteBarrierMode::UpdateWriteBarrier
  } else {
    WriteBarrierMode::SkipWriteBarrier
  };

  unsafe {
    TaggedArrayExt::copy_elements(&to_base, args.to_start, &from_base, args.from_start, copy_size, mode);
  }
}

/// v8: src/objects/elements.cc CopyDoubleToDoubleElements
///
/// 将双精度浮点数组元素从 \`from_base\` 复制到 \`to_base\`：
/// - 支持 kCopyToEndAndInitializeToHole 模式 (自动将剩余项设为 Hole)；
/// - 支持同数组内存重叠自搬迁 (memmove 保源序)。
///
/// # Safety
/// from_base 与 to_base 必须有效且为 FixedDoubleArray。
pub unsafe fn copy_double_to_double_elements(
  from_base: Tagged<FixedDoubleArray>,
  from_start: u32,
  to_base: Tagged<FixedDoubleArray>,
  to_start: u32,
  raw_copy_size: u32,
) {
  let from_len = from_base.length();
  let to_len = to_base.length();
  debug_assert!(from_start <= from_len);
  debug_assert!(to_start <= to_len);

  let mut copy_size = raw_copy_size;
  if raw_copy_size == K_COPY_TO_END_AND_INITIALIZE_TO_HOLE {
    copy_size = (from_len.saturating_sub(from_start)).min(to_len.saturating_sub(to_start));
    let start = to_start + copy_size;
    if start < to_len {
      unsafe {
        to_base.fill_with_holes(start, to_len);
      }
    }
  }

  if copy_size == 0 {
    return;
  }
  debug_assert!(copy_size + to_start <= to_len);
  debug_assert!(copy_size + from_start <= from_len);

  if from_base.ptr() == to_base.ptr() {
    unsafe {
      to_base.move_elements(to_start, from_start, copy_size);
    }
  } else {
    unsafe {
      let src_slice = from_base.as_f64_slice();
      let mut to_copy = to_base;
      let dst_slice = to_copy.as_f64_slice_mut();
      let src = &src_slice[from_start as usize..(from_start + copy_size) as usize];
      let dst = &mut dst_slice[to_start as usize..(to_start + copy_size) as usize];
      dst.copy_from_slice(src);
    }
  }
}

/// v8: src/objects/elements.cc CopySmiToDoubleElements
///
/// 将 Smi 元素从 FixedArray 转换为 f64 并写入 FixedDoubleArray。
///
/// # Safety
/// from_base 必须为有效 FixedArray; to_base 必须为有效 FixedDoubleArray。
pub unsafe fn copy_smi_to_double_elements(
  from_base: Tagged<FixedArray>,
  from_start: u32,
  to_base: Tagged<FixedDoubleArray>,
  to_start: u32,
  raw_copy_size: u32,
  the_hole: Tagged<Object>,
) {
  let from_len = unsafe { from_base.length() };
  let to_len = to_base.length();
  debug_assert!(from_start <= from_len);
  debug_assert!(to_start <= to_len);

  let mut copy_size = raw_copy_size;
  if raw_copy_size == K_COPY_TO_END_AND_INITIALIZE_TO_HOLE {
    copy_size = from_len.saturating_sub(from_start);
    let start = to_start + copy_size;
    if start < to_len {
      unsafe {
        to_base.fill_with_holes(start, to_len);
      }
    }
  }

  if copy_size == 0 {
    return;
  }
  debug_assert!(copy_size + to_start <= to_len);
  debug_assert!(copy_size + from_start <= from_len);

  unsafe {
    for i in 0..copy_size {
      let hole_or_smi = from_base.get(from_start + i);
      if hole_or_smi == the_hole || !hole_or_smi.is_smi() {
        to_base.set_the_hole(to_start + i);
      } else {
        to_base.set(to_start + i, Smi::value(hole_or_smi.cast()) as f64);
      }
    }
  }
}

/// v8: src/objects/elements.cc CopyPackedSmiToDoubleElements
///
/// 将连续紧凑且无 Hole 的 Smi 元素快速转换为 f64 并写入 FixedDoubleArray。
///
/// # Safety
/// from_base 必须为有效 FixedArray; to_base 必须为有效 FixedDoubleArray。
pub unsafe fn copy_packed_smi_to_double_elements(
  from_base: Tagged<FixedArray>,
  from_start: u32,
  to_base: Tagged<FixedDoubleArray>,
  to_start: u32,
  packed_size: u32,
  raw_copy_size: u32,
) {
  let from_len = unsafe { from_base.length() };
  let to_len = to_base.length();
  debug_assert!(from_start <= from_len);
  debug_assert!(to_start <= to_len);
  debug_assert!(from_start <= packed_size);

  let mut copy_size = raw_copy_size;
  if raw_copy_size == K_COPY_TO_END_AND_INITIALIZE_TO_HOLE {
    copy_size = packed_size.saturating_sub(from_start);
    let start = to_start + copy_size;
    if start < to_len {
      unsafe {
        to_base.fill_with_holes(start, to_len);
      }
    }
  }

  if copy_size == 0 {
    return;
  }
  debug_assert!(copy_size + to_start <= to_len);
  debug_assert!(copy_size + from_start <= from_len);

  let num_packed = packed_size.saturating_sub(from_start).min(copy_size);
  unsafe {
    for i in 0..num_packed {
      let smi = from_base.get(from_start + i);
      debug_assert!(smi.is_smi());
      to_base.set(to_start + i, Smi::value(smi.cast()) as f64);
    }
  }
}

/// v8: src/objects/elements.cc CopyObjectToDoubleElements
///
/// 将 FixedArray 中的 Number (Smi 或 HeapNumber) 转换为 f64 写入 FixedDoubleArray，
/// 若遇到 Hole 或非数值项则写入 Hole。
///
/// # Safety
/// from_base 必须为有效 FixedArray; to_base 必须为有效 FixedDoubleArray。
pub unsafe fn copy_object_to_double_elements(
  from_base: Tagged<FixedArray>,
  from_start: u32,
  to_base: Tagged<FixedDoubleArray>,
  to_start: u32,
  raw_copy_size: u32,
  the_hole: Tagged<Object>,
) {
  let from_len = unsafe { from_base.length() };
  let to_len = to_base.length();
  debug_assert!(from_start <= from_len);
  debug_assert!(to_start <= to_len);

  let mut copy_size = raw_copy_size;
  if raw_copy_size == K_COPY_TO_END_AND_INITIALIZE_TO_HOLE {
    copy_size = from_len.saturating_sub(from_start);
    let start = to_start + copy_size;
    if start < to_len {
      unsafe {
        to_base.fill_with_holes(start, to_len);
      }
    }
  }

  if copy_size == 0 {
    return;
  }
  debug_assert!(copy_size + to_start <= to_len);
  debug_assert!(copy_size + from_start <= from_len);

  unsafe {
    for i in 0..copy_size {
      let hole_or_object = from_base.get(from_start + i);
      if hole_or_object == the_hole {
        to_base.set_the_hole(to_start + i);
      } else if let Some(val) = extract_index_number(hole_or_object) {
        to_base.set(to_start + i, val);
      } else {
        to_base.set_the_hole(to_start + i);
      }
    }
  }
}

/// v8: src/objects/elements.cc CopyFromFloat16BackingStore
///
/// 拷贝 float16 底层存储至目标缓冲区 (保持原始比特位)。
///
/// # Safety
/// src 与 dst 必须有效可访问且长度至少为 length。
pub unsafe fn copy_from_float16_to_f16(src: *const u16, dst: *mut u16, length: usize) {
  unsafe {
    ptr::copy(src, dst, length);
  }
}

/// v8: src/objects/elements.cc CopyFromFloat16BackingStore
///
/// 拷贝 float16 底层存储至目标 f32 缓冲区 (经 fp16_ieee_to_fp32 转换)。
///
/// # Safety
/// src 与 dst 必须有效可访问且长度至少为 length。
pub unsafe fn copy_from_float16_to_f32(src: *const u16, dst: *mut f32, length: usize) {
  unsafe {
    for i in 0..length {
      let raw = *src.add(i);
      *dst.add(i) = v8_base::float16::Float16::from_bits(raw).to_float32();
    }
  }
}

/// v8: src/objects/elements.cc CopyFromFloat16BackingStore
///
/// 拷贝 float16 底层存储至目标 f64 缓冲区 (经 fp16_ieee_to_fp32 转换为 f64)。
///
/// # Safety
/// src 与 dst 必须有效可访问且长度至少为 length。
pub unsafe fn copy_from_float16_to_f64(src: *const u16, dst: *mut f64, length: usize) {
  unsafe {
    for i in 0..length {
      let raw = *src.add(i);
      *dst.add(i) = v8_base::float16::Float16::from_bits(raw).to_float32() as f64;
    }
  }
}

/// 切片安全封装：拷贝 float16 切片并转为 f32。
#[inline]
pub fn copy_from_float16_slice_to_f32(src: &[u16], dst: &mut [f32]) {
  let len = src.len().min(dst.len());
  unsafe {
    copy_from_float16_to_f32(src.as_ptr(), dst.as_mut_ptr(), len);
  }
}

/// 切片安全封装：拷贝 float16 切片并转为 f64。
#[inline]
pub fn copy_from_float16_slice_to_f64(src: &[u16], dst: &mut [f64]) {
  let len = src.len().min(dst.len());
  unsafe {
    copy_from_float16_to_f64(src.as_ptr(), dst.as_mut_ptr(), len);
  }
}

// ── 深水推迟函数 (台账登记) ────────────────────────────────────────────────

/// v8: src/objects/elements.cc ThrowArrayLengthRangeError
pub fn throw_array_length_range_error() {
  todo!("readme 台账 ELEM-ISO: 需 Isolate + MessageTemplate + RangeError 异常体系");
}

/// v8: src/objects/elements.cc CopyDictionaryToObjectElements
pub fn copy_dictionary_to_object_elements() {
  todo!("readme 台账 ELEM-DICT: 需 NumberDictionary::FindEntry 字典哈希表查找与 Isolate");
}

/// v8: src/objects/elements.cc CopyDoubleToObjectElements
pub fn copy_double_to_object_elements() {
  todo!("readme 台账 ELEM-ISO: 需 Isolate + HandleScope 分配 HeapNumber 对象与写屏障");
}

/// v8: src/objects/elements.cc CopyDictionaryToDoubleElements
pub fn copy_dictionary_to_double_elements() {
  todo!("readme 台账 ELEM-DICT: 需 NumberDictionary::FindEntry 字典哈希表查找与 Isolate");
}

/// v8: src/objects/elements.cc IncludesValueSlowPath
pub fn includes_value_slow_path() {
  todo!("readme 台账 ELEM-SLOW: 需 DirectHandle<JSObject> 原型链遍历与 Isolate 句柄体系");
}

/// v8: src/objects/elements.cc IndexOfValueSlowPath
pub fn index_of_value_slow_path() {
  todo!("readme 台账 ELEM-SLOW: 需 DirectHandle<JSObject> 原型链遍历与 Isolate 句柄体系");
}

/// v8: src/objects/elements.cc ElementsAccessor::InitializeOncePerProcess
pub fn elements_accessor_initialize_once_per_process() {
  todo!("readme 台账 ELEM-STATIC: 需完整 32 个 ElementsAccessor 子类层次表全局初始化");
}

/// v8: src/objects/elements.cc ElementsAccessor::TearDown
pub fn elements_accessor_tear_down() {
  todo!("readme 台账 ELEM-STATIC: 需完整 ElementsAccessor 实例生命周期管理与全局静态清理");
}

/// v8: src/objects/elements.cc ElementsAccessor::Concat
pub fn elements_accessor_concat() {
  todo!("readme 台账 ELEM-ISO: 需 Isolate 与 JSArray 堆分配及原型链查找");
}

// ── 向后兼容别名 ────────────────────────────────────────────────────────────
pub use copy_dictionary_to_double_elements as copydictionarytodoubleelements;
pub use copy_dictionary_to_object_elements as copydictionarytoobjectelements;
pub use copy_double_to_double_elements as copydoubletodoubleelements;
pub use copy_double_to_object_elements as copydoubletoobjectelements;
pub use copy_from_float16_to_f32 as copyfromfloat16backingstore;
pub use copy_object_to_double_elements as copyobjecttodoubleelements;
pub use copy_object_to_object_elements as copyobjecttoobjectelements;
pub use copy_packed_smi_to_double_elements as copypackedsmitodoubleelements;
pub use copy_smi_to_double_elements as copysmitodoubleelements;
pub use elements_accessor_concat as elementsaccessor_concat;
pub use elements_accessor_initialize_once_per_process as elementsaccessor_initializeonceperprocess;
pub use elements_accessor_tear_down as elementsaccessor_teardown;
pub use get_write_barrier_mode as getwritebarriermode;
pub use includes_value_slow_path as includesvalueslowpath;
pub use index_of_value_slow_path as indexofvalueslowpath;
pub use is_float16_raw_bits_zero as isfloat16rawbitszero;
pub use is_identity_mapping_slice as isidentitymapping;
pub use sort_indices as sortindices;
pub use throw_array_length_range_error as throwarraylengthrangeerror;

// —— 提取不变式断言 (clang -E 对数; 分项计数生成侧锁定后烧入) ——
const _: () = {
  assert!(IMPL_ONLY_FREE_FUNCTIONS.len() == ${implOnlyFree.length});
  assert!(STATIC_METHODS.len() == 3);
  assert!(TYPED_SPECIALIZATIONS.len() == ${specs.length});
  assert!(ACCESSOR_CLASSES.len() == ${classes.length});
  assert!(TYPED_SPECIALIZATIONS.len() + STATIC_METHODS.len() + IMPL_ONLY_FREE_FUNCTIONS.len() + 4 == ${fns.length});
  assert!(TYPED_SPECIALIZATIONS.len() == 24 + 36 + 4);
};
`;

const p = writeSrc("v8-objects-arrays", "elements_impl.rs", rust);
console.log(
  `elements_impl.rs: 顶层函数 ${fns.length} (自由 ${freeFns.length} = 本模块 ${implOnlyFree.length} + elements.h 侧 4; ` +
    `静态 ${statics.length})、Typed 特化 ${specs.length}、类 ${classes.length} (类内方法计数合计 ${totalClassMethods}); 断言 ${checks} 项通过`,
);

