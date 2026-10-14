// 由 tool/src/wasm/function-body-decoder.h.js 从 v8/src/wasm/function-body-decoder.{h,cc} 生成; 修订就地改, 复跑覆盖
// 函数体解码入口: FunctionBody 布局 + LoadTransformationKind + 迭代器骨架
// 深水推迟 (W10WC 台账): ValidateFunctionBody/DecodeLocalDecls/BytecodeIterator::opcode/pc_to_next/
// CheckHardwareSupportsSimd 依赖模块字节流与 base::cpu 探测面, 骨架函数已移除,
// 前置: module-decoder 对象面 + v8-base cpu features; 待补波 1:1 填体。

use crate::{value_type::ValueType, wasm_opcodes::SigDef};

/// 函数体定位 (对应 struct FunctionBody; 字段直译: sig/offset/start/end)
#[derive(Debug, Clone, Copy)]
pub struct FunctionBody {
  /// 函数签名 (const FunctionSig* sig)
  pub sig: Option<SigDef>,
  /// 模块字节内偏移 (错误报告定位)
  pub offset: u32,
  /// 函数体起始指针 (以 wire bytes 偏移表示)
  pub start: u32,
  /// 函数体结束
  pub end: u32,
}

/// load 变换种类 (对应 enum class LoadTransformationKind : uint8_t)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum LoadTransformationKind {
  /// kSplat
  Splat = 0,
  /// kExtend
  Extend = 1,
  /// kZeroExtend
  ZeroExtend = 2,
}

/// 局部声明解码结果 (对应 struct BodyLocalDecls)
#[derive(Debug, Clone, Default)]
pub struct BodyLocalDecls {
  /// (数量, 类型) 序列
  pub locals: Vec<(u32, ValueType)>,
  /// 局部声明段之后的解码位置
  pub decoded_offset: u32,
}

/// 字节码迭代器 (对应 class BytecodeIterator; 本轮仅挂载纯游标部分)
pub struct BytecodeIterator {
  pub pc: u32,
  pub end: u32,
}

impl BytecodeIterator {
  /// 对应 reset(pc)
  pub fn reset(&mut self, pc: u32) {
    self.pc = pc;
  }

  /// 对应 has()
  #[must_use]
  pub fn has(&self) -> bool {
    self.pc < self.end
  }
}

// —— 提取不变式断言 ——
const _: () = {
  assert!(LoadTransformationKind::Splat as u8 == 0);
  assert!(LoadTransformationKind::ZeroExtend as u8 == 2);
};
