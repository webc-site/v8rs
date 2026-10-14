#!/usr/bin/env bun
// tool/src/objects/tagged.h.js — 提取 Tag 掩码与偏移量常量
// 来源: v8/include/v8-internal.h 与 v8/src/common/globals.h
// 产物: v8-objects-core/src/tagged_constants.rs
import { banner, readV8, writeSrc } from "../../lib.js";

const internalHeader = readV8("include/v8-internal.h");

// 提取 Tag 常量
const matchNum = (regex, name) => {
  const m = internalHeader.match(regex);
  if (!m) throw new Error(`无法从 v8-internal.h 匹配到常量 ${name}`);
  return parseInt(m[1], 10);
};

const kHeapObjectTag = matchNum(/const int kHeapObjectTag = (\d+);/, "kHeapObjectTag");
const kWeakHeapObjectTag = matchNum(/const int kWeakHeapObjectTag = (\d+);/, "kWeakHeapObjectTag");
const kHeapObjectTagSize = matchNum(/const int kHeapObjectTagSize = (\d+);/, "kHeapObjectTagSize");
const kHeapObjectTagMask = (1 << kHeapObjectTagSize) - 1;
const kHeapObjectReferenceTagMask = 1 << (kHeapObjectTagSize - 1);

const kForwardingTag = matchNum(/const int kForwardingTag = (\d+);/, "kForwardingTag");
const kForwardingTagSize = matchNum(/const int kForwardingTagSize = (\d+);/, "kForwardingTagSize");
const kForwardingTagMask = (1 << kForwardingTagSize) - 1;

const kSmiTag = matchNum(/const int kSmiTag = (\d+);/, "kSmiTag");
const kSmiTagSize = matchNum(/const int kSmiTagSize = (\d+);/, "kSmiTagSize");
const kSmiTagMask = (1 << kSmiTagSize) - 1;

const kHeapObjectMapOffset = matchNum(/static const int kHeapObjectMapOffset = (\d+);/, "kHeapObjectMapOffset");

// 断言保证 ABI 关键锚点
if (kHeapObjectTag !== 1) throw new Error("kHeapObjectTag 预期为 1");
if (kWeakHeapObjectTag !== 3) throw new Error("kWeakHeapObjectTag 预期为 3");
if (kHeapObjectTagMask !== 3) throw new Error("kHeapObjectTagMask 预期为 3");
if (kSmiTag !== 0) throw new Error("kSmiTag 预期为 0");
if (kSmiTagMask !== 1) throw new Error("kSmiTagMask 预期为 1");
if (kHeapObjectMapOffset !== 0) throw new Error("kHeapObjectMapOffset 预期为 0");

const body = `${banner(
  "src/objects/tagged.h.js",
  "include/v8-internal.h",
  "Tag 掩码与关键偏移量常量"
)}
//! V8 指针 Tagging 与堆对象内存布局常量

/// 堆对象 Tag 标记值 (最低位: 0b01)
pub const HEAP_OBJECT_TAG: usize = ${kHeapObjectTag};

/// 弱引用堆对象 Tag 标记值 (最低位: 0b11)
pub const WEAK_HEAP_OBJECT_TAG: usize = ${kWeakHeapObjectTag};

/// 堆对象 Tag 位宽 (2 bits)
pub const HEAP_OBJECT_TAG_SIZE: usize = ${kHeapObjectTagSize};

/// 堆对象 Tag 掩码 (0b11)
pub const HEAP_OBJECT_TAG_MASK: usize = ${kHeapObjectTagMask};

/// 堆对象引用 Tag 掩码 (0b10)
pub const HEAP_OBJECT_REFERENCE_TAG_MASK: usize = ${kHeapObjectReferenceTagMask};

/// 转发指针 Tag 标记值 (0b00)
pub const FORWARDING_TAG: usize = ${kForwardingTag};

/// 转发指针 Tag 位宽 (2 bits)
pub const FORWARDING_TAG_SIZE: usize = ${kForwardingTagSize};

/// 转发指针 Tag 掩码 (0b11)
pub const FORWARDING_TAG_MASK: usize = ${kForwardingTagMask};

/// 小整数 (Smi) Tag 标记值 (最低位: 0b0)
pub const SMI_TAG: usize = ${kSmiTag};

/// 小整数 (Smi) Tag 位宽 (1 bit)
pub const SMI_TAG_SIZE: usize = ${kSmiTagSize};

/// 小整数 (Smi) Tag 掩码 (0b1)
pub const SMI_TAG_MASK: usize = ${kSmiTagMask};

/// 堆对象 Map 指针字段的首字节偏移量 (0)
pub const HEAP_OBJECT_MAP_OFFSET: usize = ${kHeapObjectMapOffset};

/// 31 位 Smi (小整数) 掩码与移位常量 (指针压缩 / 32 位系统)
pub mod smi31 {
  pub const SHIFT_SIZE: usize = 0;
  pub const VALUE_SIZE: usize = 31;
  pub const SHIFT_BITS: usize = super::SMI_TAG_SIZE + SHIFT_SIZE;
  pub const MIN_VALUE: i32 = -1073741824; // -2^30
  pub const MAX_VALUE: i32 = 1073741823;  // 2^30 - 1
}

/// 32 位 Smi (小整数) 掩码与移位常量 (未压缩 64 位系统)
pub mod smi32 {
  pub const SHIFT_SIZE: usize = 31;
  pub const VALUE_SIZE: usize = 32;
  pub const SHIFT_BITS: usize = super::SMI_TAG_SIZE + SHIFT_SIZE;
  pub const MIN_VALUE: i32 = i32::MIN; // -2^31
  pub const MAX_VALUE: i32 = i32::MAX; // 2^31 - 1
}
`;

writeSrc("v8-objects-core", "tagged_constants.rs", body);
console.log("tagged.h.js: 成功提取 Tag 掩码与常量至 v8-objects-core/src/tagged_constants.rs");
