// 由 tool/src/snapshot/embedded/platform-embedded-file-writer-zos.js 从 v8/src/snapshot/embedded/platform-embedded-file-writer-zos.{h,cc} 生成; 修订就地改, 复跑覆盖
// z/OS 平台写入器

// PlatformEmbeddedFileWriterZOS (z/OS)。
// 十六进制字面量 (原版 HexLiteral): `%.16lx` — 16 位宽零填充;
// 文件尾声 (原版 FileEpilogue): ` end`。
// fprintf 串登记 (共 2 条, z/OS 段指令走 GOFF 宏, 其余登记):
// - `%.16lx`
// - ` end\n`

// 实现方法面 (共 20 项): DeclareLabelProlog, DeclareLabelEpilogue, DeclareUint32, DeclareSymbolGlobal, AlignToCodeAlignment, AlignToDataAlignment, Comment, DeclareLabel, SourceInfo, DeclareFunctionBegin, DeclareFunctionEnd, HexLiteral, FilePrologue, DeclareExternalFilename, FileEpilogue, IndentedDataDirective, ByteChunkDataDirective, WriteByteChunk, SectionText, SectionRoData
