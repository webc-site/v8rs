# v8-runtime 开发日志

> 2026-10-12 收敛: 自 todo.md 看板行与 Wave 状态格迁入 (仅中文)。readme 台账由整理代理另行追加。看板 (todo.md) 只留状态。

## 看板行日志 (自 todo.md 收敛)

全量运行时名表总表（table.rs 714 项全量对齐 V8 原版 runtime.h 中全部 F、I、IF_* 宏条目与 RUNTIME_FUNCTIONS 描述符表、RuntimeFunctionId 714 项 + 1 项 NumFunctions 守尾共 715 个大驼峰枚举变体与 strum 别名、RuntimeCallProperty、OptimizationStatus 8 项、DebugPrintValueType 3 项、ObjectPair 双 64 位返回值约定结构体、RuntimeEnv 窄 trait）; numbers 组 64 项基础操作运行时枚举与 6 大语义谓词（is_numbers, is_strings, is_bigint, is_atomics, is_date, is_operators）; object 组 123 项对象与集合运行时枚举与 6 大语义谓词（is_object, is_array, is_collections, is_typedarray, is_classes, is_proxy）; misc 组 294 项杂项运行时枚举与 18 大类语义谓词（scopes, module, debug, compiler, internal, wasm, function, generator, promise, regexp, symbol, literals, forin, futex, intl, shadow_realm, trace, weak_refs）; 各组函数工厂函数 100% 覆盖且与 table.rs 实现 descriptor/function_id 双向互查 0 缺失; 4 个 ESM 针校验脚本（table.js, numbers.js, object.js, misc.js）全部 100% 通过; 私有子模块与顶层纯扁平 pub use 导出杜绝二次导出; 强类型 Error 枚举与 Result; 顶层 `#![forbid(unsafe_code)]` 100% Safe Rust; 单元测试 6/6 绿, clippy 0 警告; Round 3、Round 4、Round 5 连续 3 轮独立终验审查「0 缺陷 / 0 遗漏 / 6/6 测试全绿」（clean 3/3 达成）；终验收敛冻结。
