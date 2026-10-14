# v8-interpreter

Rust port of V8 Ignition bytecode interpreter core infrastructure.

## Key Features

- **Complete Bytecode Table (217 bytecodes)**: Mechanically aligned with V8 `bytecodes.{h,cc}`, including ShortStar (Star0..Star15), Wide / ExtraWide prefixes, DebugBreak series, and full operand metadata.
- **Register System**: Virtual registers (`Register`) and register lists (`RegisterList`), strictly matching frame layouts and parameter mappings (receiver / a0..an).
- **Zero-Copy Stream Decoding**: `BytecodeArrayIterator` for high-performance bytecode streaming with multi-level operand scaling (Single / Double / Quadruple).
- **Literal and Control Flags**: `CreateArrayLiteralFlags`, `CreateObjectLiteralFlags`, `CreateClosureFlags`, `TestTypeOfFlags`, `StoreLookupSlotFlags`.
- **100% Safe Rust**: Zero raw pointers, zero panics/unwraps, zero dead code, zero warning suppressions.