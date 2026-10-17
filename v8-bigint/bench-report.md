# v8-bigint 评测报告

crate 自包含 criterion 评测; 固定种子确定性伪随机样本。

## 汇总

| 操作 | 进制 | 位宽 | v8rs | num-bigint | 对比库 / v8rs |
| --- | --- | --- | --- | --- | --- |
| add | - | 64 | 31.43 ns | 15.03 ns | 0.48 |
| add | - | 256 | 35.67 ns | 43.22 ns | 1.21 |
| add | - | 1k | 35.86 ns | 48.3 ns | 1.35 |
| add | - | 4k | 90.91 ns | 110.1 ns | 1.21 |
| add | - | 16k | 338.44 ns | 379.44 ns | 1.12 |
| add | - | 64k | 1.3 µs | 1.5 µs | 1.2 |
| sub | - | 64 | 25.77 ns | 8.37 ns | 0.32 |
| sub | - | 256 | 29.46 ns | 21.33 ns | 0.72 |
| sub | - | 1k | 38.34 ns | 28.21 ns | 0.74 |
| sub | - | 4k | 101.84 ns | 57.6 ns | 0.57 |
| sub | - | 16k | 403.08 ns | 198.21 ns | 0.49 |
| sub | - | 64k | 1.6 µs | 738.7 ns | 0.46 |
| mul | - | 64 | 27.46 ns | 14.04 ns | 0.51 |
| mul | - | 256 | 40.33 ns | 33.59 ns | 0.83 |
| mul | - | 1k | 210.13 ns | 229.78 ns | 1.09 |
| mul | - | 4k | 2.5 µs | 3 µs | 1.21 |
| mul | - | 16k | 25.2 µs | 30.5 µs | 1.21 |
| mul | - | 64k | 203.5 µs | 244.7 µs | 1.2 |
| div | - | 256 | 116.55 ns | 69.37 ns | 0.6 |
| div | - | 1k | 255.22 ns | 304.69 ns | 1.19 |
| div | - | 4k | 2.6 µs | 2.2 µs | 0.87 |
| div | - | 16k | 20.9 µs | 22.3 µs | 1.07 |
| div | - | 64k | 169.9 µs | 203.5 µs | 1.2 |
| rem | - | 256 | 106.72 ns | 70.66 ns | 0.66 |
| rem | - | 1k | 259.64 ns | 292.39 ns | 1.13 |
| rem | - | 4k | 3.4 µs | 2.3 µs | 0.66 |
| rem | - | 16k | 21.4 µs | 22 µs | 1.03 |
| rem | - | 64k | 169.4 µs | 191.2 µs | 1.13 |
| to_string | 10 | 64 | 54.7 ns | 26.79 ns | 0.49 |
| to_string | 10 | 256 | 290.94 ns | 146.66 ns | 0.5 |
| to_string | 10 | 1k | 1.9 µs | 1.9 µs | 1.01 |
| to_string | 10 | 4k | 16.4 µs | 12.6 µs | 0.77 |
| to_string | 10 | 16k | 115.7 µs | 86.2 µs | 0.75 |
| to_string | 10 | 64k | 901.9 µs | 675.1 µs | 0.75 |
| to_string | 16 | 64 | 83.17 ns | 22.94 ns | 0.28 |
| to_string | 16 | 256 | 89.78 ns | 42.51 ns | 0.47 |
| to_string | 16 | 1k | 315.71 ns | 142.82 ns | 0.45 |
| to_string | 16 | 4k | 1 µs | 609.18 ns | 0.59 |
| to_string | 16 | 16k | 3.9 µs | 1.8 µs | 0.45 |
| to_string | 16 | 64k | 15.7 µs | 7 µs | 0.44 |
| from_string | 10 | 64 | 47.98 ns | 61.54 ns | 1.28 |
| from_string | 10 | 256 | 97.06 ns | 178.15 ns | 1.84 |
| from_string | 10 | 1k | 541.47 ns | 567.78 ns | 1.05 |
| from_string | 10 | 4k | 3.5 µs | 3.3 µs | 0.95 |
| from_string | 10 | 16k | 39.2 µs | 28 µs | 0.71 |
| from_string | 10 | 64k | 275 µs | 350.1 µs | 1.27 |
| from_string | 16 | 64 | 45.56 ns | 49.18 ns | 1.08 |
| from_string | 16 | 256 | 92.25 ns | 124.01 ns | 1.34 |
| from_string | 16 | 1k | 305.14 ns | 379.55 ns | 1.24 |
| from_string | 16 | 4k | 1.1 µs | 1.7 µs | 1.63 |
| from_string | 16 | 16k | 4.1 µs | 6.1 µs | 1.49 |
| from_string | 16 | 64k | 14.9 µs | 44.2 µs | 2.96 |

## 图表

见 [bench-chart.svg](./bench-chart.svg)。

## 说明

- 对比库仅 num-bigint: malachite-nz 在本 workspace 编译过重, 未引入。
- Barrett 除法切换点为 >=13000 digit (~832k bit), 超出 64k bit 样本上限, 本轮不评测; div/rem 已覆盖 Burnikel-Ziegler 边界 (57 digit)。
- 乘法位宽跨越 schoolbook / Karatsuba (34) / Toom-Cook (210) / FFT (720) digit 阈值; 字符串转换覆盖 radix 10 (通用快速路径) 与 16 (2 幂 shortcut)。
- 样本: 已规格化 (msd 非零) 随机 digit; div/rem 除数取被除数一半位宽 (64 bit 档跳过)。
- 数值为短测量窗口 (预热 1s / 测量 2s) 下 criterion 均值点估计; 可用环境变量 BENCH_ARGS 覆盖做长测。
