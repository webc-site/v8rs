//! 测试公共件: zone 布局字符串对象竞技场。
//!
//! 布局约定 (与 src/string.rs 一致): 对象内存 = Rust 结构体逻辑布局,
//! Seq 字符尾在 `基址 + CHARS_OFFSET`; 竞技场按 8 字节对齐分配。
//!

use std::{mem::size_of, ptr};

use v8_objects_core::{
  instance_type::InstanceType,
  map::Map,
  name::{EMPTY_HASH_FIELD, Name},
  tagged::Tagged,
};
use v8_objects_strings::string::{
  CHARS_OFFSET, ConsString, ExternalOneByteString, ExternalString, ExternalTwoByteString,
  SlicedString, String as Vstring, ThinString,
};
// 扩展 trait 再导出 (测试文件 use common::* 后方法可直接调用)
pub use v8_objects_strings::string::{
  ConsExt, ExternalExt, SeqOneByteExt, SeqTwoByteExt, SlicedExt, StringExt, ThinExt,
};

/// 8 字节对齐 bump 竞技场 (对象永不回收, 测试期存活)。
///
/// 预留大容量保证 Vec 不重分配 — Tagged 指针指向 buf 内部, 搬迁即悬垂。
pub struct Arena {
  buf: Vec<u64>,
}

/// 预留容量 (字): 128K 字 = 1MB, 超出即断 (测试对象远小于此)。
const ARENA_CAPACITY_WORDS: usize = 1 << 17;

impl Default for Arena {
  fn default() -> Self {
    Self::new()
  }
}

impl Arena {
  pub fn new() -> Self {
    Self {
      buf: Vec::with_capacity(ARENA_CAPACITY_WORDS),
    }
  }

  /// 分配 size 字节 (8 对齐, 零填充), 返回绝对基址。
  fn alloc(&mut self, size: usize) -> usize {
    let words = size.div_ceil(8);
    assert!(
      self.buf.len() + words <= ARENA_CAPACITY_WORDS,
      "竞技场容量不足"
    );
    let base = self.buf.as_ptr() as usize + self.buf.len() * 8;
    self.buf.resize(self.buf.len() + words, 0);
    base
  }

  /// 分配一个指定实例类型的 Map (instance_size 不参与字符串路径)。
  pub fn map(&mut self, ty: InstanceType) -> Tagged<Map> {
    let base = self.alloc(size_of::<Map>());
    let map = Map::new(Tagged::NULL, ty, 0);
    // SAFETY: base 8 对齐且已保留 Map 大小; move 写入
    unsafe { ptr::write(base as *mut Map, map) };
    // SAFETY: base 对齐 (最低 2 位为 0)
    unsafe { Tagged::from_raw_address(base) }
  }

  /// String 头写入 (基址处)。
  fn write_header(&mut self, base: usize, map: Tagged<Map>, length: u32) {
    let header = Vstring {
      name: Name::with_hash_field(map, EMPTY_HASH_FIELD),
      length,
    };
    // SAFETY: base 8 对齐且已保留头大小
    unsafe { ptr::write(base as *mut Vstring, header) };
  }

  /// 分配顺序单字节串。
  pub fn seq_one_byte(&mut self, map: Tagged<Map>, chars: &[u8]) -> Tagged<Vstring> {
    let base = self.alloc(CHARS_OFFSET + chars.len());
    self.write_header(base, map, chars.len() as u32);
    // SAFETY: chars 尾在保留区内
    unsafe {
      ptr::copy_nonoverlapping(
        chars.as_ptr(),
        (base + CHARS_OFFSET) as *mut u8,
        chars.len(),
      );
      Tagged::from_raw_address(base)
    }
  }

  /// 分配顺序双字节串。
  pub fn seq_two_byte(&mut self, map: Tagged<Map>, chars: &[u16]) -> Tagged<Vstring> {
    let base = self.alloc(CHARS_OFFSET + chars.len() * 2);
    self.write_header(base, map, chars.len() as u32);
    // SAFETY: u16 尾在保留区内且基址 8 对齐 (2 对齐满足)
    unsafe {
      ptr::copy_nonoverlapping(
        chars.as_ptr(),
        (base + CHARS_OFFSET) as *mut u16,
        chars.len(),
      );
      Tagged::from_raw_address(base)
    }
  }

  /// 分配拼接串 (length 由调用方给定, 须等于 first+second)。
  pub fn cons(
    &mut self,
    map: Tagged<Map>,
    length: u32,
    first: Tagged<Vstring>,
    second: Tagged<Vstring>,
  ) -> Tagged<Vstring> {
    let base = self.alloc(size_of::<ConsString>());
    let value = ConsString::new(map, length, first, second);
    // SAFETY: base 8 对齐且已保留 ConsString 大小; move 写入
    unsafe {
      ptr::write(base as *mut ConsString, value);
      Tagged::from_raw_address(base)
    }
  }

  /// 分配薄串 (转发 actual)。
  pub fn thin(
    &mut self,
    map: Tagged<Map>,
    length: u32,
    actual: Tagged<Vstring>,
  ) -> Tagged<Vstring> {
    let base = self.alloc(size_of::<ThinString>());
    let value = ThinString::new(map, length, actual);
    // SAFETY: 同上
    unsafe {
      ptr::write(base as *mut ThinString, value);
      Tagged::from_raw_address(base)
    }
  }

  /// 分配切片串。
  pub fn sliced(
    &mut self,
    map: Tagged<Map>,
    length: u32,
    parent: Tagged<Vstring>,
    offset: i32,
  ) -> Tagged<Vstring> {
    let base = self.alloc(size_of::<SlicedString>());
    let value = SlicedString::new(map, length, parent, offset);
    // SAFETY: 同上
    unsafe {
      ptr::write(base as *mut SlicedString, value);
      Tagged::from_raw_address(base)
    }
  }

  /// 分配 cached 单字节外部串 (resource_data 直指静态数据)。
  pub fn external_one_byte(&mut self, map: Tagged<Map>, data: &'static [u8]) -> Tagged<Vstring> {
    let base = self.alloc(size_of::<ExternalOneByteString>());
    let value = ExternalOneByteString {
      external: ExternalString::new(
        map,
        data.len() as u32,
        data.as_ptr() as usize,
        data.as_ptr() as usize,
      ),
    };
    // SAFETY: base 8 对齐且已保留大小; move 写入
    unsafe {
      ptr::write(base as *mut ExternalOneByteString, value);
      Tagged::from_raw_address(base)
    }
  }

  /// 分配 cached 双字节外部串。
  pub fn external_two_byte(&mut self, map: Tagged<Map>, data: &'static [u16]) -> Tagged<Vstring> {
    let base = self.alloc(size_of::<ExternalTwoByteString>());
    let value = ExternalTwoByteString {
      external: ExternalString::new(
        map,
        data.len() as u32,
        data.as_ptr() as usize,
        data.as_ptr() as usize,
      ),
    };
    // SAFETY: 同上
    unsafe {
      ptr::write(base as *mut ExternalTwoByteString, value);
      Tagged::from_raw_address(base)
    }
  }
}

/// 常用 Map 族快捷构造。
pub struct Maps {
  pub seq_one: Tagged<Map>,
  pub seq_two: Tagged<Map>,
  pub cons_one: Tagged<Map>,
  pub cons_two: Tagged<Map>,
  pub sliced_one: Tagged<Map>,
  pub sliced_two: Tagged<Map>,
  pub thin_one: Tagged<Map>,
  pub thin_two: Tagged<Map>,
  pub ext_one: Tagged<Map>,
  pub ext_two: Tagged<Map>,
  pub int_one: Tagged<Map>,
  pub int_two: Tagged<Map>,
}

impl Maps {
  pub fn new(arena: &mut Arena) -> Self {
    use InstanceType::*;
    Self {
      seq_one: arena.map(SeqOneByteString),
      seq_two: arena.map(SeqTwoByteString),
      cons_one: arena.map(ConsOneByteString),
      cons_two: arena.map(ConsTwoByteString),
      sliced_one: arena.map(SlicedOneByteString),
      sliced_two: arena.map(SlicedTwoByteString),
      thin_one: arena.map(ThinOneByteString),
      thin_two: arena.map(ThinTwoByteString),
      ext_one: arena.map(ExternalOneByteString),
      ext_two: arena.map(ExternalTwoByteString),
      int_one: arena.map(InternalizedOneByteString),
      int_two: arena.map(InternalizedTwoByteString),
    }
  }
}
