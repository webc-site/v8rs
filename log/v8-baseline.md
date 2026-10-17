# v8-baseline 开发日志

> 2026-10-12 收敛: 自 todo.md 看板行与 Wave 状态格迁入 (仅中文)。readme 台账由整理代理另行追加。看板 (todo.md) 只留状态。

## 看板行日志 (自 todo.md 收敛)

9 个 ESM 工具脚本完整生成；Baseline 入口三函数 (ENTRY_SIGNATURES: CanCompileWithBaseline, GenerateBaselineCode, EmitReturnBaseline) 与批编译器 (BaselineBatchCompiler 22 条声明, INITIAL_QUEUE_SIZE=32)；Baseline 汇编器 (BaselineAssembler 99 条声明, EnsureAccumulatorPreservedScope 5 条声明, CompressionMode 及谓词, 14 项非目标架构跳过清单)；Baseline 编译器 (BaselineCompiler 76 条声明, StackCheckBehavior 及谓词, BuiltinCallJumpMode 及谓词, BytecodeOffsetTableBuilder 4 条声明, CompareBranchCandidate 2 条声明, EffectState 6 条声明, SaveAccumulatorScope 4 条声明, VISIT_BYTECODE_METHODS 217 个访问器全表, VISIT_INTRINSIC_METHODS 17 个内置函数全表)；字节码偏移迭代器 (BytecodeOffsetIterator 25 条声明)；x64 与 arm64 架构特化 (x64: SCRATCH_REGISTERS 5 项, NUM_SCRATCH_REGISTERS=5, X64_SCRATCH_REGISTER_SCOPE_DECLARATIONS 6 条, X64_FALLBACK_BUILTIN_CALL_JUMP_MODE_FOR_BASELINE, LOOP_UNROLL_SIZE=8; arm64: ARM64_SCRATCH_REGISTER_SCOPE_DECLARATIONS 6 条, INSTRUCTIONS_PER_JUMP_TARGET=1, INSTRUCTIONS_PER_LABEL=2, HELPER_TEMPLATE_COUNT=11, ARM64_FALLBACK_BUILTIN_CALL_JUMP_MODE_FOR_BASELINE)；顶层 `#![forbid(unsafe_code)]` 100% Safe Rust；单元测试 6/6 绿, clippy 0 警告；Round 1、Round 2、Round 3 连续 3 轮独立终验对抗审查「0 缺陷 / 0 遗漏 / 6/6 测试全绿」（clean 3/3 达成）；终验收敛冻结。
