// 由 tool/src/wasm/baseline/liftoff-assembler-defs.h.js 从 v8/src/wasm/baseline/liftoff-assembler-defs.h 生成; 修订就地改, 复跑覆盖
// Liftoff 缓存寄存器表 (x64+arm64 展开, 其余架构登记注释)

/// x64 目标 (V8_TARGET_ARCH_X64)
pub mod x64 {
  /// kLiftoffAssemblerGpCacheRegs (RegList)
  pub static GP_CACHE_REGS: &[&str] = &[
    "rax", "rcx", "rdx", "rbx", "rsi", "rdi", "r8", "r9", "r12", "r15",
  ];
  /// kLiftoffAssemblerFpCacheRegs (DoubleRegList)
  pub static FP_CACHE_REGS: &[&str] = &[
    "xmm0", "xmm1", "xmm2", "xmm3", "xmm4", "xmm5", "xmm6", "xmm7",
  ];
  /// kLiftoffAssemblerSimd128CacheRegs (Simd128RegList)
  pub static SIMD128_CACHE_REGS: &[&str] = &[];
  /// kLiftoffFrameSetupFunctionReg (WasmLiftoffFrameSetup builtin 用)
  pub const FRAME_SETUP_FUNCTION_REG: &str = "r12";
}

/// arm64 目标 (V8_TARGET_ARCH_ARM64)
pub mod arm64 {
  /// kLiftoffAssemblerGpCacheRegs (RegList)
  pub static GP_CACHE_REGS: &[&str] = &[
    "x0", "x1", "x2", "x3", "x4", "x5", "x6", "x7", "x8", "x9", "x10", "x11", "x12", "x13", "x14",
    "x15", "x19", "x20", "x21", "x22", "x23", "x24", "x25", "x27",
  ];
  /// kLiftoffAssemblerFpCacheRegs (DoubleRegList)
  pub static FP_CACHE_REGS: &[&str] = &[
    "d0", "d1", "d2", "d3", "d4", "d5", "d6", "d7", "d8", "d9", "d10", "d11", "d12", "d13", "d14",
    "d16", "d17", "d18", "d19", "d20", "d21", "d22", "d23", "d24", "d25", "d26", "d27",
  ];
  /// kLiftoffAssemblerSimd128CacheRegs (Simd128RegList)
  pub static SIMD128_CACHE_REGS: &[&str] = &[];
  /// kLiftoffFrameSetupFunctionReg (WasmLiftoffFrameSetup builtin 用)
  pub const FRAME_SETUP_FUNCTION_REG: &str = "x8";
}

// —— 其余架构登记 (源文本见 liftoff-assembler-defs.h 对应分支, 本产物不展开) ——
// IA32: gp=[eax,ecx,edx,esi,edi] fp=[xmm1,xmm2,xmm3,xmm4,xmm5,xmm6] simd=[] setup=edi
// MIPS64: gp=[a0,a1,a2,a3,a4,a5,a6,a7,t0,t1,t2,s7,v0,v1] fp=[f0,f2,f4,f6,f8,f10,f12,f14,f16,f18,f20,f22,f24,f26] simd=[] setup=t0
// LOONG64: gp=[a0,a1,a2,a3,a4,a5,a6,a7,t0,t1,t2,t3,t4,t5,s0,s1,s2,s5,s7] fp=[f0,f1,f2,f3,f4,f5,f6,f7,f8,f9,f10,f11,f12,f13,f14,f15,f16,f17,f18,f19,f20,f21,f22,f23,f24,f25,f26] simd=[] setup=t0
// ARM: gp=[r0,r1,r2,r3,r4,r5,r6,r7,r8,r9] fp=[d0,d1,d2,d3,d4,d5,d6,d7,d8,d9,d10,d11,d12] simd=[] setup=r4
// S390X: gp=[r2,r3,r4,r5,r6,r7,r8,cp] fp=[d0,d1,d2,d3,d4,d5,d6,d7,d8,d9,d10,d11,d12] simd=[] setup=r7
// PPC64: gp=[r3,r4,r5,r6,r7,r8,r9,r10,r15,cp] fp=[d1,d2,d3,d4,d5,d6,d7,d8,d9,d10,d11,d12] simd=[v0,v1,v2,v3,v4,v5,v6,v7,v8,v9,v10,v11,v12] setup=r15
// RISCV32: gp=[a0,a1,a2,a3,a4,a5,a6,a7,s2,t1,t2,s7] fp=[ft1,ft2,ft3,ft4,ft5,ft6,ft7,fa0,fa1,fa2,fa3,fa4,fa5,fa6,fa7,ft8,ft9,ft10] simd=[v1,v4,v5,v6,v7,v8,v9,v10,v11,v12,v13,v14,v15,v16,v17,v18,v19,v20,v21,v22,v23,v27,v28,v29,v30,v31] setup=s2

// —— 提取不变式断言 ——
const _: () = {
  const fn str_slice_contains(slice: &[&str], target: &str) -> bool {
    let mut i = 0;
    while i < slice.len() {
      let a = slice[i].as_bytes();
      let b = target.as_bytes();
      if a.len() == b.len() {
        let mut j = 0;
        let mut same = true;
        while j < a.len() {
          if a[j] != b[j] {
            same = false;
          }
          j += 1;
        }
        if same {
          return true;
        }
      }
      i += 1;
    }
    false
  }

  assert!(x64::GP_CACHE_REGS.len() == 10);
  assert!(x64::FP_CACHE_REGS.len() == 8);
  assert!(x64::SIMD128_CACHE_REGS.is_empty());
  assert!(str_slice_contains(x64::GP_CACHE_REGS, "r15"));
  assert!(!str_slice_contains(x64::GP_CACHE_REGS, "r13")); // r13 = kRootRegister
  assert!(!str_slice_contains(x64::GP_CACHE_REGS, "r10")); // r10 = kScratchRegister
  assert!(!str_slice_contains(x64::GP_CACHE_REGS, "r14")); // r14 = kPtrComprCageBaseRegister
  assert!(!str_slice_contains(x64::FP_CACHE_REGS, "xmm15"));
  assert!(!arm64::GP_CACHE_REGS.is_empty());
  assert!(!arm64::FP_CACHE_REGS.is_empty());
};
