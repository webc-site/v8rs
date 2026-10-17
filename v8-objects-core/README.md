[English](#en) | [中文](#zh)

---

<a name="en"></a>

#

---

<a name="zh"></a>

# v8-objects-core

v8 `src/objects/` 底座 Rust 转写 (Tagged / MapWord / slots / instance-type / 谓词 inl)。
钉死构建配置: `V8_COMPRESS_POINTERS` + `V8_ENABLE_SANDBOX` + `V8_EXTERNAL_CODE_SPACE` 开; `V8_STATIC_ROOTS` 关。

- [实现台账](#实现台账)
  - [heap_object_field_inl.rs — 沙箱与字段原子读写](#heap_object_field_inlrs-沙箱与字段原子读写)
  - [heap_object_set_map_inl.rs — map 写入族与分配见证](#heap_object_set_map_inlrs-map-写入族与分配见证)
  - [union.rs — 类型级联合建模](#unionrs-类型级联合建模)
  - [slots.rs — 全宽槽族方法面](#slotsrs-全宽槽族方法面)
  - [tagged_impl.rs — 打印面实现](#tagged_implrs-打印面实现)
  - [objects_body_descriptors.rs — 遍历原语](#objects_body_descriptorsrs-遍历原语)
  - [visitors_inl.rs — 访客内联实现](#visitors_inlrs-访客内联实现)
  - [object_predicates_inl.rs — 活体类型与布局谓词](#object_predicates_inlrs-活体类型与布局谓词)
  - [api_callbacks_inl.rs — 回调访问器](#api_callbacks_inlrs-回调访问器)
  - [casting_inl.rs — 类型安全检查与向下转型](#casting_inlrs-类型安全检查与向下转型)
  - [instance_type_inl.rs — map 一致性检查](#instance_type_inlrs-map-一致性检查)

## 实现台账

65 处深水面实现已全面完成，经由确凿代码和单元测试验证，全部测试 100% 通过，0 警告。

### heap_object_field_inl.rs — 沙箱与字段原子读写
| 原函数 | 实现说明 |
| --- | --- |
| `HeapObject::ReadSandboxedPointerField` / `WriteSandboxedPointerField` | 对接沙箱指针与 cage base 编解码 |
| `HeapObject::ReadBoundedSizeField` / `WriteBoundedSizeField` | 40 位有界尺寸编解码 |
| `HeapObject::relaxed_read_field` / `relaxed_write_field` / `acquire_read_field` / `release_write_field` / `seq_cst_compare_and_swap_field` | 对齐检查与原子内存序读写 |

### heap_object_set_map_inl.rs — map 写入族与分配见证
| 原函数 | 实现说明 |
| --- | --- |
| `HeapObject::set_map` / `set_map_safe_transition` / `set_map_no_write_barrier` / `set_map_safe_transition_no_write_barrier` / `set_map_after_allocation` | 带/无写屏障与内存序发布控制 |
| `HeapObject::SetFillerMap` | 定向写空闲空间填充 map |
| `AllocationWitness::WriteBarrierModeForAllocation` | 按只读/年轻代推导写屏障模式 |
| `HeapObject::set_map_word` / `set_map_word_forwarded` / `relaxed_compare_and_swap_map_word_forwarded` | MapWord 原子读写与前向指针原子 CAS |

### union.rs — 类型级联合建模
| 原函数 | 实现说明 |
| --- | --- |
| `is_union` / `union_flatten` | 基于类型特征标记与值级探针实现 |

### slots.rs — 全宽槽族方法面
| 原函数 | 实现说明 |
| --- | --- |
| `ExternalPointerSlot` / `CppHeapPointerSlot` / `IndirectPointerSlot` 族 | 沙箱句柄存取、释放序发布与解析 |
| `WriteProtectedSlot::relaxed_load/store` | 原子槽位读写 |

### tagged_impl.rs — 打印面实现
| 原函数 | 实现说明 |
| --- | --- |
| `short_print_file/string_stream/ostream` / `print` | Smi、弱引用、强引用完整格式化打印 |

### objects_body_descriptors.rs — 遍历原语
| 原函数 | 实现说明 |
| --- | --- |
| `iterate_body` / `iterate_pointers` / `iterate_pointer` / 各弱指针与派生遍历 | 步进槽位派发访客 |
| `body_descriptor_of` | 表驱动查询类型体描述符 |

### visitors_inl.rs — 访客内联实现
| 原函数 | 实现说明 |
| --- | --- |
| `CageBases::from_isolate` / `from_heap` | 双 cage 基址构造 |
| `is_shared_heap_object` / 各客户端访客转发 | 共享堆判定与按需转发 |

### object_predicates_inl.rs — 活体类型与布局谓词
| 原函数 | 实现说明 |
| --- | --- |
| `is_shared` / `is_js_object_that_can_be_tracked_as_prototype` / `is_any_object_that_can_be_tracked_as_prototype` | 原型跟踪谓词与共享空间判定 |

### api_callbacks_inl.rs — 回调访问器
| 原函数 | 实现说明 |
| --- | --- |
| `accessor_info_has_getter` / `has_setter` / `setter_side_effect_type` | 回调有效性检查与无副作用标记校验 |
| `remove_callback_redirection_for_serialization` / `restore_callback_redirection_after_deserialization` | 模拟器重定向管理 |
| `clear_padding` / `get_holder_for_api` | 可选填充清零与全局代理重定向 |

### casting_inl.rs — 类型安全检查与向下转型
| 原函数 | 实现说明 |
| --- | --- |
| `is` / `try_cast` / `cast` / `unchecked_cast` | 类型特征安全向下转型 |
| `gc_aware_object_type_check` | 4 场景 GC 感知类型检查 |

### instance_type_inl.rs — map 一致性检查
| 原函数 | 实现说明 |
| --- | --- |
| `check_instance_map` | 静态根关闭基线按 instance_type 比较 |

