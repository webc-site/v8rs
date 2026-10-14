// 由 tool/src/d8/d8-test.cc.js 从 v8/src/d8/d8-test.cc 生成; 修订就地改, 复跑覆盖
// d8 FastCApi 测试对象: 常量 + AccessPolicy 枚举 + 重载表 + 签名骨架 (>2k 行骨架级)

//! d8 FastCApi 测试对象 — 原版 v8/src/d8/d8-test.cc (FastCApiObject + Shell 模板装配)
//!
//! 语义: d8.testFastCApi 暴露的 fast-call/slow-call 双路径测试面 —
//! 验证 CFunction 快速调用、选项袋、重载分派、访问检查拦截。

/// fast-call 嵌入器数据类型标签 (原版 kFastCApiTag)
pub const FAST_API_TAG: u32 = 1;

/// 包装对象内部字段下标 (原版 kV8WrapperObjectIndex)
pub const V8_WRAPPER_OBJECT_INDEX: i32 = 1;

/// 访问策略 (原版 enum class AccessPolicy : uintptr_t, 显式 0..3)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(usize)]
pub enum AccessPolicy {
  /// 原版 kDeny
  Deny = 0,
  /// 原版 kAllow
  Allow = 1,
  /// 原版 kSameContext
  SameContext = 2,
  /// 原版 kSecurityToken
  SecurityToken = 3,
}

/// fast/slow 回调面规模 (骨架级台账; fast 34 (含 Patch 变体) + slow 25)
pub const FAST_CALLBACK_COUNT: usize = 34;
pub const SLOW_CALLBACK_COUNT: usize = 25;

/// CFunction 重载表 (原版 2 张 ×2 条目)
pub const OVERLOAD_TABLES: [(&str, [&str; 2]); 2] = [
  (
    "add_all_overload",
    ["add_all_seq_c_func", "add_all_no_options_c_func"],
  ),
  (
    "overloaded_add_all_32bit_int",
    [
      "add_all_32bit_int_6args_c_func",
      "add_all_32bit_int_5args_c_func",
    ],
  ),
];

// —— 宏台账 (cc 顶部 3 条, 全为自检早退) ——
// CHECK_SELF_OR_THROW_FAST_OPTIONS(return_value) — 带选项袋版本
// CHECK_SELF_OR_THROW_FAST(return_value) — fast 路径
// CHECK_SELF_OR_THROW_SLOW() — slow 路径

// —— Shell:: 签名骨架 (6 条, cc 定义 1:1) ——
// CreateTestFastCApiTemplate — d8.testFastCApi 模板装配 (含重载表注册)
// CreateLeafInterfaceTypeTemplate — 叶接口类型模板 (PerIsolateData 缓存)
// CreateInterceptorObject — 命名/索引拦截器测试对象
// CreateAccessCheckedObject — 访问检查测试对象
// CreateAccessCheckedInterceptorObject — 访问检查+拦截器复合对象
// SetAccessPolicy — d8.test.setAccessPolicy — true/false/same-context/security-token
// FastCApiObject 单例 — instance() 返回 kFastCApiObject 静态实例 (fast-call 自检承载)

// 骨架级登记 (原版 >2k 行, 按 hard-rule 只提取常量/枚举/表/签名;
// AddAll* / Add32BitInt* / Add64BitInt* / CopyString / ThrowNoFallback 等回调族
// 共 fast 34 (含 Patch) + slow 25 逐个签名由 Developer 侧按需展开)
