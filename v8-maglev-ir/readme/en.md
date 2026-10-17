# v8-maglev-ir · wave1 ledger

Rust 1:1 replica of the `v8/src/maglev/` IR data plane (opcodes / bitfields / node types / block flags). wave1 = pure data plane, zero graph coupling.

## wave1 delivered

| Unit | C++ source | Product | Script |
|---|---|---|---|
| Full opcode table + range predicates | maglev-ir.h NODE_BASE_LIST X-macro (ir.h:551-553; the old OPCODE_LIST name does not exist, review R5M6 N2) (anchors :193/230/274/425/508/512-550) | `src/opcodes.rs` (1512 lines) | `tool/src/maglev/maglev-ir-opcodes.js` |
| OpProperties + NodeBase bitfields | maglev-ir.h:1163-1373, :1350-1356 (AttachedDeoptInfo), :921-930, :2033-2051 | `src/op_properties.rs` | `tool/src/maglev/maglev-ir-props.js` |
| NodeType powerset | maglev-node-type.h:20-41 (18 leaves, 1<<i) / :44 / :49 | `src/node_type.rs` | `tool/src/maglev/maglev-node-type.js` |
| BasicBlock flags | maglev-basic-block.h:394-397 (three 1-bit flags) | `src/basic_block.rs` | `tool/src/maglev/maglev-basic-block.js` |
| Golden tests | expected values transcribed independently from C++ headers | `tests/{properties,opcodes,node_type,basic_block,main}.rs` (23 cases; r4 added from_bits_truncates_to_16) | — |

## Key counts

- OPCODE_COUNT = 378 (h:560; h:562 is kLastOpcode); control region 24 = 4+17+3 (354..377 derived independently)
- OpProperties full shift/width/mask table; NodeBase `[opcode:16][input_count:16][properties:16][extras:16]`
- NodeType 18 leaves with 1<<i identity + 20 composites by expression evaluation

## Deferred (wave2+)

- graph.h / bytecode frontend wiring, Node host classes, typer tables (needs v8-zone/codegen-core coupling)
- `.js` planning scripts under tool/src/maglev/ produce `.skel` scaffolding files landing in v8-maglev-ir/-frontend/-backend `src/` (not under tool/, review R5M6 F3), belonging to later waves
- BasicBlockPredecessorInfo absent in this snapshot — registered in script notes

## Gates

- `bun tool/src/maglev/{maglev-ir-opcodes,maglev-ir-props,maglev-node-type,maglev-basic-block}.js` run twice → byte-identical products (writeSrc has no built-in rustfmt; re-run `cargo +nightly fmt -p v8-maglev-ir` afterwards to restore the fixed point)
- `cargo nextest run -p v8-maglev-ir --all-features` all green (23 cases)
- `cargo clippy -p v8-maglev-ir --all-targets --all-features -- -D warnings` RC=0
- `cargo +nightly fmt -p v8-maglev-ir -- --check` clean

---

Development log (review-round records / draft-correction history) lives in ../../log/v8-maglev-ir.md (Chinese only).
