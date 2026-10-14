// 由 tool/src/extensions/externalize-string-extension.h.js 从 v8/src/extensions/externalize-string-extension.{h,cc} 生成; 修订就地改, 复跑覆盖
// ExternalizeString 扩展: 最小外化长度公式 + native 表 + 签名骨架

//! ExternalizeString 扩展 — 原版 namespace v8::internal::ExternalizeStringExtension
//!
//! 语义: 注入 4 个 native 函数, 把堆内字符串外化为 ExternalString 资源;
//! CreateExternalizable* 负责把不可外化字符串拷入旧生代/展平 ConsString。

/// ExternalizeString 扩展注册名
pub const EXTERNALIZE_STRING_EXTENSION_NAME: &str = "v8/externalize";

/// 栈上源码缓冲容量 (原版 char buffer_[400])
pub const EXTERNALIZE_STRING_BUFFER_SIZE: usize = 400;

/// 注入 JS 的 let 常量名 (原版 BuildSource %d 占位, 顺序与 MIN_LENGTHS 一致)
pub const JS_CONST_NAMES: [&str; 4] = [
  "kExternalStringMinOneByteLength",
  "kExternalStringMinTwoByteLength",
  "kExternalStringMinOneByteCachedLength",
  "kExternalStringMinTwoByteCachedLength",
];

/// 最小外化长度公式表 (原版 4 条 constexpr, S=外部指针槽宽, T=Tagged 槽宽, U16=2)
/// 公式为平台相关值 (32/64 位沙箱配置不同), 保留符号形式待 Developer 侧代入
pub const MIN_LENGTH_FORMULAS: [(&str, &str); 4] = [
  (
    "kMinOneByteLength",
    "kExternalPointerSlotSize - kTaggedSize + 1",
  ),
  (
    "kMinTwoByteLength",
    "(kExternalPointerSlotSize - kTaggedSize) / sizeof(base::uc16) + 1",
  ),
  (
    "kMinOneByteCachedLength",
    "2 * kExternalPointerSlotSize - kTaggedSize + 1",
  ),
  (
    "kMinTwoByteCachedLength",
    "(2 * kExternalPointerSlotSize - kTaggedSize) / sizeof(base::uc16) + 1",
  ),
];

/// native 函数名表 (原版 BuildSource, 顺序 1:1)
pub const NATIVE_FUNCTIONS: [&str; 4] = [
  "externalizeString",
  "createExternalizableString",
  "createExternalizableTwoByteString",
  "isOneByteString",
];

// —— 提取不变式断言 ——
const _: () = {
  assert!(EXTERNALIZE_STRING_BUFFER_SIZE == 400);
  assert!(JS_CONST_NAMES.len() == 4);
  assert!(MIN_LENGTH_FORMULAS.len() == 4);
  assert!(NATIVE_FUNCTIONS.len() == 4);
};

// —— 签名骨架 (h 声明 6 / cc 定义 6, 1:1; 均为 ConstructorBehavior::kThrow) ——
// public: GetNativeFunctionTemplate(isolate, name) — 按 name 四分发
// public static: Externalize(info) — SupportsExternalization 失败且非已外化则抛错;
//                共享串外化竞态 (HasExternalForwardingIndex) 不算失败
// public static: CreateExternalizableString(info)
// public static: CreateExternalizableTwoByteString(info)
// public static: IsOneByte(info) — 单 string 参, 回 IsOneByteRepresentation
// private static: BuildSource(buf, size) -> *const c_char
// 匿名 namespace:
//   HasExternalForwardingIndex(string) -> bool — 共享串外化转发索引探测
//   CopyConsStringToOld(isolate, cons_string, encoding) — ConsString 拷旧生代
//   CreateExternalizableString(isolate, string, encoding) — RO 串拒外化;
//     V8_COMPRESS_POINTERS 下短串拒外化; ConsString 非平且首部非空走拷贝路径

// 平台分支台账:
//   #ifdef V8_COMPRESS_POINTERS — "String is too short to be externalized." 分支,
//   x64/arm64 默认开启指针压缩 → 展开; 非 V8_ENABLE_DIRECT_HANDLE 的 OpenHandle
//   间接句柄变体属 api-inl 层, 此处登记不展开
