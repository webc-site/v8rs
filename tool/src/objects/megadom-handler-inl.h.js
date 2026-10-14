#!/usr/bin/env bun
// tool/src/objects/megadom-handler-inl.h.js — 覆盖 v8/src/objects/megadom-handler-inl.h
// → v8-objects-maps/src/megadom_handler_inl.rs  (席位表 R3O4 目标 crate; 2026-10-12 归位)
// 机械提取: accessor/context 两字段的 6 个 inline 存取器 (含 Acquire/Release 变体)。

import { readV8, writeSrc, banner } from "../../lib.js";

const src = readV8("src/objects/megadom-handler-inl.h");

let checks = 0;
const must = (c, m) => { checks++; if (!c) throw new Error(`[megadom-handler-inl.h.js] ${m}`); };

// ── 1. 方法定义面 (6 个: accessor×4 + context×2) ───────────────────────────
const defs = [...src.matchAll(/^(?:Tagged<MaybeObject>|void) MegaDomHandler::(\w+)\(/gm)].map((m) => m[1]);
must(defs.length === 6, `存取器数 ${defs.length} != 6`);
// 实际分布: accessor(), accessor(AcquireLoadTag), set_accessor(普通), set_accessor(ReleaseStoreTag), context(), set_context()
must(defs.join(",") === "accessor,accessor,set_accessor,set_accessor,context,set_context",
  `存取器序漂移: ${defs.join(",")}`);
const accessorVariants = defs.filter((d) => d === "accessor" || d === "set_accessor").length;
const contextVariants = defs.filter((d) => d === "context" || d === "set_context").length;
must(accessorVariants === 4, `accessor 变体数 ${accessorVariants} != 4`);
must(contextVariants === 2, `context 变体数 ${contextVariants} != 2`);

// ── 2. 内存序锚 ─────────────────────────────────────────────────────────────
must(src.includes("accessor_.Acquire_Load();"), "缺 accessor Acquire_Load");
must(src.includes("accessor_.Release_Store(this, value, mode);"), "缺 accessor Release_Store");
must(src.includes("accessor_.load();"), "缺 accessor 普通 load");
must(src.includes("context_.load();"), "缺 context load");
must(src.includes("context_.store(this, value, mode);"), "缺 context store (WriteBarrierMode)");
// context 无 Acquire/Release 变体 — 计数护栏
must(!/context_\.Acquire_Load/.test(src) && !/context_\.Release_Store/.test(src), "context 不应有内存序变体");

// ── 3. 写屏障锚 ─────────────────────────────────────────────────────────────
// 3 个带屏障 setter: set_accessor(普通) / set_accessor(Release) / set_context
const wbCount = [...src.matchAll(/WriteBarrierMode mode\)/g)].length;
must(wbCount === 3, `WriteBarrierMode 形参 ${wbCount} != 3`);

let rust = banner("src/objects/megadom-handler-inl.h.js", "src/objects/megadom-handler-inl.h",
  "MegaDomHandler 两字段 inline 存取器 (accessor 含内存序变体)");
rust += `//! MegaDomHandler inline 面: accessor (MaybeObject, API 回调槽) 与 context 两槽。
//! accessor 提供 Acquire_Load / Release_Store 内存序变体; context 仅普通 load/store。

/// accessor 变体数 (load/acquire/store/release)。
pub const ACCESSOR_VARIANTS: usize = ${accessorVariants};
/// context 变体数 (load/store)。
pub const CONTEXT_VARIANTS: usize = ${contextVariants};

// ── accessor (${accessorVariants} 个变体) ──
/// 原版 accessor() — 普通 load。
pub fn accessor(&self) -> MaybeObject { todo!("就地修订: megadom-handler-inl.h") }
/// 原版 accessor(AcquireLoadTag) — acquire 语义 load。
pub fn accessor_acquire(&self) -> MaybeObject { todo!("就地修订: megadom-handler-inl.h") }
/// 原版 set_accessor(value, mode) — 普通 store, 写屏障模式透传。
pub fn set_accessor(&mut self, value: MaybeObject, mode: WriteBarrierMode) { todo!("就地修订: megadom-handler-inl.h") }
/// 原版 set_accessor(value, ReleaseStoreTag, mode) — release 语义 store。
pub fn set_accessor_release(&mut self, value: MaybeObject, mode: WriteBarrierMode) { todo!("就地修订: megadom-handler-inl.h") }

// ── context (${contextVariants} 个) ──
/// 原版 context() — 普通 load (无内存序变体)。
pub fn context(&self) -> MaybeObject { todo!("就地修订: megadom-handler-inl.h") }
/// 原版 set_context(value, mode)。
pub fn set_context(&mut self, value: MaybeObject, mode: WriteBarrierMode) { todo!("就地修订: megadom-handler-inl.h") }

// ── 提取不变式断言 ──
const _: () = {
  assert!(ACCESSOR_VARIANTS == 4);
  assert!(CONTEXT_VARIANTS == 2);
};
`;

const p = writeSrc("v8-objects-maps", "megadom_handler_inl.rs", rust);
console.log(`megadom-handler-inl.h.js: ${p} — 存取器 ${defs.length} (accessor ${accessorVariants} + context ${contextVariants}), 写屏障形参 ${wbCount}, 断言 ${checks} 项全过`);
