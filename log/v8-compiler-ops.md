# v8-compiler-ops 开发日志

> 2026-10-12 收敛: 自 readme/{zh,en}.md 台账迁入 (仅中文)。看板 (todo.md) 只留状态。

## readme 台账 (2026-10-12 自 readme/{zh,en}.md 收敛)

### 缺新算法: BinaryOverlappingLsaLength → v8-compiler-ops (已闭环, 原 readme「台账」节)

- [x] `[compiler-ops] 缺新算法: BinaryOverlappingLsaLength → v8-compiler-ops` (本轮)
  - 锚定: 该名在 v8/src 全库非精确符号 (含 compiler/codegen 的 overlap/Lsa/Binary 交叉检索, 无匹配)。判定为转述, v8/src/compiler 算子层唯一可落地点为 MACHINE_PURE_OP_LIST 尾部 **Pair 二元算子表项** (`Int32Pair*`/`Word32Pair*`, machine-operator.h:573-578, cc:879-884)。真实消费面 (独立复核): `Int64Lowering` 将 Int64 运算降级为本表算子 (int64-lowering.cc:457-577), 由 **32 位后端 arm/ia32/riscv32** 的 instruction selector 消费 (VisitInt32PairAdd 等), CSA 直接构造 (raw-machine-assembler.h:592-608, code-assembler.cc:764-774)。mips64 `kMips64Lsa/kMips64Dlsa` (backend/mips64/code-generator-mips64.cc:1149-1157) 为寻址指令 (rd = rt + rs<<sa), 本库中无任何 selector 发射、与本表无消费边; `kLoadStorePairs` 仅 arm64 selector 启用 (instruction-selector-arm64.cc:7431), mips64/loong64 `VisitStorePair` 为 UNREACHABLE —— 初版「mips64/loong64 LoadStorePairs 路径消费」归因有误, 消费归属已修正, Lsa 本体仍守推迟登记第 2 条。
  - 落地: 新镜像脚本 `tool/src/compiler/machine-operator.cc.js` + 模块 `src/machine_pair_operator.rs`, 表项/属性/构造 1:1。

(注: en.md 的 Ledger 节为本节英文镜像, 无 zh.md 之外的独有信息, 未迁入; 「遗留登记」属推迟登记, 保留于 readme 未迁。)
