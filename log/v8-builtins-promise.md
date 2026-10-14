# v8-builtins-promise 开发日志

> 2026-10-12 收敛: 自 todo.md 看板行与 Wave 状态格迁入 (仅中文)。readme 台账由整理代理另行追加。看板 (todo.md) 只留状态。

## 看板行日志 (自 todo.md 收敛)

PromiseState 状态机三态 (Pending, Fulfilled, Rejected) 与 is_pending/is_settled 判定; PromiseHookType 4 级钩子; PromiseBuiltin 34 项核心内建枚举 (PromiseAll, PromiseRace, PromisePrototypeThen 等) 与 34 个零成本工厂函数; 语义分类谓词 (is_combinator 4 项, is_prototype_method 3 项, is_reaction_job 3 项, is_constructor_or_capability 4 项); PROMISE_TQ_DECLS 80 项 Torque 元数据; AwaitBehavior 枚举 (NORMAL, RESUME_ONLY); AsyncBuiltin 56 项异步/生成器/资源栈/微任务内建 (含 ResumeGeneratorTrampoline_WithCatch 序列化) 与 56 个工厂函数; 语义分类谓词 (is_async_function 7 项, is_async_generator 15 项, is_generator 6 项, is_disposable_stack 17 项, is_microtask 3 项); ASYNC_TQ_DECLS 12 项元数据; IteratorBuiltin 14 项迭代器与去优化延续栈帧内建及工厂函数; 语义分类谓词 (is_deopt_continuation 6 项, is_iterable_to_list 8 项); ITERATOR_TQ_DECLS 27 项元数据; 私有子模块与顶层纯扁平 pub use 导出杜绝二次导出; 强类型 Error 枚举与 Result; 顶层 `#![forbid(unsafe_code)]` 100% Safe Rust; 单元测试 5/5 绿, clippy 0 警告; Round 1、Round 2、Round 3 连续 3 轮独立终验审查「0 缺陷 / 0 遗漏 / 5/5 测试全绿」（clean 3/3 达成）；终验收敛冻结。
