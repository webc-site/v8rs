// 由 tool/src/utils/consts.js 从 v8/src/utils/output-stream.h;src/utils/output-stream.cc 生成; 修订就地改, 复跑覆盖

//! 输出流抽象, 移植自 v8/src/utils/output-stream (2025 新增, 上轮盘点缺口)。
//!
//! 上游实现 v8::OutputStream (profiler API) 的文件/字符串落盘;
//! Rust 版将 OutputStream 定义为本地 trait 承载同构语义。

use std::{fs::File, io, io::Write};

/// v8::OutputStream::WriteResult
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WriteResult {
  Continue,
  Abort,
}

/// v8::OutputStream (WriteAsciiChunk + EndOfStream)
pub trait OutputStream {
  /// 追加一段 ASCII 数据
  fn write_ascii_chunk(&mut self, data: &[u8]) -> io::Result<WriteResult>;
  /// 流结束 (v8 序列化协议的收尾钩子)
  fn end_of_stream(&mut self) {}
}

/// FileOutputStream: 逐段写文件, 构造即打开, Drop 时关闭
pub struct FileOutputStream {
  os: File,
}

impl FileOutputStream {
  /// 打开失败视为契约违约 (上游 ofstream 静默失败, Rust 显式化)
  pub fn create(filename: &str) -> io::Result<Self> {
    Ok(FileOutputStream {
      os: File::create(filename)?,
    })
  }

  pub fn is_open(&self) -> bool {
    true // File 语义: 持有即打开
  }
}

impl OutputStream for FileOutputStream {
  fn write_ascii_chunk(&mut self, data: &[u8]) -> io::Result<WriteResult> {
    self.os.write_all(data)?;
    Ok(WriteResult::Continue)
  }

  fn end_of_stream(&mut self) {
    let _ = self.os.flush();
  }
}

/// StringOutputStream: 逐段累积到内存串
#[derive(Default)]
pub struct StringOutputStream {
  buf: Vec<u8>,
}

impl StringOutputStream {
  pub fn new() -> Self {
    Self::default()
  }

  /// str(): 取全部累积内容
  pub fn into_string(self) -> String {
    // 上游 stringstream::str() 返回原始字节; String 只能承载 UTF-8,
    // 非法序列以 U+FFFD 替换保内容 (完整原始字节走 as_bytes)
    String::from_utf8_lossy(&self.buf).into_owned()
  }

  pub fn as_bytes(&self) -> &[u8] {
    &self.buf
  }
}

impl OutputStream for StringOutputStream {
  fn write_ascii_chunk(&mut self, data: &[u8]) -> io::Result<WriteResult> {
    self.buf.extend_from_slice(data);
    Ok(WriteResult::Continue)
  }
}
