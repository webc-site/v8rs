# v8-builtins-core

V8 内建（Builtins）核心元数据与调度注册表定义。

## 模块架构

1. **聚合生成管线**：
   - 核心模块由 `tool/src/builtins/builtins-core.js` 从原版 `v8/src/builtins/` 25 个源文件统一聚合生成；
   - 包含模块：
     - `kind.rs`: `BuiltinKind` 枚举（CPP, TFJ_TSA, TFJ, TFC_TSA, TFC, TFS, TFH, ASM, BCH）及类别判定（`is_cpp`, `has_js_linkage`, `is_code_stub`, `is_asm`, `is_bytecode_handler`）；
     - `table.rs`: `BUILTIN_TABLE` 1111 项全局内建定义清单、`BUILTIN_TABLE_LEN`、Tier0 判定（`is_tier0`）、检索辅助函数（`get_builtin`, `find_builtin`, `count_by_kind`）；
     - `descriptors.rs`: `descriptor_name` 接口描述符符号生成器；
     - `infra.rs`: `TQ_INFRA_DECLS` 607 项 Torque 基建类型声明；
     - `error.rs`: 强类型查询错误抽象。
2. **基线骨架说明**：
   - `builtins_definitions.rs`, `builtins_descriptors.rs`, `setup_builtins_internal.rs` 系早期单文件提取产物，其核心数据已完全整合入 `builtins-core.js` 聚合基线。