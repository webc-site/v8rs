#!/usr/bin/env bun
// tool/src/sandbox/generated-code-validator.cc.js
// 机械提取 v8/src/sandbox/generated-code-validator.{h,cc} + -x64.cc + -arm64.cc
// + sandbox-malloc.h: 生成代码校验器与沙箱分配

import { readV8, writeSrc, banner } from "../../lib.js";

const genH = readV8("src/sandbox/generated-code-validator.h");
const genCc = readV8("src/sandbox/generated-code-validator.cc");
const x64Cc = readV8("src/sandbox/generated-code-validator-x64.cc");
const arm64Cc = readV8("src/sandbox/generated-code-validator-arm64.cc");
const mallocH = readV8("src/sandbox/sandbox-malloc.h");

// 1. 头侧接口面
// 接口面: Validate / IsValidated (静态) + InstructionIteratorSkippingData 迭代器
const apiFns = [...genH.matchAll(/\n  static V8_EXPORT_PRIVATE (?:void|bool) (\w+)\(/g)].map((m) => m[1]);
if (apiFns.join() !== "Validate,IsValidated") throw new Error(`校验器接口漂移: ${apiFns.join()}`);
if (!genH.includes("InstructionIteratorSkippingData")) throw new Error("缺指令迭代器");

// 2. x64/arm64 双实现 (x64+arm64 为当前目标, 均展开)
const x64Fns = (x64Cc.match(/^(?:bool|void|size_t|namespace|extern)/gm) || []).length;
const arm64Fns = (arm64Cc.match(/^(?:bool|void|size_t|namespace|extern)/gm) || []).length;
if (!x64Cc.includes("GeneratedCodeValidator::ValidateImpl") || !x64Cc.includes("kMaxOperands")) throw new Error("x64 校验器锚点漂移");
if (!arm64Cc.includes("GeneratedCodeValidator::ValidateImpl")) throw new Error("arm64 校验器锚点漂移");

// 3. sandbox-malloc.h 面
if (!/template|class|malloc/i.test(mallocH)) throw new Error("sandbox-malloc.h 面漂移");

let rust = banner(
  "src/sandbox/generated-code-validator.cc.js",
  "src/sandbox/generated-code-validator.{h,cc,-x64.cc,-arm64.cc} + sandbox-malloc.h",
  "生成代码校验器骨架",
);

rust += `
/// 生成代码校验器 (原版 generated-code-validator.h):
/// 沙箱外代码跳入沙箱代码前的指令流完整性校验
pub mod generated_code_validator_skel {
  /// 接口面 (对数: ${apiFns.length})
  pub const API_METHODS: [&str; ${apiFns.length}] = [${apiFns.map((f) => `"${f}"`).join(", ")}];

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
  pub const X64_UNITS: usize = ${x64Fns};
  /// arm64 实现锚点数 (对数)
  pub const ARM64_UNITS: usize = ${arm64Fns};
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
`;

const p = writeSrc("v8-sandbox", "code_validator_generated.rs", rust);
console.log(`written ${p}; 接口=${apiFns.length}, x64=${x64Fns}, arm64=${arm64Fns}`);
