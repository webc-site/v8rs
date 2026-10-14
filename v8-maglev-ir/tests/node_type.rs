//! node_type 数据面: 叶子 18 项 (1 << i) 恒等。
//! 期望值独立抄自 v8/src/maglev/maglev-node-type.h:
//!   LEAF_NODE_TYPE_LIST :20-41 (V(Smi,(1<<0)) … V(BigInt,(1<<17))),
//!   kNumberOfLeafNodeTypes :44, V(Unknown,((1<<kNumberOfLeafNodeTypes)-1)) :49。

use aok::{OK, Void};
use log::info;
use v8_maglev_ir::*;

#[ctor::ctor(unsafe)]
fn _log_init() {
  log_init::init();
}

// 叶子位 (1 << i) 恒等: 位值 + 名表 + 计数三向对数
#[test]
fn leaf_bit_identities() -> Void {
  // 顺序逐项抄自 LEAF_NODE_TYPE_LIST 的 V(Name, (1 << i)) 右列
  let leaves: [NodeType; 18] = [
    NodeType::SMI,
    NodeType::HEAP_NUMBER,
    NodeType::NULL,
    NodeType::UNDEFINED,
    NodeType::BOOLEAN,
    NodeType::SYMBOL,
    NodeType::INTERNALIZED_STRING,
    NodeType::OTHER_STRING,
    NodeType::CONTEXT,
    NodeType::STRING_WRAPPER,
    NodeType::JS_ARRAY,
    NodeType::JS_FUNCTION,
    NodeType::OTHER_CALLABLE,
    NodeType::JS_DATA_VIEW,
    NodeType::JS_GENERATOR_OBJECT,
    NodeType::OTHER_HEAP_OBJECT,
    NodeType::OTHER_JS_RECEIVER,
    NodeType::BIG_INT,
  ];
  assert_eq!(leaves.len() as u32, NUMBER_OF_LEAF_NODE_TYPES); // 原版 :44 = 18
  for (i, leaf) in leaves.iter().enumerate() {
    assert_eq!(
      leaf.to_int(),
      1u32 << i,
      "叶子 {} 位值",
      LEAF_NODE_TYPE_NAMES[i]
    );
  }
  // 名表顺序锚: V(Smi) 首, V(BigInt) 末
  assert_eq!(LEAF_NODE_TYPE_NAMES[0], "Smi");
  assert_eq!(LEAF_NODE_TYPE_NAMES[17], "BigInt");
  // :49 V(Unknown, (1 << kNumberOfLeafNodeTypes) - 1) = 全部叶子位之并
  assert_eq!(
    NodeType::UNKNOWN.to_int(),
    (1 << NUMBER_OF_LEAF_NODE_TYPES) - 1
  );
  let mut union = 0u32;
  for leaf in leaves {
    union |= leaf.to_int();
  }
  assert_eq!(union, NodeType::UNKNOWN.to_int());
  // :51 V(None, 0)
  assert_eq!(NodeType::NONE.to_int(), 0);
  info!("OK node_type.rs 叶子 18 项 (1<<i) 恒等全过");
  OK
}

// DEFINE_NODE_TYPE_CHECK (:181-188) 生成的逐类型谓词: 自身位为真, 他叶为假
#[test]
fn leaf_type_checks() -> Void {
  assert!(node_type_is_smi(NodeType::SMI));
  assert!(!node_type_is_smi(NodeType::HEAP_NUMBER));
  assert!(node_type_is_heap_number(NodeType::HEAP_NUMBER));
  assert!(node_type_is_null(NodeType::NULL));
  assert!(node_type_is_undefined(NodeType::UNDEFINED));
  assert!(node_type_is_boolean(NodeType::BOOLEAN));
  assert!(node_type_is_symbol(NodeType::SYMBOL));
  assert!(node_type_is_internalized_string(
    NodeType::INTERNALIZED_STRING
  ));
  assert!(node_type_is_other_string(NodeType::OTHER_STRING));
  assert!(node_type_is_context(NodeType::CONTEXT));
  assert!(node_type_is_string_wrapper(NodeType::STRING_WRAPPER));
  assert!(node_type_is_js_array(NodeType::JS_ARRAY));
  assert!(!node_type_is_js_array(NodeType::JS_FUNCTION));
  assert!(node_type_is_js_function(NodeType::JS_FUNCTION));
  assert!(node_type_is_other_callable(NodeType::OTHER_CALLABLE));
  assert!(node_type_is_js_data_view(NodeType::JS_DATA_VIEW));
  assert!(node_type_is_js_generator_object(
    NodeType::JS_GENERATOR_OBJECT
  ));
  assert!(node_type_is_other_heap_object(NodeType::OTHER_HEAP_OBJECT));
  assert!(node_type_is_other_js_receiver(NodeType::OTHER_JS_RECEIVER));
  assert!(node_type_is_big_int(NodeType::BIG_INT));
  // 组合位含多叶: Number = kSmi|kHeapNumber (:55)
  assert!(node_type_is_number(NodeType::NUMBER));
  assert!(node_type_is_number(NodeType::SMI));
  assert!(!node_type_is_number(NodeType::BOOLEAN));
  // AnyHeapObject = kUnknown - kSmi (:71): Smi 叶外全真
  assert!(!node_type_is_any_heap_object(NodeType::SMI));
  assert!(node_type_is_any_heap_object(NodeType::HEAP_NUMBER));
  assert!(node_type_is_any_heap_object(NodeType::BIG_INT));
  OK
}
