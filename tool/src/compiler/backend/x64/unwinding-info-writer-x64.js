#!/usr/bin/env bun
// tool/src/compiler/backend/x64/unwinding-info-writer-x64.js
// v8/src/compiler/backend/x64/unwinding-info-writer-x64.{h,cc}
//   → v8-compiler-backend/src/unwinding_info_writer_x64.rs.skel
// x64 eh_frame 展开信息: 块初始状态缓存 + fp 跟踪
import { readV8, writeSrc, banner } from "../../../../lib.js";

const h = readV8("src/compiler/backend/x64/unwinding-info-writer-x64.h");
const cc = readV8("src/compiler/backend/x64/unwinding-info-writer-x64.cc");

// ── 对数断言 ────────────────────────────────────────────────────────────────
let checks = 0;
const must = (c, msg) => { checks++; if (!c) throw new Error(`对数失配: ${msg}`); };
const count = (s, re) => (s.match(re) || []).length;

// h: 类面
must(h.includes("class UnwindingInfoWriter {"), "h 类");
must(h.includes("explicit UnwindingInfoWriter(Zone* zone)"), "h ctor");
for (const m of ["void MaybeIncreaseBaseOffsetAt(int pc_offset, int base_delta) {",
  "void SetNumberOfInstructionBlocks(int number) {",
  "void BeginInstructionBlock(int pc_offset, const InstructionBlock* block);",
  "void EndInstructionBlock(const InstructionBlock* block);",
  "void MarkFrameConstructed(int pc_base);", "void MarkFrameDeconstructed(int pc_base);",
  "void MarkBlockWillExit() { block_will_exit_ = true; }", "void Finish(int code_size) {"])
  must(h.includes(m), `h 方法 ${m.slice(0, 45)}`);
must(h.includes("class BlockInitialState : public ZoneObject {"), "h BlockInitialState");
for (const f of ["Register register_;", "int offset_;", "bool tracking_fp_;"])
  must(h.includes(f), `h BlockInitialState 字段 ${f}`);
must(h.includes("bool enabled() const { return v8_flags.perf_prof_unwinding_info; }"), "h enabled 判定");
must(h.includes("EhFrameWriter eh_frame_writer_;") || h.includes("eh_frame_writer_"), "h writer 成员");

// cc: 实现 4
must(count(cc, /^[\w:<>* &]*UnwindingInfoWriter::/gm) === 4, "cc 成员实现 4");
for (const m of ["void UnwindingInfoWriter::BeginInstructionBlock(int pc_offset,",
  "void UnwindingInfoWriter::EndInstructionBlock(const InstructionBlock* block) {",
  "void UnwindingInfoWriter::MarkFrameConstructed(int pc_base) {",
  "void UnwindingInfoWriter::MarkFrameDeconstructed(int pc_base) {"])
  must(cc.includes(m), `cc ${m.slice(0, 50)}`);
// cc: 块状态缓存 + fp 切换
must(cc.includes("block_initial_states_"), "cc 块状态缓存消费");
must(cc.includes("tracking_fp_"), "cc fp 跟踪消费");

// ── 产物 ────────────────────────────────────────────────────────────────────
const skel = `${banner("src/compiler/backend/x64/unwinding-info-writer-x64.js",
  "src/compiler/backend/x64/unwinding-info-writer-x64.h + unwinding-info-writer-x64.cc",
  "x64 eh_frame 展开: 块初始状态缓存 + rbp 跟踪切换")}
//! 台账: enabled() = perf_prof_unwinding_info 旗标; Begin/EndInstructionBlock
//!   (BlockInitialState 缓存 reg/offset/tracking_fp 防重发); MarkFrameConstructed/
//!   Deconstructed (CFA 规则切换 rbp); MaybeIncreaseBaseOffsetAt (非 fp 期基偏);
//!   MarkBlockWillExit (跳转终结块免 End 记录); Finish → eh_frame_writer 收尾。

/// v8: UnwindingInfoWriter (x64) — Begin/End 块对 + Mark 构造/析构
// pub struct UnwindingInfoWriter { eh_frame_writer, tracking_fp, block_initial_states }
/// v8: BlockInitialState — (register, offset, tracking_fp) 三字段缓存
// struct BlockInitialState

// —— 提取不变式断言 ——
const _: () = {
  assert!(true); // 过程面; 4 实现 + 缓存槽对数见生成脚本日志
};
`;

writeSrc("v8-compiler-backend", "unwinding_info_writer_x64.rs.skel", skel);
console.log(`x64/unwinding-info-writer-x64.js: 对数 ${checks} 条全过; 方法 8 + BlockInitialState 3 字段; ` +
  `实现 4; 产物 unwinding_info_writer_x64.rs.skel`);
