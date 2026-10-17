[English](#en) | [中文](#zh)

---

<a name="en"></a>

#

---

<a name="zh"></a>

# v8-snapshot 转写台账 (Wave 9 · W9SNAP)

覆盖原版 `v8/src/snapshot` 全族 (50 文件) + `v8/src/objects/value-serializer.{h,cc}`。
本 crate 已落地序列化体系的**规则层** (字节码/枚举/位域/blob 头布局/Uint30 编解码/校验和/
源汇原语/线上格式标签) 与纯可测语义体; **依赖真实 Isolate/堆的分派面登记推迟**。

- [一、已实现 (纯可测, nextest 覆盖)](#一已实现-纯可测-nextest-覆盖)
- [二、推迟台账 (依赖真实 Isolate / 堆 / 跨模块编译期常量)](#二推迟台账-依赖真实-isolate-堆-跨模块编译期常量)
- [三、测试迁移映射](#三测试迁移映射)
- [四、依赖与红线](#四依赖与红线)
- [五、脚本基线复跑提示](#五脚本基线复跑提示)

## 一、已实现 (纯可测, nextest 覆盖)

- `snapshot_source_sink.rs` — SnapshotByteSource/SnapshotByteSink + Uint30 游程编解码 / Uint32 /
  Raw / Blob 往返 (对偶性经边界值 + 全段范围往返测)
- `serializer_deserializer.rs` — Bytecode 36 码 / SlotType / 段编码 SpaceEncoder 系列 /
  UNUSED_SERIALIZER_BYTE_CODES 133 / 段宽与可编码范围常量
- `read_only_serializer_deserializer.rs` — ro::Bytecode / BitSet / EncodedTagged /
  EncodedExternalReference 位域 (掩码互斥与覆盖 32 位经测)
- `references.rs` — SnapshotSpace / SpecialValueType / SerializerReference 位打包 (往返保真经测)
- `snapshot.rs` / `snapshot_data.rs` / `code_serializer.rs` / `embedded_data.rs` — 快照/data/
  code/embedded blob 头布局常量与偏移链 1:1 (偏移不变量经测)
- `snapshot_utils.rs` — checksum Fletcher-32 (确定性/敏感性经测)
- `snapshot_compression.rs` — 压缩 blob 长度前缀布局 framing (split/前缀往返经测)
- `value_serializer.rs` — SerializationTag 全 59 标签 + from_byte 反查 (ASCII 锚与往返经测)
- `serializer.rs`/`deserializer.rs`/`sort_builtins.rs`/`platform_embedded_file_writer_base.rs` —
  常量/枚举/方法面 (递归上限/热对象环/对齐/数据指令宽/目标枚举)
- `builtins_effects_dummy.rs` — mksnapshot 保守 `builtin_can_allocate` 恒真桩
- `snapshot_empty.rs`/`snapshot_external.rs`/`embedded_empty.rs`/`mksnapshot.rs`/`static_roots_gen.rs` —
  构建期桩面 (方法清单登记, 无 Isolate 前不可跑)

## 二、推迟台账 (依赖真实 Isolate / 堆 / 跨模块编译期常量)

逐处登记, 解锁条件写明:

1. **Serializer / Deserializer 字节码 visitor 状态机** — `serializer.cc` (1482) /
   `deserializer.cc` (1755) 全部分派。缺前置: Isolate + Heap + 对象图 (Tagged/Handle)。
   解锁: v8-execution-isolate + v8-heap-* 对象图就位后。
2. **ObjectSerializer/ObjectVisitor 按类型 Visit\* 族** (serializer-deserializer.cc 的
   Iterate*ObjectCache / CanBeDeferred) 与 read-only/context/startup/shared-heap/roots/object
   各具体 (De)Serializer 的方法体。缺前置: v8-objects-* 各类型访问器 + Isolate。
   解锁: objects/heap/isolate 三栈 API 冻结后。
3. **Snapshot::Create / Initialize / NewContextFromSnapshot / SerializeDeserializeAndVerifyForTesting**
   (`snapshot.cc`)。缺前置: Isolate 生命周期 + StartupData 装载。解锁: v8-init setup-isolate。
4. **CopySlots (Address/AtomicWord 与 V8_COMPRESS_POINTERS 下 Tagged_t 重载)** —
   缺前置: v8-common AtomicWord / Relaxed_Store + 堆压缩槽类型。解锁: 指针压缩面就位。
5. **snapshot_compression Compress/Decompress 的 zlib ZRAW 体** — 第十章 v8-snapshot 白名单无 zlib,
   且 ZRAW 裸流须与原版逐字节差分后方可接入。长度前缀布局已落, 压缩算法体推迟。
   解锁: 参考构建 zlib 差分 harness + 白名单准入 flate2/zlib 内核。
6. **code_serializer (SerializedCodeData 1035 行)** — 代码缓存双段 (untrusted/trusted) 序列化,
   依赖 Isolate + Code/SharedFunctionInfo/反馈向量。解锁: 编译器 + code 对象面就位。
7. **embedded 文件写入器族 (embedded-file-writer + platform_*_file_writer)** —
   embedded.S 汇编发射 (Prologue/Builtin 循环/PadAndAlignCode/Epilogue), 构建期面。
   常量与枚举已落; 写体依赖 EmbeddedBlob 内存与反汇编标签。解锁: mksnapshot 构建链路。
8. **mksnapshot 工具 main / SnapshotFileWriter / static-roots-gen write** — 离线快照/静态根生成,
   构建期 CLI。缺前置: Isolate + 嵌入链符号。解锁: M4 mksnapshot 最小子集。
9. **sort-builtins 聚类排序器** — 读 profiling 日志 (block_count/builtin_density marker) 建图排序,
   纯构建期分析, 依赖内建尺寸/密度日志。解锁: builtins profiling 语料。
10. **blob 表容量的跨模块编译期常量** — `blob_layout` 中 table_size = `Builtins::kBuiltinCount`;
    `snapshot_data` 魔数 `0xC0DE0000 ^ ExternalReferenceTable::kSize`。均为跨 crate 编译期常量,
    已改为 `const fn(table_size)` 参数化 / 表达式登记, 不作伪常量。
    解锁: v8-builtins kBuiltinCount 与 external-reference-table 常量导出后在 mksnapshot 装配时落值。

## 三、测试迁移映射

原版 `v8/test/cctest/test-serialize.cc` 用例绝大多数为 `UNINITIALIZED_TEST(Isolate fixture)`,
Isolate 未就位前不可跑, 处理如下:

- 已 1:1 转写断言面 (纯层可跑):
  - `test_checksum` ← SnapshotChecksum 的校验和确定性/位翻转敏感性断言 (无 Isolate 部分)
  - `test_compression_framing` ← SnapshotCompression 的长度前缀往返断言 (无 zlib 部分)
  - `test_byte_source_sink_roundtrip` ← sink 写入 / source 读回 (SerializeContext 用到的原语)
  - `test_blob_and_data_layout` ← blob 头偏移不变量
- 推迟 (依赖 Isolate, 台账二·1/2/3/5): StartupSerializerOnce/Twice, ContextSerializerContext/
  CustomContext, CustomSnapshotDataBlob* (全族), SnapshotCompression (zlib 体), SnapshotChecksum
  (VerifyChecksum 走完整 blob), CodeSerializer* 族 — 待 Isolate/zlib 解锁。
- 补充纯层往返/位域测 (原版无独立 gtest, 按 §11 规则代码补): 段编解码、SnapshotSpace/位打包、
  ro 位域、SerializationTag from_byte。

## 四、依赖与红线

- 依赖: 仅 `log` (语义日志) + `thiserror` (错误); dev: `aok`/`ctor`/`log_init`。
  v8-objects-*/heap-*/roots/builtins-* 的分派面推迟, 故暂不 `cargo add --path` 引用 (避免悬空依赖),
  解锁时再按依赖图接入。
- **crc32fast 评估**: V8 快照校验和为 adler32 (zlib 构建) 或 Fletcher-32 (无 zlib),
  二者均非 CRC32 语义; `crc32fast` 与本 crate 校验和无语义对应, **不引入**。
- 禁 panic/unwrap: 越界以 debug_assert 标契约 (原版 DCHECK 对应), 无运行时 panic 路径。

## 五、脚本基线复跑提示

`tool/src/snapshot/snapshot-source-sink.js` 与 `snapshot-compression.js` 产签名/布局骨架基线;
本席位已在 src 就地填 Uint30/framing 语义体。按 §11, **复跑这两个脚本会覆盖修订** (仅限重生成基线),
如需重生成须同步回填语义体。`tool/src/objects/value-serializer.h.js` 已重定向输出至本 crate。

