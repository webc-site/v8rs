#![cfg_attr(docsrs, feature(doc_cfg))]

//! v8/src/flags 的 Rust 转写 — V8 flag 系统 (998 主 flag, 规范配置 macOS arm64 release)
//!
//! 分层:
//! - 生成数据层: defs / implications_gen / contradictions_gen / features_gen / counts
//!   (由 tool/src/flags/*.js 从 v8 源机械展开, 对数门禁三方核对)
//! - 运行时层: value (存储) / lookup (查找) / flags (解析·蕴含·矛盾·哈希·打印) /
//!   save (快照) / features (特性枚举) / base (自含辅助)

mod base;
mod contradictions_gen;
mod counts;
mod defs;
mod error;
mod features;
mod features_gen;
mod flags;
mod implications_gen;
mod lookup;
mod save;
mod value;

pub use defs::{ALL_ENTRIES, EntryMeta, FLAGS, FlagDef};
pub use error::{Error, Result};
pub use features::{FeatureKind, FeatureStage, feature_flags_json, features};
pub use features_gen::Feature;
pub use flags::{HELP_SYNTAX, TRACING, Tracing};
pub use lookup::{
  FlagId, cmp_names, equal_name_with_suffix, equal_names, find_flag_by_name, find_implication_flag,
};
pub use save::SavedFlags;
pub use value::{Cow, FlagProcessingMode, FlagType, FlagValue, Flags, SetBy, is_any_implication};

// ── 生成表实数 (对数门禁: 表实数 == counts 基线 == tool/src/flags/stats.json) ──
pub const NUM_CONTRADICTIONS: usize = contradictions_gen::CONTRADICTIONS.len();

/// 对数基线常量 (与 tool/src/flags/stats.json 同步生成; 表实数核对见编译期断言)
pub use counts::{
  CANON_ALIASES, CANON_ALL_ENTRIES, CANON_CONTRADICTIONS, CANON_CONTRADICTIONS_WASM_SCOPED,
  CANON_FEATURES, CANON_IMPLICATIONS, CANON_KIND_GENERIC, CANON_KIND_REQUIRE, CANON_KIND_VALUE,
  CANON_KIND_WARN, CANON_PRIMARY, CANON_READONLY, CANON_RESET_CORRECTNESS, CANON_RESET_FUZZING,
  CANON_TYPE_BOOL, CANON_TYPE_FLOAT, CANON_TYPE_INT, CANON_TYPE_MAYBE_BOOL, CANON_TYPE_SIZE_T,
  CANON_TYPE_STRING, CANON_TYPE_UINT, CANON_TYPE_UINT64, RAW_DEFINE_ALIAS, RAW_DEFINE_BOOL,
  RAW_DEFINE_DEBUG_BOOL, RAW_DEFINE_DEBUG_DEVELOPER_FLAG, RAW_DEFINE_DEVELOPER_FLAG,
  RAW_DEFINE_EXPERIMENTAL_FEATURE, RAW_DEFINE_FLOAT, RAW_DEFINE_INT, RAW_DEFINE_MAYBE_BOOL,
  RAW_DEFINE_POSITIVE_SMI, RAW_DEFINE_SIZE_T, RAW_DEFINE_SMI, RAW_DEFINE_STRING, RAW_DEFINE_UINT,
  RAW_DEFINE_UINT64, RAW_DEFINE_UNSAFE_FLAG, RAW_GENERIC, RAW_IMPLICATION_TOTAL, RAW_REQUIREMENT,
};
pub use defs::{NUM_ALL_FLAGS, NUM_FLAGS};
pub use features_gen::NUM_FEATURES;
pub use implications_gen::{NUM_GENERICS, NUM_IMPLICATIONS, NUM_REQUIRES, NUM_WARNINGS};

// ── 编译期对数断言: 生成表实数 == counts 基线 (零运行时成本; 第三方
//    tool/src/flags/stats.json 由 tests/main.rs 的 counts_consistency 核对) ──

/// 编译期逐字节串比较 (const 上下文 str == 不稳定, 手写)
const fn str_eq(a: &str, b: &str) -> bool {
  let (a, b) = (a.as_bytes(), b.as_bytes());
  if a.len() != b.len() {
    return false;
  }
  let mut i = 0;
  while i < a.len() {
    if a[i] != b[i] {
      return false;
    }
    i += 1;
  }
  true
}

/// 编译期统计只读主 flag 数
const fn count_readonly(flags: &[defs::FlagDef]) -> usize {
  let mut n = 0;
  let mut i = 0;
  while i < flags.len() {
    if flags[i].readonly {
      n += 1;
    }
    i += 1;
  }
  n
}

/// 编译期按类型统计主 flag 数
const fn count_ty(flags: &[defs::FlagDef], want: value::TyTag) -> usize {
  let mut n = 0;
  let mut i = 0;
  while i < flags.len() {
    if flags[i].ty.tag().is(want) {
      n += 1;
    }
    i += 1;
  }
  n
}

/// 编译期按蕴含种类统计
const fn count_kind(imps: &[flags::Implication], want: flags::ImpKindTag) -> usize {
  let mut n = 0;
  let mut i = 0;
  while i < imps.len() {
    if imps[i].kind.tag().is(want) {
      n += 1;
    }
    i += 1;
  }
  n
}

/// 编译期核对: 矛盾表左侧 (被重置方) 不含受保护 flag (原版逐条 CHECK)
const fn contradictions_unguarded() -> bool {
  let guarded = flags::CONTRADICTION_GUARDED;
  let mut i = 0;
  while i < contradictions_gen::CONTRADICTIONS.len() {
    let (a, _) = contradictions_gen::CONTRADICTIONS[i];
    let name = defs::FLAGS[a as usize].name;
    let mut g = 0;
    while g < guarded.len() {
      if str_eq(name, guarded[g]) {
        return false;
      }
      g += 1;
    }
    i += 1;
  }
  true
}

const _: () = assert!(
  defs::FLAGS.len() == counts::CANON_PRIMARY,
  "主 flag 表对数失败"
);
const _: () = assert!(
  defs::ALL_ENTRIES.len() - defs::FLAGS.len() == counts::CANON_ALIASES,
  "别名数对数失败"
);
const _: () = assert!(
  defs::ALL_ENTRIES.len() == counts::CANON_ALL_ENTRIES,
  "全条目表对数失败"
);
const _: () = assert!(
  count_readonly(defs::FLAGS) == counts::CANON_READONLY,
  "只读数对数失败"
);
const _: () = assert!(count_ty(defs::FLAGS, value::TyTag::Bool) == counts::CANON_TYPE_BOOL);
const _: () =
  assert!(count_ty(defs::FLAGS, value::TyTag::MaybeBool) == counts::CANON_TYPE_MAYBE_BOOL);
const _: () = assert!(count_ty(defs::FLAGS, value::TyTag::Int) == counts::CANON_TYPE_INT);
const _: () = assert!(count_ty(defs::FLAGS, value::TyTag::Uint) == counts::CANON_TYPE_UINT);
const _: () = assert!(count_ty(defs::FLAGS, value::TyTag::Uint64) == counts::CANON_TYPE_UINT64);
const _: () = assert!(count_ty(defs::FLAGS, value::TyTag::Float) == counts::CANON_TYPE_FLOAT);
const _: () = assert!(count_ty(defs::FLAGS, value::TyTag::SizeT) == counts::CANON_TYPE_SIZE_T);
const _: () = assert!(count_ty(defs::FLAGS, value::TyTag::Str) == counts::CANON_TYPE_STRING);
const _: () = assert!(
  implications_gen::IMPLICATIONS.len() == counts::CANON_IMPLICATIONS,
  "蕴含表对数失败"
);
const _: () = assert!(
  count_kind(implications_gen::IMPLICATIONS, flags::ImpKindTag::Value) == counts::CANON_KIND_VALUE
);
const _: () = assert!(
  count_kind(implications_gen::IMPLICATIONS, flags::ImpKindTag::Warn) == counts::CANON_KIND_WARN
);
const _: () = assert!(
  count_kind(implications_gen::IMPLICATIONS, flags::ImpKindTag::Generic)
    == counts::CANON_KIND_GENERIC
);
const _: () = assert!(
  count_kind(implications_gen::IMPLICATIONS, flags::ImpKindTag::Require)
    == counts::CANON_KIND_REQUIRE
);
const _: () = assert!(
  features_gen::FEATURES.len() == counts::CANON_FEATURES,
  "特性表对数失败"
);
const _: () = assert!(
  contradictions_gen::CONTRADICTIONS.len() == counts::CANON_CONTRADICTIONS,
  "矛盾表对数失败"
);
const _: () = assert!(contradictions_unguarded(), "矛盾表含受保护 flag");
const _: () = assert!(
  defs::SORTED_ALL.len() == defs::ALL_ENTRIES.len(),
  "排序表对数失败"
);
const _: () = assert!(
  defs::PRIMARY_TO_ALL.len() == defs::FLAGS.len(),
  "主索引映射对数失败"
);
