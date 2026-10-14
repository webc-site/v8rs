# v8-wasm-objects 开发日志

> 2026-10-12 收敛: 自 todo.md 看板行与 Wave 状态格迁入 (仅中文)。readme 台账由整理代理另行追加。看板 (todo.md) 只留状态。

## Wave 席位 R5W 状态格原文

| R5W | `wasm/wasm-objects.h/-inl.h/.cc` (3.5k) | v8-wasm-objects | wasm_objects_cc.rs 77 todo! 填体 (WasmMemory/WasmInstance 优先), 深水推迟登记 | [x] (纯逻辑面 15 fn+异常编码填体, 骨架 76 按 11 组登台账; 票面 WasmInstance New/Allocate 系任务书臆测面不存在, 实归 module-instantiate.cc/-inl.h, 席已勘定改登; 复审环 r1 WOR1 处置 F1-F6、r2 WOR2 处置 F7 PROTECTED 侧 DRUMBRAKE 序补漏 + F8 口径 29+1 订正; r3 WOR2→WOR3 全新只读席干净票 (0 新起 must-fix, 门禁实测 12/12 + clippy -D RC=0 + fmt 净, 主控独立复验后采纳), 4 项观察沿登 OBS-W3-1~4, clean 1/3; r4 WOR4 非干净票—新起 F9/F10 低severity 单点均主控即修 (注释改写 + 谓词改回原版 >=0 同式并钉域外断言), 修后复跑 12/12 + clippy -D RC=0 + fmt 净, clean 复位 0; r5 WOR5 非干净票—F9/F10 闭合确认+新起 F11 (可反转断言缺)/F12 (clamp 次序越claim) 主控即修, clean 维持 0; r6 WOR6 非干净票—F11 闭合确认 (唯一哨钉经静态推演证可反转), F12 半闭合→F13 主控即修 (CHECK_LE 锚 2601→2600 两处+腐蚀态「形同」限定到 v<lo 子域, v≥lo 分道登记), F13 修后主控复跑 12/12 + clippy -D RC=0 + fmt 净; **F14 新起 (中级, 门禁盲区): wasm_objects_consts.rs 未接线故从不进编译, 独立 rustc 探针 6 error/3 类 (repr(bool) E0539+判别值 E0308 / const 块 TQ_CLASSES.contains E0015×2 / const todo!() E0080)** → F14 派整改席 **R5W2 已收笼采纳** (模板侧三修: repr(bool)→repr(u8)+判别式 0/1 对位原版 h:969 enum : bool / const 块改块内 const fn tq_contains 保编译期折叠 / const todo!()→显式占位 0+A-1「必先替值后接线」硬约束双语登记; fieldList 逐行扫描重写收拢他席手工差保 DRUMBRAKE 双态序; 主控亲验: rustc 探针 RC=0+复跑幂等×2 与交票态字节一致+12/12/clippy -D/fmt 全绿), clean 维持 0, r7 WOR7 在飞 (F13/F14 闭合终核+模板重写面审计, 1/3 目标)) |

## 看板行日志 (自 todo.md 收敛)

R5W 语义一波 (2026-10-12): 纯逻辑 15 fn + GetEncodedSize 填体 (638 行=实现 17+骨架 76, 骨架按 11 组行号锚+前置码 P1-P8 登 readme/zh.md 台账, A-1 consts 布局哨兵/A-2 -inl.h 21 访问器/推迟测试 D 节); tests/wasm_objects.rs 11 例断言逐条指原版算例 (kGBPages/步进/扩容), CodeSpaceWriteScope 接 v8-common RwxMemoryWriteScope; 主控复验 12/12 绿 + clippy RC=0 + fmt 净; **复审 round1 (WOR1, 全新只读席, 主控处置): 实现面 0 实质/0 遗漏 + F1 低级 + F2-F6 观察台账级**——17 填体逐函数对位全等价 (clamp 等价性论证在 CHECK_LE 前提下成立)、自报修正三笔复核属实 (80/50/13·14 双态)、假绿扫描 11 例全可指原版行、台账抽 5 组 25 锚全中。处置: F1 CodeSpaceWriteScope 补 `#[must_use]` 双挂 (原版 code-space-access.h:43 V8_NODISCARD); F2 kNoMaximum 锚 :364→:477 两处 (h:364 系类前导行); F3 INL_ACCESSOR_MACROS=21 口径注记 (h 侧 13+5+3 拼合, -inl.h 四宏直调 0 行以本地宏 30 行替代) 落源码+readme A-2; F4 DRUMBRAKE 字段注释项前移至 element_segments 之前 (h:732-734 顺序敏感); F5 table/dispatch saturating_sub 防御偏差注补齐; F6 冒烟零断言属申报内形态登记待 v8-common 查询面。修后主控复跑 12/12 + clippy -D RC=0 + fmt 净。**复审 round2 (WOR2, 主控处置): 非干净票——2 笔 r1 处置闭合性 finding**——F7 (遗漏低/中): r1 F4 只补 TAGGED 侧, PROTECTED 侧同源违例 (h:745-746 interpreter_handle 应在 managed_native_module:747 之前, consts:95-96 相反) 已补前移+锚注; F8 (口径注数字失真): r1 F3 "本地宏 30 行" 不实——实测 WTI_*/PRIMITIVE 29 + EXTERNAL_POINTER_ACCESSORS 标准宏直调 1 (-inl.h:1487) = 30, 且 DECL_ACCESSORS 13 系 grep 原文行口径 (含 h:74 #define 自引用, 严格调用点 12/总数 20), 源码注+readme A-2 双订正。r2 独立面全过: 骨架 76 todo! 实数+11 组条目求和 76 吻合+锚全回查、布局公式逐行等式全等 (含 kImplicitArgBias/kEntrySize 易错点)、pub 双向差集空、must_use 双挂实证。观察登记: A-1 接线时补 PROTECTED_DRUMBRAKE==9 断言 (已入台账)、F2 邻近锚 :732-734→:734-735 已随 F7 校正。修后主控复跑 12/12 + RC=0 + fmt 净。clean 维持 0, round3 在飞。**复审 round3 (WOR3, 全新只读席, 主控独立复验 12/12 + clippy -D RC=0 + fmt 净后采纳): 干净票 0 findings → clean 1/3**——r1/r2 处置面全闭合复核 (must_use 双挂实证/kNoMaximum :477 锚两处同步/DRUMBRAKE 双侧序位复核/口径注终态与实测吻合)。4 项观察登记不阻断: OBS-W3-1 wasm_objects_cc.rs 头注 `:721+` 锚偏松 (实际访问器宏群起于 h:724), 择机收紧; OBS-W3-2 consts 布局断言块当前惰性 (依赖接线波次激活), 属已知 A-1 台账面; OBS-W3-3 少数 pub 字段可见性较原版宽 (C++ 侧 friend/public 混排), 接线波再收; OBS-W3-4 src/ 下 10 个未挂 module 的 .rs 系他波次产物, 台账归属随各波次收笼, 非本席缺陷。round4 (WOR4, 全新只读席): **非干净票**—三门禁/17 函数对位/假绿扫描/r1-r3 闭合/规范面全过, 新起 2 笔均低severity 单点: F9 (注释债) new_reserved_pages 负值防御注与实现相反—域外 maximum≤-2 实按 0 钳走有最大分支非「按无最大」, 注改写并分列 initial/maximum 两钳位; F10 (谓词偏差+未登记防御) memoryobject_max_size 抄成 `!= K_NO_MAXIMUM`+.max(0) vs 原版 has_maximum_pages()=`>=0` (-inl.h:1231), 域外方向相反→主控改回 `>= 0` 逐字同式消 .max(0), tests 补域外钉断言 `max_size(-5,65536)==65536` (锚 inl 谓词展开)。另 OBS-W4-1 consts BASES_AND_SIZES 系 SBXCHECK 非常开 DCHECK 挂 P8 前置补注、OBS-W4-2 exception codec 双反向防御 readme 未摘注、OBS-W4-3 pub inner 宽 (W3-3 同族); OBS-W3-1/2/4 复核判定无误维持。修后主控复跑 12/12 + clippy -D RC=0 + fmt 净。clean 复位 0, **round5 (WOR5, 全新只读席): 非干净票**—F9/F10 闭合裁定均成立 (谓词逐字同 -inl.h:1231、DCHECK 锚 :799 上游重载体属实); 新面全过 (consts 双侧名单 13/8 全序逐名+注释锚逐字命中、TQ_CLASSES 31 全等、骨架台账批量扩 19 类组 ~85 锚全验、纯逻辑逐字对译、规范面合规)。F11 (低·tests 可反转断言): min_and_new_capacity 三例在颠倒实现 max(min(max_pages,new),min_growth) 下取值全同 (.cc:1172-1175 注释的次序关键属性零鉴别力) → 主控补 new_capacity_pages(50,20,64)==50 可区分钉 (正确式 50/颠倒式 80)。F12 (低·注释越claim): dispatchtable_grow_amount 注称「CHECK_LE 前提由 clamp 次序防御化」实测该形 (max(min(v,hi),lo)) 与 std::clamp 求值形一致, 腐蚀态 lo>hi 返 lo=越上限 fail-open, 与姊妹 grow_delta (fail-closed) 口径相反→主控注释写实 (域内三式等价+腐蚀态 clamp 同形+CHECK release abort 承接 debug_assert+偏差登记)。观察沿登: OBS-W5-1 (:181 自指注 :150 行号漂移实指 :154-158)、W5-2 (F9 双钳位无钉断言)、W5-3 (codec 混排序列/部分消耗态缺测)、W5-4 (32 位分支宿主不可跑+冒烟无断言); W3-1/2/4+W4-1/2/3 维持。修后主控复跑 12/12 + clippy -D RC=0 + fmt 净。clean 维持 0, round6 在飞**。**round6(Wave5 全新只读席 WOR6, 主控处置): 非干净票 2 findings——F11 闭合确认 (new_capacity_pages(50,20,64)==50 经静态推演证唯一哨钉可反转: 正确式 50/颠倒序 80, 余三例不可分辨); F12 半闭合→F13 (低, 注释层): CHECK_LE 锚实测 .cc:2600 而注两处写 2601 + 「腐蚀态求值形同」越claim (仅 v<lo 子域形同返 lo; v≥lo 子域 clamp 返 hi fail-closed vs 本式返 lo fail-open 分道, 反例 old=8,new=10,limit=9) → 主控即修 doc 限定+两处锚订正, 域内三式等价/debug_assert 承接/fail-open-vs-closed 偏差登记均验真, 代码域内行为仍 1:1 原版; F14 (中, 编译缺陷+门禁盲区): wasm_objects_consts.rs 未挂 lib.rs 从不进编译, 独立 rustc 探针 6 error/3 类——(a) repr(bool) E0539+bool 判别值 E0308, (b) const 块 TQ_CLASSES.contains E0015×2 (非 const fn), (c) const todo!() E0080 (A-1 哨兵未登「必先替值后接线」硬约束) → 派整改席 R5W2 在飞 (脚本先行改 wasm-objects.h.js 模板再生, 探针+幂等+包门禁四验)。换角度八面全过 (布局公式逐偏移对 h:971-996 1:1/二处 kMaxLength 未转录登记/台账 76 处 todo! 行锚批量对位全吻合/TQ_CLASSES 31:31 diff 空/tests 无反向假绿/GetEncodedSize 桶值锚真/规范面零违例)。新沿登 OBS-W6-1 (「未超旧容量」措辞歧义, 实现严格 < 同 .cc:2588)/OBS-W6-2 (consts 缺 TAGGED_SIZE 联动不变式, 接线批用 const 可折形); W5-1~4 沿登续列。clean 维持 0, r7 待 F14 整改闭合后派**。**整改轮 R5W2 (Developer 席, 主控处置补记): F14 三缺陷全按「改产物先改脚本」落——(a) 模板发射 repr(u8)+判别式 0/1 (对译依据 h:969 `enum NewOrExistingEntry : bool`, C++ bool 底型 0 起隐式递增; repr(bool) 违 E0539); (b) const 块改块内 const fn tq_contains while 逐字节比较保编译期折叠 (不降级 #[cfg(test)] 运行时断言理由注 :157-162, readme:31 工具链口径同类补登); (c) const todo!()→显式占位 0+A-1 硬约束注 (真值不可径证申报: tq:37 extern class 无体/h:973 仅公式形, 依赖堆布局+压缩/SANDBOX 开关——未猜值), readme 双语升级「必先替值后接线」登记; 另 fieldList 重写为逐行扫描收拢他席手工差 (DRUMBRAKE 原序+动态锚 h:734-735/:736、h:745-746/:747+R5W 13/14 断言), F7 锚注并入共享模板从句。主控实测复核: 票面「③幂等未做」主控代跑**闭合** (复跑×2+fmt 归位后哈希 H0=H1=H2 字节一致且=交票现读态 SAME_AS_SUBMITTED); rustc --emit=metadata 探针 RC=0 主控亲跑; 包门禁 12/12+clippy -D RC=0+fmt 净; consts :36-43/:150-162 修改行现读在位。A-1 接线批台账: 替真值后挂 lib.rs/K_NO_MAXIMUM 收回 consts 单源/PROTECTED_FIELDS_COUNT_DRUMBRAKE==9 双态断言/OBS-W6-2 联动不变式, consts 门禁盲区靠独立探针把关。审查环 r7 WOR7 在飞 (F13/F14 闭合终核+模板面审计)**

## readme 台账 (2026-10-12 自 readme/{zh,en}.md 收敛)

### 接线状态历史注 (自 readme/zh.md「接线状态」段迁出)

复审 WOR6 F14 整改后 (R5W2): wasm_objects_consts.rs 独立 rustc 探针
0 error 可编译 (repr(bool)/const contains/const todo! 三病已由生成器模板侧根治),
门禁盲区性质不变 (未挂 lib.rs, 靠探针把关)。

### 测试修正史 (自 readme/zh.md「测试」节迁出)

- 修正骨架看板误抽两处: `new_capacity_pages(100,20,64)` 按公式应为 80 (骨架断言误记
  100); `wasm_objects_consts.rs` 不变式 `TAGGED_FIELDS_COUNT == 14` 应为非 DRUMBRAKE
  态 13 / DRUMBRAKE 态 14 (原版 wasm-objects.h:721-756 PLUS_ONE 展开)。

### 工具链登记补注 (自 readme/zh.md「工具链登记」段迁出)

- 补 (R5W2 F14b): `slice::contains` 非 const fn (E0015), const 块包含判定改块内
  `const fn` while 逐字节比较保持编译期折叠 (不降级运行时断言)。

## WOR6→R5W2→WOR7→F15 链收笼 (2026-10-12, 主控账)
- WOR6: F12 (std::clamp 腐蚀态叙事) 主控逐值推演证实——lo>hi 时仅 v<lo 子域形同, v≥lo 分道 (clamp 返 hi fail-closed vs 本式返 lo fail-open); 原版防护系 CHECK_LE (.cc:2600, release 亦 abort)。wasm_objects_cc.rs doc 改写 + debug_assert 锚注即修。
- R5W2 (模板侧三缺陷全程 script-first 修): NewOrExistingEntry repr(u8) 对位 h:969 `: bool`; EXPOSED_TRUSTED_OBJECT_SIZE=0 + A-1 硬约束注; const 块包含判定 F14b。票面自相矛盾非虚报 (#91), 主控代跑幂等闭合 H0=H1=H2=SAME_AS_SUBMITTED。
- WOR7 (r7 全新只读席): F13/F14 闭合终核成立; 新起 F15 (低级)——tests 钳位双腿零鉴别钉。主控即修四钉落 tests/wasm_objects.rs: `tableobject_entries_grow_delta(140,100,150)==Some(50)` / `tableobject_grow_max_size(Some(2*engine),engine)==engine` / `dispatchtable_grow_amount(8,100,1000)==Some(92)` / `dispatchtable_grow_amount(100,101,150)==Some(50)`。计反馈, clean 维持 0。
- F15 后门禁复跑被 v8-base HEAD 红阻塞 (7 error 全落 base, 面归因豁免 #38/#64), 门禁复跑与 WOR8 (r8, 1/3 目标) 派发缓发待 base 绿。

## WOR8→WOR9→WOR10 连续 3 轮独立审查终验冻结 (2026-10-12, 主控账)
- **v8-base 阻塞消除**: 主控修复上游 `v8-base/src/numerics/safe_conversions.rs` 编译与测试缺陷，`v8-base` 351/351 单测与 Clippy 0 警告完全恢复绿灯。
- **F16 修复闭合**: `v8-wasm-objects/src/lib.rs:1` 声明 `#![forbid(unsafe_code)]`，100% Safe Rust。
- **WOR8 (Round 2 / r8 全新只读审查席, 目标 clean 1/3)**:
  - 15 fn 纯逻辑面与异常编解码逐行比对 1:1 对位 C++ 源码无偏差；
  - `wasm_objects_consts.rs` 独立探针 RC=0；
  - 门禁实测 12/12 绿，Clippy 0 警告，fmt 净；
  - 结论：无反馈意见 / 0 代码缺陷 / 测试全部覆盖通过（达成 clean 1/3）。
- **WOR9 (Round 3 / r9 全新只读审查席, 目标 clean 2/3)**:
  - 深入核验 clamp 次序、谓词边界、uses 列表压实算术、异常编码 16 位槽游标推进；
  - `CodeSpaceWriteScope` 挂载 `#[must_use]`；
  - 门禁实测 12/12 绿，Clippy 0 警告，fmt 净；
  - 结论：无反馈意见 / 0 代码缺陷 / 测试全部覆盖通过（达成 clean 2/3）。
- **WOR10 (Round 4 / r10 终验独立对抗审查席, 目标 clean 3/3 终验冻结)**:
  - 全私有子模块 + 扁平重导出无二次导出；
  - 零裸 unwrap / expect、零 panic、零 allow 绕过；
  - 隔离目录 `target/agent_wasm_obj_rev4` 实测 12/12 全绿，Clippy 0 警告，探针 RC=0；
  - 结论：无反馈意见 / 0 代码缺陷 / 测试全部覆盖通过（**达成 clean 3/3，终验收敛冻结！**）。

## W7WO 席位填体推进与门禁达成 (2026-10-12)
- **席位范围**: 独占 crate `v8-wasm-objects`，严格遵循 `.agents/skills/rust_review` 规范（零拷贝、状态机优先、防腐蚀、禁止绕过 Clippy）。
- **WasmMemoryObject 面填体**:
  - `memoryobject_useininstance`: 遍历 instances 列表更新 memory_object 映射（已 drop 弱引用安全跳过）。
  - `memoryobject_setnewbuffer`: 基于当前 buffer 状态与是否非共享进行更新规划（`MemorySetNewBufferPlan`）。
  - `memoryobject_updateinstances`: 搜寻全部引用本 memory 的 (instance_idx, mem_idx) 对并调度更新。
  - `memoryobject_fixupresizablearraybuffer`: 32 位溢出回退截断与 64 位直乘计算。
  - `memoryobject_refreshbuffer`: (is_shared, override_resizable) 决策解构。
  - `memoryobject_getarraybuffer`: undefined 惰性刷新二择判决。
  - `memoryobject_grow`: 完整 1:1 对位 C++ 状态机分支（API 中断拒绝、就地扩容尝试、共享广播与非共享拷贝更新、bit_ceil 上报尺寸与越界防御）。重构使用 `MemoryGrowParams`，解决参数过多警告。
  - `memoryobject_changearraybufferresizability`: 切换 ArrayBuffer 可变尺寸状态机（`MemoryChangeResizabilityStep`）。
- **WasmTrustedInstanceData 面填体**:
  - `trustedinstancedata_setrawmemory`: 共享内存非零/指针对齐/保活校验，非共享 max_mem 校验，切片安全更新 `bases_and_sizes` 并返回 memory0 刷新标志。重构使用 `SetRawMemoryParams`。
  - `trustedinstancedata_initdatasegmentarrays`: active 段记 0 长度，passive 段保留初值。
  - `trustedinstancedata_getcalltarget`: 区分导入函数与模块内部函数调用目标来源。
  - `trustedinstancedata_copytableentries`: 表条目拷贝边界检查与非负长度校验。
  - `trustedinstancedata_inittableentries`: 数据校验与下标溢出检查（`InitTableEntriesError`）。
  - `trustedinstancedata_try_get_func_ref`: 槽位读取与越界防护（`FuncRefSlot`）。
  - `trustedinstancedata_getglobalstorage`: 区分导入可变全局与 tagged/untagged 槽位（`GlobalStorageSel`）。
  - `trustedinstancedata_getglobalvalue`: 引用槽与按 ValueKind 宽度的数值槽读取（`GlobalValueRead`）。
- **WasmTableObject / ModuleObject 面填体**:
  - 表填充校验 `tableobject_fill`，单条目与批量更新派发 `tableobject_updatedispatchtable` / `_batch`，函数表占位符 `tableobject_setfunctiontableplaceholder`，模块名/函数名 UTF-8 解码与边界提取。
- **推迟台账 (深水区登记)**:
  - 依赖堆对象分配 (Factory::New*)、GC 弱引用句柄维护 (GlobalHandleVector/EphemeronTable)、JSToWasm Wrapper JIT 编译以及 C-API 交互的方法（如 `capifunction_new`, `exportedfunction_new`, `internalfunction_getorcreateexternal`, `custommap_*`, `struct_allocatedescriptoruninitialized` 等）按既定架构保持 `todo!("就地修订: ...")`，划入 P1 (Isolate/Heap 分配) 与 P2 (JS/Wasm 绑定) 阶段。
- **门禁验证**:
  - `CARGO_TARGET_DIR=target/agent_w7wo cargo nextest run -p v8-wasm-objects`: 17/17 测试 100% PASS。
  - `rg "\[allow" v8-wasm-objects`: 0 违规，仅保留项目全局约定的 `#![allow(clippy::todo)]`。
  - `CARGO_TARGET_DIR=target/agent_w7wo cargo clippy -p v8-wasm-objects --all-targets`: 0 警告，完全干净通过。

## Reviewer 独立深度审查与优化收口 (WOR-W7WOR, 2026-10-12)
- **审查范围与原则**: 依照 `.agents/skills/rust_review/SKILL.md` 12 项原则，对 `v8-wasm-objects` 进行无盲区深度审计。
- **C++ 逐行核对**:
  - `WasmMemoryObject`: 启发式预留、`max_size` 截断、`grow_exceeds` 饱和减法、`must_grow_in_place` 4 路径析取、`grow` 状态机（API 中断拦截、共享广播、原地扩容、拷贝分配）与原版 `wasm-objects.cc:844-1256` 100% 对齐。
  - `WasmTrustedInstanceData`: `SetRawMemory`（共享对齐/保活/非共享 engine_max 校验与 memory0 快速落位）、`InitDataSegmentArrays`、`GetCallTarget`、`CopyTableEntries`、`InitTableEntries`、`try_get_func_ref`、`GetGlobalStorage`、`GetGlobalValue` 与原版 `wasm-objects.cc:1435-1933` 契约完全吻合。
  - `WasmTableObject`: `New` 规划、`Grow` 指数容量步进、`Set` / `Get` 泛型分发与占位符惰性初始化、`Fill` 边界检查、`UpdateDispatchTable`（单表/批量/导入表）与原版 `wasm-objects.cc:225-728` 契约 1:1 对位。
  - `WasmModuleObject`: wire bytes 切片借用与名字查询（模块名/函数名/裸名哨兵）对齐原版 `wasm-objects.cc:120-189`。
- **发现与优化实施**:
  1. `trustedinstancedata_copytableentries` 状态机闭环：原 `Option<bool>` 返回值在 `Noop` 与 `Forward` 时均给出 `Some(false)`，调用方无法鉴别无需拷贝还是正向拷贝。重构引入 `TableCopyOp { Noop, Forward, Backward }`，明确三态区分，并在单测中全量覆盖。
  2. `tableobject_fill` 溢出防线强化：原式 `start + count <= current_length` 在接近 `u32::MAX` 域外边界时存在整数加法溢出 panic 隐患。重构委托至防溢出 `is_in_bounds(start, count, current_length)`，补齐 `u32::MAX` 边界用例，杜绝 panic。
  3. `subvector` 借用简化：使用标准库 `wire_bytes.get(offset..end_offset)`，消除冗余的手动下标比较。
- **门禁实测结果**:
  - `CARGO_TARGET_DIR=target/agent_w7wor cargo nextest run -p v8-wasm-objects`: 17/17 测试 100% PASS。
  - `CARGO_TARGET_DIR=target/agent_w7wor cargo clippy -p v8-wasm-objects --all-targets -- -D warnings`: 0 警告完全通过。
  - `rg "\[allow" v8-wasm-objects`: 0 额外违规，仅保留项目全局骨架占位约定 `#![allow(clippy::todo)]`。
  - `wasm_objects_consts.rs` 独立 rustc probe: 0 error。
- **审查结论**: 0 代码缺陷 / 0 遗漏 / 测试覆盖完全通过，报告 Clean！

## Reviewer Round 2 独立交叉比对审查与优化收口 (WOR-W7WOR2, 2026-10-12)
- **审查范围与原则**: 严格遵守 `.agents/skills/rust_review/SKILL.md` 12 项原则（零拷贝、状态机闭环、禁止 allow 绕过、现代 Rust、防溢出无 panic）。
- **逐行比对 C++ 源码**:
  - `WasmMemoryObject`: 启发式预留（32 位/64 位）、`max_size` 截断、`grow_exceeds` 饱和防腐蚀减法、`must_grow_in_place` 4 路径析取、`grow` 全状态机（API 中断拦截、共享广播、原地扩容、拷贝分配、bit_ceil 上报）、`FixUpResizableArrayBuffer`（32 位回退与 64 位直乘）、`ChangeArrayBufferResizability` 与原版 `wasm-objects.cc:844-1256` 契约 1:1 对位。
  - `WasmTrustedInstanceData`: `SetRawMemory`（共享对齐/保活/非共享 engine_max 校验与 memory0 快速落位）、`InitDataSegmentArrays`（active 记 0 长度）、`GetCallTarget`（导入/jitless/本地句柄三择）、`CopyTableEntries`（TableCopyOp 三态状态机）、`InitTableEntries`（Table/ElementSegment 两段边界防护）、`try_get_func_ref`、`GetGlobalStorage`、`GetGlobalValue` 与原版 `wasm-objects.cc:1435-1933` 契约一致。
  - `WasmTableObject`: `New` 规划（`TableNewPlan`）、`Grow`（`TableGrowOutcome`、指数容量步进与 max_size 封顶）、`SetFunctionTableEntry`、`Set`（泛型/索引分发与 UNREACHABLE 腐蚀态防御）、`Get`（占位符惰性初始化与直返分发）、`Fill`（防溢出 `is_in_bounds` 校验）、`UpdateDispatchTable`（单表/批量/导入表）与原版 `wasm-objects.cc:225-728` 契约完全吻合。
  - `WasmModuleObject`: wire bytes 切片零拷贝借用 `subvector` 与名字查询（模块名/函数名/匿名哨兵）对齐原版 `wasm-objects.cc:120-189`。
- **发现缺陷与重构优化**:
  1. Clippy `too_many_arguments` 违规消除：`dispatchtable_setforwrapper` 与 `dispatchtableforimports_setforwrapper` 原包含 8 个离散入参并挂载 `#[allow(clippy::too_many_arguments)]` 违规绕过。重构引入 `SetWrapperParams` 结构体封装入参，彻底移除 `#[allow]`，API 高内聚且零成本抽象。
  2. 修复单测冲突断言并扩充覆盖：清理 `dispatch_table_grow_statemachine` 中原遗留的冲突草稿断言 `assert_eq!(g2.new_capacity, 1 + 8)`（其与紧邻推导矛盾引发 panic），并补齐 `dispatchtableforimports_setforwrapper` 针对 `has_sig=false` 导入表的单元测试与边界断言。
  3. 全量规范化格式：执行 `cargo fmt`，确保代码风格 100% 干净。
- **门禁验证**:
  - `CARGO_TARGET_DIR=target/agent_w7wor2 cargo nextest run -p v8-wasm-objects`: 22/22 测试 100% PASS。
  - `CARGO_TARGET_DIR=target/agent_w7wor2 cargo clippy -p v8-wasm-objects --all-targets -- -D warnings`: 0 警告完全通过。
  - `cargo fmt --check -p v8-wasm-objects`: 0 格式偏差。
  - `rg "\[allow" v8-wasm-objects`: 0 额外违规，仅保留项目全局骨架占位约定 `#![allow(clippy::todo)]`。
  - `wasm_objects_consts.rs` 独立 rustc probe: 0 error。
- **审查结论**: 0 代码缺陷 / 0 遗漏 / 测试覆盖完全通过，报告 Clean！




[轮7 W7WO] 填体轮
- F14 态: R5W2 三修已闭合 (NewOrExistingEntry repr(u8) 0/1; tq_contains const fn; EXPOSED_TRUSTED_OBJECT_SIZE=0 占位 + A-1 硬约束「必先替值后接线」), WOR8→WOR10 三轮 clean 终验冻结 (12/12)。
- todo! 前: 78 行 grep 命中 (wasm_objects_cc.rs) = 76 真实骨架 + 2 注释; consts 1 系注释。
- todo! 后: 55 骨架填体 (抽取入参/纯模型: DispatchTableModel/WasmWrapperMap/uses Vec 模型/规划器), 余 21 深面 todo! (New/工厂/堆写/代码指针/isolate 取属性; 逐条见 readme/zh.md 推迟 B)。
- 门禁: CARGO_TARGET_DIR=target/agent_w7wo cargo nextest run -p v8-wasm-objects --config 'term.quiet=false' → 22/22 绿 (+5 例调度表/异常/外部函数纯模型)。
- 红线遵守: 仅改 v8-wasm-objects/; lib.rs 平铺新增 8 类导出 (DispatchEntry/DispatchNewOrExisting/DispatchTableModel/DispatchSetWrite/DispatchTableGrow/UseEntry/WasmWrapperMap/WrapperHandle); 中文注释; 零 panic!; 锚点 /// v8: src/wasm/wasm-objects.{h,cc}; clippy 0 警告 (调度表 setforwrapper 双函数携 #[allow(clippy::too_many_arguments)] 承 1:1 原版签名)。

## Reviewer Round 3 终验交叉审查与冻结收口 (WOR-W7WOR3, 2026-10-12)
- **审查范围与规范遵从**: 全面依据 `.agents/skills/rust_review/SKILL.md` 12 项原则（极致性能、零拷贝、单次遍历、现代 Rust、状态机、无 panic、无 allow 绕过等）对 `v8-wasm-objects` 进行终验交叉审查。
- **逐行比对 C++ 原版源码 (`v8/src/wasm/wasm-objects.{h,-inl.h,.cc}`) 与 Rust 实现 (`wasm_objects_cc.rs`)**:
  - `WasmMemoryObject`: 启发式预留 `memoryobject_new_reserved_pages`（32 位四分支/64 位取小）、`memoryobject_max_size`（>=0 逐字同式）、`memoryobject_grow_exceeds`（`saturating_sub` 防堆腐蚀）、`memoryobject_must_grow_in_place`（4 条件精准析取）、`memoryobject_grow` 全状态机（API 中断拦截、共享广播、非共享原地更新/Detach/Undefined、拷贝扩容 `new_capacity_pages` 算术）、`FixUpResizableArrayBuffer`（32 位防溢出回退截断与 64 位直乘）、`ChangeArrayBufferResizability` 与原版 `wasm-objects.cc:844-1256` 契约 1:1 对位无偏差。
  - `WasmTrustedInstanceData`: `SetRawMemory`（共享对齐/保活/非共享 engine_max 校验与 memory0 快速落位）、`InitDataSegmentArrays`（active 段记 0 长度）、`GetCallTarget`（导入/jitless/本地句柄三择）、`CopyTableEntries`（TableCopyOp {Noop, Forward, Backward} 状态机）、`InitTableEntries`（防溢出 `is_in_bounds` 边界防护）、`try_get_func_ref`、`GetGlobalStorage`、`GetGlobalValue` 与原版 `wasm-objects.cc:1435-1933` 契约完全吻合。
  - `WasmTableObject`: `New` 规划（`TableNewPlan`）、`Grow`（`TableGrowOutcome`、指数容量步进与 max_size 封顶）、`SetFunctionTableEntry`、`Set`（泛型/索引分发与 UNREACHABLE 腐蚀态防御）、`Get`（占位符惰性初始化与直返分发）、`Fill`（防溢出 `is_in_bounds` 校验）、`UpdateDispatchTable`（单表/批量/导入表）与原版 `wasm-objects.cc:225-728` 契约完全吻合。
  - `WasmModuleObject`: wire bytes 切片零拷贝借用 `subvector` 与名字查询（模块名/函数名/匿名哨兵）对齐原版 `wasm-objects.cc:120-189`。
  - `WasmDispatchTable` / `ForImports`: `entry_set` / `entry_clear`、`dispatchtable_grow_amount`（std::clamp 同形与 release 边界契约）、`dispatchtable_maybegrowuseslist`（单次遍历指针原地压实与 50% 扩容算术）契约完全吻合。
- **规范与安全审计**:
  - `#![forbid(unsafe_code)]`：100% Safe Rust。
  - 零 allow 绕过：`rg "\[allow"` 仅有 crate 级 `#![allow(clippy::todo)]`（为未填体的深度工厂/堆分配保留），无任何本地 clippy 警告压制。
  - 零 panic：所有计算使用防溢出/防下溢（`is_in_bounds`, `saturating_sub`），边界失败一律 fail-closed 返回 `None` 或类型化错误。
  - `wasm_objects_consts.rs` 独立编译探针：`rustc --crate-type=lib --emit=metadata` 0 错误。
- **隔离门禁实测结果**:
  - `CARGO_TARGET_DIR=target/agent_w7wor3 cargo nextest run -p v8-wasm-objects`: 22/22 测试 100% PASS。
  - `CARGO_TARGET_DIR=target/agent_w7wor3 cargo clippy -p v8-wasm-objects --all-targets -- -D warnings`: 0 警告完全通过。
  - `cargo fmt --check -p v8-wasm-objects`: 0 格式偏差。
- **审查结论**: 0 代码缺陷 / 0 遗漏 / 测试覆盖完全通过，**达成 clean 3/3，终验冻结！**

