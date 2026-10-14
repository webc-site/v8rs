// 由 tool/src/maglev/maglev-node-type.js 从 v8/src/maglev/maglev-node-type.h 生成; 修订就地改, 复跑覆盖
// NodeType 位集: 叶子 18 + 组合 20 = 38

//! Maglev IR 节点类型位集 (原版 `enum class NodeType : uint32_t`)。
//! 位值逐条抄自 LEAF/COMBINED_NODE_TYPE_LIST 表达式并由脚本求值互校。
//! 模板/引用面 (`Is<T>`/`As<T>`/`StaticTypeForMap`/`StaticTypeForConstant` 等) 非本波范围,
//! 见 readme/zh.md skip 登记。

/// 原版 `kNumberOfLeafNodeTypes`。
pub const NUMBER_OF_LEAF_NODE_TYPES: u32 = 18;
/// 原版 `using NodeTypeInt = std::underlying_type_t<NodeType>`。
pub type NodeTypeInt = u32;

/// 叶子类型名表 (原版 LEAF_NODE_TYPE_LIST 顺序)。
pub const LEAF_NODE_TYPE_NAMES: [&str; 18] = [
  "Smi",
  "HeapNumber",
  "Null",
  "Undefined",
  "Boolean",
  "Symbol",
  "InternalizedString",
  "OtherString",
  "Context",
  "StringWrapper",
  "JSArray",
  "JSFunction",
  "OtherCallable",
  "JSDataView",
  "JSGeneratorObject",
  "OtherHeapObject",
  "OtherJSReceiver",
  "BigInt",
];

/// 组合类型名 + 原文位表达式 (原版 COMBINED_NODE_TYPE_LIST 顺序)。
pub const COMBINED_NODE_TYPE_NAMES: [&str; 20] = [
  "Unknown",
  "None",
  "Callable",
  "NullOrUndefined",
  "Oddball",
  "Number",
  "Numeric",
  "NumberOrBoolean",
  "NumberOrUndefined",
  "NumberOrOddball",
  "String",
  "StringOrStringWrapper",
  "StringOrOddball",
  "Name",
  "JSPrimitive",
  "JSReceiver",
  "JSReceiverOrNull",
  "JSReceiverOrNullOrUndefined",
  "NotUndetectable",
  "AnyHeapObject",
];

/// 节点类型位集: 原版是 `enum class NodeType : uint32_t` 当位集用,
/// rust 侧建模为透明位集结构体, 判别值 1:1 复刻原版 `V(Name, value)`。
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct NodeType {
  bits: NodeTypeInt,
}

impl NodeType {
  /// 原版裸位值入口 (`static_cast<NodeType>(int)`)。
  pub const fn from_bits(bits: NodeTypeInt) -> Self {
    Self { bits }
  }

  /// 原版 `static_cast<NodeTypeInt>(type)`。
  pub const fn to_int(self) -> NodeTypeInt {
    self.bits
  }

  /// 叶子位 (1 << 0)
  pub const SMI: Self = Self::from_bits(1);
  /// 叶子位 (1 << 1)
  pub const HEAP_NUMBER: Self = Self::from_bits(2);
  /// 叶子位 (1 << 2)
  pub const NULL: Self = Self::from_bits(4);
  /// 叶子位 (1 << 3)
  pub const UNDEFINED: Self = Self::from_bits(8);
  /// 叶子位 (1 << 4)
  pub const BOOLEAN: Self = Self::from_bits(16);
  /// 叶子位 (1 << 5)
  pub const SYMBOL: Self = Self::from_bits(32);
  /// 叶子位 (1 << 6)
  pub const INTERNALIZED_STRING: Self = Self::from_bits(64);
  /// 叶子位 (1 << 7)
  pub const OTHER_STRING: Self = Self::from_bits(128);
  /// 叶子位 (1 << 8)
  pub const CONTEXT: Self = Self::from_bits(256);
  /// 叶子位 (1 << 9)
  pub const STRING_WRAPPER: Self = Self::from_bits(512);
  /// 叶子位 (1 << 10)
  pub const JS_ARRAY: Self = Self::from_bits(1024);
  /// 叶子位 (1 << 11)
  pub const JS_FUNCTION: Self = Self::from_bits(2048);
  /// 叶子位 (1 << 12)
  pub const OTHER_CALLABLE: Self = Self::from_bits(4096);
  /// 叶子位 (1 << 13)
  pub const JS_DATA_VIEW: Self = Self::from_bits(8192);
  /// 叶子位 (1 << 14)
  pub const JS_GENERATOR_OBJECT: Self = Self::from_bits(16384);
  /// 叶子位 (1 << 15)
  pub const OTHER_HEAP_OBJECT: Self = Self::from_bits(32768);
  /// 叶子位 (1 << 16)
  pub const OTHER_JS_RECEIVER: Self = Self::from_bits(65536);
  /// 叶子位 (1 << 17)
  pub const BIG_INT: Self = Self::from_bits(131072);
  /// 组合位 ((1 << kNumberOfLeafNodeTypes) - 1)
  pub const UNKNOWN: Self = Self::from_bits(262143);
  /// 组合位 0
  pub const NONE: Self = Self::from_bits(0);
  /// 组合位 kJSFunction | kOtherCallable
  pub const CALLABLE: Self = Self::from_bits(6144);
  /// 组合位 kNull | kUndefined
  pub const NULL_OR_UNDEFINED: Self = Self::from_bits(12);
  /// 组合位 kNullOrUndefined | kBoolean
  pub const ODDBALL: Self = Self::from_bits(28);
  /// 组合位 kSmi | kHeapNumber
  pub const NUMBER: Self = Self::from_bits(3);
  /// 组合位 kNumber | kBigInt
  pub const NUMERIC: Self = Self::from_bits(131075);
  /// 组合位 kNumber | kBoolean
  pub const NUMBER_OR_BOOLEAN: Self = Self::from_bits(19);
  /// 组合位 kNumber | kUndefined
  pub const NUMBER_OR_UNDEFINED: Self = Self::from_bits(11);
  /// 组合位 kNumber | kOddball
  pub const NUMBER_OR_ODDBALL: Self = Self::from_bits(31);
  /// 组合位 kInternalizedString | kOtherString
  pub const STRING: Self = Self::from_bits(192);
  /// 组合位 kString | kStringWrapper
  pub const STRING_OR_STRING_WRAPPER: Self = Self::from_bits(704);
  /// 组合位 kString | kOddball
  pub const STRING_OR_ODDBALL: Self = Self::from_bits(220);
  /// 组合位 kString | kSymbol
  pub const NAME: Self = Self::from_bits(224);
  /// 组合位 kNumber | kString | kBoolean | kNullOrUndefined | kSymbol | kBigInt
  pub const JS_PRIMITIVE: Self = Self::from_bits(131327);
  /// 组合位 kJSArray | kCallable | kStringWrapper | kJSDataView | kJSGeneratorObject | kOtherJSReceiver
  pub const JS_RECEIVER: Self = Self::from_bits(97792);
  /// 组合位 kJSReceiver | kNull
  pub const JS_RECEIVER_OR_NULL: Self = Self::from_bits(97796);
  /// 组合位 kJSReceiver | kNullOrUndefined
  pub const JS_RECEIVER_OR_NULL_OR_UNDEFINED: Self = Self::from_bits(97804);
  /// 组合位 kNumeric | kName | kBoolean
  pub const NOT_UNDETECTABLE: Self = Self::from_bits(131315);
  /// 组合位 kUnknown - kSmi
  pub const ANY_HEAP_OBJECT: Self = Self::from_bits(262142);
}

/// 类型名表 (原版 NODE_TYPE_LIST 顺序: 18 叶子后接 20 组合)。
pub const NODE_TYPE_NAMES: [(&str, NodeType); 38] = [
  ("Smi", NodeType::SMI),
  ("HeapNumber", NodeType::HEAP_NUMBER),
  ("Null", NodeType::NULL),
  ("Undefined", NodeType::UNDEFINED),
  ("Boolean", NodeType::BOOLEAN),
  ("Symbol", NodeType::SYMBOL),
  ("InternalizedString", NodeType::INTERNALIZED_STRING),
  ("OtherString", NodeType::OTHER_STRING),
  ("Context", NodeType::CONTEXT),
  ("StringWrapper", NodeType::STRING_WRAPPER),
  ("JSArray", NodeType::JS_ARRAY),
  ("JSFunction", NodeType::JS_FUNCTION),
  ("OtherCallable", NodeType::OTHER_CALLABLE),
  ("JSDataView", NodeType::JS_DATA_VIEW),
  ("JSGeneratorObject", NodeType::JS_GENERATOR_OBJECT),
  ("OtherHeapObject", NodeType::OTHER_HEAP_OBJECT),
  ("OtherJSReceiver", NodeType::OTHER_JS_RECEIVER),
  ("BigInt", NodeType::BIG_INT),
  ("Unknown", NodeType::UNKNOWN),
  ("None", NodeType::NONE),
  ("Callable", NodeType::CALLABLE),
  ("NullOrUndefined", NodeType::NULL_OR_UNDEFINED),
  ("Oddball", NodeType::ODDBALL),
  ("Number", NodeType::NUMBER),
  ("Numeric", NodeType::NUMERIC),
  ("NumberOrBoolean", NodeType::NUMBER_OR_BOOLEAN),
  ("NumberOrUndefined", NodeType::NUMBER_OR_UNDEFINED),
  ("NumberOrOddball", NodeType::NUMBER_OR_ODDBALL),
  ("String", NodeType::STRING),
  ("StringOrStringWrapper", NodeType::STRING_OR_STRING_WRAPPER),
  ("StringOrOddball", NodeType::STRING_OR_ODDBALL),
  ("Name", NodeType::NAME),
  ("JSPrimitive", NodeType::JS_PRIMITIVE),
  ("JSReceiver", NodeType::JS_RECEIVER),
  ("JSReceiverOrNull", NodeType::JS_RECEIVER_OR_NULL),
  (
    "JSReceiverOrNullOrUndefined",
    NodeType::JS_RECEIVER_OR_NULL_OR_UNDEFINED,
  ),
  ("NotUndetectable", NodeType::NOT_UNDETECTABLE),
  ("AnyHeapObject", NodeType::ANY_HEAP_OBJECT),
];

/// 原版 `enum class NodeTypeIsVariant`。
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum NodeTypeIsVariant {
  /// 不允许 lhs 为 None (原版 DCHECK_NE(type, kNone))
  Default = 0,
  /// 允许 lhs 为 None (原版注释: 通常意味着死代码)
  AllowNone = 1,
}

/// 原版 `EmptyNodeType()`: 全位清零。
pub const fn empty_node_type() -> NodeType {
  NodeType::NONE
}

/// 原版 `NodeTypeIsNeverStandalone` 登记的类型 (永不作为单独叶子类型出现)。
pub const NEVER_STANDALONE: [NodeType; 2] = [NodeType::OTHER_CALLABLE, NodeType::OTHER_STRING];

/// 原版 `NodeTypeIsNeverStandalone(type)`。
pub const fn node_type_is_never_standalone(node_type: NodeType) -> bool {
  node_type.to_int() == NodeType::OTHER_CALLABLE.to_int()
    || node_type.to_int() == NodeType::OTHER_STRING.to_int()
}

/// 原版 `IntersectType`: left & right。
pub const fn intersect_type(left: NodeType, right: NodeType) -> NodeType {
  NodeType::from_bits(left.to_int() & right.to_int())
}

/// 原版 `UnionType`: left | right。
pub const fn union_type(left: NodeType, right: NodeType) -> NodeType {
  NodeType::from_bits(left.to_int() | right.to_int())
}

/// 原版 `RemoveType`: left & !right。
pub const fn remove_type(left: NodeType, right: NodeType) -> NodeType {
  NodeType::from_bits(left.to_int() & !right.to_int())
}

/// 原版 `IsEmptyNodeType`: 无位被置。
pub const fn is_empty_node_type(node_type: NodeType) -> bool {
  node_type.to_int() == 0
}

/// 原版 `NodeTypeIs(type, to_check, variant)`: type 位集 ⊆ to_check。
pub const fn node_type_is(
  node_type: NodeType,
  to_check: NodeType,
  variant: NodeTypeIsVariant,
) -> bool {
  // 原版非 AllowNone 分支只有 DCHECK_NE(type, kNone); rust 侧无 DCHECK, 以 false 承接该非法输入
  if matches!(variant, NodeTypeIsVariant::Default) && is_empty_node_type(node_type) {
    return false;
  }
  (node_type.to_int() & !to_check.to_int()) == 0
}

/// 原版 `NodeTypeIsForPrinting`: 同 NodeTypeIs 但去 DCHECK。
pub const fn node_type_is_for_printing(node_type: NodeType, to_check: NodeType) -> bool {
  (node_type.to_int() & !to_check.to_int()) == 0
}

/// 原版 `NodeTypeCanBe(type, to_check)`: 位集有交集 (默认 allow_standalone=false)。
pub const fn node_type_can_be(node_type: NodeType, to_check: NodeType) -> bool {
  (node_type.to_int() & to_check.to_int()) != 0
}

/// 原版 `NodeTypeMayBeNullOrUndefined`。
pub const fn node_type_may_be_null_or_undefined(node_type: NodeType) -> bool {
  (node_type.to_int() & NodeType::NULL_OR_UNDEFINED.to_int()) != 0
}

// —— 原版 DEFINE_NODE_TYPE_CHECK 生成的逐类型检查 (默认 variant=kAllowNone) ——
/// 原版 `NodeTypeIsSmi(type, variant = kAllowNone)`。
pub const fn node_type_is_smi(node_type: NodeType) -> bool {
  node_type_is(node_type, NodeType::SMI, NodeTypeIsVariant::AllowNone)
}

/// 原版 `NodeTypeIsHeapNumber(type, variant = kAllowNone)`。
pub const fn node_type_is_heap_number(node_type: NodeType) -> bool {
  node_type_is(
    node_type,
    NodeType::HEAP_NUMBER,
    NodeTypeIsVariant::AllowNone,
  )
}

/// 原版 `NodeTypeIsNull(type, variant = kAllowNone)`。
pub const fn node_type_is_null(node_type: NodeType) -> bool {
  node_type_is(node_type, NodeType::NULL, NodeTypeIsVariant::AllowNone)
}

/// 原版 `NodeTypeIsUndefined(type, variant = kAllowNone)`。
pub const fn node_type_is_undefined(node_type: NodeType) -> bool {
  node_type_is(node_type, NodeType::UNDEFINED, NodeTypeIsVariant::AllowNone)
}

/// 原版 `NodeTypeIsBoolean(type, variant = kAllowNone)`。
pub const fn node_type_is_boolean(node_type: NodeType) -> bool {
  node_type_is(node_type, NodeType::BOOLEAN, NodeTypeIsVariant::AllowNone)
}

/// 原版 `NodeTypeIsSymbol(type, variant = kAllowNone)`。
pub const fn node_type_is_symbol(node_type: NodeType) -> bool {
  node_type_is(node_type, NodeType::SYMBOL, NodeTypeIsVariant::AllowNone)
}

/// 原版 `NodeTypeIsInternalizedString(type, variant = kAllowNone)`。
pub const fn node_type_is_internalized_string(node_type: NodeType) -> bool {
  node_type_is(
    node_type,
    NodeType::INTERNALIZED_STRING,
    NodeTypeIsVariant::AllowNone,
  )
}

/// 原版 `NodeTypeIsOtherString(type, variant = kAllowNone)`。
pub const fn node_type_is_other_string(node_type: NodeType) -> bool {
  node_type_is(
    node_type,
    NodeType::OTHER_STRING,
    NodeTypeIsVariant::AllowNone,
  )
}

/// 原版 `NodeTypeIsContext(type, variant = kAllowNone)`。
pub const fn node_type_is_context(node_type: NodeType) -> bool {
  node_type_is(node_type, NodeType::CONTEXT, NodeTypeIsVariant::AllowNone)
}

/// 原版 `NodeTypeIsStringWrapper(type, variant = kAllowNone)`。
pub const fn node_type_is_string_wrapper(node_type: NodeType) -> bool {
  node_type_is(
    node_type,
    NodeType::STRING_WRAPPER,
    NodeTypeIsVariant::AllowNone,
  )
}

/// 原版 `NodeTypeIsJSArray(type, variant = kAllowNone)`。
pub const fn node_type_is_js_array(node_type: NodeType) -> bool {
  node_type_is(node_type, NodeType::JS_ARRAY, NodeTypeIsVariant::AllowNone)
}

/// 原版 `NodeTypeIsJSFunction(type, variant = kAllowNone)`。
pub const fn node_type_is_js_function(node_type: NodeType) -> bool {
  node_type_is(
    node_type,
    NodeType::JS_FUNCTION,
    NodeTypeIsVariant::AllowNone,
  )
}

/// 原版 `NodeTypeIsOtherCallable(type, variant = kAllowNone)`。
pub const fn node_type_is_other_callable(node_type: NodeType) -> bool {
  node_type_is(
    node_type,
    NodeType::OTHER_CALLABLE,
    NodeTypeIsVariant::AllowNone,
  )
}

/// 原版 `NodeTypeIsJSDataView(type, variant = kAllowNone)`。
pub const fn node_type_is_js_data_view(node_type: NodeType) -> bool {
  node_type_is(
    node_type,
    NodeType::JS_DATA_VIEW,
    NodeTypeIsVariant::AllowNone,
  )
}

/// 原版 `NodeTypeIsJSGeneratorObject(type, variant = kAllowNone)`。
pub const fn node_type_is_js_generator_object(node_type: NodeType) -> bool {
  node_type_is(
    node_type,
    NodeType::JS_GENERATOR_OBJECT,
    NodeTypeIsVariant::AllowNone,
  )
}

/// 原版 `NodeTypeIsOtherHeapObject(type, variant = kAllowNone)`。
pub const fn node_type_is_other_heap_object(node_type: NodeType) -> bool {
  node_type_is(
    node_type,
    NodeType::OTHER_HEAP_OBJECT,
    NodeTypeIsVariant::AllowNone,
  )
}

/// 原版 `NodeTypeIsOtherJSReceiver(type, variant = kAllowNone)`。
pub const fn node_type_is_other_js_receiver(node_type: NodeType) -> bool {
  node_type_is(
    node_type,
    NodeType::OTHER_JS_RECEIVER,
    NodeTypeIsVariant::AllowNone,
  )
}

/// 原版 `NodeTypeIsBigInt(type, variant = kAllowNone)`。
pub const fn node_type_is_big_int(node_type: NodeType) -> bool {
  node_type_is(node_type, NodeType::BIG_INT, NodeTypeIsVariant::AllowNone)
}

/// 原版 `NodeTypeIsUnknown(type, variant = kAllowNone)`。
pub const fn node_type_is_unknown(node_type: NodeType) -> bool {
  node_type_is(node_type, NodeType::UNKNOWN, NodeTypeIsVariant::AllowNone)
}

/// 原版 `NodeTypeIsNone(type, variant = kAllowNone)`。
pub const fn node_type_is_none(node_type: NodeType) -> bool {
  node_type_is(node_type, NodeType::NONE, NodeTypeIsVariant::AllowNone)
}

/// 原版 `NodeTypeIsCallable(type, variant = kAllowNone)`。
pub const fn node_type_is_callable(node_type: NodeType) -> bool {
  node_type_is(node_type, NodeType::CALLABLE, NodeTypeIsVariant::AllowNone)
}

/// 原版 `NodeTypeIsNullOrUndefined(type, variant = kAllowNone)`。
pub const fn node_type_is_null_or_undefined(node_type: NodeType) -> bool {
  node_type_is(
    node_type,
    NodeType::NULL_OR_UNDEFINED,
    NodeTypeIsVariant::AllowNone,
  )
}

/// 原版 `NodeTypeIsOddball(type, variant = kAllowNone)`。
pub const fn node_type_is_oddball(node_type: NodeType) -> bool {
  node_type_is(node_type, NodeType::ODDBALL, NodeTypeIsVariant::AllowNone)
}

/// 原版 `NodeTypeIsNumber(type, variant = kAllowNone)`。
pub const fn node_type_is_number(node_type: NodeType) -> bool {
  node_type_is(node_type, NodeType::NUMBER, NodeTypeIsVariant::AllowNone)
}

/// 原版 `NodeTypeIsNumeric(type, variant = kAllowNone)`。
pub const fn node_type_is_numeric(node_type: NodeType) -> bool {
  node_type_is(node_type, NodeType::NUMERIC, NodeTypeIsVariant::AllowNone)
}

/// 原版 `NodeTypeIsNumberOrBoolean(type, variant = kAllowNone)`。
pub const fn node_type_is_number_or_boolean(node_type: NodeType) -> bool {
  node_type_is(
    node_type,
    NodeType::NUMBER_OR_BOOLEAN,
    NodeTypeIsVariant::AllowNone,
  )
}

/// 原版 `NodeTypeIsNumberOrUndefined(type, variant = kAllowNone)`。
pub const fn node_type_is_number_or_undefined(node_type: NodeType) -> bool {
  node_type_is(
    node_type,
    NodeType::NUMBER_OR_UNDEFINED,
    NodeTypeIsVariant::AllowNone,
  )
}

/// 原版 `NodeTypeIsNumberOrOddball(type, variant = kAllowNone)`。
pub const fn node_type_is_number_or_oddball(node_type: NodeType) -> bool {
  node_type_is(
    node_type,
    NodeType::NUMBER_OR_ODDBALL,
    NodeTypeIsVariant::AllowNone,
  )
}

/// 原版 `NodeTypeIsString(type, variant = kAllowNone)`。
pub const fn node_type_is_string(node_type: NodeType) -> bool {
  node_type_is(node_type, NodeType::STRING, NodeTypeIsVariant::AllowNone)
}

/// 原版 `NodeTypeIsStringOrStringWrapper(type, variant = kAllowNone)`。
pub const fn node_type_is_string_or_string_wrapper(node_type: NodeType) -> bool {
  node_type_is(
    node_type,
    NodeType::STRING_OR_STRING_WRAPPER,
    NodeTypeIsVariant::AllowNone,
  )
}

/// 原版 `NodeTypeIsStringOrOddball(type, variant = kAllowNone)`。
pub const fn node_type_is_string_or_oddball(node_type: NodeType) -> bool {
  node_type_is(
    node_type,
    NodeType::STRING_OR_ODDBALL,
    NodeTypeIsVariant::AllowNone,
  )
}

/// 原版 `NodeTypeIsName(type, variant = kAllowNone)`。
pub const fn node_type_is_name(node_type: NodeType) -> bool {
  node_type_is(node_type, NodeType::NAME, NodeTypeIsVariant::AllowNone)
}

/// 原版 `NodeTypeIsJSPrimitive(type, variant = kAllowNone)`。
pub const fn node_type_is_js_primitive(node_type: NodeType) -> bool {
  node_type_is(
    node_type,
    NodeType::JS_PRIMITIVE,
    NodeTypeIsVariant::AllowNone,
  )
}

/// 原版 `NodeTypeIsJSReceiver(type, variant = kAllowNone)`。
pub const fn node_type_is_js_receiver(node_type: NodeType) -> bool {
  node_type_is(
    node_type,
    NodeType::JS_RECEIVER,
    NodeTypeIsVariant::AllowNone,
  )
}

/// 原版 `NodeTypeIsJSReceiverOrNull(type, variant = kAllowNone)`。
pub const fn node_type_is_js_receiver_or_null(node_type: NodeType) -> bool {
  node_type_is(
    node_type,
    NodeType::JS_RECEIVER_OR_NULL,
    NodeTypeIsVariant::AllowNone,
  )
}

/// 原版 `NodeTypeIsJSReceiverOrNullOrUndefined(type, variant = kAllowNone)`。
pub const fn node_type_is_js_receiver_or_null_or_undefined(node_type: NodeType) -> bool {
  node_type_is(
    node_type,
    NodeType::JS_RECEIVER_OR_NULL_OR_UNDEFINED,
    NodeTypeIsVariant::AllowNone,
  )
}

/// 原版 `NodeTypeIsNotUndetectable(type, variant = kAllowNone)`。
pub const fn node_type_is_not_undetectable(node_type: NodeType) -> bool {
  node_type_is(
    node_type,
    NodeType::NOT_UNDETECTABLE,
    NodeTypeIsVariant::AllowNone,
  )
}

/// 原版 `NodeTypeIsAnyHeapObject(type, variant = kAllowNone)`。
pub const fn node_type_is_any_heap_object(node_type: NodeType) -> bool {
  node_type_is(
    node_type,
    NodeType::ANY_HEAP_OBJECT,
    NodeTypeIsVariant::AllowNone,
  )
}

// 对数: 叶子 18 (逐条 (1<<i)) + 组合 20 (表达式求值 + 7 项互校) + 变体 2 +
//   NodeTypeIsNeverStandalone 2 + 原文锚 16 + 生成检查 38 = 全过。
