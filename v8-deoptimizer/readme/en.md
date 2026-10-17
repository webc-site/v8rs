# v8-deoptimizer

Rust port of V8 deoptimization subsystem and translated state infrastructure.

## Key Features

- **Deoptimization Reasons & Kinds**:
  - `DeoptimizeReason` (68 entries): Full coverage of deopt reasons including bounds check, division by zero, missing feedback, migration failures.
  - `LazyDeoptimizeReason` (17 entries): Dedicated lazy deoptimization reasons.
  - `DeoptimizeKind`: `Eager`, `LazyAfterFastCall`, `Lazy`.
- **Translation Bytecode Opcodes (`TranslationOpcode`)**:
  - 47 entries covering JS frames (4), general frames (11), and data translation items (36).
  - Operand capacity up to 6 with frame/begin predicates.
- **Deoptimized Frame & Materialized Store**:
  - `DeoptimizedFrameInfo`: Snapshot parameters and expression stack information.
  - `MaterializedObjectStore`: Frame materialized objects store.
  - Architecture exit constants: x64 and arm64 eager/lazy exit sequence sizes.
- **100% Safe Rust**:
  - Zero unsafe, zero panics, zero naked unwraps/expects, zero dead code, zero warning suppressions.