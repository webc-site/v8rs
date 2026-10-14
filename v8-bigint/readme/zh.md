# v8-bigint

v8/src/bigint 的 Rust 转写 (自含库, 不依赖 v8-base)。模块与原版文件 1:1。

## 模块映射

| v8/src/bigint | v8-bigint/src | 说明 |
|---|---|---|
| bigint.h + bigint-inl.h | bigint.rs | digit 原语, 快操作 (加减/位运算/移位/截断), 长度预算, Status, Platform |
| bigint.h:271-275 + builtins-bigint.tq:454 BigIntBitwiseAndImpl | bitwise.rs | BitwiseAnd 顶层编排: 符号派发 / 预算长度 (min / max+1 / 正侧) / PosNeg 换参 / Canonicalize→norm |
| bigint-internal.h/.cc | bigint_internal.rs | Processor: 慢操作入口, 中断预算, CachedMod 逆元缓存 |
| div-helpers-inl.h | div_helpers.rs | 复制 / digit 内移位 / ShiftedDigits / scratch 常量 |
| div-schoolbook.cc | div_schoolbook.rs | Knuth Algorithm D + 乘法逆元 digit 除法 |
| div-burnikel.cc | div_burnikel.rs | Burnikel-Ziegler 递归除法 |
| div-barrett.cc | div_barrett.rs | Barrett 除法 + Newton 求逆 |
| (bigint-inl.h 乘法部分) | mul_helpers.rs | 单 digit 乘 / Schoolbook / CachedMod 专用变体 |
| mul-karatsuba.cc | mul_karatsuba.rs | Karatsuba 乘法 |
| mul-toom.cc | mul_toom.rs | Toom-Cook 乘法 |
| mul-fft.cc | mul_fft.rs | Schönhage-Strassen FFT 乘法 |
| fromstring.cc + bigint-inl.h 解析部分 | from_string.rs | FromStringAccumulator + 经典/快速/2 幂解析 |
| tostring.cc | to_string.rs | 经典 / 2 幂 / 快速分治转字符串 |
| vector-arithmetic-inl.h | vector_arithmetic.rs | 向量加减溢出/借位, 位长 |
| util.h | util.rs | RoundUp / ctz / 2 幂判定 |
| — | error.rs | thiserror 错误壳 (预留) |

## 转换脚本

| 脚本 (tool/src/bigint/) | 产物 | 性质 |
|---|---|---|
| gen_consts.js | from_string.rs / to_string.rs 基线 (kCharValue / kCharBits / kMaxBitsPerChar 常量表) | 基线生成器; 语义体已就地补全, 勿复跑 (会覆盖修订) |
| util.js | util.rs 整文件 | 纯规则文件, 脚本所有, 可幂等复跑 |
| bitwise-and.js | bitwise.rs.skel (签名表 + 落地/推迟处置) | 结构锚对数断言 121 条 (bigint.h 声明 ×3, bigint-inl.h 实现区循环/借位/Add(Z,1) 锚, tq BigIntBitwiseAndImpl 14 锚, objects/bigint.cc 壳 3+3, and.js 金标准 20 组逐项), 只读交叉核对, 可幂等复跑 |

其余原版文件 (bigint-inl.h 语义体 / div-* / mul-* / tostring.cc 主体) 为逐行语义转写, 非纯规则代码, 不设脚本 — 按 todo.md 十一章 "脚本只产出规则代码" 的边界执行。

## 测试覆盖映射

原版测试源: v8/test/bigint/bigint-shell.cc (9 个 Test 入口, 双算法对拍断言)。

| 原测试 (bigint-shell.cc) | Rust 测试 (tests/bigint_shell.rs) | 断言方式 (强度守恒) |
|---|---|---|
| TestKaratsuba | karatsuba | multiply_karatsuba vs multiply_schoolbook, 34..=102 双重循环全组合 (2415 例) |
| TestToom | toom | multiply_toom_cook vs multiply_karatsuba, 150..=220 全组合 |
| TestFFT | fft | multiply_fft vs multiply_toom_cook, 种子随机采样 (276 例) |
| TestBurnikel | burnikel | divide_burnikel_ziegler vs divide_schoolbook, 商与余数双断言, 28..=114 |
| TestBarrett | barrett | divide_barrett vs divide_burnikel_ziegler, 商余双断言 + 13000+ digit 随机大例 |
| TestCachedMod | cachedmod | cached_mod vs modulo_small/modulo_large, b∈2..=20 × a∈[b,2b] × 200 轮 |
| TestToString | tostring | write_impl(fast) vs write_impl(classic), size 11..46 × radix 2..=36, 长度+字节双断言 |
| TestFromString | fromstring | parse_large vs parse_classic, 12..50 × 随机 radix (非 2 幂) |
| TestFromStringBase2 | fromstring_base2 | ToString→Parse→parse_string 往返, size 1..100 × 2/4/8/16/32 进制 |

测试辅助 (tests/common/mod.rs): RNG (xorshift128 + MurmurHash3, 与原版一致, 默认种子 314159265359), GenerateRandom 四模式, GenerateRandomString, FormatHex — 均逐行转写。

### BitwiseAnd 单元测试映射

顶层编排面 `bitwise.rs` (builtins-bigint.tq:454 BigIntBitwiseAndImpl 的无堆等价: 零短路 / 符号四分支 / 长度预算 min·max+1·正侧 / NegPos 换参 / Canonicalize→norm) 的单元测试 6 例:

| 原测试来源 | Rust 测试 (tests/bitwise_and.rs) | 断言方式 (强度守恒) |
|---|---|---|
| mjsunit/harmony/bigint/and.js (bigint-tester.py 金标准) | and_vectors | 20 组 (符号, hex 幅度) 逐字节照抄; owned 版与预算缓冲非自有版双路径, 符号 + 规格化幅度双断言 |
| mjsunit/compiler/bigint-bitwise-and.js TestBitwiseAnd | and_sign_dispatch | 符号四组合 12&10 / -4&-6 / -4&10 / 12&-6 == 8 |
| mjsunit/compiler/bigint-bitwise-and.js (grow out of one digit) | and_grow_out_of_digit | -(2^63+1) & -(2^63) == -(2^64), 预算恰 max+1 |
| builtins-bigint.tq 零短路面 | and_zero_shortcut | 0n & y / x & 0n → 正零, 预算 0 |
| bigint.h:267-270 补码语义边界 | and_full_overflow_bits | (2^64-1)&-2^64 == 0; (2^66-1)&-2^64 == 3·2^64; -(2^64-1) 自与; x & -1n == x |
| tq 长度公式面 | and_result_length_budget | min / max+1 / 正侧长度 四分支逐项 + 非规范输入按 norm 计 |

### 推迟登记 (硬耦合 heap, 本 crate 边界外, 显式不实现)

- objects/bigint.cc:1723-1757 `MutableBigInt_BitwiseAnd*AndCanonicalize` 三壳 (Tagged<BigInt>/Factory) → 归 L3 objects crate
- tq `AllocateEmptyBigIntNoThrow` / `BigIntTooBig` RangeError 抛错面 → 归 L4 builtins crate
- 无隐式桩: bitwise.rs 三个公开函数全实现, 无 todo!/unimplemented!

### 跳过项与理由

| 原版内容 | 位置 | 理由 |
|---|---|---|
| ThresholdRunner (8 组阈值基准) | bigint-shell.cc | 性能调参工具, 输出耗时对比图, 无正确性断言 |
| RNG::Initialize 的 CHECK / main/CLI | bigint-shell.cc | 测试驱动壳, 由 Rust 测试框架承担 |
| CompareToDouble 单元测试 | v8/test/unittests/numbers/bigint-unittest.cc | 属于 src/objects/bigint (依赖 isolate/heap/factory), 非 src/bigint 库; 待 objects 层 crate 迁移时处理 |
| test/mjsunit/*bigint* | v8/test/mjsunit 等 | JS 引擎层端到端用例, 依赖完整运行时, 不属于本 crate; 例外: harmony/bigint/and.js 与 compiler/bigint-bitwise-and.js 的纯数值向量已作 BitwiseAnd 金标准迁入 (见上节) |

注: 本仓库 v8 checkout 无 `v8/test/unittests/bigint/` 目录, src/bigint 的原生测试只有上述 `numbers/bigint-unittest.cc` (对象层) 与 `test/bigint/bigint-shell.cc` (处理器层差分驱动, 已 1:1 迁移)。v8-common/tests/globals.rs 经实查不含引用本 crate 的 BigInt 黄金测试。

## 与原版的结构差异 (语义等价)

- Digits/RWDigits → `&[u64]` / `&mut [u64]`; Normalize → `norm()` 返回截断视图。
- 原版指针别名就地运算 (SumDiff/SubtractSigned 自身) → 独立就地变体函数 (`subtract_signed_zx/_zy` 等)。
- 原版 Processor/ProcessorImpl 两层 → 单一 `Processor<P: Platform>`; Platform 仅保留中断钩子 (分配由 Vec 承担)。
- 原版 scratch 缓存 (GetSmallScratch) → 局部 Vec (语义等价, 少一次复用)。
- V8_ADVANCED_BIGINT_ALGORITHMS 恒开 (全部高级算法已落地)。

## 覆盖盘点

原版 v8/src/bigint 16 个文件全部落地, 函数级 1:1 核对无缺口:
digit 算术 / 加减 / 位运算 (9 变体含补码语义) / 任意宽移位 (含负数向下取整与进位扩展边界) / asIntN·asUintN 截断 / 乘法 (schoolbook·karatsuba·toom·FFT 含平方与非对称分块) / 除法 (schoolbook·Burnikel-Ziegler·Barrett+Newton·CachedMod 折叠) / 字符串互转 (radix 2..=36, 2 幂特化, radix10 魔数除法, 分治快路径)。

## 门禁

```
CARGO_TARGET_DIR=target/bigint1 cargo clippy -p v8-bigint --all-features -- -D warnings
CARGO_TARGET_DIR=target/bigint1 cargo nextest run -p v8-bigint --all-features
```
