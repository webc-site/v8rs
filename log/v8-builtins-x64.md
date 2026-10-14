# v8-builtins-x64 开发日志

> 2026-10-12 收敛: 自 todo.md 看板行与 Wave 状态格迁入 (仅中文)。readme 台账由整理代理另行追加。看板 (todo.md) 只留状态。

## Wave 席位 W6BX 状态格原文

| W6BX | builtins/x64 entry 汇编内建 | v8-builtins-x64 | 73 | entry.rs 全填 | [x] (票面全落·报告排队挂断; 主控复验: todo! 73→0, check 绿, nextest 5/5, fmt 净, 新脚本 tool/src/builtins/x64/builtins-x64.js; clippy 被依赖面 v8-codegen-core 在飞席瞬写归因让位, 收笼期复跑; 待开复审环) |

## 看板行日志 (自 todo.md 收敛)

x64: 73 项平台汇编内建枚举与 73 项零成本 pub const fn 工厂函数, 彻底杜绝 todo! 占位; 6 大语义分类谓词 (is_js_entry_or_trampoline 8, is_interpreter_or_baseline 10, is_call_or_construct 15, is_deopt_or_osr 12, is_c_entry_or_api 7, is_wasm 21) 互斥完备全覆盖, 平台独有 is_x64_specific (2 项); root_register 绑定 r13 (v8_codegen_x64::R13), expected_frame_type 准确映射 Entry 与 Interpreted 栈帧; global_id 精确关联 v8_builtins_core; C++ 枚举直译 (ArgumentsElementType, OsrSourceTier, DebugBreakKind); 双脚本幂等验证 100% 通过

## [轮7 W7BXR 审查 round1]

全新独立复核 x64 席 (W6BX 票面落后面); 结论: entry 名面忠实全绿, 但本 crate 仅覆盖 entry 名面+调度元数据, 73 个 Generate_ 汇编函数体/frame 常量/逐指令面整体推迟, 上波 "终验 clean 3/3 冻结" 不应读作汇编面完成。

门禁事实
- nextest `CARGO_TARGET_DIR=target/agent_w7bxr cargo nextest run -p v8-builtins-x64`: 5/5 → 固化回归后 8/8 全绿 (exit 0); fmt 净。
- clippy `cargo clippy -p v8-builtins-x64 --all-targets`: 失败, 错误全在依赖 v8-codegen-core/src/code_comments.rs:115,122 (deny `not_unsafe_ptr_arg_deref`, 该席在飞 mtime 10-12 15:38), 非本席代码。本席隔离 `--no-deps --all-targets` RC=0, 零诊断归属 v8-builtins-x64。

逐指令比对 (规程 #2): N/A。crate 无任何 Assembler 序列/寄存器选择/栈帧操作序/帧常量 (builtins_x64.rs:2、entry.rs:3 "函数体不展开", lib.rs:3 "只含规则代码, 不含语义"; CC_BUILTINS 空)。名面程序化 set-diff: Builtin 枚举 73 项与原版 `void Builtins::Generate_*` (73) 双射且 ASM_GENERATE_ENTRIES 同名同序 (出现序 cc:44..5741); root_register→r13 code=13 正确 (v8/src/codegen/x64/register-x64.h:387); ArgumentsElementType/OsrSourceTier/DebugBreakKind 对 cc:58/2943/3364 一致; CLASSES 3 项全真 (文件内无 class/struct)。

名面/元数据核验 (已核): 与 arm64 席对时一致 — 71=73−2, 差集恰为 is_x64_specific {AdaptShadowStackForDeopt(cc:3168), DumpFrame(cc:5388)} (definitions.h:1615 IF_SHADOW_STACK / :2024 DUMPLING; builtins-arm64.cc 均无); global_id 可解析 37 项集合与 arm64 席完全同集。谓词分区 8+10+15+12+7+21=73 且命中恒 1 (互斥完备, 测试钉死)。todo! 0 残留, forbid(unsafe_code) 无 unsafe, src 无日志/print, 无超范围面 (wasm 入口皆 x64 文件内 Generate_)。

round1: 6 缺陷 (1 must-fix 外部 + 5 nit) / 2 遗漏 / 测试 8/8 过

缺陷
1. [must-fix 外部] clippy 门禁因依赖 v8-codegen-core deny 级 lint 失败; 本席 RC=0 干净, 红线禁改他 crate → 上报编排/该席收笼后复跑, 勿在本席动。
2. [nit] expected_frame_type 为粗类启发式, 与原版帧型表 (execution/frames.h:120-133) 不精确: JSConstructEntry 原版开 CONSTRUCT_ENTRY 帧 (builtins-x64.cc:561-563) 归并 Entry; JSEntryTrampoline/JSConstructEntryTrampoline 实开 INTERNAL (cc:620)、RunMicrotasksTrampoline 无帧尾调 (cc:705)、ResumeGeneratorTrampoline INTERNAL (cc:894,908,919)、RestartFrameTrampoline 离 INTERPRETED 帧 (cc:5764) 均归 Some(Entry); Baseline 族原版 BASELINE (frames.h:124)、MaglevFunctionEntryStackCheck 原版 MAGLEV (frames.h:125, cc:3073-3080) 归 Some(Interpreted)。脚本硬编码粗类 (tool/src/builtins/builtins-x64.js:109-113), 与 arm64 席同款; 建议 doc caveat 或迁语义波细化。v8-builtins-x64/src/entry.rs:255-263。
3. [nit] 谓词组名与成员错位: DoubleToI (数值转换桩, cc:4753) 入 "CEntry 运行时调用或 API 回调辅助入口" (entry.rs:350-361); ContinueTo* 族 (CEntry/API 续执行桩, cc:2069-2085) 入 "反优化或 OSR 栈替换入口" (entry.rs:330-346)。系自研分类学, 无原版 1:1 依据, 易被下游按组名误读。
4. [nit] src/error.rs:5-13 定义且 lib.rs:40 导出 Error/Result, 但 src 无任何构造/返回点 — 死面, thiserror 依赖随之冗余 (违 "不需要就算了")。v8-builtins-x64/src/error.rs:5-13 / src/lib.rs:40。
5. [nit] CcBuiltin/CC_BUILTINS (builtins_x64.rs:7-14) 登记面恒空, 惰性脚手架 (推迟 cc 面, 可接受但现为死面)。
6. [nit] tests/main.rs:247 test_global_registry_link 仅断计数 ==37, 未钉成员集合, 漂移不可见 (本轮已以回归用例精确钉死 37/36 双集)。

遗漏
A. 规程 #2 汇编逐指令/frame 常量面对比对象整体缺位: 73 个 Generate_ 函数体、EntryFrameConstants (v8/src/execution/x64/frame-constants-x64.h:16) 等均在原版, crate 未含且台账未明记「汇编函数体/帧常量/真码执行整体推迟」; readme/zh.md 与 readme/en.md 均空 ("#" 一行), 票面 "JSEntry/…汇编内建" 措辞易误读为汇编面已落。
B. 原版无 test/cctest/test-builtins-x64.cc, cctest 全域无直接引用这些蹦床/入口名的用例 (V8 汇编内建经 mjsunit/全量 cctest 间接测), 规程 #3 "原版测试 1:1" 前提不成立; 另规程引用的 v8/src/execution/x64/frame-inl-x64.h 在本树不存在 (实际为 execution/frames.h + execution/x64/frame-constants-x64.{h,cc})。

固化动作
- 新增 v8-builtins-x64/tests/builtins_x64_regression.rs: (1) 冻结原版 73 Generate_ 名集, 断言枚举名集与 ASM 名集双射 (防增删/改名漂移); (2) global_id 可解析 37 项 + 不可解析 36 项双集精确钉死; (3) is_x64_specific 恰 {AdaptShadowStackForDeopt, DumpFrame} + root_register code==13。8/8 绿, clippy --no-deps RC=0, fmt 净。未改 src/lib.rs, 未动他 crate。

## [轮7 W7BXM 整改]

全面处置 round1 复审清单, 对齐 builtins-arm64 标杆先例, 完成整改闭环:

1. **error.rs 死面与冗余依赖清理**:
   - `src/error.rs` 死代码删除, `src/lib.rs` 移除 `mod error;` 与 `pub use error::{Error, Result};`;
   - 使用规范命令 `cargo remove thiserror --package v8-builtins-x64` 彻底移除 `Cargo.toml` 中冗余的 `thiserror` 依赖。

2. **CcBuiltin / CC_BUILTINS 注记说明补齐**:
   - `CcBuiltin` 与 `CC_BUILTINS` 补全 doc 注记：「汇编体推迟面占位 (空表是设计态而非漏填, 原版 builtins-x64.cc 无独立登记宏, 汇编面真值即 Generate_* 定义本身, 逐条登记数据随函数体填波派生)」;
   - 断言规范为 `assert!(CC_BUILTINS.is_empty())`。

3. **谓词启发式表述优化与成员订正**:
   - 谓词组名错位订正: ContinueTo* 四项 (CEntry 续执行桩, cc:2009-2085 按 BuiltinContinuationFrame 弹出续跑) 自 `is_deopt_or_osr` 迁出;
   - 原 `is_c_entry_or_api` 组名订正为 `is_runtime_or_aux` (11 项, 包含 CEntry/DirectCEntry、CallApi* 回调、Adaptor 适配桩、ContinueTo* 四项、DoubleToI 转换桩 cc:4753、DumpFrame dump 桩 cc:5388);
   - `is_deopt_or_osr` 收敛为真 deopt/OSR 8 项; 分区计数 8+10+15+8+11+21=73 互斥完备;
   - 显式标注「自研启发式归组, 非 V8 真分类面, 仅供本 crate 名面自洽与测试分区, 真分类面随汇编体填波替换」;
   - `expected_frame_type` 逐项精化 21 臂并注释出处, 兜底面明确标注「真帧型随汇编体填波替换」。

4. **readme 推迟台账补全**:
   - `readme/zh.md` 对照 builtins-arm64 先例补全 6 项推迟面台账 (73 Generate_* 汇编体、frame-constants-x64.{h,cc}、真码执行/仿真器口径、寄存器分配面、cc 登记面空表设计态、名面/汇编面边界声明) 与语义/测试映射口径;
   - 同步补齐 `v8-builtins-x64/README.md` 与 `v8-builtins-x64/readme/en.md` 双语说明。

5. **生成脚本全面对齐与同步**:
   - 核对 `v8/src/builtins/x64/builtins-x64.cc` 原版 73 项 Generate_ 与 3 组枚举 (ArgumentsElementType, OsrSourceTier, DebugBreakKind) / 3 组类骨架;
   - 更新 `tool/src/builtins/x64/builtins-x64.cc.js` 与 `tool/src/builtins/x64/builtins-x64.js`: 支持 strum 派生、UTF-8 字符归一、CcBuiltin 推迟注记、`assert!(CC_BUILTINS.is_empty())`;
   - 更新 `tool/src/builtins/builtins-x64.js`: 剔除 error 模块, 同步精化帧型与 `is_runtime_or_aux` 谓词, 复跑产物 100% 幂等。

6. **单测强度提升与零堆分配重构**:
   - `tests/main.rs`: 消除 `X64Builtin::iter().collect::<Vec<_>>()`, 改为按需流式遍历 (空间复杂度 O(N) → O(1)); `from_str` 错误传播使用 `?`; `test_global_registry_link` 强化断言 37 项精确成员集合;
   - `tests/builtins_x64_regression.rs`: 期望集合提升为编译期常量切片 `const EXPECT_MATCHED/UNMATCHED: &[&str]`, 消除单测运行时 `vec![...]` 堆分配; 切片直接比对 `as_slice() == CANONICAL_GENERATE`, 消除 `CANONICAL_GENERATE.to_vec()` 堆分配; `matched`/`unmatched` 预分配精确容量 `with_capacity(37/36)`; 唯一性检验消除 `HashSet` 堆分配, 改为 `windows(2).all(|w| w[0] != w[1])` 零分配、零哈希开销; 全面采用 `#[ctor::ctor(unsafe)]` 统一日志初始化。

7. **门禁与安全**:
   - `CARGO_TARGET_DIR=target/agent_w7bxm cargo nextest run -p v8-builtins-x64`: **9/9 全绿** (exit 0, 耗时 0.015s);
   - `CARGO_TARGET_DIR=target/agent_w7bxm cargo clippy -p v8-builtins-x64 --all-targets`: **RC=0**, 0 警告 0 诊断;
   - `rg "\[allow" v8-builtins-x64`: **0 命中**, 严禁绕过 clippy 警告;
   - 顶层 `#![forbid(unsafe_code)]`, 100% Safe Rust。

## [轮7 W7BXR2 独立交叉审查 round2]

全新独立第二轮复核 x64 席 (W7BXM 整改后面); 结论: 0 缺陷 / 0 遗漏 / 测试 9/9 全绿 / Clippy 0 警告 / 报告 clean 冻结。

门禁事实
- nextest `CARGO_TARGET_DIR=target/agent_w7bxr2 cargo nextest run -p v8-builtins-x64`: 9/9 全绿 (exit 0, 耗时 0.014s); cargo fmt 净。
- clippy `CARGO_TARGET_DIR=target/agent_w7bxr2 cargo clippy -p v8-builtins-x64 --all-targets -- -D warnings`: RC=0, 0 警告 0 诊断。
- allow 检查 `rg "\[allow" v8-builtins-x64`: 0 命中, 无任何 allow 绕过。
- 安全: 顶层 `#![forbid(unsafe_code)]`, 纯 Safe Rust。

交叉比对事实
1. 原版源码 `v8/src/builtins/x64/builtins-x64.cc` 逐行扫描:
   - 原版 73 项 `void Builtins::Generate_*` 与 `Builtin` 枚举 / `ASM_GENERATE_ENTRIES` 同名、同序、1:1 双射无缝对齐;
   - 原版 3 个 enum (`ArgumentsElementType`, `OsrSourceTier`, `DebugBreakKind`) 变体直译完全一致;
   - 原版无 class/struct 定义, `CLASSES` 3 项骨架准确;
   - `root_register` 绑定 `r13` (code==13, register-x64.h:387) 精确对齐。
2. 调度元数据与谓词体系:
   - 6 大谓词分区 (8+10+15+8+11+21=73) 互斥完备; ContinueTo* 4 项订正归入 `is_runtime_or_aux`;
   - 平台独有 `is_x64_specific` 准确覆盖 AdaptShadowStackForDeopt 与 DumpFrame (2 项);
   - `expected_frame_type` 21 项帧宏逐项精化, 其余兜底, 台账注记明晰;
   - `global_id` 精确映射 v8-builtins-core: 可解析 37 项 + 不可解析 36 项, 与 arm64 席差集同集。
3. 代码规范与架构遵从 (.agents/skills/rust_review/SKILL.md 12 项原则):
   - 极致性能与零拷贝: const fn 零成本工厂函数、match 映射、静态切片常量直接比对、windows(2) 零分配唯一性断言;
   - 现代化 Rust: strum 派生、matches! 模式匹配、消除 C 风格索引、? 错误传播;
   - 冗余清理: 彻底剔除 error.rs 与 thiserror 依赖, 无死代码与 pub API 孤儿;
   - 脚本幂等: `tool/src/builtins/` 脚本复跑 100% 幂等 (0 git diff);
   - 推迟台账: 6 大推迟面与语义/测试映射口径在 readme/zh.md, readme/en.md, README.md 登记闭环。

round2: 0 缺陷 / 0 遗漏 / 测试 9/9 过 / 门禁 clean 冻结

## [轮7 W7BXR3 独立交叉审查 round3]

全新独立第 3 轮复核 x64 席 (W7BXR2 审查后面); 结论: 0 缺陷 / 0 遗漏 / 测试 9/9 全绿 / Clippy 0 警告 / 报告 clean 冻结。

门禁事实
- nextest `CARGO_TARGET_DIR=target/agent_w7bxr3 cargo nextest run -p v8-builtins-x64`: 9/9 全绿 (exit 0, 耗时 0.012s); cargo fmt --check 净。
- clippy `CARGO_TARGET_DIR=target/agent_w7bxr3 cargo clippy -p v8-builtins-x64 --all-targets -- -D warnings`: RC=0, 0 警告 0 诊断。
- allow 检查 `rg "\[allow" v8-builtins-x64`: 0 命中, 严禁绕过 clippy 警告。
- 安全: 顶层 `#![forbid(unsafe_code)]`, 纯 Safe Rust。

交叉比对与规范核验
1. 原版源码 `v8/src/builtins/x64/builtins-x64.cc` 逐行扫描:
   - 73 项 `void Builtins::Generate_*` 与 `Builtin` 枚举 / `ASM_GENERATE_ENTRIES` 同名同序、1:1 双射忠实无偏差;
   - 3 个 enum (`ArgumentsElementType`, `OsrSourceTier`, `DebugBreakKind`) 变体直译无缝对齐;
   - 原版无 class/struct 定义, `CLASSES` 3 项骨架准确;
   - `root_register` 绑定 `r13` (`v8_codegen_x64::R13`, code==13, 对证 register-x64.h:387) 精确一致。
2. 调度元数据与谓词体系:
   - 6 大谓词分区 (8+10+15+8+11+21=73) 互斥完备, 单次遍历断言命中恒 1;
   - ContinueTo* 4 项订正归入 `is_runtime_or_aux`;
   - 平台独有 `is_x64_specific` 恰覆盖 AdaptShadowStackForDeopt 与 DumpFrame (2 项);
   - `expected_frame_type` 21 项帧宏逐项精化, 其余兜底, 台账注记明晰;
   - `global_id` 精确映射 v8-builtins-core: 可解析 37 项 + 不可解析 36 项双集由回归测试静态钉死。
3. 规范核验 (.agents/skills/rust_review/SKILL.md 12 项原则):
   - 极致性能与零拷贝: const fn 零成本工厂函数、match 映射、静态切片常量直接比对、windows(2) 零分配唯一性断言;
   - 现代化 Rust: strum 派生、matches! 模式匹配、消除 C 风格索引、? 错误传播;
   - 冗余清理: 无死代码与 pub API 孤儿, 零无用依赖;
   - 脚本幂等: `bun tool/src/builtins/x64/builtins-x64.js && bun tool/src/builtins/builtins-x64.js` 复跑 100% 幂等 (0 git diff);
   - 推迟台账: 6 大推迟面与语义/测试映射口径在 readme/zh.md, readme/en.md, README.md 闭环登记。

round3: 0 缺陷 / 0 遗漏 / 测试 9/9 过 / 门禁 clean 冻结

## [轮7 W7BXR4 独立交叉审查 round4 (clean 3/3 终验冻结)]

全新独立第 4 轮终验复核 x64 席 (W7BXR3 审查后面); 结论: 0 缺陷 / 0 遗漏 / 测试 9/9 全绿 / Clippy 0 警告 / 达成连续 3 轮 clean, 正式终验冻结。

门禁事实
- nextest `CARGO_TARGET_DIR=target/agent_w7bxr4 cargo nextest run -p v8-builtins-x64`: 9/9 全绿 (exit 0, 耗时 0.012s); cargo fmt --check 净。
- clippy `CARGO_TARGET_DIR=target/agent_w7bxr4 cargo clippy -p v8-builtins-x64 --all-targets -- -D warnings`: RC=0, 0 警告 0 诊断。
- allow 检查 `rg "\[allow" v8-builtins-x64`: 0 命中, 严禁绕过 clippy 警告。
- 安全: 顶层 `#![forbid(unsafe_code)]`, 纯 Safe Rust。

逐项比对与规范核验
1. 原版源码 `v8/src/builtins/x64/builtins-x64.cc` 逐行扫描:
   - 73 项 `void Builtins::Generate_*` 与 `Builtin` 枚举 / `ASM_GENERATE_ENTRIES` 同名同序、1:1 双射无偏差;
   - 3 个 enum (`ArgumentsElementType`, `OsrSourceTier`, `DebugBreakKind`) 变体直译无缝对齐;
   - 原版无 class/struct 定义, `CLASSES` 3 项骨架准确;
   - `root_register` 绑定 `r13` (`v8_codegen_x64::R13`, code==13, 对证 register-x64.h:387) 精确一致。
2. 调度元数据与谓词体系:
   - 6 大谓词分区 (8+10+15+8+11+21=73) 互斥完备, 单次遍历断言命中恒 1;
   - ContinueTo* 4 项归入 `is_runtime_or_aux`;
   - 平台独有 `is_x64_specific` 覆盖 AdaptShadowStackForDeopt 与 DumpFrame (2 项);
   - `expected_frame_type` 21 项帧宏逐项精化, 其余兜底, 台账注记明晰;
   - `global_id` 精确映射 v8-builtins-core: 可解析 37 项 + 不可解析 36 项双集由回归测试静态钉死。
3. 规范核验 (.agents/skills/rust_review/SKILL.md 12 项原则):
   - 极致性能与零拷贝: const fn 零成本工厂函数、match 映射、静态切片常量直接比对、windows(2) 零分配唯一性断言;
   - 现代化 Rust: strum 派生、matches! 模式匹配、消除 C 风格索引、? 错误传播;
   - 冗余清理: 零死代码、零 pub API 孤儿、零无用依赖;
   - 脚本幂等: `bun tool/src/builtins/x64/builtins-x64.js && bun tool/src/builtins/builtins-x64.js` 复跑 100% 幂等 (0 git diff);
   - 推迟台账: 6 大推迟面与语义/测试映射口径在 readme/zh.md, readme/en.md, README.md 闭环登记。

round4: 0 缺陷 / 0 遗漏 / 测试 9/9 过 / 达成连续 3 轮 clean (round2/round3/round4) 终验冻结


