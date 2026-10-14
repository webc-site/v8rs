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
| **Richards** | OS kernel task dispatching | **108** | 54,076 | -99.8% | 32823.21 µs |
| **DeltaBlue** | Incremental constraint solver | **146** | 132,064 | -99.9% | 45381.50 µs |
| **Crypto** | SHA-1/MD5 cryptographic loops | **25** | 40,904 | -99.9% | 1056893.92 µs |
| **RayTrace** | 3D ray tracer & floating-point | **433** | 118,176 | -99.6% | 170738.33 µs |
| **EarleyBoyer** | Earley parser & Boyer rewrite | **130** | 115,127 | -99.9% | 511266.79 µs |
| **RegExp** | Irregexp finite-state matching | **88** | 15,863 | -99.4% | 1036652.17 µs |
| **Splay** | Splay tree & high-frequency alloc | **9** | 65,543 | -100.0% | 858852.46 µs |
| **NavierStokes** | 2D fluid dynamics PDE solver | **122** | 42,061 | -99.7% | 1219132.79 µs |

### Key Highlights

- **Startup Latency**: **6.19 ms** vs Native V8 **33.93 ms** (**5.5x faster**, **81.8% lower latency**)
- **Overall Score**: **82** vs Native V8 **60,257** (-99.9%)
- **Binary Footprint**: **1.9 MB** vs Native V8 **32.0 MB** (**94.1% reduction**)


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

