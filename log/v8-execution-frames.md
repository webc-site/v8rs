# v8-execution-frames 开发日志

> 2026-10-12 收敛: 自 todo.md 看板行与 Wave 状态格迁入 (仅中文)。readme 台账由整理代理另行追加。看板 (todo.md) 只留状态。

## 看板行日志 (自 todo.md 收敛)

4 个 ESM 转换脚本全部跑通且 0 源码漂移; StackFrameType 34 项具体帧全量对齐 (23 项标准帧 + 11 项 wasm 帧) 与 NoFrameType/NumberOfTypes/Manual 哨兵; 严格下转型谓词 (is_javascript 4 种/is_wasm 2 种/is_exit 1 种) 与广义家族分类谓词 (is_js_family/is_wasm_family/is_exit_family) 正交分立; JsFrameMarker Smi Tag 编码对齐 (Inner=0, Outermost=2) 彻底排除 GC 栈扫描指针误判崩溃; StandardFrameConstants 修复槽位碰撞 (ARG_C_OFFSET=-24, EXPRESSIONS_OFFSET=-32, FIRST_PUSHED_FRAME_VALUE_OFFSET=-32); x64 与 arm64 架构帧常量表及 static_assert 转译通过; 顶层 `#![forbid(unsafe_code)]` 100% Safe Rust; 单元测试 4/4 绿, clippy 0 警告; Round 2、Round 3、Round 4 连续 3 轮独立终验审查「0 缺陷 / 0 遗漏 / 4/4 测试全绿」（clean 3/3 达成）；终验收敛冻结。
