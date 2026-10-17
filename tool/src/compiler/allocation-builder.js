#!/usr/bin/env bun
// v8/src/compiler/allocation-builder.h + allocation-builder-inl.h
// → v8-compiler-passes/src/allocation_builder.rs.skel
//
// 规则面: AllocationBuilder 9 方法签名 + 字段布局; inl 中
// CanAllocateArray/CanAllocateSloppyArgumentElements 的尺寸上限判断语义
// (MaxRegularHeapObjectSize 门槛) 提取为规则; 其余复合分配留锚。
import { readV8, writeSrc, banner } from "../../lib.js";

const h = readV8("src/compiler/allocation-builder.h");
const inl = readV8("src/compiler/allocation-builder-inl.h");

// —— 哨兵 ——
const sentinels = [
  [h, /class AllocationBuilder final \{/, "AllocationBuilder"],
  [h, /inline void Allocate\(int size,/, "Allocate 声明"],
  [h, /void FinishAndChange\(Node\* node\) \{/, "FinishAndChange 内联体"],
  [h, /Node\* Finish\(\) \{/, "Finish"],
  [inl, /void AllocationBuilder::Allocate\(int size, AllocationType allocation,/, "inl Allocate"],
  [inl, /bool AllocationBuilder::CanAllocateArray\(/, "CanAllocateArray"],
  [inl, /bool AllocationBuilder::CanAllocateSloppyArgumentElements\(/, "CanAllocateSloppy"],
  [inl, /CHECK_GT\(size, 0\);/, "Allocate 前置"],
];
for (const [src, re, name] of sentinels) {
  if (!re.test(src)) throw new Error(`哨兵缺失: ${name}`);
}

// —— inl 方法清单对账 ——
const inlMethods = [...inl.matchAll(/^(?:void|bool) AllocationBuilder::(\w+)\(/gm)].map((m) => m[1]);
if (inlMethods.join() !== "Allocate,AllocateContext,CanAllocateArray,AllocateArray,CanAllocateSloppyArgumentElements,AllocateSloppyArgumentElements") {
  throw new Error(`inl 方法漂移: ${inlMethods}`);
}
// 头内联体方法 (Store×2 / FinishAndChange / Finish)
for (const m of ["Store", "FinishAndChange", "Finish"]) {
  if (!new RegExp(`\\b${m}\\(`).test(h)) throw new Error(`头方法缺失: ${m}`);
}

const body = `${banner("src/compiler/allocation-builder.js", "src/compiler/allocation-builder.h + src/compiler/allocation-builder-inl.h", "AllocationBuilder — 内联分配构造器 (effect 链追踪 + 复合分配助手")}
//! AllocationBuilder: 在简化算子层构造内联分配, 追踪初始 store 的 effect 链。
//!
//! 规则面: 方法签名 + 尺寸上限判断 + effect 链维护顺序。
//! 语义面 (推迟): Context::SizeFor / FixedArray::SizeFor 等布局函数
//! (objects 层), StoreField/StoreElement 节点构造。

#![allow(dead_code)]

/// v8: src/compiler/allocation-builder.h AllocationBuilder
pub struct AllocationBuilder<'a> {
  /// 原 jsgraph_
  jsgraph: &'a crate::js_graph::JSGraph,
  /// 原 broker_
  broker: &'a crate::broker::JSHeapBroker,
  /// 原 allocation_ — 当前分配节点
  allocation: Option<crate::node::Node>,
  /// 原 effect_
  effect: crate::node::Node,
  /// 原 control_
  control: crate::node::Node,
}

impl<'a> AllocationBuilder<'a> {
  /// v8: src/compiler/allocation-builder.h AllocationBuilder::AllocationBuilder
  pub fn new(
    jsgraph: &'a crate::js_graph::JSGraph,
    broker: &'a crate::broker::JSHeapBroker,
    effect: crate::node::Node,
    control: crate::node::Node,
  ) -> Self {
    Self { jsgraph, broker, allocation: None, effect, control }
  }

  /// v8: src/compiler/allocation-builder-inl.h AllocationBuilder::Allocate
  /// 原文: CHECK_GT(size,0); BeginRegion(kNotObservable) 接 effect 链,
  ///       Allocate(type,allocation) 节点 (size 常量入参), effect_ = allocation_
  pub fn allocate(&mut self, size: i32, allocation: crate::AllocationType /* 原 kYoung */, ty: crate::typer::Type /* 原 Type::Any() */) {
    assert!(size > 0);
    debug_assert!(size as i64 <= self.jsgraph.isolate().heap().max_regular_heap_object_size(allocation));
    todo!("BeginRegion + Allocate 节点拼接")
  }

  /// v8: src/compiler/allocation-builder.h AllocationBuilder::Store (FieldAccess 版)
  /// 原文: effect_ = StoreField(access)(allocation_, value, effect_, control_)
  pub fn store_field(&mut self, access: &crate::FieldAccess, value: crate::node::Node) {
    todo!("StoreField 节点拼接")
  }

  /// v8: src/compiler/allocation-builder.h AllocationBuilder::Store (ElementAccess 版)
  pub fn store_element(&mut self, access: &crate::ElementAccess, index: crate::node::Node, value: crate::node::Node) {
    todo!("StoreElement 节点拼接")
  }

  /// v8: src/compiler/allocation-builder.h AllocationBuilder::Store (常量版)
  /// 原文: IndirectPointer 时走 TrustedHeapConstant, 否则 ConstantNoHole
  pub fn store_const(&mut self, access: &crate::FieldAccess, value: crate::heap_refs::ObjectRef) {
    todo!("IndirectPointer 分派 + 常量节点")
  }

  /// v8: src/compiler/allocation-builder-inl.h AllocationBuilder::AllocateContext
  pub fn allocate_context(&mut self, variadic_part_length: i32, map: crate::heap_refs::MapRef) {
    // 原文: int size = Context::SizeFor(variadic_part_length); Allocate(size, kYoung, OtherInternal)
    todo!("Context::SizeFor + map/length 两 store")
  }

  /// v8: src/compiler/allocation-builder-inl.h AllocationBuilder::CanAllocateArray
  /// 原文: size = FIXED_ARRAY_TYPE ? FixedArray::SizeFor(length)
  ///                        : FixedDoubleArray::SizeFor(length);
  ///       return size <= MaxRegularHeapObjectSize(allocation)
  pub fn can_allocate_array(&self, length: i32, map: crate::heap_refs::MapRef, allocation: crate::AllocationType /* 原 kYoung */) -> bool {
    let size = if map.instance_type() == crate::objects::FIXED_ARRAY_TYPE {
      crate::objects::fixed_array_size_for(length)
    } else {
      crate::objects::fixed_double_array_size_for(length)
    };
    size <= self.jsgraph.isolate().heap().max_regular_heap_object_size(allocation)
  }

  /// v8: src/compiler/allocation-builder-inl.h AllocationBuilder::AllocateArray
  pub fn allocate_array(&mut self, length: i32, map: crate::heap_refs::MapRef, allocation: crate::AllocationType /* 原 kYoung */) {
    debug_assert!(self.can_allocate_array(length, map.clone(), allocation));
    todo!("Allocate + map/length 两 store (ForFixedArrayLengthLegacy)")
  }

  /// v8: src/compiler/allocation-builder-inl.h AllocationBuilder::CanAllocateSloppyArgumentElements
  /// 原文: size = SloppyArgumentsElements::SizeFor(length); return size <= MaxRegular…
  pub fn can_allocate_sloppy_argument_elements(&self, length: i32, map: crate::heap_refs::MapRef, allocation: crate::AllocationType /* 原 kYoung */) -> bool {
    let size = crate::objects::sloppy_arguments_elements_size_for(length);
    let _ = map;
    size <= self.jsgraph.isolate().heap().max_regular_heap_object_size(allocation)
  }

  /// v8: src/compiler/allocation-builder-inl.h AllocationBuilder::AllocateSloppyArgumentElements
  pub fn allocate_sloppy_argument_elements(&mut self, length: i32, map: crate::heap_refs::MapRef, allocation: crate::AllocationType /* 原 kYoung */) {
    debug_assert!(self.can_allocate_sloppy_argument_elements(length, map.clone(), allocation));
    todo!("Allocate + map/length 两 store")
  }

  /// v8: src/compiler/allocation-builder.h AllocationBuilder::FinishAndChange
  /// 原文: 类型转移 + ReplaceInput(0/1) + TrimInputCount(2) + ChangeOp(FinishRegion)
  pub fn finish_and_change(&mut self, node: &mut crate::node::Node) {
    todo!("FinishRegion 改写")
  }

  /// v8: src/compiler/allocation-builder.h AllocationBuilder::Finish
  pub fn finish(&mut self) -> crate::node::Node {
    todo!("FinishRegion(allocation_, effect_) 节点")
  }
}
`;

const p = writeSrc("v8-compiler-passes", "allocation_builder.rs.skel", body);
console.log(`written ${p}\n方法=11 inl方法=${inlMethods.length} 头内联=4`);
