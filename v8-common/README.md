[English](#en) | [中文](#zh)

---

<a name="en"></a>

# v8-common

Rust port of V8 `src/common` (L0 foundation layer). Target config: x64 + arm64 with COMPRESS_POINTERS + SHARED_CAGE + SANDBOX + EXTERNAL_CODE_SPACE enabled.

See [中文文档](./zh.md) for the full module/test mapping tables and the semantic deviation log.


---

<a name="zh"></a>

# v8-common

V8 `src/common` 的 Rust 移植 (L0 地基层)。目标配置: x64 + arm64, COMPRESS_POINTERS + SHARED_CAGE + SANDBOX + EXTERNAL_CODE_SPACE 开。

- [模块映射](#模块映射)
- [测试映射](#测试映射)
- [语义偏差登记 (关键项)](#语义偏差登记-关键项)

## 模块映射

| 原版 (v8/src/common) | Rust 模块 | 说明 |
|---|---|---|
| `globals.h` | `globals.rs` + `globals/config.rs` | 枚举/常量/类型别名; 配置布尔独立展开 |
| `globals.h` 内嵌反馈枚举 | `globals.rs` | BinaryOperationFeedback/CompareOperationFeedback/TypeOfFeedback 展平 |
| `FOREACH_WASM_TRAPREASON` | `globals.rs::WasmTrapReason` | X-macro 机械展开 |
| `include/v8-internal.h` 常量 | `internal.rs` | 底层常量层 (手工, 非脚本产物) |
| `message-template.h` | `message_template.rs` | 498 模板全量 + 字符串表 |
| `assert-scope.h` / `.cc` | `assert_scope.rs` | 泛型 PerThreadAssertScope + 23 别名 |
| `operation.h` | `operation.rs` | 22 变体 + 判定函数 |
| `ptr-compr.h` / `.cc` / `-inl.h` | `ptr_compr.rs` | 压缩方案族全语义 |
| `code-memory-access.h` / `.cc` / `-inl.h` | `code_memory_access.rs` | ThreadIsolation + 写通道 + RwxMemoryWriteScope |
| `checks.h` | `checks.rs` | DCHECK 宏 → 断言函数 |
| `simd128.h` | `simd128.rs` | Simd128 bit_cast 容器 |
| `scoped-modification.h` | `scoped_modification.rs` | RAII 改值恢复 |
| `thread-local-storage.h` | `thread_local_storage.rs` | TLS 模型常量 (宏无运行时实体) |
| `high-allocation-throughput-scope.h` | `high_allocation_throughput_scope.rs` | observer trait + RAII |
| `synchronization-point-support.h` / `.cc` | `synchronization_point_support.rs` | 同步点阻塞全语义 |

生成脚本: `tool/src/common.js` (冻结)。直出产物已就地修订, 复跑会覆盖。

## 测试映射

| 原测试文件 | Rust 测试文件 | 说明 |
|---|---|---|
| `v8/test/unittests/common/thread-isolation-unittest.cc` | `tests/thread_isolation.rs` | 7 用例 1:1; EXPECT_DEATH_IF_SUPPORTED → `assert!(result.is_err())`, 强度守恒 (原版 CHECK 失败即终止, 本 crate 禁 panic 落 Result, 登记偏差) |
| — (自建) | `tests/globals.rs` | 枚举判别值对数 + 原版 static_assert 族落编译期/运行期校验 |
| — (自建) | `tests/ptr_compr.rs` | 压缩/解压往返 + 4GB 回绕 + 半字展开 |
| — (自建) | `tests/assert_scope.rs` | 作用域进出配对 + 嵌套例外 + Release 幂等 |
| — (自建) | `tests/message_template.rs` | 498 对数 + from_int 边界 + 模板串抽查 |
| — (自建) | `tests/simd128.rs` | splat/to 位级往返 |
| — (自建, round3) | `tests/code_memory_access.rs::code_memory_registry_concurrent_stress` | 注册表并发压力: 8 线程 × 64 轮 register/unregister/bulk-split/except 交错 (fastrand 固定 seed 0x5EED_2026) + stable 页并发读 + 收尾 tear_down 空表不变式 |
| — (自建, round3) | `tests/globals.rs::globals_round3_name_collision_anchors` | 跨枚举重名变体判别值锚 (全量程序化 diff 固化) |

`v8/src/common` 其余头文件无官方单测 (gtest 目录仅 thread-isolation 一个), 上游覆盖以门禁静态断言 (原版 static_assert 编译期化) 兜底。

## 语义偏差登记 (关键项)

- CHECK/DCHECK 失败: C++ 进程终止 → Rust `Result<_, Error>` (error.rs 逐条对应原版 file:line)
- `JitPageReference` 页锁: C++ 每页自带 mutex_ → Rust 全局注册表锁 (粒度收敛, 语义等价)
- `ExternalCodeCompressionScheme::GetPtrComprCageBaseAddress(Address)` 原版 `= delete` → 不提供
- `v8_flags` 子集 (jitless / force_memory_protection_keys / memory_protection_keys) → `flags` 模块原子布尔, L3 flags crate 接管
- per-isolate 断言: isolate 字段读写经 `PerIsolateAssertState` trait, L3 接入点
- `WritableJumpTablePair` 构造: C++ 三次加锁 → Rust 单次注册表锁内完成 split+register
- `MessageTemplateFromInt` 越界: DCHECK 终止 → `None`
- `UnregisterRange` 左切加严 (round2 F4): 原版对键 < start 且跨 start 的分配不受检、原样保留 (cc:373-384, CFI 弱点); Rust `Err(SplitFreeRange)` 注册表不动
- `UnregisterAllocationsExcept` 三处加严 (round2 F4, 原版 API 全仓零调用方): 左切分配 `Err` (原版静默弃注册, cc:395-398); 右切分配 `Err` (原版静默弃注册, 无 CHECK); start 前分配恒保留 (原版 pred 须命中 keep[0] 否则弃注册, cc:395-409); keep 清单不匹配 `Err(KeepListMismatch)` 对齐原版 CHECK (cc:410)
- `WritableJitPage`/`WritableJitAllocation` 的 `page_ref`/`page_ref_mut`: Rust 扩展访问器 (原版私有成员/私有访问器, 无公开 getter; round2 F5 引证纠正), 供测试与 L3 探测
- `slow_dcheck`/`slow_dcheck_implies`: 惰性闭包入参 (round2 F1), 对齐原版禁用分支 `((void)0)` 条件不求值 (checks.h:21-22)
- `PerThreadAssertScope` 线程绑定 (round3 F7): 原版作用域删除拷贝构造且无移动构造, 栈绑定 (assert-scope.h:65-66); Rust 以 `PhantomData<*const ()>` 收紧为 !Send, 23 别名随之, 防跨线程移动后 drop 将 old_data 回写他线程 TLS; tests/assert_scope.rs 以 assert_not_impl_all! 编译期锁 + 跨线程隔离用例双保险
- `RwxMemoryWriteScope` 线程绑定 (round4 F8): 原版删除拷贝构造 (code-memory-access.h:99-100) 且自声明析构抑制移动, 权限窗口仅作用构造线程 (h:72-89); Rust 零字段载体派生 Send, 跨线程 drop 将在他线程执行 SetExecutable 使构造线程永久残留可写权限; 以 `PhantomData<*const ()>` 收紧为 !Send + !Sync, tests/code_memory_access.rs 编译期锁 (Writable* 三写通道本就因持 JitPageRef 而 !Send, 不受影响)

