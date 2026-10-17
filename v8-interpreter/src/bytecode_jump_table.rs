// 由 tool/src/interpreter/bytecode-jump-table.h.js 从 v8/src/interpreter/bytecode-jump-table.h 生成; 修订就地改, 复跑覆盖
// 字节码 switch 跳转表骨架

/// 无效索引哨兵 (原版 kInvalidIndex)
pub const INVALID_INDEX: usize = usize::MAX;
/// 无效偏移哨兵 (原版 kInvalidOffset)
pub const INVALID_OFFSET: usize = usize::MAX;

/// 原版 class BytecodeJumpTable 成员声明 (switch 目标表; 表项绑定即字节码位置, 全不匹配则落穿)
pub const BYTECODE_JUMP_TABLE_DECLARATIONS: &[&str] = &[
  "[public/method] BytecodeJumpTable(size_t constant_pool_index, int size, int case_value_base, Zone* zone) : bound_(size, zone), constant_pool_index_(constant_pool_index), size_(size), case_value_base_(case_value_base)",
  "[public/method] size_t constant_pool_index() const",
  "[public/method] size_t switch_bytecode_offset() const",
  "[public/method] OperandScale switch_bytecode_operand_scale() const",
  "[public/method] int case_value_base() const",
  "[public/method] int size() const",
  "[public/method] bool is_bound(int case_value) const",
  "[public/method] size_t ConstantPoolEntryFor(int case_value)",
  "[public/method] void set_size(int size)",
  "[private/const] static const size_t kInvalidIndex = static_cast<size_t>(-1)",
  "[private/const] static const size_t kInvalidOffset = static_cast<size_t>(-1)",
  "[private/method] void mark_bound(int case_value)",
  "[private/method] void set_switch_bytecode_offset(size_t offset, OperandScale scale)",
  "[private/field] BitVector bound_",
  "[private/field] size_t constant_pool_index_",
  "[private/field] size_t switch_bytecode_offset_ = kInvalidOffset",
  "[private/field] OperandScale switch_bytecode_operand_scale_ = OperandScale::kSingle",
  "[private/field] int size_",
  "[private/field] int case_value_base_",
];

/// switch 表项常量池索引 = 基址 + (case 值 - case 基值) (原版 ConstantPoolEntryFor 公式)
#[inline]
#[must_use]
pub const fn constant_pool_entry_for(
  constant_pool_index: usize,
  case_value_base: i32,
  case_value: i32,
) -> usize {
  constant_pool_index + (case_value - case_value_base) as usize
}

// —— 提取不变式断言 ——
const _: () = {
  assert!(BYTECODE_JUMP_TABLE_DECLARATIONS.len() == 19);
  assert!(INVALID_INDEX == usize::MAX);
  assert!(constant_pool_entry_for(100, 3, 5) == 102);
};
