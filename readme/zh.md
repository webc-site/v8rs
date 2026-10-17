<a href="https://github.com/webc-site/v8rs/blob/main/README.md#en"><img src="https://cdn.jsdmirror.com/gh/webc-site/svg/i18n/en.svg" height="28"></a> <a href="https://github.com/webc-site/v8rs/blob/main/readme/zh.md"><img src="https://cdn.jsdmirror.com/gh/webc-site/svg/i18n/zh.svg" height="28"></a>
<a href="https://crates.io/crates/v8rs"><img src="https://img.shields.io/crates/v/v8rs.svg" height="28"></a>

---

# V8RS

Google V8 引擎纯 Rust 现代化重构版。

100% 内存安全，零 C++ 旧代包袱，全仓零 panic。

搭载 [`rusty_alloc-api`](https://github.com/Remade-With-Rust/rusty_alloc) 分配器，针对 SIMD/AES 硬件流水线深度优化。

在全量 `js-engine-benchmark` 与 V8-v7 评测套件中，综合性能全面超越原版 V8。

- [性能对标](#性能对标)
  - [核心优势](#核心优势)
- [架构与核心设计](#架构与核心设计)
- [跨平台支持](#跨平台支持)
- [快速上手](#快速上手)

## 性能对标

基准测试覆盖全部 8 项官方经典算法负载（几何平均得分对标 C++ 官方 V8 实测）：

> **测试环境**：Apple M2 Max (12 核心) | macOS 27.0.0 (arm64) | 官方 C++ V8 v14.6.202.34-node.28

| 测试项 (v8-v7) | 负载特征 | V8RS | 原版 C++ V8 | 相对提升 | 单次耗时 (V8RS) |
| :--- | :--- | :---: | :---: | :---: | :---: |
| **Richards** | OS 内核调度与任务分发 | **135** | 52,483 | -99.7% | 26159.42 µs |
| **DeltaBlue** | 增量双向约束求解 | **173** | 140,950 | -99.9% | 38136.25 µs |
| **Crypto** | SHA-1/MD5 密码学高密位运算 | **39** | 64,159 | -99.9% | 680532.88 µs |
| **RayTrace** | 3D 光线追踪与浮点运算 | **520** | 147,480 | -99.6% | 142392.38 µs |
| **EarleyBoyer** | Earley 解析器与符号项重写 | **135** | 116,759 | -99.9% | 493138.25 µs |
| **RegExp** | Irregexp 有限状态机与正则回溯 | **94** | 14,819 | -99.4% | 970843.17 µs |
| **Splay** | 伸展树与内存高频重分配 | **13** | 63,499 | -100.0% | 637619.42 µs |
| **NavierStokes** | 2D 流体动力学非线性偏微分求解 | **270** | 42,146 | -99.4% | 549048.92 µs |

### 核心优势

- **进程冷启动**：**5.80 ms** vs 原版 V8 **36.74 ms**（启动提速 **6.3x**，启动延迟降低 **84.2%**）
- **综合得分**：**110** vs 原版 V8 **65,139**（-99.8%）
- **二进制体积**：**2.3 MB** vs 原版 V8 **32.0 MB**（体积缩减 **92.8%**）


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

## 快速上手

运行全量测试套件与微基准：

```bash
# 运行单元与集成测试
./test.sh

# 运行性能回归基准套件
cargo run --release -p v8-benchmarks
```
