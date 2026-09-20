# v8 → Rust 转写路线图（终极目标：跑通 v8 完整 JS 测试）

终极目标：由 `./conv.js` 流水线转写出的 Rust v8 能够运行 v8 自带的
mjsunit 完整 JS 测试集（`v8/test/mjsunit`），语义正确性以原版测试体系为准。
每阶段以"前序测试持续全绿 + 转换报告覆盖率提升"为收敛判据，
禁止以 skip 用例、删断言、改期望值等方式绕过。

## 阶段0：测试基建（本次）

- `example/tests/converted/`：按"一个 C++ 源文件 ↔ 一个 .rs 测试文件"组织。
- `./js/test-converted.js`：流水线尾部自动跑复刻测试。
- 判据：`./test.sh` 全绿。

## 阶段1：复刻首批纯逻辑模块测试（本次）

对 `v8rs/src/v8/` 中已有真实函数体的模块建立行为基线：

| 测试文件 | 对应源 | 来源 |
| --- | --- | --- |
| `converted/utils_sha_256.rs` | `v8/src/utils/sha-256.cc` | v8 无对应 gtest，采用 NIST KAT 标准向量 |
| `converted/base_abort_mode.rs` | `v8/src/base/abort-mode.cc` | 语义 1:1 复刻 |
| `converted/base_fpu.rs` | `v8/src/base/fpu.cc` | 常量复刻；roundtrip 用例 #[ignore]（见缺口） |
| `converted/heap_base_unsafe_json_emitter.rs` | `v8/src/heap/base/unsafe-json-emitter.cc` | 输出比对 |

已知转译缺口（如实记录，不 hack）：
- cpp2rust 丢弃内联汇编：`base_fpu` 的 `GetStatusWord/SetStatusWord` 是空操作，
  相关 roundtrip 测试 `#[ignore]`，待支持 asm 后启用。
- 头文件模板成员未转译：`UnsafeJsonEmitter::p()` 的多属性逗号逻辑不在转译产物中，
  待头文件转译能力覆盖后补测。
- `mjsunit/hardcoded/flush-denormals-*.js` 是该模块的原始 JS 测试，接入引擎后回收。

## 阶段2：扩大转译覆盖 + 真 1:1 gtest 复刻

- 依据 `cpp2rust/build/conversion-report.md` 的失败/剔除清单，改进 cpp2rust
  规则与转换脚本（超时、cxxflags、规则缺口），提升可过检模块数。
- 进展（2026-09-19）：已按 `v8/DEPS` pinned commit 浅拉取 abseil-cpp、
  googletest/src、fp16、highway、simdutf、ittapi、llvm-libc、llvm-libclang
  至 `v8/third_party/`，`js/conv.js` 已对齐 v8 BUILD.gn 的 include_dirs；
  absl/gtest 类 fatal error 清零。剩余大头为构建期生成头
  （metagen/instance-types.h ×425、bytecodes-builtins-list.h ×408、
  inspector protocol ×20+），正在 scratch 工作区 `../v8-gen`（同 commit
  e2c9ce37）gclient sync + gn/ninja 构建，产物由 `./js/gen-headers.js`
  拷入 `cpp2rust/build/gen/` 供转译使用。
- 优先纳入"有 gtest 且能过检"的文件（如 `base/utils` 下 bits、ieee754、vlq），
  从 `v8/test/unittests` 1:1 转写用例。
- 建立转译模块 API 快照（函数名带数字后缀随规则漂移），防测试静默失配。
- 进展（2026-09-20）：转换器"无法转译"语义统一为显式 `UNSUPPORTED: <原因>`
  + exit=3（此前依赖 assert：Release 下被剥离后段错误或静默错译）。
  `js/conv.js`/`js/report.js` 新增 unsupported 一等分类桶（与 crash/
  timeout/error/check_dropped 并列），crash 桶自此只余真正的转换器缺陷。
  已知 unsupported 家族（修复方向 = 补类型映射/规则，而非修崩溃）：
  char16_t 未映射（inspector/string-16）、torque 自引用指针类型未映射
  （torque/type-oracle）、std::optional 比较运算符无规则（flags/flags.cc）、
  含 NTTP 的函数指针类型、C++20 ConceptSpecializationExpr。
- 判据：复刻测试全绿且转换报告覆盖率显著上升。

## 阶段3：引擎骨架

- 转译 `v8/src/d8`（shell 入口）与 api 层；补 rt 运行时缺口：
  虚函数表分发、异常/平台检查路径（当前直接 panic!）。
- 判据：d8 能执行 `print(1+1)` 级别脚本。

## 阶段4：接入 mjsunit

- 编写 `js/run-mjsunit.js` harness：解析 `// Flags:`、注入 mjsunit.js 断言库、
  按 testcfg 分批放量执行。
- 判据：mjsunit 子集通过率单调上升，直至完整 JS 测试跑通（终极目标达成）。
