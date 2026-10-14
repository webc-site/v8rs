// 由 tool/src/snapshot/embedded/platform-embedded-file-writer-base.js 从 v8/src/snapshot/embedded/platform-embedded-file-writer-base.{h,cc} 生成; 修订就地改, 复跑覆盖
// 平台嵌入文件写入器基类

/// v8: src/snapshot/embedded/platform-embedded-file-writer-base.h DataDirective
/// 汇编数据指令 (原版 enum DataDirective, 顺序赋值)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum DataDirective {
  // kByte — 1 字节/项
  Byte = 0,
  // kLong — 4 字节/项
  Long = 1,
  // kQuad — 8 字节/项
  Quad = 2,
  // kOcta — 16 字节/项
  Octa = 3,
}

impl DataDirective {
  /// 原版 DataDirectiveSize(directive)
  #[inline]
  #[must_use]
  pub const fn size(self) -> usize {
    match self {
      Self::Byte => 1,
      Self::Long => 4,
      Self::Quad => 8,
      Self::Octa => 16,
    }
  }
}

/// v8: src/snapshot/embedded/platform-embedded-file-writer-base.h EmbeddedTargetOs
/// 嵌入目标操作系统 (原版 enum class EmbeddedTargetOs)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum EmbeddedTargetOs {
  // kAIX
  AIX,
  // kAndroid
  Android,
  // kChromeOS
  ChromeOS,
  // kFuchsia
  Fuchsia,
  // kMac
  Mac,
  // kWin
  Win,
  // kStarboard
  Starboard,
  // kZOS
  ZOS,
  // kGeneric
  Generic,
}

/// v8: src/snapshot/embedded/platform-embedded-file-writer-base.h EmbeddedTargetArch
/// 嵌入目标架构 (原版 enum class EmbeddedTargetArch)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum EmbeddedTargetArch {
  // kArm
  Arm,
  // kArm64
  Arm64,
  // kIA32
  IA32,
  // kX64
  X64,
  // kGeneric
  Generic,
}

// PlatformEmbeddedFileWriterBase (原版抽象基类, 为 embedded.S 输出平台相关汇编):
// 纯虚方法面 (共 15 项):
// - `SectionText`
// - `SectionRoData`
// - `AlignToCodeAlignment`
// - `AlignToDataAlignment`
// - `DeclareUint32`
// - `DeclareSymbolGlobal`
// - `DeclareLabel`
// - `SourceInfo`
// - `DeclareFunctionBegin`
// - `DeclareFunctionEnd`
// - `Comment`
// - `FilePrologue`
// - `DeclareExternalFilename`
// - `FileEpilogue`
// - `IndentedDataDirective`

// 带默认实现: AlignToPageSizeIfNeeded, DeclareLabelProlog, DeclareLabelEpilogue, Newline, MaybeEmitUnwindData;
// MaybeEmitUnwindData 仅 x64 Windows 发射 unwind 数据, 其余平台空操作。
// 工厂: NewPlatformEmbeddedFileWriter(target_arch, target_os) 按目标选实现。
