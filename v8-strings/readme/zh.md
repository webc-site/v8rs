# v8-strings (中文)

`v8/src/strings` (~8k 行, V8_INTL_SUPPORT 上游默认路径) 的 Rust 转写。
纯算法/表/谓词层: 不依赖堆对象 (String/Isolate/Factory), 对象耦合外壳归 v8-objects-strings / v8-api。

## 文件 → 模块映射

| v8 源文件 (行数) | Rust 模块 | 方式 |
|---|---|---|
| char-predicates.h/.cc/-inl.h (331) | src/char_predicates.rs | 表 1:1 (kOneByteCharFlags 256 项); 慢路径 icu_properties (IdStart/IdContinue/Uppercase 集) |
| unicode.h/-inl.h/.cc (3374; INTL 有效 ~400) | src/unicode.rs | Utf8/Utf16/Wtf8 编解码 1:1 自写; Uppercase/Letter → icu_properties; 非 INTL 大表 ~2300 行不迁移 (上游 INTL 路径不编译, 由 icu 表承接) |
| third_party/utf8-decoder/*.h (179) | src/utf8_dfa.rs | DFA 两表逐数字提取, from_table 显式转换 (无 unsafe) |
| unicode-decoder.h/.cc (321) | src/unicode_decoder.rs | NonAsciiStart (usize SWAR) + Utf8/Wtf8/Strict 三解码器 1:1 |
| string-case.h/.cc (203) | src/string_case.rs | SWAR 1:1 (usize word); prefix 精确停点 (原版保守下界的兼容加强) |
| string-hasher.h/.cc/-inl.h (622) | src/string_hasher.rs + src/rapidhash.rs + src/hash_field.rs + src/hash_seed.rs | rapidhash 算法 1:1 自写 (语义绑定, 禁库); HashSequentialString/TryParseArrayIndex 1:1 |
| v8/src/objects/name.h 哈希字段位域 (越界) | src/hash_field.rs | 位域常量 1:1 展开; v8-objects-strings 落地后归并 |
| v8/src/numbers/hash-seed.* + rapidhash secret.h (越界) | src/hash_seed.rs | 上游 v8-numbers 未开发, 原地承接; 归并点 |
| string-search.h (563) | src/string_search.rs | BM/BMH/线性/单字符策略与升降级 1:1; Isolate 三表内联为结构体成员; u8 定位 memchr 承接 |
| uri.h/.cc (678) | src/uri.rs | Encode/Decode/Escape/Unescape 纯算法内核 1:1 (输入 &[u8]/&[u16], 输出 Vec); 堆外壳归 objects |
| string-stream.h/.cc (709; POD 部分 ~350) | src/string_stream.rs | 分配器 + Put/Add 格式引擎 (%s/%w/%k/%i/%d/%u/%x/%c/%X/%f/%g/%G/%e/%E/%p); 对象打印归 objects |

不迁移 (登记理由):
- string-builder.h/.cc/-inl.h (717) — 全耦合 FixedArray/SeqString/Isolate, 归 v8-objects-strings (todo 看板 87 行)
- owning-external-string-resource.h (98) — 耦合 v8::String::ExternalStringResource (API 层), 归 v8-api
- UnicodeConfig (unicode.h 322-376) — 深耦合对象分配 (shared flag), 归 objects 层
- unicode.cc 非 INTL 大表 (~2300 行) — 上游 V8_INTL_SUPPORT 默认开启不编译, 语义由 icu_properties 承接
- V8_ENABLE_SEEDED_ARRAY_INDEX_HASH (默认 false) — xorshift-multiply 混淆未承接

## 转换脚本 (tool/src/strings/)

- char-predicates.js → src/char_predicates.rs (表 + 位标志 + 快慢路径)
- unicode.js → src/unicode.rs + utf8_dfa.rs + unicode_decoder.rs (DFA 表从 third_party 逐数字提取; 校验 transitions=256/states=108|88)
- string-hasher.js → src/string_hasher.rs + hash_field.rs + hash_seed.rs + rapidhash.rs (name.h 位域按 kBitsPerInt=32 展开; RAPIDHASH_DEFAULT_SECRET 从 secret.h 提取)
- string-algos.js → src/string_search.rs + uri.rs + string_case.rs + string_stream.rs (常量/字符集合/枚举/骨架)
- unittest-data.js → tests/unicode_cases.rs (黄金数据 132 组, 从 unicode-unittest.cc 大表提取)

复跑幂等; 复跑会覆盖就地修订, 仅限重生成基线。

## 库承接 vs 自写边界 (todo 十章白名单)

- icu_properties (icu4x 2.x): IdStart/IdContinue (u_hasBinaryProperty 等价, 含 Other_ID_*), Uppercase (u_isupper), GeneralCategory (Zs 白空白 / L* 字母) — 属性表承接
- memchr: string_search 的 u8 subject 首字符定位
- 自写 (语义绑定, 禁库): rapidhash 全算法 (string hash 的 seed+迭代序), UTF-8/UTF-16 编解码 DFA 与循环, BM/BMH 搜索策略机, URI 编解码, SWAR 大小写, 哈希字段位域

## 测试映射 (unittests/strings 1:1)

| 原测试 | Rust 测试 | 说明 |
|---|---|---|
| char-predicates-unittest.cc (5 TEST) | tests/char_predicates.rs (5) | 测试名保留 (white_space / white_space_or_line_terminator / identifier_start / identifier_part / supplementary_plane_identifiers) |
| unicode-unittest.cc (3 TEST) | tests/unicode.rs (3) + tests/unicode_cases.rs | 黄金数据 132 组转数据文件; 三路解码对比 (normal/incremental/utf16) |
| (规约补充) | tests/uri.rs, tests/string_search.rs, tests/string_case.rs, tests/string_stream.rs, tests/hash_seed.rs 等 | 并行 Test 会话按 mjsunit/cctest 规约补充 |
