[English](#en) | [中文](#zh)

---

<a name="en"></a>

#

---

<a name="zh"></a>

# v8-objects-maps 推迟台账 (deferred ledger)

函数级对账 (todo.md 十二.1) 与延期登记 (十二.6)。每个仍留 `todo!` 的接口在此登记:
原版 `file:line` + 前置条件 + 解锁 crate。填体完成后从本表移除并转 tests 断言。

- [推迟条目](#推迟条目)
- [冻结 checklist 关联](#冻结-checklist-关联)

## 推迟条目

| 组 ID | 原版 file:line | 符号 | 推迟原因 (前置条件) | 解锁 crate |
|---|---|---|---|---|
| PD-ISO | property-descriptor.h:44 | `PropertyDescriptor::ToObject` | 需 Isolate + DirectHandle<JSObject>/Factory 构造对象 | v8-runtime, v8-objects-js-core, v8-heap-core |
| PD-ISO | property-descriptor.h:46 | `ToPropertyDescriptorObject` | 需 PropertyDescriptorObject 工厂构造 (Isolate/Factory) | v8-runtime, v8-heap-core |
| PD-ISO | property-descriptor.h:50 | `ToPropertyDescriptor` | 需 Isolate + Handle<JSAny> 属性读取 (GetOwnProperty/HasOwnProperty) | v8-runtime, v8-objects-js-core |
| PD-ISO | property-descriptor.h:54 | `CompletePropertyDescriptor` | 需 Isolate + ReadOnlyRoots (true/false/undefined/the_hole) | v8-runtime, v8-roots |
| FI-IC | field-index-inl.h:41 | `FieldIndex::ForSmiLoadHandler` | 依赖 LoadHandler 位段 (IsInobject/StorageOffsetInWords/IsDouble), 定义于 handler-configuration.h | v8-ic (LoadHandler 位段), v8-heap-core (PropertyArray 偏移) |
| PDO-WIT | property-descriptor-object-inl.h:23 | `PropertyDescriptorObject::new` | 需 AllocationWitness → ReadOnlyRoots.the_hole_value / property_descriptor_object_map + 分配 | v8-roots, v8-heap-core |
| PI-ISO | prototype-inl.h:19 | `PrototypeIterator::new_handle` | 需 Isolate + IndirectHandle<JSReceiver> + factory | v8-runtime, v8-heap-core |
| PI-ISO | prototype-inl.h:32 | `PrototypeIterator::new_tagged` | 需 Isolate + Tagged<JSReceiver> + Advance | v8-runtime, v8-objects-js-core |
| PI-ISO | prototype-inl.h:44 | `PrototypeIterator::new_map_tagged` | 需 Isolate + GetPrototypeChainRootMap + prototype | v8-runtime, v8-objects-js-core |
| PI-ISO | prototype-inl.h:58 | `PrototypeIterator::new_map_handle` | 需 Isolate + DirectHandle<Map> + handle 构造 | v8-runtime, v8-heap-core |
| PI-ISO | prototype-inl.h:74 | `PrototypeIterator::has_access` | 需 IsAccessCheckNeeded + Isolate::MayAccess | v8-runtime |
| PI-ISO | prototype-inl.h:85 | `PrototypeIterator::advance` | 需 IsJSProxy + ReadOnlyRoots/factory null_value | v8-runtime, v8-builtins-proxy-reflect, v8-roots |
| PI-ISO | prototype-inl.h:98 | `PrototypeIterator::advance_ignoring_proxies` | 需 JSPrototype::map + Map::prototype + handle(prototype, isolate) | v8-runtime, v8-objects-js-core |
| PI-ISO | prototype-inl.h:113 | `PrototypeIterator::advance_following_proxies` | 需 HasAccess (Isolate::MayAccess/native_context) + JSProxy::GetPrototype (可抛异常) | v8-runtime, v8-builtins-proxy-reflect |
| PI-ISO | prototype-inl.h:125 | `PrototypeIterator::advance_following_proxies_ignoring_access_checks` | 需 JSProxy::kMaxIterationLimit + JSProxy::GetPrototype | v8-runtime, v8-builtins-proxy-reflect |
| DC-ISO | dependent-code-inl.h:22 | `DeordinateDependencyGroups` (raw 转发) | 需 Isolate + DependentCode 共享/只读判定 + 码失效 | v8-runtime, v8-heap-core, v8-deoptimizer |
| DC-ISO | dependent-code-inl.h:24 | `DeoptimizeDependencyGroups` (Tagged) | 需 Isolate + HeapLayout 共享/只读空间判定 + Code 遍历 | v8-runtime, v8-heap-core, v8-deoptimizer |
| DC-ISO | dependent-code-inl.h:41 | `MarkCodeForDeoptimization` | 需 Isolate + DependentCode 链表 + Code::MarkForDeoptimization | v8-runtime, v8-heap-core, v8-deoptimizer |
| MD-BD | megadom-handler.h:42 | `ObjectTraits<MegaDomHandler>::BodyDescriptor` | 需 FixedWeakBodyDescriptor 弱引用扫描起点 (offsetof accessor_) | v8-heap-gc |
| MD-PR | megadom-handler.h:32-35 | `MegaDomHandler::BriefPrintDetails/DECL_PRINTER/DECL_VERIFIER` | 诊断输出与堆校验宏 | v8-diagnostics |
| TR-ISO | transitions.cc (剩余 29 方法) | `TransitionsAccessor`/`TransitionArray` handlified 面 | clang -E 生成的 usize 签名骨架存档; **语义面已由 `crate::transitions` 用 Tagged/Name/Map 类型 1:1 实现** (search_transition/insert/sort/search/for_each/traverse/target_details/is_matching_map)。本表仅覆盖需 Isolate+DirectHandle (可变句柄)+堆分配 (WeakFixedArray 原型过渡扩容)+写屏障的 handlified 分支。**波 10 W10MAPS 已就地填体 7 个** (:17 GetSimpleTransition/:333 CanHaveMoreTransitions/:517 SetNumberOfPrototypeTransitions/:554 SetMigrationTarget/:572·:588 ReplaceTransitions×2 按 usize↔Tagged 地址契约委托 crate::transitions; :893 operator<< 纯枚举→串), 断言转 `tests/test_transitions.rs` | v8-runtime (Isolate), v8-heap-core (分配/写屏障), v8-ic (原型过渡) |
| MU-ISO | map-updater.cc (剩余 31 方法) | `MapUpdater` .cc 面 | clang -E usize 签名骨架存档; **可脱离 Isolate 的语义面已由 `crate::map_updater::MapUpdater` 用 Tagged 类型 1:1 实现** (find_root_map/find_field_owner/generalize_field/reconfigure_to_data_field/try_reconfigure_to_data_field_inplace/reconfigure_elements_kind/complete_inobject_slack_tracking)。本表覆盖需 Isolate+DirectHandle<Map>/FieldType+过渡树迁移状态机 (Update/UpdateImpl/FindTargetMap/ConstructNewMap/BuildDescriptorArray/Normalize/ApplyPrototypeTransition 等) 的 handlified 分支。**波 10 W10MAPS 已就地填体 1 个** (:27 EqualImmutableValues 纯 Tagged 相等比较), 断言转 `tests/objects_maps_unittest.rs` | v8-runtime (Isolate), v8-heap-core (Map/FieldType 分配), v8-ic (过渡) |
| MU-ISO | map-updater.cc:33 | `GeneralizeFieldType` | 需 DirectHandle<FieldType> + isolate | v8-runtime, v8-heap-core |
| MU-ISO | map-updater.cc:41 | `PrintGeneralization` | 诊断输出, 依赖 FILE*/Isolate | v8-diagnostics, v8-runtime |

## 冻结 checklist 关联

- 12.1 函数级对账: 上述条目即未填体登记面, 0 未登记 (全部登记于本表)。
- 波 10 W10MAPS 填体 8 件 (TR-ISO 7 + MU-ISO 1), 已按头部规则从本表移除并转 tests 断言; 其余 todo! 78 实调用 (骨架含 2 处文档注释提及, 共 80 处置) 全部登记于本表。
- 解锁后逐条重开: 依赖 crate 就位即把对应 `todo!` 填体、从本表移除、补 1:1 tests。

