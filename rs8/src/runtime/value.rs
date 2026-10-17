//! 运行时动态值类型模型
//!
//! 零拷贝与高效现代 Rust 实现，支持基础类型与复合对象。
//! 基于 gxhash 高性能哈希容器与 Arc 紧凑封装，优化内存占用。

use std::{
  fmt,
  ops::{Deref, DerefMut},
  ptr::fn_addr_eq,
  sync::Arc,
  time::Instant,
};

use gxhash::HashMap;
use itoa::Buffer;
use parking_lot::RwLock;
use v8_interpreter::{
  BytecodeDispatcher, GlobalsTable, RegExpCore, SliceKind, TIME_KEY, TIMESTAMP_KEY, TrimKind,
  Value, bytecode_dispatcher::get_substitution, is_regexp_readonly_key,
};
// 字符边界 helper 与切片窗口语义的单一真相在下层 `v8_interpreter::bytecode_dispatcher`
// (与字节码链同源, 一处定义两链共读; 此前宿主副本借过 slice 的负索引算法致 substring 偏)
pub(crate) use v8_interpreter::{char_count, char_to_byte_index};
use v8_numbers::{
  conversions::{
    ConversionFlag, MAX_FRACTION_DIGITS, double_to_exponential, double_to_fixed,
    double_to_precision, double_to_radix, double_to_string, string_to_double,
  },
  double_to_int32, double_to_uint32,
};

use crate::{
  error::{Error, Result},
  runtime::{
    ast::{Expr, ForOfHead, Stmt},
    bytecode_bridge::{
      interpreter_value_to_js_value, interpreter_value_to_js_value_with_globals,
      js_value_to_interpreter_value,
    },
    environment::Environment,
  },
};

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
  ErrorCtor {
    proto: Box<JsValue>,
    name: String,
  },
  ConsoleTime(Arc<RwLock<HashMap<String, Instant>>>),
  ConsoleTimeEnd(Arc<RwLock<HashMap<String, Instant>>>),
  /// Math 单目面 (12.27): ceil/trunc/cbrt/tan/asin/acos/atan/log2/log10/sign 共用一个变体,
  /// 按名分派纯 f64→f64 映射; 方法名单一真相在 `init_builtins` 的 MATH_UNARY 表
  MathUnary(&'static str),
  BoxedValueOf(Box<JsValue>),
  BoxedToString(Box<JsValue>),
  ArrayPush(Arc<RwLock<ArrayStore>>),
  ArrayPop(Arc<RwLock<ArrayStore>>),
  ArrayShift(Arc<RwLock<ArrayStore>>),
  ArraySlice(Arc<RwLock<ArrayStore>>),
  ArrayJoin(Arc<RwLock<ArrayStore>>),
  ArrayIndexOf(Arc<RwLock<ArrayStore>>),
  ArraySplice(Arc<RwLock<ArrayStore>>),
  ArrayReverse(Arc<RwLock<ArrayStore>>),
  ArrayToString(Arc<RwLock<ArrayStore>>),
  /// 比较器是用户函数, `NativeFn::call` 只有 `&self` 拿不到执行态, 故仅在
  /// `invoke_function_named` 就地排序 (与字节码链路的分发器拦截同构)
  ArraySort(Arc<RwLock<ArrayStore>>),
  ArrayIncludes(Arc<RwLock<ArrayStore>>),
  ArrayConcat(Arc<RwLock<ArrayStore>>),
  ArrayLastIndexOf(Arc<RwLock<ArrayStore>>),
  ArrayFill(Arc<RwLock<ArrayStore>>),
  ArrayUnshift(Arc<RwLock<ArrayStore>>),
  /// 回调是用户函数, 与 `ArraySort` 同类: 仅在 `invoke_function_named` 就地执行
  ArrayFind(Arc<RwLock<ArrayStore>>),
  /// findIndex 与 find 同款回调截获, 只是交出下标而非元素
  ArrayFindIndex(Arc<RwLock<ArrayStore>>),
  /// reduceRight 截获臂: 与 reduce 同款累子规则, 反向遍历
  ArrayReduceRight(Arc<RwLock<ArrayStore>>),
  /// flatMap 截获臂: 回调结果只压平一层
  ArrayFlatMap(Arc<RwLock<ArrayStore>>),
  /// 无回调的数组面第三轮 (12.27): at/flat/copyWithin 直接在 NativeFn::call 里跑
  ArrayAt(Arc<RwLock<ArrayStore>>),
  ArrayFlat(Arc<RwLock<ArrayStore>>),
  ArrayCopyWithin(Arc<RwLock<ArrayStore>>),
  /// Function.prototype.toString 的替身: 给源码原文 (parser span 切片, #23 ⑤);
  /// 无源可切 (宿主构造) 退回 native 摘要
  FunctionToString(Arc<SharedFunctionInfo>),
  ObjectToString,
  ObjectValueOf(Arc<RwLock<HashMap<String, JsValue>>>),
  ObjectHasOwnProperty(Arc<RwLock<HashMap<String, JsValue>>>),
  StringHasOwnProperty(Arc<str>),
  StringCharAt(Arc<str>),
  StringCharCodeAt(Arc<str>),
  StringIndexOf(Arc<str>),
  /// 第八轮 (12.32): 行为复用引擎 `str_last_index_of` 纯函数 (一处定义)
  StringLastIndexOf(Arc<str>),
  StringSubstring(Arc<str>),
  StringSlice(Arc<str>),
  StringSubstr(Arc<str>),
  StringConcat(Arc<str>),
  StringReplace(Arc<str>),
  StringSplit(Arc<str>),
  StringMatch(Arc<str>),
  StringToLowerCase(Arc<str>),
  StringToUpperCase(Arc<str>),
  StringIncludes(Arc<str>),
  StringRepeat(Arc<str>),
  StringPad(Arc<str>, bool),
  StringAt(Arc<str>),
  StringCodePointAt(Arc<str>),
  /// 字符串面第三轮 (12.27): 行为在引擎 `str_trim/str_starts_with/str_ends_with/str_replace_all`
  /// 纯函数一处定义, 与字节码链同源
  StringTrim(Arc<str>, v8_interpreter::TrimKind),
  StringStartsWith(Arc<str>),
  StringEndsWith(Arc<str>),
  StringReplaceAll(Arc<str>),
  StringToString(Arc<str>),
  NumberToFixed(f64),
  NumberToPrecision(f64),
  NumberToExponential(f64),
  NumberToString(f64),
  NumberValueOf(f64),
  BooleanToString(bool),
  BooleanValueOf(bool),
  RegExpTest(Arc<RegExpData>),
  RegExpExec(Arc<RegExpData>),
  /// 正则实例继承面回落 (12.36): 宿主闭表无原型链遍历, Object.prototype 的
  /// hasOwnProperty/propertyIsEnumerable 逐键显式对齐; own 名册单一真相 = own_property_names
  RegExpHasOwnProperty(Arc<RegExpData>),
  RegExpPropertyIsEnumerable(Arc<RegExpData>),
  InterpNative(v8_interpreter::NativeFn),
  /// 字节码闭包 + 其所属全局表句柄。句柄必须随值一起带出: `NativeFn::call` 拿不到调用方
  /// 的执行态, 若只带闭包本体就只能在被调点重建一张空表, 被调函数将看不见宿主全局
  /// (对标原版: JSFunction 自带 NativeContext)。带 `Closure` 而非 `InterpFunction`,
  /// 是让词法环境、实例属性与函数身份跨引擎往返都不掉
  InterpFunction(Arc<v8_interpreter::Closure>, GlobalsTable),
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
        let _ = obj.set_property(NAME_KEY, JsValue::String(name.clone().into()));
        let _ = obj.set_property(MESSAGE_KEY, JsValue::String(msg.into()));
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
      Self::BoxedToString(inner) => Ok(JsValue::String((**inner).to_string().into())),
      Self::ArrayPush(arr) => Ok(JsValue::array_push_inlined(arr, args)),
      Self::ArrayPop(arr) => Ok(JsValue::array_pop_inlined(arr)),
      Self::ArrayShift(arr) => Ok(JsValue::array_shift_inlined(arr)),
      Self::ArraySlice(arr) => Ok(JsValue::array_slice_inlined(arr, args)),
      Self::ArrayJoin(arr) => Ok(JsValue::array_join_inlined(arr, args)),
      Self::ArrayIndexOf(arr) => Ok(JsValue::array_index_of_inlined(arr, args)),
      Self::ArraySplice(arr) => Ok(JsValue::array_splice_inlined(arr, args)),
      Self::ArrayReverse(arr) => Ok(JsValue::array_reverse_inlined(arr)),
      Self::ArrayToString(arr) => Ok(JsValue::array_to_string_inlined(arr)),
      Self::ArrayIncludes(arr) => Ok(JsValue::array_includes_inlined(arr, args)),
      Self::ArrayConcat(arr) => Ok(JsValue::array_concat_inlined(arr, args)),
      Self::ArrayLastIndexOf(arr) => Ok(JsValue::array_last_index_of_inlined(arr, args)),
      Self::ArrayFill(arr) => Ok(JsValue::array_fill_inlined(arr, args)),
      Self::ArrayUnshift(arr) => Ok(JsValue::array_unshift_inlined(arr, args)),
      Self::ArrayAt(arr) => {
        let guard = arr.read();
        let len = guard.len() as i64;
        let idx = args
          .first()
          .map(|v| v8_interpreter::to_index_i64(v.to_number()))
          .unwrap_or(0);
        let i = if idx < 0 {
          len.saturating_add(idx)
        } else {
          idx
        };
        Ok(if i < 0 || i as usize >= guard.len() {
          JsValue::Undefined
        } else {
          guard[i as usize].clone()
        })
      }
      Self::ArrayFlat(arr) => {
        let depth = args
          .first()
          .filter(|v| !matches!(v, JsValue::Undefined))
          .map_or(1, |v| v8_interpreter::to_index_i64(v.to_number()));
        let mut out = Vec::new();
        js_flatten_into(&arr.read(), depth.max(0), &mut out);
        Ok(JsValue::new_array(out))
      }
      Self::ArrayCopyWithin(arr) => {
        // 与字节码链同款: 源/目标各自负索引回绕并夹到 [0, len], 源段快照后写回
        let wrap = |v: i64, len: usize| -> usize {
          let t = if v < 0 {
            (len as i64).saturating_add(v)
          } else {
            v
          };
          t.clamp(0, len as i64) as usize
        };
        let len = arr.read().len();
        let target = wrap(
          args
            .first()
            .map(|v| v8_interpreter::to_index_i64(v.to_number()))
            .unwrap_or(0),
          len,
        );
        let src_start = wrap(
          args
            .get(1)
            .map(|v| v8_interpreter::to_index_i64(v.to_number()))
            .unwrap_or(0),
          len,
        );
        let src_end = wrap(
          args
            .get(2)
            .filter(|v| !matches!(v, JsValue::Undefined))
            .map_or(len as i64, |v| v8_interpreter::to_index_i64(v.to_number())),
          len,
        );
        let count = src_end
          .saturating_sub(src_start)
          .min(len.saturating_sub(target));
        if count > 0 {
          let snapshot: Vec<JsValue> = arr.read()[src_start..src_start + count].to_vec();
          arr.write()[target..target + count].clone_from_slice(&snapshot);
        }
        Ok(JsValue::Array(Arc::clone(arr)))
      }
      Self::ArrayFind(_) | Self::ArrayFindIndex(_) => Err(Error::TypeError(
        "find/findIndex 需要回调用户函数, 只能经 invoke_function_named 截获".to_string(),
      )),
      Self::ArrayReduceRight(_) | Self::ArrayFlatMap(_) => Err(Error::TypeError(
        "reduceRight/flatMap 需要回调用户函数, 只能经 invoke_function_named 截获".to_string(),
      )),
      Self::ArraySort(_) => Err(Error::TypeError(
        "sort 需要比较器回调, 只能经 invoke_function_named 截获".to_string(),
      )),
      Self::MathUnary(name) => {
        let n = args.first().map(|v| v.to_number()).unwrap_or(f64::NAN);
        Ok(JsValue::Number(match *name {
          "ceil" => n.ceil(),
          "trunc" => n.trunc(),
          "cbrt" => n.cbrt(),
          "tan" => n.tan(),
          "asin" => n.asin(),
          "acos" => n.acos(),
          "atan" => n.atan(),
          "log2" => n.log2(),
          "log10" => n.log10(),
          // Rust 的 signum 对 0/NaN 交 ±1, JS 要 IEEE 口径: ±0 保号, NaN 传 NaN
          "sign" => {
            if n > 0.0 {
              1.0
            } else if n < 0.0 {
              -1.0
            } else {
              n
            }
          }
          _ => f64::NAN,
        }))
      }
      Self::ObjectToString => Ok(JsValue::String("[object Object]".into())),
      Self::FunctionToString(shared) => Ok(JsValue::String(
        match &shared.src {
          Some(s) => s.clone(),
          None => native_function_summary(shared.name.as_deref()),
        }
        .into(),
      )),
      Self::ObjectValueOf(obj) => Ok(JsValue::Object(Arc::clone(obj))),
      Self::ObjectHasOwnProperty(obj) => {
        let key = args.first().map(|v| v.to_string()).unwrap_or_default();
        let guard = obj.read();
        Ok(JsValue::Boolean(guard.contains_key(&key)))
      }
      // 字符串基元接收者的 hasOwnProperty 回落 (宿主闭表无原型链遍历, 与引擎 boxing 路径对拍)
      Self::StringHasOwnProperty(s) => {
        let key = args.first().map(|v| v.to_string()).unwrap_or_default();
        let own = if key == LENGTH_KEY {
          true
        } else if let Ok(idx) = key.parse::<usize>() {
          idx < char_count(s)
        } else {
          false
        };
        Ok(JsValue::Boolean(own))
      }
      Self::StringCharAt(s) => {
        let idx = args
          .first()
          .map(|v| v.to_int32().max(0) as usize)
          .unwrap_or(0);
        if s.is_ascii() {
          if let Some(&b) = s.as_bytes().get(idx) {
            return Ok(JsValue::String((b as char).to_string().into()));
          }
          return Ok(JsValue::String(String::new().into()));
        }
        let ch_str = s
          .chars()
          .nth(idx)
          .map(|c| c.to_string())
          .unwrap_or_default();
        Ok(JsValue::String(ch_str.into()))
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
      Self::StringLastIndexOf(s) => {
        let needle = args.first().map(|v| v.to_string()).unwrap_or_default();
        let c_len = char_count(s) as i64;
        // 与引擎臂同源: 缺省/undefined/NaN→len; 任意负数→-1; 正数截断后夹 len
        let pos = match args.get(1) {
          None | Some(JsValue::Undefined) => c_len,
          Some(v) => {
            let x = v.to_number();
            if x.is_nan() {
              c_len
            } else {
              let n = v8_interpreter::to_index_i64(x);
              if n < 0 { -1 } else { n.min(c_len) }
            }
          }
        };
        Ok(JsValue::Number(
          v8_interpreter::str_last_index_of(s, &needle, pos) as f64,
        ))
      }
      Self::StringSubstring(s) | Self::StringSlice(s) | Self::StringSubstr(s) => {
        let kind = match self {
          Self::StringSubstring(_) => SliceKind::Substring,
          Self::StringSlice(_) => SliceKind::Slice,
          _ => SliceKind::Substr,
        };
        let c_len = char_count(s) as i64;
        let start = args
          .first()
          .map(|v| v8_interpreter::to_index_i64(v.to_number()))
          .unwrap_or(0);
        let end = args
          .get(1)
          .filter(|v| !matches!(v, JsValue::Undefined))
          .map(|v| v8_interpreter::to_index_i64(v.to_number()));
        let (from_c, to_c) = v8_interpreter::slice_window(kind, c_len, start, end);
        let from_b = char_to_byte_index(s, from_c);
        let to_b = char_to_byte_index(s, to_c);
        Ok(JsValue::String(s[from_b..to_b].into()))
      }
      Self::StringConcat(s) => {
        let mut res = s.to_string();
        for a in args {
          res.push_str(&a.to_string());
        }
        Ok(JsValue::String(res.into()))
      }
      Self::StringReplace(s) => {
        let pattern = args.first().cloned().unwrap_or(JsValue::Undefined);
        let subst = args.get(1).cloned().unwrap_or(JsValue::Undefined);
        // 函数替换实参要回调用户函数, 只能经 invoke_function_named 截获 (同 ArrayFind)
        if subst.is_callable() {
          return Err(Error::TypeError(
            "replace 的函数回调只能经 invoke_function_named 截获".to_string(),
          ));
        }
        let repl_str = subst.to_string();
        match pattern {
          JsValue::RegExp(re) => {
            // 手动命中循环: $ 序列走引擎唯一 get_substitution (fancy 的 replace 只认 $1/$name,
            // 不认 $&/$`/$'), 回调与替换统一在同口径
            let mut out = String::new();
            let mut last_end = 0usize;
            let mut cur = 0usize;
            loop {
              if cur > s.len() {
                break;
              }
              let caps_opt = if re.core.global {
                re.core.compiled.captures_from_pos(&**s, cur)
              } else {
                re.core.compiled.captures(&**s)
              };
              let Ok(Some(caps)) = caps_opt else { break };
              let Some(whole) = caps.get(0) else { break };
              let (ms, me) = (whole.start(), whole.end());
              if ms >= last_end {
                out.push_str(&s[last_end..ms]);
              }
              let groups: Vec<Option<&str>> =
                caps.iter().skip(1).map(|m| m.map(|x| x.as_str())).collect();
              out.push_str(&get_substitution(
                &repl_str,
                &s[ms..me],
                &s[..ms],
                &s[me..],
                &groups,
              ));
              last_end = me;
              if !re.core.global {
                break;
              }
              cur = if me == ms { ms + 1 } else { me.max(cur + 1) };
            }
            out.push_str(&s[last_end.min(s.len())..]);
            Ok(JsValue::String(out.into()))
          }
          _ => {
            // 字符串模式: 字面量单处替换, $ 序列同样生效 (node 口径, 与字节码链一致)
            let pat_str = pattern.to_string();
            if let Some(pos) = s.find(&pat_str) {
              let piece = get_substitution(
                &repl_str,
                &s[pos..pos + pat_str.len()],
                &s[..pos],
                &s[pos + pat_str.len()..],
                &[],
              );
              let mut res = String::with_capacity(s.len() + piece.len());
              res.push_str(&s[..pos]);
              res.push_str(&piece);
              res.push_str(&s[pos + pat_str.len()..]);
              Ok(JsValue::String(res.into()))
            } else {
              Ok(JsValue::String(s.clone()))
            }
          }
        }
      }
      Self::StringSplit(s) => {
        let sep = args.first().cloned().unwrap_or(JsValue::Undefined);
        // 缺省/undefined 分隔符按规范整串单元素返回 (拿 "undefined" 当针是巧合对拍)
        if matches!(sep, JsValue::Undefined) {
          return Ok(JsValue::new_array(vec![JsValue::String(s.clone())]));
        }
        let limit = args.get(1).and_then(|v| {
          let n = v.to_int32();
          if n >= 0 { Some(n as usize) } else { None }
        });
        let mut items = Vec::new();
        match sep {
          JsValue::RegExp(re) => {
            // 捕获组拼入结果 + 空匹配规则, 与桥端共用 regex_split_parts 一处定义 (#23 面)
            items.extend(
              regex_split_parts(s, &re.core, limit)
                .into_iter()
                .map(|p| p.map_or(JsValue::Undefined, |s| JsValue::String(s.into()))),
            );
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
                items.push(JsValue::String(c.to_string().into()));
              }
            } else {
              for part in s.split(&sep_str) {
                if let Some(lim) = limit
                  && items.len() >= lim
                {
                  break;
                }
                items.push(JsValue::String(part.into()));
              }
            }
          }
        }
        Ok(JsValue::new_array(items))
      }
      Self::StringMatch(s) => {
        let pat = args.first().cloned().unwrap_or(JsValue::Undefined);
        if let JsValue::RegExp(re) = pat {
          if re.core.global {
            let mut results = Vec::new();
            let mut cur_pos = 0;
            while cur_pos <= s.len() {
              if let Ok(Some(mat)) = re.core.compiled.find_from_pos(&**s, cur_pos) {
                results.push(JsValue::String(mat.as_str().to_string().into()));
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
          // 字符串模式: node 会 RegExpCreate 按正则解释, 宿主先按字面量单处命中 (无元字符时
          // 与 node 一致), 结果数组挂 index/input (与字节码链同口径, 元字符分歧记待办)
          let pat_str = pat.to_string();
          if let Some(pos) = s.find(&pat_str) {
            let res =
              JsValue::new_array(vec![JsValue::String((&s[pos..pos + pat_str.len()]).into())]);
            let _ = res.set_property("index", JsValue::Number(char_count(&s[..pos]) as f64));
            let _ = res.set_property("input", JsValue::String(s.clone()));
            Ok(res)
          } else {
            Ok(JsValue::Null)
          }
        }
      }
      Self::StringToLowerCase(s) => Ok(JsValue::String(s.to_lowercase().into())),
      Self::StringToUpperCase(s) => Ok(JsValue::String(s.to_uppercase().into())),
      Self::StringIncludes(s) => {
        // 缺省搜索参按 ToString(undefined)="undefined" 口径, 空串默认会让 includes() 恒真
        let needle = args
          .first()
          .map(|v| v.to_string())
          .unwrap_or_else(|| "undefined".to_string());
        let from = args
          .get(1)
          .filter(|v| !matches!(v, JsValue::Undefined))
          .map(|v| v8_interpreter::to_index_i64(v.to_number()))
          .unwrap_or(0);
        Ok(JsValue::Boolean(v8_interpreter::str_includes(
          s, &needle, from,
        )))
      }
      Self::StringRepeat(s) => {
        let count = args
          .first()
          .map(|v| v8_interpreter::to_index_i64(v.to_number()))
          .unwrap_or(0);
        v8_interpreter::str_repeat(s, count)
          .map(|s| JsValue::String(s.into()))
          .ok_or_else(|| Error::RangeError("Invalid count value".to_string()))
      }
      Self::StringPad(s, start) => {
        let width = args
          .first()
          .map(|v| v8_interpreter::to_index_i64(v.to_number()))
          .unwrap_or(0);
        let pad = args
          .get(1)
          .filter(|v| !matches!(v, JsValue::Undefined))
          .map(|v| v.to_string())
          .unwrap_or_else(|| " ".to_string());
        v8_interpreter::str_pad(s, width, &pad, *start)
          .map(|s| JsValue::String(s.into()))
          .ok_or_else(|| {
            Error::RangeError(format!(
              "{} max value is 2^53-1",
              if *start { "padStart" } else { "padEnd" }
            ))
          })
      }
      Self::StringAt(s) => {
        let idx = args
          .first()
          .map(|v| v8_interpreter::to_index_i64(v.to_number()))
          .unwrap_or(0);
        Ok(match v8_interpreter::str_at(s, idx) {
          Some(ch) => JsValue::String(ch.to_string().into()),
          None => JsValue::Undefined,
        })
      }
      Self::StringCodePointAt(s) => {
        // 越界给 undefined (区别于 charCodeAt 的 NaN, node 语义如此)
        let idx = args
          .first()
          .map(|v| v8_interpreter::to_index_i64(v.to_number()))
          .unwrap_or(0);
        Ok(match v8_interpreter::str_code_point_at(s, idx) {
          Some(cp) => JsValue::Number(cp as f64),
          None => JsValue::Undefined,
        })
      }
      Self::StringToString(s) => Ok(JsValue::String(s.clone())),
      Self::StringTrim(s, kind) => Ok(JsValue::String(v8_interpreter::str_trim(s, *kind).into())),
      Self::StringStartsWith(s) => {
        // 缺省搜索参按 ToString("undefined") 口径 (与 includes/startsWith 的 node 语义一致)
        let needle = args
          .first()
          .map(|v| v.to_string())
          .unwrap_or_else(|| "undefined".to_string());
        let from = args
          .get(1)
          .filter(|v| !matches!(v, JsValue::Undefined))
          .map(|v| v8_interpreter::to_index_i64(v.to_number()))
          .unwrap_or(0);
        Ok(JsValue::Boolean(v8_interpreter::str_starts_with(
          s, &needle, from,
        )))
      }
      Self::StringEndsWith(s) => {
        let needle = args
          .first()
          .map(|v| v.to_string())
          .unwrap_or_else(|| "undefined".to_string());
        // 缺省 endPosition 是全长 (与 startsWith 的缺省 0 相反), i64::MAX 由纯函数夹到 len
        let pos = args
          .get(1)
          .filter(|v| !matches!(v, JsValue::Undefined))
          .map_or(i64::MAX, |v| v8_interpreter::to_index_i64(v.to_number()));
        Ok(JsValue::Boolean(v8_interpreter::str_ends_with(
          s, &needle, pos,
        )))
      }
      Self::StringReplaceAll(s) => {
        if args.get(1).is_some_and(|v| v.is_callable()) {
          return Err(Error::TypeError(
            "replaceAll 的函数回调未接入 (战役 C)".to_string(),
          ));
        }
        if let Some(JsValue::RegExp(re)) = args.first() {
          if !re.core.global {
            return Err(Error::TypeError(
              "String.prototype.replaceAll called with a non-global RegExp argument".to_string(),
            ));
          }
          // g 标志借道 replace 的全局命中循环 ($ 序列同口径)
          return Self::StringReplace(s.clone()).call(args);
        }
        let pat = args
          .first()
          .map(|v| v.to_string())
          .unwrap_or_else(|| "undefined".to_string());
        let repl = args
          .get(1)
          .map(|v| v.to_string())
          .unwrap_or_else(|| "undefined".to_string());
        Ok(JsValue::String(
          v8_interpreter::str_replace_all(s, &pat, &repl).into(),
        ))
      }
      Self::NumberToFixed(num) => {
        // 参数按 ToIntegerOrInfinity 归约 (NaN→0), 越界 (含 ±Infinity) 抛 RangeError;
        // 不能走 to_int32 的模 2^32, 否则 Infinity 会被折成 0 悄悄吞掉
        let f = args.first().map(|v| v.to_number() as i64).unwrap_or(0);
        if !(0..=MAX_FRACTION_DIGITS as i64).contains(&f) {
          return Err(Error::RangeError(
            "toFixed() digits argument must be between 0 and 100".to_string(),
          ));
        }
        Ok(JsValue::String(double_to_fixed(*num, f as i32).into()))
      }
      Self::NumberToPrecision(num) => {
        // 缺省参数按 ToString 走, 不是某个默认精度
        let Some(p) = args
          .first()
          .filter(|v| !matches!(v, JsValue::Undefined))
          .map(|v| v.to_number() as i64)
        else {
          return Ok(JsValue::String(double_to_string(*num).into()));
        };
        if !(1..=MAX_FRACTION_DIGITS as i64).contains(&p) {
          return Err(Error::RangeError(
            "toPrecision() argument must be between 1 and 100".to_string(),
          ));
        }
        Ok(JsValue::String(double_to_precision(*num, p as i32).into()))
      }
      Self::NumberToExponential(num) => {
        // 与 toPrecision 同族: 参数缺省 (或显式 undefined) 时 f=-1 走最短数字; 越界抛 RangeError
        let Some(f) = args
          .first()
          .filter(|v| !matches!(v, JsValue::Undefined))
          .map(|v| v.to_number() as i64)
        else {
          return Ok(JsValue::String(double_to_exponential(*num, -1).into()));
        };
        if !(0..=MAX_FRACTION_DIGITS as i64).contains(&f) {
          return Err(Error::RangeError(
            "toExponential() argument must be between 0 and 100".to_string(),
          ));
        }
        Ok(JsValue::String(
          double_to_exponential(*num, f as i32).into(),
        ))
      }
      Self::NumberToString(num) => {
        // 只有 undefined (含缺省) 退成十进制; null/NaN/0 经 ToIntegerOrInfinity 落到 0 照样抛
        let radix = args
          .first()
          .filter(|v| !matches!(v, JsValue::Undefined))
          .map_or(10, |v| v.to_number() as i64);
        if !(2..=36).contains(&radix) {
          return Err(Error::RangeError(
            "toString() radix argument must be between 2 and 36".to_string(),
          ));
        }
        if radix == 10 {
          return Ok(JsValue::String(double_to_string(*num).into()));
        }
        Ok(JsValue::String(double_to_radix(*num, radix as i32).into()))
      }
      Self::NumberValueOf(num) => Ok(JsValue::Number(*num)),
      Self::BooleanToString(b) => Ok(JsValue::String(b.to_string().into())),
      Self::BooleanValueOf(b) => Ok(JsValue::Boolean(*b)),
      Self::RegExpTest(re) => {
        let text = args.first().map(|v| v.to_string()).unwrap_or_default();
        let mut start_pos = 0;
        if re.core.tracks_last_index() {
          start_pos = *re.core.last_index.read();
          let c_len = char_count(&text);
          if start_pos > c_len {
            *re.core.last_index.write() = 0;
            return Ok(JsValue::Boolean(false));
          }
        }
        let start_byte = char_to_byte_index(&text, start_pos);
        let haystack = if start_byte < text.len() {
          &text[start_byte..]
        } else {
          ""
        };
        // sticky 锚定: 命中起点必须恰为 lastIndex (haystack 已切片 → 起点 0), 靠后视作未命中
        let mat_opt = match re.core.compiled.find(haystack) {
          Ok(Some(mat)) if !(re.core.sticky() && mat.start() != 0) => Some(mat),
          _ => None,
        };
        if let Some(mat) = mat_opt {
          if re.core.tracks_last_index() {
            let end_char = if text.is_ascii() {
              start_byte + mat.end()
            } else {
              text[..start_byte + mat.end()].chars().count()
            };
            *re.core.last_index.write() = end_char;
          }
          Ok(JsValue::Boolean(true))
        } else {
          if re.core.tracks_last_index() {
            *re.core.last_index.write() = 0;
          }
          Ok(JsValue::Boolean(false))
        }
      }
      Self::RegExpExec(re) => {
        let text = args.first().map(|v| v.to_string()).unwrap_or_default();
        let mut start_pos = 0;
        if re.core.tracks_last_index() {
          start_pos = *re.core.last_index.read();
          let c_len = char_count(&text);
          if start_pos > c_len {
            *re.core.last_index.write() = 0;
            return Ok(JsValue::Null);
          }
        }
        let start_byte = char_to_byte_index(&text, start_pos);
        let haystack = if start_byte < text.len() {
          &text[start_byte..]
        } else {
          ""
        };
        // sticky 锚定同 RegExpTest: 整体命中起点须为 0 (切片口径), 靠后视作未命中
        match re.core.compiled.captures(haystack) {
          Ok(Some(caps)) if !(re.core.sticky() && caps.get(0).map(|m| m.start()) != Some(0)) => {
            let mut list = Vec::with_capacity(caps.len());
            for m in caps.iter() {
              if let Some(mat) = m {
                list.push(JsValue::String(mat.as_str().to_string().into()));
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
              if re.core.tracks_last_index() {
                *re.core.last_index.write() = end_pos;
              }
              let res_arr = JsValue::new_array(list);
              let _ = res_arr.set_property("index", JsValue::Number(match_start as f64));
              let _ = res_arr.set_property("input", JsValue::String(text.into()));
              Ok(res_arr)
            } else {
              Ok(JsValue::Null)
            }
          }
          _ => {
            if re.core.tracks_last_index() {
              *re.core.last_index.write() = 0;
            }
            Ok(JsValue::Null)
          }
        }
      }
      Self::RegExpHasOwnProperty(re) => {
        let key = args.first().map(|v| v.to_string()).unwrap_or_default();
        Ok(JsValue::Boolean(re.own_property_names().contains(&key)))
      }
      Self::RegExpPropertyIsEnumerable(re) => {
        // lastIndex 自有但不可枚举; expando/遮蔽键普通可枚举 (node: keys(/re 带 foo)=["foo"])
        let key = args.first().map(|v| v.to_string()).unwrap_or_default();
        Ok(JsValue::Boolean(re.props.read().contains_key(&key)))
      }
      Self::InterpNative(native) => {
        let interp_args: Vec<v8_interpreter::Value> = args
          .iter()
          .map(|a| js_value_to_interpreter_value(a).unwrap_or(Value::Undefined))
          .collect();
        // 宿主侧 NativeFn::call 没有 receiver 形参, 这里只能先传 Undefined;
        // 真正打通要等 战役 C 把两套值模型/两套 NativeFn 合一
        match native.call(&Value::Undefined, &interp_args) {
          Ok(res) => Ok(interpreter_value_to_js_value(&res)),
          Err(e) => Err(Error::RuntimeError(e.to_string())),
        }
      }
      Self::InterpFunction(closure, globals) => {
        // 复用产生该函数的那张全局表, 不在调用点重建执行态
        let mut d = BytecodeDispatcher::with_globals(Arc::clone(globals));
        let interp_args: Vec<v8_interpreter::Value> = args
          .iter()
          .map(|a| js_value_to_interpreter_value(a).unwrap_or(Value::Undefined))
          .collect();
        match d.call_function(
          &Value::Function(Arc::clone(closure)),
          &Value::Undefined,
          &interp_args,
        ) {
          Ok(res) => Ok(interpreter_value_to_js_value_with_globals(&res, globals)),
          Err(e) => Err(Error::RuntimeError(e.to_string())),
        }
      }
      Self::Context(ctx) => (ctx.call_fn)(ctx.payload, args),
    }
  }
}

/// `flat` 的递归压平 (12.27): 与引擎 `flatten_into` 同款, 深度>0 展开数组元素并降深度
fn js_flatten_into(elems: &[JsValue], depth: i64, out: &mut Vec<JsValue>) {
  for e in elems {
    if depth > 0
      && let JsValue::Array(inner) = e
    {
      js_flatten_into(&inner.read(), depth - 1, out);
    } else {
      out.push(e.clone());
    }
  }
}

impl PartialEq for NativeFn {
  fn eq(&self, other: &Self) -> bool {
    match (self, other) {
      (Self::Ptr(a), Self::Ptr(b)) => fn_addr_eq(*a, *b),
      (Self::ObjectToString, Self::ObjectToString) => true,
      (Self::MathUnary(a), Self::MathUnary(b)) => a == b,
      (Self::ArrayPush(a), Self::ArrayPush(b))
      | (Self::ArrayPop(a), Self::ArrayPop(b))
      | (Self::ArrayShift(a), Self::ArrayShift(b))
      | (Self::ArraySlice(a), Self::ArraySlice(b))
      | (Self::ArrayJoin(a), Self::ArrayJoin(b))
      | (Self::ArrayIndexOf(a), Self::ArrayIndexOf(b))
      | (Self::ArraySplice(a), Self::ArraySplice(b))
      | (Self::ArrayReverse(a), Self::ArrayReverse(b))
      | (Self::ArrayToString(a), Self::ArrayToString(b))
      | (Self::ArraySort(a), Self::ArraySort(b))
      | (Self::ArrayIncludes(a), Self::ArrayIncludes(b))
      | (Self::ArrayConcat(a), Self::ArrayConcat(b))
      | (Self::ArrayLastIndexOf(a), Self::ArrayLastIndexOf(b))
      | (Self::ArrayFill(a), Self::ArrayFill(b))
      | (Self::ArrayUnshift(a), Self::ArrayUnshift(b))
      | (Self::ArrayFind(a), Self::ArrayFind(b))
      | (Self::ArrayFindIndex(a), Self::ArrayFindIndex(b))
      | (Self::ArrayAt(a), Self::ArrayAt(b))
      | (Self::ArrayFlat(a), Self::ArrayFlat(b))
      | (Self::ArrayCopyWithin(a), Self::ArrayCopyWithin(b))
      | (Self::ArrayReduceRight(a), Self::ArrayReduceRight(b))
      | (Self::ArrayFlatMap(a), Self::ArrayFlatMap(b)) => Arc::ptr_eq(a, b),
      (Self::ObjectValueOf(a), Self::ObjectValueOf(b))
      | (Self::ObjectHasOwnProperty(a), Self::ObjectHasOwnProperty(b)) => Arc::ptr_eq(a, b),
      (Self::StringCharAt(a), Self::StringCharAt(b))
      | (Self::StringCharCodeAt(a), Self::StringCharCodeAt(b))
      | (Self::StringIndexOf(a), Self::StringIndexOf(b))
      | (Self::StringLastIndexOf(a), Self::StringLastIndexOf(b))
      | (Self::StringSubstring(a), Self::StringSubstring(b))
      | (Self::StringSlice(a), Self::StringSlice(b))
      | (Self::StringSubstr(a), Self::StringSubstr(b))
      | (Self::StringConcat(a), Self::StringConcat(b))
      | (Self::StringReplace(a), Self::StringReplace(b))
      | (Self::StringSplit(a), Self::StringSplit(b))
      | (Self::StringMatch(a), Self::StringMatch(b))
      | (Self::StringToLowerCase(a), Self::StringToLowerCase(b))
      | (Self::StringToUpperCase(a), Self::StringToUpperCase(b))
      | (Self::StringIncludes(a), Self::StringIncludes(b))
      | (Self::StringRepeat(a), Self::StringRepeat(b))
      | (Self::StringAt(a), Self::StringAt(b))
      | (Self::StringCodePointAt(a), Self::StringCodePointAt(b))
      | (Self::StringStartsWith(a), Self::StringStartsWith(b))
      | (Self::StringEndsWith(a), Self::StringEndsWith(b))
      | (Self::StringReplaceAll(a), Self::StringReplaceAll(b))
      | (Self::StringToString(a), Self::StringToString(b))
      | (Self::StringHasOwnProperty(a), Self::StringHasOwnProperty(b)) => a == b,
      (Self::StringPad(a, sa), Self::StringPad(b, sb)) => sa == sb && a == b,
      (Self::StringTrim(a, ka), Self::StringTrim(b, kb)) => ka == kb && a == b,
      (Self::FunctionToString(a), Self::FunctionToString(b)) => Arc::ptr_eq(a, b),
      (Self::NumberToFixed(a), Self::NumberToFixed(b))
      | (Self::NumberToPrecision(a), Self::NumberToPrecision(b))
      | (Self::NumberToExponential(a), Self::NumberToExponential(b))
      | (Self::NumberToString(a), Self::NumberToString(b))
      | (Self::NumberValueOf(a), Self::NumberValueOf(b)) => a == b,
      (Self::BooleanToString(a), Self::BooleanToString(b))
      | (Self::BooleanValueOf(a), Self::BooleanValueOf(b)) => a == b,
      (Self::RegExpTest(a), Self::RegExpTest(b))
      | (Self::RegExpExec(a), Self::RegExpExec(b))
      | (Self::RegExpHasOwnProperty(a), Self::RegExpHasOwnProperty(b))
      | (Self::RegExpPropertyIsEnumerable(a), Self::RegExpPropertyIsEnumerable(b)) => {
        Arc::ptr_eq(a, b)
      }
      (Self::InterpNative(a), Self::InterpNative(b)) => a == b,
      // 表句柄在往返中会出现副本, 身份只看闭包实例: 两次 `mk()` 造出的闭包必须不相等
      (Self::InterpFunction(a, _), Self::InterpFunction(b, _)) => Arc::ptr_eq(a, b),
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
  /// 箭头函数标记 (12.28): 不绑定自身 this/arguments, new 调用一律 TypeError
  pub is_arrow: bool,
  /// 源码原文切片 (parser 按 token 字节偏移切出, #23 残项⑤: `String(fn)`/`fn.toString()` 的真相)
  ///
  /// `None` = 宿主构造、无 JS 源可比对 (内建替身等), 字符串化退回 `[Function: name]` 摘要
  pub src: Option<String>,
  /// rest 形参绑定 (12.40): (绑定名, 实参收集起点)。绑定名不在 params 里 ——
  /// params 只装按位形参, 尾段快照由各链进帧时刻单独绑名 (箭头同样吃自身调用实参)
  pub rest_param: Option<(String, usize)>,
}

impl SharedFunctionInfo {
  /// 创建并预分析函数静态元数据 (收集局部提升变量、内部函数与 arguments 引用)
  pub fn new(name: Option<String>, params: Vec<String>, body: Vec<Stmt>) -> Self {
    Self::new_with_src(name, params, body, None, None)
  }

  /// 同上, 另存源码原文切片与 rest 形参绑定 (parser 的两个函数入口调用, #23 ⑤ / 12.40)
  pub fn new_with_src(
    name: Option<String>,
    params: Vec<String>,
    body: Vec<Stmt>,
    src: Option<String>,
    rest_param: Option<(String, usize)>,
  ) -> Self {
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
      is_arrow: false,
      src,
      rest_param,
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
      is_arrow: false,
      src: None,
      rest_param: None,
    }
  }

  /// 创建箭头函数元数据 (12.28): 静态分析与普通函数同源, 只多翻 is_arrow 一位
  pub fn new_arrow(
    params: Vec<String>,
    body: Vec<Stmt>,
    src: Option<String>,
    rest_param: Option<(String, usize)>,
  ) -> Self {
    let mut info = Self::new_with_src(None, params, body, src, rest_param);
    info.is_arrow = true;
    info
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
    Stmt::Labeled { body, .. } => {
      collect_hoisted_stmt(body, hoisted_vars, hoisted_funcs);
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
    // 解构暂态在解析出口已展开成普通 VarDecl, 这里只可能收到旁漏的残形: 盯右值即可
    Stmt::VarPattern { init, .. } => init.as_ref().is_some_and(expr_uses_arguments),
    // for-of 残形同理: 迭代源、赋值目标与体内都照常盯 arguments
    Stmt::ForOf { src, head, body } => {
      expr_uses_arguments(src)
        || stmt_uses_arguments(body)
        || match head {
          ForOfHead::Target(e) => expr_uses_arguments(e),
          ForOfHead::Binding(_) | ForOfHead::Pattern(_) => false,
        }
    }
    // 内部函数声明有其自身的作用域与 arguments，不污染外层作用域
    Stmt::FunctionDecl(_) => false,
    Stmt::Labeled { body, .. } => stmt_uses_arguments(body),
    Stmt::Break(_) | Stmt::Continue(_) | Stmt::Empty => false,
  }
}

fn expr_uses_arguments(expr: &Expr) -> bool {
  match expr {
    Expr::Identifier(name) => name == "arguments",
    Expr::Unary { expr, .. } | Expr::Typeof(expr) => expr_uses_arguments(expr),
    Expr::Binary { left, right, .. } => expr_uses_arguments(left) || expr_uses_arguments(right),
    Expr::Assign { target, value, .. } => expr_uses_arguments(target) || expr_uses_arguments(value),
    // 暂态解构赋值同理 (见 stmt_uses_arguments): 展开后不会走到, 盯右值防御
    Expr::AssignPattern { value, .. } => expr_uses_arguments(value),
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
    Expr::Object(entries) => entries.iter().any(|p| expr_uses_arguments(&p.value)),
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
    // 内部函数表达式有其自身的作用域与 arguments，不污染外层作用域;
    // 箭头例外: 它没有自己的 arguments, 体内引用就是外层的 (12.28)
    Expr::FunctionExpr(shared) => shared.is_arrow && check_uses_arguments(&shared.body),
    Expr::Literal(_) | Expr::RegExp { .. } | Expr::This => false,
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

/// 正则表达式数据封装 (战役 C-2b 第一刀拆家): 内核字段 (pattern/flags/编译体/lastIndex)
/// 整体迁入引擎 `v8_interpreter::RegExpCore` (一处定义, 第二刀 `Value::RegExp` 直持
/// `Arc<RegExpCore>` 跨桥浅拷贝共享身份); 宿主侧只留 expando / 方法遮蔽自有袋 ——
/// 袋值是宿主 `JsValue` 类型, 跨不过 crate 边界, 故与内核分家
#[derive(Clone)]
pub struct RegExpData {
  /// 正则内核 (Arc 共享: 同一实例跨桥往返 lastIndex/编译体保持同一身份)
  pub core: Arc<RegExpCore>,
  /// expando / 方法遮蔽自有袋 (12.37): node 正则实例是普通对象, `r.foo=1`/`r.exec=fn` 落自有键;
  /// 只读访问器面 (source/flags/六布尔) 不入库, lastIndex 走内核专用字段
  pub props: Arc<RwLock<HashMap<String, JsValue>>>,
}

impl RegExpData {
  /// 正则实例的自有属性名册唯一真相 (宿主 getOwnPropertyNames 与 hasOwnProperty 回落两处共用,
  /// 一处定义)。node 实测口径: `lastIndex` 恒自有 (不可枚举, 非全局正则也带) + expando/方法
  /// 遮蔽自有键 (12.37); `source`/`flags`/六布尔是只读访问器、`exec`/`test`/`toString` 基形是
  /// 原型方法 —— 未遮蔽则非自有
  #[must_use]
  pub fn own_property_names(&self) -> Vec<String> {
    let mut names = Vec::with_capacity(self.props.read().len() + 1);
    names.push(LAST_INDEX_KEY.to_string());
    names.extend(self.props.read().keys().cloned());
    names
  }

  /// expando 自有可枚举键 (Object.keys/for-in/propertyIsEnumerable 消费; lastIndex 不可枚举不在此列)
  #[must_use]
  pub fn enumerable_own_names(&self) -> Vec<String> {
    self.props.read().keys().cloned().collect()
  }
}

/// 规范 RegExpSplit 一处定义已迁引擎 `v8_interpreter::regexp::regex_split_parts`
/// (战役 C-2b 拆家, 内核消费); 宿主与桥端经此再导出共读同一份实现
pub use v8_interpreter::regex_split_parts;

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
    // 箭头没有 [[Construct]] 也不挂 prototype 属性: 原型对象与 constructor 回链都不铺
    if shared.is_arrow {
      let func_data = Arc::new(Self {
        shared,
        closure,
        properties: Arc::new(RwLock::new(HashMap::default())),
        native_impl: None,
      });
      return JsValue::Function(func_data);
    }
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
        self.shared.name.clone().unwrap_or_default().into(),
      ));
    }
    let guard = self.properties.read();
    if let Some(val) = guard.get(prop) {
      return Ok(val.clone());
    }
    if let Some(proto) = guard.get(PROTO_KEY) {
      let proto_clone = proto.clone();
      drop(guard);
      // 链顶为 null (Object.create(null) 顶到的函数原型链尾) 即止, 不再向下解引用
      if !matches!(proto_clone, JsValue::Null | JsValue::Undefined) {
        let res = proto_clone.get_property(prop)?;
        if res != JsValue::Undefined {
          return Ok(res);
        }
      }
    } else {
      drop(guard);
    }

    // #23 ⑤: toString 在回落 Function.prototype 之前先解析为源码替身 —— prelude 的
    // Function.prototype 经 __proto__ 会链到 Object.prototype 的 "[object Function]"
    // 分支截胡; Function.prototype 面物化归战役 C (BC 链引擎臂同序)
    if prop == TO_STRING_KEY {
      return Ok(JsValue::Native(NativeFn::FunctionToString(Arc::clone(
        &self.shared,
      ))));
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

/// 属性/方法键的单一真相在下层 `v8_interpreter::bytecode_dispatcher` (与字节码链的
/// 内建表同源, 一处定义两链共读; 与 `@@get:` 前缀键同一收敛策略, 见 12.12)
pub use v8_interpreter::{
  AT_KEY, CHAR_AT_KEY, CHAR_CODE_AT_KEY, CODE_POINT_AT_KEY, CONCAT_KEY, CONSTRUCTOR_KEY,
  COPY_WITHIN_KEY, ENDS_WITH_KEY, EXEC_KEY, FILL_KEY, FIND_INDEX_KEY, FIND_KEY, FLAGS_KEY,
  FLAT_KEY, FLAT_MAP_KEY, GLOBAL_KEY, HAS_OWN_PROPERTY_KEY, INCLUDES_KEY, INDEX_KEY, INDEX_OF_KEY,
  INPUT_KEY, JOIN_KEY, LAST_INDEX_KEY, LAST_INDEX_OF_KEY, LENGTH_KEY, MATCH_KEY, MESSAGE_KEY,
  NAME_KEY, PAD_END_KEY, PAD_START_KEY, POP_KEY, PROPERTY_IS_ENUMERABLE_KEY, PROTO_KEY,
  PROTOTYPE_KEY, PUSH_KEY, REDUCE_RIGHT_KEY, REPEAT_KEY, REPLACE_ALL_KEY, REPLACE_KEY, REVERSE_KEY,
  SHIFT_KEY, SLICE_KEY, SORT_KEY, SOURCE_KEY, SPLICE_KEY, SPLIT_KEY, STARTS_WITH_KEY, SUBSTR_KEY,
  SUBSTRING_KEY, SYM_TO_PRIMITIVE, TEST_KEY, TO_EXPONENTIAL_KEY, TO_FIXED_KEY, TO_LOWER_CASE_KEY,
  TO_PRECISION_KEY, TO_STRING_KEY, TO_UPPER_CASE_KEY, TRIM_END_KEY, TRIM_KEY, TRIM_LEFT_KEY,
  TRIM_RIGHT_KEY, TRIM_START_KEY, UNSHIFT_KEY, VALUE_OF_KEY,
};

/// 宿主构造、无 JS 源可切的函数的字符串化形态 (node 对 native 的输出同款)
#[inline]
pub fn native_function_summary(name: Option<&str>) -> String {
  format!(
    "function {}() {{ [native code] }}",
    name.unwrap_or("anonymous")
  )
}

/// 格式化 Symbol 属性键 (一处定义 = 引擎 `format_symbol_key`, 宿主仅再导出)
pub use v8_interpreter::format_symbol_key;

/// `o[k]` 键控访问的键归一规则
///
/// 读点与写点必须共用这一条 (原先 get_element / set_element 各抄一份): 一处放行另一处不放行
/// 会造成写进 A 键、读 B 键的静默丢值
#[inline]
pub fn element_key(key: &JsValue) -> String {
  match key {
    JsValue::String(s) => s.to_string(),
    JsValue::Symbol(s) => format_symbol_key(s),
    JsValue::Number(n) => double_to_string(*n),
    other => other.to_string(),
  }
}

/// 能直接落元素槽的数组下标: 非负有限整数且不越过数组上限, 其余键走字符串慢路径
///
/// 读点、写点、以及"元素槽是否遮蔽原型回落"的判定共用这一条: 三处各抄一份必然漂移,
/// 一处放行另一处不放行会让同一个下标在不同读点给出不同结果
#[inline]
pub fn is_canonical_index(n: f64) -> bool {
  n >= 0.0 && n.fract() == 0.0 && !n.is_infinite() && n <= (u32::MAX - 1) as f64
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
      Self::Default => JsValue::String("default".into()),
      Self::String => JsValue::String("string".into()),
      Self::Number => JsValue::String("number".into()),
    }
  }
}

/// 宿主数组存储: 元素槽 + 稀疏具名属性槽 (#23)
///
/// `exec`/非全局 `match` 的结果是数组但要挂 `index`/`input` —— 裸 `Vec` 存不下具名键。
/// 稀疏形态对齐引擎侧 `ArrayData` (v8-interpreter): 不存具名键就不分配 `props`;
/// 战役 C 值模型合一后本结构与引擎侧只留一份
#[derive(Debug, Default, Clone)]
pub struct ArrayStore {
  pub elems: Vec<JsValue>,
  pub props: Option<HashMap<String, JsValue>>,
}

impl ArrayStore {
  #[inline]
  pub fn with_elems(items: Vec<JsValue>) -> Self {
    Self {
      elems: items,
      props: None,
    }
  }

  #[inline]
  fn props_mut(&mut self) -> &mut HashMap<String, JsValue> {
    self.props.get_or_insert_with(HashMap::default)
  }
}

impl Deref for ArrayStore {
  type Target = Vec<JsValue>;
  #[inline]
  fn deref(&self) -> &Vec<JsValue> {
    &self.elems
  }
}

impl DerefMut for ArrayStore {
  #[inline]
  fn deref_mut(&mut self) -> &mut Vec<JsValue> {
    &mut self.elems
  }
}

/// 运行时动态值类型
#[derive(Clone)]
pub enum JsValue {
  Undefined,
  /// 数组洞哨兵 (战役 C ③, 与引擎 `Value::TheHole` 同形): 只栖身于数组元素槽
  /// (字面量空位 / `new Array(n)` / `delete a[i]`), 一切用户可见读点在汇合处物化成
  /// `Undefined`; 存在性判定 (`in`/hasOwn/枚举/JSON/join) 按洞口径处理
  TheHole,
  Null,
  Boolean(bool),
  Number(f64),
  BigInt(i128),
  String(Arc<str>),
  Symbol(String),
  Array(Arc<RwLock<ArrayStore>>),
  Object(Arc<RwLock<HashMap<String, JsValue>>>),
  RegExp(Arc<RegExpData>),
  Function(Arc<FunctionData>),
  Native(NativeFn),
}

impl PartialEq for JsValue {
  fn eq(&self, other: &Self) -> bool {
    match (self, other) {
      (Self::Undefined, Self::Undefined)
      | (Self::Null, Self::Null)
      | (Self::TheHole, Self::TheHole) => true,
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
      // C-2b 第二刀: 按内核指针相等 —— 同实例跨桥浅拷贝共享 `Arc<RegExpCore>` 即相等
      // (node 对象身份), 两次字面量各持独立内核不等
      (Self::RegExp(a), Self::RegExp(b)) => Arc::ptr_eq(&a.core, &b.core),
      _ => false,
    }
  }
}

impl fmt::Debug for JsValue {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Self::Undefined => write!(f, "undefined"),
      Self::TheHole => write!(f, "TheHole"),
      Self::Null => write!(f, "null"),
      Self::Boolean(b) => write!(f, "{b}"),
      Self::Number(n) => f.write_str(&double_to_string(*n)),
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
      Self::RegExp(re) => write!(f, "/{}/", re.core.as_str()),
      Self::Native(_) => write!(f, "[NativeFunction]"),
    }
  }
}

impl fmt::Display for JsValue {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Self::Undefined => write!(f, "undefined"),
      Self::TheHole => write!(f, "the_hole"),
      Self::Null => write!(f, "null"),
      Self::Boolean(b) => write!(f, "{b}"),
      Self::Number(n) => f.write_str(&double_to_string(*n)),
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
      Self::RegExp(re) => write!(f, "/{}/{}", re.core.pattern, re.core.flags),
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
    Self::Array(Arc::new(RwLock::new(ArrayStore::with_elems(items))))
  }

  /// 对标 V8 Builtin Inlining: Array.prototype.push 原生就地内联
  #[inline]
  pub fn array_push_inlined(arr: &RwLock<ArrayStore>, args: &[JsValue]) -> JsValue {
    let mut guard = arr.write();
    guard.extend(args.iter().cloned());
    JsValue::Number(guard.len() as f64)
  }

  /// 对标 V8 Builtin Inlining: Array.prototype.pop 原生就地内联
  #[inline]
  pub fn array_pop_inlined(arr: &RwLock<ArrayStore>) -> JsValue {
    let mut guard = arr.write();
    guard.pop().unwrap_or(JsValue::Undefined)
  }

  /// 对标 V8 Builtin Inlining: Array.prototype.shift 原生就地内联
  #[inline]
  pub fn array_shift_inlined(arr: &RwLock<ArrayStore>) -> JsValue {
    let mut guard = arr.write();
    if guard.is_empty() {
      JsValue::Undefined
    } else {
      guard.remove(0)
    }
  }

  /// 对标 V8 Builtin Inlining: Array.prototype.reverse 原生就地内联
  #[inline]
  pub fn array_reverse_inlined(arr: &Arc<RwLock<ArrayStore>>) -> JsValue {
    let mut guard = arr.write();
    guard.reverse();
    JsValue::Array(Arc::clone(arr))
  }

  /// 对标 V8 Builtin Inlining: Array.prototype.splice 原生就地内联
  pub fn array_splice_inlined(arr: &RwLock<ArrayStore>, args: &[JsValue]) -> JsValue {
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
  pub fn array_slice_inlined(arr: &RwLock<ArrayStore>, args: &[JsValue]) -> JsValue {
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

  /// Array 内建方法表的单一真相
  ///
  /// 与字节码链路同构: 方法名在取属性与 `in` 判定两处都要用, 各列一份必然漂移
  pub fn array_builtin(arr: &Arc<RwLock<ArrayStore>>, prop: &str) -> Option<NativeFn> {
    Some(match prop {
      PUSH_KEY => NativeFn::ArrayPush(Arc::clone(arr)),
      POP_KEY => NativeFn::ArrayPop(Arc::clone(arr)),
      SHIFT_KEY => NativeFn::ArrayShift(Arc::clone(arr)),
      SLICE_KEY => NativeFn::ArraySlice(Arc::clone(arr)),
      JOIN_KEY => NativeFn::ArrayJoin(Arc::clone(arr)),
      INDEX_OF_KEY => NativeFn::ArrayIndexOf(Arc::clone(arr)),
      SPLICE_KEY => NativeFn::ArraySplice(Arc::clone(arr)),
      REVERSE_KEY => NativeFn::ArrayReverse(Arc::clone(arr)),
      TO_STRING_KEY => NativeFn::ArrayToString(Arc::clone(arr)),
      SORT_KEY => NativeFn::ArraySort(Arc::clone(arr)),
      INCLUDES_KEY => NativeFn::ArrayIncludes(Arc::clone(arr)),
      CONCAT_KEY => NativeFn::ArrayConcat(Arc::clone(arr)),
      LAST_INDEX_OF_KEY => NativeFn::ArrayLastIndexOf(Arc::clone(arr)),
      FILL_KEY => NativeFn::ArrayFill(Arc::clone(arr)),
      UNSHIFT_KEY => NativeFn::ArrayUnshift(Arc::clone(arr)),
      FIND_KEY => NativeFn::ArrayFind(Arc::clone(arr)),
      FIND_INDEX_KEY => NativeFn::ArrayFindIndex(Arc::clone(arr)),
      REDUCE_RIGHT_KEY => NativeFn::ArrayReduceRight(Arc::clone(arr)),
      FLAT_MAP_KEY => NativeFn::ArrayFlatMap(Arc::clone(arr)),
      AT_KEY => NativeFn::ArrayAt(Arc::clone(arr)),
      FLAT_KEY => NativeFn::ArrayFlat(Arc::clone(arr)),
      COPY_WITHIN_KEY => NativeFn::ArrayCopyWithin(Arc::clone(arr)),
      _ => return None,
    })
  }

  /// ECMA-262 ToIntegerOrInfinity 后按尾部归一下标 (与引擎 `tail_index` 同口径):
  /// indexOf/includes 起跳与 fill 界共用; undefined 交回调用方缺省, NaN→0, 越界钳到 [0,len]
  #[inline]
  fn tail_index(v: &JsValue, len: i64, on_undefined: i64) -> i64 {
    if matches!(v, JsValue::Undefined) {
      return on_undefined;
    }
    let n = v.to_number();
    if n.is_nan() {
      0
    } else if n >= len as f64 {
      len
    } else if n <= -(len as f64) {
      0
    } else if n < 0.0 {
      len + n.trunc() as i64
    } else {
      n.trunc() as i64
    }
  }

  /// 对标 V8 Builtin Inlining: Array.prototype.includes 原生就地内联
  pub fn array_includes_inlined(arr: &RwLock<ArrayStore>, args: &[JsValue]) -> JsValue {
    let target = args.first().unwrap_or(&JsValue::Undefined);
    // SameValueZero = 严格相等 + NaN 自等 (+0/-0 在严格相等下本就相等)
    let target_is_nan = matches!(target, JsValue::Number(n) if n.is_nan());
    // 第二实参 fromIndex 与 indexOf 同口径起跳 (负数从尾部归一)
    let guard = arr.read();
    let from_idx = args
      .get(1)
      .map(|v| Self::tail_index(v, guard.len() as i64, 0))
      .unwrap_or(0) as usize;
    JsValue::Boolean(guard.iter().skip(from_idx).any(|v| {
      // 战役 C ③: 洞当 undefined 比 (node `[1,,2].includes(undefined)` 真)
      let v = match v {
        JsValue::TheHole => &JsValue::Undefined,
        other => other,
      };
      if target_is_nan {
        matches!(v, JsValue::Number(n) if n.is_nan())
      } else {
        v == target
      }
    }))
  }

  /// 对标 V8 Builtin Inlining: Array.prototype.concat 原生就地内联
  pub fn array_concat_inlined(arr: &RwLock<ArrayStore>, args: &[JsValue]) -> JsValue {
    let mut out: Vec<JsValue> = arr.read().iter().cloned().collect();
    for arg in args {
      match arg {
        JsValue::Array(other) => out.extend(other.read().iter().cloned()),
        _ => out.push(arg.clone()),
      }
    }
    JsValue::new_array(out)
  }

  /// 对标 V8 Builtin Inlining: Array.prototype.indexOf 原生就地内联
  pub fn array_index_of_inlined(arr: &RwLock<ArrayStore>, args: &[JsValue]) -> JsValue {
    let target = args.first().unwrap_or(&JsValue::Undefined);
    let guard = arr.read();
    let from_idx = args
      .get(1)
      .map(|v| Self::tail_index(v, guard.len() as i64, 0))
      .unwrap_or(0) as usize;
    let pos = guard
      .iter()
      .enumerate()
      .skip(from_idx)
      .find(|(_, v)| *v == target)
      .map(|(i, _)| i as f64)
      .unwrap_or(-1.0);
    JsValue::Number(pos)
  }

  /// 对标 V8 Builtin Inlining: Array.prototype.lastIndexOf 原生就地内联
  ///
  /// 规范用严格相等 (NaN 永不命中, 与 includes 的 SameValueZero 相区分);
  /// fromIndex 为负从尾部起, 缺省 len-1
  pub fn array_last_index_of_inlined(arr: &RwLock<ArrayStore>, args: &[JsValue]) -> JsValue {
    let target = args.first().unwrap_or(&JsValue::Undefined);
    let guard = arr.read();
    let len = guard.len() as i64;
    let from = match args.get(1) {
      Some(v) => {
        let n = v.to_int32() as i64;
        if n < 0 { len + n } else { n }
      }
      None => len - 1,
    };
    let pos = if from < 0 || len == 0 {
      -1.0
    } else {
      let upper = (from as usize).min(guard.len() - 1);
      guard[..=upper]
        .iter()
        .rposition(|v| v == target)
        .map_or(-1.0, |i| i as f64)
    };
    JsValue::Number(pos)
  }

  /// 对标 V8 Builtin Inlining: Array.prototype.fill 原生就地内联
  ///
  /// start/end 走 `tail_index` 界归一 (undefined 缺省 0/len, NaN→0); 返回接收者本身
  pub fn array_fill_inlined(arr: &Arc<RwLock<ArrayStore>>, args: &[JsValue]) -> JsValue {
    let value = args.first().cloned().unwrap_or(JsValue::Undefined);
    let mut guard = arr.write();
    let len = guard.len() as i64;
    let start = args.get(1).map_or(0, |v| Self::tail_index(v, len, 0));
    let end = args.get(2).map_or(len, |v| Self::tail_index(v, len, len));
    if start < end {
      for slot in &mut guard[start as usize..end as usize] {
        *slot = value.clone();
      }
    }
    drop(guard);
    JsValue::Array(Arc::clone(arr))
  }

  /// 对标 V8 Builtin Inlining: Array.prototype.unshift 原生就地内联; 返回新长度
  pub fn array_unshift_inlined(arr: &RwLock<ArrayStore>, args: &[JsValue]) -> JsValue {
    let mut guard = arr.write();
    if !args.is_empty() {
      guard.splice(..0, args.iter().cloned());
    }
    JsValue::Number(guard.len() as f64)
  }

  /// 对标 V8 Builtin Inlining: Array.prototype.join 原生就地内联
  pub fn array_join_inlined(arr: &RwLock<ArrayStore>, args: &[JsValue]) -> JsValue {
    let sep_holder;
    let sep: &str = match args.first() {
      None | Some(JsValue::Undefined) => ",",
      Some(JsValue::String(s)) => s,
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
        JsValue::Undefined | JsValue::Null | JsValue::TheHole => {}
        JsValue::String(s) => joined.push_str(s),
        _ => joined.push_str(&v.to_js_string()),
      }
    }
    JsValue::String(joined.into())
  }

  /// 对标 V8 Builtin Inlining: Array.prototype.toString 原生就地内联
  #[inline]
  pub fn array_to_string_inlined(arr: &RwLock<ArrayStore>) -> JsValue {
    Self::array_join_inlined(arr, &[])
  }

  /// 编译并创建正则表达式 (规范化编译内核在引擎 `RegExpCore::new`, 一处定义;
  /// 宿主只保留袋的挂载与错误包装形态)
  pub fn new_regexp(pattern: &str, flags: &str) -> Result<Self> {
    let core = RegExpCore::new(pattern, flags)
      .map_err(|e| Error::RuntimeError(format!("Invalid RegExp '{pattern}': {e}")))?;
    Ok(Self::RegExp(Arc::new(RegExpData {
      core: Arc::new(core),
      props: Arc::new(RwLock::new(HashMap::default())),
    })))
  }

  /// 以既有内核复用构造正则实例 (C-2b 第二刀桥端还原: 不重编译, lastIndex 与内核 Arc 共享,
  /// 袋整表一次落位)
  #[must_use]
  pub fn regexp_from_core(core: Arc<RegExpCore>, props: HashMap<String, Self>) -> Self {
    Self::RegExp(Arc::new(RegExpData {
      core,
      props: Arc::new(RwLock::new(props)),
    }))
  }

  /// 抽象相等比较
  pub fn abstract_eq(&self, other: &Self) -> bool {
    match (self, other) {
      (Self::Undefined, Self::Undefined)
      | (Self::Null, Self::Null)
      | (Self::TheHole, Self::TheHole) => true,
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
      Self::Array(_) => Self::String(self.to_js_string().into()),
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
        Self::String(self.to_js_string().into())
      }
      Self::RegExp(re) => Self::String(format!("/{}/{}", re.core.pattern, re.core.flags).into()),
      Self::Function(_) | Self::Native(_) => Self::String(self.to_js_string().into()),
      _ => self.clone(),
    }
  }

  /// 转换为字符串
  pub fn to_js_string(&self) -> String {
    match self {
      Self::Undefined => "undefined".to_string(),
      Self::TheHole => "the_hole".to_string(),
      Self::Null => "null".to_string(),
      Self::Boolean(b) => b.to_string(),
      Self::Number(n) => double_to_string(*n),
      Self::BigInt(n) => {
        let mut buf = Buffer::new();
        buf.format(*n).to_string()
      }
      Self::String(s) => s.to_string(),
      Self::Symbol(s) => format!("Symbol({s})"),
      Self::Array(arr) => {
        let guard = arr.read();
        let mut result = String::new();
        for (i, item) in guard.iter().enumerate() {
          if i > 0 {
            result.push(',');
          }
          match item {
            Self::Undefined | Self::Null | Self::TheHole => {}
            Self::String(s) => result.push_str(s),
            _ => result.push_str(&item.to_js_string()),
          }
        }
        result
      }
      Self::Object(_) => "[object Object]".to_string(),
      Self::RegExp(re) => format!("/{}/{}", re.core.pattern, re.core.flags),
      Self::Function(func) => match &func.shared.src {
        Some(s) => s.clone(),
        None => native_function_summary(func.name()),
      },
      // 无自有属性的字节码闭包落成 JsValue::Native, 字符串化同样走源码文本 (#23 ⑤)
      Self::Native(NativeFn::InterpFunction(closure, _)) => match &closure.func.src {
        Some(s) => s.clone(),
        None => native_function_summary(closure.func.name.as_deref()),
      },
      Self::Native(_) => "function () { [native code] }".to_string(),
    }
  }

  /// 转换为布尔值
  pub fn to_boolean(&self) -> bool {
    match self {
      Self::Undefined | Self::Null | Self::TheHole => false,
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
      Self::Undefined | Self::TheHole => f64::NAN,
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
        // ECMA-262 9.3.1 全量口径: 0x/0o/0b 前缀、±Infinity、空白由 V8 移植的
        // `string_to_double` 一处判定 (原 Rust parse 只认十进制, "0b101" 会错吐 NaN)
        string_to_double(s.as_bytes(), ConversionFlag::AllowNonDecimalPrefix, 0.0)
      }
      Self::Array(arr) => {
        let guard = arr.read();
        if guard.is_empty() {
          0.0
        } else if guard.len() == 1 {
          // 单元素走 ToPrimitive=join 口径: 洞/undefined/null 物化空串转数得 0 (node `+[undefined]`=0)
          match &guard[0] {
            JsValue::TheHole | JsValue::Undefined | JsValue::Null => 0.0,
            other => other.to_number(),
          }
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

  /// 数组读面物化 (战役 C ③, 与引擎 `Value::materialized` 同形): 洞哨兵只栖身元素槽,
  /// 出槽即 undefined —— 一处定义; `in`/keys/for-in/JSON 各有自己的册不走这里
  #[inline]
  fn materialized(val: &Self) -> Self {
    match val {
      Self::TheHole => Self::Undefined,
      other => other.clone(),
    }
  }

  /// 索引元素读取
  pub fn get_element(&self, key: &JsValue) -> Result<JsValue> {
    match key {
      JsValue::Number(n) => {
        if is_canonical_index(*n) {
          let uidx = *n as usize;
          match self {
            Self::Array(arr) => {
              let guard = arr.read();
              return Ok(
                guard
                  .get(uidx)
                  .map_or(JsValue::Undefined, Self::materialized),
              );
            }
            Self::String(s) => {
              if let Some(&b) = s.as_bytes().get(uidx)
                && b < 0x80
              {
                return Ok(JsValue::String((b as char).to_string().into()));
              } else if let Some(ch) = s.chars().nth(uidx) {
                return Ok(JsValue::String(ch.to_string().into()));
              }
              return Ok(JsValue::Undefined);
            }
            _ => {}
          }
        }
        self.get_property(&element_key(key))
      }
      other => self.get_property(&element_key(other)),
    }
  }

  /// 索引元素写入
  pub fn set_element(&self, key: &JsValue, val: JsValue) -> Result<()> {
    match key {
      JsValue::Number(n) => {
        if is_canonical_index(*n)
          && let Self::Array(arr) = self
        {
          let uidx = *n as usize;
          let mut guard = arr.write();
          if uidx >= guard.len() {
            // 中间空位交洞 (node `a=[1]; a[3]=2` 后 1/2 是洞非属性), 战役 C ③
            guard.resize(uidx + 1, JsValue::TheHole);
          }
          // 写入侧兜底: 泄漏的洞哨兵落槽即物化 (同引擎 set_element)
          guard[uidx] = Self::materialized(&val);
          return Ok(());
        }
        self.set_property(&element_key(key), val)
      }
      other => self.set_property(&element_key(other), val),
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
          return Ok(
            guard
              .get(idx)
              .map_or(JsValue::Undefined, Self::materialized),
          );
        }
        // 自有具名键先于内建替身 (对齐引擎侧 Array 读序): exec 结果的 index/input 由此可见
        if let Some(props) = arr.read().props.as_ref()
          && let Some(val) = props.get(prop)
        {
          return Ok(val.clone());
        }
        // 内建方法名单一真相: `in` 判定与本处共用同一张表, 各列一份必然漂移
        Ok(Self::array_builtin(arr, prop).map_or(JsValue::Undefined, JsValue::Native))
      }

      Self::Object(obj) => {
        let guard = obj.read();
        if let Some(val) = guard.get(prop) {
          return Ok(val.clone());
        }
        let proto_opt = guard.get(PROTO_KEY).cloned();
        drop(guard);
        // 链显式止于 null (Object.create(null) 产物): 无任何继承面, 直连的内建 toString/valueOf
        // 兜底也不给 (node `Object.create(null).toString` 交 undefined); 无 `__proto__` 键的
        // 模型对象才沿用下方兜底近似
        if matches!(proto_opt, Some(JsValue::Null) | Some(JsValue::Undefined)) {
          return Ok(JsValue::Undefined);
        }
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

      Self::Function(func) => {
        // toString 的源码替身在 FunctionData::get_property 内部兜底 (一处定义)
        func.get_property(prop)
      }

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
            return Ok(JsValue::String((b as char).to_string().into()));
          } else if let Some(ch) = s.chars().nth(idx) {
            return Ok(JsValue::String(ch.to_string().into()));
          }
          return Ok(JsValue::Undefined);
        }
        match prop {
          CHAR_AT_KEY => Ok(JsValue::Native(NativeFn::StringCharAt(s.clone()))),
          CHAR_CODE_AT_KEY => Ok(JsValue::Native(NativeFn::StringCharCodeAt(s.clone()))),
          INDEX_OF_KEY => Ok(JsValue::Native(NativeFn::StringIndexOf(s.clone()))),
          LAST_INDEX_OF_KEY => Ok(JsValue::Native(NativeFn::StringLastIndexOf(s.clone()))),
          SUBSTRING_KEY => Ok(JsValue::Native(NativeFn::StringSubstring(s.clone()))),
          SLICE_KEY => Ok(JsValue::Native(NativeFn::StringSlice(s.clone()))),
          SUBSTR_KEY => Ok(JsValue::Native(NativeFn::StringSubstr(s.clone()))),
          CONCAT_KEY => Ok(JsValue::Native(NativeFn::StringConcat(s.clone()))),
          REPLACE_KEY => Ok(JsValue::Native(NativeFn::StringReplace(s.clone()))),
          SPLIT_KEY => Ok(JsValue::Native(NativeFn::StringSplit(s.clone()))),
          MATCH_KEY => Ok(JsValue::Native(NativeFn::StringMatch(s.clone()))),
          TO_LOWER_CASE_KEY => Ok(JsValue::Native(NativeFn::StringToLowerCase(s.clone()))),
          TO_UPPER_CASE_KEY => Ok(JsValue::Native(NativeFn::StringToUpperCase(s.clone()))),
          INCLUDES_KEY => Ok(JsValue::Native(NativeFn::StringIncludes(s.clone()))),
          REPEAT_KEY => Ok(JsValue::Native(NativeFn::StringRepeat(s.clone()))),
          PAD_START_KEY => Ok(JsValue::Native(NativeFn::StringPad(s.clone(), true))),
          PAD_END_KEY => Ok(JsValue::Native(NativeFn::StringPad(s.clone(), false))),
          AT_KEY => Ok(JsValue::Native(NativeFn::StringAt(s.clone()))),
          CODE_POINT_AT_KEY => Ok(JsValue::Native(NativeFn::StringCodePointAt(s.clone()))),
          TRIM_KEY => Ok(JsValue::Native(NativeFn::StringTrim(
            s.clone(),
            TrimKind::Both,
          ))),
          TRIM_START_KEY | TRIM_LEFT_KEY => Ok(JsValue::Native(NativeFn::StringTrim(
            s.clone(),
            TrimKind::Start,
          ))),
          TRIM_END_KEY | TRIM_RIGHT_KEY => Ok(JsValue::Native(NativeFn::StringTrim(
            s.clone(),
            TrimKind::End,
          ))),
          STARTS_WITH_KEY => Ok(JsValue::Native(NativeFn::StringStartsWith(s.clone()))),
          ENDS_WITH_KEY => Ok(JsValue::Native(NativeFn::StringEndsWith(s.clone()))),
          REPLACE_ALL_KEY => Ok(JsValue::Native(NativeFn::StringReplaceAll(s.clone()))),
          TO_STRING_KEY | VALUE_OF_KEY => Ok(JsValue::Native(NativeFn::StringToString(s.clone()))),
          // Object.prototype 面的基元回落: 宿主闭表不遍历原型, 逐键显式对齐引擎 boxing
          HAS_OWN_PROPERTY_KEY => Ok(JsValue::Native(NativeFn::StringHasOwnProperty(s.clone()))),
          _ => Ok(JsValue::Undefined),
        }
      }

      Self::Number(n) => {
        let num = *n;
        match prop {
          TO_FIXED_KEY => Ok(JsValue::Native(NativeFn::NumberToFixed(num))),
          TO_PRECISION_KEY => Ok(JsValue::Native(NativeFn::NumberToPrecision(num))),
          TO_EXPONENTIAL_KEY => Ok(JsValue::Native(NativeFn::NumberToExponential(num))),
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
        // 自有 expando/方法遮蔽袋先查 (own 强于原型面, node e8/e10: r.exec=fn 遮蔽生效);
        // 只读访问器名与 lastIndex 不会入袋 (set 臂拒收/专存), 置顶无歧义
        if let Some(v) = re.props.read().get(prop) {
          return Ok(v.clone());
        }
        if prop == SOURCE_KEY {
          return Ok(JsValue::String(re.core.pattern.clone().into()));
        }
        if prop == FLAGS_KEY {
          return Ok(JsValue::String(re.core.flags.clone().into()));
        }
        if prop == LAST_INDEX_KEY {
          let idx = *re.core.last_index.read();
          return Ok(JsValue::Number(idx as f64));
        }
        // 六个布尔标志读值与 bridge 落表同源 (boolean_flags 一张表)
        if let Some((_, v)) = re.core.boolean_flags().iter().find(|(n, _)| *n == prop) {
          return Ok(JsValue::Boolean(*v));
        }
        match prop {
          TEST_KEY => Ok(JsValue::Native(NativeFn::RegExpTest(Arc::clone(re)))),
          EXEC_KEY => Ok(JsValue::Native(NativeFn::RegExpExec(Arc::clone(re)))),
          // Object.prototype 继承面 (与 String 基元回落同款显式对齐, 消除 AST/BC 链分裂:
          // BC 经原型链取 prelude JS 函数, 宿主闭表在此物化同语义原生)
          HAS_OWN_PROPERTY_KEY => Ok(JsValue::Native(NativeFn::RegExpHasOwnProperty(Arc::clone(
            re,
          )))),
          PROPERTY_IS_ENUMERABLE_KEY => Ok(JsValue::Native(NativeFn::RegExpPropertyIsEnumerable(
            Arc::clone(re),
          ))),
          TO_STRING_KEY => Ok(JsValue::Native(NativeFn::StringToString(
            format!("/{}/{}", re.core.pattern, re.core.flags).into(),
          ))),
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
          // 拉长交洞 (node `a.length=3` 后 `0 in a` 假), 与引擎 set_property length 臂同口径
          guard.resize(val.to_int32().max(0) as usize, JsValue::TheHole);
          return Ok(());
        }
        if prop.as_bytes().first().is_some_and(|b| b.is_ascii_digit())
          && let Ok(idx) = prop.parse::<usize>()
        {
          let mut guard = arr.write();
          if idx >= guard.len() {
            // 中间空位交洞, 同 set_element
            guard.resize(idx + 1, JsValue::TheHole);
          }
          guard[idx] = Self::materialized(&val);
          return Ok(());
        }
        // 具名键落稀疏槽 (裸 Vec 时代被静默丢弃, #23)
        arr.write().props_mut().insert(prop.to_string(), val);
        Ok(())
      }
      Self::RegExp(re) => {
        if prop == LAST_INDEX_KEY {
          *re.core.last_index.write() = val.to_int32().max(0) as usize;
        } else if is_regexp_readonly_key(prop) {
          // 只读访问器赋值静默忽略 (node sloppy: `r.source="X"` 无效, 读回原 pattern);
          // 名册单点在引擎 bytecode_dispatcher, 宿主与引擎 set 护栏共读同一张表
        } else {
          // expando / 方法遮蔽自有键 (exec/test/toString 可被实例自有键遮蔽, node e8/e9/e10)
          re.props.write().insert(prop.to_string(), val);
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
