// 迁移自 v8/test/unittests/objects/object-unittest.cc:
//   TEST(Object, InstanceTypeList) / InstanceTypeListOrder / StructListOrder
// + instance-type.h 的 static_assert 锚点 (232-292) 规约化
// + casting.h 不可承载部分 (isolate 依赖) 跳过登记于脚本头
use aok::{OK, Void};
use v8_objects_core::instance_type::{
  self as it, FIRST_JS_RECEIVER_TYPE, FIRST_NONSTRING_TYPE, FIRST_STRUCT_TYPE, FIRST_TYPE,
  InstanceType, LAST_CUSTOM_ELEMENTS_RECEIVER, LAST_JS_FUNCTION_TYPE, LAST_SPECIAL_RECEIVER_TYPE,
  LAST_STRING_TYPE, LAST_TYPE, LAST_TYPE_VALUE, STRING_TYPE_COUNT, VARIANT_COUNT,
};

#[ctor::ctor(unsafe)]
fn init() {
  log_init::init();
}

/// 规约 (object-unittest.cc:24-48): 字符串类型全部命中 IsString 位掩码,
/// 非字符串全部不命中; 枚举判别值连续覆盖 0..LAST_TYPE 的全部实心点。
#[test]
fn object_instance_type_list() -> Void {
  let mut solid = 0usize;
  for raw in 0..=LAST_TYPE_VALUE {
    if let Ok(ty) = InstanceType::from_raw(raw) {
      solid += 1;
      let expect = raw < FIRST_NONSTRING_TYPE.raw();
      assert_eq!(ty.is_string(), expect, "IsString 掩码失配 @ {raw}");
      // 位掩码与区间判定的交叉一致性 (kIsNotStringMask == bit7)
      assert_eq!(
        (raw as u32 & it::K_IS_NOT_STRING_MASK) == it::K_STRING_TAG,
        expect
      );
    }
  }
  assert_eq!(solid, VARIANT_COUNT, "实心判别值数应等于变体数");
  assert_eq!(STRING_TYPE_COUNT, 24);
  OK
}

/// 规约 (object-unittest.cc:59-83): 手写字符串段严格递增且从 0 起。
#[test]
fn object_instance_type_list_order() -> Void {
  use InstanceType::*;
  // INSTANCE_TYPE_LIST_BASE (objects-definitions.h:31-55) 声明序
  let strings = [
    InternalizedTwoByteString,
    ExternalInternalizedTwoByteString,
    InternalizedOneByteString,
    ExternalInternalizedOneByteString,
    UncachedExternalInternalizedTwoByteString,
    UncachedExternalInternalizedOneByteString,
    SeqTwoByteString,
    ConsTwoByteString,
    ExternalTwoByteString,
    SlicedTwoByteString,
    ThinTwoByteString,
    SeqOneByteString,
    ConsOneByteString,
    ExternalOneByteString,
    SlicedOneByteString,
    ThinOneByteString,
    UncachedExternalTwoByteString,
    UncachedExternalOneByteString,
    SharedSeqTwoByteString,
    SharedExternalTwoByteString,
    SharedSeqOneByteString,
    SharedExternalOneByteString,
    SharedUncachedExternalTwoByteString,
    SharedUncachedExternalOneByteString,
  ];
  assert_eq!(FIRST_TYPE.raw(), 0);
  assert_eq!(strings[0].raw(), FIRST_TYPE.raw());
  for w in strings.windows(2) {
    assert!(
      w[0].raw() < w[1].raw(),
      "字符串段失序: {} !< {}",
      w[0].raw(),
      w[1].raw()
    );
  }
  assert_eq!(LAST_STRING_TYPE, FIRST_NONSTRING_TYPE.raw() - 1);
  OK
}

/// 规约 (object-unittest.cc:85-101): STRUCT_LIST 严格递增, 首项 = FIRST_STRUCT_TYPE。
#[test]
fn object_struct_list_order() -> Void {
  use InstanceType::*;
  // STRUCT_LIST_GENERATOR (objects-definitions.h:138-192) 顺序
  let structs = [
    PromiseFulfillReactionJobTask,
    PromiseRejectReactionJobTask,
    AsyncResumeTask,
    CallableTask,
    CallbackTask,
    PromiseResolveThenableJobTask,
    AccessCheckInfo,
    AccessorPair,
    AliasedArgumentsEntry,
    AllocationMemento,
    ArrayBoilerplateDescription,
    AsyncGeneratorRequest,
    BreakPoint,
    BreakPointInfo,
    BytecodeWrapper,
    CallSiteInfo,
    ClassBoilerplate,
    ClassPositions,
    CodeWrapper,
    DebugScriptScopeInfo,
    EnumCache,
    ErrorStackData,
    FeedbackCell,
    ForInEnumeratorHolder,
    FunctionTemplateRareData,
    ModuleRequest,
    PromiseCapability,
    PromiseReaction,
    PropertyDescriptorObject,
    PrototypeInfo,
    PrototypeSharedClosureInfo,
    RegExpBoilerplateDescription,
    RegExpDataWrapper,
    Script,
    ScriptOrModule,
    SourceTextModuleInfoEntry,
    StackFrameInfo,
    StackTraceInfo,
    TemplateObjectDescription,
    Tuple2,
    WasmExceptionTag,
  ];
  assert_eq!(structs[0].raw(), FIRST_STRUCT_TYPE);
  for w in structs.windows(2) {
    assert!(w[0].raw() < w[1].raw(), "STRUCT_LIST 失序");
  }
  OK
}

/// 规约 (instance-type.h:232-251 static_assert): ABI 锚点与函数区间。
#[test]
fn instance_type_abi_anchors() -> Void {
  assert_eq!(InstanceType::Symbol.raw(), 0x80); // kFirstNonstringType
  assert_eq!(InstanceType::Oddball.raw(), 0x83);
  assert_eq!(InstanceType::Foreign.raw(), 0xcc);
  assert_eq!(InstanceType::JSSpecialApiObject.raw(), 0x410);
  assert_eq!(InstanceType::JSObject.raw(), 0x421);
  assert_eq!(it::FIRST_JS_API_OBJECT_TYPE, 0x422);
  assert_eq!(it::LAST_JS_API_OBJECT_TYPE, 0x80a);
  assert_eq!(it::PROPERTY_DICTIONARY_TYPE, InstanceType::NameDictionary); // swiss 默认关
  assert_eq!(it::DEPENDENT_CODE_TYPE, InstanceType::WeakArrayList);
  assert_eq!(it::BIGINT_TYPE, InstanceType::BigIntBase);
  assert_eq!(it::JS_FUNCTION_TYPE, InstanceType::JSFunctionWithPrototype);
  assert_eq!(
    it::FIRST_CALLABLE_JS_FUNCTION_TYPE.raw(),
    it::FIRST_JS_FUNCTION_TYPE
  );
  assert_eq!(
    it::LAST_CALLABLE_JS_FUNCTION_TYPE.raw(),
    InstanceType::JSClassConstructor.raw() - 1
  );
  // classConstructor 是 JS 函数区间末项, 且不在可调用子区间 (251-255)
  assert_eq!(
    InstanceType::JSClassConstructor.raw(),
    LAST_JS_FUNCTION_TYPE
  );
  assert!(InstanceType::JSClassConstructor.raw() > it::LAST_CALLABLE_JS_FUNCTION_TYPE.raw());
  // Torque 有符号处理约束 (292)
  assert!(LAST_TYPE.raw() < 1 << 15);
  assert_eq!(LAST_TYPE.raw(), LAST_TYPE_VALUE);
  OK
}

/// 规约 (instance-type.h:266-288 CHECK_INSTANCE_TYPE):
/// special/custom-elements 区间的成员恰为预期集合 (含 WASM 开配置)。
#[test]
fn instance_type_special_receiver_ranges() -> Void {
  let in_special = |t: InstanceType| {
    t.raw() >= FIRST_JS_RECEIVER_TYPE && t.raw() <= LAST_SPECIAL_RECEIVER_TYPE.raw()
  };
  let in_custom = |t: InstanceType| {
    t.raw() >= FIRST_JS_RECEIVER_TYPE && t.raw() <= LAST_CUSTOM_ELEMENTS_RECEIVER.raw()
  };
  use InstanceType::*;
  for t in [
    WasmStruct,
    WasmArray,
    JSProxy,
    JSGlobalObject,
    JSGlobalProxy,
    JSModuleNamespace,
    JSDeferredModuleNamespace,
    JSSpecialApiObject,
  ] {
    assert!(in_special(t), "{t:?} 应在 special receiver 区间");
    assert!(in_custom(t), "{t:?} 应在 custom elements 区间");
  }
  // JSPrimitiveWrapper: 不在 special 区间, 在 custom elements 区间
  assert!(!in_special(JSPrimitiveWrapper));
  assert!(in_custom(JSPrimitiveWrapper));
  // 区间外的普通 JSObject 子类
  for t in [JSArray, JSDate, JSObject, Symbol, HeapNumber, Map] {
    assert!(!in_special(t), "{t:?} 不应在 special receiver 区间");
  }
  // 伪类型别名对齐
  assert_eq!(
    LAST_SPECIAL_RECEIVER_TYPE.raw(),
    it::LAST_JS_SPECIAL_OBJECT_TYPE
  );
  assert_eq!(
    LAST_CUSTOM_ELEMENTS_RECEIVER.raw(),
    it::LAST_JS_CUSTOM_ELEMENTS_OBJECT_TYPE
  );
  OK
}

/// 规约 (instance-type.h:116-118 IsShortcutCandidate):
/// 仅未内部化的 cons 字符串可被 GC 快捷。
#[test]
fn instance_type_shortcut_candidate() -> Void {
  // CONS_TWO_BYTE=65 / CONS_ONE_BYTE=73: 唯一命中对
  assert!(it::is_shortcut_candidate(65));
  assert!(it::is_shortcut_candidate(73));
  for raw in [0u16, 8, 24, 64, 72, 66, 74, 67, 75, 69, 77, 128] {
    assert!(!it::is_shortcut_candidate(raw), "@{raw}");
  }
  OK
}
