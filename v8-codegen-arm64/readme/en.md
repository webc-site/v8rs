# v8-codegen-arm64 — 1:1 port of V8 arm64 code generation

## Module mapping

| v8 original | generator script (tool/src/codegen/arm64/) | crate src |
| --- | --- | --- |
| src/codegen/arm64/constants-arm64.h | constants-arm64.h.js | src/constants.rs |
| src/codegen/arm64/register-arm64.h (+ src/codegen/register-base.h DEFINE_REGISTER_NAMES) | register-arm64.h.js | src/registers.rs |

Naming: C++ `kFoo` → Rust `K_FOO` (aligned with v8-common globals.rs); enum variant `kFoo` → `Foo`; NEON/SVE acronyms protected. Re-running a script overwrites manual edits; run `cargo fmt -p v8-codegen-arm64` after regeneration.

## Deviations

1. Ticket says `registers-arm64.cc`: this fork has no such file; it is `register-arm64.h` (data plane, ported) and `register-arm64.cc` (VectorFormat transforms, logic plane, deferred).
2. `TrampolineFlags` / `StaticLinks` / `kFlagsOffset`: removed from current upstream constants-arm64.h; nothing to align — recorded as deviation, not omission.
3. Legacy print name `zeroreg`: code 31 stringifies to `x31` here (DEFINE_REGISTER_NAMES); the simulator alias table spells it `xzr` (execution/arm64/simulator, different layer).
4. `RegisterName(sp)` internal code 63 is out of bounds: upstream reads Names[63] (UB); we add the same bounds guard as v8-codegen-x64 and return "invalid".
5. High-bit literals (e.g. 0x8000000000000000) are UB in C++; kept as exact two's-complement via `0x..._u64 as i64`.
6. al/nv exclusion in NegateCondition preserved as debug_assert (mirrors upstream DCHECK).
7. Dependencies: the wave1 data plane (constants/registers) is itself zero-coupled; the crate's current v8-codegen-core dependency was introduced by the concurrent assembler wave — wave1 added no third-party dependency.

## Deferred (wave2: gated on v8-codegen-core settling)

- macro-assembler-arm64 family (MacroAssembler / full Operand semantics / MemOperand / RelocInfo coupling)
- decoder-arm64 + diagnostics/arm64 disassembly goldens (host is arm64, native per-instruction comparison)
- register-arm64.cc VectorFormat transform family
- instruction classes Fixed/FMask/Mask enums and assembler-arm64.cc emission
- CPURegList and AreAliased/AreSameFormat/AreConsecutive predicates
- constant-pool-arm64 / simulator register name tables (belong to v8-execution-simulator)

## Encode kernel assessment: hand-written 1:1 vs asmkit (conclusion, for sign-off)

**Conclusion: hand-written 1:1; do not adopt asmkit.**

1. Audit chain: v8rs requires every value to be mechanically countable against upstream; asmkit's field descriptors are a non-1:1 remapping of INSTRUCTION_FIELDS_LIST, which would break the audit chain.
2. Dependency risk: asmkit is a third-party package; any drift invalidates the whole ledger. The codegen layer currently keeps a zero third-party dependency policy.
3. Cost: the encode kernel is just offset/width/mask triples plus insert/extract helpers; the data plane is already machine-generated, so hand-writing is cheap and guarded by 88 field-identity tests.
4. Landing: encode kernel stays hand-written in-crate, delivered with the decoder golden wave (wave2).

## Gates

- `bun tool/src/codegen/arm64/constants-arm64.h.js && bun tool/src/codegen/arm64/register-arm64.h.js && cargo fmt -p v8-codegen-arm64` — idempotent (product hashes unchanged)
- `cargo nextest run -p v8-codegen-arm64` — all green (10 tests: constants golden, name↔code roundtrip, aliases, calling convention + concurrent assembler tests)
- `cargo clippy -p v8-codegen-arm64 --all-targets -- -D warnings` RC=0; `cargo fmt --check` clean
