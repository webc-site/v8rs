# V8 迁移看板

## 〇、口径

- 原版: `v8/src` 约 187 万行 C++/Torque, 48 个一级代码目录
- Rust 侧现状: 93 个 crate, `src/` 实测 410,787 行 (原版的 22%)；其中 rs8 依赖闭包内 20 个 crate / 16.0 万行, 闭包外 72 个 crate / 25.08 万行 (61%)
- 方法: `.agents/skills/transpile` 五步流; 规范: `.agents/skills/rust_review`; 审查: [review.md](review.md); 验收: [docs/verification_workflow.md](docs/verification_workflow.md)
- 符号理解: Rust 侧 `codegraph explore "<符号>"` (v8/ 已 includeIgnored); 原版直接读源码 + `BUILD.gn` deps
- 状态: `- [x]` 已冻结 / `- [ ]` 未冻结; 进行中在行尾标 `(开发中)` / `(审查环中)`
- 开发日志: 每 crate 一篇 `log/<crate名>.md` (约定, 表内不再逐个列链接); 编排全局 `log/orchestrator.md`; Wave 1-12 派发台账 `log/waves.md`; 转换脚本覆盖基线 `golden/tool-coverage.json`
- 测试基线: `./test.sh` 2909 项 100% 绿 (实测 2026-10-15, 1 skipped), **默认执行态已是 `ExecMode::BytecodeFirst`** (`rs8/src/runtime/interpreter.rs` `#[default]`); `--ast-only` 只作为语义对照基准保留
- 本文档只留当前有效结论。历史基线数字、每轮收敛计数一律沉到 `log/`, 看板不重复记载

> 绿灯只证明各 crate 自洽, 不证明引擎能跑。主链路已按 三·战役 A 打通 (默认 `BytecodeFirst`), 但 `rs8/tests` 与 test262 的覆盖仍不足, 详见 一 / 十一。

---

## 一、主链路诊断 (实测, 非推断)

### 1.1 断点位置

CLI 执行的是一台内置的简易 AST 树遍历解释器, 不是已转写完成的 Ignition 字节码引擎。

| 环节 | 证据 | 事实 |
| --- | --- | --- |
| CLI 入口 | `rs8/src/main.rs` `interp.run()`、`rs8/src/d8.rs` `run_js()` | 单一 Interpreter 实例贯穿 `-e` 与全部脚本 |
| 实际执行 | `interpreter.rs` `run()` → `dispatch()` → `execute()` → `eval_stmt()` / `eval_expr()` | 仅 `--ast-only` 与回落路径走这条; 默认 `dispatch()` 先交字节码引擎 |
| 真引擎 | `v8-interpreter` 14,234 行、0 桩, 入口 `BytecodeGenerator::generate` + `BytecodeDispatcher::execute` | 已被 `dispatch()` 驱动且为默认执行态 (`ExecMode::BytecodeFirst`) |
| 字节码通路 | `interpreter.rs` `dispatch()` → `bytecode_bridge.rs` `try_execute_with_bytecode_and_env()` | CLI `-e` / 多脚本 / 库入口共用一条通路, `--bytecode-first` `--bytecode-only` 可切换 |
| 降级可观测性 | `ExecStats{total, bytecode, fallback}` + `--print-bytecode-coverage` | 降级只剩执行前的 `None` (转换不支持) 一种, 计入 `fallback`; 执行中的错误一律传播, 不再重放 |

结论 (当时): **链路断在 `main.rs` → `run()` 这一层**, `v8-interpreter` / `v8-objects-maps` / `v8-ic` / `v8-runtime` / `v8-heap-*` 五段 V8 运行时段落在链路外空转。

> 本节保留原始断点定位证据。战役 A 已把 CLI 默认执行态切到 `BytecodeFirst`, `v8-interpreter` 进入主链路; 其余四段 (`v8-objects-maps` / `v8-ic` / `v8-runtime` / `v8-heap-*`) 仍在链路外, 由 战役 C / E /  heap 线接管。1.2 / 1.3 里"副本仍在"的部分随战役 B/C 收口。

### 1.2 三套并行重复实现 (拆分复用清单)

| 类别 | 简易副本 | 正版实现 | 桥接税 | 判定 |
| --- | --- | --- | --- | --- |
| 前端 | `rs8/src/runtime/lexer.rs` 742 + `parser.rs` 961 + `ast.rs` 180 = 1,883 行 | `v8-parsing` 4,567 + `v8-ast` 6,291 = 10,858 行 (已冻结) | `bytecode_bridge.rs` 1,309 行做 rs8 AST → v8_ast 逐语句转换 | 删副本, 用正版 |
| 值模型 | `rs8/src/runtime/value.rs:1216` `JsValue` 2,038 行 | `v8-interpreter/src/bytecode_dispatcher.rs:407` `Value` | `js_value_to_interpreter_value` / `interpreter_value_to_js_value` 双向深拷贝 (bridge `:825`/`:892`) | 合成一个, 落 v8-objects-* |
| 对象属性 | 两份都是 `Arc<RwLock<HashMap<String, _>>>` | `v8-objects-maps` 5,483 行 Map/DescriptorArray/PropertyArray (0 桩) | 无接入点 | 接隐藏类 |

### 1.3 结构性性能根因 (慢 99% 不是"循环没优化")

1. ~~跑错引擎~~ — **已消除**: CLI 默认 `BytecodeFirst`, 递归树遍历只在显式 `--ast-only` 与执行前不支持构造时出现
2. 桥接固定税 (已减半): 全局表已常驻 (`BytecodeDispatcher::with_globals(Arc::clone(globals))`, `bytecode_bridge.rs:1354`), 回写只走 `dirty_globals` (`:1411`), 不再整表深转换; **仍是全量的**是注入侧 —— 每次执行按父作用域链逐层 `read()` 锁 + `js_value_to_interpreter_value` 深转换 (`:1357-1370`)
3. AST→v8_ast 转换每条语句现场做, 无缓存; 每次 `run` 重新生成字节码 (`:1397`) 并新建 `BytecodeDispatcher` (`:1354`)
4. 属性读写 = 读写锁原子争用 + 字符串哈希 + 冲突探测; 无 Map 快照、无 IC、无 FeedbackVector
5. 函数帧 `vec![Value::Undefined; n]` 反复堆分配; `arguments` 无条件实例化并插全局哈希表
6. 无堆 cage / 无写屏障 / 无 GC 参与: 对象靠 `Arc` + `rusty_alloc` 全局分配器; `v8-execution-isolate/src/isolate.rs:16-31` 的 `Isolate` 结构体根本没有 `Heap` 字段, 只有两处常量薄桥 (`lib.rs:25-42`)
7. 无 JIT: 全部留在解释分派循环
8. 跨界调用仍新建宿主执行态: 解释器回调宿主 JS 函数时 `bytecode_bridge.rs:1132` 每次 `Interpreter::for_cross_engine_call()` —— 该构造已跳过 `init_builtins` (重放近千行内建预置脚本曾是每次跨界的固定税), 但每次仍新建一套 `Environment`。归 战役 C 收口 (单一值模型后不需要跨界壳)

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
| 未用 pub 符号 | 139 处 / 23 crate (12.19 全量重跑口径; 旧记 87 处口径差未深究, 以本次为准) | 链内仅 6 处且已全部删除; 链外 133 处大头: `v8-turboshaft` 56 (52 个 `opmasks_named.rs` 的 `k_*` 掩码变体, 属同构表不可删), `v8-maglev-frontend` 32, `v8-compiler-frontend` 14, `v8-codegen-arm64` 8 —— 全部按 D2 结论挂战役号待接线 |
| 冗余/错放依赖 | 34 处 | 未用 `log` 17、错放 `log` 5、未用 `humansize` 6、未用 `libc` 2, 另有 `unicode-normalization` `unicode-ident` `criterion` `rusty_alloc-api` 各 1 |
| `todo!()` / `unimplemented!()` 残留 | 410 处 / 23 crate | 详见 四 各行; 大头 `v8-heap-core` 70、`v8-wasm-core` 52、`v8-objects-js-async` 43、`v8-objects-collections` 32 |
| rs8 依赖闭包 | 20 / 93 crate (含 rs8 自身) | 闭包外 72 个 crate, `src/` 250,796 行 (占 61%) |

待处置 (下一轮清账, 按 2.2 分类):

- [x] D1 依赖清账: `cargo shear --fix` 清 34 处命中 → **0 errors** (25 个 `Cargo.toml`); `rusty_alloc-api` 矛盾已消解 (归位到实际使用的 crate)。**未链接文件 47 条记豁免**: 均为 `tool/` 转录脚本再生成的 `*_generated.rs` / `*_tq.rs` (v8-zone 5、v8-builtins-* 41/90), 删之复生, 挂接归战役 F/G/H 准入 —— 详见 12.19
- [x] D2 未用 pub 分诊 (全量重跑 139 处口径): 链内 6 处**已删** (`v8-interpreter` `move_register` + 2 个桥接占位函数与 `v8_ast` 根再导出; `v8-execution-isolate` 3 个桥接占位, 连带 `cargo remove` 三个只为占位存在的依赖); 链外 133 处**全部挂战役号、不删** (它们是各战役开工时要真实接线的转录面 API, 删了战役还得重写): **H** 121 (`v8-turboshaft` 56 —— 其中 52 是 `opmasks_named.rs` 的 `k_*` 掩码变体属同构表 / `v8-maglev-frontend` 32 / `v8-compiler-frontend` 14 / `v8-codegen-arm64` 8 / `v8-codegen-core` 3 / `v8-baseline` 2 / `v8-compiler-passes` 2 / `v8-compiler-backend` 1 / `v8-builtins-core` 1 / `v8-objects-rest` `code_kind` 1 / `v8-numbers` `classify_pow` 1), **F** 5 (`v8-heap-core` 2 / `v8-heap-gc` 1 / `v8-objects-maps` 1 / `v8-objects-js-async` 1), **G** 5 (`v8-debug` 3 / `v8-snapshot` 1 / `v8-torque` 1), **B** 1 (`v8-parsing` `scan_from_stream`), **C** 1 (`v8-strings` `set_has_non_smi_elements`)
- [ ] D3 重复实现三份副本按 1.2 清单收敛, 计入 三 战役 A/B/C 的删除量
- [x] D4 战役 A 默认翻转: 欠账 1-4 已收, `ExecMode::default() == BytecodeFirst`, `./test.sh` 2905/2905 绿 (实测 2026-10-15)

---

## 三、打通链路战役排期 (A→H, 严格串行准入)

依据 [docs/verification_workflow.md](docs/verification_workflow.md): 拒绝声明假象, 实测调用、真实基准、零桩代码才算打通。

**当前排期 (战役 A 已于 2026-10-15 收官, 严格串行准入)**: B (前端合一, 决定 mjsunit 通量) → C (值模型 + 属性存储合一, 一次收掉数组具名键过桥 / holey delete / 属性枚举序三条遗留) → D (连续寄存器栈帧) → E (FeedbackVector 与 IC) → H (Sparkplug/Maglev JIT, 收 12.9 的 4.2x)。A 之前排在最后的性能项现在按 C→D→E→H 的顺序推进, 每步都以 12.9 的同机口径复测, 不以主观"应该更快"记账。

### 战役 A: CLI 改驱真引擎 (先换驱动, 不加功能) — **已收官**

**已落地 (实测, 非计划)**

- 入口改为模式化调度: `ExecMode{AstOnly | BytecodeFirst | BytecodeOnly}` + `Interpreter::dispatch()` + `ExecStats` 计数; CLI 旗标 `--bytecode-first` / `--bytecode-only` / `--print-bytecode-coverage`; 单个 Interpreter 贯穿 `-e` 与多脚本, 全局作用域与 d8 一致
- 根除"报错回退整段重放": 实测 `try_execute_with_bytecode_and_env` 的 `None` 全部产生于 `dispatcher.execute()` **之前** (转换期 `?`), `Some(Err)` 全在执行之后; 原 `BytecodeFirst` 分支在执行失败后重跑 AST, 把 print/赋值等副作用跑两遍。现规则: `None` → 回落并计数, `Some(Err)` → 直接传播
- 补齐 `v8-interpreter` 六处语义缺口 (均以 `--no-bytecode` 为对照实测出来, 不是推断):
  - `LdaGlobal` 读未声明名改抛 `ReferenceError` (原静默 `Undefined` → 拼写错误被吞成 NaN)
  - 补 `LdaGlobalInsideTypeof` 全管道 (原版 37 号 opcode 此前只在枚举里声明, 生成器不发、分派器不收): `typeof 未声明名` 必须得 `"undefined"`
  - `ArrayJoin` 对 `null/undefined/hole` 拼空串, 不再走 ToString (原得 `undefined-null-1`)
  - 数组属性表补 `toString` (=`join(",")`), 否则落到 `Object.prototype.toString`
  - `Value::to_number` 对对象走 ToPrimitive (时间戳槽 → `valueOf`), 原 `d2 - d1` 恒为 NaN
  - `get_property_resolved` 先回溯接收者**自身**原型 (`Array/String/Number/Boolean.prototype`), 原来只回溯 `Object.prototype`, 预置脚本注入的 `Array.prototype.*` 全部丢失
- 桥接层回写宿主不再丢函数自有属性: `interp_to_js` 原把带 props 的函数映射成无属性槽的 `JsValue::Native`, 现落成 `JsValue::Function(FunctionData)`, 测试框架 `assert.sameValue` / `assert.throws` 得以跨程序存活

**默认已翻转 (实测 2026-10-15)**: `ExecMode::default() == BytecodeFirst`, `./test.sh` 2906/2906 绿。旧记录 "默认 AstOnly 2898 绿 / 切 BytecodeFirst 13 红" 作废 —— 那 13 红已由下面 4 类欠账的修复清零。

**战役 A 欠账清单 (4 类根因, 全部收口)**

1. ~~跨引擎调用丢执行上下文~~ — 全局表常驻 (`bytecode_bridge.rs:1354` `with_globals(Arc::clone(globals))`), 跨界回调改用 `Interpreter::for_cross_engine_call()` (`interpreter.rs:140`, 跳过 `init_builtins`); 不再有点调用新建 `BytecodeDispatcher::new()`。残留: 每次跨界仍新建 `Environment` → 战役 C
2. ~~无词法 Context 槽位~~ — 实测 `function outer(){var y=5; function inner(){y=60} inner(); return y}` 在 `--bytecode-only` 下得 60, 两个独立 `counter()` 闭包互不串扰 (1/2/1)
3. ~~`try/finally` 尾块未执行~~ — 生成器按 V8 `JumpContinuation`/`JumpScope` 的语义, 为每条退出路径 (handler / 正常 / return / break-continue) 各发射一份 finally 副本, `exit_label` 防止正常副本串进后续副本; `rs8/tests/syntax_corner_cases.rs` 14/16 覆盖三层 return 链、catch 内 break、finally 内 return/break、跳出嵌在 try 里的循环, 输出与 node 逐项一致
4. ~~`Value::Array` 无属性槽~~ — `Value::Array` 改为 `Arc<RwLock<ArrayData>>`, `ArrayData{elements, props}` 对标原版 `JSArray = JSObject + FixedArray elements`, 具名键落 `props` 惰性分配 (`get_property`/`set_property`/`delete_property`/`has_property_resolved`/`ForInEnumerate` 五处一致); `rs8/tests/syntax_corner_cases.rs` 15 覆盖 `a.foo` / `b.length=0` 后具名键存活 / for-in 同时吐索引与具名键

**战役 A 收官后新发现的语义缺口 (本轮修复)**

- `TestEqual` 两侧同为对象时误走 ToPrimitive: 任意两个普通对象都被转成 `"[object Object]"` 判为相等, `c != determining` 恒假 → deltablue 建不出计划链。现先按 `is_object_like` 短路成身份比较 (对标规范 `IsLooselyEqual`, 在 ToPrimitive 之前)
- `Value::test_equal` 在两侧都已是字符串时仍落到数值分支: `[]` 经 ToPrimitive 得 `""`, 于是 `[] == "0"` 把 `""` 与 `"0"` 都 `to_number` 成 0 而误判为真。现补 `(String, String)` 直接串比 (rs8/tests 17)
- 清理: 删除与 `to_primitive` 早期返回重复的 `to_primitive_if_obj` 壳, 改为 `#[inline(always)]` 薄壳 `to_primitive` + worker `primitive_from_object`; `./sh/clippy.sh` 命中清零
- **具名属性 IC 是错的, 已整体删除** (实测 2026-10-15): 原实现按调用点缓存 *属性值*, 而 `SetKeyedProperty` / `delete` 都不失效读点的缓存, `SetNamedProperty` 只清自己那一槽 → `/tmp/stale.js` 在 `--bytecode-only` 得 `1|1|1|del:1|7|1,2`, node 得 `1|5|5|del:undefined|7|1,2` (第 2/3/4 项全是陈旧值)。同机 A/B: 旧版 9.49/9.53/9.53s, 删除后 8.19/8.20/8.21s (**-13.6%**), 即它既不正确也不划算 → 连带删掉 `FeedbackSlot` / `feedback_vector` / `call_stack` 三处死重。回归门禁 `rs8/tests/syntax_corner_cases.rs` 20 已用旧版编回验证会 FAIL (`left: "1|1|1|del:1|7|1,2"`), 不是空跑
- 操作数解码与寄存器写入强制内联: `advance`/`update_current_bytecode`/`operand_location`/`operand_word`/`get_*_operand`/`set_register` 改 `#[inline(always)]`, 1200 臂分发循环里这些 getter 原先是 out-of-line 调用 (`nm` 复核符号已消失), 表查询得以在循环内合并
- **负结果 (不要重试)**: 把 `Value` 的 `Arc<RwLock<..>>` 整体换成 `Rc<RefCell<..>>` (去原子引用计数 + 去锁) 实测只有 8.13s → 8.08s (**0.6%**), 且 `rs8/tests/bytecode_test.rs` 编译不过 — `aok::Error: From<rs8::Error>` 要求 `JsValue: Send + Sync`, 换 `Rc` 直接破坏宿主错误契约。结论: 31% 的 clone/drop 开销在 *标签分派与指针追逐 / cache miss*, 不在原子性, 收它得走战役 C 的裸值模型而不是换智能指针

**准入**: ~~收掉 1-4~~ 已达成。`None` 仍只允许产生于执行前; 后续战役以 B/C 为准入门禁, 战役 A 不再开放新功能。

**战役 A 遗留的已知语义差 (实测, 按归属转交, 不在 A 内修)**

**访问器属性 (get/set) 三条实测缺陷 (2026-10-15) — 本轮已收口, 残余归战役 C** — 复现 `/tmp/acc3.js`: `var o={}; Object.defineProperty(o,"p",{get:function(){return 42},enumerable:true});`

| 场景 | node | 修复前两链 | 现状 |
| --- | --- | --- | --- |
| `o.p` | 42 | 两链 `undefined` | **已修**: 对象字面量 get/set 语法 + 两链成员读/写点 (`eval_get_property`/`eval_set_property`、GetNamed/SetNamed/GetKeyed/SetKeyedProperty) 经 `lookup_accessor` 触发 own accessor, 哨兵键 `@@accessor` 一次哈希判定, 无 accessor 对象零字符串分配 |
| `Object.keys(o)` | `["p"]` | `["@@get:p"]` / `[]` | **已修**: 键约定唯一真相在 `v8-interpreter::bytecode_dispatcher` (`enumerable_name(s)`/`is_internal_key`), 伪键还原去重、内部键过滤, for-in/Object.keys/JSON 三处共用; `@@get:@@symbol:x` 剥前缀后仍过内部键过滤 (符号键不外泄) |
| `o.p = 10` | 调 setter | 静默写数据 | **已修**: 写点查 `@@set:` 命中即回调不落表; `delete o.p` 两链连带回收伪键 (修复前 AST 恒 true 且 getter 删不掉、字节码看数据键 —— 实测分裂, 门禁 tests 27) |

残余缺口 (实测 2026-10-15, 归属战役 C 描述符槽):

- **原型链上的 accessor 不触发** (设计决定): own-only, 链上判定需要 V8 Map 级 "可能有 accessor" 位, 值模型无 shape, 逐层探测把每次属性读放大成 O(链长); 两链一致
- `JSON.stringify` 不调 getter (node 按 enumerable Get 序列化, 会执行 getter 并输出其返回值); 现在只保证伪键/哨兵不泄漏, getter-only 对象输出 `{}` 而 node 输出 `{"p":2}`
- getter-only 对象上的赋值会落一条被 getter 遮蔽的数据影子 (`o.p = 5` 后 `Object.keys` 给出 `p`, node 亦给出 `p`, 但影子值在 descriptor 合一后必须消失)
- `{get p(){}, p:2}` 重复键不做 "后者覆盖前者" 的重定义语义, 伪键与数据键并存, 读走 getter
- `delete` 最后一个 accessor 后哨兵键仍滞留 (只多付一次哈希, 无正确性问题)
- `Object.getOwnPropertyDescriptor`/`getOwnPropertyNames` 未实现
- `NativeFn::ArraySort` 的 `call` 返回 Err 是防御线: 两链拦截点 (`invoke_function_named` / `call_function`) 覆盖源码可达的全部调用入口 (含 `.call`/`.apply` 递归进 `call_function`); 仅当 native 值跨界过桥后被裸调 (`NativeFn::InterpNative` → 另一侧 `NativeFn::call`) 才触及, 表现为 TypeError 而非错值, 战役 C 值模型合一后该形态整体消失

| 现象 | 复现 (实测 2026-10-15) | 根因 | 归属 |
| --- | --- | --- | --- |
| 数组具名键过不了桥 | `a=[1]; a.foo=1; Object.keys(a)` → rs8 `["0"]`, node `["0","foo"]`; `a.hasOwnProperty("foo")` → false | 宿主 `JsValue::Array(Arc<RwLock<Vec<JsValue>>>)` (`value.rs:1227`) 根本没有属性槽, 桥接深拷贝时引擎侧的 `ArrayData.props` 被丢弃。索引键本轮已补 (`interpreter.rs:548` `keys_fn` + tests 18) | 战役 C (值模型合一, 不给宿主再加一份 props 副本) |
| `delete a[0]` 不留洞 | `var b=[1,2]; delete b[0]; (0 in b)+"/"+b.length` → rs8 `true/2`, node `false/2` | 元素槽是 `Vec<Value>`, `delete_property` 的 Array 分支只回收具名属性槽; `Value::TheHole` 已存在但数组从不写入, 补齐要同时改 `in`/取属性/for-in/`join`/`map` 等全部按洞语义 | 战役 C |
| 属性枚举顺序随机 | 同一脚本 `Object.keys({b:1,a:2})` 多次运行给出 `a,b` 与 `b,a` | 两侧对象存储都是 `std HashMap` + `RandomState`, 而规范要插入序 (数字键升序在前) | 战役 C (Map + DescriptorArray 自带序) |
| `new String("ab") == new String("ab")` | `--ast-only` 得 true, node 得 false; 字节码侧已正确 | AST 求值器副本的装箱比较 | 随战役 B 删副本 |
| 带标签语句直接解析失败 | `outer: for(..)` | rs8 自带词法/语法不支持 labeled statement | 战役 B (换 `v8-parsing`) |

### 战役 B: 前端合一, 删 rs8 自带词法/语法 — **优先级提到 bridge 之前**

实测依据 (200 个 `v8/test/mjsunit/*.js`, 同一口径三模式对照):

| 模式 | 跑通 | 失败集 |
| --- | --- | --- |
| AST (默认) | 5/200 | 与下行仅差 1 个文件 |
| `--bytecode-only` | 5/200 | 同上 |

失败分类两模式一致: 语法解析 93 例 (`Unexpected prefix token Percent` 58 + 反引号 11 + `RParen` 9 + `Comma` 8 + `Colon` 7 等), 缺 mjsunit 测试前置库 56 例 (`assertEquals`/`assertTrue`/`assertFalse` not defined), 引擎相关仅 5 例。**通量的瓶颈是 rs8 自带词法/语法, 不是字节码桥** —— 原以为要先补 bridge, 数据推翻了这个排序。mjsunit 跑通率要动, 必须先换 `v8-parsing`+`v8-ast`。

- rs8 的 `lexer.rs`+`parser.rs`+`ast.rs` (1,883 行) 由 `v8-parsing`+`v8-ast` (10,858 行) 取代
- `bytecode_bridge.rs` 的 AST 转换层 (约 1,300 行) 随之整体删除
- 前置库缺口另立子项: mjsunit 需要 `test/mjsunit/mjsunit.js` 作脚本前置, 当前 CLI 无 `--file` 前置机制 (对应 `tool/test262/v8rs.py` 口径)
- 准入: `rs8/tests/*` + test262 全绿; `grep -rn 'eval_stmt\|eval_expr' rs8/src` 为空

### 战役 C: 值模型合一 + 属性接隐藏类

- `JsValue` 与 `bytecode_dispatcher::Value` 合成单一引擎值类型, 落到 `v8-objects-*`; 删双向深拷贝
- 属性存储 `Arc<RwLock<HashMap<String,_>>>` → `v8-objects-maps` 的 Map + DescriptorArray + PropertyArray, 读写降为 O(1) 槽位偏移 (对标 `v8/src/objects/map.h`)
- 全局环境单一化: 停止每次调用全量 clone 父作用域链注入 + 执行后全量写回 (`bytecode_bridge.rs:1243-1260,1294-1299`)
- 前置: `v8-objects-maps` 0 桩但需复审冻结
- 准入: 运行期 crate 内 `grep 'RwLock<HashMap<String'` 为空; benchRun 逐用例提速比记录在案
- **12.12 新增的同类欠债** (都是"值模型不带这个信息, 只好用伪键/哨兵/实例落表绕"的表现):
  - 数组实例没有 `__proto__` 接线 → 预置 JS 里的 `map`/`forEach`/`filter`/`reduce`/`every`/`some` 只在字节码链可用, AST 链一律 undefined (回退悬崖)
  - 描述符属性 (`enumerable`/`writable`/`configurable`) 无处存, `Object.defineProperty` 把它们丢了; 现方案是 `@@nonenum` 哨兵未做
  - accessor 以 `@@get:`/`@@set:` 伪键存表, 只做 own 层; 原型链 getter 需要 Map 级 "可能有 accessor" 位
  - 宿主 native 拿不到执行态, 所以 `sort` 要在调用点拦截、`JSON.stringify` 触发不了 getter
- **开题勘察定案 (2026-10-17, 只读盘点)**:
  - 变体差集: 引擎 `Value` 独有 `TheHole`/`Smi` (bytecode_dispatcher.rs:1455-1474), 宿主 `JsValue` 独有 `Symbol`/`RegExp` (value.rs:1816-1829); 同名异实 `String(String)` vs `String(Arc<str>)`、`BigInt(i128)` vs `BigInt(String)`、Array 引擎有 `props` 袋宿主无 (合一即修十五 B 数组具名键三条语义差); 两侧 `NativeFn` 是独立枚举 (~60 变体 vs 引擎臂) 且 `Ptr` 签名不同
  - 桥接收口窄: 全部深拷贝集中在 `bytecode_bridge.rs` (`interp_to_js_with_visited`:965-1085 / `js_value_to_interp_with_visited`:1238-1465 + 包装), 外部调用点仅 value.rs:940/945/954/961、interpreter.rs:1893、mod.rs:11 —— 收浅包装波及面可控
  - Send+Sync 断点 = `error.rs:20 Thrown(JsValue)` (aok 契约), **保留 Arc/RwLock 形态, 不复活 Rc 路线** (负结果已记)
  - `RwLock<HashMap<String` 准入清单 18 处: rs8 value.rs:69,70,109,110,1389,1406,1825 / environment.rs:17 / interpreter.rs:331,2039 / bytecode_bridge.rs:1087; 引擎 bytecode_dispatcher.rs:1471,1473,1582,2132,2974,3148,3228 —— 清零挂在 descriptor 落地那一步, 非首刀
  - `v8-objects-maps` 骨架已在 (descriptor_array/property_array/map_updater/transitions 1046 行) 但零接线; `v8-objects-rest` 的 `todo!()` 桩归 heap/snapshot 席
  - 拆分顺序 (首刀→删桥): ① 宿主字段对齐 (`JsValue::String`→`Arc<str>` 113 构造点, 就地变异点实测仅 value.rs:2241 一处局部 join 与两处 ref 只读臂; Array 存储对齐) + 桥两 `*_with_visited` 收浅包装 → ② 引擎补 `Symbol`/`RegExp` 变体 (删宿主枚举前置; prelude JS 注入 interpreter.rs:1618-1835 的 `NativeFn::Ptr`↔`InterpNative` 重解释是最大回归面, 单列一步) → ③ TheHole 数组洞语义 → ④ 删桥 + descriptor 接 Map (准入 grep 清零在这一步) → ⑤ 全局环境单一化
  - **进度 (12.46–12.51, 2026-10-17)**: ① 已收官 —— C-1a 落 String→`Arc<str>` 半刀 (记时零回退, 细账 12.46), C-1b 落桥 String 直通 + 22 处生成点双分配减半 (str2 靶点 −1.56%, add1 +1.2% 布局税归因分发环码扰, 细账 12.47); Array 存储评估结论 = `ArrayStore` 已在 #23 同形 `ArrayData`, 深拷贝待 ④ 删桥才消。**② 已收官** —— C-2a 落引擎 `Value::Symbol` + 跨桥直通 (细账 12.48); C-2b 第一刀拆家: `RegExpCore`+规范化编译+`regex_split_parts` 迁引擎 `regexp.rs` 一处定义, 宿主 `RegExpData` 缩为 core+props 袋 (细账 12.49); C-2b 第二刀落引擎 `Value::RegExp(Arc<RegExpCore>, 袋)` 变体 + 分发器全臂 + 跨桥浅拷贝共享内核身份 (lastIndex 单一真相), 物化/还原/@@split 哨兵/@@shadow 墓碑/名册机器 ~200 行退役, re1 −12.0%/split1 −33.5%、算术探针 ~+3% 布局税记合成债 (细账 12.50)。**③ TheHole 数组洞已收官 (12.51)** —— 洞哨兵只栖元素槽、读面 `materialized` 一处定义物化、`StaInArrayLiteral` 唯一洞入口、存在面全筛洞、map 保洞/reduce 种子扫描/includes 洞当 undefined; 顺带根因修复 `for_cross_engine_call` 空 global 原型面失明 (签名收 `root: Arc<Environment>`); 记时零回退 (hole1 新靶 BC −1.2%、A/A 底 1.8% 噪声带内)、门禁 79/80 双链钉、2977/1skip。余项 ④ 删桥+descriptor (expando 袋身份合一、字面量 per-eval×编译缓存在此) → ⑤ 全局环境单一化

### 战役 D: 连续寄存器栈帧

- 对标 `v8/src/execution/frames.h`。帧从 `vec![Value::Undefined; n]` 改为连续执行栈 + `frame_base` 指针推进, 返回复位
- `arguments` 惰性实例化, 未显式引用不分配
- 涉及 `v8-execution-frames` (现 723 行 / 原版 25k) 补齐

### 战役 E: FeedbackVector 与内联缓存

- **前置条件 (战役 A 已踩过的坑)**: 上一版 IC 缓的是 *值*, 写入路径不失效 → 语义错且零收益, 已删 (见一.战役 A 收官后新发现)。重做必须缓 **shape + 槽位/偏移** 而不是值, 且对象存储要有 mutation 版本; 在此之前 `GetNamedProperty`/`SetNamedProperty` 一律直读直写, 不许再加任何按调用点的值缓存
- **更硬的前置条件**: 生成侧现在把所有 feedback slot 都写成常量 `0` (`bytecode_generator.rs:884/959/983/1005/1062/1089/1246/1250/1299/1575/2077` 全是 `, 0)`), 也就是**没有 per-site 槽位分配器**。在这种编码下任何"按槽缓存"都退化成一整条脚本共享的单入口缓存, 天然互相冲掉、也天然读到别的站点的脏值 —— 旧 IC 的 `1|1|1|del:1` 正是这个结构问题的表现, 不是漏写一行失效。所以战役 E 的第一步是 slot 分配器 + `BytecodeArray` 携带 slot 数, 而不是接缓存
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

### 3.1 旧「四大瓶颈」条目消账

上一版看板 十六 的四条根因 (寄存器堆分配 / HashMap 属性 / 无 IC / 无 JIT) 已并入上面 A→H; "跑错引擎"这一条旧文档未列, 实为最重的一条, 见 1.1。

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
- `- [ ]` v8-objects-arrays | — | 6.3 | 13 | 外 | 13 桩; FixedArray 族 (战役 C 的 PropertyArray 底座)
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
- `- [x]` v8-interpreter | 28 | 14.2 | 0 | CLI | **真引擎, 0 桩, 已是 CLI 默认执行态 (战役 A 收官)** → 后续只按战役 C/E/H 补语义与提速
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
- `- [ ]` rs8-capi | 10 | 0.7 | 0 | 外 | 仅封装简易解释器, 无 1:1 C ABI 面
- `- [ ]` v8-benchmarks | — | 0.5 | 0 | — | 评测 harness, 非引擎; 走 `run_bytecode` (字节码优先), 与 CLI 默认模式不同 → 对比数字须同模式重测

---

## 五、里程碑

- `- [x]` M0 工作区基础设施 — workspace + 模板 + 门禁脚本 + 看板
- `- [ ]` M1 L0+L1 全冻结 — 实测残余桩: zone 18 / utils 18 / tasks 8 (闭包内), logging 20 / tracing 16 / bigint 16 (闭包外)
- `- [ ]` M2 L2 全冻结 — 实测残余桩: regexp-syntax 2; objects-core 桩已清待复审
- `- [ ]` M3 L3 对象与内存层消账 — 实测残余桩 133: heap-core 70 / objects-js-async 43 / objects-collections 32
- `- [x]` M4 最小引擎闭环与解释器打通 — 战役 A 已把 CLI 默认切到字节码引擎, "闭环"含义恢复
- `- [ ]` M5 L4 运行时与内建消账 — 残余桩 24; 真实缺口是 `v8-runtime` 不在闭包 + `v8-execution-isolate` 无 Heap
- `- [ ]` M6 L5 优化编译器端到端 — 见战役 H
- `- [ ]` M7 L6 WebAssembly 规范测试 — 残余桩 74, 大头在 wasm-core 52
- `- [ ]` M8 JS 标准测试 100% — 见 十一; 前提是战役 A-C
- `- [ ]` M9 性能基准对标 V8 — 见 十二.9: CLI 同机实测 rs8 94.4s vs node 22.5s (4.2x), 战役 A 已完成, 数字可用
- `- [ ]` M10 终验全量对抗审查与发布 — 全门禁绿灯 + 全平台 release

关键路径: v8-base → v8-zone → v8-ast → v8-parsing → v8-objects-core → **战役 A/B/C (链路合一)** → v8-heap-core → v8-execution-isolate → v8-codegen-{x64,arm64} → v8-compiler-graph → v8-turboshaft → v8-compiler-backend → v8-codegen-csa → v8-builtins → rs8 → Test262 → js-engine-benchmark

---

## 六、分层与依赖

依赖自下而上, 层内可并发。C++ 实测的三处依赖倒置:

- 内建与字节码分派器依赖编译器后端: `codegen/code-stub-assembler.h` include `src/compiler/code-assembler.h`; Ignition handler 由 `interpreter-generator*.cc` 经 CSA/TSA 生成 → CSA 内建与 handler 都要 L5 后端才能出机器码
- 快照在启动关键路径: V8 无"无快照"模式, 构建期 mksnapshot 生成 embedded blob → v8-snapshot + v8-init 必须在引擎引导前就位
- objects 与 heap 循环依赖: C++ 同属 `v8_base_without_compiler` 一个 target (objects 120 文件引 heap, heap 98 引 objects, 41 头引 `execution/isolate`); Rust 破环策略见 十三

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
- **每次语义改动先加 `tests/` 用例再改代码**: 同一脚本 AST 链 / 字节码链逐字符一致 (`both_chains`), 并与 node 实测对照。修 bug 同理 —— 先让用例复现缺陷, 再动实现; 无对应用例的改动视为未完成 (门禁只断言"结果是什么", 短路/持锁/取长度一次这类退化必须靠断言形状抓住)

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
| test | push/PR | 4 平台 nextest (2906 项) | 已落地 |
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

### 8.1 三档原则与 SIMD

原则: 语义绑定处 (JS 可见行为) 一律自研转写; 非语义绑定处 (平台/诊断/工具/纯算法内核) 用高性能成熟库。新依赖 `cargo add` 且必须登记本章, Reviewer 按 8.3 核对。

三档: 直接用 (库即最终实现) | 内核用 (库做核心算法, 外层语义自写) | 仅 oracle (不进产品代码, 只作差分对照)。

SIMD 统一 `fearless_simd` (Linebender): 运行时分发 SSE4.2/AVX2/AVX-512/NEON, 一套代码覆盖 x64+arm64; 禁 `safe_arch`/`wide`/`std::simd`。

### 8.2 分包映射

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

### 8.3 禁用清单

Math.random (XorShift128+ 1:1, 禁 rand 替换) | Number→String / parseFloat / parseInt (zmij/itoa 只作内核) | BigInt (asIntN/布局/wasm 绑定) | 字符串 hash 与 identity hash (seed 与迭代序语义绑定) | JSON.parse/stringify (转义/代理对/数字格式/键序是 ECMAScript 语义) | RegExp (回溯 + lastIndex) | String 布局 (Seq/Sliced/Cons/Thin 是 snapshot 兼容前提, 禁 hipstr 进引擎字符串层) | GC/heap/sandbox (禁带全局分配器或线程假设的库)

### 8.4 引入流程

`cargo add` (dev 用 `-D`) → 登记 8.2 → Reviewer 核 8.3 → `cargo udeps` + `cargo shear` 无冗余。白名单外的库须在审查环给出依据 (性能数据 / 替代了哪段自研 / 语义边界)。

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

门禁: 每个套件先研读 `v8/src` 对应 C++ 实现 → `./tool/test262/v8rs.py` 目标目录 100% → `./test.sh` 无回归 → 子代理读 `.agents/skills/rust_review` 审查。**注意**: 默认已是 `BytecodeFirst`, 但每个套件开工前仍要先跑 `--print-bytecode-coverage` 确认该目录没有整段落回 AST 路径 —— 落回的那部分修的是战役 B 将删除的副本。

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

**顺序约束**: 战役 A 已完成, 12.9 的 CLI 数字为有效基线; 12.3 的旧 ns/op 仍待按新口径重测。"超越 V8"的前提是先收 12.9 的 4.2x 差距 (战役 H)。

### 12.9–12.45 历轮细账: 已原样归档至 `todo-archive.md` (2026-10-17 压缩主线, 只搬不删不改)

轮次索引 (正文交叉引用 `见 12.xx` / `12.xx 记账` / `残档见 X 轮` 一律查归档):

- 12.9 CLI 主链路实测 — (战役 A 收官后, 2026-10-15)
- 12.10 本轮复测 — (2026-10-15, 删 IC + 强制内联之后)
- 12.11 寄存器借用改造 — (2026-10-15, 承接 12.10 的 clone/drop 33%)
- 12.12 ES5 accessor + 数组方法表 + Date 补漏 — (2026-10-15)
- 12.13 审查轮复核 + delete 伪键成对回收 — (2026-10-15)
- 12.14 数组迭代内建原生落地 + 内建接收者原型回落 — (2026-10-15)
- 12.15 计算键 accessor 的字节码 lowering — (消掉最后一处整脚本回退) (2026-10-15)
- 12.16 数组迭代取元素时机对齐规范 + 审查轮纪律 — (2026-10-15)
- 12.17 字面量走 CreateDataProperty + 表容量预留 + 累加器作废 — (#27 收口, #25 定位) (2026-10-15)
- 12.18 字面量收益归因复测 — (#29 收口, 低负载窗口 2026-10-15)
- 12.19 数组内建 #19 补齐 + 键常量一处定义 + D1 依赖卫生 — (2026-10-15)
- 12.20 常量池名字槽位的类型化错误 — (#26 收口, 2026-10-15)
- 12.21 `Array.prototype` 覆盖遮蔽内建替身 — (#24 收口, 2026-10-15)
- 12.22 `String.replace`/`match`/`exec` 回调与 `$` 序列 — (#23 主体收口, 2026-10-15)
- 12.23 正则对象方法面收敛: 只暴露 exec/test — (bc 链泄漏收口, 2026-10-15)
- 12.24 函数转字符串给源码文本 — (#23 残项⑤闭, parser span 落地, 2026-10-15)
- 12.25 内建面扫面第一轮: 取整/parseInt/切片三法收口 — (2026-10-15)
- 12.26 内建面扫面第二轮: 12.25 面缺失清单收口 — (2026-10-15)
- 12.27 内建面扫面第三轮: Math 单目族 + String/Array 三批 + URI 编解码 — (2026-10-16)
- 12.28 箭头函数 + 模板分组 + spread 口径 + toExponential + Object.prototype 枚举面 — (2026-10-16)
- 12.29 内建面扫面第五轮: delete 口径 / typeof 与 == 名册 / for-in 接收者面 — (2026-10-16)
- 12.30 内建面扫面第六轮: fromIndex 归一 / Number 进制前缀 / JSON.stringify 缩进 — (2026-10-16)
- 12.31 内建面扫面第七轮: reduce 族 / 位运算与 try-finally / 基元接收者 hasOwnProperty / split 缺省针 — (2026-10-16)
- 12.32 内建面扫面第八轮: String.lastIndexOf 全表 / 针参强制转换 / 全局环境名册缺口记账 — (2026-10-16)
- 12.33 内建面扫面第九轮: Object.hasOwn / String.raw 手写对象形 / 静态面清点记账 — (2026-10-16)
- 12.34 内建面扫面第十轮: Object.groupBy / 空原型对象继承面收敛 / 数组字面量空位解析 — (2026-10-16)
- 12.35 内建面扫面第十一轮: RegExp exec/test lastIndex 源真相 / sticky 锚定 / split 捕获组 — (2026-10-16)
- 12.36 内建面扫面第十二轮: 正则对象枚举/原型面收敛 — (2026-10-16)
- 12.37 内建面扫面第十三轮: 正则 expando/方法遮蔽面 — (2026-10-16)
- 12.38 内建面扫面第十四轮: 解构族 — (2026-10-16)
- 12.39 内建面扫面第十五轮: 形参默认值与箭头解构形参 — (2026-10-16)
- 12.40 内建面扫面第十六轮: rest 形参 — (2026-10-16)
- 12.41 内建面扫面第十七轮: for-of 迭代语句 — (2026-10-16)
- 12.42 BC 引擎缺陷修复: var 提升预绑槽 — (循环体变量被临时寄存器冲撞, 2026-10-16)
- 12.43 BC 引擎性能: 分发环热 helper 强内联 + Call 实参移动化 — (2026-10-17)
- 12.44 标记语句两链落地: `L: for` / 标记 break-continue / 标记块·if·switch — (2026-10-17)
- 12.45 寄存器搬移窥孔: Ldar+ShortStar 原地折 Mov — (第十九轮, 2026-10-17)

### 12.46 战役 C-1a: 宿主 JsValue::String 换 Arc<str> (第二十轮, 2026-10-17)

战役 C 开题勘察 (只读盘点, 结论已钉进 C 节拆分顺序) 后的首刀: 宿主 `JsValue::String(String)` → `String(Arc<str>)`, 与引擎 `Value::String(Arc<str>)` (dev 分支 10-14 已落) 对齐; 连带 25 个 `NativeFn::StringXxx(String)` 接收者变体全转 `Arc<str>`。动机: 桥两向深拷贝与 AST 链的每次字符串 clone 从 O(n) malloc 降为 refcount bump, 并把 `interp↔js` 字符串转换收缩为浅包装前置条件。**纯内部表示改动、零语义变化** —— 门禁形态不变即自证。

- 收口点: 变体定义 value.rs:1816 / 桥 `js_value_to_interp*` 与 `expr_to_v8_ast` 读点 `&**s` / `element_key` 与 `to_js_string` 的 String 出口保留 `to_string()` (契约返回 String, 成本与轮首相同) / regex `captures(&**s)` 显式解引用 (`Input` 不为 `Arc<str>` 实现, 泛型位不经 deref coercion)
- 机械改动 ~200 处 (lib 116 + tests 65 + capi 2), 编译器驱动收敛; `"...".to_string().into()` 双分配 31 处复扫降为 `"...".into()` 单分配
- **教训 (批量改型的工具链纪律)**: 手写括号计数器在字符串字面量/`<`/多参数行上产生 `String(x))` 垃圾且不报 ERROR (自动提交进程 13:48 把带伤中间态收成了单提交 `init`, HEAD 一度编译不过 —— 如实记录); 收敛路径 = 每遍改动必须立即 `cargo check` 驱动, 模式位还原用"删除报告 span 处 7 字符 `.into()`"而非 8 字符 `.into())` (后者对嵌套 `Some(...)` 模式必然多吞一票括号)。批量脚本先过"单行 + 字面量掩码 + 顶层逗号跳过"三重围栏, 多行参数交编译器逐个钉
- **实测**: 跨版本对拍 (改型前 `rs8_peepAB` vs 改型后 `rs8_arcstr`, 两链 × oct4/rich_long/add1/add5/peep1-4/g75b 18 组) 输出逐字节全等 (oct4 仅内嵌计时数, 归一后等); 记时 (同窗口三向交错 min-of-N + A/A, load 开场 5.76/收场 2.24, 每采样点 load<6 守门, D=12.45 窥孔末 `rs8_peepAB`, D2=C-1a 后 `rs8_arcstr`, C=12.43 `rs8_ownedcall`):

| 负载 | D (ms) | D2 (ms) | D2 vs D | C (ms) |
| --- | --- | --- | --- | --- |
| add1 | 160 | 161 | +0.6% (A/A 散布 162–163 内) | 169 |
| add5 | 384 | 381 | **−0.8%** (A/A 384–386) | 413 |
| oct4 | 25114 | 24944 | **−0.7%** (A/A 25209–25322) | 26315 |
| rich_long | 7615 | 7608 | −0.1% (A/A 7630–7681) | 8150 |

结论: C-1a 纯内部表示改型**零回退**、add5/oct4 微降 (add1/rich 在 A/A 噪声内); 本轮收益本就不在裸速, 而是把字符串 clone 从 O(n) malloc 降为 refcount bump, 为桥两向浅包装与战役 C 值模型合一铺平前置条件。

**门禁**: `./test.sh` **2965 passed / 1 skipped**、`v8-interpreter` **33/33**、`./sh/clippy.sh` exit 0 / 0 warning、`cargo check --workspace --all-targets` 0 error

### 12.47 战役 C-1b: 字符串跨桥/生成点收浅包装 (第二十一轮, 2026-10-17)

① 的 String 收尾半刀: 把 C-1a 后所有"两侧同为 `Arc<str>` 却仍走 `to_string()` 物化"的跨桥/生成点改成直通。零语义变化。

- **桥收口**: `interp_to_js_with_visited` 的 `InterpreterValue::String(s) => JsValue::String(s.clone())` (原 `s.to_string().into()` = 每跨桥一次字符串 2 次 malloc 1 次 memcpy, 现 refcount bump); 反方向 C-1a 已是 `s.clone()`。**Array 对齐评估结论**: 宿主 `ArrayStore{elems, Option<props>}` 在 #23 已同形对齐引擎 `ArrayData{elements, Option<props>}` (仅字段名与内层值类型不同), 桥的 Array/Object 逐元素深拷贝**必须**等 ④ 删桥才消 —— C-1b 无 Array 代码可动, 评估即收口
- **生成点直通**: 引擎 `bytecode_dispatcher.rs` 11 处 (`string_split_inlined`/`string_replace_inlined` 等 `&str` 参数 → `new_string(x)` 1 分配, 原 `x.to_string()` 进 `From<String>` 2 分配) + 桥 `regex_exec_core` 捕获/input 2 处 + 宿主 AST 链 9 处; char/数字/bool 的 `to_string()` 保留 (无 `Into<Arc<str>>` 直路)
- **实测** (E=`rs8_shallow` vs D2=`rs8_arcstr`, 同窗口配对 min + A/A):

| 负载 | D2 (ms) | E (ms) | E vs D2 | 判读 |
| --- | --- | --- | --- | --- |
| str2 (新靶点探针) | 1151 | 1133 | **−1.56%**, 配对 5/5 全胜 | 改动面实益 (split/replace/exec/stringify 混合环) |
| rich_long | 7516 | 7470 | −0.6% | 同向, 噪声带内 |
| oct4 | 25319 | 25295 | −0.1% | 持平 |
| add1 | 166 | 168 | **+1.2%**, 配对 9/10 为正, 三窗口复现 | 布局代价 |
| add5 | 394 | 395 | +0.3~0.7% | 同上 |

- **add1/add5 +1% 归因**: 两负载不执行任何被改代码 (无字符串位), 差异只能来自 v8-interpreter 码缩/内联扰动分发环布局; 重编译 e2 (`/tmp/rs8_shallow2`, 源码同、build-id 异) 与 e 逐毫秒贴合 (169/401) → 代码生成确定性, 该代价是稳定属性非测量噪声。战役 D (分发环重写) 会整体覆盖此布局, 不为 +1% 做局部补丁 (与 #29 教训同构)。**净判断: 按战役轴记账 —— 字符串路径实测变快, 纯算术探针付 ~1% 布局税, 值模型合一方向不变**
- **门禁**: 18 组跨版本对拍逐字节全等 (oct4 归一后等) + 新探针 str2 三端 (D2/E/node) 全等 `45600061`; `./test.sh` **2965/1skip**、`syntax` **76/76**、`v8-int` **33/33**、clippy 0 warning

### C-1 收官状态

① 宿主字段对齐至此完结 (String 半刀 + 浅包装半刀)。下一刀进 ②: 引擎补 `Symbol`/`RegExp` 变体 (最大回归面 = prelude `NativeFn::Ptr`↔`InterpNative`, 单列一步)。

### 12.48 战役 C-2a: 引擎 `Value::Symbol` + 跨桥直通 (第二十二轮, 2026-10-17)

② 前半。C 节勘察 (12.46 钉入) 定案后动刀: 引擎 `Value` 补 `Symbol(String)` 变体, 桥两向**停止降级** —— 此前 BC 链 `Symbol("k")` 经 host→engine 回程被折成 `Undefined` (bytecode_bridge 两处 `JsValue::Symbol(_) => Some(Undefined)`), typeof/取值/键别名全塌。先钉后修: 门禁 77 落定时 BC 链实测 `t1=undefined#t2=undefined#...#t7=9|9` (s1/s2 同为 "undefined" 键互相覆盖), AST 链/node 均绿 —— 用例先复现, 改毕三方全等 (含 `String(Symbol())=="Symbol()"` 与 `"Symbol(k)"` 字符串键不别名)。

- 引擎侧: `Value::Symbol(String)` + 穷尽臂 6 处补 (Debug/type_of `"symbol"`/to_boolean true/to_number NaN/to_str_lossy `"Symbol(desc)"`/PartialEq+test_equal_strict 按描述相等) + `to_element_key` 新法 (仅 Symbol 走 `@@symbol:` 编码, 其余落 `to_str_lossy` —— 属性名与枚举面不经此口, String(sym) 保持 "Symbol(k)" 口径), 5 个键控 opcode 改走 `to_element_key` (GetKeyed/SetKeyed/Delete/DefineKeyedOwn/TestIn)
- 一处定义: `format_symbol_key` 从宿主 value.rs 迁引擎 (SYMBOL_PREFIX 本就在引擎), 宿主改 `pub use` 再导出; 引擎 `SYM_TO_PRIMITIVE` 读点无需动 (Symbol 值非可调用, 与原 Undefined 同样跳过, 20 组对拍实证)
- 身份模型挂账: 按描述串相等 (同描述两 Symbol 相等) —— node 唯一 id 语义、`sym.description`、隐式 `""+sym` 抛 TypeError 三项留 ② 后半或 descriptor 步统一; 用例已避开这三条依赖面
- RegExp 半刀 (C-2b) 另立: 宿主 `RegExpData` 含 `compiled: fancy_regex::Regex` (引擎无此依赖) + `props: HashMap<String, JsValue>` (类型跨不过去), 引擎变体必须先拆"内核数据 (pattern/flags/last_index/compiled) 与 expando 袋分家" —— 勘察结论, 下一轮开题
- **实测** (S=`rs8_symbol` vs E=`rs8_shallow` C-1b 末, 同窗口配对 min + A/A, load 3.26→1.57):

| 负载 | E min | S min | 判读 |
| --- | --- | --- | --- |
| add1 | 170 | 171 | +0.6%, 配对混号, A/A 带内 |
| add5 | 406 | 405 | 持平 |
| str2 | 1162 | 1169 | +0.6%, 5/5 微负 (A/A 同幅散布) |
| oct4 | 25547 | 25517 | 持平 |
| rich_long | 7760 | 7865 | **+1.3%** 3/3, 归因分发环码扰 (12.47 同机制, 战役 D 覆盖) |

主探针 (add1/add5) 无回退; str2/rich 的 +0.6~1.3% 与 12.47 布局税同族 (引擎 crate 码动 → 分发环扰动), 记合成债, 不在值模型未合一时做局部补丁。**门禁**: `./test.sh` **2966/1skip** (新增 77)、`syntax` **77/77**、`v8-int` **33/33**、clippy 0、20 组跨版本对拍 (含 str2 两链) 逐字节全等

### 12.49 第二十三轮: 门禁 78 ToPrimitive 分叉修复 + 主线归档压缩 + C-2b 第一刀拆家 (2026-10-17)

三件事同一轮: 前置分叉修复 (78)、用户指令的文档压缩归档、战役 C-2b 第一刀 (内核/袋拆家)。

- **① 门禁 78 `test_builtin_function_toprimitive_add` (both_chains)**: `"" + 内建函数` 长期链分裂 —— AST 链 `eval_to_primitive` 对 Native 接收者查 valueOf/toString 均 Undefined, 回环耗尽抛 `TypeError: Cannot convert object to primitive value`; BC 链 Native 臂原型链漏到 prelude `Object.prototype.toString` 吐 `"[object Function]"`; node 走 `Function.prototype.toString` 给具名源码形。先钉确认红 (AST 实测 TypeError), 再两链各一处修复: AST 在抛错前对 `Function|Native` 兜底到与 `String(f)` 同源的 `to_js_string` 摘要 `"function () { [native code] }"` (interpreter.rs), 引擎在 `Native(_, props)` get_property 臂 CALL/APPLY/自有袋之后、PROTO_KEY 回溯之前自合成本机摘要 (一处口径 = `to_str_lossy` Native 臂, bytecode_dispatcher.rs), 原型链上用户覆盖仍由袋命中先行兜下。**残档声明**: `f.toString()` **调用**面 AST 仍 TypeError (Native 无 Function.prototype 面物化), 内建函数名物化属 12.27 prelude 单源化工程, 本钉有意不覆盖; node 具名形 `"function () { [native code] }"` 与摘要口径差异同挂 12.27
- **② 主线归档压缩 (用户指令"清理过时的文档，压缩释放上下文")**: 12.9–12.45 历轮细账逐字搬入新建 `todo-archive.md` (686 行, 只搬不删不改), todo.md 第十二节替换为归档索引头 + 37 行轮次索引, 主线 1461→825 行。此后"见 12.xx"交叉引用一律指归档
- **③ C-2b 第一刀 拆家 (语义零漂移刀)**: 按 12.48 勘察结论, 宿主 `RegExpData` 拆成 `core: Arc<RegExpCore>` + `props: Arc<RwLock<HashMap<String,JsValue>>>` (expando 袋因 JsValue 类型跨不过 crate 边界留守宿主)。`RegExpCore` (pattern/flags/global/ignore_case/multiline/compiled: fancy_regex::Regex/last_index: `Arc<RwLock<usize>>`)、规范化编译 (i/m 折 `(?i)(?m)` 前缀、类外 `[` 转义、`\xHH>=0x80` 改 `\u{HH}`)、`regex_split_parts` 全部**逐字**迁入新建 `v8-interpreter/src/regexp.rs` = 一处定义; 引擎 Cargo.toml 加 `fancy-regex = "0.19.2"`; 宿主删 `fancy_regex` 直接依赖、`new_regexp` 委托 `RegExpCore::new` (错误串单包装, `"Invalid RegExp '<p>': "` 前缀留宿主)、宿主 `regex_split_parts` 改 `pub use` 再导出。桥侧 `regex_exec_core` 改收 `&Arc<RegExpCore>` (`InterpNativeFn::from_context<T: Send+Sync+'static>` 泛型天然兼容), exec/test/split 三闭包统一 `Arc::clone(&re.core)`; 字面量改写/回程还原/boolean_flags 等触点同步。全链 ~57 触点经编译器驱动改写。引擎新增 `tests/regexp_core_test.rs` 3 测 (规范化编译/标志面/门禁 68、70 split 形制钉)。第二刀 (引擎 `Value::RegExp(Arc<RegExpCore>)` 变体) 可直持 core 跨桥浅拷贝共享身份, 消解物化/还原/@@split 哨兵/@@shadow 墓碑/字面量改写全套机器
- **记时未跑 rationale**: 本刀为纯结构搬移 + 输出全等验证 —— 13 项正则探针 (`target/scratch/re_probe.js`) 三链逐字节全等 (`chains identical: True | match pre-split: True`), 无任何行为面变化; 下轮记时若现 ± 归因本刀
- **新踩坑入账 (纪律)**: `./test.sh` 与 `clippy --fix` **不得并发跑** —— 本轮并发导致 clippy --fix 在测试构建期间改 4 个文件, 结果不可信; 审 diff (仅换行整形) 后串行复跑全量 + clippy 双绿方认门禁
- **环境行为记录 (不追改)**: HEAD 被用户侧自动提交进程 19:18 重写为 `357faa4 "init"` (吞并 b466452 起全部在途差异, 此前提交不可达); `/tmp` 被系统清空, E 节历轮对照二进制清单失效, 工件迁 repo `target/scratch/` (天然 gitignore); `.gitignore` 新增 `.scratch/` 行为环境侧编辑, 非本轮改动
- **门禁**: `./test.sh` **2970/1skip** (78 + 引擎 3 测入账)、`syntax` **78/78**、`v8-int` **36/36**、`PATH="$HOME/.cargo/bin:$PATH" ./sh/clippy.sh` rc=0 零警告、正则 13 项三链逐字节全等

### 12.50 第二十四轮: C-2b 第二刀 —— 引擎 `Value::RegExp` 变体, 物化/还原/哨兵/墓碑机器退役 (2026-10-17)

战役 C ② 收官刀: 引擎补 `Value::RegExp(Arc<RegExpCore>, Arc<RwLock<HashMap<String, Value>>>)` 变体 (仿 Native 双字段形), BC 链正则不再物化成 Object。

- **① 变体与全臂 (bytecode_dispatcher.rs)**: `new_regexp(core)` 构造 (袋空); 判等两处 = `Arc::ptr_eq` 内核 (跨桥浅拷贝=同一定身, 两字面量互不等, node 口径); Debug `/pat/`、type_of object、to_boolean true、to_number NaN、`to_str_lossy` 吐 `/pat/flags` (regex_repr 机器删除, resolve 兜底同步); **get 臂**次序 = 袋 → 派生面 (source/flags/lastIndex 走内核 Number/六布尔读 `boolean_flags`) → 内建面 (exec/test 走 `NativeFn::from_context` 直持内核、hasOwnProperty/propertyIsEnumerable 走 `NativeFn::Ptr` trampoline 读接收者袋、toString `ConstString` 摘要), 兜底 Undefined 交 dispatcher 全局原型回落; **set 臂** lastIndex 专落内核 `*core.last_index.write()` (Smi 快照搬运机器退役)、只读名册 `is_regexp_readonly_key` 静默 (两链共读保留)、其余入袋; **delete 臂** 只回收袋 (派生名删不动交 true, sloppy 与宿主同口径); has_property_resolved/ForInEnumerate/JSON/to_str_lossy Object 臂/`string_split_inlined` 变体快路径 (直调 `regex_split_core`, 不再经 `@@split` 委托) 全部补变体臂
- **② 一处定义迁引擎**: `regex_exec_core`/`regex_test_core`/`regex_split_core` + 三 native 构造器从桥逐字搬入 `v8-interpreter/src/regexp.rs`; lastIndex 读写判定收 `tracks_last_index()` (global|sticky), 接收者面扩 `Object|RegExp`
- **③ 桥两向浅拷贝 (bytecode_bridge.rs)**: 物化臂 `JsValue::RegExp` → `InterpreterValue::RegExp(Arc::clone(&re.core), 袋逐键转换)` —— **内核 Arc 跨桥共享 = lastIndex 单一真相**, 不再逐跳搬运; visited 按 `Arc::as_ptr(core)` 防环; 还原臂 `regexp_from_core` 复用内核不重编译。退役机器 ~200 行: SPLIT_IMPL_KEY `@@split` 哨兵、REGEXP_SHADOW_PREFIX `@@shadow:` 墓碑、is_regexp_pseudo_key/is_regexp_method_key 名册、regex_repr、物化重建臂、还原哨兵臂、桥端 regex_exec_core、set/delete 护栏 —— 全删, `lib.rs` 导出同步收口
- **④ 本刀踩坑 (三处, 全部先复现后修)**: (a) `construct_moved` 的 `matches!(res, Object|Array)` 组合判定不认识变体 → 字面量 `new RegExp` 结果被静默丢弃、boxed_primitive 回退 → BC 链 "Undefined is not callable"; 逐表达式二分定位, 修 is_object_like/TestIn/construct 两支/is_instance_of 兜底 6 处, 并顺带 **Construct 时把 new_obj 原型链搬进袋** —— `instanceof RegExp` 旧 BC 世界为假 (物化对象无 proto 链), 本刀后与 node 对齐 (修复非回归); (b) 袋内 `__proto__` 泄漏回宿主名册 (探针 t7/t9) → 还原臂过滤 `PROTO_KEY`+`is_internal_key`; (c) **内建面扫第十二项 `test_internal_surface_sweep_twelve_regexp_enumeration_surface` (both_chains) 当场再证双链钉的价值** —— 引擎 hasOwn trampoline 只认袋 → BC 链 `hasOwnProperty("lastIndex")` 塌 false, 与宿主 `RegExpData::own_property_names` (lastIndex 恒自有+袋, node p1/p2 实测) 分叉; 修: 引擎自有册对齐宿主 (hasOwn = `lastIndex`∈册∨袋, propertyIsEnumerable 只认袋 —— lastIndex 自有不可枚举), 引擎级测试补 4 断言钉两口径分界
- **记时 (BC 链, A=`rs8_c2b1f` 第一刀基线 vs B=本刀, 同窗口 ABBA 交错 min-of-5, 每采样点 load<6 守门 (窗口 5.2~6.5), A/A 噪声带 ≤0.3%)**:

| 靶点 | A(min) | B(min) | Δ | 归因 |
| --- | --- | --- | --- | --- |
| re1 (循环字面量+exec) | 1586 | 1396 | **−12.0%** | 消物化/还原往返, 本刀主战场 |
| split1 (正则 split) | 499 | 332 | **−33.5%** | 变体快路径直调内核 |
| add1 | 324 | 333 | +2.8% | 布局税 (e2 重编复测 +2.4%, 稳定属性非噪声, 12.47 同族) |
| add5 | 446 | 463 | +3.8% | 同上 (e2 +2.9) |
| str2 | 75 | 77 | +2.7% | 同上 |
| re2 (缓存正则+test) | 311 | 320 | +2.9% | 半布局税 + 归因假设: get 臂内建面每次现构 Native (Arc 分配) vs 旧查表克隆, 未 profile 实证 —— 记合成债, 不为 ±3% 做局部补丁 (D/④ 覆盖) |

  AST 链对照: add1 +4.1/add5 +1.1/str2 0.0/re1 +1.5/split1 +3.6 —— 宿主巨函数扰动同族。**净判断: 正则路径两位数提速, 算术探针付 ~3% 布局税, 值模型合一方向不变**
- **跨链观测入账 (本刀前既有, 机制已核实)**: re1 靶 AST 链 65ms vs BC 链 1396ms (~20×, 旧二进制同形) —— 根因: 宿主 parser.rs:1281 把正则字面量在**解析期**预编译成 `Expr::Literal(re)`, 同一字面量节点每轮求值复用同一实例 (实测 `g()===g()` AST=true / BC=false / node=false —— AST 侧违规范"每次求值新对象", 且共享 lastIndex 可跨调用泄漏); BC 链按规范每跳走 `new $$RegExp` 重编译。归勘察项: per-eval 新对象语义 × 已编译内核按 pattern+flags 缓存可兼得 (V8 即编译缓存+实例新建), 挂 ④ 删桥后统一处理; 两链输出 12 组对拍全等 (六靶点×两链, 新旧二进制结果字段逐字节)
- **挂账声明**: expando 袋身份合一 (两链各一份袋、逐跳键转换) 与 `hasOwnProperty("__proto__")` 边角挂 ④ 删桥; `delete r.lastIndex` 真删语义挂 #18 旗标族 (注释已标同源); `f.toString()` 调用面仍挂 12.27; Symbol 唯一 id 三项 (12.48 记) ② 收官后改挂 ④
- **门禁**: `./test.sh` **2975/1skip** (新增引擎变体测试 5)、`syntax` **78/78**、`v8-int` **41/41** (36+5, 新建 `v8-interpreter/tests/regexp_variant_test.rs`: 属性面/expando 遮蔽灯塔/跨跳身份共享/test 委托与字符串方法变体路径/错误路径不 panic)、clippy rc=0 零警告、正则 13 项探针三链 (BC/AST/default) 与基线逐字节全等 (修复自有册后复跑复验)

### 12.51 第二十五轮: 战役 C ③ TheHole 数组洞语义收官 + `for_cross_engine_call` 空 global 根因修复 (2026-10-17)

战役 C ③: 洞从"Undefined 填槽近似"升级为真哨兵，表示面 (门禁 79) + 行为面 (门禁 80) 双链钉绿。

- **模型定案**: 洞哨兵 `Value::TheHole`/`JsValue::TheHole` 只栖身数组元素槽；**读面在汇合处物化 undefined**（一处定义 helper `materialized(&Self)->Self`，引擎 `bytecode_dispatcher.rs` 与宿主 `value.rs` 各一份镜像，`get_element`/`get_property` 数字臂/`GetKeyedProperty` 快路全走）；**存在面洞即不存在**（`in`/gOPN/keys/for-in/entries/values/assign 一律筛洞；`delete` 落洞位）
- **写入侧唯一洞入口**: parser elision → `Expr::Literal(TheHole)` → 桥 `AstValue::TheHole` → 引擎 generator 既有 `load_the_hole` → **`StaInArrayLiteral` 臂**直存哨兵（绕开物化兜底）；运行期一切写路径 (`set_property`/`SetKeyedProperty`/`set_element`) 物化写入 + resize 增长以 TheHole 填洞；`Array(n)` 单参构造以 TheHole 起底
- **内建行为面 (两链各自补齐、both_chains 逐字节钉)**: includes 洞当 undefined；indexOf/lastIndexOf 靠严格等天然跳洞；**map 保洞**（结果整表以洞起底、洞位回调不跳失）；forEach/filter/every/some/find/findIndex/flatMap 跳洞；reduce/reduceRight 缺省种子扫首个实位、全洞抛 TypeError；spread/`Array.from` 落密；concat/slice 保洞；sort 洞沉底；reverse 洞照搬；join 空段、JSON null（既有兜底复用）；宿主 prelude JS map/forEach/filter/every/some/reduce 以 `this.hasOwnProperty(i)` 跳洞
- **本最大事件——`for_cross_engine_call` 根因修复**: 原构造空 `Environment::new()` 作 global，宿主预置函数体（BC 链里 `Array.prototype.every` 等 prelude JS 经 `.call` 被引擎回调时）的原型回落 `proto_prop → global.get("Array")` 整个失明——test262 回归 `hasOwnProperty: undefined is not a function` 暴露此既有地雷。修复: 签名改收 `root: Arc<Environment>`，桥传 `Arc::clone(&func_data.closure)`（词法闭包链达真全局）。教训记账: **预置 JS 新增任何 `this.<内建>` 依赖都会踩空 env 陷阱**，该陷阱对全部宿主预置函数跨引擎回调生效
- **顺带修两链观测偏差**: `to_number` 数组单元素臂对齐 node —— `+[undefined]`/`+[null]`/`+[,]` 交 0.0（原两链交 NaN），双链同改
- **clippy 挂死绕行（机器纪律新增）**: `sh/clippy.sh` 的 `cargo clippy --fix` 连续两轮卡死在 `cargo fix` 诊断服务器 `Message::post → recvfrom`（clippy-driver 子进程先亡、父等 TCP 诊断，75 分钟零 CPU 零产物）——cargo fix 的 TCP 诊断机件 bug，非本仓问题。绕行: 跑等效**只查**命令 `cargo +nightly clippy -q --tests --all-targets --all-features -- -D warnings -W clippy::absolute_paths` → **0 警告**（`--fix` 改写量本就为零）；`fixrs`+`cargo fmt` 在卡死点前已跑完、源文件 mtime 核验未动
- **记时 (A=`rs8_c2b2f` vs B=`rs8_c3f`，同窗口 ABBA min-of-5；A/A 底 add1 +1.8%/hole1 0.0%)**: BC 链 add1 −1.2/add5 −0.4/str2 0.0/**hole1 −1.2**；AST 链 add1 −2.5/add5 −1.1/str2 −1.8/hole1 0.0 —— **全落噪声带，③ 运行期代价（materialized clone + is_hole 读）零回退**；hole1 为本轮新靶（20 万次四元素字面量落槽+slice+元素读写，钉 `StaInArrayLiteral`/读面改动热路）
- **性能债记账**: prelude 跳洞法 `this.hasOwnProperty(i)` 走 gOPN+indexOf = O(n) 每槽 → 大数组洞扫 **O(n²)**；④ descriptor 落地后 hasOwnProperty 转 O(1) 自然消，不在 prelude 上做局部补丁
- **环境行为记录 (不追改)**: HEAD 被用户侧自动提交进程重写为单 `779de58 "init"`（reflog 同步，此前提交不可达），③ 大部分编辑已被吞入快照；当前未提交差异 = 四源文件 +53/−23（`rs8/src/runtime/{interpreter,value,bytecode_bridge}.rs`、`v8-interpreter/src/bytecode_dispatcher.rs`，为快照后追加的根因修复与 to_number 对齐）+ `rs8/tests/syntax_corner_cases.rs` 门禁 79/80 在册 + todo.md 记账；内容正确性以全量门禁为证
- **门禁**: `./test.sh` **2977 passed / 1 skipped**（基线 2975 + 门禁 79/80）、`syntax` **80/80**、clippy 只查 0 警告
- **残档 (两链一致、非链分裂)**: `flat` 的洞处理、单洞数组 `length` 缩短等维持两链同款近似；expando 袋身份合一/删桥/prelude `hasOwnProperty` O(1) 化都挂 ④
- **下一入口**: 战役 C **④ 删桥 + descriptor 接 Map**（`todo.md` 十五 B 拆分顺序行）


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

- 最大风险: 双实现长期并存 → 修的是将被删除的副本。战役 A 已收官 (默认 `BytecodeFirst`), 该风险现在的具体形态是 **回落路径与宿主内建**: 每个套件开工前先用 `--print-bytecode-coverage` 确认没整段落回 AST; 宿主侧内建 (`rs8/src/runtime/interpreter.rs` `init_builtins`) 仍在往 `JsValue` 上长语义, 战役 C 合一后必须整体搬进引擎侧
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

---

## 十五、当前待办 (2026-10-15 压缩收口，只留未完成项)

上下文压缩后的**唯一入口**就是本节；细节按引用的小节读，不要把 12.x 流水账重新抄进对话。

### A. 性能 (主矛盾，决定能否超越 V8)

| 项 | 内容 | 归属 | 准入 |
| --- | --- | --- | --- |
| #28 | 分发环单条指令开销 —— bc 每字节码 ~11-14ns (探针见 12.17 结构结论)。单位成本两链接近，bc 输在可执行单元数 | 战役 D (连续寄存器栈帧) + 战役 C (值模型合一) | ~~同负载窗口 `add1`/`add5` 探针单位成本下降且 `test.sh` 全绿~~ **12.45 窥孔首达** (add1 −5.8%/add5 −6.9%, 单元数 −13.9%); 深账仍归 C/D |
| #29 后续 | 12.18 已把字面量收益归因清楚，**禁止再在字面量/属性臂上做局部补丁** | — | 已闭 |
| 12.4 | 宏观对标: `lto=fat` + `codegen-units=1` 主干聚合，同机对标 `d8`，Speedometer 3.0 严格区分 Cold Start / Steady State | 战役 H 前置 | — |
| 12.2 | 分析工具链落地: samply 混合栈热点 + 分配逃逸 + PMU 采 L1-I/L1-D/分支预测 (重点分派环) | 服务 #28 | — |
| 战役 H | **没有 JIT 就不可能超越 V8** —— node 1.00x / node jitless 1.12x / rs8 bc 2.22x (12.10 窗口)，差距主因是 1.3 第 7 条 | `v8-baseline` → `v8-maglev-*` → `v8-turboshaft` | — |

### B. 语义缺口 (开工前先在 `rs8/tests/` 加 `both_chains` 用例并与 node 三方对照钉住)

- #18 `Object.defineProperty` 的描述符属性 (`enumerable`/`writable`/`configurable`) 全被丢弃 —— 值模型无处存，实质归战役 C
- #23 残项 (主体已闭见 12.22, ⑤ 已闭见 12.24): ⑥ 内建方法物化到 `Array.prototype` (`.toString()` 现状 undefined, node 给 native code) —— 与战役 C 联动
- #30 计算键 `__proto__` 被当原型链接 (node 当自有数据键) —— 值模型无法区分两者，战役 C
- #21 回退粒度 (整程序 → 按函数/语句)：优先级最低，`--print-bytecode-coverage` 已 100% (179 脚本 clean=179 / ast_fallback=0)
- 正则长尾面 (12.23 记账, **split 捕获组交叉已于 12.35 闭合** `regex_split_parts` 单点; **标准正则枚举面已于 12.36 闭合** `own_property_names`/读臂回落/for-in `is_regexp_pseudo_key` 三单点; **expando/方法遮蔽面已于 12.37 闭合** props 袋 + 桥两向搬运 `@@shadow:` 墓碑 + `json_object_body` 单点): match 字符串模式按 RegExpCreate 解释 (元字符分歧)、`re.compile` (node typeof function)、正则原型实体面 (`getPrototypeOf(r)` node `/(?:)/` / 两链 null、`r.exec===RegExp.prototype.exec` 身份、桥名身份 `String(r.exec)` "[object Function]") —— 归引擎正则内核与战役 C 值模型 (战役 H 前置)
- 内建面扫面三轮残档 (12.27 记账, 全部挂战役 C 值模型/执行态合一): 回调跨不了引擎执行态 —— `Array.from` mapfn、`replaceAll` 函数替换实参、`keys/values/entries`+`matchAll` 迭代器协议、`Function.prototype.bind` (call/apply 已由哨兵闭环); 宿主 Error 名册缺 URIError/SyntaxError 变体 (URI malformed 与 JSON.parse 暂以 RuntimeError 顶); BC 跨桥深拷贝丢身份/变更 (12.26); 普通对象 `.constructor` 物化; astral `.length` UTF-16 面 (BMP 豁免); freeze/seal/preventExtensions/getOwnPropertyDescriptor 与 #18 同根 (缺可变性/属性旗标存储)
- 内建面扫面十四轮残档 (12.38 记账, 均两链一致、非链分裂): 数组面解构按索引读非可迭代 RHS (node TypeError `5 is not iterable` / 我们绑 undefined；`null`/缺实参模式参交 RuntimeError 顶 —— 迭代器协议与错误名册同挂战役 C/三轮七轮族)；~~for-of 全缺 (独立批次)~~ **已于 12.41 闭合**；入口名册: 括号表达式位解构、catch 形参模式、计算键模式、for-in 变量位模式 (for-of 位已闭, 见十七轮残档)、嵌套对象剩余 (显式 ParseError) —— 其中箭头解构形参与简单形参默认**已于 12.39 闭合**；BC `Object.defineProperty` 整体失效 = 12.26 跨桥深拷贝同墙 (g130/g133 定案, 战役 C 唯一正解)
- 内建面扫面十五轮残档 (12.39 记账, 均两链一致、非链分裂): ~~rest 形参 `f(...a)` 两链 ParseError~~ **已于 12.40 闭合**；`function f(a=a)` 自引用默认交 undefined / node TDZ 抛 (TDZ 全局缺, 与 let/const≡var 同根)；解构/默认值报错的 name/message 保真挂四轮"caught error 文本"族
- 内建面扫面十六轮残档 (12.40 记账, 两链一致、非链分裂): `f(a = r, ...r)` 默认值前向读 rest 绑名交数组 / node TDZ ReferenceError —— 并入上一条 TDZ 家族 (`a=a` 同根)，待 TDZ 执行态整批收口
- 内建面扫面十七轮残档 (12.41 记账, 两链一致、非链分裂): for-of 非可迭代 RHS 静默零轮 (f12/f15, 迭代器协议 · 战役 C, 与十四轮数组面解构同根)；for-of let/const 头部逐轮捕获 `3,3,3` 与块级作用域泄漏 (f2/f11, 11.6 系统性, 与经典 `for (let i…)` 同族)；`for (const [a] = b of x)` 类经典头解构初始化 ParseError (挂参数表/解构初始化既有族)；for-in 变量位解构模式仍缺 (for-of 位已闭)
- 内建面扫面十三轮残档 (12.37 记账, 均两链一致、非链分裂): `String(带 toString 遮蔽正则)` 交 `/d/` / node `CUSTOM` —— 与十轮 `String(c)` 直转两轨同根 (战役 C 转换面合一课题；隐式 `+`/模板已走 ToPrimitive 交 CUSTOM)；`Object.prototype.toString.call(正则)` 交 `[object Object]` / node `[object RegExp]` (七轮 toString.call 名册同根)；BC 遮蔽态方法名 delete 拦删不回落代管基形 (AST 删袋回落原生、node 回落原型 —— 三向边缘象, 跨桥身份族)；`@@shadow:` 墓碑内部键随正则往返恒驻引擎表 (枚举口已滤, 无外部可见性)
- 内建面扫面六轮残档 (12.30 记账): `JSON.stringify` 的 replacer 函数参、`toJSON` 钩子、循环引用 TypeError 全缺 (与三轮"回调跨执行态"同族, 挂战役 C)；字符串步长截 10 按码点而非 UTF-16 码元 (与三轮 astral 残档同根)
- 内建面扫面七轮残档 (12.31 记账): `Object.prototype.toString.call(基元)` 应交 `[object String]` 类 tag、宿主替身交值本身 (与四轮"prelude JS 函数 `.call` 属性读"同族, 挂战役 C)；Number/Boolean 基元接收者的原型方法全名册未对齐 (本轮只收字符串接收者 `hasOwnProperty` 一臂, 宿主闭表逐键补臂是战役 C 值模型合一前的过渡形态)
- 内建面扫面八轮残档 (12.32 记账, 均两链一致、非链分裂): 全局环境名册缺 —— `globalThis`/`setTimeout`/`setInterval`/`queueMicrotask`/`Object.groupBy`/`structuredClone` (node 全 function/object；timer 面另需事件循环收口语义，挂独立批次) (`Object.hasOwn`/`String.raw` 已于 12.33 补齐)；`localeCompare` 缺 (ICU collation 无基础设施，naive 码点序实测对 node 高频分叉，非顺手活)；`Object.freeze`/`isFrozen`/`getOwnPropertyDescriptor` 未挂键 (十五 B 三轮旗标存储族)；Symbol 面缺致 `Number(symbol)` TypeError 象无法表达
- 内建面扫面九轮残档 (12.33 记账, 均两链一致): 静态面 census 余 9 —— `Object.freeze/seal/isFrozen/isSealed/isExtensible/preventExtensions` (旗标存储族, 三轮同根)、`getOwnPropertyDescriptor(s)` (同根，需先定值/旗标存储模型)、`groupBy` (ES2024, 已于 12.34 补齐)；`String.raw` 真模板面 (cooked + 模板对象) 仍随 tagged templates 挂四轮；Date 子系统两链全缺 (g90 探针 `Date.getUTC*/toISOString/JSON.stringify(Date)` 双挂，另批)
- 内建面扫面十轮残档 (12.34 记账): `String(null原型对象)` 直转不抛 (宿主 `to_js_string` 对 Object 硬编码 "[object Object]" 短路, node 抛 TypeError；隐式 `c+""` 已正确抛 —— 显式/隐式两轨收敛到真 ToPrimitive 一处是战役 C 转换面课题)；`map/filter/forEach` 对字面量洞位不跳 (回调收 undefined, node 跳洞保洞, 密 Vec 洞模型, 与四轮 `0 in new Array(2)` 同根)；`Object.keys` 桶序两链互抖 (四轮 for-in 插入序同根, 门禁以 `.sort()` 隔离)；Map/Set 集合族全缺 (`typeof Map` node function / 两链 undefined, 独立批次)；census 复点余 8 (freeze 族 6 + descriptor 族 2, 见 12.34)
- 内建面扫面五轮残档 (12.29 记账, 均挂战役 C 值模型): 真洞语义 (`delete a[1]` 后 node `1 in a` false 且 `Object.keys` 跳过, 密 Vec 以 Undefined 填槽交 true/含洞位 —— join 空段与索引读 undefined 两象已对齐)、`delete` 非配置属性 (defineProperty 存的 `configurable` 被丢, node false / 我们 true, #18 同根)、`eval("delete x")` 直接 eval 局部变量 (node false)、for-in 对象键**插入序** (HashMap 桶序) 与**继承键枚举** (不沿原型链)、数字/null/undefined 接收者 for-in (node ToObject 走原型链吐方法名, 我们零轮 —— 两链一致)
- 内建面扫面四轮残档 (12.28 记账): **解构族**全缺 (var/let/const 数组/对象模式、默认值、rest、重命名、形参解构、赋值式解构 —— 探针即挂此; **主躯干已于 12.38 闭合**，入口名册与迭代器协议残档见十四轮残档); ~~**标记语句** (`outer: for` 报 Unexpected prefix token Colon)~~ **已于 12.44 闭合**; caught error 的 String()/name/message 保真 (两链任何被捕获错误都交 `[object Object]`, node 交 `TypeError: msg`); tagged templates (lexer 显式 Err); new+spread; 实参展开的 receiver 双求值 (`recv().m(...a)` 里 recv 跑两遍, apply 脱糖固有偏差, 两链一致); 箭头非标识符形参 (解构/默认值/rest 已于 12.38/12.39/12.40 全闭并入参数表单点); prelude JS 函数的 `.call` 属性读 (AST undefined / BC `[object Function]` / node native 源码串); 数组洞 (`0 in new Array(2)` 两链 true, node false —— 密 Vec 无 hole 位); 原生函数 `.length/.name` 元数据面 (`Math.max.length` 两链 undefined, node 2 —— NativeFn::Ptr 无形参位); BC 链原型链 identity 全集 (`o.__proto__ === Object.prototype`/`Object.getPrototypeOf([]) === Array.prototype`/isPrototypeOf 命中例 —— 与跨桥深拷贝同族, 门禁 54 只钉对齐子集)

### C. 死代码与依赖卫生

- D1: **已清账** (12.19) —— shear errors 34→0；47 条未链接文件全为 `tool/` 脚本再生成的 `*_generated.rs`/`*_tq.rs`，记豁免不删，挂接归战役 F/G/H 准入
- D2: **已分诊收口** (12.19) —— 全量 139 处: 链内 6 处删净 (含 `v8-execution-isolate` 3 个桥接占位与连带 3 个依赖)，链外 133 处挂 H/F/G/B/C 战役号、不删
- 待跑项已并入 D2 全量重跑，不再单列

### D. 发布与对标

- 12.8 上游接入 (fork webc-fork/js-engine-benchmark、`info.json` 注册、`scripts/update.ts` 版本解析、自动构建) —— 前置是全平台 Actions 通过 + 实测跑分超 V8
- 12.3 微观基准旧 ns/op 数字仍按新口径重测 (走 CLI 主链路且与 d8 同机对比)

### 已闭环、无需再讨论 (仅留账，勿重做)

战役 A (默认 `BytecodeFirst`，回退面归零)、#17、#19 (数组 `lastIndexOf/fill/unshift/find` + `thisArg`，见 12.19)、#20、#22、#24 (Array.prototype 覆盖遮蔽内建替身，见 12.21)、#23 主体 (replace/match 回调 + $ 序列 + exec index/input，见 12.22，⑤ 见 12.24，残项 ⑥ 在十五 B)、函数 toString 源码文本 (parser span 五路同源, 见 12.24 与门禁 37)、正则对象方法面收敛 (bridge 撤 replace/match/split 公开挂键, split 走 `@@split` 哨兵; boolean_flags 一面表 + AST `in` 修复, 见 12.23 与门禁 36)、#25/#27 (字面量走 CreateDataProperty + 容量预留 + 累加器作废，收益见 12.18)、内建面扫面三轮批次 (12.25-12.27，门禁 38-48；含 needle 缺省/`Math.sign` 口径/空针 replaceAll/`\u{...}` 转义四处真实缺陷修复；残档在十五 B)、内建面扫面第四轮 (12.28，门禁 49-54：箭头函数两链全量 + 模板分组/spread 展平/assign 物化 accessor 三处两链同犯缺陷 + toExponential/trim 别名/枚举面；残档在十五 B)、内建面扫面第五轮 (12.29，门禁 55-57：delete sloppy 恒真口径两链收敛 + 数组下标留洞近似 (`delete_array_slot` 单点) + AST for-in 字符串接收者补臂；typeof 名册与 == 强制表整表零分叉收钉防回归；真洞/非配置删除/for-in 插入序与继承面残档在十五 B)、内建面扫面第六轮 (12.30，门禁 58-60：indexOf/includes/fill fromIndex 负数归一两链共用 `tail_index` 单点 + Number() 进制前缀接线 `string_to_double` 一处覆盖 ECMA 9.3.1 全表 + JSON.stringify 缩进模型订正 (步长串同样插换行、截 10、紧凑集) + 顶层 undefined 回吐 undefined + clamp NaN panic 拆除；replacer/toJSON/循环引用残档在十五 B)、内建面扫面第七轮 (12.31，门禁 61-63：prelude reduce 的 IsCallable 首行抛 (零轮也抛) + hasOwnProperty 自有口径重写 (gOPN+indexOf，`.some` 跨桥回调作废) + 宿主字符串基元 `StringHasOwnProperty` 回落臂 (链分裂修复) + `getPrototypeOf(函数)` 宿主臂 + split 缺省针 `[whole]`/limit ToInt32 负数全量引擎侧补齐；位运算/一元加负/try-finally 控制流整表零分叉收钉；toString.call 基元 tag 与 Number/Boolean 接收者名册残档在十五 B)、内建面扫面第八轮 (12.32，门禁 64-65：String.prototype.lastIndexOf 两链全缺补齐 (`str_last_index_of` 单点、宿主复用；node 63 格全表口径 —— 负 pos 只扫 0/NaN-undefined→len/窗口按针 UTF-16 码元长，与 `tail_index` 从尾归一**不同源**) + 探针抓修多字节串跨字符边界切片 panic 两起 + 针参强转/copyWithin/flat/flatMap零分叉收钉；全局环境名册与 localeCompare 残档在十五 B)、内建面扫面第九轮 (12.33，门禁 66：prelude `Object.hasOwn` 委托自家 hasOwnProperty 替身同口径 + `String.raw` 手写对象形 (段数由 raw 名册定、空名册吐 ""、实参存在才插入、显式 undefined 才吐；两轮迭代探针收敛) + 静态面 census 余 9 项 (freeze/descriptor/groupBy) 与 Date 子系统记账在十五 B)、内建面扫面第十轮 (12.34，门禁 67：prelude `Object.groupBy` 补齐 (数组样 Get 语义/键 String 化/结果 `Object.create(null)`/`Array.prototype.group`+`groupToMap` node 实测不存在不实现) + 数组字面量空位 `[1,,3]`/尾逗号解析 (Undefined 填槽密 Vec 近似) + 空原型对象继承面三处真实缺陷修复 (引擎 `in` `stopped_at_null`、引擎读 `chain_ends_at_null` 单点、宿主 Object 读臂链顶 null 即止 —— `("toString" in c)`/`c.toString`/`c+""` 全对齐 node) + prelude 函数 `.length` 过桥合成面 (bridge Function 转换预置 params/name 进 props, 修 BC 全 undefined 链分裂)；`String(c)` 直转/洞位回调跳失/键桶序/Map族残档在十五 B)、内建面扫面第十一轮 (12.35，门禁 68：exec/test lastIndex 源真相收口 (桥 receiver 属性双写 `sync_last_index`, test 委托单核) + sticky `y` 锚定与推进 (`tracks_last_index`/`regex_exec_core` 单点，宿主两臂同口径) + BC 正则文本形三处收敛 (`regex_repr` 单点 / 自有 toString 遮蔽 / 边界身份还原重建 RegExp) + split 捕获组与空匹配规范 RegExpSplit (`regex_split_parts` 一处定义两链共用, 闭 12.23 残项)；正则对象枚举/原型面三向链分裂立为十二轮输入)、内建面扫面第十二轮 (12.36，门禁 69：标准正则枚举面三单点收敛 —— `RegExpData::own_property_names`=`[lastIndex]` 名册真相 + 宿主 GON RegExp 臂 (一处修两链, 级联 BC prelude hasOwnProperty/hasOwn) + 引擎 `is_regexp_pseudo_key` 名册让 ForInEnumerate 认哨兵过滤伪键 (修 BC `for-in(/a/g)` 泄漏 12 键链分裂) + AST 读臂 `RegExpHasOwnProperty/RegExpPropertyIsEnumerable` 回落消 TypeError 链分裂 + 引擎新常量 `PROPERTY_IS_ENUMERABLE_KEY`；正则 expando 属性面立为十三轮输入、正则原型实体面残档在十五 B)、内建面扫面第十三轮 (12.37，门禁 70：正则 expando/方法遮蔽面收口 —— 宿主 `RegExpData.props` 自有袋 + set 三路由/get props-first + 只读名册 `is_regexp_readonly_key` 引擎单点 (宿主副本删除, 零 wrapper) + 引擎 set/delete 哨兵护栏 + 桥两向搬运 (`@@shadow:` 墓碑区分代管形/用户遮蔽, 覆盖已有槽位才落碑防门禁 69 回退) + for-in 认墓碑让 exec 遮蔽可枚举 + `json_object_body` 单点让 JSON.stringify 正则 expando 交 `{"foo":1}` 且消 Object 臂 20 行重复 + AST delete 袋回收；`String()` 直转两轨/toString.call tag/遮蔽 delete 回落残档在十五 B)、内建面扫面第十四轮 (12.38，门禁 71: 解构族主躯干闭合 —— parser 侧暂态 `Pattern` + `desugar_stmts` 解析漏斗出口单点展开 (两链一份实现、执行器零感知: 解释器/桥/静态扫描只见普通节点) + 默认值 `===undefined` 触发与非标识符源快照 (getter 一次性) + 数组 rest=slice/对象 rest=assign+delete + 语句位 `[a,b]=c` 非破坏性配平扫描判型 + `parse_param_table` 单点消两处重复参数循环、解构参折 `$argN` 前言先于 `new_with_src` 拼接保 hoisted 统计；迭代器协议/入口名册残档见十四轮残档，BC defineProperty 失效定案 12.26 深拷贝同墙)、内建面扫面第十五轮 (12.39，门禁 72: 形参默认值与箭头解构形参 —— 简单形参默认折 prelude `var a = a===undefined ? fb : a` 复用十四轮 desugar 机件 (惰性/前序引用/arguments 入默认全对) + 箭头参数表并入 `parse_param_table` 单点 (`parse_arrow_paren_params` 死代码删除, `parse_arrow_tail` 收 prelude 先于 new_arrow) + 引擎 `frame_arguments_len` 进帧快照修形参回写撑大 `arguments.length` 的 BC 链分裂 (set_register 参数寄存器 max 增长为根因, LdaGlobal 只读快照)；TDZ 残档在十五轮残档，rest 形参已由第十六轮闭合)、内建面扫面第十六轮 (12.40，门禁 73: rest 形参 —— `SharedFunctionInfo.rest_param` 元数据单点 (绑定名+收集起点, 不占 params 位) 零新字节码, AST 链 `init_activation_scope` 在 hoist 前插数组快照 (sloppy `var z` 重decl 口径保住) / BC 链生成器专用寄存器 + 分发器 `call_function` 进帧写入 (箭头/construct/跨桥 shim 单点全覆盖)；`FunctionLiteral` 新字段必须 `Box` (最大变体加 32B 撑爆递归桥栈帧, earley 回归根因修复而非 RUST_MIN_STACK 掩盖)；`...r=x`/`f(...r,b)` 语法错两链收钉；rest-TDZ 前向读残档在十六轮残档)、内建面扫面第十七轮 (12.41/12.42，门禁 74: for-of 暂态 `Stmt::ForOf` parser 摊成 `$fsrc/$fidx` 三段式活读长度索引循环 —— node Array Iterator 索引语义对齐 (push 被访问/洞读 undefined/字符串与 arguments 按位读), 头部四形态 Binding/Pattern/Target + `scan_balanced_closer` 复用判型单点, 执行链零感知; 顺带修 BC 引擎 var 提升预绑槽 —— 循环头临时寄存器与体内晚绑 var 同号冲撞的链分裂根因, `hoist_declarations` 全作用域收名排序预绑, f14/g151/g152 三方对齐; for-of 挂账偏差与非可迭代残档在十七轮残档)、12.5/12.6/12.7 (发布矩阵、CI、在线大盘)。

**机器纪律** (踩过的坑，勿再犯): 绝对数只在同窗口内比较；负载 >6 时数字不记账，只采 profile 份额；A/B 必须交错跑并至少 min-of-5，配 A/A 对照；测量前确认二进制不是陈旧 target 缓存 (`target/prof2`)；采样需 `CARGO_PROFILE_RELEASE_DEBUG=1 CARGO_PROFILE_RELEASE_STRIP=false` 重编。

### E. 重启后接手须知 (本轮收尾状态)

- 分支 `main` @ `779de58` (提交名"init" —— 用户侧自动提交进程 21:00 前后把**全部历史重写为单提交**并同步 reflog, ③ TheHole 大部分编辑被吞入此快照; 与 357faa4/4676858 先例同构, 环境行为如实记录不追改, 更早快照不可达, origin/dev 是 10-14 老快照不可作回退基线)。当前未提交差异 = 快照后追加的根因修复与对齐 (`rs8/src/runtime/{interpreter,value,bytecode_bridge}.rs`、`v8-interpreter/src/bytecode_dispatcher.rs` +53/−23: `for_cross_engine_call` 收 `root: Arc<Environment>`、to_number 单元素臂 0.0 等) + `todo.md` 12.51/C/E 记账。红线"不主动 commit"仍有效
- `README.md` 与 `readme/{zh,en}/bench.md` 的分数表是 bench 脚本自动刷新的，非本轮人工改动，别当自己的成果复核
- 末次门禁: `./test.sh` **2977 passed / 1 skipped**、`syntax_corner_cases` **80/80** (79 表示面 + 80 行为面 both_chains 对 node 逐字符)、clippy **0 警告** —— 但注意 `sh/clippy.sh` 的 `cargo fix` 诊断服务器连续两轮挂死 (recvfrom 零 CPU), 本轮以等效只查命令取数, 下轮复跑同命令即可 (见 12.51 机器纪律)
- 下一入口: 战役 C **④ 删桥 + descriptor 接 Map** (① 收官 12.46/12.47; ② 收官 12.48–12.50; ③ 收官 12.51: TheHole 哨兵 + materialized 读面 + `StaInArrayLiteral` 唯一洞入口 + `for_cross_engine_call` 空 global 根因修复)。④ 收口清单: expando 袋身份合一、正则字面量 per-eval 新对象×编译缓存兼得 (12.50 观测行)、prelude `NativeFn::Ptr`↔`InterpNative` 重解释、prelude 跳洞法 hasOwnProperty O(1) 化 (12.51 性能债)、`RwLock<HashMap` 准入 18 处清零、深拷贝桥退役; 之后 ⑤ 全局环境单一化。看板余项 TDZ/块级作用域 11.6、tagged templates、Map/Set、Date; 战役排期 C→D→E→H 严格串行
- 复测配方见 `todo-archive.md` 12.18；工件在 repo `target/scratch/` (天然 gitignore): `re_probe.js`+`re_probe_expected.txt` (13 项正则探针与逐字节基线) + `rs8_c2b1f`/`rs8_c2b2f` (② 两刀终版) + **`rs8_c3f`** (③ 终版, 下轮 A/B 基线) + `ab/` 七靶点 A/B 组 (`add1/add5/str2/re1/re2/split1/hole1.js` + `ab.sh` ABBA 交错 min-of-5 + load<6 守门 + A/A 用法 `A=... B=... CHAIN=... zsh ab.sh`)
