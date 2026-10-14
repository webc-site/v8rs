# v8-handles (English)

Rust transpilation of the `v8/src/handles` core family (11 files, ~2.5k lines):
Handle/DirectHandle/HandleScope/SealHandleScope, LocalHandles/LocalHandleScope,
MaybeHandle/MaybeDirectHandle/MaybeObjectHandle, PersistentHandles (+List/Scope),
SharedObjectConveyorHandles.

No real GC heap: storage is a testable chunked-address-array — a thread_local `HandleArena`
(blocks + spare + HandleScopeData) mirroring the original isolate->handle_scope_data() and
HandleScopeImplementer blocks/spare layout. next/limit slot-pointer bumping, Enter/Leave
chained scopes and DeleteExtensions block recycling (with spare reuse) are 1:1; a block is
`[Address; KB-2]`.

## Group → file mapping

| v8 sources (lines) | Rust module | script |
|---|---|---|
| handles.h + handles.cc + handles-inl.h (1772) | src/handles.rs | tool/src/handles/handles.js |
| local-handles.h + local-handles-inl.h + local-handles.cc (337) | src/local_handles.rs | tool/src/handles/local-handles.js |
| maybe-handles.h + maybe-handles-inl.h (671) | src/maybe_handles.rs | tool/src/handles/maybe-handles.js |
| persistent-handles.h + persistent-handles.cc (334) | src/persistent_handles.rs | tool/src/handles/persistent-handles.js |
| shared-object-conveyor-handles.h + .cc (83) | src/shared_object_conveyor_handles.rs | tool/src/handles/shared-object-conveyor-handles.js |

Deferred (not transpiled): global-handles.* (Isolate/Heap bound), traced-handles.* (cppgc
bound), handle-scope-implementer.* (thread archive + context stacks bound; its chunking face
is inlined into HandleArena and will be merged once that family lands).

## Verification

Constant/enum/API-surface assertions live in the generation scripts (audit ledger in
log/v8-handles.md); the two zap constants are compile-time asserted against v8_common
(single source of truth). Rust-idiom deviations (bool+out → Option, CHECK → Result,
RAII Enter/Leave, phantom-typed Tagged) are documented in the Chinese readme. 54 semantic
tests cover the listed guarantees.
