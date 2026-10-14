// 由 tool/src/objects/cell.h.js 从 v8/src/objects/cell.{h,-inl.h,tq} 生成; 修订就地改, 复跑覆盖
// Cell 单值槽对象骨架

/// 单值槽 (原版 V8_OBJECT class Cell : public HeapObject)
/// 布局 (tq): map + maybe_value (MaybeObject, 可为弱引用/Smi 清空值)
pub struct Cell;

impl Cell {
  /// 唯一字段偏移提示: maybe_value 紧随 map (TaggedMember<MaybeObject>)
  /// 数值常量由布局求解器统一定版, 此处不臆测
  pub const FIELD: &str = "maybe_value";
}

// —— 签名骨架 (原版公开接口, 供 Developer 补语义) ——
// explicit Cell(const AllocationWitness&);
// Cell(const AllocationWitness&, Tagged<Smi>);
// Cell(const AllocationWitness&, Tagged<ClearedWeakValue>);
// Tagged<MaybeObject> maybe_value() const; (+RelaxedLoadTag 重载)
// void set_maybe_value(Tagged<MaybeObject>, WriteBarrierMode);
// Tagged<Object> value() const; (+RelaxedLoadTag 重载)
// void set_value(Tagged<Object>, WriteBarrierMode);
// DECL_PRINTER(Cell) / DECL_VERIFIER(Cell)
pub const CELL_SIGNATURES: [&str; 8] = [
  "ctor_witness",
  "ctor_smi",
  "ctor_cleared_weak",
  "maybe_value",
  "set_maybe_value",
  "value",
  "set_value",
  "verify_print",
];

// —— 提取不变式断言 ——
const _: () = {
  assert!(CELL_SIGNATURES.len() == 8);
  assert!(Cell::FIELD == "maybe_value");
};
