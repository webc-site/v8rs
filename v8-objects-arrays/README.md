[English](#en) | [中文](#zh)

---

<a name="en"></a>

#

---

<a name="zh"></a>

# v8-objects-arrays 推迟台账 (deferred ledger)

函数级对账 (todo.md 十二.1) 与延期登记 (十二.6)。每个仍留 `todo!` 的接口在此登记:
原版 `file:line` + 前置条件 + 解锁 crate。填体完成后从本表移除并转 tests 断言。

- [推迟条目](#推迟条目)
- [冻结 checklist 关联](#冻结-checklist-关联)

## 推迟条目

| 组 ID | 原版 file:line | 符号 | 推迟原因 (前置条件) | 解锁 crate |
|---|---|---|---|---|
| ELEM-ISO | elements.h:259 / elements.cc:5942 | `ArrayConstructInitializeElements` | 需 Isolate + Factory + JSArray/JSObject / JavaScriptArguments 堆对象构造 | v8-runtime, v8-heap-core, v8-objects-js-core |
| ELEM-CSA | elements.h:266 / elements.cc:6048 | `CopyFastNumberJSArrayElementsToTypedArray` | CSA trampoline, 需 Context, JSArray, JSTypedArray 堆对象定义 | v8-runtime, v8-objects-js-core, v8-builtins-typedarray |
| ELEM-CSA | elements.h:272 / elements.cc:6072 | `CopyTypedArrayElementsToTypedArray` | CSA trampoline, 需 JSTypedArray 堆对象定义 | v8-runtime, v8-builtins-typedarray |
| ELEM-CSA | elements.h:276 / elements.cc:6093 | `CopyTypedArrayElementsSlice` | CSA trampoline, 需 JSTypedArray 堆对象定义与多态访问器调度 | v8-runtime, v8-builtins-typedarray |
| ELEM-ISO | elements.cc:202 | `ThrowArrayLengthRangeError` | 需 Isolate + MessageTemplate + RangeError 异常体系 | v8-runtime, v8-execution-isolate |
| ELEM-DICT | elements.cc:264 | `CopyDictionaryToObjectElements` | 需 NumberDictionary::FindEntry 字典哈希表查找与 Isolate | v8-objects-collections, v8-runtime |
| ELEM-ISO | elements.cc:314 | `CopyDoubleToObjectElements` | 需 Isolate + HandleScope 分配 HeapNumber 对象与写屏障 | v8-runtime, v8-heap-core |
| ELEM-DICT | elements.cc:528 | `CopyDictionaryToDoubleElements` | 需 NumberDictionary::FindEntry 字典哈希表查找与 Isolate | v8-objects-collections, v8-runtime |
| ELEM-SLOW | elements.cc:596 | `IncludesValueSlowPath` | 需 DirectHandle<JSObject> 原型链遍历与 Isolate 句柄体系 | v8-runtime, v8-objects-js-core |
| ELEM-SLOW | elements.cc:618 | `IndexOfValueSlowPath` | 需 DirectHandle<JSObject> 原型链遍历与 Isolate 句柄体系 | v8-runtime, v8-objects-js-core |
| ELEM-STATIC | elements.cc:6109 | `ElementsAccessor::InitializeOncePerProcess` | 需完整 32 个 ElementsAccessor 子类层次表全局初始化 | v8-runtime, v8-objects-arrays |
| ELEM-STATIC | elements.cc:6138 | `ElementsAccessor::TearDown` | 需完整 ElementsAccessor 实例生命周期管理与全局静态清理 | v8-runtime, v8-objects-arrays |
| ELEM-ISO | elements.cc:6146 | `ElementsAccessor::Concat` | Array.prototype.concat 复杂路径，需 Isolate 及 JSArray 堆分配及原型链查找 | v8-runtime, v8-heap-core, v8-builtins-array |

## 冻结 checklist 关联

- 12.1 函数级对账: 上述条目即未填体登记面, 13 处 todo! 全部 1:1 严密登记于本表。
- 语义落地面: 辅助函数 10 项已全部就地 1:1 填体落地 (IsFloat16RawBitsZero, IsIdentityMapping, GetWriteBarrierMode, SortIndices, CopyObjectToObjectElements, CopyDoubleToDoubleElements, CopySmiToDoubleElements, CopyPackedSmiToDoubleElements, CopyObjectToDoubleElements, CopyFromFloat16BackingStore)，配齐 9 项单元测试。
- 解锁后逐条重开: 依赖 crate 就位即把对应 `todo!` 填体、从本表移除、补 1:1 tests。
