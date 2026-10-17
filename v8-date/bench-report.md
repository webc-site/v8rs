# v8-date 性能评测报告

> 基于 criterion 的 v8rs v8-date crate 与对比库评测

- 生成时间: 2026-10-11T08:24:39.101Z
- criterion 参数: `--warm-up-time 2 --measurement-time 3 --sample-size 20`
- crate: `v8-date`

## 评测操作说明

- parse_date_time: 日期时间字符串 -> Epoch 毫秒（ISO / RFC 2822 / legacy / 超界年份 / 非法样本）
- epoch_ms_to_ymd: Epoch 毫秒 -> (年, 月, 日)，DateCache 缓存命中查询路径
- epoch_ms_to_time_of_day: Epoch 毫秒 -> 日内 (时, 分, 秒, 毫秒)，LocalTimePhase 风格纯函数拆分

std-systemtime 无字符串解析能力，未注册 parse 操作，仅作时间戳处理对照组。

## 评测结果

| 评测操作 | 实现 | 平均耗时 | 中位数 | 相对 v8-date | 标准误 |
| --- | --- | --- | --- | --- | --- |
| epoch_ms_to_time_of_day | `chrono` | 403.2 ns | 403.2 ns | 2.04x | ±0.1 ns |
| epoch_ms_to_time_of_day | `std-systemtime` | 413.2 ns | 408.3 ns | 2.09x | ±2.8 ns |
| epoch_ms_to_time_of_day | `v8-date` | 197.7 ns | 194.7 ns | 1.00x | ±3.0 ns |
| epoch_ms_to_ymd | `chrono` | 763.2 ns | 639.5 ns | 4.95x | ±65.9 ns |
| epoch_ms_to_ymd | `std-systemtime` | 841.5 ns | 836.5 ns | 5.46x | ±2.0 ns |
| epoch_ms_to_ymd | `v8-date` | 154.0 ns | 151.8 ns | 1.00x | ±1.7 ns |
| parse_date_time | `chrono` | 2.50 µs | 2.50 µs | 1.67x | ±1.8 ns |
| parse_date_time | `v8-date` | 1.49 µs | 1.47 µs | 1.00x | ±36.1 ns |

耗时为一次遍历完整固定样本集的总时间，越小越好；倍率大于 1x 表示慢于 v8-date。
