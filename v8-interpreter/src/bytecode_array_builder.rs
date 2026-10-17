// 由 tool/src/interpreter/bytecode-array-builder.h.js 从 v8/src/interpreter/bytecode-array-builder.{h,cc} 生成; 修订就地改, 复跑覆盖
// 字节码构建器体系与真实 AST/指令发射逻辑

use std::{
  collections::{HashSet, VecDeque},
  fmt,
  sync::Arc,
};

use v8_ast::{AstBinaryOp, AstCompareOp, AstCountOp, AstUnaryOp};

use crate::{
  bytecode_array_iterator::BytecodeArrayIterator,
  bytecode_array_writer::BytecodeArrayWriter,
  bytecode_label::{BytecodeLabel, BytecodeLoopHeader},
  bytecode_node::BytecodeNode,
  bytecode_operands::OperandScale,
  bytecode_register::Register,
  bytecode_source_info::BytecodeSourceInfo,
  bytecodes::{Bytecode, Bytecodes},
  error::Result,
};

/// ToBoolean 转换模式 (原版 enum class ToBooleanMode)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum ToBooleanMode {
  /// 对累加器执行 ToBoolean 转换
  ConvertToBoolean = 0,
  /// 累加器已是布尔值
  AlreadyBoolean = 1,
}

/// JS 两种 nil (原版 enum NilValue)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum NilValue {
  /// null
  NullValue = 0,
  /// undefined
  UndefinedValue = 1,
}

/// 解释器函数闭包结构 (包含形参、名称与已生成的字节码数组)
#[derive(Clone, PartialEq)]
pub struct InterpFunction {
  pub name: Option<String>,
  pub params: Vec<String>,
  pub captured_locals: Vec<String>,
  pub bytecode_array: BytecodeArray,
  pub uses_arguments: bool,
  /// 箭头函数标记: 调用点换入定格 this、拒绝 new、不挂 prototype
  pub is_arrow: bool,
  /// 函数源码原文切片 (随字面量带过来, Function.prototype.toString 的真相, #23 ⑤)
  pub src: Option<String>,
  /// rest 形参落地 (12.40): (收集起点, 生成器预分配的本帧局部寄存器号)。
  /// 进帧时刻分发器把实参尾段快照写进该寄存器, 之后按普通局部读写
  pub rest_binding: Option<(usize, i32)>,
}

impl fmt::Debug for InterpFunction {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    f.debug_struct("InterpFunction")
      .field("name", &self.name)
      .field("params", &self.params)
      .finish()
  }
}

/// 常量池项
#[derive(Debug, Clone, PartialEq)]
pub enum ConstantPoolEntry {
  Number(f64),
  /// 字符串常量: 与 `Value::String` 同理由 `Arc<str>` 承载 —— 每条 `LdaConstant` 都要克隆一份,
  /// 用 String 就是每载入一次字面量做一次 malloc + memcpy; 常量池去重后同名只会留一份, 克隆
  /// 随之退化成一次引用计数。
  String(Arc<str>),
  BigInt(Arc<str>),
  Function(Arc<InterpFunction>),
}

impl ConstantPoolEntry {
  /// 构造字符串常量 (来源可为 `String` / `&str`)
  #[inline]
  #[must_use]
  pub fn new_string(s: impl Into<Arc<str>>) -> Self {
    Self::String(s.into())
  }

  /// 变体名, 供名字槽位类型不符时如实报告 (#26)
  #[must_use]
  pub const fn kind_name(&self) -> &'static str {
    match self {
      Self::Number(_) => "Number",
      Self::String(_) => "String",
      Self::BigInt(_) => "BigInt",
      Self::Function(_) => "Function",
    }
  }
}

/// 异常表条目 (用于 try-catch 异常分派)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct HandlerTableEntry {
  pub start: usize,
  pub end: usize,
  pub target: usize,
}

/// 解释器字节码数组 (生成与执行载体)
#[derive(Debug, Clone, PartialEq, Default)]
pub struct BytecodeArray {
  pub bytecodes: Vec<u8>,
  pub constants: Vec<ConstantPoolEntry>,
  pub register_count: usize,
  pub parameter_count: usize,
  pub handler_table: Vec<HandlerTableEntry>,
}

impl BytecodeArray {
  #[inline]
  #[must_use]
  pub const fn new(
    bytecodes: Vec<u8>,
    constants: Vec<ConstantPoolEntry>,
    register_count: usize,
    parameter_count: usize,
  ) -> Self {
    Self {
      bytecodes,
      constants,
      register_count,
      parameter_count,
      handler_table: Vec::new(),
    }
  }

  #[inline]
  #[must_use]
  pub const fn with_handler_table(
    bytecodes: Vec<u8>,
    constants: Vec<ConstantPoolEntry>,
    register_count: usize,
    parameter_count: usize,
    handler_table: Vec<HandlerTableEntry>,
  ) -> Self {
    Self {
      bytecodes,
      constants,
      register_count,
      parameter_count,
      handler_table,
    }
  }

  #[inline]
  #[must_use]
  pub fn bytecodes(&self) -> &[u8] {
    &self.bytecodes
  }

  #[inline]
  #[must_use]
  pub fn constants(&self) -> &[ConstantPoolEntry] {
    &self.constants
  }

  #[inline]
  #[must_use]
  pub const fn register_count(&self) -> usize {
    self.register_count
  }

  #[inline]
  #[must_use]
  pub const fn parameter_count(&self) -> usize {
    self.parameter_count
  }

  #[inline]
  #[must_use]
  pub fn handler_table(&self) -> &[HandlerTableEntry] {
    &self.handler_table
  }

  /// 在异常表中寻找当前偏移处最内层的 catch 目标
  #[inline]
  #[must_use]
  pub fn find_catch_handler(&self, offset: usize) -> Option<usize> {
    self
      .handler_table
      .iter()
      .filter(|e| e.start <= offset && offset < e.end)
      .min_by_key(|e| e.end.saturating_sub(e.start))
      .map(|e| e.target)
  }
}

/// 字节码构建器 (承接上层 AST 生成器调用，发射字节码与管理寄存器/常量)
#[derive(Debug, Default, Clone)]
pub struct BytecodeArrayBuilder {
  writer: BytecodeArrayWriter,
  constants: Vec<ConstantPoolEntry>,
  parameter_count: usize,
  local_register_count: usize,
  next_register_index: i32,
  max_register_count: usize,
  handler_table: Vec<HandlerTableEntry>,
}

impl BytecodeArrayBuilder {
  #[inline]
  #[must_use]
  pub fn new(parameter_count: usize, locals_count: usize) -> Self {
    Self {
      writer: BytecodeArrayWriter::new(),
      constants: Vec::new(),
      parameter_count,
      local_register_count: locals_count,
      next_register_index: locals_count as i32,
      max_register_count: locals_count,
      handler_table: Vec::new(),
    }
  }

  #[inline]
  #[must_use]
  pub const fn parameter_count(&self) -> usize {
    self.parameter_count
  }

  #[inline]
  #[must_use]
  pub fn handler_table(&self) -> &[HandlerTableEntry] {
    &self.handler_table
  }

  #[inline]
  #[must_use]
  pub const fn local_register_count(&self) -> usize {
    self.local_register_count
  }

  #[inline]
  #[must_use]
  pub const fn total_register_count(&self) -> usize {
    self.max_register_count
  }

  /// 分配一个虚拟寄存器
  pub fn allocate_register(&mut self) -> Register {
    let reg = Register::new(self.next_register_index);
    self.next_register_index += 1;
    if (self.next_register_index as usize) > self.max_register_count {
      self.max_register_count = self.next_register_index as usize;
    }
    reg
  }

  /// 释放寄存器 (回退栈顶寄存器)
  pub fn free_register(&mut self, reg: Register) {
    if reg.index() + 1 == self.next_register_index
      && self.next_register_index > (self.local_register_count as i32)
    {
      self.next_register_index -= 1;
    }
  }

  /// 登记或获取常量池索引
  pub fn get_or_insert_constant(&mut self, entry: ConstantPoolEntry) -> usize {
    if let Some(pos) = self.constants.iter().position(|c| *c == entry) {
      pos
    } else {
      self.constants.push(entry);
      self.constants.len() - 1
    }
  }

  /// 登记或获取字符串常量池索引 (无堆分配借用查找)
  pub fn get_or_insert_string(&mut self, s: &str) -> usize {
    if let Some(pos) = self.constants.iter().position(|c| match c {
      ConstantPoolEntry::String(entry) => &**entry == s,
      _ => false,
    }) {
      pos
    } else {
      self.constants.push(ConstantPoolEntry::new_string(s));
      self.constants.len() - 1
    }
  }

  /// 写入字节码节点
  #[inline]
  pub fn emit(&mut self, node: BytecodeNode) {
    self.writer.write(&node);
  }

  /// 加载 Smi/整数立即数到累加器
  pub fn load_literal_smi(&mut self, value: i32) -> Result<()> {
    if value == 0 {
      self.emit(BytecodeNode::new(
        Bytecode::LdaZero,
        BytecodeSourceInfo::none(),
      )?);
    } else {
      self.emit(BytecodeNode::with_operands(
        Bytecode::LdaSmi,
        &[value as u32],
        BytecodeSourceInfo::none(),
      )?);
    }
    Ok(())
  }

  /// 加载 undefined 到累加器
  pub fn load_undefined(&mut self) -> Result<()> {
    self.emit(BytecodeNode::new(
      Bytecode::LdaUndefined,
      BytecodeSourceInfo::none(),
    )?);
    Ok(())
  }

  /// 加载 null 到累加器
  pub fn load_null(&mut self) -> Result<()> {
    self.emit(BytecodeNode::new(
      Bytecode::LdaNull,
      BytecodeSourceInfo::none(),
    )?);
    Ok(())
  }

  /// 加载 the_hole 到累加器
  pub fn load_the_hole(&mut self) -> Result<()> {
    self.emit(BytecodeNode::new(
      Bytecode::LdaTheHole,
      BytecodeSourceInfo::none(),
    )?);
    Ok(())
  }

  /// 加载 true 到累加器
  pub fn load_true(&mut self) -> Result<()> {
    self.emit(BytecodeNode::new(
      Bytecode::LdaTrue,
      BytecodeSourceInfo::none(),
    )?);
    Ok(())
  }

  /// 加载 false 到累加器
  pub fn load_false(&mut self) -> Result<()> {
    self.emit(BytecodeNode::new(
      Bytecode::LdaFalse,
      BytecodeSourceInfo::none(),
    )?);
    Ok(())
  }

  /// 加载布尔值
  pub fn load_boolean(&mut self, value: bool) -> Result<()> {
    if value {
      self.load_true()
    } else {
      self.load_false()
    }
  }

  /// 加载常量池项
  pub fn load_constant(&mut self, entry: ConstantPoolEntry) -> Result<()> {
    let idx = self.get_or_insert_constant(entry);
    self.emit(BytecodeNode::with_operands(
      Bytecode::LdaConstant,
      &[idx as u32],
      BytecodeSourceInfo::none(),
    )?);
    Ok(())
  }

  /// 加载全局变量 (LdaGlobal)
  pub fn load_global(&mut self, name: impl AsRef<str>, feedback_slot: usize) -> Result<()> {
    let idx = self.get_or_insert_string(name.as_ref());
    self.emit(BytecodeNode::with_operands(
      Bytecode::LdaGlobal,
      &[idx as u32, feedback_slot as u32],
      BytecodeSourceInfo::none(),
    )?);
    Ok(())
  }

  /// 在 typeof 上下文内加载全局变量 (LdaGlobalInsideTypeof)
  ///
  /// 原版单列此 opcode: `typeof 未声明名` 必须返回 "undefined" 而不是抛 ReferenceError
  pub fn load_global_inside_typeof(
    &mut self,
    name: impl AsRef<str>,
    feedback_slot: usize,
  ) -> Result<()> {
    let idx = self.get_or_insert_string(name.as_ref());
    self.emit(BytecodeNode::with_operands(
      Bytecode::LdaGlobalInsideTypeof,
      &[idx as u32, feedback_slot as u32],
      BytecodeSourceInfo::none(),
    )?);
    Ok(())
  }

  /// 存储全局变量 (StaGlobal)
  pub fn store_global(&mut self, name: impl AsRef<str>, feedback_slot: usize) -> Result<()> {
    let idx = self.get_or_insert_string(name.as_ref());
    self.emit(BytecodeNode::with_operands(
      Bytecode::StaGlobal,
      &[idx as u32, feedback_slot as u32],
      BytecodeSourceInfo::none(),
    )?);
    Ok(())
  }

  /// 加载寄存器到累加器 (Ldar)
  pub fn load_accumulator_with_register(&mut self, reg: Register) -> Result<()> {
    self.emit(BytecodeNode::with_operands(
      Bytecode::Ldar,
      &[reg.to_operand() as u32],
      BytecodeSourceInfo::none(),
    )?);
    Ok(())
  }

  /// 存储累加器到寄存器 (Star / ShortStar)
  pub fn store_accumulator_in_register(&mut self, reg: Register) -> Result<()> {
    if let Some(short_star) = reg.try_to_short_star() {
      self.emit(BytecodeNode::new(short_star, BytecodeSourceInfo::none())?);
    } else {
      self.emit(BytecodeNode::with_operands(
        Bytecode::Star,
        &[reg.to_operand() as u32],
        BytecodeSourceInfo::none(),
      )?);
    }
    Ok(())
  }

  /// 加载命名属性 (GetNamedProperty)
  pub fn load_named_property(
    &mut self,
    object: Register,
    name: &str,
    feedback_slot: usize,
  ) -> Result<()> {
    let idx = self.get_or_insert_string(name);
    self.emit(BytecodeNode::with_operands(
      Bytecode::GetNamedProperty,
      &[object.to_operand() as u32, idx as u32, feedback_slot as u32],
      BytecodeSourceInfo::none(),
    )?);
    Ok(())
  }

  /// 存储命名属性 (SetNamedProperty)
  pub fn set_named_property(
    &mut self,
    object: Register,
    name: &str,
    feedback_slot: usize,
  ) -> Result<()> {
    let idx = self.get_or_insert_string(name);
    self.emit(BytecodeNode::with_operands(
      Bytecode::SetNamedProperty,
      &[object.to_operand() as u32, idx as u32, feedback_slot as u32],
      BytecodeSourceInfo::none(),
    )?);
    Ok(())
  }

  /// 字面量构建期定义自有命名属性 (DefineNamedOwnPropertyInLiteral)
  ///
  /// 与 `set_named_property` 的分工照原版: 对象字面量的落表走 **CreateDataProperty**, 不打
  /// setter, 同名后定义覆盖先前定义 (含 accessor)
  pub fn define_named_property_in_literal(
    &mut self,
    object: Register,
    name: &str,
    feedback_slot: usize,
  ) -> Result<()> {
    let idx = self.get_or_insert_string(name);
    self.emit(BytecodeNode::with_operands(
      Bytecode::DefineNamedOwnPropertyInLiteral,
      &[object.to_operand() as u32, idx as u32, feedback_slot as u32],
      BytecodeSourceInfo::none(),
    )?);
    Ok(())
  }

  /// 定义自有键控属性 (DefineKeyedOwnProperty, 原版操作数 Reg+Reg+Flag8+FeedbackSlot)
  ///
  /// 字面量的计算键走这条而不是 `set_keyed_property`: 同样是 CreateDataProperty 语义。
  /// Flag8 目前只用到 kNoFlags (原版的 kSetFunctionName 服务于方法命名, 本侧由 lower 完成)
  pub fn define_keyed_property(
    &mut self,
    object: Register,
    key: Register,
    feedback_slot: usize,
  ) -> Result<()> {
    self.emit(BytecodeNode::with_operands(
      Bytecode::DefineKeyedOwnProperty,
      &[
        object.to_operand() as u32,
        key.to_operand() as u32,
        0,
        feedback_slot as u32,
      ],
      BytecodeSourceInfo::none(),
    )?);
    Ok(())
  }

  /// 加载键控属性 (GetKeyedProperty)
  pub fn load_keyed_property(&mut self, object: Register, feedback_slot: usize) -> Result<()> {
    self.emit(BytecodeNode::with_operands(
      Bytecode::GetKeyedProperty,
      &[object.to_operand() as u32, feedback_slot as u32],
      BytecodeSourceInfo::none(),
    )?);
    Ok(())
  }

  /// 存储键控属性 (SetKeyedProperty)
  pub fn set_keyed_property(
    &mut self,
    object: Register,
    key: Register,
    feedback_slot: usize,
  ) -> Result<()> {
    self.emit(BytecodeNode::with_operands(
      Bytecode::SetKeyedProperty,
      &[
        object.to_operand() as u32,
        key.to_operand() as u32,
        feedback_slot as u32,
      ],
      BytecodeSourceInfo::none(),
    )?);
    Ok(())
  }

  /// 存储数组字面量元素 (StaInArrayLiteral)
  pub fn store_in_array_literal(
    &mut self,
    array: Register,
    index: Register,
    feedback_slot: usize,
  ) -> Result<()> {
    self.emit(BytecodeNode::with_operands(
      Bytecode::StaInArrayLiteral,
      &[
        array.to_operand() as u32,
        index.to_operand() as u32,
        feedback_slot as u32,
      ],
      BytecodeSourceInfo::none(),
    )?);
    Ok(())
  }

  /// 调用属性方法 (CallProperty / CallProperty0..2)
  pub fn call_property(
    &mut self,
    callable: Register,
    receiver: Register,
    args: &[Register],
    feedback_slot: usize,
  ) -> Result<()> {
    let node = match args.len() {
      0 => BytecodeNode::with_operands(
        Bytecode::CallProperty0,
        &[
          callable.to_operand() as u32,
          receiver.to_operand() as u32,
          feedback_slot as u32,
        ],
        BytecodeSourceInfo::none(),
      )?,
      1 => BytecodeNode::with_operands(
        Bytecode::CallProperty1,
        &[
          callable.to_operand() as u32,
          receiver.to_operand() as u32,
          args[0].to_operand() as u32,
          feedback_slot as u32,
        ],
        BytecodeSourceInfo::none(),
      )?,
      2 => BytecodeNode::with_operands(
        Bytecode::CallProperty2,
        &[
          callable.to_operand() as u32,
          receiver.to_operand() as u32,
          args[0].to_operand() as u32,
          args[1].to_operand() as u32,
          feedback_slot as u32,
        ],
        BytecodeSourceInfo::none(),
      )?,
      count => BytecodeNode::with_operands(
        Bytecode::CallProperty,
        &[
          callable.to_operand() as u32,
          receiver.to_operand() as u32,
          (count + 1) as u32,
          feedback_slot as u32,
        ],
        BytecodeSourceInfo::none(),
      )?,
    };
    self.emit(node);
    Ok(())
  }

  /// 调用未指定接收者函数 (CallUndefinedReceiver / CallUndefinedReceiver0..2)
  pub fn call_undefined_receiver(
    &mut self,
    callable: Register,
    args: &[Register],
    feedback_slot: usize,
  ) -> Result<()> {
    let node = match args.len() {
      0 => BytecodeNode::with_operands(
        Bytecode::CallUndefinedReceiver0,
        &[callable.to_operand() as u32, feedback_slot as u32],
        BytecodeSourceInfo::none(),
      )?,
      1 => BytecodeNode::with_operands(
        Bytecode::CallUndefinedReceiver1,
        &[
          callable.to_operand() as u32,
          args[0].to_operand() as u32,
          feedback_slot as u32,
        ],
        BytecodeSourceInfo::none(),
      )?,
      2 => BytecodeNode::with_operands(
        Bytecode::CallUndefinedReceiver2,
        &[
          callable.to_operand() as u32,
          args[0].to_operand() as u32,
          args[1].to_operand() as u32,
          feedback_slot as u32,
        ],
        BytecodeSourceInfo::none(),
      )?,
      count => BytecodeNode::with_operands(
        Bytecode::CallUndefinedReceiver,
        &[
          callable.to_operand() as u32,
          args[0].to_operand() as u32,
          count as u32,
          feedback_slot as u32,
        ],
        BytecodeSourceInfo::none(),
      )?,
    };
    self.emit(node);
    Ok(())
  }

  /// 构造函数调用 (Construct)
  pub fn construct(
    &mut self,
    constructor: Register,
    args: &[Register],
    feedback_slot: usize,
  ) -> Result<()> {
    let first_arg = if args.is_empty() {
      0
    } else {
      args[0].to_operand() as u32
    };
    self.emit(BytecodeNode::with_operands(
      Bytecode::Construct,
      &[
        constructor.to_operand() as u32,
        first_arg,
        args.len() as u32,
        feedback_slot as u32,
      ],
      BytecodeSourceInfo::none(),
    )?);
    Ok(())
  }

  /// 创建空数组字面量 (CreateEmptyArrayLiteral)
  pub fn create_empty_array_literal(&mut self, feedback_slot: usize) -> Result<()> {
    self.emit(BytecodeNode::with_operands(
      Bytecode::CreateEmptyArrayLiteral,
      &[feedback_slot as u32],
      BytecodeSourceInfo::none(),
    )?);
    Ok(())
  }

  /// 创建数组字面量 (CreateArrayLiteral)
  pub fn create_array_literal(
    &mut self,
    constant_entry: usize,
    feedback_slot: usize,
    flags: u8,
  ) -> Result<()> {
    self.emit(BytecodeNode::with_operands(
      Bytecode::CreateArrayLiteral,
      &[constant_entry as u32, feedback_slot as u32, flags as u32],
      BytecodeSourceInfo::none(),
    )?);
    Ok(())
  }

  /// 创建空对象字面量 (CreateEmptyObjectLiteral)
  pub fn create_empty_object_literal(&mut self) -> Result<()> {
    self.emit(BytecodeNode::new(
      Bytecode::CreateEmptyObjectLiteral,
      BytecodeSourceInfo::none(),
    )?);
    Ok(())
  }

  /// 创建对象字面量 (CreateObjectLiteral)
  pub fn create_object_literal(
    &mut self,
    constant_entry: usize,
    feedback_slot: usize,
    flags: u8,
  ) -> Result<()> {
    self.emit(BytecodeNode::with_operands(
      Bytecode::CreateObjectLiteral,
      &[constant_entry as u32, feedback_slot as u32, flags as u32],
      BytecodeSourceInfo::none(),
    )?);
    Ok(())
  }

  /// 二元运算 (Acc = Reg OP Acc)
  pub fn binary_operation(
    &mut self,
    op: v8_ast::AstBinaryOp,
    reg: Register,
    feedback_slot: u8,
  ) -> Result<()> {
    let bc = match op {
      AstBinaryOp::Add => Bytecode::Add,
      AstBinaryOp::Sub => Bytecode::Sub,
      AstBinaryOp::Mul => Bytecode::Mul,
      AstBinaryOp::Div => Bytecode::Div,
      AstBinaryOp::Mod => Bytecode::Mod,
      AstBinaryOp::Exp => Bytecode::Exp,
      AstBinaryOp::BitOr => Bytecode::BitwiseOr,
      AstBinaryOp::BitAnd => Bytecode::BitwiseAnd,
      AstBinaryOp::BitXor => Bytecode::BitwiseXor,
      AstBinaryOp::Shl => Bytecode::ShiftLeft,
      AstBinaryOp::Sar => Bytecode::ShiftRight,
      AstBinaryOp::Shr => Bytecode::ShiftRightLogical,
      _ => Bytecode::Add,
    };
    self.emit(BytecodeNode::with_operands(
      bc,
      &[reg.to_operand() as u32, feedback_slot as u32],
      BytecodeSourceInfo::none(),
    )?);
    Ok(())
  }

  /// 一元运算
  pub fn unary_operation(&mut self, op: v8_ast::AstUnaryOp, feedback_slot: u8) -> Result<()> {
    match op {
      AstUnaryOp::Not => self.emit(BytecodeNode::new(
        Bytecode::LogicalNot,
        BytecodeSourceInfo::none(),
      )?),
      AstUnaryOp::Sub => self.emit(BytecodeNode::with_operands(
        Bytecode::Negate,
        &[feedback_slot as u32],
        BytecodeSourceInfo::none(),
      )?),
      AstUnaryOp::BitNot => self.emit(BytecodeNode::with_operands(
        Bytecode::BitwiseNot,
        &[feedback_slot as u32],
        BytecodeSourceInfo::none(),
      )?),
      AstUnaryOp::TypeOf => self.emit(BytecodeNode::with_operands(
        Bytecode::TypeOf,
        &[feedback_slot as u32],
        BytecodeSourceInfo::none(),
      )?),
      AstUnaryOp::Void => self.load_undefined()?,
      AstUnaryOp::Add => self.emit(BytecodeNode::with_operands(
        Bytecode::ToNumber,
        &[feedback_slot as u32],
        BytecodeSourceInfo::none(),
      )?),
      AstUnaryOp::Delete => self.load_false()?,
    }
    Ok(())
  }

  /// 自增自减
  pub fn count_operation(&mut self, op: v8_ast::AstCountOp, feedback_slot: u8) -> Result<()> {
    let bc = match op {
      AstCountOp::Inc => Bytecode::Inc,
      AstCountOp::Dec => Bytecode::Dec,
    };
    self.emit(BytecodeNode::with_operands(
      bc,
      &[feedback_slot as u32],
      BytecodeSourceInfo::none(),
    )?);
    Ok(())
  }

  /// 比较运算
  pub fn compare_operation(
    &mut self,
    op: v8_ast::AstCompareOp,
    reg: Register,
    feedback_slot: u8,
  ) -> Result<()> {
    match op {
      AstCompareOp::Eq => self.emit(BytecodeNode::with_operands(
        Bytecode::TestEqual,
        &[reg.to_operand() as u32, feedback_slot as u32],
        BytecodeSourceInfo::none(),
      )?),
      AstCompareOp::EqStrict => self.emit(BytecodeNode::with_operands(
        Bytecode::TestEqualStrict,
        &[reg.to_operand() as u32, feedback_slot as u32],
        BytecodeSourceInfo::none(),
      )?),
      AstCompareOp::NotEq => {
        self.emit(BytecodeNode::with_operands(
          Bytecode::TestEqual,
          &[reg.to_operand() as u32, feedback_slot as u32],
          BytecodeSourceInfo::none(),
        )?);
        self.emit(BytecodeNode::new(
          Bytecode::LogicalNot,
          BytecodeSourceInfo::none(),
        )?);
      }
      AstCompareOp::NotEqStrict => {
        self.emit(BytecodeNode::with_operands(
          Bytecode::TestEqualStrict,
          &[reg.to_operand() as u32, feedback_slot as u32],
          BytecodeSourceInfo::none(),
        )?);
        self.emit(BytecodeNode::new(
          Bytecode::LogicalNot,
          BytecodeSourceInfo::none(),
        )?);
      }
      AstCompareOp::LessThan => self.emit(BytecodeNode::with_operands(
        Bytecode::TestLessThan,
        &[reg.to_operand() as u32, feedback_slot as u32],
        BytecodeSourceInfo::none(),
      )?),
      AstCompareOp::GreaterThan => self.emit(BytecodeNode::with_operands(
        Bytecode::TestGreaterThan,
        &[reg.to_operand() as u32, feedback_slot as u32],
        BytecodeSourceInfo::none(),
      )?),
      AstCompareOp::LessThanEq => self.emit(BytecodeNode::with_operands(
        Bytecode::TestLessThanOrEqual,
        &[reg.to_operand() as u32, feedback_slot as u32],
        BytecodeSourceInfo::none(),
      )?),
      AstCompareOp::GreaterThanEq => self.emit(BytecodeNode::with_operands(
        Bytecode::TestGreaterThanOrEqual,
        &[reg.to_operand() as u32, feedback_slot as u32],
        BytecodeSourceInfo::none(),
      )?),
      AstCompareOp::InstanceOf => self.emit(BytecodeNode::with_operands(
        Bytecode::TestInstanceOf,
        &[reg.to_operand() as u32, feedback_slot as u32],
        BytecodeSourceInfo::none(),
      )?),
      AstCompareOp::In => self.emit(BytecodeNode::with_operands(
        Bytecode::TestIn,
        &[reg.to_operand() as u32, feedback_slot as u32],
        BytecodeSourceInfo::none(),
      )?),
    }
    Ok(())
  }

  /// 创建新跳转标签
  #[inline]
  pub fn new_label(&mut self) -> BytecodeLabel {
    self.writer.new_label()
  }

  /// 无条件跳转
  pub fn jump(&mut self, label: BytecodeLabel) -> Result<()> {
    let node = BytecodeNode::with_operands_and_scale(
      Bytecode::Jump,
      &[0],
      OperandScale::Quadruple,
      BytecodeSourceInfo::none(),
    )?;
    self.writer.write_jump(&node, label);
    Ok(())
  }

  /// 真值跳转
  pub fn jump_if_true(&mut self, mode: ToBooleanMode, label: BytecodeLabel) -> Result<()> {
    let bc = match mode {
      ToBooleanMode::AlreadyBoolean => Bytecode::JumpIfTrue,
      ToBooleanMode::ConvertToBoolean => Bytecode::JumpIfToBooleanTrue,
    };
    let node = BytecodeNode::with_operands_and_scale(
      bc,
      &[0],
      OperandScale::Quadruple,
      BytecodeSourceInfo::none(),
    )?;
    self.writer.write_jump(&node, label);
    Ok(())
  }

  /// 假值跳转
  pub fn jump_if_false(&mut self, mode: ToBooleanMode, label: BytecodeLabel) -> Result<()> {
    let bc = match mode {
      ToBooleanMode::AlreadyBoolean => Bytecode::JumpIfFalse,
      ToBooleanMode::ConvertToBoolean => Bytecode::JumpIfToBooleanFalse,
    };
    let node = BytecodeNode::with_operands_and_scale(
      bc,
      &[0],
      OperandScale::Quadruple,
      BytecodeSourceInfo::none(),
    )?;
    self.writer.write_jump(&node, label);
    Ok(())
  }

  /// null 跳转
  pub fn jump_if_null(&mut self, label: BytecodeLabel) -> Result<()> {
    let node = BytecodeNode::with_operands_and_scale(
      Bytecode::JumpIfNull,
      &[0],
      OperandScale::Quadruple,
      BytecodeSourceInfo::none(),
    )?;
    self.writer.write_jump(&node, label);
    Ok(())
  }

  /// undefined 跳转
  pub fn jump_if_undefined(&mut self, label: BytecodeLabel) -> Result<()> {
    let node = BytecodeNode::with_operands_and_scale(
      Bytecode::JumpIfUndefined,
      &[0],
      OperandScale::Quadruple,
      BytecodeSourceInfo::none(),
    )?;
    self.writer.write_jump(&node, label);
    Ok(())
  }

  /// 循环回跳 (JumpLoop)
  pub fn jump_loop(
    &mut self,
    header: &BytecodeLoopHeader,
    loop_depth: i32,
    feedback_slot: u32,
  ) -> Result<()> {
    let cur = self.writer.current_bytecode_size();
    let mut delta = cur.saturating_sub(header.offset());
    let probe_node = BytecodeNode::with_operands(
      Bytecode::JumpLoop,
      &[delta as u32, loop_depth as u32, feedback_slot],
      BytecodeSourceInfo::none(),
    )?;
    if probe_node.operand_scale() != OperandScale::Single {
      delta += 1;
    }
    let node = BytecodeNode::with_operands(
      Bytecode::JumpLoop,
      &[delta as u32, loop_depth as u32, feedback_slot],
      BytecodeSourceInfo::none(),
    )?;
    self.writer.write_jump_loop(&node, header);
    Ok(())
  }

  /// 绑定前向跳转标签
  pub fn bind(&mut self, label: BytecodeLabel) {
    self.writer.bind_label(label);
  }

  /// 绑定循环头
  pub fn bind_loop_header(&mut self, header: &mut BytecodeLoopHeader) {
    self.writer.bind_loop_header(header);
  }

  /// 函数返回 (Return)
  pub fn return_op(&mut self) -> Result<()> {
    self.emit(BytecodeNode::new(
      Bytecode::Return,
      BytecodeSourceInfo::none(),
    )?);
    Ok(())
  }

  /// 抛出异常 (Throw)
  pub fn throw_op(&mut self) -> Result<()> {
    self.emit(BytecodeNode::new(
      Bytecode::Throw,
      BytecodeSourceInfo::none(),
    )?);
    Ok(())
  }

  /// 获取类型字符串 (TypeOf)
  pub fn type_of(&mut self, feedback_slot: u8) -> Result<()> {
    self.emit(BytecodeNode::with_operands(
      Bytecode::TypeOf,
      &[feedback_slot as u32],
      BytecodeSourceInfo::none(),
    )?);
    Ok(())
  }

  /// 收集对象可枚举键 (ForInEnumerate)
  pub fn for_in_enumerate(&mut self, receiver: Register) -> Result<()> {
    self.emit(BytecodeNode::with_operands(
      Bytecode::ForInEnumerate,
      &[receiver.to_operand() as u32],
      BytecodeSourceInfo::none(),
    )?);
    Ok(())
  }

  /// 准备 ForIn 枚举状态 (ForInPrepare)
  pub fn for_in_prepare(
    &mut self,
    cache_info_triple: Register,
    feedback_slot: usize,
  ) -> Result<()> {
    self.emit(BytecodeNode::with_operands(
      Bytecode::ForInPrepare,
      &[cache_info_triple.to_operand() as u32, feedback_slot as u32],
      BytecodeSourceInfo::none(),
    )?);
    Ok(())
  }

  /// 获取下一个枚举键 (ForInNext)
  pub fn for_in_next(
    &mut self,
    receiver: Register,
    index: Register,
    cache_info_pair: Register,
    feedback_slot: usize,
  ) -> Result<()> {
    self.emit(BytecodeNode::with_operands(
      Bytecode::ForInNext,
      &[
        receiver.to_operand() as u32,
        index.to_operand() as u32,
        cache_info_pair.to_operand() as u32,
        feedback_slot as u32,
      ],
      BytecodeSourceInfo::none(),
    )?);
    Ok(())
  }

  /// 递增 ForIn 循环计数器 (ForInStep)
  pub fn for_in_step(&mut self, index: Register) -> Result<()> {
    self.emit(BytecodeNode::with_operands(
      Bytecode::ForInStep,
      &[index.to_operand() as u32],
      BytecodeSourceInfo::none(),
    )?);
    Ok(())
  }

  /// 检查 ForIn 是否完成 (JumpIfForInDone)
  pub fn jump_if_for_in_done(
    &mut self,
    index: Register,
    cache_length: Register,
    label: BytecodeLabel,
  ) -> Result<()> {
    let node = BytecodeNode::with_operands_and_scale(
      Bytecode::JumpIfForInDone,
      &[
        0,
        index.to_operand() as u32,
        cache_length.to_operand() as u32,
      ],
      OperandScale::Quadruple,
      BytecodeSourceInfo::none(),
    )?;
    self.writer.write_jump(&node, label);
    Ok(())
  }

  /// 属性删除 (DeletePropertySloppy)
  pub fn delete_property_sloppy(&mut self, obj: Register) -> Result<()> {
    self.emit(BytecodeNode::with_operands(
      Bytecode::DeletePropertySloppy,
      &[obj.to_operand() as u32],
      BytecodeSourceInfo::none(),
    )?);
    Ok(())
  }

  /// 标记异常处理器区间与跳转目标
  pub fn mark_handler_entry(&mut self, start: usize, end: usize, target: usize) {
    self
      .handler_table
      .push(HandlerTableEntry { start, end, target });
  }

  /// 获取当前字节码偏移
  #[inline]
  #[must_use]
  pub fn current_bytecode_offset(&self) -> usize {
    self.writer.current_bytecode_size()
  }

  /// 构建产出 BytecodeArray
  pub fn build(self) -> BytecodeArray {
    let mut bytecodes = self.writer.into_bytecodes();
    reduce_register_moves(&mut bytecodes, &self.handler_table);
    BytecodeArray::with_handler_table(
      bytecodes,
      self.constants,
      self.max_register_count,
      self.parameter_count,
      self.handler_table,
    )
  }
}

/// 寄存器搬移窥孔 (12.45, 对标原版字节码优化器 ReduceRegisterMoves 的思路):
/// 把相邻 `Ldar rX; ShortStar rY` 就地折成一条 `Mov rX, rY`。
/// 折成立的条件缺一不可:
/// - Ldar 是单刻度 (2 字节) + ShortStar (1 字节) —— 与窄刻度 Mov (3 字节) 等长, 全部
///   字节偏移原地不变, 相对跳转操作数与 handler 区间无需重定位;
/// - 对的中间边界 (Star 落点) 不在任何跳转目标 / handler 边界里 —— 否则有执行流会从
///   被折指令的中部进入;
/// - Star 之后累加器已死 —— 向后近似遍历后继控制流, 任何先**读** acc 的指令都拦折,
///   **覆写** acc 的指令终止该路径; 走到数组尾 (run_from 以 acc 作返回值) 拦折。
///
/// 收益: oct4_all 直方图实测 Ldar+ShortStar* 是全部执行单元的头名 (Ldar 24.7%),
/// 每折一对省一次分发与一对累加器 clone/drop (值从寄存器直搬寄存器, 不再绕累加器)。
pub fn reduce_register_moves(bytecodes: &mut [u8], handler_table: &[HandlerTableEntry]) {
  if bytecodes.len() < 4 {
    return;
  }
  #[derive(Clone, Copy)]
  struct Instr {
    off: usize,
    bc: Bytecode,
    scale: OperandScale,
    jump_target: Option<usize>,
  }
  let mut instrs: Vec<Instr> = Vec::with_capacity(bytecodes.len() / 2);
  let mut boundaries: HashSet<usize> = HashSet::new();
  let mut it = BytecodeArrayIterator::new(bytecodes);
  while !it.done() {
    let bc = it.current_bytecode();
    let mut jump_target = None;
    if Bytecodes::is_jump(bc) || Bytecodes::is_switch(bc) {
      if let Ok(t) = it.get_jump_target_offset() {
        jump_target = Some(t);
        boundaries.insert(t);
      } else {
        // 解码不出目标 (畸形表) → 保守: 整个函数不折
        return;
      }
    }
    instrs.push(Instr {
      off: it.current_offset(),
      bc,
      scale: it.current_operand_scale(),
      jump_target,
    });
    it.advance();
  }
  for h in handler_table {
    boundaries.insert(h.start);
    boundaries.insert(h.end);
    boundaries.insert(h.target);
  }
  let target_index = |off: usize| instrs.binary_search_by(|p| p.off.cmp(&off)).ok();
  // Star 之后 acc 是否不可观测: 后继图上找不到先读 acc 的路径即为死
  let acc_dead_after = |start: usize| -> bool {
    let mut q: VecDeque<usize> = VecDeque::new();
    let mut seen = vec![false; instrs.len()];
    q.push_back(start);
    let mut budget = 64usize;
    while let Some(i) = q.pop_front() {
      if i >= instrs.len() {
        // 走到流尾: run_from 正常完成时以累加器作返回值, acc 仍可观测
        return false;
      }
      if seen[i] {
        continue;
      }
      seen[i] = true;
      if budget == 0 {
        return false;
      }
      budget -= 1;
      let b = instrs[i].bc;
      if Bytecodes::reads_accumulator(b) {
        return false;
      }
      if Bytecodes::writes_or_clobbers_accumulator(b) {
        continue;
      }
      if Bytecodes::is_jump(b) || Bytecodes::is_switch(b) {
        match instrs[i].jump_target.and_then(target_index) {
          Some(ti) => q.push_back(ti),
          None => return false,
        }
        if !Bytecodes::is_unconditional_jump(b) {
          q.push_back(i + 1);
        }
        continue;
      }
      q.push_back(i + 1);
    }
    true
  };
  let mut folds: Vec<(usize, u8)> = Vec::new();
  let mut i = 0usize;
  while i + 1 < instrs.len() {
    let k = instrs[i];
    let next = instrs[i + 1];
    if k.bc == Bytecode::Ldar
      && k.scale == OperandScale::Single
      && Bytecodes::is_short_star(next.bc)
      && next.off == k.off + 2
      && !boundaries.contains(&next.off)
      && !boundaries.contains(&(k.off + 1))
      && acc_dead_after(i + 2)
    {
      // dst 操作数按 `Register::to_operand` 的位移编码存字节 (窄刻度下为带符号单字节),
      // 与 MoveRegister 发射路径同轨; 直接存 index 会被 from_operand 解成参数寄存器
      folds.push((k.off, Register::from_short_star(next.bc).to_operand() as u8));
      i += 2;
      continue;
    }
    i += 1;
  }
  for (off, dst) in folds.iter() {
    bytecodes[*off] = Bytecode::Mov as u8;
    bytecodes[*off + 2] = *dst;
  }
}

/// 原版 class/struct BytecodeArrayBuilder 成员声明 (机械 1:1 提取, 221 条)
pub const BYTECODE_ARRAY_BUILDER_DECLARATIONS: &[&str] = &[
  "[public/method] BytecodeArrayBuilder( Zone* zone, int parameter_count, int locals_count, FeedbackVectorSpec* feedback_vector_spec = nullptr, SourcePositionTableBuilder::RecordingMode source_position_mode = SourcePositionTableBuilder::RECORD_SOURCE_POSITIONS)",
  "[public/method] BytecodeArrayBuilder(const BytecodeArrayBuilder&) = delete",
  "[public/method] BytecodeArrayBuilder& operator=(const BytecodeArrayBuilder&) = delete",
  "[public/method] int CheckBytecodeMatches(Handle<BytecodeArray> bytecode)",
  "[public/method] uint16_t parameter_count() const",
  "[public/method] uint16_t max_arguments() const",
  "[public/method] void UpdateMaxArguments(int max_arguments)",
  "[public/method] int locals_count() const",
  "[public/method] int potentially_throwing_bytecode_count() const",
  "[public/method] int fixed_register_count() const",
  "[public/method] int total_register_count() const",
  "[public/method] Register Local(int index) const",
  "[public/method] Register Parameter(int parameter_index) const",
  "[public/method] Register Receiver() const",
  "[public/method] BytecodeArrayBuilder& LoadConstantPoolEntry(size_t entry)",
  "[public/method] BytecodeArrayBuilder& LoadLiteral(Tagged<Smi> value)",
  "[public/method] BytecodeArrayBuilder& LoadLiteral(double value)",
  "[public/method] BytecodeArrayBuilder& LoadLiteral(const AstRawString* raw_string)",
  "[public/method] BytecodeArrayBuilder& LoadLiteral(const AstConsString* cons_string)",
  "[public/method] BytecodeArrayBuilder& LoadLiteral(const Scope* scope)",
  "[public/method] BytecodeArrayBuilder& LoadLiteral(AstBigInt bigint)",
  "[public/method] BytecodeArrayBuilder& LoadUndefined()",
  "[public/method] BytecodeArrayBuilder& LoadNull()",
  "[public/method] BytecodeArrayBuilder& LoadTheHole()",
  "[public/method] BytecodeArrayBuilder& LoadTdzHole()",
  "[public/method] BytecodeArrayBuilder& LoadTrue()",
  "[public/method] BytecodeArrayBuilder& LoadFalse()",
  "[public/method] BytecodeArrayBuilder& LoadBoolean(bool value)",
  "[public/method] BytecodeArrayBuilder& SetPrototypeProperties(size_t index_obj, size_t slot)",
  "[public/method] BytecodeArrayBuilder& LoadGlobal(const AstRawString* name, int feedback_slot, TypeofMode typeof_mode)",
  "[public/method] BytecodeArrayBuilder& StoreGlobal(const AstRawString* name, int feedback_slot)",
  "[public/method] BytecodeArrayBuilder& LoadContextSlot(Register context, Variable* variable, int depth)",
  "[public/method] BytecodeArrayBuilder& StoreContextSlot(Register context, Variable* variable, int depth)",
  "[public/method] BytecodeArrayBuilder& LoadModuleVariable(int cell_index, int depth)",
  "[public/method] BytecodeArrayBuilder& StoreModuleVariable(int cell_index, int depth)",
  "[public/method] BytecodeArrayBuilder& LoadAccumulatorWithRegister(Register reg)",
  "[public/method] BytecodeArrayBuilder& StoreAccumulatorInRegister(Register reg)",
  "[public/method] BytecodeArrayBuilder& MoveRegister(Register from, Register to)",
  "[public/method] BytecodeArrayBuilder& LoadNamedProperty(Register object, const AstRawString* name, int feedback_slot)",
  "[public/method] BytecodeArrayBuilder& LoadNamedPropertyFromSuper(Register object, const AstRawString* name, int feedback_slot)",
  "[public/method] BytecodeArrayBuilder& LoadKeyedProperty(Register object, int feedback_slot)",
  "[public/method] BytecodeArrayBuilder& LoadEnumeratedKeyedProperty(Register object, Register enum_index, Register cache_type, int feedback_slot)",
  "[public/method] BytecodeArrayBuilder& GetPrivateField(Register context, int slot_index, int depth, Register object, int feedback_slot)",
  "[public/method] BytecodeArrayBuilder& SetPrivateField(Register context, int slot_index, int depth, Register object, int feedback_slot)",
  "[public/method] BytecodeArrayBuilder& LoadIteratorProperty(Register object, int feedback_slot)",
  "[public/method] BytecodeArrayBuilder& GetIterator(Register object, int load_feedback_slot, int call_feedback_slot)",
  "[public/method] BytecodeArrayBuilder& ArrayDestructure(RegisterList outputs, int count)",
  "[public/method] BytecodeArrayBuilder& LoadAsyncIteratorProperty(Register object, int feedback_slot)",
  "[public/method] BytecodeArrayBuilder& DefineKeyedOwnPropertyInLiteral( Register object, Register name, DefineKeyedOwnPropertyInLiteralFlags flags, int feedback_slot)",
  "[public/method] BytecodeArrayBuilder& SetNamedProperty(Register object, const AstRawString* name, int feedback_slot, LanguageMode language_mode)",
  "[public/method] BytecodeArrayBuilder& SetNamedProperty(Register object, size_t constant_pool_entry, int feedback_slot, LanguageMode language_mode)",
  "[public/method] BytecodeArrayBuilder& DefineNamedOwnProperty(Register object, const AstRawString* name, int feedback_slot)",
  "[public/method] BytecodeArrayBuilder& DefineNamedOwnPropertyInLiteral( Register object, const AstRawString* name, int feedback_slot)",
  "[public/method] BytecodeArrayBuilder& SetKeyedProperty(Register object, Register key, int feedback_slot, LanguageMode language_mode)",
  "[public/method] BytecodeArrayBuilder& DefineKeyedOwnProperty( Register object, Register key, DefineKeyedOwnPropertyFlags flags, int feedback_slot)",
  "[public/method] BytecodeArrayBuilder& StoreInArrayLiteral(Register array, Register index, int feedback_slot)",
  "[public/method] BytecodeArrayBuilder& StoreClassFieldsInitializer(Register constructor, int feedback_slot)",
  "[public/method] BytecodeArrayBuilder& LoadClassFieldsInitializer(Register constructor, int feedback_slot)",
  "[public/method] BytecodeArrayBuilder& LoadLookupSlot(const AstRawString* name, TypeofMode typeof_mode)",
  "[public/method] BytecodeArrayBuilder& LoadLookupContextSlot(const AstRawString* name, TypeofMode typeof_mode, ContextMode context_mode, int slot_index, int depth)",
  "[public/method] BytecodeArrayBuilder& LoadLookupGlobalSlot(const AstRawString* name, TypeofMode typeof_mode, int feedback_slot, int depth)",
  "[public/method] BytecodeArrayBuilder& StoreLookupSlot( const AstRawString* name, LanguageMode language_mode, LookupHoistingMode lookup_hoisting_mode)",
  "[public/method] BytecodeArrayBuilder& CreateClosure(size_t shared_function_info_entry, int slot, int flags)",
  "[public/method] BytecodeArrayBuilder& CreateBlockContext(const Scope* scope)",
  "[public/method] BytecodeArrayBuilder& CreateCatchContext(Register exception, const Scope* scope)",
  "[public/method] BytecodeArrayBuilder& CreateFunctionContext(const Scope* scope, int slots)",
  "[public/method] BytecodeArrayBuilder& CreateEvalContext(const Scope* scope, int slots)",
  "[public/method] BytecodeArrayBuilder& CreateWithContext(Register object, const Scope* scope)",
  "[public/method] BytecodeArrayBuilder& CreateArguments(CreateArgumentsType type)",
  "[public/method] BytecodeArrayBuilder& CreateRegExpLiteral(const AstRawString* pattern, int literal_index, int flags)",
  "[public/method] BytecodeArrayBuilder& CreateArrayLiteral(size_t constant_elements_entry, int literal_index, int flags)",
  "[public/method] BytecodeArrayBuilder& CreateEmptyArrayLiteral(int literal_index)",
  "[public/method] BytecodeArrayBuilder& CreateArrayFromIterable()",
  "[public/method] BytecodeArrayBuilder& CreateObjectLiteral(size_t constant_properties_entry, int literal_index, int flags)",
  "[public/method] BytecodeArrayBuilder& CreateEmptyObjectLiteral()",
  "[public/method] BytecodeArrayBuilder& CloneObject(Register source, int flags, int feedback_slot)",
  "[public/method] BytecodeArrayBuilder& GetTemplateObject( size_t template_object_description_entry, int feedback_slot)",
  "[public/method] BytecodeArrayBuilder& PushContext(Register context)",
  "[public/method] BytecodeArrayBuilder& PopContext(Register context)",
  "[public/method] BytecodeArrayBuilder& CallProperty(Register callable, RegisterList args, int feedback_slot)",
  "[public/method] BytecodeArrayBuilder& CallUndefinedReceiver(Register callable, RegisterList args, int feedback_slot)",
  "[public/method] BytecodeArrayBuilder& CallAnyReceiver(Register callable, RegisterList args, int feedback_slot)",
  "[public/method] BytecodeArrayBuilder& TailCall(Register callable, RegisterList args, int feedback_slot)",
  "[public/method] BytecodeArrayBuilder& CallWithSpread(Register callable, RegisterList args, int feedback_slot)",
  "[public/method] BytecodeArrayBuilder& Construct(Register constructor, RegisterList args, int feedback_slot)",
  "[public/method] BytecodeArrayBuilder& ConstructWithSpread(Register constructor, RegisterList args, int feedback_slot)",
  "[public/method] BytecodeArrayBuilder& ConstructForwardAllArgs(Register constructor, int feedback_slot)",
  "[public/method] BytecodeArrayBuilder& CallRuntime(Runtime::FunctionId function_id, RegisterList args)",
  "[public/method] BytecodeArrayBuilder& CallRuntime(Runtime::FunctionId function_id, Register arg)",
  "[public/method] BytecodeArrayBuilder& CallRuntime(Runtime::FunctionId function_id)",
  "[public/method] BytecodeArrayBuilder& CallRuntimeForPair(Runtime::FunctionId function_id, RegisterList args, RegisterList return_pair)",
  "[public/method] BytecodeArrayBuilder& CallRuntimeForPair(Runtime::FunctionId function_id, Register arg, RegisterList return_pair)",
  "[public/method] BytecodeArrayBuilder& CallJSRuntime(int context_index, RegisterList args)",
  "[public/method] BytecodeArrayBuilder& BinaryOperation(Token::Value binop, Register reg, int feedback_slot)",
  "[public/method] BytecodeArrayBuilder& Add_StringConstant_Internalize( Token::Value binop, Register reg, int feedback_slot, AddStringConstantAndInternalizeVariant as_variant)",
  "[public/method] BytecodeArrayBuilder& BinaryOperationSmiLiteral(Token::Value binop, Tagged<Smi> literal, int feedback_slot)",
  "[public/method] BytecodeArrayBuilder& UnaryOperation(Token::Value op, int feedback_slot)",
  "[public/method] BytecodeArrayBuilder& LogicalNot(ToBooleanMode mode)",
  "[public/method] BytecodeArrayBuilder& TypeOf(int feedback_slot)",
  "[public/method] BytecodeArrayBuilder& GetSuperConstructor(Register out)",
  "[public/method] BytecodeArrayBuilder& FindNonDefaultConstructorOrConstruct( Register this_function, Register new_target, RegisterList output)",
  "[public/method] BytecodeArrayBuilder& Delete(Register object, LanguageMode language_mode)",
  "[public/method] BytecodeArrayBuilder& CompareOperation(Token::Value op, Register reg, int feedback_slot)",
  "[public/method] BytecodeArrayBuilder& CompareReference(Register reg)",
  "[public/method] BytecodeArrayBuilder& CompareUndetectable()",
  "[public/method] BytecodeArrayBuilder& CompareUndefined()",
  "[public/method] BytecodeArrayBuilder& CompareNull()",
  "[public/method] BytecodeArrayBuilder& CompareNil(Token::Value op, NilValue nil)",
  "[public/method] BytecodeArrayBuilder& CompareTypeOf( TestTypeOfFlags::LiteralFlag literal_flag)",
  "[public/method] BytecodeArrayBuilder& ToObject(Register out)",
  "[public/method] BytecodeArrayBuilder& ToName()",
  "[public/method] BytecodeArrayBuilder& ToString()",
  "[public/method] BytecodeArrayBuilder& ToBoolean(ToBooleanMode mode)",
  "[public/method] BytecodeArrayBuilder& ToNumber(int feedback_slot)",
  "[public/method] BytecodeArrayBuilder& ToNumeric(int feedback_slot)",
  "[public/method] BytecodeArrayBuilder& MarkHandler(int handler_id, HandlerTable::CatchPrediction will_catch)",
  "[public/method] BytecodeArrayBuilder& MarkTryBegin(int handler_id, Register context)",
  "[public/method] BytecodeArrayBuilder& MarkTryEnd(int handler_id)",
  "[public/method] BytecodeArrayBuilder& DropHandlerEntry(int handler_id)",
  "[public/method] BytecodeArrayBuilder& Bind(BytecodeLabel* label)",
  "[public/method] BytecodeArrayBuilder& Bind(BytecodeLoopHeader* label)",
  "[public/method] BytecodeArrayBuilder& Bind(BytecodeJumpTable* jump_table, int case_value)",
  "[public/method] BytecodeArrayBuilder& Jump(BytecodeLabel* label)",
  "[public/method] BytecodeArrayBuilder& JumpLoop(BytecodeLoopHeader* loop_header, int loop_depth, int position, int feedback_slot)",
  "[public/method] BytecodeArrayBuilder& JumpIfTrue(ToBooleanMode mode, BytecodeLabel* label)",
  "[public/method] BytecodeArrayBuilder& JumpIfFalse(ToBooleanMode mode, BytecodeLabel* label)",
  "[public/method] BytecodeArrayBuilder& JumpIfJSReceiver(BytecodeLabel* label)",
  "[public/method] BytecodeArrayBuilder& JumpIfNull(BytecodeLabel* label)",
  "[public/method] BytecodeArrayBuilder& JumpIfNotNull(BytecodeLabel* label)",
  "[public/method] BytecodeArrayBuilder& JumpIfUndefined(BytecodeLabel* label)",
  "[public/method] BytecodeArrayBuilder& JumpIfUndefinedOrNull(BytecodeLabel* label)",
  "[public/method] BytecodeArrayBuilder& JumpIfNotUndefined(BytecodeLabel* label)",
  "[public/method] BytecodeArrayBuilder& JumpIfNil(BytecodeLabel* label, Token::Value op, NilValue nil)",
  "[public/method] BytecodeArrayBuilder& JumpIfNotNil(BytecodeLabel* label, Token::Value op, NilValue nil)",
  "[public/method] BytecodeArrayBuilder& JumpIfForInDone(BytecodeLabel* label, Register index, Register cache_length)",
  "[public/method] BytecodeArrayBuilder& SwitchOnSmiNoFeedback(BytecodeJumpTable* jump_table)",
  "[public/method] BytecodeArrayBuilder& SetPendingMessage()",
  "[public/method] BytecodeArrayBuilder& Throw()",
  "[public/method] BytecodeArrayBuilder& ReThrow()",
  "[public/method] BytecodeArrayBuilder& Abort(AbortReason reason)",
  "[public/method] BytecodeArrayBuilder& Return()",
  "[public/method] BytecodeArrayBuilder& ThrowReferenceErrorIfTdzHole(const AstRawString* name)",
  "[public/method] BytecodeArrayBuilder& ThrowSuperNotCalledIfTdzHole()",
  "[public/method] BytecodeArrayBuilder& ThrowSuperAlreadyCalledIfNotTdzHole()",
  "[public/method] BytecodeArrayBuilder& ThrowIfNotSuperConstructor(Register constructor)",
  "[public/method] BytecodeArrayBuilder& Debugger()",
  "[public/method] BytecodeArrayBuilder& IncBlockCounter(int slot)",
  "[public/method] BytecodeArrayBuilder& ForInEnumerate(Register receiver)",
  "[public/method] BytecodeArrayBuilder& ForInPrepare(RegisterList cache_info_triple, int feedback_slot)",
  "[public/method] BytecodeArrayBuilder& ForInNext(Register receiver, Register index, RegisterList cache_type_array_pair, int feedback_slot)",
  "[public/method] BytecodeArrayBuilder& ForInStep(Register index)",
  "[public/method] BytecodeArrayBuilder& ForOfNext(Register object, Register next, int call_slot)",
  "[public/method] BytecodeArrayBuilder& SuspendGenerator(Register generator, RegisterList registers, int suspend_id)",
  "[public/method] BytecodeArrayBuilder& SwitchOnGeneratorState(Register generator, BytecodeJumpTable* jump_table)",
  "[public/method] BytecodeArrayBuilder& ResumeGenerator(Register generator, RegisterList registers)",
  "[public/method] int NewHandlerEntry()",
  "[public/method] BytecodeJumpTable* AllocateJumpTable(int size, int case_value_base)",
  "[public/method] void TrimJumpTable(BytecodeJumpTable* jump_table, int size)",
  "[public/method] BytecodeRegisterOptimizer* GetRegisterOptimizer()",
  "[public/method] size_t GetConstantPoolEntry(const AstRawString* raw_string)",
  "[public/method] size_t GetConstantPoolEntry(const AstConsString* cons_string)",
  "[public/method] size_t GetConstantPoolEntry(AstBigInt bigint)",
  "[public/method] size_t GetConstantPoolEntry(const Scope* scope)",
  "[public/method] size_t GetConstantPoolEntry(double number)",
  "[public/method] SINGLETON_CONSTANT_ENTRY_TYPES(ENTRY_GETTER) size_t AllocateDeferredConstantPoolEntry()",
  "[public/method] void SetDeferredConstantPoolEntry(size_t entry, Handle<Object> object)",
  "[public/method] void InitializeReturnPosition(FunctionLiteral* literal)",
  "[public/method] void SetStatementPosition(Statement* stmt, bool is_breakable = true)",
  "[public/method] std::optional<BytecodeSourceInfo> MaybePopSourcePosition(int scope_start)",
  "[public/method] void PushSourcePosition(BytecodeSourceInfo source_info)",
  "[public/method] void SetStatementPosition(int position, bool is_breakable = true)",
  "[public/method] void SetExpressionPosition(Expression* expr)",
  "[public/method] void SetExpressionPosition(int position)",
  "[public/method] void SetExpressionAsStatementPosition(Expression* expr, bool is_breakable = true)",
  "[public/method] bool RemainderOfBlockIsDead() const",
  "[public/method] uint32_t GetInputRegisterOperand(Register reg)",
  "[public/method] uint32_t GetOutputRegisterOperand(Register reg)",
  "[public/method] uint32_t GetInputOutputRegisterOperand(Register reg)",
  "[public/method] uint32_t GetInputRegisterListOperand(RegisterList reg_list)",
  "[public/method] uint32_t GetOutputRegisterListOperand(RegisterList reg_list)",
  "[public/method] void OutputLdarRaw(Register reg)",
  "[public/method] void OutputStarRaw(Register reg)",
  "[public/method] void OutputMovRaw(Register src, Register dest)",
  "[public/method] void EmitFunctionStartSourcePosition(int position)",
  "[public/method] size_t current_bytecode_size() const",
  "[public/method] BytecodeRegisterAllocator* register_allocator()",
  "[public/method] const BytecodeRegisterAllocator* register_allocator() const",
  "[public/method] Zone* zone() const",
  "[private/method] const FeedbackVectorSpec* feedback_vector_spec() const",
  "[private/method] V8_INLINE BytecodeSourceInfo CurrentSourcePosition(Bytecode bytecode)",
  r"[private/method] \ template <typename... Operands> \ V8_INLINE void Output##Name(Operands... operands)",
  r"[private/method] \ template <typename... Operands> \ V8_INLINE void Output##Name(BytecodeLabel* label, Operands... operands)",
  "[private/method] BYTECODE_LIST(DECLARE_BYTECODE_OUTPUT, DECLARE_BYTECODE_OUTPUT) V8_INLINE void OutputJumpLoop(BytecodeLoopHeader* loop_header, int loop_depth, int feedback_slot)",
  "[private/method] V8_INLINE void OutputSwitchOnSmiNoFeedback(BytecodeJumpTable* jump_table)",
  "[private/method] bool RegisterIsValid(Register reg) const",
  "[private/method] bool RegisterListIsValid(RegisterList reg_list) const",
  "[private/method] void SetDeferredSourceInfo(BytecodeSourceInfo source_info)",
  "[private/method] void AttachOrEmitDeferredSourceInfo(BytecodeNode* node)",
  "[private/method] void Write(BytecodeNode* node)",
  "[private/method] void WriteJump(BytecodeNode* node, BytecodeLabel* label)",
  "[private/method] void WriteJumpLoop(BytecodeNode* node, BytecodeLoopHeader* loop_header)",
  "[private/method] void WriteSwitch(BytecodeNode* node, BytecodeJumpTable* label)",
  "[private/method] BytecodeArrayBuilder& Illegal()",
  "[private/method] BytecodeArrayWriter* bytecode_array_writer()",
  "[private/method] ConstantArrayBuilder* constant_array_builder()",
  "[private/method] const ConstantArrayBuilder* constant_array_builder() const",
  "[private/method] HandlerTableBuilder* handler_table_builder()",
  "[private/field] Zone* zone_",
  "[private/field] FeedbackVectorSpec* feedback_vector_spec_",
  "[private/field] bool bytecode_generated_",
  "[private/field] ConstantArrayBuilder constant_array_builder_",
  "[private/field] HandlerTableBuilder handler_table_builder_",
  "[private/field] uint16_t parameter_count_",
  "[private/field] uint16_t max_arguments_",
  "[private/field] int local_register_count_",
  "[private/field] BytecodeRegisterAllocator register_allocator_",
  "[private/field] BytecodeArrayWriter bytecode_array_writer_",
  "[private/field] BytecodeRegisterOptimizer* register_optimizer_",
  "[private/field] BytecodeSourceInfo latest_source_info_",
  "[private/field] BytecodeSourceInfo deferred_source_info_",
  "[private/field] int potentially_throwing_bytecode_count_",
];

/// 原版 class/struct ThrowTrackingScope 成员声明 (机械 1:1 提取, 4 条)
pub const THROW_TRACKING_SCOPE_DECLARATIONS: &[&str] = &[
  "[public/method] explicit ThrowTrackingScope(BytecodeArrayBuilder* builder) : builder_(builder), start_count_(builder->potentially_throwing_bytecode_count())",
  "[public/method] bool HasEmittedThrowingBytecode() const",
  "[private/field] BytecodeArrayBuilder* builder_",
  "[private/field] int start_count_",
];

// —— 提取不变式断言 ——
const _: () = {
  assert!(BYTECODE_ARRAY_BUILDER_DECLARATIONS.len() == 221);
  assert!(THROW_TRACKING_SCOPE_DECLARATIONS.len() == 4);
};
