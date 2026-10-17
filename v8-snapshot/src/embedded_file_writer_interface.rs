// 由 tool/src/snapshot/embedded/embedded-file-writer-interface.js 从 v8/src/snapshot/embedded/embedded-file-writer-interface.h 生成; 修订就地改, 复跑覆盖
// 嵌入文件写入器接口

/// 默认嵌入变体名 (原版 kDefaultEmbeddedVariant)
pub const DEFAULT_EMBEDDED_VARIANT: &str = "Default";

/// v8: src/snapshot/embedded/embedded-file-writer-interface.h EmbeddedFileWriterInterface
/// EmbeddedFileWriterInterface (原版纯虚接口, isolate 编译期内建源码位置登记):
pub trait EmbeddedFileWriterInterface {
  // 原版: `LookupOrAddExternallyCompiledFilename`
  // 原版: `GetExternallyCompiledFilename`
  // 原版: `GetExternallyCompiledFilenameCount`
  // 原版: `PrepareBuiltinSourcePositionMap`
  // 原版: `SetBuiltinUnwindData`
  fn lookup_or_add_externally_compiled_filename(&mut self, filename: &str) -> i32;
  fn get_externally_compiled_filename(&self, index: i32) -> &str;
  fn get_externally_compiled_filename_count(&self) -> i32;
  // isolate 在以内建跳板替换已编译 InstructionStream 前调用
  fn prepare_builtin_source_position_map(&mut self);
}

// LabelInfo (原版 struct): 偏移 + 符号名, 由 Developer 落型。
// 分支登记: #if defined(V8_OS_WIN64) — SetBuiltinUnwindData 仅 Win64 构建。
