# v8-builtins-misc 开发日志

> 2026-10-12 收敛: 自 todo.md 看板行与 Wave 状态格迁入 (仅中文)。readme 台账由整理代理另行追加。看板 (todo.md) 只留状态。

## 看板行日志 (自 todo.md 收敛)

模块划分扁平导出 (number 30, object_func 103, data_time_json 165, system 274, wasm 9, 合计 581 项内建全量映射); 581 项零成本 pub const fn 工厂函数; 谓词分类精准对齐 (NumberBuiltin: is_bigint 10, is_number_prototype 4, is_math 1, is_baseline_or_feedback 10, is_trampoline 2; ObjectFuncBuiltin: is_callsite 22, is_constructor 18, is_function_prototype 6, is_error 4, is_baseline_or_feedback 11; DateTimeJsonBuiltin: is_date 51, is_intl 98, is_temporal 8, is_json 5, is_console 5; SystemBuiltin: is_ic 85, is_trampoline 22, is_call 37, is_interpreter 8, is_shadow_realm 6, is_atomics 9; WasmBuiltin: is_conversion 3, is_wrapper 4, is_deopt 1); global_id 与 descriptor_name 成功关联至 v8-builtins-core 全局注册表; 408 项 Torque 元数据声明 (NUMBER 159, OBJECT_FUNC 30, DATA_TIME_JSON 1, SYSTEM 27, WASM 191); ESM 生成脚本 tool/src/builtins/builtins-misc.js 100% 幂等; 强类型 Error 枚举与 Result; 顶层 `#![forbid(unsafe_code)]` 100% Safe Rust; 单元测试 6/6 绿, clippy 0 警告; Round 2、Round 3、Round 4 连续 3 轮独立终验审查「0 缺陷 / 0 遗漏 / 6/6 测试全绿」（clean 3/3 达成）；终验收敛冻结。
