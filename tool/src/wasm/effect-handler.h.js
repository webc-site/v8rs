#!/usr/bin/env bun
// tool/src/wasm/effect-handler.h.js
// 机械提取 v8/src/wasm/effect-handler.h: EffectHandlerTagIndex 位域 (bit 0 = is_switch, 其余 31 位 = index)

import { readV8, writeSrc, banner } from "../../lib.js";

const hSrc = readV8("src/wasm/effect-handler.h");

const m = hSrc.match(/struct __attribute__\(\(packed\)\) EffectHandlerTagIndex \{([\s\S]*?)\n\};/);
if (!m) throw new Error("未找到 EffectHandlerTagIndex");
const body = m[1];
const isSwitch = body.match(/IsSwitchField = base::BitField<bool, (\d+), (\d+)>;/);
const index = body.match(/IndexField = IsSwitchField::Next<uint32_t, (\d+)>;/);
if (!isSwitch || !index) throw new Error("位域定义漂移");
if (isSwitch[1] !== "0" || isSwitch[2] !== "1") throw new Error("IsSwitchField 应为 bit0 宽 1");
if (index[1] !== "31") throw new Error("IndexField 应宽 31");
if (!body.includes("uint32_t tag_and_kind;")) throw new Error("存储字段漂移");

console.log("EffectHandlerTagIndex: switch@bit0(1), index@bit1(31)");

let rust = banner(
  "src/wasm/effect-handler.h.js",
  "src/wasm/effect-handler.h",
  "效果处理器标签索引位域 (packed 32 位)",
);

rust += `
/// 效果处理器标签索引 (对应 struct EffectHandlerTagIndex, packed)
/// 布局: bit0 = is_switch, bit1..31 = tag index
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(C, packed)]
pub struct EffectHandlerTagIndex {
  /// tag_and_kind
  pub tag_and_kind: u32,
}

impl EffectHandlerTagIndex {
  /// IS_SWITCH 位移/宽度
  pub const IS_SWITCH_SHIFT: u32 = 0;
  pub const IS_SWITCH_BITS: u32 = 1;
  /// INDEX 位移/宽度
  pub const INDEX_SHIFT: u32 = 1;
  pub const INDEX_BITS: u32 = 31;
  /// INDEX 掩码
  pub const INDEX_MASK: u32 = (1 << Self::INDEX_BITS) - 1;

  /// 对应 is_switch()
  #[inline]
  #[must_use]
  pub const fn is_switch(self) -> bool {
    (self.tag_and_kind & 1) != 0
  }

  /// 对应 index()
  #[inline]
  #[must_use]
  pub const fn index(self) -> u32 {
    (self.tag_and_kind >> Self::INDEX_SHIFT) & Self::INDEX_MASK
  }

  /// 对应 encode(is_switch, tag_index)
  #[must_use]
  pub const fn encode(is_switch: bool, tag_index: u32) -> Self {
    Self {
      tag_and_kind: (is_switch as u32) | ((tag_index & Self::INDEX_MASK) << Self::INDEX_SHIFT),
    }
  }

  /// 对应 raw_value()
  #[inline]
  #[must_use]
  pub const fn raw_value(self) -> u32 {
    self.tag_and_kind
  }
}

// —— 提取不变式断言 ——
const _: () = {
  assert!(size_of::<EffectHandlerTagIndex>() == 4);
  let e = EffectHandlerTagIndex::encode(true, 0x1234);
  assert!(e.is_switch());
  assert!(e.index() == 0x1234);
  assert!(e.raw_value() == 0x2469);
  let n = EffectHandlerTagIndex::encode(false, u32::MAX);
  assert!(!n.is_switch());
  // 索引域仅 31 位, 高位被截断 (与原版 IndexField::encode 行为一致)
  assert!(n.index() == 0x7fff_ffff);
};
`;
;

const p = writeSrc("v8-wasm-core", "effect_handler.rs", rust);
console.log(`written ${p}`);
