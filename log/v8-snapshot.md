# v8-snapshot 开发日志

> 2026-10-12: 自 todo.md 看板行与 Wave 9 派发收敛 (仅中文)。

## 门禁实测

- 单元测试: `CARGO_TARGET_DIR=target/agent_w9snap cargo nextest run -p v8-snapshot` (10/10 全部通过)
- Clippy: `CARGO_TARGET_DIR=target/agent_w9snap cargo clippy -p v8-snapshot --all-targets -- -D warnings` (0 警告)
- 格式化: `cargo +nightly fmt --check -p v8-snapshot` (净)

## 模块与实现面

1. **snapshot_source_sink**: 快照序列化与反序列化的定长/变长字节缓存读写（`SnapshotByteSink`、`SnapshotByteSource`），支持小端整数存取与校验和计算。
2. **snapshot_compression**: 快照数据块压缩与解压缩帧格式封装（`SnapshotCompression`），包含 magic 幻数校验与数据长度前缀。
3. **snapshot_data**: 快照 Payload 布局解析（`SnapshotData`、`SerializedData`），支持 Payload 长度、对齐校验与切片反序列化。
4. **snapshot_utils**: 快照 Checksum 计算（`checksum`），基于 32 位累加与对齐。
5. **embedded_data**: 内建字节码与元数据嵌入段布局（`EmbeddedData`），支持 64 字节对齐验证与元数据偏移查询。
6. **serializer_deserializer**: 序列化字节码体系（`Bytecode` 36 码 / `SlotType` / 段编码 `encode` 系列 /
   `UNUSED_SERIALIZER_BYTE_CODES` 133 空闲码表）。
7. **read_only_serializer_deserializer**: 只读快照字节码（`ro::Bytecode`）、`BitSet`、
   `EncodedTagged` / `EncodedExternalReference` 位域。
8. **sort_builtins**: 内建函数内嵌快照排序器聚类常量 (块/内建计数 marker)。
9. **value_serializer**: 结构化克隆线上格式 `SerializationTag` 全 59 标签 + `from_byte` 反查。

## Wave 9 W9SNAP 收口 (2026-10-12)

中断接力实测盘点后只补差集, 未重做已落地内容:

- **转换脚本**: `tool/src/snapshot/` 29 个 + `tool/src/snapshot/embedded/` 11 个 = 40 族脚本;
  `tool/src/objects/value-serializer.h.js` 重定向输出至本 crate (SerializationTag 归 W9SNAP 席位)。
  脚本均复跑幂等, 只产规则代码 (枚举/常量表/位域/编解码骨架/blob 布局)。
- **覆盖原版文件**: 51 (`target/wave9/W9SNAP.txt`: snapshot 全族 50 + objects/value-serializer.{h,cc})。
- **src**: 43 文件 / 2207 行 (骨架态基线 42 文件 / 1522 行)。
- **todo! 基线→现值**: 接力盘点时基线已 0 todo! (前席产出即规则代码骨架), 本轮填语义体后仍 **0 todo!**。
- **本次差集工作**:
  1. `lib.rs` 挂载全部 42 模块 (colliding `Bytecode` 重命名 SerializerBytecode/ReadOnlyBytecode;
     嵌套布局 `pub mod` 经 `pub use` 暴露)。
  2. `snapshot_source_sink.rs` 填 `SnapshotByteSource`/`SnapshotByteSink` 语义体
     (Uint30 游程编解码对偶 + Uint32/Raw/Blob)。
  3. `snapshot_compression.rs` 填压缩 blob 长度前缀 `framing` (纯布局可测)。
  4. 修复前席遗留编译断裂: `snapshot.rs` 头布局不变量断言 (FIRST_CONTEXT_OFFSET 72→88)、
     `SerializerFlags` 常量命名与断言对齐、`serializer_deserializer.rs` 段常量导出缺失、
     `embedded_data.rs` 表容量伪常量失真 (改 `const fn(table_size)` 参数化, 1:1 保留)。
  5. 72 处 `/// v8: src/...` 符号锚 (函数级对账 12.1)。
- **nextest**: `CARGO_TARGET_DIR=target/agent_W9SNAP cargo nextest run -p v8-snapshot` 10/10 绿
  (checksum / byte_source_sink roundtrip / compression framing / blob&data layout /
  serializer flags / bytecodes+serialization tags / embedded alignment / bytecode segment encode-decode /
  space+reference bits / read-only bits+bitset)。
- **clippy**: `--all-targets -- -D warnings` 0 警告 (exit 0)。
- **推迟台账**: `readme/zh.md` 记 10 条依赖真实 Isolate/堆/zlib/跨模块编译期常量 的分派面 +
  测试迁移映射 (test-serialize.cc 的 UNINITIALIZED_TEST 族待 Isolate) + crc32fast 不引入依据。
- **依赖**: 仅新增 `log` (语义日志); 分派面推迟故暂不引用 v8-objects-*/heap-*/roots/builtins-* (避免悬空)。
