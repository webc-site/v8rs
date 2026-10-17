#![cfg_attr(docsrs, feature(doc_cfg))]

mod error;
pub use error::{Error, Result};

/// 字符谓词 (v8/src/strings/char-predicates.*)。
pub mod char_predicates;
pub mod char_predicates_cc_generated;
/// Name::raw_hash_field 位域 (v8/src/objects/name.h, 越界落地)。
pub mod hash_field;
/// 字符串哈希种子 (v8/src/numbers/hash-seed.*, 越界落地)。
pub mod hash_seed;
/// 独占外部字符串资源 (v8/src/strings/owning-external-string-resource.h)。
pub mod owning_external_string_resource_generated;
/// rapidhash (third_party/rapidhash-v8)。
pub mod rapidhash;
/// 字符串构建器 (v8/src/strings/string-builder.*)。
pub mod string_builder_generated;
/// ASCII 大小写快速转换 (v8/src/strings/string-case.*)。
pub mod string_case;
/// 字符串哈希 (v8/src/strings/string-hasher.*)。
pub mod string_hasher;
pub mod string_hasher_inl_generated;
/// 字符串搜索 (v8/src/strings/string-search.h)。
pub mod string_search;
/// 诊断字符串流 (v8/src/strings/string-stream.*)。
pub mod string_stream;
/// Unicode 工具 (v8/src/strings/unicode.*)。
pub mod unicode;
pub mod unicode_cc_generated;
/// UTF-8 解码器 (v8/src/strings/unicode-decoder.*)。
pub mod unicode_decoder;
/// URI 编解码内核 (v8/src/strings/uri.*)。
pub mod uri;
/// UTF-8 DFA (third_party/utf8-decoder)。
pub mod utf8_dfa;
