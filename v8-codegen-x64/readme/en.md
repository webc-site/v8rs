# v8-codegen-x64 · wave1 ledger

Rust 1:1 replica of `v8/src/codegen/x64/`. wave1 = zero-coupling pure data plane.

## wave1 delivered

| Unit | C++ source | Product | Script |
|---|---|---|---|
| Full constant table | `src/codegen/x64/constants-x64.h` | `src/constants.rs` | `tool/src/codegen/x64/constants-x64.h.js` |
| Register data plane | `src/codegen/x64/register-x64.h` + `src/codegen/register-base.h` | `src/registers.rs` | `tool/src/codegen/x64/register-x64.h.js` |
| Golden tests | same headers (expected values transcribed independently from C++) | `tests/x64_data.rs` (13 cases; crate total 21/21 — the other 8 belong to a concurrent seat's `tests/main.rs`) | — |

Ticket-vs-snapshot drift notes:

- `constants-x64.h` in this snapshot holds exactly 2 constants (`kRootRegisterBias=128`, `kMaxPCRelativeCodeRangeInMB=2048`).
- `OperandSize` / `TrampolineFlags` named in the ticket do not exist anywhere in this V8 tree for x64; nothing invented.
- `kNoReg` is really `RegisterBase::kCode_no_reg = -1` (register-base.h) → mirrored as `K_CODE_NO_REG`.
- `registers-x64.cc/.h` is actually `register-x64.h` here; name tables live in-header via the `DEFINE_REGISTER_NAMES` X-macro.

## Script counts

`constants-x64.h.js` — constants 2/2, per-value + per-type golden check, naming-map asserts, idempotent reruns (output passes through nightly rustfmt fixed point).

`register-x64.h.js` — 34 anchors plus full sequence checks:

- GENERAL_REGISTERS 16; ALWAYS_ALLOCATABLE 11; MAYBE_ALLOCATABLE both branches 0/1 (r14)
- XMM base/ext 16/16; YMM base/ext 16/16; ALLOCATABLE_DOUBLE 15
- Register calling-convention aliases 24; Double/YMM aliases 3; kPtrComprCageBaseRegister branches 2
- kCArgRegs WIN/SYSV = 4/6; callee-save GPR WIN/SYSV = 7/5; callee-save FP WIN/SYSV = 10/0
- Scalars kNumRegs=16 / kWindowsHomeStackSlots=4 / kSimdMaskRegisters=false / kCode_no_reg=-1
- 7 header-aligned static assertions (1:1 with header `static_assert`s: sizeof<=int ×3, kAfterLast<=int8, kDoubleAfterLast==kYMMAfterLast both branches, cross-table length invariants); full product contains 19 `const _:` in total (the other 12 are script-added naming-map/inverse invariants)

## Deferred (wave2 / second tier)

- **macro-assembler-x64 family** (`macro-assembler-x64.{h,cc}`, `assembler-x64.{h,cc}`, `assembler-x64-inl.h`): blocked on green codegen-core + second-tier plan. `src/{assembler,operand}.rs`, the `v8_codegen_core` dep in `error.rs`/`Cargo.toml`, and `deferred/wave2_assembler_tests.rs` are concurrent-seat in-flight work, not wave1 scope.
- **diagnostics/x64 disassembler plane**: wave2.
- **asmkit kernel evaluation items**: `reglist-x64.h` (MacroAssembler-coupled methods, FormatValue/Print), `interface-descriptors-x64-inl.h`, `fma-instr.h`/`sse-instr.h` opcode enums, `cpu-x64.cc` — decision left to wave2.
- **Config switches**: APX_F `high_bit` masked branch already taken (numerically equal to the cc:72 non-masked branch on legal domain 0-15, see review round 1 D4); AVX10 (`K_*_AFTER_LAST_AVX10` materialized as consts, `from_code` bounds follow default build); `ArgumentPaddingSlots` build flag taken as function parameter (`V8_X64_16BYTE_STACK_ALIGNMENT_BOOL`); `kFPAliasing` (AliasingKind belongs to platform-neutral base, hook up after codegen-core lands); `ReassignRegister` generic helper (needs trait design).
- **tests/**: dynamic halves of `v8/test/unittests` register/disasm cases deferred; wave1 keeps only statically verifiable golden tables.
