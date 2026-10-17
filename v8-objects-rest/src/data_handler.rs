// 由 tool/src/objects/data-handler.h.js 从 v8/src/objects/data-handler.{h,-inl.h,tq} 生成; 修订就地改, 复跑覆盖
// DataHandler 抽象基: 弹性域公式与字段面

/// 数据处理器抽象基 (原版 V8_OBJECT class DataHandler : Struct, V8_IT_ABSTRACT)
/// LoadHandler/StoreHandler 的不可 Smi 编码形态基类
pub struct DataHandler;

impl DataHandler {
  /// 弹性域字节宽 (原版 sizeof(DataHandler::FlexibleDataType) = MaybeObject 槽宽)
  /// 数值由布局求解器定版, 公式按参数直译
  #[inline]
  #[must_use]
  pub const fn offset_of(data_start: usize, flexible_size: usize, index: usize) -> usize {
    data_start + index * flexible_size
  }

  /// 原版 SizeFor
  #[inline]
  #[must_use]
  pub const fn size_for(data_start: usize, flexible_size: usize, count: usize) -> usize {
    data_start + count * flexible_size
  }
}

/// tq 布局字段序 (原版 extern class DataHandler)
pub const TQ_FIELDS: [&str; 2] = [
  "smi_handler",
  "validity_cell",
];

/// 可选数据域容量 (原版 data1..data5)
pub const MAX_DATA_FIELD_COUNT: usize = 5;

// —— 签名骨架 ——
// smi_handler / set_smi_handler(Smi|Code) / validity_cell / set_validity_cell
// data_field_count / data1..data5 + set_data1..5 (MaybeObject, 写屏障)
pub const SIGNATURES: [&str; 5] = ["smi_handler", "validity_cell", "data_field_count", "data_slots", "verify"];

// —— 提取不变式断言 ——
const _: () = {
  assert!(MAX_DATA_FIELD_COUNT == 5);
  assert!(DataHandler::size_for(8, 4, 5) == 28);
  assert!(DataHandler::offset_of(8, 4, 3) == 20);
  assert!(TQ_FIELDS.len() == 2);
  assert!(TQ_FIELDS[0] == "smi_handler");
};
