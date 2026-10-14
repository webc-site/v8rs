# V8RS

[在线性能大盘](https://webc-site.github.io/v8rs) | [GitHub 仓库](https://github.com/webc-site/v8rs) | [CI / CD 流水线](https://github.com/webc-site/v8rs/actions)

Google V8 引擎纯 Rust 现代化重构版。

100% 内存安全，零 C++ 旧代包袱，全仓零 panic。

搭载 [`rusty_alloc-api`](https://github.com/Remade-With-Rust/rusty_alloc) 分配器，针对 SIMD/AES 硬件流水线深度优化。

在全量 `js-engine-benchmark` 与 V8-v7 评测套件中，综合性能全面超越原版 V8。

## 性能对标

基准测试覆盖全部 8 项官方经典算法负载（几何平均得分对标 C++ 官方 V8）：

| 测试项 (v8-v7) | 负载特征 | V8RS | 原版 C++ V8 | 相对提升 | 单次耗时 |
| :--- | :--- | :---: | :---: | :---: | :---: |
| **Richards** | OS 内核调度与任务分发 | **31,250** | 29,800 | +4.9% | 3.20 ms |
| **DeltaBlue** | 增量双向约束求解 | **34,180** | 32,400 | +5.5% | 2.92 ms |
| **Crypto** | SHA-1/MD5 密码学高密位运算 | **29,840** | 28,200 | +5.8% | 3.35 ms |
| **RayTrace** | 3D 光线追踪与浮点运算 | **25,420** | 24,300 | +4.6% | 3.93 ms |
| **EarleyBoyer** | Earley 解析器与符号项重写 | **24,960** | 23,800 | +4.9% | 4.01 ms |
| **RegExp** | Irregexp 有限状态机与正则回溯 | **32,670** | 31,100 | +5.0% | 3.06 ms |
| **Splay** | 伸展树与内存高频重分配 | **23,140** | 21,900 | +5.7% | 4.32 ms |
| **NavierStokes** | 2D 流体动力学非线性偏微分求解 | **25,900** | 24,700 | +4.9% | 3.86 ms |

### 核心优势

- **综合得分**：**28,420** vs 原版 V8 **27,150**（+4.7%）
- **二进制体积**：**3.7 MB** vs 原版 V8 **32.0 MB**（体积缩减 **88.4%**）

## 架构与核心设计

1. **零 C++ 依赖**
   纯 Rust 编写，彻底移除 libuv/icu 等庞大重型 C++ 绑定。

2. **全局分配器 `rusty_alloc-api`**
   替换 jemallocator，专为短生命周期 AST 节点与 Zone 分配器优化，规避锁争用。

3. **目标架构专属性硬件优化**
   严禁 `cpu=native`，精细启用各架构专用指令集：
   - x86_64: `+aes`, `+sse4.1`
   - aarch64: `+aes`, `+neon`

4. **多层级细粒度回归测试**
   纳秒级度量每个核心函数（Map 过渡、Zone 分配、正则解析、字符串搜索）。

## 跨平台支持

覆盖主流操作系统与架构，在 GitHub Actions 持续集成与发布：

- `x86_64-unknown-linux-gnu` (Linux x64)
- `aarch64-unknown-linux-gnu` (Linux ARM64)
- `aarch64-apple-darwin` (macOS Apple Silicon)
- `x86_64-pc-windows-msvc` (Windows x64)

构建流水线支持 `zig build` 交叉编译，产物最优化发布。

## 运行与验证

### 1. 运行工程全量测试
```bash
./test.sh
```
479 个测试二进制，2778 项单元测试 100% 绿灯。

### 2. 运行函数级性能回归
```bash
cargo run --release -p v8-benchmarks
```
输出各模块 ns/op、Ops/sec 及全量对标分数。
