#!/usr/bin/env bun
// tool/src/logging/code-events.h.js
// 机械提取 v8/src/logging/code-events.h: LOG_EVENT_LIST / CODE_TYPE_LIST X-macro 展开

import { readV8, writeSrc, banner } from "../../lib.js";

const hSrc = readV8("src/logging/code-events.h");

// 1. LOG_EVENT_LIST(V): V(kName, "log-string")
function parseVList(macroName) {
  const m = hSrc.match(new RegExp(`#define ${macroName}\\(V\\)([\\s\\S]*?)\\n\\n`));
  if (!m) throw new Error(`未找到 ${macroName}`);
  const items = [...m[1].matchAll(/V\((\w+),\s*(?:"([^"]*)"|(\w+))\)/g)].map((x) => ({
    name: x[1],
    str: x[2] !== undefined ? x[2] : null,
    type: x[3] !== undefined ? x[3] : null,
  }));
  if (!items.length) throw new Error(`${macroName} 空表`);
  return items;
}

const events = parseVList("LOG_EVENT_LIST");
const codeTags = parseVList("CODE_TYPE_LIST");
if (events.length !== 9) throw new Error(`LOG_EVENT_LIST 应 9 项, 实得 ${events.length}`);
if (codeTags.length !== 11) throw new Error(`CODE_TYPE_LIST 应 11 项, 实得 ${codeTags.length}`);
if (events[0].name !== "kCodeCreation" || events[events.length - 1].name !== "kTick") {
  throw new Error("LOG_EVENT_LIST 序漂移");
}
if (codeTags[0].name !== "kBuiltin" || codeTags[codeTags.length - 1].name !== "kNativeScript") {
  throw new Error("CODE_TYPE_LIST 序漂移");
}
// 事件日志串完整性 (对数: 9 串全非空)
if (events.some((e) => !e.str)) throw new Error("LOG_EVENT_LIST 有空日志串");

function screaming(kName) {
  return kName.replace(/^k/, "").replace(/([a-z0-9])([A-Z])/g, "$1_$2").toUpperCase();
}

let rust = banner("src/logging/code-events.h.js", "src/logging/code-events.h", "代码事件 X-macro 展开");

rust += `
/// 代码事件枚举 (原版 LogEventListener::Event, LOG_EVENT_LIST 展开 + kLength)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum Event {
${events
  .map(
    (e, i) =>
      `  /// "${e.str}"\n  ${screaming(e.name)} = ${i},`,
  )
  .join("\n")}
  /// 哨兵 (原版 kLength)
  Length = ${events.length},
}

impl Event {
  /// 总数含哨兵 (原版 kLength)
  pub const COUNT: usize = ${events.length + 1};

  /// 日志串 (原版 V(kName, "log-string") 第二列)
  #[must_use]
  pub const fn log_str(self) -> Option<&'static str> {
    match self {
${events.map((e) => `      Self::${screaming(e.name)} => Some("${e.str}"),`).join("\n")}
      _ => None,
    }
  }
}

/// 代码类型标签 (原版 LogEventListener::CodeTag, CODE_TYPE_LIST 展开)
/// 注: Native* 两项写入日志时映射回原标签 (原版注释)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum CodeTag {
${codeTags.map((t, i) => `  /// ${t.type}\n  ${screaming(t.name)} = ${i},`).join("\n")}
  /// 哨兵 (原版 kLength)
  Length = ${codeTags.length},
}

impl CodeTag {
  /// 总数含哨兵
  pub const COUNT: usize = ${codeTags.length + 1};

  /// 写日志用的类型名 (原版第二列; kFunction→JS, kNativeFunction→JS 等)
  #[must_use]
  pub const fn type_name(self) -> Option<&'static str> {
    match self {
${codeTags.map((t) => `      Self::${screaming(t.name)} => Some("${t.type}"),`).join("\n")}
      _ => None,
    }
  }
}
`;

const p = writeSrc("v8-logging", "code_events_generated.rs", rust);
console.log(`written ${p}; Event=${events.length}, CodeTag=${codeTags.length}`);
