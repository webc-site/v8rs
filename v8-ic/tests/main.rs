use aok::{OK, Void};
use log::info;
use strum::IntoEnumIterator;
use v8_ic::{
  AssemblerConfig, AssemblerKind, BinaryOpKind, CacheTable, CallOptimization, CallerFrameType,
  Error, HANDLER_TQ_DECLS, HolderLookup, IC, ICInfo, ICStats, IcKind, InlineCacheState,
  KeyedStoreCheckMap, KeyedStoreIncrementLength, LoadHandlerConfig, LoadHandlerKind,
  NamedPropertyType, PRIMARY_TABLE_SIZE, SECONDARY_TABLE_SIZE, StoreHandlerConfig,
  StoreHandlerKind, StubCache, TransitionMode, UnaryOpKind,
};

#[ctor::ctor(unsafe)]
fn _log_init() {
  log_init::init();
}

#[test]
fn test_ic_state_machine() -> Void {
  assert!(!NamedPropertyType::NotOwn.is_own());
  assert!(NamedPropertyType::Own.is_own());

  assert!(!CallerFrameType::Any.is_baseline());
  assert!(CallerFrameType::Baseline.is_baseline());

  assert!(!KeyedStoreCheckMap::DontCheckMap.should_check_map());
  assert!(KeyedStoreCheckMap::CheckMap.should_check_map());

  assert!(!KeyedStoreIncrementLength::DontIncrementLength.should_increment_length());
  assert!(KeyedStoreIncrementLength::IncrementLength.should_increment_length());

  assert!(!TransitionMode::NoTransition.has_transition());
  assert!(TransitionMode::TransitionToDouble.has_transition());
  assert!(TransitionMode::TransitionToObject.has_transition());

  let states: Vec<_> = InlineCacheState::iter().collect();
  assert_eq!(states.len(), 7);
  assert!(InlineCacheState::Monomorphic.is_monomorphic());
  assert!(InlineCacheState::Polymorphic.is_polymorphic());
  assert!(InlineCacheState::Megamorphic.is_megamorphic());
  assert!(InlineCacheState::Generic.is_generic());

  let mut ic = IC::new(IcKind::LoadIC, CallerFrameType::Baseline);
  assert_eq!(ic.state(), InlineCacheState::Uninitialized);
  assert_eq!(ic.old_state(), InlineCacheState::Uninitialized);
  assert_eq!(ic.kind(), IcKind::LoadIC);
  assert!(ic.caller_frame_type().is_baseline());
  assert!(ic.is_any_load());
  assert!(!ic.is_any_store());

  ic.update_state(InlineCacheState::Monomorphic)?;
  assert_eq!(ic.state(), InlineCacheState::Monomorphic);
  assert_eq!(ic.old_state(), InlineCacheState::Uninitialized);

  ic.mark_recompute_handler();
  assert_eq!(ic.state(), InlineCacheState::RecomputeHandler);
  assert_eq!(ic.old_state(), InlineCacheState::Monomorphic);

  let store_ic = IC::new(IcKind::StoreGlobalIC, CallerFrameType::Any);
  assert!(!store_ic.is_any_load());
  assert!(store_ic.is_any_store());

  OK
}

#[test]
fn test_handler_kinds_and_bitfield() -> Void {
  let load_kinds: Vec<_> = LoadHandlerKind::iter().collect();
  assert_eq!(load_kinds.len(), 16);
  assert!(LoadHandlerKind::Element.is_element());
  assert!(LoadHandlerKind::ElementWithTransition.is_element());
  assert!(LoadHandlerKind::Field.is_field());
  assert!(LoadHandlerKind::Proxy.is_proxy());

  let store_kinds: Vec<_> = StoreHandlerKind::iter().collect();
  assert_eq!(store_kinds.len(), 12);
  assert_eq!(StoreHandlerKind::KINDS_NUMBER, 12);
  assert!(StoreHandlerKind::Field.is_field());
  assert!(StoreHandlerKind::ConstField.is_field());
  assert!(StoreHandlerKind::SharedStructField.is_field());
  assert!(StoreHandlerKind::Proxy.is_proxy());
  assert!(StoreHandlerKind::GlobalProxy.is_proxy());

  // LoadHandlerConfig 编码与解码
  let cfg = LoadHandlerConfig {
    kind: LoadHandlerKind::Field,
    do_access_check_on_lookup_start_object: true,
    lookup_on_lookup_start_object: false,
    descriptor_index: 42,
  };
  let encoded = cfg.encode();
  let decoded = LoadHandlerConfig::decode(encoded)?;
  assert_eq!(cfg, decoded);

  // StoreHandlerConfig 编码与解码
  let s_cfg = StoreHandlerConfig {
    kind: StoreHandlerKind::ConstField,
    is_inobject: true,
    representation: 3,
    storage_offset_in_words: 108,
  };
  let s_encoded = s_cfg.encode();
  let s_decoded = StoreHandlerConfig::decode(s_encoded)?;
  assert_eq!(s_cfg, s_decoded);
  assert!(StoreHandlerConfig::decode(14).is_err());

  assert_eq!(HANDLER_TQ_DECLS.len(), 2);
  assert_eq!(HANDLER_TQ_DECLS[0].1, "LoadHandler");
  assert_eq!(HANDLER_TQ_DECLS[1].1, "StoreHandler");

  OK
}

#[test]
fn test_stub_cache_and_optimization() -> Void {
  assert_eq!(PRIMARY_TABLE_SIZE, 4096);
  assert_eq!(SECONDARY_TABLE_SIZE, 1024);

  let mut cache = StubCache::new();
  assert_eq!(cache.get(0x1234, 0x5678), None);

  // 写入并读取
  cache.set(0x1234, 0x5678, 0xCAFE_BABE);
  assert_eq!(cache.get(0x1234, 0x5678), Some(0xCAFE_BABE));

  // 模拟同一 Primary 槽位冲突
  let conflict_name = 0x1234;
  let conflict_map = 0x5678 ^ (PRIMARY_TABLE_SIZE as u32);
  assert_eq!(
    StubCache::primary_offset(0x1234, 0x5678),
    StubCache::primary_offset(conflict_name, conflict_map)
  );

  cache.set(conflict_name, conflict_map, 0xDEAD_BEEF);
  // 新条目在 Primary
  assert_eq!(cache.get(conflict_name, conflict_map), Some(0xDEAD_BEEF));
  // 老条目被驱逐到了 Secondary 表，依然能命中！
  assert_eq!(cache.get(0x1234, 0x5678), Some(0xCAFE_BABE));

  cache.clear();
  assert_eq!(cache.get(0x1234, 0x5678), None);
  assert_eq!(cache.get(conflict_name, conflict_map), None);

  // 辅助枚举
  assert_eq!(CacheTable::Primary.to_string(), "kPrimary");
  assert_eq!(CacheTable::Secondary.to_string(), "kSecondary");
  assert_eq!(HolderLookup::NotFound.to_string(), "kHolderNotFound");

  // CallOptimization
  let opt = CallOptimization {
    constant_function: Some(0x1000),
    is_simple_api_call: true,
    accept_any_receiver: false,
  };
  assert!(opt.is_constant_call());
  assert!(opt.is_simple_api_call());
  assert!(!opt.accept_any_receiver());

  // ICStats
  let mut stats = ICStats::new();
  assert!(stats.begin());
  stats.record(ICInfo {
    type_name: "LoadIC".into(),
    function_name: "foo".into(),
    ..Default::default()
  });
  stats.end();
  assert_eq!(stats.dump(), "ICStats: recorded 1 events");

  OK
}

#[test]
fn test_assembler_kinds() -> Void {
  let kinds: Vec<_> = AssemblerKind::iter().collect();
  assert_eq!(kinds.len(), 4);
  assert!(AssemblerKind::AccessorAssembler.is_accessor());
  assert!(AssemblerKind::BinaryOpAssembler.is_binary_op());
  assert!(AssemblerKind::UnaryOpAssembler.is_unary_op());
  assert!(AssemblerKind::KeyedStoreGeneric.is_keyed_store());

  let b_kinds: Vec<_> = BinaryOpKind::iter().collect();
  assert_eq!(b_kinds.len(), 11);
  assert!(BinaryOpKind::Add.is_arithmetic());
  assert!(BinaryOpKind::BitwiseAnd.is_bitwise());

  let u_kinds: Vec<_> = UnaryOpKind::iter().collect();
  assert_eq!(u_kinds.len(), 2);

  let cfg = AssemblerConfig::default();
  assert!(!cfg.collect_feedback);
  assert!(!cfg.is_baseline);

  OK
}

#[test]
fn test_error_display() -> Void {
  let err1 = Error::InvalidStateTransition("Monomorphic -> Generic".into());
  assert_eq!(
    err1.to_string(),
    "Invalid IC state transition: Monomorphic -> Generic"
  );

  let err2 = Error::HandlerBitfieldOverflow("DescriptorBits > 12".into());
  assert_eq!(
    err2.to_string(),
    "Handler bitfield encoding overflow: DescriptorBits > 12"
  );

  let err3 = Error::StubCacheMiss(0x4000, "prop".into());
  assert_eq!(
    err3.to_string(),
    "StubCache lookup failure: map 0x4000, name prop"
  );

  let err4 = Error::UnsupportedHandlerKind("Unknown 99".into());
  assert_eq!(err4.to_string(), "Unsupported IC handler kind: Unknown 99");

  info!("All v8-ic tests passed cleanly");
  OK
}
