# v8-tasks 开发日志

> 2026-10-12 收敛: 自 readme/{zh,en}.md 台账迁入 (仅中文)。看板 (todo.md) 只留状态。

## readme 台账 (2026-10-12 自 readme/{zh,en}.md 收敛)

### 终验复核 (2026-10-11, 第 5 轮独立审查)

配额 `CARGO_TARGET_DIR=target/agent_tasks_rev5`，三件套独立实跑：

- `cargo check -p v8-tasks --all-features --all-targets` → 0 错
- `cargo clippy -p v8-tasks --all-features --all-targets -- -D warnings` → 0 警告
- `cargo nextest run -p v8-tasks --all-features` → 27 tests run: 27 passed, 0 skipped
  （cancelable_tasks 17 + task_utils 4 + operations_barrier 5 + main.rs 接线冒烟 1）

### 终验裁定冻结 (2026-10-11, 第 6 轮独立终验审查, clean 3/3 达成)

配额 `CARGO_TARGET_DIR=target/agent_tasks_rev6`，三件套独立实跑：

- `cargo check -p v8-tasks --all-features --all-targets` → 0 错
- `cargo clippy -p v8-tasks --all-features --all-targets -- -D warnings` → 0 警告
- `cargo nextest run -p v8-tasks --no-fail-fast` → 27 tests run: 27 passed, 0 skipped（10 轮压测 0 flake）

覆盖核对（对照本 checkout `v8/test/unittests/tasks/`）：

- cancelable-tasks-unittest.cc 实测 11 个 TEST_F，11/11 同名迁移，另 6 个契约补充用例（SPEC 12-15a）；
- background-compile-task-unittest.cc 整文件跳过维持（依赖 Isolate/Compiler 等 L3+ 引擎层）；
- operations-barrier / task-utils 契约提取规约维持（SPEC 16-25）；
- 27 条 SPEC 与 27 个测试 1:1 对齐，断言强度守恒。

src 侧 panic 纪律复核：`panic!/unwrap()/expect()/unreachable!/todo!` 在 src/ 0 命中；unsafe 块全部具备完备 SAFETY 证明；0 dead code；0 allow 绕过。

### en.md 独有信息补记

- en.md 终验段独有: 早期登记 cancelable-tasks-unittest.cc「10 用例」系计数错误, 第 5 轮终验复核实为 11 个 TEST_F, 全部 1:1 迁移 (另 4 笔契约补充); 第 5 轮终验通过数原记录 25/25 (后扩至 27)。(出处: readme/en.md Final acceptance recheck 段, 2026-10-11)
