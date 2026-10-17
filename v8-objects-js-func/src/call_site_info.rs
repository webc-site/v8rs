// 由 tool/src/objects/call-site-info.h.js 从 v8/src/objects/call-site-info.{h,cc} 生成; 修订就地改, 复跑覆盖
// CallSiteInfo 栈帧标志位域 (V8_ENABLE_DRUMBRAKE 关闭分支展开, 开启时追加 IsWasmInterpretedFrame 位 7)

/// CallSiteInfo 标志位域 (原版 IsWasmBit..IsDeferredBaselineFrameBit BitField 链)
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct CallSiteFlags(pub u32);

impl CallSiteFlags {
  /// IsWasmBit: 位 0, 宽 1
  pub const IsWasm_SHIFT: u32 = 0;
  pub const IsWasm_MASK: u32 = 0x1;
  /// IsStrictBit: 位 1, 宽 1
  pub const IsStrict_SHIFT: u32 = 1;
  pub const IsStrict_MASK: u32 = 0x2;
  /// IsConstructorBit: 位 2, 宽 1
  pub const IsConstructor_SHIFT: u32 = 2;
  pub const IsConstructor_MASK: u32 = 0x4;
  /// IsAsyncBit: 位 3, 宽 1
  pub const IsAsync_SHIFT: u32 = 3;
  pub const IsAsync_MASK: u32 = 0x8;
  /// IsBuiltinBit: 位 4, 宽 1
  pub const IsBuiltin_SHIFT: u32 = 4;
  pub const IsBuiltin_MASK: u32 = 0x10;
  /// IsSourcePositionComputedBit: 位 5, 宽 1
  pub const IsSourcePositionComputed_SHIFT: u32 = 5;
  pub const IsSourcePositionComputed_MASK: u32 = 0x20;
  /// IsDeferredBaselineFrameBit: 位 6, 宽 1
  pub const IsDeferredBaselineFrame_SHIFT: u32 = 6;
  pub const IsDeferredBaselineFrame_MASK: u32 = 0x40;

  /// 原始位串
  #[inline]
  #[must_use]
  pub const fn bits(self) -> u32 {
    self.0
  }

  /// 读取 IsWasm 位 (原版 IsWasmBit)
  #[inline]
  #[must_use]
  pub const fn is_wasm(self) -> bool {
    self.0 & Self::IsWasm_MASK != 0
  }

  /// 读取 IsStrict 位 (原版 IsStrictBit)
  #[inline]
  #[must_use]
  pub const fn is_strict(self) -> bool {
    self.0 & Self::IsStrict_MASK != 0
  }

  /// 读取 IsConstructor 位 (原版 IsConstructorBit)
  #[inline]
  #[must_use]
  pub const fn is_constructor(self) -> bool {
    self.0 & Self::IsConstructor_MASK != 0
  }

  /// 读取 IsAsync 位 (原版 IsAsyncBit)
  #[inline]
  #[must_use]
  pub const fn is_async(self) -> bool {
    self.0 & Self::IsAsync_MASK != 0
  }

  /// 读取 IsBuiltin 位 (原版 IsBuiltinBit)
  #[inline]
  #[must_use]
  pub const fn is_builtin(self) -> bool {
    self.0 & Self::IsBuiltin_MASK != 0
  }

  /// 读取 IsSourcePositionComputed 位 (原版 IsSourcePositionComputedBit)
  #[inline]
  #[must_use]
  pub const fn is_source_position_computed(self) -> bool {
    self.0 & Self::IsSourcePositionComputed_MASK != 0
  }

  /// 读取 IsDeferredBaselineFrame 位 (原版 IsDeferredBaselineFrameBit)
  #[inline]
  #[must_use]
  pub const fn is_deferred_baseline_frame(self) -> bool {
    self.0 & Self::IsDeferredBaselineFrame_MASK != 0
  }
}

/// Flag 掩码枚举 (原版 enum Flag : uint32_t)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct CallSiteFlag(pub u32);

impl CallSiteFlag {
  /// kNone
  pub const NONE: Self = Self(0);
  /// kIsWasm = IsWasmBit::kMask
  pub const IsWasm: Self = Self(0x1);
  /// kIsStrict = IsStrictBit::kMask
  pub const IsStrict: Self = Self(0x2);
  /// kIsConstructor = IsConstructorBit::kMask
  pub const IsConstructor: Self = Self(0x4);
  /// kIsAsync = IsAsyncBit::kMask
  pub const IsAsync: Self = Self(0x8);
  /// kIsBuiltin = IsBuiltinBit::kMask
  pub const IsBuiltin: Self = Self(0x10);
  /// kIsSourcePositionComputed = IsSourcePositionComputedBit::kMask
  pub const IsSourcePositionComputed: Self = Self(0x20);
  /// kIsDeferredBaselineFrame = IsDeferredBaselineFrameBit::kMask
  pub const IsDeferredBaselineFrame: Self = Self(0x40);

  /// 含判定
  #[inline]
  #[must_use]
  pub const fn contains(self, other: Self) -> bool {
    self.0 & other.0 == other.0
  }

  /// 合并
  #[inline]
  #[must_use]
  pub const fn union(self, other: Self) -> Self {
    Self(self.0 | other.0)
  }
}

/// 位域总数, 非 DRUMBRAKE 分支 (原版 kFlagCount; DRUMBRAKE 开启时为 8)
pub const CALL_SITE_FLAG_COUNT: usize = 7;

/// 延迟帧原始捕获数组布局 (原版 enum Fields)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum CallSiteFields {
  /// kCode
  Code = 0,
  /// kReceiver
  Receiver = 1,
  /// kFunction
  Function = 2,
  /// kOffset
  Offset = 3,
  /// kFlags
  Flags = 4,
  /// kCount
  Count = 5,
}

impl CallSiteFields {
  /// 布局槽数 (原版 kCount)
  pub const COUNT: usize = 6;
}

/// 未知源位置哨兵 (原版 kUnknown = kNoSourcePosition)
pub const CALL_SITE_UNKNOWN: i32 = -1;

/// 延迟标志掩码, 仅在裸捕获数组中有效 (原版 kDeferredFlagsMask)
pub const DEFERRED_FLAGS_MASK: u32 = 0x40;

// —— 提取不变式断言 ——
const _: () = {
  assert!(CALL_SITE_FLAG_COUNT == 7);
  assert!(DEFERRED_FLAGS_MASK == 0x40);
  assert!(CALL_SITE_UNKNOWN == -1);
  assert!(CallSiteFields::COUNT == 6);
  assert!(CallSiteFlags::default().bits() == 0);
};
