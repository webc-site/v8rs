# v8-diagnostics

Rust transcription of `v8/src/diagnostics` — wave1 zero-coupling subset (platform-independent pure logic).
Data surface is generated from upstream C++ snapshots by `tool/src/diagnostics/*.js` (`*_data.rs`, repeat runs byte-identical, throws on missing needle); behavior surface is hand-written 1:1.

## Module map

| v8 source | crate module | generator (needles) | notes |
|---|---|---|---|
| `src/diagnostics/eh-frame.{h,cc}` + `x64/arm64/eh-frame-<arch>.cc` | `src/eh_frame_data.rs` | `eh-frame.js` (45) | DwarfOpcodes 10 + encoding specifiers 5 + tag/mask/layout constants + x64/arm64 arch params (alignment × RA pseudo-reg code × CIE initial state) |
| same (behavior) | `src/eh_frame.rs` | — | `EhFrameWriter` (CIE/FDE/.eh_frame_hdr) + `EhFrameIterator` (read-back) + ULEB/SLEB codec |
| `src/diagnostics/gdb-jit.cc` | `src/gdb_jit_data.rs` | `gdb-jit.js` (25) | 7 struct layout tables (ELFHeader/ELFSectionHeader/MachO*/JITCodeEntry/JITDescriptor, 64-bit) + 4 enum groups + ident/needles |
| same (behavior) | `src/gdb_jit.rs` | — | `Writer` buffer primitives + `ElfImage`/`MachOImage` assembly + `GdbJitRegistry` linked list |
| `src/diagnostics/perf-jit.{h,cc}` | `src/perf_jit_data.rs` | `perf-jit.js` (14) | 6 record structs (repr(C) POD) + 5 events + 9 ELF machines + header/buffer constants |
| same (behavior) | `src/perf_jit.rs` | — | `header_bytes`/`code_load_bytes`/`debug_info_bytes`/`unwinding_bytes` serialization (LE, byte-identical to C++ memory layout) |
| `src/diagnostics/objects-printer.cc` (anon ns) | `src/objects_print_data.rs` | `objects-printer.js` (12) | PrintDouble needles + safe-integer bounds + HelperState/ZipMode name tables |
| same (behavior) | `src/objects_print.rs` | — | `print_double`/`double_to_integer`/`is_minus_zero` + enum ToString |
| `src/diagnostics/basic-block-profiler.{h,cc}` | `src/basic_block_profiler.rs` | `basic-block-profiler.js` (10) | `BasicBlockProfilerData` (reverse post-order block count/branches/schedule/log/Display) + `BasicBlockProfiler` (singleton/reset/coverage bitmap/lock-free harvest) + `BuiltinsCallGraph` (call graph map) |
| `src/diagnostics/compilation-statistics.{h,cc}` | `src/compilation_statistics.rs` | `compilation-statistics.js` (9) | `BasicStats` (time delta µs/space/graph size/accumulate/JSON) + `CompilationStatistics` (phases/totals) + `printable_statistics` (human double-line/machine format) |
| `src/diagnostics/objects-debug.cc` | `src/objects_debug_generated.rs` | `objects-debug.cc.js` (3) | `VERIFY_REGISTRY` 259 `<Class>::<Class>Verify` validator registry table |
| — | `src/error.rs` | — | `Error`/`Result` placeholder (no wave1 error sites; variants land with the IO-coupled wave) |

## Test map (tests/, 39 cases)

| file | cases | anchors |
|---|---|---|
| `tests/eh_frame.rs` | 12 | x64/arm64 CIE goldens (28 B) + full 72 B stream golden + iterator round-trip (±624485 LEB cases aligned with upstream `eh-frame-iterator-unittest`) + 5th-byte bit4-6 uleb overflow-rejection branch + advance_location widths + rule directives + 2 handmade skip_cie byte tests |
| `tests/gdb_jit.rs` | 8 | Writer primitives (LE writes/truncation/align/LEB) + ELF64 header & section-table patch-back + MachO64 sizeofcmds/fileoff + registry head-insert/unlink/destroy guard + 3 reentrancy migrations (linked flag) + body-section addr-zero regression |
| `tests/perf_jit.rs` | 5 | 4 record types field-by-field offsets + 8-byte zero padding + empty-unwinding .eh_frame_hdr hack |
| `tests/objects_print.rs` | 4 | PrintDouble 4 branches + DoubleToInteger branch table + name tables + print_double_manual_six |
| `tests/basic_block_profiler.rs` | 4 | ProfilerData lifecycle/Log/Display descending print + coverage bitmap/single-pass reset harvest + call graph lookup + Verify registry 259 anchor assertions |
| `tests/compilation_statistics.rs` | 5 | BasicStats saturating accumulation + JSON serialization + phases and totals aggregation + human double-line/machine format output + zero time/space/graph edge safety |
| `tests/main.rs` | 1 | crate smoke (real assertion via exported decode_uleb128) |

## Concurrent-seat orphan files (plane yielded to its owning seat)

`src/` contains 12 files NOT wired into `lib.rs`'s module graph (invisible to all three gates): disasm.rs, disassembler.rs, code_tracer.rs, unwinder.rs, unwinding_info_win64.rs, 6× etw_*, gdb_jit_h_generated.rs. All 8 `todo!("待 Developer 修订")` placeholders in the crate concentrate in 4 of them: disasm 3, code_tracer 3, unwinder 1, disassembler 1; the remaining orphans have been landed by their owning seat as todo!-free data planes. They are another seat's in-flight content, not this ticket's delivery; deletion/repair is out of scope here. This registration removes the "unregistered" defect; the owning seat must land or remove them before its own freeze.

## Deferred ledger (not in wave1, with blockers)

| surface | blocker |
|---|---|
| eh_frame: `Label`/`RelocInfo`/`CodeDesc` patch-back (`GetEhFrame`/`EmitExternalCodeInfo`) | v8-codegen-core (not ready) |
| gdb_jit: DWARF debug sections (`.debug_*`/CFI, WriteDebugInfo/WriteLineInfo) | CodeDescription/RelocInfo coupling (codegen-core + L3 objects) |
| gdb_jit: ELF symbol table (`CreateSymbolsTable`/`ELFSymbol`/`ELFSymbolTable`, .symtab+.strtab, gdb-jit.cc:721-869,1015-1035) | depends on CodeDescription (desc->name/CodeSize) |
| gdb_jit: ia32 32-bit branch | V8 ia32 frozen; 64-bit only |
| objects_print: `ObjectImpl::Print` and HeapObject body printing | Tagged/Isolate coupling (L3 objects) |
| objects_print: PrintDouble default branch | intentional deviation: Rust Display shortest round-trip vs C++ ostream %g 6 significant digits; not blocking, not reconciled |
| perf_jit: Isolate timestamps/PID/TID, `jit-<pid>.dump` file IO, `LogClose` | Isolate/filesystem coupling; inputs already injected by caller |
| eh_frame/gdb_jit: real CodeStubAssembler fill chain | same as codegen-core |
| basic_block_profiler: `CopyToJSHeap` / `OnHeapBasicBlockProfilerData` | JS heap object allocation coupled with Isolate (returns 0 safely in pure logic plane) |
| objects_debug: each `Verify` method body (~4300 lines of invariant checks) | depends on each object's complete internal layout fields and Isolate runtime context |

## Re-run & gates

- Scripts: `cd tool && bun src/diagnostics/<name>.js` (pure text scan, fixed-order output, no timestamps; regenerating overwrites in-place revisions)
- Gates: `cargo nextest run -p v8-diagnostics --all-features` / `cargo clippy -p v8-diagnostics --all-targets --all-features -- -D warnings` / `cargo +nightly fmt -p v8-diagnostics -- --check`

(Development log and review-round ledger: see ../../log/v8-diagnostics.md)
