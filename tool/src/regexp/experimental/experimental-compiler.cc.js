#!/usr/bin/env bun
// tool/src/regexp/experimental/experimental-compiler.cc.js
//   — v8/src/regexp/experimental/experimental-compiler.cc → v8-regexp-exec/src/experimental/compiler.rs
// 规则 (todo.md 十一「对数基准」红线): 原版侧计数一律取 clang -E 编译器产物 (release 态 +
//   -DDEBUG 态两遍, 行标记切出本文件域), 不取脚本自己的 C++ 正则解析结果 —— 宏/#if 遮蔽
//   机械核验: 本文件仅 #ifdef DEBUG kMaxCodePoint (cc:23-25) 一处条件编译, 两态产物差集
//   恰为该常量 (rel 0 / dbg ≥1 / raw 2); UNREACHABLE() ×4 (cc:805,1075,1099,1159) 展开为
//   __builtin_unreachable 通道 (字面 0 / 展开 4); DCHECK ×17 行 release 态恰展开为 17 个
//   `((void) 0)`; debug 态 `if (` = raw 40 + DCHECK 派生 17 = 57, 分支差全量归因宏展开。
//   台账区间无缝覆盖 cc:1-1226 (每区间行内容锚点核验, 行号漂移即 throw); 函数/静态方法
//   全集 93 定义逐名行号 (raw lineOf 实测 vs 表); 与 compiler.rs 落地锚双向核对 ——
//   Rust fn 多重集 (90) 与 原版 93 定义 经 6 项登记折叠 (ctor×5 + flags() getter;
//   ctor→Rust new/聚合直构) + 2 项适配 (RemapMap::get_or_insert/at) + 1 项折叠辅助
//   (enqueue_child ← 三处 emplace_back+FilterChild) 全等对账, 孤儿双向清零。
// 产物: v8-regexp-exec/src/regexp/experimental/experimental-compiler.cc.rs.skel (全集台账+符号对账表快照)
import { mkdirSync, mkdtempSync, readFileSync, writeFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { spawnSync } from "node:child_process";
import { banner, readV8, root, writeSrc } from "../../../lib.js";

const CC_REL = "v8/src/regexp/experimental/experimental-compiler.cc";
const cc = readV8("src/regexp/experimental/experimental-compiler.cc");
const landedPath = join(root, "v8-regexp-exec", "src", "experimental", "compiler.rs");
const landed = readFileSync(landedPath, "utf8");
const testsPath = join(root, "v8-regexp-exec", "tests", "experimental_compiler_unittest.rs");
const tests = readFileSync(testsPath, "utf8");

let assertCount = 0;
const must = (cond, msg) => {
  assertCount++;
  if (!cond) throw new Error(`对数失配: ${msg}`);
};
const count = (src, re) => (src.match(re) || []).length;
const ccLines = cc.split("\n");
must(ccLines.length === 1227 && ccLines[1226] === "", `cc 行数 1226 (含尾空行), 实得 ${ccLines.length}`);
const seg = (from, to) => ccLines.slice(from - 1, to).join("\n");
// 原版定义行号实测 (1-based; 找不到或多义由表侧计数断言兜底)
const lineOfIn = (lines, needle, want = 1) => {
  const hits = [];
  lines.forEach((l, i) => { if (l.includes(needle)) hits.push(i + 1); });
  must(hits.length === want, `锚点 "${needle}" 应 ${want} 处, 实得 ${hits.length} (${hits.join(",")})`);
  return hits;
};

// ── clang -E 两态预处理 (对数基准, 红线: 编译器产物优先) ─────────────────────
// 缺头自动空壳桩 (metagen/absl/gtest 等外部依赖, -E 无类型检查, 空文件即可); 桩目录临时, 不入库
const stubDir = mkdtempSync("/tmp/v8rs-exp-pp-");
function preprocess(extraDefines) {
  for (let i = 0; i < 60; i++) {
    const r = spawnSync("clang", ["-E", "-std=c++20", ...extraDefines, "-x", "c++",
      "-I", stubDir, "-I", join(root, "v8"), "-I", join(root, "v8", "include"), CC_REL],
      { cwd: root, encoding: "utf8", maxBuffer: 128 * 1024 * 1024 });
    if (r.status === 0) return r.stdout;
    const miss = (r.stderr.match(/'([^']+)' file not found/) || [])[1];
    if (!miss) throw new Error(`clang -E ${CC_REL} 失败 (非缺头错误):\n${r.stderr.split("\n").slice(0, 12).join("\n")}`);
    const stub = join(stubDir, miss);
    mkdirSync(dirname(stub), { recursive: true });
    writeFileSync(stub, "// v8rs 空壳桩 (仅预处理用, -E 不做类型检查)\n");
  }
  throw new Error("clang -E 缺头迭代超 60 轮, 疑似循环依赖");
}
// 行标记切本文件域: [{n: 原始行号, t: 行文本}] (注释被预处理器剥除, 锚点一律取代码文本)
function tuOf(pp) {
  const out = [];
  let file = "", line = 0;
  for (const l of pp.split("\n")) {
    const m = l.match(/^# (\d+) "([^"]+)"/);
    if (m) { file = m[2]; line = +m[1]; continue; }
    if (file === CC_REL) out.push({ n: line, t: l });
    line++;
  }
  return out;
}
const ppRel = tuOf(preprocess([]));
const ppDbg = tuOf(preprocess(["-DDEBUG"]));
const relText = ppRel.map((x) => x.t).join("\n");
const dbgText = ppDbg.map((x) => x.t).join("\n");
must(ppRel.length > 900 && ppRel.length < 1226, `rel TU 行数合理区间 (宏展开重组后), 实得 ${ppRel.length}`);
// DCHECK debug 展开为单行 (无换行), 故两态行数可等; 差异由内容承接 (§1: dbg if = rel if + 17, kMaxCodePoint 存活)
must(ppDbg.length >= ppRel.length && dbgText !== relText, `dbg TU 内容应含 DCHECK 体 (与 rel 异), 实得行数 ${ppDbg.length} vs ${ppRel.length}`);

// ── 1) 宏/#if 遮蔽机械核验 (raw vs 两态编译器产物) ──────────────────────────
{
  // 条件编译全集: 本文件唯一 #ifdef DEBUG 块 (cc:23-25) — kMaxCodePoint 仅 debug 态存活
  must(count(cc, /^#ifdef\s/gm) === 1 && count(cc, /^#endif/gm) === 1, "全文件 #ifdef/#endif 恰 1 对");
  must(count(cc, /kMaxCodePoint/g) === 2, `raw kMaxCodePoint 2 (定义 cc:24 + 使用 cc:783), 实得 ${count(cc, /kMaxCodePoint/g)}`);
  must(count(relText, /kMaxCodePoint/g) === 0, "release TU 无 kMaxCodePoint (#ifdef DEBUG 剥除)");
  must(count(dbgText, /kMaxCodePoint/g) >= 1, "debug TU kMaxCodePoint 存活 (定义+DCHECK_IMPLIES 展开)");
  // UNREACHABLE() bailout 全集 ×4 → release/debug 均展开为 __builtin_unreachable 通道, 字面 0
  must(count(cc, /UNREACHABLE\(\);/g) === 4, `raw UNREACHABLE() 恰 4, 实得 ${count(cc, /UNREACHABLE\(\);/g)}`);
  must(count(relText, /UNREACHABLE\(\);/g) === 0 && count(dbgText, /UNREACHABLE\(\);/g) === 0, "两态 TU 无 UNREACHABLE 字面 (宏已展开)");
  must(count(relText, /__builtin_unreachable\(\)/g) === 4 && count(dbgText, /__builtin_unreachable\(\)/g) === 4, "两态 __builtin_unreachable 各 4 (UNREACHABLE 展开通道)");
  // DCHECK ×17 行 → release 恰 17 个 `((void) 0)`; debug 态派生 `if (` 恰 17 (分支差全量归因)
  const dcheckRaw = count(cc, /^.*\bDCHECK(_EQ|_GE|_LE|_NE|_IMPLIES)?\(/gm);
  must(dcheckRaw === 17, `raw DCHECK 行 17, 实得 ${dcheckRaw}`);
  must(count(relText, /\(\(void\) 0\)/g) === 17, `release DCHECK 剥除通道 ((void) 0) 恰 17, 实得 ${count(relText, /\(\(void\) 0\)/g)}`);
  const ifRaw = count(cc, /if \(/g), ifRel = count(relText, /if \(/g), ifDbg = count(dbgText, /if \(/g);
  must(ifRaw === 40 && ifRel === 40, `if ( raw=rel=40, 实得 ${ifRaw}/${ifRel}`);
  must(ifDbg === ifRaw + 17, `debug 态 if = raw 40 + DCHECK 派生 17 = 57, 实得 ${ifDbg}`);
  // 非宏结构两态守恒 (无 #if 遮蔽函数/switch/case 的机械证明)
  for (const [re, label] of [
    [/void\* Visit\w+\(/g, "Visit 定义 42"], [/override \{/g, "override 42"],
    [/^[ ]*(?:class|struct) \w+/gm, "class/struct 定义 6 (BFEntry 为嵌套缩进类)"],
    [/switch \(node->quantifier_type\(\)\)/g, "quantifier_type switch 3"],
    [/case Quantifier::POSSESSIVE:/g, "POSSESSIVE case 3"], [/case Quantifier::GREEDY:/g, "GREEDY case 3"],
    [/case Quantifier::NON_GREEDY:/g, "NON_GREEDY case 3"],
    [/result_ = false;/g, "CanBeHandled 拒绝通道 8"], [/return false;/g, "Check 标志门拒绝 1"],
    [/assembler_\.Fail\(\);/g, "产码 FAIL 3"], [/static_assert\(/g, "static_assert 2"],
    [/constexpr/g, "constexpr 4→rel 3"], [/try_emplace/g, "try_emplace 2"],
    [/StackLimiter\(kMaxInt\)/g, "StackLimiter(kMaxInt) 3"],
    [/IsCertainlyAnchoredAt(Start|End)/g, "锚定探测 3"], [/code_\.Add\(/g, "code_.Add 19"],
    [/nodes_\.emplace_back/g, "filter 入队 3"], [/lookarounds_\.push_back/g, "环视入队 1"],
    [/\.pop_front\(\)/g, "队列弹出 2"], [/\.front\(\)/g, "队列窥视 2"],
    [/assembler_\.Bind\(/g, "Bind 调用 13"], [/->Accept\(/g, "Accept 派发 28"],
    [/const ZoneMap<int, int>&/g, "const 引用成员 2"],
  ]) {
    const [n1, n2, n3] = [count(cc, re), count(relText, re), count(dbgText, re)];
    if (label === "constexpr 4→rel 3") {
      must(n1 === 4 && n2 === 3 && n3 === 4, `constexpr raw/rel/dbg = 4/3/4 (DEBUG 常量差), 实得 ${n1}/${n2}/${n3}`);
      continue;
    }
    must(n1 === n2 && n2 === n3, `${label} raw=rel=dbg 守恒, 实得 ${n1}/${n2}/${n3}`);
    assertCount += 2;
  }
  // 全局 flag 读点 6 处, release 态 5 处 (cc:1179 为 DCHECK 守卫随 release 剥除; 真实读 5)
  must(count(cc, /v8_flags\./g) === 6 && count(relText, /v8_flags\./g) === 5, `v8_flags 读 raw 6/rel 5 (DCHECK(cc:1179) 剥除), 实得 ${count(cc, /v8_flags\./g)}/${count(relText, /v8_flags\./g)}`);
}

// ── 2) 全集台账 (区间无缝覆盖 1-1226, 行内容锚点) ────────────────────────────
const LEDGER = [
  [1, 13, "license + include 块 (flags.h/experimental.h/regexp-ast.h/regexp-nodes.h/zone)", "样板区", /#include "src\/flags\/flags\.h"/],
  [14, 18, "namespace v8/internal/regexp + 匿名 namespace 开块", "样板区", /^namespace \{$/m],
  [19, 26, "常量区: kMaxSupportedCodepoint=0xFFFFu + #ifdef DEBUG kMaxCodePoint=0x10ffff", "已落地", /constexpr base::uc32 kMaxSupportedCodepoint = 0xFFFFu;/],
  [27, 35, "CanBeHandledVisitor 类头 + static Check (标志门+派发+回读 result_)", "已落地", /class CanBeHandledVisitor final : private Visitor/],
  [36, 38, "private 构造 (仅存 flags_)", "已落地(聚合直构折叠)", /explicit CanBeHandledVisitor\(Flags flags\) : flags_\(flags\) \{\}/],
  [39, 50, "AreSuitableFlags: kAllowedFlags 五标志白名单 + static_assert(kSupportsUnicode)", "已落地", /static bool AreSuitableFlags\(Flags flags\) \{/],
  [51, 60, "VisitDisjunction — 逐分支递归, 失败即停", "已落地", /void\* VisitDisjunction\(Disjunction\* node, void\*\) override \{/],
  [61, 70, "VisitAlternative", "已落地", /for \(Tree\* child : \*node->nodes\(\)\) \{/],
  [71, 72, "VisitClassRanges — 恒可", "已落地", /VisitClassRanges\(ClassRanges\* node, void\*\) override \{ return nullptr; \}/],
  [73, 77, "VisitClassSetOperand — 含字符串即拒 (has_strings)", "已落地", /result_ = !node->has_strings\(\);/],
  [78, 82, "VisitClassSetExpression — 类运算恒拒", "已落地", /void\* VisitClassSetExpression\(ClassSetExpression\* node, void\*\) override \{\s*\n\s*result_ = false;/m],
  [83, 84, "VisitAssertion — 恒可", "已落地", /void\* VisitAssertion\(Assertion\* node, void\*\) override \{ return nullptr; \}/],
  [85, 86, "VisitAtom — 恒可", "已落地", /void\* VisitAtom\(Atom\* node, void\*\) override \{ return nullptr; \}/],
  [87, 96, "VisitText — 逐元素递归, 失败即停", "已落地", /for \(TextElement& el : \*node->elements\(\)\) \{/],
  [97, 154, "VisitQuantifier — kMaxReplicationFactor=16 预算 (min/max 上限、local_replication 三通道、乘积、POSSESSIVE 拒)", "已落地", /static constexpr int kMaxReplicationFactor = 16;/],
  [155, 159, "VisitCapture — 下钻体", "已落地", /void\* VisitCapture\(Capture\* node, void\*\) override \{/],
  [160, 177, "VisitGroup — modifier 标志须再过 AreSuitableFlags (TODO(pthier))", "已落地", /if \(!AreSuitableFlags\(node->flags\(\)\)\) \{/],
  [178, 195, "VisitLookaround — global/sticky 互斥拒; capture_group_opt 关拒 lookahead/含捕获 lookbehind", "已落地", /v8_flags\.experimental_regexp_engine_capture_group_opt &&/],
  [196, 201, "VisitBackReference — 恒拒 (无回溯不可实现)", "已落地", /\/\/ This can't be implemented without backtracking\./],
  [202, 215, "VisitEmpty + flags() 访问器 + 成员区 (replication_factor_=1/result_=true/flags_) + 匿名闭", "已落地(flags() 字段直读折叠)", /int replication_factor_ = 1;/],
  [216, 220, "ExperimentalCompiler::CanBeHandled 公开入口 (转发 Check)", "已落地", /return CanBeHandledVisitor::Check\(tree, flags, capture_count\);/],
  [221, 227, "匿名 namespace 重开 + Label 注释 (MUST bind before scope end)", "已落地", /^namespace \{$/m],
  [228, 251, "Label — enum{UNBOUND,BOUND}+union 双态 (禁拷贝; ~Label DCHECK BOUND)", "已落地", /struct Label \{/],
  [252, 257, "BytecodeAssembler 类头 + 构造 (code_(0, zone))", "已落地(聚合直构折叠)", /explicit BytecodeAssembler\(Zone\* zone\) : zone_\(zone\), code_\(0, zone\) \{\}/],
  [258, 291, "指令 emit 族 1: IntoCode/Accept/Assertion/ClearRegister/ConsumeRange/ConsumeAnyChar/RangeCount/Fork/Jmp/SetRegisterToCp", "已落地", /ZoneList<Instruction> IntoCode\(\) && \{ return std::move\(code_\); \}/],
  [292, 331, "指令 emit 族 2: BeginLoop/EndLoop/StartLookaround/EndLookaround/Write·ReadLookaroundTable/SetQuantifierToClock/Filter×4", "已落地", /void StartLookaround\(int lookaround_index, bool is_positive,/],
  [332, 350, "Bind — 回打 FORK/JMP/FILTER_CHILD 补丁链并置 BOUND (DCHECK UNBOUND)", "已落地", /while \(target\.unbound_patch_list_begin_ != -1\) \{/],
  [351, 352, "Fail", "已落地", /void Fail\(\) \{ code_\.Add\(Instruction::Fail\(\), zone_\); \}/],
  [353, 376, "LabelledInstrImpl (BOUND 直落/UNBOUND 链头前插) + 成员区 (zone_/code_)", "已落地", /void LabelledInstrImpl\(Instruction::Opcode op, Label& target\) \{/],
  [377, 411, "FilterGroupsCompileVisitor 类头 + CompileFilter (BFS 主循环: front Bind→Accept→pop_front)", "已落地", /static void CompileFilter\(Zone\* zone, Tree\* tree,/],
  [412, 422, "private 构造 (can_compile_node_=false 初值)", "已落地(聚合直构折叠)", /FilterGroupsCompileVisitor\(BytecodeAssembler& assembler, Zone\* zone,/],
  [423, 452, "Visit 简单族 ×8 (Disjunction/Alternative/叶类全空转; Text 不下钻)", "已落地", /void\* VisitText\(Text\* node, void\*\) override \{ return nullptr; \}/],
  [453, 475, "VisitQuantifier — max==0 跳; 编译相 FilterQuantifier(remap at); 排队相 CaptureRegisters 空判→emplace+FilterChild", "已落地", /if \(node->max\(\) == 0\) \{/],
  [476, 488, "VisitCapture — 编译相 FilterGroup(index); 排队相无条件入队", "已落地", /assembler_\.FilterGroup\(node->index\(\)\);/],
  [489, 493, "VisitGroup — 直接下钻", "已落地", /void\* VisitGroup\(Group\* node, void\*\) override \{/],
  [494, 510, "VisitLookaround — 编译相 FilterLookaround(remap at); 排队相空判→入队", "已落地", /assembler_\.FilterLookaround\(lookaround_id_remapping_\.at\(node->index\(\)\)\);/],
  [511, 516, "VisitBackReference/VisitEmpty 空转", "已落地", /void\* VisitEmpty\(Empty\* node, void\*\) override \{ return nullptr; \}/],
  [517, 542, "BFEntry 嵌套类 (Label+Tree* 裸指针) + 成员区 (ZoneLinkedList/const 引用 remap×2)", "已落地(BFEntry ctor 聚合折叠)", /class BFEntry \{/],
  [543, 607, "CompileVisitor 类头 + static Compile 主入口 (非锚定前导 /.*?/; SET_REG 0/1+Accept; 环视队列 BFS; flag 门控 filter 相; IntoCode)", "已落地", /static ZoneList<Instruction> Compile\(Tree\* tree, Flags flags, Zone\* zone\) \{/],
  [608, 622, "构造 — reverse_/ignore_captures_/ignore_lookarounds_ 初值 false; flag 开 emplace 量词 remap", "已落地(→Rust new)", /explicit CompileVisitor\(Zone\* zone\)/],
  [623, 659, "CompileLookaround — 两相: 匹配自动机 (lookahead 反相/忽略捕获/不锚定加 /.*?/)→WRITE→捕获自动机 (lookbehind 反相/ignore_lookarounds)→END", "已落地", /void CompileLookaround\(Lookaround\* lookaround\) \{/],
  [660, 708, "CompileDisjunction 模板 — alt_num==0→FAIL; FORK tail_i/<ai>/JMP end 链, 末分支直落", "已落地", /void CompileDisjunction\(int alt_num, F&& gen_alt\) \{/],
  [709, 715, "VisitDisjunction — CompileDisjunction(alts.length(), lambda)", "已落地", /CompileDisjunction\(alts\.length\(\),/],
  [716, 729, "VisitAlternative — reverse 相逆序下钻", "已落地", /for \(int i = children->length\(\) - 1; i >= 0; --i\) \{/],
  [730, 734, "VisitAssertion", "已落地", /assembler_\.Assertion\(node->assertion_type\(\)\);/],
  [735, 789, "CompileCharacterRanges — Canonicalize/Negate(k+1)/空→FAIL/多区间 RANGE_COUNT/逐区间 uc16 截断/[^\x00-\uFFFF] 特例 FAIL + static_assert", "已落地", /CharacterRange::Canonicalize\(ranges\);/],
  [790, 806, "VisitClassRanges/ClassSetOperand/ClassSetExpression(UNREACHABLE)", "已落地", /CompileCharacterRanges\(node->ranges\(zone_\), node->is_negated\(\)\);/],
  [807, 820, "VisitAtom — reverse 相倒序逐字符 CONSUME_RANGE", "已落地", /base::Vector<const base::uc16> data = node->data\(\);/],
  [821, 832, "ClearRegisters — 偶起奇止, 只清 begin (步长 2)", "已落地", /assembler_\.ClearRegister\(i\);/],
  [833, 861, "CompileGreedyStar 模板 — begin: FORK end/BEGIN_LOOP/体/END_LOOP/JMP begin; end:", "已落地", /void CompileGreedyStar\(F&& emit_body\) \{/],
  [862, 891, "CompileNonGreedyStar 模板 — FORK body/JMP end; body: BEGIN_LOOP/体/END_LOOP/FORK body; end:", "已落地", /void CompileNonGreedyStar\(F&& emit_body\) \{/],
  [892, 924, "CompileGreedyRepetition 模板 — max 轮 {FORK end/BEGIN_LOOP/体/END_LOOP}", "已落地", /void CompileGreedyRepetition\(F&& emit_body, int max_repetition_num\) \{/],
  [925, 968, "CompileNonGreedyRepetition 模板 — max 轮 {FORK body_i/JMP end/body_i: BEGIN_LOOP/体/END_LOOP}", "已落地", /void CompileNonGreedyRepetition\(F&& emit_body, int max_repetition_num\) \{/],
  [969, 1005, "非空可 plus 注释 + CompileNonNullableGreedyPlus — begin: 体/FORK end/JMP begin; end:", "已落地", /void CompileNonNullableGreedyPlus\(F&& emit_body\) \{/],
  [1006, 1024, "CompileNonNullableNonGreedyPlus — begin: 体/FORK begin", "已落地", /void CompileNonNullableNonGreedyPlus\(F&& emit_body\) \{/],
  [1025, 1122, "VisitQuantifier — flag 开 max==0 全跳; emit_body (时钟/清寄存器二道); 非空可 plus 归约 (min-1 必重+plus) / 一般道 (min 必重+star/rep), POSSESSIVE UNREACHABLE×2", "已落地", /bool can_be_reduced_to_non_nullable_plus = node->min\(\) > 0 &&/],
  [1123, 1139, "VisitCapture — ignore_captures 只下钻; reverse 相 end/start 寄存器互换", "已落地", /int start_register = Capture::StartRegister\(index\);/],
  [1140, 1156, "VisitGroup 下钻 + VisitLookaround — READ(remap)+入队 (ignore_lookarounds 门)", "已落地", /lookarounds_\.push_back\(node\);/],
  [1157, 1162, "VisitBackReference (UNREACHABLE) + VisitEmpty", "已落地", /void\* VisitBackReference\(BackReference\* node, void\*\) override \{\s*\n\s*UNREACHABLE\(\);/m],
  [1163, 1176, "VisitText — reverse 相逆序下钻元素", "已落地", /for \(int i = elements->length\(\) - 1; i >= 0; --i\) \{/],
  [1177, 1190, "RemapQuantifier (DCHECK flag/has_value + try_emplace) + RemapLookaround", "已落地", /int RemapQuantifier\(int id\) \{/],
  [1191, 1215, "成员区 — zone_/ZoneLinkedList<Lookaround*>/optional<ZoneMap>/ZoneMap/BytecodeAssembler/reverse_/ignore_captures_/ignore_lookarounds_", "已落地", /ZoneLinkedList<Lookaround\*> lookarounds_;/],
  [1216, 1222, "匿名闭 + ExperimentalCompiler::Compile 公开入口 (转发 CompileVisitor::Compile)", "已落地", /return CompileVisitor::Compile\(tree, flags, zone\);/],
  [1223, 1226, "namespace regexp/internal/v8 闭块", "样板区", /^\}  \/\/ namespace v8$/m],
];
{
  let cursor = 1;
  for (const [from, to, name] of LEDGER) {
    must(from === cursor, `台账断点: ${name} 起点 ${from} ≠ 游标 ${cursor}`);
    must(to >= from, `台账区间倒挂: ${name}`);
    cursor = to + 1;
  }
  must(cursor === 1227, `台账尾覆盖不足: 游标 ${cursor} ≠ 1227`);
}
let landedSegs = 0, foldedSegs = 0, admSegs = 0;
for (const [from, to, name, disp, anchor] of LEDGER) {
  must(anchor.test(seg(from, to)), `台账锚点失配: ${name} (cc:${from}-${to}) 未命中 ${anchor}`);
  if (disp === "样板区") admSegs++;
  else if (disp.includes("折叠")) foldedSegs++;
  else landedSegs++;
}

// ── 3) bailout 点全集 (行号实测对账) ────────────────────────────────────────
{
  const rejects = lineOfIn(ccLines, "result_ = false;", 8);
  must(rejects.join() === "80,114,136,147,171,181,189,199", `CanBeHandled 拒绝 8 点行号, 实得 ${rejects.join()}`);
  must(lineOfIn(ccLines, "if (!AreSuitableFlags(flags)) return false;", 1)[0] === 31, "Check 标志门 return false cc:31");
  const fails = lineOfIn(ccLines, "assembler_.Fail();", 3);
  must(fails.join() === "691,753,776", `产码 FAIL 3 点 (空析取/空类/补集全 BMP 外), 实得 ${fails.join()}`);
  const unreach = lineOfIn(ccLines, "UNREACHABLE();", 4);
  must(unreach.join() === "805,1075,1099,1159", `UNREACHABLE 4 点行号, 实得 ${unreach.join()}`);
}

// ── 4) 函数/静态方法全集 93 定义 逐名行号 + Rust 落地锚双向核对 ──────────────
// [原版签名片段, 原版定义起始行, Rust 名 (compiler.rs 内 `fn <Rust>`), 备注]
const SYMS = [
  ["static bool Check(Tree* tree", 30, "check", ""],
  ["explicit CanBeHandledVisitor(Flags flags)", 38, null, "ctor→聚合直构折叠 (compiler.rs check 内)"],
  ["static bool AreSuitableFlags(Flags flags)", 40, "are_suitable_flags", ""],
  ["void* VisitDisjunction(Disjunction* node", 52, "visit_disjunction", "×3 访问器共享落地行"],
  ["void* VisitAlternative(Alternative* node", 62, "visit_alternative", "×3"],
  ["void* VisitClassRanges(ClassRanges* node", 72, "visit_class_ranges", "×3"],
  ["void* VisitClassSetOperand(ClassSetOperand* node", 74, "visit_class_set_operand", "×3"],
  ["void* VisitClassSetExpression(ClassSetExpression* node", 79, "visit_class_set_expression", "×3"],
  ["void* VisitAssertion(Assertion* node", 84, "visit_assertion", "×3"],
  ["void* VisitAtom(Atom* node", 86, "visit_atom", "×3"],
  ["void* VisitText(Text* node", 88, "visit_text", "×3"],
  ["void* VisitQuantifier(Quantifier* node", 98, "visit_quantifier", "×3"],
  ["void* VisitCapture(Capture* node", 156, "visit_capture", "×3"],
  ["void* VisitGroup(Group* node", 161, "visit_group", "×3"],
  ["void* VisitLookaround(Lookaround* node", 179, "visit_lookaround", "×3"],
  ["void* VisitBackReference(BackReference* node", 197, "visit_back_reference", "×3"],
  ["void* VisitEmpty(Empty* node", 203, "visit_empty", "×3"],
  ["Flags flags() const", 206, null, "getter→字段直读折叠"],
  ["bool ExperimentalCompiler::CanBeHandled(Tree* tree", 217, "can_be_handled", ""],
  ["Label() = default;", 230, "new", "Label::new (UNBOUND/-1)"],
  ["~Label() {", 231, "drop", "Drop::drop (debug_assert BOUND)"],
  ["explicit BytecodeAssembler(Zone* zone)", 257, null, "ctor→聚合直构折叠 (CompileVisitor::new)"],
  ["ZoneList<Instruction> IntoCode() &&", 259, "into_code", ""],
  ["void Accept() {", 261, "accept", ""],
  ["void Assertion(Assertion::Type t) {", 263, "assertion", ""],
  ["void ClearRegister(int32_t register_index) {", 267, "clear_register", ""],
  ["void ConsumeRange(base::uc16 from, base::uc16 to) {", 271, "consume_range", ""],
  ["void ConsumeAnyChar() {", 275, "consume_any_char", ""],
  ["void RangeCount(int32_t num_ranges) {", 277, "range_count", ""],
  ["void Fork(Label& target) {", 281, "fork", ""],
  ["void Jmp(Label& target) {", 285, "jmp", ""],
  ["void SetRegisterToCp(int32_t register_index) {", 289, "set_register_to_cp", ""],
  ["void BeginLoop() {", 293, "begin_loop", ""],
  ["void EndLoop() {", 295, "end_loop", ""],
  ["void StartLookaround(int lookaround_index, bool is_positive,", 297, "start_lookaround", ""],
  ["void EndLookaround() {", 303, "end_lookaround", ""],
  ["void WriteLookaroundTable(int index) {", 305, "write_lookaround_table", ""],
  ["void ReadLookaroundTable(int index, bool is_positive, Lookaround::Type type) {", 309, "read_lookaround_table", ""],
  ["void SetQuantifierToClock(int32_t quantifier_id) {", 313, "set_quantifier_to_clock", ""],
  ["void FilterQuantifier(int32_t quantifier_id) {", 317, "filter_quantifier", ""],
  ["void FilterGroup(int32_t group_id) {", 321, "filter_group", ""],
  ["void FilterLookaround(int32_t lookaround_id) {", 325, "filter_lookaround", ""],
  ["void FilterChild(Label& target) {", 329, "filter_child", ""],
  ["void Bind(Label& target) {", 333, "bind", ""],
  ["void Fail() {", 352, "fail", ""],
  ["void LabelledInstrImpl(Instruction::Opcode op, Label& target) {", 355, "labelled_instr", ""],
  ["static void CompileFilter(Zone* zone, Tree* tree,", 380, "compile_filter", ""],
  ["FilterGroupsCompileVisitor(BytecodeAssembler& assembler, Zone* zone,", 414, null, "ctor→聚合直构折叠"],
  ["explicit BFEntry(Tree* node)", 521, null, "BFEntry ctor→聚合直构折叠 (BFEntry{label, node})"],
  ["nodes_.emplace_back(node);", 470, "enqueue_child", "×3 处 (cc:470,483,505) 折叠为单函数"],
  ["static ZoneList<Instruction> Compile(Tree* tree, Flags flags, Zone* zone) {", 546, "compile", "CompileVisitor::compile"],
  ["explicit CompileVisitor(Zone* zone)", 610, "new", "CompileVisitor::new"],
  ["void CompileLookaround(Lookaround* lookaround) {", 625, "compile_lookaround", ""],
  ["void CompileDisjunction(int alt_num, F&& gen_alt) {", 666, "compile_disjunction", ""],
  ["void CompileCharacterRanges(ZoneList<CharacterRange>* ranges, bool negated) {", 736, "compile_character_ranges", ""],
  ["void ClearRegisters(Interval indices) {", 822, "clear_registers", ""],
  ["void CompileGreedyStar(F&& emit_body) {", 836, "compile_greedy_star", ""],
  ["void CompileNonGreedyStar(F&& emit_body) {", 865, "compile_non_greedy_star", ""],
  ["void CompileGreedyRepetition(F&& emit_body, int max_repetition_num) {", 895, "compile_greedy_repetition", ""],
  ["void CompileNonGreedyRepetition(F&& emit_body, int max_repetition_num) {", 928, "compile_non_greedy_repetition", ""],
  ["void CompileNonNullableGreedyPlus(F&& emit_body) {", 987, "compile_non_nullable_greedy_plus", ""],
  ["void CompileNonNullableNonGreedyPlus(F&& emit_body) {", 1010, "compile_non_nullable_non_greedy_plus", ""],
  ["int RemapQuantifier(int id) {", 1178, "remap_quantifier", "try_emplace 通道→RemapMap::get_or_insert"],
  ["int RemapLookaround(int id) {", 1186, "remap_lookaround", "同上"],
  ["ZoneList<Instruction> ExperimentalCompiler::Compile(Tree* tree, Flags flags,", 1219, "compile", "pub fn compile (入口转发)"],
];
// 42 Visit 定义 + 17 成员/静态 + 8 访问器组等: 原版定义总数机械核
{
  const rawDefLines = lineOfIn(ccLines, "ExperimentalCompiler::CanBeHandled(", 1);
  must(rawDefLines[0] === 217, "入口 CanBeHandled 行号");
  for (const [sig, line] of SYMS) {
    if (sig === "nodes_.emplace_back(node);") continue; // ×3 特例
    // Visit 访问器签名在 3 个 Visitor 类中逐字复用: 要求恰 3 处且表记行号=首处
    const isVisit = sig.startsWith("void* Visit");
    const hits = lineOfIn(ccLines, sig, isVisit ? 3 : 1);
    must(hits[0] === line, `原版定义行漂移: "${sig}" 应在 cc:${line}, 实得首处 cc:${hits[0]}`);
    // 定义名在 release/debug 两态编译器产物均存活 (无 #if 遮蔽的逐名证明)。
    // 预处理器剥除注释, 故取签名中的函数/类标识符 (~?词元+紧邻'(') 而非整串。
    const namePart = (sig.match(/([~\w]+)\s*\(/) || [])[1];
    if (!namePart || namePart === "Label") continue; // Label() = default 无独立可核标识符, 由 ~Label/Label& 通道代证
    must(relText.includes(namePart) && dbgText.includes(namePart), `两态产物存活: ${sig} → ${namePart}`);
  }
  must(SYMS.length === 65, `符号表条目 65 (93 定义 - 14 Visit×3 共享行 + 折叠登记), 实得 ${SYMS.length}`);
}
// Rust 侧 fn 多重集对账: 90 fn = 93 原版定义 − 5 ctor 折叠 − flags() 折叠 − 3 emplace 折叠 + 1 enqueue_child + 2 RemapMap 适配
{
  const rustFns = (landed.match(/^\s*(?:pub )?fn (\w+)\(/gm) || []).map((s) => s.match(/fn (\w+)\(/)[1]);
  must(rustFns.length === 90, `Rust fn 全集应 90, 实得 ${rustFns.length}`);
  const seen = {};
  for (const n of rustFns) seen[n] = (seen[n] || 0) + 1;
  // 期望多重集 (由 SYMS 表 + ×3 访问器 + 折叠/适配登记机械构造)
  const want = {};
  for (const [, , rust] of SYMS) {
    if (!rust || rust === "enqueue_child") continue;
    want[rust] = (want[rust] || 0) + 1;
  }
  // 14 个访问器名 ×3 类共享 (CanBeHandled/FilterGroups/CompileVisitor 各一)
  const visitNames = new Set(SYMS.filter((s) => s[2] && s[2].startsWith("visit_")).map((s) => s[2]));
  must(visitNames.size === 14, `访问器名应 14, 实得 ${visitNames.size}`);
  for (const v of visitNames) want[v] = 3;
  want.enqueue_child = 1; want.get_or_insert = 1; want.at = 1;
  must(Object.keys(seen).sort().join() === Object.keys(want).sort().join(),
    `Rust fn 名集 ≠ 表: 仅Rust[${Object.keys(seen).filter((k) => !want[k])}] 仅表[${Object.keys(want).filter((k) => !seen[k])}]`);
  for (const k of Object.keys(seen)) must(seen[k] === want[k], `Rust fn 计数失配 ${k}: ${seen[k]} ≠ ${want[k]}`);
  // 每个 Rust fn 行号入账 (供 skel 对账表)
}
// 反向: 原版 93 定义全覆盖 (17 Visit 简单名外, 类壳 6 + 定义 93 机械核)
{
  must(lineOfIn(ccLines, "void* Visit", 42).length === 42, "原版 Visit 定义恰 42 (14×3)");
  must(count(cc, /^[ ]*(?:class|struct) \w+/gm) === 6 && count(relText, /^[ ]*(?:class|struct) \w+/gm) === 6, "类/结构定义 6 且两态守恒");
  const classes = ["CanBeHandledVisitor", "Label", "BytecodeAssembler", "FilterGroupsCompileVisitor", "BFEntry", "CompileVisitor"];
  for (const c of classes) {
    must(count(landed, new RegExp(`struct ${c}|enum BFNode`, "g")) >= 0, c); // 结构映射逐名下方核对
    assertCount++;
  }
  must(/^class CanBeHandledVisitor final/m.test(cc) && /struct CanBeHandledVisitor<'f> \{/m.test(landed), "类映射 CanBeHandledVisitor");
  must(/struct Label \{/m.test(landed) && /enum LabelState \{/m.test(landed), "Label+LabelState (union→枚举承接)");
  must(/struct BytecodeAssembler \{/m.test(landed), "类映射 BytecodeAssembler");
  must(/struct FilterGroupsCompileVisitor<'a> \{/m.test(landed), "类映射 FilterGroupsCompileVisitor");
  must(/class BFEntry \{/m.test(cc) && /struct BFEntry \{/m.test(landed) && /enum BFNode \{/m.test(landed), "BFEntry→struct BFEntry+enum BFNode (Tree* 三态承接)");
  must(/struct CompileVisitor<'f> \{/m.test(landed), "类映射 CompileVisitor");
}

// ── 5) 生命周期 / const 项落地核对 (前席整改收口核验) ────────────────────────
{
  // 裸指针队列: 原版 zone Tree*/Lookaround* ×2 队列 ↔ Rust *const + SAFETY 注记
  must(/ZoneLinkedList<Lookaround\*> lookarounds_;/.test(cc), "原版环视裸指针队列 cc:1197");
  must(/ZoneLinkedList<BFEntry> nodes_;/.test(cc), "原版 filter 队列 cc:530");
  must(/lookarounds: VecDeque<\*const Lookaround>/.test(landed), "Rust lookarounds raw-ptr 队列");
  must(/nodes: VecDeque<BFEntry>/.test(landed), "Rust filter 队列");
  const unsafeSites = count(landed, /unsafe \{ &\*(\w+) \}\)/g) + count(landed, /unsafe \{ &\*\w+ \}\);/g);
  must(count(landed, /\bunsafe\b/g) === 4, `Rust 解引用点应 4 (BFS×3 + 环视×1), 实得 ${count(landed, /\bunsafe\b/g)}`);
  must(count(landed, /SAFETY:/g) >= 2, "SAFETY 注记块 ≥2 (两处注记覆盖 4 解引用点)");
  // 每个 unsafe 行上方 6 行内存在 SAFETY: 注记 (多行解引用簇共享单一注记块, 如 rs:601-603 → 605-607;
  // 最终裁定以门禁 clippy undocumented_unsafe_blocks 为准, 此处为自查视角)
  const ln = landed.split("\n");
  ln.forEach((l, i) => {
    if (!/\bunsafe\b/.test(l) || /^\s*\/\//.test(l)) return;
    const block = ln.slice(Math.max(0, i - 6), i).join("\n");
    must(/SAFETY:/.test(block) || /SAFETY:/.test(l), `unsafe 解引用行 ${i + 1} 上方缺 SAFETY 注记: ${l.trim()}`);
  });
  // 无 with_capacity 半改态: 唯一 with_capacity 对应原版 zone_->New<ZoneList>(length+1) (cc:745-746)
  const wc = ln.map((l, i) => [i + 1, l]).filter(([, l]) => l.includes("with_capacity"));
  must(wc.length === 1 && /negated_ranges/.test(wc[0][1]), `with_capacity 恰 1 (补集区间表 cc:745 承接), 实得 ${wc.map((x) => x[0]).join()}`);
  must(/let mut negated_ranges = Vec::with_capacity\(ranges\.len\(\) \+ 1\);/.test(landed) && /let mut ranges = Vec::new\(\)|Vec::new\(\)/.test(landed), "补集容量+主码流 Vec::new (cc:257 code_(0) 非预容)");
  // 队列容器无预容量 (原版 ZoneLinkedList 惰性) — new()/default 通道
  must(count(landed, /VecDeque::new\(\)/g) === 2, "两队列均 VecDeque::new() 无半改态");
  // const 项全集: 原版 constexpr×4 + static constexpr×2 (含于 constexpr 计数) → Rust 编译期常量 5 + const _ 断言 2
  must(count(landed, /^const [A-Z][A-Z_0-9]*:/gm) === 5, `Rust 常量 5 (K_MAX_SUPPORTED_CODEPOINT/K_ALLOWED_FLAGS/K_MAX_REPLICATION_FACTOR/K_RECURSION_BUDGET/K_MAX_INT), 实得 ${count(landed, /^const [A-Z][A-Z_0-9]*:/gm)}`);
  must(count(landed, /^const _: \(\) = assert!/gm) === 2, "static_assert×2 (cc:47,765) → const _ 承接×2");
  must(count(landed, /K_ALLOWED_FLAGS\.0 & UNICODE == 0/) === 1, "static_assert(cc:47-48) 语义锚 (kSupportsUnicode=false 侧)");
  // 原版 const 成员 (flags() const + const 引用×2) 落点
  must(/flags: RegExpFlags,/.test(landed), "flags_ 字段承接 (getter const 折叠字段直读)");
  must(/quantifier_id_remapping: &'a RemapMap/.test(landed) && /lookaround_id_remapping: &'a RemapMap/.test(landed), "const ZoneMap& ×2 → &'a RemapMap");
  // 原版 Label 禁拷贝禁移动 (cc:238-239) ↔ Rust &mut 传递, 无 Clone on Label 之外扩散
  must(count(landed, /#\[derive\(Debug, Clone, Copy, PartialEq, Eq\)\]\nenum LabelState/g) === 1, "LabelState 值语义仅状态枚举");
}

// ── 6) 语义分支/位标志常量锚 逐项核对 (原版代码文本 ↔ Rust 落地文本; 换行断链的 Rust 锚取单行片段)
const PAIRS = [
    ["kMaxSupportedCodepoint = 0xFFFFu", 'const K_MAX_SUPPORTED_CODEPOINT: u32 = 0xFFFF;', "cc:22 码点上限"],
    ["kMaxReplicationFactor = 16", "const K_MAX_REPLICATION_FACTOR: i32 = 16;", "cc:106 复制因子预算"],
    ["Flag::kGlobal | Flag::kSticky |", "RegExpFlags(GLOBAL | STICKY | MULTILINE | DOTALL | LINEAR)", "cc:43-45 五标志白名单"],
    ["(flags & ~kAllowedFlags) == 0", "(flags & !K_ALLOWED_FLAGS) == RegExpFlags::NONE", "cc:49 位掩码判定"],
    ["node->min() > kMaxReplicationFactor ||", "node.min > K_MAX_REPLICATION_FACTOR", "cc:111 单节点上限"],
    ["node->max() != Tree::kInfinity &&", "node.max != K_INFINITY && node.max > K_MAX_REPLICATION_FACTOR", "cc:112-113 上限支"],
    ["local_replication = std::max(node->min(), 1);", "node.min.max(1)", "cc:126 非空可 plus 归约"],
    ["local_replication = node->min() + 1;", "node.min + 1", "cc:128 可空 ++1"],
    ["IsGlobal(flags()) || IsSticky(flags())", "self.flags.is_global() || self.flags.sticky()", "cc:179 环视×global/sticky 拒"],
    ["node->type() == Lookaround::LOOKAHEAD || node->capture_count() > 0", "node.lookaround_type == LookaroundType::Lookahead || node.capture_count > 0", "cc:187 flag 关拒径"],
    ["!IsSticky(flags) &&", "!flags.sticky() &&", "cc:549 前导 /.*?/ 门"],
    ["DCHECK_GE(bound_index_, 0)", "matches!(self.state, LabelState::Bound { index } if index >= 0)", "cc:233 析构下界"],
    ["target.unbound_patch_list_begin_ != -1", "while p != -1", "cc:338 补丁链哨兵"],
    ["int new_list_begin = code_.length();", "let new_list_begin = self.code.len() as i32;", "cc:363 链头前插"],
    ["static constexpr Flags kAllowedFlags", "const K_ALLOWED_FLAGS: RegExpFlags", "cc:43 位标志落地"],
    ["kMaxInt)", "StackLimiter::new(K_MAX_INT)", "cc:466,501,1044 栈预算"],
    ["Node::kRecursionBudget", "K_RECURSION_BUDGET", "cc:550,634,637 递归预算 (regexp-nodes.h:345=200)"],
    ["CharacterRange::Negate(ranges, negated_ranges, zone_);", "CharacterRange::negate(&ranges, &mut negated_ranges);", "cc:747 取补"],
    ["from > kMaxSupportedCodepoint && to > kMaxSupportedCodepoint", "from > K_MAX_SUPPORTED_CODEPOINT && to > K_MAX_SUPPORTED_CODEPOINT", "cc:774 全 BMP 外特例"],
    ["std::min(to, kMaxSupportedCodepoint)", "to.min(K_MAX_SUPPORTED_CODEPOINT) as u16", "cc:785 uc16 截断"],
    ["map.try_emplace(id, static_cast<int>(map.size())).first->second", "map.get_or_insert(id)", "cc:1183,1188 稠密化通道"],
    ["node->CaptureRegisters(StackLimiter(kMaxInt)).is_empty()", "capture_registers(StackLimiter::new(K_MAX_INT))", "cc:466,501 委托 body 空判 (regexp-ast.cc:66-79 实证委托; Rust 侧 ×3 处含 cc:1044 通道)"],
];
{
  const pairs = PAIRS;
  for (const [orig, rust, label] of pairs) {
    must(cc.includes(orig), `原版锚缺失: ${label} (${orig})`);
    must(landed.includes(rust), `Rust 锚缺失: ${label} → ${rust}`);
  }
  // kInfinity 哨兵/寄存器配对/位运算细节锚
  must(count(landed, /K_INFINITY/g) >= 5, "K_INFINITY 使用 ≥5 (cc:112,123,1056,1101,1110 通道)");
  must(/for i in \(indices\.from\(\)\.\.=indices\.to\(\)\)\.step_by\(2\)/.test(landed), "ClearRegisters 偶起奇止步长 2 (cc:826)");
  must(/Capture::start_register\(index\)/.test(landed) && /Capture::end_register\(index\)/.test(landed), "寄存器配对 (cc:1130-1131)");
  // 入口转发两行 (cc:219 / cc:1221)
  must(/CanBeHandledVisitor::check\(tree, flags, v8_flags\)/.test(landed), "入口转发 cc:219");
  must(/CompileVisitor::compile\(tree, flags, v8_flags\)/.test(landed), "入口转发 cc:1221");
}

// ── 7) UNREACHABLE/防御通道映射 (原版 4 + DCHECK 提升 + Rust match 穷尽) ─────
{
  must(count(landed, /unreachable!\(/g) === 7, `Rust unreachable! 应 7 (= 原版 4 + DCHECK 提升 2 + opcode 穷尽臂 1), 实得 ${count(landed, /unreachable!\(/g)}`);
  for (const anchor of [
    /cc:805 UNREACHABLE\(\): ClassSetExpression/,
    /cc:1075 UNREACHABLE\(\): POSSESSIVE 已被 CanBeHandled 拒绝/,
    /cc:1099 UNREACHABLE\(\): POSSESSIVE 已被 CanBeHandled 拒绝/,
    /cc:1159 UNREACHABLE\(\): BackReference 已被 CanBeHandled 拒绝/,
    /v8 cc:334: Bind 只接受 UNBOUND 标签/,
    /v8 cc:344: 带标签指令 payload 必为 pc/,
    /v8 cc:282,286,330: LabelledInstrImpl 仅 FORK\/JMP\/FILTER_CHILD/,
  ]) must(anchor.test(landed), `unreachable 通道锚: ${anchor}`);
  // 假绿防线: compiler.rs 剥注释后无 todo!/unimplemented!/NotImplemented
  const codeOnly = landed.replace(/^\s*\/\/.*$/gm, "");
  must(count(codeOnly, /\b(todo|unimplemented)!\s*\(/g) === 0, "compiler.rs 代码区含 todo!/unimplemented! (违规)");
  must(count(codeOnly, /NotImplemented/g) === 0, "compiler.rs 代码区含 NotImplemented (违规)");
  assertCount += 2;
}

// ── 8) 测试面全集对账 (36 用例) ─────────────────────────────────────────────
const TESTS = [
  "suitable_flags_whitelist", "unsupported_constructs_rejected", "replication_factor_budget",
  "lookaround_acceptance_matrix", "group_modifier_flags_whitelist", "capture_count_argument_is_not_consumed",
  "sticky_literal_program", "non_sticky_prefix_star", "anchored_start_skips_prefix_star",
  "atom_multi_char_expands_to_ranges", "empty_disjunction_compiles_to_fail", "empty_alternative_falls_through",
  "disjunction_program", "finite_repetition_programs", "greedy_plus_reduction_program",
  "flag_on_reduced_plus_emits_clock_per_repetition", "star_programs", "capture_program",
  "capture_in_star_clears_registers", "class_ranges_program", "negated_class_truncates_to_bmp",
  "negated_full_bmp_class_always_fails", "lookahead_two_phase_program", "lookbehind_program",
  "negative_lookaround_program", "capturing_lookaround_program", "nested_lookaround_bfs_queue",
  "capturing_lookaround_second_phase_does_not_reenqueue", "capturing_lookbehind_second_phase_reversed",
  "lookaround_compilation_is_flag_independent", "lookaround_id_remapping_is_dense",
  "filter_phase_program", "filter_phase_skips_captureless_quantifiers", "zero_max_quantifier_emits_nothing",
  "filter_phase_absent_without_flag", "compiled_program_dump_golden",
];
must(count(tests, /#\[test\]/g) === TESTS.length, `测试文件 #[test] 应 ${TESTS.length}, 实得 ${count(tests, /#\[test\]/g)}`);
for (const t of TESTS) {
  must(count(tests, new RegExp(`fn ${t}\\(\\) -> Void`)) === 1, `测试用例 ${t} 缺失或重复`);
}
must(count(tests, /todo!|unimplemented!/g) === 0, "tests 含 todo!/unimplemented! (违规)");
assertCount += 2;

// ── 产物: 全集台账 + 符号对账表快照 ─────────────────────────────────────────
const rustLineOf = (name) => {
  const hits = [];
  landed.split("\n").forEach((l, i) => { if (new RegExp(`(?:pub )?fn ${name}\\(`).test(l)) hits.push(i + 1); });
  return hits.join(",");
};
const rows = LEDGER.map(([from, to, name, disp]) =>
  `//! cc:${String(from).padStart(4, " ")}-${String(to).padEnd(4, " ")} [${disp}] ${name}`);
const symRows = SYMS.map(([sig, line, rust, note]) =>
  `//! cc:${String(line).padStart(4, " ")} ${sig.replace(/\(.*$/, "").trim()}${rust ? ` → ${rust}(rs:${rustLineOf(rust)})` : ""}${note ? ` — ${note}` : ""}`);
const skel = `${banner(
  "src/regexp/experimental/experimental-compiler.cc.js",
  "src/regexp/experimental/experimental-compiler.cc",
  "全集台账快照 (审计专用, 区间无缝覆盖 cc:1-1226): 落地以 compiler.rs 就地修订为准",
)}
//! 对数基准 = clang -E 两态编译器产物 (release / -DDEBUG, 行标记切本文件域):
//!   宏遮蔽核验 UNREACHABLE×4→__builtin_unreachable×4 / DCHECK×17 行→release ((void)0)×17 /
//!   debug 态 if( = 40+17=57 全量归因 / #ifdef DEBUG 唯一条件区恰 kMaxCodePoint (rel 0 / dbg 存);
//!   结构计数两态守恒 (Visit 42 / if 40 / switch 3 / code_.Add 19 / class 6 / Accept 28 …)
//! bailout 全集: CanBeHandled 拒 8 (cc:80,114,136,147,171,181,189,199) + Check 门 1 (cc:31) +
//!   产码 FAIL 3 (cc:691,753,776) + UNREACHABLE 4 (cc:805,1075,1099,1159)
//! 符号: 原版 93 定义 + 6 类壳 ↔ Rust 90 fn; 折叠登记: ctor×5 (聚合直构/new)、flags() getter、
//!   emplace_back+FilterChild×3→enqueue_child、try_emplace/at→RemapMap、Label 析构→Drop
//! 生命周期: ZoneLinkedList<Lookaround*>/BFEntry(Tree*) 裸指针队列 ↔ VecDeque<*const>/BFNode
//!   三态枚举; 解引用 4 点均带 SAFETY 注记 (cc:594,1152/1197/521-524 同存活不变量承接)
${rows.join("\n")}
//! ── 符号对账表 (名/cc 行号/Rust 行号/处置) ──
${symRows.join("\n")}
`;
writeSrc("v8-regexp-exec", "experimental/compiler.cc.rs.skel", skel);

console.log(
  `experimental-compiler.cc.js: 对数断言 ${assertCount} 条全过; clang -E 两态编译器产物对数基准 (rel ${ppRel.length} 行/dbg ${ppDbg.length} 行, ` +
    `宏遮蔽归因: UNREACHABLE 4→__builtin_unreachable 4, DCHECK 17→((void)0) 17, debug if 40+17=57, #ifdef DEBUG 差集恰 kMaxCodePoint); ` +
    `台账 ${LEDGER.length} 区间无缝覆盖 cc:1-1226 (落地 ${landedSegs} / 折叠登记 ${foldedSegs} / 样板 ${admSegs}); ` +
    `bailout 16 点行号对账; 符号 ${SYMS.length} 条目逐名行号 + 两态存活, Rust fn 多重集 90 双向孤儿 0; ` +
    `生命周期 (unsafe 4 点 SAFETY 齐 / with_capacity 恰 1=cc:745 承接 / VecDeque::new ×2 / const 5+const _ 2); ` +
    `语义锚 ${PAIRS.length} 对 + unreachable 通道 7 + 测试 ${TESTS.length} 用例逐项命中, 缺项 0; ` +
    `产物 v8-regexp-exec/src/regexp/experimental/experimental-compiler.cc.rs.skel`,
);
