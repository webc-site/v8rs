# v8-maglev-frontend 开发日志

> 2026-10-12: 自 todo.md 看板行与 Wave 9 派发收敛 (仅中文)。

## 门禁实测

- 单元测试: `cargo nextest run -p v8-maglev-frontend` (17/17 全部通过)
- Clippy: `cargo clippy -p v8-maglev-frontend --all-targets -- -D warnings` (0 警告)
- 格式化: `cargo +nightly fmt --check -p v8-maglev-frontend` (净)

## 模块与实现面

1. **hamt**: 高效不可变持久化哈希阵列映射树（`Hamt`、`Leaf`、`BranchHeader`、`HamtIterator`），用于 Maglev 编译期已知节点切面（`KnownNodeAspects`）与逃逸分析的高效结构共享存储。
2. **测试覆盖**: 完整移植 V8 C++ 原版测试套件 `test/unittests/maglev/hamt-unittest.cc`（17 项用例：包含空状态、查找、哈希碰撞链、深层递归分支分裂、不可变合并零分配优化、左连接合并与深度迭代器遍历等），100% 通过。
3. **依赖整理**: 消除未使用依赖（`gxhash`、`v8-interpreter`、`v8-maglev-ir`），保持 crate 极简干净。

## Wave 9 W9MFb 接力 (中断收口)

前任 W9MF 撞轮次上限中断, `graph.rs` (937 行, 已就地写就) 未挂载 (lib.rs 只 `mod hamt`), 且因 orphan 反向删除了 `gxhash`/`v8-maglev-ir`/`v8-interpreter` 依赖; lib.rs doc 引用 readme/zh.md 台账但文件缺失。

### 实测盘点 (差集)

- wc -l src: **1834** 行 → 现 **2143** 行 (挂载 graph + 新建 labeller/frame_state + 填体 + todo 深水印)
- todo!: **9** → **7** (get_constant/get_trusted_constant 已按原版 cc:132-153/169-184 落只读 intern 快路径, 剩 7 处真深水)
- 空/skel 清点: `frontend.rs.skel` (25) / `passes.rs.skel` (28) 保持草案 (建图前端 + 优化遍全族 Isolate/反馈向量深水, 依指令不强挂)
- 编译错误归属: 前任留态 `cargo check` 空跑绿 (graph orphan)

### 填体 / 挂载 / 脚本核查 / 测试

1. **graph.rs 挂载 (补差集, 不重做)**: lib.rs 加 `mod frame_state; mod labeller; mod graph;` + pub use; `cargo add v8-maglev-ir v8-interpreter gxhash`; 修 `is_dead/mark_dead` 的 `u32 & BLOCK_FLAG_IS_DEAD(usize)` 位宽对齐; 收敛 iter/iter_rev 冗余 `#[must_use]` (Iterator 已 must_use)。
2. **残余 todo! 7 处**: 逐处按 `todo!("原函数名: 前置; 解锁 crate")` 带参消息登记 (对齐 v8-objects-core 惯例, 消 unused 变量警告), 全部三要素入 readme/zh.md §三。
3. **skel 收编**: `labeller.rs` (175 行) + `frame_state.rs` (63 行) 依 maglev-graph-labeller.{h,cc} / maglev-interpreter-frame-state.{h,cc} 手撰数据面挂载 (RegisterNode emplace .second 去重 / GetBlockLabel 按需分配 / MergeDead 计数核); 打印族 + merge 状态机深水入 §四。frontend/passes 两 .skel 不强挂 (§四, 21 符号组)。
4. **脚本核查 (十一)**: 席位 4 脚本 `hamt.h.js`/`maglev-graph.cc.js`/`frontend.js`/`passes.js` 齐 (首行 shebang, 纯 ESM, 只 import node + ../../lib.js, writeSrc 直写本 crate, /// v8: 锚); 按「复跑仅限重生成基线, 修订就地改」未复跑 (graph.rs 已就地修订)。
5. **测试 1:1 补强**: `tests/graph.rs` 新建 13 例覆盖新挂载数据面 (add 自增 id / AddBlocksAt 重编号 / SweepDead 压缩 / stack-slots 单写 / 常量 intern 去重 / labeller 注册去重+provenance+block_label+line_number / MergeDead 收支 / ClearRecordedFrames / has_recursive_calls); hamt 16 例已 1:1; builder/processor 12 例依 `TestWithNativeContextAndZone` Isolate fixture → 延期台账 §五。

### 收官门禁 (W9MFb 单包, `CARGO_TARGET_DIR=target/agent_W9MFb`)

- `cargo clippy -p v8-maglev-frontend --all-targets -- -D warnings`: **0 警告** (RC=0)
- `cargo nextest run -p v8-maglev-frontend`: **29/29 全绿** (hamt 16 + graph 13, 较基线 17 增 12)
- `cargo +nightly fmt --check -p v8-maglev-frontend`: 净
- 台账: readme/zh.md 三要素 (原函数名 / 前置 / 解锁 crate) 齐: graph.rs 7 todo! + 建图/优化族 17 组符号 + 12 延期测试
- 红线遵守: 未改他 crate (v8-maglev-ir/v8-interpreter 只读引用); 未碰 lib.js / 非本族脚本; 无 git; 无全仓 test.sh/clippy.sh; 无 round/tmp/zz_ 文件名。
