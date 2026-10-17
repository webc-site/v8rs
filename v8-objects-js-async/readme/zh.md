# v8-objects-js-async

## 文件族 → 模块映射 (tool 脚本登记)

| 原版文件族 (v8/src/objects/) | 模块 (src/) | 生成脚本 (tool/src/objects/) |
| --- | --- | --- |
| managed.{h,inl.h,cc,tq} | managed.rs | managed.h.js |
| backing-store.{h,cc} | backing_store_cc.rs | backing-store.cc.js |
| value-serializer.{h,cc} (+ value-serializer-skel.h) | value_serializer_skel.rs | value-serializer-skel.h.js |
| option-utils.{h,cc} | option_utils.rs | option-utils.h.js |
| cpp-heap-object-wrapper.{h,-inl.h} | cpp_heap_object_wrapper.rs | cpp-heap-object-wrapper.h.js |
| allocation-site-scopes.{h,-inl.h} | allocation_site_scopes.rs | allocation-site-scopes.h.js |
| regexp-match-info.{-inl.h,cc,tq} | regexp_match_info.rs | regexp-match-info.h.js |
| intl-objects.{h,cc,tq} | intl_objects.rs | intl-objects.h.js |
| promise.{h,-inl.h,tq} / js-promise.{h,-inl.h,tq} | promise.rs / js_promise.rs | (挂载) |
| microtask.{h,-inl.h,tq} | microtask.rs | (挂载) |
| cpp-heap-external-object.{h,tq} | cpp_heap_external_object.rs | (挂载) |
| allocation-site.tq / bigint.tq / oddball / heap-number / hash-seed-wrapper | allocation_site_tq.rs / bigint_tq.rs / oddball.rs / heap_number.rs / hash_seed_wrapper.rs | (挂载) |
| simd.{h,cc} | simd.rs | (挂载) |
| waiter-queue-node | waiter_queue_node.rs | (挂载) |

## 推迟台账 (依赖 Isolate / PageAllocator / Handles / cpp heap pointer / ICU)

原函数名 (V8 C++ 侧) · 前置条件 (缺什么) · 解锁 crate (谁就位后开工)。

### backing-store.cc (13 面, W7O4 已填 5 面: ctor / GetWasmReservationSize / WrapAllocation / EmptyBackingStore / MakeWasmMemoryResizableByJS)

| 原函数名 | 前置条件 | 解锁 crate |
| --- | --- | --- |
| BackingStore::Allocate | Isolate (array_buffer_allocator / counters / heap->AllocateExternalBackingStore) + Sandbox 边界校验 | v8-execution-isolate + v8-sandbox + v8-heap-core |
| BackingStore::SetAllocatorFromIsolate | Isolate::array_buffer_allocator_shared + v8::ArrayBuffer::Allocator + 联合体 placement new | v8-execution-isolate + v8-api |
| BackingStore::TryAllocateAndPartiallyCommitMemory | Isolate + PageAllocator + Heap::allocator::RetryCustomAllocate + Sandbox + GC 触发 | v8-execution-isolate + v8-heap-core + v8-sandbox |
| BackingStore::AllocateWasmMemory | wasm engine + Isolate + 共享内存对象表 | v8-wasm-core + v8-execution-isolate |
| BackingStore::CopyWasmMemory | AllocateWasmMemory 递归 | v8-wasm-core |
| BackingStore::GrowWasmMemoryInPlace | PageAllocator + Isolate + WasmEngine + 原子 CAS + shared memory grow | v8-wasm-core + v8-execution-isolate + v8-base-platform |
| BackingStore::BroadcastSharedWasmMemoryGrow | Isolate + SharedWasmMemoryData::isolates_ + WasmMemoryObject Handle | v8-wasm-core + v8-execution-isolate + v8-handles |
| BackingStore::RemoveSharedWasmMemoryObjects | Isolate + GlobalBackingStoreRegistry (mutex + unordered_map) | v8-wasm-core + v8-execution-isolate |
| BackingStore::UpdateSharedWasmMemoryObjects | Isolate + GlobalBackingStoreRegistry + Handle 遍历 | v8-wasm-core + v8-execution-isolate + v8-handles |
| BackingStore::ResizeInPlace | PageAllocator + JSArrayBuffer::kMaxByteLength + memset + RoundUpToPageSize | v8-objects-js-core (JSArrayBuffer) + v8-base-platform + v8-execution-isolate |
| BackingStore::GrowInPlace | PageAllocator + 原子 CAS + memset + JSArrayBuffer::kMaxByteLength | v8-objects-js-core + v8-base-platform + v8-execution-isolate |
| BackingStore::get_v8_api_array_buffer_allocator | v8::ArrayBuffer::Allocator + TypeSpecificData 联合体 shared_ptr 分支 | v8-api + v8-execution-isolate |
| BackingStore::get_shared_wasm_memory_data | SharedWasmMemoryData + Isolate 集 (vector<Isolate*>) | v8-wasm-core + v8-execution-isolate |

### value-serializer.cc / value-serializer-skel.h (序列化族 5 面; 缓冲级 varint/zigzag/raw bytes 原语已在 W6O4 就位)

| 原函数名 | 前置条件 | 解锁 crate |
| --- | --- | --- |
| ValueSerializer::WriteObject | Isolate + Handle<JSObject> + SerializationTag 分派 (tag→write_*) + 循环引用表 | v8-execution-isolate + v8-handles + v8-objects-js-core |
| ValueDeserializer::ReadObject | Isolate + Handle<Object> + tag 分派 (tag→read_*) + back-ref 表 | v8-execution-isolate + v8-handles + v8-objects-js-core |
| ValueDeserializer::TransferArrayBuffer | JSArrayBuffer + Handle + 带外转移 id 表 | v8-objects-js-core (JSArrayBuffer) + v8-handles |
| ValueDeserializer::ReadJSArrayBuffer | JSArrayBuffer 句柄槽 + BackingStore + Handle | v8-objects-js-core + v8-handles + v8-execution-isolate |
| ValueDeserializer::ReadJSArrayBufferView | JSArrayBufferView 槽位恢复 + Handle | v8-objects-js-core + v8-handles |

### managed.{h,inl.h,cc} (4 面; UpdateEstimatedSize 收支核已就位)

| 原函数名 | 前置条件 | 解锁 crate |
| --- | --- | --- |
| CppGCManagedBase::GetWrapper | ReadCppHeapPointerField (kCppGCManagedTag) + cpp 堆指针表 | v8-sandbox + v8-objects-core (字段访问器) |
| CppGCManagedBase::estimated_size | 依赖 GetWrapper | v8-sandbox + v8-objects-core |
| ManagedObjectFinalizer (managed.cc) | GlobalHandles::Destroy + SetSecondPassCallback + Handle | v8-handles + v8-heap-cppgc |
| ManagedObjectFinalizerSecondPass (managed.cc) | Isolate::UnregisterManagedPtrDestructor + shared_ptr delete + ExternalMemoryAccounter | v8-execution-isolate + v8-handles + v8-heap-cppgc |

### cpp-heap-object-wrapper.h / -inl.h (4 面)

| 原函数名 | 前置条件 | 解锁 crate |
| --- | --- | --- |
| CppHeapObjectWrapper::From | ExternalObject / CppGCManagedBase / JSObject 三路分派 + Isolate + offsetof 表 | v8-objects-core + v8-objects-js-core + v8-execution-isolate |
| InitializeCppHeapWrapper | SetupLazilyInitializedCppHeapPointerField + 惰性初始化 | v8-sandbox |
| GetCppHeapWrappable | ReadCppHeapPointerField + tag 范围校验 | v8-sandbox |
| SetCppHeapWrappable | WriteLazilyInitializedCppHeapPointerField + ForCppHeapPointer 写屏障 | v8-sandbox + v8-heap-cppgc |

### option-utils.h / .cc (4 面; DefaultNumberOption / GetNumberOption / GetNumberOptionAsDouble / GetStringOption\<T\> / GetStringOrBooleanOption\<T\> 纯语义核已就位)

| 原函数名 | 前置条件 | 解锁 crate |
| --- | --- | --- |
| GetOptionsObject | Isolate + factory.new_empty_object + Object 类型判定 + TypeError(kInvalidArgument) 抛出 | v8-execution-isolate + v8-objects-js-core |
| CoerceOptionsToObject | Isolate + Object::ToObject + TypeError 抛出 | v8-execution-isolate + v8-objects-js-core |
| GetStringOption | Isolate + JSReceiver::Get + Object::ToString + 原生 %String% realm | v8-execution-isolate + v8-objects-js-core |
| GetBoolOption | Isolate + JSReceiver::Get + Object::BooleanValue | v8-execution-isolate + v8-objects-js-core |

### intl-objects.h / .cc (7 面, 3344 行 .cc; ICU collator / locale / number format 依赖)

| 原函数名 | 前置条件 | 解锁 crate |
| --- | --- | --- |
| Intl::CanonicalizeLocaleList | Isolate + JSArray + Intl 枚举 + Lookup 匹配 | v8-execution-isolate + v8-objects-js-core + ICU |
| Intl::GetCanonicalLocales | Isolate + JSArray + icu::Locale | ICU + v8-execution-isolate |
| Intl::SupportedValuesOf | icu::Locale + 支持键枚举 + Isolate + JSArray | ICU + v8-execution-isolate |
| Intl::CompareStrings | icu::Collator + Handle<String> + 快路径 | ICU + v8-handles |
| Intl::SetNumberFormatDigitOptions | option-utils Get* + Isolate + Handle<Object> | v8-execution-isolate + v8-objects-js-core |
| Intl::ResolveLocale | icu::Locale + locale matching + Isolate | ICU + v8-execution-isolate |
| Intl::ToLanguageTag | icu::Locale → BCP 47 序列化 | ICU |

### allocation-site-scopes.h / -inl.h (4 面)

| 原函数名 | 前置条件 | 解锁 crate |
| --- | --- | --- |
| AllocationSiteContext::InitializeTraversal | Isolate + Handle<AllocationSite> top/current 双游标 | v8-handles + v8-execution-isolate + v8-objects-maps (AllocationSite) |
| AllocationSiteUsageContext::EnterNewScope | Handle<AllocationSite> + nested_site 推进 | v8-handles + v8-objects-maps |
| AllocationSiteUsageContext::ExitScope | boilerplate DCHECK + Handle 弹出 | v8-handles + v8-objects-maps |
| AllocationSiteUsageContext::ShouldCreateMemento | v8_flags.pretenuring_threshold + CanTrack(instance_type) + ShouldTrack(elements_kind) | v8-flags + v8-objects-maps |

### regexp-match-info.{-inl.h,cc,tq} (2 面; capture 槽位公式已就位)

| 原函数名 | 前置条件 | 解锁 crate |
| --- | --- | --- |
| RegExpMatchInfo::New | Isolate + FixedArray alloc + MemsetTagged + 空 subject/input | v8-execution-isolate + v8-objects-arrays |
| RegExpMatchInfo::ReserveCaptures | Isolate + CopyElements + 新建 Handle + 平移 | v8-execution-isolate + v8-objects-arrays + v8-handles |

## 已填语义体面 (W7O4)

- **backing_store_cc.rs** (5): `BackingStore::new` ctor (含 8 条 DCHECK/CHECK 与 flags 落位) / `get_wasm_reservation_size` 公式 / `wrap_allocation` (含 EmptyDeleter fn 指针识别) / `empty_backing_store` / `make_wasm_memory_resizable_by_js`; 附 backing-store.h inline 访问器面 (is_shared / is_resizable_by_js / is_immutable / is_wasm_memory / is_wasm_memory64 / has_guard_regions / wasm_memory_flag / byte_length / max_byte_length / byte_capacity / id / is_empty / per_isolate_accounting_length) 及 Flag 位标 / WasmMemoryFlag / DeleterCallback 类型承接。
- 单元测试覆盖: reservation 三分支 (wasm32 全 guard / wasm64 + arm64 附加页 / 非 guard 直返) / EmptyBackingStore / WrapAllocation 双 deleter / 位标翻转 / id 单调 / AllocationStatus 与 Flag 常量对数。

## 收笼计数

| 阶段 | 基线 | 现值 | 差 |
| --- | --- | --- | --- |
| W6O4 → W6O4b 差集 (70 → 48) | 70 | 48 | -22 |
| W7O4 本轮填体 (48 → 43) | 48 | 43 | -5 |
| W11JA 补波 (43 → 43) | 43 | 43 | 0 |
| 剩余推迟台账 | — | 43 | 见上表 |

## W11JA 补波说明 (2026-10-13)

- 实测盘点: 43 todo! 全部深水系 (Isolate / Handle / v8-sandbox cpp 堆指针表 / v8-heap-cppgc / ICU / PageAllocator / Wasm 引擎), 均在本 crate 依赖集 (v8-objects-arrays / core / js-core / js-func / maps) 之外; 兄弟 objects crate 一律不链 handles/isolate/sandbox, 此为分层常态。实现差集 = 0, 不动冻结。
- 本轮只补合规差集:
  - 43 处 todo! 消息全部规范化为 `原函数名: 前置; 解锁 crate` 带参格式 (依上表台账逐条落位); 之前 intl_objects 7 面缺函数名、多面缺「解锁 crate」。
  - 深水 stub 补 `/// v8: src/objects/<file>.{h,cc,inl.h} Class::method` 锚 (intl/option/cpp-heap/allocation-site/regexp/value-serializer)。
  - 纯语义核补测试 (原为已实现但零覆盖): managed 外部内存收支核 / option-utils ECMA402 纯核 / value_serializer 缓冲级 varint·zigzag·double·raw 线上往返。
- 脚本核查: 本族 tool/src/objects/*.js 为「生成种子」, 落地文件经多轮就地修订已超集 (WriteHeader/ExpandBuffer/Release 等已填, 复跑会回退); 依「不推翻冻结」红线, 未重跑种子脚本, 保持落地版为权威。脚本为纯函数输入, 输出确定 (结构幂等)。
- 门禁: clippy `--all-targets -D warnings` 0 警告; nextest 10/10 全绿 (7 基线 + 3 新增)。
