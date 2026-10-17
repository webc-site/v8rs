#!/usr/bin/env bun
// v8/src/compiler/common-utils.cc + common-utils.h
// → v8-compiler-passes/src/common_utils.rs.skel
//
// 全族一个函数。规则面: ConcatenateStrings 的完整分支链
// (空串短路 → kMaxLength 门槛 → 100 阈值 Cons 化 → 单双字节展平),
// 常量 kConstantStringFlattenMaxSize 逐字提取。
import { readV8, writeSrc, banner } from "../../lib.js";

const h = readV8("src/compiler/common-utils.h");
const cc = readV8("src/compiler/common-utils.cc");

// —— 哨兵 ——
const sentinels = [
  [h, /MaybeHandle<String> ConcatenateStrings\(Handle<String> left,/, "声明"],
  [cc, /constexpr int32_t kConstantStringFlattenMaxSize = 100;/, "展平阈值常量"],
  [cc, /if \(left->length\(\) == 0\) return right;/, "左空短路"],
  [cc, /String::kMaxLength/, "kMaxLength 门槛"],
  [cc, /NewConsString\(left, right, AllocationType::kOld\)/, "ConsString 路径"],
  [cc, /left->IsOneByteRepresentation\(\) && right->IsOneByteRepresentation\(\)/, "单字节判据"],
  [cc, /static_assert\(ConsString::kMinLength <= kConstantStringFlattenMaxSize\);/, "kMinLength 静态断言"],
];
for (const [src, re, name] of sentinels) {
  if (!re.test(src)) throw new Error(`哨兵缺失: ${name}`);
}
// 双字节路径必须存在
if (!/NewRawTwoByteString/.test(cc)) throw new Error("双字节路径缺失");

const body = `${banner("src/compiler/common-utils.js", "src/compiler/common-utils.h + src/compiler/common-utils.cc", "utils::ConcatenateStrings — 编译期常量字符串拼接")}
//! ConcatenateStrings: broker 线程安全字符串拼接。
//!
//! 规则面: 分支链 6 条 + 阈值常量。

#![allow(dead_code)]

/// v8: src/compiler/common-utils.cc kConstantStringFlattenMaxSize
/// 超过此长度 → ConsString (避免重复拼接的二次方代价);
/// static_assert(ConsString::kMinLength <= 100) 保证后台线程路径安全
pub const K_CONSTANT_STRING_FLATTEN_MAX_SIZE: i32 = 100;

/// v8: src/compiler/common-utils.cc utils::ConcatenateStrings
/// 分支链 (原文顺序):
/// 1. left 空 → right; 2. right 空 → left;
/// 3. 总长 > String::kMaxLength → 空 MaybeHandle;
/// 4. 总长 > 100 → NewConsString(kOld);
/// 5. 双方 OneByte 表示 → NewRawOneByteString(kOld) 展平拼接;
/// 6. 否则 → NewRawTwoByteString(kOld) 展平拼接。
/// 注: 非 readonly 空间字符串需 SharedStringAccessGuardIfNeeded。
pub fn concatenate_strings(
  left: crate::handles::Handle<crate::objects::String>,
  right: crate::handles::Handle<crate::objects::String>,
  broker: &mut crate::broker::JSHeapBroker,
) -> Option<crate::handles::Handle<crate::objects::String>> {
  if left.length() == 0 {
    return Some(right);
  }
  if right.length() == 0 {
    return Some(left);
  }
  if left.length() as i64 + right.length() as i64 > crate::objects::string::K_MAX_LENGTH as i64 {
    return None;
  }
  let length = left.length() + right.length();
  if length > K_CONSTANT_STRING_FLATTEN_MAX_SIZE {
    return Some(
      broker.local_isolate_or_isolate()
        .factory()
        .new_cons_string(left, right, crate::AllocationType::Old)
        .to_handle_checked(),
    );
  }
  let result_is_one_byte = left.is_one_byte_representation() && right.is_one_byte_representation();
  // 语义面: guard 判定 + 逐字符复制 + canonical handle 包装
  todo!("展平拼接 (one_byte={result_is_one_byte})")
}
`;

const p = writeSrc("v8-compiler-passes", "common_utils.rs.skel", body);
console.log(`written ${p}\n函数=1 分支链=6 常量=1 (阈值 100)`);
