# v8-objects-strings

字符串对象族: String 及 7 个具体子类 / 内部化表 / 转发表 / 数字串缓存 / 比较器 / StringSet 形状 / RegExp 迭代器。

## 偏差登记 (原版对照)

| 偏差 | 原版 | Rust 承接 |
|---|---|---|
| `CHARS_OFFSET` | kSeqCharsOffset=12 (压缩) | `size_of::<String>()`=24 (Rust 逻辑布局, Tagged 槽 usize 宽; 两者不可混用) |
| MapWord 双参调用 | `FromForwardingAddress(object)` 单参 | tests 2 处双参已修回单参 |
| Cons 段比对长度检查 | `kNoLengthCheck` DCHECK 在段长>剩余子串时误伤 | 段内改传 `Prefix` (长度已检语义等价) |
| SlicedString 嵌套 | 不变量禁止 (string.h:1190) | get_flat_content/VisitFlat/段源防御性循环穿透 (合法输入行为一致) |
| WriteUtf8Bytes trail 合并 | Encode 写 trail 位置 4 字节 (原版疑似写歪) | 覆盖回写契约: out 自 lead 3 字节起 (v8-strings encode 契约) |
| set_raw_hash_field_if_empty | compare_exchange_strong | 单线程「仅空字段写」, 原子性归宿主堆层 |
| as_array/integer_index 慢路径补算 | EnsureRawHash (roots seed) | HashSeedData::DEFAULT (宿主接 roots 后改注入) |
| to_std_string 非法 UTF-8 | std::string 原样 | Result + Unsupported (孤立代理 CESU-8 字节) |

## 原版 → Rust 测试映射 (todo 7.3)

| 原版测试 | Rust 测试 | 说明 |
|---|---|---|
| cctest/test-strings.cc:539 Traverse | tests/string_cons.rs | ConsStringIterator 拼接回读 |
| cctest/test-strings.cc:771/853 边界+深树 | tests/string_cons.rs | 爆栈重启 (深度 > 32) + WriteToFlat/2 |
| cctest/test-strings.cc:1919 StringEquals | tests/string_equals.rs | 跨形态等值 |
| cctest/test-strings.cc:1852/1900 哈希索引 | tests/string_hash.rs | HashArrayIndexStrings / ArrayIndexHashRoundTrip |
| cctest/test-strings.cc:881/1141/1612 UTF-8 | tests/string_utf8.rs | 长度/编码/替换/非 Latin-1 SWAR |
| unittests/objects/string-table-unittest.cc | tests/string_table_test.rs | HasString/Internalize + 容量阶梯/并发 |
| number-string-cache (unittests 无独立文件) | tests/number_string_cache_test.rs | 规约自 inl.h 直迁 |
| string-comparator (随 cctest isolate 用例) | tests/string_comparator_test.rs | 规约自 cc 直迁 |
| string-set (随 HashTable 基建) | tests/string_set_test.rs | 仅 Shape 规约, New/Add/Has 推迟 v8-objects-collections |
| string-forwarding-table (test-shared-strings) | tests/string_forwarding_table_test.rs | 块阶梯/Record 原子族/生命周期; isolate 段推迟 |
| js-regexp-string-iterator (无独立测试) | tests/js_regexp_string_iterator_test.rs | 位段+字段往返, 运行时推迟 L4 |
| string.h/-inl 布局/谓词/展平 | tests/string_shape.rs, string_flat.rs, string_misc.rs | snapshot 常量锚定 + 展平/搜索/杂项 |

跳过项理由: 需要 isolate/factory/heap 的用例 (Flatten 分配、外部化、共享堆) 推迟 L3/L4, 见各源文件头注释登记。
