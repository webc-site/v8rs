//! 抽象语法树解释执行器
//!
//! 零拷贝与高效现代 Rust 实现，支持端到端 JavaScript 解释执行

use std::{
  borrow::Cow,
  f64::consts::{E, PI},
  result::Result as StdResult,
  sync::Arc,
  time::Instant,
};

use gxhash::HashMap;
use itoa::Buffer;
use parking_lot::RwLock;
use v8_interpreter::{TIME_KEY, TIMESTAMP_KEY};
use v8_numbers::{double_to_int32, double_to_uint32};

use crate::{
  error::{Error, Result},
  runtime::{
    ast::*,
    bytecode_bridge::try_execute_with_bytecode_and_env,
    environment::{ActivationScopeInit, Environment},
    lexer::Lexer,
    parser::Parser,
    value::{
      CONSTRUCTOR_KEY, FunctionData, GET_SYM_TO_PRIMITIVE, GETTER_PREFIX, JsValue, MESSAGE_KEY,
      NAME_KEY, NativeFn, PROTO_KEY, PROTOTYPE_KEY, PreferredType, SYM_TO_PRIMITIVE, TO_STRING_KEY,
      VALUE_OF_KEY, char_count, collect_hoisted, format_getter_key, format_symbol_key,
    },
  },
};

/// 位移操作掩码
const SHIFT_MASK: u32 = 0x1F;

/// 内部控制流中断与错误信号
#[derive(Debug)]
pub(crate) enum ExecSignal {
  Return(JsValue),
  Break,
  Continue,
  Error(Error),
}

impl From<Error> for ExecSignal {
  fn from(err: Error) -> Self {
    Self::Error(err)
  }
}

impl From<ExecSignal> for Error {
  fn from(sig: ExecSignal) -> Self {
    match sig {
      ExecSignal::Error(e) => e,
      ExecSignal::Return(_) => Error::RuntimeError("unexpected return".to_string()),
      ExecSignal::Break => Error::RuntimeError("unexpected break".to_string()),
      ExecSignal::Continue => Error::RuntimeError("unexpected continue".to_string()),
    }
  }
}

/// 执行路径选择模式
#[derive(Debug, Default, PartialEq, Eq, Clone, Copy)]
pub enum ExecMode {
  /// 强制 AST 求值器。当前默认: 字节码引擎尚有 4 类结构性缺口 (见 todo.md 三·战役 A 的欠账清单),
  /// 翻转默认值的前提是 `./test.sh` 在 BytecodeFirst 下全绿
  #[default]
  AstOnly,
  /// 优先走 V8 Ignition 字节码引擎, 桥接层执行前判定不支持的构造回落 AST 求值器
  BytecodeFirst,
  /// 只允许字节码引擎, 不支持即报错, 用于战役 A 收官门禁
  BytecodeOnly,
}

/// 执行路径计数, 用于量化主链路真实打通比例
#[derive(Debug, Default, Clone, Copy)]
pub struct ExecStats {
  /// 进入调度的程序 (脚本片段) 总数
  pub total: usize,
  /// 由字节码引擎完成的次数
  pub bytecode: usize,
  /// 桥接层不支持该构造而回落 AST 的次数 (仅在执行开始前判定, 无副作用重放)
  pub fallback: usize,
}

impl ExecStats {
  /// 字节码路径占比, 战役 A 的准入门禁读数
  pub fn coverage(&self) -> f64 {
    if self.total == 0 {
      0.0
    } else {
      self.bytecode as f64 / self.total as f64
    }
  }
}

pub struct Interpreter {
  pub global: Arc<Environment>,
  pub call_stack: Vec<String>,
  pub mode: ExecMode,
  pub stats: ExecStats,
}

impl Default for Interpreter {
  fn default() -> Self {
    Self::new()
  }
}

impl Interpreter {
  pub fn new() -> Self {
    let global = Arc::new(Environment::new());
    Self::init_builtins(&global);
    Self {
      global,
      call_stack: Vec::new(),
      mode: ExecMode::default(),
      stats: ExecStats::default(),
    }
  }

  /// 跨界调用 (字节码引擎回调宿主函数) 专用的轻量执行态
  ///
  /// 被调函数的变量解析只走 `FunctionData.closure` 派生出的激活记录, `self.global` 仅在
  /// `run()`/`dispatch()` 顶层使用, 所以这里必须跳过 `init_builtins`: 重放整段内建预置脚本
  /// (词法分析 + 语法分析 + 求值近千行) 曾是每次跨界调用的固定开销 (战役 A 欠账 1)。
  /// 若日后让语句求值依赖 `self.global`, 此构造函数必须改为接收真实全局环境。
  #[must_use]
  pub fn for_cross_engine_call() -> Self {
    Self {
      global: Arc::new(Environment::new()),
      call_stack: Vec::new(),
      mode: ExecMode::AstOnly,
      stats: ExecStats::default(),
    }
  }

  fn init_builtins(env: &Arc<Environment>) {
    // 全局基础字面量与数学极值
    env.define("undefined".to_string(), JsValue::Undefined);
    env.define("NaN".to_string(), JsValue::Number(f64::NAN));
    env.define("Infinity".to_string(), JsValue::Number(f64::INFINITY));

    let alert_fn = NativeFn::Ptr(|args: &[JsValue]| -> Result<JsValue> {
      eprintln!("[JS ALERT] {:?}", args);
      Ok(JsValue::Undefined)
    });
    env.define("alert".to_string(), JsValue::Native(alert_fn));

    // 全局类型与数值转换函数
    let is_nan_fn = NativeFn::Ptr(|args: &[JsValue]| -> Result<JsValue> {
      let is_nan = args.first().map(|v| v.to_number().is_nan()).unwrap_or(true);
      Ok(JsValue::Boolean(is_nan))
    });
    env.define("isNaN".to_string(), JsValue::Native(is_nan_fn.clone()));

    let is_finite_fn = NativeFn::Ptr(|args: &[JsValue]| -> Result<JsValue> {
      let is_fin = args
        .first()
        .map(|v| {
          let n = v.to_number();
          !n.is_nan() && !n.is_infinite()
        })
        .unwrap_or(false);
      Ok(JsValue::Boolean(is_fin))
    });
    env.define(
      "isFinite".to_string(),
      JsValue::Native(is_finite_fn.clone()),
    );

    let parse_int_fn = NativeFn::Ptr(|args: &[JsValue]| -> Result<JsValue> {
      let s = args.first().map(|v| v.to_string()).unwrap_or_default();
      let trimmed = s.trim();
      let mut radix = args.get(1).map(|v| v.to_int32()).unwrap_or(10);
      if radix == 0 {
        radix = 10;
      }
      if !(2..=36).contains(&radix) {
        return Ok(JsValue::Number(f64::NAN));
      }
      let mut chars = trimmed.chars().peekable();
      let sign = if let Some(&c) = chars.peek() {
        if c == '+' || c == '-' {
          chars.next();
          if c == '-' { -1.0 } else { 1.0 }
        } else {
          1.0
        }
      } else {
        return Ok(JsValue::Number(f64::NAN));
      };
      if radix == 16 && (trimmed.starts_with("0x") || trimmed.starts_with("0X")) {
        chars.next();
        chars.next();
      }
      let mut digits = String::new();
      for c in chars {
        if c.is_digit(radix as u32) {
          digits.push(c);
        } else {
          break;
        }
      }
      if digits.is_empty() {
        Ok(JsValue::Number(f64::NAN))
      } else if let Ok(val) = i64::from_str_radix(&digits, radix as u32) {
        Ok(JsValue::Number((val as f64) * sign))
      } else {
        Ok(JsValue::Number(f64::NAN))
      }
    });
    env.define(
      "parseInt".to_string(),
      JsValue::Native(parse_int_fn.clone()),
    );

    let parse_float_fn = NativeFn::Ptr(|args: &[JsValue]| -> Result<JsValue> {
      let s = args.first().map(|v| v.to_string()).unwrap_or_default();
      let trimmed = s.trim();
      let mut end = trimmed.len();
      while end > 0 {
        if let Ok(f) = trimmed[..end].parse::<f64>() {
          return Ok(JsValue::Number(f));
        }
        end -= 1;
      }
      Ok(JsValue::Number(f64::NAN))
    });
    env.define(
      "parseFloat".to_string(),
      JsValue::Native(parse_float_fn.clone()),
    );

    let eval_fn = NativeFn::Ptr(|args: &[JsValue]| -> Result<JsValue> {
      if let Some(JsValue::String(code)) = args.first() {
        run_js(code)
      } else if let Some(other) = args.first() {
        Ok(other.clone())
      } else {
        Ok(JsValue::Undefined)
      }
    });
    env.define("eval".to_string(), JsValue::Native(eval_fn));

    // console 对象
    let console = JsValue::new_object();
    let log_fn = NativeFn::Ptr(|args: &[JsValue]| -> Result<JsValue> {
      let line = args
        .iter()
        .map(|v| v.to_string())
        .collect::<Vec<_>>()
        .join(" ");
      println!("{line}");
      Ok(JsValue::Undefined)
    });
    let _ = console.set_property("log", JsValue::Native(log_fn.clone()));

    let timers: Arc<RwLock<HashMap<String, Instant>>> = Arc::new(RwLock::new(HashMap::default()));
    let time_fn = NativeFn::ConsoleTime(Arc::clone(&timers));
    let _ = console.set_property("time", JsValue::Native(time_fn));

    let time_end_fn = NativeFn::ConsoleTimeEnd(Arc::clone(&timers));
    let _ = console.set_property("timeEnd", JsValue::Native(time_end_fn));

    env.define("console".to_string(), console);

    // Math 对象
    let math = JsValue::new_object();
    let floor_fn = NativeFn::Ptr(|args: &[JsValue]| -> Result<JsValue> {
      let n = args.first().map(|v| v.to_number()).unwrap_or(f64::NAN);
      Ok(JsValue::Number(n.floor()))
    });
    let _ = math.set_property("floor", JsValue::Native(floor_fn));

    let abs_fn = NativeFn::Ptr(|args: &[JsValue]| -> Result<JsValue> {
      let n = args.first().map(|v| v.to_number()).unwrap_or(f64::NAN);
      Ok(JsValue::Number(n.abs()))
    });
    let _ = math.set_property("abs", JsValue::Native(abs_fn));

    let sqrt_fn = NativeFn::Ptr(|args: &[JsValue]| -> Result<JsValue> {
      let n = args.first().map(|v| v.to_number()).unwrap_or(f64::NAN);
      Ok(JsValue::Number(n.sqrt()))
    });
    let _ = math.set_property("sqrt", JsValue::Native(sqrt_fn));

    let sin_fn = NativeFn::Ptr(|args: &[JsValue]| -> Result<JsValue> {
      let n = args.first().map(|v| v.to_number()).unwrap_or(f64::NAN);
      Ok(JsValue::Number(n.sin()))
    });
    let _ = math.set_property("sin", JsValue::Native(sin_fn));

    let cos_fn = NativeFn::Ptr(|args: &[JsValue]| -> Result<JsValue> {
      let n = args.first().map(|v| v.to_number()).unwrap_or(f64::NAN);
      Ok(JsValue::Number(n.cos()))
    });
    let _ = math.set_property("cos", JsValue::Native(cos_fn));

    let min_fn = NativeFn::Ptr(|args: &[JsValue]| -> Result<JsValue> {
      let mut m = f64::INFINITY;
      for a in args {
        let n = a.to_number();
        if n.is_nan() {
          return Ok(JsValue::Number(f64::NAN));
        }
        if n < m {
          m = n;
        }
      }
      Ok(JsValue::Number(m))
    });
    let _ = math.set_property("min", JsValue::Native(min_fn));

    let max_fn = NativeFn::Ptr(|args: &[JsValue]| -> Result<JsValue> {
      let mut m = f64::NEG_INFINITY;
      for a in args {
        let n = a.to_number();
        if n.is_nan() {
          return Ok(JsValue::Number(f64::NAN));
        }
        if n > m {
          m = n;
        }
      }
      Ok(JsValue::Number(m))
    });
    let _ = math.set_property("max", JsValue::Native(max_fn));

    let log_fn_clone = log_fn.clone();
    env.define("print".to_string(), JsValue::Native(log_fn_clone));

    let pow_fn = NativeFn::Ptr(|args: &[JsValue]| -> Result<JsValue> {
      let base = args.first().map(|v| v.to_number()).unwrap_or(f64::NAN);
      let exp = args.get(1).map(|v| v.to_number()).unwrap_or(f64::NAN);
      Ok(JsValue::Number(base.powf(exp)))
    });
    let _ = math.set_property("pow", JsValue::Native(pow_fn));

    let log_math_fn = NativeFn::Ptr(|args: &[JsValue]| -> Result<JsValue> {
      let n = args.first().map(|v| v.to_number()).unwrap_or(f64::NAN);
      Ok(JsValue::Number(n.ln()))
    });
    let _ = math.set_property("log", JsValue::Native(log_math_fn));

    let exp_fn = NativeFn::Ptr(|args: &[JsValue]| -> Result<JsValue> {
      let n = args.first().map(|v| v.to_number()).unwrap_or(f64::NAN);
      Ok(JsValue::Number(n.exp()))
    });
    let _ = math.set_property("exp", JsValue::Native(exp_fn));

    let round_fn = NativeFn::Ptr(|args: &[JsValue]| -> Result<JsValue> {
      let n = args.first().map(|v| v.to_number()).unwrap_or(f64::NAN);
      Ok(JsValue::Number(n.round()))
    });
    let _ = math.set_property("round", JsValue::Native(round_fn));

    let _ = math.set_property("PI", JsValue::Number(PI));
    let _ = math.set_property("E", JsValue::Number(E));

    env.define("Math".to_string(), math);

    // String 构造函数与静态方法
    let str_ctor = NativeFn::Ptr(|args: &[JsValue]| -> Result<JsValue> {
      let val = args.first().map(|v| v.to_string()).unwrap_or_default();
      Ok(JsValue::String(val))
    });
    let string_data =
      FunctionData::new_native(Some("String".to_string()), Arc::clone(env), str_ctor);
    let from_char_code = NativeFn::Ptr(|args: &[JsValue]| -> Result<JsValue> {
      let mut s = String::with_capacity(args.len());
      for a in args {
        let code = a.to_int32();
        if let Some(ch) = char::from_u32(code as u32) {
          s.push(ch);
        }
      }
      Ok(JsValue::String(s))
    });
    let _ = string_data.set_property("fromCharCode", JsValue::Native(from_char_code));
    env.define(
      "String".to_string(),
      JsValue::Function(Arc::new(string_data)),
    );

    // Number 构造函数与静态常量/方法
    let number_ctor = NativeFn::Ptr(|args: &[JsValue]| -> Result<JsValue> {
      let val = args.first().map(|v| v.to_number()).unwrap_or(0.0);
      Ok(JsValue::Number(val))
    });
    let number_data =
      FunctionData::new_native(Some("Number".to_string()), Arc::clone(env), number_ctor);
    let _ = number_data.set_property("MAX_VALUE", JsValue::Number(f64::MAX));
    let _ = number_data.set_property("MIN_VALUE", JsValue::Number(f64::from_bits(1)));
    let _ = number_data.set_property("NaN", JsValue::Number(f64::NAN));
    let _ = number_data.set_property("POSITIVE_INFINITY", JsValue::Number(f64::INFINITY));
    let _ = number_data.set_property("NEGATIVE_INFINITY", JsValue::Number(f64::NEG_INFINITY));
    let _ = number_data.set_property("EPSILON", JsValue::Number(f64::EPSILON));
    let _ = number_data.set_property("MAX_SAFE_INTEGER", JsValue::Number(9007199254740991.0));
    let _ = number_data.set_property("MIN_SAFE_INTEGER", JsValue::Number(-9007199254740991.0));
    let _ = number_data.set_property("isNaN", JsValue::Native(is_nan_fn.clone()));
    let _ = number_data.set_property("isFinite", JsValue::Native(is_finite_fn.clone()));
    let _ = number_data.set_property("parseInt", JsValue::Native(parse_int_fn.clone()));
    let _ = number_data.set_property("parseFloat", JsValue::Native(parse_float_fn.clone()));
    env.define(
      "Number".to_string(),
      JsValue::Function(Arc::new(number_data)),
    );

    // Boolean 构造函数
    let bool_ctor = NativeFn::Ptr(|args: &[JsValue]| -> Result<JsValue> {
      let val = args.first().map(|v| v.to_boolean()).unwrap_or(false);
      Ok(JsValue::Boolean(val))
    });
    let bool_data =
      FunctionData::new_native(Some("Boolean".to_string()), Arc::clone(env), bool_ctor);
    env.define(
      "Boolean".to_string(),
      JsValue::Function(Arc::new(bool_data)),
    );

    // Date 构造函数与静态方法
    let now_fn = NativeFn::Ptr(|_args: &[JsValue]| -> Result<JsValue> {
      use std::time::{SystemTime, UNIX_EPOCH};
      let ms = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_millis() as f64)
        .unwrap_or(0.0);
      Ok(JsValue::Number(ms))
    });
    let date_ctor = NativeFn::Ptr(|_args: &[JsValue]| -> Result<JsValue> {
      use std::time::{SystemTime, UNIX_EPOCH};
      let ms = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_millis() as f64)
        .unwrap_or(0.0);
      let obj = JsValue::new_object();
      let _ = obj.set_property(TIMESTAMP_KEY, JsValue::Number(ms));
      let _ = obj.set_property(TIME_KEY, JsValue::Number(ms));
      Ok(obj)
    });
    let date_data = FunctionData::new_native(Some("Date".to_string()), Arc::clone(env), date_ctor);
    let _ = date_data.set_property("now", JsValue::Native(now_fn));
    env.define("Date".to_string(), JsValue::Function(Arc::new(date_data)));

    // Object 构造函数与静态方法
    let object_ctor = NativeFn::Ptr(|args: &[JsValue]| -> Result<JsValue> {
      if let Some(first) = args.first() {
        match first {
          JsValue::Undefined | JsValue::Null => Ok(JsValue::new_object()),
          JsValue::Object(_) | JsValue::Array(_) | JsValue::Function(_) | JsValue::RegExp(_) => {
            Ok(first.clone())
          }
          JsValue::Number(n) => {
            let obj = JsValue::new_object();
            let n_val = *n;
            let _ = obj.set_property(
              VALUE_OF_KEY,
              JsValue::Native(NativeFn::boxed_value_of(JsValue::Number(n_val))),
            );
            let _ = obj.set_property(
              TO_STRING_KEY,
              JsValue::Native(NativeFn::boxed_to_string(JsValue::Number(n_val))),
            );
            Ok(obj)
          }
          JsValue::Boolean(b) => {
            let obj = JsValue::new_object();
            let b_val = *b;
            let _ = obj.set_property(
              VALUE_OF_KEY,
              JsValue::Native(NativeFn::boxed_value_of(JsValue::Boolean(b_val))),
            );
            let _ = obj.set_property(
              TO_STRING_KEY,
              JsValue::Native(NativeFn::boxed_to_string(JsValue::Boolean(b_val))),
            );
            Ok(obj)
          }
          JsValue::String(s) => {
            let obj = JsValue::new_object();
            let _ = obj.set_property(
              VALUE_OF_KEY,
              JsValue::Native(NativeFn::boxed_value_of(JsValue::String(s.clone()))),
            );
            let _ = obj.set_property(
              TO_STRING_KEY,
              JsValue::Native(NativeFn::boxed_to_string(JsValue::String(s.clone()))),
            );
            Ok(obj)
          }
          JsValue::Symbol(s) => {
            let obj = JsValue::new_object();
            let s_desc = format!("Symbol({s})");
            let _ = obj.set_property(
              VALUE_OF_KEY,
              JsValue::Native(NativeFn::boxed_value_of(JsValue::Symbol(s.clone()))),
            );
            let _ = obj.set_property(
              TO_STRING_KEY,
              JsValue::Native(NativeFn::boxed_to_string(JsValue::String(s_desc))),
            );
            Ok(obj)
          }
          JsValue::BigInt(n) => {
            let obj = JsValue::new_object();
            let n_val = *n;
            let _ = obj.set_property(
              VALUE_OF_KEY,
              JsValue::Native(NativeFn::boxed_value_of(JsValue::BigInt(n_val))),
            );
            let _ = obj.set_property(
              TO_STRING_KEY,
              JsValue::Native(NativeFn::boxed_to_string(JsValue::BigInt(n_val))),
            );
            Ok(obj)
          }
          _ => Ok(JsValue::new_object()),
        }
      } else {
        Ok(JsValue::new_object())
      }
    });
    let object_data =
      FunctionData::new_native(Some("Object".to_string()), Arc::clone(env), object_ctor);
    let create_fn = NativeFn::Ptr(|args: &[JsValue]| -> Result<JsValue> {
      let obj = JsValue::new_object();
      if let Some(proto) = args.first()
        && matches!(proto, JsValue::Object(_) | JsValue::Null)
      {
        let _ = obj.set_property("__proto__", proto.clone());
      }
      Ok(obj)
    });
    let _ = object_data.set_property("create", JsValue::Native(create_fn));
    let keys_fn = NativeFn::Ptr(|args: &[JsValue]| -> Result<JsValue> {
      let mut keys = Vec::new();
      if let Some(JsValue::Object(pm)) = args.first() {
        let guard = pm.read();
        keys.reserve(guard.len());
        for k in guard.keys() {
          if k != "__proto__" {
            keys.push(JsValue::String(k.clone()));
          }
        }
      }
      Ok(JsValue::new_array(keys))
    });
    let define_property_fn = NativeFn::Ptr(|args: &[JsValue]| -> Result<JsValue> {
      if args.len() < 2 {
        return Ok(JsValue::Undefined);
      }
      let obj = args[0].clone();
      let prop_key = match &args[1] {
        JsValue::Symbol(s) => format!("@@symbol:{s}"),
        other => other.to_js_string(),
      };
      if let Some(desc) = args.get(2) {
        if let Ok(val) = desc.get_property("value")
          && val != JsValue::Undefined
        {
          let _ = obj.set_property(&prop_key, val);
        }
        if let Ok(get_fn) = desc.get_property("get")
          && matches!(get_fn, JsValue::Function(_) | JsValue::Native(_))
        {
          let _ = obj.set_property(&format!("@@get:{prop_key}"), get_fn);
        }
        if let Ok(set_fn) = desc.get_property("set")
          && matches!(set_fn, JsValue::Function(_) | JsValue::Native(_))
        {
          let _ = obj.set_property(&format!("@@set:{prop_key}"), set_fn);
        }
      }
      Ok(obj)
    });
    let _ = object_data.set_property("defineProperty", JsValue::Native(define_property_fn));
    let _ = object_data.set_property("keys", JsValue::Native(keys_fn));
    env.define(
      "Object".to_string(),
      JsValue::Function(Arc::new(object_data)),
    );

    // Symbol 构造函数与内置 Symbol
    let symbol_ctor = NativeFn::Ptr(|args: &[JsValue]| -> Result<JsValue> {
      let desc = args.first().map(|v| v.to_string()).unwrap_or_default();
      Ok(JsValue::Symbol(desc))
    });
    let symbol_data =
      FunctionData::new_native(Some("Symbol".to_string()), Arc::clone(env), symbol_ctor);
    let _ = symbol_data.set_property(
      "toPrimitive",
      JsValue::Symbol("Symbol.toPrimitive".to_string()),
    );
    env.define(
      "Symbol".to_string(),
      JsValue::Function(Arc::new(symbol_data)),
    );

    // BigInt 构造函数
    let bigint_ctor = NativeFn::Ptr(|args: &[JsValue]| -> Result<JsValue> {
      if let Some(first) = args.first() {
        match first {
          JsValue::BigInt(n) => Ok(JsValue::BigInt(*n)),
          JsValue::Number(n) => {
            if n.is_nan() || n.is_infinite() {
              return Err(Error::TypeError(
                "Cannot convert NaN or Infinity to BigInt".into(),
              ));
            }
            Ok(JsValue::BigInt(*n as i128))
          }
          JsValue::Boolean(b) => Ok(JsValue::BigInt(if *b { 1 } else { 0 })),
          JsValue::String(s) => {
            let trimmed = s.trim();
            if trimmed.is_empty() {
              return Ok(JsValue::BigInt(0));
            }
            let val = trimmed
              .parse::<i128>()
              .map_err(|_| Error::TypeError(format!("Cannot convert {s} to BigInt")))?;
            Ok(JsValue::BigInt(val))
          }
          _ => Err(Error::TypeError("Cannot convert value to BigInt".into())),
        }
      } else {
        Ok(JsValue::BigInt(0))
      }
    });
    let bigint_data =
      FunctionData::new_native(Some("BigInt".to_string()), Arc::clone(env), bigint_ctor);
    env.define(
      "BigInt".to_string(),
      JsValue::Function(Arc::new(bigint_data)),
    );

    // Array 构造函数与静态方法
    let array_ctor = NativeFn::Ptr(|args: &[JsValue]| -> Result<JsValue> {
      if args.len() == 1 && matches!(args[0], JsValue::Number(_)) {
        let len = args[0].to_int32().max(0) as usize;
        let mut items = Vec::with_capacity(len.min(100_000));
        items.resize(len, JsValue::Undefined);
        Ok(JsValue::new_array(items))
      } else {
        Ok(JsValue::new_array(args.to_vec()))
      }
    });
    let array_data =
      FunctionData::new_native(Some("Array".to_string()), Arc::clone(env), array_ctor);
    let is_array_fn = NativeFn::Ptr(|args: &[JsValue]| -> Result<JsValue> {
      let is_arr = args
        .first()
        .map(|v| matches!(v, JsValue::Array(_)))
        .unwrap_or(false);
      Ok(JsValue::Boolean(is_arr))
    });
    let _ = array_data.set_property("isArray", JsValue::Native(is_array_fn));
    if let Ok(arr_proto) = array_data.get_property(PROTOTYPE_KEY)
      && let Some(obj_ctor) = env.get("Object")
      && let Ok(obj_proto) = obj_ctor.get_property("prototype")
    {
      let _ = arr_proto.set_property(PROTO_KEY, obj_proto);
    }
    env.define("Array".to_string(), JsValue::Function(Arc::new(array_data)));

    // JSON 对象
    let json_obj = JsValue::new_object();
    fn serialize_to_json(val: &JsValue) -> Option<String> {
      match val {
        JsValue::Undefined | JsValue::Function(_) | JsValue::Native(_) | JsValue::Symbol(_) => None,
        JsValue::BigInt(n) => {
          let mut buf = Buffer::new();
          Some(buf.format(*n).to_string())
        }
        JsValue::RegExp(_) => Some("{}".to_string()),
        JsValue::Null => Some("null".to_string()),
        JsValue::Boolean(b) => Some(b.to_string()),
        JsValue::Number(n) => {
          if n.is_nan() || n.is_infinite() {
            Some("null".to_string())
          } else if n.fract() == 0.0 {
            Some((*n as i64).to_string())
          } else {
            Some(n.to_string())
          }
        }
        JsValue::String(s) => {
          let escaped = format!(
            "\"{}\"",
            s.replace('\\', "\\\\")
              .replace('"', "\\\"")
              .replace('\n', "\\n")
              .replace('\r', "\\r")
              .replace('\t', "\\t")
          );
          Some(escaped)
        }
        JsValue::Array(arr) => {
          let guard = arr.read();
          let items: Vec<String> = guard
            .iter()
            .map(|item| serialize_to_json(item).unwrap_or_else(|| "null".to_string()))
            .collect();
          Some(format!("[{}]", items.join(",")))
        }
        JsValue::Object(obj) => {
          let guard = obj.read();
          let mut keys: Vec<&String> = guard.keys().filter(|k| k.as_str() != "__proto__").collect();
          keys.sort();
          let mut pairs = Vec::with_capacity(keys.len());
          for k in keys {
            if let Some(v) = guard.get(k)
              && let Some(s) = serialize_to_json(v)
            {
              pairs.push(format!("\"{k}\":{s}"));
            }
          }
          Some(format!("{{{}}}", pairs.join(",")))
        }
      }
    }
    let stringify_fn = NativeFn::Ptr(|args: &[JsValue]| -> Result<JsValue> {
      if let Some(v) = args.first() {
        match serialize_to_json(v) {
          Some(s) => Ok(JsValue::String(s)),
          None => Ok(JsValue::Undefined),
        }
      } else {
        Ok(JsValue::Undefined)
      }
    });
    let _ = json_obj.set_property("stringify", JsValue::Native(stringify_fn));
    env.define("JSON".to_string(), json_obj);

    // 标准异常类构造器
    let object_proto = env
      .get("Object")
      .and_then(|o| o.get_property("prototype").ok())
      .unwrap_or_else(JsValue::new_object);

    // Function 构造函数与原型对象 (原型链挂接 Object.prototype)
    let func_proto = JsValue::new_object();
    let _ = func_proto.set_property(PROTO_KEY, object_proto.clone());
    let func_ctor =
      NativeFn::Ptr(|_args: &[JsValue]| -> Result<JsValue> { Ok(JsValue::new_object()) });
    let func_data =
      FunctionData::new_native(Some("Function".to_string()), Arc::clone(env), func_ctor);
    let _ = func_data.set_property(PROTOTYPE_KEY, func_proto);
    env.define(
      "Function".to_string(),
      JsValue::Function(Arc::new(func_data)),
    );

    // RegExp 构造函数与原型对象
    let regexp_ctor = NativeFn::Ptr(|args: &[JsValue]| -> Result<JsValue> {
      let pattern = args.first().map(|v| v.to_string()).unwrap_or_default();
      let flags = args.get(1).map(|v| v.to_string()).unwrap_or_default();
      JsValue::new_regexp(&pattern, &flags)
    });
    let regexp_data =
      FunctionData::new_native(Some("RegExp".to_string()), Arc::clone(env), regexp_ctor);
    if let Ok(reg_proto) = regexp_data.get_property(PROTOTYPE_KEY) {
      let _ = reg_proto.set_property(PROTO_KEY, object_proto.clone());
    }
    env.define(
      "RegExp".to_string(),
      JsValue::Function(Arc::new(regexp_data)),
    );

    let error_types = [
      "Error",
      "Test262Error",
      "TypeError",
      "RangeError",
      "ReferenceError",
      "SyntaxError",
      "URIError",
      "EvalError",
    ];
    for err_name in error_types {
      let proto = JsValue::new_object();
      let _ = proto.set_property(PROTO_KEY, object_proto.clone());
      let _ = proto.set_property(NAME_KEY, JsValue::String(err_name.to_string()));
      let _ = proto.set_property(MESSAGE_KEY, JsValue::String(String::new()));

      let err_ctor = NativeFn::error_ctor(err_name.to_string(), proto.clone());
      let err_data =
        FunctionData::new_native(Some(err_name.to_string()), Arc::clone(env), err_ctor);
      let func_val = JsValue::Function(Arc::new(err_data));
      let _ = proto.set_property(CONSTRUCTOR_KEY, func_val.clone());
      let _ = func_val.set_property(PROTOTYPE_KEY, proto);
      env.define(err_name.to_string(), func_val);
    }

    // 执行内置原型与反射支持预设脚本
    let prelude = r#"
      Object.prototype.toString = function () {
        if (this === undefined) return "[object Undefined]";
        if (this === null) return "[object Null]";
        if (typeof this === "boolean") return "[object Boolean]";
        if (typeof this === "number") return "[object Number]";
        if (typeof this === "string") return "[object String]";
        if (Array.isArray(this)) return "[object Array]";
        if (typeof this === "function") return "[object Function]";
        return "[object Object]";
      };
      Object.prototype.valueOf = function () { return this; };
      Object.prototype.hasOwnProperty = function (p) { return p in this; };
      Array.prototype.join = function (sep) {
        if (sep === undefined) sep = ",";
        var s = "";
        for (var i = 0; i < this.length; i++) {
          if (i > 0) s = s + sep;
          var v = this[i];
          if (v !== undefined && v !== null) s = s + v;
        }
        return s;
      };
      Array.prototype.map = function (fn) {
        var res = [];
        for (var i = 0; i < this.length; i++) {
          res.push(fn(this[i], i, this));
        }
        return res;
      };
      Array.prototype.indexOf = function (val, fromIndex) {
        var start = fromIndex || 0;
        for (var i = start; i < this.length; i++) {
          if (this[i] === val) return i;
        }
        return -1;
      };
      Array.prototype.slice = function (start, end) {
        var res = [];
        var len = this.length;
        var from = start < 0 ? Math.max(len + start, 0) : Math.min(start || 0, len);
        var to = end === undefined ? len : (end < 0 ? Math.max(len + end, 0) : Math.min(end, len));
        for (var i = from; i < to; i++) {
          res.push(this[i]);
        }
        return res;
      };
      Array.prototype.concat = function () {
        var res = [];
        for (var i = 0; i < this.length; i++) res.push(this[i]);
        for (var a = 0; a < arguments.length; a++) {
          var arg = arguments[a];
          if (Array.isArray(arg)) {
            for (var j = 0; j < arg.length; j++) res.push(arg[j]);
          } else {
            res.push(arg);
          }
        }
        return res;
      };
      Array.prototype.forEach = function (fn) {
        for (var i = 0; i < this.length; i++) {
          fn(this[i], i, this);
        }
      };
      Array.prototype.filter = function (fn) {
        var res = [];
        for (var i = 0; i < this.length; i++) {
          if (fn(this[i], i, this)) res.push(this[i]);
        }
        return res;
      };
      Array.prototype.reduce = function (fn, initial) {
        var idx = 0;
        var acc = initial;
        if (acc === undefined) {
          if (this.length === 0) throw new TypeError("Reduce of empty array with no initial value");
          acc = this[0];
          idx = 1;
        }
        for (var i = idx; i < this.length; i++) {
          acc = fn(acc, this[i], i, this);
        }
        return acc;
      };
      Array.prototype.every = function (fn) {
        for (var i = 0; i < this.length; i++) {
          if (!fn(this[i], i, this)) return false;
        }
        return true;
      };
      Array.prototype.some = function (fn) {
        for (var i = 0; i < this.length; i++) {
          if (fn(this[i], i, this)) return true;
        }
        return false;
      };
    "#;
    let mut lexer = Lexer::new(prelude);
    if let Ok(tokens) = lexer.tokenize_all() {
      let mut parser = Parser::new(tokens);
      if let Ok(stmts) = parser.parse_program() {
        Self::hoist_declarations(&stmts, env);
        let mut interp = Interpreter {
          global: Arc::clone(env),
          call_stack: Vec::new(),
          // 内建预置引导必须走 AST: 此时全局作用域尚未建完, 字节码桥会反向依赖它
          mode: ExecMode::AstOnly,
          stats: ExecStats::default(),
        };
        for s in &stmts {
          let _ = interp.eval_stmt(s, env);
        }
      }
    }
  }

  fn hoist_declarations(stmts: &[Stmt], env: &Arc<Environment>) {
    let mut hoisted_vars = Vec::new();
    let mut hoisted_funcs = Vec::new();
    collect_hoisted(stmts, &mut hoisted_vars, &mut hoisted_funcs);

    for v in hoisted_vars {
      if !env.has_own(&v) {
        env.define(v, JsValue::Undefined);
      }
    }
    if !hoisted_funcs.is_empty() {
      let obj_proto = env.get_object_prototype();
      for f in hoisted_funcs {
        let name = f.name.clone().unwrap_or_default();
        let func_val = FunctionData::from_shared_with_proto(f, Arc::clone(env), obj_proto.clone());
        env.define(name, func_val);
      }
    }
  }

  /// 执行代码片段: 先词法语法, 再按 mode 选执行路径
  pub fn run(&mut self, source: &str) -> Result<JsValue> {
    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize_all()?;
    let mut parser = Parser::new(tokens);
    let program = parser.parse_program()?;
    self.dispatch(&program)
  }

  /// 按 ExecMode 调度执行, 并把每条程序的实测走向计入 stats
  ///
  /// 回落只允许发生在 `None` (桥接层在执行前就判定不支持); 字节码一旦开跑,
  /// 其错误必须直接传播 —— 重放整段程序会把 print/赋值等副作用执行两遍。
  pub fn dispatch(&mut self, stmts: &[Stmt]) -> Result<JsValue> {
    self.stats.total += 1;
    if self.mode == ExecMode::AstOnly {
      return self.execute(stmts);
    }
    match try_execute_with_bytecode_and_env(stmts, Some(&self.global)) {
      Some(Ok(val)) => {
        self.stats.bytecode += 1;
        Ok(val)
      }
      Some(Err(err)) => Err(err),
      None if self.mode == ExecMode::BytecodeFirst => {
        self.stats.fallback += 1;
        self.execute(stmts)
      }
      None => Err(Error::Unsupported("该脚本无法由字节码引擎执行".to_string())),
    }
  }

  /// 输出主链路覆盖率读数
  pub fn print_coverage(&self) {
    let s = self.stats;
    println!(
      "bytecode-coverage: {:.1}% (total={} bytecode={} ast_fallback={})",
      s.coverage() * 100.0,
      s.total,
      s.bytecode,
      s.fallback
    );
  }

  pub fn execute(&mut self, stmts: &[Stmt]) -> Result<JsValue> {
    let env = Arc::clone(&self.global);
    Self::hoist_declarations(stmts, &env);

    let mut last = JsValue::Undefined;
    for stmt in stmts {
      match self.eval_stmt(stmt, &env) {
        Ok(v) => last = v,
        Err(ExecSignal::Return(v)) => return Ok(v),
        Err(ExecSignal::Break) => {
          return Err(Error::RuntimeError(
            "Illegal break outside of loop".to_string(),
          ));
        }
        Err(ExecSignal::Continue) => {
          return Err(Error::RuntimeError(
            "Illegal continue outside of loop".to_string(),
          ));
        }
        Err(ExecSignal::Error(err)) => return Err(err),
      }
    }
    Ok(last)
  }

  /// 创建符合规范的 JavaScript 异常对象并正确绑定原型链
  pub fn create_error(&self, error_name: &str, message: &str, env: &Arc<Environment>) -> JsValue {
    let mut props = HashMap::default();
    props.insert("name".to_string(), JsValue::String(error_name.to_string()));
    props.insert("message".to_string(), JsValue::String(message.to_string()));
    let proto_res = if let Some(ctor) = env.get(error_name) {
      ctor.get_property(PROTOTYPE_KEY)
    } else if let Some(err_ctor) = env.get("Error") {
      err_ctor.get_property(PROTOTYPE_KEY)
    } else {
      Err(Error::RuntimeError("Error constructor not found".into()))
    };
    if let Ok(proto) = proto_res {
      props.insert(PROTO_KEY.to_string(), proto);
    }
    JsValue::Object(Arc::new(RwLock::new(props)))
  }

  fn eval_get_property(&mut self, receiver: &JsValue, prop: &str) -> Result<JsValue> {
    if let JsValue::Object(o) = receiver {
      let guard = o.read();
      if prop == SYM_TO_PRIMITIVE {
        if let Some(getter) = guard.get(GET_SYM_TO_PRIMITIVE).cloned() {
          drop(guard);
          return self.invoke_function(&getter, Some(receiver.clone()), &[]);
        }
      } else {
        let has_getter = guard
          .keys()
          .any(|k| k.as_bytes().first() == Some(&b'@') && k.starts_with(GETTER_PREFIX));
        if has_getter {
          let getter_key = format_getter_key(prop);
          if let Some(getter) = guard.get(&getter_key).cloned() {
            drop(guard);
            return self.invoke_function(&getter, Some(receiver.clone()), &[]);
          }
        }
      }
    }
    receiver.get_property(prop)
  }

  fn eval_to_cow_str<'a>(
    &mut self,
    val: &'a JsValue,
    _env: &Arc<Environment>,
  ) -> Result<Cow<'a, str>> {
    match val {
      JsValue::String(s) => Ok(Cow::Borrowed(s.as_str())),
      JsValue::Undefined => Ok(Cow::Borrowed("undefined")),
      JsValue::Null => Ok(Cow::Borrowed("null")),
      JsValue::Boolean(true) => Ok(Cow::Borrowed("true")),
      JsValue::Boolean(false) => Ok(Cow::Borrowed("false")),
      JsValue::Symbol(_) => Err(Error::TypeError(
        "Cannot convert a Symbol value to a string".into(),
      )),
      JsValue::BigInt(n) => {
        let mut buf = Buffer::new();
        Ok(Cow::Owned(buf.format(*n).to_string()))
      }
      JsValue::Number(n) => {
        let num = *n;
        if num.is_nan() {
          Ok(Cow::Borrowed("NaN"))
        } else if num.is_infinite() {
          if num.is_sign_positive() {
            Ok(Cow::Borrowed("Infinity"))
          } else {
            Ok(Cow::Borrowed("-Infinity"))
          }
        } else if num == 0.0 {
          Ok(Cow::Borrowed("0"))
        } else if num.fract() == 0.0 && num.abs() < 1e15 {
          let mut buf = Buffer::new();
          Ok(Cow::Owned(buf.format(num as i64).to_string()))
        } else {
          Ok(Cow::Owned(num.to_string()))
        }
      }
      other => Ok(Cow::Owned(other.to_js_string())),
    }
  }

  fn eval_to_primitive(
    &mut self,
    val: &JsValue,
    hint: PreferredType,
    _env: &Arc<Environment>,
  ) -> Result<JsValue> {
    match val {
      JsValue::Undefined
      | JsValue::Null
      | JsValue::Boolean(_)
      | JsValue::Number(_)
      | JsValue::String(_)
      | JsValue::BigInt(_)
      | JsValue::Symbol(_) => Ok(val.clone()),

      JsValue::Object(_)
      | JsValue::Array(_)
      | JsValue::Function(_)
      | JsValue::Native(_)
      | JsValue::RegExp(_) => {
        // 1. 检查是否存在 [Symbol.toPrimitive]
        let exotic_to_prim = self.eval_get_property(val, SYM_TO_PRIMITIVE)?;
        if !matches!(exotic_to_prim, JsValue::Undefined | JsValue::Null) {
          if !matches!(exotic_to_prim, JsValue::Function(_) | JsValue::Native(_)) {
            return Err(Error::TypeError(
              "[Symbol.toPrimitive] is not a function".into(),
            ));
          }
          let res =
            self.invoke_function(&exotic_to_prim, Some(val.clone()), &[hint.to_js_value()])?;
          if matches!(
            res,
            JsValue::Object(_) | JsValue::Array(_) | JsValue::Function(_) | JsValue::RegExp(_)
          ) {
            return Err(Error::TypeError(
              "Cannot convert object to primitive value".into(),
            ));
          }
          return Ok(res);
        }

        // 2. OrdinaryToPrimitive
        let is_date = if let JsValue::Object(o) = val {
          o.read().contains_key(TIMESTAMP_KEY)
        } else {
          false
        };
        let method_names =
          if hint == PreferredType::String || (hint == PreferredType::Default && is_date) {
            [TO_STRING_KEY, VALUE_OF_KEY]
          } else {
            [VALUE_OF_KEY, TO_STRING_KEY]
          };

        for name in &method_names {
          let method = self.eval_get_property(val, name)?;
          if matches!(method, JsValue::Function(_) | JsValue::Native(_)) {
            let res = self.invoke_function(&method, Some(val.clone()), &[])?;
            if !matches!(
              res,
              JsValue::Object(_) | JsValue::Array(_) | JsValue::Function(_) | JsValue::RegExp(_)
            ) {
              return Ok(res);
            }
          }
        }
        Err(Error::TypeError(
          "Cannot convert object to primitive value".into(),
        ))
      }
    }
  }

  fn eval_add(
    &mut self,
    lprim: &JsValue,
    rprim: &JsValue,
    env: &Arc<Environment>,
  ) -> Result<JsValue> {
    if matches!(lprim, JsValue::String(_)) || matches!(rprim, JsValue::String(_)) {
      let ls = self.eval_to_cow_str(lprim, env)?;
      let rs = self.eval_to_cow_str(rprim, env)?;
      let mut out = String::with_capacity(ls.len() + rs.len());
      out.push_str(&ls);
      out.push_str(&rs);
      Ok(JsValue::String(out))
    } else {
      match (lprim, rprim) {
        (JsValue::Symbol(_), _) | (_, JsValue::Symbol(_)) => Err(Error::TypeError(
          "Cannot convert a Symbol value to a number".into(),
        )),
        (JsValue::BigInt(a), JsValue::BigInt(b)) => Ok(JsValue::BigInt(a.wrapping_add(*b))),
        (JsValue::BigInt(_), _) | (_, JsValue::BigInt(_)) => Err(Error::TypeError(
          "Cannot mix BigInt and other types, use explicit conversions".into(),
        )),
        _ => {
          let ln = lprim.to_number();
          let rn = rprim.to_number();
          Ok(JsValue::Number(ln + rn))
        }
      }
    }
  }

  fn apply_assign_op(
    &mut self,
    cur: &JsValue,
    op: &AssignOp,
    rval: &JsValue,
    env: &Arc<Environment>,
  ) -> Result<JsValue> {
    if let (JsValue::Number(nc), JsValue::Number(nr)) = (cur, rval) {
      match op {
        AssignOp::Assign => return Ok(rval.clone()),
        AssignOp::AddAssign => return Ok(JsValue::Number(nc + nr)),
        AssignOp::SubAssign => return Ok(JsValue::Number(nc - nr)),
        AssignOp::MulAssign => return Ok(JsValue::Number(nc * nr)),
        AssignOp::DivAssign => return Ok(JsValue::Number(nc / nr)),
        AssignOp::ModAssign => return Ok(JsValue::Number(nc % nr)),
        AssignOp::BitAndAssign => {
          return Ok(JsValue::Number(
            (double_to_int32(*nc) & double_to_int32(*nr)) as f64,
          ));
        }
        AssignOp::BitOrAssign => {
          return Ok(JsValue::Number(
            (double_to_int32(*nc) | double_to_int32(*nr)) as f64,
          ));
        }
        AssignOp::BitXorAssign => {
          return Ok(JsValue::Number(
            (double_to_int32(*nc) ^ double_to_int32(*nr)) as f64,
          ));
        }
        AssignOp::ShlAssign => {
          return Ok(JsValue::Number(
            (double_to_int32(*nc) << (double_to_uint32(*nr) & SHIFT_MASK)) as f64,
          ));
        }
        AssignOp::ShrAssign => {
          return Ok(JsValue::Number(
            (double_to_int32(*nc) >> (double_to_uint32(*nr) & SHIFT_MASK)) as f64,
          ));
        }
        AssignOp::SarAssign => {
          return Ok(JsValue::Number(
            (double_to_uint32(*nc) >> (double_to_uint32(*nr) & SHIFT_MASK)) as f64,
          ));
        }
      }
    }
    if let (JsValue::String(sl), JsValue::String(sr)) = (cur, rval)
      && *op == AssignOp::AddAssign
    {
      let mut out = String::with_capacity(sl.len() + sr.len());
      out.push_str(sl);
      out.push_str(sr);
      return Ok(JsValue::String(out));
    }
    if let (JsValue::BigInt(a), JsValue::BigInt(b)) = (cur, rval)
      && *op == AssignOp::AddAssign
    {
      return Ok(JsValue::BigInt(a.wrapping_add(*b)));
    }
    match op {
      AssignOp::Assign => Ok(rval.clone()),
      AssignOp::AddAssign => {
        let lp = self.eval_to_primitive(cur, PreferredType::Default, env)?;
        let rp = self.eval_to_primitive(rval, PreferredType::Default, env)?;
        self.eval_add(&lp, &rp, env)
      }
      AssignOp::SubAssign => Ok(JsValue::Number(cur.to_number() - rval.to_number())),
      AssignOp::MulAssign => Ok(JsValue::Number(cur.to_number() * rval.to_number())),
      AssignOp::DivAssign => Ok(JsValue::Number(cur.to_number() / rval.to_number())),
      AssignOp::ModAssign => Ok(JsValue::Number(cur.to_number() % rval.to_number())),
      AssignOp::BitAndAssign => Ok(JsValue::Number((cur.to_int32() & rval.to_int32()) as f64)),
      AssignOp::BitOrAssign => Ok(JsValue::Number((cur.to_int32() | rval.to_int32()) as f64)),
      AssignOp::BitXorAssign => Ok(JsValue::Number((cur.to_int32() ^ rval.to_int32()) as f64)),
      AssignOp::ShlAssign => Ok(JsValue::Number(
        (cur.to_int32() << (rval.to_uint32() & SHIFT_MASK)) as f64,
      )),
      AssignOp::ShrAssign => Ok(JsValue::Number(
        (cur.to_int32() >> (rval.to_uint32() & SHIFT_MASK)) as f64,
      )),
      AssignOp::SarAssign => Ok(JsValue::Number(
        (cur.to_uint32() >> (rval.to_uint32() & SHIFT_MASK)) as f64,
      )),
    }
  }

  fn eval_stmt(&mut self, stmt: &Stmt, env: &Arc<Environment>) -> StdResult<JsValue, ExecSignal> {
    match stmt {
      Stmt::Expr(expr) => self.eval_expr(expr, env).map_err(ExecSignal::from),

      Stmt::VarDecl { name, init } => {
        if let Some(e) = init {
          let val = self.eval_expr(e, env)?;
          if !env.set(name, val.clone()) {
            env.define(name.clone(), val);
          }
        }
        Ok(JsValue::Undefined)
      }

      Stmt::VarDecls(decls) => {
        for (name, init) in decls {
          if let Some(e) = init {
            let val = self.eval_expr(e, env)?;
            if !env.set(name, val.clone()) {
              env.define(name.clone(), val);
            }
          }
        }
        Ok(JsValue::Undefined)
      }

      Stmt::Block(stmts) => {
        let mut last = JsValue::Undefined;
        for s in stmts {
          last = self.eval_stmt(s, env)?;
        }
        Ok(last)
      }

      Stmt::If {
        cond,
        then_branch,
        else_branch,
      } => {
        let c = self.eval_expr(cond, env)?;
        if c.to_boolean() {
          self.eval_stmt(then_branch, env)
        } else if let Some(eb) = else_branch {
          self.eval_stmt(eb, env)
        } else {
          Ok(JsValue::Undefined)
        }
      }

      Stmt::While { cond, body } => {
        let mut last = JsValue::Undefined;
        loop {
          let c = self.eval_expr(cond, env)?;
          if !c.to_boolean() {
            break;
          }
          match self.eval_stmt(body, env) {
            Ok(v) => last = v,
            Err(ExecSignal::Break) => break,
            Err(ExecSignal::Continue) => continue,
            Err(e) => return Err(e),
          }
        }
        Ok(last)
      }

      Stmt::For {
        init,
        cond,
        step,
        body,
      } => {
        if let Some(i) = init {
          self.eval_stmt(i, env)?;
        }
        let mut last = JsValue::Undefined;
        loop {
          if let Some(c) = cond {
            let res = self.eval_expr(c, env)?;
            if !res.to_boolean() {
              break;
            }
          }
          match self.eval_stmt(body, env) {
            Ok(v) => last = v,
            Err(ExecSignal::Break) => break,
            Err(ExecSignal::Continue) => {}
            Err(e) => return Err(e),
          }
          if let Some(s) = step {
            self.eval_expr(s, env)?;
          }
        }
        Ok(last)
      }

      Stmt::FunctionDecl(_) => Ok(JsValue::Undefined),

      Stmt::Return(expr) => {
        let val = if let Some(e) = expr {
          self.eval_expr(e, env)?
        } else {
          JsValue::Undefined
        };
        Err(ExecSignal::Return(val))
      }

      Stmt::Break => Err(ExecSignal::Break),
      Stmt::Continue => Err(ExecSignal::Continue),

      Stmt::Try {
        block,
        catch_param,
        catch_block,
        finally_block,
      } => {
        let block_env = Arc::new(Environment::new_child(env));
        let mut try_result = Ok(JsValue::Undefined);
        for s in block {
          match self.eval_stmt(s, &block_env) {
            Ok(v) => try_result = Ok(v),
            Err(e) => {
              try_result = Err(e);
              break;
            }
          }
        }

        let mut res = match try_result {
          Ok(v) => Ok(v),
          Err(ExecSignal::Error(err)) => {
            if let Some(cb) = catch_block {
              let catch_env = Arc::new(Environment::new_child(env));
              if let Some(p) = catch_param {
                let err_val = match &err {
                  Error::Thrown(v) => v.clone(),
                  Error::ReferenceError(msg) => self.create_error("ReferenceError", msg, env),
                  Error::TypeError(msg) => self.create_error("TypeError", msg, env),
                  Error::RuntimeError(msg) => {
                    if msg.contains("is not defined") {
                      self.create_error("ReferenceError", msg, env)
                    } else if msg.contains("is not a function") || msg.contains("Cannot convert") {
                      self.create_error("TypeError", msg, env)
                    } else {
                      self.create_error("Error", msg, env)
                    }
                  }
                  _ => self.create_error("Error", &err.to_string(), env),
                };
                catch_env.define(p.clone(), err_val);
              }
              let mut catch_res = Ok(JsValue::Undefined);
              for s in cb {
                catch_res = self.eval_stmt(s, &catch_env);
                if catch_res.is_err() {
                  break;
                }
              }
              catch_res
            } else {
              Err(ExecSignal::Error(err))
            }
          }
          Err(other) => Err(other),
        };

        if let Some(fb) = finally_block {
          let fin_env = Arc::new(Environment::new_child(env));
          for s in fb {
            let fin_res = self.eval_stmt(s, &fin_env);
            if fin_res.is_err() {
              res = fin_res;
              break;
            }
          }
        }
        res
      }

      Stmt::Throw(expr) => {
        let v = self.eval_expr(expr, env)?;
        Err(ExecSignal::Error(Error::Thrown(v)))
      }

      Stmt::DoWhile { body, cond } => {
        let mut last = JsValue::Undefined;
        loop {
          match self.eval_stmt(body, env) {
            Ok(v) => last = v,
            Err(ExecSignal::Break) => break,
            Err(ExecSignal::Continue) => {}
            Err(e) => return Err(e),
          }
          if !self.eval_expr(cond, env)?.to_boolean() {
            break;
          }
        }
        Ok(last)
      }

      Stmt::ForIn {
        variable,
        is_var: _,
        object,
        body,
      } => {
        let obj_val = self.eval_expr(object, env)?;
        let keys = match obj_val {
          JsValue::Object(o) => {
            let guard = o.read();
            let mut keys = Vec::with_capacity(guard.len());
            for k in guard.keys() {
              if k != PROTO_KEY {
                keys.push(k.clone());
              }
            }
            keys
          }
          JsValue::Array(a) => {
            let len = a.read().len();
            let mut keys = Vec::with_capacity(len);
            let mut buf = Buffer::new();
            for i in 0..len {
              keys.push(buf.format(i).to_string());
            }
            keys
          }
          _ => Vec::new(),
        };
        let mut last = JsValue::Undefined;
        for k in keys {
          if !env.set(variable, JsValue::String(k.clone())) {
            env.define(variable.clone(), JsValue::String(k));
          }
          match self.eval_stmt(body, env) {
            Ok(v) => last = v,
            Err(ExecSignal::Break) => break,
            Err(ExecSignal::Continue) => continue,
            Err(e) => return Err(e),
          }
        }
        Ok(last)
      }

      Stmt::Switch {
        discriminant,
        cases,
      } => {
        let disc_val = self.eval_expr(discriminant, env)?;
        let mut matched = false;
        let mut last = JsValue::Undefined;
        for c in cases {
          if !matched {
            if let Some(test) = &c.test {
              let test_val = self.eval_expr(test, env)?;
              if test_val == disc_val {
                matched = true;
              }
            } else {
              matched = true;
            }
          }
          if matched {
            for s in &c.body {
              match self.eval_stmt(s, env) {
                Ok(v) => last = v,
                Err(ExecSignal::Break) => return Ok(last),
                Err(ExecSignal::Continue) => return Err(ExecSignal::Continue),
                Err(e) => return Err(e),
              }
            }
          }
        }
        Ok(last)
      }

      Stmt::Empty => Ok(JsValue::Undefined),
    }
  }

  fn eval_expr(&mut self, expr: &Expr, env: &Arc<Environment>) -> Result<JsValue> {
    match expr {
      Expr::Literal(val) => Ok(val.clone()),

      Expr::Identifier(name) => env
        .get(name)
        .ok_or_else(|| Error::ReferenceError(format!("{name} is not defined"))),

      Expr::This => {
        if let Some(this_val) = env.get("this") {
          Ok(this_val)
        } else {
          Ok(JsValue::Undefined)
        }
      }

      Expr::Typeof(expr) => {
        let val = match &**expr {
          Expr::Identifier(name) => env.get(name).unwrap_or(JsValue::Undefined),
          _ => self.eval_expr(expr, env)?,
        };
        let type_str = match val {
          JsValue::Undefined => "undefined",
          JsValue::Null => "object",
          JsValue::Boolean(_) => "boolean",
          JsValue::Number(_) => "number",
          JsValue::BigInt(_) => "bigint",
          JsValue::String(_) => "string",
          JsValue::Symbol(_) => "symbol",
          JsValue::Function(_) | JsValue::Native(_) => "function",
          JsValue::Object(_) | JsValue::Array(_) | JsValue::RegExp(_) => "object",
        };
        Ok(JsValue::String(type_str.to_string()))
      }

      Expr::Unary { op, expr } => match op {
        UnaryOp::PreInc | UnaryOp::PreDec | UnaryOp::PostInc | UnaryOp::PostDec => {
          let is_inc = matches!(op, UnaryOp::PreInc | UnaryOp::PostInc);
          let is_pre = matches!(op, UnaryOp::PreInc | UnaryOp::PreDec);
          match &**expr {
            Expr::Identifier(id) => {
              let old_val = env.get(id).unwrap_or(JsValue::Undefined);
              let old_num = old_val.to_number();
              let next = JsValue::Number(if is_inc { old_num + 1.0 } else { old_num - 1.0 });
              if !env.set(id, next.clone()) {
                env.define(id.clone(), next.clone());
              }
              if is_pre { Ok(next) } else { Ok(old_val) }
            }
            Expr::Member { object, property } => {
              let obj = self.eval_expr(object, env)?;
              let old_val = obj.get_property(property)?;
              let old_num = old_val.to_number();
              let next = JsValue::Number(if is_inc { old_num + 1.0 } else { old_num - 1.0 });
              obj.set_property(property, next.clone())?;
              if is_pre { Ok(next) } else { Ok(old_val) }
            }
            Expr::Index { object, index } => {
              let obj = self.eval_expr(object, env)?;
              let idx_val = self.eval_expr(index, env)?;
              let old_val = obj.get_element(&idx_val)?;
              let old_num = old_val.to_number();
              let next = JsValue::Number(if is_inc { old_num + 1.0 } else { old_num - 1.0 });
              obj.set_element(&idx_val, next.clone())?;
              if is_pre { Ok(next) } else { Ok(old_val) }
            }
            _ => Err(Error::RuntimeError(format!("Invalid lvalue for {op:?}"))),
          }
        }
        UnaryOp::Neg => {
          let val = self.eval_expr(expr, env)?;
          match val {
            JsValue::BigInt(n) => Ok(JsValue::BigInt(-n)),
            _ => Ok(JsValue::Number(-val.to_number())),
          }
        }
        UnaryOp::Not => {
          let val = self.eval_expr(expr, env)?;
          Ok(JsValue::Boolean(!val.to_boolean()))
        }
        UnaryOp::BitNot => {
          let val = self.eval_expr(expr, env)?;
          Ok(JsValue::Number((!val.to_int32()) as f64))
        }
        UnaryOp::Pos => {
          let val = self.eval_expr(expr, env)?;
          match val {
            JsValue::BigInt(_) => Err(Error::TypeError(
              "Cannot convert a BigInt value to a number".into(),
            )),
            _ => Ok(JsValue::Number(val.to_number())),
          }
        }
        UnaryOp::Void => {
          self.eval_expr(expr, env)?;
          Ok(JsValue::Undefined)
        }
        UnaryOp::Delete => match &**expr {
          Expr::Member { object, property } => {
            let obj = self.eval_expr(object, env)?;
            if let JsValue::Object(o) = obj {
              o.write().remove(property);
            }
            Ok(JsValue::Boolean(true))
          }
          Expr::Index { object, index } => {
            let obj = self.eval_expr(object, env)?;
            let idx_val = self.eval_expr(index, env)?;
            if let JsValue::Object(o) = obj {
              match idx_val {
                JsValue::Number(n) => {
                  let mut buf = Buffer::new();
                  let key = buf.format(n as i64);
                  o.write().remove(key);
                }
                _ => {
                  let prop = idx_val.to_string();
                  o.write().remove(&prop);
                }
              }
            }
            Ok(JsValue::Boolean(true))
          }
          _ => Ok(JsValue::Boolean(true)),
        },
      },

      Expr::Binary { op, left, right } => {
        // 短路求值
        if *op == BinaryOp::LogicalAnd {
          let l = self.eval_expr(left, env)?;
          if !l.to_boolean() {
            return Ok(l);
          }
          return self.eval_expr(right, env);
        }
        if *op == BinaryOp::LogicalOr {
          let l = self.eval_expr(left, env)?;
          if l.to_boolean() {
            return Ok(l);
          }
          return self.eval_expr(right, env);
        }

        let l = self.eval_expr(left, env)?;
        let r = self.eval_expr(right, env)?;

        if let (JsValue::Number(nl), JsValue::Number(nr)) = (&l, &r) {
          match op {
            BinaryOp::Add => return Ok(JsValue::Number(nl + nr)),
            BinaryOp::Sub => return Ok(JsValue::Number(nl - nr)),
            BinaryOp::Mul => return Ok(JsValue::Number(nl * nr)),
            BinaryOp::Div => return Ok(JsValue::Number(nl / nr)),
            BinaryOp::Mod => return Ok(JsValue::Number(nl % nr)),
            BinaryOp::BitAnd => {
              return Ok(JsValue::Number(
                (double_to_int32(*nl) & double_to_int32(*nr)) as f64,
              ));
            }
            BinaryOp::BitOr => {
              return Ok(JsValue::Number(
                (double_to_int32(*nl) | double_to_int32(*nr)) as f64,
              ));
            }
            BinaryOp::BitXor => {
              return Ok(JsValue::Number(
                (double_to_int32(*nl) ^ double_to_int32(*nr)) as f64,
              ));
            }
            BinaryOp::Shl => {
              return Ok(JsValue::Number(
                (double_to_int32(*nl) << (double_to_uint32(*nr) & SHIFT_MASK)) as f64,
              ));
            }
            BinaryOp::Shr => {
              return Ok(JsValue::Number(
                (double_to_int32(*nl) >> (double_to_uint32(*nr) & SHIFT_MASK)) as f64,
              ));
            }
            BinaryOp::Sar => {
              return Ok(JsValue::Number(
                (double_to_uint32(*nl) >> (double_to_uint32(*nr) & SHIFT_MASK)) as f64,
              ));
            }
            BinaryOp::Lt => return Ok(JsValue::Boolean(nl < nr)),
            BinaryOp::Lte => return Ok(JsValue::Boolean(nl <= nr)),
            BinaryOp::Gt => return Ok(JsValue::Boolean(nl > nr)),
            BinaryOp::Gte => return Ok(JsValue::Boolean(nl >= nr)),
            BinaryOp::StrictEq | BinaryOp::Eq => return Ok(JsValue::Boolean(nl == nr)),
            BinaryOp::StrictNotEq | BinaryOp::NotEq => return Ok(JsValue::Boolean(nl != nr)),
            _ => {}
          }
        }

        match op {
          BinaryOp::Add => match (&l, &r) {
            (JsValue::Number(nl), JsValue::Number(nr)) => Ok(JsValue::Number(nl + nr)),
            (JsValue::String(sl), JsValue::String(sr)) => {
              let mut out = String::with_capacity(sl.len() + sr.len());
              out.push_str(sl);
              out.push_str(sr);
              Ok(JsValue::String(out))
            }
            (JsValue::BigInt(a), JsValue::BigInt(b)) => Ok(JsValue::BigInt(a.wrapping_add(*b))),
            _ => {
              let lp = self.eval_to_primitive(&l, PreferredType::Default, env)?;
              let rp = self.eval_to_primitive(&r, PreferredType::Default, env)?;
              self.eval_add(&lp, &rp, env)
            }
          },
          BinaryOp::Sub => Ok(JsValue::Number(l.to_number() - r.to_number())),
          BinaryOp::Mul => Ok(JsValue::Number(l.to_number() * r.to_number())),
          BinaryOp::Div => Ok(JsValue::Number(l.to_number() / r.to_number())),
          BinaryOp::Mod => Ok(JsValue::Number(l.to_number() % r.to_number())),

          BinaryOp::BitAnd => Ok(JsValue::Number((l.to_int32() & r.to_int32()) as f64)),
          BinaryOp::BitOr => Ok(JsValue::Number((l.to_int32() | r.to_int32()) as f64)),
          BinaryOp::BitXor => Ok(JsValue::Number((l.to_int32() ^ r.to_int32()) as f64)),
          BinaryOp::Shl => Ok(JsValue::Number(
            (l.to_int32() << (r.to_uint32() & SHIFT_MASK)) as f64,
          )),
          BinaryOp::Shr => Ok(JsValue::Number(
            (l.to_int32() >> (r.to_uint32() & SHIFT_MASK)) as f64,
          )),
          BinaryOp::Sar => Ok(JsValue::Number(
            (l.to_uint32() >> (r.to_uint32() & SHIFT_MASK)) as f64,
          )),

          BinaryOp::Eq => Ok(JsValue::Boolean(l.abstract_eq(&r))),
          BinaryOp::NotEq => Ok(JsValue::Boolean(!l.abstract_eq(&r))),
          BinaryOp::StrictEq => Ok(JsValue::Boolean(l == r)),
          BinaryOp::StrictNotEq => Ok(JsValue::Boolean(l != r)),

          BinaryOp::Lt => {
            let lp = l.to_primitive(PreferredType::Number);
            let rp = r.to_primitive(PreferredType::Number);
            match (&lp, &rp) {
              (JsValue::String(s1), JsValue::String(s2)) => Ok(JsValue::Boolean(s1 < s2)),
              _ => Ok(JsValue::Boolean(lp.to_number() < rp.to_number())),
            }
          }
          BinaryOp::Lte => {
            let lp = l.to_primitive(PreferredType::Number);
            let rp = r.to_primitive(PreferredType::Number);
            match (&lp, &rp) {
              (JsValue::String(s1), JsValue::String(s2)) => Ok(JsValue::Boolean(s1 <= s2)),
              _ => Ok(JsValue::Boolean(lp.to_number() <= rp.to_number())),
            }
          }
          BinaryOp::Gt => {
            let lp = l.to_primitive(PreferredType::Number);
            let rp = r.to_primitive(PreferredType::Number);
            match (&lp, &rp) {
              (JsValue::String(s1), JsValue::String(s2)) => Ok(JsValue::Boolean(s1 > s2)),
              _ => Ok(JsValue::Boolean(lp.to_number() > rp.to_number())),
            }
          }
          BinaryOp::Gte => {
            let lp = l.to_primitive(PreferredType::Number);
            let rp = r.to_primitive(PreferredType::Number);
            match (&lp, &rp) {
              (JsValue::String(s1), JsValue::String(s2)) => Ok(JsValue::Boolean(s1 >= s2)),
              _ => Ok(JsValue::Boolean(lp.to_number() >= rp.to_number())),
            }
          }

          BinaryOp::In => match &r {
            JsValue::Object(_) | JsValue::Function(_) => {
              let key = l.to_string();
              let mut curr = Some(r.clone());
              while let Some(val) = curr {
                match val {
                  JsValue::Object(obj) => {
                    let guard = obj.read();
                    if guard.contains_key(&key) {
                      return Ok(JsValue::Boolean(true));
                    }
                    curr = guard.get(PROTO_KEY).cloned();
                  }
                  JsValue::Function(func) => {
                    let guard = func.properties.read();
                    if guard.contains_key(&key)
                      || key == "length"
                      || key == "name"
                      || key == PROTOTYPE_KEY
                    {
                      return Ok(JsValue::Boolean(true));
                    }
                    curr = None;
                  }
                  _ => break,
                }
              }
              Ok(JsValue::Boolean(false))
            }
            JsValue::Array(arr) => {
              let key = l.to_string();
              if key == "length" {
                return Ok(JsValue::Boolean(true));
              }
              if let Ok(idx) = key.parse::<usize>() {
                let len = arr.read().len();
                Ok(JsValue::Boolean(idx < len))
              } else {
                Ok(JsValue::Boolean(false))
              }
            }
            _ => Err(Error::TypeError(format!(
              "Cannot use 'in' operator to search for '{l}' in {r}"
            ))),
          },
          BinaryOp::InstanceOf => {
            let target_proto = match r.get_property(PROTOTYPE_KEY) {
              Ok(p) => p,
              Err(_) => return Ok(JsValue::Boolean(false)),
            };
            if !matches!(target_proto, JsValue::Object(_) | JsValue::Function(_)) {
              return Ok(JsValue::Boolean(false));
            }
            let obj_proto = env
              .get("Object")
              .and_then(|o| o.get_property("prototype").ok());
            let mut curr = match &l {
              JsValue::Object(obj) => {
                let guard = obj.read();
                guard
                  .get("__proto__")
                  .cloned()
                  .or_else(|| obj_proto.clone())
              }
              JsValue::Array(_) => env
                .get("Array")
                .and_then(|a| a.get_property("prototype").ok())
                .or_else(|| obj_proto.clone()),
              JsValue::Function(func) => {
                let guard = func.properties.read();
                guard
                  .get("__proto__")
                  .cloned()
                  .or_else(|| {
                    env
                      .get("Function")
                      .and_then(|f| f.get_property("prototype").ok())
                  })
                  .or_else(|| obj_proto.clone())
              }
              JsValue::RegExp(_) => env
                .get("RegExp")
                .and_then(|re| re.get_property("prototype").ok())
                .or_else(|| obj_proto.clone()),
              _ => None,
            };
            let mut depth = 0;
            while let Some(proto) = curr {
              depth += 1;
              if depth > 1024 {
                break;
              }
              if proto == target_proto {
                return Ok(JsValue::Boolean(true));
              }
              curr = match &proto {
                JsValue::Object(obj) => {
                  let guard = obj.read();
                  if let Some(next) = guard.get("__proto__").cloned() {
                    Some(next)
                  } else if Some(&proto) != obj_proto.as_ref() {
                    obj_proto.clone()
                  } else {
                    None
                  }
                }
                _ => None,
              };
            }
            Ok(JsValue::Boolean(false))
          }

          BinaryOp::Comma => Ok(r),
          BinaryOp::LogicalAnd | BinaryOp::LogicalOr => unreachable!(),
        }
      }

      Expr::Assign { target, op, value } => {
        let rval = self.eval_expr(value, env)?;

        match &**target {
          Expr::Identifier(name) => {
            let final_val = if *op == AssignOp::Assign {
              rval
            } else {
              let cur = env.get(name).unwrap_or(JsValue::Undefined);
              self.apply_assign_op(&cur, op, &rval, env)?
            };
            if !env.set(name, final_val.clone()) {
              env.define(name.clone(), final_val.clone());
            }
            Ok(final_val)
          }
          Expr::Member { object, property } => {
            let obj = self.eval_expr(object, env)?;
            let final_val = if *op == AssignOp::Assign {
              rval
            } else {
              let cur = obj.get_property(property)?;
              self.apply_assign_op(&cur, op, &rval, env)?
            };
            obj.set_property(property, final_val.clone())?;
            Ok(final_val)
          }
          Expr::Index { object, index } => {
            let obj = self.eval_expr(object, env)?;
            let idx_val = self.eval_expr(index, env)?;
            let final_val = if *op == AssignOp::Assign {
              rval
            } else {
              let cur = obj.get_element(&idx_val)?;
              self.apply_assign_op(&cur, op, &rval, env)?
            };
            obj.set_element(&idx_val, final_val.clone())?;
            Ok(final_val)
          }
          _ => Err(Error::RuntimeError("Invalid assignment target".to_string())),
        }
      }

      Expr::Member { object, property } => {
        let obj = self.eval_expr(object, env)?;
        if let JsValue::Array(ref arr) = obj
          && property == "length"
        {
          return Ok(JsValue::Number(arr.read().len() as f64));
        }
        if let JsValue::String(ref s) = obj
          && property == "length"
        {
          return Ok(JsValue::Number(char_count(s) as f64));
        }
        obj.get_property(property).map_err(|e| {
          Error::RuntimeError(format!(
            "{e} at '{property}' (stack: {:?})",
            self.call_stack
          ))
        })
      }

      Expr::Index { object, index } => {
        let obj = self.eval_expr(object, env)?;
        let idx = self.eval_expr(index, env)?;
        obj.get_element(&idx).map_err(|e| {
          Error::RuntimeError(format!("{e} at '[{idx:?}]' (stack: {:?})", self.call_stack))
        })
      }

      Expr::Call { callee, args } => {
        // 识别 Function.prototype.call 与 apply 反射调用
        if let Expr::Member { object, property } = &**callee {
          if property == "call" {
            let func_val = self.eval_expr(object, env)?;
            match args.len() {
              0 => return self.invoke_function(&func_val, Some(JsValue::Undefined), &[]),
              1 => {
                let this_val = self.eval_expr(&args[0], env)?;
                return self.invoke_function(&func_val, Some(this_val), &[]);
              }
              2 => {
                let this_val = self.eval_expr(&args[0], env)?;
                let a0 = self.eval_expr(&args[1], env)?;
                return self.invoke_function(&func_val, Some(this_val), &[a0]);
              }
              3 => {
                let this_val = self.eval_expr(&args[0], env)?;
                let a0 = self.eval_expr(&args[1], env)?;
                let a1 = self.eval_expr(&args[2], env)?;
                return self.invoke_function(&func_val, Some(this_val), &[a0, a1]);
              }
              _ => {
                let mut eval_args = Vec::with_capacity(args.len());
                for a in args {
                  eval_args.push(self.eval_expr(a, env)?);
                }
                let this_val = eval_args[0].clone();
                return self.invoke_function(&func_val, Some(this_val), &eval_args[1..]);
              }
            }
          }
          if property == "apply" {
            let func_val = self.eval_expr(object, env)?;
            let this_val = if let Some(first) = args.first() {
              self.eval_expr(first, env)?
            } else {
              JsValue::Undefined
            };
            let mut real_args = Vec::new();
            if let Some(second) = args.get(1) {
              let arg_val = self.eval_expr(second, env)?;
              if let JsValue::Array(arr) = arg_val {
                real_args = arr.read().clone();
              }
            }
            return self.invoke_function(&func_val, Some(this_val), &real_args);
          }
        }

        let (func, this_opt, callee_name) = match &**callee {
          Expr::Member { object, property } => {
            let obj = self.eval_expr(object, env)?;
            // 对标 V8 Builtin Inlining (builtins-array.cc / js-builtin-reducer.cc):
            // 对接收者为 Array 的高频方法就地快速内联，彻底消除生成临时闭包对象的堆分配
            if let JsValue::Array(ref arr) = obj
              && let Some(res) = self.try_inline_array_call(arr, property, args, env)
            {
              return res;
            }
            let f = obj.get_property(property).map_err(|e| {
              Error::RuntimeError(format!(
                "{e} at '{property}' (stack: {:?})",
                self.call_stack
              ))
            })?;
            (f, Some(obj), property.clone())
          }
          Expr::Index { object, index } => {
            let obj = self.eval_expr(object, env)?;
            let idx = self.eval_expr(index, env)?;
            if let JsValue::Array(ref arr) = obj
              && let JsValue::String(ref prop) = idx
              && let Some(res) = self.try_inline_array_call(arr, prop, args, env)
            {
              return res;
            }
            let f = obj.get_element(&idx).map_err(|e| {
              Error::RuntimeError(format!("{e} at '[{idx:?}]' (stack: {:?})", self.call_stack))
            })?;
            (f, Some(obj), idx.to_string())
          }
          Expr::Identifier(name) => {
            let f = self.eval_expr(callee, env)?;
            (f, None, name.clone())
          }
          _ => {
            let f = self.eval_expr(callee, env)?;
            (f, None, "<anonymous>".to_string())
          }
        };

        if !matches!(func, JsValue::Native(_) | JsValue::Function(_)) {
          return Err(Error::TypeError(format!(
            "'{callee_name}': {func:?} is not a function (stack: {:?})",
            self.call_stack
          )));
        }

        match args.len() {
          0 => self.invoke_function_named(&func, this_opt, &[], &callee_name),
          1 => {
            let a0 = self.eval_expr(&args[0], env)?;
            self.invoke_function_named(&func, this_opt, &[a0], &callee_name)
          }
          2 => {
            let a0 = self.eval_expr(&args[0], env)?;
            let a1 = self.eval_expr(&args[1], env)?;
            self.invoke_function_named(&func, this_opt, &[a0, a1], &callee_name)
          }
          3 => {
            let a0 = self.eval_expr(&args[0], env)?;
            let a1 = self.eval_expr(&args[1], env)?;
            let a2 = self.eval_expr(&args[2], env)?;
            self.invoke_function_named(&func, this_opt, &[a0, a1, a2], &callee_name)
          }
          _ => {
            let evaluated_args = self.eval_args(args, env)?;
            self.invoke_function_named(&func, this_opt, &evaluated_args, &callee_name)
          }
        }
      }

      Expr::Array(items) => match items.len() {
        0 => Ok(JsValue::new_array(Vec::new())),
        1 => {
          let v0 = self.eval_expr(&items[0], env)?;
          Ok(JsValue::new_array(vec![v0]))
        }
        2 => {
          let v0 = self.eval_expr(&items[0], env)?;
          let v1 = self.eval_expr(&items[1], env)?;
          Ok(JsValue::new_array(vec![v0, v1]))
        }
        _ => {
          let evaluated = self.eval_args(items, env)?;
          Ok(JsValue::new_array(evaluated))
        }
      },

      Expr::Object(entries) => {
        let mut map = HashMap::default();
        map.reserve(entries.len() + 1);
        if let Some(obj_proto) = env.get_object_prototype() {
          map.insert(PROTO_KEY.to_string(), obj_proto);
        }
        for (k, v) in entries {
          let key_str = match k {
            ObjectKey::Ident(s) => s.clone(),
            ObjectKey::Computed(expr) => {
              let k_val = self.eval_expr(expr, env)?;
              match k_val {
                JsValue::Symbol(s) => format_symbol_key(&s),
                other => other.to_js_string(),
              }
            }
          };
          let val = self.eval_expr(v, env)?;
          map.insert(key_str, val);
        }
        Ok(JsValue::Object(Arc::new(RwLock::new(map))))
      }

      Expr::FunctionExpr(shared) => Ok(FunctionData::from_shared(
        Arc::clone(shared),
        Arc::clone(env),
      )),

      Expr::Conditional {
        condition,
        consequent,
        alternate,
      } => {
        let cond = self.eval_expr(condition, env)?;
        if cond.to_boolean() {
          self.eval_expr(consequent, env)
        } else {
          self.eval_expr(alternate, env)
        }
      }

      Expr::New { callee, args } => {
        let callee_val = self.eval_expr(callee, env)?;
        match args.len() {
          0 => self.invoke_constructor(&callee_val, &[]),
          1 => {
            let a0 = self.eval_expr(&args[0], env)?;
            self.invoke_constructor(&callee_val, &[a0])
          }
          2 => {
            let a0 = self.eval_expr(&args[0], env)?;
            let a1 = self.eval_expr(&args[1], env)?;
            self.invoke_constructor(&callee_val, &[a0, a1])
          }
          _ => {
            let evaluated_args = self.eval_args(args, env)?;
            self.invoke_constructor(&callee_val, &evaluated_args)
          }
        }
      }
    }
  }

  fn invoke_constructor(
    &mut self,
    callee_val: &JsValue,
    evaluated_args: &[JsValue],
  ) -> Result<JsValue> {
    match callee_val {
      JsValue::Native(nat) => nat.call(evaluated_args),
      JsValue::Function(func_data) => {
        let fname = format!("new {}", func_data.name().unwrap_or("anonymous"));
        self.call_stack.push(fname);
        let res = self.run_constructor(func_data, callee_val, evaluated_args);
        self.call_stack.pop();
        res
      }
      other => Err(Error::TypeError(format!(
        "{other:?} is not a constructor (stack: {:?})",
        self.call_stack
      ))),
    }
  }

  #[inline]
  fn eval_args_discard(&mut self, args: &[Expr], env: &Arc<Environment>) -> Result<()> {
    for a in args {
      self.eval_expr(a, env)?;
    }
    Ok(())
  }

  #[inline]
  fn eval_args(&mut self, args: &[Expr], env: &Arc<Environment>) -> Result<Vec<JsValue>> {
    let mut evaluated = Vec::with_capacity(args.len());
    for a in args {
      evaluated.push(self.eval_expr(a, env)?);
    }
    Ok(evaluated)
  }

  #[inline]
  fn try_inline_array_call(
    &mut self,
    arr: &Arc<RwLock<Vec<JsValue>>>,
    prop: &str,
    args: &[Expr],
    env: &Arc<Environment>,
  ) -> Option<Result<JsValue>> {
    match prop {
      "push" | "pop" | "shift" | "reverse" | "splice" | "slice" | "indexOf" | "join"
      | "toString" => Some(self.exec_inline_array_call(arr, prop, args, env)),
      _ => None,
    }
  }

  fn exec_inline_array_call(
    &mut self,
    arr: &Arc<RwLock<Vec<JsValue>>>,
    prop: &str,
    args: &[Expr],
    env: &Arc<Environment>,
  ) -> Result<JsValue> {
    match prop {
      "push" => match args.len() {
        0 => {
          let guard = arr.read();
          Ok(JsValue::Number(guard.len() as f64))
        }
        1 => {
          let val = self.eval_expr(&args[0], env)?;
          let mut guard = arr.write();
          guard.push(val);
          Ok(JsValue::Number(guard.len() as f64))
        }
        _ => {
          let mut evaluated_args = self.eval_args(args, env)?;
          let mut guard = arr.write();
          let new_len = guard.len() + evaluated_args.len();
          guard.append(&mut evaluated_args);
          Ok(JsValue::Number(new_len as f64))
        }
      },
      "pop" => {
        self.eval_args_discard(args, env)?;
        Ok(JsValue::array_pop_inlined(arr))
      }
      "shift" => {
        self.eval_args_discard(args, env)?;
        Ok(JsValue::array_shift_inlined(arr))
      }
      "reverse" => {
        self.eval_args_discard(args, env)?;
        Ok(JsValue::array_reverse_inlined(arr))
      }
      "toString" => {
        self.eval_args_discard(args, env)?;
        Ok(JsValue::array_to_string_inlined(arr))
      }
      "splice" => match args.len() {
        0 => Ok(JsValue::array_splice_inlined(arr, &[])),
        1 => {
          let a0 = self.eval_expr(&args[0], env)?;
          Ok(JsValue::array_splice_inlined(arr, &[a0]))
        }
        2 => {
          let a0 = self.eval_expr(&args[0], env)?;
          let a1 = self.eval_expr(&args[1], env)?;
          Ok(JsValue::array_splice_inlined(arr, &[a0, a1]))
        }
        3 => {
          let a0 = self.eval_expr(&args[0], env)?;
          let a1 = self.eval_expr(&args[1], env)?;
          let a2 = self.eval_expr(&args[2], env)?;
          Ok(JsValue::array_splice_inlined(arr, &[a0, a1, a2]))
        }
        _ => {
          let evaluated_args = self.eval_args(args, env)?;
          Ok(JsValue::array_splice_inlined(arr, &evaluated_args))
        }
      },
      "slice" => match args.len() {
        0 => Ok(JsValue::array_slice_inlined(arr, &[])),
        1 => {
          let a0 = self.eval_expr(&args[0], env)?;
          Ok(JsValue::array_slice_inlined(arr, &[a0]))
        }
        2 => {
          let a0 = self.eval_expr(&args[0], env)?;
          let a1 = self.eval_expr(&args[1], env)?;
          Ok(JsValue::array_slice_inlined(arr, &[a0, a1]))
        }
        _ => {
          let evaluated_args = self.eval_args(args, env)?;
          Ok(JsValue::array_slice_inlined(arr, &evaluated_args))
        }
      },
      "indexOf" => match args.len() {
        0 => Ok(JsValue::array_index_of_inlined(arr, &[])),
        1 => {
          let a0 = self.eval_expr(&args[0], env)?;
          Ok(JsValue::array_index_of_inlined(arr, &[a0]))
        }
        2 => {
          let a0 = self.eval_expr(&args[0], env)?;
          let a1 = self.eval_expr(&args[1], env)?;
          Ok(JsValue::array_index_of_inlined(arr, &[a0, a1]))
        }
        _ => {
          let evaluated_args = self.eval_args(args, env)?;
          Ok(JsValue::array_index_of_inlined(arr, &evaluated_args))
        }
      },
      "join" => match args.len() {
        0 => Ok(JsValue::array_join_inlined(arr, &[])),
        1 => {
          let a0 = self.eval_expr(&args[0], env)?;
          Ok(JsValue::array_join_inlined(arr, &[a0]))
        }
        _ => {
          let evaluated_args = self.eval_args(args, env)?;
          Ok(JsValue::array_join_inlined(arr, &evaluated_args))
        }
      },
      _ => unreachable!(),
    }
  }

  fn setup_activation_env(
    func_data: &Arc<FunctionData>,
    this_opt: Option<JsValue>,
    evaluated_args: &[JsValue],
  ) -> Arc<Environment> {
    let call_env = Arc::new(Environment::new_child(&func_data.closure));
    let args_val = if func_data.shared.uses_arguments {
      Some(JsValue::new_array(evaluated_args.to_vec()))
    } else {
      None
    };
    let hoisted_funcs_instantiated = if func_data.shared.hoisted_funcs.is_empty() {
      Vec::new()
    } else {
      let obj_proto = call_env.get_object_prototype();
      func_data
        .shared
        .hoisted_funcs
        .iter()
        .map(|f| {
          let func_val = FunctionData::from_shared_with_proto(
            Arc::clone(f),
            Arc::clone(&call_env),
            obj_proto.clone(),
          );
          (f.name.clone().unwrap_or_default(), func_val)
        })
        .collect()
    };

    call_env.init_activation_scope(ActivationScopeInit {
      this_opt,
      fname_opt: func_data
        .name()
        .map(|n| (n, JsValue::Function(Arc::clone(func_data)))),
      arguments: args_val,
      params: func_data.params(),
      args: evaluated_args,
      hoisted_vars: &func_data.shared.hoisted_vars,
      hoisted_funcs: hoisted_funcs_instantiated,
    });

    call_env
  }

  fn run_constructor(
    &mut self,
    func_data: &Arc<FunctionData>,
    callee_val: &JsValue,
    evaluated_args: &[JsValue],
  ) -> Result<JsValue> {
    if let Some(ref nat) = func_data.native_impl {
      let res = nat.call(evaluated_args)?;
      if let JsValue::Object(ref obj) = res {
        let has_proto = obj.read().contains_key(PROTO_KEY);
        if !has_proto && let Ok(proto) = callee_val.get_property(PROTOTYPE_KEY) {
          let _ = res.set_property(PROTO_KEY, proto);
        }
      }
      return Ok(res);
    }
    let instance = JsValue::new_object();
    if let Ok(proto) = callee_val.get_property(PROTOTYPE_KEY) {
      let _ = instance.set_property(PROTO_KEY, proto);
    }
    let call_env = Self::setup_activation_env(func_data, Some(instance.clone()), evaluated_args);

    let mut returned_val: Option<JsValue> = None;
    for stmt in &func_data.shared.body {
      match self.eval_stmt(stmt, &call_env) {
        Ok(_) => {}
        Err(ExecSignal::Return(v)) => {
          returned_val = Some(v);
          break;
        }
        Err(ExecSignal::Break | ExecSignal::Continue) => {
          return Err(Error::RuntimeError(
            "Illegal break/continue in constructor".to_string(),
          ));
        }
        Err(ExecSignal::Error(err)) => return Err(err),
      }
    }

    if let Some(ret) = returned_val {
      if ret.is_object_like() {
        Ok(ret)
      } else {
        Ok(instance)
      }
    } else {
      Ok(instance)
    }
  }

  /// 统一函数执行器
  pub fn invoke_function(
    &mut self,
    func: &JsValue,
    this_opt: Option<JsValue>,
    evaluated_args: &[JsValue],
  ) -> Result<JsValue> {
    self.invoke_function_named(func, this_opt, evaluated_args, "anonymous")
  }

  fn invoke_function_named(
    &mut self,
    func: &JsValue,
    this_opt: Option<JsValue>,
    evaluated_args: &[JsValue],
    callee_name: &str,
  ) -> Result<JsValue> {
    match func {
      JsValue::Native(nat) => nat.call(evaluated_args),
      JsValue::Function(func_data) => {
        let fname = func_data.name().unwrap_or(callee_name).to_string();
        self.call_stack.push(fname);
        let res = self.run_function(func_data, this_opt, evaluated_args);
        self.call_stack.pop();
        res
      }
      other => Err(Error::TypeError(format!(
        "{callee_name}: {other:?} is not a function (stack: {:?})",
        self.call_stack
      ))),
    }
  }

  fn run_function(
    &mut self,
    func_data: &Arc<FunctionData>,
    this_opt: Option<JsValue>,
    evaluated_args: &[JsValue],
  ) -> Result<JsValue> {
    if let Some(ref nat) = func_data.native_impl {
      return nat.call(evaluated_args);
    }
    let call_env = Self::setup_activation_env(func_data, this_opt, evaluated_args);

    for stmt in &func_data.shared.body {
      match self.eval_stmt(stmt, &call_env) {
        Ok(_) => {}
        Err(ExecSignal::Return(v)) => return Ok(v),
        Err(ExecSignal::Break | ExecSignal::Continue) => {
          return Err(Error::RuntimeError(
            "Illegal break/continue in function".to_string(),
          ));
        }
        Err(ExecSignal::Error(err)) => return Err(err),
      }
    }
    Ok(JsValue::Undefined)
  }
}

/// 执行一段完整的 JavaScript 源码, 走 Interpreter 的默认模式 (当前为 AST 求值器)
pub fn run_js(source: &str) -> Result<JsValue> {
  Interpreter::new().run(source)
}

/// 以指定执行模式运行源码
fn run_with(source: &str, mode: ExecMode) -> Result<JsValue> {
  let mut interp = Interpreter::new();
  interp.mode = mode;
  interp.run(source)
}

/// 字节码引擎优先执行, 桥接层执行前判定不支持的构造回落 AST; 基准与差分对照入口
pub fn run_bytecode(source: &str) -> Result<JsValue> {
  run_with(source, ExecMode::BytecodeFirst)
}

/// 只允许 Ignition 字节码引擎执行, 不支持即报错, 战役 A 收官门禁用
pub fn run_strict_bytecode(source: &str) -> Result<JsValue> {
  run_with(source, ExecMode::BytecodeOnly)
}
