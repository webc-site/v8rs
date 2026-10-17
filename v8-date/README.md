[English](#en) | [中文](#zh)

---

<a name="en"></a>

# v8-date (English)

Rust transcription of `v8/src/date` (~1.9k lines).
JS Date semantics bound 1:1: date string parsing (ISO 8601 + lenient legacy format), timezone rule caching (DateCache, 32-segment LRU + DST bisection), UTC/local conversion, boundary conditions (year 0/275760, millisecond precision, NaN propagation) — line-by-line correspondence with upstream.

- [File → module mapping](#file-module-mapping)
- [Conversion scripts (tool/src/date/)](#conversion-scripts-toolsrcdate)
- [Test mapping (unittests/date 1:1)](#test-mapping-unittestsdate-11)

## File → module mapping

| v8 source (lines) | Rust module | Notes |
|---|---|---|
| date.h (275) | src/date_cache/constants.rs (52) + src/algorithms.rs (207) | Constants extracted by script; DaysFromTime (div_euclid ≡ floor correction) / TimeInDay / TryTimeClip (trunc+0.0 ≡ floor/ceil) / Weekday (rem_euclid ≡ negative+7) / EquivalentYear / EquivalentTime 1:1 |
| date.cc (654) | src/date_cache/cache.rs (484) + src/algorithms.rs + src/dateparser/parser.rs::parse_date_time_string | DateCache state machine 1:1: ProbeCache / ExtendTheAfterSegment / LeastRecentlyUsedCacheItem / LocalOffsetInMs (optimistic hit → probe → miss → hit → large jump → DST bisection, 5 rounds); YearMonthDayFromDays with ymd fast path (1..=28); MakeDay/MakeTime/MakeDate (trunc+0.0 removes -0); ToDateString five-mode formatting (negative year %05d / extended year ±%06d); ParseDateTimeString (UTC_OFFSET NaN → ToUTC, out of range → NaN); trailing UNREACHABLE() rewritten to return per no-panic rule; ICU branch not transplanted — timezone source injected via TimezoneProvider trait (v8-base LocalTimezoneCache backs it) |
| dateparser.h (368) | src/dateparser/{tables,tokenizer,composer}.rs (192+364+340) | InputReader/DateStringTokenizer/DateToken (tag enum) 1:1; Keyword(INVALID) token equivalent to C++ tag>=0 falling into IsKeyword branch; three Composers (Day/Time/TimeZone) 1:1, TimeZoneComposer::Write reproduces C++ unsigned 32-bit wrapping arithmetic (dateparser.cc:106 — accept iff wrapped value ≤ SmiMax); Smi validation uses 31 bits ((1<<30)-1), matching upstream pointer-compression default; whitespace classification reuses v8-strings char_predicates: word boundaries use is_white_space (no line terminators, per IsWhiteSpaceChar), whitespace skipping uses is_white_space_or_line_terminator (dateparser-inl.h:218) — LS/PS are word characters |
| dateparser.cc (201) | src/dateparser/{tables,parser,composer}.rs | KeywordTable 28 entries extracted by script (months 12 / AM-PM 2 / timezones 12 / T separator 1 / terminator 1); Lookup with non-ASCII (>=128) prefix short-circuit (explicit form of C++ uint32 direct-compare inequality); ReadMilliseconds scaling (1 digit → ×100, 2 → ×10, >3 → ÷10^(min(len,9)-3)) 1:1 |
| dateparser-inl.h (361) | src/dateparser/{parser,tokenizer}.rs | ParseES5DateTime state machine line-by-line (±6-digit extended year / -000000 rejected / 24:00 boundary / hhmm extended timezone / mandatory trailing EOF); legacy main loop (n+:: / n+. / month names / timezone names / (+\|-)hh:mm\|hh(:mm)\|hhmm / garbage-word rules) 1:1; Scan/SkipParentheses/ReadWord (half-filled 3-byte prefix + zero padding) 1:1 |
| (Isolate::CountUsage instrumentation) | parse_with_legacy_flag returns (bool, legacy) | kLegacyDateParser usage counting moved to the caller |

Not transplanted (registered rationale):
- ICU path (Intl::CreateTimezoneCache) — upstream dual branch; timezone data source decoupled via TimezoneProvider trait; default SystemTimezoneProvider wraps v8-base LocalTimezoneCache
- Isolate/GC/string object shells — belong to the objects layer; this crate takes &[u32]/&str input, zero object coupling

## Conversion scripts (tool/src/date/)

- date.h.js → date_cache/constants.rs (date.h+date.cc constants and tables; generator kept in sync with in-place revisions, rerun idempotent)
- dateparser.h.js → dateparser/tables.rs (dateparser.h+dateparser.cc keyword table; same)
- dateparser.cc.js → read-only validator: KEYWORD_TABLE 28 entries item-by-item + group counts (months 12 / AM-PM 2 / timezones 12 / T 1 / terminator 1) + POW10 7-entry log assertions
- date.cc.js → read-only validator: weekday 7 / months 12 / month lengths 12 (Σ365) / cumulative days-in-year 12×2 (last 334/335) + 6 key-constant log assertions

Generators are rerun-idempotent; rerunning overwrites in-place revisions — rerun only to regenerate a baseline. Validators are read-only.

## Test mapping (unittests/date 1:1)

| Upstream test | Rust test | Notes |
|---|---|---|
| date-cache-unittest.cc: DateCache.AdoptDefaultFirst | tests/date_cache_unittest.rs::test_adopt_default_first | mock timezone provider, rule-table driven, test name preserved |
| date-cache-unittest.cc: DateCache.AdoptDefaultMixed | tests/date_cache_unittest.rs::test_adopt_default_mixed | same |
| date-unittest.cc: DateTest.DaylightSavingsTime | tests/date_unittest.rs::test_daylight_savings_time | DST transition boundary, test name preserved |
| date-unittest.cc: DateTest.DateParseLegacyUseCounter | tests/date_unittest.rs::test_date_parse_legacy_use_counter | legacy fallback counting, test name preserved |
| (spec supplement) | tests/date_parser_unittest.rs (6) | ISO/legacy golden input→output pairs (per format branch), algorithms, five-mode formatting, end-to-end |
| (review supplement round1/round2) | tests/review_round{1,2}_unittest.rs (10) | TimeClip bounds / LRU extreme inputs / parser overflow / millisecond edge values / equivalent-year spec properties / bijective round-trip / counter overflow reset / adversarial malformed inputs / Unicode whitespace / AM-PM rules |

(Gate evidence ledger: see log/v8-date.md.)


---

<a name="zh"></a>

# v8-date (中文)

`v8/src/date` (~1.9k 行) 的 Rust 转写。
JS Date 语义绑定 1:1: 日期字符串解析 (ISO 8601 + 宽松格式), 时区规则缓存 (DateCache, 32 段 LRU + DST 二分), UTC/本地换算, 边界 (year 0/275760, 毫秒精度, NaN 传播) 与原版逐行对照。

- [文件 → 模块映射](#文件-模块映射)
- [转换脚本 (tool/src/date/)](#转换脚本-toolsrcdate)
- [测试映射 (unittests/date 1:1)](#测试映射-unittestsdate-11)

## 文件 → 模块映射

| v8 源文件 (行数) | Rust 模块 | 方式 |
|---|---|---|
| date.h (275) | src/date_cache/constants.rs (52) + src/algorithms.rs (207) | 常量脚本提取; DaysFromTime (div_euclid≡floor 修正)/TimeInDay/TryTimeClip (trunc+0.0≡floor/ceil)/Weekday (rem_euclid≡负数+7)/EquivalentYear/EquivalentTime 1:1 |
| date.cc (654) | src/date_cache/cache.rs (484) + src/algorithms.rs + src/dateparser/parser.rs::parse_date_time_string | DateCache 状态机 1:1: ProbeCache/ExtendTheAfterSegment/LeastRecentlyUsedCacheItem/LocalOffsetInMs (乐观命中→探测→缺失→命中→大跳→DST 二分 5 轮); YearMonthDayFromDays 含 ymd 缓存快速路径 (1..=28); MakeDay/MakeTime/MakeDate (trunc+0.0 消 -0); ToDateString 五模式格式化 (负年 %05d/扩展年 ±%06d); ParseDateTimeString (UTC_OFFSET NaN→ToUTC, 越界→NaN); UNREACHABLE() 尾按禁 panic 改返回; ICU 分支不迁移, 时区源经 TimezoneProvider trait 注入 (v8-base LocalTimezoneCache 承接) |
| dateparser.h (368) | src/dateparser/{tables,tokenizer,composer}.rs (192+364+340) | InputReader/DateStringTokenizer/DateToken (tag enum 化) 1:1; Keyword(INVALID) token 与 C++ tag>=0 进 IsKeyword 分支行为等价; 三 Composer (Day/Time/TimeZone) 1:1, TimeZoneComposer::Write 按 C++ unsigned 32 位回绕算术复现 (dateparser.cc:106, 回绕后 ≤ SmiMax 则接受); Smi 校验取 31 位 (1<<30)-1, 对应上游指针压缩默认配置; 空白判定复用 v8-strings char_predicates: word 边界用 is_white_space (不含行终止符, 对应 IsWhiteSpaceChar), 跳空白用 is_white_space_or_line_terminator (dateparser-inl.h:218), LS/PS 属 word 字符 |
| dateparser.cc (201) | src/dateparser/{tables,parser,composer}.rs | KeywordTable 28 条脚本提取 (月份 12/AM-PM 2/时区 12/T 分隔 1/终结 1); Lookup 含非 ASCII (>=128) 前缀短路 (C++ uint32 直接比较天然不等价的显式化); ReadMilliseconds 缩放 (1 位→×100, 2 位→×10, >3 位→÷10^(min(len,9)-3)) 1:1 |
| dateparser-inl.h (361) | src/dateparser/{parser,tokenizer}.rs | ParseES5DateTime 状态机逐行对照 (±6 位扩展年/-000000 拒绝/24:00 边界/hhmm 扩展时区/末尾强制 EOF); legacy 主循环 (n+::/n+./月名/时区名/(+|-)hh:mm|hh(:mm)|hhmm/garbage 词规则) 1:1; Scan/SkipParentheses/ReadWord (半填 3 字节前缀+0 填充) 1:1 |
| (Isolate::CountUsage 用量打点) | parse_with_legacy_flag 返回 (bool, legacy) | kLegacyDateParser 用量统计上移调用方 |

不迁移 (登记理由):
- ICU 路径 (Intl::CreateTimezoneCache) — 上游双分支, 时区数据源经 TimezoneProvider trait 解耦, 默认 SystemTimezoneProvider 包装 v8-base LocalTimezoneCache
- Isolate/GC/字符串对象外壳 — 归对象层, 本 crate 输入 &[u32]/&str, 零对象耦合

## 转换脚本 (tool/src/date/)

- date.h.js → date_cache/constants.rs (date.h+date.cc 常量与表生成; 生成器与就地修订同步, 复跑幂等)
- dateparser.h.js → dateparser/tables.rs (dateparser.h+dateparser.cc 关键字表生成; 同上)
- dateparser.cc.js → 只读校验器: KEYWORD_TABLE 28 条逐条 + 分组计数 (月份 12/AM-PM 2/时区 12/T 1/终结 1) + POW10 7 项对数断言
- date.cc.js → 只读校验器: 星期 7/月份 12/月天数 12 (Σ365)/年内累计天数 12×2 (末项 334/335) + 关键常量 6 项对数断言

生成器复跑幂等; 复跑会覆盖就地修订, 仅限重生成基线。校验器只读不写。

## 测试映射 (unittests/date 1:1)

| 原测试 | Rust 测试 | 说明 |
|---|---|---|
| date-cache-unittest.cc: DateCache.AdoptDefaultFirst | tests/date_cache_unittest.rs::test_adopt_default_first | mock 时区 provider, 规则表驱动, 测试名保留 |
| date-cache-unittest.cc: DateCache.AdoptDefaultMixed | tests/date_cache_unittest.rs::test_adopt_default_mixed | 同上 |
| date-unittest.cc: DateTest.DaylightSavingsTime | tests/date_unittest.rs::test_daylight_savings_time | DST 转换边界, 测试名保留; 登记偏差: 原版采样时刻写 `2 * 3600` (date-unittest.cc:122/128, 秒值误作毫秒), Rust 侧修正为 `2 * 3600 * 1000` (2 小时, 与 mock 规则的 2*3600 秒边界一致), 断言次数与强度不变 |
| date-unittest.cc: DateTest.DateParseLegacyUseCounter | tests/date_unittest.rs::test_date_parse_legacy_use_counter | legacy 回退计数, 测试名保留 |
| (规约补充) | tests/date_parser_unittest.rs (6) | ISO/legacy 黄金输入→输出对 (各格式分支), 算法, 五模式格式化, 端到端 |
| (审查补充 round1/round2) | tests/review_round{1,2}_unittest.rs (10) | TimeClip 边界/LRU 极端输入/解析器溢出/毫秒边值/等价年规约/双射回环/计数器溢出重置/对抗性畸形输入/Unicode 空白/AM-PM 规则 |
| (审查补充 round2 差分) | tests/review_round2_diff_unittest.rs (6) | 全语法差分矩阵 220+ 条 (ES5 合法/非法 + legacy 变体 + 括号/垃圾词/毫秒缩放/时区回绕端到端, 以原版状态机手工推演为准); DateCache 缓存窗口滑动语义 (跨 DST 段扩展/收缩/合并/大跳/回退 + 边界毫秒采样 + 无状态 oracle 对拍); 关键字表冲突面 (0 填充/长词放行/28 条前缀唯一性) |

(门禁实测记录见 ../../log/v8-date.md。)

