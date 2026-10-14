# v8-numbers

原版 `v8/src/numbers` (~2.9k 行) 的 Rust 转写: Number↔String 整族
(ECMAScript 语义红线层)、parseInt/parseFloat、Math.random 状态机、
hash-seed 数据面、ieee754 pow、IntegerLiteral。

## 文件 → 模块映射

| 原版 (v8/src/numbers) | 行数 | 本 crate (v8-numbers/src) |
|---|---|---|
| conversions.h + conversions-inl.h | 629 | conversions/mod.rs (常量/内联转换族) |
| conversions.cc 43-339 (查表+builder) | 297 | conversions/simple_builder.rs |
| conversions.cc 341-991 (parseInt) | 651 | conversions/parse_int.rs |
| conversions.cc 842-991 (StringToDouble) | 150 | conversions/parse_double.rs |
| conversions.cc 1147-1656 (Number→String) | 510 | conversions/to_string.rs |
| conversions.cc 1590-1656 (IsSpecialIndex) | 67 | conversions/special_index.rs |
| conversions 表/常量 | — | conversions/tables.rs (脚本生成) |
| hash-seed.{h,cc,-inl.h} | 251 | hash_seed.rs (脚本基线) |
| ieee754.{h,cc} | 81 | ieee754.rs |
| integer-literal.{h,-inl.h} | 151 | integer_literal.rs |
| math-random.{h,cc} | 126 | math_random.rs |

转换脚本 (tool/src/numbers/, 复跑幂等):

| 脚本 | 产物 |
|---|---|
| conversions.cc.js | src/conversions/tables.rs (2×200 项查表 + 25 常量, 对数断言) |
| hash-seed.cc.js | src/hash_seed.rs (RAPIDHASH_DEFAULT_SECRET 提取) |
| gay-data.js | tests/data/gay-{shortest,fixed,precision}.tsv (各 10 万条) |
| oracle-data.js | tests/data/oracle-{parse,fmt,rng,pow}.tsv (bun/JSC 差分语料) |

## 语义红线实现 (todo 10.3)

- Number→String 格式层 (阈值 21 位 / 0.000001 / e±XX 指数切换): 自写
  (to_string.rs, ECMA-262 9.8.1 四分支); 最短十进制内核 = zmij
  (Schubfach) 解析还原为 (digits, dec_exp) 对, gay-shortest 10 万条 100% 命中
- parseInt/parseFloat: 解析扫描层自写; 十进制舍入内核 = v8-base strtod
  (bignum 兜底正确舍入); 2^n 进制 InternalStringToIntDouble 1:1 转写
- Math.random: XorShift128+ / MurmurHash3 / ToDouble 1:1 转写,
  与公开算法独立重实现 (bun) 逐序列差分
- 整数字符串内核 itoa; toFixed/toPrecision/toExponential 内核 = v8-base
  DoubleToAscii

## 测试覆盖 (39 用例, debug + release 双 profile 实测; 含 tests/main.rs 日志样板)

| 原测试 | Rust 测试 | 说明 |
|---|---|---|
| unittests/numbers/conversions-unittest.cc | tests/conversions.rs | 测试名保留 gtest 原名; Isolate 绑定用例见"推迟"; BitField/BitField64 属 base::BitField 泛型 (转写在 v8-base/src/bit_field.rs), PositiveNumberToUint32 属 Tagged 族 (见"推迟") |
| unittests/numbers/strtod-unittest.cc | tests/conversions.rs::strtod | 主干全量 (表驱动, 期望全字面量) |
| unittests/base/dtoa-unittest.cc DtoaGayShortest | tests/dtoa_golden.rs::dtoa_gay_shortest | zmij + v8-base SHORTEST 双通道各全量 10 万条, 0 差异 (上游修复后恢复) |
| unittests/base/dtoa-unittest.cc DtoaGayFixed | tests/dtoa_golden.rs::dtoa_gay_fixed | v8-base FIXED 通道全量 10 万条, 0 差异 |
| unittests/base/dtoa-unittest.cc DtoaGayPrecision | tests/dtoa_golden.rs::dtoa_gay_precision | v8-base PRECISION 通道全量 10 万条, 0 差异 (上游修复后恢复) |
| unittests/base/dtoa-unittest.cc DtoaVariousDoubles | tests/conversions.rs::dtoa_various_doubles | 不触发缺陷的取值子集 |
| test/mjsunit/number-tostring.js | tests/conversions.rs::mjsunit_number_tostring_vectors | V8 权威向量: toString 阈值 + radix 整数/小数/大数域 |
| (bun 差分, 自制) | tests/oracle.rs | Number/parseFloat/parseInt 12.8k 条 (位级); toString/toFixed/toPrecision/toExponential 1.6 万条逐字节差分 + toString(radix) 4928 条装载 (implementation-defined 不差分); xorshift128+/MurmurHash3 序列重放; Math.pow 边界 (8 ulp 容差) |
| test/mjsunit/parse-int-float.js | tests/conversions.rs::mjsunit_parse_int_float_vectors | V8 权威向量: 空白/前缀/junk/大数 Infinity |
| unittests/numbers/conversions-unittest.cc NoHandlesForTryNumberToSize (+MaxSizePlusOne) | tests/conversions.rs::try_number_to_size | double 数值分支 (Smi/HeapNumber 分派推迟) |
| (语义规约) | tests/misc.rs | IntegerLiteral / HashSeed 数据面 |
| unittests/numbers/diy-fp-unittest.cc | — | 属 v8-base (已有 v8-base/tests/diy_fp.rs) |
| unittests/numbers/bigint-unittest.cc | — | 属 v8-bigint |

## 推迟 (对象系统/跨 crate 依赖, 未虚构符号)

- StringToInt/StringToDouble/TryStringToInt 的 Isolate/Handle<String> 入口
  (raw 等价已落地, 对象绑定待 v8-objects-*)
- StringToBigInt/BigIntLiteral/BigIntLiteralToDecimal (待 v8-bigint)
- NumberToInt32/Uint32/ToSize/PositiveNumberToUint32 的 Tagged<Object> 族
  (double 分支语义已落地: double_to_int32/try_double_to_size 等)
- HashSeed::InitializeRoots (Isolate/ReadOnlyRoots), MathRandom 的
  Context/PodArray 写回 (数值核心 initialize_and_refill 已落地)
- ieee754::pow 的 use_std_math_pow / correctness_fuzzer flag 分支
  (待 v8-flags; legacy::pow 现委托平台 libm, 特判路径已精确)
- rapidhash_make_secret 派生与 V8_ENABLE_SEEDED_ARRAY_INDEX_HASH 的
  m1..m3 模逆组 (默认构建不编译, 配置翻转时补)
