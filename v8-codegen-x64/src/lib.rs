#![cfg_attr(docsrs, feature(doc_cfg))]

pub mod assembler;
pub mod constants;
pub mod error;
pub mod operand;
pub mod registers;

pub use assembler::{
  AddqOp, AndqOp, Assembler, CallOp, CmpqOp, Condition, ImulqOp, JmpOp, MovlOp, MovqOp, OrqOp,
  PopqOp, PushqOp, SubqOp, TestqOp, XorqOp,
};
pub use constants::{K_MAX_PC_RELATIVE_CODE_RANGE_IN_MB, K_ROOT_REGISTER_BIAS};
pub use error::{Error, Result};
pub use operand::{Immediate, Operand, ScaleFactor};
pub use registers::{
  DoubleRegister, FloatRegister, NO_DREG, NO_REG, R8, R9, R10, R11, R12, R13, R14, R15, RAX, RBP,
  RBX, RCX, RDI, RDX, RSI, RSP, RegList, Register, Simd128Register, Simd256Register,
  TaggedRegister, XMM0, XMM1, XMM2, XMM3, XMM4, XMM5, XMM6, XMM7, XMM8, XMM9, XMM10, XMM11, XMM12,
  XMM13, XMM14, XMM15, XMMRegister, YMMRegister, argument_padding_slots, register_name,
  xmm_register_name, ymm_register_name,
};
// 分期未接线: disasm_x64_generated / assembler_x64_generated / macro_assembler /
// interface_descriptors / reglist / cpu / unwinder / fma_instr / sse_instr
