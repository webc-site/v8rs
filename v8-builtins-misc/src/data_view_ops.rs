// 由 tool/src/builtins/data-view-ops.js 从 v8/src/builtins/data-view-ops.h 生成; 修订就地改, 复跑覆盖
// DATAVIEW_OP_LIST X-macro 机械展开 — 枚举变体带顺序值 + ToString 映射
//! 任务映射: v8-builtins-misc; 聚合基线另归 v8-builtins-typedarray (并存不覆盖)

/// DataView 已知导入操作名 — X-macro DATAVIEW_OP_LIST(V) 展开
pub const DATAVIEW_OP_LIST: &[&str] = &[
  "BigInt64",
  "BigUint64",
  "Float32",
  "Float64",
  "Int8",
  "Int16",
  "Int32",
  "Uint8",
  "Uint16",
  "Uint32",
];

/// 原版 enum DataViewOp : uint8_t — X-macro 机械展开 (变体顺序即判别值, 共 21 个)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum DataViewOp {
  /// kGetBigInt64 (DataView.prototype.getBigInt64)
  GetBigInt64 = 0,
  /// kSetBigInt64 (DataView.prototype.setBigInt64)
  SetBigInt64 = 1,
  /// kGetBigUint64 (DataView.prototype.getBigUint64)
  GetBigUint64 = 2,
  /// kSetBigUint64 (DataView.prototype.setBigUint64)
  SetBigUint64 = 3,
  /// kGetFloat32 (DataView.prototype.getFloat32)
  GetFloat32 = 4,
  /// kSetFloat32 (DataView.prototype.setFloat32)
  SetFloat32 = 5,
  /// kGetFloat64 (DataView.prototype.getFloat64)
  GetFloat64 = 6,
  /// kSetFloat64 (DataView.prototype.setFloat64)
  SetFloat64 = 7,
  /// kGetInt8 (DataView.prototype.getInt8)
  GetInt8 = 8,
  /// kSetInt8 (DataView.prototype.setInt8)
  SetInt8 = 9,
  /// kGetInt16 (DataView.prototype.getInt16)
  GetInt16 = 10,
  /// kSetInt16 (DataView.prototype.setInt16)
  SetInt16 = 11,
  /// kGetInt32 (DataView.prototype.getInt32)
  GetInt32 = 12,
  /// kSetInt32 (DataView.prototype.setInt32)
  SetInt32 = 13,
  /// kGetUint8 (DataView.prototype.getUint8)
  GetUint8 = 14,
  /// kSetUint8 (DataView.prototype.setUint8)
  SetUint8 = 15,
  /// kGetUint16 (DataView.prototype.getUint16)
  GetUint16 = 16,
  /// kSetUint16 (DataView.prototype.setUint16)
  SetUint16 = 17,
  /// kGetUint32 (DataView.prototype.getUint32)
  GetUint32 = 18,
  /// kSetUint32 (DataView.prototype.setUint32)
  SetUint32 = 19,
  /// kByteLength (get DataView.prototype.byteLength)
  ByteLength = 20,
}

impl DataViewOp {
  /// 变体总数
  pub const COUNT: usize = 21;

  /// 原版 ToString(DataViewOp) — 机械展开 switch 表
  #[inline]
  #[must_use]
  pub const fn to_str(self) -> &'static str {
    match self {
      Self::GetBigInt64 => "DataView.prototype.getBigInt64",
      Self::SetBigInt64 => "DataView.prototype.setBigInt64",
      Self::GetBigUint64 => "DataView.prototype.getBigUint64",
      Self::SetBigUint64 => "DataView.prototype.setBigUint64",
      Self::GetFloat32 => "DataView.prototype.getFloat32",
      Self::SetFloat32 => "DataView.prototype.setFloat32",
      Self::GetFloat64 => "DataView.prototype.getFloat64",
      Self::SetFloat64 => "DataView.prototype.setFloat64",
      Self::GetInt8 => "DataView.prototype.getInt8",
      Self::SetInt8 => "DataView.prototype.setInt8",
      Self::GetInt16 => "DataView.prototype.getInt16",
      Self::SetInt16 => "DataView.prototype.setInt16",
      Self::GetInt32 => "DataView.prototype.getInt32",
      Self::SetInt32 => "DataView.prototype.setInt32",
      Self::GetUint8 => "DataView.prototype.getUint8",
      Self::SetUint8 => "DataView.prototype.setUint8",
      Self::GetUint16 => "DataView.prototype.getUint16",
      Self::SetUint16 => "DataView.prototype.setUint16",
      Self::GetUint32 => "DataView.prototype.getUint32",
      Self::SetUint32 => "DataView.prototype.setUint32",
      Self::ByteLength => "get DataView.prototype.byteLength",
    }
  }
}

/// 操作数 (不含 kByteLength)
pub const OP_COUNT: usize = 10;

// —— 对数断言 ——
const _: () = {
  assert!(DATAVIEW_OP_LIST.len() == 10);
  assert!(DataViewOp::COUNT == 21);
  assert!(OP_COUNT == 10);
  assert!(DataViewOp::GetBigInt64 as u8 == 0);
  assert!(DataViewOp::ByteLength as u8 == 20);
  assert!(DataViewOp::SetUint32.to_str() == "DataView.prototype.setUint32");
};
