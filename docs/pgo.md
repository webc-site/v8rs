# 配置文件引导优化

本文档说明如何使用 cargo-pgo 工具，结合真实工作负载对 V8RS 运行时与基准套件进行配置文件引导优化（Profile-Guided Optimization）。

## 核心原理

配置文件引导优化是一种由编译器在底层利用程序真实运行时的执行轨迹（调用频率、分支跳转几率、热点循环等）指导代码生成的优化技术。

标准流程包含三个阶段：
1. 插桩构建：编译器在生成的二进制代码中嵌入轻量探针与计数器。
2. 负载训练：执行具备代表性的基准用例，探针将热点执行轨迹写入原始数据文件（.profraw）。
3. 优化重编：合并分析训练数据生成 .profdata，编译器根据真实热点实施精准内联、冷热块重排、寄存器优先分配以及分支预测。

在密集运算与复杂解释执行场景下，PGO 通常能带来 10% 至 25% 的纯编译器级性能提速。

## 环境要求

系统需具备：
- cargo-pgo
- llvm-profdata（已随 Rust 工具链内置）

验证命令：
```bash
cargo pgo info
```

输出显示 llvm-profdata 路径即可正常使用。

## 标准优化流程

### 第一步：插桩构建

使用 cargo pgo 对目标二进制进行插桩编译：

```bash
cargo pgo build -- -p v8-benchmarks --release
```

该步骤将在目标目录生成插桩版本的 release 可执行文件。

### 第二步：运行真实基准采集轨迹

直接运行插桩生成的二进制文件，跑通全部官方基准测试套件采集热点画像：

```bash
./target/release/v8-benchmarks
```

执行期间，LLVM 运行时会自动在项目临时目录写入 .profraw 数据。

### 第三步：基于画像执行终极优化编译

使用采集到的执行画像重编二进制：

```bash
cargo pgo optimize -- -p v8-benchmarks --release
```

该命令自动调用 llvm-profdata merge 合并剖析数据，并注入 -Cprofile-use 参数由 LLVM 执行终极优化代码生成。

### 第四步：清理临时插桩数据

在完成发布或需要重新训练时，清理生成的 profile 缓存：

```bash
cargo pgo clean
```

## 工程化一键脚本

仓库提供 tool/pgo_bench.sh 实现自动化全流程训练与基准产出：

```bash
./tool/pgo_bench.sh
```

脚本将全自动串联插桩、训练、优化编译与最终跑分评估。
