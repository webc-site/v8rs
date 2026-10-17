# v8-execution-messages 开发日志

> 2026-10-12 收敛: 自 todo.md 看板行与 Wave 状态格迁入 (仅中文)。readme 台账由整理代理另行追加。看板 (todo.md) 只留状态。

## 看板行日志 (自 todo.md 收敛)

ESM 转换脚本 tool/src/execution/messages.js 提取 3 类核心枚举 (FrameSkipMode, StackTraceCollection, ToStringMessageSource) 与 43 项 TEMPLATES_WITH_MISMATCHED_ARGUMENTS 白名单; MessageLocation 源码位置区间与字节码偏移抽象; CallSiteFrame 对齐 call-site-info.cc 栈追踪格式化 (支持命名/匿名构造函数 at new <anonymous>、eval 帧、WASM 帧与零临时堆分配 append_location); MessageFormatter 实现 V8 原版占位符顺序消费、%% 模板字符串转义、白名单参数不足安全回退 undefined、强类型错误校验与 format 优雅容错降级; 顶层 `#![forbid(unsafe_code)]` 100% Safe Rust; 单元测试 4/4 绿, clippy 0 警告; Round 2、Round 3、Round 4 连续 3 轮独立终验审查「0 缺陷 / 0 遗漏 / 4/4 测试全绿」（clean 3/3 达成）；终验收敛冻结。
