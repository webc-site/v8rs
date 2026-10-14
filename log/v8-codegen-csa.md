# v8-codegen-csa 开发日志

> 2026-10-12 收敛: 自 todo.md 看板行与 Wave 状态格迁入 (仅中文)。readme 台账由整理代理另行追加。看板 (todo.md) 只留状态。

## 看板行日志 (自 todo.md 收敛)

3 个 ESM 转换脚本 tool/src/codegen/csa/{tnode.js, code-assembler.js, code-stub-assembler.js} 100% 幂等生成; 强类型 TNode<T> 与 21 项 Untagged 基本机器类型标记及 18 项 Tagged 对象类型标记落地, 编译期 size_of::<TNode<T>>() == 4 断言; CodeAssembler 包含 31 项 CompareBinaryOp (COUNT=31, 浮点/有符号/无符号比较谓词)、65 项 BinaryOp (COUNT=65, 位运算/算术谓词)、71 项 UnaryOp (COUNT=71, bitcast/转换谓词), CodeAssemblerLabel 状态机 (Unbound, Bound, Merged) 与 bind 唯一性守卫, CodeAssemblerVariable 局部变量管理, CodeAssemblerState 图状态记录, 常量/二元/一元/控制流/调用接口完备; CodeStubAssembler 包含 AllocationFlag/ExtractFixedArrayFlag/SlackTrackingMode/HoleConversionMode/PrimitiveType 枚举与位集, Smi 原语 (tagged_is_smi, tagged_not_smi, smi_tag, smi_untag, smi_to_int32, smi_add, smi_sub, smi_equal, smi_less_than 等), 堆对象与 Map 访问原语, 快速类型谓词 (is_js_receiver, is_js_object, is_string, is_undefined, is_null), 内存分配 (allocate_raw, allocate_in_young_generation, allocate_in_old_generation) 与控制流分支原语; 纯扁平导出架构; 顶层 `#![forbid(unsafe_code)]` 100% Safe Rust; 单元测试 6/6 绿, clippy 0 警告; Round 1、Round 2、Round 3 连续 3 轮独立终验审查「0 缺陷 / 0 遗漏 / 6/6 测试全绿」（clean 3/3 达成）；终验收敛冻结。
