// 由 tool/src/snapshot/embedded/embedded-file-writer.js 从 v8/src/snapshot/embedded/embedded-file-writer.{h,cc} 生成; 修订就地改, 复跑覆盖
// 嵌入汇编文件写入器

/// 临时字符串缓冲区长度 (原版 kTemporaryStringLength)
pub const TEMPORARY_STRING_LENGTH: usize = 256;
/// 外部编译文件名合成 ID 首值 (原版 kFirstExternalFilenameId)
pub const FIRST_EXTERNAL_FILENAME_ID: i32 = 1;

// EmbeddedFileWriter 方法面 (原版 h public 区, 共 9 项):
// - `LookupOrAddExternallyCompiledFilename`
// - `GetExternallyCompiledFilenameCount`
// - `PrepareBuiltinSourcePositionMap`
// - `SetBuiltinUnwindData`
// - `SetEmbeddedFile`
// - `SetEmbeddedVariant`
// - `SetTargetArch`
// - `SetTargetOs`
// - `WriteEmbedded`

// cc 实现 (机械清点): WriteBuiltin, WriteBuiltinLabels, WriteCodeSection, WriteDebugSection, WriteFileEpilogue, WriteBinaryContentsAsInlineAssembly, LookupOrAddExternallyCompiledFilename, GetExternallyCompiledFilename, GetExternallyCompiledFilenameCount, PrepareBuiltinSourcePositionMap
// 输出契约: FilePrologue → 布局描述/内建标签 → 代码段 (WriteBuiltin 循环,
// PadAndAlignCode 对齐) → 数据段 → FileEpilogue; 平台差异全部委托
// PlatformEmbeddedFileWriterBase (见 platform_embedded_file_writer_base)。
