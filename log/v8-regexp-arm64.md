# v8-regexp-arm64 开发日志

> 2026-10-12 收敛: 自 todo.md 看板行与 Wave 状态格迁入 (仅中文)。readme 台账由整理代理另行追加。看板 (todo.md) 只留状态。

## 看板行日志 (自 todo.md 收敛)

ARM64 栈帧布局 (帧指针 0、返回地址 8、FrameType -8、栈局部填充 -80、首个寄存器 -84、首个捕获 -88)、10 个被调用者保存寄存器 (x19-x28)、10 个专用寄存器映射 (current_input_offset=w21 .. regexp_stack=x19)、16 缓存寄存器与 1024 缓冲尺寸落地; ESM 转换脚本 tool/src/regexp/arm64/regexp-macro-assembler-arm64.h.js 幂等且与源码双向 100% 一致; 顶层 `#![forbid(unsafe_code)]` 100% Safe Rust; 单元测试 2/2 绿, clippy 0 警告; Round 1、Round 2、Round 3 连续 3 轮独立终验审查「0 缺陷 / 0 遗漏 / 2/2 测试全绿」（clean 3/3 达成）；终验收敛冻结。
