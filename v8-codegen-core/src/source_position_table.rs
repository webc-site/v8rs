use crate::error::{Error, Result};

pub const K_FUNCTION_ENTRY_BYTECODE_OFFSET: i32 = -1;

/// 源码位置表条目 (对应原版 v8::internal::PositionTableEntry)
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PositionTableEntry {
  pub code_offset: i32,
  pub source_position: i64,
  pub is_statement: bool,
  pub is_breakable: bool,
}

impl Default for PositionTableEntry {
  fn default() -> Self {
    Self {
      code_offset: K_FUNCTION_ENTRY_BYTECODE_OFFSET,
      source_position: 0,
      is_statement: false,
      is_breakable: true,
    }
  }
}

impl PositionTableEntry {
  pub const fn new(
    code_offset: i32,
    source_position: i64,
    is_statement: bool,
    is_breakable: bool,
  ) -> Self {
    Self {
      code_offset,
      source_position,
      is_statement,
      is_breakable,
    }
  }
}

/// 源码位置记录模式
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum RecordingMode {
  OmitSourcePositions,
  LazySourcePositions,
  #[default]
  RecordSourcePositions,
}

// ── VLQ 与 ZigZag 辅助 ───────────────────────────────────────────────────────

fn encode_uint(buf: &mut Vec<u8>, mut value: u32) {
  loop {
    let more = value > 0x7f;
    let byte = ((more as u8) << 7) | ((value & 0x7f) as u8);
    buf.push(byte);
    value >>= 7;
    if !more {
      break;
    }
  }
}

fn decode_uint(bytes: &[u8], index: &mut usize) -> Result<u32> {
  let mut shift: u32 = 0;
  let mut decoded: u32 = 0;
  loop {
    if *index >= bytes.len() {
      return Err(Error::InvalidSourcePositionTable(
        "unexpected EOF in decode_uint",
      ));
    }
    let current = bytes[*index];
    *index += 1;
    decoded |= ((current & 0x7f) as u32) << shift;
    if (current & 0x80) == 0 {
      break;
    }
    shift += 7;
    if shift >= 35 {
      return Err(Error::InvalidSourcePositionTable("VLQ uint overflow"));
    }
  }
  Ok(decoded)
}

fn encode_int(buf: &mut Vec<u8>, value: i64) {
  // Zig-Zag 编码
  let zigzag = ((value << 1) ^ (value >> 63)) as u64;
  let mut val = zigzag;
  loop {
    let more = val > 0x7f;
    let byte = ((more as u8) << 7) | ((val & 0x7f) as u8);
    buf.push(byte);
    val >>= 7;
    if !more {
      break;
    }
  }
}

fn decode_int(bytes: &[u8], index: &mut usize) -> Result<i64> {
  let mut shift: u32 = 0;
  let mut zigzag: u64 = 0;
  loop {
    if *index >= bytes.len() {
      return Err(Error::InvalidSourcePositionTable(
        "unexpected EOF in decode_int",
      ));
    }
    let current = bytes[*index];
    *index += 1;
    zigzag |= ((current & 0x7f) as u64) << shift;
    if (current & 0x80) == 0 {
      break;
    }
    shift += 7;
    if shift >= 70 {
      return Err(Error::InvalidSourcePositionTable("VLQ int overflow"));
    }
  }
  let result = ((zigzag >> 1) as i64) ^ (-((zigzag & 1) as i64));
  Ok(result)
}

// ── SourcePositionTableBuilder ──────────────────────────────────────────────

/// 源码位置表构建器 (对应原版 v8::internal::SourcePositionTableBuilder)
#[derive(Debug)]
pub struct SourcePositionTableBuilder {
  mode: RecordingMode,
  bytes: Vec<u8>,
  previous: PositionTableEntry,
}

impl SourcePositionTableBuilder {
  pub fn new(mode: RecordingMode) -> Self {
    Self {
      mode,
      bytes: Vec::new(),
      previous: PositionTableEntry::default(),
    }
  }

  #[inline]
  pub fn is_omitted(&self) -> bool {
    self.mode != RecordingMode::RecordSourcePositions
  }

  /// 添加一个源码位置条目
  pub fn add_position(
    &mut self,
    code_offset: usize,
    source_position: i64,
    is_statement: bool,
    is_breakable: bool,
  ) {
    if self.is_omitted() {
      return;
    }

    let offset = code_offset as i32;
    let delta_offset =
      if self.bytes.is_empty() && self.previous.code_offset == K_FUNCTION_ENTRY_BYTECODE_OFFSET {
        offset
      } else {
        offset - self.previous.code_offset
      };
    let delta_source = source_position - self.previous.source_position;

    let encoded_entry =
      ((delta_offset as u32) << 3) | ((is_breakable as u32) << 1) | (is_statement as u32);

    encode_uint(&mut self.bytes, encoded_entry);
    encode_int(&mut self.bytes, delta_source);

    self.previous = PositionTableEntry {
      code_offset: offset,
      source_position,
      is_statement,
      is_breakable,
    };
  }

  /// 生成并获取编码后的压缩字节
  pub fn emit(self) -> Vec<u8> {
    self.bytes
  }
}

// ── SourcePositionTableIterator ─────────────────────────────────────────────

/// 源码位置表迭代器 (对应原版 v8::internal::SourcePositionTableIterator)
pub struct SourcePositionTableIterator<'a> {
  bytes: &'a [u8],
  cursor: usize,
  current: PositionTableEntry,
  done: bool,
}

impl<'a> SourcePositionTableIterator<'a> {
  pub fn new(bytes: &'a [u8]) -> Self {
    let mut it = Self {
      bytes,
      cursor: 0,
      current: PositionTableEntry::default(),
      done: false,
    };
    it.advance();
    it
  }

  #[inline]
  pub const fn done(&self) -> bool {
    self.done
  }

  #[inline]
  pub const fn current(&self) -> Option<&PositionTableEntry> {
    if self.done { None } else { Some(&self.current) }
  }

  pub fn advance(&mut self) -> bool {
    if self.cursor >= self.bytes.len() {
      self.done = true;
      return false;
    }

    let encoded_entry = match decode_uint(self.bytes, &mut self.cursor) {
      Ok(v) => v,
      Err(_) => {
        self.done = true;
        return false;
      }
    };

    let delta_offset = (encoded_entry >> 3) as i32;
    let is_breakable = (encoded_entry & 0b010) != 0;
    let is_statement = (encoded_entry & 0b001) != 0;

    let delta_source = match decode_int(self.bytes, &mut self.cursor) {
      Ok(v) => v,
      Err(_) => {
        self.done = true;
        return false;
      }
    };

    if self.current.code_offset == K_FUNCTION_ENTRY_BYTECODE_OFFSET {
      self.current.code_offset = delta_offset;
      self.current.source_position = delta_source;
    } else {
      self.current.code_offset += delta_offset;
      self.current.source_position += delta_source;
    }

    self.current.is_statement = is_statement;
    self.current.is_breakable = is_breakable;

    true
  }
}

impl<'a> Iterator for SourcePositionTableIterator<'a> {
  type Item = PositionTableEntry;

  fn next(&mut self) -> Option<Self::Item> {
    if self.done {
      None
    } else {
      let cur = self.current;
      self.advance();
      Some(cur)
    }
  }
}
