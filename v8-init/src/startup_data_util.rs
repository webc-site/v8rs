// 由 tool/src/init/startup-data-util.js 从 v8/src/init/startup-data-util.{h,cc} 生成; 修订就地改, 复跑覆盖
// 外部启动快照数据装载辅助

/// 独立二进制 (d8/cctest/unittest) 的外部启动数据装载便利层;
/// 嵌入方通常自管启动数据。整文件条件编译于 V8_USE_EXTERNAL_STARTUP_DATA。
/// 公开 API 签名骨架 (原版 startup-data-util.h):
/// - `void InitializeExternalStartupData(const char* directory_path)`
/// - `void InitializeExternalStartupDataFromFile(const char* snapshot_blob)`

/// cc 内部辅助 (机械提取函数名): ClearStartupData, DeleteStartupData, FreeStartupData, Load, LoadFromFile, InitializeExternalStartupData, InitializeExternalStartupDataFromFile
/// 条件分支登记: #ifdef V8_USE_EXTERNAL_STARTUP_DATA x3;
/// 文件读取契约: fseek 定尾取 raw_size → fread 读入 → 不足即 Free 并置空。
