#![cfg_attr(docsrs, feature(doc_cfg))]
//! v8-compiler-ops — V8 优化编译器算子体系 (Common, Machine, Simplified, JS 与 Verifier)

mod common_operator;
mod error;
mod js_operator;
mod machine_operator;
mod machine_pair_operator;
mod simplified_operator;
mod verifier;

pub use common_operator::{
  BranchHint, BranchParameters, BranchSemantics, CommonOperatorBuilder, CommonOperatorGlobalCache,
  DeoptimizeParameters, DeoptimizeReason, IfValueParameters, ParameterInfo,
};
pub use error::{Error, Result};
pub use js_operator::{
  CallParameters, ConstructParameters, JSOperatorBuilder, JSOperatorGlobalCache,
};
pub use machine_operator::{
  LoadRepresentation, MachineOperatorBuilder, MachineOperatorGlobalCache, StoreRepresentation,
  WriteBarrierKind,
};
pub use machine_pair_operator::MachinePairOperatorGlobalCache;
pub use simplified_operator::{
  CheckBoundsFlags, CheckBoundsParameters, CheckMapsFlags, CheckMapsParameters,
  SimplifiedOperatorBuilder, SimplifiedOperatorGlobalCache,
};
pub use verifier::Verifier;
