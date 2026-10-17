# v8-codegen-csa

CodeStubAssembler (CSA) and typed SSA node (TNode) graph construction DSL.

## Architecture

- `tnode`: Typed SSA node wrapper `TNode<T>` and type tag system (machine representations and JS object types).
- `code_assembler`: Core graph assembler and control flow state machine (31 comparison ops, 65 binary ops, 71 unary ops, labels, variables, and assembler state).
- `code_stub_assembler`: High-level CodeStubAssembler DSL covering Smi operations, Map loading, type predicates, and heap allocation.
- `error`: Strongly-typed error enum and Result alias.

## Quality Standards

- 100% Safe Rust (`#![forbid(unsafe_code)]`)
- Zero panics, zero unwrap/expect
- Flat export structure without re-exporting ambiguity
- 6/6 tests passing, 0 Clippy warnings