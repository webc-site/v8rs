use v8_strings::{
  char_predicates_cc_generated::char_predicates_slow_skel,
  hash_seed::HashSeedData,
  owning_external_string_resource_generated::owning_resource_skel::{
    OwningExternalOneByteStringResource, OwningExternalStringResource, WIDTH_ASSERTS,
  },
  string_builder_generated::{
    FixedArrayBuilder, IncrementalStringBuilder, ReplacementStringBuilder, StringBuilderEncoding,
    fixed_array_builder_skel, incremental_string_builder_skel, replacement_string_builder_skel,
    string_builder_const,
  },
  string_hasher_inl_generated::string_hasher_inl_skel::{
    ZERO_HASH, add_to_hash, compute_hash_field, convert_to_8bit_reader,
    decode_array_index_from_hash_field, get_trivial_hash, hash_converting_to_8bit,
    hash_sequential_string, is_only_8bit_simd, make_array_index_hash, running_add_character,
  },
  unicode_cc_generated::{
    unibrow_cache_const::CacheEntry,
    unibrow_cache_skel::{mapping_get, predicate_get},
    unicode_codec_skel::{
      utf8_calculate_value, utf8_validate_encoding, utf8_value_of_incremental,
      utf8_value_of_incremental_finish, utf8_write_leading_ascii, utf16_has_unpaired_surrogate,
      utf16_replace_unpaired_surrogates, wtf8_scan_for_surrogates, wtf8_validate_encoding,
    },
    unicode_families,
    unicode_table_coding::{is_start, lookup_mapping, lookup_predicate},
  },
};

#[test]
fn test_char_predicates_ground_truth() {
  assert_eq!(char_predicates_slow_skel::SLOW_PREDICATES.len(), 3);
  assert_eq!(char_predicates_slow_skel::ICU_DEPS.len(), 3);

  // Identifier Start
  assert!(char_predicates_slow_skel::is_identifier_start_slow(
    b'$' as u32
  ));
  assert!(char_predicates_slow_skel::is_identifier_start_slow(
    b'_' as u32
  ));
  assert!(char_predicates_slow_skel::is_identifier_start_slow(
    b'\\' as u32
  ));
  assert!(char_predicates_slow_skel::is_identifier_start_slow(0x00E9)); // 'é'
  assert!(!char_predicates_slow_skel::is_identifier_start_slow(
    b'0' as u32
  ));

  // Identifier Part
  assert!(char_predicates_slow_skel::is_identifier_part_slow(
    b'$' as u32
  ));
  assert!(char_predicates_slow_skel::is_identifier_part_slow(
    b'0' as u32
  ));
  assert!(char_predicates_slow_skel::is_identifier_part_slow(0x200C)); // ZWNJ
  assert!(char_predicates_slow_skel::is_identifier_part_slow(0x200D)); // ZWJ

  // White Space
  assert!(char_predicates_slow_skel::is_white_space_slow(0x0009)); // TAB
  assert!(char_predicates_slow_skel::is_white_space_slow(0x000B)); // VT
  assert!(char_predicates_slow_skel::is_white_space_slow(0x000C)); // FF
  assert!(char_predicates_slow_skel::is_white_space_slow(0x0020)); // Space
  assert!(char_predicates_slow_skel::is_white_space_slow(0x00A0)); // NBSP
  assert!(char_predicates_slow_skel::is_white_space_slow(0xFEFF)); // BOM
  assert!(!char_predicates_slow_skel::is_white_space_slow(b'A' as u32));
}

#[test]
fn test_string_hasher_inl_ground_truth() {
  assert_eq!(ZERO_HASH, 27);
  assert_eq!(convert_to_8bit_reader::COMPRESSION_FACTOR, 2);
  assert_eq!(convert_to_8bit_reader::EXPANSION_FACTOR, 1);
  assert_eq!(convert_to_8bit_reader::READERS.len(), 3);
  assert_eq!(convert_to_8bit_reader::SIMD_BRANCHES.len(), 2);

  // Trivial hash
  let triv = get_trivial_hash(100);
  assert_ne!(triv, 0);

  // Array index hash encode & decode roundtrip
  let raw_idx = make_array_index_hash(12345, 5);
  let decoded = decode_array_index_from_hash_field(raw_idx);
  assert_eq!(decoded, 12345);

  // Sequential string hash
  let hash1 = hash_sequential_string(b"hello", 5, 0x12345678);
  assert_ne!(hash1, 0);

  // Add to hash
  let mixed = add_to_hash(0x12345678, 0x9ABCDEF0);
  assert_ne!(mixed, 0);

  // Running add character
  let mut running = 0u32;
  running_add_character(&mut running, b'a' as u16);
  assert_ne!(running, 0);

  // Compute hash field
  let field = compute_hash_field(0xdeadbeef, 0x1234);
  assert_ne!(field, 0);

  // 8-bit detection & converting hash
  let u16_latin1 = [b'h' as u16, b'i' as u16];
  assert!(is_only_8bit_simd(&u16_latin1, 2));

  let u16_non_latin1 = [0x4e2d, 0x6587]; // "中文"
  assert!(!is_only_8bit_simd(&u16_non_latin1, 2));

  let seed_data = HashSeedData::from_seed(0x123456);
  let conv_hash = hash_converting_to_8bit(&u16_latin1, 2, seed_data.seed, &seed_data.secrets);
  assert_ne!(conv_hash, 0);
}

#[test]
fn test_owning_external_string_resource() {
  assert_eq!(WIDTH_ASSERTS, 2);

  let one_byte_data = b"Hello, V8!".to_vec();
  let res1 = OwningExternalOneByteStringResource::new(&one_byte_data);
  assert_eq!(res1.length(), 10);
  assert!(!res1.is_empty());
  assert_eq!(res1.data(), b"Hello, V8!");

  let two_byte_data = [0x4e2du16, 0x6587u16];
  let res2 = OwningExternalStringResource::new(&two_byte_data);
  assert_eq!(res2.length(), 2);
  assert_eq!(res2.data(), &[0x4e2d, 0x6587]);

  let res_cap = OwningExternalOneByteStringResource::with_capacity(32);
  assert_eq!(res_cap.length(), 32);

  let empty_res = OwningExternalOneByteStringResource::with_capacity(0);
  assert!(empty_res.is_empty());
  assert_eq!(empty_res.length(), 0);
  assert_eq!(empty_res.data(), &[]);
}

#[test]
fn test_unicode_cc_ground_truth() {
  assert_eq!(unicode_families::PREDICATES.len(), 5);
  assert_eq!(unicode_families::CONVERTERS.len(), 5);

  // Lookup predicate
  let dummy_table = [1073741889, 90]; // start bit + value
  assert!(is_start(dummy_table[0]));
  assert!(lookup_predicate(&dummy_table, 2, 65)); // 'A'
  assert!(!lookup_predicate(&dummy_table, 0, 65)); // empty size

  // Lookup mapping
  let mapping_table = [1073741889, 4];
  let mapped = lookup_mapping(&mapping_table, 1, 65);
  assert_eq!(mapped, 66); // offset 1
  let unmapped = lookup_mapping(&mapping_table, 0, 65);
  assert_eq!(unmapped, 65);

  // Codecs
  let utf8_sample = "Hello, 世界!".as_bytes();
  assert_eq!(
    utf8_calculate_value(utf8_sample, utf8_sample.len()),
    'H' as u32
  );
  assert!(utf8_validate_encoding(utf8_sample));

  // Incremental utf8
  let mut cursor = utf8_sample;
  let mut state = 0u8;
  let mut buffer = 0u32;
  let codepoint = utf8_value_of_incremental(&mut cursor, &mut state, &mut buffer);
  assert_eq!(codepoint, 'H' as u32);
  let finish = utf8_value_of_incremental_finish(&mut state);
  assert_eq!(finish, 0);

  // UTF-16 surrogates: comprehensive edge cases
  let valid_utf16 = [0xd83d, 0xde00]; // Emoji grinning face
  assert!(!utf16_has_unpaired_surrogate(&valid_utf16));

  let unpaired_utf16 = [0xd83d, b'A' as u16];
  assert!(utf16_has_unpaired_surrogate(&unpaired_utf16));

  let mut to_replace = unpaired_utf16;
  utf16_replace_unpaired_surrogates(&mut to_replace);
  assert_eq!(to_replace[0], 0xfffd);
  assert_eq!(to_replace[1], b'A' as u16);

  // Trailing unpaired surrogate and mixed valid pair
  let mut mixed_surrogates = [0xd83d, 0xde00, 0xd800]; // valid pair followed by lone high surrogate
  utf16_replace_unpaired_surrogates(&mut mixed_surrogates);
  assert_eq!(mixed_surrogates, [0xd83d, 0xde00, 0xfffd]);

  // Lone low surrogate at beginning
  let mut lone_low = [0xdc00, b'x' as u16];
  utf16_replace_unpaired_surrogates(&mut lone_low);
  assert_eq!(lone_low, [0xfffd, b'x' as u16]);

  // WTF-8
  assert!(wtf8_validate_encoding(utf8_sample));
  let wtf8_with_surrogate = [0xed, 0xa0, 0x80];
  let surrogates = wtf8_scan_for_surrogates(&wtf8_with_surrogate);
  assert_eq!(surrogates, vec![0]);

  // Leading ascii
  let mut dst = [0u8; 16];
  let written = utf8_write_leading_ascii(b"abc\x80def", &mut dst);
  assert_eq!(written, 3);
  assert_eq!(&dst[..3], b"abc");

  // Cache: single char lowercase
  let mut pred_cache = [CacheEntry::default(); 16];
  let pred = predicate_get(&mut pred_cache, 'A' as u32);
  assert!(pred);
  let pred_cached = predicate_get(&mut pred_cache, 'A' as u32);
  assert!(pred_cached);

  let mut map_cache = [CacheEntry::default(); 16];
  let mut map_res = [0u32; 4];
  let len = mapping_get(&mut map_cache, 'A' as u32, 0, &mut map_res);
  assert_eq!(len, 1);
  assert_eq!(map_res[0], 'a' as u32);
  // Cache hit
  let len_cached = mapping_get(&mut map_cache, 'A' as u32, 0, &mut map_res);
  assert_eq!(len_cached, 1);
  assert_eq!(map_res[0], 'a' as u32);

  // Cache: multi-char lowercase expansion ('İ' U+0130 -> 'i' + dot above U+0307)
  let mut multi_map_cache = [CacheEntry::default(); 16];
  let mut multi_res = [0u32; 4];
  let multi_len = mapping_get(&mut multi_map_cache, 0x0130, 0, &mut multi_res);
  assert_eq!(multi_len, 2);
  assert_eq!(multi_res[0], 'i' as u32);
  assert_eq!(multi_res[1], 0x0307);
}

#[test]
fn test_string_builder_ground_truth() {
  assert_eq!(string_builder_const::INITIAL_PART_LENGTH, 32);
  assert_eq!(string_builder_const::MAX_PART_LENGTH, 16 * 1024);

  // FixedArrayBuilder
  let mut fab = FixedArrayBuilder::new(16);
  assert_eq!(fab.length(), 0);
  assert!(fab.has_capacity(16));
  assert!(!fab.has_non_smi_elements());
  fab.add_smi(42);
  assert!(!fab.has_non_smi_elements());
  fab.add_object(100);
  assert!(fab.has_non_smi_elements());
  assert_eq!(fab.length(), 2);
  assert_eq!(fab.as_slice(), &[42, 100]);

  // Trigger capacity doubling in FixedArrayBuilder
  for i in 0..30 {
    fab.add(i);
  }
  assert_eq!(fab.length(), 32);
  assert!(fab.capacity() >= 32);

  let mut lazy_fab = FixedArrayBuilder::lazy();
  assert_eq!(lazy_fab.length(), 0);
  lazy_fab.add(7);
  assert_eq!(lazy_fab.length(), 1);

  // ReplacementStringBuilder
  let mut rsb = ReplacementStringBuilder::new("hello world".to_string(), 4);
  assert!(rsb.is_one_byte());
  rsb.add_subject_slice(0, 5); // "hello"
  rsb.add_str(" beautiful ");
  rsb.add_subject_slice(6, 11); // "world"
  let replaced = rsb.to_string();
  assert_eq!(replaced, "hello beautiful world");

  // IncrementalStringBuilder
  let mut isb = IncrementalStringBuilder::new();
  assert_eq!(isb.current_encoding(), StringBuilderEncoding::OneByte as u8);
  assert!(isb.is_empty());

  isb.append_str("Hello");
  isb.append_character(b' ');
  isb.append_int(12345);
  isb.append_str(" ");
  isb.append_int(-999);
  isb.append_str(" ");
  isb.append_string_capped("long string that exceeds limit", 4);

  assert_eq!(isb.to_string(), "Hello 12345 -999 long<...>");
  assert!(!isb.has_overflowed());

  // Integer boundary formatting
  let mut isb_int = IncrementalStringBuilder::new();
  isb_int.append_int(i32::MIN);
  assert_eq!(isb_int.to_string(), "-2147483648");

  let mut isb_max = IncrementalStringBuilder::new();
  isb_max.append_int(i32::MAX);
  assert_eq!(isb_max.to_string(), "2147483647");

  // Multi-part growth across part_length boundaries
  let mut isb_long = IncrementalStringBuilder::new();
  let repeat_chunk = "0123456789abcdef"; // 16 bytes
  for _ in 0..10 {
    isb_long.append_str(repeat_chunk);
  }
  assert_eq!(isb_long.length(), 160);
  assert_eq!(isb_long.to_string().len(), 160);

  // Encoding upgrade
  let mut isb_wide = IncrementalStringBuilder::new();
  isb_wide.append_str("ASCII prefix ");
  isb_wide.append_u16(0x4e2d); // '中'
  assert_eq!(
    isb_wide.current_encoding(),
    StringBuilderEncoding::TwoByte as u8
  );
  assert_eq!(isb_wide.to_string(), "ASCII prefix 中");

  // Skel modules non-panicking verification
  assert!(fixed_array_builder_skel::has_capacity(8));
  fixed_array_builder_skel::ensure_capacity(0, 16);
  fixed_array_builder_skel::add(123);
  fixed_array_builder_skel::lazy(0);

  replacement_string_builder_skel::add_subject_slice(0, 0, 5);
  replacement_string_builder_skel::add_string(0);
  assert_eq!(replacement_string_builder_skel::to_string(), Some(0));
  replacement_string_builder_skel::increment_character_count(5);

  assert_eq!(incremental_string_builder_skel::current_encoding(), 1);
  incremental_string_builder_skel::append_character(b'a');
  incremental_string_builder_skel::append_str("test");
  incremental_string_builder_skel::append_string_capped(0, 10);
  assert!(incremental_string_builder_skel::current_part_can_fit(
    32, 0, 10
  ));
  assert_eq!(
    incremental_string_builder_skel::escaped_length_if_current_part_fits(10),
    80
  );
  assert_eq!(incremental_string_builder_skel::finish(), Some(0));
  incremental_string_builder_skel::change_encoding();
  incremental_string_builder_skel::accumulate(0);
  incremental_string_builder_skel::extend();
  incremental_string_builder_skel::shrink_current_part();
}
