# v8-objects-strings 开发日志

> 2026-10-12 收敛: 自 readme/{zh,en}.md 台账迁入 (仅中文)。看板 (todo.md) 只留状态。

## readme 台账 (2026-10-12 自 readme/{zh,en}.md 收敛)

### 语义轮 (WriteToFlat 语义轮, 2026-10-12)

轮4 收口时 10 个测试文件带 `#![cfg(any())]` 休眠 (引用当时缺失的 string.rs 语义面);
本轮语义面就位后删除休眠行恢复, 84/84 全绿。

#### 补齐的语义面 (src/string.rs @inl + @cc 区段)

- Ext trait 族: `StringExt` (length/get/equals/is_equal_to_u8(16)/get_flat_content/
  compute_and_set_raw_hash/as_array_index/as_integer_index/is_one_byte_representation_underneath/
  try_flat_content_of_direct) / `SeqOneByteExt` / `SeqTwoByteExt` (chars/get/set/set_chars) /
  `ConsExt` (first/second/unchecked_second/is_flat/get) / `SlicedExt` / `ThinExt` /
  `ExternalExt` (resource/resource_data)
- `FlatContent<'a>` 三态 + `StringKind::of` 24 型位运算派发 + 值构造器 4 个 + Seq 尺寸公式
- `ConsStringIterator` 算法体 (Initialize/Continue/Search/NextLeaf, 爆栈重启状态机)
- `StringCharacterStream` (单字节游标承接原版 union) + `FlatStringReader` + `SubStringRange`
- `write_to_flat` (区间递归 + 幂等复本消除) / `write_to_flat2` (反向 DFS + RepeatOptimizer,
  wtf 双变体循环) + `SinkChar`/`CharData` 域抽象
- flat 自由函数族: index_of/last_index_of/compare/calculate_line_ends/is_identifier/
  utf8_length/write_utf8/is_well_formed/is_ascii/is_one_byte_uc16/non_one_byte_start
- 堆侧函数: to_array_index/to_cstring/to_std_string/does_not_contain_escape_characters
- 慢路径: slow_equals (Thin 重启 + 哈希快否 + StringComparator 段源适配) /
  hash_string_u8(16) (trivial/Cons 缓冲/direct) / string_to_index_array(integer)

#### 本轮缺陷修复 (非票面预定)

- string_forwarding_table: `ensure_capacity` 倍容后返回旧快照指针 (load_block 越界断言) → 重读后统一返回;
  `Drop` 补 storage 各代 BlockVector 本体释放 (旧实现泄漏 160 字节/代)
- BlockVector/StringForwardingTable `unsafe impl Send/Sync` (全原子字段 + 所有权唯一, 并发表设计承接)
- 并发环境事故: 外部会话将 tests/ 回退至 HEAD (休眠版), 轮4 休眠头 10 文件已按台账约定恢复

(en.md 为空文件, 无独有信息补记。)
