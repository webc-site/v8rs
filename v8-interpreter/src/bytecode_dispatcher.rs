use std::{
  borrow::Cow,
  cmp::Ordering,
  fmt,
  mem::{replace, take},
  ptr::fn_addr_eq,
  sync::{Arc, LazyLock, OnceLock},
};

use gxhash::{HashMap, HashMapExt, HashSet};
use itoa::Buffer as ItoaBuffer;
use parking_lot::{
  RawRwLock, RwLock, RwLockReadGuard, RwLockWriteGuard,
  lock_api::{MappedRwLockReadGuard, MappedRwLockWriteGuard},
};
use v8_numbers::{
  conversions::{
    ConversionFlag, MAX_FRACTION_DIGITS, double_to_exponential, double_to_fixed,
    double_to_precision, double_to_radix, double_to_string, string_to_double,
  },
  double_to_int32, double_to_uint32,
};

use crate::{
  bytecode_array_builder::{BytecodeArray, ConstantPoolEntry, InterpFunction},
  bytecode_array_iterator::BytecodeArrayIterator,
  bytecode_register::Register,
  bytecodes::{Bytecode, Bytecodes},
  error::{Error, Result},
  regexp::{RegExpCore, regex_exec_native, regex_split_core, regex_test_native},
};

static FUNCTION_CALL: LazyLock<Value> = LazyLock::new(|| Value::new_native(NativeFn::FunctionCall));
static FUNCTION_APPLY: LazyLock<Value> =
  LazyLock::new(|| Value::new_native(NativeFn::FunctionApply));

pub const PROTO_KEY: &str = "__proto__";
pub const PROTOTYPE_KEY: &str = "prototype";
pub const CONSTRUCTOR_KEY: &str = "constructor";
pub const LENGTH_KEY: &str = "length";
pub const NAME_KEY: &str = "name";
pub const CALL_KEY: &str = "call";
pub const APPLY_KEY: &str = "apply";
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
/// ES2015+ 字符串面 (12.26 补齐): 键名一处定义, 两链 get_property 表共用
pub const REPEAT_KEY: &str = "repeat";
pub const PAD_START_KEY: &str = "padStart";
pub const PAD_END_KEY: &str = "padEnd";
pub const AT_KEY: &str = "at";
pub const CODE_POINT_AT_KEY: &str = "codePointAt";
pub const FROM_CODE_POINT_KEY: &str = "fromCodePoint";
/// ES 字符串面第三轮 (12.27): 与上组同款, 键名一处定义、两链 get_property 表共用
pub const TRIM_KEY: &str = "trim";
pub const TRIM_START_KEY: &str = "trimStart";
pub const TRIM_END_KEY: &str = "trimEnd";
/// 历史别名 (附录 B): node 仍保留 trimLeft/trimRight, 与 trimStart/trimEnd 同一实现
pub const TRIM_LEFT_KEY: &str = "trimLeft";
pub const TRIM_RIGHT_KEY: &str = "trimRight";
pub const STARTS_WITH_KEY: &str = "startsWith";
pub const ENDS_WITH_KEY: &str = "endsWith";
pub const REPLACE_ALL_KEY: &str = "replaceAll";
/// Array 面第三轮 (12.27): at 复用 AT_KEY, concat 复用 CONCAT_KEY
pub const FLAT_KEY: &str = "flat";
pub const FLAT_MAP_KEY: &str = "flatMap";
pub const REDUCE_RIGHT_KEY: &str = "reduceRight";
pub const COPY_WITHIN_KEY: &str = "copyWithin";
pub const VALUE_OF_KEY: &str = "valueOf";
pub const TO_STRING_KEY: &str = "toString";
pub const TO_FIXED_KEY: &str = "toFixed";
pub const TO_PRECISION_KEY: &str = "toPrecision";
pub const TO_EXPONENTIAL_KEY: &str = "toExponential";
/// Date 实例的时间戳槽位键。战役 C 值模型合一后由 JSDate 的 [[DateValue]] 内部槽取代
pub const TIMESTAMP_KEY: &str = "__timestamp__";
/// Date 实例的时间兼容槽位键, 与 TIMESTAMP_KEY 成对写入
pub const TIME_KEY: &str = "time";
pub const GLOBAL_KEY: &str = "global";
pub const LAST_INDEX_KEY: &str = "lastIndex";
pub const SOURCE_KEY: &str = "source";
pub const FLAGS_KEY: &str = "flags";
pub const INDEX_KEY: &str = "index";
pub const INPUT_KEY: &str = "input";
/// 环境对象里指向外层环境的隐藏键 (与 `__` 前缀约定一致, 枚举时已被过滤)
pub const CONTEXT_PARENT_KEY: &str = "__context_parent__";
/// Getter 伪键前缀
pub const GETTER_PREFIX: &str = "@@get:";
/// Setter 伪键前缀
pub const SETTER_PREFIX: &str = "@@set:";
/// accessor 哨兵键
///
/// 对象只要登记过 get/set 就写入它; 属性读点据此做一次哈希判定即可确定"这个对象没有
/// accessor", 不必逐键扫描 —— 绝大多数对象没有 accessor, 这条路径在属性读的热路径上
pub const ACCESSOR_FLAG_KEY: &str = "@@accessor";

/// 内建方法名的单一真相: 两链的属性表与 `in` 判定共用 (宿主 rs8 的 `JsValue::array_builtin`
/// 从这里 `pub use`, 不再各抄一份字面量 —— 与 `@@get:` 前缀键同源的同款问题)
pub const PUSH_KEY: &str = "push";
pub const POP_KEY: &str = "pop";
pub const SHIFT_KEY: &str = "shift";
pub const UNSHIFT_KEY: &str = "unshift";
pub const SLICE_KEY: &str = "slice";
pub const JOIN_KEY: &str = "join";
pub const INDEX_OF_KEY: &str = "indexOf";
pub const LAST_INDEX_OF_KEY: &str = "lastIndexOf";
pub const FILL_KEY: &str = "fill";
pub const FIND_KEY: &str = "find";
pub const FIND_INDEX_KEY: &str = "findIndex";
pub const MAP_KEY: &str = "map";
pub const FOR_EACH_KEY: &str = "forEach";
pub const FILTER_KEY: &str = "filter";
pub const REDUCE_KEY: &str = "reduce";
pub const EVERY_KEY: &str = "every";
pub const SOME_KEY: &str = "some";
pub const SPLICE_KEY: &str = "splice";
pub const REVERSE_KEY: &str = "reverse";
pub const SORT_KEY: &str = "sort";
pub const INCLUDES_KEY: &str = "includes";
pub const CONCAT_KEY: &str = "concat";
pub const SUBSTR_KEY: &str = "substr";
pub const HAS_OWN_PROPERTY_KEY: &str = "hasOwnProperty";
pub const PROPERTY_IS_ENUMERABLE_KEY: &str = "propertyIsEnumerable";
pub const MESSAGE_KEY: &str = "message";
/// 符号内部前缀 (与 `@@get:`/`@@set:` 同一套伪键约定)
pub const SYMBOL_PREFIX: &str = "@@symbol:";
/// 符号作属性键时的编码 (一处定义, 宿主 `element_key` 与引擎 `to_element_key` 共读)
#[must_use]
pub fn format_symbol_key(desc: &str) -> String {
  let mut s = String::with_capacity(SYMBOL_PREFIX.len() + desc.len());
  s.push_str(SYMBOL_PREFIX);
  s.push_str(desc);
  s
}
/// Symbol.toPrimitive 内部属性键
pub const SYM_TO_PRIMITIVE: &str = "@@symbol:Symbol.toPrimitive";
/// 内部伪键 (`@@get:` / `@@set:` / `@@symbol:` / `@@accessor`), 不参与枚举与序列化
///
/// 按 `@@` 双 @ 判定而不是单 `@`: 用户完全可以有 `@foo` 这样的普通键, 单 @ 会把它误杀
#[inline]
pub fn is_internal_key(k: &str) -> bool {
  k.starts_with("@@")
}

/// 正则实例的只读访问器面 (node: `source`/`flags`/六布尔是 RegExp.prototype 上 getter-only
/// 访问器): 赋值静默忽略。引擎 `Value::RegExp` set 臂与宿主 `JsValue::set_property` RegExp 臂
/// 委托同一张名册 (一处定义两链共读); `exec`/`test`/`toString` 可被袋键遮蔽、`lastIndex`
/// 专存内核, 都不在此列
#[inline]
pub fn is_regexp_readonly_key(k: &str) -> bool {
  matches!(
    k,
    SOURCE_KEY
      | FLAGS_KEY
      | GLOBAL_KEY
      | "ignoreCase"
      | "multiline"
      | "sticky"
      | "unicode"
      | "dotAll"
  )
}

/// RegExp 值的 `hasOwnProperty` 内建: 自有面 = `lastIndex` 恒自有 + expando 袋
/// (node 口径与宿主 `RegExpData::own_property_names` 同册; 派生访问器与 exec/test/toString
/// 住 RegExp.prototype, 未遮蔽则非自有)。非正则接收者退化为各自属性表的 contains
/// (仅作 `.call` 挪用的兜底)
fn regexp_bag_has_own(receiver: &Value, args: &[Value]) -> Result<Value> {
  let key = args
    .first()
    .map(|v| v.to_str_lossy().into_owned())
    .unwrap_or_default();
  let own = match receiver {
    Value::RegExp(_, props) => key == LAST_INDEX_KEY || props.read().contains_key(key.as_str()),
    Value::Object(o) => o.read().contains_key(key.as_str()),
    Value::Array(a) => a
      .read()
      .props
      .as_ref()
      .is_some_and(|p| p.contains_key(key.as_str())),
    Value::Function(c) => c.props.read().contains_key(key.as_str()),
    Value::Native(_, p) => p.read().contains_key(key.as_str()),
    _ => false,
  };
  Ok(Value::Boolean(own))
}

/// `propertyIsEnumerable`: 只有袋键可枚举 (lastIndex 自有但不可枚举, 与宿主 `enumerable_own_names`
/// 同口径; 派生名不住袋, 天然 false)
fn regexp_bag_prop_enumerable(receiver: &Value, args: &[Value]) -> Result<Value> {
  let key = args
    .first()
    .map(|v| v.to_str_lossy().into_owned())
    .unwrap_or_default();
  let enumerable = match receiver {
    Value::RegExp(_, props) => props.read().contains_key(key.as_str()),
    other => {
      return regexp_bag_has_own(other, args);
    }
  };
  Ok(Value::Boolean(enumerable))
}

/// 伪键还原成可枚举属性名: `@@get:p`/`@@set:p` 归到 `p`, 其余内部键与 `__` 前缀槽位键不可枚举
///
/// 剥完前缀的剩余部分仍要过一次内部键过滤: 符号键 accessor 落表是 `@@get:@@symbol:x`,
/// 直接吐还原串会把 Symbol 内部键当普通属性名泄进 `Object.keys`/`for-in`/JSON
#[inline]
pub fn enumerable_name(k: &str) -> Option<&str> {
  match k
    .strip_prefix(GETTER_PREFIX)
    .or_else(|| k.strip_prefix(SETTER_PREFIX))
  {
    Some(rest) => (!is_internal_key(rest) && !rest.starts_with("__")).then_some(rest),
    None => (!is_internal_key(k) && !k.starts_with("__")).then_some(k),
  }
}

/// 自身可枚举属性名: 伪键还原后按名去重 (get/set 成对定义会在表里留两条同名键)
///
/// 两条链共用同一份规则, 否则 `Object.keys` 与 `for-in` 会给出不同的键集。
/// 分两趟收集: 普通键在哈希表里天然唯一, 不需要任何比对 (for-in/Object.keys 是热路径,
/// 全表 `out.contains` 去重会让大对象退化 O(n²)); 只有伪键还原名才可能与普通键或彼此撞名
pub fn enumerable_names<'a, I: Iterator<Item = &'a String>>(keys: I) -> Vec<&'a str> {
  let mut out: Vec<&str> = Vec::with_capacity(keys.size_hint().0);
  let mut accessors: Vec<&str> = Vec::new();
  for k in keys {
    if k.starts_with(GETTER_PREFIX) || k.starts_with(SETTER_PREFIX) {
      if let Some(rest) = enumerable_name(k) {
        accessors.push(rest);
      }
    } else if enumerable_name(k).is_some() {
      out.push(k);
    }
  }
  for name in accessors {
    if !out.contains(&name) {
      out.push(name);
    }
  }
  out
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

/// 字面量构建期的自有属性定义 —— 原版 kDefineNamedOwnPropertyInLiteral /
/// kDefineKeyedOwnProperty 的 CreateDataProperty 语义: **直接落数据, 绝不打 setter**,
/// 且同名后定义整体覆盖先前定义
///
/// 两条覆盖方向都要处理, 因为 accessor 在表里是伪键而不是描述符:
/// - 数据键覆盖 accessor (`{ get y(){ return 1; }, y: 2 }` 在 node 读 2, 伪键不摘就还是 1)
/// - accessor 覆盖数据键 (`{ z: 3, get z(){ return 4; }` 读 4; 读侧本就优先伪键, 这里把数据键
///   摘掉是让键集与 `delete` 后的状态也和 AST 链同源)
///
/// 哨兵键保留: 摘它要全表扫 `@@get:`/`@@set:`, 代价远大于此后每次属性读多付的一次哈希
fn define_own_entry(table: &mut HashMap<String, Value>, key: &str, val: Value) {
  if let Some(name) = key
    .strip_prefix(GETTER_PREFIX)
    .or_else(|| key.strip_prefix(SETTER_PREFIX))
  {
    table.remove(name);
  } else if table.contains_key(ACCESSOR_FLAG_KEY) {
    table.remove(&format_getter_key(key));
    table.remove(&format_setter_key(key));
  }
  table.insert(key.to_string(), val);
}

/// 全局对象上兜底原型链的构造函数名
const OBJECT_CTOR: &str = "Object";
/// 数组接收者自带原型对象的构造函数名
const ARRAY_CTOR: &str = "Array";

/// 内建值对应的构造函数名, 供原型回溯定位其自带原型对象
fn builtin_ctor_name(val: &Value) -> Option<&'static str> {
  match val {
    Value::Array(_) => Some(ARRAY_CTOR),
    Value::String(_) => Some("String"),
    Value::Boolean(_) => Some("Boolean"),
    Value::Smi(_) | Value::Number(_) => Some("Number"),
    _ => None,
  }
}

/// 字符索引转字节偏移 (字符边界安全，杜绝切片 panic)
#[inline]
pub fn char_to_byte_index(s: &str, char_idx: usize) -> usize {
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
pub fn char_count(s: &str) -> usize {
  if s.is_ascii() {
    s.len()
  } else {
    s.chars().count()
  }
}

/// 字符串切片三法种类 (node 实测三种负索引/交换规则互斥, 借用任何一方语义都是 bug:
/// substring 负与 NaN 一律夹 0 且 start>end 交换; slice 负从尾部起算不交换;
/// substr 第二参是长度而非终点, Annex B)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SliceKind {
  Substring,
  Slice,
  Substr,
}

/// 切片实参 → 整数索引 (ToIntegerOrInfinity 的饱和化): NaN→0, +∞→i64::MAX,
/// -∞→i64::MIN, 其余向零截断。to_i32 类转换会把 ∞ 折成 0, 令 `s.substring(2, Infinity)`
/// 与 `substring(2, -Infinity)` 双双走错交换臂 (node 分别给 "cdef"/"ab")
#[must_use]
#[inline]
pub fn to_index_i64(x: f64) -> i64 {
  if x.is_nan() {
    0
  } else if x == f64::INFINITY {
    i64::MAX
  } else if x == f64::NEG_INFINITY {
    i64::MIN
  } else {
    x as i64
  }
}

/// 切片窗口单一真相, 两链共走 (宿主 value.rs 与引擎臂此前各抄一份且已漂移 ——
/// substring 曾借 slice 的负索引从尾算法): 入参为 ToInteger 饱和化后的 i64,
/// `end=None` 表示实参缺席, 返回 (from_char, to_char) 且保证 from <= to
#[must_use]
pub fn slice_window(kind: SliceKind, c_len: i64, start: i64, end: Option<i64>) -> (usize, usize) {
  let len = c_len.max(0) as usize;
  let wrap = |n: i64| {
    if n < 0 {
      c_len.saturating_add(n).max(0) as usize
    } else {
      (n as usize).min(len)
    }
  };
  match kind {
    SliceKind::Substring => {
      let s = start.clamp(0, c_len) as usize;
      let e = end.unwrap_or(c_len).clamp(0, c_len) as usize;
      if s <= e { (s, e) } else { (e, s) }
    }
    SliceKind::Slice => {
      let s = wrap(start);
      let e = end.map_or(len, wrap);
      if s < e { (s, e) } else { (e, e) }
    }
    SliceKind::Substr => {
      let s = wrap(start);
      let l = end.unwrap_or(c_len).max(0) as usize;
      (s, s.saturating_add(l).min(len))
    }
  }
}

/// Number → 十进制串: 转发 `v8-numbers` 的 conversions.cc 1:1 移植, 引擎侧不重复实现
/// (平局规则是 half-away-from-zero, 与 Rust `{:.f}` 的 round-half-even 差一位, 手写必然偏)
#[inline]
fn number_to_js_string(n: f64) -> String {
  double_to_string(n)
}

/// `repeat` 纯实现 (12.26 起两链共用一处定义): count 为 ToIntegerOrInfinity 饱和化后的
/// i64 (NaN→0 给空串); 负数或 Infinity (i64::MAX 哨兵) 由调用链折成 RangeError → None
#[must_use]
pub fn str_repeat(s: &str, count: i64) -> Option<String> {
  if count < 0 || count == i64::MAX {
    return None;
  }
  let mut out = String::with_capacity(s.len().saturating_mul(count as usize));
  for _ in 0..count {
    out.push_str(s);
  }
  Some(out)
}

/// `padStart`/`padEnd` 纯实现: width 为饱和化整数, `start` 选左/右填;
/// pad 空串或已够长原样返回; width=Infinity (i64::MAX) 折 RangeError → None
/// (node: "abc".padStart(Infinity) 抛)
#[must_use]
pub fn str_pad(s: &str, width: i64, pad: &str, start: bool) -> Option<String> {
  if width == i64::MAX {
    return None;
  }
  let chars: Vec<char> = s.chars().collect();
  if pad.is_empty() || width <= chars.len() as i64 {
    return Some(s.to_string());
  }
  let pad_chars: Vec<char> = pad.chars().collect();
  let mut fill: String = String::with_capacity(width as usize - chars.len());
  for i in 0..(width as usize - chars.len()) {
    fill.push(pad_chars[i % pad_chars.len()]);
  }
  if start {
    fill.push_str(s);
  } else {
    fill = s.to_string() + &fill;
  }
  Some(fill)
}

/// `includes` 纯实现: 码点口径 (与引擎 charAt/length 同一既有约定, astral 偏差见战役 C),
/// from 负/NaN 夹 0、超尾夹 len (node: `"abc".includes("", 4)` 真 —— 空针在夹取位总命中)
#[must_use]
pub fn str_includes(hay: &str, needle: &str, from: i64) -> bool {
  if from <= 0 {
    return hay.contains(needle);
  }
  let chars: Vec<char> = hay.chars().collect();
  let start = (from as usize).min(chars.len());
  let tail: String = chars[start..].iter().collect();
  tail.contains(needle)
}

/// `String.prototype.lastIndexOf` 纯实现: 返回 JS 口径位 (与 `.length`/indexOf 同款 BMP 码点近似,
/// astral 偏差同战役 C 记账)。非空针逐起点匹配, 窗口右界按 **UTF-16 码元**数推进并停在字符
/// 边界上 (JS length 口径, 且杜绝跨字符边界切片 panic)
#[must_use]
pub fn str_last_index_of(hay: &str, needle: &str, pos: i64) -> i32 {
  // node 实测: 空针交钳位后的起点 (负→0); 负 pos 只扫下标 0 (起点为针即 0, 否则 -1);
  // 正 pos 扫 [0, min(pos, len)] 取最后命中。与 indexOf/includes 的"从尾归一"语义不同
  let len = hay.chars().count() as i64;
  if needle.is_empty() {
    return pos.max(0).min(len) as i32;
  }
  if pos < 0 {
    return if hay.starts_with(needle) { 0 } else { -1 };
  }
  let pos = pos as usize;
  let n_cu = needle.chars().map(char::len_utf16).sum::<usize>();
  let hay_cu = hay.chars().map(char::len_utf16).sum::<usize>();
  // 合法起点上界: min(pos, len - needle) (node `lastIndexOf("lll",2)` 交 -1 —— 区间空)
  let last_start = pos.min(hay_cu.saturating_sub(n_cu));
  let mut found: i32 = -1;
  let mut start_b = 0usize; // 当前起点字节位
  // cu_at = 当前起点的 JS 位 (BMP 码点近似), 逐字符 +1 即 enumerate 序
  for (cu_at, first) in hay.chars().enumerate() {
    if cu_at > last_start {
      break; // 起点越过合法上界, 更右的起点不可能命中
    }
    let mut end = first.len_utf8();
    let mut cu = first.len_utf16();
    for c in hay[start_b + first.len_utf8()..].chars() {
      if cu >= n_cu {
        break;
      }
      end += c.len_utf8();
      cu += c.len_utf16();
    }
    if hay[start_b..start_b + end].starts_with(needle) {
      found = cu_at as i32;
    }
    start_b += first.len_utf8();
  }
  found
}

/// `at` 纯实现: 负索引从尾起算, 越界 → None (undefined)
#[must_use]
pub fn str_at(s: &str, idx: i64) -> Option<char> {
  let chars: Vec<char> = s.chars().collect();
  let i = if idx < 0 {
    chars.len() as i64 + idx
  } else {
    idx
  };
  if i < 0 || i as usize >= chars.len() {
    None
  } else {
    Some(chars[i as usize])
  }
}

/// `codePointAt` 纯实现: 码点口径, 越界 → None (node 给 undefined)
#[must_use]
pub fn str_code_point_at(s: &str, idx: i64) -> Option<u32> {
  str_at(s, idx).map(|c| c as u32)
}

/// `Object.is` 的 SameValue 语义 (Number 面上 NaN==NaN、+0≠-0), 非 Number 走类型化相等
#[must_use]
pub fn same_value_x64(x: f64, y: f64) -> bool {
  if x.is_nan() {
    return y.is_nan();
  }
  x == y && (x != 0.0 || x.is_sign_negative() == y.is_sign_negative())
}

/// trim 家族方向 (12.27): 变体只做 receiver 捕获, 行为在此一处定义 (StringPad 双 bool 同款先例)
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum TrimKind {
  Both,
  Start,
  End,
}

/// ES 规范的 WhiteSpace ∪ LineTerminator: Rust `is_whitespace` 是 Unicode White_Space 属性,
/// 不含 ZWNBSP 与行分隔符 U+2028/2029, 需按规范补齐 (NBSP/VT/FF/CR 该属性已含)
fn is_js_whitespace(ch: char) -> bool {
  ch.is_whitespace() || matches!(ch, '\u{FEFF}' | '\u{2028}' | '\u{2029}')
}

#[must_use]
pub fn str_trim(s: &str, kind: TrimKind) -> String {
  match kind {
    TrimKind::Both => s.trim_matches(is_js_whitespace).to_string(),
    TrimKind::Start => s.trim_start_matches(is_js_whitespace).to_string(),
    TrimKind::End => s.trim_end_matches(is_js_whitespace).to_string(),
  }
}

/// `startsWith` 纯实现: 码点口径 (战役 C 前记偏差), from 负/NaN 夹 0、超尾夹 len
/// (node: `"abc".startsWith("", 200)` 真 —— 空针在夹取位总命中)
#[must_use]
pub fn str_starts_with(hay: &str, needle: &str, from: i64) -> bool {
  if from <= 0 {
    return hay.starts_with(needle);
  }
  let chars: Vec<char> = hay.chars().collect();
  let start = (from as usize).min(chars.len());
  chars[start..]
    .iter()
    .collect::<String>()
    .starts_with(needle)
}

/// `endsWith` 纯实现: 第二参是**截断末尾** (与 startsWith 的前移口径不同),
/// 负/NaN 夹 0、超尾夹 len (node: `"abc".endsWith("", 200)` 真)
#[must_use]
pub fn str_ends_with(hay: &str, needle: &str, pos: i64) -> bool {
  let chars: Vec<char> = hay.chars().collect();
  let end = if pos < 0 {
    0
  } else {
    (pos as usize).min(chars.len())
  };
  chars[..end].iter().collect::<String>().ends_with(needle)
}

/// `replaceAll` 字符串搜索面: 逐次命中, $ 序列与 replace 同口径 (字符串模式无捕获组, $n 恒字面);
/// 空搜索串在每个码点前后都插入 (node: `"abc".replaceAll("", "-")` → "-a-b-c-")
#[must_use]
pub fn str_replace_all(hay: &str, search: &str, repl: &str) -> String {
  let mut out = String::with_capacity(hay.len());
  if search.is_empty() {
    // 空针在每个字符前各插一次替换, 原文照旧保留, 末尾再补一次
    for (b, c) in hay.char_indices() {
      out.push_str(&get_substitution(repl, "", &hay[..b], &hay[b..], &[]));
      out.push(c);
    }
    out.push_str(&get_substitution(repl, "", hay, "", &[]));
    return out;
  }
  let mut cursor = 0usize;
  while let Some(off) = hay[cursor..].find(search) {
    let start = cursor + off;
    let end = start + search.len();
    out.push_str(&hay[cursor..start]);
    out.push_str(&get_substitution(
      repl,
      search,
      &hay[..start],
      &hay[end..],
      &[],
    ));
    cursor = end;
  }
  out.push_str(&hay[cursor..]);
  out
}

/// URI 组件非保留字 (encodeURIComponent 额外直通): 字母数字 + `-_.!~*'()`
const URI_UNRESERVED_EXTRA: &[u8] = b"-_.!~*'()";
/// encodeURI 再直通的分隔符 (ES 19.2.6.3 reservedUnescaped)
const URI_RESERVED: &[u8] = b";/?:@&=+$,#[]";

/// `encodeURI(Component)`: 非保留字符原样, 其余按 UTF-8 逐字节大写 %XX (12.27 一处定义,
/// `is_uri` 位选 encodeURI 的宽直通集)
#[must_use]
pub fn uri_encode(s: &str, is_uri: bool) -> String {
  let mut out = String::with_capacity(s.len());
  for ch in s.chars() {
    // 直通判定必须先限 ASCII: `ch as u8` 会把 U+202D 之类折成 '-' 误直通
    let pass = ch.is_ascii()
      && (ch.is_ascii_alphanumeric()
        || URI_UNRESERVED_EXTRA.contains(&(ch as u8))
        || (is_uri && URI_RESERVED.contains(&(ch as u8))));
    if pass {
      out.push(ch);
    } else {
      for b in ch.encode_utf8(&mut [0u8; 4]).as_bytes() {
        out.push_str(&format!("%{b:02X}"));
      }
    }
  }
  out
}

/// `decodeURI(Component)`: %XX 串按 UTF-8 组字节再解码, 非法转义/残缺多字节 → Err;
/// decodeURI 对保留字的合法转义**保持原文** (node: `decodeURI("c%3Dd")` 仍是 "%3D")
/// (战役 C 前记偏差: astral 按码点而非 UTF-16 代理对)
pub fn uri_decode(s: &str, is_uri: bool) -> Option<String> {
  let bytes = s.as_bytes();
  let mut out: Vec<u8> = Vec::with_capacity(s.len());
  let mut i = 0usize;
  while i < bytes.len() {
    if bytes[i] != b'%' {
      out.push(bytes[i]);
      i += 1;
      continue;
    }
    let v = hex_byte(bytes, i)?;
    if is_uri && URI_RESERVED.contains(&v) {
      out.extend_from_slice(&bytes[i..i + 3]);
    } else {
      out.push(v);
    }
    i += 3;
  }
  String::from_utf8(out).ok()
}

/// 读取 `%XX` 一字节; 缺位或非十六进制 → None (hex_byte 的 "%%" 判定由上层循环走到时自然失败)
fn hex_byte(bytes: &[u8], at: usize) -> Option<u8> {
  let hi = *bytes.get(at + 1)?;
  let lo = *bytes.get(at + 2)?;
  let h = hi.to_ascii_uppercase();
  let l = lo.to_ascii_uppercase();
  // '0'..='9' → 0-9; 'A'..='F' → 10-15
  let hv = match h {
    b'0'..=b'9' => h - b'0',
    b'A'..=b'F' => h - b'A' + 10,
    _ => return None,
  };
  let lv = match l {
    b'0'..=b'9' => l - b'0',
    b'A'..=b'F' => l - b'A' + 10,
    _ => return None,
  };
  Some(hv << 4 | lv)
}

/// `flat` 的递归压平: depth>0 时数组元素展开并降深度, 其余原样入列 (12.27)
fn flatten_into(elems: &[Value], depth: i64, out: &mut Vec<Value>) {
  for e in elems {
    if depth > 0
      && let Value::Array(inner) = e
    {
      flatten_into(&read_elems(inner), depth - 1, out);
    } else {
      out.push(e.clone());
    }
  }
}

/// 原生上下文状态载荷 (零 dyn 泛型擦除，极速函数指针分派)
pub struct NativeContext {
  payload: *const (),
  call_fn: fn(*const (), &Value, &[Value]) -> Result<Value>,
  drop_fn: fn(*const ()),
}

/// 原生回调签名 (带载荷 `T` 的函数指针, 零 dyn 分派)
pub type ContextFn<T> = fn(&T, &Value, &[Value]) -> Result<Value>;

unsafe impl Send for NativeContext {}
unsafe impl Sync for NativeContext {}

impl Drop for NativeContext {
  fn drop(&mut self) {
    (self.drop_fn)(self.payload);
  }
}

/// 需要回调用户函数的数组迭代方法: 与 `ArraySort` 同一类 (回调要执行态, `NativeFn::call`
/// 只有 `&self`), 表里放标记, 实际在 `call_function` 里就地跑
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum ArrayIterKind {
  Map,
  ForEach,
  Filter,
  Reduce,
  Every,
  Some,
  Find,
  FindIndex,
  /// reduceRight 与 reduce 同款回调, 只是自尾反向; flatMap 回调后压平一层
  ReduceRight,
  FlatMap,
}

impl ArrayIterKind {
  /// 方法名: 内建表登记与错误信息共用一处
  const fn name(self) -> &'static str {
    match self {
      Self::Map => MAP_KEY,
      Self::ForEach => FOR_EACH_KEY,
      Self::Filter => FILTER_KEY,
      Self::Reduce => REDUCE_KEY,
      Self::Every => EVERY_KEY,
      Self::Some => SOME_KEY,
      Self::Find => FIND_KEY,
      Self::FindIndex => FIND_INDEX_KEY,
      Self::ReduceRight => REDUCE_RIGHT_KEY,
      Self::FlatMap => FLAT_MAP_KEY,
    }
  }
}

/// 原生函数句柄枚举分发 (纯 Enum Dispatch，杜绝 dyn 虚表与闭包堆分配)
#[derive(Clone)]
pub enum NativeFn {
  Ptr(fn(&Value, &[Value]) -> Result<Value>),
  Noop,
  FunctionCall,
  FunctionApply,
  ArrayPush(Arc<RwLock<ArrayData>>),
  ArrayPop(Arc<RwLock<ArrayData>>),
  ArrayShift(Arc<RwLock<ArrayData>>),
  ArraySlice(Arc<RwLock<ArrayData>>),
  ArrayJoin(Arc<RwLock<ArrayData>>),
  ArrayIndexOf(Arc<RwLock<ArrayData>>),
  ArrayLastIndexOf(Arc<RwLock<ArrayData>>),
  ArrayFill(Arc<RwLock<ArrayData>>),
  ArrayUnshift(Arc<RwLock<ArrayData>>),
  ArraySplice(Arc<RwLock<ArrayData>>),
  ArrayReverse(Arc<RwLock<ArrayData>>),
  ArrayIncludes(Arc<RwLock<ArrayData>>),
  ArrayConcat(Arc<RwLock<ArrayData>>),
  /// 排序需要回调比较器, 只能在 `call_function` 里截获 (对标 FunctionCall/FunctionApply 标记)
  ArraySort(Arc<RwLock<ArrayData>>),
  /// map/forEach/filter/reduce/every/some 的回调同上, 六者共用一个变体按 kind 分派
  ArrayIter(ArrayIterKind, Arc<RwLock<ArrayData>>),
  StringCharCodeAt(Arc<str>),
  StringCharAt(Arc<str>),
  StringIndexOf(Arc<str>),
  /// 第八轮 (12.32): 行为在 `str_last_index_of` 纯函数一处定义, 宿主复用同一份
  StringLastIndexOf(Arc<str>),
  /// substring: 负数/NaN 一律夹 0, start>end 交换 (与 slice 语义不同, 三分立变体)
  StringSubstring(Arc<str>),
  /// slice: 负数从尾部起算, 不交换
  StringSlice(Arc<str>),
  /// substr(start, length): 负 start 从尾部起算, 第二参是长度 (Annex B)
  StringSubstr(Arc<str>),
  StringSplit(Arc<str>),
  StringReplace(Arc<str>),
  StringMatch(Arc<str>),
  StringToLowerCase(Arc<str>),
  StringToUpperCase(Arc<str>),
  /// includes/repeat/padStart/padEnd/at/codePointAt (12.26): 行为在 str_* 纯函数一处定义,
  /// 变体只做 receiver 捕获; bool 位选 padStart(左)/padEnd(右)
  StringIncludes(Arc<str>),
  StringRepeat(Arc<str>),
  StringPad(Arc<str>, bool),
  StringAt(Arc<str>),
  StringCodePointAt(Arc<str>),
  /// 字符串面第三轮 (12.27): 行为在 str_trim/str_starts_with/str_ends_with/str_replace_all
  /// 纯函数一处定义; replaceAll 的正则面在 call 臂分流 (非 g 标志 → TypeError)
  StringTrim(Arc<str>, TrimKind),
  StringStartsWith(Arc<str>),
  StringEndsWith(Arc<str>),
  StringReplaceAll(Arc<str>),
  StringConcat(Arc<str>),
  /// Array 面第三轮: at/flat/copyWithin 无回调直接跑, reduceRight/flatMap 走 ArrayIter 截获
  ArrayAt(Arc<RwLock<ArrayData>>),
  ArrayFlat(Arc<RwLock<ArrayData>>),
  ArrayCopyWithin(Arc<RwLock<ArrayData>>),
  NumberToString(f64),
  NumberToFixed(f64),
  NumberToPrecision(f64),
  NumberToExponential(f64),
  ConstString(Arc<str>),
  /// 装箱原始值 (new Boolean/new Number/new String) 的 valueOf: 原样回吐预置的原始值
  Const(Box<Value>),
  Context(Arc<NativeContext>),
}

impl PartialEq for NativeFn {
  fn eq(&self, other: &Self) -> bool {
    match (self, other) {
      (Self::Ptr(a), Self::Ptr(b)) => fn_addr_eq(*a, *b),
      (Self::Noop, Self::Noop) => true,
      (Self::FunctionCall, Self::FunctionCall) => true,
      (Self::FunctionApply, Self::FunctionApply) => true,
      (Self::NumberToString(a), Self::NumberToString(b))
      | (Self::NumberToFixed(a), Self::NumberToFixed(b))
      | (Self::NumberToPrecision(a), Self::NumberToPrecision(b))
      | (Self::NumberToExponential(a), Self::NumberToExponential(b)) => a == b,
      (Self::ConstString(a), Self::ConstString(b)) => a == b,
      (Self::Const(a), Self::Const(b)) => a == b,
      (Self::ArrayPush(a), Self::ArrayPush(b))
      | (Self::ArrayPop(a), Self::ArrayPop(b))
      | (Self::ArrayShift(a), Self::ArrayShift(b))
      | (Self::ArraySlice(a), Self::ArraySlice(b))
      | (Self::ArrayJoin(a), Self::ArrayJoin(b))
      | (Self::ArraySplice(a), Self::ArraySplice(b))
      | (Self::ArrayReverse(a), Self::ArrayReverse(b))
      | (Self::ArrayIncludes(a), Self::ArrayIncludes(b))
      | (Self::ArrayConcat(a), Self::ArrayConcat(b))
      | (Self::ArraySort(a), Self::ArraySort(b))
      | (Self::ArrayIndexOf(a), Self::ArrayIndexOf(b))
      | (Self::ArrayLastIndexOf(a), Self::ArrayLastIndexOf(b))
      | (Self::ArrayFill(a), Self::ArrayFill(b))
      | (Self::ArrayUnshift(a), Self::ArrayUnshift(b))
      | (Self::ArrayAt(a), Self::ArrayAt(b))
      | (Self::ArrayFlat(a), Self::ArrayFlat(b))
      | (Self::ArrayCopyWithin(a), Self::ArrayCopyWithin(b)) => Arc::ptr_eq(a, b),
      (Self::ArrayIter(ka, a), Self::ArrayIter(kb, b)) => ka == kb && Arc::ptr_eq(a, b),
      (Self::StringCharCodeAt(a), Self::StringCharCodeAt(b))
      | (Self::StringCharAt(a), Self::StringCharAt(b))
      | (Self::StringIndexOf(a), Self::StringIndexOf(b))
      | (Self::StringLastIndexOf(a), Self::StringLastIndexOf(b))
      | (Self::StringSubstring(a), Self::StringSubstring(b))
      | (Self::StringSlice(a), Self::StringSlice(b))
      | (Self::StringSubstr(a), Self::StringSubstr(b))
      | (Self::StringIncludes(a), Self::StringIncludes(b))
      | (Self::StringRepeat(a), Self::StringRepeat(b))
      | (Self::StringAt(a), Self::StringAt(b))
      | (Self::StringCodePointAt(a), Self::StringCodePointAt(b))
      | (Self::StringSplit(a), Self::StringSplit(b))
      | (Self::StringReplace(a), Self::StringReplace(b))
      | (Self::StringMatch(a), Self::StringMatch(b))
      | (Self::StringToLowerCase(a), Self::StringToLowerCase(b))
      | (Self::StringStartsWith(a), Self::StringStartsWith(b))
      | (Self::StringEndsWith(a), Self::StringEndsWith(b))
      | (Self::StringReplaceAll(a), Self::StringReplaceAll(b))
      | (Self::StringConcat(a), Self::StringConcat(b))
      | (Self::StringToUpperCase(a), Self::StringToUpperCase(b)) => a == b,
      (Self::StringPad(a, sa), Self::StringPad(b, sb)) => sa == sb && a == b,
      (Self::StringTrim(a, ka), Self::StringTrim(b, kb)) => ka == kb && a == b,
      (Self::Context(a), Self::Context(b)) => Arc::ptr_eq(a, b),
      _ => false,
    }
  }
}

impl NativeFn {
  /// 携带上下文的原生回调; `receiver` 是调用点的 this (对标原版从激活记录取 receiver)
  ///
  pub fn from_context<T: Send + Sync + 'static>(ctx: T, func: ContextFn<T>) -> Self {
    struct Payload<T> {
      ctx: T,
      func: ContextFn<T>,
    }
    let boxed = Box::into_raw(Box::new(Payload { ctx, func })) as *const ();
    fn call_trampoline<T>(raw: *const (), receiver: &Value, args: &[Value]) -> Result<Value> {
      let p = unsafe { &*(raw as *const Payload<T>) };
      (p.func)(&p.ctx, receiver, args)
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

  pub fn call(&self, receiver: &Value, args: &[Value]) -> Result<Value> {
    match self {
      Self::Ptr(f) => f(receiver, args),
      Self::Noop | Self::FunctionCall | Self::FunctionApply => Ok(Value::Undefined),
      Self::ConstString(s) => Ok(Value::new_string(s.clone())),
      Self::Const(v) => Ok(v.as_ref().clone()),
      Self::NumberToString(n) => {
        // 只有 undefined (含缺省) 退成十进制; null/NaN/0 经 ToIntegerOrInfinity 落到 0,
        // 越界照样 RangeError (node: (5).toString(null) 抛, (5).toString(0) 抛)
        let radix = args
          .first()
          .filter(|v| !matches!(v, Value::Undefined))
          .map_or(10, |v| v.to_number() as i64);
        if !(2..=36).contains(&radix) {
          return Err(Error::ExecutionError(
            "RangeError: toString() radix argument must be between 2 and 36".to_string(),
          ));
        }
        if radix == 10 {
          return Ok(Value::new_string(number_to_js_string(*n)));
        }
        Ok(Value::new_string(double_to_radix(*n, radix as i32)))
      }
      Self::NumberToFixed(n) => {
        // digits 省略即 0; ToNumber 后 NaN→0, 越界 (含 ±Infinity) 抛 RangeError
        let f = args.first().map(|v| v.to_number() as i64).unwrap_or(0);
        if !(0..=MAX_FRACTION_DIGITS as i64).contains(&f) {
          return Err(Error::ExecutionError(
            "RangeError: toFixed() digits argument must be between 0 and 100".to_string(),
          ));
        }
        Ok(Value::new_string(double_to_fixed(*n, f as i32)))
      }
      Self::NumberToPrecision(n) => {
        // 与 toFixed 不同: 参数缺省 (或显式 undefined) 时按 ToString 走, 不是某个默认精度
        let Some(p) = args
          .first()
          .filter(|v| !matches!(v, Value::Undefined))
          .map(|v| v.to_number() as i64)
        else {
          return Ok(Value::new_string(number_to_js_string(*n)));
        };
        if !(1..=MAX_FRACTION_DIGITS as i64).contains(&p) {
          return Err(Error::ExecutionError(
            "RangeError: toPrecision() argument must be between 1 and 100".to_string(),
          ));
        }
        Ok(Value::new_string(double_to_precision(*n, p as i32)))
      }
      Self::NumberToExponential(n) => {
        // 与 toPrecision 同族: 参数缺省 (或显式 undefined) 时 f=-1 走最短数字; 越界抛 RangeError
        let Some(f) = args
          .first()
          .filter(|v| !matches!(v, Value::Undefined))
          .map(|v| v.to_number() as i64)
        else {
          return Ok(Value::new_string(double_to_exponential(*n, -1)));
        };
        if !(0..=MAX_FRACTION_DIGITS as i64).contains(&f) {
          return Err(Error::ExecutionError(
            "RangeError: toExponential() argument must be between 0 and 100".to_string(),
          ));
        }
        Ok(Value::new_string(double_to_exponential(*n, f as i32)))
      }
      Self::ArrayPush(arr) => {
        let mut guard = write_elems(arr);
        guard.extend(args.iter().cloned());
        Ok(Value::Smi(guard.len() as i32))
      }
      Self::ArrayPop(arr) => Ok(write_elems(arr).pop().unwrap_or(Value::Undefined)),
      Self::ArrayShift(arr) => {
        let mut guard = write_elems(arr);
        if guard.is_empty() {
          Ok(Value::Undefined)
        } else {
          Ok(guard.remove(0))
        }
      }
      Self::ArraySlice(arr) => {
        let guard = read_elems(arr);
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
        let guard = read_elems(arr);
        let mut res = String::new();
        for (i, v) in guard.iter().enumerate() {
          if i > 0 {
            res.push_str(&sep);
          }
          // ECMA-262 Array.prototype.join: null/undefined/hole 一律拼空串, 不走 ToString
          if !matches!(v, Value::Null | Value::Undefined | Value::TheHole) {
            res.push_str(&v.to_str_lossy());
          }
        }
        Ok(Value::new_string(res))
      }
      Self::ArrayIndexOf(arr) => {
        let target = args.first().unwrap_or(&Value::Undefined);
        let guard = read_elems(arr);
        let from_idx = args
          .get(1)
          .map(|v| tail_index(v, guard.len() as i64, 0))
          .unwrap_or(0) as usize;
        let idx = guard
          .iter()
          .enumerate()
          .skip(from_idx)
          .find(|(_, v)| v.test_equal_strict(target))
          .map(|(i, _)| i as i32)
          .unwrap_or(-1);
        Ok(Value::Smi(idx))
      }
      // 规范: lastIndexOf 用严格相等 (NaN 永不命中, 与 includes 的 SameValueZero 相区分);
      // fromIndex 为负从尾部起, 缺省 len-1
      Self::ArrayLastIndexOf(arr) => {
        let target = args.first().unwrap_or(&Value::Undefined);
        let guard = read_elems(arr);
        let len = guard.len();
        let from = match args.get(1) {
          Some(v) => {
            let n = v.to_i32() as i64;
            if n < 0 { len as i64 + n } else { n }
          }
          None => len as i64 - 1,
        };
        if from < 0 || len == 0 {
          return Ok(Value::Smi(-1));
        }
        let upper = (from as usize).min(len - 1);
        let idx = guard[..=upper]
          .iter()
          .rposition(|v| v.test_equal_strict(target))
          .map_or(-1, |i| i as i32);
        Ok(Value::Smi(idx))
      }
      // fill 是写点: start/end 走 tail_index 界归一 (undefined 缺省 0/len, NaN→0), 返回接收者本身
      Self::ArrayFill(arr) => {
        let value = args.first().cloned().unwrap_or(Value::Undefined);
        let mut guard = write_elems(arr);
        let len = guard.len() as i64;
        let start = args.get(1).map_or(0, |v| tail_index(v, len, 0));
        let end = args.get(2).map_or(len, |v| tail_index(v, len, len));
        if start < end {
          for slot in &mut guard[start as usize..end as usize] {
            *slot = value.clone();
          }
        }
        drop(guard);
        Ok(Value::Array(Arc::clone(arr)))
      }
      Self::ArrayUnshift(arr) => {
        let mut guard = write_elems(arr);
        if !args.is_empty() {
          guard.splice(..0, args.iter().cloned());
        }
        Ok(Value::Smi(guard.len() as i32))
      }
      // 变更型方法必须就地改这张表: 预置脚本里的 Array.prototype.splice 拿到的是接收者副本,
      // 改完即丢, 所以 战役 C 值模型合一之前变更型一律走引擎内建
      Self::ArraySplice(arr) => {
        let mut guard = write_elems(arr);
        let len = guard.len();
        let start = match args.first() {
          Some(s) => {
            let n = s.to_i32();
            if n < 0 {
              (len as i32 + n).max(0) as usize
            } else {
              (n as usize).min(len)
            }
          }
          None => 0,
        };
        let delete_count = match args.get(1) {
          Some(d) => {
            let n = d.to_i32();
            if n < 0 {
              0
            } else {
              (n as usize).min(len - start)
            }
          }
          None => len - start,
        };
        // 只有显式给了删除数才接受插入项: [1,2,3].splice(1) 是删到尾, 不是原地插入
        let inserted: Vec<Value> = if args.len() > 2 {
          args[2..].to_vec()
        } else {
          Vec::new()
        };
        let removed: Vec<Value> = guard
          .splice(start..start + delete_count, inserted)
          .collect();
        drop(guard);
        Ok(Value::new_array_with_elements(removed))
      }
      Self::ArrayReverse(arr) => {
        write_elems(arr).reverse();
        Ok(Value::Array(Arc::clone(arr)))
      }
      Self::ArrayIncludes(arr) => {
        // SameValueZero = 严格相等 + NaN 自等 (+0/-0 在 === 下本就相等)
        let target = args.first().unwrap_or(&Value::Undefined);
        let target_is_nan = matches!(target, Value::Number(n) if n.is_nan());
        // 第二实参 fromIndex 与 indexOf 同口径起跳 (负数从尾部归一)
        let guard = read_elems(arr);
        let from_idx = args
          .get(1)
          .map(|v| tail_index(v, guard.len() as i64, 0))
          .unwrap_or(0) as usize;
        Ok(Value::Boolean(guard.iter().skip(from_idx).any(|v| {
          // 战役 C ③: 洞当 undefined 比 (node `[1,,2].includes(undefined)` 真), 宿主同款
          let v = match v {
            Value::TheHole => &Value::Undefined,
            other => other,
          };
          if target_is_nan {
            matches!(v, Value::Number(n) if n.is_nan())
          } else {
            v.test_equal_strict(target)
          }
        })))
      }
      Self::ArrayConcat(arr) => {
        let mut out = read_elems(arr).clone();
        for arg in args {
          match arg {
            Value::Array(other) => out.extend(read_elems(other).iter().cloned()),
            _ => out.push(arg.clone()),
          }
        }
        Ok(Value::new_array_with_elements(out))
      }
      Self::ArraySort(_) => Err(Error::ExecutionError(
        "sort 需要比较器回调, 只能经 call_function 截获".to_string(),
      )),
      Self::ArrayIter(kind, _) => Err(Error::ExecutionError(format!(
        "{} 需要回调用户函数, 只能经 call_function 截获",
        kind.name()
      ))),
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
            return Ok(Value::new_string((b as char).to_string()));
          }
        } else if let Some(ch) = string.chars().nth(idx) {
          return Ok(Value::new_string(ch.to_string()));
        }
        Ok(Value::new_string(String::new()))
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
      Self::StringLastIndexOf(string) => {
        let needle = args
          .first()
          .map(Value::to_str_lossy)
          .unwrap_or(Cow::Borrowed("undefined"));
        let c_len = char_count(string) as i64;
        // node 实测口径 (全表钉门禁): 缺省/undefined/NaN→len; 任意负数→-1 (区间空,
        // 非 includes/indexOf 的"从尾归一"); 正数截断后夹 len; 对象等 ToNumber→NaN→len
        let pos = match args.get(1) {
          None | Some(Value::Undefined) => c_len,
          Some(v) => {
            let x = v.to_number();
            if x.is_nan() {
              c_len
            } else {
              let n = to_index_i64(x);
              if n < 0 { -1 } else { n.min(c_len) }
            }
          }
        };
        Ok(Value::Smi(str_last_index_of(string, &needle, pos)))
      }
      Self::StringSubstring(string) | Self::StringSlice(string) | Self::StringSubstr(string) => {
        let kind = match self {
          Self::StringSubstring(_) => SliceKind::Substring,
          Self::StringSlice(_) => SliceKind::Slice,
          _ => SliceKind::Substr,
        };
        let c_len = char_count(string) as i64;
        let start = args
          .first()
          .map(|v| to_index_i64(v.to_number()))
          .unwrap_or(0);
        let end = args
          .get(1)
          .filter(|v| !matches!(v, Value::Undefined))
          .map(|v| to_index_i64(v.to_number()));
        let (from_c, to_c) = slice_window(kind, c_len, start, end);
        let from_b = char_to_byte_index(string, from_c);
        let to_b = char_to_byte_index(string, to_c);
        Ok(Value::new_string(&string[from_b..to_b]))
      }
      Self::StringSplit(string) => Value::string_split_inlined(string, args),
      Self::StringReplace(string) => Value::string_replace_inlined(string, args),
      Self::StringMatch(string) => Value::string_match_inlined(string, args),
      Self::StringToLowerCase(string) => Ok(Value::new_string(string.to_lowercase())),
      Self::StringToUpperCase(string) => Ok(Value::new_string(string.to_uppercase())),
      Self::StringIncludes(string) => {
        let needle = args
          .first()
          .map(Value::to_str_lossy)
          .unwrap_or(Cow::Borrowed("undefined"));
        let from = args
          .get(1)
          .filter(|v| !matches!(v, Value::Undefined))
          .map(|v| to_index_i64(v.to_number()))
          .unwrap_or(0);
        Ok(Value::Boolean(str_includes(string, &needle, from)))
      }
      Self::StringRepeat(string) => {
        let count = args
          .first()
          .map(|v| to_index_i64(v.to_number()))
          .unwrap_or(0);
        str_repeat(string, count)
          .map(Value::new_string)
          .ok_or_else(|| Error::ExecutionError("RangeError: Invalid count value".to_string()))
      }
      Self::StringPad(string, start) => {
        let width = args
          .first()
          .map(|v| to_index_i64(v.to_number()))
          .unwrap_or(0);
        let pad = args
          .get(1)
          .filter(|v| !matches!(v, Value::Undefined))
          .map_or(Cow::Borrowed(" "), Value::to_str_lossy);
        str_pad(string, width, &pad, *start)
          .map(Value::new_string)
          .ok_or_else(|| {
            Error::ExecutionError(format!(
              "RangeError: {} max value is 2^53-1",
              if *start { "padStart" } else { "padEnd" }
            ))
          })
      }
      Self::StringAt(string) => {
        let idx = args
          .first()
          .map(|v| to_index_i64(v.to_number()))
          .unwrap_or(0);
        Ok(match str_at(string, idx) {
          Some(ch) => Value::new_string(ch.to_string()),
          None => Value::Undefined,
        })
      }
      Self::StringCodePointAt(string) => {
        // 越界给 undefined (区别于 charCodeAt 的 NaN, node 语义如此)
        let idx = args
          .first()
          .map(|v| to_index_i64(v.to_number()))
          .unwrap_or(0);
        Ok(match str_code_point_at(string, idx) {
          Some(cp) => Value::Number(cp as f64),
          None => Value::Undefined,
        })
      }
      Self::StringTrim(string, kind) => Ok(Value::new_string(str_trim(string, *kind))),
      Self::StringStartsWith(string) => {
        let needle = args
          .first()
          .map(Value::to_str_lossy)
          .unwrap_or(Cow::Borrowed("undefined"));
        let from = args
          .get(1)
          .filter(|v| !matches!(v, Value::Undefined))
          .map(|v| to_index_i64(v.to_number()))
          .unwrap_or(0);
        Ok(Value::Boolean(str_starts_with(string, &needle, from)))
      }
      Self::StringEndsWith(string) => {
        // 缺省 endPosition 是**全长** (与 startsWith 的缺省 0 相反), 给 i64::MAX 由纯函数夹到 len
        let needle = args
          .first()
          .map(Value::to_str_lossy)
          .unwrap_or(Cow::Borrowed("undefined"));
        let pos = args
          .get(1)
          .filter(|v| !matches!(v, Value::Undefined))
          .map_or(i64::MAX, |v| to_index_i64(v.to_number()));
        Ok(Value::Boolean(str_ends_with(string, &needle, pos)))
      }
      Self::StringReplaceAll(string) => {
        if matches!(args.get(1), Some(Value::Function(_) | Value::Native(..))) {
          return Err(Error::ExecutionError(
            "TypeError: replaceAll 回调替换未接入 (战役 C)".to_string(),
          ));
        }
        let pat = args.first().unwrap_or(&Value::Undefined);
        // 正则面: 非 g 标志按规范 TypeError (node 口径); g 标志借道 replace 的全局 exec 循环,
        // $ 序列同口径; 字符串面走 str_replace_all 纯函数
        if let Ok(exec_val) = pat.get_property(EXEC_KEY)
          && matches!(exec_val, Value::Native(..) | Value::Function(_))
        {
          let is_global = pat
            .get_property(GLOBAL_KEY)
            .map(|v| v.to_boolean())
            .unwrap_or(false);
          if !is_global {
            return Err(Error::ExecutionError(
              "TypeError: String.prototype.replaceAll called with a non-global RegExp argument"
                .to_string(),
            ));
          }
          return Value::string_replace_inlined(string, args);
        }
        let search = pat.to_str_lossy();
        let repl = args
          .get(1)
          .map(Value::to_str_lossy)
          .unwrap_or(Cow::Borrowed("undefined"));
        Ok(Value::new_string(str_replace_all(string, &search, &repl)))
      }
      Self::StringConcat(string) => {
        // 参数按 ToString 单真相拼接 (null→"null"、true→"true"、数字走 dtoa)
        let mut out = String::with_capacity(string.len() + 16);
        out.push_str(string);
        for a in args {
          out.push_str(&a.to_string_lossy());
        }
        Ok(Value::new_string(out))
      }
      Self::ArrayAt(arr) => {
        let guard = read_elems(arr);
        let len = guard.len() as i64;
        let idx = args
          .first()
          .map(|v| to_index_i64(v.to_number()))
          .unwrap_or(0);
        // 负索引从尾起算; i64::MIN 用 saturating 防回绕
        let i = if idx < 0 {
          len.saturating_add(idx)
        } else {
          idx
        };
        Ok(if i < 0 || i as usize >= guard.len() {
          Value::Undefined
        } else {
          guard[i as usize].clone()
        })
      }
      Self::ArrayFlat(arr) => {
        let depth = args
          .first()
          .filter(|v| !matches!(v, Value::Undefined))
          .map_or(1, |v| to_index_i64(v.to_number()));
        let mut out = Vec::new();
        flatten_into(&read_elems(arr), depth.max(0), &mut out);
        Ok(Value::new_array_with_elements(out))
      }
      Self::ArrayCopyWithin(arr) => {
        // 目标区/源区各自夹到 [0, len], 快照源段再写回 (重叠方向由快照兜底, node 同结果)
        let wrap = |v: i64, len: usize| -> usize {
          let t = if v < 0 {
            (len as i64).saturating_add(v)
          } else {
            v
          };
          t.clamp(0, len as i64) as usize
        };
        let len = read_elems(arr).len();
        let target = wrap(
          args
            .first()
            .map(|v| to_index_i64(v.to_number()))
            .unwrap_or(0),
          len,
        );
        let src_start = wrap(
          args
            .get(1)
            .map(|v| to_index_i64(v.to_number()))
            .unwrap_or(0),
          len,
        );
        let src_end = wrap(
          args
            .get(2)
            .filter(|v| !matches!(v, Value::Undefined))
            .map_or(len as i64, |v| to_index_i64(v.to_number())),
          len,
        );
        let count = src_end
          .saturating_sub(src_start)
          .min(len.saturating_sub(target));
        if count > 0 {
          let snapshot: Vec<Value> = read_elems(arr)[src_start..src_start + count].to_vec();
          write_elems(arr)[target..target + count].clone_from_slice(&snapshot);
        }
        Ok(Value::Array(Arc::clone(arr)))
      }
      Self::Context(ctx) => (ctx.call_fn)(ctx.payload, receiver, args),
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
  /// 字符串值
  ///
  /// 存 `Arc<str>` 而非 `String`: 寄存器与累加器之间的每次传递都是值克隆, 原版是堆指针拷贝,
  /// 用 String 就等于每条 `LdaConstant`/`Star` 都 malloc + memcpy 一遍字面量 (profile 里
  /// clone/drop 占掉解释循环的三成)。克隆退化为一次引用计数, 字面量也不再逐份复制。
  String(Arc<str>),
  BigInt(String),
  /// 符号值 (战役 C-2 第一步): 身份模型沿用宿主口径 —— 按描述串相等, 唯一 id 语义挂账;
  /// 作属性键走 `@@symbol:` 前缀编码 (与宿主 `element_key` 一处定义)
  Symbol(String),
  Array(Arc<RwLock<ArrayData>>),
  Object(Arc<RwLock<HashMap<String, Value>>>),
  Function(Arc<Closure>),
  Native(NativeFn, Arc<RwLock<HashMap<String, Value>>>),
  /// 正则对象 (战役 C-2b 第二刀): 直持 `Arc<RegExpCore>`, 跨桥浅拷贝即共享身份 ——
  /// pattern/flags/六布尔/lastIndex/编译体住内核 (引擎 `regexp.rs` 一处定义),
  /// `props` 袋只装用户自有键 (expando / 方法遮蔽), 读臂袋先行即遮蔽语义;
  /// 取代旧"物化成普通对象 + @@split 哨兵 + @@shadow 墓碑 + 伪键名册"全套代管机器
  RegExp(Arc<RegExpCore>, Arc<RwLock<HashMap<String, Value>>>),
}

/// 数组的两块存储: 元素槽 + 具名属性槽
///
/// 对标原版 `JSArray` = `JSObject` (属性) + `FixedArray` (元素)。此前只有元素槽,
/// `a.foo = 1` 被 `set_property` 静默丢弃, 读回 `undefined` —— 与 node/V8 不一致。
/// 属性槽用 `Option` 惰性分配: 纯数组负载 (基准里的绝大多数) 不必为用不到的表多花一次堆分配。
#[derive(Debug, Clone, Default)]
pub struct ArrayData {
  pub elements: Vec<Value>,
  pub props: Option<HashMap<String, Value>>,
}

impl ArrayData {
  #[inline]
  fn props_mut(&mut self) -> &mut HashMap<String, Value> {
    self.props.get_or_insert_with(HashMap::default)
  }
}

/// 取元素槽的读守卫
///
/// `ArrayData` 分成两块存储后, 内建方法与元素 opcode 只碰元素; 经守卫映射可以把原有的
/// `guard.len()` / `guard.iter()` / `guard[i]` 写法整片保留, 不必逐处补 `.elements`
#[inline]
fn read_elems(arr: &Arc<RwLock<ArrayData>>) -> MappedRwLockReadGuard<'_, RawRwLock, Vec<Value>> {
  RwLockReadGuard::map(arr.read(), |data| &data.elements)
}

/// 取元素槽的写守卫
#[inline]
fn write_elems(arr: &Arc<RwLock<ArrayData>>) -> MappedRwLockWriteGuard<'_, RawRwLock, Vec<Value>> {
  RwLockWriteGuard::map(arr.write(), |data| &mut data.elements)
}

/// ECMA-262 ToIntegerOrInfinity 后按尾部归一下标: indexOf/includes 起跳与 fill 界共用一份口径
/// (undefined 交回调用方缺省, NaN→0, ≥len→len, ≤-len→0, 负数→len+n, 其余截断)
#[inline]
fn tail_index(v: &Value, len: i64, on_undefined: i64) -> i64 {
  if matches!(v, Value::Undefined) {
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

/// 元素槽的最大可寻址下标
///
/// `length` 在本引擎里以 `Value::Smi` (i32) 表达, 超过它的下标既写不进也读不回长度; 更要紧的是
/// 下标会参与 `resize(idx + 1)` —— f64 → usize 的饱和转换把 1e21 这类越界值折成 `usize::MAX`,
/// `idx + 1` 直接溢出 panic。超出即退回具名属性慢路径, 与 V8 把非数组下标当字符串键的行为一致
const MAX_ELEM_INDEX: usize = i32::MAX as usize;

/// 键控访问的数组快路径下标: 只有非负整数键能直接落元素槽, 其余交给具名慢路径
///
/// Get/SetKeyedProperty 两处判定必须逐字一致 (一处放行另一处不放行会造成写走 A 路径、读走 B
/// 路径的键分裂), 因此收进同一个函数
#[inline]
fn fast_elem_index(key: &Value) -> Option<usize> {
  let idx = match key {
    Value::Smi(i) if *i >= 0 => *i as usize,
    Value::Number(n) if *n >= 0.0 && n.fract() == 0.0 => *n as usize,
    _ => return None,
  };
  (idx <= MAX_ELEM_INDEX).then_some(idx)
}

/// 规范下标字符串 → 元素槽序号
///
/// 只认十进制、无先导零、不越界三种形态同时成立的键 (`"007"` / `"+7"` / `"1e2"` 按规范是普通
/// 字符串属性, 不是数组下标)。判定不分配: 这条在 `in` 的臂上，为它拼一次 `to_string` 就把筛除
/// 原型回落省下的钱又付回去了
#[inline]
#[must_use]
pub fn canonical_index(prop: &str) -> Option<usize> {
  let bytes = prop.as_bytes();
  if bytes.is_empty()
    || bytes.len() > 10
    || (bytes[0] == b'0' && bytes.len() > 1)
    || !bytes.iter().all(|b| b.is_ascii_digit())
  {
    return None;
  }
  // 长度已夹在 1..=10, parse 不会溢出也不会失败
  let idx = prop.parse::<usize>().ok()?;
  (idx <= MAX_ELEM_INDEX).then_some(idx)
}

/// 函数值实例 (对标原版 JSFunction: 共享代码 + 各自的属性表 + 各自的词法环境)
///
/// 原版每个 JSFunction 都带一个 `context` 字段, 指向创建它那一帧的环境; 内层函数读写外层变量
/// 走的就是它。我们此前只有 (代码, 属性表) 两件, 被捕获的变量只能借用全局表来回搬运 —— 同名
/// 变量互相覆盖、递归调用取到旧值、闭包实例身份也丢了 (两次 `mk()` 比较相等)。
#[derive(Debug)]
pub struct Closure {
  /// 编译期产物: 同一函数字面量的所有实例共享
  pub func: Arc<InterpFunction>,
  /// 实例自有属性 (`prototype`、用户挂上去的键等)
  pub props: RwLock<HashMap<String, Value>>,
  /// 创建时刻的环境链, 顶层为 `Undefined`
  pub context: Value,
  /// 箭头函数创建时刻定格的词法 this (普通函数不读此位, 恒为 `Undefined`)
  pub lexical_this: Value,
  /// 箭头函数创建时刻定格的词法实参快照; `None` = 词法链上没有函数 (脚本顶层定义的箭头)
  pub lexical_arguments: Option<Vec<Value>>,
}

/// 该数值能否折叠成 `Value::Smi`
///
/// 只排除负零: 原版把 -0 作为 ReadOnlyRoots 里的规范 HeapNumber (static-roots-*.h 的
/// kMinusZero), 从不落成 Smi; 一旦被归一化成 Smi(0), `1 / -0 === -Infinity` 与
/// `SameValue(+0, -0) === false` 这类语义会全部失真。其余整数值 (含负数) 照旧走 Smi。
#[must_use]
pub fn fits_smi(n: f64) -> bool {
  !(n == 0.0 && n.is_sign_negative())
    && n.fract() == 0.0
    && n >= (i32::MIN as f64)
    && n <= (i32::MAX as f64)
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
      (Self::Symbol(a), Self::Symbol(b)) => a == b,
      (Self::Array(a), Self::Array(b)) => Arc::ptr_eq(a, b),
      (Self::Object(a), Self::Object(b)) => Arc::ptr_eq(a, b),
      // 正则按内核指针相等 (node 对象身份口径): 同实例跨桥浅拷贝共享 core 即相等,
      // 两次 /a/ 字面量各有独立 core 不等; 袋不参与比较 (与 Native 只比 fn 同口径)
      (Self::RegExp(a, _), Self::RegExp(b, _)) => Arc::ptr_eq(a, b),
      (Self::Function(a), Self::Function(b)) => Arc::ptr_eq(a, b),
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
      Self::Symbol(s) => write!(f, "Symbol({s})"),
      Self::RegExp(re, _) => write!(f, "/{}/", re.as_str()),
      Self::Array(arr) => write!(f, "Array({:?})", *read_elems(arr)),
      Self::Object(obj) => write!(f, "Object({:?})", *obj.read()),
      Self::Function(closure) => write!(f, "Function({:?})", closure.func),
      Self::Native(..) => write!(f, "NativeFunction"),
    }
  }
}

impl Value {
  /// 构造空数组
  #[inline]
  #[must_use]
  pub fn new_array() -> Self {
    Self::Array(Arc::new(RwLock::new(ArrayData::default())))
  }

  /// 构造已知元素数组
  #[inline]
  #[must_use]
  pub fn new_array_with_elements(elems: Vec<Self>) -> Self {
    Self::Array(Arc::new(RwLock::new(ArrayData {
      elements: elems,
      props: None,
    })))
  }

  /// 构造字符串值
  ///
  /// 变体存 `Arc<str>`, 但各处来源 (`String` / `&str` / `char`) 不一; 走构造器让它们统一
  /// 收敛成一次 `Into<Arc<str>>`, 免得每个站点手写转换。
  #[inline]
  #[must_use]
  pub fn new_string(s: impl Into<Arc<str>>) -> Self {
    Self::String(s.into())
  }

  /// 构造空对象
  #[inline]
  #[must_use]
  pub fn new_object() -> Self {
    Self::Object(Arc::new(RwLock::new(HashMap::default())))
  }

  /// 构造正则对象值 (内核 Arc 直接迁入, 不重编译; 空 expando 袋)
  #[inline]
  #[must_use]
  pub fn new_regexp(core: Arc<RegExpCore>) -> Self {
    Self::RegExp(core, Arc::new(RwLock::new(HashMap::default())))
  }

  /// 按已知的属性个数构造对象
  ///
  /// 字面量的属性数在生成期就是静态量, 一次把 backing store 铺到位，逐属性落表就不会半路
  /// `reserve_rehash` (原版靠 Map 里预铺的 N 个 in-object 槽达到同一效果)
  #[inline]
  #[must_use]
  pub fn new_object_with(cap: usize) -> Self {
    Self::Object(Arc::new(RwLock::new(HashMap::with_capacity(cap))))
  }

  /// 构造函数对象 (自动附加 prototype 属性对象)
  #[inline]
  #[must_use]
  pub fn new_function(func: Arc<InterpFunction>) -> Self {
    Self::new_function_with_context(func, Self::Undefined)
  }

  /// 在指定词法环境下构造闭包实例
  ///
  /// `context` 必须是创建它那一帧的环境 (顶层传 `Undefined`); 原版 JSFunction 的 `context`
  /// 字段就在此刻定格, 后续内层函数读写外层变量全靠它回溯, 而不是事后从全局表里捞。
  #[must_use]
  pub fn new_function_with_context(func: Arc<InterpFunction>, context: Self) -> Self {
    Self::function_value(func, context, Self::Undefined, None)
  }

  /// 构造箭头函数实例: `lexical_this` 与 `lexical_arguments` 都在创建时刻定格
  #[must_use]
  pub fn new_arrow_function(
    func: Arc<InterpFunction>,
    context: Self,
    lexical_this: Self,
    lexical_arguments: Option<Vec<Self>>,
  ) -> Self {
    Self::function_value(func, context, lexical_this, lexical_arguments)
  }

  /// 闭包实例的单一构造点 (普通/箭头共用)
  fn function_value(
    func: Arc<InterpFunction>,
    context: Self,
    lexical_this: Self,
    lexical_arguments: Option<Vec<Self>>,
  ) -> Self {
    let mut props = HashMap::default();
    // 箭头没有 [[Construct]], 规范也不挂 prototype 属性 (node: `'prototype' in fn === false`)
    let proto = if func.is_arrow {
      None
    } else {
      let proto = Self::new_object();
      props.insert(PROTOTYPE_KEY.to_string(), proto.clone());
      Some(proto)
    };
    let func_val = Self::Function(Arc::new(Closure {
      func,
      props: RwLock::new(props),
      context,
      lexical_this,
      lexical_arguments,
    }));
    // 对标原版: 普通函数自带 prototype.constructor, 实例靠它回溯构造器
    // (assert.throws 之类宿主代码读 thrown.constructor 会在此处取不到而 TypeError)
    if let Some(proto) = proto {
      let _ = proto.set_property(CONSTRUCTOR_KEY, func_val.clone());
    }
    func_val
  }

  pub fn string_split_inlined(string: &str, args: &[Self]) -> Result<Self> {
    let sep = args.first().unwrap_or(&Self::Undefined);
    // 缺省/undefined 分隔符按规范整串单元素返回 —— 拿 "undefined" 当针是巧合对拍 (串里含
    // "undefined" 就漏), 与宿主 StringSplit 臂同口径
    if matches!(sep, Self::Undefined) {
      return Ok(Self::new_array_with_elements(vec![Self::new_string(
        string.to_string(),
      )]));
    }
    // limit 与宿主同口径: ToInt32 后负数视为不限 (node ToUint32(-1)≈u32::MAX), 0 交空表
    let limit = args
      .get(1)
      .map(|v| {
        let n = v.to_i32();
        if n >= 0 { Some(n as usize) } else { None }
      })
      .unwrap_or(None);
    // C-2b 第二刀: 正则分隔符直落内核 split (旧 @@split 哨兵委托已由变体消解);
    // 用户对象带 exec 的鸭子回路仍在下方案, 两链口径不变
    if let Self::RegExp(core, _) = sep {
      return regex_split_core(
        core,
        &Self::Undefined,
        &[
          Self::new_string(string),
          args.get(1).cloned().unwrap_or(Self::Undefined),
        ],
      );
    }
    if let Ok(exec_val) = sep.get_property(EXEC_KEY)
      && let Self::Native(exec_fn, _) = exec_val
    {
      let _ = sep.set_property(LAST_INDEX_KEY, Self::Smi(0));
      let mut parts = Vec::new();
      let mut last_end = 0;
      let mut cur_search_pos = 0;
      let text_val = Self::new_string(string);

      loop {
        if cur_search_pos > string.len() {
          break;
        }
        let match_res = exec_fn.call(
          &Self::Undefined,
          &[text_val.clone(), Self::Smi(cur_search_pos as i32)],
        )?;
        if let Self::Array(arr) = &match_res {
          let guard = read_elems(arr);
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
          parts.push(Self::new_string(piece));
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
      parts.push(Self::new_string(remainder));
      let _ = sep.set_property(LAST_INDEX_KEY, Self::Smi(0));
      if let Some(lim) = limit {
        parts.truncate(lim);
      }
      return Ok(Self::new_array_with_elements(parts));
    }

    let sep_str = sep.to_str_lossy();
    let mut parts: Vec<Self> = if sep_str.is_empty() {
      string
        .chars()
        .map(|c| Self::new_string(c.to_string()))
        .collect()
    } else {
      string.split(&*sep_str).map(Self::new_string).collect()
    };
    if let Some(lim) = limit {
      parts.truncate(lim);
    }
    Ok(Self::new_array_with_elements(parts))
  }

  /// 无回调入口: 函数替换实参不经此处 (分发器在调用点截获, 同 ArraySort/ArrayIter)
  pub fn string_replace_inlined(string: &str, args: &[Self]) -> Result<Self> {
    Self::string_replace_core(string, args, &mut |_, _| {
      Err(Error::ExecutionError(
        "replace: callback must be intercepted by dispatcher".to_string(),
      ))
    })
  }

  fn string_replace_core(
    string: &str,
    args: &[Self],
    invoke: &mut dyn FnMut(&Self, &[Self]) -> Result<String>,
  ) -> Result<Self> {
    let pattern = args.first().unwrap_or(&Self::Undefined);
    let subst = args.get(1).cloned().unwrap_or(Self::Undefined);
    let use_fn = matches!(subst, Self::Function(_) | Self::Native(..));
    let repl_str = if use_fn {
      String::new()
    } else {
      subst.to_string_lossy()
    };

    // (原"@@replace 委托"分支移除) 该分支检查的是**字符串键** "replace" 而非 @@replace:
    // 命中面只有 bridge 自家 regex 对象 (与下方 exec 循环同语义) 和 `{replace: f}` 普通对象
    // (node 按字符串字面量处理) —— 保留反而制造两链分裂, 统一走 exec 循环

    if let Ok(exec_val) = pattern.get_property(EXEC_KEY)
      && let Self::Native(exec_fn, _) = exec_val
    {
      let is_global = pattern
        .get_property(GLOBAL_KEY)
        .map(|v| v.to_boolean())
        .unwrap_or(false);
      let _ = pattern.set_property(LAST_INDEX_KEY, Self::Smi(0));

      let mut out = String::new();
      let mut last_end = 0usize;
      let mut cur_search_pos = 0usize;
      let text_val = Self::new_string(string);

      loop {
        if cur_search_pos > string.len() {
          break;
        }
        let match_res = exec_fn.call(
          &Self::Undefined,
          &[text_val.clone(), Self::Smi(cur_search_pos as i32)],
        )?;
        let Self::Array(arr) = &match_res else { break };
        let (match_str, groups, start_idx) = {
          let guard = read_elems(arr);
          let whole = guard.first().map(Self::to_string_lossy).unwrap_or_default();
          // 字节起点就地重算: exec 的 `index` 是码单位口径 (与 node/宿主一致), 这里切片要的是
          // 字节位 —— 混用会在非 ASCII 文本上错位; 空匹配 find 给 0, 即当前搜索位
          let bstart = string[cur_search_pos..]
            .find(&*whole)
            .map(|off| cur_search_pos + off)
            .unwrap_or(cur_search_pos);
          let groups: Vec<Option<String>> = guard
            .get(1..)
            .map(|gs| {
              gs.iter()
                .map(|v| {
                  if matches!(v, Self::Undefined) {
                    None
                  } else {
                    Some(v.to_string_lossy())
                  }
                })
                .collect()
            })
            .unwrap_or_default();
          (whole, groups, bstart)
        };
        let end_idx = start_idx + match_str.len();

        if start_idx >= last_end && start_idx <= string.len() {
          out.push_str(&string[last_end..start_idx]);
        }
        let piece = Self::substitute_one(
          &repl_str,
          &subst,
          &match_str,
          &groups,
          &string[..start_idx],
          &string[end_idx.min(string.len())..],
          char_count(&string[..start_idx]),
          &text_val,
          invoke,
        )?;
        out.push_str(&piece);
        last_end = end_idx;

        if !is_global {
          break;
        }
        if match_str.is_empty() {
          cur_search_pos = start_idx + 1;
        } else {
          cur_search_pos = end_idx.max(cur_search_pos + 1);
        }
      }
      if last_end < string.len() {
        out.push_str(&string[last_end..]);
      }
      let _ = pattern.set_property(LAST_INDEX_KEY, Self::Smi(0));
      Ok(Self::new_string(out))
    } else {
      // 字符串模式: 字面量单处替换, $ 序列与回调同样生效 (node 口径)
      let pat_str = pattern.to_str_lossy();
      if let Some(pos) = string.find(&*pat_str) {
        let whole = &string[pos..pos + pat_str.len()];
        let piece = Self::substitute_one(
          &repl_str,
          &subst,
          whole,
          &[],
          &string[..pos],
          &string[pos + pat_str.len()..],
          char_count(&string[..pos]),
          &Self::new_string(string),
          invoke,
        )?;
        let mut out = String::with_capacity(string.len() + piece.len());
        out.push_str(&string[..pos]);
        out.push_str(&piece);
        out.push_str(&string[pos + pat_str.len()..]);
        Ok(Self::new_string(out))
      } else {
        Ok(Self::new_string(string))
      }
    }
  }

  /// 单次命中的替换文本: 回调 (全匹配, 组..., offset, 全串) 或 $ 序列展开
  #[allow(clippy::too_many_arguments)]
  fn substitute_one(
    repl_str: &str,
    subst: &Self,
    match_str: &str,
    groups: &[Option<String>],
    before: &str,
    after: &str,
    offset: usize,
    text_val: &Self,
    invoke: &mut dyn FnMut(&Self, &[Self]) -> Result<String>,
  ) -> Result<String> {
    if matches!(subst, Self::Function(_) | Self::Native(..)) {
      let mut cb_args = Vec::with_capacity(groups.len() + 3);
      cb_args.push(Self::new_string(match_str));
      for g in groups {
        cb_args.push(
          g.as_ref()
            .map(|s| Self::new_string(s.clone()))
            .unwrap_or(Self::Undefined),
        );
      }
      cb_args.push(Self::Smi(offset as i32));
      cb_args.push(text_val.clone());
      return invoke(subst, &cb_args);
    }
    let grefs: Vec<Option<&str>> = groups.iter().map(|g| g.as_deref()).collect();
    Ok(get_substitution(repl_str, match_str, before, after, &grefs))
  }

  pub fn string_match_inlined(string: &str, args: &[Self]) -> Result<Self> {
    let pat = args.first().unwrap_or(&Self::Undefined);
    // "字符串键 match 委托"分支已移除 (同 string_replace_inlined): 统一走 exec 循环,
    // 非全局单点命中直接返回 exec 结果 (自带 index/input)
    if let Ok(exec_val) = pat.get_property(EXEC_KEY)
      && let Self::Native(exec_fn, _) = exec_val
    {
      let is_global = pat
        .get_property(GLOBAL_KEY)
        .map(|v| v.to_boolean())
        .unwrap_or(false);
      let text_val = Self::new_string(string);
      let _ = pat.set_property(LAST_INDEX_KEY, Self::Smi(0));

      if is_global {
        let mut results = Vec::new();
        let mut cur_search_pos = 0;
        loop {
          if cur_search_pos > string.len() {
            break;
          }
          let match_res = exec_fn.call(
            &Self::Undefined,
            &[text_val.clone(), Self::Smi(cur_search_pos as i32)],
          )?;
          if let Self::Array(arr) = &match_res {
            let guard = read_elems(arr);
            let match_str = guard.first().map(Self::to_string_lossy).unwrap_or_default();
            // 字节起点就地重算 (同 string_replace_inlined 口径)
            let start_idx = string[cur_search_pos..]
              .find(&*match_str)
              .map(|off| cur_search_pos + off)
              .unwrap_or(cur_search_pos);
            let end_idx = start_idx + match_str.len();

            results.push(Self::new_string(match_str.clone()));

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
        exec_fn.call(&Self::Undefined, &[text_val, Self::Smi(0)])
      }
    } else {
      // 字符串/原始值模式: node 会 RegExpCreate 按正则解释, 引擎侧没有正则内核 —— 先按字面量
      // 单处命中 (无元字符时与 node 一致), 捕获组恒空; 元字符分歧已记待办 (战役 H 引擎正则收口)
      let pat_str = pat.to_string_lossy();
      match string.find(&*pat_str) {
        Some(pos) => {
          let res = Self::new_array_with_elements(vec![Self::new_string(
            string[pos..pos + pat_str.len()].to_string(),
          )]);
          let chars = char_count(&string[..pos]);
          let _ = res.set_property(INDEX_KEY, Self::Number(chars as f64));
          let _ = res.set_property(INPUT_KEY, Self::new_string(string));
          Ok(res)
        }
        None => Ok(Self::Null),
      }
    }
  }

  pub fn new_native(native: NativeFn) -> Self {
    let mut props = HashMap::default();
    let proto = Self::new_object();
    props.insert(PROTOTYPE_KEY.to_string(), proto);
    Self::Native(native, Arc::new(RwLock::new(props)))
  }

  /// 装箱原始值: 原生构造器返回原始值时, 实例仍是对象但要能回吐该值
  ///
  /// 对标原版 JSReceiver 的 [[PrimValue]]。缺这一步, `new Boolean(true) + true` 会走
  /// Object.prototype.toString 塌成 "[object Object]true", 而规范要求 2
  #[inline]
  #[must_use]
  pub(crate) fn boxed_primitive(obj: Self, prim: Self) -> Self {
    if matches!(prim, Self::Undefined | Self::Null) || prim.is_object_like() {
      return obj;
    }
    let as_string = prim.to_str_lossy().to_string();
    let _ = obj.set_property(
      VALUE_OF_KEY,
      Self::new_native_bare(NativeFn::Const(Box::new(prim))),
    );
    let _ = obj.set_property(
      TO_STRING_KEY,
      Self::new_native_bare(NativeFn::ConstString(as_string.into())),
    );
    obj
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

  /// 是否为需要 ToPrimitive 的对象侧操作数
  #[inline]
  #[must_use]
  pub(crate) fn is_object_like(&self) -> bool {
    matches!(
      self,
      Self::Object(_) | Self::Array(_) | Self::Function(..) | Self::Native(..) | Self::RegExp(..)
    )
  }

  /// 两个操作数都已经是原始值 (算术 opcode 的快路径判据)
  #[inline]
  #[must_use]
  pub(crate) fn both_primitive(lhs: &Self, rhs: &Self) -> bool {
    !lhs.is_object_like() && !rhs.is_object_like()
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
      Self::Symbol(_) => "symbol",
      Self::Function(..) | Self::Native(..) => "function",
      Self::Array(_) | Self::Object(_) | Self::RegExp(..) => "object",
    }
  }

  /// Array 内建方法表的单一真相
  ///
  /// 原版这些方法挂在 `Array.prototype` 上, 一次查表即可; 这里方法名在取属性与 `in` 判定两处都
  /// 要用, 各列一份必然漂移 (曾出现 `"sort" in a` 为假而 `a.sort` 可用的分裂)
  fn array_builtin(arr: &Arc<RwLock<ArrayData>>, prop: &str) -> Option<NativeFn> {
    Some(match prop {
      PUSH_KEY => NativeFn::ArrayPush(Arc::clone(arr)),
      POP_KEY => NativeFn::ArrayPop(Arc::clone(arr)),
      SHIFT_KEY => NativeFn::ArrayShift(Arc::clone(arr)),
      SLICE_KEY => NativeFn::ArraySlice(Arc::clone(arr)),
      JOIN_KEY => NativeFn::ArrayJoin(Arc::clone(arr)),
      // Array.prototype.toString === join(",")，缺这一步会掉到 Object.prototype.toString
      TO_STRING_KEY => NativeFn::ArrayJoin(Arc::clone(arr)),
      INDEX_OF_KEY => NativeFn::ArrayIndexOf(Arc::clone(arr)),
      LAST_INDEX_OF_KEY => NativeFn::ArrayLastIndexOf(Arc::clone(arr)),
      FILL_KEY => NativeFn::ArrayFill(Arc::clone(arr)),
      UNSHIFT_KEY => NativeFn::ArrayUnshift(Arc::clone(arr)),
      SPLICE_KEY => NativeFn::ArraySplice(Arc::clone(arr)),
      REVERSE_KEY => NativeFn::ArrayReverse(Arc::clone(arr)),
      SORT_KEY => NativeFn::ArraySort(Arc::clone(arr)),
      INCLUDES_KEY => NativeFn::ArrayIncludes(Arc::clone(arr)),
      CONCAT_KEY => NativeFn::ArrayConcat(Arc::clone(arr)),
      MAP_KEY => NativeFn::ArrayIter(ArrayIterKind::Map, Arc::clone(arr)),
      FOR_EACH_KEY => NativeFn::ArrayIter(ArrayIterKind::ForEach, Arc::clone(arr)),
      FILTER_KEY => NativeFn::ArrayIter(ArrayIterKind::Filter, Arc::clone(arr)),
      REDUCE_KEY => NativeFn::ArrayIter(ArrayIterKind::Reduce, Arc::clone(arr)),
      EVERY_KEY => NativeFn::ArrayIter(ArrayIterKind::Every, Arc::clone(arr)),
      SOME_KEY => NativeFn::ArrayIter(ArrayIterKind::Some, Arc::clone(arr)),
      FIND_KEY => NativeFn::ArrayIter(ArrayIterKind::Find, Arc::clone(arr)),
      FIND_INDEX_KEY => NativeFn::ArrayIter(ArrayIterKind::FindIndex, Arc::clone(arr)),
      REDUCE_RIGHT_KEY => NativeFn::ArrayIter(ArrayIterKind::ReduceRight, Arc::clone(arr)),
      FLAT_MAP_KEY => NativeFn::ArrayIter(ArrayIterKind::FlatMap, Arc::clone(arr)),
      AT_KEY => NativeFn::ArrayAt(Arc::clone(arr)),
      FLAT_KEY => NativeFn::ArrayFlat(Arc::clone(arr)),
      COPY_WITHIN_KEY => NativeFn::ArrayCopyWithin(Arc::clone(arr)),
      _ => return None,
    })
  }

  /// Number 装箱后的内建方法表 (Smi 与 Number 共用)
  fn number_builtin(val: &Value, prop: &str) -> Option<NativeFn> {
    let n = match val {
      Self::Smi(i) => *i as f64,
      Self::Number(n) => *n,
      _ => return None,
    };
    Some(match prop {
      VALUE_OF_KEY => NativeFn::Const(Box::new(val.clone())),
      TO_STRING_KEY => NativeFn::NumberToString(n),
      TO_FIXED_KEY => NativeFn::NumberToFixed(n),
      TO_PRECISION_KEY => NativeFn::NumberToPrecision(n),
      TO_EXPONENTIAL_KEY => NativeFn::NumberToExponential(n),
      _ => return None,
    })
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
          // 链顶为 null (Object.create(null) 产物) 即止交 undefined, 不再向下解引用
          if matches!(proto, Self::Null | Self::Undefined) {
            return Ok(Self::Undefined);
          }
          return proto.get_property_depth(prop, depth + 1);
        }
        Ok(Self::Undefined)
      }
      Self::Array(arr) => {
        if prop == LENGTH_KEY {
          let len = read_elems(arr).len();
          return Ok(Self::Smi(len as i32));
        }
        if prop.as_bytes().first().is_some_and(|b| b.is_ascii_digit())
          && let Ok(idx) = prop.parse::<usize>()
          && idx <= MAX_ELEM_INDEX
        {
          let guard = read_elems(arr);
          return Ok(guard.get(idx).map_or(Self::Undefined, Self::materialized));
        }
        // 自有具名键必须遮蔽内建方法: 原版内建挂在 Array.prototype 上, 自有属性优先级更高,
        // `a.push = f` 之后 a.push() 必须调用 f。此前先查方法表, 写进 props 的键永远读不回来
        if let Some(props) = arr.read().props.as_ref()
          && let Some(val) = props.get(prop)
        {
          return Ok(val.clone());
        }
        match Self::array_builtin(arr, prop) {
          Some(native) => Ok(Self::new_native_bare(native)),
          None => Ok(Self::Undefined),
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
              return Ok(Self::new_string((b as char).to_string()));
            }
          } else if let Some(ch) = s.chars().nth(idx) {
            return Ok(Self::new_string(ch.to_string()));
          }
        }
        match prop {
          "charCodeAt" => Ok(Self::new_native_bare(NativeFn::StringCharCodeAt(s.clone()))),
          "charAt" => Ok(Self::new_native_bare(NativeFn::StringCharAt(s.clone()))),
          "indexOf" => Ok(Self::new_native_bare(NativeFn::StringIndexOf(s.clone()))),
          "lastIndexOf" => Ok(Self::new_native_bare(NativeFn::StringLastIndexOf(
            s.clone(),
          ))),
          "substring" => Ok(Self::new_native_bare(NativeFn::StringSubstring(s.clone()))),
          "slice" => Ok(Self::new_native_bare(NativeFn::StringSlice(s.clone()))),
          "substr" => Ok(Self::new_native_bare(NativeFn::StringSubstr(s.clone()))),
          "split" => Ok(Self::new_native_bare(NativeFn::StringSplit(s.clone()))),
          "replace" => Ok(Self::new_native_bare(NativeFn::StringReplace(s.clone()))),
          "match" => Ok(Self::new_native_bare(NativeFn::StringMatch(s.clone()))),
          "toLowerCase" => Ok(Self::new_native_bare(NativeFn::StringToLowerCase(
            s.clone(),
          ))),
          "toUpperCase" => Ok(Self::new_native_bare(NativeFn::StringToUpperCase(
            s.clone(),
          ))),
          INCLUDES_KEY => Ok(Self::new_native_bare(NativeFn::StringIncludes(s.clone()))),
          REPEAT_KEY => Ok(Self::new_native_bare(NativeFn::StringRepeat(s.clone()))),
          PAD_START_KEY => Ok(Self::new_native_bare(NativeFn::StringPad(s.clone(), true))),
          PAD_END_KEY => Ok(Self::new_native_bare(NativeFn::StringPad(s.clone(), false))),
          AT_KEY => Ok(Self::new_native_bare(NativeFn::StringAt(s.clone()))),
          CODE_POINT_AT_KEY => Ok(Self::new_native_bare(NativeFn::StringCodePointAt(
            s.clone(),
          ))),
          TRIM_KEY => Ok(Self::new_native_bare(NativeFn::StringTrim(
            s.clone(),
            TrimKind::Both,
          ))),
          TRIM_START_KEY | TRIM_LEFT_KEY => Ok(Self::new_native_bare(NativeFn::StringTrim(
            s.clone(),
            TrimKind::Start,
          ))),
          TRIM_END_KEY | TRIM_RIGHT_KEY => Ok(Self::new_native_bare(NativeFn::StringTrim(
            s.clone(),
            TrimKind::End,
          ))),
          STARTS_WITH_KEY => Ok(Self::new_native_bare(NativeFn::StringStartsWith(s.clone()))),
          ENDS_WITH_KEY => Ok(Self::new_native_bare(NativeFn::StringEndsWith(s.clone()))),
          REPLACE_ALL_KEY => Ok(Self::new_native_bare(NativeFn::StringReplaceAll(s.clone()))),
          CONCAT_KEY => Ok(Self::new_native_bare(NativeFn::StringConcat(s.clone()))),
          "toString" | "valueOf" => Ok(Self::new_native_bare(NativeFn::ConstString(s.clone()))),
          _ => Ok(Self::Undefined),
        }
      }
      Self::Function(closure) => {
        let func = &closure.func;
        let props = &closure.props;
        if prop == LENGTH_KEY {
          return Ok(Self::Smi(func.params.len() as i32));
        }
        if prop == NAME_KEY {
          return Ok(Self::new_string(func.name.clone().unwrap_or_default()));
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
          // 链顶为 null (Object.create(null) 产物) 即止交 undefined, 不再向下解引用
          if matches!(proto, Self::Null | Self::Undefined) {
            return Ok(Self::Undefined);
          }
          return proto.get_property_depth(prop, depth + 1);
        }
        // #23 ⑤: 函数没有自有 toString 且无原型链时给源码替身 (Function.prototype 面未物化)
        if prop == TO_STRING_KEY {
          return Ok(Self::new_native_bare(NativeFn::ConstString(Arc::from(
            func.src.clone().unwrap_or_else(|| {
              format!(
                "function {}() {{ [native code] }}",
                func.name.as_deref().unwrap_or("anonymous")
              )
            }),
          ))));
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
        // #23 ⑤ 同口径: 内建函数没有自有 toString 时自合成本机摘要 (一处口径 = to_str_lossy
        // 的 Native 臂), 否则加法强转漏到继承的 Object.prototype.toString 吐 "[object Function]",
        // 与 AST 链分叉 (门禁 78); 原型链上用户覆盖仍由 guard 命中先行兜下
        if prop == TO_STRING_KEY {
          return Ok(Self::new_native_bare(NativeFn::ConstString(
            self.to_str_lossy().as_ref().into(),
          )));
        }
        if let Some(proto) = guard.get(PROTO_KEY) {
          // 链顶为 null (Object.create(null) 产物) 即止交 undefined, 不再向下解引用
          if matches!(proto, Self::Null | Self::Undefined) {
            return Ok(Self::Undefined);
          }
          return proto.get_property_depth(prop, depth + 1);
        }
        Ok(Self::Undefined)
      }
      Self::RegExp(core, props) => {
        // 读臂次序与宿主 JsValue::RegExp 完全对齐: 自有袋先行 (expando / 方法遮蔽强于内建),
        // 再内核派生名, 最后内建面; 只读访问器名与 lastIndex 不会入袋 (set 臂拒收/专存)
        if let Some(v) = props.read().get(prop) {
          return Ok(v.clone());
        }
        if prop == SOURCE_KEY {
          return Ok(Self::new_string(core.pattern.clone()));
        }
        if prop == FLAGS_KEY {
          return Ok(Self::new_string(core.flags.clone()));
        }
        if prop == LAST_INDEX_KEY {
          return Ok(Self::Number(*core.last_index.read() as f64));
        }
        // 六个布尔标志读值与 boolean_flags 一张表同源 (一处定义)
        if let Some((_, v)) = core.boolean_flags().iter().find(|(n, _)| *n == prop) {
          return Ok(Self::Boolean(*v));
        }
        match prop {
          EXEC_KEY => Ok(regex_exec_native(core)),
          TEST_KEY => Ok(regex_test_native(core)),
          // 自有面 = lastIndex 恒自有 + expando 袋 (与宿主 own_property_names 同册):
          // hasOwnProperty 按此册, 可枚举性只认袋 (lastIndex 不可枚举)
          HAS_OWN_PROPERTY_KEY => Ok(Self::new_native_bare(NativeFn::Ptr(regexp_bag_has_own))),
          PROPERTY_IS_ENUMERABLE_KEY => Ok(Self::new_native_bare(NativeFn::Ptr(
            regexp_bag_prop_enumerable,
          ))),
          TO_STRING_KEY => Ok(Self::new_native_bare(NativeFn::ConstString(
            format!("/{}/{}", core.pattern, core.flags).into(),
          ))),
          _ => Ok(Self::Undefined),
        }
      }
      Self::Smi(_) | Self::Number(_) => match Self::number_builtin(self, prop) {
        Some(native) => Ok(Self::new_native_bare(native)),
        None => Ok(Self::Undefined),
      },
      Self::Boolean(b) => {
        if prop == VALUE_OF_KEY {
          Ok(Self::new_native_bare(NativeFn::Const(Box::new(
            Self::Boolean(*b),
          ))))
        } else if prop == TO_STRING_KEY {
          let s = if *b { "true" } else { "false" };
          Ok(Self::new_native_bare(NativeFn::ConstString(s.into())))
        } else {
          Ok(Self::Undefined)
        }
      }
      _ => Ok(Self::Undefined),
    }
  }

  /// 删除属性 — sloppy 口径: 除数组 `length` (不可删) 外恒 true (node: 缺失键/继承键
  /// delete 都交 true, 非 configurable 拦截挂战役 C 属性旗标); 与 AST 链 DeleteUnary
  /// 的恒 true 同口径, 两链不再分叉
  pub fn delete_property(&self, prop: &str) -> bool {
    match self {
      Self::RegExp(_, props) => {
        // 袋里只有用户自有键: 删得掉的只有 expando/遮蔽; 内核派生名不在袋中, 天然删不动
        // (node 口径 delete r.source 交 true 且不生效, 同结果)。delete r.lastIndex node
        // 实为 true 且真删 —— 残档挂十五 B (#18 旗标族同源)
        props.write().remove(prop);
        true
      }
      Self::Object(obj) => {
        let mut guard = obj.write();
        guard.remove(prop);
        // accessor 以伪键落表: `delete o.q` 必须把 `@@get:q` 与 `@@set:q` 两条都回收, 否则
        // 删完赋值仍会打到旧 setter (与 AST 链路分裂)。这里不能用 `any`/`||` 短路求值,
        // 短路只删命中的那条; 哨兵键保留 —— 清它要全表扫 @@get/@@set, 代价远多于多一次哈希探测
        if guard.contains_key(ACCESSOR_FLAG_KEY) {
          let _ = guard.remove(&format_getter_key(prop));
          let _ = guard.remove(&format_setter_key(prop));
        }
        true
      }
      Self::Function(closure) => {
        closure.props.write().remove(prop);
        true
      }
      Self::Native(_, props) => {
        props.write().remove(prop);
        true
      }
      Self::Array(arr) => {
        // `length` 是不可删除属性; 索引键的 delete 按规范留洞 (战役 C ③): 槽写 TheHole,
        // 读侧物化 undefined、`in`/keys/for-in 即不存在, 与 node 全等
        if prop == LENGTH_KEY {
          return false;
        }
        if let Some(idx) = canonical_index(prop) {
          let mut guard = write_elems(arr);
          if let Some(slot) = guard.get_mut(idx) {
            *slot = Value::TheHole;
          }
        } else if let Some(props) = &mut arr.write().props {
          props.remove(prop);
        }
        true
      }
      _ => true,
    }
  }

  /// 设置命名/键控属性
  pub fn set_property(&self, prop: &str, val: Self) -> Result<()> {
    match self {
      Self::RegExp(core, props) => {
        // 次序与宿主 JsValue::RegExp set 臂逐字对齐: lastIndex 专存内核, 只读访问器名静默
        // 忽略 (名册 is_regexp_readonly_key 两链共读), 其余键入袋即遮蔽内建面 (node e10 语义)
        if prop == LAST_INDEX_KEY {
          *core.last_index.write() = val.to_i32().max(0) as usize;
        } else if is_regexp_readonly_key(prop) {
          // 只读访问器赋值无效 (node sloppy: `r.source="X"` 读回原 pattern)
        } else {
          props.write().insert(prop.to_string(), val);
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
      Self::Function(closure) => {
        closure.props.write().insert(prop.to_string(), val);
        Ok(())
      }
      Self::Native(_, props) => {
        props.write().insert(prop.to_string(), val);
        Ok(())
      }
      Self::Array(arr) => {
        if prop == LENGTH_KEY {
          let new_len = val.to_u32() as usize;
          // 拉长交洞 (node: `a.length=3` 后 `0 in a` 假), 截断照 resize
          write_elems(arr).resize(new_len, Self::TheHole);
          return Ok(());
        }
        // 上界与 get_property / fast_elem_index 同一条: 超界的数字键按具名属性存 (V8 亦然),
        // 且不让 `idx + 1` 带着 usize::MAX 进 resize (溢出 panic)
        if prop.as_bytes().first().is_some_and(|b| b.is_ascii_digit())
          && let Ok(idx) = prop.parse::<usize>()
          && idx <= MAX_ELEM_INDEX
        {
          let mut guard = write_elems(arr);
          if idx >= guard.len() {
            // 中间空位交洞: `a=[1]; a[3]=2` 后 1/2 是洞非属性
            guard.resize(idx + 1, Self::TheHole);
          }
          // 写入侧兜底: 泄漏的洞哨兵落槽即物化 (同 set_element)
          guard[idx] = Self::materialized(&val);
          return Ok(());
        }
        // 非索引具名键落在属性槽: 此前被静默丢弃, `a.foo = 1` 读回 undefined, 与 V8 不符
        arr.write().props_mut().insert(prop.to_string(), val);
        Ok(())
      }
      _ => Ok(()),
    }
  }

  /// 数组读面物化 (战役 C ③): 洞哨兵只栖身元素槽, 出槽即 undefined —— 一处定义,
  /// `a[i]`/`get_property` 数字臂等全部读点共用; `in`/keys/JSON 各有自己的册不走这里
  #[inline]
  fn materialized(val: &Self) -> Self {
    match val {
      Self::TheHole => Self::Undefined,
      other => other.clone(),
    }
  }

  /// 按数值索引获取元素
  pub fn get_element(&self, index: usize) -> Result<Self> {
    match self {
      Self::Array(arr) => Ok(
        read_elems(arr)
          .get(index)
          .map_or(Self::Undefined, Self::materialized),
      ),
      Self::Object(obj) => {
        let guard = obj.read();
        let mut buf = ItoaBuffer::new();
        let key = buf.format(index);
        Ok(guard.get(key).cloned().unwrap_or(Self::Undefined))
      }
      Self::String(s) => {
        if s.is_ascii() {
          if let Some(&b) = s.as_bytes().get(index) {
            Ok(Self::new_string((b as char).to_string()))
          } else {
            Ok(Self::Undefined)
          }
        } else if let Some(ch) = s.chars().nth(index) {
          Ok(Self::new_string(ch.to_string()))
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
        let mut guard = write_elems(arr);
        if index >= guard.len() {
          // 中间空位交洞, 与 set_property 数字臂同口径
          guard.resize(index + 1, Self::TheHole);
        }
        // 写入侧兜底: 泄漏的洞哨兵落槽即物化 (node `a[1]=undefined` 后 `1 in a` 真)
        guard[index] = Self::materialized(&val);
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
      Self::Symbol(_) => true,
      Self::Array(_)
      | Self::Object(_)
      | Self::Function(..)
      | Self::Native(..)
      | Self::RegExp(..) => true,
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
        // ECMA-262 9.3.1 全量口径: 0x/0o/0b 前缀、±Infinity、空白由 V8 移植的
        // `string_to_double` 一处判定 (原 Rust parse 只认十进制, "0b101" 会错吐 NaN)
        string_to_double(s.as_bytes(), ConversionFlag::AllowNonDecimalPrefix, 0.0)
      }
      Self::BigInt(s) => s.parse::<f64>().unwrap_or(0.0),
      Self::Symbol(_) => f64::NAN,
      Self::Array(arr) => {
        let guard = read_elems(arr);
        if guard.is_empty() {
          0.0
        } else if guard.len() == 1 {
          // 洞物化空串再转数 (node `+[,1 的前身]`: ToPrimitive 走 join 交 "" → 0)
          match &guard[0] {
            Self::TheHole | Self::Undefined | Self::Null => 0.0,
            other => other.to_number(),
          }
        } else {
          f64::NAN
        }
      }
      Self::Object(obj) => {
        let guard = obj.read();
        // ToPrimitive(hint=number) 与宿主 AST 求值器同序: 时间戳槽 → 兼容槽 → 原始值 valueOf
        let primitive = guard
          .get(TIMESTAMP_KEY)
          .or_else(|| guard.get(TIME_KEY))
          .or_else(|| {
            guard
              .get(VALUE_OF_KEY)
              .filter(|v| !matches!(v, Self::Object(_) | Self::Function(..) | Self::Native(..)))
          });
        primitive.map_or(f64::NAN, Self::to_number)
      }
      Self::Function(..) | Self::Native(..) | Self::RegExp(..) => f64::NAN,
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
      Self::Number(n) => Cow::Owned(number_to_js_string(*n)),
      Self::String(s) => Cow::Borrowed(s),
      Self::BigInt(s) => Cow::Borrowed(s.as_str()),
      // 与宿主 to_string 同口径: String(sym) 吐 "Symbol(desc)" (隐式转换抛 TypeError 挂账符号唯一 id 语义)
      Self::Symbol(s) => Cow::Owned(format!("Symbol({s})")),
      Self::Array(arr) => {
        let guard = read_elems(arr);
        let mut out = String::new();
        for (i, item) in guard.iter().enumerate() {
          if i > 0 {
            out.push(',');
          }
          out.push_str(&item.to_str_lossy());
        }
        Cow::Owned(out)
      }
      // C-2b 第二刀: 正则文本面直取内核 (旧"物化对象 + regex_repr 还原"同链路已由变体消解)
      Self::RegExp(core, _) => Cow::Owned(format!("/{}/{}", core.pattern, core.flags)),
      Self::Object(_) => Cow::Borrowed("[object Object]"),
      Self::Function(closure) => match &closure.func.src {
        // 源码原文是 toString 的真相 (#23 ⑤); 无源 (宿主带过的替身) 才退回摘要
        Some(s) => Cow::Borrowed(s),
        None => Cow::Owned(format!(
          "function {}() {{ [native code] }}",
          closure.func.name.as_deref().unwrap_or("anonymous")
        )),
      },
      Self::Native(..) => Cow::Borrowed("function () { [native code] }"),
    }
  }

  /// 元素/键控访问的键归一 (与宿主 `element_key` 同口径): Symbol 走 `@@symbol:` 前缀编码,
  /// 其余值同 `to_str_lossy`; 属性名/枚举面不经此口 (node: String(sym) 吐 "Symbol(k)")
  #[must_use]
  pub fn to_element_key(&self) -> Cow<'_, str> {
    match self {
      Self::Symbol(s) => Cow::Owned(format_symbol_key(s)),
      other => other.to_str_lossy(),
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
      return Self::new_string(out);
    }
    if let (_, Self::String(b)) = (self, other) {
      let self_str = self.to_str_lossy();
      let mut out = String::with_capacity(self_str.len() + b.len());
      out.push_str(&self_str);
      out.push_str(b);
      return Self::new_string(out);
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
      // 两侧都已是字符串时按字符串比较: ToPrimitive 之后 `[]` 是 "", 若继续落到下面的数值分支,
      // "" 和 "0" 会被转成同一个 0, `[] == "0"` 就误判为真
      (Self::String(a), Self::String(b)) => a == b,
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
      (Self::Symbol(a), Self::Symbol(b)) => a == b,
      (Self::Array(a), Self::Array(b)) => Arc::ptr_eq(a, b),
      (Self::Object(a), Self::Object(b)) => Arc::ptr_eq(a, b),
      // 正则按内核指针相等 (node 对象身份口径): 同实例跨桥浅拷贝共享 core 即相等,
      // 两次 /a/ 字面量各有独立 core 不等; 袋不参与比较 (与 Native 只比 fn 同口径)
      (Self::RegExp(a, _), Self::RegExp(b, _)) => Arc::ptr_eq(a, b),
      (Self::Function(a), Self::Function(b)) => Arc::ptr_eq(a, b),
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

/// 全局变量表句柄
///
/// 对标原版 NativeContext: 一张表被同一执行 isolate 内所有执行态共享。跨界调用只克隆句柄
/// (一次原子加计数), 既不逐键复制整表, 也不在调用点重建空表。
pub type GlobalsTable = Arc<RwLock<HashMap<String, Value>>>;

/// 字节码分发器 / 解释执行器
#[derive(Debug, Clone, Default)]
pub struct BytecodeDispatcher {
  accumulator: Value,
  reg_stack: Vec<Value>,
  param_stack: Vec<Value>,
  frame_reg_base: usize,
  frame_param_base: usize,
  frame_param_len: usize,
  /// 进帧时刻的实参个数 (不含 receiver): `arguments` 名册的唯一长度真相。
  /// 不能复用 frame_param_len —— 形参默认值等降级体会以参数寄存器号回写形参名,
  /// `set_register` 会把 frame_param_len 撑大, 让 `arguments.length` 虚报 (12.39)
  frame_arguments_len: usize,
  /// 当前帧正在执行的字节码偏移, 供异常展开时查询本帧 HandlerTable
  frame_offset: usize,
  /// `throw` 抛出的原值; `Error` 只带字符串, 所以待抛值必须挂在执行态上随展开传递
  pending_exception: Option<Value>,
  arguments_materialized: bool,
  saved_arguments: Option<Value>,
  /// 当前帧的箭头实参源 (12.28): 外层 `Some` = 本帧是箭头帧, 内层 `Some(Vec)` = 创建时刻
  /// 定格的词法实参快照 (顶层定义的箭头没有外层函数, 内层为 `None` → `arguments` 未绑定);
  /// `None` = 普通帧, `arguments` 照旧取本帧参数切片
  frame_arguments_lex: Option<Option<Vec<Value>>>,
  globals: GlobalsTable,
  /// 本次执行中被 StaGlobal 写过的名字
  ///
  /// 宿主 (rs8) 的全局环境是另一套值模型, 每次执行后必须把新写入的全局同步过去。若整表回写,
  /// 每次都要对上千个内建做一遍深转换, 且会把函数/对象换成新的副本、抹掉引用身份。原版
  /// GlobalDictionary 只在 property 实际变更时才标记, 这里同构: 只暴露脏名给宿主。
  dirty_globals: HashSet<String>,
  /// `Array.prototype` 上被用户写过/删过的内建方法名 (#24)
  ///
  /// 内建表是 `Array.prototype` 基线的原生替身, 一旦某个同名键在原型上被动过, 替身对这个名字
  /// 就不再可信: 读侧改走原型回溯 (给出用户函数, 或被删后的 `undefined`)。只有写侧命中
  /// "名字在内建表里 ∧ 接收者是 Array.prototype" 才记账, 读侧也只在内建表命中时多查一次
  /// 集合, 未覆盖的原型保持空集 → 热路径零额外开销
  array_proto_watch: HashSet<String>,
  /// 当前帧的词法环境 (顶层为 `Undefined`)
  ///
  /// 对标原版的 context 链条: 每个被调用的闭包都带创建时刻的环境, 帧内
  /// 被捕获的名字存进本帧的环境对象里, 沿 `CONTEXT_PARENT_KEY` 向外回溯。顶层脚本没有帧环境,
  /// 保持 `Undefined`, 全局读写走原来的全局表, 因此这条链不会给顶层循环增加任何开销。
  cur_context: Value,
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
      frame_arguments_len: 0,
      frame_offset: 0,
      pending_exception: None,
      arguments_materialized: false,
      saved_arguments: None,
      frame_arguments_lex: None,
      globals: GlobalsTable::default(),
      dirty_globals: HashSet::default(),
      array_proto_watch: HashSet::default(),
      cur_context: Value::Undefined,
    }
  }

  /// 复用既有全局表构造分发器 (跨界调用入口: 被调函数必须看见调用方的全局绑定)
  ///
  /// 寄存器栈按需用 `set_register` 扩容, 此处不预分配, 免得每次跨界都付一次大 Vec 的分配
  #[inline]
  #[must_use]
  pub fn with_globals(globals: GlobalsTable) -> Self {
    Self {
      accumulator: Value::Undefined,
      reg_stack: Vec::new(),
      param_stack: Vec::new(),
      frame_reg_base: 0,
      frame_param_base: 0,
      frame_param_len: 0,
      frame_arguments_len: 0,
      frame_offset: 0,
      pending_exception: None,
      arguments_materialized: false,
      saved_arguments: None,
      frame_arguments_lex: None,
      globals,
      dirty_globals: HashSet::default(),
      array_proto_watch: HashSet::default(),
      cur_context: Value::Undefined,
    }
  }

  /// 本次执行写过的全局名, 宿主据此只同步真正变更的绑定
  #[inline]
  #[must_use]
  pub fn dirty_globals(&self) -> &HashSet<String> {
    &self.dirty_globals
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

  /// 全局表句柄, 供宿主在跨界值转换时随函数一起带出
  #[inline]
  #[must_use]
  pub fn global_table(&self) -> &GlobalsTable {
    &self.globals
  }

  /// 写入全局绑定: 键已存在就地覆值, 只有首次落键才分配 String
  ///
  /// `insert` 会换成一个新 key 并丢弃旧的, 循环里的 `StaGlobal` 于是每次都要 malloc/free 一次
  /// 变量名; 原版全局写入是 property 数组的原位更新, 这里用 get_mut 复现同一效果。
  #[inline]
  pub fn set_global(&mut self, name: &str, val: Value) {
    let mut table = self.globals.write();
    if let Some(slot) = table.get_mut(name) {
      *slot = val;
    } else {
      table.insert(name.to_string(), val);
    }
  }

  /// 表是共享句柄, 无法向外借出引用, 因此按值返回
  #[inline]
  #[must_use]
  pub fn get_global(&self, name: &str) -> Option<Value> {
    self.globals.read().get(name).cloned()
  }

  /// 只做存在性判断, 避免注入侧为检查而克隆整个值
  #[inline]
  #[must_use]
  pub fn has_global(&self, name: &str) -> bool {
    self.globals.read().contains_key(name)
  }

  /// 为一次调用建出被调帧的环境对象
  ///
  /// 原版只在函数确有 context slot 时才分配 Context, 这里同构: 没有名字被子函数捕获时
  /// 直接沿用闭包自带的链 (顶层闭包即 `Undefined`), 一次调用不多付一次哈希表分配。
  #[must_use]
  fn frame_context(closure: &Closure) -> Value {
    let locals = &closure.func.captured_locals;
    if locals.is_empty() {
      return closure.context.clone();
    }
    let mut map: HashMap<String, Value> = HashMap::default();
    map.insert(CONTEXT_PARENT_KEY.to_string(), closure.context.clone());
    for name in locals {
      map.insert(name.clone(), Value::Undefined);
    }
    Value::Object(Arc::new(RwLock::new(map)))
  }

  /// 沿环境链找到持有该名字的那一帧
  fn find_context(&self, name: &str) -> Option<Arc<RwLock<HashMap<String, Value>>>> {
    let mut curr = self.cur_context.clone();
    loop {
      let Value::Object(map) = curr else {
        return None;
      };
      if map.read().contains_key(name) {
        return Some(map);
      }
      curr = map.read().get(CONTEXT_PARENT_KEY).cloned()?;
    }
  }

  /// 读一个变量名: 先沿当前帧的环境链回溯, 再退到全局表
  #[inline]
  #[must_use]
  pub fn resolve_name(&self, name: &str) -> Option<Value> {
    if matches!(self.cur_context, Value::Undefined) {
      return self.get_global(name);
    }
    self
      .find_context(name)
      .and_then(|cell| cell.read().get(name).cloned())
      .or_else(|| self.get_global(name))
  }

  /// 写一个变量名: 环境链上已有该绑定就写进那一帧, 否则落到全局表
  ///
  /// 环境单元不是全局, 因此不参与 `dirty_globals` 登记 —— 宿主只该同步真正的全局变更,
  /// 否则每次闭包内赋值都会把帧内变量抄回宿主全局表, 同名外层变量被覆盖。
  pub fn assign_name(&mut self, name: &str, val: Value) {
    if !matches!(self.cur_context, Value::Undefined)
      && let Some(cell) = self.find_context(name)
    {
      cell.write().insert(name.to_string(), val);
      return;
    }
    self.set_global(name, val);
    // 已登记过就不再动: `insert(name.to_string())` 会先分配 String 再发现键已存在而丢弃它,
    // 于是循环体里每次 `StaGlobal` 都付一次 malloc/free。`contains(&str)` 借用查一次哈希即可,
    // 只有真正缺失 (每个名字一次) 才多算第二次哈希并分配
    if !self.dirty_globals.contains(name) {
      self.dirty_globals.insert(name.to_string());
    }
  }

  /// 按"名字槽位"读常量池: 越界、类型不符、命中三分 (#26)
  ///
  /// 原版由 bytecode verifier 在编译期保证这些槽位的池项必为 string, 运行期不存在此分支;
  /// 本侧没有独立校验遍, 出错时也不能再拿"越界"顶罪 —— 索引有效但变体不对是编码缺陷,
  /// 报成 `ConstantPoolOutOfBounds` 会把排查引向池容量而不是发错常量的一方
  #[inline(always)]
  fn name_constant(array: &BytecodeArray, idx: usize) -> Result<&str> {
    match array.constants().get(idx) {
      None => Err(Error::ConstantPoolOutOfBounds(idx)),
      Some(ConstantPoolEntry::String(name)) => Ok(name),
      Some(other) => Err(Error::ConstantPoolTypeMismatch {
        idx,
        kind: other.kind_name(),
      }),
    }
  }

  /// 内建表的名字面: 对空接收者派生自 `array_builtin` 本体, 与表零漂移 (#24)
  ///
  /// 暴露给宿主 AST 链复用 —— 两链的写侧监视必须按同一份名单记账, 名单漂移即两链分裂
  #[must_use]
  pub fn is_array_builtin_name(prop: &str) -> bool {
    // 长度预筛: 22 个名字只占 3..8 与 11 三档, 普通属性写一次移位比较就出局
    const LEN_MASK: u32 =
      (1 << 3) | (1 << 4) | (1 << 5) | (1 << 6) | (1 << 7) | (1 << 8) | (1 << 11);
    let l = prop.len();
    l < 32 && (LEN_MASK & (1 << l)) != 0 && {
      static PROBE: OnceLock<Arc<RwLock<ArrayData>>> = OnceLock::new();
      let probe = PROBE.get_or_init(|| Arc::new(RwLock::new(ArrayData::default())));
      Value::array_builtin(probe, prop).is_some()
    }
  }

  /// 写侧记账 (#24): 名字命中内建表且接收者正是当前的 `Array.prototype` 才登记
  fn watch_array_proto(&mut self, recv: &Arc<RwLock<HashMap<String, Value>>>, prop: &str) {
    if self.array_proto_watch.contains(prop) {
      return;
    }
    if let Some(Value::Object(p)) = self.global_proto(ARRAY_CTOR)
      && Arc::ptr_eq(recv, &p)
    {
      self.array_proto_watch.insert(prop.to_string());
    }
  }

  /// `#[inline(always)]`: 采样 (2026-10-17, Richards 400 轮) 里它仍是独立符号 110 样本,
  /// `#[inline]` 在 1200 臂巨型函数面前只是建议, 不保证落地
  #[inline(always)]
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

  #[inline(always)]
  fn get_register(&self, reg: Register) -> Result<Value> {
    self.get_register_ref(reg).cloned()
  }

  #[inline(always)]
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

  /// 属性读写前的 receiver 校验 (对标原版 ToObject: 在 null/undefined 上取或设属性抛 TypeError)
  ///
  /// 只做在 opcode 侧而不是 `Value::get_property` 里: 原型回溯、`in`、typeof 之类内部查找必须
  /// 容忍空值, 否则 `typeof x.y` 与 `("x" in obj)` 的语义会一起坏掉
  /// `#[inline(always)]` + 抛错支 outline: 采样 (2026-10-17) 独立符号 120 样本, 快路径只是一次
  /// 标签判定; 错误文本拼装 (两次 format!) 不该随探测进分发循环
  #[inline(always)]
  fn check_property_receiver(obj: &Value, key: &str, writing: bool) -> Result<()> {
    if !matches!(obj, Value::Undefined | Value::Null) {
      return Ok(());
    }
    Self::property_receiver_type_error(obj, key, writing)
  }

  #[inline(never)]
  fn property_receiver_type_error(obj: &Value, key: &str, writing: bool) -> Result<()> {
    let (action, detail) = if writing {
      ("set", format!("setting '{key}')"))
    } else {
      ("read", format!("reading '{key}')"))
    };
    Err(Error::ExecutionError(format!(
      "TypeError: Cannot {action} properties of {} ({detail}",
      obj.to_str_lossy()
    )))
  }

  /// 引擎内部错误 → ECMAScript 异常对象
  ///
  /// 对标原版: 跨越 catch 边界的运行时错误必须是 Error 实例 (name/message 齐备, 原型指向对应内建
  /// 构造函数), 否则 `assert.throws(TypeError, f)` 与 `e instanceof TypeError` 整类断言失效。
  /// 消息前缀 "XxxError: " 决定挂哪个原型; ExecutionError 只取内层文本, 否则会带上 Rust 的包装
  fn error_object(&self, err: &Error) -> Value {
    let raw = match err {
      Error::ExecutionError(msg) => msg.clone(),
      other => other.to_string(),
    };
    let (name, message) = match raw.split_once(": ") {
      Some((head, tail)) if head.ends_with("Error") => (head.to_string(), tail.to_string()),
      _ => ("Error".to_string(), raw),
    };
    let obj = Value::new_object();
    let _ = obj.set_property(NAME_KEY, Value::new_string(name.clone()));
    let _ = obj.set_property("message", Value::new_string(message));
    if let Some(proto) = self.global_proto(&name) {
      let _ = obj.set_property(PROTO_KEY, proto);
    }
    obj
  }

  /// 原版 ToPrimitive 的入口: 原始值原样透传, 只有对象侧才付回溯 hook 的代价
  ///
  /// 对标原版 Add / 比较等运算的前置步骤, 标量对在算术 opcode 外壳里已经短路过, 这里只剩
  /// 一侧是对象的混合场景。名字不取 `to_*` 前缀: 该方法要跑用户侧 valueOf/toString, 会分配
  /// 寄存器并压调用栈, `self` 必须可变, 而 clippy 对 `to_*` 只接受 `&self`/`self`
  #[inline(always)]
  fn primitive_of(&mut self, val: &Value, hint_number: bool) -> Result<Value> {
    if val.is_object_like() {
      return self.primitive_from_object(val, hint_number);
    }
    Ok(val.clone())
  }

  /// hint_number 决定 hook 顺序: 对象侧先 valueOf, 结果仍是对象再 toString
  ///
  /// 数组、装箱对象直接 to_number 会塌成 NaN, 而规范要求 [1, 2] + [3, 4] === "1,23,4"、
  /// d2 - d1 是毫秒差。Date 在本模型里没有 [[DateValue]] 内部槽, 用 __timestamp__ 槽表达
  /// (与宿主 AST 求值器同序)
  fn primitive_from_object(&mut self, val: &Value, hint_number: bool) -> Result<Value> {
    let mut date_slot = None;
    let mut is_date = false;
    if let Value::Object(o) = val {
      let guard = o.read();
      if let Some(ts) = guard.get(TIMESTAMP_KEY).or_else(|| guard.get(TIME_KEY)) {
        is_date = true;
        date_slot = Some(ts.clone());
      }
    }
    if hint_number && let Some(ts) = date_slot {
      return Ok(ts);
    }
    // 与宿主 to_primitive 一致: Date 的 default hint 也优先 toString
    let hooks: [&str; 2] = if is_date {
      [TO_STRING_KEY, VALUE_OF_KEY]
    } else {
      [VALUE_OF_KEY, TO_STRING_KEY]
    };
    for hook in hooks {
      let callee = self.get_property_resolved(val, hook)?;
      if !matches!(callee, Value::Function(..) | Value::Native(..)) {
        continue;
      }
      let primitive = self.call_function(&callee, val, &[])?;
      if !primitive.is_object_like() {
        return Ok(primitive);
      }
    }
    Err(Error::ExecutionError(
      "TypeError: cannot convert object to primitive value".to_string(),
    ))
  }

  /// 算术 opcode 的统一外壳: 标量对零克隆直读, 对象侧先各走一次 ToPrimitive
  ///
  /// 原版在 kAdd/kSub/... 之前都有 ToPrimitive; 判定留在 opcode 侧而不是 Value 里, 因为
  /// Value 拿不到 dispatcher 就无法调用 valueOf/toString。规范里的数组拼接
  /// ([1,2] + [3,4])、装箱对象 ([{valueOf:1}] * 2) 全靠这一层。
  /// op 用泛型闭包而非函数指针: 每个 opcode 单态化后可直接内联 Value 的运算实现
  #[inline(always)]
  fn numeric_bin_op<F>(&mut self, lhs_reg: Register, hint_number: bool, op: F) -> Result<()>
  where
    F: Fn(&Value, &Value) -> Value,
  {
    let lhs = self.get_register_ref(lhs_reg)?;
    if Value::both_primitive(lhs, &self.accumulator) {
      self.accumulator = op(lhs, &self.accumulator);
      return Ok(());
    }
    let lhs_val = lhs.clone();
    let rhs_val = take(&mut self.accumulator);
    let primitive_lhs = self.primitive_of(&lhs_val, hint_number)?;
    let primitive_rhs = self.primitive_of(&rhs_val, hint_number)?;
    self.accumulator = op(&primitive_lhs, &primitive_rhs);
    Ok(())
  }

  /// 比较 opcode 的统一外壳, 语义同 numeric_bin_op
  #[inline(always)]
  fn compare_bin_op<F>(&mut self, lhs_reg: Register, hint_number: bool, op: F) -> Result<()>
  where
    F: Fn(&Value, &Value) -> bool,
  {
    let lhs = self.get_register_ref(lhs_reg)?;
    if Value::both_primitive(lhs, &self.accumulator) {
      self.accumulator = Value::Boolean(op(lhs, &self.accumulator));
      return Ok(());
    }
    let lhs_val = lhs.clone();
    let rhs_val = take(&mut self.accumulator);
    let primitive_lhs = self.primitive_of(&lhs_val, hint_number)?;
    let primitive_rhs = self.primitive_of(&rhs_val, hint_number)?;
    self.accumulator = Value::Boolean(op(&primitive_lhs, &primitive_rhs));
    Ok(())
  }

  /// 一元运算前置: 累加器交给 ToPrimitive, 标量侧由它内联透传
  fn acc_primitive(&mut self) -> Result<Value> {
    let val = take(&mut self.accumulator);
    self.primitive_of(&val, true)
  }

  /// 取 receiver 自身上 `prefix + prop` 的 accessor 函数值
  ///
  /// 没有 accessor 的对象只付一次 `contains_key` (静态键, 零分配)。原型链上的 accessor 暂不
  /// 触发: 判定"整条链有没有 accessor"需要 V8 的 Map 级标志位, 本值模型没有 shape (战役 C),
  /// 逐层探测会把每次属性读的一次哈希放大成 O(链长)。
  ///
  /// `#[inline(always)]`: 采样 (2026-10-17, Richards 400 轮) 里独立符号 515 样本 —— 快路径只是
  /// 一次锁 + 一次哈希探测, 调用开销与探测本体同量级; 拼前缀键的冷路径留在本函数内不展开
  #[inline(always)]
  fn lookup_accessor(obj: &Value, prop: &str, prefix: &str) -> Option<Value> {
    let Value::Object(map) = obj else {
      return None;
    };
    let guard = map.read();
    if !guard.contains_key(ACCESSOR_FLAG_KEY) {
      return None;
    }
    Self::lookup_accessor_slow(&guard, prop, prefix)
  }

  /// 哨兵命中后的拼前缀键查表 (罕见路径, 不随探测一起内联进分发循环)
  #[inline(never)]
  fn lookup_accessor_slow(
    guard: &RwLockReadGuard<'_, HashMap<String, Value>>,
    prop: &str,
    prefix: &str,
  ) -> Option<Value> {
    let key = format!("{prefix}{prop}");
    guard
      .get(&key)
      .filter(|v| matches!(v, Value::Function(_) | Value::Native(..)))
      .cloned()
  }

  /// 解析命名属性 (单次原型回溯，根除 O(N^2) 重复锁与哈希遍历)
  #[inline]
  pub fn get_property_resolved(&self, obj: &Value, prop: &str) -> Result<Value> {
    let val = obj.get_property(prop)?;
    // #24: 内建表命中但该名字在 Array.prototype 上被动过 → 替身让位, 继续原型回溯
    // (空集短路: 没有覆盖发生的热路径一次哈希都不付)
    let shadowed = !self.array_proto_watch.is_empty()
      && matches!(val, Value::Native(..))
      && matches!(obj, Value::Array(_))
      && self.array_proto_watch.contains(prop);
    if !matches!(val, Value::Undefined) && !shadowed {
      return Ok(val);
    }
    // 链显式止于 null (Object.create(null) 族) 的接收者没有任何继承面: 全局 prototype
    // 回落与 toString 兜底都要跳过 (node `Object.create(null).toString` 交 undefined)
    if self.chain_ends_at_null(obj) {
      return Ok(Value::Undefined);
    }
    // 内建接收者先回溯自身原型: Array.prototype / String.prototype 上的方法由宿主
    // 预置脚本注入, 只查 Object.prototype 会把它们全部丢掉
    if let Some(ctor_name) = builtin_ctor_name(obj)
      && let Some(v) = self.proto_prop(ctor_name, prop)
    {
      return Ok(v);
    }
    if let Some(v) = self.proto_prop(OBJECT_CTOR, prop) {
      return Ok(v);
    }
    if prop == TO_STRING_KEY {
      // 正则变体自带内建 toString 面 (get 臂), 不会漏到这里; 此处只剩普通对象兜底
      return Ok(Value::new_native_bare(NativeFn::ConstString(
        "[object Object]".into(),
      )));
    }
    Ok(Value::Undefined)
  }

  /// 读取全局内建构造函数的 prototype 对象
  fn global_proto(&self, ctor_name: &str) -> Option<Value> {
    let ctor = self.globals.read().get(ctor_name)?.clone();
    ctor.get_property(PROTOTYPE_KEY).ok()
  }

  /// 原型链是否显式止于 `__proto__ = null` (Object.create(null) 族)。为真时该对象没有任何
  /// 继承面, 全局 Object.prototype 回落与 `in` 兜底都须跳过, 否则 `toString in c` 误真;
  /// 链未物化 (无 `__proto__` 键) 的模型对象交 false, 沿用既有回落近似
  fn chain_ends_at_null(&self, obj: &Value) -> bool {
    let mut curr = obj.clone();
    let mut depth = 0;
    while depth < 64 {
      depth += 1;
      let next = match &curr {
        Value::Object(o) => o.read().get(PROTO_KEY).cloned(),
        Value::Array(a) => a
          .read()
          .props
          .as_ref()
          .and_then(|p| p.get(PROTO_KEY).cloned()),
        Value::Function(closure) => closure.props.read().get(PROTO_KEY).cloned(),
        Value::Native(_, props) => props.read().get(PROTO_KEY).cloned(),
        _ => return false,
      };
      match next {
        Some(Value::Null) => return true,
        Some(v) => curr = v,
        None => return false,
      }
    }
    false
  }

  /// 读取全局内建构造函数 prototype 上的同名属性, 未命中返回 None
  fn proto_prop(&self, ctor_name: &str, prop: &str) -> Option<Value> {
    let v = self.global_proto(ctor_name)?.get_property(prop).ok()?;
    if matches!(v, Value::Undefined) {
      None
    } else {
      Some(v)
    }
  }

  /// 检查属性是否存在于对象或其原型链中 (TestIn)
  pub fn has_property_resolved(&self, obj: &Value, prop: &str) -> bool {
    let mut curr = Some(obj.clone());
    let mut depth = 0;
    let mut stopped_at_null = false;
    while let Some(cur_val) = curr {
      depth += 1;
      if depth > 64 {
        break;
      }
      if matches!(cur_val, Value::Function(..) | Value::Native(..))
        && (prop == LENGTH_KEY
          || prop == NAME_KEY
          || prop == CALL_KEY
          || prop == APPLY_KEY
          // 箭头没有 [[Construct]] 也不挂 prototype 面 (node: `'prototype' in fn` 假)
          || (prop == PROTOTYPE_KEY && !matches!(&cur_val, Value::Function(closure) if closure.func.is_arrow)))
      {
        return true;
      }
      match cur_val {
        Value::Object(o) => {
          let guard = o.read();
          if guard.contains_key(prop) {
            return true;
          }
          // accessor 只以 `@@get:k`/`@@set:k` 伪键落表, `in` 判存在必须把伪键算上;
          // 哨兵命中才拼键查找, 没有 accessor 的对象仍只付一次哈希
          if guard.contains_key(ACCESSOR_FLAG_KEY)
            && [GETTER_PREFIX, SETTER_PREFIX]
              .iter()
              .any(|prefix| guard.contains_key(&format!("{prefix}{prop}")))
          {
            return true;
          }
          // 链顶显式 null (Object.create(null)) 即止: 不得再回落全局 Object.prototype 面
          match guard.get(PROTO_KEY) {
            Some(Value::Null) => {
              stopped_at_null = true;
              curr = None;
            }
            other => curr = other.cloned(),
          }
        }
        Value::RegExp(core, props) => {
          if props.read().contains_key(prop) {
            return true;
          }
          // 内核派生名与三个内建面在 node 经 RegExp.prototype 可见, `in` 一律答 true
          // (与旧物化对象"自有键在场即 true"同结果); 袋无 PROTO 槽, 链就此交回全局原型回落
          if prop == SOURCE_KEY
            || prop == FLAGS_KEY
            || prop == LAST_INDEX_KEY
            || matches!(prop, EXEC_KEY | TEST_KEY | TO_STRING_KEY)
            || core.boolean_flags().iter().any(|(n, _)| *n == prop)
          {
            return true;
          }
          curr = None;
        }
        Value::Function(closure) => {
          let guard = closure.props.read();
          if guard.contains_key(prop) {
            return true;
          }
          // 链顶显式 null (Object.create(null)) 即止: 不得再回落全局 Object.prototype 面
          match guard.get(PROTO_KEY) {
            Some(Value::Null) => {
              stopped_at_null = true;
              curr = None;
            }
            other => curr = other.cloned(),
          }
        }
        Value::Native(_, props) => {
          let guard = props.read();
          if guard.contains_key(prop) {
            return true;
          }
          // 链顶显式 null (Object.create(null)) 即止: 不得再回落全局 Object.prototype 面
          match guard.get(PROTO_KEY) {
            Some(Value::Null) => {
              stopped_at_null = true;
              curr = None;
            }
            other => curr = other.cloned(),
          }
        }
        Value::Array(a) => {
          // #24: 被监视的名字说明用户在 Array.prototype 上动过手脚, `in` 的真相在原型上,
          // 内建替身不得抢先答 true (node: `delete Array.prototype.map` 后 `"map" in a` 为假)
          if prop == LENGTH_KEY
            || (Value::array_builtin(&a, prop).is_some() && !self.array_proto_watch.contains(prop))
          {
            return true;
          }
          // 一次取 ArrayData 守卫同时看元素与具名属性: 嵌套 read 同一把锁在有写者等待时会自堵
          let data = a.read();
          if data
            .props
            .as_ref()
            .is_some_and(|props| props.contains_key(prop))
          {
            return true;
          }
          if let Some(idx) = canonical_index(prop) {
            // 元素槽就是下标键的全部真相: 命中即真, 越界即假, **不回溯 Array.prototype**。
            // 读侧 `a[i]` 同口径筛除原型 (见 eval_get_element 与 fast_elem_index), 一侧回溯一侧
            // 不回溯就自相矛盾 —— `(7 in a)` 真而 `a[7]` undefined, `if (i in a) a[i]` 这类代码
            // 直接踩空; 与 node 的偏差 (`Array.prototype[7] = x` 时 node 两边都真) 已由门禁 28 钉住
            // 战役 C ③: 洞槽即不存在 (node `1 in [1,,2]` 假), 与读侧物化 undefined 分册
            return idx < data.elements.len() && !matches!(data.elements[idx], Value::TheHole);
          }
          // 数组的方法面挂在 Array.prototype 上, `"map" in [1]` 必须为真; 该原型的 `__proto__`
          // 由预置脚本接到 Object.prototype, 交给循环继续走剩余链
          curr = self.global_proto(ARRAY_CTOR);
        }
        // 基本类型不是 Object, `in` 左侧为原始值时由调用点抛 TypeError, 这里不得回溯原型
        _ => break,
      }
    }
    // 链未显式止于 null 才回落全局 Object.prototype: Object.create(null) 的产物 (显式 __proto__
    // = null) 无原型方法面, node `("toString" in Object.create(null))` 交 false
    if !stopped_at_null
      && let Some(proto) = self.global_proto(OBJECT_CTOR)
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
      Value::Function(closure) => closure.props.read().get(PROTO_KEY).cloned(),
      Value::Native(_, props) => props.read().get(PROTO_KEY).cloned(),
      Value::RegExp(_, props) => props.read().get(PROTO_KEY).cloned(),
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
        (Value::Function(a), Value::Function(b)) => Arc::ptr_eq(a, b),
        _ => false,
      } {
        return true;
      }
      curr = match &cur_val {
        Value::Object(o) => o.read().get(PROTO_KEY).cloned(),
        Value::Function(closure) => closure.props.read().get(PROTO_KEY).cloned(),
        Value::Native(_, props) => props.read().get(PROTO_KEY).cloned(),
        Value::RegExp(_, props) => props.read().get(PROTO_KEY).cloned(),
        _ => None,
      };
    }

    if let Some(obj_proto) = self.global_proto(OBJECT_CTOR)
      && match (&target_proto, &obj_proto) {
        (Value::Object(a), Value::Object(b)) => Arc::ptr_eq(a, b),
        _ => false,
      }
      && matches!(
        instance,
        Value::Object(_) | Value::Array(_) | Value::Function(..) | Value::RegExp(..)
      )
    {
      return true;
    }

    false
  }

  /// Array.prototype.sort: 就地排序后返回接收者本身
  ///
  /// 无比较器按 ToString 的码点序比较 (ECMA-262: undefined 恒排末尾且不参与比较, 空位一并
  /// 跳过); 有比较器时逐项回调用户函数。比较器抛错没法中断 Rust 的稳定排序, 因此记录首个
  /// 错误、排序返回后再传播 —— 规范本就允许数组停留在半排序状态
  fn array_sort(
    &mut self,
    arr: &Arc<RwLock<ArrayData>>,
    comparator: Option<&Value>,
  ) -> Result<Value> {
    // 排序过程中会回调用户代码, 先脱锁取快照, 免得重入写同一张表自堵
    let (mut sorted, trailing) = {
      let guard = read_elems(arr);
      let defined: Vec<Value> = guard
        .iter()
        .filter(|v| !matches!(v, Value::Undefined | Value::TheHole))
        .cloned()
        .collect();
      let skipped = guard.len() - defined.len();
      (defined, skipped)
    };
    match comparator {
      // 缓存键版稳定排序: 无比较器路线上 to_str_lossy 每元素只算一次,
      // 而不是每次比较分配两个 String
      None => sorted.sort_by_cached_key(Value::to_string_lossy),
      Some(cb) => {
        let cb = cb.clone();
        let mut failure: Option<String> = None;
        sorted.sort_by(|a, b| {
          if failure.is_some() {
            return Ordering::Equal;
          }
          match self.call_function(&cb, &Value::Undefined, &[a.clone(), b.clone()]) {
            Ok(v) => {
              let n = v.to_number();
              if n < 0.0 {
                Ordering::Less
              } else if n > 0.0 {
                Ordering::Greater
              } else {
                Ordering::Equal
              }
            }
            Err(e) => {
              failure = Some(e.to_string());
              Ordering::Equal
            }
          }
        });
        if let Some(msg) = failure {
          return Err(Error::ExecutionError(msg));
        }
      }
    }
    for _ in 0..trailing {
      sorted.push(Value::Undefined);
    }
    *write_elems(arr) = sorted;
    Ok(Value::Array(Arc::clone(arr)))
  }

  /// 数组迭代类内建 (`map`/`forEach`/`filter`/`reduce`/`every`/`some`), 语义逐项对齐原版:
  /// 回调按 `(element, index, array)` 无 receiver 调用, `reduce` 无初值时从下标 1 起
  ///
  /// 不做成 `NativeFn::call` 的原因与 `sort` 相同: 回调是用户函数, 要分发器才能执行。
  /// 走原生还顺带绕开了"预置 JS 函数跨引擎回调时闭包环境丢失"的坑 (见 todo 战役 C)
  fn array_iter(
    &mut self,
    kind: ArrayIterKind,
    arr: &Arc<RwLock<ArrayData>>,
    args: &[Value],
  ) -> Result<Value> {
    let name = kind.name();
    let callback = match args.first() {
      Some(cb @ (Value::Function(_) | Value::Native(..))) => cb.clone(),
      Some(other) => {
        return Err(Error::ExecutionError(format!(
          "TypeError: {name} callback is not a function: {}",
          other.to_str_lossy()
        )));
      }
      None => {
        return Err(Error::ExecutionError(format!(
          "TypeError: {name} expects a callback function"
        )));
      }
    };
    // 规范口径: 长度取一次, 元素每步现取一次并在回调前定值。整表快照会让回调里 pop 同一个数组
    // 时继续交出旧元素 (`a.map` 在 `f` 里 pop: node `1,2,<hole>`, 快照版 `1,2,3`); 而"先读元素
    // 再调回调"决定了 `filter` 收集的是入选前那一行的值 (`f` 里改写 e[0]: node 收 1, 不是 99)
    //
    // 每步的读锁都必须在回调前释放: 回调可能同步增删同一个数组, 持读锁跨回调会自堵
    let len = read_elems(arr).len();
    let receiver = Value::Array(Arc::clone(arr));
    // 战役 C ③: 回调实参走读面物化 (回调永不见洞哨兵); 跳洞判定另走 is_hole 现读,
    // 与"每步现取"同一把锁纪律 —— 回调里增删同一数组时两侧都看当下真相
    let elem = |i: usize| {
      read_elems(arr)
        .get(i)
        .map_or(Value::Undefined, Value::materialized)
    };
    let is_hole = |i: usize| {
      read_elems(arr)
        .get(i)
        .is_some_and(|v| matches!(v, Value::TheHole))
    };
    // 规范: 回调的 this 绑定 arguments[1] (thisArg); reduce/reduceRight 的 arguments[1] 是初值, 恒 undefined
    let this_arg = if matches!(kind, ArrayIterKind::Reduce | ArrayIterKind::ReduceRight) {
      Value::Undefined
    } else {
      args.get(1).cloned().unwrap_or(Value::Undefined)
    };
    let mut call = |i: usize, element: Value, carry: Vec<Value>| {
      let mut argv = carry;
      argv.extend([element, Value::Number(i as f64), receiver.clone()]);
      self.call_function(&callback, &this_arg, &argv)
    };
    match kind {
      ArrayIterKind::Map => {
        // 洞跳回调且原样进结果 (整表以洞起底, 实位逐个覆写) —— node map 保洞
        let mut out = vec![Value::TheHole; len];
        for (i, slot) in out.iter_mut().enumerate() {
          if is_hole(i) {
            continue;
          }
          *slot = call(i, elem(i), Vec::new())?;
        }
        Ok(Value::new_array_with_elements(out))
      }
      ArrayIterKind::ForEach => {
        for i in 0..len {
          if is_hole(i) {
            continue;
          }
          call(i, elem(i), Vec::new())?;
        }
        Ok(Value::Undefined)
      }
      ArrayIterKind::Filter => {
        let mut out = Vec::new();
        for i in 0..len {
          if is_hole(i) {
            continue;
          }
          let element = elem(i);
          if call(i, element.clone(), Vec::new())?.to_boolean() {
            out.push(element);
          }
        }
        Ok(Value::new_array_with_elements(out))
      }
      ArrayIterKind::Every => {
        for i in 0..len {
          if is_hole(i) {
            continue;
          }
          if !call(i, elem(i), Vec::new())?.to_boolean() {
            return Ok(Value::Boolean(false));
          }
        }
        Ok(Value::Boolean(true))
      }
      ArrayIterKind::Some => {
        for i in 0..len {
          if is_hole(i) {
            continue;
          }
          if call(i, elem(i), Vec::new())?.to_boolean() {
            return Ok(Value::Boolean(true));
          }
        }
        Ok(Value::Boolean(false))
      }
      // find 交出命中的元素本身 (不是新数组也不是布尔), 未命中 undefined
      ArrayIterKind::Find => {
        for i in 0..len {
          if is_hole(i) {
            continue;
          }
          let element = elem(i);
          if call(i, element.clone(), Vec::new())?.to_boolean() {
            return Ok(element);
          }
        }
        Ok(Value::Undefined)
      }
      // findIndex 与 find 同一遍历, 只把命中换成下标 (未命中 -1)
      ArrayIterKind::FindIndex => {
        for i in 0..len {
          if is_hole(i) {
            continue;
          }
          let element = elem(i);
          if call(i, element, Vec::new())?.to_boolean() {
            return Ok(Value::Smi(i as i32));
          }
        }
        Ok(Value::Smi(-1))
      }
      ArrayIterKind::Reduce => {
        // 缺省与显式传 undefined 是两回事: 前者取**首个实位**作累子并从其后起 (洞不作累子),
        // 后者照原样参与回调
        let (mut acc, start) = match args.get(1) {
          Some(initial) => (initial.clone(), 0),
          None => {
            let mut k = 0;
            while k < len && is_hole(k) {
              k += 1;
            }
            if k >= len {
              return Err(Error::ExecutionError(
                "TypeError: Reduce of empty array with no initial value".to_string(),
              ));
            }
            (elem(k), k + 1)
          }
        };
        for i in start..len {
          if is_hole(i) {
            continue;
          }
          acc = call(i, elem(i), vec![acc])?;
        }
        Ok(acc)
      }
      // reduceRight 同款但自尾反向: 缺省取末个实位作累子、向其前反向 (洞不作累子)
      ArrayIterKind::ReduceRight => {
        let (mut acc, top) = match args.get(1) {
          Some(initial) => (initial.clone(), len),
          None => {
            let mut k = len;
            loop {
              if k == 0 {
                return Err(Error::ExecutionError(
                  "TypeError: Reduce of empty array with no initial value".to_string(),
                ));
              }
              k -= 1;
              if !is_hole(k) {
                break;
              }
            }
            (elem(k), k)
          }
        };
        for i in (0..top).rev() {
          if is_hole(i) {
            continue;
          }
          acc = call(i, elem(i), vec![acc])?;
        }
        Ok(acc)
      }
      // flatMap: 回调结果只压平一层 (深度固定 1), 非数组原样入列
      ArrayIterKind::FlatMap => {
        let mut out = Vec::with_capacity(len);
        for i in 0..len {
          if is_hole(i) {
            continue;
          }
          let element = elem(i);
          let ret = call(i, element, Vec::new())?;
          if let Value::Array(inner) = &ret {
            out.extend(read_elems(inner).iter().cloned());
          } else {
            out.push(ret);
          }
        }
        Ok(Value::new_array_with_elements(out))
      }
    }
  }

  /// 函数调用通用入口 (借用外壳): 冷调用点接口不变, clone/to_vec 只在这一处边界发生;
  /// 热臂 (Call* / 属性读写 accessor) 直连 `call_function_moved`, 免掉这份边界拷贝
  pub fn call_function(
    &mut self,
    callable: &Value,
    receiver: &Value,
    args: &[Value],
  ) -> Result<Value> {
    let mut owned = args.to_vec();
    self.call_function_moved(callable.clone(), receiver.clone(), &mut owned)
  }

  /// 函数调用移动核: receiver 按值进帧、args 逐槽 take 后 move —— 每个实参与
  /// callable/receiver 各比借用版省一次引用计数 (采样里 clone+drop 合计约 33%)
  pub fn call_function_moved(
    &mut self,
    callable: Value,
    receiver: Value,
    args: &mut [Value],
  ) -> Result<Value> {
    if callable == *FUNCTION_CALL {
      let target_fn = receiver;
      let target_receiver = if args.is_empty() {
        Value::Undefined
      } else {
        take(&mut args[0])
      };
      let tail_start = args.len().min(1);
      return self.call_function_moved(target_fn, target_receiver, &mut args[tail_start..]);
    }
    if callable == *FUNCTION_APPLY {
      let target_fn = receiver;
      let target_receiver = if args.is_empty() {
        Value::Undefined
      } else {
        take(&mut args[0])
      };
      if let Some(Value::Array(arr)) = args.get(1) {
        // 先脱锁做快照: 持读锁跨整个用户回调时, 被调函数写回同一个数组会自堵
        let mut argv = read_elems(arr).clone();
        return self.call_function_moved(target_fn, target_receiver, &mut argv);
      }
      return self.call_function_moved(target_fn, target_receiver, &mut []);
    }
    // sort 的比较器是用户函数, 只有持有分发器就地排序时才能回调。
    // 规范: 比较器只接受 undefined 或缺省, 其余不可调用值一律 TypeError
    if let Value::Native(NativeFn::ArraySort(arr), _) = &callable {
      let comparator = match args.first() {
        None | Some(Value::Undefined) => None,
        Some(cb @ (Value::Function(_) | Value::Native(..))) => Some(cb),
        Some(other) => {
          return Err(Error::ExecutionError(format!(
            "sort: comparator must be either a function or undefined: {}",
            other.to_str_lossy()
          )));
        }
      };
      return self.array_sort(arr, comparator);
    }
    // 迭代类内建同理: 回调要经分发器, 故也在调用点截获
    if let Value::Native(NativeFn::ArrayIter(kind, arr), _) = &callable {
      return self.array_iter(*kind, arr, args);
    }
    // replace 的函数替换实参同理 (22.1.3.25: 回调实参 = 全匹配, 组..., offset, 全串)
    if let Value::Native(NativeFn::StringReplace(s), _) = &callable
      && matches!(args.get(1), Some(Value::Function(_) | Value::Native(..)))
    {
      return Value::string_replace_core(s, args, &mut |subst, cb_args| {
        self
          .call_function(subst, &Value::Undefined, cb_args)
          .map(|v| v.to_string_lossy())
      });
    }
    match callable {
      Value::Native(nat, _) => nat.call(&receiver, args),
      Value::Function(closure) => {
        let func = Arc::clone(&closure.func);
        let old_acc = replace(&mut self.accumulator, Value::Undefined);

        let saved_reg_base = self.frame_reg_base;
        let saved_param_base = self.frame_param_base;
        let saved_param_len = self.frame_param_len;
        let saved_frame_arguments_len = self.frame_arguments_len;
        let saved_frame_offset = self.frame_offset;

        let reg_count = func.bytecode_array.register_count();
        let new_reg_base = self.reg_stack.len();
        self
          .reg_stack
          .resize(new_reg_base + reg_count, Value::Undefined);

        // rest 形参 (12.40): 尾段快照必须在 args 被逐槽移空之前取;
        // 箭头帧同样吃自身调用实参 (与词法 arguments 快照互不相干)
        let rest_snapshot = func.rest_binding.map(|(start, rest_reg)| {
          let tail = args.get(start.min(args.len())..).unwrap_or(&[]);
          (rest_reg, Value::new_array_with_elements(tail.to_vec()))
        });

        let new_param_base = self.param_stack.len();
        // 箭头无视调用点 receiver, 用创建时刻定格的词法 this (对标原版 JSFunction.context 携带的 this)
        let frame_receiver = if func.is_arrow {
          closure.lexical_this.clone()
        } else {
          receiver
        };
        self.param_stack.push(frame_receiver);
        for slot in args.iter_mut() {
          let val = take(slot);
          self.param_stack.push(val);
        }
        let new_param_len = args.len() + 1;

        self.frame_reg_base = new_reg_base;
        self.frame_param_base = new_param_base;
        self.frame_param_len = new_param_len;
        self.frame_arguments_len = args.len();

        if let Some((rest_reg, rest_arr)) = rest_snapshot
          && let Some(slot) = self.reg_stack.get_mut(new_reg_base + rest_reg as usize)
        {
          *slot = rest_arr;
        }

        let saved_arguments_materialized = self.arguments_materialized;
        let saved_global_arguments = self.saved_arguments.take();
        self.arguments_materialized = false;
        // 进箭头帧: 词法实参源换成创建时刻的快照 (箭头里再建箭头也继承同一快照)
        let saved_frame_arguments_lex = replace(
          &mut self.frame_arguments_lex,
          func.is_arrow.then(|| closure.lexical_arguments.clone()),
        );

        let saved_context = replace(&mut self.cur_context, Self::frame_context(&closure));

        let res = self.run_bytecode_array(&func.bytecode_array);

        self.cur_context = saved_context;
        self.frame_arguments_lex = saved_frame_arguments_lex;

        if self.arguments_materialized {
          let mut table = self.globals.write();
          if let Some(old) = self.saved_arguments.take() {
            table.insert("arguments".to_string(), old);
          } else {
            table.remove("arguments");
          }
        }
        self.arguments_materialized = saved_arguments_materialized;
        self.saved_arguments = saved_global_arguments;

        self.reg_stack.truncate(new_reg_base);
        self.param_stack.truncate(new_param_base);
        self.frame_reg_base = saved_reg_base;
        self.frame_param_base = saved_param_base;
        self.frame_param_len = saved_param_len;
        self.frame_arguments_len = saved_frame_arguments_len;
        self.frame_offset = saved_frame_offset;

        self.accumulator = old_acc;
        res
      }
      other => Err(Error::ExecutionError(format!(
        "TypeError: {other:?} is not callable (receiver: {receiver:?})"
      ))),
    }
  }

  /// 构造函数调用通用入口 (借用外壳, 同 call_function 的边界口径)
  pub fn construct(&mut self, ctor: &Value, args: &[Value]) -> Result<Value> {
    let mut owned = args.to_vec();
    self.construct_moved(ctor.clone(), &mut owned)
  }

  /// 构造调用移动核: ctor 与 args 按值下传 call_function_moved, 全程零实参克隆
  fn construct_moved(&mut self, ctor: Value, args: &mut [Value]) -> Result<Value> {
    if let Value::Native(nat, props) = &ctor {
      let new_obj = Value::new_object();
      {
        let guard = props.read();
        if let Some(proto) = guard.get(PROTOTYPE_KEY)
          && matches!(proto, Value::Object(_) | Value::Array(_))
        {
          let _ = new_obj.set_property(PROTO_KEY, proto.clone());
        }
      }
      // 原生构造器一律新建对象, 不读 receiver
      let res = nat.call(&Value::Undefined, args)?;
      if matches!(res, Value::Object(_) | Value::Array(_) | Value::RegExp(..)) {
        // C-2b 第二刀: 宿主 RegExp 构造器交回引擎正则变体 (new RegExp / 字面量 $$RegExp 路)
        // —— node 口径实例 [[Prototype]] 挂 RegExp.prototype; 变体无对象表, 把 new_obj 已挂
        // 好的 __proto__ 链搬进袋 (枚举面按 PROTO_KEY 滤掉, instanceof/`in` 经袋走链)
        if let (Value::RegExp(_, bag), Value::Object(new_map)) = (&res, &new_obj)
          && let Some(proto) = new_map.read().get(PROTO_KEY).cloned()
        {
          bag.write().insert(PROTO_KEY.to_string(), proto);
        }
        return Ok(res);
      }
      return Ok(Value::boxed_primitive(new_obj, res));
    }
    // 箭头没有 [[Construct]]: node 口径 TypeError: xxx is not a constructor
    if matches!(&ctor, Value::Function(closure) if closure.func.is_arrow) {
      return Err(Error::ExecutionError(
        "TypeError: 箭头函数不可被 new (arrow functions are not constructors)".to_string(),
      ));
    }
    if !matches!(ctor, Value::Function(_)) {
      return Err(Error::UnsupportedAstNode("value is not a constructor"));
    }
    let new_obj = Value::new_object();
    if let Ok(proto) = ctor.get_property(PROTOTYPE_KEY)
      && matches!(proto, Value::Object(_) | Value::Array(_))
    {
      let _ = new_obj.set_property(PROTO_KEY, proto);
    }
    let res = self.call_function_moved(ctor, new_obj.clone(), args)?;
    if matches!(res, Value::Object(_) | Value::Array(_) | Value::RegExp(..)) {
      Ok(res)
    } else {
      Ok(new_obj)
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
    self.frame_arguments_len = args.len();
    self.pending_exception = None;
    self.frame_arguments_lex = None;
    self.run_bytecode_array(bytecode_array)
  }

  /// 执行一帧字节码, 并在异常完成时按原版语义沿本帧 HandlerTable 展开
  ///
  /// 对标原版: 任何异常完成 (不区分 call 类指令、`throw`、属性缺失) 都要经过栈展开;
  /// 指令分派本身不做跳转, 因此生成器不必为每种错误各写一遍查表逻辑
  fn run_bytecode_array(&mut self, bytecode_array: &BytecodeArray) -> Result<Value> {
    let mut offset = 0;
    loop {
      match self.run_from(bytecode_array, offset) {
        Ok(val) => return Ok(val),
        Err(err) => {
          let Some(target) = bytecode_array.find_catch_handler(self.frame_offset) else {
            // 本帧无处理程序: 待抛值原样留给外层帧, 由宿主收到最终 Err
            return Err(err);
          };
          // 引擎自身 raised 的错误也必须落成 Error 实例, 否则 catch 到的是裸字符串
          self.accumulator = match self.pending_exception.take() {
            Some(thrown) => thrown,
            None => self.error_object(&err),
          };
          offset = target;
        }
      }
    }
  }

  /// 从 `start_offset` 起分派字节码, 正常完成时返回累加器
  #[inline(never)]
  fn run_from(&mut self, bytecode_array: &BytecodeArray, start_offset: usize) -> Result<Value> {
    let mut it = BytecodeArrayIterator::new(bytecode_array.bytecodes());
    if start_offset > 0 {
      it.set_offset(start_offset)?;
    }
    while !it.done() {
      self.frame_offset = it.current_offset();
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
              ConstantPoolEntry::String(s) => Value::new_string(s.clone()),
              ConstantPoolEntry::BigInt(b) => Value::BigInt(b.to_string()),
              ConstantPoolEntry::Function(f) => {
                // 闭包在此刻定格当前帧环境, 内层函数之后读写外层变量全靠它回溯
                if f.is_arrow {
                  // 箭头的 this 同样在创建时刻定格: 取本帧 receiver (顶层帧参数 0 越界回落 Undefined)
                  let lexical_this = self
                    .get_register_ref(Register::receiver())
                    .cloned()
                    .unwrap_or(Value::Undefined);
                  // 实参快照与 this 同刻定格: 箭头帧继承既有源, 普通帧取本帧实参,
                  // 脚本顶层 (frame_param_base=0) 没有外层函数 → None, 体内 arguments 走未绑定报错
                  let lexical_arguments = match &self.frame_arguments_lex {
                    Some(inherited) => inherited.clone(),
                    None if self.frame_param_base > 0 => {
                      let slice_len = self.frame_param_len.saturating_sub(1);
                      let start = self.frame_param_base + 1;
                      let end = start + slice_len;
                      Some(if end <= self.param_stack.len() {
                        self.param_stack[start..end].to_vec()
                      } else {
                        Vec::new()
                      })
                    }
                    None => None,
                  };
                  Value::new_arrow_function(
                    Arc::clone(f),
                    self.cur_context.clone(),
                    lexical_this,
                    lexical_arguments,
                  )
                } else {
                  Value::new_function_with_context(Arc::clone(f), self.cur_context.clone())
                }
              }
            };
            it.advance();
          } else {
            return Err(Error::ConstantPoolOutOfBounds(idx));
          }
        }
        bc @ (Bytecode::LdaGlobal | Bytecode::LdaGlobalInsideTypeof) => {
          // typeof 语境下未声明名解析为 Undefined, 其余读法必须抛 ReferenceError:
          // 静默 Undefined 会把拼写错误吞成 NaN, 与宿主 AST 求值器不一致
          let throws_if_absent = matches!(bc, Bytecode::LdaGlobal);
          let idx = it.get_unsigned_operand(0)? as usize;
          let name = Self::name_constant(bytecode_array, idx)?;
          let mut materialized = None;
          if name == "arguments" && !self.arguments_materialized {
            // 箭头帧 (12.28): arguments 取创建时刻定格的词法快照; 顶层箭头没有快照 → 未绑定,
            // 走普通名解析 (node: ReferenceError)。普通帧照旧取本帧参数切片
            materialized = match &self.frame_arguments_lex {
              Some(inner) => inner
                .as_ref()
                .map(|vals| Value::new_array_with_elements(vals.clone())),
              None => {
                // 长度用进帧快照 frame_arguments_len, 不用 frame_param_len:
                // 默认值降级体回写形参寄存器会撑大后者 (12.39)
                let slice_len = self.frame_arguments_len;
                let start = self.frame_param_base + 1;
                let end = start + slice_len;
                let args_slice = if end <= self.param_stack.len() {
                  &self.param_stack[start..end]
                } else {
                  &[]
                };
                Some(Value::new_array_with_elements(args_slice.to_vec()))
              }
            };
            if let Some(val) = &materialized {
              self.saved_arguments = self
                .globals
                .write()
                .insert("arguments".to_string(), val.clone());
              self.arguments_materialized = true;
            }
          }
          if let Some(val) = materialized {
            self.accumulator = val;
          } else if let Some(val) = self.resolve_name(name) {
            self.accumulator = val;
          } else if throws_if_absent {
            return Err(Error::ExecutionError(format!(
              "ReferenceError: {name} is not defined"
            )));
          } else {
            self.accumulator = Value::Undefined;
          }
          it.advance();
        }
        Bytecode::StaGlobal => {
          let idx = it.get_unsigned_operand(0)? as usize;
          let name = Self::name_constant(bytecode_array, idx)?;
          let val = self.accumulator.clone();
          self.assign_name(name, val);
          it.advance();
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
          let name = Self::name_constant(bytecode_array, idx)?;
          // 借寄存器而不是克隆: 属性读是热路径上最频繁的臂之一, 克隆 receiver 每次要多付
          // 一对引用计数 (采样里 clone+drop 合计约 33%, 这是其中可直接摘掉的一块)
          let getter = {
            let obj = self.get_register_ref(obj_reg)?;
            Self::check_property_receiver(obj, name, false)?;
            Self::lookup_accessor(obj, name, GETTER_PREFIX).map(|g| (g, obj.clone()))
          };
          self.accumulator = match getter {
            // accessor 命中才需要拥有权: 重入分发要 &mut self, 寄存器借用必须先放掉
            Some((func, receiver)) => self.call_function_moved(func, receiver, &mut [])?,
            None => {
              let obj = self.get_register_ref(obj_reg)?;
              self.get_property_resolved(obj, name)?
            }
          };
          it.advance();
        }
        Bytecode::SetNamedProperty => {
          let obj_reg = it.get_register_operand(0)?;
          let idx = it.get_unsigned_operand(1)? as usize;
          let name = Self::name_constant(bytecode_array, idx)?;
          // 同 GetNamedProperty: receiver 一律借用, 只有 setter 命中那一条路径才克隆
          let (setter, watch_recv) = {
            let obj = self.get_register_ref(obj_reg)?;
            Self::check_property_receiver(obj, name, true)?;
            // #24: 名字命中内建表的罕见分支才拾 Arc 强引用, &mut self 的记账放在寄存器
            // 借用放掉之后做
            let watch_recv = if Self::is_array_builtin_name(name) {
              match obj {
                Value::Object(o) => Some(Arc::clone(o)),
                _ => None,
              }
            } else {
              None
            };
            (
              Self::lookup_accessor(obj, name, SETTER_PREFIX).map(|s| (s, obj.clone())),
              watch_recv,
            )
          };
          if let Some(o) = watch_recv {
            self.watch_array_proto(&o, name);
          }
          match setter {
            Some((func, receiver)) => {
              let val = self.accumulator.clone();
              self.call_function_moved(func, receiver, &mut [val])?;
            }
            None => {
              let obj = self.get_register_ref(obj_reg)?;
              obj.set_property(name, self.accumulator.clone())?;
            }
          }
          it.advance();
        }
        // 字面量构建期的属性落表: 与 SetNamedProperty 的区别是**不打 setter** 且同名整体覆盖
        // 先前定义 (含 accessor 伪键), 语义见 define_own_entry。累加器按原版
        // ImplicitRegisterUse::kReadAndClobberAccumulator 交出后即作废 —— 省掉一次引用计数,
        // 生成器在字面量末尾本来就发 Ldar 把对象取回累加器
        Bytecode::DefineNamedOwnPropertyInLiteral => {
          let obj_reg = it.get_register_operand(0)?;
          let idx = it.get_unsigned_operand(1)? as usize;
          let name = Self::name_constant(bytecode_array, idx)?;
          let val = replace(&mut self.accumulator, Value::Undefined);
          let obj = self.get_register_ref(obj_reg)?;
          match obj {
            Value::Object(map) => define_own_entry(&mut map.write(), name, val),
            other => other.set_property(name, val)?,
          }
          it.advance();
        }
        Bytecode::DefineKeyedOwnProperty => {
          let obj_reg = it.get_register_operand(0)?;
          let key_reg = it.get_register_operand(1)?;
          let val = replace(&mut self.accumulator, Value::Undefined);
          let (key, obj) = {
            let key = self.get_register_ref(key_reg)?;
            (
              key.to_element_key().to_string(),
              self.get_register_ref(obj_reg)?,
            )
          };
          match obj {
            Value::Object(map) => define_own_entry(&mut map.write(), &key, val),
            other => other.set_property(&key, val)?,
          }
          it.advance();
        }
        Bytecode::GetKeyedProperty => {
          let obj_reg = it.get_register_operand(0)?;
          // 元素读同样是热路径最频繁的臂之一: receiver 一律借用, 不再克隆整个 Array/Object。
          // 快慢两路共用一次寄存器查找, 只在写 accumulator 时脱开借用 (accumulator 赋值要 &mut self)
          let (val, accessor) = {
            let obj = self.get_register_ref(obj_reg)?;
            if let Value::Array(arr) = obj
              && let Some(idx) = fast_elem_index(&self.accumulator)
            {
              (
                read_elems(arr)
                  .get(idx)
                  .map_or(Value::Undefined, Value::materialized),
                None,
              )
            } else {
              let key = self.accumulator.to_element_key();
              Self::check_property_receiver(obj, &key, false)?;
              match Self::lookup_accessor(obj, &key, GETTER_PREFIX) {
                // 命中 accessor: 函数与 receiver 带出块外再重入分发, 此处 val 占位不用
                Some(func) => (Value::Undefined, Some((func, obj.clone()))),
                None => (self.get_property_resolved(obj, &key)?, None),
              }
            }
          };
          self.accumulator = match accessor {
            Some((func, receiver)) => self.call_function_moved(func, receiver, &mut [])?,
            None => val,
          };
          it.advance();
        }
        Bytecode::SetKeyedProperty => {
          let obj_reg = it.get_register_operand(0)?;
          let key_reg = it.get_register_operand(1)?;
          let (setter, key_str, watch_recv) = {
            let obj = self.get_register_ref(obj_reg)?;
            let key = self.get_register_ref(key_reg)?;
            if let Value::Array(arr) = obj
              && let Some(idx) = fast_elem_index(key)
            {
              let mut guard = write_elems(arr);
              if idx >= guard.len() {
                // 中间空位交洞, 与 set_property 数字臂同口径
                guard.resize(idx + 1, Value::TheHole);
              }
              guard[idx] = Value::materialized(&self.accumulator);
              it.advance();
              continue;
            }
            let key_str = key.to_element_key();
            Self::check_property_receiver(obj, &key_str, true)?;
            // #24: 计算键写原型方法 (`Array.prototype[k] = f`) 同样记账, 口径同具名写
            let watch_recv = if Self::is_array_builtin_name(&key_str) {
              match obj {
                Value::Object(o) => Some(Arc::clone(o)),
                _ => None,
              }
            } else {
              None
            };
            (
              Self::lookup_accessor(obj, &key_str, SETTER_PREFIX).map(|s| (s, obj.clone())),
              key_str.into_owned(),
              watch_recv,
            )
          };
          if let Some(o) = watch_recv {
            self.watch_array_proto(&o, &key_str);
          }
          match setter {
            Some((func, receiver)) => {
              let val = self.accumulator.clone();
              self.call_function_moved(func, receiver, &mut [val])?;
            }
            None => {
              let obj = self.get_register_ref(obj_reg)?;
              obj.set_property(&key_str, self.accumulator.clone())?;
            }
          }
          it.advance();
        }
        Bytecode::DeletePropertySloppy | Bytecode::DeletePropertyStrict => {
          let obj_reg = it.get_register_operand(0)?;
          let key = self.accumulator.to_element_key().into_owned();
          let (ok, watch_recv) = {
            let obj = self.get_register_ref(obj_reg)?;
            // #24: 删原型上的内建名同样要让替身失能 (node: delete 后读回 undefined 而非原生)
            let watch_recv = if Self::is_array_builtin_name(&key) {
              match obj {
                Value::Object(o) => Some(Arc::clone(o)),
                _ => None,
              }
            } else {
              None
            };
            (obj.delete_property(&key), watch_recv)
          };
          if let Some(o) = watch_recv {
            self.watch_array_proto(&o, &key);
          }
          self.accumulator = Value::Boolean(ok);
          it.advance();
        }
        Bytecode::StaInArrayLiteral => {
          let arr_reg = it.get_register_operand(0)?;
          let idx_reg = it.get_register_operand(1)?;
          let arr = self.get_register_ref(arr_reg)?;
          let idx = self.get_register_ref(idx_reg)?;
          let idx = idx.to_u32() as usize;
          // 数组字面量的元素存是洞的唯一合法入口 (`[1,,2]` 的空位经 LdaTheHole 落槽),
          // 绕开 set_element 的写入侧物化; 运行期 `a[i]=x` 仍走兜底
          if let Value::Array(a) = &arr {
            let mut guard = write_elems(a);
            if idx >= guard.len() {
              guard.resize(idx + 1, Value::TheHole);
            }
            guard[idx] = self.accumulator.clone();
          } else {
            arr.set_element(idx, self.accumulator.clone())?;
          }
          it.advance();
        }
        Bytecode::CallProperty0 => {
          let callable_reg = it.get_register_operand(0)?;
          let receiver_reg = it.get_register_operand(1)?;
          let callable = self.get_register(callable_reg)?;
          let receiver = self.get_register(receiver_reg)?;
          let res = self.call_function_moved(callable, receiver, &mut []);
          self.accumulator = res?;
          it.advance();
        }
        Bytecode::CallProperty1 => {
          let callable_reg = it.get_register_operand(0)?;
          let receiver_reg = it.get_register_operand(1)?;
          let arg0_reg = it.get_register_operand(2)?;
          let callable = self.get_register(callable_reg)?;
          let receiver = self.get_register(receiver_reg)?;
          let arg0 = self.get_register(arg0_reg)?;
          let res = self.call_function_moved(callable, receiver, &mut [arg0]);
          self.accumulator = res?;
          it.advance();
        }
        Bytecode::CallProperty2 => {
          let callable_reg = it.get_register_operand(0)?;
          let receiver_reg = it.get_register_operand(1)?;
          let arg0_reg = it.get_register_operand(2)?;
          let arg1_reg = it.get_register_operand(3)?;
          let callable = self.get_register(callable_reg)?;
          let receiver = self.get_register(receiver_reg)?;
          let arg0 = self.get_register(arg0_reg)?;
          let arg1 = self.get_register(arg1_reg)?;
          let res = self.call_function_moved(callable, receiver, &mut [arg0, arg1]);
          self.accumulator = res?;
          it.advance();
        }
        Bytecode::CallProperty => {
          let callable_reg = it.get_register_operand(0)?;
          let first_arg_reg = it.get_register_operand(1)?;
          let arg_count = it.get_unsigned_operand(2)? as usize;
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
            self.call_function_moved(callable, receiver, &mut small_args[..call_args_len])
          } else {
            let mut call_args = Vec::with_capacity(call_args_len);
            for i in 0..call_args_len {
              let reg = Register::new(first_arg_reg.index() + (i + 1) as i32);
              call_args.push(self.get_register(reg)?);
            }
            self.call_function_moved(callable, receiver, &mut call_args)
          };
          self.accumulator = res?;
          it.advance();
        }
        Bytecode::CallUndefinedReceiver0 => {
          let callable_reg = it.get_register_operand(0)?;
          let callable = self.get_register(callable_reg)?;
          let res = self.call_function_moved(callable, Value::Undefined, &mut []);
          self.accumulator = res?;
          it.advance();
        }
        Bytecode::CallUndefinedReceiver1 => {
          let callable_reg = it.get_register_operand(0)?;
          let arg0_reg = it.get_register_operand(1)?;
          let callable = self.get_register(callable_reg)?;
          let arg0 = self.get_register(arg0_reg)?;
          let res = self.call_function_moved(callable, Value::Undefined, &mut [arg0]);
          self.accumulator = res?;
          it.advance();
        }
        Bytecode::CallUndefinedReceiver2 => {
          let callable_reg = it.get_register_operand(0)?;
          let arg0_reg = it.get_register_operand(1)?;
          let arg1_reg = it.get_register_operand(2)?;
          let callable = self.get_register(callable_reg)?;
          let arg0 = self.get_register(arg0_reg)?;
          let arg1 = self.get_register(arg1_reg)?;
          let res = self.call_function_moved(callable, Value::Undefined, &mut [arg0, arg1]);
          self.accumulator = res?;
          it.advance();
        }
        Bytecode::CallUndefinedReceiver => {
          let callable_reg = it.get_register_operand(0)?;
          let first_arg_reg = it.get_register_operand(1)?;
          let arg_count = it.get_unsigned_operand(2)? as usize;
          let callable = self.get_register(callable_reg)?;
          let res = if arg_count <= 8 {
            let mut small_args: [Value; 8] = Default::default();
            for (i, slot) in small_args[..arg_count].iter_mut().enumerate() {
              let reg = Register::new(first_arg_reg.index() + i as i32);
              *slot = self.get_register(reg)?;
            }
            self.call_function_moved(callable, Value::Undefined, &mut small_args[..arg_count])
          } else {
            let mut call_args = Vec::with_capacity(arg_count);
            for i in 0..arg_count {
              let reg = Register::new(first_arg_reg.index() + i as i32);
              call_args.push(self.get_register(reg)?);
            }
            self.call_function_moved(callable, Value::Undefined, &mut call_args)
          };
          self.accumulator = res?;
          it.advance();
        }
        Bytecode::Construct => {
          let ctor_reg = it.get_register_operand(0)?;
          let first_arg_reg = it.get_register_operand(1)?;
          let arg_count = it.get_unsigned_operand(2)? as usize;
          let ctor = self.get_register(ctor_reg)?;
          let res = if arg_count <= 8 {
            let mut small_args: [Value; 8] = Default::default();
            for (i, slot) in small_args[..arg_count].iter_mut().enumerate() {
              let reg = Register::new(first_arg_reg.index() + i as i32);
              *slot = self.get_register(reg)?;
            }
            self.construct_moved(ctor, &mut small_args[..arg_count])
          } else {
            let mut ctor_args = Vec::with_capacity(arg_count);
            for i in 0..arg_count {
              let reg = Register::new(first_arg_reg.index() + i as i32);
              ctor_args.push(self.get_register(reg)?);
            }
            self.construct_moved(ctor, &mut ctor_args)
          };
          self.accumulator = res?;
          it.advance();
        }
        Bytecode::CreateArrayLiteral => {
          self.accumulator = Value::new_array();
          it.advance();
        }
        Bytecode::CreateEmptyArrayLiteral => {
          self.accumulator = Value::new_array();
          it.advance();
        }
        Bytecode::CreateObjectLiteral => {
          // Flag8 是生成期算好的属性数 (见 visit_object_literal)。字面量对象不落 `__proto__`
          // (原型继承由 has_property_resolved 末尾的 Object.prototype 兜底), 属性数里已含 accessor
          // 哨兵, 所以按这个数正好铺够; 超过 255 的部分由 hashbrown 自然扩容接住
          let cap = it.get_flag8_operand(2)? as usize;
          self.accumulator = if cap == 0 {
            Value::new_object()
          } else {
            Value::new_object_with(cap)
          };
          it.advance();
        }
        Bytecode::CreateEmptyObjectLiteral => {
          self.accumulator = Value::new_object();
          it.advance();
        }
        Bytecode::Add => {
          let reg = it.get_register_operand(0)?;
          // Add 的 hint 是 default (Date 因此走 toString), 其余算术是 number
          self.numeric_bin_op(reg, false, Value::add)?;
          it.advance();
        }
        Bytecode::Sub => {
          let reg = it.get_register_operand(0)?;
          self.numeric_bin_op(reg, true, Value::sub)?;
          it.advance();
        }
        Bytecode::Mul => {
          let reg = it.get_register_operand(0)?;
          self.numeric_bin_op(reg, true, Value::mul)?;
          it.advance();
        }
        Bytecode::Div => {
          let reg = it.get_register_operand(0)?;
          self.numeric_bin_op(reg, true, Value::div)?;
          it.advance();
        }
        Bytecode::Mod => {
          let reg = it.get_register_operand(0)?;
          self.numeric_bin_op(reg, true, Value::mod_op)?;
          it.advance();
        }
        Bytecode::Exp => {
          let reg = it.get_register_operand(0)?;
          self.numeric_bin_op(reg, true, Value::exp)?;
          it.advance();
        }
        Bytecode::BitwiseOr => {
          let reg = it.get_register_operand(0)?;
          self.numeric_bin_op(reg, true, Value::bitwise_or)?;
          it.advance();
        }
        Bytecode::BitwiseAnd => {
          let reg = it.get_register_operand(0)?;
          self.numeric_bin_op(reg, true, Value::bitwise_and)?;
          it.advance();
        }
        Bytecode::BitwiseXor => {
          let reg = it.get_register_operand(0)?;
          self.numeric_bin_op(reg, true, Value::bitwise_xor)?;
          it.advance();
        }
        Bytecode::ShiftLeft => {
          let reg = it.get_register_operand(0)?;
          self.numeric_bin_op(reg, true, Value::shift_left)?;
          it.advance();
        }
        Bytecode::ShiftRight => {
          let reg = it.get_register_operand(0)?;
          self.numeric_bin_op(reg, true, Value::shift_right)?;
          it.advance();
        }
        Bytecode::ShiftRightLogical => {
          let reg = it.get_register_operand(0)?;
          self.numeric_bin_op(reg, true, Value::shift_right_logical)?;
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
          let primitive = self.acc_primitive()?;
          self.accumulator = primitive.negate();
          it.advance();
        }
        Bytecode::BitwiseNot => {
          let primitive = self.acc_primitive()?;
          self.accumulator = primitive.bitwise_not();
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
          let primitive = self.acc_primitive()?;
          self.accumulator = Value::Number(primitive.to_number());
          it.advance();
        }
        Bytecode::TestEqual => {
          let reg = it.get_register_operand(0)?;
          // 规范 IsLooselyEqual 的类型短路。`x == null` 是解释器循环里最高频的比较之一, 若先去
          // 转换, 每次 valueOf/toString 属性回溯会把整段循环拖死
          let lhs = self.get_register_ref(reg)?;
          if matches!(lhs, Value::Null | Value::Undefined)
            || matches!(self.accumulator, Value::Null | Value::Undefined)
          {
            self.accumulator = Value::Boolean(lhs.test_equal(&self.accumulator));
            it.advance();
            continue;
          }
          // 两侧同为对象时 `==` 就是身份比较, 谁都不许走 ToPrimitive: 否则任意两个普通对象先被
          // 转成 "[object Object]" 再判相等, `c != determining` 恒假 (deltablue 建不出计划链)
          if lhs.is_object_like() && self.accumulator.is_object_like() {
            self.accumulator = Value::Boolean(lhs.test_equal_strict(&self.accumulator));
            it.advance();
            continue;
          }
          self.compare_bin_op(reg, false, Value::test_equal)?;
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
          self.compare_bin_op(reg, true, Value::test_less_than)?;
          it.advance();
        }
        Bytecode::TestGreaterThan => {
          let reg = it.get_register_operand(0)?;
          self.compare_bin_op(reg, true, Value::test_greater_than)?;
          it.advance();
        }
        Bytecode::TestLessThanOrEqual => {
          let reg = it.get_register_operand(0)?;
          self.compare_bin_op(reg, true, Value::test_less_than_or_equal)?;
          it.advance();
        }
        Bytecode::TestGreaterThanOrEqual => {
          let reg = it.get_register_operand(0)?;
          self.compare_bin_op(reg, true, Value::test_greater_than_or_equal)?;
          it.advance();
        }
        Bytecode::TestInstanceOf => {
          let reg = it.get_register_operand(0)?;
          // 两侧都只读: 借寄存器值, 省掉一次整值克隆 (原型链回溯本身不产出新值)
          let is_inst = {
            let instance = self.get_register_ref(reg)?;
            self.is_instance_of(instance, &self.accumulator)
          };
          self.accumulator = Value::Boolean(is_inst);
          it.advance();
        }
        Bytecode::TestIn => {
          let reg = it.get_register_operand(0)?;
          let prop_str = {
            let prop_val = self.get_register_ref(reg)?;
            prop_val.to_element_key()
          };
          // 对标规范: `in` 的右操作数不是对象就是 TypeError, 原先对原始值静默给 false
          if !matches!(
            self.accumulator,
            Value::Object(_)
              | Value::Array(_)
              | Value::Function(..)
              | Value::Native(..)
              | Value::RegExp(..)
          ) {
            return Err(Error::ExecutionError(format!(
              "TypeError: Cannot use 'in' operator to search for '{prop_str}' in {}",
              self.accumulator.to_string_lossy()
            )));
          }
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
          self.accumulator = Value::new_string(self.accumulator.type_of().to_string());
          it.advance();
        }
        Bytecode::Throw | Bytecode::ReThrow => {
          // 只记录待抛值并终止本帧, 由 run_bytecode_array 统一查 HandlerTable 展开
          let thrown = take(&mut self.accumulator);
          let msg = match &thrown {
            Value::Object(o) => {
              let g = o.read();
              match g.get("message") {
                Some(m) => format!("Error: {}", m.to_string_lossy()),
                None => format!("{:?}", g),
              }
            }
            other => other.to_string_lossy(),
          };
          self.pending_exception = Some(thrown);
          return Err(Error::ExecutionError(msg));
        }
        Bytecode::ForInEnumerate => {
          let receiver_reg = it.get_register_operand(0)?;
          let keys: Vec<Value> = match self.get_register_ref(receiver_reg)? {
            Value::Object(obj) => {
              let guard = obj.read();
              enumerable_names(guard.keys())
                .into_iter()
                .map(Value::new_string)
                .collect()
            }
            // C-2b 第二刀: 正则枚举面 = 用户 expando 袋 (node `for (var p in /a/g)` 交空);
            // 派生访问器与 exec/test/toString 住内核面不进袋, 旧伪键过滤/墓碑机器随哨兵退役
            Value::RegExp(_, props) => {
              let guard = props.read();
              enumerable_names(guard.keys())
                .into_iter()
                .map(Value::new_string)
                .collect()
            }
            Value::Array(arr) => {
              let data = arr.read();
              let mut buf = ItoaBuffer::new();
              // 战役 C ③: 洞槽不进枚举面 (node `for (var p in [1,,2])` 只过 0/2)
              let mut keys: Vec<Value> = (0..data.elements.len())
                .filter(|&i| !matches!(data.elements[i], Value::TheHole))
                .map(|i| Value::new_string(buf.format(i).to_string()))
                .collect();
              // 索引键升序在前; 具名键只能给 HashMap 序, 规范要的插入序得等隐藏类 (战役 C)
              if let Some(props) = &data.props {
                keys.extend(props.keys().map(|k| Value::new_string(k.clone())));
              }
              keys
            }
            Value::String(s) => {
              let len = s.chars().count();
              let mut buf = ItoaBuffer::new();
              (0..len)
                .map(|i| Value::new_string(buf.format(i).to_string()))
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

          let keys_arr = self.accumulator.clone();
          let len = if let Value::Array(ref arr) = keys_arr {
            read_elems(arr).len() as i32
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
          let index_reg = it.get_register_operand(1)?;
          let pair_first = it.get_register_operand(2)?;
          let cache_array_reg = Register::new(pair_first.index() + 1);

          let index = self.get_register_ref(index_reg)?.to_i32();
          // 缓存数组只借不克隆: for-in 每轮都要走这里, 克隆整个 Array 的 Arc 是白付的
          let val = {
            let cache_array = self.get_register_ref(cache_array_reg)?;
            match cache_array {
              Value::Array(arr) => {
                let guard = read_elems(arr);
                if index >= 0 && (index as usize) < guard.len() {
                  guard[index as usize].clone()
                } else {
                  Value::Undefined
                }
              }
              _ => Value::Undefined,
            }
          };
          self.accumulator = val;
          it.advance();
        }
        Bytecode::ForInStep => {
          let index_reg = it.get_register_operand(0)?;
          let cur = self.get_register_ref(index_reg)?.to_i32();
          self.set_register(index_reg, Value::Smi(cur.saturating_add(1)))?;
          it.advance();
        }
        Bytecode::JumpIfForInDone | Bytecode::JumpIfForInDoneConstant => {
          let index_reg = it.get_register_operand(1)?;
          let length_reg = it.get_register_operand(2)?;
          let index = self.get_register_ref(index_reg)?.to_i32();
          let length = self.get_register_ref(length_reg)?.to_i32();
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

/// 22.1.3.25 GetSubstitution —— 替换串里 `$` 序列的展开 (两链唯一真相, 宿主 `String.replace` 直接复用)
///
/// `$$`→`$`, `$&`→全匹配, `` $` ``→前缀, `$'`→后缀, `$n`/`$nn`→第 n 个捕获组
/// (组号不存在时**原样保留**含 `$`, 存在但未参与给空串)。两位数优先于一位数按规范序试探
#[must_use]
pub fn get_substitution(
  tpl: &str,
  matched: &str,
  before: &str,
  after: &str,
  groups: &[Option<&str>],
) -> String {
  let bytes = tpl.as_bytes();
  let mut out = String::with_capacity(tpl.len() + matched.len());
  let mut i = 0usize;
  while i < bytes.len() {
    if bytes[i] != b'$' || i + 1 >= bytes.len() {
      let ch = tpl[i..].chars().next().unwrap_or('\0');
      out.push(ch);
      i += ch.len_utf8();
      continue;
    }
    let next = bytes[i + 1];
    if let Some(d1) = (next as char).to_digit(10) {
      let mut consumed = false;
      if d1 >= 1
        && i + 2 < bytes.len()
        && let Some(d2) = (bytes[i + 2] as char).to_digit(10)
      {
        let two = usize::try_from(d1 * 10 + d2).unwrap_or(usize::MAX);
        if two >= 10 && two <= groups.len() {
          out.push_str(groups[two - 1].unwrap_or(""));
          i += 3;
          consumed = true;
        }
      }
      if !consumed {
        let one = usize::try_from(d1).unwrap_or(usize::MAX);
        if one >= 1 && one <= groups.len() {
          out.push_str(groups[one - 1].unwrap_or(""));
          i += 2;
        } else {
          out.push('$');
          i += 1;
        }
      }
    } else {
      match next {
        b'$' => {
          out.push('$');
          i += 2;
        }
        b'&' => {
          out.push_str(matched);
          i += 2;
        }
        b'`' => {
          out.push_str(before);
          i += 2;
        }
        b'\'' => {
          out.push_str(after);
          i += 2;
        }
        _ => {
          out.push('$');
          i += 1;
        }
      }
    }
  }
  out
}
