# v8-builtins-array

V8 Array builtins and GrowableFixedArray implementation.

## Architecture

1. **ArrayBuiltin Dispatch Table (`array.rs`)**:
   - Contains 33 core array builtin variants (`ArrayConcat`, `ArrayConstructor`, `ArrayPop`, `ArrayPrototypePush`, etc.);
   - Strum two-way serialization support;
   - Semantic classification predicates (`is_prototype_method`, `is_search_method`, `is_fast_clone_method`, `is_constructor`);
   - Bidirectional linkage to `v8-builtins-core` global dispatch table (`global_id`, `descriptor_name`);
   - 67 Torque macros and types declarations (`ARRAY_TQ_DECLS`).
2. **GrowableFixedArray (`growable.rs`)**:
   - 100% Safe Rust replication of V8's `GrowableFixedArray` (`growable-fixed-array-gen.h/cc`);
   - Exact growth rate formula: `new_capacity = current_capacity + (current_capacity >> 1) + 16`;
   - Safe `reserve`, `push`, `shrink_to_fit`, capacity bounds protection (`MAX_FIXED_ARRAY_CAPACITY = 512M`).
3. **Safety & Quality Guarantees**:
   - `#![forbid(unsafe_code)]`;
   - Zero panic, zero bare unwrap/expect, zero dead code.