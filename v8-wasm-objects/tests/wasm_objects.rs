//! wasm-objects 纯逻辑面 1:1 复刻用例。
//! 原版对译来源:
//! - v8/test/cctest/wasm/test-grow-memory.cc 等 wasm-objects 相关用例全部依赖
//!   WasmRunner(Isolate), 无法等价迁移 → 已登推迟台账 (readme/zh.md)。
//! - 本文件用例从原版 wasm-objects.cc 各纯逻辑片段的 static_assert/CHECK/DCHECK
//!   语义与注释数值算例逐条提取 (锚点标注在断言旁)。
//!
//! 常量防线以 const 块编译期折叠 (本版工具链 const 上下文不支持 assert_eq!)。
use aok::{OK, Void};
use v8_wasm_core::{
  value_type::{CanonicalTypeIndex, CanonicalValueType, GenericKind, ValueKind, ValueType},
  wasm_limits::{K_SPEC_MAX_MEMORY32_PAGES, K_SPEC_MAX_MEMORY64_PAGES, K_V8_MAX_WASM_TABLE_SIZE},
};
use v8_wasm_objects::{
  BASES_AND_SIZES_LAYOUT, CODE_POINTER_WIDTH, CODE_SPACE_WRITE_REASON, CallTargetSource,
  CodeSpaceWriteScope, DispatchEntry, DispatchNewOrExisting, DispatchTableModel, EntryWrite,
  FuncRefSlot, GROW_FAIL_PATHS, GROW_REPORT_ROUNDING, GROW_SUCCESS_RETURNS, GlobalStorageSel,
  GlobalValueRead, INL_ACCESSOR_MACROS, ImplicitArgSource, InitTableEntriesError,
  K_BYTES_PER_EXCEPTION_VALUES_ARRAY_ELEMENT, K_GB_PAGES, K_GB_PAGES_FORMULA, K_NO_MAXIMUM,
  MEMORY_RESERVE_HEURISTIC, MUST_GROW_IN_PLACE, MemoryChangeResizabilityStep, MemoryGrowParams,
  SetOp, SetRawMemoryParams, SetWrapperParams, TABLE_GROW_FAIL_PATHS, TableCopyOp, TableGetOp,
  TableGrowOutcome, TableSetOp, USES_ENTRY_STEP, USES_INITIAL_LENGTH, USES_RESERVED_SLOT_OFFSET,
  UpdateMultipleKind, UseEntry, WasmWrapperMap, decode_i32_exception_value,
  decode_i64_exception_value, dispatchtable_adduse, dispatchtable_clear, dispatchtable_grow,
  dispatchtable_grow_amount, dispatchtable_maybegrowuseslist, dispatchtable_setfornonwrapper,
  dispatchtable_setforwrapper, dispatchtable_uses_min_free_entries,
  dispatchtable_uses_new_capacity, dispatchtabledata_add, dispatchtabledata_remove,
  dispatchtabledata_wrappercodepointerfordebugging, dispatchtableforimports_setfornonwrapper,
  dispatchtableforimports_setforwrapper, encode_i32_exception_value, encode_i64_exception_value,
  exceptionpackage_getencodedsize, exceptionpackage_getencodedsize_for_tag,
  externalfunction_iswasmexternalfunction, function_sig_matches_table,
  memoryobject_changearraybufferresizability, memoryobject_fixupresizablearraybuffer,
  memoryobject_getarraybuffer, memoryobject_grow, memoryobject_grow_exceeds, memoryobject_max_size,
  memoryobject_must_grow_in_place, memoryobject_new_reserved_pages, memoryobject_refreshbuffer,
  memoryobject_setnewbuffer, memoryobject_updateinstances, memoryobject_useininstance,
  min_growth_pages, moduleobject_extractutf8stringfrommodulebytes,
  moduleobject_getfunctionnameornull, moduleobject_getmodulenameornull,
  moduleobject_getrawfunctionname, new_capacity_pages, tableobject_entries_grow_delta,
  tableobject_fill, tableobject_get, tableobject_grow, tableobject_grow_exceeds,
  tableobject_grow_max_size, tableobject_new, tableobject_set,
  tableobject_setfunctiontableplaceholder, tableobject_updatedispatchtable,
  tableobject_updatedispatchtable_batch, trustedinstancedata_copytableentries,
  trustedinstancedata_getcalltarget, trustedinstancedata_getglobalstorage,
  trustedinstancedata_getglobalvalue, trustedinstancedata_initdatasegmentarrays,
  trustedinstancedata_inittableentries, trustedinstancedata_setrawmemory,
  trustedinstancedata_try_get_func_ref,
};

#[ctor::ctor(unsafe)]
fn _log_init() {
  log_init::init();
}

// —— 编译期常量防线 (原版锚点见 src 对应 doc) ——
const _: () = {
  // v8: src/wasm/wasm-objects.cc:858 kGBPages = 1024^3 / kWasmPageSize
  assert!(K_GB_PAGES == 16384);
  // v8: src/wasm/wasm-objects.h:477 kNoMaximum
  assert!(K_NO_MAXIMUM == -1);
  // v8: src/wasm/wasm-objects.cc:74/2508/2499 uses 列表布局
  assert!(USES_RESERVED_SLOT_OFFSET == 1);
  assert!(USES_INITIAL_LENGTH == 3);
  assert!(USES_ENTRY_STEP == 2);
  // v8: src/wasm/wasm-objects.cc:2790 kBytesPerExceptionValuesArrayElement
  assert!(K_BYTES_PER_EXCEPTION_VALUES_ARRAY_ELEMENT == 2);
  assert!(CODE_POINTER_WIDTH == 4);
  assert!(INL_ACCESSOR_MACROS == 21);
  assert!(GROW_FAIL_PATHS == 4);
  assert!(TABLE_GROW_FAIL_PATHS == 1);
  // v8: src/wasm/wasm-objects.cc:1087-1089 / 855-881 名单长度
  assert!(MUST_GROW_IN_PLACE.len() == 4);
  assert!(MEMORY_RESERVE_HEURISTIC.len() == 4);
};

/// v8: src/wasm/wasm-objects.cc:858 1GB 页数公式串
#[test]
fn gb_pages_formula() -> Void {
  assert_eq!(K_GB_PAGES_FORMULA, "1024 * 1024 * 1024 / kWasmPageSize");
  assert_eq!(CODE_SPACE_WRITE_REASON, "For wasm::CodeSpaceWriteScope.");
  OK
}

/// v8: src/wasm/wasm-objects.cc:1171-1175 拷贝增长步进算例 (注释: 0.5MB + 12.5%)
#[test]
fn min_and_new_capacity() -> Void {
  assert!(min_growth_pages(0) == 8); // 8 页 = 0.5MB
  assert!(min_growth_pages(64) == 80); // 64 + 8 + 8
  assert!(min_growth_pages(1000) == 1133); // 1000 + 8 + 125
  // 先 min_growth 后 max_pages 的次序算例 (.cc:1172-1175 注释)
  // min(100, max(20, 80)) = 80 — 骨架看板原记 100 为误抽, 已按公式修正
  assert!(new_capacity_pages(100, 20, 64) == 80);
  assert!(new_capacity_pages(200, 20, 64) == 80); // 取 min_growth 下限
  assert!(new_capacity_pages(200, 150, 64) == 150); // 需求高于步进下限
  // 可反转钉: min_growth(80) 越 max_pages(50) 时封顶取 50; 颠倒实现 min→max 序给 80 (复审 WOR5 F11)
  assert!(new_capacity_pages(50, 20, 64) == 50);
  OK
}

/// v8: src/wasm/wasm-objects.cc:847-885 WasmMemoryObject::New 预留启发式 (64 位分支)
#[test]
fn memory_new_reserved_pages() -> Void {
  let engine32 = K_SPEC_MAX_MEMORY32_PAGES; // 65536
  // initial 超引擎上限 → 原版返回空句柄 (.cc:853)
  assert!(memoryobject_new_reserved_pages(70000, K_NO_MAXIMUM, false, engine32).is_none());
  // 有声明最大 → min(engine, maximum) (.cc:883-884)
  assert!(memoryobject_new_reserved_pages(10, 100, false, engine32) == Some(100));
  assert!(memoryobject_new_reserved_pages(10, 70000, true, engine32) == Some(engine32));
  // 无声明最大 (含共享) → 引擎上限 (.cc:884)
  assert!(memoryobject_new_reserved_pages(10, K_NO_MAXIMUM, true, engine32) == Some(engine32));
  // memory64 由调用方代入 engine64 上限 (.cc:849-851)
  let engine64 = K_SPEC_MAX_MEMORY64_PAGES; // 262144
  assert!(memoryobject_new_reserved_pages(100000, K_NO_MAXIMUM, false, engine64) == Some(engine64));
  OK
}

/// v8: src/wasm/wasm-objects.cc:1078-1089 内存 Grow 上限/越界/原地判定
#[test]
fn memory_grow_predicates() -> Void {
  // GetMaxSize 面: 无最大取引擎; 有最大取小 (.cc:1078-1083)
  assert!(memoryobject_max_size(K_NO_MAXIMUM, 65536) == 65536);
  assert!(memoryobject_max_size(100, 65536) == 100);
  assert!(memoryobject_max_size(70000, 65536) == 65536);
  // 谓词 has_maximum_pages() = >= 0 (-inl.h:1231): 域外负值判无最大取引擎 (复审 WOR4 F10 钉死)
  assert!(memoryobject_max_size(-5, 65536) == 65536);
  // pages > max_pages - old_pages (.cc:1085)
  assert!(!memoryobject_grow_exceeds(10, 10, 20));
  assert!(memoryobject_grow_exceeds(10, 11, 20));
  assert!(memoryobject_grow_exceeds(0, 1, 0));
  // 四条件析取 (.cc:1087-1089, 与 MUST_GROW_IN_PLACE 名单一致)
  assert!(memoryobject_must_grow_in_place(true, false, false, 1));
  assert!(memoryobject_must_grow_in_place(false, true, false, 1));
  assert!(memoryobject_must_grow_in_place(false, false, true, 1));
  assert!(memoryobject_must_grow_in_place(false, false, false, 0)); // pages==0 退化原地
  assert!(!memoryobject_must_grow_in_place(false, false, false, 1));
  OK
}

/// v8: src/wasm/wasm-objects.cc:301-318 WasmTableObject::Grow 上限与扩容指数面
#[test]
fn table_grow_arithmetic() -> Void {
  let engine = K_V8_MAX_WASM_TABLE_SIZE as u64; // 1e7
  assert!(tableobject_grow_max_size(None, engine) as u64 == engine);
  assert!(tableobject_grow_max_size(Some(64), engine) == 64);
  assert!(tableobject_grow_exceeds(90, 11, 100));
  assert!(!tableobject_grow_exceeds(90, 10, 100));
  // 缺口 1 也按旧容量指数扩 (.cc:314-316)
  assert!(tableobject_entries_grow_delta(11, 10, 100) == Some(10));
  assert!(tableobject_entries_grow_delta(50, 10, 100) == Some(40));
  assert!(tableobject_entries_grow_delta(95, 50, 100) == Some(50)); // min(max(45,50), 50)
  assert!(tableobject_entries_grow_delta(10, 10, 100).is_none()); // 未超旧容量不扩
  // 封顶腿鉴别钉 (复审 WOR7 F15, .cc:318 Never grow larger than the max size): 删 .min 腿给 100 必红
  assert!(tableobject_entries_grow_delta(140, 100, 150) == Some(50));
  // min 饱和鉴别钉 (WOR7 F15, .cc:303-304 std::min): maximum 超引擎上限时封 engine, 删 .min 给 2e7 必红
  assert!(tableobject_grow_max_size(Some(2 * engine), engine) as u64 == engine);
  OK
}

/// v8: src/wasm/wasm-objects.cc:2588-2604 WasmDispatchTable::Grow 容量决策
#[test]
fn dispatch_grow_capacity() -> Void {
  // 至少 8 指数步 (.cc:2601-2603)
  assert!(dispatchtable_grow_amount(0, 1, 1000) == Some(8));
  assert!(dispatchtable_grow_amount(8, 16, 1000) == Some(8));
  assert!(dispatchtable_grow_amount(100, 101, 1000) == Some(100));
  // 未超旧容量 → 原版原地写 length (.cc:2588-2591)
  assert!(dispatchtable_grow_amount(50, 10, 1000).is_none());
  // 指数步恰落 [min_grow, max_grow] 界内 (.cc:2603 clamp)
  assert!(dispatchtable_grow_amount(10, 20, 25) == Some(10));
  assert!(dispatchtable_grow_amount(10, 12, 20) == Some(10));
  // 钳位双腿鉴别钉 (复审 WOR7 F15, .cc:2603 std::clamp): 下腿删 .max(min_grow) 给 8 必红; 上腿删 .min(max_grow) 给 100 必红
  assert!(dispatchtable_grow_amount(8, 100, 1000) == Some(92));
  assert!(dispatchtable_grow_amount(100, 101, 150) == Some(50));
  OK
}

/// v8: src/wasm/wasm-objects.cc:2503-2546 MaybeGrowUsesList 压实/扩容算术面
#[test]
fn dispatchtable_uses_list() -> Void {
  assert!(dispatchtable_uses_min_free_entries(8) == 3); // 1 + 8/4
  assert!(dispatchtable_uses_min_free_entries(0) == 1);
  assert!(dispatchtable_uses_new_capacity(4) == 7); // 2 条目 → 3 条目 * 2 + 1
  assert!(dispatchtable_uses_new_capacity(8) == 13); // 4 → 6 条目
  assert!(dispatchtable_uses_new_capacity(1) == 3); // 0 条目 → 至少 1 条目
  OK
}

/// v8: src/wasm/wasm-objects.cc:2813-2844 WasmExceptionPackage::GetEncodedSize
#[test]
fn exception_encoded_size() -> Void {
  assert!(exceptionpackage_getencodedsize(&[]) == 0);
  assert!(exceptionpackage_getencodedsize(&[ValueKind::I32, ValueKind::F32]) == 4);
  assert!(exceptionpackage_getencodedsize(&[ValueKind::I64, ValueKind::F64]) == 8);
  assert!(exceptionpackage_getencodedsize(&[ValueKind::S128]) == 8);
  assert!(exceptionpackage_getencodedsize(&[ValueKind::Ref, ValueKind::RefNull]) == 2);
  // 原版 UNREACHABLE 的 kind 计 0 (禁 panic 兜底, 不可能入异常签名)
  assert!(
    exceptionpackage_getencodedsize(&[
      ValueKind::Void,
      ValueKind::Top,
      ValueKind::Bottom,
      ValueKind::I8,
      ValueKind::I16,
      ValueKind::F16
    ]) == 0
  );
  assert!(
    exceptionpackage_getencodedsize(&[
      ValueKind::I32,
      ValueKind::I64,
      ValueKind::Ref,
      ValueKind::S128,
      ValueKind::F32,
      ValueKind::F64,
      ValueKind::RefNull
    ]) == 22
  );
  OK
}

/// v8: src/wasm/wasm-objects.cc:2738-2764 异常值 16 位槽编解码回环
#[test]
fn exception_value_codec() -> Void {
  // i32: 高低 16 位两槽 (.cc:2740-2741)
  let mut values = [0u32; 2];
  let mut cursor = 0usize;
  encode_i32_exception_value(&mut values, &mut cursor, 0x1234_abcd);
  assert!(values == [0x1234, 0xabcd]);
  assert!(cursor == 2);
  cursor = 0;
  assert!(decode_i32_exception_value(&values, &mut cursor) == Some(0x1234_abcd));
  assert!(cursor == 2);
  // i64: 4 槽, 先高 i32 段后低 i32 段 (.cc:2744-2750)
  let mut wide = [0u32; 4];
  cursor = 0;
  encode_i64_exception_value(&mut wide, &mut cursor, 0xdead_beef_1234_5678);
  assert!(wide == [0xdead, 0xbeef, 0x1234, 0x5678]);
  assert!(cursor == 4);
  cursor = 0;
  assert!(decode_i64_exception_value(&wide, &mut cursor) == Some(0xdead_beef_1234_5678));
  assert!(cursor == 4);
  // 越界防御: decode 不足返回 None 且游标不动
  cursor = 3;
  assert!(decode_i32_exception_value(&wide, &mut cursor).is_none());
  assert!(cursor == 3);
  // encode 越界 (游标后不足 2 槽) → 整段不写但仍推进游标, 与原版容量 DCHECK 前提一致
  cursor = 3;
  encode_i32_exception_value(&mut wide, &mut cursor, 1);
  assert!(cursor == 5);
  assert!(wide == [0xdead, 0xbeef, 0x1234, 0x5678]);
  OK
}

/// 布局/名单字符串常量面 (数值部分已编译期折叠)
#[test]
fn layout_constants() -> Void {
  assert_eq!(GROW_SUCCESS_RETURNS, "old_pages");
  assert_eq!(GROW_REPORT_ROUNDING, "bit_ceil(new_byte_length)");
  assert_eq!(
    BASES_AND_SIZES_LAYOUT,
    "偶数长 (base,size) 对; memory_index < len/2"
  );
  OK
}

/// v8: src/wasm/code-space-access.cc:11-12 CodeSpaceWriteScope 作用域回环
#[test]
fn code_space_scope_roundtrip() -> Void {
  {
    let _scope = CodeSpaceWriteScope::new();
    let _inner = CodeSpaceWriteScope::default();
  }
  // drop 后恢复 RX; 再次构造不应受影响
  let _again = CodeSpaceWriteScope::new();
  OK
}

/// v8: src/wasm/wasm-objects.cc:132-189 WasmModuleObject 名字字节切片与解析测试
#[test]
fn module_bytes_and_names() -> Void {
  assert_eq!(
    moduleobject_extractutf8stringfrommodulebytes(b"hello world", 0, 5),
    Some(&b"hello"[..])
  );
  assert_eq!(
    moduleobject_extractutf8stringfrommodulebytes(b"hello", 0, 10),
    None
  );
  assert_eq!(moduleobject_getmodulenameornull(false, b"mod", 0, 3), None);
  assert_eq!(
    moduleobject_getmodulenameornull(true, b"mod", 0, 3),
    Some(&b"mod"[..])
  );
  assert_eq!(moduleobject_getfunctionnameornull(None, b"foo"), None);
  assert_eq!(
    moduleobject_getfunctionnameornull(Some((0, 3)), b"foo"),
    Some(&b"foo"[..])
  );
  assert_eq!(
    moduleobject_getrawfunctionname(-1, Some((0, 3)), b"foo"),
    &b""[..]
  );
  assert_eq!(
    moduleobject_getrawfunctionname(1, Some((0, 3)), b"foo"),
    &b"foo"[..]
  );
  assert_eq!(moduleobject_getrawfunctionname(1, None, b"foo"), &b""[..]);
  OK
}

/// v8: src/wasm/wasm-objects.cc:899-1256 WasmMemoryObject 辅助决策与状态机测试
#[test]
fn memory_lifecycle_and_helpers() -> Void {
  assert!(memoryobject_useininstance(true));
  assert!(!memoryobject_useininstance(false));

  let plan_shared = memoryobject_setnewbuffer(true, true);
  assert!(plan_shared.pre_zero_length);
  assert!(plan_shared.freeze);
  assert!(plan_shared.register_shared);
  assert!(plan_shared.fixup_resizable);

  let plan_plain = memoryobject_setnewbuffer(false, false);
  assert!(!plan_plain.pre_zero_length);
  assert!(!plan_plain.freeze);

  let mems0: &[u64] = &[10, 20];
  let mems1: &[u64] = &[20, 30];
  let hits = memoryobject_updateinstances(&[Some(mems0), None, Some(mems1)], 20);
  assert_eq!(hits, vec![(0, 1), (2, 0)]);

  assert_eq!(memoryobject_fixupresizablearraybuffer(10), 10 * 65536);
  assert_eq!(memoryobject_fixupresizablearraybuffer(-1), 0);

  assert_eq!(
    memoryobject_refreshbuffer(true, Some(true)),
    (true, Some(true))
  );
  assert_eq!(memoryobject_refreshbuffer(false, Some(true)), (false, None));

  assert!(memoryobject_getarraybuffer(true));
  assert!(!memoryobject_getarraybuffer(false));

  assert_eq!(
    memoryobject_changearraybufferresizability(true, true, true, false, true),
    MemoryChangeResizabilityStep::KeepOld
  );
  assert_eq!(
    memoryobject_changearraybufferresizability(false, false, false, true, true),
    MemoryChangeResizabilityStep::RefreshShared(true)
  );
  assert_eq!(
    memoryobject_changearraybufferresizability(true, false, false, false, true),
    MemoryChangeResizabilityStep::ToggleThenRefresh { detach_old: true }
  );
  assert_eq!(
    memoryobject_changearraybufferresizability(false, false, true, false, true),
    MemoryChangeResizabilityStep::RefreshPlain
  );
  OK
}

/// v8: src/wasm/wasm-objects.cc:1053-1207 WasmMemoryObject::Grow 全状态机分支测试
#[test]
fn memory_grow_statemachine() -> Void {
  // API 中断期不允许非共享内存增长
  let res_api = memoryobject_grow(MemoryGrowParams {
    old_pages: 10,
    max_pages: 100,
    pages: 5,
    is_shared: false,
    has_guard_regions: false,
    is_resizable_by_js: false,
    is_executing_api_interrupt: true,
    stress_wasm_memory_moving: false,
    has_old_buffer: false,
    grown_byte_length: 0,
    grow_in_place_result: None,
    copy_result: None,
    correctness_fuzzer_suppressions: false,
  });
  assert_eq!(res_api.ret, -1);

  // 共享内存原地成功
  let res_shared = memoryobject_grow(MemoryGrowParams {
    old_pages: 10,
    max_pages: 100,
    pages: 5,
    is_shared: true,
    has_guard_regions: false,
    is_resizable_by_js: false,
    is_executing_api_interrupt: false,
    stress_wasm_memory_moving: false,
    has_old_buffer: false,
    grown_byte_length: 15 * 65536,
    grow_in_place_result: Some(10),
    copy_result: None,
    correctness_fuzzer_suppressions: false,
  });
  assert_eq!(res_shared.ret, 10);
  assert!(res_shared.broadcast);

  // 拷贝增长成功
  let res_copy = memoryobject_grow(MemoryGrowParams {
    old_pages: 10,
    max_pages: 100,
    pages: 5,
    is_shared: false,
    has_guard_regions: false,
    is_resizable_by_js: false,
    is_executing_api_interrupt: false,
    stress_wasm_memory_moving: true,
    has_old_buffer: true,
    grown_byte_length: 0,
    grow_in_place_result: None,
    copy_result: Some((15 * 65536, 80)),
    correctness_fuzzer_suppressions: false,
  });
  assert_eq!(res_copy.ret, 10);
  assert!(res_copy.update_instances);
  assert!(res_copy.detach_old_buffer);
  assert_eq!(res_copy.new_backing_store, Some((15 * 65536, 19)));
  OK
}

/// v8: src/wasm/wasm-objects.cc:1435-1933 WasmTrustedInstanceData 纯逻辑面测试
#[test]
fn trusted_instance_data_operations() -> Void {
  let mut bases_and_sizes = [0usize; 4];
  assert_eq!(
    trustedinstancedata_setrawmemory(
      &mut bases_and_sizes,
      SetRawMemoryParams {
        memory_index: 0,
        memories_count: 2,
        is_shared: false,
        is_memory64: false,
        engine_max_mem_bytes: 65536,
        shared_backing_stores_len: 0,
        mem_start: 0x1000,
        size_or_address: 0x2000,
        size_address_eq: None,
      },
    ),
    Some(true)
  );
  assert_eq!(bases_and_sizes[0], 0x1000);
  assert_eq!(bases_and_sizes[1], 0x2000);

  assert_eq!(
    trustedinstancedata_setrawmemory(
      &mut bases_and_sizes,
      SetRawMemoryParams {
        memory_index: 1,
        memories_count: 2,
        is_shared: false,
        is_memory64: false,
        engine_max_mem_bytes: 65536,
        shared_backing_stores_len: 0,
        mem_start: 0x3000,
        size_or_address: 0x4000,
        size_address_eq: None,
      },
    ),
    Some(false)
  );
  assert_eq!(bases_and_sizes[2], 0x3000);
  assert_eq!(bases_and_sizes[3], 0x4000);

  // 越界保护
  assert_eq!(
    trustedinstancedata_setrawmemory(
      &mut bases_and_sizes,
      SetRawMemoryParams {
        memory_index: 2,
        memories_count: 2,
        is_shared: false,
        is_memory64: false,
        engine_max_mem_bytes: 65536,
        shared_backing_stores_len: 0,
        mem_start: 0,
        size_or_address: 0,
        size_address_eq: None,
      },
    ),
    None
  );

  let segs = trustedinstancedata_initdatasegmentarrays(&[(true, 10, 20), (false, 30, 40)]);
  assert_eq!(segs, vec![(10, 0), (30, 40)]);

  assert_eq!(
    trustedinstancedata_getcalltarget(0, 10, 2, false),
    Some(CallTargetSource::Imported)
  );
  assert_eq!(
    trustedinstancedata_getcalltarget(5, 10, 2, true),
    Some(CallTargetSource::Invalid)
  );
  assert_eq!(
    trustedinstancedata_getcalltarget(5, 10, 2, false),
    Some(CallTargetSource::NativeHandle)
  );
  assert_eq!(trustedinstancedata_getcalltarget(15, 10, 2, false), None);

  assert_eq!(
    trustedinstancedata_copytableentries(0, 0, 5, 10, 10, true),
    Some(TableCopyOp::Noop)
  );
  assert_eq!(
    trustedinstancedata_copytableentries(5, 0, 3, 10, 10, true),
    Some(TableCopyOp::Backward)
  );
  assert_eq!(
    trustedinstancedata_copytableentries(0, 5, 3, 10, 10, true),
    Some(TableCopyOp::Forward)
  );
  assert_eq!(
    trustedinstancedata_copytableentries(0, 0, 15, 10, 10, true),
    None
  );

  assert_eq!(trustedinstancedata_inittableentries(0, 0, 5, 10, 10), None);
  assert_eq!(
    trustedinstancedata_inittableentries(8, 0, 5, 10, 10),
    Some(InitTableEntriesError::TableOutOfBounds)
  );
  assert_eq!(
    trustedinstancedata_inittableentries(0, 8, 5, 10, 10),
    Some(InitTableEntriesError::ElementSegmentOutOfBounds)
  );

  assert_eq!(
    trustedinstancedata_try_get_func_ref::<i32>(&FuncRefSlot::Smi),
    None
  );
  assert_eq!(
    trustedinstancedata_try_get_func_ref(&FuncRefSlot::Ref(42)),
    Some(&42)
  );

  assert_eq!(
    trustedinstancedata_getglobalstorage(true, true, false, 3, 0),
    GlobalStorageSel::ImportedMutable(3)
  );
  assert_eq!(
    trustedinstancedata_getglobalstorage(false, false, true, 0, 7),
    GlobalStorageSel::TaggedGlobals(7)
  );
  assert_eq!(
    trustedinstancedata_getglobalstorage(false, false, false, 0, 8),
    GlobalStorageSel::UntaggedGlobals(8)
  );

  assert_eq!(
    trustedinstancedata_getglobalvalue(true, ValueKind::I32),
    GlobalValueRead::Ref
  );
  assert_eq!(
    trustedinstancedata_getglobalvalue(false, ValueKind::I32),
    GlobalValueRead::Num(4)
  );
  assert_eq!(
    trustedinstancedata_getglobalvalue(false, ValueKind::I64),
    GlobalValueRead::Num(8)
  );
  assert_eq!(
    trustedinstancedata_getglobalvalue(false, ValueKind::Void),
    GlobalValueRead::Reject
  );
  OK
}

/// v8: src/wasm/wasm-objects.cc:225-728 WasmTableObject 纯逻辑面测试
#[test]
fn table_operations() -> Void {
  assert_eq!(tableobject_new(false, true, 10, false, false), None);
  let plan = tableobject_new(true, true, 10, false, true).unwrap();
  assert!(plan.is_function_table);
  assert_eq!(plan.dispatch_initial_length, 10);
  assert!(plan.update_dispatch_initial);

  assert!(matches!(
    tableobject_grow(10, 0, Some(20), 100, 10, true),
    TableGrowOutcome::Noop { old_size: 10 }
  ));
  assert!(matches!(
    tableobject_grow(10, 15, Some(20), 100, 10, true),
    TableGrowOutcome::Failed
  ));
  assert!(matches!(
    tableobject_grow(10, 5, Some(20), 100, 10, true),
    TableGrowOutcome::Grown {
      old_size: 10,
      new_size: 15,
      ..
    }
  ));

  assert!(tableobject_fill(0, 10, 10));
  assert!(!tableobject_fill(5, 6, 10));
  // 溢出安全防线: start + count 越 u32 上限不 panic 且安全判定 false
  assert!(!tableobject_fill(u32::MAX - 5, 10, u32::MAX));

  assert!(tableobject_setfunctiontableplaceholder(false, 1, 2).is_none());
  assert!(tableobject_setfunctiontableplaceholder(true, 1, 2).is_some());

  assert_eq!(
    tableobject_updatedispatchtable_batch(true),
    UpdateMultipleKind::Exported
  );
  assert_eq!(
    tableobject_updatedispatchtable_batch(false),
    UpdateMultipleKind::Capi
  );

  assert_eq!(tableobject_updatedispatchtable(true, false, false), None);
  assert_eq!(
    tableobject_updatedispatchtable(true, true, true),
    Some((ImplicitArgSource::ImportsTable, SetOp::Wrapper))
  );
  assert_eq!(
    tableobject_updatedispatchtable(false, true, false),
    Some((ImplicitArgSource::TargetInstanceData, SetOp::NonWrapper))
  );

  let placeholder = tableobject_setfunctiontableplaceholder(true, 1, 2);
  assert!(matches!(placeholder, Some(EntryWrite::Placeholder(_))));

  let vt = ValueType::generic(GenericKind::Func, true, false);
  assert_eq!(tableobject_set(&vt, true), TableSetOp::FunctionEntry);
  assert_eq!(
    tableobject_get(true, false, &vt, false),
    TableGetOp::ReturnEntry
  );
  assert_eq!(
    tableobject_get(false, false, &vt, false),
    TableGetOp::LazyInit
  );

  let canon_vt = CanonicalValueType::generic(GenericKind::Func, true, false);
  assert!(function_sig_matches_table(
    CanonicalTypeIndex::new(1),
    canon_vt,
    false
  ));
  OK
}

// —— 轮7 W7WO 调度表/异常/外部函数纯模型面 ——

/// v8: src/wasm/wasm-objects.cc:2279-2482 dispatch entry_set/entry_clear 及公开包装
#[test]
fn dispatch_entry_set_clear() -> Void {
  let mut t = DispatchTableModel {
    length: 2,
    capacity: 2,
    has_sig: true,
    jitless: false,
    drumbrake: true,
    entries: vec![DispatchEntry::EMPTY; 2],
  };
  // 非 wrapper 直写: target=call_target, 隐式参=句柄, sig=sig_id (.cc:2299-2317)
  assert_eq!(
    dispatchtable_setfornonwrapper(&mut t, 0, 0xAA, 0x1000, 7, 99, DispatchNewOrExisting::New),
    Some(())
  );
  assert_eq!(t.entries[0].target, 0x1000);
  assert_eq!(t.entries[0].implicit_arg, EntryWrite::ImplicitArg(0xAA));
  assert_eq!(t.entries[0].sig, 7);
  // 越界 SBXCHECK_BOUNDS fail-closed (.cc:2288)
  assert_eq!(
    dispatchtable_setfornonwrapper(&mut t, 2, 0xAA, 0x1000, 7, 99, DispatchNewOrExisting::New),
    None
  );
  // wrapper 写入: 既有 entry.wrapper 先 Remove 再 Add (.cc:2375-2381)
  t.entries[0].wrapper = Some(5);
  assert_eq!(
    dispatchtable_setforwrapper(
      &mut t,
      0,
      SetWrapperParams {
        implicit_arg: 0xBB,
        wrapper_handle: 9,
        code_pointer: 0x2000,
        sig_id: 8,
        function_index: 99,
        new_or_existing: DispatchNewOrExisting::Existing,
      },
    ),
    Some(())
  );
  assert_eq!(t.entries[0].target, 0x2000);
  assert_eq!(t.entries[0].wrapper, Some(9));
  assert_eq!(t.entries[0].sig, 8);
  // jitless+DRUMBRAKE: 忽略 call_target, function_index 落 target 槽 (.cc:2302-2306)
  let mut j = DispatchTableModel {
    jitless: true,
    ..t.clone()
  };
  assert_eq!(
    dispatchtable_setfornonwrapper(&mut j, 1, 0xCC, 0xDEAD, 3, 42, DispatchNewOrExisting::New),
    Some(())
  );
  assert_eq!(j.entries[1].target, 42);
  // 清空: target=失效哨兵, 隐式参空, sig=-1, wrapper 移除 (.cc:2427-2447)
  assert_eq!(
    dispatchtable_clear(&mut j, 1, DispatchNewOrExisting::Existing),
    Some(())
  );
  assert_eq!(j.entries[1].target, DispatchEntry::INVALID_CODE_POINTER);
  assert_eq!(j.entries[1].implicit_arg, EntryWrite::Empty);
  assert_eq!(j.entries[1].sig, -1);
  assert_eq!(j.entries[1].wrapper, None);
  // ForImports 无 sig 槽: sig 不被写入 (.cc:2314 requires{kSigBias})
  let mut f = DispatchTableModel {
    length: 1,
    capacity: 1,
    has_sig: false,
    jitless: false,
    drumbrake: false,
    entries: vec![DispatchEntry::EMPTY],
  };
  assert_eq!(
    dispatchtableforimports_setfornonwrapper(
      &mut f,
      0,
      0xDD,
      0x3000,
      5,
      0,
      DispatchNewOrExisting::New
    ),
    Some(())
  );
  assert_eq!(f.entries[0].target, 0x3000);
  assert_eq!(f.entries[0].sig, -1); // EMPTY 初值, 未覆写
  assert_eq!(
    dispatchtableforimports_setforwrapper(
      &mut f,
      0,
      SetWrapperParams {
        implicit_arg: 0xEE,
        wrapper_handle: 12,
        code_pointer: 0x4000,
        sig_id: 6,
        function_index: 0,
        new_or_existing: DispatchNewOrExisting::Existing,
      },
    ),
    Some(())
  );
  assert_eq!(f.entries[0].target, 0x4000);
  assert_eq!(f.entries[0].wrapper, Some(12));
  assert_eq!(f.entries[0].sig, -1); // has_sig=false, sig 仍为 -1
  OK
}

/// v8: src/wasm/wasm-objects.cc:2248-2273 WasmDispatchTableData wrappers_ 映射面
#[test]
fn dispatch_wrapper_map() -> Void {
  let mut m: WasmWrapperMap = Vec::new();
  assert!(dispatchtabledata_add(&mut m, 3, 0x11));
  assert!(!dispatchtabledata_add(&mut m, 3, 0x22)); // emplace 键存在 → 不插入
  assert_eq!(m.len(), 1);
  assert_eq!(
    dispatchtabledata_wrappercodepointerfordebugging(&m, 3),
    Some(0x11)
  );
  assert_eq!(
    dispatchtabledata_wrappercodepointerfordebugging(&m, 9),
    None
  ); // CHECK 缺失 fail-closed
  assert_eq!(dispatchtabledata_remove(&mut m, 3), 1);
  assert_eq!(dispatchtabledata_remove(&mut m, 3), 0); // erase 缺失返 0
  OK
}

/// v8: src/wasm/wasm-objects.cc:2489-2564 AddUse/MaybeGrowUsesList 压实与扩容
#[test]
fn dispatch_uses_compact_grow() -> Void {
  // 空表: 初始 1 条目区, used=0 (.cc:2507-2513)
  let mut uses: Vec<Option<UseEntry>> = Vec::new();
  let mut used = 0usize;
  dispatchtable_adduse(&mut uses, &mut used, 100, 0);
  assert_eq!(uses.len(), 1);
  assert_eq!(used, 1);
  // 满→压实不足→扩容 (+1 条目) (.cc:2540-2546)
  dispatchtable_adduse(&mut uses, &mut used, 200, 1);
  assert_eq!(uses.len(), 2);
  assert_eq!(used, 2);
  dispatchtable_adduse(&mut uses, &mut used, 300, 2);
  assert_eq!(uses.len(), 3);
  assert_eq!(used, 3);
  // 制造一个 cleared 中位并写满, 触发原地压实 (cap_slots=7→min_free=2, cleared>=2 才压实)
  let mut u2: Vec<Option<UseEntry>> = vec![Some((1, 0)), None, Some((3, 2)), Some((4, 3))];
  let mut used2 = 4usize;
  dispatchtable_maybegrowuseslist(&mut u2, &mut used2);
  // cleared=1 < min_free(1+ (7>>2)=2) → 扩容: new_entries=max(5,4+2)=6, 前缀活条目 write=3
  assert_eq!(u2.len(), 6);
  assert_eq!(used2, 3);
  assert_eq!(u2[0], Some((1, 0)));
  assert_eq!(u2[1], Some((3, 2)));
  assert_eq!(u2[2], Some((4, 3)));
  assert_eq!(u2[3], None);
  // 多 cleared 压实成功: cap_slots=9 (len4) → min_free=1+(9>>2)=3, cleared>=3 → 原地收拢
  let mut u3: Vec<Option<UseEntry>> = vec![None, Some((9, 8)), None, None];
  let mut used3 = 4usize;
  dispatchtable_maybegrowuseslist(&mut u3, &mut used3);
  assert_eq!(u3.len(), 4); // 未扩容
  assert_eq!(used3, 1); // write 游标收拢至 1 活条目
  assert_eq!(u3[0], Some((9, 8)));
  OK
}

/// v8: src/wasm/wasm-objects.cc:2567-2670 WasmDispatchTable::Grow 状态机
#[test]
fn dispatch_table_grow_statemachine() -> Void {
  // 非增长态 fail-closed (SBXCHECK_LT)
  let mut t = DispatchTableModel {
    length: 4,
    capacity: 8,
    has_sig: true,
    jitless: false,
    drumbrake: false,
    entries: (0..8)
      .map(|i| DispatchEntry {
        target: i,
        ..DispatchEntry::EMPTY
      })
      .collect(),
  };
  assert_eq!(dispatchtable_grow(&mut t, 4, 100, 100), None);
  // 越引擎表上限 fail-closed (SBXCHECK_LE)
  assert_eq!(dispatchtable_grow(&mut t, 200, 100, 100), None);
  // new_length < old_capacity → 原地改 length 复用旧表 (.cc:2588-2592)
  let g = dispatchtable_grow(&mut t, 6, 100, 100).unwrap();
  assert!(g.reused);
  assert_eq!(g.new_length, 6);
  assert_eq!(g.new_capacity, 8);
  assert_eq!(t.length, 6);
  // 真正扩容: 空隐式参条目落清空哨兵, 活条目原样搬运 (.cc:2614-2638)
  let mut t2 = DispatchTableModel {
    length: 1,
    capacity: 1,
    has_sig: true,
    jitless: false,
    drumbrake: false,
    entries: vec![DispatchEntry::EMPTY],
  };
  t2.entries[0].implicit_arg = EntryWrite::ImplicitArg(0x77);
  t2.entries[0].target = 0x999;
  t2.entries[0].sig = 4;
  let g2 = dispatchtable_grow(&mut t2, 10, 1000, 1000).unwrap();
  assert!(!g2.reused);
  // 校核: old_capacity=1, min_grow=9, max_grow=999, exp=max(1,8)=8, 域内 clamp→[9,999]→9 → new_capacity=1+9=10
  assert_eq!(g2.new_capacity, 10);
  assert_eq!(t2.length, 10);
  assert_eq!(t2.entries[0].target, 0x999);
  assert_eq!(t2.entries[0].sig, 4);
  assert_eq!(t2.entries[0].implicit_arg, EntryWrite::ImplicitArg(0x77));
  // 区外新条目 = EMPTY 初值
  assert_eq!(t2.entries[9], DispatchEntry::EMPTY);
  OK
}

/// v8: src/wasm/wasm-objects.cc:2804-3127 异常 for_tag 委派 + 外部函数谓词
#[test]
fn dispatch_exception_and_external() -> Void {
  let params = [ValueKind::I32, ValueKind::F64, ValueKind::Ref];
  assert_eq!(
    exceptionpackage_getencodedsize_for_tag(&params),
    exceptionpackage_getencodedsize(&params)
  );
  assert_eq!(exceptionpackage_getencodedsize_for_tag(&params), 2 + 4 + 1);
  assert!(externalfunction_iswasmexternalfunction(true, false));
  assert!(externalfunction_iswasmexternalfunction(false, true));
  assert!(!externalfunction_iswasmexternalfunction(false, false));
  OK
}
