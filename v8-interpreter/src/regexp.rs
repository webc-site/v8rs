//! 正则内核 (战役 C-2b 第一刀): `RegExpData` 拆家后的"内核"单一真相
//!
//! 宿主 `rs8::runtime::value::RegExpData` 自此只剩 `core: Arc<RegExpCore>` + expando 袋
//! (`props`, 宿主 JsValue 类型跨不过 crate 边界); pattern/flags/编译体/lastIndex 全部住这里,
//! 引擎第二刀 `Value::RegExp` 变体直持 `Arc<RegExpCore>` 即可跨桥浅拷贝共享身份。
//! 规范化编译与 `regex_split_parts` 从宿主逐字搬入 (一处定义, 两链共用)。

use std::{
  borrow::Cow,
  collections::HashMap,
  sync::{Arc, LazyLock},
};

use fancy_regex::Regex;
use parking_lot::{Mutex, RwLock};

use crate::{
  bytecode_dispatcher::{INDEX_KEY, INPUT_KEY, LAST_INDEX_KEY, NativeFn, Value},
  error::Error,
};

/// 编译体缓存 (战役 C-④a): 外层键 pattern、内层键 flags, 值 = 已编译 `Arc<Regex>`
///
/// V8 同款口径"每求值新实例 × 编译缓存": 字面量 per-eval 新建内核只付元数据价,
/// 重编译税一次付清。嵌套两级 map 让**命中路径零字符串分配** (`get(&str)` 借键查询);
/// 失败不入库 (每跳照抛, 与 node 口径一致); 规范化纯函数, 同键同产物, 缓存不引入可观测差异
static REGEXP_COMPILE_CACHE: LazyLock<Mutex<HashMap<String, HashMap<String, Arc<Regex>>>>> =
  LazyLock::new(|| Mutex::new(HashMap::new()));

/// 正则内核数据: 只含跨 crate 安全的字段 (宿主 expando 袋不在此列)
#[derive(Clone)]
pub struct RegExpCore {
  pub pattern: String,
  pub flags: String,
  pub global: bool,
  pub ignore_case: bool,
  pub multiline: bool,
  pub compiled: Arc<Regex>,
  pub last_index: Arc<RwLock<usize>>,
}

impl RegExpCore {
  /// 编译并创建正则内核 (原宿主 `JsValue::new_regexp` 的规范化段逐字搬入):
  /// `i`/`m` 折成 `(?i)(?m)` 前缀; 字符类外的 `[` 转义、`[[]` 内层 `[` 转义;
  /// `\xHH` ≥ 0x80 改写为 `\u{HH}` (fancy-regex 不接受字节转义的码位口径)
  ///
  /// 错误串为 fancy-regex 原文, 由宿主包成 `Invalid RegExp '<pattern>: <e>'`
  pub fn new(pattern: &str, flags: &str) -> Result<Self, String> {
    let global = flags.contains('g');
    let ignore_case = flags.contains('i');
    let multiline = flags.contains('m');

    // C-④a 编译缓存命中: 跳过规范化与重编译, 新内核只带自己的 last_index (命中零串分配)
    let cached = REGEXP_COMPILE_CACHE
      .lock()
      .get(pattern)
      .and_then(|by_flags| by_flags.get(flags))
      .cloned();
    if let Some(compiled) = cached {
      return Ok(Self {
        pattern: pattern.to_string(),
        flags: flags.to_string(),
        global,
        ignore_case,
        multiline,
        compiled,
        last_index: Arc::new(RwLock::new(0)),
      });
    }

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

    let compiled = Arc::new(Regex::new(&pat_to_compile).map_err(|e| e.to_string())?);
    REGEXP_COMPILE_CACHE.lock().insert(
      (pattern.to_string(), flags.to_string()),
      Arc::clone(&compiled),
    );

    Ok(Self {
      pattern: pattern.to_string(),
      flags: flags.to_string(),
      global,
      ignore_case,
      multiline,
      compiled,
      last_index: Arc::new(RwLock::new(0)),
    })
  }

  #[inline]
  pub fn as_str(&self) -> &str {
    &self.pattern
  }

  /// 布尔标志属性面的唯一真相: 名字 → 值 (宿主属性读与 bridge 落表两处共用, 一处定义)
  ///
  /// sticky/unicode/dotAll 没有独立字段, 从 flags 串就地判定; global/ignoreCase/multiline
  /// 是解析期落好的字段, 保留既有口径
  #[inline]
  pub fn boolean_flags(&self) -> [(&'static str, bool); 6] {
    [
      ("global", self.global),
      ("ignoreCase", self.ignore_case),
      ("multiline", self.multiline),
      ("sticky", self.flags.contains('y')),
      ("unicode", self.flags.contains('u')),
      ("dotAll", self.flags.contains('d')),
    ]
  }

  /// sticky 位: 无独立字段, 从 flags 串就地判定 (与 boolean_flags 同口径)
  #[inline]
  pub fn sticky(&self) -> bool {
    self.flags.contains('y')
  }

  /// lastIndex 是否参与匹配 (规范 RegExpBuiltinExec: global 或 sticky 之一才读/推进/复位)
  #[inline]
  pub fn tracks_last_index(&self) -> bool {
    self.global || self.sticky()
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

/// 规范 RegExpSplit 一处定义 (宿主 StringSplit 与桥端 @@split 两链共用)。
///
/// 空匹配规则: 整体命中零宽且起点 == q (上一刀口) 时不切, 前进一个字符边界重试;
/// 其余命中照常吐前段并逐个插入捕获组 (未参与组吐 undefined, ES2018 改写后口径)。
/// 全程 captures_from_pos 直接定位, 不读写 lastIndex (node 实测 split 无 lastIndex 副效应)
#[must_use]
pub fn regex_split_parts(text: &str, re: &RegExpCore, limit: Option<usize>) -> Vec<Option<String>> {
  let mut parts: Vec<Option<String>> = Vec::new();
  let size = text.len();
  let mut q = 0usize;
  let mut e = 0usize;
  while e < size {
    if limit.is_some_and(|lim| parts.len() >= lim) {
      return parts;
    }
    let Ok(Some(caps)) = re.compiled.captures_from_pos(text, e) else {
      break;
    };
    let Some(m0) = caps.get(0) else {
      break;
    };
    if m0.start() == m0.end() && m0.start() == q {
      e += 1;
      while e < size && !text.is_char_boundary(e) {
        e += 1;
      }
      continue;
    }
    parts.push(Some(text[q..m0.start()].to_string()));
    for i in 1..caps.len() {
      parts.push(caps.get(i).map(|m| m.as_str().to_string()));
    }
    q = m0.end();
    e = m0.end();
  }
  if limit.is_none_or(|lim| parts.len() < lim) {
    parts.push(Some(text[q..].to_string()));
  }
  parts
}

/// RegExpBuiltinExec 核心 (战役 C-2b 第二刀从桥 `regex_exec_core` 逐字搬入, 一处定义落引擎):
/// lastIndex 仅在 g/y 下读/推进/复位, sticky 要求整体命中恰锚在 lastIndex 否则视作未命中;
/// 命中统一双写 receiver 引擎对象属性 (JS 可见) + 内核 Arc (跨桥共享身份的唯一真相)。
/// receiver 为 Undefined / 显式传 args[1] (引擎内部 replace/split 循环) 时不读属性
pub fn regex_exec_core(
  re: &Arc<RegExpCore>,
  receiver: &Value,
  args: &[Value],
) -> Result<Value, Error> {
  let text_cow = match args.first() {
    Some(Value::String(s)) => Cow::Borrowed(&**s),
    Some(other) => Cow::Owned(other.to_string_lossy()),
    None => Cow::Borrowed(""),
  };
  let text = text_cow.as_ref();
  let from_pos = args
    .get(1)
    .map(|v| v.to_i32().max(0) as usize)
    .unwrap_or_else(|| {
      if re.tracks_last_index() {
        receiver
          .get_property(LAST_INDEX_KEY)
          .ok()
          .filter(|v| !matches!(v, Value::Undefined))
          .map(|v| v.to_i32().max(0) as usize)
          .unwrap_or_else(|| *re.last_index.read())
      } else {
        0
      }
    });
  let sync_last_index = |v: usize| {
    *re.last_index.write() = v;
    if matches!(receiver, Value::Object(_) | Value::RegExp(..)) {
      let _ = receiver.set_property(LAST_INDEX_KEY, Value::Smi(v as i32));
    }
  };
  if from_pos > text.len() {
    if re.tracks_last_index() {
      sync_last_index(0);
    }
    return Ok(Value::Null);
  }
  let caps_opt = match re.compiled.captures_from_pos(text, from_pos) {
    Ok(Some(caps)) => Some(caps),
    _ => None,
  };
  // sticky 锚定: 起点须恰为 lastIndex, 靠后的命中不认 (node /a/y 不回溯搜索)
  let hit =
    caps_opt.filter(|caps| !(re.sticky() && caps.get(0).map(|m| m.start()) != Some(from_pos)));
  if let Some(caps) = hit {
    let mut list = Vec::with_capacity(caps.len());
    for m in caps.iter() {
      if let Some(mat) = m {
        list.push(Value::new_string(mat.as_str()));
      } else {
        list.push(Value::Undefined);
      }
    }
    let whole = caps.get(0);
    if re.tracks_last_index()
      && let Some(w) = whole
    {
      sync_last_index(w.end());
    }
    let res = Value::new_array_with_elements(list);
    // exec 结果数组挂 index/input (码单位口径, 与宿主 RegExpExec 一致, #23)
    if let Some(w) = whole {
      let chars = text[..w.start()].chars().count();
      let _ = res.set_property(INDEX_KEY, Value::Number(chars as f64));
      let _ = res.set_property(INPUT_KEY, Value::new_string(text));
    }
    Ok(res)
  } else {
    if re.tracks_last_index() {
      sync_last_index(0);
    }
    Ok(Value::Null)
  }
}

/// node: test 内部同走 RegExpBuiltinExec, g/y 下与 exec 同步读/推进/复位 lastIndex,
/// 故直接委托 `regex_exec_core` (一处定义), 只把结果折叠成布尔
pub fn regex_test_core(
  re: &Arc<RegExpCore>,
  receiver: &Value,
  args: &[Value],
) -> Result<Value, Error> {
  let res = regex_exec_core(re, receiver, args)?;
  Ok(Value::Boolean(!matches!(res, Value::Null)))
}

/// `String.prototype.split(regexp)` 的内核侧实现 (原桥 @@split 哨兵闭包逐字搬入):
/// 不经属性面公开 "split" 键 (node 正则自有方法只有 exec/test, 规范那是 Symbol.split)
pub fn regex_split_core(
  re: &Arc<RegExpCore>,
  _receiver: &Value,
  args: &[Value],
) -> Result<Value, Error> {
  let text_cow = match args.first() {
    Some(Value::String(s)) => Cow::Borrowed(&**s),
    Some(other) => Cow::Owned(other.to_string_lossy()),
    None => Cow::Borrowed(""),
  };
  let text = text_cow.as_ref();
  let limit = match args.get(1) {
    Some(Value::Undefined) | None => None,
    Some(v) => {
      let n = v.to_u32();
      if n == u32::MAX {
        None
      } else {
        Some(n as usize)
      }
    }
  };
  let parts: Vec<Value> = regex_split_parts(text, re, limit)
    .into_iter()
    .map(|p| match p {
      Some(s) => Value::new_string(s),
      None => Value::Undefined,
    })
    .collect();
  Ok(Value::new_array_with_elements(parts))
}

/// 三个内核函数的原生构造器 (分发器 RegExp 臂与桥共用, 一处定义):
/// exec/test 携带 `Arc<RegExpCore>` 上下文, lastIndex 读写直落共享内核
#[inline]
pub fn regex_exec_native(core: &Arc<RegExpCore>) -> Value {
  Value::new_native(NativeFn::from_context(Arc::clone(core), regex_exec_core))
}

#[inline]
pub fn regex_test_native(core: &Arc<RegExpCore>) -> Value {
  Value::new_native(NativeFn::from_context(Arc::clone(core), regex_test_core))
}

#[inline]
pub fn regex_split_native(core: &Arc<RegExpCore>) -> Value {
  Value::new_native(NativeFn::from_context(Arc::clone(core), regex_split_core))
}
