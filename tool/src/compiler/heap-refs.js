#!/usr/bin/env bun
// v8/src/compiler/heap-refs.cc (单实现文件; .h 已由 heap-refs.h.js 覆盖)
// → v8-compiler-passes/src/heap_refs_impl.rs.skel
//
// 规则面: .cc 的 JSHeapBroker 序列化生命周期方法 + 各 XRef::Get* 查询面
// 方法清单 (逐条提取计数), 语义 (broker 模式分派/对象读取) 留锚。
import { readV8, writeSrc, banner } from "../../lib.js";

const cc = readV8("src/compiler/heap-refs.cc");

// —— 哨兵 ——
const sentinels = [
  [/void JSHeapBroker::InitializeAndStartSerializing\(/, "InitializeAndStartSerializing"],
  [/void JSHeapBroker::AddToJSFunctionCacheWorklist\(JSFunctionData\* data\) \{/, "AddToJSFunctionCacheWorklist"],
  [/void JSHeapBroker::DrainJSFunctionCacheWorklist\(\) \{/, "DrainJSFunctionCacheWorklist"],
  [/FieldIndex MapRef::GetFieldIndexFor\(InternalIndex descriptor_index\) const \{/, "MapRef 查询面"],
  [/SharedFunctionInfo::Inlineability SharedFunctionInfoRef::GetInlineability\(/, "GetInlineability"],
  [/OptionalObjectRef JSObjectRef::GetOwnConstantElement\(/, "JSObjectRef 查询面"],
  [/DCHECK_IMPLIES\(broker->mode\(\) == JSHeapBroker::kSerialized,/, "broker 模式断言"],
];
for (const [re, name] of sentinels) {
  if (!re.test(cc)) throw new Error(`哨兵缺失: ${name}`);
}

// —— 方法清单: <Ret> <Class>::<name>( 提取 ——
const impls = [...cc.matchAll(/^[\w:<>,* ]+ (\w+)::(\w+)\(/gm)].map((m) => [m[1], m[2]]);
const brokerImpls = impls.filter(([c]) => c === "JSHeapBroker").map(([, n]) => n);
if (brokerImpls.join() !== "InitializeAndStartSerializing,TryGetOrCreateData,AddToJSFunctionCacheWorklist,DrainJSFunctionCacheWorklist") {
  throw new Error(`JSHeapBroker 方法漂移: ${brokerImpls}`);
}
// XRef 查询面: Get* 方法按类分组
const gets = impls.filter(([, n]) => n.startsWith("Get"));
const getClass = [...new Set(gets.map(([c]) => c))].sort();
if (getClass.length < 8) throw new Error(`查询类 ${getClass.length} < 8`);
if (gets.length < 25) throw new Error(`Get* 方法 ${gets.length} < 25`);

const snake = (s) => s.replace(/([a-z0-9])([A-Z])/g, "$1_$2").toLowerCase();

const body = `${banner("src/compiler/heap-refs.js", "src/compiler/heap-refs.cc", "heap-refs 实现面 — broker 序列化生命周期 + 各 Ref::Get* 查询面 (${gets.length} 项骨架)")}
//! .h 的 Ref 类型声明已由 heap-refs.h.js 覆盖 (v8-compiler-graph 侧);
//! 本骨架只登记 .cc 实现面: JSHeapBroker ${brokerImpls.length} 方法 +
//! 各 Ref::Get* ${gets.length} 项。
//! 语义面 (推迟): broker 模式 (kSerializing/kSerialized/kDisabled) 分派,
//! 对象读取走 heap 对象层。

#![allow(dead_code, unused_variables)]

impl crate::broker::JSHeapBroker {
${brokerImpls.map((n) => `  /// v8: src/compiler/heap-refs.cc JSHeapBroker::${n}\n  pub fn ${snake(n)}(&mut self /* 见 .cc 参数表 */) {\n    todo!("broker 生命周期逻辑")\n  }`).join("\n")}
}

// —— 各 Ref 的 Get* 查询面 (按类分组) ——
${getClass.map((cls) => `/// v8: src/compiler/heap-refs.cc ${cls}::Get* 查询面\n/// ${gets.filter(([c]) => c === cls).map(([, n]) => n).join(", ")}\npub mod ${snake(cls.replace(/Ref$/, ""))}_queries {\n  pub const GET_METHODS: &[&str] = &[${gets.filter(([c]) => c === cls).map(([, n]) => `"${n}"`).join(", ")}];\n}`).join("\n\n")}

// 语义面: 每一 Get* 都按 broker 模式分派 (直接读 vs 序列化缓存),
// 对象布局读取属 heap 对象层 (objects/)
`;

const p = writeSrc("v8-compiler-passes", "heap_refs_impl.rs.skel", body);
console.log(`written ${p}\nJSHeapBroker方法=${brokerImpls.length} Get*总数=${gets.length} 覆盖类=${getClass.length} (${getClass.join(",")})`);
