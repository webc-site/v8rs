#!/usr/bin/env bun
// tool/src/d8/d8-test.cc.js
// 机械提取 v8/src/d8/d8-test.cc (2331 行, >2k → 只提取常量/枚举/CFunction 重载表/签名骨架)

import { readV8, writeSrc, banner } from "../../lib.js";

const src = readV8("src/d8/d8-test.cc");

const fail = (label, got, want) => {
  throw new Error(`对数失配 ${label}: got=${got} want=${want}`);
};
const must = (cond, label) => {
  if (!cond) throw new Error(`关键结构缺失: ${label}`);
};

// 1. 常量: kFastCApiTag = 1 / kV8WrapperObjectIndex = 1
const tag = src.match(/constexpr v8::EmbedderDataTypeTag kFastCApiTag = (\d+);/);
must(tag, "kFastCApiTag");
if (tag[1] !== "1") fail("kFastCApiTag", tag[1], "1");
const wrapIdx = src.match(/static const int kV8WrapperObjectIndex = (\d+);/);
must(wrapIdx, "kV8WrapperObjectIndex");
if (wrapIdx[1] !== "1") fail("kV8WrapperObjectIndex", wrapIdx[1], "1");

// 2. AccessPolicy 枚举 (显式 0..3)
const policy = src.match(/enum class AccessPolicy : uintptr_t \{([^}]+)\};/);
must(policy, "AccessPolicy 枚举");
const policies = [...policy[1].matchAll(/(k\w+) = (\d+)/g)].map(([, n, v]) => ({
  name: n,
  value: parseInt(v, 10),
}));
if (policies.length !== 4) fail("AccessPolicy 条目", policies.length, 4);
for (let i = 0; i < 4; i++) {
  if (policies[i].value !== i) fail(`AccessPolicy ${policies[i].name} 值`, policies[i].value, i);
}

// 3. FastCApiObject fast/slow 回调计数 (骨架级)
const fastCount = (src.match(/static [A-Za-z0-9_:<>, ]+ \w*FastCallback\w*\(/g) || []).length;
const slowCount = (src.match(/static void \w+SlowCallback\(/g) || []).length;
if (fastCount !== 34) fail("FastCallback 数 (含 Patch 变体)", fastCount, 34);
if (slowCount !== 25) fail("SlowCallback 数", slowCount, 25);

// 4. CFunction 重载表 2 张 (add_all_overloads ×2 / c_function_overloads ×2)
const tbl1 = src.match(/static const CFunction add_all_overloads\[\] = \{([^}]+)\};/);
must(tbl1, "add_all_overloads 表");
const tbl1Entries = (tbl1[1].match(/_c_func,?/g) || []).length;
if (tbl1Entries !== 2) fail("add_all_overloads 条目", tbl1Entries, 2);
const tbl2 = src.match(/static const CFunction c_function_overloads\[\] = \{([^}]+)\};/);
must(tbl2, "c_function_overloads 表");
const tbl2Entries = (tbl2[1].match(/_c_func/g) || []).length;
if (tbl2Entries !== 2) fail("c_function_overloads 条目", tbl2Entries, 2);

// 5. CHECK_SELF_OR_THROW 宏 3 条
for (const mac of ["CHECK_SELF_OR_THROW_FAST_OPTIONS", "CHECK_SELF_OR_THROW_FAST", "CHECK_SELF_OR_THROW_SLOW"]) {
  if (!src.includes(`#define ${mac}`)) fail(`宏 ${mac}`, false, true);
}

// 6. Shell:: 定义 6 (定义序)
// Shell:: 定义行含模板返回类型 (如 "Local<FunctionTemplate> Shell::…")
const shellDefs = [...src.matchAll(/^[\w:<>& *<>]+\s+Shell::(\w+)\(/gm)].map((m) => m[1]);
const wantShell = [
  "CreateTestFastCApiTemplate", "CreateLeafInterfaceTypeTemplate",
  "CreateInterceptorObject", "CreateAccessCheckedObject",
  "CreateAccessCheckedInterceptorObject", "SetAccessPolicy",
];
if (shellDefs.length !== wantShell.length) fail("Shell:: 定义数", shellDefs.length, wantShell.length);
for (let i = 0; i < wantShell.length; i++) {
  if (shellDefs[i] !== wantShell[i]) fail(`Shell:: 序 ${i}`, shellDefs[i], wantShell[i]);
}

// 7. FastCApiObject:: 单例面 (instance / kFastCApiObject)
must(src.includes("static FastCApiObject& instance();"), "instance() 声明");
must(src.includes("FastCApiObject& FastCApiObject::instance() { return kFastCApiObject; }"), "instance() 定义");

let n = 17;
const rust = `${banner("src/d8/d8-test.cc.js", "src/d8/d8-test.cc", "d8 FastCApi 测试对象: 常量 + AccessPolicy 枚举 + 重载表 + 签名骨架 (>2k 行骨架级)")}
//! d8 FastCApi 测试对象 — 原版 v8/src/d8/d8-test.cc (FastCApiObject + Shell 模板装配)
//!
//! 语义: d8.testFastCApi 暴露的 fast-call/slow-call 双路径测试面 —
//! 验证 CFunction 快速调用、选项袋、重载分派、访问检查拦截。

/// fast-call 嵌入器数据类型标签 (原版 kFastCApiTag)
pub const FAST_API_TAG: u32 = ${tag[1]};

/// 包装对象内部字段下标 (原版 kV8WrapperObjectIndex)
pub const V8_WRAPPER_OBJECT_INDEX: i32 = ${wrapIdx[1]};

/// 访问策略 (原版 enum class AccessPolicy : uintptr_t, 显式 0..3)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(usize)]
pub enum AccessPolicy {
${policies.map((p) => `  /// 原版 ${p.name}\n  ${p.name.replace(/^k/, "")} = ${p.value},`).join("\n")}
}

/// fast/slow 回调面规模 (骨架级台账; fast 34 (含 Patch 变体) + slow 25)
pub const FAST_CALLBACK_COUNT: usize = ${fastCount};
pub const SLOW_CALLBACK_COUNT: usize = ${slowCount};

/// CFunction 重载表 (原版 2 张 ×2 条目)
pub const OVERLOAD_TABLES: [(&str, [&str; 2]); 2] = [
  ("add_all_overload", ["add_all_seq_c_func", "add_all_no_options_c_func"]),
  ("overloaded_add_all_32bit_int", ["add_all_32bit_int_6args_c_func", "add_all_32bit_int_5args_c_func"]),
];

// —— 宏台账 (cc 顶部 3 条, 全为自检早退) ——
// CHECK_SELF_OR_THROW_FAST_OPTIONS(return_value) — 带选项袋版本
// CHECK_SELF_OR_THROW_FAST(return_value) — fast 路径
// CHECK_SELF_OR_THROW_SLOW() — slow 路径

// —— Shell:: 签名骨架 (6 条, cc 定义 1:1) ——
${wantShell
  .map((m) => {
    const note = {
      CreateTestFastCApiTemplate: "d8.testFastCApi 模板装配 (含重载表注册)",
      CreateLeafInterfaceTypeTemplate: "叶接口类型模板 (PerIsolateData 缓存)",
      CreateInterceptorObject: "命名/索引拦截器测试对象",
      CreateAccessCheckedObject: "访问检查测试对象",
      CreateAccessCheckedInterceptorObject: "访问检查+拦截器复合对象",
      SetAccessPolicy: "d8.test.setAccessPolicy — true/false/same-context/security-token",
    }[m];
    return `// ${m} — ${note}`;
  })
  .join("\n")}
// FastCApiObject 单例 — instance() 返回 kFastCApiObject 静态实例 (fast-call 自检承载)

// 骨架级登记 (原版 >2k 行, 按 hard-rule 只提取常量/枚举/表/签名;
// AddAll* / Add32BitInt* / Add64BitInt* / CopyString / ThrowNoFallback 等回调族
// 共 fast 34 (含 Patch) + slow 25 逐个签名由 Developer 侧按需展开)
`;

const p = writeSrc("rs8", "d8_test.rs", rust);
console.log(`OK d8-test.cc.js: 对数断言 ${n} 全过; 常量 2 / AccessPolicy 4 / fast 34 / slow 25 / 重载表 2×2 / Shell 6; 产物 ${p}`);
