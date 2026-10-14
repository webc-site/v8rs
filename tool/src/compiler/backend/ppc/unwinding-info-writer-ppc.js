#!/usr/bin/env bun
// tool/src/compiler/backend/ppc/unwinding-info-writer-ppc.js
// v8/src/compiler/backend/ppc/unwinding-info-writer-ppc.{h,cc}
//   → v8-compiler-backend/src/unwinding_info_writer_ppc.rs.skel
// ppc64 eh_frame: LR 保存/恢复标记 + 块初始状态缓存 (与 arm64 同构)
import { readV8, writeSrc, banner } from "../../../../lib.js";

const h = readV8("src/compiler/backend/ppc/unwinding-info-writer-ppc.h");
const cc = readV8("src/compiler/backend/ppc/unwinding-info-writer-ppc.cc");

// ── 对数断言 ────────────────────────────────────────────────────────────────
let checks = 0;
const must = (c, msg) => { checks++; if (!c) throw new Error(`对数失配: ${msg}`); };
const count = (s, re) => (s.match(re) || []).length;

must(h.includes("class UnwindingInfoWriter {"), "h 类");
must(h.includes("class BlockInitialState : public ZoneObject {"), "h BlockInitialState");
must(count(h, /saved_lr_/g) >= 2, "h saved_lr 字段消费 ≥2");
must(count(cc, /^[\w:<>* &]*UnwindingInfoWriter::/gm) === 6, "cc 成员实现 6");
for (const m of ["void UnwindingInfoWriter::BeginInstructionBlock(int pc_offset,",
  "void UnwindingInfoWriter::EndInstructionBlock(const InstructionBlock* block) {",
  "void UnwindingInfoWriter::MarkFrameConstructed(int at_pc) {",
  "void UnwindingInfoWriter::MarkFrameDeconstructed(int at_pc) {",
  "void UnwindingInfoWriter::MarkLinkRegisterOnTopOfStack(int pc_offset",
  "void UnwindingInfoWriter::MarkPopLinkRegisterFromTopOfStack(int pc_offset) {"])
  must(cc.includes(m), `cc ${m.slice(0, 50)}`);

// ── 产物 ────────────────────────────────────────────────────────────────────
const skel = `${banner("src/compiler/backend/ppc/unwinding-info-writer-ppc.js",
  "src/compiler/backend/ppc/unwinding-info-writer-ppc.h + unwinding-info-writer-ppc.cc",
  "ppc64 eh_frame: LR 保存/恢复标记 + 块初始状态 (saved_lr) 缓存")}
//! 台账: 与 arm64 同构 — Begin/End 块对 + Mark 构造/析构 + LR 双标记。

/// v8: UnwindingInfoWriter (ppc64) — 6 方法面
// pub struct UnwindingInfoWriter { eh_frame_writer, block_initial_states }

// —— 提取不变式断言 ——
const _: () = {
  assert!(true); // 过程面; 6 实现对数见生成脚本日志
};
`;

writeSrc("v8-compiler-backend", "unwinding_info_writer_ppc.rs.skel", skel);
console.log(`ppc/unwinding-info-writer-ppc.js: 对数 ${checks} 条全过; 实现 6; ` +
  `产物 unwinding_info_writer_ppc.rs.skel`);
