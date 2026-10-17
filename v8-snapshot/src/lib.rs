#![cfg_attr(docsrs, feature(doc_cfg))]

// v8-snapshot: 原版 v8/src/snapshot 全族 + v8/src/objects/value-serializer。
// 序列化/反序列化公共字节码、快照源汇、blob 头布局与校验和原语。
// 依赖真实 Isolate/堆的分派面登记推迟台账 (见 readme/zh.md)。

mod error;
pub use error::{Error, Result};

// —— 序列化引用与快照空间 ——
mod references;
pub use references::{
  NUMBER_OF_SNAPSHOT_SPACES, SnapshotSpace, SpecialValueType, serializer_reference_bits,
};

// —— 序列化/反序列化公共字节码体系 ——
mod serializer_deserializer;
/// v8: src/snapshot/serializer-deserializer.h Bytecode (序列化器字节码)
pub use serializer_deserializer::Bytecode as SerializerBytecode;
pub use serializer_deserializer::{
  DOUBLE_ALIGNMENT_SENTINEL, EMPTY_BACKING_STORE_REF_SENTINEL, FIRST_ENCODABLE_FIXED_RAW_DATA_SIZE,
  FIRST_ENCODABLE_REPEAT_ROOT_COUNT, FIRST_ENCODABLE_VARIABLE_REPEAT_ROOT_COUNT,
  FIXED_RAW_DATA_COUNT, FIXED_REPEAT_ROOT_COUNT, HOT_OBJECT_COUNT,
  LAST_ENCODABLE_FIXED_RAW_DATA_SIZE, LAST_ENCODABLE_FIXED_REPEAT_ROOT_COUNT,
  ROOT_ARRAY_CONSTANTS_COUNT, SlotType, UNUSED_SERIALIZER_BYTE_CODES, encode,
};

// —— RO 序列化/反序列化公共字节码与位域 ——
mod read_only_serializer_deserializer;
/// v8: src/snapshot/read-only-serializer-deserializer.h ro::Bytecode (只读空间字节码)
pub use read_only_serializer_deserializer::Bytecode as ReadOnlyBytecode;
pub use read_only_serializer_deserializer::{
  NUMBER_OF_BYTECODES, bitset, encoded_external_reference, encoded_tagged,
};

// —— 快照字节源/字节汇 ——
mod snapshot_source_sink;
pub use snapshot_source_sink::{SnapshotByteSink, SnapshotByteSource, UINT30_MAX};

// —— 序列化器常量与方法面 ——
mod serializer;
pub use serializer::{MAX_RECURSION_DEPTH, hot_objects_list};

// —— 反序列化器常量与方法面 ——
mod deserializer;
pub use deserializer::{CODE_EMBEDS_OBJECT_POINTER, DeserializingUserCodeOption};

// —— 序列化器内联函数面 ——
mod serializer_inl;

// —— 根/上下文/启动/共享堆序列化族 ——
mod context_deserializer;
mod context_serializer;
mod object_deserializer;
mod read_only_deserializer;
mod read_only_serializer;
mod roots_serializer;
mod shared_heap_deserializer;
mod shared_heap_serializer;
mod startup_deserializer;
mod startup_serializer;

// —— 序列化器-反序列化器 cc 面骨架 ——
mod serializer_deserializer_cc_generated;
pub use serializer_deserializer_cc_generated::SerializerDeserializerCc;

// —— 快照门面与 blob 头布局 ——
mod snapshot;
pub use snapshot::{
  DEFAULT_CONTEXT_INDEX, FIRST_ADDTL_CONTEXT_INDEX, SerializerFlags, snapshot_blob_layout,
};

// —— 快照数据 blob 头布局 ——
mod snapshot_data;
pub use snapshot_data::{layout as snapshot_data_layout, serialized_data};

// —— 快照校验和 ——
mod snapshot_utils;
pub use snapshot_utils::checksum;

// —— 快照压缩门面 ——
mod snapshot_compression;
pub use snapshot_compression::{
  PAYLOAD_LENGTH_PREFIX_SIZE, SnapshotCompression, framing, get_uncompressed_size,
};

// —— 空快照 / 外部快照 / mksnapshot 桩 ——
mod mksnapshot;
mod snapshot_empty;
mod snapshot_external;
mod static_roots_gen;

// —— 代码缓存序列化头布局 ——
mod code_serializer;
pub use code_serializer::{SANITY_CHECK_RESULT_COUNT, serialized_code_data_layout};

// —— 内建函数内嵌快照排序器 ——
mod sort_builtins;
pub use sort_builtins::{
  BUILTIN_CALL_BLOCK_DENSITY_MARKER, BUILTIN_DENSITY_MARKER, MAX_CLUSTER_SIZE,
  MAX_DENSITY_DECREASE_THRESHOLD, MIN_EDGE_PROBABILITY_THRESHOLD,
};

// —— mksnapshot 专用内建副作用桩 ——
mod builtins_effects_dummy;
pub use builtins_effects_dummy::builtin_can_allocate;

// —— 内嵌 blob 布局 ——
mod embedded_data;
pub use embedded_data::{
  CODE_ALIGNMENT_ARM64, CODE_ALIGNMENT_X64, METADATA_ALIGNMENT, ReorderedBuiltinIndex, blob_layout,
  builtin_lookup_entry, layout_description,
};
mod embedded_data_inl_generated;
pub use embedded_data_inl_generated::EmbeddedDataInl;

// —— 嵌入文件写入器族 ——
mod embedded_file_writer_interface;
pub use embedded_file_writer_interface::{DEFAULT_EMBEDDED_VARIANT, EmbeddedFileWriterInterface};
mod embedded_file_writer;
pub use embedded_file_writer::{FIRST_EXTERNAL_FILENAME_ID, TEMPORARY_STRING_LENGTH};
mod platform_embedded_file_writer_base;
pub use platform_embedded_file_writer_base::{DataDirective, EmbeddedTargetArch, EmbeddedTargetOs};
mod embedded_empty;
mod platform_embedded_file_writer_aix;
mod platform_embedded_file_writer_generic;
mod platform_embedded_file_writer_mac;
mod platform_embedded_file_writer_win;
mod platform_embedded_file_writer_zos;

// —— 结构化克隆线上格式标签 (value-serializer) ——
mod value_serializer;
pub use value_serializer::SerializationTag;
