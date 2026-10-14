// 由 tool/src/sandbox/generated-code-validator.cc.js 从 v8/src/sandbox/generated-code-validator.{h,cc,-x64.cc,-arm64.cc} + sandbox-malloc.h 生成; 修订就地改, 复跑覆盖
// 生成代码校验器骨架

/// 生成代码校验器 (原版 generated-code-validator.h):
/// 沙箱外代码跳入沙箱代码前的指令流完整性校验
pub mod generated_code_validator_skel {
  /// 接口面 (对数: 2)
  pub const API_METHODS: [&str; 2] = ["Validate", "IsValidated"];

  /// 代码生成后校验并打标 (原版 Validate)
  pub fn validate(_isolate_key: usize, _code: usize) {
    unimplemented!("骨架")
  }
  /// 查询校验标记 (原版 IsValidated)
  pub fn is_validated(_code: usize) -> bool {
    unimplemented!("骨架")
  }
  /// 指令流迭代器 (原版 InstructionIteratorSkippingData: 跳过数据段逐指令扫)
  pub fn iterate_instructions_skipping_data() {
    unimplemented!("骨架")
  }
}

/// 架构实现登记 (x64 + arm64 均为当前目标)
pub mod validator_arch {
  /// x64 实现锚点数 (对数)
  pub const X64_UNITS: usize = 3;
  /// arm64 实现锚点数 (对数)
  pub const ARM64_UNITS: usize = 3;
  /// x64 操作数上限 (原版 kMaxOperands = 4, 指令解码记录)
  pub const X64_MAX_OPERANDS: usize = 4;
}

/// 沙箱内分配器 (原版 sandbox-malloc.h: SandboxFreeDeleter 等;
/// 外部字符串等沙箱内缓冲配 unique_ptr 定制删除器)
pub mod sandbox_malloc_skel {
  /// 分配/释放骨架
  pub fn alloc(_size: usize) -> *mut u8 {
    unimplemented!("骨架")
  }
  pub fn dealloc(_ptr: *mut u8, _size: usize) {
    unimplemented!("骨架")
  }
}
