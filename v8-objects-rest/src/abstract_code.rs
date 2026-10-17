// 由 tool/src/objects/abstract-code.h.js 从 v8/src/objects/abstract-code.{h,cc,-inl.h} 生成; 修订就地改, 复跑覆盖
// Code|BytecodeArray 双分发骨架

//! AbstractCode 包装 (原版 abstract-code.{h,cc,-inl.h})
//!
//! AbstractCode 是 {Code | BytecodeArray} 的剖析期 (profiling) 联合包装:
//! BytecodeArray 在 trusted space (主 cage 外), Code 在 sandbox 内,
//! 故比较必须走全指针 (非压缩指针) — 原版 static_assert(!kAllCodeObjectsLiveInTrustedSpace).
//!
//! 推迟登记 (见 readme/zh.md 台账):
//! - `source_position`/`contains` Code 支依赖 isolate/位置表 (归 execution/codegen-core 席);
//! - `size_including_metadata` 两支尺寸面已就位 (Code/BytecodeArray 各自填体)。

use v8_objects_core::{
  instance_type_checker as itc,
  objects::instance_type_of,
  tagged::{Object, Tagged},
};

use crate::{
  bytecode_array::{BytecodeArray, BytecodeArrayExt},
  code::{BYTECODE_OFFSET_NONE, Code, CodeExt, K_NO_BUILTIN_ID},
  code_kind::CodeKind,
  error::Result,
};

/// AbstractCode 联合句柄标记 (原版 class AbstractCode final : public Union<Code, BytecodeArray>)
///
/// 单元标记: AbstractCode 无独立布局 (它就是 Code|BytecodeArray 二选一的全指针),
/// 判别经 map 的 instance type; 两支方法面见 AbstractCodeExt。
pub struct AbstractCode;

/// AbstractCode 分发面 (挂载于 Tagged&lt;AbstractCode&gt;, 逐法 IsCode 双分发)
pub trait AbstractCodeExt: Copy {
  /// 判别当前是否 Code 支 (原版 InstanceTypeChecker::IsCode(map()))
  fn is_code_variant(&self) -> bool;
  /// v8: src/objects/abstract-code-inl.h AbstractCode::GetCode
  fn get_code(&self) -> Tagged<Code>;
  /// v8: src/objects/abstract-code-inl.h AbstractCode::GetBytecodeArray
  fn get_bytecode_array(&self) -> Tagged<BytecodeArray>;
  /// v8: src/objects/abstract-code-inl.h AbstractCode::ToUnionType
  /// (基线: 联合体句柄即自身)
  fn to_union_type(&self) -> Tagged<AbstractCode>;
  /// v8: src/objects/abstract-code-inl.h AbstractCode::InstructionSize
  fn instruction_size(&self) -> Result<i32>;
  /// v8: src/objects/abstract-code-inl.h AbstractCode::InstructionStart
  fn instruction_start(&self) -> usize;
  /// v8: src/objects/abstract-code-inl.h AbstractCode::InstructionEnd
  fn instruction_end(&self) -> usize;
  /// v8: src/objects/abstract-code-inl.h AbstractCode::kind
  fn kind(&self) -> Result<CodeKind>;
  /// v8: src/objects/abstract-code-inl.h AbstractCode::builtin_id
  fn builtin_id(&self) -> i16;
  /// v8: src/objects/abstract-code-inl.h AbstractCode::has_instruction_stream
  fn has_instruction_stream(&self) -> bool;
  /// v8: src/objects/abstract-code.h AbstractCode::is_context_specialized
  fn is_context_specialized(&self) -> bool;
  /// v8: src/objects/abstract-code.h AbstractCode::osr_offset
  fn osr_offset(&self) -> i32;
  /// v8: src/objects/abstract-code-inl.h AbstractCode::SizeIncludingMetadata
  fn size_including_metadata(&self) -> Result<i32>;
  /// v8: src/objects/abstract-code-inl.h AbstractCode::contains
  /// (Code 支依赖 isolate → 台账)
  fn contains(&self, inner_pointer: usize) -> Result<bool>;
  /// v8: src/objects/abstract-code-inl.h AbstractCode::SourcePosition
  /// (两支位置表面就位即填)
  fn source_position(&self, offset: i32) -> Result<i32>;
  /// v8: src/objects/abstract-code.h AbstractCode::SourceStatementPosition
  /// (两支位置表面就位即填)
  fn source_statement_position(&self, offset: i32) -> Result<i32>;
}

impl AbstractCodeExt for Tagged<AbstractCode> {
  #[inline]
  fn is_code_variant(&self) -> bool {
    // 原版: map() 的 instance type 判别
    let obj: Tagged<Object> = self.cast();
    instance_type_of(obj).is_some_and(itc::is_code)
  }

  #[inline]
  fn get_code(&self) -> Tagged<Code> {
    // 原版: SbxCast<Code>(TrustedCast<TrustedObject>(this)) — 基线非沙箱即裸转换
    self.cast()
  }

  #[inline]
  fn get_bytecode_array(&self) -> Tagged<BytecodeArray> {
    // 原版: SbxCast<BytecodeArray>(TrustedCast<TrustedObject>(this))
    self.cast()
  }

  #[inline]
  fn to_union_type(&self) -> Tagged<AbstractCode> {
    // 原版: TrustedCast<Union<Code, BytecodeArray>>(this) — 联合体即本句柄
    *self
  }

  #[inline]
  fn instruction_size(&self) -> Result<i32> {
    if self.is_code_variant() {
      Ok(self.get_code().instruction_size())
    } else {
      // BytecodeArray 支: length()
      Ok(self.get_bytecode_array().length())
    }
  }

  #[inline]
  fn instruction_start(&self) -> usize {
    if self.is_code_variant() {
      self.get_code().instruction_start()
    } else {
      self.get_bytecode_array().get_first_bytecode_address()
    }
  }

  #[inline]
  fn instruction_end(&self) -> usize {
    if self.is_code_variant() {
      self.get_code().instruction_end()
    } else {
      // 原版: GetFirstBytecodeAddress() + length()
      let ba = self.get_bytecode_array();
      ba.get_first_bytecode_address() + ba.length() as usize
    }
  }

  #[inline]
  fn kind(&self) -> Result<CodeKind> {
    if self.is_code_variant() {
      self.get_code().kind()
    } else {
      Ok(CodeKind::INTERPRETED_FUNCTION)
    }
  }

  #[inline]
  fn builtin_id(&self) -> i16 {
    if self.is_code_variant() {
      self.get_code().builtin_id()
    } else {
      // BytecodeArray 恒 kNoBuiltinId
      K_NO_BUILTIN_ID
    }
  }

  #[inline]
  fn has_instruction_stream(&self) -> bool {
    // 原版 DCHECK(IsCode(map)); BytecodeArray 支无指令流 (恒 false)
    self.is_code_variant() && self.get_code().has_instruction_stream()
  }

  #[inline]
  fn is_context_specialized(&self) -> bool {
    if self.is_code_variant() {
      self.get_code().is_context_specialized()
    } else {
      false
    }
  }

  #[inline]
  fn osr_offset(&self) -> i32 {
    if self.is_code_variant() {
      self.get_code().osr_offset()
    } else {
      // BytecodeArray 恒 BytecodeOffset::None()
      BYTECODE_OFFSET_NONE
    }
  }

  #[inline]
  fn size_including_metadata(&self) -> Result<i32> {
    if self.is_code_variant() {
      self.get_code().size_including_metadata()
    } else {
      self.get_bytecode_array().size_including_metadata()
    }
  }

  #[inline]
  fn contains(&self, inner_pointer: usize) -> Result<bool> {
    if self.is_code_variant() {
      // 原版: GetCode()->contains(isolate, inner_pointer) (依赖 isolate → 台账)
      let _ = inner_pointer;
      todo!("推迟登记(readme/zh.md): AbstractCode::contains Code 支 isolate 归 execution 席")
    } else {
      // 原版: address() <= p && p <= address() + Size()
      let base = self.address();
      let size = self.get_bytecode_array().bytecode_array_size() as usize;
      Ok(base <= inner_pointer && inner_pointer <= base + size)
    }
  }

  #[inline]
  fn source_position(&self, offset: i32) -> Result<i32> {
    // 原版: IsCode ? GetCode()->SourcePosition : GetBytecodeArray()->SourcePosition
    // 两支位置表面 (SourcePositionTableIterator / 字节码位置表) 归 codegen-core 席 → 台账
    let _ = offset;
    todo!("推迟登记(readme/zh.md): AbstractCode::SourcePosition 两支位置表归 codegen-core 席")
  }

  #[inline]
  fn source_statement_position(&self, offset: i32) -> Result<i32> {
    let _ = offset;
    todo!("推迟登记(readme/zh.md): AbstractCode::SourcePosition 两支位置表归 codegen-core 席")
  }
}

/// AbstractCode 分发矩阵 (16 个分发方法, 每个含 Code/BytecodeArray 两支)
///
/// | 方法 | Code 支 | BytecodeArray 支 |
/// |---|---|---|
/// | SourcePosition | 转发 GetCode() | 见原版 |
/// | SourceStatementPosition | 转发 GetCode() | 见原版 |
/// | is_context_specialized | 转发 GetCode() | 见原版 |
/// | osr_offset | 转发 GetCode() | 见原版 |
/// | InstructionSize | 转发 GetCode() | 见原版 |
/// | SourcePositionTable | 转发 GetCode() | 见原版 |
/// | SizeIncludingMetadata | 转发 GetCode() | 见原版 |
/// | InstructionStart | 转发 GetCode() | 见原版 |
/// | InstructionEnd | 转发 GetCode() | 见原版 |
/// | contains | 转发 GetCode() | 见原版 |
/// | kind | 转发 GetCode() | 见原版 |
/// | builtin_id | 转发 GetCode() | 见原版 |
/// | has_instruction_stream | 转发 GetCode() | 见原版 |
/// | GetCode | 转发 GetCode() | 见原版 |
/// | GetBytecodeArray | 转发 GetCode() | 见原版 |
/// | ToUnionType | 转发 GetCode() | 见原版 |
///
/// BytecodeArray 侧固定默认: kind→INTERPRETED_FUNCTION, builtin_id→kNoBuiltinId,
/// osr_offset→BytecodeOffset::None(), is_context_specialized→false,
/// InstructionSize→length().
pub const ABSTRACT_CODE_DISPATCHERS: [&str; 16] = [
  "SourcePosition",
  "SourceStatementPosition",
  "is_context_specialized",
  "osr_offset",
  "InstructionSize",
  "SourcePositionTable",
  "SizeIncludingMetadata",
  "InstructionStart",
  "InstructionEnd",
  "contains",
  "kind",
  "builtin_id",
  "has_instruction_stream",
  "GetCode",
  "GetBytecodeArray",
  "ToUnionType",
];
