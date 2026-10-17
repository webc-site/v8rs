# v8-bigint

Rust port of v8/src/bigint (self-contained, no v8-base dependency). Modules map 1:1 to upstream files. See [中文文档](zh.md) for full tables.

## Modules

| v8/src/bigint | v8-bigint/src | note |
|---|---|---|
| bigint.h + bigint-inl.h | bigint.rs | digit primitives, fast ops (add/sub/bitwise/shift/truncate), result lengths |
| bigint.h:271-275 + builtins-bigint.tq:454 BigIntBitwiseAndImpl | bitwise.rs | BitwiseAnd top-level orchestration: sign dispatch, result-length budget (min / max+1 / positive-side), PosNeg argument swap, Canonicalize→norm; heap-free |

## BitwiseAnd unit

`bitwise.rs` provides the top-level orchestration over the digit kernels (zero shortcuts, four sign branches, result-length budget min / max+1 / positive-side, NegPos argument swap, Canonicalize→norm), heap-free; script `tool/src/bigint/bitwise-and.js` asserts 121 structural anchors; tests in `tests/bitwise_and.rs` (6 fns, see [中文](zh.md) table). Deferred (heap-coupled, out of this crate's boundary, explicitly not stubbed): objects/bigint.cc:1723-1757 MutableBigInt_BitwiseAnd*AndCanonicalize wrappers (L3 objects), tq AllocateEmptyBigIntNoThrow / BigIntTooBig throw surface (L4 builtins).

## Test coverage mapping

Upstream test source: v8/test/bigint/bigint-shell.cc (9 Test entries, differential assertions between two algorithms).

| Upstream test | Rust test (tests/bigint_shell.rs) | Assertion (strength preserved) |
|---|---|---|
| TestKaratsuba | karatsuba | karatsuba vs schoolbook, 34..=102 full sweep (2415 cases) |
| TestToom | toom | toom_cook vs karatsuba, 150..=220 |
| TestFFT | fft | fft vs toom_cook, seeded random sampling |
| TestBurnikel | burnikel | burnikel vs schoolbook, quotient+remainder |
| TestBarrett | barrett | barrett vs burnikel + one random 13000+ digit case |
| TestCachedMod | cachedmod | cached_mod vs modulo_small/modulo_large |
| TestToString | tostring | fast vs classic, size 11..46 × radix 2..=36 |
| TestFromString | fromstring | parse_large vs parse_classic |
| TestFromStringBase2 | fromstring_base2 | ToString→Parse→parse roundtrip |

Skipped: ThresholdRunner (benchmarking, no assertions); unittests/numbers/bigint-unittest.cc (belongs to src/objects/bigint, needs isolate/heap); mjsunit bigint tests (engine-level) — except harmony/bigint/and.js and compiler/bigint-bitwise-and.js numeric vectors, migrated as BitwiseAnd goldens (see above). This checkout has no v8/test/unittests/bigint/ directory; v8-common/tests/globals.rs holds no BigInt golden tests referencing this crate (verified).

## Conversion scripts

| Script (tool/src/bigint/) | Output | Nature |
|---|---|---|
| gen_consts.js | from_string.rs / to_string.rs baseline (kCharValue / kCharBits / kMaxBitsPerChar) | baseline generator; semantic bodies revised in place — do not re-run |
| util.js | util.rs whole file | pure-rule file, script-owned, idempotent |
| bitwise-and.js | bitwise.rs.skel | 121 structural-anchor assertions; read-only cross-check of bigint.h/bigint-inl.h/bigint.tq/bigint.cc + and.js goldens vs landed code; idempotent |

All remaining upstream files are line-level semantic ports (not rule code), hence unscripted per todo.md ch.11. Function-level coverage: all 16 upstream files ported 1:1, no gaps.

## Gate

```
CARGO_TARGET_DIR=target/bigint1 cargo clippy -p v8-bigint --all-features -- -D warnings
CARGO_TARGET_DIR=target/bigint1 cargo nextest run -p v8-bigint --all-features
```
