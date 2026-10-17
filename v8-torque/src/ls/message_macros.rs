// 由 tool/src/torque/ls.js 从 v8/src/torque/ls/message-macros.h 生成; 修订就地改, 复跑覆盖
// LS 消息访问器宏 (登记占位)

// 原版 message-macros.h 定义 FIELD_ACCESSOR/ACCESSOR_SET/ACCESSOR_GET 等 X-macro,
// 为 message.h 的 JSON 访问器类批量生成 set/get 方法。Rust 版 message.rs 直接以
// JsonValue 树 + 结构体字段承载, 宏无对应物; 保留本模块登记原头职责。
