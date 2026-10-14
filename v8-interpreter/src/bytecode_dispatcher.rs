use std::{
  borrow::Cow,
  fmt,
  mem::{replace, take},
  ptr::fn_addr_eq,
  sync::{Arc, LazyLock},
};

use gxhash::HashMap;
use itoa::Buffer as ItoaBuffer;
use parking_lot::RwLock;
use v8_numbers::{double_to_int32, double_to_uint32};

use crate::{
  bytecode_array_builder::{BytecodeArray, ConstantPoolEntry, InterpFunction},
  bytecode_array_iterator::BytecodeArrayIterator,
  bytecode_register::Register,
  bytecodes::{Bytecode, Bytecodes},
  error::{Error, Result},
};

pub const PROTO_KEY: &str = "__proto__";
pub const PROTOTYPE_KEY: &str = "prototype";
pub const LENGTH_KEY: &str = "length";
pub const NAME_KEY: &str = "name";
pub const CALL_KEY: &str = "call";
pub const APPLY_KEY: &str = "apply";
pub const PUSH_KEY: &str = "push";
pub const POP_KEY: &str = "pop";
pub const SHIFT_KEY: &str = "shift";
pub const SLICE_KEY: &str = "slice";
pub const JOIN_KEY: &str = "join";
pub const INDEX_OF_KEY: &str = "indexOf";
pub const SPLICE_KEY: &str = "splice";
pub const REVERSE_KEY: &str = "reverse";
pub const EXEC_KEY: &str = "exec";
pub const TEST_KEY: &str = "test";
pub const REPLACE_KEY: &str = "replace";
pub const SPLIT_KEY: &str = "split";
pub const MATCH_KEY: &str = "match";
pub const CHAR_AT_KEY: &str = "charAt";
pub const CHAR_CODE_AT_KEY: &str = "charCodeAt";
pub const SUBSTRING_KEY: &str = "substring";
pub const TO_LOWER_CASE_KEY: &str = "toLowerCase";
pub const TO_UPPER_CASE_KEY: &str = "toUpperCase";
pub const VALUE_OF_KEY: &str = "valueOf";
pub const TO_STRING_KEY: &str = "toString";
pub const GLOBAL_KEY: &str = "global";
pub const LAST_INDEX_KEY: &str = "lastIndex";
pub const SOURCE_KEY: &str = "source";
pub const FLAGS_KEY: &str = "flags";
pub const INDEX_KEY: &str = "index";
pub const INPUT_KEY: &str = "input";

static FUNCTION_CALL: LazyLock<Value> = LazyLock::new(|| Value::new_native(NativeFn::FunctionCall));

static FUNCTION_APPLY: LazyLock<Value> =
  LazyLock::new(|| Value::new_native(NativeFn::FunctionApply));

/// 字符索引转字节偏移 (字符边界安全，杜绝切片 panic)
#[inline]
fn char_to_byte_index(s: &str, char_idx: usize) -> usize {
  if s.is_ascii() {
    return char_idx.min(s.len());
  }
  s.char_indices()
    .nth(char_idx)
    .map(|(i, _)| i)
    .unwrap_or(s.len())
}

/// 计算字符数 (ASCII 快速路径)
#[inline]
fn char_count(s: &str) -> usize {
  if s.is_ascii() {
    s.len()
  } else {
    s.chars().count()
  }
}

/// 原生上下文状态载荷 (零 dyn 泛型擦除，极速函数指针分派)
pub struct NativeContext {
  payload: *const (),
  call_fn: fn(*const (), &[Value]) -> Result<Value>,
  drop_fn: fn(*const ()),
}

unsafe impl Send for NativeContext {}
unsafe impl Sync for NativeContext {}

impl Drop for NativeContext {
  fn drop(&mut self) {
    (self.drop_fn)(self.payload);
  }
}

/// 原生函数句柄枚举分发 (纯 Enum Dispatch，杜绝 dyn 虚表与闭包堆分配)
#[derive(Clone)]
pub enum NativeFn {
  Ptr(fn(&[Value]) -> Result<Value>),
  Noop,
  FunctionCall,
  FunctionApply,
  ArrayPush(Arc<RwLock<Vec<Value>>>),
  ArrayPop(Arc<RwLock<Vec<Value>>>),
  ArrayShift(Arc<RwLock<Vec<Value>>>),
  ArraySlice(Arc<RwLock<Vec<Value>>>),
  ArrayJoin(Arc<RwLock<Vec<Value>>>),
  ArrayIndexOf(Arc<RwLock<Vec<Value>>>),
  StringCharCodeAt(String),
  StringCharAt(String),
  StringIndexOf(String),
  StringSubstring(String),
  StringSplit(String),
  StringReplace(String),
  StringMatch(String),
  StringToLowerCase(String),
  StringToUpperCase(String),
  NumberToString(f64),
  ConstString(String),
  Context(Arc<NativeContext>),
}

impl PartialEq for NativeFn {
  fn eq(&self, other: &Self) -> bool {
    match (self, other) {
      (Self::Ptr(a), Self::Ptr(b)) => fn_addr_eq(*a, *b),
      (Self::Noop, Self::Noop) => true,
      (Self::FunctionCall, Self::FunctionCall) => true,
      (Self::FunctionApply, Self::FunctionApply) => true,
      (Self::NumberToString(a), Self::NumberToString(b)) => a == b,
      (Self::ConstString(a), Self::ConstString(b)) => a == b,
      (Self::ArrayPush(a), Self::ArrayPush(b))
      | (Self::ArrayPop(a), Self::ArrayPop(b))
      | (Self::ArrayShift(a), Self::ArrayShift(b))
      | (Self::ArraySlice(a), Self::ArraySlice(b))
      | (Self::ArrayJoin(a), Self::ArrayJoin(b))
      | (Self::ArrayIndexOf(a), Self::ArrayIndexOf(b)) => Arc::ptr_eq(a, b),
      (Self::StringCharCodeAt(a), Self::StringCharCodeAt(b))
      | (Self::StringCharAt(a), Self::StringCharAt(b))
      | (Self::StringIndexOf(a), Self::StringIndexOf(b))
      | (Self::StringSubstring(a), Self::StringSubstring(b))
      | (Self::StringSplit(a), Self::StringSplit(b))
      | (Self::StringReplace(a), Self::StringReplace(b))
      | (Self::StringMatch(a), Self::StringMatch(b))
      | (Self::StringToLowerCase(a), Self::StringToLowerCase(b))
      | (Self::StringToUpperCase(a), Self::StringToUpperCase(b)) => a == b,
      (Self::Context(a), Self::Context(b)) => Arc::ptr_eq(a, b),
      _ => false,
    }
  }
}

impl NativeFn {
  pub fn from_context<T: Send + Sync + 'static>(
    ctx: T,
    func: fn(&T, &[Value]) -> Result<Value>,
  ) -> Self {
    struct Payload<T> {
      ctx: T,
      func: fn(&T, &[Value]) -> Result<Value>,
    }
    let boxed = Box::into_raw(Box::new(Payload { ctx, func })) as *const ();
    fn call_trampoline<T>(raw: *const (), args: &[Value]) -> Result<Value> {
      let p = unsafe { &*(raw as *const Payload<T>) };
      (p.func)(&p.ctx, args)
    }
    fn drop_trampoline<T>(raw: *const ()) {
      unsafe {
        let _ = Box::from_raw(raw as *mut Payload<T>);
      }
    }
    Self::Context(Arc::new(NativeContext {
      payload: boxed,
      call_fn: call_trampoline::<T>,
      drop_fn: drop_trampoline::<T>,
    }))
  }

  pub fn call(&self, args: &[Value]) -> Result<Value> {
    match self {
      Self::Ptr(f) => f(args),
      Self::Noop | Self::FunctionCall | Self::FunctionApply => Ok(Value::Undefined),
      Self::ConstString(s) => Ok(Value::String(s.clone())),
      Self::NumberToString(n) => {
        if let Some(r_val) = args.first() {
          let r = r_val.to_i32();
          if (2..=36).contains(&r) && r != 10 && n.fract() == 0.0 {
            let i = *n as i64;
            let mut s = String::new();
            let mut val = i.unsigned_abs();
            let base = r as u64;
            if val == 0 {
              s.push('0');
            } else {
              while val > 0 {
                let rem = (val % base) as u32;
                val /= base;
                let ch = if rem < 10 {
                  (b'0' + rem as u8) as char
                } else {
                  (b'a' + (rem - 10) as u8) as char
                };
                s.push(ch);
              }
              if i < 0 {
                s.push('-');
              }
              s = s.chars().rev().collect();
            }
            return Ok(Value::String(s));
          }
        }
        let s = if n.fract() == 0.0 && *n >= (i64::MIN as f64) && *n <= (i64::MAX as f64) {
          let mut buf = ItoaBuffer::new();
          buf.format(*n as i64).to_string()
        } else {
          n.to_string()
        };
        Ok(Value::String(s))
      }
      Self::ArrayPush(arr) => {
        let mut guard = arr.write();
        guard.extend(args.iter().cloned());
        Ok(Value::Smi(guard.len() as i32))
      }
      Self::ArrayPop(arr) => Ok(arr.write().pop().unwrap_or(Value::Undefined)),
      Self::ArrayShift(arr) => {
        let mut guard = arr.write();
        if guard.is_empty() {
          Ok(Value::Undefined)
        } else {
          Ok(guard.remove(0))
        }
      }
      Self::ArraySlice(arr) => {
        let guard = arr.read();
        let len = guard.len();
        let start = if let Some(s) = args.first() {
          let n = s.to_i32();
          if n < 0 {
            (len as i32 + n).max(0) as usize
          } else {
            (n as usize).min(len)
          }
        } else {
          0
        };
        let end = if let Some(e) = args.get(1)
          && !matches!(e, Value::Undefined)
        {
          let n = e.to_i32();
          if n < 0 {
            (len as i32 + n).max(0) as usize
          } else {
            (n as usize).min(len)
          }
        } else {
          len
        };
        let sliced = if start < end {
          guard[start..end].to_vec()
        } else {
          Vec::new()
        };
        Ok(Value::new_array_with_elements(sliced))
      }
      Self::ArrayJoin(arr) => {
        let sep = args
          .first()
          .map(Value::to_str_lossy)
          .unwrap_or(Cow::Borrowed(","));
        let guard = arr.read();
        let mut res = String::new();
        for (i, v) in guard.iter().enumerate() {
          if i > 0 {
            res.push_str(&sep);
          }
          res.push_str(&v.to_str_lossy());
        }
        Ok(Value::String(res))
      }
      Self::ArrayIndexOf(arr) => {
        let target = args.first().unwrap_or(&Value::Undefined);
        let from_idx = args.get(1).map(|v| v.to_i32().max(0) as usize).unwrap_or(0);
        let guard = arr.read();
        let idx = guard
          .iter()
          .enumerate()
          .skip(from_idx)
          .find(|(_, v)| v.test_equal_strict(target))
          .map(|(i, _)| i as i32)
          .unwrap_or(-1);
        Ok(Value::Smi(idx))
      }
      Self::StringCharCodeAt(string) => {
        let idx = args.first().map(|v| v.to_u32() as usize).unwrap_or(0);
        let code = if string.is_ascii() {
          string.as_bytes().get(idx).map(|&b| b as u32)
        } else {
          string.chars().nth(idx).map(|ch| ch as u32)
        };
        if let Some(ch) = code {
          Ok(Value::Smi(ch as i32))
        } else {
          Ok(Value::Number(f64::NAN))
        }
      }
      Self::StringCharAt(string) => {
        let idx = args.first().map(|v| v.to_u32() as usize).unwrap_or(0);
        if string.is_ascii() {
          if let Some(&b) = string.as_bytes().get(idx) {
            return Ok(Value::String((b as char).to_string()));
          }
        } else if let Some(ch) = string.chars().nth(idx) {
          return Ok(Value::String(ch.to_string()));
        }
        Ok(Value::String(String::new()))
      }
      Self::StringIndexOf(string) => {
        let search = args
          .first()
          .map(Value::to_str_lossy)
          .unwrap_or(Cow::Borrowed(""));
        let from_char = args.get(1).map(|v| v.to_u32() as usize).unwrap_or(0);
        let from_b = char_to_byte_index(string, from_char);
        let idx = if from_b < string.len() {
          string[from_b..]
            .find(&*search)
            .map(|b_offset| {
              let match_byte = from_b + b_offset;
              if string.is_ascii() {
                match_byte as i32
              } else {
                char_count(&string[..match_byte]) as i32
              }
            })
            .unwrap_or(-1)
        } else {
          -1
        };
        Ok(Value::Smi(idx))
      }
      Self::StringSubstring(string) => {
        let c_len = char_count(string) as i64;
        let start = args
          .first()
          .map(|v| {
            let n = v.to_i32() as i64;
            if n < 0 {
              (c_len + n).max(0) as usize
            } else {
              (n as usize).min(c_len as usize)
            }
          })
          .unwrap_or(0);
        let end = args
          .get(1)
          .filter(|v| !matches!(v, Value::Undefined))
          .map(|v| {
            let n = v.to_i32() as i64;
            if n < 0 {
              (c_len + n).max(0) as usize
            } else {
              (n as usize).min(c_len as usize)
            }
          })
          .unwrap_or(c_len as usize);
        let (from_c, to_c) = if start <= end {
          (start, end)
        } else {
          (end, start)
        };
        let from_b = char_to_byte_index(string, from_c);
        let to_b = char_to_byte_index(string, to_c);
        let sub = if from_b <= to_b && from_b < string.len() {
          &string[from_b..to_b.min(string.len())]
        } else {
          ""
        };
        Ok(Value::String(sub.to_string()))
      }
      Self::StringSplit(string) => Value::string_split_inlined(string, args),
      Self::StringReplace(string) => Value::string_replace_inlined(string, args),
      Self::StringMatch(string) => Value::string_match_inlined(string, args),
      Self::StringToLowerCase(string) => Ok(Value::String(string.to_lowercase())),
      Self::StringToUpperCase(string) => Ok(Value::String(string.to_uppercase())),
      Self::Context(ctx) => (ctx.call_fn)(ctx.payload, args),
    }
  }
}

/// 解释器运行时值体系 (真实 Accumulator 与 Register 寄存器值)
#[derive(Clone, Default)]
pub enum Value {
  #[default]
  Undefined,
  Null,
  TheHole,
  Boolean(bool),
  Smi(i32),
  Number(f64),
  String(String),
  BigInt(String),
  Array(Arc<RwLock<Vec<Value>>>),
  Object(Arc<RwLock<HashMap<String, Value>>>),
  Function(Arc<InterpFunction>, Arc<RwLock<HashMap<String, Value>>>),
  Native(NativeFn, Arc<RwLock<HashMap<String, Value>>>),
}

impl PartialEq for Value {
  fn eq(&self, other: &Self) -> bool {
    match (self, other) {
      (Self::Undefined, Self::Undefined)
      | (Self::Null, Self::Null)
      | (Self::TheHole, Self::TheHole) => true,
      (Self::Boolean(a), Self::Boolean(b)) => a == b,
      (Self::Smi(a), Self::Smi(b)) => a == b,
      (Self::Number(a), Self::Number(b)) => a == b || (a.is_nan() && b.is_nan()),
      (Self::Smi(a), Self::Number(b)) | (Self::Number(b), Self::Smi(a)) => (*a as f64) == *b,
      (Self::String(a), Self::String(b)) => a == b,
      (Self::BigInt(a), Self::BigInt(b)) => a == b,
      (Self::Array(a), Self::Array(b)) => Arc::ptr_eq(a, b),
      (Self::Object(a), Self::Object(b)) => Arc::ptr_eq(a, b),
      (Self::Function(a, _), Self::Function(b, _)) => Arc::ptr_eq(a, b),
      (Self::Native(a, _), Self::Native(b, _)) => a == b,
      _ => false,
    }
  }
}

impl fmt::Debug for Value {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Self::Undefined => write!(f, "Undefined"),
      Self::Null => write!(f, "Null"),
      Self::TheHole => write!(f, "TheHole"),
      Self::Boolean(b) => write!(f, "Boolean({b})"),
      Self::Smi(n) => write!(f, "Smi({n})"),
      Self::Number(n) => write!(f, "Number({n})"),
      Self::String(s) => write!(f, "String({s:?})"),
      Self::BigInt(s) => write!(f, "BigInt({s})"),
      Self::Array(arr) => write!(f, "Array({:?})", *arr.read()),
      Self::Object(obj) => write!(f, "Object({:?})", *obj.read()),
      Self::Function(func, _) => write!(f, "Function({:?})", func),
      Self::Native(..) => write!(f, "NativeFunction"),
    }
  }
}

impl Value {
  /// 构造空数组
  #[inline]
  #[must_use]
  pub fn new_array() -> Self {
    Self::Array(Arc::new(RwLock::new(Vec::new())))
  }

  /// 构造已知元素数组
  #[inline]
  #[must_use]
  pub fn new_array_with_elements(elems: Vec<Self>) -> Self {
    Self::Array(Arc::new(RwLock::new(elems)))
  }

  /// 构造空对象
  #[inline]
  #[must_use]
  pub fn new_object() -> Self {
    Self::Object(Arc::new(RwLock::new(HashMap::default())))
  }

  /// 构造函数对象 (自动附加 prototype 属性对象)
  #[inline]
  #[must_use]
  pub fn new_function(func: Arc<InterpFunction>) -> Self {
    let mut props = HashMap::default();
    let proto = Self::new_object();
    props.insert("prototype".to_string(), proto);
    Self::Function(func, Arc::new(RwLock::new(props)))
  }

  pub fn string_split_inlined(string: &str, args: &[Self]) -> Result<Self> {
    let sep = args.first().unwrap_or(&Self::Undefined);
    if matches!(sep, Self::Object(_))
      && let Ok(split_val) = sep.get_property(SPLIT_KEY)
      && let Self::Native(split_fn, _) = split_val
    {
      return split_fn.call(&[
        Self::String(string.to_string()),
        args.get(1).cloned().unwrap_or(Self::Undefined),
      ]);
    }
    if let Ok(exec_val) = sep.get_property(EXEC_KEY)
      && let Self::Native(exec_fn, _) = exec_val
    {
      let _ = sep.set_property(LAST_INDEX_KEY, Self::Smi(0));
      let mut parts = Vec::new();
      let mut last_end = 0;
      let mut cur_search_pos = 0;
      let text_val = Self::String(string.to_string());

      loop {
        if cur_search_pos > string.len() {
          break;
        }
        let match_res = exec_fn.call(&[text_val.clone(), Self::Smi(cur_search_pos as i32)])?;
        if let Self::Array(arr) = &match_res {
          let guard = arr.read();
          let match_str = guard.first().map(Self::to_string_lossy).unwrap_or_default();
          let start_idx = match match_res.get_property(INDEX_KEY) {
            Ok(Self::Smi(i)) => i.max(0) as usize,
            Ok(Self::Number(n)) => n.max(0.0) as usize,
            _ => cur_search_pos,
          };
          let end_idx = start_idx + match_str.len();

          let piece = if start_idx >= last_end && start_idx <= string.len() {
            &string[last_end..start_idx]
          } else {
            ""
          };
          parts.push(Self::String(piece.to_string()));
          last_end = end_idx;

          if match_str.is_empty() {
            cur_search_pos = start_idx + 1;
          } else {
            cur_search_pos = end_idx.max(cur_search_pos + 1);
          }
        } else {
          break;
        }
      }
      let remainder = if last_end <= string.len() {
        &string[last_end..]
      } else {
        ""
      };
      parts.push(Self::String(remainder.to_string()));
      let _ = sep.set_property(LAST_INDEX_KEY, Self::Smi(0));
      return Ok(Self::new_array_with_elements(parts));
    }

    let sep_str = sep.to_str_lossy();
    let parts: Vec<Self> = if sep_str.is_empty() {
      string
        .chars()
        .map(|c| Self::String(c.to_string()))
        .collect()
    } else {
      string
        .split(&*sep_str)
        .map(|s| Self::String(s.to_string()))
        .collect()
    };
    Ok(Self::new_array_with_elements(parts))
  }

  pub fn string_replace_inlined(string: &str, args: &[Self]) -> Result<Self> {
    let pattern = args.first().unwrap_or(&Self::Undefined);
    let replacement = args.get(1).map(Self::to_string_lossy).unwrap_or_default();

    if matches!(pattern, Self::Object(_))
      && let Ok(rep_val) = pattern.get_property(REPLACE_KEY)
      && let Self::Native(rep_fn, _) = rep_val
    {
      return rep_fn.call(&[Self::String(string.to_string()), Self::String(replacement)]);
    }

    if let Ok(exec_val) = pattern.get_property(EXEC_KEY)
      && let Self::Native(exec_fn, _) = exec_val
    {
      let is_global = pattern
        .get_property(GLOBAL_KEY)
        .map(|v| v.to_boolean())
        .unwrap_or(false);
      let _ = pattern.set_property(LAST_INDEX_KEY, Self::Smi(0));

      if is_global {
        let mut out = String::new();
        let mut last_end = 0;
        let mut cur_search_pos = 0;
        let text_val = Self::String(string.to_string());

        loop {
          if cur_search_pos > string.len() {
            break;
          }
          let match_res = exec_fn.call(&[text_val.clone(), Self::Smi(cur_search_pos as i32)])?;
          if let Self::Array(arr) = &match_res {
            let guard = arr.read();
            let match_str = guard.first().map(Self::to_string_lossy).unwrap_or_default();
            let start_idx = match match_res.get_property(INDEX_KEY) {
              Ok(Self::Smi(i)) => i.max(0) as usize,
              Ok(Self::Number(n)) => n.max(0.0) as usize,
              _ => cur_search_pos,
            };
            let end_idx = start_idx + match_str.len();

            if start_idx >= last_end && start_idx <= string.len() {
              out.push_str(&string[last_end..start_idx]);
            }
            out.push_str(&replacement);
            last_end = end_idx;

            if match_str.is_empty() {
              cur_search_pos = start_idx + 1;
            } else {
              cur_search_pos = end_idx.max(cur_search_pos + 1);
            }
          } else {
            break;
          }
        }
        if last_end < string.len() {
          out.push_str(&string[last_end..]);
        }
        let _ = pattern.set_property(LAST_INDEX_KEY, Self::Smi(0));
        Ok(Self::String(out))
      } else {
        let match_res = exec_fn.call(&[Self::String(string.to_string()), Self::Smi(0)])?;
        if let Self::Array(arr) = &match_res {
          let guard = arr.read();
          let match_str = guard.first().map(Self::to_string_lossy).unwrap_or_default();
          let start_idx = match match_res.get_property(INDEX_KEY) {
            Ok(Self::Smi(i)) => i.max(0) as usize,
            Ok(Self::Number(n)) => n.max(0.0) as usize,
            _ => 0,
          };
          let end_idx = start_idx + match_str.len();
          let mut out = String::new();
          if start_idx <= string.len() {
            out.push_str(&string[..start_idx]);
          }
          out.push_str(&replacement);
          if end_idx <= string.len() {
            out.push_str(&string[end_idx..]);
          }
          Ok(Self::String(out))
        } else {
          Ok(Self::String(string.to_string()))
        }
      }
    } else {
      let pat_str = pattern.to_str_lossy();
      if let Some(pos) = string.find(&*pat_str) {
        let mut out = String::with_capacity(string.len() + replacement.len());
        out.push_str(&string[..pos]);
        out.push_str(&replacement);
        out.push_str(&string[pos + pat_str.len()..]);
        Ok(Self::String(out))
      } else {
        Ok(Self::String(string.to_string()))
      }
    }
  }

  pub fn string_match_inlined(string: &str, args: &[Self]) -> Result<Self> {
    let pat = args.first().unwrap_or(&Self::Undefined);
    if matches!(pat, Self::Object(_))
      && let Ok(m_val) = pat.get_property(MATCH_KEY)
      && let Self::Native(m_fn, _) = m_val
    {
      return m_fn.call(&[Self::String(string.to_string())]);
    }
    if let Ok(exec_val) = pat.get_property(EXEC_KEY)
      && let Self::Native(exec_fn, _) = exec_val
    {
      let is_global = pat
        .get_property(GLOBAL_KEY)
        .map(|v| v.to_boolean())
        .unwrap_or(false);
      let text_val = Self::String(string.to_string());
      let _ = pat.set_property(LAST_INDEX_KEY, Self::Smi(0));

      if is_global {
        let mut results = Vec::new();
        let mut cur_search_pos = 0;
        loop {
          if cur_search_pos > string.len() {
            break;
          }
          let match_res = exec_fn.call(&[text_val.clone(), Self::Smi(cur_search_pos as i32)])?;
          if let Self::Array(arr) = &match_res {
            let guard = arr.read();
            let match_str = guard.first().map(Self::to_string_lossy).unwrap_or_default();
            let start_idx = match match_res.get_property(INDEX_KEY) {
              Ok(Self::Smi(i)) => i.max(0) as usize,
              Ok(Self::Number(n)) => n.max(0.0) as usize,
              _ => cur_search_pos,
            };
            let end_idx = start_idx + match_str.len();

            results.push(Self::String(match_str.clone()));

            if match_str.is_empty() {
              cur_search_pos = start_idx + 1;
            } else {
              cur_search_pos = end_idx;
            }
          } else {
            break;
          }
        }
        let _ = pat.set_property(LAST_INDEX_KEY, Self::Smi(0));
        if results.is_empty() {
          Ok(Self::Null)
        } else {
          Ok(Self::new_array_with_elements(results))
        }
      } else {
        exec_fn.call(&[text_val, Self::Smi(0)])
      }
    } else {
      Ok(Self::Null)
    }
  }

  pub fn new_native(native: NativeFn) -> Self {
    let mut props = HashMap::default();
    let proto = Self::new_object();
    props.insert(PROTOTYPE_KEY.to_string(), proto);
    Self::Native(native, Arc::new(RwLock::new(props)))
  }

  /// 构造零额外原型分配的原生函数句柄
  #[inline]
  #[must_use]
  pub fn new_native_bare(native: NativeFn) -> Self {
    Self::Native(native, Arc::new(RwLock::new(HashMap::default())))
  }

  /// 构造带自有属性的原生函数对象
  #[inline]
  #[must_use]
  pub fn new_native_with_props(
    native: NativeFn,
    props: Arc<RwLock<HashMap<String, Value>>>,
  ) -> Self {
    Self::Native(native, props)
  }

  /// 获取 JavaScript typeof 字符串
  #[inline]
  #[must_use]
  pub fn type_of(&self) -> &'static str {
    match self {
      Self::Undefined | Self::TheHole => "undefined",
      Self::Null => "object",
      Self::Boolean(_) => "boolean",
      Self::Smi(_) | Self::Number(_) => "number",
      Self::String(_) => "string",
      Self::BigInt(_) => "bigint",
      Self::Function(..) | Self::Native(..) => "function",
      Self::Array(_) | Self::Object(_) => "object",
    }
  }

  /// 获取命名/键控属性
  #[inline]
  pub fn get_property(&self, prop: &str) -> Result<Self> {
    self.get_property_depth(prop, 0)
  }

  /// 带深度限制的原型链属性查找 (最大 64 层，彻底杜绝环形原型链导致的栈溢出)
  pub fn get_property_depth(&self, prop: &str, depth: usize) -> Result<Self> {
    if depth > 64 {
      return Ok(Self::Undefined);
    }
    match self {
      Self::Object(obj) => {
        let guard = obj.read();
        if let Some(val) = guard.get(prop) {
          return Ok(val.clone());
        }
        if let Some(proto) = guard.get(PROTO_KEY) {
          return proto.get_property_depth(prop, depth + 1);
        }
        Ok(Self::Undefined)
      }
      Self::Array(arr) => {
        if prop == LENGTH_KEY {
          let len = arr.read().len();
          return Ok(Self::Smi(len as i32));
        }
        if prop.as_bytes().first().is_some_and(|b| b.is_ascii_digit())
          && let Ok(idx) = prop.parse::<usize>()
        {
          let guard = arr.read();
          return Ok(guard.get(idx).cloned().unwrap_or(Self::Undefined));
        }
        match prop {
          "push" => Ok(Self::new_native_bare(NativeFn::ArrayPush(Arc::clone(arr)))),
          "pop" => Ok(Self::new_native_bare(NativeFn::ArrayPop(Arc::clone(arr)))),
          "shift" => Ok(Self::new_native_bare(NativeFn::ArrayShift(Arc::clone(arr)))),
          "slice" => Ok(Self::new_native_bare(NativeFn::ArraySlice(Arc::clone(arr)))),
          "join" => Ok(Self::new_native_bare(NativeFn::ArrayJoin(Arc::clone(arr)))),
          "indexOf" => Ok(Self::new_native_bare(NativeFn::ArrayIndexOf(Arc::clone(
            arr,
          )))),
          _ => Ok(Self::Undefined),
        }
      }
      Self::String(s) => {
        if prop == LENGTH_KEY {
          return Ok(Self::Smi(char_count(s) as i32));
        }
        if prop.as_bytes().first().is_some_and(|b| b.is_ascii_digit())
          && let Ok(idx) = prop.parse::<usize>()
        {
          if s.is_ascii() {
            if let Some(&b) = s.as_bytes().get(idx) {
              return Ok(Self::String((b as char).to_string()));
            }
          } else if let Some(ch) = s.chars().nth(idx) {
            return Ok(Self::String(ch.to_string()));
          }
        }
        match prop {
          "charCodeAt" => Ok(Self::new_native_bare(NativeFn::StringCharCodeAt(s.clone()))),
          "charAt" => Ok(Self::new_native_bare(NativeFn::StringCharAt(s.clone()))),
          "indexOf" => Ok(Self::new_native_bare(NativeFn::StringIndexOf(s.clone()))),
          "substring" | "slice" => Ok(Self::new_native_bare(NativeFn::StringSubstring(s.clone()))),
          "split" => Ok(Self::new_native_bare(NativeFn::StringSplit(s.clone()))),
          "replace" => Ok(Self::new_native_bare(NativeFn::StringReplace(s.clone()))),
          "match" => Ok(Self::new_native_bare(NativeFn::StringMatch(s.clone()))),
          "toLowerCase" => Ok(Self::new_native_bare(NativeFn::StringToLowerCase(
            s.clone(),
          ))),
          "toUpperCase" => Ok(Self::new_native_bare(NativeFn::StringToUpperCase(
            s.clone(),
          ))),
          "toString" | "valueOf" => Ok(Self::new_native_bare(NativeFn::ConstString(s.clone()))),
          _ => Ok(Self::Undefined),
        }
      }
      Self::Function(func, props) => {
        if prop == LENGTH_KEY {
          return Ok(Self::Smi(func.params.len() as i32));
        }
        if prop == NAME_KEY {
          return Ok(Self::String(func.name.clone().unwrap_or_default()));
        }
        if prop == CALL_KEY {
          return Ok(FUNCTION_CALL.clone());
        }
        if prop == APPLY_KEY {
          return Ok(FUNCTION_APPLY.clone());
        }
        let guard = props.read();
        if let Some(val) = guard.get(prop) {
          return Ok(val.clone());
        }
        if let Some(proto) = guard.get(PROTO_KEY) {
          return proto.get_property_depth(prop, depth + 1);
        }
        Ok(Self::Undefined)
      }
      Self::Native(_, props) => {
        if prop == CALL_KEY {
          return Ok(FUNCTION_CALL.clone());
        }
        if prop == APPLY_KEY {
          return Ok(FUNCTION_APPLY.clone());
        }
        let guard = props.read();
        if let Some(val) = guard.get(prop) {
          return Ok(val.clone());
        }
        if let Some(proto) = guard.get(PROTO_KEY) {
          return proto.get_property_depth(prop, depth + 1);
        }
        Ok(Self::Undefined)
      }
      Self::Smi(n) => {
        if prop == TO_STRING_KEY || prop == VALUE_OF_KEY {
          Ok(Self::new_native_bare(NativeFn::NumberToString(*n as f64)))
        } else {
          Ok(Self::Undefined)
        }
      }
      Self::Number(n) => {
        if prop == TO_STRING_KEY || prop == VALUE_OF_KEY {
          Ok(Self::new_native_bare(NativeFn::NumberToString(*n)))
        } else {
          Ok(Self::Undefined)
        }
      }
      Self::Boolean(b) => {
        if prop == TO_STRING_KEY || prop == VALUE_OF_KEY {
          let s = if *b { "true" } else { "false" };
          Ok(Self::new_native_bare(NativeFn::ConstString(s.to_string())))
        } else {
          Ok(Self::Undefined)
        }
      }
      _ => Ok(Self::Undefined),
    }
  }

  /// 删除属性 (返回是否成功删除)
  pub fn delete_property(&self, prop: &str) -> bool {
    match self {
      Self::Object(obj) => {
        let mut guard = obj.write();
        guard.remove(prop).is_some()
      }
      Self::Function(_, props) | Self::Native(_, props) => {
        let mut guard = props.write();
        guard.remove(prop).is_some()
      }
      _ => true,
    }
  }

  /// 设置命名/键控属性
  pub fn set_property(&self, prop: &str, val: Self) -> Result<()> {
    match self {
      Self::Object(obj) => {
        let mut guard = obj.write();
        if let Some(slot) = guard.get_mut(prop) {
          *slot = val;
        } else {
          guard.insert(prop.to_string(), val);
        }
        Ok(())
      }
      Self::Function(_, props) | Self::Native(_, props) => {
        props.write().insert(prop.to_string(), val);
        Ok(())
      }
      Self::Array(arr) => {
        if prop == LENGTH_KEY {
          let new_len = val.to_u32() as usize;
          arr.write().resize(new_len, Self::Undefined);
          return Ok(());
        }
        if prop.as_bytes().first().is_some_and(|b| b.is_ascii_digit())
          && let Ok(idx) = prop.parse::<usize>()
        {
          let mut guard = arr.write();
          if idx >= guard.len() {
            guard.resize(idx + 1, Self::Undefined);
          }
          guard[idx] = val;
          return Ok(());
        }
        Ok(())
      }
      _ => Ok(()),
    }
  }

  /// 按数值索引获取元素
  pub fn get_element(&self, index: usize) -> Result<Self> {
    match self {
      Self::Array(arr) => {
        let guard = arr.read();
        Ok(guard.get(index).cloned().unwrap_or(Self::Undefined))
      }
      Self::Object(obj) => {
        let guard = obj.read();
        let mut buf = ItoaBuffer::new();
        let key = buf.format(index);
        Ok(guard.get(key).cloned().unwrap_or(Self::Undefined))
      }
      Self::String(s) => {
        if s.is_ascii() {
          if let Some(&b) = s.as_bytes().get(index) {
            Ok(Self::String((b as char).to_string()))
          } else {
            Ok(Self::Undefined)
          }
        } else if let Some(ch) = s.chars().nth(index) {
          Ok(Self::String(ch.to_string()))
        } else {
          Ok(Self::Undefined)
        }
      }
      _ => Ok(Self::Undefined),
    }
  }

  /// 按数值索引写入元素
  pub fn set_element(&self, index: usize, val: Self) -> Result<()> {
    match self {
      Self::Array(arr) => {
        let mut guard = arr.write();
        if index >= guard.len() {
          guard.resize(index + 1, Self::Undefined);
        }
        guard[index] = val;
        Ok(())
      }
      Self::Object(obj) => {
        let mut buf = ItoaBuffer::new();
        let key = buf.format(index).to_string();
        obj.write().insert(key, val);
        Ok(())
      }
      _ => Ok(()),
    }
  }

  /// 转换为布尔值 (ToBoolean 语义)
  #[must_use]
  pub fn to_boolean(&self) -> bool {
    match self {
      Self::Undefined | Self::Null | Self::TheHole => false,
      Self::Boolean(b) => *b,
      Self::Smi(n) => *n != 0,
      Self::Number(n) => !n.is_nan() && *n != 0.0,
      Self::String(s) => !s.is_empty(),
      Self::BigInt(s) => s != "0",
      Self::Array(_) | Self::Object(_) | Self::Function(..) | Self::Native(..) => true,
    }
  }

  /// 转换为浮点数 (ToNumber 语义)
  #[must_use]
  pub fn to_number(&self) -> f64 {
    match self {
      Self::Undefined | Self::TheHole => f64::NAN,
      Self::Null => 0.0,
      Self::Boolean(b) => {
        if *b {
          1.0
        } else {
          0.0
        }
      }
      Self::Smi(n) => *n as f64,
      Self::Number(n) => *n,
      Self::String(s) => {
        let trimmed = s.trim();
        if trimmed.is_empty() {
          0.0
        } else {
          trimmed.parse::<f64>().unwrap_or(f64::NAN)
        }
      }
      Self::BigInt(s) => s.parse::<f64>().unwrap_or(0.0),
      Self::Array(arr) => {
        let guard = arr.read();
        if guard.is_empty() {
          0.0
        } else if guard.len() == 1 {
          guard[0].to_number()
        } else {
          f64::NAN
        }
      }
      Self::Object(_) | Self::Function(..) | Self::Native(..) => f64::NAN,
    }
  }

  /// 转换为 32 位有符号整数
  #[must_use]
  pub fn to_i32(&self) -> i32 {
    match self {
      Self::Smi(n) => *n,
      _ => double_to_int32(self.to_number()),
    }
  }

  /// 转换为 32 位无符号整数
  #[must_use]
  pub fn to_u32(&self) -> u32 {
    match self {
      Self::Smi(n) => *n as u32,
      _ => double_to_uint32(self.to_number()),
    }
  }

  /// 转换为字符串切片引用 (零堆分配借用优先)
  #[must_use]
  pub fn to_str_lossy(&self) -> Cow<'_, str> {
    match self {
      Self::Undefined => Cow::Borrowed("undefined"),
      Self::Null => Cow::Borrowed("null"),
      Self::TheHole => Cow::Borrowed("the_hole"),
      Self::Boolean(true) => Cow::Borrowed("true"),
      Self::Boolean(false) => Cow::Borrowed("false"),
      Self::Smi(n) => {
        let mut buf = ItoaBuffer::new();
        Cow::Owned(buf.format(*n).to_string())
      }
      Self::Number(n) => {
        if n.is_nan() {
          Cow::Borrowed("NaN")
        } else if n.is_infinite() {
          if *n > 0.0 {
            Cow::Borrowed("Infinity")
          } else {
            Cow::Borrowed("-Infinity")
          }
        } else if n.fract() == 0.0 && *n >= (i64::MIN as f64) && *n <= (i64::MAX as f64) {
          let mut buf = ItoaBuffer::new();
          Cow::Owned(buf.format(*n as i64).to_string())
        } else {
          Cow::Owned(n.to_string())
        }
      }
      Self::String(s) => Cow::Borrowed(s.as_str()),
      Self::BigInt(s) => Cow::Borrowed(s.as_str()),
      Self::Array(arr) => {
        let guard = arr.read();
        let mut out = String::new();
        for (i, item) in guard.iter().enumerate() {
          if i > 0 {
            out.push(',');
          }
          out.push_str(&item.to_str_lossy());
        }
        Cow::Owned(out)
      }
      Self::Object(_) => Cow::Borrowed("[object Object]"),
      Self::Function(func, _) => Cow::Owned(format!(
        "function {}() {{ [bytecode] }}",
        func.name.as_deref().unwrap_or("")
      )),
      Self::Native(..) => Cow::Borrowed("function () { [native code] }"),
    }
  }

  /// 转换为字符串表示
  #[inline]
  #[must_use]
  pub fn to_string_lossy(&self) -> String {
    self.to_str_lossy().into_owned()
  }

  /// 加法运算 (Acc = lhs + rhs)
  #[must_use]
  pub fn add(&self, other: &Self) -> Self {
    if let (Self::String(a), _) = (self, other) {
      let other_str = other.to_str_lossy();
      let mut out = String::with_capacity(a.len() + other_str.len());
      out.push_str(a);
      out.push_str(&other_str);
      return Self::String(out);
    }
    if let (_, Self::String(b)) = (self, other) {
      let self_str = self.to_str_lossy();
      let mut out = String::with_capacity(self_str.len() + b.len());
      out.push_str(&self_str);
      out.push_str(b);
      return Self::String(out);
    }
    if let (Self::Smi(a), Self::Smi(b)) = (self, other)
      && let Some(sum) = a.checked_add(*b)
    {
      return Self::Smi(sum);
    }
    Self::Number(self.to_number() + other.to_number())
  }

  /// 减法运算 (Acc = lhs - rhs)
  #[must_use]
  pub fn sub(&self, other: &Self) -> Self {
    if let (Self::Smi(a), Self::Smi(b)) = (self, other)
      && let Some(diff) = a.checked_sub(*b)
    {
      return Self::Smi(diff);
    }
    Self::Number(self.to_number() - other.to_number())
  }

  /// 乘法运算 (Acc = lhs * rhs)
  #[must_use]
  pub fn mul(&self, other: &Self) -> Self {
    if let (Self::Smi(a), Self::Smi(b)) = (self, other)
      && let Some(prod) = a.checked_mul(*b)
    {
      return Self::Smi(prod);
    }
    Self::Number(self.to_number() * other.to_number())
  }

  /// 除法运算 (Acc = lhs / rhs)
  #[must_use]
  pub fn div(&self, other: &Self) -> Self {
    if let (Self::Smi(a), Self::Smi(b)) = (self, other)
      && *b != 0
      && *a % *b == 0
      && let Some(quot) = a.checked_div(*b)
    {
      return Self::Smi(quot);
    }
    Self::Number(self.to_number() / other.to_number())
  }

  /// 取模运算 (Acc = lhs % rhs)
  #[must_use]
  pub fn mod_op(&self, other: &Self) -> Self {
    if let (Self::Smi(a), Self::Smi(b)) = (self, other)
      && let Some(rem) = a.checked_rem(*b)
    {
      if rem == 0 && *a < 0 {
        return Self::Number(-0.0);
      }
      return Self::Smi(rem);
    }
    Self::Number(self.to_number() % other.to_number())
  }

  /// 幂运算
  #[must_use]
  pub fn exp(&self, other: &Self) -> Self {
    Self::Number(self.to_number().powf(other.to_number()))
  }

  /// 按位或
  #[must_use]
  pub fn bitwise_or(&self, other: &Self) -> Self {
    Self::Smi(self.to_i32() | other.to_i32())
  }

  /// 按位与
  #[must_use]
  pub fn bitwise_and(&self, other: &Self) -> Self {
    Self::Smi(self.to_i32() & other.to_i32())
  }

  /// 按位异或
  #[must_use]
  pub fn bitwise_xor(&self, other: &Self) -> Self {
    Self::Smi(self.to_i32() ^ other.to_i32())
  }

  /// 左移
  #[must_use]
  pub fn shift_left(&self, other: &Self) -> Self {
    Self::Smi(self.to_i32() << (other.to_u32() & 31))
  }

  /// 有符号右移
  #[must_use]
  pub fn shift_right(&self, other: &Self) -> Self {
    Self::Smi(self.to_i32() >> (other.to_u32() & 31))
  }

  /// 无符号右移
  #[must_use]
  pub fn shift_right_logical(&self, other: &Self) -> Self {
    let res = (self.to_u32() >> (other.to_u32() & 31)) as i64;
    if res <= i32::MAX as i64 {
      Self::Smi(res as i32)
    } else {
      Self::Number(res as f64)
    }
  }

  /// 按位取反
  #[must_use]
  pub fn bitwise_not(&self) -> Self {
    Self::Smi(!self.to_i32())
  }

  /// 自增
  #[must_use]
  pub fn inc(&self) -> Self {
    if let Self::Smi(n) = self
      && let Some(res) = n.checked_add(1)
    {
      return Self::Smi(res);
    }
    Self::Number(self.to_number() + 1.0)
  }

  /// 自减
  #[must_use]
  pub fn dec(&self) -> Self {
    if let Self::Smi(n) = self
      && let Some(res) = n.checked_sub(1)
    {
      return Self::Smi(res);
    }
    Self::Number(self.to_number() - 1.0)
  }

  /// 取负
  #[must_use]
  pub fn negate(&self) -> Self {
    if let Self::Smi(n) = self
      && *n != 0
      && *n != i32::MIN
    {
      return Self::Smi(-*n);
    }
    Self::Number(-self.to_number())
  }

  /// 宽松相等 (TestEqual)
  #[must_use]
  pub fn test_equal(&self, other: &Self) -> bool {
    if self.test_equal_strict(other) {
      return true;
    }
    match (self, other) {
      (Self::Null, Self::Undefined) | (Self::Undefined, Self::Null) => true,
      (Self::Null | Self::Undefined, _) | (_, Self::Null | Self::Undefined) => false,
      _ => {
        let n1 = self.to_number();
        let n2 = other.to_number();
        !n1.is_nan() && !n2.is_nan() && n1 == n2
      }
    }
  }

  /// 严格相等 (TestEqualStrict)
  #[must_use]
  pub fn test_equal_strict(&self, other: &Self) -> bool {
    match (self, other) {
      (Self::Undefined, Self::Undefined)
      | (Self::Null, Self::Null)
      | (Self::TheHole, Self::TheHole) => true,
      (Self::Boolean(a), Self::Boolean(b)) => a == b,
      (Self::Smi(a), Self::Smi(b)) => a == b,
      (Self::Number(a), Self::Number(b)) => a == b,
      (Self::Smi(a), Self::Number(b)) | (Self::Number(b), Self::Smi(a)) => (*a as f64) == *b,
      (Self::String(a), Self::String(b)) => a == b,
      (Self::BigInt(a), Self::BigInt(b)) => a == b,
      (Self::Array(a), Self::Array(b)) => Arc::ptr_eq(a, b),
      (Self::Object(a), Self::Object(b)) => Arc::ptr_eq(a, b),
      (Self::Function(a, _), Self::Function(b, _)) => Arc::ptr_eq(a, b),
      (Self::Native(a, _), Self::Native(b, _)) => a == b,
      _ => false,
    }
  }

  /// 小于比较
  #[must_use]
  pub fn test_less_than(&self, other: &Self) -> bool {
    if let (Self::String(a), Self::String(b)) = (self, other) {
      a < b
    } else {
      self.to_number() < other.to_number()
    }
  }

  /// 大于比较
  #[must_use]
  pub fn test_greater_than(&self, other: &Self) -> bool {
    if let (Self::String(a), Self::String(b)) = (self, other) {
      a > b
    } else {
      self.to_number() > other.to_number()
    }
  }

  /// 小于等于比较
  #[must_use]
  pub fn test_less_than_or_equal(&self, other: &Self) -> bool {
    if let (Self::String(a), Self::String(b)) = (self, other) {
      a <= b
    } else {
      self.to_number() <= other.to_number()
    }
  }

  /// 大于等于比较
  #[must_use]
  pub fn test_greater_than_or_equal(&self, other: &Self) -> bool {
    if let (Self::String(a), Self::String(b)) = (self, other) {
      a >= b
    } else {
      self.to_number() >= other.to_number()
    }
  }
}

/// 单态内联缓存条目 (对标 V8 Monomorphic IC，消除对象读写锁与哈希争用)
#[derive(Debug, Clone, Default, PartialEq)]
pub struct FeedbackSlot {
  pub cached_obj_ptr: usize,
  pub cached_name: String,
  pub cached_val: Option<Value>,
}

/// 字节码分发器 / 解释执行器
#[derive(Debug, Clone, Default)]
pub struct BytecodeDispatcher {
  accumulator: Value,
  reg_stack: Vec<Value>,
  param_stack: Vec<Value>,
  frame_reg_base: usize,
  frame_param_base: usize,
  frame_param_len: usize,
  call_stack: Vec<String>,
  #[allow(dead_code)]
  feedback_vector: Vec<FeedbackSlot>,
  arguments_materialized: bool,
  saved_arguments: Option<Value>,
  globals: HashMap<String, Value>,
}

impl BytecodeDispatcher {
  #[inline]
  #[must_use]
  pub fn new() -> Self {
    Self {
      accumulator: Value::Undefined,
      reg_stack: Vec::with_capacity(4096),
      param_stack: Vec::with_capacity(1024),
      frame_reg_base: 0,
      frame_param_base: 0,
      frame_param_len: 0,
      call_stack: Vec::with_capacity(64),
      feedback_vector: Vec::with_capacity(256),
      arguments_materialized: false,
      saved_arguments: None,
      globals: HashMap::default(),
    }
  }

  #[inline]
  #[must_use]
  pub const fn accumulator(&self) -> &Value {
    &self.accumulator
  }

  #[inline]
  #[must_use]
  pub fn registers(&self) -> &[Value] {
    if self.frame_reg_base < self.reg_stack.len() {
      &self.reg_stack[self.frame_reg_base..]
    } else {
      &[]
    }
  }

  #[inline]
  #[must_use]
  pub fn globals(&self) -> &HashMap<String, Value> {
    &self.globals
  }

  #[inline]
  pub fn set_global(&mut self, name: impl Into<String>, val: Value) {
    self.globals.insert(name.into(), val);
  }

  #[inline]
  #[must_use]
  pub fn get_global(&self, name: &str) -> Option<&Value> {
    self.globals.get(name)
  }

  #[inline]
  pub fn get_register_ref(&self, reg: Register) -> Result<&Value> {
    if reg.is_parameter() {
      let p_idx = reg.to_parameter_index();
      if p_idx >= 0 && (p_idx as usize) < self.frame_param_len {
        let abs_idx = self.frame_param_base + p_idx as usize;
        if let Some(val) = self.param_stack.get(abs_idx) {
          return Ok(val);
        }
      }
      Ok(&Value::Undefined)
    } else {
      let idx = reg.index();
      if idx >= 0 {
        let abs_idx = self.frame_reg_base + idx as usize;
        if let Some(val) = self.reg_stack.get(abs_idx) {
          Ok(val)
        } else {
          Err(Error::RegisterOutOfBounds(idx))
        }
      } else {
        Err(Error::RegisterOutOfBounds(idx))
      }
    }
  }

  #[inline]
  fn get_register(&self, reg: Register) -> Result<Value> {
    self.get_register_ref(reg).cloned()
  }

  #[inline]
  fn set_register(&mut self, reg: Register, val: Value) -> Result<()> {
    if reg.is_parameter() {
      let p_idx = reg.to_parameter_index();
      if p_idx >= 0 {
        let abs_idx = self.frame_param_base + p_idx as usize;
        if abs_idx >= self.param_stack.len() {
          self.param_stack.resize(abs_idx + 1, Value::Undefined);
        }
        self.param_stack[abs_idx] = val;
        self.frame_param_len = self.frame_param_len.max((p_idx as usize) + 1);
      }
      Ok(())
    } else {
      let idx = reg.index();
      if idx >= 0 {
        let abs_idx = self.frame_reg_base + idx as usize;
        if abs_idx >= self.reg_stack.len() {
          self.reg_stack.resize(abs_idx + 1, Value::Undefined);
        }
        self.reg_stack[abs_idx] = val;
        Ok(())
      } else {
        Err(Error::RegisterOutOfBounds(idx))
      }
    }
  }

  /// 在异常表中寻找当前偏移处最内层的 catch 目标
  #[inline]
  #[must_use]
  pub fn find_catch_handler(bytecode_array: &BytecodeArray, offset: usize) -> Option<usize> {
    bytecode_array.find_catch_handler(offset)
  }

  #[inline]
  fn handle_call_result(
    &mut self,
    res: Result<Value>,
    it: &mut BytecodeArrayIterator,
    bytecode_array: &BytecodeArray,
  ) -> Result<()> {
    match res {
      Ok(val) => {
        self.accumulator = val;
        it.advance();
        Ok(())
      }
      Err(err) => {
        let current_offset = it.current_offset();
        if let Some(target) = Self::find_catch_handler(bytecode_array, current_offset) {
          if let Error::ExecutionError(msg) = &err {
            self.accumulator = Value::String(msg.clone());
          } else {
            self.accumulator = Value::String(err.to_string());
          }
          it.set_offset(target)
        } else {
          Err(err)
        }
      }
    }
  }

  /// 解析命名属性 (单次原型回溯，根除 O(N^2) 重复锁与哈希遍历)
  #[inline]
  pub fn get_property_resolved(&self, obj: &Value, prop: &str) -> Result<Value> {
    let val = obj.get_property(prop)?;
    if !matches!(val, Value::Undefined) {
      return Ok(val);
    }
    if let Some(obj_ctor) = self.globals.get("Object")
      && let Ok(proto) = obj_ctor.get_property(PROTOTYPE_KEY)
      && let Ok(v) = proto.get_property(prop)
      && !matches!(v, Value::Undefined)
    {
      return Ok(v);
    }
    if prop == TO_STRING_KEY {
      return Ok(Value::new_native_bare(NativeFn::ConstString(
        "[object Object]".to_string(),
      )));
    }
    Ok(Value::Undefined)
  }

  /// 检查属性是否存在于对象或其原型链中 (TestIn)
  pub fn has_property_resolved(&self, obj: &Value, prop: &str) -> bool {
    let mut curr = Some(obj.clone());
    let mut depth = 0;
    while let Some(cur_val) = curr {
      depth += 1;
      if depth > 64 {
        break;
      }
      match cur_val {
        Value::Object(o) => {
          let guard = o.read();
          if guard.contains_key(prop) {
            return true;
          }
          curr = guard.get(PROTO_KEY).cloned();
        }
        Value::Function(_, props) | Value::Native(_, props) => {
          if prop == LENGTH_KEY
            || prop == NAME_KEY
            || prop == PROTOTYPE_KEY
            || prop == CALL_KEY
            || prop == APPLY_KEY
          {
            return true;
          }
          let guard = props.read();
          if guard.contains_key(prop) {
            return true;
          }
          curr = guard.get(PROTO_KEY).cloned();
        }
        Value::Array(a) => {
          if prop == LENGTH_KEY
            || prop == PUSH_KEY
            || prop == POP_KEY
            || prop == SLICE_KEY
            || prop == SHIFT_KEY
            || prop == JOIN_KEY
            || prop == INDEX_OF_KEY
          {
            return true;
          }
          let guard = a.read();
          if let Ok(idx) = prop.parse::<usize>()
            && idx < guard.len()
          {
            return true;
          }
          break;
        }
        _ => break,
      }
    }
    if let Some(obj_ctor) = self.globals.get("Object")
      && let Ok(proto) = obj_ctor.get_property(PROTOTYPE_KEY)
      && let Value::Object(proto_map) = proto
      && proto_map.read().contains_key(prop)
    {
      return true;
    }
    false
  }

  /// 检查实例原型链是否继承自构造函数原型 (TestInstanceOf)
  pub fn is_instance_of(&self, instance: &Value, constructor: &Value) -> bool {
    let target_proto = match constructor.get_property(PROTOTYPE_KEY) {
      Ok(p) if matches!(p, Value::Object(_) | Value::Array(_) | Value::Function(..)) => p,
      _ => return false,
    };

    let mut curr = match instance {
      Value::Object(o) => o.read().get(PROTO_KEY).cloned(),
      Value::Function(_, p) | Value::Native(_, p) => p.read().get(PROTO_KEY).cloned(),
      _ => None,
    };

    let mut depth = 0;
    while let Some(cur_val) = curr {
      depth += 1;
      if depth > 64 {
        break;
      }
      if match (&cur_val, &target_proto) {
        (Value::Object(a), Value::Object(b)) => Arc::ptr_eq(a, b),
        (Value::Array(a), Value::Array(b)) => Arc::ptr_eq(a, b),
        (Value::Function(a_f, a_p), Value::Function(b_f, b_p)) => {
          Arc::ptr_eq(a_f, b_f) && Arc::ptr_eq(a_p, b_p)
        }
        _ => false,
      } {
        return true;
      }
      curr = match &cur_val {
        Value::Object(o) => o.read().get(PROTO_KEY).cloned(),
        Value::Function(_, p) | Value::Native(_, p) => p.read().get(PROTO_KEY).cloned(),
        _ => None,
      };
    }

    if let Some(obj_ctor) = self.globals.get("Object")
      && let Ok(obj_proto) = obj_ctor.get_property("prototype")
      && match (&target_proto, &obj_proto) {
        (Value::Object(a), Value::Object(b)) => Arc::ptr_eq(a, b),
        _ => false,
      }
      && matches!(
        instance,
        Value::Object(_) | Value::Array(_) | Value::Function(..)
      )
    {
      return true;
    }

    false
  }

  /// 函数调用通用入口
  pub fn call_function(
    &mut self,
    callable: &Value,
    receiver: &Value,
    args: &[Value],
  ) -> Result<Value> {
    if callable == &*FUNCTION_CALL {
      let target_fn = receiver;
      let target_receiver = args.first().unwrap_or(&Value::Undefined);
      let target_args = if args.len() > 1 { &args[1..] } else { &[] };
      return self.call_function(target_fn, target_receiver, target_args);
    }
    if callable == &*FUNCTION_APPLY {
      let target_fn = receiver;
      let target_receiver = args.first().unwrap_or(&Value::Undefined);
      if let Some(Value::Array(arr)) = args.get(1) {
        let guard = arr.read();
        return self.call_function(target_fn, target_receiver, &guard);
      }
      return self.call_function(target_fn, target_receiver, &[]);
    }
    match callable {
      Value::Native(nat, _) => nat.call(args),
      Value::Function(func, _) => {
        let fn_name = func.name.clone().unwrap_or_else(|| "anonymous".to_string());
        self.call_stack.push(fn_name);
        let old_acc = replace(&mut self.accumulator, Value::Undefined);

        let saved_reg_base = self.frame_reg_base;
        let saved_param_base = self.frame_param_base;
        let saved_param_len = self.frame_param_len;

        let reg_count = func.bytecode_array.register_count();
        let new_reg_base = self.reg_stack.len();
        self
          .reg_stack
          .resize(new_reg_base + reg_count, Value::Undefined);

        let new_param_base = self.param_stack.len();
        self.param_stack.push(receiver.clone());
        self.param_stack.extend_from_slice(args);
        let new_param_len = args.len() + 1;

        self.frame_reg_base = new_reg_base;
        self.frame_param_base = new_param_base;
        self.frame_param_len = new_param_len;

        let saved_arguments_materialized = self.arguments_materialized;
        let saved_global_arguments = self.saved_arguments.take();
        self.arguments_materialized = false;

        let saved_param_globals: Vec<(String, Option<Value>)> = if func.captured_locals.is_empty() {
          Vec::new()
        } else {
          func
            .captured_locals
            .iter()
            .map(|p| (p.clone(), self.globals.get(p).cloned()))
            .collect()
        };

        let res = self.run_bytecode_array(&func.bytecode_array);

        for (p_name, old_val) in saved_param_globals {
          if let Some(old) = old_val {
            self.globals.insert(p_name, old);
          }
        }

        if self.arguments_materialized {
          if let Some(old) = self.saved_arguments.take() {
            self.globals.insert("arguments".to_string(), old);
          } else {
            self.globals.remove("arguments");
          }
        }
        self.arguments_materialized = saved_arguments_materialized;
        self.saved_arguments = saved_global_arguments;

        self.reg_stack.truncate(new_reg_base);
        self.param_stack.truncate(new_param_base);
        self.frame_reg_base = saved_reg_base;
        self.frame_param_base = saved_param_base;
        self.frame_param_len = saved_param_len;

        self.accumulator = old_acc;
        self.call_stack.pop();
        res
      }
      other => Err(Error::ExecutionError(format!(
        "{other:?} is not callable (receiver: {receiver:?})"
      ))),
    }
  }

  /// 构造函数调用通用入口
  pub fn construct(&mut self, ctor: &Value, args: &[Value]) -> Result<Value> {
    match ctor {
      Value::Native(nat, props) => {
        let new_obj = Value::new_object();
        let guard = props.read();
        if let Some(proto) = guard.get(PROTOTYPE_KEY)
          && matches!(proto, Value::Object(_) | Value::Array(_))
        {
          let _ = new_obj.set_property(PROTO_KEY, proto.clone());
        }
        drop(guard);
        let res = nat.call(args)?;
        if matches!(res, Value::Object(_) | Value::Array(_)) {
          Ok(res)
        } else {
          Ok(new_obj)
        }
      }
      Value::Function(_func, _) => {
        let new_obj = Value::new_object();
        if let Ok(proto) = ctor.get_property(PROTOTYPE_KEY)
          && matches!(proto, Value::Object(_) | Value::Array(_))
        {
          let _ = new_obj.set_property(PROTO_KEY, proto);
        }
        let res = self.call_function(ctor, &new_obj, args)?;
        if matches!(res, Value::Object(_) | Value::Array(_)) {
          Ok(res)
        } else {
          Ok(new_obj)
        }
      }
      _ => Err(Error::UnsupportedAstNode("value is not a constructor")),
    }
  }

  /// 执行字节码数组
  #[inline(never)]
  pub fn execute(&mut self, bytecode_array: &BytecodeArray) -> Result<Value> {
    self.execute_with_args(bytecode_array, &[])
  }

  /// 带入参执行字节码数组
  #[inline(never)]
  pub fn execute_with_args(
    &mut self,
    bytecode_array: &BytecodeArray,
    args: &[Value],
  ) -> Result<Value> {
    self.execute_with_args_and_this(bytecode_array, Value::Undefined, args)
  }

  /// 带接收者 (this) 与入参执行字节码数组
  #[inline(never)]
  pub fn execute_with_args_and_this(
    &mut self,
    bytecode_array: &BytecodeArray,
    this_val: Value,
    args: &[Value],
  ) -> Result<Value> {
    self.accumulator = Value::Undefined;
    self.reg_stack.clear();
    self
      .reg_stack
      .resize(bytecode_array.register_count(), Value::Undefined);
    self.param_stack.clear();
    self.param_stack.reserve(args.len() + 1);
    self.param_stack.push(this_val);
    self.param_stack.extend_from_slice(args);
    self.frame_reg_base = 0;
    self.frame_param_base = 0;
    self.frame_param_len = args.len() + 1;
    self.run_bytecode_array(bytecode_array)
  }

  fn run_bytecode_array(&mut self, bytecode_array: &BytecodeArray) -> Result<Value> {
    let mut it = BytecodeArrayIterator::new(bytecode_array.bytecodes());
    while !it.done() {
      let bc = it.current_bytecode();
      match bc {
        Bytecode::LdaZero => {
          self.accumulator = Value::Smi(0);
          it.advance();
        }
        Bytecode::LdaSmi => {
          let smi = it.get_immediate_operand(0)?;
          self.accumulator = Value::Smi(smi);
          it.advance();
        }
        Bytecode::LdaUndefined => {
          self.accumulator = Value::Undefined;
          it.advance();
        }
        Bytecode::LdaNull => {
          self.accumulator = Value::Null;
          it.advance();
        }
        Bytecode::LdaTheHole => {
          self.accumulator = Value::TheHole;
          it.advance();
        }
        Bytecode::LdaTrue => {
          self.accumulator = Value::Boolean(true);
          it.advance();
        }
        Bytecode::LdaFalse => {
          self.accumulator = Value::Boolean(false);
          it.advance();
        }
        Bytecode::LdaConstant => {
          let idx = it.get_unsigned_operand(0)? as usize;
          if let Some(entry) = bytecode_array.constants().get(idx) {
            self.accumulator = match entry {
              ConstantPoolEntry::Number(n) => Value::Number(*n),
              ConstantPoolEntry::String(s) => Value::String(s.clone()),
              ConstantPoolEntry::BigInt(b) => Value::BigInt(b.clone()),
              ConstantPoolEntry::Function(f) => Value::new_function(Arc::clone(f)),
            };
            it.advance();
          } else {
            return Err(Error::ConstantPoolOutOfBounds(idx));
          }
        }
        Bytecode::LdaGlobal => {
          let idx = it.get_unsigned_operand(0)? as usize;
          let _slot = it.get_unsigned_operand(1)?;
          if let Some(ConstantPoolEntry::String(name)) = bytecode_array.constants().get(idx) {
            if name == "arguments" && !self.arguments_materialized {
              let slice_len = self.frame_param_len.saturating_sub(1);
              let start = self.frame_param_base + 1;
              let end = start + slice_len;
              let args_slice = if end <= self.param_stack.len() {
                &self.param_stack[start..end]
              } else {
                &[]
              };
              let arguments_val = Value::new_array_with_elements(args_slice.to_vec());
              self.saved_arguments = self
                .globals
                .insert("arguments".to_string(), arguments_val.clone());
              self.arguments_materialized = true;
              self.accumulator = arguments_val;
            } else if let Some(val) = self.globals.get(name) {
              self.accumulator = val.clone();
            } else {
              self.accumulator = Value::Undefined;
            }
            it.advance();
          } else {
            return Err(Error::ConstantPoolOutOfBounds(idx));
          }
        }
        Bytecode::StaGlobal => {
          let idx = it.get_unsigned_operand(0)? as usize;
          let _slot = it.get_unsigned_operand(1)?;
          if let Some(ConstantPoolEntry::String(name)) = bytecode_array.constants().get(idx) {
            self.globals.insert(name.clone(), self.accumulator.clone());
            it.advance();
          } else {
            return Err(Error::ConstantPoolOutOfBounds(idx));
          }
        }
        Bytecode::Ldar => {
          let reg = it.get_register_operand(0)?;
          self.accumulator = self.get_register(reg)?;
          it.advance();
        }
        Bytecode::Star => {
          let reg = it.get_register_operand(0)?;
          self.set_register(reg, self.accumulator.clone())?;
          it.advance();
        }
        bc if Bytecodes::is_short_star(bc) => {
          let reg = Register::from_short_star(bc);
          self.set_register(reg, self.accumulator.clone())?;
          it.advance();
        }
        Bytecode::Mov => {
          let src = it.get_register_operand(0)?;
          let dst = it.get_register_operand(1)?;
          let val = self.get_register(src)?;
          self.set_register(dst, val)?;
          it.advance();
        }
        Bytecode::GetNamedProperty => {
          let obj_reg = it.get_register_operand(0)?;
          let idx = it.get_unsigned_operand(1)? as usize;
          let slot = it.get_unsigned_operand(2)? as usize;
          let obj = self.get_register(obj_reg)?;
          if let Some(ConstantPoolEntry::String(name)) = bytecode_array.constants().get(idx) {
            let obj_ptr = match &obj {
              Value::Object(o) => Arc::as_ptr(o) as usize,
              Value::Function(_, props) | Value::Native(_, props) => Arc::as_ptr(props) as usize,
              _ => 0,
            };
            if obj_ptr != 0 && slot < self.feedback_vector.len() {
              let entry = &self.feedback_vector[slot];
              if entry.cached_obj_ptr == obj_ptr
                && entry.cached_name == *name
                && let Some(ref val) = entry.cached_val
              {
                self.accumulator = val.clone();
                it.advance();
                continue;
              }
            }
            let val = self.get_property_resolved(&obj, name)?;
            if obj_ptr != 0 {
              if slot >= self.feedback_vector.len() {
                self
                  .feedback_vector
                  .resize(slot + 1, FeedbackSlot::default());
              }
              self.feedback_vector[slot] = FeedbackSlot {
                cached_obj_ptr: obj_ptr,
                cached_name: name.clone(),
                cached_val: Some(val.clone()),
              };
            }
            self.accumulator = val;
            it.advance();
          } else {
            return Err(Error::ConstantPoolOutOfBounds(idx));
          }
        }
        Bytecode::SetNamedProperty => {
          let obj_reg = it.get_register_operand(0)?;
          let idx = it.get_unsigned_operand(1)? as usize;
          let slot = it.get_unsigned_operand(2)? as usize;
          let obj = self.get_register(obj_reg)?;
          if let Some(ConstantPoolEntry::String(name)) = bytecode_array.constants().get(idx) {
            let obj_ptr = match &obj {
              Value::Object(o) => Arc::as_ptr(o) as usize,
              Value::Function(_, props) | Value::Native(_, props) => Arc::as_ptr(props) as usize,
              _ => 0,
            };
            if obj_ptr != 0 && slot < self.feedback_vector.len() {
              self.feedback_vector[slot].cached_obj_ptr = 0;
            }
            obj.set_property(name, self.accumulator.clone())?;
            it.advance();
          } else {
            return Err(Error::ConstantPoolOutOfBounds(idx));
          }
        }
        Bytecode::GetKeyedProperty => {
          let obj_reg = it.get_register_operand(0)?;
          let _slot = it.get_unsigned_operand(1)?;
          let obj = self.get_register(obj_reg)?;
          if let Value::Array(ref arr) = obj {
            let fast_idx = match self.accumulator {
              Value::Smi(i) if i >= 0 => Some(i as usize),
              Value::Number(n) if n >= 0.0 && n.fract() == 0.0 => Some(n as usize),
              _ => None,
            };
            if let Some(idx) = fast_idx {
              let guard = arr.read();
              self.accumulator = guard.get(idx).cloned().unwrap_or(Value::Undefined);
              it.advance();
              continue;
            }
          }
          let key = self.accumulator.to_str_lossy();
          self.accumulator = self.get_property_resolved(&obj, &key)?;
          it.advance();
        }
        Bytecode::SetKeyedProperty => {
          let obj_reg = it.get_register_operand(0)?;
          let key_reg = it.get_register_operand(1)?;
          let _slot = it.get_unsigned_operand(2)?;
          let obj = self.get_register(obj_reg)?;
          let key = self.get_register(key_reg)?;
          if let Value::Array(ref arr) = obj {
            let fast_idx = match key {
              Value::Smi(i) if i >= 0 => Some(i as usize),
              Value::Number(n) if n >= 0.0 && n.fract() == 0.0 => Some(n as usize),
              _ => None,
            };
            if let Some(idx) = fast_idx {
              let mut guard = arr.write();
              if idx >= guard.len() {
                guard.resize(idx + 1, Value::Undefined);
              }
              guard[idx] = self.accumulator.clone();
              it.advance();
              continue;
            }
          }
          let key_str = key.to_str_lossy();
          obj.set_property(&key_str, self.accumulator.clone())?;
          it.advance();
        }
        Bytecode::DeletePropertySloppy | Bytecode::DeletePropertyStrict => {
          let obj_reg = it.get_register_operand(0)?;
          let obj = self.get_register(obj_reg)?;
          let key = self.accumulator.to_str_lossy();
          let ok = obj.delete_property(&key);
          self.accumulator = Value::Boolean(ok);
          it.advance();
        }
        Bytecode::StaInArrayLiteral => {
          let arr_reg = it.get_register_operand(0)?;
          let idx_reg = it.get_register_operand(1)?;
          let _slot = it.get_unsigned_operand(2)?;
          let arr = self.get_register_ref(arr_reg)?;
          let idx = self.get_register_ref(idx_reg)?;
          arr.set_element(idx.to_u32() as usize, self.accumulator.clone())?;
          it.advance();
        }
        Bytecode::CallProperty0 => {
          let callable_reg = it.get_register_operand(0)?;
          let receiver_reg = it.get_register_operand(1)?;
          let _slot = it.get_unsigned_operand(2)?;
          let callable = self.get_register(callable_reg)?;
          let receiver = self.get_register(receiver_reg)?;
          let res = self.call_function(&callable, &receiver, &[]);
          self.handle_call_result(res, &mut it, bytecode_array)?;
        }
        Bytecode::CallProperty1 => {
          let callable_reg = it.get_register_operand(0)?;
          let receiver_reg = it.get_register_operand(1)?;
          let arg0_reg = it.get_register_operand(2)?;
          let _slot = it.get_unsigned_operand(3)?;
          let callable = self.get_register(callable_reg)?;
          let receiver = self.get_register(receiver_reg)?;
          let arg0 = self.get_register(arg0_reg)?;
          let res = self.call_function(&callable, &receiver, &[arg0]);
          self.handle_call_result(res, &mut it, bytecode_array)?;
        }
        Bytecode::CallProperty2 => {
          let callable_reg = it.get_register_operand(0)?;
          let receiver_reg = it.get_register_operand(1)?;
          let arg0_reg = it.get_register_operand(2)?;
          let arg1_reg = it.get_register_operand(3)?;
          let _slot = it.get_unsigned_operand(4)?;
          let callable = self.get_register(callable_reg)?;
          let receiver = self.get_register(receiver_reg)?;
          let arg0 = self.get_register(arg0_reg)?;
          let arg1 = self.get_register(arg1_reg)?;
          let res = self.call_function(&callable, &receiver, &[arg0, arg1]);
          self.handle_call_result(res, &mut it, bytecode_array)?;
        }
        Bytecode::CallProperty => {
          let callable_reg = it.get_register_operand(0)?;
          let first_arg_reg = it.get_register_operand(1)?;
          let arg_count = it.get_unsigned_operand(2)? as usize;
          let _slot = it.get_unsigned_operand(3)?;
          let callable = self.get_register(callable_reg)?;
          let receiver = if arg_count > 0 {
            self.get_register(first_arg_reg)?
          } else {
            Value::Undefined
          };
          let call_args_len = arg_count.saturating_sub(1);
          let res = if call_args_len <= 8 {
            let mut small_args: [Value; 8] = Default::default();
            for (i, slot) in small_args[..call_args_len].iter_mut().enumerate() {
              let reg = Register::new(first_arg_reg.index() + (i + 1) as i32);
              *slot = self.get_register(reg)?;
            }
            self.call_function(&callable, &receiver, &small_args[..call_args_len])
          } else {
            let mut call_args = Vec::with_capacity(call_args_len);
            for i in 0..call_args_len {
              let reg = Register::new(first_arg_reg.index() + (i + 1) as i32);
              call_args.push(self.get_register(reg)?);
            }
            self.call_function(&callable, &receiver, &call_args)
          };
          self.handle_call_result(res, &mut it, bytecode_array)?;
        }
        Bytecode::CallUndefinedReceiver0 => {
          let callable_reg = it.get_register_operand(0)?;
          let _slot = it.get_unsigned_operand(1)?;
          let callable = self.get_register(callable_reg)?;
          let res = self.call_function(&callable, &Value::Undefined, &[]);
          self.handle_call_result(res, &mut it, bytecode_array)?;
        }
        Bytecode::CallUndefinedReceiver1 => {
          let callable_reg = it.get_register_operand(0)?;
          let arg0_reg = it.get_register_operand(1)?;
          let _slot = it.get_unsigned_operand(2)?;
          let callable = self.get_register(callable_reg)?;
          let arg0 = self.get_register(arg0_reg)?;
          let res = self.call_function(&callable, &Value::Undefined, &[arg0]);
          self.handle_call_result(res, &mut it, bytecode_array)?;
        }
        Bytecode::CallUndefinedReceiver2 => {
          let callable_reg = it.get_register_operand(0)?;
          let arg0_reg = it.get_register_operand(1)?;
          let arg1_reg = it.get_register_operand(2)?;
          let _slot = it.get_unsigned_operand(3)?;
          let callable = self.get_register(callable_reg)?;
          let arg0 = self.get_register(arg0_reg)?;
          let arg1 = self.get_register(arg1_reg)?;
          let res = self.call_function(&callable, &Value::Undefined, &[arg0, arg1]);
          self.handle_call_result(res, &mut it, bytecode_array)?;
        }
        Bytecode::CallUndefinedReceiver => {
          let callable_reg = it.get_register_operand(0)?;
          let first_arg_reg = it.get_register_operand(1)?;
          let arg_count = it.get_unsigned_operand(2)? as usize;
          let _slot = it.get_unsigned_operand(3)?;
          let callable = self.get_register(callable_reg)?;
          let res = if arg_count <= 8 {
            let mut small_args: [Value; 8] = Default::default();
            for (i, slot) in small_args[..arg_count].iter_mut().enumerate() {
              let reg = Register::new(first_arg_reg.index() + i as i32);
              *slot = self.get_register(reg)?;
            }
            self.call_function(&callable, &Value::Undefined, &small_args[..arg_count])
          } else {
            let mut call_args = Vec::with_capacity(arg_count);
            for i in 0..arg_count {
              let reg = Register::new(first_arg_reg.index() + i as i32);
              call_args.push(self.get_register(reg)?);
            }
            self.call_function(&callable, &Value::Undefined, &call_args)
          };
          self.handle_call_result(res, &mut it, bytecode_array)?;
        }
        Bytecode::Construct => {
          let ctor_reg = it.get_register_operand(0)?;
          let first_arg_reg = it.get_register_operand(1)?;
          let arg_count = it.get_unsigned_operand(2)? as usize;
          let _slot = it.get_unsigned_operand(3)?;
          let ctor = self.get_register(ctor_reg)?;
          let res = if arg_count <= 8 {
            let mut small_args: [Value; 8] = Default::default();
            for (i, slot) in small_args[..arg_count].iter_mut().enumerate() {
              let reg = Register::new(first_arg_reg.index() + i as i32);
              *slot = self.get_register(reg)?;
            }
            self.construct(&ctor, &small_args[..arg_count])
          } else {
            let mut ctor_args = Vec::with_capacity(arg_count);
            for i in 0..arg_count {
              let reg = Register::new(first_arg_reg.index() + i as i32);
              ctor_args.push(self.get_register(reg)?);
            }
            self.construct(&ctor, &ctor_args)
          };
          self.handle_call_result(res, &mut it, bytecode_array)?;
        }
        Bytecode::CreateArrayLiteral => {
          let _entry = it.get_unsigned_operand(0)?;
          let _slot = it.get_unsigned_operand(1)?;
          let _flag = it.get_unsigned_operand(2)?;
          self.accumulator = Value::new_array();
          it.advance();
        }
        Bytecode::CreateEmptyArrayLiteral => {
          let _slot = it.get_unsigned_operand(0)?;
          self.accumulator = Value::new_array();
          it.advance();
        }
        Bytecode::CreateObjectLiteral => {
          let _entry = it.get_unsigned_operand(0)?;
          let _slot = it.get_unsigned_operand(1)?;
          let _flag = it.get_unsigned_operand(2)?;
          self.accumulator = Value::new_object();
          it.advance();
        }
        Bytecode::CreateEmptyObjectLiteral => {
          self.accumulator = Value::new_object();
          it.advance();
        }
        Bytecode::Add => {
          let reg = it.get_register_operand(0)?;
          let lhs = self.get_register_ref(reg)?;
          self.accumulator = lhs.add(&self.accumulator);
          it.advance();
        }
        Bytecode::Sub => {
          let reg = it.get_register_operand(0)?;
          let lhs = self.get_register_ref(reg)?;
          self.accumulator = lhs.sub(&self.accumulator);
          it.advance();
        }
        Bytecode::Mul => {
          let reg = it.get_register_operand(0)?;
          let lhs = self.get_register_ref(reg)?;
          self.accumulator = lhs.mul(&self.accumulator);
          it.advance();
        }
        Bytecode::Div => {
          let reg = it.get_register_operand(0)?;
          let lhs = self.get_register_ref(reg)?;
          self.accumulator = lhs.div(&self.accumulator);
          it.advance();
        }
        Bytecode::Mod => {
          let reg = it.get_register_operand(0)?;
          let lhs = self.get_register_ref(reg)?;
          self.accumulator = lhs.mod_op(&self.accumulator);
          it.advance();
        }
        Bytecode::Exp => {
          let reg = it.get_register_operand(0)?;
          let lhs = self.get_register_ref(reg)?;
          self.accumulator = lhs.exp(&self.accumulator);
          it.advance();
        }
        Bytecode::BitwiseOr => {
          let reg = it.get_register_operand(0)?;
          let lhs = self.get_register_ref(reg)?;
          self.accumulator = lhs.bitwise_or(&self.accumulator);
          it.advance();
        }
        Bytecode::BitwiseAnd => {
          let reg = it.get_register_operand(0)?;
          let lhs = self.get_register_ref(reg)?;
          self.accumulator = lhs.bitwise_and(&self.accumulator);
          it.advance();
        }
        Bytecode::BitwiseXor => {
          let reg = it.get_register_operand(0)?;
          let lhs = self.get_register_ref(reg)?;
          self.accumulator = lhs.bitwise_xor(&self.accumulator);
          it.advance();
        }
        Bytecode::ShiftLeft => {
          let reg = it.get_register_operand(0)?;
          let lhs = self.get_register_ref(reg)?;
          self.accumulator = lhs.shift_left(&self.accumulator);
          it.advance();
        }
        Bytecode::ShiftRight => {
          let reg = it.get_register_operand(0)?;
          let lhs = self.get_register_ref(reg)?;
          self.accumulator = lhs.shift_right(&self.accumulator);
          it.advance();
        }
        Bytecode::ShiftRightLogical => {
          let reg = it.get_register_operand(0)?;
          let lhs = self.get_register_ref(reg)?;
          self.accumulator = lhs.shift_right_logical(&self.accumulator);
          it.advance();
        }
        Bytecode::Inc => {
          self.accumulator = self.accumulator.inc();
          it.advance();
        }
        Bytecode::Dec => {
          self.accumulator = self.accumulator.dec();
          it.advance();
        }
        Bytecode::Negate => {
          self.accumulator = self.accumulator.negate();
          it.advance();
        }
        Bytecode::BitwiseNot => {
          self.accumulator = self.accumulator.bitwise_not();
          it.advance();
        }
        Bytecode::LogicalNot | Bytecode::ToBooleanLogicalNot => {
          self.accumulator = Value::Boolean(!self.accumulator.to_boolean());
          it.advance();
        }
        Bytecode::ToBoolean => {
          self.accumulator = Value::Boolean(self.accumulator.to_boolean());
          it.advance();
        }
        Bytecode::ToNumber => {
          self.accumulator = Value::Number(self.accumulator.to_number());
          it.advance();
        }
        Bytecode::TestEqual => {
          let reg = it.get_register_operand(0)?;
          let lhs = self.get_register_ref(reg)?;
          self.accumulator = Value::Boolean(lhs.test_equal(&self.accumulator));
          it.advance();
        }
        Bytecode::TestEqualStrict => {
          let reg = it.get_register_operand(0)?;
          let lhs = self.get_register_ref(reg)?;
          self.accumulator = Value::Boolean(lhs.test_equal_strict(&self.accumulator));
          it.advance();
        }
        Bytecode::TestLessThan => {
          let reg = it.get_register_operand(0)?;
          let lhs = self.get_register_ref(reg)?;
          self.accumulator = Value::Boolean(lhs.test_less_than(&self.accumulator));
          it.advance();
        }
        Bytecode::TestGreaterThan => {
          let reg = it.get_register_operand(0)?;
          let lhs = self.get_register_ref(reg)?;
          self.accumulator = Value::Boolean(lhs.test_greater_than(&self.accumulator));
          it.advance();
        }
        Bytecode::TestLessThanOrEqual => {
          let reg = it.get_register_operand(0)?;
          let lhs = self.get_register_ref(reg)?;
          self.accumulator = Value::Boolean(lhs.test_less_than_or_equal(&self.accumulator));
          it.advance();
        }
        Bytecode::TestGreaterThanOrEqual => {
          let reg = it.get_register_operand(0)?;
          let lhs = self.get_register_ref(reg)?;
          self.accumulator = Value::Boolean(lhs.test_greater_than_or_equal(&self.accumulator));
          it.advance();
        }
        Bytecode::TestInstanceOf => {
          let reg = it.get_register_operand(0)?;
          let _slot = it.get_unsigned_operand(1)?;
          let instance = self.get_register(reg)?;
          let constructor = &self.accumulator;
          let is_inst = self.is_instance_of(&instance, constructor);
          self.accumulator = Value::Boolean(is_inst);
          it.advance();
        }
        Bytecode::TestIn => {
          let reg = it.get_register_operand(0)?;
          let _slot = it.get_unsigned_operand(1)?;
          let prop_val = self.get_register(reg)?;
          let prop_str = prop_val.to_str_lossy();
          let has = self.has_property_resolved(&self.accumulator, &prop_str);
          self.accumulator = Value::Boolean(has);
          it.advance();
        }
        Bytecode::Jump => {
          let target = it.get_jump_target_offset()?;
          it.set_offset(target)?;
        }
        Bytecode::JumpIfTrue | Bytecode::JumpIfToBooleanTrue => {
          if self.accumulator.to_boolean() {
            let target = it.get_jump_target_offset()?;
            it.set_offset(target)?;
          } else {
            it.advance();
          }
        }
        Bytecode::JumpIfFalse | Bytecode::JumpIfToBooleanFalse => {
          if !self.accumulator.to_boolean() {
            let target = it.get_jump_target_offset()?;
            it.set_offset(target)?;
          } else {
            it.advance();
          }
        }
        Bytecode::JumpIfNull => {
          if matches!(self.accumulator, Value::Null) {
            let target = it.get_jump_target_offset()?;
            it.set_offset(target)?;
          } else {
            it.advance();
          }
        }
        Bytecode::JumpIfNotNull => {
          if !matches!(self.accumulator, Value::Null) {
            let target = it.get_jump_target_offset()?;
            it.set_offset(target)?;
          } else {
            it.advance();
          }
        }
        Bytecode::JumpIfUndefined => {
          if matches!(self.accumulator, Value::Undefined) {
            let target = it.get_jump_target_offset()?;
            it.set_offset(target)?;
          } else {
            it.advance();
          }
        }
        Bytecode::JumpIfNotUndefined => {
          if !matches!(self.accumulator, Value::Undefined) {
            let target = it.get_jump_target_offset()?;
            it.set_offset(target)?;
          } else {
            it.advance();
          }
        }
        Bytecode::JumpIfUndefinedOrNull => {
          if matches!(self.accumulator, Value::Undefined | Value::Null) {
            let target = it.get_jump_target_offset()?;
            it.set_offset(target)?;
          } else {
            it.advance();
          }
        }
        Bytecode::JumpLoop => {
          let target = it.get_jump_target_offset()?;
          it.set_offset(target)?;
        }
        Bytecode::TypeOf => {
          let _slot = it.get_unsigned_operand(0)?;
          self.accumulator = Value::String(self.accumulator.type_of().to_string());
          it.advance();
        }
        Bytecode::Throw | Bytecode::ReThrow => {
          let current_offset = it.current_offset();
          if let Some(target) = Self::find_catch_handler(bytecode_array, current_offset) {
            it.set_offset(target)?;
          } else {
            let msg = if let Value::Object(ref o) = self.accumulator {
              let g = o.read();
              if let Some(m) = g.get("message") {
                format!("Error: {}", m.to_string_lossy())
              } else {
                format!("{:?}", g)
              }
            } else {
              self.accumulator.to_string_lossy()
            };
            return Err(Error::ExecutionError(msg));
          }
        }
        Bytecode::ForInEnumerate => {
          let receiver_reg = it.get_register_operand(0)?;
          let receiver = self.get_register(receiver_reg)?;
          let keys: Vec<Value> = match receiver {
            Value::Object(obj) => {
              let guard = obj.read();
              guard
                .keys()
                .filter(|k| !k.starts_with("__"))
                .cloned()
                .map(Value::String)
                .collect()
            }
            Value::Array(arr) => {
              let len = arr.read().len();
              let mut buf = ItoaBuffer::new();
              (0..len)
                .map(|i| Value::String(buf.format(i).to_string()))
                .collect()
            }
            Value::String(s) => {
              let len = s.chars().count();
              let mut buf = ItoaBuffer::new();
              (0..len)
                .map(|i| Value::String(buf.format(i).to_string()))
                .collect()
            }
            _ => Vec::new(),
          };
          self.accumulator = Value::new_array_with_elements(keys);
          it.advance();
        }
        Bytecode::ForInPrepare => {
          let triple_first = it.get_register_operand(0)?;
          let cache_type_reg = triple_first;
          let cache_array_reg = Register::new(triple_first.index() + 1);
          let cache_length_reg = Register::new(triple_first.index() + 2);
          let _slot = it.get_unsigned_operand(1)?;

          let keys_arr = self.accumulator.clone();
          let len = if let Value::Array(ref arr) = keys_arr {
            arr.read().len() as i32
          } else {
            0
          };
          self.set_register(cache_type_reg, Value::Smi(1))?;
          self.set_register(cache_array_reg, keys_arr)?;
          self.set_register(cache_length_reg, Value::Smi(len))?;
          self.accumulator = Value::Smi(0);
          it.advance();
        }
        Bytecode::ForInNext => {
          let _receiver_reg = it.get_register_operand(0)?;
          let index_reg = it.get_register_operand(1)?;
          let pair_first = it.get_register_operand(2)?;
          let cache_array_reg = Register::new(pair_first.index() + 1);
          let _slot = it.get_unsigned_operand(3)?;

          let index = self.get_register(index_reg)?.to_i32();
          let cache_array = self.get_register(cache_array_reg)?;

          if let Value::Array(ref arr) = cache_array {
            let guard = arr.read();
            if index >= 0 && (index as usize) < guard.len() {
              self.accumulator = guard[index as usize].clone();
            } else {
              self.accumulator = Value::Undefined;
            }
          } else {
            self.accumulator = Value::Undefined;
          }
          it.advance();
        }
        Bytecode::ForInStep => {
          let index_reg = it.get_register_operand(0)?;
          let cur = self.get_register(index_reg)?.to_i32();
          self.set_register(index_reg, Value::Smi(cur.saturating_add(1)))?;
          it.advance();
        }
        Bytecode::JumpIfForInDone | Bytecode::JumpIfForInDoneConstant => {
          let index_reg = it.get_register_operand(1)?;
          let length_reg = it.get_register_operand(2)?;
          let index = self.get_register(index_reg)?.to_i32();
          let length = self.get_register(length_reg)?.to_i32();
          if index >= length {
            let target = it.get_jump_target_offset()?;
            it.set_offset(target)?;
          } else {
            it.advance();
          }
        }
        Bytecode::Return => {
          return Ok(take(&mut self.accumulator));
        }
        _ => {
          it.advance();
        }
      }
    }
    Ok(self.accumulator.clone())
  }
}

/// 解释执行字节码数组函数入口
pub fn execute_bytecode(bytecode_array: &BytecodeArray) -> Result<Value> {
  let mut dispatcher = BytecodeDispatcher::new();
  dispatcher.execute(bytecode_array)
}
