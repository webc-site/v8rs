## Benchmarks

Full suite covering all 8 canonical workloads against official C++ V8 live benchmark:

> **Environment**: Apple M2 Max (12 Cores) | macOS 27.0.0 (arm64) | Native C++ V8 v14.6.202.34-node.28

| Benchmark (v8-v7) | Workload | V8RS | Native C++ V8 | Delta | Latency (V8RS) |
| :--- | :--- | :---: | :---: | :---: | :---: |
| **Richards** | OS kernel task dispatching | **106** | 52,667 | -99.8% | 33438.54 µs |
| **DeltaBlue** | Incremental constraint solver | **142** | 136,058 | -99.9% | 46471.83 µs |
| **Crypto** | SHA-1/MD5 cryptographic loops | **24** | 63,519 | -100.0% | 1111015.96 µs |
| **RayTrace** | 3D ray tracer & floating-point | **426** | 146,666 | -99.7% | 173528.83 µs |
| **EarleyBoyer** | Earley parser & Boyer rewrite | **122** | 118,431 | -99.9% | 545935.42 µs |
| **RegExp** | Irregexp finite-state matching | **70** | 15,879 | -99.6% | 1306820.67 µs |
| **Splay** | Splay tree & high-frequency alloc | **9** | 65,315 | -100.0% | 912530.33 µs |
| **NavierStokes** | 2D fluid dynamics PDE solver | **117** | 41,849 | -99.7% | 1263185.75 µs |

### Key Highlights

- **Startup Latency**: **5.70 ms** vs Native V8 **33.64 ms** (**5.9x faster**, **83.1% lower latency**)
- **Overall Score**: **78** vs Native V8 **65,605** (-99.9%)
- **Binary Footprint**: **1.9 MB** vs Native V8 **32.0 MB** (**94.1% reduction**)
