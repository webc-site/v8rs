# todo.md 历轮细账归档

- 本文件是 `todo.md` 第十二节 **12.9–12.45 历轮细账的原样归档** (2026-10-17 移出，逐字未改)。轮次编号与主线交叉引用 (`见 12.xx`、`12.xx 记账`、`残档见 X 轮`) 一律以本文件为准
- 移出动机: 主线压缩释放上下文; 归档纪律 = 只搬不删不改，账目全量保真
- 12.46 起的现役轮次 (战役 C-1a/C-1b/C-2a) 仍留在 `todo.md`

---

### 12.9 CLI 主链路实测 (战役 A 收官后, 2026-10-15)

同机同口径 (macOS arm64, `--release` + `lto=fat`, `--bytecode-only`), js-engine-benchmark v8-v7 的 richards + deltablue + crypto + splay 四件套, 用户态时间:

| 引擎 | user 时间 | 倍数 |
| --- | --- | --- |
| node 26.3 | 22.5s | 1.0x |
| rs8 字节码引擎 | 94.4s | **4.2x 慢 (约慢 76%)** |

- 四件套全部自校验通过 (V7 自带 `verify` 断言), 差的是速度不是语义; deltablue 在修掉 `TestEqual` 对象身份缺陷前根本建不出计划链
- 原口径"比 V8 慢 99%"不成立: 那是跑错引擎 (AST 副本) 时的数字。当前差距的主因是 **1.3 第 7 条 —— 无 JIT**, 解释分派循环本身已不是瓶颈
- 微基准 (每轮交叉跑两个二进制, min-of-5 与 mean-of-30 各一遍) 本轮回归判定: micro_loop 0.41→0.40、micro_call 0.23→0.23、micro_prop 0.27→0.27、micro_arr 0.55→0.55、rich_probe 4.97→5.01 (+0.9%)
- **A/A 对照**: 对两个语义完全等价的构建 (`to_primitive` 拆薄壳前后) 同法测得 mean +0.85% / median 0.00% → 上面那条 +0.9% 落在同一噪声带, 判定为代码布局漂移, 不是语义开销; 今后 <1% 的单侧差异必须有 A/A 对照才算"回归"
- 已试并驳回的优化 (数字为 rich_probe 相对值): 操作数窗口预取 −5%~−12% (更慢), 标量 `Value::clone` 内联 −3%。死路记录在案, 不再重试
- 复现配方: `CARGO_TARGET_DIR=target/prof2 cargo build --release -p rs8`, 差分测试用宿主侧前置 `var print = console.log;` 与 node 逐项比对; A/B 必须交叉跑同一轮 (每轮各跑一次两个二进制) 并至少 min-of-5, 后台有 cargo/clippy 在跑时数字不可信

### 12.10 本轮复测 (2026-10-15, 删 IC + 强制内联之后)

先把配方钉死 (上面的 94.4s/22.5s 那组无法从仓库里复现出装载方式, 保留但不作为本轮口径): 四件套拼接成单文件, `cat base.js richards.js deltablue.js crypto.js splay.js <run.js 第 38 行起的 driver> > /tmp/oct4_all.js`, 三个引擎跑同一个 `/tmp/oct4_all.js`:

| 引擎 / 模式 | real | user | 相对 |
| --- | --- | --- | --- |
| node 26.3 (JIT) | 10.06s | 11.83s | 1.00x |
| node 26.3 `--jitless` | 11.25s | 13.09s | 1.12x |
| rs8 `--bytecode-only` (本轮初) | 26.42s | 26.32s | 2.35x (vs jitless 2.01x) |
| rs8 `--bytecode-only` (寄存器借用改造后) | 25.01s | 24.89s | 2.22x (**vs jitless 1.90x**), Score 320→361 |
| rs8 `--ast-only` | 28.11s | 28.02s | 2.19x (vs jitless 2.15x) |

注: 上面 `26.42/28.11` 两行是本轮**改造前**的读数, 当时字节码链在单文件 Richards 上还输给 AST 链; 该反号已由寄存器借用改造消除, 见 12.11

- 四条 suite 全部自校验通过 (Score 320 与逐名结果齐), 差的是速度
- **修正 12.9 的"4.2x"**: 那是旧装载口径。同文件同口径实测字节码链只比 `node --jitless` 慢 **1.95x** (user 26.32 vs 13.09), 比 AST 链快 6% (26.42 vs 28.11)
- **单文件反例 (必须解释)**: `/tmp/rich_long.js` (400 轮 runRichards) 上 `--ast-only` 7.54s 反而快于 `--bytecode-only` 8.20s。即 Richards 这一个 workload 里字节码链是净劣化。**已用微基准定位到构造层面** (min-of-5, real, 同一二进制):

| 微基准 | `--ast-only` | `--bytecode-only` | 字节码相对 |
| --- | --- | --- | --- |
| micro_prop | 0.14 | 0.23 | **+64%** |
| micro_arr | 0.33 | 0.49 | **+48%** |
| micro_globalread | 0.38 | 0.57 | **+50%** |
| micro_loop | 0.24 | 0.34 | **+42%** |
| micro_objcall | 0.14 | 0.19 | **+36%** |
| micro_call | 0.20 | 0.21 | +5% |
| micro_local | 0.25 | 0.23 | -8% |

结论 (比"慢 99%"这类笼统说法可操作得多): **字节码链目前只在纯局部变量代码上不亏, 属性 / 元素 / 全局读 / 循环四类构造一律比 AST 链慢 36%~64%**。所以战役 A "打通链路"只证明了语义等价, 没有证明性能等价 —— 默认 `BytecodeFirst` 在这些 workload 上是净劣化, 这必须先还, 再谈 JIT。先前猜的 "`Environment` 侧缓存 / 跨界桥接" 两个嫌疑已被 `sample` 驳回 (相关函数在 Richards 采样里连 5 样本门槛都没过)
- 微基准单文件口径 (同机, 3 次): 删坏 IC 前 9.49/9.53/9.53s → 后 8.19/8.20/8.21s (**-13.6%**), 远超 12.9 定的 1% 噪声带, 判定为真实收益
- 再之后一步 (脏全局名集合改借用查哈希去掉每次 `StaGlobal` 的 String malloc/free + `get_jump_target_offset` 内联), 干净无并发构建下 min-of-5 复测: micro_prop 0.23→0.22, micro_arr 0.49→0.45, micro_globalread 0.57→0.52, micro_loop 0.34→0.31, micro_objcall 0.19→0.17, micro_call 0.21→0.19, micro_local 0.23→0.22 (七项全部 4%~9%); 主基准 `/tmp/rich_long.js` 8.19→7.76/7.93/8.01 (约 -3.5%), AST 链 7.33/7.41 未动。字节码链仍比 AST 慢 7% (7.90 vs 7.37), 差距从 9% 收窄
- 同轮派后审查 (rust_review) 另修两处真 bug: `rs8/src/runtime/interpreter.rs` JSON.stringify 数值臂手写 `f64::to_string` + `as i64` (1e21 不切科学计数, `|x| >= 2^53` 饱和成 `i64::MAX`)、`rs8/src/runtime/parser.rs` 数字对象键同样手写 (`{1e21: v}` 的键与 `o[1e21]` 读点不是同一个键), 两处统一走 `double_to_string`; 回归用例 21。门禁复测: `./sh/clippy.sh` 0 警告, `./test.sh` 2909 全绿 (1 skipped)
- **下一个明确性能项 (以 `sample` 的 "Sort by top of stack" 自时间直方图为准, 不再靠猜)**: `/tmp/rs10.txt` (400 轮 Richards, 总样本约 5560) 排名 —— `run_from` 自身 2279 / `Value::drop_glue` 981 / `Value::clone` 854 / `HashMap<String,Value>::get` 430 / `get_property_depth` 303 / `memcmp` 216 / `call_function` 128 / `get_register_ref` 115 / `get_property_resolved` 98 / `check_property_receiver` 98 / `test_equal` 76 / `get_mutable` 67 / `get_jump_target_offset` 66 / `Vec<Value>::resize` 40
  - **clone + drop 合计 1835 (约 33%) 是头号目标**: 根因是分发循环大量臂用 `get_register()` (返回所有权克隆的 `Value`) 而非借用。要做的是把只读热臂改成 `get_register_ref()` 借用 (`&Value`) —— 读属性、比较、传参都只需要引用; 引用计数实现本身不要再动 (`Rc/RefCell` 已实测驳回)
  - 属性链合计约 18% (`HashMap::get` 430 + `get_property_depth` 303 + `memcmp` 216 + `get_property_resolved` 98 + `check_property_receiver` 98 + `get_mutable` 67 + `set_property` 29): 结构性解法是战役 C 的 Map/DescriptorArray 合一; 现阶段可做的只有把 `check_property_receiver` 的抛错分支 outline、以及键 intern 去掉 `memcmp`
  - profile 里仍是独立符号 (LLVM 没内联进分发循环) 的两个小热函数: `get_register_ref` 115、`get_jump_target_offset` 66 -> 直接补 `#[inline(always)]`
  - **已按实测驳回的假设**: 变量解析链 (`find_context` 的 contains_key+get 双哈希、`resolve_name` 的第三次同键查找、`assign_name` 的 `insert(name.to_string())`、`frame_context` 每次带捕获变量的调用新建 HashMap) 在 Richards profile 里连 5 样本门槛都没过 —— 函数局部量走寄存器, 只有全局与被捕获名才碰这条链。先前把它记为下一项是推测, 现已更正; 除非有 profile 证据 (闭包密集 workload), 不再以它为由改动


---

### 12.11 寄存器借用改造 (2026-10-15, 承接 12.10 的 clone/drop 33%)

按 12.10 直方图指的方向做的一件事: 分发臂里凡是**只读** receiver/操作数的, 一律从 `get_register()` (克隆整个 `Value`) 改成 `get_register_ref()` (借 `&Value`)。改动集中在 `v8-interpreter/src/bytecode_dispatcher.rs`: `GetNamedProperty` / `SetNamedProperty` / `GetKeyedProperty` / `SetKeyedProperty` / `DeleteProperty*` / `TestInstanceOf` / `TestIn` / `ForInEnumerate` / `ForInNext` / `ForInStep` / `JumpIfForInDone*`。`Ldar`/`Mov`/`Star`/调用臂的参数搬运本来就必须产出拥有权的值, 不动。

同轮另两处: 脏全局名集合 `assign_name` 改为先借用查一次哈希 (去掉每次 `StaGlobal` 白付的一次 String malloc/free)、`get_jump_target_offset` 补 `#[inline(always)]` (采样里它仍是独立符号)。

实测 (干净无并发构建, min-of-5 / 主基准 3 次):

| 项 | 改造前 | 改造后 |
| --- | --- | --- |
| /tmp/rich_long.js `--bytecode-only` | 8.19~8.21s | **6.91~6.93s** (再 -16%) |
| 四件套 `/tmp/oct4_all.js` | 26.42s | **25.01s** (Score 320→361) |
| micro_prop | 0.23 | 0.20~0.21 |
| micro_arr | 0.49 | 0.43 |
| micro_globalread | 0.57 | 0.51 |
| micro_loop | 0.34 | 0.30 |
| micro_objcall | 0.19 | 0.16 |
| micro_call / micro_local | 0.21 / 0.23 | 0.18 / 0.19 |

- **12.10 那个"ast/bytecode 反号"已消除**: 同文件 `/tmp/rich_long.js` AST 链 7.33~7.41s, 字节码链现在 6.91~6.93s, 比 AST 快约 6% (原先慢 9%)。这条曾经是"链路虽通但性能上是净劣化"的核心证据, 现已翻正
- 语义门禁: `/tmp/fin_probe.js` 对 node 逐项一致 (for-in 索引键、`instanceof`、`in`、`delete` 后 `in`); 键序随机是 12.10 已登记的战役 C 遗留, 两条链都有, 非本次引入
- 验收: `./sh/clippy.sh` 0 警告, `./test.sh` 2909 全绿 (1 skipped)

---

### 12.12 ES5 accessor + 数组方法表 + Date 补漏 (2026-10-15)

**做了什么** (全部为工作区未提交改动, 基线 = commit `19167c9`):

1. **对象字面量 accessor** (`{get p(){}, set p(x){}}`、方法简写、属性简写、尾逗号)。此前这两条链路都是**硬语法错误**。
   - 语法: `rs8/src/runtime/parser.rs` 新增 `peek_at`/`parse_function_tail`/`parse_object_key`/`parse_object_literal`; `rs8/src/runtime/ast.rs` 加 `PropKind{Data,Get,Set}` + `ObjectProp`, `Expr::Object` 载荷随之改成 `Vec<ObjectProp>` (原版靠值位置 `FunctionLiteral` 的 kind 区分, 移植侧函数值不带这信息, 所以种类必须提到属性节点)
   - 表示: accessor 以伪键 `@@get:<k>` / `@@set:<k>` 落对象表 + 每对象哨兵 `@@accessor`; 键约定 (`GETTER_PREFIX`/`SETTER_PREFIX`/`ACCESSOR_FLAG_KEY`/`format_*_key`/`enumerable_name(s)`/`is_internal_key`) **唯一真相在下层** `v8-interpreter/src/bytecode_dispatcher.rs`, rs8 侧只 import —— 顺手消灭了 `"@@get:"` 字面量在两个 crate 各抄一份和 `format_setter_key` 零调用者的问题
   - 触发: 字节码链 `GetNamedProperty`/`SetNamedProperty`/`GetKeyedProperty`/`SetKeyedProperty`/`has_property_resolved`/`ForInEnumerate` 走 `lookup_accessor`; AST 链走 `eval_get_property`/`eval_set_property`/`eval_get_element`/`eval_set_element` (更新表达式、复合赋值、成员/下标读、方法 callee 全部改经这几个 helper, 不再各点自己读表)
   - **热路径纪律**: 属性读是头号热点 (12.10 profile 约 18%)。没有 accessor 的对象现在只多付一次 `contains_key(ACCESSOR_FLAG_KEY)` (静态串, 零分配), 拼前缀键只在哨兵命中时发生; AST 链原来是每次命名读都 `keys().any(...)` 的 O(n) 扫描, 已删
   - **只做 own accessor**, 原型链上的 getter 故意不触发: 那需要 V8 Map 级 "可能有 accessor" 标志位, 逐层探测会把一次哈希变成 O(链长)。记入战役 C
2. **数组内建方法表合一**: `rs8/src/runtime/value.rs` 新增 `JsValue::array_builtin` (与字节码链 `Value::array_builtin` 同构的单一真相表), `get_property` 与 `BinaryOp::In` 共用; 补 `ArraySort`/`ArrayIncludes`/`ArrayConcat`。动机是实测缺口: `Object.keys(o).sort()` 在回退到 AST 的路径上直接 `TypeError: 'sort': undefined is not a function`
   - `sort` 比较器是用户函数, `NativeFn::call` 只有 `&self` 拿不到执行态 → 两链都"表里返回 ArraySort 标记 + 调用点就地拦截" (rs8 在 `invoke_function_named`, v8 在 `call_function`), `NativeFn::call` 里 ArraySort 返回 Err 作防御
   - 顺带在两链都修了规范偏差: `a.sort(null)` 必须 TypeError (原先把不可调用值当缺省静默走默认序)
3. **Date 实例补 `getTime`/`valueOf`** + 时间戳单一来源 `epoch_ms()` (coarsetime, 替掉两处重复的 `SystemTime::now().duration_since(UNIX_EPOCH)`)。这一条是**基准失真级 bug**: 实例只有 `__timestamp__`/`time` 数值槽, `d.getTime()` 恒 undefined, Octane 的 `BenchmarkSuite.GetTime` 抛错被 driver 吞掉, 每个 suite 只跑一批 —— **Score 照样打印**, 而 `/tmp/oct4_all.js` 的耗时从 31.5s 掉到 2.03s。任何"没报错但没有内建"的缺口都按这个量级警惕
4. 数字格式化家族 (JSON.stringify 数值臂、数字对象键) 统一到 `double_to_string`, `a[1e21]` 下标溢出 panic 修复 (`MAX_ELEM_INDEX`/`fast_elem_index`), 由审查轮落地, 本轮复测确认

**语义核对** (逐行对齐 node, 探针在 /tmp): `acc_gate.js` (14 项: 读/写/复合赋值 `t.n += 1`/getter-only 忽略赋值/setter-only 读 undefined/`{get:1}` 仍是数据键/`in`/for-in 不泄漏伪键/嵌套字面量/方法+属性简写) 与 `sortp.js`+`arr_gate.js` (17 项: 默认码点序、比较器、稳定序、undefined 排尾、比较器抛错传播、`sort` 就地返回接收者、includes 的 SameValueZero、concat 不改原数组、`"sort" in [1]`) 在 `--ast-only` 与 `--bytecode-only` 下**三条输出完全一致**。新门禁 23/24/25/26 (`rs8/tests/syntax_corner_cases.rs`), 其中 23/24/26 用新增的 `both_chains()` 同时跑 `run_ast` 与 `run_strict_bytecode` 并断言两链不分裂; 25 钉住计算键 accessor 的回退边界 (BytecodeFirst 必须回退成功, BytecodeOnly 必须硬失败而不给错值)。

**实测 A/B** (基线二进制 = `19167c9` 的独立 worktree 构建, 与本轮工作树同机交错, min-of-3, `--bytecode-only`, 当时 load ~5):

| 项 | HEAD 基线 | 本轮工作树 | 变化 |
| --- | --- | --- | --- |
| micro_prop | 0.28 | 0.24~0.25 | -12% |
| micro_arr | 0.56~0.57 | 0.47~0.48 | -16% |
| micro_globalread | 0.68 | 0.57~0.59 | -14% |
| micro_loop | 0.41 | 0.35 | -15% |
| micro_objcall | 0.22 | 0.19 | -14% |
| micro_call | 0.23 | 0.20 | -13% |
| micro_local | 0.28 | 0.23 | -18% |
| /tmp/rich_long.js (400 轮) | 10.26~10.28 | 9.22~9.31 | **-9.6%** |

- **这组数字不能读成"accessor 让属性读快了 12%"**: 7 个微基准**齐降** 12~18%, 其中 `micro_call`/`micro_local` 与本轮改的代码路径无关 (纯调用/纯局部变量)。差额里含链接期代码布局收益 (分发循环是单个巨型函数, I-cache 对齐一动全动)。要把属性读的单项收益单独拿出来, 必须做单改动 A/B 或 profile, 不能拿这张表记账
- 绝对值也不可与 12.10/12.11 比: 那两轮 `rich_long` 是 6.91~7.41s, 本轮同文件同二进制口径 9.2~10.3s, 机器负载不同 (当时接近空机)。**跨轮只比配对差值**
- `/tmp/oct4_all.js` 本轮重测 (Date 修好后才是有效口径): real 31.51/31.63/32.10s, Score 277/279/283。12.10/12.11 的 26.42/25.01s 无法在当前工作树复现 (同文件在 HEAD 基线二进制上只有 2.03s, 即 GetTime 退化口径), 保留记录但不作跨轮基线

**验收**: `./sh/clippy.sh` 0 警告; `./test.sh` **2914 全绿** (1 skipped); `cargo build --release -p rs8` 无警告。

**本轮暴露/新增的缺口** (全部进战役 C 清单, 见四·L3 与 十四·风险):

- **回退路径的方法面比字节码链窄**: 预置 JS (`interpreter.rs` 的 Array.prototype 段) 定义的 `map`/`forEach`/`filter`/`reduce`/`every`/`some` 在 AST 链上全量 undefined, 因为 `JsValue::new_array` 不接 `__proto__`, 数组方法只走硬编码表。实测 `typeof [1,2].map`: node/字节码链 `function`, AST 链 `undefined`。即"一次计算键 accessor 让整脚本回退"连带把 6 个数组方法换成 TypeError —— 这是功能悬崖, 不是缺 6 个方法那么简单
- `lastIndexOf`/`fill`/`unshift`/`find` 两条链都缺
- `Object.defineProperty` 的 `enumerable`/`writable`/`configurable` 全被忽略 (node: 描述符默认非枚举可关)。acc5 探针唯一残留差异: `dpKeys=` (node) vs `dpKeys=dp` (我们)
- ~~计算键 accessor (`get [k](){}`) 在 `--bytecode-only` 下硬失败: 桥接层的属性形态没有 "计算键 + `@@get:` 前缀" 组合~~ **12.15 已修** (`"@@" + k` 落成一条 Add 表达式, 覆盖度 0%→100%)
- 原型链上的 accessor 不触发 (own-only)
- 严格模式给 getter-only 属性赋值应 TypeError, 现在静默忽略
- `JSON.stringify` 无法触发 getter: 宿主 native 拿不到 `&mut self` 的执行态, 与 `sort` 同一类结构问题 (值模型/执行态不分家)
- Date 实例现在自带 `getTime`/`valueOf` 两个可枚举方法, `Object.keys(new Date())` 与 node 的 `[]` 不一致 (原型方法 vs 实例方法); 与"数组具名属性槽"同属值模型议题
- 字符串是 `char` 序列不是 UTF-16: `Object.keys("a😀b")` 得 3 键 (node 4), `length`/`charCodeAt` 同源自洽但非规范
- 对象键序随机 (HashMap), 两链都有; 测试里一律 `.sort()` 后再断言

### 12.13 审查轮复核 + delete 伪键成对回收 (2026-10-15)

**做了什么** (承接 12.12 的审查轮, 逐条读码 + 探针复核子代理报称的 8 项修复):

- **确认为真**: `is_internal_key` 改成 `@@` 双 @ 判定 (单 @ 会误杀用户的 `@foo` 普通键); `enumerable_names` 两趟收集 (普通键零比对, 只有伪键还原名参与去重, 消掉全表 `out.contains` 的 O(n²)); `JSON.stringify` 对象臂与 `for-in`/`Object.keys` 共用同一套内部键过滤 (`interpreter.rs:760`); `eval_get_element`/`eval_set_element` 把哨兵判定放在 `element_key` 之前 (`interpreter.rs:1125-1148`, 否则无 accessor 的对象每次下标读要多付一次 String 分配); `delete o[k]` 的键归一统一走 `element_key` (`value.rs:1205`, 原先数字走 `as i64`、符号走 Display, 与写入点不同源导致非整数键删不掉); `array_sort` 默认路线 `sort_by_cached_key`
- **子代理的 delete 修复本身是错的**: 字节码链写成 `[GETTER_PREFIX, SETTER_PREFIX].iter().any(|p| guard.remove(...).is_some())` —— `any` 短路, 只删命中的那一条, `@@set:q` 残留。探针 `/tmp/dlt.js` (get/set 成对, 删后再赋值): node 与 `--ast-only` 给 `after=7|1`, `--bytecode-only` 给 `after=undefined|7` (赋值仍打到旧 setter)。改成 block 表达式无条件 remove 两条 (`bytecode_dispatcher.rs:1326-1333`); 哨兵键 `@@accessor` 保留 —— 清它要全表扫剩余 `@@get:`/`@@set:`, 代价远大于热路径上多一次哈希探测
- **门禁 27 原本抓不到这个 bug**: 用例是 `{ get q(){ return 7; } }` 单侧 accessor, "删一条" 和 "删两条" 结果相同。补成对用例 `o3` (删后再 `o3.q=7`, 断言 `pairRead:7|v=1|keys=_v,q` 对齐 node), 并把这条教训写进用例注释
- AST 链 `array_sort` 默认路线也从 `sort_by_key` 换成 `sort_by_cached_key`: 标准库文档明确 `sort_by_key` 的取键函数**可能被同一元素多次调用**, 默认序比较每次都要现算 ToString 并分配两个 String; 与字节码链对齐

**实测复核** (min-of-3, `--bytecode-only`, 与 `19167c9` 基线二进制同机交错, 本轮机器负载高于 12.12 当时):

| 项 | 基线 | 当前工作树 | 配对差 |
| --- | --- | --- | --- |
| micro_prop | 0.29~0.31 | 0.25~0.27 | -13% |
| micro_arr | 0.78~1.13 | 0.63~0.73 | -19% |
| micro_globalread | 0.69~0.83 | 0.58~0.59 | -18% |
| micro_loop | 0.47~0.72 | 0.41~0.65 | 噪声大, 不入账 |
| micro_objcall | 0.34~0.44 | 0.26~0.33 | -24% |
| micro_call | 0.27~0.45 | 0.23~0.29 | -33% (含代码布局项) |
| micro_local | 0.58~0.66 | 0.44~0.59 | -20% |
| /tmp/rich_long.js (400 轮) | 10.41~10.45 | 9.44~9.71 | **-7.5%** |

- 结论只有一条: **审查轮的热路径改写没有造成回退** (`rich_long` 配对差 -7.5%, 与 12.12 记录的 -9.6% 同量级; 微基准齐降但降幅含 I-cache/代码布局项, 不能按项记账)。绝对值比 12.12 表格高一档 (0.24→0.25, 9.22→9.44) 是负载差异, 非回退 —— 两轮的基线二进制也同步右移 (10.26→10.41)
- 探针复测: `sortp.js` 17 项在 node / `--ast-only` / `--bytecode-only` 三方完全一致 (`diff` 空), `acc_gate.js`/`arr_gate.js` 两链一致

**验收**: `syntax_corner_cases` 27/27; `./sh/clippy.sh` 0 警告; `./test.sh` **2915 全绿** (1 skipped, 较 12.12 的 2914 多出的 1 个即门禁 27)。

### 12.14 数组迭代内建原生落地 + 内建接收者原型回落 (2026-10-15)

**做了什么** (承接 十四 的 #17/#22, 全部工作区未提交):

1. **AST 链接通内建接收者的原型面** (`interpreter.rs`): 新增 `builtin_ctor_name` / `proto_fallback`, `eval_get_property`/`eval_get_element` 在未命中时回落 `Array.prototype` (String/Number/Boolean 同一路径)。根因: `JsValue::Array` 是裸 `Vec` 存不下 `__proto__`, 硬编码表又只覆盖能纯 `&self` 实现的方法, 于是预置脚本里的 `map`/`forEach`/`filter`/`reduce`/`every`/`some` 在 AST 链上全量 undefined —— 一次回退把 6 个方法换成 TypeError。回落只在未命中路径发生, 命中路径零开销
2. **字节码链把这 6 个方法做成分发器原生**: `NativeFn::ArrayIter(ArrayIterKind, Arc<RwLock<ArrayData>>)` 单变体按 kind 分派 (6 个变体只会让 PartialEq/Err 臂各抄六遍), 与 `ArraySort` 同一套"表里放标记 + `call_function` 就地截获"模式, 因为回调是用户函数、`NativeFn::call` 只有 `&self`。落表后 `a.map`/`"map" in a`/`a["map"]` 全走原生
   - **顺带消掉一类真实崩溃**: 基线二进制 (`19167c9`) 上 `var s=0; a.forEach(function(x){ s += x; })` 直接 `ReferenceError: s is not defined` —— 预置 JS 的 `map` 跨引擎回调时 `interp_callable_to_js` 用 `closure: Arc::new(Environment::default())` 造函数对象, 闭包环境整个丢掉。原生版留在分发器内执行, 不再有这次跨界。**根因未修**, 只是这 6 个方法绕开了, 记入战役 C
3. **`in` 算符两处对齐 node**: `"map" in [1]` 此前 bc 也 false (数组臂直接 break, 不回溯 Array.prototype); `"x" in "abc"` 此前 bc 静默 false 而 node/AST 链抛 TypeError —— `Bytecode::TestIn` 加接收数类型校验, 原始值一律 TypeError
4. **预置脚本死循环**: 所有数组方法的循环条件写 `i < this.length`, 规范是 GetLength 一次; 回调里往同一个数组 push 会把 `map` 撑成无限循环 (实测挂死, node 返回 3 个元素)。改为进入即取 `var len = this.length`
5. **`reduce` 的显式 `undefined` 初值**: 预置脚本按 `acc === undefined` 判定"没传初值", 于是 `a.reduce(fn, undefined)` 被当成无初值 (node 让 undefined 参与回调得 `"undefined123"`)。改用 `arguments.length < 2`, 原生版按 `args.get(1)` 的 `Some`/`None` 区分, 两链同口径
6. `array_sort` 默认路线两链统一为 `sort_by_cached_key` (标准库文档明确 `sort_by_key` 的取键函数可能被同一元素多次调用), 由 12.13 落地、本轮复核

**语义核对** (探针 /tmp, node vs `--ast-only` vs `--bytecode-only` 三方 `diff` 全等): `iter_gate.js` 17 项 (回调三参数、filter 谓词、forEach 返回 undefined、reduce 无初值/带初值/显式 undefined、every/some 短路、空数组 reduce 抛、无回调抛、迭代中自增数组取长度一次、链式调用、`[[1,2],[3]].reduce(concat)`) ; `proto_in.js` 6 项 (`typeof a.map`/`"map" in a`/`a["map"]`/`Array.prototype.map`/`for-in` 计数不泄漏原型键); `gate28` 13 项 (含 `String.prototype.byProto` 用户原型扩展、`"x" in "abc"` 抛错)。门禁 28 `test_array_iterator_methods_and_proto_fallback` 用 `both_chains` 钉住, 死循环用例必须留在门禁里 (它是唯一能抓住"取长度一次"的断言)

**实测**:

| 项 | 基线 `19167c9` | 本轮 | 说明 |
| --- | --- | --- | --- |
| /tmp/micro_map.js (`forEach`+`map`+`filter`+`reduce`, 5000 元素 ×100 轮) | **直接 ReferenceError** | bc 0.18~0.19s / ast 0.54s | 基线与本轮不可比 (那边跑不通); bc 比 ast 快 2.8× 是"原生 vs 跨界预置 JS"的差 |
| /tmp/rich_long.js (400 轮) | 10.33~10.40s | 9.35~9.50s | **-9.3%**, 与 12.13 记录的 -7.5% 同量级, 本轮改动未回退热路径 |

**验收**: `syntax_corner_cases` 28/28; `./sh/clippy.sh` 0 警告; `./test.sh` **2916 全绿** (1 skipped)。

**本轮新增/暴露的缺口** (进战役 C):

- **内建表遮蔽用户对 `Array.prototype` 方法的覆盖**: `Array.prototype.map = function(){return "custom"}` 后 node/AST 链给 `custom`, bc 链给 `[0]` (原生表先命中)。修法要么"自有→原型→内建表"三段序 (需要给数组加 `get_own_property` 窄接口), 要么加一位 `proto_overridden` 标志在写入原型时置位、读侧只读一个 bool。基线上 12 个方法 (`push`/`join`/`slice`...) 本来就遮蔽, 本轮把面扩大到 6 个, 是拿罕见monkey-patch 换常见回调整 + 性能, 但**两链分裂**是新的
- `String.prototype.replace` 的函数替换形参两链都不支持 (被当字符串拼进去: node `a42b42`, 我们 `a[Function: anonymous]b...`), 且"函数→字符串"的表示两链不同 (`[Function: anonymous]` vs `function () { [bytecode] }`) —— 与 `sort` 同类的"回调形参要执行态"结构问题
- 数组具名属性: AST 链 `a.map = "own"` 存不下 (`JsValue::Array` 无属性槽), 读回落还给出原型函数; bc 链能存能删。属值模型议题
- `interp_callable_to_js` 的 `closure: Environment::default()` 根因 (跨界丢闭包环境) 未修

**复核口径** (12.15 全量重跑, 同一二进制): `syntax_corner_cases` 28/28; `./sh/clippy.sh` 0 警告; `./test.sh` **2916 全绿** (1 skipped); 本轮四项缺口仍在 (内建表遮蔽原型覆盖 / `replace` 回调 / 数组具名属性 / 跨界丢闭包环境), 未因 12.15 的 lowering 改动而变化。

### 12.15 计算键 accessor 的字节码 lowering (消掉最后一处整脚本回退) (2026-10-15)

**做了什么**:

1. **`rs8/src/runtime/bytecode_bridge.rs` `Expr::Object`**: 原先 `(ObjectKey::Computed(_), PropKind::Get | Set) => return None` —— 一个 `get [k](){}` 让**整脚本**回退到 AST 求值器 (桥接层最后一条 `return None`)。现改成: 伪键前缀接在"求值后的键"上, 而求值后的键本身就是计算键, 所以 `"@@get:" + k` 用一条 `AstBinaryOp::Add` 表达, 属性形态仍走 `is_computed`; 哨兵 `@@accessor` 与命名键共用同一 `has_accessor` 置位。rs8 侧新增 import `GETTER_PREFIX`/`SETTER_PREFIX` (键约定的唯一真相仍在下层)
2. **门禁 25 重写** (`test_computed_key_accessor_lowers_to_bytecode`): 原 25 钉的是"BytecodeOnly 必须硬失败", 现已无意义; 新版用 `both_chains` 同时跑两链, 覆盖计算键 getter/setter 成对 (`get [k]()` + `set [k](x)`)、**数字计算键** (`get [n]()` 与 `o[5]`/`o["5"]` 同键)、`Object.keys(c).sort()`, 并显式钉住唯一残留差异 —— `JSON.stringify` 不触发 getter (node `{"ck":7}`, 两链均 `{}`), 与 12.12 记的"宿主 native 拿不到执行态"同源

**覆盖度实测** (正确性口径, 非性能; 198 个 /tmp 探针, `--bytecode-only --print-bytecode-coverage`):

- **clean=179 / fallback=0**, 全部为 `bytecode-coverage: 100.0% (total=1 bytecode=1 ast_fallback=0)`
- 余 19 个无覆盖行: 逐个查过, **全部以运行时错误终止** (故意的报错探针 `radix.js`/`gate28.js`/`fin.js`, 缺宿主符号 `db_probe*.js`/`oct4_driver.js` 的 `BenchmarkSuite`, `*_n.js` 未把 `print` 换成 `console.log`) —— 覆盖行只在成功路径打印, 是统计脚本的口径, 不是引擎的回退。上一版 sweep 的 `case` 谓词写错 (`*"ast_fallback="*[!0]*` 匹配了每一行) 得出 "clean=0 fallbacks=127", 本轮改正后为 clean=179 fallback=0
- 桥接层 lowering 侧除注释外**零** `return None` —— 回退面收敛到 0, 战役 C 的"整程序回退粒度"议题 (#21) 由此失去主要动因: 没有需要回退的构造了, 按函数/语句粒度暂无必要

**语义核对** (`/tmp/cka.js` 计算键成对 accessor、`/tmp/cka2.js` 数字计算键): node / `--ast-only` / `--bytecode-only` 三方 `7|set:8|ck` 全等; 门禁 25 断言串 `7|set:8|ck#num=num|num#json={}`。

**性能实测** (同机交错, min-of-3, `--bytecode-only`; 本轮机器负载偏高 load 8.5, 绝对值偏慢, 只读配对差):

| 项 | 基线 `19167c9` | 本轮 | 结论 |
| --- | --- | --- | --- |
| /tmp/micro_prop | 0.28 | 0.23~0.24 | 无回退 |
| /tmp/micro_arr | 0.56 | 0.46~0.47 | 无回退 |
| /tmp/micro_globalread | 0.67~0.71 | 0.54~0.55 | 无回退 |
| /tmp/micro_loop | 0.41 | 0.33~0.34 | 无回退 |
| /tmp/micro_objcall | 0.22 | 0.19 | 无回退 |
| /tmp/micro_call | 0.23~0.29 | 0.20 | 无回退 |
| /tmp/micro_local | 0.28 | 0.23 | 无回退 |
| /tmp/rich_long.js (400 轮) | 10.37~11.28s | 9.39~9.65s | **-9%**, 与 12.14 的 -9.3% 同量级 |

**新发现 (未修, 记账)**: 在"对象字面量 + 调用 + 属性读写"这个形状上**字节码链比 AST 链慢**, 且与 accessor 无关 —— `/tmp/cka_data.js` (纯数据键 `{ck:7}`, 20 万轮) ast 0.10 / bc 0.14; `/tmp/cka_hot.js` (计算键 accessor, 同循环) ast 0.25 / bc 0.29。即 12.15 的 lowering 消掉回退是**一致性/覆盖率收益, 不是性能收益**: 这类脚本原先整程序回退 (≈ast 的 0.25), 现在走 bc 反而 0.29。accessor 命中路径本身已无顺序浪费 (四条属性臂都先查 `lookup_accessor` 再做昂贵的原型回溯), 多付的是每次读写的 `format!` 前缀键分配 + 一次 flag 哈希。两条待挖: (a) 前缀键零分配 (复用缓冲); (b) **对象字面量创建在 bc 链比 AST 慢**, 这才是 Richards/octane 里普遍的形状, 优先查它

**验收**: `syntax_corner_cases` 28/28; `./sh/clippy.sh` 0 警告; `./test.sh` **2916 全绿** (1 skipped)。

### 12.16 数组迭代取元素时机对齐规范 + 审查轮纪律 (2026-10-15)

**新增长期规则 (用户要求, 已落 `AGENTS.md` 与 7.2)**: 每次语义改动必须先在 `rs8/tests/` 加钉住用例再动实现 —— 同一脚本 AST 链 / 字节码链逐字符一致 (`both_chains`) 且与 node 实测对照; 修 bug 同理, 先让用例复现缺陷。无对应用例的改动视为未完成

**做了什么**:

1. **字节码链去掉整表快照** (`bytecode_dispatcher.rs` `array_iter`): 原 `let snapshot: Vec<Value> = read_elems(arr).clone()` 让回调里 `pop` 同一个数组后仍交出旧元素 —— `b.map(function(x,i){ if(i===0) b.pop(); return x })` 我们给 `1,2,3`, node 给 `1,2,<hole>`。现按规范: **长度取一次 + 每步脱锁现取一个元素**, 取元素与回调之间不持锁 (原注释担心的自堵不变)。脱锁的注释随之改写, 因为"快照"这个手段已不在
2. **AST 链预置脚本先定值再回调** (`interpreter.rs` 预置 Array.prototype 段): `fn(this[i], i, this)` 后 `res.push(this[i])` 是两次 GetElement, 回调里改写同下标会反映到结果 —— `e.filter(function(x){ e[0]=99; return true })` 我们给 `99,2`, node 给 `1,2`。map/forEach/filter/every/some/reduce 全部改成 `var el = this[i];` 再传 `el` (规范每步只 GetElement 一次, 该值同时作回调实参与收集值)
3. **门禁 29** `test_array_iterator_element_read_timing` (`syntax_corner_cases.rs`): 12 项断言 (非可调用回调 `map(null)`/`reduce(7)` 抛、pop 后元素现取、`every`/`some` 短路计数、`forEach` 恒 undefined、空数组带初值 `reduce` 不调回调、push 不扩结果集、filter 取回调前的值)。node / `--ast-only` / `--bytecode-only` 三方逐字符全等, 覆盖度 `100.0% (total=1 bytecode=1 ast_fallback=0)`
   - **这两处为什么必须成对钉**: 本轮改动前, 一条链只在 (1) 上错、另一条只在 (2) 上错 —— 单看任一链的探针都对, `both_chains` 若不覆盖"回调里改同一个数组"就全绿。12.14 记的"取长度一次"防的是循环次数, 没防元素来源

**审查轮纪律 (记入流程)**: 本轮派出的 rust_review 子代理报告"已修 2 个真实缺陷 + 4 项优化, 2918 全绿", 逐项核实后: **一行代码都没落地** (无 `rs8/tests/gate29.rs`, `git diff` 与派发前完全一致); 其 3 条缺陷断言里 (a) "GetKeyedProperty/SetKeyedProperty 不走 `lookup_accessor`" 经 grep 为**假** (两臂分别在 `bytecode_dispatcher.rs:3027`/`:3060` 调用), (b) "`GETTER_PREFIX`/`SETTER_PREFIX` 零调用者" 经 grep 为**假** (bridge/interpreter/lib 共 15 处), 只有 (c) "常量池键非 `String` 变体时走 `ConstantPoolOutOfBounds` 而非类型化回落" 是**真**且属既有行为 (本轮 lowering 让 `@@get:` 前缀键必然以 String 常量入池, 该分支只影响非串常量)。规范分歧 (1)(2) 是**我自己**跑 `/tmp/iter_edge.js` 三方对照抓到的。**结论: 子代理报告只能当线索, 不能当交付**; 落地与实测必须回主线 (与 7.3 "oracle 来自 C++, 不来自 LLM" 同源)

**实测** (同机交错, min-of-3, 与 12.15 同窗口对照):

| 项 | 基线 `19167c9` | 12.15 | 本轮 | 说明 |
| --- | --- | --- | --- | --- |
| /tmp/micro_map.js | ReferenceError | bc 0.18~0.19 | bc **0.17** | 去掉整表 clone 的量级收益, 单次读数不可当结论 |
| /tmp/rich_long.js | 10.29~10.37 | 9.39~9.65 | 9.38~9.73 (`done 400`) | 无回退; `done 400` 即 Richards 的 queueCount/holdCount 不变量成立 |
| micro_prop/arr/globalread/loop/objcall/call/local | 0.28/0.56~0.57/0.67~0.69/0.41/0.22/0.23/0.28 | 同 | 0.23~0.24 / 0.46~0.47 / 0.54~0.55 / 0.34~0.35 / 0.19~0.20 / 0.20 / 0.23~0.25 | 与 12.15 逐项重合, 本轮改动不碰这些路径 |

**验收**: `syntax_corner_cases` **29/29**; `./sh/clippy.sh` 0 警告; `./test.sh` **2917 全绿** (1 skipped)。

**遗留** (进战役 C, 已开任务): 常量池非 `String` 键的类型化回落 (子代理 (c) 项, 既有行为, 未改); 任务 #25 "bc 链对象字面量创建比 AST 慢" (cka_data ast 0.10 / bc 0.14); #18/#19/#23/#24 不变。

---

### 12.17 字面量走 CreateDataProperty + 表容量预留 + 累加器作废 (#27 收口, #25 定位) (2026-10-15)

**做了什么**:

1. **#27 字面量落表对齐原版** (任务关闭): 原版区分"普通赋值 = Put"与"字面量构建 = CreateDataProperty" (`v8/src/interpreter/bytecodes.h:190-212`), 本端口此前一律 `SetNamed/SetKeyedProperty`, 于是 `{ get y(){return 1}, y:2 }` 两链都读回 1 而 node 是 2 —— **两条链错得一模一样, `both_chains` 抓不到**, 只能与 node 逐字符对照。落地: `bytecode_dispatcher.rs` 新增 `define_own_entry` (数据键摘同名伪键对 / 伪键摘同名数据键) + `DefineNamedOwnPropertyInLiteral`/`DefineKeyedOwnProperty` 两臂, `bytecode_array_builder.rs` 两个对应发射方法, `visit_object_literal` 改道, 桥接层把 `@@accessor` 哨兵前置到属性最前并静态预扫 `has_accessor`, AST 链 `Expr::Object` 落同一套规则。门禁 30 三方全等
2. **数组的规范下标不回溯原型** (有意偏差, 已钉): `value.rs` 抽出 `is_canonical_index` 作单一来源 (读点/写点/回落筛除共用), `eval_get_element` 改为 `builtin_ctor_name` + `proto_prop` 且筛掉数组数值下标。实测 `/tmp/canon_idx.js`: node `empty0=p0#oob2=p2`, 两链 `empty0=undefined#oob2=undefined`; own 元素恒压过原型 (`[9][0]`=9), 字符串键照旧回落 (`tag`=pTag) —— 稠密 Vec 下每个 `a[i]` 都先问一遍原型是热路径上的白付, 而 `Array.prototype[n] = x` 是极冷门写法。门禁 28 追加 `hole/own0/filled0/tag` 四项，注释里写明这是**决策不是疏漏**
3. **#25 定位**: `sample` 采样 386 个样本 → 字面量落表臂占 121 (31%)，其中 `reserve_rehash` 占 44 (**全链 11.4%**)。根因是非空字面量也走 `CreateEmptyObjectLiteral`，表从 0 容量起逐属性扩容；而 AST 链 `reserve(entries.len()+2)` 一次到位。改法: `visit_object_literal` 改发原版 `CreateObjectLiteral`，**Flag8 携带属性数** (原版该位置是属性名数组的常量池下标 + Map 的 in-object 槽位带容量；本端口无 FixedArray 常量，属性名仍逐个走 Define* 臂，只把容量补上)，执行期 `Value::new_object_with(count + OBJECT_LITERAL_SLACK)`。复采 235 个样本里 `reserve_rehash` **归零**
4. **同两臂按原版 `kReadAndClobberAccumulator` 交出累加器**: `accumulator.clone()` → `std::mem::replace(&mut self.accumulator, Value::Undefined)`，每属性省一次引用计数 (取放先于寄存器借用，否则与 `get_register_ref` 的不可变借用打架)。交接链正确性由**门禁 31** 钉 (嵌套字面量、字面量在数组/函数实参位、计算键、accessor+数据键混写) —— 这类"内层落完属性后累加器已作废，必须 `Ldar` 取回对象再交给外层"的形状，平铺字面量的门禁全抓不到，症状是嵌套形状静默变 `undefined`

**计时纪律 (本轮踩到的坑)**: 同窗口 min-of-5 读到 p2 91.3 → 81.7 → 80.5ms，但机器负载从 ~8.5 涨到 **49** (并发 rustc)，与基线 `19167c9` 的 A/B 交错读数已失真 (micro_prop 读到 base 677 / new 878ms，与 12.16 记录的 0.23s 不在一个量级)。**绝对收益本轮不记账，待低负载窗口复测**；采用的证据是与负载无关的 profile 份额。另: `target/prof2` 走 release profile (`strip=true`)，采样必须先用 `CARGO_PROFILE_RELEASE_DEBUG=1 CARGO_PROFILE_RELEASE_STRIP=false` 重编，否则全是 `???`

**顺带确认 (已登记项，本轮未动)**: 对象键枚举序既不是插入序、也不跨进程确定 (`/tmp/keyorder.js` 同一脚本两次 `for-in` 给出不同序) —— 即 180 行表的"属性枚举顺序随机"，归战役 C (Map + DescriptorArray 自带序)。含义: 门禁里凡按原样断言 `Object.keys(...)` 序的都是脆弱断言，必须 `.sort()` 后比

**结构结论 (进战役 C/D，不再做局部补丁)**: bc 链剩下的差距不在字面量，而在**每条字节码 ~11-14ns 的分发 + 引用计数**。探针 `/tmp/add1.js` / `/tmp/add5.js` (200 万轮，min-of-5): bc 330.6 → 508.5ms，即多一个 `Add`+`LdaConstant` 对多 22ns；ast 201.1 → 284.7ms，即每表达式节点 10.5ns。单位成本两链接近，bc 只是"可执行单元数"多 (寄存器搬移、属性存取都各自成指令)。要压它得动分发环与连续寄存器栈帧 (战役 D) 和值模型合一 (战役 C)，与本节 3、4 两条的口径一致

**审查轮 (rust_review 子代理，只读模式) 的三条真发现与落地**: 本轮起子代理一律**只读**（历史事故：报告一行未落地且 3 条断言里 2 条为假），结论全部自行复核后再改。复核为真的三条:

5. **`in` 与元素读同口径筛除规范下标** (引擎自相矛盾，必须修): 上文第 2 条只筛了读点，`(7 in a)` 仍回溯 `Array.prototype` 判真，于是"真但读回 `undefined`"。落地: `bytecode_dispatcher.rs` 导出 `canonical_index` (与原 `is_canonical_index` 同一套谓词，作用域为字符串键)，`has_property_resolved` 数组臂与 AST 链 `BinaryOp::In` 数组臂都改为 `canonical_index(&key)` 命中即 `idx < len` 直接返回，不再问原型。门禁 28 追加 `in7/in0/in10/inTag` 四项断言。实测: node `in7=true hole=proto7`，两链 `in7=false hole=undefined` —— 与 v8 的规范下标语义有意分叉已在注释里写明
6. **`set_element` 未共用单一来源谓词**: 手写 `*n >= 0.0 && fract==0.0 && !is_infinite && <= u32::MAX-1` 与 `is_canonical_index` 文档里"读点/写点/筛除共用"的承诺不符，改调 `is_canonical_index(*n)`
7. **`OBJECT_LITERAL_SLACK` 死代码**: 注释称"留缓冲"实现却按 `count + 2` 扩容，与第 3 条"精确铺容量"矛盾，删除常量、`CreateObjectLiteral` 臂直接用 Flag8 的 `cap`，`cap==0` 走 `new_object()`

另: 计算键 `__proto__` 的两链一致偏差 (`{["__proto__"]: {injected: 42}}` → node 当普通数据键 `injected=undefined`，两链当原型链接 `injected=42`) 确认**不是本轮回归**，根因是值模型无法区分"原型链接"与"名为 `__proto__` 的自有数据键"，归战役 C，已开任务记录

**死代码核查 (`./sh/unused.js`，本轮补做，替换上一版"仍未做"的说法)**: `./sh/unused.js rs8` → unused-pub **0 命中**; `cargo shear` **34 errors / 47 warnings**，其中 `v8-zone` 报 5 个未链接文件 (`src/containers/extra_generated.rs`、`src/zone/{accounting,segment,type_stats,zone}_generated.rs`、`src/zone/../containers/extra_generated.rs`) —— 即 D1 (34 处 shear 命中) 的账，链路内 crate (`v8-interpreter`) 尚未跑。

**验收**: `syntax_corner_cases` **31/31**; `./sh/clippy.sh` 0 警告; `./test.sh` **2919 passed** (1 skipped)。三方 (node / `--ast-only` / `--bytecode-only`) 逐字符对照: `/tmp/lit_def.js`、`/tmp/gate28c.js`、`/tmp/gate31.js`、`/tmp/canon_idx.js` (最后一项按上文口径为有意偏差)

**遗留**: #25 的剩余部分 (分发环 + 值模型) 并入战役 C/D; `__proto__` 计算键偏差开战役 C 任务; `./sh/unused.js` 只跑了 rs8，`v8-interpreter` 等链路内 crate 待跑，shear 的 34 处命中即 D1; #18/#19/#23/#24/#26 不变。

### 12.18 字面量收益归因复测 (#29 收口, 低负载窗口 2026-10-15)

**方法** (可复现，今后同类复测照此): 基线不再用陈旧快照目录，而是 `git worktree add --detach /tmp/v8rs-head HEAD` + `CARGO_TARGET_DIR=/tmp/head-target cargo build --release -p rs8`，在 worktree 里**只替换待归因的那一段代码**再重编，得到"同源码、只差一项"的对照二进制；交错 min-of-7 (`/tmp/ab3.py`，每轮两个二进制各跑一次取 min)，并同法跑 A/A 定噪声带。

| 对照 | p2 | p3 | lit_hot | micro_loop (对照) |
| --- | --- | --- | --- | --- |
| 整轮字面量改造 (Define\* + 容量 + 累加器作废) vs lowering 回退版 | **−17.4%** | **−13.7%** | **−10.2%** | ±0.0% |
| 仅表容量预留 (Flag8 铺 `with_capacity`) vs 忽略容量版 | −7.1% | −9.1% | −5.9% | — |
| 本轮未提交项 (`canonical_index` + `in` 筛除 + 删 `OBJECT_LITERAL_SLACK`) vs HEAD | — | — | +0.2% | +2.1% (A/A 同二进制 −0.3%~−1.1%) |

- 容量项约占整轮收益的一半，另一半来自"直落数据键免 setter 查找"+"两臂交出累加器省一次引用计数"
- 未提交项判定为**语义修正 + 性能中性**，落在 A/A 噪声带内，不是回归
- **端到端**: `oct4_all.js` (四件套单文件拼接) 同窗口 32.65s → 32.58s，**−0.19%** —— 字面量不是四件套的主矛盾，12.17 的结构结论 (每字节码 ~11-14ns 分发 + 引用计数) 仍是唯一大头，归战役 D/C
- **更正 12.17 的旧基线口径**: `/tmp/v8rs-base` 那份快照与 HEAD 在 `interpreter.rs`(513 行) / `value.rs`(227 行) / dispatcher(960 行) / builder / generator 上都不同，是**跨多轮**的差值。所以先前记的 p2 −26.9%、micro_loop −12.5% 不能记到字面量头上 (loop 里没有字面量却"提速 12.5%"即是污染信号)，本节三档才是本轮账
- 复测完已 `git worktree remove --force /tmp/v8rs-head`，主工作树未动

### 12.19 数组内建 #19 补齐 + 键常量一处定义 + D1 依赖卫生 (2026-10-15)

**语义 (#19 收口)**: 两链共缺 `lastIndexOf` / `fill` / `unshift` / `find`，本轮同口径补齐:
- 字节码链: `NativeFn` 新增 `ArrayLastIndexOf/ArrayFill/ArrayUnshift` 臂 (纯内建可在 `NativeFn::call` 就地完成)，`find` 的回调是用户函数、`call` 只有 `&self` 拿不到执行态，故沿用"表里放标记 + 分发态就地执行"路线 —— `ArrayIterKind::Find` 进 `array_iter`；`array_builtin` 表登记 4 项
- AST 链: 同样 3 个内联函数 + `invoke_function_named` 截获 `NativeFn::ArrayFind`
- 顺带修规范缺口: `map/filter/forEach/every/some/find` 此前回调接收者固定 `Undefined`，现取 `args[1]` 作 `thisArg` (`reduce` 按规范保持 `Undefined` 不变)
- 语义钉入门禁 32 `test_array_lastindexof_fill_unshift_find`，期望串与 node 逐字符对照 (覆盖: 负 `fromIndex` 按尾部折算、`fill` 的 ToIntegerOrInfinity 截断、`thisArg` 命中 `{want:3}`、`find(null)`/`find()` 的 TypeError)

**一处定义 (重复代码收敛)**: `rs8/src/runtime/value.rs` 里 43 个属性/方法键常量 (`PUSH_KEY`…`SYM_TO_PRIMITIVE`) 与 v8-interpreter 侧同名常量双份并存，此前 12.12 已把 accessor 伪键约定收到 v8-interpreter，本轮把剩余键常量一并收口: 真相源 = `bytecode_dispatcher.rs` 常量块 (doc 标注"内建方法名的单一真相")，rs8 侧整段删除并改 `pub use v8_interpreter::{...}` 再导出，`crate::runtime::value::X` 旧路径不破；合并前逐对核对 25 组重复值**无漂移**。`v8-interpreter/src/lib.rs` 根再导出同步扩容 (curated 列表，非 glob)

**D1 依赖卫生**: `cargo shear --fix` 清账 25 个 `Cargo.toml`、共 34 项未用/错位依赖 (log / humansize / libc / unicode-* / criterion / rusty_alloc-api 的 dev/build 归位)，errors **34 → 0**，12.1 的"rusty_alloc-api 未用"矛盾同批消解。47 条"未链接文件"警告全部是 `tool/` 脚本再生成的 `*_generated.rs` / `*_tq.rs` (v8-zone 5 个、v8-builtins-* 41/90 个) —— 删了也会被转录脚本重生成，且它们是战役 F/G/H 的待挂接转录面，**记豁免、不删** (挂接动作归各战役准入)

**D2 分诊 (收口)**: `./sh/unused.js .` 全量重跑共 **139 处**未用 pub (旧记 87 为口径差，以本次为准)。**链内 6 处全部删除**: `bytecode_array_builder::move_register` (全仓无调用方；`Bytecode::Mov` 分发臂保留，属原版字节码表同构面)、`lib.rs` 两个零调用桥接占位 `ast_raw_string_to_type_hint`/`is_bytecode_supported_ast_node` 及其 `pub use v8_ast::{AstNodeType, AstRawString}` 根再导出 (`v8-ast` 依赖本体仍被 generator/builder 真实使用，不动)、`v8-execution-isolate` 三个桥接占位 (`is_valid_interpreter_bytecode`/`heap_core_main_cage_pages`/`is_below_gc_activation_threshold`) 并连带 `cargo remove` 只为占位存在的 `v8-heap-core`/`v8-heap-gc`/`v8-interpreter` 依赖 —— 该 crate 与主链的真实关系 (`d8.rs` 用 `Isolate`) 不受影响；另撤 `pub use bytecode_traits::*` glob。**链外 133 处不删、挂战役号**: H 121 (turboshaft 56 含 `opmasks_named` 52 个 `k_*` 同构掩码变体 / maglev-frontend 32 / compiler-frontend 14 / codegen-arm64 8 等)、F 5 (heap-core 2 / heap-gc 1 / objects-maps 1 / objects-js-async 1)、G 5 (debug 3 / snapshot 1 / torque 1)、B 1 (`v8-parsing::scan_from_stream`)、C 1 (`v8-strings::set_has_non_smi_elements`) —— 理由: 这些是各战役开工必然要真实接线的转录面 API，现在删了届时要逐字重写，纯负工；分诊明细登记在 2.2 D2 行

**门禁**: `syntax_corner_cases` **32/32**、`./sh/clippy.sh` **0 警告**、`./test.sh` **2920 passed / 1 skipped** (链内 6 处删除 + `v8-execution-isolate` 依赖清账之后重跑，数量不变 —— 被删者均无测试挂靠)

### 12.20 常量池名字槽位的类型化错误 (#26 收口, 2026-10-15)

12.16 审查轮确认的唯一真缺陷: `LdaGlobal`/`StaGlobal`/`Get|SetNamedProperty`/`DefineNamedOwnPropertyInLiteral` 五类字节码按池索引取**名字**，此前"索引有效但变体非 String"与"索引越界"混报 `ConstantPoolOutOfBounds` —— 误导排查方向 (会去看池容量而不是发错常量的生成侧)。原版无此分支: bytecode verifier 编译期就保证名字槽位必为 string。

- 收敛为单一助手 `BytecodeDispatcher::name_constant`: 三分支 —— 越界→`ConstantPoolOutOfBounds`、变体不符→新错误 `ConstantPoolTypeMismatch { idx, kind }` (`kind` 由 `ConstantPoolEntry::kind_name` 给出)、命中→`&str` 借用 (不新增分配，`StaGlobal` 热路径原来就是一次借用，改造后保持)
- JS 源码不可达该分支 (生成侧只会把 intern 后的标识符放进名字槽)，故用例落在 v8-interpreter 单测层: `test_name_slot_rejects_non_string_constant` 手工构造 `LdaGlobal [Number常量, 0]` 断言 `TypeMismatch{idx,"Number"}`，再用真越界 99 断言仍报 `OutOfBounds` (两类缺陷不得混账)
- `LdaConstant` 臂不动: 它本就消费全部变体，越界语义正确

**门禁**: `v8-interpreter` **33/33** (新单测入列)、`syntax_corner_cases` **32/32**、`./sh/clippy.sh` **0 警告**、`./test.sh` **2921 passed / 1 skipped** (+1 即本条单测)

### 12.21 `Array.prototype` 覆盖遮蔽内建替身 (#24 收口, 2026-10-15)

node 语义取证 (`/tmp/p23` 前置探针 `/tmp/p24.js`): 覆盖 `Array.prototype.push` 后实例方法调用必须走用户函数; `delete Array.prototype.map` 是**永久移除** (own configurable data property) —— 读取得 `undefined`、`"map" in a` 为 false, 不得回落到内建替身。此前两链都把内建替身当不可遮蔽的快路径, 且两链行为还不一致 (bc 链 `in` 走替身、AST 链方法调用走内联表)。

方案: **写侧监视 + 读侧让位**, 空集零开销。

- 单一真相: 内建名清单由替身表自身推导 —— `BytecodeDispatcher::is_array_builtin_name` 用 `OnceLock` 空 `ArrayData` 探 `Value::array_builtin` (先长度位图粗筛), AST 链直接 `pub` 复用, 不再有第二份名字表
- bc 链: `array_proto_watch: HashSet<String>` 记账 (StaGlobal 之外的 `Set/SetKeyedNamedProperty`、`DeleteProperty` 五臂写侧探测接收者是否 `Array.prototype` 本体, `Arc::ptr_eq` 判定); `get_property_resolved` 在"替身命中 Native 且名字被监视"时让位原型回落, 原型缺失给 Undefined; `has_property_resolved` 替身命中同样过监视筛
- AST 链: 同口径落 `eval_get_property`/`eval_get_element`/`in` 三读点 + `eval_set_property`/`eval_set_element`/`Delete` 写点。**关键补漏**: `try_inline_array_call` 是替身表的第二份拷贝 (push/pop/… 就地内联), 方法调用根本不经过取属性路径 —— 收口时在入口加同款监视短路, 一处判定同时覆盖 Member/Index 两个调用点。此漏是"一处定义"原则的反面教材: 表拷贝了三份 (host 替身 / 内联 match / 名字清单), 下轮战役 C 应连同内联表一并收敛到替身单源
- 用例: 门禁 33 `test_array_proto_override_wins_over_builtin_table` (both_chains, node 逐字符钉住)。探针写法注意: 覆盖 `Array.prototype.push` 后**探针自身不得再用 `out.push(...)` 收集** (递归爆栈), 换 `out[n]=` 索引写

**门禁**: `syntax_corner_cases` **33/33**、`v8-interpreter` **33/33**、`./sh/clippy.sh` **0 警告**、`./test.sh` **2922 passed / 1 skipped** (+1 即门禁 33)

### 12.22 `String.replace`/`match`/`exec` 回调与 `$` 序列 (#23 主体收口, 2026-10-15)

三方取证 (`/tmp/p23b.js`, 14 项): 回调替换实参此前被 `to_string` 化成 `[Function: anonymous]`; `$&`/`` $` ``/`$'` 两链都不展开 (AST 链走 fancy `replace_all` 只认 `$1/$name` 且**缺席组号会丢字面量** —— r7 node `a[$1]b` vs 旧 AST `a[]b`); `exec`/非全局 `match` 结果缺 `index`/`input` (宿主数组没有具名属性槽, 挂了也静默丢); 字符串模式 `match("b")` 两链给 null, node 给命中数组。

- **一处定义**: `$` 序列展开收敛为引擎侧唯一纯函数 `v8_interpreter::get_substitution` (规范序: 两位数优先、缺席组号原样保留、存在未参与给空串), 宿主 AST 链直接复用
- 引擎 `string_replace_inlined` 拆为 `string_replace_core(+invoke)` + 无回调入口; 函数替换实参在 `call_function` 调用点截获 (与 ArraySort/ArrayIter 同款, 回调需要分发器); 回调实参 = 全匹配, 捕获组..., offset(码单位), 全串
- **移除两处的"字符串键委托"分支** (`replace`/`match` 属性名非 `@@`): 命中面只有 bridge 自家 regex 对象 (与 exec 循环同语义) 和 `{replace:f}` 普通对象 (node 按字符串字面量处理) —— 委托分支保留反而制造 bc 链 r6/r7 走 fancy 展开的分裂
- 宿主 `JsValue::Array` 补稀疏具名属性槽 `ArrayStore{elems, props}` (对齐引擎 `ArrayData`, Deref 到 Vec 保住全部 26 个使用点, 战役 C 合一后只留一份); bridge `exec` 闭包挂 `index`(码单位)/`input`, 两链 exec 结果同口径
- 替换循环的字节起点改为就地 `find` 重算: exec 的 `index` 是码单位 (贴 node), 切片要的是字节位 —— 混用会在非 ASCII 上错位/panic
- 字符串模式 `match`: 两链统一按**字面量**单处命中 (无元字符时与 node 一致); node 实际是 `RegExpCreate` 按正则解释, 元字符分歧 (如 `.`) 已知未闭 —— 收口归引擎正则内核 (战役 H 前置)
- AST 链 `replace` 回调经 `invoke_function_named` 截获 (`string_replace_with_callback`), 与串替换同循环口径; 宿主 `StringReplace` native 对 callable 给显式 Err 标记 (同 ArrayFind 纪律)

**门禁**: `syntax_corner_cases` **35/35** (门禁 34 both_chains 钉 14 项 + 门禁 35 二轮边角 12 项全 node-EXACT: 回调返回值不展开、`$10` 两位优先、零宽全局、`lastIndex` 复位)、`v8-interpreter` **33/33**、`./sh/clippy.sh` **0 警告**、`./test.sh` **2924 passed / 1 skipped** (+2 即门禁 34/35)

**#23 未闭残项**: ~~⑤ 函数转字符串~~ (已闭, 见 12.24); ⑥ 内建方法物化到 `Array.prototype` 对象 (`Array.prototype.push.toString()` 现为 undefined, node 给 "native code" —— 与战役 C 的替身表单源化联动)

### 12.23 正则对象方法面收敛: 只暴露 exec/test (bc 链泄漏收口, 2026-10-15)

探针 `/tmp/p23d.js` 三方对拍: node 正则对象自有方法只有 `exec`/`test`, 宿主 AST 链一致, **bc 链三个都是 function** —— bridge 早先往 regex 对象挂了 `replace`/`split`/`match` 三个公开自有键。委托分支虽在 12.22 已撤, 键本身还漏在对象面上。

- bridge **删除** `replace_fn`/`match_fn` 两个闭包与落键: 引擎 `string_replace_inlined`/`string_match_inlined` 的检测早已统一走 `EXEC_KEY` exec 循环, 闭包是纯冗余副本 (fancy-regex `replace_all` 与 exec 循环两套语义并存, 正是 12.22 r6/r7 分裂的祸根形态)
- `split_fn` 保留 (引擎 exec 循环不处理 limit 参), 落键从 `"split"` 改哨兵 `SPLIT_IMPL_KEY = "@@split"` (引擎侧唯一定义, `@@` 前缀天然过 `is_internal_key` 非枚举过滤; bridge/value.rs 只 re-export): 用户对象的字符串键 `"split"` 不再被误委托 —— 规范里那是 `Symbol.split`, 而本引擎 well-known symbol 只落过 `toPrimitive`, 不借字符串键冒充
- 探针 `/tmp/p23e.js` 11 项 × node/AST/BC/default 四方: 除 s8 全绿。split/replace/match 语义经 closure 删除后无回归 (limit、空匹配、全局步长同 node)
- **顺带收口同面两处缺口** (探针 `/tmp/g36c.js`): (a) 正则**布尔标志读值**收敛为一张表 `RegExpData::boolean_flags()` (global/ignoreCase/multiline/sticky/unicode/dotAll, 后三个从 flags 串判定) —— 宿主 `get_property` 与 bridge 落表两处共用; AST 链此前缺 `flags`/`sticky`/`unicode`/`dotAll` 读值, BC 链缺 `ignoreCase`/`multiline`/`sticky` 等 (bridge 原先只落 `global`); (b) AST 链 `x in /re/` 此前直接 TypeError —— 现走与属性读同一张面判存在, 两链 `("split" in re)=false / ("exec" in re)=true` 同 node

**已知偏差记账 (未修)**: `"a1b".split(/(1)/)` node 给 `["a","1","b"]` (规范 RegExpSplit 把捕获组交叉进结果), 两链都给 `["a","b"]` —— fancy-regex `split` 不产捕获; 收口归引擎正则内核 (战役 H 前置), 与 12.22 的 match 字符串模式元字符偏差同归宿。同归 H 的还有: `re.compile` (node typeof function, 废弃 API, 两链 undefined)、bc 链 `"@@split" in re` 为真 (哨兵键在 `in` 面未滤, AST 链为假 —— 钉面探针未含此键)

**死代码扫描复核**: `/tmp/unused_r23.txt` 与收敛后终扫 `/tmp/unused_final.txt` (均 186 行) 对 D2 基线 `/tmp/unused_full.txt` (198 行) 只减不增 —— 12 行消失正是 D2 已删项, **零新增孤儿**; 撤掉的 `REPLACE_KEY`/`MATCH_KEY`/`SPLIT_KEY` 三常量仍被宿主 `String.prototype` 物化 (value.rs) 消费, `GLOBAL_KEY` 被引擎 exec 循环消费, `boolean_flags`/`SPLIT_IMPL_KEY` 两链各有读写 —— 扫描均未标, 不算孤儿

**门禁**: `syntax_corner_cases` **36/36** (新门禁 36 both_chains 钉方法面 8 项: typeof 五件 + `in` 三件 + split 行为 + 标志读值, 全 node-EXACT)、`./sh/clippy.sh` **0 警告**、`./test.sh` **2925 passed / 1 skipped** (+1 即门禁 36)

### 12.24 函数转字符串给源码文本 (#23 残项⑤闭, parser span 落地, 2026-10-15)

探针 `/tmp/p5c.js`/`/tmp/g37.js` 三方对拍: node 的 `decl.toString()` / `String(f)` / `"" + f` 三路都给**源码原文** (含内部空白逐字), 此前两链给 `[object Function]`/`[Function: n]` 宿主摘要。

- **span 通路** (一处定义, 单向流): lexer `Token` 携带字节 `pos` → `Parser<'a>` 持 `source: &'a str`, `slice_source(start)` 以刚消费的 `}` 为右锚切原文 → `SharedFunctionInfo.src: Option<String>` (函数声明/函数表达式/accessor 简写共用 `parse_function_tail`) → bridge 落 v8-ast `FunctionLiteral.src` → `InterpFunction.src` → 两链字符串化臂各读同一 `src`
- **AST 链截胡收口**: `FunctionData::get_property` 此前经 `Function.prototype` (prelude 里 `__proto__` 链到 Object.prototype) 解析到 `"[object Function]"` 分支, JsValue 层的 undefined 兜底永不触发 —— 源码替身**上移到显式 `__proto__` 链之后、Function.prototype 回落之前**, 与 BC 链引擎臂同序; JsValue 层重复拦截删除 (一处定义)
- **`String()` 全局走单一真相**: 宿主 `str_ctor` 内建原先 `v.to_string()` (Display 摘要 `[Function: n]` 外泄) → 改 `to_js_string()` (src 感知); bridge `interp_callable_to_js` 往返掉 src → 携带 `closure.func.src`; 无自有属性的闭包落 `NativeFn::InterpFunction` 一路也在 `to_js_string` 补 src 臂
- **顺带**: AST 链 `in` 对函数不再止步自有面 —— 自有/`length`/`name`/`prototype` miss 后按 `get_property` 同口径问原型链, `("toString" in decl)` 两链同 node 真

**已知偏差记账 (未修, 归战役 C)**: `Function.prototype.toString` 恒等式 (`f.toString === Function.prototype.toString`) 与 `f.constructor` 面在 AST 链函数值上不可解析 (探针 `/tmp/p5e.js` a2 实证) —— Function.prototype 面物化本就要等替身表单源化, 不入钉

**门禁**: `syntax_corner_cases` **37/37** (新门禁 37 both_chains 钉源码文本 9 项: 声明/具名表达式/匿名/方法简写/即时-invocation 五路 + `typeof f.toString` + `String(decl)===decl.toString()` + `in` + `String(原始值)` 回归, 全 node-EXACT)、`./sh/clippy.sh` **exit 0 / 0 error** (9 条 doc-list warning 为门禁 35/36 既有同类, 非阻断)、`./test.sh` **2926 passed / 1 skipped** (+1 即门禁 37); 新符号 (`FunctionToString`/`native_function_summary`/`new_with_src`/`SPLIT_IMPL_KEY`/`boolean_flags`) 跨 crate 引用复核均有读写方, 零孤儿

### 12.25 内建面扫面第一轮: 取整/parseInt/切片三法收口 (2026-10-15)

批量探针 (每项独立脚本 × node/AST/BC 三方对拍) 扫出缺口分两档: **正确性 bug** (有实现但语义偏) 与 **面缺失** (typeof undefined)。本小节收口正确性档 + 一处链分裂。

- **Math.round**: 原实现 `n.round()` 是 Rust 的远离零取整, 规范是 `floor(x+0.5)` (平局向 +∞): round(-2.5)=-2 而非 -3; [-0.5,0) 归 **-0** (`1/round(-0.4)===-Infinity`)。宿主内建一处修, 两链同治
- **parseInt**: radix 缺省/0 时不按规范自动判基 —— `"0x10"` 给 0 (应 16), `"-0x11"` 给 0 (应 -17, 符号剥离后才判前缀); 重写为 符号 → 判 0x/0X → radix 16 剥前缀 → 逐位收集。顺带 `v.to_string()`(Display) 改 `to_js_string()` 单一真相
- **切片三法收敛为引擎侧单一真相** (探针 `/tmp/p11b.js`/`p11c.js`): 此前两链各抄一份且已漂移 —— `substring` 借了 `slice` 的负索引从尾算法 (`substring(-2)` 给 "ef", node 给 "abcdef"), bc 链还把 slice/substring 合成同一 native (交换臂对 slice 也生效, `slice(4,2)` node 给 "" 而非 "cd"), **且 bc 链整体缺 `substr`** (AST 有 —— 链分裂, `--bytecode-only` 直接 TypeError)。现: 引擎新增 `SliceKind` + `slice_window` + `to_index_i64` (±∞ 饱和索引, to_i32 会把 ∞ 折 0 致 `substring(2,Infinity)` 走错臂), 三变体 `StringSubstring/StringSlice/StringSubstr` 两链各自接线共走该窗口; 宿主 `char_count/char_to_byte_index` 副本删除改 `pub(crate) use v8_interpreter::…` (一处定义); 多字节串 (`"中文abc"`) 按字符索引不裂
- **扫面结果存档** (未修部分进任务): 面缺失两链同缺 —— `JSON.parse` (只有 stringify)、`Object.values/entries/assign/getPrototypeOf/setPrototypeOf` (create/keys 有)、`Array.prototype.findIndex` (filter/map/forEach 有)、`Number.isInteger`、`Object.is`、`String.prototype` 的 `includes/repeat/padStart/padEnd/at/codePointAt/localeCompare`、`String.fromCodePoint/String.raw`; 另 AST 链 `Array.prototype.slice.call` 面分裂 (BC 有)。低频记偏差: `localeCompare` (node 走 ICU, 对拍风险高)、`String.raw` (模板字面量依赖)

**门禁**: `syntax_corner_cases` **39/39** (新门禁 38 取整+parseInt 16 项、门禁 39 切片三法含负例/Infinity/多字节/typeof, 全 node-EXACT)、`./test.sh` **2928 passed / 1 skipped** (+2 即门禁 38/39)、`./sh/clippy.sh` **exit 0**

### 12.26 内建面扫面第二轮: 12.25 面缺失清单收口 (2026-10-15)

12.25 扫出的面缺失档全部落地 (每项先 node 探针钉字符级, 再 `both_chains` 门禁, 再实现)。

- **String.prototype `includes/repeat/padStart/padEnd/at/codePointAt`**: 引擎侧新增纯函数 `str_repeat/str_pad/str_at/str_code_point_at/str_includes` 一处真相, 两链 NativeFn 各自接线共用 (引擎 `StringIncludes/…` 五变体 + 宿主同名五变体); 键名 `REPEAT_KEY/PAD_START_KEY/PAD_END_KEY/AT_KEY/CODE_POINT_AT_KEY` 一处定义、两侧 get_property 表共用。语义修正逐条对拍: `includes` 的 fromIndex **钳到 length** (`"".includes("",4)` node true, 原判越界 false)、`repeat/pad` 的 ±Infinity 走 `to_index_i64` 饱和哨兵 → RangeError (可 catch)、`codePointAt` 越界 **undefined** (区别于 charCodeAt 的 NaN)、`pad` 空 pad / 短宽度直通
- **`String.fromCodePoint`**: 范围 `0..=0x10FFFF` 越界 RangeError, 孤代理区落 U+FFFD (见偏差档); 与 fromCharCode 并排注册
- **`JSON.parse`**: 宿主递归下降 (literal/嵌套/字符串转义/数字文法扫一遍再 f64 解析、`\uXXXX` 高低代理对合并、尾字符垃圾检测), 注册进 `init_builtins` 经桥两链共读
- **Object/Number 静态面**: `values/entries/assign/getPrototypeOf/setPrototypeOf/is` + `Number.isInteger` (不做 ToNumber) 一处宿主定义; `own_enumerable_entries` 与 keys 共用 `enumerable_names` 单表; `getPrototypeOf` 走 `NativeFn::from_context` 拿 `env.get_object_prototype()` 兜底、原始值给 null; `Object.is` 复用引擎 `same_value_x64`
- **`Array.prototype.findIndex`**: 引擎 `ArrayIterKind::FindIndex` (拦截臂交下标非元素, 命中 `Smi(i)` / 未命中 `Smi(-1)`) + 宿主 `ArrayFindIndex` 标记变体同款截获; 回调内改元素后逐步 fresh 读与 find 对齐
- **记偏差 (未入钉, 归战役 C 值模型合一)**: BC 跨桥**深拷贝**致宿主 Object 静态面身份/变更丢失 —— `Object.assign(t,..)===t`、`getPrototypeOf(child)===base`、`setPrototypeOf` 改完读不回, 门禁只钉身份安全结构性项并注释原因; astral 面 (非 BMP 索引按码点而非 UTF-16 code unit) 只钉 BMP; 多键枚举顺序 gxhash 非插入序且随进程变, 门禁 `.sort()`; JSON.parse 错误是 `Error::RuntimeError` (本引擎无 SyntaxError 变体); accessor-only 伪键在 entries 跳过

**门禁**: `syntax_corner_cases` **42/42** (新门禁 40 Object/Number/String 静态面 + JSON 往返、41 字符串五法 + fromCodePoint、42 findIndex 语义, 全 node-EXACT)、`./test.sh` **2931 passed / 1 skipped** (+3 即门禁 40-42)、`./sh/clippy.sh` **exit 0 / 0 warning** (门禁文件 23 条 doc_lazy_continuation 按数字列表续行缩进清偿)

### 12.27 内建面扫面第三轮: Math 单目族 + String/Array 三批 + URI 编解码 (2026-10-16)

12.26 扫面残档落地 (流程同前: node 探针 `/tmp/g43.js` 字符级钉 → 两链对拍 → 门禁)。

- **Math 单目族**: ceil/trunc/cbrt/tan/asin/acos/atan/log2/log10/sign 十个名字收进 `NativeFn::MathUnary(&'static str)` 按名分派一个变体 (宿主 init_builtins 一处定义, BC 经桥共读; `Ptr` 是 fn 指针、闭包不能捕获循环变量, 表驱动只能走枚举); fround/clz32/imul/atan2/hypot/random 走显式闭包。两处真实缺陷: `Math.sign` 直接用 Rust `signum` 会把 ±0 交 ±1 (JS 要 IEEE: ±0 保号、NaN 传 NaN), 门禁 43 钉死; hypot 折叠 `acc.hypot(n)` 从 0.0 起步让 ±∞ 压过 NaN (node `Math.hypot(0,Infinity,NaN)=Infinity`)
- **String 实例面**: trim/trimStart/trimEnd (引擎纯函数 `str_trim` + `is_js_whitespace` 含 U+FEFF/2028/2029, 两链 NativeFn 共用)、startsWith/endsWith (位置夹取、缺省尾界 `i64::MAX` 哨兵)、replaceAll (空针串逐字符边界插入 + 末尾补一发 —— 首版把原文字符丢了, 探针抓出; `$$`/`$&` 复用 get_substitution; 非全局正则 TypeError、全局正则走 replace 主体; 回调替换挂战役 C)、**String.prototype.concat 补链分裂** (AST 早有、BC 表漏挂 → 引擎新增 `StringConcat` 变体)。**needle 缺省真实 bug**: `startsWith()/endsWith()/includes()` 省略实参时两链都按 `""` 判 (恒 true), 规范是 ToString(undefined)="undefined" —— 引擎 4 处 + 宿主 1 处一并改
- **Array 实例面**: at/flat/copyWithin 纯同步变体 + ArrayIterKind 新增 ReduceRight/FlatMap (回调截获与 find/findIndex 同路: 引擎 `array_iter` / 宿主 `invoke_function_named` 就地执行); copyWithin 快照-克隆且**返回接收者本体** (两链 `Arc::clone`, node `a.copyWithin(...)===a` true)
- **静态面**: `Number.isSafeInteger`、`Array.of/from` (from 吃字符串/类数组; **mapfn 回调两链都显式抛 TypeError 挂战役 C** —— 宿主 Ptr 桥接层跨不了引擎执行态, 宁报错不静默)、`Object.getOwnPropertyNames` (内部键过滤, 数组交下标+length)、`Object.fromEntries` (错误信息从 Array.from 文案纠正为 fromEntries 口径)
- **URI 编解码 + lexer**: encodeURI/encodeURIComponent/decodeURI/decodeURIComponent 四个全局走引擎 `uri_encode/uri_decode` 一处定义 (直通判定先限 ASCII 防 `ch as u8` 把 U+202D 折成 `-` 误直通; decodeURI 对保留字的合法 %XX **保持原文**; 坏序列 `Error::RuntimeError("URI malformed")`)。顺带补 **`\u{...}` 花括号码点转义** (旧 lexer 只认 `\uXXXX`, `\u{1F600}` 被拆成残字面量两链同错)
- **记偏差 (战役 C/豁免档)**: URI malformed 的 `error.name` 是 Error 而非 URIError (宿主 Error 枚举无该变体); Array.from mapfn、replaceAll 函数替换实参; astral `.length` UTF-16 面按 BMP 豁免 (门禁 48 只钉编解码不钉 length); 普通对象 `.constructor.name` 未接 (探针已剔除)

**门禁**: `syntax_corner_cases` **48/48** (新 43 Math 单目族、44 trim 族/startsWith/endsWith/needle 缺省/concat/isSafeInteger、45 replaceAll、46 Array at/flat/flatMap/reduceRight/copyWithin、47 Array.from/of + gOPN/fromEntries、48 URI + `\u{...}`, 全 node-EXACT)、`./test.sh` **2937 passed / 1 skipped** (+6 即门禁 43-48)、`./sh/clippy.sh` **exit 0 / 0 warning** (宿主 value.rs 键名导入漏 10 个致模式臂退化成通配绑定, 补 import 清零 unreachable_patterns; uri_decode let-else 按 `?` 清偿)

### 12.28 箭头函数 + 模板分组 + spread 口径 + toExponential + Object.prototype 枚举面 (2026-10-16)

内建面扫面第四轮: 新语法面 (箭头函数) + 三处两链同犯的真实缺陷 + 两个补面。流程同前: node 探针 `/tmp/arr1.js`/`/tmp/arr2.js`/`/tmp/g44c.js`/`/tmp/g45.js`/`/tmp/g46.js` 字符级钉 → 两链对拍 → 门禁 49-54。

- **箭头函数**: lexer 不合并 `=>` (Equal+Greater 原样), parser 前瞻认三形态 (单标识符/括号参数表含空/块体或表达式体); 表达式体走 `parse_expression(2)` (AssignmentExpression 级, 不含逗号 —— `f(x => a, b)` 的实参边界靠它守)。语义四处: 词法 `this`/`arguments` 在**闭包创建时刻定格** (引擎 LdaConstant 处镜像原版 JSFunction.context; 宿主 `closure.get("this")` 沿词法链), 引擎箭头帧经 `frame_arguments_lex` 把外层实参快照逐级下传; 无 `prototype` 属性 (宿主构造早退/宿主 `in`/引擎 `has_property_resolved` 三处同闸), `new` 一律 TypeError, toString 交源码切片, `.length` 只数形参。v8-ast `FunctionLiteral(Options)` 挂独立 `is_arrow` 位 (上游 FunctionSyntaxKind 被裁剪到 5 值, 门禁 COUNT=5 不动); 宿主 `uses_arguments` 静态分析对箭头体穿透 (箭头没有自己的 arguments, 体内引用就是外层的)
- **模板串表达式分组 (真实缺陷)**: 脱糖没给 `${expr}` 包 parens, `` `a${1+2}b` `` 左结合交 `a12b` (两链同错) —— lexer 注入成对 LParen/RParen, 门禁 51 钉
- **spread 展平口径**: 脱糖 `[a].concat(b)` 把整串当一个元素 (`[...""]` 交长度 1) —— spread 项包一层 `Array.from(...)`, 顺带数组展开取快照 (改原数组不回头污染已展平副本), 门禁 52 钉
- **Object.assign/values/entries 物化 accessor**: 原生替身只搬值槽, `@@get:` 伪键名在对象展开里整个丢 (`{...带getter对象}` 交空, 两链同错) —— 三方法移进 JS prelude 覆盖层, 取值全走会跳 getter 的属性读点; keys 枚举面保持原生
- **toExponential**: v8-numbers 新增 owned 包装 `double_to_exponential` (f=-1 最短口径本就在格式层); 两链 `NativeFn::NumberToExponential` 变体 + 数字表挂键; 参数缺省 (含显式 undefined) 走最短数字面, 0..=100 之外 RangeError, 门禁 53 钉
- **历史别名与枚举面**: `trimLeft/trimRight` 附录 B 别名与 `trimStart/trimEnd` or-pattern 共用实现; `propertyIsEnumerable` 按 Object.keys 自有真相 (hasOwnProperty 的 `p in this` 替身会把链上方法算进来); `isPrototypeOf` 沿 proto 链 identity 比对、64 跳兜底与属性读 max-depth 同口径 (getPrototypeOf 链顶两链都不吐 null), 门禁 54 只钉两链可对齐子集

**门禁**: `syntax_corner_cases` **54/54** (新 49 箭头形态/词法 this+arguments、50 箭头面 prototype/new/toString/length、51 模板分组、52 spread 展平、53 toExponential、54 别名/枚举/assign 物化, 全 node-EXACT)、`./test.sh` **2943 passed / 1 skipped** (+6 即门禁 49-54)、`./sh/clippy.sh` exit 0 / 0 warning

- **记偏差 (战役 C / 豁免新增)**: 解构族 (var/let/const 数组/对象模式、默认值、rest、重命名、形参解构、赋值式解构)、标记语句 (`outer: for`)、caught error 的 String()/name/message 保真 (两链交 `[object Object]`)、tagged templates、new+spread、实参展开 receiver 双求值 (apply 脱糖固有, 两链一致)、箭头非标识符形参 (显式 ParseError)、prelude JS 函数的 `.call` 属性读面 (AST 交 undefined/BC 交 `[object Function]`)、数组洞 (`0 in new Array(2)` 密模型交 true)、原生函数 `.length/.name` 元数据面 (`Math.max.length` 交 undefined)、BC 链原型链 identity (`__proto__ === Object.prototype`/`getPrototypeOf([])===Array.prototype`/isPrototypeOf 全集 —— 跨桥深拷贝同族)

---

### 12.29 内建面扫面第五轮: delete 口径 / typeof 与 == 名册 / for-in 接收者面 (2026-10-16)

探针 `/tmp/g47.js` `/tmp/g55.js` 字符级钉 → 两链对拍 → 门禁 55-57。本轮没有新增内建方法，收的是**三条语义基线**在两链上的分叉。

- **delete 口径收敛 (真实缺陷)**: 引擎 `delete_property` 旧实现"键不存在返回 false"，与 sloppy 规范 (恒 true) 反着 —— 重写为 sloppy 恒 true、唯一假值 `delete arr.length`；数组下标 delete 以 `Undefined` 填槽近似留洞 (`join` 吐空段与 node 同象)，具名键回收 `props` 槽。宿主 `UnaryOp::Delete` 两臂原先各自散判且根本不处理 Array 接收者 —— 新增 `delete_array_slot` 静态方法 (与引擎 Array 臂同源单点)，Member/Index 两臂共用；`delete "abc".x`/`delete (5).x` 不报错恒 true
- **for-in 字符串接收者 (链分裂)**: 引擎 `ForInEnumerate` 有 String 臂 (`"ab"` 交 `01`)，AST 链没有 —— `_ => Vec::new()` 直接零轮。补同名臂后两链逐字符一致；数字/null/undefined 两链均零轮 (与 node 的 ToObject 语义不同，豁免见下)
- **typeof 名册与 == 强制表**: 探针整表通过 —— undeclared 标识符不抛、`typeof null === "object"`、`void 0`、null/undefined 互等但与 0/""/false 全不等、NaN 自不等、`[1]==1`/`" "==0` 全部两链与 node 逐字符对齐，收进门禁 56 防回归

**门禁**: `syntax_corner_cases` **57/57** (新 55 delete sloppy 面、56 typeof/== 名册、57 for-in 接收者面, 全 node-EXACT)、`./test.sh` **2946 passed / 1 skipped** (+3 即门禁 55-57)、`./sh/clippy.sh` exit 0 / 0 warning

- **记偏差 (战役 C 新增)**: 真洞语义 (`delete a[1]` 后 node `1 in a === false`、`Object.keys` 跳过，密 Vec 交 true 且 keys 含洞位 —— join/索引读两象已对齐)；`delete o.r` 对 `defineProperty` 非配置属性 node false / 我们 true (缺可变性旗标存储，#18 同根)；`eval("delete x")` 对直接 eval 局部变量 node false / 我们 true；for-in 对象键插入序 (HashMap 桶序) 与继承键枚举 (不沿原型链) —— 门禁 55-57 只钉可对齐子集

---

### 12.30 内建面扫面第六轮: fromIndex 归一 / Number 进制前缀 / JSON.stringify 缩进 (2026-10-16)

探针 `/tmp/g58.js` `/tmp/g59.js` `/tmp/g60.js` 字符级钉 → 两链对拍 → 门禁 58-60。本轮修的是**三处真实缺陷与一处模型误读**。

- **Number() 前缀进制 (单点接线)**: 两链 `to_number` 的 String 臂原为 `trim().parse::<f64>()` 只认十进制 —— 改接 `v8-numbers` 的 V8 移植口径 `string_to_double(AllowNonDecimalPrefix)`，一处接线全量覆盖 ECMA-262 9.3.1 (0x/0o/0b、正负号、±Infinity、裸前缀交 NaN、首尾空白)，`*`/`-` 等强制转换路径同步受益；`parseInt` 判基仍只认 0x (V8 怪癖 `parseInt("0b101")→0`，与 Number 前缀名册分家，门禁 59 钉死防"顺手扩展")
- **fromIndex 归一 (真实缺陷)**: `[1,2].indexOf(2,-1)` 原交 0、node 交 1；`[1,2].includes(1,-1)` 原交 true、node 交 false —— 引擎侧新增 `tail_index` 自由函数与宿主侧 `JsValue::tail_index` 静态方法 (ToIntegerOrInfinity + undefined 回调用方缺省 + 负数从尾部归一 + 越界钳端点)，indexOf/includes 起跳与 fill 的 start/end 界共用同一份口径，原散落的 `to_i32().max(0)` 两处与 fill 局部 `bound` 闭包两处收拢；`lastIndexOf` 反向起算口径节点五项探齐不动 (±∞/巨数饱和未探不追)
- **JSON.stringify space 缩进 (模型误读订正)**: 上一轮"字符串步长不插换行"的 `SpaceGoesFirst` 模型系误读 —— node 逐字符复验**两种步长共用同一换行模型** (项前缀 = 换行 + 子层前缀, 收口 = 换行 + 本级 cur)，只差子层前缀算法 (数值 `cur+step` 累计、字符串按绝对深度 `step.repeat(d)` 重复)；步长截 10、`""`/`0`/`NaN`/负数→紧凑、`1.7` 截断取整、`Infinity` 钳 10、数字串 `"10"` 按字面用、制表符步长对齐
- **两处缺陷自防回炉**: 前一轮 closure 的 `n.floor().clamp(0.0,10.0)` 遇 NaN 直接 panic (违"禁 panic"红线) —— 先自判 `is_nan` 归紧凑再钳；顶层 `JSON.stringify(undefined)` 原交空串 —— 按规范改回吐 undefined 值本身 (容器槽位仍照旧过滤/null，门禁 58 sj 行两象并钉)

**门禁**: `syntax_corner_cases` **60/60** (新 58 排序/回调/字符串/Number 面、59 下标归一与进制前缀全表、60 JSON 缩进全表, 全 node-EXACT)、`./test.sh` **2949 passed / 1 skipped** (+3 即门禁 58-60)、`./sh/clippy.sh` exit 0 / 0 warning

- **记偏差 (十五 B 六轮)**: `JSON.stringify` 的 replacer 函数参、`toJSON` 钩子与循环引用 TypeError 全缺 (回调跨执行态同三轮残档族)；字符串步长截 10 按码点而非 UTF-16 码元 (astral 步长未探，与三轮 astral 残档同根)

---

### 12.31 内建面扫面第七轮: reduce 族 / 位运算与 try-finally / 基元接收者 hasOwnProperty / split 缺省针 (2026-10-16)

探针 `/tmp/g61.js`–`/tmp/g72.js` 字符级钉 → 两链对拍 → 门禁 61-63。本轮修的全是**真实缺陷** (六处)，位运算/try-finally 名册整表零分叉收钉防回归。

- **reduce 族 IsCallable 前置 (真实缺陷)**: prelude `Array.prototype.reduce` 缺 `typeof fn !== "function"` 首行抛 —— 规范 IsCallable 检查先于遍历，**零轮回调也必须抛** (`[].reduce()` / `[].reduce(undefined,1)` node 交 TypeError)；门禁 61 r2 四象并钉
- **hasOwnProperty 自属性口径 (真实缺陷 + 基元回落链分裂)**: prelude 旧实现以 `p in this` 代自有判定 —— 把原型方法 (`{}.hasOwnProperty("toString")`) 计为自有，重写为 `Object.getOwnPropertyNames(this).indexOf(String(p)) !== -1` (symbol 键先行 false)。`.some` 两版试过废掉: 名册数组跨桥后回调法在重解析 prelude 内失败 (`TypeError: 'some': undefined is not a function`，属三轮回调族)，`indexOf` 桥安全且 gOPN 含数组 `length` 槽 (`keys` 不含)。**宿主链字符串基元接收者分叉**: `get_property` String 臂是闭表、不遍历原型 —— `"ab".hasOwnProperty(...)` AST false / BC+node true (链分裂)，新增 `NativeFn::StringHasOwnProperty` 显式回落臂 (下标 < char_count 或 `"length"` 交 true)，与引擎 boxing 路径对拍 (门禁 63 h3)；`(5).hasOwnProperty("x")` 两链 false 一致，Number/Boolean 基元面未对齐 node 的 ToObject 全名册、未追
- **getPrototypeOf(函数) (真实缺陷)**: 宿主 `get_prototype_of_fn` 无函数臂 —— `Object.getPrototypeOf(f)` AST 交 null (BC 正确)；补 Function/Native 臂走 `env["Function"].prototype`
- **split 两点 (真实缺陷)**: `limit` 参数此前两链都**没接** —— ToInt32 口径负数走全量 (node ToUint32(-1)≈max)、`parts.truncate(lim)` 收口 (含 regex 针路径)；`split()` 缺省针原把 `"undefined"` 当针 (`"aundefinedb"` 被劈) —— undefined 先行回吐 `[whole]`。AST 侧 limit/空串针早已正确，本轮补的是引擎侧半边
- **位运算/一元加负与 try-finally 控制流 (零分叉收钉)**: `5000000000|0→705032704` 回绕、`NaN/±Infinity|0→0`、`-1>>>0→4294967295`、`"0x10"|0→16` (靠 12.30 的 `string_to_double` 前缀接线)、`+[]→0`/`-"7"→-7` 全表对齐；return 与 finally 求值序、finally 内 return 覆写、循环 continue/break 走 finally、嵌套 catch 交还 —— 两链与 node 逐字符一致，门禁 62 整表钉死防回归

**门禁**: `syntax_corner_cases` **63/63** (新 61 reduce/every/some 族边界、62 位运算与 try-finally 控制流、63 split 缺省针/limit + hasOwnProperty 基元接收者 + accessor 面, 全 node-EXACT)、`./test.sh` **2952 passed / 1 skipped** (+3 即门禁 61-63)、`./sh/clippy.sh` exit 0 / 0 warning

- **记偏差 (十五 B 七轮)**: `Object.prototype.toString.call(基元)` 应交 `[object String]` 类 tag、宿主替身交值本身 (StringToString 兜底，未探，与"prelude JS 函数 `.call` 属性读"同族挂战役 C)；Number/Boolean 基元接收者的原型方法全名册对齐 (本轮只收字符串接收者 hasOwnProperty 一臂)

---

### 12.32 内建面扫面第八轮: String.lastIndexOf 全表 / 针参强制转换 / 全局环境名册缺口记账 (2026-10-16)

探针 `/tmp/g73.js`–`/tmp/g87.js` 字符级钉 → 两链对拍 → 门禁 64-65。本轮一处整方法缺失的真实缺陷，加一批环境面记账。

- **String.prototype.lastIndexOf 两链全缺 (真实缺陷)**: `"hello".lastIndexOf(...)` 两链直接 TypeError —— 引擎新增 `str_last_index_of` 纯函数一处定义、宿主 `NativeFn::StringLastIndexOf` 复用同一份。**node 实测口径** (63 格全表钉死): 负 pos **只扫下标 0** (`lastIndexOf("h",-1)` 交 0、`("l",-1)` 交 -1 —— 与 indexOf/includes 的 `tail_index` "从尾归一"**不同源，不可合并单点**)；NaN/undefined/对象→len、±Infinity→钳位/-1、起点上界 `min(pos, len_cu - needle_cu)` (窗口按 needle UTF-16 码元长)、空针交钳位位。口径三轮迭代 (从尾归一 → 区间空 + 布尔转写 `i32::from` 交 1 的笔误 → 全表对拍) 才收敛，期间探针还抓到并修掉**多字节串跨字符边界切片 panic** 两起 (窗口推进改逐字符累计 `len_utf8`/`len_utf16`，杜绝 `start byte index is not a char boundary`)
- **针参强制转换名册 (零分叉收钉)**: `indexOf/lastIndexOf/at/includes` 的 `null→"null"`、`undefined→"undefined"` 字面量针、`at` 截断口径、大小写敏感 —— g73/g76 两链与 node 逐字符一致，收进门禁 64/65
- **Array 无回调面 (零分叉收钉)**: `copyWithin` 负 target/缺省、`flat` 深度钳制 (0/负/NaN→0、Infinity→全平)、`flatMap`、函数 `.length` 形参位 —— g77 整表对齐 (原生函数 `.length` 面仍挂四轮残档)
- **全局环境名册缺口 (本轮记账不本轮修)**: `globalThis`/`setTimeout`/`setInterval`/`queueMicrotask`/`Object.groupBy`/`Object.hasOwn`/`structuredClone` 两链一律缺 (`typeof` 交 undefined，node 全 function/object) —— **两链一致故非链分裂**，属宿主 `init_builtins` 全局表不完整；timer/微任务面另需事件循环收口语义 (rs8 求值末同步排空还是异步续跑，影响输出序)，下一批单独处理

**门禁**: `syntax_corner_cases` **65/65** (新 64 lastIndexOf 全表与字符串搜索族、65 针参强转与 Array 无回调面, 全 node-EXACT)、`./test.sh` **2954 passed / 1 skipped** (+2 即门禁 64-65)、`./sh/clippy.sh` exit 0 / 0 warning

- **记偏差 (十五 B 八轮)**: `localeCompare` 两链缺 (ICU collation 无基础设施，naive 码点序对 node 高频分叉 —— `("A","a")` icu=1/code=-1、`("abc","ABC")` icu=-1/code=1，实测否决，非顺手活)；全局环境名册缺 (`globalThis`/timers/微任务/`Object.groupBy`/`hasOwn`/`structuredClone`，两链一致，见上)；`Object.freeze`/`isFrozen`/`getOwnPropertyDescriptor` 未挂键 (TypeError，属十五 B 三轮旗标存储族)；Symbol 面缺致 `Number(symbol)` 的 TypeError 象无法表达 (node RE / 我们走 undefined  coercion)

---

### 12.33 内建面扫面第九轮: Object.hasOwn / String.raw 手写对象形 / 静态面清点记账 (2026-10-16)

探针 `/tmp/g91.js`–`/tmp/g95.js` 字符级钉 → 两链对拍 → 门禁 66。两处纯补齐面 (prelude JS 单定义、两链共用)，外加一次全局静态面清点。

- **Object.hasOwn 两链全缺 (真实缺陷)**: prelude 新增，委托自家 `Object.prototype.hasOwnProperty.call` —— **不另立口径**，与七轮 gOPN+indexOf 替身同源，基元接收者 (`"ab"` 的 `"0"`/`"length"`) 与数组 `length` 自有面自动对齐；`hasOwn(null,…)` 走 call 装箱 TypeError 路径。g91 全象 node-EXACT (`y1=true|false|true|true|false#y2=RE|false|true`)
- **String.raw 两链全缺 (真实缺陷)**: 手写对象形 `String.raw({raw:[…]}, …subs)` 先收 (tagged templates 本体仍挂四轮残档 lexer Err)。**node 实测口径** (g93/g94/g95 矩阵钉死): 段数纯由 `raw.length` 定 (**空名册吐 ""**，首段必须入循环受 length 约束); 实参**存在**才插入 —— 缺位不吐 `"undefined"` 字面 (`raw[a,b,c]` 无 subs → `"abc"`)，显式传 undefined 才吐 (`"aundefinedbc"`); 多余实参忽略; 下标对象形名册 (`{0:"A",1:"B",length:2}`) 可用; `raw(null/undefined/{})` TypeError。算法两轮迭代收敛 (首版把 `raws[0]` 提出循环 → 空名册吐 `"undefined"` 分叉，探针抓住)
- **静态面清点 (g92 census, 两链一致缺口记账)**: node v26 `gap=none`；我们剩 `Object.freeze/seal/isFrozen/isSealed/isExtensible/preventExtensions` (旗标存储族, 三轮同根)、`Object.getOwnPropertyDescriptor(s)` (同根)、`Object.groupBy` (ES2024)。`hasOwn`/`raw` 本轮出列，余 9 个另批处理 (descriptor 族需先定值/旗标存储模型，非纯 prelude 活)
- **clippy 订正**: `str_last_index_of` 手写计数器 `cu_at` 触 `explicit_counter_loop` → 改 `chars().enumerate()`，逐字符 +1 即 enumerate 序，语义不变

**门禁**: `syntax_corner_cases` **66/66** (新 66 hasOwn+String.raw 全矩阵, 全 node-EXACT)、`./test.sh` **2955 passed / 1 skipped** (+1 即门禁 66)、`./sh/clippy.sh` exit 0 / 0 warning

- **记偏差 (十五 B 九轮)**: 静态面剩 9 缺项 (freeze 族/descriptor 族/groupBy, 见 census 上)；`String.raw` 真模板面 (cooked+模板对象) 仍随 tagged templates 挂四轮；性能项 #28 分发环两轮负载封锁 (load 6.45~18.50 > 6 阈值)，数字不记账

---

### 12.34 内建面扫面第十轮: Object.groupBy / 空原型对象继承面收敛 / 数组字面量空位解析 (2026-10-16)

探针 `/tmp/g96.js`–`/tmp/g105.js` 字符级钉 → 两链对拍 → 门禁 67。由 `Object.groupBy` 一个补齐面牵出**空原型对象继承面**的两链三处真实缺陷。

- **Object.groupBy 两链全缺 (真实缺陷)**: prelude JS 单定义两链共用。node 实测口径: 数组样 (length 驱动) 名册、元素与键按 Get 语义 (洞收 undefined, 稀疏 `[1,,3]` 也入组)、键 `String()` 化、结果挂 `Object.create(null)` (getPrototypeOf 交 null)、`length`=2、null/undefined iterable 与非 callable 次参抛 TypeError；`Array.prototype.group`/`Object.groupToMap` node v26 实测**不存在** (探针钉死，不实现)
- **数组字面量空位解析缺失 (真实缺陷, 两链共享 parser)**: `[1,,3]` 直接 ParseError "Unexpected prefix token Comma"。修复于数组字面量循环: 逗号相毗邻 → 洞位以 `Undefined` 填槽 (密 Vec 模型, 与 `new Array(2)` 同款近似)，尾逗号 `[1,2,]` 收括号即止不误收 —— `length`/JSON null/join 空段/`concat`/`indexOf` 五象与 node 逐字符对齐；`map` 跳洞 (node 保留洞位) 分叉挂洞模型残档
- **空原型对象继承面 (真实缺陷 ×3)**: `Object.create(null)` 产物在两链被全局回落兜底污染 —— ① 引擎 `in` (`has_property_resolved`) 链尾 break 后无条件回落 `global_proto(Object)` → `"toString" in c` 误真 (链分裂: AST 假/BC 真)；修: 显式 `__proto__=null` 置 `stopped_at_null` 跳回落。② 引擎读路径 `get_property_resolved` 同型回落给 `c.toString` 造出替身函数；修: 新增 `chain_ends_at_null` 单点判定 (与 get_property_depth 的 null 链顶护栏同口径, 一处判定)。③ 宿主 Object 读臂链顶 null 后仍落内建 `TO_STRING_KEY/VALUE_OF_KEY` 兜底臂；修: `Some(Null|Undefined)` 即交 undefined。**node 钉**: `("toString" in c)=false`、`c.toString=undefined`、`c+""` 抛 TypeError "Cannot convert object to primitive value" (两链现全对)、`String(c)` 应同样抛但两链交 "[object Object]" (宿主 `to_js_string` 不做真 ToPrimitive 的转换核短路, 记账另批)
- **prelude 函数 `.length` BC 链全缺 (链分裂, 真实缺陷)**: `Object.assign.length` AST 1 / BC undefined —— 宿主 JS 函数过桥变引擎 Native, 引擎 Native 臂不派生 length/name。修: `js_value_to_interpreter_value` 的 Function 转换把 `params.len()`/name **预置进 props** (合成面随桥带, 一处收敛); `Object.assign` prelude 签名补名义第二参对齐 node `.length`=2。prelude 函数 String() 交 `[native code]` 形态残差仍挂四轮 (跨桥身份族)
- **census 复点**: 静态面余 8 (freeze/seal/isFrozen/isSealed/isExtensible/preventExtensions + getOwnPropertyDescriptor(s)，descriptor 族需先定值/旗标存储模型，独立批次)；Map/Set 集合面整体缺 (`typeof Map` node function / 两链 undefined，另批)

**门禁**: `syntax_corner_cases` **67/67** (新 67 groupBy+空原型面+空位解析+静态 `.length`, 全 node-EXACT)、`./test.sh` **2956 passed / 1 skipped** (+1 即门禁 67)、`./sh/clippy.sh` exit 0 / 0 warning

- **记偏差 (十五 B 十轮)**: `String(null原型对象)` 直转不抛 (宿主 `to_js_string` 短路, 与 `c+""` 真 ToPrimitive 两轨并存 —— 收敛到一处是战役 C 转换面课题)；`map/filter/forEach` 对字面量洞位不跳 (回调收到 undefined, node 跳洞保洞, 挂密 Vec 洞模型)；`Object.keys` 桶序抖动两链互不一致 (四轮 for-in 插入序残档同根, 门禁已用 `.sort()` 隔离)；Map/Set/集合族与 descriptor/freeze 族余档见上

### 12.35 内建面扫面第十一轮: RegExp exec/test lastIndex 源真相 / sticky 锚定 / split 捕获组 (2026-10-16)

探针 `/tmp/g106.js`–`/tmp/g111.js` 字符级钉 → 三方对照 (AST/BC/node) → 门禁 68。收正则**行为面**三族缺陷；正则对象的枚举/原型面 (`/tmp/g112`–`g115` 新探得三向互异) 立为第十二轮输入。

- **exec/test lastIndex 源真相 (BC 链分裂, 真实缺陷)**: 宿主正则过桥落表带 `lastIndex` 引擎属性，而 exec 闭包只推进宿主 Arc —— JS 读到的属性恒 0；且 BC 的 `test()` 完全没有 lastIndex 副效应面 (t1 探针)。修: exec 核心以 receiver 引擎对象属性为源真相，命中/失败/越界经 `sync_last_index` **双写** (属性 + Arc)；引擎内部 replace/split 循环显式传 args[1] 位置、不触该路径。`test()` 委托同一核心收敛为单点
- **sticky `y` 锚定与推进 (两链同犯 × node, 真实缺陷)**: 原实现 y 不读 lastIndex (非锚定从 0 匹配)、命中不推进、失败不复位。`RegExpData::sticky()/tracks_last_index()` 单点定义 (g∨y 才参与 lastIndex，非 g/y 手置值原样保留 —— node `l3=z|3` 钉)；锚定语义 = 整体命中起点必须恰为探测点，靠后视作未命中；桥端 `regex_exec_core` 一处 + 宿主 `RegExpTest/RegExpExec` 两臂同口径。`gy` 组合循环、lastIndex 越界 (`null|0` 复位) 全对齐
- **BC 正则文本形链分裂 ×2 (String(/re/) 与 JSON.stringify(/re/))**: node `/a(b)c/g` 与 `{}`，BC 曾 "[object Object]" 与全表泄漏。三处收敛: ① 引擎 `Value::regex_repr` 单点 (认桥端独有的 `"@@split"` 哨兵, 拼 "/source/flags")，接 `to_str_lossy` Object 臂与 `get_property_resolved` TO_STRING 兜底；② 真凶是 `t+""`/`t.toString()` 解到桥来的 `Object.prototype.toString` 原生 —— 桥构造预置**自有 toString** (ConstString) 遮蔽；③ 边界身份还原 `interp_to_js_with_visited` 认哨兵重建 `JsValue::RegExp` (带 lastIndex 搬运)，宿主 JSON/转换臂原生直接受益 —— 一处还原喂所有宿主原生接收者
- **split 捕获组拼入 + 空匹配规则 (两链同犯 × node, 闭 12.23 "split `/(1)/`" 残项)**: `regex_split_parts` 一处定义 (宿主 StringSplit 与桥端 `@@split` 两链共用)，按 ES2018 后 RegExpSplit: 未参与捕获组插 `undefined` 非 "undefined" 字面 (node d5 JSON 形 `["a","1",null,""]`)；零宽命中恰在上一刀口 q → 不切、前进一个字符边界重试 (c2 `["a","c","d"]`、d2 `["","b"]`、c9 `["x"]` 皆由此推出)；node split 无 lastIndex 副效应 (预置 1 实测保留) —— `captures_from_pos` 直接定位天然满足；limit/命名组/尾部空段矩阵全绿
- 探针教训: 正则 `a+b` 的量词作用于**前一 token** (`a+` 后接单个 b)，据此误推 node 基准错误的过程反证了引擎正确 —— 三处缺陷全是自身分叉

**门禁**: `syntax_corner_cases` **68/68** (新 68 exec 捕获组/lastIndex/index/input + sticky 全象 + test 副效应 + gy 组合 + toString/JSON 文本形 + replace $ 序列 + split 全矩阵, 全 node-EXACT)、`./test.sh` **2957 passed / 1 skipped** (+1 即门禁 68)、`./sh/clippy.sh` exit 0 / 0 warning

- **记偏差 (十五 B 十一轮 → 第十二轮输入)**: 正则对象枚举/原型面三向链分裂 —— `Object.getOwnPropertyNames(/a/g)` node `["lastIndex"]` / AST `[]` / BC 泄漏全表；`r.propertyIsEnumerable` AST undefined (宿主 RegExp 读臂无方法继承回落) / BC 有但 `String()` 吐 "[object Function]"；`Object.getPrototypeOf(r)` node `/(?:)/` / 两链 null；`String(r.exec)` BC 匿名化 (跨桥身份族四轮同根) —— 正则对象是 `$$RegExp` 桥往返构造、无 RegExp.prototype 实体，归战役 C 值模型口径；性能项 #28 第四轮负载封锁 (load 7.79~10.17 > 6)，数字不记账

### 12.36 内建面扫面第十二轮: 正则对象枚举/原型面收敛 (2026-10-16)

探针 `/tmp/g118.js`–`/tmp/g120.js` 字符级钉 → 三方对照 (AST/BC/node) → 门禁 69。收十一轮记账的枚举面链分裂：标准正则的 GON/keys/hasOwnProperty/propertyIsEnumerable/hasOwn/for-in 面**三方逐字符对齐**；expando 属性面新探得、立为十三轮输入。

- **正则实例 own 名册一处定义**: `RegExpData::own_property_names()` = `[lastIndex]` —— node 实测**非全局正则也带 lastIndex** 自有槽 (`Object.getOwnPropertyNames(/b/)` = `["lastIndex"]`)；`source`/`flags`/六布尔是原型访问器、`exec`/`test`/`toString` 是原型方法，均非自有。宿主 GON 原生加 RegExp 臂为唯一名册消费点 —— **一处修两链受益** (BC 经边界身份还原喂进同一宿主臂)，并级联修好 BC prelude JS `hasOwnProperty`/`Object.hasOwn` (gOPN+indexOf 口径) 与 AST `Object.hasOwn`；keys 臂不动 (lastIndex 不可枚举 → 恒 `[]`，node 同)
- **AST 读臂回落 (链分裂修复)**: `r.hasOwnProperty`/`r.propertyIsEnumerable` AST 原为 undefined (TypeError, BC 有值 —— 真链分裂)，宿主 RegExp 读臂物化 `NativeFn::RegExpHasOwnProperty` (own 判定复用 `own_property_names` 单点) / `RegExpPropertyIsEnumerable` (恒 false: 唯一自有键不可枚举)，与 `StringHasOwnProperty` 回落臂同款显式对齐策略；引擎常量表新增 `PROPERTY_IS_ENUMERABLE_KEY` (单一真相, 两链 re-export)
- **BC for-in 伪键泄漏 (链分裂, 真实缺陷)**: `for (var p in /a/g)` node/AST `[]` / BC 吐 12 个桥代管键 (source/flags/六布尔/exec/test/toString/lastIndex) —— ForInEnumerate 走引擎对象原始 map, 不经边界身份还原。修: 引擎 `is_regexp_pseudo_key` 名册单点 + ForInEnumerate Object 臂按 `"@@split"` 哨兵识别正则对象才过滤 (普通对象零影响, 用户 expando 保留)；gy 组合正则同样交空；顺带收字符串装箱枚举回归护栏 (`"x"[0]` 写忽略 → for-in/keys `["0"]`)
- 十一轮记账面收口: g112–g115 的 GON/propertyIsEnumerable 分歧转 node-EXACT；`getPrototypeOf(正则)` 实体与 `String(r.exec)` 跨桥身份仍挂档 (见下)

**门禁**: `syntax_corner_cases` **69/69** (新 69: GON/keys/hasOwnProperty 四键/propertyIsEnumerable/hasOwn/typeof/for-in 标准+gy/装箱回归, 全 node-EXACT)、`./test.sh` **2958 passed / 1 skipped** (+1 即门禁 69)、`./sh/clippy.sh` exit 0 / 0 warning

- **记偏差 (十五 B 十二轮 → 第十三轮输入)**: **正则 expando 属性面两链各向不同方向分裂 (真实链分裂)** —— `r.foo=1` 后 node: `r.foo`=1 / GON `["lastIndex","foo"]` / `hasOwnProperty("foo")`=true / for-in `["foo"]`；AST 全丢 (宿主 `JsValue::RegExp` 无属性袋, `set_property` 只存 lastIndex, 读 undefined)；BC 读 1 (引擎对象存表) 但 GON/hasOwnProperty 不见 foo (名册静态)。修复需 `RegExpData` 增设 props 存储面 + 桥两向搬运 + `own_property_names` 动态化 —— 战役 C 值模型本体，非零散臂可收；性能项 #28 第五轮负载封锁 (load 7.69 > 6)，数字不记账

### 12.37 内建面扫面第十三轮: 正则 expando/方法遮蔽面 (2026-10-16)

探针 `/tmp/g121.js` (expando 13 象) + `/tmp/g123.js`/`/tmp/g124.js` (JSON 面) 字符级钉 → 三方对照 → 门禁 70。十二轮记账的 expando 链分裂收口：**g121 e1–e9/e11–e13 与 JSON 面三方逐字符对齐** (含 exec 遮蔽的 hasOwnProperty/for-in/调用、`r.source="X"` 静默忽略、delete 名册回收、lastIndex 与 expando 并存推进)。

- **宿主 `RegExpData.props` 自有袋 (存储面补齐)**: expando/方法遮蔽键落袋；`set_property` 三路由 —— lastIndex 走专用字段、只读访问器名静默忽略、其余入袋；`get_property` props-first (own 强于代管面, `r.exec=fn` 遮蔽生效)；名册动态化 `own_property_names` = lastIndex + 袋键、`enumerable_own_names` = 袋键 (keys/for-in/propertyIsEnumerable/hasOwn/GON 全消费同两名册, 一处定义)；十二轮静态 `[lastIndex]` 口径作废
- **只读名册单点收敛**: 引擎新增 `is_regexp_readonly_key` (source/flags/六布尔)，宿主 set 臂与引擎 set 护栏共读同一张表 —— 原宿主副本 `RegExpData::is_readonly_accessor` 删除 (零 wrapper)；引擎 `Value::set_property`/`delete_property` Object 臂按 `"@@split"` 哨兵识别正则才设防 (普通对象零成本一条哈希探测后直过)
- **桥两向搬运 (链分裂核心修复)**: host→engine 构造臂把 props 逐键落引擎表 (同名 exec/test/toString 覆盖代管基形即遮蔽语义)；engine→host 身份还原按 **`@@shadow:{name}` 墓碑键**区分代管形与用户遮蔽 —— 引擎 set 臂在**覆盖已有槽位**时落墓碑 (构造首写与桥搬运首轮不触发, 门禁 69 标准正则名册不回退)，还原侧非内部非 lastIndex 键: 伪名带墓碑 = 用户遮蔽 → 回搬入袋，无墓碑 = 代管形 → 丢弃。for-in 过滤同步认墓碑 (exec 遮蔽 node 口径可枚举 → BC `["exec"]` 对齐)
- **JSON.stringify 正则 expando 参与 (node `{"foo":1}`)**: 原 RegExp 臂硬编码 `"{}"` 改走袋上对象体序列化；序列化体重构为 `json_object_body` 单点 (Object 臂与正则臂共用, 键滤内部伪键/`__` 槽位键、排序、pairs 空退化 `{}` —— 原两臂 20 行重复消除)；裸正则袋空自然回到 `"{}"` 形，嵌套/缩进面全对齐
- **delete 面**: AST Member/Index 两臂补正则袋回收 (`delete r.bar` 后名册剩 lastIndex)；引擎 delete 护栏对全部伪名一律拦删 (BC 拦得住代管形断供, 但遮蔽态删除不回落 —— 见记账)

**门禁**: `syntax_corner_cases` **70/70** (新 70: 读写/名册/枚举/hasOwn/in/只读忽略/exec 遮蔽三象/lastIndex 推进/delete 回收/JSON 三口径, 全 node-EXACT)、`./test.sh` **2959 passed / 1 skipped** (+1 即门禁 70)、`./sh/clippy.sh` exit 0 / 0 warning

- **记偏差 (十五 B 十三轮)**: `String(r4)` (toString 遮蔽后显式直转) 两链一致交 `/d/` / node `CUSTOM` —— 与 12.34 记账的 `String(c)` 直转两轨同根 (战役 C 转换面合一课题, 隐式 `+`/模板已正确走 ToPrimitive 交 CUSTOM)；`Object.prototype.toString.call(正则)` 交 `[object Object]` / node `[object RegExp]` (七轮 toString.call 名册同根)；BC 遮蔽态方法名 delete 拦删不回落代管基形 (AST 删袋回落原生可用, node 删后回落原型 —— 三向边缘象, 跨桥身份族)；引擎对象袋内 `@@shadow:` 墓碑键随正则往返恒驻 map (内部键全枚举口已滤, 无外部可见性)；性能项 #28 第六轮负载封锁 (load 8.64 > 6)，数字不记账

### 12.38 内建面扫面第十四轮: 解构族 (2026-10-16)

探针 `/tmp/g125.js` (7 形) + `/tmp/g129p.js` (15 形) + `/tmp/g131p.js` (getter/eval 定性) 字符级钉 → 三方对照 → 门禁 71。闭合四轮残档"解构族全缺"主躯干: g125 七项与 g129 的 e1/e3/e4/e6/e7/e8/e9/e11/e12/e13/e14/e12 共 12 形三方逐字符对齐 (嵌套、无空格、混列、rest 越界、对象 rest 摘键、字符串源、形参默认、右值一次性、空模式、空位尾逗号、既有绑定)。

- **解析器侧暂态 Pattern + desugar 展开单点 (两链一份实现)**: AST 新增 `Pattern` (Binding/Array/Object/Default) 纯中间形态与 `Stmt::VarPattern`/`Expr::AssignPattern` 暂态节点；`parse_pattern` 独立解析数组/对象模式 (表达式解析器不可复用 —— `take()` 破坏性消费无回溯，且表达式位 `[...]`/`{...}` 早已折成 concat/Object.assign 形态)；`desugar_stmts` 挂 `parse_program`/`parse_block` 两漏斗出口，把暂态展开成普通 VarDecl/赋值/索引/成员/`slice`/`Object.assign`+`delete` 语句序列 —— 解释器、字节码桥、hoisted/uses_arguments 扫描零感知，无任何新增执行臂 (一处定义红线)
- **展开口径**: 容器层先落 `$dN` 临时量保证右值只求值一次 (探针 t9: cnt=1)；默认值 `=== undefined` 判定 (非缺省判定, null 不触发)，非标识符源先快照 `$u` 再条件式 (属性/getter 只读一次, node 口径)；数组空位占索引、尾逗号合法、`...rest` 折 `base.slice(n)`；对象 rest 折 `Object.assign({}, base)` 后按消费键逐个 `delete $r.k`；嵌套模式递归展开、叶绑定免临时量直落初值
- **语句位 `[a]=b` 判型**: LBracket 头非破坏性配平扫描 (只数括号深度)，闭合后紧跟孤立 `=` 才认定 (排除 `==` 与 `=>`=Equal+Greater 复合形)；普通索引语句 `[1,2][0]`、块头 `{...}` 零扰动 (门禁 t13 护栏)
- **解构参数**: 新增 `parse_param_table` 单点，函数声明分支与 `parse_function_tail` 两处重复参数循环消除合并；模式参折 `$argN` 槽名 + 体内前言 VarPattern，**拼接与 desugar 必须先于 `SharedFunctionInfo::new_with_src`** (hoisted_vars 按最终 body 静态扫, 后拼即漏声明)；`f({v})`/`f([m, n=4])` 三方一致 (t7/t8/t16)
- **暂态防线与零扰动**: var 分支纯标识符名册保持原 VarDecl/VarDecls 输出形态 (既有门禁零扰动)，混列逐条冲刷拆语句；解释器 `eval_stmt`/`eval_expr` 补 ParseError 报错臂 (禁 panic)、桥两 lowering 函数补 None 回落臂、`uses_arguments` 两扫描补右值盯防臂

**门禁**: `syntax_corner_cases` **71/71** (新 71: 16 项嵌套/默认/rest/空位/形参/一次性/空模式/护栏, 全 node-EXACT)、`./test.sh` **2960 passed / 1 skipped** (+1 即门禁 71)、`./sh/clippy.sh` exit 0 / 0 warning

- **记偏差 (十五 B 十四轮)**: 数组面非可迭代 RHS (node `TypeError: 5 is not iterable` / 我们按索引读绑 undefined) —— 迭代器协议缺失，与三轮 keys/values/matchAll 残档同根 (战役 C)；模式参缺实参 (node `TypeError: Cannot read properties of undefined` / 我们 RuntimeError 顶, 错误名册七/八轮同根)；~~for-of 全缺 (探针 e11a 两链抛, 独立批次)~~ **已于 12.41 闭合**；入口名册未接: 括号表达式位解构 `([a]=b)` 作子表达式、箭头函数解构形参、catch 形参模式、计算键模式 `{[k]:v}`、for-in 变量位模式 (for-of 位已于 12.41 闭合)、简单形参默认 `f(a=1)` (与四轮"箭头非标识符形参"同挂)、嵌套对象剩余 `{...{a}}` (显式 ParseError)。旁支新定案: BC 链 `Object.defineProperty` 整体失效 (g130: AST 42|1 / BC undefined|0；g133 定性: 同宿主 Native 的 `Object.assign` BC 原位变更亦丢只回还返回物) = **12.26 "BC 跨桥深拷贝丢身份/变更"同墙**，归战役 C 值模型唯一正解，不单点打补丁 (逐 Native 回写与一处定义冲突)；性能项 #28 第七轮连续负载封锁 (load 8.58 > 6)，数字不记账

### 12.39 内建面扫面第十五轮: 形参默认值与箭头解构形参 (2026-10-16)

探针 `/tmp/g134p.js` (9 形) + `/tmp/g136p.js`/`/tmp/g137p.js` (arguments/边缘象) 字符级钉 → 三方对照 → 门禁 72。闭合十四轮入口名册的"简单形参默认 `f(a=1)`"与"箭头函数解构形参"两项: **t1–t11 全 11 项三方逐字符对齐** (undefined 触发/null 不触发、fallback 惰性、前序形参引用、arguments 入默认值、箭头默认/解构/混合、抛象两链一致)。

- **简单形参默认折 prelude 复用十四轮机件**: `parse_param_table` 识别 `name = expr` 后落一条 `Stmt::VarPattern { Default{Binding(name), fb}, init: Identifier(name) }`，desugar 成 `var a = a===undefined ? expr : a` —— 实参槽缺省即 undefined (g136 前提钉死)，短路条件保证 fallback 只在触发时求值 (t3: cnt=1)，左到右序 + 前序名可读 (t2: `b=a+1` → NaN 对齐)
- **箭头参数表并入单点**: `parse_arrow_paren_params` 删除 (死代码)，箭头路径直走 `self.skip()` + `parse_param_table`，`parse_arrow_tail` 收 prelude 参在 `new_arrow` 前 `splice_prelude` (new_arrow 委托 new_with_src, hoisted 统计同样先看最终 body)；`(({v})=>v)`/`(([a,b])=>a+b)`/`((x={v=1})=>…)` 全通；`paren_is_arrow_head` 零改动 (解构括号照常配平)
- **链分裂修复 (arguments.length 虚报)**: BC 链 `function q3(a=1){} q3()` 的 `arguments.length` 交 1 / node 0 —— 根因: 默认值降级体 `var a = …` 以**参数寄存器号**回写形参名, `set_register` 的 `frame_param_len.max(...)` 把进帧实参长度撑大 (g136 定案: 仅同形参名回写触发, 解构槽 `$argN` 不触发)。修复走源真相单点: 引擎新增 `frame_arguments_len` (进帧时刻 args.len() 快照, call_function/execute_with_args_and_this 两入口设置 + 随帧保存恢复)，`LdaGlobal` 的 arguments 物化只读该快照, `frame_param_len` 回归寄存器寻址本职。AST 链同探针零扰动 (arguments 直取 evaluated_args)
- **两链一致抛象收钉**: 非可迭代源入数组模式 (`f(1)` 配 `[y=a]` 默认) 与 default 源缺方法 (`z('xy').join`) 两链同抛 (错误文本名册族另档)；`a=a` 自引用默认交 undefined / node TDZ ReferenceError (TDZ 全局缺, 记偏差)

**门禁**: `syntax_corner_cases` **72/72** (新 72: 11 项默认值语义/箭头形参/arguments 长度/惰性求值/抛象, 全 node-EXACT)、`./test.sh` **2961 passed / 1 skipped** (+1 即门禁 72)、`./sh/clippy.sh` exit 0 / 0 warning

- **记偏差 (十五 B 十五轮)**: rest 形参 `f(...a)`/箭头 `(...b)=>…` 两链 ParseError (需收集实参的形参模型, 独立批次)；`function f(a=a)` 自引用默认 undefined / node TDZ 抛 (TDZ 与 let/const≡var 同根)；g136 顺带钉正: **无默认形参时两链 arguments.length 本已对齐** (q1 0|1|2 三方)，本轮修复补齐有默认象

---

### 12.40 内建面扫面第十六轮: rest 形参 (2026-10-16)

探针 `/tmp/g139.js` (10 项) + `/tmp/g140.js` (13 项边缘象) 三方对照首轮即逐字符全等 → 门禁 73。闭合十五轮残档的 rest 形参 `f(...a)` / 箭头 `(...b)=>…` 两链 ParseError: **t1–t16 全 16 项三方对齐** (收集/空尾/尾位判定、`.length` 排除 rest、箭头吃自身调用实参、`arguments` 共存、解构形参后 rest、嵌套闭包)。

- **`rest_param` 元数据单点、零新字节码**: `SharedFunctionInfo.rest_param: Option<(String, usize)>` (绑定名 + 收集起点 = rest 前按位形参数, 名**不占** params 位) —— 由 `parse_param_table` 一处识别 `...r` ( break 后 `expect(RParen)` 天然拒 `...r,`)，两链各自在进帧点解码: AST 链 `ActivationScopeInit.rest_param` 在 `init_activation_scope` 按位形参之后、hoist 之前插 rest 数组快照 (hoist 跳已有键, 保住 `function q(...z){ var z=5; }` sloppy 重decl 口径); BC 链经桥 `FunctionLiteral.rest_param` 传给字节码生成器
- **BC 链寄存器方案**: 生成器在 hoist 前为 rest 名 `allocate_register()` 专用局存 —— 捕获态走 `load_accumulator_with_register + store_global` (与捕获形参同型)，非捕获态 `locals.insert`；`CompiledFunction` 三元组第三项 `(start, reg)` 挂 `InterpFunction.rest_binding`，分发器 `call_function` 进帧时 `reg_stack[new_reg_base+reg] = Value::new_array_with_elements(args[start..])` (get_mut 守卫, 零 panic)。箭头天然正确 (自身调用实参, 与 12.28 词法 `frame_arguments_lex` 快照解耦)、construct/跨桥原生 shim 全部经 `call_function` 单点自动覆盖
- **结构体尺寸根因 (回归教训)**: `FunctionLiteral` 是 AstNode 最大变体，新字段直放 `Option<(String, usize)>` (+32B) 按全体节点数放大 `expr_to_v8_ast_scoped` 递归桥栈帧 → `test_earley_boyer_direct_bytecode` 2MiB 线程栈溢出 (4MiB 过, lldb EXC_BAD_ACCESS 定位)。修法是 `Option<Box<(String, usize)>>` 而非 RUST_MIN_STACK 掩盖 —— **给最大变体加字段一律先想 Box**
- **语法错收钉**: `...r = x` 与 `f(...r, b)` 两链 parse 期 ParseError (node SyntaxError, 文本保真挂"caught error 文本"族)；`.length` 口径 BC 用 `params.len()`、rest 名不在 params 天然排除
- **记偏差 (十六轮)**: `f(a = r, ...r)` 默认值前向读 rest —— rest 绑定先于体存在故读到数组 / node ReferenceError (TDZ 家族, 与 `a=a` 同根, 非链分裂)

**门禁**: `syntax_corner_cases` **73/73** (新 73: 16 项 rest 语义, 全 node-EXACT)、`./test.sh` **2962 passed / 1 skipped** (+1 即门禁 73)、`./sh/clippy.sh` exit 0 / 0 warning (3-tuple 签名以 `ParamTable`/`CompiledFunction` 命名别名消 type_complexity, 非 allow 压制)

### 12.41 内建面扫面第十七轮: for-of 迭代语句 (2026-10-16)

探针 `/tmp/g143.js` (19 项) 三方对照: 17 项 node-EXACT、4 项两链一致挂账 (f2/f11/f12/f15)、1 项链分裂 (f14) —— 链分裂按红线必修, 根因见 12.42。闭合十四轮残档的 "for-of 全缺"。

- **parser desugar 单点、执行链零感知**: 暂态 `Stmt::ForOf { head, src, body }` 只在解析漏斗出口 (`desugar_stmts`) 摊成 `$fsrc/$fidx` 三段式索引经典 For (条件**活读** `$fsrc.length`), 与 node Array Iterator 索引语义对齐 —— 迭代中 push 会被访问 (f9=3)、洞读 undefined (f8)、字符串/arguments 按位读 (f10); 解释器/桥/静态扫描只见普通节点, 桥与解释器对残形显式报错 (与 `VarPattern` 同型防线)
- **头部四形态**: 声明位 `Binding(name)` (含未声明标识符, 同走 `bind(name, elem, true)`)、解构位 `Pattern` (声明走 `parse_pattern`; 赋值式走新增 `looks_like_destructure_for_head` —— 复用十四轮 `scan_balanced_closer` 配平扫描单点, 判 "闭合符后紧跟标识符 of")、成员/下标写位 `Target(Expr)` (`for (o.p of …)`/`for (o[k] of …)`), 其余目标显式 ParseError; 上下文关键字 `of` = `TokenKind::Identifier("of")`, `var of=[7,8]; for (v of of)` 边缘象两链+node 三方全等
- **记偏差 (十七轮)**: f2 `typeof y` 块外可见 (let 块级作用域缺, 11.6 系统性) / f11 `3,3,3` (let≡var 逐轮捕获, 11.6) / f12 f15 非可迭代 RHS 静默零轮 (迭代器协议, 战役 C 家族 —— 十四轮"数组面解构非可迭代"同根)

### 12.42 BC 引擎缺陷修复: var 提升预绑槽 (循环体变量被临时寄存器冲撞, 2026-10-16)

f14 链分裂根因**与 for-of 无关** (纯 ES5 亦复现, g146/g149/g150/g151 阶梯收窄): `var` 声明在循环体内而循环头先编 —— `hoist_declarations` 只递归 Block, 体内 `var w` 延迟到体生成时才 `bind_local`; 此时循环条件已把 `visit_binary` 的临时寄存器 (reg1) 编进字节码并释放回分配器, `w` 复用同号槽 → 每轮回边条件重执行 `Star reg1` 用左值快照踩写 `w` (读回计数器末值)。分派器 `set_register` + opcode 标签探针钉死毒写站点 (`Star1`, 固定偏移在循环头), codegen 侧证明绑定本身正确。

- **修法 (一处定义)**: `hoist_declarations` 非顶层路径先 `collect_local_defs` 全作用域收名 (含 For/While/If/Switch/Try 体, 复用既有遍历函数)、排序保槽号确定可复现、逐个 `bind_local` 预绑 —— 局部位居分配流最前, 临时寄存器不可能再被晚绑 var 复用; rest 形参寄存器在预绑前分配 (与 `rest_binding` 记录一致), 形参/重名经 `locals` 缓存天然幂等, 捕获名 `bind_local` 返回 None 不入寄存器
- 顶层不受影响 (is_top_level 名字全走全局表, 预绑显式跳过); 门禁 74 以 f18 = 纯 ES5 `for(;c<3;){w=c*100;c=c+1;}return w` → 200 直接收钉回归

**门禁**: `syntax_corner_cases` **74/74** (新 74: 19 项 for-of, node-EXACT 与两链一致挂账分列注释)、`./test.sh` **2963 passed / 1 skipped** (+1 即门禁 74)、`./sh/clippy.sh` exit 0 / 0 warning

### 12.43 BC 引擎性能: 分发环热 helper 强内联 + Call 实参移动化 (2026-10-17)

12.10 profile 续项: 分发环热 helper 调用开销 + Call 臂实参双重引用计数抖动 (采样 clone/drop ≈33%)。

- **内联轮**: `get_register_ref`/`get_register`/`name_constant`/操作数解码族 → `#[inline(always)]`; `lookup_accessor`、`check_property_receiver` 拆出冷支 outline、热支内联; `nm` 复核剩余非内联符号均为刻意 outline
- **移动实参轮 (一处定义)**: 新增 `call_function_moved(callable, receiver, &mut args)` 移动核, 原 `call_function` 降为借用 wrapper (`to_vec`+clone 转调) 兼容全部旧调用点; `construct`/`construct_moved` 同型; 热臂 (GetNamed/GetKeyed accessor、SetNamed/SetKeyed setter、CallProperty0/1/2、CallProperty/CallUndefinedReceiver、Construct) 全切实版 —— 实参经 `mem::take` 逐槽空进 `param_stack`, 消掉每参一次 refcount 往返; FUNCTION_CALL/FUNCTION_APPLY 拦截臂 `take(&mut args[0])` + `tail_start` 递归; **rest 形参快照必须先于 args 被 take 移空**
- **实测** (min-of-5, `date +%s%N` 挂钟, load<6 窗口, head/inline/ownedcall 三快照交错; 单位 s, oct4_all / rich_long):

  | 构建 | oct4_all | rich_long |
  |---|---|---|
  | head | 31.810 | 9.472 |
  | inline | 25.920 (**−18.5%**) | 8.244 (**−13.0%**) |
  | ownedcall | 25.485 (−1.7%) | 8.196 (−0.6%) |

  ownedcall 增量落在轮间噪声带内 (head 散布 ≈1.5%, oct4 上 inline/ownedcall 散布 4.9%/7.8%), 但两 workload 最小值同向且 A/C 输出对拍全等 (micro×8 + rich_long 逐字节同, oct4 归一化一致), 保留不回滚
- 快照构建配方: `CARGO_TARGET_DIR=target/prof2 CARGO_PROFILE_RELEASE_DEBUG=2 CARGO_PROFILE_RELEASE_STRIP=false cargo build --release -p rs8` —— profile release 自带 `strip=true`, 不显式关就没有符号
- 已驳回项维持 (勿重试): Rc<RefCell> 对象模型、操作数窗口预取、标量 clone 强制内联
- 死代码扫描: 孤儿探测 186 行与 D2 基线一致, 本轮改动零新增孤儿

**门禁**: `./test.sh` **2963 passed / 1 skipped**、`syntax_corner_cases` **74/74**、`v8-interpreter` **33/33**、`./sh/clippy.sh` exit 0 / 0 warning (需 `PATH="$HOME/.cargo/bin:$PATH"`, Homebrew cargo 遮蔽 rustup 代理)

### 12.44 标记语句两链落地: `L: for` / 标记 break-continue / 标记块·if·switch (2026-10-17)

上游 ./v8 在 **parser 解析期**把 `break/continue.target()` 解析成 Breakable/IterationStatement 指针, 无 LabelledStatement 节点; 我们语法树只到语句串, 等价改为 **解析期按名校验 + 执行期按名匹配** (String 标签替代指针, 校验时机与错误面同原版)。node 语义实测钉死: `break L` 可指任意带标签语句 (循环/block/if/switch)、`continue L` 仅迭代语句、未定义标签与越界裸 break/continue = parse-time SyntaxError、`A: B: for` 双标签同指一循环、裸 break **穿透**标记块找最近循环、函数体是词法边界 (`(function(){ break outer })()` 报 Undefined label)。

- **parser 单点** (`rs8/parser.rs`): `parse_statement` 包一层 (标记识别 + `loop_depth/switch_depth` 圈层计数, 体解析整段覆盖故 For/ForIn/ForOf 子形态零扰动), 连续 `ident:` 收拢成**一条** `Stmt::Labeled{labels, body}`; 标签是否指迭代语句按冒号链后头记号预记 (continue L 校验先于体解析可见); `take_label` 同行限制 (原版 [no LineTerminator here]); 函数体三入口 (decl/expr/arrow) 统一走 `parse_function_block` 屏障存删上下文
- **desugar 联合臂**: `Labeled × for-of` —— 展开产 `[fsrc导言, fidx导言, For]` 时标签只落尾部 For、导言留同层前 (若 splat 包 Block 则 continue L 找不到循环目标); 其余 splat 语义不变
- **AST 链** (`interpreter.rs`): `ExecSignal::Break/Continue(Option<String>)` 带名; 四循环臂抽成 `eval_while/do_while/for/for_in(labels)` 方法 (裸臂传 `&[]`), 按名匹配才消费、不匹配合规外传; `eval_labeled` 体是循环交循环臂、否则跑完吞指自己的 `break L` (= 语句末尾); switch 臂只吞 `Break(None)`
- **BC 链**: v8-ast 七类可标记节点 (三循环+for-in/block/if/switch) 加 `labels: Vec<String>`、Break/ContinueStatement 加 `label: Option<String>` —— **构造仍全收敛 factory** (新字段缺省空, 签名零改动, 波及面勘察定案后一次通过); 桥接 `attach_labels` 单点附加 (死标签如 `L: var` 静默丢, 体内不可能出现跳), `is_var` for-in 包 `Block[decl,forin]` 时标签挂内层环; 生成器 `LoopScope{+labels,+bare_breakable}` —— 标记 block/if 推 `continue_target: None` 伪层 (裸 break 不停靠、continue L 天然只落循环层), `visit_break/continue` 按名 `rposition` 走栈, 穿出 try-finally 复用 `jump_to` 续体链 (门禁 outer23 实证 `t1#f1` 收尾序)
- **记偏差**: `break` 后换行接标识符 node 报 "Illegal newline after break", 我们按 ASI 断句成裸 break + 后续语句 (边缘形态, 门禁不含); **既有挂账暴露**: AST 链普通对象 for-in 枚举序按哈希 (BC 链插入序), 门禁 L20/L21 段以 `sort()` 免疫 —— 属 11.x 对象键序系统性缺口, 非本轮引入

**门禁**: `syntax_corner_cases` **76/76** (新 75 正路 41 段 node-EXACT 串; 新 76 错误面 7 条措辞逐对齐 `Undefined label 'x'` / `Illegal continue statement: 'L' does not denote an iteration statement` 等)、`./test.sh` **2965 passed / 1 skipped** (+2 即门禁 75/76)、`v8-interpreter` **33/33**、`./sh/clippy.sh` exit 0 / 0 warning

### 12.45 寄存器搬移窥孔: Ldar+ShortStar 原地折 Mov (第十九轮, 2026-10-17)

#28/12.10 profile 续: 符号化画像 (load 3.57 窗口只采份额) `run_from` self 55.39% + `drop_glue::<Value>` 16.19% + `Value::clone` 13.41% —— clone/drop ≈30% 全部 caller=run_from。临时原子直方图 (oct4_all 全程, TOTAL_DISPATCH 3,198,016,796 ≈9.4ns/条, 与 #28 的 11-14ns 同口径): **Ldar 24.71% 头名**, Star8 6.83% / LdaSmi 6.91% / Star7 6.42% / Star9 5.99% / Add 5.27%; bigram 头名 Star8→Ldar 6.76% —— `Ldar+ShortStar*` 寄存器搬移占全部执行单元 ~55% 头名集团, 即"值经累加器中转"的搬移本身是最大可执行单元来源。

**做法 (生成侧收口, 零新操作码, 非臂上补丁)**: `build()` 单点 (唯一收口, 脚本/函数两调用点) 落 `reduce_register_moves` —— 把相邻 `Ldar rX(窄 2B); ShortStar rY(1B)` 就地折成 ISA 既有窄 `Mov rX,rY(3B)`: **等长原地改写**, 字节偏移全不变, 相对跳转操作数与 handler 区间零重定位; 每折一对省一次分发与一对累加器 clone/drop (值从寄存器直搬寄存器, 不再绕 acc)。安全性三条件缺一不可:

1. 刻度与长度匹配 (Ldar 必须单刻度 2B + ShortStar 1B);
2. 中间边界 (Star 落点) 不在任何跳转/switch 目标或 handler start/end/target 里 —— 否则执行流可从被折指令中部进入; 解码不出跳转目标则整函数不折;
3. Star 之后 acc 已死 —— 在**原始指令流**后继图 BFS (预算 64): 任何先**读** acc 的指令拦折, **覆写** acc 终止该路径, 走满数组尾 (run_from 以 acc 作返回值) 拦折; 链式折对经归纳论证互斥安全 (被折 Ldar 的"覆写终止"性由链尾未折对或流尾兜底)。

**教训 (寄存器操作数编码)**: 字节 = `Register::to_operand() = -4 - index` (receiver=252, arg0=253); ShortStar 的寄存器号在 opcode 里 (`from_short_star(bc).index() = Star0(215) - bc`)。首版把 dst 写成裸 `index() as u8`, `from_operand` 解成假想参数槽 → r0 从未被写 → `new F` 报 `TypeError: Cannot set properties of undefined (setting 'name')`; 经 RS8_PEEP_DEBUG 反汇编 dump 折前/折后字节流比对定位, 修回 `.to_operand()`。**勿再用手写寄存器字节**。

**实测** (同二进制 `RS8_B_NOLF=1` 门控 A/B, 窗口起点 load 2.37, ABBA 交错 + A/A 对照, min, 单位 ms):

| 负载 | 无折 | 有折 | Δ | 配对 |
|---|---|---|---|---|
| add1 | 191 | 180 | **−5.8%** | 折胜 5/5 |
| add5 | 432 | 402 | **−6.9%** | 折胜 5/5 |
| oct4_all | 25787 | 25454 | −1.3% (A/A 散布 7% 内) | 折胜 3/3 |
| rich_long | 8325 | 7799 | −6.3% | 折胜 3/3 |

执行单元数: TOTAL_DISPATCH 3,198,016,796 → **2,754,506,508 (−13.9%)**, 折出的 Mov 占 16.86% 执行 (Ldar 790M→330M)。D(折) vs C(12.43 ownedcall 快照) oct4 25558/25706 min 无回退。输出对拍: rich_long/add1/add5 折与不折逐字节全等; oct4 差异仅其内嵌计时数 (A/A 同抖), 数值归一后全等。**#28 准入本轮达成**: 同负载窗口 add1/add5 单位成本下降且 `test.sh` 全绿。临时门与 `env::var_os` 导入已删, 折叠无条件生效。

**门禁**: `./test.sh` **2965 passed / 1 skipped**、`syntax_corner_cases` **76/76**、`v8-interpreter` **33/33**、`./sh/clippy.sh` exit 0 / 0 warning。残账: clone/drop ≈30% 的**正解仍是战役 C 值模型合一** (窥孔只减了搬移对, 没动值模型); 直方图/bigram 插桩已全部移除。

