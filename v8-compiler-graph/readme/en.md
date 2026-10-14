# v8-compiler-graph

Rust transcription of the V8 `src/compiler` TurboFan IR foundation layer (tier 1). The original 8-file family is ~4.3k lines of C++ (opcodes.h, operator.h/cc, operator-properties.h/cc, node.h/cc, node-properties.h/cc, turbofan-graph.h/cc, use-info.h); plus a registered subset of bytecode-graph-builder.h/cc (pure graph-building face). Rust side: ~5.6k lines in src + ~1.7k lines in tests, all produced as baselines by the custom generator scripts under `tool/src/compiler/`, then hand-revised in place.

Memory model: Node uses v8-zone bump allocation with a 1:1 Use-prefix layout (inputs inline after Node / overflow via OutOfLineInputs, Use doubly-linked list); mutable Node fields wrapped in `UnsafeCell` (legitimate writes behind a shared `&Node` view, mirroring the original const methods that mutate memory). Graph borrows `&'z Zone<'a>` (1:1 with the original `TFGraph(Zone*)`); node lifetimes match the zone.

## Generator scripts (tool/src/compiler/, re-run = regenerate baseline, then cargo fmt)

| Script | Upstream source | Output | In-script assertions |
|---|---|---|---|
| `opcodes.h.js` | `src/compiler/opcodes.h` + `opcodes.cc` + `operator-properties.cc` (predicate table) | `src/opcodes.rs` | Full X-macro tree expansion: **1036 total variants** (cross-checked against the C preprocessor `cpp` golden standard, all 13 group counts match: control=22 common=47 constant=13 inner=27 mconst=8 simplified=322 machine=234 simd128=287 js=124 atomic=27 mcmp=16 scmp=20 jscmp=6); target build wasm=on/SIMD128=on/SIMD256=off/hard-sandbox=off/CPED=on; variant uniqueness, range predicates ⟺ table bits fully consistent, NeedsExactContext JS-124 four mutually exclusive partitions 69/30/2/23, HasFrameStateInput true group 92 + parameterized 1 (JSCallRuntime) |
| `operator.js` | `src/compiler/operator.h` + `operator.cc` | `src/operator.rs` | Property 7 bit values + kFoldable/kEliminatable/kKontrol/kPure combinations bitwise identical; OPERATOR_PROPERTY_LIST print order |
| `operator-properties.js` | `src/compiler/operator-properties.h` + `.cc` | `src/operator_properties.rs` | 6 sentinels (case grouping / GetTotalInputCount formula verbatim); partition data guaranteed by opcodes.h.js |
| `node.js` | `src/compiler/node.h` + `node.cc` | `src/node.rs` | 11 layout sentinels (Use prefix stride / GetUsePtr reverse order / capacity floor 1 / slack +3 / OOL growth 2n+3 / bit-field widths) + 20-function transcription-face census; Rust const assertions Node=24B/Use=24B/OOL=16B, kOutlineMarker=15/kMaxInlineCapacity=14 |
| `node-properties.js` | `src/compiler/node-properties.h` + `.cc` | `src/node_properties.rs` | 10 layout formulas verbatim + 23-function transcription-face census |
| `turbofan-graph.js` | `src/compiler/turbofan-graph.h` + `.cc` | `src/graph.rs` | 8 sentinels (NextNodeId increment / factory delegation / SubgraphScope restore / NodeCount formula) |
| `use-info.js` | `src/compiler/use-info.h` + `representation-change.cc:24-145` (Truncation lattice) + `compiler/globals.h` (CheckForMinusZeroMode) + `codegen/machine-type.h` (minimal subset) + `feedback-source.h` (placeholder) | `src/use_info.rs` + `src/machine_type.rs` | Extraction order verified MachineRepresentation=21/MachineSemantic=11/TruncationKind=7/TypeCheckKind=13; 14 sentinels on the Truncation lattice |
| `bytecode-graph-builder.js` | `src/compiler/bytecode-graph-builder.h` + `.cc` (pure graph-building subset) | `src/build_graph.rs` | 12 mandatory presence anchors (entry `BuildGraphFromBytecode` declaration / `broker->target_native_context()` / CreateGraph head-tail SetStart·SetEnd paths / growth formula verbatim) + 10 exact-literal tallies (`native_context_node_` face: 6 occurrences; each buffer field: 1; `fill_n` + `buffer[count]=control` + `NewNode(phi_op,…,true)` Phi/EffectPhi isomorphic pairs: 2 each) |

## Module map

| Upstream (v8/src/compiler) | Rust module | Notes |
|---|---|---|
| `opcodes.h` + `opcodes.cc` | `src/opcodes.rs` | IrOpcode enum, 1036 variants (repr(u16), discriminant = original enum order); `OPCODE_PROPS` bit table, 21 bits (CONTROL/COMMON/…/EXACT_CONTEXT_*); range predicates is_common/is_control/is_js/is_machine + table-bit predicates; mnemonic via strum IntoStaticStr; `from_repr_unchecked` for hot paths (`Node::opcode`) |
| `operator.h` + `operator.cc` | `src/operator.rs` | Operator (opcode/Properties bits/mnemonic/port counts) + `OperatorCounts` collapsing constructor arguments (in-place revision) + Operator1<T> single-parameter template; ZeroIfEliminatable/NoThrow/Pure const fn; original CHECK/CheckRange → `Result` |
| `operator-properties.h` + `.cc` | `src/operator_properties.rs` | HasContextInput/HasFrameStateInput/GetTotalInputCount/IsBasicBlockBegin/NeedsExactContext; opcode-face predicates live in the IrOpcode bit table; parameterized branches (JSCallRuntime/JSCreateArguments) deferred with `Err(NeedsOpParam)` |
| `node.h` + `node.cc` | `src/node.rs` | Node/Use/OutOfLineInputs/Edge + InputEdges/Inputs/UseEdges/Uses iterators; zone bump + Use-prefix layout; AppendInput/ReplaceInput/RemoveInput/InsertInput(s)/TrimInputCount/EnsureInputCount/ReplaceUses/OwnedBy(2)/Kill/Clone/Verify(debug) |
| `node-properties.h` + `.cc` | `src/node_properties.rs` | first/past index family + input accessors + edge categories + replacer family + ReplaceUses/CollectControlProjections/FindFrameStateBefore/FindSuccessfulControlProjection/GetOuterContext/IsSame/Equals/HashCode |
| `turbofan-graph.h` + `.cc` | `src/graph.rs` | Graph (borrowed zone, Cell id allocator, decorators Vec<Rc<dyn GraphDecorator>> added/removed by data-pointer identity) + SubgraphScope RAII; mark_max retained + reserve_mark_domain arena rotation (NodeMarker deferred to tier 2) |
| `use-info.h` (+ `representation-change.cc` Truncation lattice) | `src/use_info.rs` | UseInfo/Truncation/IdentifyZeros/TypeCheckKind/CheckForMinusZeroMode; full lattice Generalize/LessGeneral migration (incompatible combinations are FATAL upstream → Err) |
| `codegen/machine-type.h` (minimal subset) | `src/machine_type.rs` | MachineRepresentation(21)/MachineSemantic(11)/MachineType + common constructors + PointerRepresentation (Word64 on 64-bit); full migration belongs to the codegen layer and will move out later |
| `bytecode-graph-builder.h` + `.cc` (pure graph-building face) | `src/build_graph.rs` | Landed: `InputBuffer::ensure` (EnsureInputBufferSize cc:4671, growth formula `required+64+old capacity`) / `create_start`·`create_end` (CreateGraph Start/End skeleton cc:1191-1205, non-empty DCHECK → Err) / `new_phi` (NewPhi cc:4798 and NewEffectPhi cc:4806 share one implementation, count inputs + trailing control, node created incomplete); `common()->Start/End/Phi/EffectPhi` operator construction belongs to v8-compiler-ops, injected as arguments per crate convention |
| — (handwritten) | `src/error.rs` / `src/lib.rs` | Original CHECK/FATAL/UNREACHABLE → centralized `Error` mapping; converged public exports |

Dependencies: v8-zone (bump allocation) / v8-base (BitField32, hash_combine) / strum (enums) / thiserror / log. dev: aok / ctor / log_init.

## Test map (82/82 passing)

| Original test file | Rust test file | Cases | Notes |
|---|---|---|---|
| `test/unittests/compiler/node-unittest.cc` | `tests/node_unittest.rs` | 23 | New/NewWithInputs/InputIterator×3/UseIterator×3/OwnedBy/ReplaceUsesNone/AppendInput/TrimThenAppend/BigNodes (512, out-of-line) all 1:1; plus RemoveInput/InsertInput(s)/ReplaceInput/EnsureInputCount/Kill-IsDead/BranchUseCount/Clone/zone reuse/Display/Operator1/Mark semantics |
| `test/unittests/compiler/opcodes-unittest.cc` | `tests/opcodes_unittest.rs` | 13 | IsCommon/IsControl/IsJs/IsConstant/IsComparison/Mnemonic full sweeps (table bits ⟺ range cross-check); plus totals/discriminants/build-config sampling/small predicates/FrameState 92 count/ExactContext partitions 69-30-2-23/top-level section counts/bit orthogonality |
| `test/unittests/compiler/node-properties-unittest.cc` | `tests/node_properties_unittest.rs` | 15 | ReplaceUses four paths + CollectControlProjections Branch/Call/Switch; plus layout index family/accessors/edge categories/replacer family/control projection/IsSame/Equals-Hash/OuterContext/FindFrameStateBefore/IsValueIdentity/predicate family |
| — (original tests need operator parameters/object layer) | `tests/operator_unittest.rs` | 12 | Properties bits and combinations/ZeroIf*/constructor counts and validation/props_str; OperatorProperties: context/frame_state/total(6-5-2)/basic_block_begin(11)/needs_exact_context four groups + Err face |
| — (no upstream unit tests) | `tests/graph_unittest.rs` | 9 | NewNode unique id/NodeCount/NewNodeUnchecked/decorator fire-remove (identity)/CloneNode/SubgraphScope RAII/simd face/Mark domain/zone access |
| — (no upstream unit tests) | `tests/use_info_unittest.rs` | 7 | Truncation lattice partial order/Generalize rules and Err/±0 identification/query face and description/UseInfo construction face/minus_zero_check/equality and hash/FeedbackSource placeholder/MachineType subset |
| — (original tests need broker/bytecode, not yet in place) | `tests/build_graph_unittest.rs` | 3 | Graph-build invariants: InputBuffer growth formula verbatim (10→74→218, zero reallocation when sufficient)/CreateGraph Start-End skeleton (contiguous ids/empty exit_controls Err consumes no id/input edge order/use chains each 1)/new_phi (count inputs + trailing control/use chains 3+1/buffer reuse without reallocation) |
| — (smoke) | `tests/main.rs` | 1 | smoke |

## Semantic deviation register

- Operator construction collapsed to `(opcode, properties, mnemonic, OperatorCounts)` (in-place revision; original takes 9 positional arguments); opcode-range validation [0, LAST_U16] is a Rust hardening (DCHECK only upstream).
- `Operator1<T>` uses composition instead of inheritance; the original unchecked `OpParameter<T>(op)` cast does not migrate — parameters are fetched directly through the strongly typed `&Operator1<T>`; ClassTag (DEBUG builds) does not migrate.
- Graph's zone is a borrow (`Graph::new(&zone)`); decorators are `Vec<Rc<dyn GraphDecorator>>` (original ZoneVector<GraphDecorator*>; Box cannot support identity-based removal); simd_stores/decorators use std Vec (self-referential zone containers are not viable).
- Original CHECK/DCHECK/FATAL/UNREACHABLE → `Result<_, Error>` (errors centralized in `src/error.rs`); debug_assert kept where purely assertive.
- The null-input FATAL branch of Node::New is eliminated by construction via `&Node` types (internal paths of the Clone/Insert family can still leave null slots; `input_at` returns Option).
- Node's `type_` field and Print(std::ostream) deep display deferred (turbofan-types layer).
- `InputBuffer`: original uses raw `Node**` slots and `EnsureInputBufferSize` returns the base address; the Rust side types slots as `&'z Node` (non-null guaranteed by the type), ensure returns no raw pointer, access via fill/set_at/view (view contract = the same caller discipline as the original "write first, then hand to NewNode"; out-of-range debug_assert).
- `new_phi`: the two original functions NewPhi/NewEffectPhi differ only at the operator construction site — merged into one, operator injected as argument (original `common()->Phi(kTagged,n)`/`EffectPhi(n)`); same for `create_start`/`create_end` (`common()->Start(arity)`/`End(n)` and the arity formula belong to v8-compiler-ops).
- CreateGraph's trailing DCHECK_NE(0u, exit_controls_.size()) → `Err(CountOutOfRange)` (crate CHECK-mapping convention).

## Deferred register (tier 2 / v8-compiler-ops)

- Hard-coupled face of the `bytecode-graph-builder.cc` graph-build body (the unit anchored by the ledger name NativeContext::BuildGraph) → **v8-compiler-frontend** (todo.md board ownership; needs JSHeapBroker/JSGraph/BytecodeAnalysis/interpreter bytecode face): `BytecodeGraphBuilder` constructor and the full `CreateGraph` chain (cc:1059-1210), `Environment`/dependency tracking (cc:552-1116), `MakeNode` context/frame-state/effect/control wiring and implicit exception continuations (cc:4707-4796), `VisitBytecodes`/`Visit*` bytecode handler family (cc:1473 onward), `ExitThenEnterExceptionHandlers` (cc:4679, HandlerTable), `CreateNativeContextNode`/`BuildLoadNativeContextField`/`CreateFeedbackVectorNode` (cc:1147-1166, jsgraph constant nodes need the broker), `GetParameter`/`cached_parameters_` (cc:1123-1145, needs ParameterInfo), `MergeControl`/`MergeEffect`/`MergeValue` (cc:4815-4870, needs common()->Loop/Merge/Phi parameterized operator reconstruction), OSR/peeled loops/`AdvanceIteratorsTo` (cc:1268-1471), node_origins/source_positions tables (decorator face).
- `common-operator.h` `StartNode::OutputArityForFormalParameterCount` (cc:775) and the `common()->Start/End/Phi/EffectPhi/Loop/Merge` constructors → v8-compiler-ops (this crate's `create_start`/`create_end`/`new_phi` already reserve the operator-as-argument call sites).
- `common-operator.h` / `machine-operator.h` / `simplified-operator.h` / `js-operator.h` → v8-compiler-ops crate (the opcodes.rs bit table is its foundation).
- `verifier.h` (VerifyNode checks inside NewNode), `turbofan-graph-visualizer` (Graph::Print/AsRPO).
- `node-marker.h` (NodeMarker/NodeMarkerBase, monotone domain takeover of mark_max), `node-cache.h` / `node-matchers.h` / `aux-data.h` / `schedule.h`.
- NodeProperties: FindProjection/CollectValueProjections (needs Operator1<size_t> parameters), MergeControlToEnd/RemoveControlFromEnd (needs CommonOperatorBuilder), GetProjectionType (needs full CallDescriptor/MachineType), InferMapsUnsafe/GetJSCreateMap/CanBePrimitive/CanBeNullOrUndefined (needs JSHeapBroker/MapInference), Type family (needs turbofan-types).
- OperatorProperties: JSCallRuntime branch (Runtime::NeedsExactContext / Linkage::NeedsFrameStateInput, needs the runtime-function table).
- Remaining parts of `representation-change.cc` (needs the simplified/machine operator layer); FeedbackSource object-layer upgrade (feedback-vector).
- Full migration of `codegen/machine-type.h` (ElementSize*/MachineReprToString etc., codegen layer).

Gates: clippy `--all-targets -D warnings` 0 findings; nextest 82/82.

Development log and ledger anchoring verdict: see ../../log/v8-compiler-graph.md
