# v8-common 开发日志

> 2026-10-12 收敛: 自 readme/{zh,en}.md 台账迁入 (仅中文)。看板 (todo.md) 只留状态。

## readme 台账 (2026-10-12 自 readme/{zh,en}.md 收敛)

### round3 全量程序化核验 (脚本一次性执行, 结论固化于 tests)

- message-template: 原版 MESSAGE_TEMPLATES 498 条 ↔ Rust enum 变体序 + MESSAGE_TEMPLATE_STRINGS 全表, 0 名错位 / 0 串错位 / 哨兵一致 / 条件串双分支一致; 全表 FNV-1a-64 金标独立复算 = tests/message_template.rs 固化值
- globals.h: 491 常量/枚举名全量 diff — 显式值级对齐 354; 值差 28 条与缺失 15 条逐条人工核销 (脚本 X-macro 未展开 / 嵌套类成员 / 跨枚举重名 / 条件分支解析盲区), 真实缺陷 0 / 真实遗漏 0; 反馈枚举 (BinaryOperationFeedback 12 值 / CompareOperationFeedback 17 值 / TypeOfFeedback 7 值) 逐值人工对齐
- assert-scope: 嵌套 old_data 配对 / Release 幂等 / thread_local 隔离逐条对照 assert-scope.cc:31-61; 跨线程允许面差异见 round3 F7

(en.md 无日志内容, 无独有信息补记。)
