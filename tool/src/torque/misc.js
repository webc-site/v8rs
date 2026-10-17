#!/usr/bin/env bun
// v8/src/torque 基础设施组 → v8-torque/src 直出
// 覆盖: constants.h → constants.rs; source-positions.{h,cc} → source_positions.rs;
//       utils.{h,cc} → utils.rs; global-context.{h,cc} → global_context.rs;
//       torque-compiler.{h,cc} → torque_compiler.rs; torque.cc → torque.rs;
//       kythe-data.{h,cc} → kythe_data.rs; server-data.{h,cc} → server_data.rs;
//       layout-loader.{h,cc} → layout_loader.rs; runtime-support.h → runtime_support.rs;
//       runtime-macro-shims.h → runtime_macro_shims.rs
// 同名族 .h/.cc 合并一个模块, .cc 部分以 "// ── 原版 xxx.cc ──" 分隔。
// 骨架策略: 类型层全量建模; 仅实现平凡存取/arena 机械; 语义函数 Err(Error::Unimplemented)
// 或 TODO(填体) 占位; .cc 全量签名清单以注释保留 (填体正交任务源)。
import { readV8, writeSrc, banner } from "../../lib.js";

const CRATE = "v8-torque";

// ---------- 通用提取器 (脚本内本地副本) ----------
// 单遍词法化 FSM: 注释与字符串/字符字面量同层判定 (字符串内 // 不误删),
// 换行 1:1 保留 → 行号稳定; 支持 \' \" 转义、行尾续行、R"( raw string
const noCmt = (s) => {
  let out = "", i = 0;
  const n = s.length;
  while (i < n) {
    const c = s[i];
    if (c === "/" && s[i + 1] === "*") {
      const end = s.indexOf("*/", i + 2);
      const seg = s.slice(i, end < 0 ? n : end + 2);
      for (const ch of seg) if (ch === "\n") out += "\n";
      i += seg.length;
    } else if (c === "/" && s[i + 1] === "/") {
      while (i < n && s[i] !== "\n") i++;
    } else if (c === '"' || (c === "R" && /^R"[^ ()\\\t\v\f\n]*\(/.test(s.slice(i, i + 64)))) {
      const raw = c === "R" ? /^R"([^ ()\\\t\v\f\n]*)\(/.exec(s.slice(i, i + 64))[0] : "";
      if (raw) {
        const close = ")" + raw.slice(2, -1) + '"';
        const end = s.indexOf(close, i + raw.length);
        const body = s.slice(i + raw.length, end < 0 ? n : end);
        for (const ch of body) if (ch === "\n") out += "\n";
        out += '""';
        i = end < 0 ? n : end + close.length;
      } else {
        out += '""';
        i++;
        while (i < n && s[i] !== '"' && s[i] !== "\n") {
          if (s[i] === "\\") {
            if (s[i + 1] === "\n") out += "\n";
            i += 2;
          } else i++;
        }
        if (s[i] === '"') i++;
      }
    } else if (c === "'") {
      out += "''";
      i++;
      while (i < n && s[i] !== "'" && s[i] !== "\n") {
        i += s[i] === "\\" ? 2 : 1;
      }
      if (s[i] === "'") i++;
    } else {
      out += c;
      i++;
    }
  }
  return out;
};

// 只去注释保留字符串字面量 (常量值提取用)
const stripCommentsOnly = (s) =>
  s.replace(/\/\*[\s\S]*?\*\//g, "").replace(/\/\/.*$/gm, "");

function topSigs(text) {
  const out = [];
  // scope 栈: ns=命名空间/extern 块, cls=class/struct 块 (成员继续收集), fn=函数体 (不收集)
  const scope = [];
  let unit = "", startLine = 0;
  const top = () => scope[scope.length - 1] ?? "ns";
  const fin = () => {
    const s = unit.trim().replace(/\s+/g, " ").replace(/;$/, "");
    if (s.includes("(") && !/^(namespace|using|typedef|friend|enum|struct|class|template|static_assert|if|for|while|switch|else|do|try|catch|return|case|default|#)/.test(s) && /[)]\s*(const)?\s*(override)?\s*(noexcept)?\s*$/.test(s)) {
      out.push({ sig: s, line: startLine });
    }
  };
  for (const [i, raw] of noCmt(text).split("\n").entries()) {
    if (/^\s*#/.test(raw)) continue;
    // 顶层全大写宏行 (EXPORT_CONTEXTUAL_VARIABLE(...) / DISABLE_CFI_ICALL) 不入签名
    if (/^[A-Z][A-Z0-9_]*(\(.*\))?\s*;?\s*$/.test(raw.trim())) continue;
    for (const ch of raw) {
      if (ch === "{") {
        const t = top();
        const u = unit.trim();
        // 类头判据: class/struct 关键字且无 `(` (排除函数签名与 template <class T> 前缀)
        const isClassHead = !u.includes("(") && /^(class|struct)\b/.test(u);
        if (t === "ns") {
          if (u === "" || /^(namespace|extern)\b/.test(u)) { scope.push("ns"); unit = ""; continue; }
          // 类/结构块: 不收类头签名, 成员方法继续收集
          if (isClassHead) { scope.push("cls"); unit = ""; continue; }
          fin(); scope.push("fn"); unit = ""; continue;
        }
        if (t === "cls") {
          if (u !== "") fin(); // 类内方法定义: 出条后入函数体
          // 嵌套类/结构继续按类块收集 (template <T> class X : Y { 也归 cls)
          scope.push(!u.includes("(") && /\b(class|struct)\b/.test(u) ? "cls" : "fn");
          unit = "";
          continue;
        }
        scope.push("fn");
      } else if (ch === "}") {
        scope.pop();
      } else if (top() === "ns" || top() === "cls") {
        if (!unit.trim()) startLine = i + 1;
        unit += ch;
      }
    }
    // `;` 结尾单元单独出条 (P2-3: 防多条声明粘连 1 条), 无 `(` 的纯声明仅清空
    if (/;\s*$/.test(raw) && top() !== "fn") {
      if (unit.includes("(")) fin();
      unit = "";
    }
  }
  return out;
}

function classBlock(header, text, from = 0) {
  const re = new RegExp(`(?:class|struct)\\s+(?:V8_EXPORT_PRIVATE\\s+|final\\s+)*${header}(?:\\s+final)?[^\\{;]*`);
  const m = text.slice(from).match(re);
  if (!m) return null;
  const headStart = from + m.index;
  const open = text.indexOf("{", headStart);
  if (open < 0) return null;
  let depth = 0;
  for (let j = open; j < text.length; j++) {
    if (text[j] === "{") depth++;
    else if (text[j] === "}") { depth--; if (depth === 0) return { head: m[0].replace(/\s+/g, " "), body: text.slice(open + 1, j) }; }
  }
  return null;
}

function memberFields(body) {
  const out = [];
  for (const raw of body.split("\n")) {
    const line = raw.trim();
    if (!line.endsWith(";") || line.includes("(")) continue;
    if (/^(DEFINE_|DECLARE_|friend|using|typedef|enum|static)/.test(line) || line.startsWith("//")) continue;
    const m = line.slice(0, -1).match(/^(.+?)\s*\b(\w+)$/);
    if (m && !/[<>=]/.test(m[2])) out.push({ type: m[1].replace(/\s+/g, " "), name: m[2] });
  }
  return out;
}

function sigList(f, note = "签名清单 (填体正交任务源)") {
  const s = readV8(f);
  let dd = `// ── 原版 ${f} ──\n`;
  dd += banner(`src/torque/misc.js`, f, note).split("\n").filter(Boolean).slice(1).join("\n") + "\n";
  const sigs = topSigs(s);
  // 防静默: 清单条数按文件行数比例校验 (< 行数/200 视为抽取异常)
  const lineCount = s.split("\n").length;
  if (f.endsWith(".cc") && sigs.length < lineCount / 200) {
    console.error(`[torque] ${f} ${lineCount} 行仅抽到 ${sigs.length} 条签名 (< 行数/200) — noCmt/topSigs 异常, 防静默检查`);
  }
  for (const x of sigs) dd += `// L${x.line}: ${x.sig}\n`;
  return dd;
}

// ===========================================================================
// constants.h → constants.rs (真转换)
// ===========================================================================
function genConstants() {
  const rel = "src/torque/constants.h";
  // 折叠 "= \n    " 续行, 多行声明的常量值可提取
  const s = stripCommentsOnly(readV8(rel)).replace(/=\s*\n\s*/g, "= ");
  const out = [];
  out.push(banner("src/torque/misc.js", rel, "类型字符串常量 + 位标志全量 (真转换)"));
  out.push(`// 原版 base::Flags<XxxFlag> → pub type XxxFlags = u32; 位组合经 as u32`);
  out.push(``);
  out.push(`/// 常量前缀, 类型名 constexpr 判别用`);
  out.push(`pub const CONSTEXPR_TYPE_PREFIX: &str = "constexpr ";`);
  out.push(``);
  // 字符串常量 1:1 (跳过已单独输出的 CONSTEXPR_TYPE_PREFIX)
  for (const m of s.matchAll(/static const char\* const (\w+) = "([^"]*)";/g)) {
    const [, name, val] = m;
    if (name === "CONSTEXPR_TYPE_PREFIX") continue;
    out.push(`pub const ${name}: &str = "${val}";`);
  }
  out.push(``);

  // 位标志枚举 1:1
  const flags = [
    ["AbstractTypeFlag", "AbstractTypeFlags"],
    ["ClassFlag", "ClassFlags"],
    ["StructFlag", "StructFlags"],
  ];
  for (const [name, flagsAlias] of flags) {
    const m = s.match(new RegExp(`enum class ${name} \\{([^}]*)\\};`));
    if (!m) { console.error(`constants 缺失 ${name}`); continue; }
    const items = [...m[1].matchAll(/k(\w+) = ([^,\n]+),?/g)].map(([, v, e]) => [v, e.trim()]);
    out.push(`/// ${name} 1:1 (${items.length} 位)`);
    out.push(`#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]`);
    out.push(`#[repr(u32)]`);
    out.push(`pub enum ${name} {`);
    for (const [v, e] of items) out.push(`  ${v} = ${e},`);
    out.push(`}`);
    out.push(``);
    out.push(`/// 原版 using ${flagsAlias} = base::Flags<${name}>`);
    out.push(`pub type ${flagsAlias} = u32;`);
    out.push(``);
  }
  {
    const m = s.match(/enum class FieldSynchronization \{([^}]*)\};/);
    const vs = [...m[1].matchAll(/k(\w+),?/g)].map((x) => x[1]);
    out.push(`/// FieldSynchronization 1:1 (首变体 #[default], 供 Default 结构体引用)`);
    out.push(`#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]`);
    out.push(`#[repr(u8)]`);
    out.push(`pub enum FieldSynchronization {`);
    out.push(`  #[default]`);
    out.push(`  ${vs.join(", ")}`);
    out.push(`}`);
    out.push(``);
  }
  out.push(`/// 原版 IsConstexprName: "constexpr " 前缀判别`);
  out.push(`#[must_use]`);
  out.push(`pub fn is_constexpr_name(name: &str) -> bool {`);
  out.push(`  name.starts_with(CONSTEXPR_TYPE_PREFIX)`);
  out.push(`}`);
  out.push(``);
  out.push(`/// 原版 GetNonConstexprName`);
  out.push(`#[must_use]`);
  out.push(`pub fn get_non_constexpr_name(name: &str) -> &str {`);
  out.push(`  name.strip_prefix(CONSTEXPR_TYPE_PREFIX).unwrap_or(name)`);
  out.push(`}`);
  out.push(``);
  out.push(`/// 原版 GetConstexprName`);
  out.push(`#[must_use]`);
  out.push(`pub fn get_constexpr_name(name: &str) -> String {`);
  out.push(`  if is_constexpr_name(name) {`);
  out.push(`    return name.to_owned();`);
  out.push(`  }`);
  out.push(`  format!("{CONSTEXPR_TYPE_PREFIX}{name}")`);
  out.push(`}`);
  writeSrc(CRATE, "constants.rs", out.join("\n") + "\n");
}

// ===========================================================================
// source-positions.{h,cc} → source_positions.rs
// ===========================================================================
function genSourcePositions() {
  const relH = "src/torque/source-positions.h";
  const relCc = "src/torque/source-positions.cc";
  const out = [];
  out.push(banner("src/torque/misc.js", relH, "源位置体系 (SourceId/LineAndColumn/SourcePosition/SourceFileMap)"));
  out.push(`use crate::error::{Error, Result};`);
  out.push(``);
  out.push(`// 原版 SourceId 包装 int → Rust newtype; SourceFileMap 上下文单例 → 驱动持有的普通结构体;`);
  out.push(`// CurrentSourceFile/CurrentSourcePosition 上下文 → 由调用方显式传参 (登记偏差)。`);
  out.push(``);
  out.push(`/// 原版 SourceId (-1 = 无效)`);
  out.push(`#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]`);
  out.push(`pub struct SourceId(pub i32);`);
  out.push(``);
  out.push(`impl SourceId {`);
  out.push(`  #[must_use]`);
  out.push(`  pub const fn invalid() -> Self { Self(-1) }`);
  out.push(``);
  out.push(`  #[must_use]`);
  out.push(`  pub const fn is_valid(self) -> bool { self.0 != -1 }`);
  out.push(`}`);
  out.push(``);
  out.push(`/// 原版 LineAndColumn; K_UNKNOWN_OFFSET = -1`);
  out.push(`#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]`);
  out.push(`pub struct LineAndColumn {`);
  out.push(`  pub offset: i32,`);
  out.push(`  pub line: i32,`);
  out.push(`  pub column: i32,`);
  out.push(`}`);
  out.push(``);
  out.push(`impl LineAndColumn {`);
  out.push(`  pub const K_UNKNOWN_OFFSET: i32 = -1;`);
  out.push(``);
  out.push(`  #[must_use]`);
  out.push(`  pub const fn invalid() -> Self { Self { offset: -1, line: -1, column: -1 } }`);
  out.push(``);
  out.push(`  #[must_use]`);
  out.push(`  pub const fn with_unknown_offset(line: i32, column: i32) -> Self {`);
  out.push(`    Self { offset: Self::K_UNKNOWN_OFFSET, line, column }`);
  out.push(`  }`);
  out.push(`}`);
  out.push(``);
  out.push(`impl Default for LineAndColumn {`);
  out.push(`  fn default() -> Self { Self::invalid() }`);
  out.push(`}`);
  out.push(``);
  out.push(`/// 原版 SourcePosition`);
  out.push(`#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]`);
  out.push(`pub struct SourcePosition {`);
  out.push(`  pub source: SourceId,`);
  out.push(`  pub start: LineAndColumn,`);
  out.push(`  pub end: LineAndColumn,`);
  out.push(`}`);
  out.push(``);
  out.push(`impl SourcePosition {`);
  out.push(`  #[must_use]`);
  out.push(`  pub const fn invalid() -> Self {`);
  out.push(`    Self { source: SourceId::invalid(), start: LineAndColumn::invalid(), end: LineAndColumn::invalid() }`);
  out.push(`  }`);
  out.push(``);
  out.push(`  #[must_use]`);
  out.push(`  pub fn is_valid(self) -> bool {`);
  out.push(`    self.source.is_valid() && self.start.offset >= 0 && self.end.offset >= 0`);
  out.push(`  }`);
  out.push(``);
  out.push(`  /// 原版 CompareStartIgnoreColumn`);
  out.push(`  #[must_use]`);
  out.push(`  pub fn compare_start_ignore_column(self, pos: Self) -> bool {`);
  out.push(`    self.start.line == pos.start.line && self.source == pos.source`);
  out.push(`  }`);
  out.push(``);
  out.push(`  /// 原版 Contains`);
  out.push(`  #[must_use]`);
  out.push(`  pub fn contains(self, pos: LineAndColumn) -> bool {`);
  out.push(`    if pos.line < self.start.line || pos.line > self.end.line {`);
  out.push(`      return false;`);
  out.push(`    }`);
  out.push(`    if pos.line == self.start.line && pos.column < self.start.column {`);
  out.push(`      return false;`);
  out.push(`    }`);
  out.push(`    !(pos.line == self.end.line && pos.column >= self.end.column)`);
  out.push(`  }`);
  out.push(`}`);
  out.push(``);
  out.push(`impl Default for SourcePosition {`);
  out.push(`  fn default() -> Self { Self::invalid() }`);
  out.push(`}`);
  out.push(``);
  out.push(`/// 原版 SourceFileMap (上下文单例 → 结构体, 由编译驱动持有)`);
  out.push(`#[derive(Debug, Clone, Default)]`);
  out.push(`pub struct SourceFileMap {`);
  out.push(`  pub sources: Vec<String>,`);
  out.push(`  pub v8_root: String,`);
  out.push(`}`);
  out.push(``);
  out.push(`impl SourceFileMap {`);
  out.push(`  #[must_use]`);
  out.push(`  pub fn new(v8_root: String) -> Self { Self { sources: Vec::new(), v8_root } }`);
  out.push(``);
  out.push(`  /// 原版 AddSource: 登记 SourceId = 下标`);
  out.push(`  pub fn add_source(&mut self, path: String) -> SourceId {`);
  out.push(`    self.sources.push(path);`);
  out.push(`    SourceId((self.sources.len() - 1) as i32)`);
  out.push(`  }`);
  out.push(``);
  out.push(`  /// 原版 GetSourceId`);
  out.push(`  #[must_use]`);
  out.push(`  pub fn get_source_id(&self, path: &str) -> Option<SourceId> {`);
  out.push(`    self.sources.iter().position(|p| p == path).map(|i| SourceId(i as i32))`);
  out.push(`  }`);
  out.push(``);
  out.push(`  #[must_use]`);
  out.push(`  pub fn all_sources(&self) -> Vec<SourceId> {`);
  out.push(`    (0..self.sources.len()).map(|i| SourceId(i as i32)).collect()`);
  out.push(`  }`);
  out.push(``);
  out.push(`  /// 原版 PathFromV8Root`);
  out.push(`  pub fn path_from_v8_root(&self, file: SourceId) -> Result<String> {`);
  out.push(`    if !file.is_valid() || file.0 as usize >= self.sources.len() {`);
  out.push(`      return Err(Error::OpenFile(format!("{file:?}")));`);
  out.push(`    }`);
  out.push(`    Ok(self.sources[file.0 as usize].clone())`);
  out.push(`  }`);
  out.push(``);
  out.push(`  /// 原版 PathFromV8RootWithoutExtension`);
  out.push(`  pub fn path_from_v8_root_without_extension(&self, file: SourceId) -> Result<String> {`);
  out.push(`    let path = self.path_from_v8_root(file)?;`);
  out.push(`    // TODO(填体): 去扩展名语义对齐原版 (仅去最后一个 . 后缀)`);
  out.push(`    Ok(path)`);
  out.push(`  }`);
  out.push(``);
  out.push(`  /// 原版 AbsolutePath`);
  out.push(`  pub fn absolute_path(&self, file: SourceId) -> Result<String> {`);
  out.push(`    // TODO(填体): v8_root 前缀拼接`);
  out.push(`    self.path_from_v8_root(file)`);
  out.push(`  }`);
  out.push(``);
  out.push(`  /// 原版 FileRelativeToV8RootExists`);
  out.push(`  #[must_use]`);
  out.push(`  pub fn file_relative_to_v8_root_exists(&self, path: &str) -> bool {`);
  out.push(`    // TODO(填体): 以 v8_root 为基准的 fs 存在性检查`);
  out.push(`    let _ = (self, path);`);
  out.push(`    false`);
  out.push(`  }`);
  out.push(`}`);
  out.push(``);
  out.push(`/// 原版 PositionAsString (自由函数, 依赖 SourceFileMap → 显式传参)`);
  out.push(`#[must_use]`);
  out.push(`pub fn position_as_string(map: &SourceFileMap, pos: SourcePosition) -> String {`);
  out.push(`  // TODO(填体): 失败路径与原版对齐 (原版直接取, 无错误分支)`);
  out.push(`  let path = map.path_from_v8_root(pos.source).unwrap_or_default();`);
  out.push(`  format!("{}:{}:{}", path, pos.start.line + 1, pos.start.column + 1)`);
  out.push(`}`);
  out.push(``);
  out.push(sigList(relCc, "签名清单"));
  writeSrc(CRATE, "source_positions.rs", out.join("\n") + "\n");
}

// ===========================================================================
// utils.{h,cc} → utils.rs
// ===========================================================================
function genUtils() {
  const relH = "src/torque/utils.h";
  const relCc = "src/torque/utils.cc";
  const out = [];
  out.push(banner("src/torque/misc.js", relH, "工具集: Stack/StackRange/ResidueClass/Worklist/Deduplicator/TorqueMessage"));
  out.push(`use crate::error::{Error, Result};`);
  out.push(`use crate::source_positions::SourcePosition;`);
  out.push(`use std::collections::{HashMap, HashSet, VecDeque};`);
  out.push(`use std::fs;`);
  out.push(`use std::hash::Hash;`);
  out.push(`use std::path::Path;`);
  out.push(``);
  out.push(`// 原版 C++ 流式工具 (PrintList/PrintCommaSeparatedList/NullOStream/IfDefScope/`);
  out.push(`// NamespaceScope/IncludeGuardScope/IncludeObjectMacrosScope) → Rust 版生成器直接拼`);
  out.push(`// String, 不引入 std::io::Write 流抽象 (登记偏差, 见 cpp_builder.rs)。`);
  out.push(`// 原版 EraseIf/TransformVector → Rust 迭代器适配器 (retain/map), 不另设函数。`);
  out.push(``);
  out.push(`/// TorqueMessage::Kind 1:1`);
  out.push(`#[derive(Debug, Clone, Copy, PartialEq, Eq)]`);
  out.push(`#[repr(u8)]`);
  out.push(`pub enum MessageKind { Error, Lint }`);
  out.push(``);
  out.push(`/// 原版 TorqueMessage 1:1`);
  out.push(`#[derive(Debug, Clone)]`);
  out.push(`pub struct TorqueMessage {`);
  out.push(`  pub message: String,`);
  out.push(`  pub position: Option<SourcePosition>,`);
  out.push(`  pub kind: MessageKind,`);
  out.push(`}`);
  out.push(``);
  out.push(`// 原版 TorqueMessages 上下文 → 编译结果收集 (torque_compiler.rs);`);
  out.push(`// 原版 MessageBuilder/Error()/Lint()/ReportError 异常流 → Result 传播:`);
  out.push(`//   Error(...).Throw() → return Err(Error::Message(..));`);
  out.push(`//   TorqueAbortCompilation → Error::Abort。`);
  out.push(``);
  out.push(`/// 原版 BottomOffset 1:1 (自栈底偏移)`);
  out.push(`#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]`);
  out.push(`pub struct BottomOffset {`);
  out.push(`  pub offset: usize,`);
  out.push(`}`);
  out.push(``);
  out.push(`impl BottomOffset {`);
  out.push(`  /// 原版 operator+(size_t)`);
  out.push(`  #[must_use]`);
  out.push(`  pub fn plus(self, x: usize) -> Self { Self { offset: self.offset + x } }`);
  out.push(``);
  out.push(`  /// 原版 operator-(size_t), 下溢为调用方契约错误 (原版 DCHECK)`);
  out.push(`  #[must_use]`);
  out.push(`  pub fn minus(self, x: usize) -> Self {`);
  out.push(`    debug_assert!(self.offset >= x);`);
  out.push(`    Self { offset: self.offset - x }`);
  out.push(`  }`);
  out.push(`}`);
  out.push(``);
  out.push(`/// 原版 StackRange 1:1 (栈顶区间 [begin, end))`);
  out.push(`#[derive(Debug, Clone, Copy, PartialEq, Eq)]`);
  out.push(`pub struct StackRange {`);
  out.push(`  pub begin: BottomOffset,`);
  out.push(`  pub end: BottomOffset,`);
  out.push(`}`);
  out.push(``);
  out.push(`impl StackRange {`);
  out.push(`  #[must_use]`);
  out.push(`  pub const fn new(begin: BottomOffset, end: BottomOffset) -> Self { Self { begin, end } }`);
  out.push(``);
  out.push(`  #[must_use]`);
  out.push(`  pub fn size(self) -> usize { self.end.offset - self.begin.offset }`);
  out.push(``);
  out.push(`  /// 原版 Extend: 相邻区间合并 (原版 DCHECK_EQ(end, adjacent.begin))`);
  out.push(`  pub fn extend(&mut self, adjacent: Self) {`);
  out.push(`    debug_assert_eq!(self.end, adjacent.begin);`);
  out.push(`    self.end = adjacent.end;`);
  out.push(`  }`);
  out.push(`}`);
  out.push(``);
  out.push(`/// 原版 Stack<T> 1:1 — 底部偏移寻址栈 (cfg/求值栈共用)`);
  out.push(`#[derive(Debug, Clone, PartialEq, Eq, Default)]`);
  out.push(`pub struct Stack<T> {`);
  out.push(`  elements: Vec<T>,`);
  out.push(`}`);
  out.push(``);
  out.push(`impl<T> Stack<T> {`);
  out.push(`  #[must_use]`);
  out.push(`  pub fn new() -> Self { Self { elements: Vec::new() } }`);
  out.push(``);
  out.push(`  #[must_use]`);
  out.push(`  pub fn from_vec(v: Vec<T>) -> Self { Self { elements: v } }`);
  out.push(``);
  out.push(`  #[must_use]`);
  out.push(`  pub fn size(&self) -> usize { self.elements.len() }`);
  out.push(``);
  out.push(`  /// 原版 Peek`);
  out.push(`  #[must_use]`);
  out.push(`  pub fn peek(&self, from_bottom: BottomOffset) -> &T {`);
  out.push(`    &self.elements[from_bottom.offset]`);
  out.push(`  }`);
  out.push(``);
  out.push(`  /// 原版 Poke`);
  out.push(`  pub fn poke(&mut self, from_bottom: BottomOffset, x: T) {`);
  out.push(`    self.elements[from_bottom.offset] = x;`);
  out.push(`  }`);
  out.push(``);
  out.push(`  pub fn push(&mut self, x: T) {`);
  out.push(`    self.elements.push(x);`);
  out.push(`  }`);
  out.push(``);
  out.push(`  /// 原版 TopRange`);
  out.push(`  #[must_use]`);
  out.push(`  pub fn top_range(&self, slot_count: usize) -> StackRange {`);
  out.push(`    debug_assert!(self.size() >= slot_count);`);
  out.push(`    let above = self.above_top();`);
  out.push(`    StackRange::new(above.minus(slot_count), above)`);
  out.push(`  }`);
  out.push(``);
  out.push(`  /// 原版 PushMany`);
  out.push(`  pub fn push_many(&mut self, v: &[T]) -> StackRange`);
  out.push(`  where`);
  out.push(`    T: Clone,`);
  out.push(`  {`);
  out.push(`    for x in v {`);
  out.push(`      self.push(x.clone());`);
  out.push(`    }`);
  out.push(`    self.top_range(v.len())`);
  out.push(`  }`);
  out.push(``);
  out.push(`  #[must_use]`);
  out.push(`  pub fn top(&self) -> &T {`);
  out.push(`    self.peek(self.above_top().minus(1))`);
  out.push(`  }`);
  out.push(``);
  out.push(`  /// 原版 Pop (空栈为调用方契约, 原版 DCHECK; Rust 以 Option 表达, 登记偏差)`);
  out.push(`  pub fn pop(&mut self) -> Option<T> {`);
  out.push(`    self.elements.pop()`);
  out.push(`  }`);
  out.push(``);
  out.push(`  /// 原版 PopMany`);
  out.push(`  pub fn pop_many(&mut self, count: usize) -> Vec<T> {`);
  out.push(`    debug_assert!(self.elements.len() >= count);`);
  out.push(`    self.elements.split_off(self.elements.len() - count)`);
  out.push(`  }`);
  out.push(``);
  out.push(`  /// 原版 AboveTop: 栈顶之上的无效偏移 (供 StackRange 用)`);
  out.push(`  #[must_use]`);
  out.push(`  pub fn above_top(&self) -> BottomOffset {`);
  out.push(`    BottomOffset { offset: self.size() }`);
  out.push(`  }`);
  out.push(``);
  out.push(`  /// 原版 DeleteRange: 删区间, 高位槽下移补位`);
  out.push(`  pub fn delete_range(&mut self, range: StackRange) {`);
  out.push(`    if range.size() == 0 {`);
  out.push(`      return;`);
  out.push(`    }`);
  out.push(`    self.elements.drain(range.begin.offset..range.end.offset);`);
  out.push(`  }`);
  out.push(`}`);
  out.push(``);
  out.push(`/// 原版 Worklist<T> 1:1 — 去重工作队列`);
  out.push(`#[derive(Debug)]`);
  out.push(`pub struct Worklist<T: Copy + Eq + Hash> {`);
  out.push(`  queue: VecDeque<T>,`);
  out.push(`  contained: HashSet<T>,`);
  out.push(`}`);
  out.push(``);
  out.push(`// 手写 Default: derive 会附加 T: Default 约束, 不需要`);
  out.push(`impl<T: Copy + Eq + Hash> Default for Worklist<T> {`);
  out.push(`  fn default() -> Self {`);
  out.push(`    Self { queue: VecDeque::new(), contained: HashSet::new() }`);
  out.push(`  }`);
  out.push(`}`);
  out.push(``);
  out.push(`impl<T: Copy + Eq + Hash> Worklist<T> {`);
  out.push(`  #[must_use]`);
  out.push(`  pub fn is_empty(&self) -> bool { self.queue.is_empty() }`);
  out.push(``);
  out.push(`  /// 返回是否首次入队`);
  out.push(`  pub fn enqueue(&mut self, value: T) -> bool {`);
  out.push(`    if self.contained.contains(&value) {`);
  out.push(`      return false;`);
  out.push(`    }`);
  out.push(`    self.queue.push_back(value);`);
  out.push(`    self.contained.insert(value);`);
  out.push(`    true`);
  out.push(`  }`);
  out.push(``);
  out.push(`  /// 原版 Dequeue (空队列为调用方契约, 原版 DCHECK)`);
  out.push(`  pub fn dequeue(&mut self) -> Option<T> {`);
  out.push(`    let value = self.queue.pop_front()?;`);
  out.push(`    self.contained.remove(&value);`);
  out.push(`    Some(value)`);
  out.push(`  }`);
  out.push(`}`);
  out.push(``);
  out.push(`/// 原版 Deduplicator<T> 1:1 — 去重存储, 返回稳定下标`);
  out.push(`#[derive(Debug)]`);
  out.push(`pub struct Deduplicator<T: Clone + Eq + Hash> {`);
  out.push(`  storage: Vec<T>,`);
  out.push(`  index: HashMap<T, usize>,`);
  out.push(`}`);
  out.push(``);
  out.push(`impl<T: Clone + Eq + Hash> Default for Deduplicator<T> {`);
  out.push(`  fn default() -> Self {`);
  out.push(`    Self { storage: Vec::new(), index: HashMap::new() }`);
  out.push(`  }`);
  out.push(`}`);
  out.push(``);
  out.push(`impl<T: Clone + Eq + Hash> Deduplicator<T> {`);
  out.push(`  /// 原版 Add: 已存在则复用, 否则插入; 返回稳定下标`);
  out.push(`  /// (原版 unordered_set::insert O(1); 值→下标双向表, 线扫退化已修)`);
  out.push(`  pub fn intern(&mut self, x: T) -> usize {`);
  out.push(`    if let Some(&i) = self.index.get(&x) {`);
  out.push(`      return i;`);
  out.push(`    }`);
  out.push(`    let i = self.storage.len();`);
  out.push(`    self.storage.push(x.clone());`);
  out.push(`    self.index.insert(x, i);`);
  out.push(`    i`);
  out.push(`  }`);
  out.push(``);
  out.push(`  #[must_use]`);
  out.push(`  pub fn get(&self, i: usize) -> &T { &self.storage[i] }`);
  out.push(`}`);
  out.push(``);
  out.push(`/// 原版 ResidueClass 1:1 — 模 2^m 剩余类 (对齐约束的部分知识)`);
  out.push(`#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]`);
  out.push(`pub struct ResidueClass {`);
  out.push(`  value: usize,`);
  out.push(`  modulus_log_2: usize,`);
  out.push(`}`);
  out.push(``);
  out.push(`impl ResidueClass {`);
  out.push(`  pub const K_MAX_MODULUS_LOG2: usize = usize::BITS as usize;`);
  out.push(``);
  out.push(`  #[must_use]`);
  out.push(`  pub fn new(value: usize, modulus_log_2: usize) -> Self {`);
  out.push(`    let m = modulus_log_2.min(Self::K_MAX_MODULUS_LOG2);`);
  out.push(`    // TODO(填体): value %= 1<<m 的归一化与乘法模数推导`);
  out.push(`    Self { value, modulus_log_2: m }`);
  out.push(`  }`);
  out.push(``);
  out.push(`  /// 原版 Unknown: 0 mod 1 = 全体整数`);
  out.push(`  #[must_use]`);
  out.push(`  pub fn unknown() -> Self { Self::new(0, 0) }`);
  out.push(``);
  out.push(`  #[must_use]`);
  out.push(`  pub fn single_value(&self) -> Option<usize> {`);
  out.push(`    if self.modulus_log_2 == Self::K_MAX_MODULUS_LOG2 {`);
  out.push(`      return Some(self.value);`);
  out.push(`    }`);
  out.push(`    None`);
  out.push(`  }`);
  out.push(``);
  out.push(`  /// 原版 AlignmentLog2`);
  out.push(`  #[must_use]`);
  out.push(`  pub fn alignment_log2(&self) -> usize {`);
  out.push(`    // TODO(填体): value==0 → modulus_log_2, 否则 value.trailing_zeros()`);
  out.push(`    let _ = self;`);
  out.push(`    0`);
  out.push(`  }`);
  out.push(``);
  out.push(`  // 原版 Add / Mul 语义见原版 operator+ / operator*; TODO(填体): add/mul 与对齐推导`);
  out.push(`}`);
  out.push(``);
  out.push(`/// 原版 StringLiteralQuote`);
  out.push(`#[must_use]`);
  out.push(`pub fn string_literal_quote(s: &str) -> String {`);
  out.push(`  // TODO(填体): 转义规则对齐原版 (换行/引号/反斜杠)`);
  out.push(`  let _ = s;`);
  out.push(`  String::new()`);
  out.push(`}`);
  out.push(``);
  out.push(`/// 原版 StringLiteralUnquote`);
  out.push(`#[must_use]`);
  out.push(`pub fn string_literal_unquote(s: &str) -> String {`);
  out.push(`  // TODO(填体): 去引号与反转义`);
  out.push(`  let _ = s;`);
  out.push(`  String::new()`);
  out.push(`}`);
  out.push(``);
  out.push(`/// 原版 FileUriDecode`);
  out.push(`pub fn file_uri_decode(s: &str) -> Result<String> {`);
  out.push(`  // TODO(填体): file:// URI → 文件路径`);
  out.push(`  let _ = s;`);
  out.push(`  Err(Error::Unimplemented)`);
  out.push(`}`);
  out.push(``);
  out.push(`/// 原版 IsLowerCamelCase`);
  out.push(`#[must_use]`);
  out.push(`pub fn is_lower_camel_case(s: &str) -> bool {`);
  out.push(`  // TODO(填体)`);
  out.push(`  let _ = s;`);
  out.push(`  false`);
  out.push(`}`);
  out.push(``);
  out.push(`/// 原版 IsUpperCamelCase`);
  out.push(`#[must_use]`);
  out.push(`pub fn is_upper_camel_case(s: &str) -> bool {`);
  out.push(`  // TODO(填体)`);
  out.push(`  let _ = s;`);
  out.push(`  false`);
  out.push(`}`);
  out.push(``);
  out.push(`/// 原版 IsSnakeCase`);
  out.push(`#[must_use]`);
  out.push(`pub fn is_snake_case(s: &str) -> bool {`);
  out.push(`  // TODO(填体)`);
  out.push(`  let _ = s;`);
  out.push(`  false`);
  out.push(`}`);
  out.push(``);
  out.push(`/// 原版 IsValidNamespaceConstName`);
  out.push(`#[must_use]`);
  out.push(`pub fn is_valid_namespace_const_name(s: &str) -> bool {`);
  out.push(`  // TODO(填体)`);
  out.push(`  let _ = s;`);
  out.push(`  false`);
  out.push(`}`);
  out.push(``);
  out.push(`/// 原版 IsValidTypeName`);
  out.push(`#[must_use]`);
  out.push(`pub fn is_valid_type_name(s: &str) -> bool {`);
  out.push(`  // TODO(填体)`);
  out.push(`  let _ = s;`);
  out.push(`  false`);
  out.push(`}`);
  out.push(``);
  out.push(`/// 原版 CapifyStringWithUnderscores`);
  out.push(`#[must_use]`);
  out.push(`pub fn capify_string_with_underscores(s: &str) -> String {`);
  out.push(`  // TODO(填体)`);
  out.push(`  let _ = s;`);
  out.push(`  String::new()`);
  out.push(`}`);
  out.push(``);
  out.push(`/// 原版 CamelifyString`);
  out.push(`#[must_use]`);
  out.push(`pub fn camelify_string(s: &str) -> String {`);
  out.push(`  // TODO(填体)`);
  out.push(`  let _ = s;`);
  out.push(`  String::new()`);
  out.push(`}`);
  out.push(``);
  out.push(`/// 原版 SnakeifyString`);
  out.push(`#[must_use]`);
  out.push(`pub fn snakeify_string(s: &str) -> String {`);
  out.push(`  // TODO(填体)`);
  out.push(`  let _ = s;`);
  out.push(`  String::new()`);
  out.push(`}`);
  out.push(``);
  out.push(`/// 原版 DashifyString`);
  out.push(`#[must_use]`);
  out.push(`pub fn dashify_string(s: &str) -> String {`);
  out.push(`  // TODO(填体)`);
  out.push(`  let _ = s;`);
  out.push(`  String::new()`);
  out.push(`}`);
  out.push(``);
  out.push(`/// 原版 UnderlinifyPath`);
  out.push(`#[must_use]`);
  out.push(`pub fn underlinify_path(path: &str) -> String {`);
  out.push(`  // TODO(填体)`);
  out.push(`  let _ = path;`);
  out.push(`  String::new()`);
  out.push(`}`);
  out.push(``);
  out.push(`/// 原版 StartsWithSingleUnderscore`);
  out.push(`#[must_use]`);
  out.push(`pub fn starts_with_single_underscore(s: &str) -> bool {`);
  out.push(`  // TODO(填体): 精确单下划线语义 (排除 __ 前缀)`);
  out.push(`  let _ = s;`);
  out.push(`  false`);
  out.push(`}`);
  out.push(``);
  out.push(`/// 原版 ReplaceFileContentsIfDifferent: 内容不同才覆写; 返回是否写入`);
  out.push(`pub fn replace_file_contents_if_different(file_path: &str, contents: &str) -> Result<bool> {`);
  out.push(`  let path = Path::new(file_path);`);
  out.push(`  let old = fs::read(path).ok();`);
  out.push(`  if old.as_deref() == Some(contents.as_bytes()) {`);
  out.push(`    return Ok(false);`);
  out.push(`  }`);
  out.push(`  fs::write(path, contents)?;`);
  out.push(`  Ok(true)`);
  out.push(`}`);
  out.push(``);
  out.push(`// 原版 kBaseNamespaceName / kTestNamespaceName`);
  out.push(`pub const K_BASE_NAMESPACE_NAME: &str = "base";`);
  out.push(`pub const K_TEST_NAMESPACE_NAME: &str = "test";`);
  out.push(``);
  out.push(sigList(relCc, "签名清单"));
  writeSrc(CRATE, "utils.rs", out.join("\n") + "\n");
}

// ===========================================================================
// global-context.{h,cc} → global_context.rs
// ===========================================================================
function genGlobalContext() {
  const relH = "src/torque/global-context.h";
  const relCc = "src/torque/global-context.cc";
  const out = [];
  out.push(banner("src/torque/misc.js", relH, "全局上下文 + 目标架构 (上下文单例 → 驱动持有结构体)"));
  out.push(`use crate::ast::Ast;`);
  out.push(`use crate::ast::IncludeSelector;`);
  out.push(`use crate::cpp_builder::File;`);
  out.push(`use crate::declarable::{Declarable, DeclarableId};`);
  out.push(`use crate::source_positions::SourceId;`);
  out.push(`use std::collections::{BTreeMap, BTreeSet};`);
  out.push(``);
  out.push(`/// 原版 CppInclude 1:1`);
  out.push(`#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]`);
  out.push(`pub struct CppInclude {`);
  out.push(`  pub include_path: String,`);
  out.push(`  pub include_selector: IncludeSelector,`);
  out.push(`}`);
  out.push(``);
  out.push(`impl CppInclude {`);
  out.push(`  #[must_use]`);
  out.push(`  pub fn csa_selected(&self) -> bool {`);
  out.push(`    matches!(self.include_selector, IncludeSelector::Csa | IncludeSelector::Any)`);
  out.push(`  }`);
  out.push(``);
  out.push(`  #[must_use]`);
  out.push(`  pub fn tsa_selected(&self) -> bool {`);
  out.push(`    matches!(self.include_selector, IncludeSelector::Tsa | IncludeSelector::Any)`);
  out.push(`  }`);
  out.push(`}`);
  out.push(``);
  out.push(`/// 原版 GlobalContext::KytheInlineMetadata 1:1`);
  out.push(`#[derive(Debug, Clone, Default)]`);
  out.push(`pub struct KytheInlineMetadata {`);
  out.push(`  pub begin: usize,`);
  out.push(`  pub end: usize,`);
  out.push(`  pub vname_sig: String,`);
  out.push(`  pub vname_path: String,`);
  out.push(`  pub source_begin: usize,`);
  out.push(`  pub source_end: usize,`);
  out.push(`  pub is_call_anchor: bool,`);
  out.push(`}`);
  out.push(``);
  out.push(`impl KytheInlineMetadata {`);
  out.push(`  /// 原版 Generates`);
  out.push(`  #[must_use]`);
  out.push(`  pub fn generates(begin: usize, end: usize, vname_sig: String, vname_path: String) -> Self {`);
  out.push(`    Self { begin, end, vname_sig, vname_path, source_begin: 0, source_end: 0, is_call_anchor: false }`);
  out.push(`  }`);
  out.push(``);
  out.push(`  /// 原版 CallAnchor`);
  out.push(`  #[must_use]`);
  out.push(`  pub fn call_anchor(begin: usize, end: usize, source_begin: usize, source_end: usize, vname_path: String) -> Self {`);
  out.push(`    Self { begin, end, vname_sig: String::new(), vname_path, source_begin, source_end, is_call_anchor: true }`);
  out.push(`  }`);
  out.push(`}`);
  out.push(``);
  out.push(`/// 原版 PerFileStreams — 单文件生成物集合`);
  out.push(`/// 登记偏差: 原版 csa_headerfile/csa_ccfile/class_definition_ccfile 三个 stringstream`);
  out.push(`/// 与 cpp::File 双层封装, Rust 版 File 直接持有 String 缓冲, 三字段并入 File.buf。`);
  out.push(`#[derive(Debug, Default)]`);
  out.push(`pub struct PerFileStreams {`);
  out.push(`  pub file: Option<SourceId>,`);
  out.push(`  pub csa_header: File,`);
  out.push(`  pub csa_header_rules: Vec<KytheInlineMetadata>,`);
  out.push(`  pub csa_cc: File,`);
  out.push(`  pub csa_cc_rules: Vec<KytheInlineMetadata>,`);
  out.push(`  pub class_definition_cc: File,`);
  out.push(`  pub required_builtin_includes: BTreeSet<SourceId>,`);
  out.push(`}`);
  out.push(``);
  out.push(`/// 原版 GlobalContext (ContextualClass 单例 → 编译驱动持有的结构体)`);
  out.push(`/// 声明ables 存储与 Ast 一样改为 arena; 指针字段 → DeclarableId。`);
  out.push(`#[derive(Debug, Default)]`);
  out.push(`pub struct GlobalContext {`);
  out.push(`  pub collect_language_server_data: bool,`);
  out.push(`  pub collect_kythe_data: bool,`);
  out.push(`  pub force_assert_statements: bool,`);
  out.push(`  pub annotate_ir: bool,`);
  out.push(`  pub torque_dwarf: bool,`);
  out.push(`  pub has_kythe_inline_metadata: bool,`);
  out.push(`  pub kythe_default_corpus: String,`);
  out.push(`  default_namespace: Option<DeclarableId>,`);
  out.push(`  pub ast: Ast,`);
  out.push(`  declarables: Vec<Declarable>,`);
  out.push(`  cpp_includes: BTreeSet<CppInclude>,`);
  out.push(`  generated_per_file: BTreeMap<SourceId, PerFileStreams>,`);
  out.push(`  fresh_ids: BTreeMap<String, usize>,`);
  out.push(`  macros_for_cc_debug_output: Vec<(DeclarableId, SourceId)>,`);
  out.push(`  macros_for_cc_debug_output_set: BTreeSet<(DeclarableId, SourceId)>,`);
  out.push(`  instance_types_initialized: bool,`);
  out.push(`}`);
  out.push(``);
  out.push(`impl GlobalContext {`);
  out.push(`  /// 原版 GlobalContext(Ast) 构造`);
  out.push(`  #[must_use]`);
  out.push(`  pub fn new(ast: Ast) -> Self {`);
  out.push(`    // TODO(填体): 构造 default_namespace (base / test 命名空间预声明)`);
  out.push(`    Self { ast, ..Self::default() }`);
  out.push(`  }`);
  out.push(``);
  out.push(`  /// 原版 RegisterDeclarable: 登记 arena, 返回稳定下标`);
  out.push(`  pub fn register_declarable(&mut self, d: Declarable) -> DeclarableId {`);
  out.push(`    self.declarables.push(d);`);
  out.push(`    (self.declarables.len() - 1) as DeclarableId`);
  out.push(`  }`);
  out.push(``);
  out.push(`  #[must_use]`);
  out.push(`  pub fn all_declarables(&self) -> &[Declarable] { &self.declarables }`);
  out.push(``);
  out.push(`  #[must_use]`);
  out.push(`  pub fn declarable(&self, id: DeclarableId) -> &Declarable {`);
  out.push(`    &self.declarables[id as usize]`);
  out.push(`  }`);
  out.push(``);
  out.push(`  #[must_use]`);
  out.push(`  pub fn declarable_mut(&mut self, id: DeclarableId) -> &mut Declarable {`);
  out.push(`    &mut self.declarables[id as usize]`);
  out.push(`  }`);
  out.push(``);
  out.push(`  /// 原版 GetDefaultNamespace`);
  out.push(`  #[must_use]`);
  out.push(`  pub const fn default_namespace(&self) -> Option<DeclarableId> { self.default_namespace }`);
  out.push(``);
  out.push(`  /// 原版 AddCppInclude`);
  out.push(`  pub fn add_cpp_include(&mut self, include_path: String, include_selector: IncludeSelector) {`);
  out.push(`    self.cpp_includes.insert(CppInclude { include_path, include_selector });`);
  out.push(`  }`);
  out.push(``);
  out.push(`  #[must_use]`);
  out.push(`  pub fn cpp_includes(&self) -> &BTreeSet<CppInclude> { &self.cpp_includes }`);
  out.push(``);
  out.push(`  /// 原版 GeneratedPerFile`);
  out.push(`  pub fn generated_per_file_mut(&mut self, file: SourceId) -> &mut PerFileStreams {`);
  out.push(`    let s = self.generated_per_file.entry(file).or_default();`);
  out.push(`    s.file = Some(file);`);
  out.push(`    s`);
  out.push(`  }`);
  out.push(``);
  out.push(`  #[must_use]`);
  out.push(`  pub fn generated_per_file(&self) -> &BTreeMap<SourceId, PerFileStreams> {`);
  out.push(`    &self.generated_per_file`);
  out.push(`  }`);
  out.push(``);
  out.push(`  /// 原版 MakeUniqueName`);
  out.push(`  #[must_use]`);
  out.push(`  pub fn make_unique_name(&mut self, base: &str) -> String {`);
  out.push(`    let n = self.fresh_ids.entry(base.to_owned()).or_default();`);
  out.push(`    *n += 1;`);
  out.push(`    format!("{base}_{}", *n - 1)`);
  out.push(`  }`);
  out.push(``);
  out.push(`  /// 原版 SetInstanceTypesInitialized`);
  out.push(`  pub fn set_instance_types_initialized(&mut self) {`);
  out.push(`    debug_assert!(!self.instance_types_initialized);`);
  out.push(`    self.instance_types_initialized = true;`);
  out.push(`  }`);
  out.push(``);
  out.push(`  #[must_use]`);
  out.push(`  pub const fn is_instance_types_initialized(&self) -> bool { self.instance_types_initialized }`);
  out.push(``);
  out.push(`  /// 原版 EnsureInCCDebugOutputList`);
  out.push(`  pub fn ensure_in_cc_debug_output_list(&mut self, macro_: DeclarableId, source: SourceId) {`);
  out.push(`    let item = (macro_, source);`);
  out.push(`    if self.macros_for_cc_debug_output_set.insert(item) {`);
  out.push(`      self.macros_for_cc_debug_output.push(item);`);
  out.push(`    }`);
  out.push(`  }`);
  out.push(``);
  out.push(`  #[must_use]`);
  out.push(`  pub fn all_macros_for_cc_debug_output(&self) -> &[(DeclarableId, SourceId)] {`);
  out.push(`    &self.macros_for_cc_debug_output`);
  out.push(`  }`);
  out.push(`}`);
  out.push(``);
  out.push(`/// 原版 TargetArchitecture (ContextualClass → 结构体, 编译选项 force_32bit 决定)`);
  out.push(`#[derive(Debug, Clone, Copy)]`);
  out.push(`pub struct TargetArchitecture {`);
  out.push(`  tagged_size: usize,`);
  out.push(`  raw_ptr_size: usize,`);
  out.push(`  smi_tag_and_shift_size: i32,`);
  out.push(`  external_ptr_size: usize,`);
  out.push(`  cppheap_ptr_size: usize,`);
  out.push(`  trusted_ptr_size: usize,`);
  out.push(`}`);
  out.push(``);
  out.push(`impl TargetArchitecture {`);
  out.push(`  /// 原版 TargetArchitecture(bool force_32bit)`);
  out.push(`  #[must_use]`);
  out.push(`  pub fn new(force_32bit: bool) -> Self {`);
  out.push(`    // TODO(填体): 64/32 位两套尺寸常量对齐 global-context.cc`);
  out.push(`    let _ = force_32bit;`);
  out.push(`    Self {`);
  out.push(`      tagged_size: 0,`);
  out.push(`      raw_ptr_size: 0,`);
  out.push(`      smi_tag_and_shift_size: 0,`);
  out.push(`      external_ptr_size: 0,`);
  out.push(`      cppheap_ptr_size: 0,`);
  out.push(`      trusted_ptr_size: 0,`);
  out.push(`    }`);
  out.push(`  }`);
  out.push(``);
  out.push(`  #[must_use]`);
  out.push(`  pub const fn tagged_size(&self) -> usize { self.tagged_size }`);
  out.push(``);
  out.push(`  #[must_use]`);
  out.push(`  pub const fn raw_ptr_size(&self) -> usize { self.raw_ptr_size }`);
  out.push(``);
  out.push(`  #[must_use]`);
  out.push(`  pub const fn external_pointer_size(&self) -> usize { self.external_ptr_size }`);
  out.push(``);
  out.push(`  #[must_use]`);
  out.push(`  pub const fn cppheap_pointer_size(&self) -> usize { self.cppheap_ptr_size }`);
  out.push(``);
  out.push(`  #[must_use]`);
  out.push(`  pub const fn trusted_pointer_size(&self) -> usize { self.trusted_ptr_size }`);
  out.push(``);
  out.push(`  /// 原版 ProtectedPointerSize == TaggedSize`);
  out.push(`  #[must_use]`);
  out.push(`  pub const fn protected_pointer_size(&self) -> usize { self.tagged_size }`);
  out.push(``);
  out.push(`  /// 原版 MaxHeapAlignment == TaggedSize`);
  out.push(`  #[must_use]`);
  out.push(`  pub const fn max_heap_alignment(&self) -> usize { self.tagged_size }`);
  out.push(``);
  out.push(`  #[must_use]`);
  out.push(`  pub const fn are_pointers_compressed(&self) -> bool { self.tagged_size < self.raw_ptr_size }`);
  out.push(``);
  out.push(`  #[must_use]`);
  out.push(`  pub const fn smi_tag_and_shift_size(&self) -> i32 { self.smi_tag_and_shift_size }`);
  out.push(`}`);
  out.push(``);
  out.push(sigList(relCc, "签名清单"));
  writeSrc(CRATE, "global_context.rs", out.join("\n") + "\n");
}

// ===========================================================================
// torque-compiler.{h,cc} → torque_compiler.rs + torque.cc → torque.rs
// ===========================================================================
function genCompiler() {
  const relH = "src/torque/torque-compiler.h";
  const relCc = "src/torque/torque-compiler.cc";
  const out = [];
  out.push(banner("src/torque/misc.js", relH, "编译器选项/结果/入口 (字段真转换)"));
  out.push(`use crate::error::{Error, Result};`);
  out.push(`use crate::kythe_data::KytheConsumer;`);
  out.push(`use crate::server_data::LanguageServerData;`);
  out.push(`use crate::source_positions::SourceFileMap;`);
  out.push(`use crate::utils::TorqueMessage;`);
  out.push(``);
  out.push(`/// 原版 TorqueCompilerOptions 1:1 (15 字段)`);
  out.push(`#[derive(Debug, Clone, Default)]`);
  out.push(`pub struct TorqueCompilerOptions {`);
  out.push(`  pub output_directory: String,`);
  out.push(`  pub v8_root: String,`);
  out.push(`  pub collect_language_server_data: bool,`);
  out.push(`  pub collect_kythe_data: bool,`);
  out.push(`  pub output_tsa: bool,`);
  out.push(`  /// dcheck(...) 仅 debug 构建生成; LS 支持可强制生成`);
  out.push(`  pub force_assert_statements: bool,`);
  out.push(`  /// Forge 32 位交叉编译模式`);
  out.push(`  pub force_32bit_output: bool,`);
  out.push(`  /// 输出附加 IR 注释`);
  out.push(`  pub annotate_ir: bool,`);
  out.push(`  /// 源路径含 v8-root 前缀时剥除`);
  out.push(`  pub strip_v8_root: bool,`);
  out.push(`  /// 生成 Torque DWARF 行信息`);
  out.push(`  pub torque_dwarf: bool,`);
  out.push(`  /// metagen layout JSON 路径; 空则关闭校验`);
  out.push(`  pub layout_json_path: String,`);
  out.push(`  /// @cppObjectLayoutDefinition 类的字段改由 layout JSON 提供 (过渡开关)`);
  out.push(`  pub use_cpp_layouts: bool,`);
  out.push(`  /// layout-positions.json 可选路径 (逐成员 C++ 源位置)`);
  out.push(`  pub layout_positions_path: String,`);
  out.push(`  /// 生成 #pragma kythe_inline_metadata`);
  out.push(`  pub kythe_inline_metadata: bool,`);
  out.push(`  pub kythe_default_corpus: String,`);
  out.push(`}`);
  out.push(``);
  out.push(`// 原版 CurrentCompilerOptions 上下文 → 显式传参 (登记偏差)`);
  out.push(``);
  out.push(`/// 原版 TorqueCompilerResult 1:1`);
  out.push(`#[derive(Debug, Default)]`);
  out.push(`pub struct TorqueCompilerResult {`);
  out.push(`  /// SourceId → 文件名映射, 错误时填充以便还原出错位置`);
  out.push(`  pub source_file_map: Option<SourceFileMap>,`);
  out.push(`  /// LS 数据, collect_language_server_data 开启时填充`);
  out.push(`  pub language_server_data: LanguageServerData,`);
  out.push(`  /// 编译期收集的错误消息`);
  out.push(`  pub messages: Vec<TorqueMessage>,`);
  out.push(`}`);
  out.push(``);
  out.push(`/// 原版 TorqueCompilationUnit 1:1`);
  out.push(`#[derive(Debug, Clone, Default)]`);
  out.push(`pub struct TorqueCompilationUnit {`);
  out.push(`  pub source_file_path: String,`);
  out.push(`  pub file_content: String,`);
  out.push(`}`);
  out.push(``);
  out.push(`/// 原版 CompileTorque(source, options)`);
  out.push(`pub fn compile_torque(source: &str, options: TorqueCompilerOptions) -> Result<TorqueCompilerResult> {`);
  out.push(`  // TODO(填体): CompileCurrentAst 全流程 (见原版 torque-compiler.cc)`);
  out.push(`  let _ = (source, options);`);
  out.push(`  Err(Error::Unimplemented)`);
  out.push(`}`);
  out.push(``);
  out.push(`/// 原版 CompileTorque(files, options) 重载`);
  out.push(`pub fn compile_torque_files(files: &[String], options: TorqueCompilerOptions) -> Result<TorqueCompilerResult> {`);
  out.push(`  // TODO(填体): 逐文件读入 + SourceFileMap 登记 + compile_torque`);
  out.push(`  let _ = (files, options);`);
  out.push(`  Err(Error::Unimplemented)`);
  out.push(`}`);
  out.push(``);
  out.push(`/// 原版 CompileTorqueForKythe`);
  out.push(`pub fn compile_torque_for_kythe(`);
  out.push(`  units: Vec<TorqueCompilationUnit>,`);
  out.push(`  options: TorqueCompilerOptions,`);
  out.push(`  kythe_consumer: &mut dyn KytheConsumer,`);
  out.push(`) -> Result<TorqueCompilerResult> {`);
  out.push(`  // TODO(填体)`);
  out.push(`  let _ = (units, options, kythe_consumer);`);
  out.push(`  Err(Error::Unimplemented)`);
  out.push(`}`);
  out.push(``);
  out.push(sigList(relCc, "主驱动流程签名清单"));
  writeSrc(CRATE, "torque_compiler.rs", out.join("\n") + "\n");

  // torque.cc → torque.rs (主入口)
  const relTq = "src/torque/torque.cc";
  const outTq = [];
  outTq.push(banner("src/torque/misc.js", relTq, "主入口 (命令行) 骨架"));
  outTq.push(`use crate::error::{Error, Result};`);
  outTq.push(`use crate::torque_compiler::TorqueCompilerOptions;`);
  outTq.push(``);
  outTq.push(`/// 原版 main/WrappedMain → run(argv)`);
  outTq.push(`/// argv[0] 之后为参数; 返回进程退出码 (原版 0/1)`);
  outTq.push(`pub fn run(argv: &[String], options: TorqueCompilerOptions) -> Result<i32> {`);
  outTq.push(`  // TODO(填体): 参数解析 (--output/-o, --v8-root, --language-server 等) 对齐 torque.cc`);
  outTq.push(`  let _ = (argv, options);`);
  outTq.push(`  Err(Error::Unimplemented)`);
  outTq.push(`}`);
  outTq.push(``);
  outTq.push(sigList(relTq, "主入口签名清单"));
  writeSrc(CRATE, "torque.rs", outTq.join("\n") + "\n");
}

// ===========================================================================
// kythe-data.{h,cc} → kythe_data.rs
// ===========================================================================
function genKythe() {
  const relH = "src/torque/kythe-data.h";
  const relCc = "src/torque/kythe-data.cc";
  const out = [];
  out.push(banner("src/torque/misc.js", relH, "Kythe 索引数据 (consumer trait + 数据收集)"));
  out.push(`use crate::error::{Error, Result};`);
  out.push(`use crate::source_positions::SourcePosition;`);
  out.push(``);
  out.push(`/// 原版 kythe_entity_t = uint64_t`);
  out.push(`pub type KytheEntity = u64;`);
  out.push(``);
  out.push(`/// 原版 KythePosition 1:1`);
  out.push(`#[derive(Debug, Clone, Default)]`);
  out.push(`pub struct KythePosition {`);
  out.push(`  pub file_path: String,`);
  out.push(`  pub start_offset: u64,`);
  out.push(`  pub end_offset: u64,`);
  out.push(`}`);
  out.push(``);
  out.push(`/// 原版 KytheConsumer::Kind 1:1`);
  out.push(`#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]`);
  out.push(`#[repr(u8)]`);
  out.push(`pub enum KytheKind { Unspecified, Constant, Function, ClassField, Variable, Type }`);
  out.push(``);
  out.push(`/// 原版 KytheConsumer (纯虚基类 → trait)`);
  out.push(`pub trait KytheConsumer {`);
  out.push(`  /// 原版 AddDefinition(kind, name, pos) — 默认返回 0`);
  out.push(`  fn add_definition(&mut self, kind: KytheKind, name: &str, pos: &KythePosition) -> KytheEntity {`);
  out.push(`    let _ = (kind, name, pos);`);
  out.push(`    0`);
  out.push(`  }`);
  out.push(``);
  out.push(`  /// 原版 AddDefinition(kind, name, pos, is_extern, readable_name) 重载 — 默认转发`);
  out.push(`  fn add_definition_ext(`);
  out.push(`    &mut self,`);
  out.push(`    kind: KytheKind,`);
  out.push(`    name: &str,`);
  out.push(`    pos: &KythePosition,`);
  out.push(`    is_extern: bool,`);
  out.push(`    readable_name: &str,`);
  out.push(`  ) -> KytheEntity {`);
  out.push(`    let _ = (is_extern, readable_name);`);
  out.push(`    self.add_definition(kind, name, pos)`);
  out.push(`  }`);
  out.push(``);
  out.push(`  /// 原版 AddUse`);
  out.push(`  fn add_use(&mut self, kind: KytheKind, entity: KytheEntity, use_pos: &KythePosition);`);
  out.push(``);
  out.push(`  /// 原版 AddCall`);
  out.push(`  fn add_call(`);
  out.push(`    &mut self,`);
  out.push(`    kind: KytheKind,`);
  out.push(`    caller_entity: KytheEntity,`);
  out.push(`    call_pos: &KythePosition,`);
  out.push(`    callee_entity: KytheEntity,`);
  out.push(`  );`);
  out.push(`}`);
  out.push(``);
  out.push(`/// 原版 KytheData (ContextualClass → 结构体; consumer 单持有)`);
  out.push(`#[derive(Default)]`);
  out.push(`pub struct KytheData {`);
  out.push(`  consumer: Option<Box<dyn KytheConsumer>>,`);
  out.push(`}`);
  out.push(``);
  out.push(`impl KytheData {`);
  out.push(`  /// 原版 SetConsumer`);
  out.push(`  pub fn set_consumer(&mut self, consumer: Box<dyn KytheConsumer>) {`);
  out.push(`    self.consumer = Some(consumer);`);
  out.push(`  }`);
  out.push(``);
  out.push(`  /// 原版 KytheData::AddDefinition 系列 — 转发 consumer`);
  out.push(`  pub fn add_definition(&mut self, kind: KytheKind, name: &str, pos: &KythePosition) -> KytheEntity {`);
  out.push(`    // TODO(填体): 原版无 consumer 时静默; 位置换算 (SourcePosition → KythePosition)`);
  out.push(`    self.consumer.as_mut().map_or(0, |c| c.add_definition(kind, name, pos))`);
  out.push(`  }`);
  out.push(``);
  out.push(`  pub fn add_use(&mut self, kind: KytheKind, entity: KytheEntity, use_pos: &KythePosition) {`);
  out.push(`    // TODO(填体)`);
  out.push(`    if let Some(c) = self.consumer.as_mut() {`);
  out.push(`      c.add_use(kind, entity, use_pos);`);
  out.push(`    }`);
  out.push(`  }`);
  out.push(``);
  out.push(`  pub fn add_call(&mut self, kind: KytheKind, caller: KytheEntity, pos: &KythePosition, callee: KytheEntity) {`);
  out.push(`    // TODO(填体)`);
  out.push(`    if let Some(c) = self.consumer.as_mut() {`);
  out.push(`      c.add_call(kind, caller, pos, callee);`);
  out.push(`    }`);
  out.push(`  }`);
  out.push(``);
  out.push(`  /// 原版 GetPosition: SourcePosition → KythePosition`);
  out.push(`  pub fn get_position(_pos: SourcePosition) -> Result<KythePosition> {`);
  out.push(`    // TODO(填体): SourceFileMap 路径 + 偏移换算`);
  out.push(`    Err(Error::Unimplemented)`);
  out.push(`  }`);
  out.push(`}`);
  out.push(``);
  out.push(sigList(relCc, "签名清单"));
  writeSrc(CRATE, "kythe_data.rs", out.join("\n") + "\n");
}

// ===========================================================================
// server-data.{h,cc} → server_data.rs
// ===========================================================================
function genServerData() {
  const relH = "src/torque/server-data.h";
  const relCc = "src/torque/server-data.cc";
  const out = [];
  out.push(banner("src/torque/misc.js", relH, "Language Server 数据 (定义跳转表)"));
  out.push(`use crate::declarable::DeclarableId;`);
  out.push(`use crate::global_context::GlobalContext;`);
  out.push(`use crate::source_positions::{LineAndColumn, SourceId, SourcePosition};`);
  out.push(`use crate::type_oracle::TypeOracle;`);
  out.push(`use std::collections::BTreeMap;`);
  out.push(``);
  out.push(`/// 原版 DefinitionsMap = map<SourcePosition, SourcePosition> → BTreeMap (确定性序)`);
  out.push(`pub type DefinitionsMap = BTreeMap<SourcePosition, SourcePosition>;`);
  out.push(``);
  out.push(`/// 原版 Symbols = vector<Declarable*>`);
  out.push(`pub type Symbols = Vec<DeclarableId>;`);
  out.push(``);
  out.push(`/// 原版 SymbolsMap = map<SourceId, Symbols>`);
  out.push(`pub type SymbolsMap = BTreeMap<SourceId, Symbols>;`);
  out.push(``);
  out.push(`/// 原版 LanguageServerData (ContextualClass → 结构体)`);
  out.push(`#[derive(Debug, Default)]`);
  out.push(`pub struct LanguageServerData {`);
  out.push(`  definitions_map: DefinitionsMap,`);
  out.push(`  symbols_map: SymbolsMap,`);
  out.push(`  global_context: Option<Box<GlobalContext>>,`);
  out.push(`  type_oracle: Option<Box<TypeOracle>>,`);
  out.push(`}`);
  out.push(``);
  out.push(`impl LanguageServerData {`);
  out.push(`  /// 原版 AddDefinition: token 位置 → 定义位置`);
  out.push(`  pub fn add_definition(&mut self, token: SourcePosition, definition: SourcePosition) {`);
  out.push(`    self.definitions_map.insert(token, definition);`);
  out.push(`  }`);
  out.push(``);
  out.push(`  /// 原版 FindDefinition: 同文件二分查找 token 所指定义`);
  out.push(`  #[must_use]`);
  out.push(`  pub fn find_definition(&self, source: SourceId, pos: LineAndColumn) -> Option<SourcePosition> {`);
  out.push(`    // TODO(填体): 原版按 (source, line, column) 上界二分`);
  out.push(`    let _ = (source, pos);`);
  out.push(`    None`);
  out.push(`  }`);
  out.push(``);
  out.push(`  /// 原版 SetGlobalContext (接管所有权 + 预处理符号表)`);
  out.push(`  pub fn set_global_context(&mut self, global_context: GlobalContext) {`);
  out.push(`    self.global_context = Some(Box::new(global_context));`);
  out.push(`    // TODO(填体): PrepareAllDeclarableSymbols (按 SourceId 拆分 + 过滤自动生成)`);
  out.push(`  }`);
  out.push(``);
  out.push(`  /// 原版 SetTypeOracle`);
  out.push(`  pub fn set_type_oracle(&mut self, type_oracle: TypeOracle) {`);
  out.push(`    self.type_oracle = Some(Box::new(type_oracle));`);
  out.push(`  }`);
  out.push(``);
  out.push(`  /// 原版 SymbolsForSourceId (原版 operator[] 默认空表 → 返回空切片, 登记偏差)`);
  out.push(`  #[must_use]`);
  out.push(`  pub fn symbols_for_source_id(&self, id: SourceId) -> &[DeclarableId] {`);
  out.push(`    self.symbols_map.get(&id).map_or(&[], |v| v.as_slice())`);
  out.push(`  }`);
  out.push(``);
  out.push(`  /// 原版 PrepareAllDeclarableSymbols (私有)`);
  out.push(`  pub fn prepare_all_declarable_symbols(&mut self) {`);
  out.push(`    // TODO(填体)`);
  out.push(`  }`);
  out.push(`}`);
  out.push(``);
  out.push(sigList(relCc, "签名清单"));
  writeSrc(CRATE, "server_data.rs", out.join("\n") + "\n");
}

// ===========================================================================
// layout-loader.{h,cc} → layout_loader.rs
// ===========================================================================
function genLayoutLoader() {
  const relH = "src/torque/layout-loader.h";
  const out = [];
  out.push(banner("src/torque/misc.js", relH, "C++ 布局 JSON 加载/校验"));
  out.push(`use crate::error::{Error, Result};`);
  out.push(``);
  out.push(`/// 原版 VerifyCppLayouts: metagen layout JSON 逐记录比对 @cppObjectLayoutDefinition`);
  out.push(`/// 类的 base/base size/header size/全部字段; 不一致即 Torque 错误。`);
  out.push(`/// 必须在 TypeOracle::FinalizeAggregateTypes 之后运行。`);
  out.push(`pub fn verify_cpp_layouts(layout_json_path: &str) -> Result<()> {`);
  out.push(`  // TODO(填体)`);
  out.push(`  let _ = layout_json_path;`);
  out.push(`  Err(Error::Unimplemented)`);
  out.push(`}`);
  out.push(``);
  out.push(`/// 原版 ImportCppLayouts: 用 layout JSON 填充 @cppObjectLayoutDefinition 类字段;`);
  out.push(`/// 必须在解析之后、CompileCurrentAst 之前运行。`);
  out.push(`/// positions_path 可选: 有则诊断指向 C++ 成员, 无则指向 .tq 类声明。`);
  out.push(`pub fn import_cpp_layouts(layout_json_path: &str, positions_path: &str) -> Result<()> {`);
  out.push(`  // TODO(填体)`);
  out.push(`  let _ = (layout_json_path, positions_path);`);
  out.push(`  Err(Error::Unimplemented)`);
  out.push(`}`);
  out.push(``);
  out.push(sigList("src/torque/layout-loader.cc", "签名清单 (1844 行, 校验逻辑大头)"));
  writeSrc(CRATE, "layout_loader.rs", out.join("\n") + "\n");
}

// ===========================================================================
// runtime-support.h / runtime-macro-shims.h → 占位模块
// ===========================================================================
function genRuntimeShims() {
  let out = banner("src/torque/misc.js", "src/torque/runtime-support.h", "原版 C++ 运行时支撑占位");
  out += `
// 原版 runtime-support.h 为 Torque 产出 C++ 代码的运行时支撑 (Runtime abort /
// CSA 支撑函数声明)。Rust 版 Torque 产出 Rust 代码, C++ 支撑头无对应物。
// 保留本模块登记原头职责, 待 CSA 生成器定型后决定是否需要 Rust 等价运行时。
`;
  writeSrc(CRATE, "runtime_support.rs", out);

  out = banner("src/torque/misc.js", "src/torque/runtime-macro-shims.h", "原版 C++ macro shims 占位");
  out += `
// 原版 runtime-macro-shims.h 定义 TorqueRuntimeMacroShims / TorqueDebugMacroShims
// 命名空间, 供生成 C++ 侧调用 CodeStubAssembler 成员 shim。Rust 版产出 Rust 代码,
// C++ shim 命名空间无对应物; 原版 ExternMacro 的 CCName 前缀逻辑保留在 declarable.rs。
`;
  writeSrc(CRATE, "runtime_macro_shims.rs", out);
}

// ===========================================================================
// 主流程
// ===========================================================================
genConstants();
genSourcePositions();
genUtils();
genGlobalContext();
genCompiler();
genKythe();
genServerData();
genLayoutLoader();
genRuntimeShims();
console.log("== v8-torque 基础设施组直出完毕: constants/source_positions/utils/global_context/torque_compiler/torque/kythe_data/server_data/layout_loader/runtime_support/runtime_macro_shims ==");
