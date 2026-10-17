# v8-execution-isolate

Rust 1:1 port of V8's core execution isolation (`Isolate`) and execution state management.

## Core Components

1. **`Isolate`**:
   - Independent JavaScript execution context tied to a `ThreadId`.
   - Coordinates `StackGuard`, `Protectors`, `MicrotaskQueue`, and `VMState`.
   - Exception throw, catch, and pending status tracking.
   - Comprehensive execution limits checking (`check_execution_limits`).

2. **`Protectors`**:
   - 22 deoptimization protectors matching `v8/src/execution/protectors.h`.
   - 22-bit compressed bitmask state machine initialized to all valid (`ALL_VALID_MASK = 0x003F_FFFF`).
   - Fine-grained `is_intact` and `invalidate` methods.

3. **`StackGuard`**:
   - 12 interrupt causes and 3 interrupt levels (`NoGC`, `NoHeapWrites`, `AnyEffect`).
   - Atomic stack limits (`jslimit`, `real_jslimit`, `climit`, `real_climit`) and interrupt sentinel.
   - Stack overflow detection and thread-safe interrupt request/clear operations.

4. **`VMState` & `EncodedCSignature`**:
   - 11 VM execution states (`JS`, `GC`, `Parser`, `BytecodeCompiler`, `Compiler`, `External`, `Idle`, etc.).
   - `ExternalCallbackScope` RAII guard.
   - `EncodedCSignature` for C calling conventions.
   - Cross-platform `ThreadId`.

5. **`MicrotaskQueue`**:
   - Ring buffer microtask queue with nesting limit guard.
   - Supports `Auto`, `Scoped`, and `Explicit` microtask policies.

6. **`IsolateData` Layout**:
   - 80 primary fields and subfields `IsolateFieldId` matching V8 C++ layout.