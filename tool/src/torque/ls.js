#!/usr/bin/env bun
// v8/src/torque/ls/ (Language Server, 11 文件) → 直出 ls/ 子模块
//   ls/globals.h                    → ls/globals.rs
//   ls/json.h/.cc                   → ls/json.rs
//   ls/json-parser.h/.cc            → ls/json_parser.rs
//   ls/message.h                    → ls/message.rs
//   ls/message-handler.h/.cc        → ls/message_handler.rs
//   ls/message-macros.h             → ls/message_macros.rs
//   ls/message-pipe.h               → ls/message_pipe.rs
//   ls/torque-language-server.cc    → ls/torque_language_server.rs
// LS 组为编译器外围 (最后位): JSON 模型真转换 + 消息/处理器签名骨架。
import { readV8, writeSrc, banner } from "../../lib.js";

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

function topSigs(text) {
  const out = [];
  let ns = 0, fn = 0, unit = "", startLine = 0, live = true;
  const fin = () => {
    const s = unit.trim().replace(/\s+/g, " ");
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
        if (fn === 0) {
          const u = unit.trim();
          if (u === "" || /^(namespace|extern)\b/.test(u)) { ns++; unit = ""; live = true; continue; }
          if (live) fin();
          unit = ""; live = false; fn++;
        } else fn++;
      } else if (ch === "}") {
        if (fn > 0) fn--; else ns--;
      } else if (fn === 0 && live) {
        if (!unit.trim()) startLine = i + 1;
        unit += ch;
      }
    }
    if (fn === 0) {
      if (/;\s*$/.test(raw) && !unit.includes("(")) unit = "";
      live = true;
    }
  }
  return out;
}

function sigBlock(f, note = "签名清单 (填体正交任务源)") {
  const s = readV8(f);
  let dd = `// ── 原版 ${f} ──\n`;
  const sigs = topSigs(s);
  if (sigs.length === 0 && f.endsWith(".cc") && s.length > 400) {
    console.error(`[torque] ${f} 非空但抽到 0 条签名 — noCmt/topSigs 异常, 防静默检查`);
  }
  for (const x of sigs) dd += `// L${x.line}: ${x.sig}\n`;
  return dd;
}

// ===========================================================================
// ls/globals.h → ls/globals.rs
// ===========================================================================
function genLsGlobals() {
  let d = banner("src/torque/ls.js", "src/torque/ls/globals.h", "LS 全局: Logger (上下文单例 → 结构体)");
  d += `
// 原版 Logger 上下文单例 (VSCode 下 stdout 被协议占用, 诊断写文件) → 结构体;
// TorqueFileList 上下文 → 编译驱动字段 (登记偏差)。
`;
  d += `/// 原版 Logger (ContextualClass → 结构体)\n#[derive(Debug, Default)]\npub struct Logger {\n  enabled: bool,\n  /// 原版 std::ofstream → 缓冲 + 路径 (惰性打开)\n  logfile_path: Option<String>,\n  buffer: String,\n}\n\n`;
  d += `impl Logger {\n  /// 原版 Enable\n  pub fn enable(&mut self, path: String) {\n    self.enabled = true;\n    self.logfile_path = Some(path);\n  }\n\n`;
  d += `  #[must_use]\n  pub const fn enabled(&self) -> bool {\n    self.enabled\n  }\n\n`;
  d += `  /// 原版 Log → 文件缓冲
  pub fn log(&mut self, line: &str) {
    if !self.enabled {
      return;
    }
    // TODO(填体): 接 log crate (dev 依赖, 运行时日志走文件缓冲)
    self.buffer.push_str(line);
    self.buffer.push('\\n');
  }

`;
  d += `  /// 原版 Flush\n  pub fn flush(&mut self) {\n    // TODO(填体): 落盘 logfile_path\n  }\n}\n\n`;
  d += `/// 原版 TorqueFileList 上下文 → 显式传递\npub type TorqueFileList = Vec<String>;\n`;
  writeSrc("v8-torque", "ls/globals.rs", d);
}

// ===========================================================================
// ls/json.{h,cc} → ls/json.rs (真转换: tag 枚举 → Rust enum)
// ===========================================================================
function genLsJson() {
  let d = banner("src/torque/ls.js", "src/torque/ls/json.h", "LS JSON 模型 (tag union → enum)");
  d += `
use crate::error::{Error, Result};
use std::collections::BTreeMap;

// 原版 JsonValue tag{OBJECT,ARRAY,STRING,NUMBER,BOOL,IS_NULL} + 5 字段 union →
// Rust enum; unique_ptr 成员由 enum 载荷直接承担。JsonObject/std::map → BTreeMap
// (确定性序, 登记偏差: 原版 map 本就有序)。
`;

  d += `/// 原版 using JsonObject = map<string, JsonValue>\npub type JsonObject = BTreeMap<String, JsonValue>;\n\n`;
  d += `/// 原版 using JsonArray = vector<JsonValue>\npub type JsonArray = Vec<JsonValue>;\n\n`;
  d += `/// 原版 JsonValue 1:1 (tag + union → enum)\n#[derive(Debug, Clone, PartialEq, Default)]\npub enum JsonValue {\n  Object(JsonObject),\n  Array(JsonArray),\n  String(String),\n  Number(f64),\n  Bool(bool),\n  #[default]\n  Null,\n}\n\n`;
  d += `/// 原版 JsonMember (object 键值对)\n#[derive(Debug, Clone, PartialEq)]\npub struct JsonMember {\n  pub name: String,\n  pub value: JsonValue,\n}\n\n`;
  d += `impl JsonValue {\n  /// 原版 From(double)\n  #[must_use]\n  pub fn number(n: f64) -> Self {\n    Self::Number(n)\n  }\n\n`;
  d += `  /// 原版 From(JsonObject)\n  #[must_use]\n  pub fn object(o: JsonObject) -> Self {\n    Self::Object(o)\n  }\n\n`;
  d += `  /// 原版 From(bool)\n  #[must_use]\n  pub fn boolean(b: bool) -> Self {\n    Self::Bool(b)\n  }\n\n`;
  d += `  /// 原版 From(string)\n  #[must_use]\n  pub fn string(s: String) -> Self {\n    Self::String(s)\n  }\n\n`;
  d += `  /// 原版 From(JsonArray)\n  #[must_use]\n  pub fn array(a: JsonArray) -> Self {\n    Self::Array(a)\n  }\n\n`;
  d += `  /// 原版 JsonNull\n  #[must_use]\n  pub fn json_null() -> Self {\n    Self::Null\n  }\n\n`;
  d += `  /// 原版 IsNumber/ToNumber → 健壮返回\n  pub fn to_number(&self) -> Result<f64> {\n    match self {\n      Self::Number(n) => Ok(*n),\n      _ => Err(Error::Unimplemented),\n    }\n  }\n\n`;
  d += `  /// 原版 ToBool\n  pub fn to_bool(&self) -> Result<bool> {\n    match self {\n      Self::Bool(b) => Ok(*b),\n      _ => Err(Error::Unimplemented),\n    }\n  }\n\n`;
  d += `  /// 原版 ToString\n  pub fn to_string_ref(&self) -> Result<&str> {\n    match self {\n      Self::String(s) => Ok(s),\n      _ => Err(Error::Unimplemented),\n    }\n  }\n\n`;
  d += `  /// 原版 ToObject\n  pub fn to_object(&self) -> Result<&JsonObject> {\n    match self {\n      Self::Object(o) => Ok(o),\n      _ => Err(Error::Unimplemented),\n    }\n  }\n\n`;
  d += `  /// 原版 ToArray\n  pub fn to_array(&self) -> Result<&JsonArray> {\n    match self {\n      Self::Array(a) => Ok(a),\n      _ => Err(Error::Unimplemented),\n    }\n  }\n`;
  d += `}\n\n`;
  d += `/// 原版 SerializeToString\n#[must_use]\npub fn serialize_to_string(value: &JsonValue) -> String {\n  // TODO(填体): 序列化格式对齐原版 json.cc\n  let _ = value;\n  String::new()\n}\n`;
  d += sigBlock("src/torque/ls/json.cc", "签名清单");
  writeSrc("v8-torque", "ls/json.rs", d);
}

// ===========================================================================
// ls/json-parser.{h,cc} → ls/json_parser.rs
// ===========================================================================
function genLsJsonParser() {
  let d = banner("src/torque/ls.js", "src/torque/ls/json-parser.h", "LS JSON 解析器");
  d += `
use crate::error::{Error, Result};
use crate::ls::json::JsonValue;

/// 原版 JsonParserResult 1:1 (error 字段解析失败时携带消息)
#[derive(Debug, Clone, Default)]
pub struct JsonParserResult {
  pub value: Option<JsonValue>,
  pub error: Option<String>,
}

impl JsonParserResult {
  /// 原版成功路径
  #[must_use]
  pub const fn success(value: JsonValue) -> Self {
    Self { value: Some(value), error: None }
  }

  /// 原版失败路径 (原版静态标志 → 显式字段, 登记偏差)
  #[must_use]
  pub fn failure(message: String) -> Self {
    Self { value: None, error: Some(message) }
  }
}

/// 原版 ParseJson
pub fn parse_json(input: &str) -> Result<JsonParserResult> {
  // TODO(填体): 手写递归下降 (LS 组自含, 不引 JSON 库)
  let _ = input;
  Err(Error::Unimplemented)
}
`;
  d += sigBlock("src/torque/ls/json-parser.cc", "签名清单");
  writeSrc("v8-torque", "ls/json_parser.rs", d);
}

// ===========================================================================
// ls/message.h → ls/message.rs
// ===========================================================================
function genLsMessage() {
  const s = noCmt(readV8("src/torque/ls/message.h"));
  let d = banner("src/torque/ls.js", "src/torque/ls/message.h", "LS 消息模型 (LSP JSON 协议)");
  d += `
use crate::ls::json::JsonObject;

// 原版 BaseJsonAccessor/Message/NestedJsonAccessor 是 JSON 树上的强类型访问器;
// Rust 版直接以 JsonValue 树承载, 各消息类型保留字段清单 (访问器骨架见原版)。
`;

  // 类清单 → 字段骨架 (访问器类字段少, 机械提取)
  const classes = ["Message", "ResponseError", "InitializeParams", "FileListParams",
    "DidChangeWatchedFilesRegistrationOptions", "FileEvent", "DidChangeWatchedFilesParams",
    "SaveOptions", "TextDocumentSyncOptions", "ServerCapabilities", "InitializeResult",
    "Registration", "RegistrationParams", "JsonPosition", "Range", "Location",
    "TextDocumentIdentifier", "TextDocumentPositionParams", "Diagnostic",
    "PublishDiagnosticsParams"];
  for (const n of classes) {
    d += `/// 原版 ${n} (JSON 访问器; 字段清单见原版)\n#[derive(Debug, Clone, Default)]\npub struct ${n} {\n  pub object: JsonObject,\n}\n\n`;
  }
  d += `/// 原版 FileSystemWatcher::WatchKind
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum WatchKind { Create, Change, Delete }

`;
  d += `/// 原版 Diagnostic::DiagnosticSeverity
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum DiagnosticSeverity { Error, Warning, Information, Hint }

`;
  d += `/// 原版 SymbolKind (无作用域 enum)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum SymbolKind { File, Module, Namespace, Package, Class, Method, Property, Field, Constructor, Enum, Interface, Function, Variable, Constant, String, Number, Boolean, Array, Object, Key, Null, EnumMember, Struct, Event, Operator, TypeParameter }

`;
  d += `// 填体提示: 原版访问器方法 (set/get/ToJsonObject) 按 JsonValue 树直填;
// helper 函数 (ToJson/FromJson) 随 message_handler 填体补齐。
`;
  writeSrc("v8-torque", "ls/message.rs", d);
}

// ===========================================================================
// ls/message-handler.{h,cc} → ls/message_handler.rs
// ===========================================================================
function genLsMessageHandler() {
  let d = banner("src/torque/ls.js", "src/torque/ls/message-handler.h", "LS 消息处理器");
  d += `
use crate::error::{Error, Result};
use crate::ls::json::JsonValue;

/// 原版 MessageHandler 1:1 (纯虚 base → trait)
pub trait MessageHandler {
  /// 原版 HandleMessage
  fn handle_message(&mut self, message: &JsonValue) -> Result<()>;
}

/// 原版 CompilerHandler : MessageHandler (定义于 message-handler.cc)
#[derive(Debug, Default)]
pub struct CompilerHandler;

impl MessageHandler for CompilerHandler {
  fn handle_message(&mut self, message: &JsonValue) -> Result<()> {
    // TODO(填体): initialize / didOpenTextDocument / didChangeTextDocument /
    //   debugging request 分派 (见 .cc 清单)
    let _ = message;
    Err(Error::Unimplemented)
  }
}
`;
  d += sigBlock("src/torque/ls/message-handler.cc", "签名清单");
  writeSrc("v8-torque", "ls/message_handler.rs", d);
}

// ===========================================================================
// ls/message-macros.h → ls/message_macros.rs
// ===========================================================================
function genLsMessageMacros() {
  let d = banner("src/torque/ls.js", "src/torque/ls/message-macros.h", "LS 消息访问器宏 (登记占位)");
  d += `
// 原版 message-macros.h 定义 FIELD_ACCESSOR/ACCESSOR_SET/ACCESSOR_GET 等 X-macro,
// 为 message.h 的 JSON 访问器类批量生成 set/get 方法。Rust 版 message.rs 直接以
// JsonValue 树 + 结构体字段承载, 宏无对应物; 保留本模块登记原头职责。
`;
  writeSrc("v8-torque", "ls/message_macros.rs", d);
}

// ===========================================================================
// ls/message-pipe.{h,cc} → ls/message_pipe.rs
// ===========================================================================
function genLsMessagePipe() {
  let d = banner("src/torque/ls.js", "src/torque/ls/message-pipe.h", "LS 消息管道 (stdin/stdout LSP 帧)");
  d += `
use crate::error::{Error, Result};
use crate::ls::json::JsonValue;

/// 原版 ReadMessage: 从 stdin 读 Content-Length 帧并解析
pub fn read_message() -> Result<JsonValue> {
  // TODO(填体): Content-Length 帧协议
  Err(Error::Unimplemented)
}

/// 原版 WriteMessage: 序列化并写 stdout
pub fn write_message(message: JsonValue) -> Result<()> {
  // TODO(填体)
  let _ = message;
  Err(Error::Unimplemented)
}
`;
  writeSrc("v8-torque", "ls/message_pipe.rs", d);
}

// ===========================================================================
// ls/torque-language-server.cc → ls/torque_language_server.rs
// ===========================================================================
function genLsMain() {
  let d = banner("src/torque/ls.js", "src/torque/ls/torque-language-server.cc", "LS 主循环");
  d += `
use crate::error::{Error, Result};

/// 原版 RunLanguageServer: 读入文件清单 + 消息循环
pub fn run_language_server(files: Vec<String>) -> Result<i32> {
  // TODO(填体): Logger::Enable / CompilerHandler 循环
  let _ = files;
  Err(Error::Unimplemented)
}
`;
  d += sigBlock("src/torque/ls/torque-language-server.cc", "签名清单");
  writeSrc("v8-torque", "ls/torque_language_server.rs", d);
}

// ===========================================================================
// 主流程
// ===========================================================================
genLsGlobals();
genLsJson();
genLsJsonParser();
genLsMessage();
genLsMessageHandler();
genLsMessageMacros();
genLsMessagePipe();
genLsMain();
console.log("== v8-torque LS 组直出完毕: ls/{globals,json,json_parser,message,message_handler,message_macros,message_pipe,torque_language_server} ==");
