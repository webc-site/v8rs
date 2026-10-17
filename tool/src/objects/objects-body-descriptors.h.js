#!/usr/bin/env bun
// tool/src/objects/objects-body-descriptors.h.js — 对象体描述符生成
// 组→文件映射 (本脚本管辖):
//   v8/src/objects/objects-body-descriptors.h + objects-body-descriptors-inl.h
//   → v8-objects-core/src/objects_body_descriptors.rs
// 规则面: 描述符类族拓扑 (18 类/mixin) + 每类 Visit 原语清单 + -inl 侧逐类型
//   BodyDescriptor 实例清单 (Type::BodyDescriptor → 基描述符); 遍历语义留骨架
import { readV8, writeSrc, banner } from "../../lib.js";

const h = readV8("src/objects/objects-body-descriptors.h");
const inl = readV8("src/objects/objects-body-descriptors-inl.h");

// —— 基础设施类清单 (h 侧, 声明顺序) ——
const infraAll = [...h.matchAll(/^class (\w+)(?:\s+final)?[\s:]/gm)].map((m) => m[1]);
const infra = [...new Set(infraAll)].map((n) => ({ name: n, base: "-" }));
const INFRA_EXPECT = ["BodyDescriptorBase", "DataOnlyBodyDescriptor", "FixedRangeBodyDescriptor", "FixedBodyDescriptor",
  "SuffixRangeBodyDescriptor", "FlexibleBodyDescriptor", "StructBodyDescriptor", "SuffixRangeWeakBodyDescriptor",
  "FixedWeakBodyDescriptor", "FlexibleWeakBodyDescriptor", "SubclassBodyDescriptor",
  "FixedExposedTrustedObjectBodyDescriptor", "StackedBodyDescriptor"];
if (infra.map((c) => c.name).join() !== INFRA_EXPECT.join())
  throw new Error(`描述符类清单漂移:\n  期望 ${INFRA_EXPECT.join(",")}\n  实得 ${infra.map((c) => c.name).join(",")}`);

// mixin 结构体 (WithStrongTrustedPointer / WithExternalPointer / WithProtectedPointer)
const mixins = [...h.matchAll(/^struct (With\w+) \{$/gm)].map((m) => m[1]);
if (mixins.join() !== "WithStrongTrustedPointer,WithExternalPointer,WithProtectedPointer")
  throw new Error(`mixin 清单漂移: ${mixins}`);
// 别名
if (!/using WithStrongCodePointer =/.test(h)) throw new Error("WithStrongCodePointer 别名缺失");
if (!/using FixedBodyDescriptorFor =/.test(h)) throw new Error("FixedBodyDescriptorFor 别名缺失");

// —— Iterate 原语清单 (BodyDescriptorBase) ——
// 只取 BodyDescriptorBase 块内 (含 doc 注释示例的首个 IterateBody)
const baseBlock = h.slice(0, h.indexOf("// This class describes a body of an object without any pointers"));
const prims = [...baseBlock.matchAll(/static inline void (Iterate\w+)\(/g)].map((m) => m[1]);
const PRIM_EXPECT = ["IterateBody", "IteratePointers", "IteratePointer", "IterateCustomWeakPointers", "IterateCustomWeakPointer",
  "IterateEphemeron", "IterateMaybeWeakPointers", "IterateMaybeWeakPointer", "IterateTrustedPointer", "IterateTrustedPointer",
  "IterateCodePointer", "IterateCodePointer", "IterateSelfIndirectPointer", "IterateProtectedPointer", "IterateJSDispatchEntry",
  "IterateJSObjectBodyImpl", "IterateJSObjectBodyWithoutEmbedderFieldsImpl"];
if (prims.join() !== PRIM_EXPECT.join())
  throw new Error(`Iterate 原语清单漂移 (应 ${PRIM_EXPECT.length}):\n  ${prims}`);


// —— -inl 侧: 逐类型 BodyDescriptor 定义清单 ——
// 形态: class <Owner>::BodyDescriptor [final] : public <Base>
const defs = [...inl.matchAll(/^class (?:V8_EXPORT_PRIVATE\s+)?([\w:<>]+?)::(\w*BodyDescriptor\w*)(?:\s+final)?[\s:]*\n?\s*(?::\s*public\s+)?([^\{\n]*)/gm)]
  .map((m) => ({ owner: m[1].replace(/<.*>/, ""), kind: m[2], base: m[3].replace(/\s+/g, " ").trim() || "-" }));
if (defs.length < 110) throw new Error(`逐类型 BodyDescriptor 应 ≥110, 实得 ${defs.length}`);
// 锚点抽查: 首尾与代表性条目
const owners = defs.map((d) => d.owner);
for (const need2 of ["HeapNumber", "JSObject", "JSFunction", "Map", "ScopeInfo", "FeedbackVector", "DescriptorArray", "SwissNameDictionary", "Code", "InstructionStream"])
  if (!owners.includes(need2)) throw new Error(`-inl 缺 ${need2}::BodyDescriptor`);
if (owners[0] !== "JSAPIObjectWithEmbedderSlots") throw new Error(`首个 BodyDescriptor 应 JSAPIObjectWithEmbedderSlots: ${owners[0]}`);
// 基类分布
const baseDist = new Map();
for (const d of defs) {
  const b = d.base.replace(/<.*>/, "<..>").split(" ").slice(-1)[0];
  baseDist.set(b, (baseDist.get(b) ?? 0) + 1);
}
// DataOnly 家族数量锚 (HeapNumber/HashSeedWrapper/TurbofanBitsetType 等)
const dataOnly = defs.filter((d) => /DataOnlyBodyDescriptor/.test(d.base)).length;
if (dataOnly < 8) throw new Error(`DataOnly 家族应 ≥8, 实得 ${dataOnly}`);
// -inl 侧 static_assert 对数登记
const saCount = (inl.match(/static_assert\(/g) || []).length;
if (saCount < 30) throw new Error(`-inl static_assert 应 ≥30, 实得 ${saCount}`);

const body = `${banner("src/objects/objects-body-descriptors.h.js", "src/objects/objects-body-descriptors.h + objects-body-descriptors-inl.h", "描述符拓扑与 Visit 原语清单; 遍历语义留骨架")}
//! 对象体描述符 (原版 objects-body-descriptors.{h,-inl.h})
//!
//! 每个 V8 对象类型经 \`T::BodyDescriptor\` 声明 GC 遍历规则:
//! 强指针区间 / 可弱区间 / 仅数据 / 可变长后缀 / 父子叠加 / mixin 叠层.

// ===== 基础设施类拓扑 (h 侧 ${infra.length} 类 + ${mixins.length} mixin) =====
/// 描述符基类族 (声明顺序: 名, 基类, 语义)
pub const DESCRIPTOR_CLASSES: [(&str, &str, &str); ${infra.length}] = [
${infra.map((c) => {
    const SEM = {
      BodyDescriptorBase: "Visit 原语宿主 (仅静态方法)",
      DataOnlyBodyDescriptor: "无指针对象, IterateBody 为空",
      FixedRangeBodyDescriptor: "强指针位于 [start,end) 定区间",
      FixedBodyDescriptor: "定尺寸 + 定区间 (start==end 时退化为 DataOnly)",
      SuffixRangeBodyDescriptor: "强指针位于 [start,object_size) 后缀",
      FlexibleBodyDescriptor: "可变尺寸后缀, SizeOf 从 map 取",
      StructBodyDescriptor: "Struct 通用: FlexibleBodyDescriptor<sizeof(Struct)>",
      SuffixRangeWeakBodyDescriptor: "后缀区间可含弱引用",
      FixedWeakBodyDescriptor: "定尺寸 + 定区间可弱",
      FlexibleWeakBodyDescriptor: "可变尺寸后缀可弱",
      SubclassBodyDescriptor: "父描述符 ∪ 子描述符 (父定尺寸且区间不相交)",
      FixedExposedTrustedObjectBodyDescriptor: "ExposedTrustedObject: 自间接指针 + Fixed 体",
      StackedBodyDescriptor: "Base + 逐层 mixin 叠加",
    };
    return `  ("${c.name}", "${c.base}", "${SEM[c.name] ?? ""}"),`;
  }).join("\n")}
];

/// 指针字段 mixin (作用于任意 Base 的 BodyDescriptor 内嵌类)
pub const DESCRIPTOR_MIXINS: [&str; ${mixins.length}] = [${mixins.map((m) => `"${m}"`).join(", ")}];

// ===== Visit 原语 (BodyDescriptorBase, ${[...new Set(prims)].length} 种) =====
${[...new Set(prims)].map((p) => `/// 原版 ${p}\npub fn ${(p.replace(/([a-z0-9])([A-Z])/g, "$1_$2").toLowerCase())}(/* obj, offsets..., v: &mut impl ObjectVisitor */) {\n  todo!("就地修订: 区间步进 kTaggedSize 逐槽派发 v.Visit*)\n}\n`).join("")}
// ===== 逐类型描述符清单 (-inl 侧, ${defs.length} 个) =====
/// Type::BodyDescriptor → 基描述符 (对数基准, GC 遍历表生成输入)
pub const TYPE_BODY_DESCRIPTORS: [(&str, &str); ${defs.length}] = [
${defs.map((d) => `  ("${d.owner}", "${d.base.replace(/"/g, "'")}"),`).join("\n")}
];

/// 基描述符分布对数 (DataOnly ${dataOnly} / 其余 ${defs.length - dataOnly}; -inl static_assert ${saCount} 处)
pub const DESCRIPTOR_BASE_STATS: (usize, usize, usize) = (${dataOnly}, ${defs.length - dataOnly}, ${saCount});

/// 描述符查询 (原版经 C++ 模板特化解析; Rust 侧由类型系统或查表实现)
pub fn body_descriptor_of(/* owner: &str */) -> Option<&'static str> {
  todo!("就地修订: 查 TYPE_BODY_DESCRIPTORS, Developer 修订环决定表驱动或 trait 化")
}
`;

const p = writeSrc("v8-objects-core", "objects_body_descriptors.rs", body);
console.log(`objects-body-descriptors.h.js: 已生成 ${p} (基类族 ${infra.length}, mixin ${mixins.length}, 原语 ${new Set(prims).size}, 逐类型 ${defs.length}, static_assert ${saCount})`);
