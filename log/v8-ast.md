# v8-ast 开发日志

> 2026-10-12 收敛: 自 readme/{zh,en}.md 台账迁入 (仅中文)。看板 (todo.md) 只留状态。

## readme 台账 (2026-10-12 自 readme/{zh,en}.md 收敛)

### 阶段交付叙事 (自 readme/zh.md 导语迁入)

第一阶段 (类型层骨架) 落地节点全集、类别桥接、Visitor、值工厂、变量层与模块描述符; 第二阶段 (当前) 落地节点字段补全、工厂全构造器 + Zone 分配路径、scopes.cc 解析期语义第一阶段, src 约 5.7k 行 (含生成基线) + tests 约 1.7k 行。

注: 原导语中「第二阶段架构决策」三条 (值语义主干 / arena 句柄 / Zone 分配路径) 属设计决定, 随内容留驻 readme (标题去轮次标签改为「架构决策」); X-macro 对数锚点、语义要点/偏差登记、测试映射、门禁、第三阶段拆块建议均属结构性文档, 未迁。readme 导语已改写为「当前落地范围」陈述。

### readme/en.md 独有信息补记

- en.md 为 zh.md 英文缩略版, 其 Phase 1 / Phase 2 delivered 叙事与 zh 同源, 不搬英文原文。
- en.md 独有的用法信息「`bun tool/src/ast/ast.h.js` 等可复跑再生生成基线」已并入 readme/zh.md「文件 → 模块映射」一节, 非日志。
