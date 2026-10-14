// 由 tool/src/snapshot/embedded/platform-embedded-file-writer-generic.js 从 v8/src/snapshot/embedded/platform-embedded-file-writer-generic.{h,cc} 生成; 修订就地改, 复跑覆盖
// ELF 风格平台写入器 (Linux/Android/ChromeOS/Fuchsia/Starboard)

// PlatformEmbeddedFileWriterGeneric (ELF 风格)。
// 符号前缀 (原版 SYMBOL_PREFIX 宏机械展开): ""
// 汇编指令表 (原版 DirectiveAsString):
// - kByte → `.byte`
// - kLong → `.long`
// - kQuad → `.quad`
// - kOcta → `.octa`

// 段指令 (机械提取): `.section .text.hot.embedded\n`, `.section .text\n`, `.section .rodata\n`, `.section .note.GNU-stack,\`
// 全局符号: .global + .hidden (降动态链接开销);
// ChromeOS 特例 SectionText 走 .text.hot.embedded。
// 实现方法面 (共 17 项): SectionText, SectionRoData, DeclareUint32, DeclareSymbolGlobal, AlignToCodeAlignment, AlignToPageSizeIfNeeded, AlignToDataAlignment, Comment, DeclareLabel, SourceInfo, DeclareFunctionBegin, DeclareFunctionEnd, FilePrologue, DeclareExternalFilename, FileEpilogue, IndentedDataDirective, ByteChunkDataDirective
