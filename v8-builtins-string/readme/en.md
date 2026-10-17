# v8-builtins-string

V8 String builtins dispatch and metadata implementation.

## Architecture

1. **StringBuiltin Dispatch Table (`string.rs`)**:
   - Contains 26 core string builtin variants (`StringAddNoMapCheck`, `StringCompare`, `StringFromCharCode`, `StringPrototypeReplace`, etc.);
   - Strum two-way serialization support;
   - Semantic classification predicates:
     - `is_prototype_method`: String.prototype method classification;
     - `is_comparison`: Comparison builtins (including WASM);
     - `is_wasm_string`: WASM dedicated string fast path builtins;
     - `is_case_conversion`: Case conversion method builtins;
   - Bidirectional linkage to `v8-builtins-core` global dispatch table (`global_id`, `descriptor_name`).
2. **Safety & Quality Guarantees**:
   - `#![forbid(unsafe_code)]`;
   - Zero panic, zero bare unwrap/expect, zero dead code.