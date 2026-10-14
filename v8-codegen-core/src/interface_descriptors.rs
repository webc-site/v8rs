use crate::error::Result;

pub const K_JS_BUILTIN_REGISTER_PARAMS: usize = 4;
pub const K_MAX_BUILTIN_REGISTER_PARAMS: usize = 5;
pub const K_MAX_TFS_BUILTIN_REGISTER_PARAMS: usize = 6;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum StackArgumentOrder {
  #[default]
  HighToLow,
  LowToHigh,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct CallInterfaceDescriptorFlags(u32);

impl CallInterfaceDescriptorFlags {
  pub const NONE: Self = Self(0);
  pub const NO_CONTEXT: Self = Self(1 << 0);
  pub const CALLEE_SAVE_REGISTERS: Self = Self(1 << 1);
  pub const ALLOW_VAR_ARGS: Self = Self(1 << 2);

  #[inline]
  pub const fn union(self, other: Self) -> Self {
    Self(self.0 | other.0)
  }

  #[inline]
  pub const fn contains(self, other: Self) -> bool {
    (self.0 & other.0) == other.0
  }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RegistersInit<'a> {
  pub flags: CallInterfaceDescriptorFlags,
  pub tag: u32,
  pub return_count: usize,
  pub param_count: usize,
  pub stack_order: StackArgumentOrder,
  pub register_params: &'a [usize],
  pub return_registers: &'a [usize],
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct CallInterfaceDescriptorData {
  pub flags: CallInterfaceDescriptorFlags,
  pub tag: u32,
  pub return_count: usize,
  pub param_count: usize,
  pub stack_order: StackArgumentOrder,
  pub register_params: Vec<usize>,
  pub return_registers: Vec<usize>,
}

impl CallInterfaceDescriptorData {
  #[inline]
  pub const fn new() -> Self {
    Self {
      flags: CallInterfaceDescriptorFlags::NONE,
      tag: 0,
      return_count: 0,
      param_count: 0,
      stack_order: StackArgumentOrder::HighToLow,
      register_params: Vec::new(),
      return_registers: Vec::new(),
    }
  }

  #[inline]
  pub fn initialize_registers(&mut self, init: RegistersInit<'_>) -> Result<()> {
    self.flags = init.flags;
    self.tag = init.tag;
    self.return_count = init.return_count;
    self.param_count = init.param_count;
    self.stack_order = init.stack_order;
    self.register_params = init.register_params.to_vec();
    self.return_registers = init.return_registers.to_vec();
    Ok(())
  }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CallInterfaceDescriptor {
  data: CallInterfaceDescriptorData,
}

impl CallInterfaceDescriptor {
  #[inline]
  pub const fn new(data: CallInterfaceDescriptorData) -> Self {
    Self { data }
  }

  #[inline]
  pub fn has_context_parameter(&self) -> bool {
    !self
      .data
      .flags
      .contains(CallInterfaceDescriptorFlags::NO_CONTEXT)
  }

  #[inline]
  pub fn callee_save_registers(&self) -> bool {
    self
      .data
      .flags
      .contains(CallInterfaceDescriptorFlags::CALLEE_SAVE_REGISTERS)
  }

  #[inline]
  pub fn allow_var_args(&self) -> bool {
    self
      .data
      .flags
      .contains(CallInterfaceDescriptorFlags::ALLOW_VAR_ARGS)
  }

  #[inline]
  pub const fn return_count(&self) -> usize {
    self.data.return_count
  }

  #[inline]
  pub const fn param_count(&self) -> usize {
    self.data.param_count
  }

  #[inline]
  pub fn register_param_count(&self) -> usize {
    self.data.register_params.len()
  }

  #[inline]
  pub fn stack_param_count(&self) -> usize {
    self
      .data
      .param_count
      .saturating_sub(self.register_param_count())
  }

  #[inline]
  pub const fn stack_order(&self) -> StackArgumentOrder {
    self.data.stack_order
  }

  #[inline]
  pub fn register_param(&self, index: usize) -> Option<usize> {
    self.data.register_params.get(index).copied()
  }

  #[inline]
  pub fn register_return(&self, index: usize) -> Option<usize> {
    self.data.return_registers.get(index).copied()
  }
}
