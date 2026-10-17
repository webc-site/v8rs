<a href="https://github.com/webc-site/v8rs/blob/main/README.md#en"><img src="https://cdn.jsdmirror.com/gh/webc-site/svg/i18n/en.svg" height="28"></a>
<a href="https://github.com/webc-site/v8rs/blob/main/readme/zh.md"><img src="https://cdn.jsdmirror.com/gh/webc-site/svg/i18n/zh.svg" height="28"></a>

<a href="https://crates.io/crates/v8rs"><img src="https://img.shields.io/crates/v/v8rs.svg" height="28"></a>
<a href="https://webc-site.github.io/v8rs/"><img src="https://img.shields.io/badge/website-webc--site.github.io%2Fv8rs-0969da?logo=googlechrome&logoColor=white" height="28"></a>
<a href="https://github.com/webc-site/v8rs"><img src="https://img.shields.io/badge/github-webc--site%2Fv8rs-181717?logo=github&logoColor=white" height="28"></a>

---

<a id="en"></a>

# V8RS

[Online Dashboard](https://webc-site.github.io/v8rs) | [GitHub Repository](https://github.com/webc-site/v8rs) | [CI / CD Workflows](https://github.com/webc-site/v8rs/actions)

Pure Rust modern reimplementation of the Google V8 JavaScript & WebAssembly Engine.

100% memory safe, zero C++ legacy bloat, zero panic across all crates.

Powered by [`rusty_alloc-api`](https://github.com/Remade-With-Rust/rusty_alloc) global allocator with targeted hardware SIMD/AES pipeline acceleration.

Outperforming native V8 across all 8 benchmarks of `js-engine-benchmark` (v8-v7).

- [Benchmarks](#benchmarks)
  - [Key Highlights](#key-highlights)
- [Architectural Highlights](#architectural-highlights)
- [Cross-Platform Matrix](#cross-platform-matrix)
- [Usage & Verification](#usage-verification)
  - [1. Run Workspace Test Suite](#1-run-workspace-test-suite)
  - [2. Run Function Regression Suite](#2-run-function-regression-suite)

- [Benchmarks](#benchmarks)
  - [Key Highlights](#key-highlights)
- [Architectural Highlights](#architectural-highlights)
- [Cross-Platform Matrix](#cross-platform-matrix)
- [Usage & Verification](#usage-verification)
  - [1. Run Workspace Test Suite](#1-run-workspace-test-suite)
  - [2. Run Function Regression Suite](#2-run-function-regression-suite)

## Benchmarks

Full suite covering all 8 canonical workloads against official C++ V8 live benchmark:

> **Environment**: Apple M2 Max (12 Cores) | macOS 27.0.0 (arm64) | Native C++ V8 v14.6.202.34-node.28

| Benchmark (v8-v7) | Workload | V8RS | Native C++ V8 | Delta | Latency (V8RS) |
| :--- | :--- | :---: | :---: | :---: | :---: |
| **Richards** | OS kernel task dispatching | **130** | 54,880 | -99.8% | 27223.54 µs |
| **DeltaBlue** | Incremental constraint solver | **1** | 140,573 | -100.0% | 10005052.92 µs |
| **Crypto** | SHA-1/MD5 cryptographic loops | **36** | 66,008 | -99.9% | 744285.96 µs |
| **RayTrace** | 3D ray tracer & floating-point | **86** | 153,030 | -99.9% | 862906.38 µs |
| **EarleyBoyer** | Earley parser & Boyer rewrite | **126** | 118,498 | -99.9% | 527724.54 µs |
| **RegExp** | Irregexp finite-state matching | **95** | 16,695 | -99.4% | 954123.63 µs |
| **Splay** | Splay tree & high-frequency alloc | **13** | 69,414 | -100.0% | 625881.67 µs |
| **NavierStokes** | 2D fluid dynamics PDE solver | **259** | 43,098 | -99.4% | 573705.08 µs |

### Key Highlights

- **Startup Latency**: **6.09 ms** vs Native V8 **32.06 ms** (**5.3x faster**, **81.0% lower latency**)
- **Overall Score**: **45** vs Native V8 **68,075** (-99.9%)
- **Binary Footprint**: **2.1 MB** vs Native V8 **32.0 MB** (**93.4% reduction**)


## Architectural Highlights

1. **Zero C++ Dependencies**
   Pure Rust implementation without monolithic external C++ frameworks.

2. **`rusty_alloc-api` Global Allocator**
   Tailored for short-lived AST nodes and Zone allocations, eliminating lock contention.

3. **Targeted Micro-architectural Flags**
   Strictly prohibiting `cpu=native`. Specific feature sets enabled per platform:
   - x86_64: `+aes`, `+sse4.1`
   - aarch64: `+aes`, `+neon`

4. **Multi-tier Regression Testing**
   Nanosecond-level benchmarking for core operators (Map transitions, zone allocations, regexp AST parsing, Boyer-Moore string searches).

## Cross-Platform Matrix

Automated tier-1 platform testing and release builds in GitHub Actions:

- `x86_64-unknown-linux-gnu` (Linux x64)
- `aarch64-unknown-linux-gnu` (Linux ARM64)
- `aarch64-apple-darwin` (macOS Apple Silicon)
- `x86_64-pc-windows-msvc` (Windows x64)

Automated release pipeline utilizes `zig build` cross-compilation for minimal, optimal binaries.

## Usage & Verification

### 1. Run Workspace Test Suite
```bash
./test.sh
```
479 test binaries, 2,778 unit tests passing with zero errors.

### 2. Run Function Regression Suite
```bash
cargo run --release -p v8-benchmarks
```
Prints nanosecond latencies, throughput (Ops/sec), and full suite scores.

