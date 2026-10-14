# v8-bigint 开发日志

> 2026-10-12 收敛: 自 readme/{zh,en}.md 台账迁入 (仅中文)。看板 (todo.md) 只留状态。

## readme 台账 (2026-10-12 自 readme/{zh,en}.md 收敛)

### BitwiseAnd 单元落地记录 (原小节「BitwiseAnd 单元 (台账 [bigint] 缺新算法: BigInt::BitwiseAnd, 2026-10 落地)」, 自 readme/zh.md 迁入)

digit 内核 BitwiseAnd_PosPos/NegNeg/PosNeg (bigint-inl.h:1146-1187) 早已在 bigint.rs 1:1 就位; 本轮补的是缺失的顶层编排面 `bitwise.rs` (builtins-bigint.tq:454 BigIntBitwiseAndImpl 的无堆等价: 零短路 / 符号四分支 / 长度预算 min·max+1·正侧 / NegPos 换参 / Canonicalize→norm)。脚本对数 121 条 (bitwise-and.js), 测试 6 例。6 例测试的映射表属结构性文档, 留驻 readme (小节更名「BitwiseAnd 单元测试映射」)。

### 缺口盘点与补口 (原小节「缺口盘点结论 (2026-10-11 实测复核)」, 自 readme/zh.md 迁入)

- 2026-10-11 实测复核: 原版 v8/src/bigint 16 个文件全部落地, 函数级 1:1 核对无缺口 (覆盖清单留驻 readme「覆盖盘点」一节)。
- 后续补口 (2026-10): 台账项 "缺新算法: BigInt::BitwiseAnd" — digit 内核既在, 缺的是 tq 顶层编排面 (符号派发 / 长度预算 / 换参 / 规格化), 已由 bitwise.rs 落地并配 and.js 20 组金标准; objects 三壳与 BigIntTooBig 抛错面按边界推迟登记 (推迟登记仍留 readme)。

### readme/en.md 独有信息补记

- en.md 的「BitwiseAnd unit (todo: [bigint] missing BigInt::BitwiseAnd, landed 2026-10)」小节与 zh 同源, 不搬英文原文; 叙事段 (this unit adds ...) 自 en 删除。
- en.md 独有措辞 "heap-free" 与 "exact-budget API" 已被 zh「无堆等价」「预算缓冲非自有版」覆盖, 无补充信息。
- en.md 行 "Full function-level audit (2026-10-11): no gaps — all 16 upstream files ported 1:1" 的日期化审计表述已并入上条 2026-10-11 复核记录, en 正文改为无日期陈述。
