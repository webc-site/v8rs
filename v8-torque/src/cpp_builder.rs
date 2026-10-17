// 由 tool/src/torque/generators.js 从 v8/src/torque/cpp-builder.h 生成; 修订就地改, 复跑覆盖
// C++ 代码构建辅助 (类/函数/文件缓冲)

use std::{collections::BTreeSet, mem};

use crate::{
  error::{Error, Result},
  source_positions::SourcePosition,
};

// 原版面向 ostream 的构建器 → Rust String 缓冲; include 语义随生成器定型复核
// (Rust 版产出 Rust 代码, cpp-builder 主要服务 cc-debug 输出路径)。
/// 原版 TemplateParameter 1:1
#[derive(Debug, Clone, Default)]
pub struct TemplateParameter {
  pub name: String,
  pub type_: String,
}

impl TemplateParameter {
  #[must_use]
  pub fn named(name: String) -> Self {
    Self {
      name,
      type_: String::new(),
    }
  }

  #[must_use]
  pub fn typed(type_: String, name: String) -> Self {
    Self { name, type_ }
  }
}

/// 原版 cpp::Class 1:1
#[derive(Debug, Clone, Default)]
pub struct Class {
  template_parameters: Vec<TemplateParameter>,
  name: String,
}

impl Class {
  #[must_use]
  pub fn new(name: String) -> Self {
    Self {
      template_parameters: Vec::new(),
      name,
    }
  }

  #[must_use]
  pub fn with_template_parameters(
    template_parameters: Vec<TemplateParameter>,
    name: String,
  ) -> Self {
    Self {
      template_parameters,
      name,
    }
  }

  /// 原版 GetName
  #[must_use]
  pub fn name(&self) -> &str {
    &self.name
  }

  /// 原版 GetTemplateParameters
  #[must_use]
  pub fn template_parameters(&self) -> &[TemplateParameter] {
    &self.template_parameters
  }
}

/// 原版 FUNCTION_FLAG_LIST X-macro 展开 (Function::FunctionFlag)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u32)]
pub enum FunctionFlag {
  Inline = 0x01,
  V8Inline = 0x03,
  Const = 0x04,
  Constexpr = 0x08,
  Export = 0x10,
  Static = 0x20,
}

/// 原版 base::Flags<FunctionFlag> → 位掩码 u32
pub type FunctionFlags = u32;

/// 原版 Function::Parameter 1:1
#[derive(Debug, Clone, Default)]
pub struct Parameter {
  pub type_: String,
  pub name: String,
  pub default_value: String,
}

impl Parameter {
  #[must_use]
  pub fn new(type_: String, name: String, default_value: String) -> Self {
    Self {
      type_,
      name,
      default_value,
    }
  }
}

/// 原版 cpp::Function 1:1 (owning_class 指针 → Option<String> 类名)
#[derive(Debug, Clone, Default)]
pub struct Function {
  pub pos: SourcePosition,
  pub owning_class: Option<String>,
  pub description: String,
  name: String,
  return_type: String,
  parameters: Vec<Parameter>,
  flags: FunctionFlags,
}

impl Function {
  pub const K_AUTOMATIC_INDENTATION: i32 = -1;

  /// 原版 Function(name)
  #[must_use]
  pub fn new(name: String) -> Self {
    Self {
      pos: SourcePosition::invalid(),
      owning_class: None,
      description: String::new(),
      name,
      return_type: String::new(),
      parameters: Vec::new(),
      flags: 0,
    }
  }

  /// 原版 Function(owner, name)
  #[must_use]
  pub fn in_class(owner: &Class, name: String) -> Self {
    let mut f = Self::new(name);
    f.owning_class = Some(owner.name().to_owned());
    f
  }

  /// 原版 DefaultGetter
  #[must_use]
  pub fn default_getter(return_type: String, owner: &Class, name: String) -> Self {
    let mut f = Self::in_class(owner, name);
    f.return_type = return_type;
    f.set_flag(FunctionFlag::Inline, true);
    f.set_flag(FunctionFlag::Const, true);
    f
  }

  /// 原版 DefaultSetter
  #[must_use]
  pub fn default_setter(
    owner: &Class,
    name: String,
    parameter_type: String,
    parameter_name: String,
  ) -> Self {
    let mut f = Self::in_class(owner, name);
    f.return_type = "void".to_owned();
    f.parameters.push(Parameter::new(
      parameter_type,
      parameter_name,
      String::new(),
    ));
    f.set_flag(FunctionFlag::Inline, true);
    f
  }

  /// 原版 SetFlag
  pub fn set_flag(&mut self, flag: FunctionFlag, value: bool) {
    if value {
      self.flags |= flag as u32;
    } else {
      self.flags &= !(flag as u32);
    }
  }

  /// 原版 HasFlag
  #[must_use]
  pub fn has_flag(&self, flag: FunctionFlag) -> bool {
    (self.flags & flag as u32) == flag as u32
  }

  /// 原版 SetInline / IsInline
  pub fn set_inline(&mut self, v: bool) {
    self.set_flag(FunctionFlag::Inline, v);
  }

  #[must_use]
  pub fn is_inline(&self) -> bool {
    self.has_flag(FunctionFlag::Inline)
  }

  /// 原版 SetV8Inline / IsV8Inline
  pub fn set_v8_inline(&mut self, v: bool) {
    self.set_flag(FunctionFlag::V8Inline, v);
  }

  #[must_use]
  pub fn is_v8_inline(&self) -> bool {
    self.has_flag(FunctionFlag::V8Inline)
  }

  /// 原版 SetConst / IsConst
  pub fn set_const(&mut self, v: bool) {
    self.set_flag(FunctionFlag::Const, v);
  }

  #[must_use]
  pub fn is_const(&self) -> bool {
    self.has_flag(FunctionFlag::Const)
  }

  /// 原版 SetConstexpr / IsConstexpr
  pub fn set_constexpr(&mut self, v: bool) {
    self.set_flag(FunctionFlag::Constexpr, v);
  }

  #[must_use]
  pub fn is_constexpr(&self) -> bool {
    self.has_flag(FunctionFlag::Constexpr)
  }

  /// 原版 SetExport / IsExport
  pub fn set_export(&mut self, v: bool) {
    self.set_flag(FunctionFlag::Export, v);
  }

  #[must_use]
  pub fn is_export(&self) -> bool {
    self.has_flag(FunctionFlag::Export)
  }

  /// 原版 SetStatic / IsStatic
  pub fn set_static(&mut self, v: bool) {
    self.set_flag(FunctionFlag::Static, v);
  }

  #[must_use]
  pub fn is_static(&self) -> bool {
    self.has_flag(FunctionFlag::Static)
  }

  /// 原版 SetDescription / SetName / SetReturnType
  pub fn set_description(&mut self, description: String) {
    self.description = description;
  }

  pub fn set_name(&mut self, name: String) {
    self.name = name;
  }

  pub fn set_return_type(&mut self, return_type: String) {
    self.return_type = return_type;
  }

  /// 原版 AddParameter
  pub fn add_parameter(&mut self, type_: String, name: String, default_value: String) {
    self
      .parameters
      .push(Parameter::new(type_, name, default_value));
  }

  /// 原版 InsertParameter
  pub fn insert_parameter(
    &mut self,
    index: usize,
    type_: String,
    name: String,
    default_value: String,
  ) {
    self
      .parameters
      .insert(index, Parameter::new(type_, name, default_value));
  }

  /// 原版 GetParameters
  #[must_use]
  pub fn parameters(&self) -> &[Parameter] {
    &self.parameters
  }

  /// 原版 GetParameterNames
  #[must_use]
  pub fn parameter_names(&self) -> Vec<&str> {
    self.parameters.iter().map(|p| p.name.as_str()).collect()
  }

  /// 原版 PrintDeclaration
  pub fn print_declaration(&self, stream: &mut String, indentation: i32) -> Result<()> {
    // TODO(填体)
    let _ = (stream, indentation);
    Err(Error::Unimplemented)
  }

  /// 原版 PrintDefinition
  pub fn print_definition(
    &self,
    stream: &mut String,
    builder: &dyn Fn(&mut String),
    indentation: i32,
  ) -> Result<()> {
    // TODO(填体)
    let _ = (stream, builder, indentation);
    Err(Error::Unimplemented)
  }

  /// 原版 PrintBeginDefinition / PrintEndDefinition / PrintInlineDefinition
  pub fn print_begin_definition(&self, stream: &mut String, indentation: i32) -> Result<()> {
    // TODO(填体)
    let _ = (stream, indentation);
    Err(Error::Unimplemented)
  }

  pub fn print_end_definition(&self, stream: &mut String, indentation: i32) -> Result<()> {
    // TODO(填体)
    let _ = (stream, indentation);
    Err(Error::Unimplemented)
  }
}

/// 原版 cpp::File 1:1 (ostream* → String 缓冲; namespace 栈保序)
#[derive(Debug, Default)]
pub struct File {
  pub buf: String,
  includes: BTreeSet<String>,
  namespaces: Vec<String>,
}

impl File {
  /// 原版 BeginIncludeGuard
  pub fn begin_include_guard(&mut self, name: &str) {
    self.buf.push_str("#ifndef ");
    self.buf.push_str(name);
    self.buf.push_str("\n#define ");
    self.buf.push_str(name);
    self.buf.push('\n');
  }

  /// 原版 EndIncludeGuard
  pub fn end_include_guard(&mut self, name: &str) {
    self.buf.push_str("#endif  // ");
    self.buf.push_str(name);
    self.buf.push('\n');
  }

  /// 原版 BeginNamespace (name 版)
  pub fn begin_namespace(&mut self, name: &str) {
    self.buf.push_str("namespace ");
    self.buf.push_str(name);
    self.buf.push_str(" {\n");
    self.namespaces.push(name.to_owned());
  }

  /// 原版 BeginNamespace (name0, name1) 嵌套版
  pub fn begin_namespace2(&mut self, name0: &str, name1: &str) {
    self.begin_namespace(name0);
    self.begin_namespace(name1);
  }

  /// 原版 EndNamespace (name 版)
  pub fn end_namespace(&mut self, name: &str) -> Result<()> {
    let top = self.namespaces.pop().ok_or(Error::Unimplemented)?;
    if top != name {
      return Err(Error::Unimplemented);
    }
    self.buf.push_str("}  // namespace ");
    self.buf.push_str(name);
    self.buf.push('\n');
    Ok(())
  }

  /// 原版 EndNamespace (name0, name1) 嵌套版
  pub fn end_namespace2(&mut self, name0: &str, name1: &str) -> Result<()> {
    self.end_namespace(name1)?;
    self.end_namespace(name0)
  }

  /// 原版 AddInclude
  pub fn add_include(&mut self, include: String) {
    self.includes.insert(include);
  }

  /// 原版 operator<< → write
  pub fn write(&mut self, s: &str) {
    self.buf.push_str(s);
  }

  /// include 区渲染 (原版在 PrintDeclaration 前集中输出)
  pub fn flush_includes(&mut self) {
    // TODO(填体): 输出位置与原版对齐
    for inc in mem::take(&mut self.includes) {
      self.buf.push_str("#include \"");
      self.buf.push_str(&inc);
      self.buf.push_str("\"\n");
    }
  }
}

/// 原版 IncludeGuardScope (RAII → 显式 begin/end 对, 登记偏差)
#[derive(Debug)]
pub struct IncludeGuardScope {
  name: String,
  active: bool,
}

impl IncludeGuardScope {
  #[must_use]
  pub fn begin(file: &mut File, name: String) -> Self {
    file.begin_include_guard(&name);
    Self { name, active: true }
  }

  pub fn end(self, file: &mut File) {
    if self.active {
      file.end_include_guard(&self.name);
    }
  }
}

// ── 原版 src/torque/cpp-builder.cc ──
// L12: void Function::PrintDeclarationHeader(std::ostream& stream, int indentation) const
// L47: void Function::PrintDeclaration(std::ostream& stream, int indentation) const
// L55: void Function::PrintDefinition( std::ostream& stream, const std::function<void(std::ostream&)>& builder, int indentation) const
// L65: void Function::PrintInlineDefinition( std::ostream& stream, const std::function<void(std::ostream&)>& builder, int indentation) const
// L76: void Function::PrintBeginDefinition(std::ostream& stream, int indentation) const
// L129: void Function::PrintEndDefinition(std::ostream& stream, int indentation) const
// L134: void File::BeginIncludeGuard(const std::string& name)
// L141: void File::EndIncludeGuard(const std::string& name)
// L145: void File::BeginNamespace(std::string name)
// L152: void File::BeginNamespace(std::string name0, std::string name1)
// L157: void File::EndNamespace(const std::string& name)
// L164: void File::EndNamespace(const std::string& name0, const std::string& name1)
