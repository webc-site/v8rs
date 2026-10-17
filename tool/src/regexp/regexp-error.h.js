#!/usr/bin/env bun
// tool/src/regexp/regexp-error.h.js — v8/src/regexp/regexp-error.h → v8-regexp-syntax/src/regexp_error_generated.rs
// 机械提取 REGEXP_ERROR_MESSAGES X-macro (Name, "message"), 展开枚举 + 消息表 + 对数断言
import { banner, readV8, writeSrc } from "../../lib.js";

const src = readV8("src/regexp/regexp-error.h");
const m = src.match(/#define REGEXP_ERROR_MESSAGES\(T\)([\s\S]*?)\n\nenum class Error/);
if (!m) throw new Error("regexp-error.h: REGEXP_ERROR_MESSAGES 结构漂移");

// T(Name, "message") — message 可跨行 (续行字符串拼接)
const rows = [];
const macroBody = m[1].replace(/\\\n\s*/g, " ");
for (const r of macroBody.matchAll(/T\((\w+),\s*"((?:[^"\\]|\\.)*)"\)/g)) {
  rows.push({ name: r[1], msg: r[2] });
}
if (rows.length === 0) throw new Error("REGEXP_ERROR_MESSAGES 零条目");
if (rows[0].name !== "None") throw new Error("首项应为 None");
const nonNoneRows = rows.slice(1);
console.log(`regexp-error: ${nonNoneRows.length} 项 (排除 None) ✓`);

const variants = nonNoneRows.map((r) => `  ${r.name},`).join("\n");
const arms = nonNoneRows.map((r) => `      RegexpError::${r.name} => "${r.msg}",`).join("\n");

const body = `${banner("src/regexp/regexp-error.h.js", "src/regexp/regexp-error.h", "REGEXP_ERROR_MESSAGES X-macro展开; None → 成功哨兵不进枚举")}
/// 正则解析/编译错误类别 (原版 enum class Error, kNone 以 Option/成功路径表达)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RegexpError {
${variants}
}

pub const NUM_REGEXP_ERRORS: usize = ${nonNoneRows.length};

// ── 对数断言: 枚举判别数 == 宏实数 ──
const _: () = assert!(RegexpError::VARIANTS.len() == NUM_REGEXP_ERRORS);

impl RegexpError {
  /// 全部变体 (None 哨兵除外, 序与原版一致)
  pub const VARIANTS: &'static [RegexpError] = &[
${nonNoneRows.map((r) => `    RegexpError::${r.name},`).join("\n")}
  ];

  /// 错误消息 (原版 ErrorString)
  #[must_use]
  pub const fn message(self) -> &'static str {
    match self {
${arms}
    }
  }

  /// 是否栈溢出类 (原版 ErrorIsStackOverflow: kStackOverflow / kAnalysisStackOverflow / kTooLarge)
  #[must_use]
  pub const fn is_stack_overflow(self) -> bool {
    matches!(self, RegexpError::StackOverflow | RegexpError::AnalysisStackOverflow | RegexpError::TooLarge)
  }
}
`;

writeSrc("v8-regexp-syntax", "regexp_error_generated.rs", body);
console.log("written: regexp_error_generated.rs");
