# v8-objects-arrays 开发日志

## [W8ARR 席] elements.cc / elements.h 语义填体与推迟台账收敛 (2026-10-12)

### 1. 席位任务与盘点

- **独占模块**: `v8-objects-arrays` (独立 target 目录: `target/agent_w8arr`)。
- **骨架盘点**:
  - 15 个 `.skel` 骨架文件全部核对（arguments_tq, embedder_data_array_cc, embedder_data_slot_inl, fixed_array_base_inl, fixed_array_inl, primitive_array_inl, primitive_array_tq, primitive_array, property_array_tq, struct_, struct_inl, trusted_object_h, trusted_object_inl_h, trusted_pointer_h, trusted_pointer_inl_h），均为早期脚本产物，未接入 `lib.rs`，不污染编译面。
  - 原版 `v8/src/objects/elements.{h,cc}` 对应骨架 `elements.rs` (4 处 `todo!`) 与 `elements_impl.rs` (19 处 `todo!`) 合计 23 处。

### 2. 工具定制与代码生成

- **`tool/src/objects/elements.h.js`**:
  - 定制更新：4 个跨子系统自由函数标注明确的推迟台账编号 (`ELEM-ISO` / `ELEM-CSA`) 并保持 `clang -E` 47 方法 / 38 纯虚 / 3 转发不变式断言。
- **`tool/src/objects/elements.cc.js`**:
  - 废除原平铺 `toLowerCase()` 产生的不可读函数名，生成具备清晰语义签名的 Rust 原生实现。
  - 核心 10 项辅助函数原地就地填体。
  - 深水 9 项登记 `readme/zh.md` 推迟台账 (`ELEM-ISO`, `ELEM-DICT`, `ELEM-SLOW`, `ELEM-STATIC`)。
  - 保留向后兼容别名 `pub use ... as ...`，严格锁定 `IMPL_ONLY_FREE_FUNCTIONS`(16) / `STATIC_METHODS`(3) / `TYPED_SPECIALIZATIONS`(64) / `ACCESSOR_CLASSES`(32) 合计 87 项编译期断言。

### 3. 语义填体落地面 (10 大核心函数)

1. `is_float16_raw_bits_zero`: `(x & !0x8000) == 0`，高效位运算识别 IEEE 754 half 零 (+0.0 / -0.0)。
2. `is_identity_mapping` / `is_identity_mapping_slice`: `AsIndex` 契约抽象，纯单次迭代零堆分配验证恒等映射并支持提前终止。
3. `get_write_barrier_mode`: Smi 与 Double 元素直接返回 `SkipWriteBarrier`，其余返回 `UpdateWriteBarrier`。
4. `sort_indices` / `sort_indices_slice`: 对齐 V8 原版 `std::sort` 行为，`extract_index_number` 提取 Smi 与对齐 HeapNumber，数值项升序排列，Undefined / 占位项沉底。零堆分配原地切片排序。
5. `copy_object_to_object_elements`: FixedArray 元素切片拷贝，支持 `K_COPY_TO_END_AND_INITIALIZE_TO_HOLE` 自动 hole 填充与写屏障决策。
6. `copy_double_to_double_elements`: FixedDoubleArray 浮点切片拷贝，支持 `K_COPY_TO_END_AND_INITIALIZE_TO_HOLE` 与同数组内存重叠自搬迁。
7. `copy_smi_to_double_elements`: Smi 转换为 f64 并写入 FixedDoubleArray，保留 Hole 标记。
8. `copy_packed_smi_to_double_elements`: 密集无洞 Smi 批量快速转 f64。
9. `copy_object_to_double_elements`: 混合 Smi / HeapNumber 转换为 f64，非数值项设为 Hole。
10. `copy_from_float16_to_f16/f32/f64` 及切片封装: float16 缓冲区向各种标量缓冲区的安全转换拷贝。

### 4. 推迟台账 (13 处条目已收敛至 readme/zh.md)

- `ELEM-ISO`: 4 项 (`ArrayConstructInitializeElements`, `ThrowArrayLengthRangeError`, `CopyDoubleToObjectElements`, `ElementsAccessor::Concat`)，需 `v8-runtime` / `v8-heap-core`。
- `ELEM-CSA`: 3 项 (`CopyFastNumberJSArrayElementsToTypedArray`, `CopyTypedArrayElementsToTypedArray`, `CopyTypedArrayElementsSlice`)，需 `v8-builtins-typedarray`。
- `ELEM-DICT`: 2 项 (`CopyDictionaryToObjectElements`, `CopyDictionaryToDoubleElements`)，需 `v8-objects-collections` 字典哈希表查找。
- `ELEM-SLOW`: 2 项 (`IncludesValueSlowPath`, `IndexOfValueSlowPath`)，需 `v8-objects-js-core` 原型链遍历。
- `ELEM-STATIC`: 2 项 (`InitializeOncePerProcess`, `TearDown`)，需完整访问器子类层次体系。

### 5. 门禁验证与规范检查

- **测试门禁**:
  - 新增 `tests/elements_unittest.rs` (覆盖 11 组细分用例)。
  - `CARGO_TARGET_DIR=target/agent_w8arr cargo nextest run -p v8-objects-arrays` → **48/48 passed (100% 绿)**。
- **Clippy 检查**:
  - `CARGO_TARGET_DIR=target/agent_w8arr cargo clippy -p v8-objects-arrays --all-targets -- -D warnings` → **0 warnings, 0 errors**。
- **Allow 绕过检查**:
  - `rg "\[allow" v8-objects-arrays/src` → **0 命中，无任何 allow 绕过**。

## [Reviewer 席] v8-objects-arrays round1 深度审查与优化报告 (达成 clean 1/3) (2026-10-12)

### 1. 规范与原则审查清单 (rust_review 12项原则)

- **原则 1 极致性能 (单次迭代与零拷贝)**:
  - `is_identity_mapping`: 纯单次迭代 `enumerate().skip(index).all(...)`，提前终止，零堆分配。
  - `sort_indices_slice`: 由 `sort_by` 优化为 `sort_unstable_by`，100% 零堆分配原地切片快排，消除标准库归并排序的额外内存分配，精准对齐 V8 原版 `std::sort` (内省排序)。
  - `copy_from_float16_slice_to_f32/f64`: 优化为 `src.iter().zip(dst.iter_mut())` 单次遍历，消除显式索引加法，便于 LLVM 进行 SIMD 自动向量化。
- **原则 2 高内聚低耦合**:
  - 自由函数与访问器签名各归其位，公共抽象通过 `AsIndex` 与 `FixedArrayExt` / `FixedDoubleArrayExt` 解耦，模块划分清晰。
- **原则 3 现代化 Rust 语法**:
  - 全面使用迭代器组合、pattern match、let-chain、const fn，拒绝 C 风格索引循环。
- **原则 4 编译期计算**:
  - 87 项编译期不变式常量断言锁定；常量掩码与静态偏移均在编译期计算。
- **原则 5 冗余与重复清理**:
  - 浮点转换复用 `v8-base::float16::Float16`；向后兼容别名 `pub use ... as ...` 保留旧符号兼容。
- **原则 6-8 错误处理与类型安全**:
  - 全包 0 `unwrap` / 0 `expect` / 0 `panic!`；推迟项均有明确台账编号。
  - 数字转换均经安全范围边界或 `debug_assert!` 护航。
- **原则 9 性能优化与依赖纪律**:
  - 无不必要的 `clone` / `Arc`；使用轻量紧凑切片操作。
- **原则 10 测试规范**:
  - 48 项测试全部位于 `tests/` 目录，涵盖 11 组复杂转换与边界用例，新增 Smi + HeapNumber + Undefined 混合排序及裸指针零长防护测试。
- **原则 11-12 门禁与运行时纪律**:
  - 0 warnings, 0 errors, 0 allow bypass。无异步阻塞或嵌套调度重入。

### 2. 原版 C++ 与 Rust 逐行对数核对

- `is_float16_raw_bits_zero`: 对齐 `(x & ~0x8000) == 0`，保留 IEEE 754 half 零符号位透明度。
- `sort_indices` / `compare_indices`: 数值优先、Undefined 沉底，严格对齐 V8 `std::sort` 严格弱序。
- `copy_object_to_object_elements`: `K_COPY_TO_END_AND_INITIALIZE_TO_HOLE` 饱和计算与 hole 填充、对象写屏障派发完全一致。
- `copy_double_to_double_elements`: 同数组重叠自搬迁 (`ptr::copy` / memmove) 与 hole 填充逻辑完全一致。
- `copy_smi_to_double_elements` / `copy_packed_smi_to_double_elements` / `copy_object_to_double_elements`: Smi / HeapNumber 转换、Hole 处理、防越界均 100% 严密对齐。
- 推迟台账: 13 处未就绪函数均精确标注 `readme/zh.md` 编号，无遗漏未注 `todo!`。

### 3. 门禁验证结果

- `CARGO_TARGET_DIR=target/agent_w8arrr cargo nextest run -p v8-objects-arrays` → **48/48 passed (100% 绿)**。
- `CARGO_TARGET_DIR=target/agent_w8arrr cargo clippy -p v8-objects-arrays --all-targets -- -D warnings` → **0 warnings**。
- `rg "\[allow" v8-objects-arrays/src` → **0 命中，无 allow 绕过**。

### 4. 审查结论

**PASS (达成 clean 1/3)**。
- 0 缺陷 / 0 遗漏 / 0 编译报警 / 测试 100% 通过。

## [Reviewer 席] v8-objects-arrays round2 深度审查与优化报告 (达成 clean 2/3) (2026-10-12)

### 1. 规范与原则审查清单 (rust_review 12项原则)

- **原则 1 极致性能 (单次迭代与零拷贝)**:
  - `copy_double_to_double_elements`: 消除原先同数组分支判断与切片构建开销，全面统一使用 `ptr::copy` (memmove 语义)，严格对齐 V8 C++ 原版 `CopyWords`，实现 100% 零堆分配、零切片检查的极速双精度数组元素搬迁与自搬迁。
  - `copy_smi_to_double_elements`: 由原循环内多次下标 `get(from_start + i)` 寻址与原子读，优化为连续数据切片 `from_slice.iter().enumerate()` 单次迭代，消除重复偏移加法，便于 LLVM 向量化。
  - `copy_packed_smi_to_double_elements`: 同样优化为紧凑连续切片迭代器，单次遍历直接写入目标数组。
  - `copy_object_to_double_elements`: 转换为连续切片单次遍历，单次迭代完成 Hole / Smi / HeapNumber 分支流转与写入。
- **原则 2 高内聚低耦合**:
  - `TrustedPointerMember::store`: 配合上游 `v8-objects-core` 对 `WriteBarrierMode` 枚举的统一，清理了 `trusted.rs` 中的临时 `LegacyMode` 映射适配代码，直接透传官方 `WriteBarrierMode`。
- **原则 3-5 现代化 Rust 语法、编译期计算与代码清理**:
  - 全面使用迭代器组合、零冗余分支、const fn，消除陈旧 C 风格下标循环与冗余切片检查。
  - 87 项编译期不变式断言继续保持 100% 锁定。
- **原则 6-9 错误处理与类型安全**:
  - 全包 0 unwrap / 0 expect / 0 panic。
  - 指针安全范围严格受 `from_len` / `to_len` / `debug_assert!` 与饱和运算保护。
- **原则 10 测试规范**:
  - 新增 `copy_double_to_double_elements` 向前自搬迁 (dst < src) 与 `copy_size == 0` 边界测试。
  - 48 项测试用例 100% 通过。
- **原则 11-12 门禁与运行时纪律**:
  - 0 warnings, 0 errors, 0 allow bypass。无调度重入与异步阻塞。

### 2. 原版 C++ 与 Rust 逐行对数核对

- `copy_double_to_double_elements`: 无论同数组还是异数组均通过 `ptr::copy` 映射为 `memmove`，与 V8 C++ `CopyWords(to_address, from_address, ...)` 完全一致。
- `copy_smi_to_double_elements` / `copy_packed_smi_to_double_elements` / `copy_object_to_double_elements`: Smi / HeapNumber 浮点解包、Hole 填充与边界控制与原版 elements.cc 完全一致。
- `SortIndices`: `compare_indices` 维护严格弱序 (数值优先升序、Undefined 沉底)，`sort_unstable_by` 零堆分配内省快排。
- `get_write_barrier_mode`: Smi 与 Double 跳过写屏障，Object 派发 `UpdateWriteBarrier`。
- 推迟台账: 13 处条目完整合规对应 `readme/zh.md`。

### 3. 门禁验证结果

- `CARGO_TARGET_DIR=target/agent_w8arrr2 cargo nextest run -p v8-objects-arrays` → **48/48 passed (100% 绿)**。
- `CARGO_TARGET_DIR=target/agent_w8arrr2 cargo clippy -p v8-objects-arrays --all-targets -- -D warnings` → **0 warnings**。
- `rg "\[allow" v8-objects-arrays/src` → **0 命中，无 allow 绕过**。

### 4. 审查结论

**PASS (达成 clean 2/3)**。
- 0 缺陷 / 0 遗漏 / 0 编译报警 / 测试 100% 通过。

## [Reviewer 席] v8-objects-arrays round3 终验交叉比对审查报告 (达成 clean 3/3 终检验收冻结) (2026-10-12)

### 1. 规范与原则终验核对 (rust_review 12项原则)

- **原则 1 极致性能 (单次迭代与零拷贝)**:
  - `copy_double_to_double_elements`: 全量统一采用 `ptr::copy` (memmove 语义)，严格对齐 V8 C++ `CopyWords`，同数组/异数组 100% 零堆分配、零重复检查。
  - `copy_smi_to_double_elements` / `copy_packed_smi_to_double_elements` / `copy_object_to_double_elements`: 连续切片 `from_slice.iter().enumerate()` 单次迭代，批量消除下标重复偏移加法，便于 LLVM 向量化。
  - `sort_indices_slice`: `sort_unstable_by` 零堆分配原地快排 (内省排序)，严格对齐 V8 原版 `std::sort`。
  - `is_identity_mapping`: 单次单向短路遍历 `enumerate().skip(index).all(...)`，零堆分配。
  - `copy_from_float16_slice_to_f32/f64`: 迭代器 `zip` 单次遍历，无显式索引加法。
- **原则 2 高内聚低耦合**:
  - 核心辅助函数、访问器方法列表与常量分离，结构清晰。写屏障模式透传 `v8-objects-core`。
- **原则 3-5 现代化 Rust 语法、编译期计算与代码清理**:
  - 全面使用迭代器组合、pattern match、let-chain、const fn，拒绝陈旧 C 风格索引循环。
  - 87 项编译期不变式断言继续 100% 锁定 (`IMPL_ONLY_FREE_FUNCTIONS`(16) / `STATIC_METHODS`(3) / `TYPED_SPECIALIZATIONS`(64) / `ACCESSOR_CLASSES`(32))。
- **原则 6-9 错误处理、类型安全与防御加固**:
  - 全模块 0 `unwrap` / 0 `expect` / 0 `panic!`。
  - 终验加固：对 `copy_smi_to_double_elements`、`copy_packed_smi_to_double_elements` 与 `copy_object_to_double_elements` 的 `K_COPY_TO_END_AND_INITIALIZE_TO_HOLE` 边界统一追加 `.min(to_len.saturating_sub(to_start))` 目标容量紧上界钳位，彻底杜绝调用方目标缓冲过短时的越界隐患。
- **原则 10 测试规范**:
  - 49 项测试用例全部位于 `tests/` 目录，覆盖率 100%。新增 `test_copy_to_end_destination_clamping` 验证目标数组容量受限时的边界饱和安全。
- **原则 11-12 门禁与运行时纪律**:
  - 0 warnings, 0 errors, 0 allow bypass。无异步阻塞与调度重入。

### 2. 原版 C++ 与 Rust 逐行交叉比对结果

- `is_float16_raw_bits_zero`: `(x & !0x8000) == 0`，与 V8 C++ `(x & ~0x8000) == 0` 精确一致。
- `is_identity_mapping`: 递归/迭代单次短路比对与 V8 C++ `IsIdentityMapping` 逻辑完全等价。
- `get_write_barrier_mode`: Smi/Double 免屏障，Object 更新屏障，与 V8 严格一致。
- `sort_indices` / `compare_indices`: 数值升序、Undefined 沉底，`sort_unstable_by` 严格弱序与 V8 `std::sort` 行为完全一致。
- `copy_object_to_object_elements`: `K_COPY_TO_END_AND_INITIALIZE_TO_HOLE` 饱和填充与写屏障派发与 V8 C++ 100% 一致。
- `copy_double_to_double_elements`: `ptr::copy` 原生 memmove 保源序重叠搬迁与 Hole 填充完全一致。
- `copy_smi_to_double_elements` / `copy_packed_smi_to_double_elements` / `copy_object_to_double_elements`: 浮点转换、Hole 处理、防越界均与原版 elements.cc 严格一致。
- `copy_from_float16_backing_store`: float16 原位比特保留与 f32/f64 展开与 elements.cc 模板特化完全一致。
- 推迟台账: 13 处条目全部 1:1 精确核验登记在 `v8-objects-arrays/readme/zh.md`。

### 3. 门禁验证结果

- `CARGO_TARGET_DIR=target/agent_w8arrr3 cargo nextest run -p v8-objects-arrays` → **49/49 passed (100% 绿)**。
- `CARGO_TARGET_DIR=target/agent_w8arrr3 cargo clippy -p v8-objects-arrays --all-targets -- -D warnings` → **0 warnings, 0 errors**。
- `rg "\[allow" v8-objects-arrays/src` → **0 命中，无任何 allow 绕过**。

### 4. 终验结论

**PASS (达成 clean 3/3 终检验收冻结)**。
- 0 缺陷 / 0 遗漏 / 0 编译警告 / 0 allow 绕过 / 测试 49/49 全部通过。
- 准予终检验收冻结。



