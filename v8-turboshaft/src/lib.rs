#![cfg_attr(docsrs, feature(doc_cfg))]

use v8_compiler_graph::MachineRepresentation;
mod error;
pub use error::{Error, Result};

mod index;
pub use index::{
  AnyFrameState, BlockIndex, Compressed, EXTERNAL_ID_FLAG, EagerFrameState, Float32, Float64,
  GENERATION_MASK, GENERATION_MASK_SHIFT, InternalTag, LazyFrameState, OpIndex,
  OperationStorageSlot, OptionalOpIndex, SLOTS_PER_ID, Simd128, Simd256, UNMASK_GENERATION_MASK,
  Word32, Word64, WordPtr, abstract_rep,
};

mod representations;
pub use representations::{
  MaybeRegisterRepresentation, MemoryRepresentation, RegisterRepresentation,
};

mod types;
pub use types::*;

mod operations;
pub use operations::*;

mod phase;
pub use phase::{Phase, PipelineData, TurboshaftPipelineKind};

mod graph;
pub use graph::{
  Block, BlockKind, CustomDataKind, DEFAULT_INITIAL_CAPACITY, Graph, INVALID_PREDECESSOR_INDEX,
  MIN_BLOCK_CAPACITY, OperationBuffer, Origin,
};

mod opmasks;
pub use opmasks::{MaskBuilder, OP_MASK_OPCODE_BASE, OpMask, OpMaskField};

mod opmasks_named;
pub use opmasks_named::*;

mod fast_hash;
pub use fast_hash::{
  FastHash, fast_hash_combine, fast_hash_combine2, fast_hash_pair, fast_hash_range,
};

mod layered_hash_map;
pub use layered_hash_map::{GROWTH_FACTOR, LayeredHashMap, NEED_RESIZE_PERCENTAGE};

mod sidetable;
pub use sidetable::{
  FixedBlockSidetable, FixedOpIndexSidetable, FixedSidetable, GrowingBlockSidetable,
  GrowingOpIndexSidetable, GrowingSidetable, op_index_belongs_to_table_graph,
};

/// 桥接: 从算子表示映射到 Turboshaft 内存表达 (真实映射与符号性判定)
pub fn machine_rep_to_memory_rep(rep: v8_compiler_ops::LoadRepresentation) -> MemoryRepresentation {
  if rep.is_signed() {
    match rep.representation() {
      MachineRepresentation::Word8 => MemoryRepresentation::Int8,
      MachineRepresentation::Word16 => MemoryRepresentation::Int16,
      MachineRepresentation::Word32 => MemoryRepresentation::Int32,
      MachineRepresentation::Word64 => MemoryRepresentation::Int64,
      other => MemoryRepresentation::from_machine_representation(other),
    }
  } else {
    MemoryRepresentation::from_machine_representation(rep.representation())
  }
}
