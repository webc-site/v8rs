// 由 tool/src/builtins/builtins-typedarray.js 生成; 来源 v8/src/builtins 下 9 文件
// 内建名表 + 核心调度元数据 (23 内建)
// 来源: builtins-arraybuffer.cc, builtins-dataview.cc, builtins-sharedarraybuffer.cc, data-view-ops.h 等

use strum::{Display, EnumIter, EnumString};
use v8_builtins_core::{descriptor_name, find_builtin};

/// Buffer / DataView / Atomics 内建名表 (23 项)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Display, EnumIter, EnumString)]
pub enum Builtin {
  ArrayBufferConstructor,
  #[strum(serialize = "ArrayBufferConstructor_DoNotInitialize")]
  ArrayBufferConstructorDoNotInitialize,
  ArrayBufferPrototypeResize,
  ArrayBufferPrototypeSlice,
  ArrayBufferPrototypeSliceToImmutable,
  ArrayBufferPrototypeTransfer,
  ArrayBufferPrototypeTransferToFixedLength,
  ArrayBufferPrototypeTransferToImmutable,
  AtomicsCompareExchange,
  AtomicsExchange,
  AtomicsIsLockFree,
  AtomicsLoad,
  AtomicsNotify,
  AtomicsPause,
  AtomicsStore,
  AtomicsWait,
  AtomicsWaitAsync,
  DataViewConstructor,
  SharedArrayBufferPrototypeGetByteLength,
  SharedArrayBufferPrototypeGrow,
  SharedArrayBufferPrototypeSlice,
  SharedArrayConstructor,
  SharedArrayIsSharedArray,
}

impl Builtin {
  /// 获取内建的 V8 原版规范名称
  #[inline]
  pub const fn name(&self) -> &'static str {
    match self {
      Self::ArrayBufferConstructor => "ArrayBufferConstructor",
      Self::ArrayBufferConstructorDoNotInitialize => "ArrayBufferConstructor_DoNotInitialize",
      Self::ArrayBufferPrototypeResize => "ArrayBufferPrototypeResize",
      Self::ArrayBufferPrototypeSlice => "ArrayBufferPrototypeSlice",
      Self::ArrayBufferPrototypeSliceToImmutable => "ArrayBufferPrototypeSliceToImmutable",
      Self::ArrayBufferPrototypeTransfer => "ArrayBufferPrototypeTransfer",
      Self::ArrayBufferPrototypeTransferToFixedLength => {
        "ArrayBufferPrototypeTransferToFixedLength"
      }
      Self::ArrayBufferPrototypeTransferToImmutable => "ArrayBufferPrototypeTransferToImmutable",
      Self::AtomicsCompareExchange => "AtomicsCompareExchange",
      Self::AtomicsExchange => "AtomicsExchange",
      Self::AtomicsIsLockFree => "AtomicsIsLockFree",
      Self::AtomicsLoad => "AtomicsLoad",
      Self::AtomicsNotify => "AtomicsNotify",
      Self::AtomicsPause => "AtomicsPause",
      Self::AtomicsStore => "AtomicsStore",
      Self::AtomicsWait => "AtomicsWait",
      Self::AtomicsWaitAsync => "AtomicsWaitAsync",
      Self::DataViewConstructor => "DataViewConstructor",
      Self::SharedArrayBufferPrototypeGetByteLength => "SharedArrayBufferPrototypeGetByteLength",
      Self::SharedArrayBufferPrototypeGrow => "SharedArrayBufferPrototypeGrow",
      Self::SharedArrayBufferPrototypeSlice => "SharedArrayBufferPrototypeSlice",
      Self::SharedArrayConstructor => "SharedArrayConstructor",
      Self::SharedArrayIsSharedArray => "SharedArrayIsSharedArray",
    }
  }

  /// 关联至全局注册表 `BUILTIN_TABLE` 获取全局内建 ID
  #[inline]
  pub fn global_id(&self) -> Option<usize> {
    find_builtin(self.name())
  }

  /// 获取对应的接口描述符符号
  #[inline]
  pub fn descriptor_name(&self) -> String {
    descriptor_name(self.name())
  }

  /// 是否为 ArrayBuffer 核心或方法
  #[inline]
  pub const fn is_array_buffer(&self) -> bool {
    matches!(
      self,
      Self::ArrayBufferConstructor
        | Self::ArrayBufferConstructorDoNotInitialize
        | Self::ArrayBufferPrototypeResize
        | Self::ArrayBufferPrototypeSlice
        | Self::ArrayBufferPrototypeSliceToImmutable
        | Self::ArrayBufferPrototypeTransfer
        | Self::ArrayBufferPrototypeTransferToFixedLength
        | Self::ArrayBufferPrototypeTransferToImmutable
    )
  }

  /// 是否为 Atomics 核心操作
  #[inline]
  pub const fn is_atomics(&self) -> bool {
    matches!(
      self,
      Self::AtomicsCompareExchange
        | Self::AtomicsExchange
        | Self::AtomicsIsLockFree
        | Self::AtomicsLoad
        | Self::AtomicsNotify
        | Self::AtomicsPause
        | Self::AtomicsStore
        | Self::AtomicsWait
        | Self::AtomicsWaitAsync
    )
  }

  /// 是否为 SharedArrayBuffer 核心操作
  #[inline]
  pub const fn is_shared_array_buffer(&self) -> bool {
    matches!(
      self,
      Self::SharedArrayBufferPrototypeGetByteLength
        | Self::SharedArrayBufferPrototypeGrow
        | Self::SharedArrayBufferPrototypeSlice
        | Self::SharedArrayConstructor
        | Self::SharedArrayIsSharedArray
    )
  }

  /// 是否为 DataView 构造函数
  #[inline]
  pub const fn is_dataview(&self) -> bool {
    matches!(self, Self::DataViewConstructor)
  }
}

/// DataView 操作类型枚举 (忠实对应 v8/src/builtins/data-view-ops.h: DataViewOp)
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Display, EnumIter, EnumString)]
pub enum DataViewOp {
  GetBigInt64,
  SetBigInt64,
  GetBigUint64,
  SetBigUint64,
  GetFloat32,
  SetFloat32,
  GetFloat64,
  SetFloat64,
  GetInt8,
  SetInt8,
  GetInt16,
  SetInt16,
  GetInt32,
  SetInt32,
  GetUint8,
  SetUint8,
  GetUint16,
  SetUint16,
  GetUint32,
  SetUint32,
  ByteLength,
}

impl DataViewOp {
  /// 原版规范字符串表示 (对应 data-view-ops.h: ToString(DataViewOp op))
  #[inline]
  pub const fn as_str(&self) -> &'static str {
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

  /// 是否为读取操作 (Getter)
  #[inline]
  pub const fn is_getter(&self) -> bool {
    matches!(
      self,
      Self::GetBigInt64
        | Self::GetBigUint64
        | Self::GetFloat32
        | Self::GetFloat64
        | Self::GetInt8
        | Self::GetInt16
        | Self::GetInt32
        | Self::GetUint8
        | Self::GetUint16
        | Self::GetUint32
        | Self::ByteLength
    )
  }

  /// 是否为写入操作 (Setter)
  #[inline]
  pub const fn is_setter(&self) -> bool {
    matches!(
      self,
      Self::SetBigInt64
        | Self::SetBigUint64
        | Self::SetFloat32
        | Self::SetFloat64
        | Self::SetInt8
        | Self::SetInt16
        | Self::SetInt32
        | Self::SetUint8
        | Self::SetUint16
        | Self::SetUint32
    )
  }

  /// 对应操作的数值元素字节宽度
  #[inline]
  pub const fn element_size(&self) -> usize {
    match self {
      Self::GetInt8 | Self::SetInt8 | Self::GetUint8 | Self::SetUint8 => 1,
      Self::GetInt16 | Self::SetInt16 | Self::GetUint16 | Self::SetUint16 => 2,
      Self::GetInt32
      | Self::SetInt32
      | Self::GetUint32
      | Self::SetUint32
      | Self::GetFloat32
      | Self::SetFloat32 => 4,
      Self::GetBigInt64
      | Self::SetBigInt64
      | Self::GetBigUint64
      | Self::SetBigUint64
      | Self::GetFloat64
      | Self::SetFloat64 => 8,
      Self::ByteLength => 0,
    }
  }
}

// 统一提供 23 项内建构造入口工厂函数
pub const fn array_buffer_constructor() -> Builtin {
  Builtin::ArrayBufferConstructor
}

pub const fn array_buffer_constructor_do_not_initialize() -> Builtin {
  Builtin::ArrayBufferConstructorDoNotInitialize
}

pub const fn array_buffer_prototype_resize() -> Builtin {
  Builtin::ArrayBufferPrototypeResize
}

pub const fn array_buffer_prototype_slice() -> Builtin {
  Builtin::ArrayBufferPrototypeSlice
}

pub const fn array_buffer_prototype_slice_to_immutable() -> Builtin {
  Builtin::ArrayBufferPrototypeSliceToImmutable
}

pub const fn array_buffer_prototype_transfer() -> Builtin {
  Builtin::ArrayBufferPrototypeTransfer
}

pub const fn array_buffer_prototype_transfer_to_fixed_length() -> Builtin {
  Builtin::ArrayBufferPrototypeTransferToFixedLength
}

pub const fn array_buffer_prototype_transfer_to_immutable() -> Builtin {
  Builtin::ArrayBufferPrototypeTransferToImmutable
}

pub const fn atomics_compare_exchange() -> Builtin {
  Builtin::AtomicsCompareExchange
}

pub const fn atomics_exchange() -> Builtin {
  Builtin::AtomicsExchange
}

pub const fn atomics_is_lock_free() -> Builtin {
  Builtin::AtomicsIsLockFree
}

pub const fn atomics_load() -> Builtin {
  Builtin::AtomicsLoad
}

pub const fn atomics_notify() -> Builtin {
  Builtin::AtomicsNotify
}

pub const fn atomics_pause() -> Builtin {
  Builtin::AtomicsPause
}

pub const fn atomics_store() -> Builtin {
  Builtin::AtomicsStore
}

pub const fn atomics_wait() -> Builtin {
  Builtin::AtomicsWait
}

pub const fn atomics_wait_async() -> Builtin {
  Builtin::AtomicsWaitAsync
}

pub const fn data_view_constructor() -> Builtin {
  Builtin::DataViewConstructor
}

pub const fn shared_array_buffer_prototype_get_byte_length() -> Builtin {
  Builtin::SharedArrayBufferPrototypeGetByteLength
}

pub const fn shared_array_buffer_prototype_grow() -> Builtin {
  Builtin::SharedArrayBufferPrototypeGrow
}

pub const fn shared_array_buffer_prototype_slice() -> Builtin {
  Builtin::SharedArrayBufferPrototypeSlice
}

pub const fn shared_array_constructor() -> Builtin {
  Builtin::SharedArrayConstructor
}

pub const fn shared_array_is_shared_array() -> Builtin {
  Builtin::SharedArrayIsSharedArray
}

/// 来自 `arraybuffer.tq` / `data-view.tq` 的声明清单 (共 36 条)
pub const TQ_DECLS: &[(&str, &str)] = &[
  ("extern macro", "ArrayBufferWasmMemorySymbol"),
  ("macro", "MakeDataViewGetterNameString"),
  ("macro", "MakeDataViewSetterNameString"),
  ("macro", "WasDetached"),
  ("macro", "ValidateDataView"),
  ("javascript builtin", "DataViewPrototypeGetBuffer"),
  ("javascript builtin", "DataViewPrototypeGetByteLength"),
  ("javascript builtin", "DataViewPrototypeGetByteOffset"),
  ("extern macro", "BitcastInt32ToFloat32"),
  ("extern macro", "BitcastFloat32ToInt32"),
  ("extern macro", "Float64ExtractLowWord32"),
  ("extern macro", "Float64ExtractHighWord32"),
  ("extern macro", "Float64InsertLowWord32"),
  ("extern macro", "Float64InsertHighWord32"),
  ("extern macro", "DataViewBuiltinsAssembler"),
  ("extern macro", "DataViewBuiltinsAssembler"),
  ("macro", "LoadDataView8"),
  ("macro", "LoadDataView16"),
  ("macro", "LoadDataView32"),
  ("macro", "LoadDataViewFloat16"),
  ("macro", "LoadDataViewFloat64"),
  ("macro", "MakeBigIntOn64Bit"),
  ("macro", "MakeBigIntOn32Bit"),
  ("macro", "MakeBigInt"),
  ("macro", "LoadDataViewBigInt"),
  ("extern macro", "DataViewBuiltinsAssembler"),
  ("extern macro", "ToNumber"),
  ("extern macro", "TruncateFloat64ToWord32"),
  ("extern macro", "DataViewBuiltinsAssembler"),
  ("macro", "StoreDataView8"),
  ("macro", "StoreDataView16"),
  ("macro", "StoreDataView32"),
  ("macro", "StoreDataView64"),
  ("extern macro", "DataViewBuiltinsAssembler"),
  ("extern macro", "DataViewBuiltinsAssembler"),
  ("macro", "StoreDataViewBigInt"),
];
