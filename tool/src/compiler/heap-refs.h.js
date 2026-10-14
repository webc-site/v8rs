#!/usr/bin/env bun
// tool/src/compiler/heap-refs.h.js
// 从 v8/src/compiler/heap-refs.h 提取 HeapRef 引用类型体系 (ObjectRef, HeapObjectRef, MapRef, JSFunctionRef, SharedFunctionInfoRef)
import { readV8 } from "../../lib.js";

const h = readV8("src/compiler/heap-refs.h");

// 1. 提取与校验 HEAP_BROKER_OBJECT_LIST_BASE
const listMatch = h.match(/#define\s+HEAP_BROKER_OBJECT_LIST_BASE\([^)]+\)\s*\\([\s\S]*?)\n\n#define HEAP_BROKER_OBJECT_LIST/);
if (!listMatch) {
  throw new Error("未找到 HEAP_BROKER_OBJECT_LIST_BASE 宏定义");
}

const objectListRaw = listMatch[1];
const refTypes = [];
for (const line of objectListRaw.split("\n")) {
  const m = line.match(/(?:BACKGROUND_SERIALIZED|NEVER_SERIALIZED)\((\w+)\)/);
  if (m) {
    refTypes.push(m[1]);
  }
}

const requiredRefs = [
  "Map",
  "JSFunction",
  "SharedFunctionInfo",
  "JSObject",
  "JSReceiver",
  "HeapObject",
];

for (const req of requiredRefs) {
  if (!refTypes.includes(req)) {
    throw new Error(`HEAP_BROKER_OBJECT_LIST 缺失关键类型: ${req}`);
  }
}

// 2. 校验关键类声明及其继承结构
const classAnchors = [
  [/class\s+(?:V8_EXPORT_PRIVATE\s+)?ObjectRef\s*\{/, "ObjectRef 根引用类"],
  [/class\s+(?:V8_EXPORT_PRIVATE\s+)?HeapObjectRef\s*:\s*public\s+ObjectRef\s*\{/, "HeapObjectRef : ObjectRef"],
  [/class\s+(?:V8_EXPORT_PRIVATE\s+)?MapRef\s*:\s*public\s+HeapObjectRef\s*\{/, "MapRef : HeapObjectRef"],
  [/class\s+(?:V8_EXPORT_PRIVATE\s+)?JSFunctionRef\s*:\s*public\s+JSObjectRef\s*\{/, "JSFunctionRef : JSObjectRef"],
  [/class\s+(?:V8_EXPORT_PRIVATE\s+)?SharedFunctionInfoRef\s*:\s*public\s+HeapObjectRef\s*\{/, "SharedFunctionInfoRef : HeapObjectRef"],
];

for (const [re, name] of classAnchors) {
  if (!re.test(h)) {
    throw new Error(`类声明锚点缺失: ${name}`);
  }
}

// 3. 校验核心访问接口
const methodAnchors = [
  // ObjectRef
  [/bool\s+IsSmi\(\)\s*const;/, "ObjectRef::IsSmi"],
  [/int\s+AsSmi\(\)\s*const;/, "ObjectRef::AsSmi"],
  [/bool\s+equals\(ObjectRef\s+other\)\s*const;/, "ObjectRef::equals"],
  [/size_t\s+hash_value\(\)\s*const;?/, "ObjectRef::hash_value"],
  // HeapObjectRef
  [/MapRef\s+map\(JSHeapBroker\*\s+broker\)\s*const;/, "HeapObjectRef::map"],
  [/HeapObjectType\s+GetHeapObjectType\(JSHeapBroker\*\s+broker\)\s*const;/, "HeapObjectRef::GetHeapObjectType"],
  // MapRef
  [/int\s+instance_size\(\)\s*const;/, "MapRef::instance_size"],
  [/InstanceType\s+instance_type\(\)\s*const;/, "MapRef::instance_type"],
  [/int\s+GetInObjectProperties\(\)\s*const;/, "MapRef::GetInObjectProperties"],
  [/int\s+NumberOfOwnDescriptors\(\)\s*const;/, "MapRef::NumberOfOwnDescriptors"],
  [/ElementsKind\s+elements_kind\(\)\s*const;/, "MapRef::elements_kind"],
  [/bool\s+is_stable\(\)\s*const;/, "MapRef::is_stable"],
  [/bool\s+is_constructor\(\)\s*const;/, "MapRef::is_constructor"],
  [/bool\s+is_deprecated\(\)\s*const;/, "MapRef::is_deprecated"],
  // JSFunctionRef
  [/SharedFunctionInfoRef\s+shared\(JSHeapBroker\*\s+broker\)\s*const;/, "JSFunctionRef::shared"],
  [/MapRef\s+initial_map\(JSHeapBroker\*\s+broker\)\s*const;/, "JSFunctionRef::initial_map"],
  [/bool\s+has_initial_map\(JSHeapBroker\*\s+broker\)\s*const;/, "JSFunctionRef::has_initial_map"],
  // SharedFunctionInfoRef
  [/Builtin\s+builtin_id\(\)\s*const;/, "SharedFunctionInfoRef::builtin_id"],
];

for (const [re, name] of methodAnchors) {
  if (!re.test(h)) {
    throw new Error(`方法签名锚点缺失: ${name}`);
  }
}

// 4. 校验 AccessMode 与 OddballType 枚举
const accessModeMatch = h.match(/enum\s+class\s+AccessMode\s*:\s*uint8_t\s*\{([^}]+)\};/);
if (!accessModeMatch) throw new Error("未找到 AccessMode 枚举");
const accessModes = accessModeMatch[1].split(",").map((s) => s.trim().split("=")[0].trim()).filter(Boolean);

const oddballMatch = h.match(/enum\s+class\s+OddballType\s*:\s*uint8_t\s*\{([^}]+)\};/);
if (!oddballMatch) throw new Error("未找到 OddballType 枚举");
const oddballTypes = oddballMatch[1]
  .replace(/\/\/[^\n]*/g, "")
  .split(",")
  .map((s) => s.trim())
  .filter(Boolean);

console.log("✓ HeapRef 引用类型体系与核心方法锚点校验通过:");
console.log(`  提取 Ref 族成员数: ${refTypes.length}`);
console.log(`  AccessMode: [${accessModes.join(", ")}]`);
console.log(`  OddballType: [${oddballTypes.join(", ")}]`);
console.log(`  核心方法锚点: ${methodAnchors.length} 项全部匹配`);
