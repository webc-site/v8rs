#!/usr/bin/env bun
// tool/src/objects/js-shared-array.h.js — JSSharedArray 骨架 (in-object length 槽)
// 组→文件映射: js-shared-array.h + js-shared-array.tq
// 产物: v8-objects-js-core/src/js_shared_array.rs.skel
import { readV8, writeSrc, banner } from "../../lib.js";

const h = readV8("src/objects/js-shared-array.h");
const tq = readV8("src/objects/js-shared-array.tq");

let checks = 0;
const must = (c, m) => { checks++; if (!c) throw new Error(`锚点失配: ${m}`); };

// ── 1. 类签名 + in-object 字段枚举 ──────────────────────────────────────
must(h.includes("V8_OBJECT class JSSharedArray : public AlwaysSharedSpaceJSObject {"), "继承 AlwaysSharedSpaceJSObject");
must(h.includes("kLengthFieldIndex = 0,"), "kLengthFieldIndex = 0");
must(h.includes("kInObjectFieldCount,"), "kInObjectFieldCount 枚举项");
must(h.includes("static const int kHeaderSize;"), "kHeaderSize 声明");
must(h.includes("static const int kSize;"), "kSize 声明");
must(h.includes("inline constexpr int JSSharedArray::kHeaderSize = sizeof(JSSharedArray);"), "kHeaderSize 定义");
must(h.includes("inline constexpr int JSSharedArray::kSize =\n    JSSharedArray::kHeaderSize +\n    (kTaggedSize * JSSharedArray::kInObjectFieldCount);"), "kSize 公式");
must(h.includes("DECL_PRINTER(JSSharedArray)"), "DECL_PRINTER");
must(h.includes("EXPORT_DECL_VERIFIER(JSSharedArray)"), "EXPORT_DECL_VERIFIER");

// ── 2. tq 交叉 (空类体, 字段全在 in-object) ────────────────────────────
must(tq.includes("extern class JSSharedArray extends AlwaysSharedSpaceJSObject {}"), "tq 空类体");

// ── 3. 槽位推导 (非压缩 8B) ────────────────────────────────────────────
// AlwaysSharedSpaceJSObject == JSObject 尺寸 (js-struct.h:48 static_assert) → 头 3 槽 24
const K_HEADER = 24;
const K_LENGTH = 24; // kHeaderSize + kLengthFieldIndex * kTaggedSize
const K_SIZE = 32; // 1 个 in-object 槽

// ── 4. 骨架产物 ──────────────────────────────────────────────────────────
const body = `${banner("src/objects/js-shared-array.h.js", "src/objects/js-shared-array.h + js-shared-array.tq", "JSSharedArray length in-object 槽差集骨架")}
//! JSSharedArray: 共享空间数组; length 常量字段住 in-object 槽 0, 恒等于 elements().length()。

/// v8: src/objects/js-shared-array.h JSSharedArray::kHeaderSize (= sizeof, 即 JSObject 头)
pub const K_HEADER_SIZE: usize = ${K_HEADER};
/// v8: src/objects/js-shared-array.h JSSharedArray::kLengthFieldIndex (in-object 槽 0)
pub const K_LENGTH_FIELD_INDEX: usize = 0;
pub const K_LENGTH_OFFSET: usize = ${K_LENGTH};
/// v8: src/objects/js-shared-array.h JSSharedArray::kSize (kHeaderSize + 1 槽)
pub const K_SIZE: usize = ${K_SIZE};

// length 访问器: AccessorInfo 只读 (TODO v8:12547: 空间优化候选 — 换 elements().length() 直读)
// 推迟: DECL_PRINTER / EXPORT_DECL_VERIFIER (L4 诊断); 共享值写屏障走 AlwaysSharedSpaceJSObject 实例类型快路径
`;

const p = writeSrc("v8-objects-js-core", "js_shared_array.rs.skel", body);
console.log(
  `js-shared-array.h.js: ${p} — in-object 槽 1 (length@${K_LENGTH})、kSize ${K_SIZE}、断言 ${checks} 项全部通过`,
);
