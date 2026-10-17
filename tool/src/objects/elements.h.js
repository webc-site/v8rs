#!/usr/bin/env bun
// tool/src/objects/elements.h.js — 覆盖 v8/src/objects/elements.h + elements-inl.h
// → v8-objects-arrays/src/elements.rs
// 机械提取: ElementsAccessor 方法全集 (纯虚/虚/inline/static) + inl 转发 + 自由函数签名骨架
// 对数 oracle: clang -E (elements-inl.h 含 elements.h, 一遍取全); linemarker 切自身行

import { execFileSync } from "node:child_process";
import { mkdirSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";
import { readV8, writeSrc, banner, root } from "../../lib.js";

// ── clang -E oracle (缺失 generated 头 stub 自愈) ────────────────────────
const stubDir = join(tmpdir(), "v8rs-clang-stub");
function clangE(rel, defines = []) {
  for (let i = 0; i < 40; i++) {
    try {
      return execFileSync(
        "clang",
        [
          "-E", "-x", "c++", join(root, "v8", rel), "-std=c++20",
          "-I", join(root, "v8"), "-I", join(root, "v8", "include"), "-I", stubDir,
          ...defines.map((d) => `-D${d}`),
        ],
        { encoding: "utf8", maxBuffer: 512 * 1024 * 1024, stdio: ["ignore", "pipe", "pipe"] },
      );
    } catch (e) {
      const err = String(e.stderr || e.message);
      const m = err.match(/fatal error: '([^']+)' file not found/);
      if (!m) throw new Error(`clang -E ${rel} 失败:\n${err.slice(0, 800)}`);
      mkdirSync(join(stubDir, m[1], ".."), { recursive: true });
      writeFileSync(join(stubDir, m[1]), "");
    }
  }
  throw new Error(`clang -E ${rel} stub 循环超限`);
}
function sliceSelf(iText, relSuffix) {
  const out = [];
  let cur = "";
  for (const ln of iText.split("\n")) {
    const m = ln.match(/^# \d+ "([^"]+)"/);
    if (m) {
      cur = m[1];
      continue;
    }
    if (cur.endsWith(relSuffix)) out.push(ln);
  }
  return out.join("\n");
}

let checks = 0;
const must = (cond, msg) => {
  checks++;
  if (!cond) throw new Error(`锚点失配: ${msg}`);
};

const iInl = clangE("src/objects/elements-inl.h"); // 含 elements.h 全量展开
const hSelf = sliceSelf(iInl, "src/objects/elements.h");
const inlSelf = sliceSelf(iInl, "src/objects/elements-inl.h");

// 1. ElementsAccessor 类体切分
const clsStart = hSelf.indexOf("class ElementsAccessor {");
const clsEnd = hSelf.indexOf("\n};", clsStart);
must(clsStart >= 0 && clsEnd > clsStart, "elements.h 缺 ElementsAccessor 类体");
const cls = hSelf.slice(clsStart, clsEnd);

// 2. 方法声明提取: [virtual|static|inline]... name(params) [const] [= 0];
//    .i 中 V8_WARN_UNUSED_RESULT 已展开为 __attribute__((...)) — 先剥离
const clsClean = cls.replace(/__attribute__\s*\(\([^)]*\)\)/g, "");
const KW = "static|inline|virtual|constexpr|V8_WARN_UNUSED_RESULT|V8_EXPORT_PRIVATE|friend";
const methods = [];
for (const m of clsClean.matchAll(new RegExp(`(?:(?:${KW})\\s+)*[\\w:<>~,&*\\s]+?\\b(\\w+)\\s*\\(([^;{]*?)\\)\\s*(const)?\\s*(?:noexcept)?\\s*(=\\s*0)?\\s*;`, "g"))) {
  const [full, name, params, isConst, pure] = m;
  if (["if", "for", "while", "switch", "return", "sizeof", "defined", "operator"].includes(name)) continue;
  if (/^(static_assert|DCHECK|CHECK|using)\b/.test(full.trim())) continue;
  let kind;
  if (pure) kind = "pure";
  else if (/\bvirtual\b/.test(full)) kind = "virtual";
  else if (/^static\b|^\s*static\b/.test(full)) kind = "static";
  else kind = "inline";
  methods.push({ name, kind, isConst: !!isConst, decl: full.trim() });
}
// 带体 static (ForKind) 与 = default 特形 (构造/析构) 手工补 — 锚定原文
if (clsClean.includes("static ElementsAccessor* ForKind(ElementsKind elements_kind) {"))
  methods.push({ name: "ForKind", kind: "static" });
else throw new Error("ForKind 带体声明锚失配");
if (clsClean.includes("virtual ~ElementsAccessor() = default;"))
  methods.push({ name: "~ElementsAccessor", kind: "virtual" });
else throw new Error("析构 = default 锚失配");
if (clsClean.includes("ElementsAccessor() = default;"))
  methods.push({ name: "ElementsAccessor", kind: "inline" });
else throw new Error("构造 = default 锚失配");
// 重载 (HasElement 纯虚 4 参 + inline 3 参) 按 (name, kind) 去重, 保留双行
const uniq = [];
for (const m of methods) {
  if (!uniq.some((u) => u.name === m.name && u.kind === m.kind)) uniq.push(m);
}
const nameSet = new Set(uniq.map((m) => m.name));
must(uniq.length === 47, `方法全集应 47 (38 纯虚 + 1 虚析构 + 4 inline/ctor + 4 static), 实得 ${uniq.length}`);
const byKind = (k) => uniq.filter((m) => m.kind === k);
must(byKind("pure").length === 38, `纯虚方法应 38 (= 0 结尾; grep 40 含 2 处默认参数误报), 实得 ${byKind("pure").length}`);
must(byKind("virtual").length === 1 && byKind("virtual")[0].name === "~ElementsAccessor",
  "虚方法应仅析构 1 个");
must(byKind("inline").length === 4, `inline (3 转发 + 默认构造) 应 4, 实得 ${byKind("inline").length}`);
must(byKind("static").length === 4, `static 方法应 4, 实得 ${byKind("static").length}`);
for (const n of ["ForKind", "InitializeOncePerProcess", "TearDown", "Concat"])
  must(byKind("static").some((m) => m.name === n), `static 缺 ${n}`);
for (const n of ["Validate", "HasElement", "HasEntry", "Get", "GetAtomic", "HasAccessors",
  "NumberOfElements", "SetLength", "CollectElementIndices", "CollectValuesOrEntries",
  "PrependElementIndices", "AddElementsToKeyAccumulator", "TransitionElementsKind",
  "GrowCapacityAndConvert", "GrowCapacity", "Set", "SetAtomic", "SwapAtomic",
  "CompareAndSwapAtomic", "Add", "Push", "Unshift", "Pop", "Shift", "Normalize",
  "GetCapacity", "Fill", "IncludesValue", "IndexOfValue", "LastIndexOfValue", "Reverse",
  "CopyElements", "CreateListFromArrayLike", "CopyTypedArrayElementsSlice",
  "GetEntryForIndex", "GetDetails", "Reconfigure", "Delete"])
  must(nameSet.has(n), `ElementsAccessor::${n} 缺失`);

// 3. elements-inl.h 三个转发体锚
must(inlSelf.includes("return CollectElementIndices(\n      object, direct_handle(object->elements(), keys->isolate()), keys);"),
  "inl CollectElementIndices 转发体失配");
must(inlSelf.includes("return PrependElementIndices(isolate, object,\n                               direct_handle(object->elements(), isolate), keys,\n                               convert, filter);"),
  "inl PrependElementIndices 转发体失配");
must(inlSelf.includes("return HasElement(isolate, holder, index, holder->elements(), filter);"),
  "inl HasElement 转发体失配");

// 4. 自由函数 (elements.h 尾部)
for (const fn of ["ArrayConstructInitializeElements", "CopyFastNumberJSArrayElementsToTypedArray",
  "CopyTypedArrayElementsToTypedArray", "CopyTypedArrayElementsSlice"])
  must(hSelf.includes(fn), `自由函数 ${fn} 缺失`);

// 5. 静态访问器表锚
must(hSelf.includes("static ElementsAccessor** elements_accessors_;"), "elements_accessors_ 锚失配");

// ── 生成 ────────────────────────────────────────────────────────────────
const methodRows = uniq.map((m) => `  ("${m.name}", MethodKind::${m.kind.charAt(0).toUpperCase() + m.kind.slice(1)}),`);
const pureList = byKind("pure").map((m) => `  "${m.name}",`);

let rust = banner("src/objects/elements.h.js", "src/objects/elements.h + elements-inl.h", "ElementsAccessor 接面清单与自由函数骨架; 对数 clang -E");
rust += `//! elements: ElementsAccessor 接面 (多态元素访问器) 的方法清单骨架。
//! 类型系统依赖 Isolate/JSObject/KeyAccumulator 等未落地子系统 — 本模块只登记接面,
//! 语义实现挂 elements_impl (elements.cc 骨架) 与后续 Developer 修订。

/// ElementsAccessor 方法类别。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MethodKind {
  /// 纯虚 (子类必须实现)
  Pure,
  /// 虚 (仅析构)
  Virtual,
  /// 内联转发 (定义于 elements-inl.h)
  Inline,
  /// 静态 (ForKind / InitializeOncePerProcess / TearDown / Concat)
  Static,
}

/// v8: src/objects/elements.h class ElementsAccessor 方法全集 (clang -E 对数: 47 项)。
pub const ELEMENTS_ACCESSOR_METHODS: [(&str, MethodKind); ${uniq.length}] = [
${methodRows.join("\n")}
];

/// 纯虚方法名单 (38; 每个访问器子类的实现义务清单)。
pub const PURE_VIRTUAL_METHODS: [&str; ${byKind("pure").length}] = [
${pureList.join("\n")}
];

/// v8: src/objects/elements.h elements-inl.h 转发体语义
/// (CollectElementIndices / PrependElementIndices / HasElement 以 holder->elements() 为后备仓)。
pub const INL_FORWARDERS: [&str; 3] = [
  "CollectElementIndices",
  "PrependElementIndices",
  "HasElement",
];

/// v8: src/objects/elements.h ArrayConstructInitializeElements
pub fn array_construct_initialize_elements() {
  todo!("readme 台账 ELEM-ISO: 需 Isolate + Factory + JSArray/JSObject / JavaScriptArguments");
}

/// v8: src/objects/elements.h CopyFastNumberJSArrayElementsToTypedArray
pub fn copy_fast_number_js_array_elements_to_typed_array() {
  todo!("readme 台账 ELEM-CSA: 需 Context, JSArray, JSTypedArray 堆对象定义");
}

/// v8: src/objects/elements.h CopyTypedArrayElementsToTypedArray
pub fn copy_typed_array_elements_to_typed_array() {
  todo!("readme 台账 ELEM-CSA: 需 JSTypedArray 堆对象定义");
}

/// v8: src/objects/elements.h CopyTypedArrayElementsSlice
pub fn copy_typed_array_elements_slice() {
  todo!("readme 台账 ELEM-CSA: 需 JSTypedArray 堆对象定义");
}

// —— 提取不变式断言 (clang -E 对数; 各 kind 分项计数在生成侧锁定后烧入) ——
const _: () = {
  assert!(ELEMENTS_ACCESSOR_METHODS.len() == ${uniq.length});
  assert!(PURE_VIRTUAL_METHODS.len() == ${byKind("pure").length});
  assert!(INL_FORWARDERS.len() == 3);
  assert!(ELEMENTS_ACCESSOR_METHODS.len() == ${byKind("pure").length} + 1 + ${byKind("inline").length} + ${byKind("static").length});
};
`;

const p = writeSrc("v8-objects-arrays", "elements.rs", rust);
console.log(
  `elements.rs: 方法全集 ${uniq.length} (纯虚 ${byKind("pure").length} + 虚 1 + inline 4 + static 4)、` +
    `inl 转发 3、自由函数骨架 4; 断言 ${checks} 项通过`,
);
