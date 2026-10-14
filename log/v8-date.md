# v8-date 开发日志

> 2026-10-12 收敛: 自 readme/{zh,en}.md 台账迁入 (仅中文)。看板 (todo.md) 只留状态。

## readme 台账 (2026-10-12 自 readme/{zh,en}.md 收敛)

### 门禁实测 (自 readme/zh.md「测试映射」节末行迁入)

门禁实测: cargo check 0 错 / clippy -D warnings 0 警告 / nextest 26 通过 0 失败
(debug+release 双跑, 含 round2 差分 6 用例)。

en.md 同位置为旧数字 (nextest 20 passed 0 failed), 且其测试映射表缺 review_round2_diff
一行 — 判为过期快照; 英文原文不搬, 无其他独有信息。
