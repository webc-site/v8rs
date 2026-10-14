# v8-strings 核心操作评测

> 方法: criterion 短程测量 (sample-size 10, 预热 1s, 测量 1s), jemalloc 全局分配器。

> 语料: 固定确定性样本 (ASCII / Latin / CJK / 代理对混合), 长度 1k 与 32 码点; 数值为 criterion 整样本每次迭代均值纳秒。

## transcode_encode · UTF-16 转 UTF-8

| 实现 | ascii_1k (1024) | cjk_1k (1024) | latin_1k (1024) | mixed_1k (1233) | mixed_32 (39) |
| --- | --- | --- | --- | --- | --- |
| v8rs | 1.62 us (1.24x) | 2.97 us (1.71x) | 2.23 us (1.51x) | 3.22 us (1.32x) | 105.0 ns (0.80x) |
| std | 1.30 us (1.00x) | 1.74 us (1.00x) | 1.48 us (1.00x) | 2.43 us (1.00x) | 132.0 ns (1.00x) |

## transcode_decode · UTF-8 转 UTF-16

| 实现 | ascii_1k (1024) | cjk_1k (1024) | latin_1k (1024) | mixed_1k (1233) | mixed_32 (39) |
| --- | --- | --- | --- | --- | --- |
| v8rs | 72.1 ns (0.05x) | 9.86 us (2.79x) | 5.53 us (2.00x) | 8.12 us (2.27x) | 234.9 ns (2.02x) |
| std | 1.57 us (1.00x) | 3.54 us (1.00x) | 2.76 us (1.00x) | 3.57 us (1.00x) | 116.0 ns (1.00x) |

## copy_chars · CopyChars (良构复制)

| 实现 | ascii_1k (1024) | cjk_1k (1024) | latin_1k (1024) | mixed_1k (1233) | mixed_32 (39) |
| --- | --- | --- | --- | --- | --- |
| v8rs | 626.8 ns (23.81x) | 623.0 ns (23.98x) | 631.4 ns (24.07x) | 822.7 ns (26.51x) | 25.9 ns (12.02x) |
| std | 26.3 ns (1.00x) | 26.0 ns (1.00x) | 26.2 ns (1.00x) | 31.0 ns (1.00x) | 2.2 ns (1.00x) |

## scan_identifier · 标识符 start/continue 扫描

| 实现 | ascii_1k (1024) | cjk_1k (1024) | latin_1k (1024) | mixed_1k (1233) | mixed_32 (39) |
| --- | --- | --- | --- | --- | --- |
| v8rs | 942.2 ns (0.85x) | 7.40 us (2.28x) | 4.68 us (0.67x) | 8.25 us (1.24x) | 246.8 ns (1.18x) |
| unicode-ident | 1.28 us (1.16x) | 1.19 us (0.37x) | 1.60 us (0.23x) | 1.23 us (0.19x) | 40.7 ns (0.19x) |
| std | 1.11 us (1.00x) | 3.24 us (1.00x) | 7.03 us (1.00x) | 6.65 us (1.00x) | 208.7 ns (1.00x) |

## case_lower · ASCII 小写映射

| 实现 | ascii_1k (1024) | cjk_1k (1024) | latin_1k (1024) | mixed_1k (1233) | mixed_32 (39) |
| --- | --- | --- | --- | --- | --- |
| v8rs | 83.1 ns (3.83x) | 0.7 ns (0.01x) | 0.7 ns (0.02x) | 0.7 ns (0.01x) | 0.7 ns (0.20x) |
| std | 21.7 ns (1.00x) | 58.3 ns (1.00x) | 39.3 ns (1.00x) | 48.0 ns (1.00x) | 3.7 ns (1.00x) |

## find_byte · 单字节搜索

| 实现 | ascii_1k (1024) | cjk_1k (1024) | latin_1k (1024) | mixed_1k (1233) | mixed_32 (39) |
| --- | --- | --- | --- | --- | --- |
| v8rs | 4.6 ns (0.31x) | 3.6 ns (0.72x) | 3.6 ns (1.38x) | 4.8 ns (0.30x) | 3.7 ns (0.63x) |
| memchr | 2.6 ns (0.18x) | 1.2 ns (0.24x) | 1.3 ns (0.50x) | 3.3 ns (0.21x) | 1.3 ns (0.21x) |
| std | 14.8 ns (1.00x) | 5.0 ns (1.00x) | 2.6 ns (1.00x) | 16.3 ns (1.00x) | 5.9 ns (1.00x) |

## find_substring · 子串搜索

| 实现 | ascii_1k (1024) | cjk_1k (1024) | latin_1k (1024) | mixed_1k (1233) | mixed_32 (39) |
| --- | --- | --- | --- | --- | --- |
| v8rs | 63.5 ns (0.07x) | 648.4 ns (0.25x) | 577.6 ns (0.33x) | 78.5 ns (0.04x) | 9.4 ns (0.13x) |
| memchr | 32.5 ns (0.03x) | 102.1 ns (0.04x) | 49.9 ns (0.03x) | 59.5 ns (0.03x) | 17.0 ns (0.24x) |
| std | 936.3 ns (1.00x) | 2.55 us (1.00x) | 1.74 us (1.00x) | 2.08 us (1.00x) | 71.4 ns (1.00x) |

## normalize · NFC 规范化

| 实现 | ascii_1k (1024) | cjk_1k (1024) | latin_1k (1024) | mixed_1k (1233) | mixed_32 (39) |
| --- | --- | --- | --- | --- | --- |
| unicode-normalization | 9.80 us (-) | 12.02 us (-) | 25.31 us (-) | 18.83 us (-) | 538.6 ns (-) |

倍率: 相对 std 基线的纳秒比 (越小越快)。
