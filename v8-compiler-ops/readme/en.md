# v8-compiler-ops

Rust transcription of V8 `src/compiler` operator system (Common / Machine / Simplified / JS operators + Verifier). Depends on v8-compiler-graph and v8-zone.

## Generator scripts (tool/src/compiler/, re-run = regenerate baseline, then cargo fmt)

| Script | Upstream source | Output | In-script assertions |
|---|---|---|---|
| `common-operator.h.js` | `common-operator.h/.cc` | `common_operator.rs` | static op table + parameterized op counts |
| `machine-operator.h.js` | `machine-operator.h/.cc` + `write-barrier-kind.h` | `machine_operator.rs` | 56 pure ops from PURE_BINARY_OP_LIST_32/64, field by field |
| `machine-operator.cc.js` | `machine-operator.cc:775-884` (Pair tail of MACHINE_PURE_OP_LIST) + `machine-operator.h:573-578` | `machine_pair_operator.rs` | cc entries 6 == h no-arg getters 6 == expected 6; props kNoProperties and counts (4/3, 0, 2) asserted per field, throws on mismatch |
| `simplified-operator.h.js` | `simplified-operator.h/.cc` | `simplified_operator.rs` | simplified op table |
| `js-operator.h.js` | `js-operator.h/.cc` | `js_operator.rs` | JS op macro counts (incl. FeedbackVector value input) |

## Module map

| Upstream | Rust module | Notes |
|---|---|---|
| `machine-operator.*` (PURE_BINARY_OP_LIST) | `machine_operator.rs` | global pure-op cache + Load/Store/Unaligned parameterized ops via zone |
| `machine-operator.*` (Pair binary tail entries, cc:879-884) | `machine_pair_operator.rs` | Int32Pair{Add,Sub,Mul} (4 value-in / 2 value-out), Word32Pair{Shl,Shr,Sar} (3/2); kPure; separate cache with `impl MachineOperatorBuilder` getters (module-split adaptation of the single C++ cache) |
| `verifier.h/.cc` (core subset) | `verifier.rs` | count-driven node/graph contracts; new table entries verified automatically |

## Tests (16/16 passing)

`machine_operator_unittest.rs` covers the 6 Pair table entries field by field, entry count == 6 and Pair-vs-single opcode discriminant mutual exclusion; `verifier_unittest.rs` covers Pair nodes with 4/3 inputs against total_input_count plus missing/extra edge rejection; other files cover common/simplified/js families.

## Ledger

- Deferred (evidence and ownership):
  - `StorePairRepresentation` + `TryStorePair` (h:192-212/1257, cc:971-1035/1487-1517/2207): the pair-representation table switches at compile time across V8_TARGET_ARCH_64_BIT x V8_COMPRESS_POINTERS (37/17/0 entries); this crate's builder has no Flags/arch input yet — landing a fixed table would be a fake fix. Blocked on MachineOperatorBuilder::Flags (kLoadStorePairs) infra.
  - mips64 `Lsa/Dlsa` instruction bodies and `kLsaSaBits`/`LsaSaValue` bitfields belong to arch crates (v8-codegen-mips64 / v8-compiler-backend in todo.md), not the operator tables.
  - Overlapping string copy length logic (e.g. `CodeStubAssembler::MoveElements`, codegen/code-stub-assembler.h:2490) belongs to v8-codegen-csa.
