# v8-regexp-x64 开发日志

> 2026-10-12 收敛: 自 todo.md 看板行与 Wave 状态格迁入 (仅中文)。readme 台账由整理代理另行追加。看板 (todo.md) 只留状态。

## 看板行日志 (自 todo.md 收敛)

双 ABI (SysV 2 callee-save / Win 4 callee-save) 栈帧偏移布局、本地变量 (SuccessfulCaptures, StringStartMinusOne, BacktrackCount, RegExpStackBasePointer, RegisterZero)、寄存器寻址函数 register_location_offset(index)、8 项专用寄存器映射与 1024 缓冲尺寸落地; ESM 转换脚本 tool/src/regexp/x64/regexp-macro-assembler-x64.h.js 幂等且与源码双向 100% 一致; 顶层 `#![forbid(unsafe_code)]` 100% Safe Rust; 单元测试 3/3 绿, clippy 0 警告; Round 2、Round 3、Round 4 连续 3 轮独立终验审查「0 缺陷 / 0 遗漏 / 3/3 测试全绿」（clean 3/3 达成）；终验收敛冻结。
