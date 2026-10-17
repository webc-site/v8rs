# v8-builtins-x64

Transpiled bindings for V8 x64 assembly builtins **entry surface** (names, enumerations, registration scaffolding). This crate does not contain assembly function bodies; "clean 3/3 freeze" in logs refers strictly to entry surface fidelity, not full assembly implementation. Deferral ledger and test mappings are recorded below (Wave 7 W7BXM remediation).

## Covered Surface (Entry Surface)

- `Builtin` enum with 73 items bijectively mapping to original `void Builtins::Generate_*` in `v8/src/builtins/x64/builtins-x64.cc:44..5741`, aligned identically with `ASM_GENERATE_ENTRIES`;
- 73 zero-cost `const fn` factory functions; `ArgumentsElementType`, `OsrSourceTier`, and `DebugBreakKind` direct enums; `CLASSES` with 3 class skeleton entries;
- `root_register()` bound to r13 (`v8_codegen_x64::R13`, code == 13, per `v8/src/codegen/x64/register-x64.h:387`);
- `global_id` linked via `v8-builtins-core` global table: 37 resolvable / 36 non-resolvable (identical resolvable subset with arm64 seat);
- Platform-specific entries `is_x64_specific` covering `{AdaptShadowStackForDeopt, DumpFrame}`;
- Top-level `#![forbid(unsafe_code)]`, 100% Safe Rust.

## Deferral Ledger (Fully Deferred to Subsequent Waves)

1. **Assembly Function Bodies**: 73 `Generate_*` MacroAssembler instruction sequences (rax-r15 register operations, rbp-rsp frame prologue/epilogue, branch alignment, RIP-relative constants) are not expanded;
2. **Frame Constants**: `v8/src/execution/x64/frame-constants-x64.h/.cc` deferred (note: `frame-inl-x64.h` does not exist in V8 tree);
3. **Execution / Simulator Semantics**: x64 execution requires x86-64 hardware or emulator; execution semantics belong to simulator wave;
4. **Register Allocation**: `RegisterConfiguration` traversal deferred to assembly body wave;
5. **Cc Registration Surface**: `CcBuiltin` and `CC_BUILTINS` are placeholders for deferred assembly registration;
6. **Boundary Statement**: This crate covers only entry names and dispatch metadata; assembly instruction bodies are deferred.

## Semantic Grouping Statement

- 6 categorization predicates (`is_js_entry_or_trampoline`, etc., partitioning 8+10+15+8+11+21=73 mutually exclusive and exhaustive) are heuristic classifications; ContinueTo* entries are grouped into `is_runtime_or_aux`;
- `expected_frame_type` is refined for 21 entries based on original frame macros; unrefined members use heuristic fallbacks.

## Test Mapping

Original V8 has no standalone `test/cctest/test-builtins-x64.cc`; builtins are indirectly verified via full test suites. Tests here provide regression fidelity:

- `tests/main.rs` (5) + `tests/builtins_x64_regression.rs` (4) (9 total): Zero heap allocation streaming iterators, compile-time constant slices, bijective mapping verification, exact 37/36 membership, refined frame types, root register verification;
- Functional execution belongs to milestone rs8.

## Dependency Ledger

- `thiserror` dependency removed following deletion of dead `error.rs`;
- Clippy: `cargo clippy -p v8-builtins-x64 --all-targets` passes with 0 warnings;
- Generation scripts: `tool/src/builtins/x64/builtins-x64.cc.js`, `tool/src/builtins/x64/builtins-x64.js`, and `tool/src/builtins/builtins-x64.js` updated and 100% idempotent.