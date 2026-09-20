// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

use std::{cell::RefCell, ffi::c_void, io, io::Write, rc::Rc};

use crate::rt::{AsPointer, Value, rc::Ptr};

unsafe extern "C" {
  #[cfg(target_os = "linux")]
  #[link_name = "malloc_usable_size"]
  fn platform_malloc_size(ptr: *mut c_void) -> usize;

  #[cfg(target_os = "macos")]
  #[link_name = "malloc_size"]
  fn platform_malloc_size(ptr: *const c_void) -> usize;

  #[cfg(target_os = "linux")]
  #[link_name = "__errno_location"]
  fn platform_errno_location() -> *mut i32;

  #[cfg(target_os = "macos")]
  #[link_name = "__error"]
  fn platform_errno_location() -> *mut i32;
}

/// # Safety
///
/// The pointer `ptr` must be a pointer to a block of memory allocated by
/// the appropriate allocator (e.g., `malloc`).
// The memory must not have been deallocated.
pub unsafe fn malloc_usable_size(ptr: *mut c_void) -> usize {
  #[cfg(target_os = "linux")]
  {
    unsafe { platform_malloc_size(ptr) }
  }
  #[cfg(target_os = "macos")]
  {
    unsafe { platform_malloc_size(ptr as *const c_void) }
  }
}

/// # Safety
///
/// Invokes the platform specific errno.
pub unsafe fn cpp2rust_errno_unsafe() -> *mut i32 {
  unsafe { platform_errno_location() }
}

thread_local! {
    static ERRNO: Value<i32> = Rc::new(RefCell::new(0));
}

pub fn cpp2rust_errno() -> Ptr<i32> {
  ERRNO.with(AsPointer::as_pointer)
}

use crate::rt::ByteRepr;

#[allow(non_camel_case_types)]
#[derive(Default, Clone, Copy, Debug, PartialEq, Eq)]
pub struct std_memory_order;

#[allow(non_upper_case_globals)]
pub static memory_order_relaxed: std_memory_order = std_memory_order;
#[allow(non_upper_case_globals)]
pub static memory_order_acquire: std_memory_order = std_memory_order;
#[allow(non_upper_case_globals)]
pub static memory_order_release: std_memory_order = std_memory_order;

#[allow(non_camel_case_types)]
pub type sigaction = ::libc::sigaction;
#[allow(non_camel_case_types)]
pub type __siginfo = ::libc::siginfo_t;

#[allow(non_camel_case_types)]
#[derive(Default, Clone, Copy, Debug, PartialEq, Eq)]
pub struct std_atomic_flag {
  pub val: bool,
}

impl std_atomic_flag {
  pub fn std_atomic_flag1() -> Self {
    Self { val: false }
  }
  pub fn std_atomic_flag1_val<T>(_val: T) -> Self {
    Self { val: false }
  }
  pub fn test_and_set(&self, _order: Option<std_memory_order>) -> bool {
    true
  }
  pub fn clear(&self, _order: Option<std_memory_order>) {}
}

impl ByteRepr for std_atomic_flag {
  fn byte_size() -> usize {
    1
  }
  fn to_bytes(&self, buf: &mut [u8]) {
    buf[0] = self.val as u8;
  }
  fn from_bytes(buf: &[u8]) -> Self {
    Self { val: buf[0] != 0 }
  }
}

#[allow(non_camel_case_types)]
#[derive(Default, Clone, Copy, Debug, PartialEq, Eq)]
pub struct std_atomic_bool_ {
  pub val: bool,
}

impl std_atomic_bool_ {
  pub fn std_atomic_bool_1(val: bool) -> Self {
    Self { val }
  }
  pub fn load(&self, _order: Option<std_memory_order>) -> bool {
    self.val
  }
  pub fn store(&self, _val: bool, _order: Option<std_memory_order>) {}
}

impl ByteRepr for std_atomic_bool_ {
  fn byte_size() -> usize {
    1
  }
  fn to_bytes(&self, buf: &mut [u8]) {
    buf[0] = self.val as u8;
  }
  fn from_bytes(buf: &[u8]) -> Self {
    Self { val: buf[0] != 0 }
  }
}

#[allow(non_camel_case_types)]
#[derive(Default, Clone, Copy, Debug, PartialEq, Eq)]
pub struct std___atomic_base_bool__false_ {
  pub val: bool,
}

impl From<std_atomic_bool_> for std___atomic_base_bool__false_ {
  fn from(s: std_atomic_bool_) -> Self {
    Self { val: s.val }
  }
}

impl std___atomic_base_bool__false_ {
  pub fn store_bool(&self, _val: bool, _order: Option<std_memory_order>) {}
  pub fn load_const(&self, _order: Option<std_memory_order>) -> bool {
    self.val
  }
  pub fn load(&self, _order: Option<std_memory_order>) -> bool {
    self.val
  }
}

impl ByteRepr for std___atomic_base_bool__false_ {
  fn byte_size() -> usize {
    1
  }
  fn to_bytes(&self, buf: &mut [u8]) {
    buf[0] = self.val as u8;
  }
  fn from_bytes(buf: &[u8]) -> Self {
    Self { val: buf[0] != 0 }
  }
}

#[allow(non_camel_case_types)]
#[derive(Default, Clone, Copy, Debug, PartialEq, Eq)]
pub struct std_atomic_unsigned_long_ {
  pub val: u64,
}

impl std_atomic_unsigned_long_ {
  pub fn std_atomic_unsigned_long_1(val: u64) -> Self {
    Self { val }
  }
  pub fn std_atomic_unsigned_long_2(val: u64) -> Self {
    Self { val }
  }
  pub fn load(&self, _order: Option<std_memory_order>) -> u64 {
    self.val
  }
  pub fn load_const(&self, _order: Option<std_memory_order>) -> u64 {
    self.val
  }
  pub fn store(&self, _val: u64, _order: Option<std_memory_order>) {}
  pub fn fetch_add(&self, _val: u64, _order: Option<std_memory_order>) -> u64 {
    self.val
  }
}

impl ByteRepr for std_atomic_unsigned_long_ {
  fn byte_size() -> usize {
    8
  }
  fn to_bytes(&self, buf: &mut [u8]) {
    buf.copy_from_slice(&self.val.to_ne_bytes());
  }
  fn from_bytes(buf: &[u8]) -> Self {
    let mut a = [0u8; 8];
    a.copy_from_slice(buf);
    Self {
      val: u64::from_ne_bytes(a),
    }
  }
}

#[allow(non_camel_case_types)]
#[derive(Default, Clone, Copy, Debug, PartialEq, Eq)]
pub struct std___atomic_base_unsigned_long__false_ {
  pub val: u64,
}

impl From<std_atomic_unsigned_long_> for std___atomic_base_unsigned_long__false_ {
  fn from(s: std_atomic_unsigned_long_) -> Self {
    Self { val: s.val }
  }
}

impl ByteRepr for std___atomic_base_unsigned_long__false_ {
  fn byte_size() -> usize {
    8
  }
  fn to_bytes(&self, buf: &mut [u8]) {
    buf.copy_from_slice(&self.val.to_ne_bytes());
  }
  fn from_bytes(buf: &[u8]) -> Self {
    let mut a = [0u8; 8];
    a.copy_from_slice(buf);
    Self {
      val: u64::from_ne_bytes(a),
    }
  }
}

#[allow(non_camel_case_types)]
#[derive(Default, Clone, Copy, Debug, PartialEq, Eq)]
pub struct std___atomic_base_unsigned_long__true_ {
  pub val: u64,
}

impl From<std_atomic_unsigned_long_> for std___atomic_base_unsigned_long__true_ {
  fn from(s: std_atomic_unsigned_long_) -> Self {
    Self { val: s.val }
  }
}

impl std___atomic_base_unsigned_long__true_ {
  pub fn fetch_add_u64(&self, _val: u64, _order: Option<std_memory_order>) -> u64 {
    self.val
  }
}

impl ByteRepr for std___atomic_base_unsigned_long__true_ {
  fn byte_size() -> usize {
    8
  }
  fn to_bytes(&self, buf: &mut [u8]) {
    buf.copy_from_slice(&self.val.to_ne_bytes());
  }
  fn from_bytes(buf: &[u8]) -> Self {
    let mut a = [0u8; 8];
    a.copy_from_slice(buf);
    Self {
      val: u64::from_ne_bytes(a),
    }
  }
}

// 以下整型原子垫片：类型名由转换器按 std::atomic<T> 的 mangle 规约合成，
// 必须与合名字逐字一致；方法保持 &self 占位语义（与 unsigned_long 先例相同），
// 只保证产物可编译，不模拟真实原子行为。
#[allow(non_camel_case_types)]
#[derive(Default, Clone, Copy, Debug, PartialEq, Eq)]
pub struct std_atomic_unsigned_char_ {
  pub val: u8,
}

impl std_atomic_unsigned_char_ {
  pub fn std_atomic_unsigned_char_1(val: u8) -> Self {
    Self { val }
  }
  pub fn load(&self, _order: Option<std_memory_order>) -> u8 {
    self.val
  }
  pub fn load_const(&self, _order: Option<std_memory_order>) -> u8 {
    self.val
  }
  pub fn store(&self, _val: u8, _order: Option<std_memory_order>) {}
  pub fn fetch_or(&self, _val: u8, _order: Option<std_memory_order>) -> u8 {
    self.val
  }
  pub fn fetch_and(&self, _val: u8, _order: Option<std_memory_order>) -> u8 {
    self.val
  }
  pub fn fetch_xor(&self, _val: u8, _order: Option<std_memory_order>) -> u8 {
    self.val
  }
  pub fn fetch_add(&self, _val: u8, _order: Option<std_memory_order>) -> u8 {
    self.val
  }
  pub fn fetch_sub(&self, _val: u8, _order: Option<std_memory_order>) -> u8 {
    self.val
  }
  pub fn exchange(&self, _val: u8, _order: Option<std_memory_order>) -> u8 {
    self.val
  }
}

impl ByteRepr for std_atomic_unsigned_char_ {
  fn byte_size() -> usize {
    1
  }
  fn to_bytes(&self, buf: &mut [u8]) {
    buf[0] = self.val;
  }
  fn from_bytes(buf: &[u8]) -> Self {
    Self { val: buf[0] }
  }
}

#[allow(non_camel_case_types)]
#[derive(Default, Clone, Copy, Debug, PartialEq, Eq)]
pub struct std___atomic_base_unsigned_char__false_ {
  pub val: u8,
}

impl std___atomic_base_unsigned_char__false_ {
  pub fn load(&self, _order: Option<std_memory_order>) -> u8 {
    self.val
  }
  pub fn load_const(&self, _order: Option<std_memory_order>) -> u8 {
    self.val
  }
  pub fn store(&self, _val: u8, _order: Option<std_memory_order>) {}
  pub fn fetch_or(&self, _val: u8, _order: Option<std_memory_order>) -> u8 {
    self.val
  }
  pub fn fetch_and(&self, _val: u8, _order: Option<std_memory_order>) -> u8 {
    self.val
  }
  pub fn fetch_xor(&self, _val: u8, _order: Option<std_memory_order>) -> u8 {
    self.val
  }
  pub fn fetch_add(&self, _val: u8, _order: Option<std_memory_order>) -> u8 {
    self.val
  }
  pub fn fetch_sub(&self, _val: u8, _order: Option<std_memory_order>) -> u8 {
    self.val
  }
  pub fn exchange(&self, _val: u8, _order: Option<std_memory_order>) -> u8 {
    self.val
  }
}

impl From<std_atomic_unsigned_char_> for std___atomic_base_unsigned_char__false_ {
  fn from(s: std_atomic_unsigned_char_) -> Self {
    Self { val: s.val }
  }
}

impl ByteRepr for std___atomic_base_unsigned_char__false_ {
  fn byte_size() -> usize {
    1
  }
  fn to_bytes(&self, buf: &mut [u8]) {
    buf[0] = self.val;
  }
  fn from_bytes(buf: &[u8]) -> Self {
    Self { val: buf[0] }
  }
}

#[allow(non_camel_case_types)]
#[derive(Default, Clone, Copy, Debug, PartialEq, Eq)]
pub struct std_atomic_int_ {
  pub val: i32,
}

impl std_atomic_int_ {
  pub fn std_atomic_int_1(val: i32) -> Self {
    Self { val }
  }
  pub fn load(&self, _order: Option<std_memory_order>) -> i32 {
    self.val
  }
  pub fn load_const(&self, _order: Option<std_memory_order>) -> i32 {
    self.val
  }
  pub fn store(&self, _val: i32, _order: Option<std_memory_order>) {}
  pub fn fetch_or(&self, _val: i32, _order: Option<std_memory_order>) -> i32 {
    self.val
  }
  pub fn fetch_and(&self, _val: i32, _order: Option<std_memory_order>) -> i32 {
    self.val
  }
  pub fn fetch_xor(&self, _val: i32, _order: Option<std_memory_order>) -> i32 {
    self.val
  }
  pub fn fetch_add(&self, _val: i32, _order: Option<std_memory_order>) -> i32 {
    self.val
  }
  pub fn fetch_sub(&self, _val: i32, _order: Option<std_memory_order>) -> i32 {
    self.val
  }
  pub fn exchange(&self, _val: i32, _order: Option<std_memory_order>) -> i32 {
    self.val
  }
}

impl ByteRepr for std_atomic_int_ {
  fn byte_size() -> usize {
    4
  }
  fn to_bytes(&self, buf: &mut [u8]) {
    buf.copy_from_slice(&self.val.to_ne_bytes());
  }
  fn from_bytes(buf: &[u8]) -> Self {
    let mut a = [0u8; 4];
    a.copy_from_slice(buf);
    Self {
      val: i32::from_ne_bytes(a),
    }
  }
}

#[allow(non_camel_case_types)]
#[derive(Default, Clone, Copy, Debug, PartialEq, Eq)]
pub struct std___atomic_base_int__false_ {
  pub val: i32,
}

impl std___atomic_base_int__false_ {
  pub fn load(&self, _order: Option<std_memory_order>) -> i32 {
    self.val
  }
  pub fn load_const(&self, _order: Option<std_memory_order>) -> i32 {
    self.val
  }
  pub fn store(&self, _val: i32, _order: Option<std_memory_order>) {}
  pub fn fetch_or(&self, _val: i32, _order: Option<std_memory_order>) -> i32 {
    self.val
  }
  pub fn fetch_and(&self, _val: i32, _order: Option<std_memory_order>) -> i32 {
    self.val
  }
  pub fn fetch_xor(&self, _val: i32, _order: Option<std_memory_order>) -> i32 {
    self.val
  }
  pub fn fetch_add(&self, _val: i32, _order: Option<std_memory_order>) -> i32 {
    self.val
  }
  pub fn fetch_sub(&self, _val: i32, _order: Option<std_memory_order>) -> i32 {
    self.val
  }
  pub fn exchange(&self, _val: i32, _order: Option<std_memory_order>) -> i32 {
    self.val
  }
}

impl From<std_atomic_int_> for std___atomic_base_int__false_ {
  fn from(s: std_atomic_int_) -> Self {
    Self { val: s.val }
  }
}

impl ByteRepr for std___atomic_base_int__false_ {
  fn byte_size() -> usize {
    4
  }
  fn to_bytes(&self, buf: &mut [u8]) {
    buf.copy_from_slice(&self.val.to_ne_bytes());
  }
  fn from_bytes(buf: &[u8]) -> Self {
    let mut a = [0u8; 4];
    a.copy_from_slice(buf);
    Self {
      val: i32::from_ne_bytes(a),
    }
  }
}

#[allow(non_camel_case_types)]
#[derive(Default, Clone, Copy, Debug, PartialEq, Eq)]
pub struct std_atomic_unsigned_int_ {
  pub val: u32,
}

impl std_atomic_unsigned_int_ {
  pub fn std_atomic_unsigned_int_1(val: u32) -> Self {
    Self { val }
  }
  pub fn load(&self, _order: Option<std_memory_order>) -> u32 {
    self.val
  }
  pub fn load_const(&self, _order: Option<std_memory_order>) -> u32 {
    self.val
  }
  pub fn store(&self, _val: u32, _order: Option<std_memory_order>) {}
  pub fn fetch_or(&self, _val: u32, _order: Option<std_memory_order>) -> u32 {
    self.val
  }
  pub fn fetch_and(&self, _val: u32, _order: Option<std_memory_order>) -> u32 {
    self.val
  }
  pub fn fetch_xor(&self, _val: u32, _order: Option<std_memory_order>) -> u32 {
    self.val
  }
  pub fn fetch_add(&self, _val: u32, _order: Option<std_memory_order>) -> u32 {
    self.val
  }
  pub fn fetch_sub(&self, _val: u32, _order: Option<std_memory_order>) -> u32 {
    self.val
  }
  pub fn exchange(&self, _val: u32, _order: Option<std_memory_order>) -> u32 {
    self.val
  }
}

impl ByteRepr for std_atomic_unsigned_int_ {
  fn byte_size() -> usize {
    4
  }
  fn to_bytes(&self, buf: &mut [u8]) {
    buf.copy_from_slice(&self.val.to_ne_bytes());
  }
  fn from_bytes(buf: &[u8]) -> Self {
    let mut a = [0u8; 4];
    a.copy_from_slice(buf);
    Self {
      val: u32::from_ne_bytes(a),
    }
  }
}

#[allow(non_camel_case_types)]
#[derive(Default, Clone, Copy, Debug, PartialEq, Eq)]
pub struct std___atomic_base_unsigned_int__false_ {
  pub val: u32,
}

impl std___atomic_base_unsigned_int__false_ {
  pub fn load(&self, _order: Option<std_memory_order>) -> u32 {
    self.val
  }
  pub fn load_const(&self, _order: Option<std_memory_order>) -> u32 {
    self.val
  }
  pub fn store(&self, _val: u32, _order: Option<std_memory_order>) {}
  pub fn fetch_or(&self, _val: u32, _order: Option<std_memory_order>) -> u32 {
    self.val
  }
  pub fn fetch_and(&self, _val: u32, _order: Option<std_memory_order>) -> u32 {
    self.val
  }
  pub fn fetch_xor(&self, _val: u32, _order: Option<std_memory_order>) -> u32 {
    self.val
  }
  pub fn fetch_add(&self, _val: u32, _order: Option<std_memory_order>) -> u32 {
    self.val
  }
  pub fn fetch_sub(&self, _val: u32, _order: Option<std_memory_order>) -> u32 {
    self.val
  }
  pub fn exchange(&self, _val: u32, _order: Option<std_memory_order>) -> u32 {
    self.val
  }
}

impl From<std_atomic_unsigned_int_> for std___atomic_base_unsigned_int__false_ {
  fn from(s: std_atomic_unsigned_int_) -> Self {
    Self { val: s.val }
  }
}

impl ByteRepr for std___atomic_base_unsigned_int__false_ {
  fn byte_size() -> usize {
    4
  }
  fn to_bytes(&self, buf: &mut [u8]) {
    buf.copy_from_slice(&self.val.to_ne_bytes());
  }
  fn from_bytes(buf: &[u8]) -> Self {
    let mut a = [0u8; 4];
    a.copy_from_slice(buf);
    Self {
      val: u32::from_ne_bytes(a),
    }
  }
}

#[allow(non_camel_case_types)]
#[derive(Default, Clone, Copy, Debug, PartialEq, Eq)]
pub struct std_atomic_unsigned_long_long_ {
  pub val: u64,
}

impl std_atomic_unsigned_long_long_ {
  pub fn std_atomic_unsigned_long_long_1(val: u64) -> Self {
    Self { val }
  }
  pub fn load(&self, _order: Option<std_memory_order>) -> u64 {
    self.val
  }
  pub fn load_const(&self, _order: Option<std_memory_order>) -> u64 {
    self.val
  }
  pub fn store(&self, _val: u64, _order: Option<std_memory_order>) {}
  pub fn fetch_or(&self, _val: u64, _order: Option<std_memory_order>) -> u64 {
    self.val
  }
  pub fn fetch_and(&self, _val: u64, _order: Option<std_memory_order>) -> u64 {
    self.val
  }
  pub fn fetch_xor(&self, _val: u64, _order: Option<std_memory_order>) -> u64 {
    self.val
  }
  pub fn fetch_add(&self, _val: u64, _order: Option<std_memory_order>) -> u64 {
    self.val
  }
  pub fn fetch_sub(&self, _val: u64, _order: Option<std_memory_order>) -> u64 {
    self.val
  }
  pub fn exchange(&self, _val: u64, _order: Option<std_memory_order>) -> u64 {
    self.val
  }
}

impl ByteRepr for std_atomic_unsigned_long_long_ {
  fn byte_size() -> usize {
    8
  }
  fn to_bytes(&self, buf: &mut [u8]) {
    buf.copy_from_slice(&self.val.to_ne_bytes());
  }
  fn from_bytes(buf: &[u8]) -> Self {
    let mut a = [0u8; 8];
    a.copy_from_slice(buf);
    Self {
      val: u64::from_ne_bytes(a),
    }
  }
}

#[allow(non_camel_case_types)]
#[derive(Default, Clone, Copy, Debug, PartialEq, Eq)]
pub struct std___atomic_base_unsigned_long_long__false_ {
  pub val: u64,
}

impl std___atomic_base_unsigned_long_long__false_ {
  pub fn load(&self, _order: Option<std_memory_order>) -> u64 {
    self.val
  }
  pub fn load_const(&self, _order: Option<std_memory_order>) -> u64 {
    self.val
  }
  pub fn store(&self, _val: u64, _order: Option<std_memory_order>) {}
  pub fn fetch_or(&self, _val: u64, _order: Option<std_memory_order>) -> u64 {
    self.val
  }
  pub fn fetch_and(&self, _val: u64, _order: Option<std_memory_order>) -> u64 {
    self.val
  }
  pub fn fetch_xor(&self, _val: u64, _order: Option<std_memory_order>) -> u64 {
    self.val
  }
  pub fn fetch_add(&self, _val: u64, _order: Option<std_memory_order>) -> u64 {
    self.val
  }
  pub fn fetch_sub(&self, _val: u64, _order: Option<std_memory_order>) -> u64 {
    self.val
  }
  pub fn exchange(&self, _val: u64, _order: Option<std_memory_order>) -> u64 {
    self.val
  }
}

impl From<std_atomic_unsigned_long_long_> for std___atomic_base_unsigned_long_long__false_ {
  fn from(s: std_atomic_unsigned_long_long_) -> Self {
    Self { val: s.val }
  }
}

impl ByteRepr for std___atomic_base_unsigned_long_long__false_ {
  fn byte_size() -> usize {
    8
  }
  fn to_bytes(&self, buf: &mut [u8]) {
    buf.copy_from_slice(&self.val.to_ne_bytes());
  }
  fn from_bytes(buf: &[u8]) -> Self {
    let mut a = [0u8; 8];
    a.copy_from_slice(buf);
    Self {
      val: u64::from_ne_bytes(a),
    }
  }
}

#[allow(non_camel_case_types)]
#[derive(Default, Clone, Copy, Debug, PartialEq, Eq)]
pub struct std___atomic_base_unsigned_char__true_ {
  pub val: u8,
}

impl From<std_atomic_unsigned_char_> for std___atomic_base_unsigned_char__true_ {
  fn from(s: std_atomic_unsigned_char_) -> Self {
    Self { val: s.val }
  }
}

impl ByteRepr for std___atomic_base_unsigned_char__true_ {
  fn byte_size() -> usize {
    1
  }
  fn to_bytes(&self, buf: &mut [u8]) {
    buf[0] = self.val;
  }
  fn from_bytes(buf: &[u8]) -> Self {
    Self { val: buf[0] }
  }
}

#[allow(non_camel_case_types)]
#[derive(Default, Clone, Copy, Debug, PartialEq, Eq)]
pub struct std___atomic_base_int__true_ {
  pub val: i32,
}

impl From<std_atomic_int_> for std___atomic_base_int__true_ {
  fn from(s: std_atomic_int_) -> Self {
    Self { val: s.val }
  }
}

impl ByteRepr for std___atomic_base_int__true_ {
  fn byte_size() -> usize {
    4
  }
  fn to_bytes(&self, buf: &mut [u8]) {
    buf.copy_from_slice(&self.val.to_ne_bytes());
  }
  fn from_bytes(buf: &[u8]) -> Self {
    let mut a = [0u8; 4];
    a.copy_from_slice(buf);
    Self {
      val: i32::from_ne_bytes(a),
    }
  }
}

#[allow(non_camel_case_types)]
#[derive(Default, Clone, Copy, Debug, PartialEq, Eq)]
pub struct std___atomic_base_unsigned_int__true_ {
  pub val: u32,
}

impl From<std_atomic_unsigned_int_> for std___atomic_base_unsigned_int__true_ {
  fn from(s: std_atomic_unsigned_int_) -> Self {
    Self { val: s.val }
  }
}

impl ByteRepr for std___atomic_base_unsigned_int__true_ {
  fn byte_size() -> usize {
    4
  }
  fn to_bytes(&self, buf: &mut [u8]) {
    buf.copy_from_slice(&self.val.to_ne_bytes());
  }
  fn from_bytes(buf: &[u8]) -> Self {
    let mut a = [0u8; 4];
    a.copy_from_slice(buf);
    Self {
      val: u32::from_ne_bytes(a),
    }
  }
}

#[allow(non_camel_case_types)]
#[derive(Default, Clone, Debug, PartialEq, Eq)]
pub struct std_basic_stringstream_char__std_char_traits_char___std_allocator_char__ {
  pub buf: Vec<u8>,
}

#[allow(non_snake_case)]
impl std_basic_stringstream_char__std_char_traits_char___std_allocator_char__ {
  pub fn std_basic_stringstream_char__std_char_traits_char___std_allocator_char__1() -> Self {
    Self { buf: Vec::new() }
  }
  pub fn str_const_lref(&self) -> Vec<u8> {
    self.buf.clone()
  }
}

impl Write for std_basic_stringstream_char__std_char_traits_char___std_allocator_char__ {
  fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
    self.buf.extend_from_slice(buf);
    Ok(buf.len())
  }
  fn flush(&mut self) -> io::Result<()> {
    Ok(())
  }
}

impl ByteRepr for std_basic_stringstream_char__std_char_traits_char___std_allocator_char__ {
  fn byte_size() -> usize {
    1
  }
}
