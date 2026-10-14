// 由 tool/src/interpreter/interpreter-intrinsics.h.js 从 v8/src/interpreter/interpreter-intrinsics.{h,cc} 生成; 修订就地改, 复跑覆盖

/// 内在函数表 (原版 INTRINSICS_LIST 展开, 17 项; 列 = 大写名 / 运行时函数名 / 参数个数, -1 为可变)
pub static INTRINSICS: [(&str, &str, i32); 17] = [
  ("AsyncFunctionAwait", "async_function_await_caught", 2),
  ("AsyncFunctionEnter", "async_function_enter", 2),
  ("AsyncFunctionReject", "async_function_reject", 2),
  ("AsyncFunctionResolve", "async_function_resolve", 2),
  ("AsyncGeneratorAwait", "async_generator_await_caught", 2),
  ("AsyncGeneratorReject", "async_generator_reject", 2),
  ("AsyncGeneratorResolve", "async_generator_resolve", 3),
  (
    "AsyncGeneratorYieldWithAwait",
    "async_generator_yield_with_await",
    2,
  ),
  ("CreateJSGeneratorObject", "create_js_generator_object", 2),
  ("GeneratorGetResumeMode", "generator_get_resume_mode", 1),
  ("GeneratorClose", "generator_close", 1),
  ("GetImportMetaObject", "get_import_meta_object", 0),
  ("CopyDataProperties", "copy_data_properties", 2),
  (
    "CopyDataPropertiesWithExcludedPropertiesOnStack",
    "copy_data_properties_with_excluded_properties_on_stack",
    -1,
  ),
  ("CreateIterResultObject", "create_iter_result_object", 2),
  ("GeneratorYieldResult", "generator_yield_result", 2),
  (
    "CreateAsyncFromSyncIterator",
    "create_async_from_sync_iterator",
    1,
  ),
];

/// 内在函数 id (原版 IntrinsicsHelper::IntrinsicId; kIdCount = 17 <= kMaxUInt8)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum IntrinsicId {
  /// 原版 kAsyncFunctionAwait (2 参数)
  AsyncFunctionAwait = 0,
  /// 原版 kAsyncFunctionEnter (2 参数)
  AsyncFunctionEnter = 1,
  /// 原版 kAsyncFunctionReject (2 参数)
  AsyncFunctionReject = 2,
  /// 原版 kAsyncFunctionResolve (2 参数)
  AsyncFunctionResolve = 3,
  /// 原版 kAsyncGeneratorAwait (2 参数)
  AsyncGeneratorAwait = 4,
  /// 原版 kAsyncGeneratorReject (2 参数)
  AsyncGeneratorReject = 5,
  /// 原版 kAsyncGeneratorResolve (3 参数)
  AsyncGeneratorResolve = 6,
  /// 原版 kAsyncGeneratorYieldWithAwait (2 参数)
  AsyncGeneratorYieldWithAwait = 7,
  /// 原版 kCreateJSGeneratorObject (2 参数)
  CreateJSGeneratorObject = 8,
  /// 原版 kGeneratorGetResumeMode (1 参数)
  GeneratorGetResumeMode = 9,
  /// 原版 kGeneratorClose (1 参数)
  GeneratorClose = 10,
  /// 原版 kGetImportMetaObject (0 参数)
  GetImportMetaObject = 11,
  /// 原版 kCopyDataProperties (2 参数)
  CopyDataProperties = 12,
  /// 原版 kCopyDataPropertiesWithExcludedPropertiesOnStack (-1 参数)
  CopyDataPropertiesWithExcludedPropertiesOnStack = 13,
  /// 原版 kCreateIterResultObject (2 参数)
  CreateIterResultObject = 14,
  /// 原版 kGeneratorYieldResult (2 参数)
  GeneratorYieldResult = 15,
  /// 原版 kCreateAsyncFromSyncIterator (1 参数)
  CreateAsyncFromSyncIterator = 16,
}

impl IntrinsicId {
  /// 总数 (原版 kIdCount)
  pub const ID_COUNT: usize = 17;

  /// 对应运行时函数名
  #[must_use]
  pub const fn runtime_function_name(self) -> &'static str {
    INTRINSICS[self as usize].1
  }

  /// 期望参数个数 (-1 表示可变)
  #[must_use]
  pub const fn argument_count(self) -> i32 {
    INTRINSICS[self as usize].2
  }
}

/// 原版 class IntrinsicsHelper 成员声明之外运行时 id 映射为纯查表:
/// FromRuntimeId/ToRuntimeId 由 INTRINSICS 第二列 1:1 反查, 修订阶段接入 runtime crate。
/// 原版 class/struct IntrinsicsHelper 成员声明 (机械 1:1 提取, 3 条)
pub const INTRINSICS_HELPER_DECLARATIONS: &[&str] = &[
  "[public/method] V8_EXPORT_PRIVATE static bool IsSupported(Runtime::FunctionId function_id)",
  "[public/method] static IntrinsicId FromRuntimeId(Runtime::FunctionId function_id)",
  "[public/method] static Runtime::FunctionId ToRuntimeId(IntrinsicId intrinsic_id)",
];

// —— 提取不变式断言 ——
const _: () = {
  assert!(INTRINSICS_HELPER_DECLARATIONS.len() == 3);
  assert!(IntrinsicId::ID_COUNT == 17);
  assert!(matches!(
    IntrinsicId::CopyDataProperties
      .runtime_function_name()
      .as_bytes(),
    b"copy_data_properties"
  ));
  assert!(IntrinsicId::CreateAsyncFromSyncIterator.argument_count() == 1);
};
