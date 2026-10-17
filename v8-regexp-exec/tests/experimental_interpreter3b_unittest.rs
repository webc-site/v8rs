//! EXP-3b 批段 (线程调度+状态机转移) 端到端测试。
//!
//! 手工构造字节码序列向量 (bytecode.rs 工厂) + 合成输入, 逐臂核对原版语义:
//! FORK 拷贝/入队序 (cc:921-984)、JMP (cc:985-987)、ACCEPT 落定位+低优先级清除
//! (cc:988-998)、SET_QUANTIFIER_TO_CLOCK (cc:999-1002)、CLEAR_REGISTER
//! (cc:1004-1010)、SET_REGISTER_TO_CP (cc:1011-1019)、BEGIN/END_LOOP
//! (cc:1025-1040)、START/END_LOOKAROUND (cc:1041-1055)、preamble 去重短路
//! (cc:899-903)、调度外围 RunActiveThreads (cc:1120-1127) /
//! RunActiveThreadsToEnd (cc:697-752) / FindNextMatch (cc:831-878)。
//! 推迟臂 (EXP-3c/3d/4) 仅断言 NotImplemented 挂起形态, 不执行臂体 (禁假绿);
//! blocked/唤醒全链 e2e 依赖 EXP-3c 字符消费臂, 本票以手动置入 blocked 的
//! 区间线程核验唤醒覆盖 best 的调度序 (cc:739-748)。

use aok::{OK, Void};
use v8_regexp_exec::experimental::{
  bytecode::{Instruction, Opcode, Payload},
  interpreter::{
    ConsumedCharacter, EXP_BATCH_3D, EXP_BATCH_4, InterpreterError, InterpreterThread,
    K_UNDEFINED_REGISTER_VALUE, NfaInterpreter,
  },
};
use v8_regexp_syntax::LookaroundType;

#[ctor::ctor(unsafe)]
fn _log_init() {
  log_init::init();
}

// ── 装配辅助 (寄存器数恒 2, 线程经 new_empty 与解释器计数同源) ────────────────

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

// ── FORK (cc:921-984) ─────────────────────────────────────────────────────────

#[test]
fn sched_fork_copies_and_priority() -> Void {
  // 程序: [0 置存1, 1 FORK 4, 2 BEGIN_LOOP, 3 END_LOOP, 4 置存0, 5 ACCEPT]
  // 母线程 pc0-3 因空串迭代销毁; fork 线程 (拷贝母线程 pc1 时寄存器快照) 入队尾,
  // 由 run_active_threads 弹出续跑 — 核对 cc:922-975 拷贝/入队/优先级承接
  let bytecode = [
    Instruction::set_register_to_cp(1),
    Instruction::fork(4),
    Instruction::begin_loop(),
    Instruction::end_loop(),
    Instruction::set_register_to_cp(0),
    Instruction::accept(),
  ];
  let mut nfa = make_nfa(&bytecode, &[], false);
  let mother = thread_at(&nfa, 0);
  nfa.run_active_thread(mother).unwrap();

  // 母线程 END_LOOP 销毁 (return Ok), fork 挂起在 active 尾
  assert!(!nfa.found_match(), "母线程销毁路径不应落定位");
  assert_eq!(nfa.active_threads.len(), 1, "fork 线程入队 (cc:975)");
  let fork = &nfa.active_threads[0];
  assert_eq!(fork.pc, 4, "fork 目标 pc (cc:922)");
  assert_eq!(
    fork.register_array,
    vec![K_UNDEFINED_REGISTER_VALUE, 0],
    "fork 寄存器拷贝母线程 FORK 时点快照: [-1, 置存(输入位0)=0] (cc:926-930)"
  );
  assert_eq!(
    fork.consumed_since_last_quantifier,
    ConsumedCharacter::DidConsume,
    "fork 继承母线程 consumed 态 (cc:923-924)"
  );
  assert_eq!(
    nfa.clock, 4,
    "母线程 pc0-3 共 4 指令 (cc:891 每指令 ++clock)"
  );

  nfa.run_active_threads().unwrap();
  assert_eq!(nfa.clock, 6, "fork 续跑 pc4-5 两台指令");
  assert!(nfa.found_match());
  assert_eq!(
    nfa.best_match_thread.as_ref().unwrap().register_array,
    vec![0, 0],
    "fork 线程 ACCEPT 落定位: pc4 置存 + 拷贝快照 (cc:988-998)"
  );
  assert!(nfa.active_threads.is_empty());
  OK
}

#[test]
fn sched_fork_target_negative() -> Void {
  // 原版 cc:922 int 直存由下轮 SBXCHECK 兜获; Rust 装配点显式拒且未入队 (状态未变)
  let bytecode = [Instruction::fork(-1), Instruction::accept()];
  let mut nfa = make_nfa(&bytecode, &[], false);
  let t = thread_at(&nfa, 0);
  let Err(err) = nfa.run_active_thread(t) else {
    panic!("FORK 负目标应显式拒绝");
  };
  match err {
    InterpreterError::BadBytecode { item, orig } => {
      assert!(item.contains("FORK 臂 目标 pc 为负"), "{item}");
      assert!(orig.starts_with("cc:922"), "{orig}");
    }
    other => panic!("期望 BadBytecode, 实得 {other}"),
  }
  assert_eq!(
    nfa.clock, 1,
    "preamble 已 ++clock 后臂内失败 (cc:891 先于分发)"
  );
  assert!(nfa.active_threads.is_empty(), "失败未入队, 状态未变");
  assert!(!nfa.found_match());
  OK
}

#[test]
fn sched_fork_flag_deferred_memory_probe() -> Void {
  // flag 开 FORK 完成拷贝/入队后经 cc:977-980 内存治理接线 — 探针显式失败
  // (本体 EXP-3d), 断言仅接线可达与挂起形态, 不执行探针本体
  let bytecode = [Instruction::fork(1), Instruction::accept()];
  let mut nfa = make_nfa(&bytecode, &[], true);
  let t = thread_at(&nfa, 0);
  let Err(err) = nfa.run_active_thread(t) else {
    panic!("flag 开 FORK 应经内存探针显式失败");
  };
  match err {
    InterpreterError::NotImplemented { item, orig, batch } => {
      assert_eq!(item, "CheckMemoryConsumption");
      assert_eq!(orig, "cc:1183-1208");
      assert_eq!(batch, EXP_BATCH_3D);
    }
    other => panic!("期望 NotImplemented 探针, 实得 {other}"),
  }
  assert_eq!(
    nfa.active_threads.len(),
    1,
    "fork 已入队 (cc:975) 后才触达接线 (cc:977-980)"
  );
  assert_eq!(nfa.clock, 1);
  assert!(!nfa.found_match());
  OK
}

// ── ACCEPT / JMP / preamble (cc:886-903, 985-998) ─────────────────────────────

#[test]
fn sched_accept_replaces_and_clears() -> Void {
  // [0 ACCEPT, 1 ACCEPT]: 首个 ACCEPT 落定位并清低优先级 active; blocked 不动
  // (其成员优先级更高, 唤醒后可覆盖 best — cc:988-998 + cc:714-716 论证)
  let bytecode = [Instruction::accept(), Instruction::accept()];
  let mut nfa = make_nfa(&bytecode, &[], false);

  let mut first = thread_at(&nfa, 0);
  first.register_array[0] = 11;
  let mut junk = thread_at(&nfa, 0);
  junk.register_array[0] = 77;
  nfa.active_threads.push(junk);
  let mut sentinel = thread_at(&nfa, 1);
  sentinel.register_array[0] = 99;
  nfa.blocked_threads.push(sentinel);

  nfa.run_active_thread(first).unwrap();
  assert!(nfa.found_match());
  assert!(
    nfa.active_threads.is_empty(),
    "ACCEPT 清除全部剩余 active (cc:996)"
  );
  assert_eq!(nfa.blocked_threads.len(), 1, "blocked 优先级更高, 不得清除");
  assert_eq!(
    nfa.best_match_thread.as_ref().unwrap().register_array[0],
    11
  );

  // 第二个 ACCEPT (pc1 另位避去重) 覆盖旧 best (旧线程销毁 = Option 覆写 drop)
  let mut second = thread_at(&nfa, 1);
  second.register_array[0] = 22;
  nfa.active_threads.push(thread_at(&nfa, 1));
  nfa.run_active_thread(second).unwrap();
  assert_eq!(
    nfa.best_match_thread.as_ref().unwrap().register_array[0],
    22
  );
  assert_eq!(nfa.blocked_threads.len(), 1, "两轮 ACCEPT 均不动 blocked");
  OK
}

#[test]
fn sched_jmp_and_pc_dedup() -> Void {
  // [0 JMP 0]: 第一轮跳转, 第二轮 (pc, consumed) 去重短路销毁 (cc:899-902)
  let bytecode = [Instruction::jmp(0)];
  let mut nfa = make_nfa(&bytecode, &[], false);
  let t = thread_at(&nfa, 0);
  nfa.run_active_thread(t).unwrap();
  assert_eq!(nfa.clock, 2, "取指两次: 标记 + 去重短路各 ++clock (cc:891)");
  assert!(!nfa.found_match());
  assert!(nfa.active_threads.is_empty());

  // 再来一条同 (pc, consumed) 线程: 循环头即刻销毁, 仅 +1 clock
  let again = thread_at(&nfa, 0);
  nfa.run_active_thread(again).unwrap();
  assert_eq!(nfa.clock, 3);

  // JMP 正常跳转: [0 JMP 2, 1 诱饵, 2 ACCEPT]
  let jmp = [
    Instruction::jmp(2),
    Instruction::jmp(0),
    Instruction::accept(),
  ];
  let mut nfa = make_nfa(&jmp, &[], false);
  nfa.run_active_thread(thread_at(&nfa, 0)).unwrap();
  assert!(nfa.found_match(), "JMP 目标 ACCEPT (cc:986)");
  assert_eq!(
    nfa.best_match_thread.as_ref().unwrap().pc,
    2,
    "跳至 pc2 后 ACCEPT, pc 不自增 (cc:988-998)"
  );
  OK
}

// ── 寄存器/量词状态臂 (cc:999-1019) ───────────────────────────────────────────

#[test]
fn sched_set_quantifier_clock() -> Void {
  // flag 开: [0 SET_QUANTIFIER 0, 1 ACCEPT] — 量词时钟 = 执行时刻 clock (cc:1000)
  let bytecode = [
    Instruction::set_quantifier_to_clock(0),
    Instruction::accept(),
  ];
  let mut nfa = make_nfa(&bytecode, &[], true);
  assert_eq!(nfa.quantifier_count, 1, "构造器扫描 max(id+1) (cc:441)");
  nfa.run_active_thread(thread_at(&nfa, 0)).unwrap();
  let best = nfa.best_match_thread.as_ref().unwrap();
  assert_eq!(
    best.quantifier_clock_array,
    Some(vec![1]),
    "首指令执行后 clock==1 记入量词 0 (cc:999-1001)"
  );

  // 索引越界 (负 id 使扫描计数不动): cc:1221-1227 DCHECK 族 → Rust BadBytecode
  let bad = [
    Instruction {
      opcode: Opcode::SetQuantifierToClock,
      payload: Payload::QuantifierId(-1),
    },
    Instruction::accept(),
  ];
  let mut nfa = make_nfa(&bad, &[], true);
  let Err(InterpreterError::BadBytecode { item, orig }) = nfa.run_active_thread(thread_at(&nfa, 0))
  else {
    panic!("负量词索引应显式拒绝");
  };
  assert!(
    item.contains("SET_QUANTIFIER_TO_CLOCK 臂 量词索引越界"),
    "{item}"
  );
  assert!(orig.contains("cc:999"), "{orig}");
  assert_eq!(nfa.clock, 1);
  OK
}

#[test]
fn sched_clear_register() -> Void {
  // [0 CLEAR_REGISTER 0, 1 ACCEPT]: 寄存器复位哨兵 -1 (cc:1007)
  let bytecode = [Instruction::clear_register(0), Instruction::accept()];
  let mut nfa = make_nfa(&bytecode, &[], false);
  let mut t = thread_at(&nfa, 0);
  t.register_array[0] = 9;
  t.register_array[1] = 8;
  nfa.run_active_thread(t).unwrap();
  let best = nfa.best_match_thread.as_ref().unwrap();
  assert_eq!(
    best.register_array,
    vec![K_UNDEFINED_REGISTER_VALUE, 8],
    "仅索引 0 复位, 其余不动 (cc:1005-1009)"
  );
  assert_eq!(nfa.clock, 2);
  OK
}

#[test]
fn sched_set_register_cp_capture_clock() -> Void {
  // flag 关: [0 SET_REGISTER_TO_CP 1, 1 ACCEPT] — 寄存器 = 当前 input_index (cc:1014)
  let bytecode = [Instruction::set_register_to_cp(1), Instruction::accept()];
  let mut nfa = make_nfa(&bytecode, &[0x0061, 0x0062], false);
  nfa.set_input_index(1);
  nfa.run_active_thread(thread_at(&nfa, 0)).unwrap();
  let best = nfa.best_match_thread.as_ref().unwrap();
  assert_eq!(
    best.register_array,
    vec![K_UNDEFINED_REGISTER_VALUE, 1],
    "置存当前输入位 1 (cc:1014)"
  );
  assert_eq!(
    best.capture_clock_array, None,
    "flag 关无捕获时钟 (cc:1350-1354)"
  );

  // flag 开: 同程序追加捕获时钟 = 执行时刻 clock (cc:1015-1017)
  let mut nfa = make_nfa(&bytecode, &[0x0061, 0x0062], true);
  nfa.set_input_index(1);
  nfa.run_active_thread(thread_at(&nfa, 0)).unwrap();
  let best = nfa.best_match_thread.as_ref().unwrap();
  assert_eq!(
    best.capture_clock_array,
    Some(vec![0, 1]),
    "clock==1 记入寄存器 1"
  );
  assert_eq!(nfa.clock, 2);
  OK
}

#[test]
fn sched_register_bounds_defense() -> Void {
  // SBXCHECK_BOUNDS (cc:1005/1012) → 显式拒, 状态未变 (pc 未自增, 线程未回队)
  let cases = [
    (
      Instruction::set_register_to_cp(9),
      "SET_REGISTER_TO_CP 臂 寄存器索引越界",
      "cc:1012-1013",
    ),
    (
      Instruction::clear_register(-1),
      "CLEAR_REGISTER 臂 寄存器索引越界",
      "cc:1005-1006",
    ),
  ];
  for (inst, want_item, want_orig) in cases {
    let bytecode = [inst, Instruction::accept()];
    let mut nfa = make_nfa(&bytecode, &[], false);
    let Err(err) = nfa.run_active_thread(thread_at(&nfa, 0)) else {
      panic!("{want_item}: 应显式拒绝");
    };
    match err {
      InterpreterError::BadBytecode { item, orig } => {
        assert!(item.contains(want_item), "{item}");
        assert_eq!(orig, want_orig);
      }
      other => panic!("期望 BadBytecode, 实得 {other}"),
    }
    assert_eq!(nfa.clock, 1, "preamble 后臂内失败");
    assert!(nfa.active_threads.is_empty() && !nfa.found_match());
  }
  OK
}

// ── 量词环 (cc:1025-1040) 与环视主轨 (cc:1041-1055) ───────────────────────────

#[test]
fn sched_begin_end_loop_destroy() -> Void {
  // [0 BEGIN_LOOP, 1 END_LOOP, 2 ACCEPT] 自 pc0: BEGIN 复位 DidNotConsume (cc:1027),
  // END_LOOP 见整轮未消费 → 线程销毁 (cc:1034-1036 空串量词迭代禁令)
  let bytecode = [
    Instruction::begin_loop(),
    Instruction::end_loop(),
    Instruction::accept(),
  ];
  let mut nfa = make_nfa(&bytecode, &[], false);
  nfa.run_active_thread(thread_at(&nfa, 0)).unwrap();
  assert_eq!(nfa.clock, 2, "BEGIN/END 各取指一次后销毁");
  assert!(!nfa.found_match(), "销毁路径不落定位");
  // 去重槽核对: pc0 以 DidConsume 标记 (new_empty 初值), pc1 以 DidNotConsume 标记
  assert_eq!(nfa.pc_last_input_index[0].having_consumed_character, 0);
  assert_eq!(nfa.pc_last_input_index[1].not_having_consumed_character, 0);
  assert_eq!(
    nfa.pc_last_input_index[2],
    Default::default(),
    "ACCEPT 未触达"
  );
  OK
}

#[test]
fn sched_end_loop_after_consume_advance() -> Void {
  // END_LOOP 见 DidConsume → 放行迭代 (cc:1038 ++t.pc): 自 pc1 起跑 (线程初值
  // DidConsume, 模拟一轮已消费进入环末)
  let bytecode = [
    Instruction::begin_loop(),
    Instruction::end_loop(),
    Instruction::accept(),
  ];
  let mut nfa = make_nfa(&bytecode, &[], false);
  nfa.run_active_thread(thread_at(&nfa, 1)).unwrap();
  assert_eq!(nfa.clock, 2, "END_LOOP 放行 + ACCEPT 两台 (cc:1038)");
  assert!(nfa.found_match());
  assert_eq!(nfa.best_match_thread.as_ref().unwrap().pc, 2);

  // BEGIN_LOOP 复位语义单独核验: DidConsume 线程跑 BEGIN → 后续 END_LOOP 销毁
  let mut nfa = make_nfa(&bytecode, &[], false);
  let mut t = thread_at(&nfa, 0);
  t.consumed_since_last_quantifier = ConsumedCharacter::DidConsume;
  nfa.run_active_thread(t).unwrap();
  assert!(
    !nfa.found_match(),
    "BEGIN 已复位为 DidNotConsume → END 销毁"
  );
  OK
}

#[test]
fn sched_end_lookaround_sets_best() -> Void {
  // [0 START_LOOKAROUND, 1 END_LOOKAROUND, 2 ACCEPT]: 主轨 START 透传 (cc:1042),
  // END_LOOKAROUND 与 ACCEPT 共体落定位+清 active (cc:1045-1055)
  let bytecode = [
    Instruction::start_lookaround(0, true, LookaroundType::Lookbehind),
    Instruction::end_lookaround(),
    Instruction::accept(),
  ];
  let mut nfa = make_nfa(&bytecode, &[], false);
  let mut junk = thread_at(&nfa, 2);
  junk.register_array[0] = 55;
  nfa.active_threads.push(junk);
  let t = thread_at(&nfa, 0);
  nfa.run_active_thread(t).unwrap();
  assert_eq!(
    nfa.clock, 2,
    "START/END 两台, END 不 ++pc 即落定位 (cc:1045-1053)"
  );
  let best = nfa.best_match_thread.as_ref().unwrap();
  assert_eq!(
    best.pc, 1,
    "END_LOOKAROUND 落定位时 pc 停在环末 (break 语义)"
  );
  assert!(nfa.active_threads.is_empty(), "共体清除低优先级 active");
  OK
}

// ── 调度外围 (cc:1120-1127 / 697-752 / 831-878) ───────────────────────────────

#[test]
fn sched_run_active_threads_pop_order() -> Void {
  // active 低→高序, 尾弹最高优先 (cc:1122 RemoveLast): 高优先线程先跑, 其 FORK
  // 入队再被其 ACCEPT 清除, 低优先线程根本没跑到
  let bytecode = [Instruction::fork(1), Instruction::accept()];
  let mut nfa = make_nfa(&bytecode, &[], false);
  let mut lo = thread_at(&nfa, 0);
  lo.register_array[0] = 1;
  let mut hi = thread_at(&nfa, 0);
  hi.register_array[0] = 2;
  nfa.active_threads.extend([lo, hi]);
  nfa.run_active_threads().unwrap();
  let best = nfa.best_match_thread.expect("hi 线程应抢先 ACCEPT");
  assert_eq!(best.register_array[0], 2, "尾=最高优先先弹 (cc:1120-1127)");
  assert_eq!(
    nfa.clock, 2,
    "hi: FORK+ACCEPT 两台; lo/fork 被 ACCEPT 清空未跑"
  );
  assert!(nfa.active_threads.is_empty());
  OK
}

#[test]
fn sched_run_threads_to_end_stop_conditions() -> Void {
  // (a) 无匹配无线程: 逐输入位空转至耗尽 (cc:714-716 正向轨界)
  let quiet = [Instruction::begin_loop(), Instruction::end_loop()];
  let mut nfa = make_nfa(&quiet, &[0x0061, 0x0062], false);
  nfa.run_active_threads_to_end().unwrap();
  assert_eq!(nfa.input_index, 2, "输入喂尽仍无命中 (无 64 触发: 2<64)");
  assert_eq!(nfa.clock, 0, "全程无指令执行");
  assert!(!nfa.found_match());

  // (b) blocked 唤醒覆盖 best: ACCEPT 后 blocked 非空 → 继续喂入 (cc:714-716),
  // 唤醒高优先线程再 ACCEPT 覆盖 (cc:988-998 优先级论证)
  let wake = [
    Instruction::accept(),
    Instruction::consume_range(0x0061, 0x007A),
    Instruction::accept(),
  ];
  let mut nfa = make_nfa(&wake, &[0x0061], false);
  let mut low = thread_at(&nfa, 0);
  low.register_array[0] = 11;
  nfa.active_threads.push(low);
  let mut high = thread_at(&nfa, 1);
  high.register_array[0] = 22;
  nfa.blocked_threads.push(high);
  nfa.run_active_threads_to_end().unwrap();
  assert_eq!(nfa.input_index, 1);
  assert!(nfa.blocked_threads.is_empty(), "唤醒摘空 (cc:1171 Rewind)");
  assert_eq!(
    nfa.best_match_thread.as_ref().unwrap().register_array[0],
    22,
    "高优先 blocked 线程唤醒后 ACCEPT 覆盖 best"
  );
  assert_eq!(
    nfa.clock, 2,
    "low ACCEPT 1 台 + 唤醒 high ACCEPT 1 台 (flush 不计数)"
  );

  // (c) reverse 轨 FoundMatch 即停 (环视反向自动机用, cc:714-716 反界 0<idx<=len)
  let accept = [Instruction::accept()];
  let mut nfa = make_nfa(&accept, &[0x0061, 0x0062], false);
  nfa.reverse = true;
  nfa.input_index = 2;
  nfa.active_threads.push(thread_at(&nfa, 0));
  nfa.run_active_threads_to_end().unwrap();
  assert_eq!(
    nfa.input_index, 2,
    "首轮 ACCEPT 且 blocked 空 → 条件即伪不喂入"
  );
  assert!(nfa.found_match());
  OK
}

#[test]
fn sched_interrupt_probe_at_64_ticks() -> Void {
  // cc:733-737 接线: 每 64 输入位一次 HandleInterrupts — 本体 EXP-4 探针显式失败,
  // 断言仅触发时机 (input_index 抵达 64) 与挂起形态, 不执行本体
  let bytecode = [Instruction::begin_loop(), Instruction::end_loop()];
  let input = vec![0x0061u16; 64];
  let mut nfa = make_nfa(&bytecode, &input, false);
  let Err(err) = nfa.run_active_threads_to_end() else {
    panic!("64 输入位应触达中断探针");
  };
  match err {
    InterpreterError::NotImplemented { item, orig, batch } => {
      assert_eq!(item, "HandleInterrupts");
      assert_eq!(orig, "cc:757-816");
      assert_eq!(batch, EXP_BATCH_4);
    }
    other => panic!("期望 NotImplemented 探针, 实得 {other}"),
  }
  assert_eq!(nfa.input_index, 64, "第 64 位喂入后取模命中 (cc:734)");
  assert_eq!(nfa.clock, 0, "探针先于本轮 flush/run");
  OK
}

#[test]
fn sched_find_next_match_lookaround_priority() -> Void {
  // 仅无捕获后视轨: FindNextMatch 挂起各环视线程 (cc:866-870), 后入队先弹出 =
  // 环视自动机优先于主线程跑。
  // 主线程在 pc 0 处读表 (期望 true), 环视线程在 pc 2 处写表 (写入 true)。
  // 若环视线程优先弹出运行, 表先置为 true, 主线程随后读表成功并 ACCEPT;
  // 若主线程先运行, 读到初始 false 立即销毁, 无法达成 ACCEPT。
  let bytecode = [
    // 主线程执行路径: 读表 -> accept
    Instruction::read_look_table(0, true, LookaroundType::Lookbehind),
    Instruction::accept(),
    // 环视线程执行路径 (match_pc = 2): 启动环视 -> 写表 -> 结束
    Instruction::start_lookaround(0, true, LookaroundType::Lookbehind),
    Instruction::write_look_table(0),
    Instruction::end_lookaround(),
  ];
  let mut nfa = make_nfa(&bytecode, &[0x0061], false);
  assert!(nfa.only_captureless_lookbehinds);
  assert_eq!(nfa.lookarounds.len(), 1);
  assert_eq!(nfa.lookarounds[0].match_pc, 2, "构造器登记 match_pc=2");
  assert_eq!(
    nfa.lookarounds[0].capture_pc, 4,
    "WRITE 登记 capture_pc=i+1 (cc:422)"
  );

  // 驱动搜索: 环视线程后入先出优先执行写表, 主线程随后读表成功并 accept
  assert!(nfa.find_next_match().is_ok(), "两线程正常执行收敛");
  assert!(
    nfa.found_match(),
    "环视优先写表使得主线程顺利读表命中 ACCEPT"
  );
  assert_eq!(
    nfa.lookbehind_table.as_ref().map(|t| t[0]),
    Some(true),
    "环视线程首臂优先执行并成功写入优化轨 lookbehind_table"
  );
  OK
}

#[test]
fn sched_find_next_match_zero_width_repeat() -> Void {
  // [0 SET_REGISTER_TO_CP 0, 1 ACCEPT]: 零宽程序多轮驱动 (SetInputIndex 推进起点,
  // 原版 FindMatches 消费面 cc:486-539 → EXP-4, 此处逐轮手工推进核验 FindNextMatch
  // cc:845 去重复位 + cc:864 主线程重挂)
  let bytecode = [Instruction::set_register_to_cp(0), Instruction::accept()];
  let mut nfa = make_nfa(&bytecode, &[0x0061, 0x0062], false);
  for round in 0..3 {
    nfa.find_next_match().unwrap();
    assert!(nfa.found_match(), "第 {round} 轮零宽必中");
    assert_eq!(
      nfa.best_match_thread.as_ref().unwrap().register_array[0],
      round,
      "置存 = 本轮起始 input_index"
    );
    assert_eq!(nfa.clock, 2 * (round as u64 + 1), "每轮两台指令 (cc:891)");
    assert!(nfa.active_threads.is_empty() && nfa.blocked_threads.is_empty());
    if round < 2 {
      nfa.set_input_index(round + 1);
    }
  }
  // 去重表跨轮复位核验: 末轮后表项 = 标记态; 再起一轮应先复位 (cc:845)
  assert_ne!(nfa.pc_last_input_index[0], Default::default());
  OK
}
