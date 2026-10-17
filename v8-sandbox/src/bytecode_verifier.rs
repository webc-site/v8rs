// 由 tool/src/sandbox/bytecode-verifier.h.js 从 v8/src/sandbox/bytecode-verifier.{h,cc} 生成; 修订就地改, 复跑覆盖
// BytecodeVerifier 骨架

// 沙箱字节码校验器 — 校验逻辑 (cc 328 行) 为纯语义面, 登记跳过。

/// 原 class BytecodeVerifier (全静态接口)
pub struct BytecodeVerifier;

impl BytecodeVerifier {
  // 原签名: static Verify
  // 原签名: static VerifyLight
  // 原签名: static VerifyFull
  // 原签名: static IsAllowedRuntimeFunction
  // 原签名: static FailVerification
  // 原签名: static Check
}

// FailVerification 原签名参数: (const char* reason)
// TODO(dev): 按字节码遍历补 Verify/VerifyLight/VerifyFull 校验实现。

// —— 提取不变式断言 ——
const _: () = {};
