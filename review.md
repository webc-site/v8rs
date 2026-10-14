# 独立审查子代理执行规范与反编译核查指南

本文档定义审查子代理（Reviewer / Reverify Auditor）的标准作业规范、反编译与物证核查技术，以及自动化排查流程。

---

## 核心准则

1. 工具裁决，物证说话
   严禁仅凭静态 diff 或主观臆断下结论。每条审查结论必须附带机器可复现的物证：
   - 原版 C++ 路径与行号对照（如 `v8/src/ast/scopes.cc:123`）
   - Rust 源码路径与行号（如 `rs8/src/runtime/interpreter.rs:456`）
   - 命令行执行输出、测试用例或反编译符号输出

2. 冻结判定不可与修复同源
   审查子代理与开发子代理必须在完全独立的会话中派生，切断上下文记忆共振，杜绝盲区共振导致的伪冻结。

3. 会动的审查
   严禁纸上谈兵。审查过程必须真实执行编译、运行测试、采集性能画像、检索符号。

4. 杜绝假实现与孤岛代码
   重点排查代码虽已转写但未接入实际执行链路、或运行时静默降级回退的虚假实现。

---

## 四阶工具核查技术

### 第一阶：依赖拓扑穿透审查

目的：识别代码库中被虚假声明、或者在编译链路中实际被完全孤立的 crate。

核查方法：
1. 检查目标二进制是否真正依赖核心优化库：
   ```bash
   cargo tree -p v8-benchmarks | grep -E "v8-ic|v8-objects-maps|v8-interpreter"
   ```
2. 检索某优化模块的全仓反向依赖：
   ```bash
   cargo tree --workspace -i v8-ic
   ```
   若反向依赖仅有自身，判定为死代码孤岛。

自动化脚本：
```bash
./tool/reverify_deps.sh
```

---

### 第二阶：反编译与符号核验

目的：检查经过编译器激进优化、LTO 与死代码消除后，目标优化函数与结构体是否真实存在于最终二进制中。

核查方法：
1. 符号反查与命名空间过滤：
   ```bash
   nm -C /tmp/rust/release/v8-benchmarks | grep -E "v8_ic|v8_objects_maps|BytecodeDispatcher"
   ```
   若输出为空，证明该模块在最终可执行文件中被完全裁减，未参与实际运行。
2. 字符串与特征常数扫描：
   ```bash
   strings /tmp/rust/release/v8-benchmarks | grep -E "BytecodeDispatcher|FeedbackVector"
   ```

自动化脚本：
```bash
./tool/reverify_symbols.sh /tmp/rust/release/v8-benchmarks
```

---

### 第三阶：执行路径与降级退化排查

目的：防止桥接层或虚拟机在面对复杂语法时静默返回 None、回退为极慢的后备路径。

核查关键点：
1. 桥接器白名单完整性：
   检查抽象语法树转写模块（如 `rs8/src/runtime/bytecode_bridge.rs`）。
   检查 `match expr` 与 `match stmt` 是否存在大量的 `_ => None` 通配降级分支。
2. 调度器指令覆盖：
   检查分派循环（如 `v8-interpreter/src/bytecode_dispatcher.rs`）中是否存在未实现而仅做 `it.advance()` 的空分支。
3. 动态断言防护：
   在基准用例执行入口处加入日志断言，记录是否触发了解释器降级逻辑。

---

### 第四阶：性能采样与叶子热点定位

目的：用纳秒级采样数据定位瓶颈，拒绝盲目重构。

核查方法：
1. 在 release 构建中保留调试符号（可通过环境变量或临时配置，不修改 Cargo.toml）：
   ```bash
   cargo build -p v8-benchmarks --release --config 'profile.release.strip=false' --config 'profile.release.debug=1'
   ```
2. 启动基准测试并使用系统采样器捕获运行堆栈：
   ```bash
   /tmp/rust/release/v8-benchmarks & PID=$!
   sample $PID 2 -file /tmp/bench.sample
   wait $PID
   ```
3. 过滤并统计耗时最高的叶子函数：
   ```bash
   cat /tmp/bench.sample | rustfilt | sed -E 's/^[ \t\+\!\:\*\|]+//' | grep '^[0-9]' | awk '{print $1, $2}' | sort -k2 | awk '{count[$2]+=$1} END {for (fn in count) print count[fn], fn}' | sort -n -r | head -n 30
   ```

自动化脚本：
```bash
./tool/profile_sample.sh
```

---

## 经典语言与架构陷阱排查清单

审查子代理在审查运行时实现时，必须逐项核对以下高频缺陷：

### 1. 浮点转整型饱和截断陷阱
- 现象：Rust 原生 `f64 as i32` 遇大数（如 `4025962439.0`、`4294967295.0`）会饱和截断为 `i32::MAX`（2147483647），导致伪随机数卡死或位运算错乱。
- 规范：必须统一使用 `v8_numbers::{double_to_int32, double_to_uint32}`，严格对标 ECMA-262 模 2^32 补码回绕标准。
- 审查要求：全仓严禁直接出现裸 `as i32`、`as u32` 承接浮点数位运算。

### 2. 作用域链中的锁与堆分配
- 现象：在循环（`Stmt::For`、`Stmt::While`）中无条件创建 `Environment::new_child`，造成每秒数十万次锁争用与哈希表分配。
- 规范：在 ES3/ES5 语义下，`var` 声明属于函数级作用域，循环结构直接复用当前环境。
- 审查要求：检查函数调用是否在创建期完成形参和局部变量提升预计算，禁止每次调用深度递归全遍历语法树。

### 3. 内建对象方法的临时闭包分配
- 现象：在 `get_property` 中访问 `push`、`pop`、`slice` 等内置方法时，每次都临时构造 `Arc::new(move |args| ...)` 闭包。
- 规范：对标 V8 内建方法内联机制（Builtin Inlining），在调用侧识别接收者类型并直接内联就地求值。

---

## 门禁验收标准

审查子代理判定通过前，必须确认以下门禁全绿：

1. 格式化检查：`cargo +nightly fmt --all --check` 0 差异。
2. 静态检查：`./sh/clippy.sh` 0 警告、0 错误，严禁使用 `#[allow(...)]` 绕过。
3. 全量测试：`./test.sh` 全部通过（当前测试基线 2882+ 项全过，0 失败）。
4. 基准无退化：`cargo run -p v8-benchmarks --release` 验证 8 大用例正常跑通，综合几何得分稳步提升。
