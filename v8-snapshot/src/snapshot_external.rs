// 由 tool/src/snapshot/snapshot-external.js 从 v8/src/snapshot/snapshot-external.cc 生成; 修订就地改, 复跑覆盖
// 外部快照加载

// 外部快照构建桩 (原版 snapshot-external.cc, 仅 V8_USE_EXTERNAL_STARTUP_DATA
// 构建编译, 否则 #error)。
// 函数面 (共 3 条):
// - `SetSnapshotFromFile`
// - `ShouldVerifyChecksum`
// - `DefaultSnapshotBlob`

// 契约: external_startup_blob 全局单槽 + LazyMutex 互斥;
// SetSnapshotFromFile 只允许装载一次 (重复装载 DCHECK);
// 分支登记: #ifdef V8_TARGET_OS_ANDROID x3 — Android 上外部快照
// 校验和每进程最多核验一次 (external_startup_checksum_verified 单次闸)。
