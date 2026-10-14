// 由 tool/src/wasm/pgo.h.js 从 v8/src/wasm/pgo.{h,cc} 生成; 修订就地改, 复跑覆盖
// Wasm PGO 配置信息骨架 (签名骨架, 语义面待 Developer 修订)

// 深水推迟 (readme/zh.md 台账): DumpProfileToFile / LoadProfileFromFile 依赖
// WasmModule 与 tiering-budget 面 (pgo.cc), 骨架不入挂载面。
// 另: 字段零访问 (死 getter 清理) — 结构体直暴字段, 与模块内部结构约定一致。

/// 剖析信息: 已执行函数集 + 已 tier-up 函数集 (对应原版 ProfileInformation)
/// 原版禁止拷贝; 语义实现待 Developer 修订
pub struct ProfileInformation {
  pub executed_functions: Vec<u32>,
  pub tiered_up_functions: Vec<u32>,
}
