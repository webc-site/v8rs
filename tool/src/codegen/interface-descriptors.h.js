#!/usr/bin/env bun
// tool/src/codegen/interface-descriptors.h.js
// 机械提取 v8/src/codegen/interface-descriptors.{h,-inl.h,cc} —
// StackArgumentOrder 枚举 + ParameterIndices 生成宏 + 描述符类面骨架。
// 语义面: 默认寄存器选择/初始化实现登记跳过。
// 产物: v8-codegen-core/src/interface_descriptors_generated.rs
//   (interface_descriptors.rs 为手写产物, 本脚本落 _generated 旁路, 禁覆写)

import { readV8, writeSrc, banner } from "../../lib.js";

const H_REL = "src/codegen/interface-descriptors.h";
const INL_REL = "src/codegen/interface-descriptors-inl.h";
const CC_REL = "src/codegen/interface-descriptors.cc";
const h = readV8(H_REL);
readV8(INL_REL); // 家族覆盖锚
readV8(CC_REL); // 家族覆盖锚

// 1. StackArgumentOrder 枚举 (显式别名值)
const saoM = h.match(/enum class StackArgumentOrder\s*\{([\s\S]*?)\n\};/);
if (!saoM) throw new Error("未找到 StackArgumentOrder 枚举");
const saoBody = saoM[1].replace(/\/\/[^\n]*/g, "");
const sao = [...saoBody.matchAll(/k(\w+)\s*(?:=\s*(k\w+))?/g)].map((m) => ({ name: "k" + m[1], alias: m[2] || null }));
if (sao.length !== 4) throw new Error(`StackArgumentOrder 条数不符: ${sao.length} != 4`);
if (sao[2].alias !== "kHighToLow" || sao[3].alias !== "kLowToHigh") throw new Error("别名锚不符");

// 2. ParameterIndices 生成宏 (DEFINE_PARAMETERS* 系列)
const paramMacros = [...h.matchAll(/^#define (DEFINE_PARAMETERS\w*)\(/gm)].map((m) => m[1]);
if (paramMacros.length < 5) throw new Error(`ParameterIndices 宏不足: ${paramMacros.length}`);

// 3. 类面: CallInterfaceDescriptor 关键方法锚
for (const anchor of [
  "static constexpr inline auto DefaultRegisterArray();",
  "static constexpr inline Register ContextRegister() {",
  "enum Flag {",
]) {
  if (!h.includes(anchor)) throw new Error(`缺锚 ${anchor.slice(0, 40)}`);
}
// Flag 枚举条目
const flagM = h.match(/enum Flag\s*\{([\s\S]*?)\n  \};/);
if (!flagM) throw new Error("未找到 Flag 枚举");
const flags = [...flagM[1].matchAll(/^\s*(k\w+)/gm)].map((m) => m[1]);
if (flags.length < 2) throw new Error(`Flag 条目异常: ${flags.length}`);

let rust = banner("src/codegen/interface-descriptors.h.js", "src/codegen/interface-descriptors.{h,-inl.h,cc}", "接口描述符枚举与骨架");
rust += `
// 语义面: 描述符默认寄存器/初始化/派生类登记表 (inl/cc) 登记跳过。

/// 栈参数序 (原 enum class StackArgumentOrder)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum StackArgumentOrder {
  /// 首参在栈最高地址
  HighToLow = 0,
  /// 首参在栈最低地址
  LowToHigh = 1,
}

/// 原 kDefault = kHighToLow (手写汇编/builtin 历史默认)
pub const STACK_ARGUMENT_ORDER_DEFAULT: StackArgumentOrder = StackArgumentOrder::HighToLow;
/// JS 调用约定使用 kLowToHigh
pub const STACK_ARGUMENT_ORDER_JS: StackArgumentOrder = StackArgumentOrder::LowToHigh;

/// 原参数索引生成宏族 (展开为每描述符 enum ParameterIndices):
${paramMacros.map((m) => `// - ${m}`).join("\n")}

/// 原 CallInterfaceDescriptor::Flag
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DescriptorFlag {
${flags.map((f) => `  /// ${f}\n  ${f.replace(/^k/, "")},`).join("\n")}
}

/// 原 CallInterfaceDescriptor 类面骨架
pub struct CallInterfaceDescriptor;

impl CallInterfaceDescriptor {
  // 原签名: static constexpr auto DefaultRegisterArray()
  // 原签名: static constexpr auto DefaultDoubleRegisterArray()
  // 原签名: static constexpr auto DefaultReturnRegisterArray()
  // 原签名: static constexpr auto DefaultReturnDoubleRegisterArray()
  // 原签名: static constexpr Register ContextRegister()
  // TODO(dev): 依平台寄存器配置补默认数组。
}

// —— 提取不变式断言 ——
const _: () = {
  assert!(STACK_ARGUMENT_ORDER_DEFAULT == StackArgumentOrder::HighToLow);
};
`;

const p = writeSrc("v8-codegen-core", "interface_descriptors_generated.rs", rust);
console.log(`written ${p} (SAO 4, 宏 ${paramMacros.length}, Flag ${flags.length})`);
