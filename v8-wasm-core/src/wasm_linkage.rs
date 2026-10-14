// 由 tool/src/wasm/wasm-linkage.h.js 从 v8/src/wasm/wasm-linkage.h 生成; 修订就地改, 复跑覆盖
// wasm 调用约定寄存器表 (x64+arm64 展开, 其余架构登记注释)

use crate::wasm_opcodes::SigType;

/// wasm 实例参数在调用描述符中的固定下标 (对应 kWasmInstanceDataParameterIndex)
pub const WASM_INSTANCE_DATA_PARAMETER_INDEX: i32 = 0;

/// x64 目标下的平台常量 (对应各 constexpr bool, x64 分支取值)
pub mod x64 {
  /// kGpParamRegisters
  pub static GP_PARAM_REGISTERS: &[&str] = &["rsi", "rax", "rdx", "rcx", "rbx", "r9"];
  /// kGpReturnRegisters
  pub static GP_RETURN_REGISTERS: &[&str] = &["rax", "rdx"];
  /// kFpParamRegisters
  pub static FP_PARAM_REGISTERS: &[&str] = &["xmm1", "xmm2", "xmm3", "xmm4", "xmm5", "xmm6"];
  /// kFpReturnRegisters
  pub static FP_RETURN_REGISTERS: &[&str] = &["xmm1", "xmm2"];
  /// kSimd128ParamRegisters
  pub static SIMD128_PARAM_REGISTERS: &[&str] = &[];
  /// kSimd128ReturnRegisters
  pub static SIMD128_RETURN_REGISTERS: &[&str] = &[];

  /// kIsFpAlwaysDouble (x64 = false)
  pub const IS_FP_ALWAYS_DOUBLE: bool = false;
  /// kIsBigEndian (x64 = false)
  pub const IS_BIG_ENDIAN: bool = false;
  /// kIsBigEndianOnSim (x64 = false)
  pub const IS_BIG_ENDIAN_ON_SIM: bool = false;
  /// kFP32NeedNanBox (x64 = false)
  pub const FP32_NEED_NAN_BOX: bool = false;
}

/// arm64 目标下的平台常量 (对应各 constexpr bool, arm64 分支取值)
pub mod arm64 {
  /// kGpParamRegisters
  pub static GP_PARAM_REGISTERS: &[&str] = &["x7", "x0", "x2", "x3", "x4", "x5", "x6"];
  /// kGpReturnRegisters
  pub static GP_RETURN_REGISTERS: &[&str] = &["x0", "x1"];
  /// kFpParamRegisters
  pub static FP_PARAM_REGISTERS: &[&str] = &["d0", "d1", "d2", "d3", "d4", "d5", "d6", "d7"];
  /// kFpReturnRegisters
  pub static FP_RETURN_REGISTERS: &[&str] = &["d0", "d1"];
  /// kSimd128ParamRegisters
  pub static SIMD128_PARAM_REGISTERS: &[&str] = &[];
  /// kSimd128ReturnRegisters
  pub static SIMD128_RETURN_REGISTERS: &[&str] = &[];

  /// kIsFpAlwaysDouble (arm64 = false; 仅 PPC64 为 true)
  pub const IS_FP_ALWAYS_DOUBLE: bool = false;
  /// kIsBigEndian (arm64 = false)
  pub const IS_BIG_ENDIAN: bool = false;
  /// kIsBigEndianOnSim (arm64 = false)
  pub const IS_BIG_ENDIAN_ON_SIM: bool = false;
  /// kFP32NeedNanBox (arm64 = false; 仅 RISCV64 为 true)
  pub const FP32_NEED_NAN_BOX: bool = false;
}

// —— 其余架构登记 (本产物不展开, 源文本位于 wasm-linkage.h 对应分支) ——
// ia32: gp_param=[esi,eax,edx,ecx] gp_ret=[eax,edx] fp_param=[xmm1,xmm2,xmm3,xmm4,xmm5,xmm6] fp_ret=[xmm1,xmm2]
// arm: gp_param=[r3,r0,r2,r6] gp_ret=[r0,r1] fp_param=[d0,d1,d2,d3,d4,d5,d6,d7] fp_ret=[d0,d1]
// mips64: gp_param=[a0,a2,a3,a4,a5,a6,a7] gp_ret=[v0,v1] fp_param=[f2,f4,f6,f8,f10,f12,f14] fp_ret=[f2,f4]
// LOONG64: gp_param=[a7,a0,a2,a3,a4,a5,a6] gp_ret=[a0,a1] fp_param=[f0,f1,f2,f3,f4,f5,f6,f7] fp_ret=[f0,f1]
// ppc64: gp_param=[r10,r3,r5,r6,r7,r8,r9] gp_ret=[r3,r4] fp_param=[d1,d2,d3,d4,d5,d6,d7,d8] fp_ret=[d1,d2]
// s390x: gp_param=[r6,r2,r4,r5] gp_ret=[r2,r3] fp_param=[d0,d2,d4,d6] fp_ret=[d0,d2]
// riscv64: gp_param=[a7,a0,a2,a3,a4,a5,a6] gp_ret=[a0,a1] fp_param=[fa0,fa1,fa2,fa3,fa4,fa5,fa6,fa7] fp_ret=[fa0,fa1]
// unknown: gp_param=[] gp_ret=[] fp_param=[] fp_ret=[]

/// 链接位置分配器 (对应 class LinkageAllocator, 骨架)
pub struct LinkageAllocator {
  gp_count_: usize,
  gp_offset_: usize,
  fp_count_: usize,
  fp_offset_: usize,
  simd128_count_: usize,
  simd128_offset_: usize,
  slot_size_: usize,
}

impl LinkageAllocator {
  /// 对应 LinkageAllocator(gp, gpc, fp, fpc, simd128, simd128c)
  #[must_use]
  pub const fn new(gpc: usize, fpc: usize, simd128c: usize) -> Self {
    Self {
      gp_count_: gpc,
      gp_offset_: 0,
      fp_count_: fpc,
      fp_offset_: 0,
      simd128_count_: simd128c,
      simd128_offset_: 0,
      slot_size_: 0,
    }
  }

  /// 对应 CanAllocateGP()
  #[must_use]
  pub const fn can_allocate_gp(&self) -> bool {
    self.gp_offset_ < self.gp_count_
  }

  /// 对应 CanAllocateFP(rep) (非 ARM 分支: fp_offset_ < fp_count_)
  #[must_use]
  pub const fn can_allocate_fp(&self) -> bool {
    self.fp_offset_ < self.fp_count_
  }

  /// 对应 CanAllocateSimd128(rep)
  #[must_use]
  pub const fn can_allocate_simd128(&self) -> bool {
    self.simd128_offset_ < self.simd128_count_
  }

  /// 对应 NextGpReg() -> 寄存器编号 (骨架: 返回游标, 具体编码待修订)
  pub fn next_gp_reg(&mut self) -> usize {
    unimplemented!("骨架: 游标自增待与寄存器编码表对接")
  }

  /// 对应 NextFpReg(rep) (非 ARM 分支)
  pub fn next_fp_reg(&mut self) -> usize {
    unimplemented!("骨架: 游标自增待与寄存器编码表对接")
  }

  /// 对应 HasSimd128Regs()
  #[must_use]
  pub const fn has_simd128_regs(&self) -> bool {
    self.simd128_count_ > 0
  }

  /// 对应 NumStackSlots()
  #[must_use]
  pub const fn num_stack_slots(&self) -> usize {
    self.slot_size_
  }

  /// 对应 EndSlotArea()
  pub fn end_slot_area(&mut self) {
    unimplemented!("骨架: 对齐槽分配器待对接");
  }
}

/// 参数/返回位置分配助手 (对应 class LinkageLocationAllocator, 骨架)
/// 字段逐字保留原版成员名; slot_offset_ 读面待 Next 骨架对接后启用
#[allow(dead_code)]
pub struct LinkageLocationAllocator {
  allocator_: LinkageAllocator,
  slot_offset_: i32,
}

impl LinkageLocationAllocator {
  /// 对应模板构造 (gp, fp, simd128, slot_offset)
  #[must_use]
  pub const fn new(gpc: usize, fpc: usize, simd128c: usize, slot_offset: i32) -> Self {
    Self {
      allocator_: LinkageAllocator::new(gpc, fpc, simd128c),
      slot_offset_: slot_offset,
    }
  }

  /// 对应 Next(rep): 先寄存器后栈槽
  pub fn next(&mut self, _rep: SigType) -> LinkageLocation {
    unimplemented!("骨架: 参照原版 LinkageLocationAllocator::Next 修订")
  }

  /// 对应 NumStackSlots()
  #[must_use]
  pub const fn num_stack_slots(&self) -> usize {
    self.allocator_.num_stack_slots()
  }

  /// 对应 EndSlotArea()
  pub fn end_slot_area(&mut self) {
    self.allocator_.end_slot_area();
  }
}

/// 链接位置 (对应 LinkageLocation, 最小骨架)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LinkageLocation {
  /// 寄存器位置 (编号, 类型)
  Register(i32, SigType),
  /// 调用者栈槽 (负索引)
  CallerFrameSlot(i32, SigType),
}

// —— 提取不变式断言 ——
const _: () = {
  use crate::table_eq::str_eq;
  assert!(WASM_INSTANCE_DATA_PARAMETER_INDEX == 0);
  assert!(x64::GP_PARAM_REGISTERS.len() == 6);
  assert!(x64::GP_RETURN_REGISTERS.len() == 2);
  assert!(x64::FP_PARAM_REGISTERS.len() == 6);
  assert!(x64::FP_RETURN_REGISTERS.len() == 2);
  assert!(arm64::GP_PARAM_REGISTERS.len() == 7);
  assert!(arm64::GP_RETURN_REGISTERS.len() == 2);
  assert!(arm64::FP_PARAM_REGISTERS.len() == 8);
  assert!(str_eq(
    x64::GP_PARAM_REGISTERS[WASM_INSTANCE_DATA_PARAMETER_INDEX as usize],
    "rsi"
  ));
  assert!(str_eq(
    arm64::GP_PARAM_REGISTERS[WASM_INSTANCE_DATA_PARAMETER_INDEX as usize],
    "x7"
  ));
  assert!(x64::SIMD128_PARAM_REGISTERS.is_empty());
  assert!(arm64::SIMD128_PARAM_REGISTERS.is_empty());
};
