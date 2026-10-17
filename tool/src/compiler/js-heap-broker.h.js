#!/usr/bin/env bun
// tool/src/compiler/js-heap-broker.h.js
// 校验 JSHeapBroker 模式枚举 (kDisabled, kSerializing, kSerialized, kRetired) 与 broker 基本配置
import { readV8 } from "../../lib.js";

const h = readV8("src/compiler/js-heap-broker.h");
const cc = readV8("src/compiler/js-heap-broker.cc");

// 1. 校验与提取 BrokerMode 枚举
const modeMatch = h.match(/enum\s+BrokerMode\s*\{([^}]+)\};/);
if (!modeMatch) {
  throw new Error("未找到 JSHeapBroker::BrokerMode 枚举定义");
}

const rawModes = modeMatch[1]
  .split(",")
  .map((s) => s.trim())
  .filter(Boolean);
const expectedModes = ["kDisabled", "kSerializing", "kSerialized", "kRetired"];

for (const exp of expectedModes) {
  if (!rawModes.includes(exp)) {
    throw new Error(`BrokerMode 缺失枚举项: ${exp}`);
  }
}

// 2. 校验 JSHeapBroker 核心方法与状态转换锚点
const brokerAnchors = [
  [/BrokerMode\s+mode\(\)\s*const/, "mode() 查询方法"],
  [/void\s+StopSerializing\(\)/, "StopSerializing() 接口"],
  [/void\s+Retire\(\)/, "Retire() 接口"],
  [/bool\s+SerializingAllowed\(\)\s*const/, "SerializingAllowed() 接口"],
  [/void\s+InitializeAndStartSerializing\(/, "InitializeAndStartSerializing() 接口"],
  [/Zone\*\s+zone\(\)\s*const/, "zone() 分配器访问器"],
  [/bool\s+tracing_enabled\(\)\s*const/, "tracing_enabled() 接口"],
  [/PtrComprCageBase\s+cage_base\(\)\s*const/, "cage_base() 指针压缩基址"],
  [/GetOrCreateDataFlags/, "GetOrCreateDataFlags 标志位定义"],
  [/RefsMap\*\s+refs_;/, "refs_ 对象缓存映射表"],
  [/CompilationDependencies\*\s+dependencies_/, "dependencies_ 编译依赖成员"],
];

for (const [re, name] of brokerAnchors) {
  if (!re.test(h)) {
    throw new Error(`js-heap-broker.h 锚点缺失: ${name}`);
  }
}

// 3. 校验 js-heap-broker.cc 状态机断言
const ccAnchors = [
  [/CHECK_EQ\(mode_,\s*kSerializing\);[\s\S]*?mode_\s*=\s*kSerialized;/, "StopSerializing 状态转移 (kSerializing -> kSerialized)"],
  [/CHECK_EQ\(mode_,\s*kSerialized\);[\s\S]*?mode_\s*=\s*kRetired;/, "Retire 状态转移 (kSerialized -> kRetired)"],
];

for (const [re, name] of ccAnchors) {
  if (!re.test(cc)) {
    throw new Error(`js-heap-broker.cc 锚点缺失: ${name}`);
  }
}

console.log("✓ JSHeapBroker 模式枚举与关键配置锚点校验通过:");
console.log(`  BrokerMode: [${expectedModes.join(", ")}]`);
console.log(`  锚点检查: ${brokerAnchors.length + ccAnchors.length} 项全部匹配`);
