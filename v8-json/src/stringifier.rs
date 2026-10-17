use std::fmt::Write;

use crate::{
  error::{Error, Result},
  escape_table::{get_escape_str, is_do_not_escape},
  value::JsonValue,
};

/// JSON 序列化器
pub struct JsonStringifier<'a> {
  gap: Option<&'a str>,
  indent_step: &'a str,
  output: String,
  current_indent: String,
  depth: usize,
  max_depth: usize,
}

impl<'a> JsonStringifier<'a> {
  /// 构造紧凑序列化器 (无空格换行缩进)
  #[inline]
  pub fn new() -> Self {
    Self::with_indent(None)
  }

  /// 构造带缩进格式化的序列化器
  #[inline]
  pub fn with_indent(indent: Option<&'a str>) -> Self {
    let gap = indent.filter(|s| !s.is_empty());
    Self {
      gap,
      indent_step: gap.unwrap_or(""),
      output: String::new(),
      current_indent: String::new(),
      depth: 0,
      max_depth: 1000,
    }
  }

  /// 将 JsonValue 序列化为 JSON 字符串
  pub fn stringify(mut self, value: &JsonValue) -> Result<String> {
    self.serialize_value(value)?;
    Ok(self.output)
  }

  fn serialize_value(&mut self, value: &JsonValue) -> Result<()> {
    match value {
      JsonValue::Null => {
        self.output.push_str("null");
        Ok(())
      }
      JsonValue::Bool(b) => {
        if *b {
          self.output.push_str("true");
        } else {
          self.output.push_str("false");
        }
        Ok(())
      }
      JsonValue::Number(n) => {
        if n.is_nan() || n.is_infinite() {
          self.output.push_str("null");
        } else if *n == 0.0 && n.is_sign_negative() {
          self.output.push_str("-0");
        } else if n.fract() == 0.0 && *n >= (i64::MIN as f64) && *n <= (i64::MAX as f64) {
          let _ = write!(self.output, "{:.0}", n);
        } else {
          let _ = write!(self.output, "{n}");
        }
        Ok(())
      }
      JsonValue::String(s) => {
        self.serialize_string(s);
        Ok(())
      }
      JsonValue::Array(arr) => self.serialize_array(arr),
      JsonValue::Object(obj) => self.serialize_object(obj),
    }
  }

  fn serialize_string(&mut self, s: &str) {
    self.output.push('"');
    let bytes = s.as_bytes();
    let mut last_emit = 0;

    for (i, &b) in bytes.iter().enumerate() {
      if is_do_not_escape(b) {
        continue;
      }

      if let Some(esc) = get_escape_str(b) {
        if last_emit < i {
          self.output.push_str(&s[last_emit..i]);
        }
        self.output.push_str(esc);
        last_emit = i + 1;
      }
    }

    if last_emit < bytes.len() {
      self.output.push_str(&s[last_emit..]);
    }

    self.output.push('"');
  }

  fn serialize_array(&mut self, arr: &[JsonValue]) -> Result<()> {
    if arr.is_empty() {
      self.output.push_str("[]");
      return Ok(());
    }

    self.depth += 1;
    if self.depth > self.max_depth {
      return Err(Error::CircularReference);
    }

    self.output.push('[');

    if self.gap.is_some() {
      let prev_len = self.current_indent.len();
      self.current_indent.push_str(self.indent_step);

      for (i, elem) in arr.iter().enumerate() {
        if i > 0 {
          self.output.push(',');
        }
        self.output.push('\n');
        self.output.push_str(&self.current_indent);
        self.serialize_value(elem)?;
      }

      self.current_indent.truncate(prev_len);
      self.output.push('\n');
      self.output.push_str(&self.current_indent);
    } else {
      for (i, elem) in arr.iter().enumerate() {
        if i > 0 {
          self.output.push(',');
        }
        self.serialize_value(elem)?;
      }
    }

    self.output.push(']');
    self.depth -= 1;
    Ok(())
  }

  fn serialize_object(&mut self, obj: &[(String, JsonValue)]) -> Result<()> {
    if obj.is_empty() {
      self.output.push_str("{}");
      return Ok(());
    }

    self.depth += 1;
    if self.depth > self.max_depth {
      return Err(Error::CircularReference);
    }

    self.output.push('{');

    if self.gap.is_some() {
      let prev_len = self.current_indent.len();
      self.current_indent.push_str(self.indent_step);

      for (i, (key, val)) in obj.iter().enumerate() {
        if i > 0 {
          self.output.push(',');
        }
        self.output.push('\n');
        self.output.push_str(&self.current_indent);
        self.serialize_string(key);
        self.output.push_str(": ");
        self.serialize_value(val)?;
      }

      self.current_indent.truncate(prev_len);
      self.output.push('\n');
      self.output.push_str(&self.current_indent);
    } else {
      for (i, (key, val)) in obj.iter().enumerate() {
        if i > 0 {
          self.output.push(',');
        }
        self.serialize_string(key);
        self.output.push(':');
        self.serialize_value(val)?;
      }
    }

    self.output.push('}');
    self.depth -= 1;
    Ok(())
  }
}

impl Default for JsonStringifier<'_> {
  fn default() -> Self {
    Self::new()
  }
}
