// 由 tool/src/snapshot/embedded/embedded-empty.js 从 v8/src/snapshot/embedded/embedded-empty.cc 生成; 修订就地改, 复跑覆盖
// 空嵌入 blob 桩

// 无内嵌数据构建桩 (原版 embedded-empty.cc, "Used for building without embedded data"):
// 四个 C 链接符号桩 (机械登记):
// - `v8_Default_embedded_blob_code_` — 单字节 {0} 或 0
// - `v8_Default_embedded_blob_code_size_` — 单字节 {0} 或 0
// - `v8_Default_embedded_blob_data_` — 单字节 {0} 或 0
// - `v8_Default_embedded_blob_data_size_` — 单字节 {0} 或 0

// 分支登记: #if V8_ENABLE_DRUMBRAKE — DrumBrake 解释器指令 handler 桩
// (FOREACH_LOAD_STORE_INSTR_HANDLER 展开为 Builtins_##name 空指针)。
