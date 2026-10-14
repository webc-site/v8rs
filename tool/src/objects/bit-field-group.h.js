#!/usr/bin/env bun
// tool/src/objects/bit-field-group.h.js — 位域组基础设施生成
// 组→文件映射 (本脚本管辖):
//   v8/src/objects/bit-field-group.h → v8-objects-rest/src/bit_field_group.rs
// 规则面: BitFieldGroup<Derived, U> 类型契约 (位链无缝/命名转换规则/get/with 语义)
import { readV8, writeSrc, banner } from "../../lib.js";

const h = readV8("src/objects/bit-field-group.h");

// —— 结构锚 ——
if (!/template <typename Derived, typename U>\nclass BitFieldGroup \{/.test(h)) throw new Error("BitFieldGroup 模板声明缺失");
if (!/static_assert\(std::is_unsigned_v<U>\);/.test(h)) throw new Error("U 无符号约束缺失");
if (!/using UnderlyingType = U;/.test(h)) throw new Error("UnderlyingType 别名缺失");
if (!/constexpr operator U\(\) const \{ return value_; \}/.test(h)) throw new Error("隐式转 U 缺失");
if (!/return Bit::decode\(value_\);/.test(h)) throw new Error("get 走 Bit::decode 缺失");
if (!/return Derived\(Bit::update\(value_, value\)\);/.test(h)) throw new Error("with 走 Bit::update 缺失");

// —— 命名与布局规则 (doc 面提取, 3 条首行; 断言文本在原文续行, 直接锚 h) ——
const rules = [...h.matchAll(/^\/\/  - (.+)$/gm)].map((m) => m[1]);
if (rules.length !== 3) throw new Error(`Torque 命名布局规则应 3 条, 实得 ${rules.length}`);
if (!rules[0].includes("Every type alias in the struct must be a base::BitField")) throw new Error("规则 1 (位链无缝) 漂移");
if (!h.includes("snake_case (IsCallableBit -> is_callable)")) throw new Error("规则 2 (命名转换) 漂移");
if (!h.includes("Map::Bits1 -> MapBits1")) throw new Error("规则 3 (Tq 结构名) 漂移");

const body = `${banner("src/objects/bit-field-group.h.js", "src/objects/bit-field-group.h", "位域组契约与 get/with 语义")}
//! 位域组 (原版 bit-field-group.h)
//!
//! \`BitFieldGroup<Derived, U>\` 是对象内位域成员的承载基类:
//! 派生 struct 里全部 \`using XBit = base::BitField<..>\` 按声明序无缝铺满 U,
//! metagen 由此生成 Torque bitfield struct (布局两侧互验).

/// 命名/布局规则 (原版 doc 1:1, ${rules.length} 条)
pub const BIT_FIELD_GROUP_RULES: [&str; ${rules.length}] = [
${rules.map((r) => `  "${r.replace(/"/g, "'")}",`).join("\n")}
];

/// 位域组 (U 为无符号底层; 位链由派生类型的 BitField 别名定义)
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct BitFieldGroup<U: Copy> {
  /// 原始位串 (原版 value_, 0 初始化)
  pub value: U,
}

// get<Bit>/with<Bit> 依赖位段基建 (Bit::FieldType / Bit::update) 与 Derived 回包,
// 待位段类型落位后由 Developer 就地实现 (原版语义锚):
//   get<Bit>:  Bit::decode(value_)
//   with<Bit>: Derived(Bit::update(value_, value)) — 副本不落盘, 调用方写回成员
`;

const p = writeSrc("v8-objects-rest", "bit_field_group.rs", body);
console.log(`bit-field-group.h.js: 已生成 ${p} (规则 ${rules.length}, 语义锚 6)`);
