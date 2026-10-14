#![forbid(unsafe_code)]

mod code_assembler;
mod code_stub_assembler;
mod error;
mod tnode;

pub use code_assembler::{
  BinaryOp, BuiltinKind, CallInterfaceDescriptor, CodeAssembler, CodeAssemblerLabel,
  CodeAssemblerState, CodeAssemblerVariable, CompareBinaryOp, LabelState, MachineRep, NodeOp,
  NodeRecord, UnaryOp,
};
pub use code_stub_assembler::{
  AllocationFlag, AllocationFlags, CodeStubAssembler, ExtractFixedArrayFlag, HoleConversionMode,
  PrimitiveType, SlackTrackingMode,
};
pub use error::{Error, Result};
pub use tnode::{
  BoolT, ContextTag, DescriptorArray, ExternalPointerT, False, FixedArray, Float32T, Float64T,
  HeapObject, Hole, Int8T, Int16T, Int32T, Int64T, IntPtrT, IntegralT, JSArray, JSObject,
  JSReceiver, Map, NodeId, Null, Object, Oddball, RawPtrT, SandboxedPtrT, Smi, StringTag, TNode,
  TaggedIndex, True, TypeTag, Uint8T, Uint16T, Uint32T, Uint64T, UintPtrT, Undefined, UntaggedT,
  Word32T, Word64T, WordT,
};
