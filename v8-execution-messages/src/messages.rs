// 由 tool/src/execution/messages.js 从 v8/src/execution/messages.h, src/execution/messages.cc 生成; 修订就地改, 复跑覆盖
// 错误消息与栈踪格式化 (FrameSkipMode, MessageLocation, MessageFormatter)

use v8_common::{MessageTemplate, template_string};

use crate::error::{Error, Result};

/// 栈展开帧跳过模式 (对应原版 FrameSkipMode)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum FrameSkipMode {
  /// 无条件跳过第一帧 (如 Error 构造函数自身的 BUILTIN_EXIT 帧)
  SkipFirst = 0,
  /// 跳过所有帧直至遇到指定的调用者函数
  SkipUntilSeen = 1,
  /// 不跳过任何帧
  SkipNone = 2,
}

/// 栈踪收集开关 (对应原版 ErrorUtils::StackTraceCollection)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum StackTraceCollection {
  /// 收集栈踪
  Enabled = 0,
  /// 禁用栈踪收集 (如反序列化错误对象时)
  Disabled = 1,
}

/// ToString 消息来源选择 (对应原版 ErrorUtils::ToStringMessageSource)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum ToStringMessageSource {
  /// 优先使用原始消息
  PreferOriginalMessage = 0,
  /// 使用当前 message 属性
  CurrentMessageProperty = 1,
}

/// 错误发生的源码位置 (对应原版 MessageLocation)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct MessageLocation {
  /// 源码起始位置
  pub start_pos: i32,
  /// 源码结束位置
  pub end_pos: i32,
  /// 字节码偏移
  pub bytecode_offset: i32,
  /// 行号 (1-indexed, 若已解析)
  pub line_number: Option<i32>,
  /// 列号 (1-indexed, 若已解析)
  pub column_number: Option<i32>,
}

impl MessageLocation {
  /// 创建已知源码区间的位置
  #[inline]
  pub const fn with_positions(start_pos: i32, end_pos: i32) -> Self {
    Self {
      start_pos,
      end_pos,
      bytecode_offset: -1,
      line_number: None,
      column_number: None,
    }
  }

  /// 创建已知字节码偏移的位置
  #[inline]
  pub const fn with_bytecode_offset(bytecode_offset: i32) -> Self {
    Self {
      start_pos: -1,
      end_pos: -1,
      bytecode_offset,
      line_number: None,
      column_number: None,
    }
  }
}

/// 调用栈单帧信息 (对应原版 CallSiteInfo 文本化投影)
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct CallSiteFrame {
  /// 函数名称
  pub function_name: Option<String>,
  /// 脚本名称或文件路径
  pub script_name: Option<String>,
  /// 行号 (1-indexed)
  pub line_number: Option<i32>,
  /// 列号 (1-indexed)
  pub column_number: Option<i32>,
  /// 是否为 eval 动态执行
  pub is_eval: bool,
  /// 是否为构造函数调用 (new ...)
  pub is_constructor: bool,
  /// 是否为 WebAssembly 帧
  pub is_wasm: bool,
}

impl CallSiteFrame {
  /// 格式化单帧调用栈描述 (例如 "at foo (test.js:10:5)" 或 "at test.js:10:5")
  pub fn format(&self) -> String {
    let mut s = String::new();
    s.push_str("at ");

    if self.is_wasm {
      if let Some(ref fn_name) = self.function_name {
        s.push_str(fn_name);
        s.push_str(" (");
        self.append_location(&mut s);
        s.push(')');
      } else {
        s.push_str("wasm-function (");
        self.append_location(&mut s);
        s.push(')');
      }
      return s;
    }

    if self.is_constructor {
      s.push_str("new ");
      if let Some(ref fn_name) = self.function_name {
        s.push_str(fn_name);
      } else {
        s.push_str("<anonymous>");
      }
      s.push_str(" (");
      self.append_location(&mut s);
      s.push(')');
      return s;
    }

    if let Some(ref fn_name) = self.function_name {
      s.push_str(fn_name);
      s.push_str(" (");
      self.append_location(&mut s);
      s.push(')');
    } else if self.is_eval {
      s.push_str("eval (");
      self.append_location(&mut s);
      s.push(')');
    } else {
      self.append_location(&mut s);
    }

    s
  }

  fn append_location(&self, s: &mut String) {
    use std::fmt::Write;
    if let Some(ref script) = self.script_name {
      s.push_str(script);
      if let Some(line) = self.line_number {
        let _ = write!(s, ":{line}");
        if let Some(col) = self.column_number {
          let _ = write!(s, ":{col}");
        }
      }
    } else if let Some(line) = self.line_number {
      s.push_str("<anonymous>");
      let _ = write!(s, ":{line}");
      if let Some(col) = self.column_number {
        let _ = write!(s, ":{col}");
      }
    } else {
      s.push_str("<unknown location>");
    }
  }
}

/// 参数不匹配时允许默认回退 "undefined" 的模板清单 (原版 kTemplatesWithMismatchedArguments 共 43 项)
pub const TEMPLATES_WITH_MISMATCHED_ARGUMENTS: &[MessageTemplate] = &[
  MessageTemplate::ConstAssign,
  MessageTemplate::ConstructorNotReceiver,
  MessageTemplate::DataCloneErrorDetachedArrayBuffer,
  MessageTemplate::DataCloneErrorOutOfMemory,
  MessageTemplate::IncompatibleMethodReceiver,
  MessageTemplate::InvalidArgument,
  MessageTemplate::InvalidArrayLength,
  MessageTemplate::InvalidAtomicAccessIndex,
  MessageTemplate::InvalidDataViewLength,
  MessageTemplate::InvalidIndex,
  MessageTemplate::InvalidLhsInAssignment,
  MessageTemplate::InvalidLhsInFor,
  MessageTemplate::InvalidLhsInPostfixOp,
  MessageTemplate::InvalidLhsInPrefixOp,
  MessageTemplate::InvalidPrivateBrandReinitialization,
  MessageTemplate::InvalidPrivateFieldReinitialization,
  MessageTemplate::InvalidPrivateMemberWrite,
  MessageTemplate::InvalidRegExpExecResult,
  MessageTemplate::InvalidTimeValue,
  MessageTemplate::InvalidWeakMapKey,
  MessageTemplate::InvalidWeakSetValue,
  MessageTemplate::IteratorReduceNoInitial,
  MessageTemplate::JsonParseShortString,
  MessageTemplate::JsonParseUnexpectedEOS,
  MessageTemplate::JsonParseUnexpectedTokenEndStringWithContext,
  MessageTemplate::JsonParseUnexpectedTokenShortString,
  MessageTemplate::JsonParseUnexpectedTokenStartStringWithContext,
  MessageTemplate::JsonParseUnexpectedTokenSurroundStringWithContext,
  MessageTemplate::MustBePositive,
  MessageTemplate::NotIterable,
  MessageTemplate::NotTypedArray,
  MessageTemplate::ProxyNonObject,
  MessageTemplate::ProxyPrivate,
  MessageTemplate::ProxyRevoked,
  MessageTemplate::ProxyTrapReturnedFalsishFor,
  MessageTemplate::ReduceNoInitial,
  MessageTemplate::SpreadIteratorSymbolNonCallable,
  MessageTemplate::SymbolIteratorInvalid,
  MessageTemplate::TopLevelAwaitStalled,
  MessageTemplate::UndefinedOrNullToObject,
  MessageTemplate::UnexpectedStrictReserved,
  MessageTemplate::UnexpectedTokenIdentifier,
  MessageTemplate::WeakRefsCleanupMustBeCallable,
];

/// 消息格式化器 (对应原版 MessageFormatter)
pub struct MessageFormatter;

impl MessageFormatter {
  /// 检查给定模板是否允许参数不匹配
  #[inline]
  pub fn allows_mismatched_arguments(template: MessageTemplate) -> bool {
    TEMPLATES_WITH_MISMATCHED_ARGUMENTS.contains(&template)
  }

  /// 依据原版 V8 占位符替换规则格式化任意模板字符串
  ///
  /// - `%` 占位符顺序消费 args
  /// - `%%` 转义为单个 `%`
  /// - 若参数不足且允许 mismatch，回退为 `"undefined"`
  pub fn try_format_template_str(
    tpl: &str,
    args: &[&str],
    allows_mismatch: bool,
  ) -> Result<String> {
    let mut result = String::with_capacity(tpl.len() + args.len() * 16);
    let mut arg_idx = 0;
    let mut chars = tpl.chars().peekable();

    while let Some(c) = chars.next() {
      if c == '%' {
        if chars.peek() == Some(&'%') {
          chars.next();
          result.push('%');
        } else if arg_idx < args.len() {
          result.push_str(args[arg_idx]);
          arg_idx += 1;
        } else if allows_mismatch {
          result.push_str("undefined");
        } else {
          return Err(Error::MissingCustomTemplateArgument {
            expected: arg_idx + 1,
            got: args.len(),
          });
        }
      } else {
        result.push(c);
      }
    }

    if arg_idx < args.len() && !allows_mismatch {
      return Err(Error::TooManyCustomTemplateArguments {
        expected: arg_idx,
        got: args.len(),
      });
    }

    Ok(result)
  }

  /// 依据原版 V8 占位符替换规则格式化错误模板
  pub fn try_format(template: MessageTemplate, args: &[&str]) -> Result<String> {
    let tpl = template_string(template);
    let allows_mismatch = Self::allows_mismatched_arguments(template);
    Self::try_format_template_str(tpl, args, allows_mismatch).map_err(|e| match e {
      Error::MissingCustomTemplateArgument { expected, got } => Error::MissingTemplateArgument {
        template,
        expected,
        got,
      },
      Error::TooManyCustomTemplateArguments { expected, got } => Error::TooManyTemplateArguments {
        template,
        expected,
        got,
      },
      other => other,
    })
  }

  /// 强健格式化：即使参数不匹配也绝不 panic，尽最大努力拼接或回退模板原串
  pub fn format(template: MessageTemplate, args: &[&str]) -> String {
    match Self::try_format(template, args) {
      Ok(s) => s,
      Err(_) => {
        // 参数不足或过多时优雅回退
        let tpl = template_string(template);
        let mut s = String::from(tpl);
        if !args.is_empty() {
          s.push_str(" [args: ");
          s.push_str(&args.join(", "));
          s.push(']');
        }
        s
      }
    }
  }
}

/// 栈展开格式化器 (对应原版 ErrorUtils::FormatStackTrace 文本输出逻辑)
pub struct StackTraceFormatter;

impl StackTraceFormatter {
  /// 格式化 JavaScript 标准异常栈跟踪文本
  ///
  /// 输出示例:
  /// ```text
  /// TypeError: Cannot read properties of undefined
  ///     at foo (app.js:10:5)
  ///     at bar (app.js:20:9)
  /// ```
  pub fn format_stack_trace(error_title: &str, frames: &[CallSiteFrame]) -> String {
    let mut out = String::with_capacity(error_title.len() + frames.len() * 40);
    out.push_str(error_title);

    for frame in frames {
      out.push_str("\n    ");
      out.push_str(&frame.format());
    }

    out
  }
}

// ── 编译期静态断言 ──
const _: () = {
  assert!(TEMPLATES_WITH_MISMATCHED_ARGUMENTS.len() == 43);
  assert!(FrameSkipMode::SkipFirst as u8 == 0);
  assert!(FrameSkipMode::SkipUntilSeen as u8 == 1);
  assert!(FrameSkipMode::SkipNone as u8 == 2);
  assert!(StackTraceCollection::Enabled as u8 == 0);
  assert!(StackTraceCollection::Disabled as u8 == 1);
  assert!(ToStringMessageSource::PreferOriginalMessage as u8 == 0);
  assert!(ToStringMessageSource::CurrentMessageProperty as u8 == 1);
};
