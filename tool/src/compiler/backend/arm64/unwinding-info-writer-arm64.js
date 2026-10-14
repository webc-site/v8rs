#!/usr/bin/env bun
// tool/src/compiler/backend/arm64/unwinding-info-writer-arm64.js
// v8/src/compiler/backend/arm64/unwinding-info-writer-arm64.{h,cc}
//   → v8-compiler-backend/src/unwinding_info_writer_arm64.rs.skel
// arm64 eh_frame: LR 保存/恢复标记 + 块初始状态缓存
import { readV8, writeSrc, banner } from "../../../../lib.js";

const h = readV8("src/compiler/backend/arm64/unwinding-info-writer-arm64.h");
const cc = readV8("src/compiler/backend/arm64/unwinding-info-writer-arm64.cc");

// ── 对数断言 ────────────────────────────────────────────────────────────────
let checks = 0;
const must = (c, msg) => { checks++; if (!c) throw new Error(`对数失配: ${msg}`); };
const count = (s, re) => (s.match(re) || []).length;

// h: 类面 (arm64 特色: saved_lr 状态)
must(h.includes("class UnwindingInfoWriter {"), "h 类");
must(h.includes("explicit UnwindingInfoWriter(Zone* zone)"), "h ctor");
must(h.includes("class BlockInitialState : public ZoneObject {"), "h BlockInitialState");
must(h.includes("explicit BlockInitialState(bool saved_lr) : saved_lr_(saved_lr) {}"), "h saved_lr 单字段");
must(h.includes("ZoneVector<const BlockInitialState*> block_initial_states_;"), "h 状态缓存向量");
for (const m of ["void SetNumberOfInstructionBlocks(int number) {",
  "void BeginInstructionBlock(int pc_offset, const InstructionBlock* block);",
  "void EndInstructionBlock(const InstructionBlock* block);",
  "void MarkFrameConstructed(int at_pc);", "void MarkFrameDeconstructed(int at_pc);",
  "void Finish(int code_size) {"])
  if (m.includes("(")) must(h.includes(m.split("{")[0].trim()), `h 方法 ${m.slice(0, 40)}`);

// cc: 实现 6 (含 LR 两方法)
must(count(cc, /^[\w:<>* &]*UnwindingInfoWriter::/gm) === 6, `cc 成员实现 6`);
for (const m of ["void UnwindingInfoWriter::BeginInstructionBlock(int pc_offset,",
  "void UnwindingInfoWriter::EndInstructionBlock(const InstructionBlock* block) {",
  "void UnwindingInfoWriter::MarkFrameConstructed(int at_pc) {",
  "void UnwindingInfoWriter::MarkFrameDeconstructed(int at_pc) {",
  "void UnwindingInfoWriter::MarkLinkRegisterOnTopOfStack(int pc_offset,",
  "void UnwindingInfoWriter::MarkPopLinkRegisterFromTopOfStack(int pc_offset) {"])
  must(cc.includes(m), `cc ${m.slice(0, 50)}`);
must(cc.includes("block_initial_states_"), "cc 状态缓存消费");

// ── 产物 ────────────────────────────────────────────────────────────────────
const skel = `${banner("src/compiler/backend/arm64/unwinding-info-writer-arm64.js",
  "src/compiler/backend/arm64/unwinding-info-writer-arm64.h + unwinding-info-writer-arm64.cc",
  "arm64 eh_frame: LR 栈顶保存/恢复标记 + 块初始状态 (saved_lr) 缓存")}
//! 台账: Begin/EndInstructionBlock (BlockInitialState(saved_lr) 缓存);
//!   MarkFrameConstructed/Deconstructed (CFA 切 sp/fp + LR 登记);
//!   MarkLinkRegisterOnTopOfStack / MarkPopLinkRegisterFromTopOfStack
//!   (arm64 特色: LR 压栈/弹栈的 eh_frame 记录); Finish 收尾。

/// v8: UnwindingInfoWriter (arm64) — 块对 + LR 双标记
// pub struct UnwindingInfoWriter { eh_frame_writer, block_initial_states }
/// v8: BlockInitialState — saved_lr 单字段
// struct BlockInitialState { saved_lr: bool }
/// v8: MarkLinkRegisterOnTopOfStack / MarkPopLinkRegisterFromTopOfStack
// fn mark_link_register_on_top_of_stack(pc_offset, reg_offset)
// fn mark_pop_link_register_from_top_of_stack(pc_offset)

// —— 提取不变式断言 ——
const _: () = {
  assert!(true); // 过程面; 6 实现对数见生成脚本日志
};
`;

writeSrc("v8-compiler-backend", "unwinding_info_writer_arm64.rs.skel", skel);
console.log(`arm64/unwinding-info-writer-arm64.js: 对数 ${checks} 条全过; 实现 6 (含 LR 双标记); ` +
  `BlockInitialState saved_lr; 产物 unwinding_info_writer_arm64.rs.skel`);
