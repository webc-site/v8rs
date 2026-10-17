// 由 tool/src/wasm/wasm-objects.h.js 从 v8/src/wasm/wasm-objects.{h,tq} 生成; 修订就地改, 复跑覆盖
// Wasm 对象布局常量、枚举与 torque 类清单

/// 挂起恢复行为 (对应原版 enum class OnResume : int)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum OnResume {
  /// kContinue
  Continue = 0,
  /// kThrow
  Throw = 1,
}

/// 字符串内部化标记 (对应原版 InternalizeString)
pub type InternalizeString = bool;
/// kInternalize
pub const INTERNALIZE: InternalizeString = true;
/// kNoInternalize
pub const NO_INTERNALIZE: InternalizeString = false;

/// 内存对象无最大页数哨兵 (原版 WasmMemoryObject::kNoMaximum)
pub const MEMORY_NO_MAXIMUM: i32 = -1;

/// 反馈常量布局 (对应原版 FeedbackConstants)
pub mod feedback_constants {
  pub const HEADER_SLOTS: i32 = 1;
  pub const SLOTS_PER_INSTRUCTION: i32 = 2;
}

/// 调度表条目写入情境 (对应原版 WasmDispatchTable::NewOrExistingEntry)
/// (整改 R5W2 F14a) 对译依据: 原版 v8/src/wasm/wasm-objects.h:969
/// `enum NewOrExistingEntry : bool { kNewEntry, kExistingEntry };` —— C++ bool 底
/// 型枚举器自 0 起隐式递增, 实际取值 kNewEntry=0、kExistingEntry=1; Rust 禁
/// repr(bool) (E0539), 而 bool 布局保证恰为 1 字节取 0/1, 故 repr(u8) + 0/1
/// 判别式与原版逐值同形。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum NewOrExistingEntry {
  /// kNewEntry
  NewEntry = 0,
  /// kExistingEntry
  ExistingEntry = 1,
}

/// WasmDispatchTable 布局 (公式登记, 依赖布局参数, 不折叠 — Developer 侧按
/// 构建配置代入: EXPOSED_TRUSTED_OBJECT_SIZE / K_UINT32_SIZE / K_TAGGED_SIZE /
/// TAGGED_SIZE_8_BYTES)
pub mod dispatch_table_layout {
  pub const K_UINT32_SIZE: usize = 4;
  pub const K_TAGGED_SIZE: usize = 4; // 指针压缩开; 关闭时为 8
  pub const TAGGED_SIZE_8_BYTES: bool = false; // 压缩开 → false
  /// 布局参数: sizeof(ExposedTrustedObject)。
  /// A-1 深水哨兵 (整改 R5W2 F14c): 原 todo!() 在 const 上下文被禁 (E0080),
  /// 改登记为可在 const 上下文折叠的显式占位 0。
  /// 硬约束: 本模块接线进 lib.rs 前, 必须先把占位 0 替换为
  /// sizeof(ExposedTrustedObject) 真值, 未替值不得接线 —— 真值不可从原版
  /// wasm-objects.{h,tq} 径证: tq:37 `extern class
  /// WasmTrustedInstanceData extends ExposedTrustedObject;` 无类体,
  /// h:973 仅登记公式形; 取值依赖堆对象布局件 (v8-heap-base/
  /// v8-objects-core) 与指针压缩/SANDBOX 构建开关。见 v8-wasm-objects
  /// readme A-1 台账 (双语已登「必先替值后接线」)。
  pub const EXPOSED_TRUSTED_OBJECT_SIZE: usize = 0; // A-1 占位: 接线前必替换真值

  /// kLengthOffset = sizeof(ExposedTrustedObject)
  pub const LENGTH_OFFSET: usize = EXPOSED_TRUSTED_OBJECT_SIZE;
  pub const CAPACITY_OFFSET: usize = LENGTH_OFFSET + K_UINT32_SIZE;
  pub const PROTECTED_OFFHEAP_DATA_OFFSET: usize = CAPACITY_OFFSET + K_UINT32_SIZE;
  pub const PROTECTED_USES_OFFSET: usize = PROTECTED_OFFHEAP_DATA_OFFSET + K_TAGGED_SIZE;
  pub const TABLE_TYPE_OFFSET: usize = PROTECTED_USES_OFFSET + K_TAGGED_SIZE;
  pub const PADDING_SIZE: usize = if TAGGED_SIZE_8_BYTES { K_UINT32_SIZE } else { 0 };
  pub const ENTRIES_OFFSET: usize = TABLE_TYPE_OFFSET + K_UINT32_SIZE + PADDING_SIZE;
  pub const HEADER_SIZE: usize = ENTRIES_OFFSET;
  /// 条目: [target(u32) | sig(u32) | implicit_arg(tagged)]; DRUMBRAKE 开时 target 位放 function_index
  pub const TARGET_BIAS: usize = 0;
  pub const SIG_BIAS: usize = TARGET_BIAS + K_UINT32_SIZE;
  pub const IMPLICIT_ARG_BIAS: usize = SIG_BIAS + K_UINT32_SIZE;
  pub const ENTRY_SIZE: usize = IMPLICIT_ARG_BIAS + K_TAGGED_SIZE;
}

/// WasmTrustedInstanceData tagged 字段名单 (原版 WASM_TAGGED_INSTANCE_DATA_FIELDS)
/// 偏移由 torque 生成, 此处只登记 1:1 名单
pub const TAGGED_INSTANCE_FIELDS: &[(&str, &str)] = &[
  ("kInstanceObjectOffset", "instance_object"),
  ("kNativeContextOffset", "native_context"),
  ("kMemoryObjectsOffset", "memory_objects"),
  ("kUntaggedGlobalsBufferOffset", "untagged_globals_buffer"),
  ("kTaggedGlobalsBufferOffset", "tagged_globals_buffer"),
  ("kImportedMutableGlobalsBuffersOffset", "imported_mutable_globals_buffers"),
  ("kImportedMutableGlobalsOffsetsOffset", "imported_mutable_globals_offsets"),
  ("kTablesOffset", "tables"),
  ("kFuncRefsOffset", "func_refs"),
  ("kManagedObjectMapsOffset", "managed_object_maps"),
  ("kFeedbackVectorsOffset", "feedback_vectors"),
  ("kWellKnownImportsOffset", "well_known_imports"),
  // 仅 V8_ENABLE_DRUMBRAKE: ("kImportedFunctionIndicesOffset", "imported_function_indices"),
  // v8: wasm-objects.h:734-735 IF_WASM_DRUMBRAKE 位在 element_segments(:736) 之前, 顺序敏感数组须保持此序
  ("kElementSegmentsOffset", "element_segments"),
];

/// WasmTrustedInstanceData protected 字段名单 (原版 WASM_PROTECTED_INSTANCE_DATA_FIELDS)
pub const PROTECTED_INSTANCE_FIELDS: &[(&str, &str)] = &[
  ("kProtectedMemoryBasesAndSizesOffset", "memory_bases_and_sizes"),
  ("kProtectedSharedMemoryBackingStoresOffset", "shared_memory_backing_stores"),
  ("kProtectedDataSegmentsOffset", "data_segments"),
  ("kProtectedDispatchTable0Offset", "dispatch_table0"),
  ("kProtectedDispatchTablesOffset", "dispatch_tables"),
  ("kProtectedDispatchTableForImportsOffset", "dispatch_table_for_imports"),
  ("kProtectedTagsTableOffset", "tags_table"),
  // 仅 V8_ENABLE_DRUMBRAKE: ("kProtectedInterpreterHandleOffset", "interpreter_handle"),
  // v8: wasm-objects.h:745-746 IF_WASM_DRUMBRAKE 位在 managed_native_module(:747) 之前, 顺序敏感数组须保持此序 (复审 WOR2 F7, 同 TAGGED 侧规则)
  ("kProtectedManagedNativeModuleOffset", "managed_native_module"),
];

/// 字段计数 (原版 kTaggedFieldsCount / kProtectedFieldsCount, PLUS_ONE 展开)
/// DRUMBRAKE 关闭态
pub const TAGGED_FIELDS_COUNT: usize = 13;
pub const TAGGED_FIELDS_COUNT_DRUMBRAKE: usize = 14;
pub const PROTECTED_FIELDS_COUNT: usize = 8;
pub const PROTECTED_FIELDS_COUNT_DRUMBRAKE: usize = 9;

/// torque 类清单 (原版 wasm-objects.tq, extern class 1:1)
pub const TQ_CLASSES: &[&str] = &[
  "WasmTrustedInstanceData",
  "WasmInstanceObject",
  "WasmImportData",
  "WasmFastApiCallData",
  "WasmInternalFunction",
  "WasmFuncRef",
  "WasmFunctionData",
  "WasmExportedFunctionData",
  "WasmCapiFunctionData",
  "WasmResumeData",
  "WasmSuspenderObject",
  "WasmContinuationObject",
  "WasmStackObject",
  "WasmExceptionTag",
  "WasmExceptionPackage",
  "WasmModuleObject",
  "WasmDispatchTable",
  "WasmDispatchTableForImports",
  "WasmTableObject",
  "WasmMemoryObject",
  "WasmGlobalObject",
  "WasmTagObject",
  "WasmTypeInfo",
  "WasmObject",
  "WasmStruct",
  "WasmArray",
  "WasmCustomMap",
  "WasmCustomMapWrapper",
  "WasmStringViewIter",
  "WasmNull",
  "WasmSuspendingObject",
];

// —— 提取不变式断言 ——
// (整改 R5W2 F14b) 原形 TQ_CLASSES.contains(...) 处于 const 上下文违 E0015
// (slice::contains 非 const fn)。选型: 保持编译期折叠 —— 块内 const fn 以
// while 循环逐元素逐字节比较替代 contains (不降级为运行时 #[cfg(test)] 断言,
// 以免断言面削弱为仅测试期生效); 口径与 readme 工具链登记 (本版 nightly const
// 上下文受限, 不变式统一 assert! 折叠) 同源, 该类补登于彼。
const _: () = {
  const fn tq_contains(classes: &[&str], name: &str) -> bool {
    let mut i = 0;
    while i < classes.len() {
      let a = classes[i].as_bytes();
      let b = name.as_bytes();
      if a.len() == b.len() {
        let mut j = 0;
        let mut same = true;
        while j < a.len() {
          if a[j] != b[j] {
            same = false;
          }
          j += 1;
        }
        if same {
          return true;
        }
      }
      i += 1;
    }
    false
  }
  assert!(MEMORY_NO_MAXIMUM == -1);
  assert!(feedback_constants::HEADER_SLOTS == 1);
  assert!(feedback_constants::SLOTS_PER_INSTRUCTION == 2);
  // 修订 (R5W): 原骨架误写 TAGGED_FIELDS_COUNT == 14; 原版 PLUS_ONE 展开
  // 非 DRUMBRAKE 态 13 项 (wasm-objects.h:721-736 WASM_TAGGED_INSTANCE_DATA_FIELDS)
  assert!(TAGGED_FIELDS_COUNT == 13);
  assert!(TAGGED_FIELDS_COUNT_DRUMBRAKE == 14);
  assert!(PROTECTED_FIELDS_COUNT == 8);
  assert!(tq_contains(TQ_CLASSES, "WasmTrustedInstanceData"));
  assert!(tq_contains(TQ_CLASSES, "WasmDispatchTable"));
};
