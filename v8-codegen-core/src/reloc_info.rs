#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum RelocMode {
  NoInfo = 0,
  CodeTarget = 1,
  RelativeCodeTarget = 2,
  CompressedEmbeddedObject = 3,
  FullEmbeddedObject = 4,
  JsDispatchHandle = 5,
  WasmCall = 6,
  WasmStubCall = 7,
  WasmCodePointerTableEntry = 8,
  WasmCanonicalSigId = 9,
  WasmCodePointer = 10,
  ExternalReference = 11,
  InternalReference = 12,
  InternalReferenceEncoded = 13,
  OffHeapTarget = 14,
  NearBuiltinEntry = 15,
  ConstPool = 16,
  VeneerPool = 17,
  DeoptScriptOffset = 18,
  DeoptInliningId = 19,
  DeoptReason = 20,
  DeoptId = 21,
  DeoptNodeId = 22,
  PcJump = 23,
  NumberOfModes = 24,
}

pub const K_TAG_BITS: u32 = 3;
pub const K_TAG_MASK: u32 = (1 << K_TAG_BITS) - 1;
pub const K_LONG_TAG_BITS: u32 = 6;
pub const K_DEFAULT_TAG: u32 = 0;
pub const K_EMBEDDED_OBJECT_TAG: u32 = 1;
pub const K_CODE_TARGET_TAG: u32 = 2;
pub const K_WASM_STUB_CALL_TAG: u32 = 3;
pub const K_SMALL_PC_DELTA_BITS: u32 = 5;
pub const K_SMALL_PC_DELTA_MASK: u32 = (1 << K_SMALL_PC_DELTA_BITS) - 1;
pub const K_MAX_SMALL_PC_DELTA: u32 = K_SMALL_PC_DELTA_MASK;
pub const K_ALL_MODES_MASK: u32 = (1 << (RelocMode::NumberOfModes as u32)) - 1;

#[inline]
pub const fn mode_mask(mode: RelocMode) -> u32 {
  1 << (mode as u32)
}

#[inline]
pub const fn code_target_mask() -> u32 {
  mode_mask(RelocMode::CodeTarget) | mode_mask(RelocMode::RelativeCodeTarget)
}

#[inline]
pub const fn embedded_object_mode_mask() -> u32 {
  mode_mask(RelocMode::FullEmbeddedObject) | mode_mask(RelocMode::CompressedEmbeddedObject)
}

#[inline]
pub const fn js_dispatch_handle_mode_mask() -> u32 {
  mode_mask(RelocMode::JsDispatchHandle)
}

#[inline]
pub const fn all_real_modes_mask() -> u32 {
  let first = RelocMode::CodeTarget as u32;
  let last = RelocMode::VeneerPool as u32;
  let mut mask = 0u32;
  let mut i = first;
  while i <= last {
    mask |= 1 << i;
    i += 1;
  }
  mask
}

#[inline]
pub const fn apply_mask() -> u32 {
  mode_mask(RelocMode::CodeTarget)
    | mode_mask(RelocMode::NearBuiltinEntry)
    | mode_mask(RelocMode::InternalReference)
    | mode_mask(RelocMode::WasmCall)
    | mode_mask(RelocMode::WasmStubCall)
}

#[inline]
pub const fn post_codegen_relocation_mask() -> u32 {
  mode_mask(RelocMode::CodeTarget)
    | mode_mask(RelocMode::CompressedEmbeddedObject)
    | mode_mask(RelocMode::FullEmbeddedObject)
    | mode_mask(RelocMode::NearBuiltinEntry)
    | mode_mask(RelocMode::WasmStubCall)
    | mode_mask(RelocMode::RelativeCodeTarget)
    | js_dispatch_handle_mode_mask()
    | apply_mask()
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct RelocInfo {
  pub pc: usize,
  pub rmode: RelocMode,
}

impl RelocInfo {
  #[inline]
  pub const fn new(pc: usize, rmode: RelocMode) -> Self {
    Self { pc, rmode }
  }

  #[inline]
  pub fn is_real_reloc_mode(&self) -> bool {
    let mode = self.rmode as u32;
    mode >= RelocMode::CodeTarget as u32 && mode <= RelocMode::VeneerPool as u32
  }

  #[inline]
  pub fn is_code_target(&self) -> bool {
    self.rmode == RelocMode::CodeTarget
  }

  #[inline]
  pub fn is_code_target_mode(&self) -> bool {
    self.rmode == RelocMode::CodeTarget || self.rmode == RelocMode::RelativeCodeTarget
  }

  #[inline]
  pub fn is_embedded_object(&self) -> bool {
    self.rmode == RelocMode::CompressedEmbeddedObject || self.rmode == RelocMode::FullEmbeddedObject
  }

  #[inline]
  pub fn is_full_embedded_object(&self) -> bool {
    self.rmode == RelocMode::FullEmbeddedObject
  }

  #[inline]
  pub fn is_compressed_embedded_object(&self) -> bool {
    self.rmode == RelocMode::CompressedEmbeddedObject
  }

  #[inline]
  pub fn is_wasm_call(&self) -> bool {
    self.rmode == RelocMode::WasmCall
  }

  #[inline]
  pub fn is_shareable_reloc_mode(&self) -> bool {
    self.rmode == RelocMode::WasmCall
  }

  #[inline]
  pub fn is_external_reference(&self) -> bool {
    self.rmode == RelocMode::ExternalReference
  }

  #[inline]
  pub fn is_deopt(&self) -> bool {
    let m = self.rmode as u32;
    m >= RelocMode::DeoptScriptOffset as u32 && m <= RelocMode::DeoptNodeId as u32
  }
}

#[derive(Clone, Debug, Default)]
pub struct RelocInfoWriter {
  records: Vec<RelocInfo>,
}

impl RelocInfoWriter {
  #[inline]
  pub const fn new() -> Self {
    Self {
      records: Vec::new(),
    }
  }

  #[inline]
  pub fn write(&mut self, info: RelocInfo) {
    self.records.push(info);
  }

  #[inline]
  pub fn records(&self) -> &[RelocInfo] {
    &self.records
  }

  pub fn encode(&self) -> Vec<u8> {
    let mut out = Vec::new();
    let mut last_pc = 0usize;
    for r in &self.records {
      let delta = (r.pc.saturating_sub(last_pc)) as u32;
      last_pc = r.pc;
      out.extend_from_slice(&(r.rmode as u32).to_le_bytes());
      out.extend_from_slice(&delta.to_le_bytes());
    }
    out
  }
}

pub struct RelocIterator<'a> {
  records: &'a [RelocInfo],
  mask: u32,
  index: usize,
}

impl<'a> RelocIterator<'a> {
  #[inline]
  pub fn all(records: &'a [RelocInfo]) -> Self {
    Self {
      records,
      mask: u32::MAX,
      index: 0,
    }
  }

  #[inline]
  pub fn new(records: &'a [RelocInfo], mask: u32) -> Self {
    Self {
      records,
      mask,
      index: 0,
    }
  }
}

impl<'a> Iterator for RelocIterator<'a> {
  type Item = &'a RelocInfo;

  fn next(&mut self) -> Option<Self::Item> {
    while self.index < self.records.len() {
      let rec = &self.records[self.index];
      self.index += 1;
      if (mode_mask(rec.rmode) & self.mask) != 0 {
        return Some(rec);
      }
    }
    None
  }
}
