#!/usr/bin/env bun
// tool/src/regexp/experimental/experimental-interpreter.cc.js
//   — v8/src/regexp/experimental/experimental-interpreter.cc → v8-regexp-exec/src/experimental/interpreter.rs
// 规则: 全集逐项登记 —— 台账区间必须无缝覆盖 cc:1-1607 (机械对账禁漏项), 每区间以
//   原版行内容锚点核验 (行号漂移即 throw)。登记本仓库实况 (新版时钟广度优先 NFA 引擎,
//   任务书旧引擎描述 BitVector/PositionRanges/Bytecode::Execute 不存在于本文件 —
//   主循环类比映射 RunActiveThread cc:886-1115, 数据结构族映射 FilterGroups/
//   InterpreterThread/LastInputIndex/LookaroundInfo/常量)。
//   Execute 主循环 (RunActiveThread) 按批段切 3 段登记: EXP-3b 线程调度+状态机转移
//   (EXP-3b 席位已落地, 含调度外围 RunActiveThreads/RunActiveThreadsToEnd/FindNextMatch
//   与 FORK/ACCEPT 内联子调用接线探针) / EXP-3c 字符消费+谓词绑定 (推迟) /
//   EXP-3d 环视表+捕获过滤+内存治理 (推迟) ; 入口/中断/裸字节面 → EXP-4 (推迟)。
//   常量全集数值实测核对: cc:21-23 (-1/-1/uint64 -1→u64::MAX) + cc:733 (64)。
// 产物: v8-regexp-exec/src/regexp/experimental/experimental-interpreter.cc.rs.skel (全集台账快照)
//   + 落地对数核对 (interpreter.rs/tests 逐项命中, 缺项 throw 列票单)
import { readFileSync } from "node:fs";
import { join } from "node:path";
import { banner, readV8, root, writeSrc } from "../../../lib.js";

const cc = readV8("src/regexp/experimental/experimental-interpreter.cc");

let assertCount = 0;
function must(cond, msg) {
  assertCount++;
  if (!cond) throw new Error(`对数失配: ${msg}`);
}
const count = (src, re) => (src.match(re) || []).length;
const ccLines = cc.split("\n");
must(ccLines.length === 1608 || ccLines[1607] === "", `cc 行数 1607 (含尾空行), 实得 ${ccLines.length}`);
const seg = (from, to) => ccLines.slice(from - 1, to).join("\n");

// ── 常量全集数值实测核对 (任务书 12.2 常量类) ───────────────────────────────
must(seg(21, 21).includes("constexpr int kUndefinedRegisterValue = -1"), "cc:21 kUndefinedRegisterValue=-1");
must(seg(22, 22).includes("constexpr int kUndefinedMatchIndexValue = -1"), "cc:22 kUndefinedMatchIndexValue=-1");
must(seg(23, 23).includes("constexpr uint64_t kUndefinedClockValue = -1"), "cc:23 kUndefinedClockValue=-1(uint64 回绕全 1)");
must(seg(733, 733).includes("static constexpr int kTicksBetweenInterruptHandling = 64"), "cc:733 kTicks=64 (函数内 static constexpr)");

// ── 全集台账 (区间, 名称, 种类, 处置, 行内容锚点正则) ────────────────────────
const A = "EXP-3a 骨架票落地", B = "EXP-3b 已落地(线程调度+状态机转移)", C = "EXP-3c 字符消费+谓词绑定",
  D = "EXP-3d 环视表+捕获过滤+内存治理", E = "EXP-4 入口面", ADM = "登记不落地 (样板区)";
const LEDGER = [
  [1, 15, "license + include 块", ADM, /#include "src\/regexp\/experimental\/experimental-interpreter\.h"/],
  [16, 20, "namespace v8/internal/regexp + 匿名 namespace 开块", ADM, /^namespace \{$/m],
  [21, 23, "常量 ×3 (kUndefinedRegister/MatchIndex/ClockValue)", A, /kUndefinedClockValue/],
  [24, 61, "SatisfiesAssertion<Character> 自由函数 (7 断言臂)", A, /bool SatisfiesAssertion\(Assertion::Type/],
  [62, 71, "ToInstructionVector (裸字节→Vector<Instruction>)", E, /base::Vector<Instruction> ToInstructionVector\(/],
  [72, 96, "ToCharacterVector 模板 + uint8_t/uc16 特化 ×2", E, /base::Vector<const Character> ToCharacterVector\(/],
  [97, 122, "FilterGroups 类头 + 静态 Filter 入口", A, /static base::Vector<int> Filter\(/],
  [123, 141, "FilterGroups 私有构造 + Up()", A, /void Up\(\) \{/],
  [142, 156, "IncrementPC() + IsAtNodeEnd()", A, /void IncrementPC\(\) \{/],
  [157, 238, "FilterGroups::Run 显式栈遍历主体", A, /base::Vector<int> Run\(base::Vector<int> registers_/],
  [239, 251, "FilterGroups 字段区 (pc_/max_clock_/双 ZoneStack/bytecode_)", A, /ZoneStack<uint64_t> max_clock_stack_;/],
  [252, 347, "NfaInterpreter 类头 + 架构注释 (时钟 BFS/优先级/双轨)", A, /^class NfaInterpreter \{$/m],
  [348, 479, "NfaInterpreter 构造器 (扫描段 A / Isolate·ToXxxVector 前段 E / 分配器 D)", `${A}+${E}+${D}`, /NfaInterpreter\(Isolate\* isolate, RegExp::CallOrigin call_origin,/],
  [480, 539, "FindMatches 成员 (主驱动, 驱动 find_next_match 面 EXP-3b 已落地)", E, /V8_WARN_UNUSED_RESULT int FindMatches\(int32_t\* output_registers,/],
  [540, 589, "InterpreterThread 嵌套类 + ConsumedCharacter 枚举", A, /class InterpreterThread \{/],
  [590, 635, "FillLookaroundTable (反向逐位置预构建环视表)", D, /V8_WARN_UNUSED_RESULT int FillLookaroundTable\(\) \{/],
  [636, 695, "FillLookaroundCaptures (环视捕获回填)", D, /V8_WARN_UNUSED_RESULT int FillLookaroundCaptures\(/],
  [696, 756, "RunActiveThreadsToEnd (时钟推进主驱动, kTicks=64 接线 HandleInterrupts 探针)", B, /V8_WARN_UNUSED_RESULT int RunActiveThreadsToEnd\(\) \{/],
  [757, 817, "HandleInterrupts (StackLimitCheck/Isolate 栈守护; 本票接线探针)", E, /V8_WARN_UNUSED_RESULT int HandleInterrupts\(\) \{/],
  [818, 830, "SetInputIndex", A, /void SetInputIndex\(int new_input_index\) \{/],
  [831, 885, "FindNextMatch (逐起始位重启动机, 含环视线程挂起)", B, /V8_WARN_UNUSED_RESULT int FindNextMatch\(\) \{/],
  [886, 907, "主循环 preamble: SBXCHECK 边界/++clock/去重短路/取指 (Execute 批段 1/3)", B, /\+\+clock;/],
  [908, 920, "主循环臂: CONSUME_RANGE·RANGE_COUNT 挂起 + ASSERTION 求值 (Execute 批段 2/3, EXP-3c 已落地)", C, /case Instruction::CONSUME_RANGE:/],
  [921, 1019, "主循环臂: FORK·JMP·ACCEPT·SET_QUANTIFIER·CLEAR_REGISTER·SET_REGISTER_TO_CP (Execute 批段 3/3a)", B, /case Instruction::FORK: \{/],
  [1020, 1024, "主循环臂: FILTER_* ×4 UNREACHABLE 防御 (原版四 case 折叠, Rust 加固 BadBytecode)", B, /case Instruction::FILTER_LOOKAROUND:/],
  [1025, 1055, "主循环臂: BEGIN_LOOP·END_LOOP·START_LOOKAROUND·END_LOOKAROUND", B, /case Instruction::BEGIN_LOOP:/],
  [1056, 1119, "主循环臂: WRITE/READ_LOOKAROUND_TABLE (双轨表读写+命中登记, EXP-3d 已落地)", D, /case Instruction::WRITE_LOOKAROUND_TABLE:/],
  [1120, 1131, "RunActiveThreads (弹出-驱动循环)", B, /V8_WARN_UNUSED_RESULT int RunActiveThreads\(\) \{/],
  [1132, 1173, "FlushBlockedThreads (逆序唤醒/范围比对/销毁)", A, /void FlushBlockedThreads\(base::uc16 input_char\) \{/],
  [1174, 1175, "FoundMatch (best_match_thread 在场)", A, /bool FoundMatch\(\) const/],
  [1176, 1182, "ApproximateTotalMemoryUsage (预算×线程数)", A, /size_t ApproximateTotalMemoryUsage\(\) \{/],
  [1183, 1209, "CheckMemoryConsumption (64MB 治理阈值面; 本票 FORK 接线探针)", D, /int CheckMemoryConsumption\(\) \{/],
  [1210, 1242, "Get*Array getter ×5 (指针→Vector 包装)", `${A} (Vec 字段直读折叠)`, /base::Vector<uint64_t> GetLookaroundClockArray\(/],
  [1243, 1336, "New/Free 数组族 ×15 (RecyclingZoneAllocator 分配回收)", D, /void FreeLookaroundClockArray\(/],
  [1337, 1356, "NewEmptyThread (哨兵/0 填充工厂)", A, /InterpreterThread NewEmptyThread\(int pc\) \{/],
  [1357, 1379, "NewUninitializedThread (FORK 专用工厂)", A, /InterpreterThread NewUninitializedThread\(int pc\) \{/],
  [1380, 1412, "GetFilteredRegisters (FilterGroups 调用接线)", D, /V8_WARN_UNUSED_RESULT int GetFilteredRegisters\(/],
  [1413, 1426, "DestroyThread (五数组回收)", `${A} (Rust=drop)+${D} (回收链)`, /void DestroyThread\(InterpreterThread t\) \{/],
  [1427, 1452, "IsPcProcessed (去重查询, 含冗余性论证注释)", A, /bool IsPcProcessed\(int pc,/],
  [1453, 1466, "MarkPcProcessed (去重登记)", A, /void MarkPcProcessed\(int pc,/],
  [1467, 1491, "Nfa 字段组 1 (isolate_/call_origin_/no_gc_/句柄/bytecode_/input_/clock)", `${A}+${E} (句柄面)`, /uint64_t clock;/],
  [1492, 1509, "LastInputIndex 嵌套类 + pc_last_input_index_ 字段", A, /class LastInputIndex \{/],
  [1510, 1531, "线程列表字段 + 分配器 ×5 + best_match_thread_", `${A}+${D} (分配器)`, /std::optional<InterpreterThread> best_match_thread_;/],
  [1532, 1546, "LookaroundInfo 结构 + lookarounds_ 字段", A, /struct LookaroundInfo \{/],
  [1547, 1562, "环视真值表字段 ×2 + only_captureless_lookbehinds_", A, /bool only_captureless_lookbehinds_;/],
  [1563, 1578, "reverse_/current_lookaround_/filter_groups_pc_/memory 预算/zone_", `${A}+${E} (zone_)`, /Zone\* zone_;/],
  [1579, 1582, "NfaInterpreter 类闭 + 匿名 namespace 闭", ADM, /^\};$/m],
  [1583, 1604, "ExperimentalRegExpInterpreter::FindMatches 入口 (one/two-byte 分派)", E, /int ExperimentalRegExpInterpreter::FindMatches\(/],
  [1605, 1607, "namespace regexp/internal/v8 闭块", ADM, /^\}  \/\/ namespace v8$/m],
];

// 1) 区间无缝覆盖 1..1607 (全集逐项登记, 禁漏项的机械证明)
{
  let cursor = 1;
  for (const [from, to, name] of LEDGER) {
    must(from === cursor, `台账断点: ${name} 起点 ${from} ≠ 游标 ${cursor}`);
    must(to >= from, `台账区间倒挂: ${name}`);
    cursor = to + 1;
  }
  must(cursor === 1608, `台账尾覆盖不足: 游标 ${cursor} ≠ 1608`);
}
// 2) 每区间行内容锚点核验 (行号漂移即 throw) + 区间行数入账
let landedA = 0, landedB = 0, cCount = 0, dCount = 0, eCount = 0, admCount = 0;
for (const [from, to, name, disp, anchor] of LEDGER) {
  must(anchor.test(seg(from, to)), `台账锚点失配: ${name} (cc:${from}-${to}) 未命中 ${anchor}`);
  if (disp.startsWith(A)) landedA++;
  if (disp.startsWith(B)) landedB++;
  if (disp.includes(C)) cCount++;
  if (disp.includes(D)) dCount++;
  if (disp.includes(E)) eCount++;
  if (disp === ADM) admCount++;
}
// 3) 主方法数对账: 主循环 switch cc:907 的 case 全集 19 (19 opcode 一一 case, 无 default);
//    FilterGroups::Run cc:167 另有 4 case (FILTER_*) — 全文件 23 处对账
must(count(seg(886, 1119), /case Instruction::/g) === 19, `主循环 case 应 19, 实得 ${count(seg(886, 1119), /case Instruction::/g)}`);
must(count(cc, /case Instruction::/g) === 23, `全文件 case 应 23 (19+4), 实得 ${count(cc, /case Instruction::/g)}`);

// ── 产物: 全集台账快照 skel ────────────────────────────────────────────────
const rows = LEDGER.map(([from, to, name, disp]) =>
  `//! cc:${String(from).padStart(4, " ")}-${String(to).padEnd(4, " ")} [${disp}] ${name}`);
const skel = `${banner(
  "src/regexp/experimental/experimental-interpreter.cc.js",
  "src/regexp/experimental/experimental-interpreter.cc",
  "全集台账快照 (审计专用, 区间无缝覆盖 cc:1-1607): 落地以 interpreter.rs 就地修订为准",
)}
//! 本仓库实况 = 新版时钟广度优先 NFA 引擎 (任务书旧引擎描述 BitVector/CharRanges/
//!   PositionRanges/PendingBitsets/ThreadState/Bytecode::Execute 在本文件不存在;
//!   主循环类比映射 RunActiveThread cc:886-1115, 数据结构族映射下方 FilterGroups/
//!   InterpreterThread/LastInputIndex/LookaroundInfo/常量, 按实况登记)
//! Execute 主循环 (RunActiveThread) 批段切分 (实况):
//!   批段 1 线程调度+状态机转移 EXP-3b 已落地 (preamble + 10 实现臂 + FILTER_* 加固
//!   防御臂 + 调度外围 run_active_threads/run_active_threads_to_end/find_next_match;
//!   FORK→CheckMemoryConsumption cc:977-980 与 kTicks=64→HandleInterrupts cc:733-737
//!   为接线探针, 本体各归 EXP-3d/EXP-4) / 批段 2 字符消费+谓词绑定 EXP-3c 已落地
//!   (原推迟臂转实现臂) / 批段 3 环视表+捕获过滤+内存治理 EXP-3d 主循环 2 臂已落地
//!   (WRITE/READ_LOOKAROUND_TABLE; FillLookaroundTable/CheckMemoryConsumption 本体仍推迟)
//! 常量实测核对: kUndefinedRegisterValue=-1(int) kUndefinedMatchIndexValue=-1(int)
//!   kUndefinedClockValue=-1(uint64→回绕 u64::MAX) kTicksBetweenInterruptHandling=64
${rows.join("\n")}
`;
writeSrc("v8-regexp-exec", "experimental/interpreter.cc.rs.skel", skel);

// ── 落地对数核对 (interpreter.rs 数据结构族+主体/调度外围逐项命中) ───────────
const landed = readFileSync(join(root, "v8-regexp-exec", "src", "experimental", "interpreter.rs"), "utf8");
const missing = [];
const landIn = (hay, re, want, label) => {
  const got = count(hay, re);
  if (got !== want) missing.push(`${label}: got=${got} want=${want}`);
  assertCount++;
};
const landMin = (hay, re, label) => {
  const got = count(hay, re);
  if (got < 1) missing.push(`${label}: got=${got} want>=1`);
  assertCount++;
};
const land = (re, want, label) => landIn(landed, re, want, label);
// 常量族 (数值实测: -1/-1/u64::MAX/64 与 cc:21-23/733 对账)
land(/pub const K_UNDEFINED_REGISTER_VALUE: i32 = -1;/g, 1, "Rust 常量 kUndefinedRegisterValue=-1");
land(/pub const K_UNDEFINED_MATCH_INDEX_VALUE: i32 = -1;/g, 1, "Rust 常量 kUndefinedMatchIndexValue=-1");
land(/pub const K_UNDEFINED_CLOCK_VALUE: u64 = u64::MAX;/g, 1, "Rust 常量 kUndefinedClockValue (uint64 -1 回绕)");
land(/pub const K_TICKS_BETWEEN_INTERRUPT_HANDLING: i32 = 64;/g, 1, "Rust 常量 kTicks=64");
// 批段台账常量 (推迟臂 NotImplemented.batch 消费面, 与原版 cc:21 风格同款 constexpr 承接)
land(/pub const EXP_BATCH_3C: &str = "EXP-3c 字符消费\+谓词绑定";/g, 1, "批段常量 EXP-3c");
land(/pub const EXP_BATCH_3D: &str = "EXP-3d 环视表\+捕获过滤\+内存治理";/g, 1, "批段常量 EXP-3d");
land(/pub const EXP_BATCH_4: &str = "EXP-4 入口面";/g, 1, "批段常量 EXP-4");
// FilterGroups 族 6 成员 ↔ cc 98-250 六方法 (Filter/ctor/Up/IncrementPC/IsAtNodeEnd/Run)
for (const [re, label] of [
  [/pub fn filter_groups\(/, "Filter (公开入口折叠 ctor+Run)"],
  [/fn new\(pc: usize, bytecode: &'a \[Instruction\]\)/, "私有构造"],
  [/fn up\(&mut self\)/, "Up()"],
  [/fn increment_pc\(&mut self\)/, "IncrementPC()"],
  [/fn is_at_node_end\(&self\)/, "IsAtNodeEnd()"],
  [/fn run\(\s*&mut self,/, "Run(...)"],
]) land(re, 1, `Rust FilterGroups::${label}`);
// 线程族: ConsumedCharacter 2 变体 + InterpreterThread 7 字段 + 双工厂
land(/pub enum ConsumedCharacter \{\s*\/\/\/.*DidConsume\s*DidConsume,\s*\/\/\/.*DidNotConsume\s*DidNotConsume,\s*\}/g, 1, "Rust ConsumedCharacter 双变体");
land(/pub struct InterpreterThread \{/g, 1, "Rust InterpreterThread");
land(/pub fn new_empty\(/g, 1, "Rust new_empty (NewEmptyThread cc:1340)");
land(/pub fn new_uninitialized\(/g, 1, "Rust new_uninitialized (NewUninitializedThread cc:1360)");
land(/pub struct LastInputIndex \{/g, 1, "Rust LastInputIndex");
land(/pub struct LookaroundInfo \{/g, 1, "Rust LookaroundInfo");
// NfaInterpreter 已落方法面 (EXP-3a 数据结构/谓词/扫描 + EXP-3b 主体与调度外围)
for (const [re, label] of [
  [/pub fn new\(/, "构造器扫描段"],
  [/fn fetch\(&self, pc: i32, orig: &'static str\) -> Result<\(usize, Instruction\)>/, "fetch 取指+SBXCHECK 带下标返回 (cc:888-889)"],
  [/fn fetch_at\(&self, pc: i32, orig: &'static str\)/, "fetch_at flush 侧越查 (cc:1143/1153)"],
  [/pub fn set_input_index\(/, "SetInputIndex"],
  [/pub fn found_match\(/, "FoundMatch"],
  [/pub fn approximate_total_memory_usage\(/, "ApproximateTotalMemoryUsage"],
  [/pub fn is_pc_processed\(/, "IsPcProcessed"],
  [/pub fn mark_pc_processed\(/, "MarkPcProcessed"],
  [/pub fn flush_blocked_threads\(/, "FlushBlockedThreads"],
  [/pub fn run_active_thread\(&mut self, mut thread: InterpreterThread\) -> Result<\(\)> \{/g, "RunActiveThread 主体 (EXP-3b 落地, 按值收线程)"],
  [/fn new_empty_thread\(&self, pc: i32\)/, "new_empty_thread 绑定 (cc:864/868 消费)"],
  [/fn new_uninitialized_thread\(&self, pc: i32\)/, "new_uninitialized_thread 绑定 (cc:922 消费)"],
  [/fn set_best_match\(&mut self, thread: InterpreterThread\)/, "set_best_match ACCEPT/END_LOOKAROUND 共体"],
  [/fn copy_array_slot<T: Copy>\(/, "copy_array_slot FORK 拷贝族 (cc:932-971)"],
  [/fn bounded_index\(/, "bounded_index SBXCHECK_BOUNDS 加固 (cc:1005/1012)"],
  [/fn deferred_arm\(/, "deferred_arm 推迟臂显式失败通道"],
  [/pub fn run_active_threads\(&mut self\) -> Result<\(\)>/, "RunActiveThreads (cc:1120-1127)"],
  [/pub fn run_active_threads_to_end\(&mut self\) -> Result<\(\)>/, "RunActiveThreadsToEnd (cc:697-752)"],
  [/pub fn find_next_match\(&mut self\) -> Result<\(\)>/, "FindNextMatch (cc:831-878)"],
  [/pub fn handle_interrupts\(&self\)/, "HandleInterrupts 接线探针 (本体 EXP-4)"],
  [/pub fn check_memory_consumption\(&self\)/, "CheckMemoryConsumption 接线探针 (本体 EXP-3d)"],
]) land(re, 1, `Rust NfaInterpreter::${label}`);

// EXP-3b 实现臂全集对账 (10 实现臂 match 形 + preamble 关键动作);
//   EXP-3c/3d 批段 5 臂已落地 (原 deferred_arm 推迟臂转实现臂, match 形强断言 — 只增不减)
const LAND_ARMS = [
  [/\(Opcode::Fork, Payload::Pc\(alt_pc\)\)/, "FORK 臂 (cc:921-984)"],
  [/\(Opcode::Jmp, Payload::Pc\(jmp_pc\)\)/, "JMP 臂 (cc:985-987)"],
  [/\(Opcode::Accept, Payload::None\)/, "ACCEPT 臂 (cc:988-998)"],
  [/\(Opcode::SetQuantifierToClock, Payload::QuantifierId\(/, "SET_QUANTIFIER_TO_CLOCK 臂 (cc:999-1002)"],
  [/\(Opcode::ClearRegister, Payload::RegisterIndex\(/, "CLEAR_REGISTER 臂 (cc:1004-1010)"],
  [/\(Opcode::SetRegisterToCp, Payload::RegisterIndex\(/, "SET_REGISTER_TO_CP 臂 (cc:1011-1019)"],
  [/\(Opcode::BeginLoop, Payload::None\)/, "BEGIN_LOOP 臂 (cc:1025-1029)"],
  [/\(Opcode::EndLoop, Payload::None\)/, "END_LOOP 臂 (cc:1030-1040)"],
  [/\(Opcode::StartLookaround, Payload::Lookaround\(_\)\)/, "START_LOOKAROUND 臂 (cc:1041-1043)"],
  [/\(Opcode::EndLookaround, Payload::None\)/, "END_LOOKAROUND 臂 (cc:1045-1055)"],
  [/\(Opcode::ConsumeRange, Payload::ConsumeRange\(_\)\)/, "CONSUME_RANGE 挂起臂 (cc:908-912, EXP-3c 已落地)"],
  [/\(Opcode::RangeCount, Payload::NumRanges\(_\)\)/, "RANGE_COUNT 挂起臂 (cc:908-912 共臂体, EXP-3c 已落地)"],
  [/\(Opcode::Assertion, Payload::Assertion\(assertion\)\)/, "ASSERTION 求值绑定臂 (cc:913-920, EXP-3c 已落地)"],
  [/\(Opcode::WriteLookaroundTable, Payload::LookaroundId\(lookaround_id\)\)/, "WRITE_LOOKAROUND_TABLE 臂 双轨表写 (cc:1056-1071, EXP-3d 已落地)"],
  [/\(Opcode::ReadLookaroundTable, Payload::Lookaround\(lookaround\)\)/, "READ_LOOKAROUND_TABLE 臂 双轨表读 (cc:1072-1112, EXP-3d 已落地)"],
];
for (const [re, label] of LAND_ARMS) land(re, 1, `实现臂 ${label}`);
land(/self\.clock \+= 1;/g, 1, "preamble ++clock (cc:891)");
land(/if self\.is_pc_processed\(pc, thread\.consumed_since_last_quantifier\) \{\s*return Ok\(\(\)\);/g, 1, "preamble 去重短路 (cc:899-902 DestroyThread=drop)");
land(/self\.mark_pc_processed\(pc, thread\.consumed_since_last_quantifier\);/g, 1, "preamble MarkPcProcessed (cc:903)");
land(/主循环混入 FILTER_\*/g, 1, "FILTER_* 防御臂加固 (cc:1020-1024 UNREACHABLE→BadBytecode)");
land(/self\.active_threads\.push\(fork\);/g, 1, "FORK 入队 (cc:975)");
land(/self\.check_memory_consumption\(\)\?;/g, 1, "FORK→CheckMemoryConsumption 接线 (cc:977-980)");
land(/self\.handle_interrupts\(\)\?;/g, 1, "kTicks=64→HandleInterrupts 接线 (cc:733-737)");
land(/self\.input_index % K_TICKS_BETWEEN_INTERRUPT_HANDLING == 0/g, 1, "kTicks 取模 (cc:734)");
land(/while let Some\(thread\) = self\.active_threads\.pop\(\)/g, 1, "RunActiveThreads RemoveLast 弹出序 (cc:1122)");
land(/self\.active_threads\.push\(self\.new_empty_thread\(0\)\);/g, 1, "FindNextMatch 主线程 (cc:864)");
land(/self\.active_threads\.push\(self\.new_empty_thread\(match_pc\)\);/g, 1, "FindNextMatch 环视线程挂起 (cc:866-870)");

// 推迟臂路由退役登记: EXP-3c×3 (CONSUME_RANGE/RANGE_COUNT/ASSERTION) 与
//   EXP-3d×2 (WRITE/READ_LOOKAROUND_TABLE) 已由 Developer 落地为实现臂
//   (上表 LAND_ARMS 尾 5 行 match 形强断言); deferred_arm 消费面余探针 2 处 (下锚)
// 接线探针对账 (本体行锚 + 批段常量在 deferred_arm 调用处)
land(/deferred_arm\("HandleInterrupts", "cc:757-816", EXP_BATCH_4\)/, 1, "探针 HandleInterrupts → EXP-4");
land(/deferred_arm\("CheckMemoryConsumption", "cc:1183-1208", EXP_BATCH_3D\)/, 1, "探针 CheckMemoryConsumption → EXP-3d");
// 原版臂行锚与 cc 实况对账 (case 行号双引用防漂移)
for (const [line, label] of [[908, "CONSUME_RANGE"], [913, "ASSERTION"], [921, "FORK"], [985, "JMP"],
  [988, "ACCEPT"], [999, "SET_QUANTIFIER_TO_CLOCK"], [1004, "CLEAR_REGISTER"], [1011, "SET_REGISTER_TO_CP"],
  [1020, "FILTER_QUANTIFIER"], [1025, "BEGIN_LOOP"], [1030, "END_LOOP"], [1041, "START_LOOKAROUND"],
  [1045, "END_LOOKAROUND"], [1056, "WRITE_LOOKAROUND_TABLE"], [1072, "READ_LOOKAROUND_TABLE"]]) {
  must(seg(line, line).includes(`Instruction::${label}`), `cc:${line} 应为 case ${label}`);
}
// FORK 臂拷贝序对账 (consumed 继承/寄存器全拷/flag 开四段拷贝/负目标前置拒绝)
land(/fork\.consumed_since_last_quantifier = thread\.consumed_since_last_quantifier;/g, 1, "FORK consumed 继承 (cc:923-924)");
land(/fork\.register_array\.copy_from_slice\(&thread\.register_array\);/g, 1, "FORK 寄存器全拷 (cc:926-930)");
// 拷贝段容忍 rustfmt 换行形 (lookaround 两字段行宽超 100 必被折行, 与仓内
// CI `cargo +nightly fmt --check` 实况对齐; 字段名/顺序/实参源仍逐项强断言)
for (const [field, anchor] of [
  ["quantifier_clock_array", "cc:933-941"], ["capture_clock_array", "cc:943-950"],
  ["lookaround_match_index_array", "cc:953-961"], ["lookaround_clock_array", "cc:963-971"],
]) land(new RegExp(`copy_array_slot\\(\\s*&mut fork\\.${field},\\s*&thread\\.${field},?\\s*\\)`), 1, `FORK 拷贝段 ${field} (${anchor})`);
land(/FORK 臂 目标 pc 为负/g, 1, "FORK 负目标加固 (cc:922)");
// 推迟登记双引用 (原版行号 + Rust 台账符号) 命中 — 台账在模块头/探针 doc 双处, 取 ≥1
landMin(landed, /EXP-4 入口面/, "模块头 EXP-4 批段台账");
landMin(landed, /HandleInterrupts cc:757-816/, "HandleInterrupts 本体推迟登记 (EXP-4)");
landMin(landed, /ToInstructionVector cc:63-71/, "ToInstructionVector 推迟登记");
landMin(landed, /FindNextMatch cc:831-878/, "FindNextMatch 落地对账 (EXP-3b)");
landMin(landed, /RunActiveThreadsToEnd cc:697-752/, "RunActiveThreadsToEnd 落地对账 (EXP-3b)");
landMin(landed, /RunActiveThreads cc:1120-1127/, "RunActiveThreads 落地对账 (EXP-3b)");
landMin(landed, /CheckMemoryConsumption cc:1183-1208/, "CheckMemoryConsumption 本体推迟登记 (EXP-3d)");
landMin(landed, /FillLookaroundTable cc:591-633/, "FillLookaroundTable 推迟登记");
// 禁 todo! / 假绿: NotImplemented 通道在场且代码区 (剥注释后) 无内建 panic 宏
land(/InterpreterError::NotImplemented \{/g, 1, "NotImplemented 显式失败通道 (deferred_arm 单点)");
const codeOnly = landed.replace(/^\s*\/\/.*$/gm, "");
if (count(codeOnly, /\b(todo|unimplemented)!\s*\(/g) !== 0) missing.push("interpreter.rs 代码含 todo!/unimplemented! (违规)");
assertCount++;

// 测试面: EXP-3a 规约 25 用例 + 主循环路由/防御 2 用例 (随 EXP-3b 更名) +
// EXP-3b 调度新 bin 17 用例, 全名逐项命中 + 金标准常量断言
const tests = readFileSync(join(root, "v8-regexp-exec", "tests", "experimental_interpreter_unittest.rs"), "utf8");
const TESTS = [
  "constants_golden", "line_terminator_golden", "regexp_word_golden",
  "satisfies_assertion_input_bounds", "satisfies_assertion_line", "satisfies_assertion_boundary",
  "filter_groups_root_group_keep_and_discard", "filter_groups_quantifier_gating",
  "filter_groups_sibling_resume", "filter_groups_lookaround_track", "filter_groups_defense",
  "thread_new_empty_golden", "last_input_index_golden", "pc_processed_roundtrip",
  "nfa_ctor_captureless_lookbehind_track", "nfa_ctor_lookahead_switches_track",
  "nfa_ctor_capture_in_lookaround_switches_track", "nfa_ctor_quantifier_count_and_filter_pc",
  "nfa_ctor_skips_last_instruction", "nfa_ctor_defense", "flush_consume_range_hit_and_miss",
  "flush_range_count_multi_range", "flush_blocked_priority_reverse_activation",
  "flush_blocked_defense", "run_active_thread_batch_routing",
  "run_active_thread_dispatch_defense", "match_and_memory_surface",
];
for (const t of TESTS) landIn(tests, new RegExp(`fn ${t}\\(\\) -> Void`), 1, `测试用例 ${t}`);
landIn(tests, /assert_eq!\(K_UNDEFINED_CLOCK_VALUE, u64::MAX\)/g, 1, "金标准: u64::MAX 实测");
landIn(tests, /assert_eq!\(K_UNDEFINED_CLOCK_VALUE, 0_u64\.wrapping_sub\(1\)\)/g, 1, "金标准: -1 回绕表达式");
landMin(tests, /cc:390 边界/, "cc:390 循环界 quirk 金标准");

const tests3b = readFileSync(join(root, "v8-regexp-exec", "tests", "experimental_interpreter3b_unittest.rs"), "utf8");
const TESTS_3B = [
  "sched_fork_copies_and_priority", "sched_fork_target_negative",
  "sched_fork_flag_deferred_memory_probe", "sched_accept_replaces_and_clears",
  "sched_jmp_and_pc_dedup", "sched_set_quantifier_clock", "sched_clear_register",
  "sched_set_register_cp_capture_clock", "sched_register_bounds_defense",
  "sched_begin_end_loop_destroy", "sched_end_loop_after_consume_advance",
  "sched_end_lookaround_sets_best", "sched_find_next_match_lookaround_priority",
  "sched_find_next_match_zero_width_repeat", "sched_run_threads_to_end_stop_conditions",
  "sched_interrupt_probe_at_64_ticks", "sched_run_active_threads_pop_order",
];
for (const t of TESTS_3B) landIn(tests3b, new RegExp(`fn ${t}\\(\\) -> Void`), 1, `3b 测试用例 ${t}`);
// 3b bin 规范面: ctor log_init + aok Void/OK 收尾 + 用例数对账
landIn(tests3b, /#\[ctor::ctor\(unsafe\)\]/g, 1, "3b 规范: ctor log_init");
landIn(tests3b, /log_init::init\(\);/g, 1, "3b 规范: log_init::init()");
landIn(tests3b, /\) -> Void \{/g, TESTS_3B.length, "3b 规范: Void 返回用例数");
landIn(tests3b, /^\s*OK\s*$/gm, TESTS_3B.length, "3b 规范: OK 收尾数");

if (count(tests + tests3b, /todo!|unimplemented!/g) !== 0) missing.push("tests 含 todo!/unimplemented! (违规)");
assertCount++;
if (missing.length) throw new Error(`interpreter.rs/tests 落地对数缺项 (票单):\n  ${missing.join("\n  ")}`);

console.log(
  `experimental-interpreter.cc.js: 对数断言 ${assertCount} 条全过; 台账 ${LEDGER.length} 区间无缝覆盖 cc:1-1607` +
    ` (EXP-3a 落地 ${landedA} 段, EXP-3b 落地 ${landedB} 段; 批段引用 EXP-3c×${cCount} / EXP-3d×${dCount}` +
    ` / EXP-4×${eCount}, 样板 ${admCount} 段); 常量 4 值实测核对 (-1/-1/u64::MAX 回绕/64);` +
    ` 主循环 case ×19 无 default; 实现臂 ${LAND_ARMS.length} (含 EXP-3c/3d 批段 5 臂由推迟转落地)` +
    ` + preamble 3 动作 + 调度外围 3 方法 + 接线探针 2 (deferred_arm 消费面) + FORK 拷贝族 4` +
    ` 逐项落地对账 (cc case 行 15 点实况核验); 测试 ${TESTS.length}+${TESTS_3B.length} 用例名逐项命中, 缺项 0;` +
    ` 产物 v8-regexp-exec/src/regexp/experimental/experimental-interpreter.cc.rs.skel`,
);
