// v8-objects-maps/src/name_ext.rs

//! NameTaggedExt: 为 `Tagged<Name>` 提供便捷的哈希字段访问扩展方法。

use std::ptr;

use v8_objects_core::{name::Name, tagged::Tagged};
use v8_objects_strings::string::String as V8String;

/// 对 `Tagged<Name>` 的扩展方法
pub trait NameTaggedExt {
  /// 获取已计算的 30 位哈希载荷，若为空指针则返回 0
  fn hash(&self) -> u32;

  /// 校验 Name 是否匹配指定的 V8String 实例
  fn matches_v8_string(&self, s: &V8String) -> bool;
}

impl NameTaggedExt for Tagged<Name> {
  #[inline]
  fn hash(&self) -> u32 {
    if self.is_null() {
      0
    } else {
      // SAFETY: self 不是空指针，指向有效的 Name 堆对象内存
      unsafe { (*self.as_ptr()).hash() }
    }
  }

  #[inline]
  fn matches_v8_string(&self, s: &V8String) -> bool {
    if self.is_null() {
      return false;
    }
    let self_hash = self.hash();
    let str_hash = s.name.hash();
    if self_hash != 0 && str_hash != 0 && self_hash != str_hash {
      return false;
    }
    // SAFETY: self 已判定非空
    unsafe {
      ptr::eq(
        self.as_ptr().cast::<u8>(),
        (s as *const V8String).cast::<u8>(),
      )
    }
  }
}
