// 由 tool/src/snapshot/snapshot-empty.js 从 v8/src/snapshot/snapshot-empty.cc 生成; 修订就地改, 复跑覆盖
// 空快照桩 (无快照构建)

// 空快照构建桩 (原版 snapshot-empty.cc, "Used for building without snapshots"):
// 桩函数面 (共 6 条):
// - `SetNativesFromFile` — UNREACHABLE 或返回空/null
// - `SetSnapshotFromFile` — UNREACHABLE 或返回空/null
// - `ReadNatives` — UNREACHABLE 或返回空/null
// - `DisposeNatives` — UNREACHABLE 或返回空/null
// - `DefaultSnapshotBlob` — UNREACHABLE 或返回空/null
// - `ShouldVerifyChecksum` — UNREACHABLE 或返回空/null

// 分支登记: #ifdef V8_USE_EXTERNAL_STARTUP_DATA x1 — 该配置下
// SetNativesFromFile/SetSnapshotFromFile 为 UNREACHABLE 桩;
// DefaultSnapshotBlob 恒空, ShouldVerifyChecksum 恒 false。
