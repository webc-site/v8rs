use aok::{OK, Void};
use log::info;
use v8_maglev_ir::{
  NodeType, OPCODE_COUNT, Opcode, is_conversion_node, is_value_node, node_type_is_js_array,
};

#[ctor::ctor(unsafe)]
fn _log_init() {
  log_init::init();
}

// crate 冒烟: 真实断言导出面可用 (替代模板恒真占位测试)。
#[test]
fn crate_smoke() -> Void {
  // 计数锚: maglev-ir.h:560-562 `kOpcodeCount = NODE_BASE_LIST(PLUS_ONE)`,
  // 目标配置 (UD=off / INTL=off) 独立清点 NODE_BASE_LIST = 378。
  assert_eq!(OPCODE_COUNT, 378);
  // 区间谓词锚: maglev-ir.h:546-550 CONTROL_NODE_LIST 含 V(Jump);
  // :274 VALUE_NODE_LIST 含 V(Identity) 且不含控制节点。
  assert!(is_value_node(Opcode::Identity));
  // :230-272 CONVERSION_NODE_LIST 首 ChangeInt32ToFloat64 末 UnsafeSmiUntag。
  assert!(is_conversion_node(Opcode::ChangeInt32ToFloat64));
  assert!(is_conversion_node(Opcode::UnsafeSmiUntag));
  // 位集谓词锚: maglev-node-type.h:32 V(JSArray, (1 << 10)) (:30 系 V(Context))。
  assert!(node_type_is_js_array(NodeType::JS_ARRAY));
  assert!(!node_type_is_js_array(NodeType::SMI));
  info!("crate smoke ok");
  OK
}
