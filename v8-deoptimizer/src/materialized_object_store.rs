// 由 tool/src/deoptimizer/materialized-object-store.cc.js 从 v8/src/deoptimizer/materialized-object-store.cc 生成; 修订就地改, 复跑覆盖
// 物化对象存储骨架

// 原 class MaterializedObjectStore — 以 stack_id 索引的物化对象栈。
// 方法注册表 (6 条):
// - Get
// - Set
// - Remove
// - StackIdToIndex
// - GetStackEntries
// - EnsureStackEntries
// 字段面: stack_id_ 向量 (每项为一帧的物化对象组)

pub struct MaterializedObjectStore;

impl Default for MaterializedObjectStore {
  /// 构造空存储 (原版 ctor: isolate 挂接面待 execution 层就位)
  fn default() -> Self {
    Self
  }
}

impl MaterializedObjectStore {
  /// 构造空存储 (Default 别名, 原版 ctor 面)
  #[must_use]
  pub const fn new() -> Self {
    Self
  }
}

// TODO(dev): 补存储与移除实现。

// —— 提取不变式断言 ——
const _: () = {};
