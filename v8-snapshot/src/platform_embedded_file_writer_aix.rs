// 由 tool/src/snapshot/embedded/platform-embedded-file-writer-aix.js 从 v8/src/snapshot/embedded/platform-embedded-file-writer-aix.{h,cc} 生成; 修订就地改, 复跑覆盖
// XCOFF 风格平台写入器 (AIX)

// PlatformEmbeddedFileWriterAIX (XCOFF 风格)。
// 符号前缀 (原版 SYMBOL_PREFIX 机械展开): ""
// 汇编指令表 (原版 DirectiveAsString; AIX 特例 kQuad → `.llong`):
// - kByte → `.byte`
// - kLong → `.long`
// - kQuad → `.llong`

// 段/对齐指令 (机械提取): `.csect [GL], 6\n`, `.csect[RO]\n`, `.align 2\n`, `.globl %s, hidden\n`, `.align %d\n`, `.align 3\n`, `.globl %s\n`, `.xline %d, \`, `.csect %s[DS]\n`, `.llong .%s, 0, 0\n`
// 函数描述符: .csect <name>[DS] + .llong .<name>, 0, 0 (AIX 调用约定);
// 源信息用 .xline; 全局符号 .globl <name>, hidden。
// 实现方法面 (共 16 项): SectionText, SectionRoData, DeclareUint32, DeclareSymbolGlobal, AlignToCodeAlignment, AlignToDataAlignment, Comment, DeclareLabel, SourceInfo, DeclareFunctionBegin, DeclareFunctionEnd, FilePrologue, DeclareExternalFilename, FileEpilogue, IndentedDataDirective, ByteChunkDataDirective
