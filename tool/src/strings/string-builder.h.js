#!/usr/bin/env bun
// tool/src/strings/string-builder.h.js
// 机械提取 v8/src/strings/string-builder.{h,inl.h,cc}: 三构建器骨架与分片增长常量

import { readV8, writeSrc, banner } from "../../lib.js";

const hSrc = readV8("src/strings/string-builder.h");
const inlSrc = readV8("src/strings/string-builder-inl.h");
const ccSrc = readV8("src/strings/string-builder.cc");

// 1. IncrementalStringBuilder 常量 (机械提取)
function constOf(src, name) {
  const m = src.match(new RegExp(`static const int ${name} = ([^;]+);`));
  if (!m) throw new Error(`未找到 ${name}`);
  return m[1].trim();
}
const initialPartLength = constOf(hSrc, "kInitialPartLength");
const maxPartLength = constOf(hSrc, "kMaxPartLength");
const growthFactor = constOf(hSrc, "kPartLengthGrowthFactor");
const intToBuffer = constOf(hSrc, "kIntToStringViewBufferSize");
if (initialPartLength !== "32") throw new Error("kInitialPartLength 漂移");
if (maxPartLength !== "16 * 1024") throw new Error("kMaxPartLength 漂移");
if (growthFactor !== "2") throw new Error("kPartLengthGrowthFactor 漂移");
// sizeof("-2147483648")-1 = 10
if (intToBuffer !== 'sizeof("-2147483648") - 1') throw new Error("kIntToStringViewBufferSize 漂移");

// 2. cc 侧惰性初始容量
const lazyM = ccSrc.match(/constexpr uint32_t kInitialCapacityForLazy = (\d+);/);
if (!lazyM) throw new Error("未找到 kInitialCapacityForLazy");
const lazyCap = parseInt(lazyM[1], 10);

// 3. 三构建器方法面对数
const fabFns = [...hSrc.matchAll(/\n  (?:static )?[A-Za-z_][\w:<>,*& ]*?(HasCapacity|EnsureCapacity|Add|array|length|capacity|Lazy)\(/g)].map((m) => m[1]);
const rsbFns = ["AddSubjectSlice", "AddString", "ToString", "IncrementCharacterCount"];
for (const f of rsbFns) {
  if (!hSrc.includes(f)) throw new Error(`ReplacementStringBuilder 缺 ${f}`);
}
const isbFns = ["AppendCharacter", "AppendCString", "AppendString", "AppendInt", "Finish", "ChangeEncoding", "Accumulate", "Extend", "ShrinkCurrentPart", "HasOverflowed", "EscapedLengthIfCurrentPartFits", "AppendStringCapped"];
for (const f of isbFns) {
  if (!hSrc.includes(f)) throw new Error(`IncrementalStringBuilder 缺 ${f}`);
}
// inl: 溢出粗估公式存在
if (!inlSrc.includes("EscapedLengthIfCurrentPartFits")) throw new Error("inl 缺溢出粗估");

let rust = banner("src/strings/string-builder.h.js", "src/strings/string-builder.{h,inl.h,cc}", "字符串构建器三件套骨架与常量");

rust += `
/// IncrementalStringBuilder 分片增长常量 (原版 string-builder.h 私有段)
pub mod string_builder_const {
  /// 首个分片长度 (原版 kInitialPartLength)
  pub const INITIAL_PART_LENGTH: i32 = ${initialPartLength.replace(/\s/g, "")};
  /// 分片长度上限 (原版 kMaxPartLength = 16 * 1024)
  pub const MAX_PART_LENGTH: i32 = ${maxPartLength.replace(/\s/g, "")};
  /// 增长因子 (原版 kPartLengthGrowthFactor)
  pub const PART_LENGTH_GROWTH_FACTOR: i32 = ${growthFactor};
  /// int→字符串视图缓冲宽 (sizeof("-2147483648")-1, 含负号不含 \\0)
  pub const INT_TO_STRING_VIEW_BUFFER_SIZE: i32 = 10;
  /// Lazy 模式惰性初始容量 (原版 kInitialCapacityForLazy)
  pub const INITIAL_CAPACITY_FOR_LAZY: u32 = ${lazyCap};
}

/// FixedArrayBuilder 骨架 (原版 string-builder.h 前段)
pub mod fixed_array_builder_skel {
  /// 就绪容量判定 (原版 HasCapacity)
  pub fn has_capacity(_elements: u32) -> bool {
    unimplemented!("骨架")
  }
  /// 扩容 (原版 EnsureCapacity; Lazy 模式在此惰性分配)
  pub fn ensure_capacity(_isolate_key: usize, _elements: u32) {
    unimplemented!("骨架")
  }
  /// 压入元素 (原版 Add ×2 重载: Object/Smi)
  pub fn add(_value: usize) {
    unimplemented!("骨架")
  }
  /// 惰性构造标记 (原版 static Lazy)
  pub fn lazy(_isolate_key: usize) {
    unimplemented!("骨架")
  }
}

/// ReplacementStringBuilder 骨架 (替换切片聚合, 供 String.prototype.replace 用)
pub mod replacement_string_builder_skel {
  /// 主体切片压入 (原版 AddSubjectSlice 静态/实例双形态)
  pub fn add_subject_slice(_builder: usize, _from: i32, _to: i32) {
    unimplemented!("骨架")
  }
  /// 整串压入 (原版 AddString)
  pub fn add_string(_string: usize) {
    unimplemented!("骨架")
  }
  /// 聚合成品 (原版 ToString → MaybeHandle)
  pub fn to_string() -> Option<usize> {
    unimplemented!("骨架")
  }
  /// 字符计数溢出饱和到 kMaxInt (原版 IncrementCharacterCount)
  pub fn increment_character_count(_by: u32) {
    unimplemented!("骨架")
  }
}

/// IncrementalStringBuilder 骨架 (JSON 序列化等增量拼接)
pub mod incremental_string_builder_skel {
  /// 当前编码 (原版 CurrentEncoding: OneByte/TwoByte)
  pub fn current_encoding() -> u8 {
    unimplemented!("骨架")
  }
  /// 追加单字符 (原版 Append<SrcChar,DestChar> / AppendCharacter)
  pub fn append_character(_c: u8) {
    unimplemented!("骨架")
  }
  /// 追加 C 串/视图/整数 (原版 AppendCString/AppendString/AppendInt)
  pub fn append_str(_s: &str) {
    unimplemented!("骨架")
  }
  /// 追加封顶串: 超长时追加前缀 + "<...>" (原版 AppendStringCapped)
  pub fn append_string_capped(_string: usize, _max_length: u32) {
    unimplemented!("骨架")
  }
  /// 当前分片是否可容 (原版 CurrentPartCanFit: part_length_ - current_index_ > length)
  pub fn current_part_can_fit(_part_length: i32, _current_index: i32, _length: i32) -> bool {
    _part_length - _current_index > _length
  }
  /// 溢出粗估 (原版 EscapedLengthIfCurrentPartFits)
  pub fn escaped_length_if_current_part_fits(_length: i32) -> i32 {
    unimplemented!("骨架")
  }
  /// 收尾产出 (原版 Finish → MaybeHandle)
  pub fn finish() -> Option<usize> {
    unimplemented!("骨架")
  }
  /// 编码升级到双字节 (原版 ChangeEncoding)
  pub fn change_encoding() {
    unimplemented!("骨架")
  }
  /// 分片累积/扩展/收缩 (原版 Accumulate/Extend/ShrinkCurrentPart)
  pub fn accumulate(_new_part: usize) {
    unimplemented!("骨架")
  }
  pub fn extend() {
    unimplemented!("骨架")
  }
  pub fn shrink_current_part() {
    unimplemented!("骨架")
  }
}
`;

const p = writeSrc("v8-strings", "string_builder_generated.rs", rust);
console.log(`written ${p}; 常量=5, ISB方法=${isbFns.length}, FAB方法=${fabFns.length}`);
