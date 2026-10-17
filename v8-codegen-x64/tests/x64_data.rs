// v8-codegen-x64 wave1 金标准对表测试
//
// 期望值直接抄自 v8/src/codegen/x64/{constants-x64.h, register-x64.h} 与
// v8/src/codegen/register-base.h (C++ 侧为独立事实源), 与生成脚本互为对账。

use v8_codegen_x64::registers as rg;

// register-base.h 基准编码
#[test]
fn no_reg_baseline() {
  assert_eq!(rg::K_CODE_NO_REG, -1); // kCode_no_reg = -1
  assert_eq!(rg::K_NUM_REGS, 16); // kNumRegs = 16
  assert!(!rg::NO_REG.is_valid());
  assert_eq!(rg::NO_REG.code(), rg::K_CODE_NO_REG);
  assert_eq!(rg::register_name(rg::NO_REG), "invalid");
  assert!(rg::RAX.is_valid());
}

// 通用寄存器名表 + 编号映射 (GENERAL_REGISTERS / RegisterCode)
#[test]
fn gpr_codes_and_names() {
  const NAMES: [&str; 16] = [
    "rax", "rcx", "rdx", "rbx", "rsp", "rbp", "rsi", "rdi", "r8", "r9", "r10", "r11", "r12", "r13",
    "r14", "r15",
  ];
  assert_eq!(rg::GENERAL_REG_NAMES, NAMES);
  const CODES: [i8; 16] = [
    rg::reg_code::RAX,
    rg::reg_code::RCX,
    rg::reg_code::RDX,
    rg::reg_code::RBX,
    rg::reg_code::RSP,
    rg::reg_code::RBP,
    rg::reg_code::RSI,
    rg::reg_code::RDI,
    rg::reg_code::R8,
    rg::reg_code::R9,
    rg::reg_code::R10,
    rg::reg_code::R11,
    rg::reg_code::R12,
    rg::reg_code::R13,
    rg::reg_code::R14,
    rg::reg_code::R15,
  ];
  for (i, c) in CODES.iter().enumerate() {
    assert_eq!(*c, i as i8, "RegisterCode 序漂移 @ {i}");
  }
  assert_eq!(rg::reg_code::K_REG_AFTER_LAST, 16); // kRegAfterLast
  let regs: [rg::Register; 16] = [
    rg::RAX,
    rg::RCX,
    rg::RDX,
    rg::RBX,
    rg::RSP,
    rg::RBP,
    rg::RSI,
    rg::RDI,
    rg::R8,
    rg::R9,
    rg::R10,
    rg::R11,
    rg::R12,
    rg::R13,
    rg::R14,
    rg::R15,
  ];
  for (i, r) in regs.iter().enumerate() {
    assert_eq!(r.code(), i as i8);
    assert_eq!(rg::register_name(*r), NAMES[i]);
  }
}

// 枚举互斥: 编码两两不同、名字两两不同
#[test]
fn code_exclusion() {
  let regs: [rg::Register; 16] = [
    rg::RAX,
    rg::RCX,
    rg::RDX,
    rg::RBX,
    rg::RSP,
    rg::RBP,
    rg::RSI,
    rg::RDI,
    rg::R8,
    rg::R9,
    rg::R10,
    rg::R11,
    rg::R12,
    rg::R13,
    rg::R14,
    rg::R15,
  ];
  for i in 0..16 {
    for j in (i + 1)..16 {
      assert_ne!(regs[i], regs[j], "GPR 编码冲突 {i}!={j}");
    }
  }
  assert_eq!(rg::GENERAL_REG_NAMES.len(), 16);
  let mut sorted = rg::GENERAL_REG_NAMES.to_vec();
  sorted.sort_unstable();
  assert!(
    sorted.windows(2).all(|w| w[0] != w[1]),
    "GPR 名字表存在重名"
  );

  let dregs: [rg::XMMRegister; 32] = [
    rg::XMM0,
    rg::XMM1,
    rg::XMM2,
    rg::XMM3,
    rg::XMM4,
    rg::XMM5,
    rg::XMM6,
    rg::XMM7,
    rg::XMM8,
    rg::XMM9,
    rg::XMM10,
    rg::XMM11,
    rg::XMM12,
    rg::XMM13,
    rg::XMM14,
    rg::XMM15,
    rg::XMM16,
    rg::XMM17,
    rg::XMM18,
    rg::XMM19,
    rg::XMM20,
    rg::XMM21,
    rg::XMM22,
    rg::XMM23,
    rg::XMM24,
    rg::XMM25,
    rg::XMM26,
    rg::XMM27,
    rg::XMM28,
    rg::XMM29,
    rg::XMM30,
    rg::XMM31,
  ];
  for i in 0..32 {
    assert_eq!(dregs[i].code(), i as i8);
    for j in (i + 1)..32 {
      assert_ne!(dregs[i], dregs[j], "XMM 编码冲突");
    }
  }
}

// XMM/YMM 名字表与 afterLast (static_assert kDoubleAfterLast == kYMMAfterLast)
#[test]
fn xmm_ymm_tables() {
  for i in 0..16 {
    assert_eq!(rg::XMM_REG_NAMES[i], format!("xmm{i}"));
    assert_eq!(rg::YMM_REG_NAMES[i], format!("ymm{i}"));
    assert_eq!(rg::XMM_EXT_REG_NAMES[i], format!("xmm{}", i + 16));
    assert_eq!(rg::YMM_EXT_REG_NAMES[i], format!("ymm{}", i + 16));
    assert_eq!(rg::ALL_XMM_REG_NAMES[i], format!("xmm{i}"));
    assert_eq!(rg::ALL_YMM_REG_NAMES[i], format!("ymm{i}"));
  }
  assert_eq!(rg::XMM_REG_NAMES.len(), rg::YMM_REG_NAMES.len());
  assert_eq!(rg::ALL_XMM_REG_NAMES.len(), 32);
  assert_eq!(rg::ALL_YMM_REG_NAMES.len(), 32);
  assert_eq!(
    rg::double_code::K_DOUBLE_AFTER_LAST,
    rg::ymm_code::K_YMM_AFTER_LAST
  );
  assert_eq!(rg::double_code::K_DOUBLE_AFTER_LAST, 16);
  assert_eq!(rg::double_code::K_DOUBLE_AFTER_LAST_AVX10, 32);
  assert_eq!(rg::ymm_code::K_YMM_AFTER_LAST_AVX10, 32);
  assert_eq!(rg::xmm_register_name(rg::XMM0), "xmm0");
  assert_eq!(rg::xmm_register_name(rg::NO_DREG), "invalid");
  assert_eq!(rg::ymm_register_name(rg::YMM15), "ymm15");
  assert_eq!(rg::NO_DREG.code(), rg::K_CODE_NO_REG);
  assert_eq!(rg::double_code::XMM15, 15);
  assert_eq!(rg::ymm_code::YMM31, 31);
}

// REX/modR/M 编码位规则 (bit4 / high_bit / low_bits / is_byte_register)
#[test]
fn encoding_bit_rules() {
  for code in 0..16i8 {
    let r = rg::Register::from_code(code);
    assert_eq!(r.bit4(), ((code >> 4) & 0x1) as u8);
    assert_eq!(r.high_bit(), ((code >> 3) & 1) as u8); // APX 掩码支公式 (register-x64.h:70; 非掩码支为 code>>3, hdr:72, 域 0-15 数值等价)
    assert_eq!(r.low_bits(), (code & 0x7) as u8);
    assert_eq!(r.is_byte_register(), code <= 3);
    assert_eq!(
      r.high_bit() * 8 + r.low_bits(),
      code as u8,
      "编码可逆重建 code={code}"
    );
  }
  assert!(rg::RAX.is_byte_register() && rg::RBX.is_byte_register());
  assert!(!rg::RSP.is_byte_register());
  assert_eq!(rg::R8.high_bit(), 1);
  assert_eq!(rg::RAX.high_bit(), 0);
}

// C 调用约定双 ABI 表
#[test]
fn abi_tables() {
  assert_eq!(rg::K_C_ARG_REGS_WIN, [rg::RCX, rg::RDX, rg::R8, rg::R9]);
  assert_eq!(
    rg::K_C_ARG_REGS_SYSV,
    [rg::RDI, rg::RSI, rg::RDX, rg::RCX, rg::R8, rg::R9]
  );
  assert_eq!(rg::K_REGISTER_PASSED_ARGUMENTS_WIN, 4);
  assert_eq!(rg::K_REGISTER_PASSED_ARGUMENTS_SYSV, 6);
  assert_eq!(rg::K_WINDOWS_HOME_STACK_SLOTS, 4);
  assert_eq!(
    rg::C_CALL_CALLEE_SAVE_REGISTERS_WIN,
    [
      rg::RBX,
      rg::RDI,
      rg::RSI,
      rg::R12,
      rg::R13,
      rg::R14,
      rg::R15
    ]
  );
  assert_eq!(
    rg::C_CALL_CALLEE_SAVE_REGISTERS_SYSV,
    [rg::RBX, rg::R12, rg::R13, rg::R14, rg::R15]
  );
  let fp_win: [rg::XMMRegister; 10] = [
    rg::XMM6,
    rg::XMM7,
    rg::XMM8,
    rg::XMM9,
    rg::XMM10,
    rg::XMM11,
    rg::XMM12,
    rg::XMM13,
    rg::XMM14,
    rg::XMM15,
  ];
  assert_eq!(rg::C_CALL_CALLEE_SAVE_FP_REGISTERS_WIN, fp_win);
  assert!(rg::C_CALL_CALLEE_SAVE_FP_REGISTERS_SYSV.is_empty());
}

// 调用约定别名常量对表
#[test]
fn alias_map() {
  let pairs: [(&str, rg::Register); 24] = [
    ("kStackPointerRegister", rg::K_STACK_POINTER_REGISTER),
    ("kReturnRegister0", rg::K_RETURN_REGISTER0),
    ("kReturnRegister1", rg::K_RETURN_REGISTER1),
    ("kReturnRegister2", rg::K_RETURN_REGISTER2),
    ("kJSFunctionRegister", rg::K_JS_FUNCTION_REGISTER),
    ("kContextRegister", rg::K_CONTEXT_REGISTER),
    ("kAllocateSizeRegister", rg::K_ALLOCATE_SIZE_REGISTER),
    (
      "kInterpreterAccumulatorRegister",
      rg::K_INTERPRETER_ACCUMULATOR_REGISTER,
    ),
    (
      "kInterpreterBytecodeOffsetRegister",
      rg::K_INTERPRETER_BYTECODE_OFFSET_REGISTER,
    ),
    (
      "kInterpreterBytecodeArrayRegister",
      rg::K_INTERPRETER_BYTECODE_ARRAY_REGISTER,
    ),
    (
      "kInterpreterDispatchTableRegister",
      rg::K_INTERPRETER_DISPATCH_TABLE_REGISTER,
    ),
    (
      "kJavaScriptCallArgCountRegister",
      rg::K_JAVA_SCRIPT_CALL_ARG_COUNT_REGISTER,
    ),
    (
      "kJavaScriptCallCodeStartRegister",
      rg::K_JAVA_SCRIPT_CALL_CODE_START_REGISTER,
    ),
    (
      "kJavaScriptCallTargetRegister",
      rg::K_JAVA_SCRIPT_CALL_TARGET_REGISTER,
    ),
    (
      "kJavaScriptCallNewTargetRegister",
      rg::K_JAVA_SCRIPT_CALL_NEW_TARGET_REGISTER,
    ),
    (
      "kJavaScriptCallExtraArg1Register",
      rg::K_JAVA_SCRIPT_CALL_EXTRA_ARG1_REGISTER,
    ),
    (
      "kJavaScriptCallDispatchHandleRegister",
      rg::K_JAVA_SCRIPT_CALL_DISPATCH_HANDLE_REGISTER,
    ),
    (
      "kRuntimeCallFunctionRegister",
      rg::K_RUNTIME_CALL_FUNCTION_REGISTER,
    ),
    (
      "kRuntimeCallArgCountRegister",
      rg::K_RUNTIME_CALL_ARG_COUNT_REGISTER,
    ),
    ("kRuntimeCallArgvRegister", rg::K_RUNTIME_CALL_ARGV_REGISTER),
    ("kWasmImplicitArgRegister", rg::K_WASM_IMPLICIT_ARG_REGISTER),
    (
      "kWasmTrapHandlerFaultAddressRegister",
      rg::K_WASM_TRAP_HANDLER_FAULT_ADDRESS_REGISTER,
    ),
    ("kScratchRegister", rg::K_SCRATCH_REGISTER),
    ("kRootRegister", rg::K_ROOT_REGISTER),
  ];
  let golden: [rg::Register; 24] = [
    rg::RSP,
    rg::RAX,
    rg::RDX,
    rg::R8,
    rg::RDI,
    rg::RSI,
    rg::RDX,
    rg::RAX,
    rg::R9,
    rg::R12,
    rg::R15,
    rg::RAX,
    rg::RCX,
    rg::RDI,
    rg::RDX,
    rg::RBX,
    rg::R15,
    rg::RBX,
    rg::RAX,
    rg::R15,
    rg::RSI,
    rg::R10,
    rg::R10,
    rg::R13,
  ];
  assert_eq!(pairs.len(), golden.len());
  for (i, (name, got)) in pairs.iter().enumerate() {
    assert_eq!(*got, golden[i], "别名 {name} 指向漂移");
  }
  // 指针压缩双支
  assert_eq!(rg::K_PTR_COMPR_CAGE_BASE_REGISTER_WITH_COMPRESSION, rg::R14);
  assert_eq!(
    rg::K_PTR_COMPR_CAGE_BASE_REGISTER_WITHOUT_COMPRESSION,
    rg::NO_REG
  );
  // Double/YMM 别名
  assert_eq!(rg::K_SCRATCH_DOUBLE_REG, rg::XMM15);
  assert_eq!(rg::K_SCRATCH_SIMD256_REG, rg::YMM15);
  assert_eq!(rg::K_FP_RETURN_REGISTER0, rg::XMM0);
  const _: () = assert!(!rg::K_SIMD_MASK_REGISTERS);
}

// 可分配寄存器集合对表 + 集合关系
#[test]
fn allocatable_sets() {
  assert_eq!(
    rg::ALWAYS_ALLOCATABLE_GENERAL_REGISTERS,
    [
      rg::RAX,
      rg::RBX,
      rg::RDX,
      rg::RCX,
      rg::RSI,
      rg::RDI,
      rg::R8,
      rg::R9,
      rg::R11,
      rg::R12,
      rg::R15
    ]
  );
  assert!(rg::MAYBE_ALLOCATABLE_GENERAL_REGISTERS_WITH_COMPRESSION.is_empty());
  assert_eq!(
    rg::MAYBE_ALLOCATABLE_GENERAL_REGISTERS_WITHOUT_COMPRESSION,
    [rg::R14]
  );
  assert_eq!(
    rg::ALLOCATABLE_GENERAL_REGISTERS_WITH_COMPRESSION,
    rg::ALWAYS_ALLOCATABLE_GENERAL_REGISTERS
  );
  assert_eq!(
    rg::ALLOCATABLE_GENERAL_REGISTERS_WITHOUT_COMPRESSION,
    [
      rg::RAX,
      rg::RBX,
      rg::RDX,
      rg::RCX,
      rg::RSI,
      rg::RDI,
      rg::R8,
      rg::R9,
      rg::R11,
      rg::R12,
      rg::R15,
      rg::R14
    ]
  );
  // rsp/rbp/r10/r13/r14(压缩支) 不在 always 集内
  for banned in [rg::RSP, rg::RBP, rg::R10, rg::R13] {
    assert!(!rg::ALWAYS_ALLOCATABLE_GENERAL_REGISTERS.contains(&banned));
  }
  let alloc_d: [rg::XMMRegister; 15] = [
    rg::XMM0,
    rg::XMM1,
    rg::XMM2,
    rg::XMM3,
    rg::XMM4,
    rg::XMM5,
    rg::XMM6,
    rg::XMM7,
    rg::XMM8,
    rg::XMM9,
    rg::XMM10,
    rg::XMM11,
    rg::XMM12,
    rg::XMM13,
    rg::XMM14,
  ];
  assert_eq!(rg::ALLOCATABLE_DOUBLE_REGISTERS, alloc_d);
}

// ArgumentPaddingSlots 双支 (V8_X64_16BYTE_STACK_ALIGNMENT_BOOL true/false)
#[test]
fn argument_padding_slots() {
  for n in 0..8i32 {
    assert_eq!(rg::argument_padding_slots(n, false), 0);
    assert_eq!(rg::argument_padding_slots(n, true), n & 1);
  }
}

// TaggedRegister 薄封装语义
#[test]
fn tagged_register_passthrough() {
  let t = rg::TaggedRegister::new(rg::K_JS_FUNCTION_REGISTER);
  assert_eq!(t.reg, rg::RDI);
}

// D1 回归: 越界 code 镜像原版 uint16_t 窄化 no-op (不 panic、不误置 bit0/bit4)
#[test]
fn reglist_phantom_code_noop() {
  let mut list = rg::RegList::empty();
  for code in [16i8, 20, 31, 32, 100] {
    let phantom = rg::Register::with_code(code);
    list.set(phantom);
    assert!(!list.has(phantom));
    assert_eq!(list.len(), 0);
    assert_eq!(list.bits(), 0);
  }
  // 域内合法 code 不受影响
  list.set(rg::RAX);
  assert!(list.has(rg::RAX));
  list.clear(rg::RAX);
  assert_eq!(list.bits(), 0);
}

// round4 D1 强化: 三张名表 invalid 支全覆盖 (ext 段 16-31 + 越界/负值/no_reg)
#[test]
fn name_table_invalid_branches() {
  // GENERAL: with_code 绕过 from_code 守护直达 invalid 支
  assert_eq!(rg::register_name(rg::NO_REG), "invalid");
  assert_eq!(rg::register_name(rg::Register::with_code(16)), "invalid");
  assert_eq!(rg::register_name(rg::Register::with_code(-2)), "invalid");
  assert_eq!(
    rg::register_name(rg::Register::with_code(i8::MAX)),
    "invalid"
  );
  assert_eq!(rg::register_name(rg::R15), "r15");
  // XMM: 32 项表, ext 段可达
  for code in 16..32i8 {
    let name = rg::xmm_register_name(rg::XMMRegister::with_code(code));
    assert_eq!(name, format!("xmm{code}"), "XMM ext 名表 code={code}");
  }
  assert_eq!(rg::xmm_register_name(rg::NO_DREG), "invalid");
  assert_eq!(
    rg::xmm_register_name(rg::XMMRegister::with_code(32)),
    "invalid"
  );
  assert_eq!(
    rg::xmm_register_name(rg::XMMRegister::with_code(-2)),
    "invalid"
  );
  // YMM: ext 段可达 (AVX10 支名表物化)
  for code in 16..32i8 {
    let name = rg::ymm_register_name(rg::YMMRegister::with_code(code));
    assert_eq!(name, format!("ymm{code}"), "YMM ext 名表 code={code}");
  }
  assert_eq!(rg::ymm_register_name(rg::NO_YMM_REG), "invalid");
  assert_eq!(
    rg::ymm_register_name(rg::YMMRegister::with_code(32)),
    "invalid"
  );
  assert_eq!(
    rg::ymm_register_name(rg::YMMRegister::with_code(-2)),
    "invalid"
  );
}

// round4 D2 强化: from_xmm 码恒等 + no_reg 三态 + K_SIMD_MASK 运行时断言
#[test]
fn ymm_from_xmm_and_no_reg() {
  for code in 0..16i8 {
    let xmm = rg::XMMRegister::with_code(code);
    let ymm = rg::YMMRegister::from_xmm(xmm);
    assert_eq!(ymm.code(), code, "from_xmm 码恒等 code={code}");
    assert!(ymm.is_valid());
    assert_eq!(
      rg::ymm_register_name(ymm),
      rg::xmm_register_name(xmm).replace("xmm", "ymm")
    );
  }
  // no_reg 一律 kCode_no_reg=-1, is_valid 仅排除 -1 (register-base.h:41)
  assert!(!rg::NO_REG.is_valid());
  assert!(!rg::NO_DREG.is_valid());
  assert!(!rg::NO_YMM_REG.is_valid());
  assert_eq!(rg::NO_DREG.code(), rg::K_CODE_NO_REG);
  assert_eq!(rg::NO_YMM_REG.code(), rg::K_CODE_NO_REG);
  // x64 无独立 SIMD mask 寄存器组 (register-x64.h kSimdMaskRegisters=false)
  const _: () = assert!(!rg::K_SIMD_MASK_REGISTERS);
}
