# v8-regexp-compiler wave1 (pure-logic surface of v8/src/regexp compiler side)

Conversion script: `tool/src/regexp/regexp-bytecodes.h.js` (bun, idempotent,
generates `src/bytecodes_generated.rs`, prints cross-counts on every run:
61 opcodes = INVALID 1 + BASIC 35 + SPECIAL 18 + PEEPHOLE 7; 11 operand types;
4 flag bits; max operand count 20; total instruction sizes 732; cross-checked
against v8-regexp-exec source: Bytecode 61/61, layouts 61/61, widths 11/11).

## Landed (wave1)

| Module | Upstream | Content |
| --- | --- | --- |
| `bytecodes_generated.rs` | regexp-bytecodes.h, four tables | 61 opcodes, 11 operand types (size/alignment/limits/is-basic), 4 BytecodeFlag bits, per-opcode operand name/type tables, GOLDEN_SIZES baseline |
| `bytecodes.rs` | regexp-bytecodes-inl.h:107-152 | CalculateAlignedOffsets 4-byte-block layout (const-evaluated, per-opcode compile-time assert against GOLDEN_SIZES), IsValid/FromByte/Size/Flags, fixed-width LE operand read/write (endianness note: all target platforms little-endian) |
| `dump.rs` | regexp-bytecodes.cc:19-98 | char-exact AsHex/AsUC32/enum-name tables, disassemble(_single), walk, encode |
| `quick_check.rs` | regexp-compiler.h:78-146 + cc:2226-2275 | QuickCheckDetails/Position Clear/Advance/Merge/Rationalize (two-byte shift ≥4 is UB upstream; emulated with wrapping shift per x86/arm64 semantics, registered) |
| `boyer_moore.rs` | regexp-compiler.h:173-207 + cc:3730-3894 | ContainedInLattice/Combine/AddRange, BoyerMoorePositionInfo (bitset<128> → u128), lookahead container, FindWorthwhile/BestInterval with injected frequency callback (decouples from Compiler) |
| `frequency.rs` | regexp-compiler.h:544-587 | FrequencyCollator per-128 statistics |
| `dynamic_bitset.rs` | regexp-compiler.cc:429-464 | DynamicBitSet (32-bit bitmap + lazy dedup tail) |
| `range_split.rs` | regexp-compiler.h:755-777 + tonode.cc:210-264 + cc:2289-2305 | UnicodeRangeSplitter four-bucket five-segment split, Range(S)ContainsLatin1Equivalents |
| `node_data.rs` | regexp-nodes.h:34-139/262-289 + regexp-compiler.h:304-505/530-538 | FOR_EACH_NODE_TYPE 9-entry visitor table, NodeInfo interest propagation, EatsAtLeastInfo, PreloadState, ParkedGrant, Trace data plane (TriBool/FlushMode/DeferredActionUndoType/RegisterFlushInfo/bit fields/CP-offset limits) |
| `flags.rs` | regexp-compiler.h:63-74 | NeedsUnicodeCaseEquivalents, kMaxLookaheadForBoyerMoore=8, kPatternTooShortForBoyerMoore=2 |

CharacterRange canonicalization/merge already lives in the frozen
`v8-regexp-syntax::char_range`; this crate reuses it (homology registered).

## Deferred (wave2, waiting for v8-codegen-core to be green)

- RegExpAssembler / RegExpMacroAssembler-driven family: Compiler host,
  Emit/Visit dispatch, Generate.*, peephole handlers
- Trace::Flush / AdvanceCurrentPositionInTrace emit/abort channels and the
  ActionNode chain (mentions_reg/GetStoredPosition/ScanDeferredActions)
- The `%p` pointer column of RegExpBytecodeDisassemble (runtime addresses are
  nondeterministic; only the `%4x` offset column is kept here)
- BoyerMooreLookahead EmitSkipInstructions/BuildSkipTable/GetSkipTable
  (macro-assembler + Handle<ByteArray> coupling)
- Node IR host and parser wiring (BuildCharacterClassMap etc.)
- TypeCheck/Anchor tables: HEAD upstream no longer has these structures
  (ticket vocabulary lagged); the node table is carried by FOR_EACH_NODE_TYPE
  and landed in `node_data.rs`
