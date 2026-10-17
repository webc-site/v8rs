[English](#en) | [中文](#zh)

---

<a name="en"></a>

# v8-diagnostics

Rust transcription of `v8/src/diagnostics` — wave1 zero-coupling subset (platform-independent pure logic).
Data surface is generated from upstream C++ snapshots by `tool/src/diagnostics/*.js` (`*_data.rs`, repeat runs byte-identical, throws on missing needle); behavior surface is hand-written 1:1.

- [Module map](#module-map)
- [Test map (tests/, 39 cases)](#test-map-tests-39-cases)
- [Concurrent-seat orphan files (plane yielded to its owning seat)](#concurrent-seat-orphan-files-plane-yielded-to-its-owning-seat)
- [Deferred ledger (not in wave1, with blockers)](#deferred-ledger-not-in-wave1-with-blockers)
- [Re-run & gates](#re-run-gates)

## Module map

| v8 source | crate module | generator (needles) | notes |
|---|---|---|---|
| `src/diagnostics/eh-frame.{h,cc}` + `x64/arm64/eh-frame-<arch>.cc` | `src/eh_frame_data.rs` | `eh-frame.js` (45) | DwarfOpcodes 10 + encoding specifiers 5 + tag/mask/layout constants + x64/arm64 arch params (alignment × RA pseudo-reg code × CIE initial state) |
| same (behavior) | `src/eh_frame.rs` | — | `EhFrameWriter` (CIE/FDE/.eh_frame_hdr) + `EhFrameIterator` (read-back) + ULEB/SLEB codec |
| `src/diagnostics/gdb-jit.cc` | `src/gdb_jit_data.rs` | `gdb-jit.js` (25) | 7 struct layout tables (ELFHeader/ELFSectionHeader/MachO*/JITCodeEntry/JITDescriptor, 64-bit) + 4 enum groups + ident/needles |
| same (behavior) | `src/gdb_jit.rs` | — | `Writer` buffer primitives + `ElfImage`/`MachOImage` assembly + `GdbJitRegistry` linked list |
| `src/diagnostics/perf-jit.{h,cc}` | `src/perf_jit_data.rs` | `perf-jit.js` (14) | 6 record structs (repr(C) POD) + 5 events + 9 ELF machines + header/buffer constants |
| same (behavior) | `src/perf_jit.rs` | — | `header_bytes`/`code_load_bytes`/`debug_info_bytes`/`unwinding_bytes` serialization (LE, byte-identical to C++ memory layout) |
| `src/diagnostics/objects-printer.cc` (anon ns) | `src/objects_print_data.rs` | `objects-printer.js` (12) | PrintDouble needles + safe-integer bounds + HelperState/ZipMode name tables |
| same (behavior) | `src/objects_print.rs` | — | `print_double`/`double_to_integer`/`is_minus_zero` + enum ToString |
| `src/diagnostics/basic-block-profiler.{h,cc}` | `src/basic_block_profiler.rs` | `basic-block-profiler.js` (10) | `BasicBlockProfilerData` (reverse post-order block count/branches/schedule/log/Display) + `BasicBlockProfiler` (singleton/reset/coverage bitmap/lock-free harvest) + `BuiltinsCallGraph` (call graph map) |
| `src/diagnostics/compilation-statistics.{h,cc}` | `src/compilation_statistics.rs` | `compilation-statistics.js` (9) | `BasicStats` (time delta µs/space/graph size/accumulate/JSON) + `CompilationStatistics` (phases/totals) + `printable_statistics` (human double-line/machine format) |
| `src/diagnostics/objects-debug.cc` | `src/objects_debug_generated.rs` | `objects-debug.cc.js` (3) | `VERIFY_REGISTRY` 259 `<Class>::<Class>Verify` validator registry table |
| — | `src/error.rs` | — | `Error`/`Result` placeholder (no wave1 error sites; variants land with the IO-coupled wave) |

## Test map (tests/, 39 cases)

| file | cases | anchors |
|---|---|---|
| `tests/eh_frame.rs` | 12 | x64/arm64 CIE goldens (28 B) + full 72 B stream golden + iterator round-trip (±624485 LEB cases aligned with upstream `eh-frame-iterator-unittest`) + 5th-byte bit4-6 uleb overflow-rejection branch + advance_location widths + rule directives + 2 handmade skip_cie byte tests |
| `tests/gdb_jit.rs` | 8 | Writer primitives (LE writes/truncation/align/LEB) + ELF64 header & section-table patch-back + MachO64 sizeofcmds/fileoff + registry head-insert/unlink/destroy guard + 3 reentrancy migrations (linked flag) + body-section addr-zero regression |
| `tests/perf_jit.rs` | 5 | 4 record types field-by-field offsets + 8-byte zero padding + empty-unwinding .eh_frame_hdr hack |
| `tests/objects_print.rs` | 4 | PrintDouble 4 branches + DoubleToInteger branch table + name tables + print_double_manual_six |
| `tests/basic_block_profiler.rs` | 4 | ProfilerData lifecycle/Log/Display descending print + coverage bitmap/single-pass reset harvest + call graph lookup + Verify registry 259 anchor assertions |
| `tests/compilation_statistics.rs` | 5 | BasicStats saturating accumulation + JSON serialization + phases and totals aggregation + human double-line/machine format output + zero time/space/graph edge safety |
| `tests/main.rs` | 1 | crate smoke (real assertion via exported decode_uleb128) |

## Concurrent-seat orphan files (plane yielded to its owning seat)

`src/` contains 12 files NOT wired into `lib.rs`'s module graph (invisible to all three gates): disasm.rs, disassembler.rs, code_tracer.rs, unwinder.rs, unwinding_info_win64.rs, 6× etw_*, gdb_jit_h_generated.rs. All 8 `todo!("待 Developer 修订")` placeholders in the crate concentrate in 4 of them: disasm 3, code_tracer 3, unwinder 1, disassembler 1; the remaining orphans have been landed by their owning seat as todo!-free data planes. They are another seat's in-flight content, not this ticket's delivery; deletion/repair is out of scope here. This registration removes the "unregistered" defect; the owning seat must land or remove them before its own freeze.

## Deferred ledger (not in wave1, with blockers)

| surface | blocker |
|---|---|
| eh_frame: `Label`/`RelocInfo`/`CodeDesc` patch-back (`GetEhFrame`/`EmitExternalCodeInfo`) | v8-codegen-core (not ready) |
| gdb_jit: DWARF debug sections (`.debug_*`/CFI, WriteDebugInfo/WriteLineInfo) | CodeDescription/RelocInfo coupling (codegen-core + L3 objects) |
| gdb_jit: ELF symbol table (`CreateSymbolsTable`/`ELFSymbol`/`ELFSymbolTable`, .symtab+.strtab, gdb-jit.cc:721-869,1015-1035) | depends on CodeDescription (desc->name/CodeSize) |
| gdb_jit: ia32 32-bit branch | V8 ia32 frozen; 64-bit only |
| objects_print: `ObjectImpl::Print` and HeapObject body printing | Tagged/Isolate coupling (L3 objects) |
| objects_print: PrintDouble default branch | intentional deviation: Rust Display shortest round-trip vs C++ ostream %g 6 significant digits; not blocking, not reconciled |
| perf_jit: Isolate timestamps/PID/TID, `jit-<pid>.dump` file IO, `LogClose` | Isolate/filesystem coupling; inputs already injected by caller |
| eh_frame/gdb_jit: real CodeStubAssembler fill chain | same as codegen-core |
| basic_block_profiler: `CopyToJSHeap` / `OnHeapBasicBlockProfilerData` | JS heap object allocation coupled with Isolate (returns 0 safely in pure logic plane) |
| objects_debug: each `Verify` method body (~4300 lines of invariant checks) | depends on each object's complete internal layout fields and Isolate runtime context |

## Re-run & gates

- Scripts: `cd tool && bun src/diagnostics/<name>.js` (pure text scan, fixed-order output, no timestamps; regenerating overwrites in-place revisions)
- Gates: `cargo nextest run -p v8-diagnostics --all-features` / `cargo clippy -p v8-diagnostics --all-targets --all-features -- -D warnings` / `cargo +nightly fmt -p v8-diagnostics -- --check`

(Development log and review-round ledger: see ../../log/v8-diagnostics.md)


---

<a name="zh"></a>

# v8-diagnostics

`v8/src/diagnostics` 的 Rust 转写 — wave1 零耦合子集 (平台无关纯逻辑面)。
数据面由 `tool/src/diagnostics/*.js` 从上游 C++ 快照生成 (`*_data.rs`, 复跑幂等, 缺针即 throw)；行为面手工 1:1。

- [模块映射](#模块映射)
- [测试映射 (tests/, 共 39 例)](#测试映射-tests-共-39-例)
- [并发席孤儿文件登记 (让面归属席)](#并发席孤儿文件登记-让面归属席)
- [推迟台账 (wave1 未落, 阻塞原因)](#推迟台账-wave1-未落-阻塞原因)
- [复跑与门禁](#复跑与门禁)

## 模块映射

| v8 源文件 | 本 crate 模块 | 生成脚本 (对数) | 说明 |
|---|---|---|---|
| `src/diagnostics/eh-frame.{h,cc}` + `x64/arm64/eh-frame-<arch>.cc` | `src/eh_frame_data.rs` | `eh-frame.js` (45) | DwarfOpcodes 10 + 编码说明符 5 + 标签/掩码/布局常量 + x64/arm64 架构参数 (对齐因子 × RA 伪寄存器码 × CIE 初始态) |
| 同上 (行为) | `src/eh_frame.rs` | — | `EhFrameWriter` (CIE/FDE/.eh_frame_hdr 写出) + `EhFrameIterator` (回读) + ULEB/SLEB 编解码 |
| `src/diagnostics/gdb-jit.cc` | `src/gdb_jit_data.rs` | `gdb-jit.js` (25) | ELFHeader/ELFSectionHeader/MachO*/JITCodeEntry/JITDescriptor 7 结构布局表 (64 位) + 枚举 4 组 + ident/针 |
| 同上 (行为) | `src/gdb_jit.rs` | — | `Writer` 缓冲原语 + `ElfImage`/`MachOImage` 镜像拼装 + `GdbJitRegistry` 注册链 |
| `src/diagnostics/perf-jit.{h,cc}` | `src/perf_jit_data.rs` | `perf-jit.js` (14) | 6 记录结构 (repr(C) POD) + 事件 5 + ELF machine 9 + 头尺寸/缓冲常量 |
| 同上 (行为) | `src/perf_jit.rs` | — | `header_bytes`/`code_load_bytes`/`debug_info_bytes`/`unwinding_bytes` 序列化 (小端, 与 C++ 内存布局逐字节一致) |
| `src/diagnostics/objects-printer.cc` (匿名 ns) | `src/objects_print_data.rs` | `objects-printer.js` (12) | PrintDouble 针 + 安全整数边界 + HelperState/ZipMode 名表 |
| 同上 (行为) | `src/objects_print.rs` | — | `print_double`/`double_to_integer`/`is_minus_zero` + 枚举 ToString |
| `src/diagnostics/basic-block-profiler.{h,cc}` | `src/basic_block_profiler.rs` | `basic-block-profiler.js` (10) | `BasicBlockProfilerData` (逆后序块计数/分支/调度/日志/Display) + `BasicBlockProfiler` (单例/清零/覆盖率位图/无锁收割) + `BuiltinsCallGraph` (调用图映射) |
| `src/diagnostics/compilation-statistics.{h,cc}` | `src/compilation_statistics.rs` | `compilation-statistics.js` (9) | `BasicStats` (耗时微秒/内存/图规模/累加/JSON) + `CompilationStatistics` (阶段/总量) + `printable_statistics` (人读双线/机器格式) |
| `src/diagnostics/objects-debug.cc` | `src/objects_debug_generated.rs` | `objects-debug.cc.js` (3) | `VERIFY_REGISTRY` 259 条全量 `<Class>::<Class>Verify` 校验器注册表 |
| — | `src/error.rs` | — | `Error`/`Result` 占位 (wave1 无落子错误面, 变体随 IO 耦合波次补) |

## 测试映射 (tests/, 共 39 例)

| 测试文件 | 例数 | 锚定 |
|---|---|---|
| `tests/eh_frame.rs` | 12 | x64/arm64 CIE 金标 (28 字节) + 全流 72 字节金标 + 迭代器回读 (对齐上游 `eh-frame-iterator-unittest` ±624485 LEB 用例) + 第 5 字节 bit4-6 uleb 溢出拒绝分支 + advance 宽度退化 + 规则指令编码 + skip_cie 手工字节 2 例 |
| `tests/gdb_jit.rs` | 8 | Writer 原语 (LE 直写/截断/对齐/LEB) + ELF64 镜像头与节头表回填 + MachO64 sizeofcmds/fileoff 回填 + 注册链头插/摘链/destroy 防护 + linked 重入迁移 3 例 + 实体节 addr 置零回归 |
| `tests/perf_jit.rs` | 5 | 4 类记录逐字段偏移 + 8 字节补零 + 空 unwinding 落空 .eh_frame_hdr hack |
| `tests/objects_print.rs` | 4 | PrintDouble 四分支 + DoubleToInteger 逐分支 + 名表 + print_double_manual_six |
| `tests/basic_block_profiler.rs` | 4 | ProfilerData 生命周期/Log/Display 降序打印 + 覆盖率位图/单次遍历重置收割 + 调用图检索 + Verify 注册表 259 锚点断言 |
| `tests/compilation_statistics.rs` | 5 | BasicStats 饱和累加 + JSON 序列化 + 阶段与总量统计聚合 + 人读双线/机器格式输出 + 零耗时/零分配/零图规模空安全 |
| `tests/main.rs` | 1 | crate 冒烟 (真实断言导出面 decode_uleb128) |

## 并发席孤儿文件登记 (让面归属席)

`src/` 下 12 个文件未挂 `lib.rs` 模块图 (三门禁均不可见): disasm.rs、disassembler.rs、code_tracer.rs、unwinder.rs、unwinding_info_win64.rs、6× etw_*、gdb_jit_h_generated.rs。全库 `todo!("待 Developer 修订")` 共 8 处，集中于其中 4 个文件: disasm 3、code_tracer 3、unwinder 1、disassembler 1；其余孤儿文件已由归属席落为无 todo! 数据面。属并发席在飞内容、非本票交付面；本票不代删代修，此处登记消"未登记"缺陷，归属席冻结前须落地或移除。

## 推迟台账 (wave1 未落, 阻塞原因)

| 面 | 阻塞 |
|---|---|
| eh_frame: `Label`/`RelocInfo`/`CodeDesc` 回填 (`GetEhFrame`/`EmitExternalCodeInfo`) | v8-codegen-core (未就绪) |
| gdb_jit: DWARF 调试节生成 (`.debug_*`/CFI, WriteDebugInfo/WriteLineInfo) | CodeDescription/RelocInfo 耦合 (codegen-core + L3 objects) |
| gdb_jit: ELF 符号表 (`CreateSymbolsTable`/`ELFSymbol`/`ELFSymbolTable`, .symtab+.strtab, gdb-jit.cc:721-869,1015-1035) | 依赖 CodeDescription (desc->name/CodeSize) |
| gdb_jit: ia32 32 位分支 | V8 ia32 面已冻结, 仅保 64 位 |
| objects_print: `ObjectImpl::Print` 等 HeapObject 主体打印 | Tagged/Isolate 耦合 (L3 objects) |
| objects_print: PrintDouble 默认分支 | 有意偏离: Rust Display 最短往返 vs C++ ostream %g 6 位有效, 不阻塞、不追平 |
| perf_jit: Isolate 取时间戳/PID/TID、`jit-<pid>.dump` 文件 IO、`LogClose` | Isolate/文件系统耦合, 入参已由调用方注入解耦 |
| eh_frame/gdb_jit: 真实 CodeStubAssembler 反填链 | 同 codegen-core |
| basic_block_profiler: `CopyToJSHeap` / `OnHeapBasicBlockProfilerData` | JS 堆对象分配与 Isolate 耦合 (纯逻辑面安全返回 0) |
| objects_debug: 各 `Verify` 方法体 (~4300 行不变式检查) | 依赖各对象全量内部布局字段与 Isolate 运行时上下文 |

## 复跑与门禁

- 脚本: `cd tool && bun src/diagnostics/<name>.js`, 产物与 `*_data.rs` 基线零差异 (复跑幂等); 缺针/项数不符即 throw, 不写文件。
- 门禁: `cargo nextest run -p v8-diagnostics --all-features` / `cargo clippy -p v8-diagnostics --all-targets --all-features -- -D warnings` / `cargo +nightly fmt -p v8-diagnostics -- --check`。

(开发日志与复审轮记录见 ../../log/v8-diagnostics.md)

