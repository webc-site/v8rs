# v8-strings 开发日志

> 2026-10-12 收敛: 自 readme/{zh,en}.md 台账迁入 (仅中文)。看板 (todo.md) 只留状态。

## readme 台账 (2026-10-12 自 readme/{zh,en}.md 收敛)

### 门禁实测 (readme/zh.md 原文, 2026-10-11 时点)

门禁实测: cargo check 0 错 0 警 / clippy -D warnings 0 / nextest 27 通过 0 失败。

(en.md 为空文件, 无独有信息补记。)

## readme 台账 (2026-10-12 补迁)

### round2 复跑门禁 (自 README.md 生成物抢救, 2026-10-12)

门禁实测 (round2 复跑): clippy -D warnings 0 / nextest 64 通过 0 失败 (54 原有 + 10 round2 差分)。

### round3 对抗审查 (自 README.md 生成物抢救, 2026-10-12)


差分结论 (Python 独立参考实现, 临时 harness 审后移除):
- rapidhash 全宏 + ConvertTo8BitReader: 10500 随机向量逐位一致 (长度 0..256 含 47/48/49 边界)
- HashSequentialString 全分支 (array-index/integer-index/trivial/非索引): 1285 向量一致 (含 MAX_ARRAY_INDEX/MAX_SAFE_INTEGER/前导零/16383 截断边界; 参考实现需补 `c-'0'` 回绕与 `value<<2` u32 截断两处语义才对齐)
- utf8 计数/编码 (Length/Encode/LengthOneByte/EncodeOneByte + 整串引擎截断/代理回退): 334 万向量 0 差异
- StringStream vs libc snprintf: 浮点 13 指令 × 27 值 0 差异; 整数宽度/零填充 0 差异

findings (均双引用, 已修复):
1. F1 src/unicode.rs:356/400 — `capacity=0 且 write_null=true` 时 `capacity - usize::from(write_null)` usize 回绕 → release 越界 panic; 原版 unicode-inl.h:222 `CHECK_LE(content_capacity, capacity)` 受控 abort 的等价防线缺失。修复: 空缓冲提前返回 (0,0)。回归: tests/review_round5_unittest.rs test_utf8_encode_string_engine。
2. F2 src/string_stream.rs:256 — `%c` 产出 Unicode 字符串, 与 C SNPrintF 单字节截断语义不符 (上游唯一用点 string-stream.cc:360 为 ASCII 域)。修复: 低 8 位单字节直发。回归: tests/review_round5_unittest.rs test_string_stream_char_truncation_and_int_widths。
3. F3 src/string_stream.rs:319 — 整数精度 (%.3d 最少位数) 与左对齐 (%-8d) 未承接; 上游全部 Add/SNPrintF 字面量零使用, 登记 SNPrintF 语义子集边界 (doc 注释)。
4. F4 src/hash_field.rs:78 — `is_integer_index` doc 误标 Name::IsIntegerIndex (精确 decode 语义), 实现为 CSA IsUniqueNameNoIndex 保守掩码语义 (ForwardingIndex 判真); 修 doc 不改语义。
5. F5 tests/string_hasher.rs — running_string_hasher 原断言 h1==h2 为同输入自比 (弱断言); 补 SuperFastHash 独立重算黄金值 (含 seed 参与 / 代理对串)。

登记 (不修): IsOnly8Bit 检查缺失时 ConvertTo8BitReader 高位字节静默截断 (调用点均经 is_only_8_bit 预检, 原版为 DCHECK 域); ScanForSurrogates 尾部 0xED 原版越界读改安全 get; encode_string_* 容量截断写满即停 (原版解码端无容量检查越界写, 收窄为安全截断)。

门禁实测 (round3 复跑): clippy -D warnings 0 / nextest 65 通过 0 失败 (64 原有 + 1 round3 回归)。

