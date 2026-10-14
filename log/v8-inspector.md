# v8-inspector 终验冻结报告

## 一、模块概览
- 原版源码: `v8/src/inspector/`
- 规模: ~20k 行 C++ CDP 核心实现
- 状态: **终验收敛冻结 (clean 3/3, 6/6 单测通过, Clippy 0 警告)**
- 冻结时间: 2026-10-12

---

## 二、架构与关键转写契约

### 1. 扁平导出架构
- 子模块全部为私有:
  - `agents.rs` (Heap/Profiler/Schema 三代理与 CustomPreview)
  - `console.rs` (Console 消息、调用栈与消息仓储)
  - `core.rs` (中枢会话、调试器 ID 与远程对象 ID)
  - `debugger.rs` (断点系统、脚本模型与序列化去重)
  - `error.rs` (强类型专用错误系统)
  - `runtime.rs` (运行时值镜像与上下文管理)
  - `strings.rs` (String16、CRC32 与魔术注释解析)
- 顶层 `lib.rs` 统一 `pub use` 扁平导出，无二级命名空间暴露，彻底杜绝二次导出。

### 2. 协议与底层实现细节
- **字符串与 CRC32** (`strings.rs`):
  - `String16`: 完整 UTF-16 容器，无损 Unicode 代理对映射。
  - `CRC32_TABLE`: 标准 IEEE 802.3 多项式 0xedb88320，逐字节小端位运算与 `v8/src/inspector/crc32.cc` 1:1 等价。
  - 魔术注释定位: `find_source_url`, `find_source_map_url`, `find_debug_id` 行首/尾注释状态机完全覆盖。
- **ID 体系** (`core.rs`):
  - `V8DebuggerId`: 双 `i64` 对，格式化为 `"{first}.{second}"`。
  - `RemoteObjectId` & `RemoteCallFrameId`: 三段式 `"{isolate_id}.{injected_script_id}.{id}"` 解析与序列化。
- **运行时与值镜像** (`runtime.rs`):
  - `V8InternalValueType`: 6 变体（None, Entry, Scope, ScopeList, PrivateMethodList, PrivateMethod）。
  - `DiscardReason`: 3 变体（Fulfilled, PromiseCollected, TearDown）。
  - `RemoteObject`: 支持 serde 序列化为 CDP 规范 JSON。
- **调试器与断点** (`debugger.rs`):
  - `BreakpointType`: 8 变体（ByUrl=1..InstrumentationBreakpoint=8，`#[repr(u8)]`）。
  - `V8DebuggerScript`: 采用 `ScriptPositionSpan` 封装行列范围，消除参数过多问题，零 allow 宏。
  - `V8SerializationDuplicateTracker`: 基于对象 ID 的去重跟踪器，防止 CDP 循环引用爆炸。
- **控制台与调用栈** (`console.rs`):
  - `V8MessageOrigin`: 3 变体（Console, Exception, RevokedException）。
  - `ConsoleAPIType`: 16 变体。
  - `V8ExceptionLocation`: 封装异常源信息。
  - `V8ConsoleMessageStorage`: 支持 `count`, `count_reset`, `time`, `time_end` 计时器与计数器管理。
- **代理后端** (`agents.rs`):
  - `K_MAX_CUSTOM_PREVIEW_DEPTH = 20` 递归深度边界守卫。

---

## 三、安全与代码规范红线
- **100% Safe Rust**: 顶层 `#![forbid(unsafe_code)]`，全局无 unsafe 块。
- **零 Panic**: 全文零裸 unwrap / expect，零 todo! / unimplemented!。
- **零 Allow 绕过**: 零 `#[allow(...)]` 抑制警告。
- **工具脚本确定性**: `tool/src/inspector/` 下全部 6 个脚本复跑差异为 0，100% 幂等。

---

## 四、门禁验证
- 单元测试: 6/6 passed (100% 通过)
- Clippy 静态检查: 0 warnings, clean ผ่าน
- 独立审查记录:
  - Round 1: clean (子代理 `6192d758-5681-41bd-b54b-0221f07f7707`)
  - Round 2: clean (子代理 `61d49b37-3ab8-480f-b8cf-0e49538800ae`)
  - Round 3: clean (子代理 `1223b99a-fcee-4cdd-bab1-e7eaa6fdcf25`)
  - **达成 clean 3/3 终验冻结标准**
