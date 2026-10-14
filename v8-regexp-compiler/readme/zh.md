# v8-regexp-compiler wave1 (v8/src/regexp 编译器侧纯逻辑面)

转换脚本: `tool/src/regexp/regexp-bytecodes.h.js` (bun, 幂等, 生成
`src/bytecodes_generated.rs`, 运行即对数: opcode 61 = INVALID 1 + BASIC 35 +
SPECIAL 18 + PEEPHOLE 7; 操作数类型 11; flag 位 4; 操作数上限 20; 尺寸合计
732; 与 exec 交叉对数 Bytecode 61/61、布局 61/61、宽度 11/11)。

## 已落地 (wave1)

| 模块 | 原版对应 | 内容 |
| --- | --- | --- |
| `bytecodes_generated.rs` | regexp-bytecodes.h 四表 | 61 opcode、11 操作数类型 (宽/对齐/限幅/IsBasic)、BytecodeFlag 4 位、操作数名/类型表、GOLDEN_SIZES 基线 |
| `bytecodes.rs` | regexp-bytecodes-inl.h:107-152 | CalculateAlignedOffsets 4 字节块排布 (const 计算 + 编译期逐项 assert GOLDEN_SIZES)、IsValid/FromByte/Size/Flags、Operand 定宽 LE 读写 (端序登记: 目标平台均小端) |
| `dump.rs` | regexp-bytecodes.cc:19-98 | AsHex/AsUC32/枚举名表逐字符格式、disassemble(_single)、walk 布局行走、encode 组装 |
| `quick_check.rs` | regexp-compiler.h:78-146 + cc:2226-2275 | QuickCheckDetails/Position 的 Clear/Advance/Merge/Rationalize (two-byte ≥4 位移位原版 UB, 按 x86/arm64 实际语义 wrapping 承接登记) |
| `boyer_moore.rs` | regexp-compiler.h:173-207 + cc:3730-3894 | ContainedInLattice/Combine/AddRange 格律、BoyerMoorePositionInfo (bitset<128>→u128)、Lookahead 容器面、FindWorthwhile/BestInterval (频率回调注入解耦 Compiler) |
| `frequency.rs` | regexp-compiler.h:544-587 | FrequencyCollator per-128 统计 |
| `dynamic_bitset.rs` | regexp-compiler.cc:429-464 | DynamicBitSet (kFirstLimit=32 位图 + 惰性去重表) |
| `range_split.rs` | regexp-compiler.h:755-777 + tonode.cc:210-264 + cc:2289-2305 | UnicodeRangeSplitter 四桶五段分类、Range(S)ContainsLatin1Equivalents |
| `node_data.rs` | regexp-nodes.h:34-139/262-289 + regexp-compiler.h:304-505/530-538 | FOR_EACH_NODE_TYPE 9 项访问表、NodeInfo 位兴趣传播、EatsAtLeastInfo、PreloadState、ParkedGrant、Trace 数据面 (TriBool/FlushMode/DeferredActionUndoType/RegisterFlushInfo/位域组/CP 界限) |
| `flags.rs` | regexp-compiler.h:63-74 | NeedsUnicodeCaseEquivalents、kMaxLookaheadForBoyerMoore=8、kPatternTooShortForBoyerMoore=2 |

CharacterRange 合并 (canonicalize) 已在冻结的 `v8-regexp-syntax::char_range`
落地, 本 crate 直接复用 (同源性登记)。

## 推迟 (二梯队, 待 v8-codegen-core 绿)

- RegExpAssembler / RegExpMacroAssembler 驱动面全族: Compiler 宿主、
  Emit/Visit 双分派、Generate.*、Peephole 处理器
- Trace::Flush / AdvanceCurrentPositionInTrace 的 emit/abort 通道与
  ActionNode 链表宿主 (mentions_reg/GetStoredPosition/ScanDeferredActions)
- RegExpBytecodeDisassemble 的 `%p` 指针列 (运行期地址非确定性, 本落地仅保
  `%4x` 偏移列)
- BoyerMooreLookahead 的 EmitSkipInstructions/BuildSkipTable/GetSkipTable
  (宏汇编器 + Handle<ByteArray> 耦合)
- Node IR 宿主与 Parser 接线 (BuildCharacterClassMap 等)
- TypeCheck/Anchor 表: HEAD 原版已无同名结构 (票面词汇滞后), 节点访问表以
  FOR_EACH_NODE_TYPE 承接并落地于 `node_data.rs`

## 同源性与依赖白名单

- 仅依赖 v8-base / v8-regexp-syntax (冻结) + thiserror。
- 禁止依赖 v8-regexp-exec (在飞面): 其 `regexp_interpreter.rs` 的 Bytecode
  枚举/BYTECODE_LAYOUTS/OperandType 宽度表由脚本 (生成期) 与单测 (运行期读
  源文本) 双通道逐项交叉对数, 不 link。
