# v8-regexp-exec 开发日志

> 2026-10-12 收敛: 自 todo.md 看板行与 Wave 状态格迁入 (仅中文)。readme 台账由整理代理另行追加。看板 (todo.md) 只留状态。

## 看板行日志 (自 todo.md 收敛)

RegExpInterpreter 61 条字节码分发闭环与 4 字节自身/块对齐算法、回溯栈 1KB 初始 + 动态 64MB 硬上限、Latin-1 与 UC16 编码和反向引用; RegExpStack 线程局部归档与恢复协议; ExperimentalNfaInterpreter 线程优先级调度与 pc 去重、EXP-3b/c/d 各批段时钟与双轨（lookbehind_table 优化轨 / lookaround_table 主轨）环视真值表构建读写; 彻底消除全部 allow 绕过与裸 unwrap/expect/panic; 单元测试 192/192 绿, clippy 0 警告; Round 1、Round 2、Round 3 连续 3 轮独立终验审查「0 缺陷 / 0 遗漏 / 192/192 测试全绿」（clean 3/3 达成）；终验收敛冻结。

## readme 台账 (2026-10-12 自 readme/{zh,en}.md 收敛)

### RegExpExecInternal 票 (todo 原登记「桩实现」) 闭环记录 — 2026-10-11

#### 落地/修订面
- `tests/regexp_interpreter_p3b_unittest.rs`: CheckSpecialClassRanges 操作数改转义字母编码 (6 处)。根因实证: C++ `regexp-ast.h:102-112` `StandardCharacterSet : char` 值即字母 ('d'=100/'w'=119/'W'=87/'n'=110/'*'=42)，生成器 `regexp-bytecode-generator.cc:354` 直落 char；旧测试误用枚举判别序号。src 侧审计 0 改动。
- `tests/regexp_interpreter_unittest.rs`: 旧「未落地台账」改写为落地台账 `p3c_p3d_handlers_landed_ledger` (16 项逐名 cc 行锚, 零删项; 正向断言 !NotImplemented/pc 前进/StepLimitExceeded/counts/%4)。
- `tests/regexp_interpreter_p3c_p3d_unittest.rs` (新建 16 例): 逐 handler 对 `regexp-interpreter.cc` 880/894/908/922/937/952/967/975/983 + 1010/1023/1036/1049/1066/1086/1111。
- `tool/src/regexp/regexp-interpreter.cc.js`: 双行号台账 457 条对数, 复跑 skel md5 幂等。
- P-3c-icu 4 臂显式推迟 (`NotImplemented{cc_line:46}`): cc:46-61 uc16 NoCase 走 `RegExpMacroAssembler::CaseInsensitiveCompare` (Isolate/ICU 硬耦合)。

#### 只读审计轮 (全新独立复审) + 主控代修
- 审计结论: 本票「缺陷在测试侧」成立非假绿; 台账零删项属实; 16 例顶层行锚全准; 脚本 457 实跑幂等; src/regexp_interpreter.rs 实质零改动。
- **D2 修复 (真语义缺陷, init 批 REG-1b1 遗留)**: `regexp_interpreter.rs` backref Latin1 NoCase 核 cc:75-76 原版 unsigned 减法回绕, Rust 直减 debug 构建下流 panic → 改 `wrapping_sub`; 回归测试 `backref_no_case_latin1_underflow_regression` ({/[→0x7B、\x01/!→0x21 两对最小复现)。
- **D1 修复 (文档级行锚漂移)**: rs 4 处 (888→889, 1092→1090, 1094→1092, 1100→1098) + p3c_p3d 档 7 处 (882→883, 888→889, 1100→1098) + unittest 档 2 处 (882→883, 1092→1090)，均按本树实测 cc 行号修正。

#### 门禁实测
- nextest --no-fail-fast: 191 例 189 绿; 2 红 (`sched_find_next_match_lookaround_priority`, `run_active_thread_batch_routing`) 均属并发 EXP 席在飞面, 非本票。
- clippy -D warnings RC=0; fmt 差仅在 `experimental/interpreter.rs` (并发席在写), 本票四档零差。

#### 推迟面汇总
- P-1/P-2/P-4 入口与 Isolate 耦合面; P-3c-icu 4 臂 (ICU CaseInsensitiveCompare)。

(注: en.md 为本节英文镜像, 无 zh.md 之外的独有信息, 未迁入。)

## 轮7 W7LIT (零头席)

- [轮7 W7LIT] grep `todo!` 命中 3 → 实测编译面真 todo 0/0: 三处 (`regexp_interpreter.rs:1864`, `experimental/interpreter.rs:102/1560`) 均为「禁 todo!/禁 panic 伪装」红线说明文字, 非 todo 宏调用, 无填体目标。既有推迟面 (P-3c-icu 等) 维持原显式失败通道 `deferred_arm`, 不改动。门禁实测 211/211 全绿 (含本 crate)。
