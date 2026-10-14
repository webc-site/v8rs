// 由 tool/src/objects/value-serializer.h.js 从 v8/src/objects/value-serializer.{h,cc} 生成; 修订就地改, 复跑覆盖
// 结构化克隆线上格式 SerializationTag 全量表
//! SerializationTag: 结构化序列化 (structured clone) 线上格式标签。
//!
//! 数值即 ASCII 字节 (kVersion=0xFF 除外)。语义实现面 (序列化器/反序列化器,
//! 约 3000 行) 不在本产物范围, 由 Developer 就地实现。

/// 原版 enum class SerializationTag : uint8_t, 共 59 标签。
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum SerializationTag {
  /// kVersion — version:uint32_t (if at beginning of data, sets version > 0)
  Version = 255,
  /// kPadding — ignore
  Padding = 0,
  /// kVerifyObjectCount '?' — refTableSize:uint32_t (previously used for sanity checks; safe to ignore)
  Verifyobjectcount = 63,
  /// kTheHole '-' — Oddballs (no data).
  Thehole = 45,
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
  Bigint = 90,
  /// kUtf8String 'S' — byteLength:uint32_t, then raw data
  Utf8string = 83,
  /// kOneByteString '"'
  Onebytestring = 34,
  /// kTwoByteString 'c'
  Twobytestring = 99,
  /// kObjectReference '^' — Reference to a serialized object. objectID:uint32_t
  Objectreference = 94,
  /// kBeginJSObject 'o' — Beginning of a JS object.
  Beginjsobject = 111,
  /// kEndJSObject '{' — End of a JS object. numProperties:uint32_t
  Endjsobject = 123,
  /// kBeginSparseJSArray 'a' — Beginning of a sparse JS array. length:uint32_t Elements and properties are written as key/value pairs, like objects.
  Beginsparsejsarray = 97,
  /// kEndSparseJSArray '@' — End of a sparse JS array. numProperties:uint32_t length:uint32_t
  Endsparsejsarray = 64,
  /// kBeginDenseJSArray 'A' — Beginning of a dense JS array. length:uint32_t |length| elements, followed by properties as key/value pairs
  Begindensejsarray = 65,
  /// kEndDenseJSArray '$' — End of a dense JS array. numProperties:uint32_t length:uint32_t
  Enddensejsarray = 36,
  /// kDate 'D' — Date. millisSinceEpoch:double
  Date = 68,
  /// kTrueObject 'y' — Boolean object. No data.
  Trueobject = 121,
  /// kFalseObject 'x'
  Falseobject = 120,
  /// kNumberObject 'n' — Number object. value:double
  Numberobject = 110,
  /// kBigIntObject 'z' — BigInt object. Bitfield:uint32_t, then raw digits storage.
  Bigintobject = 122,
  /// kStringObject 's' — String object, UTF-8 encoding. byteLength:uint32_t, then raw data.
  Stringobject = 115,
  /// kRegExp 'R' — Regular expression, UTF-8 encoding. byteLength:uint32_t, raw data, flags:uint32_t.
  Regexp = 82,
  /// kBeginJSMap ';' — Beginning of a JS map.
  Beginjsmap = 59,
  /// kEndJSMap ':' — End of a JS map. length:uint32_t.
  Endjsmap = 58,
  /// kBeginJSSet ''' — Beginning of a JS set.
  Beginjsset = 39,
  /// kEndJSSet ',' — End of a JS set. length:uint32_t.
  Endjsset = 44,
  /// kArrayBuffer 'B' — Array buffer. byteLength:uint32_t, then raw data.
  Arraybuffer = 66,
  /// kImmutableArrayBuffer 'C' — Immutable array buffer. byteLength:uint32_t, then raw data.
  Immutablearraybuffer = 67,
  /// kSharedImmutableArrayBuffer 'E' — Shared immutable array buffer (zero-copy for postMessage). backingStoreID:uint32_t
  Sharedimmutablearraybuffer = 69,
  /// kResizableArrayBuffer '~' — Resizable array buffer. byteLength:uint32_t, maxLength:uint32_t, raw data.
  Resizablearraybuffer = 126,
  /// kArrayBufferTransfer 't' — Array buffer (transferred). transferID:uint32_t
  Arraybuffertransfer = 116,
  /// kArrayBufferView 'V' — View into an array buffer. subtag:ArrayBufferViewTag, byteOffset:uint32_t, byteLength:uint32_t For typed arrays, byteOffset and byteLength must be divisible by the size of the element. Note: kArrayBufferView is special, and should have an ArrayBuffer (or an ObjectReference to one) serialized just before it. This is a quirk arising from the previous stack-based implementation.
  Arraybufferview = 86,
  /// kSharedArrayBuffer 'u' — Shared array buffer. transferID:uint32_t
  Sharedarraybuffer = 117,
  /// kSharedObject 'p' — A HeapObject shared across Isolates. sharedValueID:uint32_t
  Sharedobject = 112,
  /// kWasmModuleTransfer 'w' — A wasm module object transfer. next value is its index.
  Wasmmoduletransfer = 119,
  /// kHostObject — The delegate is responsible for processing all following data. This "escapes" to whatever wire format the delegate chooses.
  Hostobject = 92,
  /// kWasmMemoryTransfer 'm' — A transferred WebAssembly.Memory object. maximumPages:int32_t, then by SharedArrayBuffer tag and its data.
  Wasmmemorytransfer = 109,
  /// kError 'r' — A list of (subtag: ErrorTag, [subtag dependent data]). See ErrorTag for details.
  Error = 114,
  /// kLegacyReservedMessagePort 'M' — The following tags are reserved because they were in use in Chromium before the kHostObject tag was introduced in format version 13, at   v8           refs/heads/master@{#43466}   chromium/src refs/heads/master@{#453568} They must not be reused without a version check to prevent old values from starting to deserialize incorrectly. For simplicity, it's recommended to avoid them altogether. This is the set of tags that existed in SerializationTag.h at that time and still exist at the time of this writing (i.e., excluding those that were removed on the Chromium side because there should be no real user data containing them). It might be possible to also free up other tags which were never persisted (e.g. because they were used only for transfer) in the future.
  Legacyreservedmessageport = 77,
  /// kLegacyReservedBlob 'b'
  Legacyreservedblob = 98,
  /// kLegacyReservedBlobIndex 'i'
  Legacyreservedblobindex = 105,
  /// kLegacyReservedFile 'f'
  Legacyreservedfile = 102,
  /// kLegacyReservedFileIndex 'e'
  Legacyreservedfileindex = 101,
  /// kLegacyReservedDOMFileSystem 'd'
  Legacyreserveddomfilesystem = 100,
  /// kLegacyReservedFileList 'l'
  Legacyreservedfilelist = 108,
  /// kLegacyReservedFileListIndex 'L'
  Legacyreservedfilelistindex = 76,
  /// kLegacyReservedImageData '#'
  Legacyreservedimagedata = 35,
  /// kLegacyReservedImageBitmap 'g'
  Legacyreservedimagebitmap = 103,
  /// kLegacyReservedImageBitmapTransfer 'G'
  Legacyreservedimagebitmaptransfer = 71,
  /// kLegacyReservedOffscreenCanvas 'H'
  Legacyreservedoffscreencanvas = 72,
  /// kLegacyReservedCryptoKey 'K'
  Legacyreservedcryptokey = 75,
  /// kLegacyReservedRTCCertificate 'k'
  Legacyreservedrtccertificate = 107,
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
      63 => Some(Self::Verifyobjectcount),
      45 => Some(Self::Thehole),
      95 => Some(Self::Undefined),
      48 => Some(Self::Null),
      84 => Some(Self::True),
      70 => Some(Self::False),
      73 => Some(Self::Int32),
      85 => Some(Self::Uint32),
      78 => Some(Self::Double),
      90 => Some(Self::Bigint),
      83 => Some(Self::Utf8string),
      34 => Some(Self::Onebytestring),
      99 => Some(Self::Twobytestring),
      94 => Some(Self::Objectreference),
      111 => Some(Self::Beginjsobject),
      123 => Some(Self::Endjsobject),
      97 => Some(Self::Beginsparsejsarray),
      64 => Some(Self::Endsparsejsarray),
      65 => Some(Self::Begindensejsarray),
      36 => Some(Self::Enddensejsarray),
      68 => Some(Self::Date),
      121 => Some(Self::Trueobject),
      120 => Some(Self::Falseobject),
      110 => Some(Self::Numberobject),
      122 => Some(Self::Bigintobject),
      115 => Some(Self::Stringobject),
      82 => Some(Self::Regexp),
      59 => Some(Self::Beginjsmap),
      58 => Some(Self::Endjsmap),
      39 => Some(Self::Beginjsset),
      44 => Some(Self::Endjsset),
      66 => Some(Self::Arraybuffer),
      67 => Some(Self::Immutablearraybuffer),
      69 => Some(Self::Sharedimmutablearraybuffer),
      126 => Some(Self::Resizablearraybuffer),
      116 => Some(Self::Arraybuffertransfer),
      86 => Some(Self::Arraybufferview),
      117 => Some(Self::Sharedarraybuffer),
      112 => Some(Self::Sharedobject),
      119 => Some(Self::Wasmmoduletransfer),
      92 => Some(Self::Hostobject),
      109 => Some(Self::Wasmmemorytransfer),
      114 => Some(Self::Error),
      77 => Some(Self::Legacyreservedmessageport),
      98 => Some(Self::Legacyreservedblob),
      105 => Some(Self::Legacyreservedblobindex),
      102 => Some(Self::Legacyreservedfile),
      101 => Some(Self::Legacyreservedfileindex),
      100 => Some(Self::Legacyreserveddomfilesystem),
      108 => Some(Self::Legacyreservedfilelist),
      76 => Some(Self::Legacyreservedfilelistindex),
      35 => Some(Self::Legacyreservedimagedata),
      103 => Some(Self::Legacyreservedimagebitmap),
      71 => Some(Self::Legacyreservedimagebitmaptransfer),
      72 => Some(Self::Legacyreservedoffscreencanvas),
      75 => Some(Self::Legacyreservedcryptokey),
      107 => Some(Self::Legacyreservedrtccertificate),
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
  assert!(SerializationTag::BeginJsObject as u8 == b'o');
  assert!(SerializationTag::EndJsObject as u8 == b'{');
  assert!(SerializationTag::HostObject as u8 == b'\\');
};
