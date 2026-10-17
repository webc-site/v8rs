# v8-ast (English)

Rust port of V8 `src/ast`, the L2 frontend layer on the M4 critical path (v8-base → v8-zone → **v8-ast** → v8-parsing). Upstream: 19 files / ~15.3k lines of C++.

See [中文](zh.md) for the complete file→module mapping table, X-macro expansion counts and phase-3 breakdown. Key points:

- C++ virtual dispatch through multiple inheritance → Rust tagged enums + `enum_dispatch`. `AstNode` (60 variants, ordered per `AST_NODE_LIST` + `FAILURE_NODE_LIST`) is the dynamic dispatch enum; `Statement` (22) / `Expression` (35) / `Declaration` (2) bridge enums mirror `AsStatement()/AsExpression()` downcasts.
- Conversion scripts under `tool/src/ast/` generate the `*_generated.rs` baselines (idempotent, hash-verified); `bun tool/src/ast/ast.h.js` etc. regenerate them.
- `AstStringConstant` table: 177 entries (128 one-char ASCII + 49 named) expanded from the upstream X-macro; node-type discriminants 0..=59 match upstream expansion order 1:1.
- Upstream pointer identities (`Scope*` / `Variable*`) map to arena handles (`ScopeId` / `VariableId` owned by `ScopeTree`); nodes reference scopes/variables via `Option<ScopeId>` / `Option<VariableId>` without `Rc<RefCell>` or unsafe.
- Gates: `cargo check` clean, clippy `--all-targets -D warnings` clean, `cargo fmt --check` clean, nextest 23/23.
