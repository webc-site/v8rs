#![forbid(unsafe_code)]

mod assembler;
mod error;
mod handler;
mod ic;
mod stub_cache;

pub use assembler::{AssemblerConfig, AssemblerKind, BinaryOpKind, UnaryOpKind};
pub use error::{Error, Result};
pub use handler::{
  HANDLER_TQ_DECLS, LoadHandlerConfig, LoadHandlerKind, StoreHandlerConfig, StoreHandlerKind,
};
pub use ic::{
  CallerFrameType, IC, IcFeedbackEnv, IcKind, IcMapEnv, InlineCacheState, KeyedStoreCheckMap,
  KeyedStoreIncrementLength, NamedPropertyType, TransitionMode,
};
pub use stub_cache::{
  CacheTable, CallOptimization, HolderLookup, ICInfo, ICStats, MAX_IC_INFO, PRIMARY_TABLE_BITS,
  PRIMARY_TABLE_SIZE, SECONDARY_TABLE_BITS, SECONDARY_TABLE_SIZE, StubCache, StubCacheEntry,
};
