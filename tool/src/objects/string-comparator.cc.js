#!/usr/bin/env bun
// tool/src/objects/string-comparator.cc.js — 覆盖 string-comparator 同名族:
//   v8/src/objects/string-comparator.h + string-comparator.cc
//   → v8-objects-strings/src/string_comparator.rs
//
// 映射登记 (C++ → Rust):
//   State { is_one_byte_/length_/buffer8|16 union } → State { is_one_byte,
//     length, buffer: Option<FlatSegment> }
//   FlatSegment (union 裸指针) → 裸指针 + 长度 的 Copy 结构
//     (借用切片形态受 lending-iterator 生命周期约束无法落 State —
//      段源 next_segment 返回借用段后再借自身即冲突; 原版即裸指针,
//      SAFETY 契约 = 原版 DisallowGarbageCollection: 比较期段内存有效)
//   ConsStringIterator (string.h, 归 OS-1 string.rs 槽位) → trait
//     SegmentSource::next_segment —— ConsString 遍历绑定推迟, 本模块落地
//     "逐段取扁平段" 的状态机核心 (Init/Advance 语义 1:1)
//   VisitOneByteString/VisitTwoByteString → FlatSegment::one_byte/two_byte
//   Equals<Chars1,Chars2> 4 路分派 → 宽化 u32 逐值比较 (CompareCharsEqual;
//     向量化交给编译器)
//   String::VisitFlat → SegmentSource 首段 (由调用方适配)
//
// 跳过区段登记 (推迟):
//   顶层 StringComparator::Equals(Tagged<String>, Tagged<String>) 对象入参:
//     String 对象访问归 OS-1 → 本模块暴露 equals(sources..., length)
//   SharedStringAccessGuardIfNeeded: 共享堆访问守卫归 L3 heap → 省略
import { banner, readV8, writeSrc } from "../../lib.js";

const h = readV8("src/objects/string-comparator.h");
const cc = readV8("src/objects/string-comparator.cc");

const anchors = [
  [h, "void VisitOneByteString(const uint8_t* chars, int length) {", "h"],
  [h, "static inline bool Equals(State* state_1, State* state_2, int to_check) {", "h"],
  [h, "ConsStringIterator iter_;", "h"],
  [cc, "int to_check = std::min(state_1_.length_, state_2_.length_);", "cc"],
  [cc, "state_1_.Advance(to_check, access_guard);", "cc"],
];
for (const [s, sub, f] of anchors) {
  if (!s.includes(sub)) throw new Error(`锚点失配: ${f} 无 "${sub.slice(0, 52)}..."`);
}

const body = `${banner("src/objects/string-comparator.cc.js", "src/objects/string-comparator.{h,cc}", "逐字符分段比较状态机 1:1; 段以裸指针形态承接 (原版 union 裸指针同款), ConsStringIterator 绑定 trait 化 (登记见脚本头)")}
//! 字符串逐字符比较状态机: 分段拉取扁平字符段, 跨段推进比对。
//!
//! 用于 ConsString 等非扁平串的等值比较 (避免整串扁平化拷贝)。
//! 段源抽象 (SegmentSource) 对应原版 ConsStringIterator::Next +
//! String::VisitFlat 组合; String 对象绑定归 OS-1, 登记见脚本头。

/// 扁平字符段 (VisitOneByteString/VisitTwoByteString 的数据形态)。
///
/// 裸指针 + 长度 (原版 \`const uint8_t*/uint16_t*\` union 的等价物;
/// 借用切片形态受 lending-iterator 生命周期约束无法落 State, 原版即裸指针)。
///
/// # SAFETY 契约 (构造方)
/// 段内存须在比较期间保持有效且不可变 (原版 DisallowGarbageCollection
/// 同款契约: 比较期字符串不可被 GC 移动或转型)。
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct FlatSegment {
  data: SegData,
  len: u32,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum SegData {
  One(*const u8),
  Two(*const u16),
}

impl FlatSegment {
  /// 以单字节切片构造 (安全入口: 借用转裸指针, 契约见类型文档)。
  #[inline]
  #[must_use]
  pub const fn one_byte(chars: &[u8]) -> Self {
    Self {
      data: SegData::One(chars.as_ptr()),
      len: chars.len() as u32,
    }
  }

  /// 以双字节切片构造。
  #[inline]
  #[must_use]
  pub const fn two_byte(chars: &[u16]) -> Self {
    Self {
      data: SegData::Two(chars.as_ptr()),
      len: chars.len() as u32,
    }
  }

  /// 段字符数。
  #[inline]
  #[must_use]
  pub const fn len(&self) -> u32 {
    self.len
  }

  /// 是否单字节编码。
  #[inline]
  #[must_use]
  pub const fn is_one_byte(&self) -> bool {
    matches!(self.data, SegData::One(_))
  }

  /// 是否空段 (零长段不承载字符, 状态机会跳过)。
  #[inline]
  #[must_use]
  pub const fn is_empty(&self) -> bool {
    self.len == 0
  }

  /// 读单字节前 n 字符。
  ///
  /// # Panics (debug)
  /// 非单字节段。
  #[inline]
  fn as_one_byte(&self, n: usize) -> &[u8] {
    match self.data {
      SegData::One(ptr) => {
        debug_assert!(n <= self.len as usize);
        // SAFETY: 构造契约保证段内存有效; n 在段长内
        unsafe { std::slice::from_raw_parts(ptr, n) }
      }
      SegData::Two(_) => {
        debug_assert!(false, "单字节读取不可用于双字节段");
        &[]
      }
    }
  }

  /// 读双字节前 n 字符。
  #[inline]
  fn as_two_byte(&self, n: usize) -> &[u16] {
    match self.data {
      SegData::Two(ptr) => {
        debug_assert!(n <= self.len as usize);
        // SAFETY: 构造契约保证段内存有效; n 在段长内
        unsafe { std::slice::from_raw_parts(ptr, n) }
      }
      SegData::One(_) => {
        debug_assert!(false, "双字节读取不可用于单字节段");
        &[]
      }
    }
  }

  /// 段内前移 consumed 字符 (指针累加的原版 buffer8_ += consumed)。
  #[inline]
  const fn advance_in_place(mut self, consumed: u32) -> Self {
    self.data = match self.data {
      SegData::One(ptr) => {
        // SAFETY: consumed 在段长内 (调用方 debug_assert); 裸指针偏移不解引用
        SegData::One(unsafe { ptr.add(consumed as usize) })
      }
      SegData::Two(ptr) => SegData::Two(unsafe { ptr.add(consumed as usize) }),
    };
    self.len -= consumed;
    self
  }
}

impl std::fmt::Debug for FlatSegment {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.debug_struct("FlatSegment")
      .field("one_byte", &self.is_one_byte())
      .field("len", &self.len)
      .finish()
  }
}

/// 段源: 依序产出扁平字符段 (ConsStringIterator::Next 的抽象;
/// String 对象绑定推迟, 登记见脚本头)。
pub trait SegmentSource {
  /// 下一段; None 表示段耗尽。
  fn next_segment(&mut self) -> Option<FlatSegment>;
}

/// 单段源 (扁平串适配): 整串一段, 之后耗尽。
#[derive(Debug)]
pub struct SingleSegmentSource {
  segment: Option<FlatSegment>,
}

impl SingleSegmentSource {
  /// 以单段构造。
  #[must_use]
  pub const fn new(segment: FlatSegment) -> Self {
    Self {
      segment: Some(segment),
    }
  }
}

impl SegmentSource for SingleSegmentSource {
  fn next_segment(&mut self) -> Option<FlatSegment> {
    self.segment.take()
  }
}

/// 多段源 (段序列; 测试与 ConsString 适配用)。
#[derive(Debug)]
pub struct MultiSegmentSource {
  segments: Vec<FlatSegment>,
  next: usize,
}

impl MultiSegmentSource {
  /// 以段列表构造。
  #[must_use]
  pub fn new(segments: Vec<FlatSegment>) -> Self {
    Self { segments, next: 0 }
  }
}

impl SegmentSource for MultiSegmentSource {
  fn next_segment(&mut self) -> Option<FlatSegment> {
    let seg = self.segments.get(self.next).copied();
    if seg.is_some() {
      self.next += 1;
    }
    seg
  }
}

/// 比较状态 (string-comparator.h:17 State)。
///
/// \`buffer: Option<FlatSegment>\` 对应原版 union 裸指针缓冲 +
/// is_one_byte_/length_ 三元组。
#[derive(Clone, Copy, Debug)]
struct State {
  is_one_byte: bool,
  length: u32,
  buffer: Option<FlatSegment>,
}

impl State {
  /// 初值 (State 构造: is_one_byte_ = true, length_ = 0)。
  const fn new() -> Self {
    Self {
      is_one_byte: true,
      length: 0,
      buffer: None,
    }
  }

  /// 初始化: 拉首个非空段 (cc:13 State::Init = VisitFlat + iter 首段;
  /// 零长段不承载字符, 跳过 — 原版不会对空串进入比较器)。
  fn init(&mut self, source: &mut dyn SegmentSource) {
    if let Some(segment) = next_non_empty(source) {
      self.visit(segment);
    }
  }

  /// 载入段 (VisitOneByteString / VisitTwoByteString)。
  fn visit(&mut self, segment: FlatSegment) {
    self.is_one_byte = segment.is_one_byte();
    self.length = segment.len();
    self.buffer = Some(segment);
  }

  /// 推进 consumed 个字符 (cc:29 State::Advance)。
  ///
  /// 段内剩余 → 指针前移; 段耗尽 → 拉下一段。
  fn advance(&mut self, consumed: u32, source: &mut dyn SegmentSource) {
    debug_assert!(consumed <= self.length);
    if self.length != consumed {
      // 仍在缓冲段内: 段内前移 (原版 buffer += consumed)
      self.buffer = self.buffer.map(|b| b.advance_in_place(consumed));
      self.length -= consumed;
      return;
    }
    // 段耗尽: 拉下一个非空段 (iter_.Next + VisitFlat)
    if let Some(segment) = next_non_empty(source) {
      self.visit(segment);
    } else {
      // 原版 DCHECK(!next.is_null()): 两串等长且比对到尾, 理论不可达;
      // 状态置空, 后续由 equals 的 None 分支兜底 (禁 panic)
      self.buffer = None;
      self.length = 0;
    }
  }
}

/// 拉取下一个非零长段 (零长段不承载字符)。
fn next_non_empty(source: &mut dyn SegmentSource) -> Option<FlatSegment> {
  loop {
    match source.next_segment() {
      Some(segment) if !segment.is_empty() => return Some(segment),
      Some(_) => continue,
      None => return None,
    }
  }
}

/// 双串比较器 (string-comparator.h:16)。
///
/// 原版持两个可复用 State; Rust 侧状态依附比较调用构造
/// (语义等价: 原版 State 亦每次 Equals 重新 Init)。
#[derive(Default)]
pub struct StringComparator;

impl StringComparator {
  /// 等值比较 (cc:50 Equals): 分段推进, 任一段不等 → false, 全长比对完 → true。
  ///
  /// 前置: 两源总字符数均为 length (原版由调用方 String::Equals 长度预检)。
  pub fn equals(
    &mut self,
    source_1: &mut dyn SegmentSource,
    source_2: &mut dyn SegmentSource,
    length: u32,
  ) -> bool {
    let mut state_1 = State::new();
    let mut state_2 = State::new();
    state_1.init(source_1);
    state_2.init(source_2);

    let mut remaining = length;
    loop {
      let (a, b) = match (state_1.buffer, state_2.buffer) {
        (Some(a), Some(b)) => (a, b),
        // 双侧同步耗尽: 剩余为 0 即相等 (空串), 否则契约破坏判不等
        (None, None) => return remaining == 0,
        // 单侧提前耗尽: 前置契约破坏, 判不等 (禁 panic; 原版 DCHECK 兜底)
        (None, _) | (_, None) => return false,
      };
      let to_check = state_1.length.min(state_2.length);
      debug_assert!(to_check > 0 && to_check <= length);
      let n = to_check as usize;
      // 4 路编码分派 (cc:60-71); 混编宽化至 u32 逐值比较 (CompareCharsEqual)
      let is_equal = match (a.is_one_byte(), b.is_one_byte()) {
        (true, true) => a.as_one_byte(n) == b.as_one_byte(n),
        (true, false) => a
          .as_one_byte(n)
          .iter()
          .zip(b.as_two_byte(n))
          .all(|(&x, &y)| u32::from(x) == u32::from(y)),
        (false, true) => a
          .as_two_byte(n)
          .iter()
          .zip(b.as_one_byte(n))
          .all(|(&x, &y)| u32::from(x) == u32::from(y)),
        (false, false) => a.as_two_byte(n) == b.as_two_byte(n),
      };
      // 不等即出
      if !is_equal {
        return false;
      }
      remaining -= to_check;
      // 全长比对完: 相等
      if remaining == 0 {
        return true;
      }
      state_1.advance(to_check, source_1);
      state_2.advance(to_check, source_2);
    }
  }
}
`;

writeSrc("v8-objects-strings", "string_comparator.rs", body);
console.log("string_comparator.rs: State/Init/Advance 状态机 + 4 路分派 1:1; 段源 trait 化登记");
