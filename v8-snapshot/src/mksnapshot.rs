// 由 tool/src/snapshot/mksnapshot.js 从 v8/src/snapshot/mksnapshot.cc 生成; 修订就地改, 复跑覆盖
// mksnapshot 工具骨架

// mksnapshot: 离线快照生成工具入口 (346 行, 语义由 Developer 落)。

// SnapshotFileWriter 方法面 (原版, 共 5 项):
// - `SetSnapshotFile`
// - `SetStartupBlobFile`
// - `WriteSnapshot`
// - `MaybeWriteStartupBlob`
// - `MaybeWriteSnapshotFile`

// 生成的嵌入文件骨架 (原版 fprintf 模板, 机械登记):
// ```c
// alignas(kPointerAlignment) static const uint8_t blob_data[] = { ... };
// static const int blob_size = <n>;
// static const v8::StartupData blob = { blob_data, blob_size };
// ```
// 嵌入链: EmbeddedData::FromBlob → EmbeddedFileWriter::WriteEmbedded
// (符号约定对齐 embedded_data.rs / embedded_file_writer)。
