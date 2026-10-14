//! EXP-3c 批段 (字符消费+谓词绑定) 落地测试。
//!
//! 规约来源差异申报 (与台账 experimental-interpreter.cc.js 同步登记): 票面所指
//! v8/test/unittests/regexp/experimental-interpreter-unittests.cc 实测不存在
//! (本仓 unittests/regexp/ 仅 fuzzer/grammar/unittest 三件); 原版引擎的字符消费
//! 阻塞/唤醒与断言绑定用例面实际位于 mjsunit regexp-experimental.js (JS 级全链,
//! 依赖编译器 EXP-2 与入口面 EXP-4), 故本 bin 按 3b 先例以手工构造字节码序列
//! (bytecode.rs 工厂) 复刻其语义面, 逐臂核对原版行号:
//! CONSUME_RANGE/RANGE_COUNT 挂起臂 (cc:908-912 共臂体 blocked_threads_.Add +
//! return, 经 block_thread 单点化)、ASSERTION 求值绑定臂 (cc:913-920, 谓词
//! SatisfiesAssertion(cc:25-61) 绑定 input_/input_index_ 实参族 cc:914-915)、
//! 唤醒侧 flush_blocked_threads (cc:1132-1172, EXP-3a 已落) 构成的阻塞/唤醒全链
//! (find_next_match + run_active_threads_to_end 驱动, cc:831-878/697-752)。
//! 前席约束: kTicks=64→HandleInterrupts 探针本体 EXP-4 落地前, ≥64 输入位会预期性
//! 失败 — 本 bin e2e 输入恒 <64 位, 尊重原版时钟/长度语义, 不绕过探针。

use aok::{OK, Void};
use v8_regexp_exec::experimental::{
  bytecode::{Instruction, Opcode, Payload},
  interpreter::{
    ConsumedCharacter, InterpreterError, InterpreterThread, K_UNDEFINED_REGISTER_VALUE,
    LastInputIndex, NfaInterpreter,
  },
};
use v8_regexp_syntax::AssertionType;

#[ctor::ctor(unsafe)]
fn _log_init() {
  log_init::init();
}

// ── 装配辅助 (与 3b bin 同款: 寄存器数恒 2, 线程与解释器计数同源) ────────────

fn make_nfa<'a>(bytecode: &'a [Instruction], input: &'a [u16], flag: bool) -> NfaInterpreter<'a> {
  NfaInterpreter::new(bytecode, input, 0, 2, flag).unwrap()
}

fn thread_at(host: &NfaInterpreter<'_>, pc: i32) -> InterpreterThread {
  InterpreterThread::new_empty(
    pc,
    host.register_count_per_match,
    host.quantifier_count,
    host.lookarounds.len(),
    host.capture_group_opt,
    host.only_captureless_lookbehinds,
  )
}

// ── CONSUME_RANGE 挂起臂 (cc:908-912) ────────────────────────────────────────

#[test]
fn c3c_consume_range_blocks() -> Void {
  // [0 CONSUME_RANGE 'a'-'z', 1 ACCEPT]: 臂体 = blocked_threads_.Add(t) + return
  // success (cc:910-911); pc 不自增 — 唤醒侧 flush 在同一输入位比对 (cc:1143)
  let bytecode = [
    Instruction::consume_range(0x0061, 0x007A),
    Instruction::accept(),
  ];
  let mut nfa = make_nfa(&bytecode, &[0x0061], false);
  let mut t = thread_at(&nfa, 0);
  t.register_array[0] = 42; // 线程身份标记
  nfa.run_active_thread(t).unwrap();

  assert_eq!(nfa.clock, 1, "preamble ++clock 一台 (cc:891)");
  assert_eq!(nfa.blocked_threads.len(), 1, "挂起入 blocked (cc:910)");
  let blocked = &nfa.blocked_threads[0];
  assert_eq!(blocked.pc, 0, "臂体无 ++pc (cc:908-912)");
  assert_eq!(
    blocked.register_array[0], 42,
    "线程原样入队 (寄存器快照不动)"
  );
  assert_eq!(
    blocked.consumed_since_last_quantifier,
    ConsumedCharacter::DidConsume,
    "consumed 态臂内不改 (cc:908-912 仅入队)"
  );
  assert!(nfa.active_threads.is_empty(), "按值收线程已出队");
  assert!(!nfa.found_match());
  // preamble 去重登记已落 (cc:903), 与臂体无副作用正交
  assert_eq!(nfa.pc_last_input_index[0].having_consumed_character, 0);

  // 同 (pc, consumed) 再投一条: preamble 去重短路 (cc:899-902), 不入 blocked 第二次
  nfa.run_active_thread(thread_at(&nfa, 0)).unwrap();
  assert_eq!(nfa.clock, 2);
  assert_eq!(nfa.blocked_threads.len(), 1, "冗余线程销毁, 不入队");
  OK
}

// ── RANGE_COUNT 挂起臂 (cc:908-912) + 唤醒侧多区间扫描配对 ───────────────────

#[test]
fn c3c_range_count_blocks() -> Void {
  // [0 RANGE_COUNT 2, 1 CR '0'-'9', 2 CR 'a'-'f', 3 ACCEPT]: 与 CONSUME_RANGE
  // 同臂体挂起 (cc:908-912 双 case 折叠); 唤醒侧按 RANGE_COUNT 头连查 2 个范围,
  // 命中任一即 pc 跳全区间末 3 (cc:1143-1158, EXP-3a flush 已落 — 本臂配对面)
  let bytecode = [
    Instruction::range_count(2),
    Instruction::consume_range(0x0030, 0x0039),
    Instruction::consume_range(0x0061, 0x0066),
    Instruction::accept(),
  ];
  let mut nfa = make_nfa(&bytecode, &[0x0062], false);
  nfa.run_active_thread(thread_at(&nfa, 0)).unwrap();

  assert_eq!(nfa.clock, 1);
  assert_eq!(
    nfa.blocked_threads.len(),
    1,
    "RANGE_COUNT 同路径挂起 (cc:908-912)"
  );
  assert_eq!(
    nfa.blocked_threads[0].pc, 0,
    "挂起于 RANGE_COUNT 头, pc 未自增"
  );

  // 唤醒 'b' (0x0062): 头 pc0 RANGE_COUNT → ranges=2, pc→1; 扫 pc1 数字域不中,
  // pc2 a-f 中 → t.pc = next_pc = 3 (cc:1149-1158)
  nfa.flush_blocked_threads(0x0062).unwrap();
  assert!(nfa.blocked_threads.is_empty());
  assert_eq!(nfa.active_threads.len(), 1);
  assert_eq!(
    nfa.active_threads[0].pc, 3,
    "多区间命中跳全区间末 (cc:1158)"
  );
  assert_eq!(
    nfa.active_threads[0].consumed_since_last_quantifier,
    ConsumedCharacter::DidConsume,
    "唤醒标记已消费 (cc:1159-1160)"
  );
  OK
}

// ── ASSERTION 求值绑定臂 (cc:913-920) ─────────────────────────────────────────

#[test]
fn c3c_assertion_pass_advance() -> Void {
  // [0 ASSERTION START_OF_INPUT, 1 ACCEPT]: 谓词真 → ++t.pc 续跑 (cc:919-920),
  // 线程不销毁不入 blocked; input_index=0 时 ^ 成立
  let bytecode = [
    Instruction::assertion(AssertionType::StartOfInput),
    Instruction::accept(),
  ];
  let mut nfa = make_nfa(&bytecode, &[0x0061, 0x0062], false);
  nfa.run_active_thread(thread_at(&nfa, 0)).unwrap();

  assert!(nfa.found_match(), "谓词真值路径应续跑至 ACCEPT");
  assert_eq!(nfa.best_match_thread.as_ref().unwrap().pc, 1);
  assert_eq!(nfa.clock, 2, "ASSERTION+ACCEPT 两台 (cc:919 续循环)");
  assert!(nfa.blocked_threads.is_empty(), "通过路径不挂起");
  assert!(nfa.active_threads.is_empty());
  OK
}

#[test]
fn c3c_assertion_fail_destroy() -> Void {
  // [0 ASSERTION END_OF_INPUT, 1 ACCEPT] 于 input_index=0 (非串尾): 谓词假 →
  // DestroyThread + return success (cc:916-918, Rust drop); ACCEPT 臂不得触达
  let bytecode = [
    Instruction::assertion(AssertionType::EndOfInput),
    Instruction::accept(),
  ];
  let mut nfa = make_nfa(&bytecode, &[0x0061, 0x0062], false);
  nfa.run_active_thread(thread_at(&nfa, 0)).unwrap();

  assert_eq!(nfa.clock, 1, "仅 ASSERTION 一台即销毁");
  assert!(!nfa.found_match(), "销毁路径不落定位");
  assert!(nfa.blocked_threads.is_empty() && nfa.active_threads.is_empty());
  assert!(nfa.best_match_thread.is_none());
  // 销毁线程不可复活: 重投同态线程 → preamble 去重短路 (cc:899-902)
  nfa.run_active_thread(thread_at(&nfa, 0)).unwrap();
  assert_eq!(nfa.clock, 2);
  assert!(!nfa.found_match());
  OK
}

#[test]
fn c3c_assertion_input_index_binding() -> Void {
  // 绑定核验: 原版 cc:914-915 实参 = (inst.payload.assertion_type, input_,
  // input_index_) — 谓词必须读解释器当前输入位而非恒 0。
  // 输入 "ab c" (W W N W), BOUNDARY (cc:46-56) 逐位真值: 0真 1假 2真 3真 4真
  let bytecode = [
    Instruction::assertion(AssertionType::Boundary),
    Instruction::accept(),
  ];
  let input = [0x0061u16, 0x0062, 0x0020, 0x0063];
  let expect = [true, false, true, true, true];
  for (pos, want) in expect.iter().enumerate() {
    let mut nfa = NfaInterpreter::new(&bytecode, &input, pos as i32, 2, false).unwrap();
    nfa.run_active_thread(thread_at(&nfa, 0)).unwrap();
    assert_eq!(nfa.found_match(), *want, "BOUNDARY @ input_index={pos}");
    assert_eq!(
      nfa.clock,
      if *want { 2 } else { 1 },
      "假值即销毁 (cc:916-918) / 真值续跑 (cc:919-920) @ {pos}"
    );
  }
  // NON_BOUNDARY 逐位取反绑定 (cc:57-58 递归取反臂经本臂消费)
  let nb = [
    Instruction {
      opcode: Opcode::Assertion,
      payload: Payload::Assertion(AssertionType::NonBoundary),
    },
    Instruction::accept(),
  ];
  for (pos, want) in expect.iter().enumerate() {
    let mut nfa = NfaInterpreter::new(&nb, &input, pos as i32, 2, false).unwrap();
    nfa.run_active_thread(thread_at(&nfa, 0)).unwrap();
    assert_eq!(nfa.found_match(), !want, "NON_BOUNDARY @ input_index={pos}");
  }
  OK
}

// ── 阻塞/唤醒全链 e2e (挂起臂 × 调度外围 × flush) ────────────────────────────

#[test]
fn c3c_block_wakeup_e2e_match() -> Void {
  // 复刻 mjsunit Test(/[a-z]/, …) 消费面: [0 CR 'a'-'z', 1 SET_REG 0, 2 ACCEPT]
  // 输入 "1a": 起始位 0 — '1' 不中唤醒 (线程销毁, cc:1167-1168), 整轮无匹配;
  // 起始位 1 (原版 FindMatches 逐位重启动机消费面 cc:486-539 → EXP-4, 此处手工
  // 重投 find_next_match) — 'a' 唤醒续跑 ACCEPT, 置存 = 唤醒后 input_index
  let bytecode = [
    Instruction::consume_range(0x0061, 0x007A),
    Instruction::set_register_to_cp(0),
    Instruction::accept(),
  ];
  let input = [0x0031u16, 0x0061]; // '1' 'a'
  let mut nfa = make_nfa(&bytecode, &input, false);

  // 第 1 轮 (起始位 0): 主线程挂起 → flush('1') 不中销毁 → 输入耗尽无匹配
  nfa.find_next_match().unwrap();
  assert!(!nfa.found_match(), "'1' 不在 [a-z], 线程销毁后无后续");
  assert_eq!(nfa.input_index, 2, "输入喂尽 (cc:714-716 正向轨界)");
  assert!(nfa.blocked_threads.is_empty() && nfa.active_threads.is_empty());

  // 第 2 轮 (起始位 1): 重投线程挂起 → flush('a') 命中 → pc1 置存 → ACCEPT
  nfa.set_input_index(1);
  nfa.find_next_match().unwrap();
  assert!(
    nfa.found_match(),
    "'a' 唤醒命中后 ACCEPT (cc:908-912×cc:1156-1166 全链)"
  );
  let best = nfa.best_match_thread.as_ref().unwrap();
  assert_eq!(best.pc, 2);
  assert_eq!(
    best.register_array,
    vec![2, K_UNDEFINED_REGISTER_VALUE],
    "SET_REGISTER_TO_CP 置存 = 唤醒后 input_index (2)"
  );
  assert_eq!(nfa.input_index, 2);
  assert!(
    nfa.blocked_threads.is_empty(),
    "FoundMatch 且 blocked 空 → 停驱 (cc:714-716)"
  );

  // 第 3 轮 (起始位 2, 尾后位): 挂起线程无字符可唤醒 → 轮末仍 blocked, 无匹配
  nfa.set_input_index(2);
  nfa.find_next_match().unwrap();
  assert!(
    !nfa.found_match(),
    "上一轮 best 于 find_next_match 入口清除 (cc:859-862)"
  );
  assert_eq!(
    nfa.blocked_threads.len(),
    1,
    "输入耗尽时挂起线程保留 (cc:714 轨界先于唤醒)"
  );
  assert_eq!(nfa.blocked_threads[0].pc, 0);
  OK
}

#[test]
fn c3c_range_count_wakeup_e2e() -> Void {
  // [0 RANGE_COUNT 2, 1 CR '0'-'9', 2 CR 'a'-'f', 3 ACCEPT] 经 find_next_match:
  // 挂起 (cc:908-912) → 字符 '5' 唤醒命中数字区间 → pc3 ACCEPT
  let bytecode = [
    Instruction::range_count(2),
    Instruction::consume_range(0x0030, 0x0039),
    Instruction::consume_range(0x0061, 0x0066),
    Instruction::accept(),
  ];
  let input = [0x0035u16]; // '5'
  let mut nfa = make_nfa(&bytecode, &input, false);
  nfa.find_next_match().unwrap();
  assert!(nfa.found_match());
  let best = nfa.best_match_thread.as_ref().unwrap();
  assert_eq!(best.pc, 3, "RANGE_COUNT 区间末落位");
  assert_eq!(
    nfa.clock, 2,
    "挂起 1 台 + ACCEPT 1 台 (flush 不计数, cc:1132-1172)"
  );
  assert!(nfa.blocked_threads.is_empty() && nfa.active_threads.is_empty());

  // 'g' 双区间皆不中 → 唤醒即销毁, 全链无匹配
  let input2 = [0x0067u16];
  let mut nfa = make_nfa(&bytecode, &input2, false);
  nfa.find_next_match().unwrap();
  assert!(!nfa.found_match());
  assert!(nfa.blocked_threads.is_empty() && nfa.active_threads.is_empty());
  assert_eq!(nfa.clock, 1, "仅挂起一台");
  OK
}

#[test]
fn c3c_blocked_priority_via_arms() -> Void {
  // 优先级不变量经臂体核验: blocked 高→低序 (active 尾弹先挂者先入队), 唤醒
  // 逆序使 active 复现低→高 (cc:1133-1136); 高优先线程 (pc0 [a-z]) 挂起在前,
  // 低优先 (pc2 [A-Z]) 在后, 'a' 唤醒: 低者不中销毁, 高者命中 ACCEPT
  let bytecode = [
    Instruction::consume_range(0x0061, 0x007A),
    Instruction::accept(),
    Instruction::consume_range(0x0041, 0x005A),
    Instruction::accept(),
  ];
  let mut nfa = make_nfa(&bytecode, &[], false);
  let mut low = thread_at(&nfa, 2);
  low.register_array[0] = 11;
  let mut high = thread_at(&nfa, 0);
  high.register_array[0] = 22;
  nfa.active_threads.extend([low, high]); // active 低→高 (cc:1510-1512 序约定)

  nfa.run_active_threads().unwrap();
  assert_eq!(nfa.blocked_threads.len(), 2, "两线程经 cc:908-912 挂起");
  assert_eq!(
    nfa.blocked_threads[0].register_array[0], 22,
    "高优先先弹先挂起"
  );
  assert_eq!(nfa.blocked_threads[1].register_array[0], 11);

  nfa.flush_blocked_threads(0x0061).unwrap(); // 'a'
  assert!(nfa.blocked_threads.is_empty());
  assert_eq!(
    nfa.active_threads.len(),
    1,
    "低优先 [A-Z] 不中销毁 (cc:1167-1168)"
  );
  nfa.run_active_threads().unwrap();
  let best = nfa.best_match_thread.expect("高优先线程 ACCEPT");
  assert_eq!(best.register_array[0], 22);
  assert_eq!(nfa.clock, 3, "两挂起 + 一 ACCEPT, flush 不计数");
  OK
}

// ── 防御面 (恶意 input_index 显式拒, 禁回绕假绿) ─────────────────────────────

#[test]
fn c3c_defense_negative_input_index() -> Void {
  // 原版 cc:28-29 DCHECK_LE/GE 前置; Rust ASSERTION 臂以 try_from 显式拒负
  // input_index (usize 回绕会让谓词读越界/假绿)。去重表预置非 -1 值, 避开
  // input_index=-1 与 LastInputIndex 初值哨兵重合导致的 preamble 短路 (cc:899)
  let bytecode = [
    Instruction::assertion(AssertionType::StartOfInput),
    Instruction::accept(),
  ];
  let mut nfa = make_nfa(&bytecode, &[0x0061], false);
  nfa.pc_last_input_index[0] = LastInputIndex::new(5, 6);
  nfa.input_index = -1;

  let Err(err) = nfa.run_active_thread(thread_at(&nfa, 0)) else {
    panic!("负 input_index 于 ASSERTION 臂应显式拒绝");
  };
  match err {
    InterpreterError::BadBytecode { item, orig } => {
      assert!(item.contains("ASSERTION 臂 input_index 为负"), "{item}");
      assert!(orig.starts_with("cc:914-915"), "{orig}");
    }
    other => panic!("期望 BadBytecode, 实得 {other}"),
  }
  assert_eq!(
    nfa.clock, 1,
    "preamble 已 ++clock 后臂内失败 (cc:891 先于分发)"
  );
  assert!(
    nfa.blocked_threads.is_empty() && nfa.active_threads.is_empty() && !nfa.found_match(),
    "失败路径状态未变"
  );
  OK
}
