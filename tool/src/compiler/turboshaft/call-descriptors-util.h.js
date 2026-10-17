#!/usr/bin/env bun
// tool/src/compiler/turboshaft/call-descriptors-util.h.js
// 机械提取 v8/src/compiler/turboshaft/call-descriptors-util.h
// → v8-turboshaft/src/call_descriptors_util.rs.skel
//
// 提取面: kMaxArgumentCount=8 + IndexTag 编号机制 + CallDescriptorBuilder
// (ARG 宏/ArgumentsBase/argument index 递增 static_assert)。
import { readV8, writeSrc, banner } from "../../../lib.js";

const h = readV8("src/compiler/turboshaft/call-descriptors-util.h");

// 1. 常量
const m = h.match(/static constexpr std::size_t kMaxArgumentCount = (\d+);/);
if (!m) throw new Error("未找到 kMaxArgumentCount");
const maxArg = Number(m[1]);
if (maxArg !== 8) throw new Error(`kMaxArgumentCount=${maxArg} 预期 8`);

// 2. ARG 宏 (index 登记机制)
const defineArgM = h.match(/#define DEFINE_TURBOSHAFT_CALL_DESCRIPTOR_ARG\(type, name\)([\s\S]*?)static_assert\(name##_index < kMaxArgumentCount\)/);
if (!defineArgM) throw new Error("DEFINE_TURBOSHAFT_CALL_DESCRIPTOR_ARG 宏体缺失");
for (const [re, label] of [
  [/name##_index/, "index 计数展开"],
  [/static_assert\(name##_index < kMaxArgumentCount\)/, "index < kMaxArgumentCount static_assert"],
]) {
  if (!re.test(h)) throw new Error(`哨兵缺失: ${label}`);
}

// 3. IndexTag 机制 + ArgumentsBase
for (const [re, label] of [
  [/struct IndexTag : public IndexTag<I - 1>/, "IndexTag 递减继承链"],
  [/struct IndexTag<0>\s*\{/, "IndexTag<0> 特化"],
  [/static constexpr size_t value = 0;/, "IndexTag<0>::value = 0"],
  [/struct ArgumentsBase/, "ArgumentsBase"],
  [/using arguments_vector_t = base::SmallVector<OpIndex, kMaxArgumentCount>;/, "arguments_vector_t"],
  [/struct CallDescriptorBuilder/, "CallDescriptorBuilder"],
  [/void operator\(\)\(const CallDescriptor\* desc/, "operator()(desc, ...) 合成调用"],
]) {
  if (!re.test(h)) throw new Error(`哨兵缺失: ${label}`);
}

// 4. 生成 Rust
let rust = banner(
  "src/compiler/turboshaft/call-descriptors-util.h.js",
  "src/compiler/turboshaft/call-descriptors-util.h",
  "调用描述符构造工具: 参数编号宏机制骨架",
);
rust += `
/// 原版 CallDescriptorBuilder::kMaxArgumentCount
pub const MAX_ARGUMENT_COUNT: usize = ${maxArg};

/// v8: src/compiler/turboshaft/call-descriptors-util.h IndexTag
///
/// 原版技巧: IndexTag<N> : IndexTag<N-1> 线性继承, 重载决议选择最大 N,
/// 编译期数参数个数 (Rust 侧由过程宏/元组长度替代)。
pub struct IndexTag;

/// v8: src/compiler/turboshaft/call-descriptors-util.h CallDescriptorBuilder
///
/// ARG(type, name) 宏为每个参数生成:
/// - 成员 V<type> name
/// - name##_index 递增计数 (static_assert < ${maxArg})
/// Arguments: ArgumentsBase + ARG 序列; arguments_vector_t: SmallVector<OpIndex, ${maxArg}>
pub struct CallDescriptorBuilder;

impl CallDescriptorBuilder {
  /// 原版 constexpr TSCallDescriptor 合成 operator()(callee, arguments, effects...)
  pub fn build_call_descriptor(&self) -> &'static crate::phase::TsCallDescriptor {
    unimplemented!("签名骨架, Developer 修订")
  }
}

// —— 提取不变式断言 ——
const _: () = {
  assert!(MAX_ARGUMENT_COUNT == 8);
};
`;

const p = writeSrc("v8-turboshaft", "call_descriptors_util.rs.skel", rust);
console.log(`written ${p}`);
console.log(`常量=1 (kMaxArgumentCount=${maxArg}) 宏机制=DEFINE_TURBOSHAFT_CALL_DESCRIPTOR_ARG+IndexTag 哨兵=8`);
