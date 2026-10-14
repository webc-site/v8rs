// 由 tool/src/deoptimizer/deoptimized-frame-info.cc.js 从 v8/src/deoptimizer/deoptimized-frame-info.cc 生成; 修订就地改, 复跑覆盖
// 反优化帧信息骨架

// 语义面: 帧遍历与参数/表达式栈抽取实现登记跳过。
// 构造期填充字段 (3 条):
// 字段面: parameters_ (参数值数组) / context_ / expression_stack_ (表达式栈,
// 不含累加器); 构造期经 SetParameter/SetExpression 逐槽填充。
pub struct DeoptimizedFrameInfo {
  pub parameter_count: usize,
  pub stack_height: usize,
}

impl DeoptimizedFrameInfo {
  /// 构造 (原版 ctor: parameter_count 与表达式栈高, 帧值物化待 translated state 就位)
  #[must_use]
  pub const fn new(parameter_count: usize, stack_height: usize) -> Self {
    Self {
      parameter_count,
      stack_height,
    }
  }
}

impl Default for DeoptimizedFrameInfo {
  fn default() -> Self {
    Self::new(0, 0)
  }
}

// TODO(dev): 依 translated state 补帧值物化。

// —— 提取不变式断言 ——
const _: () = {};
