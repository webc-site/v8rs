// 由 tool/src/heap/cppgc-internal/name-trait.cc.js 从 v8/src/heap/cppgc-internal/name-trait.cc 生成;
// 型面 (HeapObjectName/HeapObjectNameForUnnamedObject/NameTraitBase/NameTrait/NameCallback) 声明在
// include/cppgc/internal/name-trait.h, NameProvider 哨兵名在 include/cppgc/name-provider.h;
// 修订就地改, 复跑覆盖。

use core::ffi::c_void;
use std::{any::type_name, marker::PhantomData};

/// v8: include/cppgc/name-provider.h:31 NameProvider::kHiddenName — 隐藏名哨兵 (脱敏模式用)
pub const K_HIDDEN_NAME: &str = "InternalNode";
/// v8: include/cppgc/name-provider.h:37 NameProvider::kNoNameDeducible — 无法推导名哨兵
pub const K_NO_NAME_DEDUCIBLE: &str = "<No name>";

/// v8: include/cppgc/internal/name-trait.h:57-60 HeapObjectName
/// 原版 `const char* value` 为 leaky 拥有权 (new char[] 不回收), Rust 侧 'static 切片同语义。
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HeapObjectName {
  /// 原版 value
  pub value: &'static str,
  /// 原版 name_was_hidden
  pub name_was_hidden: bool,
}

/// v8: include/cppgc/internal/name-trait.h:62-65 HeapObjectNameForUnnamedObject
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum HeapObjectNameForUnnamedObject {
  /// 原版 kUseClassNameIfSupported
  UseClassNameIfSupported = 0,
  /// 原版 kUseHiddenName
  UseHiddenName = 1,
}

/// v8: include/cppgc/internal/name-trait.h:134-135 NameCallback
pub type NameCallback = fn(*const c_void, HeapObjectNameForUnnamedObject) -> HeapObjectName;

/// v8: include/cppgc/name-provider.h:62-71 NameProvider (纯虚接口面)
pub trait NameProvider {
  /// 原版 NameProvider::GetHumanReadableName
  fn get_human_readable_name(&self) -> &'static str;
}

/// v8: include/cppgc/name-provider.h:44-46 NameProvider::SupportsCppClassNamesAsObjectNames
/// 目标配置 (CPPGC_SUPPORTS_OBJECT_NAMES + clang) 恒 true; 官方 Android/Fuchsia 构建为 false,
/// 属配置裁剪面 (五: 架构范围), 不实现。
pub const fn supports_cpp_class_names_as_object_names() -> bool {
  true
}

/// v8: include/cppgc/internal/name-trait.h:67-70 NameTraitBase
pub struct NameTraitBase;

impl NameTraitBase {
  /// v8: include/cppgc/internal/name-trait.h:69 GetNameFromTypeSignature (protected static)
  /// v8: src/heap/cppgc-internal/name-trait.cc:24-38 — 解析 `[T = <名>]` 签名。
  /// 原版 C++ 需以 null 结尾 C 字符串返回而执行 new char[] 拷贝并泄露；
  /// Rust 静态签名借用可以直接作为 &'static str 零拷贝、零分配返回，消除所有 Box::leak。
  pub fn get_name_from_type_signature(signature: Option<&'static str>) -> HeapObjectName {
    let Some(raw) = signature else {
      // 原版 name-trait.cc:27 — 空指针臂
      return HeapObjectName {
        value: K_NO_NAME_DEDUCIBLE,
        name_was_hidden: false,
      };
    };
    if !raw.ends_with(']') {
      return HeapObjectName {
        value: K_NO_NAME_DEDUCIBLE,
        name_was_hidden: false,
      };
    }
    let Some(found) = raw.rfind("T = ") else {
      return HeapObjectName {
        value: K_NO_NAME_DEDUCIBLE,
        name_was_hidden: false,
      };
    };
    let start = found + 4;
    let end = raw.len() - 1;
    let Some(name) = raw.get(start..end) else {
      return HeapObjectName {
        value: K_NO_NAME_DEDUCIBLE,
        name_was_hidden: false,
      };
    };
    HeapObjectName {
      value: name,
      name_was_hidden: false,
    }
  }
}

/// v8: include/cppgc/internal/name-trait.h:74-132 NameTrait<T>
/// 原版 GetNameFor 双过载 (const NameProvider* / const void*) 按 T 是否派生 NameProvider 决议;
/// Rust 无偏特化 → 拆为 get_name (仅 T: NameProvider 可实现) 与 get_name_unnamed 两入口,
/// 两者签名同为 NameCallback, 可直接注册进 GCInfo.name。
pub struct NameTrait<T>(PhantomData<T>);

impl<T> NameTrait<T> {
  /// 原版 name-trait.h:77-85 HasNonHiddenName — 目标配置 CPPGC_SUPPORTS_COMPILE_TIME_TYPENAME=1
  /// 恒 true (name-trait.h:18-19)。
  pub const fn has_non_hidden_name() -> bool {
    true
  }

  /// 原版 name-trait.h:87-90 GetName → GetNameFor(const void*, mode) 无名臂
  /// (name-trait.h:100-131)。obj 在不支持类名的配置下才被读取, 本配置恒不用 → 保留签名位。
  pub fn get_name_unnamed(
    _object: *const c_void,
    mode: HeapObjectNameForUnnamedObject,
  ) -> HeapObjectName {
    if mode == HeapObjectNameForUnnamedObject::UseHiddenName {
      // 原版 :102-104
      return HeapObjectName {
        value: K_HIDDEN_NAME,
        name_was_hidden: true,
      };
    }
    // 原版 :106-107 CPPGC_SUPPORTS_COMPILE_TIME_TYPENAME → GetTypename<T>()
    // Rust 侧 type_name::<T>() 为等价编译期型名 (命名空间分隔符差异登记于台账)。
    HeapObjectName {
      value: type_name::<T>(),
      name_was_hidden: false,
    }
  }
}

impl<T: NameProvider> NameTrait<T> {
  /// 原版 name-trait.h:87-90 GetName → GetNameFor(const NameProvider*, mode) (name-trait.h:93-98)
  /// — 用户已供名, 不算无名。
  ///
  /// # Safety
  /// `object` 必须是 `T` 的有效 payload 指针 (原版 static_cast<const T*>(obj) 的同形契约)。
  pub unsafe fn get_name(
    object: *const c_void,
    _mode: HeapObjectNameForUnnamedObject,
  ) -> HeapObjectName {
    // SAFETY: 调用契约保证 object 为 T 的有效指针, 与原版 static_cast 后解引用同形。
    let provider = unsafe { &*(object.cast::<T>()) };
    HeapObjectName {
      value: provider.get_human_readable_name(),
      name_was_hidden: false,
    }
  }
}
