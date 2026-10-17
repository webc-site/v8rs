# v8-builtins-core 开发日志

> 2026-10-12 收敛: 自 todo.md 看板行与 Wave 状态格迁入 (仅中文)。readme 台账由整理代理另行追加。看板 (todo.md) 只留状态。

## 看板行日志 (自 todo.md 收敛)

ESM 转换脚本 tool/src/builtins/builtins-core.js 聚合 25 个原版源文件 100% 幂等生成; BuiltinKind 9 类内建实现 (CPP, TFJ_TSA, TFJ, TFC_TSA, TFC, TFS, TFH, ASM, BCH) 及其分类谓词与 strum 双向映射; BUILTIN_TABLE 1111 项全局内建定义清单 (kind, name, line) 及 BUILTIN_TABLE_LEN、表首项 DeoptimizationEntry_Eager (ASM, line 77)、表末项 StringPrototypeToUpperCase (CPP, line 2020); Tier0 核心内建名单与 is_tier0 边界判定; 快速检索与频次统计 (get_builtin, find_builtin, count_by_kind); descriptor_name 接口描述符生成; TQ_INFRA_DECLS 607 项 Torque 基建类型声明; 强类型 Error 枚举与 Result; 顶层 `#![forbid(unsafe_code)]` 100% Safe Rust; 单元测试 7/7 绿, clippy 0 警告; Round 1、Round 2、Round 3 连续 3 轮独立终验审查「0 缺陷 / 0 遗漏 / 7/7 测试全绿」（clean 3/3 达成）；终验收敛冻结。
