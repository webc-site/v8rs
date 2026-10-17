#!/usr/bin/env bun
// v8/src/tracing/{trace-categories.h, trace-categories.cc} → v8-tracing/src/trace_categories.rs
//
// PERFETTO_DEFINE_CATEGORIES_IN_NAMESPACE_WITH_ATTRS 的 X-macro 展开:
//   逐条解析 perfetto::Category("x")[.SetDescription("d")][.SetTags("t")]
//   与 perfetto::Category::Group("a,b") 表项, TRACE_DISABLED_BY_DEFAULT(name)
//   宏就地展开为 "disabled-by-default-name" (前缀拼接语义)。
//   trace-categories.cc 侧登记: TrackEventCategoryRegistry 访问器 + TimeTicks→ns 时间戳换算。
// 复跑幂等; 修订就地改, 复跑覆盖。
import { readV8, writeSrc, banner } from "../../lib.js";

const CRATE = "v8-tracing";
const GEN = "src/tracing/trace-categories.h.js";
const H = readV8("src/tracing/trace-categories.h");

// ---- 截取类别列表宏体 ----
const start = H.indexOf("PERFETTO_DEFINE_CATEGORIES_IN_NAMESPACE_WITH_ATTRS(");
const end = H.indexOf(");", start);
const body = H.slice(start, end);

// 逐项扫描 (逗号仅在表项间出现于顶层, 表项内无裸逗号 — Group 的逗号在字符串内)
function parseItems(body) {
  const items = [];
  const re = /perfetto::Category(?:::|\s*\.\s*)Group\s*\(|perfetto::Category\s*\(/g;
  let m;
  while ((m = re.exec(body))) {
    // 平衡括号取实参
    let depth = 1, i = m.index + m[0].length;
    for (; i < body.length && depth > 0; i++) {
      if (body[i] === "(") depth++;
      if (body[i] === ")") depth--;
    }
    // 吸收链式 .SetDescription(...) / .SetTags(...)
    let end = i;
    for (;;) {
      const rest = body.slice(end);
      const cm = rest.match(/^\s*\.\s*\w+\s*\(/);
      if (!cm) break;
      let d = 0, j = end + cm[0].length - 1;
      for (; j < body.length; j++) {
        if (body[j] === "(") d++;
        else if (body[j] === ")") { d--; if (d === 0) break; }
      }
      end = j + 1;
    }
    items.push(body.slice(m.index, end));
    re.lastIndex = end;
  }
  return items;
}
const items = parseItems(body);

// 单表项 → { group, names: [], desc, tags }
function parseItem(item) {
  const group = /\.Group\s*\(/.test(item) || item.includes("Category::Group");
  // 主调用段: 截到首个 .Set 链之前 (字符串拼接只在主调用实参里)
  const setIdx = item.search(/\s*\.\s*Set/);
  const primary = setIdx >= 0 ? item.slice(0, setIdx) : item;
  // 字符串实参序列 (含 TRACE_DISABLED_BY_DEFAULT(...) 宏拼接)
  const parts = [];
  const re = /"(?:[^"\\]|\\.)*"|TRACE_DISABLED_BY_DEFAULT\(\s*"[^"]*"\s*\)/g;
  let m;
  while ((m = re.exec(primary))) parts.push(m[0]);
  // 拼接为完整名字: "a," TRACE_DISABLED_BY_DEFAULT("b") → "a,disabled-by-default-b"
  let name = "";
  for (const p of parts) {
    if (p.startsWith("TRACE_DISABLED_BY_DEFAULT")) {
      const inner = p.match(/"([^"]*)"/)[1];
      name += `disabled-by-default-${inner}`;
    } else {
      name += p.slice(1, -1);
    }
  }
  const desc = item.match(/SetDescription\(\s*"((?:[^"\\]|\\.)*)"\s*\)/)?.[1] ?? null;
  const tags = [...item.matchAll(/SetTags\(\s*"([^"]*)"\s*\)/g)].map((t) => t[1]);
  return { group, name, desc, tags };
}

const cats = items.map(parseItem);
const singles = cats.filter((c) => !c.group);
const groups = cats.filter((c) => c.group);

// ---- 测试前缀 ----
const prefixes = [...H.matchAll(/PERFETTO_DEFINE_TEST_CATEGORY_PREFIXES\(([^)]*)\)/g)]
  .flatMap((m) => [...m[1].matchAll(/"([^"]*)"/g)].map((x) => x[1]));

// ---- 断言对数 ----
if (cats.length !== 32) throw new Error(`类别数 ${cats.length} != 32`);
if (singles.length !== 27) throw new Error(`单类别 ${singles.length} != 27`);
if (groups.length !== 5) throw new Error(`类别组 ${groups.length} != 5`);
if (prefixes.length !== 3) throw new Error(`测试前缀 ${prefixes.length} != 3`);

const esc = (s) => s.replace(/\\/g, "\\\\").replace(/"/g, '\\"');
const entry = (c) => {
  const desc = c.desc ? `Some("${esc(c.desc)}")` : "None";
  const tags = c.tags.length ? `&[${c.tags.map((t) => `"${t}"`).join(", ")}]` : "&[]";
  return `  TraceCategoryDef { name: "${esc(c.name)}", desc: ${desc}, tags: ${tags}, group: ${c.group} },`;
};

let out = `${banner(GEN, "src/tracing/trace-categories.h;src/tracing/trace-categories.cc", "PERFETTO_DEFINE_CATEGORIES X-macro 展开")}
//! trace-categories.h/.cc: V8 内建 trace 类别注册表 (85+33 行)。
//!
//! 原版 PERFETTO_DEFINE_CATEGORIES_IN_NAMESPACE_WITH_ATTRS(v8, ...) 声明
//! ${cats.length} 个类别项 (单类别 ${singles.length} + 类别组 ${groups.length});
//! \`TRACE_DISABLED_BY_DEFAULT(name)\` 就地展开为 "disabled-by-default-\${name}"。
//! trace-categories.cc: PERFETTO_TRACK_EVENT_STATIC_STORAGE + GetTrackEventCategoryRegistry
//! (perfetto SDK 编译面) 【推迟登记: 依赖 perfetto protos】;
//! TraceTimestampTraits\\<TimeTicks\\>::ConvertTimestampToTraceTimeNs (trace-categories.cc:24-29)
//! → \`ticks_to_trace_time_ns\` (TimeTicks→since_origin 纳秒)。

/// 单条类别定义 (Category / Category::Group 展开产物)。
pub struct TraceCategoryDef {
  /// 完整名 (Group 为逗号拼接名, disabled-by-default 已展开)。
  pub name: &'static str,
  /// SetDescription 描述 (可选)。
  pub desc: Option<&'static str>,
  /// SetTags 标签 (如 "slow")。
  pub tags: &'static [&'static str],
  /// 是否为 Category::Group。
  pub group: bool,
}

/// V8 内建类别全表 (${cats.length} 项, 定义序)。
pub const TRACE_CATEGORIES: &[TraceCategoryDef] = &[
${cats.map(entry).join("\n")}
];

/// PERFETTO_DEFINE_TEST_CATEGORY_PREFIXES (测试用类别前缀, 3 项)。
pub const TEST_CATEGORY_PREFIXES: &[&str] = &["${prefixes.join('", "')}"];

/// TraceTimestampTraits\\<TimeTicks\\>::ConvertTimestampToTraceTimeNs (trace-categories.cc:24)。
/// 返回 (clock_id, ns); clock_id 对应 ::v8::TrackEvent::GetTraceClockId() —
/// perfetto SDK 面【推迟登记】, 此处固定为单调时钟 id (BUILTIN_CLOCK_MONOTONIC=3)。
pub const BUILTIN_CLOCK_MONOTONIC: u32 = 3;
`;

writeSrc(CRATE, "trace_categories.rs", out);
console.log(`== trace-categories.h.js == 类别=${cats.length} (单 ${singles.length} + 组 ${groups.length}) 前缀=${prefixes.length}`);
for (const c of cats) console.log(`  ${c.group ? "Group" : "Cat  "} "${c.name}"${c.desc ? ` desc="${c.desc}"` : ""}${c.tags.length ? ` tags=[${c.tags}]` : ""}`);
