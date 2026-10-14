# v8-tasks 测试规约清单 (7.3 规约先行)

来源: `v8/test/unittests/tasks/cancelable-tasks-unittest.cc` (TEST_F CancelableTaskManagerTest)。
背景: `background-compile-task-unittest.cc` 依赖完整引擎 (Isolate/Compiler/flags)，属 L3+ 层，
本 crate 无法承载，整文件跳过，理由登记 `readme/zh.md`。
原版无 operations-barrier 单测；`ops_barrier` 一节规约从 `operations-barrier.h` 契约注释提取。

## fixture (对照)
- Fixture: 持有 manager + `cancel_triggered` 原子位; `CancelAndWait/TryAbortAll` 先置触发位再调管理器
- TestTask 继承 Task+Cancelable; 模式 kDoNothing / kWaitTillCancelTriggered / kCheckNotRun
- kCheckNotRun: 原 `EXPECT_TRUE(false)` 若被执行 → Rust 以 `ran` 标志位 + `assert!(!ran)` 等价承接 (产品测试禁 panic 噪声)
- SequentialRunner / ThreadedRunner(base::Thread) → Option 包装 / v8_base_platform::Thread
- 原版忙等 `while(result==0)` 属事件同步(自旋于原子进度量), 非墙钟依赖, 保留 (7.4)

## 规约 (每条一行: 编号 | 原断言 | 断言强度)

### cancelable_tasks.rs
1. empty_cancelable_task_manager: 空管理器 CancelAndWait 直接返回 (无断言, 不挂死即通过)
2. sequential_cancel_and_wait: result1==0 初值; CancelAndWait 后 result1==0; Run 后 result1==0 且 body 未执行
3. sequential_multiple_tasks: id1==1, id2==2; runner1.Run 后 result1==1; runner2.Run 后 result2==2; CancelAndWait 后 TryAbort(1)==kTaskRemoved, TryAbort(2)==kTaskRemoved
4. threaded_multiple_tasks_started: 两任务并发执行, 均置 result 后 CancelAndWait 收尾(等 running 任务); result1==1, result2==2
5. threaded_multiple_tasks_not_run: 先 CancelAndWait(取消 waiting 任务); 线程 Start+Join 后 result==0 且不执行 body
6. remove_before_cancel_and_wait: id==1; TryAbort(id)==kTaskAborted; Start+Join; CancelAndWait; result1==0
7. remove_after_cancel_and_wait: id==1; Start+Join 后 CancelAndWait; TryAbort(id)==kTaskRemoved; result1==1
8. remove_unmanaged_id: 未注册 id 1/2/3 TryAbort 均为 kTaskRemoved (CancelAndWait 前后同)
9. empty_try_abort_all: 空管理器 TryAbortAll()==kTaskRemoved; CancelAndWait 收尾
10. threaded_multiple_tasks_not_run_try_abort_all: TryAbortAll()==kTaskAborted; Start+Join; result==0 不执行; CancelAndWait
11. threaded_multiple_tasks_started_try_abort_all: task1 运行中(忙等触发位), TryAbortAll()∈{kTaskAborted,kTaskRunning}(原版 AnyOf); task2 未启动被取消: result1==1, result2==0; CancelAndWait
12. register_after_cancel_and_wait (cancelable-task.h:41-42 契约注释): CancelAndWait 后 Register → 任务立即被取消, id==kInvalidTaskId(0), Run 不执行
13. try_abort_invalid_id (CHECK_NE 改写): TryAbort(0) → Err(InvalidTaskId)
13a. task_id_counter_overflow (CHECK_NE 回绕防御): 计数器达到 u64::MAX 钉死并返回 Err(TaskIdOverflow), 杜绝 ABA 与算术溢出 panic
14. cancelable_task_run_guard: CancelableTask::run 未取消时执行 body 一次; 取消后 run 不执行 (对应 Run() final { if TryRun() })
15. cancelable_idle_task_deadline: CancelableIdleTask::run(deadline) 传递 deadline 参数给 body (IdleTask 接口契约)
15a. compare_exchange_previous_semantics (cancelable-task.h:128-136 `*previous` 契约): CAS 成功时 previous 保持交换前传入值; 失败时取回实际观察值

### task_utils.rs (task-utils.{h,cc} 语义: 闭包包装)
16. make_cancelable_task_runs: 未取消 → run 执行闭包副作用
17. make_cancelable_task_canceled: 管理器 CancelAndWait → run 不执行闭包
18. make_cancelable_idle_task_runs: 未取消 → run(deadline) 执行且收到 deadline 值
19. make_cancelable_task_id: 返回任务 id==递增登记号 (Register 折叠进构造的原版行为)

### operations_barrier.rs (契约提取, 原版无单测)
20. try_lock_before_cancel: CancelAndWait 前 TryLock 得有效 Token (operator bool==true)
21. token_drop_decrements: Token 析构即宣告完成; 多 Token 后全部析构, CancelAndWait 不挂死
22. cancel_and_wait_blocks: 持 Token 时另一线程 CancelAndWait 阻塞, Token 释放后被唤醒返回
23. try_lock_after_cancel: CancelAndWait 后 TryLock 得无效 Token (bool==false)
24. cancelled_accessor: cancel 前 cancelled()==false, 后为 true
25. token_move_assign: `a = b` 释放 a 旧引用不双重释放; 后续 CancelAndWait 正常收敛 (对应 operator=)
