# v8-base 开发日志

> 2026-10-12 收敛: 自 readme/{zh,en}.md 台账迁入 (仅中文)。看板 (todo.md) 只留状态。

## readme 台账 (2026-10-12 自 readme/{zh,en}.md 收敛)

### 已核对要点 (A2 数值族核对/修复记录, 自 readme/zh.md「A2 数值族」迁出)

注: 原节中 ieee754 / division_by_constant / bignum→strings 三条为终态等价与
推迟/偏差登记, 仍留 readme/zh.md; 以下两条为修复史与核对过程叙事, 迁入本文件。

- **float16** — `from_float32`/`to_float32` 按 fp16_ieee RNE 转写; 修正 `to_float32` 符号位映射 (half bit15 → float32 bit31, 掩码后整体左移 16).
- **numbers/*** — cached_powers 87 项表 + 两个 cache 函数与原版逐字节一致; diy_fp/bignum 核心 (Chunk 乘加进位、Compare、PlusCompare、Clamp、Align、SubtractTimes、Square、MultiplyByPowerOfTen 的 FIVE_27/FIVE_13、BignumToFixed 精度) 逐条核对; fast_dtoa 的 libdivide `DIV` 表 (9 组 magic) 与 BiggestPowerTen 分组界 (30/27/24/20/17/14/10/7/4/1) 核对; bignum_dtoa 的 `FixupMultiply10`/`GenerateShortestDigits`/`GenerateCountedDigits`/`BignumToFixed`/`InitialScaledStartValues*` 核对; dtoa/strtod 调度与 Grisu/Bignum 兜底转写; `fixup_multiply_10` 出参语义与原版一致.

(en.md 仅 1 字节空壳, 无英文台账可迁。)

- [轮7 W7A1] 收笼结果: 红态修复已核 E0432 解除 (checked_math.rs 挂载版完整实现 Checked/CheckedOps 0 todo; safe_conversions 失效 import 移除; numerics.rs 挂载两者); skel 0 件新增收编 (文件表 11 主目标 checked/safe_conversions/clamped/overflowing/threaded_list/small_vector/page_allocator/region_allocator/bit_field/random/atomicops 均已 .rs 落地, 全仓 0 真实 todo!); 剩余 25 headless skel + 11 子目录 skel 归类为已折入 .rs 6 件 (function_ref/bind_internal→functional, bits_cc→bits, ieee754_cc→ieee754, logging_cc→logging, basic_ops_impl→byte_conversions) 与平台/模板无 Rust 对应物跳过 (cpu_*/stack_trace_*/win32/ios/qnx/ubsan/base_export/export_template/ostream_operators/integral_constant_like); 门禁 351/351 绿。
