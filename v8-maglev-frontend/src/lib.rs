//! v8/src/maglev 前端族的 1:1 转写 crate (Wave 9 席位 W9MF)。
//!
//! 覆盖面 (target/wave9/W9MF.txt, 25 原版文件): hamt / graph / graph-builder /
//! graph-optimizer / graph-printer / graph-labeller / graph-processor /
//! graph-verifier / inlining / interpreter-frame-state / kna-processor /
//! known-node-aspects / loop-peeler / cse / deopt-frame-visitor。
//! 破环: 编译宿主 (MaglevCompilationInfo/Isolate 面) 以 seam 承接, 深水登记 readme/zh.md 台账。

#![cfg_attr(docsrs, feature(doc_cfg))]

mod error;
pub use error::{Error, Result};

mod hamt;
pub use hamt::{
  BranchHeader, HAMT_BITS, HAMT_MASK, HAMT_MAX_SHIFT, HAMT_WIDTH, Hamt, HamtHashable,
  Iterator as HamtIterator, Leaf, Node as HamtNode, NodeType as HamtNodeType,
  StackEntry as HamtStackEntry,
};

mod frame_state;
pub use frame_state::{BasicBlockType, ContextScopeInfo, MergePointCounters, MergePointFrameRef};

mod labeller;
pub use labeller::{MaglevGraphLabeller, NodeInfo, Provenance};

mod graph;
pub use graph::{
  BasicBlockRec, BlockRef, CallSiteRef, CompilationUnitRef, ConstantValue, DeoptFrameRef, Graph,
  InlinedAllocationRef, InlinedFunctionRef, InliningTreeDebugInfo, K_MAX_UINT32,
  MaglevLoopPeelInfo, NO_INPUT, NodeRec, NodeRef, OSRValue, RootIndex,
};
