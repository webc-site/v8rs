# v8-common 评测报告

v8-common (L0 地基层工具) 核心热点操作, 基于 criterion 评测; 实现方: crate API (v8-common)、std 等价写法 (std)、gxhash 容器对照 (gxhash)。

生成时间: 2026-10-11T08:03:05.822Z

## 绝对耗时
| 操作 | 实现 | 中位数 / 次 | 均值 / 次 | 95% 置信区间 (中位数) |
|---|---|---|---|---|
| 指针标签判定 (Smi / HeapObject / Weak) | v8-common | 1.38 µs | 1.39 µs | 1.35 µs ~ 1.42 µs |
| 指针压缩 → 解压往返 | v8-common | 872.5 ns | 879.9 ns | 868.8 ns ~ 886.1 ns |
| 对齐向上取整 + padding 算术 | v8-common | 1.82 µs | 1.83 µs | 1.82 µs ~ 1.83 µs |
| object_hash 建表 + 探针 (容器) | v8-common | 16.38 µs | 16.40 µs | 16.33 µs ~ 16.44 µs |
| 指针标签判定 (Smi / HeapObject / Weak) | std | 1.46 µs | 1.46 µs | 1.46 µs ~ 1.46 µs |
| 指针压缩 → 解压往返 | std | 620.5 ns | 620.0 ns | 619.6 ns ~ 621.1 ns |
| 对齐向上取整 + padding 算术 | std | 1.99 µs | 1.98 µs | 1.97 µs ~ 1.99 µs |
| object_hash 建表 + 探针 (容器) | std | 17.59 µs | 24.74 µs | 17.48 µs ~ 26.25 µs |
| 指针标签判定 (Smi / HeapObject / Weak) | gxhash | 1.67 µs | 1.95 µs | 1.50 µs ~ 2.10 µs |
| 指针压缩 → 解压往返 | gxhash | 897.8 ns | 998.6 ns | 892.8 ns ~ 938.9 ns |
| 对齐向上取整 + padding 算术 | gxhash | 2.00 µs | 2.01 µs | 1.99 µs ~ 2.01 µs |
| object_hash 建表 + 探针 (容器) | gxhash | 10.18 µs | 10.19 µs | 10.10 µs ~ 10.25 µs |

## 相对加速比 (>1 = 快于 std 基线)
| 操作 | 实现 | 相对 std 加速比 |
|---|---|---|
| 指针标签判定 (Smi / HeapObject / Weak) | v8-common | 1.06× |
| 指针压缩 → 解压往返 | v8-common | 0.71× |
| 对齐向上取整 + padding 算术 | v8-common | 1.09× |
| object_hash 建表 + 探针 (容器) | v8-common | 1.07× |
| 指针标签判定 (Smi / HeapObject / Weak) | std | 1.00× |
| 指针压缩 → 解压往返 | std | 1.00× |
| 对齐向上取整 + padding 算术 | std | 1.00× |
| object_hash 建表 + 探针 (容器) | std | 1.00× |
| 指针标签判定 (Smi / HeapObject / Weak) | gxhash | 0.87× |
| 指针压缩 → 解压往返 | gxhash | 0.69× |
| 对齐向上取整 + padding 算术 | gxhash | 0.99× |
| object_hash 建表 + 探针 (容器) | gxhash | 1.73× |

> 加速比为相对 std 等价写法基线的中位数耗时比值, 越大越好。
>
> criterion 0.8, tikv-jemallocator 全局分配器, 每项 20 样本 × 500ms, fastrand 固定种子输入。
