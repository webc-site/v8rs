# V8 迁移看板

## 〇、口径

- 原版: `v8/src` 约 187 万行 C++/Torque, 48 个一级代码目录
- Rust 侧现状: 90 个 crate, `src/` 实测 410,787 行 (原版的 22%)；其中 rs8 依赖闭包内 160k 行, 闭包外 250k 行 (61%)
- 方法: `.agents/skills/transpile` 五步流; 规范: `.agents/skills/rust_review`; 审查: [review.md](review.md); 验收: [docs/verification_workflow.md](docs/verification_workflow.md)
- 符号理解: Rust 侧 `codegraph explore "<符号>"` (v8/ 已 includeIgnored); 原版直接读源码 + `BUILD.gn` deps
- 状态: `- [x]` 已冻结 / `- [ ]` 未冻结; 进行中在行尾标 `(开发中)` / `(审查环中)`
- 开发日志: 每 crate 一篇 `log/<crate名>.md` (约定, 表内不再逐个列链接); 编排全局 `log/orchestrator.md`; Wave 1-12 派发台账 `log/waves.md`; 转换脚本覆盖基线 `golden/tool-coverage.json`
- 测试基线: `./test.sh` 2898 项 100% 绿 (上一轮实测)
- 本文档只留当前有效结论。历史基线数字、每轮收敛计数一律沉到 `log/`, 看板不重复记载

> 绿灯只证明各 crate 自洽, 不证明引擎能跑。**当前 2898 项测试没有一项走通 CLI → V8 引擎主链路**, 详见 一。

---

## 一、主链路诊断 (实测, 非推断)

### 1.1 断点位置

CLI 执行的是一台内置的简易 AST 树遍历解释器, 不是已转写完成的 Ignition 字节码引擎。

| 环节 | 证据 | 事实 |
| --- | --- | --- |
| CLI 入口 | `rs8/src/main.rs:60` `rs8/src/main.rs:72`、`rs8/src/d8.rs:261` | 调 `run_js()` / `Interpreter::run()` |
| 实际执行 | `rs8/src/runtime/interpreter.rs:878` `run()` → `:886` `execute()` → `:1184` `eval_stmt()` / `:1464` `eval_expr()` | 自有 Lexer/Parser/AST 递归求值 |
| 真引擎 | `v8-interpreter` 14,234 行、0 桩, 入口 `BytecodeGenerator::generate` + `BytecodeDispatcher::execute` | 完整, 但无人驱动 |
| 唯一字节码通路 | `rs8/src/runtime/interpreter.rs:2452` `run_bytecode()` → `rs8/src/runtime/bytecode_bridge.rs:1235` | 生产调用点仅 `v8-benchmarks/src/lib.rs:248` 与 `rs8/tests/bytecode_test.rs`; CLI 不调 |
| 静默降级 | `interpreter.rs:2459-2461` bridge 返回 `None` 即回落 `interpreter.execute()`; `bytecode_bridge.rs` 有 8 处 `return None` | 走没走字节码不可观测, 违反"拒绝声明假象" |

结论: **链路断在 `main.rs` → `run()` 这一层**。`v8-interpreter` / `v8-objects-maps` / `v8-ic` / `v8-runtime` / `v8-heap-*` 五段 V8 运行时段落在链路外空转。

### 1.2 三套并行重复实现 (拆分复用清单)

| 类别 | 简易副本 | 正版实现 | 桥接税 | 判定 |
| --- | --- | --- | --- | --- |
| 前端 | `rs8/src/runtime/lexer.rs` 742 + `parser.rs` 961 + `ast.rs` 180 = 1,883 行 | `v8-parsing` 4,567 + `v8-ast` 6,291 = 10,858 行 (已冻结) | `bytecode_bridge.rs` 1,309 行做 rs8 AST → v8_ast 逐语句转换 | 删副本, 用正版 |
| 值模型 | `rs8/src/runtime/value.rs:1216` `JsValue` 2,038 行 | `v8-interpreter/src/bytecode_dispatcher.rs:407` `Value` | `js_value_to_interpreter_value` / `interpreter_value_to_js_value` 双向深拷贝 (bridge `:825`/`:892`) | 合成一个, 落 v8-objects-* |
| 对象属性 | 两份都是 `Arc<RwLock<HashMap<String, _>>>` | `v8-objects-maps` 5,483 行 Map/DescriptorArray/PropertyArray (0 桩) | 无接入点 | 接隐藏类 |

### 1.3 结构性性能根因 (慢 99% 不是"循环没优化")

1. 跑错引擎: CLI 用递归树遍历, V8 的字节码寄存器机闲置
2. 桥接固定税: `try_execute_with_bytecode_and_env` 每次调用按父作用域链逐层 `read()` 锁 + 全量 clone 全局表注入 dispatcher (`bytecode_bridge.rs:1243-1260`), 执行后再全量 `write()` 写回 (`:1294-1299`) — 开销与 JS 负载无关, 与全局变量数成正比
3. AST→v8_ast 转换每条语句现场做, 无缓存; 每次 `run` 重新生成字节码
4. 属性读写 = 读写锁原子争用 + 字符串哈希 + 冲突探测; 无 Map 快照、无 IC、无 FeedbackVector
5. 函数帧 `vec![Value::Undefined; n]` 反复堆分配; `arguments` 无条件实例化并插全局哈希表
6. 无堆 cage / 无写屏障 / 无 GC 参与: 对象靠 `Arc` + `rusty_alloc` 全局分配器; `v8-execution-isolate/src/isolate.rs:16-31` 的 `Isolate` 结构体根本没有 `Heap` 字段, 只有两处常量薄桥 (`lib.rs:25-42`)
7. 无 JIT: 全部留在解释分派循环

### 1.4 "桩 0" 不等于"已实现" (看板读法警告)

本轮死代码清理把未被调用的桩删掉了, 因此 `桩=0` 只代表"没有 panic 占位", 不代表功能等价原版。实测完成度:

- 严重缩水 (现状/原版 < 15%): `v8-compiler-passes` 958/80k、`v8-compiler-frontend` 1,394/70k、`v8-wasm-turbofan` 136/35k、`v8-wasm-liftoff` 480/27k、`v8-wasm-js` 274/15k、`v8-api` 503/16k、`v8-init` 255/10k、`v8-maglev-backend` 860/34k、`v8-compiler-backend` 2,578/58k、`v8-turboshaft` 5,401/87k、`v8-execution-frames` 723/25k、`v8-execution-messages` 383/10k、`v8-ic` 759/18k、`v8-objects-js-core` 1,822/25k、`v8-snapshot` 2,212/16k、`v8-heap-gc` 3,103/45k
- 判定标准以 一/三 的"链路 + 实测调用"为准, 不以 `todo!()` 计数为准
- 历史遗留的 `审计驳回: REFUTED - 残余 N 处桩` 标注已按实测重写进 四, 旧计数作废

---

## 二、死代码与重复代码核查: `./sh/unused.js`

### 2.1 用法

```bash
./sh/unused.js [目录]     # 缺省当前目录
```

- 从给定目录向上找最近的含 `Cargo.toml` 目录作为 workspace 顶层; 找不到则退出码 1 并打印 `目录不存在`
- 三级串联:
  1. `rust-analyzer scip . --output <tmp>/index.scip` — 全 workspace 符号索引 (配置 `cargo.allTargets=false`, `cfg.setTest=false`, 即不把测试算作调用者)
  2. `cargo workspace-unused-pub --scip <tmp>/index.scip` — 报"跨包 `pub` 但无任何调用者"的符号
  3. `cargo shear` — 报 `Cargo.toml` 冗余/错放依赖
- 缺工具自动安装: `rustup component add rust-analyzer`、`cargo install cargo-workspace-unused-pub`、`cargo install cargo-shear`
- 输出: stdout = 未用 pub 清单 (按 `crate/src/文件.rs` 分组, 行号 + 签名) + shear 依赖清单; stderr = 工具进度与 rust-analyzer 诊断
- 噪声已在脚本内过滤: `cargo_workspace_unused_pub` 进度行、`Found ` 计数行、以及 `/src/bin/` 块 (真实入口不是死代码)
- 退出码: 0 正常; 1 目录非法或 SCIP 失败。`cargo-workspace-unused-pub` 内部用 2 表示"发现未用项", 脚本已吸收为 0, 以 stdout 是否有内容为准
- 耗时: 全仓约 11 分钟, SCIP 索引占大头; 产物写 `mktemp` 临时目录, 不入库
- 单 crate 定向核查: `cd <crate> && ../../sh/unused.js .` (仍会向上解析到 workspace 顶层, 故结果是全仓的; 想只看该 crate 就 grep 输出头部的 `crate/src/...` 分组行)

### 2.2 门禁规则

- 每轮收官跑一次, 结果按 2.3 格式登记进本节, 原始输出不入库
- crate 冻结门禁: 该 crate 不得出现在未用 pub 报告中, 或逐条写明豁免理由
- `cargo shear` 报的冗余/错放依赖必须删或移; 新增依赖只能 `cargo add` 并登记 八
- 豁免白名单 (这些是 1:1 转写完整性要求, 不是死代码):
  - 原版常量/枚举/offset 查表面: `v8-api` `v8-flags` `v8-common` `v8-objects-core` `v8-wasm-*` 的 `*_generated.rs`
  - snapshot / roots / root-index 表
  - 仅供 `unittests` / `cctest` 1:1 转写的测试专用导出
  - 尚未接入但已登记在 三 战役排期里的运行时段 (必须带战役编号, 不许无声存在)
- **链路外 crate ≠ 死代码**, 但必须在 四 的"链路"列显式标 `外`, 并在 三 有对应战役条目

### 2.3 当前基线 (2026-10-14, 全仓实测)

| 项 | 数量 | 说明 |
| --- | --- | --- |
| 未用 pub 符号 | 87 处 / 30 文件 / 23 crate | `v8-codegen-arm64/instructions.rs` 8 处、`v8-compiler-frontend/graph_assembler.rs` 9 处最集中 |
| 冗余/错放依赖 | 34 处 | 未用 `log` 17、错放 `log` 5、未用 `humansize` 6、未用 `libc` 2, 另有 `unicode-normalization` `unicode-ident` `criterion` `rusty_alloc-api` 各 1 |
| `todo!()` / `unimplemented!()` 残留 | 410 处 / 23 crate | 详见 四 各行; 大头 `v8-heap-core` 70、`v8-wasm-core` 52、`v8-objects-js-async` 43、`v8-objects-collections` 32 |
| rs8 依赖闭包 | 20 / 90 crate | 闭包外 250,796 行 (61%) |

待处置 (下一轮清账, 按 2.2 分类):

- [ ] D1 依赖清账: 删 34 处 shear 命中; 特别核对 `rusty_alloc-api` 被报未用与 十二.1 "全工作区接入" 的矛盾 (若确实只在 bench 二进制生效, 移到对应 crate)
- [ ] D2 未用 pub 87 处分诊: 属 2.2 白名单的登记豁免; 属战役排期的挂战役号; 其余删除
- [ ] D3 重复实现三份副本按 1.2 清单收敛, 计入 三 战役 A/B/C 的删除量

---

## 三、打通链路战役排期 (A→H, 严格串行准入)

依据 [docs/verification_workflow.md](docs/verification_workflow.md): 拒绝声明假象, 实测调用、真实基准、零桩代码才算打通。

### 战役 A: CLI 改驱真引擎 (先换驱动, 不加功能)

- `rs8/src/main.rs:60,72` 与 `rs8/src/d8.rs:261` 的入口从 `run_js` / `Interpreter::run` 切到 `run_bytecode`
- 删除 `interpreter.rs:2459-2461` 的静默回落, 未覆盖改为显式 `Err(Unsupported)` 冒泡到 CLI 报错
- `bytecode_bridge.rs` 8 处 `return None` 逐个补或逐个变成硬错误
- 加覆盖率可观测: 输出 `--print-bytecode-coverage`, 报告实际走字节码的语句数/总数
- 准入: test262 与 `./test.sh` 全绿 + `node ./tool/benchRun.js` 不快于 AST 版则先补 bridge 缺口

### 战役 B: 前端合一, 删 rs8 自带词法/语法

- rs8 的 `lexer.rs`+`parser.rs`+`ast.rs` (1,883 行) 由 `v8-parsing`+`v8-ast` (10,858 行) 取代
- `bytecode_bridge.rs` 的 AST 转换层 (约 1,300 行) 随之整体删除
- 准入: `rs8/tests/*` + test262 全绿; `grep -rn 'eval_stmt\|eval_expr' rs8/src` 为空

### 战役 C: 值模型合一 + 属性接隐藏类

- `JsValue` 与 `bytecode_dispatcher::Value` 合成单一引擎值类型, 落到 `v8-objects-*`; 删双向深拷贝
- 属性存储 `Arc<RwLock<HashMap<String,_>>>` → `v8-objects-maps` 的 Map + DescriptorArray + PropertyArray, 读写降为 O(1) 槽位偏移 (对标 `v8/src/objects/map.h`)
- 全局环境单一化: 停止每次调用全量 clone 父作用域链注入 + 执行后全量写回 (`bytecode_bridge.rs:1243-1260,1294-1299`)
- 前置: `v8-objects-maps` 0 桩但需复审冻结
- 准入: 运行期 crate 内 `grep 'RwLock<HashMap<String'` 为空; benchRun 逐用例提速比记录在案

### 战役 D: 连续寄存器栈帧

- 对标 `v8/src/execution/frames.h`。帧从 `vec![Value::Undefined; n]` 改为连续执行栈 + `frame_base` 指针推进, 返回复位
- `arguments` 惰性实例化, 未显式引用不分配
- 涉及 `v8-execution-frames` (现 723 行 / 原版 25k) 补齐

### 战役 E: FeedbackVector 与内联缓存

- `v8-objects-rest` (8,906 行, 13 桩) 的 FeedbackVector 接入字节码反馈槽; `v8-interpreter` 生成期分配 slot
- `v8-ic` 现仅 759 行 (原版 `ic.cc` 规模远大于此), 先按原版补齐 StubCache 与 IC 状态机, 再接 `GetNamedProperty` / `SetNamedProperty`
- monomorphic 命中: Map 一致即跳过原型链与结构检索, 直接槽位读写

### 战役 F: 堆与 GC 进链路

- `v8-execution-isolate::Isolate` 补 `Heap` 字段, 接 `v8-heap-core` 分配路径; 对象从 `Arc` + std 分配迁到 cage + 写屏障
- `v8-heap-gc` 真实 Scavenger / MarkCompact 接入 (对标 `v8/src/heap`)
- 前置: `v8-heap-core` 70 桩、`v8-sandbox` 10 桩、`v8-roots` 14 桩清零

### 战役 G: API 面与引导成型

- `v8-api` 从 503 行常量表扩成真实 trait 面 (Isolate / Context / Script / Value / HandleScope); `rs8` 与 `rs8-capi` 改为消费 `v8-api`
- `v8-init` 255 行 → 真实 setup-isolate / setup-context, 接 `v8-snapshot` (2,212 行, 原版 16k 需补)
- `v8-runtime` 13,754 行 (当前不在 rs8 闭包) 接入, 使 CEntry / Runtime call 可用

### 战役 H: JIT 与 PGO

- `v8-baseline` (Sparkplug, 1,187 行) 直出机器码 + 热点计数触发
- `v8-maglev-*` 试点真实指令选择与 regalloc, 再推 `v8-turboshaft` + `v8-compiler-backend` (两者现状 5.4k/2.6k, 原版 87k/58k, 需按原版重建)
- 发布构建按 [docs/pgo.md](docs/pgo.md) 串联真实负载训练

### 16.x 旧"四大瓶颈"条目消账

旧 十六 的四条根因 (寄存器堆分配 / HashMap 属性 / 无 IC / 无 JIT) 已并入上面 A→H; "跑错引擎"这一条旧文档未列, 实为最重的一条, 见 1.1。

---

## 四、模块看板

列: `crate | 原版k行 | 现状k行 | 桩 | 链路 | 状态与缺口`。规模列为 `src/*.rs` 实测, 原版列为旧文档估算。

链路: `CLI` = rs8 直接依赖; `闭包` = 传递依赖进入; `外` = 不在 rs8 依赖图 (须对应 三 的战役号)。

### L0 地基层

- `- [x]` v8-common | 8 | 7.7 | 0 | 闭包 | 引擎地基, 全局常量/Tagged/Smi
- `- [x]` v8-base | 28 | 21.0 | 0 | 闭包 | 通用算法/位操/RNG/CPU 探测
- `- [x]` v8-base-platform | 10 | 4.4 | 0 | 闭包 | mmap/信号量/线程/虚拟内存
- `- [ ]` v8-zone | 3 | 3.8 | 18 | 闭包 | arena bump 分配器, 18 桩未清 (战役 F 依赖)
- `- [x]` v8-flags | 7 | 32.7 | 0 | 闭包 | 启动 flag 解析
- `- [ ]` v8-logging | 8 | 5.7 | 20 | 外 | 20 桩; 战役 F 前不接入
- `- [ ]` v8-utils | 5 | 3.3 | 18 | 闭包 | 18 桩
- `- [ ]` v8-tasks | 1 | 0.8 | 8 | 闭包 | 8 桩
- `- [ ]` v8-tracing | 4 | 3.7 | 16 | 外 | 16 桩

### L1 数值与字符串层

- `- [ ]` v8-numbers | 3 | 3.9 | 0 | 闭包 | 桩已清, 待复审冻结 (zmij 内核 + ECMAScript 格式层)
- `- [ ]` v8-bigint | 6 | 5.6 | 16 | 外 | 16 桩
- `- [ ]` v8-strings | 8 | 4.6 | 0 | 闭包 | 桩已清, 待复审冻结 (icu4x + memchr)
- `- [x]` v8-date | 2 | 2.3 | 0 | 外 | 日期解析/时区缓存

### L2 前端层

- `- [x]` v8-ast | 15 | 6.3 | 0 | CLI | 已冻结; 但 CLI 只用其节点类型经 bridge, 不用其解析产物 (战役 B 后为唯一 AST)
- `- [x]` v8-parsing | 23 | 4.6 | 0 | **外** | 已冻结却完全没接: CLI 走 rs8 自带 parser → 战役 B
- `- [ ]` v8-regexp-syntax | 10 | 5.7 | 2 | 外 | 2 桩
- `- [x]` v8-torque | 30 | 19.7 | 0 | 外 | Torque→Rust 生成器; 生成布局尚未接 objects 层
- `- [ ]` v8-objects-core | 30 | 16.6 | 0 | 闭包 | 桩 65→0, 待复审; 运行时值模型未采用 → 战役 C

### L3 对象与内存层

- `- [x]` v8-objects-strings | 9 | 5.6 | 0 | 闭包 | StringShape/Flat/哈希
- `- [ ]` v8-objects-arrays | 6k? | 6.3 | 13 | 外 | 13 桩; FixedArray 族 (战役 C 的 PropertyArray 底座)
- `- [ ]` v8-objects-maps | 15 | 5.5 | 0 | 外 | 桩 78→0; **隐藏类无运行时消费者** → 战役 C
- `- [ ]` v8-objects-collections | 15 | 6.9 | 32 | 外 | 32 桩
- `- [x]` v8-objects-js-core | 25 | 1.8 | 0 | 外 | 冻结但仅剩骨架 (7%), JSObject 未接运行时 → 战役 C/F
- `- [x]` v8-objects-js-func | 20 | 3.5 | 0 | 外 | 骨架 (17%), SharedFunctionInfo 未接 → 战役 F/G
- `- [ ]` v8-objects-js-async | 10 | 5.7 | 43 | 外 | 43 桩
- `- [ ]` v8-objects-rest | 25 | 8.9 | 13 | 外 | 13 桩; FeedbackVector 在此 → 战役 E
- `- [x]` v8-handles | 6 | 2.1 | 0 | 外 | 句柄/作用域; 运行时未用 → 战役 C
- `- [ ]` v8-roots | 10 | 8.2 | 14 | 外 | 14 桩 → 战役 F
- `- [x]` v8-heap-base | 5 | 1.9 | 0 | 闭包 | GC 基础: 页/空间/脏位集
- `- [ ]` v8-heap-core | 35 | 9.5 | 70 | 闭包 | 70 桩; 仅常量薄桥入闭包, 无分配路径 → 战役 F
- `- [ ]` v8-heap-gc | 45 | 3.1 | 0 | 闭包 | 骨架 (7%); Scavenger/MarkCompact 未打通 → 战役 F
- `- [ ]` v8-heap-cppgc | 19 | 1.7 | 3 | 外 | 3 桩, 后置
- `- [ ]` v8-sandbox | 14 | 3.1 | 10 | 闭包 | 10 桩 → 战役 F

### L4 执行与代码生成层

- `- [ ]` v8-codegen-core | 35 | 9.9 | 0 | 闭包 | 桩 32→0, 待复审; 汇编器基类/RelocInfo
- `- [x]` v8-codegen-x64 | 22 | 3.9 | 0 | 外 | 冻结; 无生产者 (无 JIT) → 战役 H
- `- [x]` v8-codegen-arm64 | 32 | 7.4 | 0 | 外 | 同上
- `- [x]` v8-builtins-x64 / arm64 | 5.8 各 | 1.0 / 0.9 | 0 | 外 | 汇编内建 trampoline; 无嵌入码可加载 → 战役 H
- `- [x]` v8-codegen-csa | 20 | 2.5 | 0 | 外 | CSA 图 DSL; 未生成 handler → 战役 H
- `- [x]` v8-interpreter | 28 | 14.2 | 0 | CLI | **真引擎, 0 桩, CLI 主路径不调** → 战役 A 的第一目标
- `- [ ]` v8-execution-isolate | 40 | 2.1 | 0 | CLI | `Isolate` 无 `Heap` 字段, 仅线程状态与空防护器 → 战役 F
- `- [x]` v8-execution-frames | 25 | 0.7 | 0 | 外 | 骨架 (3%) → 战役 D
- `- [x]` v8-execution-messages | 10 | 0.4 | 0 | 外 | 骨架 (4%); 异常格式化未接 CLI
- `- [x]` v8-builtins-core | 20 | 2.2 | 0 | 外 | 内建注册表; 未装进运行时 → 战役 G
- `- [x]` v8-builtins-array | 35 | 3.2 | 0 | 外 | 骨架 (9%)
- `- [x]` v8-builtins-string | 20 | 1.6 | 0 | 外 | 骨架 (8%)
- `- [x]` v8-builtins-typedarray | 25 | 2.2 | 0 | 外 | 骨架 (9%)
- `- [x]` v8-builtins-promise | 20 | 2.8 | 0 | 外 | 骨架 (14%)
- `- [x]` v8-builtins-collections | 15 | 1.6 | 0 | 外 | 骨架 (11%)
- `- [x]` v8-builtins-proxy-reflect | 10 | 0.9 | 0 | 外 | 骨架 (9%)
- `- [x]` v8-builtins-regexp | 8 | 1.3 | 0 | 外 | 骨架 (16%)
- `- [x]` v8-builtins-misc | 30 | 13.4 | 0 | 外 | 骨架 (45%), 相对最厚一片
- `- [x]` v8-ic | 18 | 0.8 | 0 | 外 | 骨架 (4%), 反向依赖 0 → 战役 E
- `- [x]` v8-runtime | 23 | 13.8 | 0 | **外** | 13.8k 行 RUNTIME_FUNCTION 大全不在闭包 → 战役 G
- `- [x]` v8-json | 7 | 1.4 | 0 | 外 | JSON 语义自研; 未接内建分发
- `- [ ]` v8-regexp-exec | 14 | 7.5 | 2 | 外 | 2 桩
- `- [ ]` v8-regexp-compiler | 20 | 3.8 | 11 | 外 | 11 桩
- `- [ ]` v8-regexp-x64 / arm64 | 2.7 / 2.9 | 0.17 / 0.10 | 0 | 外 | 仅栈帧常量骨架, 宏汇编后端未写
- `- [x]` v8-debug | 19 | 5.7 | 0 | 外 | 调试器/break 链
- `- [x]` v8-deoptimizer | 10 | 1.2 | 0 | 外 | 反优化; 无 JIT 则无对象可反优化
- `- [x]` v8-diagnostics | 16 | 3.4 | 0 | 外 | perf-jit/gdb-jit/对象打印
- `- [ ]` v8-execution-simulator | — | 1.4 | 0 | 外 | 指令模拟器 (arm64 主机跑 x64 码路径用)

### L5 优化编译器层

- `- [x]` v8-compiler-graph | 45 | 5.2 | 0 | 外 | 海图 IR 基础
- `- [x]` v8-compiler-ops | 60 | 5.4 | 0 | 外 | 算子定义/校验
- `- [x]` v8-compiler-broker | 30 | 2.4 | 0 | 外 | 骨架 (8%)
- `- [ ]` v8-compiler-frontend | 70 | 1.4 | 11 | 外 | 骨架 (2%) + 11 桩 → 战役 H
- `- [x]` v8-compiler-passes | 80 | 1.0 | 0 | 外 | 骨架 (1%); 旧记"已完成 25 用例"须按 1.4 重审
- `- [ ]` v8-turboshaft | 87 | 5.4 | 0 | 外 | 骨架 (6%) → 战役 H
- `- [ ]` v8-compiler-backend | 58 | 2.6 | 0 | 外 | 骨架 (4%), 指令选择/regalloc 未实现 → 战役 H
- `- [x]` v8-maglev-ir | 40 | 2.5 | 0 | 外 | 骨架 (6%)
- `- [ ]` v8-maglev-frontend | 40 | 2.1 | 1 | 外 | 骨架 (5%) + 1 桩 → 战役 H
- `- [ ]` v8-maglev-backend | 34 | 0.9 | 0 | 外 | 骨架 (3%) → 战役 H
- `- [ ]` v8-baseline | 11 | 1.2 | 0 | 外 | Sparkplug, 机器码直出未实现 → 战役 H
- `- [x]` v8-compiler-dispatcher | 2 | 0.6 | 0 | 外 | 后台编译调度

### L6 WebAssembly 层

- `- [ ]` v8-wasm-core | 50 | 12.0 | 52 | 外 | 52 桩, L6 最大缺口
- `- [ ]` v8-wasm-objects | 25 | 2.9 | 22 | 外 | 22 桩
- `- [ ]` v8-wasm-liftoff | 27 | 0.5 | 0 | 外 | 骨架 (2%), Liftoff 未实现
- `- [ ]` v8-wasm-turbofan | 35 | 0.1 | 0 | 外 | 骨架 (0.4%)
- `- [x]` v8-wasm-interp | 29 | 5.5 | 0 | 外 | DrumBrake 解释器, 默认关闭
- `- [ ]` v8-wasm-js | 15 | 0.3 | 0 | 外 | 骨架 (2%), JS API 绑定未写
- `- [ ]` v8-trap-handler | 2 | 1.3 | 5 | 外 | 5 桩

### L7 快照与外壳层

- `- [x]` v8-snapshot | 16 | 2.2 | 0 | 外 | 骨架 (14%), 序列化面无 embedded blob 产出
- `- [ ]` v8-init | 10 | 0.3 | 0 | 外 | 骨架 (3%), 引擎引导未打通 → 战役 G
- `- [ ]` v8-api | 16 | 0.5 | 0 | CLI | 仅常量表, 无 Isolate/Context/Script/Value trait → 战役 G
- `- [ ]` v8-libplatform | 3 | 1.5 | 10 | CLI | 10 桩, 默认平台/任务调度
- `- [x]` v8-inspector | 20 | 2.2 | 0 | 外 | CDP 协议
- `- [x]` v8-profiler | 18 | 0.8 | 0 | 外 | 采样分析
- `- [x]` v8-extensions | 1 | 0.5 | 0 | 外 | 内建扩展
- `- [x]` v8-dumpling | 1 | 0.9 | 0 | 外 | —
- `- [x]` v8-fuzzilli | 0.8 | 0.6 | 0 | 外 | fuzzing 集成, 不阻塞
- `- [ ]` rs8 | 15 | 9.1 | 0 | — | 9,062 行, 内含简易前端与求值器 → 战役 A/B/C 主战场
- `- [ ]` rs8-capi | 10 | 0.7 | 0 | 外 | 仅封装简易解释器, 无 1:1 C ABI 面 (旧 九)
- `- [ ]` v8-benchmarks | — | 0.5 | 0 | — | 评测 harness, 非引擎; `run_bytecode` 当前唯一生产调用点

---

## 五、里程碑

- `- [x]` M0 工作区基础设施 — workspace + 模板 + 门禁脚本 + 看板
- `- [ ]` M1 L0+L1 全冻结 — 实测残余桩: zone 18 / utils 18 / tasks 8 (闭包内), logging 20 / tracing 16 / bigint 16 (闭包外)
- `- [ ]` M2 L2 全冻结 — 实测残余桩: regexp-syntax 2; objects-core 桩已清待复审
- `- [ ]` M3 L3 对象与内存层消账 — 实测残余桩 133: heap-core 70 / objects-js-async 43 / objects-collections 32
- `- [x]` M4 最小引擎闭环与解释器打通 — **仅解释器自身打通, CLI 未接入** → 撤销"闭环"含义, 由战役 A 重做
- `- [ ]` M5 L4 运行时与内建消账 — 残余桩 24; 真实缺口是 `v8-runtime` 不在闭包 + `v8-execution-isolate` 无 Heap
- `- [ ]` M6 L5 优化编译器端到端 — 见战役 H
- `- [ ]` M7 L6 WebAssembly 规范测试 — 残余桩 74, 大头在 wasm-core 52
- `- [ ]` M8 JS 标准测试 100% — 见 十一; 前提是战役 A-C
- `- [ ]` M9 性能基准对标 V8 — 见 十二; 战役 A 完成前数字无意义
- `- [ ]` M10 终验全量对抗审查与发布 — 全门禁绿灯 + 全平台 release

关键路径: v8-base → v8-zone → v8-ast → v8-parsing → v8-objects-core → **战役 A/B/C (链路合一)** → v8-heap-core → v8-execution-isolate → v8-codegen-{x64,arm64} → v8-compiler-graph → v8-turboshaft → v8-compiler-backend → v8-codegen-csa → v8-builtins → rs8 → Test262 → js-engine-benchmark

---

## 六、分层与依赖

依赖自下而上, 层内可并发。C++ 实测的三处依赖倒置:

- 内建与字节码分派器依赖编译器后端: `codegen/code-stub-assembler.h` include `src/compiler/code-assembler.h`; Ignition handler 由 `interpreter-generator*.cc` 经 CSA/TSA 生成 → CSA 内建与 handler 都要 L5 后端才能出机器码
- 快照在启动关键路径: V8 无"无快照"模式, 构建期 mksnapshot 生成 embedded blob → v8-snapshot + v8-init 必须在引擎引导前就位
- objects 与 heap 循环依赖: C++ 同属 `v8_base_without_compiler` 一个 target (objects 120 文件引 heap, heap 98 引 objects, 41 头引 `execution/isolate`); Rust 破环策略见 七

规模实测 (原版): codegen/x64 18.3k + diagnostics/x64 3.8k; codegen/arm64 26.8k + diagnostics/arm64 5.0k; turboshaft 87k; builtins 全目录 135k; wasm/interpreter 29k; heap/cppgc-internal 15.9k + cppgc-js 3.2k

- L0 地基 ~65k | L1 数值与字符串 ~19k | L2 前端 ~78k | L3 对象与内存 ~245k | L4 执行 ~430k | L5 优化编译器 ~490k | L6 wasm ~170k | L7 快照/API/外壳 ~90k

---

## 七、工程规范 (合并原 一/七/十二/十三, 规则只写一次)

### 7.1 每包流水线

1. `./sh/new.sh <crate名>` 建微 crate (自动进 workspace)
2. 研读原版目录 + `v8/test/unittests` / `v8/test/cctest` 对应用例
3. Developer 子代理: Rust 实现 + 单测, 严格按 `.agents/skills/rust_review`
4. Reviewer 子代理: 逐行比对原版, 原版测试 1:1 转写进 `tests/`
5. 对抗审查环: 每轮全新 Reviewer 独立审查; 有缺陷 → 修复并归零连续计数; 连续 3 轮 "0 缺陷 / 0 遗漏 / 全过" → 冻结
6. 3 轮 clean 只是必要条件; 7.3 机器门禁不过, clean 计数不算
7. **冻结追加硬条件**: 通过 `./sh/unused.js` 未用 pub 核查 (二.2 白名单外为 0); 若该 crate 属运行时段, 还必须给出实测调用证据 (链路列非 `外`, 或挂 三 的战役号)

### 7.2 红线

- 写完必测: `cargo nextest run -p <crate名>` 实跑, 100% 绿才交付
- 依赖只用 `cargo add` / `cargo add --path`, 禁手改 Cargo.toml; 新库登记 八; `cargo shear` 无冗余
- 注释中文; 禁 panic; 错误集中 `error.rs`; 日志用 `log` crate
- 测试统一 `#[ctor::ctor(unsafe)] log_init::init()`, 返回 `aok::Void` 以 `OK` 结尾
- 异步测试 `compio::runtime::Runtime::new()?.block_on(async { ... })`
- 语义绑定处禁库替换 (八.3)

### 7.3 质量门禁 (oracle 来自 C++, 不来自 LLM)

- 函数级对账: 原版侧 `clang -Xclang -ast-dump=json` 列全函数/常量/枚举入 `golden/<crate>/symbols.json`; Rust 侧每项带锚 `/// v8: src/zone/zone.cc Zone::New`; 脚本 diff, 未登记 >0 不得冻结
- 金标准三类入 `golden/<crate>/`: 常量/枚举/offset/sizeof 逐项相等断言; 纯函数 I/O 录制 (dtoa/strtod/dateparser/scanner token/regexp AST/wasm decoder); 运行时输出 (`--print-bytecode`, 复用原版 `bytecode_expectations/*.golden`)
- 参考构建: `ref/args.gn` 钉死 Chromium 默认配置 (`v8_enable_pointer_compression=true`, `v8_enable_sandbox=true`); 本机 arm64 + CI x64 各构建一次; CI 只读 golden
- 双侧差分: 适用 numbers/bigint/strings/date/regexp-syntax/parsing/wasm-core/utils; dev 测试挂参考静态库 + 薄 `extern "C"` shim, proptest 同输入双跑逐字节比对; C++ 侧 ASan + UBSan
- 变异测试: `cargo mutants -p <crate> --in-diff <diff>` (审查环中) / 全量一次 (冻结前); 存活变异三选一: 补测试 / 标等价 (`.cargo/mutants.toml` + 理由) / 登记原版同样不覆盖
- 有界证明: `kani::any()` 证 Smi 编解码/指针压缩/弱引用标记位/BitField/LEB128, 每 crate ≤10 harness, 试点 v8-common / v8-objects-core, 不作冻结门禁
- 延期台账: 依赖 `TestWithIsolate` 而跳过的原版用例, 写进 crate README "延期用例" 一节 (用例名 / 缺什么前置 / 解锁 crate); 上游冻结时编排者 grep 全台账重开下游
- tests/ 卫生: 新用例并入与原版 gtest 同名文件; 出现 `round`/`tmp`/`zz_` 文件名 → CI 失败
- unsafe 棘轮: clippy 加 `-W clippy::undocumented_unsafe_blocks -W clippy::multiple_unsafe_ops_per_block`; 每 crate unsafe 数存基线, 增长需 PR 说明
- 静态加严 + 死代码: `./sh/unused.js` (二) 纳入每轮收官

### 7.4 内存安全动态核查 (Miri)

- 分档: A 必跑 (v8-zone / v8-objects-core / v8-objects-strings / v8-objects-arrays / v8-handles / v8-compiler-graph / v8-utils / v8-heap-base, 后续加 heap-core/heap-gc/roots/codegen-core/turboshaft/maglev-ir) | B 部分跑 (v8-base / v8-common / v8-tasks / v8-flags / v8-logging / v8-sandbox / v8-base-platform, 屏蔽 mmap/JIT 写保护/pkru/信号/栈探测) | C 不跑 (零 unsafe: ast/bigint/date/regexp-syntax/torque/wasm-core; unsafe ≤7: numbers/strings/tracing/parsing) | C 不跑: trap-handler 与执行 JIT 机器码的测试 (改 ASan + 真机)
- 新 crate 建立时即定档, 写进 四
- 代码约定: 平台调用集中少数模块并给 `#[cfg(miri)]` 后端 (页分配→`std::alloc`, JIT 写保护→空操作, cpuid→固定特性集); 跑不了的测试用 `#[cfg_attr(miri, ignore = "FFI: ...")]` 且必写原因; 大数据压测 `const N: usize = if cfg!(miri) { 64 } else { 100_000 }`; 指针压缩 cage 在 Miri 用小 cage; provenance 用 `cage_base.wrapping_byte_add(offset)` / `with_addr`, 不开 `-Zmiri-strict-provenance`
- 参数: 门禁 Stacked Borrows; 夜间 `-Zmiri-tree-borrows` 只报告不阻塞; 并发 crate `-Zmiri-many-seeds=0..8`; 禁 `-Zmiri-disable-isolation`; `.config/nextest.toml` 的 `[profile.default-miri]` 放宽 30s
- 准入: CI 白名单只放已本地跑绿的 crate, 只增不减; 当前白名单 `v8-zone`, 下一批 objects-core / objects-strings / handles / compiler-graph / utils / heap-base。实测基线: v8-zone 78/78 通过 16.7s
- 子代理改了 `unsafe` 必须本地跑该 crate Miri

### 7.5 子代理方法论

- 签名先行: 先出类型骨架 (struct/enum/签名/布局对齐点), 冻结签名后再派函数体, 填体按函数并发
- 类型 oracle: prompt 必带目标签名; `rustc` + `clippy` 硬 oracle; 同一单元 best-of-N, 编译过 + 单测过者胜出
- 切块粒度: 按函数/依赖闭包切单元, 单次投喂几百行; 跨块只给签名 (整仓投喂成功率骤降)
- 派发上限 (单代理 150 轮预算): 一次 ≤10 个原版文件 / ≤6k 行原版; 超出按文件族拆分并发
- 宏先展开: `#define`/`#ifdef` 机械展开为显式常量/函数再翻译, 禁让子代理"理解"宏; builtins/interface-descriptors 密集区逐个登记展开产物
- 两阶段: 一阶段功能等价直译 (允许非惯用) → 冻结; 二阶段按 rust_review 惯用法重构 + 全量回归; 分开派发
- 全局术语表 `.agents/glossary.md` (C++ ↔ Rust 命名 + gtest → Rust 写法映射), 所有 prompt 注入
- 审查规范详见 [review.md](review.md): 冻结判定不可与修复同源 (独立会话派生); 上下文绝对隔离; 工具裁决物证说话; 严禁仅凭 diff 臆断, 必须真实编译/测试/反编译验证; 四阶工具核查 = `./tool/reverify_deps.sh` (拓扑依赖穿透) + `./tool/reverify_symbols.sh` (符号核验) + `./tool/profile_sample.sh` (执行路径与采样), 终局 `./tool/review_gate.sh`
- 测试迁移: 先提断言规约清单 (每断言一行) 再按规约生成 Rust 测试; 断言强度守恒, 弱化/删除 = 假绿; 测试名保留原名; 覆盖映射登记 crate README, 跳过项写理由
- 差分模糊: 双侧 harness 共享 corpus; 比对前滤非确定性 (随机序/时间/地址/线程交错); 引擎级 rs8 vs d8/node, LLM 作输入变异引擎, wasm 用 wasm-smith; crash 固化为 `tests/` 回归 + 语料入库; 确定性前置 = 冻结 flags/随机种子/线程数/mock 时钟

### 7.6 并发与隔离

- 一个席位 = 一个互斥原版文件族 + 一组独占 crate; 席位间禁改对方文件与对方 crate 的 `lib.rs`
- 每席 `CARGO_TARGET_DIR=target/agent_<席名>` 且只跑 `-p <crate>`; **禁子席位跑全仓 `./test.sh` / `./sh/clippy.sh`** (后者带 `--fix`); 全仓门禁由主控编排者收官把关
- 席位文件表 `target/wave<n>/<席名>.txt` (相对 `v8/`, 编排者生成, 不入库)
- 同层无依赖 crate 一次性并发派发; 开发与审查流水线重叠
- 中断接力: 撞轮次上限后, 接力代理先实测盘点 (行数/空文件/编译错误归属) 再收尾, 禁重做已落地内容
- 提供方瞬时故障: 中断 ≠ 未落地。重派前三件套 `wc -l src` + 空文件清点 + `cargo check/clippy/nextest -p <crate>`, 只补差集

### 7.7 CI 流水线

| 任务 | 触发 | 内容 | 状态 |
| --- | --- | --- | --- |
| lint | push/PR | nightly `fmt --check` + `clippy --all-targets --all-features -D warnings` (只读, 无 `--fix`) | 已落地 |
| test | push/PR | 4 平台 nextest (2898 项) | 已落地 |
| miri | push/PR | 白名单 matrix | 已落地 |
| miri-tb | 每日/手动 | Tree Borrows 报告, `continue-on-error` | 已落地 |
| udeps | 每日/手动 | `cargo +nightly udeps --workspace --all-targets` | 已落地 |
| deadcode | 每轮收官/手动 | `./sh/unused.js` (未用 pub + `cargo shear`) | 本轮起接入 |
| audit | push/PR | 函数级对账 + 金标准 + tests 卫生 + unsafe 棘轮 | 待 7.3 脚本 |
| mutants | 每周 + PR `--in-diff` | cargo-mutants | 待 P1 |
| sanitizer | 每日 | Linux nightly ASan (base-platform/sandbox/trap-handler) + TSan (tasks) | 待 P2 |

PR 必过 lint + test + miri + audit; 每日/每周 job 失败自动开 issue 或看板标注, 不阻塞 PR; 所有 job 设 `timeout-minutes`, 权限 `contents: read`。

---

## 八、开源库复用策略

原则: 语义绑定处 (JS 可见行为) 一律自研转写; 非语义绑定处 (平台/诊断/工具/纯算法内核) 用高性能成熟库。新依赖 `cargo add` 且必须登记本章, Reviewer 按 8.3 核对。

三档: 直接用 (库即最终实现) | 内核用 (库做核心算法, 外层语义自写) | 仅 oracle (不进产品代码, 只作差分对照)。

SIMD 统一 `fearless_simd` (Linebender): 运行时分发 SSE4.2/AVX2/AVX-512/NEON, 一套代码覆盖 x64+arm64; 禁 `safe_arch`/`wide`/`std::simd`。

分包映射:

- v8-base — `fastrand`+`getrandom` (仅种子源, 引擎内 XorShift128+ 自研) / `parking_lot` / `coarsetime` / `crossbeam-deque` / `fearless_simd`
- v8-base-platform — `rustix` 或 `libc` / `memmap2`
- v8-zone — 内核评估 `bumpalo`, 外层 Zone API 1:1 自写
- v8-flags — 自研; 静态表 `phf`
- v8-utils — `gxhash` / `itoa`
- v8-tasks、v8-libplatform — `parking_lot` + `crossbeam-deque`; 不引异步运行时
- v8-numbers — `zmij` (浮点格式化内核) + `itoa`; ECMAScript Number→String 格式层自写
- v8-bigint — 内核评估 `malachite`; asIntN/布局/wasm 互操作自写
- v8-strings — `icu4x` 2.0 (CaseMap/Normalization/BreakIterator) / `unicode-ident` / `memchr` / `fearless_simd`
- v8-ast、v8-parsing、v8-regexp-syntax、v8-torque — 自研; `unicode-ident` + `memchr` + `fearless_simd` + `phf`; AST 分派 `enum_dispatch`
- v8-codegen-{core,x64,arm64} — 上层自研 (label 绑定/reloc-info/长分支 trampoline/伪指令展开); 指令 encode 内核评估 `asmkit` (AsmJit 移植), 须先差分逐字节一致才采用, 否则全转写; 禁反汇编库进编译路径
- v8-diagnostics — 反汇编转写原版; `iced-x86` / `yaxpeax-arm` 降为仅 oracle
- v8-interpreter、v8-builtins-*、v8-ic、v8-runtime — 自研; 热路径 `memchr` + `fearless_simd`
- v8-json — 语义自研; 扫描内核 `fearless_simd`
- v8-regexp-* — 自研 (回溯 + 捕获语义, 禁 regex/RE2 类库)
- v8-compiler-backend — regalloc 内核评估 `regalloc3` / `regalloc2`; VirtualRegister/frame 引用消除外层自写; 先 Maglev/Liftoff 试点过差分再上 TurboFan
- v8-wasm-core — decoder 自研; `wasmparser` 仅差分 oracle
- v8-inspector — CDP 自研; 协议 JSON 层 `sonic-rs`
- v8-snapshot、v8-api、v8-profiler — 自研; 校验和评估 `crc32fast`
- rs8 / rs8-capi — 工具层 `jiff` / `hipstr`; capi 头 `cbindgen`
- 全局分配器 — `rusty_alloc-api` (workspace 钉版 2.2.6)
- 横切: `HashMap`/`HashSet` 一律 `gxhash` (引擎语义 hash 自研, 禁 gxhash 替换) | 静态表 `phf` | 枚举配 `strum` | 消 `dyn` 用 `enum_dispatch` | 非语义 JSON 一律 `sonic-rs` (禁 serde_json) | 测试数据 `fastrand`
- `fearless_simd` 位点: CopyChars/UTF 转码/大块比较 + scanner 空白批量 + json 扫描 + builtins 字符串搜索 (memcpy 类交编译器自动向量化)
- asmkit `jit` allocator 不采用: V8 代码内存是 code range cage 体系一部分, 独立 JIT 堆语义不匹配; W^X/双映射思路仅参考, 落地在 v8-heap-core 自研

禁用清单: Math.random (XorShift128+ 1:1, 禁 rand 替换) | Number→String / parseFloat / parseInt (zmij/itoa 只作内核) | BigInt (asIntN/布局/wasm 绑定) | 字符串 hash 与 identity hash (seed 与迭代序语义绑定) | JSON.parse/stringify (转义/代理对/数字格式/键序是 ECMAScript 语义) | RegExp (回溯 + lastIndex) | String 布局 (Seq/Sliced/Cons/Thin 是 snapshot 兼容前提, 禁 hipstr 进引擎字符串层) | GC/heap/sandbox (禁带全局分配器或线程假设的库)

引入流程: `cargo add` (dev 用 `-D`) → 登记 8.2 → Reviewer 核 8.3 → `cargo udeps` + `cargo shear` 无冗余。白名单外的库须在审查环给出依据 (性能数据 / 替代了哪段自研 / 语义边界)。

---

## 九、工具链与命令

- 任务放 `.mise/tasks/*.sh` (mise 自动发现), `sh/*` 为单一事实源; frontmatter 用 `#MISE description` / `#MISE depends` / `#MISE dir`
- 任务表: `build` `test` `clippy` `fmt` `check` `ci` (fmt+clippy+test) `new` `ctest` `fuzz` `diff` `miri` `mutants` `audit`；子代理门禁命令统一 `mise run ci`
- 用法: `mise run ci` / `mise tasks ls` / `mise run new -- v8-zone`
- `./test.sh` = 全仓 `cargo nextest run` (仅编排者收官用)
- `./sh/clippy.sh` 带 `--fix`, 子席位禁跑
- `./sh/udeps.sh` 冗余依赖; `./sh/unused.js` 未用 pub + 依赖冗余 (二)
- 批量转换工具 `tool/`, 目录结构镜像 `v8/`: 单文件 `v8/src/common/globals.h` → `tool/src/common/globals.h.js`; 同名族 (.h/.cc/-inl.h) 合一脚本
- 脚本规范: 首行 `#!/usr/bin/env bun`; 纯 JS 现代 ESM, 禁 TS; 只 import node 内置 + `tool/lib.js` (公共件, 编排者维护, 子代理禁改); 禁运行时依赖; 复跑幂等
- 脚本只产出规则代码 (枚举/常量表/位标志/类型别名/X-macro 展开/签名骨架) 直写目标 crate; 流程: 写脚本 → 跑出初版 → 对数核对 → Developer 就地修订语义 → `cargo nextest run -p <crate>` → 门禁。**修订后复跑脚本会覆盖修订**, 复跑仅限重生成基线
- 对数基准: 原版侧一律取编译器产物 (`clang -E` 或 `clang -Xclang -ast-dump=json`), 不取脚本自己的正则解析结果, 否则两边一起假绿
- 脚本产物不是成品: 仍走 Developer + Reviewer 审查环; 脚本入库
- 覆盖审计: `node tool/tool-coverage.js` → `golden/tool-coverage.json` (每轮末必跑, 含范围外口径); 范围外 = 非目标架构 (arm/ia32/loong64/mips64/ppc/riscv/s390 及其兄弟目录) 186 文件, 记入 `OUT_OF_SCOPE_DIRS`, 不计缺口不阻塞里程碑
- 性能: `node ./tool/benchRun.js` (v8-v7 套件 JSON 输出); PGO 见 [docs/pgo.md](docs/pgo.md) 与 `./tool/pgo_bench.sh` `./tool/pgo_train.js`
- 分析: `./tool/profile_sample.sh` (采样定位), samply + Firefox Profiler (JS 栈与原生栈混合热点), `dhat-rs` + rusty_alloc stats (堆外泄漏与高频临时分配)
- 当前基线 (轮6 重锚 2026-10-12): v8_total 3327 / 脚本 1690 / 已覆 3117 / 缺口 210 (范围内 0, 范围外 210) / 陈旧引用 0

---

## 十、C 应用程序接口兼容层

目标: 导出与 `v8/include/v8-*.h` C++ API 面 1:1 的 C ABI, 让 C++ 生态测试 (cctest/test-api) 原样编译链接到 Rust 实现。挂 M8, 硬依赖战役 G (v8-api 真实 trait 面)。

结构: `crate-type = ["cdylib","staticlib","rlib"]`; 全工程唯一 ABI 边界, unsafe 集中于此, 内部调 v8-api / rs8 安全层。`v8/include/v8-*.h` 每头 → `src/` 一模块, 1:1 命名。C++ 类 → 不透明 handle + C 函数; `Local<T>`/`Persistent<T>` → 显式 handle, 所有权与生命周期写进 SAFETY; `std::function` → C 函数指针 + `void* user_data`; `Maybe<T>`/`TryCatch` → 返回码 + out 参数。`build.rs` 用 cbindgen 生成单头 `include/rs8-capi.h`; ABI 冻结后语义版本化; 关键结构布局 static assert。

目录: `rs8-capi/{Cargo.toml, build.rs, src/{lib,handles,isolate,context,value,primitive,object,function,script,json,date,container,promise,proxy,regexp,typed_array,template,exception,message,platform,initialization,snapshot}.rs, include/rs8-capi.h, tests/, ctest/{main.cc,test-api.cc}}`

同源验证: `v8/test/cctest/test-api.cc` 用例 1:1 进 `ctest/` (类型替换 `v8::Local<T>` → capi handle, 断言流程原样); 构建 `mise run ctest` 编 `ctest/*.cc` 链 `librs8_capi.dylib` + gtest; 门禁双轨 `./test.sh` + `ctest` 全绿; 先迁 `unittests/api` (Rust 侧) 再分批迁 `cctest/test-api` (C++ 侧), 每批登记覆盖映射。

现状: 692 行, 仅封装简易 AST 解释器 (`rs8-capi/src/handles.rs:74` 贴 `v8-api` 常量表), 无 1:1 C ABI 面。

---

## 十一、Test262 官方标准测试

基础设施 (已落地):

- `./tool/test262/v8.py` — 对原版 V8 跑 Test262, 未通过用例记入 `tool/test262/v8_fail.yml`
- `./tool/test262/v8rs.py` — 读 `v8_fail.yml` 自动跳过 V8 未通过用例, 驱动 `rs8`, 产出 `test262_report.json` / `.md`
- 口径: 以 V8 原生实现为唯一基准, V8 通过的用例在 `rs8` 必须 100% 通过

待修清单:

- `- [x]` 11.1 加法运算符 (expressions/addition 48/48): BigInt/Number 混合 TypeError、ToPrimitive 装箱与原生 valueOf 映射、无显式 return 规范化 undefined、Symbol.toPrimitive 合法性与 getter 异常透传
- `- [ ]` 11.2 算术运算符 (subtraction/multiplication/division/modulus/exponentiation): BigInt 与 Number 语义与类型异常; NaN/Infinity/-0/+0 边界
- `- [ ]` 11.3 位运与移位 (bitwise-*/bitwise-shift): 32 位截断; BigInt 位运算
- `- [ ]` 11.4 相等比较 (equality/strict-equality/relational): `===`/`==` 规范; ToPrimitive 路径与优先级
- `- [ ]` 11.5 逻辑与赋值 (logical-*/assignment): 短路与左值求值顺序; 复合赋值解构
- `- [ ]` 11.6 声明与作用域 (variable/let/const): var 提升; let/const 块级与 TDZ
- `- [ ]` 11.7 分支循环 (switch/for/for-in/for-of/while/do-while): 迭代协议消费与结果解包; 带标签 break/continue
- `- [ ]` 11.8 异常控制流 (try): try-catch-finally 嵌套与 return/break/throw 优先级; Error 构造与原型链
- `- [ ]` 11.9 对象系统 (built-ins/Object): 装箱、create、defineProperty/getOwnPropertyDescriptor、原型查找与枚举
- `- [ ]` 11.10 函数反射 (built-ins/Function): call/apply/bind; arguments 长度、索引映射与迭代特性
- `- [ ]` 11.11 核心内建 (Array/String/Number/Boolean/Symbol/BigInt): 原型方法与规范一致

门禁: 每个套件先研读 `v8/src` 对应 C++ 实现 → `./tool/test262/v8rs.py` 目标目录 100% → `./test.sh` 无回归 → 子代理读 `.agents/skills/rust_review` 审查。**注意**: 当前 `rs8` 跑的是简易解释器, 战役 A 前修的都是将被删除的副本, 修前须确认该项是否在 bridge 的 8 个降级点上。

---

## 十二、性能评测、发布与对标

- `- [x]` 12.1 分配器现代化: 全 workspace 删 `tikv-jemallocator`, 统一 `rusty_alloc-api` 2.2.6; 适配 10 个评测 crate 的 `benches/main.rs` (与 2.3 `cargo shear` 报 `rusty_alloc-api` 未用存在矛盾, 见 D1)
- `- [ ]` 12.2 分析工具链: samply 混合栈热点 / rusty_alloc stats + dhat-rs 分配逃逸 / PMU 采 L1-I、L1-D miss 与分支预测失败 (重点 Ignition 分派循环)
- `- [x]` 12.3 微观基准: `v8-benchmarks` 对 js-engine-benchmark v8-v7 逐用例, 3 平台 + Linux ARM 输出 JSON 归档 (`.github/workflows/benchmark.yml`)。上一轮数值 (ns/op): crypto 1.77 / richards 0.70 / deltablue 2.47 / earley-boyer 1.17 / navier-stokes 332.10 / raytrace 1.18 / regexp 533.84 / splay 7.04 / string 7.17 / map-transition 0.17。**按 1.1 证据, 这组数走的是 `run_bytecode` 而非 CLI, 且未与 d8 同机对比 → 标记为待重测**
- `- [ ]` 12.4 宏观对标: `rs8`/`v8rs` 主干聚合, `lto=fat` + `codegen-units=1` 抹平跨包开销, 同机对标 `d8` (Speedometer 3.0 现实用户旅程, 严格区分 Cold Start 与 Steady State, 杜绝合成测试过拟合)
- `- [x]` 12.5 全平台发布: `.github/workflows/release.yml` 覆盖 Linux (x86_64/aarch64/armv7/riscv64gc, gnu+musl)、macOS (aarch64/x86_64)、Windows (x86_64/aarch64); `zig 0.13.0` + `cargo-zigbuild` 锁 glibc 2.17; `opt-level=3` `lto=fat` `codegen-units=1` `panic=abort` `strip=true`; 硬件旗标禁 `cpu=native` — x86_64 `+aes,+sse4.1`, aarch64 `+aes,+neon`
- `- [x]` 12.6 CI: `.github/workflows/ci.yml` 4 平台 (ubuntu x86_64 / ubuntu-24.04-arm / macos-latest / windows-latest), `checkout@v4` + `dtolnay/rust-toolchain@stable` + `taiki-e/install-action@v2`
- `- [x]` 12.7 在线大盘: Svelte 5 + Pug/Stylus + `@3-/stylus`/`lightningcss`; 多平台切换 + 原生 SVG 柱状 (Score / Score-MB 切换) + 中英切换 + 纯白设计; `pages.yml` 发布至 webc-site.github.io/v8rs
- `- [ ]` 12.8 上游接入: fork 至 webc-fork/js-engine-benchmark, 完成引擎注册 (`info.json`)、版本解析 (`scripts/update.ts`)、自动构建 (`.github/workflows/update.yml`); `Score/MB` 主张 — 纯 Rust 紧凑二进制 8~12MB vs V8 35~60MB 且 0 动态链接依赖; 待全平台 Actions 通过且实测跑分超越 V8 后向官方 ahaoboy/js-engine-benchmark 发 PR

**顺序约束**: 战役 A 未完成前, 12.3/12.4 的一切数字都不可用于"超越 V8"的论断。

---

## 十三、关键设计决策

- 对象模型: C++ tagged 指针 + 多继承 → `#[repr(transparent)]` 包装 + tagged enum; 堆布局与原版对齐 (snapshot 兼容前提); 对外安全, unsafe 集中 objects/heap 内部逐块标 SAFETY
- GC 保留原版算法: Scavenger / MinorMS / MarkCompact 1:1; 所有权冲突处 arena + 手写 visitor, 不引外部 GC 库
- Torque 重写为 Rust 代码生成器: `.tq` → Rust (原版是 `.tq` → CSA C++); 内建体系前置条件
- 架构范围: 先 x64 + arm64; ia32/mips64/ppc/s390/riscv/loong64 后置可选, 不阻塞里程碑
- `panic = "abort"`: 可失败路径走 `Result` (aok 风格 error.rs), 禁 panic/unwrap (100% 安全除外)
- zone 语义自研: bump 分配 + 作用域回收 + accounting; 内核可评估 bumpalo, 外部行为不变
- 测试 1:1 复刻: `v8/test/unittests` (按 src 目录镜像) 优先, 其次 `cctest`; dtoa 用 `unittests/gay-*` 黄金数据; 引擎级测试挂 rs8 按里程碑推进
- 构建配置钉死: Chromium 默认 x64/arm64 release, `v8_enable_pointer_compression=true` + `v8_enable_sandbox=true`, args 入库 `ref/args.gn`; 非目标配置不实现或 feature 隔离默认关
- 破环: objects/heap/execution/builtins 在 C++ 是一个环。下层只认 ID 与布局 (Tagged / Map 指针 / 常量), 不认上层类型; 需回调上层 (分配/写屏障/Isolate) → 下层定义窄 trait, 上层 impl, 泛型单态化零成本, **禁 `dyn`**; 一个环需 >5 个回调 trait 才能拆开 → 合并成单 crate 多模块, 不硬拆; 新建 L3/L4 crate 前先用 include 图出环报告再定拆法
- 布局金标准: 字段 offset 唯一真相 = 参考构建导出的 offset 表; 手写布局在 tests 逐项断言等于金标准; torque 链路通后生成布局同样对金标准断言, 再删手写副本

---

## 十四、风险与待决决策

风险:

- 最大风险 (本轮新增): 双实现长期并存 → 修的是将被删除的副本。对策 = 战役 A/B/C 优先于任何新功能, 未打通链路前不接 test262 新套件 (十一)
- "桩清零"被误读为"实现完成" → 1.4 读法警告 + 四 的规模列强制并列原版
- 冻结 crate 实际在链路外 → 四 的"链路"列 + 二.2 未用 pub 门禁
- C++ UB / 浮点边界语义漂移 → 1:1 测试迁移 + Reviewer 逐行比对原版
- unsafe 蔓延 → 每轮专项审查, 集中在 objects/heap 少数模块 + unsafe 棘轮 (7.3)
- 总量 187 万行周期长 → 里程碑分批冻结, 开发与审查重叠
- mjsunit/test262 长尾 → 起步目标 (M5 ≥80%), 不追求一次性 100%
- d8 语义差异误报 → rs8 输出格式对齐 `v8/src/d8`, message 测试逐条比对
- 独立审查同盲区 → 机器门禁先于 clean 计数, Reviewer 轮换模型与提示策略
- 测试假绿 → cargo-mutants 量化测试强度, 金标准来自 C++ 实跑
- 范围蔓延 → 非目标架构/配置不投入, 记 `OUT_OF_SCOPE_DIRS`

待决:

- Q1 M4 路线: A 严格 1:1 (先编译器后端子集再 CSA 生成 handler) | B 参考解释器 (match 分派, 测试专用不进发布, 作 Ignition handler 的差分 oracle)。**实测已按 B 落地 (v8-interpreter 真引擎 + rs8 简易副本), 但未把 B 收口成 A → 战役 A 就是收口动作, 建议维持 B 的实现 + 强制单一驱动路径**
- Q2 目标 gn 配置: 确认 pointer compression + sandbox 开启 (影响已冻结的 objects-core)
- Q3 破环阈值: 确认"回调 trait >5 就合并 crate"
- Q4 参考构建: 需 `depot_tools` 本机建一次 V8 (约 1 小时), 只产 `ref/` golden, 不进 Rust 构建链
- Q5 (新): 战役 A 切换后, 简易解释器副本保留为差分 oracle 还是立即删除? 建议保留一个版本周期, 打 feature 开关, 用于 test262 逐项对照, 战役 C 结束即删
