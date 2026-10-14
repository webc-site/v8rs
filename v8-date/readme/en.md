# v8-date (English)

Rust transcription of `v8/src/date` (~1.9k lines).
JS Date semantics bound 1:1: date string parsing (ISO 8601 + lenient legacy format), timezone rule caching (DateCache, 32-segment LRU + DST bisection), UTC/local conversion, boundary conditions (year 0/275760, millisecond precision, NaN propagation) — line-by-line correspondence with upstream.

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
