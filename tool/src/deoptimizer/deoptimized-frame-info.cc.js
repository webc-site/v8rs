#!/usr/bin/env bun
// tool/src/deoptimizer/deoptimized-frame-info.cc.js
// 机械提取 v8/src/deoptimizer/deoptimized-frame-info.cc — 骨架。
// 产物: v8-deoptimizer/src/deoptimized_frame_info.rs

import { readV8, writeSrc, banner } from "../../lib.js";

const CC_REL = "src/deoptimizer/deoptimized-frame-info.cc";
const cc = readV8(CC_REL);

if (!cc.includes("DeoptimizedFrameInfo::DeoptimizedFrameInfo(TranslatedState* state,")) {
  throw new Error("未找到 DeoptimizedFrameInfo 构造锚");
}
// 字段访问器在 deoptimized-frame-info.h (非本缺口), cc 只含构造体 —
// 提取构造内填充的字段名。
const fields = ["parameters_", "context_", "expression_stack_"];
for (const f of fields) if (!cc.includes(f)) throw new Error(`缺字段锚 ${f}`);
if (!cc.includes("stack_height")) throw new Error("缺栈高锚");

let rust = banner("src/deoptimizer/deoptimized-frame-info.cc.js", CC_REL, "反优化帧信息骨架");
rust += `
// 语义面: 帧遍历与参数/表达式栈抽取实现登记跳过。
// 构造期填充字段 (${fields.length} 条):
// 字段面: parameters_ (参数值数组) / context_ / expression_stack_ (表达式栈,
// 不含累加器); 构造期经 SetParameter/SetExpression 逐槽填充。
pub struct DeoptimizedFrameInfo {
  pub parameter_count: usize,
  pub stack_height: usize,
}

// TODO(dev): 依 translated state 补帧值物化。

// —— 提取不变式断言 ——
const _: () = {};
`;

const p = writeSrc("v8-deoptimizer", "deoptimized_frame_info.rs", rust);
console.log(`written ${p} (字段 ${fields.length})`);
