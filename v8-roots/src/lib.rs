#![cfg_attr(docsrs, feature(doc_cfg))]

mod error;
pub mod read_only_roots;
pub mod root_index;
pub mod roots_table;

pub use error::{Error, Result};
pub use read_only_roots::ReadOnlyRoots;
pub use root_index::{ROOT_NAMES, RootIndex};
pub use roots_table::RootsTable;
