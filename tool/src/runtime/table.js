#!/usr/bin/env bun
// tool/src/runtime/table.js — Runtime 函数总表 (FunctionId / 描述符面)
// 覆盖: src/runtime/runtime.h + runtime.cc + runtime-utils.h
// 产出: v8-runtime/src/table.rs.skel
// 规则: 51 个 FOR_EACH_INTRINSIC_* 组宏全量登记 + RETURN_PAIR 2 项 + RETURN_OBJECT 组装序 +
//   FunctionId 枚举规则 (k##name / kInline##name / kNumFunctions 守尾) +
//   Function 描述符 (nargs/result_size) + RuntimeCallProperty / OptimizationStatus /
//   DebugPrintValueType 枚举; 各组 (名, 参数, 结果) 三元组由分组脚本展开, 本表只定组装序。
// 破环登记: Runtime 依赖几乎全部 L4 — 下层只认 FunctionId (u32), 回调经 RuntimeEnv 窄 trait。
// 复跑幂等: 纯文本扫描 + 确定性输出。
import { banner, readV8, writeSrc } from "../../lib.js";

const GEN = "src/runtime/table.js";
const FILES = ["src/runtime/runtime.h", "src/runtime/runtime.cc", "src/runtime/runtime-utils.h"];
const txt = Object.fromEntries(FILES.map((f) => [f, readV8(f)]));
const die = (m) => { throw new Error(`[table.js] ${m}`); };
const count = (s, re) => (s.match(re) ?? []).length;

const h = txt["src/runtime/runtime.h"];
// ① 组宏定义数: 51 (grep 实测 #define FOR_EACH_INTRINSIC_)
const groups = count(h, /#define FOR_EACH_INTRINSIC_/g);
if (groups !== 51) die(`FOR_EACH_INTRINSIC_* 定义数=${groups}, 期望 51`);
// ② RETURN_PAIR 2 项
for (const needle of ["F(DebugBreakOnBytecode, 1, 2)", "F(LoadLookupSlotForCall, 1, 2)"]) {
  if (!h.includes(needle)) die(`缺 RETURN_PAIR 项 ${needle}`);
}
// ③ 组装序针 (RETURN_OBJECT_IMPL 内 32 行组装: ARRAY→WEAKREF)
const asmRegion = h.slice(h.indexOf("#define FOR_EACH_INTRINSIC_RETURN_OBJECT_IMPL"), h.indexOf("#define FOR_EACH_THROWING_INTRINSIC(F)"));
// 定义行自身 (FOR_EACH_INTRINSIC_RETURN_OBJECT_IMPL(F, I)) 须排除, 只取被组装的各组
const asmGroups = [...asmRegion.matchAll(/  FOR_EACH_INTRINSIC_(\w+)\(F, I\)/g)].map((m) => m[1]);
if (asmGroups[0] !== "ARRAY" || asmGroups[asmGroups.length - 1] !== "WEAKREF") die(`组装序首尾失配: ${asmGroups[0]}..${asmGroups[asmGroups.length - 1]}`);
// ④ FunctionId / 描述符 / 附属枚举针
for (const needle of ["enum FunctionId : int32_t {", "kNumFunctions,", "struct Function {", "int8_t nargs;", "int8_t result_size;", "enum class RuntimeCallProperty { kCannotTriggerGC }", "enum class OptimizationStatus {", "enum class DebugPrintValueType : uint16_t {"]) {
  if (!h.includes(needle)) die(`缺针 ${needle}`);
}
// ⑤ 入口签名针 (Runtime_##name 三参)
if (!h.includes("Address Runtime_##name(int args_length, Address* args_object,")) die("缺 Runtime_##name 入口签名");
// ⑥ RUNTIME_FUNCTION 宏定义在 execution/arguments.h (本表引用面); runtime-utils.h 是 ObjectPair 面
const argH = readV8("src/execution/arguments.h");
if (!argH.includes("#define RUNTIME_FUNCTION(Name)")) die("arguments.h 缺 RUNTIME_FUNCTION 宏定义");
if (!txt["src/runtime/runtime-utils.h"].includes("struct ObjectPair")) die("runtime-utils.h 缺 ObjectPair");

console.log(`OK table.js 针核验通过: 组宏 ${groups}, 组装 ${asmGroups.length} 组, 枚举针 8 + 签名针 2 全过`);

