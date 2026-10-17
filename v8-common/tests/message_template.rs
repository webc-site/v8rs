//! message-template 对数与查找校验
//!
//! 断言规约清单 (先规约后断言, 强度守恒):
//! - count: 498 变体/串表/哨兵对齐
//! - from_int_bounds: 边界与还原 (h:796)
//! - all_strings_digest: 498 串 FNV-1a-64 金标 (原版逐条机械比对固化) +
//!   三元条目默认分支串 (h:113-117)
//! - strings_spot: 抽查若干含 % 占位与两段拼接串

use aok::{OK, Void};
use v8_common::{
  JS_IMMUTABLE_ARRAYBUFFER, K_MESSAGE_COUNT, MESSAGE_TEMPLATE_STRINGS, MessageTemplate,
  message_template_from_int, template_string,
};

#[ctor::ctor(unsafe)]
fn _log_init() {
  log_init::init();
}

// 三元分支配置常量: 目标配置 js_immutable_arraybuffer=false
const _: () = assert!(!JS_IMMUTABLE_ARRAYBUFFER);

/// 对数: 枚举 498 变体 (kMessageCount 哨兵), 字符串表等长
#[test]
fn message_template_count() -> Void {
  assert_eq!(MESSAGE_TEMPLATE_STRINGS.len(), 498);
  assert_eq!(K_MESSAGE_COUNT, 498);
  // 尾部变体判别值 = count - 1 (MESSAGE_TEMPLATES 顺序 1:1)
  assert_eq!(
    MessageTemplate::ExpectAnObjectWithUsing as u16,
    K_MESSAGE_COUNT - 1
  );
  assert_eq!(MessageTemplate::None as u16, 0);
  OK
}

/// MessageTemplateFromInt: 边界与还原 (message-template.h:796)
#[test]
fn message_template_from_int_bounds() -> Void {
  assert_eq!(message_template_from_int(0), Some(MessageTemplate::None));
  assert_eq!(
    message_template_from_int(2),
    Some(MessageTemplate::CyclicProto)
  );
  assert_eq!(
    message_template_from_int(K_MESSAGE_COUNT as i32 - 1),
    Some(MessageTemplate::ExpectAnObjectWithUsing)
  );
  // 越界: 原版 DCHECK 终止 → Rust None (登记)
  assert_eq!(message_template_from_int(-1), None);
  assert_eq!(message_template_from_int(K_MESSAGE_COUNT as i32), None);
  OK
}

/// 全表 498 串摘要校验 (FNV-1a-64 金标, 审查环对原版
/// MESSAGE_TEMPLATES 逐条机械比对 0 差异后固化; 防后续单串漂移假绿)
#[test]
fn message_template_all_strings_digest() -> Void {
  let mut h: u64 = 0xCBF2_9CE4_8422_2325;
  for i in 0..K_MESSAGE_COUNT as i32 {
    let t = message_template_from_int(i).expect("dense 0..count range");
    assert_eq!(t as u16, i as u16, "FromRepr dense order");
    for b in template_string(t).as_bytes() {
      h ^= *b as u64;
      h = h.wrapping_mul(0x0000_0100_0000_01B3);
    }
  }
  assert_eq!(h, 0xd6be_06cb_e879_e7fc);
  // 三元条目 (message-template.h:113-117) 默认分支: js_immutable_arraybuffer=false
  assert_eq!(
    template_string(MessageTemplate::TypedArrayValidateWriteErrorOperation),
    "Cannot perform % on a detached or out-of-bounds ArrayBuffer"
  );
  OK
}

/// 模板串抽查: 与原版 MESSAGE_TEMPLATES 字符串一致 (含 % 占位)
#[test]
fn message_template_strings() -> Void {
  assert_eq!(template_string(MessageTemplate::None), "");
  assert_eq!(
    template_string(MessageTemplate::CyclicProto),
    "Cyclic __proto__ value"
  );
  assert_eq!(
    template_string(MessageTemplate::UncaughtException),
    "Uncaught %"
  );
  assert_eq!(
    template_string(MessageTemplate::ConflictingPrivateName),
    "Operation is ambiguous because there are more than one private name'%' on the object"
  );
  // C++ 两段拼接串
  assert_eq!(
    template_string(MessageTemplate::JsonParseShortString),
    "\"%\" is not valid JSON"
  );
  // Display 走同一查找
  assert_eq!(MessageTemplate::Unsupported.to_string(), "Not supported");
  // TypedArrayValidateWriteErrorOperation 默认分支串
  assert_eq!(
    template_string(MessageTemplate::TypedArrayValidateWriteErrorOperation),
    "Cannot perform % on a detached or out-of-bounds ArrayBuffer"
  );
  OK
}
