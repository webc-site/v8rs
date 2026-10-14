//! 运行时动态值类型模型
//!
//! 零拷贝与高效现代 Rust 实现，支持基础类型与复合对象。
//! 基于 gxhash 高性能哈希容器与 Arc 紧凑封装，优化内存占用。

use std::{fmt, ptr::fn_addr_eq, sync::Arc, time::Instant};

use fancy_regex::Regex;
use gxhash::HashMap;
use itoa::Buffer;
use parking_lot::RwLock;
use v8_interpreter::{BytecodeDispatcher, TIME_KEY, TIMESTAMP_KEY, Value};
use v8_numbers::{double_to_int32, double_to_uint32};

use crate::{
  error::{Error, Result},
  runtime::{
    ast::{Expr, Stmt},
    bytecode_bridge::{interpreter_value_to_js_value, js_value_to_interpreter_value},
    environment::Environment,
  },
};

/// 计算字符串字符数
#[inline]
pub(crate) fn char_count(s: &str) -> usize {
  if s.is_ascii() {
    s.len()
  } else {
    s.chars().count()
  }
}

/// 将字符偏移量转换为安全的 UTF-8 字节偏移量
#[inline]
pub(crate) fn char_to_byte_index(s: &str, char_idx: usize) -> usize {
  if s.is_ascii() {
    return char_idx.min(s.len());
  }
  s.char_indices()
    .nth(char_idx)
    .map(|(i, _)| i)
    .unwrap_or(s.len())
}

/// 原生上下文状态载荷 (零 dyn 泛型擦除，极速函数指针分派)
pub struct NativeContext {
  payload: *const (),
  call_fn: fn(*const (), &[JsValue]) -> Result<JsValue>,
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
  Ptr(fn(&[JsValue]) -> Result<JsValue>),
  ErrorCtor { proto: Box<JsValue>, name: String },
  ConsoleTime(Arc<RwLock<HashMap<String, Instant>>>),
  ConsoleTimeEnd(Arc<RwLock<HashMap<String, Instant>>>),
  BoxedValueOf(Box<JsValue>),
  BoxedToString(Box<JsValue>),
  ArrayPush(Arc<RwLock<Vec<JsValue>>>),
  ArrayPop(Arc<RwLock<Vec<JsValue>>>),
  ArrayShift(Arc<RwLock<Vec<JsValue>>>),
  ArraySlice(Arc<RwLock<Vec<JsValue>>>),
  ArrayJoin(Arc<RwLock<Vec<JsValue>>>),
  ArrayIndexOf(Arc<RwLock<Vec<JsValue>>>),
  ArraySplice(Arc<RwLock<Vec<JsValue>>>),
  ArrayReverse(Arc<RwLock<Vec<JsValue>>>),
  ArrayToString(Arc<RwLock<Vec<JsValue>>>),
  ObjectToString,
  ObjectValueOf(Arc<RwLock<HashMap<String, JsValue>>>),
  ObjectHasOwnProperty(Arc<RwLock<HashMap<String, JsValue>>>),
  StringCharAt(String),
  StringCharCodeAt(String),
  StringIndexOf(String),
  StringSubstring(String),
  StringSubstr(String),
  StringConcat(String),
  StringReplace(String),
  StringSplit(String),
  StringMatch(String),
  StringToLowerCase(String),
  StringToUpperCase(String),
  StringToString(String),
  NumberToFixed(f64),
  NumberToPrecision(f64),
  NumberToString(f64),
  NumberValueOf(f64),
  BooleanToString(bool),
  BooleanValueOf(bool),
  RegExpTest(Arc<RegExpData>),
  RegExpExec(Arc<RegExpData>),
  InterpNative(v8_interpreter::NativeFn),
  InterpFunction(Arc<v8_interpreter::InterpFunction>),
  Context(Arc<NativeContext>),
}

impl NativeFn {
  #[inline]
  pub fn error_ctor(name: String, proto: JsValue) -> Self {
    Self::ErrorCtor {
      name,
      proto: Box::new(proto),
    }
  }

  #[inline]
  pub fn boxed_value_of(val: JsValue) -> Self {
    Self::BoxedValueOf(Box::new(val))
  }

  #[inline]
  pub fn boxed_to_string(val: JsValue) -> Self {
    Self::BoxedToString(Box::new(val))
  }

  pub fn from_context<T: Send + Sync + 'static>(
    ctx: T,
    func: fn(&T, &[JsValue]) -> Result<JsValue>,
  ) -> Self {
    struct Payload<T> {
      ctx: T,
      func: fn(&T, &[JsValue]) -> Result<JsValue>,
    }
    let boxed = Box::into_raw(Box::new(Payload { ctx, func })) as *const ();
    fn call_trampoline<T>(raw: *const (), args: &[JsValue]) -> Result<JsValue> {
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

  pub fn call(&self, args: &[JsValue]) -> Result<JsValue> {
    match self {
      Self::Ptr(f) => f(args),
      Self::ErrorCtor { proto, name } => {
        let msg = args.first().map(|v| v.to_string()).unwrap_or_default();
        let obj = JsValue::new_object();
        let _ = obj.set_property(PROTO_KEY, (**proto).clone());
        let _ = obj.set_property(NAME_KEY, JsValue::String(name.clone()));
        let _ = obj.set_property(MESSAGE_KEY, JsValue::String(msg));
        Ok(obj)
      }
      Self::ConsoleTime(timers) => {
        let label = args
          .first()
          .map(|v| v.to_string())
          .unwrap_or_else(|| "default".to_string());
        timers.write().insert(label, Instant::now());
        Ok(JsValue::Undefined)
      }
      Self::ConsoleTimeEnd(timers) => {
        let label = args
          .first()
          .map(|v| v.to_string())
          .unwrap_or_else(|| "default".to_string());
        if let Some(start) = timers.write().remove(&label) {
          let elapsed = start.elapsed();
          println!("{label}: {:.3}ms", elapsed.as_secs_f64() * 1000.0);
        }
        Ok(JsValue::Undefined)
      }
      Self::BoxedValueOf(inner) => Ok((**inner).clone()),
      Self::BoxedToString(inner) => Ok(JsValue::String((**inner).to_string())),
      Self::ArrayPush(arr) => Ok(JsValue::array_push_inlined(arr, args)),
      Self::ArrayPop(arr) => Ok(JsValue::array_pop_inlined(arr)),
      Self::ArrayShift(arr) => Ok(JsValue::array_shift_inlined(arr)),
      Self::ArraySlice(arr) => Ok(JsValue::array_slice_inlined(arr, args)),
      Self::ArrayJoin(arr) => Ok(JsValue::array_join_inlined(arr, args)),
      Self::ArrayIndexOf(arr) => Ok(JsValue::array_index_of_inlined(arr, args)),
      Self::ArraySplice(arr) => Ok(JsValue::array_splice_inlined(arr, args)),
      Self::ArrayReverse(arr) => Ok(JsValue::array_reverse_inlined(arr)),
      Self::ArrayToString(arr) => Ok(JsValue::array_to_string_inlined(arr)),
      Self::ObjectToString => Ok(JsValue::String("[object Object]".to_string())),
      Self::ObjectValueOf(obj) => Ok(JsValue::Object(Arc::clone(obj))),
      Self::ObjectHasOwnProperty(obj) => {
        let key = args.first().map(|v| v.to_string()).unwrap_or_default();
        let guard = obj.read();
        Ok(JsValue::Boolean(guard.contains_key(&key)))
      }
      Self::StringCharAt(s) => {
        let idx = args
          .first()
          .map(|v| v.to_int32().max(0) as usize)
          .unwrap_or(0);
        if s.is_ascii() {
          if let Some(&b) = s.as_bytes().get(idx) {
            return Ok(JsValue::String((b as char).to_string()));
          }
          return Ok(JsValue::String(String::new()));
        }
        let ch_str = s
          .chars()
          .nth(idx)
          .map(|c| c.to_string())
          .unwrap_or_default();
        Ok(JsValue::String(ch_str))
      }
      Self::StringCharCodeAt(s) => {
        let idx = args
          .first()
          .map(|v| v.to_int32().max(0) as usize)
          .unwrap_or(0);
        if s.is_ascii() {
          if let Some(&b) = s.as_bytes().get(idx) {
            return Ok(JsValue::Number(b as f64));
          }
          return Ok(JsValue::Number(f64::NAN));
        }
        if let Some(ch) = s.chars().nth(idx) {
          Ok(JsValue::Number(ch as u32 as f64))
        } else {
          Ok(JsValue::Number(f64::NAN))
        }
      }
      Self::StringIndexOf(s) => {
        let needle = args.first().map(|v| v.to_string()).unwrap_or_default();
        let from_c = args
          .get(1)
          .map(|v| v.to_int32().max(0) as usize)
          .unwrap_or(0);
        let from_b = char_to_byte_index(s, from_c);
        let pos = if from_b < s.len() {
          s[from_b..]
            .find(&needle)
            .map(|p| {
              let match_b = from_b + p;
              if s.is_ascii() {
                match_b as f64
              } else {
                s[..match_b].chars().count() as f64
              }
            })
            .unwrap_or(-1.0)
        } else {
          -1.0
        };
        Ok(JsValue::Number(pos))
      }
      Self::StringSubstring(s) => {
        let c_len = char_count(s) as i64;
        let start = args
          .first()
          .map(|v| {
            let n = v.to_int32() as i64;
            if n < 0 {
              (c_len + n).max(0) as usize
            } else {
              (n as usize).min(c_len as usize)
            }
          })
          .unwrap_or(0);
        let end = args
          .get(1)
          .filter(|v| !matches!(v, JsValue::Undefined))
          .map(|v| {
            let n = v.to_int32() as i64;
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
        let from_b = char_to_byte_index(s, from_c);
        let to_b = char_to_byte_index(s, to_c);
        let sub = if from_b <= to_b && from_b < s.len() {
          &s[from_b..to_b.min(s.len())]
        } else {
          ""
        };
        Ok(JsValue::String(sub.to_string()))
      }
      Self::StringSubstr(s) => {
        let c_len = char_count(s) as i64;
        let start = args
          .first()
          .map(|v| {
            let n = v.to_int32() as i64;
            if n < 0 {
              (c_len + n).max(0) as usize
            } else {
              (n as usize).min(c_len as usize)
            }
          })
          .unwrap_or(0);
        let length = args
          .get(1)
          .map(|v| v.to_int32().max(0) as usize)
          .unwrap_or(c_len as usize);
        let from_b = char_to_byte_index(s, start);
        let to_b = char_to_byte_index(s, start + length);
        let sub = if from_b <= to_b && from_b < s.len() {
          &s[from_b..to_b.min(s.len())]
        } else {
          ""
        };
        Ok(JsValue::String(sub.to_string()))
      }
      Self::StringConcat(s) => {
        let mut res = s.clone();
        for a in args {
          res.push_str(&a.to_string());
        }
        Ok(JsValue::String(res))
      }
      Self::StringReplace(s) => {
        let pattern = args.first().cloned().unwrap_or(JsValue::Undefined);
        let replacement = args.get(1).map(|v| v.to_string()).unwrap_or_default();
        match pattern {
          JsValue::RegExp(re) => {
            let rep_cow = if re.global {
              re.compiled.replace_all(s, replacement.as_str())
            } else {
              re.compiled.replace(s, replacement.as_str())
            };
            Ok(JsValue::String(rep_cow.to_string()))
          }
          _ => {
            let pat_str = pattern.to_string();
            if let Some(pos) = s.find(&pat_str) {
              let mut res = String::with_capacity(s.len() + replacement.len());
              res.push_str(&s[..pos]);
              res.push_str(&replacement);
              res.push_str(&s[pos + pat_str.len()..]);
              Ok(JsValue::String(res))
            } else {
              Ok(JsValue::String(s.clone()))
            }
          }
        }
      }
      Self::StringSplit(s) => {
        let sep = args.first().cloned().unwrap_or(JsValue::Undefined);
        let limit = args.get(1).and_then(|v| {
          let n = v.to_int32();
          if n >= 0 { Some(n as usize) } else { None }
        });
        let mut items = Vec::new();
        match sep {
          JsValue::RegExp(re) => {
            for p in re.compiled.split(s).flatten() {
              if let Some(lim) = limit
                && items.len() >= lim
              {
                break;
              }
              items.push(JsValue::String(p.to_string()));
            }
          }
          _ => {
            let sep_str = sep.to_string();
            if sep_str.is_empty() {
              for c in s.chars() {
                if let Some(lim) = limit
                  && items.len() >= lim
                {
                  break;
                }
                items.push(JsValue::String(c.to_string()));
              }
            } else {
              for part in s.split(&sep_str) {
                if let Some(lim) = limit
                  && items.len() >= lim
                {
                  break;
                }
                items.push(JsValue::String(part.to_string()));
              }
            }
          }
        }
        Ok(JsValue::new_array(items))
      }
      Self::StringMatch(s) => {
        let pat = args.first().cloned().unwrap_or(JsValue::Undefined);
        if let JsValue::RegExp(re) = pat {
          if re.global {
            let mut results = Vec::new();
            let mut cur_pos = 0;
            while cur_pos <= s.len() {
              if let Ok(Some(mat)) = re.compiled.find_from_pos(s, cur_pos) {
                results.push(JsValue::String(mat.as_str().to_string()));
                if mat.end() == cur_pos {
                  cur_pos += 1;
                } else {
                  cur_pos = mat.end();
                }
              } else {
                break;
              }
            }
            if results.is_empty() {
              Ok(JsValue::Null)
            } else {
              Ok(JsValue::new_array(results))
            }
          } else {
            let exec_val = JsValue::RegExp(re).get_property("exec")?;
            if let JsValue::Native(exec_fn) = exec_val {
              exec_fn.call(&[JsValue::String(s.clone())])
            } else {
              Ok(JsValue::Null)
            }
          }
        } else {
          Ok(JsValue::Null)
        }
      }
      Self::StringToLowerCase(s) => Ok(JsValue::String(s.to_lowercase())),
      Self::StringToUpperCase(s) => Ok(JsValue::String(s.to_uppercase())),
      Self::StringToString(s) => Ok(JsValue::String(s.clone())),
      Self::NumberToFixed(num) => {
        let digits = args
          .first()
          .map(|v| v.to_int32().clamp(0, 20) as usize)
          .unwrap_or(0);
        Ok(JsValue::String(format!("{num:.digits$}")))
      }
      Self::NumberToPrecision(num) => {
        let prec = args
          .first()
          .map(|v| v.to_int32().clamp(1, 21) as usize)
          .unwrap_or(3);
        Ok(JsValue::String(format!("{num:.prec$}")))
      }
      Self::NumberToString(num) => {
        let radix = args.first().map(|v| v.to_int32()).unwrap_or(10);
        if radix == 16 {
          Ok(JsValue::String(format!("{:x}", *num as i64)))
        } else if radix == 2 {
          Ok(JsValue::String(format!("{:b}", *num as i64)))
        } else if radix == 8 {
          Ok(JsValue::String(format!("{:o}", *num as i64)))
        } else if num.fract() == 0.0 && !num.is_infinite() && !num.is_nan() {
          let mut buf = Buffer::new();
          Ok(JsValue::String(buf.format(*num as i64).to_string()))
        } else {
          Ok(JsValue::String(num.to_string()))
        }
      }
      Self::NumberValueOf(num) => Ok(JsValue::Number(*num)),
      Self::BooleanToString(b) => Ok(JsValue::String(b.to_string())),
      Self::BooleanValueOf(b) => Ok(JsValue::Boolean(*b)),
      Self::RegExpTest(re) => {
        let text = args.first().map(|v| v.to_string()).unwrap_or_default();
        let mut start_pos = 0;
        if re.global {
          start_pos = *re.last_index.read();
          let c_len = char_count(&text);
          if start_pos > c_len {
            *re.last_index.write() = 0;
            return Ok(JsValue::Boolean(false));
          }
        }
        let start_byte = char_to_byte_index(&text, start_pos);
        let haystack = if start_byte < text.len() {
          &text[start_byte..]
        } else {
          ""
        };
        if let Ok(Some(mat)) = re.compiled.find(haystack) {
          if re.global {
            let end_char = if text.is_ascii() {
              start_byte + mat.end()
            } else {
              text[..start_byte + mat.end()].chars().count()
            };
            *re.last_index.write() = end_char;
          }
          Ok(JsValue::Boolean(true))
        } else {
          if re.global {
            *re.last_index.write() = 0;
          }
          Ok(JsValue::Boolean(false))
        }
      }
      Self::RegExpExec(re) => {
        let text = args.first().map(|v| v.to_string()).unwrap_or_default();
        let mut start_pos = 0;
        if re.global {
          start_pos = *re.last_index.read();
          let c_len = char_count(&text);
          if start_pos > c_len {
            *re.last_index.write() = 0;
            return Ok(JsValue::Null);
          }
        }
        let start_byte = char_to_byte_index(&text, start_pos);
        let haystack = if start_byte < text.len() {
          &text[start_byte..]
        } else {
          ""
        };
        match re.compiled.captures(haystack) {
          Ok(Some(caps)) => {
            let mut list = Vec::with_capacity(caps.len());
            for m in caps.iter() {
              if let Some(mat) = m {
                list.push(JsValue::String(mat.as_str().to_string()));
              } else {
                list.push(JsValue::Undefined);
              }
            }
            if let Some(whole) = caps.get(0) {
              let end_pos = if text.is_ascii() {
                start_byte + whole.end()
              } else {
                text[..start_byte + whole.end()].chars().count()
              };
              let match_start = if text.is_ascii() {
                start_byte + whole.start()
              } else {
                text[..start_byte + whole.start()].chars().count()
              };
              if re.global {
                *re.last_index.write() = end_pos;
              }
              let res_arr = JsValue::new_array(list);
              let _ = res_arr.set_property("index", JsValue::Number(match_start as f64));
              let _ = res_arr.set_property("input", JsValue::String(text));
              Ok(res_arr)
            } else {
              Ok(JsValue::Null)
            }
          }
          _ => {
            if re.global {
              *re.last_index.write() = 0;
            }
            Ok(JsValue::Null)
          }
        }
      }
      Self::InterpNative(native) => {
        let interp_args: Vec<v8_interpreter::Value> = args
          .iter()
          .map(|a| js_value_to_interpreter_value(a).unwrap_or(Value::Undefined))
          .collect();
        match native.call(&interp_args) {
          Ok(res) => Ok(interpreter_value_to_js_value(&res)),
          Err(e) => Err(Error::RuntimeError(e.to_string())),
        }
      }
      Self::InterpFunction(func) => {
        let mut d = BytecodeDispatcher::new();
        let interp_args: Vec<v8_interpreter::Value> = args
          .iter()
          .map(|a| js_value_to_interpreter_value(a).unwrap_or(Value::Undefined))
          .collect();
        match d.call_function(
          &Value::Function(Arc::clone(func), Arc::new(RwLock::new(HashMap::default()))),
          &Value::Undefined,
          &interp_args,
        ) {
          Ok(res) => Ok(interpreter_value_to_js_value(&res)),
          Err(e) => Err(Error::RuntimeError(e.to_string())),
        }
      }
      Self::Context(ctx) => (ctx.call_fn)(ctx.payload, args),
    }
  }
}

impl PartialEq for NativeFn {
  fn eq(&self, other: &Self) -> bool {
    match (self, other) {
      (Self::Ptr(a), Self::Ptr(b)) => fn_addr_eq(*a, *b),
      (Self::ObjectToString, Self::ObjectToString) => true,
      (Self::ArrayPush(a), Self::ArrayPush(b))
      | (Self::ArrayPop(a), Self::ArrayPop(b))
      | (Self::ArrayShift(a), Self::ArrayShift(b))
      | (Self::ArraySlice(a), Self::ArraySlice(b))
      | (Self::ArrayJoin(a), Self::ArrayJoin(b))
      | (Self::ArrayIndexOf(a), Self::ArrayIndexOf(b))
      | (Self::ArraySplice(a), Self::ArraySplice(b))
      | (Self::ArrayReverse(a), Self::ArrayReverse(b))
      | (Self::ArrayToString(a), Self::ArrayToString(b)) => Arc::ptr_eq(a, b),
      (Self::ObjectValueOf(a), Self::ObjectValueOf(b))
      | (Self::ObjectHasOwnProperty(a), Self::ObjectHasOwnProperty(b)) => Arc::ptr_eq(a, b),
      (Self::StringCharAt(a), Self::StringCharAt(b))
      | (Self::StringCharCodeAt(a), Self::StringCharCodeAt(b))
      | (Self::StringIndexOf(a), Self::StringIndexOf(b))
      | (Self::StringSubstring(a), Self::StringSubstring(b))
      | (Self::StringSubstr(a), Self::StringSubstr(b))
      | (Self::StringConcat(a), Self::StringConcat(b))
      | (Self::StringReplace(a), Self::StringReplace(b))
      | (Self::StringSplit(a), Self::StringSplit(b))
      | (Self::StringMatch(a), Self::StringMatch(b))
      | (Self::StringToLowerCase(a), Self::StringToLowerCase(b))
      | (Self::StringToUpperCase(a), Self::StringToUpperCase(b))
      | (Self::StringToString(a), Self::StringToString(b)) => a == b,
      (Self::NumberToFixed(a), Self::NumberToFixed(b))
      | (Self::NumberToPrecision(a), Self::NumberToPrecision(b))
      | (Self::NumberToString(a), Self::NumberToString(b))
      | (Self::NumberValueOf(a), Self::NumberValueOf(b)) => a == b,
      (Self::BooleanToString(a), Self::BooleanToString(b))
      | (Self::BooleanValueOf(a), Self::BooleanValueOf(b)) => a == b,
      (Self::RegExpTest(a), Self::RegExpTest(b)) | (Self::RegExpExec(a), Self::RegExpExec(b)) => {
        Arc::ptr_eq(a, b)
      }
      (Self::InterpNative(a), Self::InterpNative(b)) => a == b,
      (Self::InterpFunction(a), Self::InterpFunction(b)) => Arc::ptr_eq(a, b),
      (Self::Context(a), Self::Context(b)) => Arc::ptr_eq(a, b),
      _ => false,
    }
  }
}

impl fmt::Debug for NativeFn {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "[NativeFunction]")
  }
}

impl From<fn(&[JsValue]) -> Result<JsValue>> for NativeFn {
  #[inline]
  fn from(f: fn(&[JsValue]) -> Result<JsValue>) -> Self {
    Self::Ptr(f)
  }
}

/// 函数静态只读元数据 (对标 V8 SharedFunctionInfo，全局共享 AST 与分析元数据，彻底消除运行时深拷贝)
#[derive(Clone, Debug)]
pub struct SharedFunctionInfo {
  /// 函数名称
  pub name: Option<String>,
  /// 形参列表
  pub params: Vec<String>,
  /// 语法树函数体语句列表
  pub body: Vec<Stmt>,
  /// 预分析缓存的提升局部变量名
  pub hoisted_vars: Vec<String>,
  /// 预分析缓存的内部函数声明 (静态共享元数据引用)
  pub hoisted_funcs: Vec<Arc<SharedFunctionInfo>>,
  /// 静态分析标记：函数体内是否引用了 arguments
  pub uses_arguments: bool,
}

impl SharedFunctionInfo {
  /// 创建并预分析函数静态元数据 (收集局部提升变量、内部函数与 arguments 引用)
  pub fn new(name: Option<String>, params: Vec<String>, body: Vec<Stmt>) -> Self {
    let mut hoisted_vars = Vec::new();
    let mut hoisted_funcs = Vec::new();
    collect_hoisted(&body, &mut hoisted_vars, &mut hoisted_funcs);
    let uses_arguments = check_uses_arguments(&body);
    Self {
      name,
      params,
      body,
      hoisted_vars,
      hoisted_funcs,
      uses_arguments,
    }
  }

  /// 创建空元数据 (供原生函数使用)
  pub fn new_empty(name: Option<String>) -> Self {
    Self {
      name,
      params: Vec::new(),
      body: Vec::new(),
      hoisted_vars: Vec::new(),
      hoisted_funcs: Vec::new(),
      uses_arguments: false,
    }
  }
}

/// 预分析缓存的内部函数声明
pub type HoistedFunction = Arc<SharedFunctionInfo>;

#[inline]
fn add_hoisted_var(vars: &mut Vec<String>, name: &str) {
  if !vars.iter().any(|v| v == name) {
    vars.push(name.to_string());
  }
}

/// 收集语法树中需提升的局部变量与内部函数声明 (线性扫描去重，消除额外堆分配)
pub fn collect_hoisted(
  stmts: &[Stmt],
  hoisted_vars: &mut Vec<String>,
  hoisted_funcs: &mut Vec<Arc<SharedFunctionInfo>>,
) {
  for stmt in stmts {
    collect_hoisted_stmt(stmt, hoisted_vars, hoisted_funcs);
  }
}

/// 递归分析单条语句中提升的声明 (不进入内层函数体，保持词法边界)
pub fn collect_hoisted_stmt(
  stmt: &Stmt,
  hoisted_vars: &mut Vec<String>,
  hoisted_funcs: &mut Vec<Arc<SharedFunctionInfo>>,
) {
  match stmt {
    Stmt::FunctionDecl(shared) => {
      hoisted_funcs.push(Arc::clone(shared));
    }
    Stmt::VarDecl { name, .. } => add_hoisted_var(hoisted_vars, name),
    Stmt::VarDecls(decls) => {
      for (name, _) in decls {
        add_hoisted_var(hoisted_vars, name);
      }
    }
    Stmt::Block(inner) => {
      collect_hoisted(inner, hoisted_vars, hoisted_funcs);
    }
    Stmt::If {
      then_branch,
      else_branch,
      ..
    } => {
      collect_hoisted_stmt(then_branch, hoisted_vars, hoisted_funcs);
      if let Some(eb) = else_branch {
        collect_hoisted_stmt(eb, hoisted_vars, hoisted_funcs);
      }
    }
    Stmt::While { body, .. } | Stmt::DoWhile { body, .. } => {
      collect_hoisted_stmt(body, hoisted_vars, hoisted_funcs);
    }
    Stmt::For { init, body, .. } => {
      if let Some(in_stmt) = init {
        collect_hoisted_stmt(in_stmt, hoisted_vars, hoisted_funcs);
      }
      collect_hoisted_stmt(body, hoisted_vars, hoisted_funcs);
    }
    Stmt::ForIn {
      variable,
      is_var,
      body,
      ..
    } => {
      if *is_var {
        add_hoisted_var(hoisted_vars, variable);
      }
      collect_hoisted_stmt(body, hoisted_vars, hoisted_funcs);
    }
    Stmt::Switch { cases, .. } => {
      for c in cases {
        collect_hoisted(&c.body, hoisted_vars, hoisted_funcs);
      }
    }
    Stmt::Try {
      block,
      catch_block,
      finally_block,
      ..
    } => {
      collect_hoisted(block, hoisted_vars, hoisted_funcs);
      if let Some(cb) = catch_block {
        collect_hoisted(cb, hoisted_vars, hoisted_funcs);
      }
      if let Some(fb) = finally_block {
        collect_hoisted(fb, hoisted_vars, hoisted_funcs);
      }
    }
    _ => {}
  }
}

/// 静态分析函数体内是否引用了 arguments (对标 V8 静态作用域分析，避免无谓的 arguments 对象与锁分配)
pub fn check_uses_arguments(body: &[Stmt]) -> bool {
  body.iter().any(stmt_uses_arguments)
}

fn stmt_uses_arguments(stmt: &Stmt) -> bool {
  match stmt {
    Stmt::Expr(expr) => expr_uses_arguments(expr),
    Stmt::VarDecl { name, init } => {
      name == "arguments" || init.as_ref().is_some_and(expr_uses_arguments)
    }
    Stmt::VarDecls(decls) => decls
      .iter()
      .any(|(name, init)| name == "arguments" || init.as_ref().is_some_and(expr_uses_arguments)),
    Stmt::Block(stmts) => stmts.iter().any(stmt_uses_arguments),
    Stmt::If {
      cond,
      then_branch,
      else_branch,
    } => {
      expr_uses_arguments(cond)
        || stmt_uses_arguments(then_branch)
        || else_branch.as_ref().is_some_and(|b| stmt_uses_arguments(b))
    }
    Stmt::While { cond, body } | Stmt::DoWhile { cond, body } => {
      expr_uses_arguments(cond) || stmt_uses_arguments(body)
    }
    Stmt::For {
      init,
      cond,
      step,
      body,
    } => {
      init.as_ref().is_some_and(|s| stmt_uses_arguments(s))
        || cond.as_ref().is_some_and(expr_uses_arguments)
        || step.as_ref().is_some_and(expr_uses_arguments)
        || stmt_uses_arguments(body)
    }
    Stmt::ForIn {
      variable,
      object,
      body,
      ..
    } => variable == "arguments" || expr_uses_arguments(object) || stmt_uses_arguments(body),
    Stmt::Return(expr_opt) => expr_opt.as_ref().is_some_and(expr_uses_arguments),
    Stmt::Switch {
      discriminant,
      cases,
    } => {
      expr_uses_arguments(discriminant)
        || cases.iter().any(|c| {
          c.test.as_ref().is_some_and(expr_uses_arguments) || c.body.iter().any(stmt_uses_arguments)
        })
    }
    Stmt::Try {
      block,
      catch_param,
      catch_block,
      finally_block,
    } => {
      block.iter().any(stmt_uses_arguments)
        || catch_param.as_deref() == Some("arguments")
        || catch_block
          .as_ref()
          .is_some_and(|b| b.iter().any(stmt_uses_arguments))
        || finally_block
          .as_ref()
          .is_some_and(|b| b.iter().any(stmt_uses_arguments))
    }
    Stmt::Throw(expr) => expr_uses_arguments(expr),
    // 内部函数声明有其自身的作用域与 arguments，不污染外层作用域
    Stmt::FunctionDecl(_) => false,
    Stmt::Break | Stmt::Continue | Stmt::Empty => false,
  }
}

fn expr_uses_arguments(expr: &Expr) -> bool {
  match expr {
    Expr::Identifier(name) => name == "arguments",
    Expr::Unary { expr, .. } | Expr::Typeof(expr) => expr_uses_arguments(expr),
    Expr::Binary { left, right, .. } => expr_uses_arguments(left) || expr_uses_arguments(right),
    Expr::Assign { target, value, .. } => expr_uses_arguments(target) || expr_uses_arguments(value),
    Expr::Call { callee, args } => {
      if let Expr::Identifier(id) = &**callee
        && id == "eval"
      {
        return true;
      }
      expr_uses_arguments(callee) || args.iter().any(expr_uses_arguments)
    }
    Expr::Member { object, .. } => expr_uses_arguments(object),
    Expr::Index { object, index } => expr_uses_arguments(object) || expr_uses_arguments(index),
    Expr::Array(items) => items.iter().any(expr_uses_arguments),
    Expr::Object(entries) => entries.iter().any(|(_, v)| expr_uses_arguments(v)),
    Expr::Conditional {
      condition,
      consequent,
      alternate,
    } => {
      expr_uses_arguments(condition)
        || expr_uses_arguments(consequent)
        || expr_uses_arguments(alternate)
    }
    Expr::New { callee, args } => {
      expr_uses_arguments(callee) || args.iter().any(expr_uses_arguments)
    }
    // 内部函数表达式有其自身的作用域与 arguments，不污染外层作用域
    Expr::FunctionExpr(_) => false,
    Expr::Literal(_) | Expr::This => false,
  }
}

/// 闭包函数定义 (对标 V8 JSFunction，持有全局共享静态元数据与词法作用域)
#[derive(Clone)]
pub struct FunctionData {
  /// 静态共享元数据 (对标 V8 SharedFunctionInfo)
  pub shared: Arc<SharedFunctionInfo>,
  /// 词法作用域闭包环境 (Context)
  pub closure: Arc<Environment>,
  /// 函数对象自有属性
  pub properties: Arc<RwLock<HashMap<String, JsValue>>>,
  /// 原生函数实现
  pub native_impl: Option<NativeFn>,
}

/// 正则表达式数据封装
#[derive(Clone)]
pub struct RegExpData {
  pub pattern: String,
  pub flags: String,
  pub global: bool,
  pub ignore_case: bool,
  pub multiline: bool,
  pub compiled: fancy_regex::Regex,
  pub last_index: Arc<RwLock<usize>>,
}

impl RegExpData {
  #[inline]
  pub fn as_str(&self) -> &str {
    &self.pattern
  }

  #[inline]
  pub fn is_match(&self, text: &str) -> bool {
    self.compiled.is_match(text).unwrap_or(false)
  }

  #[inline]
  pub fn captures<'a>(&self, text: &'a str) -> Option<fancy_regex::Captures<'a, str>> {
    self.compiled.captures(text).ok().flatten()
  }
}

impl FunctionData {
  #[inline]
  pub fn name(&self) -> Option<&str> {
    self.shared.name.as_deref()
  }

  #[inline]
  pub fn params(&self) -> &[String] {
    &self.shared.params
  }

  #[inline]
  pub fn body(&self) -> &[Stmt] {
    &self.shared.body
  }

  /// 创建新的函数闭包数据，自动绑定默认原型对象并预先分析变量提升
  pub fn new(
    name: Option<String>,
    params: Vec<String>,
    body: Vec<Stmt>,
    closure: Arc<Environment>,
  ) -> Self {
    let shared = Arc::new(SharedFunctionInfo::new(name, params, body));
    let mut props = HashMap::default();
    let proto = JsValue::new_object();
    if let Some(obj_proto) = closure.get_object_prototype() {
      let _ = proto.set_property(PROTO_KEY, obj_proto);
    }
    props.insert(PROTOTYPE_KEY.to_string(), proto);

    Self {
      shared,
      closure,
      properties: Arc::new(RwLock::new(props)),
      native_impl: None,
    }
  }

  /// 构造已知静态元数据与可选原型引用的函数对象 (零 AST 递归重入，零重复 Object 查找)
  pub fn from_shared_with_proto(
    shared: Arc<SharedFunctionInfo>,
    closure: Arc<Environment>,
    obj_proto: Option<JsValue>,
  ) -> JsValue {
    let proto = if let Some(p) = obj_proto {
      let mut map = HashMap::default();
      map.insert(PROTO_KEY.to_string(), p);
      JsValue::Object(Arc::new(RwLock::new(map)))
    } else {
      JsValue::new_object()
    };
    let mut props = HashMap::default();
    props.insert(PROTOTYPE_KEY.to_string(), proto.clone());

    let func_data = Arc::new(Self {
      shared,
      closure,
      properties: Arc::new(RwLock::new(props)),
      native_impl: None,
    });
    let func_val = JsValue::Function(func_data);
    let _ = proto.set_property(CONSTRUCTOR_KEY, func_val.clone());
    func_val
  }

  /// 构造已知静态元数据的函数对象
  pub fn from_shared(shared: Arc<SharedFunctionInfo>, closure: Arc<Environment>) -> JsValue {
    let obj_proto = closure.get_object_prototype();
    Self::from_shared_with_proto(shared, closure, obj_proto)
  }

  /// 构造已知预分析提升元数据与可选原型引用的函数对象 (兼容接口)
  pub fn new_preanalyzed_with_proto(
    shared: Arc<SharedFunctionInfo>,
    closure: Arc<Environment>,
    obj_proto: Option<JsValue>,
  ) -> JsValue {
    Self::from_shared_with_proto(shared, closure, obj_proto)
  }

  /// 创建带原生实现与自有属性表的内置函数对象
  pub fn new_native(name: Option<String>, closure: Arc<Environment>, native: NativeFn) -> Self {
    let mut props = HashMap::default();
    props.insert(PROTOTYPE_KEY.to_string(), JsValue::new_object());
    Self {
      shared: Arc::new(SharedFunctionInfo::new_empty(name)),
      closure,
      properties: Arc::new(RwLock::new(props)),
      native_impl: Some(native),
    }
  }

  /// 构造完整的函数对象并自动双向绑定 prototype.constructor
  pub fn new_function(
    name: Option<String>,
    params: Vec<String>,
    body: Vec<Stmt>,
    closure: Arc<Environment>,
  ) -> JsValue {
    let shared = Arc::new(SharedFunctionInfo::new(name, params, body));
    Self::from_shared(shared, closure)
  }

  /// 获取函数属性
  pub fn get_property(&self, prop: &str) -> Result<JsValue> {
    if prop == LENGTH_KEY {
      return Ok(JsValue::Number(self.shared.params.len() as f64));
    }
    if prop == NAME_KEY {
      return Ok(JsValue::String(
        self.shared.name.clone().unwrap_or_default(),
      ));
    }
    let guard = self.properties.read();
    if let Some(val) = guard.get(prop) {
      return Ok(val.clone());
    }
    if let Some(proto) = guard.get(PROTO_KEY) {
      let proto_clone = proto.clone();
      drop(guard);
      let res = proto_clone.get_property(prop)?;
      if res != JsValue::Undefined {
        return Ok(res);
      }
    } else {
      drop(guard);
    }

    if let Some(fn_ctor) = self.closure.get("Function")
      && let Ok(fn_proto) = fn_ctor.get_property(PROTOTYPE_KEY)
      && let Ok(res) = fn_proto.get_property(prop)
      && res != JsValue::Undefined
    {
      return Ok(res);
    }
    if let Some(obj_proto) = self.closure.get_object_prototype() {
      return obj_proto.get_property(prop);
    }
    Ok(JsValue::Undefined)
  }

  /// 设置函数属性
  pub fn set_property(&self, prop: &str, val: JsValue) -> Result<()> {
    let mut guard = self.properties.write();
    if let Some(slot) = guard.get_mut(prop) {
      *slot = val;
    } else {
      guard.insert(prop.to_string(), val);
    }
    Ok(())
  }
}

pub const CONSTRUCTOR_KEY: &str = "constructor";
pub const VALUE_OF_KEY: &str = "valueOf";
pub const TO_STRING_KEY: &str = "toString";
pub const NAME_KEY: &str = "name";
pub const MESSAGE_KEY: &str = "message";
pub const GET_SYM_TO_PRIMITIVE: &str = "@@get:@@symbol:Symbol.toPrimitive";

/// 对象原型属性键
pub const PROTO_KEY: &str = "__proto__";
/// 原型链属性键
pub const PROTOTYPE_KEY: &str = "prototype";
/// 符号内部前缀
pub const SYMBOL_PREFIX: &str = "@@symbol:";
/// Getter 内部前缀
pub const GETTER_PREFIX: &str = "@@get:";
/// Setter 内部前缀
pub const SETTER_PREFIX: &str = "@@set:";
/// Symbol.toPrimitive 内部属性键
pub const SYM_TO_PRIMITIVE: &str = "@@symbol:Symbol.toPrimitive";

pub const LENGTH_KEY: &str = "length";
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
pub const SUBSTR_KEY: &str = "substr";
pub const CONCAT_KEY: &str = "concat";
pub const TO_LOWER_CASE_KEY: &str = "toLowerCase";
pub const TO_UPPER_CASE_KEY: &str = "toUpperCase";
pub const TO_FIXED_KEY: &str = "toFixed";
pub const TO_PRECISION_KEY: &str = "toPrecision";
pub const HAS_OWN_PROPERTY_KEY: &str = "hasOwnProperty";
pub const SOURCE_KEY: &str = "source";
pub const FLAGS_KEY: &str = "flags";
pub const GLOBAL_KEY: &str = "global";
pub const LAST_INDEX_KEY: &str = "lastIndex";
pub const INDEX_KEY: &str = "index";
pub const INPUT_KEY: &str = "input";

/// 格式化 Symbol 属性键
#[inline]
pub fn format_symbol_key(desc: &str) -> String {
  let mut s = String::with_capacity(SYMBOL_PREFIX.len() + desc.len());
  s.push_str(SYMBOL_PREFIX);
  s.push_str(desc);
  s
}

/// 格式化 Getter 属性键
#[inline]
pub fn format_getter_key(prop: &str) -> String {
  let mut s = String::with_capacity(GETTER_PREFIX.len() + prop.len());
  s.push_str(GETTER_PREFIX);
  s.push_str(prop);
  s
}

/// 格式化 Setter 属性键
#[inline]
pub fn format_setter_key(prop: &str) -> String {
  let mut s = String::with_capacity(SETTER_PREFIX.len() + prop.len());
  s.push_str(SETTER_PREFIX);
  s.push_str(prop);
  s
}

/// ECMAScript PreferredType 转换提示
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PreferredType {
  Default,
  String,
  Number,
}

impl PreferredType {
  #[inline]
  pub fn as_str(self) -> &'static str {
    match self {
      Self::Default => "default",
      Self::String => "string",
      Self::Number => "number",
    }
  }

  #[inline]
  pub fn to_js_value(self) -> JsValue {
    match self {
      Self::Default => JsValue::String("default".to_string()),
      Self::String => JsValue::String("string".to_string()),
      Self::Number => JsValue::String("number".to_string()),
    }
  }
}

/// 运行时动态值类型
#[derive(Clone)]
pub enum JsValue {
  Undefined,
  Null,
  Boolean(bool),
  Number(f64),
  BigInt(i128),
  String(String),
  Symbol(String),
  Array(Arc<RwLock<Vec<JsValue>>>),
  Object(Arc<RwLock<HashMap<String, JsValue>>>),
  RegExp(Arc<RegExpData>),
  Function(Arc<FunctionData>),
  Native(NativeFn),
}

impl PartialEq for JsValue {
  fn eq(&self, other: &Self) -> bool {
    match (self, other) {
      (Self::Undefined, Self::Undefined) | (Self::Null, Self::Null) => true,
      (Self::Boolean(a), Self::Boolean(b)) => a == b,
      (Self::Number(a), Self::Number(b)) => {
        if a.is_nan() || b.is_nan() {
          false
        } else {
          a == b
        }
      }
      (Self::BigInt(a), Self::BigInt(b)) => a == b,
      (Self::String(a), Self::String(b)) => a == b,
      (Self::Symbol(a), Self::Symbol(b)) => a == b,
      (Self::Array(a), Self::Array(b)) => Arc::ptr_eq(a, b),
      (Self::Object(a), Self::Object(b)) => Arc::ptr_eq(a, b),
      (Self::Function(a), Self::Function(b)) => Arc::ptr_eq(a, b),
      (Self::Native(a), Self::Native(b)) => a == b,
      (Self::RegExp(a), Self::RegExp(b)) => Arc::ptr_eq(a, b),
      _ => false,
    }
  }
}

impl fmt::Debug for JsValue {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Self::Undefined => write!(f, "undefined"),
      Self::Null => write!(f, "null"),
      Self::Boolean(b) => write!(f, "{b}"),
      Self::Number(n) => {
        if n.fract() == 0.0 && !n.is_infinite() && !n.is_nan() {
          let mut buf = Buffer::new();
          write!(f, "{}", buf.format(*n as i64))
        } else {
          write!(f, "{n}")
        }
      }
      Self::BigInt(n) => write!(f, "{n}n"),
      Self::String(s) => write!(f, "\"{s}\""),
      Self::Symbol(s) => write!(f, "Symbol({s})"),
      Self::Array(arr) => {
        let guard = arr.read();
        write!(f, "{guard:?}")
      }
      Self::Object(obj) => {
        let guard = obj.read();
        write!(f, "{guard:?}")
      }
      Self::Function(func) => {
        write!(f, "[Function: {}]", func.name().unwrap_or("anonymous"))
      }
      Self::RegExp(re) => write!(f, "/{}/", re.as_str()),
      Self::Native(_) => write!(f, "[NativeFunction]"),
    }
  }
}

impl fmt::Display for JsValue {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Self::Undefined => write!(f, "undefined"),
      Self::Null => write!(f, "null"),
      Self::Boolean(b) => write!(f, "{b}"),
      Self::Number(n) => {
        if n.fract() == 0.0 && !n.is_infinite() && !n.is_nan() {
          let mut buf = Buffer::new();
          write!(f, "{}", buf.format(*n as i64))
        } else {
          write!(f, "{n}")
        }
      }
      Self::BigInt(n) => write!(f, "{n}"),
      Self::String(s) => write!(f, "{s}"),
      Self::Symbol(s) => write!(f, "Symbol({s})"),
      Self::Array(arr) => {
        let guard = arr.read();
        write!(f, "[")?;
        for (i, item) in guard.iter().enumerate() {
          if i > 0 {
            write!(f, ", ")?;
          }
          write!(f, "{item}")?;
        }
        write!(f, "]")
      }
      Self::Object(obj) => {
        let guard = obj.read();
        write!(f, "{{ ")?;
        for (i, (k, v)) in guard.iter().enumerate() {
          if i > 0 {
            write!(f, ", ")?;
          }
          write!(f, "{k}: {v}")?;
        }
        write!(f, " }}")
      }
      Self::Function(func) => {
        write!(f, "[Function: {}]", func.name().unwrap_or("anonymous"))
      }
      Self::RegExp(re) => write!(f, "/{}/{}", re.pattern, re.flags),
      Self::Native(_) => write!(f, "[NativeFunction]"),
    }
  }
}

impl JsValue {
  /// 判断是否为 ECMAScript 原始值类型 (Primitive)
  #[inline]
  pub fn is_primitive(&self) -> bool {
    matches!(
      self,
      Self::Undefined
        | Self::Null
        | Self::Boolean(_)
        | Self::Number(_)
        | Self::BigInt(_)
        | Self::String(_)
        | Self::Symbol(_)
    )
  }

  /// 判断是否为可调用对象 (Callable)
  #[inline]
  pub fn is_callable(&self) -> bool {
    matches!(self, Self::Function(_) | Self::Native(_))
  }

  /// 判断是否为引用对象类型 (Object-like)
  #[inline]
  pub fn is_object_like(&self) -> bool {
    !self.is_primitive()
  }

  /// 创建空对象
  pub fn new_object() -> Self {
    Self::Object(Arc::new(RwLock::new(HashMap::default())))
  }

  /// 创建数组
  pub fn new_array(items: Vec<JsValue>) -> Self {
    Self::Array(Arc::new(RwLock::new(items)))
  }

  /// 对标 V8 Builtin Inlining: Array.prototype.push 原生就地内联
  #[inline]
  pub fn array_push_inlined(arr: &RwLock<Vec<JsValue>>, args: &[JsValue]) -> JsValue {
    let mut guard = arr.write();
    guard.extend(args.iter().cloned());
    JsValue::Number(guard.len() as f64)
  }

  /// 对标 V8 Builtin Inlining: Array.prototype.pop 原生就地内联
  #[inline]
  pub fn array_pop_inlined(arr: &RwLock<Vec<JsValue>>) -> JsValue {
    let mut guard = arr.write();
    guard.pop().unwrap_or(JsValue::Undefined)
  }

  /// 对标 V8 Builtin Inlining: Array.prototype.shift 原生就地内联
  #[inline]
  pub fn array_shift_inlined(arr: &RwLock<Vec<JsValue>>) -> JsValue {
    let mut guard = arr.write();
    if guard.is_empty() {
      JsValue::Undefined
    } else {
      guard.remove(0)
    }
  }

  /// 对标 V8 Builtin Inlining: Array.prototype.reverse 原生就地内联
  #[inline]
  pub fn array_reverse_inlined(arr: &Arc<RwLock<Vec<JsValue>>>) -> JsValue {
    let mut guard = arr.write();
    guard.reverse();
    JsValue::Array(Arc::clone(arr))
  }

  /// 对标 V8 Builtin Inlining: Array.prototype.splice 原生就地内联
  pub fn array_splice_inlined(arr: &RwLock<Vec<JsValue>>, args: &[JsValue]) -> JsValue {
    let mut guard = arr.write();
    let len = guard.len() as i64;
    let start = args
      .first()
      .map(|v| {
        let n = v.to_int32() as i64;
        if n < 0 {
          (len + n).max(0) as usize
        } else {
          (n as usize).min(guard.len())
        }
      })
      .unwrap_or(0);
    let delete_count = args
      .get(1)
      .map(|v| {
        let n = v.to_int32();
        if n < 0 {
          0
        } else {
          (n as usize).min(guard.len().saturating_sub(start))
        }
      })
      .unwrap_or(guard.len().saturating_sub(start));
    let removed: Vec<JsValue> = if args.len() > 2 {
      guard
        .splice(start..start + delete_count, args[2..].iter().cloned())
        .collect()
    } else {
      guard.drain(start..start + delete_count).collect()
    };
    JsValue::new_array(removed)
  }

  /// 对标 V8 Builtin Inlining: Array.prototype.slice 原生就地内联
  pub fn array_slice_inlined(arr: &RwLock<Vec<JsValue>>, args: &[JsValue]) -> JsValue {
    let guard = arr.read();
    let len = guard.len() as i64;
    let start = args
      .first()
      .map(|v| {
        let n = v.to_int32() as i64;
        if n < 0 {
          (len + n).max(0) as usize
        } else {
          (n as usize).min(guard.len())
        }
      })
      .unwrap_or(0);
    let end = args
      .get(1)
      .map(|v| {
        let n = v.to_int32() as i64;
        if n < 0 {
          (len + n).max(0) as usize
        } else {
          (n as usize).min(guard.len())
        }
      })
      .unwrap_or(guard.len());
    let sub = if start <= end && start < guard.len() {
      guard[start..end].to_vec()
    } else {
      Vec::new()
    };
    JsValue::new_array(sub)
  }

  /// 对标 V8 Builtin Inlining: Array.prototype.indexOf 原生就地内联
  pub fn array_index_of_inlined(arr: &RwLock<Vec<JsValue>>, args: &[JsValue]) -> JsValue {
    let target = args.first().unwrap_or(&JsValue::Undefined);
    let from_idx = args
      .get(1)
      .map(|v| v.to_int32().max(0) as usize)
      .unwrap_or(0);
    let guard = arr.read();
    let pos = guard
      .iter()
      .enumerate()
      .skip(from_idx)
      .find(|(_, v)| *v == target)
      .map(|(i, _)| i as f64)
      .unwrap_or(-1.0);
    JsValue::Number(pos)
  }

  /// 对标 V8 Builtin Inlining: Array.prototype.join 原生就地内联
  pub fn array_join_inlined(arr: &RwLock<Vec<JsValue>>, args: &[JsValue]) -> JsValue {
    let sep_holder;
    let sep: &str = match args.first() {
      None | Some(JsValue::Undefined) => ",",
      Some(JsValue::String(s)) => s.as_str(),
      Some(other) => {
        sep_holder = other.to_js_string();
        &sep_holder
      }
    };
    let guard = arr.read();
    let mut joined = String::new();
    for (i, v) in guard.iter().enumerate() {
      if i > 0 {
        joined.push_str(sep);
      }
      match v {
        JsValue::Undefined | JsValue::Null => {}
        JsValue::String(s) => joined.push_str(s),
        _ => joined.push_str(&v.to_js_string()),
      }
    }
    JsValue::String(joined)
  }

  /// 对标 V8 Builtin Inlining: Array.prototype.toString 原生就地内联
  #[inline]
  pub fn array_to_string_inlined(arr: &RwLock<Vec<JsValue>>) -> JsValue {
    Self::array_join_inlined(arr, &[])
  }

  /// 编译并创建正则表达式
  pub fn new_regexp(pattern: &str, flags: &str) -> Result<Self> {
    let global = flags.contains('g');
    let ignore_case = flags.contains('i');
    let multiline = flags.contains('m');

    let mut normalized = String::with_capacity(pattern.len() + 8);
    let mut chars = pattern.chars().peekable();
    let mut in_char_class = false;
    while let Some(c) = chars.next() {
      if c == '\\' {
        if chars.peek() == Some(&'x') {
          chars.next();
          let h1 = chars.next();
          let h2 = chars.next();
          if let (Some(c1), Some(c2)) = (h1, h2) {
            if let Ok(byte_val) = u8::from_str_radix(&format!("{c1}{c2}"), 16) {
              if byte_val >= 0x80 {
                normalized.push_str(&format!("\\u{{{:04x}}}", byte_val));
              } else {
                normalized.push_str(&format!("\\x{c1}{c2}"));
              }
            } else {
              normalized.push_str(&format!("\\x{c1}{c2}"));
            }
          } else {
            normalized.push_str("\\x");
            if let Some(c1) = h1 {
              normalized.push(c1);
            }
          }
        } else if let Some(escaped) = chars.next() {
          normalized.push('\\');
          normalized.push(escaped);
        } else {
          normalized.push('\\');
        }
      } else if c == '[' {
        if in_char_class {
          normalized.push_str("\\[");
        } else {
          in_char_class = true;
          normalized.push('[');
        }
      } else if c == ']' {
        in_char_class = false;
        normalized.push(']');
      } else {
        normalized.push(c);
      }
    }

    let mut flag_prefix = String::new();
    if ignore_case {
      flag_prefix.push('i');
    }
    if multiline {
      flag_prefix.push('m');
    }
    let pat_to_compile = if flag_prefix.is_empty() {
      normalized
    } else {
      format!("(?{flag_prefix}){normalized}")
    };

    let compiled = Regex::new(&pat_to_compile)
      .map_err(|e| Error::RuntimeError(format!("Invalid RegExp '{pattern}': {e}")))?;

    Ok(Self::RegExp(Arc::new(RegExpData {
      pattern: pattern.to_string(),
      flags: flags.to_string(),
      global,
      ignore_case,
      multiline,
      compiled,
      last_index: Arc::new(RwLock::new(0)),
    })))
  }

  /// 抽象相等比较
  pub fn abstract_eq(&self, other: &Self) -> bool {
    match (self, other) {
      (Self::Undefined, Self::Undefined) | (Self::Null, Self::Null) => true,
      (Self::Undefined, Self::Null) | (Self::Null, Self::Undefined) => true,
      (Self::Number(a), Self::Number(b)) => {
        if a.is_nan() || b.is_nan() {
          false
        } else {
          a == b
        }
      }
      (Self::BigInt(a), Self::BigInt(b)) => a == b,
      (Self::String(a), Self::String(b)) => a == b,
      (Self::Symbol(a), Self::Symbol(b)) => a == b,
      (Self::Boolean(a), Self::Boolean(b)) => a == b,
      (Self::Boolean(_), _) => Self::Number(self.to_number()).abstract_eq(other),
      (_, Self::Boolean(_)) => self.abstract_eq(&Self::Number(other.to_number())),
      (Self::Number(_), Self::String(_)) => {
        let r_num = other.to_number();
        if r_num.is_nan() {
          false
        } else {
          self.abstract_eq(&Self::Number(r_num))
        }
      }
      (Self::String(_), Self::Number(_)) => {
        let l_num = self.to_number();
        if l_num.is_nan() {
          false
        } else {
          Self::Number(l_num).abstract_eq(other)
        }
      }
      (Self::Array(a), Self::Array(b)) => Arc::ptr_eq(a, b),
      (Self::Object(a), Self::Object(b)) => Arc::ptr_eq(a, b),
      (Self::Function(a), Self::Function(b)) => Arc::ptr_eq(a, b),
      (Self::Native(a), Self::Native(b)) => a == b,
      (Self::String(_) | Self::Number(_), Self::Array(_) | Self::Object(_)) => {
        self.abstract_eq(&other.to_primitive(PreferredType::Default))
      }
      (Self::Array(_) | Self::Object(_), Self::String(_) | Self::Number(_)) => {
        self.to_primitive(PreferredType::Default).abstract_eq(other)
      }
      _ => false,
    }
  }

  /// ECMA-262 7.1.1 ToPrimitive 抽象操作
  pub fn to_primitive(&self, hint: PreferredType) -> Self {
    if self.is_primitive() {
      return self.clone();
    }
    match self {
      Self::Array(_) => Self::String(self.to_js_string()),
      Self::Object(obj) => {
        let guard = obj.read();
        if (hint == PreferredType::Number || hint == PreferredType::Default)
          && let Some(v) = guard.get(TIMESTAMP_KEY).or_else(|| guard.get(TIME_KEY))
        {
          return v.clone();
        }
        if (hint == PreferredType::Number || hint == PreferredType::Default)
          && let Some(v) = guard.get("valueOf")
          && v.is_primitive()
        {
          return v.clone();
        }
        Self::String(self.to_js_string())
      }
      Self::RegExp(re) => Self::String(format!("/{}/{}", re.pattern, re.flags)),
      Self::Function(_) | Self::Native(_) => Self::String(self.to_js_string()),
      _ => self.clone(),
    }
  }

  /// 转换为字符串
  pub fn to_js_string(&self) -> String {
    match self {
      Self::Undefined => "undefined".to_string(),
      Self::Null => "null".to_string(),
      Self::Boolean(b) => b.to_string(),
      Self::Number(n) => {
        if n.fract() == 0.0 && !n.is_infinite() && !n.is_nan() {
          let mut buf = Buffer::new();
          buf.format(*n as i64).to_string()
        } else {
          n.to_string()
        }
      }
      Self::BigInt(n) => {
        let mut buf = Buffer::new();
        buf.format(*n).to_string()
      }
      Self::String(s) => s.clone(),
      Self::Symbol(s) => format!("Symbol({s})"),
      Self::Array(arr) => {
        let guard = arr.read();
        let mut result = String::new();
        for (i, item) in guard.iter().enumerate() {
          if i > 0 {
            result.push(',');
          }
          match item {
            Self::Undefined | Self::Null => {}
            Self::String(s) => result.push_str(s),
            _ => result.push_str(&item.to_js_string()),
          }
        }
        result
      }
      Self::Object(_) => "[object Object]".to_string(),
      Self::RegExp(re) => format!("/{}/{}", re.pattern, re.flags),
      Self::Function(func) => format!(
        "function {}() {{ [native code] }}",
        func.name().unwrap_or("anonymous")
      ),
      Self::Native(_) => "function () { [native code] }".to_string(),
    }
  }

  /// 转换为布尔值
  pub fn to_boolean(&self) -> bool {
    match self {
      Self::Undefined | Self::Null => false,
      Self::Boolean(b) => *b,
      Self::Number(n) => *n != 0.0 && !n.is_nan(),
      Self::BigInt(n) => *n != 0,
      Self::String(s) => !s.is_empty(),
      Self::Symbol(_) => true,
      Self::Array(_) | Self::Object(_) | Self::RegExp(_) | Self::Function(_) | Self::Native(_) => {
        true
      }
    }
  }

  /// 转换为数字
  pub fn to_number(&self) -> f64 {
    match self {
      Self::Undefined => f64::NAN,
      Self::Null => 0.0,
      Self::Boolean(b) => {
        if *b {
          1.0
        } else {
          0.0
        }
      }
      Self::Number(n) => *n,
      Self::BigInt(n) => *n as f64,
      Self::Symbol(_) | Self::RegExp(_) => f64::NAN,
      Self::String(s) => {
        let trimmed = s.trim();
        if trimmed.is_empty() {
          0.0
        } else {
          trimmed.parse::<f64>().unwrap_or(f64::NAN)
        }
      }
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
      Self::Object(obj) => {
        let guard = obj.read();
        if let Some(v) = guard.get(TIMESTAMP_KEY).or_else(|| guard.get(TIME_KEY)) {
          v.to_number()
        } else {
          f64::NAN
        }
      }
      Self::Function(_) | Self::Native(_) => f64::NAN,
    }
  }

  /// 转换为 32 位带符号整数
  #[inline]
  pub fn to_int32(&self) -> i32 {
    double_to_int32(self.to_number())
  }

  /// 转换为 32 位无符号整数
  #[inline]
  pub fn to_uint32(&self) -> u32 {
    double_to_uint32(self.to_number())
  }

  /// 索引元素读取
  pub fn get_element(&self, key: &JsValue) -> Result<JsValue> {
    match key {
      JsValue::Number(n) => {
        if *n >= 0.0 && n.fract() == 0.0 && !n.is_infinite() && *n <= (u32::MAX - 1) as f64 {
          let uidx = *n as usize;
          match self {
            Self::Array(arr) => {
              let guard = arr.read();
              return Ok(guard.get(uidx).cloned().unwrap_or(JsValue::Undefined));
            }
            Self::String(s) => {
              if let Some(&b) = s.as_bytes().get(uidx)
                && b < 0x80
              {
                return Ok(JsValue::String((b as char).to_string()));
              } else if let Some(ch) = s.chars().nth(uidx) {
                return Ok(JsValue::String(ch.to_string()));
              }
              return Ok(JsValue::Undefined);
            }
            _ => {}
          }
        }
        let k = key.to_js_string();
        self.get_property(&k)
      }
      JsValue::String(s) => self.get_property(s),
      JsValue::Symbol(s) => self.get_property(&format_symbol_key(s)),
      other => {
        let k = other.to_string();
        self.get_property(&k)
      }
    }
  }

  /// 索引元素写入
  pub fn set_element(&self, key: &JsValue, val: JsValue) -> Result<()> {
    match key {
      JsValue::Number(n) => {
        if *n >= 0.0
          && n.fract() == 0.0
          && !n.is_infinite()
          && *n <= (u32::MAX - 1) as f64
          && let Self::Array(arr) = self
        {
          let uidx = *n as usize;
          let mut guard = arr.write();
          if uidx >= guard.len() {
            guard.resize(uidx + 1, JsValue::Undefined);
          }
          guard[uidx] = val;
          return Ok(());
        }
        let k = key.to_js_string();
        self.set_property(&k, val)
      }
      JsValue::String(s) => self.set_property(s, val),
      JsValue::Symbol(s) => self.set_property(&format_symbol_key(s), val),
      other => {
        let k = other.to_string();
        self.set_property(&k, val)
      }
    }
  }

  /// 属性读取
  pub fn get_property(&self, prop: &str) -> Result<JsValue> {
    match self {
      Self::Array(arr) => {
        if prop == LENGTH_KEY {
          let guard = arr.read();
          return Ok(JsValue::Number(guard.len() as f64));
        }
        if prop.as_bytes().first().is_some_and(|b| b.is_ascii_digit())
          && let Ok(idx) = prop.parse::<usize>()
        {
          let guard = arr.read();
          return Ok(guard.get(idx).cloned().unwrap_or(JsValue::Undefined));
        }
        match prop {
          PUSH_KEY => Ok(JsValue::Native(NativeFn::ArrayPush(Arc::clone(arr)))),
          POP_KEY => Ok(JsValue::Native(NativeFn::ArrayPop(Arc::clone(arr)))),
          SHIFT_KEY => Ok(JsValue::Native(NativeFn::ArrayShift(Arc::clone(arr)))),
          JOIN_KEY => Ok(JsValue::Native(NativeFn::ArrayJoin(Arc::clone(arr)))),
          SLICE_KEY => Ok(JsValue::Native(NativeFn::ArraySlice(Arc::clone(arr)))),
          INDEX_OF_KEY => Ok(JsValue::Native(NativeFn::ArrayIndexOf(Arc::clone(arr)))),
          SPLICE_KEY => Ok(JsValue::Native(NativeFn::ArraySplice(Arc::clone(arr)))),
          REVERSE_KEY => Ok(JsValue::Native(NativeFn::ArrayReverse(Arc::clone(arr)))),
          TO_STRING_KEY => Ok(JsValue::Native(NativeFn::ArrayToString(Arc::clone(arr)))),
          _ => Ok(JsValue::Undefined),
        }
      }

      Self::Object(obj) => {
        let guard = obj.read();
        if let Some(val) = guard.get(prop) {
          return Ok(val.clone());
        }
        let proto_opt = guard.get(PROTO_KEY).cloned();
        drop(guard);
        if let Some(proto) = proto_opt {
          return proto.get_property(prop);
        }
        match prop {
          TO_STRING_KEY => Ok(JsValue::Native(NativeFn::ObjectToString)),
          VALUE_OF_KEY => Ok(JsValue::Native(NativeFn::ObjectValueOf(Arc::clone(obj)))),
          HAS_OWN_PROPERTY_KEY => Ok(JsValue::Native(NativeFn::ObjectHasOwnProperty(Arc::clone(
            obj,
          )))),
          _ => Ok(JsValue::Undefined),
        }
      }

      Self::Function(func) => func.get_property(prop),

      Self::String(s) => {
        if prop == LENGTH_KEY {
          return Ok(JsValue::Number(char_count(s) as f64));
        }
        if prop.as_bytes().first().is_some_and(|b| b.is_ascii_digit())
          && let Ok(idx) = prop.parse::<usize>()
        {
          if let Some(&b) = s.as_bytes().get(idx)
            && b < 0x80
          {
            return Ok(JsValue::String((b as char).to_string()));
          } else if let Some(ch) = s.chars().nth(idx) {
            return Ok(JsValue::String(ch.to_string()));
          }
          return Ok(JsValue::Undefined);
        }
        match prop {
          CHAR_AT_KEY => Ok(JsValue::Native(NativeFn::StringCharAt(s.clone()))),
          CHAR_CODE_AT_KEY => Ok(JsValue::Native(NativeFn::StringCharCodeAt(s.clone()))),
          INDEX_OF_KEY => Ok(JsValue::Native(NativeFn::StringIndexOf(s.clone()))),
          SUBSTRING_KEY | SLICE_KEY => Ok(JsValue::Native(NativeFn::StringSubstring(s.clone()))),
          SUBSTR_KEY => Ok(JsValue::Native(NativeFn::StringSubstr(s.clone()))),
          CONCAT_KEY => Ok(JsValue::Native(NativeFn::StringConcat(s.clone()))),
          REPLACE_KEY => Ok(JsValue::Native(NativeFn::StringReplace(s.clone()))),
          SPLIT_KEY => Ok(JsValue::Native(NativeFn::StringSplit(s.clone()))),
          MATCH_KEY => Ok(JsValue::Native(NativeFn::StringMatch(s.clone()))),
          TO_LOWER_CASE_KEY => Ok(JsValue::Native(NativeFn::StringToLowerCase(s.clone()))),
          TO_UPPER_CASE_KEY => Ok(JsValue::Native(NativeFn::StringToUpperCase(s.clone()))),
          TO_STRING_KEY | VALUE_OF_KEY => Ok(JsValue::Native(NativeFn::StringToString(s.clone()))),
          _ => Ok(JsValue::Undefined),
        }
      }

      Self::Number(n) => {
        let num = *n;
        match prop {
          TO_FIXED_KEY => Ok(JsValue::Native(NativeFn::NumberToFixed(num))),
          TO_PRECISION_KEY => Ok(JsValue::Native(NativeFn::NumberToPrecision(num))),
          TO_STRING_KEY => Ok(JsValue::Native(NativeFn::NumberToString(num))),
          VALUE_OF_KEY => Ok(JsValue::Native(NativeFn::NumberValueOf(num))),
          _ => Ok(JsValue::Undefined),
        }
      }

      Self::Boolean(b) => {
        let val = *b;
        match prop {
          TO_STRING_KEY => Ok(JsValue::Native(NativeFn::BooleanToString(val))),
          VALUE_OF_KEY => Ok(JsValue::Native(NativeFn::BooleanValueOf(val))),
          _ => Ok(JsValue::Undefined),
        }
      }

      Self::RegExp(re) => {
        if prop == SOURCE_KEY {
          return Ok(JsValue::String(re.pattern.clone()));
        }
        if prop == GLOBAL_KEY {
          return Ok(JsValue::Boolean(re.global));
        }
        if prop == "ignoreCase" {
          return Ok(JsValue::Boolean(re.ignore_case));
        }
        if prop == "multiline" {
          return Ok(JsValue::Boolean(re.multiline));
        }
        if prop == LAST_INDEX_KEY {
          let idx = *re.last_index.read();
          return Ok(JsValue::Number(idx as f64));
        }
        match prop {
          TEST_KEY => Ok(JsValue::Native(NativeFn::RegExpTest(Arc::clone(re)))),
          EXEC_KEY => Ok(JsValue::Native(NativeFn::RegExpExec(Arc::clone(re)))),
          TO_STRING_KEY => Ok(JsValue::Native(NativeFn::StringToString(format!(
            "/{}/{}",
            re.pattern, re.flags
          )))),
          _ => Ok(JsValue::Undefined),
        }
      }

      Self::Native(_) => Ok(JsValue::Undefined),

      _ => Err(Error::RuntimeError(format!(
        "Cannot read property '{prop}' of {self:?}"
      ))),
    }
  }

  /// 属性设置
  pub fn set_property(&self, prop: &str, val: JsValue) -> Result<()> {
    match self {
      Self::Array(arr) => {
        if prop == "length" {
          let mut guard = arr.write();
          guard.resize(val.to_int32().max(0) as usize, JsValue::Undefined);
          return Ok(());
        }
        if prop.as_bytes().first().is_some_and(|b| b.is_ascii_digit())
          && let Ok(idx) = prop.parse::<usize>()
        {
          let mut guard = arr.write();
          if idx >= guard.len() {
            guard.resize(idx + 1, JsValue::Undefined);
          }
          guard[idx] = val;
          return Ok(());
        }
        Ok(())
      }
      Self::RegExp(re) => {
        if prop == "lastIndex" {
          *re.last_index.write() = val.to_int32().max(0) as usize;
        }
        Ok(())
      }
      Self::Object(obj) => {
        let mut guard = obj.write();
        if let Some(slot) = guard.get_mut(prop) {
          *slot = val;
        } else {
          guard.insert(prop.to_string(), val);
        }
        Ok(())
      }
      Self::Function(func) => func.set_property(prop, val),
      _ => Ok(()),
    }
  }
}
