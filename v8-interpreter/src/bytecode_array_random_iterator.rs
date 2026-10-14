// 由 tool/src/interpreter/bytecode-array-random-iterator.h.js 从 v8/src/interpreter/bytecode-array-random-iterator.{h,cc} 生成; 修订就地改, 复跑覆盖

//! 随机访问迭代器: 偏移缓存表 + 当前索引 (原版字段 offsets_/current_index_ 的机械展开)

/// 原版 class/struct BytecodeArrayRandomIterator 成员声明 (机械 1:1 提取, 17 条)
pub const BYTECODE_ARRAY_RANDOM_ITERATOR_DECLARATIONS: &[&str] = &[
  "[public/method] BytecodeArrayRandomIterator(Handle<BytecodeArray> bytecode_array, Zone* zone)",
  "[public/method] BytecodeArrayRandomIterator(const BytecodeArrayRandomIterator&) = delete",
  "[public/method] BytecodeArrayRandomIterator& operator=(const BytecodeArrayRandomIterator&) = delete",
  "[public/method] BytecodeArrayRandomIterator& operator++()",
  "[public/method] BytecodeArrayRandomIterator& operator--()",
  "[public/method] BytecodeArrayRandomIterator& operator+=(int offset)",
  "[public/method] BytecodeArrayRandomIterator& operator-=(int offset)",
  "[public/method] int current_index() const",
  "[public/method] int size() const",
  "[public/method] void GoToIndex(int index)",
  "[public/method] void GoToStart()",
  "[public/method] void GoToEnd()",
  "[public/method] bool IsValid() const",
  "[private/field] ZoneVector<int> offsets_",
  "[private/field] int current_index_",
  "[private/method] void Initialize()",
  "[private/method] void UpdateOffsetFromIndex()",
];

// —— 提取不变式断言 ——
const _: () = {
  assert!(BYTECODE_ARRAY_RANDOM_ITERATOR_DECLARATIONS.len() == 17);
};
