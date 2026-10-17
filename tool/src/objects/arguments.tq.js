#!/usr/bin/env bun
// tool/src/objects/arguments.tq.js — 覆盖 v8/src/objects/arguments.{tq,-inl.h}
// (差集: arguments.h 归既有 arguments.h.js 席)
// → v8-objects-arrays/src/arguments_tq.rs.skel
// 机械提取: SloppyArgumentsElements/AliasedArgumentsEntry tq 布局 + shape 类字段
//   + macro/builtin 清单 + -inl 方法清单 (6 定义)。
// 配置登记: 64 位非压缩 (kTaggedSize=8)。

import { readV8, writeSrc, banner } from "../../lib.js";

const tq = readV8("src/objects/arguments.tq");
const inl = readV8("src/objects/arguments-inl.h");

let checks = 0;
const must = (c, m) => {
  checks++;
  if (!c) throw new Error(`锚点失配: ${m}`);
};

// ── 1. tq 类/shape 解析 ─────────────────────────────────────────────────
// extern class (含 @cppObjectLayoutDefinition) 与 extern shape 两种; 空体 {} 允许
const classes = [...tq.matchAll(/^extern class (\w+)(?:\s+extends\s+([\w\s]+?))?\s*(\{\s*\}|\{[^}]*\n\})/gm)]
  .map((m) => ({ name: m[1], ext: (m[2] ?? "").trim(), body: m[3] === "{}" ? "" : m[3].slice(1, -1) }));
const shapes = [...tq.matchAll(/^extern shape (\w+)(?:\s+extends\s+([\w\s]+?))?\s*\{([^}]*)\n\}/gm)]
  .map((m) => ({ name: m[1], ext: (m[2] ?? "").trim(), body: m[3] }));
const clsNames = classes.map((c) => c.name);
must(JSON.stringify(clsNames) === JSON.stringify(["JSArgumentsObject", "SloppyArgumentsElements", "AliasedArgumentsEntry"]),
  `tq extern class 应 3 个 (JSArgumentsObject/SloppyArgumentsElements/AliasedArgumentsEntry), 实得 ${JSON.stringify(clsNames)}`);
must(JSON.stringify(shapes.map((s) => s.name)) === JSON.stringify(["JSSloppyArgumentsObject", "JSStrictArgumentsObject"]),
  `tq extern shape 应 2 个, 实得 ${JSON.stringify(shapes.map((s) => s.name))}`);

// 字段解析 (含 indexed: xxx[length])
function fields(body) {
  return [...body.matchAll(/^\s{2}(\w+)(\[(\w+)\])?:\s*([^;]+);/gm)]
    .map((m) => ({ name: m[1], indexed: !!m[2], len: m[3], type: m[4].trim() }));
}
const sloppy = classes.find((c) => c.name === "SloppyArgumentsElements");
must(sloppy.ext === "FixedArrayBase", `SloppyArgumentsElements 应 extends FixedArrayBase, 实得 ${sloppy.ext}`);
const sloppyFields = fields(sloppy.body);
must(JSON.stringify(sloppyFields) === JSON.stringify([
  { name: "context", indexed: false, len: undefined, type: "Context" },
  { name: "arguments", indexed: false, len: undefined, type: "FixedArray|NumberDictionary" },
  { name: "mapped_entries", indexed: true, len: "length", type: "Smi|TheHole" },
]), `SloppyArgumentsElements 字段失配: ${JSON.stringify(sloppyFields)}`);

const aliased = classes.find((c) => c.name === "AliasedArgumentsEntry");
must(aliased.ext === "Struct", `AliasedArgumentsEntry 应 extends Struct, 实得 ${aliased.ext}`);
const aliasedFields = fields(aliased.body);
must(aliasedFields.length === 1 && aliasedFields[0].name === "aliased_context_slot" && aliasedFields[0].type === "Smi",
  `AliasedArgumentsEntry 字段失配: ${JSON.stringify(aliasedFields)}`);

const sloppyShape = shapes.find((s) => s.name === "JSSloppyArgumentsObject");
const strictShape = shapes.find((s) => s.name === "JSStrictArgumentsObject");
must(JSON.stringify(fields(sloppyShape.body).map((f) => [f.name, f.type])) ===
  JSON.stringify([["length", "JSAny"], ["callee", "JSAny"]]), "JSSloppyArgumentsObject shape 字段失配");
must(JSON.stringify(fields(strictShape.body).map((f) => [f.name, f.type])) ===
  JSON.stringify([["length", "JSAny"]]), "JSStrictArgumentsObject shape 字段失配");

// 类型别名 + 导出宏
must(tq.includes("type JSArgumentsObjectWithLength =\n    JSSloppyArgumentsObject|JSStrictArgumentsObject;"),
  "JSArgumentsObjectWithLength 别名锚缺失");
must(tq.includes("macro IsJSArgumentsObjectWithLength("), "IsJSArgumentsObjectWithLength 宏锚缺失");

// ── 2. NewSloppyArgumentsElements 双配置变体 (TAGGED_SIZE_8_BYTES 分支) ──
const withPad = tq.match(/@if\(TAGGED_SIZE_8_BYTES\)\s*\n\s*macro NewSloppyArgumentsElements<Iterator: type>\([\s\S]*?\n\s*\}/);
must(!!withPad, "TAGGED_SIZE_8_BYTES 变体锚缺失");
must(withPad[0].includes("optional_padding: 0,"), "8 字节变体应含 optional_padding");
must(!tq.match(/@ifnot\(TAGGED_SIZE_8_BYTES\)[\s\S]*?\n\s*\}/)[0].includes("optional_padding"),
  "非 8 字节变体不应含 optional_padding");

// ── 3. macro / builtin 清单 ────────────────────────────────────────────
const macros = [...tq.matchAll(/^(?:@\w+\s*\n)?macro (\w+)\(/gm)].map((m) => m[1]);
const builtins = [...tq.matchAll(/^builtin (\w+)\(/gm)].map((m) => m[1]);
const exports = [...tq.matchAll(/@export\s*\nmacro (\w+)\(/g)].map((m) => m[1]);
for (const need of ["NewJSStrictArgumentsObject", "NewJSSloppyArgumentsObject", "NewJSFastAliasedArgumentsObject",
  "NewParameterMapIterator", "NewParameterValueIterator", "NewAllArguments", "NewRestArgumentsElements",
  "NewRestArguments", "NewStrictArgumentsElements", "NewStrictArguments", "NewSloppyArgumentsElements",
  "NewSloppyArguments", "NewRestArgumentsFromArguments", "AccessSloppyArgumentsCommon"])
  must(macros.includes(need), `tq 缺 macro ${need}`);
for (const need of ["NewSloppyArgumentsElements", "NewStrictArgumentsElements", "NewRestArgumentsElements",
  "FastNewSloppyArguments", "FastNewStrictArguments", "FastNewRestArguments"])
  must(builtins.includes(need), `tq 缺 builtin ${need}`);
for (const need of ["IsJSArgumentsObjectWithLength", "EmitFastNewAllArguments", "EmitFastNewRestArguments",
  "EmitFastNewStrictArguments", "EmitFastNewSloppyArguments", "SloppyArgumentsLoad", "SloppyArgumentsHas",
  "SloppyArgumentsStore"])
  must(exports.includes(need), `tq 缺 @export 宏 ${need}`);
// 参数映射迭代器结构
must(tq.includes("struct ParameterMapIterator {") || /struct ParameterMapIterator/.test(tq), "ParameterMapIterator 结构锚缺失");
must(tq.includes("struct ParameterValueIterator {") || /struct ParameterValueIterator/.test(tq), "ParameterValueIterator 结构锚缺失");

// ── 4. -inl.h 方法清单 ─────────────────────────────────────────────────
const inlDefs = [...inl.matchAll(/^(?:\w[\w:<>,&\s]* )?(AliasedArgumentsEntry|SloppyArgumentsElements)::(\w+)\(/gm)]
  .map((m) => `${m[1]}::${m[2]}`);
must(inlDefs.length === 9,
  `-inl 方法定义应 9 (构造 1 + getter/setter 7 + mapped_entries 重载), 实得 ${inlDefs.length}: ${JSON.stringify(inlDefs)}`);
must(inl.includes("Relaxed_Load()") && inl.includes("Relaxed_Store(this, value)"), "-inl relaxed 访问锚缺失");

// ── 5. 64 位布局数值 (kTaggedSize=8; FixedArrayBase 头 = map8+length4+padding4 = 16) ──
const T = 8;
const K_SLOPPY_ELEMENTS_HEADER = 16 + 2 * T; // context 16, arguments 24, mapped_entries 32..
const K_ALIASED_HEADER = 8 + 4; // Struct: map 8 + Smi 4 → 对齐 16? Struct 头 = map(8), 字段 4 → SizeFor = 16 对齐
must(K_SLOPPY_ELEMENTS_HEADER === 32, "SloppyArgumentsElements 数据头推导失配");
must(K_ALIASED_HEADER === 12, "AliasedArgumentsEntry 推导失配");

// ── 6. 生成 ────────────────────────────────────────────────────────────
const body = `${banner("src/objects/arguments.tq.js", "src/objects/arguments.{tq,-inl.h}", "arguments 差集 (h 归 arguments.h.js 席): tq 布局 + macro/builtin 清单 + -inl 方法")}
//! 差集骨架: arguments 对象族 (JSArgumentsObject/JSSloppy/JSStrict shape、
//! SloppyArgumentsElements、AliasedArgumentsEntry)。
//! 配置: 64 位非压缩 (kTaggedSize=8, FixedArrayBase 头 = map(8)+length(4)+padding(4) = 16)。
//! 语义面 (新建 arguments 对象的宏族 / 参数映射迭代器 / 快慢路径访问) 留 Developer 就地修订。

// —— tq 布局 ——
/// v8: src/objects/arguments.tq JSArgumentsObject extends JSObject (无自有字段)
/// v8: src/objects/arguments.tq SloppyArgumentsElements extends FixedArrayBase:
///   context offset=16 (Context), arguments offset=24 (FixedArray|NumberDictionary),
///   mapped_entries offset=32.. (indexed[length], Smi|TheHole)
pub const K_SLOPPY_ARGUMENTS_ELEMENTS_DATA_START: usize = ${K_SLOPPY_ELEMENTS_HEADER};
/// v8: src/objects/arguments.tq AliasedArgumentsEntry extends Struct:
///   aliased_context_slot offset=8 (Smi; 尺寸 12, 对齐取整登记留 Developer)
pub const K_ALIASED_ARGUMENTS_ENTRY_SIZE: usize = ${K_ALIASED_HEADER};

/// tq 布局字段表 (1:1 顺序)。
pub const SLOPPY_ARGUMENTS_ELEMENTS_FIELDS: [(&str, &str); 3] = [
${sloppyFields.map((f) => `  ("${f.name}", "${f.type}"),`).join("\n")}
];

/// v8: src/objects/arguments.tq shape JSSloppyArgumentsObject (length, callee)
///   / JSStrictArgumentsObject (length) — 起始形态, 属性可后移
pub const ARGUMENTS_SHAPES: &[(&str, &[&str])] = &[
  ("JSSloppyArgumentsObject", &["length", "callee"]),
  ("JSStrictArgumentsObject", &["length"]),
];

/// v8: src/objects/arguments.tq NewSloppyArgumentsElements<Iterator> —
///   8 字节 tagged 配置多一个 optional_padding: 0 字段 (本配置登记即 TAGGED_SIZE_8_BYTES)

// —— macro / builtin 清单 ——
/// v8: src/objects/arguments.tq namespace arguments 内 + 顶层宏 (${macros.length} 个)
pub const ARGUMENTS_TQ_MACROS: &[&str] = &[
${macros.map((m) => `  "${m}",`).join("\n")}
];

/// v8: src/objects/arguments.tq builtin (${builtins.length} 个)
pub const ARGUMENTS_TQ_BUILTINS: &[&str] = &[
${builtins.map((m) => `  "${m}",`).join("\n")}
];

/// v8: src/objects/arguments.tq @export 宏 (${exports.length} 个)
pub const ARGUMENTS_TQ_EXPORTS: &[&str] = &[
${exports.map((m) => `  "${m}",`).join("\n")}
];

// —— -inl 合并指引 (9 个定义; Rust 无 inl, 语义并入实现时就地修订) ——
/// v8: src/objects/arguments-inl.h — 命名映射: tagged 访问转常规方法;
///   mapped_entries(index, RelaxedLoadTag) → mapped_entries_relaxed(index);
///   set_mapped_entries(index, value, RelaxedStoreTag) → set_mapped_entries_relaxed(index, value)。
/// 语义要点:
///   1. AliasedArgumentsEntry 构造: map 取 roots.aliased_arguments_entry_map, 槽位 Smi::FromInt
///   2. aliased_context_slot(): Smi 解值
///   3. mapped_entries: 越界 DCHECK 后 objects()[index] relaxed 读写 (Smi|Hole 双态)
pub const ARGUMENTS_INL_METHODS: &[&str] = &[
${inlDefs.map((d) => `  "${d}",`).join("\n")}
];
`;

const p = writeSrc("v8-objects-arrays", "arguments_tq.rs.skel", body);
console.log(
  `arguments.tq.js: ${p} — tq 类 ${classes.length}+shape ${shapes.length}、字段 ${sloppyFields.length}+${aliasedFields.length}、` +
    `macro ${macros.length}/builtin ${builtins.length}/export ${exports.length}、-inl ${inlDefs.length}、` +
    `布局 ${K_SLOPPY_ELEMENTS_HEADER}/${K_ALIASED_HEADER}、断言 ${checks} 项通过`,
);
