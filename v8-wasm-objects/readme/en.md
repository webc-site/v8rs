# v8-wasm-objects

Structural ledger lives in `readme/zh.md` (Chinese): filled pure-logic surface with original
`v8/src/wasm/wasm-objects.cc` anchors, deferred registry (missing prerequisites and
unlocking crates) for all Isolate/heap-coupled skeletons, and test migration status.

A-1 sentinel: `wasm_objects_consts.rs`
`dispatch_table_layout::EXPOSED_TRUSTED_OBJECT_SIZE` is an explicit const-foldable
placeholder `0`. Hard constraint: its true `sizeof(ExposedTrustedObject)` value MUST be
substituted before the module is wired into `lib.rs` — never wire with the placeholder.
The true value is not directly provable from the original `wasm-objects.{h,tq}`
(tq:37 extern class has no body; h:973 only records the formula shape); it depends on
heap-object layout crates and pointer-compression/SANDBOX flags. The file stays outside
package gates until wired; a standalone `rustc` compile probe (currently 0 errors) is its
compile guard.
