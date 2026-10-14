//! 实验性线性时间正则引擎解释器 (v8/src/regexp/experimental/experimental-interpreter.{h,cc})。
//!
//! 架构中文摘要 (原版 cc:252-346 头注释): 广度优先 NFA 模拟, 无回溯, 输入串线性时间。
//! 线程 (InterpreterThread, 非 OS 线程) 相对输入位置同步推进, 共享当前 `input_index`;
//! ACCEPT 优先级语义: 高优先级线程先行, 低优先级在找到匹配后丢弃; 环视 (lookaround)
//! 分两轨: 仅无捕获后视 (captureless lookbehind) 走逐位置 `lookbehind_table_` 优化轨,
//! 否则预构建 `lookaround_table_` (cc:1547-1551)。捕获组按时钟 (clock) 过滤
//! (FilterGroups, cc:98-250)。
//!
//! ── EXP-3a 骨架票落地面 (本文件) ────────────────────────────────────────────
//! - 常量全集 (cc:21-23, cc:733)
//! - 断言谓词 [`satisfies_assertion`] (cc:25-61) 与字符谓词表复用
//! - [`FilterGroups`] 捕获过滤状态机 (cc:98-250) 全量
//! - [`InterpreterThread`] + [`ConsumedCharacter`] (cc:544-589)、
//!   [`LastInputIndex`] (cc:1492-1502)、[`LookaroundInfo`] (cc:1532-1536)
//! - [`NfaInterpreter`] 构造器字节码扫描面 (cc:348-479 的纯逻辑段)、
//!   [`NfaInterpreter::flush_blocked_threads`] (cc:1132-1172)、
//!   pc 去重对 (cc:1440-1465)、[`NfaInterpreter::found_match`]/
//!   [`NfaInterpreter::set_input_index`]/[`NfaInterpreter::approximate_total_memory_usage`]
//! - 主循环 [`NfaInterpreter::run_active_thread`] (cc:886-1115) 主体 EXP-3b/EXP-3c
//!   分批落地, 见下
//!
//! ── EXP-3b 已落地 (线程调度+状态机转移, 本席位) ────────────────────────────
//! - 主循环 preamble: 边界取指 (cc:888-889 SBXCHECK)、++clock (cc:891)、去重短路
//!   (cc:899-902 IsPcProcessed 命中即销毁) + MarkPcProcessed (cc:903)
//! - 10 实现臂: FORK cc:921-984 (含 flag 开五数组拷贝段 cc:932-973; 内联子调用
//!   CheckMemoryConsumption cc:977-980 本票仅落接线探针, 本体 EXP-3d)、
//!   JMP cc:985-987、ACCEPT cc:988-998 (set_best_match 落定位+低优先级清除)、
//!   SET_QUANTIFIER_TO_CLOCK cc:999-1002、CLEAR_REGISTER cc:1004-1010、
//!   SET_REGISTER_TO_CP cc:1011-1019 (flag 开并记捕获时钟 cc:1015-1017)、
//!   BEGIN_LOOP cc:1025-1029、END_LOOP cc:1030-1040 (空串量词迭代销毁)、
//!   START_LOOKAROUND cc:1041-1043 (主轨透传)、END_LOOKAROUND cc:1045-1055;
//!   FILTER_* 防御臂 cc:1020-1024 (原版 UNREACHABLE, 本票按 FilterGroups::run
//!   先例收敛为 BadBytecode 加固)
//! - 调度外围: [`NfaInterpreter::run_active_threads`] (RunActiveThreads cc:1120-1127,
//!   尾弹最高优先)、[`NfaInterpreter::run_active_threads_to_end`]
//!   (RunActiveThreadsToEnd cc:697-752, kTicks=64→HandleInterrupts 接线 cc:733-737
//!   仅探针, 本体 EXP-4)、[`NfaInterpreter::find_next_match`] (FindNextMatch cc:831-878,
//!   含仅无捕获后视轨环视线程挂起 cc:866-870); 多匹配驱动
//!   FindMatches 成员 cc:486-539 → EXP-4
//!
//! ── EXP-3c 已落地 (字符消费+谓词绑定, 本席位) ──────────────────────────────
//! - CONSUME_RANGE/RANGE_COUNT 挂起臂 (cc:908-912 共臂体 `blocked_threads_.Add(t,
//!   zone_); return success`): 经消费挂起辅助 `block_thread` (本 impl 内私有单点)
//!   入 blocked (高→低优先级序), 与唤醒侧 [`NfaInterpreter::flush_blocked_threads`]
//!   (EXP-3a 已落) 构成阻塞/唤醒全链; one/two-byte 双形态 = 原版 `Character` 模板
//!   于入口面 (cc:1583-1603, EXP-4) 以 uint8_t/uc16 双实例化, 本席位按既有惯例以
//!   u16 码元切片单点注入承接 (见参数注入台账), 臂体逐码元比较语义与模板版全等
//! - ASSERTION 求值绑定臂 (cc:913-920): 绑定 EXP-3a 已落纯函数
//!   [`satisfies_assertion`] (cc:25-61 全 7 断言臂; 原版即自由函数, 无 Isolate/
//!   正则字节码表耦合 → 本批无推迟登记项); 谓词假 → 线程销毁 (=drop, cc:916),
//!   真 → `++pc` 续跑 (cc:919); 实参 `input_`/`input_index_` 绑定 cc:914-915
//!
//! ── Execute 主循环 (RunActiveThread) 剩余推迟批段 ──────────────────────────
//! - EXP-3d 环视表 + 捕获过滤 + 内存治理: WRITE_LOOKAROUND_TABLE cc:1056-1071、
//!   READ_LOOKAROUND_TABLE cc:1072-1112、FillLookaroundTable cc:591-633、
//!   FillLookaroundCaptures cc:636-695、GetFilteredRegisters 接线 cc:1380-1411
//!   (FilterGroups 本体已落地)、CheckMemoryConsumption 本体 cc:1183-1208
//!   (FORK 接线与探针 [`NfaInterpreter::check_memory_consumption`] 本票已落,
//!   flag 开 FORK 由探针显式失败; 阈值消费 v8-flags defs.rs:5975 默认 1024MB,
//!   届时建议随 FindMatches 面切 &V8Flags 注入 — compiler.rs EXP-2 同款先例)、
//!   RecyclingZoneAllocator 回收链 cc:1243-1335/1521-1528 (Rust 暂以 Vec 所有权承接,
//!   DestroyThread cc:1413-1425 即 drop)
//! - EXP-4 入口面 (experimental.{h,cc} 席位, 禁碰面): FindMatches 入口 cc:1583-1603 +
//!   NfaInterpreter::FindMatches cc:486-539、HandleInterrupts cc:757-816 本体 (Isolate 栈
//!   守护/GC; kTicks 接线探针 [`NfaInterpreter::handle_interrupts`] 本票已落)、
//!   ToInstructionVector cc:63-71 (裸字节解码)、ToCharacterVector cc:73-96
//!   (String FlatContent)、h:20-33 ExperimentalRegExpInterpreter 声明面
//!
//! ── 参数注入适配 (前席 "L3 前占位" 规则, 同 regexp_utils.rs) ────────────────
//! Isolate/Tagged/String/CallOrigin/zone/RecyclingZoneAllocator 依赖面以切片与标量注入:
//! 字节码 `&[Instruction]` (bytecode.rs 已落地 IR, 裸编码解码推迟 EXP-4)、输入
//! `&[u16]` (one-byte 输入由入口方展宽, 单点适配登记)、flag
//! `experimental_regexp_engine_capture_group_opt` (v8-flags defs.rs:5968) 以布尔参数注入。
//! 本席禁触 Cargo.toml: v8-strings 等价谓词 (unicode.rs:543 / char_predicates.rs:194)
//! 不可达, 谓词经冻结依赖 v8-regexp-syntax::tables_generated 表重建 (语义等价推导见
//! [`is_line_terminator`]/[`is_regexp_word`])。
//!
//! 错误集中面: 本模块局部 [`InterpreterError`]; 并入 src/error.rs 推迟登记
//! (本席禁触 error.rs, 原版 cc:21-23/888-889 哨兵与 SBXCHECK 面在此承接)。

use std::result;

use thiserror::Error;
use v8_regexp_syntax::{
  AssertionType, LookaroundType,
  tables_generated::{LINE_TERMINATOR_RANGES, WORD_RANGES},
};

use super::bytecode::{Instruction, Opcode, Payload, Uc16Range};

// ── 错误面 ──────────────────────────────────────────────────────────────────

/// 解释器错误 (原版无 Rust 对应物: 原版以错误码 int (RegExp::InternalResult,
/// regexp.h:111-114) 与 UNREACHABLE/DCHECK 崩溃通道表达; Rust 分解为
/// [`NotImplemented`](InterpreterError::NotImplemented) 推迟批段与
/// [`BadPc`](InterpreterError::BadPc)/[`BadBytecode`](InterpreterError::BadBytecode)
/// 恶意/畸形字节码防御面 — 原版 SBXCHECK cc:888-889/1005-1006/1012-1013 的等价物)
#[derive(Error, Debug, PartialEq, Eq)]
pub enum InterpreterError {
  /// 推迟批段项: 主体按台账在 EXP-3d/4 席位落地 (EXP-3c 批段 2 已落地;
  /// 禁 todo!()/假绿的显式失败通道, 单点构造见 [`deferred_arm`])
  #[error("未实现: {item} (原版 experimental-interpreter.cc:{orig}; 计划批段 {batch})")]
  NotImplemented {
    /// 推迟项 (Rust 符号 + 原版函数名, 台账可 grep)
    item: &'static str,
    /// 原版行号锚
    orig: &'static str,
    /// 后续批段 (EXP-3c/3d/4, 台账常量 [`EXP_BATCH_3C`]/[`EXP_BATCH_3D`]/[`EXP_BATCH_4`])
    batch: &'static str,
  },
  /// pc 越界 (v8: cc:886-889 RunActiveThread 入口 SBXCHECK_GE/SBXCHECK_LT 等价)
  #[error("畸形字节码: pc {pc} 不在指令窗 [0, {len}) (原版 {orig})")]
  BadPc {
    pc: i32,
    len: usize,
    orig: &'static str,
  },
  /// payload 配对/取值域违规 (原版以 union 按 opcode 约定直读不设防, Rust tagged
  /// Payload 显式拒绝; 编译器 EXP-2 保证合法程序永不触达)
  #[error("畸形字节码: {item} (原版 {orig})")]
  BadBytecode {
    item: &'static str,
    orig: &'static str,
  },
}

/// 模块内结果别名 (src/error.rs 并入前局部定义; 禁改 Cargo.toml/error.rs 约束下的落点)
pub type Result<T> = result::Result<T, InterpreterError>;

// ── 常量全集 (v8: cc:21-23 匿名 namespace constexpr + cc:733 函数内 static constexpr) ──

/// 未定义寄存器值 (v8: cc:21 `constexpr int kUndefinedRegisterValue = -1`)
pub const K_UNDEFINED_REGISTER_VALUE: i32 = -1;

/// 未定义环视命中索引值 (v8: cc:22 `constexpr int kUndefinedMatchIndexValue = -1`)
pub const K_UNDEFINED_MATCH_INDEX_VALUE: i32 = -1;

/// 未定义时钟值 (v8: cc:23 `constexpr uint64_t kUndefinedClockValue = -1` —
/// uint64 窄化回绕为全 1, 金标准 = u64::MAX)
pub const K_UNDEFINED_CLOCK_VALUE: u64 = u64::MAX;

/// 中断处理节拍 (v8: cc:733 `static constexpr int kTicksBetweenInterruptHandling = 64`,
/// 定义于 RunActiveThreadsToEnd 体内; 接线本票已落, 见 run_active_threads_to_end)
pub const K_TICKS_BETWEEN_INTERRUPT_HANDLING: i32 = 64;

/// 兆字节 (v8: include/v8-internal.h:41-42 `constexpr int KB = 1024; MB = KB * 1024`;
/// cc:1194-1195 CheckMemoryConsumption 阈值换算消费面, EXP-3d 落地引入)
pub const K_MB: u64 = 1024 * 1024;

// ── 批段台账字符串 ([`InterpreterError::NotImplemented`] 的 batch 消费面) ──────

/// EXP-3c 批段常量 (字符消费+谓词绑定臂; EXP-3c 席位已落地 — 批段 2 三臂迁出
/// 推迟通道后本常量保留作台账对账与 tests 面路由断言消费)
pub const EXP_BATCH_3C: &str = "EXP-3c 字符消费+谓词绑定";

/// EXP-3d 推迟批段 (环视表+捕获过滤+内存治理)
pub const EXP_BATCH_3D: &str = "EXP-3d 环视表+捕获过滤+内存治理";

/// EXP-4 推迟批段 (入口面)
pub const EXP_BATCH_4: &str = "EXP-4 入口面";

// ── 字符谓词 (v8: cc:42-55 消费的 unibrow::IsLineTerminator / IsRegExpWord) ────

/// 区间表命中判定 (半开闭区间逐对比较; 表全集来自 v8-regexp-syntax 金标准表)
#[inline]
fn in_ranges(ranges: &[(u32, u32)], c: u32) -> bool {
  ranges.iter().any(|&(lo, hi)| c >= lo && c <= hi)
}

/// 行终止符 (v8: src/strings/unicode.h:278-280 `unibrow::IsLineTerminator` —
/// LF(0x0A)/CR(0x0D)/LS(0x2028)/PS(0x2029) 四值; 复用冻结依赖
/// v8-regexp-syntax tables_generated.rs:25 LINE_TERMINATOR_RANGES
/// `[(10,10),(13,13),(8232,8233)]` 逐值相等)
#[inline]
#[must_use]
pub fn is_line_terminator(c: u16) -> bool {
  in_ranges(&LINE_TERMINATOR_RANGES, u32::from(c))
}

/// 正则 \\w 字符 (v8: src/strings/char-predicates-inl.h:82-84 `IsRegExpWord` =
/// `IsAlphaNumeric(c) || c == '_'`; IsAlphaNumeric cc:30-32 = AsciiAlphaToLower
/// 落 [a,z] 或十进制数字。推导: 真值集恰为 0-9/A-Z/a-z/_ , 与冻结依赖
/// tables_generated.rs:19 WORD_RANGES `[(48,57),(65,90),(95,95),(97,122)]`
/// 在全 u16 域逐值相等 (c ≥ 0x7B 且非 '_' 时两式同假; ASCII 大小写经
/// `c | 0x20` 归一与 A-Z 区间直判等价))
#[inline]
#[must_use]
pub fn is_regexp_word(c: u16) -> bool {
  in_ranges(&WORD_RANGES, u32::from(c))
}

// ── 断言谓词 (v8: cc:25-61 `SatisfiesAssertion<Character>`) ────────────────────

/// 零宽断言求值 (v8: cc:25-61 `SatisfiesAssertion`)。
///
/// `Character` 模板参数适配: 输入注入为 UTF-16 码元切片, one-byte 串由入口方
/// (EXP-4) 展宽为 u16 — 谓词逐位比较语义与模板版全等。
/// `position` 承接原版 `int position` (DCHECK_LE/GE cc:28-29 → debug_assert)。
#[must_use]
pub fn satisfies_assertion(assertion: AssertionType, context: &[u16], position: usize) -> bool {
  // 原版 cc:28-29 DCHECK_LE(position, context.length()) / DCHECK_GE(position, 0)
  debug_assert!(position <= context.len());
  match assertion {
    // cc:32-33
    AssertionType::StartOfInput => position == 0,
    // cc:34-35
    AssertionType::EndOfInput => position == context.len(),
    // cc:36-39 UNREACHABLE(): \Z 需 /u, 实验性引擎拒绝 /u, 该断言永不到达解释器
    // (同 bytecode.rs dump 面处置, 100% 不可达才用 unreachable)
    AssertionType::EndOfBuffer => {
      unreachable!("v8 cc:36-39 UNREACHABLE(): END_OF_BUFFER 依赖 /u, 实验性引擎不产出")
    }
    // cc:40-42
    AssertionType::StartOfLine => position == 0 || is_line_terminator(context[position - 1]),
    // cc:43-45
    AssertionType::EndOfLine => position == context.len() || is_line_terminator(context[position]),
    // cc:46-56
    AssertionType::Boundary => {
      if context.is_empty() {
        false
      } else if position == 0 {
        is_regexp_word(context[position])
      } else if position == context.len() {
        is_regexp_word(context[position - 1])
      } else {
        is_regexp_word(context[position - 1]) != is_regexp_word(context[position])
      }
    }
    // cc:57-58 递归取反
    AssertionType::NonBoundary => !satisfies_assertion(AssertionType::Boundary, context, position),
  }
}

// ── 捕获过滤状态机 (v8: cc:98-250 `class FilterGroups`) ────────────────────────

/// 量词/分组捕获按时钟过滤 (v8: cc:98-250 `class FilterGroups`)。
///
/// 算法中文摘要 (原版 cc:107-117 注释): 解释器维护 clock (已执行指令计数), 到达量词
/// 或捕获组时记录当时 clock; 匹配完成后, 比较各捕获组的 clock 与其父量词/组的
/// clock, 只保留"父节点最后一轮迭代之后才定义"的组 (非最后一轮迭代走过的组丢弃,
/// 丢弃父节点即整棵子树不访问)。字节码结构由 `FilterGroupsCompileVisitor`
/// (experimental-compiler.cc, EXP-2 席位) 生成。
///
/// 适配登记: 原版经 Zone* 分配 ZoneStack 并返回 base::Vector 引用; Rust 用本地
/// Vec 栈 + `filtered_registers` 原地写 (调用方持 Vec 所有权, 无别名返回)。
struct FilterGroups<'a> {
  /// v8: cc:239 `int pc_` (usize 化, 越界经 get 显式 Err)
  pc: usize,
  /// v8: cc:241-243 `uint64_t max_clock_` — 最后遇到的量词/组时钟; clock 小于它的组丢弃
  max_clock: u64,
  /// v8: cc:246 `ZoneStack<int> pc_stack_`
  pc_stack: Vec<usize>,
  /// v8: cc:247 `ZoneStack<uint64_t> max_clock_stack_`
  max_clock_stack: Vec<u64>,
  /// v8: cc:249 `base::Vector<const Instruction> bytecode_`
  bytecode: &'a [Instruction],
}

impl<'a> FilterGroups<'a> {
  /// v8: cc:125-130 私有构造器 (max_clock_ 初值 0)
  fn new(pc: usize, bytecode: &'a [Instruction]) -> Self {
    Self {
      pc,
      max_clock: 0,
      pc_stack: Vec::new(),
      max_clock_stack: Vec::new(),
      bytecode,
    }
  }

  /// v8: cc:134-141 `Up()` — 回退父节点, 恢复 pc 与 max_clock; 栈空即过滤完成
  fn up(&mut self) {
    if !self.pc_stack.is_empty() {
      self.pc = self.pc_stack.pop().unwrap_or(0);
      self.max_clock = self.max_clock_stack.pop().unwrap_or(0);
    }
  }

  /// v8: cc:144-150 `IncrementPC()` — 节点末回退父节点, 否则 pc+1
  fn increment_pc(&mut self) {
    if self.is_at_node_end() {
      self.up();
    } else {
      self.pc += 1;
    }
  }

  /// v8: cc:152-155 `IsAtNodeEnd()` — 下一指令不再是 FILTER_CHILD 即为本节点末
  fn is_at_node_end(&self) -> bool {
    // 调用前置 (run 循环已按 pc 取指成功): pc < len ⇒ pc+1 ≤ len, 索引判定安全
    self.pc + 1 == self.bytecode.len() || self.bytecode[self.pc + 1].opcode != Opcode::FilterChild
  }

  /// v8: cc:157-237 `Run(...)` — 显式栈迭代遍历 FILTER_* 树; 原版对 FILTER_* 之外的
  /// 指令 UNREACHABLE (cc:231-232), Rust 加固为 [`BadBytecode`](InterpreterError::BadBytecode)
  fn run(
    &mut self,
    registers: &[i32],
    quantifiers_clocks: &[u64],
    capture_clocks: &[u64],
    lookaround_clocks: Option<&[u64]>,
    filtered_registers: &mut [i32],
  ) -> Result<()> {
    self.pc_stack.push(self.pc);
    self.max_clock_stack.push(self.max_clock);

    while !self.pc_stack.is_empty() {
      let inst = *self.bytecode.get(self.pc).ok_or(InterpreterError::BadPc {
        pc: self.pc as i32,
        len: self.bytecode.len(),
        orig: "cc:166 bytecode_[pc_] 直读 (编译器不变量)",
      })?;
      match (inst.opcode, inst.payload) {
        // cc:168-178: 仅在本节点尚未到末时才压回跳帧 (下一指令仍是兄弟 FILTER_CHILD)
        (Opcode::FilterChild, Payload::Pc(child_pc)) => {
          if !self.is_at_node_end() {
            self.pc_stack.push(self.pc + 1);
            self.max_clock_stack.push(self.max_clock);
          }
          // 进入子节点 (原版 int payload.pc 直赋; 负值即畸形程序 → Err)
          self.pc = usize::try_from(child_pc).map_err(|_| InterpreterError::BadBytecode {
            item: "FILTER_CHILD 目标 pc 为负",
            orig: "cc:177 pc_ = instr.payload.pc",
          })?;
        }

        // cc:180-197: 捕获组按时钟决定保留/丢弃; 丢弃即整棵子树不访问 (Up)
        (Opcode::FilterGroup, Payload::GroupId(group_id)) => {
          // 原版 cc:184 `int register_id = 2 * group_id;`
          let register_id = usize::try_from(i64::from(group_id) * 2).map_err(|_| {
            InterpreterError::BadBytecode {
              item: "FILTER_GROUP register_id 越出非负域",
              orig: "cc:184-185 register_id = 2 * group_id",
            }
          })?;
          let clock = *capture_clocks
            .get(register_id)
            .ok_or(InterpreterError::BadBytecode {
              item: "FILTER_GROUP 捕获时钟越界",
              orig: "cc:186 capture_clocks_[register_id] 直读",
            })?;
          if clock >= self.max_clock && clock != K_UNDEFINED_CLOCK_VALUE {
            // cc:187-188 拷贝组起止寄存器
            let (Some(&start), Some(&end)) =
              (registers.get(register_id), registers.get(register_id + 1))
            else {
              return Err(InterpreterError::BadBytecode {
                item: "FILTER_GROUP 寄存器越界",
                orig: "cc:187-188 filtered_registers_[register_id(+1)] = registers_[...]",
              });
            };
            if register_id + 1 >= filtered_registers.len() {
              return Err(InterpreterError::BadBytecode {
                item: "FILTER_GROUP 过滤寄存器越界",
                orig: "cc:187-188 filtered_registers_[register_id(+1)] = registers_[...]",
              });
            }
            filtered_registers[register_id] = start;
            filtered_registers[register_id + 1] = end;
            self.increment_pc();
          } else {
            // cc:190-195: 组应丢弃 → 其子树一并丢弃
            self.up();
          }
        }

        // cc:199-213: 量词按时钟决定是否深入; 保留则刷新 max_clock
        (Opcode::FilterQuantifier, Payload::QuantifierId(quantifier_id)) => {
          let index =
            usize::try_from(quantifier_id).map_err(|_| InterpreterError::BadBytecode {
              item: "FILTER_QUANTIFIER 索引为负",
              orig: "cc:200 quantifier_id",
            })?;
          let clock = *quantifiers_clocks
            .get(index)
            .ok_or(InterpreterError::BadBytecode {
              item: "FILTER_QUANTIFIER 量词时钟越界",
              orig: "cc:203 quantifiers_clocks_[quantifier_id] 直读",
            })?;
          if clock >= self.max_clock {
            self.max_clock = clock;
            self.increment_pc();
          } else {
            self.up();
          }
        }

        // cc:215-229: 环视过滤; lookaround_clocks 缺省 (仅无捕获后视轨) 恒放行
        (Opcode::FilterLookaround, Payload::LookaroundId(lookaround_id)) => {
          let keep = match lookaround_clocks {
            None => true,
            Some(clocks) => {
              let index =
                usize::try_from(lookaround_id).map_err(|_| InterpreterError::BadBytecode {
                  item: "FILTER_LOOKAROUND 索引为负",
                  orig: "cc:218 payload.lookaround_id",
                })?;
              *clocks.get(index).ok_or(InterpreterError::BadBytecode {
                item: "FILTER_LOOKAROUND 环视时钟越界",
                orig: "cc:218 lookaround_clocks->at(id) 直读",
              })?
                >= self.max_clock
            }
          };
          if keep {
            self.increment_pc();
          } else {
            self.up();
          }
        }

        // cc:231-232 UNREACHABLE(): FILTER_* 段只应含 FILTER_* 四指令
        _ => {
          return Err(InterpreterError::BadBytecode {
            item: "捕获过滤链混入非 FILTER_* 指令",
            orig: "cc:231-232 (原版 UNREACHABLE, Rust 加固为 Err)",
          });
        }
      }
    }
    Ok(())
  }
}

/// v8: cc:100-122 `FilterGroups::Filter` 静态入口 — 从 `pc` 起的 FILTER_* 段遍历
/// 字节码, 保留末轮迭代定义的捕获寄存器到 `filtered_registers` (原地写)。
pub fn filter_groups(
  pc: usize,
  registers: &[i32],
  quantifiers_clocks: &[u64],
  capture_clocks: &[u64],
  lookaround_clocks: Option<&[u64]>,
  filtered_registers: &mut [i32],
  bytecode: &[Instruction],
) -> Result<()> {
  FilterGroups::new(pc, bytecode).run(
    registers,
    quantifiers_clocks,
    capture_clocks,
    lookaround_clocks,
    filtered_registers,
  )
}

// ── 线程状态 (v8: cc:544-589 `class InterpreterThread`) ───────────────────────

/// 线程自上次进入量词后是否消费过字符 (v8: cc:546
/// `enum class ConsumedCharacter { DidConsume, DidNotConsume }`)。
/// 空串量词迭代不允许, 据此区分可退出量词的线程与不可退出的线程 (cc:584-588)。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConsumedCharacter {
  /// cc:546 DidConsume
  DidConsume,
  /// cc:546 DidNotConsume
  DidNotConsume,
}

/// 执行字节码的线程状态 (v8: cc:544-589 `class InterpreterThread`)。
///
/// 适配登记: 原版六个成员是指向 RecyclingZoneAllocator 数组的裸指针 (cc:565-582),
/// Rust 暂以 Vec 所有权承接 (销毁 = drop, cc:1413-1425 DestroyThread 的回收复用链
/// 推迟 EXP-3d, 台账 interpreter.rs::run_active_thread 批段表)。指针 → Vec 后
/// GetRegisterArray 等五个 getter (cc:1210-1241) 折叠为字段直读, 长度不变量
/// (原版 DCHECK_EQ) 由各构造器保证 (见 [`InterpreterThread::new_empty`])。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InterpreterThread {
  /// v8: cc:563-565 `int pc` — 下一条待执行指令下标
  pub pc: i32,
  /// v8: cc:566-569 `int* register_array_begin` — 恒长 `register_count_per_match`
  pub register_array: Vec<i32>,
  /// v8: cc:571-574 `int* lookaround_match_index_array_begin` —
  /// 原版 nullptr ⇔ Rust None (仅无捕获后视轨或 flag 关)
  pub lookaround_match_index_array: Option<Vec<i32>>,
  /// v8: cc:576-580 `uint64_t* quantifier_clock_array_begin`
  pub quantifier_clock_array: Option<Vec<u64>>,
  /// v8: cc:576-581 `uint64_t* captures_clock_array_begin` (原版字段名复数 captures,
  /// Rust 统一 capture_clock 与构造形参/工厂名对齐)
  pub capture_clock_array: Option<Vec<u64>>,
  /// v8: cc:576-582 `uint64_t* lookaround_clock_array_begin`
  pub lookaround_clock_array: Option<Vec<u64>>,
  /// v8: cc:584-588 `ConsumedCharacter consumed_since_last_quantifier`
  pub consumed_since_last_quantifier: ConsumedCharacter,
}

impl InterpreterThread {
  /// v8: cc:1337-1355 `NewEmptyThread` — 新建线程并按语义填充:
  /// 寄存器 = [`K_UNDEFINED_REGISTER_VALUE`], 命中索引 = [`K_UNDEFINED_MATCH_INDEX_VALUE`],
  /// 各时钟数组 = 0; flag 关时全部时钟/索引数组为 None (cc:1350-1354),
  /// 仅无捕获后视轨时环视两数组为 None (cc:1344-1348)。
  #[must_use]
  pub fn new_empty(
    pc: i32,
    register_count: usize,
    quantifier_count: usize,
    lookaround_count: usize,
    capture_group_opt: bool,
    only_captureless_lookbehinds: bool,
  ) -> Self {
    let clocks = if capture_group_opt {
      let lookaround = if only_captureless_lookbehinds {
        None
      } else {
        Some(vec![K_UNDEFINED_MATCH_INDEX_VALUE; lookaround_count])
      };
      (
        Some(vec![0u64; quantifier_count]),
        Some(vec![0u64; register_count]),
        if only_captureless_lookbehinds {
          None
        } else {
          Some(vec![0u64; lookaround_count])
        },
        lookaround,
      )
    } else {
      // cc:1350-1354: flag 关 → 除寄存器外全 nullptr
      (None, None, None, None)
    };
    let (
      quantifier_clock_array,
      capture_clock_array,
      lookaround_clock_array,
      lookaround_match_index_array,
    ) = clocks;
    Self {
      pc,
      register_array: vec![K_UNDEFINED_REGISTER_VALUE; register_count],
      lookaround_match_index_array,
      quantifier_clock_array,
      capture_clock_array,
      lookaround_clock_array,
      consumed_since_last_quantifier: ConsumedCharacter::DidConsume,
    }
  }

  /// v8: cc:1357-1378 `NewUninitializedThread` — FORK 专用 (唯一调用点 cc:922):
  /// 原版 allocate 后不填充, 立即被 std::copy 全量覆盖后才可读。Rust 无未初始化
  /// Vec, 以 0 占位 (语义等价: EXP-3b FORK 臂已落地 — 五数组先全量拷贝后消费
  /// ([`NfaInterpreter::run_active_thread`] Fork 臂), 拷贝未覆位的哨兵值永不观察)。
  #[must_use]
  pub fn new_uninitialized(
    pc: i32,
    register_count: usize,
    quantifier_count: usize,
    lookaround_count: usize,
    capture_group_opt: bool,
    only_captureless_lookbehinds: bool,
  ) -> Self {
    let clocks = if capture_group_opt {
      let lookaround = if only_captureless_lookbehinds {
        None
      } else {
        Some(vec![0i32; lookaround_count])
      };
      (
        Some(vec![0u64; quantifier_count]),
        Some(vec![0u64; register_count]),
        if only_captureless_lookbehinds {
          None
        } else {
          Some(vec![0u64; lookaround_count])
        },
        lookaround,
      )
    } else {
      (None, None, None, None)
    };
    let (
      quantifier_clock_array,
      capture_clock_array,
      lookaround_clock_array,
      lookaround_match_index_array,
    ) = clocks;
    Self {
      pc,
      register_array: vec![0; register_count],
      lookaround_match_index_array,
      quantifier_clock_array,
      capture_clock_array,
      lookaround_clock_array,
      consumed_since_last_quantifier: ConsumedCharacter::DidConsume,
    }
  }
}

// ── pc 去重表项 (v8: cc:1492-1502 `class LastInputIndex`) ─────────────────────

/// 每 pc 上次激活时的 input_index 快照, 按 consumed 态分槽
/// (v8: cc:1492-1502 `class LastInputIndex`)。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LastInputIndex {
  /// v8: cc:1500 `int having_consumed_character`
  pub having_consumed_character: i32,
  /// v8: cc:1501 `int not_having_consumed_character`
  pub not_having_consumed_character: i32,
}

impl LastInputIndex {
  /// v8: cc:1495-1498 双参构造
  #[must_use]
  pub const fn new(having_consumed_character: i32, not_having_consumed_character: i32) -> Self {
    Self {
      having_consumed_character,
      not_having_consumed_character,
    }
  }
}

impl Default for LastInputIndex {
  /// v8: cc:1493-1494 默认构造委托 (-1, -1)
  fn default() -> Self {
    Self::new(-1, -1)
  }
}

// ── 环视信息 (v8: cc:1532-1536 `struct LookaroundInfo`) ───────────────────────

/// 一个环视的 match/capture 自动机入口与方向 (v8: cc:1532-1536 `struct LookaroundInfo`)。
/// 数组序即优先级序 (编译器保证父环视自动机先于子环视出现, cc:1538-1544)。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LookaroundInfo {
  /// v8: cc:1533 `int match_pc`
  pub match_pc: i32,
  /// v8: cc:1534 `int capture_pc`
  pub capture_pc: i32,
  /// v8: cc:1535 `Lookaround::Type type`
  pub type_: LookaroundType,
}

impl Default for LookaroundInfo {
  /// v8: cc:422 占位填充值 `{-1, -1, LOOKBEHIND}`
  fn default() -> Self {
    Self {
      match_pc: -1,
      capture_pc: -1,
      type_: LookaroundType::Lookbehind,
    }
  }
}

// ── NFA 解释器 (v8: cc:252-1579 `template <class Character> class NfaInterpreter`) ──

/// 广度优先无回溯 NFA 解释器 (v8: cc:252-1579 `NfaInterpreter<Character>`)。
///
/// 已落地: EXP-3a 构造器字节码扫描面 (环视表/FILTER 段/量词计数)、线程容器与
/// 去重表、FlushBlockedThreads、断言/过滤纯函数; EXP-3b 主循环
/// [`run_active_thread`](NfaInterpreter::run_active_thread) 主体 (preamble + 10
/// 实现臂 + FILTER_* 加固防御臂) 与调度外围 [`run_active_threads`](NfaInterpreter::run_active_threads)/
/// [`run_active_threads_to_end`](NfaInterpreter::run_active_threads_to_end)/
/// [`find_next_match`](NfaInterpreter::find_next_match)。
///
/// 推迟字段登记 (参数注入替代, 见模块头):
/// - `isolate_`/`call_origin_`/`no_gc_`/`bytecode_object_`/`input_object_`
///   (cc:1467-1484) → EXP-4 入口面
/// - 五个 RecyclingZoneAllocator (cc:1519-1528) → Vec 所有权适配, 回收复用 EXP-3d
/// - `filter_groups_pc_`/`memory_consumption_per_thread_` 等消费面
///   (GetFilteredRegisters cc:1380-1411、CheckMemoryConsumption cc:1183-1208) → EXP-3d
pub struct NfaInterpreter<'a> {
  /// v8: cc:1474 `base::Vector<const Instruction> bytecode_` (裸字节解码面 cc:63-71 → EXP-4)
  pub bytecode: &'a [Instruction],
  /// v8: cc:1483 `base::Vector<const Character> input_` — 注入 u16 码元切片
  pub input: &'a [u16],
  /// v8: cc:1484 `int input_index_`
  pub input_index: i32,
  /// v8: cc:1486-1487 `uint64_t clock` — 全局指令执行计数 (原版字段名无下划线后缀)
  pub clock: u64,
  /// v8: cc:1476-1477 `const int register_count_per_match_`
  pub register_count_per_match: usize,
  /// v8: cc:1479-1480 `int quantifier_count_` (构造器扫描得出)
  pub quantifier_count: usize,
  /// v8: cc:1558-1561 `bool only_captureless_lookbehinds_`
  pub only_captureless_lookbehinds: bool,
  /// v8: cc:1563-1564 `bool reverse_` — 输入反向遍历态 (环视捕获回溯用)
  pub reverse: bool,
  /// v8: cc:1566-1568 `int current_lookaround_`
  pub current_lookaround: i32,
  /// v8: cc:1570-1574 `std::optional<int> filter_groups_pc_` — 首条 FILTER_* 指令 pc
  pub filter_groups_pc: Option<usize>,
  /// v8: cc:1538-1545 `ZoneList<LookaroundInfo> lookarounds_`
  pub lookarounds: Vec<LookaroundInfo>,
  /// v8: cc:1504-1508 `base::Vector<LastInputIndex> pc_last_input_index_` (长 = 指令数)
  pub pc_last_input_index: Vec<LastInputIndex>,
  /// v8: cc:1510-1512 `ZoneList<InterpreterThread> active_threads_` — 低→高优先级
  pub active_threads: Vec<InterpreterThread>,
  /// v8: cc:1514-1517 `ZoneList<InterpreterThread> blocked_threads_` — 高→低优先级
  pub blocked_threads: Vec<InterpreterThread>,
  /// v8: cc:1530 `std::optional<InterpreterThread> best_match_thread_`
  pub best_match_thread: Option<InterpreterThread>,
  /// v8: cc:1553-1556 `std::optional<ZoneList<bool>> lookbehind_table_`
  pub lookbehind_table: Option<Vec<bool>>,
  /// v8: cc:1547-1551 `std::optional<ZoneVector<ZoneVector<bool>>> lookaround_table_`
  pub lookaround_table: Option<Vec<Vec<bool>>>,
  /// v8: cc:1435 等 `v8_flags.experimental_regexp_engine_capture_group_opt` —
  /// 参数注入 (v8-flags defs.rs:5968 默认 false)
  pub capture_group_opt: bool,
  /// v8: cc:1576 `uint64_t memory_consumption_per_thread_` — flag 关原版未初始化,
  /// Rust 置 0 (消费面 ApproximateTotalMemoryUsage 仅 flag 开时被调, 语义不弱)
  pub memory_consumption_per_thread: u64,
  /// v8: cc:1194 全局 `v8_flags.experimental_regexp_engine_capture_group_opt_max_memory_usage`
  /// (MB 计, v8-flags defs.rs:5975 默认 1024) — 参数注入台账同款: 原版解释器只读
  /// 该 flag, Rust 以字段承接 (new() 置 flag 默认值, 测试面可直改字段覆写阈值)
  pub capture_group_opt_max_memory_usage_mb: u64,
}

impl<'a> NfaInterpreter<'a> {
  /// v8: cc:348-479 构造器之纯逻辑段 — 迭代字节码登记 FILTER_* 段首 pc、环视信息、
  /// 量词计数, 并按轨初始化环视真值表与去重表。
  ///
  /// 推迟登记 (硬耦合段): Isolate/句柄入参与 `ToInstructionVector`/
  /// `ToCharacterVector` (cc:348-360 初始化列表前段) → EXP-4;
  /// 五分配器 emplace (cc:453-454, 465-466) → Vec 适配, 回收链 EXP-3d。
  pub fn new(
    bytecode: &'a [Instruction],
    input: &'a [u16],
    input_index: i32,
    register_count_per_match: usize,
    capture_group_opt: bool,
  ) -> Result<Self> {
    // cc:381-383 DCHECK(!bytecode_.empty()) / DCHECK_GE/LE(input_index_, ...)
    debug_assert!(!bytecode.is_empty());
    debug_assert!((0..=(input.len() as i32)).contains(&input_index));

    let mut lookarounds: Vec<LookaroundInfo> = Vec::new();
    // cc:387-389 `std::optional<LookaroundInfo> lookaround; bool in_lookaround; int lookaround_index;`
    // Rust 将 START 期 index 折入 pending (原版三变量各自延续, WRITE 臂用 START 值)
    let mut pending: Option<(usize, LookaroundInfo)> = None;
    let mut in_lookaround = false;
    let mut filter_groups_pc: Option<usize> = None;
    let mut quantifier_count: usize = 0;
    let mut only_captureless_lookbehinds = true;

    // cc:390 `for (int i = 0; i < bytecode_.length() - 1; ++i)` — 原版不扫最后一条
    // 指令 (真实行为, 金标准测试向量固化此边界)
    for i in 0..bytecode.len().saturating_sub(1) {
      let inst = bytecode[i];

      // cc:393-407 START_LOOKAROUND: 记录 match_pc 与方向; LOOKAHEAD 即脱优化轨
      if inst.opcode == Opcode::StartLookaround {
        debug_assert!(pending.is_none(), "cc:394 DCHECK(!lookaround.has_value())");
        in_lookaround = true;
        let Payload::Lookaround(payload) = inst.payload else {
          return Err(InterpreterError::BadBytecode {
            item: "START_LOOKAROUND payload 配对",
            orig: "cc:399 payload.lookaround",
          });
        };
        // 原版 cc:399 u32 index 直存 int, 后续按此补位; 上界加固: 合法程序
        // lookaround 数 ≤ 指令数 (编译器不变量), 越界即畸形
        let index =
          usize::try_from(payload.index()).map_err(|_| InterpreterError::BadBytecode {
            item: "START_LOOKAROUND 索引越界",
            orig: "cc:399/421 lookaround_index 补位循环 (原版无检查, Rust 上界加固)",
          })?;
        if index > bytecode.len() {
          return Err(InterpreterError::BadBytecode {
            item: "START_LOOKAROUND 索引越界",
            orig: "cc:399/421 (编译器不变量: 环视数 ≤ 指令数)",
          });
        }
        pending = Some((
          index,
          LookaroundInfo {
            match_pc: i as i32,
            capture_pc: -1,
            type_: payload.type_(),
          },
        ));
        if payload.type_() == LookaroundType::Lookahead {
          only_captureless_lookbehinds = false;
        }
      }

      // cc:409-411 环视内的捕获寄存器 → 脱优化轨
      if inst.opcode == Opcode::SetRegisterToCp && in_lookaround {
        only_captureless_lookbehinds = false;
      }

      // cc:413-426 WRITE_LOOKAROUND_TABLE: 补 capture_pc=i+1, 按 index 落位 (先补占位)
      if inst.opcode == Opcode::WriteLookaroundTable {
        let (index, mut info) = pending.take().ok_or(InterpreterError::BadBytecode {
          item: "WRITE_LOOKAROUND_TABLE 无前置 START_LOOKAROUND",
          orig: "cc:414 DCHECK(lookaround.has_value())",
        })?;
        info.capture_pc = (i + 1) as i32;
        while lookarounds.len() <= index {
          lookarounds.push(LookaroundInfo::default());
        }
        lookarounds[index] = info;
      }

      // cc:428-430 END_LOOKAROUND
      if inst.opcode == Opcode::EndLookaround {
        in_lookaround = false;
      }

      // cc:432-437 首条 FILTER_* 即过滤段起点 (Instruction::is_filter 白名单不含
      // FILTER_LOOKAROUND — bytecode.rs h:290-296 陷阱面同款)
      if filter_groups_pc.is_none() && inst.is_filter() {
        debug_assert!(capture_group_opt, "cc:435 DCHECK(v8_flags....)");
        filter_groups_pc = Some(i);
      }

      // cc:439-443 量词计数取最大 id+1
      if inst.opcode == Opcode::SetQuantifierToClock {
        debug_assert!(capture_group_opt, "cc:440 DCHECK(v8_flags....)");
        let Payload::QuantifierId(id) = inst.payload else {
          return Err(InterpreterError::BadBytecode {
            item: "SET_QUANTIFIER_TO_CLOCK payload 配对",
            orig: "cc:441 payload.quantifier_id",
          });
        };
        // 原版 std::max(quantifier_count_, id + 1); id<0 时 id+1 ≤ 0 不更新 (保留原语义)
        let candidate = i64::from(id) + 1;
        if candidate > quantifier_count as i64 {
          quantifier_count = candidate as usize;
        }
      }
    }

    // cc:446-460 环视真值表与分配器初始化 (分配器 emplace 推迟 EXP-3d)
    let mut lookbehind_table = None;
    let mut lookaround_table = None;
    if only_captureless_lookbehinds {
      // cc:447-449 ZoneList 预置 lookarounds.len() 个 false
      lookbehind_table = Some(vec![false; lookarounds.len()]);
    } else {
      debug_assert!(capture_group_opt, "cc:451 DCHECK(v8_flags....)");
      // cc:457-459 逆序逐行 emplace_back(input_length + 1) — 逐行内容同构, 序无关
      lookaround_table = Some(vec![vec![false; input.len() + 1]; lookarounds.len()]);
    }

    // cc:462-475 预算单线程内存消耗 (CheckMemoryConsumption 消费面 → EXP-3d;
    // flag 关原版不初始化, Rust 置 0)
    let memory_consumption_per_thread = if capture_group_opt {
      let registers = (register_count_per_match * size_of::<i32>()) as u64;
      let quantifiers = (quantifier_count * size_of::<u64>()) as u64;
      let capture_clocks = (register_count_per_match * size_of::<u64>()) as u64;
      let lookaround_clocks = (lookarounds.len() * size_of::<u64>()) as u64;
      let lookaround_index = (lookarounds.len() * size_of::<i32>()) as u64;
      registers
        .saturating_add(quantifiers)
        .saturating_add(capture_clocks)
        .saturating_add(lookaround_clocks)
        .saturating_add(lookaround_index)
        .saturating_add(size_of::<InterpreterThread>() as u64)
    } else {
      0
    };

    Ok(Self {
      bytecode,
      input,
      input_index,
      // cc:361 clock(0)
      clock: 0,
      register_count_per_match,
      quantifier_count,
      only_captureless_lookbehinds,
      // cc:377-379 reverse_(false), current_lookaround_(-1)
      reverse: false,
      current_lookaround: -1,
      filter_groups_pc,
      lookarounds,
      // cc:362-364 + 477-478 按指令数分配并整表填 LastInputIndex()
      pc_last_input_index: vec![LastInputIndex::default(); bytecode.len()],
      // cc:365-366 空线程表 (原版带初始容量 0 的 ZoneList)
      active_threads: Vec::new(),
      blocked_threads: Vec::new(),
      best_match_thread: None,
      lookbehind_table,
      lookaround_table,
      capture_group_opt,
      memory_consumption_per_thread,
      // cc:1194 全局 flag 只读面 → 字段承接 (defs.rs:5975 默认 1024 MB 实测值)
      capture_group_opt_max_memory_usage_mb: 1024,
    })
  }

  /// 取指并做沙箱边界防御 (v8: cc:888-889 `SBXCHECK_GE(t.pc, 0)`/`SBXCHECK_LT(t.pc,
  /// bytecode_.size())` 等价; 原版负 pc/越界 pc 在 release 亦显式 CHECK 中止)。
  /// 成功返回 (usize 下标, 指令) — 下标供主循环去重表复用, 免二次窄化
  fn fetch(&self, pc: i32, orig: &'static str) -> Result<(usize, Instruction)> {
    usize::try_from(pc)
      .ok()
      .and_then(|index| self.bytecode.get(index).copied().map(|inst| (index, inst)))
      .ok_or(InterpreterError::BadPc {
        pc,
        len: self.bytecode.len(),
        orig,
      })
  }

  /// [`fetch`](NfaInterpreter::fetch) 的下标无关读体 (FlushBlockedThreads 只消费
  /// 指令值: 原版 cc:1143/1153 直读, 越界即畸形 → BadPc)
  fn fetch_at(&self, pc: i32, orig: &'static str) -> Result<Instruction> {
    Ok(self.fetch(pc, orig)?.1)
  }

  /// v8: cc:818-824 `SetInputIndex` — 更新后续 FindNextMatch 的起始输入索引
  pub fn set_input_index(&mut self, new_input_index: i32) {
    // cc:820-821 DCHECK_GE/LE
    debug_assert!((0..=(self.input.len() as i32)).contains(&new_input_index));
    self.input_index = new_input_index;
  }

  /// v8: cc:1174 `FoundMatch()` — 已 ACCEPT 且最优匹配线程在场
  #[must_use]
  pub fn found_match(&self) -> bool {
    self.best_match_thread.is_some()
  }

  /// v8: cc:1176-1179 `ApproximateTotalMemoryUsage()` —
  /// (blocked+active 线程数) × 单线程预算 (flag 关时预算为 0, 与原版仅 flag 开
  /// 调用面一致)
  #[must_use]
  pub fn approximate_total_memory_usage(&self) -> u64 {
    ((self.blocked_threads.len() + self.active_threads.len()) as u64)
      .saturating_mul(self.memory_consumption_per_thread)
  }

  /// v8: cc:1427-1451 `IsPcProcessed` — 同一 (pc, consumed 态) 在本 input_index
  /// 已执行过即冗余线程 (后到者优先级必低, 可弃)。
  /// 前置: pc 已经调用方边界校验 (原版 run 循环 SBXCHECK 后直读, cc:1444/1447)
  #[must_use]
  pub fn is_pc_processed(&self, pc: usize, consumed: ConsumedCharacter) -> bool {
    debug_assert!(pc < self.pc_last_input_index.len());
    let entry = self.pc_last_input_index[pc];
    match consumed {
      ConsumedCharacter::DidConsume => entry.having_consumed_character == self.input_index,
      ConsumedCharacter::DidNotConsume => entry.not_having_consumed_character == self.input_index,
    }
  }

  /// v8: cc:1453-1465 `MarkPcProcessed` — 登记 (pc, consumed 态) 于当前 input_index
  /// 已执行。前置同 [`is_pc_processed`](NfaInterpreter::is_pc_processed) (cc:1459/1462)
  pub fn mark_pc_processed(&mut self, pc: usize, consumed: ConsumedCharacter) {
    debug_assert!(pc < self.pc_last_input_index.len());
    let entry = &mut self.pc_last_input_index[pc];
    match consumed {
      ConsumedCharacter::DidConsume => entry.having_consumed_character = self.input_index,
      ConsumedCharacter::DidNotConsume => entry.not_having_consumed_character = self.input_index,
    }
  }

  /// v8: cc:1129-1172 `FlushBlockedThreads` — 以 `input_char` 唤醒全部阻塞线程:
  /// blocked 为高→低优先级, 逆序激活使 active 保持低→高 (cc:1133-1135);
  /// RANGE_COUNT 头则连查 `num_ranges` 个 CONSUME_RANGE (原版 DCHECK_EQ 逐指令,
  /// cc:1154), 命中即 pc 跳区间末并标记已消费; 未命中线程销毁 (= Vec drop,
  /// 回收复用链推迟 EXP-3d)。
  pub fn flush_blocked_threads(&mut self, input_char: u16) -> Result<()> {
    for i in (0..self.blocked_threads.len()).rev() {
      let mut t = self.blocked_threads.remove(i);
      // cc:1138-1141 `int32_t ranges = 1; bool has_matched = false;`
      let mut ranges: usize = 1;
      let mut has_matched = false;

      let head = self.fetch_at(
        t.pc,
        "cc:1143 bytecode_[t.pc] (阻塞 pc 经 run 循环 SBXCHECK)",
      )?;
      if head.opcode == Opcode::RangeCount {
        let Payload::NumRanges(num) = head.payload else {
          return Err(InterpreterError::BadBytecode {
            item: "RANGE_COUNT payload 配对",
            orig: "cc:1144 payload.num_ranges",
          });
        };
        // 原版 int32 直用 (负值使内层循环空转 → 线程销毁); Rust 显式拒绝负值,
        // 合法程序 (编译器保证 num_ranges ≥ 1) 语义不弱
        ranges = usize::try_from(num).map_err(|_| InterpreterError::BadBytecode {
          item: "RANGE_COUNT num_ranges 为负",
          orig: "cc:1144 num_ranges",
        })?;
        // cc:1145 ++t.pc
        t.pc += 1;
      }

      // cc:1147-1149 `int next_pc = t.pc + ranges;` — 区间末 (跳离全部范围指令)
      let next_pc = t.pc.saturating_add(ranges as i32);
      // cc:1151-1164 逐范围比对 [min, max] 闭区间
      let mut pc = t.pc;
      while pc < next_pc {
        let inst = self.fetch_at(pc, "cc:1153 bytecode_[pc] 范围区扫描")?;
        debug_assert_eq!(
          inst.opcode,
          Opcode::ConsumeRange,
          "cc:1154 DCHECK_EQ(inst.opcode, Instruction::CONSUME_RANGE)"
        );
        let Payload::ConsumeRange(range) = inst.payload else {
          return Err(InterpreterError::BadBytecode {
            item: "CONSUME_RANGE payload 配对",
            orig: "cc:1155 payload.consume_range",
          });
        };
        let Uc16Range { min, max } = range;
        if input_char >= min && input_char <= max {
          t.pc = next_pc;
          t.consumed_since_last_quantifier = ConsumedCharacter::DidConsume;
          has_matched = true;
          break;
        }
        pc += 1;
      }

      // cc:1165-1169 命中入 active, 未命中 DestroyThread (drop)
      if has_matched {
        self.active_threads.push(t);
      }
    }
    // cc:1171 `blocked_threads_.Rewind(0);` — remove 逆序摘取后已自然清空
    debug_assert!(self.blocked_threads.is_empty());
    Ok(())
  }

  /// v8: cc:1340 `NewEmptyThread` 解释器绑定面 — 以自身计数装配新空线程
  /// (FindNextMatch cc:864 主线程 / cc:868 环视线程消费)
  fn new_empty_thread(&self, pc: i32) -> InterpreterThread {
    InterpreterThread::new_empty(
      pc,
      self.register_count_per_match,
      self.quantifier_count,
      self.lookarounds.len(),
      self.capture_group_opt,
      self.only_captureless_lookbehinds,
    )
  }

  /// v8: cc:1360 `NewUninitializedThread` 解释器绑定面 — FORK 臂专用 (cc:922)
  fn new_uninitialized_thread(&self, pc: i32) -> InterpreterThread {
    InterpreterThread::new_uninitialized(
      pc,
      self.register_count_per_match,
      self.quantifier_count,
      self.lookarounds.len(),
      self.capture_group_opt,
      self.only_captureless_lookbehinds,
    )
  }

  /// ACCEPT/END_LOOKAROUND 共用落定位 (v8: cc:988-998 与 cc:1045-1055 体全等):
  /// 旧 best 销毁 (= Rust Option 覆写即 drop, 原版 DCHECK cc 注释保证新旧线程不同源)
  /// → best_match_thread_ = 本线程 → 清除全部剩余 active_threads_ (低优先级线程,
  /// 只能产出更劣匹配)。blocked 不动: 其成员皆先于本线程弹出过 active, 优先级必
  /// 高于本线程, 唤醒后若 ACCEPT 依规覆盖 best (原版注释 cc:287-300 优先级论证)。
  fn set_best_match(&mut self, thread: InterpreterThread) {
    self.best_match_thread = Some(thread);
    self.active_threads.clear();
  }

  /// 字符消费挂起辅助 (v8: cc:908-912 CONSUME_RANGE/RANGE_COUNT 共臂体
  /// `blocked_threads_.Add(t, zone_); return kInternalRegExpSuccess` 的单点化):
  /// blocked 恒高→低优先级序 (active 尾弹最高优先线程先挂起先入队), 唤醒侧
  /// [`flush_blocked_threads`](NfaInterpreter::flush_blocked_threads) 逆序摘取恢复
  /// active 低→高序 (cc:1133-1136); `zone_` 参由 Vec 所有权适配省略 (回收链 EXP-3d)。
  fn block_thread(&mut self, thread: InterpreterThread) {
    self.blocked_threads.push(thread);
  }

  /// v8: cc:880-1115 `RunActiveThread` — 主循环 (本仓类比原版旧引擎 `Bytecode::Execute`
  /// 的线程状态机转移主循环; EXP-3b 批段 1 + EXP-3c 批段 2 席位落地)。
  ///
  /// 形参按值收线程 (原版 cc:886 按值 `InterpreterThread t`): 循环内突变 pc/consumed,
  /// 终局三通道 — 销毁 (return/drop, cc:900/916/1036)、入 best (ACCEPT/END_LOOKAROUND
  /// move)、入 blocked (CONSUME_RANGE/RANGE_COUNT, EXP-3c 已落臂 cc:908-912 经
  /// `block_thread` 挂起后返回)。
  ///
  /// 原版 cc:905 取指在去重检查之后; Rust 在同址单次读同时承接 SBXCHECK 边界与
  /// 取指 (无副作用, 顺序对调语义等价, 登记适配)。
  pub fn run_active_thread(&mut self, mut thread: InterpreterThread) -> Result<()> {
    loop {
      // cc:888-889 SBXCHECK_GE/LT: pc 边界 + 取指
      let (pc, inst) = self.fetch(thread.pc, "cc:888-889 SBXCHECK")?;

      // cc:891 ++clock (全局指令计数; u64 溢出由原版注释 cc:893-896 论证不可达)
      self.clock += 1;
      // cc:897 DCHECK_GT(clock, 0)
      debug_assert!(self.clock > 0);

      // cc:899-902 去重短路: 同 (pc, consumed 态) 在当前输入位已执行过 → 冗余线程
      // 销毁 (DestroyThread = Rust drop, 回收链 EXP-3d; 优先级论证见 cc:1427-1439 注释)
      if self.is_pc_processed(pc, thread.consumed_since_last_quantifier) {
        return Ok(());
      }
      // cc:903 MarkPcProcessed
      self.mark_pc_processed(pc, thread.consumed_since_last_quantifier);

      match (inst.opcode, inst.payload) {
        // ── EXP-3c 实现臂 (字符消费+谓词绑定, 批段 2/3) ──
        (Opcode::ConsumeRange, Payload::ConsumeRange(_)) => {
          // cc:908-912 (与 RANGE_COUNT 共臂体): 挂起入 blocked, 本轮驱动收敛;
          // pc 不自增 (唤醒侧 flush_blocked_threads cc:1143 在同一输入位比对范围)
          self.block_thread(thread);
          return Ok(());
        }
        (Opcode::RangeCount, Payload::NumRanges(_)) => {
          // cc:908-912 共臂体: RANGE_COUNT 与 CONSUME_RANGE 同址挂起, 唤醒侧按
          // RANGE_COUNT 头连查 num_ranges 个范围 (cc:1143-1146)
          self.block_thread(thread);
          return Ok(());
        }
        (Opcode::Assertion, Payload::Assertion(assertion)) => {
          // cc:913-920 谓词绑定: 原版自由函数 SatisfiesAssertion(inst.payload
          // .assertion_type, input_, input_index_) (cc:25-61) — 无 Isolate/字节码表
          // 耦合, EXP-3a 已落 [`satisfies_assertion`] 本臂直绑; 输入 one/two-byte
          // 双形态 = 原版 Character 模板入口双实例化 (cc:1583-1603 → EXP-4), 本仓
          // u16 码元切片注入折叠 (模块头参数注入台账), 逐码元比较语义全等
          // 原版 cc:28-29 DCHECK_LE/GE(position) 承接: 负 input_index 显式拒
          // (合法驱动 invariant 0 ≤ input_index ≤ len, 恶意态加固, 禁索引回绕假绿)
          let position =
            usize::try_from(self.input_index).map_err(|_| InterpreterError::BadBytecode {
              item: "ASSERTION 臂 input_index 为负 (DCHECK cc:28-29 前置承接)",
              orig: "cc:914-915 SatisfiesAssertion(..., input_index_)",
            })?;
          if !satisfies_assertion(assertion, self.input, position) {
            // cc:916-918 DestroyThread(t) + return success — Rust drop 即销毁
            return Ok(());
          }
          // cc:919-920 ++t.pc; break — 谓词真值续下一指令
          thread.pc += 1;
        }
        // ── EXP-3b 实现臂 (线程调度+状态机转移) ──
        (Opcode::Fork, Payload::Pc(alt_pc)) => {
          // cc:922 NewUninitializedThread(inst.payload.pc) — 原版 int 直存由下轮
          // SBXCHECK cc:888 兜获; Rust 在装配点显式拒 (未入队, 状态未变)
          if alt_pc < 0 {
            return Err(InterpreterError::BadBytecode {
              item: "FORK 臂 目标 pc 为负",
              orig: "cc:922 NewUninitializedThread(inst.payload.pc)",
            });
          }
          let mut fork = self.new_uninitialized_thread(alt_pc);
          // cc:923-924 fork 继承母线程 consumed 态
          fork.consumed_since_last_quantifier = thread.consumed_since_last_quantifier;
          // cc:926-930 寄存器全量拷贝 (DCHECK_EQ cc:928 长度同源不变量)
          fork.register_array.copy_from_slice(&thread.register_array);
          if self.capture_group_opt {
            // cc:932-973 时钟/索引数组四段拷贝 (flag 开); 缺位/失配即畸形线程
            // (非本解释器装配态, 原版 DCHECK_EQ cc:937-968 加固为显式拒)
            let malformed = || InterpreterError::BadBytecode {
              item: "FORK 臂 时钟数组缺位/失配",
              orig: "cc:932-971 DCHECK_EQ 长度族",
            };
            if !copy_array_slot(
              &mut fork.quantifier_clock_array,
              &thread.quantifier_clock_array,
            ) {
              return Err(malformed());
            }
            if !copy_array_slot(&mut fork.capture_clock_array, &thread.capture_clock_array) {
              return Err(malformed());
            }
            if !self.only_captureless_lookbehinds {
              // cc:952-961 命中索引 (脱优化轨专属) / cc:963-971 环视时钟
              if !copy_array_slot(
                &mut fork.lookaround_match_index_array,
                &thread.lookaround_match_index_array,
              ) {
                return Err(malformed());
              }
              if !copy_array_slot(
                &mut fork.lookaround_clock_array,
                &thread.lookaround_clock_array,
              ) {
                return Err(malformed());
              }
            }
          }
          // cc:975 分叉线程入队尾: active 低→高序下继承母线程弹出后留下的最高
          // 优先槽位 (h:61-91: fork 优先级低于母但相对其余线程同序)
          self.active_threads.push(fork);
          // cc:977-980 内存治理接线 (本体 cc:1183-1208 → EXP-3d 探针, flag 开触发)
          if self.capture_group_opt {
            self.check_memory_consumption()?;
          }
          // cc:982 ++t.pc (母线程续下一指令)
          thread.pc += 1;
        }
        (Opcode::Jmp, Payload::Pc(jmp_pc)) => {
          // cc:986 t.pc = inst.payload.pc — 负/越界目标由下轮循环头 SBXCHECK 等价物捕获
          thread.pc = jmp_pc;
        }
        (Opcode::Accept, Payload::None) => {
          // cc:988-998 落定位 + 清除低优先级 active; 线程所有权入 best_match_thread_
          self.set_best_match(thread);
          return Ok(());
        }
        (Opcode::SetQuantifierToClock, Payload::QuantifierId(quantifier_id)) => {
          // cc:999-1001 GetQuantifierClockArray(t)[id] = clock; ++t.pc
          // 原版直读不设防 (cc:1221-1227 DCHECK flag/非空; 编译器不变量 cc:440:
          // 此指令仅在 flag 开程序中出现); Rust tagged None/越界显式拒
          let index = bounded_index(
            quantifier_id,
            self.quantifier_count,
            "SET_QUANTIFIER_TO_CLOCK 臂 量词索引越界",
            "cc:999-1000 直读 + cc:1221-1227 DCHECK 族",
          )?;
          let clock = self.clock;
          let Some(clocks) = thread.quantifier_clock_array.as_mut() else {
            return Err(InterpreterError::BadBytecode {
              item: "SET_QUANTIFIER_TO_CLOCK 臂 量词时钟数组缺位 (线程非 flag 开装配态)",
              orig: "cc:1221-1227 GetQuantifierClockArray DCHECK flag/DCHECK_NOT_NULL",
            });
          };
          clocks[index] = clock;
          thread.pc += 1;
        }
        (Opcode::ClearRegister, Payload::RegisterIndex(register_index)) => {
          // cc:1005-1009 SBXCHECK_BOUNDS → 加固 Err; 寄存器复位哨兵; ++t.pc
          let index = bounded_index(
            register_index,
            self.register_count_per_match,
            "CLEAR_REGISTER 臂 寄存器索引越界 (SBXCHECK_BOUNDS)",
            "cc:1005-1006",
          )?;
          thread.register_array[index] = K_UNDEFINED_REGISTER_VALUE;
          thread.pc += 1;
        }
        (Opcode::SetRegisterToCp, Payload::RegisterIndex(register_index)) => {
          // cc:1012-1018 SBXCHECK_BOUNDS → 加固 Err; 寄存器置当前输入位; ++t.pc
          let index = bounded_index(
            register_index,
            self.register_count_per_match,
            "SET_REGISTER_TO_CP 臂 寄存器索引越界 (SBXCHECK_BOUNDS)",
            "cc:1012-1013",
          )?;
          thread.register_array[index] = self.input_index;
          if self.capture_group_opt {
            // cc:1015-1017 捕获时钟登记 (FilterGroups 过滤数据源)
            let clock = self.clock;
            let Some(clocks) = thread.capture_clock_array.as_mut() else {
              return Err(InterpreterError::BadBytecode {
                item: "SET_REGISTER_TO_CP 臂 捕获时钟数组缺位 (线程非 flag 开装配态)",
                orig: "cc:1228-1234 GetCaptureClockArray DCHECK_NOT_NULL",
              });
            };
            clocks[index] = clock;
          }
          thread.pc += 1;
        }
        (Opcode::BeginLoop, Payload::None) => {
          // cc:1026-1028 量词入口复位 consumed 态 (空串迭代检测起点)
          thread.consumed_since_last_quantifier = ConsumedCharacter::DidNotConsume;
          thread.pc += 1;
        }
        (Opcode::EndLoop, Payload::None) => {
          // cc:1030-1040 空串量词迭代禁止: 整轮未消费字符的线程销毁 (=drop, cc:1036)
          if thread.consumed_since_last_quantifier == ConsumedCharacter::DidNotConsume {
            return Ok(());
          }
          thread.pc += 1;
        }
        (Opcode::StartLookaround, Payload::Lookaround(_)) => {
          // cc:1041-1043 主轨透传 (环视线程由 find_next_match 挂起 / EXP-3d 表臂承接
          // 构建, 此处 payload 仅构造器扫描消费)
          thread.pc += 1;
        }
        (Opcode::EndLookaround, Payload::None) => {
          // cc:1045-1055 环视自动机落定位 (与 ACCEPT 共体 set_best_match)
          self.set_best_match(thread);
          return Ok(());
        }
        // cc:1020-1024 FILTER_* ×4: 原版 UNREACHABLE (过滤段不参与取指执行, 仅
        // FilterGroups::run 直读); 本票按 FilterGroups::run 同款先例收敛为
        // BadBytecode 加固防御臂 (恶意/畸形字节码显式拒, 禁崩溃通道)
        (
          Opcode::FilterGroup
          | Opcode::FilterQuantifier
          | Opcode::FilterLookaround
          | Opcode::FilterChild,
          _,
        ) => {
          return Err(InterpreterError::BadBytecode {
            item: "主循环混入 FILTER_* (原版 UNREACHABLE)",
            orig: "cc:1020-1024",
          });
        }
        // ── EXP-3d 实现臂: 环视真值表读写 (批段 3/3, 本席位已落地) ──
        (Opcode::WriteLookaroundTable, Payload::LookaroundId(lookaround_id)) => {
          // cc:1056-1071: 到达本指令 = 当前环视线程在 (current_)input_index 位命中;
          // 按轨写表后线程销毁 (return, cc:1070 DestroyThread=Rust drop)。
          if !self.only_captureless_lookbehinds {
            // cc:1062-1063 主轨: SBXCHECK_BOUNDS(current_lookaround_, table.size());
            //            table[current_lookaround_][input_index_] = true
            let row = bounded_index(
              self.current_lookaround,
              self.lookaround_table.as_ref().map_or(0, Vec::len),
              "WRITE_LOOKAROUND_TABLE 臂 current_lookaround 越出主轨表行界 (SBXCHECK_BOUNDS)",
              "cc:1062",
            )?;
            let col = bounded_index(
              self.input_index,
              self.lookaround_table.as_ref().map_or(0, |t| t[row].len()),
              "WRITE_LOOKAROUND_TABLE 臂 input_index 越出主轨表列界 (SBXCHECK_BOUNDS)",
              "cc:1063 直读 [input_index_]",
            )?;
            // 表构造器恒 Some (cc:456-459 emplace 于脱优化轨); None 即装配违规
            self
              .lookaround_table
              .as_mut()
              .ok_or(InterpreterError::BadBytecode {
                item: "WRITE_LOOKAROUND_TABLE 臂 主轨表缺位 (cc:456 构造 invariant)",
                orig: "cc:1063 (*lookaround_table_)[..]",
              })?[row][col] = true;
          } else {
            // cc:1065-1067 优化轨: SBXCHECK_BOUNDS(payload.lookaround_id,
            //            lookbehind_table_->length()); Set(lookaround_id, true)
            let slot = bounded_index(
              lookaround_id,
              self.lookbehind_table.as_ref().map_or(0, Vec::len),
              "WRITE_LOOKAROUND_TABLE 臂 lookaround_id 越出优化轨表界 (SBXCHECK_BOUNDS)",
              "cc:1065-1066",
            )?;
            self
              .lookbehind_table
              .as_mut()
              .ok_or(InterpreterError::BadBytecode {
                item: "WRITE_LOOKAROUND_TABLE 臂 优化轨表缺位 (cc:447-449 构造 invariant)",
                orig: "cc:1067 lookbehind_table_->Set(...)",
              })?[slot] = true;
          }
          // cc:1070-1071 DestroyThread(t) + return success — Rust drop 即销毁
          return Ok(());
        }
        (Opcode::ReadLookaroundTable, Payload::Lookaround(lookaround)) => {
          // cc:1072-1112: 按预构建真值表裁决环视 — 表值 != is_positive 即线程销毁
          // (cc:1085/1106 DestroyThread + return); 命中则 ++pc 续跑。优先级表保证
          // 读点所查环视已全部跑完 (原版 cc:1075-1076 注释论证)。
          // payload.index() 位域 u32; 合法程序 < 指令数 (编译器不变量), 越出 int 域
          // 即畸形, 显式拒 (原版 cc:1079/1101 int32 直读)
          let index =
            i32::try_from(lookaround.index()).map_err(|_| InterpreterError::BadBytecode {
              item: "READ_LOOKAROUND_TABLE 臂 index 位域越出 int 域",
              orig: "cc:1079/1101 payload.lookaround.index()",
            })?;
          let is_positive = lookaround.is_positive();
          if !self.only_captureless_lookbehinds {
            // cc:1079-1080 SBXCHECK_BOUNDS(index, lookaround_table_->size())
            let row = bounded_index(
              index,
              self.lookaround_table.as_ref().map_or(0, Vec::len),
              "READ_LOOKAROUND_TABLE 臂 环视索引越出主轨表行界 (SBXCHECK_BOUNDS)",
              "cc:1079-1080",
            )?;
            let col = bounded_index(
              self.input_index,
              self.lookaround_table.as_ref().map_or(0, |t| t[row].len()),
              "READ_LOOKAROUND_TABLE 臂 input_index 越出主轨表列界 (SBXCHECK_BOUNDS)",
              "cc:1082-1083 直读 [input_index_]",
            )?;
            let hit = self
              .lookaround_table
              .as_ref()
              .ok_or(InterpreterError::BadBytecode {
                item: "READ_LOOKAROUND_TABLE 臂 主轨表缺位 (cc:456 构造 invariant)",
                orig: "cc:1082 (*lookaround_table_)[..]",
              })?[row][col];
            if hit != is_positive {
              // cc:1085-1086 DestroyThread(t) + return success
              return Ok(());
            }
            // cc:1090-1096: 正当环视登记命中信息 (时钟 + 命中输入位)。原版条件
            // `is_positive() && !only_captureless_lookbehinds_` 的后者在本分支恒真
            // (折叠冗余, 语义全等); 线程两数组由 new_empty/new_uninitialized 于
            // 脱优化轨装配 Some (cc:1344-1348), None 即恶意线程态显式拒
            if is_positive {
              let clock = self.clock;
              let input_index = self.input_index;
              let malformed = || InterpreterError::BadBytecode {
                item: "READ_LOOKAROUND_TABLE 臂 环视登记数组缺位 (线程非脱优化轨装配态)",
                orig: "cc:1213-1219/1235-1241 GetLookaround*Array DCHECK_NOT_NULL",
              };
              // cc:1092 GetLookaroundClockArray(t)[index] = clock (Get*Array getter
              // cc:1235-1241 折叠为字段直读, 长度 invariant 由线程工厂同源装配)
              let Some(clocks) = thread.lookaround_clock_array.as_mut() else {
                return Err(malformed());
              };
              clocks[row] = clock;
              // cc:1094 GetLookaroundMatchIndexArray(t)[index] = input_index_
              let Some(indexes) = thread.lookaround_match_index_array.as_mut() else {
                return Err(malformed());
              };
              indexes[row] = input_index;
            }
            // cc:1098-1099 ++t.pc; break — 续下一指令
            thread.pc += 1;
          } else {
            // cc:1101-1102 优化轨: lookbehind_index = payload.lookaround.index();
            //            SBXCHECK_BOUNDS(lookbehind_index, lookbehind_table_->length())
            let slot = bounded_index(
              index,
              self.lookbehind_table.as_ref().map_or(0, Vec::len),
              "READ_LOOKAROUND_TABLE 臂 后视索引越出优化轨表界 (SBXCHECK_BOUNDS)",
              "cc:1102",
            )?;
            let hit = self
              .lookbehind_table
              .as_ref()
              .ok_or(InterpreterError::BadBytecode {
                item: "READ_LOOKAROUND_TABLE 臂 优化轨表缺位 (cc:447-449 构造 invariant)",
                orig: "cc:1104 lookbehind_table_->at(...)",
              })?[slot];
            if hit != is_positive {
              // cc:1106-1107 DestroyThread(t) + return success
              return Ok(());
            }
            // cc:1110-1111 ++t.pc; break (优化轨不登记命中信息 — 无捕获后视无捕获可登记)
            thread.pc += 1;
          }
        }
        // tagged Payload 下 opcode/载荷错配 (C++ union 无此态, Rust 显式拒绝)
        (opcode, _) => {
          return Err(InterpreterError::BadBytecode {
            item: match_name(opcode),
            orig: "h:299-319 payload union 与 opcode 静态配对 (工厂函数全为合法配对)",
          });
        }
      }
    }
  }

  /// v8: cc:1117-1127 `RunActiveThreads` — 逐个弹出 active 尾 (最高优先级) 驱动,
  /// 直至全部挂起/销毁/ACCEPT; 出错即停 (原版 V8_WARN_UNUSED_RESULT, Rust ? 上抛)。
  /// 收敛后 active 恒空 (blocked 按原序保留, 优先级论证见 set_best_match 注释)。
  pub fn run_active_threads(&mut self) -> Result<()> {
    while let Some(thread) = self.active_threads.pop() {
      self.run_active_thread(thread)?;
    }
    Ok(())
  }

  /// v8: cc:697-752 `RunActiveThreadsToEnd` — 时钟推进主驱动: 首轮跑 active 至挂起,
  /// 之后每轮喂入一个输入码元唤醒全部 blocked 再跑 active。停止条件 (cc:707-716):
  /// - 输入耗尽 (正向轨 input_index 抵尾; reverse 轨越过 0, 环视反向自动机用);
  /// - 已 FoundMatch 且 blocked 清空 (ACCEPT 已清低优先级 active, 剩余 blocked 皆
  ///   高优先级, 清空即无更优匹配可能)。
  ///
  /// 推迟登记: cc:733-737 kTicks=64 中断检查仅接线, 本体 [`handle_interrupts`]
  /// (NfaInterpreter::handle_interrupts) 为 EXP-4 探针 (输入每跨 64 位触发一次显式失败)。
  pub fn run_active_threads_to_end(&mut self) -> Result<()> {
    // cc:700-705 首轮: 初始线程跑至挂起 (含全零宽程序直接 ACCEPT/销毁的情形)
    self.run_active_threads()?;

    let input_len = self.input.len() as i32;
    // cc:714-716 双轨范围谓词 (reverse: (0, len]; 正向: [0, len))
    while (if self.reverse {
      0 < self.input_index && self.input_index <= input_len
    } else {
      0 <= self.input_index && self.input_index < input_len
    }) && !(self.found_match() && self.blocked_threads.is_empty())
    {
      // cc:717 DCHECK(active_threads_.is_empty()) — run_active_threads 收敛后不变量
      debug_assert!(self.active_threads.is_empty());

      // cc:719-721 仅无捕获后视轨: 后视真值表按新输入位整表清零
      if let Some(table) = self.lookbehind_table.as_mut() {
        table.iter_mut().for_each(|slot| *slot = false);
      }

      // cc:723-731 取本轮码元 (reverse 先退再读; 正向读后退), 原版直读由谓词保证界内
      if self.reverse {
        self.input_index -= 1;
      }
      let input_char = self.input[self.input_index as usize];
      if !self.reverse {
        self.input_index += 1;
      }

      // cc:733-737 每 64 个输入位一次中断检查接线 (本体 Isolate 栈守护面 → EXP-4)
      if self.input_index % K_TICKS_BETWEEN_INTERRUPT_HANDLING == 0 {
        self.handle_interrupts()?;
      }

      // cc:739-740 唤醒全部 blocked (逆序入 active, 低→高保持优先级序)
      self.flush_blocked_threads(input_char)?;

      // cc:742-748 再跑 active 至挂起/ACCEPT
      self.run_active_threads()?;
    }

    Ok(())
  }

  /// v8: cc:826-878 `FindNextMatch` — 自当前 input_index 起新一轮搜索: 复位去重表
  /// (cc:845-846, 原版 TODO cc:833-844 保留), 清残线程池与 best, 装配主线程 (pc 0,
  /// 最低优先级) 与仅无捕获后视轨各环视自动机线程 (cc:866-870, 后入队先弹出 =
  /// 环视优先跑, 编译器保证父先子后见 cc:1538-1544), 驱动至收敛。
  /// 结果消费: [`found_match`](NfaInterpreter::found_match) +
  /// [`best_match_thread`](NfaInterpreter::best_match_thread); 多匹配循环与零长
  /// 前进策略 FindMatches 成员 cc:486-539 → EXP-4。
  pub fn find_next_match(&mut self) -> Result<()> {
    // cc:832 DCHECK(active_threads_.is_empty()) — 上轮 run_active_threads 收敛不变量
    debug_assert!(self.active_threads.is_empty());
    // cc:845-846 去重表整表复位 LastInputIndex()
    self.pc_last_input_index.fill(LastInputIndex::default());
    // cc:848-857 清残线程 (原版 DestroyThread 循环 + Rewind(0); Rust = clear/drop,
    // RecyclingZoneAllocator 回收复用链 EXP-3d)
    self.blocked_threads.clear();
    self.active_threads.clear();
    // cc:859-862 清上一轮 best (DestroyThread + nullopt)
    self.best_match_thread = None;

    // cc:864 主线程 (pc 0, 队首 = 最低优先级)
    self.active_threads.push(self.new_empty_thread(0));
    // cc:866-870 仅无捕获后视轨: 各环视自动机线程依次入队 (弹出先于主线程)
    if self.only_captureless_lookbehinds {
      for i in 0..self.lookarounds.len() {
        let match_pc = self.lookarounds[i].match_pc;
        self.active_threads.push(self.new_empty_thread(match_pc));
      }
    }

    // cc:872-875 驱动至收敛
    self.run_active_threads_to_end()
  }

  /// v8: cc:754-816 `HandleInterrupts` — StackLimitCheck/Isolate 栈守护/GC 后
  /// LATIN1↔UC16 轨切换重开面 (硬耦合入口席位, 台账批段 EXP-4)。本票仅落
  /// cc:733-737 调用接线, 探针在此显式失败 (禁假绿), 每 64 输入位触发一次。
  pub fn handle_interrupts(&self) -> Result<()> {
    deferred_arm("HandleInterrupts", "cc:757-816", EXP_BATCH_4)
  }

  /// v8: cc:1181-1208 `CheckMemoryConsumption` — FORK cc:977-980 内存治理接线。
  /// 阈值比较本体 (cc:1189-1195, 消费 v8-flags defs.rs:5975 默认 1024MB) 与超限
  /// 处置 (cc:1196-1207 Isolate::StackOverflow/异常通道, CallOrigin 面) 台账批段
  /// EXP-3d; 本票探针显式失败 (禁假绿), flag 开 FORK 即触发。
  pub fn check_memory_consumption(&self) -> Result<()> {
    deferred_arm("CheckMemoryConsumption", "cc:1183-1208", EXP_BATCH_3D)
  }
}

/// FORK 臂数组拷贝族 (v8: cc:932-971 四段 std::copy 的指针→Vec 适配): 源/目标同位
/// Some 且等长即全量拷贝返回 true; 任一缺位/长度失配返回 false 由调用方加固
/// (原版 DCHECK_EQ cc:937-968 由同源工厂保证, 恶意线程态显式拒)
fn copy_array_slot<T: Copy>(dst: &mut Option<Vec<T>>, src: &Option<Vec<T>>) -> bool {
  match (dst.as_mut(), src.as_ref()) {
    (Some(d), Some(s)) if d.len() == s.len() => {
      d.copy_from_slice(s);
      true
    }
    _ => false,
  }
}

/// SBXCHECK_BOUNDS 加固面 (v8: cc:1005-1006/1012-1013 越界即中止, 原版 release 亦
/// CHECK; Rust 分解为负值/越上界显式拒, 合法程序 (编译器保证索引界内) 永不触达)
fn bounded_index(
  index: i32,
  count: usize,
  item: &'static str,
  orig: &'static str,
) -> Result<usize> {
  usize::try_from(index)
    .ok()
    .filter(|i| *i < count)
    .ok_or(InterpreterError::BadBytecode { item, orig })
}

/// 推迟臂/探针统一显式失败通道 (禁 todo!()/假绿: 携台账批段与原版行锚, 不执行臂体)
fn deferred_arm(item: &'static str, orig: &'static str, batch: &'static str) -> Result<()> {
  Err(InterpreterError::NotImplemented { item, orig, batch })
}

/// 错配诊断用 opcode 名 (与 bytecode.rs 打印标签同源)
fn match_name(opcode: Opcode) -> &'static str {
  match opcode {
    Opcode::Accept => "ACCEPT",
    Opcode::Assertion => "ASSERTION",
    Opcode::ClearRegister => "CLEAR_REGISTER",
    Opcode::ConsumeRange => "CONSUME_RANGE",
    Opcode::RangeCount => "RANGE_COUNT",
    Opcode::Fork => "FORK",
    Opcode::Jmp => "JMP",
    Opcode::SetRegisterToCp => "SET_REGISTER_TO_CP",
    Opcode::SetQuantifierToClock => "SET_QUANTIFIER_TO_CLOCK",
    Opcode::FilterQuantifier => "FILTER_QUANTIFIER",
    Opcode::FilterGroup => "FILTER_GROUP",
    Opcode::FilterLookaround => "FILTER_LOOKAROUND",
    Opcode::FilterChild => "FILTER_CHILD",
    Opcode::BeginLoop => "BEGIN_LOOP",
    Opcode::EndLoop => "END_LOOP",
    Opcode::StartLookaround => "START_LOOKAROUND",
    Opcode::EndLookaround => "END_LOOKAROUND",
    Opcode::WriteLookaroundTable => "WRITE_LOOKAROUND_TABLE",
    Opcode::ReadLookaroundTable => "READ_LOOKAROUND_TABLE",
  }
}
