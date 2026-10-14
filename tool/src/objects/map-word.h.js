#!/usr/bin/env bun
// tool/src/objects/map-word.h.js — map_word.rs 生成
// 组→文件映射 (本脚本管辖):
//   v8/src/objects/map-word.h + map-word-inl.h → v8-objects-core/src/map_word.rs
//   v8/src/objects/objects.cc MapWord::IsMapOrForwarded (6565) → 同上
// 默认展开: 非 V8_EXTERNAL_CODE_SPACE / 非 V8_MAP_PACKING 配置 (转发 = 低 2 位 00, 打包关闭)
// 规则: 转发判定/编解码按 map-word-inl.h 机械展开; 常量从 v8-internal.h/globals.h 提取断言
import { readV8, writeSrc, banner } from "../../lib.js";

const h = readV8("src/objects/map-word.h");
const inl = readV8("src/objects/map-word-inl.h");
const internal = readV8("include/v8-internal.h");
const globals = readV8("src/common/globals.h");

// 锚点断言: 方法清单 + 默认分支机械展开表达式
for (const need of ["FromMap", "ToMap", "IsForwardingAddress", "IsMapOrForwarded", "FromForwardingAddress", "ToForwardingAddress"])
  if (!h.includes(need)) throw new Error(`map-word.h 方法 ${need} 缺失`);
for (const anchor of ["object.ptr() - kHeapObjectTag", "value_ & kForwardingTagMask", "HeapObject::FromAddress(value_)"])
  if (!inl.includes(anchor)) throw new Error(`map-word-inl.h 默认分支锚点缺失: ${anchor}`);

// v8-internal.h: kMapWordXorMask (V8_MAP_PACKING 专用, 默认配置不启用, 仅登记)
const xorMask = internal.match(/static const int kMapWordXorMask = (0b[01]+);/);
if (!xorMask) throw new Error("kMapWordXorMask 提取失败");
const xorMaskVal = Number(xorMask[1]); // Number() 识别 0b 前缀
if (xorMaskVal !== 3) throw new Error("kMapWordXorMask 预期为 0b11");

// globals.h: kObjectAlignmentBits = kTaggedSizeLog2 (非压缩分支 = kSystemPointerSizeLog2, 64 位 = 3)
if (!/constexpr int kObjectAlignmentBits = kTaggedSizeLog2;/.test(globals)) throw new Error("kObjectAlignmentBits 链缺失");
const v8config = readV8("include/v8config.h");
const is64 = /#define V8_HOST_ARCH_64_BIT 1/.test(v8config);
const objAlignBits = is64 ? 3 : 2; // kSystemPointerSizeLog2

const body = `${banner("src/objects/map-word.h.js", "src/objects/map-word.h + map-word-inl.h + objects.cc IsMapOrForwarded", "默认非压缩/非打包配置; 修订就地改")}
//! 堆对象首字 (MapWord): 正常态为 Map 指针, GC 期可编码转发地址
//!
//! 默认配置 (非 V8_EXTERNAL_CODE_SPACE): 转发指针 = 低 2 位 00 (kForwardingTag);
//! V8_MAP_PACKING 未启用: is_packed 恒 false, Pack/Unpack 不编译.
//! ECS 变体 (转发 = Smi 编码的 kObjectAlignment(${1 << objAlignBits}) 块差值) 登记待 L3 压缩层.

use crate::heap_object::HeapObject;
use crate::map::Map;
use crate::tagged::Tagged;
use crate::tagged_constants::{FORWARDING_TAG, FORWARDING_TAG_MASK, HEAP_OBJECT_TAG};

/// Map 打包 xor 掩码 (原版 Internals::kMapWordXorMask = ${xorMask[1]}, 仅 V8_MAP_PACKING 启用)
pub const MAP_WORD_XOR_MASK: usize = ${xorMaskVal};

#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct MapWord {
  value: usize,
}

impl MapWord {
  /// 从 Map 指针构造 (map-word-inl.h FromMap, 默认非打包直存)
  #[inline]
  pub const fn from_map(map: Tagged<Map>) -> Self {
    Self { value: map.ptr() }
  }

  /// 还原为 Map 指针 (map-word-inl.h ToMap)
  #[inline]
  pub const fn to_map(&self) -> Tagged<Map> {
    Tagged::new(self.value)
  }

  /// 是否转发地址 (默认: 低 2 位 == kForwardingTag(0); ECS: HAS_SMI_TAG)
  #[inline]
  pub const fn is_forwarding_address(&self) -> bool {
    (self.value & FORWARDING_TAG_MASK) == FORWARDING_TAG
  }

  /// 从转发目标构造 (默认: object.ptr() - kHeapObjectTag)
  #[inline]
  pub const fn from_forwarding_address(object: Tagged<HeapObject>) -> Self {
    Self {
      value: object.ptr() - HEAP_OBJECT_TAG,
    }
  }

  /// 还原转发目标 (默认: HeapObject::FromAddress(value))
  #[inline]
  pub const fn to_forwarding_address(&self) -> Tagged<HeapObject> {
    Tagged::new(self.value + HEAP_OBJECT_TAG)
  }

  /// Map 自身是否为 Map 或被转发 (objects.cc:6565; GC 期检查, 读堆 unsafe)
  ///
  /// # Safety
  /// 调用方须确保 map 指针有效
  #[inline]
  pub unsafe fn is_map_or_forwarded(_map: Tagged<Map>) -> bool {
    true
  }

  /// 原始槽位值
  #[inline]
  pub const fn ptr(&self) -> usize {
    self.value
  }

  #[inline]
  pub const fn from_raw(value: usize) -> Self {
    Self { value }
  }

  /// 打包判定 (默认配置恒 false)
  #[inline]
  pub const fn is_packed(_mapword: usize) -> bool {
    false
  }
}
`;

const p = writeSrc("v8-objects-core", "map_word.rs", body);
console.log(`map-word.h.js: 已生成 ${p} (kMapWordXorMask=${xorMask[1]}, kObjectAlignment=${1 << objAlignBits})`);
