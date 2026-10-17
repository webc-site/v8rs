// 由 tool/src/interpreter/constant-array-builder.h.js 从 v8/src/interpreter/constant-array-builder.{h,cc} 生成; 修订就地改, 复跑覆盖

/// 8 位操作数分片容量 (原版 k8BitCapacity = 1 << 8)
pub const K8BIT_CAPACITY: usize = 256;
/// 16 位操作数分片容量 (原版 k16BitCapacity = (1 << 16) - 256)
pub const K16BIT_CAPACITY: usize = 65280;
/// 32 位操作数分片容量 (原版 k32BitCapacity = kMaxUInt32 - k16 - k8 + 1)
pub const K32BIT_CAPACITY: usize = 4294901760;

/// 单例常量条目类型 (原版 SINGLETON_CONSTANT_ENTRY_TYPES 展开, 8 项)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum SingletonConstantEntryType {
  /// 运行时根: async_iterator_symbol
  AsyncIteratorSymbol,
  /// 运行时根: class_fields_symbol
  ClassFieldsSymbol,
  /// 运行时根: empty_object_boilerplate_description
  EmptyObjectBoilerplateDescription,
  /// 运行时根: empty_array_boilerplate_description
  EmptyArrayBoilerplateDescription,
  /// 运行时根: empty_fixed_array
  EmptyFixedArray,
  /// 运行时根: iterator_symbol
  IteratorSymbol,
  /// 运行时根: interpreter_trampoline_symbol
  InterpreterTrampolineSymbol,
  /// 运行时根: nan_value
  NaN,
}

/// 条目类别 (原版 Entry::Tag, 10 显式 + 8 单例, 顺序敏感)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum EntryTag {
  /// 原版 kDeferred
  Deferred = 0,
  /// 原版 kHandle
  Handle = 1,
  /// 原版 kSmi
  Smi = 2,
  /// 原版 kRawString
  RawString = 3,
  /// 原版 kConsString
  ConsString = 4,
  /// 原版 kHeapNumber
  HeapNumber = 5,
  /// 原版 kBigInt
  BigInt = 6,
  /// 原版 kScope
  Scope = 7,
  /// 原版 kUninitializedJumpTableSmi
  UninitializedJumpTableSmi = 8,
  /// 原版 kJumpTableSmi
  JumpTableSmi = 9,
  /// 原版 kAsyncIteratorSymbol
  AsyncIteratorSymbol = 10,
  /// 原版 kClassFieldsSymbol
  ClassFieldsSymbol = 11,
  /// 原版 kEmptyObjectBoilerplateDescription
  EmptyObjectBoilerplateDescription = 12,
  /// 原版 kEmptyArrayBoilerplateDescription
  EmptyArrayBoilerplateDescription = 13,
  /// 原版 kEmptyFixedArray
  EmptyFixedArray = 14,
  /// 原版 kIteratorSymbol
  IteratorSymbol = 15,
  /// 原版 kInterpreterTrampolineSymbol
  InterpreterTrampolineSymbol = 16,
  /// 原版 kNaN
  NaN = 17,
}

/// 原版 class/struct ConstantArrayBuilder 成员声明 (机械 1:1 提取, 32 条)
pub const CONSTANT_ARRAY_BUILDER_DECLARATIONS: &[&str] = &[
  "[public/const] static const size_t k8BitCapacity = 1u << kBitsPerByte",
  "[public/const] static const size_t k16BitCapacity = (1u << 2 * kBitsPerByte) - k8BitCapacity",
  "[public/const] static const size_t k32BitCapacity = kMaxUInt32 - k16BitCapacity - k8BitCapacity + 1",
  "[public/method] explicit ConstantArrayBuilder(Zone* zone)",
  "[public/method] size_t size() const",
  "[public/method] size_t Insert(Tagged<Smi> smi)",
  "[public/method] size_t Insert(double number)",
  "[public/method] size_t Insert(const AstRawString* raw_string)",
  "[public/method] size_t Insert(const AstConsString* cons_string)",
  "[public/method] size_t Insert(AstBigInt bigint)",
  "[public/method] size_t Insert(const Scope* scope)",
  "[public/method] SINGLETON_CONSTANT_ENTRY_TYPES(INSERT_ENTRY) size_t InsertDeferred()",
  "[public/method] size_t InsertJumpTable(size_t size)",
  "[public/method] void SetDeferredAt(size_t index, Handle<Object> object)",
  "[public/method] void SetJumpTableSmi(size_t index, Tagged<Smi> smi)",
  "[public/method] OperandSize CreateReservedEntry( OperandSize minimum_operand_size = OperandSize::kNone)",
  "[public/method] size_t CommitReservedEntry(OperandSize operand_size, Tagged<Smi> value)",
  "[public/method] void DiscardReservedEntry(OperandSize operand_size)",
  "[private/field] struct ConstantArraySlice",
  "[private/field] class Entry",
  "[private/method] index_t AllocateIndex(Entry constant_entry)",
  "[private/method] index_t AllocateIndexArray(Entry constant_entry, size_t size)",
  "[private/method] index_t AllocateReservedEntry(Tagged<Smi> value)",
  "[private/field] struct ConstantArraySlice final : public ZoneObject",
  "[private/method] ConstantArraySlice* IndexToSlice(size_t index) const",
  "[private/method] ConstantArraySlice* OperandSizeToSlice(OperandSize operand_size) const",
  "[private/field] ConstantArraySlice* idx_slice_[3]",
  "[private/field] base::TemplateHashMapImpl<intptr_t, index_t, base::KeyEqualityMatcher<intptr_t>, ZoneAllocationPolicy> constants_map_",
  "[private/field] ZoneMap<Tagged<Smi>, index_t> smi_map_",
  "[private/field] ZoneVector<std::pair<Tagged<Smi>, index_t>> smi_pairs_",
  "[private/field] ZoneMap<double, index_t> heap_number_map_",
  "[private/method] SINGLETON_CONSTANT_ENTRY_TYPES(SINGLETON_ENTRY_FIELD)",
];

/// 原版 class/struct Entry 成员声明 (机械 1:1 提取, 16 条)
pub const CONSTANT_ARRAY_ENTRY_DECLARATIONS: &[&str] = &[
  "[public/method] explicit Entry(Tagged<Smi> smi) : smi_(smi), tag_(Tag::kSmi)",
  "[public/method] explicit Entry(double heap_number) : heap_number_(heap_number), tag_(Tag::kHeapNumber)",
  "[public/method] explicit Entry(const AstRawString* raw_string) : raw_string_(raw_string), tag_(Tag::kRawString)",
  "[public/method] explicit Entry(const AstConsString* cons_string) : cons_string_(cons_string), tag_(Tag::kConsString)",
  "[public/method] explicit Entry(AstBigInt bigint) : bigint_(bigint), tag_(Tag::kBigInt)",
  "[public/method] explicit Entry(const Scope* scope) : scope_(scope), tag_(Tag::kScope)",
  "[public/method] static Entry NAME()",
  "[public/method] SINGLETON_CONSTANT_ENTRY_TYPES(CONSTRUCT_ENTRY) static Entry Deferred()",
  "[public/method] static Entry UninitializedJumpTableSmi()",
  "[public/method] bool IsDeferred() const",
  "[public/method] bool IsJumpTableEntry() const",
  "[public/method] void SetDeferred(Handle<Object> handle)",
  "[public/method] void SetJumpTableSmi(Tagged<Smi> smi)",
  "[private/method] explicit Entry(Tag tag) : tag_(tag)",
  "[private/field] union",
  "[private/field] tag_",
];

// —— 提取不变式断言 ——
const _: () = {
  assert!(CONSTANT_ARRAY_BUILDER_DECLARATIONS.len() == 32);
  assert!(CONSTANT_ARRAY_ENTRY_DECLARATIONS.len() == 16);
  assert!(K8BIT_CAPACITY == 256);
  assert!(K16BIT_CAPACITY == 65280);
  assert!(K32BIT_CAPACITY == 4294901760);
};
