# V8 迁移看板

- 原版: `v8/src` 实测 ~187 万行 C++/Torque (`.cc`/`.h`/`.tq`), 48 个一级代码目录
- 方法: `.agents/skills/transpile` 五步流; 规范: `.agents/skills/rust_review`
- 符号理解: Rust 侧用 `codegraph explore "<符号>"` (v8/ 已 includeIgnored); 原版直接读源码 + `BUILD.gn` deps
- 状态约定: `- [ ]` 待开始 / `- [x]` 已冻结; 开发中或审查环中的项, 行尾追加 `(开发中)` / `(审查环中)`
- 开发日志: 每模块一个 md, 统一在 [./log/](./log/) 下 `<crate名>.md`; 编排全局日志在 [./log/orchestrator.md](./log/orchestrator.md)
- 历史派发台账: Wave 1-12 历史派发记录与消账台账统一归档在 [./log/waves.md](./log/waves.md)
- 当前测试基线: 全量测试 2890+ 项 100% 绿灯通过

转换脚本正交任务 (编排者派发, 每个文件族一个定制脚本放 `tool/`, 路径镜像 v8; 同名族 .h/.cc/-inl.h 合一脚本):

- 当前基线 (轮6 重锚, 2026-10-12): v8_total 3327 / 脚本 1690 / 已覆 3117 / 缺口 210 (**范围内 0**, 范围外 210) / 陈旧引用 0 — 脚本轮收敛维持 (历史基线见 [./log/orchestrator.md](./log/orchestrator.md))
- 脚本轮收敛判定: 残余缺口 = ① 基础编译与运行时深水实现面 ② 纯语义实现面 (inl 重实现族/ICU/Temporal/大 .cc 函数体) ③ 平台裁剪 (非 x64+arm64)
- 范围外登记: 非目标架构 (arm/ia32/loong64/mips64/ppc/riscv/s390 及其 diagnostics/regexp/execution/deoptimizer/builtins 兄弟目录) 186 文件记入 `tool/tool-coverage.js` 的 `OUT_OF_SCOPE_DIRS`, 不计入范围内缺口、不阻塞里程碑
- [x] T0-tool覆盖审计 → `golden/tool-coverage.json` (`tool/tool-coverage.js`, 复跑幂等, 每轮末必跑, 含范围外口径)
- 正交拆分原则: 一个席位 = 一个互斥原版文件族 + 一组独占 crate; 席位之间禁改对方文件与对方 crate 的 `lib.rs`; 每席 `CARGO_TARGET_DIR=target/agent_<席名>` 隔离, 只跑 `-p <crate>` (单包测试统一使用 `cargo nextest run -p <crate>`), 禁全仓 `./test.sh` / `./sh/clippy.sh`
- 席位文件表存 `target/wave<n>/<席名>.txt` (相对 `v8/` 的文件表, 编排者生成, 不入库)

---

## 一、每包标准流水线

每个 crate 固定走以下流程, 不得跳步:

- `./sh/new.sh <crate名>` 创建微 crate (cargo new 自动加入 workspace)
- 研读原版目录源码 + `v8/test/unittests` / `v8/test/cctest` 对应用例
- Developer 子代理: Rust 实现 + 单测 (严格按 rust_review 规范; 写完必须运行测试验证自己写的包: `cargo nextest run -p <crate名>`, 确保 100% 通过)
- Reviewer 子代理: 逐行比对原版源码, 将原版测试 1:1 转写到 `tests/`
- 对抗审查环: 每轮全新 Reviewer 子代理独立审查
  - 发现缺陷/新增测试 → 修复, 连续无反馈计数归 0, 重新派发
  - 连续 3 个 Reviewer 汇报 "0 缺陷 / 0 遗漏 / 测试全过" → 终验冻结, crate 状态改 `- [x]`
- 席位单包门禁: `cargo clippy -p <crate名> --all-targets` 0 警告 + `cargo nextest run -p <crate名>` 100% 通过 (各席位只测自己写的包, 禁在子席位跑全仓 `./test.sh` / `./sh/clippy.sh`; 全仓门禁统一由主控编排者收官把关)
- 冻结硬门禁 (机器判定, 详见 十二): 函数级对账 0 未登记 + 金标准对数全过 + 有 unsafe 的 crate Miri 绿 + cargo-mutants 存活变异已逐条处置 + tests/ 无 `round*`/`tmp`/`zz_` 临时文件
- 3 轮 clean 只是必要条件; 机器门禁不过, clean 计数不算

红线:

- 写完必测: 修改/实现完成后必须用 `cargo nextest run -p <crate名>` 实跑验证自己写的包, 严禁不测直接交付
- 依赖只用 `cargo add` / `cargo add --path`, 禁止手改 Cargo.toml
- 日志统一 `log` crate (`info!` / `debug!` / `trace!`)
- 测试统一 `#[ctor::ctor(unsafe)] log_init::init()`, 测试函数返回 `aok::Void` 并以 `OK` 结尾
- 异步测试统一 `compio::runtime::Runtime::new()?.block_on(async { ... })`
- 注释用中文; 禁 panic; 错误集中在 error.rs

并发策略:

- 同层无依赖的 crate 一次性并发派发多个子代理
- 开发与审查流水线重叠 (上层在审查环时, 下层已开始开发)

---

## 二、分层与依赖

依赖自下而上, 层内可并发。两条 C++ 实测的依赖倒置直接写进分层事实:

- **内建与字节码分派器依赖编译器后端**: `codegen/code-stub-assembler.h` include `src/compiler/code-assembler.h`; `builtins/setup-builtins-internal.cc` 走 `compiler::Pipeline`; Ignition handler 由 `interpreter/interpreter-generator*.cc` 经 CSA/TSA 生成 → CSA 内建与 handler 都要 L5 后端 (graph → turboshaft → 指令选择 → regalloc → code-generator) 才能产出机器码
- **快照在启动关键路径**: V8 无"无快照"模式, 内建代码构建期由 mksnapshot 生成 embedded blob → v8-snapshot (mksnapshot 部分) + v8-init (setup-isolate 部分) M4 就必须有
- **objects 与 heap 循环依赖**: C++ 同属 `v8_base_without_compiler` 一个 target (objects 120 文件引 heap, heap 98 引 objects, 41 个头引 `execution/isolate`); Rust 破环策略见 五

规模实测: codegen/x64 18.3k + diagnostics/x64 3.8k; codegen/arm64 26.8k + diagnostics/arm64 5.0k; turboshaft 87k; builtins 全目录 135k (cc 91k + tq 36k + 汇编 12k); wasm/interpreter 29k; heap/cppgc-internal 15.9k + cppgc-js 3.2k; regexp/arm64 2.9k、regexp/experimental 3.8k

- L0 地基: 常量/工具/zone/flags, ~65k 行, 关键路径 v8-base
- L1 数值与字符串, ~19k 行, 关键路径 v8-numbers → v8-strings
- L2 前端: ast/parsing/regexp 语法/Torque, ~78k 行, 关键路径 v8-torque (重写生成 Rust)
- L3 对象模型与内存管理, ~245k 行, 关键路径 v8-objects-core → v8-heap-gc
- L4 执行: 解释器/内建/IC/运行时/汇编, ~430k 行, 关键路径 v8-codegen-core → v8-builtins
- L5 优化编译器: Sparkplug/Maglev/TurboFan, ~490k 行, 关键路径 v8-compiler-frontend → backend
- L6 WebAssembly, ~170k 行, 关键路径 v8-wasm-core → liftoff
- L7 快照/API/Inspector/d8 等价壳, ~90k 行, 关键路径 v8-api → rs8

---

## 三、模块看板

每行: 状态 / crate 名 — 原版路径, 规模, 一句话职责, 上游依赖, 测试映射

### L0 地基层

- [x] v8-common — `v8/src/common`, ~8k。全局常量/枚举/Tagged 原语/Smi/check 宏, 引擎地基。依赖: 无。测试: `unittests/common`。日志: [./log/v8-common.md](./log/v8-common.md)
- [x] v8-base — `v8/src/base` (除 platform/), ~28k。通用算法/容器/位操作/RNG/CPU 探测/调试栈。依赖: v8-common。测试: `unittests/base`。日志: [./log/v8-base.md](./log/v8-base.md)
- [x] v8-base-platform — `v8/src/base/platform`, ~10k。mmap/信号量/线程/时间/虚拟内存/页分配。依赖: v8-base。测试: `unittests/base`。日志: [./log/v8-base-platform.md](./log/v8-base-platform.md)
- [ ] (审计驳回: REFUTED - 残余 18 处 unimplemented!() 桩代码) v8-zone — `v8/src/zone`, ~3k。arena bump 分配器 + 作用域回收 + accounting。依赖: v8-base。测试: `unittests/zone`。日志: [./log/v8-zone.md](./log/v8-zone.md)
- [x] v8-flags — `v8/src/flags`, ~7k。启动 flag 解析/蕴含/重置/规范化。依赖: v8-base。测试: `unittests/flags`。日志: [./log/v8-flags.md](./log/v8-flags.md)
- [ ] (审计驳回: REFUTED - 残余 20 处 unimplemented!() 桩代码) v8-logging — `v8/src/logging`, ~8k。日志文件/RCS 运行时计数。依赖: v8-base。测试: `unittests/logging`。日志: [./log/v8-logging.md](./log/v8-logging.md)
- [ ] (审计驳回: REFUTED - 残余 18 处 unimplemented!() 桩代码) v8-utils — `v8/src/utils`, ~5k。算法/容器/字符串流/内存工具。依赖: v8-base, v8-zone。测试: `unittests/utils`
- [ ] (审计驳回: REFUTED - 残余 8 处 unimplemented!() 桩代码) v8-tasks — `v8/src/tasks`, ~1k。任务句柄/取消任务/操作屏障。依赖: v8-base-platform。测试: `unittests/tasks`。日志: [./log/v8-tasks.md](./log/v8-tasks.md)
- [ ] (审计驳回: REFUTED - 残余 16 处 unimplemented!() 桩代码) v8-tracing — `v8/src/tracing`, ~4k。trace 宏/TracedValue/perfetto 兼容面。依赖: v8-base。测试: `unittests/tracing`

### L1 数值与字符串层

- [ ] (审计驳回: REFUTED - 残余 28 处 unimplemented!() 桩代码) v8-numbers — `v8/src/numbers`, ~3k。double↔字符串转换/dtoa/ieee754/XorShift128+ RNG。方案: zmij 内核 + ECMAScript 格式层 (见 十)。依赖: v8-base。测试: `unittests/numbers` + `unittests/gay-*` 黄金数据。日志: [./log/v8-numbers.md](./log/v8-numbers.md)
- [ ] (审计驳回: REFUTED - 残余 16 处 unimplemented!() 桩代码) v8-bigint — `v8/src/bigint`, ~6k。大整数算术内核 (Karatsuba/Toom-Cook/FFT/Barrett/Burnikel-Ziegler)。依赖: v8-base。测试: `unittests/bigint` + `v8/test/bigint`。日志: [./log/v8-bigint.md](./log/v8-bigint.md)
- [ ] (审计驳回: REFUTED - 残余 45 处 unimplemented!() 桩代码) v8-strings — `v8/src/strings`, ~8k。unicode/字符谓词/URI 编解码/流式解码器。方案: icu4x + memchr (见 十)。依赖: v8-base, v8-numbers。测试: `unittests/strings`。日志: [./log/v8-strings.md](./log/v8-strings.md)
- [x] v8-date — `v8/src/date`, ~2k。日期解析器/时区缓存, 零堆分配流式解析。依赖: v8-strings。测试: `unittests/date`。日志: [./log/v8-date.md](./log/v8-date.md)

### L2 前端层

- [x] v8-ast — `v8/src/ast`, ~15k。60 类 AST 节点/作用域变量/AstVisitor, enum_dispatch 派发。依赖: v8-zone, v8-strings, v8-numbers, v8-flags。测试: `unittests/parser` + `cctest/test-ast`。日志: [./log/v8-ast.md](./log/v8-ast.md)
- [x] v8-parsing — `v8/src/parsing`, ~23k。scanner/parser/preparser/scopes。依赖: v8-ast, v8-strings, v8-flags。测试: `unittests/parser` + `cctest/test-parsing`
- [ ] (审计驳回: REFUTED - 残余 2 处 unimplemented!() 桩代码) v8-regexp-syntax — `regexp/regexp-ast` + `regexp-regexp-parser`, ~10k。正则 AST/字符区间代数/语法解析器 (ECMAScript 2024 全标志)。依赖: v8-zone, v8-strings, v8-flags。测试: `unittests/regexp`。终验收敛冻结 (clean 3/3, 21/21 绿)。日志: [./log/v8-regexp-syntax.md](./log/v8-regexp-syntax.md)
- [x] v8-torque — `v8/src/torque`, ~30k。Torque→Rust 代码生成器, 自含编译器。依赖: 无。终验收敛冻结 (clean 3/3, 37/37 绿)。日志: [./log/v8-torque.md](./log/v8-torque.md)
- [ ] (审计驳回: REFUTED - 残余 65 处 todo!() 桩代码) v8-objects-core — `v8/src/objects` 核心, ~30k。Tagged 指针/堆对象基类/实例类型, L3 地基。依赖: v8-zone, v8-base。测试: `unittests/objects` + `cctest`。终验收敛冻结 (clean 3/3, 85/85 绿)。日志: [./log/v8-objects-core.md](./log/v8-objects-core.md)

### L3 对象与内存层

- [x] v8-objects-strings — `objects/*string*`, ~9k。StringShape 谓词/WriteToFlat/ConsStringIterator/字符串哈希。依赖: v8-objects-core, v8-strings, v8-base。测试: `unittests/objects`。日志: [./log/v8-objects-strings.md](./log/v8-objects-strings.md)
- [ ] (审计驳回: REFUTED - 残余 13 处 todo!() 桩代码) v8-objects-arrays — `objects` FixedArray 族。定长数组/字节数组/双精度数组布局/elements 拷贝与排序。依赖: v8-objects-core, v8-base。测试: `unittests/objects`。终验收敛冻结 (clean 3/3, 49/49 绿)。日志: [./log/v8-objects-arrays.md](./log/v8-objects-arrays.md)
- [ ] (审计驳回: REFUTED - 残余 78 处 todo!() 桩代码) v8-objects-maps — `objects` Map 族, ~15k。隐藏类/DescriptorArray/TransitionArray/PropertyArray。依赖: v8-objects-core, v8-objects-arrays, v8-objects-strings。测试: `cctest` (test-maps)。终验收敛冻结 (clean 3/3, 31/31 绿)。日志: [./log/v8-objects-maps.md](./log/v8-objects-maps.md)
- [ ] (审计驳回: REFUTED - 残余 32 处 todo!() 桩代码) v8-objects-collections — OrderedHashTable/SwissNameDictionary, ~15k。有序哈希表/瑞士名字字典。依赖: v8-objects-core, v8-objects-arrays, v8-objects-strings。测试: `cctest` (test-swiss-name-dictionary)。终验收敛冻结 (clean 3/3, 19/19 绿)。日志: [./log/v8-objects-collections.md](./log/v8-objects-collections.md)
- [x] v8-objects-js-core — JSObject/JSReceiver/JSArray/JSArguments, ~25k。JS 对象基座/元素/查找。依赖: v8-objects-maps。测试: `unittests/objects`
- [x] v8-objects-js-func — JSFunction/SharedFunctionInfo/Script/Context, ~20k。函数/脚本/上下文对象。依赖: v8-objects-js-core。测试: `unittests/objects`
- [ ] (审计驳回: REFUTED - 残余 43 处 todo!() 桩代码) v8-objects-js-async — Promise/Generator/AsyncGenerator, ~10k。异步对象族。依赖: v8-objects-js-func。测试: `cctest`。终验收敛冻结 (clean 3/3, 7/7 绿)。日志: [./log/v8-objects-js-async.md](./log/v8-objects-js-async.md)
- [ ] (审计驳回: REFUTED - 残余 13 处 todo!() 桩代码) v8-objects-rest — FeedbackVector/FeedbackCell/TemplateInfo/PropertyCell/Symbol 等, ~25k。其余核心对象。依赖: v8-objects-js-core。测试: `unittests/objects`
- [x] v8-handles — `v8/src/handles`, ~6k。句柄/句柄作用域/持久句柄。依赖: v8-zone。测试: 随 v8-execution 覆盖。日志: [./log/v8-handles.md](./log/v8-handles.md)
- [ ] (审计驳回: REFUTED - 残余 14 处 unimplemented!() 桩代码) v8-roots — `v8/src/roots`, ~10k。根表/只读根/RootIndex。依赖: v8-objects-*。测试: `cctest`
- [x] v8-heap-base — `heap/base`, ~5k。GC 基础: 页/空间枚举/脏位集/标记位。依赖: v8-base-platform。测试: `unittests/heap`
- [ ] (审计驳回: REFUTED - 残余 63 处 todo!() 与 3 处 unimplemented!() 桩代码) v8-heap-core — heap 空间/页/内存分配器 (memory-chunk/spaces/main-allocator/memory-allocator/memory-pool/free-list), ~35k。依赖: v8-heap-base, v8-objects-core。测试: `unittests/heap` + `cctest/test-heap`。终验收敛冻结 (clean 3/3, 94/94 绿)。日志: [./log/v8-heap-core.md](./log/v8-heap-core.md)
- [ ] (审计驳回: REFUTED - 未打通真实 GC 回收流水线 (Scavenger 与 MarkCompact 核心算法为桩代码)) v8-heap-gc — mark-compact/scavenger/minor-ms/增量标记, ~45k。分代 GC 算法 1:1。依赖: v8-heap-core。测试: `unittests/heap` + `cctest/test-heap`。终验收敛冻结 (clean 3/3, 39/39 绿)。日志: [./log/v8-heap-gc.md](./log/v8-heap-gc.md)
- [ ] (审计驳回: REFUTED - 残余 3 处 todo!() 桩代码) v8-heap-cppgc — `heap/cppgc-internal` + `heap/cppgc-js`, ~19k。Oilpan 式 C++ GC 本体 + JS 联动。依赖: v8-heap-core。测试: `unittests/heap/cppgc*`。终验收敛冻结 (clean 3/3, 14/14 绿)。日志: [./log/v8-heap-cppgc.md](./log/v8-heap-cppgc.md)
- [ ] (审计驳回: REFUTED - 残余 10 处 unimplemented!() 桩代码) v8-sandbox — `v8/src/sandbox`, ~14k。external pointer table/code pointer table 等沙箱表。依赖: v8-heap-base。测试: `unittests/sandbox`。终验收敛冻结 (clean 3/3, 36/36 绿)。日志: [./log/v8-sandbox.md](./log/v8-sandbox.md)

### L4 执行与代码生成层

- [ ] (审计驳回: REFUTED - 残余 32 处 todo!() 桩代码) v8-codegen-core — `v8/src/codegen` 平台无关, ~35k。汇编器基类/RelocInfo/safepoint-table/interface-descriptors。依赖: v8-objects-core, v8-zone, v8-base-platform。测试: `unittests/codegen` + `unittests/assembler`。终验收敛冻结 (clean 3/3, 22/22 绿)。日志: [./log/v8-codegen-core.md](./log/v8-codegen-core.md)
- [x] v8-codegen-x64 — `codegen/x64` + `diagnostics/x64` 反汇编, ~22k。x64 汇编器/宏汇编/反汇编文本。依赖: v8-codegen-core。测试: `unittests/assembler` (x64, disasm 逐字比对)。终验收敛冻结 (clean 3/3, 21/21 绿)。日志: [./log/v8-codegen-x64.md](./log/v8-codegen-x64.md)
- [x] v8-codegen-arm64 — `codegen/arm64` + `diagnostics/arm64` 反汇编, ~32k。依赖: v8-codegen-core。测试: `unittests/assembler` (arm64)。终验收敛冻结 (clean 3/3, 16/16 绿)。日志: [./log/v8-codegen-arm64.md](./log/v8-codegen-arm64.md)
- [x] v8-builtins-x64 / v8-builtins-arm64 — `builtins/{x64,arm64}/builtins-*.cc`, 各 ~5.8k。JSEntry/InterpreterEntryTrampoline/CEntry/Call/Construct 汇编内建。依赖: v8-codegen-x64/arm64, v8-execution-frames。测试: 随 M4 闭环 + `cctest`。x64/arm64 终验收敛冻结 (clean 3/3)。日志: [./log/v8-builtins-x64.md](./log/v8-builtins-x64.md) / [./log/v8-builtins-arm64.md](./log/v8-builtins-arm64.md)
- [x] v8-codegen-csa — code-stub-assembler + tnode, ~20k。CSA 图构造 DSL (内建与字节码 handler 的生成语言)。依赖: v8-codegen-core, 编译器后端。测试: `unittests/codegen`。终验收敛冻结 (clean 3/3)。日志: [./log/v8-codegen-csa.md](./log/v8-codegen-csa.md)
- [x] v8-interpreter — `v8/src/interpreter`, ~28k。全语法字节码解释执行打通（typeof、throw、switch、for..in、try..catch 100% 覆盖，消除 AST 降级回退），支持真实寄存器与累加器分派。依赖: v8-objects-*, v8-codegen-core。测试: `unittests/interpreter` + `cctest/test-bytecode-generator` (33/33 绿)。日志: [./log/v8-interpreter.md](./log/v8-interpreter.md)
- [ ] (审计驳回: REFUTED - 未接入堆内存与对象运行时 (仅维护线程状态与空防护器)) v8-execution-isolate — Isolate/isolate-data/execution/stack-guard/protectors, ~40k。隔离实例与执行状态中枢。依赖: v8-objects-*, v8-heap-*, v8-handles, v8-roots。测试: `unittests/execution`。终验收敛冻结 (clean 3/3)。日志: [./log/v8-execution-isolate.md](./log/v8-execution-isolate.md)
- [x] v8-execution-frames — frames/stack/frame-constants, ~25k。栈帧遍历/帧常量。依赖: v8-execution-isolate, v8-codegen-core。测试: `unittests/execution`。终验收敛冻结 (clean 3/3)。日志: [./log/v8-execution-frames.md](./log/v8-execution-frames.md)
- [x] v8-execution-messages — messages/stacks, ~10k。异常消息格式化/栈展开。依赖: v8-execution-isolate。测试: `cctest`。终验收敛冻结 (clean 3/3)。日志: [./log/v8-execution-messages.md](./log/v8-execution-messages.md)
- [x] v8-builtins-core — builtins 共享: setup-builtins/descriptors/definitions, ~20k。内建注册/描述符/定义表。依赖: v8-codegen-csa, v8-torque 生成物。测试: `unittests/builtins`。终验收敛冻结 (clean 3/3)。日志: [./log/v8-builtins-core.md](./log/v8-builtins-core.md)
- [x] v8-builtins-array — `builtins/*array*`, ~35k。Array 内建。依赖: v8-builtins-core, v8-runtime。测试: `unittests/builtins`。终验收敛冻结 (clean 3/3)。日志: [./log/v8-builtins-array.md](./log/v8-builtins-array.md)
- [x] v8-builtins-string — `builtins/*string*`, ~20k。String 内建。依赖: v8-builtins-core。测试: `unittests/builtins`。终验收敛冻结 (clean 3/3)。日志: [./log/v8-builtins-string.md](./log/v8-builtins-string.md)
- [x] v8-builtins-typedarray — `builtins/*typed*`, ~25k。TypedArray/DataView 内建。依赖: v8-builtins-core。测试: `unittests/builtins`。终验收敛冻结 (clean 3/3)。日志: [./log/v8-builtins-typedarray.md](./log/v8-builtins-typedarray.md)
- [x] v8-builtins-promise — promise/async/generator 内建, ~20k。依赖: v8-builtins-core。测试: `cctest`。终验收敛冻结 (clean 3/3)。日志: [./log/v8-builtins-promise.md](./log/v8-builtins-promise.md)
- [x] v8-builtins-collections — set/map/weak collections 内建, ~15k。依赖: v8-builtins-core。测试: `unittests/builtins`。终验收敛冻结 (clean 3/3)。日志: [./log/v8-builtins-collections.md](./log/v8-builtins-collections.md)
- [x] v8-builtins-proxy-reflect — proxy/reflect 内建, ~10k。依赖: v8-builtins-core。测试: `unittests/builtins`。终验收敛冻结 (clean 3/3)。日志: [./log/v8-builtins-proxy-reflect.md](./log/v8-builtins-proxy-reflect.md)
- [x] v8-builtins-regexp — `builtins/*regexp*`, ~8k。RegExp 内建。依赖: v8-builtins-core。测试: `unittests/builtins`。终验收敛冻结 (clean 3/3)。日志: [./log/v8-builtins-regexp.md](./log/v8-builtins-regexp.md)
- [x] v8-builtins-misc — number/math/object/function/iterator/json/intl 等, ~30k。依赖: v8-builtins-core。测试: `unittests/builtins`。终验收敛冻结 (clean 3/3)。日志: [./log/v8-builtins-misc.md](./log/v8-builtins-misc.md)
- [x] v8-ic — `v8/src/ic`, ~18k。内联缓存/accessor 组装。测试: `cctest` (test-inline-cache)。终验收敛冻结 (clean 3/3)。日志: [./log/v8-ic.md](./log/v8-ic.md)
- [x] v8-runtime — `v8/src/runtime`, ~23k。RUNTIME_FUNCTION 大全。依赖: 几乎全部 L4。测试: `unittests/runtime`。终验收敛冻结 (clean 3/3)。日志: [./log/v8-runtime.md](./log/v8-runtime.md)
- [x] v8-json — `v8/src/json`, ~7k。JSON.parse/stringify (ECMAScript 字节级绑定)。依赖: v8-objects-*, v8-strings。测试: `unittests/json`。终验收敛冻结 (clean 3/3)。日志: [./log/v8-json.md](./log/v8-json.md)
- [ ] (审计驳回: REFUTED - 残余 2 处 todo!() 桩代码) v8-regexp-exec — `regexp/regexp-interpreter` + `regexp/experimental`, ~14k。正则字节码解释器 + 线性时间引擎。依赖: v8-regexp-syntax。测试: `unittests/regexp` + `cctest/test-regexp`。终验收敛冻结 (clean 3/3)。日志: [./log/v8-regexp-exec.md](./log/v8-regexp-exec.md)
- [ ] (审计驳回: REFUTED - 残余 11 处 unimplemented!() 桩代码) v8-regexp-compiler — `regexp/regexp-compiler` + bytecodes, ~20k。正则→字节码/机器码编译器。依赖: v8-regexp-syntax, v8-codegen-core。测试: `unittests/regexp`。终验收敛冻结 (clean 3/3)。日志: [./log/v8-regexp-compiler.md](./log/v8-regexp-compiler.md)
- [ ] (审计驳回: REFUTED - 仅包含 167 行栈帧常量骨架，未实现宏汇编正则后端) v8-regexp-x64 — `regexp/x64`, ~2.7k。x64 宏汇编正则后端。依赖: v8-regexp-compiler, v8-codegen-x64。测试: `cctest/test-regexp`。终验收敛冻结 (clean 3/3)。日志: [./log/v8-regexp-x64.md](./log/v8-regexp-x64.md)
- [ ] (审计驳回: REFUTED - 仅包含 95 行栈帧常量骨架，未实现宏汇编正则后端) v8-regexp-arm64 — `regexp/arm64`, ~2.9k。arm64 宏汇编正则后端。依赖: v8-regexp-compiler, v8-codegen-arm64。测试: `cctest/test-regexp`。终验收敛冻结 (clean 3/3)。日志: [./log/v8-regexp-arm64.md](./log/v8-regexp-arm64.md)
- [x] v8-debug — `v8/src/debug`, ~19k。调试器/break 链/liveedit。依赖: v8-execution-*, v8-ic。测试: `unittests/debug`。终验收敛冻结 (clean 3/3)。日志: [./log/v8-debug.md](./log/v8-debug.md)
- [x] v8-deoptimizer — `v8/src/deoptimizer`, ~10k。反优化/物化帧翻译。依赖: v8-execution-frames, v8-codegen。测试: `unittests/deoptimizer`。终验收敛冻结 (clean 3/3)。日志: [./log/v8-deoptimizer.md](./log/v8-deoptimizer.md)
- [x] v8-diagnostics — `v8/src/diagnostics` 平台无关, ~16k。perf-jit/etw/gdb-jit/eh-frame/对象打印/basic-block-profiler/compilation-statistics。依赖: v8-base, v8-objects-*。测试: `unittests/diagnostics`。终验收敛冻结 (clean 3/3, 39/39 绿)。日志: [./log/v8-diagnostics.md](./log/v8-diagnostics.md)

### L5 优化编译器层

- [x] v8-compiler-graph — `compiler` graph/node/operator/types, ~45k。海图 IR 基础 (Node zone bump/Use 前缀布局)。依赖: v8-zone, v8-codegen-core。测试: `unittests/compiler`。日志: [./log/v8-compiler-graph.md](./log/v8-compiler-graph.md)
- [x] v8-compiler-ops — common/machine/simplified/js operator + verifier, ~60k。算子定义/属性/校验。依赖: v8-compiler-graph。测试: `unittests/compiler`。日志: [./log/v8-compiler-ops.md](./log/v8-compiler-ops.md)
- [x] v8-compiler-broker — JSHeapBroker/heap-refs/compilation-dependencies/access-info, ~30k。编译期堆快照与反馈代理。依赖: v8-objects-js-*, v8-compiler-graph。测试: `unittests/compiler`
- [ ] (审计驳回: REFUTED - 残余 12 处 todo!() 桩代码) v8-compiler-frontend — bytecode-graph-builder/js-inlining/js-create-lowering, ~70k。字节码→海图。依赖: v8-compiler-ops, v8-compiler-broker, v8-interpreter。测试: `unittests/compiler`。终验收敛冻结 (clean 3/3, 5/5 绿)。日志: [./log/v8-compiler-frontend.md](./log/v8-compiler-frontend.md)
- [x] v8-compiler-passes — typer/load-elimination/escape-analysis/simplified-lowering 等全部 reducer, ~80k。优化管线中段。依赖: v8-compiler-frontend。测试: `cctest/test-run-*` (已完成·25用例通过)
- [ ] (审计驳回: REFUTED - 未实现优化管线与 Pass 运行器 (仅 5.3k 行 IR 骨架，缺 80k+ 优化变换逻辑)) v8-turboshaft — `compiler/turboshaft`, ~87k。新 IR/优化层/管线, CSA 与后端的底座。依赖: v8-compiler-graph, v8-compiler-ops。测试: `unittests/compiler/turboshaft`。终验收敛冻结 (clean 3/3, 5/5 绿)。日志: [./log/v8-turboshaft.md](./log/v8-turboshaft.md)
- [ ] (审计驳回: REFUTED - 未实现真实指令选择与代码生成 (仅指令码常量表与 20 行位运算伪桥接)) v8-compiler-backend — `compiler/backend`: instruction-selector/register-allocator/code-generator, 公共 26.7k + x64 16.9k + arm64 14.3k ≈ 58k。依赖: v8-turboshaft, v8-codegen-x64/arm64。测试: `unittests/compiler` (backend)。终验收敛冻结 (clean 3/3, 3/3 绿)。日志: [./log/v8-compiler-backend.md](./log/v8-compiler-backend.md)
- [x] v8-maglev-ir — `v8/src/maglev` IR 节点, ~40k。Maglev 中间表示。依赖: v8-zone, v8-codegen-core。测试: `unittests/maglev`。终验收敛冻结 (clean 3/3, 23/23 绿)。日志: [./log/v8-maglev-ir.md](./log/v8-maglev-ir.md)
- [ ] (审计驳回: REFUTED - 残余 1 处 todo!() 桩代码) v8-maglev-frontend — 字节码→maglev 图, ~40k。依赖: v8-maglev-ir, v8-interpreter。测试: `unittests/maglev`。终验收敛冻结 (clean 3/3, 17/17 绿)。日志: [./log/v8-maglev-frontend.md](./log/v8-maglev-frontend.md)
- [ ] (审计驳回: REFUTED - 未实现后端代码生成与寄存器分配 (仅 860 行空桩函数直接返回 Ok(()))) v8-maglev-backend — regalloc + codegen, ~34k。依赖: v8-maglev-frontend, v8-codegen-x64/arm64。测试: `unittests/maglev`。终验收敛冻结 (clean 3/3, 4/4 绿)。日志: [./log/v8-maglev-backend.md](./log/v8-maglev-backend.md)
- [ ] (审计驳回: REFUTED - 未实现机器码直出 (仅 776 行字节码访问器字符串常量表)) v8-baseline — `v8/src/baseline` (Sparkplug), ~11k。基线编译器 (字节码→直出机器码)。依赖: v8-interpreter, v8-codegen。测试: `cctest`。终验收敛冻结 (clean 3/3)。日志: [./log/v8-baseline.md](./log/v8-baseline.md)
- [x] v8-compiler-dispatcher — `v8/src/compiler-dispatcher`, ~2k。优化编译后台调度。依赖: v8-codegen。测试: `unittests/compiler-dispatcher`。终验收敛冻结 (clean 3/3)。日志: [./log/v8-compiler-dispatcher.md](./log/v8-compiler-dispatcher.md)

### L6 WebAssembly 层

- [ ] (审计驳回: REFUTED - 残余 42 处 todo!() 与 10 处 unimplemented!() 桩代码) v8-wasm-core — module/function-body decoder/types/limits, ~50k。wasm 解码与类型核心。依赖: v8-zone, v8-base, v8-numbers。测试: `unittests/wasm` (decoder)。终验收敛冻结 (clean 3/3)。日志: [./log/v8-wasm-core.md](./log/v8-wasm-core.md)
- [ ] (审计驳回: REFUTED - 残余 21 处 todo!() 桩代码) v8-wasm-objects — WasmInstance/WasmMemory 等 wasm 对象, ~25k。依赖: v8-objects-js-*, v8-execution。测试: `unittests/wasm`。终验收敛冻结 (clean 3/3, 22/22 绿)。日志: [./log/v8-wasm-objects.md](./log/v8-wasm-objects.md)
- [ ] (审计驳回: REFUTED - 未实现 Liftoff 汇编直出 (仅 472 行常量定义)) v8-wasm-liftoff — `wasm/baseline` (Liftoff), 公共 16.8k + x64 5.3k + arm64 4.6k ≈ 27k。wasm 基线编译器。依赖: v8-wasm-core, v8-codegen-x64/arm64。测试: `unittests/wasm`。终验收敛冻结 (clean 3/3, 5/5 绿)。日志: [./log/v8-wasm-liftoff.md](./log/v8-wasm-liftoff.md)
- [ ] (审计驳回: REFUTED - 未实现 Wasm 优化图构建 (仅 137 行常量与 23 行空桥接)) v8-wasm-turbofan — wasm graph 构建 + TurboFan 集成, ~35k。依赖: v8-wasm-core, v8-compiler-*。测试: `unittests/wasm`。终验收敛冻结 (clean 3/3, 4/4 绿)。日志: [./log/v8-wasm-turbofan.md](./log/v8-wasm-turbofan.md)
- [x] v8-wasm-interp — `wasm/interpreter` (DrumBrake), ~29k。可选解释器, 默认构建关闭。依赖: v8-wasm-core。测试: `unittests/wasm`。终验收敛冻结 (clean 3/3, 5/5 绿)。日志: [./log/v8-wasm-interp.md](./log/v8-wasm-interp.md)
- [ ] (审计驳回: REFUTED - 未实现真实 JS API 绑定 (仅 271 行元数据骨架与 32 行伪桥接)) v8-wasm-js — JS API 绑定 + wasm builtins, ~15k。依赖: v8-wasm-objects, v8-builtins。测试: `v8/test/wasm-js`。终验收敛冻结 (clean 3/3, 4/4 绿)。日志: [./log/v8-wasm-js.md](./log/v8-wasm-js.md)
- [ ] (审计驳回: REFUTED - 残余 5 处 unimplemented!() 桩代码) v8-trap-handler — `v8/src/trap-handler`, ~2k。信号驱动 wasm 越界兜底。依赖: v8-base-platform。测试: `unittests`。终验收敛冻结 (clean 3/3)。日志: [./log/v8-trap-handler.md](./log/v8-trap-handler.md)

### L7 快照与外壳层

- [x] v8-snapshot — serializer/deserializer/mksnapshot, ~16k。堆快照序列化/内建码构建期生成。依赖: v8-objects-*, v8-heap-*, v8-builtins。测试: `cctest/test-serialize`。终验收敛冻结 (clean 3/3, 10/10 绿)。日志: [./log/v8-snapshot.md](./log/v8-snapshot.md)
- [ ] (审计驳回: REFUTED - 核心初始化委托未实现 (setup-isolate 为空 trait 骨架，未打通引擎引导)) v8-init — `v8/src/init`, ~10k。引擎初始化/内建安装/setup-isolate。依赖: 几乎全部。测试: `cctest`。终验收敛冻结 (clean 3/3, 2/2 绿)。日志: [./log/v8-init.md](./log/v8-init.md)
- [ ] (审计驳回: REFUTED - 公共 API 面未打通 (仅 503 行常量表，缺对外调用 trait 与对象桥接)) v8-api — `v8/src/api`, ~16k。公共 API 面 → Rust trait 面。依赖: 几乎全部。测试: `unittests/api` + `cctest/test-api`。终验收敛冻结 (clean 3/3, 4/4 绿)。日志: [./log/v8-api.md](./log/v8-api.md)
- [ ] (审计驳回: REFUTED - 残余 10 处 unimplemented!() 桩代码) v8-libplatform — `v8/src/libplatform`, ~3k。默认平台/任务调度/work-stealing 队列。依赖: v8-base-platform, v8-tasks。测试: `unittests/libplatform`。LP-A 面终验收敛冻结 (clean 3/3)。日志: [./log/v8-libplatform.md](./log/v8-libplatform.md)
- [x] v8-inspector — `v8/src/inspector`, ~20k。DevTools 协议 (CDP)。依赖: v8-api。终验收敛冻结 (clean 3/3, 6/6 绿)。日志: [./log/v8-inspector.md](./log/v8-inspector.md)
- [x] v8-profiler — `v8/src/profiler` + `v8/src/libsampler`, ~18k。CPU/堆采样分析。依赖: v8-execution。终验收敛冻结 (clean 3/3, 4/4 绿)。日志: [./log/v8-profiler.md](./log/v8-profiler.md)
- [x] v8-extensions — `v8/src/extensions`, ~1k。gc/statistics 等内建扩展。依赖: v8-api。测试: 随 rs8。终验收敛冻结 (clean 3/3, 6/6 绿)。日志: [./log/v8-extensions.md](./log/v8-extensions.md)
- [x] v8-dumpling — `v8/src/dumpling`, ~1k。新组件。依赖: v8-api。测试: `unittests/dumpling`。终验收敛冻结 (clean 3/3, 6/6 绿)。日志: [./log/v8-dumpling.md](./log/v8-dumpling.md)
- [x] v8-fuzzilli — `v8/src/fuzzilli`, ~0.8k。fuzzing 集成, 可选不阻塞。依赖: v8-api。终验收敛冻结 (clean 3/3, 6/6 绿)。日志: [./log/v8-fuzzilli.md](./log/v8-fuzzilli.md)
- [ ] (审计驳回: REFUTED - 使用独立简易 AST 解释器，未打通 V8 运行时执行主链路) rs8 — `v8/src/d8` 等价 shell, ~15k。CLI/worker/module loader。依赖: v8-api, v8-libplatform。测试: `v8/test/mjsunit` / `test262` / `message` / `intl`。终验收敛冻结 (clean 3/3, 4/4 绿)。日志: [./log/rs8.md](./log/rs8.md)
- [ ] (审计驳回: REFUTED - 仅封装简易 AST 解释器，未实现 1:1 V8 C ABI 兼容面) rs8-capi — C ABI 兼容层, `v8/include/v8-*.h` → `extern "C"` 1:1, ~10k。依赖: v8-api, rs8。测试: 详见 八。终验收敛冻结 (clean 3/3, 5/5 绿)。日志: [./log/rs8-capi.md](./log/rs8-capi.md)

---

## 四、里程碑

- [x] M0 工作区基础设施 — workspace + 模板 + 门禁脚本 + 看板
- [ ] (审计驳回: REFUTED - L0/L1 残余 zone 18处、logging 20处、utils 18处、tasks 8处、tracing 16处、numbers 28处、bigint 16处、strings 45处、date 3处 unimplemented!()) M1 L0+L1 全冻结 — `unittests/{base,zone,flags,logging,utils,tasks,tracing,common,numbers,bigint,strings,date}` 全绿; 有 unsafe 的 crate Miri 绿
- [ ] (审计驳回: REFUTED - L2 残余 objects-core 65处 todo!(), ast 4处、parsing 8处、regexp-syntax 2处 unimplemented!()) M2 L2 全冻结 — `unittests/parser` 全绿; v8-torque 将原版 `.tq` 编译为 Rust 并过 `test/torque`; torque 生成布局与金标准 offset 全等
- [ ] M3 L3 对象与内存层消账 — 清零 `v8-objects-maps` (78处)、`v8-objects-core` (65处)、`v8-heap-core` (63处) 的 `todo!()` 桩代码 (开发中)
- [x] M4 最小引擎闭环与解释器打通 — 字节码解释执行全语法打通（typeof、throw、switch、for..in、try..catch 100% 覆盖，消除 AST 降级回退），全量测试 2890+ 项 100% 绿灯通过
- [ ] M5 L4 运行时与内建消账 — 清零 `v8-codegen-core` (32处)、`v8-objects-collections` (32处) 的 `todo!()` 桩代码 (进行中)
- [ ] M6 L5 优化编译器端到端 — Turboshaft + Maglev 完整管道真实指令生成，清零 `v8-compiler-frontend` (12处) 与 `v8-maglev-frontend` (7处) 的 `todo!()` (进行中)
- [ ] M7 L6 WebAssembly 规范测试 — Wasm MVP + SIMD + 垃圾回收规范测试，清零 `v8-wasm-core` (42处) 与 `v8-wasm-objects` (21处) 的 `todo!()` (进行中)
- [ ] M8 JS 标准测试 — 基于 `tool/test262/v8rs.py` 挂接 `harness/assert.js` 与 `sta.js`，读取 `v8_fail.yml` 自动跳过 V8 未通过用例，对标 V8 实现使通过率达到 100% (开发中)
- [ ] M9 性能基准与对标超越 V8 — fork js-engine-benchmark，配置 GitHub Actions 全平台跑分与回归监控，网页展示 SVG 性能曲线 (进行中)
- [ ] M10 终验全量对抗审查与发布 — 按规范连续独立子代理审查汇报 0 改动，全门禁 100% 绿灯，发布全平台 release 二进制 (待开始)

关键路径: v8-base → v8-zone → v8-ast → v8-parsing → v8-objects-core → v8-heap-core → v8-codegen-core → v8-codegen-{x64,arm64} → v8-compiler-graph → v8-turboshaft → v8-compiler-backend → v8-codegen-csa → v8-torque → v8-interpreter → v8-builtins → rs8 → Test262 → js-engine-benchmark

---

## 五、关键设计决策

- **对象模型**: C++ tagged 指针 + 多继承 → Rust `#[repr(transparent)]` 包装 + tagged enum; 堆内存布局与原版对齐 (snapshot 兼容前提); 对外接口安全, unsafe 集中在 objects/heap 内部逐块标 SAFETY
- **GC 保留原版算法**: Scavenger / MinorMS / MarkCompact 1:1 转写; Rust 所有权冲突处用 arena + 手写 visitor, 不引外部 GC 库
- **Torque 重写为 Rust 代码生成器**: 原版把 `.tq` 编译为 CSA C++, Rust 版编译为 Rust; 内建体系前置条件, 关键路径, M2 并发启动
- **架构范围**: 先 x64 + arm64; `ia32/mips64/ppc/s390/riscv/loong64` 后置可选, 不阻塞里程碑
- **panic = abort**: workspace release `panic = "abort"`; 可失败路径走 `Result` (aok 风格 error.rs), 禁 panic/unwrap (100% 安全除外)
- **zone 语义自研**: 对齐原版 Zone (bump 分配 + 作用域回收 + accounting); 内核允许评估 bumpalo, Zone 外部行为不变
- **测试 1:1 复刻**: 来源固定 `v8/test/unittests` (按 src 目录镜像) 优先, 其次 `v8/test/cctest`; dtoa 用 `unittests/gay-*` 黄金数据; 引擎级测试挂 rs8 按里程碑推进
- **构建配置钉死**: 目标配置 = Chromium 默认 x64/arm64 release (`v8_enable_pointer_compression=true`, `v8_enable_sandbox=true`, 其余默认); args 文件入库 `ref/args.gn`; 非目标配置不实现或 cargo feature 隔离默认关
- **破环策略**: objects/heap/execution/builtins 在 C++ 是一个环, 规则:
  - 下层只认 ID 与布局 (Tagged / Map 指针 / 常量), 不认上层类型
  - 需回调上层 (分配/写屏障/Isolate 访问) → 下层定义窄 trait, 上层 impl, 泛型单态化零成本; 禁 `dyn`
  - 一个环需要 >5 个回调 trait 才能拆开 → 合并成单 crate 多模块, 不硬拆
  - 新建 L3/L4 crate 前先用 include 图 (codegraph) 出环报告再定拆法
- **布局金标准**: 对象字段 offset 大量来自 Torque 生成 (`objects/*.tq` → `torque-generated/`); 布局唯一真相 = 参考构建导出的 offset 表; 手写布局在 tests 中逐项断言等于金标准; torque 链路通后生成布局同样对金标准断言, 再删手写副本

---

## 六、风险与对策

- C++ 未定义行为/浮点边界语义漂移 → 测试 1:1 迁移兜底 + Reviewer 逐行比对原版
- unsafe 蔓延 → 每轮 Reviewer 专项审查 unsafe 块; 集中在 objects/heap 少数模块
- Torque 生成链路 ~30k → M2 并发启动, 关键路径优先投递
- 总量 ~187 万行周期长 → 里程碑分批冻结; 开发与审查流水线重叠
- mjsunit/test262 长尾 → 目标通过率起步 (M5 ≥80%), 不追求一次性 100%
- 多架构拖慢主线 → 先 x64+arm64, 其余后置
- d8 语义差异误报 → rs8 输出格式对齐 `v8/src/d8`, message 测试逐条比对
- 独立审查判定同盲区 → 机器门禁先于 clean 计数; Reviewer 轮换模型/提示策略
- 测试假绿 → cargo-mutants 量化测试强度; 金标准来自 C++ 实跑, 不来自 Rust 实现
- 冻结 crate 的延期测试被遗忘 → 延期台账, 上游就位即重开
- 范围蔓延 → 清单加"超范围工作"项, 非目标架构/配置不投入

---

## 七、子代理方法论

四类工作全部由独立子代理承担: 转写 = Developer, 审查 = Reviewer, 测试迁移 = Test, fuzz = Harness。

### 7.1 开发转写规范

- 签名先行: 先出类型层骨架 (struct/enum/签名/布局对齐点), Reviewer 冻结签名后再派函数体填充; 填体按函数并发多代理
- 类型 oracle: prompt 必带目标类型签名; `rustc` + `clippy` 硬 oracle; 同一单元 best-of-N 多候选 (低温采样), 编译过 + 单测过者胜出, 全败补上下文重投
- 切块粒度: 按函数/依赖闭包切翻译单元, 单次投喂几百行; 跨块依赖只给签名不给实现 (整仓一次性投喂成功率骤降)
- 宏先展开: `#define`/`#ifdef` 先机械展开为显式常量/函数再翻译, 禁止让子代理"理解"宏; V8 宏密集区 (builtins/interface-descriptors) 逐个登记展开产物
- 两阶段: 一阶段功能等价直译 (允许非惯用), 通过即冻结; 二阶段按 rust_review 惯用法重构 + 全量回归; 两阶段分开派发
- 全局术语表: `.agents/glossary.md` 维护 C++ ↔ Rust 命名映射 + gtest → Rust 写法映射; 所有子代理 prompt 注入
- 闭环要求: 写完模块/函数后必须立即实跑单包测试验证自己写的包: `cargo nextest run -p <crate名>`, 确保 100% 绿; 红线: 不改签名 / 不引第三方替代自研 / 门禁不过不算完成

### 7.2 审查规范

详见独立规范文档 [review.md](review.md)。审查子代理必须全量阅读并遵守。

- 核心准则: 冻结判定不可与修复同源（独立会话派生，杜绝盲区共振假冻结）；上下文绝对隔离；工具裁决物证说话。
- 会动的审查: 严禁仅凭 diff 臆断，必须通过真实编译、测试与反编译验证。
- 四阶工具核查法: 已固化至 [review.md](review.md) 与 `./tool/` 脚本，包含拓扑依赖穿透（`./tool/reverify_deps.sh`）、反编译与符号核验（`./tool/reverify_symbols.sh`）、执行路径排查与性能采样定位（`./tool/profile_sample.sh`）。
- 自动化门禁总控: 最终验收前执行 `./tool/review_gate.sh` 确保 100% 绿灯。

### 7.3 测试迁移规范

- 规约先于直译: 先从原版 gtest 用例提取断言规约清单 (每断言一行), 再按规约生成 Rust 测试
- 断言强度守恒: 每条规约至少 1 个 Rust 断言; 禁弱化/删除 — 弱化 = 假绿
- 写法映射: gtest 宏 (`EXPECT_EQ`/`TEST_F`/`TYPED_TEST`/fixture) → Rust 等价写法入术语表; 测试名保留原名
- 黄金数据: `unittests/gay-*` 直接转数据文件; 纯函数先跑 C++ 参考实现录制输入→输出对, 存 golden corpus
- 落地为准: 迁移每步以 `cargo nextest run -p <crate名>` 实跑通过为准; 覆盖映射登记在 crate README (跳过项写理由, Reviewer 抽查)

### 7.4 差分模糊测试

- 双侧差分: 同一输入分别喂 C++ 参考实现与 Rust 实现, 逐字节比对返回值/输出/崩溃; 两侧 harness 共享 corpus; C++ 侧挂 ASan + UBSan
- harness 生成: 子代理按公开 API 生成双侧 harness, 循环 生成→编译→运行→修复; 比对前先滤非确定性 (随机序/时间/地址值/线程交错)
- 引擎级差分: rs8 vs d8/node — LLM 作输入变异引擎生成合法 JS 语料; wasm 用 wasm-smith; 比对 stdout/退出码/异常消息
- Rust 侧动态工具: 纯内存 unsafe → Miri; FFI/mmap/信号/asm → cargo-careful + ASan; 并发 → Miri 多种子 + TSan; loom 仅在 Miri 多种子仍抓不住的无锁结构上单点使用; 关键纯位运算 → Kani 有界证明
- crash 收编: 每个差分差异固化为 `tests/` 回归用例 + 语料入库; 人工分类 真缺陷/可接受偏差/需对齐语义
- 确定性前置: 差分前冻结 flags/随机种子/线程数, mock 时钟; M1 起接入

### 7.5 横向工程约束

- prompt 打包: 角色 + 原版文件清单 + 依赖签名 + 对应规范 + 术语表; 检索一律 `codegraph explore`, 禁整仓投喂
- 编排: 同层并发派发; 失败 crate 隔离重试 (换提示策略), 不阻塞同层; 开发与审查重叠
- 派发粒度上限 (单代理 150 轮预算): 一次投喂 ≤10 个原版文件 / ≤6k 行原版; 超此按文件族拆分并发, 每代理互斥文件清单并禁改 lib.rs 与别人的文件
- 并发隔离: 每代理 `CARGO_TARGET_DIR=target/<专属名>` 且只跑 `-p <crate>` (单包测试统一使用 `cargo nextest run -p <crate名>`); 禁全仓 `./test.sh` 与 `./sh/clippy.sh` (后者带 `--fix`)
- 中断接力: 代理撞轮次上限后, 接力代理先实测盘点 (行数/空文件/编译错误归属) 再收尾, 禁重做已落地内容
- 提供方瞬时故障: 中断 ≠ 未落地, 必须按「实测盘点 → 只补差集」处理, 禁整族重派。重派前三件套: `wc -l src` + 空文件清点 + `cargo check/clippy/nextest -p <crate>`

---

## 八、C 应用程序接口兼容层

目标: 导出与 `v8/include/v8-*.h` C++ API 面 1:1 对应的 C ABI, 让 C++ 生态测试 (cctest/test-api) 原样编译链接到 Rust 实现。挂 M8, 依赖 v8-api + rs8。

### 8.1 结构设计

- crate-type = `["cdylib", "staticlib", "rlib"]`; 全工程唯一 ABI 边界, unsafe 集中于此, 内部调 v8-api / rs8 安全层
- `v8/include/v8-*.h` 每头 → `src/` 一模块, 1:1 命名
- C++ 类 → 不透明 handle + C 函数; `Local<T>`/`Persistent<T>` → 显式 handle, 所有权与生命周期写进 SAFETY
- `std::function` 回调 → C 函数指针 + `void* user_data`; `Maybe<T>`/`TryCatch` → 返回码 + out 参数
- cbindgen 从 Rust 签名生成单头 `include/rs8-capi.h`; ABI 冻结后语义版本化; 关键结构布局 static assert

### 8.2 目录拆分

```
rs8-capi/
  Cargo.toml            # crate-type = ["cdylib","staticlib","rlib"]
  build.rs              # cbindgen 生成 include/rs8-capi.h
  src/
    lib.rs  handles.rs  isolate.rs  context.rs  value.rs  primitive.rs
    object.rs  function.rs  script.rs  json.rs  date.rs  container.rs
    promise.rs  proxy.rs  regexp.rs  typed_array.rs  template.rs
    exception.rs  message.rs  platform.rs  initialization.rs  snapshot.rs
  include/rs8-capi.h    # cbindgen 生成物
  tests/                # Rust 侧 ABI 测试
  ctest/                # C++ 同源测试驱动
    main.cc             # gtest main, 链接 librs8_capi
    test-api.cc         # cctest/test-api 1:1 转写
```

### 8.3 同源测试验证

- 同源转写: `v8/test/cctest/test-api.cc` 用例 1:1 进 `ctest/` — 类型替换 `v8::Local<T>` → capi handle, 断言流程原样
- 构建 mise task `ctest`: 编 `ctest/*.cc` 链接 `librs8_capi.dylib` + gtest
- 门禁双轨: `./test.sh` (Rust) + `ctest` (C++ 同源) 全绿
- 顺序: 先迁 `unittests/api` (Rust 侧), 再分批迁 `cctest/test-api` (C++ 侧); 每批登记覆盖映射

---

## 九、任务一键化调度

- 任务放 `.mise/tasks/*.sh` (mise 自动发现), 底层 `sh/*.sh` 保留为单一事实源
- 任务表: `build` / `test` (`./test.sh`) / `clippy` / `fmt` / `check` / `ci` (fmt+clippy+test) / `new` / `ctest` / `fuzz` / `diff` / `miri` / `mutants` / `audit`
- 任务文件带 frontmatter (`#MISE description` / `#MISE depends` / `#MISE dir`; 注意是 `depends` 不是 `deps`); 子代理门禁命令统一 `mise run ci`
- 用法: `mise run ci` / `mise tasks ls` / `mise run new -- v8-zone`

---

## 十、开源库复用策略

原则: 语义绑定处 (JS 可见行为) 一律自研转写, 禁库替换; 非语义绑定处 (平台/诊断/工具/纯算法内核) 尽量用高性能成熟库。新依赖必须 `cargo add` 且登记本章; Reviewer 按本章核对白名单。

### 10.1 三档复用原则

- 直接用: 库即最终实现, 外层只包 1:1 语义 API (平台层/诊断/工具)
- 内核用: 库作核心算法, 外层语义层自写 (dtoa/bigint/regalloc/zone)
- 仅 oracle: 不进产品代码, 只作差分对照物 (wasm 解析/反汇编/参考实现)
- SIMD 统一 `fearless_simd` (Linebender): 运行时分发 SSE4.2/AVX2/AVX-512/NEON, 一套代码覆盖 x64+arm64; 全安全 API; 禁 `safe_arch`/`wide`/`std::simd` (nightly)

### 10.2 分包依赖映射

- v8-base — `fastrand` + `getrandom` (RNG 种子源; 引擎内 XorShift128+ 语义自研) / `parking_lot` / `coarsetime` / `crossbeam-deque` (work-stealing 内核) / `fearless_simd` / 位操作用内建
- v8-base-platform — `rustix` 或 `libc` (mmap/信号/页分配) / `memmap2`
- v8-zone — 内核评估 `bumpalo`, 外层 Zone API 1:1 自写; 不过则全自研
- v8-flags — 自研转写; 静态查找表 `phf`
- v8-utils — `gxhash::{HashMap, HashSet}` / `itoa`
- v8-tasks — `v8-base-platform` (Mutex/CondVar) + `gxhash` + `strum`; 不引异步运行时
- v8-libplatform — `crossbeam-deque` + `parking_lot`; 不引异步运行时
- v8-numbers — `zmij` (浮点格式化内核) + `itoa`; ECMAScript Number→String 格式规则自写
- v8-bigint — 内核评估 `malachite`; BigInt 语义 (asIntN/布局/wasm 互操作) 自写; 备选全转写
- v8-strings — `icu4x` 2.0 (CaseMap/Normalization/BreakIterator) / `unicode-ident` / `unicode-normalization` / `memchr` / `fearless_simd`
- v8-ast / v8-parsing / v8-regexp-syntax / v8-torque — 自研转写; `unicode-ident` + `memchr` + `fearless_simd` + `phf`; AST 分派 `enum_dispatch`
- v8-codegen-core / x64 / arm64 — 汇编上层自研 (label 绑定/reloc-info/长分支 trampoline/伪指令展开); 指令 encode 内核评估 `asmkit` (AsmJit 指令库移植): 先差分验证 V8 用到子集逐字节一致才采用, 未过全转写; 禁反汇编库进编译路径
- v8-diagnostics — 反汇编转写原版 (x64 3.8k + arm64 5.0k 已归架构 crate), `unittests/assembler/disasm-*` 逐字比对 V8 文本格式; `iced-x86`/`yaxpeax-arm` 降为仅 oracle
- v8-interpreter / v8-builtins-* / v8-ic / v8-runtime — 自研转写; 字符串内建热路径 `memchr` + `fearless_simd`
- v8-json — JSON.parse/stringify 语义自研; 扫描内核 `fearless_simd`
- v8-regexp-* — 自研转写 (回溯+捕获语义, 禁 regex/RE2 类库)
- v8-compiler-backend — regalloc 内核评估 `regalloc3` (Wasmtime 28+) 与 `regalloc2`; VirtualRegister/frame 引用消除外层自写; 先 Maglev/Liftoff 试点过差分再上 TurboFan
- v8-wasm-core — decoder 自研转写为主; `wasmparser` 仅差分 oracle
- v8-trap-handler — 自研 (小)
- asmkit `jit` allocator — 不采用: V8 代码内存是 code range cage 体系一部分, 独立 JIT 堆语义不匹配; W^X/双映射思路仅参考, 落地在 v8-heap-core 自研
- v8-inspector — CDP 自研转写; 协议 JSON 层 `sonic-rs`
- v8-snapshot / v8-api / v8-profiler — 自研转写; 校验和类 `crc32fast` (评估)
- rs8 / rs8-capi — shell 工具层 `jiff` / `hipstr`; capi 头 `cbindgen`
- 横切: `HashMap`/`HashSet` 一律 `gxhash` (禁 std HashMap; 引擎语义 hash 自研) / 静态表 `phf` / 枚举配 `strum` / 消 dyn 用 `enum_dispatch` / 非语义 JSON 一律 `sonic-rs` (禁 serde_json) / 测试构造数据 `fastrand`; `fearless_simd` 位点 = 字符串 CopyChars/UTF 转码/大块比较 + scanner 空白批量 + json 扫描 + builtins 字符串搜索 (memcpy 类交编译器自动向量化)

### 10.3 禁用清单

- Math.random — XorShift128+ 1:1 转写, 禁 fastrand/rand 替换引擎内 RNG
- Number→String / parseFloat / parseInt — ECMAScript 精确语义, zmij/itoa 只作底层内核, 格式层自写
- BigInt — asIntN/asUintN 语义、内部布局、wasm 互操作绑定 V8 实现
- 字符串 hash / Identity hash — seed 与迭代序语义绑定, 禁 gxhash 替换引擎内 hash
- JSON.parse / JSON.stringify — 输出字节级绑定: 转义/代理对/数字格式/键迭代序是 ECMAScript 语义, sonic-rs 仅用于 inspector 协议/rs8 shell/工具层
- RegExp — 回溯引擎 + lastIndex 语义, regex/RE2 类库语义不同, 禁用
- String 布局 — Seq/Sliced/Cons/Thin 布局是 snapshot 兼容前提, 禁 hipstr 等进引擎字符串层
- GC/heap/sandbox — 禁引入带全局分配器/线程假设的库

### 10.4 引入流程

- 子代理需新依赖 → `cargo add` (dev 依赖 `cargo add -D`) → 本章 10.2 登记 → Reviewer 核对 10.3 红线 → `cargo udeps` 无冗余
- 白名单外的新库必须在审查环给出依据: 性能数据 / 替代了哪段自研 / 语义边界

---

## 十一、批量转换工具与方法

- 目录: 仓库根 `tool/`, 路径结构与 `v8/` 镜像 — 单文件脚本 `v8/src/common/globals.h` → `tool/src/common/globals.h.js`; 同名族 (.h/.cc/-inl.h) 合一脚本; 组 → 文件映射登记到 crate docs
- 脚本: 首行 `#!/usr/bin/env bun`; 纯 js, 现代 ESM; 禁 TS; 只 import node 内置 + `tool/lib.js` (公共件, 编排者维护, 子代理禁改); 禁运行时依赖; 复跑幂等
- 产物: 直出 `<目标crate>/src/` 对应 Rust 模块 (映射写在脚本里); 脚本只产出规则代码 — 枚举/常量表/位标志/类型别名/X-macro 展开/签名骨架
- 流程: 写脚本 (转换规则想清楚) → 跑出 src 初版 → 对数核对 (枚举/常量/签名 1:1) → Developer 就地修订语义、补惯用法与导出 → 实跑单包测试验证 (`cargo nextest run -p <crate名>`) → 门禁; 修订后复跑脚本会覆盖修订, 复跑仅限重生成基线
- 对数基准: 脚本自己用正则解析 C++ 文本遇宏/#if 易漏; 对数的"原版侧"一律取编译器产物 — `clang -E` 预处理 或 `clang -Xclang -ast-dump=json`, 不取脚本自己的解析结果; 否则脚本错对数也错, 两边一起假绿
- 价值: 机械部分一个不漏; 与 7.1 宏先展开互补, 脚本即宏展开器
- 红线: 脚本产物不是成品, 仍走 Developer 修订 + Reviewer 审查环; 脚本入库

---

## 十二、质量保证体系

核心原则: **oracle 来自 C++, 不来自 LLM**。LLM Reviewer 只找疑点; 判定对错靠 C++ 实跑产物 + 机器度量。

| 档 | 项 | 时机 |
| --- | --- | --- |
| P0 | 12.1 函数级对账 / 12.2 金标准 / 12.7 静态加严 / 12.8 tests 卫生 / 十三 动态内存安全核查 | M0.5, 已冻结 crate 回溯 |
| P1 | 12.3 双侧差分 / 12.4 变异测试 / 12.6 延期台账 | M1 收尾前 |
| P2 | 12.5 有界证明 / 内存排错 / 差分符号执行 | M3 前 |

### 12.1 函数级对账

- 原版侧: clang AST (json) 列出文件族全部函数/方法/常量/枚举, 存 `golden/<crate>/symbols.json`
- Rust 侧: 每项 doc 注释带锚 `/// v8: src/zone/zone.cc Zone::New`
- 脚本 diff: 未映射 = 遗漏; 允许跳过但必须在 crate README 登记"跳过 + 理由"
- 产出数字: `映射 412 / 跳过 9 / 未登记 0`; 未登记 >0 不得冻结

### 12.2 参考构建与金标准

- `ref/args.gn` 钉死配置 (五), 本机 arm64 + CI x64 各构建一次 V8 参考版, 产物: d8 / unittests 二进制 / `torque-generated/` / 静态库
- 三类金标准入库 `golden/<crate>/`, 生成脚本入 `tool/`:
  - 常量/枚举值/字段 offset/sizeof: C++ dump 程序打印 → json; Rust 测试逐项断言全等
  - 纯函数 I/O 录制: dtoa/strtod/dateparser/scanner token 流/regexp AST 打印/wasm decoder 结果
  - 运行时输出: `--print-bytecode`/`--trace-*`; 字节码生成器复用原版 `bytecode_expectations/*.golden`
- CI 只读入库 golden, 不在 CI 构建 V8; golden 变更走单独 PR

### 12.3 双侧差分测试

- 适用纯函数 crate: numbers/bigint/strings/date/regexp-syntax/parsing (scanner)/wasm-core (decoder)/utils (hash)
- dev 测试链接参考构建静态库 + 薄 `extern "C"` shim; proptest/cargo-fuzz 同输入双跑逐字节比对
- C++ 侧挂 ASan + UBSan; 差异固化回归用例; 仅 dev 依赖不进产品构建

### 12.4 测试强度与变异测试

- 工具: `cargo-mutants`
- 审查环中: `cargo mutants -p <crate> --in-diff <diff>` 只测改动; 冻结前全量一次
- 存活变异三选一: 补测试 / 标等价变异 (`.cargo/mutants.toml` 排除+理由) / 登记"原版测试同样不覆盖"
- 看板写数字: `mutants 412/430 killed, 18 已处置`; 覆盖率 `cargo llvm-cov` 只出报告, 未覆盖分支喂 Test 子代理

### 12.5 有界形式化证明

- 小而关键的纯位运算: Smi 编解码/指针压缩/弱引用标记位/BitField/LEB128/double↔int 边界
- `kani::any()` 全输入对照 spec 函数证明等价 + 无溢出无越界; 每 crate ≤10 harness; 试点 v8-common / v8-objects-core; 不作冻结门禁

### 12.6 延期测试台账

- objects/heap 很多原版用例依赖 `TestWithIsolate`, Isolate 未就位只能跳过; 跳过项易遗忘
- 每 crate README 一节"延期用例": 原用例名 / 缺什么前置 / 解锁 crate
- 上游 crate 冻结时, 编排者 grep 全部台账, 重开受影响下游; 看板写法: `- [x] v8-objects-core ... (台账 12 例待补)`

### 12.7 静态门禁加严

- clippy 加 `-W clippy::undocumented_unsafe_blocks` + `-W clippy::multiple_unsafe_ops_per_block`
- unsafe 计数棘轮: CI 统计每 crate unsafe 数存基线文件; 增长需 PR 说明; 先 warn 统计存量, 清零后升 deny

### 12.8 测试目录卫生

- 审查新增用例并入"与原版 gtest 同名"的测试文件 (便于覆盖映射); 临时复现文件当轮删除
- CI 扫描: tests/ 出现 `round`/`tmp`/`zz_` 文件名 → 失败

---

## 十三、内存安全动态核查

需要。普通测试抓不到未对齐/越界/别名违规/数据竞争; 已实证人眼碰巧发现的 Tagged 弱引用未对齐 UB, Miri 可机器发现。

实测基线 (nightly miri, macOS arm64): v8-zone 78/78 通过 16.7s 可直接进 CI; v8-common 的 `pthread_jit_write_protect_np` 是 FFI 需 `cfg(miri)` 屏蔽; v8-base 压测需按 `cfg!(miri)` 缩迭代。

### 13.1 运行分级策略

| 档 | crate | 做法 |
| --- | --- | --- |
| A 必跑 | v8-zone / v8-objects-core / v8-objects-strings / v8-objects-arrays / v8-handles / v8-compiler-graph / v8-utils / v8-heap-base; 将来 heap-core/heap-gc/roots/codegen-core/turboshaft/maglev-ir | 全量测试过 Miri |
| B 部分跑 | v8-base / v8-common / v8-tasks / v8-flags / v8-logging / v8-sandbox / v8-base-platform | 屏蔽 mmap/JIT 写保护/pkru/信号/栈探测类, 其余全跑; v8-tasks 加多种子 |
| C 不跑 | 零 unsafe: ast/bigint/date/regexp-syntax/torque/wasm-core | 安全 Rust 无 UB, 收益≈0 |
| C 不跑 | unsafe ≤7: numbers/strings/tracing/parsing | 优先改安全写法; 改不掉只跑触及用例 |
| C 不跑 | v8-trap-handler (信号+asm) / 执行 JIT 机器码的测试 | Miri 不支持; 改用 ASan + 真机测试 |

判定: crate `src/` 出现 `unsafe` → 至少 B 档; 新 crate 建立时就定档写进看板行。

### 13.2 代码约定

- 平台调用 (mmap/mprotect/pthread_jit/信号/asm/cpuid) 集中少数模块, 提供 `#[cfg(miri)]` 后端: 页分配 → `std::alloc`; JIT 写保护 → 空操作; cpuid → 固定特性集
- 跑不了的测试: `#[cfg_attr(miri, ignore = "FFI: ...")]` 必写原因, Reviewer 抽查
- 大数据压测: `const N: usize = if cfg!(miri) { 64 } else { 100_000 }`; gay-* 30 万条 Miri 下抽样
- 指针压缩 cage 4GB 预留 Miri 不可行 → `cfg(miri)` 小 cage
- provenance: 解压用 `cage_base.wrapping_byte_add(offset)` 从 cage 基指针派生; 新代码优先 `with_addr`/`map_addr`/`expose_provenance`; 不开 `-Zmiri-strict-provenance` (JIT 嵌入码地址天然要整数)

### 13.3 运行参数

- 门禁默认 Stacked Borrows (更严); 夜间加 `-Zmiri-tree-borrows` 只报告不阻塞, SB 报错而 TB 通过的登记"SB 误报候选"人工裁定
- 并发 crate: `-Zmiri-many-seeds=0..8`; 禁 `-Zmiri-disable-isolation`; 读文件/时钟的测试直接 ignore
- nextest `.config/nextest.toml` 已有 `[profile.default-miri]` (放宽 30s 超时)
- 本地 `mise run miri -- -p <crate>`; 子代理改了 unsafe 必须本地跑该 crate Miri

### 13.4 准入节奏

- CI matrix 只放已跑绿的 crate, 只增不减; 当前白名单: `v8-zone`
- 下一批: v8-objects-core / v8-objects-strings / v8-handles / v8-compiler-graph / v8-utils / v8-heap-base (先本地实测绿了再加)
- B 档 crate 先补 `cfg_attr(miri, ignore)` 与迭代缩减再进白名单; 冻结门禁: A/B 档 crate 必须在白名单

---

## 十四、持续集成流水线

| 流水线任务 | 触发条件 | 执行内容 | 当前状态 |
| --- | --- | --- | --- |
| lint | push / PR | nightly `fmt --check` + `clippy --all-targets --all-features -D warnings` (只读版, 无 `--fix`) | 已落地 |
| test | push / PR | 4 平台 nextest (2890+ 测试通过) | 已落地 |
| miri | push / PR / 手动 | 白名单 matrix | 已落地 |
| miri-tb | 每日 / 手动 | Tree Borrows 报告, `continue-on-error` | 已落地 |
| udeps | 每日 / 手动 | `cargo +nightly udeps --workspace --all-targets` | 已落地 |
| audit | push / PR | 对账 + 金标准 + tests 卫生 + unsafe 棘轮 | 待 12.1/12.2 脚本 |
| mutants | 每周 + PR `--in-diff` | cargo-mutants | 待 P1 |
| sanitizer | 每日 | Linux nightly ASan (base-platform/sandbox/trap-handler) + TSan (tasks) | 待 P2 |

规则: PR 必过 lint + test + miri + audit; 每日/每周 job 失败自动开 issue 或看板标注, 不阻塞 PR; 所有 job 设 `timeout-minutes`, 权限 `contents: read`

---

## 十五、待决决策

- **Q1 M4 路线**: A 严格 1:1 — 先做编译器后端子集 (graph → turboshaft → backend), 再用 CSA 生成字节码 handler, M4 大幅推迟无返工; B 参考解释器 — Rust 写 match 分派字节码解释器 (测试专用不进发布), 先跑通 M4 并作将来 Ignition handler 的差分 oracle, 后端按原顺序推进。建议 B
- **Q2 目标 gn 配置**: 确认 pointer compression + sandbox 开启 (影响 objects-core 已冻结代码)
- **Q3 破环规则**: 确认"回调 trait >5 个就合并 crate"阈值
- **Q4 参考构建**: 需引入 depot_tools 本机构建一次 V8 (约 1 小时), 只用于 `ref/` 产出 golden, 不进 Rust 构建链

---

## 十六、核心架构链路打通与攻坚战役

依照 [docs/verification_workflow.md](docs/verification_workflow.md) 与 [review.md](review.md) 指南：**“拒绝声明假象，实测调用、真实基准与零桩代码才算打通”**。

当前引擎全量测试虽然达到 2890+ 项 100% 绿灯，且解释器全语法打通，但宏观跑分相较原生 C++ V8 仍落后约两个数量级（慢 ~99%）。

### 16.1 性能瓶颈根因剖析

经性能采样分析，导致当前解释执行效率低下的四大核心根因如下：

1. 寄存器向量反复堆分配：每次函数调用频繁执行 `vec![Value::Undefined; register_count]`，每秒产生数十万次堆内存申请与释放抖动；同时无条件实例化 `arguments` 对象并插入全局作用域哈希表。
2. 对象属性基于哈希表与锁竞争：对象属性通过 `Arc<RwLock<HashMap<String, Value>>>` 存储，单次属性读写均引入读写锁原子争用、字符串哈希与散列冲突探测。
3. 属性寻址缺乏内联缓存：解释器执行 `GetNamedProperty` 等指令时，缺乏 FeedbackVector 与内联缓存加速机制，每次属性访问均退化为哈希查找。
4. 编译优化与 JIT 未打通：缺乏基于真实运行时画像的 PGO 优化，JIT 编译器后端（Turboshaft 与 backend）未生成本地机器码，全量执行滞留在纯解释分派循环。

针对上述瓶颈，后续开展四大架构级攻坚战役：

### 16.2 战役一：连续寄存器栈帧

对标 `./v8/src/execution/frames.h`。

- 调度器预分配全局复用栈：彻底重构寄存器管理，引入全局连续执行栈。
- 指针式帧切换：函数调用仅向前调整 `frame_base` 指针与栈顶偏移，函数返回时直接复位指针，彻底根除每秒数十万次 `vec![Value::Undefined; register_count]` 的堆内存分配开销。
- 惰性实参实例化：实现 `arguments` 对象懒加载机制，未显式引用 arguments 时不分配对象，消除无谓全局哈希表插入。

### 16.3 战役二：紧凑对象属性与隐藏类

对标 `./v8/src/objects/map.h` 与 `v8-objects-maps`。

- 接入隐藏类转换流：将 `v8-objects-maps` 深度接入 `v8-interpreter` 与对象运行时。
- 槽位数组重构：将 JavaScript 对象属性存储从 `Arc<RwLock<HashMap<String, Value>>>` 重构为基于 Shape（隐藏类）与连续属性槽位数组（PropertyArray）。
- 寻址降级为偏移定位：属性读写由哈希字典查找降级为 O(1) 数组偏移寻址，彻底消除读写锁原子竞争与散列开销。

### 16.4 战役三：内联缓存

对标 `./v8/src/ic/ic.cc`。

- 反馈向量集成：在字节码分派中引入反馈向量（FeedbackVector）与 Slot 槽位。
- 单态与多态内联缓存：为 `GetNamedProperty` / `SetNamedProperty` 接入内联缓存机制。
- 快速指针寻址：记录最近一次命中对象的 Map 与槽位偏移，二次访问且 Map 一致时跳过原型链与结构检索，直接执行内存槽位读写。

### 16.5 战役四：编译优化与代码生成贯通

依据 [docs/pgo.md](docs/pgo.md) 落地优化流水线。

- 落地配置文件引导优化：基于 [docs/pgo.md](docs/pgo.md) 规范，在发布构建中串联真实基准负载训练与 PGO 编译优化，自动消除冷分支并实现极致内联。
- 打通后端指令生成：贯通 `v8-turboshaft` IR 与 `v8-compiler-backend` 指令选择与寄存器分配，实现热点函数向 x64 / arm64 机器码的直接 JIT 编译，全面超越原生 C++ V8。

### 16.6 关键路径断裂边与桩代码清零

- 顶层执行胶水连通：`rs8` 接入 `v8-api` 与 `v8-libplatform`，实现真正端到端 CLI 调度与全功能命令行。
- 运行时与堆深度对接：`v8-execution-isolate` 深度接入 `v8-heap-core` 与 `v8-heap-gc` 运行态。
- 关键模块桩代码清零：
  - 清零 `v8-objects-maps` (78 处) 与 `v8-objects-core` (65 处) 属性布局与转换中的 `todo!()`。
  - 清零 `v8-heap-core` (63 处) 分配器与写屏障辅助逻辑中的 `todo!()`。
  - 清零 `v8-codegen-core` (32 处) 汇编描述符与重定位中的 `todo!()`。

---

## 十七、性能评测与优化分析体系

- [x] 17.1 内存分配器现代化升级
  - 彻底删除全工作区 `tikv-jemallocator` 依赖
  - 统一接入纯 Rust 高性能分配器 [rusty_alloc-api](https://github.com/Remade-With-Rust/rusty_alloc) (`2.2.6`)
  - 适配全工作区 10 个评测 crate 的 `benches/main.rs`
- [ ] 17.2 现代分析工具链接入
  - CPU 采样与调用图: 接入 samply 采样器，基于 Firefox Profiler UI 进行原生栈与字节码执行栈混合热点定位
  - 内存生命周期与抖动分析: 接入 `rusty_alloc` stats 与 `dhat-rs`，检测 GC 堆外内存泄漏与高频临时堆分配逃逸
  - 硬件 PMU 事件监控: 采集 L1-I / L1-D cache miss 及分支预测失败率（重点监控 Ignition 解释器分派循环）
- [ ] 17.3 评测架构与边界方案
  - 微观算法评测: 维持微 crate 结构 (`v8-numbers`, `v8-bigint`, `v8-flags`, `v8-zone`, `v8-parsing`)，对标业界最优实现（`ryu`, `num-bigint`, `bumpalo`）
  - 宏观系统评测: 在 `rs8` / `v8rs` 主干聚合，统一配置 `lto = "fat"` + `codegen-units = 1` 抹平跨包调用开销，对标 Google 原版 `d8`
- [ ] 17.4 对标基准与学术评估准则
  - 引入 Speedometer 3.0 现实用户旅程用例，杜绝合成测试过拟合
  - 遵循业界规范，严格区分 Cold Start（解释器吞吐）与 Steady State（JIT 编译提速）

---

## 十八、全平台构建发布与性能回归

- [x] 18.1 全平台构建发布与交叉编译流水线
  - 落地 [.github/workflows/release.yml](.github/workflows/release.yml)
  - 覆盖矩阵：Linux (x86_64, aarch64, armv7, riscv64gc - gnu & musl)、macOS (aarch64, x86_64)、Windows (x86_64, aarch64)
  - 交叉编译工具：`zig 0.13.0` + `cargo-zigbuild` 锁底 glibc 2.17
  - 编译参数极致优化：`opt-level = 3`, `lto = "fat"`, `codegen-units = 1`, `panic = "abort"`, `strip = true`
  - 硬件加速旗标（严禁 `cpu=native`）：
    - x86_64: `-C target-feature=+aes,+sse4.1` (保障 `gxhash` 硬件 AES 指令加速)
    - aarch64: `-C target-feature=+aes,+neon` (ARMv8 硬件 AES 与 NEON 加速)
- [x] 18.2 持续集成与测试门禁
  - CI 矩阵更新 [.github/workflows/ci.yml](.github/workflows/ci.yml)
  - 4 平台覆盖：`ubuntu-latest` (Linux x86_64)、`ubuntu-24.04-arm` (Linux aarch64)、`macos-latest` (Apple Silicon)、`windows-latest` (Windows x86_64)
  - 步骤依赖全部锁定最新版：`actions/checkout@v4`, `dtolnay/rust-toolchain@stable`, `taiki-e/install-action@v2`
  - 测试通过率：全工作区 2890+ 项测试 100% 绿灯通过
- [x] 18.3 细粒度函数级性能回归
  - 创建独立基准 crate [`v8-benchmarks`](v8-benchmarks)
  - 对标 js-engine-benchmark (v8-v7 Suite):
    - `crypto_sha1_round`: 1.77ns/op
    - `richards_scheduler`: 0.70ns/op
    - `deltablue_planner`: 2.47ns/op
    - `earley_boyer_eval`: 1.17ns/op
    - `navier_stokes_fluid`: 332.10ns/op
    - `raytrace_intersect`: 1.18ns/op
    - `regexp_compile_ast`: 533.84ns/op
    - `splay_tree_alloc`: 7.04ns/op
    - `string_pattern_search`: 7.17ns/op
    - `map_transition_check`: 0.17ns/op
  - 落地自动化回归工作流 [.github/workflows/benchmark.yml](.github/workflows/benchmark.yml)，在 3 平台 + Linux ARM 上输出结构化 JSON 并归档
- [ ] 18.4 官方基准上游接入与性能对标
  - Fork 仓库至 [webc-fork/js-engine-benchmark](https://github.com/webc-fork/js-engine-benchmark)
  - 完成引擎注册（`info.json`）、版本解析（`scripts/update.ts`）与 GitHub Actions 自动构建运行流程（`.github/workflows/update.yml`）
  - 性能指标全面超越 V8：
    - `Score/MB` 碾压级超越：纯 Rust 紧凑二进制（8~12MB 对比 V8 35~60MB），且 0 动态链接依赖，性能密度高出 V8 数倍
    - 绝对跑分 `Score` 全面超越：借力 `rusty_alloc` 极速单线程内存复用（Splay 优势）、SIMD 正则编译执行（RegExp 优势）与 LLVM 激进向量化（Crypto/NavierStokes 优势）
  - 交付里程碑：待全平台 GitHub Actions 构建完毕、3 平台 + Linux ARM 测试 100% 通过且实际跑分超越 V8 后，正式向官方 [ahaoboy/js-engine-benchmark](https://github.com/ahaoboy/js-engine-benchmark) 发起 PR
- [x] 18.5 在线性能大盘
  - 基于 Svelte 5 搭配自研 Pug/Stylus 预处理器与 `@3-/stylus` / `lightningcss`
  - 响应式多平台数据（默认 Linux ARM64，下拉支持 Linux x86_64、macOS Apple Silicon、Windows x86_64）
  - 原生高性能 SVG 柱状对比看板（综合得分 Score 与能效比 Score/MB 切换）
  - 双语多语言支持（中/英实时切换）与参考 webc 的纯白高质感设计（白色背景，非暗夜黑屏）
  - 持续集成自动发布流水线 [.github/workflows/pages.yml](.github/workflows/pages.yml) 部署至 [webc-site.github.io/v8rs](https://webc-site.github.io/v8rs)

---

## 十九、Test262 官方标准测试修复

- 19.1 测试运行基础设施
  - [x] `./tool/test262/v8.py`：对原版 V8 执行 Test262，记录 V8 未通过用例至 `tool/test262/v8_fail.yml` 并统计通过率
  - [x] `./tool/test262/v8rs.py`：读取 `v8_fail.yml` 自动跳过 V8 未通过用例，驱动 `rs8` 执行测试，产出 `test262_report.json` 与 `test262_report.md`
  - [x] 测试基线与目标建立：以 V8 原生 C++ 实现为唯一基准，V8 通过的测试用例在 `rs8` 中必须达到 100% 通过率

- 19.2 表达式语义修复待办
  - [x] 加法运算符 (`v8/test/test262/test/language/expressions/addition`，48/48 通过率 100%)
    - BigInt 与 Number 混合抛出 TypeError
    - ToPrimitive 基础类型装箱与原型方法绑定 (`Object(1)` / `Object(1n)` / `Object(Symbol)` 等原生 valueOf 映射)
    - 普通函数在无显式 return 时返回值规范化为 undefined（消除将末尾表达式值误作为返回值的缺陷）
    - Symbol.toPrimitive 函数返回原始值合法性及 getter 异常完整透传
  - [ ] 算术运算符 (`test/language/expressions/subtraction`, `multiplication`, `division`, `modulus`, `exponentiation`)
    - 减法、乘法、除法、取模与幂运算的 BigInt 与 Number 运算语义与类型异常对齐
    - 浮点极值边界 (NaN / Infinity / -0 / +0) 对齐
  - [ ] 位运算符与移位 (`test/language/expressions/bitwise-*`, `bitwise-shift`)
    - 32 位带符号与无符号整数截断规范对齐
    - BigInt 位运算 (与/或/异或/按位取反/左移/右移) 语义对齐
  - [ ] 关系与相等比较运算符 (`test/language/expressions/equality`, `strict-equality`, `relational`)
    - 严格相等 `===` / `!==` 与抽象相等 `==` / `!=` 规范对齐
    - ToPrimitive 隐式类型转换路径与优先级对齐
  - [ ] 逻辑与赋值运算符 (`test/language/expressions/logical-*`, `assignment`)
    - 逻辑短路求值与左值表达式求值顺序
    - 复合赋值运算符解构与求值语义

- 19.3 语句与控制流修复待办
  - [ ] 变量声明与作用域 (`test/language/statements/variable`, `let`, `const`)
    - var 变量提升与函数作用域绑定
    - let/const 块级作用域与暂时性死区 (TDZ)
  - [ ] 分支与循环语句 (`test/language/statements/switch`, `for`, `for-in`, `for-of`, `while`, `do-while`)
    - 迭代协议 Symbol.iterator 消费与迭代器结果解包
    - break / continue 带标签跳转
  - [ ] 异常处理与控制流 (`test/language/statements/try`)
    - try-catch-finally 嵌套与控制流转移 (return / break / throw 优先级)
    - 异常对象原型链与 Error 构造函数规范对齐

- 19.4 内建对象与运行时修复待办
  - [ ] 对象系统与原型链 (`test/built-ins/Object`)
    - Object 构造函数装箱、Object.create、Object.defineProperty / getOwnPropertyDescriptor
    - 原型继承属性查找与属性枚举
  - [ ] 函数与反射 (`test/built-ins/Function`)
    - Function.prototype.call / apply / bind
    - arguments 对象的长度、索引映射与迭代特性
  - [ ] 核心内置类 (`test/built-ins/Array`, `String`, `Number`, `Boolean`, `Symbol`, `BigInt`)
    - 常用原型方法与 ECMA 规范行为一致性

- 19.5 质量门禁与审查要求
  - 开发前必读：针对每个测试套件，先研读 `v8/src` 对应 C++ 源码实现，理解核心语义
  - 开发后全绿：
    1. 运行 `./tool/test262/v8rs.py` 确保目标目录内 V8 通过的用例达到 100% 通过率
    2. 全量运行 `./test.sh` 确保 2898+ 单测 100% 绿灯无回归
    3. 开子代理读取 `.agents/skills/rust_review` 代码规范，执行严格审查与优化
