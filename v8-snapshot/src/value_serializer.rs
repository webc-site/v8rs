// 由 tool/src/objects/value-serializer.h.js 从 v8/src/objects/value-serializer.{h,cc} 生成; 修订就地改, 复跑覆盖
// 结构化克隆线上格式 SerializationTag 全量表
//! SerializationTag: 结构化序列化 (structured clone) 线上格式标签。
//!
//! 数值即 ASCII 字节 (kVersion=0xFF 除外)。语义实现面 (序列化器/反序列化器,
//! 约 3000 行) 不在本产物范围, 由 Developer 就地实现。

/// v8: src/objects/value-serializer.cc SerializationTag
/// 原版 enum class SerializationTag : uint8_t, 共 59 标签。
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum SerializationTag {
  /// kVersion — version:uint32_t (if at beginning of data, sets version > 0)
  Version = 255,
  /// kPadding — ignore
  Padding = 0,
  /// kVerifyObjectCount '?' — refTableSize:uint32_t (previously used for sanity checks; safe to ignore)
  VerifyObjectCount = 63,
  /// kTheHole '-' — Oddballs (no data).
  TheHole = 45,
  /// kUndefined '_'
  Undefined = 95,
  /// kNull '0'
  Null = 48,
  /// kTrue 'T'
  True = 84,
  /// kFalse 'F'
  False = 70,
  /// kInt32 'I' — Number represented as 32-bit integer, ZigZag-encoded (like sint32 in protobuf)
  Int32 = 73,
  /// kUint32 'U' — Number represented as 32-bit unsigned integer, varint-encoded (like uint32 in protobuf)
  Uint32 = 85,
  /// kDouble 'N' — Number represented as a 64-bit double. Host byte order is used (N.B. this makes the format non-portable).
  Double = 78,
  /// kBigInt 'Z' — BigInt. Bitfield:uint32_t, then raw digits storage.
  BigInt = 90,
  /// kUtf8String 'S' — byteLength:uint32_t, then raw data
  Utf8String = 83,
  /// kOneByteString '"'
  OneByteString = 34,
  /// kTwoByteString 'c'
  TwoByteString = 99,
  /// kObjectReference '^' — Reference to a serialized object. objectID:uint32_t
  ObjectReference = 94,
  /// kBeginJSObject 'o' — Beginning of a JS object.
  BeginJSObject = 111,
  /// kEndJSObject '{' — End of a JS object. numProperties:uint32_t
  EndJSObject = 123,
  /// kBeginSparseJSArray 'a' — Beginning of a sparse JS array. length:uint32_t Elements and properties are written as key/value pairs, like objects.
  BeginSparseJSArray = 97,
  /// kEndSparseJSArray '@' — End of a sparse JS array. numProperties:uint32_t length:uint32_t
  EndSparseJSArray = 64,
  /// kBeginDenseJSArray 'A' — Beginning of a dense JS array. length:uint32_t |length| elements, followed by properties as key/value pairs
  BeginDenseJSArray = 65,
  /// kEndDenseJSArray '$' — End of a dense JS array. numProperties:uint32_t length:uint32_t
  EndDenseJSArray = 36,
  /// kDate 'D' — Date. millisSinceEpoch:double
  Date = 68,
  /// kTrueObject 'y' — Boolean object. No data.
  TrueObject = 121,
  /// kFalseObject 'x'
  FalseObject = 120,
  /// kNumberObject 'n' — Number object. value:double
  NumberObject = 110,
  /// kBigIntObject 'z' — BigInt object. Bitfield:uint32_t, then raw digits storage.
  BigIntObject = 122,
  /// kStringObject 's' — String object, UTF-8 encoding. byteLength:uint32_t, then raw data.
  StringObject = 115,
  /// kRegExp 'R' — Regular expression, UTF-8 encoding. byteLength:uint32_t, raw data, flags:uint32_t.
  RegExp = 82,
  /// kBeginJSMap ';' — Beginning of a JS map.
  BeginJSMap = 59,
  /// kEndJSMap ':' — End of a JS map. length:uint32_t.
  EndJSMap = 58,
  /// kBeginJSSet ''' — Beginning of a JS set.
  BeginJSSet = 39,
  /// kEndJSSet ',' — End of a JS set. length:uint32_t.
  EndJSSet = 44,
  /// kArrayBuffer 'B' — Array buffer. byteLength:uint32_t, then raw data.
  ArrayBuffer = 66,
  /// kImmutableArrayBuffer 'C' — Immutable array buffer. byteLength:uint32_t, then raw data.
  ImmutableArrayBuffer = 67,
  /// kSharedImmutableArrayBuffer 'E' — Shared immutable array buffer (zero-copy for postMessage). backingStoreID:uint32_t
  SharedImmutableArrayBuffer = 69,
  /// kResizableArrayBuffer '~' — Resizable array buffer. byteLength:uint32_t, maxLength:uint32_t, raw data.
  ResizableArrayBuffer = 126,
  /// kArrayBufferTransfer 't' — Array buffer (transferred). transferID:uint32_t
  ArrayBufferTransfer = 116,
  /// kArrayBufferView 'V' — View into an array buffer. subtag:ArrayBufferViewTag, byteOffset:uint32_t, byteLength:uint32_t For typed arrays, byteOffset and byteLength must be divisible by the size of the element. Note: kArrayBufferView is special, and should have an ArrayBuffer (or an ObjectReference to one) serialized just before it. This is a quirk arising from the previous stack-based implementation.
  ArrayBufferView = 86,
  /// kSharedArrayBuffer 'u' — Shared array buffer. transferID:uint32_t
  SharedArrayBuffer = 117,
  /// kSharedObject 'p' — A HeapObject shared across Isolates. sharedValueID:uint32_t
  SharedObject = 112,
  /// kWasmModuleTransfer 'w' — A wasm module object transfer. next value is its index.
  WasmModuleTransfer = 119,
  /// kHostObject — The delegate is responsible for processing all following data. This "escapes" to whatever wire format the delegate chooses.
  HostObject = 92,
  /// kWasmMemoryTransfer 'm' — A transferred WebAssembly.Memory object. maximumPages:int32_t, then by SharedArrayBuffer tag and its data.
  WasmMemoryTransfer = 109,
  /// kError 'r' — A list of (subtag: ErrorTag, [subtag dependent data]). See ErrorTag for details.
  Error = 114,
  /// kLegacyReservedMessagePort 'M' — The following tags are reserved because they were in use in Chromium before the kHostObject tag was introduced in format version 13, at   v8           refs/heads/master@{#43466}   chromium/src refs/heads/master@{#453568} They must not be reused without a version check to prevent old values from starting to deserialize incorrectly. For simplicity, it's recommended to avoid them altogether. This is the set of tags that existed in SerializationTag.h at that time and still exist at the time of this writing (i.e., excluding those that were removed on the Chromium side because there should be no real user data containing them). It might be possible to also free up other tags which were never persisted (e.g. because they were used only for transfer) in the future.
  LegacyReservedMessagePort = 77,
  /// kLegacyReservedBlob 'b'
  LegacyReservedBlob = 98,
  /// kLegacyReservedBlobIndex 'i'
  LegacyReservedBlobIndex = 105,
  /// kLegacyReservedFile 'f'
  LegacyReservedFile = 102,
  /// kLegacyReservedFileIndex 'e'
  LegacyReservedFileIndex = 101,
  /// kLegacyReservedDOMFileSystem 'd'
  LegacyReservedDOMFileSystem = 100,
  /// kLegacyReservedFileList 'l'
  LegacyReservedFileList = 108,
  /// kLegacyReservedFileListIndex 'L'
  LegacyReservedFileListIndex = 76,
  /// kLegacyReservedImageData '#'
  LegacyReservedImageData = 35,
  /// kLegacyReservedImageBitmap 'g'
  LegacyReservedImageBitmap = 103,
  /// kLegacyReservedImageBitmapTransfer 'G'
  LegacyReservedImageBitmapTransfer = 71,
  /// kLegacyReservedOffscreenCanvas 'H'
  LegacyReservedOffscreenCanvas = 72,
  /// kLegacyReservedCryptoKey 'K'
  LegacyReservedCryptoKey = 75,
  /// kLegacyReservedRTCCertificate 'k'
  LegacyReservedRTCCertificate = 107,
}

impl SerializationTag {
  /// 标签总数
  pub const COUNT: usize = 59;

  /// 字节反查
  #[inline]
  #[must_use]
  pub const fn from_byte(b: u8) -> Option<Self> {
    match b {
      255 => Some(Self::Version),
      0 => Some(Self::Padding),
      63 => Some(Self::VerifyObjectCount),
      45 => Some(Self::TheHole),
      95 => Some(Self::Undefined),
      48 => Some(Self::Null),
      84 => Some(Self::True),
      70 => Some(Self::False),
      73 => Some(Self::Int32),
      85 => Some(Self::Uint32),
      78 => Some(Self::Double),
      90 => Some(Self::BigInt),
      83 => Some(Self::Utf8String),
      34 => Some(Self::OneByteString),
      99 => Some(Self::TwoByteString),
      94 => Some(Self::ObjectReference),
      111 => Some(Self::BeginJSObject),
      123 => Some(Self::EndJSObject),
      97 => Some(Self::BeginSparseJSArray),
      64 => Some(Self::EndSparseJSArray),
      65 => Some(Self::BeginDenseJSArray),
      36 => Some(Self::EndDenseJSArray),
      68 => Some(Self::Date),
      121 => Some(Self::TrueObject),
      120 => Some(Self::FalseObject),
      110 => Some(Self::NumberObject),
      122 => Some(Self::BigIntObject),
      115 => Some(Self::StringObject),
      82 => Some(Self::RegExp),
      59 => Some(Self::BeginJSMap),
      58 => Some(Self::EndJSMap),
      39 => Some(Self::BeginJSSet),
      44 => Some(Self::EndJSSet),
      66 => Some(Self::ArrayBuffer),
      67 => Some(Self::ImmutableArrayBuffer),
      69 => Some(Self::SharedImmutableArrayBuffer),
      126 => Some(Self::ResizableArrayBuffer),
      116 => Some(Self::ArrayBufferTransfer),
      86 => Some(Self::ArrayBufferView),
      117 => Some(Self::SharedArrayBuffer),
      112 => Some(Self::SharedObject),
      119 => Some(Self::WasmModuleTransfer),
      92 => Some(Self::HostObject),
      109 => Some(Self::WasmMemoryTransfer),
      114 => Some(Self::Error),
      77 => Some(Self::LegacyReservedMessagePort),
      98 => Some(Self::LegacyReservedBlob),
      105 => Some(Self::LegacyReservedBlobIndex),
      102 => Some(Self::LegacyReservedFile),
      101 => Some(Self::LegacyReservedFileIndex),
      100 => Some(Self::LegacyReservedDOMFileSystem),
      108 => Some(Self::LegacyReservedFileList),
      76 => Some(Self::LegacyReservedFileListIndex),
      35 => Some(Self::LegacyReservedImageData),
      103 => Some(Self::LegacyReservedImageBitmap),
      71 => Some(Self::LegacyReservedImageBitmapTransfer),
      72 => Some(Self::LegacyReservedOffscreenCanvas),
      75 => Some(Self::LegacyReservedCryptoKey),
      107 => Some(Self::LegacyReservedRTCCertificate),
      _ => None,
    }
  }
}

// —— 提取不变式断言 ——
const _: () = {
  assert!(SerializationTag::COUNT == 59);
  assert!(SerializationTag::Version as u8 == 0xFF);
  assert!(SerializationTag::Padding as u8 == 0);
  assert!(SerializationTag::True as u8 == b'T');
  assert!(SerializationTag::False as u8 == b'F');
  assert!(SerializationTag::BeginJSObject as u8 == b'o');
  assert!(SerializationTag::EndJSObject as u8 == b'{');
  assert!(SerializationTag::HostObject as u8 == b'\\');
};
