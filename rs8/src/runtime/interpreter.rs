//! 抽象语法树解释执行器
//!
//! 零拷贝与高效现代 Rust 实现，支持端到端 JavaScript 解释执行

use std::{
  borrow::Cow,
  cmp::Ordering,
  f64::consts::{E, PI},
  iter::repeat_n,
  result::Result as StdResult,
  slice::from_ref,
  str::from_utf8,
  sync::Arc,
  time::{Instant, SystemTime, UNIX_EPOCH},
};

use coarsetime::Clock;
use gxhash::{HashMap, HashSet};
use itoa::Buffer;
use parking_lot::RwLock;
use v8_interpreter::{
  ACCESSOR_FLAG_KEY, BytecodeDispatcher, GETTER_PREFIX, GlobalsTable, SETTER_PREFIX, TIME_KEY,
  TIMESTAMP_KEY, canonical_index, enumerable_names, format_getter_key, format_setter_key,
  is_internal_key, same_value_x64, to_index_i64,
};
use v8_numbers::{conversions::double_to_string, double_to_int32, double_to_uint32};

use crate::{
  error::{Error, Result},
  runtime::{
    ast::*,
    bytecode_bridge::try_execute_with_bytecode_in,
    environment::{ActivationScopeInit, Environment},
    lexer::Lexer,
    parser::Parser,
    value::{
      ArrayStore, CONSTRUCTOR_KEY, FunctionData, JsValue, LENGTH_KEY, MESSAGE_KEY, NAME_KEY,
      NativeFn, PROTO_KEY, PROTOTYPE_KEY, PreferredType, SYM_TO_PRIMITIVE, TO_STRING_KEY,
      VALUE_OF_KEY, char_count, collect_hoisted, element_key, format_symbol_key,
      is_canonical_index,
    },
  },
};

/// 位移操作掩码
const SHIFT_MASK: u32 = 0x1F;

/// epoch 毫秒的单一来源: 粗粒度钟走 vDSO 缓存, 比 `SystemTime::now` 便宜一个量级, 而基准套件
/// 每轮都取时间戳
#[inline]
fn epoch_ms() -> f64 {
  Clock::now_since_epoch().as_millis() as f64
}

/// 内部控制流中断与错误信号
#[derive(Debug)]
pub(crate) enum ExecSignal {
  Return(JsValue),
  /// break/break L (Some = 标签名; 循环/标记语句臂按名匹配消费, 不匹配继续外传)
  Break(Option<String>),
  Continue(Option<String>),
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
      ExecSignal::Break(_) => Error::RuntimeError("unexpected break".to_string()),
      ExecSignal::Continue(_) => Error::RuntimeError("unexpected continue".to_string()),
    }
  }
}

/// 执行路径选择模式
#[derive(Debug, Default, PartialEq, Eq, Clone, Copy)]
pub enum ExecMode {
  /// 强制 AST 求值器
  AstOnly,
  /// 优先走 V8 Ignition 字节码引擎 (当前默认), 仅当桥接层在**执行前**判定构造不支持才回落
  /// AST 求值器; 执行期错误一律传播, 不重放
  #[default]
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
  /// 字节码引擎的全局表, 与 `global` 一一对应但在整个解释器生命周期内常驻
  ///
  /// 对标原版 NativeContext.global_dictionary: 段与段之间共享同一张表, 既免去每次把上千
  /// 内建深转换一遍, 也让函数/对象的引用身份跨段保持 (每次重建表会让
  /// `thrown.constructor === ErrCtor` 这类比较必然为 false)。
  /// 表优先于 `global`: 脚本写过的绑定以本表为准, 宿主侧 `global.define` 只对表里没有的
  /// 名字生效, 因此运行中途把 ExecMode 从 BytecodeFirst 切到 AstOnly 会读到旧值。
  pub interp_globals: GlobalsTable,
  pub call_stack: Vec<String>,
  pub mode: ExecMode,
  pub stats: ExecStats,
  /// `Array.prototype` 上被用户写过/删过的内建方法名 (#24, 与字节码链 `array_proto_watch` 同口径)
  ///
  /// 宿主硬编码表 (value.rs `array_builtin`) 是 `Array.prototype` 基线的替身; 名字一旦被动过,
  /// 替身对它失能, 读侧改为回落原型。空集时读路径只多一次 bool 级 `is_empty` 短路
  array_proto_watch: HashSet<String>,
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
      interp_globals: GlobalsTable::default(),
      global,
      call_stack: Vec::new(),
      mode: ExecMode::default(),
      stats: ExecStats::default(),
      array_proto_watch: HashSet::default(),
    }
  }

  /// 跨界调用 (字节码引擎回调宿主函数) 专用的轻量执行态
  ///
  /// 被调函数的变量解析只走 `FunctionData.closure` 派生的激活记录, `self.global` 仅在
  /// `run()`/`dispatch()` 顶层使用, 所以这里必须跳过 `init_builtins`: 重放整段内建预置脚本
  /// (词法分析 + 语法分析 + 求值近千行) 曾是每次跨界调用的固定开销 (战役 A 欠账 1)。
  /// 若日后让语句求值依赖 `self.global`, 此构造函数必须改为接收真实全局环境。
  #[must_use]
  /// 跨引擎回调的派生解释器: 函数本身在传参环境里求值, 但接收者的具名读回落
  /// (`eval_get_property` → `proto_prop`) 查的是 `global` 字段 —— 留空环境会让
  /// `Array.prototype`/`Object.prototype` 面在 BC 链回调里整体失明
  /// (`Array.prototype.every.call([1], f)` 内 `this.hasOwnProperty` 读 undefined 即此坑)
  pub fn for_cross_engine_call(root: Arc<Environment>) -> Self {
    Self {
      interp_globals: GlobalsTable::default(),
      global: root,
      call_stack: Vec::new(),
      mode: ExecMode::AstOnly,
      stats: ExecStats::default(),
      array_proto_watch: HashSet::default(),
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
    // URI 编解码四件套 (12.27): 行为在引擎 uri_encode/uri_decode 一处定义;
    // 非法转义按规范抛 URIError, 本宿主错误枚举无该变体 → RuntimeError 承载, 名字偏差记档
    let encode_uri_fn = NativeFn::Ptr(|args: &[JsValue]| -> Result<JsValue> {
      let s = args.first().map(|v| v.to_js_string()).unwrap_or_default();
      Ok(JsValue::String(v8_interpreter::uri_encode(&s, true).into()))
    });
    env.define("encodeURI".to_string(), JsValue::Native(encode_uri_fn));
    let encode_uri_component_fn = NativeFn::Ptr(|args: &[JsValue]| -> Result<JsValue> {
      let s = args.first().map(|v| v.to_js_string()).unwrap_or_default();
      Ok(JsValue::String(
        v8_interpreter::uri_encode(&s, false).into(),
      ))
    });
    env.define(
      "encodeURIComponent".to_string(),
      JsValue::Native(encode_uri_component_fn),
    );
    let decode_uri_fn = NativeFn::Ptr(|args: &[JsValue]| -> Result<JsValue> {
      let s = args.first().map(|v| v.to_js_string()).unwrap_or_default();
      v8_interpreter::uri_decode(&s, true)
        .map(|s| JsValue::String(s.into()))
        .ok_or_else(|| Error::RuntimeError("URI malformed".to_string()))
    });
    env.define("decodeURI".to_string(), JsValue::Native(decode_uri_fn));
    let decode_uri_component_fn = NativeFn::Ptr(|args: &[JsValue]| -> Result<JsValue> {
      let s = args.first().map(|v| v.to_js_string()).unwrap_or_default();
      v8_interpreter::uri_decode(&s, false)
        .map(|s| JsValue::String(s.into()))
        .ok_or_else(|| Error::RuntimeError("URI malformed".to_string()))
    });
    env.define(
      "decodeURIComponent".to_string(),
      JsValue::Native(decode_uri_component_fn),
    );

    let parse_int_fn = NativeFn::Ptr(|args: &[JsValue]| -> Result<JsValue> {
      let s = args.first().map(|v| v.to_js_string()).unwrap_or_default();
      let trimmed = s.trim();
      // radix 0/缺省走规范自动判基: 带符号前缀剥离后认 0x/0X → 16, 否则 10
      let mut radix = args.get(1).map(|v| v.to_int32()).unwrap_or(0);
      if radix != 0 && !(2..=36).contains(&radix) {
        return Ok(JsValue::Number(f64::NAN));
      }
      let bytes = trimmed.as_bytes();
      let mut start = 0usize;
      let mut sign = 1.0;
      if start < bytes.len() && (bytes[start] == b'+' || bytes[start] == b'-') {
        if bytes[start] == b'-' {
          sign = -1.0;
        }
        start += 1;
      }
      let rest = &trimmed[start..];
      // 判基只认 0x/0X→16 (V8 口径; `parseInt("0b101")` node 判 10 交 0, 与前缀名册分家)
      let has_hex_prefix = rest.starts_with("0x") || rest.starts_with("0X");
      if radix == 0 {
        radix = if has_hex_prefix { 16 } else { 10 };
      }
      let digits_src = if radix == 16 && has_hex_prefix {
        &rest[2..]
      } else {
        rest
      };
      let mut digits = String::new();
      for c in digits_src.chars() {
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
      // 规范 Math.round = floor(x+0.5) (平局向 +∞), 不是 Rust 的远离零取整;
      // [-0.5, 0) 归 -0, ±0/±∞/NaN 原样返回
      if n == 0.0 || n.is_nan() || n.is_infinite() {
        return Ok(JsValue::Number(n));
      }
      if (-0.5..0.0).contains(&n) {
        return Ok(JsValue::Number(-0.0));
      }
      Ok(JsValue::Number((n + 0.5).floor()))
    });
    let _ = math.set_property("round", JsValue::Native(round_fn));

    // Math 单目面补齐 (12.27 扫面): 语义就是 f64→f64 映射, 收进 MathUnary 一个变体按名分派
    // (sign 走 Rust signum: NaN→NaN、±0 保号, 与 JS 口径一致); 表即方法名单一真相
    const MATH_UNARY: [&str; 10] = [
      "ceil", "trunc", "cbrt", "tan", "asin", "acos", "atan", "log2", "log10", "sign",
    ];
    for name in MATH_UNARY {
      let _ = math.set_property(name, JsValue::Native(NativeFn::MathUnary(name)));
    }
    // 非纯映射的整型/多元面: clz32/imul 走 ToUint32/ToInt32 单一真相 (v8_numbers),
    // hypot 成对折叠 libm 语义 (±∞ 优先于 NaN, 与 node 同), atan2 双参
    let fround_fn = NativeFn::Ptr(|args: &[JsValue]| -> Result<JsValue> {
      let n = args.first().map(|v| v.to_number()).unwrap_or(f64::NAN);
      Ok(JsValue::Number(n as f32 as f64))
    });
    let _ = math.set_property("fround", JsValue::Native(fround_fn));
    let clz32_fn = NativeFn::Ptr(|args: &[JsValue]| -> Result<JsValue> {
      let x = args
        .first()
        .map(|v| double_to_uint32(v.to_number()))
        .unwrap_or(0);
      Ok(JsValue::Number(x.leading_zeros() as f64))
    });
    let _ = math.set_property("clz32", JsValue::Native(clz32_fn));
    let imul_fn = NativeFn::Ptr(|args: &[JsValue]| -> Result<JsValue> {
      let a = args
        .first()
        .map(|v| double_to_int32(v.to_number()))
        .unwrap_or(0);
      let b = args
        .get(1)
        .map(|v| double_to_int32(v.to_number()))
        .unwrap_or(0);
      Ok(JsValue::Number(a.wrapping_mul(b) as f64))
    });
    let _ = math.set_property("imul", JsValue::Native(imul_fn));
    let atan2_fn = NativeFn::Ptr(|args: &[JsValue]| -> Result<JsValue> {
      let y = args.first().map(|v| v.to_number()).unwrap_or(f64::NAN);
      let x = args.get(1).map(|v| v.to_number()).unwrap_or(f64::NAN);
      Ok(JsValue::Number(y.atan2(x)))
    });
    let _ = math.set_property("atan2", JsValue::Native(atan2_fn));
    let hypot_fn = NativeFn::Ptr(|args: &[JsValue]| -> Result<JsValue> {
      let mut acc = 0.0f64;
      for a in args {
        acc = acc.hypot(a.to_number());
      }
      Ok(JsValue::Number(acc))
    });
    let _ = math.set_property("hypot", JsValue::Native(hypot_fn));
    // Math.random 无法与 node 对拍值: splitmix64 (时钟纳秒混种子), 门禁只钉 [0,1) 区间
    let random_fn = NativeFn::Ptr(|_args: &[JsValue]| -> Result<JsValue> {
      use std::sync::atomic::{AtomicU64, Ordering};
      static SEED: AtomicU64 = AtomicU64::new(0x2545_F491_4F6C_DD1D);
      let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.subsec_nanos() as u64 ^ d.as_secs())
        .unwrap_or(0xDEAD_BEEF);
      let mut z = SEED.fetch_add(0x9E37_79B9_7F4A_7C15, Ordering::Relaxed) ^ nanos;
      z ^= z >> 30;
      z = z.wrapping_mul(0xBF58_476D_1CE4_E5B9);
      z ^= z >> 27;
      z = z.wrapping_mul(0x94D0_49BB_1331_11EB);
      z ^= z >> 31;
      Ok(JsValue::Number((z >> 11) as f64 / (1u64 << 53) as f64))
    });
    let _ = math.set_property("random", JsValue::Native(random_fn));

    let _ = math.set_property("PI", JsValue::Number(PI));
    let _ = math.set_property("E", JsValue::Number(E));

    env.define("Math".to_string(), math);

    // String 构造函数与静态方法
    let str_ctor = NativeFn::Ptr(|args: &[JsValue]| -> Result<JsValue> {
      // 走 to_js_string 单一真相 (函数给源码文本, #23 ⑤); Display 的 [Function: x] 摘要不外泄
      let val = args.first().map(|v| v.to_js_string()).unwrap_or_default();
      Ok(JsValue::String(val.into()))
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
      Ok(JsValue::String(s.into()))
    });
    let _ = string_data.set_property("fromCharCode", JsValue::Native(from_char_code));
    // fromCodePoint: 参数按整数码位取 (非 to_int32 回绕), 越出 0..=0x10FFFF 抛 RangeError
    let from_code_point = NativeFn::Ptr(|args: &[JsValue]| -> Result<JsValue> {
      let mut s = String::with_capacity(args.len());
      for a in args {
        let code = to_index_i64(a.to_number());
        if !(0..=0x10_FFFF).contains(&code) {
          return Err(Error::RangeError(format!("Invalid code point {code}")));
        }
        match char::from_u32(code as u32) {
          Some(ch) => s.push(ch),
          // 代理区码位是 node 合法入参但 Rust char 存不下: 落 U+FFFD (孤代理偏差记战役 C)
          None => s.push('\u{FFFD}'),
        }
      }
      Ok(JsValue::String(s.into()))
    });
    let _ = string_data.set_property("fromCodePoint", JsValue::Native(from_code_point));
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
    // 不做 ToNumber 强制转换: 字符串 "1" 一律 false (node 语义与全局 isFinite 相反)
    let is_integer_fn = NativeFn::Ptr(|args: &[JsValue]| -> Result<JsValue> {
      Ok(JsValue::Boolean(
        matches!(args.first(), Some(JsValue::Number(n)) if n.is_finite() && n.fract() == 0.0),
      ))
    });
    let _ = number_data.set_property("isInteger", JsValue::Native(is_integer_fn));
    // isSafeInteger 与 isInteger 同款不做 ToNumber 强制转换; 界限 ±(2^53-1)
    let is_safe_integer_fn = NativeFn::Ptr(|args: &[JsValue]| -> Result<JsValue> {
      Ok(JsValue::Boolean(
        matches!(args.first(), Some(JsValue::Number(n))
        if n.is_finite() && n.fract() == 0.0 && n.abs() <= 9007199254740991.0),
      ))
    });
    let _ = number_data.set_property("isSafeInteger", JsValue::Native(is_safe_integer_fn));
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
    let now_fn =
      NativeFn::Ptr(|_args: &[JsValue]| -> Result<JsValue> { Ok(JsValue::Number(epoch_ms())) });
    let date_ctor = NativeFn::Ptr(|_args: &[JsValue]| -> Result<JsValue> {
      let ms = epoch_ms();
      let obj = JsValue::new_object();
      let _ = obj.set_property(TIMESTAMP_KEY, JsValue::Number(ms));
      let _ = obj.set_property(TIME_KEY, JsValue::Number(ms));
      // 方法必须随实例落表: 只有数值槽时 `d.getTime()` 恒为 undefined, Octane 的
      // BenchmarkSuite.GetTime 因此静默退化成一轮, 基准数字失真而 Score 照样打印
      let time_of = JsValue::Native(NativeFn::boxed_value_of(JsValue::Number(ms)));
      let _ = obj.set_property("getTime", time_of.clone());
      let _ = obj.set_property(VALUE_OF_KEY, time_of);
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
              JsValue::Native(NativeFn::boxed_to_string(JsValue::String(s_desc.into()))),
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
      match args.first() {
        // ToObject 对 null/undefined 直接抛 TypeError, 不是静默返回空表
        None | Some(JsValue::Null) | Some(JsValue::Undefined) => {
          return Err(Error::TypeError(
            "Cannot convert undefined or null to object".to_string(),
          ));
        }
        // 数组的索引键也是自有可枚举键: `Object.keys([1,2])` 必须给 ["0","1"], 原实现只认
        // Object, 数组一律返回空
        Some(JsValue::Array(pm)) => {
          let guard = pm.read();
          keys.reserve(guard.len());
          for i in 0..guard.len() {
            // 战役 C ③: 洞槽不进 keys (node Object.keys([1,,2]) = ["0","2"])
            if matches!(guard[i], JsValue::TheHole) {
              continue;
            }
            keys.push(JsValue::String(i.to_string().into()));
          }
        }
        Some(JsValue::String(s)) => {
          let len = s.chars().count();
          keys.reserve(len);
          for i in 0..len {
            keys.push(JsValue::String(i.to_string().into()));
          }
        }
        Some(JsValue::Object(pm)) => {
          let guard = pm.read();
          keys.reserve(guard.len());
          // 伪键还原成真实属性名并去重 (`@@get:p`/`@@set:p` 都归到 `p`), 内部键剔除
          for name in enumerable_names(guard.keys()) {
            keys.push(JsValue::String(name.into()));
          }
        }
        // 正则实例可枚举面 (node e6): lastIndex 自有不可枚举, 只有 expando/遮蔽键进 keys
        Some(JsValue::RegExp(re)) => {
          for name in re.enumerable_own_names() {
            keys.push(JsValue::String(name.into()));
          }
        }
        _ => {}
      }
      Ok(JsValue::new_array(keys))
    });
    let define_property_fn = NativeFn::Ptr(|args: &[JsValue]| -> Result<JsValue> {
      if args.len() < 2 {
        return Ok(JsValue::Undefined);
      }
      let obj = args[0].clone();
      let prop_key = match &args[1] {
        JsValue::Symbol(s) => format_symbol_key(s),
        other => other.to_js_string(),
      };
      if let Some(desc) = args.get(2) {
        if let Ok(val) = desc.get_property("value")
          && val != JsValue::Undefined
        {
          let _ = obj.set_property(&prop_key, val);
        }
        // 与对象字面量同一套伪键约定, 且必须登记哨兵键: 属性读点靠它判定有没有 accessor
        let mut has_accessor = false;
        if let Ok(get_fn) = desc.get_property("get")
          && matches!(get_fn, JsValue::Function(_) | JsValue::Native(_))
        {
          let _ = obj.set_property(&format_getter_key(&prop_key), get_fn);
          has_accessor = true;
        }
        if let Ok(set_fn) = desc.get_property("set")
          && matches!(set_fn, JsValue::Function(_) | JsValue::Native(_))
        {
          let _ = obj.set_property(&format_setter_key(&prop_key), set_fn);
          has_accessor = true;
        }
        if has_accessor {
          let _ = obj.set_property(ACCESSOR_FLAG_KEY, JsValue::Boolean(true));
        }
      }
      Ok(obj)
    });
    let _ = object_data.set_property("defineProperty", JsValue::Native(define_property_fn));
    let _ = object_data.set_property("keys", JsValue::Native(keys_fn));

    // values/entries/assign 的键集与 keys_fn 同一真相: 索引键族 (Array/String) 与自有可枚举名
    // (Object, 伪键归并去重)。accessor 值物化在 JS prelude 覆盖层完成 (属性读点会跳 getter),
    // 这里的原生实现只搬值槽, 作为 prelude 之前的兜底存在
    fn own_enumerable_entries(obj: &JsValue) -> Vec<(String, JsValue)> {
      match obj {
        JsValue::Array(pm) => {
          let guard = pm.read();
          (0..guard.len())
            // 战役 C ③: 洞槽不进可枚举条目面 (values/entries/assign/for-in 共走此处)
            .filter(|&i| !matches!(guard[i], JsValue::TheHole))
            .map(|i| {
              (
                i.to_string(),
                guard.get(i).cloned().unwrap_or(JsValue::Undefined),
              )
            })
            .collect()
        }
        JsValue::String(s) => s
          .chars()
          .enumerate()
          .map(|(i, ch)| (i.to_string(), JsValue::String(ch.to_string().into())))
          .collect(),
        JsValue::Object(pm) => {
          let guard = pm.read();
          enumerable_names(guard.keys())
            .into_iter()
            .filter_map(|name| guard.get(name).map(|val| (name.to_string(), val.clone())))
            .collect()
        }
        // Number/Boolean/Symbol 装箱面尚无自有可枚举键, 与 node 的 [] 一致
        _ => Vec::new(),
      }
    }
    fn to_object_checked(arg: Option<&JsValue>) -> Result<JsValue> {
      match arg {
        // ToObject 对 null/undefined 直接抛 TypeError (与 keys_fn 同款护栏)
        None | Some(JsValue::Null) | Some(JsValue::Undefined) => Err(Error::TypeError(
          "Cannot convert undefined or null to object".to_string(),
        )),
        Some(v) => Ok(v.clone()),
      }
    }
    let values_fn = NativeFn::Ptr(|args: &[JsValue]| -> Result<JsValue> {
      let target = to_object_checked(args.first())?;
      Ok(JsValue::new_array(
        own_enumerable_entries(&target)
          .into_iter()
          .map(|(_, val)| val)
          .collect(),
      ))
    });
    let entries_fn = NativeFn::Ptr(|args: &[JsValue]| -> Result<JsValue> {
      let target = to_object_checked(args.first())?;
      Ok(JsValue::new_array(
        own_enumerable_entries(&target)
          .into_iter()
          .map(|(k, val)| JsValue::new_array(vec![JsValue::String(k.into()), val]))
          .collect(),
      ))
    });
    let assign_fn = NativeFn::Ptr(|args: &[JsValue]| -> Result<JsValue> {
      let target = to_object_checked(args.first())?;
      // 原始 target 经 ToObject 装箱后写入不持久, node 返回装箱对象; 先原样回吐 (偏差记战役 C)
      if !matches!(target, JsValue::Object(_) | JsValue::Array(_)) {
        return Ok(target);
      }
      for src in &args[1..] {
        if matches!(src, JsValue::Null | JsValue::Undefined) {
          continue;
        }
        for (k, val) in own_enumerable_entries(src) {
          let _ = target.set_property(&k, val);
        }
      }
      Ok(target)
    });
    let get_prototype_of_fn = NativeFn::from_context(
      Arc::clone(env),
      |env: &Arc<Environment>, args: &[JsValue]| -> Result<JsValue> {
        let own = match args.first() {
          // 只读自有 `__proto__` 槽 (不经 get_property, 避免顺链继承误报)
          Some(JsValue::Object(pm)) => pm.read().get(PROTO_KEY).cloned(),
          Some(JsValue::Array(pm)) => {
            let guard = pm.read();
            guard.props.as_ref().and_then(|p| p.get(PROTO_KEY)).cloned()
          }
          _ => None,
        };
        Ok(own.unwrap_or_else(|| {
          match args.first() {
            // 无槽的宿主构造对象回落 Object.prototype (node 口径)
            Some(JsValue::Object(_) | JsValue::Array(_)) => {
              env.get_object_prototype().unwrap_or(JsValue::Null)
            }
            // 函数值 (含原生) 缺槽回落全局 Function.prototype (node 口径)
            Some(JsValue::Function(_) | JsValue::Native(_)) => env
              .get("Function")
              .and_then(|f| f.get_property(PROTOTYPE_KEY).ok())
              .unwrap_or(JsValue::Null),
            // 原始值的原型挂在 String.prototype/Number.prototype 等包装面上, 该表尚未物化
            // (战役 C): 先给 null 而不是错误
            _ => JsValue::Null,
          }
        }))
      },
    );
    let set_prototype_of_fn = NativeFn::Ptr(|args: &[JsValue]| -> Result<JsValue> {
      let Some(target) = args.first() else {
        return Err(Error::TypeError(
          "Object.setPrototypeOf called on null or undefined".to_string(),
        ));
      };
      if !matches!(target, JsValue::Object(_) | JsValue::Array(_)) {
        return Err(Error::TypeError(
          "Object.setPrototypeOf called on non-object".to_string(),
        ));
      }
      let proto = args.get(1).cloned().unwrap_or(JsValue::Null);
      if !matches!(proto, JsValue::Object(_) | JsValue::Null) {
        return Err(Error::TypeError(
          "Object prototype may only be an Object or null".to_string(),
        ));
      }
      target.set_property(PROTO_KEY, proto)?;
      Ok(target.clone())
    });
    let is_fn = NativeFn::Ptr(|args: &[JsValue]| -> Result<JsValue> {
      let x = args.first().cloned().unwrap_or(JsValue::Undefined);
      let y = args.get(1).cloned().unwrap_or(JsValue::Undefined);
      // SameValue: NaN 自等且不与 0 同, +0 与 -0 相异; 其余类型走结构化相等
      Ok(JsValue::Boolean(match (&x, &y) {
        (JsValue::Number(a), JsValue::Number(b)) => same_value_x64(*a, *b),
        _ => x == y,
      }))
    });
    let _ = object_data.set_property("values", JsValue::Native(values_fn));
    let _ = object_data.set_property("entries", JsValue::Native(entries_fn));
    let _ = object_data.set_property("assign", JsValue::Native(assign_fn));
    let _ = object_data.set_property("getPrototypeOf", JsValue::Native(get_prototype_of_fn));
    let _ = object_data.set_property("setPrototypeOf", JsValue::Native(set_prototype_of_fn));
    let _ = object_data.set_property("is", JsValue::Native(is_fn));
    // getOwnPropertyNames (12.27): 与 keys 同表但不过滤可枚举; 内部伪键与 __proto__ 槽不外泄;
    // 函数面 length/name/prototype 未物化记战役 C
    let get_own_prop_names_fn = NativeFn::Ptr(|args: &[JsValue]| -> Result<JsValue> {
      let mut names: Vec<String> = Vec::new();
      match args.first() {
        Some(JsValue::Object(o)) => {
          names.extend(
            o.read()
              .keys()
              .filter(|k| !is_internal_key(k) && *k != "__proto__")
              .cloned(),
          );
        }
        Some(JsValue::Array(a)) => {
          let guard = a.read();
          // 战役 C ③: 洞槽不进名册 (node gOPN([1,,2]) = ["0","2","length"])
          names.extend(
            (0..guard.len())
              .filter(|&i| !matches!(guard[i], JsValue::TheHole))
              .map(|i| i.to_string()),
          );
          names.push("length".to_string());
          if let Some(props) = guard.props.as_ref() {
            names.extend(
              props
                .keys()
                .filter(|k| !is_internal_key(k) && *k != "__proto__")
                .cloned(),
            );
          }
        }
        Some(JsValue::String(s)) => {
          names.extend((0..s.chars().count()).map(|i| i.to_string()));
          names.push("length".to_string());
        }
        // 正则实例 own 名册 (node 实测): lastIndex + expando/遮蔽键, 名册单一真相 =
        // RegExpData::own_property_names (12.37 动态袋)
        Some(JsValue::RegExp(re)) => {
          names.extend(re.own_property_names());
        }
        Some(JsValue::Undefined | JsValue::Null) | None => {
          return Err(Error::TypeError(
            "Cannot convert undefined or null to object".to_string(),
          ));
        }
        Some(_) => {}
      }
      Ok(JsValue::new_array(
        names
          .into_iter()
          .map(|s| JsValue::String(s.into()))
          .collect(),
      ))
    });
    let _ = object_data.set_property(
      "getOwnPropertyNames",
      JsValue::Native(get_own_prop_names_fn),
    );
    // fromEntries (12.27): 条目取数组/类数组, 键走 ToString; Map 等迭代器条目记战役 C
    let from_entries_fn = NativeFn::Ptr(|args: &[JsValue]| -> Result<JsValue> {
      // 单条 entry → (键, 值): 数组取 [0],[1]; 类数组对象按 "0"/"1" 属性读
      fn entry_pair(e: &JsValue) -> (String, JsValue) {
        match e {
          JsValue::Array(inner) => {
            let g = inner.read();
            (
              g.first().map(|x| x.to_js_string()).unwrap_or_default(),
              g.get(1).cloned().unwrap_or(JsValue::Undefined),
            )
          }
          other => (
            other
              .get_property("0")
              .unwrap_or(JsValue::Undefined)
              .to_js_string(),
            other.get_property("1").unwrap_or(JsValue::Undefined),
          ),
        }
      }
      let obj = JsValue::new_object();
      match args.first() {
        Some(JsValue::Array(a)) => {
          let list = a.read().to_vec();
          for e in list {
            let (k, v) = entry_pair(&e);
            obj.set_property(&k, v)?;
          }
        }
        Some(other @ (JsValue::Object(_) | JsValue::Function(_) | JsValue::Native(_))) => {
          let n = v8_interpreter::to_index_i64(
            other
              .get_property("length")
              .unwrap_or(JsValue::Undefined)
              .to_number(),
          )
          .max(0) as usize;
          for i in 0..n {
            let e = other
              .get_property(&i.to_string())
              .unwrap_or(JsValue::Undefined);
            let (k, v) = entry_pair(&e);
            obj.set_property(&k, v)?;
          }
        }
        _ => {
          return Err(Error::TypeError(
            "Object.fromEntries: expects an iterable of entries".to_string(),
          ));
        }
      }
      Ok(obj)
    });
    let _ = object_data.set_property("fromEntries", JsValue::Native(from_entries_fn));
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
        // 战役 C ③: new Array(n) 交的是 n 个洞不是 n 个 undefined (node `0 in new Array(3)` 假)
        items.resize(len, JsValue::TheHole);
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
    // Array.of: 参数即元素 (与单参 Number 的 Array 构造判长口径相对, node 19.4.2)
    let of_fn = NativeFn::Ptr(|args: &[JsValue]| -> Result<JsValue> {
      Ok(JsValue::new_array(args.to_vec()))
    });
    let _ = array_data.set_property("of", JsValue::Native(of_fn));
    // Array.from 基础面 (12.27): 字符串/数组/类数组 {length}; mapfn 回调要执行态而宿主 Ptr
    // 拿不到 → 战役 C (回调物化), 宁报错不静默忽略
    let from_fn = NativeFn::Ptr(|args: &[JsValue]| -> Result<JsValue> {
      if args.get(1).is_some_and(|v| v.is_callable()) {
        return Err(Error::TypeError(
          "Array.from: mapfn 回调未接入 (战役 C)".to_string(),
        ));
      }
      match args.first() {
        Some(JsValue::String(s)) => Ok(JsValue::new_array(
          s.chars()
            .map(|c| JsValue::String(c.to_string().into()))
            .collect(),
        )),
        Some(JsValue::Array(a)) => Ok(JsValue::new_array(a.read().to_vec())),
        Some(JsValue::Undefined | JsValue::Null) | None => Err(Error::TypeError(format!(
          "Array.from: {} is not iterable",
          args
            .first()
            .map(|v| v.to_js_string())
            .unwrap_or_else(|| "undefined".to_string())
        ))),
        Some(other @ (JsValue::Object(_) | JsValue::Function(_) | JsValue::Native(_))) => {
          let len_v = other.get_property("length").unwrap_or(JsValue::Undefined);
          let n = v8_interpreter::to_index_i64(len_v.to_number()).max(0) as usize;
          let mut items = Vec::with_capacity(n.min(1_000_000));
          for i in 0..n {
            items.push(
              other
                .get_property(&i.to_string())
                .unwrap_or(JsValue::Undefined),
            );
          }
          Ok(JsValue::new_array(items))
        }
        Some(primitive) => Err(Error::TypeError(format!(
          "Array.from: {} is not iterable",
          primitive.to_js_string()
        ))),
      }
    });
    let _ = array_data.set_property("from", JsValue::Native(from_fn));
    if let Ok(arr_proto) = array_data.get_property(PROTOTYPE_KEY)
      && let Some(obj_ctor) = env.get("Object")
      && let Ok(obj_proto) = obj_ctor.get_property("prototype")
    {
      let _ = arr_proto.set_property(PROTO_KEY, obj_proto);
    }
    env.define("Array".to_string(), JsValue::Function(Arc::new(array_data)));

    // JSON 对象
    let json_obj = JsValue::new_object();
    /// space 实参归一后的步长 (Number 或截 10 字符的 String)
    ///
    /// 两种步长都走换行缩进, 只差在层的前缀算法: 数值 = 每层累计补齐空格, 字符串 =
    /// 步长按绝对深度重复拼接。两链共用同一闭包 (bridge 直发 `NativeFn::Ptr`, 无第二份实现)
    enum JsIndent {
      Numeric(String),
      String(String),
    }
    /// 具名键→值映射的 JSON 对象体 (Object 臂与正则 expando 臂共用, 一处定义 12.37):
    /// 键集与 for-in/Object.keys 同一套真相 —— 内部伪键 (`@@get:`/`@@accessor` 等) 与
    /// `__` 槽位键 (`__proto__`/`__timestamp__`) 不得泄进序列化结果; 全键序列化落空交 "{}"
    fn json_object_body(
      map: &HashMap<String, JsValue>,
      opts: Option<&JsIndent>,
      lead: &str,
      close_seg: &str,
      child_cur: &str,
      depth: usize,
    ) -> String {
      let mut keys: Vec<&String> = map
        .keys()
        .filter(|k| !is_internal_key(k) && !k.starts_with("__"))
        .collect();
      keys.sort();
      let sep = if opts.is_none() { ":" } else { ": " };
      let mut pairs = Vec::with_capacity(keys.len());
      for k in keys {
        if let Some(v) = map.get(k)
          && let Some(s) = serialize_to_json_ex(v, opts, child_cur, depth + 1)
        {
          pairs.push(format!("{lead}\"{k}\"{sep}{s}"));
        }
      }
      if pairs.is_empty() {
        return "{}".to_string();
      }
      let mut obj_out = String::from("{");
      obj_out.push_str(&pairs.join(","));
      obj_out.push_str(close_seg);
      obj_out.push('}');
      obj_out
    }
    /// JSON 值 → 字符串。`opts=None` 走紧凑形; 有值时数值/字符串两种步长共用同一换行模型,
    /// 仅子层前缀算法不同 (数值累计 `cur+step`, 字符串按深度 `step.repeat(d)`)
    fn serialize_to_json_ex(
      val: &JsValue,
      opts: Option<&JsIndent>,
      cur: &str,
      depth: usize,
    ) -> Option<String> {
      let compact = opts.is_none();
      let (numeric, step) = match opts {
        Some(JsIndent::Numeric(s)) => (true, s.as_str()),
        Some(JsIndent::String(s)) => (false, s.as_str()),
        None => (false, ""),
      };
      // 子层缩进前缀: 收口行换行后直接铺这一串; 本层收口段 = 换行 + 本级 cur
      let child_cur = if compact {
        String::new()
      } else if numeric {
        format!("{cur}{step}")
      } else {
        step.repeat(depth + 1)
      };
      let lead = if compact {
        String::new()
      } else {
        format!("\n{child_cur}")
      };
      let close_seg = if compact {
        String::new()
      } else {
        format!("\n{cur}")
      };
      // 每项自带前置换行 + 缩进, 连接符恒为逗号
      let join: &str = ",";
      let esc = |s: &str| -> String {
        format!(
          "\"{}\"",
          s.replace('\\', "\\\\")
            .replace('"', "\\\"")
            .replace('\n', "\\n")
            .replace('\r', "\\r")
            .replace('\t', "\\t")
        )
      };
      let nest = |v: &JsValue, inner: &str, d: usize| -> String {
        serialize_to_json_ex(v, opts, inner, d).unwrap_or_else(|| "null".to_string())
      };
      match val {
        // 洞与 undefined 同谱 (node: JSON 数组位吐 null, 对象键位整体缺席)
        JsValue::Undefined
        | JsValue::TheHole
        | JsValue::Function(_)
        | JsValue::Native(_)
        | JsValue::Symbol(_) => None,
        JsValue::BigInt(n) => {
          let mut buf = Buffer::new();
          Some(buf.format(*n).to_string())
        }
        JsValue::RegExp(re) => {
          // expando/遮蔽键按普通自有属性参与序列化 (node: `/a/g` 带 foo → `{"foo":1}`);
          // 代管面 (source/flags/lastIndex…) 恒不在袋里, 裸正则自然退化成原 "{}" 形
          let bag = re.props.read();
          Some(json_object_body(
            &bag, opts, &lead, &close_seg, &child_cur, depth,
          ))
        }
        JsValue::Null => Some("null".to_string()),
        JsValue::Boolean(b) => Some(b.to_string()),
        JsValue::Number(n) => {
          // JSON 的数值就是 ToString: 手写 `f64::to_string` 不切科学计数法 (1e21 会印成
          // 一长串数字), `as i64` 又把超出 2^53 的整值截成 i64::MAX
          if n.is_nan() || n.is_infinite() {
            Some("null".to_string())
          } else {
            Some(double_to_string(*n))
          }
        }
        JsValue::String(s) => Some(esc(s)),
        JsValue::Array(arr) => {
          let guard = arr.read();
          if guard.is_empty() {
            return Some("[]".to_string());
          }
          let items: Vec<String> = guard
            .iter()
            .map(|item| format!("{lead}{}", nest(item, &child_cur, depth + 1)))
            .collect();
          // 收口行的缩进就是本级 inner (数字模式 = cur+step, 字符串模式与 lead 同串)
          let mut arr_out = String::from("[");
          arr_out.push_str(&items.join(join));
          arr_out.push_str(&close_seg);
          arr_out.push(']');
          Some(arr_out)
        }
        JsValue::Object(obj) => {
          let guard = obj.read();
          Some(json_object_body(
            &guard, opts, &lead, &close_seg, &child_cur, depth,
          ))
        }
      }
    }
    let stringify_fn = NativeFn::Ptr(|args: &[JsValue]| -> Result<JsValue> {
      let Some(v) = args.first() else {
        return Ok(JsValue::Undefined);
      };
      // 第三实参只认 Number/String (规范 GetIndentPrefix 口径); Boolean/Object/缺省一律紧凑形
      let opts = match args.get(2) {
        Some(JsValue::Number(n)) => {
          // 规范口径: NaN→0 层紧凑, ±∞ 由 clamp 钳到 [0,10]; `clamp` 自身遇 NaN 会 panic
          let count = if n.is_nan() {
            0
          } else {
            n.floor().clamp(0.0, 10.0) as usize
          };
          if count == 0 {
            None
          } else {
            Some(JsIndent::Numeric(" ".repeat(count)))
          }
        }
        Some(JsValue::String(s)) if !s.is_empty() => {
          Some(JsIndent::String(s.chars().take(10).collect::<String>()))
        }
        _ => None,
      };
      // 顶层序列化不出文本 (undefined/function/symbol) 时按规范返回 undefined 值本身, 非空串
      let body = match serialize_to_json_ex(v, opts.as_ref(), "", 0) {
        Some(s) => s,
        None => return Ok(JsValue::Undefined),
      };
      Ok(JsValue::String(body.into()))
    });
    // JSON.parse: 与 stringify 配套的最小递归下降解析器 (RFC 8259 语法面)
    struct JsonParser<'a> {
      bytes: &'a [u8],
      pos: usize,
    }

    impl JsonParser<'_> {
      fn fail(what: &str, at: usize) -> Error {
        Error::RuntimeError(format!("JSON.parse: unexpected {what} at position {at}"))
      }
      fn skip_ws(&mut self) {
        while matches!(
          self.bytes.get(self.pos),
          Some(b' ') | Some(b'\t') | Some(b'\n') | Some(b'\r')
        ) {
          self.pos += 1;
        }
      }
      fn starts_with(&self, word: &str) -> bool {
        self.bytes[self.pos..].starts_with(word.as_bytes())
      }
      fn parse_literal(&mut self, word: &'static str, val: JsValue) -> Result<JsValue> {
        if self.starts_with(word) {
          self.pos += word.len();
          Ok(val)
        } else {
          Err(Self::fail("token", self.pos))
        }
      }
      fn parse_value(&mut self) -> Result<JsValue> {
        self.skip_ws();
        match self.bytes.get(self.pos).copied() {
          Some(b'{') => {
            self.pos += 1;
            let obj = JsValue::new_object();
            self.skip_ws();
            if self.bytes.get(self.pos) == Some(&b'}') {
              self.pos += 1;
              return Ok(obj);
            }
            loop {
              self.skip_ws();
              if self.bytes.get(self.pos) != Some(&b'"') {
                return Err(Self::fail("property name", self.pos));
              }
              let key = self.parse_string()?;
              self.skip_ws();
              if self.bytes.get(self.pos) != Some(&b':') {
                return Err(Self::fail("':'", self.pos));
              }
              self.pos += 1;
              let val = self.parse_value()?;
              obj.set_property(&key, val)?;
              self.skip_ws();
              match self.bytes.get(self.pos) {
                Some(b',') => self.pos += 1,
                Some(b'}') => {
                  self.pos += 1;
                  break;
                }
                _ => return Err(Self::fail("',' or '}'", self.pos)),
              }
            }
            Ok(obj)
          }
          Some(b'[') => {
            self.pos += 1;
            let mut items = Vec::new();
            self.skip_ws();
            if self.bytes.get(self.pos) == Some(&b']') {
              self.pos += 1;
              return Ok(JsValue::new_array(items));
            }
            loop {
              items.push(self.parse_value()?);
              self.skip_ws();
              match self.bytes.get(self.pos) {
                Some(b',') => self.pos += 1,
                Some(b']') => {
                  self.pos += 1;
                  break;
                }
                _ => return Err(Self::fail("',' or ']'", self.pos)),
              }
            }
            Ok(JsValue::new_array(items))
          }
          Some(b'"') => Ok(JsValue::String(self.parse_string()?.into())),
          Some(b't') => self.parse_literal("true", JsValue::Boolean(true)),
          Some(b'f') => self.parse_literal("false", JsValue::Boolean(false)),
          Some(b'n') => self.parse_literal("null", JsValue::Null),
          Some(c) if c == b'-' || c.is_ascii_digit() => self.parse_number(),
          _ => Err(Self::fail("token", self.pos)),
        }
      }
      fn parse_number(&mut self) -> Result<JsValue> {
        let start = self.pos;
        if self.bytes.get(self.pos) == Some(&b'-') {
          self.pos += 1;
        }
        while self.pos < self.bytes.len() && self.bytes[self.pos].is_ascii_digit() {
          self.pos += 1;
        }
        if self.bytes.get(self.pos) == Some(&b'.') {
          self.pos += 1;
          while self.pos < self.bytes.len() && self.bytes[self.pos].is_ascii_digit() {
            self.pos += 1;
          }
        }
        if matches!(self.bytes.get(self.pos), Some(b'e') | Some(b'E')) {
          self.pos += 1;
          if matches!(self.bytes.get(self.pos), Some(b'+') | Some(b'-')) {
            self.pos += 1;
          }
          while self.pos < self.bytes.len() && self.bytes[self.pos].is_ascii_digit() {
            self.pos += 1;
          }
        }
        from_utf8(&self.bytes[start..self.pos])
          .ok()
          .and_then(|t| t.parse::<f64>().ok())
          .map(JsValue::Number)
          .ok_or_else(|| Self::fail("number", start))
      }
      fn hex4(&mut self) -> Result<u32> {
        if self.pos + 4 > self.bytes.len() {
          return Err(Self::fail("unicode escape", self.pos));
        }
        let raw = from_utf8(&self.bytes[self.pos..self.pos + 4])
          .ok()
          .and_then(|t| u32::from_str_radix(t, 16).ok())
          .unwrap_or(0xFFFF);
        self.pos += 4;
        Ok(raw)
      }
      fn parse_string(&mut self) -> Result<String> {
        self.pos += 1;
        let mut out = String::new();
        let mut segment = self.pos;
        while self.pos < self.bytes.len() {
          let b = self.bytes[self.pos];
          if b == b'"' {
            out.push_str(
              from_utf8(&self.bytes[segment..self.pos])
                .map_err(|_| Self::fail("invalid UTF-8", self.pos))?,
            );
            self.pos += 1;
            return Ok(out);
          }
          if b != b'\\' {
            self.pos += 1;
            continue;
          }
          out.push_str(
            from_utf8(&self.bytes[segment..self.pos])
              .map_err(|_| Self::fail("invalid UTF-8", self.pos))?,
          );
          self.pos += 1;
          let esc = self.bytes.get(self.pos).copied().unwrap_or(0);
          let plain = match esc {
            b'"' | b'\\' | b'/' => esc,
            b'b' => 0x08,
            b'f' => 0x0C,
            b'n' => b'\n',
            b'r' => b'\r',
            b't' => b'\t',
            b'u' => {
              self.pos += 1;
              let hi = self.hex4()?;
              let cp = if (0xD800..0xDC00).contains(&hi)
                && self.bytes.get(self.pos) == Some(&b'\\')
                && self.bytes.get(self.pos + 1) == Some(&b'u')
              {
                self.pos += 2;
                let lo = self.hex4()?;
                if (0xDC00..0xE000).contains(&lo) {
                  0x1_0000 + ((hi - 0xD800) << 10) + (lo - 0xDC00)
                } else {
                  hi
                }
              } else {
                hi
              };
              // 孤代理 Rust String 存不下, 落 U+FFFD (UTF-16 面偏差记战役 C)
              out.push(char::from_u32(cp).unwrap_or('\u{FFFD}'));
              self.pos += 1;
              segment = self.pos;
              continue;
            }
            _ => return Err(Self::fail("escape character", self.pos)),
          };
          out.push(plain as char);
          self.pos += 1;
          segment = self.pos;
        }
        Err(Self::fail("unterminated string", self.pos))
      }
    }
    let parse_fn = NativeFn::Ptr(|args: &[JsValue]| -> Result<JsValue> {
      // 本引擎错误枚举没有 SyntaxError 变体: 语法错折 RuntimeError, 可捕获性不变
      let text = args.first().map(|v| v.to_js_string()).unwrap_or_default();
      let mut p = JsonParser {
        bytes: text.as_bytes(),
        pos: 0,
      };
      let val = p.parse_value()?;
      p.skip_ws();
      if p.pos != p.bytes.len() {
        return Err(JsonParser::fail("trailing characters", p.pos));
      }
      Ok(val)
    });
    let _ = json_obj.set_property("stringify", JsValue::Native(stringify_fn));
    let _ = json_obj.set_property("parse", JsValue::Native(parse_fn));
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
      let _ = proto.set_property(NAME_KEY, JsValue::String(err_name.into()));
      let _ = proto.set_property(MESSAGE_KEY, JsValue::String(String::new().into()));

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
      // own 判定走自有名册 (`p in this` 替身会把链上的 toString 等算成自有, node 交 false);
      // keys 替身又漏数组 length —— 用 gOPN (含不可枚举的自有面)。只配 indexOf 不配
      // `.some`: 名册数组跨桥后回调法接不上 (BC 深拷贝家族), String(p) 让 hasOwnProperty(0)
      // 命中 "0", symbol 键本模型不存在恒 false
      Object.prototype.hasOwnProperty = function (p) {
        if (typeof p === "symbol") return false;
        return Object.getOwnPropertyNames(this).indexOf(typeof p === "string" ? p : String(p)) !== -1;
      };
      Object.prototype.propertyIsEnumerable = function (p) {
        return Object.keys(this).indexOf(p) !== -1;
      };
      // ES2022 静态面: 复用自家 hasOwnProperty 替身 (call 借位), 不另立口径防两处漂移
      Object.hasOwn = function (o, p) {
        return Object.prototype.hasOwnProperty.call(o, p);
      };
      // String.raw: 模板面 (tagged templates) 缺致真用法不可表达, 手写对象形先收 (探针钉)。
      // 段数由 raw 名册定 (len-1), 实参多退少补; 实参计数走 ToLength 钳位 (node
      // `raw({raw:[..]}, "a", 2**40 个)` 面) —— 本轮以常规面为准
      String.raw = function (site) {
        var raws = (site === undefined || site === null) ? undefined : site.raw;
        if (raws === undefined || raws === null) {
          throw new TypeError("Cannot read properties of null (reading 'raw')");
        }
        // node 实测: 段数由 raw 名册定 (空名册吐 ""); 实参**存在**才插入 (缺位不吐
        // "undefined" 字面, 显式传 undefined 才吐); 多余实参忽略
        var res = "";
        for (var i = 0; i < raws.length; i++) {
          if (i > 0 && i < arguments.length) res = res + String(arguments[i]);
          res = res + String(raws[i]);
        }
        return res;
      };
      // 沿 proto 链逐跳 identity 比对; getPrototypeOf 在链顶 (Object.prototype) 两链都
      // 不吐 null, 故 64 跳兜底与引擎属性读的 max-depth 同口径, 防自紧环死循环
      Object.prototype.isPrototypeOf = function (v) {
        if (v === undefined || v === null || typeof v !== "object") return false;
        var o = Object.getPrototypeOf(v);
        for (var i = 0; i < 64 && o !== undefined && o !== null; i++) {
          if (o === this) return true;
          o = Object.getPrototypeOf(o);
        }
        return false;
      };
      // assign/values/entries 走属性读点取值: 原生替身只会搬值槽, 源对象上的 accessor
      // 会被整个丢掉 (node 的 CopyDataProperties/ToKeyValue 都按 Get 语义跳 getter,
      // `{...带getter对象}` 与 Object.values 都应物化); keys 枚举面保持原生
      // 第二形参只凑 .length 位 (node Object.assign.length=2, 展开逻辑走 arguments 不受影响)
      Object.assign = function (target, source) {
        if (target === undefined || target === null) {
          throw new TypeError("Cannot convert undefined or null to object");
        }
        for (var i = 1; i < arguments.length; i++) {
          var src = arguments[i];
          if (src === undefined || src === null) continue;
          var keys = Object.keys(src);
          for (var j = 0; j < keys.length; j++) {
            target[keys[j]] = src[keys[j]];
          }
        }
        return target;
      };
      Object.values = function (obj) {
        var keys = Object.keys(obj);
        var res = [];
        for (var i = 0; i < keys.length; i++) res.push(obj[keys[i]]);
        return res;
      };
      Object.entries = function (obj) {
        var keys = Object.keys(obj);
        var res = [];
        for (var i = 0; i < keys.length; i++) res.push([keys[i], obj[keys[i]]]);
        return res;
      };
      // ES2024 Object.groupBy: 数组样 (length 驱动) 名册; 元素与键都按 Get 语义 (洞交
      // undefined, node 稀疏 [1,,3] 也收 undefined 成员); 键 String() 化; 结果挂
      // Object.create(null) (node getPrototypeOf 交 null)。回调走 JS 世界内调用 (reduce
      // 族先例); 非数组样迭代器另挂三轮迭代器协议残档
      Object.groupBy = function (iterable, fn) {
        if (typeof fn !== "function") throw new TypeError("fn is not a function");
        if (iterable === undefined || iterable === null) {
          throw new TypeError("Cannot convert undefined or null to object");
        }
        var groups = Object.create(null);
        var len = iterable.length;
        for (var i = 0; i < len; i++) {
          var el = iterable[i];
          var key = String(fn(el, i));
          if (groups[key] === undefined) groups[key] = [el];
          else groups[key].push(el);
        }
        return groups;
      };
      // 以下数组方法全部先取一次 this.length 再循环: 规范是 GetLength 一次, 回调里往同一个数组
      // push 不会把循环撑成无限 (`a.map(function(){a.push(0)})` 在 node 返回 3 个元素)
      //
      // 带回调的方法一律"先取元素进局部变量, 再调回调": 规范每步只 GetElement 一次, 且把这次读到
      // 的值同时用于回调实参与 filter 的收集。写成 fn(this[i], ...) 再 res.push(this[i]) 会让回调
      // 里对同一下标的改写反映到结果上 (`e.filter(function(x){e[0]=99;return true})` 交出 99,
      // node 交 1), 元素被 pop 掉时也会少收一项
      Array.prototype.join = function (sep) {
        if (sep === undefined) sep = ",";
        var s = "";
        var len = this.length;
        for (var i = 0; i < len; i++) {
          if (i > 0) s = s + sep;
          var v = this[i];
          if (v !== undefined && v !== null) s = s + v;
        }
        return s;
      };
      Array.prototype.map = function (fn) {
        // 战役 C ③: 洞跳回调且原样进结果 (new Array(len) 自带洞, 只补写实位)
        var len = this.length;
        var res = new Array(len);
        for (var i = 0; i < len; i++) {
          if (!this.hasOwnProperty(i)) continue;
          var el = this[i];
          res[i] = fn(el, i, this);
        }
        return res;
      };
      Array.prototype.indexOf = function (val, fromIndex) {
        var start = fromIndex || 0;
        var len = this.length;
        for (var i = start; i < len; i++) {
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
        var len = this.length;
        for (var i = 0; i < len; i++) res.push(this[i]);
        for (var a = 0; a < arguments.length; a++) {
          var arg = arguments[a];
          if (Array.isArray(arg)) {
            var argLen = arg.length;
            for (var j = 0; j < argLen; j++) res.push(arg[j]);
          } else {
            res.push(arg);
          }
        }
        return res;
      };
      Array.prototype.forEach = function (fn) {
        var len = this.length;
        for (var i = 0; i < len; i++) {
          if (!this.hasOwnProperty(i)) continue;
          var el = this[i];
          fn(el, i, this);
        }
      };
      Array.prototype.filter = function (fn) {
        var res = [];
        var len = this.length;
        for (var i = 0; i < len; i++) {
          if (!this.hasOwnProperty(i)) continue;
          var el = this[i];
          if (fn(el, i, this)) res.push(el);
        }
        return res;
      };
      Array.prototype.reduce = function (fn, initial) {
        // 规范先做 IsCallable 检查再进循环 —— 零次循环也不放过 ([1].reduce() 必须抛)
        if (typeof fn !== "function") throw new TypeError("Reduce of non-callable callback");
        // 缺省与显式传 undefined 是两回事: 只有"没传第二个参数"才取首个实位作累子 (洞不作累子)
        var len = this.length;
        var idx = 0;
        var acc = initial;
        if (arguments.length < 2) {
          if (len === 0) throw new TypeError("Reduce of empty array with no initial value");
          while (idx < len && !this.hasOwnProperty(idx)) idx++;
          if (idx >= len) throw new TypeError("Reduce of array with only holes");
          acc = this[idx];
          idx = idx + 1;
        }
        for (var i = idx; i < len; i++) {
          if (!this.hasOwnProperty(i)) continue;
          var el = this[i];
          acc = fn(acc, el, i, this);
        }
        return acc;
      };
      Array.prototype.every = function (fn) {
        var len = this.length;
        for (var i = 0; i < len; i++) {
          if (!this.hasOwnProperty(i)) continue;
          var el = this[i];
          if (!fn(el, i, this)) return false;
        }
        return true;
      };
      Array.prototype.some = function (fn) {
        var len = this.length;
        for (var i = 0; i < len; i++) {
          if (!this.hasOwnProperty(i)) continue;
          var el = this[i];
          if (fn(el, i, this)) return true;
        }
        return false;
      };
    "#;
    let mut lexer = Lexer::new(prelude);
    if let Ok(tokens) = lexer.tokenize_all() {
      let mut parser = Parser::new(tokens, prelude);
      if let Ok(stmts) = parser.parse_program() {
        Self::hoist_declarations(&stmts, env);
        let mut interp = Interpreter {
          global: Arc::clone(env),
          // 预置走 AstOnly, 这张表只是结构体占位, 不会被桥接层读到
          interp_globals: GlobalsTable::default(),
          call_stack: Vec::new(),
          // 内建预置引导必须走 AST: 此时全局作用域尚未建完, 字节码桥会反向依赖它
          mode: ExecMode::AstOnly,
          stats: ExecStats::default(),
          array_proto_watch: HashSet::default(),
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
    let mut parser = Parser::new(tokens, source);
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
    match try_execute_with_bytecode_in(stmts, Some(&self.global), &self.interp_globals) {
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
        Err(ExecSignal::Break(_)) => {
          return Err(Error::RuntimeError(
            "Illegal break outside of loop".to_string(),
          ));
        }
        Err(ExecSignal::Continue(_)) => {
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
    props.insert("name".to_string(), JsValue::String(error_name.into()));
    props.insert("message".to_string(), JsValue::String(message.into()));
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

  /// 取 receiver 自身上 `prefix + prop` 的 accessor 函数值
  ///
  /// 没有 accessor 的对象只付一次 `contains_key` (静态键, 零分配); 命中才拼键再查。
  /// 原型链上的 accessor 暂不触发: 判定"整条链有没有 accessor"需要 V8 那种 Map 级标志位,
  /// 本引擎值模型没有 shape (战役 C), 逐层探测会把每次属性读的一次哈希放大成 O(链长)
  fn lookup_accessor(obj: &JsValue, prop: &str, prefix: &str) -> Option<JsValue> {
    let JsValue::Object(map) = obj else {
      return None;
    };
    let guard = map.read();
    if !guard.contains_key(ACCESSOR_FLAG_KEY) {
      return None;
    }
    let key = format!("{prefix}{prop}");
    guard
      .get(&key)
      .filter(|v| matches!(v, JsValue::Function(_) | JsValue::Native(_)))
      .cloned()
  }

  /// Array 接收者 delete 口径 (与引擎 `delete_property` Array 臂同源): `length` 不可删;
  /// 下标键按规范留洞 —— 密 Vec 无 hole 位 (战役 C), 以 Undefined 填槽近似; 具名键回收 props 槽
  fn delete_array_slot(obj: &JsValue, key: &str) -> Option<bool> {
    let JsValue::Array(a) = obj else {
      return None;
    };
    if key == LENGTH_KEY {
      return Some(false);
    }
    let mut guard = a.write();
    if let Some(idx) = canonical_index(key) {
      if let Some(slot) = guard.get_mut(idx) {
        // 战役 C ③: 索引 delete 落洞哨兵 (读面物化 undefined, in/keys/for-in 即不存在),
        // 与引擎 delete_property 数组臂同口径
        *slot = JsValue::TheHole;
      }
    } else if let Some(props) = &mut guard.props {
      props.remove(key);
    }
    Some(true)
  }

  /// 对象是否登记过 accessor 哨兵 (非对象恒假, 零成本短路)
  #[inline]
  fn has_accessor(obj: &JsValue) -> bool {
    matches!(obj, JsValue::Object(map) if map.read().contains_key(ACCESSOR_FLAG_KEY))
  }

  /// 内建接收者对应的全局构造函数名, 决定属性读回落到哪张原型表
  #[inline]
  fn builtin_ctor_name(val: &JsValue) -> Option<&'static str> {
    match val {
      JsValue::Array(_) => Some("Array"),
      JsValue::String(_) => Some("String"),
      JsValue::Boolean(_) => Some("Boolean"),
      JsValue::Number(_) => Some("Number"),
      _ => None,
    }
  }

  /// 内建构造函数 prototype 上的同名属性: 内建接收者的方法面回落, 未命中给 None
  ///
  /// `JsValue::Array` 是裸 `Vec`, 存不下 `__proto__`, 硬编码表只覆盖能纯 `&self` 实现的读写;
  /// `map`/`filter`/`reduce` 这些要回调用户函数的由预置脚本注入 `Array.prototype`, 只能在读点
  /// 按名回溯 (与字节码链 `get_property_resolved` 同构)。只在未命中路径调用, 命中路径零开销
  fn proto_prop(&self, ctor: &str, prop: &str) -> Option<JsValue> {
    let proto = self.global.get(ctor)?.get_property(PROTOTYPE_KEY).ok()?;
    if matches!(proto, JsValue::Undefined) {
      return None;
    }
    // 原型对象的 `__proto__` 已接 Object.prototype, 一次 get_property 走完整链
    proto
      .get_property(prop)
      .ok()
      .filter(|v| !matches!(v, JsValue::Undefined))
  }

  /// 写侧记账 (#24): 名字命中内建表且接收者正是当前 `Array.prototype` 时登记 (名单与
  /// 字节码链共用 `BytecodeDispatcher::is_array_builtin_name` 一处真相)
  fn watch_array_proto(&mut self, recv: &Arc<RwLock<HashMap<String, JsValue>>>, prop: &str) {
    if !BytecodeDispatcher::is_array_builtin_name(prop) || self.array_proto_watch.contains(prop) {
      return;
    }
    let Some(ctor) = self.global.get("Array") else {
      return;
    };
    if let Ok(JsValue::Object(p)) = ctor.get_property(PROTOTYPE_KEY)
      && Arc::ptr_eq(recv, &p)
    {
      self.array_proto_watch.insert(prop.to_string());
    }
  }

  /// 属性读点: 先试 own accessor, 再走值模型, 内建接收者未命中时回落自身原型
  fn eval_get_property(&mut self, receiver: &JsValue, prop: &str) -> Result<JsValue> {
    if let Some(getter) = Self::lookup_accessor(receiver, prop, GETTER_PREFIX) {
      return self.invoke_function(&getter, Some(receiver.clone()), &[]);
    }
    let val = receiver.get_property(prop)?;
    // #24: 内建替身命中但该名字在 Array.prototype 上被动过 → 让位给原型回落 (空集零开销);
    // 原型上也没有 (被删) 时给 Undefined, 不得回到替身 —— node 的 delete 是永久移除
    let shadowed = !self.array_proto_watch.is_empty()
      && matches!(val, JsValue::Native(_))
      && matches!(receiver, JsValue::Array(_))
      && self.array_proto_watch.contains(prop);
    if matches!(val, JsValue::Undefined) || shadowed {
      if let Some(ctor) = Self::builtin_ctor_name(receiver)
        && let Some(hit) = self.proto_prop(ctor, prop)
      {
        return Ok(hit);
      }
      if shadowed {
        return Ok(JsValue::Undefined);
      }
    }
    Ok(val)
  }

  /// 属性写点: 命中 `@@set:prop` 时调用 setter, 数据属性不落表
  fn eval_set_property(&mut self, receiver: &JsValue, prop: &str, val: JsValue) -> Result<JsValue> {
    if let Some(setter) = Self::lookup_accessor(receiver, prop, SETTER_PREFIX) {
      self.invoke_function(&setter, Some(receiver.clone()), from_ref(&val))?;
      return Ok(val);
    }
    // #24: 写侧记账 (非内建名一次长度位判定就出局)
    if let JsValue::Object(o) = receiver {
      self.watch_array_proto(o, prop);
    }
    receiver.set_property(prop, val.clone())?;
    Ok(val)
  }

  /// 键控读 (`o[k]`): 哨兵命中才归一键并试 accessor, 未命中回落到原路径
  ///
  /// 先查哨兵再算 `element_key`: 数字键的 `element_key` 要格式化字符串, 放哨兵前面会让
  /// 每个无 accessor 的对象键控读多付一次 String 分配 (且 get_element 内部还要再算一遍)
  fn eval_get_element(&mut self, obj: &JsValue, idx: &JsValue) -> Result<JsValue> {
    if Self::has_accessor(obj) {
      let key = element_key(idx);
      if let Some(getter) = Self::lookup_accessor(obj, &key, GETTER_PREFIX) {
        return self.invoke_function(&getter, Some(obj.clone()), &[]);
      }
    }
    let val = obj.get_element(idx)?;
    // #24: 键控读同口径 —— 替身命中且名字被监视时让位原型, 原型缺失(被删)给 Undefined
    let shadowed = !self.array_proto_watch.is_empty()
      && matches!(val, JsValue::Native(_))
      && matches!(obj, JsValue::Array(_))
      && matches!(idx, JsValue::String(_))
      && self.array_proto_watch.contains(&idx.to_js_string());
    // 回溯原型前先筛掉两种必然不中的形状, 判定都不分配: 普通对象没有原型方法面 (否则每个
    // miss 都要付一次 `element_key` 的 String 分配 + 一次全局表哈希), 数组的数值下标读的是
    // 元素槽 —— 字节码链 `GetKeyedProperty` 的 `fast_elem_index` 快路径同样不回溯, 一侧回溯
    // 一侧不回溯就是两链分裂 (`Array.prototype[0] = x` 后 `[][0]`: node 给 x, 两链一律给
    // undefined, 属已知偏差)
    if (matches!(val, JsValue::Undefined) || shadowed)
      && let Some(ctor) = Self::builtin_ctor_name(obj)
      && !matches!((obj, idx), (JsValue::Array(_), JsValue::Number(n)) if is_canonical_index(*n))
    {
      let key = element_key(idx);
      if let Some(hit) = self.proto_prop(ctor, &key) {
        return Ok(hit);
      }
      if shadowed {
        return Ok(JsValue::Undefined);
      }
    }
    Ok(val)
  }

  /// 键控写 (`o[k] = v`) 的 accessor 版, 键算时机与 `eval_get_element` 同侧
  fn eval_set_element(&mut self, obj: &JsValue, idx: &JsValue, val: JsValue) -> Result<JsValue> {
    if Self::has_accessor(obj) {
      let key = element_key(idx);
      if let Some(setter) = Self::lookup_accessor(obj, &key, SETTER_PREFIX) {
        self.invoke_function(&setter, Some(obj.clone()), from_ref(&val))?;
        return Ok(val);
      }
    }
    obj.set_element(idx, val.clone())?;
    // #24: 计算键写原型方法同样记账 (键为字符串时才可能命中内建名)
    if let (JsValue::Object(o), JsValue::String(s)) = (obj, idx) {
      self.watch_array_proto(o, s);
    }
    Ok(val)
  }

  fn eval_to_cow_str<'a>(
    &mut self,
    val: &'a JsValue,
    _env: &Arc<Environment>,
  ) -> Result<Cow<'a, str>> {
    match val {
      JsValue::String(s) => Ok(Cow::Borrowed(&**s)),
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
        // 整值且 |x| < 1e15: 该区间内格式层必走定点, itoa 输出与它逐字节一致, 省一次 dtoa;
        // 其余交给 v8-numbers, Rust 的 `to_string` 不切科学计数法 (1e-7 会印成 0.0000001)
        if num.fract() == 0.0 && num.abs() < 1e15 {
          let mut buf = Buffer::new();
          Ok(Cow::Owned(buf.format(num as i64).to_string()))
        } else {
          Ok(Cow::Owned(double_to_string(num)))
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
      // 洞漏进强转面即物化 undefined (读面汇合口径, 不该到达此处, 到也不炸)
      JsValue::TheHole => Ok(JsValue::Undefined),
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
        // Native 内建没有 Function.prototype 面物化 (12.27 挂账): 回环耗尽没找到可调用方法时
        // 走与 `String(f)` 同一份 to_js_string 摘要 —— node 口径给源码/摘要而非 TypeError (门禁 78)
        if matches!(val, JsValue::Function(_) | JsValue::Native(_)) {
          return Ok(JsValue::String(val.to_js_string().into()));
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
      Ok(JsValue::String(out.into()))
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
      return Ok(JsValue::String(out.into()));
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

      Stmt::While { cond, body } => self.eval_while(&[], cond, body, env),

      Stmt::For {
        init,
        cond,
        step,
        body,
      } => self.eval_for(&[], init, cond, step, body, env),

      Stmt::FunctionDecl(_) => Ok(JsValue::Undefined),

      Stmt::Return(expr) => {
        let val = if let Some(e) = expr {
          self.eval_expr(e, env)?
        } else {
          JsValue::Undefined
        };
        Err(ExecSignal::Return(val))
      }

      Stmt::Break(label) => Err(ExecSignal::Break(label.clone())),
      Stmt::Continue(label) => Err(ExecSignal::Continue(label.clone())),
      Stmt::Labeled { labels, body } => self.eval_labeled(labels, body, env),

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
                  Error::RangeError(msg) => self.create_error("RangeError", msg, env),
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

      Stmt::DoWhile { body, cond } => self.eval_do_while(&[], body, cond, env),

      Stmt::ForIn {
        variable,
        is_var: _,
        object,
        body,
      } => self.eval_for_in(&[], variable, object, body, env),

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
                Err(ExecSignal::Break(None)) => return Ok(last),
                Err(ExecSignal::Continue(c)) => return Err(ExecSignal::Continue(c)),
                Err(e) => return Err(e),
              }
            }
          }
        }
        Ok(last)
      }

      Stmt::Empty => Ok(JsValue::Undefined),

      // 解构/for-of 暂态在 parse_program/parse_block 出口已 desugar 成普通语句,
      // 执行链看到它即解析旁路 bug (报错而非 panic)
      Stmt::VarPattern { .. } | Stmt::ForOf { .. } => Err(ExecSignal::from(Error::ParseError(
        "destructuring must be desugared by parser".to_string(),
      ))),
    }
  }

  /// 标记语句执行: 体是迭代语句则把标签交给循环臂按名消费 (continue L 必须回到循环推进点);
  /// 其余形态跑完体后吞掉指着自己的 break L (= 跳到语句末尾), 裸/他名信号照常外传
  fn eval_labeled(
    &mut self,
    labels: &[String],
    body: &Stmt,
    env: &Arc<Environment>,
  ) -> StdResult<JsValue, ExecSignal> {
    let r = match body {
      Stmt::While { cond, body } => self.eval_while(labels, cond, body, env),
      Stmt::DoWhile { body, cond } => self.eval_do_while(labels, body, cond, env),
      Stmt::For {
        init,
        cond,
        step,
        body,
      } => self.eval_for(labels, init, cond, step, body, env),
      Stmt::ForIn {
        variable,
        object,
        body,
        ..
      } => self.eval_for_in(labels, variable, object, body, env),
      _ => self.eval_stmt(body, env),
    };
    match r {
      Err(ExecSignal::Break(Some(l))) if labels.contains(&l) => Ok(JsValue::Undefined),
      other => other,
    }
  }

  fn eval_while(
    &mut self,
    labels: &[String],
    cond: &Expr,
    body: &Stmt,
    env: &Arc<Environment>,
  ) -> StdResult<JsValue, ExecSignal> {
    let mut last = JsValue::Undefined;
    loop {
      let c = self.eval_expr(cond, env)?;
      if !c.to_boolean() {
        break;
      }
      match self.eval_stmt(body, env) {
        Ok(v) => last = v,
        Err(ExecSignal::Break(None)) => break,
        Err(ExecSignal::Break(Some(l))) if labels.contains(&l) => break,
        Err(ExecSignal::Continue(None)) => continue,
        Err(ExecSignal::Continue(Some(l))) if labels.contains(&l) => continue,
        Err(e) => return Err(e),
      }
    }
    Ok(last)
  }

  fn eval_do_while(
    &mut self,
    labels: &[String],
    body: &Stmt,
    cond: &Expr,
    env: &Arc<Environment>,
  ) -> StdResult<JsValue, ExecSignal> {
    let mut last = JsValue::Undefined;
    loop {
      match self.eval_stmt(body, env) {
        Ok(v) => last = v,
        Err(ExecSignal::Break(None)) => break,
        Err(ExecSignal::Break(Some(l))) if labels.contains(&l) => break,
        Err(ExecSignal::Continue(None)) => {}
        Err(ExecSignal::Continue(Some(l))) if labels.contains(&l) => {}
        Err(e) => return Err(e),
      }
      if !self.eval_expr(cond, env)?.to_boolean() {
        break;
      }
    }
    Ok(last)
  }

  fn eval_for(
    &mut self,
    labels: &[String],
    init: &Option<Box<Stmt>>,
    cond: &Option<Expr>,
    step: &Option<Expr>,
    body: &Stmt,
    env: &Arc<Environment>,
  ) -> StdResult<JsValue, ExecSignal> {
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
        Err(ExecSignal::Break(None)) => break,
        Err(ExecSignal::Break(Some(l))) if labels.contains(&l) => break,
        Err(ExecSignal::Continue(None)) => {}
        Err(ExecSignal::Continue(Some(l))) if labels.contains(&l) => {}
        Err(e) => return Err(e),
      }
      if let Some(s) = step {
        self.eval_expr(s, env)?;
      }
    }
    Ok(last)
  }

  fn eval_for_in(
    &mut self,
    labels: &[String],
    variable: &str,
    object: &Expr,
    body: &Stmt,
    env: &Arc<Environment>,
  ) -> StdResult<JsValue, ExecSignal> {
    let obj_val = self.eval_expr(object, env)?;
    let keys = match obj_val {
      JsValue::Object(o) => {
        let guard = o.read();
        enumerable_names(guard.keys())
          .into_iter()
          .map(str::to_string)
          .collect()
      }
      JsValue::Array(a) => {
        let guard = a.read();
        let len = guard.len();
        let mut keys = Vec::with_capacity(len);
        let mut buf = Buffer::new();
        for i in 0..len {
          // 战役 C ③: 洞槽不进 for-in (与引擎 ForInEnumerate 数组臂同口径)
          if matches!(guard[i], JsValue::TheHole) {
            continue;
          }
          keys.push(buf.format(i).to_string());
        }
        keys
      }
      // 字符串接收者按索引枚举 (与引擎 `ForInEnumerate` String 臂同口径)
      JsValue::String(s) => {
        let mut keys = Vec::with_capacity(s.chars().count());
        let mut buf = Buffer::new();
        for i in 0..s.chars().count() {
          keys.push(buf.format(i).to_string());
        }
        keys
      }
      // 正则接收者只枚举 expando/遮蔽自有键 (与引擎 `ForInEnumerate` 伪键过滤同口径,
      // node: 标准正则交空, `r.foo=1` 后吐 ["foo"]; lastIndex 不可枚举)
      JsValue::RegExp(re) => re.enumerable_own_names(),
      _ => Vec::new(),
    };
    let mut last = JsValue::Undefined;
    for k in keys {
      if !env.set(variable, JsValue::String(k.clone().into())) {
        env.define(variable.to_owned(), JsValue::String(k.into()));
      }
      match self.eval_stmt(body, env) {
        Ok(v) => last = v,
        Err(ExecSignal::Break(None)) => break,
        Err(ExecSignal::Break(Some(l))) if labels.contains(&l) => break,
        Err(ExecSignal::Continue(None)) => continue,
        Err(ExecSignal::Continue(Some(l))) if labels.contains(&l) => continue,
        Err(e) => return Err(e),
      }
    }
    Ok(last)
  }

  fn eval_expr(&mut self, expr: &Expr, env: &Arc<Environment>) -> Result<JsValue> {
    match expr {
      Expr::Literal(val) => Ok(val.clone()),

      // C-④a per-eval 新实例: 编译体走 RegExpCore 缓存, 每次求值只付元数据价
      Expr::RegExp { pattern, flags } => JsValue::new_regexp(pattern, flags),

      Expr::AssignPattern { .. } => Err(Error::ParseError(
        "destructuring assignment must be desugared by parser".to_string(),
      )),

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
          JsValue::Undefined | JsValue::TheHole => "undefined",
          JsValue::Null => "object",
          JsValue::Boolean(_) => "boolean",
          JsValue::Number(_) => "number",
          JsValue::BigInt(_) => "bigint",
          JsValue::String(_) => "string",
          JsValue::Symbol(_) => "symbol",
          JsValue::Function(_) | JsValue::Native(_) => "function",
          JsValue::Object(_) | JsValue::Array(_) | JsValue::RegExp(_) => "object",
        };
        Ok(JsValue::String(type_str.into()))
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
              let old_val = self.eval_get_property(&obj, property)?;
              let old_num = old_val.to_number();
              let next = JsValue::Number(if is_inc { old_num + 1.0 } else { old_num - 1.0 });
              self.eval_set_property(&obj, property, next.clone())?;
              if is_pre { Ok(next) } else { Ok(old_val) }
            }
            Expr::Index { object, index } => {
              let obj = self.eval_expr(object, env)?;
              let idx_val = self.eval_expr(index, env)?;
              let old_val = self.eval_get_element(&obj, &idx_val)?;
              let old_num = old_val.to_number();
              let next = JsValue::Number(if is_inc { old_num + 1.0 } else { old_num - 1.0 });
              self.eval_set_element(&obj, &idx_val, next.clone())?;
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
            // Array 接收者走引擎 `delete_property` 同口径 (length 不可删, 下标留洞)
            if let Some(handled) = Self::delete_array_slot(&obj, property) {
              return Ok(JsValue::Boolean(handled));
            }
            if let JsValue::Object(o) = &obj {
              // #24: 删原型上的内建名要让替身失能, 记账放在释放写锁之后
              let watch = o.clone();
              let mut guard = o.write();
              guard.remove(property.as_str());
              // 与被删键成对的 accessor 伪键一并回收, 否则删完 getter 仍读得出旧值
              if guard.contains_key(ACCESSOR_FLAG_KEY) {
                for prefix in [GETTER_PREFIX, SETTER_PREFIX] {
                  guard.remove(&format!("{prefix}{property}"));
                }
              }
              drop(guard);
              self.watch_array_proto(&watch, property.as_str());
            }
            // 正则 expando 自有键回收 (node e12: delete r.bar 后名册只剩 lastIndex)
            if let JsValue::RegExp(re) = &obj {
              re.props.write().remove(property.as_str());
            }
            // sloppy 删除恒 true (缺失/继承键亦然), 与引擎 `delete_property` 同口径
            Ok(JsValue::Boolean(true))
          }
          Expr::Index { object, index } => {
            let obj = self.eval_expr(object, env)?;
            let idx_val = self.eval_expr(index, env)?;
            // 键归一必须与读写点共用 element_key: 原先数字走 itoa(n as i64)、符号走 Display,
            // 和写入键 (1.5 → "1.5"、Symbol → "@@symbol:x") 不是同一个键, delete 静默失效
            let key = element_key(&idx_val);
            if let Some(handled) = Self::delete_array_slot(&obj, &key) {
              return Ok(JsValue::Boolean(handled));
            }
            if let JsValue::Object(o) = &obj {
              let watch = o.clone();
              let mut guard = o.write();
              guard.remove(&key);
              if guard.contains_key(ACCESSOR_FLAG_KEY) {
                for prefix in [GETTER_PREFIX, SETTER_PREFIX] {
                  guard.remove(&format!("{prefix}{key}"));
                }
              }
              drop(guard);
              self.watch_array_proto(&watch, &key);
            }
            // 正则 expando 自有键回收 (与 Member 臂同口径)
            if let JsValue::RegExp(re) = &obj {
              re.props.write().remove(&key);
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
              Ok(JsValue::String(out.into()))
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
                    // accessor 只以伪键落表, `in` 判存在要把 `@@get:k`/`@@set:k` 也算上
                    if guard.contains_key(ACCESSOR_FLAG_KEY)
                      && [GETTER_PREFIX, SETTER_PREFIX]
                        .iter()
                        .any(|prefix| guard.contains_key(&format!("{prefix}{key}")))
                    {
                      return Ok(JsValue::Boolean(true));
                    }
                    curr = guard.get(PROTO_KEY).cloned();
                  }
                  JsValue::Function(func) => {
                    let guard = func.properties.read();
                    // 箭头不继承 Function.prototype 的 prototype 面 (node: `'prototype' in fn` 假)
                    let proto_hit = key == PROTOTYPE_KEY && !func.shared.is_arrow;
                    if guard.contains_key(&key) || key == "length" || key == "name" || proto_hit {
                      return Ok(JsValue::Boolean(true));
                    }
                    drop(guard);
                    // 自有面 miss 后按属性读同一口径问原型链 (node: `in` 沿链,
                    // "toString" in decl 为真; #23 ⑤ 的源码替身就在这张面上)
                    if !matches!(func.get_property(&key), Err(_) | Ok(JsValue::Undefined)) {
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
              if key == LENGTH_KEY {
                return Ok(JsValue::Boolean(true));
              }
              // 元素槽就是下标键的全部真相: 命中即真, 越界即假, **不回溯 Array.prototype** ——
              // 读侧 `a[i]` 同口径筛除 (eval_get_element), 一侧回溯一侧不回溯会让 `(7 in a)` 真而
              // `a[7]` undefined, 属引擎自相矛盾。字符串键仍按内建面与原型链找 (`"map" in [1]` 真),
              // 与字节码链 `has_property_resolved` 的数组臂同口径, 分歧即两链分裂
              if let Some(idx) = canonical_index(&key) {
                let guard = arr.read();
                // 战役 C ③: 洞槽即不存在 (node `1 in [1,,2]` 假), 与读侧物化分册
                return Ok(JsValue::Boolean(
                  idx < guard.len() && !matches!(guard[idx], JsValue::TheHole),
                ));
              }
              // 内建接收者的方法挂在原型上, `"map" in [1]` 为真; #24: 被监视的名字内建替身
              // 不得抢先答 true (node: `delete Array.prototype.map` 后为假), 真相只看原型
              Ok(JsValue::Boolean(
                (JsValue::array_builtin(arr, &key).is_some()
                  && !self.array_proto_watch.contains(&key))
                  || self.proto_prop("Array", &key).is_some(),
              ))
            }
            JsValue::RegExp(re) => {
              // 存在判定与属性读同一张面 (node: 标志位/source/flags/lastIndex 与 exec/test
              // 都是原型访问器, `in` 为真; match/replace/split 不在面上为假) —— 早先这里
              // 直接落 `_` 抛 TypeError
              let key = l.to_string();
              let present = !matches!(
                JsValue::RegExp(Arc::clone(re)).get_property(&key),
                Err(_) | Ok(JsValue::Undefined)
              );
              Ok(JsValue::Boolean(present))
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
              let cur = self.eval_get_property(&obj, property)?;
              self.apply_assign_op(&cur, op, &rval, env)?
            };
            // eval_set_property 把写入值原样带回 (赋值表达式要产出它), 省掉一次克隆
            self.eval_set_property(&obj, property, final_val)
          }
          Expr::Index { object, index } => {
            let obj = self.eval_expr(object, env)?;
            let idx_val = self.eval_expr(index, env)?;
            let final_val = if *op == AssignOp::Assign {
              rval
            } else {
              let cur = self.eval_get_element(&obj, &idx_val)?;
              self.apply_assign_op(&cur, op, &rval, env)?
            };
            self.eval_set_element(&obj, &idx_val, final_val)
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
        self.eval_get_property(&obj, property).map_err(|e| {
          Error::RuntimeError(format!(
            "{e} at '{property}' (stack: {:?})",
            self.call_stack
          ))
        })
      }

      Expr::Index { object, index } => {
        let obj = self.eval_expr(object, env)?;
        let idx = self.eval_expr(index, env)?;
        self.eval_get_element(&obj, &idx).map_err(|e| {
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
                real_args = arr.read().elems.clone();
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
            let f = self.eval_get_property(&obj, property).map_err(|e| {
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
            let f = self.eval_get_element(&obj, &idx).map_err(|e| {
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
        map.reserve(entries.len() + 2);
        if let Some(obj_proto) = env.get_object_prototype() {
          map.insert(PROTO_KEY.to_string(), obj_proto);
        }
        // 哨兵键先行 (与 bytecode_bridge 的 Expr::Object 同序): 构建期的覆盖判定要看得到它
        if entries.iter().any(|p| !matches!(p.kind, PropKind::Data)) {
          map.insert(ACCESSOR_FLAG_KEY.to_string(), JsValue::Boolean(true));
        }
        for p in entries {
          let key_str = match &p.key {
            ObjectKey::Ident(s) => s.clone(),
            ObjectKey::Computed(expr) => {
              let k_val = self.eval_expr(expr, env)?;
              match k_val {
                JsValue::Symbol(s) => format_symbol_key(&s),
                other => other.to_js_string(),
              }
            }
          };
          let val = self.eval_expr(&p.value, env)?;
          // 字面量构建期是 CreateDataProperty: 同名后定义整体覆盖先前定义, 规则与字节码链的
          // define_own_entry 逐条对应 —— 数据键摘掉同名伪键对, 伪键摘掉同名数据键。没有 accessor
          // 的字面量只多付一次哨兵哈希, 不为它拼前缀串
          if matches!(p.kind, PropKind::Data) {
            if map.contains_key(ACCESSOR_FLAG_KEY) {
              map.remove(&format_getter_key(&key_str));
              map.remove(&format_setter_key(&key_str));
            }
            map.insert(key_str, val);
          } else {
            map.remove(&key_str);
            let stored = match p.kind {
              PropKind::Get => format_getter_key(&key_str),
              _ => format_setter_key(&key_str),
            };
            map.insert(stored, val);
          }
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
        // 箭头函数没有 [[Construct]] (无 prototype 语义), node 口径 TypeError
        if func_data.shared.is_arrow {
          return Err(Error::TypeError(format!(
            "{} is not a constructor (stack: {:?})",
            func_data.name().unwrap_or("arrow"),
            self.call_stack
          )));
        }
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
    arr: &Arc<RwLock<ArrayStore>>,
    prop: &str,
    args: &[Expr],
    env: &Arc<Environment>,
  ) -> Option<Result<JsValue>> {
    // #24: 内联表是原生替身的另一份拷贝 —— 名字被监视时让位正常取点 (空集一次哈希短路)
    if !self.array_proto_watch.is_empty() && self.array_proto_watch.contains(prop) {
      return None;
    }
    match prop {
      "push" | "pop" | "shift" | "reverse" | "splice" | "slice" | "indexOf" | "join"
      | "toString" => Some(self.exec_inline_array_call(arr, prop, args, env)),
      _ => None,
    }
  }

  fn exec_inline_array_call(
    &mut self,
    arr: &Arc<RwLock<ArrayStore>>,
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
    let is_arrow = func_data.shared.is_arrow;
    // 箭头不吃调用点 this: 定格词法定义链上的 this (原版 JSFunction.context 同一路径)
    let this_opt = if is_arrow {
      func_data.closure.get("this")
    } else {
      this_opt
    };
    // arguments 同理不自绑, 沿词法链回溯外层函数的绑定
    let args_val = if func_data.shared.uses_arguments && !is_arrow {
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
      // rest 形参吃自身调用实参尾段快照 (箭头同理, 与词法 arguments 互不相干, 12.40)
      rest_param: func_data
        .shared
        .rest_param
        .as_ref()
        .map(|(name, start)| (name.as_str(), evaluated_args.get(*start..).unwrap_or(&[]))),
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
        Err(ExecSignal::Break(_) | ExecSignal::Continue(_)) => {
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
      // sort 的比较器是用户函数, NativeFn::call 只有 &self 拿不到执行态, 故在此就地排序。
      // 规范: 比较器只接受 undefined 或缺省, 其余不可调用值一律 TypeError
      JsValue::Native(NativeFn::ArraySort(arr)) => match evaluated_args.first() {
        None | Some(JsValue::Undefined) => self.array_sort(arr, None),
        Some(cb) if cb.is_callable() => self.array_sort(arr, Some(cb)),
        Some(other) => Err(Error::TypeError(format!(
          "sort: comparator must be either a function or undefined: {}",
          other.to_js_string()
        ))),
      },
      // find 的回调同理: 表里只放标记, 在持有执行态的调用点就地迭代
      JsValue::Native(NativeFn::ArrayFind(arr)) => {
        let callback = match evaluated_args.first() {
          Some(cb) if cb.is_callable() => cb.clone(),
          Some(other) => {
            return Err(Error::TypeError(format!(
              "find: callback must be a function: {}",
              other.to_js_string()
            )));
          }
          None => {
            return Err(Error::TypeError(
              "find: expects a callback function".to_string(),
            ));
          }
        };
        let this_arg = evaluated_args.get(1).cloned().unwrap_or(JsValue::Undefined);
        let receiver = JsValue::Array(Arc::clone(arr));
        let len = arr.read().len();
        for i in 0..len {
          // 每步现取一个元素 (与字节码链同款): 回调里改写同下标不得影响后续交出
          let element = arr.read().get(i).cloned().unwrap_or(JsValue::Undefined);
          if self
            .invoke_function(
              &callback,
              Some(this_arg.clone()),
              &[element.clone(), JsValue::Number(i as f64), receiver.clone()],
            )?
            .to_boolean()
          {
            return Ok(element);
          }
        }
        Ok(JsValue::Undefined)
      }
      // findIndex 与 find 同一遍历截获, 只把命中换成下标 (未命中 -1)
      JsValue::Native(NativeFn::ArrayFindIndex(arr)) => {
        let callback = match evaluated_args.first() {
          Some(cb) if cb.is_callable() => cb.clone(),
          Some(other) => {
            return Err(Error::TypeError(format!(
              "findIndex: callback must be a function: {}",
              other.to_js_string()
            )));
          }
          None => {
            return Err(Error::TypeError(
              "findIndex: expects a callback function".to_string(),
            ));
          }
        };
        let this_arg = evaluated_args.get(1).cloned().unwrap_or(JsValue::Undefined);
        let receiver = JsValue::Array(Arc::clone(arr));
        let len = arr.read().len();
        for i in 0..len {
          let element = arr.read().get(i).cloned().unwrap_or(JsValue::Undefined);
          if self
            .invoke_function(
              &callback,
              Some(this_arg.clone()),
              &[element, JsValue::Number(i as f64), receiver.clone()],
            )?
            .to_boolean()
          {
            return Ok(JsValue::Number(i as f64));
          }
        }
        Ok(JsValue::Number(-1.0))
      }
      // reduceRight 截获臂: 与 reduce 同款累子规则 (缺省取尾元素作累子、显式 undefined 照参),
      // 反向遍历; 每步现取元素与 find 同口径
      JsValue::Native(NativeFn::ArrayReduceRight(arr)) => {
        let callback = match evaluated_args.first() {
          Some(cb) if cb.is_callable() => cb.clone(),
          Some(other) => {
            return Err(Error::TypeError(format!(
              "reduceRight: callback must be a function: {}",
              other.to_js_string()
            )));
          }
          None => {
            return Err(Error::TypeError(
              "reduceRight: expects a callback function".to_string(),
            ));
          }
        };
        let receiver = JsValue::Array(Arc::clone(arr));
        let len = arr.read().len();
        let (mut acc, top) = match evaluated_args.get(1) {
          Some(initial) => (initial.clone(), len),
          None => {
            if len == 0 {
              return Err(Error::TypeError(
                "Reduce of empty array with no initial value".to_string(),
              ));
            }
            (
              arr
                .read()
                .get(len - 1)
                .cloned()
                .unwrap_or(JsValue::Undefined),
              len - 1,
            )
          }
        };
        for i in (0..top).rev() {
          let element = arr.read().get(i).cloned().unwrap_or(JsValue::Undefined);
          acc = self.invoke_function(
            &callback,
            None,
            &[acc, element, JsValue::Number(i as f64), receiver.clone()],
          )?;
        }
        Ok(acc)
      }
      // flatMap 截获臂: 回调结果只压平一层
      JsValue::Native(NativeFn::ArrayFlatMap(arr)) => {
        let callback = match evaluated_args.first() {
          Some(cb) if cb.is_callable() => cb.clone(),
          Some(other) => {
            return Err(Error::TypeError(format!(
              "flatMap: callback must be a function: {}",
              other.to_js_string()
            )));
          }
          None => {
            return Err(Error::TypeError(
              "flatMap: expects a callback function".to_string(),
            ));
          }
        };
        let this_arg = evaluated_args.get(1).cloned().unwrap_or(JsValue::Undefined);
        let receiver = JsValue::Array(Arc::clone(arr));
        let len = arr.read().len();
        let mut out = Vec::with_capacity(len);
        for i in 0..len {
          let element = arr.read().get(i).cloned().unwrap_or(JsValue::Undefined);
          let ret = self.invoke_function(
            &callback,
            Some(this_arg.clone()),
            &[element, JsValue::Number(i as f64), receiver.clone()],
          )?;
          if let JsValue::Array(inner) = &ret {
            out.extend(inner.read().iter().cloned());
          } else {
            out.push(ret);
          }
        }
        Ok(JsValue::new_array(out))
      }
      // replace 的函数替换实参同理 (22.1.3.25: 回调实参 = 全匹配, 组..., offset, 全串)
      JsValue::Native(NativeFn::StringReplace(s))
        if evaluated_args.get(1).is_some_and(|v| v.is_callable()) =>
      {
        self.string_replace_with_callback(s, evaluated_args)
      }
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

  /// `String.replace` 的函数回调版 (AST 链, #23): 逐命中回调, 与 `NativeFn::StringReplace`
  /// 的串替换路径同一循环口径 (字节推进、空匹配 +1、非全局只换第一处)
  fn string_replace_with_callback(&mut self, s: &str, args: &[JsValue]) -> Result<JsValue> {
    let pattern = &args[0];
    let cb = &args[1];
    let text_val = JsValue::String(s.into());
    let mut out = String::new();
    let mut last_end = 0usize;
    match pattern {
      JsValue::RegExp(re) => {
        let global = re.core.global;
        let mut cur = 0usize;
        loop {
          if cur > s.len() {
            break;
          }
          let caps_opt = if global {
            re.core.compiled.captures_from_pos(s, cur)
          } else {
            re.core.compiled.captures(s)
          };
          let caps = match caps_opt {
            Ok(Some(c)) => c,
            _ => break,
          };
          let Some(whole) = caps.get(0) else { break };
          let (ms, me) = (whole.start(), whole.end());
          if ms >= last_end {
            out.push_str(&s[last_end..ms]);
          }
          let mut cb_args: Vec<JsValue> = Vec::with_capacity(caps.len() + 2);
          cb_args.push(JsValue::String(s[ms..me].into()));
          for g in caps.iter().skip(1) {
            cb_args.push(
              g.map(|m| JsValue::String(m.as_str().to_string().into()))
                .unwrap_or(JsValue::Undefined),
            );
          }
          cb_args.push(JsValue::Number(char_count(&s[..ms]) as f64));
          cb_args.push(text_val.clone());
          let piece = self
            .invoke_function(cb, Some(JsValue::Undefined), &cb_args)?
            .to_js_string();
          out.push_str(&piece);
          last_end = me;
          if !global {
            break;
          }
          cur = if me == ms { ms + 1 } else { me.max(cur + 1) };
        }
      }
      _ => {
        let pat_str = pattern.to_js_string();
        if let Some(pos) = s.find(&pat_str) {
          out.push_str(&s[..pos]);
          let cb_args = [
            JsValue::String(s[pos..pos + pat_str.len()].to_string().into()),
            JsValue::Number(char_count(&s[..pos]) as f64),
            text_val,
          ];
          out.push_str(
            &self
              .invoke_function(cb, Some(JsValue::Undefined), &cb_args)?
              .to_js_string(),
          );
          last_end = pos + pat_str.len();
        }
      }
    }
    out.push_str(&s[last_end.min(s.len())..]);
    Ok(JsValue::String(out.into()))
  }

  /// Array.prototype.sort: 就地排序后返回接收者本身
  ///
  /// 无比较器按 ToString 的码点序比较 (undefined 恒排末尾且不参与比较); 有比较器时逐项回调用户
  /// 函数。比较器抛错没法中断 Rust 的稳定排序, 因此记录首个错误、排序返回后再传播 ——
  /// 规范本就允许数组停留在半排序状态
  fn array_sort(
    &mut self,
    arr: &Arc<RwLock<ArrayStore>>,
    comparator: Option<&JsValue>,
  ) -> Result<JsValue> {
    // 排序过程中会回调用户代码, 先脱锁取快照, 免得重入写同一张表自堵
    let (mut sorted, trailing) = {
      let guard = arr.read();
      let defined: Vec<JsValue> = guard
        .iter()
        .filter(|v| !matches!(v, JsValue::Undefined))
        .cloned()
        .collect();
      let skipped = guard.len() - defined.len();
      (defined, skipped)
    };
    match comparator {
      // 缓存键版: `sort_by_key` 不保证每个元素只求键一次 (标准库文档明确"可能被多次调用"),
      // 默认路线每次比较都要现算 ToString 并分配 String; 与字节码链路同用 cached 版
      None => sorted.sort_by_cached_key(|a| a.to_js_string()),
      Some(cb) => {
        let cb = cb.clone();
        let mut failure: Option<Error> = None;
        sorted.sort_by(|a, b| {
          if failure.is_some() {
            return Ordering::Equal;
          }
          match self.invoke_function(&cb, None, &[a.clone(), b.clone()]) {
            Ok(v) => v.to_number().partial_cmp(&0.0).unwrap_or(Ordering::Equal),
            Err(e) => {
              failure = Some(e);
              Ordering::Equal
            }
          }
        });
        if let Some(err) = failure {
          return Err(err);
        }
      }
    }
    sorted.extend(repeat_n(JsValue::Undefined, trailing));
    arr.write().elems = sorted;
    Ok(JsValue::Array(Arc::clone(arr)))
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
        Err(ExecSignal::Break(_) | ExecSignal::Continue(_)) => {
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

/// 执行一段完整的 JavaScript 源码, 走 Interpreter 的默认模式 (当前为 BytecodeFirst)
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

/// 强制走 AST 解释器: 双链路差分门禁用, 字节码链路能跑的构造这里也必须能跑
pub fn run_ast(source: &str) -> Result<JsValue> {
  run_with(source, ExecMode::AstOnly)
}
