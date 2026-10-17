# v8-parsing 词法扫描评测

> v8rs Scanner (完整 ECMAScript 词法语义) 对比 单遍手写简易 tokenizer (只扫空白/注释/标点的最小等价路径)。语义成本 = v8-scanner 平均耗时 / naive-tokenizer 平均耗时, 表示完整词法语义的额外开销倍率; lookahead 一行反过来, 表示 4 槽环形缓存命中相对无缓存重扫的收益。

生成时间: 2026-10-11T08:28:12.478Z

## scan_tokens

全量词法扫描: 标识符/关键字/数字字面量/字符串/模板/注释/正则起始歧义

| 实现 | 样本 | 规模 | 平均耗时 | 中位数 | 标准差 | 吞吐 | 语义成本 |
| --- | --- | --- | --- | --- | --- | --- | --- |
| naive-tokenizer | large 69.84_KiB | 69.84 KiB | 84.64 µs | 84.50 µs | 958.0 ns | 805.9 MiB/s | - |
| v8-scanner | large 69.84_KiB | 69.84 KiB | 374.03 µs | 375.53 µs | 6.11 µs | 182.4 MiB/s | 4.42× |
| naive-tokenizer | medium 6.98_KiB | 6.98 KiB | 8.38 µs | 8.33 µs | 111.6 ns | 813.9 MiB/s | - |
| v8-scanner | medium 6.98_KiB | 6.98 KiB | 37.45 µs | 37.26 µs | 905.8 ns | 182.1 MiB/s | 4.47× |
| naive-tokenizer | small 596_B | 596 B | 714.5 ns | 708.5 ns | 27.7 ns | 795.5 MiB/s | - |
| v8-scanner | small 596_B | 596 B | 3.03 µs | 3.02 µs | 30.4 ns | 187.5 MiB/s | 4.24× |

## scan_regexp

正则起始歧义重扫: '/' 处走 scan_regexp_pattern + scan_regexp_flags 公开 API

| 实现 | 样本 | 规模 | 平均耗时 | 中位数 | 标准差 | 吞吐 | 语义成本 |
| --- | --- | --- | --- | --- | --- | --- | --- |
| naive-tokenizer | regexp 195_B | 195 B | 259.5 ns | 259.0 ns | 3.5 ns | 716.8 MiB/s | - |
| v8-scanner | regexp 195_B | 195 B | 971.8 ns | 965.7 ns | 13.3 ns | 191.4 MiB/s | 3.75× |

## lookahead

4 槽环形 lookahead 命中路径: 缓存填满后连续 peek_ahead / peek_ahead_ahead

| 实现 | 样本 | 规模 | 平均耗时 | 中位数 | 标准差 | 吞吐 | 语义成本 |
| --- | --- | --- | --- | --- | --- | --- | --- |
| naive-tokenizer | hit_path | 1024 elem | 5.82 µs | 5.83 µs | 95.5 ns | 176 Melem/s | - |
| v8-scanner | hit_path | 1024 elem | 644.8 ns | 644.6 ns | 4.8 ns | 1588 Melem/s | 0.11× |

## literal_fill

LiteralBuffer 写入: start 复位 + add_char 单字节 (Latin-1) 路径

| 实现 | 样本 | 规模 | 平均耗时 | 中位数 | 标准差 | 吞吐 | 语义成本 |
| --- | --- | --- | --- | --- | --- | --- | --- |
| naive-tokenizer | latin1 | 131072 elem | 82.90 µs | 82.44 µs | 882.4 ns | 1581 Melem/s | - |
| v8-scanner | latin1 | 131072 elem | 88.20 µs | 88.48 µs | 899.9 ns | 1486 Melem/s | 1.06× |
