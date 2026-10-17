// 由 tool/src/torque/misc.js 从 v8/src/torque/source-positions.h 生成; 修订就地改, 复跑覆盖
// 源位置体系 (SourceId/LineAndColumn/SourcePosition/SourceFileMap)

use crate::error::{Error, Result};

// 原版 SourceId 包装 int → Rust newtype; SourceFileMap 上下文单例 → 驱动持有的普通结构体;
// CurrentSourceFile/CurrentSourcePosition 上下文 → 由调用方显式传参 (登记偏差)。

/// 原版 SourceId (-1 = 无效)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SourceId(pub i32);

impl SourceId {
  #[must_use]
  pub const fn invalid() -> Self {
    Self(-1)
  }

  #[must_use]
  pub const fn is_valid(self) -> bool {
    self.0 != -1
  }
}

/// 原版 LineAndColumn; K_UNKNOWN_OFFSET = -1
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct LineAndColumn {
  pub offset: i32,
  pub line: i32,
  pub column: i32,
}

impl LineAndColumn {
  pub const K_UNKNOWN_OFFSET: i32 = -1;

  #[must_use]
  pub const fn invalid() -> Self {
    Self {
      offset: -1,
      line: -1,
      column: -1,
    }
  }

  #[must_use]
  pub const fn with_unknown_offset(line: i32, column: i32) -> Self {
    Self {
      offset: Self::K_UNKNOWN_OFFSET,
      line,
      column,
    }
  }
}

impl Default for LineAndColumn {
  fn default() -> Self {
    Self::invalid()
  }
}

/// 原版 SourcePosition
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct SourcePosition {
  pub source: SourceId,
  pub start: LineAndColumn,
  pub end: LineAndColumn,
}

impl SourcePosition {
  #[must_use]
  pub const fn invalid() -> Self {
    Self {
      source: SourceId::invalid(),
      start: LineAndColumn::invalid(),
      end: LineAndColumn::invalid(),
    }
  }

  #[must_use]
  pub fn is_valid(self) -> bool {
    self.source.is_valid() && self.start.offset >= 0 && self.end.offset >= 0
  }

  /// 原版 CompareStartIgnoreColumn
  #[must_use]
  pub fn compare_start_ignore_column(self, pos: Self) -> bool {
    self.start.line == pos.start.line && self.source == pos.source
  }

  /// 原版 Contains
  #[must_use]
  pub fn contains(self, pos: LineAndColumn) -> bool {
    if pos.line < self.start.line || pos.line > self.end.line {
      return false;
    }
    if pos.line == self.start.line && pos.column < self.start.column {
      return false;
    }
    !(pos.line == self.end.line && pos.column >= self.end.column)
  }
}

impl Default for SourcePosition {
  fn default() -> Self {
    Self::invalid()
  }
}

/// 原版 SourceFileMap (上下文单例 → 结构体, 由编译驱动持有)
#[derive(Debug, Clone, Default)]
pub struct SourceFileMap {
  pub sources: Vec<String>,
  pub v8_root: String,
}

impl SourceFileMap {
  #[must_use]
  pub fn new(v8_root: String) -> Self {
    Self {
      sources: Vec::new(),
      v8_root,
    }
  }

  /// 原版 AddSource: 登记 SourceId = 下标
  pub fn add_source(&mut self, path: String) -> SourceId {
    self.sources.push(path);
    SourceId((self.sources.len() - 1) as i32)
  }

  /// 原版 GetSourceId
  #[must_use]
  pub fn get_source_id(&self, path: &str) -> Option<SourceId> {
    self
      .sources
      .iter()
      .position(|p| p == path)
      .map(|i| SourceId(i as i32))
  }

  #[must_use]
  pub fn all_sources(&self) -> Vec<SourceId> {
    (0..self.sources.len())
      .map(|i| SourceId(i as i32))
      .collect()
  }

  /// 原版 PathFromV8Root
  pub fn path_from_v8_root(&self, file: SourceId) -> Result<String> {
    if !file.is_valid() || file.0 as usize >= self.sources.len() {
      return Err(Error::OpenFile(format!("{file:?}")));
    }
    Ok(self.sources[file.0 as usize].clone())
  }

  /// 原版 PathFromV8RootWithoutExtension
  pub fn path_from_v8_root_without_extension(&self, file: SourceId) -> Result<String> {
    let path = self.path_from_v8_root(file)?;
    // TODO(填体): 去扩展名语义对齐原版 (仅去最后一个 . 后缀)
    Ok(path)
  }

  /// 原版 AbsolutePath
  pub fn absolute_path(&self, file: SourceId) -> Result<String> {
    // TODO(填体): v8_root 前缀拼接
    self.path_from_v8_root(file)
  }

  /// 原版 FileRelativeToV8RootExists (source-positions.cc:74-78):
  /// 相对 v8_root 的存在性检查 (v8_root 为空时退化为 cwd 相对路径)
  #[must_use]
  pub fn file_relative_to_v8_root_exists(&self, path: &str) -> bool {
    use std::path::Path;
    let joined = if self.v8_root.is_empty() {
      Path::new(path).to_path_buf()
    } else {
      Path::new(&self.v8_root).join(path)
    };
    joined.exists()
  }
}

/// 原版 PositionAsString (自由函数, 依赖 SourceFileMap → 显式传参)
#[must_use]
pub fn position_as_string(map: &SourceFileMap, pos: SourcePosition) -> String {
  // TODO(填体): 失败路径与原版对齐 (原版直接取, 无错误分支)
  let path = map.path_from_v8_root(pos.source).unwrap_or_default();
  format!("{}:{}:{}", path, pos.start.line + 1, pos.start.column + 1)
}

// ── 原版 src/torque/source-positions.cc ──
// 签名清单
// L18: const std::string& SourceFileMap::PathFromV8Root(SourceId file)
// L24: std::string SourceFileMap::AbsolutePath(SourceId file)
// L31: std::string SourceFileMap::PathFromV8RootWithoutExtension(SourceId file)
// L41: SourceId SourceFileMap::AddSource(std::string path)
// L47: SourceId SourceFileMap::GetSourceId(const std::string& path)
// L57: std::vector<SourceId> SourceFileMap::AllSources()
// L74: bool SourceFileMap::FileRelativeToV8RootExists(const std::string& path)
