// 由 tool/src/snapshot/embedded/platform-embedded-file-writer-win.js 从 v8/src/snapshot/embedded/platform-embedded-file-writer-win.{h,cc} 生成; 修订就地改, 复跑覆盖
// COFF 风格平台写入器 (Windows)

// PlatformEmbeddedFileWriterWin (COFF 风格)。
// 符号前缀 (原版 SYMBOL_PREFIX 宏, 双分支机械展开):
// - 32 位架构 (非 x64/arm64) → "_" (符号加下划线)
// - 目标 x64 + arm64 → ""
// 汇编指令表 (原版 DirectiveAsString, V8_COMPILER_IS_MSVC 三分支):
// - MSVC 非 Arm64: kByte → `BYTE`, kLong → `DWORD`, kQuad → `QWORD` (kOcta UNREACHABLE)
// - MSVC Arm64: kByte → `DCB`, kLong → `DCDU`, kQuad → `DCQU` (kOcta UNREACHABLE)
// - 非 MSVC: kByte → `.byte`, kLong → `.long`, kQuad → `.quad`, kOcta → `.octa`
// unwind 登记: MaybeEmitUnwindData 在 V8_OS_WIN64 且 CanEmitUnwindInfoForBuiltins()
// 时发射 x64 unwind 数据; V8_OS_WIN_ARM64 下仅目标 arch 为 Arm64 时处理
// (交叉构建宿主 unwind 忽略, 仅构建期使用)。
// 实现方法面 (共 26 项): DirectiveAsString, MaybeEmitUnwindData, SectionText, SectionRoData, DeclareUint32, StartPdataSection, EndPdataSection, StartXdataSection, EndXdataSection, DeclareExternalFunction, DeclareRvaToSymbol, DeclareSymbolGlobal, AlignToCodeAlignment, AlignToDataAlignment, Comment, DeclareLabel, SourceInfo, DeclareFunctionBegin, DeclareFunctionEnd, HexLiteral, FilePrologue, DeclareExternalFilename, FileEpilogue, IndentedDataDirective, ByteChunkDataDirective, WriteByteChunk
