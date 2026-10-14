#!/usr/bin/env bun
// tool/src/objects/property-descriptor-object-inl.h.js — 覆盖 v8/src/objects/property-descriptor-object-inl.h
// → v8-objects-maps/src/property_descriptor_object_inl.rs  (席位表 R3O4 目标 crate; 2026-10-12 归位)
// 机械提取: 构造器初值 (flags=Smi 0, value/get/set=the_hole) + 8 个存取器 + 字段类型面。

import { readV8, writeSrc, banner } from "../../lib.js";

const src = readV8("src/objects/property-descriptor-object-inl.h");

let checks = 0;
const must = (c, m) => { checks++; if (!c) throw new Error(`[property-descriptor-object-inl.h.js] ${m}`); };

// ── 1. 构造器初值锚 ────────────────────────────────────────────────────────
must(src.includes("flags_(Smi::zero())"), "构造器 flags_ 应初始化 Smi::zero()");
must(src.includes("value_(witness.roots().the_hole_value())"), "构造器 value_ 应初始化 the_hole");
must(src.includes("get_(witness.roots().the_hole_value())"), "构造器 get_ 应初始化 the_hole");
must(src.includes("set_(witness.roots().the_hole_value())"), "构造器 set_ 应初始化 the_hole");
must(src.includes("witness.roots().property_descriptor_object_map()"), "构造器应用 property_descriptor_object_map");

// ── 2. 存取器面 (8 个: 4 字段 × load/store) ────────────────────────────────
const loads = [...src.matchAll(/PropertyDescriptorObject::(\w+)\(\) const/g)].map((m) => m[1]);
must(loads.join(",") === "flags,value,get,set", `load 存取器序漂移: ${loads.join(",")}`);
const setters = [...src.matchAll(/PropertyDescriptorObject::set_(\w+)\(/g)].map((m) => m[1]);
must(setters.join(",") === "flags,value,get,set", `setter 序漂移: ${setters.join(",")}`);
// flags 是 int 直存取 (Smi 装箱), 其余三元组走 Tagged 字段
must(src.includes("int PropertyDescriptorObject::flags() const { return flags_.load().value(); }"), "flags 应返回 int (Smi 解箱)");
must(src.includes("Smi::FromInt(value)"), "set_flags 应 Smi::FromInt");

// ── 3. 字段类型面 (union 语义) ──────────────────────────────────────────────
must(src.includes("Tagged<UnionOf<JSAny, TheHole>> PropertyDescriptorObject::value()"), "value 应为 UnionOf<JSAny, TheHole>");
const ftCount = [...src.matchAll(/Tagged<UnionOf<FunctionTemplateInfo, JSAny, TheHole>>/g)].length;
must(ftCount === 4, `FunctionTemplateInfo 联合类型出现 ${ftCount} != 4 (get/set 的 load/store 声明)`);

// ── 4. 写屏障形参 ──────────────────────────────────────────────────────────
const wbCount = [...src.matchAll(/WriteBarrierMode mode\)/g)].length;
must(wbCount === 3, `WriteBarrierMode 形参 ${wbCount} != 3 (value/get/set 的 setter)`);

let rust = banner("src/objects/property-descriptor-object-inl.h.js", "src/objects/property-descriptor-object-inl.h",
  "PropertyDescriptorObject inline 存取器 + 构造器 the_hole 初值约定");
rust += `//! PropertyDescriptorObject inline 面: 4 槽 (flags Smi / value / get / set)。
//! 初值约定: flags = Smi 0; value/get/set = the_hole (与 descriptor-object 的位标志面互补)。

/// flags: int 经 Smi 装箱存取 (Is*/Has* 位标志, 定义面见 property-descriptor-object.h)。
pub fn flags(&self) -> i32 { todo!("就地修订: property-descriptor-object-inl.h") }
pub fn set_flags(&mut self, value: i32) { todo!("就地修订: property-descriptor-object-inl.h") }

/// value: UnionOf<JSAny, TheHole> — the_hole 表示未设置。
pub fn value(&self) -> Tagged { todo!("就地修订: property-descriptor-object-inl.h") }
pub fn set_value(&mut self, value: Tagged, mode: WriteBarrierMode) { todo!("就地修订: property-descriptor-object-inl.h") }

/// get: UnionOf<FunctionTemplateInfo, JSAny, TheHole>。
pub fn get(&self) -> Tagged { todo!("就地修订: property-descriptor-object-inl.h") }
pub fn set_get(&mut self, value: Tagged, mode: WriteBarrierMode) { todo!("就地修订: property-descriptor-object-inl.h") }

/// set: UnionOf<FunctionTemplateInfo, JSAny, TheHole>。
pub fn set(&self) -> Tagged { todo!("就地修订: property-descriptor-object-inl.h") }
pub fn set_set(&mut self, value: Tagged, mode: WriteBarrierMode) { todo!("就地修订: property-descriptor-object-inl.h") }

/// 原版构造器: flags=Smi::zero(), value/get/set=the_hole_value()。
pub fn new(witness: &AllocationWitness) -> Self { todo!("就地修订: property-descriptor-object-inl.h") }

// ── 提取不变式断言 ──
const _: () = {
  assert!(LOAD_ACCESSORS == ${loads.length});
  assert!(SET_ACCESSORS == ${setters.length});
  assert!(UNION_FT_SLOTS == ${ftCount});
};
pub const LOAD_ACCESSORS: usize = ${loads.length};
pub const SET_ACCESSORS: usize = ${setters.length};
pub const UNION_FT_SLOTS: usize = ${ftCount};
`;

const p = writeSrc("v8-objects-maps", "property_descriptor_object_inl.rs", rust);
console.log(`property-descriptor-object-inl.h.js: ${p} — load ${loads.length} + set ${setters.length} + 构造器 1, 联合类型锚 ${ftCount}, 断言 ${checks} 项全过`);
