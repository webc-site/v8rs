#!/usr/bin/env bun
// v8/src/torque/earley-parser.{h,cc} + torque-parser.{h,cc} → 直出
//   earley-parser.h/.cc → earley_parser.rs (Earley 通用解析: ParseResult/Symbol/Rule/Item/Lexer/Grammar)
//   torque-parser.h/.cc → torque_parser.rs (Torque 文法: 入口 + token 表 + 产生式动作清单)
// ParseResult 的类型擦除 → Rust enum ParseResultValue (TypeId 变体 1:1);
// Symbol/Rule/Item 裸指针互联 → SymbolId/RuleId/ItemId arena。
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
  // 逐行词法段化 (词法态跨行存续): code 段跑 scope 判定, 字面量段原文入 unit
  // (P2-1: 只擦注释不擦字符串, Token(",") 等串内容保真; 串内 {} 不参与 scope 计数)
  const lex = { st: "code", delim: "", buf: "", emitted: 0 };
  for (const [i, raw] of text.split("\n").entries()) {
    const segs = lexLine(raw, lex);
    // 行视图 = 注释剥离 + 串占位 (等价旧 noCmt 输出), 行级判定沿用
    const view = segs.map((s) => (s.s !== undefined ? (s.s.startsWith("'") ? "''" : '""') : s.c)).join("");
    if (/^\s*#/.test(view)) continue;
    // 顶层全大写宏行 (EXPORT_CONTEXTUAL_VARIABLE(...) / DISABLE_CFI_ICALL) 不入签名
    if (/^[A-Z][A-Z0-9_]*(\(.*\))?\s*;?\s*$/.test(view.trim())) continue;
    for (const seg of segs) {
      if (seg.s !== undefined) {
        // 字面量段: 原文入 unit, 内部括号/花括号不参与 scope
        if (top() === "ns" || top() === "cls") {
          if (!unit.trim()) startLine = i + 1;
          unit += seg.s;
        }
        continue;
      }
      for (const ch of seg.c) {
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
    }
    // `;` 结尾单元单独出条 (P2-3: 防多条声明粘连 1 条), 无 `(` 的纯声明仅清空
    if (/;\s*$/.test(view) && top() !== "fn") {
      if (unit.includes("(")) fin();
      unit = "";
    }
  }
  return out;
}

// 行级词法段化: 产出 [{c: "code 可见字符"} | {s: "字面量原文"}] 有序段;
// 词法态跨行存续 (块注释 / 行注释 / "…" 转义续行 / '…' / R"delim(…)delim"),
// 跨行字面量按行切片成段 (buf/emitted 记已发量), 拼接各段即完整原文
function lexLine(raw, lex) {
  const segs = [];
  let code = "";
  const flushCode = () => {
    if (code) { segs.push({ c: code }); code = ""; }
  };
  // 字面量收口: 发剩余切片并复位; 行尾未收口只发切片, buf 留存续行
  const closeStr = () => {
    if (lex.emitted < lex.buf.length) segs.push({ s: lex.buf.slice(lex.emitted) });
    lex.buf = "";
    lex.emitted = 0;
  };
  const emitTail = () => {
    if (lex.emitted < lex.buf.length) {
      segs.push({ s: lex.buf.slice(lex.emitted) });
      lex.emitted = lex.buf.length;
    }
  };
  let i = 0;
  const n = raw.length;
  while (i < n) {
    if (lex.st === "block") {
      const end = raw.indexOf("*/", i);
      if (end < 0) { i = n; } else { lex.st = "code"; i = end + 2; }
    } else if (lex.st === "line") {
      i = n;
    } else if (lex.st === "str" || lex.st === "char") {
      const q = lex.st === "str" ? '"' : "'";
      const c = raw[i];
      if (c === "\\" && i + 1 < n) { lex.buf += raw.slice(i, i + 2); i += 2; }
      else if (c === q) { lex.buf += c; closeStr(); lex.st = "code"; i++; }
      else { lex.buf += c; i++; }
    } else if (lex.st === "raw") {
      const close = ")" + lex.delim + '"';
      const end = raw.indexOf(close, i);
      const stop = end < 0 ? n : end + close.length;
      lex.buf += raw.slice(i, stop);
      i = stop;
      if (end >= 0) { closeStr(); lex.st = "code"; }
    } else {
      const c = raw[i], c2 = i + 1 < n ? raw[i + 1] : "";
      if (c === "/" && c2 === "*") { flushCode(); lex.st = "block"; i += 2; }
      else if (c === "/" && c2 === "/") { flushCode(); lex.st = "line"; i += 2; }
      else if (c === '"' || (c === "R" && /^R"[^ ()\\\t\v\f\n]*\(/.test(raw.slice(i, i + 64)))) {
        flushCode();
        const m = c === "R" ? /^R"([^ ()\\\t\v\f\n]*)\(/.exec(raw.slice(i, i + 64)) : null;
        if (m) { lex.st = "raw"; lex.delim = m[1]; lex.buf = m[0]; lex.emitted = 0; i += m[0].length; }
        else { lex.st = "str"; lex.buf = '"'; lex.emitted = 0; i++; }
      } else if (c === "'") { flushCode(); lex.st = "char"; lex.buf = "'"; lex.emitted = 0; i++; }
      else { code += c; i++; }
    }
  }
  if (lex.st === "line") lex.st = "code";
  emitTail();
  flushCode();
  return segs;
}

function classBlock(header, text, from = 0) {
  const re = new RegExp(`(?:class|struct|enum class)\\s+(?:V8_EXPORT_PRIVATE\\s+|final\\s+)*${header}(?:\\s+final)?[^\\{;]*`);
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

function sigBlock(f, note = "签名清单 (填体正交任务源)") {
  const s = readV8(f);
  let dd = `// ── 原版 ${f} ──\n`;
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
// earley-parser.{h,cc} → earley_parser.rs
// ===========================================================================
function genEarley() {
  const relH = "src/torque/earley-parser.h";
  const eh = noCmt(readV8(relH));

  // ParseResultTypeId 变体先行提取, banner 动态计数 (P2-1: 硬编码 56 漂移 → 57)
  const mTypeId = eh.match(/enum class ParseResultHolderBase::TypeId\s*\{([\s\S]*?)\};/);
  if (!mTypeId) throw new Error("ParseResultTypeId 未提取");
  const vs = [...mTypeId[1].matchAll(/\bk(\w+),/g)].map((x) => x[1]);

  let d = banner("src/torque/parser.js", relH, `Earley 解析器 (ParseResultValue ${vs.length} 变体 + arena)`);
  d += `
use crate::ast::{
  Annotation, AnnotationParameter, BitFieldDeclaration, ClassFieldExpression,
  GenericParameter, ImplicitParameters, IncrementDecrementOperator, IntegerLiteral, LabelAndTypes,
  NameAndExpression, NameAndTypeExpression, NodeId, ParameterList, StructFieldExpression,
  TypeswitchCase,
};
use crate::error::{Error, Result};
use crate::ls::json::{JsonMember, JsonValue};
use crate::source_positions::SourcePosition;
use crate::torque_parser::{EnumEntry, ExpressionWithSource, TypeList};
use std::collections::{BTreeMap, BTreeSet};

// ParseResult 的 dynamicany → Rust enum ParseResultValue (TypeId 变体 1:1);
// Symbol/Rule/Item 裸指针互联 → SymbolId/RuleId/ItemId arena (GrammarTables);
// InputPosition (const char*) → 字节偏移 usize + 源串显式传参 (登记偏差)。
`;

  // ParseResultTypeId 全量展开
  {
    const m = mTypeId;
    // 变体名 → Rust 载荷类型映射 (1:1)
    const PAYLOAD = {
      StdString: "String", Bool: "bool", Int32: "i32", Double: "f64",
      IntegerLiteral: "IntegerLiteral", StdVectorOfString: "Vec<String>",
      ExpressionPtr: "NodeId", IdentifierPtr: "NodeId", OptionalIdentifierPtr: "Option<NodeId>",
      StatementPtr: "NodeId", DeclarationPtr: "NodeId", TypeExpressionPtr: "NodeId",
      OptionalTypeExpressionPtr: "Option<NodeId>", TryHandlerPtr: "NodeId",
      NameAndTypeExpression: "NameAndTypeExpression", EnumEntry: "EnumEntry",
      StdVectorOfEnumEntry: "Vec<EnumEntry>", ImplicitParameters: "ImplicitParameters",
      OptionalImplicitParameters: "Option<ImplicitParameters>",
      NameAndExpression: "NameAndExpression", Annotation: "Annotation",
      VectorOfAnnotation: "Vec<Annotation>", AnnotationParameter: "AnnotationParameter",
      OptionalAnnotationParameter: "Option<AnnotationParameter>",
      ClassFieldExpression: "ClassFieldExpression", StructFieldExpression: "StructFieldExpression",
      BitFieldDeclaration: "BitFieldDeclaration",
      StdVectorOfNameAndTypeExpression: "Vec<NameAndTypeExpression>",
      StdVectorOfNameAndExpression: "Vec<NameAndExpression>",
      StdVectorOfClassFieldExpression: "Vec<ClassFieldExpression>",
      StdVectorOfStructFieldExpression: "Vec<StructFieldExpression>",
      StdVectorOfBitFieldDeclaration: "Vec<BitFieldDeclaration>",
      IncrementDecrementOperator: "IncrementDecrementOperator",
      OptionalStdString: "Option<String>", StdVectorOfStatementPtr: "Vec<NodeId>",
      StdVectorOfDeclarationPtr: "Vec<NodeId>", StdVectorOfStdVectorOfDeclarationPtr: "Vec<Vec<NodeId>>",
      StdVectorOfExpressionPtr: "Vec<NodeId>", ExpressionWithSource: "ExpressionWithSource",
      ParameterList: "ParameterList", TypeList: "TypeList", OptionalTypeList: "Option<TypeList>",
      LabelAndTypes: "LabelAndTypes", StdVectorOfLabelAndTypes: "Vec<LabelAndTypes>",
      StdVectorOfTryHandlerPtr: "Vec<NodeId>", OptionalStatementPtr: "Option<NodeId>",
      OptionalExpressionPtr: "Option<NodeId>", TypeswitchCase: "TypeswitchCase",
      StdVectorOfTypeswitchCase: "Vec<TypeswitchCase>", StdVectorOfIdentifierPtr: "Vec<NodeId>",
      OptionalClassBody: "Option<NodeId>", GenericParameter: "GenericParameter",
      GenericParameters: "Vec<GenericParameter>",
      JsonValue: "JsonValue", JsonMember: "JsonMember",
      StdVectorOfJsonValue: "Vec<JsonValue>",
      StdVectorOfJsonMember: "Vec<JsonMember>",
    };
    d += `/// 原版 ParseResultHolderBase::TypeId 1:1 (${vs.length} 变体)\n#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]\n#[repr(u8)]\npub enum ParseResultTypeId {\n`;
    for (const v of vs) d += `  ${v.replace(/^k/, "")},\n`;
    d += `}\n\n`;
    const missing = vs.filter((v) => !(v.replace(/^k/, "") in PAYLOAD));
    if (missing.length) throw new Error(`ParseResultValue 载荷缺失: ${missing.join(",")}`);
    d += `/// 原版 ParseResult (类型擦除 holder) → 值枚举; ParseResult 即其别名\n#[derive(Debug, Clone)]\npub enum ParseResultValue {\n`;
    for (const v of vs) d += `  ${v.replace(/^k/, "")}(${PAYLOAD[v.replace(/^k/, "")]}),\n`;
    d += `}\n\n`;
    d += `/// 原版 using ParseResultTypeId\npub type ParseResult = ParseResultValue;\n\n`;
  }

  d += `/// 原版 using InputPosition = const char* → 字节偏移\npub type InputPosition = usize;\n\n`;
  d += `/// 原版 MatchedInput 1:1 (begin/end 为源串字节偏移)\n#[derive(Debug, Clone)]\npub struct MatchedInput {\n  pub begin: InputPosition,\n  pub end: InputPosition,\n  pub pos: SourcePosition,\n}\n\n`;
  d += `impl MatchedInput {\n  /// 原版 ToString: 取源串切片\n  #[must_use]\n  pub fn matched_text<'a>(&self, input: &'a str) -> &'a str {\n    &input[self.begin..self.end]\n  }\n}\n\n`;

  d += `/// 原版 ParseResultIterator 1:1 (产生式动作的子结果游标)\n#[derive(Debug)]\npub struct ParseResultIterator {\n  results: Vec<ParseResultValue>,\n  i: usize,\n  matched_input: MatchedInput,\n}\n\n`;
  d += `impl ParseResultIterator {\n  #[must_use]\n  pub fn new(results: Vec<ParseResultValue>, matched_input: MatchedInput) -> Self {\n    Self { results, i: 0, matched_input }\n  }\n\n`;
  d += `  /// 原版 Next (越界为调用方契约 CHECK; Rust 以 Option 表达)\n  pub fn next_result(&mut self) -> Option<ParseResultValue> {\n    let r = self.results.get(self.i).cloned();\n    if r.is_some() {\n      self.i += 1;\n    }\n    r\n  }\n\n`;
  d += `  #[must_use]\n  pub fn has_next(&self) -> bool { self.i < self.results.len() }\n\n`;
  d += `  #[must_use]\n  pub const fn matched_input(&self) -> &MatchedInput { &self.matched_input }\n`;
  d += `}\n\n`;

  d += `/// 原版 LexerResult 1:1\n#[derive(Debug, Clone)]\npub struct LexerResult {\n  pub token_symbols: Vec<SymbolId>,\n  pub token_contents: Vec<MatchedInput>,\n}\n\n`;

  d += `/// 原版 Action = optional<ParseResult>(*)(ParseResultIterator*)\n/// 原版动作经 Contextual 拿 CurrentAst/CurrentSourcePosition; Rust 版由调用方\n/// 线程化 (登记偏差, 填体时定形)。\npub type Action = fn(&mut ParseResultIterator) -> Option<ParseResultValue>;\n\n`;
  d += `/// 原版 DefaultAction\n#[must_use]\npub fn default_action(child_results: &mut ParseResultIterator) -> Option<ParseResultValue> {\n  child_results.next_result()\n}\n\n`;

  d += `/// 符号 arena 索引 (对应 C++ Symbol*)\npub type SymbolId = u32;\n`;
  d += `/// 规则 arena 索引 (对应 C++ Rule*)\npub type RuleId = u32;\n`;
  d += `/// 项 arena 索引 (对应 C++ const Item*)\npub type ItemId = u32;\n\n`;

  d += `/// 原版 Rule 1:1 (left 由 Symbol::AddRule 回填 → Option)\n#[derive(Debug, Clone)]\npub struct Rule {\n  pub left: Option<SymbolId>,\n  pub right: Vec<SymbolId>,\n  pub action: Action,\n}\n\n`;
  d += `/// 原版 Symbol 1:1 (终结符 = 无规则)\n#[derive(Debug, Clone, Default)]\npub struct Symbol {\n  pub rules: Vec<RuleId>,\n}\n\n`;
  d += `impl Symbol {\n  #[must_use]\n  pub fn is_terminal(&self) -> bool { self.rules.is_empty() }\n\n`;
  d += `  /// 原版 rule_number\n  #[must_use]\n  pub fn rule_number(&self) -> usize { self.rules.len() }\n`;
  d += `}\n\n`;

  d += `/// 原版 Item 1:1 (Earley 核心项; prev/child 供 AST 重建)\n#[derive(Debug, Clone, Copy)]\npub struct Item {\n  pub rule: RuleId,\n  pub mark: usize,\n  pub start: usize,\n  pub pos: usize,\n  pub prev: Option<ItemId>,\n  pub child: Option<ItemId>,\n}\n\n`;

  d += `/// 符号/规则 arena (Symbol 稳定地址由下标承担)\n#[derive(Debug, Default)]\npub struct GrammarTables {\n  pub symbols: Vec<Symbol>,\n  pub rules: Vec<Rule>,\n}\n\n`;
  d += `impl GrammarTables {\n  /// 原版 Symbol::AddRule: 建 Rule 并回填 left\n  pub fn add_rule(&mut self, left: SymbolId, right: Vec<SymbolId>, action: Action) -> RuleId {\n    let r = Rule { left: Some(left), right, action };\n    self.rules.push(r);\n    let id = (self.rules.len() - 1) as RuleId;\n    self.symbols[left as usize].rules.push(id);\n    id\n  }\n\n`;
  d += `  /// 原版 NewSymbol\n  pub fn new_symbol(&mut self) -> SymbolId {\n    self.symbols.push(Symbol::default());\n    (self.symbols.len() - 1) as SymbolId\n  }\n\n`;
  d += `  #[must_use]\n  pub fn rule_right(&self, r: RuleId) -> &[SymbolId] { &self.rules[r as usize].right }\n}`;
  d += `\n\n`;

  d += `/// 原版 Lexer 1:1 (patterns/keywords 存 SymbolId, arena 在 GrammarTables)\n#[derive(Debug, Default)]\npub struct Lexer {\n  pub whitespace: Option<PatternFunction>,\n  pub patterns: Vec<(PatternFunction, SymbolId)>,\n  pub keywords: BTreeMap<String, SymbolId>,\n}\n\n`;
  d += `/// 原版 PatternFunction = bool(*)(InputPosition*) → (源串, 游标) 版\npub type PatternFunction = fn(&str, &mut InputPosition) -> bool;\n\n`;
  d += `impl Lexer {\n  /// 原版 SetWhitespace\n  pub fn set_whitespace(&mut self, ws: PatternFunction) {\n    self.whitespace = Some(ws);\n  }\n\n`;
  d += `  /// 原版 Pattern: 登记 pattern 终结符\n  pub fn pattern(&mut self, tables: &mut GrammarTables, f: PatternFunction) -> SymbolId {\n    let s = tables.new_symbol();\n    self.patterns.push((f, s));\n    s\n  }\n\n`;
  d += `  /// 原版 Token: 关键字终结符 (同字面量复用)\n  pub fn token(&mut self, tables: &mut GrammarTables, keyword: &str) -> SymbolId {\n    if let Some(&s) = self.keywords.get(keyword) {\n      return s;\n    }\n    let s = tables.new_symbol();\n    self.keywords.insert(keyword.to_owned(), s);\n    s\n  }\n\n`;
  d += `  /// 原版 RunLexer\n  pub fn run_lexer(&self, input: &str, tables: &GrammarTables, origin: SourcePosition) -> Result<LexerResult> {\n    // TODO(填体): 最长匹配原则 + 关键字优先\n    let _ = (input, tables, origin);\n    Err(Error::Unimplemented)\n  }\n`;
  d += `}\n\n`;

  d += `/// 原版 Grammar 1:1 (子类化模式 → TorqueGrammar 组合 Grammar, 见 torque_parser.rs)\n#[derive(Debug, Default)]\npub struct Grammar {\n  pub tables: GrammarTables,\n  pub lexer: Lexer,\n  pub generated_symbols: Vec<SymbolId>,\n  pub start: Option<SymbolId>,\n}\n\n`;
  d += `impl Grammar {\n  /// 原版 Token/Pattern/SetWhitespace 转发\n  pub fn token(&mut self, s: &str) -> SymbolId {\n    let tables = &mut self.tables;\n    self.lexer.token(tables, s)\n  }\n\n`;
  d += `  pub fn pattern(&mut self, f: PatternFunction) -> SymbolId {\n    let tables = &mut self.tables;\n    self.lexer.pattern(tables, f)\n  }\n\n`;
  d += `  pub fn set_whitespace(&mut self, ws: PatternFunction) {\n    self.lexer.set_whitespace(ws);\n  }\n\n`;
  d += `  /// 原版 NewSymbol: 造生成符号 (存 generated_symbols)\n  pub fn new_symbol(&mut self) -> SymbolId {\n    let s = self.tables.new_symbol();\n    self.generated_symbols.push(s);\n    s\n  }\n\n`;
  d += `  /// 原版 Sequence: 符号序列符号 (at most one 结果)\n  pub fn sequence(&mut self, symbols: Vec<SymbolId>) -> SymbolId {\n    let s = self.new_symbol();\n    self.tables.add_rule(s, symbols, yield_matched_input);\n    s\n  }\n\n`;
  d += `  /// 原版 NonemptyList (T 参数经动作函数指针编码 → 显式传 singleton/extend 动作)\n  pub fn nonempty_list(&mut self, element: SymbolId, separator: Option<SymbolId>, singleton: Action, extend: Action) -> SymbolId {\n    let list = self.new_symbol();\n    let r1 = Rule { left: Some(list), right: vec![element], action: singleton };\n    let r2 = Rule {\n      left: Some(list),\n      right: separator.map_or(vec![list, element], |sep| vec![list, sep, element]),\n      action: extend,\n    };\n    self.tables.rules.push(r1);\n    self.tables.symbols[list as usize].rules.push((self.tables.rules.len() - 1) as RuleId);\n    self.tables.rules.push(r2);\n    self.tables.symbols[list as usize].rules.push((self.tables.rules.len() - 1) as RuleId);\n    list\n  }\n\n`;
  d += `  /// 原版 CheckIf\n  pub fn check_if(&mut self, x: SymbolId) -> SymbolId {\n    let s = self.new_symbol();\n    self.tables.add_rule(s, vec![x], yield_true);\n    self.tables.add_rule(s, vec![], yield_false);\n    s\n  }\n`;
  d += `}\n\n`;
  d += `// 原版 Grammar 模板助手 (TryOrDefault/List/Optional/YieldIntegralConstant/\n// MakeSingletonVector/MakeExtendedVector/CastParseResult/YieldDefaultValue) 的\n// T 参数由 Action 函数指针具体化承载, 组合器随填体按需补齐。\n\n`;
  d += `/// 原版 YieldMatchedInput (取规则匹配源文本; 输入串经动作上下文传入, 填体)\npub fn yield_matched_input(_child_results: &mut ParseResultIterator) -> Option<ParseResultValue> {\n  // TODO(填体): matched_text 依赖源串线程化\n  None\n}\n\n`;
  d += `/// 原版 YieldIntegralConstant<bool, true/false>\npub fn yield_true(_child_results: &mut ParseResultIterator) -> Option<ParseResultValue> {\n  Some(ParseResultValue::Bool(true))\n}\n\n`;
  d += `pub fn yield_false(_child_results: &mut ParseResultIterator) -> Option<ParseResultValue> {\n  Some(ParseResultValue::Bool(false))\n}\n\n`;
  d += `/// 原版 RunEarleyAlgorithm\npub fn run_earley_algorithm(start: SymbolId, tokens: &LexerResult, processed: &mut BTreeSet<ItemKey>) -> Result<ItemId> {\n  // TODO(填体)\n  let _ = (start, tokens, processed);\n  Err(Error::Unimplemented)\n}\n\n`;
  d += `/// Item 等价键 (rule/mark/start/pos; prev/child 不参与, 对应原版 operator==/hash)\npub type ItemKey = (RuleId, usize, usize, usize);\n\n`;
  d += `/// 原版 ParseTokens\npub fn parse_tokens(start: SymbolId, tokens: &LexerResult) -> Result<ParseResultValue> {\n  // TODO(填体)\n  let _ = (start, tokens);\n  Err(Error::Unimplemented)\n}\n\n`;
  d += `/// Item::GetMatchedInput (项覆盖的源区间)\n#[must_use]\npub fn item_matched_input(item: &Item, tokens: &LexerResult) -> MatchedInput {\n  let start = &tokens.token_contents[item.start];\n  let end = if item.start == item.pos {\n    &tokens.token_contents[item.start]\n  } else {\n    &tokens.token_contents[item.pos - 1]\n  };\n  MatchedInput {\n    begin: start.begin,\n    end: end.end,\n    pos: SourcePosition { source: start.pos.source, start: start.pos.start, end: end.pos.end },\n  }\n}\n`;

  d += `\n` + sigBlock("src/torque/earley-parser.cc", "签名清单");
  writeSrc("v8-torque", "earley_parser.rs", d);
}

// ===========================================================================
// torque-parser.{h,cc} → torque_parser.rs
// ===========================================================================
function genTorqueParser() {
  const relH = "src/torque/torque-parser.h";
  const relCc = "src/torque/torque-parser.cc";
  const cc = readV8(relCc);

  let d = banner("src/torque/parser.js", relH, "Torque 文法: 解析入口 + token 表 + 产生式动作清单");
  d += `
use crate::ast::{Ast, ClassFieldExpression, NodeId};
use crate::earley_parser::{Grammar, LexerResult};
use crate::error::{Error, Result};
use crate::source_positions::SourcePosition;

// 原版 ParseTorque 经 CurrentAst 上下文添加声明 → Rust 显式传 &mut Ast。
// 原版 TorqueGrammar ~200 个 Symbol 成员 → Rust 组合 earley_parser::Grammar,
// 符号随文法构造动态登记 (grammar.tables), token 关键字全量见 GRAMMAR_TOKENS。
`;

  d += `/// 原版 using TypeList = std::vector<TypeExpression*>\npub type TypeList = Vec<NodeId>;\n\n`;
  d += `/// 原版 ExpressionWithSource 1:1\n#[derive(Debug, Clone)]\npub struct ExpressionWithSource {\n  pub expression: NodeId,\n  pub source: String,\n}\n\n`;
  d += `/// 原版 EnumEntry 1:1\n#[derive(Debug, Clone)]\npub struct EnumEntry {\n  pub name: NodeId,\n  pub type_: Option<NodeId>,\n  pub alias_entry: Option<String>,\n}\n\n`;

  // Token 表 (规则密集生成物, 保留生成态)
  const tokens = [];
  for (const m of cc.matchAll(/Token\("((?:[^"\\]|\\.)*)"\)/g)) {
    if (!tokens.includes(m[1])) tokens.push(m[1]);
  }
  d += `/// 文法关键字/操作符 token 全量 (${tokens.length} 个, 抽自 torque-parser.cc Token("...") 首现序)\n`;
  d += `pub const GRAMMAR_TOKENS: [&str; ${tokens.length}] = [\n`;
  for (const t of tokens) d += `  "${t}",\n`;
  d += `];\n\n`;

  d += `/// 原版 TorqueGrammar (struct : Grammar → 组合)\n#[derive(Debug, Default)]\npub struct TorqueGrammar {\n  pub grammar: Grammar,\n}\n\n`;
  d += `impl TorqueGrammar {\n  /// 原版 TorqueGrammar(): 全文法构造 (Symbol 成员 + 规则 + 动作绑定)\n  #[must_use]\n  pub fn new() -> Self {\n    // TODO(填体): 文法构造 — 本编译器最大填体块之一 (见产生式动作清单)\n    Self::default()\n  }\n\n`;
  d += `  /// 原版 ParseTorqueGrammar: lexer + earley 主流程\n  pub fn parse(&mut self, input: &str, origin: SourcePosition) -> Result<LexerResult> {\n    // TODO(填体)\n    let _ = (input, origin);\n    Err(Error::Unimplemented)\n  }\n`;
  d += `}\n\n`;

  d += `/// 原版 ParseTorque: 源码 → 追加到 Ast\npub fn parse_torque(input: &str, ast: &mut Ast, origin: SourcePosition) -> Result<()> {\n  // TODO(填体)\n  let _ = (input, ast, origin);\n  Err(Error::Unimplemented)\n}\n\n`;
  d += `/// 原版 ParseTorqueTypeExpression (V8_TQ_* 注解参数用)\npub fn parse_torque_type_expression(input: &str, ast: &mut Ast) -> Result<NodeId> {\n  // TODO(填体)\n  let _ = (input, ast);\n  Err(Error::Unimplemented)\n}\n\n`;
  d += `/// 原版 ParseTorqueExpression\npub fn parse_torque_expression(input: &str, ast: &mut Ast) -> Result<NodeId> {\n  // TODO(填体)\n  let _ = (input, ast);\n  Err(Error::Unimplemented)\n}\n\n`;
  d += `/// 原版 ParseTorqueClassFields (V8_TQ_TAIL_SECTIONS 类字段序列)\npub fn parse_torque_class_fields(input: &str, ast: &mut Ast) -> Result<Vec<ClassFieldExpression>> {\n  // TODO(填体)\n  let _ = (input, ast);\n  Err(Error::Unimplemented)\n}\n\n`;
  d += `/// 原版 MakeCall\n#[must_use]\npub fn make_call(callee: NodeId, generic_arguments: &[NodeId], arguments: &[NodeId], otherwise: &[NodeId], ast: &mut Ast) -> NodeId {\n  // TODO(填体): 纯 AST 构造\n  let _ = (callee, generic_arguments, arguments, otherwise, ast);\n  0\n}\n`;

  // 产生式动作函数全量清单 (规则密集生成物; 直接对原文抽取, 串内容保真)
  const sigs = topSigs(cc);
  const ccLineCount = cc.split("\n").length;
  if (sigs.length < ccLineCount / 200) {
    console.error(`[torque] torque-parser.cc ${ccLineCount} 行仅抽到 ${sigs.length} 条签名 (< 行数/200) — noCmt/topSigs 异常, 防静默检查`);
  }
  d += `\n// ── 原版 torque-parser.cc: 文法产生式动作函数全量签名清单 (${sigs.length} 个, 填体正交任务源) ──\n`;
  for (const x of sigs) d += `// L${x.line}: ${x.sig}\n`;
  d += `\n` + sigBlock(relH, "解析入口签名清单");
  writeSrc("v8-torque", "torque_parser.rs", d);
}

// ===========================================================================
// 主流程
// ===========================================================================
genEarley();
genTorqueParser();
console.log("== v8-torque 解析组直出完毕: earley_parser.rs + torque_parser.rs ==");
