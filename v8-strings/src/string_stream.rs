// 由 tool/src/strings/string-stream.h.js 从 v8/src/strings/string-stream.h + string-stream.cc 生成; 修订就地改, 复跑覆盖
// 常量与分配器/格式元素骨架 1:1; Put/Add 格式引擎与对象打印分层就地补写

//! 诊断字符串流: 定长/可增长分配器 + printf 风格格式引擎 (%s/%w/%k/%i/%d/%u/%x/%c/%X/%f/%g/%G/%e/%E/%p)。
//! 对象打印 (PrintName/PrintUsingMap/PrintFixedArray/...) 依赖堆对象, 归 objects 层。

use core::str::from_utf8;

/// 初始容量 (kInitialCapacity)。
pub const INITIAL_CAPACITY: usize = 16;
/// mentioned-object 缓存上限 (kMentionedObjectCacheMaxSize)。
pub const MENTIONED_OBJECT_CACHE_MAX_SIZE: usize = 256;

/// 分配器 (StringAllocator): allocate 一次分配, grow 扩容并把旧内容搬到新缓冲。
pub trait StringAllocator {
  /// 分配 bytes 字节。
  fn allocate(&mut self, bytes: usize) -> Option<&mut [u8]>;
  /// 扩容: bytes 进出参; 返回新缓冲 (含旧内容) 或 None (容量不变)。
  fn grow(&mut self, bytes: &mut usize) -> Option<&mut [u8]>;
}

/// 堆分配器 (HeapStringAllocator): Vec 承载, 扩容翻倍。
#[derive(Debug, Default)]
pub struct HeapStringAllocator {
  space: Vec<u8>,
}

impl StringAllocator for HeapStringAllocator {
  fn allocate(&mut self, bytes: usize) -> Option<&mut [u8]> {
    self.space.clear();
    self.space.resize(bytes, 0);
    Some(&mut self.space)
  }

  fn grow(&mut self, bytes: &mut usize) -> Option<&mut [u8]> {
    let new_bytes = bytes.saturating_mul(2);
    if new_bytes <= *bytes {
      return None; // 溢出: 保持旧缓冲
    }
    self.space.resize(new_bytes, 0);
    *bytes = new_bytes;
    Some(&mut self.space)
  }
}

/// 定长分配器 (FixedStringAllocator): 不扩容, grow 报告固定长度。
pub struct FixedStringAllocator<'a> {
  buffer: &'a mut [u8],
}

impl<'a> FixedStringAllocator<'a> {
  #[must_use]
  pub fn new(buffer: &'a mut [u8]) -> Self {
    Self { buffer }
  }
}

impl StringAllocator for FixedStringAllocator<'_> {
  fn allocate(&mut self, bytes: usize) -> Option<&mut [u8]> {
    if self.buffer.is_empty() {
      return None;
    }
    let len = self.buffer.len().min(bytes);
    Some(&mut self.buffer[..len])
  }

  fn grow(&mut self, bytes: &mut usize) -> Option<&mut [u8]> {
    *bytes = self.buffer.len();
    None
  }
}

/// 格式元素 (FmtElm); 对象/句柄变体归 objects 层, 此处保留 POD 形态。
#[derive(Debug, Clone, Copy)]
pub enum FmtElm<'a> {
  Int(i32),
  Double(f64),
  /// %s。
  CStr(&'a str),
  /// %w: uc16 串按字节窄化输出。
  LcStr(&'a [u16]),
  /// %k: 字符值按可打印性选 \\xNN / \\uNNNN。
  Char(i32),
  /// %o: 对象短打印 (objects 层注入)。
  Obj(usize),
  /// %p。
  Pointer(usize),
}

/// 控制字符: 配置格式元素 (如 %.5s 的 ".5") (IsControlChar)。
#[inline]
#[must_use]
pub fn is_control_char(c: u8) -> bool {
  c.is_ascii_digit() || c == b'.' || c == b'-'
}

/// 对象短打印回调: %o 与 %p 的 objects 层注入点。
pub type PrintObjectFn<'f> = &'f mut dyn FnMut(&mut StringStream, usize);

/// 诊断字符串流 (StringStream): Put 逐字节写入, Add printf 风格格式化。
/// 满 4 字节即以 "...\n" 截断 (与原版一致)。
pub struct StringStream<'a, 'f> {
  allocator: &'a mut dyn StringAllocator,
  capacity: usize,
  length: usize,
  buffer: Vec<u8>,
  /// %o/%p 的对象打印回调。
  print_object: Option<PrintObjectFn<'f>>,
}

impl<'a, 'f> StringStream<'a, 'f> {
  /// 构造 (初始容量 kInitialCapacity)。
  pub fn new(allocator: &'a mut dyn StringAllocator) -> Self {
    let mut s = Self {
      allocator,
      capacity: INITIAL_CAPACITY,
      length: 0,
      buffer: Vec::new(),
      print_object: None,
    };
    if let Some(buf) = s.allocator.allocate(INITIAL_CAPACITY) {
      if !buf.is_empty() {
        s.capacity = buf.len();
        s.buffer = buf.to_vec();
      } else {
        s.capacity = INITIAL_CAPACITY;
        s.buffer.resize(INITIAL_CAPACITY, 0);
      }
    } else {
      s.buffer.resize(INITIAL_CAPACITY, 0);
    }
    s
  }

  /// 注入对象打印回调 (%o)。
  pub fn set_print_object(&mut self, f: PrintObjectFn<'f>) {
    self.print_object = Some(f);
  }

  #[inline]
  fn full(&self) -> bool {
    self.capacity <= self.length + 1
  }

  /// 写入单字节; 满则截断标记 "...\n" 并返回 false (StringStream::Put)。
  pub fn put(&mut self, c: u8) -> bool {
    if self.full() {
      return false;
    }
    if self.capacity >= 2 && self.length >= self.capacity - 2 {
      // 距满差 2: 请求扩容
      let mut new_capacity = self.capacity;
      let _ = self.allocator.grow(&mut new_capacity);
      if new_capacity > self.capacity {
        self.capacity = new_capacity;
        self.buffer.resize(new_capacity, 0);
      } else {
        // 缓冲到头: 写截断标记
        self.length = self.capacity - 1;
        let l = self.length;
        if l >= 4 {
          self.buffer[l - 4] = b'.';
          self.buffer[l - 3] = b'.';
          self.buffer[l - 2] = b'.';
          self.buffer[l - 1] = b'\n';
          self.buffer[l] = 0;
        } else {
          self.buffer[..l].fill(b'.');
          self.buffer[l] = 0;
        }
        return false;
      }
    }
    self.buffer[self.length] = c;
    self.buffer[self.length + 1] = 0;
    self.length += 1;
    true
  }

  /// 追加字节串。
  pub fn put_slice(&mut self, s: &[u8]) -> bool {
    for &c in s {
      if !self.put(c) {
        return false;
      }
    }
    true
  }

  /// uc16 串按字节窄化输出 (%w 语义)。
  pub fn put_ucs16(&mut self, s: &[u16]) -> bool {
    for &c in s {
      if !self.put(c as u8) {
        return false;
      }
    }
    true
  }

  /// printf 风格格式化 (StringStream::Add): 仅支持本引擎的指令集。
  pub fn add(&mut self, format: &str, elms: &[FmtElm]) {
    if self.full() {
      return;
    }
    let f = format.as_bytes();
    let mut offset = 0usize;
    let mut elm = 0usize;
    while offset < f.len() {
      if f[offset] != b'%' || elm == elms.len() {
        self.put(f[offset]);
        offset += 1;
        continue;
      }
      // 收集控制字符序列 (栈上定长缓冲, 避免单指令堆分配)
      let mut temp_buf = [0u8; 32];
      let mut temp_len = 0;
      temp_buf[temp_len] = f[offset];
      temp_len += 1;
      offset += 1;
      while offset < f.len() && is_control_char(f[offset]) && temp_len < 31 {
        temp_buf[temp_len] = f[offset];
        temp_len += 1;
        offset += 1;
      }
      if offset >= f.len() {
        return;
      }
      let ty = f[offset];
      temp_buf[temp_len] = ty;
      temp_len += 1;
      offset += 1;
      let directive = &temp_buf[..temp_len];
      let current = elms[elm];
      elm += 1;
      match (ty, current) {
        (b's', FmtElm::CStr(v)) => {
          self.put_slice(v.as_bytes());
        }
        (b'w', FmtElm::LcStr(v)) => {
          self.put_ucs16(v);
        }
        (b'k', FmtElm::Char(v)) => {
          if (0x20..=0x7F).contains(&v) {
            self.put(v as u8);
          } else if v <= 0xFF {
            self.add("\\x%02x", &[FmtElm::Int(v)]);
          } else {
            self.add("\\u%04x", &[FmtElm::Int(v)]);
          }
        }
        (b'i' | b'd' | b'u' | b'o' | b'x' | b'c' | b'X', FmtElm::Int(v)) => {
          let formatted = format_int_directive(directive, v);
          self.put_slice(formatted.as_bytes());
        }
        (b'f' | b'g' | b'G' | b'e' | b'E', FmtElm::Double(v)) => {
          if v.is_infinite() {
            let s = if v < 0.0 { "-inf" } else { "inf" };
            self.put_slice(s.as_bytes());
          } else if v.is_nan() {
            self.put_slice(b"nan");
          } else {
            let formatted = format_double_directive(directive, v);
            self.put_slice(formatted.as_bytes());
          }
        }
        (b'o', FmtElm::Obj(id)) => {
          if let Some(f) = self.print_object.take() {
            f(self, id);
            self.print_object = Some(f);
          }
        }
        (b'p', FmtElm::Pointer(p)) => {
          let formatted = format!("{p:#x}");
          self.put_slice(formatted.as_bytes());
        }
        _ => {
          // 类型不匹配: 原版 DCHECK 后仍按字面输出指令字符
          log::trace!(
            "StringStream::Add 指令与参数类型不匹配: {}",
            String::from_utf8_lossy(directive)
          );
        }
      }
    }
  }

  /// 无参 Add: 原样输出格式串。
  pub fn add_str(&mut self, s: &str) {
    self.put_slice(s.as_bytes());
  }

  /// 当前长度 (不含结尾 0)。
  #[must_use]
  pub fn length(&self) -> usize {
    self.length
  }

  /// 输出为字符串 (ToCString 语义, 截到当前长度)。
  #[must_use]
  pub fn to_string_lossy(&self) -> String {
    String::from_utf8_lossy(&self.buffer[..self.length]).into_owned()
  }

  /// 重置 (Reset)。
  pub fn reset(&mut self) {
    self.length = 0;
    if !self.buffer.is_empty() {
      self.buffer[0] = 0;
    }
  }
}

/// 整数指令格式化: %.<N>i/d/u/x/c/X — 由 std 格式化承接 (SNPrintF 语义子集)。
fn format_int_directive(directive: &[u8], v: i32) -> String {
  // 指令形如 b"%d" / b"%.5i" / b"%02x"; 解析宽度、精度与类型
  let ty = directive[directive.len() - 1];
  let spec = &directive[1..directive.len() - 1]; // 去掉 '%' 与类型字符
  let is_zero_padded = spec.starts_with(b"0") && spec.len() > 1;
  let precision = if let Some(rest) = spec.strip_prefix(b".") {
    from_utf8(rest).ok().and_then(|p| p.parse::<usize>().ok())
  } else {
    None
  };
  let width = if !spec.starts_with(b".") && !spec.is_empty() {
    from_utf8(spec).ok().and_then(|p| p.parse::<usize>().ok())
  } else {
    None
  };
  match (ty, is_zero_padded, width, precision) {
    (b'x', true, Some(w), _) => format!("{v:0w$x}"),
    (b'x', false, Some(w), _) => format!("{v:w$x}"),
    (b'x', ..) => format!("{v:x}"),
    (b'X', true, Some(w), _) => format!("{v:0w$X}"),
    (b'X', false, Some(w), _) => format!("{v:w$X}"),
    (b'X', ..) => format!("{v:X}"),
    (b'c', ..) => char::from_u32(v as u32).map_or_else(|| v.to_string(), |c| c.to_string()),
    (b'u', true, Some(w), _) => format!("{:0w$}", v as u32),
    (b'u', ..) => format!("{}", v as u32),
    (b'o', true, Some(w), _) => format!("{:0w$o}", v as u32),
    (b'o', false, Some(w), _) => format!("{:w$o}", v as u32),
    (b'o', ..) => format!("{:o}", v as u32),
    (b'i' | b'd' | _, _, _, Some(p)) => format!("{v:.p$}"),
    (b'i' | b'd' | _, true, Some(w), _) => format!("{v:0w$}"),
    (b'i' | b'd' | _, false, Some(w), _) => format!("{v:w$}"),
    _ => v.to_string(),
  }
}

/// 浮点指令格式化: %f/%g/%G/%e/%E。
fn format_double_directive(directive: &[u8], v: f64) -> String {
  let ty = directive[directive.len() - 1];
  match ty {
    // %f 为 vsnprintf 默认精度 6 (base::SNPrintF 兼容; Rust {v} 只给最短形)
    b'f' => format!("{v:.6}"),
    b'e' => fmt_e(v, false),
    b'E' => fmt_e(v, true),
    _ => format!("{v}"),
  }
}

/// vsnprintf %e 语义: 默认精度 6, 指数至少两位带显式符号 (base::SNPrintF 兼容)。
/// Rust 内建 `{v:e}` 只给最短形 ("1e3"), 此处按位拼装 "1.000000e+03"。
fn fmt_e(v: f64, upper: bool) -> String {
  const PREC: usize = 6;
  let raw = format!("{v:e}");
  let (mant, exp) = match raw.split_once('e') {
    Some((m, e)) => (m, e),
    None => (raw.as_str(), "0"),
  };
  let exp_neg = exp.starts_with('-');
  let exp_num: i32 = exp.trim_start_matches(['-', '+']).parse().unwrap_or(0);
  let (int_part, frac_digits) = match mant.split_once('.') {
    Some((i, f)) => (i, format!("{i}{f}")),
    None => (mant, mant.to_string()),
  };
  let digits: String = frac_digits.chars().filter(|c| c.is_ascii_digit()).collect();
  let mut frac = digits.get(1..).unwrap_or("").to_string();
  while frac.len() < PREC {
    frac.push('0');
  }
  frac.truncate(PREC);
  let e = if upper { 'E' } else { 'e' };
  let sign = if exp_neg { '-' } else { '+' };
  format!("{int_part}.{frac}{e}{sign}{exp_num:02}")
}
