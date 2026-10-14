// 由 tool/src/wasm/wasm-objects.cc.js 从 v8/src/wasm/wasm-objects.{cc,-inl.h} 生成; 修订就地改, 复跑覆盖
// Wasm 对象行为规则: 内存分配启发式/Grow 状态机/uses 列表布局 + 方法签名骨架
// R5W 修订: 填体不依赖 Isolate/堆面的纯逻辑切片 (内存/表/调度表/异常编码);
// 依赖 Isolate/堆写屏障的方法保留 todo! 骨架, 逐条推迟登记见 readme/zh.md
// 轮7 W7WO 修订: 骨架填体 55/76 —— 以抽取入参/纯模型 (调度表条目 Vec 模型、
// uses 槽位 Vec 模型、页数/偏移算术、决策规划器) 1:1 复刻原版控制流;
// 仅工厂分配/堆写/代码指针面保留 todo!, 收口 readme/zh.md 推迟台账 B 节

//! Wasm 堆对象行为规则面 (对象布局常量在 wasm_objects_consts.rs)。

// 骨架区 todo! 为待填体占位, 不被任何已实现面调用; 逐条推迟登记见 readme/zh.md

use std::mem::align_of;

use v8_wasm_core::{
  value_type::{
    CanonicalTypeIndex, CanonicalValueType, GenericKind, ValueKind, ValueType, ValueTypeBits,
    value_kind_size,
  },
  wasm_constants::{K_ANONYMOUS_FUNC_INDEX, WASM_PAGE_SIZE},
};

/// dispatch table uses 列表: 首槽保留, 其后 (instance, table_index) 对
/// v8: src/wasm/wasm-objects.cc kReservedSlotOffset
pub const USES_RESERVED_SLOT_OFFSET: u32 = 1;
/// uses 列表初始容量: 1 保留槽 + 1 对
/// v8: src/wasm/wasm-objects.cc WasmDispatchTable::MaybeGrowUsesList
pub const USES_INITIAL_LENGTH: u32 = 3;
/// AddUse 每条记录步进 2 (instance + table_index)
/// v8: src/wasm/wasm-objects.cc WasmDispatchTable::AddUse
pub const USES_ENTRY_STEP: u32 = 2;

/// 内存对象无最大声明哨兵 (原版 WasmMemoryObject::kNoMaximum)
/// v8: src/wasm/wasm-objects.h:477 WasmMemoryObject::kNoMaximum
/// 注: 与 wasm_objects_consts::MEMORY_NO_MAXIMUM 同源, consts 模块解锁接线后合并为单一来源
pub const K_NO_MAXIMUM: i32 = -1;

/// 32 位平台内存预留启发式: 1GB 页数上限 (1024^3 / kWasmPageSize)
/// v8: src/wasm/wasm-objects.cc WasmMemoryObject::New
pub const K_GB_PAGES_FORMULA: &str = "1024 * 1024 * 1024 / kWasmPageSize";
/// 1GB 对应的 wasm 页数 (启发式实际数值)
/// v8: src/wasm/wasm-objects.cc:858 kGBPages
pub const K_GB_PAGES: usize = 1024 * 1024 * 1024 / WASM_PAGE_SIZE;
/// 启发式分支序: 初始>1GB 用初始; 有最大取 min(maximum, allocMax); 共享无最大用 allocMax; 非共享无最大用初始
/// v8: src/wasm/wasm-objects.cc WasmMemoryObject::New (V8_TARGET_ARCH_32_BIT)
#[rustfmt::skip]
pub static MEMORY_RESERVE_HEURISTIC: &[&str] = &[
    "initial>1GB -> initial",
    "hasMaximum -> min(maximum, allocMax)",
    "shared&&!maximum -> allocMax",
    "!shared&&!maximum -> initial",
];

/// 必须原地增长的四条件
/// v8: src/wasm/wasm-objects.cc WasmMemoryObject::Grow
#[rustfmt::skip]
pub static MUST_GROW_IN_PLACE: &[&str] = &[
    "backing_store.is_shared()",
    "backing_store.has_guard_regions()",
    "backing_store.is_resizable_by_js()",
    "pages == 0",
];

/// 拷贝增长最小步进: old_pages + 8 + old_pages/8 (约 0.5MB + 12.5%)
/// v8: src/wasm/wasm-objects.cc WasmMemoryObject::Grow
pub fn min_growth_pages(old_pages: usize) -> usize {
  old_pages + 8 + (old_pages >> 3)
}

/// 新容量: min(max_pages, max(new_pages, min_growth)) — 先下限后上限
/// v8: src/wasm/wasm-objects.cc WasmMemoryObject::Grow
pub fn new_capacity_pages(max_pages: usize, new_pages: usize, old_pages: usize) -> usize {
  max_pages.min(new_pages.max(min_growth_pages(old_pages)))
}

/// Grow 失败 (-1) 路径: 内存 4 条 + 表超限 1 条
/// v8: src/wasm/wasm-objects.cc WasmMemoryObject::Grow / WasmTableObject::Grow
pub const GROW_FAIL_PATHS: usize = 4;
pub const TABLE_GROW_FAIL_PATHS: usize = 1;

/// 成功返回旧页数 (共享内存返回同步后的原值)
/// v8: src/wasm/wasm-objects.cc WasmMemoryObject::Grow
pub const GROW_SUCCESS_RETURNS: &str = "old_pages";
/// 托管对象报告大小取 bit_ceil, 避免循环增长触发过多 GC
/// v8: src/wasm/wasm-objects.cc WasmMemoryObject::Grow
pub const GROW_REPORT_ROUNDING: &str = "bit_ceil(new_byte_length)";

/// 异常值数组元素宽度 (uint16)
/// v8: src/wasm/wasm-objects.cc kBytesPerExceptionValuesArrayElement
pub const K_BYTES_PER_EXCEPTION_VALUES_ARRAY_ELEMENT: u32 = 2;

/// 全局引用值最小偏移: FixedArray::OffsetOfElementAt(0) - kHeapObjectTag
/// v8: src/wasm/wasm-objects.cc WasmGlobalObject::New
pub const K_MIN_REF_OFFSET_FROM_TAGGED_BUFFER: &str =
  "FixedArray::OffsetOfElementAt(0) - kHeapObjectTag";
/// 全局数值最小偏移: ByteArray::OffsetOfElementAt(0) - kHeapObjectTag
/// v8: src/wasm/wasm-objects.cc WasmGlobalObject::New
pub const K_MIN_NUM_OFFSET_FROM_TAGGED_BUFFER: &str =
  "ByteArray::OffsetOfElementAt(0) - kHeapObjectTag";

/// -inl.h 侧防线: bases_and_sizes 长度必须为偶数, memory_index < len/2
/// v8: src/wasm/wasm-objects-inl.h WasmTrustedInstanceData::memory_base/memory_size
pub const BASES_AND_SIZES_LAYOUT: &str = "偶数长 (base,size) 对; memory_index < len/2";
/// WasmCodePointer 与 u32 同宽
/// v8: src/wasm/wasm-objects-inl.h static_assert(sizeof(WasmCodePointer) == sizeof(uint32_t))
pub const CODE_POINTER_WIDTH: usize = 4;
/// -inl.h 访问器宏展开数 21 的口径注: 系 wasm-objects.h 侧 DECL_ACCESSORS(13)+DECL_TRUSTED_POINTER(5)+DECL_OPTIONAL(3) 拼合复现;
/// (13 为 grep 原文行口径含 h:74 #define 体自引用, 严格调用点口径 12 对应总数 20);
/// -inl.h 本体四宏 (DEF_GETTER/ACCESSORS/TRUSTED_POINTER_ACCESSORS/OPTIONAL_ACCESSORS) 顶层直调 0 行,
/// 实际直调为 WTI_*/PRIMITIVE 本地宏 29 行 + EXTERNAL_POINTER_ACCESSORS 标准宏 1 行 (-inl.h:1487) = 30
/// v8: src/wasm/wasm-objects.h:721+ / wasm-objects-inl.h:1487
pub const INL_ACCESSOR_MACROS: usize = 21;

// —— 填体: 纯逻辑面 (不依赖 Isolate/堆, 逐条带原版锚点) ——

/// 新建内存对象预留页数启发式: initial 超引擎上限返回 None (原版返回空句柄);
/// 32 位平台按 MEMORY_RESERVE_HEURISTIC 四分支, 64 位取 has_maximum ? min(engine, maximum) : engine。
/// engine_maximum 由调用方按地址宽度取 max_mem32/64_pages() (依赖 flags) 后代入;
/// 其后的 BackingStore 分配与对象接续见推迟台账。
/// v8: src/wasm/wasm-objects.cc:844-885 WasmMemoryObject::New
pub fn memoryobject_new_reserved_pages(
  initial: i32,
  maximum: i32,
  shared: bool,
  engine_maximum: usize,
) -> Option<usize> {
  // 原版调用面 DCHECK (.cc:795 系上游重载, 本模函数体 .cc:844-885 内无): maximum == kNoMaximum || maximum >= 0;
  // 域外负值 (<= -2) 此处按 0 钳位走有最大分支 (预留 0 页), initial 负值同按 0 钳位 (复审 WOR4 F9)
  let initial = initial.max(0) as usize;
  if initial > engine_maximum {
    return None;
  }
  let has_maximum = maximum != K_NO_MAXIMUM;
  if cfg!(target_pointer_width = "32") {
    let allocation_maximum = K_GB_PAGES.min(engine_maximum);
    if initial > K_GB_PAGES {
      Some(initial)
    } else if has_maximum {
      Some((maximum.max(0) as usize).min(allocation_maximum))
    } else if shared {
      Some(allocation_maximum)
    } else {
      Some(initial)
    }
  } else if has_maximum {
    Some(engine_maximum.min(maximum.max(0) as usize))
  } else {
    Some(engine_maximum)
  }
}

/// 内存可增长页数上限: 引擎 32/64 位上限与声明最大 (若有) 取小
/// engine_max_pages 由调用方按 is_memory64 选 max_mem64/32_pages() (依赖 flags) 代入
/// 谓词逐字同原版 has_maximum_pages() = maximum_pages() >= 0 (-inl.h:1231); 域外负值原版判无最大返 engine_max, 复审 WOR4 F10
/// v8: src/wasm/wasm-objects.cc:1078-1083 WasmMemoryObject::Grow
pub fn memoryobject_max_size(maximum_pages: i32, engine_max_pages: usize) -> usize {
  if maximum_pages >= 0 {
    engine_max_pages.min(maximum_pages as usize)
  } else {
    engine_max_pages
  }
}

/// 页数增量是否超上限: pages > max_pages - old_pages
/// 原版 DCHECK_GE(max_pages, old_pages); saturating 对应堆腐蚀防御, 反向偏差判超
/// v8: src/wasm/wasm-objects.cc:1084-1085 WasmMemoryObject::Grow
pub fn memoryobject_grow_exceeds(old_pages: usize, pages: u32, max_pages: usize) -> bool {
  pages as usize > max_pages.saturating_sub(old_pages)
}

/// 必须原地增长判定: 共享 / 带 guard 区 / JS 可变长 / 零页增长 任一命中
/// v8: src/wasm/wasm-objects.cc:1087-1089 WasmMemoryObject::Grow
pub fn memoryobject_must_grow_in_place(
  is_shared: bool,
  has_guard_regions: bool,
  is_resizable_by_js: bool,
  pages: u32,
) -> bool {
  is_shared || has_guard_regions || is_resizable_by_js || pages == 0
}

/// 表增长尺寸上限: min(引擎表上限, 声明最大; 无声明取引擎上限)
/// 原版 static_assert(kV8MaxWasmTableSize <= kMaxUInt32), 截断安全;
/// engine_max_table_size 由调用方以 max_table_size() (依赖 flags) 代入
/// v8: src/wasm/wasm-objects.cc:301-304 WasmTableObject::Grow
pub fn tableobject_grow_max_size(maximum_length: Option<u64>, engine_max_table_size: u64) -> u32 {
  engine_max_table_size.min(maximum_length.unwrap_or(engine_max_table_size)) as u32
}

/// 表增长增量是否超上限: count > max_size - old_size
/// 原版 DCHECK_LE(old_size, max_size) (.cc:305) 后 unchecked 减法; saturating 为反例态防御偏差 (同 memory 侧 :150 口径)
/// v8: src/wasm/wasm-objects.cc:305-306 WasmTableObject::Grow
pub fn tableobject_grow_exceeds(old_size: u32, count: u32, max_size: u32) -> bool {
  count > max_size.saturating_sub(old_size)
}

/// entries 数组扩容槽数: 未超旧容量返回 None; 否则 max(缺口, 旧容量) 实现指数增长,
/// 并封顶 max_size - old_capacity (绝不越声明最大; DCHECK 前提同上, saturating 防御)
/// v8: src/wasm/wasm-objects.cc:312-318 WasmTableObject::Grow
pub fn tableobject_entries_grow_delta(
  new_size: u32,
  old_capacity: u32,
  max_size: u32,
) -> Option<u32> {
  if new_size <= old_capacity {
    return None;
  }
  let gap = new_size - old_capacity;
  Some(
    gap
      .max(old_capacity)
      .min(max_size.saturating_sub(old_capacity)),
  )
}

/// 调度表需新增容量: new_length 未超旧容量返回 None (原版原地写 length 复用旧表);
/// 否则按旧容量指数增长 (至少 8), 夹在 [min_grow, max_grow] 之间。
/// 域内 (min_grow <= max_grow) 三式等价: std::clamp (.cc:2603) ↔ max(min(v,hi),lo) ↔ min(max(v,lo),hi);
/// 腐蚀态 lo > hi 时仅 v < lo 子域与 std::clamp 形同 (双方返 lo); v >= lo 子域 std::clamp 返 hi (fail-closed)
/// 而本式恒返 lo (fail-open), 不形同——原版防护系 CHECK_LE (.cc:2600, release 亦 abort) 使腐蚀态不可达,
/// Rust 以 debug_assert 承接 CHECK, 可达 release 行为偏差与姊妹函数 tableobject_entries_grow_delta (fail-closed) 口径相反, 登记偏差
/// (复审 WOR5 F12 / WOR6 F13: 原注「clamp 次序防御化」系越claim; 「腐蚀态求值形同」越claim 已限定到子域, CHECK_LE 锚订正 2600)
/// v8: src/wasm/wasm-objects.cc:2588-2604 WasmDispatchTable::Grow
pub fn dispatchtable_grow_amount(old_capacity: u32, new_length: u32, limit: u32) -> Option<u32> {
  if new_length < old_capacity {
    return None;
  }
  let max_grow = limit.saturating_sub(old_capacity);
  let min_grow = new_length - old_capacity;
  debug_assert!(
    min_grow <= max_grow,
    "原版 CHECK_LE(min_grow, max_grow) (.cc:2600)"
  );
  Some(old_capacity.max(8).min(max_grow).max(min_grow))
}

/// uses 列表压实判定阈值: 至少 1 + capacity/4 个空条目才算压实成功
/// v8: src/wasm/wasm-objects.cc:2533-2536 WasmDispatchTable::MaybeGrowUsesList
pub fn dispatchtable_uses_min_free_entries(capacity: u32) -> u32 {
  1 + (capacity >> 2)
}

/// uses 列表压实不足时的新容量: 条目数 = capacity/2, 增 50% 且至少 1 条目,
/// 回乘 2 槽/条目再加保留槽
/// v8: src/wasm/wasm-objects.cc:2540-2546 WasmDispatchTable::MaybeGrowUsesList
pub fn dispatchtable_uses_new_capacity(capacity: u32) -> u32 {
  let old_entries = capacity >> 1;
  let new_entries = (old_entries + 1).max(old_entries + (old_entries >> 1));
  new_entries * 2 + USES_RESERVED_SLOT_OFFSET
}

/// 异常包编码元素数: i32/f32 记 2, i64/f64 记 4, s128 记 8, 引用记 1 (逐参数求和);
/// 原版对 kVoid/kTop/kBottom/kI8/kI16/kF16 UNREACHABLE (不可能入异常签名), 禁 panic 纪律下计 0
/// v8: src/wasm/wasm-objects.cc:2813-2844 WasmExceptionPackage::GetEncodedSize
pub fn exceptionpackage_getencodedsize(params: &[ValueKind]) -> u32 {
  params
    .iter()
    .map(|kind| match kind {
      ValueKind::I32 | ValueKind::F32 => 2,
      ValueKind::I64 | ValueKind::F64 => 4,
      ValueKind::S128 => 8,
      ValueKind::Ref | ValueKind::RefNull => 1,
      _ => 0,
    })
    .sum()
}

/// i32 异常值编码为两个 16 位槽并推进游标 2
/// 容量不足原版靠调用方 DCHECK; 越界防御为不写入但仍推进游标 (与预分配语义一致)
/// v8: src/wasm/wasm-objects.cc:2738-2742 EncodeI32ExceptionValue
pub fn encode_i32_exception_value(values: &mut [u32], cursor: &mut usize, value: u32) {
  if let Some(slots) = values.get_mut(*cursor..*cursor + 2) {
    slots.copy_from_slice(&[value >> 16, value & 0xffff]);
  }
  *cursor += 2;
}

/// i64 异常值编码为两个 i32 段 (先高 32 位后低 32 位), 共 4 槽
/// v8: src/wasm/wasm-objects.cc:2744-2750 EncodeI64ExceptionValue
pub fn encode_i64_exception_value(values: &mut [u32], cursor: &mut usize, value: u64) {
  encode_i32_exception_value(values, cursor, (value >> 32) as u32);
  encode_i32_exception_value(values, cursor, value as u32);
}

/// i32 异常值解码: (高槽 << 16) | (低槽 & 0xffff); 越界返回 None 且游标不动
/// v8: src/wasm/wasm-objects.cc:2752-2757 DecodeI32ExceptionValue
pub fn decode_i32_exception_value(values: &[u32], cursor: &mut usize) -> Option<u32> {
  let msb = *values.get(*cursor)?;
  let lsb = *values.get(*cursor + 1)?;
  *cursor += 2;
  Some((msb << 16) | (lsb & 0xffff))
}

/// i64 异常值解码: 两段 i32 拼 (高 32 << 32) | 低 32
/// v8: src/wasm/wasm-objects.cc:2759-2764 DecodeI64ExceptionValue
pub fn decode_i64_exception_value(values: &[u32], cursor: &mut usize) -> Option<u64> {
  let msb = u64::from(decode_i32_exception_value(values, cursor)?);
  let lsb = u64::from(decode_i32_exception_value(values, cursor)?);
  Some((msb << 32) | lsb)
}

// —— 提取不变式断言 ——
// 注: 本版工具链 const 上下文不支持 assert_eq!/调用非 const fn, 统一 assert! 折叠常量等式;
// 涉函数用例改由 tests/wasm_objects.rs 运行期断言覆盖
const _: () = {
  assert!(USES_RESERVED_SLOT_OFFSET == 1);
  assert!(USES_INITIAL_LENGTH == 3);
  assert!(USES_ENTRY_STEP == 2);
  assert!(MEMORY_RESERVE_HEURISTIC.len() == 4);
  assert!(MUST_GROW_IN_PLACE.len() == 4);
  assert!(GROW_FAIL_PATHS == 4);
  assert!(K_BYTES_PER_EXCEPTION_VALUES_ARRAY_ELEMENT == 2);
  assert!(CODE_POINTER_WIDTH == 4);
  assert!(INL_ACCESSOR_MACROS == 21);
  assert!(K_GB_PAGES == 16384);
  assert!(K_NO_MAXIMUM == -1);
};

// —— 方法面 (填体: 纯模型/规划器; 推迟: todo! 骨架, 逐条见 readme/zh.md 台账 B 节) ——

/// wire bytes 子区间提取 (base::Vector::SubVector; 越界 fail-closed 返回 None,
/// 原版 DCHECK 腐蚀态不可达, 本式拒绝; 复审纪律禁 panic)
/// v8: src/wasm/wasm-objects.cc:136 SubVector (unibrow 侧 DCHECK)
pub(crate) fn subvector(wire_bytes: &[u8], offset: usize, end_offset: usize) -> Option<&[u8]> {
  wire_bytes.get(offset..end_offset)
}

/// WasmModuleObject 方法面
/// v8: src/wasm/wasm-objects.cc:120 WasmModuleObject::New
pub fn moduleobject_new() {
  // 纯逻辑面 无: NewJSObject(wasm_module_constructor) + managed_native_module/script 两字段写
  todo!("就地修订: WasmModuleObject::New")
}
/// 名字字节切片提取: 原版 UTF8 校验在 decode 面 (DCHECK ValidateEncoding),
/// 字符串工厂 (InternalizeUtf8String/NewStringFromUtf8 lossy) 推迟 P1;
/// 本式填 SubVector 切片面 (name_vec = wire_bytes[offset..end_offset])
/// v8: src/wasm/wasm-objects.cc:132-148 WasmModuleObject::ExtractUtf8StringFromModuleBytes
pub fn moduleobject_extractutf8stringfrommodulebytes(
  wire_bytes: &[u8],
  offset: usize,
  end_offset: usize,
) -> Option<&[u8]> {
  // internalize/shared 分支只影响字符串形态 (推迟 P1), 切片提取与之无关
  subvector(wire_bytes, offset, end_offset)
}
/// 模块名: 未置名返回 None (原版空句柄), 置名走切片提取
/// v8: src/wasm/wasm-objects.cc:151-159 WasmModuleObject::GetModuleNameOrNull
pub fn moduleobject_getmodulenameornull(
  name_is_set: bool,
  wire_bytes: &[u8],
  offset: usize,
  end_offset: usize,
) -> Option<&[u8]> {
  if !name_is_set {
    return None;
  }
  moduleobject_extractutf8stringfrommodulebytes(wire_bytes, offset, end_offset)
}
/// 函数名: lazily_generated_names.LookupFunctionName 推迟 P5 (名字面),
/// 查询结果 (Option<(offset,end)>) 由调用方代入; 未置名返回 None
/// v8: src/wasm/wasm-objects.cc:161-173 WasmModuleObject::GetFunctionNameOrNull
pub fn moduleobject_getfunctionnameornull(
  name_ref: Option<(usize, usize)>,
  wire_bytes: &[u8],
) -> Option<&[u8]> {
  // 原版 DCHECK_LT(func_index, functions.size()) 系调用方前置
  let (offset, end) = name_ref?;
  moduleobject_extractutf8stringfrommodulebytes(wire_bytes, offset, end)
}
/// 裸函数名: 匿名哨兵 (-1) 返回空切片; 名字未置 (GetNameOrNull 空) 返回空切片
/// v8: src/wasm/wasm-objects.cc:175-189 WasmModuleObject::GetRawFunctionName
pub fn moduleobject_getrawfunctionname(
  func_index: i32,
  name_ref: Option<(usize, usize)>,
  wire_bytes: &[u8],
) -> &[u8] {
  if func_index == K_ANONYMOUS_FUNC_INDEX {
    return &[];
  }
  match name_ref {
    Some((offset, end)) => subvector(wire_bytes, offset, end).unwrap_or(&[]),
    None => &[],
  }
}

/// WasmTableObject::SetFunctionTablePlaceholder 的 Tuple2 条目模型
/// (原版 Tuple2(instance_object, Smi(func_index)) 抽取标识态)
/// v8: src/wasm/wasm-objects.cc:713-728 WasmTableObject::SetFunctionTablePlaceholder
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TablePlaceholder {
  /// 目标实例标识 (抽取自 instance_object)
  pub instance: u64,
  /// 函数索引 (原版 Smi)
  pub func_index: i32,
}

/// 表条目/调度条目写入记录 (空槽、隐式参引用、或函数表占位符)
/// v8: src/wasm/wasm-objects.cc:713-728 WasmTableObject::SetFunctionTablePlaceholder
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EntryWrite {
  /// 空 (原版 Smi::zero / kNull)
  Empty,
  /// 隐式参实例标识 (抽取自 Tagged<WasmTrustedInstanceData/WasmImportData>)
  ImplicitArg(u64),
  /// 函数表占位符 (Tuple2)
  Placeholder(TablePlaceholder),
}

/// WasmTableObject 方法面
/// 新建表规划器: CHECK(type.is_ref()) 越界返回 None; 派发面 = entries 初始填充/
/// 调度表建立/初始值批量更新三决策 (NewFixedArray/NewJSObject 等工厂面推迟 P1 P2)
/// v8: src/wasm/wasm-objects.cc:225-292 WasmTableObject::New
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TableNewPlan {
  /// 是否函数表 (原版 is_function_table, canonical_type.IsFunctionType())
  pub is_function_table: bool,
  /// 调度表初始长度 (函数表 initial, 否则空表 0)
  pub dispatch_initial_length: u32,
  /// 是否需对新表 0..initial 批量更新调度条目 (函数表且 initial>0 且初始值非
  /// wasm null 且是 funcref 四条件, .cc:246)
  pub update_dispatch_initial: bool,
}
pub fn tableobject_new(
  type_is_ref: bool,
  canonical_is_function_type: bool,
  initial: u32,
  initial_value_is_wasm_null: bool,
  initial_value_is_funcref: bool,
) -> Option<TableNewPlan> {
  if !type_is_ref {
    return None; // 原版 CHECK(type.is_ref()) 域外 fail-closed
  }
  let is_function_table = canonical_is_function_type;
  Some(TableNewPlan {
    is_function_table,
    dispatch_initial_length: if is_function_table { initial } else { 0 },
    update_dispatch_initial: is_function_table
      && initial > 0
      && !initial_value_is_wasm_null
      && initial_value_is_funcref,
  })
}
/// 表 Grow 结果 (count==0 退化返 old_size; 越限 Failed 对应原版 -1;
/// Grown 携 entries 扩容槽数与调度表新长; 条目 Set 初始化与调度表复制推迟面见台账)
/// v8: src/wasm/wasm-objects.cc:294-371 WasmTableObject::Grow
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TableGrowOutcome {
  /// return -1
  Failed,
  /// 退化: count==0, 返回 old_size
  Noop { old_size: u32 },
  /// 成功: 返回 old_size; entries_grow=None 表示 entries 容量够不扩 (.cc:311);
  /// 有调度表时 dispatch_grow_to=Some(new_size) (.cc:337-342);
  /// 初始化填充区间 [fill_start, fill_end) (.cc:366-368)
  Grown {
    old_size: u32,
    new_size: u32,
    entries_grow: Option<u32>,
    dispatch_grow_to: Option<u32>,
    fill_start: u32,
    fill_end: u32,
  },
}
pub fn tableobject_grow(
  old_size: u32,
  count: u32,
  maximum_length: Option<u64>,
  engine_max_table_size: u64,
  old_capacity: u32,
  has_dispatch_table: bool,
) -> TableGrowOutcome {
  if count == 0 {
    return TableGrowOutcome::Noop { old_size };
  }
  let max_size = tableobject_grow_max_size(maximum_length, engine_max_table_size);
  if tableobject_grow_exceeds(old_size, count, max_size) {
    return TableGrowOutcome::Failed;
  }
  let new_size = old_size + count; // 域内不溢 (原版 DCHECK_GE(kMaxInt,new_size))
  let entries_grow = tableobject_entries_grow_delta(new_size, old_capacity, max_size);
  let dispatch_grow_to = if has_dispatch_table {
    Some(new_size) // 原版 DCHECK_EQ(old_size, dispatch_table->length()) 前置由调用方保证
  } else {
    None
  };
  TableGrowOutcome::Grown {
    old_size,
    new_size,
    entries_grow,
    dispatch_grow_to,
    fill_start: old_size,
    fill_end: new_size,
  }
}
/// v8: src/wasm/wasm-objects.cc:373 WasmTableObject::JSToWasmElement
pub fn tableobject_jstowasmelement() {
  // 纯逻辑面 无: JSToWasmObject (JS 值→wasm 引用转换+类型校验+错误消息) 整体深面
  todo!("就地修订: WasmTableObject::JSToWasmElement")
}
/// 函数表条目写入操作决策: wasm null → 调度表 Clear(kExistingEntry) + 条目置
/// wasm_null; 否则 → GetOrCreateExternal (推迟 P4) + 批量更新 1 条目 + 条目写值
/// v8: src/wasm/wasm-objects.cc:383-400 WasmTableObject::SetFunctionTableEntry
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SetFunctionEntryOp {
  /// 条目与调度表清空 (原版 entry 写 ReadOnlyRoots().wasm_null)
  ClearToWasmNull,
  /// 调度表更新 + 条目写 funcref
  UpdateDispatch,
}
pub fn tableobject_setfunctiontableentry(entry_is_wasm_null: bool) -> SetFunctionEntryOp {
  if entry_is_wasm_null {
    SetFunctionEntryOp::ClearToWasmNull
  } else {
    SetFunctionEntryOp::UpdateDispatch
  }
}
/// 表条目写入操作决策 (值本体与 dispatch/entries 堆写推迟 P1 P2):
/// has_index: 有签名 → 函数表路径, 否则直写; generic 大类族直写, kFunc 函数表路径;
/// 原版 kBottom/kTop/kVoid/kExternString UNREACHABLE (腐蚀态防御, 禁 panic 同记 Reject)
/// v8: src/wasm/wasm-objects.cc:402-455 WasmTableObject::Set
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TableSetOp {
  /// 函数表路径 (SetFunctionTableEntry)
  FunctionEntry,
  /// 直写 entries
  DirectSet,
  /// 原版 UNREACHABLE 分支 (腐蚀值拒绝)
  Reject,
}
pub fn tableobject_set(table_type: &ValueType, has_signature: bool) -> TableSetOp {
  if table_type.has_index() {
    // 原版 DCHECK(table->has_trusted_data()); has_signature = module->has_signature(type.ref_index())
    return if has_signature {
      TableSetOp::FunctionEntry
    } else {
      TableSetOp::DirectSet
    };
  }
  match table_type.generic_kind() {
    // 原版 :421-439 直写族 (Extern/String/StringView*/Eq/Struct/Array/Any/I31/None/
    // NoFunc/NoExtern/Exn/NoExn/Cont/NoCont/Waitqueue/NoWaitqueue); kFunc 函数表路径;
    // kBottom/kTop/kVoid/kExternString 落 Reject (原版 UNREACHABLE)
    Some(GenericKind::Func) => TableSetOp::FunctionEntry,
    Some(
      GenericKind::Bottom | GenericKind::Top | GenericKind::Void | GenericKind::ExternString,
    ) => TableSetOp::Reject,
    Some(_) => TableSetOp::DirectSet,
    // 位域不可判别 (腐蚀态) → 同原版 UNREACHABLE 防御
    None => TableSetOp::Reject,
  }
}
/// 表条目读取操作决策 (条目值本体在堆面, 此处仅分发; LazyInit 后的 GetOrCreateFuncRef
/// 与回写推迟 P1 P2): null/funcref 直返; has_index 的 array/struct/cont 直返,
/// 否则签名元素走占位符惰性初始化; generic 直返族同 Set
/// v8: src/wasm/wasm-objects.cc:457-536 WasmTableObject::Get
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TableGetOp {
  /// 有效条目, 原样返回
  ReturnEntry,
  /// 占位符 (Tuple2), 惰性创建 func_ref 并回写
  LazyInit,
  /// 原版 UNREACHABLE 分支 (非法 unsafe_type 拒绝)
  Reject,
}
pub fn tableobject_get(
  entry_is_wasm_null: bool,
  entry_is_funcref: bool,
  table_type: &ValueType,
  element_is_array_struct_or_cont: bool,
) -> TableGetOp {
  if entry_is_wasm_null || entry_is_funcref {
    return TableGetOp::ReturnEntry;
  }
  if table_type.has_index() {
    // 原版 element_type = type(module).ref_index(); array/struct/cont → 直返,
    // 否则 DCHECK(module->has_signature) 落占位符路径
    return if element_is_array_struct_or_cont {
      TableGetOp::ReturnEntry
    } else {
      TableGetOp::LazyInit
    };
  }
  match table_type.generic_kind() {
    // 原版 :499-521: kFunc 置位 is_func 落占位符路径; 直返族同 Set; 腐蚀/不可判别 Reject
    Some(GenericKind::Func) => TableGetOp::LazyInit,
    Some(
      GenericKind::Bottom | GenericKind::Top | GenericKind::Void | GenericKind::ExternString,
    ) => TableGetOp::Reject,
    Some(_) => TableGetOp::ReturnEntry,
    None => TableGetOp::Reject, // 原版 `if (!is_func) UNREACHABLE()` 显式拒绝腐蚀
  }
}
/// Fill 前置校验: start/count 均不超当前长且 start+count 不越界
/// (原版三 DCHECK_LE; 防溢出无 panic 闭环校验); 通过后逐 i 走 Set 分发 (tableobject_set)
/// v8: src/wasm/wasm-objects.cc:538-552 WasmTableObject::Fill
pub fn tableobject_fill(start: u32, count: u32, current_length: u32) -> bool {
  is_in_bounds(start, count, current_length)
}
/// 批量更新派发 (UpdateDispatchTableMultiple .cc:192-222):
/// exported → 取 func_data/instance/module 后循环 count 次单条目更新;
/// 否则 DCHECK capi 循环 count 次 capi 更新; SBXCHECK_BOUNDS(func_index) 在单条目面
/// v8: src/wasm/wasm-objects.cc:192-222/601 WasmTableObject::UpdateDispatchTable (批量)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UpdateMultipleKind {
  /// WasmExportedFunction 路径 (需 SBXCHECK_BOUNDS 函数索引域校验)
  Exported,
  /// WasmCapiFunction 路径 (原版 DCHECK(IsWasmCapiFunction))
  Capi,
}
pub fn tableobject_updatedispatchtable_batch(
  external_is_exported_function: bool,
) -> UpdateMultipleKind {
  if external_is_exported_function {
    UpdateMultipleKind::Exported
  } else {
    UpdateMultipleKind::Capi
  }
}
/// 单条目更新派发决策 (.cc:623-676): implicit_arg = 导入函数走
/// dispatch_table_for_imports 槽位, 否则目标 instance_data; wrapper 缓存命中 →
/// SetForWrapper, 未命中 → SetForNonWrapper; 签名匹配失败 (原版
/// SBXCHECK(FunctionSigMatchesTable)) fail-closed 返回 None
/// v8: src/wasm/wasm-objects.cc:623-676 WasmTableObject::UpdateDispatchTable (单表)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ImplicitArgSource {
  /// 导入函数: 取 dispatch_table_for_imports 的 implicit_arg
  ImportsTable,
  /// 本模块函数: 目标 instance_data 本体
  TargetInstanceData,
}
/// 调度表条目写入形态
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SetOp {
  /// 带 wrapper (offheap 记引用)
  Wrapper,
  /// 直写 call_target
  NonWrapper,
}
pub fn tableobject_updatedispatchtable(
  func_is_imported: bool,
  sig_matches_table: bool,
  maybe_wrapper_present: bool,
) -> Option<(ImplicitArgSource, SetOp)> {
  if !sig_matches_table {
    return None; // 原版 SBXCHECK abort, 此处 fail-closed
  }
  Some((
    if func_is_imported {
      ImplicitArgSource::ImportsTable
    } else {
      ImplicitArgSource::TargetInstanceData
    },
    if maybe_wrapper_present {
      SetOp::Wrapper
    } else {
      SetOp::NonWrapper
    },
  ))
}
/// 签名匹配三择 (FunctionSigMatchesTable .cc:562-580): (1) 泛 funcref 表收任意函数;
/// (2) 常见终型按 canonical 索引相等; (3) 其余全量子类型检查 (canonicalizer 深面,
/// is_canonical_subtype 由调用方代入, 推迟 P5)
/// v8: src/wasm/wasm-objects.cc:561-580 FunctionSigMatchesTable
pub fn function_sig_matches_table(
  sig_id: CanonicalTypeIndex,
  table_type: CanonicalValueType,
  is_canonical_subtype: bool,
) -> bool {
  if !table_type.has_index() && table_type.generic_kind() == Some(GenericKind::Func) {
    return true;
  }
  if sig_id == table_type.ref_index() {
    return true;
  }
  is_canonical_subtype
}
/// 占位符条目决策: 原版 CHECK(has_instance_object) 腐蚀态 fail-closed 返回 None;
/// 实例标识与 func_index 由调用方抽取
/// v8: src/wasm/wasm-objects.cc:713-728 WasmTableObject::SetFunctionTablePlaceholder
pub fn tableobject_setfunctiontableplaceholder(
  has_instance_object: bool,
  instance: u64,
  func_index: i32,
) -> Option<EntryWrite> {
  if !has_instance_object {
    return None;
  }
  Some(EntryWrite::Placeholder(TablePlaceholder {
    instance,
    func_index,
  }))
}

/// WasmSuspendingObject 方法面
/// v8: src/wasm/wasm-objects.cc:730 WasmSuspendingObject::New
pub fn suspendingobject_new() {
  // 纯逻辑面 无: NewJSObject(wasm_suspending_constructor) + set_callable
  todo!("就地修订: WasmSuspendingObject::New")
}

/// WasmMemoryObject 方法面
/// v8: src/wasm/wasm-objects.cc:795 WasmMemoryObject::New (已含 buffer/backing_store 的接续面)
pub fn memoryobject_new_with_backing_store() {
  // 纯逻辑面 已析: DCHECK(maximum==kNoMaximum||>=0) 域外钳位见
  // memoryobject_new_reserved_pages 注; 函数本体为 CppGCManaged 创建 + ctor 字段写 +
  // SetNewBuffer/共享注册接续 (P1 P2 P3), 无独立纯算式
  todo!("就地修订: WasmMemoryObject::New (backing store)")
}
/// v8: src/wasm/wasm-objects.cc:844 WasmMemoryObject::New (按页数分配面; 预留启发式已填体
/// memoryobject_new_reserved_pages, BackingStore::AllocateWasmMemory 与接续推迟)
pub fn memoryobject_new_from_pages() {
  // 纯逻辑面 已填: 预留页数四分支启发式 (.cc:856-885)
  todo!("就地修订: WasmMemoryObject::New (页数)")
}
/// UseInInstance 分发决策: 共享内存 → instance 槽位写 (shared_memory_backing_stores
/// 保活, 不入弱引用追踪), 非共享 → instances 弱列表追加 (.cc:917-924)
/// v8: src/wasm/wasm-objects.cc:899-929 WasmMemoryObject::UseInInstance
pub fn memoryobject_useininstance(is_shared: bool) -> bool {
  is_shared
}
/// SetNewBuffer 操作规划: GSAB 预零长度必须先于任何分配 (GC 触发面), 冻结/注册限
/// 共享, FixUp 限可变长; 末步 array_buffer 写与符号属性 (推迟 P1) 恒执行
/// v8: src/wasm/wasm-objects.cc:932-962 WasmMemoryObject::SetNewBuffer
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MemorySetNewBufferPlan {
  /// 共享且 JS 可变形: set_byte_length(0) 预清零
  pub pre_zero_length: bool,
  /// 共享: SetIntegrityLevel(FROZEN) 冻结
  pub freeze: bool,
  /// 共享: GlobalBackingStoreRegistry 注册
  pub register_shared: bool,
  /// JS 可变形: FixUpResizableArrayBuffer (结果见 memoryobject_fixupresizablearraybuffer)
  pub fixup_resizable: bool,
}
pub fn memoryobject_setnewbuffer(
  is_shared: bool,
  is_resizable_by_js: bool,
) -> MemorySetNewBufferPlan {
  MemorySetNewBufferPlan {
    pre_zero_length: is_shared && is_resizable_by_js,
    freeze: is_shared,
    register_shared: is_shared,
    fixup_resizable: is_resizable_by_js,
  }
}
/// UpdateInstances 双层遍历纯检索面: 逐 instance 跳过已清弱引用 (None), 在
/// memory_objects 槽位表内找等于自身 id 的下标 (原版内层线性查找); 返回
/// (instance_idx, mem_idx) 供调用方接续 SetInstanceMemory (堆写推迟 P2)
/// v8: src/wasm/wasm-objects.cc:964-988 WasmMemoryObject::UpdateInstances
pub fn memoryobject_updateinstances(
  instances_memories: &[Option<&[u64]>],
  self_id: u64,
) -> Vec<(usize, usize)> {
  let mut hits = Vec::new();
  for (instance_idx, memories) in instances_memories.iter().enumerate() {
    let Some(memories) = memories else { continue }; // elem.IsCleared() 跳过
    for (mem_idx, &id) in memories.iter().enumerate() {
      if id == self_id {
        hits.push((instance_idx, mem_idx));
      }
    }
  }
  hits
}
/// 可变形 buffer 的 max_byte_length 计算: maximum_pages * kWasmPageSize;
/// 32 位平台 UINTPTR_MAX 不可表示时 (65536 页整 = 2^32) 回退 UINTPTR_MAX-页大小
/// (原版注释: 该 hack 实践不可观测); 64 位直乘
/// v8: src/wasm/wasm-objects.cc:990-1027 WasmMemoryObject::FixUpResizableArrayBuffer
pub fn memoryobject_fixupresizablearraybuffer(maximum_pages: i32) -> usize {
  if cfg!(target_pointer_width = "32") {
    let max_byte_length64 = (maximum_pages.max(0) as u64) * (WASM_PAGE_SIZE as u64);
    const UINTPTR_MAX32: u64 = u32::MAX as u64;
    if max_byte_length64 > UINTPTR_MAX32 {
      UINTPTR_MAX32 as usize - WASM_PAGE_SIZE
    } else {
      max_byte_length64 as usize
    }
  } else {
    maximum_pages.max(0) as usize * WASM_PAGE_SIZE
  }
}
/// RefreshBuffer 构造选择: 共享 → SharedArrayBuffer 构造 (+可选覆写可变形位,
/// 原版 DCHECK_IMPLIES(override.has_value(), bs_shared)), 非共享 → 普通构造;
/// 产物恒接 SetNewBuffer (.cc:1049)
/// v8: src/wasm/wasm-objects.cc:1029-1051 WasmMemoryObject::RefreshBuffer
pub fn memoryobject_refreshbuffer(
  is_shared: bool,
  override_resizable: Option<bool>,
) -> (bool, Option<bool>) {
  (is_shared, is_shared.then_some(override_resizable).flatten())
}
/// GetArrayBuffer 惰性刷新决策: array_buffer 为 undefined → RefreshBuffer,
/// 否则复用现 buffer (堆写 P3 推迟, 此处纯二择)
/// v8: src/wasm/wasm-objects.cc:1209-1223 WasmMemoryObject::GetArrayBuffer
pub fn memoryobject_getarraybuffer(buffer_is_undefined: bool) -> bool {
  buffer_is_undefined
}
/// Grow 全状态机规划器 (上限/原地判定/步进已填体的 helper 在此接续):
/// 分支序 1:1 .cc:1053-1207 —— 越限-1 → api 中断拒绝 → try 原地 → 必原地失败-1
/// (fuzzer FATAL 面记位) → 共享广播路径 → 非共享原地路径 → 拷贝增长路径
/// (new_capacity 钳位; 分配失败-1 并置非确定性位)。堆侧结果 (原地增长返回值/
/// 拷贝产物) 由调用方以 Option 注入; 返回 ret=-1 失败或旧页数 (共享为同步后值)
/// v8: src/wasm/wasm-objects.cc:1053-1207 WasmMemoryObject::Grow
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MemoryGrowOutcome {
  /// 返回值: -1 失败, 否则旧页数 (共享内存为 GrowWasmMemoryInPlace 同步值)
  pub ret: i32,
  /// 共享: BroadcastSharedWasmMemoryGrow
  pub broadcast: bool,
  /// 非共享成功: UpdateInstances
  pub update_instances: bool,
  /// 旧 buffer 需 Detach
  pub detach_old_buffer: bool,
  /// 旧 buffer 需置 undefined
  pub undefine_buffer: bool,
  /// 可变形旧 buffer 直改 byte_length (原地成功路径)
  pub set_buffer_byte_length: Option<usize>,
  /// managed_backing_store 上报 bit_ceil 尺寸 (pages>0 才上报)
  pub estimated_size: Option<usize>,
  /// 拷贝增长新 store (byte_length, capacity)
  pub new_backing_store: Option<(usize, usize)>,
  /// 拷贝分配失败: WasmEngine::set_had_nondeterminism
  pub nondeterminism: bool,
  /// correctness_fuzzer_suppressions 下原版 FATAL (禁 panic 纪律, 记位待上层)
  pub fuzzer_fatal_pending: bool,
}
/// 内存 Grow 状态机入参
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MemoryGrowParams {
  pub old_pages: usize,
  pub max_pages: usize,
  pub pages: u32,
  pub is_shared: bool,
  pub has_guard_regions: bool,
  pub is_resizable_by_js: bool,
  pub is_executing_api_interrupt: bool,
  pub stress_wasm_memory_moving: bool,
  pub has_old_buffer: bool,
  pub grown_byte_length: usize,
  pub grow_in_place_result: Option<usize>,
  pub copy_result: Option<(usize, usize)>,
  pub correctness_fuzzer_suppressions: bool,
}

pub fn memoryobject_grow(p: MemoryGrowParams) -> MemoryGrowOutcome {
  let fail = |nondeterminism: bool, fuzzer: bool| MemoryGrowOutcome {
    ret: -1,
    broadcast: false,
    update_instances: false,
    detach_old_buffer: false,
    undefine_buffer: false,
    set_buffer_byte_length: None,
    estimated_size: None,
    new_backing_store: None,
    nondeterminism,
    fuzzer_fatal_pending: fuzzer,
  };
  // .cc:1078-1085 上限与越限 (max_pages 由 memoryobject_max_size 求得)
  if memoryobject_grow_exceeds(p.old_pages, p.pages, p.max_pages) {
    return fail(false, false);
  }
  let must_grow_in_place = memoryobject_must_grow_in_place(
    p.is_shared,
    p.has_guard_regions,
    p.is_resizable_by_js,
    p.pages,
  );
  // .cc:1096-1101 api 中断期不允许增长非共享内存 (缓存 memory base 失效防护)
  if p.is_executing_api_interrupt && !p.is_shared && p.pages != 0 {
    return fail(false, false);
  }
  // .cc:1102-1107 try 原地 (必原地或非 stress 移动), 失败态由注入结果表达
  let try_grow_in_place = must_grow_in_place || !p.stress_wasm_memory_moving;
  let result_inplace = if try_grow_in_place {
    p.grow_in_place_result
  } else {
    None
  };
  // .cc:1108-1115 必原地而失败: fuzzer 下 FATAL, 否则 -1
  if must_grow_in_place && result_inplace.is_none() {
    return fail(false, p.correctness_fuzzer_suppressions);
  }
  if p.is_shared {
    // .cc:1117-1137 共享路径: 原版 DCHECK(result_inplace.has_value()) — 上一步已拦
    let Some(ret_pages) = result_inplace else {
      return fail(false, p.correctness_fuzzer_suppressions); // 腐蚀态 fail-closed 防御
    };
    return MemoryGrowOutcome {
      ret: ret_pages as i32,
      broadcast: true,
      update_instances: false,
      detach_old_buffer: false,
      // 广播应已重置 AB (非可变形旧 buffer): 原版 CHECK(undefined), 此处仅记账
      undefine_buffer: p.pages > 0 && p.has_old_buffer && !p.is_resizable_by_js,
      set_buffer_byte_length: None,
      estimated_size: (p.pages > 0).then(|| p.grown_byte_length.next_power_of_two()),
      new_backing_store: None,
      nondeterminism: false,
      fuzzer_fatal_pending: false,
    };
  }
  let new_pages = p.old_pages + p.pages as usize;
  if let Some(ret_pages) = result_inplace {
    // .cc:1142-1157 非共享原地成功: 可变形 buffer 直改长度, 否则 Detach+undefined
    debug_assert!(
      ret_pages == p.old_pages,
      "原版 DCHECK_EQ(result_inplace, old_pages) (.cc:1151)"
    );
    return MemoryGrowOutcome {
      ret: ret_pages as i32,
      broadcast: false,
      update_instances: true,
      detach_old_buffer: p.has_old_buffer && !p.is_resizable_by_js,
      undefine_buffer: p.has_old_buffer && !p.is_resizable_by_js,
      set_buffer_byte_length: if p.has_old_buffer && p.is_resizable_by_js {
        Some(new_pages * WASM_PAGE_SIZE)
      } else {
        None
      },
      estimated_size: (p.pages > 0).then(|| p.grown_byte_length.next_power_of_two()),
      new_backing_store: None,
      nondeterminism: false,
      fuzzer_fatal_pending: false,
    };
  }
  // .cc:1158-1207 拷贝增长: DCHECK(!has_old_buffer || !resizable)、DCHECK_LT(old,new)
  debug_assert!(new_pages > p.old_pages);
  let new_capacity = new_capacity_pages(p.max_pages, new_pages, p.old_pages);
  let Some((new_byte_length, _)) = p.copy_result else {
    // 原版分配失败: fuzzer FATAL + set_had_nondeterminism + -1
    return fail(true, p.correctness_fuzzer_suppressions);
  };
  MemoryGrowOutcome {
    ret: p.old_pages as i32,
    broadcast: false,
    update_instances: true,
    detach_old_buffer: p.has_old_buffer, // 此路径 has_old_buffer 必非可变形 (.cc:1158 DCHECK)
    undefine_buffer: p.has_old_buffer,
    set_buffer_byte_length: None,
    estimated_size: Some(new_byte_length.next_power_of_two()), // 拷贝成功恒 pages>0 上报
    new_backing_store: Some((new_byte_length, new_capacity)),
    nondeterminism: false,
    fuzzer_fatal_pending: false,
  }
}
/// ChangeArrayBufferResizability 决策树 (.cc:1225-1256): 现 buffer 已合要求 →
/// 原样复用; 共享 → RefreshBuffer(覆写位); 可变形位不符 → MakeResizableByJS +
/// 条件 Detach + RefreshBuffer; 其余 → RefreshBuffer
/// v8: src/wasm/wasm-objects.cc:1225-1256 WasmMemoryObject::ChangeArrayBufferResizability
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MemoryChangeResizabilityStep {
  /// 复用现 buffer, 无副作用
  KeepOld,
  /// 共享: RefreshBuffer(override=new_resizability)
  RefreshShared(bool),
  /// 位翻转: MakeWasmMemoryResizableByJS(new); 有旧 buffer 则 Detach; RefreshBuffer
  ToggleThenRefresh { detach_old: bool },
  /// 直接 RefreshBuffer(无覆写)
  RefreshPlain,
}
pub fn memoryobject_changearraybufferresizability(
  has_old_buffer: bool,
  old_buffer_resizable: bool,
  backing_store_resizable: bool,
  is_shared: bool,
  new_resizability: bool,
) -> MemoryChangeResizabilityStep {
  if has_old_buffer && old_buffer_resizable == new_resizability {
    return MemoryChangeResizabilityStep::KeepOld;
  }
  if is_shared {
    return MemoryChangeResizabilityStep::RefreshShared(new_resizability);
  }
  if backing_store_resizable != new_resizability {
    return MemoryChangeResizabilityStep::ToggleThenRefresh {
      detach_old: has_old_buffer,
    };
  }
  MemoryChangeResizabilityStep::RefreshPlain
}

/// WasmGlobalObject 方法面
/// v8: src/wasm/wasm-objects.cc:1258 WasmGlobalObject::New
pub fn globalobject_new() {
  // 纯逻辑面 已析: buffer 缺省时 offset 取 kMinRef/NumOffsetFromTaggedBuffer 二择
  // (.cc:1272-1285, 表达式串已登常量); 本体为 ctor 分配 + 字段写 (P1 P2) + 布局哨 A-1
  todo!("就地修订: WasmGlobalObject::New")
}

/// 全局存储选择 (.cc:1896-1914): 可变+导入 → 导入 buffer 槽 + 记录内偏移;
/// 否则按 is_ref 取 tagged/untagged globals buffer 与各自 OffsetOfElementAt 系
/// (tagged 走 FixedArray, untagged 走 ByteArray)。返回 (storage 选择, 索引/偏移索引)
/// v8: src/wasm/wasm-objects.cc:1896-1914 WasmTrustedInstanceData::GetGlobalStorage
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GlobalStorageSel {
  /// 导入可变全局: (mutable_imported_global_index, 需再读 buffers/offsets 两表)
  ImportedMutable(u32),
  /// tagged_globals_buffer + FixedArray::OffsetOfElementAt(index_in_buffer)
  TaggedGlobals(u32),
  /// untagged_globals_buffer + ByteArray::OffsetOfElementAt(index_in_buffer)
  UntaggedGlobals(u32),
}
pub fn trustedinstancedata_getglobalstorage(
  is_mutable: bool,
  is_imported: bool,
  is_ref: bool,
  mutable_imported_global_index: u32,
  index_in_buffer: u32,
) -> GlobalStorageSel {
  if is_mutable && is_imported {
    return GlobalStorageSel::ImportedMutable(mutable_imported_global_index);
  }
  if is_ref {
    GlobalStorageSel::TaggedGlobals(index_in_buffer)
  } else {
    GlobalStorageSel::UntaggedGlobals(index_in_buffer)
  }
}
/// 全局值读取形态 (.cc:1916-1933): is_ref → Tagged 槽 load (推迟 P2);
/// 数值 → 按 ValueKind 宽度非对齐读; f16 特殊 (fp16→fp32 转换)。返回读取宽度选择
/// v8: src/wasm/wasm-objects.cc:1916-1933 WasmTrustedInstanceData::GetGlobalValue
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GlobalValueRead {
  /// Tagged 引用槽
  Ref,
  /// 数值: 读取宽度 (1/2/4/8/16 字节)
  Num(u8),
  /// 原版 UNREACHABLE (kVoid/kTop/kBottom 等不可入全局)
  Reject,
}
pub fn trustedinstancedata_getglobalvalue(is_ref: bool, kind: ValueKind) -> GlobalValueRead {
  if is_ref {
    return GlobalValueRead::Ref;
  }
  match value_kind_size(kind) {
    // 原版 FOREACH_WASMVALUE_CTYPES 按 ctype 宽度非对齐读; 引用已由 is_ref 分支
    Some(size)
      if matches!(
        kind,
        ValueKind::I8
          | ValueKind::I16
          | ValueKind::I32
          | ValueKind::I64
          | ValueKind::F32
          | ValueKind::F64
          | ValueKind::S128
          | ValueKind::F16
      ) =>
    {
      GlobalValueRead::Num(size)
    }
    _ => GlobalValueRead::Reject, // kVoid/kTop/kBottom 与 Ref 外不可判别族, 原版 UNREACHABLE
  }
}

/// WasmTrustedInstanceData 方法面
/// 原始内存写入校验+落位: 非共享 CHECK_LE(size, engine_max_mem_bytes);
/// 共享 CHECK_NE(0)+对齐(kSizeOf=align_of usize)+已保活 backing_store 且
/// byte_length_address 相符 (size_address_eq 由调用方抽取)。越界 fail-closed
/// 返回 None 不写; 通过则写 bases_and_sizes[m..m+2], 返回是否需同步 memory0
/// 快访问字段 (.cc:1470-1473)。shared_backing_stores_len 为 CHECK_LT 前置
/// v8: src/wasm/wasm-objects.cc:1435-1474 WasmTrustedInstanceData::SetRawMemory
/// 原始内存写入入参
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SetRawMemoryParams {
  pub memory_index: usize,
  pub memories_count: usize,
  pub is_shared: bool,
  pub is_memory64: bool,
  pub engine_max_mem_bytes: usize,
  pub shared_backing_stores_len: usize,
  pub mem_start: usize,
  pub size_or_address: usize,
  pub size_address_eq: Option<usize>,
}

pub fn trustedinstancedata_setrawmemory(
  bases_and_sizes: &mut [usize],
  p: SetRawMemoryParams,
) -> Option<bool> {
  if p.memory_index >= p.memories_count {
    return None; // CHECK_LT(memory_index, memories.size())
  }
  if p.is_shared {
    if p.size_or_address == 0
      || !p.size_or_address.is_multiple_of(align_of::<usize>())
      || p.memory_index >= p.shared_backing_stores_len
      || p.size_address_eq != Some(p.size_or_address)
    {
      return None; // 共享三连 CHECK + byte_length_address 相符 CHECK_EQ
    }
  } else {
    let _ = p.is_memory64; // 上限选择 (max_mem64/32_bytes) 由调用方代入 engine_max_mem_bytes
    if p.size_or_address > p.engine_max_mem_bytes {
      return None; // CHECK_LE 域外 (腐蚀态) fail-closed
    }
  }
  let slots = bases_and_sizes.get_mut(p.memory_index * 2..p.memory_index * 2 + 2)?;
  slots[0] = p.mem_start;
  slots[1] = p.size_or_address;
  Some(p.memory_index == 0)
}
/// v8: src/wasm/wasm-objects.cc:1472 WasmTrustedInstanceData::New
pub fn trustedinstancedata_new() {
  // 纯逻辑面 已析: 各数组尺寸式 (.cc:1489-1530: imported_functions/well_known/
  // func_refs=m_functions.size/mutable global 表/elements 非空计数/tables/memories 偶数
  // 对齐/managed maps/feedback/tag 表) 属 New 体内联, 本体为全字段工厂分配 (P1 P2), 无独立纯算式
  todo!("就地修订: WasmTrustedInstanceData::New")
}
/// 数据段数组初始化映射: active 段长度记 0 (memory.init 与已 drop 被动段同义),
/// 被动段保留 source 长度; 产出 WireBytesRef{offset,length} 序 (data_segments()->set
/// 堆写推迟 P2, 此处纯映射). DCHECK num==0 || num==segments.size() 由调用方保证
/// v8: src/wasm/wasm-objects.cc:1630-1648 WasmTrustedInstanceData::InitDataSegmentArrays
pub fn trustedinstancedata_initdatasegmentarrays(segments: &[(bool, u32, u32)]) -> Vec<(u32, u32)> {
  segments
    .iter()
    .map(|&(active, offset, len)| (offset, if active { 0 } else { len }))
    .collect()
}
/// 调用目标来源 (.cc:1652-1665): SBXCHECK_BOUNDS 越界 fail-closed None;
/// 导入函数 → dispatch_table_for_imports 槽; jitless → kInvalidWasmCodePointer;
/// 否则 native_module 代码指针柄 (推迟 P4)
/// v8: src/wasm/wasm-objects.cc:1652-1665 WasmTrustedInstanceData::GetCallTarget
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CallTargetSource {
  /// 导入函数: 取 imports 调度表 target 槽
  Imported,
  /// jitless: 无效码指针
  Invalid,
  /// 本模块: NativeModule::GetCodePointerHandle (P4)
  NativeHandle,
}
pub fn trustedinstancedata_getcalltarget(
  func_index: u32,
  num_functions: u32,
  num_imported_functions: u32,
  jitless: bool,
) -> Option<CallTargetSource> {
  if func_index >= num_functions {
    return None; // SBXCHECK_BOUNDS
  }
  if func_index < num_imported_functions {
    return Some(CallTargetSource::Imported);
  }
  if jitless {
    return Some(CallTargetSource::Invalid);
  }
  Some(CallTargetSource::NativeHandle)
}
/// 表间拷贝执行决策 (越界为 None 对应原版 return false)
/// v8: src/wasm/wasm-objects.cc:1667-1706 WasmTrustedInstanceData::CopyTableEntries
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TableCopyOp {
  /// 无需操作 (同表同位置或 count == 0)
  Noop,
  /// 正向拷贝: src + i -> dst + i
  Forward,
  /// 反向拷贝: src < dst 时防重叠覆盖 (src + count - 1 - i -> dst + count - 1 - i)
  Backward,
}

/// 表间拷贝 (table.init): CHECK_LT 双表界由调用方前置; IsInBounds 越界返回 None
/// (原版 return false); 通过则返回明确的 TableCopyOp 拷贝规划 (Noop / Forward / Backward)
/// v8: src/wasm/wasm-objects.cc:1667-1706 WasmTrustedInstanceData::CopyTableEntries
pub fn trustedinstancedata_copytableentries(
  dst: u32,
  src: u32,
  count: u32,
  dst_table_len: u32,
  src_table_len: u32,
  same_table: bool,
) -> Option<TableCopyOp> {
  if !is_in_bounds(dst, count, dst_table_len) || !is_in_bounds(src, count, src_table_len) {
    return None;
  }
  if (same_table && dst == src) || count == 0 {
    return Some(TableCopyOp::Noop);
  }
  if src < dst {
    Some(TableCopyOp::Backward)
  } else {
    Some(TableCopyOp::Forward)
  }
}
/// IsInBounds(start, size, bound): start+size<=bound 且 size<=bound (base/limits.h 语义)
pub(crate) fn is_in_bounds(start: u32, count: u32, bound: u32) -> bool {
  start <= bound && count <= bound - start
}
/// 元素段→表初始化拷贝 (elem.init): 段界/表界两次 IsInBounds, 失败分别返回 trap
/// 种类 (原版 MessageTemplate); 段加载初始化 (InitializeElementSegment) 推迟 P2
/// v8: src/wasm/wasm-objects.cc:1708-1742 WasmTrustedInstanceData::InitTableEntries
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InitTableEntriesError {
  /// kWasmTrapTableOutOfBounds
  TableOutOfBounds,
  /// kWasmTrapElementSegmentOutOfBounds
  ElementSegmentOutOfBounds,
}
pub fn trustedinstancedata_inittableentries(
  dst: u32,
  src: u32,
  count: u32,
  table_len: u32,
  segment_len: u32,
) -> Option<InitTableEntriesError> {
  if !is_in_bounds(dst, count, table_len) {
    return Some(InitTableEntriesError::TableOutOfBounds);
  }
  if !is_in_bounds(src, count, segment_len) {
    return Some(InitTableEntriesError::ElementSegmentOutOfBounds);
  }
  None
}
/// func_refs 槽取用: Smi 占位 (未创建) → None, Ref → Some
/// v8: src/wasm/wasm-objects.cc:1744-1749 WasmTrustedInstanceData::try_get_func_ref
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FuncRefSlot<T> {
  /// IsSmi: 未创建
  Smi,
  /// 既有 func_ref (抽取表示)
  Ref(T),
}
pub fn trustedinstancedata_try_get_func_ref<T>(slot: &FuncRefSlot<T>) -> Option<&T> {
  match slot {
    FuncRefSlot::Smi => None,
    FuncRefSlot::Ref(r) => Some(r),
  }
}
/// v8: src/wasm/wasm-objects.cc:1771 WasmTrustedInstanceData::GetOrCreateFuncRef
pub fn trustedinstancedata_getorcreatefuncref() {
  // 纯逻辑面 已析: try_get 快路径 (已填体) + is_import = func_index <
  // num_imported_functions + SBXCHECK(!is_import||Is<WasmImportData>); 本体为
  // internal/funcref 工厂分配与回写 (P1 P2 P4)
  todo!("就地修订: WasmTrustedInstanceData::GetOrCreateFuncRef")
}

/// WasmInternalFunction 方法面
/// external 槽取用: undefined 占位 (未创建) → None, 既有 JSFunction (抽取标识) → Some
/// v8: src/wasm/wasm-objects.cc:1832-1837 WasmInternalFunction::try_get_external
pub fn internalfunction_try_get_external<T>(external: &Option<T>) -> Option<&T> {
  external.as_ref()
}
/// v8: src/wasm/wasm-objects.cc:1839-1865 WasmInternalFunction::GetOrCreateExternal
pub fn internalfunction_getorcreateexternal() {
  // 纯逻辑面 已析: try_get_external 快路径 (已填体); 本体 CreateExportedFunction
  // (工厂+wrapper 缓存, P1 P4)
  todo!("就地修订: WasmInternalFunction::GetOrCreateExternal")
}

/// WasmExportedFunction 方法面
/// v8: src/wasm/wasm-objects.cc:1867-1894 WasmExportedFunction::GetWrapper
pub fn exportedfunction_getwrapper() {
  // 纯逻辑面 无: wrapper 缓存/内建码选择/编译 (P4 wrapper cache + builtins)
  todo!("就地修订: WasmExportedFunction::GetWrapper")
}
/// 导出函数判定谓词 (.cc:2847-2863): IsJSFunction ∧ (code 为 JS_TO_WASM_FUNCTION ∨
/// builtin ∈ {JSToWasmWrapper, WasmPromising, WasmStressSwitch} ∧
/// shared 携 WasmExportedFunctionData)。抽取位由调用方代入
/// v8: src/wasm/wasm-objects.cc:2847-2863 WasmExportedFunction::IsWasmExportedFunction
pub fn exportedfunction_iswasmexportedfunction(
  is_jsfunction: bool,
  code_is_js_to_wasm_function: bool,
  builtin_is_jstowasm_wrapper: bool,
  builtin_is_promising: bool,
  builtin_is_stress_switch: bool,
  shared_has_exported_data: bool,
) -> bool {
  if !is_jsfunction {
    return false;
  }
  let wrapper_hit = code_is_js_to_wasm_function
    || builtin_is_jstowasm_wrapper
    || builtin_is_promising
    || builtin_is_stress_switch;
  // DRUMBRAKE 态另认 kJSToWasmInterpreterWrapper (P6, 现以 wrapper 位代入)
  wrapper_hit && shared_has_exported_data
}
/// v8: src/wasm/wasm-objects.cc:2906 WasmExportedFunction::New (按 wrapper 推导 promise 的重载)
pub fn exportedfunction_new() {
  // 纯逻辑面 已析: promise = builtin_id==kWasmPromising ? kPromise : kNoPromise
  // (.cc:2913-2916 一行三元的推导已并入本注, 本体为工厂分配链 P1 P4 P7)
  todo!("就地修订: WasmExportedFunction::New (推导面)")
}
/// v8: src/wasm/wasm-objects.cc:2920 WasmExportedFunction::New (全参重载)
pub fn exportedfunction_new_full() {
  // 纯逻辑面 无: function_data/SFI/JSFunction 工厂链与 script/external 回写 (P1 P4 P7)
  todo!("就地修订: WasmExportedFunction::New (全参)")
}

/// WasmStruct 方法面
/// GC 安全类型解引用链决策: map → (unchecked) WasmTypeInfo → type_index →
/// canonicalizer.LookupStruct (P5 推迟, 返回索引待上层查表); 原版 DCHECK
/// instance_type ∈ {WASM_STRUCT, WASM_CUSTOM_MAP}, 域外 fail-closed None
/// v8: src/wasm/wasm-objects.cc:1935-1945 WasmStruct::GcSafeType
pub fn struct_gcsafetype(
  map_instance_type_is_struct_or_custom_map: bool,
  type_info_type_index: u32,
) -> Option<u32> {
  if !map_instance_type_is_struct_or_custom_map {
    return None;
  }
  Some(type_info_type_index)
}
/// 描述符 struct 分配前置决策: num_supertypes = subtyping_depth+1 (恒 ≥1);
/// 共享型原版 UNIMPLEMENTED → None fail-closed; prototype 条件 =
/// wasm_js_interop ∧ 首字段非 Smi ∧ 是 JSReceiver (.cc:2012-2017)
/// v8: src/wasm/wasm-objects.cc:1979-2033 WasmStruct::AllocateDescriptorUninitialized
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DescriptorAllocPlan {
  /// described_index (module.canonical_type_id(type.describes), P5 代入)
  pub described_canonical_index: CanonicalTypeIndex,
  pub num_supertypes: i32,
  /// 需 Map::SetPrototype(rtt, prototype)
  pub set_prototype: bool,
}
pub fn struct_allocatedescriptoruninitialized(
  type_is_shared: bool,
  described_canonical_index: CanonicalTypeIndex,
  subtyping_depth: i32,
  wasm_js_interop: bool,
  first_field_is_smi: bool,
  first_field_is_jsreceiver: bool,
) -> Option<DescriptorAllocPlan> {
  if type_is_shared {
    return None; // UNIMPLEMENTED 分支 fail-closed
  }
  Some(DescriptorAllocPlan {
    described_canonical_index,
    num_supertypes: subtyping_depth + 1,
    set_prototype: wasm_js_interop && !first_field_is_smi && first_field_is_jsreceiver,
  })
}
/// 字段寻址纯面: offset = header_size + field_offset; 读取形态按字段 kind
/// (Ref → Tagged load, F16 → fp16→fp32 转换读, kVoid/kTop/kBottom UNREACHABLE →
/// Reject); 本体堆读推迟 P2, field_offset 由 canonical struct 布局 (P5) 代入
/// v8: src/wasm/wasm-objects.cc:2102-2137 GetFieldValueImpl / :2139 WasmStruct::GetFieldValue
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FieldRead {
  /// 数值/f16: 按 kind 宽度非对齐读 (f16 读出后 fp16→fp32)
  Kind(ValueKind),
  /// 引用: TaggedField load
  Ref,
  /// 原版 UNREACHABLE (kVoid/kTop/kBottom 腐蚀防御)
  Reject,
}
pub fn struct_getfieldvalue(
  field_offset: usize,
  header_size: usize,
  field_kind: ValueKind,
) -> (usize, FieldRead) {
  let read = match field_kind {
    ValueKind::Ref | ValueKind::RefNull => FieldRead::Ref,
    ValueKind::Void | ValueKind::Top | ValueKind::Bottom => FieldRead::Reject,
    k => FieldRead::Kind(k),
  };
  (header_size + field_offset, read)
}

/// WasmCustomMap 方法面
/// 合并描述符态分配前置决策: 被描述型自身是描述符 → CustomMap 尺寸/实例型,
/// 否则 Struct 尺寸 (WasmCustomMap::Size/WasmStruct::Size 布局推迟 A-1 P2);
/// prototype 条件同 struct 侧; 共享型 UNIMPLEMENTED → None
/// v8: src/wasm/wasm-objects.cc:2049-2100 WasmCustomMap::AllocateUninitialized
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CustomMapSizeSel {
  /// 被描述型是描述符 (决定 described_size/instance_type 二择)
  pub described_is_descriptor: bool,
  pub num_supertypes: i32,
  pub set_prototype: bool,
}
pub fn custommap_allocateuninitialized(
  type_is_shared: bool,
  described_is_descriptor: bool,
  subtyping_depth: i32,
  wasm_js_interop: bool,
  first_field_is_smi: bool,
  first_field_is_jsreceiver: bool,
) -> Option<CustomMapSizeSel> {
  if type_is_shared {
    return None;
  }
  Some(CustomMapSizeSel {
    described_is_descriptor,
    num_supertypes: subtyping_depth + 1,
    set_prototype: wasm_js_interop && !first_field_is_smi && first_field_is_jsreceiver,
  })
}
/// 同 struct 侧共用 GetFieldValueImpl, 仅 header_size 取 CustomMap 头 (A-1 布局代入)
/// v8: src/wasm/wasm-objects.cc:2143-2145 WasmCustomMap::GetFieldValue
pub fn custommap_getfieldvalue(
  field_offset: usize,
  header_size: usize,
  field_kind: ValueKind,
) -> (usize, FieldRead) {
  struct_getfieldvalue(field_offset, header_size, field_kind)
}
/// v8: src/wasm/wasm-objects.cc:2147-2176 WasmCustomMap::CreateJSWrapper
pub fn custommap_createjswrapper() {
  // 纯逻辑面 无: NewMapWithMetaMap/NewJSObjectFromMap 工厂链与 prototype/extensible 写
  // (P1 P2 P7); 前置 DCHECK(IsNull(js_wrapper)) 属调用方复查面
  todo!("就地修订: WasmCustomMap::CreateJSWrapper")
}

/// WasmArray 方法面
/// 元素寻址纯面: element_offset(index) 由长度/类型布局给出 (A-1 P2 代入),
/// 读取形态同字段族 — packed i8/i16 在此路径原版亦经 FOREACH 宏读出
/// v8: src/wasm/wasm-objects.cc:2178-2206 WasmArray::GetElement
pub fn array_getelement(_element_offset: usize, element_kind: ValueKind) -> FieldRead {
  match element_kind {
    ValueKind::Ref | ValueKind::RefNull => FieldRead::Ref,
    ValueKind::Void | ValueKind::Top | ValueKind::Bottom => FieldRead::Reject,
    k => FieldRead::Kind(k),
  }
}
/// 置位前置校验: 元素型须是引用 (原版 DCHECK is_ref); 通过后 TaggedField store +
/// 条件写屏障 (堆面 P2 推迟)
/// v8: src/wasm/wasm-objects.cc:2208-2216 WasmArray::SetTaggedElement
pub fn array_settaggedelement(element_is_ref: bool) -> bool {
  element_is_ref
}

/// WasmTagObject 方法面
/// v8: src/wasm/wasm-objects.cc:2218-2236 WasmTagObject::New
pub fn tagobject_new() {
  // 纯逻辑面 无: NewJSObject(wasm_tag_constructor) + canonical_type_index/tag/
  // trusted_data 字段写 (P1 P2)
  todo!("就地修订: WasmTagObject::New")
}
/// 签名匹配: CanonicalTypeIndex{canonical_type_index 字段 as u32} == 期望索引
/// v8: src/wasm/wasm-objects.cc:2238-2241 WasmTagObject::MatchesSignature
pub fn tagobject_matchessignature(canonical_type_index: i32, expected: CanonicalTypeIndex) -> bool {
  CanonicalTypeIndex::new(canonical_type_index as u32) == expected
}

/// WasmCapiFunction 方法面
/// v8: src/wasm/wasm-objects.cc:2243-2245 WasmCapiFunction::sig
pub fn capifunction_sig() {
  // 纯逻辑面 无: shared()->wasm_capi_function_data()->internal()->sig() 三段字段链 (P7 P2)
  todo!("就地修订: WasmCapiFunction::sig")
}
/// 签名匹配: canonical sig 索引相等 (原版 DEBUG 段 CHECK 各类型无索引; TODO(14034)
/// 若允许超型再改子类型检查); 越位索引不可达 (kV8MaxWasmTypes ≤ int32), as 安全
/// v8: src/wasm/wasm-objects.cc:2678-2692 WasmCapiFunction::MatchesSignature
pub fn capifunction_matchessignature(sig_index: i32, other_canonical_sig_index: u32) -> bool {
  CanonicalTypeIndex::new(sig_index as u32) == CanonicalTypeIndex::new(other_canonical_sig_index)
}
/// capi 函数判定: IsJSFunction ∧ shared 携 WasmCapiFunctionData (code kind 检查
/// 原版注释停用, TODO(jkummerow))
/// v8: src/wasm/wasm-objects.cc:2865-2878 WasmCapiFunction::IsWasmCapiFunction
pub fn capifunction_iswasmcapifunction(is_jsfunction: bool, shared_has_capi_data: bool) -> bool {
  is_jsfunction && shared_has_capi_data
}
/// v8: src/wasm/wasm-objects.cc:2880-2903 WasmCapiFunction::New
pub fn capifunction_new() {
  // 纯逻辑面 无: ExternalReference 重定向 + CapiFunctionData/SFI/JSFunctionBuilder
  // 工厂链与 external 回写 (P1 P4 P7)
  todo!("就地修订: WasmCapiFunction::New")
}

/// 调度表条目隐式参句柄标识 (抽取自 shared_ptr<WasmWrapperHandle>; 与原版一致的
/// offheap wrappers_ 映射键为 int index, 值为句柄)
/// v8: src/wasm/wasm-objects.h WasmDispatchTableData::wrappers_
pub type WrapperHandle = u64;

/// 新建/覆写条目区分 (与 consts::NewOrExistingEntry repr(u8) 0/1 对齐;
/// consts 模块接线后二者合流为单一来源)
/// v8: src/wasm/wasm-objects.h WasmDispatchTable::NewOrExistingEntry
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum DispatchNewOrExisting {
  /// kNewEntry: 初始化新条目, 不得读未初始化内存
  New = 0,
  /// kExistingEntry: 覆写既有条目, 需递减被覆写 wrapper 引用计数 (offheap Remove)
  Existing = 1,
}

/// 调度表条目纯模型 (原版为堆内 fixed-array 三字段 target/implicit_arg/sig;
/// jitless+DRUMBRAKE 时 function_index 复用 target 槽, kFunctionIndexBias=kTargetBias)
/// v8: src/wasm/wasm-objects.cc:2279-2318 SetForNonWrapper (模板族)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DispatchEntry {
  /// 调用目标代码指针 (u32 沙箱化 code pointer)
  pub target: u32,
  /// 规范签名索引 (WasmDispatchTable 有; ForImports 无 sig 槽恒 -1)
  pub sig: i32,
  /// 隐式参 (WasmTrustedInstanceData / WasmImportData 句柄, 或空)
  pub implicit_arg: EntryWrite,
  /// offheap wrapper 句柄 (SetForWrapper 写入, 覆写/清空时移除)
  pub wrapper: Option<WrapperHandle>,
}

impl DispatchEntry {
  /// 原版 wasm::kInvalidWasmCodePointer (Clear 写入的失效哨兵)
  /// 注: wasm-constants.h 未导出该常量名, Clear 以 u32::MAX 表征哨兵位, consts 接线后合流
  pub const INVALID_CODE_POINTER: u32 = u32::MAX;
  /// 新建零值条目 (隐式参空, target 0, wrapper 无; sig 依 has_sig 由调用方定)
  pub const EMPTY: DispatchEntry = DispatchEntry {
    target: 0,
    sig: -1,
    implicit_arg: EntryWrite::Empty,
    wrapper: None,
  };
}

/// 调度表纯模型: length/capacity 为条目单位 (原版 kLengthOffset/kCapacityOffset);
/// has_sig 区分 WasmDispatchTable(true)/ForImports(false); jitless/drumbrake 复刻
/// v8_flags.wasm_jitless 与 V8_ENABLE_DRUMBRAKE 编译/运行期分支
/// v8: src/wasm/wasm-objects.cc:2279-2482 dispatch table 模板族
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DispatchTableModel {
  /// 有效条目数 (原版 length, SBXCHECK_BOUNDS 上界)
  pub length: u32,
  /// 已分配容量 (原版 capacity)
  pub capacity: u32,
  /// 是否含 sig 槽 (WasmDispatchTable=true / ForImports=false)
  pub has_sig: bool,
  /// v8_flags.wasm_jitless
  pub jitless: bool,
  /// V8_ENABLE_DRUMBRAKE (jitless 下 function_index 落 target 槽需此编译开关)
  pub drumbrake: bool,
  /// 条目向量 (len == capacity)
  pub entries: Vec<DispatchEntry>,
}

/// 单个条目的写入意图 (非 wrapper / wrapper 两路, 携 jitless function_index 与 sig_id)
/// v8: src/wasm/wasm-objects.cc:2279-2393 SetForNonWrapper / SetForWrapper
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DispatchSetWrite {
  /// 非 wrapper: 隐式参为 WasmTrustedInstanceData, 直写 call_target
  NonWrapper {
    implicit_arg: u64,
    call_target: u32,
    function_index: u32,
    sig_id: u32,
  },
  /// wrapper: 隐式参为 WasmImportData, 写 wrapper code_pointer 并登记 offheap 句柄
  Wrapper {
    implicit_arg: u64,
    wrapper_handle: WrapperHandle,
    code_pointer: u32,
    function_index: u32,
    sig_id: u32,
  },
}

/// 条目写核心: 1:1 复刻 SetForNonWrapper/SetForWrapper 堆写序 (SBXCHECK_BOUNDS →
/// 视 jitless 写 target/function_index → 视 existing 递减 wrapper → 写隐式参 →
/// 视 has_sig 写 sig)。越界 fail-closed 返回 None (替代原版 SBXCHECK_BOUNDS abort)。
/// sig 面 ForImports (has_sig=false) 忽略 sig_id, 与原版 `requires { kSigBias }` 一致。
/// v8: src/wasm/wasm-objects.cc:2279-2393 SetForNonWrapper / SetForWrapper
pub(crate) fn entry_set(
  table: &mut DispatchTableModel,
  index: usize,
  new_or_existing: DispatchNewOrExisting,
  write: DispatchSetWrite,
) -> Option<()> {
  // SBXCHECK_BOUNDS(index, dispatch_table->length())
  if index >= table.length as usize {
    return None;
  }
  let entry = table.entries.get_mut(index)?;
  match write {
    DispatchSetWrite::NonWrapper {
      implicit_arg,
      call_target,
      function_index,
      sig_id,
    } => {
      if !table.jitless {
        // 覆写既有: offheap_data()->Remove(index) 递减 wrapper 引用
        if new_or_existing == DispatchNewOrExisting::Existing {
          entry.wrapper = None;
        }
        entry.target = call_target;
      } else if table.drumbrake {
        // jitless: 忽略 call_target, function_index 复用 target 槽
        entry.target = function_index;
      }
      entry.implicit_arg = EntryWrite::ImplicitArg(implicit_arg);
      if table.has_sig {
        entry.sig = sig_id as i32;
      }
    }
    DispatchSetWrite::Wrapper {
      implicit_arg,
      wrapper_handle,
      code_pointer,
      function_index,
      sig_id,
    } => {
      // 隐式参先写 (原版顺序: implicit_arg → target)
      entry.implicit_arg = EntryWrite::ImplicitArg(implicit_arg);
      if !table.jitless {
        if new_or_existing == DispatchNewOrExisting::Existing {
          entry.wrapper = None;
        }
        entry.target = code_pointer;
        // offheap_data()->Add(index, wrapper_handle)
        entry.wrapper = Some(wrapper_handle);
      } else if table.drumbrake {
        entry.target = function_index;
      }
      if table.has_sig {
        entry.sig = sig_id as i32;
      }
    }
  }
  Some(())
}

/// 条目清空核心: SBXCHECK_BOUNDS → 视 existing 递减 wrapper → 隐式参置空 →
/// target 写失效哨兵 → 视 has_sig 写 sig=-1。
/// (WasmDispatchTable::Clear 额外 DRUMBRAKE 间接调用缓存清理循环依赖 isolate/
/// interpreter 深面, 推迟 P6, 见 readme/zh.md)
/// v8: src/wasm/wasm-objects.cc:2427-2447 DispatchTableClear
pub(crate) fn entry_clear(
  table: &mut DispatchTableModel,
  index: usize,
  new_or_existing: DispatchNewOrExisting,
) -> Option<()> {
  if index >= table.length as usize {
    return None;
  }
  let entry = table.entries.get_mut(index)?;
  if new_or_existing == DispatchNewOrExisting::Existing {
    entry.wrapper = None;
  }
  entry.implicit_arg = EntryWrite::Empty;
  entry.target = DispatchEntry::INVALID_CODE_POINTER;
  if table.has_sig {
    entry.sig = -1;
  }
  Some(())
}

/// 离堆 wrapper 映射 (原版 WasmDispatchTableData::wrappers_: std::map<int, handle>;
/// 纯模型以按插入序去重的 Vec<(index, handle)> 承载)
pub type WasmWrapperMap = Vec<(i32, WrapperHandle)>;

/// DEBUG 取 wrapper code_pointer: CHECK_NE(it, end()) 缺失 fail-closed 返回 None
/// (返回句柄标识, 句柄 -> code_pointer() 解引用为深面)
/// v8: src/wasm/wasm-objects.cc:2248-2253 WasmDispatchTableData::WrapperCodePointerForDebugging
pub fn dispatchtabledata_wrappercodepointerfordebugging(
  wrappers: &WasmWrapperMap,
  index: i32,
) -> Option<WrapperHandle> {
  wrappers.iter().find(|(k, _)| *k == index).map(|(_, h)| *h)
}
/// 新增 wrapper: emplace(index, handle); 键存在则不插入并返回 was_inserted=false
/// (原版 DCHECK(was_inserted))
/// v8: src/wasm/wasm-objects.cc:2265-2271 WasmDispatchTableData::Add
pub fn dispatchtabledata_add(
  wrappers: &mut WasmWrapperMap,
  index: i32,
  handle: WrapperHandle,
) -> bool {
  if wrappers.iter().any(|(k, _)| *k == index) {
    return false;
  }
  wrappers.push((index, handle));
  true
}
/// 移除 wrapper: erase(index); 返回删除条数 (0/1)
/// v8: src/wasm/wasm-objects.cc:2273 WasmDispatchTableData::Remove
pub fn dispatchtabledata_remove(wrappers: &mut WasmWrapperMap, index: i32) -> usize {
  let before = wrappers.len();
  wrappers.retain(|(k, _)| *k != index);
  before - wrappers.len()
}

/// WasmDispatchTable 方法面 (entry_set/entry_clear 核心 + 公开包装)
/// 非 wrapper 直写: 隐式参 = WasmTrustedInstanceData, call_target = 代码指针
/// v8: src/wasm/wasm-objects.cc:2320-2333 WasmDispatchTable::SetForNonWrapper
pub fn dispatchtable_setfornonwrapper(
  table: &mut DispatchTableModel,
  index: usize,
  implicit_arg: u64,
  call_target: u32,
  sig_id: u32,
  function_index: u32,
  new_or_existing: DispatchNewOrExisting,
) -> Option<()> {
  entry_set(
    table,
    index,
    new_or_existing,
    DispatchSetWrite::NonWrapper {
      implicit_arg,
      call_target,
      function_index,
      sig_id,
    },
  )
}
/// 调度表 wrapper 写入入参
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SetWrapperParams {
  pub implicit_arg: u64,
  pub wrapper_handle: WrapperHandle,
  pub code_pointer: u32,
  pub sig_id: u32,
  pub function_index: u32,
  pub new_or_existing: DispatchNewOrExisting,
}

/// wrapper 写入: 隐式参 = WasmImportData, wrapper code_pointer 直写并登记句柄
/// v8: src/wasm/wasm-objects.cc:2395-2409 WasmDispatchTable::SetForWrapper
pub fn dispatchtable_setforwrapper(
  table: &mut DispatchTableModel,
  index: usize,
  p: SetWrapperParams,
) -> Option<()> {
  entry_set(
    table,
    index,
    p.new_or_existing,
    DispatchSetWrite::Wrapper {
      implicit_arg: p.implicit_arg,
      wrapper_handle: p.wrapper_handle,
      code_pointer: p.code_pointer,
      function_index: p.function_index,
      sig_id: p.sig_id,
    },
  )
}
/// 清空条目 (核心 entry_clear; DRUMBRAKE 间接调用缓存清理推迟 P6)
/// v8: src/wasm/wasm-objects.cc:2449-2478 WasmDispatchTable::Clear
pub fn dispatchtable_clear(
  table: &mut DispatchTableModel,
  index: usize,
  new_or_existing: DispatchNewOrExisting,
) -> Option<()> {
  entry_clear(table, index, new_or_existing)
}

/// uses 列表条目 (原版 ProtectedWeakFixedArray 每 2 槽 (instance, table_index) 一对)
/// v8: src/wasm/wasm-objects.cc:2489-2564 AddUse / MaybeGrowUsesList
pub type UseEntry = (u64, i32);

/// uses 压实/扩容: 1:1 复刻 MaybeGrowUsesList 三态 —— 空表初始化 (1 条目区),
/// 有富余直接返回, 满则先压实 (cleared >= min_free_entries 就地收拢), 压实不足则
/// 扩容 50% 且至少 1 条目 (新数组搬运活条目, 已压实前缀)。以条目为单位建模,
/// 槽容量 = 条目数*2+1 与原版算术严格对应。无堆分配 (Vec resize 表征新数组)。
/// v8: src/wasm/wasm-objects.cc:2503-2564 WasmDispatchTable::MaybeGrowUsesList
pub fn dispatchtable_maybegrowuseslist(uses: &mut Vec<Option<UseEntry>>, used: &mut usize) {
  let cap_entries = uses.len();
  if cap_entries == 0 {
    // 初始 3 槽 (1 保留 + 1 对) -> 1 个条目区; used_length = kReservedSlotOffset -> 0 条目
    uses.resize(
      ((USES_INITIAL_LENGTH - USES_RESERVED_SLOT_OFFSET) / USES_ENTRY_STEP) as usize,
      None,
    );
    *used = 0;
    return;
  }
  if *used < cap_entries {
    // used_length < capacity: 有富余, 原样返回
    return;
  }
  // 满: 先压实 (write 游标跳过 cleared)
  let mut cleared = 0usize;
  let mut write = 0usize;
  for i in 0..cap_entries {
    if uses[i].is_none() {
      cleared += 1;
      continue;
    }
    if write != i {
      uses.swap(write, i);
    }
    write += 1;
  }
  let cap_slots =
    (cap_entries * USES_ENTRY_STEP as usize + USES_RESERVED_SLOT_OFFSET as usize) as u32;
  let min_free_entries = dispatchtable_uses_min_free_entries(cap_slots) as usize;
  if cleared >= min_free_entries {
    // 压实已释放足量空位: 原地收拢
    *used = write;
    return;
  }
  // 压实不足: 扩容 (条目数 +50% 且至少 +1), 新数组搬运 [0, write) 活条目
  let new_entries = (cap_entries + 1).max(cap_entries + (cap_entries / 2));
  uses.resize(new_entries, None);
  *used = write;
}

/// 登记一个使用方 (先确保容量, 再于游标处落 (instance, table_index), used+1)
/// 原版 SetEntry(cursor)/SetUsedLength(cursor+2) 槽步 2, 条目步 1
/// v8: src/wasm/wasm-objects.cc:2489-2500 WasmDispatchTable::AddUse
pub fn dispatchtable_adduse(
  uses: &mut Vec<Option<UseEntry>>,
  used: &mut usize,
  instance: u64,
  table_index: i32,
) {
  dispatchtable_maybegrowuseslist(uses, used);
  let cursor = *used;
  uses[cursor] = Some((instance, table_index));
  *used = cursor + 1;
}

/// 调度表增长规划: SBXCHECK_LT(old_length,new_length) 与 SBXCHECK_LE(new_length,
/// max_table_size) 腐蚀/非增长态 fail-closed 返回 None; new_length < old_capacity 时
/// 原地写 length 复用旧表; 否则指数扩容 (dispatchtable_grow_amount), 逐条目搬运
/// (隐式参空 -> 清空哨兵, 否则原样), 尾随区清零。
/// (原版 offheap wrapper move 与 用户列表 set_protected_uses/dispatch_tables/dispatch_table0
/// 更新为堆/Isolate 深面, 推迟 P1 P2)
/// v8: src/wasm/wasm-objects.cc:2567-2670 WasmDispatchTable::Grow
pub fn dispatchtable_grow(
  table: &mut DispatchTableModel,
  new_length: u32,
  max_table_size: u32,
  limit: u32,
) -> Option<DispatchTableGrow> {
  let old_length = table.length;
  // SBXCHECK_LT(old_length, new_length): 仅真正增长才调用, 否则 fail-closed
  if new_length <= old_length {
    return None;
  }
  // SBXCHECK_LE(new_length, max_table_size): 由不可信数据算得, 越上限拒绝
  if new_length > max_table_size {
    return None;
  }
  let old_capacity = table.capacity;
  let Some(grow) = dispatchtable_grow_amount(old_capacity, new_length, limit) else {
    // new_length < old_capacity: 原地写 length 复用旧表 (区内字段已清空)
    table.length = new_length;
    return Some(DispatchTableGrow {
      reused: true,
      new_length,
      new_capacity: old_capacity,
    });
  };
  let new_capacity = old_capacity + grow;
  let has_sig = table.has_sig;
  let mut new_entries = vec![DispatchEntry::EMPTY; new_capacity as usize];
  for i in 0..old_length as usize {
    let src = table.entries[i];
    if src.implicit_arg == EntryWrite::Empty {
      // 原版 implicit_arg == Smi::zero() -> DispatchTableClear(new, i, kNewEntry)
      new_entries[i] = DispatchEntry {
        target: DispatchEntry::INVALID_CODE_POINTER,
        sig: if has_sig { -1 } else { 0 },
        implicit_arg: EntryWrite::Empty,
        wrapper: None,
      };
    } else {
      new_entries[i] = src;
    }
  }
  table.entries = new_entries;
  table.length = new_length;
  table.capacity = new_capacity;
  Some(DispatchTableGrow {
    reused: false,
    new_length,
    new_capacity,
  })
}

/// 增长结果: reused=true 为原地改 length 复用旧表; false 为新容量搬运
/// v8: src/wasm/wasm-objects.cc:2588-2613 WasmDispatchTable::Grow
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DispatchTableGrow {
  /// 是否复用旧表 (未越旧容量)
  pub reused: bool,
  /// 新 length
  pub new_length: u32,
  /// 新 capacity (复用时等旧容量)
  pub new_capacity: u32,
}

/// WasmDispatchTableForImports 方法面 (无 sig 槽; 由 table.has_sig=false 驱动核心跳过 sig 写)
/// v8: src/wasm/wasm-objects.cc:2335-2348 WasmDispatchTableForImports::SetForNonWrapper
pub fn dispatchtableforimports_setfornonwrapper(
  table: &mut DispatchTableModel,
  index: usize,
  implicit_arg: u64,
  call_target: u32,
  sig_id: u32,
  function_index: u32,
  new_or_existing: DispatchNewOrExisting,
) -> Option<()> {
  entry_set(
    table,
    index,
    new_or_existing,
    DispatchSetWrite::NonWrapper {
      implicit_arg,
      call_target,
      function_index,
      sig_id,
    },
  )
}
/// v8: src/wasm/wasm-objects.cc:2411-2425 WasmDispatchTableForImports::SetForWrapper
pub fn dispatchtableforimports_setforwrapper(
  table: &mut DispatchTableModel,
  index: usize,
  p: SetWrapperParams,
) -> Option<()> {
  entry_set(
    table,
    index,
    p.new_or_existing,
    DispatchSetWrite::Wrapper {
      implicit_arg: p.implicit_arg,
      wrapper_handle: p.wrapper_handle,
      code_pointer: p.code_pointer,
      function_index: p.function_index,
      sig_id: p.sig_id,
    },
  )
}
/// v8: src/wasm/wasm-objects.cc:2479-2482 WasmDispatchTableForImports::Clear
pub fn dispatchtableforimports_clear(
  table: &mut DispatchTableModel,
  index: usize,
  new_or_existing: DispatchNewOrExisting,
) -> Option<()> {
  entry_clear(table, index, new_or_existing)
}

/// WasmExceptionPackage 方法面
/// v8: src/wasm/wasm-objects.cc:2694-2698 WasmExceptionPackage::New (按尺寸建值数组重载)
pub fn exceptionpackage_new_with_size() {
  // 纯逻辑面 无: NewFixedArray(size) + 委派 New (工厂 P1)
  todo!("就地修订: WasmExceptionPackage::New (尺寸)")
}
/// v8: src/wasm/wasm-objects.cc:2700-2710 WasmExceptionPackage::New (按值数组重载)
pub fn exceptionpackage_new() {
  // 纯逻辑面 无: NewJSObject(wasm_exception_constructor) + InObjectPropertyPutAtIndex
  // kTagIndex/kValuesIndex (工厂/堆写 P1 P2)
  todo!("就地修订: WasmExceptionPackage::New (值数组)")
}
/// v8: src/wasm/wasm-objects.cc:2713-2722 WasmExceptionPackage::GetExceptionTag
pub fn exceptionpackage_getexceptiontag() {
  // 纯逻辑面 无: JSReceiver::GetProperty(wasm_exception_tag_symbol) (isolate 取属性 P1)
  todo!("就地修订: WasmExceptionPackage::GetExceptionTag")
}
/// v8: src/wasm/wasm-objects.cc:2725-2736 WasmExceptionPackage::GetExceptionValues
pub fn exceptionpackage_getexceptionvalues() {
  // 纯逻辑面 无: JSReceiver::GetProperty(wasm_exception_values_symbol) (isolate 取属性 P1)
  todo!("就地修订: WasmExceptionPackage::GetExceptionValues")
}
/// 按 (module, tag) 求编码尺寸: 原版先 module->canonical_sig_id(tag->sig_index) 经
/// canonicalizer 查规范签名再逐参数求和; 规范签名查找推迟 P5, 纯逻辑面 = 参数类型列
/// 求和 (委派 exceptionpackage_getencodedsize)
/// v8: src/wasm/wasm-objects.cc:2804-2810 WasmExceptionPackage::GetEncodedSize (module+tag)
pub fn exceptionpackage_getencodedsize_for_tag(params: &[ValueKind]) -> u32 {
  exceptionpackage_getencodedsize(params)
}

/// WasmExternalFunction 方法面
/// 外部函数判定 = 导出函数 ∨ capi 函数 (IsWasmExportedFunction ∨ IsWasmCapiFunction)
/// v8: src/wasm/wasm-objects.cc:3124-3127 WasmExternalFunction::IsWasmExternalFunction
pub fn externalfunction_iswasmexternalfunction(is_exported: bool, is_capi: bool) -> bool {
  is_exported || is_capi
}

/// WasmExceptionTag 方法面
/// v8: src/wasm/wasm-objects.cc:3129-3138 WasmExceptionTag::New
pub fn exceptiontag_new() {
  // 纯逻辑面 无: NewObject<Map>(prototype, ...) + package/name 字段写 (工厂/堆写 P1 P2)
  todo!("就地修订: WasmExceptionTag::New")
}
