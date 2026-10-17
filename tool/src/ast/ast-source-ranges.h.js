#!/usr/bin/env bun
// tool/src/ast/ast-source-ranges.h.js
// 机械提取 v8/src/ast/ 访问器族: ast-source-ranges.h + source-range-ast-visitor.{h,cc} +
// prettyprinter.{h,cc} + ast-traversal-visitor.h + ast-function-literal-id-reindexer.{h,cc}

import { readV8, writeSrc, banner } from "../../lib.js";

const srH = readV8("src/ast/ast-source-ranges.h");
const svH = readV8("src/ast/source-range-ast-visitor.h");
const svCc = readV8("src/ast/source-range-ast-visitor.cc");
const ppH = readV8("src/ast/prettyprinter.h");
const ppCc = readV8("src/ast/prettyprinter.cc");
const tvH = readV8("src/ast/ast-traversal-visitor.h");
const riH = readV8("src/ast/ast-function-literal-id-reindexer.h");
const riCc = readV8("src/ast/ast-function-literal-id-reindexer.cc");

// 1. SourceRangeKind 枚举 (机械提取)
const kindM = srH.match(/enum class SourceRangeKind \{([^}]+)\};/);
if (!kindM) throw new Error("未找到 SourceRangeKind");
const kinds = kindM[1].split(",").map((s) => s.trim()).filter(Boolean);
if (kinds.join() !== "kBody,kCatch,kContinuation,kElse,kFinally,kRight,kThen") {
  throw new Error(`SourceRangeKind 漂移: ${kinds.join()}`);
}

// 2. 范围表类族 (ast-source-ranges.h class 集)
const rangeClasses = [...srH.matchAll(/\nclass (\w+) final : public AstNodeSourceRanges/g)].map((m) => m[1]);
if (rangeClasses.length < 4) throw new Error(`范围表类应 ≥4, 实得 ${rangeClasses.length}: ${rangeClasses.join(",")}`);

// 3. 打印器双类
for (const k of ["class CallPrinter", "class AstPrinter"]) {
  if (!ppH.includes(k)) throw new Error(`prettyprinter.h 缺 ${k}`);
}
const spreadM = ppH.match(/enum class SpreadErrorInArgsHint \{ (\w+), (\w+) \};/);
if (!spreadM || spreadM[1] !== "kErrorInArgs") throw new Error("SpreadErrorInArgsHint 漂移");

// 4. 遍历器/重排器锚点
for (const k of ["AstTraversalVisitor"]) {
  if (!tvH.includes(k)) throw new Error(`ast-traversal-visitor.h 缺 ${k}`);
}
if (!riH.includes("AstFunctionLiteralIdReindexer") || !riCc.includes("AstFunctionLiteralIdReindexer::")) {
  throw new Error("重排器面漂移");
}
// cc 存在性
if (!svCc.includes("SourceRangeAstVisitor::")) throw new Error("source-range-ast-visitor.cc 缺实现");
if (!ppCc.includes("CallPrinter::") || !ppCc.includes("AstPrinter::")) throw new Error("prettyprinter.cc 缺实现");

function camel(kName) {
  return kName.replace(/^k/, "").toLowerCase().replace(/(^|_)(\w)/g, (_, __, c) => c.toUpperCase());
}

let rust = banner(
  "src/ast/ast-source-ranges.h.js",
  "src/ast/ast-source-ranges.h + source-range-ast-visitor.{h,cc} + prettyprinter.{h,cc} + ast-traversal-visitor.h + ast-function-literal-id-reindexer.{h,cc}",
  "AST 范围/打印/遍历/重排骨架",
);

rust += `
/// 源码范围类别 (原版 enum class SourceRangeKind, ${kinds.length} 项)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum SourceRangeKind {
${kinds.map((k, i) => `  /// \`${k}\`\n  ${camel(k)} = ${i},`).join("\n")}
}

impl SourceRangeKind {
  pub const COUNT: usize = ${kinds.length};
}

/// 节点范围表类族 (原版 ast-source-ranges.h, ${rangeClasses.length} 类, 各挂若干 SourceRangeKind)
pub const RANGE_TABLE_CLASSES: [&str; ${rangeClasses.length}] = [${rangeClasses.map((c) => `"${c}"`).join(", ")}];

/// 打印器骨架 (语义见手写 visitor.rs)
pub mod ast_printer_skel {
  /// 调用打印器: 找出可能抛错的调用点 (原版 CallPrinter; spread 误用提示双态)
  pub const SPREAD_HINT_STATES: [&str; 2] = ["kErrorInArgs", "kNoErrorInArgs"];
  /// AST 打印器: 缩进式反解析 (原版 AstPrinter; IndentedScope 栈式缩进)
  pub const PRINTER_CLASSES: [&str; 2] = ["CallPrinter", "AstPrinter"];

  /// 打印入口骨架
  pub fn print(_program: usize) -> String {
    unimplemented!("骨架")
  }
}

/// 遍历/重排骨架
pub mod ast_visitor_skel {
  /// 通用遍历 (原版 AstTraversalVisitor: 模板 DFS, 默认不递归子表达式可短路)
  pub fn traverse(_node: usize) {
    unimplemented!("骨架")
  }
  /// 函数字面量 id 重排 (原版 AstFunctionLiteralIdReindexer:
  /// 快照恢复时把 id 段平移到新起点)
  pub fn reindex_function_literal_ids(_base: i32, _old_id: i32) {
    unimplemented!("骨架")
  }
  /// 源码范围访问器 (原版 SourceRangeAstVisitor: try/catch/finally 与
  /// 逻辑操作续接范围收集)
  pub fn collect_source_ranges() {
    unimplemented!("骨架")
  }
}
`;

const p = writeSrc("v8-ast", "visitors_generated.rs", rust);
console.log(`written ${p}; Kind=${kinds.length}, 范围类=${rangeClasses.length}`);
