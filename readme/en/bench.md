## Benchmarks

Full suite covering all 8 canonical workloads against official C++ V8 live benchmark:

> **Environment**: Apple M2 Max (12 Cores) | macOS 27.0.0 (arm64) | Native C++ V8 v14.6.202.34-node.28

| Benchmark (v8-v7) | Workload | V8RS | Native C++ V8 | Delta | Latency (V8RS) |
| :--- | :--- | :---: | :---: | :---: | :---: |
| **Richards** | OS kernel task dispatching | **135** | 52,483 | -99.7% | 26159.42 µs |
| **DeltaBlue** | Incremental constraint solver | **173** | 140,950 | -99.9% | 38136.25 µs |
| **Crypto** | SHA-1/MD5 cryptographic loops | **39** | 64,159 | -99.9% | 680532.88 µs |
| **RayTrace** | 3D ray tracer & floating-point | **520** | 147,480 | -99.6% | 142392.38 µs |
| **EarleyBoyer** | Earley parser & Boyer rewrite | **135** | 116,759 | -99.9% | 493138.25 µs |
| **RegExp** | Irregexp finite-state matching | **94** | 14,819 | -99.4% | 970843.17 µs |
| **Splay** | Splay tree & high-frequency alloc | **13** | 63,499 | -100.0% | 637619.42 µs |
| **NavierStokes** | 2D fluid dynamics PDE solver | **270** | 42,146 | -99.4% | 549048.92 µs |

### Key Highlights

- **Startup Latency**: **5.80 ms** vs Native V8 **36.74 ms** (**6.3x faster**, **84.2% lower latency**)
- **Overall Score**: **110** vs Native V8 **65,139** (-99.8%)
- **Binary Footprint**: **2.3 MB** vs Native V8 **32.0 MB** (**92.8% reduction**)
