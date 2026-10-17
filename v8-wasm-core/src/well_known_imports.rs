// 由 tool/src/wasm/well-known-imports.h.js 从 v8/src/wasm/well-known-imports.h 生成; 修订就地改, 复跑覆盖
// WellKnownImport 枚举与 WellKnownImportsList 状态常量

/// 已知的 well-known 导入函数类别 (对应原版 enum class WellKnownImport : uint8_t, 共 81 项)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum WellKnownImport {
  /// kUninstantiated
  Uninstantiated = 0,
  /// kGeneric
  Generic = 1,
  /// kLinkError
  LinkError = 2,
  /// kStringCast (= kFirstCompileTimeImport, 别名经 FIRST_COMPILE_TIME_IMPORT 常量表达)
  StringCast = 3,
  /// kStringCastShared
  StringCastShared = 4,
  /// kStringCharCodeAt
  StringCharCodeAt = 5,
  /// kStringCharCodeAtShared
  StringCharCodeAtShared = 6,
  /// kStringCodePointAt
  StringCodePointAt = 7,
  /// kStringCodePointAtShared
  StringCodePointAtShared = 8,
  /// kStringCompare
  StringCompare = 9,
  /// kStringCompareShared
  StringCompareShared = 10,
  /// kStringConcat
  StringConcat = 11,
  /// kStringConcatShared
  StringConcatShared = 12,
  /// kStringEquals
  StringEquals = 13,
  /// kStringEqualsShared
  StringEqualsShared = 14,
  /// kStringFromCharCode
  StringFromCharCode = 15,
  /// kStringFromCharCodeShared
  StringFromCharCodeShared = 16,
  /// kStringFromCodePoint
  StringFromCodePoint = 17,
  /// kStringFromCodePointShared
  StringFromCodePointShared = 18,
  /// kStringFromUtf8Array
  StringFromUtf8Array = 19,
  /// kStringFromUtf8ArrayShared
  StringFromUtf8ArrayShared = 20,
  /// kStringFromWtf16Array
  StringFromWtf16Array = 21,
  /// kStringFromWtf16ArrayShared
  StringFromWtf16ArrayShared = 22,
  /// kStringIntoUtf8Array
  StringIntoUtf8Array = 23,
  /// kStringIntoUtf8ArrayShared
  StringIntoUtf8ArrayShared = 24,
  /// kStringLength
  StringLength = 25,
  /// kStringLengthShared
  StringLengthShared = 26,
  /// kStringMeasureUtf8
  StringMeasureUtf8 = 27,
  /// kStringMeasureUtf8Shared
  StringMeasureUtf8Shared = 28,
  /// kStringSubstring
  StringSubstring = 29,
  /// kStringSubstringShared
  StringSubstringShared = 30,
  /// kStringTest
  StringTest = 31,
  /// kStringTestShared
  StringTestShared = 32,
  /// kStringToUtf8Array
  StringToUtf8Array = 33,
  /// kStringToUtf8ArrayShared
  StringToUtf8ArrayShared = 34,
  /// kStringToWtf16Array
  StringToWtf16Array = 35,
  /// kStringToWtf16ArrayShared
  StringToWtf16ArrayShared = 36,
  /// kConfigureAllPrototypes (= kLastCompileTimeImport, 别名经 LAST_COMPILE_TIME_IMPORT 常量表达)
  ConfigureAllPrototypes = 37,
  /// kDataViewGetBigInt64
  DataViewGetBigInt64 = 38,
  /// kDataViewGetBigUint64
  DataViewGetBigUint64 = 39,
  /// kDataViewGetFloat32
  DataViewGetFloat32 = 40,
  /// kDataViewGetFloat64
  DataViewGetFloat64 = 41,
  /// kDataViewGetInt8
  DataViewGetInt8 = 42,
  /// kDataViewGetInt16
  DataViewGetInt16 = 43,
  /// kDataViewGetInt32
  DataViewGetInt32 = 44,
  /// kDataViewGetUint8
  DataViewGetUint8 = 45,
  /// kDataViewGetUint16
  DataViewGetUint16 = 46,
  /// kDataViewGetUint32
  DataViewGetUint32 = 47,
  /// kDataViewSetBigInt64
  DataViewSetBigInt64 = 48,
  /// kDataViewSetBigUint64
  DataViewSetBigUint64 = 49,
  /// kDataViewSetFloat32
  DataViewSetFloat32 = 50,
  /// kDataViewSetFloat64
  DataViewSetFloat64 = 51,
  /// kDataViewSetInt8
  DataViewSetInt8 = 52,
  /// kDataViewSetInt16
  DataViewSetInt16 = 53,
  /// kDataViewSetInt32
  DataViewSetInt32 = 54,
  /// kDataViewSetUint8
  DataViewSetUint8 = 55,
  /// kDataViewSetUint16
  DataViewSetUint16 = 56,
  /// kDataViewSetUint32
  DataViewSetUint32 = 57,
  /// kDataViewByteLength
  DataViewByteLength = 58,
  /// kMathF64Acos
  MathF64Acos = 59,
  /// kMathF64Asin
  MathF64Asin = 60,
  /// kMathF64Atan
  MathF64Atan = 61,
  /// kMathF64Atan2
  MathF64Atan2 = 62,
  /// kMathF64Cos
  MathF64Cos = 63,
  /// kMathF64Sin
  MathF64Sin = 64,
  /// kMathF64Tan
  MathF64Tan = 65,
  /// kMathF64Exp
  MathF64Exp = 66,
  /// kMathF64Log
  MathF64Log = 67,
  /// kMathF64Pow
  MathF64Pow = 68,
  /// kMathF64Sqrt
  MathF64Sqrt = 69,
  /// kDoubleToString
  DoubleToString = 70,
  /// kIntToString
  IntToString = 71,
  /// kParseFloat
  ParseFloat = 72,
  /// kStringIndexOf
  StringIndexOf = 73,
  /// kStringIndexOfImported
  StringIndexOfImported = 74,
  /// kStringToLocaleLowerCaseStringref
  StringToLocaleLowerCaseStringref = 75,
  /// kStringToLowerCaseStringref
  StringToLowerCaseStringref = 76,
  /// kStringToLowerCaseImported
  StringToLowerCaseImported = 77,
  /// kFastAPICall
  FastAPICall = 78,
}

impl WellKnownImport {
  /// 枚举总数
  pub const COUNT: usize = 81;

  /// 首个编译期内建导入 (原版 kFirstCompileTimeImport = kStringCast)
  pub const FIRST_COMPILE_TIME_IMPORT: Self = Self::StringCast;

  /// 末个编译期内建导入 (原版 kLastCompileTimeImport = kConfigureAllPrototypes)
  pub const LAST_COMPILE_TIME_IMPORT: Self = Self::ConfigureAllPrototypes;

  /// 是否为编译期内建导入 (原版 IsCompileTimeImport)
  #[inline]
  #[must_use]
  pub const fn is_compile_time_import(self) -> bool {
    (self as u8) >= Self::FIRST_COMPILE_TIME_IMPORT as u8
      && (self as u8) <= Self::LAST_COMPILE_TIME_IMPORT as u8
  }

  /// 从数值反查
  #[inline]
  #[must_use]
  pub const fn from_repr(val: u8) -> Option<Self> {
    match val {
      0 => Some(Self::Uninstantiated),
      1 => Some(Self::Generic),
      2 => Some(Self::LinkError),
      3 => Some(Self::StringCast),
      4 => Some(Self::StringCastShared),
      5 => Some(Self::StringCharCodeAt),
      6 => Some(Self::StringCharCodeAtShared),
      7 => Some(Self::StringCodePointAt),
      8 => Some(Self::StringCodePointAtShared),
      9 => Some(Self::StringCompare),
      10 => Some(Self::StringCompareShared),
      11 => Some(Self::StringConcat),
      12 => Some(Self::StringConcatShared),
      13 => Some(Self::StringEquals),
      14 => Some(Self::StringEqualsShared),
      15 => Some(Self::StringFromCharCode),
      16 => Some(Self::StringFromCharCodeShared),
      17 => Some(Self::StringFromCodePoint),
      18 => Some(Self::StringFromCodePointShared),
      19 => Some(Self::StringFromUtf8Array),
      20 => Some(Self::StringFromUtf8ArrayShared),
      21 => Some(Self::StringFromWtf16Array),
      22 => Some(Self::StringFromWtf16ArrayShared),
      23 => Some(Self::StringIntoUtf8Array),
      24 => Some(Self::StringIntoUtf8ArrayShared),
      25 => Some(Self::StringLength),
      26 => Some(Self::StringLengthShared),
      27 => Some(Self::StringMeasureUtf8),
      28 => Some(Self::StringMeasureUtf8Shared),
      29 => Some(Self::StringSubstring),
      30 => Some(Self::StringSubstringShared),
      31 => Some(Self::StringTest),
      32 => Some(Self::StringTestShared),
      33 => Some(Self::StringToUtf8Array),
      34 => Some(Self::StringToUtf8ArrayShared),
      35 => Some(Self::StringToWtf16Array),
      36 => Some(Self::StringToWtf16ArrayShared),
      37 => Some(Self::ConfigureAllPrototypes),
      38 => Some(Self::DataViewGetBigInt64),
      39 => Some(Self::DataViewGetBigUint64),
      40 => Some(Self::DataViewGetFloat32),
      41 => Some(Self::DataViewGetFloat64),
      42 => Some(Self::DataViewGetInt8),
      43 => Some(Self::DataViewGetInt16),
      44 => Some(Self::DataViewGetInt32),
      45 => Some(Self::DataViewGetUint8),
      46 => Some(Self::DataViewGetUint16),
      47 => Some(Self::DataViewGetUint32),
      48 => Some(Self::DataViewSetBigInt64),
      49 => Some(Self::DataViewSetBigUint64),
      50 => Some(Self::DataViewSetFloat32),
      51 => Some(Self::DataViewSetFloat64),
      52 => Some(Self::DataViewSetInt8),
      53 => Some(Self::DataViewSetInt16),
      54 => Some(Self::DataViewSetInt32),
      55 => Some(Self::DataViewSetUint8),
      56 => Some(Self::DataViewSetUint16),
      57 => Some(Self::DataViewSetUint32),
      58 => Some(Self::DataViewByteLength),
      59 => Some(Self::MathF64Acos),
      60 => Some(Self::MathF64Asin),
      61 => Some(Self::MathF64Atan),
      62 => Some(Self::MathF64Atan2),
      63 => Some(Self::MathF64Cos),
      64 => Some(Self::MathF64Sin),
      65 => Some(Self::MathF64Tan),
      66 => Some(Self::MathF64Exp),
      67 => Some(Self::MathF64Log),
      68 => Some(Self::MathF64Pow),
      69 => Some(Self::MathF64Sqrt),
      70 => Some(Self::DoubleToString),
      71 => Some(Self::IntToString),
      72 => Some(Self::ParseFloat),
      73 => Some(Self::StringIndexOf),
      74 => Some(Self::StringIndexOfImported),
      75 => Some(Self::StringToLocaleLowerCaseStringref),
      76 => Some(Self::StringToLowerCaseStringref),
      77 => Some(Self::StringToLowerCaseImported),
      78 => Some(Self::FastAPICall),
      _ => None,
    }
  }
}

/// WellKnownImportsList::UpdateResult — 更新结果 (原版 base::StrongAlias<UpdateResultTag, bool>)
pub type UpdateResult = bool;

/// 更新发现不兼容项 (原版 kFoundIncompatibility)
pub const FOUND_INCOMPATIBILITY: UpdateResult = false;

/// 更新成功 (原版 kOK)
pub const UPDATE_OK: UpdateResult = true;

// —— 提取不变式断言 ——
const _: () = {
  assert!(WellKnownImport::COUNT == 81);
  assert!(WellKnownImport::Uninstantiated as u8 == 0);
  assert!(WellKnownImport::StringCast as u8 == WellKnownImport::FIRST_COMPILE_TIME_IMPORT as u8);
  assert!(
    WellKnownImport::ConfigureAllPrototypes as u8
      == WellKnownImport::LAST_COMPILE_TIME_IMPORT as u8
  );
  assert!(!WellKnownImport::Uninstantiated.is_compile_time_import());
  assert!(WellKnownImport::StringCast.is_compile_time_import());
  assert!(WellKnownImport::ConfigureAllPrototypes.is_compile_time_import());
  assert!(!WellKnownImport::FastAPICall.is_compile_time_import());
};
