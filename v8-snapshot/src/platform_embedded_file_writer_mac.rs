// 由 tool/src/snapshot/embedded/platform-embedded-file-writer-mac.js 从 v8/src/snapshot/embedded/platform-embedded-file-writer-mac.{h,cc} 生成; 修订就地改, 复跑覆盖
// Mach-O 风格平台写入器 (macOS)

// PlatformEmbeddedFileWriterMac (Mach-O 风格)。
// 汇编指令表 (原版 DirectiveAsString):
// - kByte → `.byte`
// - kLong → `.long`
// - kQuad → `.quad`
// - kOcta → `.octa`

// 段指令 (机械提取): `.text\n`, `.const_data\n`, `.private_extern _%s\n`, `.balign 16384\n`, `.balign %d\n`, `.balign 16384\n`, `.balign 8\n`, `.loc %d %d\n`, `.file %d \`
// (代码 .text / 只读数据 .const_data)
// 全局符号: .private_extern _<name> — 原版注释登记: 用 .global 会引发
// 嵌入 blob 哈希运行时校验失败, 故用 private_extern。
// 实现方法面 (共 16 项): SectionText, SectionRoData, DeclareUint32, DeclareSymbolGlobal, AlignToCodeAlignment, AlignToPageSizeIfNeeded, AlignToDataAlignment, Comment, DeclareLabel, SourceInfo, DeclareFunctionBegin, DeclareFunctionEnd, FilePrologue, DeclareExternalFilename, FileEpilogue, IndentedDataDirective
