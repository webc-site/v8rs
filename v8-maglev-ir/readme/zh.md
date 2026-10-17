# v8-maglev-ir · wave1 台账

Rust 1:1 复刻 `v8/src/maglev/` IR 数据面 (opcode/位域/节点类型/块旗)。wave1 = 零图耦合纯数据面。

## wave1 完成清单

| 单元 | C++ 源 | 产物 | 脚本 |
|---|---|---|---|
| Opcode 全表 + 区间谓词 | maglev-ir.h NODE_BASE_LIST X-macro (ir.h:551-553; 旧书 OPCODE_LIST 系不存在的宏名, 复审 R5M6 N2) (锚 :193/230/274/425/508/512-550) | `src/opcodes.rs` (1512 行) | `tool/src/maglev/maglev-ir-opcodes.js` |
| OpProperties + NodeBase 位段 | maglev-ir.h:1163-1373, :1350-1356 (AttachedDeoptInfo), :921-930, :2033-2051 | `src/op_properties.rs` | `tool/src/maglev/maglev-ir-props.js` |
| NodeType 幂集 | maglev-node-type.h:20-41 (叶子 18 项 1<<i) /:44/:49 | `src/node_type.rs` | `tool/src/maglev/maglev-node-type.js` |
| BasicBlock 旗位 | maglev-basic-block.h:394-397 (3 个 1-bit 旗) | `src/basic_block.rs` | `tool/src/maglev/maglev-basic-block.js` |
| 金标准测试 | 期望值独立抄自 C++ 头 | `tests/{properties,opcodes,node_type,basic_block,main}.rs` (23 例; r4 增 from_bits_truncates_to_16) | — |

## 关键对数

- OPCODE_COUNT = 378 (h:560; h:562 系 kLastOpcode); 控制区 24 = 4+17+3 (354..377 独立推得)
- OpProperties 位域 shift/width/mask 全表; NodeBase `[opcode:16][input_count:16][properties:16][extras:16]`
- NodeType 叶子 18 项 1<<i 恒等 + 组合 20 项表达式求值

## 推迟清单 (wave2+)

- graph.h / bytecode 前端接线、Node 宿主类、typer 表 (依赖 v8-zone/codegen-core 联动)
- tool/src/maglev/ 下 .js 规划脚本 (graph/nodes/opcodes/repr/frontend/regalloc/codegen 等) 所产 .skel 脚手架件分落 v8-maglev-ir/-frontend/-backend 的 src/ (非 tool/ 目录, 复审 R5M6 F3), 属后续波次
- BasicBlockPredecessorInfo 本快照缺失已登脚本注

## 门禁

- `bun tool/src/maglev/{maglev-ir-opcodes,maglev-ir-props,maglev-node-type,maglev-basic-block}.js` 连跑两次产物字节一致 (writeSrc 无内置 fmt, 复跑后需 `cargo +nightly fmt -p v8-maglev-ir` 归位)
- `cargo nextest run -p v8-maglev-ir --all-features` 全绿 (23 例)
- `cargo clippy -p v8-maglev-ir --all-targets --all-features -- -D warnings` RC=0
- `cargo +nightly fmt -p v8-maglev-ir -- --check` 净

---

开发日志 (复审轮次记录/草稿校正史) 见 ../../log/v8-maglev-ir.md。
