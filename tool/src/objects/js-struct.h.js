#!/usr/bin/env bun
// tool/src/objects/js-struct.h.js — JSSharedStruct 族骨架 (共享空间抽象基类 + 注册表方法对数)
// 组→文件映射: js-struct.h + js-struct.cc + js-struct.tq
// 产物: v8-objects-js-core/src/js_struct.rs.skel
import { readV8, writeSrc, banner } from "../../lib.js";

const h = readV8("src/objects/js-struct.h");
const cc = readV8("src/objects/js-struct.cc");
const tq = readV8("src/objects/js-struct.tq");

let checks = 0;
const must = (c, m) => { checks++; if (!c) throw new Error(`锚点失配: ${m}`); };
const eq = (a, e, m) => { checks++; if (a !== e) throw new Error(`锚点失配: ${m} (期望 ${e}, 实得 ${a})`); };
const count = (s, re) => (s.match(re) || []).length;

// ── 1. 类层次与尺寸不变量 ────────────────────────────────────────────────
must(h.includes("V8_OBJECT class AlwaysSharedSpaceJSObject : public JSObject {"), "基类声明");
must(h.includes("V8_IT_ABSTRACT;"), "V8_IT_ABSTRACT");
must(h.includes("static_assert(sizeof(AlwaysSharedSpaceJSObject) == sizeof(JSObject));"), "尺寸 == JSObject");
must(h.includes("V8_OBJECT class JSSharedStruct : public AlwaysSharedSpaceJSObject {"), "JSSharedStruct 继承");
must(h.includes("inline constexpr int AlwaysSharedSpaceJSObject::kHeaderSize ="), "基类 kHeaderSize 外置定义");
must(h.includes("inline constexpr int JSSharedStruct::kHeaderSize = sizeof(JSSharedStruct);"), "子类 kHeaderSize 外置定义");
eq(count(h, /TaggedMember</g), 0, "族内自有 TaggedMember 数 (应为 0, 字段全在 in-object)");

// ── 2. SharedStructTypeRegistry ─────────────────────────────────────────
must(h.includes("class SharedStructTypeRegistry final {"), "registry final 类");
must(h.includes("static constexpr Tagged<Smi> deleted_element() { return Smi::FromInt(1); }"), "deleted_element = Smi(1)");
must(h.includes("class Data;"), "Data 前向声明");
must(cc.includes("class SharedStructTypeRegistry::Data : public OffHeapHashTableBase<Data> {"), "cc Data : OffHeapHashTableBase");
// registry 公共/私有方法签名对数
const regPub = ["Register", "IterateElements", "NotifyElementsRemoved"];
const regPriv = ["RegisterNoThrow", "CheckIfEntryMatches", "EnsureCapacity"];
for (const m of regPub) must(h.includes(`MaybeDirectHandle<Map> ${m}(`) || h.includes(`void ${m}(`), `registry 公共方法 ${m}`);
for (const m of regPriv) must(h.includes(`${m}(`), `registry 私有方法 ${m}`);
must(h.includes("base::Mutex data_mutex_;"), "data_mutex_ (全访问互斥)");

// ── 3. cc 方法定义对数 ──────────────────────────────────────────────────
const defs = [...cc.matchAll(/^(?:Maybe<[^>]+>|MaybeDirectHandle<[^>]+>|MaybeHandle<[^>]+>|void|bool|DirectHandle<Map>) (AlwaysSharedSpaceJSObject|JSSharedStruct|SharedStructTypeRegistry)::(\w+)\(/gm)]
  .map((m) => `${m[1]}::${m[2]}`);
eq(defs.filter((d) => d.startsWith("AlwaysSharedSpaceJSObject")).length, 5, "AlwaysSharedSpaceJSObject:: 定义数 (PrepareMapNoEnumerableProperties x2 重载 + WithEnumerable + DefineOwnProperty + HasInstance)");
eq(defs.filter((d) => d.startsWith("JSSharedStruct")).length, 5, "JSSharedStruct:: 定义数");
must(cc.includes("void AlwaysSharedSpaceJSObject::PrepareMapNoEnumerableProperties("), "PrepareMapNoEnumerableProperties 定义");
must(cc.includes("is_inobject ? JSSharedStruct::kHeaderSize + field_index * kTaggedSize"), "in-object 偏移公式锚 (js-struct.cc:252)");

// ── 4. tq 交叉 ──────────────────────────────────────────────────────────
must(tq.includes("@abstract"), "tq @abstract");
must(tq.includes("extern class AlwaysSharedSpaceJSObject extends JSObject {}"), "tq 基类");
must(tq.includes("extern class JSSharedStruct extends AlwaysSharedSpaceJSObject {}"), "tq 子类");

// ── 5. 骨架产物 ──────────────────────────────────────────────────────────
const body = `${banner("src/objects/js-struct.h.js", "src/objects/js-struct.h + js-struct.cc + js-struct.tq", "JSSharedStruct/AlwaysSharedSpaceJSObject/注册表差集骨架")}
//! 共享结构: 抽象基类零新增字段; 具体字段经 map 的 DescriptorArray 描述, 偏移
//! kHeaderSize + field_index * kTaggedSize (js-struct.cc CreateInstanceMap)。

/// v8: src/objects/js-struct.h AlwaysSharedSpaceJSObject::kHeaderSize (= JSObject 头)
pub const K_HEADER_SIZE: usize = 24; // 非压缩 8B; static_assert == sizeof(JSObject)
/// v8: src/objects/js-struct.h JSSharedStruct::kHeaderSize
pub const K_SHARED_STRUCT_HEADER_SIZE: usize = 24;
/// v8: src/objects/js-struct.h SharedStructTypeRegistry::deleted_element
pub const K_REGISTRY_DELETED_ELEMENT: i32 = 1; // Smi::FromInt(1)

// 方法映射 (语义推迟到 heap/registry 层):
//   AlwaysSharedSpaceJSObject: PrepareMapNoEnumerableProperties x2 / PrepareMapWithEnumerableProperties /
//     DefineOwnProperty / HasInstance (6 个, js-struct.cc)
//   JSSharedStruct: CreateInstanceMap / GetRegistryKey / IsRegistryKeyDescriptor /
//     GetElementsTemplate / IsElementsTemplateDescriptor (5 个)
//   SharedStructTypeRegistry: Register / IterateElements / NotifyElementsRemoved (公共) +
//     RegisterNoThrow / CheckIfEntryMatches / EnsureCapacity (私有), Data 基于 OffHeapHashTableBase,
//     deleted_element 哨兵 = Smi(1), base::Mutex 全访问互斥
`;

const p = writeSrc("v8-objects-js-core", "js_struct.rs.skel", body);
console.log(
  `js-struct.h.js: ${p} — 类 2 (零自有字段)、AlwaysSharedSpaceJSObject 定义 ${defs.filter((d) => d.startsWith("AlwaysSharedSpaceJSObject")).length}/5、JSSharedStruct 定义 ${defs.filter((d) => d.startsWith("JSSharedStruct")).length}/5、registry 方法 ${regPub.length + regPriv.length}/6、断言 ${checks} 项全部通过`,
);
